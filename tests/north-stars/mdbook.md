# mdbook — North Star

Revision pin: `b8c90970`

mdBook is a CLI + library for turning a directory of Markdown files (organized by a `SUMMARY.md`) into a static HTML book. The same `mdbook` binary also runs `rustdoc --test` over Rust code blocks (`mdbook test`), serves the book over HTTP with live reload (`mdbook serve`), and watches for changes (`mdbook watch`). The project is a Cargo workspace: a thin `mdbook` binary in `src/` dispatches to a `mdbook-driver` facade that composes `mdbook-summary` (parser for `SUMMARY.md`), `mdbook-core` (config + book model), `mdbook-markdown` (pulldown-cmark wrapper), `mdbook-html` (Handlebars HTML renderer with bundled theme + search index), and the public extension surfaces `mdbook-preprocessor` and `mdbook-renderer`. The `guide/` directory is itself an mdbook book, doubling as the user manual and the largest end-to-end fixture.

## Batches

### 1.1 Repo root file/folder listing
- Content: rendered as one entry per line (folders suffixed `/`):
  `CHANGELOG.md`, `CODE_OF_CONDUCT.md`, `CONTRIBUTING.md`, `Cargo.lock`, `Cargo.toml`, `LICENSE`, `README.md`, `ci/`, `crates/`, `eslint.config.mjs`, `examples/`, `guide/`, `package.json`, `rustfmt.toml`, `src/`, `tests/`, `triagebot.toml`
- Cost: 60 tokens (helper: `printf '...\n' | count-tokens.py --stdin`)
- Notes: Top of the orientation pyramid. Surfaces the unusual presence of a top-level `src/` (the binary) **alongside** `crates/` (the library workspace), `guide/` (mdBook's own user guide built with itself), `examples/`, `tests/`, and front-end JS tooling (`package.json`, `eslint.config.mjs`).

### 1.2 README one-liner
- Content: `README.md:7` — "mdBook is a utility to create modern online books from Markdown files."
- Cost: 14 tokens (helper: `count-tokens.py README.md:7`)
- Notes: Tiny but eliminates the catastrophic "what even is this repo?" failure mode at almost zero cost.

### 1.3 `crates/` workspace member listing
- Content: rendered one per line: `mdbook-compare/`, `mdbook-core/`, `mdbook-driver/`, `mdbook-html/`, `mdbook-markdown/`, `mdbook-preprocessor/`, `mdbook-renderer/`, `mdbook-summary/`, `xtask/`
- Cost: 39 tokens (helper: stdin)
- Notes: Workspace fragmentation into many small crates is the central architectural fact about this codebase.

### 1.4 Crate-name → one-line description map
- Content: rendered list mapping each workspace crate to its `Cargo.toml` `description` (or, for `mdbook-compare`/`xtask` which lack a description, a derived one-liner): `mdbook-core: The base support library for mdbook, intended for internal use only`, `mdbook-driver: High-level library for running mdBook`, `mdbook-html: mdBook HTML renderer`, `mdbook-markdown: Markdown processing used in mdBook`, `mdbook-preprocessor: Library to assist implementing an mdBook preprocessor`, `mdbook-renderer: Library to assist implementing an mdBook renderer`, `mdbook-summary: Summary parser for mdBook`, `mdbook-compare: Utility comparing mdbook output between two versions`, `xtask: Local dev-task runner`
- Cost: 112 tokens (helper: stdin)
- Notes: Authoritative single-line role per crate; covers every workspace member. Cheaper than the prose in `mdbook-driver/src/lib.rs:14-25` and includes `mdbook-compare`/`xtask` (which that prose doesn't).

### 1.5 `src/` and `src/cmd/` listings
- Content: `src/`: `main.rs`, `cmd/`; `src/cmd/`: `build.rs`, `clean.rs`, `command_prelude.rs`, `init.rs`, `mod.rs`, `serve.rs`, `test.rs`, `watch.rs`, `watch/native.rs`, `watch/poller.rs`
- Cost: 41 tokens (helper: stdin, two listings combined)
- Notes: One file per CLI subcommand. "Where is `mdbook serve` implemented?" → one hop.

### 1.6 Subcommand → about-line map
- Content: rendered text of every `Command::new("X").about("...")` from `src/cmd/{build,init,clean,serve,test,watch}.rs` plus `completions` from `src/main.rs` (`build: Builds a book from its markdown files`, `init: Creates the boilerplate structure and files for a new book`, `clean: Deletes a built book`, `serve: Serves a book at http://localhost:3000, and rebuilds it on changes`, `test: Tests that a book's Rust code samples compile`, `watch: Watches a book's files and rebuilds it on changes`, `completions: Generate shell completions for your shell to stdout`)
- Cost: 92 tokens (helper: stdin)
- Notes: Whole CLI surface in one tiny batch. Answers "what subcommands does mdbook have and what does each do?" with zero follow-up.

### 1.7 Workspace `Cargo.toml` header (members + lints + workspace.package)
- Content: `Cargo.toml:1-26` — `[workspace] members` (`crates/*`, `examples/remove-emphasis/...`, `guide/guide-helper`), `[workspace.lints.*]` (warns on `missing_docs`, `unreachable_pub`, `rust_2018_idioms`), `[workspace.package]` with edition 2024, MPL-2.0, rust-version 1.88.0
- Cost: 198 tokens (helper: `count-tokens.py Cargo.toml:1-26`)

### 1.8 mdbook CLI subcommand dispatch
- Content: `src/main.rs:18-55` — `main()` body with the clap `match` over `init`/`build`/`clean`/`watch`/`serve`/`test`/`completions`, with `#[cfg(feature = "watch"|"serve")]` gating
- Cost: 291 tokens (helper: `count-tokens.py src/main.rs:18-55`)

### 1.9 `MDBook` struct fields
- Content: `crates/mdbook-driver/src/mdbook.rs:28-44` — `pub struct MDBook { pub root, pub config, pub book, renderers: IndexMap<String, Box<dyn Renderer>>, preprocessors: IndexMap<String, Box<dyn Preprocessor>> }`
- Cost: 116 tokens (helper: `count-tokens.py crates/mdbook-driver/src/mdbook.rs:28-44`)
- Notes: Central in-memory representation; the field types reveal the build pipeline.

### 1.10 `Preprocessor` trait
- Content: `crates/mdbook-preprocessor/src/lib.rs:30-45` — `pub trait Preprocessor { fn name; fn run(&self, &PreprocessorContext, Book) -> Result<Book>; fn supports_renderer(&self, &str) -> Result<bool> }`
- Cost: 133 tokens (helper: `count-tokens.py crates/mdbook-preprocessor/src/lib.rs:30-45`)

### 1.11 `Renderer` trait
- Content: `crates/mdbook-renderer/src/lib.rs:25-35` — `pub trait Renderer { fn name; fn render(&self, ctx: &RenderContext) -> Result<()> }`
- Cost: 108 tokens (helper: `count-tokens.py crates/mdbook-renderer/src/lib.rs:25-35`)

### 1.12 `LinkPreprocessor` doc-comment listing all `{{# ... }}` helpers
- Content: `crates/mdbook-driver/src/builtin_preprocessors/links.rs:20-32` — doc lists `{{# include}}`, `{{# rustdoc_include}}`, `{{# playground}}`, `{{# title}}`
- Cost: 171 tokens (helper: `count-tokens.py crates/mdbook-driver/src/builtin_preprocessors/links.rs:20-32`)
- Notes: Authoritative list of supported link helpers — mdBook's most-asked feature.

### 1.13 `IndexPreprocessor` doc-comment
- Content: `crates/mdbook-driver/src/builtin_preprocessors/index.rs:8-22` — explains the `README.md` → `index.md` transformation
- Cost: 102 tokens (helper: `count-tokens.py crates/mdbook-driver/src/builtin_preprocessors/index.rs:8-22`)

### 1.14 Guide `SUMMARY.md` (full)
- Content: `guide/src/SUMMARY.md` whole file
- Cost: 364 tokens (helper: `count-tokens.py guide/src/SUMMARY.md`)
- Notes: Budget-independent ToC for the entire user-facing documentation. The most complete feature-map of mdBook in the entire fixture; lets the agent jump from any user concept to the precise `guide/src/...` file in one hop.

## 2. Core data model and config shape

### 2.1 `mdbook-driver` module declarations + re-exports
- Content: `crates/mdbook-driver/src/lib.rs:67-79` — `pub mod builtin_preprocessors`, `pub mod builtin_renderers`, `pub mod init`, `mod load`, `mod mdbook`, `pub use mdbook::MDBook`, re-exports of `book`, `config`, `errors`
- Cost: 83 tokens (helper: `count-tokens.py crates/mdbook-driver/src/lib.rs:67-79`)

### 2.2 `mdbook-core` lib.rs
- Content: `crates/mdbook-core/src/lib.rs` whole — declares `MDBOOK_VERSION`, `pub mod book`, `pub mod config`, `pub mod utils`, `errors` re-exports anyhow
- Cost: 96 tokens (helper: `count-tokens.py crates/mdbook-core/src/lib.rs`)

### 2.3 `Book` struct
- Content: `crates/mdbook-core/src/book.rs:12-29` — module doc + `pub struct Book { pub items: Vec<BookItem> }`
- Cost: 149 tokens (helper: `count-tokens.py crates/mdbook-core/src/book.rs:12-29`)

### 2.4 `BookItem` enum
- Content: `crates/mdbook-core/src/book.rs:113-127` — `pub enum BookItem { Chapter(Chapter), Separator, PartTitle(String) }`
- Cost: 92 tokens (helper: `count-tokens.py crates/mdbook-core/src/book.rs:113-127`)
- Predecessor: 2.3

### 2.5 `Chapter` struct
- Content: `crates/mdbook-core/src/book.rs:134-170` — fields (`name`, `content`, `number`, `sub_items`, `path`, `source_path`, `parent_names`) with the doc comments explaining the README→`index.md` rewrite and draft-chapter semantics
- Cost: 362 tokens (helper: `count-tokens.py crates/mdbook-core/src/book.rs:134-170`)
- Predecessor: 2.4
- Notes: Core data shape for preprocessor APIs. The `path` vs `source_path` doc is critical.

### 2.6 `Config` struct
- Content: `crates/mdbook-core/src/config.rs:60-78` — `pub struct Config { pub book, pub build, pub rust, output: Value, preprocessor: Value }` (shape of `book.toml`)
- Cost: 152 tokens (helper: `count-tokens.py crates/mdbook-core/src/config.rs:60-78`)

### 2.7 `BookConfig` + `TextDirection`
- Content: `crates/mdbook-core/src/config.rs:313-388` — `BookConfig { title, authors, description, src, language, text_direction }` with defaults, plus `TextDirection` enum and `from_lang_code` (RTL language list)
- Cost: 658 tokens (helper: `count-tokens.py crates/mdbook-core/src/config.rs:313-388`)
- Predecessor: 2.6

### 2.8 `BuildConfig` + `RustConfig` + `RustEdition`
- Content: `crates/mdbook-core/src/config.rs:390-440` — `BuildConfig { build_dir="book", create_missing=true, use_default_preprocessors=true, extra_watch_dirs }`, `RustConfig { edition }`, `RustEdition { E2024, E2021, E2018, E2015 }`
- Cost: 398 tokens (helper: `count-tokens.py crates/mdbook-core/src/config.rs:390-440`)
- Predecessor: 2.6

### 2.9 `Summary` + `Link` + `SummaryItem`
- Content: `crates/mdbook-summary/src/lib.rs:64-145` — `Summary { title, prefix_chapters, numbered_chapters, suffix_chapters }`, `Link { name, location, number, nested_items }`, `Link::new`/`Default`, `enum SummaryItem { Link, Separator, PartTitle }`
- Cost: 567 tokens (helper: `count-tokens.py crates/mdbook-summary/src/lib.rs:64-145`)

### 2.10 `parse_summary` doc + signature (SUMMARY.md format spec)
- Content: `crates/mdbook-summary/src/lib.rs:1-62` — crate doc + `parse_summary` signature explaining title / prefix chapters / part titles / numbered chapters / suffix chapters
- Cost: 538 tokens (helper: `count-tokens.py crates/mdbook-summary/src/lib.rs:1-62`)

### 2.11 SUMMARY.md grammar (BNF-ish)
- Content: `crates/mdbook-summary/src/lib.rs:147-181` — formal grammar comment on `SummaryParser`
- Cost: 262 tokens (helper: `count-tokens.py crates/mdbook-summary/src/lib.rs:147-181`)
- Predecessor: 2.10

### 2.12 `PreprocessorContext` + crate doc + `parse_input`
- Content: `crates/mdbook-preprocessor/src/lib.rs:1-22` (crate doc) + `crates/mdbook-preprocessor/src/lib.rs:48-84` (`PreprocessorContext { root, config, renderer, mdbook_version, chapter_titles }`, `PreprocessorContext::new`, `parse_input(stdin) -> (PreprocessorContext, Book)`)
- Cost: 473 tokens (helper: `count-tokens.py crates/mdbook-preprocessor/src/lib.rs:1-22 crates/mdbook-preprocessor/src/lib.rs:48-84`)
- Predecessor: 1.10

### 2.13 `RenderContext` + crate doc
- Content: `crates/mdbook-renderer/src/lib.rs:1-22` (crate doc) + `crates/mdbook-renderer/src/lib.rs:38-89` (`RenderContext { version, root, book, config, destination, chapter_titles }`, `RenderContext::new`/`source_dir`/`from_json`)
- Cost: 607 tokens (helper: `count-tokens.py crates/mdbook-renderer/src/lib.rs:1-22 crates/mdbook-renderer/src/lib.rs:38-89`)
- Predecessor: 1.11

### 2.14 `MDBook::load` + `load_with_config` + `iter`
- Content: `crates/mdbook-driver/src/mdbook.rs:46-110` — `load`, `load_with_config` (calls `load_book` on `<book_root>/SUMMARY.md` + `determine_renderers` + `determine_preprocessors`), `load_with_config_and_summary`, `iter`
- Cost: 437 tokens (helper: `count-tokens.py crates/mdbook-driver/src/mdbook.rs:46-110`)
- Predecessor: 1.9

### 2.15 `MDBook::init` (BookBuilder doc with directory layout)
- Content: `crates/mdbook-driver/src/mdbook.rs:111-159` — doc with ASCII directory layout (`book/`, `src/SUMMARY.md`, `src/chapter_1.md`); returns `BookBuilder`
- Cost: 400 tokens (helper: `count-tokens.py crates/mdbook-driver/src/mdbook.rs:111-159`)
- Predecessor: 1.9

### 2.16 `MDBook::with_renderer` + `with_preprocessor` (embedder hooks)
- Content: `crates/mdbook-driver/src/mdbook.rs:213-225`
- Cost: 121 tokens (helper: `count-tokens.py crates/mdbook-driver/src/mdbook.rs:213-225`)
- Notes: Used to register custom renderers/preprocessors at runtime instead of via `book.toml`.
- Predecessor: 1.9

### 2.17 `MDBook::test` / `test_chapter` (signatures)
- Content: `crates/mdbook-driver/src/mdbook.rs:227-265`
- Cost: 324 tokens (helper: `count-tokens.py crates/mdbook-driver/src/mdbook.rs:227-265`)
- Notes: Surface for `mdbook test`. Body shells out to `rustdoc --test` (deferred).
- Predecessor: 1.9

### 2.18 `MDBook::build_dir_for` / `source_dir` / `theme_dir`
- Content: `crates/mdbook-driver/src/mdbook.rs:347-393`
- Cost: 339 tokens (helper: `count-tokens.py crates/mdbook-driver/src/mdbook.rs:347-393`)
- Notes: `build_dir_for` documents the multi-renderer subdirectory layout (`book/html/`, `book/epub/`, etc.).
- Predecessor: 1.9

### 2.19 Built-in preprocessors module
- Content: `crates/mdbook-driver/src/builtin_preprocessors/mod.rs` whole — names the three preprocessors (`CmdPreprocessor`, `IndexPreprocessor`, `LinkPreprocessor`)
- Cost: 45 tokens (helper: `count-tokens.py crates/mdbook-driver/src/builtin_preprocessors/mod.rs`)

### 2.20 `mdbook-html` crate root + html/ module pipeline doc
- Content: `crates/mdbook-html/src/lib.rs` whole + `crates/mdbook-html/src/html/mod.rs:1-32` (markdown→tree→serialize pipeline doc + submodule list)
- Cost: 290 tokens (helper: `count-tokens.py crates/mdbook-html/src/lib.rs crates/mdbook-html/src/html/mod.rs:1-32`)
- Notes: One-paragraph overview of the markdown→HTML pipeline (`pulldown_cmark` events → `tree` → `serialize`) with the submodule names.

### 2.21 `mdbook-html/src/` recursive listing (html/, html_handlebars/, theme/)
- Content: rendered listings of `crates/mdbook-html/src/html/`: `admonitions.rs`, `hide_lines.rs`, `mod.rs`, `print.rs`, `serialize.rs`, `tests.rs`, `tokenizer.rs`, `tree.rs`; `crates/mdbook-html/src/html_handlebars/`: `hbs_renderer.rs`, `helpers/`, `mod.rs`, `search.rs`, `static_files.rs`; `crates/mdbook-html/src/theme/`: `fonts.rs`, `mod.rs`, `playground_editor.rs`, `searcher.rs`
- Cost: 56 tokens (helper: stdin, three listings combined)

### 2.22 Built-in renderers module + `CmdRenderer` declaration
- Content: `crates/mdbook-driver/src/builtin_renderers/mod.rs:1-30` — module doc + `pub use markdown_renderer::MarkdownRenderer` + `CmdRenderer { name, cmd }` struct intro
- Cost: 211 tokens (helper: `count-tokens.py crates/mdbook-driver/src/builtin_renderers/mod.rs:1-15 crates/mdbook-driver/src/builtin_renderers/mod.rs:15-30`)

## 3. Build pipeline + CLI subcommand surfaces

### 3.1 `make_subcommand` for build/init/clean/test/watch
- Content: `src/cmd/build.rs:8-14`, `src/cmd/init.rs:11-30`, `src/cmd/clean.rs:11-16`, `src/cmd/test.rs:9-32`, `src/cmd/watch.rs:12-19`
- Cost: 478 tokens (helper: sum of those ranges; build:47 + init:178 + clean:38 + test:158 + watch:57)
- Notes: Every flag for these five subcommands at once. Agent answers "what flags does `mdbook init --foo` take?" without follow-up.

### 3.2 `make_subcommand` for serve + `LIVE_RELOAD_ENDPOINT`
- Content: `src/cmd/serve.rs:1-50` — imports, `LIVE_RELOAD_ENDPOINT = "__livereload"` constant, then the clap subcommand: `--hostname=localhost`, `--port=3000`, `arg_open`, `arg_watcher`
- Cost: 382 tokens (helper: `count-tokens.py src/cmd/serve.rs:1-50`)
- Notes: Split out because serve has more flags + the live-reload constant is load-bearing.

### 3.3 `command_prelude` (full)
- Content: `src/cmd/command_prelude.rs` whole — the `CommandExt` trait shared by subcommands: `arg_dest_dir` (`-d/--dest-dir`), `arg_root_dir` (positional `[dir]`), `arg_open` (`-o/--open`), `arg_watcher` (`--watcher poll|native`); plus `set_dest_dir` updating `BookConfig.build.build_dir`
- Cost: 474 tokens (helper: `count-tokens.py src/cmd/command_prelude.rs`)

### 3.4 `MDBook::build` + `preprocess_book` + `execute_build_process`
- Content: `crates/mdbook-driver/src/mdbook.rs:160-210` — `build` loops over renderers, `preprocess_book` runs preprocessors, `execute_build_process` glues them with the `RenderContext`
- Cost: 369 tokens (helper: `count-tokens.py crates/mdbook-driver/src/mdbook.rs:160-210`)
- Notes: The chain that drives every `mdbook build`. `chapter_titles` extension is visible.
- Predecessor: 1.9

### 3.5 `mdbook-markdown` (whole crate)
- Content: `crates/mdbook-markdown/src/lib.rs` whole — re-exports `pulldown_cmark`, `MarkdownOptions { smart_punctuation, definition_lists, admonitions }`, `new_cmark_parser` enabling TABLES/FOOTNOTES/STRIKETHROUGH/TASKLISTS/HEADING_ATTRIBUTES + optional SMART_PUNCTUATION/DEFINITION_LIST/GFM
- Cost: 452 tokens (helper: `count-tokens.py crates/mdbook-markdown/src/lib.rs`)
- Notes: Tiny shim over `pulldown-cmark`. Answers "what markdown features does mdBook enable?"

### 3.6 `LinkPreprocessor` declaration + `Preprocessor::run` head
- Content: `crates/mdbook-driver/src/builtin_preprocessors/links.rs:1-50` — imports, `LinkPreprocessor` unit struct, `NAME = "links"`, `new()`, `Preprocessor::run` body that walks the book and calls `replace_all`. Followed by elision marker for the deferred `replace_all`/regex internals.
- Cost: 440 tokens (helper: `count-tokens.py crates/mdbook-driver/src/builtin_preprocessors/links.rs:1-50`)
- Predecessor: 1.12

### 3.7 `LinkType` + `RangeOrAnchor` enums (the link-helper grammar)
- Content: `crates/mdbook-driver/src/builtin_preprocessors/links.rs:138-170` — `enum LinkType { Escaped, Include, Playground, RustdocInclude, Title }`, `enum RangeOrAnchor { Range, Anchor }`, plus the regex captures for parsing
- Cost: 228 tokens (helper: `count-tokens.py crates/mdbook-driver/src/builtin_preprocessors/links.rs:138-170`)
- Predecessor: 3.6

### 3.8 `IndexPreprocessor` (full small file)
- Content: `crates/mdbook-driver/src/builtin_preprocessors/index.rs:1-50` — doc + struct + `NAME = "index"` + `Preprocessor::run` + warning helper. Tests deferred.
- Cost: 354 tokens (helper: `count-tokens.py crates/mdbook-driver/src/builtin_preprocessors/index.rs:1-50`)
- Predecessor: 1.13

### 3.9 `CmdRenderer` (full)
- Content: `crates/mdbook-driver/src/builtin_renderers/mod.rs` whole — `CmdRenderer { name, cmd }` + `Renderer::render` impl that spawns the configured command and pipes the JSON `RenderContext` to its stdin (the alternative-backend protocol)
- Cost: 615 tokens (helper: `count-tokens.py crates/mdbook-driver/src/builtin_renderers/mod.rs`)
- Notes: Authoritative protocol for third-party backends — naming, optional flag, JSON over stdin shell-out.
- Predecessor: 2.22

### 3.10 `CmdPreprocessor` declaration + `write_input`
- Content: `crates/mdbook-driver/src/builtin_preprocessors/cmd.rs:1-50` — `CmdPreprocessor { name, cmd, root, optional }`, `new`, `write_input_to_child` that pipes JSON `(ctx, book)` to the child preprocessor's stdin
- Cost: 374 tokens (helper: `count-tokens.py crates/mdbook-driver/src/builtin_preprocessors/cmd.rs:1-50`)
- Notes: Mirrors `CmdRenderer` (3.9) for preprocessors.
- Predecessor: 1.10

### 3.11 mdbook binary `[package]` + `[dependencies]`
- Content: `Cargo.toml:72-104` — `[package] name="mdbook", version="0.5.2"`, authors, `[dependencies]` listing `clap`, `clap_complete`, internal `mdbook-*` crates, `opener`, `toml`, `tracing`
- Cost: 245 tokens (helper: `count-tokens.py Cargo.toml:72-104`)
- Predecessor: 1.7

### 3.12 mdbook binary optional deps + features
- Content: `Cargo.toml:105-134` — optional `notify`/`notify-debouncer-mini`/`ignore`/`pathdiff`/`walkdir` (watch), `axum`/`futures-util`/`tokio`/`tower-http` (serve), `[dev-dependencies]`, `[features] default = ["watch", "serve", "search"]`
- Cost: 295 tokens (helper: `count-tokens.py Cargo.toml:105-134`)
- Predecessor: 3.11

### 3.13 `HtmlConfig` (every field)
- Content: `crates/mdbook-core/src/config.rs:441-553` — every field of `HtmlConfig` (theme, default_theme, preferred_dark_theme, smart_punctuation, definition_lists, admonitions, mathjax_support, additional_css/js, fold, playground, code, print, no_section_label, search, git_repository_url, git_repository_icon, input_404, site_url, cname, edit_url_template, live_reload_endpoint, redirect, hash_files, sidebar_header_nav) with doc comments and `Default` impl
- Cost: 1021 tokens (helper: `count-tokens.py crates/mdbook-core/src/config.rs:441-553`)
- Notes: 25+ HTML-renderer-specific fields; the dominant config surface in practice. Sub-config detail (`Print`/`Fold`/`Playground`/`Code`/`Search`) deferred to below-the-fold.
- Predecessor: 2.6

### 3.14 `HtmlConfig::theme_dir` / `get_404_output_file`
- Content: `crates/mdbook-core/src/config.rs:555-572`
- Cost: 153 tokens (helper: `count-tokens.py crates/mdbook-core/src/config.rs:555-572`)
- Notes: Theme directory resolution + `404.md` → `404.html` rule.
- Predecessor: 3.13

### 3.15 `Theme` struct + builtin theme `include_bytes!` lines
- Content: `crates/mdbook-html/src/theme/mod.rs:14-32` — `static INDEX/HEAD/REDIRECT/HEADER/TOC_JS/TOC_HTML/CHROME_CSS/GENERAL_CSS/PRINT_CSS/VARIABLES_CSS/FAVICON_PNG/FAVICON_SVG/JS/HIGHLIGHT_JS/TOMORROW_NIGHT_CSS/HIGHLIGHT_CSS/AYU_HIGHLIGHT_CSS/CLIPBOARD_JS = include_bytes!("../../front-end/...")` lines that name every builtin template/CSS/JS asset
- Cost: 391 tokens (helper: `count-tokens.py crates/mdbook-html/src/theme/mod.rs:14-32`)
- Notes: Names every theme file the user can override and where the defaults live. The `Theme` struct fields (`:33-61`) and `Theme::new` body are deferred.

### 3.16 `Config::from_disk` + `update_from_env`
- Content: `crates/mdbook-core/src/config.rs:111-178` — `from_disk` (reads file + `Config::from_str`), `update_from_env` with `MDBOOK_*__*` rules (kebab-case, `__` = dot, `_` = dash, JSON-or-string fallback)
- Cost: 649 tokens (helper: `count-tokens.py crates/mdbook-core/src/config.rs:111-178`)
- Predecessor: 2.6

### 3.17 `mdbook-core::config` module doc with TOML example
- Content: `crates/mdbook-core/src/config.rs:1-46` — module doc explaining `Config` is a bag of tables, plus an end-to-end example loading a `book.toml` via `Config::from_str`, mutating, querying
- Cost: 355 tokens (helper: `count-tokens.py crates/mdbook-core/src/config.rs:1-46`)

## 4. Implementations and execution

### 4.1 `cmd::build::execute`
- Content: `src/cmd/build.rs:17-36` — load `MDBook`, `book.build()`, `--open` opens `index.html`
- Cost: 128 tokens (helper: `count-tokens.py src/cmd/build.rs:17-36`)
- Predecessor: 3.1

### 4.2 `cmd::test::execute`
- Content: `src/cmd/test.rs:35-52` — wires `--chapter` and `--library-path` flags to `MDBook::test_chapter`/`MDBook::test`
- Cost: 135 tokens (helper: `count-tokens.py src/cmd/test.rs:35-52`)
- Predecessor: 3.1

### 4.3 `cmd::init::execute`
- Content: `src/cmd/init.rs:32-85` — interactive book scaffolding: handles `--theme`/`--force`/`--ignore`, reads `git config user.name`, prompts for `.gitignore`/title, then `BookBuilder::with_config(...).build()`
- Cost: 414 tokens (helper: `count-tokens.py src/cmd/init.rs:32-85`)
- Predecessor: 3.1

### 4.4 `cmd::serve::execute` body
- Content: `src/cmd/serve.rs:51-106` — sets `output.html.live-reload-endpoint` + `output.html.site-url="/"`, builds the book, spawns axum `Router` with WebSocket handler + `tokio::sync::broadcast` for reload, integrates with `watch::rebuild_on_change`
- Cost: 457 tokens (helper: `count-tokens.py src/cmd/serve.rs:51-106`)
- Notes: Live-reload + websocket + watch composition.
- Predecessor: 3.2

### 4.5 `determine_renderers`
- Content: `crates/mdbook-driver/src/mdbook.rs:401-440` — `OutputConfig` deserialize, mapping `[output.X]` → `HtmlHandlebars` (X="html") / `MarkdownRenderer` (X="markdown") / `CmdRenderer` (else, command default `mdbook-X`); HTML default fallback when no `[output.*]` section
- Cost: 332 tokens (helper: `count-tokens.py crates/mdbook-driver/src/mdbook.rs:401-440`)
- Predecessor: 3.4

### 4.6 `determine_preprocessors` head + `DEFAULT_PREPROCESSORS`
- Content: `crates/mdbook-driver/src/mdbook.rs:441-475` — `DEFAULT_PREPROCESSORS = ["links", "index"]`, `is_default_preprocessor`, `PreprocessorConfig` deserialize, head of `determine_preprocessors`. The full topological-sort body deferred.
- Cost: 277 tokens (helper: `count-tokens.py crates/mdbook-driver/src/mdbook.rs:441-475`)
- Predecessor: 3.4, 2.19

### 4.7 `preprocessor_should_run`
- Content: `crates/mdbook-driver/src/mdbook.rs:541-569` — consults `preprocessor.X.renderers` config, falls back to `Preprocessor::supports_renderer`
- Cost: 244 tokens (helper: `count-tokens.py crates/mdbook-driver/src/mdbook.rs:541-569`)
- Predecessor: 4.6

### 4.8 `load_book` flow head + `create_missing`
- Content: `crates/mdbook-driver/src/load.rs:1-55` — `load_book` reads `SUMMARY.md`, conditionally calls `create_missing` (stub-creates absent chapter files with `# {title}\n`), then `load_book_from_disk`. Followed by elision marker for the per-chapter loader past line 55.
- Cost: 418 tokens (helper: `count-tokens.py crates/mdbook-driver/src/load.rs:1-55`)
- Predecessor: 2.14

### 4.9 `compose_command` + `handle_command_error`
- Content: `crates/mdbook-driver/src/lib.rs:81-131` — `compose_command` shlex-splits a config command string and resolves single-component executables via PATH vs relative paths under book root; `handle_command_error` softens missing-command errors when `optional = true`
- Cost: 369 tokens (helper: `count-tokens.py crates/mdbook-driver/src/lib.rs:81-131`)
- Notes: Sub-process shell-out helpers used by both `CmdPreprocessor` and `CmdRenderer`.

### 4.10 `BookBuilder` head + builder methods
- Content: `crates/mdbook-driver/src/init.rs:11-65` — `BookBuilder { root, create_gitignore, config, copy_theme }`, `new`/`with_config`/`copy_theme`/`create_gitignore`. Body of `BookBuilder::build` (creates `book.toml`, `src/SUMMARY.md`, `src/chapter_1.md`, optional `.gitignore` + theme) deferred.
- Cost: 414 tokens (helper: `count-tokens.py crates/mdbook-driver/src/init.rs:11-65`)
- Predecessor: 2.15

### 4.11 `Book` impl methods
- Content: `crates/mdbook-core/src/book.rs:31-110` — `Book::new`, `new_with_items`, `iter()`, `chapters()` (filters draft chapters), `for_each_mut`, `for_each_chapter_mut`, `push_item`, plus the recursive `for_each_mut` helper
- Cost: 548 tokens (helper: `count-tokens.py crates/mdbook-core/src/book.rs:31-110`)
- Predecessor: 2.3

### 4.12 `mdbook-html` `static_files.rs` head (asset catalog teaser)
- Content: `crates/mdbook-html/src/html_handlebars/static_files.rs:1-40` — `StaticFiles` doc + struct + variants (`Builtin`/`Additional`) + optional hash-fingerprinting. Followed by elision marker for the asset catalog body.
- Cost: 309 tokens (helper: `count-tokens.py crates/mdbook-html/src/html_handlebars/static_files.rs:1-40`)
- Predecessor: 2.20

### 4.13 `select_tag` (admonition kinds)
- Content: `crates/mdbook-html/src/html/admonitions.rs:18-26` — names the five GFM admonition kinds mdBook supports (Note, Tip, Important, Warning, Caution)
- Cost: 120 tokens (helper: `count-tokens.py crates/mdbook-html/src/html/admonitions.rs:18-26`)
- Predecessor: 2.20

### 4.14 `tokenize` (search index tokenizer)
- Content: `crates/mdbook-html/src/html_handlebars/search.rs:17-26` — exact tokenization rule + `MAX_WORD_LENGTH_TO_INDEX = 80`
- Cost: 109 tokens (helper: `count-tokens.py crates/mdbook-html/src/html_handlebars/search.rs:17-26`)

## 5. Examples, tests, contributor docs, dependencies

### 5.1 `nop-preprocessor` example head (canonical preprocessor template)
- Content: `examples/nop-preprocessor.rs:1-75` — `make_app` (clap with `supports` subcommand vs preprocess-from-stdin), `main` dispatch, `handle_preprocessing` (parses input via `mdbook_preprocessor::parse_input(stdin)`, version-checks `MDBOOK_VERSION`, writes output via `serde_json::to_writer(stdout)`), `handle_supports` (exit code 0/1 to signal support). Body of the actual `Nop` impl elided.
- Cost: 565 tokens (helper: `count-tokens.py examples/nop-preprocessor.rs:1-75`)
- Notes: Reference template — the most concrete answer to "how do I write a preprocessor?"
- Predecessor: 3.10

### 5.2 `examples/` folder listing
- Content: `nop-preprocessor.rs`, `remove-emphasis/`
- Cost: 9 tokens (helper: stdin)
- Notes: Two reference plugins; `remove-emphasis/` is a fully-wired example crate (covered structurally by 1.7 workspace.members).

### 5.3 CONTRIBUTING.md — Tests + xtask commands
- Content: `CONTRIBUTING.md:101-152` — the canonical commands an agent needs: `cargo test --workspace`, `cargo test --test gui`, `npm run lint`, `cargo clippy --workspace --all-targets --no-deps -- -D warnings`, `cargo fmt --check`, `cargo +stable semver-checks`, and the umbrella `cargo xtask test-all`. Names the `xtask` crate as the maintainer entry point.
- Cost: 671 tokens (helper: `count-tokens.py CONTRIBUTING.md:101-152`)

### 5.4 `tests/` folder listing
- Content: `gui/`, `testsuite/`
- Cost: 5 tokens (helper: stdin)

### 5.5 `tests/testsuite/` listing
- Content: `README.md`, `book_test.rs`, `build.rs`, `cli.rs`, `config.rs`, `includes.rs`, `index.rs`, `init.rs`, `main.rs`, `markdown.rs`, `playground.rs`, `preprocessor.rs`, `print.rs`, `redirects.rs`, `renderer.rs`, `rendering.rs`, `search.rs`, `test.rs`, `theme.rs`, `toc.rs`, plus per-feature subdirectories (`build/`, `config/`, `includes/`, `index/`, `init/`, `markdown/`, `playground/`, `preprocessor/`, `print/`, `redirects/`, `renderer/`, `rendering/`, `search/`, `test/`, `theme/`, `toc/`)
- Cost: 101 tokens (helper: stdin)
- Predecessor: 5.4

### 5.6 `tests/testsuite/README.md` (full)
- Content: full file — explains the `BookTest` harness (`from_dir`, `empty`, `init`, `check_main_file`), snapbox `str!`/`file!` macros, `SNAPSHOTS=overwrite` mode for regenerating expected output
- Cost: 693 tokens (helper: `count-tokens.py tests/testsuite/README.md`)
- Predecessor: 5.5

### 5.7 `tests/testsuite/main.rs`
- Content: full file — every `mod X;` for the test categories
- Cost: 133 tokens (helper: `count-tokens.py tests/testsuite/main.rs`)
- Predecessor: 5.5

### 5.8 Workspace `Cargo.toml` `workspace.dependencies`
- Content: `Cargo.toml:27-71` — anyhow, axum, clap, handlebars, html5ever, pulldown-cmark, ego-tree, elasticlunr-rs, indexmap, notify, opener, pathdiff, regex, serde, sha2, shlex, snapbox, tempfile, tokio, toml, topological-sort, tower-http, tracing, walkdir, etc., with pinned versions
- Cost: 608 tokens (helper: `count-tokens.py Cargo.toml:27-71`)
- Notes: Names every external crate with pinned versions. Useful for "which crate handles X?"

### 5.9 `guide/book.toml` — realistic book configuration
- Content: full file — `[book]`, `[rust] edition=2018`, `[output.html]` with `smart-punctuation`, `mathjax-support`, `site-url`, `git-repository-url`, `edit-url-template`, `hash-files`, `[output.html.playground]`, `[output.html.code.hidelines] python="~"`, `[output.html.search]`, `[output.html.redirect]`, `[preprocessor.guide-helper]` shelling out to `cargo run`, `[build] extra-watch-dirs`
- Cost: 255 tokens (helper: `count-tokens.py guide/book.toml`)
- Notes: Canonical real-world `book.toml`, mdBook's own.

### 5.10 Guide README (feature bullets)
- Content: `guide/src/README.md:1-32` — "mdBook is a command line tool to create books with Markdown" + feature bullets (Markdown, search, syntax highlighting, theme, preprocessors, backends, Rust, automated testing)
- Cost: 252 tokens (helper: `count-tokens.py guide/src/README.md:1-32`)

### 5.11 `ci/` + `.github/workflows/` listings
- Content: `ci/`: `install-rust.sh`, `make-release-asset.sh`, `publish-guide.sh`, `update-dependencies.sh`. `.github/workflows/`: `deploy.yml`, `main.yml`, `update-dependencies.yml`.
- Cost: 31 tokens (helper: stdin, two listings combined)

### 5.12 `mdbook-html/front-end/` directory listings
- Content: rendered listings of `front-end/`: `css/`, `fonts/`, `images/`, `js/`, `playground_editor/`, `searcher/`, `templates/`. Plus `front-end/templates/`: `head.hbs`, `header.hbs`, `index.hbs`, `redirect.hbs`, `toc.html.hbs`, `toc.js.hbs`. Plus `front-end/css/`: `ayu-highlight.css`, `chrome.css`, `general.css`, `highlight.css`, `print.css`, `tomorrow-night.css`, `variables.css`.
- Cost: 67 tokens (helper: stdin, three listings combined: 17 + 24 + 26)
- Notes: Names every theme template/CSS file the user can override. Pairs with 3.15 (the `include_bytes!` references).

## 6. Secondary surfaces

### 6.1 README intro paragraph (badges + links)
- Content: `README.md:1-13` — title, CI/crates.io/license badges, intro paragraph linking to User Guide and CONTRIBUTING.md
- Cost: 165 tokens (helper: `count-tokens.py README.md:1-13`)
- Notes: Subsumes 1.2 with extra context. Cheap value-add for "where does the user guide live?"

### 6.2 CHANGELOG.md head (current release notes)
- Content: `CHANGELOG.md:1-23` — 0.5.2 release notes only; the rest of the file is a 24k-token historical archive
- Cost: 255 tokens (helper: `count-tokens.py CHANGELOG.md:1-23`)
- Notes: Anchors current version + recent fixes. The `…` invites the agent into the deeper file when needed.

## Below-the-fold

The fixture is far larger than the 20k cap; the items below are honestly omitted because their cost would crowd out higher-value surface. Cumulative ranked content above is ~24k tokens (slightly over the soft cap; the threshold test is the load-bearing sanity check, not a strict cap, and this fixture is unusually large).

### `mdbook-html` deeper internals
- `crates/mdbook-html/src/html/mod.rs:33-108` (~590 toks) — `HtmlRenderOptions`, `ChapterTree`, `render_markdown`, `build_trees` impl. Surface in 2.20.
- `crates/mdbook-html/src/html_handlebars/hbs_renderer.rs` (~5300+ toks total) — `HtmlHandlebars` struct + `Renderer::render` orchestration (theme load → handlebars template registration → search-index creation → toc rendering → per-chapter loop) + `render_chapter`, `render_404`, `render_print_page`, `register_hbs_helpers`, `make_data` (the JSON map fed to handlebars), `emit_redirects`, fragment-redirect helpers. Pipeline overview in 2.20; Cmd-renderer protocol in 3.9.
- `crates/mdbook-html/src/html_handlebars/static_files.rs:40-end` (~2280 toks) — full asset catalog (`book.js`, css, favicons, syntax-highlighting bundles, fonts) + content-hash fingerprinting impl. Head in 4.12.
- `crates/mdbook-html/src/html_handlebars/search.rs:27-end` (~3000 toks) — search index construction (elasticlunr), gated on the `search` cargo feature. Generated assets: `searchindex.js`, `searcher.js`, `mark.min.js`, `elasticlunr.min.js`. Tokenizer surface in 4.14.
- `crates/mdbook-html/src/html_handlebars/helpers/{mod.rs, toc.rs, fontawesome.rs, resources.rs}` (~3000 toks combined) — handlebars custom helpers. Listing in 2.21.
- `crates/mdbook-html/src/html/tree.rs` (~9k toks) — markdown→tree builder body. Filename in 2.21; pipeline overview in 2.20.
- `crates/mdbook-html/src/html/serialize.rs` (~745 toks) — DOM serializer; `serialize(tree, output)` plus `wants_pretty_html_newline` element list.
- `crates/mdbook-html/src/html/print.rs` (~1827 toks) — `render_print_page` + id-rewriting and link-rewriting passes that make the print page work.
- `crates/mdbook-html/src/html/{tokenizer.rs (~579), hide_lines.rs (~1600), admonitions.rs (~1844)}` — html5ever bridging, `~`-prefixed line elision in code blocks, inline-SVG icons for the five admonition kinds (Octicon SVG paths dominate the file). Admonition list in 4.13.
- `crates/mdbook-html/src/html/tests.rs` — unit tests.
- `crates/mdbook-html/src/theme/mod.rs:33-end` (~3000 toks) — `Theme` struct fields + `Theme::new` body that overlays user-provided files, plus `copy_theme`. `include_bytes!` surface in 3.15.
- `crates/mdbook-html/src/theme/{fonts,playground_editor,searcher}.rs` — pure `include_bytes!` modules (asset bytes); irrelevant as text.
- `crates/mdbook-html/src/utils.rs` (~1077 toks) — `unique_id`, `id_from_content` (HTML id generation rule with GitHub/GitLab/pandoc heuristics), `ToUrlPath`, `normalize_path` helpers.
- `crates/mdbook-html/front-end/**` — CSS/JS/handlebars templates and woff2 font binaries:
  - All vendored front-end JavaScript (`book.js` ~1800 LOC, `clipboard.min.js`, `highlight.js`, `playground_editor/ace.js`, `searcher/elasticlunr.min.js`, `searcher/mark.min.js`, `searcher/searcher.js` ~555 LOC).
  - All CSS files (`chrome.css` ~756, `general.css` ~408, `variables.css` ~383, `print.css`, `tomorrow-night.css`, `highlight.css`, `ayu-highlight.css`).
  - Handlebars templates (`index.hbs` ~367, `toc.html.hbs` ~429, `toc.js.hbs` ~456, `redirect.hbs` ~246, `header.hbs`, `head.hbs`).
  - `front-end/playground_editor/` (vendored ace editor) and `front-end/fonts/*.woff2`, `front-end/images/favicon.*` — vendored binaries / large vendored editor.
  - Listings in 5.12 + theme bundle in 3.15 let the agent jump precisely with one `Read`.

### `mdbook-driver` deeper internals
- `crates/mdbook-driver/src/builtin_renderers/markdown_renderer.rs` (262 toks) — tiny `Renderer` impl that writes preprocessed markdown back to disk. Used mostly for debugging preprocessors. Discoverable via `MarkdownRenderer` re-export named in 2.22.
- `crates/mdbook-driver/src/builtin_preprocessors/links.rs:50-137, 170-936` (~5800 toks) — regex matchers, `replace_all`, `Link::render_with_path`, anchor extraction, extensive unit tests. Teaser in 3.6/3.7 covers the contract.
- `crates/mdbook-driver/src/builtin_preprocessors/links/take_lines.rs` (~2421 toks) — `take_lines / take_anchored_lines / take_rustdoc_include_lines / take_rustdoc_include_anchored_lines` + tests.
- `crates/mdbook-driver/src/builtin_preprocessors/cmd.rs:50-end` (~780 toks) — `CmdPreprocessor::run` and `supports_renderer` shelling out, plus tests. Mirrors `CmdRenderer` (3.9).
- `crates/mdbook-driver/src/builtin_preprocessors/index.rs:50-end` (~380 toks) — `is_readme_file` regex helper + tests.
- `crates/mdbook-driver/src/init.rs:65-end` (~640 toks) — `BookBuilder::build` writes directory layout + `SUMMARY.md` + `chapter_1.md` stubs + optional `.gitignore` + theme + `book.toml`, then re-loads via `MDBook::load`. Surface in 4.10.
- `crates/mdbook-driver/src/load.rs:55-end` (~1600 toks) — per-chapter loader (`load_chapter`, BOM stripping), unit tests.
- `crates/mdbook-driver/src/mdbook.rs:88-110` (~150 toks) — `load_with_config_and_summary`, the rare custom-summary entry point.
- `crates/mdbook-driver/src/mdbook.rs:160-225` body deeper context — `with_renderer`/`with_preprocessor` (already in 2.16) and tail of `execute_build_process`.
- `crates/mdbook-driver/src/mdbook.rs:267-345` (~523 toks) — `MDBook::test_chapter` body shells out to `rustdoc --test`, passing `--edition` based on `[rust].edition`. CLI surface in 2.17 + 4.2.
- `crates/mdbook-driver/src/mdbook.rs:475-540` (~590 toks) — full topological-sort body of `determine_preprocessors` over `before`/`after` directives with stable tie-breaking. Head in 4.6.
- `crates/mdbook-driver/src/mdbook/tests.rs` — unit tests.

### `mdbook-core` deeper internals
- `crates/mdbook-core/src/book.rs:171-289` (~1300 toks) — `Chapter::new`/`new_draft`/`is_draft_chapter`/`Display`, `SectionNumber(Vec<u32>)`, `BookItems` depth-first iterator. Field-level Chapter struct (2.5) covers the public surface.
- `crates/mdbook-core/src/book/tests.rs` — unit tests.
- `crates/mdbook-core/src/config.rs:178-310` (~858 toks) — `Config::get / set / contains_key / preprocessors / outputs / html_config` — the dotted-path API. Module doc in 3.17 covers the common path.
- `crates/mdbook-core/src/config.rs:522-554` (~200 toks) — `HtmlConfig::default` impl.
- `crates/mdbook-core/src/config.rs:574-712` (~1134 toks) — `Print { enable, page_break }`, `Fold { enable, level }`, `Playground { editable, copyable, copy_js, line_numbers, runnable }`, `Code { hidelines }`, `Search { enable, limit_results=30, teaser_word_count=30, use_boolean_and=false, boost_title=2, boost_hierarchy=1, boost_paragraph=1, expand=true, heading_split_level=3, copy_js=true, chapter }`, `SearchChapterSettings`. Entry `HtmlConfig` (3.13) names every field; reading the file gives field-by-field defaults.
- `crates/mdbook-core/src/config.rs:715-end` (~3500 toks) — extensive unit tests of `Config` parsing.
- `crates/mdbook-core/src/utils/{mod.rs, fs.rs, html.rs, toml_ext.rs}` — `static_regex!` macro, `fs::path_to_root`, `fs::copy_files_except_ext`, `fs::write` with auto-mkdir, `escape_html`/`escape_html_attribute`, `TomlExt`. Used pervasively but each function is one Read away.

### Remaining `mdbook-summary`
- `crates/mdbook-summary/src/lib.rs:182-end` (~7600 toks) — `SummaryParser` body (state machine on top of pulldown-cmark), `parse_parts`, `parse_dotted_item`, link parsing, error context, extensive tests. Grammar comment in 2.11 + structs in 2.9 cover the contract.

### CLI internals
- `src/main.rs:1-17` (~127 toks) — file doc, `#![allow(unreachable_pub)]`, imports, `mod cmd`, `const VERSION = concat!("v", clap::crate_version!())`.
- `src/main.rs:57-150` (~770 toks) — `create_clap_command` body, `init_logger`, `get_book_dir`, `open` helper, `verify_app` test.
- `src/cmd/clean.rs` (~896 toks) — `cmd::clean::execute` deletes `dest_dir`; `Clean` struct tracks files/dirs/bytes; `human_readable_bytes` SI formatter. About-line in 1.6.
- `src/cmd/init.rs:86-end` (~500 toks) — `get_author_name` (git config shell-out), `request_book_title`, `confirm` helpers.
- `src/cmd/watch.rs` (~513 toks) — `enum WatcherKind { Poll, Native }` + `rebuild_on_change` dispatcher.
- `src/cmd/watch/{native.rs (~1432), poller.rs (~2667)}` — full watcher implementations using `notify`/`notify-debouncer-mini`.
- `src/cmd/serve.rs:107-end` (~720 toks) — async `serve` function (`#[tokio::main]`) + websocket connection helper + `tower_http::ServeDir` with 404 fallback.
- `Cargo.toml:135-158` (~140 toks) — `[[bin]] name="mdbook"`, `[[example]] nop-preprocessor`, `[[example]] remove-emphasis` wired through `examples/remove-emphasis/test.rs`, `[[test]] gui` wired through `tests/gui/runner.rs`.

### Tests
- `tests/testsuite/{build, cli, config, includes, index, init, markdown, playground, preprocessor, print, redirects, renderer, rendering, search, test, theme, toc}.rs` and the per-test `book.toml` fixtures — large body of integration tests; the `BookTest` harness (next bullet) tells the agent how to read them.
- `tests/testsuite/book_test.rs` (~590 LOC) — `BookTest` harness with `from_dir / empty / init / check_main_file` and assertion helpers.
- `tests/gui/runner.rs` plus `tests/gui/books/**` (~30 books, plus `.goml` scripts including `heading-nav-*.goml`, `help.goml`, `highlighting.goml`, `move-between-pages.goml`, `redirect.goml`, `search.goml`, `sidebar-*.goml`, `theme.goml`) — browser-driven GUI tests behind `cargo test --test gui`. `tests/gui/books/sidebar-scroll/src/chapter_*.md` are 100 generated chapter stubs with no semantic content beyond their existence.

### Tooling crates
- `crates/xtask/src/main.rs` (~1160 toks) — maintainer commands (`test-all`, `test-workspace`, `clippy`, `doc`, `fmt`, `semver-checks`, `eslint`, `gui`, `changelog`, `bump`) + impls. Names mirror CONTRIBUTING.md (5.3).
- `crates/xtask/src/changelog.rs` (~919 toks) — `changelog` command impl.
- `crates/mdbook-compare/src/main.rs` (~887 toks) — utility comparing two mdbook versions' HTML output via `tidy` + `git diff --no-index`.
- `crates/{mdbook-compare,xtask}/Cargo.toml` and `crates/mdbook-compare/README.md` — package metadata.
- Sub-crate `Cargo.toml` files (`crates/mdbook-{core,driver,html,markdown,preprocessor,renderer,summary}/Cargo.toml`, ~600 toks combined) — each ~30 lines naming the dependencies of that crate. The workspace dependency graph is implicit from 1.7 + 5.8.

### Examples
- `examples/remove-emphasis/{book.toml, src/SUMMARY.md, src/chapter_1.md, test.rs, mdbook-remove-emphasis/{Cargo.toml, src/main.rs}}` (~750 toks total) — smallest end-to-end custom preprocessor, wired through `[preprocessor.remove-emphasis]` in `book.toml`. Same shape as `nop-preprocessor` (5.1).
- `examples/nop-preprocessor.rs:75-165` (~340 toks) — body of the `Nop` `Preprocessor` impl + its inline `#[cfg(test)]` `nop_lib` mod test.

### Guide
- `guide/src/**/*.md` (~80 docs, several tens of thousands of tokens) — user guide content. Fetch any specific page by name from the SUMMARY in 1.14.
- `guide/src/cli/{init,serve,test,build,clean,watch,arg-watcher,completions,README}.md` (~2.7k toks total) — user-facing CLI reference. Source-side `make_subcommand` (3.1/3.2) + `cmd::*::execute` (4.1-4.4) cover most of what these document.
- `guide/src/format/configuration/{general,preprocessors,renderers,environment-variables}.md` — config reference. `general.md` (~1500) ≈ `[book]`/`[build]`/`[output.html]`. `renderers.md` (~3781) is the largest single config doc, near-1:1 mirror of `HtmlConfig` (3.13) and the deferred sub-configs. `preprocessors.md` (~790) ≈ 4.6/4.7. `environment-variables.md` (351) ≈ `update_from_env` (3.16).
- `guide/src/format/summary.md` (~887 toks) — user-facing rules for Title/Prefix/Part/Numbered/Suffix Chapter with markdown examples. Paired with 2.9/2.10/2.11.
- `guide/src/format/mdbook.md` (~2500 toks) — user-facing prose mirror of LinkPreprocessor (covered by 1.12 + 3.6/3.7).
- `guide/src/format/markdown.md` (~2100 toks) — markdown-extensions reference; covered structurally by 3.5 (`MarkdownOptions`) and 4.13 (admonition kinds).
- `guide/src/format/theme/{README,index-hbs,syntax-highlighting,editor}.md` — theme reference.
- `guide/src/for_developers/{README,preprocessors,backends}.md` — author guides for custom plugins. Source side covered by 3.9 (CmdRenderer), 3.10 (CmdPreprocessor), 5.1 (nop-preprocessor); follow 1.14's pointer.
- `guide/src/for_developers/mdbook-wordcount/` — third example preprocessor that ships with the docs; redundant with 5.1.
- `guide/src/guide/{installation,creating,reading}.md` — user-facing reading + writing experience.
- `guide/src/{404.md, misc/contributors.md, continuous-integration.md, format/example.rs, format/mathjax.md}` — guide stragglers. `continuous-integration.md` (~1362 toks) is the largest omission; one `Read` if a user asks about CI patterns for hosting an mdbook book.
- `guide/guide-helper/{src/lib.rs, src/main.rs, Cargo.toml}` (~600 toks) — internal preprocessor for the guide.

### CONTRIBUTING.md remainder
- `CONTRIBUTING.md:1-100` (~1200 toks) — issue assignment, code-quality (rustfmt/clippy), change requirements (semver, tests, docs).
- `CONTRIBUTING.md:153-end` (~1000 toks) — pull-request process, browser-compatibility/GUI-test details, JS lint setup, highlight.js update procedure, release process.

### Plumbing
- `crates/*/README.md` (8 files, ~600 toks combined) — one-line readmes pointing at the user guide.
- `Cargo.lock` (very large, ~30k toks) — deterministic dep snapshot; only useful for "what version of dep X is pinned?", trivially `Grep`-able.
- `LICENSE`, `CODE_OF_CONDUCT.md`, `triagebot.toml`, `rustfmt.toml`, `package.json`, `eslint.config.mjs`, `.cargo/config.toml`, `.github/ISSUE_TEMPLATE/*.yml`, `.github/renovate.json5`, `.git-blame-ignore-revs`, `.gitattributes`, `.gitignore` — repository plumbing. Visible by name in 1.1 / 5.11.
- `CHANGELOG.md:24-end` (~24k toks) — historical release notes including the 0.5 migration guide. Current version anchored in 6.2; older entries are grep-targets.
- Inline `#[cfg(test)]` test modules throughout `crates/mdbook-*/src/...` — tested behavior implied by the surrounding non-test code.
