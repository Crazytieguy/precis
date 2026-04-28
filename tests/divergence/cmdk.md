scores: Sim=0.355 Reached=26/42 Early=9 Late=12 Partial=2 Missing=14 Used=9021/10000

## Verdict

Verdict: ranking-race bound
Likely primary lever: raise high-overlap discovered candidates over competing batches
Evidence: 3 ranking-recoverable (w×gap=0.72), 8 wrong-slice/granularity (w×gap=0.30), 3 no-discovered (w×gap=0.08)
Secondary intervention: free final budget for 2 too-expensive candidates
Loss reasons: 0 predecessor-gated, 2 too-expensive, 1 discovered-unscheduled
Top rows: 3.1
Note: likely lever is heuristic; verify `Sim` moves, not just bucket counts.

## Top opportunities

_`w(t)×gap` is a non-additive priority score: Σ exp(-exp_t/τ) × (1 - credit) per row, τ=2000. Same time weighting and credit gap as Sim, but rows can overlap between opportunities — sums across rows are an upper bound on Sim impact, not an additive estimate._

| intervention | rows | w(t)×gap | bands ≤3k/≤6k/total | evidence | top row ids |
|:-------------|-----:|---------:|:----------------------|:---------|:------------|
| tune ranking for discovered unscheduled candidates | 1 | 0.54 | 1/1/1 | high-overlap candidates fit but did not win, exact total=1/1 | 3.1 |
| split wrong-slice walker batches | 8 | 0.30 | 0/3/8 | nearby candidates have low exact atom overlap | 5.4, 7.2, 7.3, 5.5, 8.2, ... |
| free final budget / demote late waste | 2 | 0.18 | 0/2/2 | high-overlap candidates exceed final remaining budget, exact total=110/121 | 6.1, 6.2 |
| add walker candidates for no-discovered rows | 3 | 0.08 | 0/0/3 | NS rows have no discovered line candidate | 8.1, 8.3, 10.2 |

Tiers: 1=6/8 reached, 0 partial, 2 missing, avg=0.75; 2=3/3 reached, 0 partial, 0 missing, avg=0.96; 3=0/1 reached, 0 partial, 1 missing, avg=0.00; 4=5/5 reached, 0 partial, 0 missing, avg=0.97; 5=4/6 reached, 2 partial, 0 missing, avg=0.76; 6=0/2 reached, 0 partial, 2 missing, avg=0.00; 7=1/3 reached, 0 partial, 2 missing, avg=0.29; 8=0/3 reached, 0 partial, 3 missing, avg=0.00; 9=3/4 reached, 0 partial, 1 missing, avg=0.68; 10=4/7 reached, 0 partial, 3 missing, avg=0.61

## Diagnosis rollup

| diagnosis | rows | missing | partial | timing | likely lever |
|:----------|-----:|--------:|--------:|-------:|:-------------|
| ranking-recoverable | 3 | 3 | 0 | 0 | value/ranking |
| wrong-slice / granularity | 8 | 6 | 2 | 0 | walker granularity / wrong slice |
| no discovered candidate | 3 | 3 | 0 | 0 | walker coverage or predecessor-gated emit |
| fs/listing | 2 | 2 | 0 | 0 | filesystem/listing value |
| timing-only | 24 | 0 | 0 | 24 | usually no code change |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | w(t)×gap | likely lever |
|:------------|-----:|---------:|:-------------|
| too expensive at final margin | 2 | 0.18 | free final budget |
| discovered unscheduled | 1 | 0.54 | tune ranking |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=28, unscheduled bbox=3, scheduled same-file=3, fs-only=3, no discovered candidate=3

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | aligned | none | 1 |
| scheduled bbox | aligned | low | 2 |
| scheduled bbox | early | low | 3 |
| scheduled bbox | early | high | 4 |
| scheduled bbox | early | full | 1 |
| scheduled bbox | late | low | 4 |
| scheduled bbox | late | high | 3 |
| scheduled bbox | late | full | 5 |
| scheduled bbox | missing | low | 3 |
| scheduled bbox | partial | low | 2 |
| unscheduled bbox | missing | high | 2 |
| unscheduled bbox | missing | full | 1 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 3.1 | 1232 | — | — | 0.00 | missing | Architecture — Approach paragraph (DOM-as-truth) | [unscheduled bbox exact=1/1] ARCHITECTURE.md section #1 (1 atoms, discovered unscheduled) |
| 6.1 | 4490 | — | — | 0.00 | missing | index.tsx — onKeyDown switch (next/prev/Home/End/Enter) | [unscheduled bbox exact=58/61] export body at cmdk/src/index.tsx:169 body 170 (58 atoms, too expensive at final margin) |
| 6.2 | 5181 | — | — | 0.00 | missing | index.tsx — sort() body (DOM-as-truth, in code) | [unscheduled bbox exact=52/60] export body at cmdk/src/index.tsx:169 body 170 (52 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 5.4 | 3397 | — | — | 0.55 | partial | FAQ — accessibility, virtualization, RSC, etc. | [scheduled bbox exact=6/11] README.md section #35 (t=7987, 6 atoms) |
| 5.5 | 3705 | — | — | 0.72 | partial | README — Group / Separator / Empty / Loading subsections | [scheduled bbox exact=8/32] headings outline in README.md (t=2760, 8 atoms) |
| 7.2 | 5647 | — | — | 0.00 | missing | command-score — SCORE_* constants (positive weights) | [scheduled same-file] export body at cmdk/src/command-score.ts:155 body 156 (t=3828, 6 atoms) |
| 7.3 | 6081 | — | — | 0.00 | missing | command-score — PENALTY_* constants (decay weights) | [scheduled same-file] export body at cmdk/src/command-score.ts:155 body 156 (t=3828, 6 atoms) |
| 8.2 | 6516 | — | — | 0.00 | missing | Playwright config — test dir + dev-server hookup | [scheduled same-file] imports in playwright.config.ts (t=895, 1 atoms) |
| 9.4 | 8625 | — | — | 0.17 | missing | index.tsx — useCmdk + useValue helpers | [scheduled bbox exact=7/41] module item at cmdk/src/index.tsx:1010 (t=1120, 7 atoms); better unscheduled exact=19/41: module item body at cmdk/src/index.tsx:1010 body 1019 (19 atoms, discovered unscheduled) |
| 10.1 | 9027 | — | — | 0.20 | missing | Architecture — Discarded approaches (rejected alternatives) | [scheduled bbox exact=2/10] headings outline in ARCHITECTURE.md (t=736, 2 atoms); better unscheduled exact=5/10: ARCHITECTURE.md section #1 (5 atoms, discovered unscheduled) |
| 10.7 | 9996 | — | — | 0.45 | missing | index.tsx — Empty body | [scheduled bbox exact=3/9] export doc at cmdk/src/index.tsx:899 (t=1726, 3 atoms) |

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
| 1.4 | 257 | 6830 | +6573 | 1.00 | late | Library runtime dependencies | [scheduled bbox exact=6/6] package dependencies in cmdk/package.json (t=6830, 6 atoms) |
| 1.5 | 291 | 2206 | +1915 | 1.00 | late | pnpm workspace members | [scheduled bbox exact=4/4] plaintext config pnpm-workspace.yaml (t=2206, 4 atoms) |
| 1.8 | 751 | 3066 | +2315 | 1.00 | late | Root scripts — how to build/run/test | [scheduled bbox exact=10/10] package scripts in package.json (t=3066, 10 atoms) |
| 2.1 | 802 | 2760 | +1958 | 1.00 | late | README — top-level section headings | [scheduled bbox exact=1/7] README.md section #2 (t=9021, 54 atoms) |
| 2.2 | 949 | 2760 | +1811 | 1.00 | late | README — every Parts heading (with [cmdk-*] selectors) | [scheduled bbox exact=10/10] headings outline in README.md (t=2760, 19 atoms) |
| 2.3 | 1166 | 9021 | +7855 | 0.87 | late | README — basic Use snippet | [scheduled bbox exact=20/23] README.md section #2 (t=9021, 20 atoms) |
| 4.1 | 1342 | 2446 | +1104 | 0.88 | late | index.tsx — module imports + 'use client' | [scheduled bbox exact=7/8] imports in cmdk/src/index.tsx (t=2446, 7 atoms) |
| 4.2 | 1557 | 1576 | +19 | 0.96 | aligned | index.tsx — public exports surface | [scheduled bbox exact=10/24] module item at cmdk/src/index.tsx:930 (t=1294, 10 atoms) |
| 4.3 | 1718 | 1576 | -142 | 1.00 | aligned+over | index.tsx — every component declaration line | [scheduled bbox exact=0/9] export body at cmdk/src/index.tsx:664 body 665 (t=4464, 49 atoms) |
| 4.4 | 1857 | 507 | -1350 | 1.00 | early | index.tsx — data-attribute selectors + SELECT_EVENT | [scheduled bbox exact=1/8] module item at cmdk/src/index.tsx:161 (t=679, 1 atoms) |
| 5.1 | 2516 | 7231 | +4715 | 0.81 | late | README — Item subsection (the most-asked-about part) | [scheduled bbox exact=9/27] README.md section #16 (t=7231, 9 atoms) |
| 5.2 | 2755 | 7612 | +4857 | 0.83 | late | README — Command (root) value/filter/keywords/loop prose | [scheduled bbox exact=1/12] README.md section #9 (t=7480, 14 atoms) |
| 5.3 | 3157 | 9021 | +5864 | 0.84 | late | README — Dialog usage with ⌘K keybind | [scheduled bbox exact=32/38] README.md section #2 (t=9021, 32 atoms) |
| 5.6 | 3856 | 5506 | +1650 | 0.80 | late | useCommandState — state slice subscription hook | [scheduled bbox exact=5/10] README.md section #27 (t=5506, 5 atoms) |
| 7.1 | 5336 | 3828 | -1508 | 0.88 | aligned | command-score — exported scoring function signature | [scheduled bbox exact=6/8] export body at cmdk/src/command-score.ts:155 body 156 (t=3828, 6 atoms) |
| 9.1 | 7365 | 4464 | -2901 | 0.86 | early | index.tsx — Item header (registration + state subscriptions) | [scheduled bbox exact=23/28] export body at cmdk/src/index.tsx:664 body 665 (t=4464, 23 atoms) |
| 9.2 | 7686 | 4464 | -3222 | 0.81 | early | index.tsx — Item render output (the cmdk-item div) | [scheduled bbox exact=26/32] export body at cmdk/src/index.tsx:664 body 665 (t=4464, 26 atoms) |
| 9.3 | 8154 | 3634 | -4520 | 0.88 | early | index.tsx — Group component body | [scheduled bbox exact=34/40] export body at cmdk/src/index.tsx:729 body 730 (t=3634, 34 atoms) |
| 10.3 | 9603 | 4586 | -5017 | 0.86 | early | Architecture — Performance + Groups bodies | [scheduled bbox exact=4/7] headings outline in ARCHITECTURE.md (t=736, 4 atoms) |
| 10.4 | 9626 | 2794 | -6832 | 1.00 | early | README — Install snippet | [scheduled bbox exact=3/3] README.md section #1 (t=2794, 3 atoms) |
| 10.5 | 9729 | 809 | -8920 | 0.91 | early | tsup build config | [scheduled bbox exact=9/11] export at cmdk/tsup.config.ts:3 (t=809, 9 atoms) |
| 10.6 | 9892 | 1879 | -8013 | 0.83 | early | index.tsx — Separator body | [scheduled bbox exact=5/12] export body at cmdk/src/index.tsx:774 body 775 (t=1673, 5 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 13 | 1452 | README.md section #<n> |
| 2 | 118 | export doc at cmdk/src/index.tsx:<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 386 | 1.00 | 386 | 5892 | ARCHITECTURE.md section #0 |
| 242 | 1.00 | 242 | 5302 | json config tsconfig.json |
| 210 | 1.00 | 210 | 6418 | package identity in cmdk/package.json |
| 160 | 0.43 | 375 | 7987 | README.md section #35 |
| 148 | 1.00 | 148 | 8357 | README.md section #11 |
| 146 | 1.00 | 146 | 8209 | README.md section #12 |
| 145 | 1.00 | 145 | 6563 | package entrypoints in cmdk/package.json |
| 143 | 1.00 | 143 | 5002 | README.md section #36 |
| 135 | 1.00 | 135 | 2341 | module item at cmdk/src/index.tsx:1081 |
| 133 | 1.00 | 133 | 3199 | package dependencies in package.json |
| 1414 | — | — | — | +16 more rows |
