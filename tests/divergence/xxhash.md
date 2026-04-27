scores: Sim=0.406 Reached=7/40 Early=3 Late=0 Partial=0 Missing=33 Used=1314/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 5 | 4 | 0 | 1 | 0.72 |
| 2 | 7 | 0 | 0 | 7 | 0.00 |
| 3 | 5 | 1 | 0 | 4 | 0.20 |
| 4 | 14 | 0 | 0 | 14 | 0.00 |
| 5 | 8 | 1 | 0 | 7 | 0.11 |
| 6 | 1 | 1 | 0 | 0 | 0.82 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.3 | 221 | — | — | 0.00 | missing | Package doc + module path |  |
| 1.4 | 387 | 445 | +58 | 0.81 | aligned | README — public API code-fence sketch | README.md section #0 (t=445, 13 atoms) |
| 1.5 | 463 | 445 | -18 | 0.80 | aligned | README — purego/asm note | README.md section #0 (t=445, 4 atoms) |
| 2.1 | 603 | — | — | 0.00 | missing | Digest struct + zero-value caveat |  |
| 2.2 | 712 | — | — | 0.00 | missing | Constructors — New and NewWithSeed |  |
| 2.3 | 907 | — | — | 0.00 | missing | Reset / ResetWithSeed — full bodies |  |
| 2.4 | 1077 | — | — | 0.00 | missing | Remaining Digest methods — Size, BlockSize, Write, Sum, Sum64 signatures |  |
| 2.5 | 1196 | — | — | 0.00 | missing | Sum64 / writeBlocks signatures (asm build) |  |
| 2.6 | 1341 | — | — | 0.00 | missing | Sum64String / WriteString signatures (unsafe build) |  |
| 2.7 | 1460 | — | — | 0.00 | missing | MarshalBinary / UnmarshalBinary signatures + magic constants |  |
| 3.1 | 1507 | 223 | -1284 | 1.00 | early | Sub-directory listings (.github, dynamic, xxhsum, xxhashbench) |  |
| 3.2 | 1648 | — | — | 0.00 | missing | All test/benchmark function names across the repo |  |
| 3.3 | 1765 | — | — | 0.00 | missing | testall.sh — full body |  |
| 3.4 | 2081 | — | — | 0.00 | missing | xxhsum CLI — main + usage |  |
| 3.5 | 2400 | — | — | 0.00 | missing | CI matrix — what configurations are tested |  |
| 4.1 | 2592 | — | — | 0.00 | missing | Prime constants + primes array |  |
| 4.2 | 2716 | — | — | 0.00 | missing | round + mergeRound — the core mixer |  |
| 4.3 | 2918 | — | — | 0.00 | missing | rol* one-line helpers (locations) |  |
| 4.4 | 3084 | — | — | 0.00 | missing | Little-endian / append / consume byte helpers |  |
| 4.5 | 3473 | — | — | 0.00 | missing | Write — streaming entry body |  |
| 4.6 | 3949 | — | — | 0.00 | missing | Digest.Sum64 — finalize body |  |
| 4.7 | 4062 | — | — | 0.00 | missing | Digest.Sum — append big-endian bytes |  |
| 4.8 | 4212 | — | — | 0.00 | missing | MarshalBinary body — wire format |  |
| 4.9 | 4421 | — | — | 0.00 | missing | UnmarshalBinary body — validation + parse |  |
| 4.10 | 5119 | — | — | 0.00 | missing | Pure-Go Sum64 body (xxhash_other.go) |  |
| 4.11 | 5355 | — | — | 0.00 | missing | Pure-Go writeBlocks body |  |
| 4.12 | 5535 | — | — | 0.00 | missing | xxhash_safe.go full body — the appengine fallback |  |
| 4.13 | 5940 | — | — | 0.00 | missing | Unsafe string conversion — inlining commentary + sliceHeader |  |
| 4.14 | 6070 | — | — | 0.00 | missing | Unsafe Sum64String / WriteString bodies |  |
| 5.1 | 6391 | 1022 | -5369 | 0.85 | early | README — Benchmarks section (perf table) | README.md section #2 (t=1022, 16 atoms) |
| 5.2 | 6724 | — | — | 0.00 | missing | TestAll — known-answer test vectors |  |
| 5.3 | 7016 | — | — | 0.00 | missing | bench_test.go — input-size grid + benchmark shape |  |
| 5.4 | 7396 | — | — | 0.00 | missing | xxhashbench — structure of the cross-library benchmark |  |
| 5.5 | 8093 | — | — | 0.00 | missing | amd64 asm — register defs and macros |  |
| 5.6 | 8652 | — | — | 0.00 | missing | amd64 asm — Sum64 prologue, state setup, and main block loop |  |
| 5.7 | 9358 | — | — | 0.00 | missing | amd64 asm — tail handlers (loop8 / try4 / try1) + finalize |  |
| 5.8 | 9801 | — | — | 0.00 | missing | amd64 asm — writeBlocks body |  |
| 6.1 | 9946 | 585 | -9361 | 0.82 | early | README — Compatibility (supported Go versions) | README.md section #1 (t=585, 8 atoms) |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 292 | 1.00 | 292 | 1314 | plaintext config LICENSE.txt |
| 144 | 1.00 | 144 | 729 | README.md section #3 |
