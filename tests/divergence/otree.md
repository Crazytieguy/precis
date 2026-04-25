scores: Sim=0.417 Reached=17/50 Early=5 Late=5 Partial=13 Missing=20 Used=9976/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 6 | 5 | 1 | 0 | 0.90 |
| 2 | 11 | 2 | 3 | 6 | 0.38 |
| 3 | 10 | 1 | 3 | 6 | 0.43 |
| 4 | 6 | 3 | 1 | 2 | 0.59 |
| 5 | 13 | 5 | 4 | 4 | 0.59 |
| 6 | 4 | 1 | 1 | 2 | 0.47 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 54 | — | — | 0.50 | partial | README H1 + crate one-liner | README headline in README.md (t=62, 2 atoms) |
| 1.3 | 92 | 401 | +309 | 1.00 | late | src/ module layout |  |
| 1.4 | 156 | 7075 | +6919 | 1.00 | late | main.rs module declarations | mod/use plumbing in src/main.rs (t=7075, 9 atoms) |
| 1.5 | 185 | 6567 | +6382 | 1.00 | late | README section headings | README.md section #3 (t=6567, 27 atoms) |
| 1.6 | 329 | 201 | -128 | 0.92 | early | Cargo package metadata | [package] in Cargo.toml (t=201, 11 atoms) |
| 2.1 | 391 | 3733 | +3342 | 1.00 | late | ContentType variants — locations only | pub item at src/parse/mod.rs:18 (t=3733, 13 atoms) |
| 2.2 | 534 | — | — | 0.60 | partial | ContentType enum context | pub item at src/parse/mod.rs:18 (t=3733, 14 atoms) |
| 2.3 | 653 | 4083 | +3430 | 1.00 | late | Parser trait method signatures | pub item at src/parse/mod.rs:36 (t=4083, 23 atoms) |
| 2.4 | 836 | — | — | 0.16 | missing | ContentType::new_parser dispatch | impl method sigs in src/parse/mod.rs (t=3761, 3 atoms) |
| 2.5 | 931 | — | — | 0.70 | partial | Tree struct fields | pub item at src/tree.rs:14 (t=926, 7 atoms) |
| 2.6 | 1144 | — | — | 0.76 | partial | ItemValue + FieldType | pub item at src/tree.rs:42 (t=768, 8 atoms) |
| 2.7 | 1215 | — | — | 0.00 | missing | Tree public fn signatures |  |
| 2.8 | 1432 | — | — | 0.00 | missing | main.rs run() — config + flag dispatch |  |
| 2.9 | 1755 | — | — | 0.00 | missing | main.rs run() — content type + data read |  |
| 2.10 | 1929 | — | — | 0.00 | missing | main.rs run() — --to short-circuit |  |
| 2.11 | 2171 | — | — | 0.00 | missing | main.rs run() — size check, Tree, App, ui::start |  |
| 3.2 | 2495 | — | — | 0.72 | partial | App struct fields | pub item at src/ui/app.rs:53 (t=5066, 23 atoms) |
| 3.3 | 2739 | — | — | 0.28 | missing | ElementInFocus + Refresh + ShowResult | pub item at src/ui/app.rs:53 (t=5066, 23 atoms) |
| 3.4 | 3020 | — | — | 0.04 | missing | ui::start event loop | pub-item names surface in src/ui/mod.rs (t=2456, 2 atoms) |
| 3.5 | 3205 | — | — | 0.00 | missing | App method names |  |
| 3.6 | 3560 | — | — | 0.36 | missing | TreeOverview struct + impl method names | pub item at src/ui/tree_overview.rs:19 (t=4442, 12 atoms) |
| 3.7 | 3838 | — | — | 0.00 | missing | TreeOverview::on_key action dispatch |  |
| 3.8 | 4084 | — | — | 0.46 | missing | DataBlock struct + method names | pub item at src/ui/data_block.rs:16 (t=3090, 12 atoms) |
| 3.9 | 4359 | — | — | 0.67 | partial | Filter widget surface | pub-item names surface in src/ui/filter.rs (t=2840, 8 atoms) |
| 3.10 | 4672 | — | — | 0.75 | partial | Footer / Header / Popup surfaces | pub item at src/ui/header.rs:11 (t=2727, 6 atoms) |
| 4.2 | 4844 | — | — | 0.71 | partial | SyntaxToken enum | pub item at src/parse/syntax.rs:11 (t=4299, 12 atoms) |
| 4.3 | 5287 | — | — | 0.00 | missing | Per-parser Parser impl headers |  |
| 4.4 | 5491 | 4083 | -1408 | 0.89 | aligned | Parser::parse_root default body | pub item at src/parse/mod.rs:36 (t=4083, 17 atoms) |
| 4.5 | 5796 | — | — | 0.10 | missing | AnyParser auto-detect parse_root | pub item at src/parse/any.rs:9 (t=3780, 3 atoms) |
| 4.6 | 5872 | 4206 | -1666 | 0.86 | aligned | syntax helper signatures | pub-item names surface in src/parse/syntax.rs (t=4184, 5 atoms) |
| 5.1 | 5888 | 1391 | -4497 | 1.00 | early | config/ module layout |  |
| 5.2 | 6234 | — | — | 0.69 | partial | Config struct field names | pub item at src/config/mod.rs:17 (t=2294, 24 atoms) |
| 5.3 | 6578 | — | — | 0.00 | missing | Action enum variants |  |
| 5.4 | 6976 | — | — | 0.00 | missing | Default key bindings |  |
| 5.5 | 7174 | 2403 | -4771 | 0.80 | early | Key enum + KeyAction | pub item at src/config/keys.rs:195 (t=9579, 64 atoms) |
| 5.6 | 7644 | — | — | 0.76 | partial | Per-branch struct fields (Tree / Layout / Filter / Data) | pub-item names surface in src/config/mod.rs (t=1492, 16 atoms) |
| 5.7 | 7894 | — | — | 0.72 | partial | Editor + Header + Footer struct fields | pub-item names surface in src/config/mod.rs (t=1492, 10 atoms) |
| 5.8 | 8331 | — | — | 0.08 | missing | Config::load + Config::parse | impl method sigs in src/config/mod.rs (t=5725, 6 atoms) |
| 5.9 | 8613 | — | — | 0.04 | missing | Config::get_path resolution | impl method sigs in src/config/mod.rs (t=5725, 2 atoms) |
| 5.10 | 8986 | — | — | 0.70 | partial | Color struct + Colors namespaces | pub item at src/config/colors.rs:162 (t=6862, 24 atoms) |
| 5.11 | 9095 | 5984 | -3111 | 1.00 | early | DataColors fields | pub item at src/config/colors.rs:78 (t=5984, 21 atoms) |
| 5.13 | 9311 | 4595 | -4716 | 0.88 | early | Types config struct | pub item at src/config/types.rs:28 (t=4595, 14 atoms) |
| 6.2 | 9652 | — | — | 0.25 | missing | examples/ + docs/ listings |  |
| 6.3 | 9808 | — | — | 0.62 | partial | Changelog version headings | docs/changelog.md section #3 (t=9818, 9 atoms) |
| 6.4 | 9967 | — | — | 0.00 | missing | clipboard OS routing |  |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 870 | 1.00 | 870 | 9579 | pub item at src/config/keys.rs:195 |
| 599 | 0.66 | 905 | 8611 | pub item at src/cmd.rs:13 |
| 561 | 0.96 | 583 | 6567 | README.md section #3 |
| 364 | 0.86 | 425 | 5725 | impl method sigs in src/config/mod.rs |
| 321 | 1.00 | 321 | 7396 | [dependencies] in Cargo.toml |
| 216 | 0.94 | 231 | 1351 | README.md section #1 |
| 188 | 1.00 | 188 | 7584 | mod/use plumbing in src/ui/mod.rs |
| 177 | 0.92 | 194 | 1120 | README.md section #2 |
| 147 | 0.50 | 295 | 6862 | pub item at src/config/colors.rs:162 |
| 144 | 0.89 | 163 | 9818 | docs/changelog.md section #3 |
| 129 | 0.50 | 259 | 5984 | pub item at src/config/colors.rs:78 |
| 125 | 0.59 | 213 | 7075 | mod/use plumbing in src/main.rs |
| 119 | 1.00 | 119 | 5300 | mod/use plumbing in src/config/mod.rs |
| 115 | 1.00 | 115 | 5181 | mod/use plumbing in src/parse/mod.rs |
| 106 | 0.88 | 122 | 7706 | docs/changelog.md section #1 |
| 91 | 1.00 | 91 | 555 | macro_export bodies across src |
| 87 | 1.00 | 87 | 3346 | pub item at src/config/colors.rs:286 |
| 78 | 0.80 | 98 | 8709 | docs/changelog.md section #2 |
| 76 | 0.50 | 153 | 4595 | pub item at src/config/types.rs:28 |
| 70 | 0.80 | 88 | 9976 | docs/changelog.md section #9 |
| 68 | 0.67 | 103 | 330 | README.md section #4 |
| 65 | 1.00 | 65 | 3259 | pub item at src/config/colors.rs:251 |
| 58 | 0.83 | 70 | 9888 | docs/changelog.md section #10 |
| 58 | 1.00 | 58 | 677 | pub item at src/live_reload.rs:16 |
