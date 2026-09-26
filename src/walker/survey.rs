//! The repository survey: which language the tree is written in and
//! which single source file, if any, carries a dominant share of it.
//! One capped walk over the essential source answers both.

use std::collections::{HashMap, VecDeque};
use std::path::{Path, PathBuf};

use crate::fs_util::{DirFilter, PROBE_ENTRY_CAP};

/// Minimum share of the tree's essential source bytes for the largest
/// source file to count as the repository's spine.
const DOMINANT_SOURCE_MASS_SHARE: f64 = 0.20;

/// Upper bound on a spine file's size, checked before the file is read.
/// Past this, a single file is a generated table or an amalgamated bundle
/// rather than something a reader is meant to read more of.
const DOMINANT_SOURCE_MAX_FILE_BYTES: u64 = 400_000;

/// The tree's essential source files, as one walk: byte mass per
/// language family, plus the per-file candidate list.
pub(super) struct EssentialSource {
    per_language: HashMap<&'static str, u64>,
    candidates: Vec<(PathBuf, u64, &'static str)>,
}

/// Walk `root` for its essential source. Non-essential subtrees (tests,
/// examples, vendored, tooling) are not entered, since nothing under one
/// counts, so a large test corpus can neither dilute the mass nor spend
/// the walk's budget. `None` once it has read [`PROBE_ENTRY_CAP`]
/// entries.
///
/// Enumeration mirrors the walkers' own rules rather than inventing a
/// second traversal policy: gitignore exclusion via `filter`, the
/// heavy-directory blocklist via [`crate::fs_util::should_skip_dir`]
/// (the only thing bounding a walk of a non-repository tree, where the
/// filter is inert by design), and non-following file types so a
/// symlink is neither descended into nor weighed as source.
pub(super) fn enumerate_essential_source(
    root: &Path,
    filter: &DirFilter,
) -> Option<EssentialSource> {
    let mut per_language: HashMap<&'static str, u64> = HashMap::new();
    let mut candidates: Vec<(PathBuf, u64, &'static str)> = Vec::new();
    let mut entries_read = 0;
    // Breadth first: the answer doesn't depend on the order, and a tree
    // past the cap can reach it after opening far fewer directories.
    let mut queue = VecDeque::from([root.to_path_buf()]);
    while let Some(dir) = queue.pop_front() {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            entries_read += 1;
            if entries_read > PROBE_ENTRY_CAP {
                return None;
            }
            let Ok(file_type) = entry.file_type() else {
                continue;
            };
            let path = entry.path();
            if filter.excludes(&path, file_type.is_dir()) {
                continue;
            }
            if file_type.is_dir() {
                let name = entry.file_name();
                let name = name.to_string_lossy();
                if !crate::fs_util::should_skip_dir(&name)
                    && crate::value::non_essential_factor(&path, root) >= 1.0
                {
                    queue.push_back(path);
                }
            } else if file_type.is_file() {
                let Some(language) = language_group(&path) else {
                    continue;
                };
                if crate::value::non_essential_factor(&path, root) < 1.0 {
                    continue;
                }
                let Ok(len) = entry.metadata().map(|m| m.len()) else {
                    continue;
                };
                *per_language.entry(language).or_default() += len;
                candidates.push((path, len, language));
            }
        }
    }
    Some(EssentialSource {
        per_language,
        candidates,
    })
}

/// Select a source file in the `primary` language that carries at least
/// [`DOMINANT_SOURCE_MASS_SHARE`] of the essential source — the largest
/// one when several do. A candidate over
/// [`DOMINANT_SOURCE_MAX_FILE_BYTES`] is never read, and one whose text
/// reads as machine-generated (a banner, or minified line lengths) never
/// wins.
pub(super) fn find_dominant_source_file(
    source: &EssentialSource,
    primary: &str,
) -> Option<PathBuf> {
    // The spine has to be written in the language the repository is
    // written in — a vendored JS bundle inside a Go tree is source mass
    // but it is not what the repo is about.
    let total: u64 = source.per_language.values().sum();
    if total == 0 {
        return None;
    }
    source
        .candidates
        .iter()
        .filter(|(_, len, language)| {
            *language == primary
                && *len <= DOMINANT_SOURCE_MAX_FILE_BYTES
                && *len as f64 / total as f64 >= DOMINANT_SOURCE_MASS_SHARE
        })
        .filter(|(path, _, _)| {
            std::fs::read_to_string(path)
                .is_ok_and(|text| !super::plaintext::is_machine_generated_text(&text))
        })
        .max_by(|a, b| a.1.cmp(&b.1).then_with(|| b.0.cmp(&a.0)))
        .map(|(path, _, _)| path.clone())
}

/// The language with the most bytes, ranked over a sorted vector rather
/// than the hash map's iteration order; `None` on a tie for the lead.
pub(super) fn find_primary_language(source: &EssentialSource) -> Option<&'static str> {
    let mut by_mass: Vec<(&'static str, u64)> = source
        .per_language
        .iter()
        .map(|(&language, &bytes)| (language, bytes))
        .collect();
    by_mass.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(b.0)));
    let (primary, primary_bytes) = *by_mass.first()?;
    let tied = by_mass
        .get(1)
        .is_some_and(|&(_, bytes)| bytes == primary_bytes);
    (!tied).then_some(primary)
}

/// Extension → language family. Every extension the code engine parses
/// or the plaintext fallback reads as language source is its own family,
/// except those whose files sit side by side with another's in one
/// codebase (a `.h` beside its `.c`, a `.ts` beside its `.js`, an
/// interface beside its implementation), which join it. `None` for
/// anything that isn't hand-authored code.
pub(in crate::walker) fn language_group(path: &Path) -> Option<&'static str> {
    let ext = path.extension()?.to_str()?.to_ascii_lowercase();
    let claimed = super::code::parsed_extensions()
        .chain(
            super::plaintext::SOURCE_TEXT_LANGUAGE_EXTENSIONS
                .iter()
                .copied(),
        )
        .find(|&claimed| claimed == ext)?;
    Some(match claimed {
        "cc" | "cpp" | "cxx" | "h" | "hpp" | "hh" | "hxx" | "cu" | "cuh" | "mm" => "c",
        "jsx" | "cjs" | "mjs" | "ts" | "tsx" | "mts" | "cts" => "js",
        "pyi" => "py",
        "lhs" => "hs",
        "mli" => "ml",
        "hrl" => "erl",
        "exs" => "ex",
        "pm" => "pl",
        "pp" | "dpr" | "lpr" => "pas",
        "f90" | "f95" | "f03" | "f08" | "for" => "f",
        "cbl" | "cpy" => "cob",
        "ads" => "adb",
        "sv" | "svh" => "v",
        "vhd" => "vhdl",
        "sty" => "cls",
        other => other,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::walker::plaintext::SOURCE_TEXT_LANGUAGE_EXTENSIONS;

    fn dominant_source_file_of(root: &Path) -> Option<PathBuf> {
        let source = enumerate_essential_source(root, &DirFilter::new(root))?;
        find_dominant_source_file(&source, find_primary_language(&source)?)
    }

    /// The spine detector runs on whatever path a user points precis
    /// at, including trees that are not repositories — where `DirFilter`
    /// is inert and the heavy-directory blocklist is the only thing
    /// between the scan and a dependency tree.
    #[test]
    fn survey_dominant_file_skips_heavy_dirs_on_non_git_tree() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        std::fs::create_dir_all(root.join("node_modules/dep")).unwrap();
        std::fs::create_dir_all(root.join("src")).unwrap();
        // Bigger than the spine, and in the same language — if it were
        // scanned it would both win the candidate race and dilute the
        // share out of range.
        std::fs::write(
            root.join("node_modules/dep/bundle.js"),
            "x = 1\n".repeat(4000),
        )
        .unwrap();
        std::fs::write(root.join("src/core.js"), "y = 2\n".repeat(100)).unwrap();

        let found = dominant_source_file_of(root);
        assert_eq!(found.as_deref(), Some(root.join("src/core.js").as_path()));
    }

    /// Typed source discovery rejects symlinks (non-following file
    /// types), so a link must not be weighed as source mass or selected
    /// as the spine — otherwise the scheduler boosts a path no walker
    /// ever emits, and the link's target is double-counted.
    #[cfg(unix)]
    #[test]
    fn survey_dominant_file_ignores_in_root_file_symlink() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        std::fs::write(root.join("core.py"), "y = 2\n".repeat(100)).unwrap();
        std::os::unix::fs::symlink("core.py", root.join("alias.py")).unwrap();
        std::fs::write(root.join("helper.py"), "z = 3\n".repeat(60)).unwrap();

        let found = dominant_source_file_of(root);
        assert_eq!(found.as_deref(), Some(root.join("core.py").as_path()));

        // Removing the real file leaves only the link plus a peer; the
        // link must not stand in for the mass it points at.
        std::fs::remove_file(root.join("core.py")).unwrap();
        assert_eq!(
            dominant_source_file_of(root).as_deref(),
            Some(root.join("helper.py").as_path())
        );
    }

    /// Two languages at exactly equal mass have no "primary", and the
    /// answer must not come from hash iteration order — same input, same
    /// output, across processes.
    #[test]
    fn survey_dominant_file_declines_a_tied_primary_language() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        std::fs::write(root.join("core.py"), "y = 2\n".repeat(100)).unwrap();
        std::fs::write(root.join("core.go"), "y = 2\n".repeat(100)).unwrap();
        assert_eq!(dominant_source_file_of(root), None);

        // One byte of lead is enough to make the question answerable.
        std::fs::write(root.join("core.py"), "y = 2\n".repeat(100) + "z").unwrap();
        assert_eq!(
            dominant_source_file_of(root).as_deref(),
            Some(root.join("core.py").as_path())
        );
    }

    /// Generated code and minified bundles are source mass nobody reads;
    /// the spine is the hand-written implementation even when either is
    /// bigger.
    #[test]
    fn survey_dominant_file_passes_over_generated_and_minified_code() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        std::fs::create_dir_all(root.join("src")).unwrap();
        std::fs::write(root.join("src/app.bundle.js"), "var a=1;".repeat(1000)).unwrap();
        std::fs::write(
            root.join("src/client.ts"),
            "// Code generated by protoc. DO NOT EDIT.\n".to_string() + &"x = 1\n".repeat(1000),
        )
        .unwrap();
        std::fs::write(root.join("src/core.ts"), "y = 2\n".repeat(900)).unwrap();
        std::fs::write(root.join("src/util.ts"), "z = 3\n".repeat(200)).unwrap();

        let found = dominant_source_file_of(root);
        assert_eq!(found.as_deref(), Some(root.join("src/core.ts").as_path()));
    }

    /// A checked-in table too large to be read as a spine still counts as
    /// source mass, but is neither read nor picked.
    #[test]
    fn survey_dominant_file_skips_oversized_source() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        std::fs::create_dir_all(root.join("src/generated")).unwrap();
        std::fs::write(
            root.join("src/generated/table.c"),
            "int t[] = {1, 2, 3};\n".repeat(25_000),
        )
        .unwrap();
        std::fs::write(root.join("src/main.c"), "int main(void) { return 0; }\n").unwrap();

        assert_eq!(dominant_source_file_of(root), None);
    }

    #[test]
    fn survey_language_group_covers_every_parsed_extension() {
        let group = |ext: &str| language_group(&PathBuf::from(format!("x.{ext}")));
        for ext in crate::walker::code::parsed_extensions() {
            assert!(group(ext).is_some(), ".{ext} is parsed but has no family");
            assert!(
                !SOURCE_TEXT_LANGUAGE_EXTENSIONS.contains(&ext),
                ".{ext} is claimed by both the code engine and the plaintext fallback"
            );
        }
        assert_eq!(group("MTS"), group("js"));
        assert_eq!(group("cpp"), group("h"));
        assert_eq!(group("yaml"), None);
    }
}
