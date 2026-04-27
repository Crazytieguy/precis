scores: Sim=0.433 Reached=18/50 Early=8 Late=5 Partial=12 Missing=20 Used=9476/10000

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
| 1.2 | 54 | — | — | 0.50 | partial | README H1 + crate one-liner | README headline in README.md (t=79, 1 atoms) |
| 1.3 | 92 | 374 | +282 | 1.00 | late | src/ module layout |  |
| 1.4 | 156 | 5581 | +5425 | 1.00 | late | main.rs module declarations | mod/use plumbing in src/main.rs (t=5581, 9 atoms) |
| 1.5 | 185 | 320 | +135 | 1.00 | late | README section headings | README.md section #4 (t=7220, 12 atoms) |
| 1.6 | 329 | 255 | -74 | 0.92 | aligned | Cargo package metadata | [package] in Cargo.toml (t=255, 11 atoms) |
| 2.1 | 391 | 3100 | +2709 | 1.00 | late | ContentType variants — locations only | pub item at src/parse/mod.rs:18 (t=3100, 13 atoms) |
| 2.2 | 534 | — | — | 0.60 | partial | ContentType enum context | pub item at src/parse/mod.rs:18 (t=3100, 14 atoms) |
| 2.3 | 653 | 3403 | +2750 | 1.00 | late | Parser trait method signatures | pub item at src/parse/mod.rs:36 (t=3403, 23 atoms) |
| 2.4 | 836 | — | — | 0.16 | missing | ContentType::new_parser dispatch | impl method sigs in src/parse/mod.rs (t=2850, 3 atoms) |
| 2.5 | 931 | — | — | 0.70 | partial | Tree struct fields | pub item at src/tree.rs:14 (t=1455, 7 atoms) |
| 2.6 | 1144 | — | — | 0.76 | partial | ItemValue + FieldType | pub item at src/tree.rs:42 (t=618, 8 atoms) |
| 2.7 | 1215 | — | — | 0.00 | missing | Tree public fn signatures |  |
| 2.8 | 1432 | — | — | 0.00 | missing | main.rs run() — config + flag dispatch |  |
| 2.9 | 1755 | — | — | 0.00 | missing | main.rs run() — content type + data read |  |
| 2.10 | 1929 | — | — | 0.00 | missing | main.rs run() — --to short-circuit |  |
| 2.11 | 2171 | — | — | 0.00 | missing | main.rs run() — size check, Tree, App, ui::start |  |
| 3.2 | 2495 | — | — | 0.72 | partial | App struct fields | pub item at src/ui/app.rs:53 (t=5272, 23 atoms) |
| 3.3 | 2739 | — | — | 0.28 | missing | ElementInFocus + Refresh + ShowResult | pub item at src/ui/app.rs:53 (t=5272, 23 atoms) |
| 3.4 | 3020 | — | — | 0.04 | missing | ui::start event loop | pub-item names surface in src/ui/mod.rs (t=1780, 2 atoms) |
| 3.5 | 3205 | — | — | 0.00 | missing | App method names |  |
| 3.6 | 3560 | — | — | 0.36 | missing | TreeOverview struct + impl method names | pub item at src/ui/tree_overview.rs:19 (t=4040, 12 atoms) |
| 3.7 | 3838 | — | — | 0.00 | missing | TreeOverview::on_key action dispatch |  |
| 3.8 | 4084 | — | — | 0.46 | missing | DataBlock struct + method names | pub item at src/ui/data_block.rs:16 (t=3897, 12 atoms) |
| 3.9 | 4359 | — | — | 0.67 | partial | Filter widget surface | pub-item names surface in src/ui/filter.rs (t=2164, 8 atoms) |
| 3.10 | 4672 | — | — | 0.75 | partial | Footer / Header / Popup surfaces | pub item at src/ui/header.rs:11 (t=2121, 6 atoms) |
| 4.1 | 4715 | 2801 | -1914 | 1.00 | early | parse/ module layout |  |
| 4.2 | 4844 | — | — | 0.71 | partial | SyntaxToken enum | pub item at src/parse/syntax.rs:11 (t=3619, 12 atoms) |
| 4.3 | 5287 | — | — | 0.00 | missing | Per-parser Parser impl headers |  |
| 4.4 | 5491 | 3403 | -2088 | 0.89 | early | Parser::parse_root default body | pub item at src/parse/mod.rs:36 (t=3403, 17 atoms) |
| 4.5 | 5796 | — | — | 0.10 | missing | AnyParser auto-detect parse_root | pub item at src/parse/any.rs:9 (t=2939, 3 atoms) |
| 4.6 | 5872 | 3526 | -2346 | 0.86 | early | syntax helper signatures | pub-item names surface in src/parse/syntax.rs (t=3504, 5 atoms) |
| 5.1 | 5888 | 754 | -5134 | 1.00 | early | config/ module layout |  |
| 5.2 | 6234 | — | — | 0.69 | partial | Config struct field names | pub item at src/config/mod.rs:17 (t=1727, 24 atoms) |
| 5.3 | 6578 | — | — | 0.00 | missing | Action enum variants |  |
| 5.4 | 6976 | — | — | 0.00 | missing | Default key bindings |  |
| 5.5 | 7174 | 2410 | -4764 | 0.80 | early | Key enum + KeyAction | pub item at src/config/keys.rs:195 (t=9153, 64 atoms) |
| 5.6 | 7644 | — | — | 0.76 | partial | Per-branch struct fields (Tree / Layout / Filter / Data) | pub-item names surface in src/config/mod.rs (t=855, 16 atoms) |
| 5.7 | 7894 | — | — | 0.72 | partial | Editor + Header + Footer struct fields | pub-item names surface in src/config/mod.rs (t=855, 10 atoms) |
| 5.8 | 8331 | — | — | 0.08 | missing | Config::load + Config::parse | impl method sigs in src/config/mod.rs (t=5038, 6 atoms) |
| 5.9 | 8613 | — | — | 0.04 | missing | Config::get_path resolution | impl method sigs in src/config/mod.rs (t=5038, 2 atoms) |
| 5.10 | 8986 | — | — | 0.70 | partial | Color struct + Colors namespaces | pub item at src/config/colors.rs:162 (t=6808, 24 atoms) |
| 5.11 | 9095 | 5840 | -3255 | 1.00 | early | DataColors fields | pub item at src/config/colors.rs:78 (t=5840, 21 atoms) |
| 5.13 | 9311 | 4193 | -5118 | 0.88 | early | Types config struct | pub item at src/config/types.rs:28 (t=4193, 14 atoms) |
| 6.2 | 9652 | — | — | 0.25 | missing | examples/ + docs/ listings |  |
| 6.3 | 9808 | 6482 | -3326 | 1.00 | early | Changelog version headings | headings outline in docs/changelog.md (t=6482, 53 atoms) |
| 6.4 | 9967 | — | — | 0.00 | missing | clipboard OS routing |  |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 4 | 489 | pub item at src/config/colors.rs:<n> |
| 5 | 483 | README.md section #<n> |
| 2 | 182 | docs/changelog.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 870 | 1.00 | 870 | 9153 | pub item at src/config/keys.rs:195 |
| 626 | 0.69 | 905 | 8125 | pub item at src/cmd.rs:13 |
| 348 | 0.82 | 425 | 5038 | impl method sigs in src/config/mod.rs |
| 321 | 1.00 | 321 | 6161 | [dependencies] in Cargo.toml |
| 188 | 1.00 | 188 | 6996 | mod/use plumbing in src/ui/mod.rs |
| 187 | 1.00 | 187 | 7220 | README.md section #4 |
| 178 | 0.60 | 295 | 6808 | pub item at src/config/colors.rs:162 |
| 165 | 0.51 | 321 | 6482 | headings outline in docs/changelog.md |
| 159 | 0.61 | 259 | 5840 | pub item at src/config/colors.rs:78 |
| 149 | 0.70 | 213 | 5581 | mod/use plumbing in src/main.rs |
| 119 | 1.00 | 119 | 4613 | mod/use plumbing in src/config/mod.rs |
| 115 | 1.00 | 115 | 4308 | mod/use plumbing in src/parse/mod.rs |
| 103 | 1.00 | 103 | 9354 | docs/changelog.md section #1 |
| 102 | 1.00 | 102 | 8283 | plaintext config .gitignore |
| 96 | 1.00 | 96 | 5368 | README.md section #6 |
| 93 | 0.61 | 153 | 4193 | pub item at src/config/types.rs:28 |
| 91 | 1.00 | 91 | 3784 | macro_export body at src/debug.rs:8 |
| 87 | 1.00 | 87 | 2666 | pub item at src/config/colors.rs:286 |
| 79 | 1.00 | 79 | 9433 | docs/changelog.md section #2 |
| 74 | 1.00 | 74 | 3693 | README.md section #1 |
| 70 | 1.00 | 70 | 9223 | README.md section #2 |
| 65 | 1.00 | 65 | 2579 | pub item at src/config/colors.rs:251 |
| 58 | 1.00 | 58 | 676 | pub item at src/live_reload.rs:16 |
| 56 | 1.00 | 56 | 8181 | README.md section #3 |
