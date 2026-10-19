#!/usr/bin/env python3
"""
OKX Android DEX Deobfuscator & Strategy Bot Bytecode Decompiler
===============================================================
Scans all 67 DEX files (`classes.dex` .. `classes67.dex`, 557,944 classes) in
`/Users/aaron/Downloads/android.apk`, builds the global `o/xxx` outer-class
deobfuscation map via `this$0` synthetic fields and Dalvik EnclosingClass
annotations, catalogs all Strategy Bot classes, and decompiles key Bot
math/formula/risk/presenter/viewmodel/DTO classes into readable Java/Kotlin
signatures and annotated Dalvik bytecode.
"""

import json
import os
import re
import struct
import sys
import zipfile
from collections import defaultdict


def read_uleb128(data: bytes, pos: int):
    result = 0
    shift = 0
    while True:
        b = data[pos]
        pos += 1
        result |= (b & 0x7F) << shift
        if (b & 0x80) == 0:
            break
        shift += 7
    return result, pos


def read_sleb128(data: bytes, pos: int):
    result = 0
    shift = 0
    while True:
        b = data[pos]
        pos += 1
        result |= (b & 0x7F) << shift
        shift += 7
        if (b & 0x80) == 0:
            if shift < 32 and (b & 0x40) != 0:
                result |= -(1 << shift)
            break
    return result, pos


def desc_to_readable(desc: str, deobf_map: dict = None) -> str:
    if not desc:
        return "void"
    arr = 0
    while desc.startswith("["):
        arr += 1
        desc = desc[1:]
    primitives = {
        "V": "void",
        "Z": "boolean",
        "B": "byte",
        "S": "short",
        "C": "char",
        "I": "int",
        "J": "long",
        "F": "float",
        "D": "double",
    }
    if desc in primitives:
        base = primitives[desc]
    elif desc.startswith("L") and desc.endswith(";"):
        if deobf_map and desc in deobf_map:
            base = deobf_map[desc]
        else:
            base = desc[1:-1].replace("/", ".")
    else:
        base = desc
    return base + ("[]" * arr)


ACCESS_FLAGS = {
    0x0001: "public",
    0x0002: "private",
    0x0004: "protected",
    0x0008: "static",
    0x0010: "final",
    0x0020: "synchronized",
    0x0040: "volatile",
    0x0080: "transient",
    0x0100: "native",
    0x0200: "interface",
    0x0400: "abstract",
    0x1000: "synthetic",
    0x4000: "enum",
}


def format_access(flags: int, is_method: bool = False) -> str:
    parts = []
    for bit, name in ACCESS_FLAGS.items():
        if is_method and bit in (0x0040, 0x0080):
            continue
        if flags & bit:
            parts.append(name)
    return " ".join(parts)


class DexParser:
    def __init__(self, dex_name: str, data: bytes):
        self.dex_name = dex_name
        self.data = data
        (
            self.string_ids_size,
            self.string_ids_off,
            self.type_ids_size,
            self.type_ids_off,
            self.proto_ids_size,
            self.proto_ids_off,
            self.field_ids_size,
            self.field_ids_off,
            self.method_ids_size,
            self.method_ids_off,
            self.class_defs_size,
            self.class_defs_off,
        ) = struct.unpack_from("<12I", data, 56)
        self._string_cache = {}

    def get_string(self, idx: int) -> str:
        if idx in self._string_cache:
            return self._string_cache[idx]
        if idx < 0 or idx >= self.string_ids_size:
            return ""
        str_off = struct.unpack_from("<I", self.data, self.string_ids_off + idx * 4)[0]
        _, pos = read_uleb128(self.data, str_off)
        end = self.data.find(b"\x00", pos)
        s = self.data[pos:end].decode("utf-8", errors="replace")
        self._string_cache[idx] = s
        return s

    def get_type(self, type_idx: int) -> str:
        if type_idx < 0 or type_idx >= self.type_ids_size:
            return ""
        str_idx = struct.unpack_from("<I", self.data, self.type_ids_off + type_idx * 4)[0]
        return self.get_string(str_idx)

    def get_field(self, field_idx: int):
        off = self.field_ids_off + field_idx * 8
        class_idx, type_idx, name_idx = struct.unpack_from("<HHI", self.data, off)
        return self.get_type(class_idx), self.get_type(type_idx), self.get_string(name_idx)

    def get_proto(self, proto_idx: int):
        off = self.proto_ids_off + proto_idx * 12
        _shorty_idx, return_type_idx, params_off = struct.unpack_from("<III", self.data, off)
        ret_type = self.get_type(return_type_idx)
        params = []
        if params_off != 0:
            size = struct.unpack_from("<I", self.data, params_off)[0]
            for i in range(size):
                t_idx = struct.unpack_from("<H", self.data, params_off + 4 + i * 2)[0]
                params.append(self.get_type(t_idx))
        return ret_type, params

    def get_method(self, method_idx: int):
        off = self.method_ids_off + method_idx * 8
        class_idx, proto_idx, name_idx = struct.unpack_from("<HHI", self.data, off)
        ret_type, params = self.get_proto(proto_idx)
        return self.get_type(class_idx), self.get_string(name_idx), ret_type, params

    def disassemble_code_item(self, code_off: int, deobf_map: dict = None) -> list:
        if code_off == 0:
            return []
        registers_size, ins_size, outs_size, tries_size, debug_info_off, insns_size = (
            struct.unpack_from("<HHHHII", self.data, code_off)
        )
        insns_start = code_off + 16
        insns = []
        pc = 0
        while pc < insns_size:
            word = struct.unpack_from("<H", self.data, insns_start + pc * 2)[0]
            op = word & 0xFF
            hi = (word >> 8) & 0xFF

            # Handle payload pseudo-instructions
            if word == 0x0100:  # packed-switch-payload
                size = struct.unpack_from("<H", self.data, insns_start + (pc + 1) * 2)[0]
                pc += 4 + size * 2
                continue
            elif word == 0x0200:  # sparse-switch-payload
                size = struct.unpack_from("<H", self.data, insns_start + (pc + 1) * 2)[0]
                pc += 2 + size * 4
                continue
            elif word == 0x0300:  # fill-array-data-payload
                elem_width = struct.unpack_from("<H", self.data, insns_start + (pc + 1) * 2)[0]
                size = struct.unpack_from("<I", self.data, insns_start + (pc + 2) * 2)[0]
                total_bytes = elem_width * size
                pc += 4 + (total_bytes + 1) // 2
                continue

            # Decode common Dalvik opcodes with constant / field / method / type resolution
            if op in (0x1A,):  # const-string vAA, string@BBBB
                str_idx = struct.unpack_from("<H", self.data, insns_start + (pc + 1) * 2)[0]
                s = self.get_string(str_idx)
                insns.append(f"0x{pc:04x}: const-string v{hi}, {json.dumps(s, ensure_ascii=False)}")
                pc += 2
            elif op in (0x1B,):  # const-string/jumbo vAA, string@BBBBBBBB
                str_idx = struct.unpack_from("<I", self.data, insns_start + (pc + 1) * 2)[0]
                s = self.get_string(str_idx)
                insns.append(f"0x{pc:04x}: const-string/jumbo v{hi}, {json.dumps(s, ensure_ascii=False)}")
                pc += 3
            elif op in (0x1C, 0x1F, 0x20, 0x22, 0x23):  # const-class, check-cast, instance-of, new-instance, new-array
                type_idx = struct.unpack_from("<H", self.data, insns_start + (pc + 1) * 2)[0]
                t_str = desc_to_readable(self.get_type(type_idx), deobf_map)
                op_names = {0x1C: "const-class", 0x1F: "check-cast", 0x20: "instance-of", 0x22: "new-instance", 0x23: "new-array"}
                insns.append(f"0x{pc:04x}: {op_names[op]} v{hi & 0xF}, {t_str}")
                pc += 2
            elif 0x52 <= op <= 0x5F:  # iget* / iput*
                f_idx = struct.unpack_from("<H", self.data, insns_start + (pc + 1) * 2)[0]
                c_desc, f_type, f_name = self.get_field(f_idx)
                c_str = desc_to_readable(c_desc, deobf_map)
                t_str = desc_to_readable(f_type, deobf_map)
                kind = "iget" if op <= 0x58 else "iput"
                insns.append(f"0x{pc:04x}: {kind} v{hi & 0xF}, v{hi >> 4}, {c_str}.{f_name}:{t_str}")
                pc += 2
            elif 0x60 <= op <= 0x6D:  # sget* / sput*
                f_idx = struct.unpack_from("<H", self.data, insns_start + (pc + 1) * 2)[0]
                c_desc, f_type, f_name = self.get_field(f_idx)
                c_str = desc_to_readable(c_desc, deobf_map)
                t_str = desc_to_readable(f_type, deobf_map)
                kind = "sget" if op <= 0x66 else "sput"
                insns.append(f"0x{pc:04x}: {kind} v{hi}, {c_str}.{f_name}:{t_str}")
                pc += 2
            elif 0x6E <= op <= 0x72 or 0x74 <= op <= 0x78:  # invoke-* / invoke-*/range
                m_idx = struct.unpack_from("<H", self.data, insns_start + (pc + 1) * 2)[0]
                c_desc, m_name, ret_t, params = self.get_method(m_idx)
                c_str = desc_to_readable(c_desc, deobf_map)
                ret_str = desc_to_readable(ret_t, deobf_map)
                params_str = ", ".join(desc_to_readable(p, deobf_map) for p in params)
                inv_names = {
                    0x6E: "invoke-virtual", 0x6F: "invoke-super", 0x70: "invoke-direct",
                    0x71: "invoke-static", 0x72: "invoke-interface",
                    0x74: "invoke-virtual/range", 0x75: "invoke-super/range", 0x76: "invoke-direct/range",
                    0x77: "invoke-static/range", 0x78: "invoke-interface/range",
                }
                insns.append(f"0x{pc:04x}: {inv_names[op]} {c_str}.{m_name}({params_str}):{ret_str}")
                pc += 3
            elif op in (0x12,):  # const/4
                reg = hi & 0xF
                lit = (hi >> 4)
                if lit & 0x8:
                    lit -= 16
                insns.append(f"0x{pc:04x}: const/4 v{reg}, {lit}")
                pc += 1
            elif op in (0x13,):  # const/16
                lit = struct.unpack_from("<h", self.data, insns_start + (pc + 1) * 2)[0]
                insns.append(f"0x{pc:04x}: const/16 v{hi}, {lit}")
                pc += 2
            elif op in (0x14,):  # const
                lit = struct.unpack_from("<i", self.data, insns_start + (pc + 1) * 2)[0]
                insns.append(f"0x{pc:04x}: const v{hi}, {lit}")
                pc += 3
            elif op in (0x15,):  # const/high16
                lit = struct.unpack_from("<h", self.data, insns_start + (pc + 1) * 2)[0] << 16
                insns.append(f"0x{pc:04x}: const/high16 v{hi}, 0x{lit & 0xffffffff:08x}")
                pc += 2
            elif op in (0x16,):  # const-wide/16
                lit = struct.unpack_from("<h", self.data, insns_start + (pc + 1) * 2)[0]
                insns.append(f"0x{pc:04x}: const-wide/16 v{hi}, {lit}")
                pc += 2
            elif op in (0x17,):  # const-wide/32
                lit = struct.unpack_from("<i", self.data, insns_start + (pc + 1) * 2)[0]
                insns.append(f"0x{pc:04x}: const-wide/32 v{hi}, {lit}")
                pc += 3
            elif op in (0x18,):  # const-wide
                lit = struct.unpack_from("<q", self.data, insns_start + (pc + 1) * 2)[0]
                insns.append(f"0x{pc:04x}: const-wide v{hi}, {lit}")
                pc += 5
            elif op in (0x19,):  # const-wide/high16
                lit = struct.unpack_from("<h", self.data, insns_start + (pc + 1) * 2)[0] << 48
                insns.append(f"0x{pc:04x}: const-wide/high16 v{hi}, 0x{lit & 0xffffffffffffffff:016x}")
                pc += 2
            elif op in (0x0E, 0x0F, 0x10, 0x11):  # return*
                ret_names = {0x0E: "return-void", 0x0F: f"return v{hi}", 0x10: f"return-wide v{hi}", 0x11: f"return-object v{hi}"}
                insns.append(f"0x{pc:04x}: {ret_names[op]}")
                pc += 1
            else:
                # Advance by standard Dalvik instruction width
                if op in (
                    0x00, 0x01, 0x04, 0x07, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11,
                    0x12, 0x1D, 0x1E, 0x21, 0x27, 0x28,
                ) or (0x44 <= op <= 0x51) or (0x7B <= op <= 0xCF):
                    pc += 1
                elif op in (
                    0x02, 0x05, 0x08, 0x13, 0x15, 0x16, 0x19, 0x1A, 0x1C, 0x1F, 0x20, 0x22,
                    0x23, 0x29, 0x2D, 0x2E, 0x2F, 0x30, 0x31,
                ) or (0x32 <= op <= 0x3D) or (0x52 <= op <= 0x6D) or (0xD0 <= op <= 0xE2):
                    pc += 2
                elif op in (
                    0x03, 0x06, 0x09, 0x14, 0x17, 0x1B, 0x24, 0x25, 0x26, 0x2A, 0x2B, 0x2C,
                ) or (0x6E <= op <= 0x78):
                    pc += 3
                elif op == 0x18:
                    pc += 5
                else:
                    pc += 1
        return insns

    def parse_class_def(self, class_idx: int, deobf_map: dict = None, disassemble: bool = False) -> dict:
        off = self.class_defs_off + class_idx * 32
        (
            c_type_idx,
            access_flags,
            super_idx,
            interfaces_off,
            source_file_idx,
            annotations_off,
            class_data_off,
            static_values_off,
        ) = struct.unpack_from("<8I", self.data, off)

        c_desc = self.get_type(c_type_idx)
        super_desc = self.get_type(super_idx) if super_idx != 0xFFFFFFFF else ""
        source_file = self.get_string(source_file_idx) if source_file_idx != 0xFFFFFFFF else ""

        interfaces = []
        if interfaces_off != 0:
            size = struct.unpack_from("<I", self.data, interfaces_off)[0]
            for i in range(size):
                t_idx = struct.unpack_from("<H", self.data, interfaces_off + 4 + i * 2)[0]
                interfaces.append(self.get_type(t_idx))

        static_fields = []
        instance_fields = []
        direct_methods = []
        virtual_methods = []

        if class_data_off != 0:
            pos = class_data_off
            sf_size, pos = read_uleb128(self.data, pos)
            if_size, pos = read_uleb128(self.data, pos)
            dm_size, pos = read_uleb128(self.data, pos)
            vm_size, pos = read_uleb128(self.data, pos)

            f_idx = 0
            for _ in range(sf_size):
                diff, pos = read_uleb128(self.data, pos)
                acc, pos = read_uleb128(self.data, pos)
                f_idx += diff
                _, f_type, f_name = self.get_field(f_idx)
                static_fields.append({"name": f_name, "type": f_type, "access": format_access(acc)})

            f_idx = 0
            for _ in range(if_size):
                diff, pos = read_uleb128(self.data, pos)
                acc, pos = read_uleb128(self.data, pos)
                f_idx += diff
                _, f_type, f_name = self.get_field(f_idx)
                instance_fields.append({"name": f_name, "type": f_type, "access": format_access(acc)})

            m_idx = 0
            for _ in range(dm_size):
                diff, pos = read_uleb128(self.data, pos)
                acc, pos = read_uleb128(self.data, pos)
                code_off, pos = read_uleb128(self.data, pos)
                m_idx += diff
                _, m_name, ret_t, params = self.get_method(m_idx)
                m_entry = {
                    "name": m_name,
                    "return_type": ret_t,
                    "params": params,
                    "access": format_access(acc, True),
                    "code_off": code_off,
                }
                if disassemble and code_off != 0:
                    m_entry["insns"] = self.disassemble_code_item(code_off, deobf_map)
                direct_methods.append(m_entry)

            m_idx = 0
            for _ in range(vm_size):
                diff, pos = read_uleb128(self.data, pos)
                acc, pos = read_uleb128(self.data, pos)
                code_off, pos = read_uleb128(self.data, pos)
                m_idx += diff
                _, m_name, ret_t, params = self.get_method(m_idx)
                m_entry = {
                    "name": m_name,
                    "return_type": ret_t,
                    "params": params,
                    "access": format_access(acc, True),
                    "code_off": code_off,
                }
                if disassemble and code_off != 0:
                    m_entry["insns"] = self.disassemble_code_item(code_off, deobf_map)
                virtual_methods.append(m_entry)

        return {
            "dex": self.dex_name,
            "descriptor": c_desc,
            "deobfuscated": deobf_map.get(c_desc) if deobf_map else None,
            "access": format_access(access_flags),
            "super": super_desc,
            "interfaces": interfaces,
            "source_file": source_file,
            "static_fields": static_fields,
            "instance_fields": instance_fields,
            "direct_methods": direct_methods,
            "virtual_methods": virtual_methods,
        }


def render_pseudo_java(cls_info: dict, deobf_map: dict) -> str:
    desc = cls_info["descriptor"]
    readable_name = desc_to_readable(desc, deobf_map)
    orig_comment = f" /* obfuscated: {desc} */" if desc in deobf_map else ""
    super_str = desc_to_readable(cls_info["super"], deobf_map) if cls_info["super"] else "java.lang.Object"
    ifaces = [desc_to_readable(i, deobf_map) for i in cls_info["interfaces"]]

    lines = []
    lines.append(f"// DEX: {cls_info['dex']} | Source: {cls_info['source_file'] or 'ProGuard/R8'}")
    header = f"{cls_info['access']} class {readable_name}{orig_comment}"
    if super_str != "java.lang.Object":
        header += f" extends {super_str}"
    if ifaces:
        header += " implements " + ", ".join(ifaces)
    lines.append(header + " {")

    if cls_info["static_fields"]:
        lines.append("    // Static Fields")
        for f in cls_info["static_fields"]:
            lines.append(f"    {f['access']} {desc_to_readable(f['type'], deobf_map)} {f['name']};")
        lines.append("")

    if cls_info["instance_fields"]:
        lines.append("    // Instance Fields")
        for f in cls_info["instance_fields"]:
            lines.append(f"    {f['access']} {desc_to_readable(f['type'], deobf_map)} {f['name']};")
        lines.append("")

    for m in cls_info["direct_methods"] + cls_info["virtual_methods"]:
        params_str = ", ".join(
            f"{desc_to_readable(p, deobf_map)} p{idx}" for idx, p in enumerate(m["params"])
        )
        ret_str = desc_to_readable(m["return_type"], deobf_map)
        sig = f"    {m['access']} {ret_str} {m['name']}({params_str})"
        insns = m.get("insns", [])
        if not insns:
            lines.append(sig + ";")
        else:
            lines.append(sig + " {")
            for insn in insns:
                lines.append(f"        {insn}")
            lines.append("    }\n")

    lines.append("}\n")
    return "\n".join(lines)


def main():
    apk_path = sys.argv[1] if len(sys.argv) > 1 else "/Users/aaron/Downloads/android.apk"
    out_dir = sys.argv[2] if len(sys.argv) > 2 else "/Users/aaron/Desktop/aaron/okx/reconstructed/dex_decompiled"
    os.makedirs(out_dir, exist_ok=True)

    print(f"[*] Opening {apk_path} ...")
    zf = zipfile.ZipFile(apk_path, "r")
    dex_names = sorted(
        [n for n in zf.namelist() if n.startswith("classes") and n.endswith(".dex")],
        key=lambda x: int(x[7:-4] or "1"),
    )
    print(f"[*] Found {len(dex_names)} DEX files (`{dex_names[0]}` .. `{dex_names[-1]}`).")

    deobf_map = {}  # "Lo/xxx;" -> "com.okinc.tradingbot...."
    bot_classes_summary = []
    package_counts = defaultdict(int)
    total_classes = 0

    # Key DEX files containing Strategy Bot & UnifyTrade Bot classes
    # First pass across all DEX files: count packages, recover `this$0` outer class names, collect bot classes
    target_dex_parsers = {}

    for dname in dex_names:
        raw = zf.read(dname)
        parser = DexParser(dname, raw)
        total_classes += parser.class_defs_size

        has_bot = False
        for c_idx in range(parser.class_defs_size):
            off = parser.class_defs_off + c_idx * 32
            c_type_idx, access_flags, super_idx, interfaces_off, source_file_idx, annotations_off, class_data_off, _ = (
                struct.unpack_from("<8I", raw, off)
            )
            c_desc = parser.get_type(c_type_idx)
            # Track top-level package
            if c_desc.startswith("L"):
                parts = c_desc[1:-1].split("/")
                pkg = "/".join(parts[:3]) if len(parts) >= 3 else parts[0]
                package_counts[pkg] += 1

            is_bot_cls = (
                c_desc.startswith("Lcom/okinc/tradingbot/")
                or c_desc.startswith("Lcom/okinc/unify_trade/bot/")
                or c_desc.startswith("Lcom/okinc/unify_trade/biz/bot/")
                or c_desc.startswith("Lcom/okinc/kline/features/botkline/")
                or c_desc.startswith("Luniffi/trading_bot/")
                or c_desc.startswith("Luniffi/social_trading/")
            )
            if is_bot_cls:
                has_bot = True
                # If inner class (`Outer$Inner`), inspect `this$0` field to recover obfuscated `Lo/xxx;` outer class
                if "$" in c_desc and class_data_off != 0:
                    outer_prefix = c_desc.split("$")[0]
                    pos = class_data_off
                    sf_size, pos = read_uleb128(raw, pos)
                    if_size, pos = read_uleb128(raw, pos)
                    for _ in range(sf_size):
                        _, pos = read_uleb128(raw, pos)
                        _, pos = read_uleb128(raw, pos)
                    f_idx = 0
                    for _ in range(if_size):
                        diff, pos = read_uleb128(raw, pos)
                        _acc, pos = read_uleb128(raw, pos)
                        f_idx += diff
                        _, f_type, f_name = parser.get_field(f_idx)
                        if f_name == "this$0" and f_type.startswith("Lo/"):
                            deobf_map[f_type] = outer_prefix[1:].replace("/", ".")
                        elif f_idx == 0 and if_size == 1 and f_type.startswith("Lo/"):
                            deobf_map.setdefault(f_type, outer_prefix[1:].replace("/", "."))

        if has_bot:
            target_dex_parsers[dname] = parser

    print(f"[*] Scanned {total_classes:,} total classes across {len(dex_names)} DEX files.")
    print(f"[*] Recovered {len(deobf_map):,} obfuscated `o/xxx` outer classes via synthetic inner-class links.")

    # Second pass on target DEX files: parse all Bot classes + their recovered `Lo/xxx;` outer classes
    decompiled_groups = defaultdict(list)

    for dname, parser in sorted(target_dex_parsers.items(), key=lambda x: int(x[0][7:-4] or "1")):
        raw = parser.data
        for c_idx in range(parser.class_defs_size):
            off = parser.class_defs_off + c_idx * 32
            c_type_idx = struct.unpack_from("<I", raw, off)[0]
            c_desc = parser.get_type(c_type_idx)

            is_bot_cls = (
                c_desc.startswith("Lcom/okinc/tradingbot/")
                or c_desc.startswith("Lcom/okinc/unify_trade/bot/")
                or c_desc.startswith("Lcom/okinc/unify_trade/biz/bot/")
                or c_desc.startswith("Lcom/okinc/kline/features/botkline/")
                or c_desc.startswith("Luniffi/trading_bot/")
                or c_desc in deobf_map
            )
            if not is_bot_cls:
                continue

            readable = desc_to_readable(c_desc, deobf_map)
            # Determine if we should disassemble full method bytecode for this class
            should_disasm = (
                c_desc in deobf_map
                or any(
                    kw in readable
                    for kw in (
                        "Math",
                        "Liquidate",
                        "MinInvest",
                        "Presenter",
                        "ViewModel",
                        "Interactor",
                        "Param",
                        "Request",
                        "Response",
                        "Entity",
                        "Config",
                        "Risk",
                        "Calculator",
                        "SpotGrid",
                        "ContractGrid",
                        "FuturesGrid",
                        "InfinityGrid",
                        "SpotDca",
                        "ContractDca",
                        "SmartArb",
                        "Arbitrage",
                        "SmartPortfolio",
                        "Recurring",
                        "Signal",
                        "Dcd",
                        "Flywheel",
                        "Twap",
                        "Iceberg",
                        "BotDetail",
                        "BotKline",
                    )
                )
            ) and "$" not in c_desc

            cls_info = parser.parse_class_def(c_idx, deobf_map, disassemble=should_disasm)
            bot_classes_summary.append({
                "dex": dname,
                "descriptor": c_desc,
                "readable_name": readable,
                "super": desc_to_readable(cls_info["super"], deobf_map),
                "interfaces": [desc_to_readable(i, deobf_map) for i in cls_info["interfaces"]],
                "fields_count": len(cls_info["static_fields"]) + len(cls_info["instance_fields"]),
                "methods_count": len(cls_info["direct_methods"]) + len(cls_info["virtual_methods"]),
            })

            if should_disasm:
                # Categorize into bot family file
                low = readable.lower()
                if "dca" in low or "martingale" in low:
                    group = "dca_martingale_bots"
                elif "grid" in low:
                    group = "grid_bots"
                elif "arb" in low:
                    group = "arbitrage_bots"
                elif "portfolio" in low or "recurring" in low:
                    group = "portfolio_and_recurring_bots"
                elif "signal" in low or "dcd" in low or "flywheel" in low:
                    group = "signal_and_dcd_bots"
                elif "twap" in low or "iceberg" in low:
                    group = "twap_and_iceberg_slicing_bots"
                elif "kline" in low:
                    group = "bot_kline_overlays"
                elif "uniffi.trading_bot" in low:
                    group = "uniffi_kotlin_bindings"
                else:
                    group = "core_bot_framework_and_detail"

                if len(decompiled_groups[group]) < 150:
                    decompiled_groups[group].append(render_pseudo_java(cls_info, deobf_map))

    # Write outputs
    summary_path = os.path.join(out_dir, "dex_bot_classes_catalog.json")
    with open(summary_path, "w", encoding="utf-8") as f:
        json.dump(
            {
                "apk_path": apk_path,
                "total_dex_files": len(dex_names),
                "total_apk_classes": total_classes,
                "total_bot_classes_cataloged": len(bot_classes_summary),
                "recovered_obfuscated_outer_classes": deobf_map,
                "top_packages": dict(sorted(package_counts.items(), key=lambda x: -x[1])[:60]),
                "classes": bot_classes_summary,
            },
            f,
            indent=2,
            ensure_ascii=False,
        )
    print(f"[+] Wrote {summary_path} ({len(bot_classes_summary):,} bot classes cataloged)")

    for group, java_blocks in decompiled_groups.items():
        group_path = os.path.join(out_dir, f"{group}.java")
        with open(group_path, "w", encoding="utf-8") as f:
            f.write(f"// =============================================================================\n")
            f.write(f"// Decompiled OKX v6.194.0 Strategy Bot Module: {group}\n")
            f.write(f"// Classes in file: {len(java_blocks)}\n")
            f.write(f"// =============================================================================\n\n")
            f.write("\n".join(java_blocks))
        print(f"[+] Wrote {group_path} ({len(java_blocks)} decompiled classes)")


if __name__ == "__main__":
    main()
