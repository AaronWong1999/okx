#!/usr/bin/env python3
"""Pass 2: re-extract the deduped candidate frames at full native resolution."""
import json
import subprocess
from pathlib import Path

VID_DIR = Path("/Users/aaron/Desktop/aaron/okx")
WORK = Path("/tmp/okxbots")
FULL = WORK / "full"
FPS = 2


def slug(name):
    stem = name.replace("ScreenRecording_10-19-2026_", "").replace("ScreenRecording_", "").replace(".MP4", "")
    return stem


def main():
    cands = json.loads((WORK / "candidates.json").read_text())
    FULL.mkdir(parents=True, exist_ok=True)
    index = []
    for meta in cands:
        s = meta["slug"]
        vf = VID_DIR / meta["video"]
        outdir = FULL / s
        outdir.mkdir(parents=True, exist_ok=True)
        kept = meta["kept"]  # e.g. f_00012.jpg  -> 1-based frame index at 2fps
        idxs = sorted(int(k.split("_")[1].split(".")[0]) for k in kept)

        # Extract ALL 2fps frames as PNG (same fps filter as pass 1 => exact
        # frame correspondence), then keep only the deduped indices.
        tmp = outdir / "_tmp"
        tmp.mkdir(exist_ok=True)
        for old in tmp.glob("*.png"):
            old.unlink()
        cmd = ["ffmpeg", "-v", "error", "-i", str(vf), "-vf", f"fps={FPS}",
               str(tmp / "s_%05d.png"), "-y"]
        subprocess.run(cmd, check=True)
        made = sorted(tmp.glob("s_*.png"))
        assert len(made) >= max(idxs), f"{s}: expected >= {max(idxs)} got {len(made)}"
        recs = []
        for i in idxs:
            p = tmp / f"s_{i:05d}.png"
            assert p.exists(), f"{s}: missing {p.name}"
            dest = outdir / f"t{i:05d}.png"
            p.rename(dest)
            recs.append({"src_index": i, "timestamp": round((i - 1) / FPS, 3), "file": dest.name})
        for p in tmp.glob("*.png"):
            p.unlink()
        tmp.rmdir()
        index.append({"video": meta["video"], "slug": s, "frames": recs})
        print(f"{s}: {len(recs)} full-res frames", flush=True)

    (WORK / "full_index.json").write_text(json.dumps(index, indent=2))
    print("TOTAL", sum(len(i["frames"]) for i in index))


if __name__ == "__main__":
    main()
