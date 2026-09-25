//! Shared workspace-member resolution for manifest-aware walkers
//! (Cargo `Cargo.toml`, npm/pnpm `package.json`). Per-language modules
//! supply the manifest filename and the raw entry list; this module
//! owns the glob expansion, canonicalization, and per-run caching.

use std::cell::{OnceCell, RefCell};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

/// Damp the identity block of a workspace-member manifest — a sub-crate
/// / sub-package states its own name and version, but what the project
/// *is* is mostly inherited from the workspace root. Language-independent:
/// Cargo members and npm/pnpm members answer the reader's question the
/// same way, so both walkers price them the same.
pub(super) const WORKSPACE_MEMBER_IDENTITY_FACTOR: f64 = 0.4;

/// Per-run cache of a workspace's resolved member-manifest set plus a
/// raw-path → `is_member` lookup. Shared by the Cargo and npm/pnpm
/// workspaces.
#[derive(Default)]
pub(super) struct WorkspaceMembership {
    members: OnceCell<HashSet<PathBuf>>,
    lookup: RefCell<HashMap<PathBuf, bool>>,
}

impl WorkspaceMembership {
    pub(super) fn members(&self, init: impl FnOnce() -> HashSet<PathBuf>) -> &HashSet<PathBuf> {
        self.members.get_or_init(init)
    }

    /// `true` iff `file` resolves to a member manifest. Membership is
    /// checked via the canonical path; the raw path → result mapping
    /// is memoized.
    pub(super) fn is_member(&self, file: &Path, init: impl FnOnce() -> HashSet<PathBuf>) -> bool {
        let members = self.members(init);
        if members.is_empty() {
            return false;
        }
        if let Some(&hit) = self.lookup.borrow().get(file) {
            return hit;
        }
        let hit = file
            .canonicalize()
            .map(|c| members.contains(&c))
            .unwrap_or(false);
        self.lookup.borrow_mut().insert(file.to_path_buf(), hit);
        hit
    }
}

/// Per-run Cargo workspace state: the member manifests and the facts
/// primary-member election reads from them.
#[derive(Default)]
pub(super) struct CargoWorkspace {
    membership: WorkspaceMembership,
    member_facts: OnceCell<super::toml::MemberFacts>,
}

impl CargoWorkspace {
    /// `true` iff `file` is a workspace-member `Cargo.toml`. Memoized.
    pub(super) fn is_member(&self, file: &Path, root: &Path) -> bool {
        self.membership
            .is_member(file, || super::toml::collect_workspace_members(root))
    }

    /// `true` when the workspace's primary member is ambiguous and `file` is
    /// not one of the colliding candidates — the only case where a member can
    /// be called definitely-secondary without knowing which one is primary.
    pub(super) fn is_definite_secondary_member(&self, file: &Path, root: &Path) -> bool {
        let members = self
            .membership
            .members(|| super::toml::collect_workspace_members(root));
        let facts = self
            .member_facts
            .get_or_init(|| super::toml::read_member_facts(root, members));
        let key = file.canonicalize().unwrap_or_else(|_| file.to_path_buf());
        facts
            .ambiguous_primary()
            .is_some_and(|members| !members.contains(&key))
    }
}

/// Expand one workspace-member entry against `<root>`. Supports literal
/// paths (`./crates/foo`, `examples/bar`) and trailing-`/*` globs
/// (`crates/*`); mid-name globs are an honest no-op. Manifest existence
/// is checked downstream by [`canonical_member`].
pub(super) fn expand_member_entry(root: &Path, entry: &str) -> Vec<PathBuf> {
    let trimmed = entry.trim_start_matches("./");
    if let Some(prefix) = trimmed.strip_suffix("/*") {
        let Ok(read) = std::fs::read_dir(root.join(prefix)) else {
            return Vec::new();
        };
        return read
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.is_dir())
            .collect();
    }
    if trimmed.contains('*') || trimmed.contains('?') {
        return Vec::new();
    }
    vec![root.join(trimmed)]
}

/// Resolve `<dir>/<manifest_filename>` to its canonical path, requiring
/// the file to exist and to live under `canonical_root` (no `..` escape,
/// no absolute override).
pub(super) fn canonical_member(
    canonical_root: &Path,
    dir: &Path,
    manifest_filename: &str,
) -> Option<PathBuf> {
    let canonical_manifest = dir.join(manifest_filename).canonicalize().ok()?;
    canonical_manifest
        .starts_with(canonical_root)
        .then_some(canonical_manifest)
}
