---
name: iterate-divergence
description: Iterate walker / value changes against the divergence reports. Use when calibrating output quality on existing fixtures, deciding which lever to work on next, or any walker / value change whose effect is judged by `Score(3000)` and the per-fixture report. Pair with `Skill(add-fixture)` only when bringing a new fixture into the corpus; this skill assumes the corpus exists.
---

# iterate-divergence

The divergence reports are the calibration loop's instrument.
Per-fixture reports live in `tests/divergence/<fixture>.md`. The
reports are designed to answer "what should I change next?" at the
top — read them in that spirit. The Top opportunities section on each
fixture report (additive + subtractive subtables) is the brief; lower
sections (per-budget table, rollups, ledger, walker waste) are
reference.

For column semantics, scoring formulas, status labels, and threshold
constants, read the module-level `//!` doc at the top of
`src/divergence.rs` — that's the source of truth.

## Priority

`Score(3000)` is the highest-priority budget — the auto-injection
budget every session hits. Improvements at the other budgets (1K,
9K, and the rest of the vector) are valid lower-priority targets,
particularly useful when `Score(3000)` is close to saturated on a
fixture and the headroom is elsewhere.

A change that improves one budget while regressing another isn't
auto-rejected. Make a judgment call on whether it's net beneficial
to the long-term health of the project, and flag the tradeoff in
the commit.

Complexity is a cost. A small score gain may not be worth the code
it adds — judgment call. Net code-removing changes at flat metric
are still a ship.

## 1. Survey the corpus

Survey from per-fixture score lines (each report's first line is
`scores: Score(3000)=X.XXX ns_rows≤3K=N/T (...)`):

- Quick scan, filename-ordered: `head -1 tests/divergence/*.md`
- Sorted by Score(3000) ascending (bottom-N stuck fixtures):
  `grep -H '^scores:' tests/divergence/*.md | sort -t= -k2 -g`

The bottom 3–5 fixtures are usually where iteration focuses.
Patterns across fixtures are the strongest signal. Scan for:

- A descriptor pattern appearing in many fixtures' Top opportunities
  (either subtable).
- A path appearing in many fixtures' Top missed paths or Top wasted
  paths rollups.
- A diagnosis section (`wrong-slice`, `ranking-recoverable`, etc.)
  carrying most of the gap on multiple fixtures.

For column semantics, see the module-level `//!` doc on
`src/divergence.rs`.

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

1. **Per-budget table**. Where on the budget axis does the walker
   fall off? A walker that scores 0.6 at 3k but 0.3 at 9k has a
   different problem than one that scores 0.3 at 3k and climbs to
   0.6 at 9k.
2. **Top opportunities** section. Two subtables — additive (`gap@B`,
   dimensionless rank-weighted priority for closing partial/missing
   rows) and subtractive (`freed@B`, raw tokens the walker is putting
   into atoms outside `A_B`'s NS atom set). Different units, same
   prominence. Look at both — additive and subtractive interventions
   are equally valid moves. The `gap@1k`/`gap@3k`/`gap@9k` and
   `freed@1k`/`freed@3k`/`freed@9k` vectors show how each
   intervention's leverage decays across budgets — wide spread means
   the intervention helps disproportionately at low B; flat means it
   helps everywhere. `gap@3k` and `freed@3k` are each subtable's sort
   key. The companion **Top missed paths** rollup (top of arrival
   ledger) and **Top wasted paths** rollup (top of walker waste)
   surface where on the filesystem missing vs. wasted content
   concentrates — pair them to spot path-mismatch suppression
   candidates.
3. **Arrival ledger**, only the section for the diagnosis you're
   working on, and only the rows you need as evidence — usually 3–5
   per fixture. Other ledger sections and the walker-waste detail
   below are reference; read them if the Top opportunities seem off.

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

Regenerates every per-fixture report in one pass. For read-only
spot-checks of a single fixture: `cargo t fixture_baselines_<name>`.

## 5. Read the diff across the whole corpus

```bash
git diff tests/divergence/
```

Always look at all fixtures, not just the ones you targeted — a
change that helps the targeted set may regress others, and the
corpus-wide diff is the cheap signal.

Success criteria, in order of importance:

1. **The targeted score moved** in the right direction on the
   targeted fixture(s). If another budget regressed, see the
   tradeoff guidance under Priority — judgment call on net long-
   term project benefit, flagged in the commit.
2. **The change generalizes.** The walker / value rule should apply
   wherever the relevant condition holds, not only on the corpus
   fixtures. If you can't articulate the rule independent of which
   fixture rows it helps, the change is overfit.
3. **`gap@3k` dropped on the targeted opportunity** on the fixtures
   you targeted. If it didn't, the change didn't address what you
   thought.
4. **Complexity paid for itself.** A small score gain may not be
   worth the code added — judgment call on whether the rule earns
   its complexity. A code-removing or heuristic-simplifying change
   at flat metric is a ship.
5. **The dominant diagnosis may shift** on individual fixtures —
   that's fine when the change genuinely fixed one bucket and
   another now dominates.

Bucket counts and `gap@3k` are heuristic-derived attention directors,
not the optimization target. A change that drops a bucket count
without moving `Score(3000)` means the metric was overstating
headroom on those rows; the intervention didn't recover what you
thought.

After non-trivial changes, run `/simplify`.

## 6. Commit each coherent improvement

A coherent improvement is one walker / value change with measurable
positive impact on the targeted score on the targeted fixture(s),
no material unflagged regressions elsewhere, and an explainable
mechanism. Each gets its own commit; commit messages should focus
on *why* the change makes the walker better in general, not on
which specific fixture rows it helped.

How many improvements to attempt in a session is the user's call —
this skill drives one improvement at a time. After each commit, the
user decides whether to continue iterating or stop.

### Tune mixed results before reverting

When an attempt wins on the target but regresses elsewhere, the
structural insight is usually correct and the magnitudes just need
calibration. Tune values before reverting. Reverting *the current
attempt* is for fundamentally misaligned changes, not for first-pass
numbers. (This is about polishing the change you're working on now;
un-shipping past commits is a separate avenue, below.)

## Un-shipping as an iteration avenue

Distinct from tuning the change you're currently working on:
reverting a *prior* commit is a valid forward iteration in its own
right. The corpus inherits walker / value complexity from earlier
iterations; some of it may not be paying for itself under the
current metric. Try reverting a candidate commit, regenerating
baselines, checking whether `Score(3000)` stays flat or improves.
If it does, un-ship — code reduction at flat metric is a ship.
Same success criteria, same commit hygiene as forward changes.

## When stuck

A divergence that resists general fixes is a sign to consult more
perspectives.

- Spawn an Agent for independent brainstorming on the walker /
  value design space.
- Run `codex-companion task` to delegate ideation or analysis to
  Codex.
- Re-read the `## Divergence diagnostic — deferred architectural
  items` section in `docs/design-notes.md` — current diagnosis labels
  can be off when the diagnostic itself is approximating.

## Anti-Goodhart discipline

The reports are heuristic dashboards over the per-budget Score
vector with `Score(3000)` as the highest-priority budget. They
direct attention; they don't define success.

- **Don't optimize `gap@3k` directly.** It's a non-additive priority
  score, not a `Score(3000)` delta. A change that drops `gap@3k`
  without moving `Score(3000)` is suspect.
- **Always check the full `git diff tests/divergence/` after each
  iteration.** A change that helps the targeted fixture(s) may
  regress others; the corpus-wide diff is the cheap signal.
- **Don't change North Star files to match the walker.** That's
  moving the goalpost. Frozen NSs are the calibration target; the
  walker has to come to them.
- **Diagnosis labels are heuristic.** When a fixture's dominant
  diagnosis doesn't match what your code change targeted, trust the
  code change first and inspect what the diagnosis missed — not the
  other way around.
- **Verify suppression candidates against North Stars.** Before
  treating a content class as a suppression candidate, grep
  `tests/north-stars/` for whatever discriminator you're considering
  — folder, file extension, file name, symbol pattern. A
  discriminator that hits NS in one fixture but is pure waste in
  others is a refinement target, not a suppression target.

## Things to *not* do

- Don't make changes tuned to a specific fixture's NS without a
  plausible generalization. The rule has to apply wherever the
  relevant condition holds; the evidence can be one fixture if the
  rule is general.
- Don't optimize bucket counts or `gap@3k` as a goal — they direct
  attention, they aren't the metric. The metric is `Score(B)`,
  highest-priority at B=3000.
- Don't ship neutral or marginally-positive changes that *add* code.
  Complexity has to pay for itself.
- Don't commit a "set of improvements" — each coherent change gets
  its own commit.
