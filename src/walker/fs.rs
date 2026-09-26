//! Filesystem walker: directory listings and their value, plus
//! per-extension file enumeration for the other walkers. Only does
//! `read_dir` — never reads file contents. Pure listing lives in
//! [`crate::fs_util::list_dir`]; this module holds walker-specific
//! policy (which directories recurse, how a directory's role prices its
//! listing).

use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    path::{Path, PathBuf},
};

use crate::batch::{Batch, FsKey};
use crate::content::{BatchContent, FsEntries, FsGroup};
use crate::fs_util::{DirFilter, EntryKind, PROBE_ENTRY_CAP, list_dir};

use super::{WalkCtx, file_depth_factor, path_depth_factor};

/// Seed: list the root directory.
pub fn seed(ctx: &WalkCtx) -> Vec<Batch> {
    dir_listing_batch(ctx.root().to_path_buf(), ctx)
        .into_iter()
        .collect()
}

/// Listings of the subdirectories a just-scheduled listing names.
pub fn expand_listed(key: &FsKey, ctx: &WalkCtx) -> Vec<Batch> {
    let FsKey::DirListing { dir } = key;
    list_dir(dir, ctx.dir_filter())
        .iter()
        .filter(|(_, kind)| matches!(kind, EntryKind::Directory))
        .map(|(name, _)| dir.join(name))
        .filter(|child| should_recurse_dir(child))
        .filter_map(|child| dir_listing_batch(child, ctx))
        .collect()
}

/// Files `dir`'s listing shows whose extension matches any of `exts`,
/// in name order.
pub fn files_with_any_extension(dir: &Path, exts: &[&str], ctx: &WalkCtx) -> Vec<PathBuf> {
    list_dir(dir, ctx.dir_filter())
        .iter()
        .filter(|(name, kind)| {
            matches!(kind, EntryKind::File)
                && Path::new(name)
                    .extension()
                    .and_then(|actual| actual.to_str())
                    .is_some_and(|actual| exts.iter().any(|ext| actual.eq_ignore_ascii_case(ext)))
        })
        .map(|(name, _)| dir.join(name))
        .collect()
}

/// Listing of `dir`, run on through every directory that holds only one
/// subdirectory (`src/main/java/org/acme/`): such a listing names one
/// path segment, so it is bought and keyed with the first listing below
/// it that names more, at the lower of the two listings' values.
fn dir_listing_batch(dir: PathBuf, ctx: &WalkCtx) -> Option<Batch> {
    let mut dir = dir;
    let mut children = list_dir(&dir, ctx.dir_filter());
    if children.is_empty() {
        return None;
    }
    let chain_head_value = dir_listing_value(&dir, &children, ctx);
    let mut groups = Vec::new();
    loop {
        groups.push(FsGroup {
            parent: dir.clone(),
            entries: FsEntries::Listed(listed_entries(&children).map(PathBuf::from).collect()),
        });
        let Some((name, EntryKind::Directory)) = children.iter().next() else {
            break;
        };
        let only_child = dir.join(name);
        let grandchildren = list_dir(&only_child, ctx.dir_filter());
        if children.len() > 1 || grandchildren.is_empty() || !should_recurse_dir(&only_child) {
            break;
        }
        dir = only_child;
        children = grandchildren;
    }
    let value = chain_head_value.min(dir_listing_value(&dir, &children, ctx));
    if value == 0.0 {
        return None;
    }
    Some(Batch {
        key: FsKey::DirListing { dir }.into(),
        predecessor: None,
        content: BatchContent::Fs { groups },
        value,
    })
}

/// Entries a listing of `children` names: all but asset sidecars.
fn listed_entries(children: &BTreeMap<String, EntryKind>) -> impl Iterator<Item = &String> {
    children.keys().filter(|name| !is_sidecar(name, children))
}

/// A game engine's per-asset metadata file (`player.png.meta`,
/// `player.png.import`) beside the asset it describes. A listing leaves
/// it out, and its `…` row marks the gap.
fn is_sidecar(name: &str, siblings: &BTreeMap<String, EntryKind>) -> bool {
    [".meta", ".import", ".uid"].iter().any(|suffix| {
        name.strip_suffix(suffix)
            .is_some_and(|asset| siblings.contains_key(asset))
    })
}

/// Value of a directory listing before its location prior —
/// classification moves the prior, not this base.
const LISTING_VALUE: f64 = 1230.0;

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
        && ctx.fs_state.holds_source(dir, ctx.dir_filter())
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
    LISTING_VALUE * depth * fanout * catalog_child_factor * media_roster_factor(children)
}

/// Fewest media files for a listing to count as a media roster.
const MEDIA_ROSTER_MIN_FILES: usize = 11;

/// A listing of many images, fonts, audio or video files is priced by
/// its share of other entries: the media names say what the pictures
/// are, not what the project is, and a roster of nothing but media is
/// never bought. A few media files, a logo and a screenshot, cost little
/// and keep full value.
fn media_roster_factor(children: &BTreeMap<String, EntryKind>) -> f64 {
    let listed: Vec<&String> = listed_entries(children).collect();
    let media = listed
        .iter()
        .filter(|name| matches!(children[name.as_str()], EntryKind::File) && is_media_file(name))
        .count();
    if media < MEDIA_ROSTER_MIN_FILES {
        return 1.0;
    }
    1.0 - media as f64 / listed.len() as f64
}

const MEDIA_EXTENSIONS: &[&str] = &[
    "png", "jpg", "jpeg", "gif", "webp", "svg", "ico", "icns", "bmp", "tif", "tiff", "psd", "ttf",
    "otf", "woff", "woff2", "eot", "mp3", "ogg", "wav", "mp4", "webm", "mov",
];

fn is_media_file(name: &str) -> bool {
    Path::new(name)
        .extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| {
            MEDIA_EXTENSIONS
                .iter()
                .any(|media| ext.eq_ignore_ascii_case(media))
        })
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
    if !ctx.fs_state.holds_source(parent, ctx.dir_filter()) {
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
    holds_source: RefCell<HashMap<PathBuf, bool>>,
    child_dir_counts: RefCell<HashMap<PathBuf, usize>>,
}

impl FsState {
    /// Whether `dir` holds a source file at any depth, reading at most
    /// [`PROBE_ENTRY_CAP`] entries.
    pub(in crate::walker) fn holds_source(&self, dir: &Path, filter: &DirFilter) -> bool {
        let mut entry_budget = PROBE_ENTRY_CAP;
        self.budgeted_holds_source(dir, filter, &mut entry_budget)
            .unwrap_or(false)
    }

    /// `None` when the budget ran out first. Only final answers are
    /// cached: one cut short depends on what the probe read first.
    fn budgeted_holds_source(
        &self,
        dir: &Path,
        filter: &DirFilter,
        entry_budget: &mut usize,
    ) -> Option<bool> {
        if let Some(&holds) = self.holds_source.borrow().get(dir) {
            return Some(holds);
        }
        let holds = holds_source_uncached(self, dir, filter, entry_budget)?;
        self.holds_source
            .borrow_mut()
            .insert(dir.to_path_buf(), holds);
        Some(holds)
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

fn holds_source_uncached(
    state: &FsState,
    dir: &Path,
    filter: &DirFilter,
    entry_budget: &mut usize,
) -> Option<bool> {
    let Ok(read_dir) = std::fs::read_dir(dir) else {
        return Some(false);
    };
    for entry in read_dir.flatten() {
        if *entry_budget == 0 {
            return None;
        }
        *entry_budget -= 1;
        let Ok(file_type) = entry.file_type() else {
            continue;
        };
        let path = entry.path();
        if filter.excludes(&path, file_type.is_dir()) {
            continue;
        }
        let holds = if file_type.is_dir() {
            !crate::fs_util::should_skip_dir(&entry.file_name().to_string_lossy())
                && state.budgeted_holds_source(&path, filter, entry_budget)?
        } else {
            file_type.is_file() && is_source_inventory_file(&path)
        };
        if holds {
            return Some(true);
        }
    }
    Some(false)
}

/// A directory is a source directory because of what its files *are*,
/// not because of which languages this crate happens to parse — a
/// `com/google/gson/` of `.java` is as much a package as a `src/` of
/// `.ts`. Markdown counts too: a docs directory is an inventory, and a
/// listing of its pages often carries the orientation value.
fn is_source_inventory_file(path: &Path) -> bool {
    super::language_group(path).is_some()
        || path
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("md") || ext.eq_ignore_ascii_case("mdx"))
}

fn inventory_depth_factor(dir: &Path, ctx: &WalkCtx, non_essential: f64) -> f64 {
    // Clamp depth at 2 so nested examples/docs neither disappear nor
    // act like entrypoints; floor non-essential at 0.5.
    let depth = ctx.depth_from_root(dir).min(2);
    crate::value::depth_factor(depth) * non_essential.max(0.5)
}

/// Heavy-directory names block traversal, except a `build/` that holds
/// Rust source: a checked-in module may be named `build`, while Cargo's
/// own output lives under `target/`, which the walk never enters. A
/// translated mirror is named, not listed: its entries repeat the names
/// of the directory it translates.
fn should_recurse_dir(dir: &Path) -> bool {
    let Some(name) = dir.file_name() else {
        return false;
    };
    let name = name.to_string_lossy();
    if is_locale_mirror(dir, &name) {
        return false;
    }
    if name != "build" {
        return !crate::fs_util::should_skip_dir(&name);
    }
    std::fs::read_dir(dir).is_ok_and(|entries| {
        entries.flatten().any(|entry| {
            let path = entry.path();
            path.extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext| ext.eq_ignore_ascii_case("rs"))
                && path.is_file()
        })
    })
}

/// One of several translations named after the directory they mirror:
/// `pages.ar/`, `pages.pt_BR/` beside `pages/`.
fn is_locale_mirror(dir: &Path, name: &str) -> bool {
    let (Some(stem), Some(parent)) = (locale_mirror_stem(name), dir.parent()) else {
        return false;
    };
    let mirrors = std::fs::read_dir(parent)
        .into_iter()
        .flatten()
        .flatten()
        .filter(|entry| entry.file_name().to_str().and_then(locale_mirror_stem) == Some(stem))
        .count();
    mirrors >= 2 && parent.join(stem).is_dir()
}

/// `pages` of `pages.ar`, `pages.pt_BR`, `pages.zh-Hant`.
fn locale_mirror_stem(name: &str) -> Option<&str> {
    let (stem, code) = name.rsplit_once('.').filter(|(stem, _)| !stem.is_empty())?;
    let (language, region) = code
        .split_once(['_', '-'])
        .map_or((code, None), |(language, region)| (language, Some(region)));
    let is_locale = language.len() == 2
        && language.bytes().all(|byte| byte.is_ascii_lowercase())
        && region.is_none_or(|region| {
            (2..=4).contains(&region.len())
                && region.bytes().all(|byte| byte.is_ascii_alphanumeric())
        });
    is_locale.then_some(stem)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::batch::BatchKey;

    #[test]
    fn fs_listing_leaves_out_asset_sidecars() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path();
        for name in [
            "boat.png",
            "boat.png.meta",
            "orphan.meta",
            "scene.tscn.import",
        ] {
            std::fs::write(root.join(name), "").unwrap();
        }
        let ctx = WalkCtx::new(root.to_path_buf());
        let BatchContent::Fs { groups } = &seed(&ctx)[0].content else {
            panic!("a listing is Fs content");
        };
        let FsEntries::Listed(entries) = &groups[0].entries else {
            panic!("a listing names its entries");
        };
        let expected: Vec<PathBuf> = ["boat.png", "orphan.meta", "scene.tscn.import"]
            .map(PathBuf::from)
            .into();
        assert_eq!(entries, &expected);
    }

    #[test]
    fn fs_media_rosters_are_priced_by_their_other_entries() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path();
        let write = |dir: &str, names: &[String]| {
            std::fs::create_dir_all(root.join(dir)).unwrap();
            for name in names {
                std::fs::write(root.join(dir).join(name), "").unwrap();
            }
        };
        let shots: Vec<String> = (0..MEDIA_ROSTER_MIN_FILES)
            .map(|index| format!("shot{index}.png"))
            .collect();
        write("gallery", &shots);
        write("screens", &shots);
        write("screens", &["capture.py".to_string()]);
        write(
            "branding",
            &["logo.svg".to_string(), "screenshot.png".to_string()],
        );
        write("notes", &["a.txt".to_string(), "b.txt".to_string()]);
        let ctx = WalkCtx::new(root.to_path_buf());
        let value = |dir: &str| dir_listing_batch(root.join(dir), &ctx).map(|batch| batch.value);
        assert_eq!(value("gallery"), None);
        assert!(value("screens").unwrap() < value("notes").unwrap() / 10.0);
        assert_eq!(value("branding"), value("notes"));
    }

    #[test]
    fn fs_locale_mirrors_are_named_not_listed() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path();
        for dir in [
            "pages/common",
            "pages.ar/common",
            "pages.pt_BR/common",
            "glossary/node",
            "glossary/node.js",
        ] {
            std::fs::create_dir_all(root.join(dir)).unwrap();
        }
        assert!(should_recurse_dir(&root.join("pages")));
        assert!(!should_recurse_dir(&root.join("pages.ar")));
        assert!(!should_recurse_dir(&root.join("pages.pt_BR")));
        assert!(should_recurse_dir(&root.join("glossary/node.js")));
    }

    #[test]
    fn fs_rust_build_dirs_recurse_but_generated_trees_stay_excluded() {
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

        let lists = |dir: &Path, listed: &Path| {
            let ctx = WalkCtx::new(root.to_path_buf());
            let key = FsKey::DirListing {
                dir: dir.to_path_buf(),
            };
            expand_listed(&key, &ctx).iter().any(|batch| {
                matches!(&batch.key, BatchKey::Fs(FsKey::DirListing { dir }) if dir == listed)
            })
        };
        assert!(lists(&root.join("src"), &source_build));
        assert!(lists(root, &root_build));
        assert!(!lists(root, &root.join("target")));
    }
}
