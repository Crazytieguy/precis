scores: Score(3000)=0.509 ns_rows≤3K=20/50 (reached=7 partial=3 missing=10)

## Per-budget scores

| B | A_B | I(B) | C(B) | Score(B) | walker_used |
|--:|----:|-----:|-----:|---------:|------------:|
| 1000 | 98 | 0.783 | 0.356 | 0.528 | 949 |
| 1442 | 153 | 0.756 | 0.259 | 0.442 | 1441 |
| 2080 | 194 | 0.793 | 0.352 | 0.529 | 2065 |
| 3000 | 279 | 0.788 | 0.329 | 0.509 | 2777 |
| 4327 | 405 | 0.760 | 0.245 | 0.431 | 4193 |
| 6240 | 645 | 0.773 | 0.380 | 0.542 | 6032 |
| 9000 | 904 | 0.747 | 0.359 | 0.518 | 8664 |

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 0 ranking-recoverable (gap@3k=0.00), 33 wrong-slice/granularity (gap@3k=2.18), 0 no-discovered (gap@3k=0.00)
Top rows: 2.8, 2.9, 2.4, 3.2, 3.3, ...

## Top opportunities

_`gap@B` is a non-additive priority score: `Σ over atoms in row: (1 − damped_credit(a)) / rank(a)` evaluated at budget B's walker state. Approximates how much closing the row would lift `Score(B)` (via the Importance numerator); not an exact delta. `gap@3k` is the primary sort key. `gap@1k` and `gap@9k` show how the same intervention scales across the budget vector — gap is monotone non-increasing in B (walker has more budget at higher B). Rows can overlap between opportunities; sums are upper bounds on Score(B) impact, not additive estimates._

| intervention | rows | gap@1k | gap@3k | gap@9k | evidence | top row ids |
|:-------------|-----:|-------:|-------:|-------:|:---------|:------------|
| split wrong-slice walker batches | 33 | 2.58 | 2.18 | 1.93 | nearby candidates have low exact atom overlap | 2.8, 2.9, 2.4, 3.2, 3.3, ... |

## Diagnosis rollup

| diagnosis | rows | missing | partial | likely lever |
|:----------|-----:|--------:|--------:|:-------------|
| wrong-slice / granularity | 33 | 27 | 6 | walker granularity / wrong slice |
| fs/listing | 1 | 1 | 0 | filesystem/listing value |
| mixed/unknown | 6 | 6 | 0 | inspect row |

Candidate hint kinds: scheduled bbox=28, scheduled same-file=11, fs-only=1 _(candidates are walker batches discovered this run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` isn't proof that no emit path exists)._

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | missing | none | 4 |
| scheduled bbox | missing | low | 12 |
| scheduled bbox | missing | high | 1 |
| scheduled bbox | missing | full | 5 |
| scheduled bbox | partial | low | 6 |

## Arrival ledger by diagnosis

### wrong-slice / granularity

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 1.2 | 54 | 0.50 | 0.97 | missing | README H1 + crate one-liner | [scheduled bbox exact=1/2] README headline in README.md (t=79, 1 atoms) |
| 2.2 | 534 | 0.60 | 0.85 | partial | ContentType enum context | [scheduled bbox exact=6/10] pub item at src/parse/mod.rs:18 (t=1621, 14 atoms) |
| 2.4 | 836 | 0.16 | 0.14 | missing | ContentType::new_parser dispatch | [scheduled bbox exact=3/13] impl method sigs in src/parse/mod.rs (t=1371, 3 atoms) |
| 2.5 | 931 | 0.70 | 0.98 | partial | Tree struct fields | [scheduled bbox exact=7/10] pub item at src/tree.rs:14 (t=2135, 7 atoms) |
| 2.6 | 1144 | 0.76 | 0.84 | partial | ItemValue + FieldType | [scheduled bbox exact=8/25] pub item at src/tree.rs:42 (t=1839, 8 atoms) |
| 2.7 | 1215 | 0.00 | 0.00 | missing | Tree public fn signatures | [scheduled same-file] pub item at src/tree.rs:42 (t=1839, 8 atoms) |
| 2.8 | 1432 | 0.00 | 0.00 | missing | main.rs run() — config + flag dispatch | [scheduled same-file] mod/use plumbing in src/main.rs (t=4406, 22 atoms) |
| 2.9 | 1755 | 0.00 | 0.00 | missing | main.rs run() — content type + data read | [scheduled same-file] mod/use plumbing in src/main.rs (t=4406, 22 atoms) |
| 2.10 | 1929 | 0.00 | 0.00 | missing | main.rs run() — --to short-circuit | [scheduled same-file] mod/use plumbing in src/main.rs (t=4406, 22 atoms) |
| 2.11 | 2171 | 0.00 | 0.00 | missing | main.rs run() — size check, Tree, App, ui::start | [scheduled same-file] mod/use plumbing in src/main.rs (t=4406, 22 atoms) |
| 3.2 | 2495 | 0.03 | 0.03 | missing | App struct fields | [scheduled bbox exact=23/32] pub item at src/ui/app.rs:53 (t=5836, 23 atoms) |
| 3.3 | 2739 | 0.28 | 0.22 | missing | ElementInFocus + Refresh + ShowResult | [scheduled bbox exact=0/29] pub item at src/ui/app.rs:53 (t=5836, 23 atoms) |
| 3.4 | 3020 | 0.04 | 0.06 | missing | ui::start event loop | [scheduled bbox exact=2/27] pub item at src/ui/mod.rs:35 (t=1084, 2 atoms); better unscheduled exact=15/27: pub item body at src/ui/mod.rs:35 body 39 (15 atoms, too expensive at final margin) |
| 3.5 | 3205 | 0.00 | 0.00 | missing | App method names | [scheduled same-file] pub item at src/ui/app.rs:53 (t=5836, 23 atoms) |
| 3.6 | 3560 | 0.03 | 0.03 | missing | TreeOverview struct + impl method names | [scheduled bbox exact=12/33] pub item at src/ui/tree_overview.rs:19 (t=4838, 12 atoms) |
| 3.7 | 3838 | 0.00 | 0.00 | missing | TreeOverview::on_key action dispatch | [scheduled same-file] pub item at src/ui/tree_overview.rs:19 (t=4838, 12 atoms) |
| 3.8 | 4084 | 0.04 | 0.04 | missing | DataBlock struct + method names | [scheduled bbox exact=12/26] pub item at src/ui/data_block.rs:16 (t=4193, 12 atoms) |
| 3.9 | 4359 | 0.67 | 0.67 | missing | Filter widget surface | [scheduled bbox exact=8/33] pub-item names surface in src/ui/filter.rs (t=2640, 8 atoms) |
| 3.10 | 4672 | 0.75 | 0.86 | partial | Footer / Header / Popup surfaces | [scheduled bbox exact=6/36] pub item at src/ui/header.rs:11 (t=2556, 6 atoms) |
| 4.2 | 4844 | 0.00 | 0.00 | missing | SyntaxToken enum | [scheduled bbox exact=12/17] pub item at src/parse/syntax.rs:11 (t=3224, 12 atoms) |
| 4.3 | 5287 | 0.00 | 0.00 | missing | Per-parser Parser impl headers | [scheduled same-file] pub-item names surface in src/parse/json.rs (t=2597, 4 atoms) |
| 4.5 | 5796 | 0.10 | 0.09 | missing | AnyParser auto-detect parse_root | [scheduled bbox exact=3/30] pub item at src/parse/any.rs:9 (t=1460, 3 atoms) |
| 4.6 | 5872 | 0.00 | 0.00 | missing | syntax helper signatures | [scheduled bbox exact=0/7] pub item body at src/parse/syntax.rs:139 body 140 (t=7377, 9 atoms) |
| 5.2 | 6234 | 0.03 | 0.03 | missing | Config struct field names | [scheduled bbox exact=24/35] pub item at src/config/mod.rs:17 (t=3049, 24 atoms) |
| 5.3 | 6578 | 0.00 | 0.00 | missing | Action enum variants | [scheduled same-file] pub item at src/config/keys.rs:195 (t=9534, 64 atoms) |
| 5.4 | 6976 | 0.00 | 0.00 | missing | Default key bindings | [scheduled same-file] pub item at src/config/keys.rs:195 (t=9534, 64 atoms) |
| 5.5 | 7174 | 0.20 | 0.27 | missing | Key enum + KeyAction | [scheduled bbox exact=0/25] pub item at src/config/keys.rs:195 (t=9534, 64 atoms) |
| 5.6 | 7644 | 0.76 | 0.79 | partial | Per-branch struct fields (Tree / Layout / Filter / Data) | [scheduled bbox exact=10/45] pub-item names surface in src/config/mod.rs (t=556, 16 atoms) |
| 5.7 | 7894 | 0.72 | 0.76 | partial | Editor + Header + Footer struct fields | [scheduled bbox exact=6/25] pub-item names surface in src/config/mod.rs (t=556, 10 atoms) |
| 5.8 | 8331 | 0.00 | 0.00 | missing | Config::load + Config::parse | [scheduled bbox exact=6/39] impl method sigs in src/config/mod.rs (t=5602, 6 atoms) |
| 5.9 | 8613 | 0.00 | 0.00 | missing | Config::get_path resolution | [scheduled bbox exact=2/24] impl method sigs in src/config/mod.rs (t=5602, 2 atoms) |
| 5.10 | 8986 | 0.00 | 0.00 | missing | Color struct + Colors namespaces | [scheduled bbox exact=0/37] pub item at src/config/colors.rs:162 (t=7259, 24 atoms) |
| 6.4 | 9967 | 0.00 | 0.00 | missing | clipboard OS routing | [scheduled same-file] pub item at src/clipboard.rs:28 (t=435, 2 atoms) |

### fs/listing

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 6.2 | 9652 | 0.25 | 0.25 | missing | examples/ + docs/ listings | fs-only |

### mixed/unknown

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 1.4 | 156 | 0.00 | 0.00 | missing | main.rs module declarations | [scheduled bbox exact=9/9] mod/use plumbing in src/main.rs (t=4406, 9 atoms) |
| 5.11 | 9095 | 0.00 | 0.00 | missing | DataColors fields | [scheduled bbox exact=11/11] pub item at src/config/colors.rs:78 (t=6291, 21 atoms) |
| 5.12 | 9221 | 0.00 | 0.00 | missing | TreeColors fields | [scheduled bbox exact=12/12] pub item at src/config/colors.rs:162 (t=7259, 23 atoms) |
| 5.13 | 9311 | 0.12 | 0.09 | missing | Types config struct | [scheduled bbox exact=7/8] pub item at src/config/types.rs:28 (t=4991, 14 atoms) |
| 6.1 | 9599 | 0.04 | 0.03 | missing | CommandArgs flag list | [scheduled bbox exact=25/25] pub item at src/cmd.rs:13 (t=8506, 73 atoms) |
| 6.3 | 9808 | 0.00 | 0.00 | missing | Changelog version headings | [scheduled bbox exact=13/13] headings outline in docs/changelog.md (t=6933, 53 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 4 | 489 | pub item at src/config/colors.rs:<n> |
| 5 | 483 | README.md section #<n> |
| 2 | 182 | docs/changelog.md section #<n> |

## Walker waste, primary-actionable (first_t ≤ 3000, off-NS spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 58 | 1.00 | 58 | 1897 | pub item at src/live_reload.rs:16 |

## Walker waste, late (first_t > 3000, off-NS spend ≥ 50) — higher-budget calibration only

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 870 | 1.00 | 870 | 9534 | pub item at src/config/keys.rs:195 |
| 626 | 0.69 | 905 | 8506 | pub item at src/cmd.rs:13 |
| 348 | 0.82 | 425 | 5602 | impl method sigs in src/config/mod.rs |
| 321 | 1.00 | 321 | 6612 | [dependencies] in Cargo.toml |
| 188 | 1.00 | 188 | 4594 | mod/use plumbing in src/ui/mod.rs |
| 187 | 1.00 | 187 | 7601 | README.md section #4 |
| 178 | 0.60 | 295 | 7259 | pub item at src/config/colors.rs:162 |
| 165 | 0.51 | 321 | 6933 | headings outline in docs/changelog.md |
| 159 | 0.61 | 259 | 6291 | pub item at src/config/colors.rs:78 |
| 149 | 0.70 | 213 | 4406 | mod/use plumbing in src/main.rs |
| 1469 | — | — | — | +16 more rows |
