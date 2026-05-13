scores: Score(3000)=0.658 ns_rows≤3K=20/40 (reached=9 partial=4 missing=7)

## Per-budget scores

| B | A_B | I(B) | C(B) | compl(B) | Score(B) | walker_used |
|--:|----:|-----:|-----:|---------:|---------:|------------:|
| 1000 | 90 | 0.775 | 0.261 | 0.754 | 0.450 | 987 |
| 1442 | 117 | 0.823 | 0.436 | 0.843 | 0.599 | 1436 |
| 2080 | 161 | 0.780 | 0.380 | 0.777 | 0.545 | 1682 |
| 3000 | 257 | 0.849 | 0.511 | 0.801 | 0.658 | 2953 |
| 4327 | 376 | 0.829 | 0.477 | 0.798 | 0.629 | 4057 |
| 6240 | 517 | 0.864 | 0.697 | 0.891 | 0.776 | 6232 |
| 9000 | 728 | 0.838 | 0.540 | 0.844 | 0.673 | 6732 |

## Top opportunities

### Additive (close partial / missing rows)

| intervention | rows | gap@1k | gap@3k | gap@9k | evidence | top row ids |
|:-------------|-----:|-------:|-------:|-------:|:---------|:------------|
| split wrong-slice walker batches | 17 | 1.29 | 1.06 | 0.61 | nearby candidates have low exact atom overlap | 1.3, 3.4, 4.5, 3.2, 5.4, ... |
| add walker candidates for no-discovered rows | 6 | 0.49 | 0.49 | 0.49 | NS rows have no discovered line candidate | 3.5, 5.5, 5.7, 5.6, 3.3, ... |

### Subtractive (suppress consistently off-NS batches)

| pattern | batches | freed@1k | freed@3k | freed@9k | evidence | top batch ids |
|:--------|--------:|---------:|---------:|---------:|:---------|:--------------|
| go module file xxhashbench/go.mod | 1 | 133 | 133 | 133 | off_3k=133 | go module file xxhashbench/go.mod |
| go package + imports in dynamic/plugin.go | 1 | 0 | 55 | 55 | off_3k=55 | go package + imports in dynamic/plugin.go |
| go decl body at dynamic/plugin.go:19 | 1 | 0 | 54 | 54 | off_3k=54 | go decl body at dynamic/plugin.go:19 |
| go decl names surface in dynamic/plugin.go | 1 | 52 | 52 | 52 | off_3k=52 | go decl names surface in dynamic/plugin.go |
| go decl names surface in xxhash.go | 1 | 0 | 8 | 8 | off_3k=108 | go decl names surface in xxhash.go |

Top missed paths (NS rows ≤ 3K): xxhash.go (3 rows, 37 atoms), xxhsum/xxhsum.go (1 row, 32 atoms), .github/workflows/test.yml (1 row, 30 atoms), xxhash_test.go (1 row, 16 atoms), testall.sh (1 row, 10 atoms), xxhash_asm.go (1 row, 9 atoms), xxhash_unsafe.go (1 row, 8 atoms), go.mod (1 row, 6 atoms), +1 more

## Arrival ledger by diagnosis

### wrong-slice / granularity

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 1.3 | 221 | 0.50 | missing | Package doc + module path | [scheduled bbox exact=2/6] go module file go.mod (t=112, 2 atoms) |
| 2.2 | 712 | 0.73 | partial | Constructors — New and NewWithSeed | [scheduled bbox exact=4/11] go decl names surface in xxhash.go (t=2394, 4 atoms) |
| 2.5 | 1196 | 0.78 | partial | Sum64 / writeBlocks signatures (asm build) | [scheduled bbox exact=3/9] go decl doc at xxhash_asm.go:12 (t=626, 3 atoms) |
| 2.6 | 1341 | 0.75 | partial | Sum64String / WriteString signatures (unsafe build) | [scheduled bbox exact=0/8] go package + imports in xxhash_unsafe.go (t=653, 4 atoms) |
| 3.2 | 1648 | 0.12 | missing | All test/benchmark function names across the repo | [scheduled bbox exact=5/16] go test names surface in xxhash_test.go (t=6440, 9 atoms) |
| 3.4 | 2081 | 0.25 | missing | xxhsum CLI — main + usage | [scheduled bbox exact=20/32] go decl body at xxhsum/xxhsum.go:11 (t=6217, 20 atoms) |
| 4.1 | 2592 | 0.62 | missing | Prime constants + primes array | [scheduled bbox exact=7/13] go decl at xxhash.go:11 (t=2421, 7 atoms) |
| 4.2 | 2716 | 0.47 | missing | round + mergeRound — the core mixer | [scheduled bbox exact=4/13] go decl body at xxhash.go:229 (t=3047, 4 atoms) |
| 4.4 | 3084 | 0.69 | partial | Little-endian / append / consume byte helpers | [scheduled bbox exact=6/13] go decl names surface in xxhash.go (t=2394, 6 atoms) |
| 4.5 | 3473 | 0.06 | missing | Write — streaming entry body | [scheduled bbox exact=28/37] go decl body at xxhash.go:75 (t=4409, 28 atoms) |
| 4.8 | 4212 | 0.16 | missing | MarshalBinary body — wire format | [scheduled bbox exact=10/13] go decl body at xxhash.go:176 (t=3440, 10 atoms) |
| 4.12 | 5535 | 0.44 | missing | xxhash_safe.go full body — the appengine fallback | [scheduled bbox exact=4/16] go decl names surface in xxhash_safe.go (t=382, 4 atoms) |
| 4.13 | 5940 | 0.21 | missing | Unsafe string conversion — inlining commentary + sliceHeader | [scheduled bbox exact=2/28] go decl names surface in xxhash_unsafe.go (t=452, 6 atoms) |
| 4.14 | 6070 | 0.70 | partial | Unsafe Sum64String / WriteString bodies | [scheduled bbox exact=5/10] go decl body at xxhash_unsafe.go:45 (t=1628, 5 atoms) |
| 5.2 | 6724 | 0.00 | missing | TestAll — known-answer test vectors | [scheduled bbox exact=2/19] go test names surface in xxhash_test.go (t=6440, 2 atoms) |
| 5.3 | 7016 | 0.00 | missing | bench_test.go — input-size grid + benchmark shape | [scheduled bbox exact=2/32] go test names surface in bench_test.go (t=6359, 2 atoms) |
| 5.4 | 7396 | 0.00 | missing | xxhashbench — structure of the cross-library benchmark | [scheduled bbox exact=2/35] go test names surface in xxhashbench/xxhashbench_test.go (t=6232, 2 atoms) |

### no discovered candidate

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 3.3 | 1765 | 0.00 | missing | testall.sh — full body | no discovered line candidate |
| 3.5 | 2400 | 0.00 | missing | CI matrix — what configurations are tested | no discovered line candidate |
| 5.5 | 8093 | 0.00 | missing | amd64 asm — register defs and macros | no discovered line candidate |
| 5.6 | 8652 | 0.00 | missing | amd64 asm — Sum64 prologue, state setup, and main block loop | no discovered line candidate |
| 5.7 | 9358 | 0.00 | missing | amd64 asm — tail handlers (loop8 / try4 / try1) + finalize | no discovered line candidate |
| 5.8 | 9801 | 0.00 | missing | amd64 asm — writeBlocks body | no discovered line candidate |

### mixed/unknown

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 1.5 | 463 | 0.80 | partial | README — purego/asm note | [scheduled bbox exact=4/5] README.md section #0 (t=1209, 4 atoms) |
| 4.6 | 3949 | 0.05 | missing | Digest.Sum64 — finalize body | [scheduled bbox exact=33/41] go decl body at xxhash.go:129 (t=5065, 33 atoms) |
| 4.7 | 4062 | 0.14 | missing | Digest.Sum — append big-endian bytes | [scheduled bbox exact=12/15] go decl body at xxhash.go:113 (t=3153, 12 atoms) |
| 4.9 | 4421 | 0.11 | missing | UnmarshalBinary body — validation + parse | [scheduled bbox exact=15/18] go decl body at xxhash.go:190 (t=3984, 15 atoms) |
| 4.10 | 5119 | 0.02 | missing | Pure-Go Sum64 body (xxhash_other.go) | [scheduled bbox exact=48/56] go decl body at xxhash_other.go:7 (t=5986, 48 atoms) |
| 4.11 | 5355 | 0.08 | missing | Pure-Go writeBlocks body | [scheduled bbox exact=11/13] go decl body at xxhash_other.go:64 (t=4621, 11 atoms) |
| 5.1 | 6391 | 0.10 | missing | README — Benchmarks section (perf table) | [scheduled bbox exact=16/20] README.md section #2 (t=3782, 16 atoms) |

Top wasted paths (off-NS at 3K): dynamic/plugin.go (161t, 3 batches), xxhashbench/go.mod (133t, 1 batch), README.md (123t, 1 batch), xxhash.go (108t, 1 batch), xxhash_unsafe.go (82t, 1 batch)

## Walker waste (walker_t ≤ 3000, off_3k ≥ 50)

| off_3k | off_3k_ratio | off_any | cost | walker_t | batch |
|-------:|-------------:|--------:|-----:|---------:|:------|
| 133 | 1.00 | 133 | 133 | 767 | go module file xxhashbench/go.mod |
| 123 | 1.00 | 0 | 123 | 1313 | README.md section #1 |
| 108 | 0.15 | 8 | 712 | 1682 | go decl names surface in xxhash.go |
| 82 | 1.00 | 0 | 82 | 1546 | go decl body at xxhash_unsafe.go:45 |
| 55 | 1.00 | 55 | 55 | 1436 | go package + imports in dynamic/plugin.go |
| 54 | 1.00 | 54 | 54 | 1628 | go decl body at dynamic/plugin.go:19 |
| 52 | 0.67 | 52 | 78 | 900 | go decl names surface in dynamic/plugin.go |
