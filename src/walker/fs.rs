//! Filesystem walker: directory listings and their value, plus
//! per-extension file enumeration for the other walkers. Only does
//! `read_dir` — never reads file contents. Pure listing lives in
//! [`crate::fs_util::list_dir`]; this module holds walker-specific
//! policy (which directories recurse, how a long listing splits, how a
//! directory's role prices its listing).

use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap, HashSet},
    path::{Path, PathBuf},
    rc::Rc,
};

use crate::batch::{Batch, BatchKey, FsKey};
use crate::content::{BatchContent, FsEntries, FsGroup};
use crate::fs_util::{DirFilter, EntryKind, PROBE_ENTRY_CAP, list_dir, lists_file};

use super::{WalkCtx, file_depth_factor, path_depth_factor};

/// Seed: list the root directory.
pub fn seed(ctx: &WalkCtx) -> Vec<Batch> {
    dir_listing_batches(ctx.root().to_path_buf(), ctx)
}

/// Listings of the subdirectories a just-scheduled listing names, and the
/// listed directory once all its entries are listed, for the file
/// walkers: a file's content never renders under a row that isn't there.
/// A third-party directory's listing names the projects it vendors; their
/// own trees are not listed.
pub fn expand_listed<'k>(key: &'k FsKey, ctx: &WalkCtx) -> (Vec<Batch>, Option<&'k Path>) {
    let (FsKey::DirListing { dir } | FsKey::DirListingTail { dir }) = key;
    let children = list_dir(dir, ctx.dir_filter());
    let (head, tail) = listing_parts(dir, &children, ctx);
    let is_tail = matches!(key, FsKey::DirListingTail { .. });
    let fully_listed = (tail.is_empty() || is_tail).then_some(dir.as_path());
    if crate::value::is_third_party_dir(dir, ctx.root()) {
        return (Vec::new(), fully_listed);
    }
    let subdirectories = if is_tail { tail } else { head }
        .into_iter()
        .filter(|name| matches!(children[*name], EntryKind::Directory))
        .map(|name| dir.join(name))
        .filter(|child| should_recurse_dir(child, ctx))
        .flat_map(|child| dir_listing_batches(child, ctx))
        .collect();
    (subdirectories, fully_listed)
}

/// Files `dir`'s listing shows whose extension matches any of `exts`,
/// in name order.
pub fn files_with_any_extension(dir: &Path, exts: &[&str], ctx: &WalkCtx) -> Vec<PathBuf> {
    list_dir(dir, ctx.dir_filter())
        .iter()
        .filter(|(name, kind)| matches!(kind, EntryKind::File) && has_extension_in(name, exts))
        .map(|(name, _)| dir.join(name))
        .collect()
}

fn has_extension_in(path: impl AsRef<Path>, exts: &[&str]) -> bool {
    path.as_ref()
        .extension()
        .and_then(|actual| actual.to_str())
        .is_some_and(|actual| exts.iter().any(|ext| actual.eq_ignore_ascii_case(ext)))
}

/// Listing of `dir`, run on through every directory that holds only one
/// subdirectory (`src/main/java/org/acme/`): such a listing names one
/// path segment, so it is bought and keyed with the first listing below
/// it that names more, at the lower of the two listings' values. The run
/// stops at a third-party directory, whose listing names its projects. A
/// long last listing is split into its head and the rest. A directory on
/// the source spine gates most of the repository's code, however few
/// entries it has, so its head is priced up; the rest keeps the plain
/// price, or a huge spine directory's names crowd out its code.
fn dir_listing_batches(dir: PathBuf, ctx: &WalkCtx) -> Vec<Batch> {
    let mut dir = dir;
    let mut children = list_dir(&dir, ctx.dir_filter());
    if children.is_empty() {
        return Vec::new();
    }
    let chain_head_value = dir_listing_value(&dir, &children, ctx);
    let mut groups = Vec::new();
    while let Some((name, EntryKind::Directory)) = children.iter().next()
        && children.len() == 1
        && !crate::value::is_third_party_dir(&dir, ctx.root())
    {
        let only_child = dir.join(name);
        let grandchildren = list_dir(&only_child, ctx.dir_filter());
        if grandchildren.is_empty() || !should_recurse_dir(&only_child, ctx) {
            break;
        }
        groups.push(listing_group(&dir, vec![name]));
        dir = only_child;
        children = grandchildren;
    }
    let value = chain_head_value.min(dir_listing_value(&dir, &children, ctx));
    if value == 0.0 {
        return Vec::new();
    }
    let head_key: BatchKey = FsKey::DirListing { dir: dir.clone() }.into();
    let (head_entries, tail_entries) = listing_parts(&dir, &children, ctx);
    let listed = head_entries.len() + tail_entries.len();
    let tail = (!tail_entries.is_empty()).then(|| Batch {
        key: FsKey::DirListingTail { dir: dir.clone() }.into(),
        predecessor: Some(head_key.clone()),
        value: value
            * (tail_entries.len() as f64 / listed as f64)
                .powf(crate::value::DEFAULT_CONCAVITY_EXPONENT),
        content: BatchContent::Fs {
            groups: vec![listing_group(&dir, tail_entries)],
        },
    });
    groups.push(listing_group(&dir, head_entries));
    let spine_factor = if ctx.is_on_source_spine(&dir) {
        SOURCE_SPINE_LISTING_BOOST
    } else {
        1.0
    };
    let head = Batch {
        key: head_key,
        predecessor: None,
        content: BatchContent::Fs { groups },
        value: value * spine_factor,
    };
    std::iter::once(head).chain(tail).collect()
}

fn listing_group(dir: &Path, entries: Vec<&String>) -> FsGroup {
    FsGroup {
        parent: dir.to_path_buf(),
        entries: FsEntries::Listed(entries.into_iter().map(PathBuf::from).collect()),
    }
}

/// Entries a listing of `children` names: all but asset sidecars.
fn listed_entries(children: &BTreeMap<String, EntryKind>) -> impl Iterator<Item = &String> {
    children.keys().filter(|name| !is_sidecar(name, children))
}

/// Multiplier on the value of a source-spine directory's listing head.
const SOURCE_SPINE_LISTING_BOOST: f64 = 2.0;

/// A listing of more entries than this is split into a head and the rest.
const LISTING_SPLIT_ENTRIES: usize = 120;
/// Entries in a split listing's head.
const LISTING_HEAD_ENTRIES: usize = 40;

/// The entries a listing of `children` names first, and the rest: a
/// long listing is split so that a big directory's first names are
/// bought even when the whole listing never is. Only an essential
/// directory's listing is split: the root is always whole, since
/// everything else hangs off it, and a long listing of tests or vendored
/// code stays one batch. The head names the essential subdirectories
/// first, then the rest, in name order.
fn listing_parts<'c>(
    dir: &Path,
    children: &'c BTreeMap<String, EntryKind>,
    ctx: &WalkCtx,
) -> (Vec<&'c String>, Vec<&'c String>) {
    let mut listed: Vec<&String> = listed_entries(children).collect();
    if dir == ctx.root()
        || listed.len() <= LISTING_SPLIT_ENTRIES
        || ctx.non_essential_factor(dir) < 1.0
    {
        return (listed, Vec::new());
    }
    listed.sort_by_key(|name| {
        !(matches!(children[*name], EntryKind::Directory)
            && ctx.non_essential_factor(&dir.join(name)) >= 1.0)
    });
    let tail = listed.split_off(LISTING_HEAD_ENTRIES);
    (listed, tail)
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
    let module_source_dir = is_module_source_dir(dir, ctx);
    let non_essential = ctx.non_essential_factor(dir);
    let source_inventory_dir = is_source_inventory_dir(dir, ctx);
    // A package module, or a partition under the repository's source
    // root, names part of the package's API wherever it sits, so it
    // prices like depth 1. Supporting source (tests, examples, docs) is
    // clamped at depth 2 so nested ones neither disappear nor act like
    // entrypoints.
    let depth = if source_inventory_dir && has_source_root_ancestor(dir, ctx)
        || module_source_dir && non_essential >= 1.0
    {
        file_depth_factor(dir, ctx, true)
    } else if non_essential < 1.0
        && (source_inventory_dir || module_source_dir || is_source_dir(dir))
    {
        crate::value::depth_factor(ctx.depth_from_root(dir).min(2)) * non_essential.max(0.5)
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
        crate::value::roster_mass(children.len()).min(crate::value::ROSTER_MASS_FACTOR_CAP)
    } else {
        1.0
    };
    let catalog_child_factor = if is_deferred_catalog_child(dir, ctx) {
        CATALOG_CHILD_LISTING_SUPPRESSION
    } else {
        1.0
    };
    LISTING_VALUE * depth * fanout * catalog_child_factor * roster_factor(children)
}

/// Fewest files for a media roster or a name catalog.
const ROSTER_MIN_FILES: usize = 11;

/// A listing is priced by its share of entries outside its rosters: many
/// images, fonts, audio or video files, whose names say what the
/// pictures are, not what the project is, and catalogs, many files
/// sharing an extension and a name prefix (`issue-*.md`,
/// `messages_*.properties`), whose names past the first say there are
/// more of the same. A listing
/// of nothing but rosters is never bought. A few media files, a logo and
/// a screenshot, cost little and keep full value.
fn roster_factor(children: &BTreeMap<String, EntryKind>) -> f64 {
    let listed: Vec<&String> = listed_entries(children).collect();
    let mut media = 0;
    let mut catalogs: HashMap<(&str, &str), usize> = HashMap::new();
    for name in &listed {
        if !matches!(children[name.as_str()], EntryKind::File) {
            continue;
        }
        if has_extension_in(name, MEDIA_EXTENSIONS) {
            media += 1;
        } else if let Some(key) = catalog_key(name) {
            *catalogs.entry(key).or_default() += 1;
        }
    }
    let roster: usize = std::iter::once(media)
        .chain(catalogs.into_values())
        .filter(|&count| count >= ROSTER_MIN_FILES)
        .sum();
    1.0 - roster as f64 / listed.len() as f64
}

/// The name prefix and extension a catalog entry shares with its
/// siblings. A module's source files share a prefix by convention
/// (`libpff_*.c`, `kubelet_*.go`), and a roster of scripts names the
/// project's commands (`scoop-*.ps1`), so neither is a catalog.
fn catalog_key(name: &str) -> Option<(&str, &str)> {
    let path = Path::new(name);
    let extension = path.extension()?.to_str()?;
    if super::language_group(path).is_some() || has_extension_in(path, SCRIPT_EXTENSIONS) {
        return None;
    }
    let (prefix_end, _) = name
        .char_indices()
        .skip(1)
        .find(|(_, char)| matches!(char, '-' | '_' | '.'))?;
    Some((&name[..prefix_end], extension))
}

const SCRIPT_EXTENSIONS: &[&str] = &["sh", "bash", "zsh", "fish", "ps1", "psm1"];

const MEDIA_EXTENSIONS: &[&str] = &[
    "png", "jpg", "jpeg", "gif", "webp", "svg", "ico", "icns", "bmp", "tif", "tiff", "psd", "ttf",
    "otf", "woff", "woff2", "eot", "mp3", "ogg", "wav", "mp4", "webm", "mov",
];

/// Min child-directory count for a parent to count as a "catalog" whose
/// per-child listings are redundant with its own listing.
const CATALOG_PARENT_MIN_CHILD_DIRS: usize = 10;
/// Deferral multiplier applied to a redundant catalog child's listing —
/// low enough to push it past the primary budget, non-zero so it stays
/// reachable at large budgets.
const CATALOG_CHILD_LISTING_SUPPRESSION: f64 = 0.05;

/// A catalog of source files that no name or entrypoint marks as a
/// source dir: a flat partition of the package (`lib/helpers/`), or an
/// inventory inside tests, examples or docs.
fn is_source_inventory_dir(dir: &Path, ctx: &WalkCtx) -> bool {
    !is_source_dir(dir)
        && !is_module_source_dir(dir, ctx)
        && ctx.fs_state.holds_source(dir, ctx.dir_filter())
        && (ctx.non_essential_factor(dir) < 1.0 || has_source_root_ancestor(dir, ctx))
}

/// Whether `dir`'s listing is deferred, not dropped, behind its
/// parent's, which already names it. The parent holds at least
/// [`CATALOG_PARENT_MIN_CHILD_DIRS`] subdirectories, and either it is a
/// source inventory, whose children are catalog leaves, or `dir` names
/// exactly the three or more entries an earlier sibling names
/// (`keyboards/*/`, `charts/*/`), so the first of each shape shows what
/// the siblings hold. Listing a one- or two-entry shape (`Cargo.toml`
/// and `src/`) costs about what naming it does, and deferring it hides
/// the code below. A declared workspace member or a package module is
/// the project's own code, whatever its layout. The walk root's parent
/// is outside the walk, so the root is never deferred.
fn is_deferred_catalog_child(dir: &Path, ctx: &WalkCtx) -> bool {
    let (Some(parent), Some(name)) = (dir.parent(), dir.file_name()) else {
        return false;
    };
    if dir == ctx.root()
        || list_dir(parent, ctx.dir_filter())
            .values()
            .filter(|kind| matches!(kind, EntryKind::Directory))
            .count()
            < CATALOG_PARENT_MIN_CHILD_DIRS
    {
        return false;
    }
    is_source_inventory_dir(parent, ctx)
        || !is_declared_workspace_member(dir, ctx)
            && !is_module_source_dir(dir, ctx)
            && ctx
                .fs_state
                .shape_repeats(parent, ctx.dir_filter())
                .contains(name.to_string_lossy().as_ref())
}

fn is_declared_workspace_member(dir: &Path, ctx: &WalkCtx) -> bool {
    ctx.is_cargo_workspace_member(&dir.join("Cargo.toml"))
        || ctx.is_js_workspace_member(&dir.join("package.json"))
}

const MODULE_ENTRYPOINT_FILES: &[&str] = &[
    "index.ts",
    "index.tsx",
    "index.js",
    "index.mjs",
    "index.cjs",
    "mod.rs",
    "__init__.py",
];
const MODULE_SIBLING_EXTS: &[&str] = &["rs", "ts", "tsx", "py"];

/// Case-insensitive, and `Sources/` counts: that is the spelling
/// SwiftPM mandates, and the same for `Source/` in Objective-C and
/// C# trees. A case-sensitive check leaves those repos with no
/// recognised source root at all. A `pkg/` beside a `go.mod` is Go's
/// library source.
fn is_source_dir(dir: &Path) -> bool {
    dir.file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|name| match name.to_ascii_lowercase().as_str() {
            "src" | "lib" | "source" | "sources" => true,
            _ => name == "pkg" && dir.with_file_name("go.mod").is_file(),
        })
}

/// A package module: a directory whose listing names its own entry file
/// (`index.ts`, `mod.rs`, `__init__.py`), or whose parent's names a file
/// of the same stem (`foo.rs` next to `foo/`). The walk root's parent
/// is outside the walk, so it is never listed.
fn is_module_source_dir(dir: &Path, ctx: &WalkCtx) -> bool {
    MODULE_ENTRYPOINT_FILES
        .iter()
        .any(|name| lists_file(&dir.join(name), ctx.dir_filter()))
        || dir != ctx.root()
            && MODULE_SIBLING_EXTS
                .iter()
                .any(|ext| lists_file(&dir.with_extension(ext), ctx.dir_filter()))
}

#[derive(Default)]
pub(in crate::walker) struct FsState {
    holds_source: RefCell<HashMap<PathBuf, bool>>,
    shape_repeats: RefCell<HashMap<PathBuf, Rc<HashSet<String>>>>,
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

    /// Names of `parent`'s subdirectories whose three or more entry names
    /// repeat an earlier subdirectory's.
    fn shape_repeats(&self, parent: &Path, filter: &DirFilter) -> Rc<HashSet<String>> {
        if let Some(repeats) = self.shape_repeats.borrow().get(parent) {
            return Rc::clone(repeats);
        }
        let mut repeats = HashSet::new();
        let mut shapes = HashSet::new();
        let listing = list_dir(parent, filter);
        let subdirs = listing
            .iter()
            .filter(|(_, kind)| matches!(kind, EntryKind::Directory));
        for (name, _) in subdirs {
            let entries = list_dir(&parent.join(name), filter);
            if entries.len() > 2 && !shapes.insert(entries.keys().cloned().collect::<Vec<_>>()) {
                repeats.insert(name.clone());
            }
        }
        let repeats = Rc::new(repeats);
        self.shape_repeats
            .borrow_mut()
            .insert(parent.to_path_buf(), Rc::clone(&repeats));
        repeats
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
    components.next().is_some() && is_source_dir(&top)
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
    super::language_group(path).is_some() || has_extension_in(path, &["md", "mdx"])
}

/// Heavy-directory names block traversal, except a `build/` that holds
/// Rust source: a checked-in module may be named `build`, while Cargo's
/// own output lives under `target/`, which the walk never enters. A
/// translated mirror or an unpacked upstream release is named, not
/// listed: its entries repeat names kept elsewhere.
fn should_recurse_dir(dir: &Path, ctx: &WalkCtx) -> bool {
    let Some(name) = dir.file_name() else {
        return false;
    };
    let name = name.to_string_lossy();
    if is_locale_mirror(dir, &name, ctx)
        || is_unpacked_release(dir, &name, ctx) && !is_declared_workspace_member(dir, ctx)
    {
        return false;
    }
    if name != "build" {
        return !crate::fs_util::should_skip_dir(&name);
    }
    !files_with_any_extension(dir, &["rs"], ctx).is_empty()
}

/// A copy of another project as its release archive unpacks, named for
/// the branch or version it was cut from (`prism-master/`,
/// `miniz-3.0.2/`) and carrying that project's license.
fn is_unpacked_release(dir: &Path, name: &str, ctx: &WalkCtx) -> bool {
    let Some((_, suffix)) = name.rsplit_once('-') else {
        return false;
    };
    let version = suffix.strip_prefix('v').unwrap_or(suffix);
    let is_release_suffix = matches!(suffix, "master" | "main")
        || version.starts_with(|first: char| first.is_ascii_digit())
            && version.contains('.')
            && version
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '.');
    is_release_suffix
        && list_dir(dir, ctx.dir_filter()).keys().any(|name| {
            let name = name.to_ascii_lowercase();
            name.starts_with("license")
                || name.starts_with("licence")
                || name.starts_with("copying")
        })
}

/// One of several translations named after the directory they mirror:
/// `pages.ar/`, `pages.pt_BR/` beside `pages/`.
fn is_locale_mirror(dir: &Path, name: &str, ctx: &WalkCtx) -> bool {
    let (Some(stem), Some(parent)) = (locale_mirror_stem(name), dir.parent()) else {
        return false;
    };
    let siblings = list_dir(parent, ctx.dir_filter());
    let mirrors = siblings
        .keys()
        .filter(|sibling| locale_mirror_stem(sibling) == Some(stem))
        .count();
    mirrors >= 2 && siblings.get(stem) == Some(&EntryKind::Directory)
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
    fn fs_long_listing_splits_into_a_head_and_the_rest() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path();
        let big = root.join("src");
        for dir in ["tests", "zlib"] {
            std::fs::create_dir_all(big.join(dir)).unwrap();
            std::fs::write(big.join(dir).join("x.py"), "").unwrap();
        }
        for index in 0..LISTING_SPLIT_ENTRIES {
            std::fs::write(big.join(format!("f{index:03}.py")), "x = 1\n").unwrap();
        }
        std::fs::create_dir_all(root.join("tests")).unwrap();
        for index in 0..=LISTING_SPLIT_ENTRIES {
            std::fs::write(root.join(format!("tests/t{index:03}.py")), "").unwrap();
        }
        let ctx = WalkCtx::new(root.to_path_buf());
        let batches = dir_listing_batches(big.clone(), &ctx);
        let [head, tail] = &batches[..] else {
            panic!("expected a head and a tail, got {}", batches.len());
        };
        let listed = |batch: &Batch| match &batch.content {
            BatchContent::Fs { groups } => match &groups[0].entries {
                FsEntries::Listed(entries) => entries.clone(),
                FsEntries::All => Vec::new(),
            },
            BatchContent::Lines { .. } => Vec::new(),
        };
        let head_entries = listed(head);
        assert_eq!(head_entries.len(), LISTING_HEAD_ENTRIES);
        assert_eq!(head_entries[0], PathBuf::from("zlib"));
        assert!(!head_entries.contains(&PathBuf::from("tests")));
        assert_eq!(tail.predecessor.as_ref(), Some(&head.key));

        let BatchKey::Fs(head_key) = &head.key else {
            panic!("a listing has an Fs key");
        };
        let (subdirs, files_listed) = expand_listed(head_key, &ctx);
        assert_eq!(subdirs.len(), 1);
        assert_eq!(files_listed, None);
        let BatchKey::Fs(tail_key) = &tail.key else {
            panic!("a listing has an Fs key");
        };
        let (subdirs, files_listed) = expand_listed(tail_key, &ctx);
        assert_eq!(subdirs.len(), 1);
        assert_eq!(files_listed, Some(big.as_path()));

        assert_eq!(dir_listing_batches(root.join("tests"), &ctx).len(), 1);
    }

    #[test]
    fn fs_root_module_check_does_not_list_the_root_parent() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().join("pkg");
        std::fs::create_dir_all(root.join("src")).unwrap();
        std::fs::write(root.join("src/lib.rs"), "").unwrap();
        std::fs::create_dir_all(temp.path().join("other/deep")).unwrap();
        let ctx = WalkCtx::new(root.clone());
        seed(&ctx);
        dir_listing_batches(root, &ctx);
        assert!(!ctx.dir_filter().has_listed(temp.path()));
    }

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
    fn fs_media_and_catalog_rosters_are_priced_by_their_other_entries() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path();
        let write = |dir: &str, names: &[String]| {
            std::fs::create_dir_all(root.join(dir)).unwrap();
            for name in names {
                std::fs::write(root.join(dir).join(name), "").unwrap();
            }
        };
        let shots: Vec<String> = (0..ROSTER_MIN_FILES)
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
        let issues: Vec<String> = (0..ROSTER_MIN_FILES)
            .map(|index| format!("issue-{index}.md"))
            .collect();
        write("issues", &issues);
        let hooks: Vec<String> = (0..ROSTER_MIN_FILES)
            .map(|index| format!("hook_{index}.sh"))
            .collect();
        write("hooks", &hooks);
        let ctx = WalkCtx::new(root.to_path_buf());
        let value = |dir: &str| {
            dir_listing_batches(root.join(dir), &ctx)
                .first()
                .map(|batch| batch.value)
        };
        assert_eq!(value("gallery"), None);
        assert!(value("screens").unwrap() < value("notes").unwrap() / 10.0);
        assert_eq!(value("branding"), value("notes"));
        assert_eq!(value("issues"), None);
        assert!(value("hooks").is_some());
    }

    #[test]
    fn fs_wide_siblings_of_one_shape_defer_all_but_the_first() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path();
        let chart_names: Vec<String> = (0..CATALOG_PARENT_MIN_CHILD_DIRS)
            .map(|index| format!("chart{index}"))
            .collect();
        for chart in &chart_names {
            std::fs::create_dir_all(root.join("charts").join(chart)).unwrap();
            for file in ["Chart.yaml", "README.md", "values.yaml"] {
                std::fs::write(root.join("charts").join(chart).join(file), "").unwrap();
            }
            std::fs::create_dir_all(root.join("crates").join(chart).join("src")).unwrap();
            std::fs::write(root.join("crates").join(chart).join("Cargo.toml"), "").unwrap();
            std::fs::write(root.join("crates").join(chart).join("src/lib.rs"), "").unwrap();
        }
        std::fs::write(root.join("charts/chart9/NOTES.txt"), "").unwrap();
        for index in 0..CATALOG_PARENT_MIN_CHILD_DIRS {
            let app = root.join(format!("apps/app{index}"));
            std::fs::create_dir_all(&app).unwrap();
            for file in ["__init__.py", "models.py", "views.py"] {
                std::fs::write(app.join(file), format!("# app {index}")).unwrap();
            }
        }
        let ctx = WalkCtx::new(root.to_path_buf());
        let value = |dir: &str| dir_listing_batches(root.join(dir), &ctx)[0].value;
        assert_eq!(
            value("charts/chart1"),
            value("charts/chart0") * CATALOG_CHILD_LISTING_SUPPRESSION
        );
        assert!(value("charts/chart9") > value("charts/chart1"));
        assert_eq!(value("crates/chart1"), value("crates/chart0"));
        assert_eq!(value("apps/app1"), value("apps/app0"));
    }

    #[test]
    fn fs_third_party_listings_name_their_projects_without_listing_them() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path();
        for dir in [
            "vendor/jquery",
            "lib/third_party/zlib",
            "source/vendor/ansi-styles",
        ] {
            std::fs::create_dir_all(root.join(dir)).unwrap();
            std::fs::write(root.join(dir).join("index.js"), "").unwrap();
        }
        let ctx = WalkCtx::new(root.to_path_buf());
        let expands = |dir: &str| {
            let key = FsKey::DirListing {
                dir: root.join(dir),
            };
            !expand_listed(&key, &ctx).0.is_empty()
        };
        assert!(!expands("vendor"));
        assert!(!expands("lib/third_party"));
        assert!(expands("source/vendor"));
        let vendor_listing = &dir_listing_batches(root.join("vendor"), &ctx)[0];
        assert_eq!(
            vendor_listing.key,
            FsKey::DirListing {
                dir: root.join("vendor")
            }
            .into()
        );
    }

    #[test]
    fn fs_mirrors_releases_and_build_output_are_named_not_listed() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path();
        for dir in [
            ".git",
            "pages/common",
            "pages.ar/common",
            "pages.pt_BR/common",
            "glossary/node",
            "glossary/node.js",
            "docs/prism-master",
            "third/miniz-3.0.2",
            "drivers/i2c-master",
            "packages/core-main",
            "packages/ui-main",
            "src/build",
            "build",
            "tools/build",
            "target",
        ] {
            std::fs::create_dir_all(root.join(dir)).unwrap();
        }
        for (file, text) in [
            ("docs/prism-master/LICENSE", ""),
            ("third/miniz-3.0.2/LICENSE", ""),
            ("packages/ui-main/LICENSE", ""),
            (
                "package.json",
                r#"{"name": "monorepo", "workspaces": ["packages/*"]}"#,
            ),
            ("packages/ui-main/package.json", r#"{"name": "ui-main"}"#),
            ("src/build/mod.rs", ""),
            ("build/probe.rs", ""),
            (".gitignore", "generated.rs\n"),
            ("tools/build/generated.rs", ""),
            ("tools/build/CMakeCache.txt", ""),
        ] {
            std::fs::write(root.join(file), text).unwrap();
        }
        let ctx = WalkCtx::new(root.to_path_buf());
        let recurses = |dir: &str| should_recurse_dir(&root.join(dir), &ctx);
        for dir in [
            "pages",
            "glossary/node.js",
            "drivers/i2c-master",
            "packages/core-main",
            "packages/ui-main",
            "src/build",
            "build",
        ] {
            assert!(recurses(dir), "{dir}");
        }
        for dir in [
            "pages.ar",
            "pages.pt_BR",
            "docs/prism-master",
            "third/miniz-3.0.2",
            "tools/build",
            "target",
        ] {
            assert!(!recurses(dir), "{dir}");
        }
    }
}
