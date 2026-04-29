scores: Score(3000)=0.677 ns_rows≤3K=22/43 (reached=13 partial=4 missing=5)

## Per-budget scores

| B | A_B | I(B) | C(B) | Score(B) | walker_used |
|--:|----:|-----:|-----:|---------:|------------:|
| 1000 | 90 | 0.821 | 0.560 | 0.678 | 877 |
| 1442 | 133 | 0.818 | 0.493 | 0.635 | 1439 |
| 2080 | 180 | 0.776 | 0.364 | 0.532 | 1439 |
| 3000 | 261 | 0.828 | 0.554 | 0.677 | 2906 |
| 4327 | 386 | 0.827 | 0.463 | 0.619 | 4275 |
| 6240 | 543 | 0.853 | 0.433 | 0.607 | 5881 |
| 9000 | 787 | 0.809 | 0.490 | 0.630 | 5881 |

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 0 ranking-recoverable (gap@3k=0.00), 23 wrong-slice/granularity (gap@3k=1.65), 2 no-discovered (gap@3k=0.13)
Secondary intervention: investigate 2 no-discovered rows
Top rows: 2.3, 3.6, 3.7, 4.7, 5.3, ...

## Top opportunities

_`gap@B` is a non-additive priority score: `Σ over atoms in row: (1 − damped_credit(a)) / rank(a)` evaluated at budget B's walker state. Approximates how much closing the row would lift `Score(B)` (via the Importance numerator); not an exact delta. `gap@3k` is the primary sort key. `gap@1k` and `gap@9k` show how the same intervention scales across the budget vector — gap is monotone non-increasing in B (walker has more budget at higher B). Rows can overlap between opportunities; sums are upper bounds on Score(B) impact, not additive estimates._

| intervention | rows | gap@1k | gap@3k | gap@9k | evidence | top row ids |
|:-------------|-----:|-------:|-------:|-------:|:---------|:------------|
| split wrong-slice walker batches | 23 | 1.79 | 1.65 | 1.13 | nearby candidates have low exact atom overlap | 2.3, 3.6, 3.7, 4.7, 5.3, ... |
| add walker candidates for no-discovered rows | 2 | 0.13 | 0.13 | 0.13 | NS rows have no discovered line candidate | 4.4, 4.1 |

## Diagnosis rollup

| diagnosis | rows | missing | partial | likely lever |
|:----------|-----:|--------:|--------:|:-------------|
| wrong-slice / granularity | 23 | 19 | 4 | walker granularity / wrong slice |
| no discovered candidate | 2 | 2 | 0 | walker coverage or predecessor-gated emit |
| fs/listing | 1 | 1 | 0 | filesystem/listing value |
| mixed/unknown | 2 | 2 | 0 | inspect row |

Candidate hint kinds: scheduled bbox=16, scheduled same-file=9, fs-only=1, no discovered candidate=2 _(candidates are walker batches discovered this run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` isn't proof that no emit path exists)._

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | missing | none | 1 |
| scheduled bbox | missing | low | 9 |
| scheduled bbox | missing | full | 2 |
| scheduled bbox | partial | low | 4 |

## Arrival ledger by diagnosis

### wrong-slice / granularity

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 1.2 | 57 | 0.67 | 0.98 | partial | README title + tagline | [scheduled bbox exact=2/3] README headline in README.md (t=393, 2 atoms) |
| 2.3 | 889 | 0.00 | 0.00 | missing | README quickstart code example | [scheduled bbox exact=16/22] README.md section #3 (t=4839, 16 atoms) |
| 2.5 | 989 | 0.75 | 0.69 | partial | Handler / WildcardHandler type aliases | [scheduled bbox exact=4/8] export names surface in src/index.ts (t=553, 5 atoms) |
| 2.6 | 1096 | 0.73 | 0.72 | partial | EventHandlerMap type | [scheduled bbox exact=5/11] export names surface in src/index.ts (t=553, 5 atoms) |
| 2.7 | 1302 | 0.81 | 0.85 | partial | README API one-line method descriptions | [scheduled bbox exact=10/21] headings outline in README.md (t=1023, 10 atoms) |
| 3.6 | 2294 | 0.06 | 0.02 | missing | README TypeScript usage section | [scheduled bbox exact=21/31] README.md section #3 (t=4839, 21 atoms) |
| 3.7 | 2626 | 0.00 | 0.00 | missing | test/index_test.ts test labels — all describe + it titles | [scheduled same-file] imports in test/index_test.ts (t=5592, 4 atoms) |
| 4.2 | 3089 | 0.00 | 0.00 | missing | package.json mocha + prettier blocks | [scheduled same-file] package dependencies in package.json (t=3272, 23 atoms) |
| 4.6 | 3890 | 0.00 | 0.00 | missing | test-types-compilation.ts preamble — Events type + handler decls | [scheduled bbox exact=1/20] imports in test/test-types-compilation.ts (t=5348, 1 atoms) |
| 4.7 | 4287 | 0.00 | 0.00 | missing | test-types-compilation.ts on()/off() blocks | [scheduled same-file] imports in test/test-types-compilation.ts (t=5348, 1 atoms) |
| 4.8 | 4485 | 0.00 | 0.00 | missing | test-types-compilation.ts emit() block | [scheduled same-file] imports in test/test-types-compilation.ts (t=5348, 1 atoms) |
| 4.9 | 4724 | 0.00 | 0.00 | missing | test/index_test.ts imports + outer-block tests | [scheduled bbox exact=4/23] imports in test/index_test.ts (t=5592, 4 atoms) |
| 4.10 | 4955 | 0.00 | 0.00 | missing | test/index_test.ts mitt# Events type + beforeEach | [scheduled same-file] imports in test/index_test.ts (t=5592, 4 atoms) |
| 5.1 | 5244 | 0.08 | 0.02 | missing | README install section | [scheduled bbox exact=17/25] README.md section #2 (t=4002, 17 atoms) |
| 5.2 | 5401 | 0.00 | 0.00 | missing | Test body: wildcard '*' invocation | [scheduled same-file] imports in test/index_test.ts (t=5592, 4 atoms) |
| 5.3 | 5926 | 0.00 | 0.00 | missing | Test bodies: on() registration semantics | [scheduled same-file] imports in test/index_test.ts (t=5592, 4 atoms) |
| 5.4 | 6411 | 0.00 | 0.00 | missing | Test bodies: off() removal semantics | [scheduled same-file] imports in test/index_test.ts (t=5592, 4 atoms) |
| 5.5 | 6664 | 0.00 | 0.00 | missing | Test bodies: emit() typed dispatch + case sensitivity | [scheduled same-file] imports in test/index_test.ts (t=5592, 4 atoms) |
| 5.6 | 7020 | 0.00 | 0.00 | missing | README API parameter tables | [scheduled bbox exact=0/15] headings outline in README.md (t=1023, 4 atoms) |
| 5.8 | 8020 | 0.51 | 0.39 | missing | README Examples / Contribute / License sections | [scheduled bbox exact=10/37] headings outline in README.md (t=1023, 22 atoms) |
| 5.10 | 8625 | 0.00 | 0.00 | missing | .editorconfig + .gitignore | [scheduled bbox exact=15/27] plaintext config .editorconfig (t=3760, 15 atoms) |
| 5.11 | 8837 | 0.16 | 0.45 | missing | .github/PULL_REQUEST_TEMPLATE.md | [scheduled bbox exact=8/19] .github/PULL_REQUEST_TEMPLATE.md section #0 (t=3377, 8 atoms) |
| 5.12 | 8863 | 0.00 | 0.00 | missing | LICENSE — MIT preamble | [scheduled bbox exact=2/3] plaintext config LICENSE (t=5881, 2 atoms) |

### no discovered candidate

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 4.1 | 2949 | 0.00 | 0.00 | missing | compressed-size CI workflow | no discovered line candidate |
| 4.4 | 3455 | 0.00 | 0.00 | missing | CI workflow (main.yml) | no discovered line candidate |

### fs/listing

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 1.4 | 120 | 0.33 | 0.33 | missing | src/ and test/ listings | fs-only |

### mixed/unknown

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 5.7 | 7520 | 0.00 | 0.00 | missing | .eslintrc — full | [scheduled bbox exact=52/52] plaintext config .eslintrc (t=5339, 52 atoms) |
| 5.9 | 8386 | 0.00 | 0.00 | missing | package.json devDependencies | [scheduled bbox exact=23/23] package dependencies in package.json (t=3272, 23 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 2 | 212 | README.md section #<n> |

## Walker waste, primary-actionable (first_t ≤ 3000, off-NS spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 126 | 0.73 | 173 | 225 | package identity in package.json |
| 120 | 0.57 | 212 | 1439 | package entrypoints in package.json |
| 80 | 1.00 | 80 | 1179 | README.md section #1 |

## Walker waste, late (first_t > 3000, off-NS spend ≥ 50) — higher-budget calibration only

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 268 | 0.93 | 289 | 5881 | plaintext config LICENSE |
| 132 | 1.00 | 132 | 3509 | README.md section #5 |
