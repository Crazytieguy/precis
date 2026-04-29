scores: Score(3000)=0.654 ns_rows≤3K=14/40 (reached=9 partial=1 missing=4)

## Per-budget scores

| B | A_B | I(B) | C(B) | Score(B) | walker_used |
|--:|----:|-----:|-----:|---------:|------------:|
| 1000 | 100 | 0.692 | 0.539 | 0.611 | 963 |
| 1442 | 163 | 0.730 | 0.552 | 0.635 | 1427 |
| 2080 | 194 | 0.715 | 0.547 | 0.625 | 2040 |
| 3000 | 263 | 0.735 | 0.582 | 0.654 | 2967 |
| 4327 | 317 | 0.724 | 0.535 | 0.622 | 4281 |
| 6240 | 446 | 0.688 | 0.391 | 0.519 | 5740 |
| 9000 | 701 | 0.685 | 0.350 | 0.489 | 8839 |

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 4 ranking-recoverable (gap@3k=0.09), 25 wrong-slice/granularity (gap@3k=0.38), 1 no-discovered (gap@3k=0.15)
Secondary intervention: free T_max budget for 1 too-expensive candidate
Top rows: 3.2, 3.3, 2.6, 3.4, 3.5, ...

## Top opportunities

_`gap@B` is a non-additive priority score: `Σ over atoms with rank ≤ |A_B|: (1 − damped_credit(a)) / rank(a)`. `gap@3k` is the primary sort key — direct proxy for `Score(3000)` headroom. `gap@1k` and `gap@9k` show how the same intervention scales across the budget vector. Rows can overlap between opportunities; sums are upper bounds on Score(B) impact, not additive estimates._

| intervention | rows | gap@1k | gap@3k | gap@9k | evidence | top row ids |
|:-------------|-----:|-------:|-------:|-------:|:---------|:------------|
| split wrong-slice walker batches | 25 | 0.00 | 0.38 | 0.90 | nearby candidates have low exact atom overlap | 3.2, 3.3, 2.6, 3.4, 3.5, ... |
| add walker candidates for no-discovered rows | 1 | 0.00 | 0.15 | 0.15 | NS rows have no discovered line candidate | 2.4 |
| free T_max budget / demote late waste | 1 | 0.09 | 0.09 | 0.09 | high-overlap candidates exceed remaining budget at T_max (caveat: not 3K-budget — see below), exact total=9/9 | 2.3 |
| finish partially-delivered NS batches | 1 | 0.00 | 0.08 | 0.07 | avg batch completion=0.54 | 2.6 |

## Diagnosis rollup

| diagnosis | rows | missing | partial | likely lever |
|:----------|-----:|--------:|--------:|:-------------|
| ranking-recoverable | 4 | 4 | 0 | value/ranking |
| wrong-slice / granularity | 25 | 24 | 1 | walker granularity / wrong slice |
| no discovered candidate | 1 | 1 | 0 | walker coverage or predecessor-gated emit |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | gap@3k | likely lever |
|:------------|-----:|-------:|:-------------|
| predecessor not scheduled | 2 | 0.00 | promote predecessor |
| too expensive at final margin | 1 | 0.09 | free T_max budget |
| discovered unscheduled | 1 | 0.00 | tune ranking |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=6, unscheduled bbox=21, unscheduled same-file=2, no discovered candidate=1

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | missing | low | 5 |
| scheduled bbox | partial | low | 1 |
| unscheduled bbox | missing | none | 2 |
| unscheduled bbox | missing | low | 15 |
| unscheduled bbox | missing | high | 1 |
| unscheduled bbox | missing | full | 3 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 2.3 | 750 | 0.00 | 0.00 | missing | README — Usage example (calls into the public API) | [unscheduled bbox exact=9/9] README.md section #2 (9 atoms, too expensive at final margin) |
| 3.6 | 4025 | 0.00 | 0.00 | missing | README — Tilde Ranges desugaring | [unscheduled bbox exact=14/17] README.md section #16 (14 atoms, discovered unscheduled) |
| 4.1 | 5749 | 0.00 | 0.00 | missing | SemVer class — method signatures (locations) | [unscheduled bbox exact=9/9] export at classes/semver.js:9 (16 atoms, predecessor not scheduled: export names surface in classes/semver.js) |
| 4.2 | 5809 | 0.00 | 0.00 | missing | Range class — method signatures (locations) | [unscheduled bbox exact=8/8] export at classes/range.js:6 (14 atoms, predecessor not scheduled: export names surface in classes/range.js) |

### wrong-slice / granularity

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 2.6 | 1667 | 0.74 | 0.54 | partial | README — all section heading locations | [scheduled bbox exact=1/23] README.md section #19 (t=8332, 40 atoms) |
| 3.2 | 2375 | 0.06 | 0.01 | missing | README — Ranges intro (operators, comparator sets, ||) | [scheduled bbox exact=27/35] README.md section #4 (t=6300, 27 atoms) |
| 3.3 | 2947 | 0.06 | 0.01 | missing | README — Prerelease Tags semantics | [scheduled bbox exact=7/34] README.md section #6 (t=6659, 7 atoms) |
| 3.4 | 3219 | 0.00 | 0.00 | missing | README — Hyphen Ranges desugaring | [scheduled bbox exact=12/19] README.md section #14 (t=8839, 12 atoms) |
| 3.5 | 3533 | 0.00 | 0.00 | missing | README — X-Ranges desugaring | [unscheduled bbox exact=13/18] README.md section #15 (13 atoms, discovered unscheduled) |
| 3.7 | 4873 | 0.00 | 0.00 | missing | README — Caret Ranges desugaring (the most complex) | [unscheduled bbox exact=34/43] README.md section #17 (34 atoms, too expensive at final margin) |
| 3.8 | 5124 | 0.11 | 0.02 | missing | README — Functions section preface (options doc) | [scheduled bbox exact=15/19] README.md section #19 (t=8332, 15 atoms) |
| 3.9 | 5681 | 0.12 | 0.05 | missing | README — Coercion semantics | [scheduled bbox exact=5/26] README.md section #49 (t=4462, 5 atoms); better unscheduled exact=12/26: README.md section #48 (12 atoms, discovered unscheduled) |
| 4.3 | 5992 | 0.00 | 0.00 | missing | Range module — internal helpers (locations) | [unscheduled same-file] export body at classes/range.js:6 body 8 (165 atoms, predecessor not scheduled: export at classes/range.js:6) |
| 4.4 | 6069 | 0.00 | 0.00 | missing | Comparator class — method signatures (locations) | [unscheduled bbox exact=7/9] export at classes/comparator.js:5 (12 atoms, predecessor not scheduled: export names surface in classes/comparator.js) |
| 4.5 | 6460 | 0.00 | 0.00 | missing | internal/constants.js — full file | [unscheduled bbox exact=10/37] export at internal/constants.js:28 (10 atoms, predecessor not scheduled: export names surface in internal/constants.js) |
| 5.1 | 7003 | 0.00 | 0.00 | missing | internal/re.js — every token name (locations) | [unscheduled bbox exact=0/42] export names surface in internal/re.js (6 atoms, discovered unscheduled) |
| 5.2 | 7475 | 0.00 | 0.00 | missing | internal/re.js — section comments and exports header | [unscheduled bbox exact=7/38] imports in internal/re.js (7 atoms, discovered unscheduled) |
| 5.3 | 7628 | 0.00 | 0.00 | missing | SemVer.compare — body (entry into compareMain || comparePre) | [unscheduled bbox exact=11/15] export body at classes/semver.js:9 body 11 (11 atoms, predecessor not scheduled: export at classes/semver.js:9) |
| 5.4 | 7934 | 0.00 | 0.00 | missing | internal/identifiers.js — full file | [unscheduled bbox exact=14/29] export body at internal/identifiers.js:4 body 5 (14 atoms, predecessor not scheduled: export at internal/identifiers.js:4) |
| 5.5 | 8148 | 0.00 | 0.00 | missing | bin/semver.js — CLI flag case lines (truncated) | [unscheduled same-file] imports in bin/semver.js (1 atoms, discovered unscheduled) |
| 6.1 | 8372 | 0.00 | 0.00 | missing | functions/ — module.exports lines (locations of every public function) | [unscheduled bbox exact=1/24] export at functions/valid.js:8 (1 atoms, predecessor not scheduled: export names surface in functions/valid.js) |
| 6.2 | 8490 | 0.00 | 0.00 | missing | ranges/ — module.exports lines (locations of every range function) | [unscheduled bbox exact=1/11] export at ranges/valid.js:13 (1 atoms, predecessor not scheduled: export names surface in ranges/valid.js) |
| 6.3 | 8897 | 0.00 | 0.00 | missing | functions/cmp.js — operator switch body | [unscheduled bbox exact=34/44] export body at functions/cmp.js:10 body 11 (34 atoms, predecessor not scheduled: export at functions/cmp.js:10) |
| 6.4 | 9449 | 0.00 | 0.00 | missing | functions/diff.js — release-type comparison body | [unscheduled bbox exact=37/49] export body at functions/diff.js:5 body 6 (42 atoms, predecessor not scheduled: export at functions/diff.js:5) |
| 7.1 | 9601 | 0.00 | 0.00 | missing | internal/parse-options.js — full file | [unscheduled bbox exact=7/17] export body at internal/parse-options.js:6 body 7 (7 atoms, predecessor not scheduled: export at internal/parse-options.js:6) |
| 7.2 | 9711 | 0.00 | 0.00 | missing | internal/debug.js — full file | [unscheduled bbox exact=1/11] export at internal/debug.js:11 (1 atoms, predecessor not scheduled: export names surface in internal/debug.js) |
| 7.3 | 9778 | 0.00 | 0.00 | missing | internal/lrucache.js — class signature + max constant | [unscheduled bbox exact=6/8] export at internal/lrucache.js:3 (8 atoms, predecessor not scheduled: export names surface in internal/lrucache.js) |
| 7.4 | 9852 | 0.00 | 0.00 | missing | ranges/min-version.js — function signature + 0.0.0 fast path | [unscheduled bbox exact=4/6] export body at ranges/min-version.js:7 body 8 (4 atoms, predecessor not scheduled: export at ranges/min-version.js:7) |
| 7.6 | 9977 | 0.00 | 0.00 | missing | bin/semver.js — entry skeleton (shebang, version load, main call) | [unscheduled bbox exact=0/5] imports in bin/semver.js (1 atoms, discovered unscheduled) |

### no discovered candidate

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 2.4 | 1011 | 0.00 | 0.00 | missing | range.bnf — canonical range grammar | no discovered line candidate |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 13 | 1868 | README.md section #<n> |
| 13 | 1430 | CHANGELOG.md section #<n> |
| 3 | 523 | CONTRIBUTING.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 499 | 0.68 | 730 | 8332 | README.md section #19 |
| 373 | 1.00 | 373 | 5740 | plaintext config .gitignore |
| 350 | 1.00 | 350 | 5232 | json config release-please-config.json |
| 330 | 1.00 | 330 | 9169 | CONTRIBUTING.md section #3 |
| 316 | 1.00 | 316 | 7034 | CHANGELOG.md section #2 |
| 265 | 0.96 | 276 | 9445 | README.md section #10 |
| 224 | 1.00 | 224 | 7452 | README.md section #53 |
| 214 | 1.00 | 214 | 8546 | CHANGELOG.md section #6 |
| 172 | 1.00 | 172 | 6472 | README.md section #45 |
| 157 | 1.00 | 157 | 4882 | README.md section #43 |
| 1964 | — | — | — | +24 more rows |
