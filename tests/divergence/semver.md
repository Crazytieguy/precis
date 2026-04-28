scores: Sim=0.537 Reached=11/40 Early=3 Late=4 Partial=1 Missing=28 Used=8445/10000

## Verdict

Verdict: coverage-gap bound
Likely primary lever: add walker candidates for no-discovered NS rows
Evidence: 4 ranking-recoverable (w×gap=1.09), 4 wrong-slice/granularity (w×gap=0.57), 21 no-discovered (w×gap=1.55)
Secondary intervention: free final budget for 2 too-expensive candidates
Loss reasons: 0 predecessor-gated, 2 too-expensive, 2 discovered-unscheduled
Top rows: 2.4, 2.5, 4.1, 4.2, 4.3, ...
Note: likely lever is heuristic; verify `Sim` moves, not just bucket counts.

## Top opportunities

_`w(t)×gap` is a non-additive priority score: Σ exp(-exp_t/τ) × (1 - credit) per row, τ=2000. Same time weighting and credit gap as Sim, but rows can overlap between opportunities — sums across rows are an upper bound on Sim impact, not an additive estimate._

| intervention | rows | w(t)×gap | bands ≤3k/≤6k/total | evidence | top row ids |
|:-------------|-----:|---------:|:----------------------|:---------|:------------|
| add walker candidates for no-discovered rows | 21 | 1.55 | 2/5/21 | NS rows have no discovered line candidate | 2.4, 2.5, 4.1, 4.2, 4.3, ... |
| free final budget / demote late waste | 2 | 0.82 | 1/2/2 | high-overlap candidates exceed final remaining budget, exact total=23/26 | 2.3, 3.6 |
| split wrong-slice walker batches | 4 | 0.57 | 1/4/4 | nearby candidates have low exact atom overlap | 3.4, 3.5, 2.6, 3.7 |
| tune ranking for discovered unscheduled candidates | 2 | 0.27 | 1/2/2 | high-overlap candidates fit but did not win, exact total=51/60 | 3.3, 3.9 |

Tiers: 1=5/5 reached, 0 partial, 0 missing, avg=0.96; 2=2/6 reached, 1 partial, 3 missing, avg=0.46; 3=3/9 reached, 0 partial, 6 missing, avg=0.29; 4=0/5 reached, 0 partial, 5 missing, avg=0.00; 5=0/5 reached, 0 partial, 5 missing, avg=0.00; 6=0/4 reached, 0 partial, 4 missing, avg=0.00; 7=1/6 reached, 0 partial, 5 missing, avg=0.15

## Diagnosis rollup

| diagnosis | rows | missing | partial | timing | likely lever |
|:----------|-----:|--------:|--------:|-------:|:-------------|
| ranking-recoverable | 4 | 4 | 0 | 0 | value/ranking |
| wrong-slice / granularity | 4 | 3 | 1 | 0 | walker granularity / wrong slice |
| no discovered candidate | 21 | 21 | 0 | 0 | walker coverage or predecessor-gated emit |
| timing-only | 9 | 0 | 0 | 9 | usually no code change |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | w(t)×gap | likely lever |
|:------------|-----:|---------:|:-------------|
| too expensive at final margin | 2 | 0.82 | free final budget |
| discovered unscheduled | 2 | 0.27 | tune ranking |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=10, unscheduled bbox=5, fs-only=2, no discovered candidate=21

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | aligned | low | 2 |
| scheduled bbox | early | low | 1 |
| scheduled bbox | early | full | 1 |
| scheduled bbox | late | low | 3 |
| scheduled bbox | missing | low | 2 |
| scheduled bbox | partial | low | 1 |
| unscheduled bbox | missing | low | 3 |
| unscheduled bbox | missing | high | 1 |
| unscheduled bbox | missing | full | 1 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 2.3 | 750 | — | — | 0.00 | missing | README — Usage example (calls into the public API) | [unscheduled bbox exact=9/9] README.md section #2 (9 atoms, too expensive at final margin) |
| 3.3 | 2947 | — | — | 0.06 | missing | README — Prerelease Tags semantics | [scheduled bbox exact=2/34] headings outline in README.md (t=731, 2 atoms); better unscheduled exact=29/34: README.md section #5 (29 atoms, discovered unscheduled) |
| 3.6 | 4025 | — | — | 0.00 | missing | README — Tilde Ranges desugaring | [unscheduled bbox exact=14/17] README.md section #6 (14 atoms, too expensive at final margin) |
| 3.9 | 5681 | — | — | 0.08 | missing | README — Coercion semantics | [scheduled bbox exact=2/26] headings outline in README.md (t=731, 2 atoms); better unscheduled exact=22/26: README.md section #13 (22 atoms, discovered unscheduled) |

### wrong-slice / granularity

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 2.6 | 1667 | — | — | 0.74 | partial | README — all section heading locations | [scheduled bbox exact=1/23] README.md section #8 (t=5533, 40 atoms); better unscheduled exact=5/23: README.md section #6 (78 atoms, too expensive at final margin) |
| 3.4 | 3219 | — | — | 0.00 | missing | README — Hyphen Ranges desugaring | [unscheduled bbox exact=12/19] README.md section #6 (12 atoms, too expensive at final margin) |
| 3.5 | 3533 | — | — | 0.00 | missing | README — X-Ranges desugaring | [unscheduled bbox exact=13/18] README.md section #6 (13 atoms, too expensive at final margin) |
| 3.7 | 4873 | — | — | 0.00 | missing | README — Caret Ranges desugaring (the most complex) | [unscheduled bbox exact=34/43] README.md section #6 (34 atoms, too expensive at final margin) |

### no discovered candidate

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 2.4 | 1011 | — | — | 0.00 | missing | range.bnf — canonical range grammar | no discovered line candidate |
| 2.5 | 1401 | — | — | 0.00 | missing | index.js — module.exports object body (canonical public API list) | no discovered line candidate |
| 4.1 | 5749 | — | — | 0.00 | missing | SemVer class — method signatures (locations) | no discovered line candidate |
| 4.2 | 5809 | — | — | 0.00 | missing | Range class — method signatures (locations) | no discovered line candidate |
| 4.3 | 5992 | — | — | 0.00 | missing | Range module — internal helpers (locations) | no discovered line candidate |
| 4.4 | 6069 | — | — | 0.00 | missing | Comparator class — method signatures (locations) | no discovered line candidate |
| 4.5 | 6460 | — | — | 0.00 | missing | internal/constants.js — full file | no discovered line candidate |
| 5.1 | 7003 | — | — | 0.00 | missing | internal/re.js — every token name (locations) | no discovered line candidate |
| 5.2 | 7475 | — | — | 0.00 | missing | internal/re.js — section comments and exports header | no discovered line candidate |
| 5.3 | 7628 | — | — | 0.00 | missing | SemVer.compare — body (entry into compareMain || comparePre) | no discovered line candidate |
| 5.4 | 7934 | — | — | 0.00 | missing | internal/identifiers.js — full file | no discovered line candidate |
| 5.5 | 8148 | — | — | 0.00 | missing | bin/semver.js — CLI flag case lines (truncated) | no discovered line candidate |
| 6.1 | 8372 | — | — | 0.00 | missing | functions/ — module.exports lines (locations of every public function) | no discovered line candidate |
| 6.2 | 8490 | — | — | 0.00 | missing | ranges/ — module.exports lines (locations of every range function) | no discovered line candidate |
| 6.3 | 8897 | — | — | 0.00 | missing | functions/cmp.js — operator switch body | no discovered line candidate |
| 6.4 | 9449 | — | — | 0.00 | missing | functions/diff.js — release-type comparison body | no discovered line candidate |
| 7.1 | 9601 | — | — | 0.00 | missing | internal/parse-options.js — full file | no discovered line candidate |
| 7.2 | 9711 | — | — | 0.00 | missing | internal/debug.js — full file | no discovered line candidate |
| 7.3 | 9778 | — | — | 0.00 | missing | internal/lrucache.js — class signature + max constant | no discovered line candidate |
| 7.4 | 9852 | — | — | 0.00 | missing | ranges/min-version.js — function signature + 0.0.0 fast path | no discovered line candidate |
| 7.6 | 9977 | — | — | 0.00 | missing | bin/semver.js — entry skeleton (shebang, version load, main call) | no discovered line candidate |

### timing-only

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 61 | 912 | +851 | 0.80 | late | package.json — name, description, main | [scheduled bbox exact=3/5] package identity in package.json (t=258, 3 atoms) |
| 1.2 | 164 | 103 | -61 | 1.00 | early | Repo top-level listing | fs-only |
| 1.3 | 281 | 1209 | +928 | 1.00 | late | package.json — bin entry and Node engines floor | [scheduled bbox exact=3/11] package entrypoints in package.json (t=912, 14 atoms) |
| 2.1 | 354 | 130 | -224 | 1.00 | early | README — title (lede) | [scheduled bbox exact=2/2] README headline in README.md (t=130, 2 atoms) |
| 2.2 | 515 | 1314 | +799 | 1.00 | late | functions/ + ranges/ listings | fs-only |
| 3.1 | 1775 | 1452 | -323 | 0.88 | aligned | README — Versions section (leading = and v) | [scheduled bbox exact=6/8] README.md section #3 (t=1452, 6 atoms) |
| 3.2 | 2375 | 4084 | +1709 | 0.80 | late | README — Ranges intro (operators, comparator sets, ||) | [scheduled bbox exact=27/35] README.md section #4 (t=4084, 27 atoms) |
| 3.8 | 5124 | 5533 | +409 | 0.84 | aligned | README — Functions section preface (options doc) | [scheduled bbox exact=15/19] README.md section #8 (t=5533, 15 atoms) |
| 7.5 | 9924 | 469 | -9455 | 0.88 | early | LICENSE first line + CONTRIBUTING.md headings | [scheduled bbox exact=4/8] CONTRIBUTING.md section #3 (t=6115, 14 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 17 | 2837 | CHANGELOG.md section #<n> |
| 5 | 1528 | README.md section #<n> |
| 3 | 523 | CONTRIBUTING.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 757 | 1.00 | 757 | 6872 | README.md section #18 |
| 592 | 1.00 | 592 | 7603 | CHANGELOG.md section #1 |
| 499 | 0.68 | 730 | 5533 | README.md section #8 |
| 389 | 1.00 | 389 | 7992 | CHANGELOG.md section #3 |
| 373 | 1.00 | 373 | 3524 | plaintext config .gitignore |
| 350 | 1.00 | 350 | 3016 | json config release-please-config.json |
| 330 | 1.00 | 330 | 6115 | CONTRIBUTING.md section #3 |
| 316 | 1.00 | 316 | 4459 | CHANGELOG.md section #2 |
| 219 | 1.00 | 219 | 8445 | CHANGELOG.md section #11 |
| 214 | 1.00 | 214 | 5747 | CHANGELOG.md section #6 |
| 1892 | — | — | — | +20 more rows |
