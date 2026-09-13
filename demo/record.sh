#!/usr/bin/env bash
# Records each demo act as its own short clip, then compresses it for the portfolio.
#
#   ./demo/record.sh          record all five acts
#   ./demo/record.sh 3        record just act 3
#   ./demo/record.sh 2 4      record acts 2 and 4
#
# Opens a fullscreen Ghostty window attached to the tmux session, captures the display
# while the act runs, then encodes. Needs Screen Recording permission for Ghostty.
#
# Output: demo/out/NN-name.mp4

set -euo pipefail
cd "$(dirname "$0")/.."
source demo/lib.sh
source demo/acts.sh

require tmux "brew install tmux"
require ffmpeg "brew install ffmpeg"
require cliclick "brew install cliclick"

# Clean up on any exit path, not just the happy one.
#
# The cursor-parking loop runs in the background for the length of a take. Without a
# trap, a failure mid-run leaves it orphaned and it keeps dragging the pointer to the
# corner every second until it is hunted down by hand.
CAP_PID=""; PARK_PID=""
cleanup() {
  [ -n "$PARK_PID" ] && kill "$PARK_PID" 2>/dev/null
  [ -n "$CAP_PID" ] && kill -INT "$CAP_PID" 2>/dev/null
  pkill -x screencapture 2>/dev/null
  return 0
}
trap cleanup EXIT INT TERM

OUT="${OUT:-demo/out}"
RAW="/tmp/rustmas-demo-raw.mov"
DISPLAY_ID="${DISPLAY_ID:-1}"
SETTLE="${SETTLE:-1.5}"    # seconds of quiet head and tail around each act

name_for() {
  case "$1" in
    1) echo "01-run-a-day" ;;        2) echo "02-independent-solver" ;;
    3) echo "03-every-year-at-once" ;; 4) echo "04-the-cache" ;;
  esac
}

open_window() {
  tmux list-clients -t "$SESSION" -F '#{client_tty}' 2>/dev/null | while read -r t; do
    tmux detach-client -t "$t" 2>/dev/null || true
  done
  # Bigger than the default 13, which gives 183 columns at fullscreen and glyphs
  # under 5px wide once a clip is scaled to 900 for a README GIF. 22 gives 112
  # columns, so the text survives the downscale.
  ghostty --font-size="${FONT_SIZE:-22}" -e tmux attach -t "$SESSION" >/dev/null 2>&1 &
  sleep 3.5

  local before after
  before=$(tmux list-clients -t "$SESSION" -F '#{client_width}' | head -1)
  osascript -e 'tell application "System Events" to keystroke "f" using {command down, control down}' >/dev/null 2>&1
  sleep 3
  after=$(tmux list-clients -t "$SESSION" -F '#{client_width}' | head -1)

  if [ "${after:-0}" -le "${before:-0}" ]; then
    echo "fullscreen did not take (${before} -> ${after} cols)." >&2
    echo "Recording now would capture the whole desktop. Fullscreen the demo window" >&2
    echo "by hand, then re-run." >&2
    exit 1
  fi
  echo "  window fullscreen: ${before} -> ${after} cols"
  pin_layout
}

# Detach rather than sending the fullscreen keystroke again. The keystroke lands on
# whatever is frontmost, which is a good way to fullscreen the wrong window; detaching
# ends the window's `tmux attach`, so the window closes and fullscreen goes with it.
close_window() {
  tmux list-clients -t "$SESSION" -F '#{client_tty}' 2>/dev/null | while read -r t; do
    tmux detach-client -t "$t" 2>/dev/null || true
  done
  sleep 1
}

# Park the pointer in the bottom-right corner before capturing.
#
# Two reasons. The arrow itself shows up in the recording, and more importantly macOS
# reveals the menu bar and title bar over a fullscreen window whenever the cursor is
# near the top of the screen, which puts furniture in an otherwise clean shot.
park_cursor() {
  local w h
  w=$(osascript -e 'tell application "Finder" to get bounds of window of desktop' 2>/dev/null | awk -F', ' '{print $3}')
  h=$(osascript -e 'tell application "Finder" to get bounds of window of desktop' 2>/dev/null | awk -F', ' '{print $4}')
  cliclick "m:${w:-1600},${h:-1000}" >/dev/null 2>&1 || true
}

start_capture() {
  park_cursor
  # Keep re-parking while the take runs. The cursor only has to drift near the top of
  # the screen once for macOS to drop the menu bar over the window, and a single nudge
  # would otherwise cost the whole clip.
  ( while true; do park_cursor; sleep 1; done ) >/dev/null 2>&1 & PARK_PID=$!
  screencapture -v -D "$DISPLAY_ID" "$RAW" >/dev/null 2>&1 & CAP_PID=$!
  sleep "$SETTLE"
}

stop_capture() {
  sleep "$SETTLE"
  kill "$PARK_PID" 2>/dev/null || true
  kill -INT "$CAP_PID" 2>/dev/null || true
  wait "$CAP_PID" 2>/dev/null || true
  sleep 1   # let the container finalize
}

# Terminal video is mostly static, so it encodes tiny. Half the retina width keeps text
# crisp (it was captured at 2x) and 30fps is plenty for typing.
encode() {
  local out="$OUT/$1.mp4"
  # Crop the top strip before scaling: it holds the notch bar, Ghostty's title, and
  # the screen-recording indicator dot, none of which belong in the clip. 141px of the
  # 1912-tall capture lands just above the caption border.
  ffmpeg -y -loglevel error -i "$RAW" \
    -vf "crop=in_w:in_h-141:0:141,scale=1624:-2" -r 30 \
    -c:v libx264 -preset slow -crf 24 -tune stillimage \
    -pix_fmt yuv420p -movflags +faststart -an "$out"
  rm -f "$RAW"
  printf '  %-34s %s\n' "$out" "$(du -h "$out" | cut -f1)"
}

main() {
  local acts=("$@"); [ ${#acts[@]} -eq 0 ] && acts=(1 2 3 4)
  mkdir -p "$OUT"
  build_layout
  open_window
  for a in "${acts[@]}"; do
    echo "recording act $a ..."
    reset_state
    start_capture
    run_act "$a"
    stop_capture
    encode "$(name_for "$a")"
  done
  say "done"
  close_window
  echo; echo "clips in $OUT/"
}

main "$@"
