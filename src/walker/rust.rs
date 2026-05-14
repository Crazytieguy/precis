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
    BodyPart, FileLines, WalkCtx, body_part_value_factor, build_file_spans, build_per_file_content,
    dedup_sorted, extend_span, file_depth_factor, file_lines_covered_by, fs::files_with_extension,
    name_of, push_rows, signature_end_row, single_file_lines_content, statement_block_parts,
};

/// Per-run Rust-walker state owned by [`WalkCtx`]. Stores cross-file
/// analyses that the Rust walker needs to memoize for a single run:
/// module visibility (which `.rs` files are reachable from `src/lib.rs`'s
/// `pub mod` graph), Cargo workspace membership, and per-directory
/// exported-macro-name sets. Pure storage — computation lives in the
/// walker's free functions and accesses state through [`WalkCtx::rust_state`].
pub struct RustState {
    module_visibility: OnceCell<HashMap<PathBuf, Visibility>>,
    exported_macros_per_dir: RefCell<HashMap<PathBuf, Arc<HashSet<String>>>>,
    workspace_members: OnceCell<HashSet<PathBuf>>,
    workspace_member_lookup: RefCell<HashMap<PathBuf, bool>>,
    nearest_member_dir_lookup: RefCell<HashMap<PathBuf, Option<PathBuf>>>,
    cargo_source_dirs: OnceCell<Vec<PathBuf>>,
    expanded_dirs: RefCell<HashSet<PathBuf>>,
    manifest_package_lookup: RefCell<HashMap<PathBuf, bool>>,
}

impl RustState {
    pub fn new() -> Self {
        Self {
            module_visibility: OnceCell::new(),
            exported_macros_per_dir: RefCell::new(HashMap::new()),
            workspace_members: OnceCell::new(),
            workspace_member_lookup: RefCell::new(HashMap::new()),
            nearest_member_dir_lookup: RefCell::new(HashMap::new()),
            cargo_source_dirs: OnceCell::new(),
            expanded_dirs: RefCell::new(HashSet::new()),
            manifest_package_lookup: RefCell::new(HashMap::new()),
        }
    }

    pub(in crate::walker) fn module_visibility_map(
        &self,
        init: impl FnOnce() -> HashMap<PathBuf, Visibility>,
    ) -> &HashMap<PathBuf, Visibility> {
        self.module_visibility.get_or_init(init)
    }

    pub(in crate::walker) fn exported_macros_in_dir(
        &self,
        dir: &Path,
        init: impl FnOnce() -> HashSet<String>,
    ) -> Arc<HashSet<String>> {
        if let Some(arc) = self.exported_macros_per_dir.borrow().get(dir) {
            return arc.clone();
        }
        let arc = Arc::new(init());
        self.exported_macros_per_dir
            .borrow_mut()
            .insert(dir.to_path_buf(), arc.clone());
        arc
    }

    /// `true` iff `file` is a `Cargo.toml` declared (or auto-promoted) as
    /// a workspace member by the seed-root `Cargo.toml`. Lookups are
    /// memoized to avoid one canonicalize syscall per signal computation.
    pub fn is_workspace_member(&self, file: &Path, root: &Path) -> bool {
        let members = self.workspace_members(root);
        if members.is_empty() {
            return false;
        }
        if let Some(&hit) = self.workspace_member_lookup.borrow().get(file) {
            return hit;
        }
        let hit = file
            .canonicalize()
            .map(|c| members.contains(&c))
            .unwrap_or(false);
        self.workspace_member_lookup
            .borrow_mut()
            .insert(file.to_path_buf(), hit);
        hit
    }

    /// Walk up from `file` to the nearest enclosing `Cargo.toml` and
    /// return its directory iff that manifest is a workspace member.
    /// Stops at the first manifest encountered — for a source file
    /// inside a non-member nested crate, returns `None` even when a
    /// member manifest exists further up. Memoized per file.
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
        self.workspace_members
            .get_or_init(|| super::toml::collect_workspace_members(root))
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
            if let Some(content) =
                build_per_file_content(file, ctx, parse_rust, collect_module_doc_lede)
            {
                out.push(batch(
                    lede_key.clone(),
                    None,
                    content,
                    crate_doc_lede_value(file, ctx),
                ));
            }
            if let Some(content) =
                build_per_file_content(file, ctx, parse_rust, collect_module_doc_body)
            {
                out.push(batch(
                    RustKey::CrateDocBody { file: file.clone() },
                    Some(BatchKey::Rust(lede_key)),
                    content,
                    crate_doc_body_value(file, ctx),
                ));
            }
        }
        if ep || is_workspace_member_source_file(file, ctx) {
            if let Some(content) = build_per_file_content(file, ctx, parse_rust, collect_mod_use) {
                out.push(batch(
                    RustKey::ModUse { file: file.clone() },
                    None,
                    content,
                    mod_use_value(file, ctx),
                ));
            }
            if let Some(content) =
                build_per_file_content(file, ctx, parse_rust, collect_method_sigs)
            {
                out.push(batch(
                    RustKey::MethodSigs { file: file.clone() },
                    None,
                    content,
                    method_sigs_value(file, ctx),
                ));
            }
        }

        // Per-item pub declarations. Parse the file once; the cached tree
        // is shared with every per-item collector below.
        let Some((source, tree)) = parse_rust(ctx, file) else {
            continue;
        };
        let items = find_pub_item_starts(&tree, &source);
        let src_lines: Vec<&str> = source.lines().collect();
        if !items.is_empty() {
            let names_key = RustKey::PubItemNames { file: file.clone() };
            let parent_names_lines = collect_pub_item_names(&items);
            if let Some(content) =
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
                    out.push(batch(
                        pub_item_key.clone(),
                        Some(names_predecessor.clone()),
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
            let entry_items = find_private_top_level_item_starts(&tree, &source);
            if !entry_items.is_empty() {
                for item in &entry_items {
                    if !should_emit_private_entry_item(
                        item.node,
                        &source,
                        example_main_entry,
                        src_main_entry,
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
                        && (name_of(item.node, &source) == Some("main")
                            || has_async_main_attribute(item.node, &source));
                    let body_split_example_main =
                        example_main_entry && is_main_fn && ctx.is_readme_cited(file);
                    let render_whole = example_entry && !body_split_example_main;
                    let entry_lines =
                        collect_private_entry_item(item.node, &source, render_whole);
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
                    let parts = body_parts_for_item(item.node, &src_lines);
                    // For src/main.rs entry bodies (single-bin entry
                    // points), each top-level statement is a distinct
                    // tutorial-step's worth of state; NS authors
                    // typically anchor on consecutive ranges of these
                    // statements (e.g. sps NS 1.6-1.10 split main.rs
                    // into 5 sections). Use a softer (sqrt) decay so
                    // peer statements stay competitive against
                    // orientation batches.
                    let part_value_factor = if src_main_entry || body_split_example_main {
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
    // `find_pub_item_starts` skips `macro_definition` nodes, and the loop's
    // early `continue` on empty pub items would silently skip macro-only
    // files like `tests/fixtures/log/src/macros.rs`.
    let exported_names = ctx
        .rust_state()
        .exported_macros_in_dir(dir, || compute_exported_macros(dir, ctx));
    let macro_names_key = RustKey::MacroNames {
        src_dir: dir.to_path_buf(),
    };
    if let Some(content) = build_cross_file_content(dir, ctx, collect_macro_name_lines) {
        out.push(batch(
            macro_names_key.clone(),
            None,
            content,
            macro_names_value(dir_depth),
        ));
    }
    let macro_predecessor = BatchKey::Rust(macro_names_key);
    for file in &rust_files {
        let Some((source, tree)) = parse_rust(ctx, file) else {
            continue;
        };
        for info in find_macro_starts(&tree, &source, &exported_names) {
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

/// Kind of a top-level pub item, used to weight its batch. Traits are the
/// load-bearing abstraction every backend implements; enums and structs
/// carry the data-model; free fns are the call surface. First-pass
/// ordering — calibrate against the north stars.
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

/// Whether a syntactically-public item is part of the external crate API.
/// `Public` (plain `pub`) competes for the budget at full weight;
/// `Restricted` (`pub(crate)` / `pub(super)` / `pub(self)` / `pub(in ...)`)
/// is compiler-visible inside the crate but not the external surface.
/// `doc_hidden` is orthogonal — public items can still be opted out of
/// rustdoc with `#[doc(hidden)]`. Together they produce a non-API factor
/// applied uniformly across the value channels in `pub_item_signals` /
/// `pub_item_doc_signals`.
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
    /// Multiplier applied to all three value channels for non-API items.
    /// Calibrated against the divergence score metric across the 10
    /// fixtures: 0.4 per axis (visibility, doc_hidden) demotes
    /// non-API items meaningfully without dropping load-bearing
    /// internal types out of the schedule entirely. 0.16 stacks for
    /// items that are *both* restricted and `#[doc(hidden)]` (the
    /// most clearly internal class).
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

fn find_pub_item_starts<'a>(tree: &'a Tree, source: &str) -> Vec<PubItemInfo<'a>> {
    find_top_level_item_starts(tree, source, TopLevelItemVisibility::Public)
}

fn find_private_top_level_item_starts<'a>(tree: &'a Tree, source: &str) -> Vec<PubItemInfo<'a>> {
    find_top_level_item_starts(tree, source, TopLevelItemVisibility::Private)
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
) -> bool {
    if example_main_entry {
        return true;
    }
    matches!(item_kind_of(node), Some(ItemKind::Fn))
        && (has_async_main_attribute(node, source)
            || (src_main_entry && name_of(node, source) == Some("main")))
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

fn has_async_main_attribute(node: Node, source: &str) -> bool {
    any_outer_attribute(node, |attr| {
        let text = &source[attr.start_byte()..attr.end_byte()];
        let compact: String = text.chars().filter(|c| !c.is_whitespace()).collect();
        compact.starts_with("tokio::main") || compact.starts_with("async_std::main")
    })
}

fn is_workspace_member_source_file(file: &Path, ctx: &WalkCtx) -> bool {
    ctx.rust_state()
        .nearest_member_dir(file, ctx.root())
        .is_some()
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

/// Multi-crate workspaces where the primary crate shares the repo
/// basename (`toasty/crates/toasty`, `sps/sps`, `mdbook/.`) tend to
/// have NS rows that anchor on each crate's `lib.rs` / `main.rs` /
/// `mod.rs` (module tree, re-exports) but not its deep per-file API
/// surface. Damping non-entrypoint files in secondary crates keeps
/// each crate's entrypoints competitive while moving the wide-but-
/// shallow per-file signature sweep further down the schedule.
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
    let secondary = secondary_workspace_member_member_factor(file, ctx);
    let cat = (0.8 * entrypoint_boost(file)).min(1.0);
    mix_signals(cat, 0.5, 0.9, rust_depth_factor(file, ctx)) * secondary
}

fn secondary_workspace_member_member_factor(file: &Path, ctx: &WalkCtx) -> f64 {
    let Some(manifest_dir) = ctx.rust_state().nearest_member_dir(file, ctx.root()) else {
        return 1.0;
    };
    let same_basename = ctx
        .root()
        .file_name()
        .zip(manifest_dir.file_name())
        .is_some_and(|(r, m)| r == m);
    if same_basename { 1.0 } else { 0.7 }
}

fn crate_doc_body_value(file: &Path, ctx: &WalkCtx) -> f64 {
    let secondary = secondary_workspace_member_member_factor(file, ctx);
    let cat = (0.35 * entrypoint_boost(file)).min(1.0);
    mix_signals(cat, 0.6, 0.75, rust_depth_factor(file, ctx)) * secondary
}

fn mod_use_value(file: &Path, ctx: &WalkCtx) -> f64 {
    let cat = (0.42 * entrypoint_boost(file)).min(1.0);
    mix_signals(cat, 0.65, 0.38, rust_depth_factor(file, ctx))
}

fn pub_item_names_value(file: &Path, ctx: &WalkCtx) -> f64 {
    // Cheap surface listing — catastrophic-omission hedge. Ranks high
    // because missing it means the agent doesn't know items exist.
    // File-level visibility applies the same axis as `ApiSurface::factor`
    // (0.4 for Restricted) — a names listing of items that aren't on the
    // public API is structurally less valuable to the agent.
    let s = file_visibility_factor(file, ctx);
    let cat = (0.8 * entrypoint_boost(file) * s).min(1.0);
    mix_signals(cat, 0.6 * s, 0.35 * s, rust_depth_factor(file, ctx))
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

/// Body weights are a strict refinement of the lede: body rarely adds
/// catastrophic info beyond what the lede already covered (so halve
/// catastrophic), follow-up stays close to the lede's value because
/// `# Examples` does save tool calls (0.55 vs lede 0.6), and ztu
/// drops to 0.55 since example walls add only marginal understanding
/// over the lede prose.
fn pub_item_doc_body_value(file: &Path, kind: ItemKind, surface: ApiSurface, ctx: &WalkCtx) -> f64 {
    let k = kind.kind_weight();
    let s = effective_surface(surface, ctx, file).factor();
    let cat = (0.10 * k * s * entrypoint_boost(file)).min(1.0);
    let fu = (0.55 * k * s).min(1.0);
    mix_signals(cat, fu, 0.55 * s, rust_depth_factor(file, ctx))
}

/// Combine a per-item local `ApiSurface` with the file's effective
/// crate-visibility from `module_visibility`. `doc_hidden` passes through.
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

fn method_sigs_value(file: &Path, ctx: &WalkCtx) -> f64 {
    let (cat, fu, ztu) = if is_entrypoint_file(file) {
        ((0.5 * entrypoint_boost(file)).min(1.0), 0.8, 0.4)
    } else {
        (0.25, 0.45, 0.25)
    };
    mix_signals(cat, fu, ztu, rust_depth_factor(file, ctx))
}

fn macro_names_value(depth: usize) -> f64 {
    mix_signals(0.75, 0.6, 0.4, depth_factor(depth))
}

/// Per-macro signals. Three demotion axes stack multiplicatively:
///
/// - `__`-prefixed name → 0.4 axis (`pub(crate)` analog)
/// - `#[doc(hidden)]` outer attribute → 0.4 axis (mirrors
///   [`ApiSurface::factor`])
/// - wrapper macro (body invokes another `#[macro_export]` sibling
///   via `$crate::<name>!`) → 0.6 axis. The wrapper axis is what
///   makes `log!` (root) outrank `error!`/`warn!`/etc. (which all
///   delegate to `$crate::log!`); without it, smaller wrapper
///   bodies win on cost concavity alone and the larger root never
///   fits at the budget tail.
///
/// A doubly-internal macro (`__-prefixed` + `#[doc(hidden)]`) lands
/// at axis = 0.16 — the same floor `ApiSurface::factor` produces for
/// a `pub(crate) #[doc(hidden)]` PubItem. The previous aggregate
/// collector excluded `__-prefixed` macros entirely; per-macro
/// emission keeps them visible at low priority so the scheduler
/// decides on budget.
fn macro_body_value(file: &Path, info: &MacroInfo, ctx: &WalkCtx) -> f64 {
    let underscore = if info.underscore_private { 0.4 } else { 1.0 };
    let doc_hidden = if info.doc_hidden { 0.4 } else { 1.0 };
    let wrapper = if info.is_wrapper { 0.6 } else { 1.0 };
    let axis = underscore * doc_hidden * wrapper;
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

fn build_cross_file_content<F>(src_dir: &Path, ctx: &WalkCtx, collect: F) -> Option<BatchContent>
where
    F: Fn(&Tree, &str) -> FileLines,
{
    let rust_files = files_with_extension(src_dir, "rs");
    if rust_files.is_empty() {
        return None;
    }
    let mut all_spans: Vec<Span> = Vec::new();
    for file in &rust_files {
        let Some((source, tree)) = parse_rust(ctx, file) else {
            continue;
        };
        let lines = collect(&tree, &source);
        all_spans.extend(build_file_spans(file, &source, lines));
    }
    if all_spans.is_empty() {
        return None;
    }
    Some(BatchContent::Lines { spans: all_spans })
}

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

fn collect_module_doc_lede(tree: &Tree, source: &str) -> FileLines {
    FileLines::new(collect_module_doc_lines(tree, source, DocSection::Lede))
}

fn collect_module_doc_body(tree: &Tree, source: &str) -> FileLines {
    FileLines::new(collect_module_doc_lines(tree, source, DocSection::Body))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DocSection {
    /// First paragraph of `//!` only.
    Lede,
    /// Everything after the first paragraph.
    Body,
}

/// Collect line numbers belonging to the crate-`//!` block, split at the
/// first Markdown heading (`//! #`, `//! ##`, …). Regular license-header
/// `// comments` above the `//!` run are skipped. The lede is everything
/// from the first `//!` up to (but not including) the first heading line;
/// the body is from the heading onwards. If no heading is present, the
/// whole block is the lede.
///
/// Doctest-hidden lines are stripped *before* the lede/body split: a
/// crate doc that opens with a fenced Rust example whose first hidden
/// line happens to read `# use crate::X;` would otherwise have its
/// boundary land on the (invisible) doctest setup, mis-splitting around
/// the real heading.
fn collect_module_doc_lines(tree: &Tree, source: &str, section: DocSection) -> Vec<usize> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut all: Vec<usize> = Vec::new();
    for child in root.children(&mut cursor) {
        if matches!(child.kind(), "line_comment" | "block_comment") {
            if is_module_doc_comment(child, source) {
                extend_span(&mut all, child, source);
            }
            continue;
        }
        break;
    }
    split_doc_lines_at_first_heading(all, source, section)
}

/// Apply the shared rustdoc heading-split: drop hidden doctest lines,
/// then partition the remaining lines at the first ATX heading. Used by
/// both module-level (`//!`) and item-level (`///`, `/** */`) doc
/// collectors so the heading rule has one source of truth.
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

/// Strip the rustdoc comment marker from a raw source line and return
/// the post-marker content. Returns `None` for purely structural lines
/// (the `/**` / `/*!` opener with no body, the `*/` closer, or a lone
/// `*` continuation marker) — those carry no doc content and shouldn't
/// participate in fence/heading detection.
///
/// Recognized prefixes (longest-match first so `///` / `//!` always win
/// over the `*` continuation rule):
///   - `///` (with up to one optional space after)
///   - `//!` (with up to one optional space after)
///   - `/**` opener; if the post-marker remainder is whitespace only,
///     return `None`; else return the remainder (single-line block doc).
///   - `/*!` opener; same as above.
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

/// Match the Markdown ATX heading rule on already-normalized rustdoc
/// content. CommonMark accepts 0–3 leading spaces of indentation, then
/// 1–6 `#` characters, then either end-of-line or a space/tab. Rejects
/// `#######` (7 hashes, more than ATX allows), `#!` (the `#![attr]`
/// shape), and `#[` (the `#[attr]` shape) — both of which appear in
/// rustdoc examples and would be false-positive headings.
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

/// Drop rustdoc doctest-hidden lines. In any rustdoc block (`///`,
/// `//!`, or `/** */` / `/*! */`), lines whose first non-whitespace
/// token is `# ` or a bare `#`, while inside a Rust fenced code block
/// (` ``` ` or ` ~~~ ` with empty/`rust`/`no_run`/`ignore`/
/// `compile_fail`/`should_panic`/`edition*` info-string), are
/// scaffolding rustdoc strips from the rendered HTML. Keeping them in a
/// token-budgeted summary spends real tokens on content the human reader
/// of the docs never sees.
///
/// Operates on a sorted list of 1-based source line numbers, all expected
/// to belong to one contiguous rustdoc block. The fence state machine
/// runs on the *normalized* content (post comment-marker), so
/// `/** ` / ` * ` / ` */` block-doc continuation lines participate too.
/// Lines whose normalized form is `None` (the bare `/**` opener,
/// `*/` closer, or lone `*` continuation) are passed through unchanged
/// — they're structural and never the target of stripping. Preserves
/// honest rendering (output is still a verbatim subset of the source —
/// just a smaller one).
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

/// Returns `Some((kind, info_string))` if `content` (the post-prefix
/// remainder of a doc line, with one optional leading space stripped) opens
/// or closes a code fence. The info string is the trailing text after the
/// fence delimiter.
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

/// Whether a code fence's info string identifies a Rust block. Empty info
/// string defaults to Rust (rustdoc convention). The info-string's first
/// comma-separated token decides; case-insensitive.
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
    let mut out = Vec::new();
    collect_outer_docs_above(child, source, &mut out);
    dedup_sorted(out)
}

fn collect_method_sigs(tree: &Tree, _source: &str) -> FileLines {
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
        let sig_end = signature_end_row(child);
        push_rows(&mut full, child.start_position().row, sig_end);
        let mut body_cursor = body.walk();
        for inner in body.children(&mut body_cursor) {
            if matches!(inner.kind(), "function_item" | "function_signature_item") {
                let inner_end = signature_end_row(inner);
                push_rows(&mut full, inner.start_position().row, inner_end);
                added_method = true;
                if inner.child_by_field_name("body").is_some() {
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

fn collect_macro_name_lines(tree: &Tree, source: &str) -> FileLines {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut out = Vec::new();
    let mut ellipses = Vec::new();
    for child in root.children(&mut cursor) {
        if child.kind() == "macro_definition" && has_macro_export(child, source) {
            let name_row = child.start_position().row;
            push_rows(&mut out, name_row, name_row);
            ellipses.push(name_row + 2);
        }
    }
    FileLines::new(out).with_ellipses(ellipses)
}

/// Per-macro orientation gathered at `expand` time so each `MacroBody`
/// candidate carries the inputs `macro_body_signals` needs without re-
/// parsing the file. Mirrors the [`PubItemInfo`] / [`ApiSurface`] split.
///
/// `is_wrapper` is the "this macro just delegates to another exported
/// macro" classification: if the body invokes `$crate::<name>!` for some
/// `<name>` that is itself `#[macro_export]`'d in the same `src_dir`,
/// the macro is a thin re-export-style wrapper (e.g. log's `error!`,
/// `warn!`, `info!`, etc. all dispatch into `log!`). Wrappers carry a
/// strict refinement of root-dispatcher signals — same demotion shape
/// `pub_item_doc_body` uses against `pub_item`. Without this axis,
/// `error!` (smaller body) out-ratios `log!` (larger root-dispatcher
/// body) on cost concavity alone.
#[derive(Debug, Clone, Copy)]
struct MacroInfo {
    start_line: usize,
    underscore_private: bool,
    doc_hidden: bool,
    is_wrapper: bool,
}

fn find_macro_starts(
    tree: &Tree,
    source: &str,
    exported_names: &HashSet<String>,
) -> Vec<MacroInfo> {
    let mut out = Vec::new();
    for_each_exported_macro(tree, source, |node, name| {
        out.push(MacroInfo {
            start_line: node.start_position().row + 1,
            underscore_private: name.starts_with("__"),
            doc_hidden: has_doc_hidden(node, source),
            is_wrapper: macro_body_is_wrapper(node, Some(name), source, exported_names),
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

/// True when the macro's body invokes `$crate::<other>!` for some
/// public-facing sibling `#[macro_export]` macro in the same `src_dir`,
/// where `<other>` is not the macro itself, not `__`-prefixed, and is
/// in `exported_names`.
///
/// Self-exclusion guards a self-recursive exported macro from
/// classifying itself as its own wrapper.
///
/// `__`-target exclusion is load-bearing: a root dispatcher like `log!`
/// invokes `$crate::__log!` (an internal helper), and without the
/// exclusion `log!` would itself be classified as a wrapper. The
/// wrapper axis demotes `error!`/`warn!`/etc. (which delegate to
/// `log!`, the public API) below the root they wrap; targeting an
/// internal helper is not the same kind of delegation.
fn macro_body_is_wrapper(
    node: Node,
    self_name: Option<&str>,
    source: &str,
    exported_names: &HashSet<String>,
) -> bool {
    let body = &source[node.start_byte()..node.end_byte()];
    exported_names
        .iter()
        .filter(|name| !name.starts_with("__") && Some(name.as_str()) != self_name)
        .any(|name| body.contains(&format!("$crate::{name}!")))
}

fn compute_exported_macros(dir: &Path, ctx: &WalkCtx) -> HashSet<String> {
    let mut out = HashSet::new();
    for file in files_with_extension(dir, "rs") {
        let Some((source, tree)) = parse_rust(ctx, &file) else {
            continue;
        };
        for_each_exported_macro(&tree, &source, |_, name| {
            out.insert(name.to_string());
        });
    }
    out
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

fn is_module_doc_comment(node: Node, source: &str) -> bool {
    let text = &source[node.start_byte()..node.end_byte()];
    text.starts_with("//!") || text.starts_with("/*!")
}

fn is_outer_doc_comment(node: Node, source: &str) -> bool {
    let text = &source[node.start_byte()..node.end_byte()];
    text.starts_with("///") || text.starts_with("/**")
}

fn collect_outer_docs_above(node: Node, source: &str, out: &mut Vec<usize>) {
    let mut cur = node.prev_sibling();
    while let Some(prev) = cur {
        match prev.kind() {
            "line_comment" | "block_comment" if is_outer_doc_comment(prev, source) => {
                extend_span(out, prev, source);
                cur = prev.prev_sibling();
            }
            "attribute_item" => cur = prev.prev_sibling(),
            _ => break,
        }
    }
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
        let dir = tempdir();
        let src = dir.path().join("src");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(
            src.join("lib.rs"),
            "pub mod public_chain;\nmod private_chain;\n",
        )
        .unwrap();
        std::fs::write(
            src.join("public_chain.rs"),
            "pub mod grandchild;\nmod gc_private;\n",
        )
        .unwrap();
        std::fs::create_dir_all(src.join("public_chain")).unwrap();
        std::fs::write(src.join("public_chain").join("grandchild.rs"), "").unwrap();
        std::fs::write(src.join("public_chain").join("gc_private.rs"), "").unwrap();
        std::fs::write(src.join("private_chain.rs"), "pub mod gc_under_private;\n").unwrap();
        std::fs::create_dir_all(src.join("private_chain")).unwrap();
        std::fs::write(src.join("private_chain").join("gc_under_private.rs"), "").unwrap();

        let ctx = WalkCtx::new(dir.path().to_path_buf());
        let map = compute_module_visibility(&ctx);

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
        let dir = tempdir();
        let src = dir.path().join("src");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("main.rs"), "mod helpers;\n").unwrap();
        std::fs::write(src.join("helpers.rs"), "").unwrap();

        let ctx = WalkCtx::new(dir.path().to_path_buf());
        let map = compute_module_visibility(&ctx);
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
        let dir = tempdir();
        let src = dir.path().join("src");
        std::fs::create_dir_all(&src).unwrap();
        // Map seeds from lib.rs, which only declares public_child.
        std::fs::write(src.join("lib.rs"), "pub mod public_child;\n").unwrap();
        std::fs::write(src.join("public_child.rs"), "").unwrap();
        // Simulate a #[path]-mounted file the resolver didn't follow.
        std::fs::write(src.join("hidden_via_path_attr.rs"), "").unwrap();
        // Files outside src/ (tests/, examples/, build.rs) should still
        // default to Public — they get the non-essential-dir discount.
        std::fs::write(dir.path().join("build.rs"), "").unwrap();

        let ctx = WalkCtx::new(dir.path().to_path_buf());
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
        let dir = tempdir();
        let src = dir.path().join("src");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("main.rs"), "").unwrap();
        std::fs::write(src.join("helpers.rs"), "").unwrap();

        let ctx = WalkCtx::new(dir.path().to_path_buf());
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
        let dir = tempdir();
        let src = dir.path().join("src");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("lib.rs"), "pub mod kv;\n").unwrap();
        std::fs::create_dir_all(src.join("kv")).unwrap();
        std::fs::write(
            src.join("kv").join("mod.rs"),
            "mod error;\npub use self::error::Error;\n",
        )
        .unwrap();
        std::fs::write(src.join("kv").join("error.rs"), "pub struct Error;\n").unwrap();

        let ctx = WalkCtx::new(dir.path().to_path_buf());
        let map = compute_module_visibility(&ctx);
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
        let dir = tempdir();
        let src = dir.path().join("src");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("lib.rs"), "mod parent;\n").unwrap();
        std::fs::create_dir_all(src.join("parent")).unwrap();
        std::fs::write(
            src.join("parent").join("mod.rs"),
            "mod child;\npub use self::child::X;\n",
        )
        .unwrap();
        std::fs::write(src.join("parent").join("child.rs"), "pub struct X;\n").unwrap();

        let ctx = WalkCtx::new(dir.path().to_path_buf());
        let map = compute_module_visibility(&ctx);
        assert_eq!(
            map.get(&src.join("parent").join("child.rs")),
            Some(&Visibility::Restricted),
        );
    }

    #[test]
    fn rust_pub_use_self_handles_use_list_and_wildcard() {
        for re_export in ["pub use self::child::{A, B};", "pub use self::child::*;"] {
            let dir = tempdir();
            let src = dir.path().join("src");
            std::fs::create_dir_all(&src).unwrap();
            std::fs::write(src.join("lib.rs"), format!("mod child;\n{re_export}\n")).unwrap();
            std::fs::write(src.join("child.rs"), "pub struct A; pub struct B;\n").unwrap();

            let ctx = WalkCtx::new(dir.path().to_path_buf());
            let map = compute_module_visibility(&ctx);
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
        let dir = tempdir();
        let src = dir.path().join("src");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(
            src.join("lib.rs"),
            "mod a;\nmod b;\npub use self::{a::X, b::*};\n",
        )
        .unwrap();
        std::fs::write(src.join("a.rs"), "pub struct X;\n").unwrap();
        std::fs::write(src.join("b.rs"), "pub struct Y;\n").unwrap();

        let ctx = WalkCtx::new(dir.path().to_path_buf());
        let map = compute_module_visibility(&ctx);
        assert_eq!(map.get(&src.join("a.rs")), Some(&Visibility::Public));
        assert_eq!(map.get(&src.join("b.rs")), Some(&Visibility::Public));
    }

    #[test]
    fn rust_pub_use_self_handles_use_as_clause() {
        let dir = tempdir();
        let src = dir.path().join("src");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(
            src.join("lib.rs"),
            "mod child;\npub use self::child::Item as Renamed;\n",
        )
        .unwrap();
        std::fs::write(src.join("child.rs"), "pub struct Item;\n").unwrap();

        let ctx = WalkCtx::new(dir.path().to_path_buf());
        let map = compute_module_visibility(&ctx);
        assert_eq!(map.get(&src.join("child.rs")), Some(&Visibility::Public),);
    }

    #[test]
    fn rust_pub_use_self_walks_chain_through_private_mods() {
        // The chain walks past private `mod b;` to lift b.rs because
        // `pub use self::a::b::X` makes `b`'s contents publicly reachable
        // even though `a.rs` only declares it privately.
        let dir = tempdir();
        let src = dir.path().join("src");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("lib.rs"), "mod a;\npub use self::a::b::X;\n").unwrap();
        std::fs::create_dir_all(src.join("a")).unwrap();
        std::fs::write(src.join("a").join("mod.rs"), "mod b;\n").unwrap();
        std::fs::write(src.join("a").join("b.rs"), "pub struct X;\n").unwrap();

        let ctx = WalkCtx::new(dir.path().to_path_buf());
        let map = compute_module_visibility(&ctx);
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
        let dir = tempdir();
        let src = dir.path().join("src");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("lib.rs"), "mod a;\npub use self::a::b::X;\n").unwrap();
        std::fs::create_dir_all(src.join("a")).unwrap();
        std::fs::write(src.join("a").join("mod.rs"), "// no mod b\n").unwrap();
        std::fs::write(src.join("a").join("b.rs"), "pub struct X;\n").unwrap();

        let ctx = WalkCtx::new(dir.path().to_path_buf());
        let map = compute_module_visibility(&ctx);
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
        let dir = tempdir();
        let src = dir.path().join("src");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(
            src.join("lib.rs"),
            "mod foo;\npub use crate::foo::Bar;\npub use my_crate::baz::Qux;\n",
        )
        .unwrap();
        std::fs::write(src.join("foo.rs"), "pub struct Bar;\n").unwrap();

        let ctx = WalkCtx::new(dir.path().to_path_buf());
        let map = compute_module_visibility(&ctx);
        assert_eq!(map.get(&src.join("foo.rs")), Some(&Visibility::Restricted),);
    }

    #[test]
    fn rust_pub_use_with_inner_visibility_does_not_lift() {
        let dir = tempdir();
        let src = dir.path().join("src");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(
            src.join("lib.rs"),
            "mod child;\npub(crate) use self::child::X;\n",
        )
        .unwrap();
        std::fs::write(src.join("child.rs"), "pub struct X;\n").unwrap();

        let ctx = WalkCtx::new(dir.path().to_path_buf());
        let map = compute_module_visibility(&ctx);
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
        let dir = tempdir();
        let src = dir.path().join("src");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("lib.rs"), "mod child;\npub mod child;\n").unwrap();
        std::fs::write(src.join("child.rs"), "").unwrap();

        let ctx = WalkCtx::new(dir.path().to_path_buf());
        let map = compute_module_visibility(&ctx);
        assert_eq!(map.get(&src.join("child.rs")), Some(&Visibility::Public));
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
    fn rust_strip_hidden_lines_default_fence() {
        let src =
            "//! ```\n//! # use anyhow::Result;\n//! fn ok() -> Result<()> { Ok(()) }\n//! ```\n";
        assert_eq!(
            strip_via(src),
            "//! ```\n//! fn ok() -> Result<()> { Ok(()) }\n//! ```",
        );
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
    fn rust_strip_hidden_lines_heading_outside_fence() {
        let src = "//! # Real heading\n//! prose\n";
        assert_eq!(strip_via(src), src.trim_end());
    }

    #[test]
    fn rust_strip_hidden_lines_double_hash_in_rust_fence() {
        // `## foo` inside a Rust fence is rendered (rustdoc consumes one #),
        // so the line must be kept.
        let src = "/// ```\n/// ## foo\n/// ```\n";
        assert_eq!(strip_via(src), src.trim_end());
    }

    #[test]
    fn rust_strip_hidden_lines_attribute_lines_kept() {
        // `#[derive(...)]` and `#![cfg(...)]` don't start with `# ` (no
        // space), so they're code, not hidden.
        let src = "/// ```\n/// #[derive(Debug)]\n/// #![allow(unused)]\n/// struct S;\n/// ```\n";
        assert_eq!(strip_via(src), src.trim_end());
    }

    #[test]
    fn rust_strip_hidden_lines_lone_hash_dropped() {
        let src = "/// ```\n/// #\n/// real\n/// ```\n";
        let got = strip_via(src);
        assert!(!got.contains("/// #\n"), "lone `#` should drop; got: {got}");
        assert!(got.contains("/// real"));
    }

    #[test]
    fn rust_strip_hidden_lines_tilde_fence() {
        let src = "/// ~~~\n/// # let x = 1;\n/// real\n/// ~~~\n";
        let got = strip_via(src);
        assert!(
            !got.contains("# let x"),
            "tilde-fence should drop `# `; got: {got}"
        );
    }

    #[test]
    fn rust_strip_hidden_lines_mismatched_fence_kind_does_not_close() {
        // Inside a backtick Rust fence, a `~~~` line is literal content (not
        // a closer). Subsequent `# ` lines stay hidden.
        let src = "/// ```\n/// # hidden 1\n/// ~~~ inline\n/// # hidden 2\n/// ```\n";
        let got = strip_via(src);
        assert!(!got.contains("# hidden 1"));
        assert!(!got.contains("# hidden 2"));
        assert!(got.contains("~~~ inline"));
    }

    #[test]
    fn rust_strip_hidden_lines_multi_fence_state_machine() {
        // Lede: prose, then a Rust fence, then a console fence, then a
        // tilde-rust fence. Hidden lines in Rust fences only.
        let src = "\
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
";
        let got = strip_via(src);
        assert!(!got.contains("# hidden a"));
        assert!(!got.contains("# hidden b"));
        assert!(got.contains("# kept-console"));
        assert!(got.contains("visible a"));
        assert!(got.contains("visible b"));
        assert!(got.contains("//! tail"));
    }

    #[test]
    fn rust_strip_hidden_lines_works_with_inner_doc() {
        // Same body for both `///` and `//!` prefix forms.
        let outer = "/// ```\n/// # hidden\n/// real\n/// ```\n";
        let inner = "//! ```\n//! # hidden\n//! real\n//! ```\n";
        assert!(!strip_via(outer).contains("# hidden"));
        assert!(!strip_via(inner).contains("# hidden"));
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
    fn rust_strip_hidden_lines_block_doc_unmarked_lines_pass_through() {
        // Block doc lines without a `*` continuation marker (rare but
        // syntactically allowed) normalize to `None` and pass through
        // — they aren't recognized as fence/heading candidates.
        let src = "/** ```\n# would-be-hidden-but-not-handled\nreal\n``` */\n";
        let got = strip_via(src);
        assert!(got.contains("# would-be-hidden-but-not-handled"));
    }

    #[test]
    fn rust_strip_hidden_lines_block_doc_strips_starred_hidden_inside_fence() {
        // Block doc with proper `*` continuation: ` * # use ...` inside a
        // Rust fence is hidden scaffolding and must be stripped, just like
        // `/// # use ...` and `//! # use ...` are today.
        let src = "/**\n * ```\n * # use crate::X;\n * real\n * ```\n */\n";
        let got = strip_via(src);
        assert!(
            !got.contains("# use crate::X"),
            "block-doc hidden line not stripped; got: {got}",
        );
        assert!(
            got.contains("* real"),
            "visible block-doc line missing; got: {got}",
        );
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
    fn rust_pub_item_doc_lede_body_split_at_heading() {
        let src = "/// Summary line.\n/// More prose.\n///\n/// # Examples\n/// example()\npub fn foo() {}\n";
        let lede = collect_doc_section_lines(src, DocSection::Lede);
        let body = collect_doc_section_lines(src, DocSection::Body);
        assert!(
            lede.iter().any(|l| l.contains("Summary line")),
            "lede missing summary: {lede:?}",
        );
        assert!(
            !lede.iter().any(|l| l.contains("# Examples")),
            "lede crossed heading: {lede:?}",
        );
        assert!(
            body.first().is_some_and(|l| l.contains("# Examples")),
            "body should start at heading: {body:?}",
        );
        assert!(body.iter().any(|l| l.contains("example()")));
    }

    #[test]
    fn rust_pub_item_doc_no_heading_emits_only_lede() {
        let src = "/// Summary line.\n/// More prose.\npub fn foo() {}\n";
        let lede = collect_doc_section_lines(src, DocSection::Lede);
        let body = collect_doc_section_lines(src, DocSection::Body);
        assert!(!lede.is_empty(), "lede should cover the doc: {lede:?}");
        assert!(body.is_empty(), "body should be empty: {body:?}");
    }

    #[test]
    fn rust_pub_item_doc_starts_with_heading_emits_only_body() {
        // When the doc's first non-hidden line is already an ATX heading,
        // the lede is empty. `expand` must wire the Body's predecessor to
        // `PubItem` (not the absent Lede); see `walker::expand`.
        let src = "/// # Examples\n/// example()\npub fn foo() {}\n";
        let lede = collect_doc_section_lines(src, DocSection::Lede);
        let body = collect_doc_section_lines(src, DocSection::Body);
        assert!(lede.is_empty(), "lede should be empty: {lede:?}");
        assert!(body.iter().any(|l| l.contains("# Examples")));
    }

    #[test]
    fn rust_pub_item_doc_strips_hidden_doctest_before_split() {
        // Hidden `# use ...` inside a Rust fence at the *start* of the
        // doc must not be classified as a heading.
        let src = "/// ```\n/// # use foo;\n/// real()\n/// ```\n///\n/// # Real Heading\n/// detail\npub fn foo() {}\n";
        let body = collect_doc_section_lines(src, DocSection::Body);
        assert!(
            body.first().is_some_and(|l| l.contains("# Real Heading")),
            "body should start at real heading: {body:?}",
        );
    }

    #[test]
    fn rust_pub_item_doc_block_doc_split_at_starred_heading() {
        let src = "/**\n * Summary.\n *\n * # Examples\n * example()\n */\npub fn foo() {}\n";
        let lede = collect_doc_section_lines(src, DocSection::Lede);
        let body = collect_doc_section_lines(src, DocSection::Body);
        assert!(
            lede.iter().any(|l| l.contains("Summary.")),
            "lede missing summary: {lede:?}",
        );
        assert!(
            !lede.iter().any(|l| l.contains("# Examples")),
            "lede crossed heading: {lede:?}",
        );
        assert!(
            body.iter().any(|l| l.contains("# Examples")),
            "body missing heading: {body:?}",
        );
    }

    #[test]
    fn rust_pub_item_doc_block_doc_strips_hidden_doctest_before_split() {
        let src = "/**\n * ```\n * # use crate::X;\n * real()\n * ```\n *\n * # Real Heading\n * detail\n */\npub fn foo() {}\n";
        let body = collect_doc_section_lines(src, DocSection::Body);
        assert!(
            body.first().is_some_and(|l| l.contains("# Real Heading")),
            "body should start at real heading: {body:?}",
        );
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

    #[test]
    fn rust_macro_self_recursive_export_is_not_classified_as_wrapper() {
        // A `#[macro_export] macro_rules! foo` whose body invokes
        // `$crate::foo!` recursively is the root macro, not a thin
        // delegate. Codex adversarial review flagged self-recursion as
        // a false-positive wrapper case before merge.
        let src = "#[macro_export]\nmacro_rules! foo {\n    () => { };\n    ($($t:tt)+) => { $crate::foo!() };\n}\n";
        let tree = parse(src);
        let names: HashSet<String> = ["foo".to_string()].into_iter().collect();
        let infos = find_macro_starts(&tree, src, &names);
        assert_eq!(infos.len(), 1);
        assert!(
            !infos[0].is_wrapper,
            "self-recursive macro must not be a wrapper",
        );
    }

    #[test]
    fn rust_macro_wrapper_classification_excludes_underscore_targets() {
        // `log!` body invokes `$crate::__log!` — `__log` is exported
        // but `__`-prefixed, which is excluded from the wrapper
        // predicate. So `log!` is not a wrapper of `__log`.
        let src = "#[macro_export]\nmacro_rules! __log {\n    () => { };\n}\n#[macro_export]\nmacro_rules! log {\n    () => { $crate::__log!() };\n}\n#[macro_export]\nmacro_rules! error {\n    () => { $crate::log!() };\n}\n";
        let tree = parse(src);
        let names: HashSet<String> = ["__log", "log", "error"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let infos = find_macro_starts(&tree, src, &names);
        let by_line: HashMap<_, _> = infos.iter().map(|i| (i.start_line, i)).collect();
        // __log: no body call to anything → not wrapper
        // log: body calls $crate::__log! (excluded) → not wrapper
        // error: body calls $crate::log! (non-__) → wrapper
        let log_line = src
            .lines()
            .position(|l| l.contains("macro_rules! log"))
            .unwrap()
            + 1;
        let error_line = src
            .lines()
            .position(|l| l.contains("macro_rules! error"))
            .unwrap()
            + 1;
        assert!(!by_line[&log_line].is_wrapper, "log! must be a root");
        assert!(by_line[&error_line].is_wrapper, "error! must be a wrapper");
    }
}
