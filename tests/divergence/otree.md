scores: Sim=0.351 Reached=19/52 Early=12 Late=5 Partial=10 Missing=23 Used=9982/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 6 | 5 | 1 | 0 | 0.93 |
| 2 | 6 | 1 | 2 | 3 | 0.45 |
| 3 | 3 | 0 | 1 | 2 | 0.24 |
| 4 | 4 | 1 | 1 | 2 | 0.43 |
| 5 | 9 | 3 | 1 | 5 | 0.39 |
| 6 | 6 | 3 | 2 | 1 | 0.71 |
| 7 | 5 | 1 | 0 | 4 | 0.17 |
| 8 | 8 | 1 | 2 | 5 | 0.42 |
| 9 | 5 | 4 | 0 | 1 | 0.79 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor |
|----|------:|----------:|--------:|-------:|:-------|:-----------|
| 1.1 | 63 | — | — | 0.60 | partial | README lede — one-line product description |
| 1.2 | 98 | 35 | -63 | 1.00 | early | Repo top-level listing |
| 1.3 | 136 | 312 | +176 | 1.00 | late | src/ listing |
| 1.4 | 200 | 5999 | +5799 | 1.00 | late | main.rs module declarations |
| 1.5 | 294 | 2967 | +2673 | 1.00 | late | src/ui/ + src/config/ + src/parse/ listings |
| 1.6 | 433 | 201 | -232 | 1.00 | early | Cargo.toml package metadata + description |
| 2.1 | 638 | — | — | 0.78 | partial | ContentType enum — supported formats |
| 2.2 | 814 | — | — | 0.00 | missing | get_content_type — extension → ContentType match |
| 2.3 | 1106 | 7699 | +6593 | 0.96 | late | CommandArgs — CLI flags (one span per `pub` field) |
| 2.4 | 1219 | — | — | 0.00 | missing | run() signature + backbone landmarks |
| 2.5 | 1562 | — | — | 0.79 | partial | Parser trait — the format-plugin interface |
| 2.6 | 1745 | — | — | 0.16 | missing | ContentType::new_parser — format → Parser dispatch |
| 3.1 | 2058 | — | — | 0.72 | partial | Tree + ItemValue + FieldType structs |
| 3.2 | 2179 | — | — | 0.00 | missing | Tree::parse + from_value signatures |
| 3.3 | 2314 | — | — | 0.00 | missing | Tree::build_item signature |
| 4.1 | 2443 | — | — | 0.71 | partial | SyntaxToken enum variants |
| 4.3 | 2905 | — | — | 0.00 | missing | Per-format extension + allow_array_root |
| 4.4 | 3072 | — | — | 0.00 | missing | AnyParser — auto-detect try-each loop |
| 5.1 | 3284 | — | — | 0.16 | missing | ElementInFocus + Refresh enums |
| 5.2 | 3573 | — | — | 0.72 | partial | App struct fields |
| 5.3 | 3630 | — | — | 0.00 | missing | App::on_key signature |
| 5.4 | 3962 | — | — | 0.00 | missing | Action enum variants — every bindable action |
| 5.6 | 4404 | — | — | 0.00 | missing | TreeOverview::on_key — action → method dispatch |
| 5.7 | 4543 | 2576 | -1967 | 0.80 | early | DataBlock struct + scroll state |
| 5.8 | 4750 | 2463 | -2287 | 0.85 | early | Filter + FilterOptions + FilterTarget + FilterAction |
| 5.9 | 5017 | — | — | 0.00 | missing | TreeOverview::change_root / reset — root-stack signatures |
| 6.1 | 5363 | — | — | 0.69 | partial | Config struct — all top-level sections |
| 6.2 | 5640 | 1371 | -4269 | 1.00 | early | Per-section config structs (Tree/Editor/Layout/Header/Footer/Filter/Data + LayoutDirection) |
| 6.3 | 5930 | — | — | 0.11 | missing | Config constants + load/parse/get_path signatures |
| 6.4 | 6291 | 9941 | +3650 | 1.00 | late | Keys struct fields — every keybindable action |
| 6.5 | 6442 | 1889 | -4553 | 0.80 | early | Key enum — every parsable key kind |
| 6.6 | 6682 | — | — | 0.70 | partial | Colors struct — top-level styled element groups |
| 7.1 | 6762 | — | — | 0.00 | missing | SyntaxToken::render signature |
| 7.2 | 6833 | 3692 | -3141 | 0.83 | early | StringValue::new + quote_field_name — shared string rules |
| 7.3 | 7029 | — | — | 0.00 | missing | Per-format highlighter function signatures |
| 7.4 | 7374 | — | — | 0.00 | missing | YAML: multi-doc + multiline specifics |
| 7.5 | 7544 | — | — | 0.00 | missing | TOML highlighter signature + section logic header |
| 8.1 | 7677 | — | — | 0.71 | partial | Color struct — fg/bg/bold/italic fields |
| 8.2 | 7943 | — | — | 0.00 | missing | Color::parse — palette-name → ratatui Color |
| 8.3 | 8274 | — | — | 0.21 | missing | HeaderContext template substitution |
| 8.4 | 8322 | 2064 | -6258 | 1.00 | early | FooterText enum |
| 8.5 | 8520 | — | — | 0.45 | missing | Popup struct + on_key dispatch |
| 8.6 | 8603 | — | — | 0.62 | partial | Edit struct + new() — external editor hand-off |
| 8.7 | 8787 | — | — | 0.37 | missing | FileWatcher::new + parse_tree signatures |
| 8.8 | 9014 | — | — | 0.00 | missing | clipboard::get_cmd — platform dispatch |
| 9.1 | 9292 | 6433 | -2859 | 0.95 | early | Cargo.toml [dependencies] — external crates |
| 9.2 | 9538 | — | — | 0.00 | missing | docs/actions.md — key-syntax rules + config example |
| 9.3 | 9551 | 234 | -9317 | 1.00 | early | Every docs/*.md file |
| 9.4 | 9591 | 274 | -9317 | 1.00 | early | examples/ listing |
| 9.5 | 9612 | 4339 | -5273 | 1.00 | early | parse/test_cases root listing |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 1117 | 0.98 | 1137 | 8992 | README.md section #0 |
| 611 | 0.68 | 905 | 7699 | pub item at src/cmd.rs:13 |
| 462 | 0.53 | 870 | 9941 | pub item at src/config/keys.rs:195 |
| 364 | 0.86 | 425 | 5232 | impl method sigs in src/config/mod.rs |
| 295 | 1.00 | 295 | 5786 | pub item at src/config/colors.rs:162 |
| 259 | 1.00 | 259 | 5491 | pub item at src/config/colors.rs:78 |
| 188 | 1.00 | 188 | 6621 | mod/use plumbing in src/ui/mod.rs |
| 153 | 1.00 | 153 | 4081 | pub item at src/config/types.rs:28 |
| 125 | 0.59 | 213 | 5999 | mod/use plumbing in src/main.rs |
| 122 | 1.00 | 122 | 6794 | docs/changelog.md section #1 |
| 119 | 1.00 | 119 | 4807 | mod/use plumbing in src/config/mod.rs |
| 115 | 1.00 | 115 | 4688 | mod/use plumbing in src/parse/mod.rs |
| 98 | 1.00 | 98 | 7855 | docs/changelog.md section #2 |
| 91 | 1.00 | 91 | 414 | macro_export bodies across src |
| 87 | 1.00 | 87 | 2832 | pub item at src/config/colors.rs:286 |
| 79 | 1.00 | 79 | 9071 | listing of 'src/parse/test_cases/yaml' |
| 65 | 1.00 | 65 | 2745 | pub item at src/config/colors.rs:251 |
| 58 | 1.00 | 58 | 7757 | listing of 'src/parse/test_cases/hcl' |
| 53 | 0.71 | 75 | 2651 | pub-item names surface in src/config/colors.rs |
| 51 | 1.00 | 51 | 4132 | docs/changelog.md section #0 |
| 51 | 1.00 | 51 | 6672 | listing of 'src/parse/test_cases/toml' |
