//! North Star loading and resolution against a fixture. Kept apart from
//! [`crate::north_star`] so the schema file the NS author reads holds
//! only types.

use std::path::{Component, Path, PathBuf};

use anyhow::{Context, Result, anyhow, bail};

use crate::content::{BatchContent, FsEntries, FsGroup, Span};
use crate::fs_util::{DirFilter, list_dir};
use crate::north_star::NorthStar;

/// Load the NS TOML at `ns_path` and verify its `revision_pin` matches
/// `<fixture_root>/.precis-pin`, the only thing tying a frozen NS to the
/// fixture revision it was authored against.
pub fn load_ns_checked(ns_path: &Path, fixture_root: &Path) -> Result<NorthStar> {
    let ns = load_ns(ns_path)?;
    let pin_path = fixture_root.join(crate::fs_util::PRECIS_PIN_FILE);
    let pin = std::fs::read_to_string(&pin_path)
        .with_context(|| format!("reading fixture pin {}", pin_path.display()))?;
    let pin_trimmed = pin.trim();
    if ns.revision_pin != pin_trimmed {
        bail!(
            "NS {} pins revision {} but fixture at {} pins {} — re-author the NS against the current fixture revision, or re-clone the fixture to the NS's pin",
            ns_path.display(),
            ns.revision_pin,
            fixture_root.display(),
            pin_trimmed
        );
    }
    Ok(ns)
}

/// Parse the NS TOML at `ns_path` without the revision-pin check.
/// Prefer [`load_ns_checked`] for anything that scores the NS against
/// walker output.
pub fn load_ns(ns_path: &Path) -> Result<NorthStar> {
    let text = std::fs::read_to_string(ns_path)
        .with_context(|| format!("reading NS {}", ns_path.display()))?;
    let ns: NorthStar =
        toml::from_str(&text).with_context(|| format!("parsing NS {}", ns_path.display()))?;
    Ok(ns)
}

/// Absolute, `..`-traversing, or through a link that resolves outside
/// `fixture_root` — a path that leaves the fixture root when joined onto
/// it. One predicate for span paths and fs-group parents so the two
/// containment checks can't drift.
pub(crate) fn path_escapes_root(fixture_root: &Path, path: &Path) -> bool {
    if path.is_absolute() || path.components().any(|c| c == Component::ParentDir) {
        return true;
    }
    match (
        fixture_root.canonicalize(),
        fixture_root.join(path).canonicalize(),
    ) {
        (Ok(root), Ok(resolved)) => !resolved.starts_with(root),
        _ => false,
    }
}

/// Resolve an NS `BatchContent` against the fixture root `filter` is
/// scoped to. Input must be raw NS content (root-relative paths) —
/// resolved output doesn't round-trip. Absolutizes span paths; expands
/// `FsEntries::All` into a concrete `Listed(paths)` via [`list_dir`] and
/// `filter`, the filter the walker builds for that root, so an NS listing
/// and the walker's listing of the same directory can't disagree about
/// what exists; verifies each parent is a directory and each `Listed`
/// entry's name one of its children.
pub fn resolve_content(content: &BatchContent, filter: &DirFilter) -> Result<BatchContent> {
    let fixture_root = filter.root();
    match content {
        BatchContent::Lines { spans, .. } => {
            let spans = spans
                .iter()
                .map(|s| {
                    // Same root-boundary invariant as fs parents: every
                    // resolve_content caller (divergence scoring included,
                    // not just validate-ns) rejects escaping spans.
                    if path_escapes_root(fixture_root, &s.path) {
                        bail!(
                            "NS span path must be fixture-root-relative and stay inside it: {}",
                            s.path.display()
                        );
                    }
                    Ok(Span {
                        path: fixture_root.join(&s.path),
                        start: s.start,
                        end: s.end,
                        render: s.render.clone(),
                    })
                })
                .collect::<Result<Vec<_>>>()?;
            Ok(BatchContent::Lines {
                spans,
                units: Vec::new(),
            })
        }
        BatchContent::Fs { groups } => {
            let resolved = groups
                .iter()
                .map(|g| resolve_fs_group(g, fixture_root, filter))
                .collect::<Result<Vec<_>>>()?;
            Ok(BatchContent::Fs { groups: resolved })
        }
    }
}

fn resolve_fs_group(group: &FsGroup, fixture_root: &Path, filter: &DirFilter) -> Result<FsGroup> {
    // Fs parents must stay inside the fixture root, so a frozen NS with
    // an escaping parent fails divergence scoring loudly.
    if path_escapes_root(fixture_root, &group.parent) {
        bail!(
            "NS fs group parent must be fixture-root-relative and stay inside it: {}",
            group.parent.display()
        );
    }
    let parent_abs = fixture_root.join(&group.parent);
    let metadata = std::fs::symlink_metadata(&parent_abs).with_context(|| {
        format!(
            "NS fs group points to non-existent parent {}",
            parent_abs.display()
        )
    })?;
    if !metadata.is_dir() {
        bail!(
            "NS fs group parent {} is a {}, not a directory",
            parent_abs.display(),
            if metadata.is_symlink() {
                "link"
            } else {
                "file"
            }
        );
    }
    let entries = match &group.entries {
        FsEntries::All => FsEntries::Listed(
            list_dir(&parent_abs, filter)
                .keys()
                .map(PathBuf::from)
                .collect(),
        ),
        FsEntries::Listed(paths) => {
            let probed = list_dir(&parent_abs, filter);
            for p in paths {
                let name = p
                    .file_name()
                    .and_then(|n| n.to_str())
                    .ok_or_else(|| anyhow!("NS fs entry path is not a valid name: {p:?}"))?;
                if !probed.contains_key(name) {
                    bail!(
                        "NS fs group at {} lists entry {:?} which is not present under parent",
                        parent_abs.display(),
                        name
                    );
                }
            }
            FsEntries::Listed(paths.clone())
        }
    };
    Ok(FsGroup {
        parent: parent_abs,
        entries,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::Render;

    fn resolve(content: &BatchContent, root: &Path) -> Result<BatchContent> {
        resolve_content(content, &DirFilter::new(root))
    }

    fn lines_content(path: &str) -> BatchContent {
        BatchContent::Lines {
            spans: vec![Span {
                path: PathBuf::from(path),
                start: 1,
                end: 1,
                render: Render::Full,
            }],
            units: Vec::new(),
        }
    }

    /// Escaping span paths must fail in `resolve_content` itself, so
    /// every caller — divergence scoring included, not only the
    /// validator — enforces the root boundary.
    #[test]
    fn ns_loader_rejects_escaping_span_paths() {
        let root = Path::new("/repo");
        assert!(resolve(&lines_content("../outside.rs"), root).is_err());
        assert!(resolve(&lines_content("/etc/passwd"), root).is_err());
        assert!(resolve(&lines_content("src/a/../b.rs"), root).is_err());
    }

    fn fs_content(parent: &str, entries: FsEntries) -> BatchContent {
        BatchContent::Fs {
            groups: vec![FsGroup {
                parent: PathBuf::from(parent),
                entries,
            }],
        }
    }

    fn resolve_error(content: &BatchContent, root: &Path) -> String {
        format!("{:#}", resolve(content, root).unwrap_err())
    }

    /// A link out of the fixture escapes it as surely as `..` does; a
    /// file or linked parent is rejected for what it is rather than
    /// resolving to an empty listing.
    #[test]
    fn ns_loader_rejects_links_out_and_malformed_fs_groups() {
        let outside = tempfile::tempdir().unwrap();
        std::fs::write(outside.path().join("secret.txt"), "x\n").unwrap();
        let fixture = tempfile::tempdir().unwrap();
        let root = fixture.path();
        std::fs::create_dir(root.join("src")).unwrap();
        std::fs::write(root.join("src/lib.rs"), "fn f() {}\n").unwrap();
        std::fs::write(root.join("README.md"), "# r\n").unwrap();
        std::os::unix::fs::symlink(outside.path(), root.join("out")).unwrap();
        std::os::unix::fs::symlink(root.join("src"), root.join("linked")).unwrap();

        assert!(resolve(&lines_content("src/lib.rs"), root).is_ok());
        assert!(resolve(&lines_content("out/secret.txt"), root).is_err());
        let listed = |names: &[&str]| FsEntries::Listed(names.iter().map(PathBuf::from).collect());
        assert!(resolve(&fs_content(".", listed(&["src"])), root).is_ok());
        assert!(resolve_error(&fs_content("README.md", FsEntries::All), root).contains("a file"));
        assert!(resolve_error(&fs_content("linked", FsEntries::All), root).contains("a link"));
        assert!(resolve_error(&fs_content("out", FsEntries::All), root).contains("stay inside"));
    }
}
