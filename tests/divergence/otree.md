scores: Sim=0.349 Reached=19/52 Early=12 Late=5 Partial=10 Missing=23 Over=1 Cap=10000

## Arrival ledger (non-aligned NS batches)

| id | exp_t | seen_t | credit | status | descriptor |
|----|------:|-------:|-------:|:-------|:-----------|
| 1.1 | 63 | — | 0.60 | partial | README lede — one-line product description |
| 1.2 | 98 | 35 | 1.00 | early | Repo top-level listing |
| 1.3 | 136 | 312 | 1.00 | late | src/ listing |
| 1.4 | 200 | 5999 | 1.00 | late | main.rs module declarations |
| 1.5 | 294 | 2967 | 1.00 | late | src/ui/ + src/config/ + src/parse/ listings |
| 1.6 | 433 | 201 | 1.00 | early | Cargo.toml package metadata + description |
| 2.1 | 638 | — | 0.78 | partial | ContentType enum — supported formats |
| 2.2 | 814 | — | 0.00 | missing | get_content_type — extension → ContentType match |
| 2.3 | 1106 | 7699 | 0.96 | late | CommandArgs — CLI flags (one span per `pub` field) |
| 2.4 | 1219 | — | 0.00 | missing | run() signature + backbone landmarks |
| 2.5 | 1562 | — | 0.79 | partial | Parser trait — the format-plugin interface |
| 2.6 | 1745 | — | 0.16 | partial | ContentType::new_parser — format → Parser dispatch |
| 3.1 | 2058 | — | 0.72 | partial | Tree + ItemValue + FieldType structs |
| 3.2 | 2179 | — | 0.00 | missing | Tree::parse + from_value signatures |
| 3.3 | 2314 | — | 0.00 | missing | Tree::build_item signature |
| 4.1 | 2443 | — | 0.71 | partial | SyntaxToken enum variants |
| 4.3 | 2905 | — | 0.00 | missing | Per-format extension + allow_array_root |
| 4.4 | 3072 | — | 0.00 | missing | AnyParser — auto-detect try-each loop |
| 5.1 | 3284 | — | 0.16 | partial | ElementInFocus + Refresh enums |
| 5.2 | 3573 | — | 0.72 | partial | App struct fields |
| 5.3 | 3630 | — | 0.00 | missing | App::on_key signature |
| 5.4 | 3962 | — | 0.00 | missing | Action enum variants — every bindable action |
| 5.6 | 4404 | — | 0.00 | missing | TreeOverview::on_key — action → method dispatch |
| 5.7 | 4543 | 2576 | 0.80 | early | DataBlock struct + scroll state |
| 5.8 | 4750 | 2463 | 0.85 | early | Filter + FilterOptions + FilterTarget + FilterAction |
| 5.9 | 5017 | — | 0.00 | missing | TreeOverview::change_root / reset — root-stack signatures |
| 6.1 | 5363 | — | 0.69 | partial | Config struct — all top-level sections |
| 6.2 | 5640 | 1371 | 1.00 | early | Per-section config structs (Tree/Editor/Layout/Header/Footer/Filter/Data + LayoutDirection) |
| 6.3 | 5930 | — | 0.11 | partial+over | Config constants + load/parse/get_path signatures |
| 6.4 | 6291 | 9941 | 1.00 | late | Keys struct fields — every keybindable action |
| 6.5 | 6442 | 1889 | 0.80 | early | Key enum — every parsable key kind |
| 6.6 | 6682 | — | 0.70 | partial | Colors struct — top-level styled element groups |
| 7.1 | 6762 | — | 0.00 | missing | SyntaxToken::render signature |
| 7.2 | 6833 | 3692 | 0.83 | early | StringValue::new + quote_field_name — shared string rules |
| 7.3 | 7029 | — | 0.00 | missing | Per-format highlighter function signatures |
| 7.4 | 7374 | — | 0.00 | missing | YAML: multi-doc + multiline specifics |
| 7.5 | 7544 | — | 0.00 | missing | TOML highlighter signature + section logic header |
| 8.1 | 7677 | — | 0.71 | partial | Color struct — fg/bg/bold/italic fields |
| 8.2 | 7943 | — | 0.00 | missing | Color::parse — palette-name → ratatui Color |
| 8.3 | 8274 | — | 0.21 | partial | HeaderContext template substitution |
| 8.4 | 8322 | 2064 | 1.00 | early | FooterText enum |
| 8.5 | 8520 | — | 0.45 | partial | Popup struct + on_key dispatch |
| 8.6 | 8603 | — | 0.62 | partial | Edit struct + new() — external editor hand-off |
| 8.7 | 8787 | — | 0.37 | partial | FileWatcher::new + parse_tree signatures |
| 8.8 | 9014 | — | 0.00 | missing | clipboard::get_cmd — platform dispatch |
| 9.1 | 9292 | 6433 | 0.95 | early | Cargo.toml [dependencies] — external crates |
| 9.2 | 9538 | — | 0.00 | missing | docs/actions.md — key-syntax rules + config example |
| 9.3 | 9551 | 234 | 1.00 | early | Every docs/*.md file |
| 9.4 | 9591 | 274 | 1.00 | early | examples/ listing |
| 9.5 | 9612 | 4339 | 1.00 | early | parse/test_cases root listing |

## Walker waste (cost ≥ 50, no NS intersection)

| first_t | cost | key |
|--------:|-----:|:----|
| 414 | 91 | Rust(MacroBodies { src_dir: "/Users/yoav/projects/precis/tests/fixtures/otree/src" }) |
| 2745 | 65 | Rust(PubItem { file: "/Users/yoav/projects/precis/tests/fixtures/otree/src/config/colors.rs", start_line: 251 }) |
| 2832 | 87 | Rust(PubItem { file: "/Users/yoav/projects/precis/tests/fixtures/otree/src/config/colors.rs", start_line: 286 }) |
| 4081 | 153 | Rust(PubItem { file: "/Users/yoav/projects/precis/tests/fixtures/otree/src/config/types.rs", start_line: 28 }) |
| 4132 | 51 | Markdown(Section { file: "/Users/yoav/projects/precis/tests/fixtures/otree/docs/changelog.md", section_index: 0 }) |
| 4688 | 115 | Rust(ModUse { file: "/Users/yoav/projects/precis/tests/fixtures/otree/src/parse/mod.rs" }) |
| 4807 | 119 | Rust(ModUse { file: "/Users/yoav/projects/precis/tests/fixtures/otree/src/config/mod.rs" }) |
| 5491 | 259 | Rust(PubItem { file: "/Users/yoav/projects/precis/tests/fixtures/otree/src/config/colors.rs", start_line: 78 }) |
| 5786 | 295 | Rust(PubItem { file: "/Users/yoav/projects/precis/tests/fixtures/otree/src/config/colors.rs", start_line: 162 }) |
| 6621 | 188 | Rust(ModUse { file: "/Users/yoav/projects/precis/tests/fixtures/otree/src/ui/mod.rs" }) |
| 6672 | 51 | Fs(DirListing { dir: "/Users/yoav/projects/precis/tests/fixtures/otree/src/parse/test_cases/toml" }) |
| 6794 | 122 | Markdown(Section { file: "/Users/yoav/projects/precis/tests/fixtures/otree/docs/changelog.md", section_index: 1 }) |
| 7757 | 58 | Fs(DirListing { dir: "/Users/yoav/projects/precis/tests/fixtures/otree/src/parse/test_cases/hcl" }) |
| 7855 | 98 | Markdown(Section { file: "/Users/yoav/projects/precis/tests/fixtures/otree/docs/changelog.md", section_index: 2 }) |
| 9071 | 79 | Fs(DirListing { dir: "/Users/yoav/projects/precis/tests/fixtures/otree/src/parse/test_cases/yaml" }) |
