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

**Affected snapshots:** bareiron, commander, d2ts, d2ts_d2ts

The output shows README.md with only headings — zero body content. The pre-rewrite output showed introductory sections that tell the reader what the project is.

In bareiron, the full introductory section (lines 1-11) was shown: project description ("Minimalist Minecraft server for memory-restrictive embedded systems"), design priorities, Minecraft/protocol version numbers, and a compatibility warning.

In commander, the pre-rewrite showed lines 1-45: the description ("The complete solution for node.js command-line interfaces"), a language-switch note, and the full table of contents. The new output shows only headings — a reader can see the section structure but not what Commander.js is or does.

In d2ts_d2ts, the pre-rewrite showed lines 28-32: "D2TS is a TypeScript implementation of differential dataflow," what it does (incremental pipelines), and ElectricSQL integration. The new output shows 16 headings (h1 through h3) but zero body text.

This is the highest-value content in a repo for building a mental model. A reader seeing only headings knows the structure but not the purpose.

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

**Affected snapshots:** cmdk, cmdk_cmdk_src, enclosed_crypto

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

## 10. Markdown h1 body omitted while h2 bodies shown (regression)

**Affected snapshots:** cmdk

ARCHITECTURE.md's introductory text (lines 3-44, directly under `# Architecture`) explains the core design constraint of the library — wanting compound components, rejecting data arrays and render props, "a terrible, terrible constraint that we've spent 2 years fighting." The new output omits this entirely, jumping from `# Architecture` (line 1) to `## Approach` (line 46).

Meanwhile, h2 section bodies are shown in full: `## Example` gets 28 lines of code, `## Performance` and `## Groups` get their body lines. The pre-rewrite output showed the complete file including the intro.

The intro is the most valuable content in ARCHITECTURE.md — it's the "why" that gives meaning to the "how" in the sections below. `HeadingBody` has a flat base value (0.7) regardless of heading level, so the scheduler sees no reason to prefer the h1 intro body over h2 bodies. Since the h1 body is longer (~43 lines vs 1-28 lines for h2 bodies), its cost/benefit ratio is worse, and it loses the budget competition.

## 11. CommonJS entry point rendered empty — require/exports not captured

**Affected snapshots:** commander

Commander's `index.js` (24 lines) is the library's entry point. It shows the module structure: which classes are imported from `lib/`, factory functions (`createCommand`, `createOption`, `createArgument`), and all exports. The pre-rewrite output showed the full file. The new output shows only the filename with zero content — the file appears completely empty.

Root cause: the TypeScript tree-sitter query captures `import_statement` (ES6 imports) and `lexical_declaration` with an `identifier` name, but CommonJS patterns don't match:

1. `const { Argument } = require('./lib/argument.js')` — this is a `lexical_declaration`, but the name is an `object_pattern` (destructuring), not an `identifier`, so the query doesn't match.
2. `exports.program = new Command()` — this is an `expression_statement` with an assignment, not any captured pattern.

The result is zero extracted items, so the file contributes nothing to the output. This is a significant gap for JavaScript projects that use CommonJS (which is still the majority of npm packages). For entry point files especially, the exports list is often the single most useful piece of information about the library.

## 12. TypeScript function overloads shown individually, consuming budget on duplicates

**Affected snapshots:** d2ts_d2ts

`src/d2.ts` has 20 TypeScript overload signatures for `StreamBuilder.pipe()` (lines 119-157) — a common pattern for type inference in pipe-style APIs (rxjs uses the same approach). Each overload differs only in the number of generic type parameters. The output shows all 20 as separate `pipe …` lines:

```
   119→  pipe …
   121→  pipe …
   123→  pipe …
       ... (20 identical lines)
   157→  pipe …
```

This consumes ~20 lines of budget to convey one fact: "StreamBuilder has a pipe method." The pre-rewrite output showed only the class name without expanding methods, so the overloads weren't visible. The new output's per-method expansion causes each overload to appear as a separate FunctionName entry at base value 1.0, and since they're all public, none gets filtered. A reader seeing 20 `pipe …` lines gains nothing over seeing one.

## 13. TypeScript `export` statements not captured — barrel files render empty

**Affected snapshots:** d2ts, d2ts_d2ts

The TypeScript query captures `(import_statement) @symbol` but not `(export_statement)`. TypeScript re-exports (`export * from './foo.js'`, `export { bar } from './baz.js'`) parse as `export_statement` nodes, not `import_statement`, so they produce zero items.

This makes barrel files appear empty in the output. In d2ts, several barrel files are the most concise description of a module's API surface:

- `packages/d2ts/src/operators/index.ts` — 20 re-exports listing every operator (pipe, map, filter, join, reduce, count, distinct, etc.). This is the single best summary of d2ts's capabilities.
- `packages/d2ts/src/sqlite/index.ts` — 3 re-exports showing the sqlite module structure.
- `packages/d2ts/src/sqlite/operators/index.ts` — 12 re-exports listing sqlite-backed operators.
- `packages/d2ts/src/index.ts`, `packages/d2mini/src/index.ts` — top-level package entry points.
- `packages/d2ql/src/query-builder/index.ts` — `export { queryBuilder, type ResultFromQueryBuilder }`.
- `packages/d2ql/src/index.ts` — has a module-level JSDoc ("D2QL is a SQL-like query language for D2TS") plus 3 exports; both the doc and the exports are lost.

The pre-rewrite output showed these barrel files with content (e.g., `export * from './pipe.js'` through `export * from './orderBy.js'` with ellipsis). The new output shows them as blank file headers or omits them entirely, wasting header cost while conveying zero information.

Downstream effect: budget freed by the missing barrel content goes to lower-value items — private helper function names in `d2ql/src/functions.ts` (8 unexported functions like `upperFunction`, `lowerFunction`) and bulk type alias names in `d2ql/src/schema.ts` (30 type names, up from 1 in the pre-rewrite output) that add noise without the definitions (see issue #9).
