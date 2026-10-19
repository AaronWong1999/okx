#!/usr/bin/env python3
"""
OKX Android AXML & resources.arsc Decoder
=========================================
Decodes binary Android XML (`AndroidManifest.xml` and `res/layout/*.xml`) and
`resources.arsc` (`81.37 MB`) from `/Users/aaron/Downloads/android.apk`.
Resolves resource IDs (`0x7fXXXXXX`) to symbolic `@type/name` references and
extracts localized `zh-CN` and `en` string tables for all Strategy Bot modules.
"""

import json
import os
import re
import struct
import sys
import zipfile
from collections import defaultdict
from xml.sax.saxutils import escape


ANDROID_ATTR_NAMES = {
    0x01010000: "theme",
    0x01010001: "label",
    0x01010002: "icon",
    0x01010003: "name",
    0x0101000E: "enabled",
    0x01010010: "exported",
    0x01010011: "process",
    0x01010018: " authorities",
    0x0101001D: "launchMode",
    0x0101001E: "screenOrientation",
    0x0101001F: "configChanges",
    0x01010027: "scheme",
    0x01010028: "host",
    0x0101002A: "path",
    0x0101002B: "pathPrefix",
    0x0101002C: "pathPattern",
    0x01010063: "id",
    0x01010064: "tag",
    0x0101006A: "padding",
    0x0101006B: "paddingLeft",
    0x0101006C: "paddingTop",
    0x0101006D: "paddingRight",
    0x0101006E: "paddingBottom",
    0x0101007F: "visibility",
    0x01010095: "textSize",
    0x01010097: "textStyle",
    0x01010098: "textColor",
    0x0101009A: "textColorHint",
    0x010100AF: "gravity",
    0x010100C4: "orientation",
    0x010100D0: "id",
    0x010100D4: "background",
    0x010100F4: "layout_width",
    0x010100F5: "layout_height",
    0x010100F6: "layout_margin",
    0x010100F7: "layout_marginLeft",
    0x010100F8: "layout_marginTop",
    0x010100F9: "layout_marginRight",
    0x010100FA: "layout_marginBottom",
    0x01010119: "src",
    0x0101011F: "maxWidth",
    0x01010120: "maxHeight",
    0x0101013F: "minWidth",
    0x01010140: "minHeight",
    0x0101014F: "text",
    0x01010150: "hint",
    0x01010153: "maxLines",
    0x0101015B: "singleLine",
    0x0101015D: "inputType",
    0x01010181: "layout_weight",
    0x010103B3: "paddingStart",
    0x010103B4: "paddingEnd",
    0x010103B5: "layout_marginStart",
    0x010103B6: "layout_marginEnd",
}


def parse_string_pool(data: bytes, offset: int) -> list:
    _type, _header_size, _size, string_count, _style_count, flags, strings_start, _styles_start = (
        struct.unpack_from("<HHIIIIII", data, offset)
    )
    is_utf8 = (flags & (1 << 8)) != 0
    offsets_base = offset + 28
    strings_base = offset + strings_start
    strings = []
    for i in range(string_count):
        str_off = struct.unpack_from("<I", data, offsets_base + i * 4)[0]
        pos = strings_base + str_off
        if is_utf8:
            u16_len = data[pos]
            pos += 1
            if u16_len & 0x80:
                pos += 1
            u8_len = data[pos]
            pos += 1
            if u8_len & 0x80:
                u8_len = ((u8_len & 0x7F) << 8) | data[pos]
                pos += 1
            s = data[pos : pos + u8_len].decode("utf-8", errors="replace")
        else:
            u16_len = struct.unpack_from("<H", data, pos)[0]
            pos += 2
            if u16_len & 0x8000:
                u16_len = ((u16_len & 0x7FFF) << 16) | struct.unpack_from("<H", data, pos)[0]
                pos += 2
            s = data[pos : pos + u16_len * 2].decode("utf-16le", errors="replace")
        strings.append(s)
    return strings


def parse_arsc(arsc_bytes: bytes):
    """
    Parses resources.arsc and returns:
    - res_id_to_symbol: {0x7f010001: ("string", "bot_name")}
    - zh_cn_strings: {"bot_name": "..."}
    - en_strings: {"bot_name": "..."}
    """
    global_strings = parse_string_pool(arsc_bytes, 12)
    first_chunk_size = struct.unpack_from("<I", arsc_bytes, 16)[0]
    pos = 12 + first_chunk_size

    res_id_to_symbol = {}
    zh_cn_strings = {}
    en_strings = {}
    default_strings = {}
    layout_files = {}

    while pos < len(arsc_bytes):
        ctype, cheader_size, csize = struct.unpack_from("<HHI", arsc_bytes, pos)
        if ctype == 0x0200:  # RES_TABLE_PACKAGE_TYPE
            pkg_id = struct.unpack_from("<I", arsc_bytes, pos + 8)[0]
            type_strings_off = struct.unpack_from("<I", arsc_bytes, pos + 268)[0]
            key_strings_off = struct.unpack_from("<I", arsc_bytes, pos + 276)[0]
            type_strings = parse_string_pool(arsc_bytes, pos + type_strings_off)
            key_strings = parse_string_pool(arsc_bytes, pos + key_strings_off)

            sub_pos = pos + cheader_size
            pkg_end = pos + csize
            while sub_pos < pkg_end:
                st_type, st_hsize, st_size = struct.unpack_from("<HHI", arsc_bytes, sub_pos)
                if st_type == 0x0201:  # RES_TABLE_TYPE_TYPE
                    type_id = arsc_bytes[sub_pos + 8]
                    res_flags = arsc_bytes[sub_pos + 9]
                    entry_count = struct.unpack_from("<I", arsc_bytes, sub_pos + 12)[0]
                    entries_start = struct.unpack_from("<I", arsc_bytes, sub_pos + 16)[0]
                    lang = arsc_bytes[sub_pos + 28 : sub_pos + 30].decode("ascii", errors="ignore").strip("\x00")
                    country = arsc_bytes[sub_pos + 30 : sub_pos + 32].decode("ascii", errors="ignore").strip("\x00")
                    type_name = type_strings[type_id - 1] if 0 < type_id <= len(type_strings) else f"type_{type_id}"

                    is_sparse = (res_flags & 0x01) != 0
                    is_offset16 = (res_flags & 0x02) != 0
                    offsets_base = sub_pos + st_hsize
                    entries_base = sub_pos + entries_start

                    for i in range(entry_count):
                        if is_sparse:
                            entry_idx, off16 = struct.unpack_from("<HH", arsc_bytes, offsets_base + i * 4)
                            entry_off = off16 * 4
                        elif is_offset16:
                            entry_idx = i
                            off16 = struct.unpack_from("<H", arsc_bytes, offsets_base + i * 2)[0]
                            if off16 == 0xFFFF:
                                continue
                            entry_off = off16 * 4
                        else:
                            entry_idx = i
                            entry_off = struct.unpack_from("<I", arsc_bytes, offsets_base + i * 4)[0]
                            if entry_off == 0xFFFFFFFF:
                                continue

                        epos = entries_base + entry_off
                        if epos + 8 > pkg_end:
                            continue
                        esize, eflags, key_idx = struct.unpack_from("<HHI", arsc_bytes, epos)
                        is_compact = (eflags & 0x04) != 0
                        is_complex = (eflags & 0x01) != 0

                        if is_compact:
                            real_key_idx = esize
                            data_type = (eflags >> 8) & 0xFF
                            data_val = key_idx
                        else:
                            real_key_idx = key_idx
                            if not is_complex and epos + esize + 8 <= pkg_end:
                                _vsize, _vres0, data_type, data_val = struct.unpack_from("<HBBI", arsc_bytes, epos + esize)
                            else:
                                data_type = 0
                                data_val = 0

                        res_id = (pkg_id << 24) | (type_id << 16) | entry_idx
                        if type_name == "layout" and data_type == 0x03 and 0 <= data_val < len(global_strings):
                            layout_files.setdefault(res_id, global_strings[data_val])

                        if 0 <= real_key_idx < len(key_strings):
                            kname = key_strings[real_key_idx]
                            if kname:
                                res_id_to_symbol.setdefault(res_id, (type_name, kname))

                            if type_name == "string" and data_type == 0x03 and 0 <= data_val < len(global_strings):
                                sval = global_strings[data_val]
                                if lang == "zh" and country in ("CN", ""):
                                    zh_cn_strings[kname] = sval
                                elif lang == "en":
                                    en_strings.setdefault(kname, sval)
                                elif lang == "":
                                    default_strings.setdefault(kname, sval)

                sub_pos += st_size
        pos += csize

    for k, v in default_strings.items():
        en_strings.setdefault(k, v)
        zh_cn_strings.setdefault(k, v)

    return res_id_to_symbol, zh_cn_strings, en_strings, layout_files


def decode_axml(axml_bytes: bytes, res_id_to_symbol: dict = None) -> str:
    """Decodes binary Android XML (AXML) into readable XML text."""
    if len(axml_bytes) < 8:
        return ""
    magic, _file_size = struct.unpack_from("<II", axml_bytes, 0)
    if magic != 0x00080003:
        return ""

    pos = 8
    strings = []
    res_ids = []
    lines = ['<?xml version="1.0" encoding="utf-8"?>']
    indent = 0
    ns_declared = False

    def format_attr_val(vtype: int, vdata: int, raw_idx: int) -> str:
        if 0 <= raw_idx < len(strings) and vtype == 0x03:
            return strings[raw_idx]
        if vtype == 0x01:  # Reference
            if res_id_to_symbol and vdata in res_id_to_symbol:
                tname, kname = res_id_to_symbol[vdata]
                return f"@{tname}/{kname}"
            return f"@0x{vdata:08x}"
        elif vtype == 0x02:  # Attribute
            if res_id_to_symbol and vdata in res_id_to_symbol:
                tname, kname = res_id_to_symbol[vdata]
                return f"?{tname}/{kname}"
            return f"?0x{vdata:08x}"
        elif vtype == 0x03:  # String
            return strings[vdata] if 0 <= vdata < len(strings) else ""
        elif vtype == 0x04:  # Float
            return f"{struct.unpack('<f', struct.pack('<I', vdata))[0]:.4g}"
        elif vtype == 0x05:  # Dimension
            units = ["px", "dp", "sp", "pt", "in", "mm"]
            unit = units[vdata & 0xF] if (vdata & 0xF) < len(units) else "px"
            mantissa = (vdata >> 8)
            if mantissa & 0x800000:
                mantissa -= 0x1000000
            radix_shifts = [0, 7, 15, 23]
            shift = radix_shifts[(vdata >> 4) & 0x3]
            val = mantissa / float(1 << shift)
            return f"{val:g}{unit}"
        elif vtype == 0x10:  # Int dec
            if vdata == 0xFFFFFFFF:
                return "match_parent"
            elif vdata == 0xFFFFFFFE:
                return "wrap_content"
            return str(struct.unpack("<i", struct.pack("<I", vdata))[0])
        elif vtype == 0x11:  # Int hex
            return f"0x{vdata:x}"
        elif vtype == 0x12:  # Boolean
            return "true" if vdata != 0 else "false"
        elif 0x1C <= vtype <= 0x1F:  # Color
            return f"#{vdata:08x}"
        return f"0x{vdata:x}"

    while pos < len(axml_bytes):
        ctype, cheader_size, csize = struct.unpack_from("<HHI", axml_bytes, pos)
        if csize == 0:
            break
        if ctype == 0x0001:  # StringPool
            strings = parse_string_pool(axml_bytes, pos)
        elif ctype == 0x0180:  # ResourceMap
            count = (csize - cheader_size) // 4
            res_ids = list(struct.unpack_from(f"<{count}I", axml_bytes, pos + cheader_size))
        elif ctype == 0x0102:  # START_TAG
            _line_no, _comment, _ns_idx, name_idx, attr_start, attr_size, attr_count = (
                struct.unpack_from("<IIIIHHH", axml_bytes, pos + 8)
            )
            tag_name = strings[name_idx] if 0 <= name_idx < len(strings) else "View"
            attrs = []
            if not ns_declared:
                attrs.append('xmlns:android="http://schemas.android.com/apk/res/android"')
                attrs.append('xmlns:app="http://schemas.android.com/apk/res-auto"')
                ns_declared = True
            abase = pos + 16 + attr_start
            for i in range(attr_count):
                aoff = abase + i * attr_size
                ans_idx, aname_idx, araw_idx, _asize, _ares0, avtype, avdata = (
                    struct.unpack_from("<IIIHBBI", axml_bytes, aoff)
                )
                aname = strings[aname_idx] if 0 <= aname_idx < len(strings) else ""
                if not aname and 0 <= aname_idx < len(res_ids):
                    rid = res_ids[aname_idx]
                    if rid in ANDROID_ATTR_NAMES:
                        aname = f"android:{ANDROID_ATTR_NAMES[rid]}"
                    elif res_id_to_symbol and rid in res_id_to_symbol:
                        aname = f"app:{res_id_to_symbol[rid][1]}"
                    else:
                        aname = f"attr_0x{rid:08x}"
                elif ans_idx != 0xFFFFFFFF and not aname.startswith(("android:", "app:")):
                    aname = f"android:{aname}"
                aval = format_attr_val(avtype, avdata, araw_idx)
                attrs.append(f'{aname}="{escape(aval)}"')
            attr_str = (" " + " ".join(attrs)) if attrs else ""
            lines.append("  " * indent + f"<{tag_name}{attr_str}>")
            indent += 1
        elif ctype == 0x0103:  # END_TAG
            indent = max(0, indent - 1)
            name_idx = struct.unpack_from("<I", axml_bytes, pos + 20)[0]
            tag_name = strings[name_idx] if 0 <= name_idx < len(strings) else "View"
            lines.append("  " * indent + f"</{tag_name}>")
        pos += csize

    return "\n".join(lines)


def main():
    apk_path = sys.argv[1] if len(sys.argv) > 1 else "/Users/aaron/Downloads/android.apk"
    out_dir = sys.argv[2] if len(sys.argv) > 2 else "/Users/aaron/Desktop/aaron/okx/reconstructed/resources"
    os.makedirs(out_dir, exist_ok=True)
    layouts_dir = os.path.join(out_dir, "layouts")
    os.makedirs(layouts_dir, exist_ok=True)

    print(f"[*] Opening {apk_path} ...")
    zf = zipfile.ZipFile(apk_path, "r")

    print("[*] Parsing resources.arsc (81.37 MB) ...")
    arsc_bytes = zf.read("resources.arsc")
    res_id_to_symbol, zh_cn_strings, en_strings, layout_files = parse_arsc(arsc_bytes)
    print(
        f"[*] Parsed {len(res_id_to_symbol):,} resource IDs, "
        f"{len(zh_cn_strings):,} zh-CN strings, {len(en_strings):,} en strings, "
        f"{len(layout_files):,} layouts."
    )

    # Filter Strategy Bot / Trading Bot strings
    bot_kw = re.compile(
        r"(bot|stgy|strategy|grid|dca|martingale|arb|twap|iceberg|recurring|portfolio|signal|dcd|flywheel|sniper|kline_bot|mp_bot|ai_bot)",
        re.IGNORECASE,
    )
    bot_zh = {k: v for k, v in sorted(zh_cn_strings.items()) if bot_kw.search(k)}
    bot_en = {k: v for k, v in sorted(en_strings.items()) if bot_kw.search(k)}

    with open(os.path.join(out_dir, "bot_strings_zh_CN.json"), "w", encoding="utf-8") as f:
        json.dump(bot_zh, f, indent=2, ensure_ascii=False)
    with open(os.path.join(out_dir, "bot_strings_en.json"), "w", encoding="utf-8") as f:
        json.dump(bot_en, f, indent=2, ensure_ascii=False)
    print(f"[+] Wrote bot_strings_zh_CN.json ({len(bot_zh):,} keys) & bot_strings_en.json ({len(bot_en):,} keys)")

    # Save Bot Resource ID map (`0x7f... -> @type/name`)
    bot_res_ids = {
        f"0x{rid:08x}": f"@{tname}/{kname}"
        for rid, (tname, kname) in sorted(res_id_to_symbol.items())
        if bot_kw.search(kname)
    }
    with open(os.path.join(out_dir, "bot_resource_id_map.json"), "w", encoding="utf-8") as f:
        json.dump(bot_res_ids, f, indent=2, ensure_ascii=False)
    print(f"[+] Wrote bot_resource_id_map.json ({len(bot_res_ids):,} resource IDs)")

    # Decode AndroidManifest.xml
    print("[*] Decoding AndroidManifest.xml ...")
    manifest_xml = decode_axml(zf.read("AndroidManifest.xml"), res_id_to_symbol)
    manifest_path = os.path.join(out_dir, "AndroidManifest.xml")
    with open(manifest_path, "w", encoding="utf-8") as f:
        f.write(manifest_xml)
    print(f"[+] Wrote {manifest_path} ({len(manifest_xml.splitlines()):,} lines)")

    # Scan and decode Strategy Bot layout XMLs from the 9,334 layout entries in resources.arsc
    print(f"[*] Scanning {len(layout_files):,} layout entries from resources.arsc for Strategy Bot layouts ...")
    bot_layouts_decoded = 0
    layout_index = {}
    zip_names = set(zf.namelist())

    for rid, entry in sorted(layout_files.items()):
        if entry not in zip_names:
            continue
        raw = zf.read(entry)
        if len(raw) < 16 or struct.unpack_from("<I", raw, 0)[0] != 0x00080003:
            continue
        xml_text = decode_axml(raw, res_id_to_symbol)
        if any(
            marker in xml_text
            for marker in (
                "tradingbot",
                "stgy_",
                "bot_detail",
                "spot_grid",
                "contract_grid",
                "futures_grid",
                "spot_dca",
                "contract_dca",
                "smart_arb",
                "smart_portfolio",
                "recurring_buy",
                "signal_bot",
                "twap_",
                "iceberg_",
                "flywheel",
                "grid_num",
                "tp_sl",
                "liquidate",
            )
        ):
            id_matches = re.findall(r"@id/([a-zA-Z0-9_]+)", xml_text)
            lead_id = id_matches[0] if id_matches else entry.replace("/", "_")
            safe_name = f"0x{rid:08x}__{entry.replace('/', '_')}__{lead_id}.xml"
            out_xml_path = os.path.join(layouts_dir, safe_name)
            with open(out_xml_path, "w", encoding="utf-8") as f:
                f.write(f"<!-- Layout Resource ID: 0x{rid:08x} | APK Entry: {entry} -->\n")
                f.write(xml_text)
            layout_index[f"0x{rid:08x}"] = {
                "apk_entry": entry,
                "decoded_file": f"layouts/{safe_name}",
                "ids": id_matches[:30],
            }
            bot_layouts_decoded += 1
            if bot_layouts_decoded >= 250:
                break

    with open(os.path.join(out_dir, "bot_layouts_index.json"), "w", encoding="utf-8") as f:
        json.dump(layout_index, f, indent=2, ensure_ascii=False)
    print(f"[+] Decoded {bot_layouts_decoded} Strategy Bot XML layouts into {layouts_dir}")


if __name__ == "__main__":
    main()
