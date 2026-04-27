scores: Sim=0.508 Reached=22/40 Early=3 Late=9 Partial=7 Missing=11 Used=6732/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 5 | 4 | 1 | 0 | 0.82 |
| 2 | 7 | 4 | 3 | 0 | 0.87 |
| 3 | 5 | 3 | 0 | 2 | 0.57 |
| 4 | 14 | 9 | 3 | 2 | 0.79 |
| 5 | 8 | 1 | 0 | 7 | 0.12 |
| 6 | 1 | 1 | 0 | 0 | 0.82 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 145 | 189 | +44 | 1.00 | late | README — title and one-sentence positioning | README headline in README.md (t=189, 4 atoms) |
| 1.3 | 221 | — | — | 0.50 | partial | Package doc + module path | go module file go.mod (t=112, 2 atoms) |
| 1.4 | 387 | 1209 | +822 | 0.81 | late | README — public API code-fence sketch | README.md section #0 (t=1209, 13 atoms) |
| 1.5 | 463 | 1209 | +746 | 0.80 | late | README — purego/asm note | README.md section #0 (t=1209, 4 atoms) |
| 2.1 | 603 | 2792 | +2189 | 1.00 | late | Digest struct + zero-value caveat | go decl at xxhash.go:29 (t=2704, 9 atoms) |
| 2.2 | 712 | — | — | 0.73 | partial | Constructors — New and NewWithSeed | go decl names surface in xxhash.go (t=2394, 4 atoms) |
| 2.3 | 907 | 2881 | +1974 | 0.81 | late | Reset / ResetWithSeed — full bodies | go decl body at xxhash.go:59 (t=2881, 6 atoms) |
| 2.4 | 1077 | 2467 | +1390 | 1.00 | late | Remaining Digest methods — Size, BlockSize, Write, Sum, Sum64 signatures | go decl body at xxhash.go:75 (t=4409, 28 atoms) |
| 2.5 | 1196 | — | — | 0.78 | partial | Sum64 / writeBlocks signatures (asm build) | go decl names surface in xxhash_asm.go (t=306, 3 atoms) |
| 2.6 | 1341 | — | — | 0.75 | partial | Sum64String / WriteString signatures (unsafe build) | go package + imports in xxhash_unsafe.go (t=653, 4 atoms) |
| 2.7 | 1460 | 2515 | +1055 | 1.00 | late | MarshalBinary / UnmarshalBinary signatures + magic constants | go decl body at xxhash.go:176 (t=3440, 10 atoms) |
| 3.1 | 1507 | 273 | -1234 | 1.00 | early | Sub-directory listings (.github, dynamic, xxhsum, xxhashbench) |  |
| 3.2 | 1648 | 6440 | +4792 | 1.00 | late | All test/benchmark function names across the repo | go test names surface in xxhash_test.go (t=6440, 9 atoms) |
| 3.3 | 1765 | — | — | 0.00 | missing | testall.sh — full body |  |
| 3.4 | 2081 | 6217 | +4136 | 0.88 | late | xxhsum CLI — main + usage | go decl body at xxhsum/xxhsum.go:11 (t=6217, 20 atoms) |
| 3.5 | 2400 | — | — | 0.00 | missing | CI matrix — what configurations are tested |  |
| 4.1 | 2592 | 3008 | +416 | 0.92 | aligned | Prime constants + primes array | go decl names surface in xxhash.go (t=2394, 7 atoms) |
| 4.2 | 2716 | — | — | 0.77 | partial | round + mergeRound — the core mixer | go decl names surface in xxhash.go (t=2394, 4 atoms) |
| 4.4 | 3084 | — | — | 0.69 | partial | Little-endian / append / consume byte helpers | go decl names surface in xxhash.go (t=2394, 6 atoms) |
| 4.5 | 3473 | 4409 | +936 | 0.81 | aligned | Write — streaming entry body | go decl body at xxhash.go:75 (t=4409, 28 atoms) |
| 4.6 | 3949 | 5065 | +1116 | 0.85 | aligned | Digest.Sum64 — finalize body | go decl body at xxhash.go:129 (t=5065, 33 atoms) |
| 4.7 | 4062 | 3153 | -909 | 0.93 | aligned | Digest.Sum — append big-endian bytes | go decl body at xxhash.go:113 (t=3153, 12 atoms) |
| 4.8 | 4212 | 3440 | -772 | 0.92 | aligned | MarshalBinary body — wire format | go decl body at xxhash.go:176 (t=3440, 10 atoms) |
| 4.9 | 4421 | 3984 | -437 | 0.94 | aligned | UnmarshalBinary body — validation + parse | go decl body at xxhash.go:190 (t=3984, 15 atoms) |
| 4.10 | 5119 | 5986 | +867 | 0.88 | aligned | Pure-Go Sum64 body (xxhash_other.go) | go decl body at xxhash_other.go:7 (t=5986, 48 atoms) |
| 4.11 | 5355 | 4621 | -734 | 0.92 | aligned | Pure-Go writeBlocks body | go decl body at xxhash_other.go:64 (t=4621, 11 atoms) |
| 4.12 | 5535 | — | — | 0.44 | missing | xxhash_safe.go full body — the appengine fallback | go decl names surface in xxhash_safe.go (t=382, 4 atoms) |
| 4.13 | 5940 | — | — | 0.21 | missing | Unsafe string conversion — inlining commentary + sliceHeader | go decl names surface in xxhash_unsafe.go (t=452, 6 atoms) |
| 4.14 | 6070 | — | — | 0.70 | partial | Unsafe Sum64String / WriteString bodies | go decl body at xxhash_unsafe.go:45 (t=1628, 5 atoms) |
| 5.1 | 6391 | 3782 | -2609 | 0.85 | early | README — Benchmarks section (perf table) | README.md section #2 (t=3782, 16 atoms) |
| 5.2 | 6724 | — | — | 0.05 | missing | TestAll — known-answer test vectors | go test names surface in xxhash_test.go (t=6440, 2 atoms) |
| 5.3 | 7016 | — | — | 0.03 | missing | bench_test.go — input-size grid + benchmark shape | go test names surface in bench_test.go (t=6359, 2 atoms) |
| 5.4 | 7396 | — | — | 0.03 | missing | xxhashbench — structure of the cross-library benchmark | go test names surface in xxhashbench/xxhashbench_test.go (t=6232, 2 atoms) |
| 5.5 | 8093 | — | — | 0.00 | missing | amd64 asm — register defs and macros |  |
| 5.6 | 8652 | — | — | 0.00 | missing | amd64 asm — Sum64 prologue, state setup, and main block loop |  |
| 5.7 | 9358 | — | — | 0.00 | missing | amd64 asm — tail handlers (loop8 / try4 / try1) + finalize |  |
| 5.8 | 9801 | — | — | 0.00 | missing | amd64 asm — writeBlocks body |  |
| 6.1 | 9946 | 1436 | -8510 | 0.82 | early | README — Compatibility (supported Go versions) | README.md section #1 (t=1436, 8 atoms) |

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
