# mdbook — North Star

Revision pin: `b8c90970`

mdBook is the Rust CLI that builds online books from Markdown — `mdbook init`, `build`, `watch`, `serve`, `test`, `clean`, `completions`. The repository is a Cargo workspace: a thin `mdbook` binary at the workspace root delegates everything to a layered set of `crates/mdbook-*` libraries (`-summary` parses `SUMMARY.md`; `-markdown` wraps `pulldown-cmark`; `-preprocessor` and `-renderer` define the two public extension traits; `-html` is the default backend with all front-end CSS/JS/templates baked in via `include_bytes!`; `-driver` is the high-level `MDBook` orchestrator; `-core` holds shared `Book`/`Chapter`/`Config` types). Around it sit a self-hosted user guide under `guide/`, an extensive snapbox-driven integration testsuite under `tests/testsuite/`, a Goml-driven GUI testsuite under `tests/gui/`, and example preprocessor/backend stubs in `examples/`.

## Batches

### 1.1 top-level repository listing
- Content: file/directory names directly under `test/fixtures/mdbook/` — `README.md`, `Cargo.toml`, `Cargo.lock`, `CHANGELOG.md`, `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, `LICENSE`, `package.json`, `eslint.config.mjs`, `rustfmt.toml`, `triagebot.toml`, `.cargo/`, `.github/`, `.gitignore`, `.gitattributes`, `.git-blame-ignore-revs`, `ci/`, `crates/`, `examples/`, `guide/`, `src/`, `tests/`
- Cost: 79 tokens (helper: `printf '...top-level entries...\n' | scripts/count-tokens.py --stdin`)
- Notes: orients the agent to the workspace shape — Rust workspace + JS lint + a self-hosted `guide/`.

### 1.2 root README
- Content: `README.md` (lines 1-21)
- Cost: 253 tokens (helper: `scripts/count-tokens.py README.md`)
- Notes: one-paragraph product description plus link to the User Guide.

### 1.3 workspace `Cargo.toml`
- Content: `Cargo.toml` (full file)
- Cost: 1441 tokens (helper: `scripts/count-tokens.py Cargo.toml`)
- Notes: lists workspace members, pinned dependency versions, and the `default = ["watch", "serve", "search"]` feature gates that decide whether `cmd::watch` / `cmd::serve` modules even compile.

### 1.4 `crates/` listing
- Content: subdirectory names of `crates/` — `mdbook-compare/`, `mdbook-core/`, `mdbook-driver/`, `mdbook-html/`, `mdbook-markdown/`, `mdbook-preprocessor/`, `mdbook-renderer/`, `mdbook-summary/`, `xtask/`
- Cost: 39 tokens (helper: `printf 'mdbook-compare/\n...\n' | scripts/count-tokens.py --stdin`)
- Notes: shows the layered crate split — without it the agent will not know `-summary` / `-preprocessor` / `-renderer` exist as standalone publishable crates.

### 1.5 `src/` (binary) tree listing
- Content: file/directory names under `src/` — `main.rs`, `cmd/mod.rs`, `cmd/build.rs`, `cmd/clean.rs`, `cmd/command_prelude.rs`, `cmd/init.rs`, `cmd/serve.rs`, `cmd/test.rs`, `cmd/watch.rs`, `cmd/watch/native.rs`, `cmd/watch/poller.rs`
- Cost: 54 tokens (helper: `printf 'cmd/\n...\nmain.rs\n' | scripts/count-tokens.py --stdin`)
- Notes: makes it instantly obvious where each subcommand lives (and that `watch` has a `native` vs `poller` split).

### 1.6 module-level docstrings of every workspace crate
- Content: `crates/mdbook-core/src/lib.rs` (full) + `crates/mdbook-driver/src/lib.rs` (lines 1-66) + `crates/mdbook-html/src/lib.rs` (full) + `crates/mdbook-markdown/src/lib.rs` (lines 1-12) + `crates/mdbook-preprocessor/src/lib.rs` (lines 1-22) + `crates/mdbook-renderer/src/lib.rs` (lines 1-21) + `crates/mdbook-summary/src/lib.rs` (lines 1-6)
- Cost: 1158 tokens (helper: `scripts/count-tokens.py crates/mdbook-core/src/lib.rs crates/mdbook-driver/src/lib.rs:1-66 crates/mdbook-html/src/lib.rs crates/mdbook-markdown/src/lib.rs:1-12 crates/mdbook-preprocessor/src/lib.rs:1-22 crates/mdbook-renderer/src/lib.rs:1-21 crates/mdbook-summary/src/lib.rs:1-6`)
- Notes: each crate's purpose, the inter-crate dependency story, the `MDBook::init` / `MDBook::load` entry-point examples, and the `MDBOOK_VERSION` constant. Single highest-value-per-token semantic batch in the fixture.

### 1.7 `src/main.rs` — subcommand dispatch
- Content: `src/main.rs` (lines 14-91) — `mod cmd;`, `main()` subcommand `match`, `create_clap_command()`
- Cost: 633 tokens (helper: `scripts/count-tokens.py src/main.rs:14-91`)
- Notes: ground-truth list of every subcommand, which are gated by `cfg(feature = "watch"/"serve")`, and the routing into `cmd::*::execute`.

### 2.1 `guide/src/SUMMARY.md`
- Content: `guide/src/SUMMARY.md` (full)
- Cost: 364 tokens (helper: `scripts/count-tokens.py guide/src/SUMMARY.md`)
- Notes: complete table of contents for the user-facing documentation. Tells the agent which user-facing concepts are documented (CLI subcommands, format/configuration, format/theme, for_developers/preprocessors+backends, continuous-integration, etc.) and exactly where each lives.

### 2.2 `crates/*/README.md` description + stability sentences
- Content: lines 7-9 of each `crates/mdbook-*/README.md` (the description sentence and the "intended for internal use only / follows semver compatibility" stability note)
- Cost: 723 tokens (helper: `scripts/count-tokens.py crates/mdbook-core/README.md:7-9 crates/mdbook-driver/README.md:7-9 crates/mdbook-html/README.md:7-9 crates/mdbook-markdown/README.md:7-9 crates/mdbook-preprocessor/README.md:7-9 crates/mdbook-renderer/README.md:7-9 crates/mdbook-summary/README.md:7-9 crates/mdbook-compare/README.md:7-9`)
- Notes: each crate README states its stability contract — `mdbook-core` is internal/no semver, `mdbook-driver` follows semver, etc. Critical for agents asked about API breakage or "is X part of the public API".

### 2.3 `Config` struct shell
- Content: `crates/mdbook-core/src/config.rs` (lines 60-110) — the `Config` struct definition with its `book`/`build`/`rust`/`output`/`preprocessor` fields plus `Default` and `FromStr` impls
- Cost: 334 tokens (helper: `scripts/count-tokens.py crates/mdbook-core/src/config.rs:60-110`)

### 2.4 `BookConfig` / `BuildConfig` / `RustConfig` / `RustEdition`
- Content: `crates/mdbook-core/src/config.rs` (lines 313-444)
- Cost: 1070 tokens (helper: `scripts/count-tokens.py crates/mdbook-core/src/config.rs:313-444`)
- Notes: full field-level definitions of the three top-level config sub-tables and the `RustEdition` enum, including doc-comments and `Default` impls. Without this the agent has no precise knowledge of what `book.toml`'s `[book]`/`[build]`/`[rust]` tables accept.

### 2.5 `MDBook` struct + module head
- Content: `crates/mdbook-driver/src/mdbook.rs` (lines 1-44)
- Cost: 346 tokens (helper: `scripts/count-tokens.py crates/mdbook-driver/src/mdbook.rs:1-44`)
- Notes: the public fields (`root`, `config`, `book`) plus the private `renderers`/`preprocessors` `IndexMap`s — pivotal mental model for how the library is used.

### 2.6 `Book`, `BookItem`, `Chapter`, `SectionNumber`
- Content: `crates/mdbook-core/src/book.rs` (lines 113-217)
- Cost: 782 tokens (helper: `scripts/count-tokens.py crates/mdbook-core/src/book.rs:113-217`)
- Notes: the type that `Preprocessor::run` and `Renderer::render` see. Doc-comments on each field tell the agent the README→index.md rewriting story (`Chapter::path` vs `Chapter::source_path`).

### 2.7 `Preprocessor` and `Renderer` traits (full crates)
- Content: full `crates/mdbook-preprocessor/src/lib.rs` + full `crates/mdbook-renderer/src/lib.rs`
- Cost: 1439 tokens (helper: `scripts/count-tokens.py crates/mdbook-preprocessor/src/lib.rs crates/mdbook-renderer/src/lib.rs`)
- Notes: these are the two stable extension points downstream crates implement; both files are small and entirely public-API including `PreprocessorContext` and `RenderContext`.

### 2.8 `mdbook-summary` types and grammar
- Content: `crates/mdbook-summary/src/lib.rs` (lines 1-145)
- Cost: 1104 tokens (helper: `scripts/count-tokens.py crates/mdbook-summary/src/lib.rs:1-145`)
- Notes: `parse_summary`, `Summary`, `Link`, `SummaryItem` plus the `SUMMARY.md` grammar EBNF in the rustdoc on `SummaryParser`. Single source of truth for what `SUMMARY.md` syntax mdBook accepts.

### 3.1 `crates/mdbook-html/src/` + `front-end/` listing
- Content: file names under `crates/mdbook-html/src/` and the asset directory names under `crates/mdbook-html/front-end/` — `lib.rs`, `utils.rs`, `html/{mod,admonitions,hide_lines,print,serialize,tests,tokenizer,tree}.rs`, `html_handlebars/{mod,hbs_renderer,search,static_files}.rs`, `html_handlebars/helpers/{mod,fontawesome,resources,toc}.rs`, `theme/{mod,fonts,playground_editor,searcher}.rs`, `front-end/{css,fonts,images,js,playground_editor,searcher,templates}/`
- Cost: 319 tokens (helper: `printf '...html crate tree...\n' | scripts/count-tokens.py --stdin`)
- Notes: HTML backend is the largest crate; this orients the agent to its sub-modules and to the bundled front-end assets that get baked into the binary via `include_bytes!`.

### 3.2 `MDBook` impl method signatures + docs (line-prefix view)
- Content: `crates/mdbook-driver/src/mdbook.rs` (lines 46-393), each line truncated at the first `(` or `{`
- Cost: 1917 tokens (helper: `scripts/count-tokens.py crates/mdbook-driver/src/mdbook.rs:46-393 --regex '^[^({]*'`)
- Notes: shows every public/private method of `MDBook` with its rustdoc — `load`, `load_with_config`, `load_with_config_and_summary`, `iter`, `init`, `build`, `preprocess_book`, `execute_build_process`, `with_renderer`, `with_preprocessor`, `test`, `test_chapter`, `build_dir_for`, `source_dir`, `theme_dir` — plus the private `determine_renderers`, `determine_preprocessors`, `preprocessor_should_run` helpers including the `DEFAULT_PREPROCESSORS = &["links", "index"]` constant. Choosing the prefix-truncated view over the full file (2790 tokens) trades ~870 tokens of method bodies for the same semantic surface.

### 3.3 built-in preprocessor surface
- Content: `crates/mdbook-driver/src/builtin_preprocessors/mod.rs` (full) + `crates/mdbook-driver/src/builtin_preprocessors/links.rs` (lines 1-50) + `crates/mdbook-driver/src/builtin_preprocessors/index.rs` (lines 1-30) + `crates/mdbook-driver/src/builtin_preprocessors/cmd.rs` (lines 1-25)
- Cost: 894 tokens (helper: `scripts/count-tokens.py crates/mdbook-driver/src/builtin_preprocessors/mod.rs crates/mdbook-driver/src/builtin_preprocessors/links.rs:1-50 crates/mdbook-driver/src/builtin_preprocessors/index.rs:1-30 crates/mdbook-driver/src/builtin_preprocessors/cmd.rs:1-25`)
- Notes: the docstring on `LinkPreprocessor` enumerates `{{# include}}`, `{{# rustdoc_include}}`, `{{# playground}}`, `{{# title}}` — the entire helper-link contract. `IndexPreprocessor` documents the README→index rewriting. `CmdPreprocessor` documents the JSON-over-stdin protocol pointer.

### 3.4 built-in renderers
- Content: `crates/mdbook-driver/src/builtin_renderers/mod.rs` (full) + `crates/mdbook-driver/src/builtin_renderers/markdown_renderer.rs` (full)
- Cost: 877 tokens (helper: `scripts/count-tokens.py crates/mdbook-driver/src/builtin_renderers/mod.rs crates/mdbook-driver/src/builtin_renderers/markdown_renderer.rs`)
- Notes: defines the `CmdRenderer` shell-out renderer (the way custom backends are spawned) and the trivial `MarkdownRenderer`.

### 3.5 default subcommand bodies (`build`, `test`) + clap helpers
- Content: `src/cmd/build.rs` (full) + `src/cmd/test.rs` (full) + `src/cmd/command_prelude.rs` (full)
- Cost: 1042 tokens (helper: `scripts/count-tokens.py src/cmd/build.rs src/cmd/test.rs src/cmd/command_prelude.rs`)
- Notes: `command_prelude.rs` defines the shared `arg_dest_dir` / `arg_root_dir` / `arg_open` clap helpers and `set_dest_dir`. `build.rs` shows the canonical `MDBook::load → set_dest_dir → build` flow other subcommands echo. `test.rs` exposes `MDBook::test` / `test_chapter`.

### 3.6 `mdbook-markdown` (full)
- Content: `crates/mdbook-markdown/src/lib.rs` (full)
- Cost: 562 tokens (helper: `scripts/count-tokens.py crates/mdbook-markdown/src/lib.rs`)
- Notes: tiny crate that exposes `MarkdownOptions` plus `new_cmark_parser` and the `pulldown-cmark` `Options` flags mdBook turns on by default (tables, footnotes, strikethrough, tasklists, heading-attributes, +smart-punctuation/definition-lists/admonitions when configured).

### 4.1 `HtmlConfig` (top-level fields)
- Content: `crates/mdbook-core/src/config.rs` (lines 445-572) — the `HtmlConfig` struct, its `Default` impl, and its `theme_dir` / `get_404_output_file` methods
- Cost: 1160 tokens (helper: `scripts/count-tokens.py crates/mdbook-core/src/config.rs:445-572`)
- Notes: enumerates every `[output.html]` field — themes, smart-punctuation, definition-lists, admonitions, mathjax, additional-css/js, fold/playground/code/print/search nesting, no_section_label, git_repository_url/icon, input_404, site_url, cname, edit_url_template, redirect, hash_files, sidebar_header_nav. Critical for any HTML-rendering or theming question. The nested sub-structs (`Print`, `Fold`, `Playground`, `Code`, `Search`) live on lines 575-712 — Read on demand when investigating those specific subsystems.

### 4.2 HTML backend pipeline module docstrings
- Content: `crates/mdbook-html/src/html_handlebars/hbs_renderer.rs` (lines 1-23) + `crates/mdbook-html/src/html/mod.rs` (lines 1-32) + `crates/mdbook-html/src/html/tree.rs` (lines 1-22) + `crates/mdbook-html/src/theme/mod.rs` (lines 1-32)
- Cost: 1151 tokens (helper: `scripts/count-tokens.py crates/mdbook-html/src/html_handlebars/hbs_renderer.rs:1-23 crates/mdbook-html/src/html/mod.rs:1-32 crates/mdbook-html/src/html/tree.rs:1-22 crates/mdbook-html/src/theme/mod.rs:1-32`)
- Notes: explains the markdown→`pulldown_cmark` events→tree→serialize pipeline, the `HtmlHandlebars` Renderer impl, and the `Theme` asset bundle including the `INDEX`/`HEAD`/`HEADER`/`*_CSS`/`*_JS` `include_bytes!` block in `theme/mod.rs:14-31`. Other html sub-module heads (`print.rs`, `serialize.rs`, `tokenizer.rs`, `hide_lines.rs`, `static_files.rs`, `search.rs`) are listed in `3.1` and Read on demand.

### 4.3 `cmd::serve` and `cmd::watch` (feature-gated subcommands)
- Content: `src/cmd/serve.rs` (full) + `src/cmd/watch.rs` (full)
- Cost: 1699 tokens (helper: `scripts/count-tokens.py src/cmd/serve.rs src/cmd/watch.rs`)
- Notes: live-reload websocket protocol (`/__livereload`), the `WatcherKind::{Poll,Native}` selection, and the `update_config` closure that overrides `output.html.live-reload-endpoint` / `output.html.site-url`. Compiled only when `serve` / `watch` features are on (default-enabled per `1.3`).

### 4.4 `tests/` listing
- Content: file names under `tests/testsuite/` plus `tests/gui/runner.rs`, the `*.goml` files, and the `gui/books/` directory — `testsuite/{main,book_test,build,cli,config,includes,index,init,markdown,playground,preprocessor,print,redirects,renderer,rendering,search,test,theme,toc}.rs`, `testsuite/README.md`, and analogous fixture-tree directories alongside each module
- Cost: 148 tokens (helper: `printf '...tests tree...\n' | scripts/count-tokens.py --stdin`)

### 4.5 `guide/` tree listing + intro features
- Content: directory listing under `guide/` — `book.toml`, `guide-helper/{Cargo.toml,src/lib.rs,src/main.rs}`, `src/{SUMMARY.md, README.md, 404.md, cli/, format/, for_developers/, guide/, misc/, continuous-integration.md}` — plus `guide/src/README.md` (lines 20-44) which is the user-facing feature bullet-list and Contributing/License footer
- Cost: 364 tokens (helpers: `printf '...guide tree...\n' | scripts/count-tokens.py --stdin` = 85; `scripts/count-tokens.py guide/src/README.md:20-44` = 279)

### 5.1 `mdbook-driver` private helpers
- Content: `crates/mdbook-driver/src/lib.rs` (lines 67-131)
- Cost: 325 tokens (helper: `scripts/count-tokens.py crates/mdbook-driver/src/lib.rs:67-131`)
- Notes: `compose_command` and `handle_command_error` — the shared shell-out plumbing used by both `CmdPreprocessor` and `CmdRenderer`.

### 5.2 `BookBuilder` (init flow)
- Content: `crates/mdbook-driver/src/init.rs` (lines 1-65)
- Cost: 481 tokens (helper: `scripts/count-tokens.py crates/mdbook-driver/src/init.rs:1-65`)
- Notes: builder methods (`with_config`, `copy_theme`, `create_gitignore`, `build`) the `mdbook init` subcommand calls into.

### 5.3 `examples/nop-preprocessor.rs` head
- Content: `examples/nop-preprocessor.rs` (lines 1-50)
- Cost: 386 tokens (helper: `scripts/count-tokens.py examples/nop-preprocessor.rs:1-50`)
- Notes: canonical minimal preprocessor showing the `mdbook-preprocessor::parse_input(io::stdin())` + `supports` subcommand contract that 3rd-party preprocessors implement.

## Below-the-fold

- **`CHANGELOG.md` (24,285 tokens)** — by far the largest single file in the fixture. Contains the 0.5 migration guide and per-version release notes. The agent can `Read` it on demand for any "what changed in version Y" question; including any meaningful slice would dominate the budget.
- **Bundled front-end assets in `crates/mdbook-html/front-end/`** — `book.js`, `clipboard.min.js`, `highlight.js`, `searcher/elasticlunr.min.js`/`mark.min.js`/`searcher.js`, `playground_editor/{ace.js, editor.js, mode-rust.js, theme-*.js}`, the seven `css/*.css` files, the `templates/*.hbs` Handlebars templates, the `OPEN-SANS`/`SOURCE-CODE-PRO` woff2 fonts, and the favicon SVG/PNG. They are bundled into the binary verbatim, are large (minified JS especially), and are listed in `1.4` + `3.1` + `4.2` so the agent knows where to look. Reading any specific one is a single `Read` away.
- **GUI tests under `tests/gui/`** — 20 `.goml` browser-ui-test scripts and ~150 fixture-book markdown files (`books/sidebar-scroll/` alone has 100 chapters). Listed in `4.4`; their content is highly repetitive.
- **Integration testsuite source bodies** (`tests/testsuite/*.rs` other than `main.rs`) — ~20 large files of snapbox-driven test cases, plus the `BookTest` harness in `book_test.rs`. Their existence and harness conventions are catalogued in `4.4`; bodies are best fetched on demand for the specific feature under investigation. The harness pattern is described in `tests/testsuite/README.md` (693 tokens) — read it directly if writing tests.
- **`Cargo.lock`** — large generated file; the meaningful pinned-version information for direct deps is in `Cargo.toml` (`1.3`).
- **`ci/*.sh`, `.github/workflows/*.yml`, `triagebot.toml`, `eslint.config.mjs`, `package.json`, `rustfmt.toml`, `.cargo/config.toml`** — small infrastructural files. Their names appear in `1.1`; their bodies rarely matter outside CI/release questions.
- **`crates/mdbook-html/src/html/{tree.rs (1155 LOC), print.rs (215 LOC), hide_lines.rs (193 LOC), serialize.rs (112 LOC), tokenizer.rs (83 LOC), admonitions.rs (26 LOC, mostly SVG)} bodies, and `crates/mdbook-html/src/html_handlebars/{hbs_renderer.rs (696 LOC), search.rs (445 LOC), static_files.rs (320 LOC)} bodies, plus the `helpers/{toc, fontawesome, resources}.rs` files and `theme/{mod, fonts, playground_editor, searcher}.rs` bodies** — the mechanical heart of the markdown→HTML transform. Module docstrings in `4.2` describe the pipeline; line-prefix or signature batches would cost ~3-4k tokens and substantially overlap that explanation. Best Read on demand.
- **`crates/mdbook-driver/src/builtin_preprocessors/links.rs` body** (~830 lines past the surface batched in `3.3`) — implements the link-helper expansion engine including `take_lines`. The user-facing helper inventory is documented in the `LinkPreprocessor` rustdoc shown in `3.3`.
- **`crates/mdbook-summary/src/lib.rs` parser body (1056 lines past the types in `2.8`)** — the recursive-descent SUMMARY parser. The grammar EBNF lives in `2.8`; the parser body is rarely needed.
- **`crates/mdbook-driver/src/load.rs` (309 LOC)** — `load_book` / `load_book_from_disk` / `create_missing` plus the chapter-loading recursion. Module head appears via `1.6` (the workspace lib.rs `mod load;` declaration); body Read on demand.
- **`crates/mdbook-driver/src/mdbook/tests.rs` and `crates/mdbook-core/src/book/tests.rs`** — unit tests for the driver and book modules.
- **Tests inside `crates/mdbook-core/src/config.rs` (lines 731-1209)** — 480 lines of `#[test]` cases including the `COMPLEX_CONFIG` example. Useful as an executable spec but redundant given the type definitions in `2.3`/`2.4`/`4.1`.
- **`HtmlConfig` sub-structs `Print`, `Fold`, `Playground`, `Code`, `Search`, `SearchChapterSettings`** (`crates/mdbook-core/src/config.rs` lines 575-712, ~1k tokens) — full nested config types for the HTML renderer. Field names are visible in the parent `HtmlConfig` (`4.1`); Read on demand for search-tuning, playground-tuning, or print-page questions.
- **`crates/mdbook-core/src/utils/{mod, fs, html, toml_ext}.rs`** — small internal helpers (`static_regex!`, `path_to_root`, `read_to_string` wrappers, `escape_html`, `TomlExt`). Names visible in `1.4`-equivalent crate tree; bodies Read on demand.
- **`crates/xtask/`** — local-development task runner (`cargo xtask test-all`, `cargo xtask bump`, `cargo xtask changelog`). Meaningful only for maintainers, not coding-agent users.
- **`crates/mdbook-compare/`** — diagnostic CLI for diffing the output of two mdBook versions; small and standalone.
- **`src/cmd/watch/{native, poller}.rs`** — the two filesystem-watcher backends (`notify_debouncer_mini` for native, polling fallback). The `WatcherKind::from_str` dispatch is shown in `4.3`; backend bodies are mechanical.
- **`src/cmd/clean.rs` (114 LOC, 896 tokens)** — implements the recursive `Clean` summary (file/dir counts + human-readable byte totals). Self-contained; useful only for the rare clean-related question.
- **`src/cmd/init.rs` (122 LOC, 912 tokens)** — interactive stdin prompts (`request_book_title`, `confirm`), theme/gitignore flag handling, and the call into `mdbook_driver::init::BookBuilder` (whose builder methods are batched in `5.2`). Read on demand for `mdbook init`-specific questions.
- **`guide/` chapter bodies** — `format/markdown.md` (2.1k tokens), `format/configuration/general.md` (974 tokens), `format/summary.md` (887 tokens), `for_developers/preprocessors.md` (1.3k tokens), `for_developers/backends.md` (2.7k tokens), the per-CLI-subcommand pages, and the rest of `format/`. Their titles are catalogued via `2.1`; the source-of-truth versions are the Rust types/trait definitions already batched.
- **`examples/remove-emphasis/`** — a fuller worked-example preprocessor (book.toml + crate + chapter + test.rs). Listed in `1.1`; a single `Read` of `mdbook-remove-emphasis/src/main.rs` is enough when needed.
- **`CONTRIBUTING.md` (2898 tokens)** — useful for contributor questions but rarely needed at the start of a coding-agent task; one `Read` away.
- **`CODE_OF_CONDUCT.md`, `LICENSE`** — boilerplate.

Cumulative cost across ranked batches (approximate):
- 1.x (orientation): ~3.7k
- 2.x (semantic models): ~9.8k
- 3.x (primary API surface): ~15.4k
- 4.x (HtmlConfig + html backend doc heads + serve/watch + tests/guide listings): ~20.0k
- 5.x (private helpers, BookBuilder, example preprocessor): ~21.1k

Group 4 brings cumulative content right to the 20k cap; group 5 is a small overshoot reserved for the largest budgets and could be dropped without loss to mid-range budgets.
