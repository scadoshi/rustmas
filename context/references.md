# External references

Things this repo depends on that live somewhere else. Each entry records what we
rely on, when it was last verified, and how to check it again.

## fornwall/advent-of-code (the solver)

- Source: `https://github.com/fornwall/advent-of-code.git`
- Cloned locally at `~/Work/advent-of-code`
- Last checked: 2026-08-04, at commit `b5d0e717` (authored 2026-07-03)
- Used by: `SolverClient::validate_answer` in `src/lib/outbound/client/solver.rs`

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
through 2025 day 12, which is exactly where our `Year::days_in` stops, so
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
- Day page: `GET /<year>/day/<day>`, same cookie, returns HTML
- Submit endpoint: `POST /<year>/day/<day>/answer`, form-encoded, fields `level`
  (1 or 2) and `answer`
- A `User-Agent` identifying this repo is sent on request, which the site asks
  automated clients to do

Etiquette that shapes the design: do not re-download inputs, do not republish
puzzle text, and expect an escalating cooldown after wrong answers.

### Submit contract

Verified live on 2026-08-06 against 2015 day 1, using a scratch account with no
stars. Every reply is **HTTP 200**, wrong answers included, so the verdict comes
entirely from the body. The message sits in `<article><p>`.

| Body contains | Verdict |
| --- | --- |
| `That's the right answer` | `Correct` |
| `your answer is too high` | `High` |
| `your answer is too low` | `Low` |
| `That's not the right answer` with no direction | `Incorrect` |
| `You gave an answer too recently` | `Cooldown`, with `You have <wait> left to wait` |
| `You don't seem to be solving the right level` | `AlreadySolved` |

Order matters when matching. A directional reply reads "That's not the right
answer; your answer is too high", so it contains the generic phrase too. Check
direction first or every directional miss classifies as generic.

The direction hint is optional and we could not force it. Guessing `1` against
`138`, and `5` against `1771`, both returned the generic wrong answer, while
`999999999` returned too high. The `too low` string is therefore inferred by
symmetry rather than observed, and it is the one row in the table above that has
not been seen in the wild.

Cooldown is one minute after a wrong answer and escalates with repeats. The
remaining time is prose (`1m 0s`, `5s`), which is why `Verdict::Cooldown` holds a
string rather than a `Duration`.

Submitting a correct answer a second time gives `AlreadySolved`, not another
confirmation. AOC is stateful, so that reply is itself the durable record of a
star, which is why no local answer cache was built.

Fixtures for these strings live in the tests at the bottom of
`src/lib/outbound/client/aoc.rs`.

### Day page contract

Verified live on 2026-08-07 across three pages. Each unlocked part is its own
`<article class="day-desc">`, so the article count says which parts you have:

| Page | Articles |
| --- | --- |
| 2015 day 1, both parts starred | 2 |
| 2016 day 1, unstarred | 1 |
| 2021 day 5, unstarred | 1 |

Part two carries `<h2 id="part2">`, but counting articles is enough and needs no
text parsing. Splitting on the literal opening tag works because articles do not
nest.

`html2text` renders each article into markdown-ish text: `##` headings,
backticks for code, `*` for emphasis and bullets. Width 80 is baked into what
gets stored, so reflowing later means storing a wider render.
