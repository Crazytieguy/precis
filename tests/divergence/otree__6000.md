scores: Sim=0.372 OffScript=0.61 Pred=0 Coverage=0.25

## Missed NS batches (credit < 0.8)

- [1.2 README lede — what otree is] credit=0.40
- [1.9 README usage — invocation examples] credit=0.00
- [1.10 ContentType enum — the six supported formats] credit=0.71
- [1.11 Parser trait — required methods] credit=0.56
- [1.12 Action enum — first half (navigation/layout)] credit=0.00
- [1.13 Action enum — second half (filter/edit/copy/help)] credit=0.00
- [1.14 Default key bindings — navigation half] credit=0.00
- [1.15 Default key bindings — filter/edit/copy/help half] credit=0.00
- [1.17 FieldType + ContentType::new_parser dispatch] credit=0.47
- [1.18 SyntaxToken enum — the shared highlight IR] credit=0.71
- [2.1 Config struct — top-level config sections] credit=0.69
- [2.2 Tree struct + ItemValue struct] credit=0.70
- [2.3 App struct + ElementInFocus / Refresh enums] credit=0.45
- [2.4 CLI flag names — one line per arg] credit=0.00
- [2.5 Default Config constants — size bounds + max_data_size] credit=0.36
- [2.6 Colors top-level sections] credit=0.29
- [2.7 Color struct — fg/bg/bold/italic fields] credit=0.28
- [2.8 Types struct — customizable type labels] credit=0.64
- [2.10 Cargo dependencies] credit=0.00
- [2.11 Tree / ItemValue fn signatures] credit=0.00
- [2.12 App method signatures] credit=0.00
- [2.13 TreeOverview method signatures] credit=0.00
- [2.14 Parser-impl method signatures (per format)] credit=0.00
- [2.15 main() + run() skeleton] credit=0.00
- [3.1 main.rs run() — load-parse-render pipeline] credit=0.00
- [3.2 App::on_key — filter-mode passthrough + action match skeleton] credit=0.00
- [3.3 App::on_key — copy action body] credit=0.00
- [3.4 TreeOverview::on_key — tree action routing] credit=0.00
- [3.5 Config::load + get_path — config discovery] credit=0.07
- [3.6 CommandArgs::get_content_type — extension → ContentType] credit=0.00
- [3.7 AnyParser — format auto-detection] credit=0.00
- [3.8 JsonParser Parser impl + highlight() signature] credit=0.16
- [3.9 TreeOverview::filter — filter entry point] credit=0.00
- [3.10 Filter widget key handling — action match] credit=0.00
- [3.11 FilterOptions::filter — per-item match predicate] credit=0.00
- [3.12 Key::parse — config key-binding grammar] credit=0.00
- [3.13 FileWatcher struct + methods] credit=0.32
- [3.14 write_clipboard — OS-specific pbcopy/wl-copy/xclip/clip] credit=0.00
- [3.15 XML parser behavior — yq-style attributes] credit=0.00

## Unmapped walker batches (cost ≥ 50)

- Markdown(Section { file: "/Users/yoav/projects/precis/tests/fixtures/otree/docs/changelog.md", section_index: 0 }) cost=51
- Rust(MacroBodies { src_dir: "/Users/yoav/projects/precis/tests/fixtures/otree/src" }) cost=91
- Rust(ModUse { file: "/Users/yoav/projects/precis/tests/fixtures/otree/src/config/mod.rs" }) cost=119
- Rust(ModUse { file: "/Users/yoav/projects/precis/tests/fixtures/otree/src/parse/mod.rs" }) cost=115
- Rust(PubItem { file: "/Users/yoav/projects/precis/tests/fixtures/otree/src/config/colors.rs", start_line: 162 }) cost=295
- Rust(PubItem { file: "/Users/yoav/projects/precis/tests/fixtures/otree/src/config/colors.rs", start_line: 251 }) cost=65
- Rust(PubItem { file: "/Users/yoav/projects/precis/tests/fixtures/otree/src/config/colors.rs", start_line: 286 }) cost=87
- Rust(PubItem { file: "/Users/yoav/projects/precis/tests/fixtures/otree/src/config/colors.rs", start_line: 78 }) cost=259
- Rust(PubItem { file: "/Users/yoav/projects/precis/tests/fixtures/otree/src/config/mod.rs", start_line: 106 }) cost=82
- Rust(PubItem { file: "/Users/yoav/projects/precis/tests/fixtures/otree/src/config/mod.rs", start_line: 118 }) cost=82
- Rust(PubItem { file: "/Users/yoav/projects/precis/tests/fixtures/otree/src/config/mod.rs", start_line: 53 }) cost=55
- Rust(PubItem { file: "/Users/yoav/projects/precis/tests/fixtures/otree/src/config/mod.rs", start_line: 62 }) cost=79
- Rust(PubItem { file: "/Users/yoav/projects/precis/tests/fixtures/otree/src/config/mod.rs", start_line: 74 }) cost=57
- Rust(PubItem { file: "/Users/yoav/projects/precis/tests/fixtures/otree/src/config/mod.rs", start_line: 91 }) cost=52
- Rust(PubItem { file: "/Users/yoav/projects/precis/tests/fixtures/otree/src/ui/data_block.rs", start_line: 16 }) cost=113
- Rust(PubItem { file: "/Users/yoav/projects/precis/tests/fixtures/otree/src/ui/tree_overview.rs", start_line: 19 }) cost=143
