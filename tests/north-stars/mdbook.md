# mdbook — North Star

Revision pin: `b8c90970`

mdBook is a Rust workspace that compiles a directory of Markdown files into
an HTML book (or other formats via plug-in renderers/preprocessors). The
top-level binary `mdbook` exposes subcommands (`init`, `build`, `watch`,
`serve`, `test`, `clean`, `completions`); the heavy lifting lives in a fan
of `mdbook-*` library crates under `crates/`. A user guide lives under
`guide/` (which itself is built with mdBook), and an integration testsuite
under `tests/testsuite/` driven by `BookTest` + snapbox.

## Batches

### 1.1 `crates/` folder listing
- Content: `ls -F crates/` rendered as a list:
  ```
  mdbook-compare/
  mdbook-core/
  mdbook-driver/
  mdbook-html/
  mdbook-markdown/
  mdbook-preprocessor/
  mdbook-renderer/
  mdbook-summary/
  xtask/
  ```
- Cost: 39 tokens (helper: `ls -F crates/ | count-tokens.py --stdin`)
- Notes: highest-leverage orienting batch — names every cooperating library
  crate at once, before any internals are shown.

### 1.2 Repository top-level entries
- Content: `ls -F` of the fixture root, rendered as one entry per line:
  ```
  CHANGELOG.md
  CODE_OF_CONDUCT.md
  CONTRIBUTING.md
  Cargo.lock
  Cargo.toml
  LICENSE
  README.md
  ci/
  crates/
  eslint.config.mjs
  examples/
  guide/
  package.json
  rustfmt.toml
  src/
  tests/
  triagebot.toml
  ```
- Cost: 60 tokens (helper: `ls -F /tests/fixtures/mdbook | count-tokens.py --stdin`)
- Notes: every other batch is a refinement of an entry here.

### 1.3 `src/cmd/` folder listing
- Content: `ls -F src/cmd/`:
  ```
  build.rs
  clean.rs
  command_prelude.rs
  init.rs
  mod.rs
  serve.rs
  test.rs
  watch/
  watch.rs
  ```
- Cost: 29 tokens

### 1.4 README prose
- Content: `README.md:7-10` (the four sentences "mdBook is a utility to
  create modern online books from Markdown files. … User Guide also serves
  as a demonstration to showcase what a book looks like."). Followed by `…`.
- Cost: 50 tokens
- Notes: README badges and licence boilerplate are below-the-fold.

### 1.5 `src/cmd/mod.rs` (subcommand declarations)
- Content: `src/cmd/mod.rs` (entire 11-line file: `pub mod build/clean/
  command_prelude/init/serve/test/watch` with `serve`/`watch` gated on
  cargo features).
- Cost: 57 tokens

### 1.6 `mdbook-core` crate root
- Content: `crates/mdbook-core/src/lib.rs` (entire 16-line file: declares
  `pub mod book/config/utils`, `pub mod errors { pub use anyhow::{Error,
  Result}; }`, and the `MDBOOK_VERSION` constant).
- Cost: 96 tokens

### 1.7 `MDBook` struct
- Content: `crates/mdbook-driver/src/mdbook.rs:28-44` (the high-level
  orchestrator struct: `pub root: PathBuf, pub config: Config, pub book:
  Book, renderers: IndexMap<String, Box<dyn Renderer>>, preprocessors:
  IndexMap<String, Box<dyn Preprocessor>>`).
- Cost: 116 tokens
- Notes: this is the primary public type of the library API — orient the
  agent around it before diving into methods.

### 1.8 Workspace members + workspace.package
- Content: `Cargo.toml:1-26` (the `[workspace]` `members = [...]` list —
  root, `crates/*`, the example preprocessor, `guide/guide-helper` — and
  `[workspace.package]` with edition 2024, MPL-2.0 licence, MSRV 1.88.0).
- Cost: 198 tokens

### 1.9 Top-level subcommand dispatch in `main.rs`
- Content: `src/main.rs:14-55` (`mod cmd;` plus the `match command.
  get_matches().subcommand()` block enumerating every subcommand and its
  `cmd::X::execute(sub_matches)` target, including the inline `completions`
  handler).
- Cost: 310 tokens
- Predecessor: 1.5
- Notes: the catastrophic-omission mitigator for "what subcommands does
  `mdbook` have, and where do they live?" — agent gets a single-hop answer
  for any of them.

### 2.1 `mdbook-driver` crate-level rustdoc (what each sister crate is for)
- Content: `crates/mdbook-driver/src/lib.rs:1-66` (the long crate doc that
  names every other `mdbook-*` crate and what each one provides:
  preprocessor trait, renderer trait, markdown parser, summary parser, HTML
  renderer, internal core; plus the `MDBook::init` and `MDBook::load`
  rustdoc examples). Followed by `…`.
- Cost: 524 tokens
- Notes: single highest-value semantic batch in the repo — gives the agent
  the full crate ontology before any other code is read.

### 2.2 Guide tree (what lives in `guide/src/`)
- Content: rendered tree of `guide/src/`:
  ```
  guide/src/
    README.md (introduction)
    404.md
    SUMMARY.md
    continuous-integration.md
    cli/                  README.md, build.md, clean.md, completions.md,
                          init.md, serve.md, test.md, watch.md, arg-watcher.md
    format/
      README.md
      configuration/      general.md, preprocessors.md, renderers.md,
                          environment-variables.md
      markdown.md
      mathjax.md
      mdbook.md           (mdBook-specific Markdown features:
                           include/playground/etc.)
      summary.md          (SUMMARY.md format)
      theme/              README.md, index-hbs.md, syntax-highlighting.md,
                          editor.md
      example.rs  images/
    for_developers/
      README.md
      backends.md         (writing a custom renderer)
      preprocessors.md    (writing a custom preprocessor)
      mdbook-wordcount/   (worked example preprocessor source)
    guide/
      README.md
      creating.md
      installation.md
      reading.md
    misc/contributors.md
  ```
- Cost: 219 tokens
- Notes: the user guide *is* mdBook's primary user-facing documentation;
  every per-page batch under §7 references this listing.

### 2.3 `tests/testsuite/` listing + main module
- Content: rendered tree of `tests/testsuite/` (one `*.rs` driver per
  feature area: build, cli, config, includes, index, init, markdown,
  playground, preprocessor, print, redirects, renderer, rendering, search,
  test, theme, toc; plus the `book_test.rs` shared driver and matching
  test-data subdirectories) **plus** `tests/testsuite/main.rs` (entire
  30-line module-list).
- Cost: 129 + 133 = 262 tokens

### 2.4 `mdbook-markdown` and `mdbook-html` crate roots
- Content: `crates/mdbook-markdown/src/lib.rs:1-12` (crate doc + `pub use
  pulldown_cmark`) combined with `crates/mdbook-html/src/lib.rs` (entire
  6-line file: `mod html; mod html_handlebars; pub mod theme; pub(crate)
  mod utils; pub use html_handlebars::HtmlHandlebars;`).
- Cost: 102 + 34 = 136 tokens
- Notes: makes "where is the HTML renderer entry point?" trivially
  discoverable as `mdbook_html::HtmlHandlebars`.

### 2.5 `mdbook-summary` crate-level rustdoc
- Content: `crates/mdbook-summary/src/lib.rs:1-12` (the `parse_summary`
  module-level documentation pointing at the SUMMARY.md format spec, plus
  the public re-export of `SectionNumber`).
- Cost: 119 tokens

### 3.1 `Preprocessor` trait + `PreprocessorContext`
- Content: `crates/mdbook-preprocessor/src/lib.rs:1-22` (crate doc + the
  `pub use` re-exports of `MDBOOK_VERSION`, `book`, `config`, `errors`)
  combined with `:23-66` (the `Preprocessor` trait — `name` / `run` /
  `supports_renderer` — and the `PreprocessorContext` struct fields
  `root` / `config` / `renderer` / `mdbook_version` / `chapter_titles`).
- Cost: 180 + 394 = 574 tokens
- Notes: trait surface that any third-party preprocessor binary must
  implement against. Catastrophic omission would send the agent guessing.

### 3.2 `Renderer` trait + `RenderContext`
- Content: `crates/mdbook-renderer/src/lib.rs:1-22` (crate doc + re-exports)
  combined with `:22-90` (the `Renderer` trait — `name` + `render` — and
  the `RenderContext` struct fields `version` / `root` / `book` / `config`
  / `destination` / `chapter_titles` and its `new` / `source_dir` /
  `from_json` methods).
- Cost: 174 + 568 = 742 tokens

### 3.3 `Book` / `BookItem` / `Chapter` / `SectionNumber` declarations
- Content: `crates/mdbook-core/src/book.rs:1-28` (module doc + `pub struct
  Book { pub items: Vec<BookItem> }` with non-exhaustive markers) combined
  with `:113-170` (the `BookItem` enum — `Chapter(Chapter) | Separator |
  PartTitle(String)` — and the `Chapter` struct with all its public fields:
  `name`, `content`, `number`, `sub_items`, `path`, `source_path`,
  `parent_names`, complete with the rustdoc warning about README→index.md
  rewriting and the `source_path` distinction).
- Cost: 208 + 483 = 691 tokens
- Notes: this is *the* central data model; every preprocessor and renderer
  manipulates these types. The README→index.md note prevents a classic
  agent-confusion bug.

### 3.4 `Book` methods
- Content: `crates/mdbook-core/src/book.rs:31-97` (the `impl Book` block:
  `new` / `new_with_items` / `iter` / `chapters` / `for_each_mut` /
  `for_each_chapter_mut` / `push_item`).
- Cost: 463 tokens
- Predecessor: 3.3

### 3.5 `Chapter` methods + `BookItems` iterator
- Content: `crates/mdbook-core/src/book.rs:171-289` (the `impl Chapter` —
  `new` / `new_draft` / `is_draft_chapter` — `Display` impl, `SectionNumber`
  impls, and `BookItems<'a>` depth-first iterator).
- Cost: 740 tokens
- Predecessor: 3.3

### 3.6 `Config` struct shape + `BookConfig` / `BuildConfig` / `RustConfig`
- Content: `crates/mdbook-core/src/config.rs:60-110` (the top-level `Config`
  struct: `book`, `build`, `rust`, `output`, `preprocessor`; plus its
  `Default` / `FromStr` impls with a brief docstring about it being an
  in-memory representation of `book.toml`) combined with `:313-353` (the
  `BookConfig` struct: `title`, `authors`, `description`, `src` (default
  `"src"`), `language` (default `"en"`), `text_direction`) and `:390-444`
  (`BuildConfig`: `build_dir` default `"book"`, `create_missing`,
  `use_default_preprocessors`, `extra_watch_dirs`; `RustConfig::edition`;
  `RustEdition` enum E2024/E2021/E2018/E2015).
- Cost: 334 + 304 + 357 = ~995 tokens
- Notes: covers ~70% of "what can a user put in `book.toml`?" without
  pulling in the giant HtmlConfig.

### 4.1 `MDBook` impl — `load` / `load_with_config` / `load_with_config_and_summary`
- Content: `crates/mdbook-driver/src/mdbook.rs:1-27` (file doc + use
  statements naming every neighbouring type) combined with `:46-110` (the
  three load constructors — `load` reads `book.toml`,
  `load_with_config` skips disk config, `load_with_config_and_summary`
  accepts a pre-parsed summary).
- Cost: 231 + 437 = 668 tokens
- Predecessor: 1.7

### 4.2 `MDBook` impl — `iter` / `init` / `build` / `preprocess_book` / `execute_build_process`
- Content: `crates/mdbook-driver/src/mdbook.rs:112-230` (chapter-iteration
  rustdoc, the `init() -> BookBuilder` pointer, and the build pipeline
  `build` → `execute_build_process` → `preprocess_book` for each
  registered renderer).
- Cost: ~1000 tokens
- Predecessor: 1.7

### 4.3 `Config` impl block — load / get / set / contains_key / html_config
- Content: `crates/mdbook-core/src/config.rs:111-260` (the entire `impl
  Config` block: `from_disk`, `update_from_env` with its detailed env-var
  rules in the docstring, `get` with its dotted-key rules, `contains_key`,
  `preprocessors`, `outputs`, `html_config`, and `set`).
- Cost: ~1320 tokens (640 + 680, helper:
  `count-tokens.py crates/mdbook-core/src/config.rs:111-176` and
  `crates/mdbook-core/src/config.rs:178-260`)
- Predecessor: 3.6

### 4.4 `determine_renderers` + `OutputConfig`
- Content: `crates/mdbook-driver/src/mdbook.rs:395-431` (the
  `OutputConfig` deserialise struct, the function that walks `[output.*]`
  tables and picks built-in `HtmlHandlebars` for `html`, `MarkdownRenderer`
  for `markdown`, otherwise `CmdRenderer` shelling out to `mdbook-{key}`;
  falls back to HTML by default).
- Cost: 309 tokens
- Predecessor: 1.7

### 4.5 `determine_preprocessors` (topological sort) + `DEFAULT_PREPROCESSORS`
- Content: `crates/mdbook-driver/src/mdbook.rs:432-541` (loads
  `[preprocessor.*]` tables, runs them through `topological_sort` honouring
  `before` / `after` deps, picks built-in `LinkPreprocessor` /
  `IndexPreprocessor` for `links` / `index` else `CmdPreprocessor` shelling
  out to `mdbook-{name}`).
- Cost: 908 tokens
- Predecessor: 1.7

### 5.1 `mdbook-driver/src/lib.rs` — `compose_command` + `handle_command_error`
- Content: `crates/mdbook-driver/src/lib.rs:67-132` (the helpers shared by
  command-renderer and command-preprocessor: shlex-parse the `command`
  string, search `PATH` if no path components else relative to book root;
  the standardised "command not found" warning that respects
  `optional = true` in `book.toml`).
- Cost: ~440 tokens

### 5.2 `LinkPreprocessor` doc + struct + `Preprocessor` impl head
- Content: `crates/mdbook-driver/src/builtin_preprocessors/links.rs:1-75`
  (the rustdoc enumerating `{{# include}}` / `{{# rustdoc_include}}` /
  `{{# playground}}` / `{{# title}}` helpers, the `LinkPreprocessor` struct
  with its `NAME = "links"` constant, and the `run` method that walks
  every chapter calling `replace_all`). Followed by `…`.
- Cost: 598 tokens
- Notes: the user-facing **link helpers** that mdBook books rely on —
  agents asked about `{{# include path:line-range}}` should land here in
  one hop.

### 5.3 `IndexPreprocessor` (README → index.md)
- Content: `crates/mdbook-driver/src/builtin_preprocessors/index.rs`
  (entire 106-line file: doc, struct, `NAME = "index"`, the `run` impl
  that rewrites any chapter whose path stem matches `(?i)readme` to
  `index.md`, with the conflict warning when both exist; tests included
  as a small bonus).
- Cost: 737 tokens

### 5.4 `CmdRenderer` (shell-out renderer protocol)
- Content: `crates/mdbook-driver/src/builtin_renderers/mod.rs` (entire
  88-line file: doc pointing at the renderer protocol, struct
  `name` / `cmd`, `Renderer::render` impl that spawns the binary, streams
  the `RenderContext` JSON to its stdin and inherits stdout/stderr, with
  `optional` handling).
- Cost: 615 tokens
- Notes: combined with 3.2 (`Renderer` trait + `RenderContext`), this fully
  specifies how to write a third-party `mdbook-foo` binary.

### 5.5 `Summary` / `SummaryItem` / `Link` types + `parse_summary` doc
- Content: `crates/mdbook-summary/src/lib.rs:14-130` — the `parse_summary`
  fn with its long docstring describing the `SUMMARY.md` syntax (title,
  prefix chapters, part titles, numbered chapters, suffix chapters), plus
  the `Summary`, `Link`, `SummaryItem` data types.
- Cost: 412 + 490 = 902 tokens
- Notes: catastrophic omission if the agent doesn't know the structure of
  `SUMMARY.md`.

### 5.6 Book loader (`load_book` / `load_book_from_disk` / `load_chapter`)
- Content: `crates/mdbook-driver/src/load.rs:1-95` (the pipeline: read
  `src/SUMMARY.md`, `parse_summary`, optionally create missing chapter
  files, then walk and load each chapter's content from disk handling BOM
  and absolute paths; `load_chapter` body included). Followed by `…` (the
  ~170-line tests module is below the fold).
- Cost: 749 tokens
- Predecessor: 5.5

### 5.7 `HtmlHandlebars` renderer overview
- Content: `crates/mdbook-html/src/html/mod.rs:1-12` (the high-level "render
  pipeline: parse → tree → transform → serialize" doc) combined with
  `:33-77` (the `HtmlRenderOptions` struct + `render_markdown` /
  `build_tree` entry points) and `:80-108` (`ChapterTree<'book>` and
  `build_trees`).
- Cost: 132 + 370 + 214 = 716 tokens
- Notes: agent now knows the HTML rendering pipeline shape without
  reading the 696-line `hbs_renderer.rs` body.

### 5.8 `HtmlHandlebars` `Renderer` impl head
- Content: `crates/mdbook-html/src/html_handlebars/hbs_renderer.rs:18-30`
  (struct + `new`) combined with `:303-345` (the `Renderer for
  HtmlHandlebars` impl head: `name() = "html"`, the start of `render` that
  resolves the theme dir, registers handlebars templates). Followed by `…`.
- Cost: 64 + 315 = 379 tokens

### 6.1 Subcommand `make_subcommand` definitions (clap)
- Content: the `make_subcommand` fn at the top of each
  `src/cmd/{build,clean,init,serve,test,watch}.rs` (lines 8-15, 11-17,
  12-29, 23-48, 9-32, 12-19 respectively) **plus** `command_prelude.rs:1-55`
  (the shared `CommandExt` trait providing `arg_dest_dir` /
  `arg_root_dir` / `arg_open` / `arg_watcher` with their full help-text
  strings).
- Cost: 1502 tokens total — 88 + 99 + 241 + 382 + 209 + 115 + 368 (helper:
  `count-tokens.py src/cmd/build.rs:1-15 src/cmd/clean.rs:1-17
  src/cmd/init.rs:1-30 src/cmd/serve.rs:1-50 src/cmd/test.rs:1-33
  src/cmd/watch.rs:1-20 src/cmd/command_prelude.rs:1-55`)
- Predecessor: 1.5

### 6.2 `init` subcommand body
- Content: `src/cmd/init.rs:30-122` (the `execute` fn: theme/--force flow,
  `--ignore git|none`, interactive title prompt via `request_book_title`,
  pulls author from `git config --get user.name`, then calls
  `MDBook::init().build()`).
- Cost: 673 tokens
- Predecessor: 6.1

### 6.3 `serve` subcommand body (live-reload websocket)
- Content: `src/cmd/serve.rs:50-152` (the `execute` fn including the
  livereload endpoint constant, the `update_config` closure that pokes
  `output.html.live-reload-endpoint` and `site-url`, the tokio + axum
  server with `ServeDir` / `ServeFile`, and the websocket handler for
  reload messages).
- Cost: 809 tokens
- Predecessor: 6.1

### 7.1 Guide `SUMMARY.md`
- Content: `guide/src/SUMMARY.md` (entire 44-line file — the user guide's
  own table of contents, which doubles as a sitemap for the docs listed
  in 2.2).
- Cost: 364 tokens
- Predecessor: 2.2

### 7.2 Guide intro
- Content: `guide/src/README.md:20-50` (the headline "mdBook is a command
  line tool to create books with Markdown" plus the bullet feature list:
  search, syntax highlighting, theme files, preprocessors, backends, Rust
  testing). Followed by `…`.
- Cost: 352 tokens

### 7.3 `tests/testsuite/README.md` (test-author handbook)
- Content: `tests/testsuite/README.md` (entire — explains `BookTest`,
  snapbox conventions, `SNAPSHOTS=overwrite`, basic test pattern with a
  worked example).
- Cost: 693 tokens
- Predecessor: 2.3

### 7.4 `BookTest` API surface
- Content: `tests/testsuite/book_test.rs:22-90` (struct definition, the
  `from_dir` / `empty` / `init` constructors that copy a fixture book into
  a temp dir; this is the entry point of every test).
- Cost: 566 tokens
- Predecessor: 7.3

### 7.5 For-developers — preprocessors guide
- Content: `guide/src/for_developers/preprocessors.md` (the prose guide to
  writing a custom preprocessor; pairs with 3.1 (trait) and the
  `CmdPreprocessor` JSON-protocol code below-the-fold for a complete
  picture).
- Cost: 1264 tokens

## Below-the-fold

This section justifies what's *not* in the ranking. Approximate cumulative
ranked content above is ~22k tokens.

### Long-tail Rust source

- `crates/mdbook-driver/src/mdbook.rs:211-345` — `MDBook::with_renderer` /
  `with_preprocessor` / `test` / `test_chapter` (~970 tokens). Niche
  embedded-API surface; struct fields in 1.7 already imply `with_renderer`
  / `with_preprocessor`, and `test_chapter` is one `Read` away.
- `crates/mdbook-driver/src/mdbook.rs:346-393` — `build_dir_for` /
  `source_dir` / `theme_dir` (~340 tokens). Trivial path-joining helpers.
- `crates/mdbook-driver/src/builtin_renderers/markdown_renderer.rs`
  (~260 tokens) — built-in renderer that just dumps preprocessed markdown.
  Useful when debugging preprocessors; one `Read` away.
- `crates/mdbook-driver/src/builtin_preprocessors/cmd.rs` (~570 tokens
  for the head) — `CmdPreprocessor` JSON-over-stdio protocol; mirror of
  `CmdRenderer` (5.4) on the preprocessor side.
- `crates/mdbook-driver/src/init.rs` (`BookBuilder`, ~625 tokens for the
  builder doc + main `build()`) — initialised by `cmd::init` (6.2) and by
  `MDBook::init()` (named in 4.2 doc).
- `crates/mdbook-driver/src/load.rs:96-309` — the ~170-line `tests` module
  for the loader. Public surface is in 5.6.
- `crates/mdbook-driver/src/builtin_preprocessors/links.rs:76-936` — the
  ~860-line link-substitution engine (regex parsing, range parsing,
  anchor resolution, recursion). The trigger surface is in 5.2 and the
  user-facing syntax is documented in `guide/src/format/mdbook.md`.
- `crates/mdbook-driver/src/builtin_preprocessors/links/take_lines.rs` —
  helper for line/anchor slicing inside `{{# include}}`.
- `crates/mdbook-summary/src/lib.rs:131-1201` — the `SummaryParser` impl
  (`parse_affix` / `parse_parts` / `parse_link` / `parse_nested_numbered`
  / etc.), the EBNF grammar comment at line 146, and ~550 lines of unit
  tests. Public surface is in 5.5.
- `crates/mdbook-core/src/config.rs:1-58` — the long module-level
  docstring with a `book.toml` / `Config::from_str` worked example.
  Useful for "show me how to read a config value programmatically", but
  4.3 already gives the API.
- `crates/mdbook-core/src/config.rs:445-712` — the giant `HtmlConfig`
  struct with all 25+ HTML-renderer flags (theme, smart_punctuation,
  fold, playground, code, print, search, redirect, hash_files, etc.) and
  the `Print` / `Fold` / `Playground` / `Code` / `Search` /
  `SearchChapterSettings` sub-configs (~2000 tokens combined). Big,
  HTML-renderer-only; one `Read` away. The user-facing prose is in
  `guide/src/format/configuration/general.md`.
- `crates/mdbook-core/src/config.rs:713-1209` — ~470 lines of `tests`
  module for config parsing/serialisation. Worked examples; not needed
  up front (location obvious from 4.3).
- `crates/mdbook-core/src/utils/{mod.rs, fs.rs, html.rs, toml_ext.rs}` —
  filesystem helpers, `static_regex!` macro, `escape_html`,
  `log_backtrace`. Used pervasively but each call site already gives the
  fully-qualified path; their existence is implied by 1.6.
- `crates/mdbook-core/src/book/tests.rs` and
  `crates/mdbook-driver/src/mdbook/tests.rs` — internal unit-test
  fixtures.
- `crates/mdbook-markdown/src/lib.rs` (entire 62-line file beyond the
  doc captured by 2.4) — `MarkdownOptions` and `new_cmark_parser` listing
  the exact pulldown-cmark options mdBook enables.
- `crates/mdbook-html/src/html_handlebars/hbs_renderer.rs:31-302,
  346-696` — ~5500 tokens of HTML-rendering glue (chapter rendering, 404
  page, print page, redirect emission, helper registration, the bulk of
  `Renderer::render`). Internal implementation detail; precise location
  is given by 5.8.
- `crates/mdbook-html/src/html/tree.rs` (1155 lines, ~7500 tokens) —
  internal pulldown-cmark-events-to-tree builder. Almost never relevant
  to a user query; precise location is given by 5.7.
- `crates/mdbook-html/src/html_handlebars/{search.rs, static_files.rs,
  helpers/}` — search-index build, static asset emission, and TOC /
  font-awesome handlebars helpers.
- `crates/mdbook-html/src/html/{print.rs, serialize.rs, tokenizer.rs,
  hide_lines.rs, admonitions.rs}` — internal HTML transforms. Locations
  discoverable from 5.7.
- `crates/mdbook-html/src/theme/` — the `Theme` struct + bundled-asset
  loaders (`fonts.rs`, `playground_editor.rs`, `searcher.rs`). The asset
  names are listed inside the file at lines 33-62 (one `Read` away from
  knowing 2.4).
- `crates/mdbook-html/src/utils.rs` — small `ToUrlPath` /
  `normalize_path` / id-uniqueness helpers.
- `crates/mdbook-html/front-end/` — bundled handlebars templates, CSS,
  JS, fonts, images. Large, almost never relevant when grepping the
  Rust source.
- `crates/mdbook-compare/src/main.rs` (~110 tokens) — utility diffing
  two `mdbook` binaries' HTML output for a given book.
- `crates/xtask/src/main.rs` and `crates/xtask/src/changelog.rs` —
  local-dev BTreeMap of named commands (`test-all` / `test-workspace` /
  `clippy` / `doc` / `fmt` / `semver-checks` / `eslint` / `gui` /
  `changelog`).

### Long-tail binary source (`src/`)

- `src/main.rs:57-149` — `create_clap_command` body and `init_logger` /
  `get_book_dir` / `open` / `verify_app` helpers. Discoverable from 1.9.
- `src/cmd/build.rs:17-36`, `src/cmd/test.rs:34-52`, `src/cmd/clean.rs:18-114`
  — build/test/clean `execute` bodies. Trivial pass-throughs and a
  byte-counting walker; the clap surface in 6.1 already names every
  flag.
- `src/cmd/watch.rs:21-80` and `src/cmd/watch/{native.rs, poller.rs}` —
  `WatcherKind` enum and the two filesystem-watcher backends (notify vs
  polling). Discoverable from 1.5.
- `src/cmd/serve.rs:108-152` — the `#[tokio::main] async fn serve` body
  and websocket-connection helper (already invoked from 6.3).

### Long-tail Cargo / build / CI

- `Cargo.toml:27-71` (~470 tokens) — `[workspace.dependencies]` table
  pinning every third-party crate version. Useful but rarely
  consulted; one `Read` away.
- `Cargo.toml:72-158` (~635 tokens) — root `[package]`,
  `[dependencies]`, optional `watch` / `serve` / `search` feature gates,
  `[dev-dependencies]`, `[features]` defaults, `[[bin]]` /
  `[[example]]` / `[[test]]` tables. Discoverable from 1.8.
- `Cargo.lock` — generated; never relevant.
- `ci/`, `.github/workflows/`, `eslint.config.mjs`, `package.json`,
  `rustfmt.toml`, `triagebot.toml`, `.cargo/`, `.git*` — build / CI
  plumbing. Locations in 1.2.

### Long-tail tests

- `tests/testsuite/{build, cli, config, includes, index, init, markdown,
  playground, preprocessor, print, redirects, renderer, rendering,
  search, test, theme, toc}.rs` — ~3300 lines of integration-test code.
  Surfaced by 2.3; each individual file is one `Read` away.
- `tests/testsuite/book_test.rs:91-590` — the rest of the `BookTest`
  helper API (build / change_file / check_main_file / spawn / etc.). One
  `Read` away from 7.4.
- `tests/testsuite/{build, cli, config, ...}/` data fixtures — small
  fixture books used by the integration tests.
- `tests/gui/` — Selenium browser-driving code (mostly JS / Rust glue).
- `examples/remove-emphasis/` and `examples/nop-preprocessor.rs`
  (referenced from `Cargo.toml`) — worked third-party preprocessor
  examples. Trait-level guidance is in 3.1.
- `guide/guide-helper/` — a tiny helper crate used by the user guide
  build.

### Long-tail user guide

- `guide/src/cli/{init, watch, serve, test, build, clean, completions,
  arg-watcher}.md` — per-subcommand reference pages (~180-530 tokens
  each, ~2700 total). Locations in 2.2; the same flags are visible in
  the clap definitions in 6.1.
- `guide/src/format/configuration/{general, preprocessors, renderers,
  environment-variables}.md` — user-prose for `book.toml` tables.
  Discoverable from 2.2 and 4.3.
- `guide/src/format/markdown.md`, `mathjax.md`, `mdbook.md` (user-prose
  for `{{# include}}` etc., complementing 5.2), `theme/*.md`,
  `summary.md` (user-prose for SUMMARY.md, complementing 5.5), `README.md`.
  All discoverable from 2.2.
- `guide/src/for_developers/backends.md` (~2700 tokens, the largest doc
  page; covers the renderer-protocol JSON spec — paired with 3.2 (trait)
  and 5.4 (`CmdRenderer`) which already convey the protocol shape) plus
  `README.md` and `mdbook-wordcount/`. Discoverable from 2.2.
- `guide/src/guide/{installation, reading, creating}.md`,
  `guide/src/continuous-integration.md`, `guide/src/misc/contributors.md`,
  `guide/src/404.md` — additional user-facing prose docs. Locations in
  2.2.

### Long-tail repository-level docs

- `CHANGELOG.md` (~3000 tokens of release-by-release notes). Pulled in
  only when a user query specifically asks about a version.
- `LICENSE` (MPL-2.0 boilerplate), `CODE_OF_CONDUCT.md`,
  `CONTRIBUTING.md` (~2900 tokens of contributor process). Locations in
  1.2.
- `crates/*/README.md` — each crate's tiny README is a one-line pointer
  to docs.rs and is subsumed by the crate-level rustdoc batches (1.6,
  2.1, 2.4, 2.5, 3.1, 3.2).

### Cap rationale

The ranking deliberately stops at ~22k tokens of source content rather
than padding to a 20k target. The mdbook fixture is large enough that
significant content is genuinely below-the-fold (the entire
`mdbook-html` rendering internals, the `links.rs` substitution engine,
all per-chapter user-guide pages), and the cut points above were chosen
so that anything dropped is either (a) one `Read` away from a
structural pointer that *is* in the ranking, or (b) so internal that an
agent would never need it for a typical query.
