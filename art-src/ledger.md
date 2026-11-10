# Art ledger

One row for each image and font under `assets/`: its hash, its verdict, how it was made, its
licence terms and where the owner approved it. `tools/ledger.py add` writes a row and
`tools/ledger.py check` checks them all (`docs/assets.md`).

| File | Hash | Verdict | Source | Licence | Review | Date |
|---|---|---|---|---|---|---|
| `assets/fonts/DepartureMono-Regular.otf` | `4d53f663155cf8bf` | placeholder | Departure Mono 1.500 by Helena Zhang (departuremono.com), unchanged | SIL Open Font License 1.1 (art-src/fonts/DepartureMono-OFL.txt) | - | 2026-11-10 |
| `assets/images/player-sheet.png` | `50ee61b662858314` | placeholder | tools/sheet.py from art-src/sprites/player-idle-*.png: PixelLab animate-pixminimax (art-src/sprites/player-idle.json) from the still art-src/sprites/player.png, PixelLab create-image-pixen (art-src/sprites/player.json) | PixelLab FAQ (read 2026-11-10): commercial use allowed; do not train models with the images | - | 2026-11-10 |
| `assets/images/enemy.png` | `d057041f7ca494bc` | placeholder | PixelLab create-image-pixen, art-src/sprites/enemy.json | PixelLab FAQ (read 2026-11-10): commercial use allowed; do not train models with the images | - | 2026-11-10 |
