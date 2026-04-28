scores: Sim=0.307 Reached=19/42 Early=9 Late=8 Partial=3 Missing=20 Used=7336/10000

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 4 ranking-recoverable (w×gap=1.01), 14 wrong-slice/granularity (w×gap=2.11), 3 no-discovered (w×gap=0.08)
Secondary intervention: free final budget for 2 too-expensive candidates
Loss reasons: 0 predecessor-gated, 2 too-expensive, 2 discovered-unscheduled
Top rows: 4.1, 4.4, 4.5, 4.2, 5.2, ...
Note: likely lever is heuristic; verify `Sim` moves, not just bucket counts.

## Top opportunities

_`w(t)×gap` is a non-additive priority score: Σ exp(-exp_t/τ) × (1 - credit) per row, τ=2000. Same time weighting and credit gap as Sim, but rows can overlap between opportunities — sums across rows are an upper bound on Sim impact, not an additive estimate._

| intervention | rows | w(t)×gap | bands ≤3k/≤6k/total | evidence | top row ids |
|:-------------|-----:|---------:|:----------------------|:---------|:------------|
| split wrong-slice walker batches | 14 | 2.11 | 5/9/14 | nearby candidates have low exact atom overlap | 4.1, 4.4, 4.5, 4.2, 5.2, ... |
| tune ranking for discovered unscheduled candidates | 2 | 0.82 | 2/2/2 | high-overlap candidates fit but did not win, exact total=23/28 | 3.1, 5.1 |
| free final budget / demote late waste | 2 | 0.18 | 0/2/2 | high-overlap candidates exceed final remaining budget, exact total=110/121 | 6.1, 6.2 |
| add walker candidates for no-discovered rows | 3 | 0.08 | 0/0/3 | NS rows have no discovered line candidate | 8.1, 8.3, 10.2 |

Tiers: 1=6/8 reached, 0 partial, 2 missing, avg=0.75; 2=3/3 reached, 0 partial, 0 missing, avg=0.96; 3=0/1 reached, 0 partial, 1 missing, avg=0.00; 4=1/5 reached, 1 partial, 3 missing, avg=0.31; 5=1/6 reached, 1 partial, 4 missing, avg=0.37; 6=0/2 reached, 0 partial, 2 missing, avg=0.00; 7=1/3 reached, 0 partial, 2 missing, avg=0.29; 8=0/3 reached, 0 partial, 3 missing, avg=0.00; 9=3/4 reached, 0 partial, 1 missing, avg=0.64; 10=4/7 reached, 1 partial, 2 missing, avg=0.65

## Diagnosis rollup

| diagnosis | rows | missing | partial | timing | likely lever |
|:----------|-----:|--------:|--------:|-------:|:-------------|
| ranking-recoverable | 4 | 4 | 0 | 0 | value/ranking |
| wrong-slice / granularity | 14 | 11 | 3 | 0 | walker granularity / wrong slice |
| no discovered candidate | 3 | 3 | 0 | 0 | walker coverage or predecessor-gated emit |
| fs/listing | 2 | 2 | 0 | 0 | filesystem/listing value |
| timing-only | 18 | 0 | 0 | 18 | usually no code change |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | w(t)×gap | likely lever |
|:------------|-----:|---------:|:-------------|
| too expensive at final margin | 2 | 0.18 | free final budget |
| discovered unscheduled | 2 | 0.82 | tune ranking |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=24, unscheduled bbox=4, scheduled same-file=7, fs-only=3, no discovered candidate=3

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | aligned | low | 1 |
| scheduled bbox | early | none | 1 |
| scheduled bbox | early | low | 2 |
| scheduled bbox | early | high | 4 |
| scheduled bbox | early | full | 1 |
| scheduled bbox | late | low | 1 |
| scheduled bbox | late | high | 2 |
| scheduled bbox | late | full | 5 |
| scheduled bbox | missing | low | 4 |
| scheduled bbox | partial | low | 3 |
| unscheduled bbox | missing | high | 3 |
| unscheduled bbox | missing | full | 1 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 3.1 | 1232 | — | — | 0.00 | missing | Architecture — Approach paragraph (DOM-as-truth) | [unscheduled bbox exact=1/1] ARCHITECTURE.md section #1 (1 atoms, discovered unscheduled) |
| 5.1 | 2516 | — | — | 0.00 | missing | README — Item subsection (the most-asked-about part) | [unscheduled bbox exact=22/27] README.md section #8 (22 atoms, discovered unscheduled) |
| 6.1 | 4490 | — | — | 0.00 | missing | index.tsx — onKeyDown switch (next/prev/Home/End/Enter) | [unscheduled bbox exact=58/61] export body at cmdk/src/index.tsx:169 (58 atoms, too expensive at final margin) |
| 6.2 | 5181 | — | — | 0.00 | missing | index.tsx — sort() body (DOM-as-truth, in code) | [unscheduled bbox exact=52/60] export body at cmdk/src/index.tsx:169 (52 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 4.1 | 1342 | — | — | 0.00 | missing | index.tsx — module imports + 'use client' | [scheduled same-file] export body at cmdk/src/index.tsx:664 (t=4468, 49 atoms) |
| 4.2 | 1557 | — | — | 0.54 | partial | index.tsx — public exports surface | [scheduled bbox exact=9/24] export names surface #1 in cmdk/src/index.tsx (t=506, 9 atoms) |
| 4.4 | 1857 | — | — | 0.00 | missing | index.tsx — data-attribute selectors + SELECT_EVENT | [scheduled same-file] export body at cmdk/src/index.tsx:664 (t=4468, 49 atoms) |
| 4.5 | 2215 | — | — | 0.00 | missing | index.tsx — internal type system (Context/State/Store) | [scheduled same-file] export body at cmdk/src/index.tsx:664 (t=4468, 49 atoms) |
| 5.2 | 2755 | — | — | 0.17 | missing | README — Command (root) value/filter/keywords/loop prose | [scheduled bbox exact=2/12] headings outline in README.md (t=1949, 2 atoms); better unscheduled exact=9/12: README.md section #4 (51 atoms, discovered unscheduled) |
| 5.4 | 3397 | — | — | 0.55 | partial | FAQ — accessibility, virtualization, RSC, etc. | [scheduled bbox exact=6/11] README.md section #20 (t=6596, 6 atoms) |
| 5.5 | 3705 | — | — | 0.47 | missing | README — Group / Separator / Empty / Loading subsections | [scheduled bbox exact=8/32] headings outline in README.md (t=1949, 8 atoms); better unscheduled exact=9/32: README.md section #9 (9 atoms, discovered unscheduled) |
| 5.6 | 3856 | — | — | 0.20 | missing | useCommandState — state slice subscription hook | [scheduled bbox exact=2/10] headings outline in README.md (t=1949, 2 atoms); better unscheduled exact=7/10: README.md section #13 (7 atoms, discovered unscheduled) |
| 7.2 | 5647 | — | — | 0.00 | missing | command-score — SCORE_* constants (positive weights) | [scheduled same-file] export body at cmdk/src/command-score.ts:155 (t=3832, 6 atoms) |
| 7.3 | 6081 | — | — | 0.00 | missing | command-score — PENALTY_* constants (decay weights) | [scheduled same-file] export body at cmdk/src/command-score.ts:155 (t=3832, 6 atoms) |
| 8.2 | 6516 | — | — | 0.00 | missing | Playwright config — test dir + dev-server hookup | [scheduled same-file] imports in playwright.config.ts (t=405, 1 atoms) |
| 9.4 | 8625 | — | — | 0.00 | missing | index.tsx — useCmdk + useValue helpers | [scheduled same-file] export body at cmdk/src/index.tsx:664 (t=4468, 49 atoms) |
| 10.1 | 9027 | — | — | 0.20 | missing | Architecture — Discarded approaches (rejected alternatives) | [scheduled bbox exact=2/10] headings outline in ARCHITECTURE.md (t=289, 2 atoms); better unscheduled exact=5/10: ARCHITECTURE.md section #1 (5 atoms, discovered unscheduled) |
| 10.7 | 9996 | — | — | 0.78 | partial | index.tsx — Empty body | [scheduled bbox exact=3/9] export doc at cmdk/src/index.tsx:899 (t=997, 3 atoms) |

### no discovered candidate

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 8.1 | 6245 | — | — | 0.00 | missing | Test file → describe-block names | no discovered line candidate |
| 8.3 | 6992 | — | — | 0.00 | missing | test/pages/keybinds — fixture for every keybind spec | no discovered line candidate |
| 10.2 | 9480 | — | — | 0.00 | missing | test/pages/group — fixture for the group.test specs | no discovered line candidate |

### fs/listing

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.6 | 409 | — | — | 0.00 | missing | Test app layout — pages + test files | fs-only |
| 1.7 | 549 | — | — | 0.00 | missing | Website (showcase) layout — themes + components | fs-only |

### timing-only

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 78 | 232 | +154 | 1.00 | late | README lede — one-sentence elevator pitch | [scheduled bbox exact=1/1] README headline in README.md (t=232, 1 atoms) |
| 1.2 | 140 | 62 | -78 | 1.00 | early | Top-level filesystem layout | fs-only |
| 1.4 | 257 | 6221 | +5964 | 1.00 | late | Library runtime dependencies | [scheduled bbox exact=6/6] package dependencies in cmdk/package.json (t=6221, 6 atoms) |
| 1.5 | 291 | 1659 | +1368 | 1.00 | late | pnpm workspace members | [scheduled bbox exact=4/4] plaintext config pnpm-workspace.yaml (t=1659, 4 atoms) |
| 1.8 | 751 | 2255 | +1504 | 1.00 | late | Root scripts — how to build/run/test | [scheduled bbox exact=10/10] package scripts in package.json (t=2255, 10 atoms) |
| 2.1 | 802 | 1949 | +1147 | 1.00 | late | README — top-level section headings | [scheduled bbox exact=1/7] README.md section #2 (t=7336, 54 atoms) |
| 2.2 | 949 | 1949 | +1000 | 1.00 | late | README — every Parts heading (with [cmdk-*] selectors) | [scheduled bbox exact=10/10] headings outline in README.md (t=1949, 19 atoms) |
| 2.3 | 1166 | 7336 | +6170 | 0.87 | late | README — basic Use snippet | [scheduled bbox exact=20/23] README.md section #2 (t=7336, 20 atoms) |
| 4.3 | 1718 | 788 | -930 | 1.00 | early | index.tsx — every component declaration line | [scheduled bbox exact=0/9] export body at cmdk/src/index.tsx:664 (t=4468, 49 atoms) |
| 5.3 | 3157 | 7336 | +4179 | 0.84 | late | README — Dialog usage with ⌘K keybind | [scheduled bbox exact=32/38] README.md section #2 (t=7336, 32 atoms) |
| 7.1 | 5336 | 3832 | -1504 | 0.88 | aligned | command-score — exported scoring function signature | [scheduled bbox exact=6/8] export body at cmdk/src/command-score.ts:155 (t=3832, 6 atoms) |
| 9.1 | 7365 | 4468 | -2897 | 0.86 | early | index.tsx — Item header (registration + state subscriptions) | [scheduled bbox exact=23/28] export body at cmdk/src/index.tsx:664 (t=4468, 23 atoms) |
| 9.2 | 7686 | 4468 | -3218 | 0.81 | early | index.tsx — Item render output (the cmdk-item div) | [scheduled bbox exact=26/32] export body at cmdk/src/index.tsx:664 (t=4468, 26 atoms) |
| 9.3 | 8154 | 3223 | -4931 | 0.88 | early | index.tsx — Group component body | [scheduled bbox exact=34/40] export body at cmdk/src/index.tsx:729 (t=3223, 34 atoms) |
| 10.3 | 9603 | 4555 | -5048 | 0.86 | early | Architecture — Performance + Groups bodies | [scheduled bbox exact=4/7] headings outline in ARCHITECTURE.md (t=289, 4 atoms) |
| 10.4 | 9626 | 1983 | -7643 | 1.00 | early | README — Install snippet | [scheduled bbox exact=3/3] README.md section #1 (t=1983, 3 atoms) |
| 10.5 | 9729 | 362 | -9367 | 0.91 | early | tsup build config | [scheduled bbox exact=9/11] export at cmdk/tsup.config.ts:3 (t=362, 9 atoms) |
| 10.6 | 9892 | 1507 | -8385 | 0.83 | early | index.tsx — Separator body | [scheduled bbox exact=5/12] export body at cmdk/src/index.tsx:774 (t=944, 5 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 4 | 497 | README.md section #<n> |
| 2 | 118 | export doc at cmdk/src/index.tsx:<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 467 | 1.00 | 467 | 3690 | export body at cmdk/src/index.tsx:833 |
| 400 | 1.00 | 400 | 2788 | export body at cmdk/src/index.tsx:787 |
| 386 | 1.00 | 386 | 5439 | ARCHITECTURE.md section #0 |
| 242 | 1.00 | 242 | 5053 | json config tsconfig.json |
| 210 | 1.00 | 210 | 5809 | package identity in cmdk/package.json |
| 179 | 1.00 | 179 | 1459 | export body at cmdk/src/index.tsx:909 |
| 178 | 1.00 | 178 | 1280 | export body at cmdk/src/index.tsx:882 |
| 160 | 0.43 | 375 | 6596 | README.md section #20 |
| 145 | 1.00 | 145 | 5954 | package entrypoints in cmdk/package.json |
| 143 | 1.00 | 143 | 4811 | README.md section #21 |
| 821 | — | — | — | +9 more rows |
