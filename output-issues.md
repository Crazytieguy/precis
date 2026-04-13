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

## 6. README body content dropped — only headings shown

**Affected snapshots:** bareiron

The output shows README.md with only the h1 and h2 headings — zero body content:
```
README.md
     1→# bareiron
    12→## Quick start
    17→## Compilation
    28→## Configuration
    39→## Non-volatile storage (optional)
    48→## Contribution
```

The pre-rewrite output showed the full introductory section (lines 1-11): project description ("Minimalist Minecraft server for memory-restrictive embedded systems"), design priorities, Minecraft/protocol version numbers, and a compatibility warning. This is the highest-value content in the repo for building a mental model. A reader seeing only headings would know the section structure but not what bareiron is or does.

## 7. C `#define` values truncated while verbose comments consume budget

**Affected snapshots:** bareiron

In `include/globals.h`, the output shows comment lines above each `#define` but truncates the actual values:
```
    18→// TCP port, Minecraft's default is 25565
    19→#define PORT …
    34→// Max render distance, determines how many chunks to send
    35→#define VIEW_DISTANCE …
```

The pre-rewrite showed values directly: `#define PORT 25565`, `#define VIEW_DISTANCE 2`. The pre-rewrite approach is more compact (one line vs two) and often more informative — `VIEW_DISTANCE 2` instantly conveys the server's minimalist constraints, while `#define VIEW_DISTANCE …` tells you nothing beyond the name. globals.h uses ~80 output lines (~¼ of the 4000-token budget), much of it on comments that restate the `#define` name.

## 8. CommunityHealth files not deprioritized at root level

**Affected snapshots:** chronos

CONTRIBUTING.md gets ~53 lines of output — more than any individual source file — for a boilerplate Amazon open source contributing guide ("how to file bugs", "how to send PRs"). The pre-rewrite output didn't show CONTRIBUTING.md content at all.

Two root causes:

1. **`files_contribution()` doesn't deprioritize root-level CommunityHealth files.** The `is_root_dir` branch only boosts Readme/Architecture to 1.5 and treats everything else as 1.0. The 0.1 CommunityHealth deprioritization only applies in the non-root branch. So CONTRIBUTING.md's child TsGroups compete at full modifier value.

2. **`is_boilerplate_heading()` doesn't match multi-word headings.** It splits on dashes (`-`, `—`, `–`) but not spaces, then does exact matches. So "Contributing Guidelines" becomes stem `"contributing guidelines"` which doesn't match `"contributing"`. Only 1 of 7 headings in CONTRIBUTING.md ("Code of Conduct") gets the boilerplate modifier. Similarly, "Security issue notifications" doesn't match `"security"`.

The combined effect: CONTRIBUTING.md headings get base_value 1.0 (h1) / 0.6 (h2) with modifier 1.0, competing on near-equal footing with README headings (modifier 1.5). This allocates ~15% of the 4000-token budget to content that builds zero understanding of the codebase.

## 9. Type alias and const bodies missing from taxonomy (regression)

**Affected snapshots:** cmdk_cmdk_src

The taxonomy has Body groups for functions (`FunctionBody`), structs (`StructBody`), and enums (`EnumBody`), but none for type aliases, const declarations, interfaces, classes, or traits. The `*Name` rendering truncates after the identifier, so the entire definition is lost.

Pre-rewrite output showed type definitions and const values:
```
    10→type Children = { children?: React.ReactNode }
    24→type SeparatorProps = DivProps & {
    25→  /** Whether this separator should always be rendered. Useful if you disable automatic filtering. */
      →  …
   154→const GROUP_SELECTOR = `[cmdk-group=""]`
   169→const Command = React.forwardRef<HTMLDivElement, CommandProps>((props, forwardedRef) => {
```

Post-rewrite output truncates all of these to just the name:
```
    10→type Children …
    24→type SeparatorProps …
   154→const GROUP_SELECTOR …
   169→const Command …
```

For cmdk, the 12+ `type` aliases define component props — they ARE the public API surface. `type ItemProps` having `disabled`, `onSelect`, `value`, `keywords`, `forceMount` properties is the most important thing to know about the Item component. The current output hides all of this, spending budget instead on internal helper function bodies (e.g. `findNextSibling` gets 8 lines of full body, `useScheduleLayoutEffect` gets 14 lines).
