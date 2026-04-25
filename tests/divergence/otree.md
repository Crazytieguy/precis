scores: Sim=0.420 Reached=18/50 Early=10 Late=5 Partial=12 Missing=20 Used=9982/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 6 | 5 | 1 | 0 | 0.90 |
| 2 | 11 | 2 | 3 | 6 | 0.38 |
| 3 | 10 | 1 | 3 | 6 | 0.43 |
| 4 | 6 | 3 | 1 | 2 | 0.59 |
| 5 | 13 | 5 | 4 | 4 | 0.59 |
| 6 | 4 | 2 | 0 | 2 | 0.58 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 54 | — | — | 0.50 | partial | README H1 + crate one-liner | README headline in README.md (t=62, 2 atoms) |
| 1.3 | 92 | 312 | +220 | 1.00 | late | src/ module layout |  |
| 1.4 | 156 | 5999 | +5843 | 1.00 | late | main.rs module declarations | mod/use plumbing in src/main.rs (t=5999, 9 atoms) |
| 1.5 | 185 | 8992 | +8807 | 1.00 | late | README section headings | README.md section #0 (t=8992, 56 atoms) |
| 1.6 | 329 | 201 | -128 | 0.92 | early | Cargo package metadata | [package] in Cargo.toml (t=201, 11 atoms) |
| 2.1 | 391 | 3219 | +2828 | 1.00 | late | ContentType variants — locations only | pub item at src/parse/mod.rs:18 (t=3219, 13 atoms) |
| 2.2 | 534 | — | — | 0.60 | partial | ContentType enum context | pub item at src/parse/mod.rs:18 (t=3219, 14 atoms) |
| 2.3 | 653 | 3569 | +2916 | 1.00 | late | Parser trait method signatures | pub item at src/parse/mod.rs:36 (t=3569, 23 atoms) |
| 2.4 | 836 | — | — | 0.16 | missing | ContentType::new_parser dispatch | impl method sigs in src/parse/mod.rs (t=3247, 3 atoms) |
| 2.5 | 931 | — | — | 0.70 | partial | Tree struct fields | pub item at src/tree.rs:14 (t=837, 7 atoms) |
| 2.6 | 1144 | — | — | 0.76 | partial | ItemValue + FieldType | pub item at src/tree.rs:42 (t=679, 8 atoms) |
| 2.7 | 1215 | — | — | 0.00 | missing | Tree public fn signatures |  |
| 2.8 | 1432 | — | — | 0.00 | missing | main.rs run() — config + flag dispatch |  |
| 2.9 | 1755 | — | — | 0.00 | missing | main.rs run() — content type + data read |  |
| 2.10 | 1929 | — | — | 0.00 | missing | main.rs run() — --to short-circuit |  |
| 2.11 | 2171 | — | — | 0.00 | missing | main.rs run() — size check, Tree, App, ui::start |  |
| 3.2 | 2495 | — | — | 0.72 | partial | App struct fields | pub item at src/ui/app.rs:53 (t=4573, 23 atoms) |
| 3.3 | 2739 | — | — | 0.28 | missing | ElementInFocus + Refresh + ShowResult | pub item at src/ui/app.rs:53 (t=4573, 23 atoms) |
| 3.4 | 3020 | — | — | 0.04 | missing | ui::start event loop | pub-item names surface in src/ui/mod.rs (t=1942, 2 atoms) |
| 3.5 | 3205 | — | — | 0.00 | missing | App method names |  |
| 3.6 | 3560 | — | — | 0.36 | missing | TreeOverview struct + impl method names | pub item at src/ui/tree_overview.rs:19 (t=3928, 12 atoms) |
| 3.7 | 3838 | — | — | 0.00 | missing | TreeOverview::on_key action dispatch |  |
| 3.8 | 4084 | — | — | 0.46 | missing | DataBlock struct + method names | pub item at src/ui/data_block.rs:16 (t=2576, 12 atoms) |
| 3.9 | 4359 | — | — | 0.67 | partial | Filter widget surface | pub-item names surface in src/ui/filter.rs (t=2326, 8 atoms) |
| 3.10 | 4672 | — | — | 0.75 | partial | Footer / Header / Popup surfaces | pub item at src/ui/header.rs:11 (t=2213, 6 atoms) |
| 4.1 | 4715 | 2967 | -1748 | 1.00 | early | parse/ module layout |  |
| 4.2 | 4844 | — | — | 0.71 | partial | SyntaxToken enum | pub item at src/parse/syntax.rs:11 (t=3785, 12 atoms) |
| 4.3 | 5287 | — | — | 0.00 | missing | Per-parser Parser impl headers |  |
| 4.4 | 5491 | 3569 | -1922 | 0.89 | early | Parser::parse_root default body | pub item at src/parse/mod.rs:36 (t=3569, 17 atoms) |
| 4.5 | 5796 | — | — | 0.10 | missing | AnyParser auto-detect parse_root | pub item at src/parse/any.rs:9 (t=3266, 3 atoms) |
| 4.6 | 5872 | 3692 | -2180 | 0.86 | early | syntax helper signatures | pub-item names surface in src/parse/syntax.rs (t=3670, 5 atoms) |
| 5.1 | 5888 | 877 | -5011 | 1.00 | early | config/ module layout |  |
| 5.2 | 6234 | — | — | 0.69 | partial | Config struct field names | pub item at src/config/mod.rs:17 (t=1780, 24 atoms) |
| 5.3 | 6578 | — | — | 0.00 | missing | Action enum variants |  |
| 5.4 | 6976 | — | — | 0.00 | missing | Default key bindings |  |
| 5.5 | 7174 | 1889 | -5285 | 0.80 | early | Key enum + KeyAction | pub item at src/config/keys.rs:195 (t=9941, 64 atoms) |
| 5.6 | 7644 | — | — | 0.76 | partial | Per-branch struct fields (Tree / Layout / Filter / Data) | pub-item names surface in src/config/mod.rs (t=978, 16 atoms) |
| 5.7 | 7894 | — | — | 0.72 | partial | Editor + Header + Footer struct fields | pub-item names surface in src/config/mod.rs (t=978, 10 atoms) |
| 5.8 | 8331 | — | — | 0.08 | missing | Config::load + Config::parse | impl method sigs in src/config/mod.rs (t=5232, 6 atoms) |
| 5.9 | 8613 | — | — | 0.04 | missing | Config::get_path resolution | impl method sigs in src/config/mod.rs (t=5232, 2 atoms) |
| 5.10 | 8986 | — | — | 0.70 | partial | Color struct + Colors namespaces | pub item at src/config/colors.rs:162 (t=5786, 24 atoms) |
| 5.11 | 9095 | 5491 | -3604 | 1.00 | early | DataColors fields | pub item at src/config/colors.rs:78 (t=5491, 21 atoms) |
| 5.12 | 9221 | 5786 | -3435 | 1.00 | early | TreeColors fields | pub item at src/config/colors.rs:162 (t=5786, 23 atoms) |
| 5.13 | 9311 | 4081 | -5230 | 0.88 | early | Types config struct | pub item at src/config/types.rs:28 (t=4081, 14 atoms) |
| 6.2 | 9652 | 274 | -9378 | 1.00 | early | examples/ + docs/ listings |  |
| 6.3 | 9808 | — | — | 0.31 | missing | Changelog version headings | docs/changelog.md section #1 (t=6794, 8 atoms) |
| 6.4 | 9967 | — | — | 0.00 | missing | clipboard OS routing |  |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 1059 | 0.93 | 1137 | 8992 | README.md section #0 |
| 870 | 1.00 | 870 | 9941 | pub item at src/config/keys.rs:195 |
| 599 | 0.66 | 905 | 7699 | pub item at src/cmd.rs:13 |
| 364 | 0.86 | 425 | 5232 | impl method sigs in src/config/mod.rs |
| 321 | 1.00 | 321 | 6433 | [dependencies] in Cargo.toml |
| 188 | 1.00 | 188 | 6621 | mod/use plumbing in src/ui/mod.rs |
| 147 | 0.50 | 295 | 5786 | pub item at src/config/colors.rs:162 |
| 129 | 0.50 | 259 | 5491 | pub item at src/config/colors.rs:78 |
| 125 | 0.59 | 213 | 5999 | mod/use plumbing in src/main.rs |
| 119 | 1.00 | 119 | 4807 | mod/use plumbing in src/config/mod.rs |
| 115 | 1.00 | 115 | 4688 | mod/use plumbing in src/parse/mod.rs |
| 106 | 0.88 | 122 | 6794 | docs/changelog.md section #1 |
| 91 | 1.00 | 91 | 414 | macro_export bodies across src |
| 87 | 1.00 | 87 | 2832 | pub item at src/config/colors.rs:286 |
| 79 | 1.00 | 79 | 9071 | listing of 'src/parse/test_cases/yaml' |
| 78 | 0.80 | 98 | 7855 | docs/changelog.md section #2 |
| 76 | 0.50 | 153 | 4081 | pub item at src/config/types.rs:28 |
| 65 | 1.00 | 65 | 2745 | pub item at src/config/colors.rs:251 |
| 58 | 1.00 | 58 | 7757 | listing of 'src/parse/test_cases/hcl' |
| 58 | 1.00 | 58 | 588 | pub item at src/live_reload.rs:16 |
| 51 | 1.00 | 51 | 6672 | listing of 'src/parse/test_cases/toml' |
