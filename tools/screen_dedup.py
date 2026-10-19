#!/usr/bin/env python3
"""Dedup frames by OCR screen signature.

Price tickers change every frame; the *structure* of a screen (which labels
are present) stays stable. So two frames with the same set of non-numeric
text lines are the same screen -> keep one representative.
"""
import json
import re
from collections import defaultdict
from pathlib import Path

INDEX = Path("/tmp/okxbots/ocr_index.json")
OUT = Path("/tmp/okxbots/screens.json")

# UI chrome / system noise that never identifies a screen
CHROME = re.compile(
    r"^(22:5\d|23:0\d|23:1\d|23:2\d|23:3\d|23:4\d|2[0-9]:[0-5]\d|1[0-9]:[0-5]\d|"
    r"100|95|9[0-9]|8[0-9]|7[0-9]|[Qq]|回|100%|Q)$"
)


def signature(lines):
    keep = []
    for raw in lines:
        t = (raw["text"] if isinstance(raw, dict) else raw).strip()
        if not t or CHROME.match(t):
            continue
        # drop pure numbers / prices / percentages
        if re.fullmatch(r"[\d.,%+\-/ ]+", t):
            continue
        if len(t) < 2:
            continue
        keep.append(t)
    # de-duplicate, order-insensitive-ish but stable
    return tuple(sorted(set(keep)))


def main():
    index = json.loads(INDEX.read_text())
    screens = {}
    for vid, rows in index.items():
        groups = defaultdict(list)
        for r in rows:
            sig = signature(r["body"])
            groups[sig].append(r)
        picked = []
        for sig, members in groups.items():
            # representative = the member with the most recognised text
            members.sort(key=lambda r: (-len(r["body"]), r["file"]))
            rep = members[0]
            picked.append({
                "file": rep["file"],
                "header": rep["header"],
                "body": rep["body"],
                "n_dupes": len(members),
                "also": [m["file"] for m in members[1:]],
                "sig_size": len(sig),
            })
        picked.sort(key=lambda x: x["file"])
        screens[vid] = picked
        print(f"{vid}: {len(rows)} frames -> {len(picked)} distinct screens")
    OUT.write_text(json.dumps(screens, ensure_ascii=False, indent=1))
    print("TOTAL", sum(len(v) for v in screens.values()))


if __name__ == "__main__":
    main()
