//! The repository survey: which language the tree is written in, which
//! single source file, if any, carries a dominant share of it, and which
//! directories hold most of it. One capped walk over the essential source
//! answers all three.

use std::collections::{HashMap, HashSet, VecDeque};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::fs_util::{DirFilter, PROBE_ENTRY_CAP};
use crate::render::Source;

use super::plaintext::is_derived_artifact_name;

/// Minimum share of the tree's essential source bytes for the largest
/// source file to count as the repository's spine.
const DOMINANT_SOURCE_MASS_SHARE: f64 = 0.20;

/// Upper bound on a spine file's size, checked before the file is read.
/// Past this, a single file is a generated table or an amalgamated bundle
/// rather than something a reader is meant to read more of.
const DOMINANT_SOURCE_MAX_FILE_BYTES: u64 = 400_000;

/// Directories that hold what a web page loads (`www/`, `static/`),
/// not the source it was built from.
const ASSET_DIR_NAMES: &[&str] = &["www", "static", "public", "assets"];

/// The tree's essential source files, as one walk: each file's path,
/// byte length and language family.
pub(super) type EssentialSource = Vec<(PathBuf, u64, &'static str)>;

/// Walk `root` for its essential source. Non-essential subtrees (tests,
/// examples, vendored, tooling) are not entered, since nothing under one
/// counts, so a large test corpus can neither dilute the mass nor spend
/// the walk's budget. Nor are asset folders, whose scripts are copies
/// served to a browser, and a derived artifact (`app.min.js`) is not
/// weighed: a bundled library can outweigh the code the repository is
/// written in. `None` once it has read [`PROBE_ENTRY_CAP`] entries.
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
    let mut files = Vec::new();
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
                    && !ASSET_DIR_NAMES.contains(&&*name)
                    && crate::value::non_essential_factor(&path, root) >= 1.0
                {
                    queue.push_back(path);
                }
            } else if file_type.is_file() {
                let Some(language) = language_group(&path) else {
                    continue;
                };
                if is_derived_artifact_name(
                    &entry.file_name().to_string_lossy().to_ascii_lowercase(),
                ) || crate::value::non_essential_factor(&path, root) < 1.0
                {
                    continue;
                }
                let Ok(len) = entry.metadata().map(|m| m.len()) else {
                    continue;
                };
                files.push((path, len, language));
            }
        }
    }
    Some(files)
}

/// Select a source file in the `primary` language that carries at least
/// [`DOMINANT_SOURCE_MASS_SHARE`] of the essential source — the largest
/// one when several do. A candidate over
/// [`DOMINANT_SOURCE_MAX_FILE_BYTES`] is never read, and one that `read`
/// refuses or whose text reads as machine-generated (a banner, or
/// minified line lengths) never wins.
pub(super) fn find_dominant_source_file(
    source: &EssentialSource,
    primary: &str,
    read: impl Fn(&Path) -> Option<Arc<Source>>,
) -> Option<PathBuf> {
    // The spine has to be written in the language the repository is
    // written in — a vendored JS bundle inside a Go tree is source mass
    // but it is not what the repo is about.
    let total: u64 = source.iter().map(|(_, len, _)| len).sum();
    if total == 0 {
        return None;
    }
    source
        .iter()
        .filter(|(_, len, language)| {
            *language == primary
                && *len <= DOMINANT_SOURCE_MAX_FILE_BYTES
                && *len as f64 / total as f64 >= DOMINANT_SOURCE_MASS_SHARE
        })
        .filter(|(path, _, _)| {
            read(path).is_some_and(|text| !super::plaintext::is_machine_generated_text(&text))
        })
        .max_by(|a, b| a.1.cmp(&b.1).then_with(|| b.0.cmp(&a.0)))
        .map(|(path, _, _)| path.clone())
}

/// The directories below `root` that each hold more than half of the
/// essential source bytes: at most one per level, so they form one chain
/// down from the root, the repository's source spine.
pub(super) fn find_source_spine(source: &EssentialSource, root: &Path) -> HashSet<PathBuf> {
    let total: u64 = source.iter().map(|(_, len, _)| len).sum();
    let mut bytes_under: HashMap<&Path, u64> = HashMap::new();
    for (path, len, _) in source {
        for dir in path.ancestors().skip(1).take_while(|dir| *dir != root) {
            *bytes_under.entry(dir).or_default() += len;
        }
    }
    bytes_under
        .into_iter()
        .filter(|&(_, bytes)| 2 * bytes > total)
        .map(|(dir, _)| dir.to_path_buf())
        .collect()
}

/// The language with the most bytes, ranked over a sorted vector rather
/// than the hash map's iteration order; `None` on a tie for the lead.
pub(super) fn find_primary_language(source: &EssentialSource) -> Option<&'static str> {
    let mut per_language: HashMap<&'static str, u64> = HashMap::new();
    for &(_, len, language) in source {
        *per_language.entry(language).or_default() += len;
    }
    let mut by_mass: Vec<(&'static str, u64)> = per_language.into_iter().collect();
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
        let ctx = crate::walker::WalkCtx::new(root.to_path_buf());
        ctx.dominant_source_file().map(Path::to_path_buf)
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

    /// The spine is the hand-written implementation. The heavy-directory
    /// blocklist keeps a dependency tree out of the scan even outside a
    /// repository, where `DirFilter` is inert; generated code and minified
    /// bundles are source mass nobody reads; and a table too large to read
    /// still counts as mass but is never picked.
    #[test]
    fn survey_dominant_file_is_the_hand_written_implementation() {
        let cases = [
            (
                vec![
                    ("node_modules/dep/bundle.js", "x = 1\n".repeat(4000)),
                    ("src/core.js", "y = 2\n".repeat(100)),
                ],
                Some("src/core.js"),
            ),
            (
                vec![
                    ("src/app.bundle.js", "var a=1;".repeat(1000)),
                    (
                        "src/client.ts",
                        "// Code generated by protoc. DO NOT EDIT.\n".to_string()
                            + &"x = 1\n".repeat(1000),
                    ),
                    ("src/core.ts", "y = 2\n".repeat(900)),
                    ("src/util.ts", "z = 3\n".repeat(200)),
                ],
                Some("src/core.ts"),
            ),
            (
                vec![
                    (
                        "src/generated/table.c",
                        "int t[] = {1, 2, 3};\n".repeat(25_000),
                    ),
                    ("src/main.c", "int main(void) { return 0; }\n".into()),
                ],
                None,
            ),
        ];
        for (files, expected) in cases {
            let tmp = tempfile::tempdir().unwrap();
            for (path, text) in files {
                let path = tmp.path().join(path);
                std::fs::create_dir_all(path.parent().unwrap()).unwrap();
                std::fs::write(path, text).unwrap();
            }
            let expected = expected.map(|path| tmp.path().join(path));
            assert_eq!(dominant_source_file_of(tmp.path()), expected);
        }
    }

    #[test]
    fn survey_source_spine_is_the_chain_of_majority_directories() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        for (path, lines) in [
            ("core/api.py", 30),
            ("core/engine/run.py", 60),
            ("side/tool.py", 20),
            ("side/app.min.js", 1000),
            ("inst/www/shared/jquery.js", 1000),
            ("tests/test_all.py", 1000),
        ] {
            let path = root.join(path);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, "x = 1\n".repeat(lines)).unwrap();
        }
        let ctx = crate::walker::WalkCtx::new(root.to_path_buf());
        let on_spine = |dir: &str| ctx.is_on_source_spine(&root.join(dir));
        assert!(on_spine("core") && on_spine("core/engine"));
        assert!(!ctx.is_on_source_spine(root) && !on_spine("side") && !on_spine("tests"));
        assert!(!on_spine("inst") && !on_spine("inst/www"));
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
