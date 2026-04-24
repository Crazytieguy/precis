scores: Sim=0.373 Reached=12/48 Early=1 Late=8 Missing=29 Over=1 Cap=10000

## Arrival ledger (non-aligned NS batches)

| id | exp_t | seen_t | credit | status | descriptor |
|----|------:|-------:|-------:|:-------|:-----------|
| 1.2 | 98 | — | 0.60 | partial | README lede — what otree is |
| 1.4 | 197 | 312 | 1.00 | late | src/ listing |
| 1.5 | 261 | 5999 | 1.00 | late | Crate-root module declarations |
| 1.6 | 296 | 1924 | 1.00 | late | src/ui listing |
| 1.7 | 339 | 2967 | 1.00 | late | src/parse listing |
| 1.8 | 355 | 877 | 1.00 | late | src/config listing |
| 1.9 | 453 | 8992 | 0.80 | late | README usage — invocation examples |
| 1.10 | 557 | — | 0.71 | partial | ContentType enum — the six supported formats |
| 1.11 | 662 | — | 0.56 | partial | Parser trait — required methods |
| 1.12 | 834 | — | 0.00 | missing | Action enum — first half (navigation/layout) |
| 1.13 | 1006 | — | 0.00 | missing | Action enum — second half (filter/edit/copy/help) |
| 1.14 | 1226 | — | 0.00 | missing | Default key bindings — navigation half |
| 1.15 | 1404 | — | 0.00 | missing | Default key bindings — filter/edit/copy/help half |
| 1.16 | 1555 | 1889 | 0.80 | aligned | Key enum — keyboard-key vocabulary |
| 1.17 | 1806 | — | 0.45 | partial | FieldType + ContentType::new_parser dispatch |
| 1.18 | 1935 | — | 0.71 | partial | SyntaxToken enum — the shared highlight IR |
| 2.1 | 2281 | — | 0.69 | partial | Config struct — top-level config sections |
| 2.2 | 2464 | — | 0.70 | partial | Tree struct + ItemValue struct |
| 2.3 | 2920 | — | 0.45 | partial | App struct + ElementInFocus / Refresh enums |
| 2.4 | 3112 | 7699 | 1.00 | late+over | CLI flag names — one line per arg |
| 2.5 | 3294 | — | 0.29 | partial | Default Config constants — size bounds + max_data_size |
| 2.6 | 3823 | — | 0.29 | partial | Colors top-level sections |
| 2.7 | 4217 | — | 0.28 | partial | Color struct — fg/bg/bold/italic fields |
| 2.8 | 4446 | — | 0.64 | partial | Types struct — customizable type labels |
| 2.9 | 4591 | 2416 | 0.84 | early | FilterTarget / FilterAction / FilterOptions |
| 2.10 | 4864 | 6433 | 1.00 | late | Cargo dependencies |
| 2.11 | 4972 | — | 0.00 | missing | Tree / ItemValue fn signatures |
| 2.12 | 5157 | — | 0.00 | missing | App method signatures |
| 2.13 | 5276 | — | 0.00 | missing | TreeOverview method signatures |
| 2.14 | 5350 | — | 0.00 | missing | Parser-impl method signatures (per format) |
| 2.15 | 5421 | — | 0.00 | missing | main() + run() skeleton |
| 3.1 | 6125 | — | 0.00 | missing | main.rs run() — load-parse-render pipeline |
| 3.2 | 6639 | — | 0.00 | missing | App::on_key — filter-mode passthrough + action match skeleton |
| 3.3 | 6827 | — | 0.00 | missing | App::on_key — copy action body |
| 3.4 | 7115 | — | 0.00 | missing | TreeOverview::on_key — tree action routing |
| 3.5 | 7547 | — | 0.06 | partial | Config::load + get_path — config discovery |
| 3.6 | 7850 | — | 0.00 | missing | CommandArgs::get_content_type — extension → ContentType |
| 3.7 | 8022 | — | 0.00 | missing | AnyParser — format auto-detection |
| 3.8 | 8245 | — | 0.10 | partial | JsonParser Parser impl + highlight() signature |
| 3.9 | 8516 | — | 0.00 | missing | TreeOverview::filter — filter entry point |
| 3.10 | 8947 | — | 0.00 | missing | Filter widget key handling — action match |
| 3.11 | 9311 | — | 0.00 | missing | FilterOptions::filter — per-item match predicate |
| 3.12 | 9513 | — | 0.00 | missing | Key::parse — config key-binding grammar |
| 3.13 | 9703 | — | 0.32 | partial | FileWatcher struct + methods |
| 3.14 | 9930 | — | 0.00 | missing | write_clipboard — OS-specific pbcopy/wl-copy/xclip/clip |
| 3.15 | 10012 | — | 0.00 | missing | XML parser behavior — yq-style attributes |

## Walker waste (cost ≥ 50, no NS intersection)

| first_t | cost | key |
|--------:|-----:|:----|
| 414 | 91 | Rust(MacroBodies { src_dir: "/Users/yoav/projects/precis/tests/fixtures/otree/src" }) |
| 1098 | 52 | Rust(PubItem { file: "/Users/yoav/projects/precis/tests/fixtures/otree/src/config/mod.rs", start_line: 91 }) |
| 1153 | 55 | Rust(PubItem { file: "/Users/yoav/projects/precis/tests/fixtures/otree/src/config/mod.rs", start_line: 53 }) |
| 1210 | 57 | Rust(PubItem { file: "/Users/yoav/projects/precis/tests/fixtures/otree/src/config/mod.rs", start_line: 74 }) |
| 1289 | 79 | Rust(PubItem { file: "/Users/yoav/projects/precis/tests/fixtures/otree/src/config/mod.rs", start_line: 62 }) |
| 1371 | 82 | Rust(PubItem { file: "/Users/yoav/projects/precis/tests/fixtures/otree/src/config/mod.rs", start_line: 106 }) |
| 1453 | 82 | Rust(PubItem { file: "/Users/yoav/projects/precis/tests/fixtures/otree/src/config/mod.rs", start_line: 118 }) |
| 2576 | 113 | Rust(PubItem { file: "/Users/yoav/projects/precis/tests/fixtures/otree/src/ui/data_block.rs", start_line: 16 }) |
| 2745 | 65 | Rust(PubItem { file: "/Users/yoav/projects/precis/tests/fixtures/otree/src/config/colors.rs", start_line: 251 }) |
| 2832 | 87 | Rust(PubItem { file: "/Users/yoav/projects/precis/tests/fixtures/otree/src/config/colors.rs", start_line: 286 }) |
| 3928 | 143 | Rust(PubItem { file: "/Users/yoav/projects/precis/tests/fixtures/otree/src/ui/tree_overview.rs", start_line: 19 }) |
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
| 9941 | 870 | Rust(PubItem { file: "/Users/yoav/projects/precis/tests/fixtures/otree/src/config/keys.rs", start_line: 195 }) |
