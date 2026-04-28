scores: Sim=0.575 Reached=14/40 Early=3 Late=8 Partial=1 Missing=25 Used=9492/10000

## Verdict

Verdict: budget-pressure bound
Likely primary lever: free final budget / demote late low-value spend
Evidence: 4 ranking-recoverable (w×gap=0.93), 21 wrong-slice/granularity (w×gap=0.70), 1 no-discovered (w×gap=0.60)
Secondary intervention: promote predecessors for 2 gated candidates
Loss reasons: 2 predecessor-gated, 1 too-expensive, 1 discovered-unscheduled
Top rows: 3.5, 3.7, 3.4, 4.3, 4.4, ...
Note: likely lever is heuristic; verify `Sim` moves, not just bucket counts.

## Top opportunities

_`w(t)×gap` is a non-additive priority score: Σ exp(-exp_t/τ) × (1 - credit) per row, τ=2000. Same time weighting and credit gap as Sim, but rows can overlap between opportunities — sums across rows are an upper bound on Sim impact, not an additive estimate._

| intervention | rows | w(t)×gap | bands ≤3k/≤6k/total | evidence | top row ids |
|:-------------|-----:|---------:|:----------------------|:---------|:------------|
| split wrong-slice walker batches | 21 | 0.70 | 0/5/21 | nearby candidates have low exact atom overlap | 3.5, 3.7, 3.4, 4.3, 4.4, ... |
| free final budget / demote late waste | 1 | 0.69 | 1/1/1 | high-overlap candidates exceed final remaining budget, exact total=9/9 | 2.3 |
| add walker candidates for no-discovered rows | 1 | 0.60 | 1/1/1 | NS rows have no discovered line candidate | 2.4 |
| tune ranking for discovered unscheduled candidates | 1 | 0.13 | 0/1/1 | high-overlap candidates fit but did not win, exact total=14/17 | 3.6 |
| promote export names surface in classes/semver.js | 1 | 0.06 | 0/1/1 | 0 files, exact total=9/9 | 4.1 |

Tiers: 1=5/5 reached, 0 partial, 0 missing, avg=0.96; 2=4/6 reached, 0 partial, 2 missing, avg=0.64; 3=4/9 reached, 1 partial, 4 missing, avg=0.49; 4=0/5 reached, 0 partial, 5 missing, avg=0.00; 5=0/5 reached, 0 partial, 5 missing, avg=0.00; 6=0/4 reached, 0 partial, 4 missing, avg=0.00; 7=1/6 reached, 0 partial, 5 missing, avg=0.15

## Diagnosis rollup

| diagnosis | rows | missing | partial | timing | likely lever |
|:----------|-----:|--------:|--------:|-------:|:-------------|
| ranking-recoverable | 4 | 4 | 0 | 0 | value/ranking |
| wrong-slice / granularity | 21 | 20 | 1 | 0 | walker granularity / wrong slice |
| no discovered candidate | 1 | 1 | 0 | 0 | walker coverage or predecessor-gated emit |
| timing-only | 12 | 0 | 0 | 12 | usually no code change |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | w(t)×gap | likely lever |
|:------------|-----:|---------:|:-------------|
| predecessor not scheduled | 2 | 0.11 | promote predecessor |
| too expensive at final margin | 1 | 0.69 | free final budget |
| discovered unscheduled | 1 | 0.13 | tune ranking |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=12, unscheduled bbox=21, unscheduled same-file=2, fs-only=2, no discovered candidate=1

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | aligned | low | 1 |
| scheduled bbox | early | low | 1 |
| scheduled bbox | early | full | 1 |
| scheduled bbox | late | low | 6 |
| scheduled bbox | late | full | 1 |
| scheduled bbox | missing | low | 1 |
| scheduled bbox | partial | low | 1 |
| unscheduled bbox | missing | none | 2 |
| unscheduled bbox | missing | low | 15 |
| unscheduled bbox | missing | high | 1 |
| unscheduled bbox | missing | full | 3 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 2.3 | 750 | — | — | 0.00 | missing | README — Usage example (calls into the public API) | [unscheduled bbox exact=9/9] README.md section #2 (9 atoms, too expensive at final margin) |
| 3.6 | 4025 | — | — | 0.00 | missing | README — Tilde Ranges desugaring | [unscheduled bbox exact=14/17] README.md section #16 (14 atoms, discovered unscheduled) |
| 4.1 | 5749 | — | — | 0.00 | missing | SemVer class — method signatures (locations) | [unscheduled bbox exact=9/9] export at classes/semver.js:9 (16 atoms, predecessor not scheduled: export names surface in classes/semver.js) |
| 4.2 | 5809 | — | — | 0.00 | missing | Range class — method signatures (locations) | [unscheduled bbox exact=8/8] export at classes/range.js:6 (14 atoms, predecessor not scheduled: export names surface in classes/range.js) |

### wrong-slice / granularity

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 3.4 | 3219 | — | — | 0.63 | partial | README — Hyphen Ranges desugaring | [scheduled bbox exact=12/19] README.md section #14 (t=8886, 12 atoms) |
| 3.5 | 3533 | — | — | 0.00 | missing | README — X-Ranges desugaring | [unscheduled bbox exact=13/18] README.md section #15 (13 atoms, discovered unscheduled) |
| 3.7 | 4873 | — | — | 0.00 | missing | README — Caret Ranges desugaring (the most complex) | [unscheduled bbox exact=34/43] README.md section #17 (34 atoms, too expensive at final margin) |
| 3.9 | 5681 | — | — | 0.42 | missing | README — Coercion semantics | [scheduled bbox exact=5/26] README.md section #49 (t=4509, 5 atoms); better unscheduled exact=12/26: README.md section #48 (12 atoms, discovered unscheduled) |
| 4.3 | 5992 | — | — | 0.00 | missing | Range module — internal helpers (locations) | [unscheduled same-file] export body at classes/range.js:6 body 8 (165 atoms, predecessor not scheduled: export at classes/range.js:6) |
| 4.4 | 6069 | — | — | 0.00 | missing | Comparator class — method signatures (locations) | [unscheduled bbox exact=7/9] export at classes/comparator.js:5 (12 atoms, predecessor not scheduled: export names surface in classes/comparator.js) |
| 4.5 | 6460 | — | — | 0.00 | missing | internal/constants.js — full file | [unscheduled bbox exact=10/37] export at internal/constants.js:28 (10 atoms, predecessor not scheduled: export names surface in internal/constants.js) |
| 5.1 | 7003 | — | — | 0.00 | missing | internal/re.js — every token name (locations) | [unscheduled bbox exact=0/42] export names surface in internal/re.js (6 atoms, discovered unscheduled) |
| 5.2 | 7475 | — | — | 0.00 | missing | internal/re.js — section comments and exports header | [unscheduled bbox exact=7/38] imports in internal/re.js (7 atoms, discovered unscheduled) |
| 5.3 | 7628 | — | — | 0.00 | missing | SemVer.compare — body (entry into compareMain || comparePre) | [unscheduled bbox exact=11/15] export body at classes/semver.js:9 body 11 (11 atoms, predecessor not scheduled: export at classes/semver.js:9) |
| 5.4 | 7934 | — | — | 0.00 | missing | internal/identifiers.js — full file | [unscheduled bbox exact=14/29] export body at internal/identifiers.js:4 body 5 (14 atoms, predecessor not scheduled: export at internal/identifiers.js:4) |
| 5.5 | 8148 | — | — | 0.00 | missing | bin/semver.js — CLI flag case lines (truncated) | [unscheduled same-file] imports in bin/semver.js (1 atoms, discovered unscheduled) |
| 6.1 | 8372 | — | — | 0.00 | missing | functions/ — module.exports lines (locations of every public function) | [unscheduled bbox exact=1/24] export at functions/valid.js:8 (1 atoms, predecessor not scheduled: export names surface in functions/valid.js) |
| 6.2 | 8490 | — | — | 0.00 | missing | ranges/ — module.exports lines (locations of every range function) | [unscheduled bbox exact=1/11] export at ranges/valid.js:13 (1 atoms, predecessor not scheduled: export names surface in ranges/valid.js) |
| 6.3 | 8897 | — | — | 0.00 | missing | functions/cmp.js — operator switch body | [unscheduled bbox exact=34/44] export body at functions/cmp.js:10 body 11 (34 atoms, predecessor not scheduled: export at functions/cmp.js:10) |
| 6.4 | 9449 | — | — | 0.00 | missing | functions/diff.js — release-type comparison body | [unscheduled bbox exact=37/49] export body at functions/diff.js:5 body 6 (42 atoms, predecessor not scheduled: export at functions/diff.js:5) |
| 7.1 | 9601 | — | — | 0.00 | missing | internal/parse-options.js — full file | [unscheduled bbox exact=7/17] export body at internal/parse-options.js:6 body 7 (7 atoms, predecessor not scheduled: export at internal/parse-options.js:6) |
| 7.2 | 9711 | — | — | 0.00 | missing | internal/debug.js — full file | [unscheduled bbox exact=1/11] export at internal/debug.js:11 (1 atoms, predecessor not scheduled: export names surface in internal/debug.js) |
| 7.3 | 9778 | — | — | 0.00 | missing | internal/lrucache.js — class signature + max constant | [unscheduled bbox exact=6/8] export at internal/lrucache.js:3 (8 atoms, predecessor not scheduled: export names surface in internal/lrucache.js) |
| 7.4 | 9852 | — | — | 0.00 | missing | ranges/min-version.js — function signature + 0.0.0 fast path | [unscheduled bbox exact=4/6] export body at ranges/min-version.js:7 body 8 (4 atoms, predecessor not scheduled: export at ranges/min-version.js:7) |
| 7.6 | 9977 | — | — | 0.00 | missing | bin/semver.js — entry skeleton (shebang, version load, main call) | [unscheduled bbox exact=0/5] imports in bin/semver.js (1 atoms, discovered unscheduled) |

### no discovered candidate

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 2.4 | 1011 | — | — | 0.00 | missing | range.bnf — canonical range grammar | no discovered line candidate |

### timing-only

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 61 | 953 | +892 | 0.80 | late | package.json — name, description, main | [scheduled bbox exact=3/5] package identity in package.json (t=268, 3 atoms) |
| 1.2 | 164 | 103 | -61 | 1.00 | early | Repo top-level listing | fs-only |
| 1.3 | 281 | 1250 | +969 | 1.00 | late | package.json — bin entry and Node engines floor | [scheduled bbox exact=3/11] package entrypoints in package.json (t=953, 14 atoms) |
| 2.1 | 354 | 130 | -224 | 1.00 | early | README — title (lede) | [scheduled bbox exact=2/2] README headline in README.md (t=130, 2 atoms) |
| 2.2 | 515 | 1355 | +840 | 1.00 | late | functions/ + ranges/ listings | fs-only |
| 2.5 | 1401 | 2697 | +1296 | 1.00 | late | index.js — module.exports object body (canonical public API list) | [scheduled bbox exact=47/47] export at index.js:45 (t=2697, 47 atoms) |
| 2.6 | 1667 | 9492 | +7825 | 0.83 | late | README — all section heading locations | [scheduled bbox exact=1/23] README.md section #19 (t=8379, 40 atoms) |
| 3.1 | 1775 | 1764 | -11 | 0.88 | aligned | README — Versions section (leading = and v) | [scheduled bbox exact=6/8] README.md section #3 (t=1764, 6 atoms) |
| 3.2 | 2375 | 6347 | +3972 | 0.80 | late | README — Ranges intro (operators, comparator sets, ||) | [scheduled bbox exact=27/35] README.md section #4 (t=6347, 27 atoms) |
| 3.3 | 2947 | 6706 | +3759 | 0.88 | late | README — Prerelease Tags semantics | [scheduled bbox exact=7/34] README.md section #6 (t=6706, 7 atoms) |
| 3.8 | 5124 | 8379 | +3255 | 0.84 | late | README — Functions section preface (options doc) | [scheduled bbox exact=15/19] README.md section #19 (t=8379, 15 atoms) |
| 7.5 | 9924 | 510 | -9414 | 0.88 | early | LICENSE first line + CONTRIBUTING.md headings | [scheduled bbox exact=4/8] CONTRIBUTING.md section #3 (t=9216, 14 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 13 | 1868 | README.md section #<n> |
| 13 | 1430 | CHANGELOG.md section #<n> |
| 3 | 523 | CONTRIBUTING.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 499 | 0.68 | 730 | 8379 | README.md section #19 |
| 373 | 1.00 | 373 | 5787 | plaintext config .gitignore |
| 350 | 1.00 | 350 | 5279 | json config release-please-config.json |
| 330 | 1.00 | 330 | 9216 | CONTRIBUTING.md section #3 |
| 316 | 1.00 | 316 | 7081 | CHANGELOG.md section #2 |
| 265 | 0.96 | 276 | 9492 | README.md section #10 |
| 224 | 1.00 | 224 | 7499 | README.md section #53 |
| 214 | 1.00 | 214 | 8593 | CHANGELOG.md section #6 |
| 172 | 1.00 | 172 | 6519 | README.md section #45 |
| 157 | 1.00 | 157 | 4929 | README.md section #43 |
| 1964 | — | — | — | +24 more rows |
