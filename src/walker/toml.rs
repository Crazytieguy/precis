//! TOML content walker. First-pass Tier 1 scope: identify `[package]`,
//! `[features]`, and `[dependencies]` / `[dev-dependencies]` sections in a
//! Cargo.toml-style manifest and emit one batch per section. Uses simple
//! line-scanning (no TOML parser) since section boundaries are
//! line-identifiable by their `[header]` lines.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::batch::{BatchContent, BatchDraft, RenderedLine};

/// Emit per-section batches for a TOML manifest.
pub fn emit_batches(path: &Path, source: &str) -> Vec<BatchDraft> {
    parse_sections(source)
        .into_iter()
        .filter_map(|section| {
            let value = value_for_section(&section.name)?;
            if section.lines.is_empty() {
                return None;
            }
            let mut lines = BTreeMap::new();
            for (n, text) in section.lines {
                if text.trim().is_empty() {
                    continue;
                }
                lines.insert(n, RenderedLine::Full(text));
            }
            if lines.is_empty() {
                return None;
            }
            let mut file_map: BTreeMap<PathBuf, BTreeMap<usize, RenderedLine>> = BTreeMap::new();
            file_map.insert(path.to_path_buf(), lines);
            Some(BatchDraft {
                content: BatchContent::Lines(file_map),
                value,
            })
        })
        .collect()
}

/// Priority value for a TOML section. None → skip (don't emit a batch).
fn value_for_section(name: &str) -> Option<f64> {
    match name {
        "package" | "workspace" | "workspace.package" => Some(900.0),
        "features" => Some(850.0),
        "dependencies" | "dev-dependencies" | "build-dependencies" => Some(600.0),
        "workspace.dependencies" => Some(600.0),
        _ => None,
    }
}

struct Section {
    name: String,
    lines: Vec<(usize, String)>,
}

fn parse_sections(source: &str) -> Vec<Section> {
    let mut sections: Vec<Section> = Vec::new();
    let mut current: Option<Section> = None;
    for (i, raw) in source.lines().enumerate() {
        let line_no = i + 1;
        let trimmed = raw.trim_start();
        // `[header]` starts a new section. Reject `[[header]]` (array-of-tables)
        // and inline-array contexts by checking the bracket comes first and is
        // followed by a non-`[` character.
        if let Some(after_open) = trimmed.strip_prefix('[')
            && !after_open.starts_with('[')
            && let Some(name) = after_open.split(']').next()
            && !name.trim().is_empty()
        {
            if let Some(sec) = current.take() {
                sections.push(sec);
            }
            current = Some(Section {
                name: name.trim().to_string(),
                lines: Vec::new(),
            });
        }
        if let Some(sec) = current.as_mut() {
            sec.lines.push((line_no, raw.to_string()));
        }
    }
    if let Some(sec) = current.take() {
        sections.push(sec);
    }
    sections
}
