#!/usr/bin/env bash
# Runs the rustmas demo in a tmux session you attach to yourself.
#
#   ./demo/run.sh                   full run
#   ./demo/run.sh 4                 just act 4 (the cache)
#   TYPE_DELAY=0.02 ./demo/run.sh   faster rehearsal pass
#
# Attach with `tmux attach -t rustmas-demo`. To record, use demo/record.sh.

set -euo pipefail
cd "$(dirname "$0")/.."
source demo/lib.sh
source demo/acts.sh

require tmux "brew install tmux"

build_layout
case "${1:-all}" in
  all) for a in 1 2 3 4; do reset_state; run_act "$a"; done ;;
  *)   run_act "$1" ;;
esac
say "done"
