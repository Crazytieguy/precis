# Output quality issues


## 6. README body content dropped — only headings shown (partially fixed)

**Partially fixed:** A 2× modifier boost for README h1 HeadingBody groups combined with a 12-line cap on markdown h1 body rendering fixed 9 of the 16 affected snapshots: commander, d2ts, d2ts_d2ts, mdbook, mdbook_guide_src, soluna, sps, sqlite_vec, toasty. These now show introductory README body content (project descriptions, status warnings, feature lists).

**Remaining affected snapshots:** bareiron, log, mcphost, pluggy, semver, typeguard, vaul

The fix doesn't help these because: (a) their h1 body content is too short for the cap to reduce cost meaningfully (bareiron, log), (b) they use RST instead of markdown with no tree-sitter parser (pluggy, typeguard), (c) the body content still can't compete on ratio in larger workspaces (mcphost, semver), or (d) the README has no headings so no HeadingBody groups exist (vaul).

In typeguard, the README.rst (49 lines) explains the library's purpose (runtime type checking for PEP 484 annotations), the two principal approaches (check_type function vs code instrumentation), and the two instrumentation options (@typechecked vs import hook). The pre-rewrite showed the entire file. The new output shows just the bare filename with zero content — even worse than heading-only, because RST has no tree-sitter parser. Additionally, 7 docs/*.rst files all render as bare filenames for the same reason.

In pluggy, the README.rst contains a complete working example (69 lines in the pre-rewrite output) demonstrating the entire hook specification and implementation API. This is the single best introduction to what pluggy is and how to use it. The new output shows zero README content.

In bareiron, the full introductory section (lines 1-11) was shown: project description ("Minimalist Minecraft server for memory-restrictive embedded systems"), design priorities, Minecraft/protocol version numbers, and a compatibility warning. The h1 body is ~8 lines — small enough that the 12-line cap doesn't reduce its cost, so the 2× boost alone isn't sufficient for it to win the budget competition.

In vaul, the README is a 3-line deprecation notice (blockquote, no headings): "This repo is unmaintained. I might come back to it at some point, but not in the near future." This is a headingless markdown file — with no `Heading` groups, there are no `HeadingBody` groups to carry the text.


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

## 14. Server architecture lost to broad-but-shallow budget distribution (regression)

**Affected snapshots:** enclosed

The pre-rewrite output showed ~170 lines of `packages/app-server/` content: auth middleware (`authenticationMiddleware`, `protectedRouteMiddleware`), config definition, 6 middleware files (cors, errors, logger, storage, timeout, config), 3 storage factories (cloudflare-kv, fs-lite, memory), notes domain types with full bodies, notes tasks, shared errors, and validation utilities. A reader could understand: Hono middleware stack → auth flow → storage abstraction → notes CRUD → task scheduling.

The new output shows ~50 lines of server content: entry points, function/type names for `server.ts`/`server.types.ts`, and constant/type names from the notes domain. The middleware layer, auth system, and storage factories are completely absent — their directories appear only as folder entries (`auth/`, `config/`, `middlewares/`, `storage/factories/`). The notes domain files are present as headers but render empty (see #13 effect 2).

Two contributing causes:

1. **Depth penalty on deep monorepo structures.** The middleware files at `packages/app-server/src/modules/app/middlewares/` have effective_depth 4 (after `packages` and `src` are normalized), yielding depth_factor 0.4. Their public functions get effective value 0.4 — enough in isolation, but uncompetitive against the volume of shallower content across 7 packages.

2. **Budget redistribution without architectural weighting.** The old output over-allocated to `packages/app-client/` (~230 lines, mostly shadcn-solid UI components). The new output correctly reduced that, but the freed budget spread evenly across all packages (more crypto, CLI, and lib internals) rather than flowing to the server. The result: broader coverage with no single package covered deeply enough to convey its architecture. The server — which defines the entire REST API, storage abstraction, and auth flow — is the biggest casualty.


## 17. C header file budget reduced — key API declarations and struct bodies lost (partially fixed)

**Partially fixed:** Added a 0.3× companion-header penalty for C/C++ implementation files (.c/.cpp/.cxx/.cc) when header files exist in the same directory. krep.c dropped from ~54 output lines to ~6 (just 2 #define constants and 2 internal typedefs). The freed budget expanded krep.h's @brief doc comments from truncated `→…` to full @param/@return annotations. Also improved neco (gained @defgroup API organization markers) and soluna (gained struct field definitions in headers, lost ~100 lines of empty .c bare filenames).

**Remaining:** krep.h still shows only ~15 of 30 function declarations (missing search_file, search_string, boyer_moore_search, kmp_search, regex_search, SIMD variants, thread_pool_submit/wait_all, match_result_add/free/merge). The freed budget went to expanded doc comments rather than additional declarations. Struct bodies (search_params_t, thread_data_t, match_position_t) are still truncated — this is covered by issue #18.

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

**Affected snapshots:** sqlite_vec

**Partially fixed:** sps_core's `src/install/cask/artifacts/` now shows as a single folder entry (not 21 individual bare filenames). The `mod_item` declarations fix also gave lib.rs and mod.rs files their `pub mod` declarations. swarm's `logs/` directory is now classified as `FileCategory::Artifact`, collapsing 31 bare filenames into a single folder entry. SVG files are now classified as binary. xlstm_blocks' 8 `.cu`/`.cuh` files in `slstm/src/util/` are now deprioritized by the companion-header penalty (`.cu` added to `is_c_implementation_extension`), freeing ~16 tokens that now show full method signatures in `sLSTMCellBase`.

**Remaining:** In sqlite_vec, ~25 empty file entries (Makefile, sqlite-vec.h.tmpl, test.sql, various examples/, scripts/, and site/ files) and ~25 empty folder entries (benchmarks/, tests/ subdirectories, etc.) consume ~100 tokens for near-zero information. The old output was more selective — it collapsed `site/` into a single folder entry rather than listing all its subfiles and subfolders individually. The budget spent on these empty entries could instead show README body content (issue #6) or enum bodies (issue #18). A general mechanism for deprioritizing unparseable bare-filename entries was attempted (0.2× weight for files without a tree-sitter language) but caused 23+ snapshot regressions due to cascading budget redistribution — most directories mix parseable and unparseable files.

## 27. Example source files absent while library README dominates budget (regression)

**Affected snapshots:** swarm

Swarm is an educational framework — the library itself is tiny (4 files, ~300 lines), and the examples ARE the core content. The old output showed function signatures from all 10 example directories: airline agent configs and tools, basic examples (handoff, context_variables, function_calling), personal_shopper database functions, support_bot query/email functions, triage_agent routing functions, weather_agent functions, and customer_service_streaming's full engine/task architecture. The new output shows only example README headings and folder entries, with 3 weather_agent `.py` files rendered completely empty (no content despite having functions like `get_weather`, `send_email`, and an Agent instantiation).

Meanwhile, the README consumes ~173 lines of output with full code examples, the "Core Contributors" list (6 names), install instructions, and documentation tables — content that's less information-dense per token than the example source signatures it displaces. The old output also showed the full README but compensated by showing example source code.

The `FileCategory::Example` factor (0.35×) is appropriate for most projects but harmful here. **Partially fixed:** the log file budget waste from #25 was addressed by classifying `logs/` as `FileCategory::Artifact` — the freed ~62 tokens now show function docstrings and signatures from 6 example directories (support_bot, personal_shopper, triage_agent). The remaining regression: 3 weather_agent files still render empty, README still dominates budget (~173 lines), and customer_service_streaming example content is still absent.


## 30. Rust lib.rs with `pub use` re-exports rendered empty in large workspaces [needs human review]

**Affected snapshots:** toasty

`crates/toasty/src/lib.rs` is the main ORM crate's entry point. Its 60 lines define the entire public API surface via `mod` declarations and `pub use` re-exports.

**Partial fix applied:** `mod_item` declarations (e.g. `pub mod cursor;`) are now captured as Import items, and `pub use` re-exports in lib.rs are no longer penalized by `reexport_contribution()`. This fixed module structure visibility in smaller Rust crates (sps_core, thiserror, toasty_codegen), but toasty's lib.rs remains empty because Import base_value (0.1) and ImportedItems base_value (1.0) can't compete on per-token ratio against FunctionName entries (~2 tokens each) in an 8000-token workspace with 8 crates. The `pub use` lines are ~3-4 tokens each, giving them a ratio of ~0.14 vs ~0.28 for function names. Fixing this likely requires either (a) a higher base_value for first-party ImportedItems, which has broad effects, or (b) a mechanism that boosts lib.rs content specifically, which requires threading file identity through the group system.

## 35. Third-party imports dropped in small single-file projects despite ample budget (regression) [needs human review]

**Affected snapshots:** xxhash_xxhsum

`xxhsum/xxhsum.go` is a 50-line single-file project with a 2000-token budget. The pre-rewrite output (470 tokens) showed the import block including `github.com/cespare/xxhash/v2` — the core dependency that tells a reader this is a wrapper around the xxhash library. The new output drops the entire import block (lines 3-9), showing only the three functions (lines 11-50).

Root cause: `Import { first_party: false, .. }` has base_value 0.0 in heuristics.rs. The comment says "3rd party imports only via dependent_siblings" — but in a single-file project there are no siblings, so the import can never be surfaced. The budget is vastly underutilized (the old output used ~24% of budget) yet the scheduler cannot select the import because its value is zero regardless of remaining capacity.

**Attempted fix:** Tried three approaches: (1) giving third-party imports base_value 0.05 — caused 38 snapshot regressions across all languages; (2) giving base_value 0.01 — still 23 regressions; (3) promoting ungated third-party imports to first-party in files.rs — 29 regressions. Import groups are so cheap (few tokens) that any non-zero base value makes them competitive everywhere, displacing function bodies and other higher-value content. Also tried adding Go first-party detection (stdlib imports don't contain dots), but this correctly classifies Go stdlib imports and the xxhsum import block becomes first-party, which fixes this specific case but adds import blocks to every Go file in every Go snapshot. A targeted fix may need scheduler-level awareness of budget utilization rate or a mechanism specific to single-file projects.
