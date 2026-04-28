scores: Sim=0.562 Reached=21/43 Early=6 Late=7 Partial=9 Missing=13 Used=5881/10000

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 0 ranking-recoverable (w×gap=0.00), 20 wrong-slice/granularity (w×gap=2.14), 2 no-discovered (w×gap=0.41)
Secondary intervention: split wrong-slice batches for 20 rows
Loss reasons: 0 predecessor-gated, 0 too-expensive, 0 discovered-unscheduled
Top rows: 1.2, 3.7, 4.2, 2.3, 2.6, ...
Note: likely lever is heuristic; verify `Sim` moves, not just bucket counts.

## Top opportunities

_`w(t)×gap` is a non-additive priority score: Σ exp(-exp_t/τ) × (1 - credit) per row, τ=2000. Same time weighting and credit gap as Sim, but rows can overlap between opportunities — sums across rows are an upper bound on Sim impact, not an additive estimate._

| intervention | rows | w(t)×gap | bands ≤3k/≤6k/total | evidence | top row ids |
|:-------------|-----:|---------:|:----------------------|:---------|:------------|
| split wrong-slice walker batches | 20 | 2.14 | 6/15/20 | nearby candidates have low exact atom overlap | 1.2, 3.7, 4.2, 2.3, 2.6, ... |
| add walker candidates for no-discovered rows | 2 | 0.41 | 1/2/2 | NS rows have no discovered line candidate | 4.1, 4.4 |

Tiers: 1=5/6 reached, 1 partial, 0 missing, avg=0.92; 2=4/7 reached, 3 partial, 0 missing, avg=0.85; 3=6/8 reached, 1 partial, 1 missing, avg=0.81; 4=2/10 reached, 0 partial, 8 missing, avg=0.22; 5=4/12 reached, 4 partial, 4 missing, avg=0.52

## Diagnosis rollup

| diagnosis | rows | missing | partial | timing | likely lever |
|:----------|-----:|--------:|--------:|-------:|:-------------|
| wrong-slice / granularity | 20 | 11 | 9 | 0 | walker granularity / wrong slice |
| no discovered candidate | 2 | 2 | 0 | 0 | walker coverage or predecessor-gated emit |
| timing-only | 16 | 0 | 0 | 16 | usually no code change |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=26, scheduled same-file=9, fs-only=1, no discovered candidate=2

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | aligned | low | 1 |
| scheduled bbox | aligned | high | 2 |
| scheduled bbox | early | low | 2 |
| scheduled bbox | early | full | 4 |
| scheduled bbox | late | low | 1 |
| scheduled bbox | late | high | 2 |
| scheduled bbox | late | full | 3 |
| scheduled bbox | missing | low | 2 |
| scheduled bbox | partial | low | 9 |

## Arrival ledger by diagnosis

### wrong-slice / granularity

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 57 | — | — | 0.67 | partial | README title + tagline | [scheduled bbox exact=2/3] README headline in README.md (t=393, 2 atoms) |
| 2.3 | 889 | — | — | 0.73 | partial | README quickstart code example | [scheduled bbox exact=16/22] README.md section #3 (t=4418, 16 atoms) |
| 2.5 | 989 | — | — | 0.75 | partial | Handler / WildcardHandler type aliases | [scheduled bbox exact=4/8] export names surface in src/index.ts (t=553, 5 atoms) |
| 2.6 | 1096 | — | — | 0.73 | partial | EventHandlerMap type | [scheduled bbox exact=5/11] export names surface in src/index.ts (t=553, 5 atoms) |
| 3.6 | 2294 | — | — | 0.71 | partial | README TypeScript usage section | [scheduled bbox exact=21/31] README.md section #3 (t=4418, 21 atoms) |
| 3.7 | 2626 | — | — | 0.00 | missing | test/index_test.ts test labels — all describe + it titles | [scheduled same-file] imports in test/index_test.ts (t=5592, 4 atoms) |
| 4.2 | 3089 | — | — | 0.00 | missing | package.json mocha + prettier blocks | [scheduled same-file] package dependencies in package.json (t=3177, 23 atoms) |
| 4.6 | 3890 | — | — | 0.05 | missing | test-types-compilation.ts preamble — Events type + handler decls | [scheduled bbox exact=1/20] imports in test/test-types-compilation.ts (t=4927, 1 atoms) |
| 4.7 | 4287 | — | — | 0.00 | missing | test-types-compilation.ts on()/off() blocks | [scheduled same-file] imports in test/test-types-compilation.ts (t=4927, 1 atoms) |
| 4.8 | 4485 | — | — | 0.00 | missing | test-types-compilation.ts emit() block | [scheduled same-file] imports in test/test-types-compilation.ts (t=4927, 1 atoms) |
| 4.9 | 4724 | — | — | 0.17 | missing | test/index_test.ts imports + outer-block tests | [scheduled bbox exact=4/23] imports in test/index_test.ts (t=5592, 4 atoms) |
| 4.10 | 4955 | — | — | 0.00 | missing | test/index_test.ts mitt# Events type + beforeEach | [scheduled same-file] imports in test/index_test.ts (t=5592, 4 atoms) |
| 5.1 | 5244 | — | — | 0.72 | partial | README install section | [scheduled bbox exact=17/25] README.md section #2 (t=3907, 17 atoms) |
| 5.2 | 5401 | — | — | 0.00 | missing | Test body: wildcard '*' invocation | [scheduled same-file] imports in test/index_test.ts (t=5592, 4 atoms) |
| 5.3 | 5926 | — | — | 0.00 | missing | Test bodies: on() registration semantics | [scheduled same-file] imports in test/index_test.ts (t=5592, 4 atoms) |
| 5.4 | 6411 | — | — | 0.00 | missing | Test bodies: off() removal semantics | [scheduled same-file] imports in test/index_test.ts (t=5592, 4 atoms) |
| 5.5 | 6664 | — | — | 0.00 | missing | Test bodies: emit() typed dispatch + case sensitivity | [scheduled same-file] imports in test/index_test.ts (t=5592, 4 atoms) |
| 5.6 | 7020 | — | — | 0.60 | partial | README API parameter tables | [scheduled bbox exact=3/15] README.md section #10 (t=5202, 7 atoms) |
| 5.11 | 8837 | — | — | 0.58 | partial | .github/PULL_REQUEST_TEMPLATE.md | [scheduled bbox exact=8/19] .github/PULL_REQUEST_TEMPLATE.md section #0 (t=3282, 8 atoms) |
| 5.12 | 8863 | — | — | 0.67 | partial | LICENSE — MIT preamble | [scheduled bbox exact=2/3] plaintext config LICENSE (t=5881, 2 atoms) |

### no discovered candidate

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 4.1 | 2949 | — | — | 0.00 | missing | compressed-size CI workflow | no discovered line candidate |
| 4.4 | 3455 | — | — | 0.00 | missing | CI workflow (main.yml) | no discovered line candidate |

### timing-only

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.3 | 104 | 225 | +121 | 1.00 | late | package.json name + version + description | [scheduled bbox exact=3/3] package identity in package.json (t=225, 3 atoms) |
| 1.4 | 120 | 3919 | +3799 | 1.00 | late | src/ and test/ listings | fs-only |
| 1.5 | 212 | 1409 | +1197 | 1.00 | late | package.json entrypoint + source fields | [scheduled bbox exact=6/6] package entrypoints in package.json (t=1409, 6 atoms) |
| 1.6 | 362 | 393 | +31 | 0.86 | aligned | README feature bullets | [scheduled bbox exact=6/7] README headline in README.md (t=393, 6 atoms) |
| 2.1 | 470 | 553 | +83 | 1.00 | aligned+over | src/index.ts public exports — name-only locations | [scheduled bbox exact=1/8] export at src/index.ts:23 (t=829, 14 atoms) |
| 2.2 | 667 | 829 | +162 | 0.82 | aligned | Emitter<Events> interface — full | [scheduled bbox exact=14/17] export at src/index.ts:23 (t=829, 14 atoms) |
| 2.4 | 912 | 588 | -324 | 1.00 | early | mitt() default-export signature | [scheduled bbox exact=3/3] export at src/index.ts:46 (t=588, 3 atoms) |
| 2.7 | 1302 | 5202 | +3900 | 0.90 | late | README API one-line method descriptions | [scheduled bbox exact=10/21] headings outline in README.md (t=1023, 10 atoms) |
| 3.1 | 1401 | 2280 | +879 | 0.82 | late | mitt() body — Map default + return-shape skeleton | [scheduled bbox exact=9/11] export body at src/index.ts:46 body 49 (t=2280, 9 atoms) |
| 3.2 | 1606 | 2280 | +674 | 0.95 | late | emit() body — the only non-trivial method | [scheduled bbox exact=18/19] export body at src/index.ts:46 body 49 (t=2280, 18 atoms) |
| 3.3 | 1717 | 2280 | +563 | 1.00 | late | on() body | [scheduled bbox exact=8/8] export body at src/index.ts:46 body 49 (t=2280, 8 atoms) |
| 4.5 | 3690 | 2515 | -1175 | 1.00 | early | package.json scripts | [scheduled bbox exact=12/12] package scripts in package.json (t=2515, 12 atoms) |
| 5.7 | 7520 | 4918 | -2602 | 1.00 | early | .eslintrc — full | [scheduled bbox exact=52/52] plaintext config .eslintrc (t=4918, 52 atoms) |
| 5.8 | 8020 | 5532 | -2488 | 0.84 | early | README Examples / Contribute / License sections | [scheduled bbox exact=10/37] headings outline in README.md (t=1023, 22 atoms) |
| 5.9 | 8386 | 3177 | -5209 | 1.00 | early | package.json devDependencies | [scheduled bbox exact=23/23] package dependencies in package.json (t=3177, 23 atoms) |
| 5.10 | 8625 | 3665 | -4960 | 0.89 | early | .editorconfig + .gitignore | [scheduled bbox exact=15/27] plaintext config .editorconfig (t=3665, 15 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 2 | 212 | README.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 268 | 0.93 | 289 | 5881 | plaintext config LICENSE |
| 132 | 1.00 | 132 | 3414 | README.md section #5 |
| 126 | 0.73 | 173 | 225 | package identity in package.json |
| 120 | 0.57 | 212 | 1409 | package entrypoints in package.json |
| 80 | 1.00 | 80 | 1166 | README.md section #1 |
