# Output quality issues

## 9. Type alias and const bodies — remaining gaps

**Largely resolved (2026-04-16):** Added `TypeAliasBody` / `ConstBody` kinds that auto-commit for compact (≤3 line) declarations through the same hatch EnumBody uses. Blind A/B over 35+ fixtures: 10 clear wins (semver_internal constants, go_multierror chain alias, xxhash primes, enclosed_crypto algorithms, htmy type aliases, tock_internal_core sentinel errors, typeguard callable aliases, toasty_core Result, enclosed_lib, htmy_renderer), 0 regressions, rest ties.

**Remaining gaps** are specifically about *class* and *interface* bodies, which this fix doesn't touch:

- **Python class bodies** with inheritance / dataclass fields (microbootstrap_instruments, microbootstrap, nano_vllm, pluggy's `TypedDict`s). A `class Foo(Bar):` line's first meaningful content is the inheritance chain; the body is the field list. Python ClassBody is already spawned — so the issue is that it's losing ratio competition, not that the group is missing. May need a Python-specific auto-commit hook for compact dataclass/TypedDict bodies.
- **JS class method signatures** (semver's Comparator/Range/SemVer). JS class bodies don't spawn a body group at all; methods are extracted via `spawn_method_children` as FunctionName groups that truncate to bare names. Showing full method signatures without changing class-level scheduling is the open problem.
- **Go interface method signatures** (tock_internal_core's `ActivityResolver` etc.). No `InterfaceBody` group for Go; interfaces render as bare `type X …`.

These three sub-gaps each want a different mechanism — the single TypeAliasBody/ConstBody fix only covered the class of items where the declaration is compact enough to auto-commit cheaply.

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

**Affected snapshots:** mdbook_guide_src, superstruct, soluna, enclosed; also toasty (workspace-level regression from the fix below).

`CONTEXT.md` reclassified from AiConfig (factor 0.1) to Normal, fixing per-crate toasty / toasty_codegen / toasty_core. Workspace-level toasty regressed: 5 CONTEXT.md heading tables crowded out real code.

**Proposed fix (diagnosed 2026-04-16):** separate two levers that are currently conflated.

- `files_base_value(role)` controls whether the *file itself* is visible (its header line in the output tree).
- `files_contribution(role)` is the modifier passed to the file's *children* (headings, bodies, etc.) — controls how aggressively its internal content competes.

The clean version is a dedicated `FileRole::ProjectContext` (CONTEXT.md, SUMMARY.md, OVERVIEW.md, NOTES.md, ROADMAP.md) with `base_value` ≈ 1.0 and children contribution ≈ 0.3, plus dropping Architecture's children contribution from 1.0→0.3.

**A/B findings (2026-04-17, tried this exact diagnosis):** 4 wins / 2 losses / 1 tie across the 7 affected snapshots — net positive, but the two regressions were **both on toasty**, the workspace that motivated the issue. Reviewers cited toasty's per-crate CONTEXT.md section headers as load-bearing (toasty's own CLAUDE.md routes agents through them), so shrinking their children modifier hurts rather than helps. mdbook_guide_src, superstruct, toasty_core, and sqlite_vec improved — for them the heading TOC really was crowding out code.

This is the opposite of the issue's original "Normal caused toasty to regress" claim. Either the prior claim was mistaken, or the fixture baseline has moved since — either way, the prescribed fix doesn't resolve the stated target.

Open questions for a future pass: (a) is the target modifier per-file rather than per-role (large-dir workspace wants lower, small-dir crate wants higher)? (b) is there a signal that CONTEXT.md *contents* carry real architectural information (depth, table density, cross-reference density) that could weight them up while down-weighting boilerplate TOCs?

## 37. Entire implementation modules omitted while siblings are shown

**Affected snapshots:** sds (sds.c), superstruct_src_structs (valid.rs, prop.rs, fallback.rs, expand.rs), xlstm (backends/, vanilla/, blas/), xlstm_blocks (backend implementation files), sps (sps-net/src/api.rs)

Current sometimes omits the file where the actual implementation lives while showing headers, configs, and adjacent code. sds is a single-header C string library where `sds.h` is shown and `sds.c` is entirely absent — the reader sees the API but not where any of it is implemented. In superstruct_src_structs, the proc-macro pipeline (`derive → try_expand → impl_struct → impl_enum`) lives in valid.rs/prop.rs/fallback.rs/expand.rs which current leaves essentially empty. In sps, the entire `sps-net/src/api.rs` module (~8 fetch/get functions defining the networking surface of a package manager) is dropped. This is not a dedup or volume issue; current is actively choosing siblings over the implementation files. Related to #21 (mcphost) but the pattern is more general — sometimes the "boring" file IS the core.

## 38. Function signatures over-elided to bare names even with budget headroom

**Affected snapshots:** mcphost_sdk, vaul, xlstm, xlstm_blocks, ky_source_errors, sps

Current frequently renders `fn foo …` / `func Foo …` / `def foo …` (name only with ellipsis) at 4000/8000 budgets where parameters and return types would fit. For typed languages parameters+returns carry most of a function's documentary value — eliding them leaves content close to information-free, since the filename already implies the function exists. Agents comparing vaul, mcphost_sdk, and the xlstm family consistently flagged that the other side's fuller signatures built a better mental model. This may be a scheduler issue (committing the name group without also committing the body/sig group when budget allows), or a value-model issue (sig groups losing per-token to cheaper name groups).

## 39. README content past top-level headings dropped — deep subsections and usage code blocks lost

**Affected snapshots:** pluggy, go_multierror, ky_source_errors, mdbook_guide_src, mcphost, enclosed

Distinct from resolved issue #6 (which addressed h1 body content for headingless READMEs): these fixtures have READMEs with deeper structure (h3/h4 subsections enumerating the API, tips, or features) and embedded usage examples in code blocks, which current collapses to top-level section headers only. ky_source_errors is a clear example — the README's `### ky.get/post/put/.../extend/create` subheadings map directly to the public API surface and are worth more than the single `## API` heading current shows. pluggy and go_multierror lose README usage examples that are the fastest path to understanding what the library does. mdbook_guide_src loses the root README narrative entirely. A fix likely involves bumping HeadingBody value for h3+ headings in README files, or special-casing README section trees for deeper body preservation.

## 46. Hidden files silently excluded — dotfile configs never reach classification

`src/group/folders.rs:15` builds the walker with `ignore::WalkBuilder::new(...)` and never calls `.hidden(false)`. The `ignore` crate's default filters out all dotfile entries before they reach `classify::is_source_file`, so hidden files are dropped at walk time regardless of role or content.

**Correctness bug.** Hiding `.mcp.json`, `.github/workflows/`, `.goreleaser.yaml`, `.vscode/`, `.claude/`, etc. implies these files don't exist in the project. Per CLAUDE.md's "Don't confuse the reader," excluding a file is a stronger claim than including it. Dotfiles aren't inherently high-signal, but they describe parts of the project's shape (MCP wiring, CI topology, release automation, editor setup, agent wiring) that can be load-bearing for specific follow-up tasks, and the walker shouldn't decide in advance that a reader won't need them.

**Plan:** re-land `.hidden(false)` (with a `.git` filter) and let the resulting files compete on the normal value/cost axis. The 2026-04-16 attempt was reverted only because the A/B review flagged regressions, but those verdicts were agent miscalibration — agents projected their own aesthetic ("editor config is noise") onto a question about project-shape signal. The A/B review workflow landed this session (see `.claude/skills/ab-snapshots/`) now has a prompt frame grounded in CLAUDE.md principles and the four failure modes — re-run with this in place. The value model for directories with sparse/empty content should also benefit from the nonlinear-in-lines treatment landing via issues.md #4.
