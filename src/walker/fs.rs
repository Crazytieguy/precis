//! Filesystem walker. Discovers directories, emits listings, and surveys
//! file sets for per-language walkers. Only does `read_dir` — never reads
//! file contents. Pure listing lives in [`crate::fs_util::list_dir`];
//! this module holds walker-specific policy (heavy-directory skip,
//! per-language file enumeration).

use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    path::{Path, PathBuf},
};

use crate::batch::{Batch, BatchKey, FsKey};
use crate::content::{BatchContent, FsEntries, FsGroup};
pub use crate::fs_util::list_dir;
use crate::fs_util::{DirFilter, EntryKind};
use crate::value::{is_colocated_test_filename, mix_signals};

use super::{WalkCtx, file_depth_factor, path_depth_factor};

/// Seed: list the root directory.
pub fn seed(ctx: &WalkCtx) -> Vec<Batch<BatchKey>> {
    dir_listing_batch(ctx.root().to_path_buf(), ctx)
        .into_iter()
        .collect()
}

/// Subdirectory listings for the just-scheduled dir's listing.
pub fn expand_subdirs(dir: &Path, ctx: &WalkCtx) -> Vec<Batch<BatchKey>> {
    let children = list_dir(dir, ctx.dir_filter());
    let mut out = Vec::new();
    for (name, kind) in children {
        if matches!(kind, EntryKind::Directory)
            && let child = dir.join(&name)
            && should_recurse_dir(&child, ctx.root())
            && let Some(batch) = dir_listing_batch(child, ctx)
        {
            out.push(batch);
        }
    }
    out
}

// ---- additional helpers ----

/// Files in `dir` (non-recursive) whose extension matches.
pub fn files_with_extension(dir: &Path, ext: &str, ctx: &WalkCtx) -> Vec<PathBuf> {
    files_with_any_extension(dir, &[ext], ctx)
}

/// Files in `dir` matching any of `exts`, in one `read_dir` pass.
pub fn files_with_any_extension(dir: &Path, exts: &[&str], ctx: &WalkCtx) -> Vec<PathBuf> {
    let Ok(read_dir) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut out: Vec<PathBuf> = read_dir
        .flatten()
        .filter_map(|e| {
            // `Path::is_file` follows symlinks. Extension discovery feeds
            // whole-file readers as well as parsers, so admitting a link
            // here could render content outside the walk root. Use the
            // directory entry's non-following type and reject symlinks
            // uniformly before any metadata or source read.
            if !e.file_type().ok()?.is_file() {
                return None;
            }
            let path = e.path();
            let actual = path.extension().and_then(|e| e.to_str())?;
            if !exts.iter().any(|ext| actual.eq_ignore_ascii_case(ext)) {
                return None;
            }
            // Same filter the listing uses: a file the listing hides
            // must not come back as a content batch.
            (!ctx.dir_filter().excludes(&path, false)).then_some(path)
        })
        .collect();
    out.sort();
    out
}

/// Recursively walk `dir` for files with `ext` (case-insensitive).
///
/// Callers hand this a directory they resolved themselves rather than
/// one the listing walk reached, so `dir` itself has to clear the
/// filter — ancestors included, since a directory-only ignore pattern
/// matches the directory and not the files inside it.
pub fn files_with_extension_recursive(dir: &Path, ext: &str, ctx: &WalkCtx) -> Vec<PathBuf> {
    if ctx.dir_filter().excludes_tree(dir, true) {
        return Vec::new();
    }
    let mut out = Vec::new();
    walk_files_recursive(dir, dir, ext, ctx, &mut out);
    out.sort();
    out
}

fn walk_files_recursive(
    dir: &Path,
    traversal_root: &Path,
    ext: &str,
    ctx: &WalkCtx,
    out: &mut Vec<PathBuf>,
) {
    let Ok(read_dir) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in read_dir.flatten() {
        // Non-following entry types, for the reason spelled out in
        // `files_with_any_extension`: `Path::is_dir`/`is_file` resolve
        // links, so this walk would otherwise descend a `docs -> /etc`
        // link straight out of the root — and a `link -> ..` one would
        // never come back at all. Links are listed by
        // `crate::fs_util::list_dir` when they stay inside the root;
        // they are never a traversal edge.
        let Ok(file_type) = entry.file_type() else {
            continue;
        };
        let path = entry.path();
        if file_type.is_dir() {
            if should_recurse_dir(&path, traversal_root) && !ctx.dir_filter().excludes(&path, true)
            {
                walk_files_recursive(&path, traversal_root, ext, ctx, out);
            }
        } else if file_type.is_file()
            && path
                .extension()
                .and_then(|e| e.to_str())
                .is_some_and(|actual| actual.eq_ignore_ascii_case(ext))
            && !ctx.dir_filter().excludes(&path, false)
        {
            out.push(path);
        }
    }
}

fn dir_listing_batch(dir: PathBuf, ctx: &WalkCtx) -> Option<Batch<BatchKey>> {
    let children = list_dir(&dir, ctx.dir_filter());
    if children.is_empty() {
        return None;
    }
    let value = dir_listing_value(&dir, &children, ctx);
    let paths: Vec<PathBuf> = children.into_keys().map(PathBuf::from).collect();
    Some(Batch {
        key: FsKey::DirListing { dir: dir.clone() }.into(),
        predecessor: None,
        content: BatchContent::Fs {
            groups: vec![FsGroup {
                parent: dir,
                entries: FsEntries::Listed(paths),
            }],
        },
        value,
    })
}

fn dir_listing_value(dir: &Path, children: &BTreeMap<String, EntryKind>, ctx: &WalkCtx) -> f64 {
    let module_source_dir = is_module_source_dir(dir);
    let source_dir = is_source_dir(dir) || is_go_pkg_wrapper(dir);
    let src_of_sibling_modules = source_dir
        && module_sibling_child_dir_count(
            dir,
            children,
            MIN_SIBLING_MODULE_CHILD_DIRS_FOR_SRC_ROOT,
        ) >= MIN_SIBLING_MODULE_CHILD_DIRS_FOR_SRC_ROOT;
    let non_essential = ctx.non_essential_factor(dir);
    // Inventory promotion: covers two cases.
    //   1. Supporting corpora (`tests/`, `examples/`, `docs/`): the
    //      `non_essential < 1.0` gate keeps "inventories" outside source
    //      orientation. Source/module dirs inside those corpora already
    //      qualify structurally via the sibling-module / module-source
    //      checks.
    //   2. Flat source partitions under a real source ancestor (`lib/`,
    //      `src/`): directories like axios's `lib/helpers` (34 JS files,
    //      no `index.js`) or generic `src/utils/` are the package's API
    //      partition. They lack a structural anchor (no entrypoint file,
    //      no sibling module), so the inventory probe is what surfaces
    //      them — without it they fall to the catch-all listing tier and
    //      their (legitimately) large listing loses every V/C race to
    //      tiny sibling dirs.
    let supporting_source_dir = non_essential < 1.0 && (source_dir || module_source_dir);
    let under_root_source_ancestor = has_source_root_ancestor(dir, ctx);
    // The structural half of that probe — "this directory is a catalog
    // of source files, and isn't a named source root or module dir" —
    // is also what decides roster mass below, at a lower strength for
    // the members the inventory tier's location clause leaves out.
    let structural_catalog = !source_dir && !module_source_dir && is_source_inventory_dir(dir, ctx);
    let source_inventory_dir =
        structural_catalog && (non_essential < 1.0 || under_root_source_ancestor);
    let readme_cited = ctx.is_readme_cited(dir);
    let (cat, fu, ztu) = if dir == ctx.root() {
        (0.95, 0.6, 0.5)
    } else if src_of_sibling_modules || module_source_dir {
        // `module_source_dir` joins the sibling-module tier: a directory
        // with its own module entrypoint (`__init__.py` / `index.ts` /
        // `mod.rs`) is the package's API surface root, and its listing
        // is itself a high-value NS anchor (rich/ NS row 1.10 is "rich/
        // package listing — all ~80 modules"). At the lower 0.6 cat,
        // an 80-name listing's ratio loses to small sibling listings of
        // peripheral dirs.
        (0.85, 0.55, 0.35)
    } else if source_dir || source_inventory_dir || readme_cited {
        // README-cited dirs (an `examples/` directory the README links
        // canonical scripts from) are part of the documented public
        // surface; treat them on par with source-inventory dirs so the
        // listing schedules early enough for per-file batches inside it
        // to compete in the early budget.
        //
        // `cat` is held at the catch-all's level rather than below it: a
        // directory we positively recognize as source should not be
        // priced under one we failed to classify. Swept 2026-07-26 —
        // the response is a shallow ridge over 0.95–1.08 and falls off
        // sharply outside it, so this sits at the low-churn end.
        (0.95, 0.5, 0.3)
    } else {
        (0.95, 0.45, 0.25)
    };
    let depth = if source_inventory_dir && under_root_source_ancestor {
        // A flat partition under a root-adjacent `lib/`/`src/` is the
        // package's API surface root (alongside the module-source tier)
        // — its listing is what names the partition, regardless of
        // layout depth. Pin to depth 1 so axios's `lib/helpers` doesn't
        // get a path-depth discount that lets the same package's tiny
        // sibling dirs out-rank it on the V/C race.
        file_depth_factor(dir, ctx, true)
    } else if supporting_source_dir || source_inventory_dir {
        inventory_depth_factor(dir, ctx, non_essential)
    } else if module_source_dir || src_of_sibling_modules {
        file_depth_factor(dir, ctx, true)
    } else {
        path_depth_factor(dir, ctx)
    };
    // Depth-1 repo-map floor: the one-line-per-entry listing of a
    // top-level directory is part of the shallow repo map NS authors
    // rank as tier-1 orientation, regardless of the dir's
    // classification — the non-essential discount belongs to the
    // dir's *contents*, not to knowing what's in it. Floor the
    // location prior for the listing batch only (at depth 1 the
    // prior is exactly the non-essential factor, so this floors the
    // discount at 0.5).
    let depth = if ctx.depth_from_root(dir) == 1 {
        depth.max(0.5)
    } else {
        depth
    };
    // Source-inventory catalogs keep size-neutral ranking: a flat
    // partition's complete listing is the API map NS authors anchor on,
    // and without the factor an N-entry listing's ratio falls as N^-k
    // against tiny same-tier sibling listings. Boosting plain
    // `src/`-named or module dirs lets the big listing itself displace
    // NS-wanted content (measured: soluna −0.210, beszel −0.073), so
    // those stay out; the rest of the structural catalog class is
    // neutralized by [`catalog_roster_mass_factor`].
    let fanout = if source_inventory_dir {
        crate::value::roster_mass_factor(children.len())
    } else if structural_catalog {
        catalog_roster_mass_factor(children)
    } else {
        1.0
    };
    // A large root-level `test/`/`spec/` dir of per-feature source files is
    // the spec index ("what test covers feature X"): NS authors rank the
    // *listing* mid-tier, but the non-essential floor (0.5) leaves its
    // ratio far below the orientation tier, so it lands past 3K (chibicc
    // test/ at ~3110 vs NS rank 7). Lift the listing — scoped to the
    // listing batch, never the test bodies — for root-adjacent test dirs
    // large enough to be a real spec suite.
    let test_index_boost =
        if source_inventory_dir && is_large_root_test_inventory_dir(dir, ctx, children) {
            TEST_INDEX_LISTING_BOOST
        } else {
            1.0
        };
    // Catalog-child suppression: when the parent is a high-fanout
    // source-inventory catalog, its own listing already names every child,
    // so each child's (near-identical) listing is redundant reference
    // detail. Without this, monaco's `src/languages/definitions/` catalog
    // (82 language dirs) spends ~1.2K of the 3K budget on 82 two-file
    // child listings, displacing the sibling-catalog listings the NS
    // ranks next (features/, deprecated/, build/). Strong deferral rather
    // than omission so the children stay reachable at large budgets.
    let catalog_child_factor = if parent_is_high_fanout_catalog(dir, ctx) {
        CATALOG_CHILD_LISTING_SUPPRESSION
    } else {
        1.0
    };
    mix_signals(cat, fu, ztu, depth) * fanout * catalog_child_factor * test_index_boost
}

/// Min child-directory count for a parent to count as a "catalog" whose
/// per-child listings are redundant with its own listing.
const CATALOG_PARENT_MIN_CHILD_DIRS: usize = 10;
/// Deferral multiplier applied to a redundant catalog child's listing —
/// low enough to push it past the primary budget, non-zero so it stays
/// reachable at large budgets.
const CATALOG_CHILD_LISTING_SUPPRESSION: f64 = 0.05;

/// True when `dir`'s parent is a high-fanout source-inventory catalog:
/// a directory of many uniform child dirs (monaco's `definitions/`,
/// tinyusb's `portable/`) whose own listing enumerates every child.
/// Named `src`/`lib`/`pkg` roots and package module dirs (with an
/// `index.*`/`__init__.py`/`mod.rs` entrypoint) are deliberately NOT
/// catalogs — their children are first-class modules, not catalog leaves.
fn parent_is_high_fanout_catalog(dir: &Path, ctx: &WalkCtx) -> bool {
    let Some(parent) = dir.parent() else {
        return false;
    };
    let parent_source_dir = is_source_dir(parent) || is_go_pkg_wrapper(parent);
    if parent_source_dir || is_module_source_dir(parent) {
        return false;
    }
    let under_source_ancestor = has_source_root_ancestor(parent, ctx);
    if ctx.non_essential_factor(parent) >= 1.0 && !under_source_ancestor {
        return false;
    }
    if !is_source_inventory_dir(parent, ctx) {
        return false;
    }
    ctx.fs_state().child_dir_count(parent, ctx.dir_filter()) >= CATALOG_PARENT_MIN_CHILD_DIRS
}

/// Fraction of the way to full roster-mass neutrality taken outside the
/// source-inventory tier. Swept 0.5 / 0.75 / 1.0 on the full corpus; the
/// response is non-monotone and 0.75 wins at every budget but 4327.
const CATALOG_ROSTER_MASS_NEUTRALIZATION: f64 = 0.75;

/// Partial [`crate::value::roster_mass_factor`] for a directory the
/// structural catalog test recognizes outside the source-inventory
/// tier. The mass counts only the entries the listing *terminates* on:
/// a file's row is the last thing the walk will say about it, while a
/// subdirectory's row is a pointer to a listing the scheduler buys
/// separately — so neutralizing a package root whose content lives in
/// subdirectories buys the index plus the swarm it seeds (measured at
/// full mass: 1751 tokens of scaffolding listings on one monorepo).
fn catalog_roster_mass_factor(children: &BTreeMap<String, EntryKind>) -> f64 {
    let terminal_entries = children
        .values()
        .filter(|kind| matches!(kind, EntryKind::File))
        .count();
    let neutral = crate::value::roster_mass_factor(terminal_entries);
    1.0 + (neutral - 1.0) * CATALOG_ROSTER_MASS_NEUTRALIZATION
}

/// Count of immediate subdirectories of `dir`.
fn child_dir_count_uncached(dir: &Path, filter: &DirFilter) -> usize {
    list_dir(dir, filter)
        .values()
        .filter(|kind| matches!(kind, EntryKind::Directory))
        .count()
}

pub(crate) const JS_MODULE_ENTRYPOINT_FILES: &[&str] = &[
    "index.ts",
    "index.tsx",
    "index.js",
    "index.mjs",
    "index.cjs",
];
const NON_JS_MODULE_ENTRYPOINT_FILES: &[&str] = &["mod.rs", "__init__.py"];
const MODULE_SIBLING_EXTS: &[&str] = &["rs", "ts", "tsx", "py"];
const MIN_SIBLING_MODULE_CHILD_DIRS_FOR_SRC_ROOT: usize = 2;

/// Case-insensitive, and `Sources/` counts: that is the spelling
/// SwiftPM mandates, and the same for `Source/` in Objective-C and
/// C# trees. A case-sensitive check leaves those repos with no
/// recognised source root at all.
pub(crate) fn is_source_dir(dir: &Path) -> bool {
    dir.file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|name| {
            matches!(
                name.to_ascii_lowercase().as_str(),
                "src" | "lib" | "source" | "sources"
            )
        })
}

/// `pkg/` directory next to a `go.mod` — the Go convention for
/// primary library code. Equivalent to `lib/`/`src/` in JS/TS.
fn is_go_pkg_wrapper(dir: &Path) -> bool {
    dir.file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|name| name == "pkg")
        && dir.parent().is_some_and(|p| p.join("go.mod").is_file())
}

fn has_module_entrypoint(dir: &Path) -> bool {
    JS_MODULE_ENTRYPOINT_FILES
        .iter()
        .chain(NON_JS_MODULE_ENTRYPOINT_FILES.iter())
        .any(|name| dir.join(name).is_file())
}

/// Go module-source detection — a dir with multiple non-test `.go`
/// files, restricted to root-level subpackages or those directly
/// under a root-level `pkg/` wrapper.
fn is_go_module_subpackage(dir: &Path) -> bool {
    const MIN_GO_FILES: usize = 5;
    // Either the parent has a go.mod (subpackage of an outer Go module —
    // gin's `binding`, lo's `it`), or the dir itself does (nested Go
    // module — lo's `exp/simd`, bubbletea's `examples`/`tutorials`).
    // Both shapes carry an API-surface listing NS authors anchor on.
    let Some(parent) = dir.parent() else {
        return false;
    };
    let has_outer_module = parent.join("go.mod").is_file();
    let has_own_module = dir.join("go.mod").is_file();
    let under_pkg_wrapper = is_go_pkg_wrapper(parent);
    if !has_outer_module && !has_own_module && !under_pkg_wrapper {
        return false;
    }
    count_go_package_source(dir, MIN_GO_FILES) >= MIN_GO_FILES
}

fn count_go_package_source(dir: &Path, target: usize) -> usize {
    let Ok(read_dir) = std::fs::read_dir(dir) else {
        return 0;
    };
    let mut count = 0;
    for entry in read_dir.flatten() {
        let Ok(ft) = entry.file_type() else { continue };
        if !ft.is_file() {
            continue;
        }
        let name = entry.file_name();
        let Some(name) = name.to_str() else { continue };
        if name.ends_with(".go") && !name.ends_with("_test.go") {
            count += 1;
            if count >= target {
                return count;
            }
        }
    }
    count
}

fn is_module_source_dir(dir: &Path) -> bool {
    if is_type_surface_dir(dir) {
        return false;
    }
    let entrypoint_module = has_module_entrypoint(dir)
        && (dir.parent().is_some_and(is_source_dir) || has_python_module_entrypoint(dir));
    entrypoint_module || is_sibling_module_source_dir(dir) || is_go_module_subpackage(dir)
}

fn is_type_surface_dir(dir: &Path) -> bool {
    dir.file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|name| matches!(name, "type" | "types" | "typings"))
}

fn has_python_module_entrypoint(dir: &Path) -> bool {
    dir.join("__init__.py").is_file()
}

fn is_sibling_module_source_dir(dir: &Path) -> bool {
    has_module_sibling_file(dir) && !is_type_surface_dir(dir)
}

fn has_module_sibling_file(dir: &Path) -> bool {
    // The filesystem root has no stem to attach a sibling extension to.
    if dir.file_name().is_none() {
        return false;
    }
    let mut sibling = dir.to_path_buf();
    MODULE_SIBLING_EXTS.iter().any(|ext| {
        sibling.set_extension(ext);
        sibling.is_file()
    })
}

fn module_sibling_child_dir_count(
    dir: &Path,
    children: &BTreeMap<String, EntryKind>,
    target: usize,
) -> usize {
    let mut count = 0;
    for (name, kind) in children {
        if matches!(kind, EntryKind::Directory) && has_module_sibling_file(&dir.join(name)) {
            count += 1;
            if count >= target {
                break;
            }
        }
    }
    count
}

#[derive(Default)]
pub(in crate::walker) struct FsState {
    source_inventory_counts: RefCell<HashMap<PathBuf, usize>>,
    child_dir_counts: RefCell<HashMap<PathBuf, usize>>,
    entry_counts: RefCell<HashMap<PathBuf, usize>>,
}

impl FsState {
    pub(in crate::walker) fn source_inventory_count(
        &self,
        dir: &Path,
        target: usize,
        filter: &DirFilter,
    ) -> usize {
        if let Some(count) = self.source_inventory_counts.borrow().get(dir).copied() {
            return count;
        }
        let count = source_inventory_count_uncached(self, dir, target, filter);
        self.source_inventory_counts
            .borrow_mut()
            .insert(dir.to_path_buf(), count);
        count
    }

    /// Count of immediate subdirectories of `dir`, cached per parent.
    /// `parent_is_high_fanout_catalog` queries the same parent once per
    /// child, so without this each child re-`read_dir`s the parent (O(N²)).
    pub(in crate::walker) fn child_dir_count(&self, dir: &Path, filter: &DirFilter) -> usize {
        if let Some(count) = self.child_dir_counts.borrow().get(dir).copied() {
            return count;
        }
        let count = child_dir_count_uncached(dir, filter);
        self.child_dir_counts
            .borrow_mut()
            .insert(dir.to_path_buf(), count);
        count
    }

    /// Total listed entries in `dir`, cached — the size of the listing
    /// the scheduler would have to buy. Cached for the same reason as
    /// [`FsState::child_dir_count`]: the catalog test asks about the
    /// same parent once per child.
    pub(in crate::walker) fn entry_count(&self, dir: &Path, filter: &DirFilter) -> usize {
        if let Some(count) = self.entry_counts.borrow().get(dir).copied() {
            return count;
        }
        let count = list_dir(dir, filter).len();
        self.entry_counts
            .borrow_mut()
            .insert(dir.to_path_buf(), count);
        count
    }
}

fn is_source_inventory_dir(dir: &Path, ctx: &WalkCtx) -> bool {
    const MIN_SOURCE_FILES: usize = 3;
    ctx.fs_state()
        .source_inventory_count(dir, MIN_SOURCE_FILES, ctx.dir_filter())
        >= MIN_SOURCE_FILES
}

/// Ratio multiplier lifting a large root-level test/spec listing out of
/// the non-essential floor so its spec-index value clears the orientation
/// tier at small budgets.
const TEST_INDEX_LISTING_BOOST: f64 = 1.8;
/// Minimum per-feature source files for a root test dir to read as a spec
/// suite worth surfacing (not a handful of smoke tests).
const MIN_TEST_INDEX_FILES: usize = 12;

/// A root-adjacent `test`/`tests`/`spec`/`specs` directory that is a
/// per-feature spec catalog (chibicc's `test/arith.c`, `test/cast.c` …) —
/// the "what tests cover feature X" index the NS ranks mid-tier. Gated
/// away from unit-test suites (commander's `tests/*.test.js`): those files
/// carry the co-located test-convention naming and mirror source modules,
/// and the NS never indexes them, so boosting the listing displaces
/// tier-1 orientation. The discriminator is the file naming — standalone
/// feature-named programs vs `.test.`/`.spec.`/`_test.` suffixes.
fn is_large_root_test_inventory_dir(
    dir: &Path,
    ctx: &WalkCtx,
    children: &BTreeMap<String, EntryKind>,
) -> bool {
    if dir.parent() != Some(ctx.root()) {
        return false;
    }
    let Some(name) = dir.file_name().and_then(|n| n.to_str()) else {
        return false;
    };
    if !is_test_dir_name(name) {
        return false;
    }
    let catalog_files = children
        .iter()
        .filter(|(child, kind)| {
            matches!(kind, EntryKind::File)
                && is_source_inventory_file(Path::new(child))
                && !is_colocated_test_filename(child)
        })
        .count();
    catalog_files >= MIN_TEST_INDEX_FILES
}

/// True when `dir` lies under a `src`/`lib`/`source`/`pkg/` dir that
/// belongs to a package root — the repo's own, or any module's.
/// Promotes flat source partitions into the inventory tier.
fn has_source_root_ancestor(dir: &Path, ctx: &WalkCtx) -> bool {
    let root = ctx.root();
    let mut parent = dir.parent();
    while let Some(p) = parent {
        if p == root || !p.starts_with(root) {
            return false;
        }
        if (is_source_dir(p) || is_go_pkg_wrapper(p))
            && p.parent().is_some_and(|owner| {
                // The repo's own `src/` promotes everything beneath it;
                // a module's `src/` promotes only its catalogs.
                owner == root
                    || (ctx.fs_state().entry_count(dir, ctx.dir_filter())
                        >= MODULE_SOURCE_ROOT_MIN_ENTRIES
                        && is_package_root_dir(owner, ctx.dir_filter()))
            })
        {
            return true;
        }
        parent = p.parent();
    }
    false
}

/// Entries a directory under a *module's* source root needs before its
/// listing is promoted into the inventory tier.
///
/// The repo has one `src/`, so promoting everything under it costs a
/// bounded number of listings. A monorepo has one per package, and
/// promoting every directory beneath every package floods the early
/// budget with small leaf partitions — measured at 35 listings before
/// the first line of code in a six-package monorepo, and ~570 tokens
/// of component directories from an embedded frontend in another.
/// Those listings are individually cheap, which is exactly why they
/// win the `value/cost^k` race and exactly why they are the wrong
/// thing to buy first: at 1000–2000 tokens there is no budget left for
/// the files they name, so the listing is paid for and nothing comes
/// back. By 3000 the files start landing and it turns positive.
///
/// Requiring the listing to be a *catalog* — a package's API
/// partition, not a leaf of four files — keeps the promotion for the
/// directories whose names are worth the tokens.
///
/// Swept 10 / 16 / 20 / 25 / 35 across the budget grid. The response
/// is a monotone trade curve, not a peak: raising it recovers early
/// budgets and gives back some of the 3000–6240 gain, and by 35 the
/// promotion is inert on the corpus. This value is therefore a
/// position on that curve, not an optimum — 20 keeps the 3000–6240
/// gain whole. `docs/design-notes.md` records the full grid and what
/// 25 buys if the early budgets are ever weighted higher.
const MODULE_SOURCE_ROOT_MIN_ENTRIES: usize = 20;

/// Manifest filenames that mark a directory as a package root — the
/// point a language's source layout is measured from.
const PACKAGE_MANIFEST_FILES: &[&str] = &[
    "pom.xml",
    "build.gradle",
    "build.gradle.kts",
    "build.sbt",
    "package.json",
    "Cargo.toml",
    "pyproject.toml",
    "setup.py",
    "go.mod",
    "composer.json",
    "Gemfile",
    "Package.swift",
    "pubspec.yaml",
    "mix.exs",
];

/// A directory holding its own package manifest. In a multi-module
/// repo (Maven/Gradle reactors, Cargo workspaces, npm monorepos) the
/// module directory plays the role the repo root plays in a
/// single-module one, so `<module>/src/…` is as much a source root as
/// `<root>/src/…`. Without this a Maven reactor's
/// `gson/src/main/java/com/google/gson/` never registers as a source
/// inventory, its 60-file listing loses the ratio race at depth 7,
/// and nothing inside it is ever discovered.
///
/// Reads through the walk's ignore filter: a manifest git ignores is
/// not this repository's statement about its own layout.
fn is_package_root_dir(dir: &Path, filter: &DirFilter) -> bool {
    let entries = list_dir(dir, filter);
    PACKAGE_MANIFEST_FILES
        .iter()
        .any(|manifest| matches!(entries.get(*manifest), Some(EntryKind::File)))
        || entries.iter().any(|(name, kind)| {
            matches!(kind, EntryKind::File)
                && (name.ends_with(".gemspec") || name.ends_with(".csproj"))
        })
}

fn source_inventory_count_uncached(
    state: &FsState,
    dir: &Path,
    target: usize,
    filter: &DirFilter,
) -> usize {
    let Ok(read_dir) = std::fs::read_dir(dir) else {
        return 0;
    };
    let mut count = 0;
    for entry in read_dir.flatten() {
        let Ok(file_type) = entry.file_type() else {
            continue;
        };
        let path = entry.path();
        if filter.excludes(&path, file_type.is_dir()) {
            continue;
        }
        if file_type.is_dir() {
            let name = entry.file_name();
            if !crate::fs_util::should_skip_dir(&name.to_string_lossy()) {
                let child_count = state.source_inventory_count(&path, target, filter);
                count += child_count.min(target.saturating_sub(count));
            }
        } else if file_type.is_file() && is_source_inventory_file(&path) {
            count += 1;
        }
        if count >= target {
            break;
        }
    }
    count.min(target)
}

/// A directory is a source directory because of what its files *are*,
/// not because of which languages this crate happens to parse — a
/// `com/google/gson/` of `.java` is as much a package as a `src/` of
/// `.ts`. The parsed languages are listed here; every other
/// hand-authored source format comes from
/// [`crate::walker::plaintext::SOURCE_TEXT_CODE_EXTENSIONS`].
fn is_source_inventory_file(path: &Path) -> bool {
    // Markdown files count here because docs directories are inventories too:
    // a listing of pages often carries the orientation value.
    const PARSED_INVENTORY_EXTS: &[&str] = &[
        "c", "cc", "cjs", "cpp", "cxx", "go", "h", "hpp", "js", "jsx", "md", "mdx", "mjs", "py",
        "rs", "ts", "tsx",
    ];
    path.extension()
        .and_then(|e| e.to_str())
        .is_some_and(|ext| {
            let lower = ext.to_ascii_lowercase();
            PARSED_INVENTORY_EXTS.contains(&lower.as_str())
                || crate::walker::plaintext::SOURCE_TEXT_LANGUAGE_EXTENSIONS
                    .contains(&lower.as_str())
        })
}

fn inventory_depth_factor(dir: &Path, ctx: &WalkCtx, non_essential: f64) -> f64 {
    // Clamp depth at 2 so nested examples/docs neither disappear nor
    // act like entrypoints; floor non-essential at 0.5.
    let depth = ctx.depth_from_root(dir).min(2);
    crate::value::depth_factor(depth) * non_essential.max(0.5)
}

/// Path-aware exception to the name-only heavy-directory policy. A checked-in
/// Rust module may legitimately be named `build/`; generated build output does
/// not gain traversal merely by containing arbitrary artifacts.
fn should_recurse_dir(dir: &Path, traversal_root: &Path) -> bool {
    let Some(name) = dir.file_name() else {
        return false;
    };
    // Lossy, matching the pre-consolidation call sites: a non-UTF-8
    // directory name is still traversed unless its lossy form is on the
    // skip list.
    let name = name.to_string_lossy();
    if name != "build" {
        return !crate::fs_util::should_skip_dir(&name);
    }
    is_owned_rust_build_dir(dir, traversal_root)
}

/// A `build/` directory is plausibly project-owned Rust source when it has an
/// immediate Rust file and either lives below a conventional source tree or is
/// explicitly paired with the package's `build.rs` script.
fn is_owned_rust_build_dir(dir: &Path, traversal_root: &Path) -> bool {
    // Existence check only — stop at the first Rust file instead of
    // collecting and sorting the whole listing (this runs for every
    // `build/` dir the traversal touches).
    let has_rust_file = std::fs::read_dir(dir).is_ok_and(|entries| {
        entries.flatten().any(|entry| {
            let path = entry.path();
            path.extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext| ext.eq_ignore_ascii_case("rs"))
                && path.is_file()
        })
    });
    if !has_rust_file {
        return false;
    }
    let Some(parent) = dir.parent() else {
        return false;
    };
    if parent.join("build.rs").is_file() {
        return true;
    }
    dir.ancestors()
        .skip(1)
        .take_while(|ancestor| ancestor.starts_with(traversal_root))
        .any(is_source_dir)
}

/// A `test`/`tests`/`spec`/`specs` directory name, case-insensitive.
pub(crate) fn is_test_dir_name(name: &str) -> bool {
    matches!(
        name.to_ascii_lowercase().as_str(),
        "test" | "tests" | "spec" | "specs"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fs_owned_rust_build_dirs_recurse_but_generated_trees_stay_excluded() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path();
        let source_build = root.join("src/build");
        let source_build_child = source_build.join("compile");
        let root_build = root.join("build");
        let target_build = root.join("target/debug/build/generated/src/build");
        std::fs::create_dir_all(&source_build_child).unwrap();
        std::fs::create_dir_all(&root_build).unwrap();
        std::fs::create_dir_all(&target_build).unwrap();
        std::fs::write(source_build.join("mod.rs"), "pub mod compile;\n").unwrap();
        std::fs::write(
            source_build_child.join("compile.rs"),
            "pub fn compile() {}\n",
        )
        .unwrap();
        std::fs::write(root_build.join("probe.rs"), "pub fn probe() {}\n").unwrap();
        std::fs::write(target_build.join("mod.rs"), "pub fn generated() {}\n").unwrap();

        let ctx = WalkCtx::new(root.to_path_buf());
        let before_build_script = files_with_extension_recursive(root, "rs", &ctx);
        assert!(before_build_script.contains(&source_build.join("mod.rs")));
        assert!(before_build_script.contains(&source_build_child.join("compile.rs")));
        assert!(!before_build_script.contains(&root_build.join("probe.rs")));
        assert!(!before_build_script.contains(&target_build.join("mod.rs")));

        std::fs::write(root.join("build.rs"), "fn main() {}\n").unwrap();
        let after_build_script = files_with_extension_recursive(root, "rs", &ctx);
        assert!(after_build_script.contains(&root_build.join("probe.rs")));
        assert!(!after_build_script.contains(&target_build.join("mod.rs")));

        let root_children = expand_subdirs(root, &ctx);
        assert!(root_children.iter().any(|batch| matches!(
            &batch.key,
            BatchKey::Fs(FsKey::DirListing { dir }) if dir == &root_build
        )));
        assert!(!root_children.iter().any(|batch| matches!(
            &batch.key,
            BatchKey::Fs(FsKey::DirListing { dir }) if dir.starts_with(root.join("target"))
        )));
    }

    /// A module's `src/` promotes a catalog partition but not a small
    /// leaf, and only when the manifest that makes it a module is
    /// something the repository actually declares.
    #[test]
    fn fs_module_source_root_promotes_catalogs_and_respects_ignored_manifests() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path();
        let catalog = root.join("packages/lib/src/api");
        let leaf = root.join("packages/lib/src/files");
        std::fs::create_dir_all(&catalog).unwrap();
        std::fs::create_dir_all(&leaf).unwrap();
        std::fs::write(root.join("packages/lib/package.json"), "{}\n").unwrap();
        for i in 0..MODULE_SOURCE_ROOT_MIN_ENTRIES {
            std::fs::write(catalog.join(format!("m{i}.ts")), "export const x = 1;\n").unwrap();
        }
        for i in 0..3 {
            std::fs::write(leaf.join(format!("f{i}.ts")), "export const y = 1;\n").unwrap();
        }

        let ctx = WalkCtx::new(root.to_path_buf());
        assert!(has_source_root_ancestor(&catalog, &ctx));
        assert!(
            !has_source_root_ancestor(&leaf, &ctx),
            "a four-file leaf is not a package's API partition"
        );

        // Same tree, but the manifest is gitignored: a manifest git
        // hides is not the repository's statement about its layout,
        // and the walker never lists it either.
        let ignored = tempfile::tempdir().unwrap();
        let iroot = ignored.path();
        let icatalog = iroot.join("packages/lib/src/api");
        std::fs::create_dir_all(&icatalog).unwrap();
        std::fs::create_dir_all(iroot.join(".git")).unwrap();
        std::fs::write(iroot.join(".gitignore"), "package.json\n").unwrap();
        std::fs::write(iroot.join("packages/lib/package.json"), "{}\n").unwrap();
        for i in 0..MODULE_SOURCE_ROOT_MIN_ENTRIES {
            std::fs::write(icatalog.join(format!("m{i}.ts")), "export const x = 1;\n").unwrap();
        }
        let ictx = WalkCtx::new(iroot.to_path_buf());
        assert!(!has_source_root_ancestor(&icatalog, &ictx));

        // Controlled comparison: the same tree with the manifest
        // un-ignored does promote, so the assertion above is about the
        // ignore rules and not about the tree's shape.
        std::fs::write(iroot.join(".gitignore"), "\n").unwrap();
        let visible_ctx = WalkCtx::new(iroot.to_path_buf());
        assert!(has_source_root_ancestor(&icatalog, &visible_ctx));
    }

    /// Roster mass is what keeps a listing's `value/cost^k` from falling
    /// as the directory grows. A wide subsystem directory outside any
    /// `src/` gets it; the same width made of subdirectories does not,
    /// because those rows are pointers to listings bought separately.
    #[test]
    fn fs_catalog_roster_mass_counts_terminal_entries_only() {
        let mut files = BTreeMap::new();
        let mut dirs = BTreeMap::new();
        for i in 0..40 {
            files.insert(format!("f{i}.c"), EntryKind::File);
            dirs.insert(format!("d{i}"), EntryKind::Directory);
        }
        let wide_catalog = catalog_roster_mass_factor(&files);
        assert!(wide_catalog > 1.0, "a wide file catalog is neutralized");
        assert!(
            wide_catalog < crate::value::roster_mass_factor(files.len()),
            "outside the inventory tier the neutralization is partial"
        );
        assert_eq!(catalog_roster_mass_factor(&dirs), 1.0);
    }
}
