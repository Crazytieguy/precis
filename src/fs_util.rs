//! Filesystem utilities used by the walker and NS loader. Raw directory
//! listing, the gitignore-aware visibility filter every listing goes
//! through, plus the internal [`EntryKind`] used by the renderer to
//! decide whether to trail a `/` on each name.
//!
//! Containment is this module's job, not each walker's: every name
//! precis surfaces comes out of [`list_dir`], and nothing it yields
//! resolves outside the walk root. See [`resolved_kind`].

use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};
use std::rc::Rc;

use ignore::Match;
use ignore::gitignore::{Gitignore, GitignoreBuilder};

/// Name of the precis-internal per-fixture revision-pin file. Never
/// appears in listings — it's tooling metadata, not fixture content.
pub const PRECIS_PIN_FILE: &str = ".precis-pin";

/// Git's metadata directory (a `gitdir:` pointer *file* in a linked
/// worktree or submodule, hence not always a directory).
const GIT_DIR: &str = ".git";

const GITIGNORE_FILE: &str = ".gitignore";

/// Directory entry kind. Internal to the walker + renderer; not part of
/// the public batch content vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum EntryKind {
    File,
    Directory,
}

/// Entries precis never surfaces whatever the ignore rules say. `.git`
/// is special-cased by git itself rather than listed in any ignore
/// file, so gitignore matching alone never hides it.
fn is_internal_entry(name: &str) -> bool {
    name == GIT_DIR || name == PRECIS_PIN_FILE
}

/// Directories the walker never recurses into — heavy/generated trees.
/// They still appear in listings; only traversal is affected.
///
/// Kept alongside the gitignore filter rather than subsumed by it:
/// inside a repository these names are almost always gitignored and the
/// list never fires, but precis also runs on trees that aren't
/// repositories (extracted archives, vendored snapshots, the fixture
/// corpus), where the filter is inert by design and this is the only
/// thing standing between the walk and a `node_modules` tree. Lives here
/// so one module owns every "the walk does not look in there" rule.
pub(crate) fn should_skip_dir(name: &str) -> bool {
    matches!(
        name,
        "target" | "node_modules" | ".git" | "dist" | "build" | ".next" | "__pycache__"
    )
}

/// Most directory entries a whole-tree probe reads: the spine survey
/// once per run, a source-inventory probe once per directory it is asked
/// about. Past it the probe answers as if nothing more were there, so a
/// huge tree unrelated to any project, like a home directory, can't stall
/// the summary. The corpus's probes read at most 3.5k entries and the
/// 186-repo robustness sweep's inventory probes at most 12k in a run.
pub(crate) const PROBE_ENTRY_CAP: usize = 20_000;

/// Gitignore-aware visibility filter for one walk root, built once per
/// run and shared by every listing and file-enumeration call.
///
/// Rules resolve the way git resolves them: the `.gitignore` in an
/// entry's own directory wins over its parents', which win over
/// `$GIT_DIR/info/exclude`, which wins over the user's global excludes
/// file. Per-directory matchers are built lazily and cached, so a walk
/// pays one `GitignoreBuilder` per directory that actually carries a
/// `.gitignore`.
pub struct DirFilter {
    /// Walk root as the caller named it. Bounds the `.gitignore`
    /// ancestor walk, and marks the one path [`list_dir`] will list
    /// through even if it is a link.
    root: PathBuf,
    /// The same root resolved, so a link's canonicalized target can be
    /// tested against it even when the root itself was reached through
    /// a link — `tests/fixtures` is one, so every fixture walk is.
    canonical_root: PathBuf,
    /// `None` when gitignore rules don't apply to this root — see
    /// [`DirFilter::new`].
    repo: Option<RepoIgnores>,
    /// Memo for [`list_dir`]: every walker and every scheduler cost
    /// probe lists the same directories again.
    listings: RefCell<HashMap<PathBuf, Rc<BTreeMap<String, EntryKind>>>>,
    /// Memo for [`lists_nothing`] on directories not listed in full.
    emptiness: RefCell<HashMap<PathBuf, bool>>,
    /// Set for a single-file walk: the one entry under `root` the filter
    /// admits. See [`DirFilter::single_file`].
    only_file: Option<PathBuf>,
}

impl std::fmt::Debug for DirFilter {
    /// The matchers themselves aren't printable and wouldn't be legible
    /// if they were; what a reader wants from a dump is whether ignore
    /// rules are active at all, and for which root.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DirFilter")
            .field("root", &self.root)
            .field("ignore_rules", &self.repo.is_some())
            .field("only_file", &self.only_file)
            .finish_non_exhaustive()
    }
}

struct RepoIgnores {
    /// `.gitignore` matcher per directory; `None` for a directory with
    /// no `.gitignore`. Lazily populated.
    per_dir: RefCell<HashMap<PathBuf, Option<Rc<Gitignore>>>>,
    /// Memo for [`DirFilter::hides_everything_in`]. Each directory is
    /// probed at most once per run, which is what keeps the answer from
    /// costing a repeated subtree walk.
    vacuous: RefCell<HashMap<PathBuf, bool>>,
    /// `$GIT_DIR/info/exclude` then the global excludes file, consulted
    /// only after the whole `.gitignore` chain came back undecided.
    fallbacks: Vec<Gitignore>,
}

impl DirFilter {
    /// Filter for a walk rooted at `root`.
    ///
    /// Gitignore rules apply only when `root` is itself the root of a
    /// git repository — the `ignore` crate's `require_git` default, and
    /// what ripgrep and `fd` do. Two reasons not to search ancestors for
    /// an enclosing repo instead:
    ///
    /// * Ignore rules are a repository's statement about its own
    ///   contents. A tree that isn't a repository may still carry a
    ///   `.gitignore` (an extracted archive, a vendored snapshot)
    ///   describing build products of a build that never ran here, and
    ///   there is no repository to tell us otherwise.
    /// * Resolving an enclosing repo would make output depend on ignore
    ///   rules living *outside* the summarized tree — including the
    ///   user's machine-global excludes file. `tests/fixtures/<f>` sits
    ///   inside precis's own repo, so the frozen corpus would start
    ///   reading precis's `.gitignore` and every contributor's
    ///   `core.excludesFile`, and baselines would stop being
    ///   reproducible.
    ///
    /// Directories below a repo root that aren't repo roots themselves
    /// fall back to the heavy-directory blocklist ([`should_skip_dir`]).
    ///
    /// Matching is pattern-only: the index is never consulted, so a
    /// force-added tracked file matching an ignore pattern is hidden
    /// here even though git doesn't consider it ignored. Same limitation
    /// as every ignore-crate consumer, and as precis v0.1.
    pub fn new(root: &Path) -> Self {
        Self::build(root, true)
    }

    fn build(root: &Path, use_global_excludes: bool) -> Self {
        let Some(common_dir) = git_common_dir(root) else {
            return Self::unfiltered(root);
        };
        let mut fallbacks = Vec::new();
        if let Some(matcher) = build_gitignore(root, &common_dir.join("info/exclude")) {
            fallbacks.push(matcher);
        }
        if use_global_excludes {
            let (global, _) = Gitignore::global();
            if !global.is_empty() {
                fallbacks.push(global);
            }
        }
        Self {
            repo: Some(RepoIgnores {
                per_dir: RefCell::new(HashMap::new()),
                vacuous: RefCell::new(HashMap::new()),
                fallbacks,
            }),
            ..Self::unfiltered(root)
        }
    }

    /// [`DirFilter::new`] without the user's global excludes file, so a
    /// test's expected listing can't change with whoever runs it.
    #[cfg(test)]
    fn without_global_excludes(root: &Path) -> Self {
        Self::build(root, false)
    }

    /// Filter with no ignore rules of its own, still scoped to `root`:
    /// [`list_dir`] keeps dropping [`is_internal_entry`] names and
    /// anything resolving outside `root`.
    fn unfiltered(root: &Path) -> Self {
        Self {
            root: root.to_path_buf(),
            canonical_root: root.canonicalize().unwrap_or_else(|_| root.to_path_buf()),
            repo: None,
            listings: RefCell::new(HashMap::new()),
            emptiness: RefCell::new(HashMap::new()),
            only_file: None,
        }
    }

    /// Filter for summarizing one file: a walk of its directory that
    /// admits `file` and nothing else, so every walker applies to it
    /// unchanged. Ignore rules don't apply — the caller named the file.
    pub fn single_file(file: &Path) -> Self {
        let root = file.parent().unwrap_or(file);
        Self {
            only_file: Some(file.to_path_buf()),
            ..Self::unfiltered(root)
        }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    /// The file a [`Self::single_file`] walk admits.
    pub(crate) fn named_file(&self) -> Option<&Path> {
        self.only_file.as_deref()
    }

    /// True when `dir` is a symbolic link rather than a real directory,
    /// and so must not be listed *through*.
    ///
    /// The root is exempt: a caller may legitimately point precis at a
    /// link, and that link is then the walk's whole scope rather than an
    /// escape from it. The CLI canonicalizes its path first, so only library
    /// callers rely on this.
    pub(crate) fn is_linked_subdirectory(&self, dir: &Path) -> bool {
        dir != self.root
            && dir != self.canonical_root
            && std::fs::symlink_metadata(dir).is_ok_and(|meta| meta.file_type().is_symlink())
    }

    /// True when `dir` holds something on disk yet the filter admits
    /// nothing anywhere beneath it.
    ///
    /// This is the self-ignoring-directory idiom — a scratch, cache or
    /// vendor directory whose own `.gitignore` is a single `*` — plus any
    /// directory left holding only such directories. Git calls these
    /// ignored (`git status --ignored` collapses the whole thing to one
    /// `dir/` row, and `git check-ignore dir/` names the nested pattern),
    /// even though no pattern matches the directory itself: the pattern
    /// that hides it lives *inside* it, one level below where a parent's
    /// listing decides.
    ///
    /// A **genuinely empty** directory is deliberately not this. It holds
    /// nothing because it holds nothing, not because precis is
    /// withholding it, and an empty directory is real repository
    /// structure — dropping its row would substitute one lie for another.
    ///
    /// Answering costs one `read_dir`, and recurses only into a directory
    /// that has no surviving file of its own, so the common case is a
    /// single probe that stops at the first visible entry. Memoized per
    /// run.
    pub(crate) fn hides_everything_in(&self, dir: &Path) -> bool {
        let Some(repo) = &self.repo else {
            return false;
        };
        if let Some(&known) = repo.vacuous.borrow().get(dir) {
            return known;
        }
        let vacuous = self.probe_hides_everything_in(dir);
        repo.vacuous.borrow_mut().insert(dir.to_path_buf(), vacuous);
        vacuous
    }

    fn probe_hides_everything_in(&self, dir: &Path) -> bool {
        // A linked directory is never listed through, so its row is all
        // there is of it and the filter never withholds that row — the
        // same reasoning `should_skip_dir` gets below.
        if self.is_linked_subdirectory(dir) {
            return false;
        }
        let Ok(read_dir) = std::fs::read_dir(dir) else {
            return false;
        };
        let mut occupied = false;
        let mut surviving_subdirs = Vec::new();
        for entry in read_dir.flatten() {
            occupied = true;
            let name = entry.file_name();
            if is_internal_entry(&name.to_string_lossy()) {
                continue;
            }
            let Ok(file_type) = entry.file_type() else {
                continue;
            };
            let path = entry.path();
            // An entry the listing drops is not a surviving one.
            let Some(kind) = resolved_kind(&path, &file_type, self) else {
                continue;
            };
            let is_dir = matches!(kind, EntryKind::Directory);
            if self.excludes(&path, is_dir) {
                continue;
            }
            if !is_dir {
                // A surviving file settles it without any recursion.
                return false;
            }
            if should_skip_dir(&name.to_string_lossy()) {
                // Heavy generated trees are listed but never entered, so
                // the row itself is the content — and descending to
                // confirm that is exactly the walk this filter exists to
                // avoid.
                return false;
            }
            surviving_subdirs.push(path);
        }
        occupied
            && surviving_subdirs
                .iter()
                .all(|child| self.hides_everything_in(child))
    }

    /// True when the walk reaches `target`, a resolved path: it lies
    /// inside the root and nothing on the way down to it is internal or
    /// ignored.
    fn reaches(&self, target: &Path, is_dir: bool) -> bool {
        let Ok(relative) = target.strip_prefix(&self.canonical_root) else {
            return false;
        };
        let mut path = self.root.clone();
        let mut components = relative.components().peekable();
        while let Some(component) = components.next() {
            path.push(component);
            let component_is_dir = is_dir || components.peek().is_some();
            if is_internal_entry(&component.as_os_str().to_string_lossy())
                || self.excludes(&path, component_is_dir)
            {
                return false;
            }
        }
        true
    }

    /// True when `path` must not appear in precis output.
    pub(crate) fn excludes(&self, path: &Path, is_dir: bool) -> bool {
        if let Some(only_file) = &self.only_file {
            return path != only_file;
        }
        let Some(repo) = &self.repo else {
            return false;
        };
        // Containment is what bounds the ancestor walk below at the repo
        // root. Without it a stray out-of-tree path would climb past the
        // root reading `.gitignore` files that don't govern this walk.
        if !path.starts_with(&self.root) {
            return false;
        }
        // Walk the `.gitignore` chain from the entry's own directory up
        // to the repo root; the first decisive match wins, matching
        // git's "deepest file, then last pattern" precedence.
        let mut dir = path.parent();
        while let Some(current) = dir {
            if let Some(matcher) = repo.matcher_for(current)
                && let Some(ignored) = decide(&matcher, path, is_dir)
            {
                return ignored;
            }
            if current == self.root {
                break;
            }
            dir = current.parent();
        }
        repo.fallbacks
            .iter()
            .find_map(|matcher| decide(matcher, path, is_dir))
            .unwrap_or(false)
    }
}

impl RepoIgnores {
    fn matcher_for(&self, dir: &Path) -> Option<Rc<Gitignore>> {
        if let Some(cached) = self.per_dir.borrow().get(dir) {
            return cached.clone();
        }
        let built = build_gitignore(dir, &dir.join(GITIGNORE_FILE)).map(Rc::new);
        self.per_dir
            .borrow_mut()
            .insert(dir.to_path_buf(), built.clone());
        built
    }
}

/// `Some(ignored)` when `matcher` decides `path`, `None` when it has no
/// opinion and the next matcher in the chain gets a say.
fn decide(matcher: &Gitignore, path: &Path, is_dir: bool) -> Option<bool> {
    match matcher.matched(path, is_dir) {
        Match::Ignore(_) => Some(true),
        Match::Whitelist(_) => Some(false),
        Match::None => None,
    }
}

/// Matcher for the ignore file at `file`, with patterns resolved
/// relative to `base`. `None` when the file is absent or has no globs.
fn build_gitignore(base: &Path, file: &Path) -> Option<Gitignore> {
    if !file.is_file() {
        return None;
    }
    let mut builder = GitignoreBuilder::new(base);
    // Errors here are per-glob: git skips lines it can't parse and keeps
    // the rest of the file, so mirror that rather than dropping the
    // whole matcher.
    builder.add(file);
    let matcher = builder.build().ok()?;
    (!matcher.is_empty()).then_some(matcher)
}

/// `$GIT_COMMON_DIR` for a repository root, or `None` when `root` isn't
/// one. `.git` is a directory in a normal clone and a `gitdir: <path>`
/// pointer file in a linked worktree or submodule.
///
/// The *common* directory rather than `$GIT_DIR`, because `info/` is one
/// of the paths git shares across a repository's worktrees: a linked
/// worktree's own git dir has no `info/exclude`, the shared one does.
fn git_common_dir(root: &Path) -> Option<PathBuf> {
    let dot_git = root.join(GIT_DIR);
    let metadata = std::fs::metadata(&dot_git).ok()?;
    if metadata.is_dir() {
        return Some(dot_git);
    }
    let pointer = std::fs::read_to_string(&dot_git).ok()?;
    let git_dir = resolve_relative_to(root, pointer.strip_prefix("gitdir:")?.trim());
    // A linked worktree's git dir names its shared parent in `commondir`;
    // a submodule's doesn't and is already the common dir.
    let Ok(common) = std::fs::read_to_string(git_dir.join("commondir")) else {
        return Some(git_dir);
    };
    Some(resolve_relative_to(&git_dir, common.trim()))
}

fn resolve_relative_to(base: &Path, target: &str) -> PathBuf {
    let target = Path::new(target);
    if target.is_absolute() {
        target.to_path_buf()
    } else {
        base.join(target)
    }
}

/// Kind an entry surfaces as, or `None` when it must not surface at all.
///
/// An ordinary entry is its own type. A symbolic link is the type of
/// what it resolves to, and only when the walk [reaches](DirFilter::reaches)
/// that: inside the root — the containment rule the whole walk rests
/// on — and not ignored. precis summarizes *a path*; a link out of that
/// path has no business contributing
/// either content or structure, and a checkout that ships one
/// (`config.ini -> ~/.config/app/credentials.ini`) would otherwise have
/// precis read arbitrary files off the machine running it and paste them
/// into whatever context the summary feeds. A dangling link resolves to
/// nothing, so it can be neither classified nor cleared, and goes the
/// same way.
///
/// A link that stays inside the root is an alias for content the walk
/// can already reach, and hiding it would misreport real repository
/// structure (`CLAUDE.md -> AGENTS.md`, `README -> README.md`), so it
/// keeps its row. A link to what the walk hides (`README.md -> .env`
/// with `.env` ignored) is an alias for nothing the walk may show, and
/// goes the way of its target.
///
/// A FIFO, socket or device is not repository content, and opening one
/// to read it can block forever, so only directories and regular files
/// surface.
///
/// Costs one `canonicalize` per link and nothing per ordinary entry.
fn resolved_kind(
    child: &Path,
    file_type: &std::fs::FileType,
    filter: &DirFilter,
) -> Option<EntryKind> {
    if !file_type.is_symlink() {
        return (file_type.is_dir() || file_type.is_file()).then(|| entry_kind(file_type.is_dir()));
    }
    let target = child.canonicalize().ok()?;
    let target_type = std::fs::metadata(&target).ok()?.file_type();
    ((target_type.is_dir() || target_type.is_file())
        && filter.reaches(&target, target_type.is_dir()))
    .then(|| entry_kind(target_type.is_dir()))
}

fn entry_kind(is_dir: bool) -> EntryKind {
    if is_dir {
        EntryKind::Directory
    } else {
        EntryKind::File
    }
}

/// Read a directory's immediate children into a name-keyed map.
///
/// Drops names that aren't UTF-8 (every consumer reopens an entry by
/// its listed name, and a lossy spelling can name a different entry),
/// `.git`, the precis-internal `.precis-pin`, anything resolving
/// outside the walk root or onto what it hides (see [`resolved_kind`]),
/// and whatever `filter` hides. Non-ignored dotfiles are repo content
/// and stay: `.github`, `.gitignore`, `.dockerignore` are all things the
/// North Stars rank, so nothing is filtered for being hidden as such.
///
/// Listing a *linked* directory yields nothing. Every name under it is
/// already reachable at the target's real path, and refusing to list
/// through a link is what keeps the walk finite: `link -> .` or
/// `link -> ..` otherwise manufactures paths without end, and no
/// per-walker cycle check would be needed if traversal only ever
/// descends through real directories — which this makes true by
/// construction.
pub fn list_dir(path: &Path, filter: &DirFilter) -> Rc<BTreeMap<String, EntryKind>> {
    if let Some(listing) = filter.listings.borrow().get(path) {
        return Rc::clone(listing);
    }
    let listing = Rc::new(read_listing(path, filter));
    filter
        .listings
        .borrow_mut()
        .insert(path.to_path_buf(), Rc::clone(&listing));
    listing
}

/// Whether the listing of `path`'s directory admits it as a file.
pub(crate) fn lists_file(path: &Path, filter: &DirFilter) -> bool {
    let (Some(dir), Some(name)) = (
        path.parent(),
        path.file_name().and_then(|name| name.to_str()),
    ) else {
        return false;
    };
    list_dir(dir, filter).get(name) == Some(&EntryKind::File)
}

/// Whether [`list_dir`] lists nothing for `path`, reading only as far
/// as the first entry it would list. Rendering asks this of every child
/// directory in a listing to mark the empty ones; a full listing of each
/// child would read two levels below every listing.
pub(crate) fn lists_nothing(path: &Path, filter: &DirFilter) -> bool {
    if let Some(listing) = filter.listings.borrow().get(path) {
        return listing.is_empty();
    }
    if let Some(&known) = filter.emptiness.borrow().get(path) {
        return known;
    }
    let empty = filter.is_linked_subdirectory(path)
        || std::fs::read_dir(path).map_or(true, |entries| {
            !entries
                .flatten()
                .any(|entry| listed_entry(path, &entry, filter).is_some())
        });
    filter
        .emptiness
        .borrow_mut()
        .insert(path.to_path_buf(), empty);
    empty
}

fn read_listing(path: &Path, filter: &DirFilter) -> BTreeMap<String, EntryKind> {
    if filter.is_linked_subdirectory(path) {
        return BTreeMap::new();
    }
    let Ok(read_dir) = std::fs::read_dir(path) else {
        return BTreeMap::new();
    };
    read_dir
        .flatten()
        .filter_map(|entry| listed_entry(path, &entry, filter))
        .collect()
}

/// The row `entry` of directory `path` lists as, if any.
fn listed_entry(
    path: &Path,
    entry: &std::fs::DirEntry,
    filter: &DirFilter,
) -> Option<(String, EntryKind)> {
    let name = entry.file_name().into_string().ok()?;
    if is_internal_entry(&name) {
        return None;
    }
    let child = path.join(&name);
    let kind = resolved_kind(&child, &entry.file_type().ok()?, filter)?;
    let is_dir = matches!(kind, EntryKind::Directory);
    if filter.excludes(&child, is_dir) {
        return None;
    }
    // A directory that hides its whole contents is ignored content
    // itself, not a directory that happens to be empty — see
    // `hides_everything_in`.
    if is_dir && filter.hides_everything_in(&child) {
        return None;
    }
    Some((name, kind))
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::process::Command;

    use super::*;

    /// Every test here builds the filter without the user's global
    /// excludes file, so expectations can't change with whoever runs
    /// `cargo t`.
    fn names_in(dir: &Path, filter: &DirFilter) -> Vec<String> {
        list_dir(dir, filter).keys().cloned().collect()
    }

    /// Root-relative paths the listing walk admits, in the walk's own
    /// order of discovery.
    fn walk_visible(root: &Path, filter: &DirFilter) -> BTreeSet<String> {
        let mut out = BTreeSet::new();
        let mut stack = vec![root.to_path_buf()];
        while let Some(dir) = stack.pop() {
            for (name, kind) in list_dir(&dir, filter).iter() {
                let child = dir.join(name);
                out.insert(
                    child
                        .strip_prefix(root)
                        .unwrap()
                        .to_string_lossy()
                        .into_owned(),
                );
                if matches!(kind, EntryKind::Directory) {
                    stack.push(child);
                }
            }
        }
        out
    }

    /// `git` with global and system config neutralized, so the oracle
    /// doesn't depend on whoever runs the suite either.
    fn git(repo: &Path, args: &[&str]) -> String {
        let out = Command::new("git")
            .args(args)
            .current_dir(repo)
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .env("GIT_CONFIG_SYSTEM", "/dev/null")
            .output()
            .expect("`git` must be on PATH to run the gitignore parity test");
        assert!(
            out.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8(out.stdout).expect("git output is UTF-8")
    }

    /// The whole filter, checked against git itself rather than against
    /// hand-reasoned expectations.
    ///
    /// The oracle is git's *walk* (`ls-files`), not `git check-ignore`.
    /// check-ignore answers "does a pattern match this path", which is a
    /// one-level question: with `outer/inner/.gitignore` holding `*` it
    /// calls `outer/inner/` ignored but `outer/` visible, while the walk
    /// — and `git status --ignored` — collapses the whole chain. The
    /// walking answer is the one a reader of the summary cares about.
    #[test]
    fn fs_util_filter_matches_the_paths_git_can_see() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path();
        git(root, &["init", "-q", "."]);
        std::fs::write(
            root.join(".gitignore"),
            // Unanchored, anchored, directory-only, and a negation whose
            // position in the file is what makes it win.
            "*.log\n/build\ntmp/\n!important.log\n",
        )
        .unwrap();
        std::fs::create_dir_all(root.join(GIT_DIR).join("info")).unwrap();
        std::fs::write(root.join(GIT_DIR).join("info/exclude"), "local-only\n").unwrap();

        std::fs::write(root.join("README.md"), "# demo\n").unwrap();
        std::fs::write(root.join("noisy.log"), "").unwrap();
        std::fs::write(root.join("important.log"), "").unwrap();
        std::fs::write(root.join("local-only"), "").unwrap();
        std::fs::create_dir_all(root.join("build/lib")).unwrap();
        std::fs::write(root.join("build/lib/out.o"), "").unwrap();
        // Anchored `/build` doesn't reach here, so this one survives.
        std::fs::create_dir_all(root.join("nested/build")).unwrap();
        std::fs::write(root.join("nested/build/keep.rs"), "").unwrap();
        // Nested .gitignore that both adds a rule and re-includes.
        std::fs::create_dir(root.join("src")).unwrap();
        std::fs::write(root.join("src/.gitignore"), "generated.rs\n!*.log\n").unwrap();
        std::fs::write(root.join("src/lib.rs"), "").unwrap();
        std::fs::write(root.join("src/generated.rs"), "").unwrap();
        std::fs::write(root.join("src/trace.log"), "").unwrap();
        // Self-ignoring scratch dir, and a parent left holding only one.
        std::fs::create_dir(root.join("scratch")).unwrap();
        std::fs::write(root.join("scratch/.gitignore"), "*\n").unwrap();
        std::fs::write(root.join("scratch/notes"), "").unwrap();
        std::fs::create_dir_all(root.join("outer/inner")).unwrap();
        std::fs::write(root.join("outer/inner/.gitignore"), "*\n").unwrap();
        std::fs::write(root.join("outer/inner/blob"), "").unwrap();
        // Unanchored `tmp/` reaches any depth.
        std::fs::create_dir_all(root.join("docs/tmp")).unwrap();
        std::fs::write(root.join("docs/guide.md"), "").unwrap();
        std::fs::write(root.join("docs/tmp/scratch.md"), "").unwrap();
        // Git cannot represent an empty directory at all.
        std::fs::create_dir(root.join("placeholder")).unwrap();

        // Every file git can see, plus the directories on the way to one.
        let mut expected: BTreeSet<String> = BTreeSet::new();
        for file in git(root, &["ls-files", "--others", "--exclude-standard"]).lines() {
            let mut prefix = String::new();
            for part in file.split('/') {
                if !prefix.is_empty() {
                    prefix.push('/');
                }
                prefix.push_str(part);
                expected.insert(prefix.clone());
            }
        }
        // The one thing git has no way to report. An empty directory is
        // real repository structure, so precis keeps listing it.
        expected.insert("placeholder".to_string());

        let filter = DirFilter::without_global_excludes(root);
        assert_eq!(walk_visible(root, &filter), expected);
    }

    /// A tree with no `.git` is not a repository, so its `.gitignore`
    /// carries no authority over what's in it — the fixture corpus is
    /// exactly this shape and must keep rendering unchanged.
    #[test]
    fn fs_util_filter_is_inert_without_a_repo() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path();
        std::fs::write(root.join(".gitignore"), "dist/\n*.log\n").unwrap();
        std::fs::create_dir(root.join("dist")).unwrap();
        std::fs::write(root.join("run.log"), "x").unwrap();

        let filter = DirFilter::without_global_excludes(root);
        assert_eq!(names_in(root, &filter), [".gitignore", "dist", "run.log"]);
    }

    #[test]
    fn fs_util_filter_honours_the_gitignore_chain_in_a_repo() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path();
        std::fs::create_dir_all(root.join(GIT_DIR).join("info")).unwrap();
        std::fs::write(root.join(GIT_DIR).join("info/exclude"), "local-only\n").unwrap();
        std::fs::write(root.join(".gitignore"), "*.log\nbuild/\n!keep.log\n").unwrap();
        std::fs::create_dir_all(root.join(".github/workflows")).unwrap();
        std::fs::create_dir(root.join("build")).unwrap();
        std::fs::create_dir(root.join("src")).unwrap();
        std::fs::write(root.join("run.log"), "x").unwrap();
        std::fs::write(root.join("keep.log"), "x").unwrap();
        std::fs::write(root.join("local-only"), "x").unwrap();
        // Nested .gitignore: applies to its own directory and below, and
        // can re-include what an ancestor ignored.
        std::fs::write(root.join("src/.gitignore"), "generated.rs\n!*.log\n").unwrap();
        std::fs::write(root.join("src/lib.rs"), "").unwrap();
        std::fs::write(root.join("src/generated.rs"), "").unwrap();
        std::fs::write(root.join("src/trace.log"), "").unwrap();

        let filter = DirFilter::without_global_excludes(root);
        // `.git` gone, ignored entries gone, dotfile repo content kept.
        assert_eq!(
            names_in(root, &filter),
            [".github", ".gitignore", "keep.log", "src"]
        );
        assert_eq!(
            names_in(&root.join("src"), &filter),
            [".gitignore", "lib.rs", "trace.log"]
        );
    }

    /// The self-ignoring-directory idiom: a scratch dir whose own
    /// `.gitignore` is `*`. No pattern matches the directory from
    /// outside — the one that hides it lives inside it — so a parent's
    /// listing has to look in. Git agrees these are ignored; it collapses
    /// them to a single `dir/` row under `git status --ignored`.
    #[test]
    fn fs_util_filter_drops_a_directory_that_hides_all_its_own_contents() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path();
        std::fs::create_dir(root.join(GIT_DIR)).unwrap();
        std::fs::write(root.join(".gitignore"), "\n").unwrap();
        // Hides everything it holds, including its own `.gitignore`.
        std::fs::create_dir(root.join("scratch")).unwrap();
        std::fs::write(root.join("scratch/.gitignore"), "*\n").unwrap();
        std::fs::write(root.join("scratch/notes.txt"), "x").unwrap();
        // Holds only such a directory — git collapses the whole chain.
        std::fs::create_dir_all(root.join("outer/inner")).unwrap();
        std::fs::write(root.join("outer/inner/.gitignore"), "*\n").unwrap();
        std::fs::write(root.join("outer/inner/blob.bin"), "x").unwrap();
        // Genuinely empty: real repository structure, keeps its row.
        std::fs::create_dir(root.join("placeholder")).unwrap();
        // Mixed: one hidden child, one real file.
        std::fs::create_dir_all(root.join("mixed/cache")).unwrap();
        std::fs::write(root.join("mixed/cache/.gitignore"), "*\n").unwrap();
        std::fs::write(root.join("mixed/real.rs"), "").unwrap();

        let filter = DirFilter::without_global_excludes(root);
        assert_eq!(
            names_in(root, &filter),
            [".gitignore", "mixed", "placeholder"]
        );
        assert_eq!(names_in(&root.join("mixed"), &filter), ["real.rs"]);
        assert!(filter.hides_everything_in(&root.join("scratch")));
        assert!(filter.hides_everything_in(&root.join("outer")));
        assert!(!filter.hides_everything_in(&root.join("placeholder")));
        assert!(!filter.hides_everything_in(&root.join("mixed")));
    }

    /// The listing layer is where containment lives, so state it here
    /// too rather than only through the renderer: a link surfaces
    /// exactly when it resolves inside the walk root, with the kind of
    /// what it resolves to, and a linked directory is never listed
    /// through.
    #[cfg(unix)]
    #[test]
    fn fs_util_list_dir_admits_only_links_that_resolve_inside_the_root() {
        use std::os::unix::fs::symlink;

        let temp = tempfile::tempdir().unwrap();
        let outside = temp.path().join("outside");
        let root = temp.path().join("repo");
        std::fs::create_dir_all(outside.join("nested")).unwrap();
        std::fs::create_dir_all(root.join("src")).unwrap();
        std::fs::write(outside.join("secret.txt"), "leak").unwrap();
        std::fs::write(root.join("README.md"), "# demo\n").unwrap();

        symlink(outside.join("secret.txt"), root.join("escaping.txt")).unwrap();
        symlink(&outside, root.join("escaping-dir")).unwrap();
        symlink("nowhere.txt", root.join("dangling.txt")).unwrap();
        symlink("README.md", root.join("CLAUDE.md")).unwrap();
        symlink("src", root.join("linked-src")).unwrap();
        symlink(".", root.join("selfloop")).unwrap();

        let filter = DirFilter::without_global_excludes(&root);
        let listed = list_dir(&root, &filter);
        assert_eq!(
            listed.keys().cloned().collect::<Vec<_>>(),
            ["CLAUDE.md", "README.md", "linked-src", "selfloop", "src"]
        );
        // A link takes the kind of what it resolves to, not `File`.
        assert_eq!(listed["CLAUDE.md"], EntryKind::File);
        assert_eq!(listed["linked-src"], EntryKind::Directory);
        // Listing through a link yields nothing — the walk descends only
        // real directories, which is what makes a cycle unreachable
        // rather than merely bounded.
        assert!(list_dir(&root.join("selfloop"), &filter).is_empty());
        assert!(list_dir(&root.join("linked-src"), &filter).is_empty());
        // ...except at the root, which is the walk's scope rather than
        // an escape from it.
        let via_link = root.join("selfloop");
        let filter = DirFilter::without_global_excludes(&via_link);
        assert!(!list_dir(&via_link, &filter).is_empty());
    }

    /// A linked worktree carries a `.git` pointer file, and keeps
    /// `info/exclude` in the shared common dir named by `commondir`.
    #[test]
    fn fs_util_filter_reads_exclude_from_a_worktree_common_dir() {
        let temp = tempfile::tempdir().unwrap();
        let common = temp.path().join("main/.git");
        let worktree_git_dir = common.join("worktrees/w");
        std::fs::create_dir_all(common.join("info")).unwrap();
        std::fs::create_dir_all(&worktree_git_dir).unwrap();
        std::fs::write(common.join("info/exclude"), "notes.txt\n").unwrap();
        std::fs::write(worktree_git_dir.join("commondir"), "../..\n").unwrap();

        let root = temp.path().join("wt");
        std::fs::create_dir(&root).unwrap();
        std::fs::write(
            root.join(GIT_DIR),
            format!("gitdir: {}\n", worktree_git_dir.display()),
        )
        .unwrap();
        std::fs::write(root.join(".gitignore"), "target/\n").unwrap();
        std::fs::create_dir(root.join("target")).unwrap();
        std::fs::write(root.join("notes.txt"), "").unwrap();
        std::fs::write(root.join("main.rs"), "").unwrap();

        let filter = DirFilter::without_global_excludes(&root);
        assert_eq!(names_in(&root, &filter), [".gitignore", "main.rs"]);
    }
}
