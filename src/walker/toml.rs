//! TOML content walker. Identifies `[package]`, `[features]`, `[dependencies]`,
//! `[dev-dependencies]`, and `[workspace.*]` sections in a manifest and emits
//! one batch per section. Section boundaries are line-identifiable, so this
//! avoids a real TOML parser — at the cost of misclassifying contrived
//! inline-table headers (acceptable for valid Cargo manifests).

use std::path::Path;

use crate::batch::{BatchDraft, RenderedLine};

use super::lines_for_file;

pub fn emit_batches(path: &Path, source: &str) -> Vec<BatchDraft> {
    parse_sections(source)
        .into_iter()
        .filter_map(|section| {
            let value = value_for_section(&section.name)?;
            let lines = section
                .lines
                .into_iter()
                .filter(|(_, t)| !t.trim().is_empty())
                .map(|(n, t)| (n, RenderedLine::Full(t)));
            lines_for_file(path, lines, value)
        })
        .collect()
}

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
