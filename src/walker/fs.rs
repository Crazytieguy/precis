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
use crate::fs_util::EntryKind;
pub use crate::fs_util::list_dir;
use crate::value::mix_signals;

use super::{WalkCtx, file_depth_factor, path_depth_factor};

/// Seed: list the root directory.
pub fn seed(ctx: &WalkCtx) -> Vec<Batch<BatchKey>> {
    dir_listing_batch(ctx.root().to_path_buf(), ctx)
        .into_iter()
        .collect()
}

/// Subdirectory listings for the just-scheduled dir's listing.
pub fn expand_subdirs(dir: &Path, ctx: &WalkCtx) -> Vec<Batch<BatchKey>> {
    let children = list_dir(dir);
    let mut out = Vec::new();
    for (name, kind) in children {
        if matches!(kind, EntryKind::Directory)
            && !should_skip_dir(&name)
            && let Some(batch) = dir_listing_batch(dir.join(&name), ctx)
        {
            out.push(batch);
        }
    }
    out
}

// ---- additional helpers ----

/// Files in `dir` (non-recursive) whose extension matches.
pub fn files_with_extension(dir: &Path, ext: &str) -> Vec<PathBuf> {
    files_with_any_extension(dir, &[ext])
}

/// Files in `dir` matching any of `exts`, in one `read_dir` pass.
pub fn files_with_any_extension(dir: &Path, exts: &[&str]) -> Vec<PathBuf> {
    let Ok(read_dir) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut out: Vec<PathBuf> = read_dir
        .flatten()
        .filter_map(|e| {
            let path = e.path();
            if !path.is_file() {
                return None;
            }
            let actual = path.extension().and_then(|e| e.to_str())?;
            exts.iter()
                .any(|ext| actual.eq_ignore_ascii_case(ext))
                .then_some(path)
        })
        .collect();
    out.sort();
    out
}

/// Recursively walk `dir` for files with `ext` (case-insensitive).
pub fn files_with_extension_recursive(dir: &Path, ext: &str) -> Vec<PathBuf> {
    let mut out = Vec::new();
    walk_files_recursive(dir, ext, &mut out);
    out.sort();
    out
}

fn walk_files_recursive(dir: &Path, ext: &str, out: &mut Vec<PathBuf>) {
    let Ok(read_dir) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in read_dir.flatten() {
        let path = entry.path();
        if path.is_dir() {
            let name = entry.file_name().to_string_lossy().into_owned();
            if !should_skip_dir(&name) {
                walk_files_recursive(&path, ext, out);
            }
        } else if path
            .extension()
            .and_then(|e| e.to_str())
            .is_some_and(|actual| actual.eq_ignore_ascii_case(ext))
        {
            out.push(path);
        }
    }
}

fn dir_listing_batch(dir: PathBuf, ctx: &WalkCtx) -> Option<Batch<BatchKey>> {
    let children = list_dir(&dir);
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
    let under_root_source_ancestor = has_root_adjacent_source_ancestor(dir, ctx);
    let source_inventory_dir = !source_dir
        && !module_source_dir
        && (non_essential < 1.0 || under_root_source_ancestor)
        && is_source_inventory_dir(dir, ctx);
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
        (0.75, 0.55, 0.35)
    } else if source_dir || source_inventory_dir || readme_cited {
        // README-cited dirs (an `examples/` directory the README links
        // canonical scripts from) are part of the documented public
        // surface; treat them on par with source-inventory dirs so the
        // listing schedules early enough for per-file batches inside it
        // to compete in the early budget.
        (0.6, 0.5, 0.3)
    } else {
        (0.5, 0.45, 0.25)
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
    let small_listing_factor = if module_source_dir || src_of_sibling_modules {
        // Module-source directories (htmy/renderer/, sps-core/src/install/)
        // anchor on their own listings — keep them at full weight.
        1.0
    } else {
        small_listing_decay(children.len(), ctx.depth_from_root(dir))
    };
    // Source-inventory catalogs keep size-neutral ranking: a flat
    // partition's complete listing is the API map NS authors anchor on,
    // and without the factor an N-entry listing's ratio falls as N^-k
    // against tiny same-tier sibling listings. Gated to the inventory
    // tier only — boosting plain `src/`-named or module dirs lets the
    // big listing itself displace NS-wanted content (measured: soluna
    // −0.210, beszel −0.073 with the looser gate).
    let fanout = if source_inventory_dir {
        crate::value::roster_mass_factor(children.len())
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
    mix_signals(cat, fu, ztu, depth) * small_listing_factor * fanout * catalog_child_factor
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
    let under_source_ancestor = has_root_adjacent_source_ancestor(parent, ctx);
    if ctx.non_essential_factor(parent) >= 1.0 && !under_source_ancestor {
        return false;
    }
    if !is_source_inventory_dir(parent, ctx) {
        return false;
    }
    child_dir_count(parent) >= CATALOG_PARENT_MIN_CHILD_DIRS
}

/// Count of immediate subdirectories of `dir`.
fn child_dir_count(dir: &Path) -> usize {
    list_dir(dir)
        .values()
        .filter(|kind| matches!(kind, EntryKind::Directory))
        .count()
}

/// Damp deeply-nested tiny directory listings — they're redundant
/// with the parent listing at depth ≥ 2.
fn small_listing_decay(child_count: usize, depth: usize) -> f64 {
    if depth < 2 {
        return 1.0;
    }
    match child_count {
        0..=2 => 0.45,
        3 => 0.7,
        _ => 1.0,
    }
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

pub(crate) fn is_source_dir(dir: &Path) -> bool {
    dir.file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|name| matches!(name, "src" | "lib" | "source"))
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
}

impl FsState {
    pub(in crate::walker) fn source_inventory_count(&self, dir: &Path, target: usize) -> usize {
        if let Some(count) = self.source_inventory_counts.borrow().get(dir).copied() {
            return count;
        }
        let count = source_inventory_count_uncached(self, dir, target);
        self.source_inventory_counts
            .borrow_mut()
            .insert(dir.to_path_buf(), count);
        count
    }
}

fn is_source_inventory_dir(dir: &Path, ctx: &WalkCtx) -> bool {
    const MIN_SOURCE_FILES: usize = 3;
    ctx.fs_state().source_inventory_count(dir, MIN_SOURCE_FILES) >= MIN_SOURCE_FILES
}

/// True when `dir` lies under a root-level `src`/`lib`/`source`/`pkg/` dir.
/// Promotes flat source partitions into the inventory tier; the
/// shallowness gate avoids crowding in multi-package layouts.
fn has_root_adjacent_source_ancestor(dir: &Path, ctx: &WalkCtx) -> bool {
    let root = ctx.root();
    let mut parent = dir.parent();
    while let Some(p) = parent {
        if p == root || !p.starts_with(root) {
            return false;
        }
        if (is_source_dir(p) || is_go_pkg_wrapper(p)) && p.parent() == Some(root) {
            return true;
        }
        parent = p.parent();
    }
    false
}

fn source_inventory_count_uncached(state: &FsState, dir: &Path, target: usize) -> usize {
    let Ok(read_dir) = std::fs::read_dir(dir) else {
        return 0;
    };
    let mut count = 0;
    for entry in read_dir.flatten() {
        let Ok(file_type) = entry.file_type() else {
            continue;
        };
        let path = entry.path();
        if file_type.is_dir() {
            let name = entry.file_name();
            if !should_skip_dir(&name.to_string_lossy()) {
                let child_count = state.source_inventory_count(&path, target);
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

fn is_source_inventory_file(path: &Path) -> bool {
    // Markdown files count here because docs directories are inventories too:
    // a listing of pages often carries the orientation value.
    const SOURCE_INVENTORY_EXTS: &[&str] = &[
        "c", "cc", "cjs", "cpp", "cxx", "go", "h", "hpp", "js", "jsx", "md", "mdx", "mjs", "py",
        "rs", "ts", "tsx",
    ];
    path.extension()
        .and_then(|e| e.to_str())
        .is_some_and(|ext| {
            SOURCE_INVENTORY_EXTS
                .iter()
                .any(|candidate| ext.eq_ignore_ascii_case(candidate))
        })
}

fn inventory_depth_factor(dir: &Path, ctx: &WalkCtx, non_essential: f64) -> f64 {
    // Clamp depth at 2 so nested examples/docs neither disappear nor
    // act like entrypoints; floor non-essential at 0.5.
    let depth = ctx.depth_from_root(dir).min(2);
    crate::value::depth_factor(depth) * non_essential.max(0.5)
}

/// Directories the walker never recurses into — heavy/generated trees.
/// They still appear in listings; only walker traversal is affected.
pub(crate) fn should_skip_dir(name: &str) -> bool {
    matches!(
        name,
        "target" | "node_modules" | ".git" | "dist" | "build" | ".next" | "__pycache__"
    )
}
