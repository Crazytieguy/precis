# mdbook — North Star

Revision pin: `b8c90970`

## Batches

### 1.1 Repo root file/folder listing
- Content: rendered as one entry per line (folders suffixed `/`):
  `CHANGELOG.md`, `CODE_OF_CONDUCT.md`, `CONTRIBUTING.md`, `Cargo.lock`, `Cargo.toml`, `LICENSE`, `README.md`, `ci/`, `crates/`, `eslint.config.mjs`, `examples/`, `guide/`, `package.json`, `rustfmt.toml`, `src/`, `tests/`, `triagebot.toml`
- Cost: 60 tokens (helper: stdin)
- Notes: Top of the orientation pyramid. Tells the agent this is a Cargo workspace with a sibling `crates/` folder, a `guide/` mdBook, an in-tree `tests/` harness, and front-end JS tooling (`package.json`, `eslint.config.mjs`).

### 1.2 README.md (full)
- Content: `tests/fixtures/mdbook/README.md` whole file
- Cost: 253 tokens (helper: `count-tokens.py README.md`)
- Notes: One sentence states what mdBook is ("utility to create modern online books from Markdown files"). Cheap, high-value for "what is this repo?".

### 1.3 `[workspace] members` line of root `Cargo.toml`
- Content: `tests/fixtures/mdbook/Cargo.toml:1-7`
- Cost: 34 tokens (helper: `count-tokens.py Cargo.toml:1-7`)
- Notes: Confirms workspace shape (`crates/*`, `examples/remove-emphasis/...`, `guide/guide-helper`).

### 1.4 `crates/` folder listing
- Content: rendered as one per line: `mdbook-compare/`, `mdbook-core/`, `mdbook-driver/`, `mdbook-html/`, `mdbook-markdown/`, `mdbook-preprocessor/`, `mdbook-renderer/`, `mdbook-summary/`, `xtask/`
- Cost: 39 tokens (helper: stdin)
- Notes: The crate split is the primary architectural fact about this codebase.

### 1.5 Crate-purpose lines from `crates/mdbook-driver/src/lib.rs`
- Content: `tests/fixtures/mdbook/crates/mdbook-driver/src/lib.rs:14-25` (the `## Additional crates` bullet list)
- Cost: 139 tokens (helper: `count-tokens.py crates/mdbook-driver/src/lib.rs:14-25`)
- Notes: One-line authoritative description of every crate's role from the project's own docs.

### 1.6 `src/` folder listing (binary crate)
- Content: `main.rs`, `cmd/`
- Cost: 5 tokens (helper: stdin)

### 1.7 `src/cmd/` folder listing
- Content: rendered one per line: `build.rs`, `clean.rs`, `command_prelude.rs`, `init.rs`, `mod.rs`, `serve.rs`, `test.rs`, `watch.rs`, `watch/native.rs`, `watch/poller.rs`
- Cost: 36 tokens (helper: stdin)

### 1.8 mdbook CLI subcommand dispatch in `main()`
- Content: `tests/fixtures/mdbook/src/main.rs:18-55`
- Cost: 291 tokens (helper: `count-tokens.py src/main.rs:18-55`)
- Notes: Exact subcommand-string → `cmd::<X>::execute` mapping with `#[cfg(feature = ...)]` gating.

### 1.9 `MDBook` struct definition
- Content: `tests/fixtures/mdbook/crates/mdbook-driver/src/mdbook.rs:28-44`
- Cost: 116 tokens (helper: `count-tokens.py crates/mdbook-driver/src/mdbook.rs:28-44`)
- Notes: Central public type; fields `root`/`config`/`book`/`renderers`/`preprocessors` reveal the build pipeline.

### 1.10 `Preprocessor` trait
- Content: `tests/fixtures/mdbook/crates/mdbook-preprocessor/src/lib.rs:23-45`
- Cost: 216 tokens (helper: `count-tokens.py crates/mdbook-preprocessor/src/lib.rs:23-45`)

### 1.11 `Renderer` trait
- Content: `tests/fixtures/mdbook/crates/mdbook-renderer/src/lib.rs:22-35`
- Cost: 127 tokens (helper: `count-tokens.py crates/mdbook-renderer/src/lib.rs:22-35`)

### 1.12 LinkPreprocessor doc-comment listing all `{{# …}}` helpers
- Content: `tests/fixtures/mdbook/crates/mdbook-driver/src/builtin_preprocessors/links.rs:20-32`
- Cost: 171 tokens (helper: `count-tokens.py crates/mdbook-driver/src/builtin_preprocessors/links.rs:20-32`)
- Notes: Authoritative list of supported helpers (`include`, `rustdoc_include`, `playground`, `title`).

### 1.13 IndexPreprocessor doc-comment
- Content: `tests/fixtures/mdbook/crates/mdbook-driver/src/builtin_preprocessors/index.rs:8-22`
- Cost: 102 tokens (helper: `count-tokens.py crates/mdbook-driver/src/builtin_preprocessors/index.rs:8-22`)
- Notes: Documents the `README.md` → `index.md` transformation.

## 2. Subcommand surface and core API

### 2.1 Guide `SUMMARY.md` (full)
- Content: `tests/fixtures/mdbook/guide/src/SUMMARY.md` whole file
- Cost: 364 tokens (helper: `count-tokens.py guide/src/SUMMARY.md`)
- Notes: Budget-independent ToC for the entire user-facing documentation. Lets the agent jump from any user concept to the precise `guide/src/...` file.

### 2.2 `cmd/mod.rs`
- Content: `tests/fixtures/mdbook/src/cmd/mod.rs` whole file
- Cost: 57 tokens

### 2.3 `make_subcommand` for build/init/clean/test/watch
- Content: `tests/fixtures/mdbook/src/cmd/build.rs:8-14`, `src/cmd/init.rs:12-29`, `src/cmd/clean.rs:11-16`, `src/cmd/test.rs:9-32`, `src/cmd/watch.rs:12-19`
- Cost: 471 tokens (helper: sum of those ranges)
- Notes: Every flag for these five subcommands at once. Agent answers "what flags does `mdbook init --foo` take?" without follow-up.

### 2.4 `make_subcommand` for serve
- Content: `tests/fixtures/mdbook/src/cmd/serve.rs:23-48`
- Cost: 191 tokens (helper: `count-tokens.py src/cmd/serve.rs:23-48`)
- Notes: Split out because serve has more flags (hostname/port/open/watcher).

### 2.5 `command_prelude.rs` shared CLI args
- Content: `tests/fixtures/mdbook/src/cmd/command_prelude.rs` whole file
- Cost: 474 tokens (helper: `count-tokens.py src/cmd/command_prelude.rs`)
- Notes: Defines `arg_dest_dir`, `arg_root_dir`, `arg_open`, `arg_watcher`, `set_dest_dir`.

### 2.6 `MDBook::load*` and `iter`/`init` method headers
- Content: `tests/fixtures/mdbook/crates/mdbook-driver/src/mdbook.rs:46-110` (load, load_with_config, load_with_config_and_summary, iter)
- Cost: 437 tokens (helper: `count-tokens.py crates/mdbook-driver/src/mdbook.rs:46-110`)
- Predecessor: 1.9.

### 2.7 `MDBook::init` + `build` method docs
- Content: `tests/fixtures/mdbook/crates/mdbook-driver/src/mdbook.rs:111-160`
- Cost: 418 tokens (helper: `count-tokens.py crates/mdbook-driver/src/mdbook.rs:111-160`)
- Predecessor: 1.9.

### 2.8 `MDBook::preprocess_book` + `execute_build_process`
- Content: `tests/fixtures/mdbook/crates/mdbook-driver/src/mdbook.rs:161-225`
- Cost: 503 tokens (helper: `count-tokens.py crates/mdbook-driver/src/mdbook.rs:161-225`)
- Notes: The actual pipeline: load → preprocess → render. Shows `chapter_titles` mechanism.
- Predecessor: 1.9.

### 2.9 `MDBook::with_renderer` + `with_preprocessor` + `build_dir_for` + `source_dir` + `theme_dir`
- Content: `tests/fixtures/mdbook/crates/mdbook-driver/src/mdbook.rs:212-225` (with_renderer/with_preprocessor) and `:346-393` (build_dir_for/source_dir/theme_dir)
- Cost: 476 tokens (helper: `count-tokens.py crates/mdbook-driver/src/mdbook.rs:212-225 crates/mdbook-driver/src/mdbook.rs:346-393`)
- Predecessor: 1.9.

### 2.10 `Config` + `BookConfig` struct definitions
- Content: `tests/fixtures/mdbook/crates/mdbook-core/src/config.rs:60-78` (Config) and `:313-351` (BookConfig)
- Cost: 452 tokens (helper: `count-tokens.py crates/mdbook-core/src/config.rs:60-78 crates/mdbook-core/src/config.rs:313-351`)

### 2.11 `BuildConfig` + `RustConfig` + `RustEdition` + `TextDirection`
- Content: `tests/fixtures/mdbook/crates/mdbook-core/src/config.rs:391-444` plus `:365-388`
- Cost: 670 tokens (helper: `count-tokens.py crates/mdbook-core/src/config.rs:365-388 crates/mdbook-core/src/config.rs:391-444`)

### 2.12 `HtmlConfig` (the big config struct)
- Content: `tests/fixtures/mdbook/crates/mdbook-core/src/config.rs:445-553`
- Cost: 1007 tokens (helper: `count-tokens.py crates/mdbook-core/src/config.rs:445-553`)
- Notes: 25+ HTML-renderer-specific fields with doc comments; the dominant config surface in practice.

### 2.13 `Print` + `Fold` + `Playground` + `Code` config structs
- Content: `tests/fixtures/mdbook/crates/mdbook-core/src/config.rs:574-645`
- Cost: 517 tokens (helper: `count-tokens.py crates/mdbook-core/src/config.rs:574-645`)

### 2.14 `Search` + `SearchChapterSettings` config
- Content: `tests/fixtures/mdbook/crates/mdbook-core/src/config.rs:646-712`
- Cost: 618 tokens (helper: `count-tokens.py crates/mdbook-core/src/config.rs:646-712`)

### 2.15 `mdbook-core::config` module doc-comment with TOML example
- Content: `tests/fixtures/mdbook/crates/mdbook-core/src/config.rs:1-46`
- Cost: 356 tokens (helper: `count-tokens.py crates/mdbook-core/src/config.rs:1-46`)
- Notes: End-to-end example of loading a `book.toml` via `Config::from_str`, mutating, querying.

### 2.16 `Book` + `BookItem` + `Chapter` + `SectionNumber` + `BookItems` definitions
- Content: `tests/fixtures/mdbook/crates/mdbook-core/src/book.rs:21-29`, `:113-127`, `:136-170`, `:218-230`, `:263-272`
- Cost: 643 tokens (helper: sum)
- Notes: Data model every preprocessor mutates. The `path` vs `source_path` doc on `Chapter` is critical.

### 2.17 `crates/mdbook-driver/src/` folder listing
- Content: `builtin_preprocessors/`, `builtin_renderers/`, `init.rs`, `lib.rs`, `load.rs`, `mdbook.rs`
- Cost: 21 tokens (helper: stdin)

### 2.18 `crates/mdbook-core/src/` folder listing
- Content: `book.rs`, `config.rs`, `lib.rs`, `utils/`
- Cost: 11 tokens (helper: stdin)

### 2.19 Built-in preprocessors / renderers folder listings
- Content: `crates/mdbook-driver/src/builtin_preprocessors/`: `cmd.rs`, `index.rs`, `links.rs`, `links/take_lines.rs`, `mod.rs` and `crates/mdbook-driver/src/builtin_renderers/`: `markdown_renderer.rs`, `mod.rs`
- Cost: 22 tokens (helper: stdin)
- Notes: Tells the agent the only built-in preprocessors are `cmd`, `index`, `links` and the only pure-Rust built-in renderer (besides HTML) is `markdown`.

### 2.20 `mdbook-html` crate root + `html/` module-level doc-comment
- Content: `tests/fixtures/mdbook/crates/mdbook-html/src/lib.rs` (full) + `crates/mdbook-html/src/html/mod.rs:1-32`
- Cost: 290 tokens (helper: sum)
- Notes: One-paragraph overview of the markdown→HTML pipeline (`pulldown_cmark` events → `tree` → `serialize`).

### 2.21 `crates/mdbook-html/src/` folder listing
- Content: `html/`, `html_handlebars/`, `lib.rs`, `theme/`, `utils.rs`
- Cost: 14 tokens (helper: stdin)

## 3. Built-in plumbing and CLI implementations

### 3.1 `cmd::build::execute`
- Content: `tests/fixtures/mdbook/src/cmd/build.rs:17-36`
- Cost: 128 tokens (helper: `count-tokens.py src/cmd/build.rs:17-36`)
- Predecessor: 2.3.

### 3.2 `cmd::test::execute`
- Content: `tests/fixtures/mdbook/src/cmd/test.rs:35-52`
- Cost: 135 tokens (helper: `count-tokens.py src/cmd/test.rs:35-52`)
- Predecessor: 2.3.

### 3.3 `cmd::serve::execute` body
- Content: `tests/fixtures/mdbook/src/cmd/serve.rs:51-106`
- Cost: 457 tokens (helper: `count-tokens.py src/cmd/serve.rs:51-106`)
- Notes: live-reload + websocket + watch composition; `LIVE_RELOAD_ENDPOINT = "__livereload"`.
- Predecessor: 2.4.

### 3.4 `cmd::init::execute`
- Content: `tests/fixtures/mdbook/src/cmd/init.rs:32-85`
- Cost: 414 tokens (helper: `count-tokens.py src/cmd/init.rs:32-85`)
- Predecessor: 2.3.

### 3.5 `determine_renderers` + `determine_preprocessors`
- Content: `tests/fixtures/mdbook/crates/mdbook-driver/src/mdbook.rs:395-540`
- Cost: 1214 tokens (helper: `count-tokens.py crates/mdbook-driver/src/mdbook.rs:395-540`)
- Notes: Resolves `[output.X]`/`[preprocessor.X]` config tables into renderer/preprocessor instances; topological sort for `before`/`after`; special-case names (`html`, `markdown`, `links`, `index`).
- Predecessor: 2.10.

### 3.6 `preprocessor_should_run`
- Content: `tests/fixtures/mdbook/crates/mdbook-driver/src/mdbook.rs:541-569`
- Cost: 244 tokens (helper: `count-tokens.py crates/mdbook-driver/src/mdbook.rs:541-569`)
- Predecessor: 1.10, 2.12.

### 3.7 `MDBook::test_chapter` body
- Content: `tests/fixtures/mdbook/crates/mdbook-driver/src/mdbook.rs:226-345`
- Cost: 862 tokens (helper: `count-tokens.py crates/mdbook-driver/src/mdbook.rs:226-345`)
- Notes: The whole rustdoc-shells-out test runner. `cmd.args(["--edition", "...."])` per `RustEdition`. Important for `mdbook test` debugging.
- Predecessor: 2.6.

### 3.8 `load_book` + `load_book_from_disk` + `create_missing`
- Content: `tests/fixtures/mdbook/crates/mdbook-driver/src/load.rs:9-78`
- Cost: 541 tokens (helper: `count-tokens.py crates/mdbook-driver/src/load.rs:9-78`)
- Notes: SUMMARY.md → Book pipeline; `build.create-missing` behavior (writes stub `# Title\n` for missing chapter files).

### 3.9 SUMMARY.md grammar + `parse_summary` doc-comment
- Content: `tests/fixtures/mdbook/crates/mdbook-summary/src/lib.rs:1-62` + `:147-181`
- Cost: 800 tokens (helper: `count-tokens.py crates/mdbook-summary/src/lib.rs:1-62 crates/mdbook-summary/src/lib.rs:147-181`)
- Notes: Authoritative grammar of the most user-visible mdBook file format.

### 3.10 `Summary`, `SummaryItem`, `Link` structs
- Content: `tests/fixtures/mdbook/crates/mdbook-summary/src/lib.rs:64-129`
- Cost: 490 tokens (helper: `count-tokens.py crates/mdbook-summary/src/lib.rs:64-129`)

### 3.11 `CmdPreprocessor` and `CmdRenderer` types
- Content: `tests/fixtures/mdbook/crates/mdbook-driver/src/builtin_preprocessors/cmd.rs:9-31` + `crates/mdbook-driver/src/builtin_renderers/mod.rs:15-30`
- Cost: 259 tokens (helper: `count-tokens.py crates/mdbook-driver/src/builtin_preprocessors/cmd.rs:9-31 crates/mdbook-driver/src/builtin_renderers/mod.rs:15-30`)
- Notes: The shell-out plugin mechanism — explains how 3rd-party plugins are invoked.

### 3.12 `nop-preprocessor` example top half
- Content: `tests/fixtures/mdbook/examples/nop-preprocessor.rs:1-72`
- Cost: 531 tokens (helper: `count-tokens.py examples/nop-preprocessor.rs:1-72`)
- Notes: Reference implementation showing the exact stdin/stdout JSON protocol (`parse_input`, `pre.run`, `to_writer(stdout, …)`) plus the `supports` subcommand convention.

### 3.13 LinkPreprocessor body teaser (`replace_all` + types)
- Content: `tests/fixtures/mdbook/crates/mdbook-driver/src/builtin_preprocessors/links.rs:1-50` (struct + run impl), an elision marker, then `:138-170` (`LineRange`/`LinkType` enums)
- Cost: 668 tokens (helper: `count-tokens.py crates/mdbook-driver/src/builtin_preprocessors/links.rs:1-50 crates/mdbook-driver/src/builtin_preprocessors/links.rs:138-170`)
- Notes: The rest (regex parsing, range/anchor handling, `from_capture`) is below-the-fold; agent can `Read` the rest if needed.
- Predecessor: 1.12.

### 3.14 `MarkdownOptions` + `new_cmark_parser`
- Content: `tests/fixtures/mdbook/crates/mdbook-markdown/src/lib.rs` whole file
- Cost: 452 tokens (helper: `count-tokens.py crates/mdbook-markdown/src/lib.rs`)
- Notes: Exact pulldown-cmark options mdBook always enables vs config-toggled.

### 3.15 `MarkdownRenderer` (full)
- Content: `tests/fixtures/mdbook/crates/mdbook-driver/src/builtin_renderers/markdown_renderer.rs` whole file
- Cost: 262 tokens (helper: `count-tokens.py crates/mdbook-driver/src/builtin_renderers/markdown_renderer.rs`)

### 3.16 `Cargo.toml` `workspace.dependencies` block
- Content: `tests/fixtures/mdbook/Cargo.toml:27-71`
- Cost: 608 tokens (helper: `count-tokens.py Cargo.toml:27-71`)
- Notes: Names every external crate (handlebars, pulldown-cmark, axum, html5ever, elasticlunr-rs, etc.) with pinned versions.

### 3.17 ci/ folder listing
- Content: `install-rust.sh`, `make-release-asset.sh`, `publish-guide.sh`, `update-dependencies.sh`
- Cost: 20 tokens (helper: stdin)

### 3.18 `.github/workflows/` listing
- Content: `deploy.yml`, `main.yml`, `update-dependencies.yml`
- Cost: 11 tokens (helper: stdin)

## 4. HTML rendering, theme, search

### 4.1 `HtmlHandlebars::render` outer body (orchestration)
- Content: `tests/fixtures/mdbook/crates/mdbook-html/src/html_handlebars/hbs_renderer.rs:303-420`
- Cost: 948 tokens (helper: `count-tokens.py crates/mdbook-html/src/html_handlebars/hbs_renderer.rs:303-420`)
- Notes: The orchestration: theme load → handlebars template registration → search-index creation → toc rendering → per-chapter loop. Search gated on `cfg(feature = "search")`.
- Predecessor: 2.20.

### 4.2 `Theme` struct + static template/CSS bytes
- Content: `tests/fixtures/mdbook/crates/mdbook-html/src/theme/mod.rs:14-61`
- Cost: 696 tokens (helper: `count-tokens.py crates/mdbook-html/src/theme/mod.rs:14-61`)
- Notes: Names every theme file the user can override and where the defaults live (`front-end/templates/`, `front-end/css/`, etc.).

### 4.3 `front-end/` folder listing
- Content: `css/`, `fonts/`, `images/`, `js/`, `playground_editor/`, `searcher/`, `templates/`
- Cost: 17 tokens (helper: stdin)

### 4.4 templates and CSS file listings
- Content: `crates/mdbook-html/front-end/templates/`: `head.hbs`, `header.hbs`, `index.hbs`, `redirect.hbs`, `toc.html.hbs`, `toc.js.hbs`; `crates/mdbook-html/front-end/css/`: `ayu-highlight.css`, `chrome.css`, `general.css`, `highlight.css`, `print.css`, `tomorrow-night.css`, `variables.css`
- Cost: 50 tokens (helper: stdin, both folders)

### 4.5 `crates/mdbook-html/src/html/` folder listing
- Content: `admonitions.rs`, `hide_lines.rs`, `mod.rs`, `print.rs`, `serialize.rs`, `tests.rs`, `tokenizer.rs`, `tree.rs`
- Cost: 39 tokens (helper: stdin)

### 4.6 `crates/mdbook-html/src/html_handlebars/` folder listing
- Content: `hbs_renderer.rs`, `helpers/` (with `fontawesome.rs`, `mod.rs`, `resources.rs`, `toc.rs`), `mod.rs`, `search.rs`, `static_files.rs`
- Cost: 35 tokens (helper: stdin)

### 4.7 `select_tag` (admonition kinds)
- Content: `tests/fixtures/mdbook/crates/mdbook-html/src/html/admonitions.rs:18-26`
- Cost: 120 tokens (helper: `count-tokens.py crates/mdbook-html/src/html/admonitions.rs:18-26`)
- Notes: Names the five GFM admonition kinds mdBook supports (Note, Tip, Important, Warning, Caution).

### 4.8 `tokenize` (search index tokenizer)
- Content: `tests/fixtures/mdbook/crates/mdbook-html/src/html_handlebars/search.rs:17-26`
- Cost: 109 tokens (helper: `count-tokens.py crates/mdbook-html/src/html_handlebars/search.rs:17-26`)
- Notes: The exact tokenization rule + `MAX_WORD_LENGTH_TO_INDEX = 80`.

### 4.9 `crates/mdbook-html/src/theme/` folder listing
- Content: `fonts.rs`, `mod.rs`, `playground_editor.rs`, `searcher.rs`
- Cost: 15 tokens (helper: stdin)

### 4.10 mdbook-html `utils.rs` (path/url helpers)
- Content: `tests/fixtures/mdbook/crates/mdbook-html/src/utils.rs:1-60` (`normalize_path`, `ToUrlPath`, `unique_id`, `id_from_content` doc-comments)
- Cost: 422 tokens (helper: `count-tokens.py crates/mdbook-html/src/utils.rs:1-60`)
- Notes: HTML id generation rule with explicit pointers to the GitHub/GitLab/pandoc heuristics.

## 5. Tests, examples, contributor docs

### 5.1 `CONTRIBUTING.md` canonical-commands section
- Content: `tests/fixtures/mdbook/CONTRIBUTING.md:101-150` (the "Tests" subsection listing every `cargo test --workspace`, `cargo test --test gui`, `npm run lint`, `cargo xtask test-all` command)
- Cost: 648 tokens (helper: `count-tokens.py CONTRIBUTING.md:101-150`)
- Notes: The high-density piece — the rest of CONTRIBUTING.md is conventional and one-hop discoverable.

### 5.2 `tests/` folder listing
- Content: `gui/`, `testsuite/`
- Cost: 5 tokens (helper: stdin)

### 5.3 `tests/testsuite/README.md`
- Content: full file
- Cost: 693 tokens (helper: `count-tokens.py tests/testsuite/README.md`)
- Notes: How to write new tests with `BookTest`, how snapbox `SNAPSHOTS=overwrite` works.

### 5.4 `tests/testsuite/` top-level listing
- Content: rendered one per line: `README.md`, `book_test.rs`, `build.rs`, `cli.rs`, `config.rs`, `includes.rs`, `index.rs`, `init.rs`, `main.rs`, `markdown.rs`, `playground.rs`, `preprocessor.rs`, `print.rs`, `redirects.rs`, `renderer.rs`, `rendering.rs`, `search.rs`, `test.rs`, `theme.rs`, `toc.rs`, plus per-feature subdirectories (`build/`, `config/`, `includes/`, `index/`, `init/`, `markdown/`, `playground/`, `preprocessor/`, `print/`, `redirects/`, `renderer/`, `rendering/`, `search/`, `test/`, `theme/`, `toc/`)
- Cost: 101 tokens (helper: stdin)
- Notes: Lets the agent jump from "I'm modifying include behaviour" to `tests/testsuite/includes.rs`.

### 5.5 `tests/gui/` listing (.goml files)
- Content: rendered one per line: `heading-nav-collapsed.goml`, `heading-nav-current-to-bottom.goml`, `heading-nav-empty.goml`, `heading-nav-filter.goml`, `heading-nav-folded.goml`, `heading-nav-large-intro.goml`, `heading-nav-markup.goml`, `heading-nav-normal-intro.goml`, `heading-nav-unusual-levels.goml`, `help.goml`, `highlighting.goml`, `move-between-pages.goml`, `redirect.goml`, `runner.rs`, `search.goml`, `sidebar-active.goml`, `sidebar-nojs.goml`, `sidebar-scroll.goml`, `sidebar.goml`, `theme.goml`, plus `books/` (fixture books for the GUI tests)
- Cost: 138 tokens (helper: stdin)

### 5.6 `examples/` folder listing
- Content: `nop-preprocessor.rs`, `remove-emphasis/` (containing `book.toml`, `mdbook-remove-emphasis/Cargo.toml`, `mdbook-remove-emphasis/src/main.rs`, `src/SUMMARY.md`, `src/chapter_1.md`, `test.rs`)
- Cost: 62 tokens (helper: stdin)
- Notes: Two reference plugins for 3rd-party-extension authors.

### 5.7 CLI guide pages: init / serve / test
- Content: `tests/fixtures/mdbook/guide/src/cli/init.md`, `serve.md`, `test.md`
- Cost: 1484 tokens (helper: `count-tokens.py guide/src/cli/init.md guide/src/cli/serve.md guide/src/cli/test.md`)
- Notes: User-facing prose for the three subcommands with the most user-visible behavior (init prompts, serve livereload, test rustdoc invocation).

### 5.8 `format/configuration/environment-variables.md`
- Content: full file
- Cost: 351 tokens (helper: `count-tokens.py guide/src/format/configuration/environment-variables.md`)
- Notes: User-facing mirror of `Config::update_from_env` (the `MDBOOK_*` env-var convention).

### 5.9 `format/summary.md`
- Content: full file
- Cost: 887 tokens (helper: `count-tokens.py guide/src/format/summary.md`)
- Notes: User-facing version of the SUMMARY.md grammar (paired with 3.9).

## Below-the-fold

The fixture is far larger than the 20k cap; the following are honestly omitted because their cost would crowd out higher-value surface. Approximate ranked content above is ~22k tokens.

### CLI implementations not shown
- **`cmd::watch::execute` body** (`src/cmd/watch.rs:21-79`, ~400 tokens) and the `cmd/watch/native.rs` + `cmd/watch/poller.rs` modules: clap definition is in 2.3, the rest is wiring discoverable via 1.7.
- **`cmd::clean::execute` + `Clean` struct** (`src/cmd/clean.rs`, ~800 tokens): mechanical; doesn't change behavior beyond removing the build directory.

### Driver internals not shown
- **`BookBuilder` field-level signatures** (`crates/mdbook-driver/src/init.rs`, ~590 tokens): callable via `MDBook::init` (covered in 2.7) which returns a `BookBuilder`; one-hop on demand.
- **`IndexPreprocessor::run` body** (`crates/mdbook-driver/src/builtin_preprocessors/index.rs:24-60`, ~280 tokens): the contract is in 1.13; the body is mechanical.
- **`Cargo.toml` `[package]`/`[features]`/`[[bin]]`** (lines 72-158, ~635 tokens): partly redundant with workspace.dependencies (3.16) and the workspace declaration (1.3).
- **`mdbook-core::utils` (mod.rs + fs.rs full)** (~1000 tokens): one-hop reads from any caller; the names that matter (`escape_html`, `read_to_string`, `write`, `path_to_root`) are visible in surrounding source.

### HTML renderer internals
- **Front-end vendored JS** (`crates/mdbook-html/front-end/js/clipboard.min.js`, `highlight.js`, `playground_editor/ace.js`, `searcher/elasticlunr.min.js`, `searcher/mark.min.js`): minified third-party libraries with no in-repo authorial content. Discoverable via 4.3.
- **Front-end fonts and images** (`*.woff2`, `*.png`, `*.svg`, license `.txt` files): binary or non-semantic.
- **Hand-authored `front-end/js/book.js`** (~1800 LOC): big and only relevant to A-JavaScript work; covered structurally by 4.3.
- **CSS theme files** (`chrome.css`, `general.css`, `print.css`, `variables.css`, `highlight.css`, `tomorrow-night.css`, `ayu-highlight.css`): one-hop reads via 4.4.
- **Handlebars templates** (`index.hbs`, `head.hbs`, `header.hbs`, `redirect.hbs`, `toc.html.hbs`, `toc.js.hbs`): one-hop reads via 4.4.
- **Tree/tokenizer/serializer bodies** (`crates/mdbook-html/src/html/tree.rs` ~9k tokens, `tokenizer.rs`, `serialize.rs`, `print.rs`, `hide_lines.rs`, `admonitions.rs` icon constants): too heavy; 2.20 + 4.5 give the structural pointers.
- **Handlebars helpers** (`crates/mdbook-html/src/html_handlebars/helpers/{toc,fontawesome,resources}.rs`): registered in `register_hbs_helpers` (visible in 4.1); discoverable via 4.6.
- **`hbs_renderer.rs` deeper bodies** (`render_chapter`, `render_404`, `render_print_page`, `make_data`, `combine_fragment_redirects`, `register_hbs_helpers`, `emit_redirects`/`emit_redirect`): over 4k tokens; deferred.
- **`html_handlebars/static_files.rs`, `search.rs` body, `theme/{fonts,playground_editor,searcher}.rs`**: deferred to one-hop reads via 4.6 / 4.9.

### Tests, fixtures, prose docs
- **`config.rs` test module** (lines 715–1209, ~3500 tokens): redundant with 2.16.
- **`book.rs` test module** (`crates/mdbook-core/src/book/tests.rs`): redundant with the struct definitions in 2.16.
- **`crates/mdbook-driver/src/builtin_preprocessors/links.rs` regex/parsing internals** (lines 50–137 + 170–936, plus `links/take_lines.rs`): over 8k tokens; the teaser in 3.13 + the LinkPreprocessor doc-comment in 1.12 cover the contract; bodies are one-hop.
- **`crates/mdbook-driver/src/builtin_preprocessors/cmd.rs` body** (after the type definition shown in 3.11): one-hop.
- **`crates/mdbook-driver/src/mdbook/tests.rs`, `crates/mdbook-core/src/utils/{html,toml_ext}.rs`**: small utility/test modules; not load-bearing for typical agent queries.
- **`crates/mdbook-summary/src/lib.rs` parser implementation** (lines 220+, ~7k tokens): grammar in 3.9 + structs in 3.10 cover the contract.
- **`tests/testsuite/*` per-feature fixture books** (each `book.toml` + `SUMMARY.md` + chapter `.md` files, dozens of folders): each is a self-contained tiny book demonstrating one feature; one-hop reads via 5.4 are far cheaper.
- **`tests/gui/books/sidebar-scroll/src/chapter_*.md`** (100 generated chapter stubs): no semantic content beyond their existence.
- **`tests/testsuite/*.rs` driver bodies**: `book_test.rs`, `cli.rs`, `config.rs`, etc. — large; the listing 5.4 + the README 5.3 are a better trade.

### Workspace tooling and large prose
- **`xtask/`, `mdbook-compare/`, `guide/guide-helper/`**: build- and release-supporting tooling; named in 1.4 for discoverability.
- **Top-level repo policy files** (`CODE_OF_CONDUCT.md`, `LICENSE`, `triagebot.toml`, `.github/ISSUE_TEMPLATE/*.yml`, `.github/renovate.json5`, `rustfmt.toml`, `package.json`, `eslint.config.mjs`, `.gitattributes`, `.gitignore`, `.git-blame-ignore-revs`, `.cargo/config.toml`): low signal; named in 1.1 / 3.17 / 3.18.
- **CHANGELOG.md** (~12k tokens): release-note prose; not load-bearing for code work.
- **Cargo.lock** (very large): never useful in-context.
- **CONTRIBUTING.md sections beyond 5.1**: prose around code-quality tooling, browser compatibility, highlight.js update procedure, and the release process — one-hop discoverable via 1.1.
- **`guide/src/format/configuration/renderers.md`** (~3800 tokens): largest single config doc, but it is a near-1:1 prose mirror of `HtmlConfig` (2.12), `Print`/`Fold`/`Playground`/`Code` (2.13), and `Search` (2.14). Cumulative redundancy isn't worth the cost; one-hop via 2.1.
- **`guide/src/format/mdbook.md`** (~2500 tokens): user-facing prose mirror of LinkPreprocessor (covered by 1.12 + 3.13).
- **`guide/src/format/markdown.md`** (~2100 tokens): markdown-extensions reference; covered structurally by 3.14 (`MarkdownOptions`) and 4.7 (admonition kinds).
- **`guide/src/cli/{README,arg-watcher,build,clean,completions,watch}.md`** (~1400 tokens): user-facing reference for the simpler subcommands; their flags are already documented by 2.3 (clap defs) and the implementation by 3.1 / `cmd::watch::execute` (in below-the-fold). Follow 2.1's pointer when needed.
- **`guide/src/format/configuration/general.md`** (~975 tokens): user-facing prose for `[book]`/`[build]`/`[rust]`; redundant with 2.10/2.11.
- **`guide/src/format/configuration/preprocessors.md`** (~790 tokens): redundant with 3.5/3.6.
- **`guide/src/for_developers/preprocessors.md`** (~1300 tokens): paired with the source in 3.12 (nop-preprocessor); follow 2.1's pointer when needed.
- **`guide/src/for_developers/backends.md`** (~2700 tokens): paired with `CmdRenderer` (3.11) and the `Renderer` trait (1.11); follow 2.1's pointer when needed.
- **Other guide pages** (`guide/src/misc/contributors.md`, `guide/src/404.md`, `guide/src/format/example.rs`, `guide/src/format/theme/{editor,index-hbs,syntax-highlighting}.md`, `guide/src/format/mathjax.md`, `guide/src/continuous-integration.md`, `guide/src/guide/{installation,creating,reading}.md`): end-user documentation reachable via 2.1.
