# Model investigation

Which model does each kind of asset work, and why. `tools/models.toml` holds the picks
that the tools use. A game's style-lock milestone starts with a new investigation: the
models change every few weeks, and a pick that is three months old is probably not the
best one. Each investigation adds a dated section at the top and updates `tools/models.toml`
in the same commit.

How to investigate:

1. List what each provider has now, from its API, not from memory:
   `GET https://openrouter.ai/api/v1/images/models`, `https://api.pixellab.ai/v2/llms.txt`,
   `GET https://api.elevenlabs.io/v1/models` and the ElevenLabs docs.
2. Read two independent boards for the image models (Artificial Analysis and Arena, text
   to image and image edit). Record the date of each board.
3. Run a small pilot for the game's real subjects with the two or three best candidates.
   The owner judges it on a review page (`tools/review.py`). A board does not know the
   game's style.
4. Record the pick, its cost from the sidecars, and what was not verified.

## 2026-11-10

Sources: the three providers' live APIs and docs, the Artificial Analysis image boards
(read 2026-11-10) and the Arena boards (page date 2026-11-08). The demo's assets were then
made with these picks, so each tool and each model id below answered a real call.

### Images: OpenRouter

`POST https://openrouter.ai/api/v1/images`; the answer has the image as base64 and its
cost in `usage.cost`.

| Use | Preset | Model | Settings | Cost |
|---|---|---|---|---|
| Key art, store art, concept art | `key_art` | `openai/gpt-image-2.6-sunburst` | `quality: max` | about $0.21 at 1024 x 1024 (third-party figure) |
| Style references | `style_ref` | the same | `quality: high`, 1:1 | about $0.05 |
| One subject on a clear ground | `transparent` | the same | `quality: high`, `background: transparent` | $0.057 and $0.062 (measured, two calls) |
| Cheap drafts | `draft` | `black-forest-labs/flux-3-image-klein` | 1K | $0.009 (listed) |

- GPT Image 2.6 Sunburst (on OpenRouter since 2026-11-04) is first on both Artificial
  Analysis boards at `max`. Arena still has 2.5 Sunburst first; it tested 2.6 only at
  medium quality, with few votes. `openai/gpt-image-2.5-sunburst` is the fallback.
- Only the OpenAI image models from 1 on (not `gpt-image-2`) and Sourceful Riverflow list
  a transparent background.
- The `transparent` preset has no aspect ratio: one answer was 1312 x 1199. Pass
  `--aspect 1:1` when the next step needs a square.
- "App icon" in a prompt drew an icon frame around the subject. Describe the subject only.

### Pixel art: PixelLab

REST v2, `https://api.pixellab.ai/v2`. Costs are in generations (about $0.008 each).

| Work | Role | Endpoint | Notes |
|---|---|---|---|
| A still, best quality | `image` | `generate-image-v2` | several candidates a call, 10 to 25 generations |
| A still, fast | `image_fast` | `create-image-pixen` | 1 generation; the demo's sprites |
| A character with 8 directions | `character` | `create-character-v3` | from text it draws a humanoid only |
| An animation | `animation` | `animate-pixminimax` | the owner's pick; up to 40 frames; 1 generation for 8 frames at 32 px (measured) |
| An animation, PixelLab's default | `animation_v3` | `animate-with-text-v3` | up to 16 frames |
| Rotations | `rotation` | `generate-8-rotations-v3` | |
| A tileset | `tileset` | `create-tileset` | Wang tiles; `mode: pro` takes a palette |
| An image to pixel art | `pixelize` | `image-to-pixelart` | 1 generation; `pixelize_pro` redraws more, 20 generations |
| A palette lock | `palette` | `reduce-colors` | several frames onto one palette |

- PixelLab's docs name `animate-with-text-v3` "the recommended default" and label
  PixMiniMax "Beta". The owner's experience is that PixMiniMax gives the better
  animations, so it is the default here. Compare both in a game's pilot.
- An animation's answer starts with the input frame: `tools/pixellab.py --drop-first`.
- `image-to-pixelart` wants an output a quarter of its input and gives an opaque image
  (measured: a clear 1024 px input came back on a grey ground). `remove-background`
  (1 generation, at most 400 x 400) makes it clear again.

### Sound: ElevenLabs

| Work | Model id | Notes |
|---|---|---|
| Sound effects | `eleven_text_to_sound_v3` | `duration_seconds` 0.5 to 30; a 0.5 s take cost 6 credits; WAV at 48 kHz worked on the Creator plan |
| Music | `music_v2_5` | `generation_mode: loop`; no WAV on this endpoint, MP3 at 192 kbps worked |
| Speech | `eleven_v4` | audio tags such as `[excited]`; 9 credits for a two-word line |
| Voice design | `eleven_ttv_v3` | three previews a call; keeping one uses a voice slot |

- The API's own defaults are older models (`eleven_text_to_sound_v2`, `music_v1`,
  `eleven_multilingual_v2`). The tools always send the model id.
- One of two takes of a 0.5 s click was silent. Make two takes or more of a short sound,
  and run `tools/encode.py measure` before a pick.
- The music loops had 0.3 s and 0.65 s of silence at the end. `tools/encode.py loop` trims
  the ends.

### Licence terms read

- PixelLab FAQ: commercial use is allowed; do not train models with the images.
- ElevenLabs terms (2026-10-09): a paid plan allows commercial use. Eleven Music terms
  (2026-10-09): every self-serve plan excludes a "Studio Game"; an indie game (revenue
  under $500,000, publisher under $1,000,000 a year) is allowed. Streaming distribution
  needs the Creator plan or higher.
- OpenRouter and OpenAI terms for image output: not read in this investigation. The owner
  confirms them before a generated image ships.

### Not verified

- Whether 2.6 Sunburst is better than 2.5 Sunburst at equal quality (the boards disagree).
- The cost of `quality: max`: no call at `max` was made.
- PixelLab's concurrency limit for each tier.
- How the two music loops sound at the join: nobody has listened to them yet.
