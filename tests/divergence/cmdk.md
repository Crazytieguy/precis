scores: Sim=0.303 Reached=21/42 Early=6 Late=11 Partial=5 Missing=16 Used=9484/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 8 | 5 | 0 | 3 | 0.62 |
| 2 | 3 | 3 | 0 | 0 | 0.96 |
| 3 | 1 | 1 | 0 | 0 | 1.00 |
| 4 | 5 | 1 | 1 | 3 | 0.31 |
| 5 | 6 | 3 | 2 | 1 | 0.65 |
| 6 | 2 | 0 | 0 | 2 | 0.00 |
| 7 | 3 | 1 | 0 | 2 | 0.29 |
| 8 | 3 | 0 | 0 | 3 | 0.00 |
| 9 | 4 | 3 | 0 | 1 | 0.64 |
| 10 | 7 | 4 | 2 | 1 | 0.71 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 78 | 222 | +144 | 1.00 | late | README lede — one-sentence elevator pitch | README headline in README.md (t=222, 1 atoms) |
| 1.2 | 140 | 62 | -78 | 1.00 | early | Top-level filesystem layout |  |
| 1.3 | 163 | 232 | +69 | 1.00 | late | Library inner layout — only two source files |  |
| 1.4 | 257 | 6636 | +6379 | 1.00 | late | Library runtime dependencies | package dependencies in cmdk/package.json (t=6636, 6 atoms) |
| 1.5 | 291 | — | — | 0.00 | missing | pnpm workspace members |  |
| 1.6 | 409 | — | — | 0.00 | missing | Test app layout — pages + test files |  |
| 1.7 | 549 | — | — | 0.00 | missing | Website (showcase) layout — themes + components |  |
| 1.8 | 751 | 1083 | +332 | 1.00 | late | Root scripts — how to build/run/test | package scripts in package.json (t=1083, 10 atoms) |
| 2.1 | 802 | 3019 | +2217 | 1.00 | late | README — top-level section headings | README.md section #2 (t=6447, 54 atoms) |
| 2.2 | 949 | 3019 | +2070 | 1.00 | late | README — every Parts heading (with [cmdk-*] selectors) | README.md section #8 (t=8937, 23 atoms) |
| 2.3 | 1166 | 6447 | +5281 | 0.87 | late | README — basic Use snippet | README.md section #2 (t=6447, 20 atoms) |
| 3.1 | 1232 | 7889 | +6657 | 1.00 | late | Architecture — Approach paragraph (DOM-as-truth) | ARCHITECTURE.md section #1 (t=7889, 1 atoms) |
| 4.1 | 1342 | — | — | 0.00 | missing | index.tsx — module imports + 'use client' |  |
| 4.2 | 1557 | — | — | 0.54 | partial | index.tsx — public exports surface | export names surface in cmdk/src/index.tsx (t=1452, 13 atoms) |
| 4.3 | 1718 | 1452 | -266 | 1.00 | aligned+over | index.tsx — every component declaration line | export body at cmdk/src/index.tsx:664 (t=5452, 49 atoms) |
| 4.4 | 1857 | — | — | 0.00 | missing | index.tsx — data-attribute selectors + SELECT_EVENT |  |
| 4.5 | 2215 | — | — | 0.00 | missing | index.tsx — internal type system (Context/State/Store) |  |
| 5.1 | 2516 | 8937 | +6421 | 0.81 | late | README — Item subsection (the most-asked-about part) | README.md section #8 (t=8937, 22 atoms) |
| 5.2 | 2755 | — | — | 0.17 | missing | README — Command (root) value/filter/keywords/loop prose | headings outline in README.md (t=3019, 2 atoms) |
| 5.3 | 3157 | 6447 | +3290 | 0.84 | late | README — Dialog usage with ⌘K keybind | README.md section #2 (t=6447, 32 atoms) |
| 5.4 | 3397 | — | — | 0.55 | partial | FAQ — accessibility, virtualization, RSC, etc. | README.md section #20 (t=5864, 6 atoms) |
| 5.5 | 3705 | — | — | 0.72 | partial | README — Group / Separator / Empty / Loading subsections | README.md section #9 (t=8018, 9 atoms) |
| 5.6 | 3856 | 8154 | +4298 | 0.80 | late | useCommandState — state slice subscription hook | README.md section #13 (t=8154, 7 atoms) |
| 6.1 | 4490 | — | — | 0.00 | missing | index.tsx — onKeyDown switch (next/prev/Home/End/Enter) |  |
| 6.2 | 5181 | — | — | 0.00 | missing | index.tsx — sort() body (DOM-as-truth, in code) |  |
| 7.1 | 5336 | 4816 | -520 | 0.88 | aligned | command-score — exported scoring function signature | export body at cmdk/src/command-score.ts:155 (t=4816, 6 atoms) |
| 7.2 | 5647 | — | — | 0.00 | missing | command-score — SCORE_* constants (positive weights) |  |
| 7.3 | 6081 | — | — | 0.00 | missing | command-score — PENALTY_* constants (decay weights) |  |
| 8.1 | 6245 | — | — | 0.00 | missing | Test file → describe-block names |  |
| 8.2 | 6516 | — | — | 0.00 | missing | Playwright config — test dir + dev-server hookup |  |
| 8.3 | 6992 | — | — | 0.00 | missing | test/pages/keybinds — fixture for every keybind spec |  |
| 9.1 | 7365 | 5452 | -1913 | 0.86 | aligned | index.tsx — Item header (registration + state subscriptions) | export body at cmdk/src/index.tsx:664 (t=5452, 23 atoms) |
| 9.2 | 7686 | 5452 | -2234 | 0.81 | aligned | index.tsx — Item render output (the cmdk-item div) | export body at cmdk/src/index.tsx:664 (t=5452, 26 atoms) |
| 9.3 | 8154 | 4223 | -3931 | 0.88 | early | index.tsx — Group component body | export body at cmdk/src/index.tsx:729 (t=4223, 34 atoms) |
| 9.4 | 8625 | — | — | 0.00 | missing | index.tsx — useCmdk + useValue helpers |  |
| 10.1 | 9027 | — | — | 0.60 | partial | Architecture — Discarded approaches (rejected alternatives) | ARCHITECTURE.md section #1 (t=7889, 5 atoms) |
| 10.2 | 9480 | — | — | 0.00 | missing | test/pages/group — fixture for the group.test specs |  |
| 10.3 | 9603 | 2596 | -7007 | 0.86 | early | Architecture — Performance + Groups bodies | headings outline in ARCHITECTURE.md (t=289, 4 atoms) |
| 10.4 | 9626 | 3053 | -6573 | 1.00 | early | README — Install snippet | README.md section #1 (t=3053, 3 atoms) |
| 10.5 | 9729 | 362 | -9367 | 0.91 | early | tsup build config | export at cmdk/tsup.config.ts:3 (t=362, 9 atoms) |
| 10.6 | 9892 | 2171 | -7721 | 0.83 | early | index.tsx — Separator body | export body at cmdk/src/index.tsx:774 (t=1608, 5 atoms) |
| 10.7 | 9996 | — | — | 0.78 | partial | index.tsx — Empty body | export body at cmdk/src/index.tsx:899 (t=1511, 3 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 8 | 1428 | README.md section #<n> |
| 2 | 663 | ARCHITECTURE.md section #<n> |
| 2 | 118 | export doc at cmdk/src/index.tsx:<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 467 | 1.00 | 467 | 4690 | export body at cmdk/src/index.tsx:833 |
| 400 | 1.00 | 400 | 3788 | export body at cmdk/src/index.tsx:787 |
| 386 | 1.00 | 386 | 7456 | ARCHITECTURE.md section #0 |
| 294 | 1.00 | 294 | 9484 | README.md section #5 |
| 277 | 1.00 | 277 | 6913 | ARCHITECTURE.md section #2 |
| 253 | 1.00 | 253 | 9190 | README.md section #18 |
| 242 | 1.00 | 242 | 2531 | json config tsconfig.json |
| 210 | 1.00 | 210 | 658 | package identity in cmdk/package.json |
| 205 | 1.00 | 205 | 8661 | README.md section #16 |
| 179 | 1.00 | 179 | 8456 | README.md section #7 |
| 179 | 1.00 | 179 | 2123 | export body at cmdk/src/index.tsx:909 |
| 178 | 1.00 | 178 | 1944 | export body at cmdk/src/index.tsx:882 |
| 160 | 0.43 | 375 | 5864 | README.md section #20 |
| 145 | 1.00 | 145 | 803 | package entrypoints in cmdk/package.json |
| 143 | 1.00 | 143 | 3366 | README.md section #21 |
| 133 | 1.00 | 133 | 2729 | package dependencies in package.json |
| 123 | 1.00 | 123 | 8277 | plaintext config .gitignore |
| 113 | 1.00 | 113 | 3223 | README.md section #22 |
| 95 | 0.50 | 189 | 6636 | package dependencies in cmdk/package.json |
| 81 | 1.00 | 81 | 7070 | README.md section #6 |
| 80 | 0.28 | 290 | 3019 | headings outline in README.md |
| 78 | 1.00 | 78 | 881 | package scripts in cmdk/package.json |
| 66 | 1.00 | 66 | 2289 | export doc at cmdk/src/index.tsx:664 |
| 52 | 1.00 | 52 | 2223 | export doc at cmdk/src/index.tsx:833 |
