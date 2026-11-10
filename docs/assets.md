# Assets

How art and sound get into the game. Generation happens at content time only: the game
never calls a provider.

## The path of a file

```
generate -> art-src/raw/ or audio-src/takes/   (git ignores both), with a sidecar JSON
review   -> the owner approves or re-rolls on a review page
pick     -> move the pick and its sidecar to art-src/<kind>/ or audio-src/<kind>/
build    -> the game's file under assets/ (a sheet, an .ogg)
ledger   -> tools/ledger.py add assets/.. --source .. --licence .. [--approved ..]
```

`tools/ledger.py check` runs in `just check`: a file under `assets/` with no row, or one
that changed after its row, fails. `--final` (the release) also refuses `placeholder`
rows. A file with unknown licence terms does not ship.

## Keys

`OPENROUTER_API_KEY`, `PIXELLAB_TOKEN`, `ELEVENLABS_TOKEN`: from the environment, then
from `.env`. Never print one. What remains: `tools/pixellab.py balance`,
`tools/audio.py credits`.

## Models

`tools/models.toml` has the model for each kind of work; `docs/research/models.md` has
the reasons, the costs, the traps and how to investigate again. M2 starts with a new
investigation.

## Art

Two routes. Pick for each kind of asset in the style pilot, and write the result in
`docs/art-style.md`.

**PixelLab directly**: what must sit on the pixel grid at game size: sprites, animations,
tiles, small icons.

```sh
tools/pixellab.py image_fast --out art-src/raw/hero \
  --args '{"description": "..", "image_size": {"width": 32, "height": 32}, "no_background": true}'
tools/pixellab.py animation --out art-src/raw/hero-walk --drop-first \
  --image first_frame=art-src/sprites/hero.png \
  --args '{"description": "walks to the right", "frame_count": 8, "no_background": true}'
tools/sheet.py assets/images/hero-sheet.png 'art-src/sprites/hero-idle-*.png' 'art-src/sprites/hero-walk-*.png'
```

The first argument is a role of `tools/models.toml` or any endpoint of the REST API
(`https://api.pixellab.ai/v2/llms.txt`). `--image KEY=FILE` puts an image into the request.

**OpenRouter, then PixelLab**: what sets the look or is bigger than a sprite: concepts,
style references, key art; and a design that must carry over from an illustration into
pixel art.

```sh
tools/image.py "the subject, isolated, no frame" --preset transparent --aspect 1:1 --out art-src/raw/concept
tools/pixellab.py pixelize --out art-src/raw/pixel --image image=concept-1024.png \
  --args '{"image_size": {"width": 1024, "height": 1024}, "output_size": {"width": 256, "height": 256}}'
tools/pixellab.py remove-background --out art-src/raw/clear --image image=art-src/raw/pixel.png \
  --args '{"image_size": {"width": 256, "height": 256}}'
```

Presets: `key_art`, `style_ref`, `transparent`, `draft`. `--ref FILE` gives a reference
image (a style reference, an earlier pick).

For a game with `Style::Smooth`, OpenRouter makes the art itself and PixelLab is not in
the path.

## Sound

```sh
tools/audio.py sfx "short soft UI click, dry" --seconds 0.5 --takes 3 --out audio-src/takes/press
tools/audio.py music "calm chiptune, 100 BPM, seamless loop" --seconds 30 --out audio-src/takes/title
tools/audio.py design "gruff old blacksmith, deep, warm" --out audio-src/takes/smith   # 3 previews
tools/audio.py voice <generated_voice_id> --name Smith --description ".." --out audio-src/voice/smith
tools/audio.py say "[whispers] They are close." --voice <voice_id> --out audio-src/takes/line

tools/encode.py measure audio-src/takes/press-*.wav
tools/encode.py sfx audio-src/sfx/press.wav assets/audio/sfx/press.ogg --peak -6
tools/encode.py loop audio-src/music/title.mp3 assets/audio/music/title.ogg --peak -6
```

- Make several takes. Measure them before the owner hears them: a take can be silent.
- `encode.py sfx` trims the silence and fades the end; `loop` trims with no fade.
- A kept voice uses a slot of the owner's account. Record its id in `docs/audio-style.md`.
- The game plays a sound through a `Sfx` message and music through `Track`
  (`src/audio.rs`).

## Review pages

```sh
tools/review.py spec.json target/review/sprites.html
```

The spec lists candidates in groups (`tools/review.py` has the format). The page has an
Approve / Re-roll verdict and a note for each, and "Copy verdicts" gives them back as
JSON. Give the owner the file's path, or publish the page if your harness can; do not
open a window. After the verdicts: move the picks, add their ledger rows with
`--approved "<where the owner approved it>"`, and record the spend in the issue.

Show pixel art at a whole-number scale (`"scale": 8`) and an animation as its sheet.
