scores: Sim=0.434 Reached=18/50 Early=7 Late=5 Partial=12 Missing=20 Used=9152/10000

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
| 1.3 | 92 | 424 | +332 | 1.00 | late | src/ module layout |  |
| 1.4 | 156 | 5824 | +5668 | 1.00 | late | main.rs module declarations | mod/use plumbing in src/main.rs (t=5824, 9 atoms) |
| 1.5 | 185 | 279 | +94 | 1.00 | late | README section headings | README.md section #5 (t=8066, 27 atoms) |
| 1.6 | 329 | 214 | -115 | 0.92 | early | Cargo package metadata | [package] in Cargo.toml (t=214, 11 atoms) |
| 2.1 | 391 | 3501 | +3110 | 1.00 | late | ContentType variants — locations only | pub item at src/parse/mod.rs:18 (t=3501, 13 atoms) |
| 2.2 | 534 | — | — | 0.60 | partial | ContentType enum context | pub item at src/parse/mod.rs:18 (t=3501, 14 atoms) |
| 2.3 | 653 | 3851 | +3198 | 1.00 | late | Parser trait method signatures | pub item at src/parse/mod.rs:36 (t=3851, 23 atoms) |
| 2.4 | 836 | — | — | 0.16 | missing | ContentType::new_parser dispatch | impl method sigs in src/parse/mod.rs (t=3529, 3 atoms) |
| 2.5 | 931 | — | — | 0.70 | partial | Tree struct fields | pub item at src/tree.rs:14 (t=949, 7 atoms) |
| 2.6 | 1144 | — | — | 0.76 | partial | ItemValue + FieldType | pub item at src/tree.rs:42 (t=733, 8 atoms) |
| 2.7 | 1215 | — | — | 0.00 | missing | Tree public fn signatures |  |
| 2.8 | 1432 | — | — | 0.00 | missing | main.rs run() — config + flag dispatch |  |
| 2.9 | 1755 | — | — | 0.00 | missing | main.rs run() — content type + data read |  |
| 2.10 | 1929 | — | — | 0.00 | missing | main.rs run() — --to short-circuit |  |
| 2.11 | 2171 | — | — | 0.00 | missing | main.rs run() — size check, Tree, App, ui::start |  |
| 3.2 | 2495 | — | — | 0.72 | partial | App struct fields | pub item at src/ui/app.rs:53 (t=5611, 23 atoms) |
| 3.3 | 2739 | — | — | 0.28 | missing | ElementInFocus + Refresh + ShowResult | pub item at src/ui/app.rs:53 (t=5611, 23 atoms) |
| 3.4 | 3020 | — | — | 0.04 | missing | ui::start event loop | pub-item names surface in src/ui/mod.rs (t=2041, 2 atoms) |
| 3.5 | 3205 | — | — | 0.00 | missing | App method names |  |
| 3.6 | 3560 | — | — | 0.36 | missing | TreeOverview struct + impl method names | pub item at src/ui/tree_overview.rs:19 (t=4323, 12 atoms) |
| 3.7 | 3838 | — | — | 0.00 | missing | TreeOverview::on_key action dispatch |  |
| 3.8 | 4084 | — | — | 0.46 | missing | DataBlock struct + method names | pub item at src/ui/data_block.rs:16 (t=4180, 12 atoms) |
| 3.9 | 4359 | — | — | 0.67 | partial | Filter widget surface | pub-item names surface in src/ui/filter.rs (t=2425, 8 atoms) |
| 3.10 | 4672 | — | — | 0.75 | partial | Footer / Header / Popup surfaces | pub item at src/ui/header.rs:11 (t=2382, 6 atoms) |
| 4.1 | 4715 | 3249 | -1466 | 1.00 | early | parse/ module layout |  |
| 4.2 | 4844 | — | — | 0.71 | partial | SyntaxToken enum | pub item at src/parse/syntax.rs:11 (t=4067, 12 atoms) |
| 4.3 | 5287 | — | — | 0.00 | missing | Per-parser Parser impl headers |  |
| 4.4 | 5491 | 3851 | -1640 | 0.89 | aligned | Parser::parse_root default body | pub item at src/parse/mod.rs:36 (t=3851, 17 atoms) |
| 4.5 | 5796 | — | — | 0.10 | missing | AnyParser auto-detect parse_root | pub item at src/parse/any.rs:9 (t=3548, 3 atoms) |
| 4.6 | 5872 | 3974 | -1898 | 0.86 | early | syntax helper signatures | pub-item names surface in src/parse/syntax.rs (t=3952, 5 atoms) |
| 5.1 | 5888 | 1085 | -4803 | 1.00 | early | config/ module layout |  |
| 5.2 | 6234 | — | — | 0.69 | partial | Config struct field names | pub item at src/config/mod.rs:17 (t=1988, 24 atoms) |
| 5.3 | 6578 | — | — | 0.00 | missing | Action enum variants |  |
| 5.4 | 6976 | — | — | 0.00 | missing | Default key bindings |  |
| 5.5 | 7174 | 2671 | -4503 | 0.80 | early | Key enum + KeyAction | pub item at src/config/keys.rs:54 (t=2671, 16 atoms) |
| 5.6 | 7644 | — | — | 0.76 | partial | Per-branch struct fields (Tree / Layout / Filter / Data) | pub-item names surface in src/config/mod.rs (t=1186, 16 atoms) |
| 5.7 | 7894 | — | — | 0.72 | partial | Editor + Header + Footer struct fields | pub-item names surface in src/config/mod.rs (t=1186, 10 atoms) |
| 5.8 | 8331 | — | — | 0.08 | missing | Config::load + Config::parse | impl method sigs in src/config/mod.rs (t=5321, 6 atoms) |
| 5.9 | 8613 | — | — | 0.04 | missing | Config::get_path resolution | impl method sigs in src/config/mod.rs (t=5321, 2 atoms) |
| 5.10 | 8986 | — | — | 0.70 | partial | Color struct + Colors namespaces | pub item at src/config/colors.rs:162 (t=6769, 24 atoms) |
| 5.11 | 9095 | 6083 | -3012 | 1.00 | early | DataColors fields | pub item at src/config/colors.rs:78 (t=6083, 21 atoms) |
| 5.13 | 9311 | 4476 | -4835 | 0.88 | early | Types config struct | pub item at src/config/types.rs:28 (t=4476, 14 atoms) |
| 6.2 | 9652 | — | — | 0.25 | missing | examples/ + docs/ listings |  |
| 6.4 | 9967 | — | — | 0.00 | missing | clipboard OS routing |  |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 6 | 975 | README.md section #<n> |
| 4 | 428 | pub item at src/config/colors.rs:<n> |
| 3 | 281 | docs/changelog.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 599 | 0.66 | 905 | 8971 | pub item at src/cmd.rs:13 |
| 553 | 0.96 | 575 | 8066 | README.md section #5 |
| 364 | 0.86 | 425 | 5321 | impl method sigs in src/config/mod.rs |
| 321 | 1.00 | 321 | 6474 | [dependencies] in Cargo.toml |
| 246 | 0.77 | 321 | 7278 | headings outline in docs/changelog.md |
| 188 | 1.00 | 188 | 6957 | mod/use plumbing in src/ui/mod.rs |
| 171 | 0.92 | 187 | 3206 | README.md section #4 |
| 147 | 0.50 | 295 | 6769 | pub item at src/config/colors.rs:162 |
| 129 | 0.50 | 259 | 6083 | pub item at src/config/colors.rs:78 |
| 128 | 0.89 | 144 | 9136 | docs/changelog.md section #3 |
| 125 | 0.59 | 213 | 5824 | mod/use plumbing in src/main.rs |
| 119 | 1.00 | 119 | 4896 | mod/use plumbing in src/config/mod.rs |
| 115 | 1.00 | 115 | 4591 | mod/use plumbing in src/parse/mod.rs |
| 91 | 1.00 | 91 | 578 | macro_export bodies across src |
| 90 | 0.88 | 103 | 7412 | docs/changelog.md section #1 |
| 87 | 1.00 | 87 | 2927 | pub item at src/config/colors.rs:286 |
| 76 | 0.50 | 153 | 4476 | pub item at src/config/types.rs:28 |
| 70 | 1.00 | 70 | 6153 | README.md section #2 |
| 65 | 1.00 | 65 | 2840 | pub item at src/config/colors.rs:251 |
| 64 | 0.67 | 96 | 1045 | README.md section #6 |
| 63 | 0.80 | 79 | 7491 | docs/changelog.md section #2 |
| 61 | 0.83 | 74 | 353 | README.md section #1 |
| 58 | 1.00 | 58 | 817 | pub item at src/live_reload.rs:16 |
| 56 | 1.00 | 56 | 5377 | README.md section #3 |
