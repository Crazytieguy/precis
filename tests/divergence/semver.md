scores: Sim=0.526 Reached=11/40 Early=3 Late=5 Partial=1 Missing=28 Used=8763/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 5 | 5 | 0 | 0 | 0.96 |
| 2 | 6 | 2 | 1 | 3 | 0.46 |
| 3 | 9 | 3 | 0 | 6 | 0.29 |
| 4 | 5 | 0 | 0 | 5 | 0.00 |
| 5 | 5 | 0 | 0 | 5 | 0.00 |
| 6 | 4 | 0 | 0 | 4 | 0.00 |
| 7 | 6 | 1 | 0 | 5 | 0.15 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 61 | 410 | +349 | 0.80 | late | package.json — name, description, main | package identity in package.json (t=258, 3 atoms) |
| 1.2 | 164 | 103 | -61 | 1.00 | early | Repo top-level listing |  |
| 1.3 | 281 | 1209 | +928 | 1.00 | late | package.json — bin entry and Node engines floor | package entrypoints in package.json (t=410, 14 atoms) |
| 1.5 | 330 | 437 | +107 | 1.00 | late | internal/ listing |  |
| 2.1 | 354 | 130 | -224 | 1.00 | early | README — title (lede) | README headline in README.md (t=130, 2 atoms) |
| 2.2 | 515 | 1314 | +799 | 1.00 | late | functions/ + ranges/ listings |  |
| 2.3 | 750 | — | — | 0.00 | missing | README — Usage example (calls into the public API) |  |
| 2.4 | 1011 | — | — | 0.00 | missing | range.bnf — canonical range grammar |  |
| 2.5 | 1401 | — | — | 0.00 | missing | index.js — module.exports object body (canonical public API list) |  |
| 2.6 | 1667 | — | — | 0.74 | partial | README — all section heading locations | README.md section #8 (t=5851, 40 atoms) |
| 3.1 | 1775 | 1452 | -323 | 0.88 | aligned | README — Versions section (leading = and v) | README.md section #3 (t=1452, 6 atoms) |
| 3.2 | 2375 | 4402 | +2027 | 0.80 | late | README — Ranges intro (operators, comparator sets, ||) | README.md section #4 (t=4402, 27 atoms) |
| 3.3 | 2947 | — | — | 0.06 | missing | README — Prerelease Tags semantics | headings outline in README.md (t=1010, 2 atoms) |
| 3.4 | 3219 | — | — | 0.00 | missing | README — Hyphen Ranges desugaring |  |
| 3.5 | 3533 | — | — | 0.00 | missing | README — X-Ranges desugaring |  |
| 3.6 | 4025 | — | — | 0.00 | missing | README — Tilde Ranges desugaring |  |
| 3.7 | 4873 | — | — | 0.00 | missing | README — Caret Ranges desugaring (the most complex) |  |
| 3.8 | 5124 | 5851 | +727 | 0.84 | aligned | README — Functions section preface (options doc) | README.md section #8 (t=5851, 15 atoms) |
| 3.9 | 5681 | — | — | 0.08 | missing | README — Coercion semantics | headings outline in README.md (t=1010, 2 atoms) |
| 4.1 | 5749 | — | — | 0.00 | missing | SemVer class — method signatures (locations) |  |
| 4.2 | 5809 | — | — | 0.00 | missing | Range class — method signatures (locations) |  |
| 4.3 | 5992 | — | — | 0.00 | missing | Range module — internal helpers (locations) |  |
| 4.4 | 6069 | — | — | 0.00 | missing | Comparator class — method signatures (locations) |  |
| 4.5 | 6460 | — | — | 0.00 | missing | internal/constants.js — full file |  |
| 5.1 | 7003 | — | — | 0.00 | missing | internal/re.js — every token name (locations) |  |
| 5.2 | 7475 | — | — | 0.00 | missing | internal/re.js — section comments and exports header |  |
| 5.3 | 7628 | — | — | 0.00 | missing | SemVer.compare — body (entry into compareMain || comparePre) |  |
| 5.4 | 7934 | — | — | 0.00 | missing | internal/identifiers.js — full file |  |
| 5.5 | 8148 | — | — | 0.00 | missing | bin/semver.js — CLI flag case lines (truncated) |  |
| 6.1 | 8372 | — | — | 0.00 | missing | functions/ — module.exports lines (locations of every public function) |  |
| 6.2 | 8490 | — | — | 0.00 | missing | ranges/ — module.exports lines (locations of every range function) |  |
| 6.3 | 8897 | — | — | 0.00 | missing | functions/cmp.js — operator switch body |  |
| 6.4 | 9449 | — | — | 0.00 | missing | functions/diff.js — release-type comparison body |  |
| 7.1 | 9601 | — | — | 0.00 | missing | internal/parse-options.js — full file |  |
| 7.2 | 9711 | — | — | 0.00 | missing | internal/debug.js — full file |  |
| 7.3 | 9778 | — | — | 0.00 | missing | internal/lrucache.js — class signature + max constant |  |
| 7.4 | 9852 | — | — | 0.00 | missing | ranges/min-version.js — function signature + 0.0.0 fast path |  |
| 7.5 | 9924 | 628 | -9296 | 0.88 | early | LICENSE first line + CONTRIBUTING.md headings | CONTRIBUTING.md section #3 (t=6433, 14 atoms) |
| 7.6 | 9977 | — | — | 0.00 | missing | bin/semver.js — entry skeleton (shebang, version load, main call) |  |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 17 | 2837 | CHANGELOG.md section #<n> |
| 5 | 1528 | README.md section #<n> |
| 3 | 523 | CONTRIBUTING.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 757 | 1.00 | 757 | 7190 | README.md section #18 |
| 592 | 1.00 | 592 | 7921 | CHANGELOG.md section #1 |
| 499 | 0.68 | 730 | 5851 | README.md section #8 |
| 389 | 1.00 | 389 | 8310 | CHANGELOG.md section #3 |
| 373 | 1.00 | 373 | 3842 | plaintext config .gitignore |
| 350 | 1.00 | 350 | 1885 | json config release-please-config.json |
| 330 | 1.00 | 330 | 6433 | CONTRIBUTING.md section #3 |
| 318 | 1.00 | 318 | 2775 | json config .github/matchers/tap.json |
| 316 | 1.00 | 316 | 4777 | CHANGELOG.md section #2 |
| 219 | 1.00 | 219 | 8763 | CHANGELOG.md section #11 |
| 214 | 1.00 | 214 | 6065 | CHANGELOG.md section #6 |
| 207 | 1.00 | 207 | 8517 | CHANGELOG.md section #10 |
| 144 | 1.00 | 144 | 772 | package scripts in package.json |
| 139 | 1.00 | 139 | 7329 | README.md section #16 |
| 135 | 1.00 | 135 | 3469 | CHANGELOG.md section #7 |
| 133 | 1.00 | 133 | 5121 | CHANGELOG.md section #13 |
| 131 | 1.00 | 131 | 2906 | CHANGELOG.md section #4 |
| 130 | 1.00 | 130 | 2457 | CONTRIBUTING.md section #4 |
| 92 | 0.68 | 135 | 410 | package entrypoints in package.json |
| 84 | 0.75 | 112 | 1209 | package dependencies in package.json |
| 73 | 1.00 | 73 | 3144 | README.md section #10 |
| 72 | 1.00 | 72 | 4899 | CHANGELOG.md section #30 |
| 72 | 1.00 | 72 | 4971 | CHANGELOG.md section #32 |
| 69 | 1.00 | 69 | 3274 | CHANGELOG.md section #18 |
| 63 | 1.00 | 63 | 1535 | CONTRIBUTING.md section #2 |
| 61 | 1.00 | 61 | 3205 | CHANGELOG.md section #20 |
| 60 | 1.00 | 60 | 3334 | CHANGELOG.md section #23 |
| 60 | 1.00 | 60 | 2966 | README.md section #17 |
| 59 | 1.00 | 59 | 4461 | CHANGELOG.md section #38 |
| 57 | 1.00 | 57 | 3023 | CHANGELOG.md section #19 |
| 51 | 1.00 | 51 | 2171 | CHANGELOG.md section #17 |
