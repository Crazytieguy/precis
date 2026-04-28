scores: Sim=0.323 Reached=24/42 Early=7 Late=12 Partial=3 Missing=15 Used=9955/10000

## Verdict

Verdict: budget-pressure bound
Likely primary lever: free final budget / demote late low-value spend
Evidence: 5 ranking-recoverable (w×gap=1.49), 8 wrong-slice/granularity (w×gap=0.29), 3 no-discovered (w×gap=0.08)
Secondary intervention: split wrong-slice batches for 8 rows
Loss reasons: 0 predecessor-gated, 5 too-expensive, 0 discovered-unscheduled
Top rows: 2.3, 3.1, 5.3, 6.1, 6.2
Note: likely lever is heuristic; verify `Sim` moves, not just bucket counts.

## Top opportunities

_`w(t)×gap` is a non-additive priority score: Σ exp(-exp_t/τ) × (1 - credit) per row, τ=2000. Same time weighting and credit gap as Sim, but rows can overlap between opportunities — sums across rows are an upper bound on Sim impact, not an additive estimate._

| intervention | rows | w(t)×gap | bands ≤3k/≤6k/total | evidence | top row ids |
|:-------------|-----:|---------:|:----------------------|:---------|:------------|
| free final budget / demote late waste | 5 | 1.49 | 2/5/5 | high-overlap candidates exceed final remaining budget, exact total=163/183 | 2.3, 3.1, 5.3, 6.1, 6.2 |
| split wrong-slice walker batches | 8 | 0.29 | 0/3/8 | nearby candidates have low exact atom overlap | 5.4, 7.2, 7.3, 5.5, 8.2, ... |
| add walker candidates for no-discovered rows | 3 | 0.08 | 0/0/3 | NS rows have no discovered line candidate | 8.1, 8.3, 10.2 |

Tiers: 1=6/8 reached, 0 partial, 2 missing, avg=0.75; 2=2/3 reached, 0 partial, 1 missing, avg=0.67; 3=0/1 reached, 0 partial, 1 missing, avg=0.00; 4=5/5 reached, 0 partial, 0 missing, avg=0.97; 5=3/6 reached, 2 partial, 1 missing, avg=0.62; 6=0/2 reached, 0 partial, 2 missing, avg=0.00; 7=1/3 reached, 0 partial, 2 missing, avg=0.29; 8=0/3 reached, 0 partial, 3 missing, avg=0.00; 9=3/4 reached, 0 partial, 1 missing, avg=0.72; 10=4/7 reached, 1 partial, 2 missing, avg=0.65

## Diagnosis rollup

| diagnosis | rows | missing | partial | timing | likely lever |
|:----------|-----:|--------:|--------:|-------:|:-------------|
| ranking-recoverable | 5 | 5 | 0 | 0 | value/ranking |
| wrong-slice / granularity | 8 | 5 | 3 | 0 | walker granularity / wrong slice |
| no discovered candidate | 3 | 3 | 0 | 0 | walker coverage or predecessor-gated emit |
| fs/listing | 2 | 2 | 0 | 0 | filesystem/listing value |
| timing-only | 23 | 0 | 0 | 23 | usually no code change |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | w(t)×gap | likely lever |
|:------------|-----:|---------:|:-------------|
| too expensive at final margin | 5 | 1.49 | free final budget |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=27, unscheduled bbox=5, scheduled same-file=3, fs-only=3, no discovered candidate=3

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | aligned | none | 1 |
| scheduled bbox | aligned | low | 1 |
| scheduled bbox | aligned | high | 2 |
| scheduled bbox | early | low | 3 |
| scheduled bbox | early | high | 2 |
| scheduled bbox | early | full | 1 |
| scheduled bbox | late | low | 5 |
| scheduled bbox | late | high | 1 |
| scheduled bbox | late | full | 6 |
| scheduled bbox | missing | low | 2 |
| scheduled bbox | partial | low | 3 |
| unscheduled bbox | missing | high | 4 |
| unscheduled bbox | missing | full | 1 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 2.3 | 1166 | — | — | 0.00 | missing | README — basic Use snippet | [unscheduled bbox exact=20/23] README.md section #2 (20 atoms, too expensive at final margin) |
| 3.1 | 1232 | — | — | 0.00 | missing | Architecture — Approach paragraph (DOM-as-truth) | [unscheduled bbox exact=1/1] ARCHITECTURE.md section #1 (1 atoms, too expensive at final margin) |
| 5.3 | 3157 | — | — | 0.00 | missing | README — Dialog usage with ⌘K keybind | [unscheduled bbox exact=32/38] README.md section #2 (32 atoms, too expensive at final margin) |
| 6.1 | 4490 | — | — | 0.00 | missing | index.tsx — onKeyDown switch (next/prev/Home/End/Enter) | [unscheduled bbox exact=58/61] export body at cmdk/src/index.tsx:169 body 170 (58 atoms, too expensive at final margin) |
| 6.2 | 5181 | — | — | 0.00 | missing | index.tsx — sort() body (DOM-as-truth, in code) | [unscheduled bbox exact=52/60] export body at cmdk/src/index.tsx:169 body 170 (52 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 5.4 | 3397 | — | — | 0.55 | partial | FAQ — accessibility, virtualization, RSC, etc. | [scheduled bbox exact=6/11] README.md section #35 (t=9879, 6 atoms) |
| 5.5 | 3705 | — | — | 0.72 | partial | README — Group / Separator / Empty / Loading subsections | [scheduled bbox exact=8/32] headings outline in README.md (t=3750, 8 atoms) |
| 7.2 | 5647 | — | — | 0.00 | missing | command-score — SCORE_* constants (positive weights) | [scheduled same-file] export body at cmdk/src/command-score.ts:155 body 156 (t=5685, 6 atoms) |
| 7.3 | 6081 | — | — | 0.00 | missing | command-score — PENALTY_* constants (decay weights) | [scheduled same-file] export body at cmdk/src/command-score.ts:155 body 156 (t=5685, 6 atoms) |
| 8.2 | 6516 | — | — | 0.00 | missing | Playwright config — test dir + dev-server hookup | [scheduled same-file] imports in playwright.config.ts (t=1176, 1 atoms) |
| 9.4 | 8625 | — | — | 0.32 | missing | index.tsx — useCmdk + useValue helpers | [scheduled bbox exact=7/41] module item at cmdk/src/index.tsx:1010 (t=1463, 7 atoms); better unscheduled exact=19/41: module item body at cmdk/src/index.tsx:1010 body 1019 (19 atoms, too expensive at final margin) |
| 10.1 | 9027 | — | — | 0.20 | missing | Architecture — Discarded approaches (rejected alternatives) | [scheduled bbox exact=2/10] headings outline in ARCHITECTURE.md (t=736, 2 atoms); better unscheduled exact=5/10: ARCHITECTURE.md section #1 (5 atoms, too expensive at final margin) |
| 10.7 | 9996 | — | — | 0.78 | partial | index.tsx — Empty body | [scheduled bbox exact=3/9] export doc at cmdk/src/index.tsx:899 (t=2239, 3 atoms) |

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
| 1.1 | 78 | 627 | +549 | 1.00 | late | README lede — one-sentence elevator pitch | [scheduled bbox exact=1/1] README headline in README.md (t=627, 1 atoms) |
| 1.2 | 140 | 62 | -78 | 1.00 | early | Top-level filesystem layout | fs-only |
| 1.4 | 257 | 8722 | +8465 | 1.00 | late | Library runtime dependencies | [scheduled bbox exact=6/6] package dependencies in cmdk/package.json (t=8722, 6 atoms) |
| 1.5 | 291 | 3076 | +2785 | 1.00 | late | pnpm workspace members | [scheduled bbox exact=4/4] plaintext config pnpm-workspace.yaml (t=3076, 4 atoms) |
| 1.8 | 751 | 4056 | +3305 | 1.00 | late | Root scripts — how to build/run/test | [scheduled bbox exact=10/10] package scripts in package.json (t=4056, 10 atoms) |
| 2.1 | 802 | 3750 | +2948 | 1.00 | late | README — top-level section headings | [scheduled bbox exact=7/7] headings outline in README.md (t=3750, 43 atoms) |
| 2.2 | 949 | 3750 | +2801 | 1.00 | late | README — every Parts heading (with [cmdk-*] selectors) | [scheduled bbox exact=10/10] headings outline in README.md (t=3750, 19 atoms) |
| 4.1 | 1342 | 3460 | +2118 | 0.88 | late | index.tsx — module imports + 'use client' | [scheduled bbox exact=7/8] imports in cmdk/src/index.tsx (t=3460, 7 atoms) |
| 4.2 | 1557 | 2030 | +473 | 0.96 | late | index.tsx — public exports surface | [scheduled bbox exact=10/24] module item at cmdk/src/index.tsx:930 (t=1637, 10 atoms) |
| 4.3 | 1718 | 2030 | +312 | 1.00 | aligned+over | index.tsx — every component declaration line | [scheduled bbox exact=0/9] export body at cmdk/src/index.tsx:664 body 665 (t=6321, 49 atoms) |
| 4.4 | 1857 | 507 | -1350 | 1.00 | early | index.tsx — data-attribute selectors + SELECT_EVENT | [scheduled bbox exact=1/8] module item at cmdk/src/index.tsx:161 (t=679, 1 atoms) |
| 4.5 | 2215 | 2976 | +761 | 1.00 | late | index.tsx — internal type system (Context/State/Store) | [scheduled bbox exact=14/30] module item at cmdk/src/index.tsx:123 (t=2976, 14 atoms) |
| 5.1 | 2516 | 9123 | +6607 | 0.81 | late | README — Item subsection (the most-asked-about part) | [scheduled bbox exact=9/27] README.md section #16 (t=9123, 9 atoms) |
| 5.2 | 2755 | 9504 | +6749 | 0.83 | late | README — Command (root) value/filter/keywords/loop prose | [scheduled bbox exact=1/12] README.md section #9 (t=9372, 14 atoms) |
| 5.6 | 3856 | 7398 | +3542 | 0.80 | late | useCommandState — state slice subscription hook | [scheduled bbox exact=5/10] README.md section #27 (t=7398, 5 atoms) |
| 7.1 | 5336 | 5685 | +349 | 0.88 | aligned | command-score — exported scoring function signature | [scheduled bbox exact=6/8] export body at cmdk/src/command-score.ts:155 body 156 (t=5685, 6 atoms) |
| 9.1 | 7365 | 6321 | -1044 | 0.86 | aligned | index.tsx — Item header (registration + state subscriptions) | [scheduled bbox exact=23/28] export body at cmdk/src/index.tsx:664 body 665 (t=6321, 23 atoms) |
| 9.2 | 7686 | 6321 | -1365 | 0.81 | aligned | index.tsx — Item render output (the cmdk-item div) | [scheduled bbox exact=26/32] export body at cmdk/src/index.tsx:664 body 665 (t=6321, 26 atoms) |
| 9.3 | 8154 | 5024 | -3130 | 0.88 | early | index.tsx — Group component body | [scheduled bbox exact=34/40] export body at cmdk/src/index.tsx:729 body 730 (t=5024, 34 atoms) |
| 10.3 | 9603 | 6453 | -3150 | 0.86 | early | Architecture — Performance + Groups bodies | [scheduled bbox exact=4/7] headings outline in ARCHITECTURE.md (t=736, 4 atoms) |
| 10.4 | 9626 | 3784 | -5842 | 1.00 | early | README — Install snippet | [scheduled bbox exact=3/3] README.md section #1 (t=3784, 3 atoms) |
| 10.5 | 9729 | 809 | -8920 | 0.91 | early | tsup build config | [scheduled bbox exact=9/11] export at cmdk/tsup.config.ts:3 (t=809, 9 atoms) |
| 10.6 | 9892 | 2749 | -7143 | 0.83 | early | index.tsx — Separator body | [scheduled bbox exact=5/12] export body at cmdk/src/index.tsx:774 body 775 (t=2186, 5 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 10 | 1077 | README.md section #<n> |
| 2 | 118 | export doc at cmdk/src/index.tsx:<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 467 | 1.00 | 467 | 5543 | export body at cmdk/src/index.tsx:833 body 834 |
| 400 | 1.00 | 400 | 4589 | export body at cmdk/src/index.tsx:787 body 788 |
| 386 | 1.00 | 386 | 7784 | ARCHITECTURE.md section #0 |
| 242 | 1.00 | 242 | 7194 | json config tsconfig.json |
| 210 | 1.00 | 210 | 8310 | package identity in cmdk/package.json |
| 179 | 1.00 | 179 | 2701 | export body at cmdk/src/index.tsx:909 body 910 |
| 178 | 1.00 | 178 | 2522 | export body at cmdk/src/index.tsx:882 body 883 |
| 160 | 0.43 | 375 | 9879 | README.md section #35 |
| 145 | 1.00 | 145 | 8455 | package entrypoints in cmdk/package.json |
| 144 | 1.00 | 144 | 3355 | module item body at cmdk/src/index.tsx:1046 body 1047 |
| 2082 | — | — | — | +23 more rows |
