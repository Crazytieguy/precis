scores: Score(3000)=0.658 ns_rows≤3K=20/40 (reached=9 partial=4 missing=7)

## Per-budget scores

| B | A_B | I(B) | C(B) | Score(B) | walker_used |
|--:|----:|-----:|-----:|---------:|------------:|
| 1000 | 90 | 0.775 | 0.261 | 0.450 | 987 |
| 1442 | 117 | 0.823 | 0.436 | 0.599 | 1436 |
| 2080 | 161 | 0.780 | 0.380 | 0.545 | 1682 |
| 3000 | 257 | 0.849 | 0.511 | 0.658 | 2953 |
| 4327 | 376 | 0.829 | 0.477 | 0.629 | 4057 |
| 6240 | 517 | 0.864 | 0.697 | 0.776 | 6232 |
| 9000 | 728 | 0.838 | 0.540 | 0.673 | 6732 |

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 0 ranking-recoverable (gap@3k=0.00), 17 wrong-slice/granularity (gap@3k=1.06), 6 no-discovered (gap@3k=0.49)
Secondary intervention: investigate 6 no-discovered rows
Top rows: 1.3, 3.4, 4.5, 3.2, 5.4, ...

## Top opportunities

_`gap@B` is a non-additive priority score: `Σ over atoms in row: (1 − damped_credit(a)) / rank(a)` evaluated at budget B's walker state. Approximates how much closing the row would lift `Score(B)` (via the Importance numerator); not an exact delta. `gap@3k` is the primary sort key. `gap@1k` and `gap@9k` show how the same intervention scales across the budget vector — gap is monotone non-increasing in B (walker has more budget at higher B). Rows can overlap between opportunities; sums are upper bounds on Score(B) impact, not additive estimates._

| intervention | rows | gap@1k | gap@3k | gap@9k | evidence | top row ids |
|:-------------|-----:|-------:|-------:|-------:|:---------|:------------|
| split wrong-slice walker batches | 17 | 1.29 | 1.06 | 0.61 | nearby candidates have low exact atom overlap | 1.3, 3.4, 4.5, 3.2, 5.4, ... |
| add walker candidates for no-discovered rows | 6 | 0.49 | 0.49 | 0.49 | NS rows have no discovered line candidate | 3.5, 5.5, 5.7, 5.6, 3.3, ... |
| finish partially-delivered NS batches | 8 | 0.51 | 0.44 | 0.28 | avg batch completion=0.50 | 1.3, 4.9, 2.5, 4.7, 4.2, ... |

## Diagnosis rollup

| diagnosis | rows | missing | partial | likely lever |
|:----------|-----:|--------:|--------:|:-------------|
| wrong-slice / granularity | 17 | 12 | 5 | walker granularity / wrong slice |
| no discovered candidate | 6 | 6 | 0 | walker coverage or predecessor-gated emit |
| mixed/unknown | 7 | 6 | 1 | inspect row |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=24, no discovered candidate=6

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | missing | low | 12 |
| scheduled bbox | missing | high | 6 |
| scheduled bbox | partial | none | 1 |
| scheduled bbox | partial | low | 4 |
| scheduled bbox | partial | high | 1 |

## Arrival ledger by diagnosis

### wrong-slice / granularity

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 1.3 | 221 | 0.50 | 0.35 | missing | Package doc + module path | [scheduled bbox exact=2/6] go module file go.mod (t=112, 2 atoms) |
| 2.2 | 712 | 0.73 | 0.99 | partial | Constructors — New and NewWithSeed | [scheduled bbox exact=4/11] go decl names surface in xxhash.go (t=2394, 4 atoms) |
| 2.5 | 1196 | 0.78 | 0.67 | partial | Sum64 / writeBlocks signatures (asm build) | [scheduled bbox exact=3/9] go decl doc at xxhash_asm.go:12 (t=626, 3 atoms) |
| 2.6 | 1341 | 0.75 | 0.90 | partial | Sum64String / WriteString signatures (unsafe build) | [scheduled bbox exact=0/8] go package + imports in xxhash_unsafe.go (t=653, 4 atoms) |
| 3.2 | 1648 | 0.12 | 0.09 | missing | All test/benchmark function names across the repo | [scheduled bbox exact=5/16] go test names surface in xxhash_test.go (t=6440, 9 atoms) |
| 3.4 | 2081 | 0.25 | 0.16 | missing | xxhsum CLI — main + usage | [scheduled bbox exact=20/32] go decl body at xxhsum/xxhsum.go:11 (t=6217, 20 atoms) |
| 4.1 | 2592 | 0.62 | 0.61 | missing | Prime constants + primes array | [scheduled bbox exact=7/13] go decl at xxhash.go:11 (t=2421, 7 atoms) |
| 4.2 | 2716 | 0.47 | 0.67 | missing | round + mergeRound — the core mixer | [scheduled bbox exact=4/13] go decl body at xxhash.go:229 (t=3047, 4 atoms) |
| 4.4 | 3084 | 0.69 | 0.99 | partial | Little-endian / append / consume byte helpers | [scheduled bbox exact=6/13] go decl names surface in xxhash.go (t=2394, 6 atoms) |
| 4.5 | 3473 | 0.06 | 0.16 | missing | Write — streaming entry body | [scheduled bbox exact=28/37] go decl body at xxhash.go:75 (t=4409, 28 atoms) |
| 4.8 | 4212 | 0.16 | 0.31 | missing | MarshalBinary body — wire format | [scheduled bbox exact=10/13] go decl body at xxhash.go:176 (t=3440, 10 atoms) |
| 4.12 | 5535 | 0.44 | 0.71 | missing | xxhash_safe.go full body — the appengine fallback | [scheduled bbox exact=4/16] go decl names surface in xxhash_safe.go (t=382, 4 atoms) |
| 4.13 | 5940 | 0.21 | 0.17 | missing | Unsafe string conversion — inlining commentary + sliceHeader | [scheduled bbox exact=2/28] go decl names surface in xxhash_unsafe.go (t=452, 6 atoms) |
| 4.14 | 6070 | 0.70 | 0.99 | partial | Unsafe Sum64String / WriteString bodies | [scheduled bbox exact=5/10] go decl body at xxhash_unsafe.go:45 (t=1628, 5 atoms) |
| 5.2 | 6724 | 0.00 | 0.00 | missing | TestAll — known-answer test vectors | [scheduled bbox exact=2/19] go test names surface in xxhash_test.go (t=6440, 2 atoms) |
| 5.3 | 7016 | 0.00 | 0.00 | missing | bench_test.go — input-size grid + benchmark shape | [scheduled bbox exact=2/32] go test names surface in bench_test.go (t=6359, 2 atoms) |
| 5.4 | 7396 | 0.00 | 0.00 | missing | xxhashbench — structure of the cross-library benchmark | [scheduled bbox exact=2/35] go test names surface in xxhashbench/xxhashbench_test.go (t=6232, 2 atoms) |

### no discovered candidate

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 3.3 | 1765 | 0.00 | 0.00 | missing | testall.sh — full body | no discovered line candidate |
| 3.5 | 2400 | 0.00 | 0.00 | missing | CI matrix — what configurations are tested | no discovered line candidate |
| 5.5 | 8093 | 0.00 | 0.00 | missing | amd64 asm — register defs and macros | no discovered line candidate |
| 5.6 | 8652 | 0.00 | 0.00 | missing | amd64 asm — Sum64 prologue, state setup, and main block loop | no discovered line candidate |
| 5.7 | 9358 | 0.00 | 0.00 | missing | amd64 asm — tail handlers (loop8 / try4 / try1) + finalize | no discovered line candidate |
| 5.8 | 9801 | 0.00 | 0.00 | missing | amd64 asm — writeBlocks body | no discovered line candidate |

### mixed/unknown

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 1.5 | 463 | 0.80 | 1.00 | partial | README — purego/asm note | [scheduled bbox exact=4/5] README.md section #0 (t=1209, 4 atoms) |
| 4.6 | 3949 | 0.05 | 0.10 | missing | Digest.Sum64 — finalize body | [scheduled bbox exact=33/41] go decl body at xxhash.go:129 (t=5065, 33 atoms) |
| 4.7 | 4062 | 0.14 | 0.43 | missing | Digest.Sum — append big-endian bytes | [scheduled bbox exact=12/15] go decl body at xxhash.go:113 (t=3153, 12 atoms) |
| 4.9 | 4421 | 0.11 | 0.22 | missing | UnmarshalBinary body — validation + parse | [scheduled bbox exact=15/18] go decl body at xxhash.go:190 (t=3984, 15 atoms) |
| 4.10 | 5119 | 0.02 | 0.03 | missing | Pure-Go Sum64 body (xxhash_other.go) | [scheduled bbox exact=48/56] go decl body at xxhash_other.go:7 (t=5986, 48 atoms) |
| 4.11 | 5355 | 0.08 | 0.13 | missing | Pure-Go writeBlocks body | [scheduled bbox exact=11/13] go decl body at xxhash_other.go:64 (t=4621, 11 atoms) |
| 5.1 | 6391 | 0.10 | 0.02 | missing | README — Benchmarks section (perf table) | [scheduled bbox exact=16/20] README.md section #2 (t=3782, 16 atoms) |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 292 | 1.00 | 292 | 6732 | plaintext config LICENSE.txt |
| 274 | 1.00 | 274 | 5339 | go decl body at dynamic/plugin.go:26 |
| 144 | 1.00 | 144 | 3297 | README.md section #3 |
| 133 | 1.00 | 133 | 900 | go module file xxhashbench/go.mod |
| 73 | 1.00 | 73 | 4057 | go decl body at xxhsum/xxhsum.go:43 |
| 55 | 1.00 | 55 | 1491 | go package + imports in dynamic/plugin.go |
| 54 | 1.00 | 54 | 1682 | go decl body at dynamic/plugin.go:19 |
| 52 | 0.67 | 78 | 978 | go decl names surface in dynamic/plugin.go |
