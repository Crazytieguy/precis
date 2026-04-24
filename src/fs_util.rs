//! Filesystem utilities used by both the NS schema resolver and the
//! filesystem walker. Raw directory listing — no policy, no filtering
//! beyond one precis-internal sentinel (`.precis-pin`, our fixture-
//! revision marker file). Walker-specific heuristics (heavy-directory
//! skip for recursive traversal, etc.) live in `walker::fs`.

use std::collections::BTreeMap;
use std::path::Path;

use crate::batch::EntryKind;

/// Name of the precis-internal per-fixture revision-pin file. Never
/// appears in listings — it's tooling metadata, not part of the fixture
/// content being summarized.
pub const PRECIS_PIN_FILE: &str = ".precis-pin";

/// Read a directory's immediate children into a name-keyed map. Names are
/// produced via `to_string_lossy` — non-UTF-8 paths (rare in practice)
/// lose information, accepted so filesystem listings round-trip through
/// TOML for schedule snapshots.
///
/// Filtering: none, except the precis-internal `.precis-pin` file. In
/// particular, hidden dotfiles (`.github`, `.gitignore`, `.dockerignore`,
/// etc.) are surfaced — a fixture's configuration is part of its content
/// and should be visible to the walker and NS. Gitignored files don't
/// appear in fixtures (clone_fixtures strips `.git` and fixtures contain
/// only tracked files), so gitignore filtering is deferred until precis
/// is used on real-world repos.
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
