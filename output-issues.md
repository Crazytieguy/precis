# Output quality issues


## ~~6. README body content dropped — only headings shown (resolved)~~

Resolved. A 2× modifier boost for README h1 HeadingBody groups combined with a 12-line cap on markdown h1 body rendering fixed 9 of the 16 affected snapshots. Auto-commit of root-level README h1 body content fixed 4 more. Remaining 4 snapshots are fundamentally limited: pluggy/typeguard use RST (no tree-sitter parser), mcphost already shows README body via ratio competition, semver's README has no h1 body content.



## 9. Type alias and const bodies missing from taxonomy (regression)

**Affected snapshots:** bareiron, cmdk, cmdk_cmdk_src, enclosed, enclosed_crypto, enclosed_lib, go_multierror, htmy, htmy_renderer, ky, ky_source_errors, mcphost_sdk, microbootstrap, microbootstrap_instruments, mitt, nano_vllm, nano_vllm_engine, pluggy, py3xui, py3xui_api, semver, semver_classes, semver_internal, superstruct, tock, tock_internal_core

This is the highest-impact unresolved issue — the 2026-04-13 blind pairwise eval against origin/main attributed the majority of the 26 current-vs-main regressions (out of 64 fixtures) to some variant of this truncation. Agents repeatedly cited missing class inheritance / Protocol bases (htmy_renderer, microbootstrap_instruments), missing struct-like type alias bodies, and missing class method signatures as decisive regressions.

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

## ~~14. Server architecture lost to broad-but-shallow budget distribution (resolved)~~

Resolved. Smoothed depth penalty cliff at depth 4 (from 0.4 to 0.55), surfacing auth and config modules in the enclosed snapshot. Remaining directories (`middlewares/`, `storage/factories/`) are a volume-based budget capture issue — tracked under #21.


## ~~17. C header file budget reduced — key API declarations and struct bodies lost (resolved)~~

Resolved through two fixes: (1) a 0.3× companion-header penalty for C/C++ implementation files freed budget from .c files to headers, and (2) a bug fix in `item_identifier` where `find_descendant_of_kind(decl, "type_identifier")` was finding parameter types (e.g. `search_params_t`) instead of function names, causing `dedup_overloads` to incorrectly collapse functions sharing the same first custom-type parameter. krep.h now shows all 29 function declarations including search_file, search_string, boyer_moore_search, kmp_search, regex_search, SIMD variants, thread_pool_submit/wait_all, match_result_add/free/merge. Struct bodies remain truncated — covered by issue #18.

The dedup bug fix also improved sds (+20 function declarations), soluna (+16 function declarations), sqlite_vec (+30 function declarations), bareiron (gained README body content + functions). neco regressed: with all 119 functions correctly extracted (vs ~50 before the fix), the FunctionName groups are larger and more expensive, causing #define constants (error codes, time units) to win budget over function declarations. This is a pre-existing scoring issue (MacroName ratio >> FunctionName ratio for large groups) exposed by the fix, not caused by it.

## 18. Struct and enum bodies elided — many small entries beat fewer large ones (partially fixed)

**Partially fixed:** Added an auto-commit mechanism for compact enum bodies (≤25 lines). When the scheduler commits an enum name, it immediately commits the body too — bypassing ratio-based competition where cheap FunctionName entries (1.0 base_value, ~2 tokens, ratio 0.5) always beat multi-line bodies (1.5 base_value, ~20 tokens, ratio 0.075). Auto-commit only fires when body value ≥1.0 and >75% budget remains, preventing displacement of other content. This fixed enum bodies in 6 snapshots: log (Level/LevelFilter variants), mdbook (TextDirection/RustEdition), otree (Key/LayoutDirection), sps (JobProcessingState), sqlite_vec (VectorElementType/DistanceMetrics/TokenTypes), toasty_core (Operation/Rows).

**Remaining affected snapshots:** log_src_kv, neco, thiserror, thiserror_impl_src, tock, toasty_codegen

The 2026-04-13 blind pairwise eval added log_src_kv to this list — the `Inner` enum variants (Boxed/Msg/Fmt) and cfg-gated submodule declarations in `sval_support`/`serde_support` are load-bearing for understanding the crate's feature-gated architecture and were cited as a decisive regression vs origin/main.

The fix only applies to EnumBody groups. StructBody auto-commit was attempted but cascades to regressions in unaffected snapshots (mcphost struct bodies displace function signatures, xlstm_blocks CUDA helpers displace Python method signatures). Well-documented Go structs are particularly problematic — Go projects tend to have many small documented structs that all qualify for auto-commit, consuming significant budget in aggregate.

The remaining affected snapshots all need struct body improvements: neco (`neco_stats` struct fields), thiserror/thiserror_impl_src (AST data model structs: Struct, Enum, Variant, Field, Attrs), tock (`AnalysisStats` struct fields), toasty_codegen (`Filter` struct fields). Fixing these likely requires either a mechanism to distinguish architecturally important structs from implementation-detail structs, or scheduler-level changes that account for opportunity cost when auto-committing bodies.

Additionally, some larger enum bodies (>25 lines) in sps (SpsError with 15+ variants, PipelineEvent with ~20 variants) and otree (CommandArgs, ContentType, SyntaxToken) remain elided. These exceed the compact body threshold. Raising the threshold to 35 has no effect (these enums are larger still, or budget threshold isn't met).

**Additional struct auto-commit attempt (2026-04-13):** Tried adding StructBody to auto-commit with stricter thresholds (budget 7/8–15/16, line limits 8–15). At 7/8 budget + 8-line limit: mdbook improved (Summary/Link struct bodies), but mcphost regressed (Go struct bodies displaced private function names) and toasty_core regressed (SchemaMutations body displaced relation type names). At 9/10 budget: only mcphost changed (arguably an improvement — gained data model, lost private helpers — but not a target snapshot). At 15/16: no effect. The target snapshots (neco, thiserror, tock, toasty_codegen) never benefit because their struct names are committed too late in scheduling (budget already consumed). The auto-commit approach fundamentally can't reach deep-project structs without also catching shallow Go structs that cause regressions.

## 21. Volume-based budget capture — large support package crowds out small core package

**Affected snapshots:** mcphost

`internal/ui/` (24 source files, generic terminal UI rendering) captures ~191 output lines (~30% of the budget). `internal/tools/` (4 source files, core MCP tool management) gets zero output lines. Both directories are at the same depth and classified as Source.

`internal/tools/` contains the project's core domain logic: `MCPToolManager` (the central type managing MCP tools across servers), `MCPConnectionPool` (connection lifecycle and health checking), and the tool mapping/invocation machinery. This is literally what MCPHost is — "a CLI host that enables LLMs to interact with external tools through MCP." A reader of the output would understand how mcphost renders spinner animations and style badges, but not how it connects to or invokes MCP tools.

The `internal/ui/` content is individually reasonable (function names at base_value 1.0, same modifier) but collectively overwhelming. With 24 files generating 100+ FunctionName groups, each cheap (~2 tokens), they win the budget competition through volume. The sublinear scaling `(item_count).powf(0.75)` dampens the advantage at the file-group level (FilesGroup base value), but doesn't limit how many TsGroups are spawned from many files. The tools package's 4 larger files generate fewer groups that individually lose to the UI's many small entries.

This is a pre-existing issue (the old output also omitted `internal/tools/`) but is more damaging after the rewrite because the old output compensated with richer content in the files it did show (struct field bodies, type definitions, doc comments). The new output's broader-but-shallower coverage makes the absence of core domain code more conspicuous.

**Attempted fix:** Tried per-directory file-count dampening (N^(-alpha) modifier for directories with >K source files). With threshold 4, exponent 0.2: 32 snapshot failures. With threshold 6, exponent 0.4: 31 failures. With threshold 8, exponent 0.3: 20 failures. With threshold 10, exponent 0.35: 0 failures (no effect). The fundamental issue is structural: TsGroups cost ~2 tokens (ratio ~0.5) while FilesGroups for `internal/tools/` cost ~12 tokens (ratio ~0.07). No dampening of UI TsGroups can bridge a 7× ratio gap. Fixing this likely requires scheduler-level changes: either a coverage-aware scheduling phase that ensures small directories get FilesGroups committed before TsGroups consume the budget, or a mechanism that discounts FilesGroup costs for small focused directories.



## ~~27. Example source files absent while library README dominates budget (resolved)~~

Resolved through incremental fixes: weather_agent files now show function signatures (agents.py, evals.py), README is ~62 lines (17% of 368 total output), and 6 example directories show source content (support_bot, personal_shopper, triage_agent, basic, weather_agent, customer_service_streaming). The customer_service_streaming src/ engine architecture is absent but this is a reasonable tradeoff — showing breadth across 6 simpler examples builds a better mental model than going deep on one complex sub-project.


## 30. Re-export entry files (Rust `lib.rs`, TS/JS `index.ts` barrels) rendered empty

**Affected snapshots:** toasty, d2ts, d2ts_d2ts, superstruct

`crates/toasty/src/lib.rs` is the main ORM crate's entry point. Its 60 lines define the entire public API surface via `mod` declarations and `pub use` re-exports. The same structural role is played by `src/index.ts` and nested barrel files (e.g. `src/operators/index.ts` in d2ts, `src/sqlite/index.ts`) in TS/JS packages, which declare the public API via `export { X } from './foo'` and — for dataflow libraries like d2ts — enumerate the operator family that is a core concept.

**Partial fix applied (Rust side):** `mod_item` declarations (e.g. `pub mod cursor;`) are now captured as Import items, and `pub use` re-exports in lib.rs are no longer penalized by `reexport_contribution()`. This fixed module structure visibility in smaller Rust crates (sps_core, thiserror, toasty_codegen), but toasty's lib.rs remains empty because Import base_value (0.1) and ImportedItems base_value (1.0) can't compete on per-token ratio against FunctionName entries (~2 tokens each) in an 8000-token workspace with 8 crates. The `pub use` lines are ~3-4 tokens each, giving them a ratio of ~0.14 vs ~0.28 for function names.

TS/JS barrel files are affected by the same ratio-competition issue and are not yet touched — the 2026-04-13 eval flagged d2ts, d2ts_d2ts, and superstruct as regressions where the other side surfaced index.ts re-exports. Fixing this likely requires either (a) a higher base_value for first-party ImportedItems, which has broad effects, or (b) a mechanism that boosts entry-file content specifically (lib.rs, src/index.ts, package index barrels), which requires threading file identity through the group system. Whatever mechanism fixes Rust lib.rs should also cover the barrel case.

## 35. Third-party imports dropped in small single-file projects despite ample budget (regression)

**Affected snapshots:** xxhash_xxhsum

`xxhsum/xxhsum.go` is a 50-line single-file project with a 2000-token budget. The pre-rewrite output (470 tokens) showed the import block including `github.com/cespare/xxhash/v2` — the core dependency that tells a reader this is a wrapper around the xxhash library. The new output drops the entire import block (lines 3-9), showing only the three functions (lines 11-50).

Root cause: `Import { first_party: false, .. }` has base_value 0.0 in heuristics.rs. The comment says "3rd party imports only via dependent_siblings" — but in a single-file project there are no siblings, so the import can never be surfaced. The budget is vastly underutilized (the old output used ~24% of budget) yet the scheduler cannot select the import because its value is zero regardless of remaining capacity.

**Attempted fix:** Tried three approaches: (1) giving third-party imports base_value 0.05 — caused 38 snapshot regressions across all languages; (2) giving base_value 0.01 — still 23 regressions; (3) promoting ungated third-party imports to first-party in files.rs — 29 regressions. Import groups are so cheap (few tokens) that any non-zero base value makes them competitive everywhere, displacing function bodies and other higher-value content. Also tried adding Go first-party detection (stdlib imports don't contain dots), but this correctly classifies Go stdlib imports and the xxhsum import block becomes first-party, which fixes this specific case but adds import blocks to every Go file in every Go snapshot. A targeted fix may need scheduler-level awareness of budget utilization rate or a mechanism specific to single-file projects.

## 36. Human-authored meta-documentation under-weighted

**Affected snapshots:** toasty, toasty_codegen, mdbook_guide_src, superstruct, soluna, enclosed

Several fixtures contain intentionally-authored overview documents: `CONTEXT.md` files in each toasty workspace crate describing crate purpose and change patterns, `ARCHITECTURE.md` in enclosed, `SUMMARY.md` (the authoritative table of contents) in mdbook_guide_src, `docs/*.md` guide files in superstruct, and the Lua API reference directory in soluna. These are the highest signal-per-token content for orientation but are either dropped entirely (toasty_codegen's CONTEXT.md shown as just its title line) or truncated past the first few lines. The current value model treats them as generic markdown without accounting for the "project front door" role. A value bump for markdown files whose name matches `CONTEXT|ARCHITECTURE|SUMMARY|DESIGN` (and similar), or whose location suggests front-door role (`docs/` top-level), may help.

## 37. Entire implementation modules omitted while siblings are shown

**Affected snapshots:** sds (sds.c), superstruct_src_structs (valid.rs, prop.rs, fallback.rs, expand.rs), xlstm (backends/, vanilla/, blas/), xlstm_blocks (backend implementation files), sps (sps-net/src/api.rs)

Current sometimes omits the file where the actual implementation lives while showing headers, configs, and adjacent code. sds is a single-header C string library where `sds.h` is shown and `sds.c` is entirely absent — the reader sees the API but not where any of it is implemented. In superstruct_src_structs, the proc-macro pipeline (`derive → try_expand → impl_struct → impl_enum`) lives in valid.rs/prop.rs/fallback.rs/expand.rs which current leaves essentially empty. In sps, the entire `sps-net/src/api.rs` module (~8 fetch/get functions defining the networking surface of a package manager) is dropped. This is not a dedup or volume issue; current is actively choosing siblings over the implementation files. Related to #21 (mcphost) but the pattern is more general — sometimes the "boring" file IS the core.

## 38. Function signatures over-elided to bare names even with budget headroom

**Affected snapshots:** mcphost_sdk, vaul, xlstm, xlstm_blocks, ky_source_errors, sps

Current frequently renders `fn foo …` / `func Foo …` / `def foo …` (name only with ellipsis) at 4000/8000 budgets where parameters and return types would fit. For typed languages parameters+returns carry most of a function's documentary value — eliding them leaves content close to information-free, since the filename already implies the function exists. Agents comparing vaul, mcphost_sdk, and the xlstm family consistently flagged that the other side's fuller signatures built a better mental model. This may be a scheduler issue (committing the name group without also committing the body/sig group when budget allows), or a value-model issue (sig groups losing per-token to cheaper name groups).

## 39. README content past top-level headings dropped — deep subsections and usage code blocks lost

**Affected snapshots:** pluggy, go_multierror, ky_source_errors, mdbook_guide_src, mcphost, enclosed

Distinct from resolved issue #6 (which addressed h1 body content for headingless READMEs): these fixtures have READMEs with deeper structure (h3/h4 subsections enumerating the API, tips, or features) and embedded usage examples in code blocks, which current collapses to top-level section headers only. ky_source_errors is a clear example — the README's `### ky.get/post/put/.../extend/create` subheadings map directly to the public API surface and are worth more than the single `## API` heading current shows. pluggy and go_multierror lose README usage examples that are the fastest path to understanding what the library does. mdbook_guide_src loses the root README narrative entirely. A fix likely involves bumping HeadingBody value for h3+ headings in README files, or special-casing README section trees for deeper body preservation.

## 40. Module-level doc comments not surfaced

**Affected snapshots:** log_src_kv, mcphost_sdk, xxhash

Rust `//!` crate/module doc headers, Go package doc comments, and Python module docstrings label a file's purpose in plain English at very low token cost, and are often the single highest-signal line per file for orientation. Current drops them in favor of per-symbol signatures. Eval agents consistently flagged this as decisive — e.g. log_src_kv's `//! Structured logging.` / `//! Structured keys.` headers tell a reader what each file is in one line where a wall of signatures cannot. `ModuleDocFirst` / `ModuleDocRest` groups exist in the taxonomy but apparently lose ratio competition against cheap name entries in these fixtures.

## 45. Java fields silently dropped

Java `field_declaration` nodes reach `ConstName::from_parse` (`src/group/ts/const_.rs`) only if `java_field_is_static_final(...)` returns true (`src/parse/mod.rs:467-484`). Regular instance fields — which define the shape of every Java DTO, entity, and record-adjacent class — are filtered out at parse time and never appear in output. The same rejection applies via whatever `ClassName::children` spawn logic does for TS/Java class bodies: methods are extracted, fields are not.

Impact: Java snapshots (and by extension TS class-body rendering, which has the same blind spot) hide the structural portion of the type. For a reader trying to understand a Java class, "what data does it carry" is roughly as important as "what methods does it expose", and the current output shows only the latter.

Fix path: extend `ClassName::children` to spawn a field-listing child group (mirroring `spawn_method_children`'s shape), gated on language. Similar to #9 in spirit — a missing body kind for a construct whose body content is load-bearing. Scoped out of the TsGroupKey refactor batch because it requires new spawn logic plus calibration, not just the structural refactor. File now so it doesn't get lost.

## 46. Hidden files silently excluded — dotfile configs never reach classification

`src/group/folders.rs:15` builds the walker with `ignore::WalkBuilder::new(...)` and never calls `.hidden(false)`. The `ignore` crate's default filters out all dotfile entries before they reach `classify::is_source_file`, so hidden files are dropped at walk time regardless of role or content.

This silently omits files that are often the single most informative artifacts for orientation: `.mcp.json` (MCP server wiring), `.env.example` (runtime configuration surface), `.nvmrc` / `.ruby-version` / `.tool-versions` (language version pinning), `.clang-format` / `.editorconfig` (style contract), `.github/workflows/*.yml` (CI topology and release automation), `.claude/` plugin wiring for this repo itself. For a reader trying to understand what a project *is*, "there's a `.mcp.json` at the root declaring two MCP servers" is higher signal than most source files.

Per CLAUDE.md's "don't confuse the reader" principle this is a correctness bug, not just an omission: the output implies these files don't exist. A reader inferring the tech stack from the visible tree will miss MCP integration, CI, environment shape, and toolchain pins entirely.

Fix path: `.hidden(false)` on the `WalkBuilder` opens the gate. Gitignore filtering stays active (it's an independent filter), so genuinely secret files like real `.env` files that are gitignored still won't surface. Classification then needs to decide what to do with dotfiles — most are config-shaped (JSON/TOML/YAML) and the existing `is_source_file` check will need to admit them, possibly with a new `FileRole` for dotfile configs so they can be weighted deliberately rather than competing as generic source. Calibration work required; non-trivial.

