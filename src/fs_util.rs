//! Filesystem utilities used by the walker and NS loader. Raw directory
//! listing, the gitignore-aware visibility filter every listing goes
//! through, plus the internal [`EntryKind`] used by the renderer to
//! decide whether to trail a `/` on each name.

use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use ignore::Match;
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use serde::{Deserialize, Serialize};

/// Name of the precis-internal per-fixture revision-pin file. Never
/// appears in listings — it's tooling metadata, not fixture content.
pub const PRECIS_PIN_FILE: &str = ".precis-pin";

/// Git's metadata directory (a `gitdir:` pointer *file* in a linked
/// worktree or submodule, hence not always a directory).
const GIT_DIR: &str = ".git";

const GITIGNORE_FILE: &str = ".gitignore";

/// Directory entry kind. Internal to the walker + renderer; not part of
/// the public batch content vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EntryKind {
    File,
    #[serde(rename = "dir")]
    Directory,
}

/// Entries precis never surfaces whatever the ignore rules say. `.git`
/// is special-cased by git itself rather than listed in any ignore
/// file, so gitignore matching alone never hides it.
fn is_internal_entry(name: &str) -> bool {
    name == GIT_DIR || name == PRECIS_PIN_FILE
}

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
    /// `None` when gitignore rules don't apply to this root — see
    /// [`DirFilter::new`].
    repo: Option<RepoIgnores>,
}

struct RepoIgnores {
    root: PathBuf,
    /// `.gitignore` matcher per directory; `None` for a directory with
    /// no `.gitignore`. Lazily populated.
    per_dir: RefCell<HashMap<PathBuf, Option<Rc<Gitignore>>>>,
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
    /// fall back to the walker's heavy-directory blocklist
    /// (`walker::fs::should_skip_dir`).
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
            return Self::none();
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
                root: root.to_path_buf(),
                per_dir: RefCell::new(HashMap::new()),
                fallbacks,
            }),
        }
    }

    /// [`DirFilter::new`] without the user's global excludes file, so a
    /// test's expected listing can't change with whoever runs it.
    #[cfg(test)]
    fn without_global_excludes(root: &Path) -> Self {
        Self::build(root, false)
    }

    /// Filter with no ignore rules of its own — `list_dir` still drops
    /// [`is_internal_entry`] names. For call sites that only re-probe
    /// entry kinds for names an already-filtered listing produced.
    pub fn none() -> Self {
        Self { repo: None }
    }

    /// [`DirFilter::excludes`] for a directory entry, without building
    /// the child path when there is nothing to ask. Directory scans
    /// call this once per entry, and joining a `PathBuf` per entry to
    /// answer "no" is the whole cost of the filter on a tree it has no
    /// opinion about.
    pub fn excludes_child(&self, dir: &Path, name: &OsStr, is_dir: bool) -> bool {
        self.repo.is_some() && self.excludes(&dir.join(name), is_dir)
    }

    /// [`DirFilter::excludes`] extended to `path`'s ancestors — true
    /// when `path` sits anywhere under an excluded directory.
    ///
    /// The listing walk doesn't need this: it prunes an ignored
    /// directory before descending, so a surviving entry's ancestors are
    /// known visible. Scans that *start* somewhere other than the walk
    /// root do (`walker::rust`'s Cargo source dirs jump straight to
    /// `<package>/examples`), and without it a directory-only pattern
    /// like `examples/` hides the directory while every file under it
    /// still gets read and parsed. Costs one match per ancestor, so use
    /// it at traversal entry points, not per directory entry.
    pub fn excludes_tree(&self, path: &Path, is_dir: bool) -> bool {
        let Some(repo) = &self.repo else {
            return false;
        };
        if self.excludes(path, is_dir) {
            return true;
        }
        let mut dir = path.parent();
        while let Some(current) = dir {
            if current == repo.root || !current.starts_with(&repo.root) {
                return false;
            }
            if self.excludes(current, true) {
                return true;
            }
            dir = current.parent();
        }
        false
    }

    /// True when `path` must not appear in precis output.
    pub fn excludes(&self, path: &Path, is_dir: bool) -> bool {
        let Some(repo) = &self.repo else {
            return false;
        };
        // Containment is what bounds the ancestor walk below at the repo
        // root. Without it a stray out-of-tree path would climb past the
        // root reading `.gitignore` files that don't govern this walk.
        if !path.starts_with(&repo.root) {
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
            if current == repo.root {
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

/// Read a directory's immediate children into a name-keyed map. Names
/// are lossy UTF-8 (rare non-UTF-8 paths lose information, accepted so
/// names round-trip through TOML).
///
/// Drops `.git`, the precis-internal `.precis-pin`, and whatever
/// `filter` hides. Non-ignored dotfiles are repo content and stay:
/// `.github`, `.gitignore`, `.dockerignore` are all things the North
/// Stars rank, so nothing is filtered for being hidden as such.
pub fn list_dir(path: &Path, filter: &DirFilter) -> BTreeMap<String, EntryKind> {
    let Ok(read_dir) = std::fs::read_dir(path) else {
        return BTreeMap::new();
    };
    read_dir
        .flatten()
        .filter_map(|e| {
            let name_os = e.file_name();
            let name = name_os.to_string_lossy().into_owned();
            if is_internal_entry(&name) {
                return None;
            }
            let kind = if e.file_type().ok()?.is_dir() {
                EntryKind::Directory
            } else {
                EntryKind::File
            };
            if filter.excludes_child(path, &name_os, matches!(kind, EntryKind::Directory)) {
                return None;
            }
            Some((name, kind))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every test here builds the filter without the user's global
    /// excludes file, so expectations can't change with whoever runs
    /// `cargo t`.
    fn names_in(dir: &Path, filter: &DirFilter) -> Vec<String> {
        list_dir(dir, filter).into_keys().collect()
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

    /// A directory-only pattern hides the directory but matches none of
    /// the files inside it, so scans that start below the walk root have
    /// to ask about ancestors.
    #[test]
    fn fs_util_filter_excludes_tree_rejects_paths_under_an_ignored_dir() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path();
        std::fs::create_dir(root.join(GIT_DIR)).unwrap();
        std::fs::write(root.join(".gitignore"), "examples/\n").unwrap();
        std::fs::create_dir_all(root.join("examples/nested")).unwrap();
        std::fs::write(root.join("examples/nested/demo.rs"), "").unwrap();

        let filter = DirFilter::without_global_excludes(root);
        let demo = root.join("examples/nested/demo.rs");
        assert!(!filter.excludes(&demo, false));
        assert!(filter.excludes_tree(&demo, false));
        assert!(filter.excludes_tree(&root.join("examples"), true));
        // The walk root itself is always visible, ignored or not.
        assert!(!filter.excludes_tree(root, true));
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
