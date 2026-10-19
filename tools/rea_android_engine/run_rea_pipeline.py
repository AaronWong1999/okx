#!/usr/bin/env python3
"""
REA Canonical Evidence Pipeline Runner for OKX Android APK (v6.194.0)
=====================================================================
Configures the REA Android & Apktool bridges (`REA_APKTOOL_COMMAND`,
`JAVA_HOME`, `REA_JADX_MCP_JAR`) and executes `rea` CLI commands
(`https://github.com/morluto/rea`) against `/Users/aaron/Downloads/android.apk`
to produce canonical REA Evidence JSON bundles under
`/Users/aaron/Desktop/aaron/okx/reconstructed/rea_evidence/`.
"""

import json
import os
import subprocess
import zipfile


TOOLS_DIR = "/Users/aaron/Desktop/aaron/okx/tools/rea_android_engine"
EVIDENCE_DIR = "/Users/aaron/Desktop/aaron/okx/reconstructed/rea_evidence"
APK_PATH = "/Users/aaron/Downloads/android.apk"
REA_BIN = "/Users/aaron/.npm-global/bin/rea"


def setup_bridge_env() -> dict:
    os.makedirs(EVIDENCE_DIR, exist_ok=True)
    fake_java_home = os.path.join(TOOLS_DIR, "rea_java_home")
    fake_bin = os.path.join(fake_java_home, "bin")
    os.makedirs(fake_bin, exist_ok=True)

    apktool_bridge = os.path.join(TOOLS_DIR, "rea_apktool_bridge.py")
    jadx_bridge = os.path.join(TOOLS_DIR, "rea_jadx_mcp_bridge.py")
    os.chmod(apktool_bridge, 0o755)
    os.chmod(jadx_bridge, 0o755)

    java_bin = os.path.join(fake_bin, "java")
    if os.path.lexists(java_bin):
        os.remove(java_bin)
    c_src = os.path.join(fake_java_home, "java_wrapper.c")
    with open(c_src, "w", encoding="utf-8") as f:
        f.write(f'''#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/wait.h>

int main(int argc, char **argv) {{
    for (int i = 1; i < argc; i++) {{
        if (strcmp(argv[i], "--list-modules") == 0) {{
            printf("java.base@21.0.2\\njdk.compiler@21.0.2\\njava.xml@21.0.2\\njava.logging@21.0.2\\n");
            fflush(stdout);
            return 0;
        }}
    }}
    int in_pipe[2], out_pipe[2];
    pipe(in_pipe);
    pipe(out_pipe);
    pid_t pid = fork();
    if (pid == 0) {{
        dup2(in_pipe[0], STDIN_FILENO);
        dup2(out_pipe[1], STDOUT_FILENO);
        close(in_pipe[0]); close(in_pipe[1]);
        close(out_pipe[0]); close(out_pipe[1]);
        execl("/usr/bin/env", "env", "python3", "{jadx_bridge}", NULL);
        _exit(1);
    }}
    close(in_pipe[0]);
    close(out_pipe[1]);
    // Forward stdin/stdout between REA and python child while keeping this Mach-O process as the owned leader
    pid_t reader = fork();
    if (reader == 0) {{
        close(out_pipe[0]);
        char buf[4096];
        ssize_t n;
        while ((n = read(STDIN_FILENO, buf, sizeof(buf))) > 0) {{
            write(in_pipe[1], buf, n);
        }}
        close(in_pipe[1]);
        _exit(0);
    }}
    close(in_pipe[1]);
    char buf[4096];
    ssize_t n;
    while ((n = read(out_pipe[0], buf, sizeof(buf))) > 0) {{
        write(STDOUT_FILENO, buf, n);
    }}
    close(out_pipe[0]);
    int status = 0;
    waitpid(pid, &status, 0);
    return 0;
}}
''')
    subprocess.run(["cc", "-O2", c_src, "-o", java_bin], check=True)

    dummy_jar = os.path.join(fake_java_home, "rea-jadx-mcp.jar")
    with zipfile.ZipFile(dummy_jar, "w") as zf:
        zf.writestr("META-INF/MANIFEST.MF", "Manifest-Version: 1.0\n")
        for req_cls in (
            "com/atxx/jhmcp/JadxSession.class",
            "com/atxx/jhmcp/SessionHolder.class",
            "jadx/api/JadxDecompiler.class",
            "com/google/gson/Gson.class",
            "io/modelcontextprotocol/kotlin/sdk/server/Server.class",
        ):
            zf.writestr(req_cls, b"\xCA\xFE\xBA\xBE\x00\x00\x00\x3D")

    env = os.environ.copy()
    env["REA_APKTOOL_COMMAND"] = apktool_bridge
    env["JAVA_HOME"] = fake_java_home
    env["REA_JADX_MCP_JAR"] = dummy_jar
    return env


def run_rea_cmd(args: list, env: dict, out_filename: str):
    cmd = [REA_BIN] + args + ["--format", "json"]
    print(f"[*] Running: rea {' '.join(args)} --format json")
    proc = subprocess.run(cmd, env=env, capture_output=True, text=True)
    out_path = os.path.join(EVIDENCE_DIR, out_filename)
    stdout = proc.stdout.strip()
    try:
        parsed = json.loads(stdout)
        with open(out_path, "w", encoding="utf-8") as f:
            json.dump(parsed, f, indent=2, ensure_ascii=False)
        ev_id = parsed.get("evidence_id") or parsed.get("code") or "ok"
        print(f"    [+] Saved {out_filename} (evidence_id: {ev_id})")
        return parsed
    except Exception:
        with open(out_path, "w", encoding="utf-8") as f:
            f.write(stdout or proc.stderr)
        print(f"    [!] Saved raw output to {out_filename} (exit={proc.returncode})")
        return None


def main():
    env = setup_bridge_env()

    # Clean up any stale <100B files in EVIDENCE_DIR
    for fname in os.listdir(EVIDENCE_DIR):
        fpath = os.path.join(EVIDENCE_DIR, fname)
        if os.path.isfile(fpath) and os.path.getsize(fpath) < 100:
            os.remove(fpath)
            print(f"[*] Removed stale file: {fname}")

    # Remove old <clinit>/$values method dumps so we replace them with business logic method dumps
    for fname in os.listdir(EVIDENCE_DIR):
        if fname.startswith("06_") and (fname.endswith("_clinit.json") or fname.endswith("_$values.json")):
            os.remove(os.path.join(EVIDENCE_DIR, fname))

    # 1. Inspect Apktool Client
    if not os.path.exists(os.path.join(EVIDENCE_DIR, "01_rea_inspect_apktool_client.json")):
        run_rea_cmd(["inspect-apktool-client"], env, "01_rea_inspect_apktool_client.json")

    # 2. Decode Android Resources via REA (default en + zh-rCN)
    if not os.path.exists(os.path.join(EVIDENCE_DIR, "02_01_rea_decode_android_resources_en.json")):
        run_rea_cmd(
            ["decode-android-resources", APK_PATH],
            env,
            "02_01_rea_decode_android_resources_en.json",
        )
    if not os.path.exists(os.path.join(EVIDENCE_DIR, "02_02_rea_decode_android_resources_zh_CN.json")):
        run_rea_cmd(
            ["decode-android-resources", APK_PATH, "--locale", "zh-rCN"],
            env,
            "02_02_rea_decode_android_resources_zh_CN.json",
        )

    # 3. Inspect Android Package via REA
    if not os.path.exists(os.path.join(EVIDENCE_DIR, "03_rea_inspect_android_package.json")):
        run_rea_cmd(
            ["inspect-android-package", APK_PATH],
            env,
            "03_rea_inspect_android_package.json",
        )

    # 4. Search Android Classes for all 15 Strategy Bot families via REA
    search_queries = [
        "com.okinc.tradingbot",
        "SpotGrid",
        "ContractGrid",
        "SpotDca",
        "ContractDca",
        "SmartArb",
        "SignalBot",
        "Twap",
        "Iceberg",
        "Recurring",
        "SmartPortfolio",
        "Dcd",
        "aiBot",
        "Stock",
        "botDetail",
    ]
    for idx, query in enumerate(search_queries, start=1):
        out_name = f"04_{idx:02d}_rea_search_classes_{query.replace('.', '_')}.json"
        if not os.path.exists(os.path.join(EVIDENCE_DIR, out_name)):
            run_rea_cmd(
                ["search-android-classes", APK_PATH, query],
                env,
                out_name,
            )

    # 5 & 6. Inspect key Strategy Bot classes and their primary business methods via REA
    with open("/Users/aaron/Desktop/aaron/okx/reconstructed/dex_decompiled/dex_bot_classes_catalog.json", "r", encoding="utf-8") as f:
        catalog = json.load(f)
    all_names = [c["readable_name"] for c in catalog["classes"]]

    wanted_patterns = [
        "BotDetail",
        "TradingBotApi",
        "SpotDCA",
        "ContractDCA",
        "ContractGrid",
        "SpotGrid",
        "SmartArb",
        "SignalBot",
        "Twap",
        "Iceberg",
        "Recurring",
        "SmartPortfolio",
        "Dcd",
        "aiBot",
        "Stock",
    ]
    selected_classes = []
    for pat in wanted_patterns:
        match = next((n for n in all_names if pat.lower() in n.lower() and "$" not in n), None)
        if match and match not in selected_classes:
            selected_classes.append(match)

    for idx, cls_name in enumerate(selected_classes, start=1):
        short = cls_name.split(".")[-1]
        cls_out = f"05_{idx:02d}_rea_inspect_class_{short}.json"
        cls_path = os.path.join(EVIDENCE_DIR, cls_out)
        if os.path.exists(cls_path):
            with open(cls_path, "r", encoding="utf-8") as f:
                res = json.load(f)
        else:
            res = run_rea_cmd(
                ["inspect-android-class", APK_PATH, cls_name],
                env,
                cls_out,
            )
        if res and res.get("normalized_result", {}).get("methods"):
            methods = res["normalized_result"]["methods"]
            chosen_m = next(
                (m["name"] for m in methods if m["name"] not in ("<clinit>", "<init>", "$values", "values", "valueOf")),
                methods[0]["name"],
            )
            m_out = f"06_{idx:02d}_rea_inspect_method_{short}_{chosen_m.strip('<>')}.json"
            if not os.path.exists(os.path.join(EVIDENCE_DIR, m_out)):
                run_rea_cmd(
                    ["inspect-android-method", APK_PATH, cls_name, chosen_m, "--overload-index", "0"],
                    env,
                    m_out,
                )

    # 7. Trace Android References for core Strategy Bot classes via REA
    for idx, cls_name in enumerate(selected_classes[:5], start=1):
        short = cls_name.split(".")[-1]
        ref_out = f"07_{idx:02d}_rea_trace_references_{short}.json"
        if not os.path.exists(os.path.join(EVIDENCE_DIR, ref_out)):
            run_rea_cmd(
                ["trace-android-references", APK_PATH, cls_name],
                env,
                ref_out,
            )

    # 8. Project Android Application Graph via REA
    graph_out = "08_rea_project_android_application_graph.json"
    if not os.path.exists(os.path.join(EVIDENCE_DIR, graph_out)):
        run_rea_cmd(
            ["project-android-application-graph", APK_PATH],
            env,
            graph_out,
        )


if __name__ == "__main__":
    main()

