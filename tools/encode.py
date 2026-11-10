#!/usr/bin/env python3
"""Measure sound takes and encode the picks for the game.

  encode.py measure FILE..              onset, dead tail, peak and length of each take
  encode.py sfx IN OUT.ogg --peak DB    trims the silence at both ends, fades the end,
                                        sets the peak, encodes
  encode.py loop IN OUT.ogg --peak DB   trims the silence at both ends (an MP3 adds some),
                                        with no fade, so the loop joins; sets the peak,
                                        encodes

IN is a WAV or an MP3 (`tools/audio.py`). Silence is below SILENCE_DB under the take's
loudest 5 ms. Needs ffmpeg (it reads the input) and oggenc (vorbis-tools: it writes the
Ogg Vorbis that Bevy plays).
"""

import argparse
import array
import math
import os
import subprocess
import sys
import tempfile
import wave

SILENCE_DB = -40.0
FADE_SECS = 0.01


def read(path):
    """The samples as 16-bit PCM, the channels and the rate."""
    with tempfile.NamedTemporaryFile(suffix=".wav") as tmp:
        subprocess.run(["ffmpeg", "-v", "error", "-y", "-i", path, "-c:a", "pcm_s16le", tmp.name],
                       check=True)
        with wave.open(tmp.name) as w:
            samples = array.array("h", w.readframes(w.getnframes()))
            return samples, w.getnchannels(), w.getframerate()


def frame_peaks(samples, channels):
    return [max(abs(s) for s in samples[i : i + channels]) for i in range(0, len(samples), channels)]


def bounds(peaks, rate):
    """The first and the last frame of the 5 ms windows above the silence line: a lone
    click in the silence does not count."""
    size = rate // 200
    windows = range(0, len(peaks), size)
    rms = [math.sqrt(sum(p * p for p in peaks[i : i + size]) / size) for i in windows]
    if not max(rms):
        sys.exit("a silent take")
    line = max(rms) * 10 ** (SILENCE_DB / 20)
    loud = [i for i, r in enumerate(rms) if r > line]
    return loud[0] * size, min((loud[-1] + 1) * size, len(peaks)) - 1


def db(value):
    return 20 * math.log10(value / 32768) if value else -math.inf


def measure(paths):
    print(f"{'take':32} {'onset ms':>8} {'tail ms':>8} {'peak dB':>8} {'secs':>6}")
    for path in paths:
        samples, channels, rate = read(path)
        peaks = frame_peaks(samples, channels)
        if not max(peaks):
            print(f"{os.path.basename(path):32} silent")
            continue
        first, last = bounds(peaks, rate)
        ms = lambda frames: frames / rate * 1000
        print(
            f"{os.path.basename(path):32} {ms(first):8.0f} {ms(len(peaks) - 1 - last):8.0f} "
            f"{db(max(peaks)):8.1f} {len(peaks) / rate:6.2f}"
        )


def encode(path, out, peak, loop):
    samples, channels, rate = read(path)
    first, last = bounds(frame_peaks(samples, channels), rate)
    fade = 0 if loop else int(FADE_SECS * rate)
    end = min(last + fade, len(samples) // channels - 1)
    samples = samples[first * channels : (end + 1) * channels]
    gain = 32767 * 10 ** (peak / 20) / max(abs(s) for s in samples)
    frames = len(samples) // channels
    for i in range(len(samples)):
        fall = min(1.0, (frames - 1 - i // channels) / fade) if fade else 1.0
        samples[i] = max(-32768, min(32767, round(samples[i] * gain * fall)))
    with tempfile.NamedTemporaryFile(suffix=".wav") as tmp:
        with wave.open(tmp.name, "wb") as w:
            w.setnchannels(channels)
            w.setsampwidth(2)
            w.setframerate(rate)
            w.writeframes(samples.tobytes())
        subprocess.run(["oggenc", "-Q", "-q", "5", "-o", out, tmp.name], check=True)
    print(f"{out}: {frames / rate:.2f} s, peak {peak} dB, {os.path.getsize(out)} bytes")


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawTextHelpFormatter)
    sub = parser.add_subparsers(dest="command", required=True)
    m = sub.add_parser("measure")
    m.add_argument("file", nargs="+")
    for name in ("sfx", "loop"):
        e = sub.add_parser(name)
        e.add_argument("file")
        e.add_argument("out")
        e.add_argument("--peak", type=float, required=True, help="the peak in dB of full scale")
    args = parser.parse_args()
    if args.command == "measure":
        measure(args.file)
    else:
        encode(args.file, args.out, args.peak, loop=args.command == "loop")


if __name__ == "__main__":
    sys.exit(main())
