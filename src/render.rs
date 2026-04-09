use crate::format::{fmt_line, format_symbol_name, truncation_marker};
use crate::layout;
use crate::parse;
use crate::schedule::{self, RenderPlanItem, Schedule, SymbolRenderSpec};
use crate::Corpus;

// ---------------------------------------------------------------------------
// Output assembly — turning a schedule into text
// ---------------------------------------------------------------------------

/// Render output from a computed schedule.
///
/// Iterates the schedule's render plan (ordered files and directory markers)
/// and renders each symbol according to its pre-resolved render spec. The
/// renderer has no knowledge of stages, groups, or the scheduling model — it
/// just assembles text from the decisions the scheduler already made.
pub fn render_scheduled(corpus: &Corpus<'_>, sched: &Schedule) -> String {
    let mut out = String::new();

    for item in &sched.render_plan {
        match item {
            RenderPlanItem::DirectoryMarker(dir) => {
                if !out.is_empty() { out.push('\n'); }
                out.push_str(&schedule::directory_marker_text(dir));
            }
            RenderPlanItem::File(file_idx) => {
                let file_idx = *file_idx;
                let fd = &corpus.files[file_idx];

                if !out.is_empty() { out.push('\n'); }
                out.push_str(&format!("{}\n", fd.info.relative_path.display()));

                let source = match &fd.source {
                    Some(s) => s,
                    None => continue,
                };
                let lines: Vec<&str> = source.lines().collect();
                let symbols = &fd.symbols;

                // Track the highest source line emitted so far (exclusive) to
                // deduplicate overlapping ranges (e.g. Go grouped const block +
                // individual const_spec symbols sharing the same first line).
                let mut emitted_up_to: usize = 0;

                for (sym_idx, sym) in symbols.iter().enumerate() {
                    let spec = match &sched.symbol_specs[file_idx][sym_idx] {
                        Some(s) => s,
                        None => continue,
                    };

                    render_symbol(
                        &mut out,
                        &lines,
                        sym,
                        spec,
                        &mut emitted_up_to,
                    );
                }
            }
        }
    }

    out
}

/// Render a single symbol according to its pre-resolved render spec.
/// Parent body ranges are truncated at the first child's doc_start, but
/// overlaps can still occur (e.g. Go grouped const blocks). The
/// `emitted_up_to` high-water mark deduplicates within a file.
fn render_symbol(
    out: &mut String,
    lines: &[&str],
    sym: &parse::Symbol,
    spec: &SymbolRenderSpec,
    emitted_up_to: &mut usize,
) {
    let layout = &sym.layout;
    let sym_line_0 = layout.sym_line_0;

    let doc_n = spec.doc_lines;
    let body_n = spec.body_lines;

    if !spec.show_name {
        return;
    }

    // Composite symbols: render as a single line prefix, progressively extended.
    if !sym.composed_prefix_lens.is_empty() {
        if sym_line_0 < *emitted_up_to {
            return;
        }
        let line = lines.get(sym_line_0).copied().unwrap_or("");
        let pl = &sym.composed_prefix_lens;
        // Names shows prefix[0], body line n extends to prefix[n]
        let depth = body_n.saturating_add(1).min(pl.len());
        let prefix_len = pl[depth - 1].min(line.len());
        let is_complete = depth >= pl.len();
        out.push_str(&fmt_line(sym_line_0, &line[..prefix_len]));
        if !is_complete {
            // Replace trailing newline with inline truncation marker
            out.pop(); // remove '\n' from fmt_line
            out.push_str(" …\n");
        }
        *emitted_up_to = sym_line_0 + 1;
        return;
    }

    // Names only
    if !spec.show_sig && doc_n == 0 && body_n == 0 {
        if sym_line_0 >= *emitted_up_to {
            if sym.kind.is_section_like() {
                // Sections/module docs: show the full line without truncation
                // marker. The text IS the name — truncating it looks broken.
                let line = lines.get(sym_line_0).copied().unwrap_or("");
                out.push_str(&fmt_line(sym_line_0, layout::strip_heading_badges(line)));
            } else {
                out.push_str(&format_symbol_name(sym, lines));
                out.push_str(" …\n");
            }
            *emitted_up_to = sym_line_0 + 1;
        }
        return;
    }

    // Signature range from layout
    let sig_end = if spec.show_sig {
        layout.sig_end
    } else {
        sym_line_0 // just the first line
    };

    // Doc comment lines (before symbol for most languages)
    let doc_lines_shown = render_line_range(out, lines, layout.doc_start, layout.doc_end, doc_n, true, emitted_up_to);

    // Signature lines (strip trailing badges from markdown heading lines)
    let is_section = sym.kind.is_section_like();
    for (i, line) in lines.iter().enumerate().take(sig_end + 1).skip(sym_line_0) {
        if i < *emitted_up_to {
            continue;
        }
        if is_section && i == sym_line_0 {
            out.push_str(&fmt_line(i, layout::strip_heading_badges(line)));
        } else {
            out.push_str(&fmt_line(i, line));
        }
        *emitted_up_to = i + 1;
    }

    // Python docstrings (after signature)
    // doc_n is a cumulative limit across pre-symbol comments and docstrings,
    // matching the scheduler's flat doc_line_tokens vector.
    let doc_n_remaining = doc_n.saturating_sub(doc_lines_shown);
    render_line_range(out, lines, layout.ds_start, layout.ds_end, doc_n_remaining, true, emitted_up_to);

    // Body lines from layout (section content for markdown, code body otherwise).
    if body_n > 0 {
        render_line_range(out, lines, layout.body_start, layout.body_end, body_n, !layout.has_children, emitted_up_to);
    }
}

/// Render up to `max_lines` from a line range, with an optional truncation marker.
/// Skips lines already emitted (index < `*emitted_up_to`).
/// Returns the number of lines actually rendered.
fn render_line_range(
    out: &mut String,
    lines: &[&str],
    start: usize,
    end: usize,
    max_lines: usize,
    show_truncation: bool,
    emitted_up_to: &mut usize,
) -> usize {
    if max_lines == 0 || start >= end {
        return 0;
    }
    // Skip lines already emitted by a previous symbol
    let effective_start = start.max(*emitted_up_to);
    if effective_start >= end {
        return 0;
    }
    let available = end - effective_start;
    let to_show = available.min(max_lines);
    let render_end = effective_start + to_show;
    for (i, line) in lines.iter().enumerate().take(render_end).skip(effective_start) {
        out.push_str(&fmt_line(i, line));
    }
    *emitted_up_to = render_end;
    if show_truncation && to_show < available {
        out.push_str(&truncation_marker(lines[render_end - 1]));
    }
    to_show
}
