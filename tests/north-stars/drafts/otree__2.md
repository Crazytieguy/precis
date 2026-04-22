# otree — North Star

Revision pin: `a02bdf44`

`otree` is a Rust TUI that renders JSON/YAML/TOML/XML/HCL/JSONL files as an interactive tree. It runs on `ratatui` + `tui-tree-widget`, parses any supported schema into a `serde_json::Value`, and drives a split-pane UI (tree overview + data block) with configurable keys/colors, filter mode, popup help, live reload, clipboard copy, and an "open in editor" action. Entry point: `src/main.rs` → `cmd::CommandArgs::parse` → `ui::start(App)`.

## Batches

### 1.1 Module tree of `src/`
- Content: rendered listing of `src/` plus `src/config/`, `src/parse/`, `src/ui/`, naming every `.rs` file (`main`, `cmd`, `tree`, `debug`, `clipboard`, `edit`, `live_reload`; `config/{colors,keys,mod,types}`; `parse/{any,hcl,json,jsonl,mod,syntax,toml,xml,yaml}`; `ui/{app,data_block,filter,footer,header,mod,popup,tree_overview}`).
- Cost: 118 tokens (helper: `printf "<layout above>" | count-tokens.py --stdin`)
- Notes: Catastrophic if missing — without it the agent cannot know `edit.rs`, `live_reload.rs`, `clipboard.rs`, `debug.rs`, the 9-file `parse/` plugin set, or the 8 ui widgets exist.

### 1.2 Repo-root listing
- Content: `.github/  assets/  config/  docs/  examples/  src/  build.rs  Cargo.toml  Cargo.lock  README.md  LICENSE  typos.toml  .gitignore`.
- Cost: 30 tokens (helper: `ls tests/fixtures/otree/ | count-tokens.py --stdin`)

### 1.3 `src/main.rs` module declarations
- Content: `src/main.rs:1-9`.
- Cost: 28 tokens (helper: `count-tokens.py src/main.rs:1-9`)

### 1.4 `parse::ContentType` enum + module header
- Content: `src/parse/mod.rs:1-34` — module re-exports and `ContentType` enum (`Json/Yaml/Toml/Xml/Hcl/Jsonl/Any`).
- Cost: 193 tokens (helper: `count-tokens.py src/parse/mod.rs:1-34`)

### 1.5 `parse::Parser` trait
- Content: `src/parse/mod.rs:36-64` — `fn extension`, `allow_array_root`, `parse`, `syntax_highlight`, and the default `parse_root` (validates array-root allowance + root-type).
- Cost: 221 tokens (helper: `count-tokens.py src/parse/mod.rs:36-64`)

### 1.6 `README.md` tagline, install, usage
- Content: `README.md:1-50`.
- Cost: 354 tokens (helper: `count-tokens.py README.md:1-50`)
- Notes: Answers "what is otree?" and "how do I run it?" in one shot.

### 1.7 `Cargo.toml` package metadata + dep list
- Content: `Cargo.toml` (whole file).
- Cost: 407 tokens (helper: `count-tokens.py Cargo.toml`)
- Notes: Version 0.6.4; full dep set (`anyhow`, `clap`, `crossterm`, `ratatui`, `hcl-rs`, `notify`, `quick-xml`, `serde_json+preserve_order`, `serde_yml`, `toml+preserve_order`, `tui-textarea`, `tui-tree-widget`, `strum`, `paste`, `regex`, `humansize`, `dirs`, `console`).

## 2. Second tier — CLI surface, core types, UI skeleton, action inventory

### 2.1 `CommandArgs` struct (all CLI flags)
- Content: `src/cmd.rs:11-109` — the `#[derive(Parser)]` struct: `path`; `--config`; `-t/--content-type`; `-o/--to`; `--disable-header/footer/filter`; `--tree-disable-selected-highlight`; `--tree-selected-symbol`; `--filter-ignore-case`; `--filter-exclude-mode`; `-f/--header-format`; `-V/--vertical`, `-H/--horizontal`; `-s/--size`; `--disable-highlight`; `-w/--wrap`; `--show-config`; `--ignore-config`; `--build-info`; `--max-data-size`; `-R/--live-reload`; `--debug <file>`; `-v/--version`.
- Cost: 644 tokens (helper: `count-tokens.py src/cmd.rs:11-109`)

### 2.2 `Config` top-level struct
- Content: `src/config/mod.rs:1-50`.
- Cost: 272 tokens (helper: `count-tokens.py src/config/mod.rs:1-50`)
- Notes: Ten sections — `tree`, `editor`, `data`, `layout`, `header`, `footer`, `filter`, `palette`, `colors`, `types`, `keys`.

### 2.3 `config/default.toml` — `[keys]` block
- Content: `config/default.toml:30-60` — every default key binding in one concise TOML block.
- Cost: 228 tokens (helper: `count-tokens.py config/default.toml:30-60`)

### 2.4 `tree.rs` — `Tree`, `ItemValue`, `HighlightKeyword`, `FieldType`
- Content: `src/tree.rs:1-50`.
- Cost: 248 tokens (helper: `count-tokens.py src/tree.rs:1-50`)
- Notes: Central data model. `Tree { parser, items: Vec<TreeItem<'static, String>>, values: HashMap<String, ItemValue>, identifies, cfg }`; `ItemValue { name, value, field_type, description, tokens }`; `FieldType { Null/Num/Bool/Str/Obj/Arr }`.

### 2.5 `SyntaxToken` enum
- Content: `src/parse/syntax.rs:1-28`.
- Cost: 111 tokens (helper: `count-tokens.py src/parse/syntax.rs:1-28`)
- Notes: IR every parser emits: `Symbol`, `Name`, `Tag`, `String`, `Number`, `Null`, `Bool`, `Section`, `Break`, `Indent`.

### 2.6 `ContentType::new_parser` dispatch
- Content: `src/parse/mod.rs:66-78`.
- Cost: 131 tokens (helper: `count-tokens.py src/parse/mod.rs:66-78`)

### 2.7 `ui/mod.rs` module header + `get_border_style`
- Content: `src/ui/mod.rs:1-33`.
- Cost: 197 tokens (helper: `count-tokens.py src/ui/mod.rs:1-33`)

### 2.8 `ui::start` terminal bootstrap
- Content: `src/ui/mod.rs:35-87`.
- Cost: 371 tokens (helper: `count-tokens.py src/ui/mod.rs:35-87`)
- Notes: Main loop handles `ShowResult::Edit/Quit` and raw-mode + mouse-capture setup/teardown.

### 2.9 `Action` enum
- Content: `src/config/keys.rs:317-348` — the `generate_actions!` invocation, single source of truth for action names.
- Cost: 216 tokens (helper: `count-tokens.py src/config/keys.rs:317-348`)

### 2.10 `Keys` default-bindings macro invocation
- Content: `src/config/keys.rs:284-315`.
- Cost: 270 tokens (helper: `count-tokens.py src/config/keys.rs:284-315`)
- Predecessor: 2.9

### 2.11 `run()` in `main.rs`
- Content: `src/main.rs:27-109`.
- Cost: 624 tokens (helper: `count-tokens.py src/main.rs:27-109`)
- Notes: Program-wide flow — load config, parse args, `args.update_config`, read file/stdin, `--to` conversion path, enforce `max_data_size`, `Tree::parse`, build `App`, `ui::start`.

### 2.12 `App` struct + associated types/consts
- Content: `src/ui/app.rs:27-163` (includes `Refresh`/`ElementInFocus`/`ScrollDirection` enums, `App` struct fields, `ShowResult`, `new`).
- Cost: 795 tokens (helper: `count-tokens.py src/ui/app.rs:27-163`)
- Notes: Blueprint for the whole UI state machine.

### 2.13 `ItemValue` impl (`plain_text`, `render`, `build_highlighted_text`)
- Content: `src/tree.rs:385-411`.
- Cost: 155 tokens (helper: `count-tokens.py src/tree.rs:385-411`)
- Predecessor: 2.4

### 2.14 `docs/actions.md` action↔keys↔description table
- Content: `docs/actions.md:1-34`.
- Cost: 657 tokens (helper: `count-tokens.py docs/actions.md:1-34`)
- Notes: Authoritative reference listing every action, its default keystroke(s), and filter-mode-specific behavior.

### 2.15 `docs/changelog.md` most-recent releases (v0.6.x)
- Content: `docs/changelog.md:1-56`.
- Cost: 515 tokens (helper: `count-tokens.py docs/changelog.md:1-56`)

## 3. Third tier — control-flow internals, config details, core parser

### 3.1 `App::show` + `refresh`/`refresh_with_fw`/`reload_tree`
- Content: `src/ui/app.rs:137-220`.
- Cost: 660 tokens (helper: `count-tokens.py src/ui/app.rs:137-220`)
- Predecessor: 2.12

### 3.2 `App::on_key` — action dispatch
- Content: `src/ui/app.rs:393-584`.
- Cost: 1205 tokens (helper: `count-tokens.py src/ui/app.rs:393-584`)
- Predecessor: 2.12
- Notes: Where every keybinding takes effect. Largest function in the repo — splitting mid-match would be incoherent.

### 3.3 `TreeOverview` struct
- Content: `src/ui/tree_overview.rs:1-49`.
- Cost: 370 tokens (helper: `count-tokens.py src/ui/tree_overview.rs:1-49`)

### 3.4 `TreeOverview::on_key` + accessors
- Content: `src/ui/tree_overview.rs:51-90`.
- Cost: 331 tokens (helper: `count-tokens.py src/ui/tree_overview.rs:51-90`)
- Predecessor: 3.3

### 3.5 `TreeOverview::change_root` / `reset` (root-stack behavior)
- Content: `src/ui/tree_overview.rs:92-148`.
- Cost: 348 tokens (helper: `count-tokens.py src/ui/tree_overview.rs:92-148`)
- Predecessor: 3.4
- Notes: `r` pushes current item as new root; `<esc>` pops back. `root_identifies` is the footer breadcrumb.

### 3.6 `TreeOverview::filter` + `filter_item`
- Content: `src/ui/tree_overview.rs:335-408`.
- Cost: 473 tokens (helper: `count-tokens.py src/ui/tree_overview.rs:335-408`)
- Predecessor: 3.3

### 3.7 `ui::filter` types (`Filter`, `FilterTarget`, `FilterAction`, `FilterOptions`, `Filter::new`)
- Content: `src/ui/filter.rs:1-50`.
- Cost: 244 tokens (helper: `count-tokens.py src/ui/filter.rs:1-50`)

### 3.8 `FilterOptions::filter` / `contains`
- Content: `src/ui/filter.rs:152-187`.
- Cost: 217 tokens (helper: `count-tokens.py src/ui/filter.rs:152-187`)
- Predecessor: 3.7
- Notes: Three targets (Key/Value/All); `All` matches either name or stringified value.

### 3.9 `DataBlock` struct + key/scroll dispatch
- Content: `src/ui/data_block.rs:1-67`.
- Cost: 441 tokens (helper: `count-tokens.py src/ui/data_block.rs:1-67`)
- Notes: Independent vertical/horizontal `ScrollbarState` with retain margin of 5 rows.

### 3.10 `Popup` struct + `on_key`/`on_scroll`/`disable`
- Content: `src/ui/popup.rs:1-45`.
- Cost: 264 tokens (helper: `count-tokens.py src/ui/popup.rs:1-45`)

### 3.11 `Footer` / `FooterText`
- Content: `src/ui/footer.rs:1-25`.
- Cost: 138 tokens (helper: `count-tokens.py src/ui/footer.rs:1-25`)

### 3.12 `Header` / `HeaderContext`
- Content: `src/ui/header.rs:1-40`.
- Cost: 268 tokens (helper: `count-tokens.py src/ui/header.rs:1-40`)
- Notes: Default header format `{version} - {data_source} ({content_type}) - {data_size}`.

### 3.13 `Config` sub-structs
- Content: `src/config/mod.rs:52-125` — `Tree`, `Editor`, `Layout`/`LayoutDirection`, `Header`, `Footer`, `Filter`, `Data`, with all `#[serde(default)]` fields.
- Cost: 431 tokens (helper: `count-tokens.py src/config/mod.rs:52-126`)
- Predecessor: 2.2

### 3.14 `Config::load` / `parse` / `get_path` / constants
- Content: `src/config/mod.rs:127-230`.
- Cost: 678 tokens (helper: `count-tokens.py src/config/mod.rs:127-230`)
- Predecessor: 2.2
- Notes: Config from `$OTREE_CONFIG` or `~/.config/otree.toml`. `MIN_LAYOUT_TREE_SIZE = 10`, `MAX_LAYOUT_TREE_SIZE = 80`, default max data size 30 MiB.

### 3.15 `Keys` struct (every action field)
- Content: `src/config/keys.rs:194-282`.
- Cost: 658 tokens (helper: `count-tokens.py src/config/keys.rs:194-282`)
- Predecessor: 2.9

### 3.16 `Key` enum + `Key::from_event`
- Content: `src/config/keys.rs:53-72`.
- Cost: 68 tokens (helper: `count-tokens.py src/config/keys.rs:53-72`)
- Notes: Just the enum. The `Key::parse` grammar body (`<ctrl-x>`/`<alt-x>`/`<fN>`/`<esc>`/…) is below-the-fold — one `Read` of `src/config/keys.rs:74-192` away.

### 3.17 `Colors` top-level struct + parse macro
- Content: `src/config/colors.rs:1-76`.
- Cost: 425 tokens (helper: `count-tokens.py src/config/colors.rs:1-76`)

### 3.18 `Color` struct + `Color::parse`
- Content: `src/config/colors.rs:341-399`.
- Cost: 357 tokens (helper: `count-tokens.py src/config/colors.rs:341-399`)
- Notes: `{ fg, bg, bold, italic, style }` — fg/bg strings parse as ratatui colors with palette-name indirection.

### 3.19 `Types` (tree type-label overrides)
- Content: `src/config/types.rs` (whole, 48 lines).
- Cost: 261 tokens (helper: `count-tokens.py src/config/types.rs`)

### 3.20 `Tree::parse` / `from_value` (root expansion)
- Content: `src/tree.rs:51-92`.
- Cost: 329 tokens (helper: `count-tokens.py src/tree.rs:51-92`)
- Predecessor: 2.4

### 3.21 JSON parser full impl (shared `highlight` fn)
- Content: `src/parse/json.rs:1-82`.
- Cost: 608 tokens (helper: `count-tokens.py src/parse/json.rs:1-82`)
- Notes: `JsonParser` (`extension="json"`, `allow_array_root=true`) + `pub fn highlight(value, indent, has_next)` which TOML/HCL/JSONL all call.

### 3.22 `AnyParser` auto-detection
- Content: `src/parse/any.rs` (whole, 61 lines).
- Cost: 365 tokens (helper: `count-tokens.py src/parse/any.rs`)
- Notes: Iterates `ContentType::iter()`, tries each parser, stores the first that succeeds. Powers stdin/unknown-extension flows.

## 4. Fourth tier — non-JSON parsers and user-feature modules

### 4.1 YAML parser — `Parser` impl + dispatch
- Content: `src/parse/yaml.rs:1-70`.
- Cost: 440 tokens (helper: `count-tokens.py src/parse/yaml.rs:1-70`)
- Notes: Multi-document when root array contains complex values; otherwise renders as a normal array.

### 4.2 YAML `highlight` body
- Content: `src/parse/yaml.rs:72-153`.
- Cost: 581 tokens (helper: `count-tokens.py src/parse/yaml.rs:72-153`)
- Predecessor: 4.1
- Notes: Uses `StringValue` (shared with TOML) for quoted/multi-line string emission.

### 4.3 TOML parser — `Parser` impl + `toml_value_to_json`
- Content: `src/parse/toml.rs:1-86`.
- Cost: 609 tokens (helper: `count-tokens.py src/parse/toml.rs:1-86`)
- Notes: TOML has no `null` — emits `""` for `Value::Null`. Always disallows array-root.

### 4.4 TOML `highlight` body (sections, arrays-of-tables)
- Content: `src/parse/toml.rs:88-193`.
- Cost: 763 tokens (helper: `count-tokens.py src/parse/toml.rs:88-193`)
- Predecessor: 4.3
- Notes: Emits `[section]` / `[[section]]` headers; falls back to `json_highlight` for non-complex arrays.

### 4.5 HCL parser (`Parser` impl + `highlight`)
- Content: `src/parse/hcl.rs:1-106`.
- Cost: 730 tokens (helper: `count-tokens.py src/parse/hcl.rs:1-106`)
- Notes: Uses `hcl-rs::from_str`; emits alternating `name = value` and `name { … }` blocks.

### 4.6 JSONL parser
- Content: `src/parse/jsonl.rs:1-38`.
- Cost: 256 tokens (helper: `count-tokens.py src/parse/jsonl.rs:1-38`)
- Notes: Each non-empty line parsed as JSON; result `Value::Array`. `extension()` returns `"json"`.

### 4.7 XML parser — `Parser` impl header
- Content: `src/parse/xml.rs:1-35`.
- Cost: 200 tokens (helper: `count-tokens.py src/parse/xml.rs:1-35`)
- Notes: Body of `read` / `NodeValues` / `highlight` are deferred below-the-fold — file + line pointers preserve the one-hop path.

### 4.8 `Edit` (external editor invocation)
- Content: `src/edit.rs:1-40`.
- Cost: 246 tokens (helper: `count-tokens.py src/edit.rs:1-40`)
- Notes: Writes item text to `{editor.dir}/otree_{id}.{ext}` (default `/tmp`), runs `cfg.editor.program` (default `$EDITOR` or `vim`), cleans up afterward — read-only in spirit.

### 4.9 `FileWatcher` (live-reload) struct + `new`/`start`
- Content: `src/live_reload.rs:1-52`.
- Cost: 282 tokens (helper: `count-tokens.py src/live_reload.rs:1-52`)

### 4.10 `watch_file` thread body (notify backend)
- Content: `src/live_reload.rs:117-141`.
- Cost: 198 tokens (helper: `count-tokens.py src/live_reload.rs:117-141`)
- Predecessor: 4.9

### 4.11 Clipboard platform dispatch
- Content: `src/clipboard.rs:1-26`.
- Cost: 181 tokens (helper: `count-tokens.py src/clipboard.rs:1-26`)
- Notes: `pbcopy` on macOS, `wl-copy`/`xclip -selection clipboard` on Linux, `clip` on Windows.

### 4.12 `debug!` macro + file logging
- Content: `src/debug.rs` (whole, 37 lines).
- Cost: 206 tokens (helper: `count-tokens.py src/debug.rs`)
- Notes: `debug!(…)` is a no-op unless `--debug <file>` has been set.

### 4.13 `App::build_edit` + `get_copy_text`
- Content: `src/ui/app.rs:675-713`.
- Cost: 307 tokens (helper: `count-tokens.py src/ui/app.rs:675-713`)
- Predecessor: 2.12
- Notes: "edit" uses `.txt` for simple scalars and the parser's extension for complex values; "copy name" emits `item.name` while "copy value" emits plain-text rendering.

### 4.14 `parse::syntax` string/quote helpers
- Content: `src/parse/syntax.rs:136-214` — `STANDARD_FIELD_NAME_RE`, `quote_field_name`, `StringValue`, `is_value_complex`.
- Cost: 590 tokens (helper: `count-tokens.py src/parse/syntax.rs:136-214`)
- Predecessor: 2.5

## 5. Fifth tier — lower-value details, small docs, misc config

### 5.1 `docs/actions.md` key-syntax reference
- Content: `docs/actions.md:35-51` — "All available keys" section (grammar for `<up>`, `<ctrl-x>`, `<alt-x>`, `<fN>`, `<esc>`, etc.).
- Cost: 179 tokens (helper: `count-tokens.py docs/actions.md:35-51`)

### 5.2 `config/default.toml` — `[colors.*]` block
- Content: `config/default.toml:62-109` — every default color for header/tree/data/footer/popup/filter sections.
- Cost: 399 tokens (helper: `count-tokens.py config/default.toml:62-109`)

### 5.3 `config/default.toml` top sections
- Content: `config/default.toml:1-29` — `[tree]`/`[editor]`/`[layout]`/`[header]`/`[footer]`/`[data]`/`[filter]`.
- Cost: 113 tokens (helper: `count-tokens.py config/default.toml:1-29`)

### 5.4 `CommandArgs::get_content_type` (extension→`ContentType`)
- Content: `src/cmd.rs:202-228`.
- Cost: 192 tokens (helper: `count-tokens.py src/cmd.rs:202-228`)
- Notes: `.json`→`Json`, `.yaml`/`.yml`→`Yaml`, `.toml`→`Toml`, `.xml`→`Xml`, `.hcl`→`Hcl`, `.jsonl`→`Jsonl`, else `Any`.

## Below-the-fold

Fixture content deliberately not ranked (with one-line justification each):

- `src/ui/app.rs:393-584` is already in 3.2; the surrounding file body beyond batches 2.12/3.1/3.2/4.13 is heavy and below-value-per-token: `App::draw` + `refresh_area` (lines 221-383, 1153 tokens), `App::on_click/on_scroll/on_focus_changed` (586-674, 618 tokens), `App::popup_help` (715-767, 376 tokens). Each file-and-line pointer is preserved here so any deeper dive is one `Read` away.
- `src/ui/tree_overview.rs:149-334` — `TreeOverview::draw` + title + `begin_filter`/`end_filter` (~1280 tokens). Rendering plumbing, not semantic.
- `src/ui/tree_overview.rs:409-493` — `nav_filter_match` (494 tokens). `n`/`N` navigation; covered in concept by 3.6.
- `src/ui/filter.rs:52-150` — `Filter::on_key` + `draw` (637 tokens). Detailed key routing inside the `tui-textarea`; the *types* are already shown in 3.7.
- `src/config/keys.rs:74-192` — `Key::parse`/`parse_keys` body (884 tokens). The grammar is described in 5.1 and exemplified by the default bindings in 2.10.
- `src/config/keys.rs:356-372` — `Keys::get_key_action` runtime lookup (104 tokens). Trivial pattern matching.
- `src/config/colors.rs:77-340` — `DataColors`/`TreeColors`/`FooterColors`/`PopupColors`/`FilterColors` struct definitions and per-field defaults (~1500 tokens). The *types* are listed in 3.17; defaults redundant with 5.2.
- `src/config/mod.rs:231-330` — `impl Default` blocks for `Tree`/`Editor`/`Layout`/`Header`/`Footer`/`Filter`/`Data` (440 tokens). Each default is already visible in `config/default.toml` (2.3, 5.2, 5.3).
- `src/tree.rs:102-366` — `Tree::build_item` (1098 tokens) and `build_item_text` + `highlight_keyword` (633 tokens). Tree construction recursion; the shape is fully implied by `FieldType` + `SyntaxToken` + root expansion (2.4, 2.5, 3.20).
- `src/parse/syntax.rs:29-349` — `SyntaxToken::render` (807 tokens) and the `TextWrapper` line-wrap engine (955 tokens). Rendering plumbing; not semantic for most queries.
- `src/parse/xml.rs:37-350` — XML `read`/`NodeValues`/`highlight` (~2200 tokens). The parser exists (4.7) and the XML semantic rules are documented in the changelog (2.15, v0.5.2 entry).
- Per-parser `#[cfg(test)] mod test` blocks in `src/parse/{json,yaml,toml,hcl,jsonl,xml}.rs` (~200-400 tokens each) — all share the same shape: `include_str!("test_cases/<format>/<name>.{src,highlight}")` → parse → highlight → assert.
- `src/parse/test_cases/**` — 40+ tiny schema fixtures (JSON/YAML/TOML/HCL/JSONL/XML raw/expected pairs). Fixture data, not semantic code.
- `examples/*.{json,yaml,toml,xml,hcl,jsonl}` — ~5k tokens of sample data. Existence is documented here; contents are fixture data and cheap to `Read` on demand.
- `config/themes/catppuccin.toml` (650 tokens) — example user theme; one-line gist is "realistic palette + color-name override example".
- `docs/changelog.md:57-156` (~655 tokens) — v0.5.x and older. Recent history (2.15) covers the v0.6 feature frontier.
- `README.md:52-86` (566 tokens) — "Roadmap" (checkbox list; every item is marked done) and "Thanks".
- `src/cmd.rs:111-200` — `CommandArgs::parse` (arg-parsing glue) and `update_config` (mechanical flag→config copies). `parse` wraps clap error handling; `update_config` is reversibly implied by the flag list in 2.1 and the config shape in 2.2.
- `src/cmd.rs:230-255` — `show_version` / `show_build_info` (191 tokens). Prints `OTREE_*` envs from `build.rs`.
- `build.rs` (735 tokens) — `vergen`-driven env emission for `OTREE_VERSION`/`OTREE_SHA`/`OTREE_BUILD_TYPE`/`OTREE_TARGET` and git-describe fallback.
- `Cargo.lock` — fully transitive dep lock; zero semantic value beyond `Cargo.toml` (1.7).
- `.github/dependabot.yml`, `.github/workflows/{cargo-check,spell-check,release-binary}.yml` — CI surface; small, discoverable, rarely queried.
- `LICENSE`, `typos.toml`, `assets/screenshot.png`, `.gitignore` — no meaningful developer-query payload.
- `docs/colors.md` — empty file (0 lines); the emptiness is the full content.
