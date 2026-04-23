//! North Star TOML schema + loader.
//!
//! The schema reuses library types where the shapes match and wraps them
//! where they don't:
//!
//! - Directly reused: [`Span`], [`Render`], [`EntryKind`] — the same values
//!   that flow through the walker/render pipeline. Drift is impossible by
//!   construction.
//! - Wrapped: [`NsFsGroup`] uses a symbolic `entries = "all"` sentinel that
//!   resolves to the library's concrete `FsGroup { parent, children }` at
//!   load time via [`walker::fs::list_dir`] so NS-declared listings match
//!   walker-rendered listings (walker filters hidden dotfiles + skips some
//!   directory classes; raw `read_dir` wouldn't).
//!
//! [`load_ns_checked`] is the one entry point both `validate-ns` and the
//! divergence test use — it enforces the revision-pin invariant so an NS
//! can't silently compare against a re-pinned fixture.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, anyhow, bail};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::batch::{BatchContent, FsGroup, Span};
use crate::walker::fs::list_dir;

/// A frozen North Star document for one fixture: ranked batches with
/// declarative spans + render specs. Parsed from TOML.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NorthStar {
    pub fixture: String,
    pub revision_pin: String,
    #[serde(default)]
    pub summary: String,
    pub batches: Vec<NsBatch>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NsBatch {
    pub id: String,
    pub descriptor: String,
    pub justification: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub predecessor: Option<String>,
    pub content: NsContent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum NsContent {
    /// One or more filesystem listings. Resolved to library `FsGroup`
    /// values at load time via [`walker::fs::list_dir`].
    Fs { groups: Vec<NsFsGroup> },
    /// Source line-range spans with render specs. Library [`Span`] values
    /// directly — same type the walker emits.
    Lines { spans: Vec<Span> },
}

/// Schema-side filesystem listing: `entries` is either the sentinel `"all"`
/// (expand by reading the directory) or an explicit list of child names.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NsFsGroup {
    pub parent: PathBuf,
    pub entries: NsEntries,
}

#[derive(Debug, Clone)]
pub enum NsEntries {
    All,
    Listed(Vec<String>),
}

impl Serialize for NsEntries {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            NsEntries::All => serializer.serialize_str("all"),
            NsEntries::Listed(names) => names.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for NsEntries {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Repr {
            Str(String),
            List(Vec<String>),
        }
        match Repr::deserialize(deserializer)? {
            Repr::Str(s) if s == "all" => Ok(NsEntries::All),
            Repr::Str(s) => Err(serde::de::Error::custom(format!(
                "invalid `entries` sentinel {s:?}; expected \"all\" or a list"
            ))),
            Repr::List(v) => Ok(NsEntries::Listed(v)),
        }
    }
}

/// Resolve a schema `NsContent` against the fixture root into a library
/// [`BatchContent`]. Reads the filesystem for `entries = "all"` groups
/// using the walker's own [`list_dir`] (so NS-declared listings match
/// walker-emitted listings).
pub fn resolve_content(content: &NsContent, fixture_root: &Path) -> Result<BatchContent> {
    match content {
        NsContent::Lines { spans } => {
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
        NsContent::Fs { groups } => {
            let resolved = groups
                .iter()
                .map(|g| resolve_fs_group(g, fixture_root))
                .collect::<Result<Vec<_>>>()?;
            Ok(BatchContent::Fs { groups: resolved })
        }
    }
}

fn resolve_fs_group(group: &NsFsGroup, fixture_root: &Path) -> Result<FsGroup> {
    let parent_abs = fixture_root.join(&group.parent);
    let children = match &group.entries {
        NsEntries::All => {
            let listed = list_dir(&parent_abs);
            if listed.is_empty() && !parent_abs.exists() {
                bail!(
                    "NS fs group points to non-existent parent {}",
                    parent_abs.display()
                );
            }
            listed
        }
        NsEntries::Listed(names) => {
            let probed = list_dir(&parent_abs);
            let mut children: BTreeMap<String, crate::batch::EntryKind> = BTreeMap::new();
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
    };
    Ok(FsGroup {
        parent: parent_abs,
        children,
    })
}

/// Load the NS TOML at `ns_path` and verify its `revision_pin` matches the
/// pin file inside the fixture directory (`<fixture_root>/.precis-pin`).
/// Fails loudly on mismatch — this is the only thing tying a frozen NS to
/// its authored fixture revision after the old review-staleness pipeline
/// was retired.
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

/// Load the NS TOML at `ns_path` without the revision-pin check. Use
/// `load_ns_checked` for anything that scores the NS against walker output.
pub fn load_ns(ns_path: &Path) -> Result<NorthStar> {
    let text = std::fs::read_to_string(ns_path)
        .with_context(|| format!("reading NS {}", ns_path.display()))?;
    let ns: NorthStar =
        toml::from_str(&text).with_context(|| format!("parsing NS {}", ns_path.display()))?;
    Ok(ns)
}
