# External references

Things this repo depends on that live somewhere else. Each entry records what we
rely on, when it was last verified, and how to check it again.

## fornwall/advent-of-code (the solver)

- Source: `https://github.com/fornwall/advent-of-code.git`
- Cloned locally at `~/Work/advent-of-code`
- Last checked: 2026-08-04, at commit `b5d0e717` (authored 2026-07-03)
- Used by: `Session::validate_answer` in `src/lib/session/mod.rs`

Hosts, all running the same WASM solver. Listed in `AOC_SOLVER_BASE_URLS`:

| URL | Platform |
| --- | --- |
| `https://advent.fly.dev` | Fly |
| `https://aoc.fornwall.workers.dev` | Cloudflare Worker |
| `https://mystifying-blackwell-9e705f.netlify.app` | Netlify Function |

Interactive docs at `https://aoc.fornwall.net/api/`, described by an
`openapi.json` linked from that page.

### Contract we rely on

`POST /solve/{year}/{day}/{part}` with the puzzle input as the raw body. The
answer comes back as `text/plain`, no trailing newline observed.

Only two status codes exist. `crates/http-server/src/main.rs` maps every `Ok` to
200 and every `Err` to 400, and `crates/worker/src/lib.rs` does the same for
Cloudflare. Nothing else comes from the application, so any 5xx is the hosting
platform rather than the solver.

Because failures are all 400, the reason lives in the body:

| Body | Meaning |
| --- | --- |
| `Unsupported year=…, day=…, part=…` | No implementation for that puzzle |
| `Invalid input` | Day is implemented, our input did not parse |
| `Invalid day 26 - must be 1-25` | Out of range, unreachable from our `Day` type |
| `Invalid part 26 - must be 1-2` | Out of range, unreachable from our `Part` type |

The `Unsupported` string is generated at `crates/core/src/lib.rs:364`, the
catch-all arm of a hand-written `match (year, day)`. Coverage on `main` runs
through 2025 day 12, which is exactly where our `days_in_year` stops, so
`Unsupported` cannot currently be triggered by a `Day` we can construct.

### How this was verified

Probed live with curl for status codes, then confirmed against the cloned source
rather than inferred from responses alone. Both agree. Re-run the probe and
re-read `crates/core/src/lib.rs` if behaviour looks different.

### Watch for

Coverage extending past 2025 day 12 when a new event runs. If the solver lags a
live event, `Unsupported` starts appearing for days we can legitimately ask
about.

## adventofcode.com

- Input endpoint: `GET /<year>/day/<day>/input`, authenticated by a `session`
  cookie read from `COOKIE`
- Submit endpoint: `POST /<year>/day/<day>/answer`, form field `level` taking 1
  or 2, not yet implemented
- A `User-Agent` identifying this repo is sent on request, which the site asks
  automated clients to do

Etiquette that shapes the design: do not re-download inputs, do not republish
puzzle text, and expect an escalating cooldown after wrong answers.
