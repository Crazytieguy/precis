# wave2-ts resume note (quota wind-down 2026-07-04)

Branch state: clean; all kept levers committed. Corpus mean 0.6105 → **0.6142**.

## Kept levers (committed, measured, targeted rows verified)
1. `353cf217` declared-types API contract (axios): roster shapes of the root
   package.json `types`-target .d.ts exempt from machinery damp + surface seed.
   axios 0.425→0.427 at 3K, +0.09 Score(B=cum) at 10K. Full-file exemption
   re-measured −0.086 (flood of small aliases); ExportNames-damped variant = baseline.
2. `01d44de9` API-spine re-export walls priced as roster (cat 0.8 axes), gated to
   reexport-target ∪ declared-entry source twin, primary package only.
   vite 0.470→0.498. Ungated: d2ts −0.023 / linkwarden −0.028 (gate fixed linkwarden).
3. `8f8a09eb` entrypoint depth-pin+boost gated to package-shallow (≤2 comps) or spine.
   axios→0.487, chalk→0.701, d2ts→0.659. Moves excalidraw validation headline
   (fixture_baselines_validation_excalidraw fails until post-merge validation regen; diff unread).
4. `4dd79b0e` chained CommonJS default-export (`exports = module.exports = X`) linking.
   express 0.567→0.577.

## Reverted
- dockly data-module recall (src-resident *.config.js undamp + multi-line data-const
  synthetic exports): recall worked but layout bodies rank ratio ~128 vs ~149 cutoff;
  82 tok of roster overhead evicted Screen-ctor credit at the 3K boundary → dockly −0.038.
  Needs transitive require-graph surface tiering (re-exported 1.0 / import-reachable
  ~0.8 / else 0.5) before retry.

## Mid-flight (not applied — next session's first step)
/code-review findings awaiting fixes (all cleanup/efficiency, no confirmed correctness bug):
- is_declared_package_entry_source: `with_extension` replaces dotted stems
  (foo.config → foo.ts probe); build filename by appending `.{ext}` instead.
  Also only strips ONE generated prefix (misses dist/esm/index.js); loop the strip.
- Memoize is_api_spine_entrypoint per file + package_entry_targets per pkg_dir
  (currently re-reads/re-parses package.json up to 2× per entrypoint file).
- Extract is_secondary_ts_workspace_member predicate (spine gate reads the tuned
  factor as a boolean).
- Drop redundant `is_entrypoint_file(file) &&` at the unchunked wall site (api_spine implies it).
- json.rs: 3rd copy of manifest-parse prologue; ENTRY_SOURCE_EXTS vs TS_JS_EXTS drift risk.
- is_mostly_reexport votes on lines (formatting-sensitive); per-group flag would be robust.
- Late finder results (all false-negative / off-corpus edge severity; none confirmed on
  training): collect_string_leaves skips JSON arrays (exports fallback-array form yields
  no entry targets); generated-prefix strip is single-level and lacks 'cjs'
  (dist/esm/index.js, dist/cjs/index.js never map to src/index.ts); commonjs chain unwrap
  stops at parenthesized_expression (`module.exports = (exports = fn)` → wrong ItemKind;
  `exports = (module.exports = X)` → missed entirely); is_pinned_entrypoint fails OPEN
  (depth 0) when strip_prefix(root) fails — symlinked root spelling would re-grant every
  deep index.ts the pin, prefer fail-closed; un-stripped 'lib' variant can spine-mark a
  committed lib/ bundle AND its src twin; depth-pin gate has no re-establishment path for
  bin-field-only CLI entries or manifest-less repos (deep src/server/index.ts loses the
  old unconditional boost — acceptable per training, revisit if off-corpus regressions).
All are behavior-preserving or false-negative-only; verify with a plain regen (expect zero divergence diffs except the entry-mapping fixes, which can only ADD spine files).

## Recommended, not implemented
- Transitive require/import-graph public-surface tier (svgo 0.097 / express utils /
  dockly grid.config enabler) — fleet js-late lever 4.
- Oversize-class chunked member catalogs above JS_CLASS_MEMBER_SPLIT_MAX (commander 0.195 unsched).
- Script-flow/expression-statement recall (commander examples, dockly/audiobookshelf index.js).
- audiobookshelf: surface-reachable JS should be primary regardless of dir name (0.359 unsched).
