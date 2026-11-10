# [006] Asset tools and the model investigation

**Status:** done
**Milestone:** M0
**Depends on:** [003]
**Created:** 2026-11-10
**Closed:** 2026-11-10

## Problem
A game needs art, sound effects, music and voices. Each generated file must be possible to make again, must have known licence terms, and must come from a model that is the best one now, not the one from an old habit.

## Approach
- Keys: the environment first, then `.env` (git ignores it; `.env.example` lists the names). `tools/common.py` reads them. No tool prints a key or writes one to a file.
- `tools/models.toml`: the model or endpoint for each kind of work. `docs/research/models.md`: the dated investigation behind each pick, and how to do the next one.
- `tools/image.py` (OpenRouter images), `tools/pixellab.py` (any PixelLab REST endpoint, with job polling), `tools/sheet.py` (frames to a sheet), `tools/audio.py` (ElevenLabs: `sfx`, `music`, `design`, `voice`, `say`, `credits`), `tools/encode.py` (measure takes; trim, level and encode to Ogg Vorbis).
- Each generation writes a sidecar JSON beside its file: the request, the model, the cost, the date.
- `tools/ledger.py`: a row for each file under `assets/` with its hash, source, licence terms and verdict (`placeholder` or `approved`); `check` fails on a file with no row or a changed hash.
- `tools/review.py`: one HTML page of candidates with Approve / Re-roll and a note for each; the owner copies the verdicts back as JSON.
- `tools/icon.py --pixel` scales a pixel art icon with hard pixels.

## Verification
Each tool answers a real call and writes its file and its sidecar ([007] uses them all). `tools/ledger.py check` fails on a file with no row and passes when each has one.

## Notes
- 2026-11-10: the investigation's picks: images `openai/gpt-image-2.6-sunburst`; PixelLab animation `animate-pixminimax` (the owner's pick; PixelLab's docs name `animate-with-text-v3` the default, kept as `animation_v3`); ElevenLabs `eleven_text_to_sound_v3`, `music_v2_5`, `eleven_v4`, `eleven_ttv_v3`.
- 2026-11-10: real calls: `image.py` (2), `pixellab.py` (pixen stills, pixminimax, image-to-pixelart, remove-background, balance), `audio.py` (sfx, music, design, voice, say, credits), `encode.py` (measure, sfx, loop), `sheet.py`, `icon.py`, `ledger.py` (add, check: 9 problems before the rows, 0 after).
- 2026-11-10: found by the calls: a silent sound take (`encode.py` now reports it), no WAV on the music endpoint (the call moves down to MP3), an opaque answer from `image-to-pixelart`, a non-square answer from the `transparent` preset. All are in `docs/research/models.md`.
- 2026-11-10: `review.py`: the page builds (6 candidates, images and players inside) and its script parses. Not checked: the page in a browser.
- 2026-11-10: REST only. The PixelLab MCP is fine to explore with, but a file that ships comes from `tools/pixellab.py`, so it has a sidecar.
- 2026-11-10: `encode.py` needs ffmpeg and oggenc (Homebrew's ffmpeg has no libvorbis); the image tools need Pillow.
