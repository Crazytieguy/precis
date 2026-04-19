# North Star — `mdbook` fixture

**Fixture root:** `test/fixtures/mdbook/`
**Revision pin:** `b8c90970` (recorded in `test/fixtures.rs`)

## Overview

`mdBook` is a CLI tool, written in Rust, that turns a directory of Markdown files into an online HTML book. It is a Cargo workspace with a strong split between two halves that are equally relevant to a coding agent landing on this fixture:

1. **A Rust workspace.** The root `Cargo.toml` declares a workspace with one binary crate (the `mdbook` CLI itself, defined at the workspace root) and seven library crates under `crates/`: `mdbook-core` (shared internal types), `mdbook-summary` (parser for the `SUMMARY.md` book-table-of-contents file), `mdbook-markdown` (a thin re-export wrapper around `pulldown-cmark` with mdbook's flag set), `mdbook-preprocessor` and `mdbook-renderer` (small, semver-stable trait crates that *third-party* preprocessors/renderers depend on), `mdbook-html` (the HTML renderer with its handlebars templates and front-end JS/CSS), and `mdbook-driver` (the high-level `MDBook` orchestrator that ties everything together). Two non-published helper crates live alongside: `mdbook-compare` (HTML diff utility) and `xtask` (developer command runner). The CLI is a thin shell over `mdbook-driver`.
2. **An mdbook documentation corpus.** `guide/` is itself a complete mdbook — its own `book.toml`, `src/SUMMARY.md`, and ~30 chapters of Markdown — that doubles as both the user guide *and* the canonical worked example of every feature the codebase implements. So Markdown files in `guide/src/` are not just documentation: they are *the spec* the Rust code aspires to implement, and the input format the parser reads. Many tasks on this fixture will require crossing between Rust source and matching guide chapters.

The plugin model is the architectural centerpiece. `Preprocessor` and `Renderer` are stable traits implemented in-process for the built-ins (`LinkPreprocessor`, `IndexPreprocessor`, `HtmlHandlebars`, `MarkdownRenderer`) and out-of-process via JSON-on-stdin/stdout for third-party plugins (`CmdPreprocessor`, `CmdRenderer`). Because of this, the trait crates `mdbook-preprocessor` and `mdbook-renderer` are intentionally tiny and over-documented — they're the contract exposed to the wider ecosystem.

Below is the priority order for an ideal precis snapshot. The structural parent of every batch is its enclosing folder (which must also be shown for the batch to make sense — Tier 1's directory tree is the implicit parent of all subsequent tiers).

---

## Tier 1 — Identity, workspace shape, crate purposes

The agent's first need: know what this repository is, how it's split into crates, what each crate's job is, and how they depend on each other.

### Batch 1.1 — Top-level README

**Content:** `test/fixtures/mdbook/README.md` (whole file).
**Tokens:** ~253.
**Why:** One paragraph that names the project and the canonical user-guide URL — the cheapest, highest-signal identity statement.

### Batch 1.2 — Workspace skeleton from root `Cargo.toml`

**Content:** Targeted line ranges from `Cargo.toml`:
- Lines 1-6: `[workspace] members = [...]` — the list of every crate in the workspace.
- Lines 21-26: `[workspace.package]` — edition, license, MSRV.
- Lines 42-50: the seven `mdbook-*` workspace dependencies with their `path = "crates/..."` declarations — the *internal* dependency catalog.
- Lines 72-89: the binary `[package]` block (name, description, keywords, exclude).
- Lines 129-134: `[features]` — `default = ["watch", "serve", "search"]`, plus the per-feature opt-in flag list.

**Tokens:** ~510.
**Why:** Tells the agent the workspace topology, the MSRV (1.88), the binary's identity, and which features gate which optional behavior — without dragging in the verbose external dependency table.
**Ordering note:** Sibling-after Batch 1.1.

### Batch 1.3 — Crate-level `//!` module-doc preambles

**Content:** Each crate's `src/lib.rs` opening doc comment block:
- `crates/mdbook-core/src/lib.rs:1-13` (~82) — "base support library, internal use only"
- `crates/mdbook-driver/src/lib.rs:1-66` (~524) — full overview including the two `MDBook::init` and `MDBook::load` examples and the cross-crate map
- `crates/mdbook-html/src/lib.rs:1-9` (~34) — "mdBook HTML renderer"
- `crates/mdbook-markdown/src/lib.rs:1-7` (~78) — "Markdown processing used in mdBook" (re-exports pulldown-cmark)
- `crates/mdbook-preprocessor/src/lib.rs:1-22` (~180) — "Library to assist implementing an mdbook preprocessor" + re-exports
- `crates/mdbook-renderer/src/lib.rs:1-21` (~169) — "Library to assist implementing an mdbook renderer" + re-exports
- `crates/mdbook-summary/src/lib.rs:1-6` (~53) — "Summary parser for mdBook"

**Tokens total:** ~1120.
**Why:** This is the cheapest way to give the agent a one-sentence purpose statement for every library crate, in the agent's own grammar (rustdoc comments). The driver's preamble in particular doubles as a how-to map for using the library API. There's a sibling ordering: `mdbook-driver` should appear before the others under `crates/`, because its docs explicitly reference the other six.

---

## Tier 2 — User guide table of contents and entry point

The mdbook user guide is both *documentation* and a *reference book* the codebase is meant to render. Its `SUMMARY.md` uses the very format that `mdbook-summary` parses, so it serves triple duty: TOC, worked example of the input format, and demo of the typical book layout.

### Batch 2.1 — `guide/src/SUMMARY.md` (full)

**Content:** `guide/src/SUMMARY.md` (44 lines).
**Tokens:** ~364.
**Why:** This is *the* navigational index for the guide corpus — every other markdown file under `guide/src/` is a chapter listed here. It also illustrates every feature of the `SUMMARY.md` grammar (`# Part Title`, prefix chapters, numbered nested chapters, draft chapter `[Draft chapter]()`, separator `-----------`, suffix chapter). Without this batch the rest of the guide is structurally unmoored.
**Structural parent:** `guide/src/` folder.

### Batch 2.2 — `guide/src/README.md` introduction

**Content:** `guide/src/README.md` (whole file).
**Tokens:** ~555.
**Why:** The prefix-chapter introduction. Lists the seven features of mdbook in a single bulleted glance ("Lightweight Markdown syntax", "Integrated search support", "Color syntax highlighting…", "Theme files", "Preprocessors", "Backends", "Written in Rust", "Automated testing of Rust code samples") with cross-links to the chapters that document each one. This is the agent's quickest path from "what is this" to "where do I look next."

---

## Tier 3 — Plugin contracts (the public extension API)

These two trait crates are the most stable, most-documented, smallest, and most strategically important Rust files in the workspace. Every third-party preprocessor or renderer in the wider ecosystem depends on these and *only* these. They define the protocol used both in-process and via the JSON-on-stdin/stdout out-of-process plugin model.

### Batch 3.1 — `mdbook-preprocessor` crate (full `lib.rs`)

**Content:** `crates/mdbook-preprocessor/src/lib.rs` (whole file, 85 lines).
**Tokens:** ~703.
**Why:** Contains the entire `Preprocessor` trait (`name`, `run`, `supports_renderer`), the `PreprocessorContext` struct (root, config, renderer, mdbook_version, chapter_titles), and the `parse_input()` helper for reading the JSON contract. This is small enough to show in full and there is no useful sub-batching — every public item is load-bearing.
**Structural parent:** `crates/mdbook-preprocessor/` folder.

### Batch 3.2 — `mdbook-renderer` crate (full `lib.rs`)

**Content:** `crates/mdbook-renderer/src/lib.rs` (whole file, 89 lines).
**Tokens:** ~736.
**Why:** Contains the `Renderer` trait (`name`, `render`), the `RenderContext` struct (version, root, book, config, destination, chapter_titles), and the `RenderContext::from_json()` constructor. Same logic as 3.1 — small, public, central, no useful sub-batching.

**Sibling ordering (3.1 before 3.2):** Preprocessors run before renderers in the build pipeline, and the documentation in `for_developers/preprocessors.md` is generally encountered before `for_developers/backends.md`. Either order works, but this one matches the data flow.

---

## Tier 4 — Core data model (the types both contracts work with)

The plugin contracts in Tier 3 are useless without the structs they pass around. These two files define the in-memory representation of a book.

### Batch 4.1 — `mdbook_core::book` module (full)

**Content:** `crates/mdbook-core/src/book.rs` (whole file, 289 lines).
**Tokens:** ~1980.
**Why:** Defines the entire book data model: `Book` (the tree, with `iter`, `chapters`, `for_each_mut`, `for_each_chapter_mut`, `push_item`, `new_with_items`), `BookItem` enum (`Chapter`, `Separator`, `PartTitle`), `Chapter` struct (name, content, number, sub_items, path, source_path, parent_names — with the critical README-vs-index `path` documentation), `SectionNumber`, and the `BookItems` depth-first iterator. Almost every preprocessor or renderer mutates these. Note the file is `#[allow(clippy::exhaustive_structs/enums)]` with the load-bearing rationale "This cannot be extended without breaking preprocessors" — that is itself an important constraint.
**Structural parent:** `crates/mdbook-core/src/` folder. Sibling-after `lib.rs` (which Tier 1 already showed).

### Batch 4.2 — `mdbook_summary` public surface and grammar

**Content:** `crates/mdbook-summary/src/lib.rs:1-145` — module preamble, `parse_summary()` doc + signature, `Summary`/`SummaryItem`/`Link` structs, and the `SummaryParser` documentation block containing the grammar definition in BNF.
**Tokens:** ~1100.
**Why:** The grammar is normative — it answers "what's a valid `SUMMARY.md`?" with precision the prose docs gloss. The `Summary` struct (prefix_chapters, numbered_chapters, suffix_chapters) is the exact shape produced by parsing and consumed by `mdbook-driver`'s loader. Skipping the parser implementation (lines 173+) and the test module saves ~7600 tokens for content the agent rarely needs eyes-on.

---

## Tier 5 — Driver entry points and CLI shape

The agent now knows the data model and the plugin contracts. Tier 5 shows how mdbook actually drives a build, and what subcommands the binary exposes.

### Batch 5.1 — `mdbook-driver` crate-level documentation

**Content:** `crates/mdbook-driver/src/lib.rs` (whole file, 132 lines).
**Tokens:** ~975.
**Why:** Already partially included in Tier 1 (its `//!` preamble is the largest of the seven). The full file adds the `compose_command` and `handle_command_error` helpers used by both `CmdPreprocessor` and `CmdRenderer` for shelling out to plugins (Shlex parsing, optional/required handling, NotFound diagnostics) — a self-contained piece of plugin infrastructure that doesn't fit elsewhere.
**Note:** This batch *replaces* the slice of this file that Tier 1 carried; do not double-count when computing what gets shown.

### Batch 5.2 — `MDBook` public surface (struct + key method signatures with rustdoc)

**Content:** Three line ranges from `crates/mdbook-driver/src/mdbook.rs`:
- Lines 1-225: imports, `MDBook` struct definition (root, config, book, renderers, preprocessors), and `load`/`load_with_config`/`load_with_config_and_summary`/`iter`/`init`/`build`/`preprocess_book`/`execute_build_process`/`with_renderer`/`with_preprocessor` — *with full bodies* because these are short and the bodies are themselves illustrative of the build pipeline.
- Lines 227-235: just the `test()` method docstring + signature (the body of `test_chapter`, lines 235-345, is a long rustdoc-shelling-out routine that's better fetched on demand).
- Lines 347-393: `build_dir_for` (with its illuminating ASCII-art comment showing the multi-renderer directory layout), `source_dir`, `theme_dir`.

**Tokens:** ~2170.
**Why:** This is the public face of the library — every CLI subcommand goes through one of these methods. Showing the bodies of `load`/`build`/`preprocess_book`/`execute_build_process` makes the data flow concrete: load → preprocess → render. Skipping the inner loop of `test_chapter` saves ~1200 tokens; the agent can fetch it when actually working on `mdbook test`.
**Structural parent:** `crates/mdbook-driver/src/` folder.

### Batch 5.3 — `mdbook` binary CLI dispatch

**Content:** `src/main.rs:1-91` — module imports, `main()` (clap match-subcommand dispatch including the `#[cfg(feature = "watch")]` and `#[cfg(feature = "serve")]` gating), `create_clap_command()` (the subcommand registration list), plus the version/about wiring.
**Tokens:** ~703.
**Why:** Authoritatively answers "what subcommands does `mdbook` expose?" — `init`, `build`, `clean`, `watch`, `serve`, `test`, `completions` — and shows which are gated behind which Cargo features. The agent needs this to map a user-facing command name to a `cmd::*::execute()` implementation. Skipping `init_logger`/`get_book_dir`/`open` (lines 92-149) saves ~460 tokens of plumbing.
**Structural parent:** `src/` folder.

---

## Tier 6 — HTML renderer architecture, theme, and `HtmlConfig`

The HTML renderer is by far the largest sub-system (`mdbook-html` is ~30k tokens of Rust plus ~8k of handlebars templates plus megabytes of woff2/CSS/JS assets). Tier 6 shows just enough to navigate it.

### Batch 6.1 — HTML crate orchestration map

**Content:**
- `crates/mdbook-html/src/lib.rs` (34) — the four-module declaration: `html`, `html_handlebars`, `theme`, `utils`; sole pub export is `HtmlHandlebars`.
- `crates/mdbook-html/src/html/mod.rs` (851) — the rendering pipeline rustdoc explaining "1. pulldown-cmark events → 2. tree → 3. serialize", plus `HtmlRenderOptions` struct, `render_markdown()`, `build_trees()`, and `ChapterTree` struct definitions.
- `crates/mdbook-html/src/html_handlebars/mod.rs` (34) — the four-module declaration: `hbs_renderer`, `helpers`, `search` (cfg-gated), `static_files`.
- `crates/mdbook-html/src/html_handlebars/helpers/mod.rs` (19) — three handlebars helpers: `fontawesome`, `resources`, `toc`.

**Tokens:** ~940.
**Why:** Together these four small files are the navigation map for ~30k tokens of HTML-renderer code. The rendering-pipeline rustdoc in particular tells the agent which file to open for each transformation phase. Skipping the body files (`hbs_renderer.rs` 5419, `search.rs` 3148, `static_files.rs` 2591, `tree.rs` 9253, etc.) defers thousands of tokens until the agent actually needs to dig in — and the orchestration map tells them where.
**Structural parents:** `crates/mdbook-html/src/`, then its `html/`, `html_handlebars/`, and `html_handlebars/helpers/` subfolders.

### Batch 6.2 — `Theme` struct and built-in static asset list

**Content:** `crates/mdbook-html/src/theme/mod.rs:1-65`.
**Tokens:** ~806.
**Why:** This range contains *every* `static FOO: &[u8] = include_bytes!("../../front-end/...");` line — i.e., the authoritative list of templates, CSS files, JS files, fonts, and favicons that ship inside the binary, paired with their on-disk locations under `front-end/`. It also defines the `Theme` struct whose fields are *exactly* the user-overridable items from `theme/` directories. This is the single best summary of "what files make up an mdbook theme" — much better than a folder listing of `front-end/` alone, because it pairs filenames with their semantic role.
**Structural parent:** `crates/mdbook-html/src/theme/` folder.

### Batch 6.3 — `HtmlConfig` (the user-facing HTML output options)

**Content:** `crates/mdbook-core/src/config.rs:446-572` — the `HtmlConfig` struct definition with rustdoc on every field, and its `Default` impl.
**Tokens:** ~1153.
**Why:** This is the canonical list of every `[output.html]` knob in `book.toml` — theme path, default/preferred-dark theme, smart-punctuation, definition-lists, admonitions, mathjax-support, additional-css/js, fold/playground/code/print sub-tables, search, git-repository-url/icon, edit-url-template, redirect map, hash-files, sidebar-header-nav, etc. The rustdoc on each field is more terse than `format/configuration/renderers.md` (which is in Tier 8) but covers the same surface; ideally both are shown. Skipping the surrounding `Config`/`BookConfig`/`BuildConfig`/`RustConfig` definitions (and the entire 477-line tests module) saves ~7500 tokens.
**Structural parent:** `crates/mdbook-core/src/` folder; sibling-after Batch 4.1.

### Batch 6.4 — `mdbook-markdown` crate (full)

**Content:** `crates/mdbook-markdown/src/lib.rs` (whole file, 62 lines).
**Tokens:** ~452.
**Why:** Trivial in size, central in role: it's the wrapper that decides which `pulldown-cmark` `Options` flags mdbook enables (TABLES, FOOTNOTES, STRIKETHROUGH, TASKLISTS, HEADING_ATTRIBUTES, conditionally SMART_PUNCTUATION, DEFINITION_LIST, GFM-for-admonitions). Re-exports `pulldown_cmark`, so a third-party preprocessor that wants to parse markdown the same way mdbook does depends on this crate. Plus the public `MarkdownOptions` struct.
**Structural parent:** `crates/mdbook-markdown/` folder.

---

## Tier 7 — Built-in plugins and a canonical extension example

These complete the picture from Tier 3's contracts: in addition to the `Preprocessor`/`Renderer` traits, mdbook ships these implementations.

### Batch 7.1 — Built-in plugin docstrings and module map

**Content:**
- `crates/mdbook-driver/src/builtin_preprocessors/mod.rs` (45) — re-exports `CmdPreprocessor`, `IndexPreprocessor`, `LinkPreprocessor` and declares the three sibling modules.
- `crates/mdbook-driver/src/builtin_preprocessors/links.rs:18-42` (~234) — the `LinkPreprocessor` rustdoc listing every supported helper (`{{# include}}`, `{{# rustdoc_include}}`, `{{# playground}}`, `{{# title}}`) and the struct definition.
- `crates/mdbook-driver/src/builtin_preprocessors/index.rs:8-22` (~102) — the `IndexPreprocessor` docstring + struct (README → index conversion).
- `crates/mdbook-driver/src/builtin_preprocessors/cmd.rs:9-19` (~82) — `CmdPreprocessor` docstring (the third-party-plugin shell-out).
- `crates/mdbook-driver/src/builtin_renderers/mod.rs:1-30` (~198) — module preamble noting "the HTML renderer can be found in mdbook_html" and the `CmdRenderer` rustdoc/struct.
- `crates/mdbook-driver/src/builtin_renderers/markdown_renderer.rs:6-22` (~90) — `MarkdownRenderer` docstring + struct (debugging-oriented Markdown output).

**Tokens:** ~750.
**Why:** Tells the agent exactly which preprocessors/renderers ship and what each does, without dragging in their (combined ~10k-token) implementation bodies. Critically, the `LinkPreprocessor` docstring is the first answer to "where is `{{#include}}` implemented?" — currently the most-used mdbook helper.
**Sibling ordering:** Preprocessors before renderers (matches build-pipeline order). Within each, `mod.rs` first (the module map), then individual plugins.

### Batch 7.2 — `examples/nop-preprocessor.rs`

**Content:** `examples/nop-preprocessor.rs` (whole file, 165 lines).
**Tokens:** ~1103.
**Why:** This is the *canonical* boilerplate that the user guide's `for_developers/preprocessors.md` chapter directs readers to copy and adapt. It demonstrates: clap-app skeleton with the `supports` subcommand, version-compatibility check using `semver`, `parse_input` → `run` → `serde_json::to_writer` round-trip, and the trick of using a `[preprocessor.nop-preprocessor.blow-up]` config key for testing. Without this, the docs in Tier 8.2 are abstract; with it, an agent has a complete starting point.
**Structural parent:** `examples/` folder.

---

## Tier 8 — User-facing reference docs (mdbook syntax + extension protocol)

The Rust source documents *how* things work; the user guide chapters document *what the user sees and writes*. These two are the most directly task-relevant chapters of the guide because they cover (a) the input syntax mdbook actually consumes and (b) the extension protocol — the two questions most likely to come up.

### Batch 8.1 — `format/mdbook.md` (canonical syntax reference)

**Content:** `guide/src/format/mdbook.md` (whole file, 357 lines).
**Tokens:** ~2525.
**Why:** This chapter is *the* normative reference for every mdbook-specific Markdown extension: hide-lines (`# `, custom prefixes, `hidelines=!!!` overrides), the Rust playground, code-block attributes (`editable`, `noplayground`, `mdbook-runnable`, `ignore`, `should_panic`, `no_run`, `compile_fail`, edition annotations), `{{#include}}` with line-range and anchor variants (`ANCHOR:` / `ANCHOR_END:`), `{{#rustdoc_include}}` (the partial-with-hidden-rest trick), `{{#playground}}`, `{{#title}}`, the `class="left"`/`"right"`/`"hidden"` HTML conventions, and Font-Awesome icon syntax. This file pairs naturally with the `LinkPreprocessor` docstring in Batch 7.1 — together they cover both what the syntax means and where it's implemented.
**Structural parent:** `guide/src/format/` folder; sibling-after `guide/src/format/summary.md` (which is below-the-fold).

### Batch 8.2 — `for_developers/preprocessors.md`

**Content:** `guide/src/for_developers/preprocessors.md` (whole file, 109 lines).
**Tokens:** ~1264.
**Why:** Documents the JSON-on-stdin-and-stdout protocol that `CmdPreprocessor` invokes. Covers the `[preprocessor.foo]` table convention, the two-call protocol (`supports <renderer>` exit-code + JSON body), the `[context, book]` JSON shape, hints for in-process implementations using `mdbook-preprocessor`, and a complete worked Python preprocessor example. References both Batch 7.2 (the nop-preprocessor example) and the `examples/remove-emphasis/` example (in below-the-fold). This is the single most-asked-for piece of documentation when adding extensions.
**Structural parent:** `guide/src/for_developers/` folder.

---

## Below-the-fold

Items considered and *deliberately* omitted from the ranked tiers, with one-line justifications. An agent that hits the cap and needs more should reach for these in roughly this order.

### Guide chapters not shown in full

- **`guide/src/for_developers/backends.md` (~2726)** — the renderer counterpart to Batch 8.2; same protocol, written as a tutorial building a word-count backend. Cut because the `Renderer` trait (3.2), `CmdRenderer` (7.1), and the driver's `compose_command`/`handle_command_error` (5.1) collectively show the protocol from the implementation side, and the For-Developers README (Tier 2's SUMMARY links it) gives the conceptual hook.
- **`guide/src/format/configuration/renderers.md` (~3781)** — exhaustive prose reference for every `[output.html.*]` option. Mostly redundant with `HtmlConfig`'s rustdoc (Batch 6.3) plus the sub-config struct definitions; fetch when an agent needs the *prose explanation* of a specific knob like `output.html.search.boost-title`.
- **`guide/src/format/configuration/general.md` (~974)**, **`preprocessors.md` (~792)**, **`environment-variables.md` (~351)** — config reference; covered structurally by `BookConfig`/`BuildConfig`/`RustConfig` in `config.rs:313-444` and by `Config::update_from_env()`'s rustdoc.
- **`guide/src/format/markdown.md` (~2111)** — overview of CommonMark + GFM extensions mdbook supports; the actual flag set is in `mdbook-markdown` (Batch 6.4), which is more authoritative.
- **`guide/src/format/summary.md` (~887)** — prose explanation of the `SUMMARY.md` format. Redundant with the BNF grammar in Batch 4.2 and the worked example in Batch 2.1.
- **`guide/src/format/theme/*.md` (~2582 across four files)** — `index-hbs.md`, `syntax-highlighting.md`, `editor.md`, `README.md`. The `Theme` struct in Batch 6.2 already lists every overridable file; fetch these when actually theming.
- **`guide/src/cli/*.md` (~2881 across nine files)** — per-subcommand prose reference. The clap definitions in `src/cmd/{init,build,serve,watch,test,clean}.rs` are more authoritative for argument shapes; fetch these for usage-style narrative.
- **`guide/src/guide/installation.md`, `creating.md`, `reading.md`, `continuous-integration.md`, `misc/contributors.md`, `404.md`** — beginner orientation. Useful for end-user-facing tasks but rarely for code work.
- **All-guide H1+H2+H3 heading skeleton (~3093)** — could be useful as a navigation aid, but the `SUMMARY.md` (Batch 2.1) already provides chapter-level navigation, and prose content is more valuable than headings within the budget.

### Rust source not shown

- **`crates/mdbook-html/src/html/tree.rs` (~9253)** and **`hbs_renderer.rs` (~5419)** — the largest single Rust files. Implementation of the markdown-event → tree → handlebars-rendering pipeline. Tier 6.1's orchestration map points the agent at them by name; opening them is a follow-up tool call away.
- **`crates/mdbook-html/src/html_handlebars/search.rs` (~3148)** — elasticlunr-rs index construction. Tier 6 mentions search is `cfg(feature = "search")`-gated; the `Search` struct fields in `config.rs` cover the user-facing knobs.
- **`crates/mdbook-html/src/html_handlebars/static_files.rs` (~2591)** — content-addressed static asset writing with `{{ resource }}` directive resolution.
- **`crates/mdbook-html/src/html/{print,admonitions,hide_lines,serialize,tokenizer}.rs`** — focused implementation modules for individual pipeline phases. Names + roles are visible from `html/mod.rs` (Batch 6.1).
- **`crates/mdbook-html/src/theme/{fonts,playground_editor,searcher}.rs`** — bundles of `&[u8]` constants for embedded asset chunks (~900 tokens combined). Names tell the story.
- **`crates/mdbook-driver/src/builtin_preprocessors/links.rs:43-end` (~6800 of 6818)** — full `LinkPreprocessor` implementation including link parsing, `find_links` regex iteration, `LinkType` variants, and a giant test suite. The 25-line docstring (Batch 7.1) plus the `format/mdbook.md` syntax reference (Batch 8.1) cover the user-visible surface.
- **`crates/mdbook-driver/src/builtin_preprocessors/links/take_lines.rs` (~2421)** — `take_lines`, `take_anchored_lines`, `take_rustdoc_include_lines`, `take_rustdoc_include_anchored_lines`. Internal implementation detail of the `{{#include}}` line-range/anchor logic.
- **`crates/mdbook-driver/src/builtin_preprocessors/index.rs:23-end` (~470)** and **`cmd.rs:20-end` (~1080)** — implementation bodies; the docstrings in Batch 7.1 cover what each does at the user level.
- **`crates/mdbook-driver/src/load.rs` (~2315)** — `load_book`, `create_missing`, `load_book_from_disk`, `load_chapter`. The relevant facts (BOM stripping, missing-file synthesis, draft chapters) are visible from `Chapter::new`/`Chapter::new_draft` rustdocs (Batch 4.1) and `BuildConfig::create_missing`.
- **`crates/mdbook-driver/src/init.rs` (~1054)** — `BookBuilder` for `mdbook init`. The driver lib.rs preamble (Tier 1.3) shows the `MDBook::init(...).create_gitignore(true).with_config(cfg).build()` usage which is the load-bearing surface.
- **`crates/mdbook-driver/src/builtin_renderers/markdown_renderer.rs:23-end` (~140)** — trivial implementation; the docstring (Batch 7.1) is enough.
- **`crates/mdbook-core/src/config.rs:1-44 module-doc, 46-309 (Config impl), 313-444 (BookConfig/BuildConfig/RustConfig/RustEdition/TextDirection), 574-712 (Print/Fold/Playground/Code/Search/SearchChapterSettings)` (~3500)** — the rest of `Config`. Important but secondary to `HtmlConfig` (Batch 6.3); the omitted sub-types are referenced from `HtmlConfig` field types and most are short structs whose names are self-describing.
- **`crates/mdbook-core/src/utils/{fs,html,toml_ext,mod}.rs` (~3700)** — `fs::write/read_to_string/copy_files_except_ext/path_to_root`, `escape_html/escape_html_attribute`, the `static_regex!` macro, the `TomlExt` extension trait. Pure utilities; the agent can search-and-find when needed.
- **`src/cmd/{build,serve,watch,test,clean,init,command_prelude}.rs` and `src/cmd/watch/{native,poller}.rs` (~6300 combined)** — per-subcommand clap definitions and execute functions. The big-picture dispatch is in `src/main.rs` (Batch 5.3); per-subcommand bodies are mostly clap arg construction + a one-line call into `MDBook::*`. The serve/watch implementations carry the most novel logic (axum HTTP server, websocket reload, native vs poll filesystem watching).
- **All `#[cfg(test)] mod tests` in core/summary/links** — quality signal for an agent considering test coverage but mostly noise for correctness questions.

### Tooling, CI, and metadata

- **`Cargo.lock` (~23040)** — exact version pins; only relevant for dependency-resolution debugging.
- **`CHANGELOG.md` (~24285)** — hugely valuable for "when was X added?" questions, but extremely repetitive and best fetched on demand.
- **`CONTRIBUTING.md` (~2898)** — workflow doc (testing, rustfmt, clippy, GUI tests, npm lint, release process). Useful for "how do I run the test suite?" but not for understanding the code.
- **`crates/xtask/src/{main,changelog}.rs` (~3200)** — the developer command runner (`cargo xtask test-all`, `bump`, `changelog`). Helpful when modifying CI; otherwise off-path.
- **`crates/mdbook-compare/src/main.rs`** — diff-two-mdbook-versions tool; used by maintainers, not by ordinary tasks.
- **`tests/testsuite/README.md` (~693), `book_test.rs` (~4886), `main.rs` (~133)** — the integration test harness (snapbox-driven `BookTest`). Worth showing if tasks involve writing tests; covered by SUMMARY.md heading-level mention. The ~140 fixture sub-directories under `tests/testsuite/*/` and `tests/gui/books/*/` are each a tiny book with `book.toml` + `src/SUMMARY.md` + a few `.md` files; they are *test data*, not source code. Their existence tells you the test harness uses fixture-directory snapshotting.
- **`tests/gui/*.goml` (~19 GUI test scripts)** and **`tests/gui/runner.rs`** — browser-ui-test harness for headless-Chrome end-to-end tests. Off-path for most tasks.
- **`.github/workflows/{main,deploy,update-dependencies}.yml`, `ci/*.sh`, `.github/ISSUE_TEMPLATE/*.yml`, `.github/renovate.json5`, `triagebot.toml`, `.gitignore`, `.gitattributes`, `.git-blame-ignore-revs`, `eslint.config.mjs`, `package.json`, `rustfmt.toml`, `.cargo/config.toml`, `CODE_OF_CONDUCT.md`, `LICENSE`** — repo metadata and CI plumbing. Their *names* may merit a one-line directory-listing batch in some contexts, but their contents are rarely task-relevant.
- **`crates/mdbook-html/front-end/{js,css,playground_editor,searcher,fonts,images}/`** — embedded front-end assets. The bundled `book.js` (~37 KB), `highlight.js`, `clipboard.min.js`, ace.js editor, elasticlunr.min.js, mark.min.js are the runtime client-side code; the CSS files are the theme stylesheets; the `.woff2` files and `*.png/svg` favicons are binary assets. The `Theme` struct (Batch 6.2) authoritatively names them; their *contents* are out-of-scope for any precis snapshot.
- **`crates/mdbook-html/front-end/templates/{index,head,header,redirect,toc.html,toc.js}.hbs` (~8127 combined)** — handlebars templates that produce every page. `index.hbs` (~3491) is the master page template; `toc.js.hbs` (~3942) generates the sidebar TOC at runtime. Worth knowing they exist (named in Batch 6.2); contents are only relevant for custom-theme work.
- **`examples/remove-emphasis/`** — a second worked preprocessor example (referenced by `for_developers/preprocessors.md` for the `pulldown-cmark-to-cmark` round-trip pattern). Slightly more advanced than the nop preprocessor (Batch 7.2) but covers the same protocol, so cut for redundancy.
- **`guide/guide-helper/src/{lib,main}.rs`** — a tiny custom preprocessor that the guide itself uses (`[preprocessor.guide-helper]` in `guide/book.toml`) to inject `{{ mdbook-version }}` substitutions. A meta-amusing self-application but specialized.
