//! Filesystem walker. Discovers directories, emits listings, and surveys
//! file sets for per-language walkers. Only does `read_dir` — never reads
//! file contents.

use std::collections::BTreeMap;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

use crate::batch::{BatchContent, BatchKey, EntryKind, FsKey, ResolvedBatch, ValueSignals};
use crate::value::{depth_factor, non_essential_factor};

use super::{Candidate, WalkCtx};

/// Seed: list the root directory.
pub fn seed(ctx: &WalkCtx) -> Vec<Candidate> {
    vec![dir_listing_candidate(ctx.root().to_path_buf(), 0)]
}

/// Expand a scheduled `FsKey::DirListing` into successor candidates:
/// subdirectory listings for each subdir. File-based candidates are
/// emitted by per-language walkers (see [`multi::expand`]).
pub fn expand(scheduled: &BatchKey, ctx: &WalkCtx) -> Vec<Candidate> {
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

pub fn materialize(key: &BatchKey, ctx: &WalkCtx) -> Option<ResolvedBatch> {
    let BatchKey::Fs(FsKey::DirListing { dir }) = key else {
        return None;
    };
    let children = list_dir(dir);
    if children.is_empty() {
        return None;
    }
    Some(ResolvedBatch {
        content: BatchContent::FileSystemEntries {
            parent: dir.clone(),
            children,
        },
        signals: dir_listing_signals_for_path(dir, dir == ctx.root(), ctx.depth_from_root(dir)),
    })
}

/// Read a directory's immediate children into a name-keyed map.
pub fn list_dir(path: &Path) -> BTreeMap<OsString, EntryKind> {
    let Ok(read_dir) = std::fs::read_dir(path) else {
        return BTreeMap::new();
    };
    read_dir
        .flatten()
        .filter_map(|e| {
            let name = e.file_name();
            if should_skip_entry(&name) {
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
        let name = entry.file_name();
        if should_skip_entry(&name) {
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

fn dir_listing_candidate(dir: PathBuf, depth: usize) -> Candidate {
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

/// Directories we never enter. Matches common heavy/generated trees.
fn should_skip_dir(name: &OsString) -> bool {
    let Some(s) = name.to_str() else {
        return false;
    };
    matches!(
        s,
        "target" | "node_modules" | ".git" | "dist" | "build" | ".next" | "__pycache__"
    )
}

/// Entries we silently skip from listings.
fn should_skip_entry(name: &OsString) -> bool {
    let Some(s) = name.to_str() else {
        return false;
    };
    // Hidden dotfiles except for a handful of commonly-referenced ones.
    if s.starts_with('.')
        && !matches!(
            s,
            ".gitignore" | ".github" | ".cargo" | ".rustfmt.toml" | ".config"
        )
    {
        return true;
    }
    false
}
