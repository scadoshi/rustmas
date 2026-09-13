# Demo acts and tmux layout for rustmas, sourced by run.sh and record.sh.
#
# rustmas is a one-command tool rather than a client/server, so most acts need a
# single working pane. Only the last one splits, to put the cache on disk beside
# the run that reads it.

CAP=0.0; MAIN=0.1; SIDE=0.2

# A short prompt for the demo panes only, set before capture starts so the command
# that sets it never appears. The default prompt is 31 characters of username and
# host, which is a quarter of a pane once the font is big enough to read in a GIF.
# %1~ tracks the directory, so it stays right when an act cds elsewhere.
set_prompt() {
  # PROMPT_EOL_MARK is the reverse-video % zsh prints when output ends mid-line.
  # Harmless in daily use, distracting at the top of every pane in a recording.
  tmux send-keys -t "$1" "export PS1='%1~ \$ ' PROMPT_EOL_MARK=''" Enter
  tmux send-keys -t "$1" 'clear' Enter
  sleep 0.15
  tmux clear-history -t "$1"   # drop the scrollback too, or the old prompt shows through
}

build_layout() {
  tmux kill-session -t "$SESSION" 2>/dev/null || true
  rm -f "$CAPTION_FILE"; : > "$CAPTION_FILE"
  cargo build --release --quiet

  tmux new-session -d -s "$SESSION" -x "$(tput cols)" -y "$(tput lines)"
  tmux set-option -t "$SESSION" -g status off
  tmux set-option -t "$SESSION" -g pane-border-status top
  tmux set-option -t "$SESSION" -g pane-border-format ' #{pane_title} '

  tmux split-window -v -b -l 3 -t "$SESSION":0.0

  CAP="$SESSION":0.0; MAIN="$SESSION":0.1
  tmux select-pane -t "$CAP"  -T 'rustmas'
  tmux select-pane -t "$MAIN" -T 'rustmas'

  set_prompt "$MAIN"
  tmux send-keys -t "$CAP" "source demo/lib.sh; caption_loop" Enter
  sleep 0.6
}

# Only the cache act wants a second pane. Add or drop it so no act carries an
# empty one, and re-pin afterwards since removing a pane re-balances the rest.
want_side_pane() {
  local have
  have=$(tmux list-panes -t "$SESSION" | wc -l | tr -d ' ')
  if [ "$1" = "yes" ] && [ "$have" -lt 3 ]; then
    tmux split-window -h -l 50% -t "$SESSION":0.1
    tmux select-pane -t "$SESSION":0.2 -T 'cache/'
    set_prompt "$SESSION":0.2
    SIDE="$SESSION":0.2
  elif [ "$1" = "no" ] && [ "$have" -ge 3 ]; then
    tmux kill-pane -t "$SESSION":0.2 2>/dev/null || true
  fi
  sleep 0.3
}

pin_layout() {
  tmux resize-pane -t "$CAP" -y 2 2>/dev/null || true
  sleep 0.3
}

reset_state() {
  local idx
  for idx in $(tmux list-panes -t "$SESSION" -F '#{pane_index}' | grep -v '^0$'); do
    tmux send-keys -t "$SESSION":0."$idx" C-c; sleep 0.2
    tmux send-keys -t "$SESSION":0."$idx" 'clear' Enter
  done
  : > "$CAPTION_FILE"
  sleep 0.4
}

# The binary reads CARGO_MANIFEST_DIR to find the project root, so it runs
# through cargo rather than as a bare path. Release, because debug builds drag
# on the brute-force days.
RM='cargo run --release --quiet'

act1_run_a_day() {
  say "rustmas: Advent of Code tooling in Rust. Fetch the puzzle, run it, check it, submit it."
  say "Let's run a day. 2015, day one."
  run_in "$MAIN" "$RM solve -y 2015 -d 1" 2.5
  say "Both parts, and what each one cost. The timings never include the network."
  pause 2
  say "Nothing was downloaded just now. The input was already cached, so this ran offline."
  pause 2
}

act2_independent_solver() {
  say "Those answers are unchecked though. Are they right?"
  run_in "$MAIN" "$RM solve -y 2015 -d 1" 2.0
  say "There is a second implementation of every puzzle out there that needs no account."
  say "rustmas can ask it, which turns someone else's solver into a regression check."
  run_in "$MAIN" "$RM solve -y 2015 -d 1 --validate" 3.0
  say "Correct, both parts. Submitting gates on this, because a wrong answer costs a cooldown."
  pause 3
}

act3_every_year() {
  say "The filters are optional. Drop them and it runs everything you have written."
  run_in "$MAIN" "clear" 0.4
  run_in "$MAIN" "$RM solve" 4.0
  say "Eleven years, and it knows which days exist without holding their inputs."
  say "Unwritten days are skipped before anything is downloaded for them."
  pause 2
  say "Then where the time went, including the slowest part across the whole run."
  pause 3
}

act4_the_cache() {
  want_side_pane yes; pin_layout
  say "Fetching puts the whole puzzle on disk, not just the input."
  run_in "$SIDE" 'ls -la cache/2015/01/' 2.5
  say "The text of both parts, the input, and one more file worth explaining."
  run_in "$SIDE" 'head -4 cache/2015/01/part_one.md' 2.5
  run_in "$SIDE" 'head -c 60 cache/2015/01/input.txt && echo' 2.0
  say "That last file is a SHA-256 of the session cookie that fetched this input."
  run_in "$SIDE" 'cat cache/2015/01/session && echo' 2.0
  say "It exists because swapping accounts once silently invalidated every cached input."
  say "Day one answered 280 one day and 138 the next. Only the changed answers gave it away."
  run_in "$MAIN" "$RM solve -y 2015 -d 1" 2.5
  say "Now a mismatched hash refetches instead of quietly solving the wrong puzzle."
  pause 3
}

run_act() {
  case "$1" in
    1) want_side_pane no; pin_layout; act1_run_a_day ;;
    2) want_side_pane no; pin_layout; act2_independent_solver ;;
    3) want_side_pane no; pin_layout; act3_every_year ;;
    4) act4_the_cache ;;
    *) echo "unknown act: $1" >&2; return 1 ;;
  esac
}
