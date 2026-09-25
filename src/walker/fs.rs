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

use crate::batch::{Batch, FsKey};
use crate::content::{BatchContent, FsEntries, FsGroup};
use crate::fs_util::{DirFilter, EntryKind, list_dir};

use super::{WalkCtx, file_depth_factor, path_depth_factor};

/// Seed: list the root directory.
pub fn seed(ctx: &WalkCtx) -> Vec<Batch> {
    dir_listing_batch(ctx.root().to_path_buf(), ctx)
        .into_iter()
        .collect()
}

/// Subdirectory listings for the just-scheduled dir's listing.
pub fn expand_subdirs(dir: &Path, ctx: &WalkCtx) -> Vec<Batch> {
    let children = list_dir(dir, ctx.dir_filter());
    let mut out = Vec::new();
    for (name, kind) in children.iter() {
        if matches!(kind, EntryKind::Directory)
            && !crate::fs_util::should_skip_dir(name)
            && let Some(batch) = dir_listing_batch(dir.join(name), ctx)
        {
            out.push(batch);
        }
    }
    out
}

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

fn dir_listing_batch(dir: PathBuf, ctx: &WalkCtx) -> Option<Batch> {
    let children = list_dir(&dir, ctx.dir_filter());
    if children.is_empty() {
        return None;
    }
    let value = dir_listing_value(&dir, &children, ctx);
    let paths: Vec<PathBuf> = children.keys().map(PathBuf::from).collect();
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

/// Value of a directory listing before its location prior —
/// classification moves the prior, not this base. Set by a full-corpus
/// sweep; see `git show a90ee9b6:docs/design-notes.md`.
const LISTING_VALUE: f64 = 1300.63;

fn dir_listing_value(dir: &Path, children: &BTreeMap<String, EntryKind>, ctx: &WalkCtx) -> f64 {
    let module_source_dir = is_module_source_dir(dir);
    let source_dir = is_source_dir(dir) || is_go_pkg_wrapper(dir);
    let non_essential = ctx.non_essential_factor(dir);
    let supporting_source_dir = non_essential < 1.0 && (source_dir || module_source_dir);
    let under_root_source_ancestor = has_source_root_ancestor(dir, ctx);
    // A catalog of source files that no name or entrypoint marks as a
    // source dir: a flat partition of the package (`lib/helpers/`), or an
    // inventory inside tests, examples or docs.
    let source_inventory_dir = !source_dir
        && !module_source_dir
        && is_source_inventory_dir(dir, ctx)
        && (non_essential < 1.0 || under_root_source_ancestor);
    // A partition under the repository's source root names part of the
    // package's API wherever it sits, so it prices like depth 1.
    let depth = if source_inventory_dir && under_root_source_ancestor {
        file_depth_factor(dir, ctx, true)
    } else if supporting_source_dir || source_inventory_dir {
        inventory_depth_factor(dir, ctx, non_essential)
    } else if module_source_dir {
        file_depth_factor(dir, ctx, true)
    } else {
        path_depth_factor(dir, ctx)
    };
    // A top-level directory's listing is part of the repo map whatever
    // the directory holds; the non-essential discount is for its contents.
    let depth = if ctx.depth_from_root(dir) == 1 {
        depth.max(0.5)
    } else {
        depth
    };
    // Without the roster factor a source inventory's ratio falls with its
    // length against tiny sibling listings.
    let fanout = if source_inventory_dir {
        crate::value::roster_mass_factor(children.len())
    } else {
        1.0
    };
    // A catalog parent's listing already names every child, so the
    // children's own listings are deferred, not dropped.
    let catalog_child_factor = if parent_is_high_fanout_catalog(dir, ctx) {
        CATALOG_CHILD_LISTING_SUPPRESSION
    } else {
        1.0
    };
    LISTING_VALUE * depth * fanout * catalog_child_factor
}

/// Min child-directory count for a parent to count as a "catalog" whose
/// per-child listings are redundant with its own listing.
const CATALOG_PARENT_MIN_CHILD_DIRS: usize = 10;
/// Deferral multiplier applied to a redundant catalog child's listing —
/// low enough to push it past the primary budget, non-zero so it stays
/// reachable at large budgets.
const CATALOG_CHILD_LISTING_SUPPRESSION: f64 = 0.05;

/// True when `dir`'s parent is a high-fanout source-inventory catalog:
/// a directory of many uniform child dirs whose own listing enumerates
/// every child.
/// Named `src`/`lib`/`pkg` roots and package module dirs (with an
/// `index.*`/`__init__.py`/`mod.rs` entrypoint) are deliberately NOT
/// catalogs — their children are first-class modules, not catalog leaves.
fn parent_is_high_fanout_catalog(dir: &Path, ctx: &WalkCtx) -> bool {
    // The walk root has no parent *inside the walk*. Without this the
    // probe reads the directory the root happens to sit in — so
    // `precis ~/projects/myrepo` suppresses the root listing whenever
    // `~/projects` holds ten-plus sibling checkouts, making the summary
    // depend on files the walk never looks at.
    if dir == ctx.root() {
        return false;
    }
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
    ctx.fs_state.child_dir_count(parent, ctx.dir_filter()) >= CATALOG_PARENT_MIN_CHILD_DIRS
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

/// A package module: a directory with its own entry file (`index.ts`,
/// `mod.rs`, `__init__.py`) or a sibling file of the same stem
/// (`foo.rs` next to `foo/`).
fn is_module_source_dir(dir: &Path) -> bool {
    has_module_entrypoint(dir) || has_module_sibling_file(dir)
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

#[derive(Default)]
pub(in crate::walker) struct FsState {
    source_inventory_counts: RefCell<HashMap<PathBuf, usize>>,
    child_dir_counts: RefCell<HashMap<PathBuf, usize>>,
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

    /// Count of immediate subdirectories of `dir`, cached per parent:
    /// `parent_is_high_fanout_catalog` asks about the same parent once
    /// per child, so recounting would be O(N²) in the parent's entries.
    pub(in crate::walker) fn child_dir_count(&self, dir: &Path, filter: &DirFilter) -> usize {
        if let Some(count) = self.child_dir_counts.borrow().get(dir).copied() {
            return count;
        }
        let count = list_dir(dir, filter)
            .values()
            .filter(|kind| matches!(kind, EntryKind::Directory))
            .count();
        self.child_dir_counts
            .borrow_mut()
            .insert(dir.to_path_buf(), count);
        count
    }
}

fn is_source_inventory_dir(dir: &Path, ctx: &WalkCtx) -> bool {
    const MIN_SOURCE_FILES: usize = 3;
    ctx.fs_state
        .source_inventory_count(dir, MIN_SOURCE_FILES, ctx.dir_filter())
        >= MIN_SOURCE_FILES
}

/// True when `dir` lies under the repository's own top-level
/// `src`/`lib`/`source`/`pkg/` dir. Promotes flat source partitions
/// into the inventory tier.
fn has_source_root_ancestor(dir: &Path, ctx: &WalkCtx) -> bool {
    let Ok(relative) = dir.strip_prefix(ctx.root()) else {
        return false;
    };
    let mut components = relative.components();
    let Some(top) = components.next() else {
        return false;
    };
    let top = ctx.root().join(top);
    components.next().is_some() && (is_source_dir(&top) || is_go_pkg_wrapper(&top))
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
