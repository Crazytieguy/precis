# otree — North Star

Revision pin: `a02bdf44`

## Batches

### 1.1 README title + tagline + usage
- Content: `README.md:1-6` (title, screenshot include, one-line tagline "A command line tool to view objects (JSON/YAML/TOML/XML) in TUI tree widget.") + `README.md:33-51` (Usage section: invocation `otree /path/to/file.{json,yaml,toml,xml}`, pointer to `~/.config/otree.toml`, pointer to `docs/actions.md` for keys, pointer to `docs/colors.md` for colors).
- Cost: 187 tokens (helper: `count-tokens.py tests/fixtures/otree/README.md:1-6 tests/fixtures/otree/README.md:33-51` → 41 + 146 = 187).
- Notes: the highest-value identity + how-to-use pair. The tagline lists JSON/YAML/TOML/XML but the code also supports HCL and JSONL (revealed by 1.6). Sized to leave headroom for the rest of the 1.x and 2.x batches under the 2× rule.

### 1.2 Root directory listing
- Content: one line per top-level entry — `.github/`, `.gitignore`, `Cargo.lock`, `Cargo.toml`, `LICENSE`, `README.md`, `assets/`, `build.rs`, `config/`, `docs/`, `examples/`, `src/`, `typos.toml`.
- Cost: 35 tokens (helper: printf listing | `count-tokens.py --stdin` → 35).
- Notes: reveals custom `build.rs`, `config/` tree, `docs/`, `examples/`.

### 1.3 `src/` listing
- Content: `clipboard.rs`, `cmd.rs`, `config/`, `debug.rs`, `edit.rs`, `live_reload.rs`, `main.rs`, `parse/`, `tree.rs`, `ui/`.
- Cost: 28 tokens.
- Notes: dedicated modules for clipboard, external editor, live reload, and debug logging each hint at feature areas a user can ask about.

### 1.4 Top-level module declarations
- Content: `src/main.rs:1-9` — `mod clipboard; mod cmd; mod config; mod debug; mod edit; mod live_reload; mod parse; mod tree; mod ui;`.
- Cost: 28 tokens.
- Notes: confirms no hidden modules.

### 1.5 `src/ui/` listing
- Content: `app.rs`, `data_block.rs`, `filter.rs`, `footer.rs`, `header.rs`, `mod.rs`, `popup.rs`, `tree_overview.rs`.
- Cost: 27 tokens.

### 1.6 `src/parse/` listing
- Content: `any.rs`, `hcl.rs`, `json.rs`, `jsonl.rs`, `mod.rs`, `syntax.rs`, `test_cases/`, `toml.rs`, `xml.rs`, `yaml.rs`.
- Cost: 33 tokens.
- Notes: reveals HCL and JSONL support beyond the README tagline, and names the `test_cases/` golden-output corpus.

### 1.7 `src/config/` + `docs/` + `config/` listings
- Content: `src/config/` files (`colors.rs`, `keys.rs`, `mod.rs`, `types.rs`); `docs/` files (`actions.md`, `changelog.md`, `colors.md`); `config/` files (`default.toml`, `themes/catppuccin.toml`).
- Cost: 35 tokens (12 + 10 + 13).
- Notes: `docs/colors.md` is zero-byte in this revision — a notable gotcha.

### 1.8 `Cargo.toml` — `[package]` stanza
- Content: `Cargo.toml:1-12` — `name = "otree"`, `version = "0.6.4"`, `edition = "2021"`, `build = "build.rs"`, MIT, authors `["fioncat"]`, categories `["command-line-utilities"]`, repository URL, description.
- Cost: 95 tokens.

### 2.1 `Cargo.toml` — dependencies
- Content: `Cargo.toml:13-36` — full `[dependencies]` and `[build-dependencies]` tables.
- Cost: 229 tokens.
- Notes: reveals the full stack — `ratatui`, `crossterm`, `tui-tree-widget`, `tui-textarea`; per-format `serde_json` + `toml` (both `preserve_order`), `serde_yml`, `quick-xml`, `hcl-rs`; plus `notify` (live reload), `dirs` (config path), `humansize`, `clap`, `anyhow`, `console`, `strum`, `paste`, `regex`; build deps `vergen`, `simple-error`.

### 2.2 README — Roadmap part 1 (UI + core actions)
- Content: `README.md:52-66` — the first ~15 roadmap items: UI header/tree/data/footer (v0.1-v0.2), filter input (v0.5), popup (v0.2), filter-keyword highlighting (v0.6); change-root / back-to-previous-root / scale up/down (v0.1); mouse click + scroll; edit in external editor ReadOnly (v0.2).
- Cost: 211 tokens.
- Predecessor: 1.1

### 2.3 `CommandArgs` — struct header + first flags
- Content: `src/cmd.rs:1-40` — imports, `#[derive(Parser)] pub struct CommandArgs { ... }`, positional `path: Option<String>` (stdin if absent), `--config`, `-t/--content-type`, `-o/--to`, `--disable-header`, `--disable-footer`, `--disable-filter`.
- Cost: 219 tokens.

### 2.4 `CommandArgs::get_content_type`
- Content: `src/cmd.rs:202-228` — the extension match: `json` → `Json`, `yaml`|`yml` → `Yaml`, `toml` → `Toml`, `xml` → `Xml`, `hcl` → `Hcl`, `jsonl` → `Jsonl`, else `Any`; falls back to `Any` if no path / no extension.
- Cost: 192 tokens.
- Predecessor: 2.3
- Notes: the definitive answer to "what file extensions does otree recognize?".

### 2.5 `Config` struct — field map
- Content: `src/config/mod.rs:16-50` — `#[derive(Serialize, Deserialize)] pub struct Config { tree, editor, data, layout, header, footer, filter, palette, colors, types, keys }` each with `#[serde(default=...)]`.
- Cost: 197 tokens.
- Notes: authoritative schema outline for `~/.config/otree.toml`.

### 2.6 `SyntaxToken` enum
- Content: `src/parse/syntax.rs:1-27` — imports and `pub enum SyntaxToken { Symbol, Name, Tag, String, Number, Null, Bool, Section, Break, Indent }`.
- Cost: 127 tokens.

### 2.7 `Tree` / `ItemValue` / `FieldType` type declarations
- Content: `src/tree.rs:1-55` — imports, `pub struct Tree`, `pub struct ItemValue { name, value, field_type, description, tokens }`, `pub enum FieldType { Null, Num, Bool, Str, Obj, Arr }`, `pub struct HighlightKeyword { text, ignore_case }`, and the `Tree::parse(cfg, data, content_type)` signature.
- Cost: 313 tokens.

### 2.8 `Parser` trait + `ContentType` enum
- Content: `src/parse/mod.rs` (whole file, 78 lines) — `pub enum ContentType { Json, Yaml, Toml, Xml, Hcl, Jsonl, Any }`, the `Parser` trait (`extension`, `allow_array_root`, `parse`, `syntax_highlight`, `parse_root` with the "root must be object or array" invariant), and `ContentType::new_parser` factory.
- Cost: 545 tokens.

### 2.9 `App` top-level state + focus enums
- Content: `src/ui/app.rs:1-84` — imports, `enum Refresh { Update, Skip, Quit, Edit(Box<Edit>) }`, `enum ElementInFocus { TreeOverview, DataBlock, Popup, Filter, None }`, `pub enum ScrollDirection { Up, Down }`, and `pub struct App { cfg, focus, last_focus, tree_overview, tree_overview_area, filter, filter_area, data_block, data_block_area, layout_direction, layout_tree_size, header, header_area, skip_header, footer, footer_area, skip_footer, foot_message, popup, before_popup_focus, fw }`.
- Cost: 500 tokens.

### 2.10 `TreeOverview` — struct + constructor
- Content: `src/ui/tree_overview.rs:1-50` — `pub struct TreeOverview { cfg, state, tree, filter_items, filter_cache, filter_nav_pos, before_filter_state, last_switches, root_switch, root_identifies }`, constants `DEFAULT_HIGHLIGHT_SYMBOL = "→ "` and `MAX_FILTER_COUNT_DISPLAY = 9999`, `new`.
- Cost: 370 tokens.
- Notes: surfaces the filter-state fields and the root-switching stack.

### 2.11 `docs/actions.md` — action / default-keys table
- Content: `docs/actions.md:1-35` — 29-action table mapping `Action` name → default keys → description.
- Cost: 657 tokens.
- Notes: answers "what key does X?" with zero follow-up reads.

### 3.1 `docs/actions.md` — key-syntax notes
- Content: `docs/actions.md:36-51` — grammar for `<up>`, `<ctrl-x>`, `<alt-x>`, `<fN>`; rebind example `[keys] select_focus = [" ", "<enter>"]`.
- Cost: 178 tokens.
- Predecessor: 2.11

### 3.2 `config/default.toml` — `[keys]`
- Content: `config/default.toml:30-60` — default key bindings in the exact TOML format a user edits.
- Cost: 228 tokens.

### 3.3 `main.rs` — `run()` startup pipeline
- Content: `src/main.rs:10-70` — imports, `fn run()`, args parse, `Config::load` / `Config::default` branch, `args.update_config`, `cfg.parse`, `args.show_config`, `debug::set_file`, content-type inference, `max_data_size` calc (MiB × 1024 × 1024), optional `FileWatcher` wiring, read-file-or-stdin.
- Cost: 419 tokens.

### 3.4 `main.rs` — `--to` conversion + size guard + TUI launch
- Content: `src/main.rs:79-116` — the `args.to` branch (parse source, re-emit target, print to stdout), `max_data_size` bail with `humansize`-formatted limit hint, `Tree::parse`, `App::new`, `set_header`, `ui::start`, and the top-level `fn main` error printer (exits 1 with `eprintln!("Error: {err:#}")`).
- Cost: 364 tokens.
- Predecessor: 3.3

### 3.5 `CommandArgs` — remaining flags
- Content: `src/cmd.rs:41-108` — remaining clap flags (`--tree-disable-selected-highlight`, `--tree-selected-symbol`, `--filter-ignore-case`, `--filter-exclude-mode`, `-f/--header-format`, `-V/--vertical`, `-H/--horizontal`, `-s/--size`, `--disable-highlight`, `-w/--wrap`, `--show-config`, `--ignore-config`, `--build-info`, `--max-data-size`, `-R/--live-reload`, `--debug`, `-v/--version`).
- Cost: 430 tokens.
- Predecessor: 2.3

### 3.6 `tree.rs` — `Tree::parse` + `from_value` + accessors
- Content: `src/tree.rs:51-100` — entry point, the root-dispatching logic (`Value::Array` and `Value::Object` expanded at the root, anything else becomes a single synthetic `"root"` item), `get_value`, `get_parser`.
- Cost: 329 tokens.
- Predecessor: 2.7

### 3.7 `ui/mod.rs` — TUI entry + border-style helper
- Content: `src/ui/mod.rs` (whole, 87 lines) — `pub fn start(app) -> Result<()>` main loop handling `ShowResult::{Edit, Quit}`, raw-mode / alternate-screen / mouse-capture setup and restore, `fn get_border_style` used by every widget, plus re-exports `pub use App; pub use HeaderContext;`.
- Cost: 568 tokens.

### 3.8 `App::new` + constants
- Content: `src/ui/app.rs:91-132` — `HEADER_HEIGHT=1`, `FOOTER_HEIGHT=1`, `FILTER_HEIGHT=3`, `POLL_EVENT_DURATION=100ms`, `HELP_URL = "https://github.com/fioncat/otree/blob/main/docs/actions.md"`, `set_header`, `new` wiring every sub-widget.
- Cost: 311 tokens.
- Predecessor: 2.9

### 3.9 `App::show` + `refresh` + `refresh_with_fw` (event loop)
- Content: `src/ui/app.rs:137-209` — outer `terminal.draw` / `event::poll` / handle loop, and the file-watcher branch consuming `FileWatcher::parse_tree` and setting `foot_message` on update.
- Cost: 567 tokens.
- Predecessor: 3.8

### 3.10 `App::draw` — frame composition
- Content: `src/ui/app.rs:220-276` — `fn draw(&mut self, frame: &mut Frame)` laying out header, footer, filter, tree overview, data block, and popup in that order.
- Cost: 393 tokens.
- Predecessor: 3.8

### 3.11 `App::on_key` — global dispatcher (teaser, elided)
- Content: `src/ui/app.rs:393-440` (signature, filter-mode pre-handle routing `FilterAction::{Edit, Confirm, Quit, Skip}` back into tree + focus, `Action::Quit` and `Action::Switch` arms), then a bare `…` elision marker, then `src/ui/app.rs:554-584` (default arm delegating per-focused-widget).
- Cost: ~430 tokens (311 + ~120).
- Predecessor: 3.8
- Notes: teases the structure (pre-handle filter → match action → delegate) while the middle arms (`ChangeLayout`, `TreeScaleUp/Down`, `Edit`, `CopyName/Value`, `Filter*`, `FilterSwitchIgnoreCase`, `ShowHelp`) stay elided — one `Read` of `app.rs:441-553` fetches them.

### 3.12 `TreeOverview::on_key`
- Content: `src/ui/tree_overview.rs:71-90` — the `match action` routing `MoveUp/Down`, `SelectFocus`, `SelectParent`, `CloseParent`, `PageUp/Down`, `SelectFirst/Last`, `ChangeRoot`, `ExpandChildren`, `ExpandAll`, `FilterNextMatch/PrevMatch`, `Reset`.
- Cost: 208 tokens.
- Predecessor: 2.10

### 3.13 `Colors` root struct + defaults
- Content: `src/config/colors.rs:1-76` — parse-macro definition, `pub struct Colors { header, focus_border, footer, tree, data, popup, filter }`, `Colors::default`, `default_header`, `default_focus_boder`.
- Cost: 425 tokens.

### 3.14 `Keys` struct — serde field map
- Content: `src/config/keys.rs:194-282` — `pub struct Keys { move_up, ..., quit, #[serde(skip)] actions }` each with `#[serde(default="Keys::default_...")]`.
- Cost: 658 tokens.
- Predecessor: 2.5

### 3.15 `Keys` defaults + `Action` enum (macro bodies)
- Content: `src/config/keys.rs:284-348` — `generate_keys_default!` listing default bindings per action field, and `generate_actions!` declaring every `Action` variant.
- Cost: 486 tokens.
- Predecessor: 3.14

### 3.16 `parse/json.rs` — full file (reference parser)
- Content: `src/parse/json.rs:1-82` — `JsonParser` + the public `pub fn highlight(value, indent, has_next) -> Vec<SyntaxToken>` that `toml.rs`, `hcl.rs`, and `jsonl.rs` all delegate to for scalar / simple-array emission.
- Cost: 608 tokens.
- Predecessor: 2.6

### 3.17 `parse/syntax.rs` — `render` function
- Content: `src/parse/syntax.rs:32-134` — token-to-`Text` rendering loop (colors via `cfg.colors.data.*`, `Break`/`Indent` handling, optional `TextWrapper` path for `cfg.data.wrap`) and `pure_text`.
- Cost: 791 tokens.
- Predecessor: 2.6

### 3.18 `tree.rs` — `build_item` recursion body
- Content: `src/tree.rs:102-276` — the `match value` dispatch building each `TreeItem` and its `ItemValue` with human description (`"= 42"`, `"[ 3 items ]"`, `"{ 2 fields }"`, etc.), invocation of `parser.syntax_highlight` for containers, path-pushing into `identifies`.
- Cost: 1098 tokens.
- Predecessor: 2.7
- Notes: sized below 2× `render` (791 → 1582 ceiling).

### 4.1 `TreeOverview` — `change_root` + `reset` (root navigation stack)
- Content: `src/ui/tree_overview.rs:92-147` — `change_root` only accepts `Array`/`Object` values, stashes current `(Tree, TreeState)` onto `root_switch` (first switch) or `last_switches`, pushes `root_identifies`. `reset` pops. Also `close_parent`, `select_parent`, `get_selected_parent`.
- Cost: 348 tokens.
- Predecessor: 2.10

### 4.2 `TreeOverview` — filter bookkeeping
- Content: `src/ui/tree_overview.rs:319-410` — `begin_filter` (stashes state), `end_filter` (restores, preserving current selection), `filter` (rebuilds `filter_items` + `filter_cache: HashMap<path, match_index>` by recursive `filter_item`), `filter_item` (parent-auto-open on match, child pruning gated by `cfg.filter.exclude_mode`).
- Cost: 613 tokens.
- Predecessor: 2.10

### 4.3 `Filter` widget — types + `on_key` + `FilterOptions::filter`
- Content: `src/ui/filter.rs:1-92` (types `Filter`, `FilterAction { Edit, Confirm, Skip, Quit }`, `FilterOptions { text, target, ignore_case }`, `FilterTarget { Key, Value, All }`, `Filter::new`, `Filter::on_key` — `Char` → insert, `Enter` → Confirm (or Quit if empty), `Esc` → Quit, `Backspace` → delete, arrows/home/end → cursor) + `src/ui/filter.rs:152-187` (`FilterOptions::filter(&ItemValue)` match logic per `FilterTarget` with `ignore_case` substring check).
- Cost: ~730 tokens.
- Predecessor: 2.10

### 4.4 `DataBlock` — struct + `new` + `on_key` / `on_scroll`
- Content: `src/ui/data_block.rs:1-68` — fields (`can_vertical_scroll`, `vertical_scroll_last`, `ScrollbarState`, `identify`, etc.), `SCROLL_RETAIN = 5` constant, `new`, `on_key` mapping `MoveUp/Down/Left/Right/SelectFirst/Last` to scroll methods, `on_scroll` for mouse-wheel.
- Cost: 441 tokens.
- Notes: body of `draw` + `update_scroll` demoted — the wiring to `cfg.data.wrap`, scrollbar gating, and identify-change reset are fetched via one `Read` of `data_block.rs:147-254`.

### 4.5 `parse/yaml.rs` — `YamlParser` impl (multi-doc handling)
- Content: `src/parse/yaml.rs:1-70` — imports, `YamlParser::parse` (multi-document YAML collected into `Value::Array`), `syntax_highlight` (emits `---` separators only when at least one document is complex, otherwise delegates to the single-value `highlight`).
- Cost: ~470 tokens.
- Predecessor: 3.16

### 4.6 `parse/toml.rs` — parser + `syntax_highlight` dispatch
- Content: `src/parse/toml.rs:1-86` — `TomlParser`, the `syntax_highlight` wrapper deciding `[section]` vs `[[section]]` wrapping per value shape, and `toml_value_to_json` (note `Number::from_f64` fallback-to-`Number::from(0)`, datetime-as-string conversion).
- Cost: 609 tokens.
- Predecessor: 3.16

### 4.7 `parse/any.rs` — format auto-detection
- Content: `src/parse/any.rs` (whole, 61 lines) — `AnyParser { inner: RefCell<Option<Box<dyn Parser>>> }`, delegation of trait methods to the inner parser, and `parse_root` iterating `ContentType::iter()` (skipping `Any`) and keeping the first parser that succeeds.
- Cost: 365 tokens.
- Predecessor: 2.8
- Notes: the stdin / unknown-extension path — relevant to any "`echo ... | otree`" question.

### 4.8 `Key` enum
- Content: `src/config/keys.rs:53-73` — `pub enum Key { Char(char), Ctrl(char), Alt(char), F(u8), Backspace, Enter, Left, Right, Up, Down, PageUp, PageDown, Tab, Esc }`.
- Cost: 68 tokens.
- Predecessor: 3.14

### 4.9 `Color` type + palette-aware `parse`
- Content: `src/config/colors.rs:341-399` — `pub struct Color { fg: Option<String>, bg: Option<String>, bold, italic, #[serde(skip)] style }`, `Color::new` (empty-string fields → `None`), `Color::parse` substituting palette keys then `ratatui::style::Color` parsing.
- Cost: 357 tokens.
- Predecessor: 3.13

### 4.10 `tree.rs` — `build_item_text` + `ItemValue` accessors
- Content: `src/tree.rs:278-322` (`build_item_text` — `Span` composition for each tree row: name + type label from `cfg.types` + value description styled from `cfg.colors.tree.*`, with branching for the keyword-highlight path) + `src/tree.rs:385-411` (`ItemValue::plain_text`, `render`, `build_highlighted_text` — the three accessors every UI widget calls).
- Cost: 523 tokens (368 + 155).
- Predecessor: 3.18

### 4.11 `TreeColors` struct + defaults
- Content: `src/config/colors.rs:161-248` — `TreeColors { border, selected, name, filter_keyword, type_str, type_null, type_bool, type_num, type_arr, type_obj, value }` + defaults (`selected = black on light_green`, `filter_keyword = black on yellow`, `type_* = cyan bold italic`).
- Cost: 500 tokens.
- Predecessor: 3.13

### 4.12 `edit.rs` — external-editor invocation
- Content: `src/edit.rs:1-65` — `Edit::new` building a `Command` from `cfg.editor.{program, args, dir}`, `{file}` substitution per arg, identify-to-filename sanitization (`/` → `_`, prefixed with `otree_`, extension from `Parser::extension`), and `Edit::run` pausing for a keypress on editor error.
- Cost: 405 tokens.

### 4.13 `clipboard.rs` — platform-dispatch
- Content: `src/clipboard.rs` (whole, 56 lines) — macOS → `pbcopy`, Linux with `WAYLAND_DISPLAY` → `wl-copy` else `xclip -selection clipboard`, Windows → `clip`, else bail with an "install a clipboard program" hint.
- Cost: 418 tokens.

### 4.14 `ui/header.rs` — `HeaderContext` format string
- Content: `src/ui/header.rs` (whole, 61 lines) — `HeaderContext::new(source: Option<String>, content_type, size)` (source is `"stdin"` when `None`), `format` substituting `{version}`/`{data_source}`/`{content_type}`/`{data_size}`, `Header::draw` centering the line.
- Cost: 410 tokens.

### 5.1 `config/default.toml` — non-keys sections
- Content: `config/default.toml:1-29` + `config/default.toml:62-101` — `[tree]`, `[editor]`, `[layout]`, `[header]`, `[footer]`, `[data]`, `[filter]` defaults, plus full `[colors.*]` sections (`[colors]`, `[colors.tree]`, `[colors.data]`, `[colors.footer]`, `[colors.popup]`, `[colors.filter]`).
- Cost: 498 tokens.

### 5.2 `docs/changelog.md` — recent versions (v0.5.2 → v0.6.4)
- Content: `docs/changelog.md:1-70` — v0.6.4 fixes (XML empty-nodes-with-fields), v0.6.3 (auto-detect content type on stdin), v0.6.2 (wrap mode), v0.6.1 (`--to` option, HCL, narrow-footer polish), v0.6.0 (filter highlight default, `filter.exclude_mode` for old behavior; `n`/`N` navigation), v0.5.2 (XML parser with `@attr` + `#text` convention).
- Cost: 627 tokens.

### 5.3 `src/parse/test_cases/` listing (flat)
- Content: recursive listing of `src/parse/test_cases/` — subfolders `hcl/`, `json/`, `jsonl/`, `toml/`, `xml/`, `yaml/` and every file in them (`<case>.<ext>` inputs paired with `<case>_highlight.<ext>` expected outputs, except self-canonical cases like `2d_array.json`).
- Cost: 351 tokens (helper: `ls -1R tests/fixtures/otree/src/parse/test_cases/ | count-tokens.py --stdin` → 351).
- Notes: single source of truth for "how are the parsers tested?".

## Below-the-fold

- **`Cargo.lock` (16,913 tokens)** — resolved dependency versions; cost dwarfs the remaining budget and the direct-deps list is already in 2.1.
- **`LICENSE`** — MIT boilerplate; identity captured by `Cargo.toml`.
- **`.gitignore`, `typos.toml`, `assets/screenshot.png`** — no semantic content; naming in 1.2 is enough.
- **`.github/dependabot.yml` + `.github/workflows/*` bodies** — routine cargo-check / cross-compile-release / `typos` spell-check; filenames visible through 1.2's `.github/` entry.
- **`build.rs` (735 tokens)** — pure build-time plumbing: `vergen` integration + `fetch_git_info` stamping `OTREE_VERSION`/`OTREE_BUILD_TYPE`/`OTREE_SHA` env vars with `-dev_<shortsha>` / `-uncommitted` suffixes used by `show_build_info`.
- **README Roadmap part 2 + Thanks** (`README.md:67-86`, ~369 tokens) — remaining completed-roadmap bullets (switch panes, jump to parent, expand children/all, close-all, help popup, clipboard copy, syntax highlight, customize colors/keys, filter items, `--debug`) and the "built for deep Kubernetes YAML" origin note. 2.2 already carries the earlier half of the roadmap.
- **`CommandArgs::parse` + `update_config`** (`src/cmd.rs:111-201`, ~570 tokens) — clap error handling and CLI-flag-to-`Config` mutable writes. `update_config` is one line per flag, mechanical; flag surface (2.3 + 3.5) and `Config` schema (2.5) already cover what's mutable.
- **`CommandArgs::show_version` + `show_build_info`** (`src/cmd.rs:229-255`) — thin `env!("OTREE_*")` wrappers.
- **`Config` impl — `load` / `parse` / path resolution / `show`** (`src/config/mod.rs:127-194`, 491 tokens) — config-file discovery (`OTREE_CONFIG` env var, `~/.config/otree.toml` fallback), layout-size validation (`MIN_LAYOUT_TREE_SIZE=10`/`MAX_LAYOUT_TREE_SIZE=80`), palette validation, and the `--show-config` serializer. Names and field-level defaults are captured by 2.5 + 5.1; one `Read` surfaces the exact invariants.
- **`Config` sub-struct decls + defaults** (`src/config/mod.rs:52-125` + `:231-329`, ~720 tokens) — `Tree`, `Editor`, `Layout` (+ `LayoutDirection`), `Header`, `Footer`, `Filter`, `Data` structs and their defaults. Notable: `Editor::default_program` uses env `EDITOR` then `"vim"`, `Editor::default_args = ["{file}"]`, `Editor::default_dir = "/tmp"`, `Layout::default_direction = Horizontal`, `Layout::default_tree_size = 40`, `Header::default_format = "{version} - {data_source} ({content_type}) - {data_size}"`, `Config::default_max_data_size = 30` MiB. All visible through `default.toml` (5.1); one `Read` confirms the fallback rules.
- **`tree.rs` — `highlight_keyword` + `string_tokens`** (`src/tree.rs:324-382`, ~385 tokens) — substring-highlight span builder and multiline-string tokenizer. Behavior fully implied by 4.10 and the `HighlightKeyword` struct (2.7).
- **`parse/yaml.rs` `fn highlight` body** (`:71-153`, ~550 tokens) — multiline `|` block rendering, field-name quoting, list indentation. Pattern mirrors `parse/json.rs` (3.16); one `Read`.
- **`parse/toml.rs` `fn highlight` body** (`:88-193`, ~1025 tokens) — TOML-specific sectioning. Pattern implied by 4.6's dispatcher.
- **`parse/hcl.rs` body** (~870 tokens non-tests) — `HclParser` (delegates parse to `hcl::from_str`) and the emitter that emits `name = value` for simple fields and `name { ... }` blocks for nested objects. Emitter pattern mirrors `toml.rs`.
- **`parse/jsonl.rs`** (256 tokens) — parses each non-empty line as JSON into a `Value::Array`. `extension()` returns `"json"` (not `"jsonl"`) — subtle, affects editor-temp-file extension from 4.12. One `Read` when relevant.
- **`parse/xml.rs` (2,803 tokens total — ~2,560 non-tests)** — heaviest parser (`quick_xml::Reader` event loop, `NodeValues` accumulator, `AttrMap`, `highlight` reconstructor). The v0.5.2 changelog entry (in 5.2) documents the `@attr` + `#text` convention explicitly; one targeted `Read` answers XML-specific queries.
- **`parse/syntax.rs` — quoting helpers and `TextWrapper`/`find_split_position`** (`:136-349`, ~1510 tokens total) — `quote_field_name`, `StringValue` (decision tree for quoting strings), `is_value_complex` (used by yaml/toml/hcl), and the wrap-mode machinery (`WRAP_SYMBOL = "⤷ "`, Unicode-safe `find_split_position`). The render entry point (3.17) surfaces the hook-points.
- **`Key::from_event`** (`src/config/keys.rs:74-120`, ~312 tokens) — `KeyEvent` → `Key` translation with CONTROL/ALT/SHIFT handling. Implied by 4.8.
- **`Key::parse` + `parse_keys`** (`src/config/keys.rs:121-192`, ~570 tokens) — config-string decoder for `<ctrl-x>` / `<alt-x>` / `<fN>` / named specials and the "no duplicate binding" invariant. Needed only for config-validation debugging; 3.1's grammar notes describe the user-facing syntax.
- **`Keys` macro definitions + `get_key_action`** (`:1-52`, `:350-372`) — `paste`-based macro plumbing and `KeyEvent`-to-`Action` dispatch.
- **`DataColors`, `FooterColors`, `PopupColors`, `FilterColors` structs + defaults** (`src/config/colors.rs:77-159` and `:250-339`) — once 4.9 + 4.11 + 5.1 are in context, the remaining four color groups are formulaic.
- **`Types` struct** (`src/config/types.rs`, 261 tokens) — six strings (`str`, `null`, `bool`, `num`, `arr`, `obj`) for tree item labels, generated via `paste` macro. Surfaced indirectly through 4.10 (`build_item_text` uses `cfg.types.*`); defaults visible in 5.1.
- **`DataBlock::draw` + `update_scroll`** (`src/ui/data_block.rs:147-254`, 710 tokens) — `Paragraph::new(text).scroll((v, h))` rendering with gated vertical/horizontal scrollbars and identify-change-triggered reset. Structure implied by 4.4's fields.
- **`ui/footer.rs`** (712 tokens) — `FooterText::{Identify, Message, None}` and the narrow-terminal ellipsis logic (`/<first_char>..`, `<N>..` collapse).
- **`ui/popup.rs`** (840 tokens) — centered 50%×50% popup with scroll; simple drawing widget for errors and the help overlay.
- **`live_reload.rs`** (876 tokens) — `FileWatcher` struct + `notify`-backed `watch_file` thread; integration points (`fw.get_err`, `fw.parse_tree`) are visible in 3.3 + 3.9. One `Read` when relevant.
- **`App` — `refresh_area` layout math** (`src/ui/app.rs:305-383`, 542 tokens) — tiny-terminal fallbacks and the `LayoutDirection::{Vertical, Horizontal}` split. Field hints via `App` struct (2.9).
- **`App` — mouse + focus handlers** (`:585-665`, 551 tokens) — `on_click`, `on_scroll`, `on_focus_changed`, `get_row_inside`.
- **`App` — `build_edit` + `get_copy_text` + `popup_help`** (`:675-767`, 683 tokens) — selected-item edit-command construction (scalars → `.txt`, containers → serialized via `parser.extension()`), copy-text selection per `Action::CopyName`/`CopyValue`, help-popup build via `serde_json::to_value(&cfg.keys)` relying on `preserve_order`.
- **`TreeOverview::expand_children` / `expand_all`** (`:176-239`, ~420 tokens) — mechanical recursion over `TreeItem`s behind the `x`/`X` actions.
- **`TreeOverview::nav_filter_match`** (`:409-481`, ~420 tokens) — `n`/`N` iteration through `filter_cache`; structure implied by 4.2.
- **`TreeOverview::draw` + `build_title`** (`:254-318`, 474 tokens) — `TreeWidget` build with scrollbar and `"[pos/count]"` filter-count title (capped at `>9999`).
- **`debug.rs`** (206 tokens) — tiny opt-in file logger gated on `--debug`.
- **`config/themes/catppuccin.toml`** (650 tokens) — worked theme example; inferable from 4.9 + 4.11 + 5.1.
- **`examples/*` bodies** — nine input files; existence named in 1.2.
- **`docs/changelog.md:70-156`** (v0.1 → v0.5.1) — superseded by 2.2 + the demoted Roadmap-part-2 item above for "what features exist?".
- **Test modules at the bottom of each `parse/*.rs`** — `include_str!` assertions against `test_cases/`; 5.3 + 3.16 together tell the agent these exist.
- **`docs/colors.md` body** — zero-byte; noted in 1.7.
