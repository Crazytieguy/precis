---
name: add-fixture
description: End-to-end process for bringing a new test fixture into precis — from declaring the repo, through drafting and freezing its North Star, through generating snapshots and running alignment review. Use when adding a new fixture or when running the snapshot/review cycle on an already-declared fixture that isn't yet active.
---

# add-fixture

Adding a fixture has eight steps. Most involve spawning agents — do not
try to draft the North Star or alignment report yourself. The whole
point is that those are produced by dedicated agents whose prompts are
the load-bearing artifacts.

Pass only the fixture name (e.g. `thiserror`) through the flow. Paths
and agent spawn strings are derived from it.

**Defaults**: the flow runs end-to-end without stopping. If the user's
invocation explicitly asks for a human checkpoint on the North Star
(e.g. "let me review the north star first"), pause at step 4; otherwise
proceed through.

## 1. Declare and clone the repo

Check `tests/data/fixtures.rs` — the shared list of all fixture repos.

- **Already declared?** Skip to step 2.
- **Not declared?** Add a `("<name>", "<git-url>", "<commit-sha>")` line under the language section. The commit SHA pins the revision — pick a recent commit and copy its hash.

Then:

```bash
cargo run --bin clone_fixtures
```

Clones any missing fixtures to `tests/fixtures/<name>/` and strips
`.git`. Already-cloned fixtures are skipped.

## 2. Spawn three North Star drafts in parallel

Each draft is independent. Three independent drafts produce materially
different rankings that the combiner can merge — one draft alone is too
brittle (single agent's idiosyncratic choices dominate).

Spawn three `north-star-author` agents **in parallel** (single message,
three tool calls). Each gets:

- Fixture root: `tests/fixtures/<name>/`
- Output path: `tests/north-stars/drafts/<name>__{1,2,3}.md` (different suffix per draft so they don't collide)
- Count-tokens helper: `scripts/count-tokens.py`
- Fixture's revision pin: the SHA from `tests/data/fixtures.rs`

The agent walks the repo, ranks ~50+ content batches against an ~20k
token cap, and writes the document. Drafts take 5–10 minutes each; in
parallel they finish together.

## 3. Combine drafts

Spawn one `north-star-combiner` agent with just the fixture name. It
reads all three drafts from `tests/north-stars/drafts/<name>__*.md`,
synthesizes them into one coherent ranking, re-verifies token costs
via the helper, and writes `tests/north-stars/<name>.md`.

## 4. Freeze (optional human review)

The combined North Star is a *frozen* reference that implementation is
judged against. From this point, don't edit `tests/north-stars/<name>.md`
— future amendments require the defect protocol (see
`docs/design-notes.md`).

If the user asked to review before proceeding: stop here, show the file
path, and wait. Otherwise: continue. Keep the drafts in place in either
case — they document the combiner's provenance.

## 5. Register in snapshot + staleness tests

Edit `tests/snapshots.rs` — add the new name to the `FIXTURES` array.
The `BUDGETS` array (`1500`, `3000`, `6000`) stays as-is unless the
user asks otherwise.

The reviewer-staleness test (`tests/reviews_fresh.rs`) picks up
automatically: it walks `tests/snapshots/fixtures/` for every `.snap`
file and checks for a matching review.

## 6. Generate snapshots

```bash
INSTA_UPDATE=always cargo t
```

Produces `tests/snapshots/fixtures/<name>__{1500,3000,6000}.snap`. If
the budgets have a panic or mismatch, fix the underlying walker/value
issue before proceeding — snapshots shouldn't be accepted against a
buggy precis.

## 7. Run alignment review on every new snapshot

Spawn **one `alignment-reviewer` agent per budget, in parallel** (three
tool calls, one message). The spawn prompt is just the identifier
`<name>__<budget>`, e.g. `thiserror__3000`. The agent:

- Derives paths (snapshot, North Star, fixture, report output)
- Computes the snapshot's SHA-256 hash
- Writes the report to `tests/reviews/<name>__<budget>.md` with the
  hash in YAML frontmatter

Reviews flag three things: Ranking divergences, Batch correctness
violations, Honesty concerns. See `.claude/agents/alignment-reviewer.md`
for the exact format.

## 8. Iterate (or report back)

Read the three reports. If they flag major ranking / predecessor
violations, the walker / value weights need tuning.

**Pursue general solutions.** Every change should improve the walker
across the general distribution of real-world codebases — the fixtures
are samples of that distribution, not the target. Prefer a small
accepted divergence on one snapshot to a fix that over-fits:
fixture-specific heuristics, filename lists, or magic constants tuned
against one repo all accumulate into a walker that silently regresses
on unseen codebases. When a change visibly improves one fixture but is
suspicious on others, re-check every existing snapshot before
accepting.

After a walker change, regenerate snapshots (step 6) **and re-run
alignment review** (step 7). The staleness test will fail if the
snapshots change and the reviews don't.

**When stuck.** A divergence that resists general fixes is not a sign
to ask the user — it's a sign to consult more perspectives first.
Spawn an Agent for independent brainstorming on the walker design
tradeoff and consult codex for a second opinion (with follow-up rounds
if the first response is underspecified). Often a design tension that
looks binary has a third option one of them will surface.

**Report to the user as a last resort.** If after that the right path
is still unclear, reporting back is entirely acceptable — but by
default exhaust the above first.

Stop iterating when:

- No Tier 1 / major / predecessor violations remain across all
  fixtures, **or**
- The remaining divergences need ontology changes the user should
  approve, **or**
- Adding another fixture at this revision level would give better
  information than further signal tuning.

## Things to *not* do

- Don't hand-write North Stars. They're large and the agent
  produces better ones than a direct draft.
- Don't edit a frozen North Star without the defect protocol.
- Don't skip the combiner — three drafts without synthesis produces
  three different rankings, not consensus.
- Don't accept snapshots into insta before reviewing the output at
  least once manually. A bad snapshot locked in is worse than no
  snapshot.
- Don't spawn reviewers serially. They're independent; parallel is both
  faster and gives consistent hash-vs-content checks.
