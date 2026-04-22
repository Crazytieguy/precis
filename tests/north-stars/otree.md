# otree — North Star

Revision pin: `a02bdf44`

`otree` is a Rust TUI (built on ratatui + tui-tree-widget) that opens a
structured-data file (JSON / YAML / TOML / XML / HCL / JSONL) or stdin and
lets the user browse it as a tree. It has a tree-overview pane, a data-block
pane with syntax highlighting, filter mode, popup help, optional
header/footer, file live-reload, readonly external-editor open, and
clipboard copy. Configuration (TOML) covers key bindings, colors/palette,
layout, and type labels. Entry point: `src/main.rs::run()` → `cmd::CommandArgs::parse` → `Tree::parse` → `ui::start(App)`.

The ranking is tuned so a small budget locates every subsystem (folder/file
listings, top-level data-model types, action inventory), a medium budget
exposes module boundaries and the wiring logic (CLI surface, `Config`
schema, parser trait, app event loop), and a larger budget supplies the
implementation bodies of the value-heavy pieces (parsers, syntax renderer,
tree builder, filter logic). The fixture has ~42k tokens of source; ranked
content sums to ~20k and the remainder is articulated in below-the-fold.
Early tiers grow gradually so the 2× rule holds at every split; several
heavier items use elision markers (`…`) to tease a signature cheaply and
demote the body.

## Batches

### 1.1 Fixture root listing
- Content: the 14 top-level entries of `tests/fixtures/otree/` — `.github/`, `.gitignore`, `.precis-pin`, `Cargo.lock`, `Cargo.toml`, `LICENSE`, `README.md`, `assets/`, `build.rs`, `config/`, `docs/`, `examples/`, `src/`, `typos.toml`.
- Cost: 39 tokens (helper: 14 names through `count-tokens.py --stdin`)
- Notes: orients the agent. Reveals the custom `build.rs`, `config/` tree, `docs/`, `examples/`.

### 1.2 `src/` folder listing (first level)
- Content: 7 files and 3 sub-folders under `src/` — `clipboard.rs`, `cmd.rs`, `debug.rs`, `edit.rs`, `live_reload.rs`, `main.rs`, `tree.rs`, `config/`, `parse/`, `ui/`.
- Cost: 28 tokens (helper: 10 names through `--stdin`)
- Notes: single most orientation-dense batch — names reveal the subsystem decomposition (clipboard, external-editor, live-reload, debug-logging each hint at feature areas).

### 1.3 Module declarations in `src/main.rs`
- Content: `src/main.rs:1-9` (the nine `mod …;` lines).
- Cost: 28 tokens (helper: `count-tokens.py src/main.rs:1-9`)
- Notes: confirms no hidden modules.

### 1.4 `src/ui/` file listing
- Content: 8 files under `src/ui/` — `app.rs`, `data_block.rs`, `filter.rs`, `footer.rs`, `header.rs`, `mod.rs`, `popup.rs`, `tree_overview.rs`.
- Cost: 27 tokens (helper: 8 names through `--stdin`)

### 1.5 `src/parse/` file listing
- Content: 9 files + `test_cases/` sub-folder under `src/parse/` — `any.rs`, `hcl.rs`, `json.rs`, `jsonl.rs`, `mod.rs`, `syntax.rs`, `test_cases/`, `toml.rs`, `xml.rs`, `yaml.rs`.
- Cost: 33 tokens (helper: 10 names through `--stdin`)
- Notes: enumerates every supported format by filename (HCL and JSONL beyond the README tagline), and names the `test_cases/` golden-output corpus.

### 1.6 `src/config/` file listing
- Content: `colors.rs`, `keys.rs`, `mod.rs`, `types.rs`.
- Cost: 12 tokens (helper: 4 names through `--stdin`)

### 1.7 `docs/` + `config/` + `config/themes/` listings
- Content: `docs/{actions.md, changelog.md, colors.md}`; `config/{default.toml, themes/}`; `themes/{catppuccin.toml}`.
- Cost: 23 tokens (helper: three listings summed — 10 + 13 ≈ 23)
- Notes: `docs/colors.md` is zero-byte in this revision — a notable gotcha worth surfacing.

### 1.8 `examples/` + `assets/` + `.github/` listings
- Content: `examples/{edge.json, edge.toml, edge.yaml, example.hcl, example.json, example.jsonl, example.toml, example.xml, example.yaml}`; `assets/{screenshot.png}`; `.github/{dependabot.yml, workflows/{cargo-check.yml, release-binary.yml, spell-check.yml}}`.
- Cost: 56 tokens (helper: three listings summed — 31 + 5 + 20)

### 1.9 Package identity
- Content: `Cargo.toml:1-11` (`[package]` block — `name = "otree"`, `version = "0.6.4"`, `description "A command line tool to view objects (json/yaml/toml) in TUI tree widget"`, `repository`, authors `["fioncat"]`, `edition = "2021"`, `build = "build.rs"`, `categories = ["command-line-utilities"]`).
- Cost: 95 tokens (helper: `count-tokens.py Cargo.toml:1-11`)

### 1.10 README tagline
- Content: `README.md:1-5` (title "OTree - Object Tree TUI Viewer", screenshot ref, tagline "A command line tool to view objects (JSON/YAML/TOML/XML) in TUI tree widget").
- Cost: 41 tokens (helper: `count-tokens.py README.md:1-5`)
- Notes: tagline lists JSON/YAML/TOML/XML but the code also supports HCL and JSONL (see 1.5, 2.5).

### 1.11 `SyntaxToken` enum
- Content: `src/parse/syntax.rs:11-27` (ten variants: `Symbol(&'static str)`, `Name(String)`, `Tag(String)`, `String(String)`, `Number(String)`, `Null(&'static str)`, `Bool(&'static str)`, `Section(String)`, `Break`, `Indent(usize)`).
- Cost: 56 tokens (helper: `count-tokens.py src/parse/syntax.rs:11-27`)
- Notes: load-bearing IR every parser emits; feeds both the data-block renderer and the `--to` conversion path.

### 2.1 `Key` enum
- Content: `src/config/keys.rs:53-72` (variants: `Char(char)`, `Ctrl(char)`, `Alt(char)`, `F(u8)`, `Backspace`, `Enter`, `Left`, `Right`, `Up`, `Down`, `PageUp`, `PageDown`, `Tab`, `Esc`).
- Cost: 68 tokens (helper: `count-tokens.py src/config/keys.rs:53-72`)

### 2.2 README usage and config pointers
- Content: `README.md:33-46` (usage examples `otree /path/to/file.{json,yaml,toml,xml}`, `otree --help`, config at `~/.config/otree.toml`, pointers to `docs/actions.md` and `docs/colors.md`).
- Cost: 103 tokens (helper: `count-tokens.py README.md:33-46`)

### 2.3 `Filter` state types
- Content: `src/ui/filter.rs:13-38` (struct `Filter { cfg, text_area, target, ignore_case }`, enum `FilterAction { Edit, Confirm, Skip, Quit }`, struct `FilterOptions { text, target, ignore_case }`, enum `FilterTarget { Key, Value, All }`).
- Cost: 100 tokens (helper: `count-tokens.py src/ui/filter.rs:13-38`)

### 2.4 `ContentType::new_parser` dispatch
- Content: `src/parse/mod.rs:66-78` (match from `ContentType` variant → concrete `Box<dyn Parser>`: `Json→JsonParser`, `Yaml→YamlParser`, `Toml→TomlParser`, `Xml→XmlParser`, `Hcl→HclParser`, `Jsonl→JsonlParser`, `Any→AnyParser::new()`).
- Cost: 131 tokens (helper: `count-tokens.py src/parse/mod.rs:66-78`)

### 2.5 `ContentType` enum (with module header)
- Content: `src/parse/mod.rs:1-34` (module submod declarations, `SyntaxToken` re-export, `pub enum ContentType { Json, Yaml, Toml, Xml, Hcl, Jsonl, Any }` with doc comments pointing to HCL spec and jsonlines.org).
- Cost: 193 tokens (helper: `count-tokens.py src/parse/mod.rs:1-34`)

### 2.6 `SyntaxToken` file teaser with elision
- Content: `src/parse/syntax.rs:1-10` (`use` statements — `ratatui::style::Style`, `ratatui::text::{Line,Span,Text}`, `regex::Regex`, `serde_json::Value`, `crate::config::Config`) then an elision marker `…` standing in for the ~3000-token remainder (`render`, `pure_text`, `quote_field_name`, `StringValue`, `is_value_complex`, `TextWrapper`, `find_split_position`, tests).
- Cost: 55 tokens (helper: `count-tokens.py src/parse/syntax.rs:1-10`)
- Predecessor: 1.11
- Notes: elision marker tells the agent "fetch rest via `Read`"; 4.15/4.16 pin the interesting sub-bodies by line range.

### 2.7 `Parser` trait + default `parse_root`
- Content: `src/parse/mod.rs:36-64` (trait methods `extension`, `allow_array_root`, `parse`, `syntax_highlight`; and the default `parse_root` that rejects scalar roots, enforces `allow_array_root` for array roots, and errors "schema '…' does not allow array as root").
- Cost: 221 tokens (helper: `count-tokens.py src/parse/mod.rs:36-64`)
- Notes: core abstraction for "how do I add/use a format?".

### 2.8 `Types` struct (type labels shown in tree rows)
- Content: `src/config/types.rs` (whole, 48 lines — six fields `str`, `null`, `bool`, `num`, `arr`, `obj` defaulted via `generate_types_default!` macro to the field name itself).
- Cost: 261 tokens (helper: `count-tokens.py src/config/types.rs`)

### 2.9 `Tree`, `ItemValue`, `FieldType` type declarations
- Content: `src/tree.rs:1-50` (imports, `pub struct Tree { parser, items, values, identifies, cfg }`, `pub struct ItemValue { name, value, field_type, description, tokens }`, `pub struct HighlightKeyword { text, ignore_case }`, `pub enum FieldType { Null, Num, Bool, Str, Obj, Arr }`).
- Cost: 248 tokens (helper: `count-tokens.py src/tree.rs:1-50`)
- Notes: central data model connecting parser output to the UI.

### 2.10 `Config` top-level struct
- Content: `src/config/mod.rs:16-50` (`#[derive(Serialize, Deserialize)] pub struct Config { tree, editor, data, layout, header, footer, filter, palette, colors, types, keys }` with `#[serde(default=…)]`).
- Cost: 197 tokens (helper: `count-tokens.py src/config/mod.rs:16-50`)
- Notes: authoritative schema outline for `~/.config/otree.toml`.

### 2.11 `App` struct + focus/refresh enums
- Content: `src/ui/app.rs:27-90` (`enum Refresh { Update, Skip, Quit, Edit(Box<Edit>) }`, `enum ElementInFocus { TreeOverview, DataBlock, Popup, Filter, None }`, `enum ScrollDirection`, `pub struct App { … }` fields, `pub enum ShowResult`).
- Cost: 281 tokens (helper: `count-tokens.py src/ui/app.rs:27-90`)
- Notes: central state the whole TUI revolves around.

### 2.12 Action enum variants
- Content: `src/config/keys.rs:317-348` (`generate_actions!` macro invocation listing all 30 variants — `MoveUp/Down/Left/Right`, `SelectFocus/Parent/First/Last`, `CloseParent`, `ChangeRoot`, `Reset`, `PageUp/Down`, `ChangeLayout`, `TreeScaleUp/Down`, `Switch`, `Edit`, `CopyName/Value`, `Filter/FilterKey/FilterValue`, `FilterNextMatch/PrevMatch`, `FilterSwitchIgnoreCase`, `ExpandChildren/All`, `ShowHelp`, `Quit`).
- Cost: 216 tokens (helper: `count-tokens.py src/config/keys.rs:317-348`)

### 2.13 Default key bindings
- Content: `src/config/keys.rs:284-315` (`generate_keys_default!` — each action → default key string(s): `move_up => ["k","<up>"]`, `select_focus => ["<enter>"]`, `change_root => ["r"]`, `filter => ["/"]`, `filter_key => ["?"]`, `filter_value => ["*"]`, `quit => ["<ctrl-c>","q"]`, …).
- Cost: 270 tokens (helper: `count-tokens.py src/config/keys.rs:284-315`)

### 2.14 `Cargo.toml` dependencies
- Content: `Cargo.toml:13-36` (`[dependencies]` — `anyhow`, `clap`, `console`, `crossterm`, `dirs`, `hcl-rs`, `humansize`, `notify`, `paste`, `quick-xml`, `ratatui`, `regex`, `serde`, `serde_json (preserve_order)`, `serde_yml`, `strum`, `toml (preserve_order)`, `tui-textarea`, `tui-tree-widget`) plus `[build-dependencies]` (`simple-error`, `vergen`).
- Cost: 229 tokens (helper: `count-tokens.py Cargo.toml:13-36`)

### 2.15 `HeaderContext` + format placeholders
- Content: `src/ui/header.rs:11-39` (`pub struct HeaderContext { version, data_source, content_type, data_size }`, `new` (source → `"stdin"` when `None`, size via `humansize::format_size(_, BINARY)`), `fn format` substituting `{version}/{data_source}/{content_type}/{data_size}`).
- Cost: 211 tokens (helper: `count-tokens.py src/ui/header.rs:11-39`)

### 2.16 `CommandArgs` — file/config/output flags
- Content: `src/cmd.rs:11-40` (`#[derive(Parser)] pub struct CommandArgs` start — positional `path: Option<String>` (stdin if absent), `--config`, `-t/--content-type`, `-o/--to`, `--disable-header`, `--disable-footer`, `--disable-filter`).
- Cost: 219 tokens (helper: `count-tokens.py src/cmd.rs:11-40`)

### 2.17 `Config` sub-structs
- Content: `src/config/mod.rs:52-125` (`Tree { disable_selected_highlight, selected_symbol }`, `Editor { program, args, dir }`, `Layout { direction, tree_size }`, `enum LayoutDirection { Vertical, Horizontal }`, `Header { disable, format }`, `Footer { disable }`, `Filter { disable, ignore_case, exclude_mode }`, `Data { disable_highlight, max_data_size, wrap }`).
- Cost: 431 tokens (helper: `count-tokens.py src/config/mod.rs:52-125`)
- Predecessor: 2.10

### 3.1 `docs/actions.md` action↔keys↔description table
- Content: `docs/actions.md:1-34` (full table `action | default keys | description` for all 30 actions, including filter-mode overloads for `move_left/right`, `select_focus/first/last`, `close_parent`, and `reset`).
- Cost: 657 tokens (helper: `count-tokens.py docs/actions.md:1-34`)
- Notes: highest-value doc for "what does each keypress do?". Zero-follow-up answer to most key/action queries. 657 ≤ 2×431 (max-above is 2.17) ✓.

### 3.2 `CommandArgs` — UI toggles and layout
- Content: `src/cmd.rs:41-75` (`--tree-disable-selected-highlight`, `--tree-selected-symbol`, `--filter-ignore-case`, `--filter-exclude-mode`, `-f/--header-format`, `-V/--vertical`, `-H/--horizontal`, `-s/--size` range `[10,80]`, `--disable-highlight`).
- Cost: 225 tokens (helper: `count-tokens.py src/cmd.rs:41-75`)
- Predecessor: 2.16

### 3.3 `CommandArgs` — misc flags
- Content: `src/cmd.rs:76-109` (`-w/--wrap`, `--show-config`, `--ignore-config`, `--build-info`, `--max-data-size <MiB>`, `-R/--live-reload`, `--debug <file>`, `-v/--version`).
- Cost: 201 tokens (helper: `count-tokens.py src/cmd.rs:76-109`)
- Predecessor: 3.2

### 3.4 `CommandArgs::get_content_type` (extension → `ContentType`)
- Content: `src/cmd.rs:202-228` (match: `json→Json`, `yaml|yml→Yaml`, `toml→Toml`, `xml→Xml`, `hcl→Hcl`, `jsonl→Jsonl`, else `Any`; falls back to `Any` if no path / no extension).
- Cost: 192 tokens (helper: `count-tokens.py src/cmd.rs:202-228`)
- Notes: definitive answer to "what file extensions does otree recognize?".

### 3.5 `CommandArgs::update_config` — CLI → `Config` propagation
- Content: `src/cmd.rs:149-201` (applies every CLI override onto `Config`: `vertical/horizontal` → `layout.direction`, `disable_*` toggles, `tree_selected_symbol`, `filter_ignore_case/exclude_mode`, `disable_highlight`, `wrap`, `header_format`, `size` → `layout.tree_size`).
- Cost: 252 tokens (helper: `count-tokens.py src/cmd.rs:149-201`)
- Predecessor: 3.3

### 3.6 `config/default.toml` — top sections
- Content: `config/default.toml:1-29` (`[tree]`, `[editor]` with `vim` + `{file}` arg + `/tmp` dir, `[layout] direction="horizontal" tree_size=40`, `[header] format="{version} - {data_source} ({content_type}) - {data_size}"`, `[footer]`, `[data] max_data_size=30`, `[filter]`).
- Cost: 113 tokens (helper: `count-tokens.py config/default.toml:1-29`)

### 3.7 `config/default.toml` — `[keys]`
- Content: `config/default.toml:30-60` (every default key binding in the exact TOML format a user edits — `move_up = ["k","<up>"]`, … `quit = ["<ctrl-c>","q"]`).
- Cost: 228 tokens (helper: `count-tokens.py config/default.toml:30-60`)

### 3.8 `docs/changelog.md` — v0.6.x entries
- Content: `docs/changelog.md:1-45` (v0.6.4 XML empty-node-with-fields fix; v0.6.3 content-type auto-detection enabling `echo '{…}' | otree`; v0.6.2 `data.wrap` / `--wrap` mode; v0.6.1 HCL support + `-o`/`--to` conversion + narrow-footer display + TOML section names).
- Cost: 341 tokens (helper: `count-tokens.py docs/changelog.md:1-45`)

### 3.9 `main.rs::run()` — args → config → data read
- Content: `src/main.rs:27-77` (parses args, loads `Config::default`/`Config::load`, applies `args.update_config(&mut cfg)`, sets up `debug::set_file` if `--debug`, derives `content_type` from extension, optionally creates `FileWatcher` for `--live-reload`, reads file or stdin bytes).
- Cost: 334 tokens (helper: `count-tokens.py src/main.rs:27-77`)

### 3.10 `main.rs::run()` — `--to` conversion + size check + UI start
- Content: `src/main.rs:79-116` (`--to target_type` branch prints converted output via `SyntaxToken::pure_text`; `max_data_size` bail with `humansize`-formatted hint; `Tree::parse` + `App::new` + `HeaderContext::new` + `ui::start`; top-level `fn main` → exits 1 with `eprintln!("Error: {err:#}")`).
- Cost: 324 tokens (helper: `count-tokens.py src/main.rs:79-116`)
- Predecessor: 3.9

### 3.11 `Tree::parse` and `Tree::from_value`
- Content: `src/tree.rs:51-92` (construct parser, call `parser.parse_root(None, data)`, then `from_value` — expands `Value::Array`/`Value::Object` at the root (no visible "root" node) and wraps scalars as `String::from("root")`).
- Cost: 329 tokens (helper: `count-tokens.py src/tree.rs:51-92`)
- Predecessor: 2.9

### 3.12 `ui::start` + terminal lifecycle
- Content: `src/ui/mod.rs:24-87` (`get_border_style`, `fn start` main loop handling `ShowResult::{Edit, Quit}`, `new_terminal` using `EnterAlternateScreen`+`EnableMouseCapture`, `restore` — always restores terminal even on error; `ShowResult::Edit` tears down + re-creates the terminal around editor run).
- Cost: 452 tokens (helper: `count-tokens.py src/ui/mod.rs:24-87`)
- Predecessor: 2.11

### 3.13 `TreeOverview` struct + `on_key` dispatch
- Content: `src/ui/tree_overview.rs:19-90` (struct fields — `state`, `filter_items`, `filter_cache`, `filter_nav_pos`, `before_filter_state`, `last_switches`, `root_switch`, `root_identifies`; constants `DEFAULT_HIGHLIGHT_SYMBOL = "→ "`, `MAX_FILTER_COUNT_DISPLAY = 9999`; `on_key` mapping `Action::MoveUp/Down/SelectFocus/…/ChangeRoot/ExpandChildren/ExpandAll/FilterNextMatch/PrevMatch/Reset` to tree-state ops).
- Cost: 559 tokens (helper: `count-tokens.py src/ui/tree_overview.rs:19-90`)
- Predecessor: 2.11

### 3.14 `docs/actions.md` — key-syntax reference
- Content: `docs/actions.md:35-51` ("All available keys" grammar — `<up>`, `<ctrl-x>`, `<alt-x>`, `<fN>` with `n ≤ 12`; rebind example `[keys] select_focus = [" ", "<enter>"]`).
- Cost: 179 tokens (helper: `count-tokens.py docs/actions.md:35-51`)
- Predecessor: 3.1

### 3.15 README roadmap
- Content: `README.md:52-81` ("Roadmap" — every checked-off feature with version: Header/Tree Overview/Data Block (v0.1), Footer (v0.2), Filter Input (v0.5), Popup (v0.2), filter-keyword highlighting (v0.6), change-root / back-to-root / scale-up-down / mouse click+scroll / edit-in-editor (v0.2) / `y`+`Y` clipboard copy (v0.2) / switch panes / jump-to-parent / expand-children (v0.5) / expand-all (v0.5) / close-all / help popup (v0.5) / syntax-highlighting (v0.2) / customizable colors+keys / filter items (v0.5) / `--debug` (v0.4)).
- Cost: 475 tokens (helper: `count-tokens.py README.md:52-81`)
- Notes: very high value per token for "what can otree do?".

### 4.1 `App::new` + constants + `show` loop
- Content: `src/ui/app.rs:91-163` (`HEADER_HEIGHT=1`, `FOOTER_HEIGHT=1`, `FILTER_HEIGHT=3`, `POLL_EVENT_DURATION=100ms`, `HELP_URL = "https://github.com/fioncat/otree/blob/main/docs/actions.md"`; `fn new` wiring every sub-widget, `set_header`, `fn show` outer loop delegating to `refresh`/`refresh_with_fw` depending on `FileWatcher` presence).
- Cost: 514 tokens (helper: `count-tokens.py src/ui/app.rs:91-163`)
- Predecessor: 2.11

### 4.2 `App::on_key` — action dispatch (event-loop core)
- Content: `src/ui/app.rs:393-584` (filter-mode routing through `Filter::on_key`, then per-`Action` arms: `Quit` with popup-dismiss short-circuit, `Switch` enforcing `can_switch_to_data_block`, `ChangeLayout`, `TreeScaleUp/Down` clamped to `Config::MIN_LAYOUT_TREE_SIZE=10`/`MAX_LAYOUT_TREE_SIZE=80` with step 2, `Edit` via `build_edit`, `CopyName/Value` via `write_clipboard` + footer "copied N B to system clipboard", `Filter/FilterKey/FilterValue` toggling `FilterTarget`, `FilterSwitchIgnoreCase`, `ShowHelp` popup; default arm delegating to `tree_overview.on_key` / `data_block.on_key` / `popup.on_key`).
- Cost: 1205 tokens (helper: `count-tokens.py src/ui/app.rs:393-584`)
- Predecessor: 2.11
- Notes: indispensable for "what happens when I press X?". Max-above = 657; 1205 ≤ 1314 ok.

### 4.3 `Tree::build_item` — recursive tree-item construction
- Content: `src/tree.rs:102-276` (maps each `serde_json::Value` variant to a `TreeItem` + `ItemValue`, computes `"{ N fields }"` / `"[ N items ]"` descriptions, recurses into arrays/objects, invokes `parser.syntax_highlight(data_name, &raw_value)` to populate `tokens` used by the data block; path-pushing into `identifies`).
- Cost: 1098 tokens (helper: `count-tokens.py src/tree.rs:102-276`)
- Predecessor: 3.11

### 4.4 `App::build_edit` + `get_copy_text` + `popup_help`
- Content: `src/ui/app.rs:675-767` (scalar vs. complex value extraction — scalars → `.txt`, containers → `parser.extension()`; `get_copy_text` returns `item.name` for `CopyName` and pure-text for `CopyValue`; `popup_help` serialises `self.cfg.keys` via `serde_json::to_value` — relies on `preserve_order` for stable action ordering).
- Cost: 683 tokens (helper: `count-tokens.py src/ui/app.rs:675-767`)
- Predecessor: 4.1

### 4.5 `TreeOverview::change_root` / `reset`
- Content: `src/ui/tree_overview.rs:92-147` (`change_root` only accepts `Array`/`Object` values, stashes current `(Tree, TreeState)` onto `root_switch` (first switch) or `last_switches`, pushes `root_identifies`; `reset` pops back; `close_parent`, `select_parent`, `get_selected_parent` helpers).
- Cost: 348 tokens (helper: `count-tokens.py src/ui/tree_overview.rs:92-147`)
- Predecessor: 3.13

### 4.6 `TreeOverview::expand_children` / `expand_all`
- Content: `src/ui/tree_overview.rs:176-238` (recursion `expand_all_recursively` / `close_all_recursively`; `x` toggles the selected subtree; `X` toggles every root).
- Cost: 423 tokens (helper: `count-tokens.py src/ui/tree_overview.rs:176-238`)
- Predecessor: 3.13

### 4.7 `TreeOverview::filter` + `filter_item`
- Content: `src/ui/tree_overview.rs:335-408` (walks tree items via `filter_item`, tests `opts.filter(item_value)`, `open_parent` expands ancestors of matches, `exclude_mode = false` keeps unmatched visible with only matches styled; builds `filter_cache: HashMap<path, match_index>`).
- Cost: 473 tokens (helper: `count-tokens.py src/ui/tree_overview.rs:335-408`)
- Predecessor: 3.13

### 4.8 `JsonParser` + shared `json::highlight`
- Content: `src/parse/json.rs:6-82` (both `impl Parser for JsonParser` — `allow_array_root = true` — and the free `pub fn highlight(value, indent, has_next)` emitting `Indent`/`Break`/`Symbol(",")`/`Symbol(": ")` for each `Value` variant; TOML/HCL/JSONL all delegate here for simple arrays).
- Cost: 587 tokens (helper: `count-tokens.py src/parse/json.rs:6-82`)
- Predecessor: 2.7

### 4.9 `YamlParser` impl + multi-doc handling
- Content: `src/parse/yaml.rs:8-70` (deserializer loop collecting multiple docs into `Value::Array`, "complex array → multi-doc with `---` separators, flat array → inline" heuristic; `allow_array_root = true`).
- Cost: 401 tokens (helper: `count-tokens.py src/parse/yaml.rs:8-70`)
- Predecessor: 2.7

### 4.10 `TomlParser` impl + `toml_value_to_json`
- Content: `src/parse/toml.rs:9-86` (`syntax_highlight` entry carries `section` name through and trims leading `Break`; `allow_array_root = false`; `toml_value_to_json` converting `TomlValue::{String,Integer,Float,Boolean,Datetime,Array,Table}` → `serde_json::Value`; `Number::from_f64` fallback-to-`Number::from(0)`, datetime-as-string; TOML has no `null` — emits `""` for `Value::Null`).
- Cost: 553 tokens (helper: `count-tokens.py src/parse/toml.rs:9-86`)
- Predecessor: 2.7

### 4.11 `XmlParser::parse` + teaser
- Content: `src/parse/xml.rs:12-36` (`impl Parser for XmlParser` with `allow_array_root = false`, `expand_empty_elements = true`; body delegates to top-level `read(reader, 0)`) then an elision marker `…`. The full parse mechanics — `NodeValues` helper collecting `@attr`-prefixed attributes, `#text`, `#cdata`, duplicate-key → array merging, `read` recursive loop handling `Start`/`Text`/`CData`/`GeneralRef`/`End`/`Eof`, `highlight` reconstructor — live at `src/parse/xml.rs:37-350` (~2200 more tokens).
- Cost: 136 tokens (helper: `count-tokens.py src/parse/xml.rs:12-36`)
- Predecessor: 2.7
- Notes: elision marker is the catastrophic-omission guard — XML's `@attr`/`#text`/`#cdata` conventions are quirky enough the agent must know they exist even when we can't afford the body (the v0.5.2 changelog entry — demoted below-the-fold — explains the convention).

### 4.12 `HclParser` impl + teaser
- Content: `src/parse/hcl.rs:8-26` (`impl Parser for HclParser` — `extension = "hcl"`, `allow_array_root = false`, `parse` via `hcl::from_str`, `syntax_highlight` delegating to `highlight(name, value, 0)`) then `…`. The `highlight` body at `hcl.rs:28-105` (~574 more tokens) emits `name = value` for simple fields, `name { … }` blocks for nested objects, falling back to `json_highlight` for arrays.
- Cost: 117 tokens (helper: `count-tokens.py src/parse/hcl.rs:8-26`)
- Predecessor: 2.7

### 4.13 `JsonlParser` impl
- Content: `src/parse/jsonl.rs:7-38` (line-by-line `serde_json::from_str` with empty-line-skip, always yielding `Value::Array`; `syntax_highlight` delegates to `json::highlight`; `extension()` returns `"json"` so the editor temp file is `.json`).
- Cost: 230 tokens (helper: `count-tokens.py src/parse/jsonl.rs:7-38`)
- Predecessor: 2.7

### 4.14 `AnyParser` (content-type auto-detection)
- Content: `src/parse/any.rs` whole file (`RefCell<Option<Box<dyn Parser>>>` holding the detected parser; `parse_root` iterating `ContentType::iter()` skipping `Any` and installing the first parser whose `parse_root` succeeds — implements `echo '{"key":"value"}' | otree`).
- Cost: 365 tokens (helper: `count-tokens.py src/parse/any.rs`)
- Predecessor: 2.7

### 4.15 `SyntaxToken::pure_text` + complexity + quoting helpers
- Content: `src/parse/syntax.rs:110-134` (text extractor used by `--to` and `copy_value`), `:136-149` (`STANDARD_FIELD_NAME_RE = ^[a-zA-Z0-9_-]+$` + `quote_field_name`), `:201-214` (recursive `is_value_complex` driving TOML/HCL section promotion).
- Cost: 377 tokens (helper: `count-tokens.py src/parse/syntax.rs:110-134 src/parse/syntax.rs:136-149 src/parse/syntax.rs:201-214`)
- Predecessor: 1.11

### 4.16 `StringValue` (multiline + numeric-looking quoting)
- Content: `src/parse/syntax.rs:151-199` (`String`/`MultiLines` enum rules: empty → `"\"\""`, contains `\n` with non-whitespace → `MultiLines`, looks numeric/boolean → must quote, matches standard field regex → bare, else quote).
- Cost: 387 tokens (helper: `count-tokens.py src/parse/syntax.rs:151-199`)
- Predecessor: 1.11

### 4.17 `DataBlock` struct + entrypoints
- Content: `src/ui/data_block.rs:1-67` (imports, struct fields `can_vertical_scroll` / `vertical_scroll_last` / `ScrollbarState` / `identify` tracking selection-change reset, `SCROLL_RETAIN = 5`; `new`; `on_key` mapping `Action::Move*/SelectFirst/SelectLast` and `on_scroll` mapping `ScrollDirection::{Up,Down}` to scroll helpers).
- Cost: 441 tokens (helper: `count-tokens.py src/ui/data_block.rs:1-67`)

### 4.18 `FilterOptions::filter` + `contains`
- Content: `src/ui/filter.rs:152-187` (match predicate: `Key` checks `item.name`, `Value` stringifies `Value::String`/`Number`, `All` ORs key and value; empty-text never matches; ignore-case via `to_lowercase`).
- Cost: 217 tokens (helper: `count-tokens.py src/ui/filter.rs:152-187`)
- Predecessor: 2.3

### 4.19 `Header::new` + `Header::draw`
- Content: `src/ui/header.rs:41-61` (centered `Paragraph` with `cfg.colors.header.style`, rendering the pre-formatted string from `HeaderContext`).
- Cost: 142 tokens (helper: `count-tokens.py src/ui/header.rs:41-61`)
- Predecessor: 2.15

### 5.1 `FileWatcher` struct + public API
- Content: `src/live_reload.rs:16-91` (`new` spawning `watch_file` thread via `notify::recommended_watcher`, `get_err`/`parse_tree` polled by `App::refresh_with_fw`; `max_data_size` enforced on each reload).
- Cost: 441 tokens (helper: `count-tokens.py src/live_reload.rs:16-91`)

### 5.2 `clipboard.rs` OS-specific command selection
- Content: `src/clipboard.rs` whole file (macOS=`pbcopy`, Linux=`wl-copy` when `WAYLAND_DISPLAY` else `xclip -selection clipboard`, Windows=`clip`; stdin-pipe invocation with `NotFound` → "install it" error; exit-status check).
- Cost: 418 tokens (helper: `count-tokens.py src/clipboard.rs`)

### 5.3 `Colors` root struct + defaults + parse macro
- Content: `src/config/colors.rs:1-76` (`generate_colors_parse!` macro, `pub struct Colors { header, focus_border, footer, tree, data, popup, filter }`, `Colors::default`, `default_header`, `default_focus_boder` = magenta bold).
- Cost: 425 tokens (helper: `count-tokens.py src/config/colors.rs:1-76`)

### 5.4 `Color` struct + palette-aware `parse`
- Content: `src/config/colors.rs:341-399` (`pub struct Color { fg: Option<String>, bg: Option<String>, bold, italic, #[serde(skip)] style }`, `Color::new` (empty-string fields → `None`), `Color::parse` substituting palette keys first, then `ratatui::style::Color::parse`).
- Cost: 357 tokens (helper: `count-tokens.py src/config/colors.rs:341-399`)
- Predecessor: 5.3

### 5.5 `Edit` external-editor setup signatures
- Content: `src/edit.rs:1-40` (imports, `pub struct Edit { path, data, cmd }`, `Edit::new(cfg, identify, data, extension)` building a command from `cfg.editor.program`+`args`+`dir` with `{file}` → `{editor.dir}/otree_{id}.{ext}` (default `/tmp`); `identify` sanitisation `/` → `_`).
- Cost: 246 tokens (helper: `count-tokens.py src/edit.rs:1-40`)

### 5.6 `debug` macro + `set_file`
- Content: `src/debug.rs` whole file (`debug!` macro, `FILE: OnceLock<String>`, `write_logs` append-only, a `FIXME` that log errors are swallowed because stderr is owned by the TUI; `debug!` is a no-op until `--debug <file>` sets the path).
- Cost: 206 tokens (helper: `count-tokens.py src/debug.rs`)

### 5.7 `config/default.toml` — `[colors.*]` + `[types]`
- Content: `config/default.toml:62-109` (every default color for header/tree/data/footer/popup/filter sections + the `[types]` defaults — mirrors the Rust defaults in `config/colors.rs`/`types.rs`).
- Cost: 399 tokens (helper: `count-tokens.py config/default.toml:62-109`)

### 5.8 Parser test-case folder listing
- Content: `src/parse/test_cases/{hcl,json,jsonl,toml,xml,yaml}/` folder names plus the `{raw, _highlight}` pair convention (one representative pair per format — `object.json`+`object_highlight.json`, `common.yaml`+`common_highlight.yaml`, `basic.toml`+`basic_highlight.toml`, `attrs.xml`+`attrs.json`, `nested.hcl`+`nested.json`, `complex_objects.jsonl`+`complex_objects_highlight.json`).
- Cost: 90 tokens (helper: 6 folder names + 12 filenames through `--stdin`)
- Notes: single source of truth for "how are the parsers tested?". Full recursive listing is 351 tokens — see below-the-fold.

### 5.9 `examples/example.json` teaser with elision
- Content: `examples/example.json:1-15` (root object with keys `string`, `number`, `float`, `boolean`, `null`, `array`, `object`; nested `object.nested_object { deep_string, deep_number }`), then `…` — remaining keys `complex_array`, `complex_object` at lines 17-35 live in the file.
- Cost: 124 tokens (helper: `count-tokens.py examples/example.json:1-15`)

### 5.10 `typos.toml` + `.gitignore`
- Content: `typos.toml` (allowlist: `ratatui`, `donot → don't`, `caf`) and `.gitignore` (`target/`, `debug/`, `*.rs.bk`, `*.pdb`).
- Cost: 88 tokens (helper: sum of `count-tokens.py` on each — 26 + 62)

## Below-the-fold

- **`src/ui/app.rs:221-383`** — `App::draw` + `refresh_area` layout (~934 tokens). Draws header/footer/filter/tree/data/popup in order; tiny-terminal `skip_header`/`skip_footer` fallbacks; vertical/horizontal split. Mechanical given 2.11 (`App` struct) and `LayoutDirection` in 2.17. Fetchable in one `Read`.
- **`src/ui/app.rs:586-673`** — `App::on_click` / `on_scroll` / `on_focus_changed` / `get_row_inside` / `can_switch_to_data_block` (~618 tokens). Click routes tree/data/filter areas and updates focus; wheel routes to focused widget; `FocusGained`/`FocusLost` save and restore `last_focus`. Semantics fully implied by 2.11 (`App` struct) and 4.2 (`App::on_key`).
- **`src/ui/app.rs:1-26`** (~235 tokens) — imports. Module wiring recoverable from 1.4.
- **`src/tree.rs:278-322`** + **`src/tree.rs:385-411`** — `Tree::build_item_text` + `ItemValue::{plain_text, render, build_highlighted_text}` (~523 tokens). Field-type → span composition with colors from `cfg.colors.tree.*` and type label from `cfg.types`. Implied by 2.8 (`Types`), 2.9 (`FieldType`), 4.3 (`build_item`), and `TreeColors` defaults below.
- **`src/tree.rs:324-382`** — `Tree::highlight_keyword` + `Tree::string_tokens` (~385 tokens). Substring-highlight span builder and multiline-string tokenizer; behaviour implied by the prior entry and `HighlightKeyword` (2.9).
- **`src/parse/xml.rs:37-350`** — `NodeValues` helper + `read` recursive parse loop + `xml::highlight` render (~2212 tokens). `XmlParser` (4.11) flags the quirky `@attr`/`#text`/`#cdata` conventions; the v0.5.2 changelog entry documents the convention explicitly; bodies are one `Read` away. Largest single demotion.
- **`src/parse/hcl.rs:28-105`** — `HclParser::highlight` body (~574 tokens). The impl (4.12) teaches HCL exists; the emitter pattern mirrors TOML's (4.10).
- **`src/parse/toml.rs:88-193`** — `toml::highlight` body emitting `[section]` / `[[section]]` headers and `'''` multiline strings (~763 tokens). 4.10 already covers `TomlParser`'s intent and `allow_array_root = false`.
- **`src/parse/yaml.rs:72-153`** — `yaml::highlight` body (~581 tokens). Block-style rendering with `- ` array prefixes and `|` multiline branch; uses `StringValue` (4.16). 4.9 covers `YamlParser`'s intent.
- **`src/parse/syntax.rs:29-108`** — `SyntaxToken::render` (~633 tokens). Turns tokens into a ratatui `Text` with `cfg.colors.data.*` styling; branches into `TextWrapper` on `cfg.data.wrap`. Hook points named by 1.11 + 4.15/4.16 and the elision invite in 2.6.
- **`src/parse/syntax.rs:216-349`** — `TextWrapper` + `find_split_position` wrap-mode rendering (~955 tokens), including `WRAP_SYMBOL = "⤷ "` and Unicode-safe split.
- **`src/parse/syntax.rs:351-432`** — `#[cfg(test)] mod tests` for `find_split_position` and split (~630 tokens). Exercises wrap behaviour on Unicode edge cases.
- **`src/ui/data_block.rs:68-146`** — `DataBlock` scroll helpers (`scroll_first`, `scroll_last`, `scroll_down`, `scroll_up`, `scroll_right`, `scroll_left`, ~350 tokens). Mechanical given 4.17's fields.
- **`src/ui/data_block.rs:147-254`** — `DataBlock::draw` + `update_scroll` (~710 tokens). `Paragraph::new(text).scroll((v, h))` with gated vertical/horizontal scrollbars and identify-change-triggered reset. Structure implied by 4.17.
- **`src/ui/filter.rs:1-12, 40-150`** — imports + `Filter::new` + `Filter::on_key` + `Filter::draw` (~670 tokens). `FilterAction` outcomes in 2.3 are the public contract; the body wires them to `tui_textarea` cursor moves (`Back`/`Forward`/`Head`/`End`) and renders `"Filter (K|V|*|I,…)"` title.
- **`src/ui/footer.rs`** — `FooterText { Identify, Message, None }` + narrow-terminal root-crumb compression (712 tokens). Variant set visible via `App::draw`; the compression algorithm collapses roots to `/<first_ch>..` and finally to `<N>..`.
- **`src/ui/popup.rs`** — `Popup` struct + `on_key`/`on_scroll`/`disable`/`draw` + `centered_rect(50, 50, …)` (840 tokens). Help/error overlays covered by 4.4 (`popup_help`) and 4.2 (`Action::ShowHelp`).
- **`src/ui/tree_overview.rs:240-333`** — `TreeOverview::draw` + `build_title` + `begin_filter` / `end_filter` (~698 tokens). Tree title becomes `"Tree Overview [pos/count]"` capped at `>9999`. Filter lifecycle saves and restores `TreeState`.
- **`src/ui/tree_overview.rs:409-480`** — `nav_filter_match` + `open_parent` (~424 tokens). `n`/`N` iteration through `filter_cache`; structure implied by 4.7.
- **`src/live_reload.rs:93-141`** — `Data` struct + `watch_file` thread body (~440 tokens). 5.1 conveys the `FileWatcher` public contract; the thread loop is plumbing.
- **`src/edit.rs:40-103`** — `Edit::run`, `run_inner`, `write_file`, `edit_file`, `delete_file` (~430 tokens). 5.5 gives the signature surface; the disk dance is predictable.
- **`src/config/mod.rs:127-230`** — `Config::load` / `parse` / `get_path` / `default` / `show` (~678 tokens). Config-file discovery (`--config` → `OTREE_CONFIG` → `~/.config/otree.toml`), `MIN_LAYOUT_TREE_SIZE = 10` / `MAX_LAYOUT_TREE_SIZE = 80` validation, palette validation, `default_max_data_size = 30` MiB; field-level defaults captured by 2.10 + 3.6 + 5.7.
- **`src/config/mod.rs:231-329`** — `impl Default` blocks for `Tree`/`Editor`/`Layout`/`Header`/`Footer`/`Filter`/`Data` (~440 tokens). Defaults echoed in `config/default.toml`; notable: `Editor::default_program` uses env `EDITOR` then `"vim"`, `Editor::default_args = ["{file}"]`, `Editor::default_dir = "/tmp"`, `Layout::default_direction = Horizontal`, `Layout::default_tree_size = 40`.
- **`src/config/keys.rs:1-52`** — `generate_keys_default!` + `generate_actions!` macro source (~270 tokens). Expansions (2.12, 2.13) are what queries land on.
- **`src/config/keys.rs:74-192`** — `Key::from_event`, `Key::parse` grammar, `parse_keys` uniqueness check (~884 tokens). Grammar described in 3.14; needed only for config-validation debugging.
- **`src/config/keys.rs:194-282`** — `Keys` struct serde-field list (~658 tokens). TOML shape of `[keys]` recoverable from defaults (2.13), actions (2.12), and the `[keys]` section in 3.7.
- **`src/config/keys.rs:350-372`** — `KeyAction` struct + `Keys::get_key_action` (~104 tokens). Trivial event-to-action dispatch.
- **`src/config/colors.rs:77-159`** — `DataColors` struct + defaults (~492 tokens): name=`yellow italic`, tag=`blue bold`, str=`green`, num=`red`, null=`blue italic`, bool=`red bold italic`, section=`cyan bold`. Values echoed in 5.7.
- **`src/config/colors.rs:161-248`** — `TreeColors` + defaults (~500 tokens): `selected = black on light_green`, `filter_keyword = black on yellow`, `type_* = cyan bold italic`, `value = dark_gray`. Pattern formulaic; values echoed in 5.7.
- **`src/config/colors.rs:250-339`** — `FooterColors` (root / identify / message), `PopupColors` (error_text / help_name / help_value), `FilterColors::{border}` + defaults (~491 tokens). Formulaic once 5.3 + 5.4 are in context.
- **`config/themes/catppuccin.toml` (whole)** — ~650 tokens of example user theme: palette with named colors (`base`, `text`, `subtext0/1`, `blue`, `lavender`, `mauve`, `pink`, …) followed by `[colors.*]` overrides referencing them. Derivable from 5.3/5.4 struct shapes.
- **`docs/changelog.md:46-156`** — v0.5.2 XML-parser notes + older v0.1→v0.5.1 entries (~830 tokens). v0.5.2 specifically explains the `@attr`/`#text`/`#cdata` convention central to 4.11; fetch when explaining XML's data shape.
- **`README.md:6-31`** (~170 tokens) — install section (`cargo install --git`, `paru -S otree`, `brew install otree`). Install instructions rarely help the agent act; tagline (1.10) and usage (2.2) carry the weight.
- **`README.md:82-86`** — "Thanks" / origin note ("built for deep Kubernetes YAML"). Non-behavioral.
- **`src/cmd.rs:1-10` + `:111-148` + `:230-255`** — imports + `CommandArgs::parse` (clap error handling + vertical/horizontal conflict check) + `show_version` / `show_build_info` (~420 tokens total). Flag surface (2.16+3.2+3.3) + `update_config` (3.5) cover the semantics; clap error plumbing is mechanical.
- **`build.rs`** (735 tokens) — `vergen`-driven env emission for `OTREE_VERSION`/`OTREE_SHA`/`OTREE_BUILD_TYPE`/`OTREE_TARGET` and git-describe fallback with `-dev_<shortsha>` / `-uncommitted` suffixes; referenced from `cmd.rs::show_version` via `env!`. Only relevant for CI/release questions.
- **`Cargo.lock`** (~44k tokens): transitive versions never help the agent; direct deps in 2.14.
- **`Cargo.toml:38-48`** — release profile (`lto = true`, `strip = true`, `incremental = false`) + clippy pedantic lints (~85 tokens).
- **`LICENSE`** — MIT (~222 tokens); non-behavioral.
- **`assets/screenshot.png`** — binary; cannot contribute text.
- **`.github/dependabot.yml` + `.github/workflows/{cargo-check,release-binary,spell-check}.yml`** (~270 tokens) — cargo check CI, musl+darwin+windows release matrix via `taiki-e/upload-rust-binary-action`, `crate-ci/typos`, weekly dependabot grouping. Relevant only for CI/release questions; one `Read` per file.
- **`src/parse/test_cases/**/*`** (~3000 tokens of tiny schema fixtures, 40+ files): JSON/YAML/TOML/HCL/JSONL/XML raw/expected pairs. Naming convention captured in 5.8; full recursive listing is 351 tokens. Contents are fixture data, cheap to `Read` on demand.
- **`#[cfg(test)] mod test { … }` blocks at the bottom of each `src/parse/*.rs`** (~1200 tokens): all share the same shape — `include_str!("test_cases/<format>/<name>")` → parse → highlight → `assert_eq!`; round-trip in both directions for most formats. Naming derivable from 5.8.
- **`examples/*` full bodies** (beyond the 124-token teaser in 5.9, ~5k tokens total): sample data files demonstrating supported schemas. Existence listed in 1.8; contents trivial to `Read` when relevant.
- **`docs/colors.md`** — empty (0 lines) in this revision; noted in 1.7.
- **Getter/delegation methods** across widgets (`TreeOverview::{get_selected, get_value, get_parser, state, state_mut, tree, get_root_identifies}`, `Popup::{is_disabled, set_data, on_scroll}`, `DataBlock::{reset, reset_scroll_*, on_scroll}`, `Filter::{get_text, set_target, switch_ignore_case, get_options}`). Structural roles are implied by the ranked struct-field lists.
