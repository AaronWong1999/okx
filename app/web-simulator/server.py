#!/usr/bin/env python3
"""
OKX v6.194.0 Strategy Bot Simulator & Reverse-Engineering Workstation Server
============================================================================
Serves:
  - `/`                 -> Interactive 15-Bot Simulator & Workstation (`app/web-simulator/`)
  - `/screenshots/...`  -> Native 1180x2556 Screen Recording Screenshots (`策略BOT截图/`)
  - `/api/screenshots`  -> Full 239-screenshot manifest from `策略BOT截图/manifest.json`
  - `/api/reconstructed`-> Summary of decoded Rust UniFFI crates, DEX classes, and REA evidence
"""

import http.server
import json
import os
import socketserver
import urllib.parse
from pathlib import Path

WEB_DIR = Path(__file__).resolve().parent
PROJECT_ROOT = WEB_DIR.parent.parent
SCREENSHOTS_DIR = PROJECT_ROOT / "策略BOT截图"
RECONSTRUCTED_DIR = PROJECT_ROOT / "reconstructed"


class OkxWorkstationHandler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=str(WEB_DIR), **kwargs)

    def do_GET(self):
        parsed = urllib.parse.urlparse(self.path)
        req_path = urllib.parse.unquote(parsed.path)

        if req_path == "/api/screenshots":
            manifest_path = SCREENSHOTS_DIR / "manifest.json"
            if manifest_path.exists():
                data = manifest_path.read_bytes()
                self.send_response(200)
                self.send_header("Content-Type", "application/json; charset=utf-8")
                self.send_header("Content-Length", str(len(data)))
                self.end_headers()
                self.wfile.write(data)
                return
            self.send_error(404, "manifest.json not found")
            return

        if req_path == "/api/reconstructed":
            summary = {
                "apk_version": "6.194.0 (737202)",
                "rust_uniffi_crates": 54,
                "trading_bot_symbols": 4962,
                "dex_files": 67,
                "dex_bot_classes": 10290,
                "screenshots_total": 239,
                "folders_total": 15,
            }
            payload = json.dumps(summary, ensure_ascii=False).encode("utf-8")
            self.send_response(200)
            self.send_header("Content-Type", "application/json; charset=utf-8")
            self.send_header("Content-Length", str(len(payload)))
            self.end_headers()
            self.wfile.write(payload)
            return

        if req_path.startswith("/screenshots/"):
            rel = req_path[len("/screenshots/") :]
            target = (SCREENSHOTS_DIR / rel).resolve()
            if str(target).startswith(str(SCREENSHOTS_DIR.resolve())) and target.is_file():
                content = target.read_bytes()
                self.send_response(200)
                if target.suffix.lower() == ".png":
                    self.send_header("Content-Type", "image/png")
                elif target.suffix.lower() == ".json":
                    self.send_header("Content-Type", "application/json; charset=utf-8")
                else:
                    self.send_header("Content-Type", "application/octet-stream")
                self.send_header("Content-Length", str(len(content)))
                self.end_headers()
                self.wfile.write(content)
                return
            self.send_error(404, "Screenshot not found")
            return

        return super().do_GET()


def main():
    port = int(os.environ.get("PORT", "8088"))
    socketserver.TCPServer.allow_reuse_address = True
    with socketserver.TCPServer(("127.0.0.1", port), OkxWorkstationHandler) as httpd:
        print(f"OKX v6.194.0 Strategy Bot Workstation serving at http://127.0.0.1:{port}")
        httpd.serve_forever()


if __name__ == "__main__":
    main()
