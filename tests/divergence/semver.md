scores: Score(3000)=0.654 ns_rows≤3K=14/40 (reached=8 partial=1 missing=5)

## Per-budget scores

| B | A_B | I(B) | C(B) | compl(B) | Score(B) | walker_used |
|--:|----:|-----:|-----:|---------:|---------:|------------:|
| 1000 | 100 | 0.692 | 0.539 | 0.874 | 0.611 | 963 |
| 1442 | 163 | 0.730 | 0.552 | 0.878 | 0.635 | 1435 |
| 2080 | 194 | 0.715 | 0.547 | 0.856 | 0.625 | 2050 |
| 3000 | 263 | 0.735 | 0.582 | 0.796 | 0.654 | 2969 |
| 4327 | 317 | 0.724 | 0.535 | 0.854 | 0.622 | 4272 |
| 6240 | 446 | 0.688 | 0.391 | 0.764 | 0.519 | 6109 |
| 9000 | 701 | 0.677 | 0.330 | 0.924 | 0.473 | 8856 |

## Top opportunities

### Additive (close partial / missing rows)

| intervention | rows | gap@1k | gap@3k | gap@9k | evidence | top row ids |
|:-------------|-----:|-------:|-------:|-------:|:---------|:------------|
| split wrong-slice walker batches | 27 | 2.41 | 2.41 | 2.10 | nearby candidates have low exact atom overlap | 1.1, 3.2, 3.3, 3.7, 5.1, ... |
| tune ranking for high-overlap unscheduled candidates | 2 | 0.15 | 0.15 | 0.15 | high-overlap candidates not in the schedule by T_max, exact total=23/26 | 2.3, 3.6 |
| add walker candidates for no-discovered rows | 1 | 0.15 | 0.15 | 0.15 | NS rows have no discovered line candidate | 2.4 |
| promote export names surface in classes/semver.js | 1 | 0.02 | 0.02 | 0.02 | 0 files, exact total=9/9 | 4.1 |
| promote export names surface in classes/range.js | 1 | 0.02 | 0.02 | 0.02 | 0 files, exact total=8/8 | 4.2 |

### Subtractive (suppress consistently off-NS batches)

| pattern | batches | freed@1k | freed@3k | freed@9k | evidence | top batch ids |
|:--------|--------:|---------:|---------:|---------:|:---------|:--------------|
| README.md section #<n> | 3 | 0 | 180 | 1603 | off_3k=180 | README.md section #27, README.md section #24, README.md section #29 |
| package scripts in package.json | 1 | 144 | 144 | 144 | off_3k=144 | package scripts in package.json |
| package entrypoints in package.json | 1 | 92 | 92 | 92 | off_3k=92 | package entrypoints in package.json |
| package dependencies in package.json | 1 | 0 | 84 | 84 | off_3k=84 | package dependencies in package.json |
| CONTRIBUTING.md section #<n> | 1 | 0 | 63 | 193 | off_3k=63 | CONTRIBUTING.md section #2 |

Top missed paths (NS rows ≤ 3K): README.md (4 rows, 101 atoms), range.bnf (1 row, 16 atoms), package.json (1 row, 5 atoms)

## Arrival ledger by diagnosis

### ranking-recoverable

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 2.3 | 750 | 0.00 | missing | README — Usage example (calls into the public API) | [unscheduled bbox exact=9/9] README.md section #2 (9 atoms, too expensive at final margin) |
| 3.6 | 4025 | 0.00 | missing | README — Tilde Ranges desugaring | [unscheduled bbox exact=14/17] README.md section #16 (14 atoms, too expensive at final margin) |
| 4.1 | 5749 | 0.00 | missing | SemVer class — method signatures (locations) | [unscheduled bbox exact=9/9] export at classes/semver.js:9 (16 atoms, predecessor not scheduled: export names surface in classes/semver.js) |
| 4.2 | 5809 | 0.00 | missing | Range class — method signatures (locations) | [unscheduled bbox exact=8/8] export at classes/range.js:6 (14 atoms, predecessor not scheduled: export names surface in classes/range.js) |

### wrong-slice / granularity

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 1.1 | 61 | 0.80 | partial | package.json — name, description, main | [scheduled bbox exact=3/5] package identity in package.json (t=268, 3 atoms) |
| 2.6 | 1667 | 0.74 | missing | README — all section heading locations | [scheduled bbox exact=1/23] README.md section #19 (t=8642, 40 atoms) |
| 3.2 | 2375 | 0.06 | missing | README — Ranges intro (operators, comparator sets, ||) | [scheduled bbox exact=27/35] README.md section #4 (t=6669, 27 atoms) |
| 3.3 | 2947 | 0.06 | missing | README — Prerelease Tags semantics | [scheduled bbox exact=7/34] README.md section #6 (t=7028, 7 atoms) |
| 3.4 | 3219 | 0.00 | missing | README — Hyphen Ranges desugaring | [scheduled bbox exact=12/19] README.md section #14 (t=9111, 12 atoms) |
| 3.5 | 3533 | 0.00 | missing | README — X-Ranges desugaring | [unscheduled bbox exact=13/18] README.md section #15 (13 atoms, too expensive at final margin) |
| 3.7 | 4873 | 0.00 | missing | README — Caret Ranges desugaring (the most complex) | [unscheduled bbox exact=34/43] README.md section #17 (34 atoms, too expensive at final margin) |
| 3.8 | 5124 | 0.11 | missing | README — Functions section preface (options doc) | [scheduled bbox exact=15/19] README.md section #19 (t=8642, 15 atoms) |
| 3.9 | 5681 | 0.12 | missing | README — Coercion semantics | [scheduled bbox exact=5/26] README.md section #49 (t=4831, 5 atoms); better unscheduled exact=12/26: README.md section #48 (12 atoms, too expensive at final margin) |
| 4.3 | 5992 | 0.00 | missing | Range module — internal helpers (locations) | [unscheduled same-file] export body at classes/range.js:6 body 8 (165 atoms, predecessor not scheduled: export at classes/range.js:6) |
| 4.4 | 6069 | 0.00 | missing | Comparator class — method signatures (locations) | [unscheduled bbox exact=7/9] export at classes/comparator.js:5 (12 atoms, predecessor not scheduled: export names surface in classes/comparator.js) |
| 4.5 | 6460 | 0.00 | missing | internal/constants.js — full file | [unscheduled bbox exact=10/37] export at internal/constants.js:28 (10 atoms, predecessor not scheduled: export names surface in internal/constants.js) |
| 5.1 | 7003 | 0.00 | missing | internal/re.js — every token name (locations) | [unscheduled bbox exact=0/42] export names surface in internal/re.js (6 atoms, discovered unscheduled) |
| 5.2 | 7475 | 0.00 | missing | internal/re.js — section comments and exports header | [unscheduled bbox exact=7/38] imports in internal/re.js (7 atoms, discovered unscheduled) |
| 5.3 | 7628 | 0.00 | missing | SemVer.compare — body (entry into compareMain || comparePre) | [unscheduled bbox exact=11/15] export body at classes/semver.js:9 body 11 (11 atoms, predecessor not scheduled: export at classes/semver.js:9) |
| 5.4 | 7934 | 0.00 | missing | internal/identifiers.js — full file | [unscheduled bbox exact=14/29] export body at internal/identifiers.js:4 body 5 (14 atoms, predecessor not scheduled: export at internal/identifiers.js:4) |
| 5.5 | 8148 | 0.00 | missing | bin/semver.js — CLI flag case lines (truncated) | [unscheduled same-file] imports in bin/semver.js (1 atoms, discovered unscheduled) |
| 6.1 | 8372 | 0.00 | missing | functions/ — module.exports lines (locations of every public function) | [unscheduled bbox exact=1/24] export at functions/valid.js:8 (1 atoms, predecessor not scheduled: export names surface in functions/valid.js) |
| 6.2 | 8490 | 0.00 | missing | ranges/ — module.exports lines (locations of every range function) | [unscheduled bbox exact=1/11] export at ranges/valid.js:13 (1 atoms, predecessor not scheduled: export names surface in ranges/valid.js) |
| 6.3 | 8897 | 0.00 | missing | functions/cmp.js — operator switch body | [unscheduled bbox exact=34/44] export body at functions/cmp.js:10 body 11 (34 atoms, predecessor not scheduled: export at functions/cmp.js:10) |
| 6.4 | 9449 | 0.00 | missing | functions/diff.js — release-type comparison body | [unscheduled bbox exact=37/49] export body at functions/diff.js:5 body 6 (42 atoms, predecessor not scheduled: export at functions/diff.js:5) |
| 7.1 | 9601 | 0.00 | missing | internal/parse-options.js — full file | [unscheduled bbox exact=7/17] export body at internal/parse-options.js:6 body 7 (7 atoms, predecessor not scheduled: export at internal/parse-options.js:6) |
| 7.2 | 9711 | 0.00 | missing | internal/debug.js — full file | [unscheduled bbox exact=1/11] export at internal/debug.js:11 (1 atoms, predecessor not scheduled: export names surface in internal/debug.js) |
| 7.3 | 9778 | 0.00 | missing | internal/lrucache.js — class signature + max constant | [unscheduled bbox exact=6/8] export at internal/lrucache.js:3 (8 atoms, predecessor not scheduled: export names surface in internal/lrucache.js) |
| 7.4 | 9852 | 0.00 | missing | ranges/min-version.js — function signature + 0.0.0 fast path | [unscheduled bbox exact=4/6] export body at ranges/min-version.js:7 body 8 (4 atoms, predecessor not scheduled: export at ranges/min-version.js:7) |
| 7.5 | 9924 | 0.88 | partial | LICENSE first line + CONTRIBUTING.md headings | [scheduled bbox exact=4/8] CONTRIBUTING.md section #3 (t=9441, 14 atoms) |
| 7.6 | 9977 | 0.00 | missing | bin/semver.js — entry skeleton (shebang, version load, main call) | [unscheduled bbox exact=0/5] imports in bin/semver.js (1 atoms, discovered unscheduled) |

### no discovered candidate

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 2.4 | 1011 | 0.00 | missing | range.bnf — canonical range grammar | no discovered line candidate |

Top wasted paths (off-NS at 3K): package.json (320t, 3 batches), README.md (180t, 3 batches), CONTRIBUTING.md (171t, 2 batches), test/ranges (56t, 1 batch)

## Walker waste rollup (by descriptor pattern)

| n | off_3k_total | pattern |
|--:|-------------:|:--------|
| 3 | 180 | README.md section #<n> |

## Walker waste (walker_t ≤ 3000, off_3k ≥ 50)

| off_3k | off_3k_ratio | off_any | cost | walker_t | batch |
|-------:|-------------:|--------:|-----:|---------:|:------|
| 144 | 1.00 | 144 | 144 | 963 | package scripts in package.json |
| 108 | 1.00 | 44 | 108 | 371 | headings outline in CONTRIBUTING.md |
| 92 | 0.68 | 92 | 135 | 787 | package entrypoints in package.json |
| 84 | 0.75 | 84 | 112 | 1107 | package dependencies in package.json |
| 71 | 1.00 | 71 | 71 | 2969 | README.md section #27 |
| 63 | 1.00 | 63 | 63 | 2050 | CONTRIBUTING.md section #2 |
| 57 | 1.00 | 57 | 57 | 2821 | README.md section #24 |
| 56 | 1.00 | 56 | 56 | 2385 | listing of 'test/ranges' |
| 52 | 1.00 | 52 | 52 | 2333 | README.md section #29 |
