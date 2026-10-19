#!/usr/bin/env python3
"""
OKX libOKCore.so UniFFI Metadata Decoder & Rust IDL Reconstructor
=================================================================
Parses the ELF .dynsym / .dynstr / .rodata sections of libOKCore.so (v6.194.0)
and decodes all `UNIFFI_META_*` binary metadata buffers exported by Mozilla UniFFI
into structured JSON and idiomatic, fully-documented Rust source code (.rs).

Recovers:
- Record structs (fields, types, optionality, default values, Rust docstrings)
- Enums (flat, associated-data, non-exhaustive, discriminant values, docstrings)
- Interfaces / Traits (constructors, methods, async flags, return/throws types, docstrings)
- Top-level Functions (arguments, defaults, return/throws types, docstrings)
- Crate source file trees from embedded panic/location metadata
"""

import json
import os
import re
import struct
import sys
from collections import defaultdict

# Exact UniFFI Type Tags in OKX v6.194.0 libOKCore.so
TYPE_U8 = 0
TYPE_U16 = 1
TYPE_U32 = 2
TYPE_U64 = 3
TYPE_I8 = 4
TYPE_I16 = 5
TYPE_I32 = 6
TYPE_I64 = 7
TYPE_F32 = 8
TYPE_F64 = 9
TYPE_BOOL = 10
TYPE_STRING = 11
TYPE_OPTION = 12
TYPE_RECORD = 13
TYPE_ENUM = 14
TYPE_INTERFACE = 16
TYPE_VEC = 17
TYPE_HASH_MAP = 18
TYPE_CUSTOM = 19
TYPE_TIMESTAMP = 20
TYPE_CALLBACK_INTERFACE = 21
TYPE_DURATION = 22
TYPE_RESULT = 23
TYPE_BYTES = 24
TYPE_TRAIT_INTERFACE = 25
TYPE_BOX = 26
TYPE_VOID = 255


class BufferReader:
    def __init__(self, buf: bytes):
        self.buf = buf
        self.pos = 0

    def remaining(self) -> int:
        return len(self.buf) - self.pos

    def u8(self) -> int:
        val = self.buf[self.pos]
        self.pos += 1
        return val

    def i8(self) -> int:
        val = struct.unpack_from("<b", self.buf, self.pos)[0]
        self.pos += 1
        return val

    def u16(self) -> int:
        val = struct.unpack_from("<H", self.buf, self.pos)[0]
        self.pos += 2
        return val

    def i16(self) -> int:
        val = struct.unpack_from("<h", self.buf, self.pos)[0]
        self.pos += 2
        return val

    def u32(self) -> int:
        val = struct.unpack_from("<I", self.buf, self.pos)[0]
        self.pos += 4
        return val

    def i32(self) -> int:
        val = struct.unpack_from("<i", self.buf, self.pos)[0]
        self.pos += 4
        return val

    def u64(self) -> int:
        val = struct.unpack_from("<Q", self.buf, self.pos)[0]
        self.pos += 8
        return val

    def i64(self) -> int:
        val = struct.unpack_from("<q", self.buf, self.pos)[0]
        self.pos += 8
        return val

    def bool(self) -> bool:
        return self.u8() != 0

    def string(self) -> str:
        length = self.u8()
        s = self.buf[self.pos : self.pos + length].decode("utf-8", errors="replace")
        self.pos += length
        return s

    def long_string(self) -> str:
        if self.remaining() < 2:
            return ""
        length = self.u16()
        s = self.buf[self.pos : self.pos + length].decode("utf-8", errors="replace")
        self.pos += length
        return s

    def read_type(self) -> str:
        tag = self.u8()
        if tag == TYPE_U8:
            return "u8"
        elif tag == TYPE_U16:
            return "u16"
        elif tag == TYPE_U32:
            return "u32"
        elif tag == TYPE_U64:
            return "u64"
        elif tag == TYPE_I8:
            return "i8"
        elif tag == TYPE_I16:
            return "i16"
        elif tag == TYPE_I32:
            return "i32"
        elif tag == TYPE_I64:
            return "i64"
        elif tag == TYPE_F32:
            return "f32"
        elif tag == TYPE_F64:
            return "f64"
        elif tag == TYPE_BOOL:
            return "bool"
        elif tag == TYPE_STRING:
            return "String"
        elif tag == TYPE_OPTION:
            inner = self.read_type()
            return f"Option<{inner}>"
        elif tag == TYPE_RECORD:
            _mod = self.string()
            name = self.string()
            return name
        elif tag == TYPE_ENUM:
            _mod = self.string()
            name = self.string()
            return name
        elif tag == TYPE_INTERFACE:
            _mod = self.string()
            name = self.string()
            return f"Arc<{name}>"
        elif tag == TYPE_VEC:
            inner = self.read_type()
            return f"Vec<{inner}>"
        elif tag == TYPE_HASH_MAP:
            k = self.read_type()
            v = self.read_type()
            return f"HashMap<{k}, {v}>"
        elif tag == TYPE_CUSTOM:
            _mod = self.string()
            name = self.string()
            builtin = self.read_type()
            return f"{name} /* {builtin} */"
        elif tag == TYPE_TIMESTAMP:
            return "SystemTime"
        elif tag == TYPE_CALLBACK_INTERFACE:
            _mod = self.string()
            name = self.string()
            return f"Box<dyn {name}>"
        elif tag == TYPE_DURATION:
            return "Duration"
        elif tag == TYPE_RESULT:
            ok_t = self.read_type()
            err_t = self.read_type()
            return f"Result<{ok_t}, {err_t}>"
        elif tag == TYPE_BYTES:
            return "Vec<u8>"
        elif tag == TYPE_TRAIT_INTERFACE:
            _mod = self.string()
            name = self.string()
            return f"Arc<dyn {name}>"
        elif tag == TYPE_BOX:
            inner = self.read_type()
            return f"Box<{inner}>"
        elif tag == TYPE_VOID:
            return "()"
        else:
            raise ValueError(f"Unknown type tag {tag} at pos {self.pos - 1}")

    def read_optional_type(self):
        has_type = self.bool()
        if not has_type:
            return None
        return self.read_type()

    def read_literal(self) -> str:
        lit_kind = self.u8()
        if lit_kind == 0:  # String
            return json.dumps(self.string(), ensure_ascii=False)
        elif lit_kind == 1:  # Int / UInt as string
            return self.string()
        elif lit_kind == 2:  # Float as string
            return self.string()
        elif lit_kind == 3:  # Boolean
            return "true" if self.bool() else "false"
        elif lit_kind == 4:  # None
            return "None"
        elif lit_kind == 5:  # Some
            inner = self.read_default_value_inner()
            return f"Some({inner})"
        elif lit_kind == 6:  # EmptySequence
            return "[]"
        elif lit_kind == 7:  # EmptyMap
            return "{}"
        elif lit_kind == 8:  # Enum
            variant = self.string()
            t = self.read_type()
            return f"{t}::{variant}"
        else:
            raise ValueError(f"Unknown literal kind {lit_kind} at pos {self.pos - 1}")

    def read_default_value_inner(self) -> str:
        default_kind = self.u8()
        if default_kind == 0:  # Default::Default
            return "Default::default()"
        elif default_kind == 1:  # Default::Literal
            return self.read_literal()
        else:
            raise ValueError(f"Unknown default kind {default_kind} at pos {self.pos - 1}")

    def read_default_value(self):
        has_default = self.bool()
        if not has_default:
            return None
        return self.read_default_value_inner()


def decode_uniffi_meta(symbol_name: str, buf: bytes) -> dict:
    r = BufferReader(buf)
    kind = r.u8()
    try:
        if kind == 0:  # FUNC
            module = r.string()
            name = r.string()
            is_async = r.bool()
            num_args = r.u8()
            args = []
            for _ in range(num_args):
                arg_name = r.string()
                arg_type = r.read_type()
                default_val = r.read_default_value()
                args.append({
                    "name": arg_name,
                    "type": arg_type,
                    "default": default_val,
                })
            return_type = r.read_type()
            doc = r.long_string()
            return {
                "kind": "func",
                "symbol": symbol_name,
                "module": module,
                "name": name,
                "is_async": is_async,
                "args": args,
                "return_type": None if return_type == "()" else return_type,
                "doc": doc,
            }

        elif kind == 1:  # METHOD
            module = r.string()
            self_name = r.string()
            name = r.string()
            is_async = r.bool()
            num_args = r.u8()
            args = []
            for _ in range(num_args):
                arg_name = r.string()
                arg_type = r.read_type()
                default_val = r.read_default_value()
                args.append({
                    "name": arg_name,
                    "type": arg_type,
                    "default": default_val,
                })
            return_type = r.read_type()
            doc = r.long_string()
            return {
                "kind": "method",
                "symbol": symbol_name,
                "module": module,
                "interface": self_name,
                "name": name,
                "is_async": is_async,
                "args": args,
                "return_type": None if return_type == "()" else return_type,
                "doc": doc,
            }

        elif kind == 2:  # RECORD
            module = r.string()
            name = r.string()
            num_fields = r.u8()
            fields = []
            for _ in range(num_fields):
                fname = r.string()
                ftype = r.read_type()
                default_val = r.read_default_value()
                fdoc = r.long_string()
                fields.append({
                    "name": fname,
                    "type": ftype,
                    "default": default_val,
                    "doc": fdoc,
                })
            doc = r.long_string()
            return {
                "kind": "record",
                "symbol": symbol_name,
                "module": module,
                "name": name,
                "fields": fields,
                "doc": doc,
            }

        elif kind == 3:  # ENUM
            module = r.string()
            name = r.string()
            shape = r.u8()  # 0 = Enum, 1 = Error(Flat), 2 = Error(Rich)
            discr_type = r.read_optional_type()
            num_variants = r.u8()
            variants = []
            saved_pos = r.pos
            try:
                for _ in range(num_variants):
                    vname = r.string()
                    has_discr = r.bool()
                    discr = r.read_literal() if has_discr else None
                    num_fields = r.u8()
                    vfields = []
                    for _ in range(num_fields):
                        fname = r.string()
                        ftype = r.read_type()
                        default_val = r.read_default_value()
                        fdoc = r.long_string()
                        vfields.append({
                            "name": fname,
                            "type": ftype,
                            "default": default_val,
                            "doc": fdoc,
                        })
                    vdoc = r.long_string()
                    variants.append({
                        "name": vname,
                        "discr": discr,
                        "fields": vfields,
                        "doc": vdoc,
                    })
                non_exhaustive = r.bool()
                doc = r.long_string()
            except Exception:
                r.pos = saved_pos
                variants = []
                for _ in range(num_variants):
                    vname = r.string()
                    has_discr = r.bool()
                    discr = r.read_literal() if has_discr else None
                    vdoc = r.long_string()
                    variants.append({
                        "name": vname,
                        "discr": discr,
                        "fields": [],
                        "doc": vdoc,
                    })
                non_exhaustive = r.bool() if r.remaining() >= 1 else False
                doc = r.long_string()
            return {
                "kind": "enum",
                "symbol": symbol_name,
                "module": module,
                "name": name,
                "shape": shape,
                "discr_type": discr_type,
                "non_exhaustive": non_exhaustive,
                "variants": variants,
                "doc": doc,
            }

        elif kind in (4, 12, 13):  # INTERFACE / TRAIT_INTERFACE
            module = r.string()
            name = r.string()
            doc = r.long_string()
            return {
                "kind": "interface",
                "symbol": symbol_name,
                "module": module,
                "name": name,
                "trait_kind": kind,
                "doc": doc,
            }

        elif kind == 6:  # NAMESPACE
            crate_name = r.string()
            ns_name = r.string()
            return {
                "kind": "namespace",
                "symbol": symbol_name,
                "module": crate_name,
                "name": ns_name,
            }

        elif kind == 7:  # CONSTRUCTOR
            module = r.string()
            self_name = r.string()
            name = r.string()
            is_async = r.bool()
            num_args = r.u8()
            args = []
            for _ in range(num_args):
                arg_name = r.string()
                arg_type = r.read_type()
                default_val = r.read_default_value()
                args.append({
                    "name": arg_name,
                    "type": arg_type,
                    "default": default_val,
                })
            return_type = r.read_type()
            doc = r.long_string()
            return {
                "kind": "constructor",
                "symbol": symbol_name,
                "module": module,
                "interface": self_name,
                "name": name,
                "is_async": is_async,
                "args": args,
                "return_type": return_type,
                "doc": doc,
            }

        elif kind == 9:  # CALLBACK_INTERFACE
            module = r.string()
            name = r.string()
            doc = r.long_string()
            return {
                "kind": "callback_interface",
                "symbol": symbol_name,
                "module": module,
                "name": name,
                "doc": doc,
            }

        elif kind == 10:  # TRAIT_METHOD
            module = r.string()
            self_name = r.string()
            index = r.u32()
            name = r.string()
            is_async = r.bool()
            num_args = r.u8()
            args = []
            for _ in range(num_args):
                arg_name = r.string()
                arg_type = r.read_type()
                default_val = r.read_default_value()
                args.append({
                    "name": arg_name,
                    "type": arg_type,
                    "default": default_val,
                })
            return_type = r.read_type()
            doc = r.long_string()
            return {
                "kind": "trait_method",
                "symbol": symbol_name,
                "module": module,
                "interface": self_name,
                "index": index,
                "name": name,
                "is_async": is_async,
                "args": args,
                "return_type": None if return_type == "()" else return_type,
                "doc": doc,
            }

        elif kind == 11:  # UNIFFI_TRAIT
            trait_disc = r.u8()
            sub_kind = r.u8()  # 1 = METHOD
            module = r.string()
            self_name = r.string()
            return {
                "kind": "uniffi_trait",
                "symbol": symbol_name,
                "module": module,
                "interface": self_name,
                "trait_disc": trait_disc,
            }

        elif kind == 14:  # OBJECT_TRAIT_IMPL
            _t1_tag = r.u8()
            module = r.string()
            obj_name = r.string()
            _t2_tag = r.u8()
            _t2_mod = r.string()
            trait_name = r.string()
            return {
                "kind": "object_trait_impl",
                "symbol": symbol_name,
                "module": module,
                "object": obj_name,
                "trait": trait_name,
            }

        elif kind == 15:  # CUSTOM_TYPE
            module = r.string()
            name = r.string()
            builtin = r.read_type()
            doc = r.long_string()
            return {
                "kind": "custom_type",
                "symbol": symbol_name,
                "module": module,
                "name": name,
                "builtin": builtin,
                "doc": doc,
            }

        else:
            return {
                "kind": f"unknown_{kind}",
                "symbol": symbol_name,
                "raw_hex": buf.hex(),
            }
    except Exception as e:
        return {
            "kind": "decode_error",
            "symbol": symbol_name,
            "error": str(e),
            "raw_hex": buf.hex(),
        }


def format_doc(doc: str, indent: str = "") -> str:
    if not doc or not doc.strip():
        return ""
    lines = []
    for line in doc.strip().splitlines():
        lines.append(f"{indent}/// {line.strip()}")
    return "\n".join(lines) + "\n"


def format_return(ret_type) -> str:
    if ret_type and ret_type != "()":
        return f" -> {ret_type}"
    return ""


def generate_rust_idl(crate_name: str, items: list) -> str:
    records = [x for x in items if x.get("kind") == "record"]
    enums = [x for x in items if x.get("kind") == "enum"]
    interfaces = [x for x in items if x.get("kind") == "interface"]
    callbacks = [x for x in items if x.get("kind") == "callback_interface"]
    constructors = [x for x in items if x.get("kind") == "constructor"]
    methods = [x for x in items if x.get("kind") in ("method", "trait_method")]
    funcs = [x for x in items if x.get("kind") == "func"]

    # Group constructors and methods by interface
    ctors_by_iface = defaultdict(list)
    for c in constructors:
        ctors_by_iface[c["interface"]].append(c)
    methods_by_iface = defaultdict(list)
    for m in methods:
        methods_by_iface[m["interface"]].append(m)

    out = []
    out.append("// =============================================================================")
    out.append(f"// Reconstructed Rust UniFFI IDL & Schema for crate: `{crate_name}`")
    out.append("// Extracted from OKX Android v6.194.0 (lib/arm64-v8a/libOKCore.so)")
    out.append(f"// Total symbols decoded: {len(items)}")
    out.append(
        f"//   Records: {len(records)} | Enums: {len(enums)} | Interfaces: {len(interfaces)}"
    )
    out.append(
        f"//   Constructors: {len(constructors)} | Methods: {len(methods)} | Functions: {len(funcs)}"
    )
    out.append("// =============================================================================\n")
    out.append("use std::collections::HashMap;")
    out.append("use std::sync::Arc;")
    out.append("use std::time::{Duration, SystemTime};\n")

    out.append("// -----------------------------------------------------------------------------")
    out.append(f"// 1. Enums ({len(enums)})")
    out.append("// -----------------------------------------------------------------------------\n")
    for e in sorted(enums, key=lambda x: x["name"]):
        if e.get("doc"):
            out.append(format_doc(e.get("doc", "")).rstrip("\n"))
        attrs = ["#[derive(Debug, Clone, PartialEq)]"]
        if e.get("non_exhaustive"):
            attrs.append("#[non_exhaustive]")
        if e.get("discr_type"):
            attrs.append(f"#[repr({e['discr_type']})]")
        out.append("\n".join(attrs))
        out.append(f"pub enum {e['name']} {{")
        for v in e.get("variants", []):
            vdoc = format_doc(v.get("doc", ""), "    ")
            if vdoc:
                out.append(vdoc.rstrip("\n"))
            vfields = v.get("fields", [])
            if vfields:
                if all(f["name"] == "" or f["name"].isdigit() for f in vfields):
                    ftypes = ", ".join(f["type"] for f in vfields)
                    out.append(f"    {v['name']}({ftypes}),")
                else:
                    out.append(f"    {v['name']} {{")
                    for f in vfields:
                        fdoc = format_doc(f.get("doc", ""), "        ")
                        if fdoc:
                            out.append(fdoc.rstrip("\n"))
                        default_comment = f" // default = {f['default']}" if f.get("default") else ""
                        out.append(f"        {f['name']}: {f['type']},{default_comment}")
                    out.append("    },")
            else:
                discr_str = f" = {v['discr']}" if v.get("discr") is not None else ""
                out.append(f"    {v['name']}{discr_str},")
        out.append("}\n")

    out.append("// -----------------------------------------------------------------------------")
    out.append(f"// 2. Records / Data Models ({len(records)})")
    out.append("// -----------------------------------------------------------------------------\n")
    for rec in sorted(records, key=lambda x: x["name"]):
        if rec.get("doc"):
            out.append(format_doc(rec["doc"]).rstrip("\n"))
        out.append("#[derive(Debug, Clone, PartialEq)]")
        out.append(f"pub struct {rec['name']} {{")
        for f in rec.get("fields", []):
            fdoc = format_doc(f.get("doc", ""), "    ")
            if fdoc:
                out.append(fdoc.rstrip("\n"))
            default_comment = f" // default = {f['default']}" if f.get("default") else ""
            out.append(f"    pub {f['name']}: {f['type']},{default_comment}")
        out.append("}\n")

    out.append("// -----------------------------------------------------------------------------")
    out.append(f"// 3. Interfaces / Interactors / Services ({len(interfaces) + len(callbacks)})")
    out.append("// -----------------------------------------------------------------------------\n")
    all_ifaces = sorted(interfaces + callbacks, key=lambda x: x["name"])
    for iface in all_ifaces:
        iname = iface["name"]
        if iface.get("doc"):
            out.append(format_doc(iface["doc"]).rstrip("\n"))
        out.append(f"pub struct {iname};")
        out.append(f"impl {iname} {{")
        for ctor in sorted(ctors_by_iface.get(iname, []), key=lambda x: x["name"]):
            if ctor.get("doc"):
                out.append(format_doc(ctor["doc"], "    ").rstrip("\n"))
            async_kw = "async " if ctor.get("is_async") else ""
            args_str = ", ".join(
                f"{a['name']}: {a['type']}"
                + (f" /* = {a['default']} */" if a.get("default") else "")
                for a in ctor.get("args", [])
            )
            ret = format_return(ctor.get("return_type"))
            out.append(f"    pub {async_kw}fn {ctor['name']}({args_str}){ret} {{ todo!() }}")
        for m in sorted(methods_by_iface.get(iname, []), key=lambda x: x["name"]):
            if m.get("doc"):
                out.append(format_doc(m["doc"], "    ").rstrip("\n"))
            async_kw = "async " if m.get("is_async") else ""
            other_args = [
                f"{a['name']}: {a['type']}"
                + (f" /* = {a['default']} */" if a.get("default") else "")
                for a in m.get("args", [])
            ]
            all_args = ", ".join(["&self"] + other_args)
            ret = format_return(m.get("return_type"))
            out.append(f"    pub {async_kw}fn {m['name']}({all_args}){ret} {{ todo!() }}")
        out.append("}\n")

    out.append("// -----------------------------------------------------------------------------")
    out.append(f"// 4. Top-Level Exported Functions ({len(funcs)})")
    out.append("// -----------------------------------------------------------------------------\n")
    for fn in sorted(funcs, key=lambda x: x["name"]):
        if fn.get("doc"):
            out.append(format_doc(fn["doc"]).rstrip("\n"))
        async_kw = "async " if fn.get("is_async") else ""
        args_str = ", ".join(
            f"{a['name']}: {a['type']}"
            + (f" /* = {a['default']} */" if a.get("default") else "")
            for a in fn.get("args", [])
        )
        ret = format_return(fn.get("return_type"))
        out.append(f"pub {async_kw}fn {fn['name']}({args_str}){ret} {{ todo!() }}\n")

    return "\n".join(out)


def parse_elf_uniffi_symbols(so_path: str, out_dir: str):
    os.makedirs(out_dir, exist_ok=True)
    print(f"[*] Opening {so_path} ...")
    with open(so_path, "rb") as f:
        data = f.read()

    # Parse 64-bit ELF section headers
    e_shoff = struct.unpack_from("<Q", data, 40)[0]
    e_shentsize = struct.unpack_from("<H", data, 58)[0]
    e_shnum = struct.unpack_from("<H", data, 60)[0]
    e_shstrndx = struct.unpack_from("<H", data, 62)[0]

    sections = []
    for i in range(e_shnum):
        off = e_shoff + i * e_shentsize
        sh_name, sh_type, sh_flags, sh_addr, sh_offset, sh_size, sh_link, sh_info, sh_addralign, sh_entsize = (
            struct.unpack_from("<IIQQQQIIQQ", data, off)
        )
        sections.append({
            "name_off": sh_name,
            "type": sh_type,
            "addr": sh_addr,
            "offset": sh_offset,
            "size": sh_size,
            "entsize": sh_entsize,
        })

    shstrtab = data[sections[e_shstrndx]["offset"] : sections[e_shstrndx]["offset"] + sections[e_shstrndx]["size"]]
    sec_by_name = {}
    for s in sections:
        end = shstrtab.find(b"\x00", s["name_off"])
        name = shstrtab[s["name_off"] : end].decode("utf-8", errors="replace")
        s["name"] = name
        sec_by_name[name] = s

    dynsym = sec_by_name[".dynsym"]
    dynstr = sec_by_name[".dynstr"]
    dynstr_bytes = data[dynstr["offset"] : dynstr["offset"] + dynstr["size"]]

    def vaddr_to_offset(vaddr: int) -> int:
        for s in sections:
            if s["addr"] <= vaddr < s["addr"] + s["size"] and s["type"] != 8:
                return s["offset"] + (vaddr - s["addr"])
        return vaddr

    num_syms = dynsym["size"] // 24
    print(f"[*] Scanning {num_syms:,} dynamic symbols in .dynsym ...")

    meta_by_crate = defaultdict(list)
    crate_symbol_counts = defaultdict(int)

    for i in range(num_syms):
        off = dynsym["offset"] + i * 24
        st_name, st_info, st_other, st_shndx, st_value, st_size = struct.unpack_from("<IBBHQQ", data, off)
        if st_name == 0 or st_size == 0:
            continue
        if dynstr_bytes[st_name : st_name + 12] != b"UNIFFI_META_":
            continue
        end = dynstr_bytes.find(b"\x00", st_name)
        sym_name = dynstr_bytes[st_name:end].decode("utf-8", errors="replace")

        file_off = vaddr_to_offset(st_value)
        buf = data[file_off : file_off + st_size]
        decoded = decode_uniffi_meta(sym_name, buf)
        module = decoded.get("module") or "unknown"
        meta_by_crate[module].append(decoded)
        crate_symbol_counts[module] += 1

    print(f"[*] Found UniFFI metadata across {len(meta_by_crate)} Rust crates:")
    for mod, count in sorted(crate_symbol_counts.items(), key=lambda x: -x[1]):
        print(f"    - {mod}: {count:,} symbols")

    # Also scan embedded Rust source paths from .rodata
    rodata = sec_by_name[".rodata"]
    rodata_bytes = data[rodata["offset"] : rodata["offset"] + rodata["size"]]
    rs_paths = sorted(set(
        m.group(0).decode("utf-8", errors="replace")
        for m in re.finditer(b"components/[a-zA-Z0-9_./-]+\\.rs", rodata_bytes)
    ))
    print(f"[*] Extracted {len(rs_paths):,} embedded Rust source file paths (`components/.../*.rs`).")

    rs_by_crate = defaultdict(list)
    for p in rs_paths:
        parts = p.split("/")
        if len(parts) >= 3:
            crate_key = f"{parts[0]}/{parts[1]}/{parts[2]}"
            rs_by_crate[crate_key].append(p)

    inventory = {
        "binary": "lib/arm64-v8a/libOKCore.so",
        "size_bytes": len(data),
        "total_uniffi_crates": len(meta_by_crate),
        "total_uniffi_symbols": sum(crate_symbol_counts.values()),
        "total_rust_source_files": len(rs_paths),
        "uniffi_symbol_counts_by_crate": dict(sorted(crate_symbol_counts.items(), key=lambda x: -x[1])),
        "rust_source_files_by_component": {k: len(v) for k, v in sorted(rs_by_crate.items(), key=lambda x: -len(x[1]))},
        "trading_bot_source_files": rs_by_crate.get("components/biz-service/trading-bot", []),
        "social_trading_source_files": rs_by_crate.get("components/biz-service/social-trading", []),
    }
    inv_path = os.path.join(out_dir, "crate_inventory.json")
    with open(inv_path, "w", encoding="utf-8") as f:
        json.dump(inventory, f, indent=2, ensure_ascii=False)
    print(f"[+] Wrote {inv_path}")

    target_crates = [
        "trading_bot",
        "social_trading",
        "core_trading",
        "retail_trading",
        "buy_sell",
        "risk",
        "candle_support",
        "retail_kline",
    ]
    for crate in target_crates:
        items = meta_by_crate.get(crate, [])
        if not items:
            continue
        json_path = os.path.join(out_dir, f"{crate}_full_schema.json")
        with open(json_path, "w", encoding="utf-8") as f:
            json.dump(items, f, indent=2, ensure_ascii=False)
        rs_code = generate_rust_idl(crate, items)
        rs_path = os.path.join(out_dir, f"{crate}_full_schema.rs")
        with open(rs_path, "w", encoding="utf-8") as f:
            f.write(rs_code)
        errors = sum(1 for x in items if x.get("kind") == "decode_error")
        print(f"[+] Wrote {rs_path} ({len(items):,} items, {errors} decode errors)")


if __name__ == "__main__":
    so_file = sys.argv[1] if len(sys.argv) > 1 else "/Users/aaron/.gemini/antigravity/brain/55a4d47b-4c2a-4e95-a865-300431c4d9cd/scratch/libOKCore.so"
    out_directory = sys.argv[2] if len(sys.argv) > 2 else "/Users/aaron/Desktop/aaron/okx/reconstructed/rust_okcore_idl"
    parse_elf_uniffi_symbols(so_file, out_directory)
