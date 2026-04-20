# anyhow — North Star

Revision pin: `769cba0b`

The crate is small but conceptually dense: a single trait-object error type
(`anyhow::Error`) with a hand-rolled vtable for one-word representation, a
`Context` trait for `.context(...)`/`.with_context(...)`, the `anyhow!` /
`bail!` / `ensure!` macros, plus `no_std` and `backtrace` cfg gating. The
ranking front-loads orientation (root listing, src/ listing, tagline,
public API surface) before the user-facing semantics (Display/Debug
output, Context, macros), then the cfg story, then internals, then tests.

## Batches

### 1.1 Root file/folder listing
- Content: rendered listing of the repo root, one entry per line:
  `Cargo.toml`, `build.rs`, `README.md`, `LICENSE-APACHE`, `LICENSE-MIT`,
  `rust-toolchain.toml`, `.gitignore`, `src/`, `tests/`, `.github/`.
- Cost: 34 tokens (helper: `echo "Cargo.toml\nbuild.rs\nREADME.md\nLICENSE-APACHE\nLICENSE-MIT\nrust-toolchain.toml\n.gitignore\nsrc/\ntests/\n.github/" | <helper> --stdin`)
- Notes: Highest-priority orientation. Tells the agent it's looking at a
  conventional Cargo crate with a build script and a `tests/` folder.

### 1.2 `src/` file listing
- Content: rendered listing of the 12 files in `src/`:
  `backtrace.rs`, `chain.rs`, `context.rs`, `ensure.rs`, `error.rs`,
  `fmt.rs`, `kind.rs`, `lib.rs`, `macros.rs`, `nightly.rs`, `ptr.rs`,
  `wrapper.rs`.
- Cost: 57 tokens (helper: `echo "<each filename on its own line>" | <helper> --stdin`)
- Notes: The names are highly diagnostic of structure (e.g., `kind.rs`,
  `nightly.rs`, `ptr.rs`) — do not omit.

### 1.3 README tagline + minimal install
- Content: `tests/fixtures/anyhow/README.md:9-17`
- Cost: 68 tokens (helper: `<helper> tests/fixtures/anyhow/README.md:9-17`)
- Notes: One-sentence description ("trait object based error type for easy
  idiomatic error handling") plus the `anyhow = "1.0"` snippet. Cheap
  semantic anchor for almost any query about the crate.

### 1.4 Cargo.toml package header
- Content: `tests/fixtures/anyhow/Cargo.toml:1-13`
- Cost: 119 tokens
- Notes: Crate name, version `1.0.101`, `categories = ["rust-patterns",
  "no-std"]`, MSRV `rust-version = "1.68"`, edition. Establishes that
  this is a published `1.0` crate with `no_std` support.

### 1.5 Cargo.toml features + dependencies
- Content: `tests/fixtures/anyhow/Cargo.toml:14-30`
- Cost: 170 tokens
- Notes: Two features (`std` default, `backtrace` optional with the
  `backtrace` crate); dev-deps include `thiserror`, `trybuild`,
  `rustversion`, `futures`, `syn`. The feature flag set is referenced
  pervasively in the source, so the agent needs this early.

### 2.1 `tests/` folder listing (top-level files + subfolders)
- Content: rendered listing — 13 `test_*.rs` files plus
  `compiletest.rs`, `common/`, `crate/`, `drop/`, `ui/`.
- Cost: 84 tokens (helper: `echo "test_autotrait.rs\n..." | <helper> --stdin`)
- Notes: The test file *names* are themselves a near-perfect API map
  (`test_chain`, `test_context`, `test_downcast`, `test_ensure`,
  `test_fmt`, `test_macros`, `test_repr`, `test_source`, ...).

### 2.2 Public top-level item declarations in `lib.rs` (no rustdoc)
- Content (concatenated, with elision markers between groups):
  - `tests/fixtures/anyhow/src/lib.rs:285-286` — `pub use anyhow as format_err;`
  - `tests/fixtures/anyhow/src/lib.rs:389-393` — `pub struct Error { inner: Own<ErrorImpl> }`
  - `tests/fixtures/anyhow/src/lib.rs:413-417` — `#[derive(Clone)] pub struct Chain<'a> { state: ... }`
  - `tests/fixtures/anyhow/src/lib.rs:468` — `pub type Result<T, E = Error> = core::result::Result<T, E>;`
  - `tests/fixtures/anyhow/src/lib.rs:616-628` — full `pub trait Context<T, E>` with both methods
  - `tests/fixtures/anyhow/src/lib.rs:648-652` — `pub fn Ok<T>(value: T) -> Result<T>`
  Each group separated by a `…` elision marker so the agent knows the
  rustdoc has been elided.
- Cost: ~245 tokens (helper: sum of the per-range counts above)
- Notes: This is the *type-level* public API at a glance. If the agent
  has only this plus the listings, it can already answer "what types are
  exported, what's the trait method shape" with zero follow-ups.

### 2.3 Mandatory `extern crate` + `mod ...;` block
- Content: `tests/fixtures/anyhow/src/lib.rs:246-263`
- Cost: 60 tokens (helper: `<helper> tests/fixtures/anyhow/src/lib.rs:246-263`)
- Notes: `extern crate alloc;`, `#[cfg(feature = "std")] extern crate std;`,
  then the 11 internal `mod ...;` declarations. Gives the agent the
  routing table from concept ("the macro parser") to file (`mod ensure`).

### 2.4 README "Details" — Result/`?`/.context/Caused by
- Content: `tests/fixtures/anyhow/README.md:21-67`
- Cost: 342 tokens
- Notes: First two bullets of the canonical tutorial:
  `Result<T, anyhow::Error>` / `anyhow::Result<T>` / `?` propagation,
  then attaching context with `.context(...)` / `.with_context(|| ...)`
  and the resulting "Caused by:" output. Doctrine and intent for the
  most common queries.
- Predecessor: 1.3

### 3.1 README "Details" — downcasting, backtrace, thiserror, `anyhow!`, `bail!`
- Content: `tests/fixtures/anyhow/README.md:68-122`
- Cost: 474 tokens
- Notes: Bullets 3-6 of the tutorial: `downcast_ref::<T>()`, the
  `RUST_BACKTRACE` / `RUST_LIB_BACKTRACE` env vars, deriving error
  types via `thiserror`, and the `anyhow!`/`bail!` macros. Predecessor
  edge to 2.4 (the bullets read as one continuous tutorial).
- Predecessor: 2.4

### 3.2 Crate-level rustdoc preamble (lib.rs, prose only)
- Content: `tests/fixtures/anyhow/src/lib.rs:1-13` — title/badges/lead
  sentence — followed by a `…` elision marker standing in for the long
  doc tutorial body (`14-189`), then `tests/fixtures/anyhow/src/lib.rs:191-208`
  for the no_std doc.
- Cost: ~412 tokens (helper: `<helper> tests/fixtures/anyhow/src/lib.rs:1-13 tests/fixtures/anyhow/src/lib.rs:191-208`; ranges are 218 + 174 plus a few tokens for the in-line ellipsis)
- Notes: README:21-122 (batch 2.4) has nearly the same tutorial; here we
  just tease the file-level header and pull the no-std caveat in early.
  The lib.rs doc body itself (lines 14-189) is below-the-fold.

### 3.3 Display representations rustdoc on `Error`
- Content: `tests/fixtures/anyhow/src/lib.rs:301-365`
- Cost: 552 tokens
- Notes: Documents what `"{}"` / `"{:#}"` / `"{:?}"` / `"{:#?}"` look
  like, including the literal "Caused by:" stack-backtrace example.
  This drives almost every formatting question. The full block is
  394 tokens at lines 288-389; here we trim the prose lead-in.
- Predecessor: 2.2

### 3.4 `Error` constructor and downcast signatures (no body)
- Content: signature ranges in `tests/fixtures/anyhow/src/error.rs`,
  each with full where-clause:
  - `30-33` `Error::new<E>(error: E) -> Self`
  - `77-80` `Error::msg<M>(message: M) -> Self`
  - `140` `Error::from_boxed(boxed_error: Box<dyn StdError + Send + Sync + 'static>) -> Self`
  - `372-375` `Error::context<C>(self, context: C) -> Self`
  - `432` `Error::backtrace(&self) -> &impl_backtrace!()` (cfg-gated)
  - `459` `Error::chain(&self) -> Chain`
  - `470` `Error::root_cause(&self) -> &(dyn StdError + 'static)`
  - `482-485` `Error::is<E>(&self) -> bool`
  - `490-493` `Error::downcast<E>(mut self) -> Result<E, Self>`
  - `554-557` `Error::downcast_ref<E>(&self) -> Option<&E>`
  - `568-571` `Error::downcast_mut<E>(&mut self) -> Option<&mut E>`
  - `622` `Error::into_boxed_dyn_error(self) -> Box<...>`
  - `663-665` `Error::reallocate_into_boxed_dyn_error_without_backtrace(self) -> Box<...>`
  Render with `…` elision markers between non-contiguous ranges.
- Cost: ~360 tokens (sum of the per-range counts above)
- Notes: This is the entire instance-method API of `Error`. With it
  plus 2.2, an agent answers "does anyhow have downcast?" / "how do I
  get the source chain?" without any follow-up reads.
- Predecessor: 2.2

### 3.5 Macro doc summaries: `bail!`, `ensure!`, `anyhow!`
- Content (concatenated):
  - `tests/fixtures/anyhow/src/macros.rs:1-10` — `bail!` summary
  - `tests/fixtures/anyhow/src/macros.rs:71-86` — `ensure!` summary
    (lifted from inside the `__ensure!` wrapping macro)
  - `tests/fixtures/anyhow/src/macros.rs:174-186` — `anyhow!` summary
- Cost: ~391 tokens
- Notes: Each summary captures equivalence ("`bail!` ≡ `return
  Err(anyhow!(…))`", "`ensure!` ≡ `if !$cond { return Err(anyhow!(…)); }`")
  and the `Result<_, anyhow::Error>` requirement. Critical for any
  user-code generation.

### 3.6 `bail!` and `anyhow!` macro definitions
- Content: `tests/fixtures/anyhow/src/macros.rs:56-68` (`macro_rules!
  bail`) and `tests/fixtures/anyhow/src/macros.rs:202-223`
  (`macro_rules! anyhow`).
- Cost: 298 tokens (helper: `<helper> tests/fixtures/anyhow/src/macros.rs:56-68 tests/fixtures/anyhow/src/macros.rs:202-223`)
- Notes: All three forms — literal, expr, and fmt. The `expr` arm
  shows the kind-dispatch pattern `(&error).anyhow_kind().new(error)`,
  which previews `kind.rs`.
- Predecessor: 3.5

### 3.7 `ensure!` user-facing macro definition (`#[cfg(doc)]` arm)
- Content: `tests/fixtures/anyhow/src/macros.rs:127-153`
- Cost: 213 tokens
- Notes: This is the `#[cfg(doc)]` form of `ensure!` — a tidy
  4-arm `macro_rules!` matching cond / cond+literal / cond+err /
  cond+fmt, exactly what users see in rustdoc. Vastly more readable
  than the recursive token-tree parser the real build uses (line
  155-172 + the giant `__parse_ensure!` in `ensure.rs`).

### 4.1 `Context` trait usage and downcast effects (rustdoc on `pub trait Context`)
- Content: `tests/fixtures/anyhow/src/lib.rs:469-518` (intro + first
  example) and `tests/fixtures/anyhow/src/lib.rs:519-541` (the
  "Effect on downcasting" rationale paragraph).
- Cost: ~517 tokens (helper: sum of 305 + 212)
- Notes: The downcasting compatibility rules are subtle and surprising;
  both examples (`SuspiciousError`, `HelperFailed`) are in
  `lib.rs:542-616` (below-the-fold). Predecessor edge to 2.2.
- Predecessor: 2.2

### 4.2 `Context` trait blanket impls (the actual dispatching logic)
- Content: `tests/fixtures/anyhow/src/context.rs:42-67` (impl
  `Context<T, E>` for `Result<T, E>` where `E: ext::StdError + ...`)
  and `tests/fixtures/anyhow/src/context.rs:90-112` (impl
  `Context<T, Infallible>` for `Option<T>`).
- Cost: ~371 tokens
- Notes: Together these say: `.context(...)` works on any `Result`
  whose error implements `std::error::Error`, *and* on any `Option<T>`
  (where `None` becomes a synthesized error from the context).
- Predecessor: 4.1

### 4.3 `Display` and `Debug` impls for `ErrorImpl` (the actual format code)
- Content: `tests/fixtures/anyhow/src/fmt.rs:6-67`
- Cost: 469 tokens
- Notes: The exact code emitting `"foo: bar"` (alt-display joins with
  ": "), `"…\n\nCaused by:\n    0: …\n    1: …"` (debug), and the
  conditional `Stack backtrace:` capitalization. Without this batch
  the agent can only describe the format from the docs; with it, it
  can answer questions like "where does the `: ` separator come from"
  in zero follow-ups.
- Predecessor: 3.4

### 4.4 `Chain` iterator definition (`chain.rs`)
- Content: `tests/fixtures/anyhow/src/chain.rs:1-102` (the entire
  file).
- Cost: 654 tokens
- Notes: `ChainState::{Linked, Buffered}`, the `Iterator`,
  `DoubleEndedIterator`, `ExactSizeIterator`, `Default` impls, and
  the cfg-conditional `pub(crate) use crate::Chain` re-export. The
  whole file is small and tightly coupled — splitting hurts.

### 4.5 `kind.rs` autoref-dispatch internals
- Content: `tests/fixtures/anyhow/src/kind.rs:1-46` (the giant
  doc-comment explaining tagged dispatch via autoref) plus
  `tests/fixtures/anyhow/src/kind.rs:55-121` (the three traits
  `AdhocKind`, `TraitKind`, `BoxedKind` and their `Adhoc::new`,
  `Trait::new`, `Boxed::new` impls).
- Cost: ~766 tokens (sum 373 + 393)
- Notes: Explains exactly why `anyhow!(expr)` resolves differently for
  `&str` vs `io::Error` vs `Box<dyn StdError + Send + Sync>`. The
  doc-comment alone (1-46) is one of the most pedagogically valuable
  comments in the crate.

### 5.1 Build-script feature-detection probe
- Content: `tests/fixtures/anyhow/build.rs:17-65`
- Cost: 482 tokens
- Notes: The whole `error_generic_member_access` decision tree —
  emits `cfg=std_backtrace` and `cfg=error_generic_member_access`
  depending on toolchain support. This is *the* source of every
  `#[cfg(error_generic_member_access)]` in `src/`, so the agent needs
  it to reason about the cfg matrix.

### 5.2 `cfg`-emission boilerplate (`build.rs` rest)
- Content: `tests/fixtures/anyhow/build.rs:71-96` — the `rustc < 81`
  / `rustc < 85` / `cargo:rustc-check-cfg` declarations.
- Cost: 318 tokens
- Notes: Names every cfg flag the project recognizes (`anyhow_no_core_error`,
  `anyhow_no_clippy_format_args`, etc.).
- Predecessor: 5.1

### 5.3 `backtrace.rs` cfg shim (no `capture` mod)
- Content: `tests/fixtures/anyhow/src/backtrace.rs:1-68`
- Cost: 401 tokens
- Notes: The `Backtrace` re-export decision tree (`std::backtrace` vs
  the `backtrace` crate vs an empty enum) plus the `impl_backtrace!`,
  `backtrace!()`, and `backtrace_if_absent!()` macros that propagate
  cfg-gated behavior into the rest of `src/`. The 339-line `mod
  capture { ... }` body (lines 70-409) is below-the-fold.

### 5.4 `wrapper.rs` — `MessageError`, `DisplayError`, `BoxedError`
- Content: `tests/fixtures/anyhow/src/wrapper.rs:1-84` (the entire
  file).
- Cost: 569 tokens
- Notes: The three `#[repr(transparent)]` newtypes that adapt
  message-only / display-only / boxed-dyn-error inputs into
  `StdError`-implementing values used by `Error::construct_from_*`.
  Compact and load-bearing; splitting hurts.

### 5.5 `nightly.rs` — generic-member-access glue
- Content: `tests/fixtures/anyhow/src/nightly.rs:1-58` (the entire
  file).
- Cost: 414 tokens
- Notes: Surface the build-probe exercises (`error::request_ref`,
  `Request::provide_ref`) and the small `MyError` proof-of-concept.
  Predecessor edge to 5.1 (the build script gates compilation of this
  file).
- Predecessor: 5.1

### 6.1 `Error` constructor docs (rustdoc, no signature)
- Content: `tests/fixtures/anyhow/src/error.rs:84-136`
- Cost: 453 tokens
- Notes: The big interop example for `Error::from_boxed` showing the
  `From<Report>` ↔ `From<anyhow::Error>` round-trip. Rare query, but
  almost catastrophic to omit if a user is writing a bridge crate.
- Predecessor: 3.4

### 6.2 `Error::context` rustdoc with `ParseError` example
- Content: `tests/fixtures/anyhow/src/error.rs:316-369`
- Cost: 454 tokens
- Notes: Documents *when* to call `error.context(...)` directly versus
  going through the `Context` trait; the `ParseError` example is the
  only place the "context computed from the error's payload" pattern
  is shown.
- Predecessor: 3.4

### 6.3 `Error::backtrace` and `Error::chain` rustdoc
- Content: `tests/fixtures/anyhow/src/error.rs:404-461`
- Cost: ~528 tokens (helper: `<helper> tests/fixtures/anyhow/src/error.rs:404-461`; equals 351 + 206 with overlap)
- Notes: Env-var doctrine for backtrace, and the canonical
  `underlying_io_error_kind` example for `chain()`. Predecessor edges
  to 3.4 (signatures live there).
- Predecessor: 3.4

### 6.4 `Error::into_boxed_dyn_error` rustdoc
- Content: `tests/fixtures/anyhow/src/error.rs:582-622`
- Cost: 475 tokens
- Notes: The non-reallocating "convert anyhow::Error back to a
  Box<dyn Error>" path and the trade-off it imposes (backtrace
  preserved via `request_ref::<Backtrace>`, downcast to original `&E`
  no longer possible). The sister method
  `reallocate_into_boxed_dyn_error_without_backtrace` (lines 631-672,
  ~464 tokens) makes the opposite trade-off; below-the-fold and one
  `Read` away once 3.4 has named the signature.
- Predecessor: 3.4

### 6.5 `ErrorImpl::error`, `backtrace`, `provide`, `chain` accessor methods
- Content: `tests/fixtures/anyhow/src/error.rs:957-1013`
- Cost: 526 tokens
- Notes: These are the unsafe accessors that the public methods on
  `Error` (and the `Display`/`Debug` impls) actually call into. The
  `Error::backtrace` panic-on-failure expectation lives here.
- Predecessor: 4.3

### 6.6 `Indented` formatter helper (and its tests)
- Content: `tests/fixtures/anyhow/src/fmt.rs:69-158`
- Cost: 477 tokens (helper: `<helper> tests/fixtures/anyhow/src/fmt.rs:69-158`)
- Notes: The numeric/non-numeric indentation logic that produces the
  `"    0: ..."` / `"    foo"` shapes the user sees. The three
  small `#[test]` cases (one_digit / two_digits / no_digits) are the
  cleanest spec for the format.
- Predecessor: 4.3

### 7.1 `test_fmt.rs` expected-output constants
- Content: `tests/fixtures/anyhow/tests/test_fmt.rs:14-67`
- Cost: 271 tokens
- Notes: Literal expected-string constants for `Display`, alt-Display,
  `Debug`, alt-Debug across a 3-level cause chain (`f → g → h`).
  These constants are an executable spec of the format better than
  any prose.

### 7.2 `test_context.rs` make_chain helper + downcast test
- Content: `tests/fixtures/anyhow/tests/test_context.rs:65-118`
- Cost: ~390 tokens (helper: `<helper> tests/fixtures/anyhow/tests/test_context.rs:65-118`; ~ 193 + 197)
- Notes: Demonstrates wrapping with `.context(MidLevel)` then
  `.context(HighLevel)` and reading back via `downcast_ref::<X>()` for
  each level. Canonical end-to-end example.

### 7.3 `test_chain.rs` — `Chain` iterator behavior
- Content: `tests/fixtures/anyhow/tests/test_chain.rs:1-69` (the
  entire file).
- Cost: 581 tokens
- Notes: Shows `chain.next()`, `.rev()`, `.next_back()`, `.len()`,
  `.size_hint()`, and `Chain::default()`. Provides the iteration
  *order* contract (outermost context first).

### 7.4 `tests/common/mod.rs` and `tests/drop/mod.rs` shared helpers
- Content: `tests/fixtures/anyhow/tests/common/mod.rs:1-14` plus
  `tests/fixtures/anyhow/tests/drop/mod.rs:1-53`.
- Cost: 355 tokens (helper: 81 + 274)
- Notes: Used by ~half of the test files. Makes the test bodies
  intelligible without follow-up reads.

### 7.5 `test_repr.rs` — single-word size + null-pointer optimization
- Content: `tests/fixtures/anyhow/tests/test_repr.rs:1-30` (the
  entire file).
- Cost: 169 tokens
- Notes: Asserts `mem::size_of::<Error>() == size_of::<usize>()` and
  the same for `Result<(), Error>`. This invariant is the entire
  rationale for the elaborate vtable in `error.rs`.

### 7.6 `tests/` index with one-line summaries
- Content: rendered file-by-file index of `tests/`, each name followed
  by a 4-10 word summary of what it tests (e.g.,
  `test_chain.rs (Chain iterator: next/rev/len/clone)`).
- Cost: 282 tokens (helper: see "Tests-folder index" stdin run above)
- Notes: Lets the agent jump from a topic ("autotraits", "FFI",
  "boxed", "convert", "macros") straight to the right test file with a
  single `Read`.

### 7.7 `tests/ui/` listing — paired `.rs` + `.stderr`
- Content: rendered listing of the seven UI test stems
  (`chained-comparison`, `empty-ensure`, `ensure-nonbool`, `must-use`,
  `no-impl`, `temporary-value`, `wrong-interpolation`), each annotated
  `.rs / .stderr`.
- Cost: 56 tokens
- Notes: Each pair is a `trybuild` compile-fail case asserting a
  specific diagnostic. Names alone tell the agent which footguns are
  guarded.

### 8.1 `Error` private vtable struct + `unsafe fn construct`
- Content: `tests/fixtures/anyhow/src/error.rs:288-316` (the
  `unsafe fn construct<E>` body) and
  `tests/fixtures/anyhow/src/error.rs:740-755` (the
  `struct ErrorVTable` definition).
- Cost: ~485 tokens (helper: 265 + 220)
- Notes: The thin-pointer trick — `Box<ErrorImpl<E>>` cast to
  `Own<ErrorImpl>` with a manual vtable. Required reading to modify
  any `construct_from_*` path.

### 8.2 `Error::construct_from_std` / `_adhoc` / `_display` / `_context` / `_boxed` shapes
- Content: `tests/fixtures/anyhow/src/error.rs:145-285`
- Cost: ~880 tokens (helper: `<helper> tests/fixtures/anyhow/src/error.rs:145-285`)
- Notes: The five family methods that wire vtables for each error
  source. Without 8.1 above, this is unreadable.
- Predecessor: 8.1

### 8.3 Concrete vtable functions (`object_drop`, `object_ref`, `object_boxed`, `object_downcast`)
- Content: `tests/fixtures/anyhow/src/error.rs:756-836`
- Cost: ~773 tokens (helper: `<helper> tests/fixtures/anyhow/src/error.rs:756-836`; equals 565 + 208)
- Notes: Implementations the vtable points at — the four `object_*`
  functions used by every constructed `Error`. The
  `context_*`/`context_chain_*` variants (lines 837-928) are
  ranked below-the-fold; they specialize the same patterns for
  `ContextError<C, E>`. This is the densest unsafe code in the crate.
- Predecessor: 8.1

### 8.4 `ErrorImpl<E>` layout, `vtable()` reader, `ContextError`
- Content: `tests/fixtures/anyhow/src/error.rs:930-955` plus
  `tests/fixtures/anyhow/src/error.rs:1015-1045` (StdError and
  Debug/Display blanket impls on `ErrorImpl<E>`).
- Cost: ~470 tokens (helper: 265 + 202)
- Notes: The `#[repr(C)]` placement of `vtable` first justifies the
  pointer trick used by `vtable()`.
- Predecessor: 8.1

### 8.5 `Error` ↔ `Box<dyn StdError + ...>` conversions and `AsRef` impls
- Content: `tests/fixtures/anyhow/src/error.rs:1047-1086`
- Cost: 304 tokens
- Notes: The three `From<Error> for Box<dyn StdError + ...>` impls and
  the two `AsRef`s, plus the marker `UnwindSafe` / `RefUnwindSafe`
  impls. Round-trip story for anyhow ↔ stdlib boxed errors.

### 8.6 `Error` `impl Drop`, `Display`, `Debug`, `From`, `Deref`
- Content: `tests/fixtures/anyhow/src/error.rs:691-738`
- Cost: 334 tokens
- Notes: Explains why `Error` formats and dereferences via the vtable
  — short and load-bearing.
- Predecessor: 8.1

### 8.7 `ptr.rs` — `Own`/`Ref`/`Mut` smart pointers
- Content: `tests/fixtures/anyhow/src/ptr.rs:1-188` (the entire
  file).
- Cost: 956 tokens
- Notes: Tiny support module used by every unsafe path in `error.rs`.
  Splitting hurts — the three types only make sense together.

### 9.1 `ensure.rs` runtime helpers (`Buf`, `render`, `BothDebug`/`NotBothDebug`)
- Content: `tests/fixtures/anyhow/src/ensure.rs:1-101`
- Cost: 665 tokens
- Notes: Explains the `(2 vs 1)` formatting suffix on
  `Condition failed: ...` errors when both sides implement `Debug`,
  and the no-alloc 40-byte `Buf` used to do it.

### 9.2 `ensure.rs` final macros: `__fancy_ensure!` and `__fallback_ensure!`
- Content: `tests/fixtures/anyhow/src/ensure.rs:884-934`
- Cost: 408 tokens
- Notes: The two terminal macros the parser dispatches to. Together
  they fully define what `ensure!` actually expands to — fancy form
  uses `BothDebug`/`NotBothDebug` autoref dispatch (predecessor 9.1),
  fallback handles the literal/expr/fmt forms.
- Predecessor: 9.1

### 9.3 `lib.rs` `__private` module
- Content: `tests/fixtures/anyhow/src/lib.rs:654-728`
- Cost: 410 tokens
- Notes: Items the macros need at expansion time:
  `format_err`/`must_use`/the `Bool` trait & impls (the last is what
  `ensure_nonbool.stderr` complains about). Required to understand any
  macro-related compilation error.

### 9.4 CI matrix essentials (`ci.yml`)
- Content: `tests/fixtures/anyhow/.github/workflows/ci.yml:19-50`
  (the `test` job + matrix + the three `cargo` invocations) plus
  `tests/fixtures/anyhow/.github/workflows/ci.yml:71-82` (minimal
  versions) plus `tests/fixtures/anyhow/.github/workflows/ci.yml:112-140`
  (clippy + miri jobs).
- Cost: ~672 tokens (helper: sum of the per-range counts: 330 + 97 + 247)
- Notes: Toolchain matrix (1.68 / 1.76 / 1.80 / 1.82 / stable / beta /
  nightly), miri pin (`nightly-2026-02-11`), `cargo check
  --no-default-features`, `cargo check --features backtrace`,
  `cargo clippy --tests -- -Dclippy::pedantic`. The `pre_ci`,
  `windows`, `doc`, and `outdated` jobs are below-the-fold.

### 9.5 `lib.rs` `Result<T>` doc + `anyhow::Ok` doc-lead
- Content: `tests/fixtures/anyhow/src/lib.rs:419-441` (the "one or two
  type parameters" subtlety) plus
  `tests/fixtures/anyhow/src/lib.rs:629-647` (the rationale paragraph
  preceding `pub fn Ok`).
- Cost: ~360 tokens (helper: 164 + 196)
- Notes: The non-obvious bits: `Result<T>` accepts one or two type
  parameters, and the `anyhow::Ok(value)` helper exists because
  `anyhow::Result::Ok(value)` cannot infer `E`. The longer worked
  example for `Result` (lines 442-467) is below-the-fold.
- Predecessor: 2.2

### 9.6 `Context::with_context` example (Option pattern)
- Content: `tests/fixtures/anyhow/src/context.rs:70-89`
- Cost: ~140 tokens (helper: `<helper> tests/fixtures/anyhow/src/context.rs:70-89`)
- Notes: The doctest showing `.context("there is no T")` on an
  `Option<T>` — only spec of the `Option` -> `Result` lift.
- Predecessor: 4.2

## Below-the-fold

The fixture's gross size is ~58k tokens; the cap is ~20k. The cumulative
content above is ~21k tokens. The omissions below are deliberate:

- `tests/fixtures/anyhow/LICENSE-APACHE` (1998 tokens) and
  `LICENSE-MIT` (216 tokens) — boilerplate; the SPDX in `Cargo.toml`
  already records `MIT OR Apache-2.0`.
- `tests/fixtures/anyhow/src/ensure.rs:103-882` (the giant
  `__parse_ensure!` recursive macro, ~17.5k tokens). Pure machinery to
  parse arbitrary Rust expressions for `ensure!`; nearly impossible to
  use even with full context. The user-facing ensure macro
  (`#[cfg(doc)]` arm) is in 3.7, the runtime helpers in 9.1, and the
  terminal macros in 9.2 — that triple is enough to answer almost any
  question without inspecting the parser.
- `tests/fixtures/anyhow/src/backtrace.rs:69-410` (the ~340-line
  `mod capture { ... }` providing a fallback `Backtrace` implementation
  via the `backtrace` crate, ~2.4k tokens). Only relevant when building
  with `--features backtrace` on a pre-1.65 toolchain that the CI
  matrix barely touches; one `Read` away once 5.3 has named the file.
- `tests/fixtures/anyhow/src/lib.rs:14-208` — body of the crate-level
  rustdoc (~1.6k tokens). Heavily redundant with README:23-122 (batch
  2.4 + 3.1); the small lead-in and the no_std caveat are in 3.2.
- `tests/fixtures/anyhow/src/lib.rs:209-247` — the `#![deny(...)]` /
  `#![allow(clippy::*)]` bag (~321 tokens). Lint preferences only.
- `tests/fixtures/anyhow/build.rs:99-207` — the `compile_probe`,
  `rustc_minor_version`, `cargo_env_var` helpers (~950 tokens). Only
  relevant if the agent is modifying the probe; the *decision* logic
  is in 5.1, and these helpers are one `Read` away.
- `tests/fixtures/anyhow/src/lib.rs:288-300` (the `Error` doc lead) and
  `lib.rs:394-417` (the `Chain` rustdoc + example) — covered by the
  signatures (2.2) plus 3.3 / 4.4.
- `tests/fixtures/anyhow/src/lib.rs:542-616` — the two extended
  `Context` downcast examples (`SuspiciousError` / `HelperFailed`,
  ~530 tokens). The conceptual statement is in 4.1; readers needing
  the full code are one `Read` away.
- `tests/fixtures/anyhow/src/error.rs:1-18` — `error.rs` use-statements
  (cheap to fetch; not load-bearing semantic content).
- `tests/fixtures/anyhow/src/error.rs:631-672` —
  `Error::reallocate_into_boxed_dyn_error_without_backtrace` rustdoc +
  body (~464 tokens). Counterpart to 6.4; nearly identical structure
  with the opposite trade-off.
- `tests/fixtures/anyhow/src/lib.rs:442-467` — long worked example for
  `Result<T>` (~190 tokens). Same shape as the README cluster_info
  example.
- `tests/fixtures/anyhow/src/error.rs:837-928` — the
  `context_downcast`, `context_drop_rest`, `context_chain_downcast`,
  `context_chain_drop_rest`, `context_backtrace` vtable functions
  (~990 tokens). Specializations of the patterns in 8.3 for
  `ContextError<C, E>` and `ContextError<C, Error>`. Same shape, more
  cases — one `Read` away once 8.3 is in context.
- `tests/fixtures/anyhow/.github/workflows/ci.yml` `pre_ci`, `windows`,
  `doc`, and `outdated` jobs (~520 tokens). Routine; the
  `pre_ci.yml@master` workflow is in another repo, the windows job
  just runs `cargo check --features backtrace`, and `outdated` is a
  scheduled freshness check.
- All `tests/test_*.rs` bodies not explicitly named above
  (`test_autotrait`, `test_backtrace`, `test_boxed`, `test_convert`,
  `test_downcast`, `test_ensure`, `test_ffi`, `test_macros`,
  `test_source`). Their *names* (in 7.6) plus the one-line summaries
  give precise jump targets; the bodies are short and trivially read.
  Notable individual omissions:
  - `tests/test_ensure.rs` (7285 tokens) — exotic-expression stress
    tests for the `__parse_ensure!` macro. Best ranked very low.
  - `tests/test_downcast.rs` (791 tokens) — the
    `LargeAlignedError(#[repr(align(64))])` case is the only piece
    not also exercised by `test_context.rs`.
- `tests/fixtures/anyhow/tests/crate/*` (84 tokens) — the secondary
  `anyhow_test` crate just re-exports `anyhow::*` under `#![no_std]`
  to verify the no-default-features build; the *fact* of it is in 9.4
  (the CI step).
- `tests/fixtures/anyhow/tests/ui/*.stderr` bodies — the listings (7.7)
  identify what each test asserts; the multi-hundred-token diagnostic
  text is one `Read` away.
- `tests/fixtures/anyhow/.github/FUNDING.yml` (6 tokens) and
  `.gitignore` (6 tokens) — orientation listing (1.1) is enough.
- `tests/fixtures/anyhow/rust-toolchain.toml` — only specifies
  `components = ["rust-src"]`; the MSRV is in `Cargo.toml`.
