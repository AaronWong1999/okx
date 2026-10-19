#!/usr/bin/env python3
"""Extract the nav-bar page title from OCR geometry.

The page title is the largest-font text in the nav strip, horizontally
centred.  Fall back to the widest centred string when the app renders
several same-size labels.
"""
import json
from pathlib import Path

OCR = Path("/tmp/okxbots/ocr_all.jsonl")
OUT = Path("/tmp/okxbots/titles.json")

# native px; nav strip sits below the status bar
NAV_Y = (120, 300)
CENTER_LO, CENTER_HI = 250, 930


def nav_title(lines, w=1180):
    cands = []
    for l in lines:
        x, y, ww, hh = l.get("x", 0), l.get("y", 0), l.get("w", 0), l.get("h", 0)
        if not (NAV_Y[0] <= y <= NAV_Y[1]):
            continue
        if l.get("conf", 0) < 0.4:
            continue
        cx = x + ww / 2
        if not (CENTER_LO - 120 <= cx <= CENTER_HI + 120):
            continue
        cands.append((hh, ww, cx, l["text"]))
    if not cands:
        # fall back: any sizeable text in the nav strip
        for l in lines:
            if NAV_Y[0] <= l.get("y", 0) <= NAV_Y[1] and l.get("h", 0) >= 22:
                cands.append((l["h"], l["w"], l.get("x", 0), l["text"]))
    if not cands:
        return None
    maxh = max(c[0] for c in cands)
    top = [c for c in cands if c[0] >= maxh * 0.85]
    # among the largest, take the one closest to horizontal centre
    best = min(top, key=lambda c: abs(c[2] - w / 2))
    return best[3]


def main():
    res = {}
    for line in OCR.read_text(errors="replace").splitlines():
        line = line.strip()
        if not line.startswith("{"):
            continue
        try:
            d = json.loads(line)
        except Exception:
            continue
        if "file" not in d or "lines" not in d:
            continue
        p = d["file"]
        vid, fname = (p.split("/full/", 1)[1].rsplit("/", 1) if "/full/" in p
                      else p.rsplit("/", 1))
        res.setdefault(vid, {})[fname] = nav_title(d["lines"])
    OUT.write_text(json.dumps(res, ensure_ascii=False, indent=1))
    for vid in res:
        vals = [v for v in res[vid].values() if v]
        print(f"{vid}: {len(res[vid])} frames, {len(set(vals))} distinct titles")
        from collections import Counter
        for t, c in Counter(vals).most_common(20):
            print(f"    {c:3}  {t}")


if __name__ == "__main__":
    main()
