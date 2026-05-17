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

/// Subdirectory listings for the dir whose listing was just scheduled.
/// File-based batches are emitted by per-language walkers; the FS walker
/// only owns directory recursion.
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

/// Enumerate the files under `dir` (non-recursive) matching an extension.
/// Returns absolute paths. Used by per-language walkers to build cross-file
/// batch scopes without opening any file.
pub fn files_with_extension(dir: &Path, ext: &str) -> Vec<PathBuf> {
    files_with_any_extension(dir, &[ext])
}

/// Like [`files_with_extension`] but accepts any of several extensions in a
/// single `read_dir` pass — convenient for walkers that handle paired
/// extensions (e.g. `.ts` + `.tsx`).
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

/// Recursively walk `dir` for files whose extension matches (case-insensitive).
/// Used for cross-file batches that span subdirectories (`src/kv/mod.rs` etc).
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
    let sibling_module_dir = is_sibling_module_source_dir(dir, has_module_sibling_file(dir));
    let module_source_dir = is_module_source_dir(dir, sibling_module_dir);
    let source_dir = is_source_dir(dir);
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
    } else if src_of_sibling_modules || sibling_module_dir || module_source_dir {
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
    let small_listing_factor = if module_source_dir || src_of_sibling_modules || sibling_module_dir
    {
        // Module-source directories (htmy/renderer/, sps-core/src/install/)
        // anchor on their own listings — keep them at full weight.
        1.0
    } else {
        small_listing_decay(children.len(), ctx.depth_from_root(dir))
    };
    mix_signals(cat, fu, ztu, depth) * small_listing_factor
}

/// Damp deeply-nested directory listings whose tiny entry count makes them
/// redundant with their parent listing. A `crates/foo/` directory containing
/// just `Cargo.toml` + `src/` adds nothing the parent `crates/` listing
/// hasn't already named — at depth ≥ 2, it's bibliographic noise. The
/// damp does not apply at the root or its direct children, where listings
/// are the orientation surface.
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
        .is_some_and(|name| matches!(name, "src" | "lib"))
}

fn has_module_entrypoint(dir: &Path) -> bool {
    JS_MODULE_ENTRYPOINT_FILES
        .iter()
        .chain(NON_JS_MODULE_ENTRYPOINT_FILES.iter())
        .any(|name| dir.join(name).is_file())
}

/// Go-specific module-source detection. A directory is a Go subpackage
/// when it contains multiple non-test `.go` files (the language rule
/// says any `.go` file declares a package, but small helper dirs are
/// usually plumbing rather than an API-surface partition; the file-
/// count threshold is calibrated against the divergence corpus). The
/// module-tier promotion is restricted to **root-level** subpackages
/// (the dir's parent contains a `go.mod`): NS authors anchor on these
/// as the package's API-surface partition (gin's `binding`/`render`,
/// lo's `it`, beszel's `agent`), on par with Rust's `mod.rs`-bearing
/// subdirs. Deeper Go subdirs and small helper dirs shouldn't crowd
/// the early budget — the parent listing already names them.
fn is_go_module_subpackage(dir: &Path) -> bool {
    const MIN_GO_FILES: usize = 5;
    let Some(parent) = dir.parent() else {
        return false;
    };
    if !parent.join("go.mod").is_file() {
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

fn is_module_source_dir(dir: &Path, sibling_module_dir: bool) -> bool {
    if is_type_surface_dir(dir) {
        return false;
    }
    let entrypoint_module = has_module_entrypoint(dir)
        && (dir.parent().is_some_and(is_source_dir) || has_python_module_entrypoint(dir));
    entrypoint_module || sibling_module_dir || is_go_module_subpackage(dir)
}

fn is_type_surface_dir(dir: &Path) -> bool {
    dir.file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|name| matches!(name, "type" | "types" | "typings"))
}

fn has_python_module_entrypoint(dir: &Path) -> bool {
    dir.join("__init__.py").is_file()
}

fn is_sibling_module_source_dir(dir: &Path, has_module_sibling_file: bool) -> bool {
    has_module_sibling_file && !is_type_surface_dir(dir)
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

/// True when `dir` has an ancestor whose basename is a recognized
/// source directory (`src` / `lib`) sitting directly under the seed
/// root. Used to promote flat source partitions (no entrypoint, no
/// sibling module) into the inventory tier when their listing is
/// what names the package's API.
///
/// The shallowness gate matters: in a single-package layout (`./lib/…`,
/// `./src/…`), every descendant is part of the one declared API
/// partition the parent listing has already named, and inventory
/// promotion surfaces the leaf partitions that lack their own anchor.
/// In multi-package layouts (`./crate-a/src/…`, deep CUDA shims) the
/// outer crate / shim already gates its own contents — promoting
/// internal leaves crowds higher-priority orientation.
fn has_root_adjacent_source_ancestor(dir: &Path, ctx: &WalkCtx) -> bool {
    let root = ctx.root();
    let mut parent = dir.parent();
    while let Some(p) = parent {
        if p == root || !p.starts_with(root) {
            return false;
        }
        if is_source_dir(p) && p.parent() == Some(root) {
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
    // Supporting inventories are useful orientation even below real source
    // content; keep the discount, but not the full 0.2 suppression.
    // Clamp at depth 2 so nested examples/docs don't behave like root
    // entrypoints, but also don't disappear solely because of layout depth.
    let depth = ctx.depth_from_root(dir).min(2);
    crate::value::depth_factor(depth) * non_essential.max(0.5)
}

/// Directories the walker never recurses into. Matches common
/// heavy/generated trees. Note: these directories still appear in
/// listings (via `fs_util::list_dir`); this only affects walker
/// traversal and per-language file enumeration.
pub(crate) fn should_skip_dir(name: &str) -> bool {
    matches!(
        name,
        "target" | "node_modules" | ".git" | "dist" | "build" | ".next" | "__pycache__"
    )
}
