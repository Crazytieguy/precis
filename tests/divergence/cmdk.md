scores: Sim=0.267 Reached=20/42 Early=9 Late=10 Partial=5 Missing=17 Used=9026/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 8 | 4 | 0 | 4 | 0.50 |
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
| 1.1 | 78 | 181 | +103 | 1.00 | late | README lede — one-sentence elevator pitch | README headline in README.md (t=181, 1 atoms) |
| 1.2 | 140 | 62 | -78 | 1.00 | early | Top-level filesystem layout |  |
| 1.3 | 163 | 795 | +632 | 1.00 | late | Library inner layout — only two source files |  |
| 1.4 | 257 | 6301 | +6044 | 1.00 | late | Library runtime dependencies | package dependencies in cmdk/package.json (t=6301, 6 atoms) |
| 1.5 | 291 | — | — | 0.00 | missing | pnpm workspace members |  |
| 1.6 | 409 | — | — | 0.00 | missing | Test app layout — pages + test files |  |
| 1.7 | 549 | — | — | 0.00 | missing | Website (showcase) layout — themes + components |  |
| 1.8 | 751 | — | — | 0.00 | missing | Root scripts — how to build/run/test |  |
| 2.1 | 802 | 2576 | +1774 | 1.00 | late | README — top-level section headings | README.md section #2 (t=6112, 54 atoms) |
| 2.2 | 949 | 2576 | +1627 | 1.00 | late | README — every Parts heading (with [cmdk-*] selectors) | README.md section #8 (t=8479, 23 atoms) |
| 2.3 | 1166 | 6112 | +4946 | 0.87 | late | README — basic Use snippet | README.md section #2 (t=6112, 20 atoms) |
| 3.1 | 1232 | 7554 | +6322 | 1.00 | late | Architecture — Approach paragraph (DOM-as-truth) | ARCHITECTURE.md section #1 (t=7554, 1 atoms) |
| 4.1 | 1342 | — | — | 0.00 | missing | index.tsx — module imports + 'use client' |  |
| 4.2 | 1557 | — | — | 0.54 | partial | index.tsx — public exports surface | export names surface in cmdk/src/index.tsx (t=1188, 13 atoms) |
| 4.3 | 1718 | 1188 | -530 | 1.00 | early | index.tsx — every component declaration line | export body at cmdk/src/index.tsx:664 (t=5103, 49 atoms) |
| 4.4 | 1857 | — | — | 0.00 | missing | index.tsx — data-attribute selectors + SELECT_EVENT |  |
| 4.5 | 2215 | — | — | 0.00 | missing | index.tsx — internal type system (Context/State/Store) |  |
| 5.1 | 2516 | 8479 | +5963 | 0.81 | late | README — Item subsection (the most-asked-about part) | README.md section #8 (t=8479, 22 atoms) |
| 5.2 | 2755 | — | — | 0.17 | missing | README — Command (root) value/filter/keywords/loop prose | headings outline in README.md (t=2576, 2 atoms) |
| 5.3 | 3157 | 6112 | +2955 | 0.84 | late | README — Dialog usage with ⌘K keybind | README.md section #2 (t=6112, 32 atoms) |
| 5.4 | 3397 | — | — | 0.55 | partial | FAQ — accessibility, virtualization, RSC, etc. | README.md section #20 (t=5529, 6 atoms) |
| 5.5 | 3705 | — | — | 0.72 | partial | README — Group / Separator / Empty / Loading subsections | README.md section #9 (t=7683, 9 atoms) |
| 5.6 | 3856 | 7819 | +3963 | 0.80 | late | useCommandState — state slice subscription hook | README.md section #13 (t=7819, 7 atoms) |
| 6.1 | 4490 | — | — | 0.00 | missing | index.tsx — onKeyDown switch (next/prev/Home/End/Enter) |  |
| 6.2 | 5181 | — | — | 0.00 | missing | index.tsx — sort() body (DOM-as-truth, in code) |  |
| 7.1 | 5336 | 4467 | -869 | 0.88 | aligned | command-score — exported scoring function signature | export body at cmdk/src/command-score.ts:155 (t=4467, 6 atoms) |
| 7.2 | 5647 | — | — | 0.00 | missing | command-score — SCORE_* constants (positive weights) |  |
| 7.3 | 6081 | — | — | 0.00 | missing | command-score — PENALTY_* constants (decay weights) |  |
| 8.1 | 6245 | — | — | 0.00 | missing | Test file → describe-block names |  |
| 8.2 | 6516 | — | — | 0.00 | missing | Playwright config — test dir + dev-server hookup |  |
| 8.3 | 6992 | — | — | 0.00 | missing | test/pages/keybinds — fixture for every keybind spec |  |
| 9.1 | 7365 | 5103 | -2262 | 0.86 | early | index.tsx — Item header (registration + state subscriptions) | export body at cmdk/src/index.tsx:664 (t=5103, 23 atoms) |
| 9.2 | 7686 | 5103 | -2583 | 0.81 | early | index.tsx — Item render output (the cmdk-item div) | export body at cmdk/src/index.tsx:664 (t=5103, 26 atoms) |
| 9.3 | 8154 | 3874 | -4280 | 0.88 | early | index.tsx — Group component body | export body at cmdk/src/index.tsx:729 (t=3874, 34 atoms) |
| 9.4 | 8625 | — | — | 0.00 | missing | index.tsx — useCmdk + useValue helpers |  |
| 10.1 | 9027 | — | — | 0.60 | partial | Architecture — Discarded approaches (rejected alternatives) | ARCHITECTURE.md section #1 (t=7554, 5 atoms) |
| 10.2 | 9480 | — | — | 0.00 | missing | test/pages/group — fixture for the group.test specs |  |
| 10.3 | 9603 | 3017 | -6586 | 0.86 | early | Architecture — Performance + Groups bodies | headings outline in ARCHITECTURE.md (t=238, 4 atoms) |
| 10.4 | 9626 | 2599 | -7027 | 1.00 | early | README — Install snippet | README.md section #1 (t=2599, 3 atoms) |
| 10.5 | 9729 | 348 | -9381 | 0.91 | early | tsup build config | export at cmdk/tsup.config.ts:3 (t=348, 9 atoms) |
| 10.6 | 9892 | 1786 | -8106 | 0.83 | early | index.tsx — Separator body | export body at cmdk/src/index.tsx:774 (t=1344, 5 atoms) |
| 10.7 | 9996 | — | — | 0.78 | partial | index.tsx — Empty body | export body at cmdk/src/index.tsx:899 (t=1247, 3 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 8 | 1428 | README.md section #<n> |
| 2 | 663 | ARCHITECTURE.md section #<n> |
| 2 | 118 | export doc at cmdk/src/index.tsx:<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 467 | 1.00 | 467 | 4341 | export body at cmdk/src/index.tsx:833 |
| 400 | 1.00 | 400 | 3439 | export body at cmdk/src/index.tsx:787 |
| 386 | 1.00 | 386 | 7121 | ARCHITECTURE.md section #0 |
| 294 | 1.00 | 294 | 9026 | README.md section #5 |
| 277 | 1.00 | 277 | 6578 | ARCHITECTURE.md section #2 |
| 253 | 1.00 | 253 | 8732 | README.md section #18 |
| 242 | 1.00 | 242 | 2267 | json config tsconfig.json |
| 210 | 1.00 | 210 | 558 | package identity in cmdk/package.json |
| 205 | 1.00 | 205 | 8203 | README.md section #16 |
| 179 | 1.00 | 179 | 7998 | README.md section #7 |
| 179 | 1.00 | 179 | 1701 | export body at cmdk/src/index.tsx:909 |
| 178 | 1.00 | 178 | 1522 | export body at cmdk/src/index.tsx:882 |
| 160 | 0.43 | 375 | 5529 | README.md section #20 |
| 145 | 1.00 | 145 | 703 | package entrypoints in cmdk/package.json |
| 143 | 1.00 | 143 | 2923 | README.md section #21 |
| 113 | 1.00 | 113 | 2780 | README.md section #22 |
| 95 | 0.50 | 189 | 6301 | package dependencies in cmdk/package.json |
| 81 | 1.00 | 81 | 6735 | README.md section #6 |
| 80 | 0.28 | 290 | 2576 | headings outline in README.md |
| 78 | 1.00 | 78 | 781 | package scripts in cmdk/package.json |
| 66 | 1.00 | 66 | 2025 | export doc at cmdk/src/index.tsx:664 |
| 52 | 1.00 | 52 | 1876 | export doc at cmdk/src/index.tsx:833 |
