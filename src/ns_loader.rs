//! North Star loading + resolution. Parses NS TOML, verifies the
//! revision-pin invariant, and expands `FsEntries::All` / `Names`
//! sentinels into concrete `Listed(...)` maps via [`crate::fs_util`].
//!
//! Separated from [`crate::north_star`] (types-only) and from
//! [`crate::batch`] (public schema vocabulary) so the types stay a clean
//! public surface.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, anyhow, bail};

use crate::batch::{BatchContent, EntryKind, FsEntries, FsGroup, Span};
use crate::fs_util::list_dir;
use crate::north_star::NorthStar;

/// Load the NS TOML at `ns_path` and verify its `revision_pin` matches
/// the pin file inside the fixture directory (`<fixture_root>/.precis-pin`).
/// Fails loudly on mismatch — this is the only thing tying a frozen NS
/// to its authored fixture revision after the old review-staleness
/// pipeline was retired.
pub fn load_ns_checked(ns_path: &Path, fixture_root: &Path) -> Result<NorthStar> {
    let ns = load_ns(ns_path)?;
    let pin_path = fixture_root.join(".precis-pin");
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

/// Resolve an NS `BatchContent` against the fixture root. Absolutizes
/// span paths; expands `FsEntries::All` / `Names` into a concrete
/// `Listed(...)` map via [`list_dir`] (same utility the walker uses, so
/// NS and walker see identical filesystem content).
pub fn resolve_content(content: &BatchContent, fixture_root: &Path) -> Result<BatchContent> {
    match content {
        BatchContent::Lines { spans } => {
            let spans = spans
                .iter()
                .map(|s| Span {
                    path: fixture_root.join(&s.path),
                    start: s.start,
                    end: s.end,
                    render: s.render.clone(),
                })
                .collect();
            Ok(BatchContent::Lines { spans })
        }
        BatchContent::Fs { groups } => {
            let resolved = groups
                .iter()
                .map(|g| resolve_fs_group(g, fixture_root))
                .collect::<Result<Vec<_>>>()?;
            Ok(BatchContent::Fs { groups: resolved })
        }
    }
}

fn resolve_fs_group(group: &FsGroup, fixture_root: &Path) -> Result<FsGroup> {
    let parent_abs = fixture_root.join(&group.parent);
    let children = match &group.entries {
        FsEntries::All => {
            let listed = list_dir(&parent_abs);
            if listed.is_empty() && !parent_abs.exists() {
                bail!(
                    "NS fs group points to non-existent parent {}",
                    parent_abs.display()
                );
            }
            listed
        }
        FsEntries::Names(names) => {
            let probed = list_dir(&parent_abs);
            let mut children: BTreeMap<String, EntryKind> = BTreeMap::new();
            for name in names {
                let kind = probed.get(name).copied().ok_or_else(|| {
                    anyhow!(
                        "NS fs group at {} lists entry {:?} which is not present under parent",
                        parent_abs.display(),
                        name
                    )
                })?;
                children.insert(name.clone(), kind);
            }
            children
        }
        FsEntries::Listed(m) => m.clone(),
    };
    Ok(FsGroup {
        parent: resolve_parent(&group.parent, fixture_root),
        entries: FsEntries::Listed(children),
    })
}

fn resolve_parent(parent: &Path, fixture_root: &Path) -> PathBuf {
    if parent.is_absolute() {
        parent.to_path_buf()
    } else {
        fixture_root.join(parent)
    }
}
