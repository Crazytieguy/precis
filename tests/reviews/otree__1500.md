---
snapshot_hash: 6a1c3833e33c7c3ba540a51407f9d79d2f858e3c77bae65e5724f7fd4d5f4878
---

## Summary

At ~1500 tokens the snapshot is a wide-but-shallow struct-signature sweep rather than a budget-respecting prefix of the North Star. Tier-1 listings 1.1 / 1.2 / 1.6 / 1.7 / 1.8 / 1.9 are present and 2.17 renders in full, but 1.3 (`main.rs` mods), 1.4 (`src/ui/` file listing), 1.5 (`src/parse/` file listing), and 1.11 (`SyntaxToken`) are absent while the snapshot simultaneously surfaces partial tier-5 batches (5.1 `FileWatcher`, 5.2 `write_clipboard`, 5.5 `Edit`, 5.6 `debug`) and the below-the-fold `Keys` / `KeyAction` struct headers from `keys.rs`. Almost every tier-2 batch beyond 2.9 / 2.17 is missing — no `Parser` trait, no `ContentType`, no `App` struct, no `Action` enum, no default keys, no dependencies, no `HeaderContext`. Several batches that do appear (1.10, 2.8, 2.9, 2.10, 2.16, 2.17) are rendered as partial batches (typically a struct header followed by `…`), which is also a batch-correctness problem. The dominant pattern is that the renderer emits a header teaser for almost every top-level declaration it sees in every file it touches, regardless of rank.

## Divergences

### Ranking
- [rust] [major] 1.3 (`src/main.rs:1-9` mod declarations) absent; `main.rs` appears as a bare filename with no body, while tier-5 content (5.1 `FileWatcher` struct, 5.2 `write_clipboard` signature, 5.5 `Edit` struct, 5.6 `debug.rs` macro + signatures) is rendered.
- [rust] [major] 1.4 (`src/ui/` file listing) absent; `ui/` appears as a folder name with no children, displaced by 5.x partials and the below-the-fold `keys.rs` `Keys` / `KeyAction` headers.
- [rust] [major] 1.5 (`src/parse/` file listing) absent; `parse/` appears as a folder name with no children, displaced as above. Notable because the North Star flags 1.5 as the only place the reader learns HCL and JSONL exist beyond the README tagline.
- [rust] [major] 1.11 (`SyntaxToken` enum at `src/parse/syntax.rs:11-27`) absent; parent folder `parse/` is empty in the snapshot, displaced by 5.1 / 5.2 / 5.5 / 5.6 and below-the-fold `keys.rs` headers.
- [markdown] [major] 2.2 (`README.md:33-46` usage + config pointers) absent; displaced by tier-5 content.
- [rust] [major] 2.3 (`src/ui/filter.rs:13-38` Filter state types) absent; displaced (and gated on the missing `ui/` listing).
- [rust] [major] 2.4 (`src/parse/mod.rs:66-78` `ContentType::new_parser` dispatch) absent; displaced.
- [rust] [major] 2.5 (`src/parse/mod.rs:1-34` `ContentType` enum) absent; displaced.
- [rust] [major] 2.6 (`src/parse/syntax.rs:1-10` SyntaxToken file teaser) absent; displaced.
- [rust] [major] 2.7 (`src/parse/mod.rs:36-64` `Parser` trait + default `parse_root`) absent; the core format abstraction has no surface in the snapshot, yet 5.1 / 5.2 / 5.5 / 5.6 are shown.
- [rust] [major] 2.11 (`src/ui/app.rs:27-90` `App` struct + focus/refresh enums) absent; the central TUI state is nowhere in the snapshot while 5.x appears.
- [rust] [major] 2.12 (`src/config/keys.rs:317-348` Action enum) absent; displaced by below-the-fold `Keys` and `KeyAction` headers from the same file.
- [rust] [major] 2.13 (`src/config/keys.rs:284-315` default key bindings) absent; displaced.
- [rust] [major] 2.14 (`Cargo.toml:13-36` dependencies) absent; `Cargo.toml` body after line 11 is truncated, displaced by 5.x content.
- [rust] [major] 2.15 (`src/ui/header.rs:11-39` `HeaderContext`) absent; displaced.
- [rust] [major] Below-the-fold `src/config/keys.rs:194-282` (`Keys` struct, header-only teaser at line 195) appears while tier-1 items 1.3 / 1.4 / 1.5 / 1.11 are missing.
- [rust] [major] Below-the-fold `src/config/keys.rs:350-372` (`KeyAction` struct, header-only teaser at line 351) appears while tier-1 items are missing.
- [rust] [predecessor] 2.17 (Config sub-structs) is rendered as ranked content without its declared predecessor 2.10 being rendered as ranked content — 2.10 appears only as a one-line `pub struct Config {` header with `…` elision, with all 11 fields dropped. Treating 2.10's header alone as "present" is charitable; if the predecessor requires 2.10's ranked content (the `#[serde(default=…)]` field list that makes Config the authoritative schema outline), 2.17 here violates the edge.

### Batch correctness
- [markdown] [minor] 1.10 partial: `README.md:1-5` renders line 1 (title) and line 3 (screenshot ref) but silently omits line 5 (the tagline "A command line tool to view objects (JSON/YAML/TOML/XML) in TUI tree widget."), with no elision marker between the rendered lines. The tagline is the load-bearing half of the batch.
- [rust] [minor] 2.1 partial: only `pub enum Key {` at line 54 with `…`; the 14 variants (`Char`, `Ctrl`, `Alt`, `F`, `Backspace`, `Enter`, arrows, `PageUp/Down`, `Tab`, `Esc`) at lines 55-71 are elided despite the batch declaring them together.
- [rust] [minor] 2.8 partial: only `pub struct Types {` at line 28 with `…`; the six fields (`str`/`null`/`bool`/`num`/`arr`/`obj`) and `generate_types_default!` wiring that the whole-file batch specifies are elided.
- [rust] [minor] 2.9 partial: `src/tree.rs:1-50` renders the struct/enum declarations at lines 14-49 but omits imports at lines 1-12 (notably `use crate::parse::{ContentType, Parser, SyntaxToken}` and the ratatui / tui-tree-widget pulls that the batch declares as part of 1-50).
- [rust] [minor] 2.10 partial: only `pub struct Config {` at line 17 with `…`; the 11 `#[serde(default = …)]` fields (tree / editor / data / layout / header / footer / filter / palette / colors / types / keys) that make the batch an authoritative schema outline are elided.
- [rust] [minor] 2.16 partial: only `pub struct CommandArgs {` at line 13 with `…`; no flag fields (`path`, `config`, `content_type`, `to`, `disable_header/footer/filter`) and none of the clap attributes that `src/cmd.rs:11-40` is about are rendered.
- [rust] [minor] 2.17 partial: sub-struct fields are shown, but the `#[derive(Debug, Clone, Serialize, Deserialize)]` attributes above each struct are silently missing. A reader relying on the batch to recreate the TOML schema surface would not know the types are serde-derived.
- [rust] [minor] 5.1 partial: only the `FileWatcher` struct fields at lines 16-25; `new` (which spawns the `watch_file` thread), `get_err`, and `parse_tree` that the North Star specifies as part of `:16-91` are elided under `…`.
- [rust] [minor] 5.2 partial: only the `write_clipboard` fn signature with `…`; the OS-specific command selection (`pbcopy` / `wl-copy` when `WAYLAND_DISPLAY` else `xclip` / `clip`) that is the entire point of the whole-file batch is missing.
- [rust] [minor] 5.5 partial: only the `Edit` struct at lines 10-14; `Edit::new(cfg, identify, data, extension)` that the North Star specifies as the load-bearing content of `:1-40` is missing.
- [rust] [minor] 5.6 partial: batch is `src/debug.rs` whole file. Snapshot shows the `debug!` macro body (lines 8-15), the `set_file` signature (line 19) with `…`, and the `write_logs` signature (line 23) with `…`; `FILE: OnceLock<String>` (line 17) and both function bodies are missing.

### Honesty
- (none)
