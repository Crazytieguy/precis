# Output quality issues

## 1. Internal-plumbing files consume disproportionate budget

**Affected snapshots:** anyhow

Files like `ptr.rs` (46 lines), `backtrace.rs` (42 lines), and `wrapper.rs` (21 lines) are pure internal implementation with no public API surface, yet together they consume ~109 lines of output. `ptr.rs` shows `Own`, `Ref`, `Mut` raw pointer wrappers with every `Clone`/`Copy`/`Send`/`Sync` impl and every method — none of which a reader needs to understand anyhow. Similarly, `backtrace.rs` shows every `Debug` impl for internal types like `BacktraceFrame`, `BacktraceSymbol`, and `BytesOrWide`.

This budget would be far better spent on public API signatures (see issue 2).

## 2. Key public API signatures collapsed to `…`

**Affected snapshots:** anyhow

For a generic error library, the type bounds ARE the API contract. The output shows:
```
    30→    pub fn new …
    77→    pub fn msg …
   372→    pub fn context …
```

But the reader needs to see:
```
    30→    pub fn new<E>(error: E) -> Self
    31→    where
    32→        E: StdError + Send + Sync + 'static,
```

The old (pre-rewrite) output showed full signatures for these key methods. The new output collapses them while spending budget on internal files. Notably, the `downcast` family methods DO get full signatures shown (via their doc-first-line groups pulling in subsequent content), making the omission of `new`/`msg`/`context` signatures even more conspicuous.

## 3. Internal `pub(crate)` methods shown alongside public API

**Affected snapshots:** anyhow

In `error.rs`, five `pub(crate) fn construct_from_*` methods and `unsafe fn construct` are shown alongside the collapsed public API methods. These are internal implementation details that a reader doesn't need. They also give a misleading impression of the module's surface area — 6 internal constructors listed next to 4 collapsed public methods suggests they're similarly important.

## 4. Cargo.toml content lines lost vs pre-rewrite

**Affected snapshots:** anyhow

The pre-rewrite output showed actual package metadata:
```
     2→name = "anyhow"
     3→version = "1.0.101"
     6→description = "Flexible concrete Error type built on std::error::Error"
```

The new output shows only section headers (`[package]`, `[features]`, `[dependencies]`). For a library, the description and dependency list provide useful context about what the crate does and what it depends on.

## 5. Macro doc summaries lost vs pre-rewrite

**Affected snapshots:** anyhow

The pre-rewrite output showed first-line doc comments for `bail!` and `anyhow!`:
```
     1→/// Return early with an error.
      →…
    58→macro_rules! bail {
   174→/// Construct an ad-hoc error from a string or existing non-`anyhow` error
   175→/// value.
      →…
   204→macro_rules! anyhow {
```

The new output just shows `macro_rules! bail …` and `macro_rules! anyhow …`. Without the doc summaries, a reader has no idea what these macros do from the precis output alone.
