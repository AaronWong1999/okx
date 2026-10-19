#!/usr/bin/env python3
"""Pass 1: extract low-res frames at fixed fps, then perceptually dedup.

Outputs a JSON list of unique frame timestamps per video plus the kept
low-res images, so pass 2 can re-extract only the winners at full res.
"""
import json
import os
import subprocess
import sys
from pathlib import Path

import numpy as np
from PIL import Image

VID_DIR = Path("/Users/aaron/Desktop/aaron/okx")
WORK = Path("/tmp/okxbots")
RAW = WORK / "raw"
FPS = 2
SCALE_W = 295  # 1/4 of 1180


def video_files():
    return sorted(VID_DIR.glob("*.MP4"))


def slug(name):
    # ScreenRecording_10-19-2026 22-53-19_1.MP4 -> 22-53-19
    stem = name.replace("ScreenRecording_", "").replace(".MP4", "")
    parts = stem.split("_")
    return parts[0] if parts else stem


def phash(img, hash_size=8):
    """DCT perceptual hash -> 64-bit int."""
    g = np.asarray(img.convert("L").resize((hash_size * 4, hash_size * 4), Image.LANCZOS), dtype=np.float64)
    # 2D DCT-II
    n = g.shape[0]
    k = np.arange(n)
    basis = np.cos(np.pi * (2 * k + 1) * np.arange(n)[:, None] / (2 * n))
    d = basis.T @ g @ basis
    low = d[:hash_size, :hash_size]
    flat = low.flatten()
    med = np.median(flat[1:])  # skip DC
    bits = (flat > med).astype(np.uint8)
    out = 0
    for b in bits:
        out = (out << 1) | int(b)
    return out, low


def hamming(a, b):
    return bin(a ^ b).count("1")


def main():
    RAW.mkdir(parents=True, exist_ok=True)
    all_results = []
    for vf in video_files():
        s = slug(vf.name)
        outdir = RAW / s
        if outdir.exists():
            subprocess.run(["rm", "-rf", str(outdir)])
        outdir.mkdir(parents=True)
        print(f"== extracting {vf.name} -> {outdir}", flush=True)
        subprocess.run(
            ["ffmpeg", "-v", "error", "-i", str(vf), "-vf", f"fps={FPS},scale={SCALE_W}:-1",
             "-q:v", "4", str(outdir / "f_%05d.jpg"), "-y"],
            check=True,
        )
        frames = sorted(outdir.glob("f_*.jpg"))
        print(f"   {len(frames)} raw frames", flush=True)

        kept = []          # list of dicts: idx, phash, low(8x8 array)
        kept_paths = []
        for i, fp in enumerate(frames):
            try:
                im = Image.open(fp)
                im.load()
            except Exception:
                continue
            h, low = phash(im)
            # compare against last few kept (screen scroll changes progressively)
            dup = False
            for k in reversed(kept[-6:]):
                hd = hamming(h, k["phash"])
                if hd < 6:
                    dup = True
                    break
                # low-frequency 8x8 DCT energy diff
                d = np.abs(low - k["low"]).mean()
                if hd < 14 and d < 2.2:
                    dup = True
                    break
            if dup:
                continue
            kept.append({"phash": h, "low": low, "index": i})
            kept_paths.append(fp)

        # remove rejected raw frames to save space
        keep_names = {p.name for p in kept_paths}
        for fp in frames:
            if fp.name not in keep_names:
                fp.unlink()

        meta = {
            "video": vf.name,
            "slug": s,
            "fps": FPS,
            "raw_count": len(frames),
            "unique_count": len(kept_paths),
            "kept": [p.name for p in kept_paths],
        }
        all_results.append(meta)
        print(f"   unique: {len(kept_paths)}", flush=True)

    (WORK / "candidates.json").write_text(json.dumps(all_results, indent=2))
    print("TOTAL unique:", sum(m["unique_count"] for m in all_results))


if __name__ == "__main__":
    main()
