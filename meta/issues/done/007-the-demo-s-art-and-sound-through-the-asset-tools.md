# [007] The demo's art and sound through the asset tools

**Status:** done
**Milestone:** M0
**Depends on:** [006]
**Created:** 2026-11-10
**Closed:** 2026-11-10

## Problem
The demo ran on squares and sine tones. The asset tools were not proven, and a new game had no example of a file with its sidecar and its ledger row.

## Approach
Make each of the demo's assets through the tools, by both routes:
- PixelLab directly: the player and the enemy (`image_fast`, 32 px), the player's 8 frames (`animation`, PixMiniMax), packed by `tools/sheet.py`.
- OpenRouter, then PixelLab: the app icon's subject (`image.py --preset transparent`), to pixel art (`pixelize`), its ground removed (`remove-background`), into both phone projects by `tools/icon.py --pixel`.
- ElevenLabs: three sound effects, two music loops, a designed voice and its line (the game over sound), encoded by `tools/encode.py`.
- A ledger row for each file under `assets/`, and `tools/check-repo.sh` runs the ledger check.

## Verification
`tools/ledger.py check` passes; a gameplay shot shows the new player; `just replay` gives the same hashes.

## Notes
- 2026-11-10: spend: OpenRouter $0.12 (two images), PixelLab 8 generations, ElevenLabs 1,238 credits.
- 2026-11-10: every row's verdict is `placeholder`: the owner has not judged these files, and nobody has listened to the sounds. The picks came from measurements (`encode.py measure`) and, for the art, from a look at the candidates.
- 2026-11-10: the first icon concept had an icon frame around the robot (the prompt said "app icon"); the second prompt described the subject only.
- 2026-11-10: the announcer voice used one voice slot of the owner's ElevenLabs account. It was deleted after the line was made (`audio-src/voice/game-over.json`).
- 2026-11-10: the takes not picked stay in `art-src/raw/` and `audio-src/takes/`, which git ignores.
