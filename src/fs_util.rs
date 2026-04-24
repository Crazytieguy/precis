//! Filesystem utilities used by the walker and NS loader. Raw directory
//! listing, plus the internal [`EntryKind`] used by the renderer to
//! decide whether to trail a `/` on each name.

use std::collections::BTreeMap;
use std::path::Path;

use serde::{Deserialize, Serialize};

/// Name of the precis-internal per-fixture revision-pin file. Never
/// appears in listings — it's tooling metadata, not fixture content.
pub const PRECIS_PIN_FILE: &str = ".precis-pin";

/// Directory entry kind. Internal to the walker + renderer; not part of
/// the public batch content vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EntryKind {
    File,
    #[serde(rename = "dir")]
    Directory,
}

/// Read a directory's immediate children into a name-keyed map. Names
/// are lossy UTF-8 (rare non-UTF-8 paths lose information, accepted so
/// names round-trip through TOML).
///
/// No filtering beyond the precis-internal `.precis-pin` file. Hidden
/// dotfiles (`.github`, `.gitignore`, etc.) are fixture content and
/// appear in listings. Gitignored files don't appear in the fixture
/// set today (clone_fixtures strips `.git` and fixtures contain only
/// tracked files).
pub fn list_dir(path: &Path) -> BTreeMap<String, EntryKind> {
    let Ok(read_dir) = std::fs::read_dir(path) else {
        return BTreeMap::new();
    };
    read_dir
        .flatten()
        .filter_map(|e| {
            let name_os = e.file_name();
            let name = name_os.to_string_lossy().into_owned();
            if name == PRECIS_PIN_FILE {
                return None;
            }
            let kind = if e.file_type().ok()?.is_dir() {
                EntryKind::Directory
            } else {
                EntryKind::File
            };
            Some((name, kind))
        })
        .collect()
}
