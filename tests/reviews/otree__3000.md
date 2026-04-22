---
snapshot_hash: b35fde6d413a8a962a57b07a4b9afe9ade0f3cfe62102eb82b6ac779f8cd7804
---

## Summary

At 3000 tokens the snapshot reaches into every major `src/` subdirectory (parse / config / ui / tree / live_reload / edit / debug / clipboard) but renders most ranked batches as struct/enum signposts with bodies elided, while spending budget on below-the-fold struct shells (`DataColors` / `TreeColors` / `FooterColors` / `PopupColors` / `FilterColors` / `Color`, `Keys` / `KeyAction`, `FooterText` / `Footer`, `PopupData` / `Popup`, `Header`, `DataBlock`, `AnyParser`). The dominant pattern is uniform "top-of-file public declarations" extraction: ranked batches that are inline bodies (2.12 actions, 2.13 default keys, 2.14 deps, 2.16 `CommandArgs` fields, 3.x everything) or pure prose (1.10 tagline line, 2.2 README usage, 3.1 actions.md table, 3.8 changelog, 3.15 roadmap) are skipped, while lower-ranked one-line struct heads from below-the-fold are kept. Several ranked batches (1.10, 2.5, 2.7, 2.8, 2.11, 2.15, 2.16) appear only as their leading declaration line — silently partial.

## Divergences

### Ranking
- [rust] [major] NS 1.3 (`src/main.rs:1-9`, nine `mod …;` declarations) absent; `main.rs` appears as a bare filename. Displaced by tier-5 / below-the-fold content (5.1 `FileWatcher` teaser, 5.5 `Edit` teaser, 5.6 `debug` macro, below-the-fold color/keys/popup/footer struct shells).
- [rust] [major] NS 1.11 (`SyntaxToken` enum at `src/parse/syntax.rs:11-27`) absent; no `syntax.rs` content appears. Displaced by below-the-fold struct shells (`FooterText`/`Footer`/`PopupData`/`Popup`/`Header`/`DataBlock`/`Color` and the `DataColors`/`TreeColors`/`FooterColors`/`PopupColors`/`FilterColors` shells).
- [markdown] [major] NS 2.2 (README usage/config pointers at `README.md:33-46`) absent. Snapshot stops at `README.md:3`. Displaced by below-the-fold shells (`Popup`, `FooterText`, `DataColors`, etc.).
- [rust] [major] NS 2.4 (`ContentType::new_parser` dispatch at `src/parse/mod.rs:66-78`) absent; only the `ContentType` header line (18) and `Parser` trait header (36) are shown from `parse/mod.rs`. Displaced by below-the-fold shells.
- [rust] [major] NS 2.6 (`src/parse/syntax.rs:1-10` file teaser with elision) absent; no `syntax.rs` excerpt at all. Displaced by below-the-fold shells.
- [rust] [major] NS 2.12 (Action enum variants at `src/config/keys.rs:317-348`) absent. Displaced by below-the-fold `KeyAction` (`keys.rs:351-354`) and `Keys` struct header (`keys.rs:195`), both below-the-fold.
- [rust] [major] NS 2.13 (default key bindings at `src/config/keys.rs:284-315`) absent. Displaced by below-the-fold `Keys` / `KeyAction` shells.
- [toml] [major] NS 2.14 (`Cargo.toml:13-36` dependencies + build-dependencies) absent. Displaced by below-the-fold color-struct shells and below-the-fold `Header` struct (`header.rs:41-44`).
- [markdown] [major] NS 3.1 (`docs/actions.md:1-34` action↔keys↔description table) absent; file appears only as a bare filename. Displaced by below-the-fold shells.
- [rust] [major] NS 3.2 (`CommandArgs` UI toggles at `cmd.rs:41-75`) absent. Displaced by below-the-fold shells.
- [rust] [major] NS 3.3 (`CommandArgs` misc flags at `cmd.rs:76-109`) absent. Displaced by below-the-fold shells.
- [rust] [major] NS 3.4 (`CommandArgs::get_content_type` at `cmd.rs:202-228`) absent. Displaced by below-the-fold shells.
- [rust] [major] NS 3.5 (`CommandArgs::update_config` at `cmd.rs:149-201`) absent. Displaced by below-the-fold shells.
- [toml] [major] NS 3.6 (`config/default.toml:1-29` top sections) absent. Displaced by below-the-fold shells.
- [toml] [major] NS 3.7 (`config/default.toml:30-60` `[keys]` block) absent. Displaced by below-the-fold shells.
- [markdown] [major] NS 3.8 (`docs/changelog.md:1-45` v0.6.x entries) absent; file appears only as a bare filename. Displaced by below-the-fold shells.
- [rust] [major] NS 3.9 (`main.rs::run` args→config→data at `main.rs:27-77`) absent. Displaced by below-the-fold shells.
- [rust] [major] NS 3.10 (`main.rs::run` `--to` + start at `main.rs:79-116`) absent. Displaced by below-the-fold shells.
- [rust] [major] NS 3.11 (`Tree::parse` + `from_value` at `tree.rs:51-92`) absent. Displaced by below-the-fold `FooterText` / `Footer` / `PopupData` / `Popup` / `Header` / `DataBlock` struct shells.
- [markdown] [major] NS 3.14 (`docs/actions.md:35-51` key-syntax reference) absent. Displaced by below-the-fold shells.
- [markdown] [major] NS 3.15 (`README.md:52-81` roadmap) absent. Displaced by below-the-fold shells.
- [rust] [major] NS 4.1 (`App::new` + constants + `show` loop at `src/ui/app.rs:91-163`) absent. Displaced by below-the-fold shells.
- [rust] [major] NS 4.2 (`App::on_key` action dispatch at `src/ui/app.rs:393-584`) absent. Displaced by below-the-fold shells.
- [rust] [major] NS 4.3 (`Tree::build_item` recursion at `src/tree.rs:102-276`) absent. Displaced by below-the-fold shells.
- [rust] [major] NS 4.4 (`App::build_edit` / `get_copy_text` / `popup_help` at `src/ui/app.rs:675-767`) absent. Displaced by below-the-fold shells.
- [rust] [major] NS 4.5 (`TreeOverview::change_root`/`reset` at `src/ui/tree_overview.rs:92-147`) absent. Displaced by below-the-fold shells.
- [rust] [major] NS 4.6 (`TreeOverview::expand_children`/`expand_all` at `src/ui/tree_overview.rs:176-238`) absent. Displaced by below-the-fold shells.
- [rust] [major] NS 4.7 (`TreeOverview::filter` + `filter_item` at `src/ui/tree_overview.rs:335-408`) absent. Displaced by below-the-fold shells.
- [rust] [major] NS 4.8 (`JsonParser` + `json::highlight` at `src/parse/json.rs:6-82`) absent; no `json.rs` excerpt. Displaced by below-the-fold shells.
- [rust] [major] NS 4.9 (`YamlParser` + multi-doc handling at `src/parse/yaml.rs:8-70`) absent; no `yaml.rs` excerpt. Displaced by below-the-fold shells.
- [rust] [major] NS 4.10 (`TomlParser` + `toml_value_to_json` at `src/parse/toml.rs:9-86`) absent; no `toml.rs` excerpt. Displaced by below-the-fold shells.
- [rust] [major] NS 4.11 (`XmlParser::parse` + teaser at `src/parse/xml.rs:12-36`) absent; no `xml.rs` excerpt. Displaced by below-the-fold shells. Catastrophic-omission guard (per NS note) lost.
- [rust] [major] NS 4.12 (`HclParser` + teaser at `src/parse/hcl.rs:8-26`) absent; no `hcl.rs` excerpt. Displaced by below-the-fold shells.
- [rust] [major] NS 4.13 (`JsonlParser` at `src/parse/jsonl.rs:7-38`) absent; no `jsonl.rs` excerpt. Displaced by below-the-fold shells.
- [rust] [major] NS 4.15 (`SyntaxToken::pure_text` + helpers at `src/parse/syntax.rs:110-149`, `:201-214`) absent. Displaced by below-the-fold shells.
- [rust] [major] NS 4.16 (`StringValue` multiline/numeric quoting at `src/parse/syntax.rs:151-199`) absent. Displaced by below-the-fold shells.
- [rust] [major] NS 4.18 (`FilterOptions::filter` + `contains` at `src/ui/filter.rs:152-187`) absent. Displaced by below-the-fold shells.
- [rust] [major] NS 4.19 (`Header::new` + `Header::draw` at `src/ui/header.rs:41-61`) absent; only the `Header` struct shell (41-44) is shown. Displaced by below-the-fold shells.

### Batch correctness
- [markdown] NS 1.10 (`README.md:1-5`) — snapshot includes only lines 1 and 3 (title and screenshot ref); line 5 (the JSON/YAML/TOML/XML tagline — per NS notes the load-bearing content of the batch) is silently omitted with no ellipsis indicator.
- [rust] NS 2.5 (`ContentType` enum with module header at `src/parse/mod.rs:1-34`) — snapshot shows only `pub enum ContentType {` (line 18) elided with `…`. Missing: module `mod …;` declarations (1-8), `pub use syntax::SyntaxToken;` re-export (10), all seven variants and their doc comments through line 34.
- [rust] NS 2.7 (`Parser` trait + default `parse_root` at `src/parse/mod.rs:36-64`) — snapshot shows only `pub trait Parser {` (line 36) elided with `…`. The four trait methods and the default `parse_root` body (including the "schema '…' does not allow array as root" error) are dropped.
- [rust] NS 2.8 (`src/config/types.rs` whole, 48 lines) — snapshot shows only `pub struct Types {` (line 28) elided with `…`. The six fields and the `generate_types_default!` macro invocation are absent.
- [rust] NS 2.11 (`src/ui/app.rs:27-90`) — snapshot includes `ScrollDirection` (48-51), `App` header (53) with fields elided, and `ShowResult` (86-89), but silently omits `Refresh { Update, Skip, Quit, Edit(Box<Edit>) }` (27-36) and `ElementInFocus { TreeOverview, DataBlock, Popup, Filter, None }` (38-45) — two of the batch's five declarations.
- [rust] NS 2.15 (`HeaderContext` at `src/ui/header.rs:11-39`) — snapshot shows only the struct shell (11-16). `HeaderContext::new` (stdin fallback, `humansize::format_size(_, BINARY)`) and the `format` fn substituting `{version}/{data_source}/{content_type}/{data_size}` are dropped.
- [rust] NS 2.16 (`CommandArgs` file/config/output flags at `src/cmd.rs:11-40`) — snapshot shows only `pub struct CommandArgs {` (line 13) elided with `…`. The ranked fields (`path`, `--config`, `-t/--content-type`, `-o/--to`, `--disable-header/footer/filter`) are absent — the entire payload.
- [rust] NS 3.12 (`ui::start` + terminal lifecycle at `src/ui/mod.rs:24-87`) — only the `start` fn signature shown; `get_border_style`, main loop body, `new_terminal`, `restore`, and the `ShowResult::Edit` teardown are all elided.
- [rust] NS 3.13 (`TreeOverview` struct + `on_key` dispatch at `src/ui/tree_overview.rs:19-90`) — only `pub struct TreeOverview {` header shown; all struct fields, the `DEFAULT_HIGHLIGHT_SYMBOL` / `MAX_FILTER_COUNT_DISPLAY` constants, and the `on_key` action dispatch are elided.
- [rust] NS 4.14 (`src/parse/any.rs` whole) — only the `AnyParser` struct header shown; the `RefCell<Option<Box<dyn Parser>>>` field, `parse_root` iteration over `ContentType::iter()`, and the first-successful-parse install logic are elided.
- [rust] NS 4.17 (`DataBlock` struct + entrypoints at `src/ui/data_block.rs:1-67`) — struct fields (16-30) present, but `new`, `on_key` (mapping `Action::Move*`/`SelectFirst`/`SelectLast`), `on_scroll`, and the `SCROLL_RETAIN = 5` constant are absent.
- [rust] NS 5.1 (`FileWatcher` at `src/live_reload.rs:16-91`) — struct fields (16-25) shown; `new` spawning `watch_file` via `notify::recommended_watcher`, `get_err`, `parse_tree` and `max_data_size` enforcement are absent.
- [rust] NS 5.2 (`src/clipboard.rs` whole) — only `pub fn write_clipboard` signature (line 28) shown; the OS-specific command selection (`pbcopy` / `wl-copy` / `xclip` / `clip`), stdin-pipe invocation, and `NotFound` → "install it" error are elided.
- [rust] NS 5.3 (`Colors` root + defaults + parse macro at `src/config/colors.rs:1-76`) — only `pub struct Colors {` header (21) shown; `generate_colors_parse!` macro and `Colors::default` / `default_header` / `default_focus_boder = magenta bold` bodies are elided.
- [rust] NS 5.4 (`Color` struct + palette-aware `parse` at `src/config/colors.rs:341-399`) — struct (342-353) present, but `Color::new` (empty-string → `None`) and `Color::parse` (palette-substitution, then `ratatui::style::Color::parse`) — the load-bearing behaviour — are absent.
- [rust] NS 5.5 (`Edit` setup signatures at `src/edit.rs:1-40`) — struct (10-14) present; imports and `Edit::new(cfg, identify, data, extension)` constructor (building `Command` from `cfg.editor.program`+`args`+`dir`, `{file}` → `{editor.dir}/otree_{id}.{ext}`, `/` → `_` sanitisation) are absent.

### Honesty
- (none)
