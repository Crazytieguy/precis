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

**Affected snapshots:** bareiron, commander, d2ts, d2ts_d2ts, log

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

**Affected snapshots:** cmdk, cmdk_cmdk_src, enclosed, enclosed_crypto, enclosed_lib, go_multierror, htmy, ky, ky_source_errors, mcphost_sdk

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

**Affected snapshots:** d2ts, d2ts_d2ts, enclosed, enclosed_crypto, enclosed_lib

The TypeScript query captures `(import_statement) @symbol` but not `(export_statement)`. TypeScript re-exports (`export * from './foo.js'`, `export { bar } from './baz.js'`) parse as `export_statement` nodes, not `import_statement`, so they produce zero items.

This has two effects:

**Effect 1: Barrel files render empty.** In d2ts, several barrel files are the most concise description of a module's API surface:

- `packages/d2ts/src/operators/index.ts` — 20 re-exports listing every operator (pipe, map, filter, join, reduce, count, distinct, etc.). This is the single best summary of d2ts's capabilities.
- `packages/d2ts/src/sqlite/index.ts` — 3 re-exports showing the sqlite module structure.
- `packages/d2ts/src/sqlite/operators/index.ts` — 12 re-exports listing sqlite-backed operators.
- `packages/d2ts/src/index.ts`, `packages/d2mini/src/index.ts` — top-level package entry points.
- `packages/d2ql/src/query-builder/index.ts` — `export { queryBuilder, type ResultFromQueryBuilder }`.
- `packages/d2ql/src/index.ts` — has a module-level JSDoc ("D2QL is a SQL-like query language for D2TS") plus 3 exports; both the doc and the exports are lost.

The pre-rewrite output showed these barrel files with content (e.g., `export * from './pipe.js'` through `export * from './orderBy.js'` with ellipsis). The new output shows them as blank file headers or omits them entirely, wasting header cost while conveying zero information.

Downstream effect: budget freed by the missing barrel content goes to lower-value items — private helper function names in `d2ql/src/functions.ts` (8 unexported functions like `upperFunction`, `lowerFunction`) and bulk type alias names in `d2ql/src/schema.ts` (30 type names, up from 1 in the pre-rewrite output) that add noise without the definitions (see issue #9).

In enclosed, barrel files like `packages/crypto/src/index.node.ts` and `index.web.ts` (23 lines each, showing the full crypto API surface via destructured `export const { deriveMasterKey, generateBaseKey, ... }`) render empty. Similarly `packages/lib/src/index.ts` (17 re-exports listing the entire library API) renders as just import lines.

**Effect 2: Functions exported via `export { name }` treated as private.** Many enclosed files use the declare-then-export pattern:

```typescript
export { createNoteRepository };
function createNoteRepository({ storage }: { storage: Storage }) { ... }
```

The function declaration IS captured (it appears as a `FunctionName` group), but since the `export` keyword is on the `export_statement` rather than on the declaration, the function receives the private visibility modifier (0.3×). Combined with depth modifiers at level 3-4 (0.7-0.4×), effective values drop to ~0.12-0.21. This causes the server's core domain files to render as empty headers despite substantial content:

- `notes.repository.ts` (123 lines, 6 functions including CRUD operations) — empty
- `notes.routes.ts` (139 lines, REST API endpoints with Zod validation) — empty
- `notes.usecases.ts` (32 lines, core business logic) — empty
- `notes.models.ts` (4 functions for note expiration/formatting) — empty

## 14. Server architecture lost to broad-but-shallow budget distribution (regression)

**Affected snapshots:** enclosed

The pre-rewrite output showed ~170 lines of `packages/app-server/` content: auth middleware (`authenticationMiddleware`, `protectedRouteMiddleware`), config definition, 6 middleware files (cors, errors, logger, storage, timeout, config), 3 storage factories (cloudflare-kv, fs-lite, memory), notes domain types with full bodies, notes tasks, shared errors, and validation utilities. A reader could understand: Hono middleware stack → auth flow → storage abstraction → notes CRUD → task scheduling.

The new output shows ~50 lines of server content: entry points, function/type names for `server.ts`/`server.types.ts`, and constant/type names from the notes domain. The middleware layer, auth system, and storage factories are completely absent — their directories appear only as folder entries (`auth/`, `config/`, `middlewares/`, `storage/factories/`). The notes domain files are present as headers but render empty (see #13 effect 2).

Two contributing causes:

1. **Depth penalty on deep monorepo structures.** The middleware files at `packages/app-server/src/modules/app/middlewares/` have effective_depth 4 (after `packages` and `src` are normalized), yielding depth_factor 0.4. Their public functions get effective value 0.4 — enough in isolation, but uncompetitive against the volume of shallower content across 7 packages.

2. **Budget redistribution without architectural weighting.** The old output over-allocated to `packages/app-client/` (~230 lines, mostly shadcn-solid UI components). The new output correctly reduced that, but the freed budget spread evenly across all packages (more crypto, CLI, and lib internals) rather than flowing to the server. The result: broader coverage with no single package covered deeply enough to convey its architecture. The server — which defines the entire REST API, storage abstraction, and auth flow — is the biggest casualty.

## 15. Private function bodies shown instead of class/method docstrings (regression)

**Affected snapshots:** htmy_renderer

The new output shows full implementation bodies of private methods while omitting class docstrings and public method docstrings that the pre-rewrite output showed. The net effect is ~79 lines of function bodies replacing ~96 lines of docstrings — a strict loss of understanding per token.

**baseline.py** is the clearest example. The old output showed the `Renderer` class docstring (explaining it's the baseline renderer, when to use it, and how it relates to other renderers), plus first-line docstrings for `render()`, `stream()`, and `__init__()`. The new output drops all of these and instead shows the full bodies of `_stream` (23 lines) and `_stream_one` (32 lines) — private methods whose logic is standard recursive rendering dispatch (isinstance checks, iteration).

**default.py** similarly loses the `Renderer` class docstring ("resolves component trees by converting them to a linked list"), the `_ComponentRenderer` docstring, and method docstrings for `__init__`/`render`/`run`. In their place, the full body of the module-level `_render_component` function is shown (15 lines of similar isinstance dispatch logic).

**typing.py** loses the `RendererType` and `StreamingRendererType` protocol class docstrings and method docstrings. Instead shows full bodies of `is_renderer` (3 lines: `getattr(obj, "render", None)`) and `is_streaming_renderer` (6 lines) — trivial type guards.

**context.py** loses the `RendererContext` class docstring and the complete `from_context` method body (which was 16 lines in a 28-line file — the old output reasonably showed the whole thing). The new output shows only 3 lines for this file.

The old output built a clear mental model: two renderer classes (baseline for debugging/benchmarking, default for production), a context utility, protocol types. The new output shows how dispatch loops work but not what the classes are for.

The heuristic values suggest this shouldn't happen — `ClassDocFirst` (base 0.4, public) should beat private `FunctionBody` (base 0.2 × visibility 0.3 = effective 0.06). Something in the scheduling is causing private function bodies to win budget over public class documentation.

## 16. Duplicated `#define` macros from conditional compilation branches

**Affected snapshots:** krep

In `krep.c`, SIMD feature flag macros are defined in multiple `#ifdef`/`#elif`/`#else` branches:

```c
#if defined(__AVX512F__) && defined(__AVX512BW__)
#define KREP_USE_AVX512 1
#define KREP_USE_AVX2 1
#define KREP_USE_SSE42 1
#elif defined(__AVX2__)
#define KREP_USE_AVX512 0
#define KREP_USE_AVX2 1
#define KREP_USE_SSE42 1
#else
#define KREP_USE_AVX512 0
#define KREP_USE_AVX2 0
#endif
```

Tree-sitter captures every `preproc_def` node regardless of which preprocessor branch it's in. The output shows the same macro name 2-3 times:

```
    49→#define KREP_USE_AVX512 …
    50→#define KREP_USE_AVX2 …
    51→#define KREP_USE_SSE42 …
    54→#define KREP_USE_AVX512 …
    55→#define KREP_USE_AVX2 …
    56→#define KREP_USE_SSE42 …
    58→#define KREP_USE_AVX512 …
    59→#define KREP_USE_AVX2 …
    66→#define KREP_USE_SSE42 …
    73→#define KREP_USE_NEON …
```

10 lines for 4 unique macro names. A reader would be confused about why the same symbol is defined three times. The pre-rewrite output didn't show these at all, instead showing more informative constants (`MAX_PATTERN_LENGTH 1024`, `LIKELY(x)`).

## 17. C header file budget reduced — key API declarations and struct bodies lost (regression)

**Affected snapshots:** krep

The rewrite dramatically shifted budget from `krep.h` (321 lines, the API header) to `krep.c` (5287 lines, the implementation). krep.h went from ~111 output lines to ~62, while krep.c went from ~15 to ~54. The result is a strictly worse mental model of the project.

**Struct bodies lost.** The pre-rewrite output showed the complete `search_params_t` struct (30 lines) with all fields — pattern fields, search options (`case_sensitive`, `use_regex`, `whole_word`, etc.), compiled regex pointer, Aho-Corasick trie pointer, max_count. This is the single most important type in the codebase — every search function takes it. Similarly, `thread_data_t` (19 lines) and `match_position_t` (5 lines) were shown with all fields. The new output truncates all of these to just `typedef struct search_params …`.

**15 function declarations lost.** The pre-rewrite output showed all 30 function declarations from krep.h. The new output shows only 15. Missing:

- `search_file`, `search_string` — two of the three public API functions
- `boyer_moore_search`, `kmp_search`, `regex_search`, `memchr_search`, `memchr_short_search` — the core search algorithm declarations
- `simd_sse42_search`, `simd_avx2_search`, `simd_avx512_search` — SIMD variants
- `thread_pool_submit`, `thread_pool_wait_all` — thread pool API
- `match_result_add`, `match_result_free`, `match_result_merge` — result management

These are replaced by `@brief` doc comment blocks (~12 lines of doc + ellipsis for functions already named) and section header comments (`/* --- Helper Functions --- */`).

**The budget went to redundant krep.c content.** krep.c gained ~39 output lines: 30 `#define` lines (including 10 duplicated SIMD flags from issue #16) and function names that largely duplicate krep.h declarations. A reader seeing `1389→uint64_t regex_search …` in krep.c gains nothing if `regex_search` is already declared in krep.h — and loses information if the header declaration was dropped to make room.

Root cause: the 2.5× `header_factor` doesn't compensate for krep.c's 16:1 line count advantage. krep.c's many ConstName groups (30 `#define` items at base_value 1.0 with sublinear scaling) pull substantial budget. In the old output, krep.c got 15 lines (2 constants + 2 gitignore struct bodies) and krep.h got 111 — this was the right distribution for a C project where the header IS the API.

## 18. Enum bodies elided while repetitive method signatures consume budget

**Affected snapshots:** log

In `src/lib.rs`, the `Level` and `LevelFilter` enums are the core of the library — their variants (Error, Warn, Info, Debug, Trace) are arguably the most important content in the entire crate. The output elides both enum bodies (`pub enum Level …`, `pub enum LevelFilter …`) while spending ~36 lines on individual method signatures for these same types, including low-value methods like `increment_severity`, `decrement_severity`, `from_usize`, and `as_str` — shown for BOTH enums since they have parallel impls.

The pre-rewrite output showed the full `Level` body with all variants and doc comments (lines 475-499, ~24 lines). The new output drops this and also drops the `LevelFilter` body (lines 636-649).

`EnumBody` has the highest base_value of any TsGroupKey (1.5), but the body is ~24 lines including per-variant doc comments. Many individual `FunctionName` entries (base_value 1.0 each, ~2 tokens each) have competitive or better value/cost ratios, so the scheduler fills the budget with small method entries before committing to the larger enum body block.

Additional budget pressure comes from `src/__private_api.rs` (~20 lines), a file whose module doc starts with "WARNING: this is not part of the crate's public API and is subject to change at any time." This file is not flagged by `is_deprioritized_file()` because no rule matches `__`-prefixed source files. The old output also showed this file — it's not a regression, but the 20 lines would be better spent on the missing enum bodies.

## 19. Go doc comments not detected for type declarations (regression)

**Affected snapshots:** mcphost_sdk

The Go tree-sitter query captures `type_spec` and `type_alias` as `@symbol`, but these are inner nodes within `type_declaration`. Doc comments in Go are siblings of `type_declaration`, one AST level above. `compute_doc_start_line` checks `node.prev_named_sibling()`, which for `type_spec` finds nothing — there are no named siblings within `type_declaration` before it.

Two effects:

1. **Doc comments never shown.** `documented` is always `false` for Go type declarations, so `StructDocFirst`, `InterfaceDocFirst`, and `TypeAliasDocFirst` groups are never spawned. In mcphost_sdk, the MCPHost struct doc ("provides programmatic access to mcphost functionality...") and Options struct doc ("configures MCPHost creation with optional overrides...") are both lost. The pre-rewrite output showed all four type doc comments in this fixture.

2. **Type declarations deprioritized.** `documented: false` applies a 0.5× `documented_contribution` penalty, making type declarations compete at half their natural value.

Functions and methods are unaffected — `function_declaration` and `method_declaration` ARE top-level nodes in Go's tree-sitter grammar, so their doc comments are reachable via `prev_named_sibling()`. The asymmetry is visible in mcphost_sdk's output: all 8 method doc comments are shown while all 4 type doc comments are missing.
