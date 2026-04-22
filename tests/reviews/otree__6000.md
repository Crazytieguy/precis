---
snapshot_hash: 808106e54bb445ab7516509159359061c7234995e7e890a43bb78e4b096201f4
---

## Summary

The 6000-budget snapshot is severely misaligned with the North Star. While most tier-1 listings and a handful of tier-2 type declarations are present, the snapshot systematically skips nearly all of the 2.x behavior content (README usage 2.2, Action enum 2.12, default key bindings 2.13, Cargo deps 2.14), almost every 3.x batch (the single most valuable doc `docs/actions.md` table in 3.1 is absent, as are the main-run path 3.9/3.10 and `CommandArgs` flag continuations 3.2/3.3/3.4/3.5), and every 4.x batch (the event-loop core 4.2, tree builder 4.3, parser impls 4.8-4.14, syntax helpers 4.15/4.16). In their place the snapshot spends heavily on below-the-fold content — ~200 lines of `config/colors.rs` struct fields (DataColors, TreeColors, FooterColors, PopupColors, FilterColors) and ~200 lines of `config/mod.rs` impl-Default stubs (all elided bodies) that the North Star explicitly demotes. The dominant pattern is: struct field headers across many files beat ranked prose, docs, and logic.

## Divergences

### Ranking
- [rust] [major] 2.2 (`README.md:33-46` usage and config pointers) missing; snapshot instead shows below-the-fold `Colors` sub-structs `DataColors`/`TreeColors`/`FooterColors`/`PopupColors`/`FilterColors` (colors.rs:78-327) and below-the-fold `config/mod.rs:127-329` impl-Default stubs.
- [rust] [major] 2.12 (Action enum `keys.rs:317-348`) missing; snapshot shows below-the-fold `Keys` struct header (`keys.rs:195`) and `KeyAction` (`keys.rs:351-354`) instead.
- [rust] [major] 2.13 (default key bindings `keys.rs:284-315`) missing; same below-the-fold displacers.
- [rust] [major] 2.14 (`Cargo.toml:13-36` dependencies) missing while below-the-fold `config/mod.rs` impl stubs and colors sub-structs are present.
- [markdown] [major] 3.1 (`docs/actions.md:1-34` action↔keys table, highest-value doc) missing; displaced by below-the-fold colors/config content.
- [rust] [major] 3.2/3.3/3.4/3.5 (`CommandArgs` flag continuations + `get_content_type` + `update_config`) all missing; displaced by below-the-fold content.
- [toml] [major] 3.6/3.7 (`config/default.toml` body) missing; displaced by below-the-fold content.
- [markdown] [major] 3.8 substantially missing — only `docs/changelog.md:1,3,5,6` of the ranked 1-45 range shown; remainder of v0.6.x entries absent while below-the-fold content is present.
- [rust] [major] 3.9/3.10 (`main.rs::run()`) missing; displaced by below-the-fold content.
- [rust] [major] 3.11 (`Tree::parse`/`from_value`) missing; displaced by below-the-fold content.
- [rust] [major] 3.12/3.13 (`ui::start`, `TreeOverview::on_key`) missing; displaced by below-the-fold content.
- [markdown] [major] 3.14/3.15 (actions.md key-syntax reference, README roadmap) missing; displaced by below-the-fold content.
- [rust] [major] 4.1/4.2 (`App::new`, `App::on_key` — event-loop core) missing; displaced by below-the-fold content.
- [rust] [major] 4.3 (`Tree::build_item`) missing; displaced by below-the-fold content.
- [rust] [major] 4.4 (`App::build_edit`/`get_copy_text`/`popup_help`) missing.
- [rust] [major] 4.5/4.6/4.7 (`TreeOverview` methods) missing.
- [rust] [major] 4.8-4.14 (every parser impl: JSON, YAML, TOML, XML, HCL, JSONL, Any) missing.
- [rust] [major] 4.15/4.16 (`SyntaxToken::pure_text`, `StringValue`) missing — only the `StringValue` enum header shown.
- [rust] [major] 4.17 (`DataBlock` entrypoints) missing — only struct fields present.
- [rust] [major] 4.18/4.19 (`FilterOptions::filter`, `Header::new/draw`) missing.
- [rust] [major] 5.1 missing body (`FileWatcher` public API `new`/`get_err`/`parse_tree`) — only struct fields shown.
- [rust] [major] 5.2 (`clipboard.rs` OS dispatch) missing — only `write_clipboard` signature shown.
- [rust] [major] 5.5 missing body (`Edit::new` constructor) — only struct shown.
- [rust] [major] 5.6 (`debug` macro + `set_file`) missing body — only macro signature stub shown.
- [toml] [major] 5.7 (`config/default.toml:62-109` colors + types) missing.
- [other-language] [major] 5.8 (parser test-case folder listing with `_highlight` pair convention) missing — only bare folder names shown with no files.
- [other-language] [major] 5.9 (`examples/example.json:1-15` teaser) missing.
- [toml] [major] 5.10 (`typos.toml` + `.gitignore` bodies) missing — only filenames listed.

### Batch correctness
- [markdown] [minor] 1.10 partial: `README.md:1,3` shown but line 5 tagline ("A command line tool to view objects (JSON/YAML/TOML/XML) in TUI tree widget") omitted, which is the entire value of the batch (the cross-ref to 1.5/2.5 regarding HCL/JSONL).
- [rust] [minor] 2.4 partial: only line 66-67 of `ContentType::new_parser` dispatch shown with `…` elision hiding the entire match body — the exact dispatch mapping that is the whole point of the batch is missing.
- [rust] [minor] 2.8 partial: `Types` struct fields (28-46) shown but the `generate_types_default!` defaulting macro is elided; the batch's point is the "defaulted to field name" convention.
- [rust] [minor] 2.9 partial: `tree.rs:14-49` shown but imports (1-13) are omitted.
- [rust] [minor] 2.11 partial: `ScrollDirection` (48-51), `App` struct (53-84), `ShowResult` (86-89) shown but `Refresh` enum (27-?) and `ElementInFocus` enum — both called out explicitly in the batch — are absent.
- [rust] [minor] 2.15 partial: `HeaderContext` struct (11-16) shown but `new` and `format` methods (17-39), which carry the `{version}/{data_source}/{content_type}/{data_size}` substitution semantics, are absent.
- [rust] [minor] 2.16 partial: only `cmd.rs:13 pub struct CommandArgs {` shown followed by `…` — none of the actual flags are present.
- [markdown] [minor] 3.8 partial: only `changelog.md:1,3,5,6` shown (v0.6.4 fixes) — v0.6.3/v0.6.2/v0.6.1 entries from the 1-45 range are missing.
- [rust] [minor] 5.3 partial: `Colors` struct (21-42) shown but imports/macro (1-20) and `default_header`/`default_focus_boder` functions (43-76) absent.
- [rust] [minor] 5.4 partial: `Color` struct (342-353) shown but `Color::new` and palette-aware `Color::parse` (354-399), which are the behavioral heart of the batch, are absent.

### Honesty
- (none)
