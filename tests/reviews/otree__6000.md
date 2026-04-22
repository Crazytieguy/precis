---
snapshot_hash: 1c238002ea85b18ddabdd971ebb468f5d84fbe126bb6df573e851767d789410c
---

## Summary

At the 6000 budget the snapshot covers the tier-1 folder/file skeleton (1.1, 1.2, 1.4-1.9, 1.11) and a handful of tier-2 type declarations (2.1, 2.3, 2.5, 2.7, 2.9, 2.10, 2.17) but is broadly misaligned against the North Star. The dominant divergence is a "wide, shallow sprinkle of struct/enum headers" pattern: nearly every `pub struct`/`pub enum` across the crate is shown as a bare declaration with body elided (`…`), including many below-the-fold entries (`FooterColors`/`PopupColors`/`FilterColors`/`Color` shells, `Keys`/`KeyAction`, `PopupData`/`Popup`, `FooterText`/`Footer`, `DataBlock` fields, `TreeOverview` struct), at the cost of completing mid-tier batches and skipping all of tier 3 entirely. A second, independent source of below-the-fold displacement is a large `docs/changelog.md` excerpt reaching down to v0.1.0 (i.e. into `:46-156`) while still omitting most of the ranked `:1-45` batch. `src/main.rs` is present only as a filename with no content — so 1.3, 3.9, 3.10 are all absent. Action inventory, default keybindings, `Cargo.toml [dependencies]`, CLI surface, `config/default.toml` entirely, README usage/roadmap, `Tree::parse`, `ui::start`, `TreeOverview::on_key`, and every tier-4 body are absent.

## Divergences

### Ranking
- [rust] [major] 1.3 (`src/main.rs:1-9` module declarations) missing; `main.rs` appears only as a filename with no body, while below-the-fold struct headers across `src/` fill the snapshot.
- [markdown] [major] 2.2 (`README.md:33-46` usage + config pointers) missing; displaced by below-the-fold `docs/changelog.md:46-156` entries (v0.5.1/v0.4.x/v0.3/v0.1) and below-the-fold struct-header spray across `config/colors.rs` sub-structs, `keys.rs::Keys`/`KeyAction`, and UI struct declarations (`ScrollDirection`, `PopupData`, `Popup`, `FooterText`, `Footer`, `Header`, `TreeOverview`).
- [rust] [major] 2.6 (`src/parse/syntax.rs:1-10` use-statements + elision teaser) missing; displaced by the same below-the-fold content.
- [rust] [major] 2.12 (Action enum variants, `src/config/keys.rs:317-348`) missing; displaced by below-the-fold `KeyAction` struct at `:351-354`, `Keys` struct header at `:195`, colors sub-struct bodies, UI widget struct headers, changelog `:46-156`.
- [rust] [major] 2.13 (default key bindings, `src/config/keys.rs:284-315`) missing with the same displacing content.
- [other-language] [major] 2.14 (`Cargo.toml:13-36` `[dependencies]`) missing while tier 4/5 struct shells across `src/` are present.
- [markdown] [major] 3.1 (`docs/actions.md:1-34` action table) missing; snapshot lists `docs/actions.md` filename only. Displaced by below-the-fold `FooterColors`/`PopupColors`/`FilterColors`/`Color` struct bodies, `Keys`/`KeyAction`, and the `docs/changelog.md:46-156` tail.
- [rust] [major] 3.2 (`src/cmd.rs:41-75` UI toggles) missing; snapshot shows only `pub struct CommandArgs {` header at line 13.
- [rust] [major] 3.3 (`src/cmd.rs:76-109` misc flags — `--wrap`, `--show-config`, `--debug`, etc.) missing.
- [rust] [major] 3.4 (`src/cmd.rs:202-228` `get_content_type` extension→ContentType dispatch) missing.
- [rust] [major] 3.5 (`src/cmd.rs:149-201` `update_config` CLI→Config propagation) missing.
- [other-language] [major] 3.6 (`config/default.toml:1-29` top sections) missing; `config/default.toml` listed as filename only.
- [other-language] [major] 3.7 (`config/default.toml:30-60` `[keys]`) missing.
- [rust] [major] 3.9 (`src/main.rs:27-77` `run()` args→config→data) missing; `main.rs` has no rendered content at all.
- [rust] [major] 3.10 (`src/main.rs:79-116` `run()` `--to`/size-check/UI start) missing.
- [rust] [major] 3.11 (`src/tree.rs:51-92` `Tree::parse`/`from_value`) missing; `tree.rs` shows only struct/enum declarations at `:14-49`.
- [markdown] [major] 3.14 (`docs/actions.md:35-51` key-syntax reference) missing.
- [markdown] [major] 3.15 (`README.md:52-81` roadmap) missing — only lines 1 and 3 of README rendered.
- [rust] [major] 4.1 (`App::new` + constants + `show` loop, `src/ui/app.rs:91-163`) missing; `app.rs` content stops at `ShowResult` (line 89).
- [rust] [major] 4.2 (`App::on_key` action dispatch, `src/ui/app.rs:393-584`) missing.
- [rust] [major] 4.3 (`Tree::build_item` recursive construction, `src/tree.rs:102-276`) missing.
- [rust] [major] 4.4 (`App::build_edit`/`get_copy_text`/`popup_help`, `src/ui/app.rs:675-767`) missing.
- [rust] [major] 4.5 (`TreeOverview::change_root`/`reset`, `src/ui/tree_overview.rs:92-147`) missing.
- [rust] [major] 4.6 (`TreeOverview::expand_children`/`expand_all`, `src/ui/tree_overview.rs:176-238`) missing.
- [rust] [major] 4.7 (`TreeOverview::filter`/`filter_item`, `src/ui/tree_overview.rs:335-408`) missing.
- [rust] [major] 4.8 (`JsonParser` impl + `json::highlight` body, `src/parse/json.rs:6-82`) missing; only struct line and `highlight` signature shown with bodies elided.
- [rust] [major] 4.9 (`YamlParser` impl + multi-doc handling, `src/parse/yaml.rs:8-70`) missing; only `pub struct YamlParser` declaration shown.
- [rust] [major] 4.10 (`TomlParser` impl + `toml_value_to_json`, `src/parse/toml.rs:9-86`) missing; only `pub struct TomlParser` declaration shown.
- [rust] [major] 4.11 (`XmlParser::parse` + teaser, `src/parse/xml.rs:12-36`) missing; only `pub struct XmlParser` declaration shown. The explicit XML `@attr`/`#text`/`#cdata` catastrophic-omission guard is lost.
- [rust] [major] 4.12 (`HclParser` impl + teaser, `src/parse/hcl.rs:8-26`) missing; only `pub struct HclParser` declaration shown.
- [rust] [major] 4.13 (`JsonlParser` impl, `src/parse/jsonl.rs:7-38`) missing; only `pub struct JsonlParser` declaration shown.
- [rust] [major] 4.14 (`AnyParser` content-type auto-detection, `src/parse/any.rs` whole file) missing; only `pub struct AnyParser { inner: RefCell<…> }` declaration shown. `parse_root`'s iteration over `ContentType::iter()` (the stdin JSON auto-detect path advertised in 3.8) is absent.
- [rust] [major] 4.15 (`SyntaxToken::pure_text` + complexity + quoting helpers, `src/parse/syntax.rs:110-134`/`:136-149`/`:201-214`) missing; only `quote_field_name` and `is_value_complex` signatures appear with bodies elided.
- [rust] [major] 4.16 (`StringValue` multiline/numeric quoting rules, `src/parse/syntax.rs:151-199`) missing bodies — only the `StringValue` enum declaration (151-154) shown.
- [rust] [major] 4.17 (`DataBlock` struct + `on_key`, `src/ui/data_block.rs:1-67`) missing core; `on_key`/`on_scroll` dispatch and `SCROLL_RETAIN` constant absent; only struct fields (16-30) shown.
- [rust] [major] 4.18 (`FilterOptions::filter`/`contains`, `src/ui/filter.rs:152-187`) missing.
- [rust] [major] 4.19 (`Header::new`/`Header::draw`, `src/ui/header.rs:41-61`) missing; only `Header` struct fields (41-44) shown.
- [rust] [major] 5.1 (`FileWatcher` public API, `src/live_reload.rs:16-91`) missing bodies; only struct fields (16-25) rendered. `new` spawning `watch_file`, `get_err`, `parse_tree` absent.
- [rust] [major] 5.2 (`src/clipboard.rs` whole — OS-specific command selection) missing; only `write_clipboard` signature at `:28` shown.
- [rust] [major] 5.3 (`Colors` root struct + defaults + parse macro, `src/config/colors.rs:1-76`) missing — struct present; `Colors::default`/`default_header`/`default_focus_boder` bodies absent.
- [rust] [major] 5.4 (`Color` struct + palette-aware `parse`, `src/config/colors.rs:341-399`) missing bodies; only `Color` struct fields (342-353) shown. `Color::new` and the palette-substituting `Color::parse` are absent.
- [rust] [major] 5.5 (`Edit` external-editor setup, `src/edit.rs:1-40`) missing; only `Edit` struct (10-14) shown.
- [rust] [major] 5.6 (`debug` macro + `set_file`, `src/debug.rs` whole) missing bodies; `debug!` macro body (8-15) is shown but `set_file` and `write_logs` bodies are elided.
- [other-language] [major] 5.7 (`config/default.toml:62-109` `[colors.*]` + `[types]`) missing.
- [generic] [major] 5.8 (parser test-case folder listing + `{raw, _highlight}` pair convention) partial at best; snapshot lists the six `test_cases/{hcl,json,jsonl,toml,xml,yaml}/` folders as empty directories and the "{raw, _highlight}" pair convention that the NS flags as the point is not surfaced.
- [markdown] [major] 5.9 (`examples/example.json:1-15` teaser) missing; only the filename is listed.
- [other-language] [major] 5.10 (`typos.toml` + `.gitignore` contents) missing; filenames only.

### Batch correctness
- [markdown] [minor] 1.10 partial: snapshot shows `README.md:1` (title) and `:3` (screenshot) but omits line 5 — the JSON/YAML/TOML/XML tagline that is the explicit reason this batch exists (per NS 1.10 notes comparing it to 1.5/2.5's wider format list).
- [rust] [minor] 2.4 partial: only the `impl ContentType` opening at `:66` and `pub fn new_parser(self) -> Box<dyn Parser> {` signature are rendered; the `ContentType→Parser` match body at `:68-78` — the whole point of the batch — is silently dropped.
- [rust] [minor] 2.8 partial: `Types` struct fields (28-46) shown, but the `generate_types_default!` macro invocation (which the NS explicitly calls out as defaulting each field to its own name) is omitted. Batch is specified as the whole 48-line file.
- [rust] [minor] 2.11 partial: `ScrollDirection` (48-51), `App` struct body (53-84), and `ShowResult` enum (86-89) are present, but `Refresh` enum (with `Edit(Box<Edit>)` variant) and `ElementInFocus` enum (lines 27-46 of the NS-specified 27-90 range) are missing — the focus/refresh enums the batch is named for.
- [rust] [minor] 2.15 partial: `HeaderContext` struct fields (`:11-16`) plus `Header` struct (`:41-44`) shown, but `HeaderContext::new` and `format` (`{version}`/`{data_source}`/`{content_type}`/`{data_size}` substitution and `humansize::format_size(_, BINARY)`) at `:17-39` are absent — the behavioral core.
- [rust] [minor] 2.16 partial: only `pub struct CommandArgs {` header (line 13) rendered with elision; every CLI flag (positional `path`, `--config`, `-t`, `-o`, `--disable-header/footer/filter`) is dropped.
- [markdown] [minor] 3.8 partial: lines from `docs/changelog.md` are scattered — the snapshot renders `:1, 3, 5-6` (v0.6.4), `:8, 10, 12, 14, 16-18, 20` (v0.6.3), `:22, 24, 26-28` (v0.6.2 first bullet), then jumps past the batch end to `:97, 99, 101`, `:111-156`. `:29-45` (v0.6.1 — HCL support, `-o`/`--to`, narrow-footer display, TOML section names that the batch notes explicitly call out) is absent while below-the-fold `:46-156` content is pulled in.
- [rust] [minor] 3.12 partial: only `pub fn start(mut app: App) -> Result<()>` signature at `:35` with elided body; `get_border_style`, the `ShowResult::{Edit, Quit}` loop, `new_terminal`, and `restore` are all absent.
- [rust] [minor] 3.13 partial: `TreeOverview` struct fields (19-30) shown, but `on_key` dispatch (mapping `Action::MoveUp/Down/…/ChangeRoot/ExpandChildren/ExpandAll/FilterNextMatch/PrevMatch/Reset` to tree-state ops) and `DEFAULT_HIGHLIGHT_SYMBOL`/`MAX_FILTER_COUNT_DISPLAY` constants — the behavioral core — are absent.

### Honesty
- (none)
