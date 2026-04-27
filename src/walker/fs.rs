//! Filesystem walker. Discovers directories, emits listings, and surveys
//! file sets for per-language walkers. Only does `read_dir` — never reads
//! file contents. Pure listing lives in [`crate::fs_util::list_dir`];
//! this module holds walker-specific policy (heavy-directory skip,
//! per-language file enumeration).

use std::path::{Path, PathBuf};

use crate::batch::{Batch, BatchKey, FsKey};
use crate::content::{BatchContent, FsEntries, FsGroup};
use crate::fs_util::EntryKind;
pub use crate::fs_util::list_dir;
use crate::value::mix_signals;

use super::{WalkCtx, path_depth_factor};

/// Seed: list the root directory.
pub fn seed(ctx: &WalkCtx) -> Vec<Batch<BatchKey>> {
    dir_listing_batch(ctx.root().to_path_buf(), ctx)
        .into_iter()
        .collect()
}

/// Subdirectory listings for the dir whose listing was just scheduled.
/// File-based batches are emitted by per-language walkers; the FS walker
/// only owns directory recursion.
pub fn expand_subdirs(dir: &Path, ctx: &WalkCtx) -> Vec<Batch<BatchKey>> {
    let children = list_dir(dir);
    let mut out = Vec::new();
    for (name, kind) in children {
        if matches!(kind, EntryKind::Directory)
            && !should_skip_dir(&name)
            && let Some(batch) = dir_listing_batch(dir.join(&name), ctx)
        {
            out.push(batch);
        }
    }
    out
}

// ---- additional helpers ----

/// Enumerate the files under `dir` (non-recursive) matching an extension.
/// Returns absolute paths. Used by per-language walkers to build cross-file
/// batch scopes without opening any file.
pub fn files_with_extension(dir: &Path, ext: &str) -> Vec<PathBuf> {
    files_with_any_extension(dir, &[ext])
}

/// Like [`files_with_extension`] but accepts any of several extensions in a
/// single `read_dir` pass — convenient for walkers that handle paired
/// extensions (e.g. `.ts` + `.tsx`).
pub fn files_with_any_extension(dir: &Path, exts: &[&str]) -> Vec<PathBuf> {
    let Ok(read_dir) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut out: Vec<PathBuf> = read_dir
        .flatten()
        .filter_map(|e| {
            let path = e.path();
            if !path.is_file() {
                return None;
            }
            let actual = path.extension().and_then(|e| e.to_str())?;
            exts.iter()
                .any(|ext| actual.eq_ignore_ascii_case(ext))
                .then_some(path)
        })
        .collect();
    out.sort();
    out
}

/// Recursively walk `dir` for files whose extension matches (case-insensitive).
/// Used for cross-file batches that span subdirectories (`src/kv/mod.rs` etc).
pub fn files_with_extension_recursive(dir: &Path, ext: &str) -> Vec<PathBuf> {
    let mut out = Vec::new();
    walk_files_recursive(dir, ext, &mut out);
    out.sort();
    out
}

fn walk_files_recursive(dir: &Path, ext: &str, out: &mut Vec<PathBuf>) {
    let Ok(read_dir) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in read_dir.flatten() {
        let path = entry.path();
        if path.is_dir() {
            let name = entry.file_name().to_string_lossy().into_owned();
            if !should_skip_dir(&name) {
                walk_files_recursive(&path, ext, out);
            }
        } else if path
            .extension()
            .and_then(|e| e.to_str())
            .is_some_and(|actual| actual.eq_ignore_ascii_case(ext))
        {
            out.push(path);
        }
    }
}

fn dir_listing_batch(dir: PathBuf, ctx: &WalkCtx) -> Option<Batch<BatchKey>> {
    let children = list_dir(&dir);
    if children.is_empty() {
        return None;
    }
    let paths: Vec<PathBuf> = children.into_keys().map(PathBuf::from).collect();
    let value = dir_listing_value(&dir, ctx);
    Some(Batch {
        key: FsKey::DirListing { dir: dir.clone() }.into(),
        predecessor: None,
        content: BatchContent::Fs {
            groups: vec![FsGroup {
                parent: dir,
                entries: FsEntries::Listed(paths),
            }],
        },
        value,
    })
}

fn dir_listing_value(dir: &Path, ctx: &WalkCtx) -> f64 {
    let (cat, fu, ztu) = if dir == ctx.root() {
        (0.95, 0.6, 0.5)
    } else {
        (0.5, 0.45, 0.25)
    };
    mix_signals(cat, fu, ztu, path_depth_factor(dir, ctx))
}

/// Directories the walker never recurses into. Matches common
/// heavy/generated trees. Note: these directories still appear in
/// listings (via `fs_util::list_dir`); this only affects walker
/// traversal and per-language file enumeration.
fn should_skip_dir(name: &str) -> bool {
    matches!(
        name,
        "target" | "node_modules" | ".git" | "dist" | "build" | ".next" | "__pycache__"
    )
}
