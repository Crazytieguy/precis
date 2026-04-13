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


## 9. Type alias and const bodies missing from taxonomy (regression) [needs human review]

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

**Attempted fix:** Tried three approaches: (1) Changing TypeAliasName/ConstName rendering to show full first line instead of truncating at name — individually correct but cascades to 54 snapshots due to changed token costs; ts_pattern regressed (lost API methods, gained README h3 headings); (2) Adding TypeAliasBody/ConstBody groups with base_value 0.3/0.2 — near-zero marginal cost for single-line upgrades makes them competitive everywhere, still 42 snapshots affected; (3) Adding all four body groups (TypeAlias, Const, Interface, Trait) — 75 failures. The fundamental tension: showing type/const values costs more tokens per entry, which shifts budget allocation globally. Any approach that shows more content will cascade. May need a mechanism that allows body content without displacing other entries (e.g., a "free upgrade" path for same-line Truncated→Complete transitions that doesn't count against budget).

## 13. Functions exported via `export { name }` treated as private

**Affected snapshots:** enclosed

Many enclosed files use the declare-then-export pattern:

```typescript
export { createNoteRepository };
function createNoteRepository({ storage }: { storage: Storage }) { ... }
```

The function declaration IS captured (it appears as a `FunctionName` group), but since the `export` keyword is on the `export_statement` rather than on the declaration, the function receives the private visibility modifier (0.3×). Combined with depth modifiers at level 3-4 (0.7-0.4×), effective values drop to ~0.12-0.21. This causes the server's core domain files to render as empty headers despite substantial content:

- `notes.repository.ts` (123 lines, 6 functions including CRUD operations) — empty
- `notes.routes.ts` (139 lines, REST API endpoints with Zod validation) — empty
- `notes.usecases.ts` (32 lines, core business logic) — empty
- `notes.models.ts` (4 functions for note expiration/formatting) — empty

Fixing this requires cross-referencing `export { name }` statements with declarations in the same file to detect that the function is public despite lacking an `export` keyword on its declaration.

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

## 21. Volume-based budget capture — large support package crowds out small core package

**Affected snapshots:** mcphost

`internal/ui/` (24 source files, generic terminal UI rendering) captures ~191 output lines (~30% of the budget). `internal/tools/` (4 source files, core MCP tool management) gets zero output lines. Both directories are at the same depth and classified as Source.

`internal/tools/` contains the project's core domain logic: `MCPToolManager` (the central type managing MCP tools across servers), `MCPConnectionPool` (connection lifecycle and health checking), and the tool mapping/invocation machinery. This is literally what MCPHost is — "a CLI host that enables LLMs to interact with external tools through MCP." A reader of the output would understand how mcphost renders spinner animations and style badges, but not how it connects to or invokes MCP tools.

The `internal/ui/` content is individually reasonable (function names at base_value 1.0, same modifier) but collectively overwhelming. With 24 files generating 100+ FunctionName groups, each cheap (~2 tokens), they win the budget competition through volume. The sublinear scaling `(item_count).powf(0.75)` dampens the advantage at the file-group level (FilesGroup base value), but doesn't limit how many TsGroups are spawned from many files. The tools package's 4 larger files generate fewer groups that individually lose to the UI's many small entries.

This is a pre-existing issue (the old output also omitted `internal/tools/`) but is more damaging after the rewrite because the old output compensated with richer content in the files it did show (struct field bodies, type definitions, doc comments). The new output's broader-but-shallower coverage makes the absence of core domain code more conspicuous.

## 23. Structurally mirrored packages shown in full — sync/async duplication wastes budget

**Affected snapshots:** py3xui

py3xui provides both synchronous (`py3xui/api/`) and asynchronous (`py3xui/async_api/`) API packages. The two are structural mirrors — identical class hierarchies, identical method names, identical signatures except for `async`/`await`. Both are shown in full: the sync API uses ~48 output lines (6 files), the async API uses ~87 output lines (6 files). Together they consume ~135 lines (~35% of the 4000-token budget) to convey the same API surface twice.

A knowledgeable human would show one package in detail and note the other mirrors it. The second package adds almost zero information — a reader seeing `async def get_by_email …` after already seeing `def get_by_email …` learns only that the async variant exists, which the package name already conveys.

This is a pre-existing issue (the old output also showed both packages fully), but the budget waste is more impactful now because class bodies and docstrings are lost (issues #9, #22), making the remaining content thinner. The ~87 lines spent on the async mirror could instead show Client model fields, Inbound fields, class docstrings, or BaseApi property type annotations — all of which build more understanding than a second listing of the same method names.

No simple heuristic detects structural mirrors in general, but the pattern is common in Python SDKs (sync/async), language bindings (C header + wrapper), and multi-platform code (platform-specific implementations with identical APIs).


## 25. Bare-filename repetitive files waste budget while structural files are empty

**Affected snapshots:** sps_core, sqlite_vec, swarm, vscode_emojis_small, vscode_emojis_medium, xlstm_blocks

In `src/install/cask/artifacts/`, 21 of 24 `.rs` files are shown as bare filenames (no content). These files follow a uniform pattern — each contains a single `pub fn install_X` function — so once the pattern is clear from 2-3 examples, additional bare filenames add no understanding. Collectively they consume ~42 tokens for information already implied by the directory structure.

Meanwhile, structural files that orient the reader on crate organization were shown empty (partially fixed: `mod_item` declarations are now captured as Import items, so lib.rs and mod.rs files show their `pub mod` declarations in sps_core):

- `src/lib.rs` (crate root, 20 lines) — now shows all top-level `pub mod` declarations ✓
- 6 `mod.rs` files (`build`, `check`, `pipeline`, `uninstall`, `upgrade`, `utils`) — now show `pub mod` declarations ✓

The remaining issue is that 21 bare-filename artifact files still consume ~42 tokens.

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

## 29. Repetitive error submodule files displace higher-value content

**Affected snapshots:** toasty_core

In toasty_core, 15 error submodule files (`src/error/adhoc.rs` through `src/error/validation.rs`) each follow an identical pattern: a `pub(super)` struct, an `impl Error` block with a public constructor (`pub fn error_name(...) -> Error`) and a public predicate (`pub fn is_error_name(&self) -> bool`). Each file contributes ~6 content lines plus a ~10-token file header, totaling ~350 tokens across all 15 files. After seeing 2-3 examples, every subsequent file is entirely predictable.

The pre-rewrite output showed none of these submodules — only `src/error.rs` with the `Error` struct body, `ErrorKind` enum, and `IntoError` trait. The freed budget went to ~30 files from `src/stmt/` showing the SQL AST type names (`Expr`, `Value`, `Type`, `Direction`, `BinaryOp`, `SetOp`, `Source`, `Query`, `Lock`, `Filter`, `Returning`, etc.) plus `src/schema/db/` types (`Column`, `Index`, `Migration`, `Table`). For a database ORM core library, the statement AST and database schema types are far more informative than individual error constructors.

The root cause is that each error submodule generates several group entries (StructName, ImplBlock, FunctionName × 2) that individually score well enough to beat the marginal cost of their file header. The `pub(super)` struct gets a 0.3× visibility penalty, but the `pub fn` methods on `impl Error` are fully public. The aggregate effect is that 15 small files with mechanical content outbid the stmt/ directory's content despite being less informative per token.

## 30. Rust lib.rs with `pub use` re-exports rendered empty in large workspaces [needs human review]

**Affected snapshots:** toasty

`crates/toasty/src/lib.rs` is the main ORM crate's entry point. Its 60 lines define the entire public API surface via `mod` declarations and `pub use` re-exports.

**Partial fix applied:** `mod_item` declarations (e.g. `pub mod cursor;`) are now captured as Import items, and `pub use` re-exports in lib.rs are no longer penalized by `reexport_contribution()`. This fixed module structure visibility in smaller Rust crates (sps_core, thiserror, toasty_codegen), but toasty's lib.rs remains empty because Import base_value (0.1) and ImportedItems base_value (1.0) can't compete on per-token ratio against FunctionName entries (~2 tokens each) in an 8000-token workspace with 8 crates. The `pub use` lines are ~3-4 tokens each, giving them a ratio of ~0.14 vs ~0.28 for function names. Fixing this likely requires either (a) a higher base_value for first-party ImportedItems, which has broad effects, or (b) a mechanism that boosts lib.rs content specifically, which requires threading file identity through the group system.

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

## 35. Third-party imports dropped in small single-file projects despite ample budget (regression) [needs human review]

**Affected snapshots:** xxhash_xxhsum

`xxhsum/xxhsum.go` is a 50-line single-file project with a 2000-token budget. The pre-rewrite output (470 tokens) showed the import block including `github.com/cespare/xxhash/v2` — the core dependency that tells a reader this is a wrapper around the xxhash library. The new output drops the entire import block (lines 3-9), showing only the three functions (lines 11-50).

Root cause: `Import { first_party: false, .. }` has base_value 0.0 in heuristics.rs. The comment says "3rd party imports only via dependent_siblings" — but in a single-file project there are no siblings, so the import can never be surfaced. The budget is vastly underutilized (the old output used ~24% of budget) yet the scheduler cannot select the import because its value is zero regardless of remaining capacity.

**Attempted fix:** Tried three approaches: (1) giving third-party imports base_value 0.05 — caused 38 snapshot regressions across all languages; (2) giving base_value 0.01 — still 23 regressions; (3) promoting ungated third-party imports to first-party in files.rs — 29 regressions. Import groups are so cheap (few tokens) that any non-zero base value makes them competitive everywhere, displacing function bodies and other higher-value content. Also tried adding Go first-party detection (stdlib imports don't contain dots), but this correctly classifies Go stdlib imports and the xxhsum import block becomes first-party, which fixes this specific case but adds import blocks to every Go file in every Go snapshot. A targeted fix may need scheduler-level awareness of budget utilization rate or a mechanism specific to single-file projects.
