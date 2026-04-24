//! Filesystem walker. Discovers directories, emits listings, and surveys
//! file sets for per-language walkers. Only does `read_dir` — never reads
//! file contents. Pure listing lives in [`crate::fs_util::list_dir`];
//! this module holds walker-specific policy (heavy-directory skip,
//! per-language file enumeration).

use std::path::{Path, PathBuf};

use crate::batch::{
    BatchContent, BatchKey, EntryKind, FsGroup, FsKey, ResolvedBatch, ValueSignals,
};
pub use crate::fs_util::list_dir;
use crate::value::{depth_factor, non_essential_factor};

use super::{Candidate, WalkCtx};

/// Seed: list the root directory.
pub fn seed(ctx: &WalkCtx) -> Vec<Candidate<BatchKey>> {
    vec![dir_listing_candidate(ctx.root().to_path_buf(), 0)]
}

/// Expand a scheduled `FsKey::DirListing` into successor candidates:
/// subdirectory listings for each subdir. File-based candidates are
/// emitted by per-language walkers (see [`multi::expand`]).
pub fn expand(scheduled: &BatchKey, ctx: &WalkCtx) -> Vec<Candidate<BatchKey>> {
    let BatchKey::Fs(FsKey::DirListing { dir }) = scheduled else {
        return Vec::new();
    };
    let children = list_dir(dir);
    let mut out = Vec::new();
    for (name, kind) in children {
        if matches!(kind, EntryKind::Directory) && !should_skip_dir(&name) {
            let sub = dir.join(&name);
            let depth = ctx.depth_from_root(&sub);
            out.push(dir_listing_candidate(sub, depth));
        }
    }
    out
}

// ---- additional helpers ----

pub fn materialize(key: &BatchKey, ctx: &WalkCtx) -> Option<ResolvedBatch> {
    let BatchKey::Fs(FsKey::DirListing { dir }) = key else {
        return None;
    };
    let children = list_dir(dir);
    if children.is_empty() {
        return None;
    }
    Some(ResolvedBatch {
        content: BatchContent::Fs {
            groups: vec![FsGroup {
                parent: dir.clone(),
                children,
            }],
        },
        signals: dir_listing_signals_for_path(dir, dir == ctx.root(), ctx.depth_from_root(dir)),
    })
}

/// Enumerate the files under `dir` (non-recursive) matching an extension.
/// Returns absolute paths. Used by per-language walkers to build cross-file
/// batch scopes without opening any file.
pub fn files_with_extension(dir: &Path, ext: &str) -> Vec<PathBuf> {
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
            if actual.eq_ignore_ascii_case(ext) {
                Some(path)
            } else {
                None
            }
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
        let name_os = entry.file_name();
        let name = name_os.to_string_lossy();
        if name == crate::fs_util::PRECIS_PIN_FILE {
            continue;
        }
        let path = entry.path();
        if path.is_dir() {
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

fn dir_listing_candidate(dir: PathBuf, depth: usize) -> Candidate<BatchKey> {
    let is_root = depth == 0;
    let signals = dir_listing_signals_for_path(&dir, is_root, depth);
    // Cost hint: small — a listing of ~10 entries is ~30-60 tokens.
    let cost_hint = 40;
    Candidate::new(FsKey::DirListing { dir }.into(), signals, cost_hint)
}

fn dir_listing_signals(is_root: bool, depth: usize) -> ValueSignals {
    if is_root {
        ValueSignals {
            catastrophic_omission: 0.95,
            follow_up_minimization: 0.6,
            zero_tool_call_understanding: 0.5,
            depth_factor: 1.0,
        }
    } else {
        ValueSignals {
            catastrophic_omission: 0.5,
            follow_up_minimization: 0.45,
            zero_tool_call_understanding: 0.25,
            depth_factor: depth_factor(depth),
        }
    }
}

/// Variant of `dir_listing_signals` that takes the dir path so the
/// non-essential-directory discount applies (tests/, examples/, benches/).
fn dir_listing_signals_for_path(
    dir: &std::path::Path,
    is_root: bool,
    depth: usize,
) -> ValueSignals {
    let mut s = dir_listing_signals(is_root, depth);
    s.depth_factor *= non_essential_factor(dir);
    s
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
