#!/usr/bin/env bash
# Turn one recorded act into a GIF for the README.
#
#   ./demo/gif.sh demo/out/01-basics.mp4          -> demo/demo.gif
#   ./demo/gif.sh demo/out/03-pubsub.mp4 pubsub   -> demo/pubsub.gif
#
# GitHub strips <video> from READMEs, so an inline demo has to be a GIF. Two
# passes: generate a palette from the clip, then map to it. A single global
# palette banks on terminal output using few colours, which it does.

set -euo pipefail
cd "$(dirname "$0")/.."

SRC="${1:?usage: gif.sh <clip.mp4> [name]}"
OUT="demo/${2:-demo}.gif"
FPS="${FPS:-12}"        # typing reads fine at 12; 24 doubles the size for nothing
WIDTH="${WIDTH:-900}"   # GitHub renders README images up to about 890px wide
PAL=$(mktemp -t gifpal).png

ffmpeg -v error -y -i "$SRC" \
  -vf "fps=$FPS,scale=$WIDTH:-1:flags=lanczos,palettegen=stats_mode=diff" "$PAL"

# bayer dithering keeps flat terminal backgrounds from developing noise, which
# the default error-diffusion adds and which costs a lot of bytes in a GIF.
ffmpeg -v error -y -i "$SRC" -i "$PAL" \
  -lavfi "fps=$FPS,scale=$WIDTH:-1:flags=lanczos[x];[x][1:v]paletteuse=dither=bayer:bayer_scale=3" \
  "$OUT"

rm -f "$PAL"
printf '%s  %s\n' "$OUT" "$(du -h "$OUT" | cut -f1)"
