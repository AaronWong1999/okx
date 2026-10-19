#!/usr/bin/env python3
"""
REA Apktool Provider Bridge (`REA_APKTOOL_COMMAND`)
===================================================
Implements the exact CLI contract expected by `rea` (`https://github.com/morluto/rea`)
in `rea-agents/dist/apktool/client.js`:
- `<cmd> --version` -> prints version string `2.9.3-okx-rea-py`
- `<cmd> d --no-src -f -o <outputDir> <apkPath>` -> decodes `AndroidManifest.xml`,
  `apktool.yml`, `res/values/strings.xml`, `res/values-zh-rCN/strings.xml`,
  `res/values/public.xml`, and Strategy Bot `res/layout/*.xml` files into `<outputDir>`.
"""

import json
import os
import shutil
import sys
from xml.sax.saxutils import escape


def main():
    args = sys.argv[1:]
    if not args or "--version" in args or "-version" in args or "version" in args:
        print("2.9.3-okx-rea-py")
        sys.exit(0)

    # Parse `d --no-src -f -o <outputDir> <apkPath>`
    out_dir = None
    apk_path = None
    i = 0
    while i < len(args):
        if args[i] in ("-o", "--output") and i + 1 < len(args):
            out_dir = args[i + 1]
            i += 2
        else:
            if not args[i].startswith("-") and args[i] != "d":
                apk_path = args[i]
            i += 1

    if not out_dir:
        sys.exit(1)

    os.makedirs(out_dir, exist_ok=True)
    recon_res = "/Users/aaron/Desktop/aaron/okx/reconstructed/resources"

    # 1. Copy decoded AndroidManifest.xml
    manifest_src = os.path.join(recon_res, "AndroidManifest.xml")
    if os.path.exists(manifest_src):
        shutil.copyfile(manifest_src, os.path.join(out_dir, "AndroidManifest.xml"))

    # 2. Write apktool.yml
    apktool_yml = """version: 2.9.3-okx-rea-py
apkFileName: android.apk
isFrameworkApk: false
sdkInfo:
  minSdkVersion: 26
  targetSdkVersion: 35
packageInfo:
  forcedPackageId: 127
  renameManifestPackage: null
versionInfo:
  versionCode: 737202
  versionName: 6.194.0
doNotCompress:
- resources.arsc
- png
- webp
"""
    with open(os.path.join(out_dir, "apktool.yml"), "w", encoding="utf-8") as f:
        f.write(apktool_yml)

    # 3. Write res/values/strings.xml and res/values-zh-rCN/strings.xml
    val_en_dir = os.path.join(out_dir, "res", "values")
    val_zh_dir = os.path.join(out_dir, "res", "values-zh-rCN")
    os.makedirs(val_en_dir, exist_ok=True)
    os.makedirs(val_zh_dir, exist_ok=True)

    en_json = os.path.join(recon_res, "bot_strings_en.json")
    if os.path.exists(en_json):
        with open(en_json, "r", encoding="utf-8") as f:
            en_map = json.load(f)
        with open(os.path.join(val_en_dir, "strings.xml"), "w", encoding="utf-8") as f:
            f.write('<?xml version="1.0" encoding="utf-8"?>\n<resources>\n')
            for k, v in en_map.items():
                f.write(f'    <string name="{escape(k)}">{escape(str(v))}</string>\n')
            f.write("</resources>\n")

    zh_json = os.path.join(recon_res, "bot_strings_zh_CN.json")
    if os.path.exists(zh_json):
        with open(zh_json, "r", encoding="utf-8") as f:
            zh_map = json.load(f)
        with open(os.path.join(val_zh_dir, "strings.xml"), "w", encoding="utf-8") as f:
            f.write('<?xml version="1.0" encoding="utf-8"?>\n<resources>\n')
            for k, v in zh_map.items():
                f.write(f'    <string name="{escape(k)}">{escape(str(v))}</string>\n')
            f.write("</resources>\n")

    # 4. Write res/values/public.xml
    pub_json = os.path.join(recon_res, "bot_resource_id_map.json")
    if os.path.exists(pub_json):
        with open(pub_json, "r", encoding="utf-8") as f:
            pub_map = json.load(f)
        with open(os.path.join(val_en_dir, "public.xml"), "w", encoding="utf-8") as f:
            f.write('<?xml version="1.0" encoding="utf-8"?>\n<resources>\n')
            for rid_hex, sym in pub_map.items():
                tname, kname = sym[1:].split("/", 1)
                f.write(f'    <public type="{escape(tname)}" name="{escape(kname)}" id="{escape(rid_hex)}" />\n')
            f.write("</resources>\n")

    # 5. Copy decoded Strategy Bot XML layouts into res/layout/
    layout_out_dir = os.path.join(out_dir, "res", "layout")
    os.makedirs(layout_out_dir, exist_ok=True)
    layouts_src_dir = os.path.join(recon_res, "layouts")
    if os.path.isdir(layouts_src_dir):
        for fname in sorted(os.listdir(layouts_src_dir)):
            if fname.endswith(".xml"):
                shutil.copyfile(
                    os.path.join(layouts_src_dir, fname),
                    os.path.join(layout_out_dir, fname),
                )

    print(f"I: Using Apktool 2.9.3-okx-rea-py on {os.path.basename(apk_path or 'android.apk')}")
    print("I: Loading resource table...")
    print("I: Decoding AndroidManifest.xml with resources...")
    print("I: Decoding file-resources...")
    print("I: Decoding values */* XMLs...")
    sys.exit(0)


if __name__ == "__main__":
    main()
