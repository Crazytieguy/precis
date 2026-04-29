---
name: iterate-divergence
description: Iterate walker / value changes against the divergence reports. Use when calibrating output quality on existing fixtures, deciding which lever to work on next, or any walker / value change whose effect is judged by `Score(3000)` and the per-fixture report. Pair with `Skill(add-fixture)` only when bringing a new fixture into the corpus; this skill assumes the corpus exists.
---

# iterate-divergence

The divergence reports are the calibration loop's instrument.
`tests/divergence/OVERVIEW.md` is the corpus index; per-fixture reports
live alongside it. The reports are designed to answer "what should I
change next?" at the top — read them in that spirit. The verdict
block and Top opportunities table on each fixture report are the
brief; lower sections (per-budget table, rollups, ledger, walker
waste) are reference to consult as needed.

For column semantics, scoring formulas, status labels, and threshold
constants, read the module-level `//!` doc at the top of
`src/divergence.rs` — that's the source of truth.

## 1. Survey the corpus

Start at `tests/divergence/OVERVIEW.md`. Rows are sorted by
`Score(3000)` ascending. Each row carries the full per-budget
`Score(B)` vector across the seven-budget grid `[1000, 1442, 2080,
3000, 4327, 6240, 9000]`, plus `verdict`, `likely primary lever`,
`evidence` (bucket counts with `rank×gap`), and `loss reasons`. The
vector exposes walker-shape signal a scalar would hide: front-loader
walkers score high at 1k and crash at 9k; trailing-loader walkers do
the reverse. `Score(3000)` is the primary objective (auto-injection
budget every session hits) but watch the rest — accepting a regression
at 1k or 9k while moving 3k is a calibration tradeoff, not free win.

Patterns across fixtures are the strongest signal. Scan for:

- A verdict label that recurs across many low-`Score(3000)` fixtures
  (e.g. `wrong-slice bound` dominating most of the corpus).
- A loss reason that recurs across `parent-gating bound` fixtures
  (e.g. `predecessor not scheduled` showing up consistently with
  `go decl at <file>` predecessors).
- A descriptor pattern that appears in many fixtures' Top
  opportunities (e.g. `promote pub-item names surfaces` showing up in
  multiple Rust fixtures).

A pattern visible in only one fixture is still actionable if the
underlying issue would plausibly show up in other real-world
codebases — fixtures sample the distribution but don't cover it. The
test is whether the *change* is general (a walker / value rule that
applies whenever the relevant condition holds), not whether the
*evidence* spans many fixtures. Avoid changes that are tuned for a
specific fixture's NS without a plausible generalization.

## 2. Drill into specific reports as evidence

Once a candidate pattern is in mind, open the per-fixture reports
that exemplify it (one is enough if it's clear; 2–4 if you want to
check the pattern generalizes). For each:

1. **Verdict block**. Confirms the pattern fits the fixture.
2. **Per-budget table**. Where on the budget axis does the walker
   fall off? A walker that scores 0.6 at 3k but 0.3 at 9k has a
   different problem than one that scores 0.3 at 3k and climbs to
   0.6 at 9k.
3. **Top opportunities** table. The `bands ≤3k/≤6k/total` column
   shows how much of the impact lands under the default 3k product
   budget vs. higher tiers. A change that only helps `>6k` rows
   doesn't move the default user experience.
4. **Arrival ledger**, only the section for the diagnosis you're
   working on, and only the rows you need as evidence — usually 3–5
   per fixture. The `comp` column shows byte-weighted batch
   completion: `comp ∈ (0.2, 0.8)` is "started but not finished",
   the band the `finish partially-delivered NS batches` opportunity
   surfaces. Other ledger sections and the rollups below are
   reference; read them if the verdict or opportunities seem off.

The goal at this step is to verify the corpus-level pattern holds in
specific fixtures and to understand what walker / value behavior is
producing it.

## 3. Make a general change

In `src/walker/<lang>.rs` or `src/value.rs`. Every change should
improve the walker across the distribution of real-world codebases —
fixtures are samples of that distribution, not targets in themselves.
Prefer an accepted divergence to a fixture-specific heuristic.

## 4. Regenerate baselines

```bash
UPDATE_BASELINES=1 cargo t fixture_baselines
```

Regenerates every per-fixture report and `OVERVIEW.md` in one pass.
For read-only spot-checks of a single fixture: `cargo t
fixture_baselines_<name>`.

## 5. Read the diff across the whole corpus

```bash
git diff tests/divergence/
```

Always look at all fixtures, not just the ones you targeted — a
change that helps the targeted set may regress others, and the
corpus-wide diff is the cheap signal.

Success criteria, in order of importance:

1. **`Score(3000)` actually moved** in the right direction on the
   targeted fixture(s), with no material regressions elsewhere on the
   primary budget. Watch the rest of the per-budget vector too — a 3k
   win that comes with a big 1k or 9k regression is a tradeoff, not a
   free win.
2. **The targeted opportunity's `rank×gap` dropped** on the fixtures
   you targeted. If it didn't, the change didn't address what you
   thought.
3. **The verdict label may shift** on individual fixtures — that's
   fine when the change genuinely fixed one bucket and another now
   dominates.

Bucket counts and `rank×gap` are heuristic-derived attention
directors, not the optimization target. A change that drops a bucket
count without moving `Score(3000)` means the metric was overstating
headroom on those rows; the intervention didn't recover what you
thought.

After non-trivial changes, run `/simplify`.

## 6. Commit each coherent improvement

A coherent improvement is one walker / value change with measurable
positive `Score(3000)` impact on the targeted fixture(s), no material
regressions elsewhere on the primary budget or the rest of the
vector, and an explainable mechanism. Each gets its own commit;
commit messages should focus on *why* the change makes the walker
better in general, not on which specific fixture rows it helped.

How many improvements to attempt in a session is the user's call —
this skill drives one improvement at a time. After each commit, the
user decides whether to continue iterating or stop.

### Tune mixed results before reverting

When an attempt wins on the target but regresses elsewhere, the
structural insight is usually correct and the magnitudes just need
calibration. Tune values before reverting. Reverting is for
fundamentally misaligned changes, not for first-pass numbers.

## When stuck

A divergence that resists general fixes is a sign to consult more
perspectives.

- Spawn an Agent for independent brainstorming on the walker /
  value design space.
- Run `codex-companion task` to delegate ideation or analysis to
  Codex.
- Re-read the `## Divergence diagnostic — deferred architectural
  items` section in `docs/design-notes.md` — current verdict labels
  can be off when the diagnostic itself is approximating.

## Anti-Goodhart discipline

The reports are heuristic dashboards over the per-budget Score
vector with `Score(3000)` as the primary objective. They direct
attention; they don't define success.

- **Don't optimize `rank×gap` directly.** It's a non-additive
  priority score, not a Score(3000) delta. A change that drops
  `rank×gap` without moving `Score(3000)` is suspect.
- **Always check the full `git diff tests/divergence/` after each
  iteration.** A change that helps the targeted fixture(s) may
  regress others; the corpus-wide diff is the cheap signal. Pay
  attention to the whole vector — accepting a 3k win that comes with
  a 1k or 9k regression is a tradeoff worth flagging in the commit
  message.
- **Don't change North Star files to match the walker.** That's
  moving the goalpost. Frozen NSs are the calibration target; the
  walker has to come to them.
- **The verdict is heuristic.** When a fixture's primary lever
  doesn't match what your code change targeted, trust the code change
  first and inspect what the verdict missed — not the other way
  around.

## Things to *not* do

- Don't make changes that are tuned to a specific fixture's NS
  without a plausible generalization. The change should be a walker
  / value rule that applies whenever the relevant condition holds —
  the evidence can be one fixture if the rule is general.
- Don't optimize bucket counts or `rank×gap` as a goal. They direct
  attention. `Score(3000)` is the primary metric and the rest of the
  vector is the watch list.
- Don't make fixture-specific patches; pursue general walker / value
  changes.
- Don't commit a "set of improvements" — each coherent improvement
  gets its own commit.
