# otree — North Star

Revision pin: `a02bdf44`

`otree` is a Rust TUI (built on ratatui + tui-tree-widget) that opens a
structured-data file (JSON / YAML / TOML / XML / HCL / JSONL) or stdin and
lets the user browse it as a tree. It has a tree-overview pane, a data-block
pane with syntax highlighting, filter mode, popup help, optional
header/footer, file live-reload, readonly external-editor open, and clipboard
copy. Configuration (TOML) covers key bindings, colors/palette, layout, and
type labels.

The ranking below is tuned so that a small budget gives the agent enough to
locate any of these subsystems, a medium budget exposes the module boundaries
and the wiring logic, and a large budget supplies the implementation bodies
of the value-heavy pieces (parsers, syntax renderer, app event loop, tree
model).

Sizing note: the early tiers grow gradually (each batch at most 2× the
largest-above) so any cut point produces a coherent slice. Elision markers
(`…`) inside batches are explicit invitations to fetch the rest via `Read`.
The fixture has roughly 42k tokens of source; ranked content targets ~20k so
that medium-sized budgets get a budget-independent coherent cross-section,
and the remainder is articulated in below-the-fold.

## Batches

### 1.1 Fixture root listing
- Content: the `ls` listing of `tests/fixtures/otree/` — 14 top-level entries (`.github/`, `.gitignore`, `.precis-pin`, `Cargo.lock`, `Cargo.toml`, `LICENSE`, `README.md`, `assets/`, `build.rs`, `config/`, `docs/`, `examples/`, `src/`, `typos.toml`).
- Cost: 39 tokens (helper: all 14 names through `count-tokens.py --stdin`)
- Notes: orients the agent; without this the snapshot reads like a heap of loose files.

### 1.2 `src/` folder listing (first level)
- Content: 7 files and 3 sub-folders under `src/` — `clipboard.rs`, `cmd.rs`, `debug.rs`, `edit.rs`, `live_reload.rs`, `main.rs`, `tree.rs`, `config/`, `parse/`, `ui/`.
- Cost: 28 tokens (helper: 10 names through `--stdin`)
- Notes: single most orientation-dense item — names reveal the subsystem decomposition.

### 1.3 Module declarations in `src/main.rs`
- Content: `src/main.rs:1-9` (the nine `mod …;` lines).
- Cost: 28 tokens (helper: `count-tokens.py src/main.rs:1-9`)

### 1.4 `src/ui/` file listing
- Content: `ls` of `src/ui/` — `app.rs`, `data_block.rs`, `filter.rs`, `footer.rs`, `header.rs`, `mod.rs`, `popup.rs`, `tree_overview.rs`.
- Cost: 36 tokens (helper: 8 names through `--stdin`)

### 1.5 `src/parse/` file listing
- Content: `ls` of `src/parse/` — `any.rs`, `hcl.rs`, `json.rs`, `jsonl.rs`, `mod.rs`, `syntax.rs`, `toml.rs`, `xml.rs`, `yaml.rs`.
- Cost: 43 tokens (helper: 9 names through `--stdin`)
- Notes: enumerates every supported format by filename.

### 1.6 `src/config/` file listing
- Content: `ls` of `src/config/` — `colors.rs`, `keys.rs`, `mod.rs`, `types.rs`.
- Cost: 17 tokens (helper: 4 names through `--stdin`)

### 1.7 `docs/` + `config/` listings
- Content: `config/{default.toml, themes/}`; `docs/{actions.md, changelog.md, colors.md}`.
- Cost: 21 tokens (helper: two `--stdin` listings summed, 8 + 13)

### 1.8 `examples/` + `assets/` + `.github/` listings
- Content: `examples/{edge.json, edge.toml, edge.yaml, example.hcl, example.json, example.jsonl, example.toml, example.xml, example.yaml}`; `assets/{screenshot.png}`; `.github/{dependabot.yml, workflows/{cargo-check.yml, release-binary.yml, spell-check.yml}}`.
- Cost: 79 tokens (helper: three listings summed, 43 + 5 + 31)

### 1.9 Package identity
- Content: `Cargo.toml:1-11` (`[package]` block — `name = "otree"`, `version = "0.6.4"`, `description` "view objects (json/yaml/toml) in TUI tree widget", `repository`, `authors`, `edition = "2021"`).
- Cost: 95 tokens (helper: `count-tokens.py Cargo.toml:1-11`)

### 1.10 README tagline
- Content: `README.md:1-5` (title "OTree - Object Tree TUI Viewer", screenshot ref, tagline "A command line tool to view objects (JSON/YAML/TOML/XML) in TUI tree widget").
- Cost: 41 tokens (helper: `count-tokens.py README.md:1-5`)

### 1.11 `SyntaxToken` enum
- Content: `src/parse/syntax.rs:11-28` (ten variants: `Symbol(&'static str)`, `Name(String)`, `Tag(String)`, `String(String)`, `Number(String)`, `Null(&'static str)`, `Bool(&'static str)`, `Section(String)`, `Break`, `Indent(usize)`).
- Cost: 56 tokens (helper: `count-tokens.py src/parse/syntax.rs:11-28`)
- Notes: load-bearing type connecting parser output and data-block rendering.

### 2.1 `Key` enum
- Content: `src/config/keys.rs:53-72` (variants: `Char(char)`, `Ctrl(char)`, `Alt(char)`, `F(u8)`, `Backspace`, `Enter`, `Left`, `Right`, `Up`, `Down`, `PageUp`, `PageDown`, `Tab`, `Esc`).
- Cost: 68 tokens (helper: `count-tokens.py src/config/keys.rs:53-72`)

### 2.2 README usage and config pointers
- Content: `README.md:33-46` (usage examples `otree /path/to/file.{json,yaml,toml,xml}`, `otree --help`, config at `~/.config/otree.toml`, pointers to `docs/actions.md` and `docs/colors.md`).
- Cost: 103 tokens (helper: `count-tokens.py README.md:33-46`)

### 2.3 `Filter` struct + `FilterAction`/`FilterTarget` + `FilterOptions`
- Content: `src/ui/filter.rs:13-38` (struct, `FilterAction { Edit, Confirm, Skip, Quit }`, `FilterOptions { text, target, ignore_case }`, `FilterTarget { Key, Value, All }`).
- Cost: 100 tokens (helper: `count-tokens.py src/ui/filter.rs:13-38`)

### 2.4 `ContentType::new_parser` dispatch
- Content: `src/parse/mod.rs:66-78` (match from `ContentType` variant → concrete `Box<dyn Parser>`: `Json→JsonParser`, `Yaml→YamlParser`, `Toml→TomlParser`, `Xml→XmlParser`, `Hcl→HclParser`, `Jsonl→JsonlParser`, `Any→AnyParser::new()`).
- Cost: 131 tokens (helper: `count-tokens.py src/parse/mod.rs:66-78`)

### 2.5 `Types` struct (type labels shown in tree rows)
- Content: `src/config/types.rs:27-48` (six fields `str`, `null`, `bool`, `num`, `arr`, `obj` — defaulted via `generate_types_default!` to the field name itself).
- Cost: 135 tokens (helper: `count-tokens.py src/config/types.rs:27-48`)

### 2.6 `Parser::parse_root` default body
- Content: `src/parse/mod.rs:45-64` (default trait method parses to `Value`, rejects scalars at the root, forbids array root unless `allow_array_root()` — produces "schema '…' does not allow array as root").
- Cost: 156 tokens (helper: `count-tokens.py src/parse/mod.rs:45-64`)
- Predecessor: 2.4

### 2.7 `Tree`, `ItemValue`, `FieldType` type shapes
- Content: `src/tree.rs:14-49` (`Tree { parser, items, values, identifies, cfg }`, `ItemValue { name, value, field_type, description, tokens }`, `HighlightKeyword { text, ignore_case }`, `FieldType {Null,Num,Bool,Str,Obj,Arr}`).
- Cost: 165 tokens (helper: `count-tokens.py src/tree.rs:14-49`)

### 2.8 `ContentType` enum and `Parser` trait
- Content: `src/parse/mod.rs:17-44` (`ContentType` enum with six real variants — `Json`, `Yaml`, `Toml`, `Xml`, `Hcl`, `Jsonl` — plus `Any` fallback, and the `Parser` trait: `extension`, `allow_array_root`, `parse`, `syntax_highlight`, `parse_root`).
- Cost: 195 tokens (helper: `count-tokens.py src/parse/mod.rs:17-44`)
- Notes: core abstraction for adding/using formats.

### 2.9 `Config` top-level struct
- Content: `src/config/mod.rs:16-50` (`Config` naming its sections: `tree`, `editor`, `data`, `layout`, `header`, `footer`, `filter`, `palette`, `colors`, `types`, `keys`).
- Cost: 197 tokens (helper: `count-tokens.py src/config/mod.rs:16-50`)

### 2.10 `App` struct and focus/refresh enums
- Content: `src/ui/app.rs:27-90` (`enum Refresh { Update, Skip, Quit, Edit(Box<Edit>) }`, `enum ElementInFocus { TreeOverview, DataBlock, Popup, Filter, None }`, `enum ScrollDirection`, `struct App { … }` fields, `pub enum ShowResult`).
- Cost: 281 tokens (helper: `count-tokens.py src/ui/app.rs:27-90`)
- Notes: central state the whole TUI revolves around.

### 2.11 Dependencies
- Content: `Cargo.toml:13-36` (`[dependencies]` — `anyhow`, `clap`, `console`, `crossterm`, `dirs`, `hcl-rs`, `humansize`, `notify`, `paste`, `quick-xml`, `ratatui`, `regex`, `serde`, `serde_json (preserve_order)`, `serde_yml`, `strum`, `toml (preserve_order)`, `tui-textarea`, `tui-tree-widget`).
- Cost: 229 tokens (helper: `count-tokens.py Cargo.toml:13-36`)

### 2.12 Action enum variants
- Content: `src/config/keys.rs:317-348` (`generate_actions!` macro invocation naming all 30 `Action` variants — `MoveUp/Down/Left/Right`, `SelectFocus/Parent/First/Last`, `CloseParent`, `ChangeRoot`, `Reset`, `PageUp/Down`, `ChangeLayout`, `TreeScaleUp/Down`, `Switch`, `Edit`, `CopyName/Value`, `Filter/FilterKey/FilterValue`, `FilterNextMatch/PrevMatch`, `FilterSwitchIgnoreCase`, `ExpandChildren/All`, `ShowHelp`, `Quit`).
- Cost: 216 tokens (helper: `count-tokens.py src/config/keys.rs:317-348`)

### 2.13 Default key bindings
- Content: `src/config/keys.rs:284-315` (each action → default key string(s): `move_up => ["k","<up>"]`, `select_focus => ["<enter>"]`, `change_root => ["r"]`, `filter => ["/"]`, `filter_key => ["?"]`, `filter_value => ["*"]`, `quit => ["<ctrl-c>","q"]`, …).
- Cost: 270 tokens (helper: `count-tokens.py src/config/keys.rs:284-315`)

### 2.14 `HeaderContext` and header format placeholders
- Content: `src/ui/header.rs:11-39` (`HeaderContext { version, data_source, content_type, data_size }` new + `format` with `{version}/{data_source}/{content_type}/{data_size}` substitutions).
- Cost: 211 tokens (helper: `count-tokens.py src/ui/header.rs:11-39`)

### 2.15 `SyntaxToken` file teaser with elision
- Content: `src/parse/syntax.rs:1-10` (`use` statements — `ratatui::style::Style`, `ratatui::text::{Line,Span,Text}`, `regex::Regex`, `Config`) then `…` marker standing in for the ~3000-token remainder (`render`, `pure_text`, `quote_field_name`, `StringValue`, `is_value_complex`, `TextWrapper`, `find_split_position`, tests).
- Cost: 65 tokens (helper: `count-tokens.py src/parse/syntax.rs:1-10`)
- Predecessor: 1.11
- Notes: elision marker tells the agent "fetch rest via `Read`"; 4.15–4.17 pin the interesting sub-bodies by line range.

### 3.1 CLI flags — file, config, output
- Content: `src/cmd.rs:11-40` (`#[derive(Parser)] struct CommandArgs` start: `path` (file or stdin), `--config`, `-t/--content-type`, `-o/--to`, `--disable-header`).
- Cost: 219 tokens (helper: `count-tokens.py src/cmd.rs:11-40`)

### 3.2 CLI flags — UI toggles and layout
- Content: `src/cmd.rs:41-75` (`--disable-footer`, `--disable-filter`, `--tree-disable-selected-highlight`, `--tree-selected-symbol`, `--filter-ignore-case`, `--filter-exclude-mode`, `-f/--header-format`, `-V/--vertical`, `-H/--horizontal`, `-s/--size` range `[10,80]`, `--disable-highlight`).
- Cost: 225 tokens (helper: `count-tokens.py src/cmd.rs:41-75`)
- Predecessor: 3.1

### 3.3 CLI flags — misc
- Content: `src/cmd.rs:76-109` (`-w/--wrap`, `--show-config`, `--ignore-config`, `--build-info`, `--max-data-size <MiB>`, `-R/--live-reload`, `--debug <file>`, `-v/--version`).
- Cost: 201 tokens (helper: `count-tokens.py src/cmd.rs:76-109`)
- Predecessor: 3.2

### 3.4 `cmd.rs` `get_content_type` (extension → ContentType)
- Content: `src/cmd.rs:202-228` (match: `json→Json`, `yaml|yml→Yaml`, `toml→Toml`, `xml→Xml`, `hcl→Hcl`, `jsonl→Jsonl`, else `Any`).
- Cost: 192 tokens (helper: `count-tokens.py src/cmd.rs:202-228`)

### 3.5 `cmd.rs::update_config` — CLI → Config propagation
- Content: `src/cmd.rs:149-201` (applies every CLI override onto `Config`: `vertical/horizontal` → `layout.direction`, `disable_*` toggles, `tree_selected_symbol`, `filter_ignore_case/exclude_mode`, `disable_highlight`, `wrap`, `header_format`, `size` → `layout.tree_size`).
- Cost: 293 tokens (helper: `count-tokens.py src/cmd.rs:149-201`)
- Predecessor: 3.3

### 3.6 `docs/changelog.md` — v0.6.x entries
- Content: `docs/changelog.md:1-45` (v0.6.4 XML empty-node fix; v0.6.3 content-type auto-detection enabling `echo '{…}' | otree`; v0.6.2 `data.wrap` / `--wrap` mode; v0.6.1 HCL + `-o`/`--to` conversion + narrow-footer display + TOML section names).
- Cost: 341 tokens (helper: `count-tokens.py docs/changelog.md:1-45`)

### 3.7 `config/default.toml` — everything above `[colors]`
- Content: `config/default.toml:1-60` (`[tree]`, `[editor]` with `vim` + `{file}` arg + `/tmp` dir, `[layout] direction="horizontal" tree_size=40`, `[header] format="{version} - {data_source} ({content_type}) - {data_size}"`, `[footer]`, `[data] max_data_size=30`, `[filter]`, full `[keys]` block).
- Cost: 341 tokens (helper: `count-tokens.py config/default.toml:1-60`)

### 3.8 `main.rs` run() — args → config → data read
- Content: `src/main.rs:27-77` (parses args, loads config, applies `args.update_config(&mut cfg)`, sets up `debug::set_file` if `--debug`, derives `content_type` from extension, optionally creates `FileWatcher` for `--live-reload`, reads file or stdin bytes).
- Cost: 334 tokens (helper: `count-tokens.py src/main.rs:27-77`)

### 3.9 `Tree::parse` and `Tree::from_value` entrypoints
- Content: `src/tree.rs:51-92` (construct parser, call `parser.parse_root(None, data)`, then `from_value` — expands `Value::Array`/`Value::Object` at the root without a visible "root" node but wraps scalars as `String::from("root")`).
- Cost: 329 tokens (helper: `count-tokens.py src/tree.rs:51-92`)
- Predecessor: 2.7

### 3.10 `main.rs` run() — `--to` conversion + size check + UI start
- Content: `src/main.rs:79-109` (`--to target_type` branch prints converted output via `SyntaxToken::pure_text`; `max_data_size` cap enforced with `humansize::format_size`; `Tree::parse` + `App::new` + `HeaderContext::new` + `ui::start`).
- Cost: 290 tokens (helper: `count-tokens.py src/main.rs:79-109`)
- Predecessor: 3.8

### 3.11 README roadmap
- Content: `README.md:52-81` ("Roadmap" — every checked-off feature with its version: Header (v0.1), Tree Overview (v0.1), Data Block (v0.1), Footer (v0.2), Filter Input (v0.5), Popup (v0.2), filter-keyword highlighting (v0.6), change-root / back-to-root / scale-up-down / mouse click+scroll / edit-in-editor (v0.2) / `y`+`Y` copy to clipboard (v0.2) / switch panes / jump-to-parent / expand-children (v0.5) / expand-all (v0.5) / close-all / help popup (v0.5) / syntax-highlighting (v0.2) / customizable colors+keys / filter items (v0.5) / `--debug` log file (v0.4)).
- Cost: 475 tokens (helper: `count-tokens.py README.md:52-81`)
- Notes: very high value per token for "what can otree do?".

### 3.12 `ui::start` + terminal lifecycle
- Content: `src/ui/mod.rs:24-87` (`get_border_style`, `start`, `new_terminal` using `EnterAlternateScreen`+`EnableMouseCapture`, `restore` — always restores terminal even on error; `ShowResult::Edit` path tears down + re-creates the terminal around editor run).
- Cost: 452 tokens (helper: `count-tokens.py src/ui/mod.rs:24-87`)
- Predecessor: 2.10

### 3.13 `TreeOverview` struct + `on_key` dispatch
- Content: `src/ui/tree_overview.rs:19-90` (struct fields — `state`, `filter_items`, `filter_cache`, `filter_nav_pos`, `before_filter_state`, `last_switches`, `root_switch`, `root_identifies` — and `on_key` mapping `Action::MoveUp/Down/SelectFocus/…/ChangeRoot/ExpandChildren/ExpandAll/FilterNextMatch/PrevMatch/Reset` to tree-state ops).
- Cost: 559 tokens (helper: `count-tokens.py src/ui/tree_overview.rs:19-90`)
- Predecessor: 2.10

### 3.14 `docs/actions.md` action-to-keys table
- Content: `docs/actions.md:1-35` (the full table `action | default keys | description` for all 30 actions, including filter-mode overloads for `move_left/right`, `select_focus/first/last`, `close_parent`, and `reset`).
- Cost: 657 tokens (helper: `count-tokens.py docs/actions.md:1-35`)
- Notes: highest-value doc for "what does each keypress do?".

### 4.1 `App::on_key` action dispatch (event-loop core)
- Content: `src/ui/app.rs:393-584` (central function — filter-mode routing through `Filter::on_key`, then per-`Action` arms: `Quit` with popup-dismiss short-circuit, `Switch` enforcing `can_switch_to_data_block`, `ChangeLayout`, `TreeScaleUp/Down` with `Config::MIN_LAYOUT_TREE_SIZE=10`/`MAX_LAYOUT_TREE_SIZE=80`, `Edit` building a tempfile, `CopyName/Value` via `write_clipboard` + footer "copied N B to system clipboard" message, `Filter/FilterKey/FilterValue` toggling `FilterTarget`, `FilterSwitchIgnoreCase`, `ShowHelp` popup; fallback delegation to `tree_overview.on_key` / `data_block.on_key` / `popup.on_key`).
- Cost: 1205 tokens (helper: `count-tokens.py src/ui/app.rs:393-584`)
- Predecessor: 2.10
- Notes: indispensable for "what happens when I press X?". Max-above = 657; 1205 ≤ 1314 ok.

### 4.2 `Tree::build_item` — recursive tree-item construction
- Content: `src/tree.rs:102-276` (maps each `serde_json::Value` variant to a `TreeItem` + `ItemValue`, computes `"{ N fields }"` / `"[ N items ]"` descriptions, recurses into arrays/objects, invokes `parser.syntax_highlight(data_name, &raw_value)` to populate `tokens` used by the data block).
- Cost: 1098 tokens (helper: `count-tokens.py src/tree.rs:102-276`)
- Predecessor: 3.9

### 4.3 `App::new` + `App::show` main loop
- Content: `src/ui/app.rs:91-163` (constants `HEADER_HEIGHT=1`, `FOOTER_HEIGHT=1`, `FILTER_HEIGHT=3`, `POLL_EVENT_DURATION=100ms`, `HELP_URL`; constructor wiring; `show` loop delegating to `refresh` vs. `refresh_with_fw` depending on `FileWatcher` presence).
- Cost: 514 tokens (helper: `count-tokens.py src/ui/app.rs:91-163`)
- Predecessor: 2.10

### 4.4 `App::build_edit` + `get_copy_text` + `popup_help`
- Content: `src/ui/app.rs:675-767` (scalar vs. complex value extraction, `txt` vs parser-extension selection; clipboard pure-text; help popup from `serde_json::to_value(&keys)` — relies on `preserve_order` for stable action ordering).
- Cost: 683 tokens (helper: `count-tokens.py src/ui/app.rs:675-767`)
- Predecessor: 4.3

### 4.5 `App` mouse + focus handlers
- Content: `src/ui/app.rs:586-673` (click-to-focus routing for tree/data/filter areas, scroll routing to focused widget, `FocusGained`/`FocusLost` saving and restoring `last_focus`, `get_row_inside` helper, `can_switch_to_data_block` guard).
- Cost: 550 tokens (helper: `count-tokens.py src/ui/app.rs:586-673`)
- Predecessor: 4.3

### 4.6 `TreeOverview` root-switch + reset + expand
- Content: `src/ui/tree_overview.rs:92-238` (`change_root` pushing current tree/state onto `last_switches`/`root_switch`, rebuilding from selected value; `reset` popping back; `close_parent`/`select_parent`/`expand_children`/`expand_all` with recursive helpers).
- Cost: 1038 tokens (helper: `count-tokens.py src/ui/tree_overview.rs:92-238`)
- Predecessor: 3.13

### 4.7 `TreeOverview::filter` + `filter_item` + `nav_filter_match`
- Content: `src/ui/tree_overview.rs:335-480` (walks tree items, calls `build_highlighted_text`, tests `opts.filter(item_value)`, `open_parent` expands ancestors of matches, `exclude_mode = false` keeps unmatched visible with only matches styled; `nav_filter_match` jumps forward/backward through `self.tree().identifies`).
- Cost: 897 tokens (helper: `count-tokens.py src/ui/tree_overview.rs:335-480`)
- Predecessor: 4.6

### 4.8 `TreeOverview::draw` + filter lifecycle
- Content: `src/ui/tree_overview.rs:240-333` (scrollbar + `TreeWidget` with `disable_selected_highlight` → `highlight_symbol("→ ")` vs. `highlight_style`; custom `selected_symbol` override; "Tree Overview [pos/count]" title with `MAX_FILTER_COUNT_DISPLAY=9999`; `begin_filter`/`end_filter` saving/restoring `TreeState`).
- Cost: 476 tokens (helper: `count-tokens.py src/ui/tree_overview.rs:240-333`)
- Predecessor: 3.13

### 4.9 `JsonParser` impl + `json::highlight`
- Content: `src/parse/json.rs:6-82` (both `impl Parser for JsonParser` with `allow_array_root = true`, and the free `highlight(value, indent, has_next)` emitting `Indent`/`Break`/`Symbol(",")`/`Symbol(": ")` for each `Value` variant).
- Cost: 587 tokens (helper: `count-tokens.py src/parse/json.rs:6-82`)
- Predecessor: 2.8
- Notes: simplest parser; TOML/HCL reuse `json_highlight` as a fallback for simple arrays.

### 4.10 `YamlParser` impl + multi-doc handling
- Content: `src/parse/yaml.rs:8-70` (deserializer loop collecting multiple docs into `Value::Array`, "complex array → multi-doc, flat array → inline" heuristic, `"---"` separators; `allow_array_root = true`).
- Cost: 401 tokens (helper: `count-tokens.py src/parse/yaml.rs:8-70`)
- Predecessor: 2.8

### 4.11 `yaml::highlight` body
- Content: `src/parse/yaml.rs:72-153` (block-style rendering: object fields with trailing `":"`, `"- "` array prefixes with `from_arr` indentation, `|` multiline branch using `StringValue::MultiLines`, `{}` / `[]` empty placeholders).
- Cost: 581 tokens (helper: `count-tokens.py src/parse/yaml.rs:72-153`)
- Predecessor: 4.10

### 4.12 `TomlParser` impl + `toml_value_to_json`
- Content: `src/parse/toml.rs:9-86` (`syntax_highlight` entry carries `section` name through and trims leading `Break`; `allow_array_root = false`; `toml_value_to_json` converting `TomlValue::{String,Integer,Float,Boolean,Datetime,Array,Table}` → `serde_json::Value`).
- Cost: 553 tokens (helper: `count-tokens.py src/parse/toml.rs:9-86`)
- Predecessor: 2.8

### 4.13 `XmlParser::parse` impl + teaser
- Content: `src/parse/xml.rs:12-36` (`impl Parser for XmlParser` with `allow_array_root = false`, `expand_empty_elements = true`; body delegates to top-level `read(reader, 0)`) then `…` marker. Full parse mechanics — `NodeValues` helper collecting `@attr`-prefixed attributes, `#text`, `#cdata`, duplicate-key → array merging, `read` recursive loop handling `Start`/`Text`/`CData`/`GeneralRef`/`End`/`Eof` — live at `src/parse/xml.rs:37-256` (~1400 more tokens).
- Cost: 136 tokens (helper: `count-tokens.py src/parse/xml.rs:12-36`)
- Predecessor: 2.8
- Notes: elision marker is the catastrophic-omission guard — XML's `@attr`/`#text`/`#cdata` conventions are quirky enough the agent must know they exist even when we can't afford the body.

### 4.14 `HclParser` impl + teaser
- Content: `src/parse/hcl.rs:8-26` (`impl Parser for HclParser` — `extension = "hcl"`, `allow_array_root = false`, `parse` via `hcl::from_str`, `syntax_highlight` delegating to `highlight(name, value, 0)`) then `…` marker. The `highlight` body at `hcl.rs:28-105` (~800 tokens) emits `name = value` for simple fields and `name { … }` blocks for complex, falling back to `json_highlight` for arrays.
- Cost: 117 tokens (helper: `count-tokens.py src/parse/hcl.rs:8-26`)
- Predecessor: 2.8

### 4.15 `JsonlParser` impl
- Content: `src/parse/jsonl.rs:7-38` (line-by-line `serde_json::from_str` with empty-line-skip, always yielding `Value::Array`; reuses `json::highlight`; declares `extension = "json"` so editor-open temp file is `.json`).
- Cost: 230 tokens (helper: `count-tokens.py src/parse/jsonl.rs:7-38`)
- Predecessor: 2.8

### 4.16 `AnyParser` (content-type auto-detection)
- Content: `src/parse/any.rs` whole file (`RefCell<Option<Box<dyn Parser>>>` holding the detected parser; `parse_root` iterating `ContentType::iter()` skipping `Any` itself and installing the first parser whose `parse_root` succeeds — implements `echo '{"key":"value"}' | otree`).
- Cost: 365 tokens (helper: `count-tokens.py src/parse/any.rs`)
- Predecessor: 2.8

### 4.17 `SyntaxToken::render`
- Content: `src/parse/syntax.rs:29-108` (turns `&[SyntaxToken]` into a ratatui `Text`, applies `cfg.colors.data.*` styles per variant, handles `Break`/`Indent`, branches into `TextWrapper::add_text_with_wrap` when `cfg.data.wrap`; returns `(Text, rows, max_width)` consumed by `DataBlock::update_scroll`).
- Cost: 633 tokens (helper: `count-tokens.py src/parse/syntax.rs:29-108`)
- Predecessor: 1.11

### 4.18 `SyntaxToken::pure_text` + quoting + complexity helpers
- Content: `src/parse/syntax.rs:110-149` and `:201-214` (text extractor used by `--to` and `copy_value`; `^[a-zA-Z0-9_-]+$` regex that decides whether field names need quoting; recursive complexity test driving TOML/HCL section promotion).
- Cost: 310 tokens (helper: `count-tokens.py src/parse/syntax.rs:110-149 src/parse/syntax.rs:201-214`)
- Predecessor: 1.11

### 4.19 `StringValue` (multiline + numeric-looking quoting)
- Content: `src/parse/syntax.rs:151-199` (`String`/`MultiLines` enum rules: empty → `"\"\""`, contains `\n` with non-whitespace → `MultiLines`, looks numeric/boolean → must quote, matches standard field regex → bare, else quote).
- Cost: 387 tokens (helper: `count-tokens.py src/parse/syntax.rs:151-199`)
- Predecessor: 1.11

### 4.20 `DataBlock` struct + entrypoints
- Content: `src/ui/data_block.rs:16-68` (struct with `can_vertical_scroll`/`can_horizontal_scroll`, `identify` tracking scroll-reset on selection change, scroll-state bookkeeping; `on_key` and `on_scroll` mapping `Action::Move*/SelectFirst/SelectLast` and `ScrollDirection` to scroll helpers).
- Cost: 335 tokens (helper: `count-tokens.py src/ui/data_block.rs:16-68`)

### 4.21 `DataBlock::draw` + `update_scroll`
- Content: `src/ui/data_block.rs:147-254` (calls `item.render(cfg, area.width)` → `(text, rows, cols)`, installs vertical/horizontal scrollbars when content exceeds area; disables horizontal scrolling when `cfg.data.wrap` on; `SCROLL_RETAIN = 5`).
- Cost: 603 tokens (helper: `count-tokens.py src/ui/data_block.rs:147-254`)
- Predecessor: 4.20

### 4.22 `FilterOptions::filter` + `contains`
- Content: `src/ui/filter.rs:152-187` (match predicate: `Key` checks `item.name`, `Value` stringifies `Value::String`/`Number`, `All` ORs key and value; empty-text never matches; ignore-case via `to_lowercase`).
- Cost: 217 tokens (helper: `count-tokens.py src/ui/filter.rs:152-187`)
- Predecessor: 2.3

### 4.23 `Header::new` + `Header::draw`
- Content: `src/ui/header.rs:41-61` (centered `Paragraph` with `cfg.colors.header.style`, rendering pre-formatted string from `HeaderContext`).
- Cost: 142 tokens (helper: `count-tokens.py src/ui/header.rs:41-61`)
- Predecessor: 2.14

### 5.1 `FileWatcher` struct + public API
- Content: `src/live_reload.rs:16-91` (`new` spawning `watch_file` thread via `notify::recommended_watcher`, `get_err`/`parse_tree` polled by `App::refresh_with_fw`; `max_data_size` enforced on each reload).
- Cost: 441 tokens (helper: `count-tokens.py src/live_reload.rs:16-91`)

### 5.2 `clipboard.rs` OS-specific command selection
- Content: `src/clipboard.rs` whole file (macOS=`pbcopy`, Linux=`wl-copy` when `WAYLAND_DISPLAY` else `xclip -selection clipboard`, Windows=`clip`; stdin-pipe invocation with `NotFound` → install-it error; exit-status check).
- Cost: 418 tokens (helper: `count-tokens.py src/clipboard.rs`)

### 5.3 Config sub-struct shapes (all sections)
- Content: `src/config/mod.rs:52-126` (`Tree { disable_selected_highlight, selected_symbol }`, `Editor { program, args, dir }`, `Layout { direction, tree_size }`, `LayoutDirection { Vertical, Horizontal }`, `Header { disable, format }`, `Footer { disable }`, `Filter { disable, ignore_case, exclude_mode }`, `Data { disable_highlight, max_data_size, wrap }`).
- Cost: 431 tokens (helper: `count-tokens.py src/config/mod.rs:52-126`)
- Predecessor: 2.9

### 5.4 `Edit` external-editor setup signatures
- Content: `src/edit.rs:10-42` (`struct Edit { path, data, cmd }`; `Edit::new(cfg, identify, data, extension)` building a command from `cfg.editor.program`+`args`+`dir` with `{file}` → `/tmp/otree_{id}.{ext}`; `Edit::run` calling `run_inner` with error-hold). The body (`run_inner`, `write_file`, `edit_file`, `delete_file`) lives at `edit.rs:44-103`.
- Cost: 218 tokens (helper: `count-tokens.py src/edit.rs:10-42`)

### 5.5 `debug` macro + `set_file`
- Content: `src/debug.rs` whole file (`debug!` macro, `FILE: OnceLock<String>`, `write_logs` append-only; the `FIXME` that log errors are swallowed because stderr is owned by the TUI).
- Cost: 206 tokens (helper: `count-tokens.py src/debug.rs`)

### 5.6 `Config::load` + path discovery
- Content: `src/config/mod.rs:127-195` (`load` reading TOML from `--config` / `OTREE_CONFIG` / `~/.config/otree.toml`; `get_path` env-var then home-dir search with `io::ErrorKind::NotFound` tolerated).
- Cost: 490 tokens (helper: `count-tokens.py src/config/mod.rs:127-195`)
- Predecessor: 2.9

### 5.7 Per-area color field lists (`DataColors`, `TreeColors`, `Colors`)
- Content: `src/config/colors.rs:20-42` (`Colors { header, focus_border, footer, tree, data, popup, filter }`) + `:77-110` (`DataColors { text, border, symbol, name, tag, str, num, null, bool, section }`) + `:161-195` (`TreeColors { border, selected, name, filter_keyword, type_str, type_null, type_bool, type_num, type_arr, type_obj, value }`).
- Cost: 581 tokens (helper: sum of `colors.rs:20-42` (142), `:77-110` (219), `:161-195` (220))
- Predecessor: 2.9

### 5.8 `Color` struct + palette parsing
- Content: `src/config/colors.rs:341-399` (`Color { fg, bg, bold, italic, style }`; `parse` looks up each name in the palette first, then `str.parse::<ratatui::style::Color>()`; bold/italic style).
- Cost: 357 tokens (helper: `count-tokens.py src/config/colors.rs:341-399`)
- Predecessor: 5.7

### 5.9 Catppuccin theme — palette only
- Content: `config/themes/catppuccin.toml:1-32` (the `[palette]` with named colors `base`, `mantle`, `crust`, `text`, `subtext0/1`, `surface0/1/2`, `overlay0/1/2`, `blue`, `lavender`, `sapphire`, `sky`, `teal`, `green`, `yellow`, `peach`, `maroon`, `red`, `mauve`, `pink`, `flamingo`, `rosewater`). The `[colors.*]` overrides (lines 33-73, ~400 more tokens) live in the file.
- Cost: 248 tokens (helper: `count-tokens.py config/themes/catppuccin.toml:1-32`)
- Predecessor: 5.8

### 5.10 `examples/example.json` teaser with elision
- Content: `examples/example.json:1-15` (root object with keys `string`, `number`, `float`, `boolean`, `null`, `array`, `object`; nested `object.nested_object { deep_string, deep_number }`), then `…` — remaining keys `complex_array`, `complex_object` at lines 17-35 live in the file.
- Cost: 138 tokens (helper: `count-tokens.py examples/example.json:1-15`)

### 5.11 Parser test-case folder listing
- Content: `src/parse/test_cases/{hcl,json,jsonl,toml,xml,yaml}/` folder names plus the `{raw, _highlight}` pair convention (one representative pair per format: `object.json`+`object_highlight.json`, `common.yaml`+`common_highlight.yaml`, `basic.toml`+`basic_highlight.toml`, `attrs.xml`+`attrs.json`, `nested.hcl`+`nested.json`, `complex_objects.jsonl`+`complex_objects_highlight.json`).
- Cost: 90 tokens (helper: `--stdin` on 6 folder names + 12 filenames)

### 5.12 `typos.toml` + `.gitignore`
- Content: `typos.toml` (allowlist: `ratatui`, `donot → don't`, `caf`) and `.gitignore` (`target/`, `debug/`, `*.rs.bk`, `*.pdb`).
- Cost: 75 tokens (helper: sum of `count-tokens.py` on each)

## Below-the-fold

- **`src/ui/app.rs:221-383`** — `App::draw` + `refresh_area` layout (~1150 tokens). The `skip_header`/`skip_footer` fallbacks for tiny terminals and the vertical/horizontal split logic are mechanical given 2.10 (`App` struct) and the `LayoutDirection` enum in 5.3. Fetchable via one `Read` when the agent needs to tweak layout.
- **`src/tree.rs:278-411`** — `Tree::build_item_text` + `highlight_keyword` + `string_tokens` (~900 tokens). Rendering each row's styled spans; 2.7 (types), 2.5 (labels), and 5.7 (Colors list) together already describe *what* is composed, with the file/line range pinned here.
- **`src/parse/xml.rs:37-350`** — `NodeValues` helper + `read` recursive parse loop + `xml::highlight` render (~2200 tokens). The XmlParser impl (4.13) flags the quirky `@attr`/`#text`/`#cdata` conventions and invites the agent in; bodies are reachable via one `Read`.
- **`src/parse/hcl.rs:28-105`** — `HclParser::highlight` body (~800 tokens). The impl (4.14) teaches HCL exists; the body detail is rarely queried.
- **`src/parse/toml.rs:88-193`** — `toml::highlight` body emitting `[section]` / `[[section]]` headers and `'''` multiline strings (~760 tokens). 4.12 already covers `TomlParser`'s intent and `allow_array_root = false`.
- **`src/parse/syntax.rs:216-349`** — `TextWrapper` + `find_split_position` wrap-mode rendering (~950 tokens). 4.17 (`SyntaxToken::render`) names `TextWrapper::add_text_with_wrap` so the agent knows where to look.
- **`src/ui/data_block.rs:69-145`** — `DataBlock` scroll helpers (`scroll_first`, `scroll_last`, `scroll_down`, `scroll_up`, `scroll_right`, `scroll_left`, ~500 tokens). Mechanical given the struct fields.
- **`src/ui/filter.rs:40-150`** — `Filter::on_key` + `Filter::draw` (~770 tokens). `FilterAction` outcomes in 2.3 are the public contract; the body wires them to `tui_textarea` cursor moves.
- **`src/ui/footer.rs`** — `Footer::draw` root-crumb compression for narrow terminals (~710 tokens). The `FooterText { Identify, Message, None }` variants (noted in 4.3) are the user-visible contract; the compression algorithm is pure rendering.
- **`src/ui/popup.rs`** — `Popup` struct + `draw` + `centered_rect(50, 50, …)` (~840 tokens). The existence of help/error overlays is covered by 4.4 (`popup_help`) and 4.1 (`Action::ShowHelp`).
- **`src/live_reload.rs:93-141`** — `watch_file` thread body + `Data` struct (~440 tokens). 5.1 conveys the `FileWatcher` public contract; the thread loop is plumbing.
- **`src/edit.rs:44-103`** — `Edit::run_inner`, `write_file`, `edit_file`, `delete_file` (~450 tokens). 5.4 gives the signature surface; the disk dance is predictable.
- **`src/config/mod.rs:197-229`** + `:230-330`** — `Config::parse` + `validate_palette` + per-section default impls (~740 tokens). Defaults are already echoed in `config/default.toml` (3.7); validation rules (tree_size 10..80) show up in 4.1's scale handlers.
- **`src/config/keys.rs:74-179` + `:350-372`** — `Key::from_event`, `Key::parse` grammar, `Keys::get_key_action` (~910 tokens). 2.1 (Key enum), 2.13 (default bindings) and 3.14 (actions table) cover user-facing semantics.
- **`src/config/keys.rs:194-282`** — `Keys` struct serde-field list (~660 tokens). The exact TOML shape `[keys]` accepts is already recoverable from 2.13 (the `generate_keys_default!` macro invocation), 3.7 (canonical config), and the `Action` variants in 2.12.
- **`src/config/colors.rs:250-339`** — `FooterColors`, `PopupColors`, `FilterColors` (~400 tokens). Naming only; pattern clear from 5.7.
- **`src/config/colors.rs:77-110, 161-195` Color defaults** — per-struct `fn default()` methods (~500 tokens): canonical baseline colors; `config/default.toml`'s `[colors.*]` sections (not ranked) mirror these verbatim. Reachable in one `Read`.
- **`config/themes/catppuccin.toml:33-73`** — the `[colors.*]` overrides referencing palette names (~400 tokens): derivable from 5.9 palette + 5.7 struct shapes.
- **`config/default.toml:62-110`** — `[colors.*]` + `[types]` sections (~400 tokens): exact-match of the Rust defaults in `config/colors.rs`/`types.rs`; low marginal information for an agent that has 5.7 + 2.5.
- **README install section (`README.md:6-31`, ~170 tokens)** — `cargo install --git`, `paru -S otree`, `brew install otree`. Install instructions rarely help the agent act; the README tagline (1.10) and usage (2.2) carry the weight.
- **`.github/` workflows (~270 tokens)** — `cargo check` CI, musl+darwin+windows release matrix via `taiki-e/upload-rust-binary-action`, `crate-ci/typos`, weekly dependabot grouping. Relevant only for CI/release questions; fetchable in one `Read` per file.
- **`build.rs`** (~720 tokens): emits `OTREE_VERSION`, `OTREE_SHA`, `OTREE_BUILD_TYPE`, `OTREE_TARGET` env vars referenced from `cmd.rs` via `env!`. Only relevant when the agent is asked about `otree --version` or CI-release mechanics; the env-var names are greppable.
- **Parser test-case data files** (`src/parse/test_cases/**/*`, ~3000 tokens): naming convention captured in 5.11; content reachable in one `Read` per fixture.
- **Full body of `examples/example.*`** beyond 5.10 (~1500 tokens): redundant for "what shape does otree accept?".
- **`Cargo.lock`** (~44k tokens): transitive versions never help the agent; direct deps are in 2.11.
- **`LICENSE`**: MIT — adds nothing behavioral.
- **`assets/screenshot.png`**: binary; cannot contribute text.
- **`#[cfg(test)] mod test { … }` blocks inside `src/parse/*.rs`** (~1200 tokens): round-trip tests driven by the test_cases folders; behavior covered by parser bodies, names derivable from 5.11.
- **Macro source for `generate_keys_default!` / `generate_actions!` / `generate_colors_parse!` / `generate_types_default!`** (~400 tokens): the *expansions* (2.12, 2.13, 5.7, 2.5) are what agent queries land on; the macro source is one `Read` away.
- **`docs/colors.md`**: empty file (1 line) in this revision.
- **`Cargo.toml:38-48`** — release profile + clippy lints (~85 tokens): tuning details.
- **Getter and delegation methods** across widgets (`TreeOverview::{get_selected, get_value, get_parser, state, state_mut, tree, on_click, on_scroll}`, `Popup::{is_disabled, set_data, on_scroll}`, `DataBlock::{reset, reset_scroll_*, on_scroll}`, `Filter::{get_text, set_target, switch_ignore_case, get_options}`): structural roles are implied by the ranked struct-field lists.
