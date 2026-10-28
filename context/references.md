# External references

Things this repo depends on that live somewhere else. Each entry records what we rely on, when it was last verified, and how to check it again.

## fornwall/advent-of-code (the solver)

- Source: `https://github.com/fornwall/advent-of-code.git`
- Clone it locally and read `crates/core/src/lib.rs` for coverage
- Last checked: 2026-08-04, at commit `b5d0e717` (authored 2026-07-03)
- Used by: `SolverClient::validate_answer` in `src/lib/outbound/client/solver_client.rs`

Three hosts run the same WASM solver. `BASE_URLS` in `solver_client.rs` lists them, tried in order:

| URL | Platform |
| --- | --- |
| `https://advent.fly.dev` | Fly |
| `https://aoc.fornwall.workers.dev` | Cloudflare Worker |
| `https://mystifying-blackwell-9e705f.netlify.app` | Netlify Function |

Interactive docs are at `https://aoc.fornwall.net/api/`, described by an `openapi.json` linked from that page.

### Contract we rely on

`POST /solve/{year}/{day}/{part}` with the puzzle input as the raw body. The answer comes back as `text/plain`, with no trailing newline observed.

Only two status codes exist:

- `crates/http-server/src/main.rs` maps every `Ok` to 200 and every `Err` to 400.
- `crates/worker/src/lib.rs` does the same for Cloudflare.

Nothing else comes from the application, so any 5xx is the hosting platform, not the solver.

Because every failure is a 400, the reason lives in the body:

| Body | Meaning |
| --- | --- |
| `Unsupported year=…, day=…, part=…` | No implementation for that puzzle |
| `Invalid input` | Day is implemented, our input did not parse |
| `Invalid day 26 - must be 1-25` | Out of range, unreachable from our `Day` type |
| `Invalid part 26 - must be 1-2` | Out of range, unreachable from our `Part` type |

The `Unsupported` string comes from `crates/core/src/lib.rs:364`, the catch-all arm of a hand-written `match (year, day)`. Coverage on `main` runs through 2025 day 12, exactly where our `Year::days_in` stops. A `Day` we can construct can't currently trigger `Unsupported`.

### How this was verified

1. Probed live with curl for status codes.
2. Confirmed against the cloned source rather than inferred from responses alone.

Both agree. If behavior looks different, re-run the probe and re-read `crates/core/src/lib.rs`.

### Watch for

Coverage extending past 2025 day 12 when a new event runs. If the solver lags a live event, `Unsupported` starts appearing for days we can legitimately ask about.

## adventofcode.com

- Input endpoint: `GET /<year>/day/<day>/input`, authenticated by a `session` cookie read from `COOKIE`
- Day page: `GET /<year>/day/<day>`, same cookie, returns HTML
- Submit endpoint: `POST /<year>/day/<day>/answer`, form-encoded, fields `level` (1 or 2) and `answer`
- Every request sends a `User-Agent` built from `REPO_URL` and `CONTACT`, since the site asks automated clients to identify themselves

The site's etiquette shapes the design:

- Don't re-download inputs.
- Don't republish puzzle text.
- Expect an escalating cooldown after wrong answers.

### Submit contract

Verified live on 2026-08-06 against 2015 day 1, using a scratch account with no stars. Every reply is **HTTP 200**, wrong answers included, so the verdict comes entirely from the body. The message sits in `<article><p>`.

| Body contains | Verdict |
| --- | --- |
| `That's the right answer` | `Correct` |
| `your answer is too high` | `High` |
| `your answer is too low` | `Low` |
| `That's not the right answer` with no direction | `Incorrect` |
| `You gave an answer too recently` | `Cooldown`, with `You have <wait> left to wait` |
| `You don't seem to be solving the right level` | `AlreadySolved` |
| `auth/login` | `NotLoggedIn` |
| none of the above | `Unrecognized`, with the start of the message |

The logged-out case was verified live on 2026-09-17.

- With no valid session, the POST answers **303** to the puzzle page. `reqwest` follows it to a 200, so the body is the puzzle, not a verdict.
- That page's nav links `auth/login`, where a logged-in page links `auth/logout`.
- `auth/login` is matched first, since nothing was graded.
- A malformed cookie gets a **500** instead, which the status check already rejects.

Order matters when matching. A directional reply reads "That's not the right answer; your answer is too high", so it contains the generic phrase too. Check direction first, or every directional miss classifies as generic.

The direction hint is optional, and we couldn't force it. Guessing `1` against `138` and `5` against `1771` both returned the generic wrong answer, while `999999999` returned too high.

So the `too low` string is inferred by symmetry, not observed. It's the one row in the table above nobody has seen in the wild.

Cooldown is one minute after a wrong answer and escalates with repeats. The remaining time is prose (`1m 0s`, `5s`), which is why `AocVerdict::Cooldown` holds a string rather than a `Duration`.

Submitting a correct answer a second time gives `AlreadySolved`, not another confirmation. AOC is stateful, so that reply is itself the durable record of a star. That's why no local answer cache was built.

Fixtures for these strings live in the tests at the bottom of `src/lib/outbound/client/aoc_client.rs`.

### Day page contract

Verified live on 2026-08-07 across three pages. Each unlocked part is its own `<article class="day-desc">`, so the article count says which parts you have:

| Page | Articles |
| --- | --- |
| 2015 day 1, both parts starred | 2 |
| 2016 day 1, unstarred | 1 |
| 2021 day 5, unstarred | 1 |

Part two carries `<h2 id="part2">`, but counting articles is enough and needs no text parsing. Splitting on the literal opening tag works because articles don't nest.

`html2text` renders each article into markdown-ish text: `##` headings, backticks for code, `*` for emphasis and bullets. Width 80 is baked into what gets stored, so reflowing later means storing a wider render.
