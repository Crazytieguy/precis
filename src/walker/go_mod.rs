//! `go.mod` / `go.work` batches: the module's identity directives, then
//! the file itself (whole when small, otherwise without its indirect
//! requires).

use std::path::Path;

use crate::batch::{Batch, BatchKey, GoModKey};
use crate::content::BatchContent;
use crate::value::mix_signals;

use super::{FileLines, WalkCtx, file_depth_factor, fs::list_dir, single_file_lines_content};

const GOMOD_WHOLE_LINE_CAP: usize = 72;

pub fn expand_in_dir(dir: &Path, ctx: &WalkCtx) -> Vec<Batch<BatchKey>> {
    let mut out = Vec::new();
    for (name, kind) in list_dir(dir, ctx.dir_filter()) {
        if !matches!(kind, crate::fs_util::EntryKind::File) {
            continue;
        }
        if !matches!(name.as_str(), "go.mod" | "go.work") {
            continue;
        }
        let path = dir.join(name);
        let identity_emitted = if let Some(content) = build_gomod_identity_content(&path, ctx) {
            out.push(Batch {
                key: GoModKey::Identity { file: path.clone() }.into(),
                predecessor: None,
                content,
                value: gomod_identity_value(&path, ctx),
            });
            true
        } else {
            false
        };
        let Some(content) = build_gomod_content(&path, ctx) else {
            continue;
        };
        // Only declare the identity batch as predecessor when it was
        // actually emitted — otherwise the GoMod batch would orphan
        // itself on a never-resolved predecessor key.
        let predecessor =
            identity_emitted.then(|| BatchKey::GoMod(GoModKey::Identity { file: path.clone() }));
        out.push(Batch {
            key: GoModKey::File { file: path.clone() }.into(),
            predecessor,
            content,
            value: gomod_value(&path, ctx),
        });
    }
    out
}

/// Identity slice of `go.mod` / `go.work` — `module`/`go`/`toolchain`
/// directives only; block bodies are skipped.
fn build_gomod_identity_content(file: &Path, ctx: &WalkCtx) -> Option<BatchContent> {
    let source = ctx.read_source(file)?;
    let mut lines = Vec::new();
    let mut in_block = false;
    for (i, raw) in source.lines().enumerate() {
        let trimmed = raw.trim();
        // Skip block bodies (`require ( … )`, etc.) so block entries
        // can't shadow identity keywords; go.mod allows directives in
        // any order, so module/go/toolchain are collected wherever
        // they appear at top level.
        if trimmed.ends_with('(') && !trimmed.starts_with("//") {
            in_block = true;
            continue;
        }
        if in_block {
            if trimmed == ")" {
                in_block = false;
            }
            continue;
        }
        let first = trimmed.split_whitespace().next().unwrap_or("");
        if matches!(first, "module" | "go" | "toolchain") {
            lines.push(i + 1);
        }
    }
    if lines.is_empty() {
        return None;
    }
    single_file_lines_content(file, &source, FileLines::new(lines))
}

/// `GoMod` content — whole for compact module files; sampled for large
/// generated dependency closures.
fn build_gomod_content(file: &Path, ctx: &WalkCtx) -> Option<BatchContent> {
    let source = ctx.read_source(file)?;
    let total_lines = source.lines().count();
    if total_lines == 0 {
        return None;
    }
    if total_lines <= GOMOD_WHOLE_LINE_CAP {
        return single_file_lines_content(
            file,
            &source,
            FileLines::new((1..=total_lines).collect()),
        );
    }

    single_file_lines_content(file, &source, bounded_gomod_lines(&source))
}

fn bounded_gomod_lines(source: &str) -> FileLines {
    let src_lines: Vec<&str> = source.lines().collect();
    let mut full = Vec::new();
    let mut i = 0;
    while i < src_lines.len() {
        let line_no = i + 1;
        let trimmed = src_lines[i].trim();
        if let Some(block) = gomod_block_start(trimmed) {
            let start_line = line_no;
            let mut body = Vec::new();
            i += 1;
            while i < src_lines.len() && src_lines[i].trim() != ")" {
                if keep_gomod_block_entry(block, src_lines[i].trim()) {
                    body.push(i + 1);
                }
                i += 1;
            }
            let close_line = (i < src_lines.len() && src_lines[i].trim() == ")").then_some(i + 1);
            if !body.is_empty() {
                full.push(start_line);
                full.extend(body);
                if let Some(close_line) = close_line {
                    full.push(close_line);
                }
            }
            if close_line.is_some() {
                i += 1;
            }
            continue;
        }
        if keep_gomod_directive_line(trimmed) {
            full.push(line_no);
        }
        i += 1;
    }
    full.sort_unstable();
    full.dedup();
    let ellipses = gomod_ellipses_for_gaps(&full, &src_lines);
    FileLines::new(full).with_ellipses(ellipses)
}

fn gomod_ellipses_for_gaps(full: &[usize], src_lines: &[&str]) -> Vec<usize> {
    if full.is_empty() {
        return Vec::new();
    }
    let mut boundaries = full.to_vec();
    boundaries.push(src_lines.len() + 1);
    let mut ellipses = Vec::new();
    for pair in boundaries.windows(2) {
        let omitted_start = pair[0] + 1;
        let omitted_end = pair[1].saturating_sub(1);
        if omitted_start > omitted_end {
            continue;
        }
        let has_substantive_omission = (omitted_start..=omitted_end).any(|line_no| {
            src_lines
                .get(line_no - 1)
                .is_some_and(|line| is_substantive_gomod_line(line.trim()))
        });
        if has_substantive_omission {
            ellipses.push(omitted_start);
        }
    }
    ellipses
}

fn is_substantive_gomod_line(trimmed: &str) -> bool {
    !trimmed.is_empty() && !trimmed.starts_with("//") && trimmed != ")"
}

fn gomod_block_start(trimmed: &str) -> Option<&str> {
    let (first, rest) = trimmed.split_once(char::is_whitespace)?;
    (rest.trim() == "(" && matches!(first, "require" | "replace" | "exclude" | "retract" | "use"))
        .then_some(first)
}

fn keep_gomod_directive_line(trimmed: &str) -> bool {
    let Some(first) = trimmed.split_whitespace().next() else {
        return false;
    };
    matches!(
        first,
        "module" | "go" | "toolchain" | "replace" | "exclude" | "retract" | "use"
    ) || (first == "require" && !trimmed.contains("// indirect"))
}

fn keep_gomod_block_entry(block: &str, trimmed: &str) -> bool {
    if trimmed.is_empty() || trimmed.starts_with("//") {
        return false;
    }
    block != "require" || !trimmed.contains("// indirect")
}

fn gomod_value(file: &Path, ctx: &WalkCtx) -> f64 {
    // Identity directives split into [`GoModKey::Identity`]; this
    // batch reflects the residual require / replace / exclude /
    // retract content.
    mix_signals(0.80, 0.60, 0.5, file_depth_factor(file, ctx, false))
}

fn gomod_identity_value(file: &Path, ctx: &WalkCtx) -> f64 {
    mix_signals(0.90, 0.75, 0.35, file_depth_factor(file, ctx, false))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::Render;

    fn gomod_lines(src: &str) -> Vec<usize> {
        gomod_rendered_lines(src)
            .into_iter()
            .map(|(line, _)| line)
            .collect()
    }

    fn gomod_rendered_lines(src: &str) -> Vec<(usize, Render)> {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("go.mod");
        std::fs::write(&path, src).unwrap();
        let ctx = WalkCtx::new(dir.path().to_path_buf());
        let content = build_gomod_content(&path, &ctx).expect("emits content");
        let BatchContent::Lines { spans } = content else {
            panic!("expected Lines content");
        };
        spans
            .iter()
            .flat_map(|span| (span.start..=span.end).map(|line| (line, span.render.clone())))
            .collect()
    }

    fn gomod_identity_lines(src: &str) -> Vec<usize> {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("go.mod");
        std::fs::write(&path, src).unwrap();
        let ctx = WalkCtx::new(dir.path().to_path_buf());
        let content = build_gomod_identity_content(&path, &ctx).expect("emits content");
        let BatchContent::Lines { spans } = content else {
            panic!("expected Lines content");
        };
        spans
            .iter()
            .flat_map(|span| span.start..=span.end)
            .collect()
    }

    #[test]
    fn go_mod_keeps_indirect_requires_replace_and_retract() {
        let src = "\
module example.com/foo

go 1.22

require (
\tgithub.com/x/y v1.0.0
\tgithub.com/x/z v0.5.0 // indirect
)

replace github.com/x/y => github.com/forked/y v2.0.0

retract v0.1.0
";
        let lines = gomod_lines(src);
        assert!(lines.contains(&1), "module clause kept");
        assert!(lines.contains(&5), "require ( kept");
        assert!(lines.contains(&6), "direct require kept");
        assert!(lines.contains(&7), "indirect require kept");
        assert!(lines.contains(&8), "require ) kept");
        assert!(lines.contains(&10), "replace kept");
        assert!(lines.contains(&12), "retract kept");
    }

    #[test]
    fn go_mod_identity_collects_module_go_and_toolchain_directives() {
        let src = "\
module example.com/foo

go 1.22

toolchain go1.22.5

require (
\tgithub.com/x/y v1.0.0
)

replace github.com/x/y => github.com/forked/y v2.0.0
";
        let lines = gomod_identity_lines(src);
        assert!(lines.contains(&1), "module clause kept");
        assert!(lines.contains(&3), "go version kept");
        assert!(lines.contains(&5), "toolchain kept");
        assert!(!lines.contains(&7), "require ( excluded from identity");
        assert!(!lines.contains(&8), "require body excluded from identity");
        assert!(
            !lines.contains(&11),
            "replace excluded from identity (not an identity directive)"
        );
    }

    #[test]
    fn go_mod_identity_handles_minimal_module() {
        let src = "module example.com/foo\n\ngo 1.22\n";
        let lines = gomod_identity_lines(src);
        assert!(lines.contains(&1));
        assert!(lines.contains(&3));
    }

    #[test]
    fn go_mod_emits_whole_file_when_no_indirect_lines_present() {
        let src = "module example.com/foo\n\ngo 1.22\n";
        let lines = gomod_lines(src);
        assert!(!lines.is_empty());
    }

    #[test]
    fn go_mod_over_cap_elides_indirect_require_tail() {
        let mut src = String::from(
            "\
module example.com/foo

go 1.22

require (
\tgithub.com/direct/a v1.0.0
",
        );
        let first_indirect_line = src.lines().count() + 1;
        for i in 0..65 {
            src.push_str(&format!("\tgithub.com/indirect/{i} v0.0.1 // indirect\n"));
        }
        let close_line = src.lines().count() + 1;
        src.push_str(")\n\n");
        let replace_line = src.lines().count() + 1;
        src.push_str("replace github.com/direct/a => ../a\n\n");
        let exclude_line = src.lines().count() + 1;
        src.push_str("exclude github.com/bad/module v1.0.0\n");

        let rendered = gomod_rendered_lines(&src);
        assert!(rendered.contains(&(1, Render::Full)), "module kept");
        assert!(rendered.contains(&(3, Render::Full)), "go directive kept");
        assert!(
            rendered.contains(&(5, Render::Full)),
            "require block opener kept"
        );
        assert!(rendered.contains(&(6, Render::Full)), "direct require kept");
        assert!(
            rendered.contains(&(close_line, Render::Full)),
            "require block closer kept"
        );
        assert!(
            rendered.contains(&(replace_line, Render::Full)),
            "replace directive kept"
        );
        assert!(
            rendered.contains(&(exclude_line, Render::Full)),
            "exclude directive kept"
        );
        assert!(
            !rendered.contains(&(first_indirect_line, Render::Full)),
            "indirect require must not render in full"
        );
        assert!(
            rendered.contains(&(first_indirect_line, Render::Ellipsis)),
            "indirect require tail should be represented by one ellipsis"
        );
    }
}
