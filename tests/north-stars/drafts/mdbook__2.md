# mdbook — North Star

Revision pin: `b8c90970`

## Batches

### 1.1 README.md (project tagline + license)
- Content: `README.md` (whole, 21 lines)
- Cost: 253 tokens (helper: `count-tokens.py README.md`)
- Notes: One-paragraph statement that mdBook generates online books from Markdown. Highest catastrophic-omission risk reducer for "what is this repo?"

### 1.2 Workspace top-level listing
- Content: rendered listing of repo root: `CHANGELOG.md / CODE_OF_CONDUCT.md / CONTRIBUTING.md / Cargo.lock / Cargo.toml / LICENSE / README.md / ci/ / crates/ / eslint.config.mjs / examples/ / guide/ / package.json / rustfmt.toml / src/ / tests/ / triagebot.toml`
- Cost: 60 tokens (helper: `printf … | count-tokens.py --stdin`)
- Notes: Surfaces the unusual presence of a top-level `src/` (the binary) **alongside** `crates/` (the library workspace), `guide/` (mdBook's own user guide built with itself), `examples/`, JS tooling.

### 1.3 `crates/` listing
- Content: rendered listing: `mdbook-compare/ / mdbook-core/ / mdbook-driver/ / mdbook-html/ / mdbook-markdown/ / mdbook-preprocessor/ / mdbook-renderer/ / mdbook-summary/ / xtask/`
- Cost: 39 tokens (helper: `printf … | count-tokens.py --stdin`)
- Notes: Workspace fragmentation into many small crates is the central architectural fact.

### 1.4 `src/` and `src/cmd/` listing
- Content: rendered listing: `main.rs / cmd/{mod, build, clean, init, serve, test, watch, command_prelude, watch/native, watch/poller}.rs`
- Cost: 52 tokens (helper: `printf … | count-tokens.py --stdin`)
- Notes: One file per CLI subcommand. "Where is `mdbook serve` implemented?" → one hop.

### 1.5 `mdbook-html/src/` recursive listing
- Content: rendered listing: `lib.rs / utils.rs / html/{mod, admonitions, hide_lines, print, serialize, tokenizer, tree, tests}.rs / html_handlebars/{mod, hbs_renderer, search, static_files, helpers/{mod, fontawesome, resources, toc}}.rs / theme/{mod, fonts, playground_editor, searcher}.rs`
- Cost: ≈ 90 tokens (helper: `printf … | count-tokens.py --stdin`)
- Notes: Names every HTML-renderer source file. The HTML renderer is the largest part of the codebase.

### 1.6 `src/main.rs` subcommand dispatch
- Content: `src/main.rs:1-55`
- Cost: 391 tokens (helper: `count-tokens.py src/main.rs:1-55`)
- Notes: The `match … get_matches().subcommand()` block names every subcommand: `init / build / clean / watch / serve / test / completions`, with `#[cfg(feature = "watch"|"serve")]` gating.

### 1.7 `mdbook-driver` crate doc-comment (high-level library entry point)
- Content: `crates/mdbook-driver/src/lib.rs:1-66`
- Cost: 524 tokens (helper: `count-tokens.py crates/mdbook-driver/src/lib.rs:1-66`)
- Notes: Names every sibling crate and its role (`mdbook_preprocessor`, `mdbook_renderer`, `mdbook_markdown`, `mdbook_summary`, `mdbook_html`, `mdbook_core`); shows the canonical usage paths via `MDBook::init()` and `MDBook::load()`. Most load-bearing prose in the repo.

### 1.8 Workspace `Cargo.toml` (workspace.members + dependencies + workspace.package)
- Content: `Cargo.toml:1-71`
- Cost: 668 tokens (helper: `count-tokens.py Cargo.toml:1-71`)
- Notes: Edition 2024, MSRV 1.88.0, license MPL-2.0. Full dependency landscape (anyhow, axum, clap, handlebars, html5ever, pulldown-cmark, serde, tokio, notify, …).

### 2.1 `MDBook` struct fields
- Content: `crates/mdbook-driver/src/mdbook.rs:28-44`
- Cost: 116 tokens (helper: `count-tokens.py crates/mdbook-driver/src/mdbook.rs:28-44`)
- Notes: `pub struct MDBook { root, config, book, renderers, preprocessors }`. Central in-memory representation.

### 2.2 `MDBook::load` (entry point)
- Content: `crates/mdbook-driver/src/mdbook.rs:46-86`
- Cost: 289 tokens (helper: `count-tokens.py crates/mdbook-driver/src/mdbook.rs:46-86`)
- Notes: Reads `book.toml`, applies env-var overrides, calls `load_book`. Primary loading path.
- Predecessor: 2.1

### 2.3 `MDBook::iter` + `init` (signatures)
- Content: `crates/mdbook-driver/src/mdbook.rs:111-159`
- Cost: 378 tokens (helper: `count-tokens.py crates/mdbook-driver/src/mdbook.rs:111-138 crates/mdbook-driver/src/mdbook.rs:140-159`)
- Notes: `iter` walks the book depth-first over `BookItem`s; `init` returns a `BookBuilder`. Body of `build`/`preprocess_book`/`execute_build_process` (4.1) elided with `…` between batches.
- Predecessor: 2.1

### 2.4 `MDBook::with_renderer` + `with_preprocessor` (embedder hooks)
- Content: `crates/mdbook-driver/src/mdbook.rs:213-225`
- Cost: 121 tokens (helper: `count-tokens.py crates/mdbook-driver/src/mdbook.rs:213-225`)
- Notes: Used to register custom renderers/preprocessors at runtime instead of via `book.toml`.
- Predecessor: 2.1

### 2.5 `MDBook::test / test_chapter` (signatures)
- Content: `crates/mdbook-driver/src/mdbook.rs:227-265`
- Cost: 324 tokens (helper: `count-tokens.py crates/mdbook-driver/src/mdbook.rs:227-265`)
- Notes: Surface for `mdbook test`. Body shells out to `rustdoc --test`; deferred.
- Predecessor: 2.1

### 2.6 `MDBook::build_dir_for / source_dir / theme_dir`
- Content: `crates/mdbook-driver/src/mdbook.rs:347-393`
- Cost: 339 tokens (helper: `count-tokens.py crates/mdbook-driver/src/mdbook.rs:347-393`)
- Notes: `build_dir_for` documents the multi-renderer subdirectory layout (`book/html/`, `book/epub/`, etc.).
- Predecessor: 2.1

### 2.7 `BookItem` enum
- Content: `crates/mdbook-core/src/book.rs:113-127`
- Cost: 92 tokens (helper: `count-tokens.py crates/mdbook-core/src/book.rs:113-127`)
- Notes: `Chapter / Separator / PartTitle`. Small but appears across all preprocessor/renderer code.

### 2.8 `Chapter` struct
- Content: `crates/mdbook-core/src/book.rs:140-170`
- Cost: 308 tokens (helper: `count-tokens.py crates/mdbook-core/src/book.rs:140-170`)
- Notes: `name, content, number, sub_items, path, source_path, parent_names`. Core data shape for preprocessor APIs.
- Predecessor: 2.7

### 2.9 `Preprocessor` trait + `PreprocessorContext`
- Content: `crates/mdbook-preprocessor/src/lib.rs` (whole, 85 lines)
- Cost: 703 tokens (helper: `count-tokens.py crates/mdbook-preprocessor/src/lib.rs`)
- Notes: `pub trait Preprocessor { name, run, supports_renderer }` and `PreprocessorContext { root, config, renderer, mdbook_version, chapter_titles }`. Anchor for the entire external preprocessor protocol.

### 2.10 `Renderer` trait + `RenderContext`
- Content: `crates/mdbook-renderer/src/lib.rs` (whole, 89 lines)
- Cost: 736 tokens (helper: `count-tokens.py crates/mdbook-renderer/src/lib.rs`)
- Notes: `pub trait Renderer { name, render }` and `RenderContext { version, root, book, config, destination, chapter_titles }`. Mirror of 2.9.

### 2.11 `Config` top-level struct
- Content: `crates/mdbook-core/src/config.rs:60-78`
- Cost: 152 tokens (helper: `count-tokens.py crates/mdbook-core/src/config.rs:60-78`)
- Notes: `pub struct Config { book, build, rust, output, preprocessor }`. Shape of `book.toml`.

### 2.12 `BookConfig` (the `[book]` table)
- Content: `crates/mdbook-core/src/config.rs:313-351`
- Cost: 300 tokens (helper: `count-tokens.py crates/mdbook-core/src/config.rs:313-351`)
- Notes: `title, authors, description, src, language, text_direction`. Most user-facing config table.
- Predecessor: 2.11

### 2.13 `BuildConfig` + `RustConfig` + `RustEdition`
- Content: `crates/mdbook-core/src/config.rs:390-444`
- Cost: 413 tokens (helper: `count-tokens.py crates/mdbook-core/src/config.rs:390-444`)
- Notes: `[build]` table (`build_dir, create_missing, use_default_preprocessors, extra_watch_dirs`) and `[rust]` (`edition` ∈ 2015/2018/2021/2024).
- Predecessor: 2.11

### 3.1 `mdbook` package definition (features + bin/test/example)
- Content: `Cargo.toml:72-158`
- Cost: 776 tokens (helper: `count-tokens.py Cargo.toml:72-158`)
- Notes: Cargo features `watch / serve / search` and which crates they enable; `gui` test harness wired through `tests/gui/runner.rs`; `remove-emphasis` example wired through `examples/remove-emphasis/test.rs`.
- Predecessor: 1.8

### 3.2 `HtmlConfig` (top-level fields)
- Content: `crates/mdbook-core/src/config.rs:445-521`
- Cost: 817 tokens (helper: `count-tokens.py crates/mdbook-core/src/config.rs:445-521`)
- Notes: Every supported `output.html.*` key (theme, smart_punctuation, definition_lists, admonitions, mathjax_support, additional_css/js, fold, playground, code, print, search, git_repository_url, input_404, site_url, cname, edit_url_template, redirect, hash_files, sidebar_header_nav). Sub-config detail (`Print/Fold/Playground/Code/Search` structs at lines 574-712) deferred to below-the-fold.
- Predecessor: 2.11

### 3.3 `HtmlConfig::theme_dir / get_404_output_file`
- Content: `crates/mdbook-core/src/config.rs:555-572`
- Cost: 153 tokens (helper: `count-tokens.py crates/mdbook-core/src/config.rs:555-572`)
- Notes: Theme directory resolution + `404.md` → `404.html` rule.
- Predecessor: 3.2

### 3.4 `Summary` struct + `Link` struct + `SummaryItem` enum
- Content: `crates/mdbook-summary/src/lib.rs:64-145`
- Cost: 567 tokens (helper: `count-tokens.py crates/mdbook-summary/src/lib.rs:64-145`)
- Notes: `Summary { title, prefix_chapters, numbered_chapters, suffix_chapters }`, `Link { name, location, number, nested_items }`, `enum SummaryItem { Link, Separator, PartTitle }`.

### 3.5 `parse_summary` doc + signature (SUMMARY.md format spec)
- Content: `crates/mdbook-summary/src/lib.rs:1-66`
- Cost: 574 tokens (helper: `count-tokens.py crates/mdbook-summary/src/lib.rs:1-66`)
- Notes: User-facing format spec — title / prefix chapters / part titles / numbered chapters / suffix chapters.

### 3.6 Builtin preprocessors module
- Content: `crates/mdbook-driver/src/builtin_preprocessors/mod.rs` (whole, 9 lines)
- Cost: 45 tokens (helper: `count-tokens.py crates/mdbook-driver/src/builtin_preprocessors/mod.rs`)
- Notes: Names the three preprocessors (`CmdPreprocessor`, `IndexPreprocessor`, `LinkPreprocessor`).

### 3.7 `LinkPreprocessor` declaration + doc-comment
- Content: `crates/mdbook-driver/src/builtin_preprocessors/links.rs:18-50`
- Cost: 296 tokens (helper: `count-tokens.py crates/mdbook-driver/src/builtin_preprocessors/links.rs:18-50`)
- Notes: Doc-comment lists `{{# include}}`, `{{# rustdoc_include}}`, `{{# playground}}`, `{{# title}}` — mdBook's most-asked feature. `NAME = "links"` and the unit struct.
- Predecessor: 3.6

### 3.8 Builtin renderers module + `CmdRenderer`
- Content: `crates/mdbook-driver/src/builtin_renderers/mod.rs` (whole, 89 lines)
- Cost: 615 tokens (helper: `count-tokens.py crates/mdbook-driver/src/builtin_renderers/mod.rs`)
- Notes: `CmdRenderer` is the protocol for third-party backends — naming, optional flag, JSON over stdin shell-out.

### 3.9 `MarkdownRenderer` (debug backend)
- Content: `crates/mdbook-driver/src/builtin_renderers/markdown_renderer.rs` (whole, 46 lines)
- Cost: 262 tokens (helper: `count-tokens.py crates/mdbook-driver/src/builtin_renderers/markdown_renderer.rs`)
- Notes: Tiny `Renderer` impl that writes preprocessed markdown back to disk.

### 3.10 `mdbook-markdown` (whole crate)
- Content: `crates/mdbook-markdown/src/lib.rs` (whole, 62 lines)
- Cost: 452 tokens (helper: `count-tokens.py crates/mdbook-markdown/src/lib.rs`)
- Notes: Tiny shim over `pulldown-cmark` exposing `MarkdownOptions { smart_punctuation, definition_lists, admonitions }` and `new_cmark_parser`. Answers "what markdown features are enabled?"

### 3.11 HTML pipeline doc-comment (markdown → tree → serialize)
- Content: `crates/mdbook-html/src/html/mod.rs:1-32`
- Cost: 256 tokens (helper: `count-tokens.py crates/mdbook-html/src/html/mod.rs:1-32`)
- Notes: Module-level doc explaining the pulldown-cmark → tree → serialize pipeline. The submodule list (`admonitions, hide_lines, print, serialize, tokenizer, tree, tests`) names every transformation. `HtmlRenderOptions / ChapterTree / render_markdown / build_trees` body deferred (one Read).

### 3.12 `Theme` struct + builtin theme bytes (`include_bytes!`)
- Content: `crates/mdbook-html/src/theme/mod.rs:1-62`
- Cost: 770 tokens (helper: `count-tokens.py crates/mdbook-html/src/theme/mod.rs:1-62`)
- Notes: `pub struct Theme { index, head, redirect, header, toc_js, toc_html, chrome_css, general_css, … }` + `static INDEX: &[u8] = include_bytes!("../../front-end/templates/index.hbs");` lines that name every builtin template/CSS/JS asset.

### 3.13 `mdbook-html` crate roots
- Content: `crates/mdbook-html/src/lib.rs` (whole, 9 lines), `crates/mdbook-html/src/html_handlebars/mod.rs` (whole, 8 lines)
- Cost: 68 tokens (helper: `count-tokens.py crates/mdbook-html/src/lib.rs crates/mdbook-html/src/html_handlebars/mod.rs`)
- Notes: Tiny module declarations exporting `HtmlHandlebars` and naming `html / html_handlebars / theme / utils` submodules.

### 3.14 `HtmlHandlebars` struct + `Renderer::render` opening
- Content: `crates/mdbook-html/src/html_handlebars/hbs_renderer.rs:1-30` and `:303-340`
- Cost: ≈ 410 tokens (helper: `count-tokens.py crates/mdbook-html/src/html_handlebars/hbs_renderer.rs:1-30 crates/mdbook-html/src/html_handlebars/hbs_renderer.rs:303-340`)
- Notes: `pub struct HtmlHandlebars` + opening of `Renderer for HtmlHandlebars::render` (theme directory selection, handlebars setup). Body deferred to below-the-fold.

### 4.1 `MDBook::build` / `preprocess_book` / `execute_build_process` bodies
- Content: `crates/mdbook-driver/src/mdbook.rs:160-210`
- Cost: 369 tokens (helper: `count-tokens.py crates/mdbook-driver/src/mdbook.rs:160-210`)
- Notes: `build` loops over renderers, `preprocess_book` runs preprocessors, `execute_build_process` glues them with the `RenderContext`. The chain that drives every `mdbook build`.
- Predecessor: 2.3

### 4.2 `cmd::serve` execute (axum + websocket)
- Content: `src/cmd/serve.rs:1-49` (subcommand args) + `:50-106` (execute body)
- Cost: 840 tokens (helper: `count-tokens.py src/cmd/serve.rs:1-49 src/cmd/serve.rs:50-106`)
- Notes: Uses axum + tokio + notify-debouncer-mini. Async `serve` body deferred.

### 4.3 `cmd::watch` (WatcherKind dispatch)
- Content: `src/cmd/watch.rs` (whole, 80 lines)
- Cost: 513 tokens (helper: `count-tokens.py src/cmd/watch.rs`)
- Notes: `enum WatcherKind { Poll, Native }` + `rebuild_on_change` dispatcher.

### 4.4 `cmd::build`
- Content: `src/cmd/build.rs` (whole, 37 lines)
- Cost: 220 tokens (helper: `count-tokens.py src/cmd/build.rs`)
- Notes: Canonical pipeline — load + optional dest-dir override + `book.build()` + open browser if `--open`.

### 4.5 `cmd::test`
- Content: `src/cmd/test.rs` (whole, 53 lines)
- Cost: 348 tokens (helper: `count-tokens.py src/cmd/test.rs`)
- Notes: Wires `--chapter` and `--library-path` flags to `MDBook::test_chapter`.

### 4.6 `LinkPreprocessor::run` body + `LinkType` enum + `RangeOrAnchor`
- Content: `crates/mdbook-driver/src/builtin_preprocessors/links.rs:44-80` and `:138-180`
- Cost: ≈ 590 tokens (helper: `count-tokens.py crates/mdbook-driver/src/builtin_preprocessors/links.rs:44-80 crates/mdbook-driver/src/builtin_preprocessors/links.rs:138-180`)
- Notes: `LinkPreprocessor::run` body + `enum LinkType { Escaped, Include, Playground, RustdocInclude, Title }` and `enum RangeOrAnchor { Range, Anchor }`. Body of `replace_all`, `Link::from_capture`, regex (~5800 toks) deferred.
- Predecessor: 3.7

### 4.7 `IndexPreprocessor` declaration + `run` + `is_readme_file`
- Content: `crates/mdbook-driver/src/builtin_preprocessors/index.rs:1-80`
- Cost: 576 tokens (helper: `count-tokens.py crates/mdbook-driver/src/builtin_preprocessors/index.rs:1-80`)
- Notes: Doc + struct + `NAME = "index"` + `Preprocessor::run` + `^readme$` regex helper (case-insensitive). Tests deferred.
- Predecessor: 3.6

### 4.8 `load_book` entry (Summary → Book)
- Content: `crates/mdbook-driver/src/load.rs:1-25`
- Cost: 234 tokens (helper: `count-tokens.py crates/mdbook-driver/src/load.rs:1-25`)
- Notes: Reads `SUMMARY.md`, applies `create_missing` flag, hands to `load_book_from_disk`. Body and `load_chapter` deferred.

### 5.1 Examples — `nop-preprocessor` (canonical custom-preprocessor template)
- Content: `examples/nop-preprocessor.rs:1-115`
- Cost: ≈ 770 tokens (helper: `count-tokens.py examples/nop-preprocessor.rs:1-115`)
- Notes: Full skeleton: `clap` + `supports`/preprocess subcommand + `mdbook_preprocessor::parse_input(stdin)` + `serde_json::to_writer(stdout, &book)` + `Preprocessor` impl. Reference template for custom preprocessors.

### 5.2 Testsuite README + main module
- Content: `tests/testsuite/README.md` (whole) and `tests/testsuite/main.rs` (whole, 31 lines)
- Cost: ≈ 670 tokens (helper: `count-tokens.py tests/testsuite/README.md tests/testsuite/main.rs`)
- Notes: Names every test module (`book_test`, `build`, `cli`, `config`, `includes`, `index`, `init`, `markdown`, `playground`, `preprocessor`, `print`, `redirects`, `renderer`, `rendering`, `search`, `test`, `theme`, `toc`) and explains the `BookTest` + snapbox conventions.

### 5.3 CONTRIBUTING.md — Tests + xtask commands
- Content: `CONTRIBUTING.md:101-152`
- Cost: 671 tokens (helper: `count-tokens.py CONTRIBUTING.md:101-152`)
- Notes: Build/test commands an agent needs: `cargo test --workspace`, `cargo test --test gui`, `npm run lint`, `cargo clippy --workspace --all-targets --no-deps -- -D warnings`, `cargo fmt --check`, `cargo +stable semver-checks`, and the umbrella `cargo xtask test-all`. Names the `xtask` crate as the maintainer entry point.

### 5.4 Guide `guide/src/SUMMARY.md`
- Content: `guide/src/SUMMARY.md` (whole, 44 lines)
- Cost: 364 tokens (helper: `count-tokens.py guide/src/SUMMARY.md`)
- Notes: Enumerates every documentation page mdBook ships with (CLI, Format, Theme, For developers, Continuous integration, etc.). Useful for "where is X documented?" — the user guide is `guide/src/`.

### 5.5 CHANGELOG.md head (current release notes — version anchor)
- Content: `CHANGELOG.md:1-23`
- Cost: 255 tokens (helper: `count-tokens.py CHANGELOG.md:1-23`)
- Notes: 0.5.2 release notes only. Version + recent fixes; the `…` invites the agent into the deeper file (24k toks total) when needed.

### 5.6 `mdbook-core` lib + `static_regex!` macro
- Content: `crates/mdbook-core/src/lib.rs` (whole, 16 lines) + `crates/mdbook-core/src/utils/mod.rs` (whole, 37 lines)
- Cost: ≈ 360 tokens (helper: `count-tokens.py crates/mdbook-core/src/lib.rs crates/mdbook-core/src/utils/mod.rs`)
- Notes: `MDBOOK_VERSION` const + `errors` re-export of anyhow + `static_regex!` macro definition (used pervasively).

## Below-the-fold

The following are deferred. For each, an agent can `Read` or `Grep` directly when needed; the batches above ensure the agent knows the file exists and roughly what it contains.

### `mdbook-html` deeper internals
- `crates/mdbook-html/src/html/mod.rs:33-108` (~590 toks) — `HtmlRenderOptions`, `ChapterTree`, `render_markdown`, `build_trees` impl. Surface in 3.11.
- `crates/mdbook-html/src/html_handlebars/hbs_renderer.rs:31-302, 341-696` (~5300 toks combined excluding what 3.14 already shows) — `render_chapter`, `render_404`, `render_print_page`, `register_hbs_helpers`, `emit_redirects`, fragment-redirect helpers, plus the full `Renderer::render` body that orchestrates template registration, chapter trees, static files, TOC, 404, print page, redirects, and `copy_files_except_ext`.
- `crates/mdbook-html/src/html_handlebars/hbs_renderer.rs:454-635` (~1356 toks) — `make_data` builds the JSON map fed to handlebars (every `HtmlConfig` field → template variable name).
- `crates/mdbook-html/src/html_handlebars/static_files.rs` (2591 toks) — `StaticFiles` doc + struct + builtin asset catalog (`book.js`, `css/{general, chrome, print, variables}.css`, favicons, `highlight.css`, `tomorrow-night.css`, `ayu-highlight.css`, `highlight.js`, `clipboard.min.js`, fonts) + content-hash fingerprinting impl.
- `crates/mdbook-html/src/html_handlebars/search.rs` (3148 toks) — search index construction (elasticlunr), gated on the `search` cargo feature. Generated assets: `searchindex.js`, `searcher.js`, `mark.min.js`, `elasticlunr.min.js`.
- `crates/mdbook-html/src/html_handlebars/helpers/{mod.rs, toc.rs, fontawesome.rs, resources.rs}` (~3000 toks combined) — handlebars custom helpers (TOC rendering, fontawesome icon shortcodes, asset URL resolver).
- `crates/mdbook-html/src/html/tree.rs:101-1155` (~8500 toks) — markdown→tree builder body. Type surface (`Node`, `Element`) at `:1-100` (~690 toks) is also deferred but worth fetching before the body.
- `crates/mdbook-html/src/html/serialize.rs` (745 toks) — DOM serializer; `serialize(tree, output)` plus `wants_pretty_html_newline` element list.
- `crates/mdbook-html/src/html/print.rs` (1827 toks) — `render_print_page` + id-rewriting and link-rewriting passes that make the print page work.
- `crates/mdbook-html/src/html/{tokenizer.rs (579), hide_lines.rs (1600), admonitions.rs (1844)}` — html5ever bridging, `~`-prefixed line elision in code blocks, inline-SVG icons for note/tip/important/warning/caution admonitions (Octicon SVG paths dominate the file).
- `crates/mdbook-html/src/html/tests.rs` — unit tests.
- `crates/mdbook-html/src/theme/mod.rs:62-333` (~2200 toks) — `Theme::new` body that overlays user-provided files, plus `copy_theme`.
- `crates/mdbook-html/src/theme/{fonts,playground_editor,searcher}.rs` — pure `include_bytes!` modules (asset bytes); irrelevant as text.
- `crates/mdbook-html/front-end/**` — CSS/JS/handlebars templates and woff2 font binaries. The themes are the *output* of the renderer; an agent answering theme-customization questions sees the asset *names* via 3.12 and `Read`s the specific file in one hop.
- `crates/mdbook-html/src/utils.rs` (1077 toks) — `unique_id`, `id_from_content`, `ToUrlPath`, `normalize_path` helpers.

### `mdbook-driver` deeper internals
- `crates/mdbook-driver/src/mdbook.rs:88-110` (~150 toks) — `load_with_config_and_summary`, the rare custom-summary entry point.
- `crates/mdbook-driver/src/mdbook.rs:267-345` (~520 toks) — `test_chapter` body shells out to `rustdoc --test`, passing `--edition` based on `[rust].edition`.
- `crates/mdbook-driver/src/mdbook.rs:395-541` (~1274 toks) — `determine_renderers` + `determine_preprocessors`. Decision logic for html/markdown/cmd renderers and topological-sort over preprocessor `before`/`after` config. `DEFAULT_PREPROCESSORS = &["links", "index"]`.
- `crates/mdbook-driver/src/builtin_preprocessors/links.rs:80-138, 180-936` (~5800 toks) — regex matchers, `replace_all`, `Link::render_with_path`, anchor extraction, extensive unit tests.
- `crates/mdbook-driver/src/builtin_preprocessors/links/take_lines.rs` (2421 toks) — `take_lines / take_anchored_lines / take_rustdoc_include_lines / take_rustdoc_include_anchored_lines` + tests.
- `crates/mdbook-driver/src/builtin_preprocessors/cmd.rs` (1158 toks) — `CmdPreprocessor` struct + JSON-over-stdin shell-out + tests. Mirrors `CmdRenderer` (3.8).
- `crates/mdbook-driver/src/builtin_preprocessors/index.rs:80-end` (~160 toks) — unit tests for `is_readme_file`.
- `crates/mdbook-driver/src/init.rs` (1054 toks) — `BookBuilder` that backs `MDBook::init`. Creates `book.toml`, `src/SUMMARY.md`, `src/chapter_1.md`, optionally `.gitignore`, optionally copies the default theme.
- `crates/mdbook-driver/src/load.rs:25-309` (~1900 toks) — `create_missing` (auto-stubs missing `.md` files referenced from `SUMMARY.md`), `load_chapter` body (BOM stripping), and extensive unit tests.
- `crates/mdbook-driver/src/lib.rs:67-132` (~450 toks) — `compose_command` + `handle_command_error` (sub-process shell-out helpers used by both `CmdPreprocessor` and `CmdRenderer`).
- `crates/mdbook-driver/src/mdbook/tests.rs` — unit tests.

### `mdbook-core` deeper internals
- `crates/mdbook-core/src/book.rs:1-29, 31-97, 218-289` (~1300 toks) — module doc, `Book` impl methods (`iter`, `chapters`, `for_each_mut`, `for_each_chapter_mut`), `SectionNumber`, `BookItems` iterator.
- `crates/mdbook-core/src/book/tests.rs` — unit tests.
- `crates/mdbook-core/src/config.rs:1-44` (~350 toks) — module doc with usage example.
- `crates/mdbook-core/src/config.rs:111-176` (~640 toks) — `update_from_env` documents the `MDBOOK_BOOK__TITLE` → `book.title` mapping with JSON fallback.
- `crates/mdbook-core/src/config.rs:178-310` (~858 toks) — `Config::get / set / contains_key / preprocessors / outputs / html_config` — the dotted-path API for accessing `output.*` and `preprocessor.*` tables.
- `crates/mdbook-core/src/config.rs:353-388` (~360 toks) — `BookConfig::default`, `TextDirection` enum, `realized_text_direction` derivation from language code.
- `crates/mdbook-core/src/config.rs:522-554` (~200 toks) — `HtmlConfig::default` impl.
- `crates/mdbook-core/src/config.rs:574-712` (~1134 toks) — `Print / Fold / Playground / Code / Search / SearchChapterSettings` HTML sub-configs. Default values inline (e.g. `limit_results: 30`, `boost_title: 2`, `heading_split_level: 3`).
- `crates/mdbook-core/src/config.rs:731-1209` (~3500 toks) — extensive unit tests of `Config` parsing.
- `crates/mdbook-core/src/utils/{fs.rs (2197), html.rs (687), toml_ext.rs (575)}` — `fs::path_to_root`, `fs::copy_files_except_ext`, `fs::write`, `escape_html`, `escape_html_attribute`, `TomlExt`. Used pervasively but each function is one Read away.

### Remaining `mdbook-summary`
- `crates/mdbook-summary/src/lib.rs:146-1201` (~7600 toks) — `SummaryParser` body (with grammar comment), `parse_parts`, `parse_dotted_item`, link parsing, error context, extensive tests.

### CLI internals
- `src/cmd/init.rs` (912 toks) — interactive book scaffolding: `--theme`, `--force`, `--title`, `--ignore` flags; reads `git config user.name`; prompt-based confirm for `.gitignore`.
- `src/cmd/clean.rs` (896 toks) — build-dir removal + a human-readable byte-count summary.
- `src/cmd/command_prelude.rs` (474 toks) — `arg_dest_dir / arg_root_dir / arg_open / arg_watcher` extension trait wiring `-d / --dest-dir`, `--watcher`, `-o / --open` consistently across commands.
- `src/cmd/watch/{native.rs (1432), poller.rs (2667)}` — full watcher implementations. The `WatcherKind` dispatcher in 4.3 names them.
- `src/cmd/serve.rs:108-152` (347 toks) — async `serve` body + websocket connection helper.
- `src/main.rs:57-150` (~770 toks) — `create_clap_command` + `init_logger` + `get_book_dir` helpers.

### Tests
- `tests/testsuite/{build, cli, config, includes, index, init, markdown, playground, preprocessor, print, redirects, renderer, rendering, search, test, theme, toc}.rs` and the per-test `book.toml` fixtures — large body of integration tests; the `BookTest` harness (next bullet) tells the agent how to read them.
- `tests/testsuite/book_test.rs` (~2000 toks) — `BookTest` harness with `from_dir / empty / init / check_main_file` and assertion helpers.
- `tests/gui/runner.rs` plus `tests/gui/books/**` (~30 books, plus `.goml` scripts) — browser-driven GUI tests behind `cargo test --test gui`.

### Tooling crates
- `crates/xtask/src/main.rs` (1160 toks) — maintainer commands (`test-all`, `test-workspace`, `clippy`, `doc`, `fmt`, `semver-checks`, `eslint`, `gui`, `changelog`, `bump`) + impls. Names mirror CONTRIBUTING.md (5.3).
- `crates/xtask/src/changelog.rs` (919 toks) — `changelog` command impl.
- `crates/mdbook-compare/src/main.rs` (887 toks) — 4-arg utility that builds two books with two mdbook binaries and HTML-tidies + diffs the output. Used to validate changes don't alter rendered HTML.
- `crates/{mdbook-compare,xtask}/Cargo.toml` and `crates/mdbook-compare/README.md` — package metadata.
- Sub-crate `Cargo.toml` files (`crates/mdbook-{core,driver,html,markdown,preprocessor,renderer,summary}/Cargo.toml`, ~600 toks combined) — each ~30 lines naming the dependencies of that crate. The workspace dependency graph is implicit from 1.7 + 1.8.

### Examples
- `examples/remove-emphasis/{book.toml, src/SUMMARY.md, src/chapter_1.md, test.rs, mdbook-remove-emphasis/{Cargo.toml, src/main.rs}}` (~750 toks total) — smallest end-to-end custom preprocessor, wired through `[preprocessor.remove-emphasis]` in `book.toml`. Same shape as `nop-preprocessor` (5.1).
- `examples/nop-preprocessor.rs:115-165` (~340 toks) — embedded `nop_lib` mod test.

### Guide
- `guide/book.toml` (255 toks) — mdBook's own user guide config — canonical example of a `book.toml`. Includes `[preprocessor.guide-helper]` shelling to `cargo run --quiet --manifest-path guide-helper/Cargo.toml`.
- `guide/src/**/*.md` (~80 docs, several tens of thousands of tokens) — user guide content. Fetch any specific page by name from the SUMMARY in 5.4.
- `guide/guide-helper/{src/lib.rs, src/main.rs, Cargo.toml}` (~600 toks) — internal preprocessor for the guide.
- `guide/src/for_developers/mdbook-wordcount/` — third example preprocessor that ships with the docs; redundant with 5.1 in shape.

### CONTRIBUTING.md remainder
- `CONTRIBUTING.md:1-100` (~1200 toks) — issue assignment, code-quality (rustfmt/clippy), change requirements (semver, tests, docs).
- `CONTRIBUTING.md:152-end` (~1000 toks) — pull-request process, browser-compatibility/GUI-test details, JS lint setup, highlight.js update procedure, release process.

### Plumbing
- `crates/*/README.md` (8 files, ~600 toks combined) — one-line readmes pointing at the user guide.
- `Cargo.lock` (huge) — only needed for "what version of dep X is pinned?", trivially `Grep`-able.
- `LICENSE`, `CODE_OF_CONDUCT.md`, `triagebot.toml`, `rustfmt.toml`, `package.json`, `eslint.config.mjs`, `.cargo/config.toml`, `.github/**` (workflows, issue templates), `.git-blame-ignore-revs`, `.gitattributes`, `.gitignore`, `ci/*.sh` — repository plumbing. Visible by name in the root listing (1.2); content is one Read or `gh` call away.
- `CHANGELOG.md:24-end` (~24k toks) — historical release notes including the 0.5 migration guide. Current version anchored in 5.5; older entries are grep-targets.
