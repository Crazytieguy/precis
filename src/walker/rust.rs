//! Rust walker. Per-item pub-declaration batches plus file-scope
//! crate-doc / mod-use / impl-method-groups and cross-file macro surface.
//!
//! Per-file keys:
//! - `CrateDocLede { file }`: `//!` opening paragraph (entrypoints only)
//! - `ModUse { file }`: `use` / `mod` / `pub use` plumbing
//! - `PubItemNames { file }`: every pub item's first line as a surface
//!   listing — a cheap catastrophic-omission hedge when individual item
//!   decls don't all fit
//! - `MethodSigs { file }`: inherent + trait impl headers + method sigs
//!
//! Per-item keys (keyed by start line so each item has a distinct batch):
//! - `PubItem { file, start_line }`: one pub item's declaration (struct
//!   fields / enum variants / trait method sigs / fn signature; no rustdoc)
//! - `PubItemDocLede { file, start_line }`: opening paragraph of the
//!   item's rustdoc — up to the first `# Heading` line, or the whole
//!   doc when no heading is present. Predecessor: matching `PubItem`.
//! - `PubItemDocBody { file, start_line }`: rest of the item's rustdoc
//!   from the first `# Heading` onward. Predecessor: matching
//!   `PubItemDocLede` (or `PubItem` directly when the doc starts with a
//!   heading and no Lede candidate is emitted).
//!
//! Cross-file keys (scoped by source directory):
//! - `MacroNames { src_dir }`: exported macro name list
//!
//! Per-macro keys (one per `#[macro_export] macro_rules!`):
//! - `MacroBody { file, start_line }`: full body of one exported macro
//!   (predecessor: `MacroNames` for the enclosing src_dir)
//!
//! Parse trees are cached in [`WalkCtx`]; the same file parsed once powers
//! every Rust batch that touches it.

use std::cell::{OnceCell, RefCell};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use tree_sitter::{Node, Tree};

use crate::batch::{Batch, BatchKey, RustKey};
use crate::content::{BatchContent, Span};
use crate::value::{depth_factor, mix_signals};

use super::{
    BodyPart, ENTRY_BODY_PART_CAP, FileLines, WalkCtx, body_part_value_factor, build_file_spans,
    build_per_file_content, coalesce_body_parts_tail, collect_doc_comments_above_filtered,
    dedup_sorted, entry_body_part_value_factor, extend_span, file_depth_factor,
    file_lines_covered_by, fs::files_with_extension, name_of, push_rows, signature_end_row,
    single_file_lines_content, statement_block_parts,
};

/// Per-run Rust-walker state owned by [`WalkCtx`] — memoizes module
/// visibility, workspace membership, and cargo source dirs across a
/// single run.
pub struct RustState {
    module_visibility: OnceCell<HashMap<PathBuf, Visibility>>,
    workspace: super::workspace::WorkspaceMembership,
    nearest_member_dir_lookup: RefCell<HashMap<PathBuf, Option<PathBuf>>>,
    cargo_source_dirs: OnceCell<Vec<PathBuf>>,
    expanded_dirs: RefCell<HashSet<PathBuf>>,
    manifest_package_lookup: RefCell<HashMap<PathBuf, bool>>,
    crate_pub_traits: OnceCell<HashSet<String>>,
}

impl RustState {
    pub fn new() -> Self {
        Self {
            module_visibility: OnceCell::new(),
            workspace: super::workspace::WorkspaceMembership::default(),
            nearest_member_dir_lookup: RefCell::new(HashMap::new()),
            cargo_source_dirs: OnceCell::new(),
            expanded_dirs: RefCell::new(HashSet::new()),
            manifest_package_lookup: RefCell::new(HashMap::new()),
            crate_pub_traits: OnceCell::new(),
        }
    }

    /// Names of `pub trait`s (not doc-hidden) declared at the top level
    /// of any crate source file in the repo. Impls of these traits are
    /// exported API surface even when the impl lives in another file
    /// (anyhow's `Context` impls in `context.rs` for the trait declared
    /// in `lib.rs`).
    fn crate_pub_traits(&self, init: impl FnOnce() -> HashSet<String>) -> &HashSet<String> {
        self.crate_pub_traits.get_or_init(init)
    }

    pub(in crate::walker) fn module_visibility_map(
        &self,
        init: impl FnOnce() -> HashMap<PathBuf, Visibility>,
    ) -> &HashMap<PathBuf, Visibility> {
        self.module_visibility.get_or_init(init)
    }

    /// `true` iff `file` is a workspace-member `Cargo.toml`. Memoized.
    pub fn is_workspace_member(&self, file: &Path, root: &Path) -> bool {
        self.workspace
            .is_member(file, || super::toml::collect_workspace_members(root))
    }

    /// Directory of the nearest enclosing `Cargo.toml` iff it's a
    /// workspace member — `None` for a non-member nested crate.
    pub(in crate::walker) fn nearest_member_dir(
        &self,
        file: &Path,
        root: &Path,
    ) -> Option<PathBuf> {
        let key = file.canonicalize().unwrap_or_else(|_| file.to_path_buf());
        if let Some(hit) = self.nearest_member_dir_lookup.borrow().get(&key) {
            return hit.clone();
        }
        let mut result = None;
        let mut dir = file.parent();
        while let Some(current) = dir {
            if !current.starts_with(root) {
                break;
            }
            let manifest = current.join("Cargo.toml");
            if manifest.is_file() {
                if self.is_workspace_member(&manifest, root) {
                    result = Some(current.to_path_buf());
                }
                break;
            }
            dir = current.parent();
        }
        self.nearest_member_dir_lookup
            .borrow_mut()
            .insert(key, result.clone());
        result
    }

    fn workspace_members(&self, root: &Path) -> &HashSet<PathBuf> {
        self.workspace
            .members(|| super::toml::collect_workspace_members(root))
    }

    pub(in crate::walker) fn cargo_source_dirs(
        &self,
        init: impl FnOnce() -> Vec<PathBuf>,
    ) -> &Vec<PathBuf> {
        self.cargo_source_dirs.get_or_init(init)
    }

    fn mark_dir_expanded(&self, dir: &Path) -> bool {
        let key = dir.canonicalize().unwrap_or_else(|_| dir.to_path_buf());
        self.expanded_dirs.borrow_mut().insert(key)
    }

    fn manifest_has_package(&self, manifest: &Path, ctx: &WalkCtx) -> bool {
        let key = manifest
            .canonicalize()
            .unwrap_or_else(|_| manifest.to_path_buf());
        if let Some(&hit) = self.manifest_package_lookup.borrow().get(&key) {
            return hit;
        }
        let hit = ctx
            .read_source(&key)
            .and_then(|text| toml::from_str::<toml::Value>(&text).ok())
            .is_some_and(|value| value.get("package").and_then(|p| p.as_table()).is_some());
        self.manifest_package_lookup.borrow_mut().insert(key, hit);
        hit
    }
}

impl Default for RustState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn expand_in_dir(dir: &Path, ctx: &WalkCtx) -> Vec<Batch<BatchKey>> {
    let mut out = expand_rust_files_in_dir_once(dir, ctx);
    if dir == ctx.root() {
        let source_dirs = ctx
            .rust_state()
            .cargo_source_dirs(|| collect_cargo_source_dirs(ctx.root(), ctx));
        for source_dir in source_dirs
            .iter()
            .filter(|source_dir| is_example_source_path(ctx.root(), source_dir))
        {
            out.extend(expand_rust_files_in_dir_once(source_dir, ctx));
        }
        for source_dir in source_dirs
            .iter()
            .filter(|source_dir| !is_example_source_path(ctx.root(), source_dir))
        {
            out.extend(gated_on_dir_listing(
                expand_rust_files_in_dir_once(source_dir, ctx),
                source_dir,
            ));
        }
    }
    out
}

fn expand_rust_files_in_dir_once(dir: &Path, ctx: &WalkCtx) -> Vec<Batch<BatchKey>> {
    if !ctx.rust_state().mark_dir_expanded(dir) {
        return Vec::new();
    }
    expand_rust_files_in_dir(dir, ctx)
}

fn expand_rust_files_in_dir(dir: &Path, ctx: &WalkCtx) -> Vec<Batch<BatchKey>> {
    let rust_files = files_with_extension(dir, "rs");
    if rust_files.is_empty() {
        return Vec::new();
    }
    let dir_depth = ctx.depth_from_root(dir);

    let mut out = Vec::new();

    for file in &rust_files {
        let ep = is_entrypoint_file(file);
        if ep {
            let lede_key = RustKey::CrateDocLede { file: file.clone() };
            let collect_section = |section| {
                move |tree: &Tree, source: &str| {
                    FileLines::new(collect_module_doc_lines(tree, source, section))
                }
            };
            let lede_emitted = if let Some(content) =
                build_per_file_content(file, ctx, parse_rust, collect_section(DocSection::Lede))
            {
                out.push(batch(
                    lede_key.clone(),
                    None,
                    content,
                    crate_doc_lede_value(file, ctx),
                ));
                true
            } else {
                false
            };
            if let Some(content) =
                build_per_file_content(file, ctx, parse_rust, collect_section(DocSection::Body))
            {
                // The lede batch is the predecessor only when it was
                // actually emitted — a crate doc that opens with a
                // heading has no lede, so the body would otherwise
                // orphan itself on a never-resolved predecessor key.
                let predecessor = lede_emitted.then_some(BatchKey::Rust(lede_key));
                out.push(batch(
                    RustKey::CrateDocBody { file: file.clone() },
                    predecessor,
                    content,
                    crate_doc_body_value(file, ctx),
                ));
            }
        }
        // ModUse stays gated to entrypoints + workspace members: `use`
        // plumbing of an arbitrary single-crate file is tiny-cost /
        // high-ratio noise that floods the mid-budget (measured: corpus
        // −0.0007, thiserror −0.045 when opened up). MethodSigs opens up
        // to every [package] source file — impl-method surface is the
        // only batch shape covering impl-dominated files (anyhow's
        // context.rs has zero other batches).
        if (ep || is_workspace_member_source_file(file, ctx))
            && let Some(content) = build_per_file_content(file, ctx, parse_rust, collect_mod_use)
        {
            let mod_decl_count = parse_rust(ctx, file)
                .map(|(source, tree)| count_top_level_mod_items(&tree, &source))
                .unwrap_or(0);
            out.push(batch(
                RustKey::ModUse { file: file.clone() },
                None,
                content,
                mod_use_value(file, ctx, mod_decl_count),
            ));
        }
        // MethodSigs covers every [package] source file — impl-method
        // surface is the only batch shape covering impl-dominated files
        // (anyhow's context.rs has zero other batches). Entrypoints keep
        // the full impl surface; other files render the exported API
        // surface only, which is the shape NS "method signatures
        // (locations)" rows take.
        if (ep || is_package_source_file(file, ctx))
            && let Some((source, tree)) = parse_rust(ctx, file)
        {
            let pub_traits = ctx
                .rust_state()
                .crate_pub_traits(|| collect_crate_pub_trait_names(ctx));
            let scope = if ep {
                MethodSigScope::All
            } else {
                MethodSigScope::ExportedOnly
            };
            let lines = collect_method_sigs(&tree, &source, scope, pub_traits);
            let method_count = count_exported_impl_methods(&tree, &source, pub_traits);
            if let Some(content) = single_file_lines_content(file, &source, lines) {
                out.push(batch(
                    RustKey::MethodSigs { file: file.clone() },
                    None,
                    content,
                    method_sigs_value(file, ctx, method_count),
                ));
            }
        }

        // Per-item pub declarations. Parse the file once; the cached tree
        // is shared with every per-item collector below.
        let Some((source, tree)) = parse_rust(ctx, file) else {
            continue;
        };
        let items = find_top_level_item_starts(&tree, &source, TopLevelItemVisibility::Public);
        let src_lines: Vec<&str> = source.lines().collect();
        if !items.is_empty() {
            // For a single-item file the names-surface batch (one line +
            // ellipsis) is redundant with the per-item batch that
            // follows: the agent gets the same name from either. Skip
            // the surface batch and let the lone PubItem stand on its
            // own with no predecessor.
            let emit_names_surface = items.len() > 1;
            let names_key = RustKey::PubItemNames { file: file.clone() };
            let parent_names_lines = collect_pub_item_names(&items);
            if emit_names_surface
                && let Some(content) =
                    single_file_lines_content(file, &source, parent_names_lines.clone())
            {
                out.push(batch(
                    names_key.clone(),
                    None,
                    content,
                    pub_item_names_value(file, ctx),
                ));
            }
            let names_predecessor = BatchKey::Rust(names_key);
            for item in &items {
                let pub_item_key = RustKey::PubItem {
                    file: file.clone(),
                    start_line: item.start_line,
                };
                let item_lines = collect_pub_item(item.node, &source);
                let parts = body_parts_for_item(item.node, &src_lines);
                // Pre-classify the doc-shape so empty Lede / empty Body
                // candidates aren't emitted. An empty Lede with a Body
                // predecessored on it would render the body unreachable.
                let raw_doc = collect_pub_item_doc_raw(item.node, &source);
                let lede_lines =
                    split_doc_lines_at_first_heading(raw_doc.clone(), &source, DocSection::Lede);
                let body_lines =
                    split_doc_lines_at_first_heading(raw_doc.clone(), &source, DocSection::Body);
                let item_has_descendants =
                    !parts.is_empty() || !lede_lines.is_empty() || !body_lines.is_empty();
                if (!file_lines_covered_by(&item_lines, &parent_names_lines)
                    || item_has_descendants)
                    && let Some(content) = single_file_lines_content(file, &source, item_lines)
                {
                    let predecessor = if emit_names_surface {
                        Some(names_predecessor.clone())
                    } else {
                        None
                    };
                    out.push(batch(
                        pub_item_key.clone(),
                        predecessor,
                        content,
                        pub_item_value(file, item.kind, item.surface, ctx),
                    ));
                }
                let item_key = BatchKey::Rust(pub_item_key.clone());
                let part_value_factor = body_part_value_factor(parts.len());
                for part in parts {
                    let Some(body_start_line) = part.start_line() else {
                        continue;
                    };
                    let Some(content) =
                        single_file_lines_content(file, &source, FileLines::new(part.lines))
                    else {
                        continue;
                    };
                    out.push(batch(
                        RustKey::PubItemBody {
                            file: file.clone(),
                            start_line: item.start_line,
                            body_start_line,
                        },
                        Some(item_key.clone()),
                        content,
                        pub_item_body_value(file, item.kind, item.surface, ctx) * part_value_factor,
                    ));
                }
                let mut lede_emitted: Option<BatchKey> = None;
                if !lede_lines.is_empty() {
                    let lede_key = RustKey::PubItemDocLede {
                        file: file.clone(),
                        start_line: item.start_line,
                    };
                    if let Some(content) = single_file_lines_content(
                        file,
                        &source,
                        collect_pub_item_doc_section(raw_doc.clone(), &source, DocSection::Lede),
                    ) {
                        out.push(batch(
                            lede_key.clone(),
                            Some(item_key.clone()),
                            content,
                            pub_item_doc_lede_value(file, item.kind, item.surface, ctx),
                        ));
                        lede_emitted = Some(BatchKey::Rust(lede_key));
                    }
                }
                if !body_lines.is_empty()
                    && let Some(content) = single_file_lines_content(
                        file,
                        &source,
                        collect_pub_item_doc_section(raw_doc.clone(), &source, DocSection::Body),
                    )
                {
                    let body_key = RustKey::PubItemDocBody {
                        file: file.clone(),
                        start_line: item.start_line,
                    };
                    out.push(batch(
                        body_key,
                        Some(lede_emitted.unwrap_or(item_key)),
                        content,
                        pub_item_doc_body_value(file, item.kind, item.surface, ctx),
                    ));
                }
            }
        }

        let example_entry = is_example_source_path(ctx.root(), file);
        let example_main_entry = example_entry && is_main_rs(file);
        let src_main_entry = is_src_main_file(file);
        if is_entrypoint_file(file) {
            let entry_items =
                find_top_level_item_starts(&tree, &source, TopLevelItemVisibility::Private);
            // Thin-`fn main` wrapper pattern: src/main.rs's `fn main` body
            // is just `match run() { ... }` (or similar error-shim), with
            // the real call-graph living in a private `fn run`/etc. NS
            // authors regularly anchor on the wrapped fn's body, but the
            // walker only emits `fn main` itself, leaving the wrapper's
            // body unscheduled. When `fn main` is thin, treat every
            // private top-level fn in the file as a peer entry item so
            // the wrapped body becomes schedulable like a Tokio
            // `fn main` body would be. Examples already get this peer
            // treatment via `example_main_entry`; the thin-wrapper case
            // is the bin-crate analog.
            let thin_main_wrapper = src_main_entry
                && entry_items.iter().any(|item| {
                    matches!(item.kind, ItemKind::Fn)
                        && name_of(item.node, &source) == Some("main")
                        && is_thin_main_body(item.node)
                });
            if !entry_items.is_empty() {
                for item in &entry_items {
                    if !should_emit_private_entry_item(
                        item.node,
                        &source,
                        example_main_entry,
                        src_main_entry,
                        thin_main_wrapper,
                    ) {
                        continue;
                    }
                    let entry_item_key = RustKey::EntryItem {
                        file: file.clone(),
                        start_line: item.start_line,
                    };
                    // Example main.rs walkthrough flow: render the main
                    // fn as signature + body parts (like src/main.rs) so
                    // tutorial steps inside the body schedule as peer
                    // anchors. Only README-cited examples get this
                    // treatment — those are the canonical "Quick Start"
                    // demos NS authors anchor on. Non-cited example
                    // main.rs stays as a whole-item batch under the
                    // general non-essential discount.
                    let is_main_fn = matches!(item.kind, ItemKind::Fn)
                        && name_of(item.node, &source) == Some("main");
                    let body_split_example_main =
                        example_main_entry && is_main_fn && ctx.is_readme_cited(file);
                    let render_whole = example_entry && !body_split_example_main;
                    let entry_lines = collect_private_entry_item(item.node, &source, render_whole);
                    if let Some(content) = single_file_lines_content(file, &source, entry_lines) {
                        out.push(batch(
                            entry_item_key.clone(),
                            None,
                            content,
                            entry_item_value(file, item.kind, item.surface, ctx),
                        ));
                    }
                    if example_entry && !body_split_example_main {
                        continue;
                    }
                    let item_key = BatchKey::Rust(entry_item_key);
                    let mut parts = body_parts_for_item(item.node, &src_lines);
                    // A README-cited *example* main (note: its parent dir
                    // is `src`, so `src_main_entry` is also true — check
                    // this case first) is a usage demo whose body is often
                    // a long run of trivial `let ..; println!(..)`
                    // statements (e.g. toasty's hello-toasty: ~44 of them).
                    // At full value these flood the schedule with dozens of
                    // equally-valued small batches that bury orientation
                    // content (crates/ listing, ARCHITECTURE outline, docs
                    // tree). Cap the tail into one trailing chunk and apply
                    // a gentle sqrt decay so the opening steps still anchor
                    // while the body stops out-massing the rest of the repo.
                    //
                    // A real bin `src/main.rs` (not under examples/) keeps
                    // full value: its top-level statements are distinct
                    // tutorial-step state that NS authors anchor on as
                    // consecutive ranges (e.g. sps NS 1.6-1.10 split main
                    // into 5 sections, hyperfine's run() into halves).
                    let part_value_factor = if body_split_example_main {
                        parts = coalesce_body_parts_tail(parts, ENTRY_BODY_PART_CAP);
                        entry_body_part_value_factor(parts.len())
                    } else if src_main_entry {
                        1.0
                    } else {
                        body_part_value_factor(parts.len())
                    };
                    for part in parts {
                        let Some(body_start_line) = part.start_line() else {
                            continue;
                        };
                        let Some(content) =
                            single_file_lines_content(file, &source, FileLines::new(part.lines))
                        else {
                            continue;
                        };
                        out.push(batch(
                            RustKey::EntryItemBody {
                                file: file.clone(),
                                start_line: item.start_line,
                                body_start_line,
                            },
                            Some(item_key.clone()),
                            content,
                            entry_item_body_value(file, item.kind, item.surface, ctx)
                                * part_value_factor,
                        ));
                    }
                }
            }
        }
    }

    // Cross-file macro batches, scoped to `dir` (non-recursive). Discovery
    // intentionally lives outside the per-file pub-item loop above —
    // `find_top_level_item_starts` skips `macro_definition` nodes, and the loop's
    // early `continue` on empty pub items would silently skip macro-only
    // files like `tests/fixtures/log/src/macros.rs`.
    let macro_names_key = RustKey::MacroNames {
        src_dir: dir.to_path_buf(),
    };
    let mut macro_name_spans: Vec<Span> = Vec::new();
    for file in &rust_files {
        let Some((source, tree)) = parse_rust(ctx, file) else {
            continue;
        };
        macro_name_spans.extend(build_file_spans(
            file,
            &source,
            collect_macro_name_lines(&tree, &source),
        ));
    }
    if !macro_name_spans.is_empty() {
        out.push(batch(
            macro_names_key.clone(),
            None,
            BatchContent::Lines {
                spans: macro_name_spans,
            },
            macro_names_value(dir_depth),
        ));
    }
    let macro_predecessor = BatchKey::Rust(macro_names_key);
    for file in &rust_files {
        let Some((source, tree)) = parse_rust(ctx, file) else {
            continue;
        };
        for info in find_macro_starts(&tree, &source) {
            if let Some(content) = single_file_lines_content(
                file,
                &source,
                collect_macro_body_at(&tree, &source, info.start_line),
            ) {
                out.push(batch(
                    RustKey::MacroBody {
                        file: file.clone(),
                        start_line: info.start_line,
                    },
                    Some(macro_predecessor.clone()),
                    content,
                    macro_body_value(file, &info, ctx),
                ));
            }
        }
    }

    out
}

fn gated_on_dir_listing(mut batches: Vec<Batch<BatchKey>>, dir: &Path) -> Vec<Batch<BatchKey>> {
    let gate = BatchKey::Fs(super::FsKey::DirListing {
        dir: dir.to_path_buf(),
    });
    for batch in &mut batches {
        if batch.predecessor.is_none() {
            batch.predecessor = Some(gate.clone());
        }
    }
    batches
}

/// Top-level pub item kind. Used for value weighting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ItemKind {
    Trait,
    Enum,
    Struct,
    Union,
    Fn,
    TypeAlias,
    Const,
    Static,
}

impl ItemKind {
    fn kind_weight(self) -> f64 {
        match self {
            ItemKind::Trait => 1.15,
            ItemKind::Enum => 1.1,
            ItemKind::Struct | ItemKind::Union => 1.0,
            ItemKind::Fn => 0.95,
            ItemKind::TypeAlias | ItemKind::Const | ItemKind::Static => 0.85,
        }
    }
}

#[derive(Debug, Clone)]
struct PubItemInfo<'a> {
    node: Node<'a>,
    start_line: usize,
    kind: ItemKind,
    surface: ApiSurface,
}

/// Whether a syntactically-public item is part of the external crate
/// API. `Restricted` (`pub(crate)`/`pub(super)`/`pub(in ...)`) and
/// `doc_hidden` each apply their own non-API factor.
#[derive(Debug, Clone, Copy)]
struct ApiSurface {
    visibility: Visibility,
    doc_hidden: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Visibility {
    Public,
    Restricted,
}

impl ApiSurface {
    /// Multiplier for non-API items — 0.4 per axis (visibility,
    /// doc_hidden); stacks to 0.16 for the doubly-internal class.
    fn factor(self) -> f64 {
        let v = match self.visibility {
            Visibility::Public => 1.0,
            Visibility::Restricted => 0.4,
        };
        let h = if self.doc_hidden { 0.4 } else { 1.0 };
        v * h
    }
}

fn item_kind_of(node: Node) -> Option<ItemKind> {
    Some(match node.kind() {
        "trait_item" => ItemKind::Trait,
        "enum_item" => ItemKind::Enum,
        "struct_item" => ItemKind::Struct,
        "union_item" => ItemKind::Union,
        "function_item" | "function_signature_item" => ItemKind::Fn,
        "type_item" => ItemKind::TypeAlias,
        "const_item" => ItemKind::Const,
        "static_item" => ItemKind::Static,
        _ => return None,
    })
}

#[derive(Debug, Clone, Copy)]
enum TopLevelItemVisibility {
    Public,
    Private,
}

fn find_top_level_item_starts<'a>(
    tree: &'a Tree,
    source: &str,
    visibility_filter: TopLevelItemVisibility,
) -> Vec<PubItemInfo<'a>> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut out = Vec::new();
    for child in root.children(&mut cursor) {
        if child.kind() == "macro_definition"
            && (matches!(visibility_filter, TopLevelItemVisibility::Private)
                || has_macro_export(child, source))
        {
            continue;
        }
        let visibility = match (visibility_filter, item_visibility(child, source)) {
            (TopLevelItemVisibility::Public, Some(visibility)) => visibility,
            (TopLevelItemVisibility::Private, None) => Visibility::Restricted,
            _ => continue,
        };
        let Some(kind) = item_kind_of(child) else {
            continue;
        };
        let start_line = child.start_position().row + 1;
        out.push(PubItemInfo {
            node: child,
            start_line,
            kind,
            surface: ApiSurface {
                visibility,
                doc_hidden: has_doc_hidden(child, source),
            },
        });
    }
    out
}

/// Files whose filename signals "crate entrypoint / main module surface".
fn is_entrypoint_file(path: &Path) -> bool {
    path.file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|n| matches!(n, "lib.rs" | "main.rs" | "mod.rs"))
}

fn should_emit_private_entry_item(
    node: Node,
    source: &str,
    example_main_entry: bool,
    src_main_entry: bool,
    thin_main_wrapper: bool,
) -> bool {
    if example_main_entry {
        return true;
    }
    if !matches!(item_kind_of(node), Some(ItemKind::Fn)) {
        return false;
    }
    if src_main_entry && name_of(node, source) == Some("main") {
        return true;
    }
    // Peer entries inside a thin-`fn main` wrapper: every private
    // top-level fn in src/main.rs (the wrapped real-main and any
    // co-resident helpers) becomes a schedulable entry item.
    // `thin_main_wrapper` already implies `src_main_entry` at the
    // call site, so no further guard is needed here.
    thin_main_wrapper
}

/// `fn main` is a thin wrapper when its body is just an error-handling
/// shim (`run().unwrap()`, `std::process::exit(real_main())`, ...). Cap
/// at 2 statements to avoid reclassifying a nontrivial `main`.
const THIN_MAIN_MAX_STATEMENTS: usize = 2;

fn is_thin_main_body(node: Node) -> bool {
    let Some(body) = node.child_by_field_name("body") else {
        return false;
    };
    let mut cursor = body.walk();
    let stmt_count = body
        .named_children(&mut cursor)
        .filter(|child| child.kind() != "line_comment" && child.kind() != "block_comment")
        .count();
    stmt_count <= THIN_MAIN_MAX_STATEMENTS
}

fn is_src_main_file(path: &Path) -> bool {
    is_main_rs(path)
        && path
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            == Some("src")
}

fn is_main_rs(path: &Path) -> bool {
    path.file_name().and_then(|n| n.to_str()) == Some("main.rs")
}

fn is_workspace_member_source_file(file: &Path, ctx: &WalkCtx) -> bool {
    ctx.rust_state()
        .nearest_member_dir(file, ctx.root())
        .is_some()
}

/// `true` iff `file` sits under a `Cargo.toml` carrying a `[package]`
/// table — its crate's source tree, whether or not the crate is a
/// workspace member. Plain single-crate repos have no `[workspace]`
/// table, so the workspace-membership check alone would leave every
/// non-entrypoint file without an impl-method surface.
fn is_package_source_file(file: &Path, ctx: &WalkCtx) -> bool {
    let mut dir = file.parent();
    while let Some(current) = dir {
        if !current.starts_with(ctx.root()) {
            break;
        }
        let manifest = current.join("Cargo.toml");
        if manifest.is_file() {
            return ctx.rust_state().manifest_has_package(&manifest, ctx);
        }
        dir = current.parent();
    }
    false
}

fn entrypoint_boost(path: &Path) -> f64 {
    let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
        return 1.0;
    };
    match name {
        "lib.rs" | "main.rs" => 1.4,
        "mod.rs" => 1.15,
        _ => 1.0,
    }
}

/// Damp non-entrypoint files in secondary workspace members so the
/// primary crate's entrypoints + sub-crates' lib.rs/main.rs/mod.rs
/// anchors win the early budget over per-file signature sweeps.
const SECONDARY_WORKSPACE_MEMBER_FACTOR: f64 = 0.7;

fn rust_depth_factor(file: &Path, ctx: &WalkCtx) -> f64 {
    let ep = is_entrypoint_file(file);
    let base = file_depth_factor(file, ctx, ep);
    if ep {
        base
    } else {
        base * secondary_workspace_member_factor(file, ctx)
    }
}

fn secondary_workspace_member_factor(file: &Path, ctx: &WalkCtx) -> f64 {
    let Some(manifest_dir) = ctx.rust_state().nearest_member_dir(file, ctx.root()) else {
        return 1.0;
    };
    let same_basename = ctx
        .root()
        .file_name()
        .zip(manifest_dir.file_name())
        .is_some_and(|(r, m)| r == m);
    if same_basename {
        1.0
    } else {
        SECONDARY_WORKSPACE_MEMBER_FACTOR
    }
}

fn crate_doc_lede_value(file: &Path, ctx: &WalkCtx) -> f64 {
    // Secondary workspace members (mdbook-html, mdbook-driver, etc.)
    // also emit a crate-doc lede from their lib.rs / main.rs even
    // though `secondary_workspace_member_factor` doesn't apply (those
    // files *are* entrypoints). NS authors rarely anchor on each
    // sub-crate's lede; damp the lede on secondary members so the
    // primary crate's lede + structural ARCHITECTURE rows compete
    // first in budget.
    let secondary = secondary_workspace_member_factor(file, ctx);
    let cat = (0.8 * entrypoint_boost(file)).min(1.0);
    mix_signals(cat, 0.5, 0.9, rust_depth_factor(file, ctx)) * secondary
}

fn crate_doc_body_value(file: &Path, ctx: &WalkCtx) -> f64 {
    let secondary = secondary_workspace_member_factor(file, ctx);
    let cat = (0.35 * entrypoint_boost(file)).min(1.0);
    mix_signals(cat, 0.6, 0.75, rust_depth_factor(file, ctx)) * secondary
}

fn mod_use_value(file: &Path, ctx: &WalkCtx, mod_decl_count: usize) -> f64 {
    // Crate entrypoints (lib.rs / main.rs) with ≥3 `mod foo;` top-level
    // declarations carry the crate's module table — the canonical list
    // of every top-level submodule. NS authors anchor on this list as
    // the crate's "module declarations" row; it's the structural analog
    // of `PubItemNames` but at the whole-crate level. The `≥3` gate
    // excludes use-heavy entrypoints (e.g. a workspace's primary
    // `main.rs` that's mostly `use` of sibling crates plus one or two
    // `mod`s); for those, the ModUse batch is plumbing-shaped and
    // anchoring on its prefix slot displaces NS-anchored content
    // elsewhere in the workspace. `mod.rs` and non-entrypoints stay
    // unboosted regardless of mod count.
    let is_crate_entry = file
        .file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|n| n == "lib.rs" || n == "main.rs");
    let crate_module_table = is_crate_entry && mod_decl_count >= 3;
    let cat_axis = if crate_module_table { 0.65 } else { 0.42 };
    let cat = (cat_axis * entrypoint_boost(file)).min(1.0);
    mix_signals(cat, 0.65, 0.38, rust_depth_factor(file, ctx))
}

/// Count top-level `mod_item` nodes (the `mod foo;` submodule table).
fn count_top_level_mod_items(tree: &Tree, _source: &str) -> usize {
    let root = tree.root_node();
    let mut cursor = root.walk();
    root.children(&mut cursor)
        .filter(|child| child.kind() == "mod_item")
        .count()
}

fn pub_item_names_value(file: &Path, ctx: &WalkCtx) -> f64 {
    // Cheap surface listing — catastrophic-omission hedge. Ranks high
    // because missing it means the agent doesn't know items exist.
    // File-level visibility applies the same axis as `ApiSurface::factor`
    // (0.4 for Restricted) — a names listing of items that aren't on the
    // public API is structurally less valuable to the agent.
    // Secondary workspace members' entrypoint files (lib.rs/main.rs/
    // mod.rs) bypass the non-entrypoint depth damp; apply the same
    // 0.7 secondary factor here so a workspace with N sub-crates
    // doesn't flood the early budget with N per-crate name surfaces.
    let s = file_visibility_factor(file, ctx);
    let secondary = secondary_workspace_member_factor(file, ctx);
    let cat = (0.8 * entrypoint_boost(file) * s).min(1.0);
    mix_signals(cat, 0.6 * s, 0.35 * s, rust_depth_factor(file, ctx)) * secondary
}

fn pub_item_value(file: &Path, kind: ItemKind, surface: ApiSurface, ctx: &WalkCtx) -> f64 {
    let k = kind.kind_weight();
    let s = effective_surface(surface, ctx, file).factor();
    let cat = (0.70 * k * s * entrypoint_boost(file)).min(1.0);
    let fu = (0.85 * k * s).min(1.0);
    mix_signals(cat, fu, 0.65 * s, rust_depth_factor(file, ctx))
}

fn pub_item_body_value(file: &Path, kind: ItemKind, surface: ApiSurface, ctx: &WalkCtx) -> f64 {
    let k = kind.kind_weight();
    let s = effective_surface(surface, ctx, file).factor();
    let body_axis = if matches!(kind, ItemKind::Fn) {
        0.95
    } else {
        0.75
    };
    let cat = (0.45 * k * body_axis * s * entrypoint_boost(file)).min(1.0);
    let fu = (0.80 * k * body_axis * s).min(1.0);
    mix_signals(cat, fu, 0.70 * body_axis * s, rust_depth_factor(file, ctx))
}

fn entry_item_value(file: &Path, kind: ItemKind, surface: ApiSurface, ctx: &WalkCtx) -> f64 {
    let k = kind.kind_weight();
    let s = if surface.doc_hidden { 0.4 } else { 1.0 };
    // Example entry items are usage flows, so they intentionally bypass the
    // generic examples/ non-essential discount while still getting depth pinning.
    let body_axis = if matches!(kind, ItemKind::Fn) {
        0.85
    } else {
        1.0
    };
    let cat = (0.50 * k * body_axis * s * entrypoint_boost(file)).min(1.0);
    let fu = (0.75 * k * body_axis * s).min(1.0);
    let depth = depth_factor(ctx.depth_from_root(file).min(1));
    mix_signals(cat, fu, 0.70 * body_axis * s, depth)
}

fn entry_item_body_value(file: &Path, kind: ItemKind, surface: ApiSurface, ctx: &WalkCtx) -> f64 {
    let k = kind.kind_weight();
    let s = if surface.doc_hidden { 0.4 } else { 1.0 };
    let body_axis = if matches!(kind, ItemKind::Fn) {
        1.0
    } else {
        0.75
    };
    let cat = (0.55 * k * body_axis * s * entrypoint_boost(file)).min(1.0);
    let fu = (0.85 * k * body_axis * s).min(1.0);
    let depth = depth_factor(ctx.depth_from_root(file).min(1));
    mix_signals(cat, fu, 0.75 * body_axis * s, depth)
}

fn pub_item_doc_lede_value(file: &Path, kind: ItemKind, surface: ApiSurface, ctx: &WalkCtx) -> f64 {
    let k = kind.kind_weight();
    let s = effective_surface(surface, ctx, file).factor();
    let cat = (0.20 * k * s * entrypoint_boost(file)).min(1.0);
    let fu = (0.6 * k * s).min(1.0);
    mix_signals(cat, fu, 0.8 * s, rust_depth_factor(file, ctx))
}

/// Body weights are a strict refinement of the lede — body rarely
/// adds catastrophic info, follow-up stays close (examples save tool
/// calls), ztu drops since example walls add marginal understanding.
fn pub_item_doc_body_value(file: &Path, kind: ItemKind, surface: ApiSurface, ctx: &WalkCtx) -> f64 {
    let k = kind.kind_weight();
    let s = effective_surface(surface, ctx, file).factor();
    let cat = (0.10 * k * s * entrypoint_boost(file)).min(1.0);
    let fu = (0.55 * k * s).min(1.0);
    mix_signals(cat, fu, 0.55 * s, rust_depth_factor(file, ctx))
}

/// Combine the per-item `ApiSurface` with the file's `module_visibility`.
fn effective_surface(local: ApiSurface, ctx: &WalkCtx, file: &Path) -> ApiSurface {
    let visibility = match (local.visibility, module_visibility(ctx, file)) {
        (Visibility::Public, Visibility::Public) => Visibility::Public,
        _ => Visibility::Restricted,
    };
    ApiSurface {
        visibility,
        doc_hidden: local.doc_hidden,
    }
}

fn file_visibility_factor(file: &Path, ctx: &WalkCtx) -> f64 {
    match module_visibility(ctx, file) {
        Visibility::Public => 1.0,
        Visibility::Restricted => 0.4,
    }
}

/// Exported-method count at which a file's impl surface stops being
/// plumbing and becomes the API roster NS authors anchor "method
/// signatures (locations)" rows on.
const METHOD_ROSTER_MIN: usize = 4;

fn method_sigs_value(file: &Path, ctx: &WalkCtx, exported_method_count: usize) -> f64 {
    let (cat, fu, ztu) = if is_entrypoint_file(file) {
        ((0.5 * entrypoint_boost(file)).min(1.0), 0.8, 0.4)
    } else if exported_method_count >= METHOD_ROSTER_MIN {
        // Impl-heavy API file: the method-sig surface is the file's
        // primary API partition, not a secondary follow-up.
        (0.6, 0.7, 0.35)
    } else {
        (0.25, 0.45, 0.25)
    };
    // Method-signature surfaces are rosters: per-batch value is
    // otherwise size-invariant while cost grows with method count, so
    // impl-heavy API files (the ones NS authors anchor "method
    // signatures" rows on) lose every ratio race to trivial two-method
    // files. Same neutralizer as dir listings / names surfaces.
    mix_signals(cat, fu, ztu, rust_depth_factor(file, ctx))
        * crate::value::roster_mass_factor(exported_method_count)
}

fn macro_names_value(depth: usize) -> f64 {
    mix_signals(0.75, 0.6, 0.4, depth_factor(depth))
}

/// Per-macro signals. Demotion axes stack multiplicatively:
/// `__`-prefixed name → 0.4, `#[doc(hidden)]` → 0.4.
fn macro_body_value(file: &Path, info: &MacroInfo, ctx: &WalkCtx) -> f64 {
    let underscore = if info.underscore_private { 0.4 } else { 1.0 };
    let doc_hidden = if info.doc_hidden { 0.4 } else { 1.0 };
    let axis = underscore * doc_hidden;
    let cat = (0.50 * axis * entrypoint_boost(file)).min(1.0);
    let fu = (0.70 * axis).min(1.0);
    mix_signals(cat, fu, 0.55 * axis, rust_depth_factor(file, ctx))
}

// --- parser ---

fn parse_rust(ctx: &WalkCtx, path: &Path) -> Option<(Arc<str>, Arc<Tree>)> {
    ctx.parse_tree(path, &tree_sitter_rust::LANGUAGE.into())
}

fn collect_cargo_source_dirs(root: &Path, ctx: &WalkCtx) -> Vec<PathBuf> {
    let mut manifests: Vec<PathBuf> = ctx
        .rust_state()
        .workspace_members(root)
        .iter()
        .cloned()
        .collect();
    let root_manifest = root.join("Cargo.toml");
    if ctx.rust_state().manifest_has_package(&root_manifest, ctx)
        && let Ok(canonical) = root_manifest.canonicalize()
    {
        manifests.push(canonical);
    }
    manifests.sort();
    manifests.dedup();

    let mut dirs = Vec::new();
    for manifest in manifests {
        if !ctx.rust_state().manifest_has_package(&manifest, ctx) {
            continue;
        }
        let Some(package_root) = manifest.parent() else {
            continue;
        };
        for source_root in ["src", "tests", "benches", "examples"] {
            dirs.extend(rust_parent_dirs_under(&package_root.join(source_root)));
        }
        if package_root.join("build.rs").is_file() {
            dirs.push(package_root.to_path_buf());
        }
    }
    dirs.sort();
    dirs.dedup();
    dirs
}

fn is_example_source_path(root: &Path, path: &Path) -> bool {
    path.strip_prefix(root)
        .unwrap_or(path)
        .components()
        .any(|c| {
            c.as_os_str()
                .to_str()
                .is_some_and(|s| matches!(s, "examples" | "example"))
        })
}

fn rust_parent_dirs_under(dir: &Path) -> Vec<PathBuf> {
    if !dir.exists() {
        return Vec::new();
    }
    let mut dirs = Vec::new();
    for file in super::fs::files_with_extension_recursive(dir, "rs") {
        if let Some(parent) = file.parent() {
            dirs.push(parent.to_path_buf());
        }
    }
    dirs.sort();
    dirs.dedup();
    dirs
}

// --- per-batch content builders ---

fn batch(
    key: RustKey,
    predecessor: Option<BatchKey>,
    content: BatchContent,
    value: f64,
) -> Batch<BatchKey> {
    Batch {
        key: key.into(),
        predecessor,
        content,
        value,
    }
}

// --- AST collectors ---

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DocSection {
    /// First paragraph of `//!` only.
    Lede,
    /// Everything after the first paragraph.
    Body,
}

/// Crate-`//!` block lines, split at the first markdown heading
/// (lede vs body). Doctest-hidden `# …` lines are stripped before the
/// split so they can't capture the heading boundary.
fn collect_module_doc_lines(tree: &Tree, source: &str, section: DocSection) -> Vec<usize> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut all: Vec<usize> = Vec::new();
    for child in root.children(&mut cursor) {
        if matches!(child.kind(), "line_comment" | "block_comment") {
            let text = &source[child.start_byte()..child.end_byte()];
            if text.starts_with("//!") || text.starts_with("/*!") {
                extend_span(&mut all, child, source);
            }
            continue;
        }
        break;
    }
    split_doc_lines_at_first_heading(all, source, section)
}

/// Shared rustdoc heading-split — strip hidden doctest lines, then
/// partition at the first ATX heading.
fn split_doc_lines_at_first_heading(
    lines: Vec<usize>,
    source: &str,
    section: DocSection,
) -> Vec<usize> {
    if lines.is_empty() {
        return Vec::new();
    }
    let stripped = strip_hidden_doctest_lines(lines, source);
    let src_lines: Vec<&str> = source.lines().collect();
    let heading_pos = stripped.iter().position(|&n| {
        src_lines
            .get(n - 1)
            .and_then(|raw| normalize_rustdoc_line(raw))
            .is_some_and(is_doc_atx_heading)
    });
    match (section, heading_pos) {
        (DocSection::Lede, Some(idx)) => stripped[..idx].to_vec(),
        (DocSection::Body, Some(idx)) => stripped[idx..].to_vec(),
        (DocSection::Lede, None) => stripped,
        (DocSection::Body, None) => Vec::new(),
    }
}

/// Strip the rustdoc marker from a raw source line. Returns `None`
/// for purely structural lines (`/**`/`/*!` openers with no body, the
/// `*/` closer, lone `*` continuation).
///
/// Prefixes (longest match first):
///   - `///` / `//!` (with one optional space)
///   - `/**` / `/*!` openers (None if remainder is whitespace-only)
///   - ` * ` / ` *` block-doc continuation (any leading whitespace
///     tolerated; one optional space after the `*` consumed).
///   - ` */` closer → `None`.
fn normalize_rustdoc_line(raw: &str) -> Option<&str> {
    let trimmed = raw.trim_start();
    if let Some(rest) = trimmed.strip_prefix("///") {
        return Some(rest.strip_prefix(' ').unwrap_or(rest));
    }
    if let Some(rest) = trimmed.strip_prefix("//!") {
        return Some(rest.strip_prefix(' ').unwrap_or(rest));
    }
    if let Some(rest) = trimmed.strip_prefix("/**") {
        let body = rest.strip_prefix(' ').unwrap_or(rest);
        if body.trim().is_empty() {
            return None;
        }
        return Some(body);
    }
    if let Some(rest) = trimmed.strip_prefix("/*!") {
        let body = rest.strip_prefix(' ').unwrap_or(rest);
        if body.trim().is_empty() {
            return None;
        }
        return Some(body);
    }
    if trimmed.starts_with("*/") {
        return None;
    }
    if let Some(rest) = trimmed.strip_prefix('*') {
        let body = rest.strip_prefix(' ').unwrap_or(rest);
        if body.trim().is_empty() {
            return None;
        }
        return Some(body);
    }
    None
}

/// CommonMark ATX heading on normalized rustdoc content — 0-3 spaces,
/// 1-6 `#`, then EOL or whitespace. Rejects `#!`/`#[` (rust attrs).
fn is_doc_atx_heading(content: &str) -> bool {
    // Leading indent: tolerate up to 3 spaces (CommonMark).
    let mut indent = 0usize;
    for b in content.bytes() {
        if b == b' ' && indent < 3 {
            indent += 1;
        } else {
            break;
        }
    }
    let after_indent = &content[indent..];
    let mut hashes = 0usize;
    for b in after_indent.bytes() {
        if b == b'#' && hashes < 7 {
            hashes += 1;
        } else {
            break;
        }
    }
    if !(1..=6).contains(&hashes) {
        return false;
    }
    let after_hashes = &after_indent[hashes..];
    matches!(after_hashes.bytes().next(), None | Some(b' ' | b'\t'))
}

/// Strip rustdoc doctest-hidden lines (`# …` inside a Rust fenced code
/// block) — rustdoc itself removes them from rendered HTML. Operates
/// on a sorted, contiguous-block list of 1-based source line numbers.
fn strip_hidden_doctest_lines(lines: Vec<usize>, source: &str) -> Vec<usize> {
    if lines.is_empty() {
        return lines;
    }
    let src_lines: Vec<&str> = source.lines().collect();

    let mut state = FenceState::Outside;
    let mut out = Vec::with_capacity(lines.len());

    for n in lines {
        let Some(raw) = src_lines.get(n.saturating_sub(1)) else {
            out.push(n);
            continue;
        };
        let Some(content) = normalize_rustdoc_line(raw) else {
            out.push(n);
            continue;
        };

        if let Some((kind, info)) = open_fence(content) {
            state = match state {
                FenceState::Outside => FenceState::Inside {
                    kind,
                    hides: is_rust_lang(info),
                },
                FenceState::Inside { kind: open, .. } if open == kind => FenceState::Outside,
                // Mismatched-kind fence inside another fence: rustdoc treats
                // this as literal content. Stay in current state.
                other => other,
            };
            out.push(n);
            continue;
        }

        if let FenceState::Inside { hides: true, .. } = state {
            let trimmed = content.trim_start();
            if trimmed == "#" || trimmed.starts_with("# ") {
                continue;
            }
        }
        out.push(n);
    }
    out
}

#[derive(Copy, Clone, PartialEq)]
enum FenceKind {
    Backtick,
    Tilde,
}

#[derive(Copy, Clone, PartialEq)]
enum FenceState {
    Outside,
    Inside { kind: FenceKind, hides: bool },
}

/// `Some((kind, info_string))` if `content` opens or closes a fence.
fn open_fence(content: &str) -> Option<(FenceKind, &str)> {
    let trimmed = content.trim_start();
    if let Some(rest) = trimmed.strip_prefix("```") {
        Some((FenceKind::Backtick, rest))
    } else if let Some(rest) = trimmed.strip_prefix("~~~") {
        Some((FenceKind::Tilde, rest))
    } else {
        None
    }
}

/// True iff the fence info string identifies a Rust block. Empty
/// defaults to Rust (rustdoc convention).
fn is_rust_lang(info: &str) -> bool {
    let token = info
        .trim()
        .split([',', ' ', '\t'])
        .next()
        .unwrap_or("")
        .trim();
    if token.is_empty() {
        return true;
    }
    const RUST_ATTRS: &[&str] = &[
        "rust",
        "no_run",
        "ignore",
        "compile_fail",
        "should_panic",
        "edition2015",
        "edition2018",
        "edition2021",
        "edition2024",
    ];
    RUST_ATTRS.iter().any(|a| token.eq_ignore_ascii_case(a))
}

fn collect_mod_use(tree: &Tree, _source: &str) -> FileLines {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut full = Vec::new();
    let mut ellipses = Vec::new();
    for child in root.children(&mut cursor) {
        match child.kind() {
            "use_declaration" => push_rows(
                &mut full,
                child.start_position().row,
                signature_end_row(child),
            ),
            "mod_item" => {
                let sig_end = signature_end_row(child);
                push_rows(&mut full, child.start_position().row, sig_end);
                if child.child_by_field_name("body").is_some() {
                    ellipses.push(sig_end + 2);
                }
            }
            _ => {}
        }
    }
    FileLines::new(dedup_sorted(full)).with_ellipses(dedup_sorted(ellipses))
}

/// Header lines for every top-level pub item (name + first line only, with
/// an ellipsis marker where the body would be). Surface listing — see
/// `PubItemNames`.
fn collect_pub_item_names(items: &[PubItemInfo<'_>]) -> FileLines {
    let mut full = Vec::new();
    let mut ellipses = Vec::new();
    for item in items {
        full.push(item.start_line);
        ellipses.push(item.start_line + 1);
    }
    FileLines::new(full).with_ellipses(ellipses)
}

/// Lines for a single pub item's decl at `start_line`. For struct/enum/
/// trait/union: whole item (fields/variants/method sigs). For fn:
/// signature with body-elision marker. No outer rustdoc — that's
/// `PubItemDocLede` and `PubItemDocBody`.
fn collect_pub_item(child: Node, source: &str) -> FileLines {
    if item_visibility(child, source).is_none() {
        return FileLines::new(Vec::new());
    }
    collect_item_lines(child, source, false)
}

fn collect_private_entry_item(child: Node, source: &str, whole: bool) -> FileLines {
    if item_kind_of(child).is_none() || item_visibility(child, source).is_some() {
        return FileLines::new(Vec::new());
    }
    collect_item_lines(child, source, whole)
}

fn collect_item_lines(child: Node, source: &str, whole: bool) -> FileLines {
    if whole {
        let mut full = Vec::new();
        // Include preceding outer `#[…]` attributes so the rendered span
        // matches NS rows that anchor on lines starting at the attribute
        // (e.g. toasty NS 2.1 wants `#[derive(toasty::Model)]` + `struct
        // User { … }` together as the User-model anchor).
        let mut cur = child.prev_sibling();
        while let Some(prev) = cur {
            if prev.kind() == "attribute_item" {
                extend_span(&mut full, prev, source);
                cur = prev.prev_sibling();
            } else {
                break;
            }
        }
        extend_span(&mut full, child, source);
        return FileLines::new(dedup_sorted(full));
    }
    let mut full = Vec::new();
    let mut ellipses = Vec::new();
    match child.kind() {
        "function_item" | "function_signature_item" => {
            let sig_end = signature_end_row(child);
            push_rows(&mut full, child.start_position().row, sig_end);
            if child.child_by_field_name("body").is_some() {
                ellipses.push(sig_end + 2);
            }
        }
        "struct_item" | "enum_item" | "trait_item" | "union_item" | "type_item" | "const_item"
        | "static_item" => {
            extend_span(&mut full, child, source);
        }
        _ => {}
    }
    FileLines::new(dedup_sorted(full)).with_ellipses(dedup_sorted(ellipses))
}

fn body_parts_for_item(child: Node, src_lines: &[&str]) -> Vec<BodyPart> {
    statement_block_parts(child.child_by_field_name("body"), src_lines, "block")
}

/// Lines of the lede or body section of the outer rustdoc preceding
/// the item at `start_line`. Both sections share the same heading-split
/// logic as the crate-level `//!` doc: doctest-hidden lines are stripped
/// first, then the remaining lines are partitioned at the first ATX
/// heading. Returns empty when the requested section is empty (note that
/// `expand` filters out empty Lede/Body candidates pre-emission, so a
/// scheduled key always produces at least one line in practice).
fn collect_pub_item_doc_section(raw: Vec<usize>, source: &str, section: DocSection) -> FileLines {
    FileLines::new(split_doc_lines_at_first_heading(raw, source, section))
}

fn collect_pub_item_doc_raw(child: Node, source: &str) -> Vec<usize> {
    collect_doc_comments_above_filtered(child, source, None, |prev, src| {
        let text = &src[prev.start_byte()..prev.end_byte()];
        matches!(prev.kind(), "line_comment" | "block_comment")
            && (text.starts_with("///") || text.starts_with("/**"))
    })
    .full
}

/// Which impl methods the `MethodSigs` surface renders.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MethodSigScope {
    /// Every impl header + method signature (entrypoint files).
    All,
    /// Exported API only: `pub fn`s of inherent impls plus all methods
    /// of impls of crate-public traits. Trait-integration plumbing
    /// (`Display` / `Debug` / sealed-trait blanket impls) is skipped —
    /// NS method-signature rows never anchor on it, and including it
    /// makes impl-heavy files' surfaces unaffordable.
    ExportedOnly,
}

/// Whether `method` (a fn inside `impl_node`'s body) is on the exported
/// API surface — see [`MethodSigScope::ExportedOnly`].
fn is_exported_method(
    impl_node: Node,
    method: Node,
    source: &str,
    pub_traits: &HashSet<String>,
) -> bool {
    match impl_trait_name(impl_node, source) {
        Some(trait_name) => pub_traits.contains(trait_name),
        None => matches!(item_visibility(method, source), Some(Visibility::Public)),
    }
}

/// Base name of the trait a trait-impl implements (`None` for inherent
/// impls). Strips generics and path qualifiers: `ser::Serialize<'a>` →
/// `Serialize`.
fn impl_trait_name<'a>(impl_node: Node, source: &'a str) -> Option<&'a str> {
    base_type_name(impl_node.child_by_field_name("trait")?, source)
}

fn base_type_name<'a>(node: Node, source: &'a str) -> Option<&'a str> {
    match node.kind() {
        "type_identifier" => Some(&source[node.start_byte()..node.end_byte()]),
        "generic_type" => base_type_name(node.child_by_field_name("type")?, source),
        "scoped_type_identifier" => base_type_name(node.child_by_field_name("name")?, source),
        _ => None,
    }
}

fn collect_method_sigs(
    tree: &Tree,
    source: &str,
    scope: MethodSigScope,
    pub_traits: &HashSet<String>,
) -> FileLines {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut full = Vec::new();
    let mut ellipses = Vec::new();
    for child in root.children(&mut cursor) {
        if child.kind() != "impl_item" {
            continue;
        }
        let Some(body) = child.child_by_field_name("body") else {
            continue;
        };
        let start_len = full.len();
        let mut added_method = false;
        // ExportedOnly renders one line per impl header / method — the
        // truncated-at-the-paren shape NS "method signatures
        // (locations)" rows take. Full multi-line signatures (params +
        // where-clauses) triple the cost without adding location info.
        let first_line_only = scope == MethodSigScope::ExportedOnly;
        let sig_end = if first_line_only {
            child.start_position().row
        } else {
            signature_end_row(child)
        };
        push_rows(&mut full, child.start_position().row, sig_end);
        let mut body_cursor = body.walk();
        for inner in body.children(&mut body_cursor) {
            if matches!(inner.kind(), "function_item" | "function_signature_item") {
                if first_line_only && !is_exported_method(child, inner, source, pub_traits) {
                    continue;
                }
                let inner_end = if first_line_only {
                    inner.start_position().row
                } else {
                    signature_end_row(inner)
                };
                push_rows(&mut full, inner.start_position().row, inner_end);
                added_method = true;
                if first_line_only || inner.child_by_field_name("body").is_some() {
                    ellipses.push(inner_end + 2);
                }
            }
        }
        if !added_method {
            full.truncate(start_len);
        }
    }
    FileLines::new(dedup_sorted(full)).with_ellipses(dedup_sorted(ellipses))
}

/// Top-level `pub trait` names (not doc-hidden) across every crate
/// source dir in the repo. Feeds [`is_exported_method`]'s trait-impl
/// check; memoized in [`RustState`].
fn collect_crate_pub_trait_names(ctx: &WalkCtx) -> HashSet<String> {
    let mut out = HashSet::new();
    let source_dirs = ctx
        .rust_state()
        .cargo_source_dirs(|| collect_cargo_source_dirs(ctx.root(), ctx))
        .clone();
    for dir in &source_dirs {
        for file in files_with_extension(dir, "rs") {
            let Some((source, tree)) = parse_rust(ctx, &file) else {
                continue;
            };
            let root = tree.root_node();
            let mut cursor = root.walk();
            for child in root.children(&mut cursor) {
                if child.kind() == "trait_item"
                    && matches!(item_visibility(child, &source), Some(Visibility::Public))
                    && !has_doc_hidden(child, &source)
                    && let Some(name) = name_of(child, &source)
                {
                    out.insert(name.to_string());
                }
            }
        }
    }
    out
}

/// Count the impl methods on the file's exported surface (the
/// [`MethodSigScope::ExportedOnly`] set). Feeds the roster tier and
/// roster-mass factor in [`method_sigs_value`].
fn count_exported_impl_methods(tree: &Tree, source: &str, pub_traits: &HashSet<String>) -> usize {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut count = 0;
    for child in root.children(&mut cursor) {
        if child.kind() != "impl_item" {
            continue;
        }
        let Some(body) = child.child_by_field_name("body") else {
            continue;
        };
        let mut body_cursor = body.walk();
        for inner in body.children(&mut body_cursor) {
            if matches!(inner.kind(), "function_item" | "function_signature_item")
                && is_exported_method(child, inner, source, pub_traits)
            {
                count += 1;
            }
        }
    }
    count
}

fn collect_macro_name_lines(tree: &Tree, source: &str) -> FileLines {
    let mut out = Vec::new();
    let mut ellipses = Vec::new();
    for_each_exported_macro(tree, source, |node, _name| {
        let row = node.start_position().row;
        push_rows(&mut out, row, row);
        ellipses.push(row + 2);
    });
    FileLines::new(out).with_ellipses(ellipses)
}

/// Per-macro orientation gathered at `expand` time so each `MacroBody`
/// candidate carries the inputs `macro_body_signals` needs without re-
/// parsing the file. Mirrors the [`PubItemInfo`] / [`ApiSurface`] split.
#[derive(Debug, Clone, Copy)]
struct MacroInfo {
    start_line: usize,
    underscore_private: bool,
    doc_hidden: bool,
}

fn find_macro_starts(tree: &Tree, source: &str) -> Vec<MacroInfo> {
    let mut out = Vec::new();
    for_each_exported_macro(tree, source, |node, name| {
        out.push(MacroInfo {
            start_line: node.start_position().row + 1,
            underscore_private: name.starts_with("__"),
            doc_hidden: has_doc_hidden(node, source),
        });
    });
    out
}

/// Read the `name` child (or fall back to first `identifier`) of a
/// `macro_definition` node and return its text.
fn macro_definition_name<'a>(node: Node, source: &'a str) -> Option<&'a str> {
    if let Some(name) = node.child_by_field_name("name") {
        return Some(&source[name.start_byte()..name.end_byte()]);
    }
    let mut cursor = node.walk();
    node.children(&mut cursor)
        .find(|c| c.kind() == "identifier")
        .map(|c| &source[c.start_byte()..c.end_byte()])
}

fn collect_macro_body_at(tree: &Tree, source: &str, start_line: usize) -> FileLines {
    let mut out = Vec::new();
    for_each_exported_macro(tree, source, |node, _| {
        if node.start_position().row + 1 == start_line {
            extend_span(&mut out, node, source);
        }
    });
    FileLines::new(dedup_sorted(out))
}

/// Walk every `#[macro_export] macro_rules!` definition at the file's
/// top level and call `f(node, name)`. Single source of truth for the
/// "exported macro definition" predicate that several helpers share.
fn for_each_exported_macro<F>(tree: &Tree, source: &str, mut f: F)
where
    F: FnMut(Node, &str),
{
    let root = tree.root_node();
    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        if child.kind() == "macro_definition"
            && has_macro_export(child, source)
            && let Some(name) = macro_definition_name(child, source)
        {
            f(child, name);
        }
    }
}

// --- Cross-file module visibility ---

/// Effective crate-visibility of `file` as a Rust module. `Public` if a
/// chain of `pub mod` declarations connects it back to `src/lib.rs`;
/// `Restricted` otherwise.
///
/// Fallback when `file` isn't in the computed map:
/// - empty map (no `<root>/src/lib.rs`, e.g. binary-only crate or
///   non-Rust seed) → `Public`. The local-syntactic check is the only
///   signal we have.
/// - non-empty map but `file` lives under `<root>/src/` →
///   `Restricted`. The resolver doesn't honor `#[path = ...]` or descend
///   into inline `pub mod` blocks with extern children, so a file
///   under `src/` not in the map is most plausibly behind one of those
///   gaps. Defaulting to `Restricted` keeps private-by-construction
///   items from sliding back to full public weight on a resolver miss.
/// - non-empty map, `file` outside `src/` (tests/, examples/, build.rs)
///   → `Public`. Those paths already get the non-essential-dir discount.
///
/// The map is computed lazily on first access and cached for the run.
/// Re-export tracking handles same-module `pub use self::path::...`
/// declarations: each declared-and-resolvable mod along the chain is
/// lifted to `Public` (gated segment-by-segment on a real `mod x;`
/// declaration in the current file). `pub use crate::...` and
/// `pub use super::...` paths are not modeled.
fn module_visibility(ctx: &WalkCtx, file: &Path) -> Visibility {
    let map = ctx
        .rust_state()
        .module_visibility_map(|| compute_module_visibility(ctx));
    if let Some(&v) = map.get(file) {
        return v;
    }
    if !map.is_empty() && file.starts_with(ctx.root().join("src")) {
        Visibility::Restricted
    } else {
        Visibility::Public
    }
}

fn compute_module_visibility(ctx: &WalkCtx) -> HashMap<PathBuf, Visibility> {
    let mut out: HashMap<PathBuf, Visibility> = HashMap::new();
    let lib = ctx.root().join("src/lib.rs");
    if !lib.exists() {
        return out;
    }
    let mut stack: Vec<(PathBuf, Visibility)> = vec![(lib, Visibility::Public)];
    while let Some((file, vis)) = stack.pop() {
        let upgrade = match out.get(&file).copied() {
            None => true,
            Some(Visibility::Public) => false, // already at the best vis
            Some(Visibility::Restricted) => matches!(vis, Visibility::Public),
        };
        if !upgrade {
            continue;
        }
        out.insert(file.clone(), vis);
        let Some((source, tree)) = parse_rust(ctx, &file) else {
            continue;
        };
        for decl in mod_decls(&tree, &source) {
            let child_vis = if decl.is_pub && matches!(vis, Visibility::Public) {
                Visibility::Public
            } else {
                Visibility::Restricted
            };
            let Some(child) = resolve_mod(&file, &decl.name) else {
                continue;
            };
            stack.push((child, child_vis));
        }
        if matches!(vis, Visibility::Public) {
            let parent_decls = mod_decls(&tree, &source);
            for path in pub_use_self_paths(&tree, &source) {
                lift_reexport_chain(&file, &parent_decls, &path, ctx, &mut stack);
            }
        }
    }
    out
}

/// Walk a `pub use self::seg1::seg2::...` re-export chain from
/// `parent_file`, lifting each declared-and-resolvable segment to
/// `Public`. Stops at the first segment that isn't a top-level `mod`
/// declaration in the current file. The mod-declaration gate keeps
/// chain-walked file lifts pinned to real `mod x;` decls — a same-named
/// file that exists for unrelated reasons (orphan, `#[path]`-mounted)
/// must not be promoted just because it's on disk.
fn lift_reexport_chain(
    parent_file: &Path,
    parent_decls: &[ModDecl],
    path: &[String],
    ctx: &WalkCtx,
    stack: &mut Vec<(PathBuf, Visibility)>,
) {
    let mut current = parent_file.to_path_buf();
    let mut decls: Vec<ModDecl> = parent_decls.to_vec();
    let mut first = true;
    for seg in path {
        if !first {
            let Some((src, tree)) = parse_rust(ctx, &current) else {
                return;
            };
            decls = mod_decls(&tree, &src);
        }
        first = false;
        if !decls.iter().any(|d| d.name == *seg) {
            return;
        }
        let Some(child) = resolve_mod(&current, seg) else {
            return;
        };
        stack.push((child.clone(), Visibility::Public));
        current = child;
    }
}

#[derive(Debug, Clone)]
struct ModDecl {
    name: String,
    is_pub: bool,
}

/// Top-level external `mod x;` declarations on this file. Inline
/// `mod x { ... }` blocks (with a body) are skipped — their items live in
/// the same source file and are subject to the per-item local syntactic
/// check, not cross-file resolution.
fn mod_decls(tree: &Tree, source: &str) -> Vec<ModDecl> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut out = Vec::new();
    for child in root.children(&mut cursor) {
        if child.kind() != "mod_item" {
            continue;
        }
        if child.child_by_field_name("body").is_some() {
            continue;
        }
        let Some(name_node) = child.child_by_field_name("name") else {
            continue;
        };
        let name = source[name_node.start_byte()..name_node.end_byte()].to_string();
        let is_pub = matches!(item_visibility(child, source), Some(Visibility::Public));
        out.push(ModDecl { name, is_pub });
    }
    out
}

/// Top-level `pub use self::...` re-exports as segment-paths *after*
/// the leading `self`. One declaration may yield multiple paths when
/// the use-clause is grouped or wildcarded
/// (`pub use self::{a::X, b::*}` → `[["a", "X"], ["b"]]`). Restricted
/// re-exports (`pub(crate) use`, no leading `self`) are skipped.
fn pub_use_self_paths(tree: &Tree, source: &str) -> Vec<Vec<String>> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut out = Vec::new();
    for child in root.children(&mut cursor) {
        if child.kind() != "use_declaration" {
            continue;
        }
        if !matches!(item_visibility(child, source), Some(Visibility::Public)) {
            continue;
        }
        let mut paths = Vec::new();
        if let Some(clause) = use_declaration_clause(child) {
            collect_use_paths(clause, source, &Vec::new(), &mut paths);
        }
        for p in paths {
            if p.first().is_some_and(|s| s == "self") && p.len() > 1 {
                out.push(p[1..].to_vec());
            }
        }
    }
    out
}

/// The argument clause of a `use_declaration` — the part after `pub?
/// use` and before the trailing `;`. Skips token children.
fn use_declaration_clause(use_decl: Node) -> Option<Node> {
    let mut cur = use_decl.walk();
    use_decl
        .children(&mut cur)
        .find(|c| !matches!(c.kind(), "visibility_modifier" | "use" | ";"))
}

/// Recursively expand a use-clause into one path per enumerated leaf.
/// `prefix` carries the segments accumulated from outer
/// `scoped_use_list` / `scoped_identifier` wrappers; each leaf appends
/// its own segments to `prefix` and pushes one entry into `out`.
fn collect_use_paths(node: Node, source: &str, prefix: &[String], out: &mut Vec<Vec<String>>) {
    match node.kind() {
        "scoped_use_list" => {
            let mut new_prefix = prefix.to_vec();
            let mut path_done = false;
            let mut cur = node.walk();
            for c in node.children(&mut cur) {
                if !path_done && append_path_segment(c, source, &mut new_prefix) {
                    path_done = true;
                } else if c.kind() == "use_list" {
                    expand_use_list(c, source, &new_prefix, out);
                }
            }
        }
        "use_wildcard" => {
            let mut new_prefix = prefix.to_vec();
            let mut cur = node.walk();
            for c in node.children(&mut cur) {
                append_path_segment(c, source, &mut new_prefix);
            }
            out.push(new_prefix);
        }
        "use_as_clause" => {
            let mut cur = node.walk();
            for c in node.children(&mut cur) {
                let mut segments = prefix.to_vec();
                if append_path_segment(c, source, &mut segments) {
                    out.push(segments);
                    return;
                }
            }
        }
        "use_list" => expand_use_list(node, source, prefix, out),
        _ => {
            let mut segments = prefix.to_vec();
            if append_path_segment(node, source, &mut segments) {
                out.push(segments);
            }
        }
    }
}

fn expand_use_list(node: Node, source: &str, prefix: &[String], out: &mut Vec<Vec<String>>) {
    let mut cur = node.walk();
    for item in node.children(&mut cur) {
        if matches!(item.kind(), "{" | "}" | ",") {
            continue;
        }
        collect_use_paths(item, source, prefix, out);
    }
}

/// Append a path-root node's segments to `out`. Handles the four leaf
/// shapes that can appear at the head of a use-clause: `scoped_identifier`
/// (recursed via [`flatten_scoped_id`]), the `self` / `crate` / `super`
/// path roots, and a bare `identifier`. Returns whether anything was
/// appended — callers use the return to track when a `scoped_use_list`'s
/// path prefix has been consumed.
fn append_path_segment(node: Node, source: &str, out: &mut Vec<String>) -> bool {
    match node.kind() {
        "scoped_identifier" => {
            flatten_scoped_id(node, source, out);
            true
        }
        "self" | "crate" | "super" => {
            out.push(node.kind().to_string());
            true
        }
        "identifier" => {
            out.push(source[node.start_byte()..node.end_byte()].to_string());
            true
        }
        _ => false,
    }
}

/// Flatten a `scoped_identifier` chain (`self::a::b::C` /
/// `a::b::C` / `crate::x`) into segment strings.
fn flatten_scoped_id(node: Node, source: &str, out: &mut Vec<String>) {
    let mut cur = node.walk();
    for c in node.children(&mut cur) {
        append_path_segment(c, source, out);
    }
}

/// Resolve a `mod x;` declaration in `parent_file` to the file that
/// `x` lives in, following Rust 2018+ module resolution. `#[path = ...]`
/// is not honored. Returns `None` if neither candidate exists on disk.
fn resolve_mod(parent_file: &Path, name: &str) -> Option<PathBuf> {
    let parent_dir = parent_file.parent()?;
    let lookup_dir = match parent_file.file_name().and_then(|n| n.to_str()) {
        Some("lib.rs" | "main.rs" | "mod.rs") => parent_dir.to_path_buf(),
        _ => {
            let stem = parent_file.file_stem()?.to_str()?;
            parent_dir.join(stem)
        }
    };
    let flat = lookup_dir.join(format!("{name}.rs"));
    if flat.exists() {
        return Some(flat);
    }
    let nested = lookup_dir.join(name).join("mod.rs");
    if nested.exists() {
        return Some(nested);
    }
    None
}

// --- AST predicates ---

/// Classify the visibility of a top-level item. `None` for items without
/// any `visibility_modifier` (i.e. private — these aren't `PubItem`
/// candidates). Plain `pub` → `Public`; any restricted form
/// (`pub(crate)` / `pub(super)` / `pub(self)` / `pub(in path)`) →
/// `Restricted`. Classification is by the trimmed text of the modifier
/// node, so future grammar additions to the restricted forms degrade
/// gracefully (anything that isn't exactly `pub` is treated as
/// restricted).
fn item_visibility(node: Node, source: &str) -> Option<Visibility> {
    let mut cursor = node.walk();
    let modifier = node
        .children(&mut cursor)
        .find(|c| c.kind() == "visibility_modifier")?;
    let text = source[modifier.start_byte()..modifier.end_byte()].trim();
    Some(if text == "pub" {
        Visibility::Public
    } else {
        Visibility::Restricted
    })
}

/// Walk the outer attributes attached to `node` (preceding siblings,
/// skipping comments) and return `true` if any of them satisfies `pred`.
/// `pred` receives the inner `attribute` node — the structured payload —
/// not the wrapping `attribute_item`. Stops at the first non-attr/
/// non-comment sibling.
fn any_outer_attribute<F>(node: Node, pred: F) -> bool
where
    F: Fn(Node) -> bool,
{
    let mut cur = node.prev_sibling();
    while let Some(prev) = cur {
        match prev.kind() {
            "attribute_item" => {
                let mut acur = prev.walk();
                if prev
                    .children(&mut acur)
                    .any(|c| c.kind() == "attribute" && pred(c))
                {
                    return true;
                }
                cur = prev.prev_sibling();
            }
            "line_comment" | "block_comment" => cur = prev.prev_sibling(),
            _ => break,
        }
    }
    false
}

/// Check whether `#[doc(hidden)]` is attached as a *direct* outer
/// attribute on `node`. Conditional forms like
/// `#[cfg_attr(..., doc(hidden))]` don't count — the outer attribute's
/// path there is `cfg_attr`, not `doc`. Treats conditionally-hidden
/// items as on-surface.
fn has_doc_hidden(node: Node, source: &str) -> bool {
    any_outer_attribute(node, |attr| is_doc_hidden_attribute(attr, source))
}

/// Match an `attribute` node whose path is `doc` and whose token-tree is
/// exactly `(hidden)`. Uses the structured AST instead of substring
/// matching so `cfg_attr(..., doc(hidden))` (whose outer path is
/// `cfg_attr`) doesn't false-fire.
fn is_doc_hidden_attribute(attr: Node, source: &str) -> bool {
    let mut cur = attr.walk();
    let mut children = attr.children(&mut cur);
    let path = children.next();
    let payload = children.next();
    let (Some(path), Some(payload)) = (path, payload) else {
        return false;
    };
    if path.kind() != "identifier" || &source[path.start_byte()..path.end_byte()] != "doc" {
        return false;
    }
    if payload.kind() != "token_tree" {
        return false;
    }
    let mut pcur = payload.walk();
    let idents: Vec<Node> = payload
        .children(&mut pcur)
        .filter(|c| c.kind() == "identifier")
        .collect();
    matches!(
        idents.as_slice(),
        [only] if &source[only.start_byte()..only.end_byte()] == "hidden"
    )
}

fn has_macro_export(node: Node, source: &str) -> bool {
    any_outer_attribute(node, |attr| {
        let mut cur = attr.walk();
        attr.children(&mut cur).any(|c| {
            c.kind() == "identifier" && &source[c.start_byte()..c.end_byte()] == "macro_export"
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tree_sitter::Parser;

    fn parse(src: &str) -> Tree {
        let mut p = Parser::new();
        p.set_language(&tree_sitter_rust::LANGUAGE.into()).unwrap();
        p.parse(src, None).unwrap()
    }

    fn surfaces(src: &str) -> Vec<(String, ApiSurface)> {
        let tree = parse(src);
        let mut out = Vec::new();
        let mut cursor = tree.root_node().walk();
        for child in tree.root_node().children(&mut cursor) {
            let Some(visibility) = item_visibility(child, src) else {
                continue;
            };
            let surface = ApiSurface {
                visibility,
                doc_hidden: has_doc_hidden(child, src),
            };
            let head = src[child.start_byte()..child.end_byte()]
                .lines()
                .next()
                .unwrap_or("")
                .to_string();
            out.push((head, surface));
        }
        out
    }

    fn write_vis_tree(files: &[(&str, &str)]) -> (TempDir, PathBuf) {
        let dir = tempdir();
        let src = dir.path().join("src");
        std::fs::create_dir_all(&src).unwrap();
        for (rel_path, contents) in files {
            let path = src.join(rel_path);
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent).unwrap();
            }
            std::fs::write(path, contents).unwrap();
        }
        (dir, src)
    }

    fn vis_map(files: &[(&str, &str)]) -> (TempDir, HashMap<PathBuf, Visibility>, PathBuf) {
        let (dir, src) = write_vis_tree(files);
        let ctx = WalkCtx::new(dir.path().to_path_buf());
        let map = compute_module_visibility(&ctx);
        (dir, map, src)
    }

    fn vis_map_ctx(
        files: &[(&str, &str)],
    ) -> (TempDir, WalkCtx, HashMap<PathBuf, Visibility>, PathBuf) {
        let (dir, src) = write_vis_tree(files);
        let ctx = WalkCtx::new(dir.path().to_path_buf());
        let map = compute_module_visibility(&ctx);
        (dir, ctx, map, src)
    }

    #[test]
    fn rust_visibility_classifies_pub_crate_super_self_in() {
        let src = r#"
pub struct A;
pub(crate) struct B;
pub(super) struct C;
pub(self) struct D;
pub(in crate::x) struct E;
struct F;
"#;
        let s = surfaces(src);
        let by_head: std::collections::HashMap<_, _> = s
            .iter()
            .map(|(h, surf)| (h.clone(), surf.visibility))
            .collect();
        assert_eq!(by_head["pub struct A;"], Visibility::Public);
        assert_eq!(by_head["pub(crate) struct B;"], Visibility::Restricted);
        assert_eq!(by_head["pub(super) struct C;"], Visibility::Restricted);
        assert_eq!(by_head["pub(self) struct D;"], Visibility::Restricted);
        assert_eq!(
            by_head["pub(in crate::x) struct E;"],
            Visibility::Restricted
        );
        // Private items have no visibility_modifier and are filtered out.
        assert!(!by_head.contains_key("struct F;"));
    }

    #[test]
    fn rust_doc_hidden_attaches_only_to_direct_doc_attribute() {
        let src = r#"
#[doc(hidden)]
pub struct A;

#[cfg_attr(feature = "f", doc(hidden))]
pub struct B;

#[doc = "regular doc"]
pub struct C;

#[doc(hidden)]
#[allow(dead_code)]
pub struct D;
"#;
        let surfaces = surfaces(src);
        let by_head: std::collections::HashMap<_, _> = surfaces
            .iter()
            .map(|(h, surf)| (h.clone(), surf.doc_hidden))
            .collect();
        // Direct #[doc(hidden)] fires.
        assert!(by_head["pub struct A;"]);
        // cfg_attr-wrapped doc(hidden) doesn't fire (outer attr is cfg_attr).
        assert!(!by_head["pub struct B;"]);
        // #[doc = "..."] doesn't fire.
        assert!(!by_head["pub struct C;"]);
        // doc(hidden) stacked with another attribute still fires.
        assert!(by_head["pub struct D;"]);
    }

    #[test]
    fn rust_api_surface_factor_composes_visibility_and_doc_hidden() {
        let pub_visible = ApiSurface {
            visibility: Visibility::Public,
            doc_hidden: false,
        };
        let pub_hidden = ApiSurface {
            visibility: Visibility::Public,
            doc_hidden: true,
        };
        let crate_visible = ApiSurface {
            visibility: Visibility::Restricted,
            doc_hidden: false,
        };
        let crate_hidden = ApiSurface {
            visibility: Visibility::Restricted,
            doc_hidden: true,
        };
        assert_eq!(pub_visible.factor(), 1.0);
        assert!((pub_hidden.factor() - 0.4).abs() < 1e-9);
        assert!((crate_visible.factor() - 0.4).abs() < 1e-9);
        assert!((crate_hidden.factor() - 0.16).abs() < 1e-9);
    }

    #[test]
    fn rust_mod_decls_extracts_top_level_external_modules() {
        let src = r#"
mod private_a;
pub mod public_b;
pub(crate) mod restricted_c;
mod inline_d { pub fn x() {} }
pub mod inline_e { }
"#;
        let tree = parse(src);
        let decls = mod_decls(&tree, src);
        let by_name: std::collections::HashMap<_, _> =
            decls.iter().map(|d| (d.name.clone(), d.is_pub)).collect();
        assert_eq!(by_name.get("private_a"), Some(&false));
        assert_eq!(by_name.get("public_b"), Some(&true));
        assert_eq!(by_name.get("restricted_c"), Some(&false)); // pub(crate) ≠ public surface
        // Inline mods (with body) are not external — their items live in
        // the same file and aren't subject to cross-file resolution.
        assert!(!by_name.contains_key("inline_d"));
        assert!(!by_name.contains_key("inline_e"));
    }

    #[test]
    fn rust_resolve_mod_2018_paths() {
        let dir = tempdir();
        let src_dir = dir.path().join("src");
        std::fs::create_dir_all(&src_dir).unwrap();
        std::fs::write(src_dir.join("lib.rs"), "").unwrap();
        std::fs::write(src_dir.join("flat.rs"), "").unwrap();
        std::fs::create_dir_all(src_dir.join("nested")).unwrap();
        std::fs::write(src_dir.join("nested").join("mod.rs"), "").unwrap();
        std::fs::create_dir_all(src_dir.join("flat")).unwrap();
        std::fs::write(src_dir.join("flat").join("child.rs"), "").unwrap();

        // From lib.rs: lookup dir is src/; finds flat.rs.
        let r = resolve_mod(&src_dir.join("lib.rs"), "flat").unwrap();
        assert_eq!(r, src_dir.join("flat.rs"));

        // From lib.rs: nested/mod.rs.
        let r = resolve_mod(&src_dir.join("lib.rs"), "nested").unwrap();
        assert_eq!(r, src_dir.join("nested").join("mod.rs"));

        // From flat.rs (a non-mod.rs file): lookup dir is src/flat/.
        let r = resolve_mod(&src_dir.join("flat.rs"), "child").unwrap();
        assert_eq!(r, src_dir.join("flat").join("child.rs"));

        // From nested/mod.rs: lookup dir is src/nested/, child not present.
        assert!(resolve_mod(&src_dir.join("nested").join("mod.rs"), "missing").is_none());
    }

    #[test]
    fn rust_compute_module_visibility_propagates_through_pub_mod_chain() {
        let (_dir, map, src) = vis_map(&[
            ("lib.rs", "pub mod public_chain;\nmod private_chain;\n"),
            ("public_chain.rs", "pub mod grandchild;\nmod gc_private;\n"),
            ("public_chain/grandchild.rs", ""),
            ("public_chain/gc_private.rs", ""),
            ("private_chain.rs", "pub mod gc_under_private;\n"),
            ("private_chain/gc_under_private.rs", ""),
        ]);

        assert_eq!(map.get(&src.join("lib.rs")), Some(&Visibility::Public));
        assert_eq!(
            map.get(&src.join("public_chain.rs")),
            Some(&Visibility::Public)
        );
        assert_eq!(
            map.get(&src.join("public_chain").join("grandchild.rs")),
            Some(&Visibility::Public)
        );
        // Private mod inside a public chain → Restricted.
        assert_eq!(
            map.get(&src.join("public_chain").join("gc_private.rs")),
            Some(&Visibility::Restricted)
        );
        // Private chain itself → Restricted.
        assert_eq!(
            map.get(&src.join("private_chain.rs")),
            Some(&Visibility::Restricted)
        );
        // pub mod inside a private chain doesn't lift visibility back to
        // Public — once Restricted, always Restricted on that path.
        assert_eq!(
            map.get(&src.join("private_chain").join("gc_under_private.rs")),
            Some(&Visibility::Restricted)
        );
    }

    #[test]
    fn rust_compute_module_visibility_skips_when_no_lib_rs() {
        let (_dir, map, _src) = vis_map(&[("main.rs", "mod helpers;\n"), ("helpers.rs", "")]);
        // No lib.rs → empty map → callers default to Public (preserve
        // local-syntactic-only behavior for binary-only crates).
        assert!(map.is_empty());
    }

    #[test]
    fn rust_module_visibility_falls_back_to_restricted_for_unresolved_files_under_src() {
        // Resolver miss simulator: a `#[path = "actual.rs"]` mod or a
        // private inline `pub mod x { mod y; }` puts files into `src/`
        // that the resolver doesn't follow. Defaulting these to Public
        // would silently restore full public-API weight; the safer
        // fallback is Restricted.
        // Map seeds from lib.rs, which only declares public_child.
        let (dir, ctx, _map, src) = vis_map_ctx(&[
            ("lib.rs", "pub mod public_child;\n"),
            ("public_child.rs", ""),
            // Simulate a #[path]-mounted file the resolver didn't follow.
            ("hidden_via_path_attr.rs", ""),
        ]);
        // Files outside src/ (tests/, examples/, build.rs) should still
        // default to Public — they get the non-essential-dir discount.
        std::fs::write(dir.path().join("build.rs"), "").unwrap();

        // Reachable via `pub mod` chain → Public.
        assert_eq!(
            module_visibility(&ctx, &src.join("public_child.rs")),
            Visibility::Public
        );
        // Resolver miss under src/ → Restricted (safer default).
        assert_eq!(
            module_visibility(&ctx, &src.join("hidden_via_path_attr.rs")),
            Visibility::Restricted
        );
        // Outside src/ → Public.
        assert_eq!(
            module_visibility(&ctx, &dir.path().join("build.rs")),
            Visibility::Public
        );
    }

    #[test]
    fn rust_module_visibility_no_lib_rs_defaults_public_everywhere() {
        // Binary-only crate: no `<root>/src/lib.rs` → empty map →
        // every file defaults to Public. Preserves the local-syntactic
        // check as the only signal.
        let (_dir, ctx, _map, src) = vis_map_ctx(&[("main.rs", ""), ("helpers.rs", "")]);
        assert_eq!(
            module_visibility(&ctx, &src.join("main.rs")),
            Visibility::Public
        );
        assert_eq!(
            module_visibility(&ctx, &src.join("helpers.rs")),
            Visibility::Public
        );
    }

    #[test]
    fn rust_pub_use_self_paths_handles_grammar_shapes() {
        let src = r#"
pub use self::error::Error;
pub use self::key::{Key, ToKey};
pub use self::value::*;
pub use self::source::Visitor as V;
pub use self::a::b::Deep;
pub use self::single_segment;
pub use self::{first_grp::X, second_grp::*};
pub use self::nested::{a::{x, y}, b};
pub use crate::foo::Bar;
pub use other::baz::Qux;
pub(crate) use self::restricted::Hidden;
use self::not_pub::Hidden;
"#;
        let tree = parse(src);
        let mut paths = pub_use_self_paths(&tree, src);
        paths.sort();
        let expected: Vec<Vec<String>> = [
            vec!["a", "b", "Deep"],
            vec!["error", "Error"],
            vec!["first_grp", "X"],
            vec!["key", "Key"],
            vec!["key", "ToKey"],
            vec!["nested", "a", "x"],
            vec!["nested", "a", "y"],
            vec!["nested", "b"],
            vec!["second_grp"],
            vec!["single_segment"],
            vec!["source", "Visitor"],
            vec!["value"],
        ]
        .into_iter()
        .map(|p| p.into_iter().map(String::from).collect())
        .collect();
        let mut expected_sorted = expected;
        expected_sorted.sort();
        assert_eq!(paths, expected_sorted);
    }

    #[test]
    fn rust_pub_use_self_lifts_private_child_to_public() {
        // log/src/kv/mod.rs shape — `mod x;` + `pub use self::x::Item;`
        // is the motivating pattern for cross-file re-export tracking.
        let (_dir, map, src) = vis_map(&[
            ("lib.rs", "pub mod kv;\n"),
            ("kv/mod.rs", "mod error;\npub use self::error::Error;\n"),
            ("kv/error.rs", "pub struct Error;\n"),
        ]);
        assert_eq!(
            map.get(&src.join("kv").join("error.rs")),
            Some(&Visibility::Public),
            "error.rs should be lifted to Public via pub use re-export",
        );
    }

    #[test]
    fn rust_pub_use_self_does_not_lift_when_parent_restricted() {
        // The parent's re-export only crosses to external API when the
        // parent itself is on the external surface.
        let (_dir, map, src) = vis_map(&[
            ("lib.rs", "mod parent;\n"),
            ("parent/mod.rs", "mod child;\npub use self::child::X;\n"),
            ("parent/child.rs", "pub struct X;\n"),
        ]);
        assert_eq!(
            map.get(&src.join("parent").join("child.rs")),
            Some(&Visibility::Restricted),
        );
    }

    #[test]
    fn rust_pub_use_self_handles_use_list_and_wildcard() {
        for re_export in ["pub use self::child::{A, B};", "pub use self::child::*;"] {
            let lib = format!("mod child;\n{re_export}\n");
            let (_dir, map, src) = vis_map(&[
                ("lib.rs", &lib),
                ("child.rs", "pub struct A; pub struct B;\n"),
            ]);
            assert_eq!(
                map.get(&src.join("child.rs")),
                Some(&Visibility::Public),
                "re-export `{re_export}` should lift child.rs to Public",
            );
        }
    }

    #[test]
    fn rust_pub_use_self_handles_grouped_under_self() {
        // Outer `scoped_use_list` whose path-prefix is the bare `self`
        // token (no `scoped_identifier`) — verifies the path_done logic.
        let (_dir, map, src) = vis_map(&[
            ("lib.rs", "mod a;\nmod b;\npub use self::{a::X, b::*};\n"),
            ("a.rs", "pub struct X;\n"),
            ("b.rs", "pub struct Y;\n"),
        ]);
        assert_eq!(map.get(&src.join("a.rs")), Some(&Visibility::Public));
        assert_eq!(map.get(&src.join("b.rs")), Some(&Visibility::Public));
    }

    #[test]
    fn rust_pub_use_self_handles_use_as_clause() {
        let (_dir, map, src) = vis_map(&[
            (
                "lib.rs",
                "mod child;\npub use self::child::Item as Renamed;\n",
            ),
            ("child.rs", "pub struct Item;\n"),
        ]);
        assert_eq!(map.get(&src.join("child.rs")), Some(&Visibility::Public),);
    }

    #[test]
    fn rust_pub_use_self_walks_chain_through_private_mods() {
        // The chain walks past private `mod b;` to lift b.rs because
        // `pub use self::a::b::X` makes `b`'s contents publicly reachable
        // even though `a.rs` only declares it privately.
        let (_dir, map, src) = vis_map(&[
            ("lib.rs", "mod a;\npub use self::a::b::X;\n"),
            ("a/mod.rs", "mod b;\n"),
            ("a/b.rs", "pub struct X;\n"),
        ]);
        assert_eq!(
            map.get(&src.join("a").join("mod.rs")),
            Some(&Visibility::Public),
            "a should be lifted",
        );
        assert_eq!(
            map.get(&src.join("a").join("b.rs")),
            Some(&Visibility::Public),
            "b should be lifted via chain walk through private mod",
        );
    }

    #[test]
    fn rust_pub_use_self_does_not_lift_undeclared_same_named_file() {
        // Without the `mod b;` gate, an orphan `a/b.rs` on disk would
        // be silently lifted by `pub use self::a::b::X;` even though
        // it isn't actually a child mod of `a`.
        let (_dir, ctx, map, src) = vis_map_ctx(&[
            ("lib.rs", "mod a;\npub use self::a::b::X;\n"),
            ("a/mod.rs", "// no mod b\n"),
            ("a/b.rs", "pub struct X;\n"),
        ]);
        assert_eq!(
            map.get(&src.join("a").join("mod.rs")),
            Some(&Visibility::Public),
        );
        assert!(!map.contains_key(&src.join("a").join("b.rs")));
        assert_eq!(
            module_visibility(&ctx, &src.join("a").join("b.rs")),
            Visibility::Restricted,
        );
    }

    #[test]
    fn rust_pub_use_without_self_prefix_does_not_lift() {
        let (_dir, map, src) = vis_map(&[
            (
                "lib.rs",
                "mod foo;\npub use crate::foo::Bar;\npub use my_crate::baz::Qux;\n",
            ),
            ("foo.rs", "pub struct Bar;\n"),
        ]);
        assert_eq!(map.get(&src.join("foo.rs")), Some(&Visibility::Restricted),);
    }

    #[test]
    fn rust_pub_use_with_inner_visibility_does_not_lift() {
        let (_dir, map, src) = vis_map(&[
            ("lib.rs", "mod child;\npub(crate) use self::child::X;\n"),
            ("child.rs", "pub struct X;\n"),
        ]);
        assert_eq!(
            map.get(&src.join("child.rs")),
            Some(&Visibility::Restricted),
        );
    }

    #[test]
    fn rust_compute_module_visibility_dual_decl_merges_to_public() {
        // log/src/kv/mod.rs pattern: cfg-gated `mod x;` + `pub mod x;`
        // pointing at the same file. We don't track cfg gates — taking
        // "any path makes it Public" is the conservative call (preserves
        // surface signal for the kv_unstable feature configuration).
        let (_dir, map, src) =
            vis_map(&[("lib.rs", "mod child;\npub mod child;\n"), ("child.rs", "")]);
        assert_eq!(map.get(&src.join("child.rs")), Some(&Visibility::Public));
    }

    /// A crate doc that opens with a heading has no lede paragraph, so
    /// no `CrateDocLede` batch is emitted; the `CrateDocBody` batch must
    /// then carry no predecessor rather than orphan on a missing key.
    #[test]
    fn rust_crate_doc_body_no_predecessor_when_heading_first() {
        let (dir, src) = write_vis_tree(&[(
            "lib.rs",
            "//! # Overview\n//!\n//! Body prose after the heading.\n\npub struct A;\n",
        )]);
        let ctx = WalkCtx::new(dir.path().to_path_buf());
        let batches = expand_in_dir(&src, &ctx);
        let body = batches
            .iter()
            .find(|b| matches!(&b.key, BatchKey::Rust(RustKey::CrateDocBody { .. })))
            .expect("CrateDocBody emitted");
        assert!(
            body.predecessor.is_none(),
            "no predecessor with heading-first doc"
        );
        assert!(
            !batches
                .iter()
                .any(|b| matches!(&b.key, BatchKey::Rust(RustKey::CrateDocLede { .. }))),
            "no CrateDocLede batch when the doc opens with a heading"
        );
    }

    /// Minimal scratch-dir helper. Avoids pulling in the `tempfile` crate
    /// for two tests; `process::id` keeps the path unique enough across
    /// concurrent test runs.
    fn tempdir() -> TempDir {
        let base = std::env::temp_dir().join(format!(
            "precis-rust-walker-test-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let _ = std::fs::remove_dir_all(&base);
        std::fs::create_dir_all(&base).unwrap();
        TempDir(base)
    }

    struct TempDir(PathBuf);
    impl TempDir {
        fn path(&self) -> &Path {
            &self.0
        }
    }
    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    // ---- strip_hidden_doctest_lines ----

    /// Run the filter on every non-blank line of `src` and return only the
    /// surviving lines, joined back into a string. Stable shape for
    /// snapshot-style assertions of multi-fence behavior.
    fn strip_via(src: &str) -> String {
        let all: Vec<usize> = (1..=src.lines().count()).collect();
        let kept = strip_hidden_doctest_lines(all, src);
        let lines: Vec<&str> = src.lines().collect();
        kept.into_iter()
            .map(|n| lines[n - 1])
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn rust_strip_hidden_lines_eq_table() {
        let cases = [
            (
                "default_fence",
                "//! ```\n//! # use anyhow::Result;\n//! fn ok() -> Result<()> { Ok(()) }\n//! ```\n",
                "//! ```\n//! fn ok() -> Result<()> { Ok(()) }\n//! ```",
            ),
            (
                "heading_outside_fence",
                "//! # Real heading\n//! prose\n",
                "//! # Real heading\n//! prose",
            ),
            // `## foo` inside a Rust fence is rendered (rustdoc consumes one #),
            // so the line must be kept.
            (
                "double_hash_in_rust_fence",
                "/// ```\n/// ## foo\n/// ```\n",
                "/// ```\n/// ## foo\n/// ```",
            ),
            // `#[derive(...)]` and `#![cfg(...)]` don't start with `# ` (no
            // space), so they're code, not hidden.
            (
                "attribute_lines_kept",
                "/// ```\n/// #[derive(Debug)]\n/// #![allow(unused)]\n/// struct S;\n/// ```\n",
                "/// ```\n/// #[derive(Debug)]\n/// #![allow(unused)]\n/// struct S;\n/// ```",
            ),
        ];
        for (case, src, expected) in cases {
            assert_eq!(strip_via(src), expected, "case `{case}`");
        }
    }

    #[test]
    fn rust_strip_hidden_lines_contains_table() {
        let cases = [
            (
                "lone_hash_dropped",
                "/// ```\n/// #\n/// real\n/// ```\n",
                &["/// #\n"][..],
                &["/// real"][..],
            ),
            (
                "tilde_fence",
                "/// ~~~\n/// # let x = 1;\n/// real\n/// ~~~\n",
                &["# let x"][..],
                &[][..],
            ),
            // Inside a backtick Rust fence, a `~~~` line is literal content (not
            // a closer). Subsequent `# ` lines stay hidden.
            (
                "mismatched_fence_kind_does_not_close",
                "/// ```\n/// # hidden 1\n/// ~~~ inline\n/// # hidden 2\n/// ```\n",
                &["# hidden 1", "# hidden 2"][..],
                &["~~~ inline"][..],
            ),
            // Lede: prose, then a Rust fence, then a console fence, then a
            // tilde-rust fence. Hidden lines in Rust fences only.
            (
                "multi_fence_state_machine",
                "\
//! intro
//! ```
//! # hidden a
//! visible a
//! ```
//! between
//! ```console
//! # kept-console
//! ```
//! ~~~rust
//! # hidden b
//! visible b
//! ~~~
//! tail
",
                &["# hidden a", "# hidden b"][..],
                &["# kept-console", "visible a", "visible b", "//! tail"][..],
            ),
            // Outer (`///`) and inner (`//!`) doc prefixes are stripped identically.
            (
                "works_with_outer_doc",
                "/// ```\n/// # hidden\n/// real\n/// ```\n",
                &["# hidden"][..],
                &[][..],
            ),
            (
                "works_with_inner_doc",
                "//! ```\n//! # hidden\n//! real\n//! ```\n",
                &["# hidden"][..],
                &[][..],
            ),
            // Block doc lines without a `*` continuation marker (rare but
            // syntactically allowed) normalize to `None` and pass through
            // — they aren't recognized as fence/heading candidates.
            (
                "block_doc_unmarked_lines_pass_through",
                "/** ```\n# would-be-hidden-but-not-handled\nreal\n``` */\n",
                &[][..],
                &["# would-be-hidden-but-not-handled"][..],
            ),
            // Block doc with proper `*` continuation: ` * # use ...` inside a
            // Rust fence is hidden scaffolding and must be stripped, just like
            // `/// # use ...` and `//! # use ...` are today.
            (
                "block_doc_strips_starred_hidden_inside_fence",
                "/**\n * ```\n * # use crate::X;\n * real\n * ```\n */\n",
                &["# use crate::X"][..],
                &["* real"][..],
            ),
        ];
        for (case, src, absent_needles, present_needles) in cases {
            let got = strip_via(src);
            for needle in absent_needles {
                assert!(
                    !got.contains(needle),
                    "case `{case}` should drop `{needle}`; got: {got}",
                );
            }
            for needle in present_needles {
                assert!(
                    got.contains(needle),
                    "case `{case}` should keep `{needle}`; got: {got}",
                );
            }
        }
    }

    #[test]
    fn rust_strip_hidden_lines_rust_attrs() {
        for attr in [
            "rust",
            "no_run",
            "ignore",
            "compile_fail",
            "should_panic",
            "edition2018",
            "edition2021",
        ] {
            let src = format!("/// ```{attr}\n/// # let x = 1;\n/// real\n/// ```\n");
            let got = strip_via(&src);
            assert!(
                !got.contains("# let x"),
                "fence `{attr}` should drop hidden line; got: {got}",
            );
            assert!(got.contains("/// real"));
        }
    }

    #[test]
    fn rust_strip_hidden_lines_non_rust_fence_keeps_hash() {
        for lang in ["console", "text", "toml", "bash", "json"] {
            let src = format!("/// ```{lang}\n/// # not hidden\n/// ```\n");
            let got = strip_via(&src);
            assert!(
                got.contains("# not hidden"),
                "fence `{lang}` should keep `# `; got: {got}",
            );
        }
    }

    #[test]
    fn rust_module_doc_split_handles_fence_before_first_real_heading() {
        // Crate doc opens with a fenced Rust example that has a `# ` hidden
        // line. The first *real* heading is `# Public API` further down. Split
        // must land on the real heading: lede gets the prose and intro fence;
        // body starts at `# Public API`.
        let src = "\
//! Crate one-liner.
//!
//! ```
//! # use crate::Foo;
//! Foo::bar();
//! ```
//!
//! # Public API
//!
//! Detail.

pub fn anchor() {}
";
        let tree = parse(src);
        let lede = collect_module_doc_lines(&tree, src, DocSection::Lede);
        let body = collect_module_doc_lines(&tree, src, DocSection::Body);
        let line_text = |n: usize| src.lines().nth(n - 1).unwrap_or("");

        // Lede contains the prose, the fence delimiters, and the visible
        // body line — but not the `# use crate::Foo;` doctest scaffolding.
        let lede_text: Vec<&str> = lede.iter().map(|&n| line_text(n)).collect();
        assert!(
            lede_text.iter().any(|t| t.contains("Crate one-liner")),
            "lede missing tagline: {lede_text:?}",
        );
        assert!(
            lede_text.iter().any(|t| t.contains("Foo::bar()")),
            "lede missing visible code line: {lede_text:?}",
        );
        assert!(
            !lede_text.iter().any(|t| t.contains("# use crate::Foo")),
            "lede leaked hidden doctest line: {lede_text:?}",
        );
        assert!(
            !lede_text.iter().any(|t| t.contains("# Public API")),
            "lede crossed the real heading: {lede_text:?}",
        );

        // Body starts at the real heading and contains the prose after it.
        let body_text: Vec<&str> = body.iter().map(|&n| line_text(n)).collect();
        assert!(
            body_text
                .first()
                .is_some_and(|t| t.contains("# Public API")),
            "body should start at real heading; got {body_text:?}",
        );
        assert!(body_text.iter().any(|t| t.contains("Detail.")));
    }

    #[test]
    fn rust_doc_atx_heading_predicate() {
        // 1–6 hashes with trailing space accepted.
        for n in 1..=6 {
            let s = format!("{} Heading", "#".repeat(n));
            assert!(is_doc_atx_heading(&s), "{n} hashes + space rejected: {s}");
        }
        // 1–6 hashes with EOL accepted.
        for n in 1..=6 {
            let s = "#".repeat(n);
            assert!(is_doc_atx_heading(&s), "{n} hashes + EOL rejected: {s}");
        }
        // 7 hashes rejected.
        assert!(!is_doc_atx_heading("####### too many"));
        // Attribute shapes rejected.
        assert!(!is_doc_atx_heading("#![feature(x)]"));
        assert!(!is_doc_atx_heading("#[derive(D)]"));
        // Hash without trailing space/tab/EOL rejected.
        assert!(!is_doc_atx_heading("#x"));
        // Tab after hashes accepted.
        assert!(is_doc_atx_heading("##\tHeading"));
        // CommonMark allows up to 3 leading spaces of indent.
        assert!(is_doc_atx_heading("   # Heading"));
        // 4 spaces of indent → code block, not a heading.
        assert!(!is_doc_atx_heading("    # Heading"));
    }

    fn collect_doc_section_lines(src: &str, section: DocSection) -> Vec<String> {
        let tree = parse(src);
        let item_node = tree
            .root_node()
            .children(&mut tree.root_node().walk())
            .find(|c| item_visibility(*c, src).is_some())
            .expect("test source must contain a pub item");
        let raw = collect_pub_item_doc_raw(item_node, src);
        let lines = collect_pub_item_doc_section(raw, src, section);
        let src_lines: Vec<&str> = src.lines().collect();
        lines
            .full
            .into_iter()
            .map(|n| src_lines[n - 1].to_string())
            .collect()
    }

    #[test]
    fn rust_pub_item_doc_split_table() {
        struct Case<'a> {
            name: &'a str,
            src: &'a str,
            lede_present: &'a [&'a str],
            lede_absent: &'a [&'a str],
            lede_empty: Option<bool>,
            body_first: Option<&'a str>,
            body_present: &'a [&'a str],
            body_empty: Option<bool>,
        }

        let cases = [
            Case {
                name: "lede_body_split_at_heading",
                src: "/// Summary line.\n/// More prose.\n///\n/// # Examples\n/// example()\npub fn foo() {}\n",
                lede_present: &["Summary line"],
                lede_absent: &["# Examples"],
                lede_empty: None,
                body_first: Some("# Examples"),
                body_present: &["example()"],
                body_empty: None,
            },
            Case {
                name: "no_heading_emits_only_lede",
                src: "/// Summary line.\n/// More prose.\npub fn foo() {}\n",
                lede_present: &[],
                lede_absent: &[],
                lede_empty: Some(false),
                body_first: None,
                body_present: &[],
                body_empty: Some(true),
            },
            // When the doc's first non-hidden line is already an ATX heading,
            // the lede is empty. `expand` must wire the Body's predecessor to
            // `PubItem` (not the absent Lede); see `walker::expand`.
            Case {
                name: "starts_with_heading_emits_only_body",
                src: "/// # Examples\n/// example()\npub fn foo() {}\n",
                lede_present: &[],
                lede_absent: &[],
                lede_empty: Some(true),
                body_first: None,
                body_present: &["# Examples"],
                body_empty: None,
            },
            // Hidden `# use ...` inside a Rust fence at the *start* of the
            // doc must not be classified as a heading.
            Case {
                name: "strips_hidden_doctest_before_split",
                src: "/// ```\n/// # use foo;\n/// real()\n/// ```\n///\n/// # Real Heading\n/// detail\npub fn foo() {}\n",
                lede_present: &[],
                lede_absent: &[],
                lede_empty: None,
                body_first: Some("# Real Heading"),
                body_present: &[],
                body_empty: None,
            },
            Case {
                name: "block_doc_split_at_starred_heading",
                src: "/**\n * Summary.\n *\n * # Examples\n * example()\n */\npub fn foo() {}\n",
                lede_present: &["Summary."],
                lede_absent: &["# Examples"],
                lede_empty: None,
                body_first: None,
                body_present: &["# Examples"],
                body_empty: None,
            },
            Case {
                name: "block_doc_strips_hidden_doctest_before_split",
                src: "/**\n * ```\n * # use crate::X;\n * real()\n * ```\n *\n * # Real Heading\n * detail\n */\npub fn foo() {}\n",
                lede_present: &[],
                lede_absent: &[],
                lede_empty: None,
                body_first: Some("# Real Heading"),
                body_present: &[],
                body_empty: None,
            },
        ];

        for case in cases {
            let lede = collect_doc_section_lines(case.src, DocSection::Lede);
            let body = collect_doc_section_lines(case.src, DocSection::Body);
            if let Some(empty) = case.lede_empty {
                assert_eq!(
                    lede.is_empty(),
                    empty,
                    "case `{}` lede: {lede:?}",
                    case.name
                );
            }
            if let Some(empty) = case.body_empty {
                assert_eq!(
                    body.is_empty(),
                    empty,
                    "case `{}` body: {body:?}",
                    case.name
                );
            }
            for needle in case.lede_present {
                assert!(
                    lede.iter().any(|l| l.contains(needle)),
                    "case `{}` lede missing `{needle}`: {lede:?}",
                    case.name,
                );
            }
            for needle in case.lede_absent {
                assert!(
                    !lede.iter().any(|l| l.contains(needle)),
                    "case `{}` lede unexpectedly contains `{needle}`: {lede:?}",
                    case.name,
                );
            }
            if let Some(needle) = case.body_first {
                assert!(
                    body.first().is_some_and(|l| l.contains(needle)),
                    "case `{}` body should start with `{needle}`: {body:?}",
                    case.name,
                );
            }
            for needle in case.body_present {
                assert!(
                    body.iter().any(|l| l.contains(needle)),
                    "case `{}` body missing `{needle}`: {body:?}",
                    case.name,
                );
            }
        }
    }

    #[test]
    fn rust_pub_item_doc_attribute_lines_are_not_headings() {
        // Visible `#![...]` and `#[...]` inside a doc fence must not be
        // mistaken for ATX headings. With no real heading, body is empty.
        let src = "/// Summary.\n/// ```\n/// #![feature(x)]\n/// #[derive(D)]\n/// real()\n/// ```\npub fn foo() {}\n";
        let lede = collect_doc_section_lines(src, DocSection::Lede);
        let body = collect_doc_section_lines(src, DocSection::Body);
        assert!(!lede.is_empty(), "lede should cover the doc: {lede:?}");
        assert!(body.is_empty(), "body should be empty: {body:?}");
    }
}
