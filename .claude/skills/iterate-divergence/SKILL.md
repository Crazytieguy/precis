---
name: iterate-divergence
description: Iterate walker / value changes against the divergence reports. Use when calibrating output quality on existing fixtures, deciding which lever to work on next, or any walker / value change whose effect is judged by `Score(3000)` and the per-fixture report. Pair with `Skill(add-fixture)` only when bringing a new fixture into the corpus; this skill assumes the corpus exists.
---

# iterate-divergence

Each training fixture has a report at `tests/divergence/<fixture>.md`:
a headline, then one table merging NS batches and walker batches on a
shared cumulative-token axis, with `Score(B=cum)` on every row. The
score formula and the columns are documented at the top of
`src/divergence.rs`.

## Validation fixtures are held out

The fixtures in the `validation` block of `tests/data/fixtures.rs` exist
to catch overfitting. Never open, grep or run precis on anything
fixture-specific for them: `tests/validation/<fixture>.md`,
`tests/north-stars/<fixture>.toml` (the answer key, sharing the directory
with training NSs), or `tests/fixtures/<fixture>/`. Scope searches to
training files; a repo-wide grep over `tests/` reads held-out answer keys.

When a validation score moves, reason from the training reports: fix
the walker generally and re-run, or accept the move because the change
is a net win on training.

## Priority

`Score(3000)` (the CLI default budget) is the primary target. It is
also close to what the plugin injects every session: a 10,000-UTF-16-unit
cap shared with `--help`, median 2,730 tokens on 48 repos outside the
corpus (measured 2026-09-25). The other grid budgets are valid targets
too, and gains at early budgets carry through later rows. Complexity is
a cost: a small gain may not pay for the code it adds, and removing code
at flat score is a ship.

## 1. Find a candidate

Survey with `head -1 tests/divergence/*.md` and open many of the
lowest-scoring fixtures: the most valuable rules recur across several
weak fixtures. Narrow as the candidate sharpens; stop once you have one
good candidate.

Reading a report: walker rows that deliver NS atoms raise Score; a new
NS row usually drops it, since it grows `A_B` with content the walker
may not have delivered. Walker rows holding Score near 1.0 followed by
an NS row that drops it sharply mean the walker spent that budget on
content the NS ranks later.

`Score(B)` credits only NS rows within `B` of the NS's own cumulative
tokens, so pulling content earlier in the walker lifts it only if the NS
ranks that content within the budget. Don't raise a batch's `value` to
force it into a budget where it earns no credit; it displaces batches
that do.

The `predecessor` column names an NS row's dependency (a refinement, or
a prerequisite concept). The metric doesn't penalize delivering a
dependent before its predecessor, but that is still a valid target.

Things that look broken outrank score-tuning: 0-cost walker batches,
wrong descriptors.

For an NS row's content and justification, grep
`tests/north-stars/<fixture>.toml` for its `id`. To see what the walker
had rendered by a row, run
`cargo run --release -- tests/fixtures/<fixture> --token-budget <walker_cum>`.

## 2. Make a general change

In `src/walker/` (parsed languages live in `src/walker/code/`) or
`src/value.rs`. The rule must hold wherever
its condition holds in real codebases, and you must be able to state it
without naming fixtures; the evidence can still be one fixture. Prefer
accepting a divergence to adding a fixture-specific heuristic.

In a worktree, run `bash scripts/lane-setup.sh` from its root before the
first build.

## 3. Regenerate and read the whole diff

```bash
UPDATE_BASELINES=1 cargo t fixture_baselines
bash scripts/grid-means.sh
git diff tests/divergence/
```

Compare `grid-means.sh` against its output on the base commit, and read
the diff for every fixture, not only the targeted ones. A change is
good when the targeted rows' Score climbed, the rule generalizes, and
the gain pays for its code. A flat `Score(3000)` doesn't show a change
is safe elsewhere; check the grid.

When a change wins on its target but regresses elsewhere, tune its
magnitudes before reverting it. Reverting an earlier commit is also a
valid move: if Score stays flat without it, that's a ship.

After non-trivial changes, run `/code-review`.

## 4. Commit

One commit per coherent change, with a message about why the walker is
better in general, not which rows it helped. How many changes to attempt
is the user's call.

When stuck, get other perspectives: an Agent brainstorming the walker
design space, or a `model-router:` GPT agent (including
`model-router:adversarial-code-reviewer` on a shipped change).

## Don't

- Edit a North Star to match the walker.
- Ship neutral or marginal changes that add code.
