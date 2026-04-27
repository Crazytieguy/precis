scores: Sim=0.304 Reached=19/42 Early=9 Late=9 Partial=3 Missing=20 Used=7336/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 8 | 6 | 0 | 2 | 0.75 |
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
| 1.4 | 257 | 6221 | +5964 | 1.00 | late | Library runtime dependencies | package dependencies in cmdk/package.json (t=6221, 6 atoms) |
| 1.5 | 291 | 1659 | +1368 | 1.00 | late | pnpm workspace members | plaintext config pnpm-workspace.yaml (t=1659, 4 atoms) |
| 1.6 | 409 | — | — | 0.00 | missing | Test app layout — pages + test files |  |
| 1.7 | 549 | — | — | 0.00 | missing | Website (showcase) layout — themes + components |  |
| 1.8 | 751 | 2255 | +1504 | 1.00 | late | Root scripts — how to build/run/test | package scripts in package.json (t=2255, 10 atoms) |
| 2.1 | 802 | 1949 | +1147 | 1.00 | late | README — top-level section headings | README.md section #2 (t=7336, 54 atoms) |
| 2.2 | 949 | 1949 | +1000 | 1.00 | late | README — every Parts heading (with [cmdk-*] selectors) | headings outline in README.md (t=1949, 19 atoms) |
| 2.3 | 1166 | 7336 | +6170 | 0.87 | late | README — basic Use snippet | README.md section #2 (t=7336, 20 atoms) |
| 3.1 | 1232 | — | — | 0.00 | missing | Architecture — Approach paragraph (DOM-as-truth) |  |
| 4.1 | 1342 | — | — | 0.00 | missing | index.tsx — module imports + 'use client' |  |
| 4.2 | 1557 | — | — | 0.54 | partial | index.tsx — public exports surface | export names surface in cmdk/src/index.tsx (t=788, 13 atoms) |
| 4.3 | 1718 | 788 | -930 | 1.00 | early | index.tsx — every component declaration line | export body at cmdk/src/index.tsx:664 (t=4468, 49 atoms) |
| 4.4 | 1857 | — | — | 0.00 | missing | index.tsx — data-attribute selectors + SELECT_EVENT |  |
| 4.5 | 2215 | — | — | 0.00 | missing | index.tsx — internal type system (Context/State/Store) |  |
| 5.1 | 2516 | — | — | 0.00 | missing | README — Item subsection (the most-asked-about part) |  |
| 5.2 | 2755 | — | — | 0.17 | missing | README — Command (root) value/filter/keywords/loop prose | headings outline in README.md (t=1949, 2 atoms) |
| 5.3 | 3157 | 7336 | +4179 | 0.84 | late | README — Dialog usage with ⌘K keybind | README.md section #2 (t=7336, 32 atoms) |
| 5.4 | 3397 | — | — | 0.55 | partial | FAQ — accessibility, virtualization, RSC, etc. | README.md section #20 (t=6596, 6 atoms) |
| 5.5 | 3705 | — | — | 0.47 | missing | README — Group / Separator / Empty / Loading subsections | headings outline in README.md (t=1949, 8 atoms) |
| 5.6 | 3856 | — | — | 0.20 | missing | useCommandState — state slice subscription hook | headings outline in README.md (t=1949, 2 atoms) |
| 6.1 | 4490 | — | — | 0.00 | missing | index.tsx — onKeyDown switch (next/prev/Home/End/Enter) |  |
| 6.2 | 5181 | — | — | 0.00 | missing | index.tsx — sort() body (DOM-as-truth, in code) |  |
| 7.1 | 5336 | 3832 | -1504 | 0.88 | aligned | command-score — exported scoring function signature | export body at cmdk/src/command-score.ts:155 (t=3832, 6 atoms) |
| 7.2 | 5647 | — | — | 0.00 | missing | command-score — SCORE_* constants (positive weights) |  |
| 7.3 | 6081 | — | — | 0.00 | missing | command-score — PENALTY_* constants (decay weights) |  |
| 8.1 | 6245 | — | — | 0.00 | missing | Test file → describe-block names |  |
| 8.2 | 6516 | — | — | 0.00 | missing | Playwright config — test dir + dev-server hookup |  |
| 8.3 | 6992 | — | — | 0.00 | missing | test/pages/keybinds — fixture for every keybind spec |  |
| 9.1 | 7365 | 4468 | -2897 | 0.86 | early | index.tsx — Item header (registration + state subscriptions) | export body at cmdk/src/index.tsx:664 (t=4468, 23 atoms) |
| 9.2 | 7686 | 4468 | -3218 | 0.81 | early | index.tsx — Item render output (the cmdk-item div) | export body at cmdk/src/index.tsx:664 (t=4468, 26 atoms) |
| 9.3 | 8154 | 3223 | -4931 | 0.88 | early | index.tsx — Group component body | export body at cmdk/src/index.tsx:729 (t=3223, 34 atoms) |
| 9.4 | 8625 | — | — | 0.00 | missing | index.tsx — useCmdk + useValue helpers |  |
| 10.1 | 9027 | — | — | 0.20 | missing | Architecture — Discarded approaches (rejected alternatives) | headings outline in ARCHITECTURE.md (t=289, 2 atoms) |
| 10.2 | 9480 | — | — | 0.00 | missing | test/pages/group — fixture for the group.test specs |  |
| 10.3 | 9603 | 4555 | -5048 | 0.86 | early | Architecture — Performance + Groups bodies | headings outline in ARCHITECTURE.md (t=289, 4 atoms) |
| 10.4 | 9626 | 1983 | -7643 | 1.00 | early | README — Install snippet | README.md section #1 (t=1983, 3 atoms) |
| 10.5 | 9729 | 362 | -9367 | 0.91 | early | tsup build config | export at cmdk/tsup.config.ts:3 (t=362, 9 atoms) |
| 10.6 | 9892 | 1507 | -8385 | 0.83 | early | index.tsx — Separator body | export body at cmdk/src/index.tsx:774 (t=944, 5 atoms) |
| 10.7 | 9996 | — | — | 0.78 | partial | index.tsx — Empty body | export body at cmdk/src/index.tsx:899 (t=847, 3 atoms) |

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
| 133 | 1.00 | 133 | 2388 | package dependencies in package.json |
| 123 | 1.00 | 123 | 5599 | plaintext config .gitignore |
| 113 | 1.00 | 113 | 4668 | README.md section #22 |
| 95 | 0.50 | 189 | 6221 | package dependencies in cmdk/package.json |
| 81 | 1.00 | 81 | 6753 | README.md section #6 |
| 80 | 0.28 | 290 | 1949 | headings outline in README.md |
| 78 | 1.00 | 78 | 6032 | package scripts in cmdk/package.json |
| 66 | 1.00 | 66 | 1625 | export doc at cmdk/src/index.tsx:664 |
| 52 | 1.00 | 52 | 1559 | export doc at cmdk/src/index.tsx:833 |
