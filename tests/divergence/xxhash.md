scores: Sim=0.508 Reached=22/40 Early=3 Late=9 Partial=7 Missing=11 Used=6732/10000

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 0 ranking-recoverable (w×gap=0.00), 12 wrong-slice/granularity (w×gap=1.19), 6 no-discovered (w×gap=0.76)
Secondary intervention: split wrong-slice batches for 12 rows
Loss reasons: 0 predecessor-gated, 0 too-expensive, 0 discovered-unscheduled
Top rows: 1.3, 2.2, 2.6, 2.5, 4.4, ...
Note: likely lever is heuristic; verify `Sim` moves, not just bucket counts.

## Top opportunities

_`w(t)×gap` is a non-additive priority score: Σ exp(-exp_t/τ) × (1 - credit) per row, τ=2000. Same time weighting and credit gap as Sim, but rows can overlap between opportunities — sums across rows are an upper bound on Sim impact, not an additive estimate._

| intervention | rows | w(t)×gap | bands ≤3k/≤6k/total | evidence | top row ids |
|:-------------|-----:|---------:|:----------------------|:---------|:------------|
| split wrong-slice walker batches | 12 | 1.19 | 5/8/12 | nearby candidates have low exact atom overlap | 1.3, 2.2, 2.6, 2.5, 4.4, ... |
| add walker candidates for no-discovered rows | 6 | 0.76 | 2/2/6 | NS rows have no discovered line candidate | 3.3, 3.5, 5.5, 5.6, 5.7, ... |

Tiers: 1=4/5 reached, 1 partial, 0 missing, avg=0.82; 2=4/7 reached, 3 partial, 0 missing, avg=0.87; 3=3/5 reached, 0 partial, 2 missing, avg=0.57; 4=9/14 reached, 3 partial, 2 missing, avg=0.79; 5=1/8 reached, 0 partial, 7 missing, avg=0.12; 6=1/1 reached, 0 partial, 0 missing, avg=0.82

## Diagnosis rollup

| diagnosis | rows | missing | partial | timing | likely lever |
|:----------|-----:|--------:|--------:|-------:|:-------------|
| wrong-slice / granularity | 12 | 5 | 7 | 0 | walker granularity / wrong slice |
| no discovered candidate | 6 | 6 | 0 | 0 | walker coverage or predecessor-gated emit |
| timing-only | 20 | 0 | 0 | 20 | usually no code change |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=31, fs-only=1, no discovered candidate=6

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | aligned | low | 3 |
| scheduled bbox | aligned | high | 5 |
| scheduled bbox | early | low | 1 |
| scheduled bbox | early | high | 1 |
| scheduled bbox | late | none | 2 |
| scheduled bbox | late | low | 4 |
| scheduled bbox | late | high | 2 |
| scheduled bbox | late | full | 1 |
| scheduled bbox | missing | low | 5 |
| scheduled bbox | partial | none | 1 |
| scheduled bbox | partial | low | 6 |

## Arrival ledger by diagnosis

### wrong-slice / granularity

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.3 | 221 | — | — | 0.50 | partial | Package doc + module path | [scheduled bbox exact=2/6] go module file go.mod (t=112, 2 atoms) |
| 2.2 | 712 | — | — | 0.73 | partial | Constructors — New and NewWithSeed | [scheduled bbox exact=4/11] go decl names surface in xxhash.go (t=2394, 4 atoms) |
| 2.5 | 1196 | — | — | 0.78 | partial | Sum64 / writeBlocks signatures (asm build) | [scheduled bbox exact=3/9] go decl doc at xxhash_asm.go:12 (t=626, 3 atoms) |
| 2.6 | 1341 | — | — | 0.75 | partial | Sum64String / WriteString signatures (unsafe build) | [scheduled bbox exact=0/8] go package + imports in xxhash_unsafe.go (t=653, 4 atoms) |
| 4.2 | 2716 | — | — | 0.77 | partial | round + mergeRound — the core mixer | [scheduled bbox exact=4/13] go decl body at xxhash.go:229 (t=3047, 4 atoms) |
| 4.4 | 3084 | — | — | 0.69 | partial | Little-endian / append / consume byte helpers | [scheduled bbox exact=6/13] go decl names surface in xxhash.go (t=2394, 6 atoms) |
| 4.12 | 5535 | — | — | 0.44 | missing | xxhash_safe.go full body — the appengine fallback | [scheduled bbox exact=4/16] go decl names surface in xxhash_safe.go (t=382, 4 atoms) |
| 4.13 | 5940 | — | — | 0.21 | missing | Unsafe string conversion — inlining commentary + sliceHeader | [scheduled bbox exact=2/28] go decl names surface in xxhash_unsafe.go (t=452, 6 atoms) |
| 4.14 | 6070 | — | — | 0.70 | partial | Unsafe Sum64String / WriteString bodies | [scheduled bbox exact=5/10] go decl body at xxhash_unsafe.go:45 (t=1628, 5 atoms) |
| 5.2 | 6724 | — | — | 0.05 | missing | TestAll — known-answer test vectors | [scheduled bbox exact=2/19] go test names surface in xxhash_test.go (t=6440, 2 atoms) |
| 5.3 | 7016 | — | — | 0.03 | missing | bench_test.go — input-size grid + benchmark shape | [scheduled bbox exact=2/32] go test names surface in bench_test.go (t=6359, 2 atoms) |
| 5.4 | 7396 | — | — | 0.03 | missing | xxhashbench — structure of the cross-library benchmark | [scheduled bbox exact=2/35] go test names surface in xxhashbench/xxhashbench_test.go (t=6232, 2 atoms) |

### no discovered candidate

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 3.3 | 1765 | — | — | 0.00 | missing | testall.sh — full body | no discovered line candidate |
| 3.5 | 2400 | — | — | 0.00 | missing | CI matrix — what configurations are tested | no discovered line candidate |
| 5.5 | 8093 | — | — | 0.00 | missing | amd64 asm — register defs and macros | no discovered line candidate |
| 5.6 | 8652 | — | — | 0.00 | missing | amd64 asm — Sum64 prologue, state setup, and main block loop | no discovered line candidate |
| 5.7 | 9358 | — | — | 0.00 | missing | amd64 asm — tail handlers (loop8 / try4 / try1) + finalize | no discovered line candidate |
| 5.8 | 9801 | — | — | 0.00 | missing | amd64 asm — writeBlocks body | no discovered line candidate |

### timing-only

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 145 | 189 | +44 | 1.00 | late | README — title and one-sentence positioning | [scheduled bbox exact=4/4] README headline in README.md (t=189, 4 atoms) |
| 1.4 | 387 | 1209 | +822 | 0.81 | late | README — public API code-fence sketch | [scheduled bbox exact=13/16] README.md section #0 (t=1209, 13 atoms) |
| 1.5 | 463 | 1209 | +746 | 0.80 | late | README — purego/asm note | [scheduled bbox exact=4/5] README.md section #0 (t=1209, 4 atoms) |
| 2.1 | 603 | 2792 | +2189 | 1.00 | late | Digest struct + zero-value caveat | [scheduled bbox exact=9/13] go decl at xxhash.go:29 (t=2704, 9 atoms) |
| 2.3 | 907 | 2881 | +1974 | 0.81 | late | Reset / ResetWithSeed — full bodies | [scheduled bbox exact=6/16] go decl body at xxhash.go:59 (t=2881, 6 atoms) |
| 2.4 | 1077 | 2467 | +1390 | 1.00 | late | Remaining Digest methods — Size, BlockSize, Write, Sum, Sum64 signatures | [scheduled bbox exact=0/10] go decl body at xxhash.go:75 (t=4409, 28 atoms) |
| 2.7 | 1460 | 2515 | +1055 | 1.00 | late | MarshalBinary / UnmarshalBinary signatures + magic constants | [scheduled bbox exact=0/8] go decl body at xxhash.go:176 (t=3440, 10 atoms) |
| 3.1 | 1507 | 273 | -1234 | 1.00 | early | Sub-directory listings (.github, dynamic, xxhsum, xxhashbench) | fs-only |
| 3.2 | 1648 | 6440 | +4792 | 1.00 | late | All test/benchmark function names across the repo | [scheduled bbox exact=5/16] go test names surface in xxhash_test.go (t=6440, 9 atoms) |
| 3.4 | 2081 | 6217 | +4136 | 0.88 | late | xxhsum CLI — main + usage | [scheduled bbox exact=20/32] go decl body at xxhsum/xxhsum.go:11 (t=6217, 20 atoms) |
| 4.1 | 2592 | 3008 | +416 | 0.92 | aligned | Prime constants + primes array | [scheduled bbox exact=7/13] go decl at xxhash.go:11 (t=2421, 7 atoms) |
| 4.5 | 3473 | 4409 | +936 | 0.81 | aligned | Write — streaming entry body | [scheduled bbox exact=28/37] go decl body at xxhash.go:75 (t=4409, 28 atoms) |
| 4.6 | 3949 | 5065 | +1116 | 0.85 | aligned | Digest.Sum64 — finalize body | [scheduled bbox exact=33/41] go decl body at xxhash.go:129 (t=5065, 33 atoms) |
| 4.7 | 4062 | 3153 | -909 | 0.93 | aligned | Digest.Sum — append big-endian bytes | [scheduled bbox exact=12/15] go decl body at xxhash.go:113 (t=3153, 12 atoms) |
| 4.8 | 4212 | 3440 | -772 | 0.92 | aligned | MarshalBinary body — wire format | [scheduled bbox exact=10/13] go decl body at xxhash.go:176 (t=3440, 10 atoms) |
| 4.9 | 4421 | 3984 | -437 | 0.94 | aligned | UnmarshalBinary body — validation + parse | [scheduled bbox exact=15/18] go decl body at xxhash.go:190 (t=3984, 15 atoms) |
| 4.10 | 5119 | 5986 | +867 | 0.88 | aligned | Pure-Go Sum64 body (xxhash_other.go) | [scheduled bbox exact=48/56] go decl body at xxhash_other.go:7 (t=5986, 48 atoms) |
| 4.11 | 5355 | 4621 | -734 | 0.92 | aligned | Pure-Go writeBlocks body | [scheduled bbox exact=11/13] go decl body at xxhash_other.go:64 (t=4621, 11 atoms) |
| 5.1 | 6391 | 3782 | -2609 | 0.85 | early | README — Benchmarks section (perf table) | [scheduled bbox exact=16/20] README.md section #2 (t=3782, 16 atoms) |
| 6.1 | 9946 | 1436 | -8510 | 0.82 | early | README — Compatibility (supported Go versions) | [scheduled bbox exact=8/11] README.md section #1 (t=1436, 8 atoms) |

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
