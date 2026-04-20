# mdbook — North Star

Revision pin: `b8c90970`

mdBook is a CLI + library for turning a directory of Markdown files (organized by a `SUMMARY.md`) into a static HTML book. The same `mdbook` binary also runs `rustdoc --test` over Rust code blocks (`mdbook test`), serves the book over HTTP with live reload (`mdbook serve`), and watches for changes (`mdbook watch`). The project is a Cargo workspace: a thin `mdbook` binary in `src/` dispatches to a `mdbook-driver` facade that composes `mdbook-summary` (parser for `SUMMARY.md`), `mdbook-core` (config + book model), `mdbook-markdown` (pulldown-cmark wrapper), `mdbook-html` (Handlebars HTML renderer with bundled theme + search index), and the public extension surfaces `mdbook-preprocessor` and `mdbook-renderer`. The `guide/` directory is itself an mdbook book and serves as both the user manual and the largest end-to-end fixture.

## Batches

### 1.1 Fixture root directory listing
- Content: top-level entries of `tests/fixtures/mdbook/`: `CHANGELOG.md`, `CODE_OF_CONDUCT.md`, `CONTRIBUTING.md`, `Cargo.lock`, `Cargo.toml`, `LICENSE`, `README.md`, `ci`, `crates`, `eslint.config.mjs`, `examples`, `guide`, `package.json`, `rustfmt.toml`, `src`, `tests`, `triagebot.toml`.
- Cost: 60 tokens (helper: `ls -1 tests/fixtures/mdbook | count-tokens.py --stdin`)

### 1.2 README one-liner
- Content: `README.md:7` — "mdBook is a utility to create modern online books from Markdown files."
- Cost: 14 tokens (helper: `count-tokens.py README.md:7`)

### 1.3 `crates/` workspace member listing
- Content: `ls -1 crates/`: `mdbook-compare`, `mdbook-core`, `mdbook-driver`, `mdbook-html`, `mdbook-markdown`, `mdbook-preprocessor`, `mdbook-renderer`, `mdbook-summary`, `xtask`.
- Cost: 39 tokens (helper: `ls -1 crates/ | count-tokens.py --stdin`)

### 1.4 `src/` and `src/cmd/` listings
- Content: `src/` (`cmd/`, `main.rs`) plus `src/cmd/` (`build.rs`, `clean.rs`, `command_prelude.rs`, `init.rs`, `mod.rs`, `serve.rs`, `test.rs`, `watch/`, `watch.rs`).
- Cost: 36 tokens (helper: `ls -1 src/ src/cmd/ | count-tokens.py --stdin`)
- Notes: Names every CLI subcommand file without reading any.

### 1.5 `mdbook-html/src/` module roster
- Content: `crates/mdbook-html/src/lib.rs` full — `mod html; mod html_handlebars; pub mod theme; pub(crate) mod utils; pub use html_handlebars::HtmlHandlebars;`.
- Cost: 34 tokens (helper: `count-tokens.py crates/mdbook-html/src/lib.rs`)

### 2.1 `mdbook-driver` module declarations
- Content: `crates/mdbook-driver/src/lib.rs:67-79` — `pub mod builtin_preprocessors; pub mod builtin_renderers; pub mod init; mod load; mod mdbook;` plus `pub use mdbook::MDBook;` and re-exports of `book`, `config`, `errors`.
- Cost: 83 tokens (helper: `count-tokens.py crates/mdbook-driver/src/lib.rs:67-79`)

### 2.2 Crate-name → one-line description map
- Content: rendered list mapping each workspace crate to its `Cargo.toml` `description`: mdbook-core "base support library", mdbook-driver "high-level library for running mdBook", mdbook-html "mdBook HTML renderer", mdbook-markdown "Markdown processing used in mdBook", mdbook-renderer "Library to assist implementing an mdbook renderer", mdbook-preprocessor "Library to assist implementing an mdbook preprocessor", mdbook-summary "Summary parser for mdBook", mdbook-compare "Utility to compare the output of two different versions of mdbook", xtask "local dev-task runner".
- Cost: ~90 tokens (helper: `echo "<list>" | count-tokens.py --stdin`)

### 2.3 Subcommand → about-line map
- Content: rendered text of every `Command::new("X").about("...")` from `src/cmd/{init,build,clean,serve,test,watch}.rs` plus `completions` from `src/main.rs`. Lines like `init: Creates the boilerplate structure and files for a new book` etc.
- Cost: ~90 tokens (helper: rendered as single stdin text)
- Notes: Entire CLI surface in one tiny batch.

### 2.4 `mdbook-core` lib.rs
- Content: `crates/mdbook-core/src/lib.rs` full — declares `MDBOOK_VERSION`, `pub mod book; pub mod config; pub mod utils;`, `errors` re-exports anyhow.
- Cost: 96 tokens (helper: `count-tokens.py crates/mdbook-core/src/lib.rs`)

### 2.5 `src/cmd/mod.rs`
- Content: `src/cmd/mod.rs` full — `pub mod build; pub mod clean; ...` with `cfg(feature = "serve")` and `cfg(feature = "watch")` gates.
- Cost: 57 tokens (helper: `count-tokens.py src/cmd/mod.rs`)

### 2.6 `src/main.rs` header
- Content: `src/main.rs:1-16` — file doc, `#![allow(unreachable_pub)]`, imports, `mod cmd;`, `const VERSION = concat!("v", clap::crate_version!())`.
- Cost: 100 tokens (helper: `count-tokens.py src/main.rs:1-16`)

### 2.7 Built-in preprocessors + renderers module rosters
- Content: `crates/mdbook-driver/src/builtin_preprocessors/mod.rs` (re-exports `CmdPreprocessor`, `IndexPreprocessor`, `LinkPreprocessor`) plus `crates/mdbook-driver/src/builtin_renderers/mod.rs:1-15` (module doc + `pub use markdown_renderer::MarkdownRenderer;` + `CmdRenderer` struct intro).
- Cost: ~140 tokens (helper: `count-tokens.py crates/mdbook-driver/src/builtin_preprocessors/mod.rs crates/mdbook-driver/src/builtin_renderers/mod.rs:1-15`)

### 2.8 `Renderer` trait
- Content: `crates/mdbook-renderer/src/lib.rs:25-35` — `pub trait Renderer { fn name(&self) -> &str; fn render(&self, ctx: &RenderContext) -> Result<()>; }`.
- Cost: 108 tokens (helper: `count-tokens.py crates/mdbook-renderer/src/lib.rs:25-35`)

### 2.9 `Preprocessor` trait
- Content: `crates/mdbook-preprocessor/src/lib.rs:30-45` — `pub trait Preprocessor { fn name; fn run(&self, &PreprocessorContext, Book) -> Result<Book>; fn supports_renderer(&self, &str) -> Result<bool>; }`.
- Cost: 133 tokens (helper: `count-tokens.py crates/mdbook-preprocessor/src/lib.rs:30-45`)

### 2.10 `Book` struct declaration
- Content: `crates/mdbook-core/src/book.rs:12-29` — module doc + `pub struct Book { pub items: Vec<BookItem> }`. Followed by elision marker.
- Cost: 149 tokens (helper: `count-tokens.py crates/mdbook-core/src/book.rs:12-29`)

### 2.11 `BookItem` enum
- Content: `crates/mdbook-core/src/book.rs:113-127` — `pub enum BookItem { Chapter(Chapter), Separator, PartTitle(String) }`. Elision marker before the larger Chapter struct.
- Cost: 92 tokens (helper: `count-tokens.py crates/mdbook-core/src/book.rs:113-127`)
- Predecessor: 2.10

### 2.12 `Config` struct declaration
- Content: `crates/mdbook-core/src/config.rs:60-78` — `pub struct Config { pub book: BookConfig, pub build: BuildConfig, pub rust: RustConfig, output: Value, preprocessor: Value }`.
- Cost: 152 tokens (helper: `count-tokens.py crates/mdbook-core/src/config.rs:60-78`)

### 2.13 `MDBook` struct declaration
- Content: `crates/mdbook-driver/src/mdbook.rs:28-44` — `pub struct MDBook { pub root: PathBuf, pub config: Config, pub book: Book, renderers: IndexMap<String, Box<dyn Renderer>>, preprocessors: IndexMap<String, Box<dyn Preprocessor>> }`.
- Cost: 116 tokens (helper: `count-tokens.py crates/mdbook-driver/src/mdbook.rs:28-44`)

### 2.14 README intro
- Content: `README.md:1-13` — title, badges, intro paragraph linking to `User Guide` and `CONTRIBUTING.md`.
- Cost: 165 tokens (helper: `count-tokens.py README.md:1-13`)

### 3.1 `mdbook-html/src/{html,html_handlebars,theme}/` listings
- Content: rendered listings of the three subdirs (admonitions.rs, hide_lines.rs, mod.rs, print.rs, serialize.rs, tokenizer.rs, tree.rs / hbs_renderer.rs, helpers/, search.rs, static_files.rs / fonts.rs, mod.rs, playground_editor.rs, searcher.rs).
- Cost: 128 tokens (helper: `ls -1 crates/mdbook-html/src/html crates/mdbook-html/src/html_handlebars crates/mdbook-html/src/theme | count-tokens.py --stdin`)

### 3.2 `tests/testsuite/` listing
- Content: `ls -1 tests/testsuite/` — feature dirs and `.rs` files (build, cli, config, includes, index, init, markdown, playground, preprocessor, print, redirects, renderer, rendering, search, test, theme, toc; plus `book_test.rs`, `main.rs`, `README.md`).
- Cost: 103 tokens (helper: `ls -1 tests/testsuite | count-tokens.py --stdin`)

### 3.3 `tests/gui/` listing
- Content: `ls -1 tests/gui/` — `runner.rs`, `books/`, and 17 `.goml` browser-test scripts (heading-nav-*, sidebar*, search, help, highlighting, move-between-pages, redirect, theme).
- Cost: 138 tokens (helper: `ls -1 tests/gui | count-tokens.py --stdin`)

### 3.4 `guide/src/` and subdirectory listings
- Content: `ls -1 guide/src guide/src/cli guide/src/format guide/src/format/configuration guide/src/format/theme guide/src/guide guide/src/for_developers` rendered as one block.
- Cost: ~165 tokens (helper: concatenated subdir listings)

### 3.5 `guide/src/SUMMARY.md` — front half
- Content: `guide/src/SUMMARY.md:1-22` — Introduction, User guide (Installation/Reading/Creating), Reference guide → CLI subcommands, start of Format section.
- Cost: 147 tokens (helper: `count-tokens.py guide/src/SUMMARY.md:1-22`)

### 3.6 `guide/src/SUMMARY.md` — back half
- Content: `guide/src/SUMMARY.md:23-44` — Format (Configuration, Theme, MathJax, Markdown), Continuous integration, For developers (Preprocessors, Backends), Contributors.
- Cost: 217 tokens (helper: `count-tokens.py guide/src/SUMMARY.md:23-44`)
- Predecessor: 3.5
- Notes: Together with 3.5, this is the most complete feature-map of mdbook in the entire fixture.

### 3.7 `src/main.rs` subcommand-dispatch match
- Content: `src/main.rs:18-55` — `main()` body with the clap `match` over `init`/`build`/`clean`/`watch`/`serve`/`test`/`completions`.
- Cost: 291 tokens (helper: `count-tokens.py src/main.rs:18-55`)

### 3.8 `src/main.rs` create_clap_command
- Content: `src/main.rs:57-91` — clap builder attaching every subcommand, post-help text, cfg-gated `watch`/`serve` attachment.
- Cost: 313 tokens (helper: `count-tokens.py src/main.rs:57-91`)

### 3.9 `RenderContext` struct + crate doc
- Content: `crates/mdbook-renderer/src/lib.rs:1-22` (crate doc) + `crates/mdbook-renderer/src/lib.rs:38-89` (`RenderContext { version, root, book, config, destination, chapter_titles }` + `RenderContext::new`/`source_dir`/`from_json`).
- Cost: ~600 tokens (helper: `count-tokens.py crates/mdbook-renderer/src/lib.rs:1-22 crates/mdbook-renderer/src/lib.rs:38-89`)
- Predecessor: 2.8

### 3.10 `PreprocessorContext` struct + crate doc
- Content: `crates/mdbook-preprocessor/src/lib.rs:1-22` (crate doc) + `crates/mdbook-preprocessor/src/lib.rs:48-84` (`PreprocessorContext { root, config, renderer, mdbook_version, chapter_titles }`, `parse_input(stdin) -> (PreprocessorContext, Book)`).
- Cost: ~570 tokens (helper: `count-tokens.py crates/mdbook-preprocessor/src/lib.rs:1-22 crates/mdbook-preprocessor/src/lib.rs:48-84`)
- Predecessor: 2.9

### 3.11 `Chapter` struct fields
- Content: `crates/mdbook-core/src/book.rs:134-170` — `Chapter` fields (`name`, `content`, `number`, `sub_items`, `path`, `source_path`, `parent_names`) with the doc comments explaining the README→`index.md` rewrite and draft-chapter semantics.
- Cost: 362 tokens (helper: `count-tokens.py crates/mdbook-core/src/book.rs:134-170`)
- Predecessor: 2.11

### 3.12 `BookConfig` + `TextDirection`
- Content: `crates/mdbook-core/src/config.rs:313-388` — `BookConfig { title, authors, description, src, language, text_direction }` with defaults, plus `TextDirection` enum and `from_lang_code` (RTL language list: ar, he, fa, ku, ur, ps, yi, ...).
- Cost: 658 tokens (helper: `count-tokens.py crates/mdbook-core/src/config.rs:313-388`)
- Predecessor: 2.12

### 3.13 `BuildConfig` + `RustConfig` + `RustEdition`
- Content: `crates/mdbook-core/src/config.rs:390-440` — `BuildConfig { build_dir="book", create_missing=true, use_default_preprocessors=true, extra_watch_dirs }`, `RustConfig { edition }`, `RustEdition { E2024, E2021, E2018, E2015 }`.
- Cost: ~400 tokens (helper: `count-tokens.py crates/mdbook-core/src/config.rs:390-440`)
- Predecessor: 2.12

### 3.14 Workspace Cargo.toml — members + package metadata
- Content: `Cargo.toml:1-26` — `[workspace]` members, `[workspace.lints.{clippy,rust}]` (warns on `missing_docs`, `unreachable_pub`, `rust_2018_idioms`), `[workspace.package]` with edition 2024, MPL-2.0, rust-version 1.88.0.
- Cost: 198 tokens (helper: `count-tokens.py Cargo.toml:1-26`)

### 4.1 mdbook-driver crate-level prose
- Content: `crates/mdbook-driver/src/lib.rs:1-66` — module doc explaining "high-level library for running mdBook", lists every sibling crate, the `search` Cargo feature, and two complete `MDBook::init`/`MDBook::load`+`build()` examples.
- Cost: 524 tokens (helper: `count-tokens.py crates/mdbook-driver/src/lib.rs:1-66`)

### 4.2 mdbook binary Cargo.toml — package + deps
- Content: `Cargo.toml:72-104` — `[package] name="mdbook", version="0.5.2"`, authors, `[dependencies]` listing `clap`, `clap_complete`, internal `mdbook-*` crates, `opener`, `toml`, `tracing`.
- Cost: 245 tokens (helper: `count-tokens.py Cargo.toml:72-104`)

### 4.3 mdbook binary optional deps + features
- Content: `Cargo.toml:105-134` — optional `notify`/`notify-debouncer-mini`/`ignore`/`pathdiff`/`walkdir` (watch), `axum`/`futures-util`/`tokio`/`tower-http` (serve), `[dev-dependencies]`, `[features] default = ["watch", "serve", "search"]`.
- Cost: 295 tokens (helper: `count-tokens.py Cargo.toml:105-134`)

### 4.4 `Book` impl methods
- Content: `crates/mdbook-core/src/book.rs:31-110` — `Book::new`, `new_with_items`, `iter()`, `chapters()` (filters draft chapters), `for_each_mut`, `for_each_chapter_mut`, `push_item`, plus the recursive `for_each_mut` helper.
- Cost: ~430 tokens (helper: `count-tokens.py crates/mdbook-core/src/book.rs:31-110`)
- Predecessor: 2.10

### 4.5 `mdbook-summary` crate doc + `Summary`
- Content: `crates/mdbook-summary/src/lib.rs:1-78` — crate doc explaining prefix/numbered/suffix chapter rules, `parse_summary` signature, and `Summary { title, prefix_chapters, numbered_chapters, suffix_chapters }`.
- Cost: 580 tokens (helper: `count-tokens.py crates/mdbook-summary/src/lib.rs:1-78`)

### 4.6 `Link` + `SummaryItem`
- Content: `crates/mdbook-summary/src/lib.rs:80-145` — `Link { name, location, number, nested_items }`, `Link::new`, `Default`, `SummaryItem { Link, Separator, PartTitle }`.
- Cost: ~410 tokens (helper: `count-tokens.py crates/mdbook-summary/src/lib.rs:80-145`)
- Predecessor: 4.5

### 4.7 `mdbook-markdown` crate (full)
- Content: `crates/mdbook-markdown/src/lib.rs` (61 lines) — re-exports `pulldown_cmark`, `MarkdownOptions { smart_punctuation, definition_lists, admonitions }`, `new_cmark_parser` enabling TABLES/FOOTNOTES/STRIKETHROUGH/TASKLISTS/HEADING_ATTRIBUTES + optional SMART_PUNCTUATION/DEFINITION_LIST/GFM.
- Cost: 452 tokens (helper: `count-tokens.py crates/mdbook-markdown/src/lib.rs`)

### 4.8 HTML pipeline overview + module roster
- Content: `crates/mdbook-html/src/html/mod.rs:1-32` — pipeline doc ("1. pulldown_cmark events → 2. tree::MarkdownTreeBuilder → 3. serialize"), submodule declarations, `pub(crate) use` re-exports.
- Cost: 256 tokens (helper: `count-tokens.py crates/mdbook-html/src/html/mod.rs:1-32`)

### 4.9 `LinkPreprocessor` summary
- Content: `crates/mdbook-driver/src/builtin_preprocessors/links.rs:18-43` — doc listing every supported helper (`{{# include}}`, `{{# rustdoc_include}}`, `{{# playground}}`, `{{# title}}`), `LinkPreprocessor` struct, `NAME = "links"`, `new()`.
- Cost: 234 tokens (helper: `count-tokens.py crates/mdbook-driver/src/builtin_preprocessors/links.rs:18-43`)

### 4.10 `IndexPreprocessor` (full)
- Content: `crates/mdbook-driver/src/builtin_preprocessors/index.rs:1-50` — converts README files to `index.md` via `is_readme_file` regex, warns on conflict.
- Cost: 354 tokens (helper: `count-tokens.py crates/mdbook-driver/src/builtin_preprocessors/index.rs:1-50`)

### 4.11 `CmdRenderer` (full)
- Content: `crates/mdbook-driver/src/builtin_renderers/mod.rs` (full 88 lines) — module doc, `CmdRenderer { name, cmd }`, `Renderer::render` impl that spawns the configured command and pipes the JSON `RenderContext` to its stdin (the alternative-backend protocol).
- Cost: ~500 tokens (helper: `count-tokens.py crates/mdbook-driver/src/builtin_renderers/mod.rs`)
- Predecessor: 2.7

### 4.12 `MDBook::load` + `init` + `iter`
- Content: `crates/mdbook-driver/src/mdbook.rs:46-110` — `load`, `load_with_config` (calls `load_book` on `<book_root>/SUMMARY.md` + `determine_renderers` + `determine_preprocessors`), `load_with_config_and_summary`, `iter`.
- Cost: 437 tokens (helper: `count-tokens.py crates/mdbook-driver/src/mdbook.rs:46-110`)
- Predecessor: 2.13

### 4.13 `MDBook::build` + `preprocess_book` + `execute_build_process`
- Content: `crates/mdbook-driver/src/mdbook.rs:160-225` — `init()` doc with ASCII directory layout, `build()`, `preprocess_book()` running each `Preprocessor` in order, `execute_build_process()`, `with_renderer`, `with_preprocessor`.
- Cost: 521 tokens (helper: `count-tokens.py crates/mdbook-driver/src/mdbook.rs:160-225`)
- Predecessor: 4.12

### 4.14 Guide: README (feature bullets)
- Content: `guide/src/README.md:1-32` — "mdBook is a command line tool to create books with Markdown" + feature bullets (Markdown, search, syntax highlighting, theme, preprocessors, backends, Rust, automated testing).
- Cost: 252 tokens (helper: `count-tokens.py guide/src/README.md:1-32`)

### 4.15 Guide: CLI/Format/Configuration READMEs
- Content: `guide/src/cli/README.md` (206 — CLI overview with one-liner per subcommand) + `guide/src/format/README.md` (42) + `guide/src/format/configuration/README.md` (116).
- Cost: 364 tokens (helper: `count-tokens.py guide/src/cli/README.md guide/src/format/README.md guide/src/format/configuration/README.md`)

### 5.1 `guide/book.toml` — realistic book configuration
- Content: `guide/book.toml` — `[book]`, `[rust] edition=2018`, `[output.html]` with `smart-punctuation`, `mathjax-support`, `site-url`, `git-repository-url`, `edit-url-template`, `hash-files`, `[output.html.playground]`, `[output.html.code.hidelines] python="~"`, `[output.html.search]`, `[output.html.redirect]`, `[preprocessor.guide-helper]` shelling out to `cargo run`, `[build] extra-watch-dirs`.
- Cost: 255 tokens (helper: `count-tokens.py guide/book.toml`)

### 5.2 `cmd::build` + `cmd::test` (full)
- Content: `src/cmd/build.rs` (full 37 lines: load `MDBook`, `build()`, `--open` opens `index.html`) + `src/cmd/test.rs` (full 53 lines: `--chapter`, `--library-path`, calls `book.test_chapter`/`book.test`).
- Cost: 568 tokens (helper: `count-tokens.py src/cmd/build.rs src/cmd/test.rs`)
- Predecessor: 2.5

### 5.3 `cmd::command_prelude` (full)
- Content: `src/cmd/command_prelude.rs` — the `CommandExt` trait shared by subcommands: `arg_dest_dir` (`-d/--dest-dir`), `arg_root_dir` (positional `[dir]`), `arg_open` (`-o/--open`), `arg_watcher` (`--watcher poll|native`); plus `set_dest_dir` updating `BookConfig.build.build_dir`.
- Cost: 474 tokens (helper: `count-tokens.py src/cmd/command_prelude.rs`)

### 5.4 `cmd::init` clap surface
- Content: `src/cmd/init.rs:11-30` — clap subcommand: `[dir]` positional, `--theme`, `--force`, `--title <title>`, `--ignore none|git`. Body in BTF.
- Cost: 178 tokens (helper: `count-tokens.py src/cmd/init.rs:11-30`)
- Predecessor: 2.5

### 5.5 `cmd::serve` clap surface + reload constant
- Content: `src/cmd/serve.rs:1-50` — imports, `LIVE_RELOAD_ENDPOINT = "__livereload"`, clap subcommand: `--hostname=localhost`, `--port=3000`, plus `arg_open`/`arg_watcher`.
- Cost: 382 tokens (helper: `count-tokens.py src/cmd/serve.rs:1-50`)
- Predecessor: 2.5

### 5.6 `Config::from_disk` + `update_from_env`
- Content: `crates/mdbook-core/src/config.rs:111-178` — `from_disk` (calls `fs::read_to_string` + `Config::from_str`), `update_from_env` with `MDBOOK_*__*` rules (env-var key → kebab-case dotted path; double underscore = dot; single underscore = dash; JSON-or-string parsing).
- Cost: 649 tokens (helper: `count-tokens.py crates/mdbook-core/src/config.rs:111-178`)
- Predecessor: 2.12

### 5.7 `HtmlConfig` struct (every field)
- Content: `crates/mdbook-core/src/config.rs:441-553` — every field of `HtmlConfig`: `theme`, `default_theme`, `preferred_dark_theme`, `smart_punctuation`, `definition_lists`, `admonitions`, `mathjax_support`, `additional_css`, `additional_js`, `fold`, `playground`, `code`, `print`, `no_section_label`, `search`, `git_repository_url`, `git_repository_icon`, `input_404`, `site_url`, `cname`, `edit_url_template`, `live_reload_endpoint`, `redirect`, `hash_files`, `sidebar_header_nav` with doc comments and `Default` impl.
- Cost: 1021 tokens (helper: `count-tokens.py crates/mdbook-core/src/config.rs:441-553`)
- Predecessor: 2.12

### 5.8 `determine_renderers`
- Content: `crates/mdbook-driver/src/mdbook.rs:401-440` — `OutputConfig` deserialize, mapping `[output.X]` → `HtmlHandlebars` (X="html") / `MarkdownRenderer` (X="markdown") / `CmdRenderer` (else, command default `mdbook-X`); HTML default fallback when no `[output.*]` section. `determine_preprocessors` body in BTF.
- Cost: 332 tokens (helper: `count-tokens.py crates/mdbook-driver/src/mdbook.rs:401-440`)
- Predecessor: 4.13

### 5.9 `compose_command` + `handle_command_error`
- Content: `crates/mdbook-driver/src/lib.rs:81-131` — `compose_command` shlex-splits a config command string and resolves single-component executables via PATH vs relative paths under book root; `handle_command_error` softens missing-command errors when `optional = true`.
- Cost: 369 tokens (helper: `count-tokens.py crates/mdbook-driver/src/lib.rs:81-131`)

### 5.10 `load_book` flow head
- Content: `crates/mdbook-driver/src/load.rs:1-55` — `load_book` reads `SUMMARY.md`, conditionally calls `create_missing` (stub-creates absent chapter files with `# {title}\n`), then `load_book_from_disk`. Elision marker for the per-chapter loader past line 55.
- Cost: 418 tokens (helper: `count-tokens.py crates/mdbook-driver/src/load.rs:1-55`)
- Predecessor: 4.12

### 5.11 `BookBuilder` head + builder methods
- Content: `crates/mdbook-driver/src/init.rs:11-65` — `BookBuilder { root, create_gitignore, config, copy_theme }`, `new`/`with_config`/`copy_theme`/`create_gitignore` builder methods.
- Cost: 414 tokens (helper: `count-tokens.py crates/mdbook-driver/src/init.rs:11-65`)
- Predecessor: 4.1

### 5.12 `CmdPreprocessor` head + `write_input`
- Content: `crates/mdbook-driver/src/builtin_preprocessors/cmd.rs:1-50` — `CmdPreprocessor { name, cmd, root, optional }`, `new`, `write_input` that serializes `(ctx, book)` as JSON to the child preprocessor's stdin.
- Cost: 374 tokens (helper: `count-tokens.py crates/mdbook-driver/src/builtin_preprocessors/cmd.rs:1-50`)
- Predecessor: 2.9

### 5.13 `nop-preprocessor` example head (canonical preprocessor template)
- Content: `examples/nop-preprocessor.rs:1-75` — module setup, `make_app`, `main` dispatch over `supports` subcommand vs preprocess-from-stdin, `handle_preprocessing` (parses input via `mdbook_preprocessor::parse_input(stdin)`, version-checks against `MDBOOK_VERSION`, writes output via `serde_json::to_writer(stdout)`), `handle_supports` (exit code 0/1 to signal support). Body of the actual `Nop` impl elided.
- Cost: 565 tokens (helper: `count-tokens.py examples/nop-preprocessor.rs:1-75`)
- Predecessor: 5.12
- Notes: Most concrete answer to "how do I write a preprocessor?"

### 5.14 Guide: General configuration intro + example
- Content: `guide/src/format/configuration/general.md:1-30` — example `book.toml` block showing `[book]`, `[rust]`, `[build]`, `[preprocessor.index]`, `[preprocessor.links]`, `[output.html]`, `[output.html.search]`.
- Cost: 126 tokens (helper: `count-tokens.py guide/src/format/configuration/general.md:1-30`)

### 5.15 Guide: General-metadata config reference
- Content: `guide/src/format/configuration/general.md:31-60` — field-by-field reference for `[book]` (title, authors, description, src, language, text-direction) with the example `book.toml` block. `[build]`/`[output.*]` field reference is in BTF (rest of `general.md` plus the dedicated `renderers.md`/`preprocessors.md` pages).
- Cost: 317 tokens (helper: `count-tokens.py guide/src/format/configuration/general.md:31-60`)
- Predecessor: 5.14

### 5.16 Guide: SUMMARY.md format
- Content: `guide/src/format/summary.md:1-50` — user-facing rules for Title, Prefix Chapter, Part Title, Numbered Chapter with markdown examples.
- Cost: 466 tokens (helper: `count-tokens.py guide/src/format/summary.md:1-50`)

### 6.1 Guide: reference page heads (orientation tease)
- Content: 15-line heads of the largest guide pages, each ending with an elision marker so the agent knows the body exists at that exact path:
  - `guide/src/format/configuration/renderers.md:1-15` (149) — `[output.html]` table reference intro
  - `guide/src/for_developers/preprocessors.md:1-15` (133) — custom-preprocessor protocol intro
  - `guide/src/for_developers/backends.md:1-15` (129) — alternative-backend (`output.X` shell-out) protocol intro
  - `guide/src/format/mdbook.md:1-15` (167) — `{{#include}}`/`{{#playground}}` syntax intro
  - `guide/src/format/markdown.md:1-15` (156) — markdown-dialect reference intro
  - `guide/src/format/theme/README.md:1-15` (145) — theme (HTML/CSS/JS) customization intro
- Cost: ~880 tokens (helper: `count-tokens.py <those six specs>`)
- Notes: Each tease has a `…` marker. Without these the agent doesn't know which guide page documents which feature; with them, one `Read` lands the user-facing detail.

### 6.2 `mdbook-html` static files + theme bundle
- Content: `crates/mdbook-html/src/html_handlebars/static_files.rs:1-40` (`StaticFiles` with `Builtin`/`Additional` variants, optional hash-fingerprinting) + `crates/mdbook-html/src/theme/mod.rs:14-32` (the bundled `include_bytes!` static-asset list: `index.hbs`, `head.hbs`, `redirect.hbs`, `header.hbs`, `toc.{js,html}.hbs`, `chrome.css`, `general.css`, `print.css`, `variables.css`, `book.js`, `highlight.js`, `highlight.css`, `tomorrow-night.css`, `ayu-highlight.css`, `clipboard.min.js`, favicon).
- Cost: ~700 tokens (helper: `count-tokens.py crates/mdbook-html/src/html_handlebars/static_files.rs:1-40 crates/mdbook-html/src/theme/mod.rs:14-32`)

### 6.3 `mdbook-html/front-end/` directory listings
- Content: rendered listings of `crates/mdbook-html/front-end/{templates,css,js,searcher,playground_editor,fonts,images}/`.
- Cost: ~210 tokens (helper: concatenated `ls -1 ... | count-tokens.py --stdin`)

### 6.4 Testsuite README + module list
- Content: `tests/testsuite/main.rs` (133 — every `mod X;` for the test categories) + `tests/testsuite/README.md` (693 — how tests are written: `BookTest::from_dir(...)`, snapbox `str!`/`file!` macros, `SNAPSHOTS=overwrite` mode for regenerating expected output).
- Cost: ~826 tokens (helper: `count-tokens.py tests/testsuite/main.rs tests/testsuite/README.md`)
- Predecessor: 3.2

## Below-the-fold

- **`Cargo.lock`** — deterministic dep snapshot, 30k+ tokens, no semantic value beyond `Cargo.toml`.
- **Bodies of `crates/mdbook-html/src/html/*.rs` past their lib.rs heads** — `tree.rs` (1155 lines, ≈8k tokens — markdown-tree builder + transformations), `admonitions.rs` (1844 tokens — note/tip/important/warning/caution GitHub octicon SVGs), `print.rs` (215 lines), `serialize.rs` (112 lines), `tokenizer.rs`, `hide_lines.rs`. Filenames in 3.1; the pipeline overview in 4.8 + one `Read` lands precisely.
- **Body of `crates/mdbook-html/src/html_handlebars/hbs_renderer.rs`** (696 lines) — `HtmlHandlebars` chapter-render mechanics. The type is named in `mdbook-html/src/lib.rs` (1.5) and instantiated in `determine_renderers` (5.8).
- **Bodies of `crates/mdbook-html/src/html_handlebars/{search.rs (445 lines), static_files.rs:40- (320 lines)}`** — internal mechanics. The static-files / theme-bundle batch (6.2) covers the head; one `Read` lands the body.
- **`crates/mdbook-html/src/theme/mod.rs:33-333`** — theme-loading logic that overlays user theme files over bundled defaults. Surface map in 6.2.
- **Body of `crates/mdbook-html/src/html/mod.rs:33-108`** — `HtmlRenderOptions { markdown_options, path, edition, config }`, `render_markdown`, `ChapterTree { chapter, html_path, tree }`, `build_trees`. Pipeline overview in 4.8 + one `Read`.
- **Bodies of `src/cmd/watch/{native,poller}.rs`** — `notify`/`notify-debouncer-mini` watcher impls + `src/cmd/watch.rs` itself (`WatcherKind::{Poll, Native}` + `rebuild_on_change`). Listed in 1.4.
- **All vendored front-end JavaScript (`book.js` 843 lines, `clipboard.min.js`, `highlight.js`, `ace.js`, `mode-rust.js`, `theme-{dawn,tomorrow_night}.js`, `elasticlunr.min.js`, `mark.min.js`, `searcher.js` 555 lines)** — listing in 6.3.
- **All CSS files (`chrome.css` 756, `general.css` 408, `variables.css` 383, `print.css`, `tomorrow-night.css`, `highlight.css`, `ayu-highlight.css`)** — style sheets. Listing in 6.3 + theme guide head (6.1) lets a styling question land precisely with one `Read`.
- **`crates/mdbook-html/front-end/templates/*.hbs`** (`index.hbs` 367, `toc.html.hbs` 429, `toc.js.hbs` 456, `redirect.hbs` 246, `header.hbs`, `head.hbs`) — Handlebars templates. Filenames in 6.3; `index-hbs.md` (in guide) describes the contract.
- **`front-end/playground_editor/` (vendored ace editor) and `front-end/fonts/*.woff2`, `images/favicon.*`** — vendored binaries / large vendored editor.
- **Body of `crates/mdbook-driver/src/builtin_preprocessors/links.rs:44-936` and `links/take_lines.rs` (253 lines)** — include-helper resolution + line-range slicing details. The user-facing `mdbook.md` (teased in 6.1) covers semantics; `LinkPreprocessor::run` is one `Read` away once the file is named in 4.9.
- **Body of `crates/mdbook-driver/src/builtin_preprocessors/cmd.rs:50-150`** — `CmdPreprocessor::run` and `supports_renderer` shelling out. Head in 5.12 + `nop-preprocessor` (5.13) covers the protocol from the writing side.
- **Body of `crates/mdbook-driver/src/init.rs:65-157`** — `BookBuilder::build` writes directory layout + `SUMMARY.md` + `chapter_1.md` stubs + optional `.gitignore` + theme + `book.toml`, then re-loads via `MDBook::load`. Surface in 5.11.
- **`src/cmd/init.rs:32-122`** — `cmd::init::execute` body: interactive `confirm()` for theme + gitignore prompts, `git config --get user.name` for author, then `BookBuilder::with_config(...).build()`. Clap surface in 5.4 + `BookBuilder` in 5.11 + its body above covers the flow.
- **`src/cmd/serve.rs:50-152`** — `cmd::serve::execute` body: sets `output.html.live-reload-endpoint` + `output.html.site-url="/"`, builds book, spawns axum `Router` with WebSocket handler + `tokio::sync::broadcast` for reload, `tower_http::ServeDir` with 404 fallback, integrates with `watch::rebuild_on_change`. Clap surface + reload endpoint constant in 5.5.
- **`src/cmd/clean.rs`** — `cmd::clean::execute` deletes `dest_dir`; `Clean` struct tracks files/dirs/bytes; `human_readable_bytes` SI formatter. About-line in 2.3 + filename in 1.4.
- **`src/cmd/watch.rs` + `src/cmd/watch/{native,poller}.rs`** — `WatcherKind::{Poll, Native}`, `rebuild_on_change` dispatch, `notify`/`notify-debouncer-mini` watcher impls. Listed in 1.4; `serve` integration named in 5.5.
- **`crates/mdbook-driver/src/mdbook.rs:227-345`** — `MDBook::test`/`test_chapter` rustdoc loop: writes preprocessed chapter to a tempdir, invokes `rustdoc --test --edition <RustEdition>`, fails on any non-zero exit. CLI surface in 5.2; `MDBook` struct in 2.13.
- **`crates/mdbook-driver/src/mdbook.rs:441-541`** — `determine_preprocessors`: `PreprocessorConfig` deserialize, `DEFAULT_PREPROCESSORS = ["links", "index"]`, `topological_sort` over `before`/`after` directives with stable tie-breaking, plus `preprocessor_should_run` consulting `preprocessor.X.renderers`. Pair with `determine_renderers` (5.8).
- **`crates/mdbook-core/src/config.rs:178-310`** — `Config::get`/`contains_key`/`set` accessors and `html_config()` helper. The struct (2.12) + `from_disk`/`update_from_env` (5.6) + `HtmlConfig` (5.7) cover the common path.
- **`crates/mdbook-core/src/config.rs:574-712`** — sub-config structs (`Print`, `Fold`, `Playground`, `Code`, `Search`, `SearchChapterSettings`). Entry `HtmlConfig` (5.7) names every field; reading the file gives the field-by-field defaults.
- **`crates/mdbook-summary/src/lib.rs:146-1201`** — recursive-descent `SummaryParser` (BNF grammar comment + state machine). The data model (4.5/4.6) plus the user-facing format guide (5.16) cover the relevant semantics; parser internals are one `Read` away.
- **`crates/mdbook-core/src/book.rs:171-289`** — `Chapter::new`/`new_draft`/`is_draft_chapter`/`Display`, `SectionNumber(Vec<u32>)` and `BookItems` depth-first iterator. Field-level Chapter struct (3.11) covers the public surface; impl details are one `Read` away.
- **`tests/testsuite/book_test.rs`** (590 lines) — `BookTest` test harness implementation (`from_dir`, `empty`, `init`, `check_main_file`, `check_all_main_files`, etc.). Testsuite intro in 6.4 names the file; one `Read` lands.
- **Bodies of `tests/testsuite/*.rs` past `book_test.rs`** — feature-by-feature test files (preprocessor.rs 491, config.rs 344, etc.). Listing (3.2) + testsuite README (6.4) lets the agent jump.
- **All `tests/testsuite/*/` test-fixture book directories** — dozens of small books used as inputs. Pointers via 3.2 suffice.
- **`tests/gui/books/`** — fixture books for the GUI tests. Listing (3.3) names the `.goml` scripts.
- **`crates/mdbook-core/src/utils/{fs.rs, html.rs, mod.rs, toml_ext.rs}`** — internal helpers (`read_to_string`, `write` with auto-mkdir, `path_to_root`, `escape_html`/`escape_html_attribute`, `static_regex!` macro, `TomlExt`). One `Read` per file.
- **`crates/xtask/src/{main.rs, changelog.rs}`** — local dev-task runner (`cargo xtask test-all`/`clippy`/`doc`/`fmt`/`semver-checks`/`eslint`/`gui`/`changelog`/`bump`).
- **`crates/mdbook-compare/src/main.rs`** — utility comparing two mdbook versions' HTML output via `tidy` + `git diff --no-index`.
- **`guide/guide-helper/` crate** — tiny preprocessor used only by the guide's own build.
- **`examples/nop-preprocessor.rs:75-165`** — body of the `Nop` preprocessor `Preprocessor` impl + its inline `#[cfg(test)]`. Head in 5.13.
- **`examples/remove-emphasis/`** — second example preprocessor; `nop-preprocessor` (5.13) is canonical.
- **`.github/workflows/{main,deploy,update-dependencies}.yml`, `ci/*.sh`, `.github/ISSUE_TEMPLATE/*.yml`, `triagebot.toml`, `eslint.config.mjs`, `package.json`, `rustfmt.toml`, `.cargo/config.toml`, `.git-blame-ignore-revs`, `.gitattributes`, `.gitignore`, `CODE_OF_CONDUCT.md`, `LICENSE`** — build-infra/config ancillaries; surfaced via 1.1.
- **`CHANGELOG.md` body** — 24k tokens; per-version detail recoverable with one `Read`.
- **`CONTRIBUTING.md` body** — labels and contributor onboarding info; one `Read` away.
- **`guide/src/cli/{init,build,clean,serve,test,watch,completions,arg-watcher}.md` bodies** — per-command CLI reference pages (~2.7k total tokens). The CLI README (4.15) one-liners + the `src/cmd/*.rs` files (named in 1.4, partially covered in 5.2/5.3/5.4/5.5) cover most of what these pages document.
- **`guide/src/format/configuration/{preprocessors,renderers,environment-variables}.md` bodies** — per-table reference (preprocessors 792, renderers 3781, env-vars 351). Renderers head teased in 6.1; preprocessors head teased in 6.1; env-vars covered by `update_from_env` source (5.6).
- **`guide/src/format/configuration/general.md:61-174`** — `[build]`/`[output.html]` field reference past general metadata. Intro + general-metadata in 5.14/5.15; full `HtmlConfig` source field reference in 5.7.
- **`guide/src/format/{markdown.md, mdbook.md, mathjax.md}` bodies** — markdown-dialect + `{{#include}}` reference. Heads teased in 6.1 + the link-preprocessor declaration (4.9) cover what features exist; `Read` for syntax detail.
- **`guide/src/format/theme/{README,index-hbs,syntax-highlighting,editor}.md` bodies** — theme reference. Head teased in 6.1.
- **`guide/src/for_developers/{README,preprocessors,backends}.md` bodies** — author guides for custom plugins. Heads in 6.1; the canonical example is in 5.13 (`nop-preprocessor.rs`); the source side is 4.11 (`CmdRenderer`) and 5.12 (`CmdPreprocessor`).
- **`guide/src/guide/{installation,creating,reading}.md` bodies** — user-facing reading + writing experience. Filenames in 3.4 listing.
- **`guide/src/README.md:33-59`** — contributing/license boilerplate; the feature bullets are in 4.14.
- **`guide/src/{404.md, misc/contributors.md, continuous-integration.md, format/example.rs, format/configuration/README.md beyond intro, for_developers/README.md beyond intro}`** — guide stragglers. `continuous-integration.md` (1362 tokens) is the largest omission; if a user asks about CI patterns for hosting an mdbook book, `Read` is one hop.
- **Inline `#[cfg(test)]` test modules** in `crates/mdbook-{core,driver}/src/...` — tested behavior implied by the surrounding non-test code.
- **`crates/*/README.md` files** — short crates.io README shells; per-crate descriptions in 2.2 capture the load-bearing content.
