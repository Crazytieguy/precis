---
name: iterate-divergence
description: Iterate walker / value changes against the divergence reports. Use when calibrating output quality on existing fixtures, deciding which lever to work on next, or any walker / value change whose effect is judged by `Score(3000)` and the per-fixture report. Pair with `Skill(add-fixture)` only when bringing a new fixture into the corpus; this skill assumes the corpus exists.
---

# iterate-divergence

The divergence reports are the calibration loop's instrument.
Per-fixture reports live in `tests/divergence/<fixture>.md`. Each
report is a single interleaved schedule table — NS batches and walker
batches merged onto a shared `cum_tokens` axis with `Score(B = cum)`
on every row.

For column semantics and scoring formula, read the module-level
`//!` doc at the top of `src/divergence.rs`.

## Validation fixtures are held out — do not open them

`tests/validation/<fixture>.md` holds a separate, held-out corpus
sampled from the GitHub language distribution. Each file is a single
`Score(3000)=…` headline line — there is no per-row table, by
design. **Do not survey, open, or iterate against these files.** The
calibration loop targets only `tests/divergence/`.

The holdout extends to anything fixture-specific for validation
repos. **Off-limits during calibration:**

- `tests/validation/<fixture>.md` (the score files)
- `tests/north-stars/<fixture>.toml` for any fixture registered with
  `per_validation_fixture_tests!` — the NS is the answer key, and
  the dir is shared with training only because building two parallel
  dirs would be process bloat for the same convention
- `tests/fixtures/<fixture>/` for the same set — the fixture source
  is what the answer key is grounded in

If a change moves a validation score, that's information — but the
debugging step is to read the *training* reports and reason about
generality, not to read any held-out fixture's source or NS.
Acceptable outcomes when a validation score drops: fix the walker
in general (using training reports) and re-run; or accept the move
because the broader change is a net win on training. Not acceptable:
opening the validation NS to figure out what content the score is
weighting.

## Priority

`Score(3000)` is the highest-priority budget — the auto-injection
budget every session hits. Outside that, prefer improvements that
shift Score in early budgets, since gains there cascade through later
rows. All budgets in the report are valid optimization targets.

Complexity is a cost. A small score gain may not be worth the code
it adds — judgment call. Net code-removing changes at flat metric
are still a ship.

## 1. Find an improvement candidate

Start breadth-first; narrow as the candidate sharpens. The stopping
condition is identifying one high-quality candidate — not exhaustive
analysis.

Survey with `head -1 tests/divergence/*.md` (training only — leave
`tests/validation/` alone, see the held-out note above). Open many
of the lowest `Score(3000)` fixtures — the broadest, highest-value
rules show up as patterns that recur across several weak fixtures,
and the wider the initial scan, the more such patterns are visible.
Narrow the fixture set as the candidate sharpens.

On each fixture, read enough rows to identify a candidate; rows
past where the pattern is clear usually aren't informative.

How the Score trajectory typically reads: walker rows that deliver
NS-aligned atoms drive Score up; new NS rows usually drop Score
because they grow `A_B` with content the walker may not have
delivered. A stretch of walker rows holding Score near 1.0 followed
by an NS row dropping it sharply means the walker was rendering
content NS doesn't prioritize at that budget.

**Predecessor column.** `NsBatch.predecessor` on NS rows shows a
logical dependency on an earlier NS batch (refinement chain or
semantic prerequisite). The score metric doesn't penalize
predecessor violations directly — but if the walker delivers atoms
for a dependent NS row before delivering atoms for its predecessor,
that's an NS-preference violation the metric won't catch, and a
valid improvement target.

Watch for things that look broken even when Score is OK: 0-cost
walker batches that should be dropped, descriptors that look wrong,
etc. Often higher-priority than score-tuning.

For NS justifications or the concrete content of an interesting
row, grep into `tests/north-stars/` or `tests/snapshots/schedule/`.
Avoid reading whole schedule TOMLs — they're large.

The change has to apply wherever the relevant condition holds, not
just on the fixture(s) where you spotted it. Fixtures sample the
distribution; they aren't the target. A pattern visible in only one
fixture is still actionable if the underlying issue would plausibly
show up in other real-world codebases — but you have to be able to
articulate the rule independent of which fixture rows it helps.

## 2. Make a general change

In `src/walker/<lang>.rs` or `src/value.rs`. Every change should
improve the walker across the distribution of real-world codebases —
fixtures are samples of that distribution, not targets in themselves.
Prefer an accepted divergence to a fixture-specific heuristic.

In a worktree (e.g. a parallel lane), run `bash scripts/lane-setup.sh`
from the worktree root before the first build. It refuses to run in
the main checkout, links `tests/fixtures`, and seeds `target/` with a
copy-on-write clone of the main checkout's, so only the precis crate
recompiles.

## 3. Regenerate baselines

```bash
UPDATE_BASELINES=1 cargo t fixture_baselines
```

Regenerates every per-fixture report in one pass. For read-only
spot-checks of a single fixture: `cargo t fixture_baselines_<name>`.

## 4. Read the diff across the whole corpus

```bash
git diff tests/divergence/
```

Always look at all fixtures, not just the ones you targeted — the
corpus-wide diff is the cheap signal.

Success criteria, in order of importance:

1. **The targeted score moved** in the right direction on the
   targeted fixture(s).
2. **The change generalizes.** The walker / value rule should apply
   wherever the relevant condition holds, not only on the corpus
   fixtures. If you can't articulate the rule independent of which
   fixture rows it helps, the change is overfit.
3. **The Score-per-row curve climbed** on the targeted rows in the
   targeted fixtures. If it didn't, the change didn't address what
   you thought.
4. **Complexity paid for itself.** A small score gain may not be
   worth the code added. A code-removing or heuristic-simplifying
   change at flat metric is a ship.

After non-trivial changes, run `/code-review`.

## 5. Commit each coherent improvement

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
structural insight could still be correct and the magnitudes just
need calibration. Tune values before reverting. Reverting *the
current attempt* is for fundamentally misaligned changes, not for
first-pass numbers. (This is about polishing the change you're
working on now; un-shipping past commits is a separate avenue,
below.)

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
- Delegate ideation or analysis to a GPT model via the
  `model-router:gpt-5.6-sol(high)` agent type, or get a critique of a
  shipped change from `model-router:adversarial-code-reviewer`.

## Things to *not* do

- Don't make changes tuned to a specific fixture's NS without a
  plausible generalization. The rule has to apply wherever the
  relevant condition holds; the evidence can be one fixture if the
  rule is general.
- Don't change North Star files to match the walker. That's moving
  the goalpost. Frozen NSs are the calibration target; the walker
  has to come to them.
- **Don't read or iterate against held-out fixtures.** The holdout
  covers `tests/validation/<name>.md`, the corresponding
  `tests/north-stars/<name>.toml`, and `tests/fixtures/<name>/` for
  any fixture registered with `per_validation_fixture_tests!` in
  `tests/fixture_baselines.rs`. Opening any of these to debug a low
  validation score is overfitting to the holdout.
- Don't ship neutral or marginally-positive changes that *add* code.
  Complexity has to pay for itself.
- Don't commit a "set of improvements" — each coherent change gets
  its own commit.
