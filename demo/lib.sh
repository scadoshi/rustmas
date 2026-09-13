# Shared helpers for the tmux demo driver.
#
# Everything here exists to make a screen recording readable. Commands type out at
# human speed instead of appearing instantly, captions explain each beat, and the
# pauses are long enough to follow. Timings are env-tunable so a take can be redone
# faster or slower without editing the acts.

SESSION="${SESSION:-rustmas-demo}"
TYPE_DELAY="${TYPE_DELAY:-0.045}"  # seconds between keystrokes
BEAT="${BEAT:-1.3}"                # pause after a command's output lands
ACT_GAP="${ACT_GAP:-2.2}"          # pause between acts
CAPTION_FILE="${CAPTION_FILE:-/tmp/rustmas-demo-caption}"

pause() { sleep "${1:-$ACT_GAP}"; }

# Type text into a pane one character at a time, then Enter. tmux send-keys -l sends
# a literal string, so looping over characters gives the typing effect.
type_in() {
  local pane="$1" text="$2" i
  for (( i=0; i<${#text}; i++ )); do
    local ch="${text:$i:1}"
    # A bare ';' is swallowed as a tmux command separator even with -l, which quietly
    # mangles anything containing one. Escaping keeps it literal.
    [ "$ch" = ";" ] && ch='\;'
    tmux send-keys -t "$pane" -l -- "$ch"
    sleep "$TYPE_DELAY"
  done
  tmux send-keys -t "$pane" Enter
}

# Type a command, then wait for its output to settle.
run_in() { type_in "$1" "$2"; pause "${3:-$BEAT}"; }

# Update the caption banner. The caption pane polls this file and only redraws when
# the text changes, which keeps it from flickering during a recording.
say() { printf '%s\n' "$1" > "$CAPTION_FILE"; pause "${2:-1.6}"; }

# Caption pane body. Redraws only on change.
caption_loop() {
  local last="" cur
  tput civis 2>/dev/null   # hide the cursor; it is distracting on camera
  while true; do
    cur=$(cat "$CAPTION_FILE" 2>/dev/null)
    if [ "$cur" != "$last" ]; then
      clear
      printf '  \033[1;36m%s\033[0m\n' "$cur"
      last="$cur"
    fi
    sleep 0.12
  done
}

require() {
  command -v "$1" >/dev/null 2>&1 || { echo "missing: $1 ($2)" >&2; exit 1; }
}
