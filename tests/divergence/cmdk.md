scores: Sim=0.275 Reached=14/42 Early=3 Late=10 Partial=7 Missing=21 Used=9998/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 8 | 6 | 0 | 2 | 0.75 |
| 2 | 3 | 3 | 0 | 0 | 0.96 |
| 3 | 1 | 1 | 0 | 0 | 1.00 |
| 4 | 5 | 0 | 1 | 4 | 0.11 |
| 5 | 6 | 2 | 4 | 0 | 0.71 |
| 6 | 2 | 0 | 0 | 2 | 0.00 |
| 7 | 3 | 0 | 0 | 3 | 0.05 |
| 8 | 3 | 0 | 0 | 3 | 0.03 |
| 9 | 4 | 0 | 0 | 4 | 0.00 |
| 10 | 7 | 2 | 2 | 3 | 0.44 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 78 | 261 | +183 | 1.00 | late | README lede — one-sentence elevator pitch | README headline in README.md (t=261, 1 atoms) |
| 1.2 | 140 | 62 | -78 | 1.00 | early | Top-level filesystem layout |  |
| 1.3 | 163 | 1118 | +955 | 1.00 | late | Library inner layout — only two source files |  |
| 1.4 | 257 | 2822 | +2565 | 1.00 | late | Library runtime dependencies | package dependencies in cmdk/package.json (t=2822, 6 atoms) |
| 1.5 | 291 | — | — | 0.00 | missing | pnpm workspace members |  |
| 1.6 | 409 | 8086 | +7677 | 1.00 | late | Test app layout — pages + test files |  |
| 1.7 | 549 | 8401 | +7852 | 1.00 | late | Website (showcase) layout — themes + components |  |
| 1.8 | 751 | — | — | 0.00 | missing | Root scripts — how to build/run/test |  |
| 2.1 | 802 | 4491 | +3689 | 1.00 | late | README — top-level section headings | README.md section #3 (t=7380, 148 atoms) |
| 2.2 | 949 | 7380 | +6431 | 1.00 | late | README — every Parts heading (with [cmdk-*] selectors) | README.md section #3 (t=7380, 140 atoms) |
| 2.3 | 1166 | 2619 | +1453 | 0.87 | late | README — basic Use snippet | README.md section #2 (t=2619, 20 atoms) |
| 3.1 | 1232 | 5334 | +4102 | 1.00 | late | Architecture — Approach paragraph (DOM-as-truth) | ARCHITECTURE.md section #1 (t=5334, 1 atoms) |
| 4.1 | 1342 | — | — | 0.00 | missing | index.tsx — module imports + 'use client' |  |
| 4.2 | 1557 | — | — | 0.54 | partial | index.tsx — public exports surface | export names surface in cmdk/src/index.tsx (t=1254, 13 atoms) |
| 4.3 | 1718 | — | — | 0.00 | missing | index.tsx — every component declaration line |  |
| 4.4 | 1857 | — | — | 0.00 | missing | index.tsx — data-attribute selectors + SELECT_EVENT |  |
| 4.5 | 2215 | — | — | 0.00 | missing | index.tsx — internal type system (Context/State/Store) |  |
| 5.1 | 2516 | 7380 | +4864 | 0.81 | late | README — Item subsection (the most-asked-about part) | README.md section #3 (t=7380, 22 atoms) |
| 5.2 | 2755 | — | — | 0.75 | partial | README — Command (root) value/filter/keywords/loop prose | README.md section #3 (t=7380, 51 atoms) |
| 5.3 | 3157 | 2619 | -538 | 0.84 | aligned | README — Dialog usage with ⌘K keybind | README.md section #2 (t=2619, 32 atoms) |
| 5.4 | 3397 | — | — | 0.55 | partial | FAQ — accessibility, virtualization, RSC, etc. | README.md section #5 (t=2029, 6 atoms) |
| 5.5 | 3705 | — | — | 0.59 | partial | README — Group / Separator / Empty / Loading subsections | README.md section #3 (t=7380, 19 atoms) |
| 5.6 | 3856 | — | — | 0.70 | partial | useCommandState — state slice subscription hook | README.md section #3 (t=7380, 7 atoms) |
| 6.1 | 4490 | — | — | 0.00 | missing | index.tsx — onKeyDown switch (next/prev/Home/End/Enter) |  |
| 6.2 | 5181 | — | — | 0.00 | missing | index.tsx — sort() body (DOM-as-truth, in code) |  |
| 7.1 | 5336 | — | — | 0.14 | missing | command-score — exported scoring function signature | export names surface in cmdk/src/command-score.ts (t=1278, 2 atoms) |
| 7.2 | 5647 | — | — | 0.00 | missing | command-score — SCORE_* constants (positive weights) |  |
| 7.3 | 6081 | — | — | 0.00 | missing | command-score — PENALTY_* constants (decay weights) |  |
| 8.1 | 6245 | — | — | 0.00 | missing | Test file → describe-block names |  |
| 8.2 | 6516 | — | — | 0.00 | missing | Playwright config — test dir + dev-server hookup |  |
| 8.3 | 6992 | — | — | 0.09 | missing | test/pages/keybinds — fixture for every keybind spec | imports in test/pages/keybinds.tsx (t=9835, 3 atoms) |
| 9.1 | 7365 | — | — | 0.00 | missing | index.tsx — Item header (registration + state subscriptions) |  |
| 9.2 | 7686 | — | — | 0.00 | missing | index.tsx — Item render output (the cmdk-item div) |  |
| 9.3 | 8154 | — | — | 0.00 | missing | index.tsx — Group component body |  |
| 9.4 | 8625 | — | — | 0.00 | missing | index.tsx — useCmdk + useValue helpers |  |
| 10.1 | 9027 | — | — | 0.50 | partial | Architecture — Discarded approaches (rejected alternatives) | ARCHITECTURE.md section #1 (t=5334, 5 atoms) |
| 10.2 | 9480 | — | — | 0.07 | missing | test/pages/group — fixture for the group.test specs | imports in test/pages/group.tsx (t=9721, 2 atoms) |
| 10.3 | 9603 | — | — | 0.57 | partial | Architecture — Performance + Groups bodies | ARCHITECTURE.md section #3 (t=1611, 2 atoms) |
| 10.4 | 9626 | 291 | -9335 | 1.00 | early | README — Install snippet | README.md section #1 (t=291, 3 atoms) |
| 10.5 | 9729 | 401 | -9328 | 0.91 | early | tsup build config | export at cmdk/tsup.config.ts:3 (t=401, 9 atoms) |
| 10.6 | 9892 | — | — | 0.00 | missing | index.tsx — Separator body |  |
| 10.7 | 9996 | — | — | 0.00 | missing | index.tsx — Empty body |  |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 1373 | 0.99 | 1385 | 4491 | README.md section #4 |
| 1188 | 0.58 | 2046 | 7380 | README.md section #3 |
| 393 | 1.00 | 393 | 4884 | ARCHITECTURE.md section #0 |
| 284 | 1.00 | 284 | 3106 | ARCHITECTURE.md section #2 |
| 247 | 1.00 | 247 | 9437 | json config website/tsconfig.json |
| 242 | 1.00 | 242 | 1520 | json config tsconfig.json |
| 241 | 1.00 | 241 | 9190 | json config test/tsconfig.json |
| 210 | 1.00 | 210 | 881 | package identity in cmdk/package.json |
| 189 | 1.00 | 189 | 8949 | imports in website/pages/index.tsx |
| 182 | 1.00 | 182 | 8378 | website/README.md section #0 |
| 176 | 0.46 | 382 | 2029 | README.md section #5 |
| 166 | 1.00 | 166 | 8729 | export names surface in website/components/icons/index.tsx |
| 145 | 1.00 | 145 | 1026 | package entrypoints in cmdk/package.json |
| 113 | 1.00 | 113 | 9662 | json config website/vercel.json |
| 112 | 1.00 | 112 | 7982 | website/README.md section #1 |
| 111 | 0.93 | 120 | 521 | README.md section #7 |
| 101 | 0.54 | 189 | 2822 | package dependencies in cmdk/package.json |
| 100 | 0.67 | 150 | 671 | README.md section #6 |
| 97 | 1.00 | 97 | 7700 | website/README.md section #2 |
| 84 | 0.50 | 168 | 261 | README headline in README.md |
| 78 | 1.00 | 78 | 1104 | package scripts in cmdk/package.json |
| 72 | 1.00 | 72 | 7786 | imports in website/components/index.ts |
| 68 | 1.00 | 68 | 8553 | imports in website/components/code/index.tsx |
| 64 | 0.14 | 450 | 5334 | ARCHITECTURE.md section #1 |
