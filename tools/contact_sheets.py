#!/usr/bin/env python3
"""Build nav-bar contact sheets restricted to the BOT-relevant segment of
each recording, and emit cell -> frame mapping."""
import json
from pathlib import Path

from PIL import Image

FULL = Path("/tmp/okxbots/full")
OUT = Path("/tmp/okxbots/sheets")
CROPS = {
    "nav": (0, 120, 1180, 620),
    "body": (0, 560, 1180, 1720),
}
COLS, ROWS = 2, 3
CELL_W = 820

# first BOT-relevant frame index per recording (everything before is app
# launch / token market pages / news feed)
BOT_START = {
    "10-19-2026 22-53-19": 157,
    "10-19-2026 22-58-20": 1,
    "10-19-2026 23-24-54": 1,
    "10-19-2026 23-27-54": 1,
    "10-19-2026 23-30-45": 1,
    "10-19-2026 23-34-06": 1,
    "10-19-2026 23-36-18": 1,
}


def build(name, crop_key, frames, idx):
    x0, y0, x1, y1 = CROPS[crop_key]
    cells = []
    for f in frames:
        im = Image.open(FULL / f).convert("RGB").crop((x0, y0, x1, y1))
        h = int(im.height * CELL_W / im.width)
        cells.append(im.resize((CELL_W, h), Image.LANCZOS))
    ch = cells[0].height
    rows = (len(cells) + COLS - 1) // COLS
    sheet = Image.new("RGB", (COLS * CELL_W + (COLS - 1) * 8, rows * ch + (rows - 1) * 8),
                       (255, 0, 255))
    for i, c in enumerate(cells):
        r, cc = divmod(i, COLS)
        sheet.paste(c, (cc * (CELL_W + 8), r * (ch + 8)))
    p = OUT / f"{name}.png"
    sheet.save(p, "PNG")
    idx[name] = [{"cell": i, "frame": f} for i, f in enumerate(frames)]
    return p


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    for old in OUT.glob("*.png"):
        old.unlink()
    screens = json.loads(Path("/tmp/okxbots/screens.json").read_text())
    idx = {}
    per = COLS * ROWS
    for vid, rows in screens.items():
        start = BOT_START.get(vid, 1)
        frames = [f"{vid}/{r['file']}" for r in rows
                  if int(r["file"][1:6]) >= start]
        for i in range(0, len(frames), per):
            chunk = frames[i:i + per]
            build(f"{vid}__{i//per:02d}", "nav", chunk, idx)
    Path("/tmp/okxbots/sheet_index.json").write_text(json.dumps(idx, ensure_ascii=False, indent=1))
    total = sum(len(v) for v in idx.values())
    print(f"{len(idx)} sheets, {total} frames")


if __name__ == "__main__":
    main()
