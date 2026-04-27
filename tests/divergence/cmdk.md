scores: Sim=0.303 Reached=18/42 Early=8 Late=8 Partial=3 Missing=21 Used=7302/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 8 | 5 | 0 | 3 | 0.62 |
| 2 | 3 | 3 | 0 | 0 | 0.96 |
| 3 | 1 | 0 | 0 | 1 | 0.00 |
| 4 | 5 | 1 | 1 | 3 | 0.31 |
| 5 | 6 | 1 | 1 | 4 | 0.37 |
| 6 | 2 | 0 | 0 | 2 | 0.00 |
| 7 | 3 | 1 | 0 | 2 | 0.29 |
| 8 | 3 | 0 | 0 | 3 | 0.00 |
| 9 | 4 | 3 | 0 | 1 | 0.64 |
| 10 | 7 | 4 | 1 | 2 | 0.65 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 78 | 222 | +144 | 1.00 | late | README lede — one-sentence elevator pitch | README headline in README.md (t=222, 1 atoms) |
| 1.2 | 140 | 62 | -78 | 1.00 | early | Top-level filesystem layout |  |
| 1.3 | 163 | 232 | +69 | 1.00 | late | Library inner layout — only two source files |  |
| 1.4 | 257 | 5385 | +5128 | 1.00 | late | Library runtime dependencies | package dependencies in cmdk/package.json (t=5385, 6 atoms) |
| 1.5 | 291 | — | — | 0.00 | missing | pnpm workspace members |  |
| 1.6 | 409 | — | — | 0.00 | missing | Test app layout — pages + test files |  |
| 1.7 | 549 | — | — | 0.00 | missing | Website (showcase) layout — themes + components |  |
| 1.8 | 751 | 1054 | +303 | 1.00 | late | Root scripts — how to build/run/test | package scripts in package.json (t=1054, 10 atoms) |
| 2.1 | 802 | 2925 | +2123 | 1.00 | late | README — top-level section headings | README.md section #2 (t=7302, 54 atoms) |
| 2.2 | 949 | 2925 | +1976 | 1.00 | late | README — every Parts heading (with [cmdk-*] selectors) | headings outline in README.md (t=2925, 19 atoms) |
| 2.3 | 1166 | 7302 | +6136 | 0.87 | late | README — basic Use snippet | README.md section #2 (t=7302, 20 atoms) |
| 3.1 | 1232 | — | — | 0.00 | missing | Architecture — Approach paragraph (DOM-as-truth) |  |
| 4.1 | 1342 | — | — | 0.00 | missing | index.tsx — module imports + 'use client' |  |
| 4.2 | 1557 | — | — | 0.54 | partial | index.tsx — public exports surface | export names surface in cmdk/src/index.tsx (t=1423, 13 atoms) |
| 4.3 | 1718 | 1423 | -295 | 1.00 | aligned+over | index.tsx — every component declaration line | export body at cmdk/src/index.tsx:664 (t=5109, 49 atoms) |
| 4.4 | 1857 | — | — | 0.00 | missing | index.tsx — data-attribute selectors + SELECT_EVENT |  |
| 4.5 | 2215 | — | — | 0.00 | missing | index.tsx — internal type system (Context/State/Store) |  |
| 5.1 | 2516 | — | — | 0.00 | missing | README — Item subsection (the most-asked-about part) |  |
| 5.2 | 2755 | — | — | 0.17 | missing | README — Command (root) value/filter/keywords/loop prose | headings outline in README.md (t=2925, 2 atoms) |
| 5.3 | 3157 | 7302 | +4145 | 0.84 | late | README — Dialog usage with ⌘K keybind | README.md section #2 (t=7302, 32 atoms) |
| 5.4 | 3397 | — | — | 0.55 | partial | FAQ — accessibility, virtualization, RSC, etc. | README.md section #20 (t=6562, 6 atoms) |
| 5.5 | 3705 | — | — | 0.47 | missing | README — Group / Separator / Empty / Loading subsections | headings outline in README.md (t=2925, 8 atoms) |
| 5.6 | 3856 | — | — | 0.20 | missing | useCommandState — state slice subscription hook | headings outline in README.md (t=2925, 2 atoms) |
| 6.1 | 4490 | — | — | 0.00 | missing | index.tsx — onKeyDown switch (next/prev/Home/End/Enter) |  |
| 6.2 | 5181 | — | — | 0.00 | missing | index.tsx — sort() body (DOM-as-truth, in code) |  |
| 7.1 | 5336 | 4473 | -863 | 0.88 | aligned | command-score — exported scoring function signature | export body at cmdk/src/command-score.ts:155 (t=4473, 6 atoms) |
| 7.2 | 5647 | — | — | 0.00 | missing | command-score — SCORE_* constants (positive weights) |  |
| 7.3 | 6081 | — | — | 0.00 | missing | command-score — PENALTY_* constants (decay weights) |  |
| 8.1 | 6245 | — | — | 0.00 | missing | Test file → describe-block names |  |
| 8.2 | 6516 | — | — | 0.00 | missing | Playwright config — test dir + dev-server hookup |  |
| 8.3 | 6992 | — | — | 0.00 | missing | test/pages/keybinds — fixture for every keybind spec |  |
| 9.1 | 7365 | 5109 | -2256 | 0.86 | early | index.tsx — Item header (registration + state subscriptions) | export body at cmdk/src/index.tsx:664 (t=5109, 23 atoms) |
| 9.2 | 7686 | 5109 | -2577 | 0.81 | early | index.tsx — Item render output (the cmdk-item div) | export body at cmdk/src/index.tsx:664 (t=5109, 26 atoms) |
| 9.3 | 8154 | 3864 | -4290 | 0.88 | early | index.tsx — Group component body | export body at cmdk/src/index.tsx:729 (t=3864, 34 atoms) |
| 9.4 | 8625 | — | — | 0.00 | missing | index.tsx — useCmdk + useValue helpers |  |
| 10.1 | 9027 | — | — | 0.20 | missing | Architecture — Discarded approaches (rejected alternatives) | headings outline in ARCHITECTURE.md (t=289, 2 atoms) |
| 10.2 | 9480 | — | — | 0.00 | missing | test/pages/group — fixture for the group.test specs |  |
| 10.3 | 9603 | 5196 | -4407 | 0.86 | early | Architecture — Performance + Groups bodies | headings outline in ARCHITECTURE.md (t=289, 4 atoms) |
| 10.4 | 9626 | 2959 | -6667 | 1.00 | early | README — Install snippet | README.md section #1 (t=2959, 3 atoms) |
| 10.5 | 9729 | 362 | -9367 | 0.91 | early | tsup build config | export at cmdk/tsup.config.ts:3 (t=362, 9 atoms) |
| 10.6 | 9892 | 2142 | -7750 | 0.83 | early | index.tsx — Separator body | export body at cmdk/src/index.tsx:774 (t=1579, 5 atoms) |
| 10.7 | 9996 | — | — | 0.78 | partial | index.tsx — Empty body | export body at cmdk/src/index.tsx:899 (t=1482, 3 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 4 | 497 | README.md section #<n> |
| 2 | 118 | export doc at cmdk/src/index.tsx:<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 467 | 1.00 | 467 | 4331 | export body at cmdk/src/index.tsx:833 |
| 400 | 1.00 | 400 | 3429 | export body at cmdk/src/index.tsx:787 |
| 386 | 1.00 | 386 | 6027 | ARCHITECTURE.md section #0 |
| 242 | 1.00 | 242 | 2502 | json config tsconfig.json |
| 210 | 1.00 | 210 | 629 | package identity in cmdk/package.json |
| 179 | 1.00 | 179 | 2094 | export body at cmdk/src/index.tsx:909 |
| 178 | 1.00 | 178 | 1915 | export body at cmdk/src/index.tsx:882 |
| 160 | 0.43 | 375 | 6562 | README.md section #20 |
| 145 | 1.00 | 145 | 774 | package entrypoints in cmdk/package.json |
| 143 | 1.00 | 143 | 5641 | README.md section #21 |
| 133 | 1.00 | 133 | 2635 | package dependencies in package.json |
| 123 | 1.00 | 123 | 6187 | plaintext config .gitignore |
| 113 | 1.00 | 113 | 5498 | README.md section #22 |
| 95 | 0.50 | 189 | 5385 | package dependencies in cmdk/package.json |
| 81 | 1.00 | 81 | 6719 | README.md section #6 |
| 80 | 0.28 | 290 | 2925 | headings outline in README.md |
| 78 | 1.00 | 78 | 852 | package scripts in cmdk/package.json |
| 66 | 1.00 | 66 | 2260 | export doc at cmdk/src/index.tsx:664 |
| 52 | 1.00 | 52 | 2194 | export doc at cmdk/src/index.tsx:833 |
