scores: Sim=0.430 Reached=18/50 Early=5 Late=5 Partial=12 Missing=20 Used=9862/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 6 | 5 | 1 | 0 | 0.90 |
| 2 | 11 | 2 | 3 | 6 | 0.38 |
| 3 | 10 | 1 | 3 | 6 | 0.43 |
| 4 | 6 | 3 | 1 | 2 | 0.59 |
| 5 | 13 | 5 | 4 | 4 | 0.59 |
| 6 | 4 | 2 | 0 | 2 | 0.56 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 54 | — | — | 0.50 | partial | README H1 + crate one-liner | README headline in README.md (t=75, 1 atoms) |
| 1.3 | 92 | 446 | +354 | 1.00 | late | src/ module layout |  |
| 1.4 | 156 | 7023 | +6867 | 1.00 | late | main.rs module declarations | mod/use plumbing in src/main.rs (t=7023, 9 atoms) |
| 1.5 | 185 | 279 | +94 | 1.00 | late | README section headings | README.md section #3 (t=6515, 27 atoms) |
| 1.6 | 329 | 214 | -115 | 0.92 | early | Cargo package metadata | [package] in Cargo.toml (t=214, 11 atoms) |
| 2.1 | 391 | 3740 | +3349 | 1.00 | late | ContentType variants — locations only | pub item at src/parse/mod.rs:18 (t=3740, 13 atoms) |
| 2.2 | 534 | — | — | 0.60 | partial | ContentType enum context | pub item at src/parse/mod.rs:18 (t=3740, 14 atoms) |
| 2.3 | 653 | 4090 | +3437 | 1.00 | late | Parser trait method signatures | pub item at src/parse/mod.rs:36 (t=4090, 23 atoms) |
| 2.4 | 836 | — | — | 0.16 | missing | ContentType::new_parser dispatch | impl method sigs in src/parse/mod.rs (t=3768, 3 atoms) |
| 2.5 | 931 | — | — | 0.70 | partial | Tree struct fields | pub item at src/tree.rs:14 (t=971, 7 atoms) |
| 2.6 | 1144 | — | — | 0.76 | partial | ItemValue + FieldType | pub item at src/tree.rs:42 (t=813, 8 atoms) |
| 2.7 | 1215 | — | — | 0.00 | missing | Tree public fn signatures |  |
| 2.8 | 1432 | — | — | 0.00 | missing | main.rs run() — config + flag dispatch |  |
| 2.9 | 1755 | — | — | 0.00 | missing | main.rs run() — content type + data read |  |
| 2.10 | 1929 | — | — | 0.00 | missing | main.rs run() — --to short-circuit |  |
| 2.11 | 2171 | — | — | 0.00 | missing | main.rs run() — size check, Tree, App, ui::start |  |
| 3.2 | 2495 | — | — | 0.72 | partial | App struct fields | pub item at src/ui/app.rs:53 (t=5022, 23 atoms) |
| 3.3 | 2739 | — | — | 0.28 | missing | ElementInFocus + Refresh + ShowResult | pub item at src/ui/app.rs:53 (t=5022, 23 atoms) |
| 3.4 | 3020 | — | — | 0.04 | missing | ui::start event loop | pub-item names surface in src/ui/mod.rs (t=2463, 2 atoms) |
| 3.5 | 3205 | — | — | 0.00 | missing | App method names |  |
| 3.6 | 3560 | — | — | 0.36 | missing | TreeOverview struct + impl method names | pub item at src/ui/tree_overview.rs:19 (t=4449, 12 atoms) |
| 3.7 | 3838 | — | — | 0.00 | missing | TreeOverview::on_key action dispatch |  |
| 3.8 | 4084 | — | — | 0.46 | missing | DataBlock struct + method names | pub item at src/ui/data_block.rs:16 (t=3097, 12 atoms) |
| 3.9 | 4359 | — | — | 0.67 | partial | Filter widget surface | pub-item names surface in src/ui/filter.rs (t=2847, 8 atoms) |
| 3.10 | 4672 | — | — | 0.75 | partial | Footer / Header / Popup surfaces | pub item at src/ui/header.rs:11 (t=2734, 6 atoms) |
| 4.2 | 4844 | — | — | 0.71 | partial | SyntaxToken enum | pub item at src/parse/syntax.rs:11 (t=4306, 12 atoms) |
| 4.3 | 5287 | — | — | 0.00 | missing | Per-parser Parser impl headers |  |
| 4.4 | 5491 | 4090 | -1401 | 0.89 | aligned | Parser::parse_root default body | pub item at src/parse/mod.rs:36 (t=4090, 17 atoms) |
| 4.5 | 5796 | — | — | 0.10 | missing | AnyParser auto-detect parse_root | pub item at src/parse/any.rs:9 (t=3787, 3 atoms) |
| 4.6 | 5872 | 4213 | -1659 | 0.86 | aligned | syntax helper signatures | pub-item names surface in src/parse/syntax.rs (t=4191, 5 atoms) |
| 5.1 | 5888 | 1398 | -4490 | 1.00 | early | config/ module layout |  |
| 5.2 | 6234 | — | — | 0.69 | partial | Config struct field names | pub item at src/config/mod.rs:17 (t=2301, 24 atoms) |
| 5.3 | 6578 | — | — | 0.00 | missing | Action enum variants |  |
| 5.4 | 6976 | — | — | 0.00 | missing | Default key bindings |  |
| 5.5 | 7174 | 2410 | -4764 | 0.80 | early | Key enum + KeyAction | pub item at src/config/keys.rs:195 (t=9841, 64 atoms) |
| 5.6 | 7644 | — | — | 0.76 | partial | Per-branch struct fields (Tree / Layout / Filter / Data) | pub-item names surface in src/config/mod.rs (t=1499, 16 atoms) |
| 5.7 | 7894 | — | — | 0.72 | partial | Editor + Header + Footer struct fields | pub-item names surface in src/config/mod.rs (t=1499, 10 atoms) |
| 5.8 | 8331 | — | — | 0.08 | missing | Config::load + Config::parse | impl method sigs in src/config/mod.rs (t=5681, 6 atoms) |
| 5.9 | 8613 | — | — | 0.04 | missing | Config::get_path resolution | impl method sigs in src/config/mod.rs (t=5681, 2 atoms) |
| 5.10 | 8986 | — | — | 0.70 | partial | Color struct + Colors namespaces | pub item at src/config/colors.rs:162 (t=6810, 24 atoms) |
| 5.11 | 9095 | 5940 | -3155 | 1.00 | early | DataColors fields | pub item at src/config/colors.rs:78 (t=5940, 21 atoms) |
| 5.13 | 9311 | 4602 | -4709 | 0.88 | early | Types config struct | pub item at src/config/types.rs:28 (t=4602, 14 atoms) |
| 6.2 | 9652 | — | — | 0.25 | missing | examples/ + docs/ listings |  |
| 6.4 | 9967 | — | — | 0.00 | missing | clipboard OS routing |  |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 4 | 975 | README.md section #<n> |
| 4 | 428 | pub item at src/config/colors.rs:<n> |
| 2 | 153 | docs/changelog.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 870 | 1.00 | 870 | 9841 | pub item at src/config/keys.rs:195 |
| 599 | 0.66 | 905 | 8971 | pub item at src/cmd.rs:13 |
| 553 | 0.96 | 575 | 6515 | README.md section #3 |
| 364 | 0.86 | 425 | 5681 | impl method sigs in src/config/mod.rs |
| 321 | 1.00 | 321 | 7344 | [dependencies] in Cargo.toml |
| 246 | 0.77 | 321 | 7853 | headings outline in docs/changelog.md |
| 188 | 1.00 | 188 | 7532 | mod/use plumbing in src/ui/mod.rs |
| 187 | 0.94 | 200 | 1358 | README.md section #1 |
| 171 | 0.92 | 187 | 1158 | README.md section #2 |
| 147 | 0.50 | 295 | 6810 | pub item at src/config/colors.rs:162 |
| 129 | 0.50 | 259 | 5940 | pub item at src/config/colors.rs:78 |
| 125 | 0.59 | 213 | 7023 | mod/use plumbing in src/main.rs |
| 119 | 1.00 | 119 | 5256 | mod/use plumbing in src/config/mod.rs |
| 115 | 1.00 | 115 | 5137 | mod/use plumbing in src/parse/mod.rs |
| 91 | 1.00 | 91 | 600 | macro_export bodies across src |
| 90 | 0.88 | 103 | 7987 | docs/changelog.md section #1 |
| 87 | 1.00 | 87 | 3353 | pub item at src/config/colors.rs:286 |
| 76 | 0.50 | 153 | 4602 | pub item at src/config/types.rs:28 |
| 65 | 1.00 | 65 | 3266 | pub item at src/config/colors.rs:251 |
| 64 | 0.67 | 96 | 375 | README.md section #4 |
| 63 | 0.80 | 79 | 8066 | docs/changelog.md section #2 |
| 58 | 1.00 | 58 | 722 | pub item at src/live_reload.rs:16 |
