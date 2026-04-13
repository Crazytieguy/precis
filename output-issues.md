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

## 4. Cargo.toml / pyproject.toml content lines lost vs pre-rewrite

**Affected snapshots:** anyhow, sps, sps_core, toasty, tomli

The pre-rewrite output showed actual package metadata:
```
     2→name = "anyhow"
     3→version = "1.0.101"
     6→description = "Flexible concrete Error type built on std::error::Error"
```

The new output shows only section headers (`[package]`, `[features]`, `[dependencies]`). For a library, the description and dependency list provide useful context about what the crate does and what it depends on.

In sps_core, the old output showed 24 lines of Cargo.toml including the full package metadata (name, version, description, authors, license, repository) and all 14 dependencies (sps-net, sps-common, anyhow, tokio, reqwest, serde, etc.). The new output shows only `[package]` (line 1) and `[dependencies]` (line 10).

In sps, the workspace has 3 sub-crate Cargo.toml files (sps-common, sps-core, sps-net). The pre-rewrite showed package name, version, and key dependencies for each (e.g., `sps-common` version 0.1.56, `sps-net` depending on `sps-common`). The new output shows only `[package]` and `[dependencies]` headers. In a multi-crate workspace, the inter-crate dependency lines (`sps-net = "0.1.56"`, `sps-common = "0.1.56"`) tell a reader the dependency graph between crates.

In toasty, the workspace Cargo.toml pre-rewrite showed `resolver = "2"` and the beginning of the `members` list (`"crates/toasty"`, `"crates/toasty-cli"`, ...) — instantly telling a reader this is a multi-crate workspace and which crates exist. The new output shows only `[workspace]` (line 1) and `[workspace.dependencies]` (line 37). Similarly, `toasty-sql/Cargo.toml` pre-rewrite showed the full file (name, version, edition, publish=false, `toasty-core.workspace = true`); the new output shows the same content, which is good — but 6 other crate Cargo.toml files show only header-only `[package]` / `[dependencies]` lines.

In tomli, pyproject.toml pre-rewrite showed `name = "tomli"`, `version = "2.4.0"`, `description = "A lil' TOML parser"`, and the build backend. The new output shows only section headers — and actually more of them (12 headers including `[tool.tox.env_run_base]`, `[tool.coverage.run]`, `[tool.coverage.report]`, three `[[tool.mypy.overrides]]`). The tool configuration sections are less useful than the package identity that was lost.

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

**Affected snapshots:** bareiron, commander, d2ts, d2ts_d2ts, log, mcphost, mdbook, mdbook_guide_src, pluggy, semver, soluna, sps, sqlite_vec, toasty, typeguard, vaul

The output shows README.md with only headings — zero body content. The pre-rewrite output showed introductory sections that tell the reader what the project is.

In typeguard, the README.rst (49 lines) explains the library's purpose (runtime type checking for PEP 484 annotations), the two principal approaches (check_type function vs code instrumentation), and the two instrumentation options (@typechecked vs import hook). The pre-rewrite showed the entire file. The new output shows just the bare filename with zero content — even worse than heading-only, because RST has no tree-sitter parser. Additionally, 7 docs/*.rst files (api.rst, extending.rst, features.rst, index.rst, userguide.rst, versionhistory.rst, contributing.rst) all render as bare filenames for the same reason. The old output showed headings and body for each (e.g., api.rst showed "API reference" + "Type checking" section, userguide showed "User guide" + "Checking types directly" section). That's 8 RST files (README + 7 docs) consuming 8 file headers for zero content.

In pluggy, the README.rst contains a complete working example (69 lines in the pre-rewrite output) demonstrating the entire hook specification and implementation API — `HookspecMarker`, `HookimplMarker`, `PluginManager`, plugin registration, and hook calling. This is the single best introduction to what pluggy is and how to use it. The new output shows zero README content. The budget goes instead to CLAUDE.md (~50 lines of AI config, see issue #8) and pyproject.toml towncrier type definitions (~30 lines of repetitive `[[tool.towncrier.type]]` sections).

In bareiron, the full introductory section (lines 1-11) was shown: project description ("Minimalist Minecraft server for memory-restrictive embedded systems"), design priorities, Minecraft/protocol version numbers, and a compatibility warning.

In commander, the pre-rewrite showed lines 1-45: the description ("The complete solution for node.js command-line interfaces"), a language-switch note, and the full table of contents. The new output shows only headings — a reader can see the section structure but not what Commander.js is or does.

In d2ts_d2ts, the pre-rewrite showed lines 28-32: "D2TS is a TypeScript implementation of differential dataflow," what it does (incremental pipelines), and ElectricSQL integration. The new output shows 16 headings (h1 through h3) but zero body text.

In mdbook_guide_src, the root README is the mdBook project introduction. The pre-rewrite showed lines 1-33: the full description ("**mdBook** is a command line tool to create books with Markdown"), the feature list (search, syntax highlighting, themes, preprocessors, backends), and a guide introduction. The new output shows only `# Introduction`, `## Contributing`, `## License`. Meanwhile, nested READMEs (cli/, for_developers/, format/, guide/) all get their body content, and the budget goes to deep h3/h4 headings across reference pages (format/configuration/renderers.md gets ~12 heading lines, format/markdown.md gets ~10).

In mdbook, the root README loses its one-line description ("mdBook is a utility to create modern online books from Markdown files") and user guide links. More impactfully, 6 of 8 crate READMEs lose their one-sentence descriptions — the pre-rewrite showed "This is the base support library... intended for internal use only" (mdbook-core), "This is the Rust library to implement a preprocessor" (mdbook-preprocessor), etc. In a multi-crate workspace, these descriptions are how a reader understands the crate decomposition: which crates are public API vs internal, and what each provides. The new output shows 8 bare `# crate-name` headings that convey the names but not the purpose or stability guarantees.

In sps, the README opens with a `[!WARNING]` blockquote (lines 3-22) announcing the project is being scrapped in favor of sps v2, with architectural rationale and a link to the new repo. This is the single most important piece of information about sps. The pre-rewrite showed all of it; the new output shows only 9 heading lines. The new output also adds boilerplate headings ("Contributing", "License") that the old correctly omitted.

In toasty, the README's h1 body (lines 3-5: "**Current status: Incubating - Toasty is not ready for production usage. The API is still evolving and documentation is lacking.**") is the single most important context about the project — it sets expectations for everything else. The pre-rewrite output showed this; the new output drops it and instead shows a `### Contribution` section (lines 117-121) with standard MIT license boilerplate. The boilerplate displaces the status warning because it's under a lower heading whose body is shorter and thus has a better cost/value ratio.

In vaul, the README is a 3-line deprecation notice (blockquote, no headings): "This repo is unmaintained. I might come back to it at some point, but not in the near future." The pre-rewrite showed this content; the new output shows the bare filename. This is a headingless markdown file — with no `Heading` groups, there are no `HeadingBody` groups to carry the text. The deprecation status is the single most important context about the project.

This is the highest-value content in a repo for building a mental model. A reader seeing only headings knows the structure but not the purpose.

## 7. C `#define` values truncated while verbose comments consume budget

**Affected snapshots:** bareiron, neco, soluna, sqlite_vec

In bareiron's `include/globals.h`, the output shows comment lines above each `#define` but truncates the actual values:
```
    18→// TCP port, Minecraft's default is 25565
    19→#define PORT …
    34→// Max render distance, determines how many chunks to send
    35→#define VIEW_DISTANCE …
```

The pre-rewrite showed values directly: `#define PORT 25565`, `#define VIEW_DISTANCE 2`. The pre-rewrite approach is more compact (one line vs two) and often more informative — `VIEW_DISTANCE 2` instantly conveys the server's minimalist constraints, while `#define VIEW_DISTANCE …` tells you nothing beyond the name. globals.h uses ~80 output lines (~¼ of the 4000-token budget), much of it on comments that restate the `#define` name.

In neco's `neco.h`, the 19 error codes and 6 time constants are all truncated:
```
   360→#define NECO_OK …
   361→#define NECO_ERROR …
   341→#define NECO_NANOSECOND …
```

The pre-rewrite showed complete lines with values and trailing doxygen descriptions: `#define NECO_OK 0 ///< Successful result (no error)`, `#define NECO_SECOND INT64_C(1000000000)`. For a C library, error codes ARE the error model — `NECO_OK 0` tells you success returns zero, `NECO_TIMEDOUT -10` tells you specific failure modes. The time constant values tell you the API uses nanoseconds. Truncating these to just names removes the most informative part of each line.

## 9. Type alias and const bodies missing from taxonomy (regression)

**Affected snapshots:** cmdk, cmdk_cmdk_src, enclosed, enclosed_crypto, enclosed_lib, go_multierror, htmy, ky, ky_source_errors, mcphost_sdk, microbootstrap, microbootstrap_instruments, mitt, nano_vllm, nano_vllm_engine, pluggy, py3xui, py3xui_api, semver, semver_internal, superstruct, tock, tock_internal_core

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

In pluggy, `HookspecOpts` and `HookimplOpts` are `TypedDict` classes whose field definitions ARE the API contract for hook specifications and implementations. The pre-rewrite output showed all fields with docstring comments — `firstresult: bool`, `historic: bool`, `warn_on_impl: Warning | None` for HookspecOpts, and `wrapper: bool`, `hookwrapper: bool`, `optionalhook: bool`, `tryfirst: bool`, `trylast: bool`, `specname: str | None` for HookimplOpts. The new output truncates both to `class HookspecOpts …` and `class HookimplOpts …`. A reader can't tell what options are available for hook specs or implementations.

In mitt, the entire library is one 123-line file (`src/index.ts`) whose API surface is defined by 6 type aliases and 1 interface. The pre-rewrite output showed `export type EventType = string | symbol`, `export type Handler<T = unknown> = (event: T) => void`, `export interface Emitter<Events extends Record<EventType, unknown>> { all: EventHandlerMap<Events>; ... }` — the complete type system that IS the library. The post-rewrite output truncates all types to `export type EventType …`, `export type Handler …`, `export interface Emitter …`. The Emitter interface loses both its generic constraint and its `all` property field (only method signatures survive, via the method extraction path). A reader can see there are 6 types and 1 interface but learns nothing about what they contain.

In microbootstrap_instruments, the same truncation loses Python class inheritance chains. The pre-rewrite output showed `class CorsInstrument(Instrument[CorsConfig]):` — telling the reader both the base class and which config type each instrument uses. The post-rewrite output shows `class CorsInstrument …`. For a package built entirely on the Instrument/Config pattern, the inheritance chain IS the architecture. This affects every class in the snapshot: `Instrument(abc.ABC, typing.Generic[InstrumentConfigT])` → `Instrument …`, `InstrumentBox` loses its `@dataclasses.dataclass` decorator and field definitions (`__instruments__: list[type[Instrument[typing.Any]]]`), config classes like `OpentelemetryConfig(BaseInstrumentConfig)` → `OpentelemetryConfig …`. The freed budget goes to function bodies (e.g. `PyroscopeSpanProcessor.on_start` gets 8 lines of span-tagging implementation, `_is_root_span` and `_format_span` get full bodies) — implementation details less valuable than the class hierarchy they displaced.

In the full microbootstrap snapshot, the same issue extends to configuration and settings classes. The pre-rewrite output showed `FastApiConfig` with all 33 dataclass fields (debug, routes, title, openapi_url, middleware, exception_handlers, etc.) — the complete set of options a user can pass to configure FastAPI. Similarly `FastStreamConfig` (12 fields), `LitestarConfig(AppConfig)` with its custom logging setup, and `BaseServiceSettings(pydantic_settings.BaseSettings)` with 6 settings fields plus `model_config`. The post-rewrite output truncates all of these to just `class FastApiConfig …`, `class BaseServiceSettings …`, etc. Additionally, the settings composition classes — `LitestarSettings(BaseServiceSettings, ServerConfig, LoggingConfig, OpentelemetryConfig, SentryConfig, LitestarPrometheusConfig, SwaggerConfig, CorsConfig, HealthChecksConfig, PyroscopeConfig)` — are truncated to `class LitestarSettings …`, hiding the 10-class multiple inheritance chain that IS the library's configuration architecture. The pre-rewrite also showed `Instrument(abc.ABC, typing.Generic[InstrumentConfigT])` with its 3 field annotations and `ApplicationBootstrapper` with its generic parameters. The freed budget goes to 20 additional README h3/h4 headings (28 heading lines vs 8 in the old output) that repeat the instrument/framework structure already visible from the source files, plus full method signatures for utility classes.

In nano_vllm, the same truncation loses Python dataclass field definitions across the entire project. The pre-rewrite output showed `Config` with all 12 dataclass fields (model, max_num_batched_tokens, max_num_seqs, max_model_len, gpu_memory_utilization, tensor_parallel_size, enforce_eager, etc.) — the complete configuration surface for the inference engine. `SamplingParams` showed its 3 fields (temperature, max_tokens, ignore_eos). `Context` showed all 8 fields defining the attention computation state (is_prefill, cu_seqlens_q/k, slot_mapping, block_tables, etc.). The post-rewrite output truncates all of these to `class Config …`, `class SamplingParams …`, `class Context …`. Additionally, `class LLM(LLMEngine): pass` — a one-line class whose inheritance chain is its entire meaning — becomes `class LLM …`, and `Qwen3ForCausalLM.packed_modules_mapping` (a dict mapping weight names to merged/sharded names for tensor parallelism) is lost. The pre-rewrite also showed `pyproject.toml` with full content including dependencies (`torch>=2.4.0`, `triton>=3.0.0`, `transformers>=4.51.0`, `flash-attn`, `xxhash`) — instantly telling a reader the runtime requirements. The post-rewrite output omits `pyproject.toml` entirely.

In nano_vllm_engine, the missing ClassBody loses Python Enum variant definitions. `SequenceStatus(Enum)` defines the three states (WAITING, RUNNING, FINISHED) that drive the scheduler's state machine — `schedule()` moves sequences to RUNNING, `preempt()` moves them to WAITING, `postprocess()` moves them to FINISHED. The pre-rewrite output showed all three variants plus the `(Enum)` base class. The post-rewrite output shows `class SequenceStatus …` — a reader doesn't know it's an enum or what states exist. Similarly, `Sequence` class attributes (`block_size = 256`, `counter = count()`) are lost. The `block_size` constant is referenced throughout the codebase (BlockManager, ModelRunner) and its value (256) is essential context for understanding the block allocation logic.

In semver_internal, `constants.js` is a pure constants file where the values ARE the content. The pre-rewrite output showed `const SEMVER_SPEC_VERSION = '2.0.0'`, `const MAX_LENGTH = 256`, `const MAX_SAFE_COMPONENT_LENGTH = 16`, and the full `RELEASE_TYPES` array with all 7 release type strings. The post-rewrite truncates all of these to just names (`const SEMVER_SPEC_VERSION …`, `const MAX_LENGTH …`, `const RELEASE_TYPES …`). Similarly, `debug.js` (11 lines total) is a single conditional expression — the pre-rewrite showed all 7 lines of the conditional, the post-rewrite shows `const debug …`. In `re.js`, `const LETTERDASHNUMBER …` hides `'[a-zA-Z0-9-]'` and `const safeRegexReplacements …` hides the actual replacement rules. The freed budget isn't even fully used — the new output is 20 lines shorter than the old.

In semver, the missing `ClassBody` group causes all JS class method signatures to be lost. The pre-rewrite output showed `Comparator` with 6 methods (`parse`, `test`, `intersects`, `toString`, etc.), `Range` with 8 methods (`constructor`, `parseRange`, `intersects`, `test`, `format`, `toString`, etc.), and `SemVer` with 7 methods (`compare`, `compareMain`, `comparePre`, `compareBuild`, `inc`, `format`, `toString`). The post-rewrite output truncates all three classes to just `class Comparator …`, `class Range …`, `class SemVer …`. These classes ARE the library — their methods define the complete API surface for version comparison, range parsing, and version manipulation. The freed budget goes instead to CHANGELOG.md headings (see #8) and individual `functions/*.js` one-liner wrappers (20 files showing `const clean …`, `const gt …`, etc.) that merely delegate to these classes.

In tock_internal_core, the missing `InterfaceBody` group causes Go interface method signatures to be lost. `ports/ports.go` defines 3 interfaces (`ActivityResolver`, `ActivityRepository`, `NotesRepository`) that are the entire API contract of this hexagonal architecture core package. The pre-rewrite output showed full interface bodies — all method signatures with parameter types and return types (22 lines). The post-rewrite truncates all three to `type ActivityResolver …`, `type ActivityRepository …`, `type NotesRepository …` (3 lines). Similarly, `errors/errors.go` defines 4 sentinel errors (`ErrActivityNotFound`, `ErrNoActiveActivity`, `ErrActivityAlreadyStarted`, `ErrCancelled`) whose values are lost — the pre-rewrite showed the full `var` block with error messages (7 lines), the post-rewrite shows only `var …` (1 line). The freed budget goes to generated mock files (see #32).

## 10. Markdown h1 body omitted while h2 bodies shown (regression)

**Affected snapshots:** cmdk

ARCHITECTURE.md's introductory text (lines 3-44, directly under `# Architecture`) explains the core design constraint of the library — wanting compound components, rejecting data arrays and render props, "a terrible, terrible constraint that we've spent 2 years fighting." The new output omits this entirely, jumping from `# Architecture` (line 1) to `## Approach` (line 46).

Meanwhile, h2 section bodies are shown in full: `## Example` gets 28 lines of code, `## Performance` and `## Groups` get their body lines. The pre-rewrite output showed the complete file including the intro.

The intro is the most valuable content in ARCHITECTURE.md — it's the "why" that gives meaning to the "how" in the sections below. `HeadingBody` has a flat base value (0.7) regardless of heading level, so the scheduler sees no reason to prefer the h1 intro body over h2 bodies. Since the h1 body is longer (~43 lines vs 1-28 lines for h2 bodies), its cost/benefit ratio is worse, and it loses the budget competition.

## 11. CommonJS entry point rendered empty — require/exports not captured

**Affected snapshots:** commander, semver, semver_classes

Commander's `index.js` (24 lines) is the library's entry point. It shows the module structure: which classes are imported from `lib/`, factory functions (`createCommand`, `createOption`, `createArgument`), and all exports. The pre-rewrite output showed the full file. The new output shows only the filename with zero content — the file appears completely empty.

Semver's root `index.js` (91 lines) is the worst case of this issue. The pre-rewrite output showed the complete file: 44 `require()` imports mapping every function and class to its source file, followed by a `module.exports` object listing all 34 public API names. This is the single most valuable file in the repository — it IS the public API surface. The new output shows it as empty. The budget that should go here instead goes to CHANGELOG.md headings (see #8) and 20 individual `functions/*.js` one-liner wrappers that redundantly list the same function names without the module structure context.

Semver's `classes/index.js` (7 lines) is the same pattern — `module.exports = { SemVer: require('./semver.js'), Range: require('./range.js'), Comparator: require('./comparator.js') }`. The pre-rewrite output showed this in full. The new output shows only the filename. This file is the single best summary of the module: three classes, their names, their source files.

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

**Affected snapshots:** d2ts, d2ts_d2ts, enclosed, enclosed_crypto, enclosed_lib, superstruct, ts_pattern

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

In ts-pattern, `src/index.ts` (6 lines) is the library's entry point defining the entire public API: `export { match }`, `export { isMatching }`, `export { Pattern, Pattern as P }`, `export { NonExhaustiveError }`. It renders completely empty — just the filename with no content. This is the fastest way for a reader to understand what the library exports, and its absence is not compensated by the detailed per-file output (which requires scanning multiple files to reconstruct the API surface).

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

**Affected snapshots:** krep, soluna, sqlite_vec

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

In soluna's `src/mutex.h`, both `#ifdef _MSC_VER` branches are captured — lines 6-9 show `mutex_t SRWLOCK`, `mutex_init(m) InitializeSRWLock(&m)`, etc., and lines 12-15 show `mutex_t pthread_mutex_t`, `mutex_init(m) pthread_mutex_init(&m, NULL)`, etc. 8 lines for 4 unique macros. The old output also showed both branches (with values), so this is pre-existing — but the new output truncates values too (issue #7), making the duplication more wasteful since neither copy is informative.

In sqlite_vec's `sqlite-vec.c`, `PORTABLE_ALIGN32` appears at lines 125 and 166 from different `#ifdef` branches (compiler-specific alignment attributes).

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

## 18. Struct and enum bodies elided — many small entries beat fewer large ones

**Affected snapshots:** log, mdbook, neco, otree, sps, sqlite_vec, thiserror, thiserror_impl_src, tock, toasty_codegen, toasty_core

Large body groups (StructBody at base_value 1.2, EnumBody at 1.5) lose budget to many small FunctionName entries (base_value 1.0, ~2 tokens each). The per-token value/cost ratio strongly favors function names, so the scheduler fills the budget with hundreds of cheap name entries before committing to any multi-line body block.

In log's `src/lib.rs`, the `Level` and `LevelFilter` enums are the core of the library — their variants (Error, Warn, Info, Debug, Trace) are arguably the most important content in the entire crate. The output elides both enum bodies (`pub enum Level …`, `pub enum LevelFilter …`) while spending ~36 lines on individual method signatures for these same types, including low-value methods like `increment_severity`, `decrement_severity`, `from_usize`, and `as_str` — shown for BOTH enums since they have parallel impls.

The pre-rewrite output showed the full `Level` body with all variants and doc comments (lines 475-499, ~24 lines). The new output drops this and also drops the `LevelFilter` body (lines 636-649).

In mdbook, every key struct and enum body is truncated to `…` while the pre-rewrite showed them in full. The losses include:

- `MDBook` struct (root, config, book, renderers, preprocessors) — the central type, 16 lines with field docs
- `Summary` struct (title, prefix_chapters, numbered_chapters, suffix_chapters) — 10 lines defining the book's TOC data model
- `Link` struct (name, location, number, nested_items) — 11 lines, the chapter reference type
- `PreprocessorContext` struct (root, config, renderer, mdbook_version) — 16 lines, the preprocessor API surface
- `RenderContext` struct (version, root, book, config, destination) — 22 lines, the renderer API surface
- `Theme` struct (21 fields for CSS/JS/font assets) — 22 lines
- `BookItem` enum (Chapter, Separator, PartTitle) — 8 lines with variant docs
- `SummaryItem` enum (Link, Separator, PartTitle) — 8 lines with variant docs

These types define mdBook's entire data model and plugin API. A reader seeing `pub struct MDBook …` and `pub struct RenderContext …` learns nothing about what data is available. The pre-rewrite output showed all of these with full field-level documentation — the struct bodies alone were ~113 lines that built a complete mental model of the architecture.

In neco's `neco.h`, the `neco_stats` typedef struct body is truncated from 12 lines to one: `287→typedef struct neco_stats { …`. The pre-rewrite showed all 11 fields with doxygen comments (`coroutines`, `sleepers`, `evwaiters`, `sigwaiters`, `senders`, `receivers`, `locked`, `waitgroupers`, `condwaiters`, `suspended`, `workers`). This struct tells a reader exactly what runtime telemetry is available — it's the only struct in the API whose fields are user-facing. The budget instead goes to showing all ~90 function declarations in the header, many truncated to just names (e.g., `int neco_yield …`).

In otree, `CommandArgs` (~90 lines of clap-derived CLI flags with doc comments) defines the entire user-facing interface — every command-line option from `--content-type` to `--live-reload` to `--wrap`. The pre-rewrite output showed all fields with documentation. The new output shows `pub struct CommandArgs …`. Similarly, `ContentType` enum variants (Json, Yaml, Toml, Xml, Hcl, Jsonl, Any — with doc comments explaining HCL and JSONL) are truncated to `pub enum ContentType …`, and `SyntaxToken` variants (Symbol, Name, Tag, String, Number, Null, Bool, Section, Break) are truncated to `pub enum SyntaxToken …`. The freed budget goes partly to README install instruction body content (~12 lines showing `paru -S otree` and `brew install otree` commands) that the pre-rewrite output correctly omitted. The old output also showed the `docs/actions.md` keybinding table (first few rows of the action/key/description reference) — for a TUI tool, this is core functionality — while the new output shows only the `# All Available Actions` heading.

In sps, the workspace spans 4 crates with several domain-defining types whose bodies are all lost. `SpsError` (15+ variants with `#[error("...")]` messages) tells a reader every failure mode in the system — the pre-rewrite showed all variants. `PipelineEvent` (~20 variants with struct fields) defines the entire event-driven architecture — download lifecycle, job processing, dependency resolution events. `InstalledArtifact` (9 variants: AppBundle, BinaryLink, ManpageLink, MovedResource, PkgUtilReceipt, Launchd, CaskroomLink, CaskroomReference — each with field docs) defines what a "cask install" means at the filesystem level. `JobProcessingState` (8 states from PendingDownload through Succeeded/Failed, with doc comments) defines the job state machine. `BuildEnvironment` struct fields with doc comments explain the build sandbox. `CaskInstallManifest` and `ResolvedDependency` struct fields were also shown in full. The pre-rewrite output showed all of these; collectively they formed the architectural skeleton of the project. The new output collapses every one to just a name.

In thiserror_impl_src, the AST data model is the most important content — `Struct` (attrs, ident, generics, fields), `Enum` (attrs, ident, generics, variants), `Variant` (original, attrs, ident, fields), `Field` (original, attrs, member, ty, contains_generic) define the parsed representation that every other module operates on. Similarly, `Attrs` (display, source, backtrace, from, transparent, fmt) directly maps to the `#[error(...)]`, `#[source]`, `#[from]`, `#[backtrace]` attributes users write. The pre-rewrite output showed all struct bodies with field definitions — collectively ~65 lines building a complete mental model of the derive macro's data flow. The new output truncates all to `pub struct Struct …`, `pub struct Attrs …`, etc. The freed budget goes to 22 private static arrays in `scan_expr.rs` (INIT, POSTFIX, ASYNC, BLOCK, BREAK_LABEL, etc.) — lookup tables for an internal expression parser state machine that convey no API-level understanding. The old output showed `scan_expr.rs` with only its single public function; the new output lists all 22 statics at ~2 tokens each while the struct bodies they displaced were ~4-6 lines each. The old output also showed `lib.rs` with module declarations and the `#[proc_macro_derive(Error, attributes(backtrace, error, from, source))]` attribute — the attribute list is the user-facing API surface — while the new output shows only the function signature.

In thiserror (root), the same impl/ losses apply — ast.rs and attr.rs struct bodies are elided, scan_expr.rs statics shown — since the root fixture includes the impl/ subtree. Additional budget pressure comes from the runtime crate's `src/aserror.rs` and `src/display.rs`, which contain repetitive blanket impls: `AsDynError` is implemented identically for `T: Error`, `dyn Error + 'a`, `dyn Error + Send + 'a`, `dyn Error + Send + Sync + 'a`, and `dyn Error + Send + Sync + UnwindSafe + 'a` (5 impls, all with the same body), plus 5 matching `Sealed` impls. `AsDisplay` has a similar 3-impl pattern. The old output also showed these, so it's not a regression, but at ~20 lines they contribute to the budget pressure that displaces the struct bodies. The old output additionally showed `impl/src/lib.rs` with the full `#[proc_macro_derive(Error, attributes(backtrace, error, from, source))]` attribute and module declarations (`mod ast; mod attr; mod expand; ...`), plus test files demonstrating real `#[derive(Error)]` usage patterns (~80 lines across test_backtrace.rs, test_from.rs, test_generics.rs, etc.). The new output drops all test file content and the module/attribute lines from lib.rs.

Additional budget pressure in log comes from `src/__private_api.rs` (~20 lines), a file whose module doc starts with "WARNING: this is not part of the crate's public API and is subject to change at any time." This file is not flagged by `is_deprioritized_file()` because no rule matches `__`-prefixed source files. The old output also showed this file — it's not a regression, but the 20 lines would be better spent on the missing enum bodies.

In toasty_codegen's `src/expand/filters.rs`, the `Filter` struct (7 fields with doc comments: `fields`, `batch`, `only_relation`, `get_method_ident`, `filter_method_ident`, `filter_method_batch_ident`, `update_method_ident`, `delete_method_ident`) defines the core data structure for the filter code generation system. The pre-rewrite output showed all fields with their doc comments (~24 lines). The new output truncates to `pub(super) struct Filter …`. Meanwhile, the less important private `BuildModelFilters` struct body (2 fields, internal helper) IS shown — an inversion of information value.

In toasty_core, the pre-rewrite showed bodies for `Operation` (8 variants with doc comments: Insert, DeleteByKey, FindPkByIndex, GetByKey, QueryPk, QuerySql, Transaction, UpdateByKey — the complete driver operation set), `Statement` (4 variants: Delete, Insert, Query, Update), `Rows` (Count/Value/Stream — the driver response model), `FieldTy` (Primitive, Embedded, BelongsTo, HasMany, HasOne — the core field type model), `ModelKind` (Root/Embedded), `AutoStrategy`/`UuidVersion`, `IndexScope`, `IndexOp`, and `Migration`. It also showed the `Capability` impl's database-specific constants (SQLITE, POSTGRESQL, MYSQL, DYNAMODB). The new output truncates all of these to just names. For a database ORM core library, these enum bodies define the fundamental abstractions — what operations exist, what field types are supported, what databases work. Budget is instead consumed by 15 error submodule files (see #29).

In tock, `AnalysisStats` in `internal/adapters/cli/analyze.go` (10 fields: TotalDuration, DeepWorkDuration, DeepWorkScore, ContextSwitches, AvgSwitchesPerDay, Chronotype, PeakHour, FocusDistribution, MostProductiveDay, AvgSessionDuration) defines the productivity analysis dimensions — it tells a reader what the `analyze` command measures. The pre-rewrite showed all fields with inline comments (e.g., `DeepWorkScore float64 // 0-100`, `Chronotype string // "Morning Lark", "Night Owl", etc.`). The new output truncates to `type AnalysisStats …`. Meanwhile, other structs in the same output (Config, Activity, DTOs) retain their bodies — the difference is that `internal/adapters/cli/` has 16 files generating ~60 FunctionName entries that outcompete AnalysisStats's body.

In sqlite_vec's `sqlite-vec.c`, the pre-rewrite showed full bodies for `VectorElementType` (3 members: FLOAT32, BIT, INT8), `Vec0TokenType` (6 members), `NpyTokenType` (10 members), `Vec0DistanceMetrics` (3 members: L2, COSINE, L1), and several `typedef enum` blocks with their values. The new output collapses most of these to just names (e.g., `enum VectorElementType …`). For a C project where enums define the API surface (vector element types, distance metrics, query plan types), these bodies are high-value — a reader can't infer the supported element types or distance metrics from the name alone.

## 19. Go doc comments not detected for type declarations (regression)

**Affected snapshots:** mcphost_sdk

The Go tree-sitter query captures `type_spec` and `type_alias` as `@symbol`, but these are inner nodes within `type_declaration`. Doc comments in Go are siblings of `type_declaration`, one AST level above. `compute_doc_start_line` checks `node.prev_named_sibling()`, which for `type_spec` finds nothing — there are no named siblings within `type_declaration` before it.

Two effects:

1. **Doc comments never shown.** `documented` is always `false` for Go type declarations, so `StructDocFirst`, `InterfaceDocFirst`, and `TypeAliasDocFirst` groups are never spawned. In mcphost_sdk, the MCPHost struct doc ("provides programmatic access to mcphost functionality...") and Options struct doc ("configures MCPHost creation with optional overrides...") are both lost. The pre-rewrite output showed all four type doc comments in this fixture.

2. **Type declarations deprioritized.** `documented: false` applies a 0.5× `documented_contribution` penalty, making type declarations compete at half their natural value.

Functions and methods are unaffected — `function_declaration` and `method_declaration` ARE top-level nodes in Go's tree-sitter grammar, so their doc comments are reachable via `prev_named_sibling()`. The asymmetry is visible in mcphost_sdk's output: all 8 method doc comments are shown while all 4 type doc comments are missing.

## 21. Volume-based budget capture — large support package crowds out small core package

**Affected snapshots:** mcphost

`internal/ui/` (24 source files, generic terminal UI rendering) captures ~191 output lines (~30% of the budget). `internal/tools/` (4 source files, core MCP tool management) gets zero output lines. Both directories are at the same depth and classified as Source.

`internal/tools/` contains the project's core domain logic: `MCPToolManager` (the central type managing MCP tools across servers), `MCPConnectionPool` (connection lifecycle and health checking), and the tool mapping/invocation machinery. This is literally what MCPHost is — "a CLI host that enables LLMs to interact with external tools through MCP." A reader of the output would understand how mcphost renders spinner animations and style badges, but not how it connects to or invokes MCP tools.

The `internal/ui/` content is individually reasonable (function names at base_value 1.0, same modifier) but collectively overwhelming. With 24 files generating 100+ FunctionName groups, each cheap (~2 tokens), they win the budget competition through volume. The sublinear scaling `(item_count).powf(0.75)` dampens the advantage at the file-group level (FilesGroup base value), but doesn't limit how many TsGroups are spawned from many files. The tools package's 4 larger files generate fewer groups that individually lose to the UI's many small entries.

This is a pre-existing issue (the old output also omitted `internal/tools/`) but is more damaging after the rewrite because the old output compensated with richer content in the files it did show (struct field bodies, type definitions, doc comments). The new output's broader-but-shallower coverage makes the absence of core domain code more conspicuous.

## 22. Python inner docstrings not detected — classes always marked undocumented

**Affected snapshots:** py3xui, py3xui_api (likely all Python fixtures)

`compute_doc_start_line()` looks for preceding sibling comment nodes above a definition. Python docstrings are inner strings — the first `expression_statement` inside the class/function body — not preceding siblings. This means all Python classes and functions are always `documented: false`, with two effects:

1. **`ClassDocFirst`/`FunctionDocFirst` groups never created.** In py3xui_api, all 7 classes (Api, BaseApi, ApiFields, ClientApi, DatabaseApi, InboundApi, ServerApi) have well-written docstrings explaining their purpose. None are shown. The pre-rewrite output showed multi-line class docstrings for all of them (e.g., Api's "provides a high-level interface to interact with the XUI API," BaseApi's full arguments/attributes listing). The new output truncates all to `class Api …`, `class BaseApi …`, etc.

2. **0.5× undocumented penalty applied.** `documented_contribution(false)` returns 0.5, reducing class and function name priority below what it should be for well-documented Python code.

The combined effect in py3xui_api: ~27 lines of budget go to autogenerated README heading body content (including `<a id="...">` HTML anchor noise lines) — content with 0.1× generated factor that the pre-rewrite correctly suppressed to headings-only. This content fills the space that class docstrings would have occupied if they were detected. The pre-rewrite output showed the README as headings-only and used the saved budget for class docstrings and ApiFields constant definitions (SUCCESS, MSG, OBJ, CLIENT_STATS — see also #9).

## 23. Structurally mirrored packages shown in full — sync/async duplication wastes budget

**Affected snapshots:** py3xui

py3xui provides both synchronous (`py3xui/api/`) and asynchronous (`py3xui/async_api/`) API packages. The two are structural mirrors — identical class hierarchies, identical method names, identical signatures except for `async`/`await`. Both are shown in full: the sync API uses ~48 output lines (6 files), the async API uses ~87 output lines (6 files). Together they consume ~135 lines (~35% of the 4000-token budget) to convey the same API surface twice.

A knowledgeable human would show one package in detail and note the other mirrors it. The second package adds almost zero information — a reader seeing `async def get_by_email …` after already seeing `def get_by_email …` learns only that the async variant exists, which the package name already conveys.

This is a pre-existing issue (the old output also showed both packages fully), but the budget waste is more impactful now because class bodies and docstrings are lost (issues #9, #22), making the remaining content thinner. The ~87 lines spent on the async mirror could instead show Client model fields, Inbound fields, class docstrings, or BaseApi property type annotations — all of which build more understanding than a second listing of the same method names.

No simple heuristic detects structural mirrors in general, but the pattern is common in Python SDKs (sync/async), language bindings (C header + wrapper), and multi-platform code (platform-specific implementations with identical APIs).


## 25. Bare-filename repetitive files waste budget while structural files are empty

**Affected snapshots:** sps_core, sqlite_vec, swarm, vscode_emojis_small, vscode_emojis_medium, xlstm_blocks

In `src/install/cask/artifacts/`, 21 of 24 `.rs` files are shown as bare filenames (no content). These files follow a uniform pattern — each contains a single `pub fn install_X` function — so once the pattern is clear from 2-3 examples, additional bare filenames add no understanding. Collectively they consume ~42 tokens for information already implied by the directory structure.

Meanwhile, structural files that orient the reader on crate organization are shown empty:

- `src/lib.rs` (crate root, 20 lines) — declares all top-level modules (`pub mod build/check/install/pipeline/uninstall/upgrade/utils`) and re-exports `UninstallOptions`. Shown with no content.
- 6 `mod.rs` files (`build`, `check`, `pipeline`, `uninstall`, `upgrade`, `utils`) — each contains `pub mod` declarations. All shown as bare filenames.

The old (pre-rewrite) output showed mod.rs files with their declarations (e.g., `build/mod.rs` → `pub mod compile; pub mod env;`, `check/mod.rs` → `pub mod installed; pub mod update;`, `install/cask/artifacts/mod.rs` → all 23 `pub mod` declarations). It also showed each artifact file's `pub fn install_X` name. The old approach was more informative per token: mod.rs declarations revealed the crate's internal structure, and artifact function names at least confirmed the pattern.

In sqlite_vec, the problem is even more extreme: ~25 empty file entries (Makefile, sqlite-vec.h.tmpl, test.sql, SECURITY.md, various examples/, scripts/, and site/ files) and ~25 empty folder entries (benchmarks/exhaustive-memory/, benchmarks/micro/, tests/afbd/, tests/correctness/, etc.). That's ~50 empty entries consuming ~100 tokens for near-zero information. The old output was more selective — it collapsed `site/` into a single folder entry rather than listing all its subfiles and subfolders individually. The budget spent on these empty entries could instead show README body content (issue #6) or enum bodies (issue #18).

In swarm, 31 `logs/session_*.json` files are shown individually as bare filenames, consuming ~62 tokens. The old output showed `logs/` as a single folder entry. These session logs follow a uniform naming pattern — once you've seen one filename, the rest add nothing. The budget could instead show example source files (see #27).

In vscode_emojis_small, 9 SVG files in `icons/light/` are listed individually plus `icons/dark/` as a folder — but dark's files are omitted, creating an asymmetric presentation that implies the two directories differ when they're identical. The old output showed just `icons/` as a single folder entry. At budget 100, these bare SVG entries consume most of the budget while `emojis.json` renders with no content (see #34).

In vscode_emojis_medium, 18 SVG files are listed individually (`icons/light/status-added.svg` through `icons/dark/status-untracked.svg`) — 9 files in `light/` and 9 identical names in `dark/`. The old output showed a single `icons/` folder entry. The SVG filenames follow a uniform `status-*.svg` pattern across two theme variants; once you've seen `icons/light/` and `icons/dark/`, individual filenames add nothing. At budget 200, these 18 bare entries consume ~36 tokens (~18% of budget) while `emojis.json` (the most important file — a 40KB emoji mapping) renders completely empty (see #34).

In xlstm_blocks, 6 `.cu`/`.cuh` files in `slstm/src/cuda/` (56-424 lines each) render as bare filenames because precis has no CUDA parser. The old output omitted these entirely. These are kernel implementations whose interfaces are already conveyed by the parsed `slstm.h` (which shows `ForwardPass`, `BackwardPass`, `BackwardPassCut` classes) and `slstm.cc` (which shows the pybind module). The bare filenames add only "these CUDA files exist" — already implied by the header.

The root cause is that bare-filename entries have a non-zero rendering cost (~2 tokens each) but zero information value beyond "this file exists." In a directory with 24 similar files, the directory name itself conveys more than 21 individual bare filenames. Budget would be better spent showing lib.rs module declarations (~15 tokens) and mod.rs structures (~5-10 tokens each), which tell the reader how the crate is organized.

## 26. Docs site pages shown individually when table-of-contents file already provides structure

**Affected snapshots:** superstruct

Superstruct has a `docs/summary.md` that lists every guide and reference page with links — it IS the table of contents. The output shows this TOC (headings: Guides, API Reference, Resources), but then also shows 13 individual docs/ files (6 guides, 7 reference pages) each with only their `# Title` heading. These 13 single-heading entries consume ~70 lines of output (file headers + title lines) to convey information already present in `docs/summary.md`.

For example, `docs/reference/coercions.md` shows `# Coercions` plus a one-line description, while `docs/summary.md` already lists `- [Coercions](./reference/coercions.md)`. The individual page adds the description text, but the structural relationship is already clear from the TOC.

The ~70 lines of redundant docs/ content displace higher-value source code. The pre-rewrite output showed these same docs pages but compensated by also showing `src/error.ts` with full `Failure` type body (8 fields) and `StructError` class body (8 fields + constructor), plus `src/index.ts` with all 6 re-exports. The new output loses all of these (see #9 and #13) — the Failure type fields (value, key, type, refinement, message, explanation, branch, path) and StructError fields are the error API surface, and the re-exports define the public API.

The DocsSite category factor (0.2×) should suppress this, but with 13 files each generating at least an h1 Heading group (base_value 1.0), the aggregate value still captures significant budget. When a summary/TOC file exists in a docs directory, individual pages' headings are almost entirely redundant.

## 27. Example source files absent while library README dominates budget (regression)

**Affected snapshots:** swarm

Swarm is an educational framework — the library itself is tiny (4 files, ~300 lines), and the examples ARE the core content. The old output showed function signatures from all 10 example directories: airline agent configs and tools, basic examples (handoff, context_variables, function_calling), personal_shopper database functions, support_bot query/email functions, triage_agent routing functions, weather_agent functions, and customer_service_streaming's full engine/task architecture. The new output shows only example README headings and folder entries, with 3 weather_agent `.py` files rendered completely empty (no content despite having functions like `get_weather`, `send_email`, and an Agent instantiation).

Meanwhile, the README consumes ~173 lines of output with full code examples, the "Core Contributors" list (6 names), install instructions, and documentation tables — content that's less information-dense per token than the example source signatures it displaces. The old output also showed the full README but compensated by showing example source code.

The `FileCategory::Example` factor (0.35×) is appropriate for most projects but harmful here. The regression is compounded by #25 (31 bare-filename log files wasting ~62 tokens that could fund ~30 example function signatures).

## 28. Python class bodies never shown — no ClassBody in taxonomy

**Affected snapshots:** swarm, typeguard, xlstm (likely all Python fixtures with dataclasses/Pydantic models)

`TsGroupKey` has `StructBody` (base_value 1.2) and `EnumBody` (1.5) but no `ClassBody`. Python class field definitions live in the class body — for Pydantic models and dataclasses, the fields ARE the class API. In swarm's `types.py`, the old output showed:

```
class Agent(BaseModel):
    name: str = "Agent"
    model: str = "gpt-4o"
    instructions: Union[str, Callable[[], str]] = "You are a helpful agent."
    functions: List[AgentFunction] = []
    tool_choice: str = None
    parallel_tool_calls: bool = True

class Response(BaseModel):
    messages: List = []
    agent: Optional[Agent] = None
    context_variables: dict = {}
```

The new output shows only `class Agent …`, `class Response …`, `class Result …`. For an agent framework, seeing Agent's 6 fields (name, model, instructions, functions, tool_choice, parallel_tool_calls) is essential to understanding the API — it's the equivalent of a Rust struct's field definitions, which get StructBody at 1.2.

This also interacts with #22 (Python docstrings not detected): classes are both undocumented (0.5× penalty) and bodyless, so the reader gets only a class name with no fields, no docstring, and no type information.

In typeguard, `_config.py` contains three classes whose bodies are the core configuration API:
- `ForwardRefPolicy(Enum)` — 3 values: ERROR, WARN, IGNORE
- `CollectionCheckStrategy(Enum)` — 2 values: FIRST_ITEM, ALL_ITEMS
- `TypeCheckConfiguration` dataclass — 4 fields with defaults: forward_ref_policy, typecheck_fail_callback, collection_check_strategy, debug_instrumentation

The pre-rewrite showed all enum values and dataclass fields with defaults (~25 lines). The new output shows only `class ForwardRefPolicy …`, `class CollectionCheckStrategy …`, `class TypeCheckConfiguration …`. For a config module, the options and their defaults ARE the API — a reader seeing just class names doesn't know what policies exist or what can be configured.

Similarly, `_transformer.py`'s `TransformMemo` dataclass (15 fields including node, parent, path, return_annotation, yield_annotation, send_annotation, is_async, local_names, etc.) was shown in full in the old output (~24 lines) but is collapsed to `class TransformMemo …` in the new output. `AnnotationTransformer.type_substitutions` (dict mapping builtins to typing equivalents) was also shown in the old output.

In xlstm, the ML config dataclasses define the architecture's hyperparameter space — the fields ARE what a reader needs to understand the model configuration. The pre-rewrite showed `mLSTMLayerConfig` with all 12 fields (`conv1d_kernel_size: int = 4`, `qkv_proj_blocksize: int = 4`, `num_heads: int = 4`, `proj_factor: float = 2.0`, `embedding_dim`, `bias`, `dropout`, `context_length`, etc.), `mLSTMBlockConfig` with its 3 fields, and `mLSTMCellConfig` with its 3 fields. The new output collapses all of these to `class X …`. Similarly, `xLSTMBlockStackConfig`, `xLSTMLargeConfig`, `sLSTMCellConfig`, and other config classes throughout the library lose their field definitions. For an ML library, the config fields tell a reader what the model's architectural knobs are — without them, the reader knows a config class exists but not what it configures.

## 29. Repetitive error submodule files displace higher-value content

**Affected snapshots:** toasty_core

In toasty_core, 15 error submodule files (`src/error/adhoc.rs` through `src/error/validation.rs`) each follow an identical pattern: a `pub(super)` struct, an `impl Error` block with a public constructor (`pub fn error_name(...) -> Error`) and a public predicate (`pub fn is_error_name(&self) -> bool`). Each file contributes ~6 content lines plus a ~10-token file header, totaling ~350 tokens across all 15 files. After seeing 2-3 examples, every subsequent file is entirely predictable.

The pre-rewrite output showed none of these submodules — only `src/error.rs` with the `Error` struct body, `ErrorKind` enum, and `IntoError` trait. The freed budget went to ~30 files from `src/stmt/` showing the SQL AST type names (`Expr`, `Value`, `Type`, `Direction`, `BinaryOp`, `SetOp`, `Source`, `Query`, `Lock`, `Filter`, `Returning`, etc.) plus `src/schema/db/` types (`Column`, `Index`, `Migration`, `Table`). For a database ORM core library, the statement AST and database schema types are far more informative than individual error constructors.

The root cause is that each error submodule generates several group entries (StructName, ImplBlock, FunctionName × 2) that individually score well enough to beat the marginal cost of their file header. The `pub(super)` struct gets a 0.3× visibility penalty, but the `pub fn` methods on `impl Error` are fully public. The aggregate effect is that 15 small files with mechanical content outbid the stmt/ directory's content despite being less informative per token.

## 30. Rust lib.rs with `mod` + `pub use` re-exports rendered empty (regression)

**Affected snapshots:** toasty

`crates/toasty/src/lib.rs` is the main ORM crate's entry point. Its 60 lines define the entire public API surface via `mod` declarations and `pub use` re-exports:

```rust
mod apply_update;
pub use apply_update::{ApplyUpdate, Query};
pub mod cursor;
pub use cursor::Cursor;
pub mod db;
pub use db::Db;
pub mod relation;
pub use relation::{BelongsTo, HasMany, HasOne};
pub mod stmt;
pub use stmt::Statement;
pub use toasty_core::{Error, Result};
```

The pre-rewrite output showed 7 lines: `pub mod cursor`, `pub mod db`, `pub mod relation`, `pub mod schema`, `pub mod stmt`, and `pub mod driver { pub use toasty_core::driver::* }`. A reader immediately understood the module structure and what the crate re-exports.

The new output renders the file completely empty — just the filename with zero content lines. The `mod` declarations and `pub use` re-exports are likely captured as Import groups with base_value 0.1, and re-exports further penalized by `reexport_contribution()` (0.1×). The combined effective value (~0.01-0.1 per item) is too low to justify the file header cost. But these lines ARE the API surface — they tell a reader what types are public, where they come from, and how the crate is organized. They're more valuable than many FunctionName entries that the budget is spent on instead.

## 31. Docs directory heading-only content displaces core source code

**Affected snapshots:** toasty

The new output shows ~92 lines of docs/ content across 10 files (ARCHITECTURE.md, CHANGE_GUIDE.md, CONTEXT.md, architecture/*.md, guide/*.md, design/*.md, roadmap/README.md). Most show only heading structures with no body content. While docs/ARCHITECTURE.md (project structure + crate overview) and docs/architecture/*.md (query engine phases, type system) are genuinely high-value, the remaining files contribute heading-only outlines of limited value:

- `docs/CHANGE_GUIDE.md` (13 heading lines) — development change guide; headings like "## Crate-Specific Patterns", "## Common Pitfalls" convey nothing without body text
- `docs/guide/pagination.md` (10 heading lines) — user guide headings only
- `docs/guide/jiff.md` (8 heading lines) — user guide headings only
- `docs/design/enums-and-embedded-structs.md` (14 lines) — design doc headings
- `docs/design/pagination.md` (8 heading lines) — design doc headings

These ~53 lines of low-value heading-only docs displace content the pre-rewrite showed: `BelongsTo<T>`, `HasMany<T>`, `HasOne<T>` relation structs with `get()` methods (the core ORM relationship types — 9 lines), `MigrationPrefixStyle` enum body (Sequential/Timestamp — 7 lines), `AutoStrategy`/`UuidVersion`/`ColumnType` enum bodies from codegen (see #18), and `Capability` database-specific constants from toasty-core (see #18).

The pre-rewrite had zero docs/ content for these files — it showed only the crate-level CONTEXT.md files. The docs/ directory is classified as `DocsSite` (category_factor 0.2), but at depth 1-2 the depth_factor is 1.0-0.7, yielding an effective contribution of 0.14-0.2. Since docs files tend to have high heading counts (each at base_value 0.6-1.0), even with the category penalty, the aggregate value of many headings across many docs files exceeds the value of a few struct bodies in deeper source directories.

## 32. Generated mock files consume budget despite test + generated suppression

**Affected snapshots:** tock_internal_core

The 3 auto-generated mock files in `ports/mocks/` consume ~82 lines (~70% of content) while the 4 core domain files get ~34 lines (~30%). The mock files should be heavily suppressed: the `mocks/` directory classifies as `FileCategory::Test` (0.15× factor), `is_generated_filename` matches `mock_*.go`, and `is_generated_file` matches "Code generated by mockery; DO NOT EDIT." in the first line. If both `folders_contribution` (0.15) and `generated_contribution` (0.1) applied multiplicatively, the effective modifier would be ~0.015× — mock content should be nearly invisible.

Yet the output shows all 3 mock files with struct bodies (`mock.Mock`), constructor functions, individual method entries, `_Call` struct types, and `_Expecter` struct types — the full mockery boilerplate scaffold. `MockActivityResolver` alone gets 37 lines showing 6 methods × (method + Call type + Expecter method) pattern. This is auto-generated code that conveys zero information beyond "these interfaces have mocks."

The pre-rewrite output showed the same ~82 lines of mock content, so the mock visibility is not a regression. But combined with the rewrite's regression on core content (interfaces collapsed to names per #9, errors collapsed to `var …`), the ratio has inverted: the old output was 114 core + 82 mock (58/42%), the new output is 34 core + 82 mock (29/71%). A reader of the new output learns more about mock boilerplate structure than about the actual domain model.

Either the test + generated factors are not stacking multiplicatively, or one of the detection mechanisms is not triggering for these files. The expected behavior is that generated test mock files should be suppressed to at most a bare filename mention.

## 33. Python `__init__.py` re-exports shown in full — redundant with per-module listings

**Affected snapshots:** typeguard

In typeguard, `__init__.py` contains 23 re-export lines (`from ._checkers import TypeCheckerCallable as TypeCheckerCallable`, etc.) that define the package's public API. The new output shows all 23 lines (~350 tokens), consuming ~9% of the 4000-token budget. The pre-rewrite showed only the non-import symbols (`config: TypeCheckConfiguration` and `def __getattr__`).

The re-exports are valuable in isolation — they tell a reader what `import typeguard` provides. But they're almost entirely redundant with the per-module listings already shown: every re-exported symbol (`check_type`, `typechecked`, `TypeCheckError`, etc.) appears in its source file's output. The 350 tokens would be far better spent on the empty README.rst (see #6) or the missing config enum values (see #28).

The `from X import Y as Y` pattern is Python's explicit re-export convention. If precis detects this as `ImportedItems { first_party: true }` (base_value 1.0) without applying the reexport penalty, that explains the over-allocation. With `reexport_contribution()` (0.1×), the effective value should be low enough to suppress most of these. This is the inverse of #30 (Rust re-exports too aggressively suppressed) — Python re-exports not suppressed enough.

## 34. Single-line JSON file renders empty — 1,837 DataSection groups produce no output (regression)

**Affected snapshots:** vscode_emojis_small, vscode_emojis_medium

`emojis.json` is a 40KB single-line JSON file with 1,837 key-value pairs (emoji name → emoji character). The pre-rewrite output showed a truncated first line: `{"100":"💯","1234":"🔢","+1":"👍","-1":"👎",...} …` — immediately telling the reader this is an emoji name→character mapping. The new output shows just the bare filename with zero content.

The JSON query captures each top-level pair as a DataSection (base_value 0.8). With 1,837 entries all on line 1, the scheduler creates many groups but they all reference the same source line. The rendered output should show at least the truncated line 1, but nothing appears. Meanwhile, 18 bare SVG filenames consume ~36 tokens of the 200-token budget (see #25).

This is the most important file in the fixture — a reader seeing only `emojis.json` with no content doesn't know it's an emoji mapping, how many entries it has, or what its structure looks like.

## 35. Third-party imports dropped in small single-file projects despite ample budget (regression)

**Affected snapshots:** xxhash_xxhsum

`xxhsum/xxhsum.go` is a 50-line single-file project with a 2000-token budget. The pre-rewrite output (470 tokens) showed the import block including `github.com/cespare/xxhash/v2` — the core dependency that tells a reader this is a wrapper around the xxhash library. The new output drops the entire import block (lines 3-9), showing only the three functions (lines 11-50).

Root cause: `Import { first_party: false, .. }` has base_value 0.0 in heuristics.rs. The comment says "3rd party imports only via dependent_siblings" — but in a single-file project there are no siblings, so the import can never be surfaced. The budget is vastly underutilized (the old output used ~24% of budget) yet the scheduler cannot select the import because its value is zero regardless of remaining capacity.
