scores: Sim=0.301 Reached=17/42 Early=4 Late=12 Partial=4 Missing=21 Used=9978/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 8 | 6 | 0 | 2 | 0.75 |
| 2 | 3 | 3 | 0 | 0 | 0.96 |
| 3 | 1 | 1 | 0 | 0 | 1.00 |
| 4 | 5 | 0 | 1 | 4 | 0.11 |
| 5 | 6 | 4 | 2 | 0 | 0.76 |
| 6 | 2 | 0 | 0 | 2 | 0.00 |
| 7 | 3 | 0 | 0 | 3 | 0.05 |
| 8 | 3 | 0 | 0 | 3 | 0.03 |
| 9 | 4 | 0 | 0 | 4 | 0.00 |
| 10 | 7 | 3 | 1 | 3 | 0.49 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 78 | 181 | +103 | 1.00 | late | README lede — one-sentence elevator pitch | README headline in README.md (t=181, 1 atoms) |
| 1.2 | 140 | 62 | -78 | 1.00 | early | Top-level filesystem layout |  |
| 1.3 | 163 | 795 | +632 | 1.00 | late | Library inner layout — only two source files |  |
| 1.4 | 257 | 3040 | +2783 | 1.00 | late | Library runtime dependencies | package dependencies in cmdk/package.json (t=3040, 6 atoms) |
| 1.5 | 291 | — | — | 0.00 | missing | pnpm workspace members |  |
| 1.6 | 409 | 8066 | +7657 | 1.00 | late | Test app layout — pages + test files |  |
| 1.7 | 549 | 8381 | +7832 | 1.00 | late | Website (showcase) layout — themes + components |  |
| 1.8 | 751 | — | — | 0.00 | missing | Root scripts — how to build/run/test |  |
| 2.1 | 802 | 1506 | +704 | 1.00 | late | README — top-level section headings | README.md section #3 (t=7356, 148 atoms) |
| 2.2 | 949 | 1506 | +557 | 1.00 | late | README — every Parts heading (with [cmdk-*] selectors) | README.md section #3 (t=7356, 140 atoms) |
| 2.3 | 1166 | 2837 | +1671 | 0.87 | late | README — basic Use snippet | README.md section #2 (t=2837, 20 atoms) |
| 3.1 | 1232 | 5466 | +4234 | 1.00 | late | Architecture — Approach paragraph (DOM-as-truth) | ARCHITECTURE.md section #1 (t=5466, 1 atoms) |
| 4.1 | 1342 | — | — | 0.00 | missing | index.tsx — module imports + 'use client' |  |
| 4.2 | 1557 | — | — | 0.54 | partial | index.tsx — public exports surface | export names surface in cmdk/src/index.tsx (t=931, 13 atoms) |
| 4.3 | 1718 | — | — | 0.00 | missing | index.tsx — every component declaration line |  |
| 4.4 | 1857 | — | — | 0.00 | missing | index.tsx — data-attribute selectors + SELECT_EVENT |  |
| 4.5 | 2215 | — | — | 0.00 | missing | index.tsx — internal type system (Context/State/Store) |  |
| 5.1 | 2516 | 7356 | +4840 | 0.81 | late | README — Item subsection (the most-asked-about part) | README.md section #3 (t=7356, 22 atoms) |
| 5.2 | 2755 | 7356 | +4601 | 0.83 | late | README — Command (root) value/filter/keywords/loop prose | README.md section #3 (t=7356, 51 atoms) |
| 5.3 | 3157 | 2837 | -320 | 0.84 | aligned | README — Dialog usage with ⌘K keybind | README.md section #2 (t=2837, 32 atoms) |
| 5.4 | 3397 | — | — | 0.55 | partial | FAQ — accessibility, virtualization, RSC, etc. | README.md section #5 (t=2254, 6 atoms) |
| 5.5 | 3705 | — | — | 0.72 | partial | README — Group / Separator / Empty / Loading subsections | README.md section #3 (t=7356, 19 atoms) |
| 5.6 | 3856 | 7356 | +3500 | 0.80 | late | useCommandState — state slice subscription hook | README.md section #3 (t=7356, 7 atoms) |
| 6.1 | 4490 | — | — | 0.00 | missing | index.tsx — onKeyDown switch (next/prev/Home/End/Enter) |  |
| 6.2 | 5181 | — | — | 0.00 | missing | index.tsx — sort() body (DOM-as-truth, in code) |  |
| 7.1 | 5336 | — | — | 0.14 | missing | command-score — exported scoring function signature | export names surface in cmdk/src/command-score.ts (t=955, 2 atoms) |
| 7.2 | 5647 | — | — | 0.00 | missing | command-score — SCORE_* constants (positive weights) |  |
| 7.3 | 6081 | — | — | 0.00 | missing | command-score — PENALTY_* constants (decay weights) |  |
| 8.1 | 6245 | — | — | 0.00 | missing | Test file → describe-block names |  |
| 8.2 | 6516 | — | — | 0.00 | missing | Playwright config — test dir + dev-server hookup |  |
| 8.3 | 6992 | — | — | 0.09 | missing | test/pages/keybinds — fixture for every keybind spec | imports in test/pages/keybinds.tsx (t=9815, 3 atoms) |
| 9.1 | 7365 | — | — | 0.00 | missing | index.tsx — Item header (registration + state subscriptions) |  |
| 9.2 | 7686 | — | — | 0.00 | missing | index.tsx — Item render output (the cmdk-item div) |  |
| 9.3 | 8154 | — | — | 0.00 | missing | index.tsx — Group component body |  |
| 9.4 | 8625 | — | — | 0.00 | missing | index.tsx — useCmdk + useValue helpers |  |
| 10.1 | 9027 | — | — | 0.60 | partial | Architecture — Discarded approaches (rejected alternatives) | ARCHITECTURE.md section #1 (t=5466, 5 atoms) |
| 10.2 | 9480 | — | — | 0.07 | missing | test/pages/group — fixture for the group.test specs | imports in test/pages/group.tsx (t=9701, 2 atoms) |
| 10.3 | 9603 | 1879 | -7724 | 0.86 | early | Architecture — Performance + Groups bodies | headings outline in ARCHITECTURE.md (t=238, 4 atoms) |
| 10.4 | 9626 | 1529 | -8097 | 1.00 | early | README — Install snippet | README.md section #1 (t=1529, 3 atoms) |
| 10.5 | 9729 | 348 | -9381 | 0.91 | early | tsup build config | export at cmdk/tsup.config.ts:3 (t=348, 9 atoms) |
| 10.6 | 9892 | — | — | 0.00 | missing | index.tsx — Separator body |  |
| 10.7 | 9996 | — | — | 0.00 | missing | index.tsx — Empty body |  |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 5 | 2788 | README.md section #<n> |
| 3 | 724 | ARCHITECTURE.md section #<n> |
| 3 | 372 | website/README.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 1318 | 0.99 | 1330 | 4647 | README.md section #4 |
| 1098 | 0.58 | 1890 | 7356 | README.md section #3 |
| 386 | 1.00 | 386 | 5033 | ARCHITECTURE.md section #0 |
| 277 | 1.00 | 277 | 3317 | ARCHITECTURE.md section #2 |
| 247 | 1.00 | 247 | 9417 | json config website/tsconfig.json |
| 242 | 1.00 | 242 | 1197 | json config tsconfig.json |
| 241 | 1.00 | 241 | 9170 | json config test/tsconfig.json |
| 210 | 1.00 | 210 | 558 | package identity in cmdk/package.json |
| 189 | 1.00 | 189 | 8929 | imports in website/pages/index.tsx |
| 182 | 1.00 | 182 | 8358 | website/README.md section #0 |
| 173 | 0.46 | 375 | 2254 | README.md section #5 |
| 166 | 1.00 | 166 | 8709 | export names surface in website/components/icons/index.tsx |
| 145 | 1.00 | 145 | 703 | package entrypoints in cmdk/package.json |
| 138 | 0.48 | 290 | 1506 | headings outline in README.md |
| 113 | 1.00 | 113 | 9642 | json config website/vercel.json |
| 104 | 0.93 | 113 | 1642 | README.md section #7 |
| 104 | 1.00 | 104 | 7792 | website/README.md section #1 |
| 101 | 0.54 | 189 | 3040 | package dependencies in cmdk/package.json |
| 95 | 0.67 | 143 | 1785 | README.md section #6 |
| 86 | 1.00 | 86 | 7688 | website/README.md section #2 |
| 78 | 1.00 | 78 | 781 | package scripts in cmdk/package.json |
| 72 | 1.00 | 72 | 7878 | imports in website/components/index.ts |
| 68 | 1.00 | 68 | 8533 | imports in website/components/code/index.tsx |
| 61 | 0.14 | 433 | 5466 | ARCHITECTURE.md section #1 |
