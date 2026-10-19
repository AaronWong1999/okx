#!/usr/bin/env python3
"""Summarise OCR results: per video, list each frame with its detected
header title (the nav-bar text) and the first N body labels."""
import json
import re
import sys
from pathlib import Path

OCR = Path("/tmp/okxbots/ocr_all.jsonl")
OUT = Path("/tmp/okxbots/ocr_index.json")

BOT_KEYS = [
    "网格", "Grid", "马丁格尔", "DCA", "Martingale", "定投", "Recurring",
    "智能组合", "Smart Portfolio", "冰山", "Iceberg", "时间加权", "TWAP",
    "信号", "Signal", "套利", "Arb", "AI", "反转", "Reverse", "策略",
]


def norm(rec):
    return rec["text"]


def main():
    recs = []
    for line in OCR.read_text(errors="replace").splitlines():
        line = line.strip()
        if not line.startswith("{"):
            continue
        try:
            d = json.loads(line)
        except Exception:
            continue
        if "file" not in d:
            continue
        recs.append(d)

    # group by video dir
    by_video = {}
    for d in recs:
        p = d["file"]
        # form A: ".../full/<video>/<file>"   form B: "<video>/<file>"
        if "/full/" in p:
            vid, fname = p.split("/full/", 1)[1].rsplit("/", 1)
        else:
            vid, fname = p.rsplit("/", 1)
        by_video.setdefault(vid, []).append((fname, d))

    index = {}
    for vid, items in by_video.items():
        items.sort()
        rows = []
        for fname, d in items:
            lines = d.get("lines", [])
            # header = text lines with y < 380 (nav bar area) sorted by x
            head = [l for l in lines if l.get("y", 9999) < 400]
            head.sort(key=lambda l: (l.get("y", 0), l.get("x", 0)))
            head_txt = " / ".join(l["text"] for l in head if l.get("conf", 0) >= 0.35)
            body = [l["text"] for l in lines
                    if l.get("y", 0) >= 400 and l.get("conf", 0) >= 0.4
                    and len(l["text"]) > 1]
            # drop keyboard noise
            body = [b for b in body if not re.fullmatch(r"[a-zA-Z0-9]{1,2}", b)]
            rows.append({"file": fname, "header": head_txt, "body": body[:28]})
        index[vid] = rows

    OUT.write_text(json.dumps(index, ensure_ascii=False, indent=1))

    for vid, rows in index.items():
        print(f"\n{'='*70}\n{vid}  ({len(rows)} frames)\n{'='*70}")
        for r in rows:
            print(f"\n[{r['file']}]  HEAD: {r['header']}")
            print("   ", " | ".join(r["body"][:18]))


if __name__ == "__main__":
    main()
