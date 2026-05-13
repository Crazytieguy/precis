scores: Score(3000)=0.587 ns_rows≤3K=19/42 (reached=11 partial=0 missing=8)

## Per-budget scores

| B | A_B | I(B) | C(B) | compl(B) | Score(B) | walker_used |
|--:|----:|-----:|-----:|---------:|---------:|------------:|
| 1000 | 112 | 0.709 | 0.196 | 1.000 | 0.373 | 960 |
| 1442 | 144 | 0.701 | 0.153 | 1.000 | 0.327 | 1308 |
| 2080 | 185 | 0.731 | 0.357 | 1.000 | 0.510 | 2070 |
| 3000 | 254 | 0.731 | 0.472 | 0.919 | 0.587 | 2864 |
| 4327 | 345 | 0.724 | 0.387 | 0.785 | 0.529 | 3828 |
| 6240 | 521 | 0.821 | 0.393 | 0.917 | 0.568 | 6227 |
| 9000 | 743 | 0.836 | 0.455 | 0.921 | 0.617 | 8990 |

## Top opportunities

### Additive (close partial / missing rows)

| intervention | rows | gap@1k | gap@3k | gap@9k | evidence | top row ids |
|:-------------|-----:|-------:|-------:|-------:|:---------|:------------|
| split wrong-slice walker batches | 16 | 0.73 | 0.72 | 0.46 | nearby candidates have low exact atom overlap | 5.1, 5.5, 8.3, 7.3, 9.4, ... |
| tune ranking for high-overlap unscheduled candidates | 3 | 0.31 | 0.31 | 0.31 | high-overlap candidates not in the schedule by T_max, exact total=111/122 | 6.1, 6.2, 3.1 |

### Subtractive (suppress consistently off-NS batches)

| pattern | batches | freed@1k | freed@3k | freed@9k | evidence | top batch ids |
|:--------|--------:|---------:|---------:|---------:|:---------|:--------------|
| module item at cmdk/src/index.tsx:1081 | 1 | 0 | 135 | 135 | off_3k=135 | module item at cmdk/src/index.tsx:1081 |
| export doc at cmdk/src/index.tsx:<n> | 2 | 0 | 118 | 118 | off_3k=118 | export doc at cmdk/src/index.tsx:664, export doc at cmdk/src/index.tsx:833 |
| headings outline in README.md | 1 | 0 | 80 | 80 | off_3k=90 | headings outline in README.md |
| module item at cmdk/src/index.tsx:1071 | 1 | 65 | 65 | 65 | off_3k=65 | module item at cmdk/src/index.tsx:1071 |
| headings outline in ARCHITECTURE.md | 1 | 27 | 27 | 27 | off_3k=57 | headings outline in ARCHITECTURE.md |

Top missed paths (NS rows ≤ 3K): README.md (3 rows, 62 atoms), website (1 row, 30 atoms), test (1 row, 23 atoms), package.json (1 row, 10 atoms), cmdk/package.json (1 row, 6 atoms), ARCHITECTURE.md (1 row, 1 atom)

## Arrival ledger by diagnosis

### ranking-recoverable

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 3.1 | 1232 | 0.00 | missing | Architecture — Approach paragraph (DOM-as-truth) | [unscheduled bbox exact=1/1] ARCHITECTURE.md section #1 (1 atoms, too expensive at final margin) |
| 6.1 | 4490 | 0.00 | missing | index.tsx — onKeyDown switch (next/prev/Home/End/Enter) | [unscheduled bbox exact=58/61] export body at cmdk/src/index.tsx:169 body 170 (58 atoms, too expensive at final margin) |
| 6.2 | 5181 | 0.00 | missing | index.tsx — sort() body (DOM-as-truth, in code) | [unscheduled bbox exact=52/60] export body at cmdk/src/index.tsx:169 body 170 (52 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 5.1 | 2516 | 0.00 | missing | README — Item subsection (the most-asked-about part) | [scheduled bbox exact=9/27] README.md section #16 (t=7597, 9 atoms) |
| 5.2 | 2755 | 0.17 | missing | README — Command (root) value/filter/keywords/loop prose | [scheduled bbox exact=1/12] README.md section #9 (t=7894, 14 atoms) |
| 5.4 | 3397 | 0.00 | missing | FAQ — accessibility, virtualization, RSC, etc. | [scheduled bbox exact=6/11] README.md section #35 (t=8506, 6 atoms) |
| 5.5 | 3705 | 0.25 | missing | README — Group / Separator / Empty / Loading subsections | [scheduled bbox exact=8/32] headings outline in README.md (t=2760, 8 atoms) |
| 5.6 | 3856 | 0.20 | missing | useCommandState — state slice subscription hook | [scheduled bbox exact=5/10] README.md section #27 (t=5762, 5 atoms) |
| 7.1 | 5336 | 0.14 | missing | command-score — exported scoring function signature | [scheduled bbox exact=6/8] export body at cmdk/src/command-score.ts:155 body 156 (t=3828, 6 atoms) |
| 7.2 | 5647 | 0.00 | missing | command-score — SCORE_* constants (positive weights) | [scheduled same-file] export body at cmdk/src/command-score.ts:155 body 156 (t=3828, 6 atoms) |
| 7.3 | 6081 | 0.00 | missing | command-score — PENALTY_* constants (decay weights) | [scheduled same-file] export body at cmdk/src/command-score.ts:155 body 156 (t=3828, 6 atoms) |
| 8.1 | 6245 | 0.00 | missing | Test file → describe-block names | [unscheduled same-file] imports in test/props.test.ts (1 atoms, discovered unscheduled) |
| 8.2 | 6516 | 0.00 | missing | Playwright config — test dir + dev-server hookup | [scheduled same-file] imports in playwright.config.ts (t=895, 1 atoms) |
| 8.3 | 6992 | 0.00 | missing | test/pages/keybinds — fixture for every keybind spec | [scheduled bbox exact=1/47] export at test/pages/keybinds.tsx:47 (t=8107, 1 atoms); better unscheduled exact=3/47: imports in test/pages/keybinds.tsx (3 atoms, discovered unscheduled) |
| 9.4 | 8625 | 0.17 | missing | index.tsx — useCmdk + useValue helpers | [scheduled bbox exact=7/41] module item at cmdk/src/index.tsx:1010 (t=1120, 7 atoms); better unscheduled exact=19/41: module item body at cmdk/src/index.tsx:1010 body 1019 (19 atoms, discovered unscheduled) |
| 10.1 | 9027 | 0.20 | missing | Architecture — Discarded approaches (rejected alternatives) | [scheduled bbox exact=2/10] headings outline in ARCHITECTURE.md (t=736, 2 atoms); better unscheduled exact=5/10: ARCHITECTURE.md section #1 (5 atoms, too expensive at final margin) |
| 10.2 | 9480 | 0.00 | missing | test/pages/group — fixture for the group.test specs | [scheduled bbox exact=1/42] export at test/pages/group.tsx:42 (t=8075, 1 atoms); better unscheduled exact=2/42: imports in test/pages/group.tsx (2 atoms, discovered unscheduled) |
| 10.3 | 9603 | 0.71 | missing | Architecture — Performance + Groups bodies | [scheduled bbox exact=4/7] headings outline in ARCHITECTURE.md (t=736, 4 atoms) |
| 10.7 | 9996 | 0.45 | missing | index.tsx — Empty body | [scheduled bbox exact=3/9] export doc at cmdk/src/index.tsx:899 (t=2070, 3 atoms) |

### fs/listing

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 1.6 | 409 | 0.00 | missing | Test app layout — pages + test files | fs-only |
| 1.7 | 549 | 0.00 | missing | Website (showcase) layout — themes + components | fs-only |

### mixed/unknown

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 1.4 | 257 | 0.00 | missing | Library runtime dependencies | [scheduled bbox exact=6/6] package dependencies in cmdk/package.json (t=7184, 6 atoms) |
| 1.8 | 751 | 0.00 | missing | Root scripts — how to build/run/test | [scheduled bbox exact=10/10] package scripts in package.json (t=3066, 10 atoms) |
| 2.3 | 1166 | 0.00 | missing | README — basic Use snippet | [scheduled bbox exact=20/23] README.md section #2 (t=9573, 20 atoms) |
| 5.3 | 3157 | 0.00 | missing | README — Dialog usage with ⌘K keybind | [scheduled bbox exact=32/38] README.md section #2 (t=9573, 32 atoms) |
| 9.1 | 7365 | 0.04 | missing | index.tsx — Item header (registration + state subscriptions) | [scheduled bbox exact=23/28] export body at cmdk/src/index.tsx:664 body 665 (t=4464, 23 atoms) |
| 9.2 | 7686 | 0.00 | missing | index.tsx — Item render output (the cmdk-item div) | [scheduled bbox exact=26/32] export body at cmdk/src/index.tsx:664 body 665 (t=4464, 26 atoms) |
| 9.3 | 8154 | 0.03 | missing | index.tsx — Group component body | [scheduled bbox exact=34/40] export body at cmdk/src/index.tsx:729 body 730 (t=3634, 34 atoms) |

Top wasted paths (off-NS at 3K): cmdk/src/index.tsx (491t, 6 batches), README.md (90t, 1 batch), cmdk/tsup.config.ts (73t, 1 batch), ARCHITECTURE.md (57t, 1 batch)

## Walker waste rollup (by descriptor pattern)

| n | off_3k_total | pattern |
|--:|-------------:|:--------|
| 2 | 118 | export doc at cmdk/src/index.tsx:<n> |

## Walker waste (walker_t ≤ 3000, off_3k ≥ 50)

| off_3k | off_3k_ratio | off_any | cost | walker_t | batch |
|-------:|-------------:|--------:|-----:|---------:|:------|
| 135 | 1.00 | 135 | 135 | 1517 | module item at cmdk/src/index.tsx:1081 |
| 97 | 1.00 | 0 | 97 | 1920 | export body at cmdk/src/index.tsx:774 body 775 |
| 90 | 0.31 | 80 | 290 | 2470 | headings outline in README.md |
| 76 | 1.00 | 0 | 76 | 1044 | module item at cmdk/src/index.tsx:1010 |
| 73 | 1.00 | 0 | 73 | 736 | export at cmdk/tsup.config.ts:3 |
| 66 | 1.00 | 66 | 66 | 2275 | export doc at cmdk/src/index.tsx:664 |
| 65 | 1.00 | 65 | 65 | 895 | module item at cmdk/src/index.tsx:1071 |
| 57 | 1.00 | 27 | 57 | 679 | headings outline in ARCHITECTURE.md |
| 52 | 1.00 | 52 | 52 | 2223 | export doc at cmdk/src/index.tsx:833 |
