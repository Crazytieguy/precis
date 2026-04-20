# Batch ontology (Stage 4)

Forward-looking design reference for Stage 7 walkers. Defines the recurring
shapes a batch can take, the abstract value heuristics that rank them, and
the predecessor patterns that cross filesystem order. Read at the start of
every implementation session.

Conventions:
- *Priority* is the coarse band a batch typically occupies (1.x = earliest
  orientation; 7.x+ = late or on-demand). Bands are advisory; an unusually
  load-bearing instance moves earlier.
- *Predecessor* names the logical dependency — the batch a reader needs
  to have seen for the current one to make sense. Distinct from filesystem
  or AST order.
- *Value signals* rank instances *within* a category — levers that move one
  up or down relative to its siblings.
- *Cross-language* = value heuristic depends only on filesystem shape,
  manifest format, or plain prose. *Language-specific* = depends on a
  parser for a particular grammar.

## 1. Recurring batch categories

### 1.1 Filesystem listings

**Repo-root entry listing.** One-line listing of the immediate children of
the seed root, folders suffixed `/`. Priority earliest; cost is tens of
tokens, value is large (without it the agent doesn't know what's there).
No predecessors. Value scales with entry-name *information density*:
unusual top-level entries (`build.rs` next to `src/`, `crates/`, `rfcs/`)
raise priority; routine names (`LICENSE-*`, `.gitignore`) carry less
weight but stay cheap to include.

**Source-tree subdirectory listings.** Listings of files (and possibly
subfolders) inside a known source directory. Priority early, immediately
after the root listing; predecessor is the parent listing. Value higher
when the file names are themselves an API map (one file per CLI
subcommand, test files named after the invariants they assert).

**Multi-listing rolls.** Several adjacent listings rendered as one batch
when each is small. Permitted only when the listings are conceptually one
slice — sibling or descendant directories the agent will want together.

### 1.2 Manifest / package identity

**Crate identity.** Tight slice of the package manifest holding name,
version, edition, MSRV, one-sentence description. Priority very early,
before bodies; no predecessor. Always include the `description` line —
often the cheapest single semantic anchor in the snapshot.

**Dependency lists.** Runtime + dev deps. Priority mid (late 1.x or 2.x);
predecessor is identity / features. Higher when external crates leak into
the public API (re-exported types, trait bounds that mention them); lower
when deps are internal-only.

**Feature flags.** The `[features]` table. Priority early-mid (1.x);
predecessor is identity. Catastrophic-omission risk: without them the
agent is lost on `#[cfg(feature = …)]` gates. Higher when features gate
substantial behaviour (cascading toggles, whole optional subsystems);
lower when a feature only flips a single optional dep.

### 1.3 Top-level prose / README

**README headline / install snippet.** Tightest "what is this project" +
how-to-add-it slice. Priority very early; no predecessor. When tight
(≤20 tokens), eliminates the catastrophic "what even is this repo?"
failure mode at almost zero cost. When richer, doubles as the canonical
use example. If the same prose appears in crate-root rustdoc, prefer one
source and demote the other.

**README tutorial slabs.** The walkthrough body of the README split into
a few coherent ranges. Priority 2.x — second tier, after declarations
are anchored. Predecessor is the headline; each subsequent slab takes
its preceding slab as predecessor. Ranks against any rustdoc analogue;
both can ship if each adds a detail the other lacks.

### 1.4 CHANGELOG / release-notes head
First few entries of `CHANGELOG.md` (current version + 1–2 prior).
Priority late. Useful for "what shipped recently?" but not for
steady-state reading. Older entries decay sharply; the head pays for
itself, the tail rarely does. Prefer a small ranked batch over a
below-fold note.

### 1.5 CI / contributor docs

**CI workflow slices.** Selected slices of `.github/workflows/*.yml`,
focused on the test matrix and unusual cfg flags. Often several
non-contiguous ranges with elision markers. Priority late (7.x or
below-the-fold); promote when nightly/MSRV matters or CI invokes unusual
flags. Routine `actions/checkout@vN` rows are pure boilerplate.

**Contributor / xtask command index.** Slice of `CONTRIBUTING.md`
enumerating canonical commands. Priority mid-to-late. Higher when the
project has a non-obvious umbrella command (`cargo xtask test-all`);
lower when the commands are defaults.

### 1.6 Build script (`build.rs`)
The `fn main` decision tree of `build.rs`; helper functions deferred
below-the-fold. Priority 6.x — gated on whether build-script cfgs leak
into `src/`. Acts as predecessor of any file whose `#[cfg(...)]`
predicates it emits. Trivial build scripts stay below-the-fold.

### 1.7 Public type declarations

**Bare struct / enum declaration (doc elided).** Single-paragraph slice
containing only the struct/enum keyword line plus fields and (for enums)
variants. The rustdoc immediately above is **split into a different
batch** so it can rank independently. Priority 1.x — declarations are the
cheapest semantic anchor for a type, and splitting the declaration off
from its rustdoc is the load-bearing move. Predecessor is the file-tree
listing that names the file. Smaller declarations rank highest;
informative field names raise priority.

**Enum with variants + brief doc.** Enum declaration shown intact when
the variant list is itself the vocabulary the rest of the API uses.
Priority 1.x. Higher when variant ordering or discriminant comments
carry semantics the rest of the crate depends on.

### 1.8 Public trait declarations
Trait header + method signatures, no bodies, with the per-method "for
implementors" notes that aren't derivable from the signature. Priority
1.x — these are the abstractions every backend or extension implements;
the snapshot must surface them. Predecessor is the file/module listing.
Higher when the trait has only 1–3 methods (cheap to include in full)
or is the *implementation contract* for third-party code. The
un-derivable doc notes are why trait declarations outrank trait-impl
bodies.

### 1.9 Macro declarations

**Macro-name list.** Rendered list of `#[macro_export] macro_rules!`
names — no bodies, no signatures. Priority 1.x. Ranks above macro
signatures when the names alone let the agent guess the call shape.

**Macro signature + arms.** Single full `macro_rules!` block with all
arms and doc. Priority split-dependent — tiny macros ship in 1.x; longer
representative ones in 2.x or 3.x. Predecessor is the macro-name list.
Higher when one macro's arms imply a family — a representative with an
elision marker for siblings can cover a whole cluster at one-macro cost.

**`#[cfg(doc)]` user-facing macro skeleton.** Doc-only arm of a macro
whose real implementation is a giant token-tree parser. Priority 1.x —
the only readable view of the macro. The full parser body stays
below-the-fold (it can single-handedly exceed the token cap). An honest
snapshot demands surfacing this view.

### 1.10 Function / method signatures (no body)

**Constructor signature group.** Several constructor signatures from the
same `impl` block, joined with `…` elision markers, no rustdoc, full
where-clauses intact. Priority 2.x; predecessor is the type's
declaration. Showing 3–5 constructors at once beats showing each
separately for the same total cost.

**Consumer-method signature group.** Same shape, for `&self`/`self`
methods. Priority 2.x — early in the second tier. Individual-method
rustdoc can stay on disk; the signature group itself answers "does X
have method Y?" without a follow-up.

**Setter / accessor blocks (intact).** Builder or accessor cluster shown
intact because each setter is so tiny that splitting wastes more context
than it saves. Priority 3.x–4.x.

### 1.11 Doc-comment slabs (rustdoc / module doc)

**Crate-doc lede.** Opening rustdoc paragraph from the crate root —
"what is this crate". Priority 1.x. Higher when *not* a near-duplicate
of the README; if it duplicates, prefer one source.

**Doc rustdoc on a single public type.** The long rustdoc above one
`pub struct` / `pub trait`, split off from the declaration so it can
rank later. Priority 3.x; predecessor is the bare declaration. Higher
when the rustdoc carries information not derivable from code (rendered
output strings, design rationale, surprising invariants, downcast or
coercion rules). When it's just an example, it competes with any
integration test that exercises the same path.

**Module-level synopsis.** Opening paragraphs of a module doc that
establish a sub-API's vocabulary (capture modifiers, dotted-path grammar,
link-helper syntax). Priority 2.x–3.x. Very high when the doc establishes
a vocabulary the rest of the snapshot will reference.

### 1.12 Function / method bodies

**Macro expansion / dispatch body.** Full body of a macro whose arms
reveal protocol-level information (argument composition, target/logger
routing, short-circuit cfgs). Priority 3.x–4.x; predecessor is the macro
signature group / name list. Higher when the body discloses syntax not
present in the doc.

**Method body with rationale doc.** Doc + body of one method together
when neither makes sense alone. Also used for pairing two related
methods whose docs and bodies make opposite tradeoffs. Priority 4.x.
Higher when the method is the *answer* to a common question (round-trip
with some well-known trait object) or pairing saves tokens vs. ranking
each independently.

**CLI subcommand `execute` body.** The function that wires a CLI
subcommand to library calls. Priority 4.x; predecessor is the
subcommand's `make_subcommand` signature group (so the surface is
anchored before the wiring).

### 1.13 Internal / unsafe core

**Vtable / repr struct.** Definition of the type-erased core of an
abstraction, often `#[repr(C)]` or `#[repr(transparent)]`. Multiple
related structs may be joined with elision markers. Priority 4.x–5.x;
late, but required for any unsafe-code question. Without these,
downstream unsafe bodies read as opaque magic.

**Constructor cluster mounting the vtable.** Hand-written constructors
that wire up the vtable (often a family of `construct_from_*` plus an
unsafe core). Priority 5.x; predecessor is the vtable struct.

**State-machine / atomics body.** Implementation of a runtime state
machine (often a `compare_exchange` cascade). Priority 4.x–5.x;
predecessor is the public API that callers see.

### 1.14 Test files as executable spec

**Whole small test file (one invariant per file).** An entire
`tests/test_*.rs` shipped in full because the file's *name* states an
invariant and the body asserts it concisely. Priority 2.x–5.x depending
on how load-bearing the invariant is — a test that asserts a core
architectural invariant ranks very early because it justifies the
design. Files where function names *are* the spec outrank edge-case
exercises. Tests with shared helpers get ranked once and unlock several
test files.

**Test file preamble + per-test fn index.** The file's `use`s and helper
definitions, followed by a rendered grep of `^(#\[test\]|fn test_)`
lines from the same file. Priority 7.x. Worth doing only when the file
is large enough to defer (~700+ tokens) and the function names are
self-describing.

**Cross-file `#[test] fn` index.** Rendered grep across several test
files whose bodies are below-the-fold. Priority 7.x. Lets one compact
batch gesture into several kilotokens of off-snapshot test bodies.

**Expected-output constants block.** The `EXPECTED_*` constants that pin
formatter / rendering output. Priority 3.x. Reading these is faster than
reading the formatter source for "what string does `{:?}` produce?"
queries. Ships next to the docs that describe the format and the code
that emits it.

### 1.15 Examples / nested smoke crates

**Reference-implementation example.** Canonical example from `examples/`,
partial body with elision marker. Priority 5.x. Higher when the example
is itself the recommended template; duplicates of the same pattern stay
below-the-fold.

**Nested smoke crate.** Tiny `Cargo.toml` + `lib.rs`/`test.rs` that
asserts a property the main crate can't easily test internally (no-std,
MSRV). Priority 7.x.

**`tests/ui/` directory listing.** Folder listing alone, no bodies.
Priority 7.x. Names alone tell the agent which footguns are guarded;
the `.rs` and `.stderr` bodies stay below-the-fold.

### 1.16 Realistic configuration file
A whole real configuration file the project itself uses. Priority 5.x.
Uniquely high for projects that *use themselves* — a dogfooded config
is a worked example of every option that matters. Synthetic config
snippets in docs lose to it.

### 1.17 Internal `mod` / `use` plumbing
The `extern crate` + `mod`/`pub mod`/`pub use` block at the top of the
crate root. Priority 2.x; predecessor is the source-tree listing. Higher
when feature gating shows up (`#[cfg(feature = "…")] pub mod …`);
reveals which modules are private.

## 2. Cross-language vs language-specific

### 2.1 Cross-language categories

Value heuristic depends only on filesystem shape, manifest format, or
plain prose. Must not be specialized to any one language:

- Filesystem listings (1.1) — root, source-tree, multi-listing rolls
- Manifest identity / dependencies / feature flags (1.2)
- README headline + tutorial slabs (1.3)
- CHANGELOG head (1.4)
- CI matrix slices and contributor command index (1.5)
- Build script `fn main` (1.6) — the *decision* to promote depends on
  whether emitted cfgs are referenced downstream, a cross-language check
- Test files as executable spec (1.14) — the "filename is the spec"
  pattern generalizes to any test framework with descriptive filenames
- Realistic configuration file (1.16)

Cross-language heuristics that must stay general:

- Listings rank by entry-name information density, not by file count.
- README slices outrank rustdoc duplicates when the agent only needs the
  surface concept; if the rustdoc adds non-trivial detail, both can ship.
- Test files where the filename is the spec are extremely high-value and
  apply to any descriptive-filename convention.
- Per-file body indexing via grep is generic — a rendered list of test
  markers belongs to any language with grep-able conventions.
- Manifests are line-range slices, not parsed structures. Stage 7 should
  not introduce a manifest-parser pass that would invalidate this.
- Multi-listing rolls (combining adjacent folder listings) are a
  cross-language tactic.

### 2.2 Language-specific categories

These depend on a parser for a particular grammar.

**Rust.** All categories in 1.7–1.13 and 1.17:

| Category | Construct |
|---|---|
| Public type declaration (1.7) | `pub struct` / `pub enum` header + fields/variants |
| Doc-elided declaration (1.7) | same, with rustdoc above split into a different batch |
| Public trait declaration (1.8) | `pub trait` with method signatures |
| Macro-name list (1.9) | `#[macro_export] macro_rules!` names |
| `macro_rules!` arms (1.9) | a single macro shown intact |
| `#[cfg(doc)]` macro skeleton (1.9) | the doc-only arm of a dual-form macro |
| Constructor signature group (1.10) | several `impl T { fn new(...) }` signatures joined with elision markers |
| Consumer-method signature group (1.10) | several `&self`/`self` method signatures joined |
| Setter / accessor cluster (1.10) | tightly-grouped builder methods shown intact |
| Crate-doc lede (1.11) | opening paragraphs of `//!` rustdoc |
| Module-level synopsis (1.11) | opening rustdoc of a module that establishes a vocabulary |
| Macro expansion body (1.12) | full `macro_rules!` arms |
| Method body + rationale doc (1.12) | one method with both rustdoc and body |
| CLI subcommand `execute` (1.12) | the function that wires CLI parsing to library calls |
| Vtable / repr struct (1.13) | `#[repr(C)]` / `#[repr(transparent)]` declarations |
| Constructor cluster (vtable mount) (1.13) | unsafe constructors that build the vtable |
| State machine / atomics body (1.13) | `compare_exchange` cascade etc. |
| Internal `mod` / `use` plumbing (1.17) | top-of-`lib.rs` module map |

**Markdown.** Markdown-as-content shows up in book-style projects (mdBook,
GitBook) and in standalone README/guide files. Two batch shapes:

- **`SUMMARY.md` whole file.** The user-facing ToC of a book. Detected
  by the file's existence at a conventional location plus the literal
  outline format. Priority 1.x — it's a navigation map. Higher when the
  ToC's section names are themselves the taxonomy the rest of the guide
  uses.
- **Heading-anchored prose slab.** First heading + body of a markdown
  file at a conventional location (a guide's landing page, a per-chapter
  overview). Priority 2.x–5.x depending on how navigational it is.
  Higher when the section opens with a feature-bullet list or
  concept-defining paragraph that the rest of the guide refers back to.

Deeper markdown handling (per-page heading walking, link extraction,
cross-page anchor resolution) is out of scope for the first iteration.
Below-the-fold name pointers suffice when the SUMMARY plus folder
listings already let the agent jump precisely.

## 3. Deviating predecessor edges

Predecessor edges that do *not* follow filesystem or AST order. Stage 7's
walker must support these — a batch's logical prerequisite can live in a
different file or a different crate.

- **Re-export list as early API map.** A re-export cluster in a module
  `mod.rs` is predecessor to any batch that uses the re-exported
  vocabulary, even when the re-exports sit one directory deep. Without
  them, the agent may never realize a subsystem exists.
- **Concept-establishing doc crosses into macro syntax.** A module doc
  that defines a sub-API's vocabulary (capture modifiers, field-syntax
  grammar) is predecessor to any macro example that uses that
  vocabulary, even when the two live in different files.
- **Public macro → internal dispatcher → private API.** When a public
  macro expands into an internal `__macro!` which calls a `__private_api`
  function, the chain forms a predecessor path across multiple files.
- **Formatting contract as predecessor of consumer methods.** A
  `Display`/`Debug` impl can be predecessor of the consumer-method
  signature group (not the struct declaration), because the formatting
  contract is the *answer* to questions raised by `Display`/`Debug`
  appearing in the signature group.
- **Body paired with signature, not type.** A method's body + rationale
  doc often lists the signature group as predecessor, not the containing
  struct.
- **Vtable as predecessor of its downcast/deref impls.** `is`/`downcast`
  bodies treat the vtable as predecessor; the body is opaque without the
  layout it indexes.
- **`#[cfg(doc)]` skeleton as predecessor of runtime helpers.** The
  readable user-facing macro arm is predecessor to any runtime helper
  functions the non-doc arms expand into.
- **Doc enumerates helpers, then code implements them.** When a source
  file opens with a doc block listing supported directives, that doc is
  predecessor of the implementing code — even when both live in the same
  file, they're split so the doc ranks first.
- **Folder listing as predecessor of a whole file.** When a listing is
  what reveals a harness exists (test suite, examples directory), batches
  inside that folder cite the listing as predecessor.

The pattern: **predecessor edges crossing file boundaries are normal
when a concept is factored across files.** Stage 7 must allow
predecessors to point to batches in other files (and other crates in a
workspace), not just the file the dependent batch lives in.

Forward-priority predecessors (a predecessor ranked *after* its
dependent) are not allowed. If the logical dependency is real, either
the ranking is wrong (predecessor should move earlier) or the edge is
spurious. Stage 7's validator should flag forward predecessor edges as
errors.

## 4. Implementation priority

### Tier 1 — must work for the first iteration to be useful

1. **Filesystem listings** (1.1) — root + immediate sub-listings of
   `src/`, `tests/`, and any other top-level source-bearing directory.
   Cross-language; the simplest walker.
2. **Manifest identity + features + dependencies** (1.2) — three thin
   line-range slices of `Cargo.toml` (or analogue). Cross-language;
   needs a manifest-aware "find these sections" routine, not a parser.
3. **README headline + tutorial slabs** (1.3) — split `README.md` into
   a small "what is this" head and one or two tutorial bodies.
4. **Public type declarations** (1.7) and **public trait declarations**
   (1.8) with rustdoc split into separate batches. The doc-elided
   declaration is the load-bearing pattern.
5. **Internal `mod` / `use` plumbing** (1.17) — cheap and high-leverage
   for orientation.
6. **Macro-name list + macro signature group** (1.9) — any macro-heavy
   crate needs this to anchor its surface.
7. **Constructor + consumer-method signature groups** (1.10) — the
   dominant 2.x pattern for any declaration-rich API.
8. **Crate-doc lede + module-level synopsis** (1.11).
9. **`SUMMARY.md` + heading-anchored prose slab** — minimum viable
   markdown walker for book-style projects and standalone guide files.

### Tier 2 — important; do not skip after the first iteration

1. **Whole small test file as executable spec** (1.14) — central to any
   crate whose tests assert architectural invariants.
2. **Method body with rationale doc** (1.12).
3. **CLI subcommand surface** — both `make_subcommand` signature group
   (1.10) and `execute` body (1.12). Any project structured as one file
   per subcommand benefits.
4. **Multi-listing rolls** (1.1) — needed for projects with deep nested
   subdirectories.
5. **Vtable / repr struct + constructor cluster + state machine body**
   (1.13) — only relevant to crates with a type-erased core or global
   mutable state, but when needed it gates everything below.
6. **Realistic configuration file** (1.16) — easy to detect (the project
   *uses* a config file format it itself defines) and very high
   value-per-token.

### Tier 3 — flag for human scrutiny

1. **`#[cfg(doc)]` macro skeleton + doc-wrapper split** (1.9) — applies
   only to macros with a dual `#[cfg(doc)]`/`#[cfg(not(doc))]` form.
2. **Build-script `fn main`** (1.6) — promote only when `src/`
   references cfgs the build script emits.
3. **Test file preamble + per-test fn index** (1.14) — large test files
   (~700+ tokens) with self-describing function names.
4. **Cross-file `#[test] fn` index** (1.14) — land deliberately when
   several related test files are all below-the-fold.
5. **Expected-output constants block** (1.14).
6. **Nested smoke crate** (1.15) — no-std smoke, MSRV smoke.
7. **`tests/ui/` directory listing** (1.15).
8. **Workspace-aware ranking** — detect `[workspace] members` and emit
   a per-crate description map.
9. **CHANGELOG head** (1.4), **CI matrix slices** (1.5), **contributor
   commands** (1.5) — opt-in promotions; each competes with below-fold
   notes on the same content.
