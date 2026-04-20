# Batch ontology (Stage 4)

Synthesis of recurring batch shapes across the three frozen north stars
(`tests/north-stars/{log,anyhow,mdbook}.md`). Frozen reference for Stage 7
walker design — read at the start of every implementation session.

Conventions used below:
- Citations of batches use `<fixture>:<batch-number>` (e.g. `log:1.5`).
- "Cross-language" means a category whose value heuristic does not depend on
  any particular grammar; "language-specific" means one whose detection or
  ranking depends on a parser for a particular language.

## 1. Recurring batch categories

### 1.1 Filesystem listings

#### Repo-root entry listing
- One-line listing of the immediate children of the seed root, folders
  suffixed `/`. Examples: `log:1.1`, `anyhow:1.1`, `mdbook:1.1`.
- **Priority: earliest.** "Highest-priority orientation" (`log:1.1`),
  "cheapest possible orientation" (`anyhow:1.1`), "top of the orientation
  pyramid" (`mdbook:1.1`). Cost is tiny (34–60 tokens) and value is large:
  catastrophic-omission risk if absent (the agent doesn't know what's there
  to ask about).
- **Predecessors: none.** Always first or near-first.
- **Value signals (within category):** value scales with the *information
  density* of the names. A repo whose root contains an unusual entry
  (`rfcs/` in `log:1.1`, `crates/` next to `src/` in `mdbook:1.1`,
  `build.rs` in `anyhow:1.1`) gets called out in the notes — those entries
  drive priority. Routine names (`LICENSE-*`, `.gitignore`) carry less
  weight but are cheap to include.

#### Source-tree subdirectory listings
- One-line listings of files (and possibly subfolders) inside a known source
  directory. Examples: `log:1.2` (`src/` and `src/kv/` combined),
  `anyhow:1.2` (`src/`), `mdbook:1.5` (`src/` + `src/cmd/`),
  `mdbook:1.3` (`crates/`), `mdbook:2.21` (three nested
  `mdbook-html/src/...` listings combined).
- **Priority: early, immediately after the root listing.** They "complete
  the file-tree orientation" (`log:1.2`).
- **Predecessor:** the parent listing they descend from
  (`log:1.2 → 1.1` is implicit; `mdbook:5.5 → 5.4` is explicit).
- **Value signals:** higher when the file names are themselves an API map
  ("the `test_*.rs` names are themselves a near-perfect API map"
  `anyhow:1.3`; "one file per CLI subcommand" `mdbook:1.5`). A directory of
  ~10 named files often beats a longer prose summary at the same cost.

#### Multi-listing rolls
- Several adjacent listings rendered as one batch when each is small.
  Examples: `mdbook:2.21` (three nested folders), `mdbook:5.11`
  (`ci/` + `.github/workflows/`), `mdbook:5.12` (`front-end/` plus two
  subfolders).
- Permitted only when the listings are conceptually one slice
  (sibling or descendant directories the agent will want together).

### 1.2 Manifest / package identity

#### Crate identity (name / version / description)
- Tight slice of the package manifest holding name, version, edition, MSRV,
  and one-sentence description. Examples: `log:1.3` (`Cargo.toml:4` +
  `:10-12`), `anyhow:1.5` (`Cargo.toml:1-13`), `mdbook` covers this via
  `mdbook:1.7` (workspace header) plus `mdbook:3.11` (binary `[package]`).
- **Priority: very early, before bodies.** "Needed before anything else
  makes sense; cheap and high-leverage" (`log:1.3`).
- **Predecessors:** none, but typically appears just after the root
  listing.
- **Value signals:** include the `description` line — it is often the
  cheapest single semantic anchor in the snapshot.

#### Dependency lists
- Runtime deps + dev-deps from the manifest. Examples: `log:1.13`,
  `anyhow:1.11` (combined with features), `mdbook:5.8` (`workspace.dependencies`).
- **Priority: mid (1.x late or 2.x).** Useful for "which crate handles X?"
  but not load-bearing for understanding the API surface itself.
- **Predecessor:** identity / features (split decisions vary).
- **Value signals:** higher when external crates leak into the public API
  (`pulldown-cmark` re-exported by `mdbook-markdown` — `mdbook:3.5`); lower
  when deps are internal-only.

#### Feature flags
- The `[features]` table. Examples: `log:1.11` (full features matrix
  including the `max_level_*` cascade), `anyhow:1.11` (combined with deps),
  `mdbook:3.12` (binary's optional deps + `default = ["watch", "serve",
  "search"]`).
- **Priority: early-mid (1.x).** "Catastrophic-omission risk" `log:1.11`,
  `anyhow:1.11` — without them the agent will be lost on `#[cfg(feature =
  …)]` gates throughout `src/`.
- **Predecessor:** identity.
- **Value signals:** higher when features gate substantial behaviour
  (`log`'s 12 `*max_level_*` toggles); lower when they only flip a single
  optional dep.

### 1.3 Top-level prose / README

#### README headline / install snippet
- Tightest "what is this project" + how-to-add-it. Examples: `log:2.4`
  (`README.md:1-56`, the library-author example), `anyhow:1.4`
  (`:9-17`, one-sentence + `anyhow = "1.0"`), `mdbook:1.2` (`:7`, one line).
- **Priority: very early.** When tight (≤20 tokens), it "eliminates the
  catastrophic 'what even is this repo?' failure mode at almost zero cost"
  (`mdbook:1.2`). When richer, it doubles as the canonical use example.
- **Predecessors:** none.
- **Value signals:** higher when prose is concise and not duplicated by
  rustdoc; if the same text appears in `lib.rs` rustdoc the README slice
  may be demoted in favour of the rustdoc one (or the whole crate-doc
  body deferred to below-the-fold — `anyhow:lib.rs:14-208` is below-fold
  precisely because it mirrors the README).

#### README tutorial slabs
- The walkthrough body of the README split into a few coherent ranges.
  Examples: `anyhow:2.2` (`README.md:21-67`, `?` + `.context`), `anyhow:2.5`
  (`:68-122`, downcasting + `RUST_BACKTRACE` + `thiserror` + macros),
  `anyhow:2.7` (`:124-180`, no-std + comparison + license).
- **Priority: 2.x — second tier, after declarations are anchored.**
- **Predecessor:** the headline (e.g. `anyhow:2.2 → 1.4`); each subsequent
  slab takes its preceding slab as predecessor (`anyhow:2.5 → 2.2`).
- **Value signals:** ranks against the rustdoc analogue. `log:2.4` (README
  library-author example) and `log:3.3` (the in-rustdoc parallel) coexist
  because each adds one detail (README has the `[dependencies]` snippet;
  rustdoc has the `set_logger` once-only warning).

### 1.4 CHANGELOG / release-notes head
- First few entries of `CHANGELOG.md` (current version + 1–2 prior).
  Examples: `mdbook:6.2` (`CHANGELOG.md:1-23`); `log` defers the whole file
  to below-the-fold but explicitly pre-promotes the 0.4.29/0.4.28 head.
- **Priority: late (6.x in `mdbook`).** Useful for "what shipped recently?"
  but not for steady-state reading.
- **Value signals:** older entries decay sharply; the head pays for itself,
  the tail rarely does.

### 1.5 CI / contributor docs

#### CI workflow slices
- Selected slices of `.github/workflows/*.yml`, focused on the test matrix
  and any unusual cfg flags. Examples: `anyhow:7.3` (test job + minimal
  versions + clippy/miri, three non-contiguous ranges with elision
  markers); `log` defers the whole workflow to below-the-fold with a
  one-line note about the matrix shape.
- **Priority: late (7.x or below-the-fold).** Promote when nightly/MSRV
  matters or when CI invokes unusual flags.
- **Value signals:** higher when the matrix surfaces toolchain pins or
  unusual feature combos (`anyhow`'s `--cfg=anyhow_nightly_testing`); the
  routine `actions/checkout@vN` rows are pure boilerplate and stay
  below-the-fold.

#### Contributor / xtask command index
- A slice of `CONTRIBUTING.md` enumerating the canonical
  `cargo test`/`cargo fmt`/etc. commands. Example: `mdbook:5.3`
  (`CONTRIBUTING.md:101-152`).
- **Priority: 5.x — useful, not load-bearing.**
- **Value signals:** higher when the project has a non-obvious umbrella
  command (`cargo xtask test-all`); lower when the commands are the
  defaults (`cargo test`).

### 1.6 Build script (`build.rs`)
- The `fn main` decision tree of `build.rs`. Example: `anyhow:6.1`
  (`build.rs:1-97`, the cfg cascade for `error_generic_member_access`,
  `std_backtrace`, etc.). Helper functions deferred (below-fold).
- **Priority: 6.x — gated on whether build-script cfgs leak into `src/`.**
- **Predecessors:** none direct, but cited as predecessor of files whose
  cfgs it emits (`anyhow:6.2 → 6.1`).
- **Value signals:** higher when `src/` references many `#[cfg(...)]`
  predicates emitted by the build script.

### 1.7 Public type declarations

#### Bare struct / enum declaration (no body, doc elided)
- Single-paragraph slice containing only the struct/enum keyword line plus
  fields and (for enums) variants. Examples: `anyhow:1.6`
  (`pub struct Error { inner: Own<ErrorImpl> }` — five lines, **doc above
  is elided to be ranked separately**), `anyhow:1.10` (`Chain<'a>`),
  `mdbook:1.9` (`MDBook` fields), `mdbook:2.3` (`Book`), `mdbook:2.4`
  (`BookItem`).
- **Priority: 1.x — declarations are the cheapest semantic anchor for a
  type.** "Splitting the declaration off from its rustdoc is the
  load-bearing move" (`anyhow:1.6`).
- **Predecessors:** the file-tree listing that names the file is sufficient.
- **Value signals:** smaller (≤200 tokens) declarations rank highest;
  field names that themselves carry meaning (`MDBook { config, book,
  renderers, preprocessors }`) raise priority.

#### Enum with variants + brief doc
- Enum declaration shown intact when the variant list is itself the
  vocabulary the rest of the API uses. Examples: `log:1.8` (`LevelFilter`),
  `log:1.9` (`Level`).
- **Priority: 1.x — early.**
- **Value signals:** higher when the variant ordering or discriminant
  comment is load-bearing (`log:1.9`'s "These … line up with the
  discriminants for `LevelFilter` below" comment is explicitly cited as
  load-bearing).

### 1.8 Public trait declarations
- Trait header + method signatures, no bodies, with the per-method
  "for implementors" notes that aren't derivable from the signature.
  Examples: `log:1.10` (`Log` trait), `anyhow:1.8` (`Context` trait),
  `mdbook:1.10` (`Preprocessor`), `mdbook:1.11` (`Renderer`).
- **Priority: 1.x — early.** These are the abstractions every backend or
  extension implements; the snapshot must surface them.
- **Predecessors:** the file/module listing.
- **Value signals:** higher when the trait has only 1–3 methods (cheap to
  include in full); higher when it's the *implementation contract* for
  third-party code (every `mdbook` plugin implements `Renderer` or
  `Preprocessor`); the *un-derivable* doc notes (`log:1.10`'s "enabled is
  *not* necessarily called before log") are why trait declarations
  outrank trait-impl bodies.

### 1.9 Macro declarations

#### Macro-name list
- Rendered list of `#[macro_export] macro_rules!` names — no bodies, no
  signatures. Example: `log:1.4` (the seven public log macros).
- **Priority: 1.x — early.** Tells the agent the macro surface exists.
- **Value signals:** ranks above macro signatures when the names alone
  let the agent guess the call shape (`error!`, `info!`).

#### Macro signature + arms (whole `macro_rules!`)
- Single full `macro_rules!` block including all arms and the doc.
  Examples: `anyhow:1.12` (`bail!`), `anyhow:1.13` (`anyhow!`),
  `log:2.9` (`error!` as representative), `log:3.1` (`log!` dispatcher),
  `log:4.6` (`log_enabled!`).
- **Priority: split-dependent.** Tiny macros ship in 1.x; longer
  representative ones ship in 2.x or 3.x with the compact ones earlier.
- **Predecessors:** the macro-name list.
- **Value signals:** higher when one macro's arms imply a family
  (`log:2.9` `error!` + the elision marker for `warn!`/`info!`/`debug!`/
  `trace!` is enough; the four siblings stay below-the-fold).

#### `#[cfg(doc)]` user-facing macro skeleton
- Doc-only arm of a macro whose real implementation is a giant token-tree
  parser. Example: `anyhow:1.14` (`ensure!` `#[cfg(doc)]` form).
- **Priority: 1.x — early; the only readable view of the macro.**
- **Predecessor:** the giant parser is below-the-fold; this skeleton
  carries the surface.
- **Value signals:** dramatically higher than the full parser body (which
  in `anyhow:ensure.rs:103-883` would single-handedly exceed the 20k cap).
  An honest snapshot demands surfacing this view.

### 1.10 Function / method signatures (no body)

#### Constructor signature group
- Several constructor signatures from the same `impl` block, with a `…`
  elision marker between them, no rustdoc, full where-clauses.
  Examples: `anyhow:2.4` (`Error::new`/`msg`/`from_boxed` — three ranges
  joined), `mdbook:3.1` (`make_subcommand` for five subcommands joined).
- **Priority: 2.x — after the type declaration.**
- **Predecessor:** the type's struct/enum declaration.
- **Value signals:** signatures with informative bounds rank higher;
  showing 3–5 constructors at once beats showing each separately for the
  same total cost.

#### Consumer-method signature group
- Same shape, for consumer methods. Example: `anyhow:2.6` (10
  `Error::is`/`downcast`/`chain`/`backtrace`/`into_boxed_dyn_error` ranges
  joined with elision markers).
- **Priority: 2.x — early in the second tier.**
- **Value signals:** the rustdoc on individual methods can stay on disk;
  the signature group itself answers "does X have method Y?" without a
  follow-up.

#### Setter / accessor blocks (intact)
- Builder or accessor cluster shown intact because each setter is so tiny
  that splitting wastes more context than it saves. Examples: `log:4.1`
  (`RecordBuilder` + every setter + `build()`), `log:3.5` (`Record`
  accessors), `log:3.10` (`Level` impl methods).
- **Priority: 3.x–4.x.**
- **Value signals:** "Setters are tiny so a signature-only split would
  barely save tokens — kept intact" (`log:4.1`).

### 1.11 Doc-comment slabs (rustdoc / module doc)

#### Crate-doc lede (the "what is this crate" paragraph)
- Opening rustdoc paragraph from the crate root. Examples: `log:1.5`
  (`src/lib.rs:11-19`, the *facade* concept), `log:1.6` (`:20-26`, the
  log-request data model). `anyhow` keeps the equivalent below-the-fold
  because the README mirrors it.
- **Priority: 1.x.**
- **Value signals:** higher when the rustdoc is *not* a near-duplicate of
  the README. If it duplicates, prefer one source and demote the other.

#### Doc rustdoc on a single public type
- The long rustdoc above one `pub struct` / `pub trait`, split off from
  the declaration so it can rank later. Examples: `anyhow:3.1`
  (`Error`'s "Display representations" rustdoc, lines 299-388, ~700
  tokens — the *only* place that shows what error messages actually look
  like); `anyhow:3.4` (`Context` trait rustdoc with the `ImportantThing`
  worked example).
- **Priority: 3.x.** Comes after the bare declaration (1.x) and the
  signature group (2.x).
- **Predecessor:** the bare declaration.
- **Value signals:** higher when the rustdoc carries information not
  derivable from code (rendered output strings, design rationale,
  surprising invariants like the `Context` downcast rule). When the
  rustdoc is just an example, it competes with the integration test that
  exercises the same path.

#### Module-level synopsis with capture syntax / format table
- The opening paragraphs of a module doc that establish a sub-API's
  vocabulary. Examples: `log:2.7` (`kv/mod.rs:1-60`, the
  `:?`/`:%`/`:err`/`:sval`/`:serde` capturing-modifier table — "gates
  correct understanding of the kv macro syntax"), `log:3.4` (kv consumer
  examples), `mdbook:3.17` (`mdbook-core::config` module doc with TOML
  example).
- **Priority: 2.x–3.x.**
- **Value signals:** very high when the doc establishes a vocabulary
  (capture modifiers, dotted-path API, link-helper grammar) the rest of
  the snapshot will reference.

### 1.12 Function / method bodies

#### Macro expansion / dispatch body
- Full body of a macro whose arms reveal protocol-level information.
  Examples: `log:3.1` (`log!` dispatcher, all four arms), `log:4.7`
  (`__log!` internal macro showing the `STATIC_MAX_LEVEL` short-circuit).
- **Priority: 3.x–4.x.** Arrives after the user-facing signature.
- **Predecessor:** the macro signature group / name list.
- **Value signals:** higher when the body discloses syntax not present in
  the doc (the `logger:`/`target:` argument composition in `log:3.1`).

#### Method body with substantial doc explaining rationale
- The doc + body of one method together when neither makes sense alone.
  Examples: `anyhow:4.1` (`Error::context` body + the rationale doc
  distinguishing it from `Context::context`), `anyhow:4.2`
  (`into_boxed_dyn_error` + `reallocate_..._without_backtrace` paired
  because their docs and bodies make opposite tradeoffs).
- **Priority: 4.x.**
- **Value signals:** higher when the method is the *answer* to a common
  question (round-trip with `Box<dyn StdError>`); higher when pairing two
  related methods saves tokens vs ranking each independently.

#### CLI subcommand `execute` body
- The function that wires a CLI subcommand to library calls. Examples:
  `mdbook:4.1` (`build::execute`), `mdbook:4.2` (`test::execute`),
  `mdbook:4.3` (`init::execute`), `mdbook:4.4` (`serve::execute`).
- **Priority: 4.x.**
- **Predecessor:** the subcommand's `make_subcommand` signature group
  (`mdbook:3.1`/`3.2`).

### 1.13 Internal / unsafe core

#### Vtable / repr struct
- Definition of the type-erased core of an abstraction, often `#[repr(C)]`
  or `#[repr(transparent)]`. Examples: `anyhow:5.1` (`ErrorVTable` +
  `ErrorImpl<E>` + `ContextError` joined with elision marker), `log:4.12`
  (static state: `LOGGER`, `STATE`, `LOG_LEVEL_NAMES`).
- **Priority: 4.x–5.x.** Late, but required for any unsafe-code question.
- **Value signals:** without these, downstream unsafe bodies "read as
  opaque magic" (`anyhow:5.1`).

#### Constructor cluster mounting the vtable
- The hand-written constructor functions that wire up the vtable.
  Example: `anyhow:5.2` (`construct_from_std`/`_adhoc`/`_display`),
  `anyhow:5.3` (the unsafe `construct<E>` core).
- **Priority: 5.x.**
- **Predecessor:** the vtable / repr struct.

#### State-machine / atomics body
- The implementation of the runtime state machine (often a
  `compare_exchange` cascade). Example: `log:4.10` (`set_logger_inner`).
- **Priority: 4.x–5.x.**
- **Predecessor:** the public API that callers see (`log:4.10 → 2.6`).

### 1.14 Test files as executable spec

#### Whole small test file (one invariant per file)
- An entire `tests/test_*.rs` shipped in full because the file's *name*
  states an invariant and the body asserts it concisely. Examples:
  `anyhow:2.8` (`test_repr.rs`, the one-word size invariant — "the single
  most important architectural invariant of the crate, asserted in
  code"), `anyhow:2.9` (`test_autotrait.rs`), `anyhow:2.10`
  (`test_ffi.rs`), `anyhow:3.7` (`test_chain.rs`), `anyhow:7.6`
  (`test_source.rs`), `log:5.3` (`tests/integration.rs`).
- **Priority: 2.x–5.x depending on how load-bearing the invariant is.**
  The "one-word repr" test ranks at `anyhow:2.8` precisely because it
  justifies the entire vtable design.
- **Value signals:** test files where the function names *are* the spec
  (one `#[test] fn assert_one_word_size` etc.) outrank tests that
  exercise edge cases. Tests with shared helpers (`anyhow:3.8`'s
  `tests/common/mod.rs` + `tests/drop/mod.rs`) get ranked once and
  unlock several test files.

#### Test file preamble + per-test fn index
- The file's `use`s and helper definitions, followed by a rendered grep
  of `^(#\[test\]|fn test_)` lines from the same file. Example:
  `anyhow:7.7` (`test_ensure.rs:1-39` + grep index).
- **Priority: 7.x — late.**
- **Value signals:** the one tool the snapshot has to make a giant test
  file navigable without showing its body. Worth doing only when the
  file is large enough to defer (~700+ lines) and the function names are
  self-describing.

#### Cross-file `#[test] fn` index
- Rendered grep of `^(#\[test\]|fn test_)` across several test files
  whose bodies are below-the-fold. Example: `anyhow:7.8`
  (five files indexed at once).
- **Priority: 7.x — late.**
- **Value signals:** lets one batch (~1000 tokens) gesture into ~3000
  tokens of test bodies kept off-snapshot.

#### Expected-output constants block
- The `EXPECTED_*` constants that pin formatter output. Example:
  `anyhow:3.2` (`test_fmt.rs:1-67`, the literal expected strings for
  `{}`/`{:#}`/`{:?}`/`{:#?}`).
- **Priority: 3.x.**
- **Value signals:** "Reading these constants is faster than reading
  `fmt.rs` for 'what string does `{:?}` produce?' queries"
  (`anyhow:3.2`). Ships next to the docs that describe the format and
  the code that emits it.

### 1.15 Examples / nested no-std / smoke crates

#### Reference-implementation example
- A canonical example from `examples/`, partial body with elision
  marker. Example: `mdbook:5.1` (`nop-preprocessor.rs:1-75`, "the most
  concrete answer to 'how do I write a preprocessor?'").
- **Priority: 5.x.**
- **Value signals:** higher when the example is itself the recommended
  template (cited as "Reference template"); duplicates of the same
  pattern (a third example of the same shape in the guide) stay
  below-the-fold.

#### Nested smoke crate
- A tiny `Cargo.toml` + `lib.rs`/`test.rs` that asserts a property the
  main crate can't easily test internally (no-std, MSRV). Examples:
  `anyhow:7.4` (`tests/crate/`, two-file batch), `log`'s
  `test_max_level_features/` (kept below-the-fold but explicitly named).
- **Priority: 7.x.**

#### `tests/ui/` directory listing (trybuild compile-fail cases)
- The folder listing alone, no bodies. Example: `anyhow:7.5`.
- **Priority: 7.x — late.**
- **Value signals:** "Names alone tell the agent which footguns are
  guarded" (`anyhow:7.5`); the `.rs` and `.stderr` bodies stay
  below-the-fold.

### 1.16 Realistic configuration file
- A whole real configuration file the project itself uses. Example:
  `mdbook:5.9` (`guide/book.toml`, mdBook's own book config).
- **Priority: 5.x.**
- **Value signals:** uniquely high for projects that *use themselves* —
  the dogfooded config is a worked example of every config option that
  matters. Synthetic config snippets in docs lose to it.

### 1.17 Internal `mod` / `use` plumbing
- The `extern crate` + `mod`/`pub mod`/`pub use` block at the top of the
  crate root. Examples: `log:2.1` (`src/lib.rs:396-410`, including the
  `extern crate core as std` no-std alias), `anyhow:2.1` (`:246-263`,
  the 11 module declarations matching `src/` listing), `mdbook:2.1`
  (`mdbook-driver` re-exports).
- **Priority: 2.x.**
- **Predecessor:** the source-tree listing.
- **Value signals:** higher when feature gating shows up (`#[cfg(feature
  = "kv")] pub mod kv` in `log:2.1`); reveals which modules are private.

## 2. Cross-language vs language-specific

### 2.1 Cross-language categories
These categories' value heuristics depend only on the filesystem, the
manifest format, or the shape of plain prose. They should not be
specialized to Rust:

| Category | Cited at |
|---|---|
| Repo-root listing (1.1) | log/anyhow/mdbook all open with this |
| Source-tree subdirectory listings (1.1) | all three fixtures |
| Multi-listing rolls (1.1) | mdbook:2.21, 5.11, 5.12 |
| Manifest identity (1.2) | log:1.3, anyhow:1.5, mdbook:1.7 |
| Dependency lists (1.2) | log:1.13, anyhow:1.11, mdbook:5.8 |
| Feature flags (1.2) | log:1.11, anyhow:1.11, mdbook:3.12 |
| README headline / install (1.3) | log:2.4, anyhow:1.4, mdbook:1.2 |
| README tutorial slabs (1.3) | anyhow:2.2/2.5/2.7 |
| CHANGELOG head (1.4) | mdbook:6.2; below-fold note in log |
| CI matrix slices (1.5) | anyhow:7.3 |
| Contributor commands (1.5) | mdbook:5.3 |
| Build script `fn main` (1.6) | anyhow:6.1 |
| Test files as exec spec (1.14) | anyhow & log both rely on this |
| Realistic config file (1.16) | mdbook:5.9 |

**Cross-language value heuristics that must NOT be specialized:**
- *Listings rank by entry-name information density*, not by file count.
  An unusual top-level entry (`build.rs`, `crates/`, `rfcs/`) raises
  priority. Routine entries (`LICENSE-*`, `.gitignore`) do not.
- *README slices outrank rustdoc duplicates* when the agent only needs
  the surface concept; if a rustdoc passage adds non-trivial details
  beyond the README, both can ship (`log:2.4` + `log:3.3`).
- *Test files where the filename is the spec* are extremely high-value
  and apply to any test framework whose convention is descriptive
  filenames (`test_repr.rs`, `test_autotrait.rs`).
- *Per-file body indexing via grep* is a generic strategy: a rendered
  list of `#[test] fn …` lines belongs to a category that any language
  with grep-able test markers can reuse.
- *Manifests are treated as line-range slices, not parsed structures.*
  `log:1.3` cites `Cargo.toml:4` plus `:10-12` — the snapshot quotes
  exact lines rather than rendering a parsed view. Stage 7 should not
  introduce a manifest-parser pass that would invalidate that.
- *Multi-listing rolls* (combining adjacent folder listings) are a
  cross-language tactic, not Rust-specific.

### 2.2 Language-specific categories
These depend on a parser:

| Category | Language | Construct |
|---|---|---|
| Public type declaration (1.7) | Rust | `pub struct` / `pub enum` header + fields/variants |
| Doc-elided declaration (1.7) | Rust | the same, with the doc comment immediately above split into a different batch |
| Public trait declaration (1.8) | Rust | `pub trait` with method signatures |
| Macro-name list (1.9) | Rust | `#[macro_export] macro_rules!` names |
| `macro_rules!` arms (1.9) | Rust | a single macro definition shown intact |
| `#[cfg(doc)]` macro skeleton (1.9) | Rust | the doc-only arm of a dual-form macro |
| Constructor signature group (1.10) | Rust | several `impl T { fn new(...) }` signatures with where-clauses, joined by elision markers |
| Consumer-method signature group (1.10) | Rust | several `&self`/`self` method signatures, joined |
| Setter / accessor cluster (1.10) | Rust | tightly-grouped builder methods shown intact |
| Crate-doc lede (1.11) | Rust | the opening paragraphs of `//!` rustdoc |
| Module-level synopsis (1.11) | Rust | the opening rustdoc of a module that establishes a vocabulary |
| Macro expansion body (1.12) | Rust | full `macro_rules!` arms |
| Method body + rationale doc (1.12) | Rust | one method with both its rustdoc and body |
| CLI subcommand `execute` (1.12) | Rust | the function that wires CLI parsing to library calls |
| Vtable / repr struct (1.13) | Rust | `#[repr(C)]` / `#[repr(transparent)]` declarations |
| Constructor cluster (vtable mount) (1.13) | Rust | unsafe constructors that build the vtable |
| State machine / atomics body (1.13) | Rust | `compare_exchange` cascade etc. |
| Internal `mod` / `use` plumbing (1.17) | Rust | top-of-`lib.rs` module map |

For markdown, the only fixture that ships markdown-as-content is
`mdbook` (which authors a book in `guide/`), and the load-bearing
markdown-specific batches are:

| Category | Language | Construct |
|---|---|---|
| `SUMMARY.md` whole file | Markdown | `mdbook:1.14` — the user-facing ToC of a book |
| Heading-anchored prose slab | Markdown | `mdbook:5.10` (`guide/src/README.md:1-32`, the feature-bullets section); below-fold notes throughout cite `guide/src/**/*.md` files by name |

Markdown is otherwise served via filesystem listings (e.g. `guide/`
contents) and below-fold name pointers — the actual prose ranks
relatively low because the SUMMARY + folder listings already let the
agent jump precisely.

## 3. Deviating predecessor edges

These are non-obvious predecessor edges — cases where filesystem or AST
order does *not* give the right ranking, and the north stars explicitly
record a logical dependency that crosses files or constructs.

- **`log:1.7 → 1.2`** — the `kv` module re-exports list (in `src/kv/mod.rs`)
  is a predecessor of nothing earlier than the source-tree listing, but
  the snapshot promotes it to 1.x because the catastrophic-omission risk
  is that without it "an agent never realises the crate has structured
  logging." The snapshot uses re-export lists as an early API map even
  when the file containing them sits one level deep.
- **`log:2.7 → 1.7`** and **`log:3.2 → 2.7`** — the kv "capturing
  modifiers" table (`:?`/`:%`/`:err`/`:sval`/`:serde`) gates *macro
  syntax* (`info!(yak:serde; …)`) shown elsewhere. The kv mod doc is a
  predecessor of the doctest-style macro example even though the macro
  example lives in `lib.rs` and the kv synopsis lives in `kv/mod.rs`.
- **`log:4.7 → 3.1` and `log:4.8 → 4.7`** — the `__log!` internal macro
  is a predecessor of the `__private_api` body, even though they live in
  different files; the chain forms because the public macro expands into
  `__log!` which calls `__private_api::log_impl`.
- **`anyhow:3.3 → 2.6`** — `fmt.rs`'s `Display`/`Debug` impl is a
  predecessor of the `Error` consumer-method signatures rather than of
  the `Error` struct declaration. The reason is that the formatting
  contract is the *answer* to questions raised by `Display`/`Debug`
  appearing in the consumer-method group.
- **`anyhow:4.1 → 2.6`** and **`anyhow:4.2 → 2.6`** — `Error::context`
  body and `Error::into_boxed_dyn_error` body both list the consumer
  signature group as predecessor, not the struct declaration. This
  explicitly groups "long body explaining a method" with its signature
  rather than with its containing type.
- **`anyhow:4.3 → 5.1`** (forward predecessor!) — the `From`/`Deref`/
  `Display`/`Debug`/`Drop` impls for `Error` (4.3) declare the `5.1`
  vtable structs as a predecessor. This is unusual: the predecessor is
  *later in the ranking*. It's still satisfied at any cut that includes
  4.3 — but it's a flag that the ranking decided to ship the impls
  before the vtable, accepting that a small budget might show 4.3
  without the rationale-providing 5.1. Stage 7 should validate that
  predecessors with priority > the dependent are intentional.
- **`anyhow:5.5 → 5.1`** — `is`/`downcast`/`downcast_ref`/`downcast_mut`
  bodies cite the vtable struct as predecessor *and* the consumer
  signatures (via 2.6); the snapshot picks the vtable as the load-bearing
  one, since the body is opaque without it.
- **`anyhow:6.4 → 1.14`** — `ensure.rs` runtime helpers' predecessor is
  the `#[cfg(doc)]` macro skeleton (1.14), not the `__ensure!` doc
  wrapper (7.2). The runtime helpers are useful to a reader who has only
  seen the user-facing arms.
- **`anyhow:7.2 → 1.14`** — the `__ensure!` doc wrapper + dispatch arm
  cites the `#[cfg(doc)]` skeleton as predecessor, even though both
  live in the same file. They are intentionally split so the readable
  surface ranks early.
- **`mdbook:3.6 → 1.12`** — `LinkPreprocessor`'s declaration + run head
  cites the `links.rs` doc-comment listing the supported helpers as
  predecessor. Order is "doc lists what's available; then the code that
  implements it."
- **`mdbook:5.1 → 3.10`** — the `nop-preprocessor` example cites
  `CmdPreprocessor`'s declaration as predecessor; the example is only
  meaningful after the agent knows the spawn-and-pipe-stdin protocol.
- **`mdbook:5.7 → 5.5`** and **`mdbook:5.6 → 5.5`** — the test-suite
  `main.rs` and `README.md` both depend on the `tests/testsuite/`
  listing, even though folder listings rarely act as predecessors of
  whole files — here the listing is what reveals the harness exists.

The pattern: **predecessor edges crossing file boundaries are normal
when the snapshot has factored a concept across files** (kv re-exports
in one file, kv producer syntax in another; macro definitions in one
file, macro expansion path through `__private_api` in another). Stage 7
must allow predecessors to point to batches in *other files* and *other
crates*, not just the file the dependent batch lives in.

## 4. Implementation priority

Categories ranked by recurrence and load-bearingness for Stage 7's first
walker iteration. The "tier 1" categories are present and load-bearing
in all three fixtures; "tier 2" appear in two; "tier 3" appear in one
fixture and need human scrutiny before generalizing.

### Tier 1 — must work for the first iteration to be useful
1. **Filesystem listings** (1.1) — root + immediate sub-listings of
   `src/`, `tests/`, and any other top-level source-bearing directory.
   Cross-language; the simplest walker. Three fixtures, every one opens
   with these.
2. **Manifest identity + features + dependencies** (1.2) — three thin
   line-range slices of `Cargo.toml` (or analogue). Cross-language;
   needs a manifest-aware "find these sections" routine, not a parser.
3. **README headline + tutorial slabs** (1.3) — split the `README.md`
   into a small "what is this" head and one or two tutorial bodies.
   Cross-language.
4. **Public type declarations** (1.7) and **public trait declarations**
   (1.8) with their docs split into separate batches — Rust-specific.
   The doc-elided declaration is the load-bearing pattern.
5. **Internal `mod` / `use` plumbing** (1.17) — Rust-specific; cheap and
   high-leverage for orientation.
6. **Macro-name list + macro signature group** (1.9) — Rust-specific;
   `log` and `anyhow` both depend on this heavily.
7. **Constructor + consumer-method signature groups** (1.10) — Rust-
   specific; the dominant 2.x pattern in `anyhow`.
8. **Crate-doc lede + module-level synopsis** (1.11) — Rust-specific
   rustdoc handling.

### Tier 2 — important; defer if budget tight, do not skip after the first iteration
1. **Whole small test file as exec spec** (1.14) — Rust-specific; central
   to `anyhow`'s ranking and present in `log`.
2. **Method body with rationale doc** (1.12) — Rust-specific.
3. **CLI subcommand surface** — both `make_subcommand` signature group
   (1.10) and `execute` body (1.12). Present in `mdbook` only at scale,
   but the pattern (one file per subcommand → one batch per surface +
   one per body) generalizes.
4. **Multi-listing rolls** (1.1) — needed for projects with deep
   subdirectories (`mdbook-html/src/`).
5. **Vtable / repr struct + constructor cluster + state machine body**
   (1.13) — Rust-specific; only `anyhow` and `log` need it, but when
   it's needed it gates everything below.
6. **Realistic config file** (1.16) — easy to detect (the project
   *uses* a config file format it itself defines); high value-per-token
   in `mdbook`.

### Tier 3 — single-fixture or single-language; flag for human scrutiny
1. **`#[cfg(doc)]` macro skeleton + doc-wrapper split** (1.9) — appears
   only in `anyhow` (`ensure!` is the only macro in the corpus with a
   dual `#[cfg(doc)]`/`#[cfg(not(doc))]` form). Stage 7 should detect
   this pattern but not over-fit a category around it.
2. **Build-script `fn main`** (1.6) — `anyhow` only. Worth a heuristic
   ("if `build.rs` exists *and* its emitted cfgs are referenced in
   `src/`, promote") — but the second condition matters; many crates
   have trivial build scripts that should stay below-the-fold.
3. **Test file preamble + per-test fn index** (1.14) and **cross-file
   `#[test] fn` index** (1.14) — `anyhow` only. The cross-file index
   in particular is a unique format; Stage 7 should land it deliberately
   if at all.
4. **Expected-output constants block** (1.14) — `anyhow:3.2` only.
   General "constants pinning observable output" might apply elsewhere
   but the fixture sample is too thin to commit.
5. **Nested smoke crate** (1.15) — `anyhow:7.4` (`tests/crate/`) and
   `log`'s `test_max_level_features/` (below-fold, but cited). Worth a
   note in the walker; not a top-tier category.
6. **`tests/ui/` directory listing** (1.15) — `anyhow:7.5`; trybuild is
   a Rust-ecosystem convention but appears in only one fixture here.
7. **Workspace-aware ranking** (`mdbook:1.3` + `1.4` mapping crate name
   → description) — only `mdbook` is a workspace. Stage 7 should detect
   `[workspace] members` and emit the per-crate description map; the
   pattern is valuable but it's single-fixture and single-language
   (Rust workspace).
8. **CHANGELOG-head** (1.4), **CI matrix slices** (1.5), **contributor
   commands** (1.5) — each appears in one fixture as a ranked batch
   (the others defer them). Treat as opt-in promotions rather than
   default categories.

### Markdown walker (Stage 7 second deliverable)
For markdown specifically, the only first-iteration categories needed
are:
- **`SUMMARY.md`-whole file** when the markdown is part of a book
  (mdBook, GitBook). Detected by the file's existence and the literal
  format.
- **README-style heading-anchored prose slab** — first heading + body,
  for any markdown file at the conventional location.

Everything deeper (per-page heading walking, link extraction, etc.) is
out of scope for the first markdown iteration; `mdbook` defers all
~80 guide pages to below-the-fold and the human reviewer accepted that.

## 5. Open questions and inconsistencies

1. **Predecessor pointing forward in priority.** `anyhow:4.3` declares
   `5.1` as predecessor — the predecessor is ranked *after* the
   dependent. Frozen and explicit, but Stage 7 needs to decide whether
   the data model permits this (the design notes describe predecessor
   as a "logical" constraint distinct from priority, which would allow
   it). Worth surfacing in the alignment reviewer's category schema.

2. **`anyhow:7.5` cost helper uses `ls`, not the count-tokens script
   directly.** `Cost: 70 tokens (helper: ls tests/fixtures/anyhow/tests/ui/
   | scripts/count-tokens.py --stdin)`. All other listings use `printf`
   for reproducibility (the file order is deterministic). This is a
   minor evidence-quality defect — the helper isn't exactly reproducible
   without specifying the locale-dependent `ls` ordering. Not a
   correctness issue with the *content* of the batch.

3. **`anyhow` uses `mod ext` private trait in `context.rs:1-68`** — the
   `Context` impl story is split across two files in a way the snapshot
   handles (`anyhow:3.5` ranges `:1-68` and `:70-113` in one batch),
   but the rationale comment "Not using map_err to save 2 useless
   frames" is cited as load-bearing without the actual `mod ext`
   structure being explained anywhere in the snapshot. Stage 7's
   walker may need to detect "private shim trait that exists only to
   make a public impl work" as a sub-pattern of the trait-impl
   category.

4. **`mdbook` numbers a section header (`## 2. Core data model and
   config shape`) inside the Batches list.** The other two fixtures use
   only `### N.M` headings under `## Batches`. Authoring style
   divergence; doesn't affect content but Stage 7's parser should
   tolerate either form.

5. **Whole-file vs split-file thresholds disagree.** `log:5.3` ships
   `tests/integration.rs` whole at 785 tokens; `anyhow` defers
   `test_context.rs` (1007 tokens) entirely to below-the-fold and
   indexes it via 7.8. The break-even point between "ship whole" and
   "name + grep-index" is implicit in author judgment. Stage 7 will
   need a heuristic; the corpus suggests something like ~800–1000
   tokens, but the *content type* matters more than the count (a
   one-purpose test file ships whole at higher cost than a multi-purpose
   one).

6. **`log` and `anyhow` both pre-promote the front of the CHANGELOG into
   below-the-fold prose** ("the most recent ~440 tokens of `:1-30`")
   without making it a ranked batch. `mdbook` makes it a ranked batch
   (`6.2`). Stage 7 should pick one convention — the `mdbook` choice
   (a small low-priority batch) is more honest than a below-the-fold
   note.

7. **`mdbook`'s ranked total exceeds the 20k cap** (~24k) and
   self-justifies in below-the-fold ("modestly over the soft cap. The
   fixture is information-dense, and the threshold test … is what set
   the size"). `anyhow` is ~22.8k; `log` lands at ~20.9k. The cap is
   nominal, not strict — Stage 7's evaluator should treat the cap as a
   soft target whose violation needs an in-snapshot justification.

8. **Doc-elided declaration as a *separate* batch from its rustdoc** is
   the most powerful pattern across the three fixtures (`anyhow:1.6` +
   `2.3` + `3.1` is the canonical example). Stage 7 must support
   splitting a single contiguous source range into N batches with
   priority skips, which the design notes already flag as the bare-
   ellipsis renderer feature being deferred. **This is a hard blocker:
   the snapshot honesty guarantee plus the split-by-priority pattern
   together require interleaved bare-ellipsis support before the first
   Rust walker iteration is meaningful.**

9. **Re-exports as orientation maps** (`log:1.7`, `anyhow:1.2` via
   filename, `mdbook:1.4` derived from crate descriptions). Three
   different mechanisms producing the same value. Stage 7 should
   pick the most general and apply it uniformly; otherwise different
   walkers will solve the same problem differently.
