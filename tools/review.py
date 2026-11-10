#!/usr/bin/env python3
"""Build a review page for generated art or audio: one HTML file with everything inside.

Each candidate shows its image (pixel art at a whole-number scale) or a player, an
Approve / Re-roll verdict and a note. The page keeps the verdicts in the browser; "Copy
verdicts" gives them as JSON, a list of {file, verdict, note}, for the owner to send back.
Each approved file that ships then gets its row: `tools/ledger.py add FILE --approved ..`.

SPEC.json:
  {"title": "...", "intro": "...", "scale": 4,
   "groups": [{"heading": "...",
               "items": [{"file": "art-src/raw/x.png", "label": "...", "scale": 8}]}]}

`scale` is for pixel art; an image with no scale fits the column. A WAV or OGG plays as
MP3 (needs ffmpeg). Needs Pillow.

Usage: tools/review.py SPEC.json OUT.html
"""

import base64
import html
import json
import subprocess
import sys
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parent.parent
AUDIO = {".wav", ".ogg", ".mp3"}

PAGE = """<!doctype html>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>__TITLE__</title>
<style>
  :root { color-scheme: dark; --ground: #14171f; --panel: #1e2330; --line: #353d52;
          --text: #eef1f6; --muted: #9aa5bd; --approve: #7ee08a; --reroll: #f07a7a; }
  body { margin: 0; background: var(--ground); color: var(--text);
         font: 16px/1.5 system-ui, -apple-system, 'Segoe UI', sans-serif; }
  main { max-width: 760px; margin: 0 auto; padding: 32px 16px 96px; display: grid; gap: 28px; }
  h1 { margin: 0; font-size: 28px; line-height: 1.15; }
  h2 { margin: 0; font-size: 13px; letter-spacing: 0.1em; text-transform: uppercase; color: var(--muted); }
  p { margin: 8px 0 0; color: var(--muted); }
  section { display: grid; gap: 12px; }
  .item { display: grid; gap: 10px; padding: 14px; border: 2px solid var(--line);
          background: var(--panel); min-width: 0; }
  .item[data-verdict="approve"] { border-color: var(--approve); }
  .item[data-verdict="reroll"] { border-color: var(--reroll); }
  .label { font-weight: 700; }
  .file { color: var(--muted); font: 13px ui-monospace, monospace; overflow-wrap: anywhere; }
  .art { overflow-x: auto; background: repeating-conic-gradient(#2a3040 0 25%, #222735 0 50%) 0 0 / 16px 16px; }
  .art img { display: block; max-width: none; }
  .art img.fit { max-width: 100%; height: auto; }
  .art img.pixel { image-rendering: pixelated; }
  audio { width: 100%; }
  .row { display: flex; flex-wrap: wrap; gap: 8px; }
  button { font: inherit; padding: 8px 14px; border: 2px solid var(--line); background: none;
           color: var(--text); cursor: pointer; }
  button[aria-pressed="true"][data-v="approve"] { border-color: var(--approve); color: var(--approve); }
  button[aria-pressed="true"][data-v="reroll"] { border-color: var(--reroll); color: var(--reroll); }
  input, textarea { font: inherit; width: 100%; box-sizing: border-box; padding: 8px;
                    border: 2px solid var(--line); background: var(--ground); color: var(--text); }
  footer { position: fixed; inset: auto 0 0 0; padding: 10px 16px; background: var(--panel);
           border-top: 2px solid var(--line); display: flex; gap: 12px; align-items: center;
           justify-content: center; }
  #out { display: none; height: 160px; font: 12px ui-monospace, monospace; }
</style>
<main>
  <header><h1>__TITLE__</h1><p>__INTRO__</p></header>
  __GROUPS__
  <textarea id="out" readonly></textarea>
</main>
<footer><span id="count"></span><button id="copy">Copy verdicts</button></footer>
<script>
  const KEY = 'review:' + document.title;
  let state = {};
  try { state = JSON.parse(localStorage.getItem(KEY)) || {}; } catch (e) {}
  const items = [...document.querySelectorAll('.item')];
  function show() {
    let done = 0;
    for (const item of items) {
      const s = state[item.dataset.file] || {};
      item.dataset.verdict = s.verdict || '';
      for (const b of item.querySelectorAll('button')) b.setAttribute('aria-pressed', b.dataset.v === s.verdict);
      if (s.verdict) done++;
    }
    document.getElementById('count').textContent = done + ' of ' + items.length + ' judged';
    try { localStorage.setItem(KEY, JSON.stringify(state)); } catch (e) {}
  }
  for (const item of items) {
    const file = item.dataset.file;
    item.querySelector('input').value = (state[file] || {}).note || '';
    item.addEventListener('click', e => {
      if (!e.target.dataset.v) return;
      const s = state[file] = state[file] || {};
      s.verdict = s.verdict === e.target.dataset.v ? '' : e.target.dataset.v;
      show();
    });
    item.querySelector('input').addEventListener('input', e => {
      (state[file] = state[file] || {}).note = e.target.value;
      show();
    });
  }
  document.getElementById('copy').addEventListener('click', async () => {
    const text = JSON.stringify(items.map(i => ({ file: i.dataset.file, ...(state[i.dataset.file] || {}) })), null, 2);
    const out = document.getElementById('out');
    out.value = text;
    out.style.display = 'block';
    try { await navigator.clipboard.writeText(text); document.getElementById('copy').textContent = 'Copied'; }
    catch (e) { out.select(); }
  });
  show();
</script>
"""


def media(path: Path, scale) -> str:
    if path.suffix in AUDIO:
        raw = path.read_bytes()
        if path.suffix != ".mp3":
            raw = subprocess.run(["ffmpeg", "-v", "error", "-i", str(path), "-f", "mp3", "-b:a", "128k", "-"],
                                 check=True, capture_output=True).stdout
        return f'<audio controls preload="none" src="data:audio/mpeg;base64,{base64.b64encode(raw).decode()}"></audio>'
    data = base64.b64encode(path.read_bytes()).decode()
    mime = "image/png" if path.suffix == ".png" else "image/jpeg"
    if scale:
        width = Image.open(path).width * scale
        return f'<div class="art"><img class="pixel" width="{width}" src="data:{mime};base64,{data}" alt=""></div>'
    return f'<div class="art"><img class="fit" src="data:{mime};base64,{data}" alt=""></div>'


def main() -> int:
    if len(sys.argv) != 3:
        print(__doc__)
        return 2
    spec = json.loads(Path(sys.argv[1]).read_text())
    groups = []
    for group in spec["groups"]:
        items = []
        for item in group["items"]:
            file = html.escape(item["file"])
            items.append(
                f'<div class="item" data-file="{file}"><div><div class="label">{html.escape(item.get("label", ""))}</div>'
                f'<div class="file">{file}</div></div>{media(ROOT / item["file"], item.get("scale", spec.get("scale")))}'
                '<div class="row"><button data-v="approve">Approve</button><button data-v="reroll">Re-roll</button></div>'
                '<input placeholder="Note" aria-label="Note"></div>'
            )
        groups.append(f'<section><h2>{html.escape(group["heading"])}</h2>{"".join(items)}</section>')
    page = (PAGE.replace("__TITLE__", html.escape(spec["title"]))
            .replace("__INTRO__", html.escape(spec.get("intro", "")))
            .replace("__GROUPS__", "\n  ".join(groups)))
    Path(sys.argv[2]).write_text(page)
    print(f"{sys.argv[2]}: {sum(len(g['items']) for g in spec['groups'])} candidates, {len(page) // 1024} KB")
    return 0


if __name__ == "__main__":
    sys.exit(main())
