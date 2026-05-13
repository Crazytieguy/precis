scores: Score(3000)=0.677 ns_rows≤3K=22/43 (reached=13 partial=4 missing=5)

## Per-budget scores

| B | A_B | I(B) | C(B) | compl(B) | Score(B) | walker_used |
|--:|----:|-----:|-----:|---------:|---------:|------------:|
| 1000 | 90 | 0.821 | 0.560 | 0.889 | 0.678 | 877 |
| 1442 | 133 | 0.818 | 0.493 | 0.770 | 0.635 | 1439 |
| 2080 | 180 | 0.776 | 0.364 | 0.770 | 0.532 | 1439 |
| 3000 | 261 | 0.828 | 0.554 | 0.873 | 0.677 | 2906 |
| 4327 | 386 | 0.827 | 0.463 | 0.923 | 0.619 | 4275 |
| 6240 | 543 | 0.853 | 0.433 | 0.905 | 0.607 | 5881 |
| 9000 | 787 | 0.809 | 0.490 | 0.922 | 0.630 | 5881 |

## Top opportunities

### Additive (close partial / missing rows)

| intervention | rows | gap@1k | gap@3k | gap@9k | evidence | top row ids |
|:-------------|-----:|-------:|-------:|-------:|:---------|:------------|
| split wrong-slice walker batches | 23 | 1.79 | 1.65 | 1.13 | nearby candidates have low exact atom overlap | 2.3, 3.6, 3.7, 4.7, 5.3, ... |
| add walker candidates for no-discovered rows | 2 | 0.13 | 0.13 | 0.13 | NS rows have no discovered line candidate | 4.4, 4.1 |

### Subtractive (suppress consistently off-NS batches)

| pattern | batches | freed@1k | freed@3k | freed@9k | evidence | top batch ids |
|:--------|--------:|---------:|---------:|---------:|:---------|:--------------|
| package identity in package.json | 1 | 126 | 126 | 126 | off_3k=126 | package identity in package.json |
| package entrypoints in package.json | 1 | 0 | 120 | 120 | off_3k=120 | package entrypoints in package.json |
| README.md section #<n> | 2 | 0 | 80 | 212 | off_3k=183 | README.md section #4, README.md section #1 |
| headings outline in README.md | 1 | 29 | 29 | 29 | off_3k=91 | headings outline in README.md |

Top missed paths (NS rows ≤ 3K): README.md (4 rows, 77 atoms), test/index_test.ts (1 row, 25 atoms), src/index.ts (2 rows, 19 atoms), .github/workflows/compressed-size.yml (1 row, 12 atoms), test (1 row, 3 atoms)

## Arrival ledger by diagnosis

### wrong-slice / granularity

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 1.2 | 57 | 0.67 | partial | README title + tagline | [scheduled bbox exact=2/3] README headline in README.md (t=393, 2 atoms) |
| 2.3 | 889 | 0.00 | missing | README quickstart code example | [scheduled bbox exact=16/22] README.md section #3 (t=4839, 16 atoms) |
| 2.5 | 989 | 0.75 | partial | Handler / WildcardHandler type aliases | [scheduled bbox exact=4/8] export names surface in src/index.ts (t=553, 5 atoms) |
| 2.6 | 1096 | 0.73 | partial | EventHandlerMap type | [scheduled bbox exact=5/11] export names surface in src/index.ts (t=553, 5 atoms) |
| 2.7 | 1302 | 0.81 | partial | README API one-line method descriptions | [scheduled bbox exact=10/21] headings outline in README.md (t=1023, 10 atoms) |
| 3.6 | 2294 | 0.06 | missing | README TypeScript usage section | [scheduled bbox exact=21/31] README.md section #3 (t=4839, 21 atoms) |
| 3.7 | 2626 | 0.00 | missing | test/index_test.ts test labels — all describe + it titles | [scheduled same-file] imports in test/index_test.ts (t=5592, 4 atoms) |
| 4.2 | 3089 | 0.00 | missing | package.json mocha + prettier blocks | [scheduled same-file] package dependencies in package.json (t=3272, 23 atoms) |
| 4.6 | 3890 | 0.00 | missing | test-types-compilation.ts preamble — Events type + handler decls | [scheduled bbox exact=1/20] imports in test/test-types-compilation.ts (t=5348, 1 atoms) |
| 4.7 | 4287 | 0.00 | missing | test-types-compilation.ts on()/off() blocks | [scheduled same-file] imports in test/test-types-compilation.ts (t=5348, 1 atoms) |
| 4.8 | 4485 | 0.00 | missing | test-types-compilation.ts emit() block | [scheduled same-file] imports in test/test-types-compilation.ts (t=5348, 1 atoms) |
| 4.9 | 4724 | 0.00 | missing | test/index_test.ts imports + outer-block tests | [scheduled bbox exact=4/23] imports in test/index_test.ts (t=5592, 4 atoms) |
| 4.10 | 4955 | 0.00 | missing | test/index_test.ts mitt# Events type + beforeEach | [scheduled same-file] imports in test/index_test.ts (t=5592, 4 atoms) |
| 5.1 | 5244 | 0.08 | missing | README install section | [scheduled bbox exact=17/25] README.md section #2 (t=4002, 17 atoms) |
| 5.2 | 5401 | 0.00 | missing | Test body: wildcard '*' invocation | [scheduled same-file] imports in test/index_test.ts (t=5592, 4 atoms) |
| 5.3 | 5926 | 0.00 | missing | Test bodies: on() registration semantics | [scheduled same-file] imports in test/index_test.ts (t=5592, 4 atoms) |
| 5.4 | 6411 | 0.00 | missing | Test bodies: off() removal semantics | [scheduled same-file] imports in test/index_test.ts (t=5592, 4 atoms) |
| 5.5 | 6664 | 0.00 | missing | Test bodies: emit() typed dispatch + case sensitivity | [scheduled same-file] imports in test/index_test.ts (t=5592, 4 atoms) |
| 5.6 | 7020 | 0.00 | missing | README API parameter tables | [scheduled bbox exact=0/15] headings outline in README.md (t=1023, 4 atoms) |
| 5.8 | 8020 | 0.51 | missing | README Examples / Contribute / License sections | [scheduled bbox exact=10/37] headings outline in README.md (t=1023, 22 atoms) |
| 5.10 | 8625 | 0.00 | missing | .editorconfig + .gitignore | [scheduled bbox exact=15/27] plaintext config .editorconfig (t=3760, 15 atoms) |
| 5.11 | 8837 | 0.16 | missing | .github/PULL_REQUEST_TEMPLATE.md | [scheduled bbox exact=8/19] .github/PULL_REQUEST_TEMPLATE.md section #0 (t=3377, 8 atoms) |
| 5.12 | 8863 | 0.00 | missing | LICENSE — MIT preamble | [scheduled bbox exact=2/3] plaintext config LICENSE (t=5881, 2 atoms) |

### no discovered candidate

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 4.1 | 2949 | 0.00 | missing | compressed-size CI workflow | no discovered line candidate |
| 4.4 | 3455 | 0.00 | missing | CI workflow (main.yml) | no discovered line candidate |

### fs/listing

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 1.4 | 120 | 0.33 | missing | src/ and test/ listings | fs-only |

### mixed/unknown

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 5.7 | 7520 | 0.00 | missing | .eslintrc — full | [scheduled bbox exact=52/52] plaintext config .eslintrc (t=5339, 52 atoms) |
| 5.9 | 8386 | 0.00 | missing | package.json devDependencies | [scheduled bbox exact=23/23] package dependencies in package.json (t=3272, 23 atoms) |

Top wasted paths (off-NS at 3K): package.json (847t, 4 batches), README.md (274t, 3 batches), tsconfig.json (149t, 1 batch)

## Walker waste rollup (by descriptor pattern)

| n | off_3k_total | pattern |
|--:|-------------:|:--------|
| 2 | 183 | README.md section #<n> |

## Walker waste (walker_t ≤ 3000, off_3k ≥ 50)

| off_3k | off_3k_ratio | off_any | cost | walker_t | batch |
|-------:|-------------:|--------:|-----:|---------:|:------|
| 366 | 1.00 | 0 | 366 | 2906 | package dependencies in package.json |
| 235 | 1.00 | 0 | 235 | 2310 | package scripts in package.json |
| 149 | 1.00 | 0 | 149 | 2662 | json config tsconfig.json |
| 126 | 0.73 | 126 | 173 | 52 | package identity in package.json |
| 120 | 0.57 | 120 | 212 | 1227 | package entrypoints in package.json |
| 103 | 1.00 | 0 | 103 | 2559 | README.md section #4 |
| 91 | 0.62 | 29 | 146 | 877 | headings outline in README.md |
| 80 | 1.00 | 80 | 80 | 1099 | README.md section #1 |
