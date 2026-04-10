/// Check if a markdown line is leading "noise" that should be skipped at
/// the start of section bodies. Matches:
/// - Blank or whitespace-only lines
/// - Markdown images: `![alt](url)` (badges/shields)
/// - Linked images: `[![alt](url)](url)` (clickable badges)
/// - Link reference definitions: `[label]: http...`
/// - HTML tags: `<div>`, `<p align=...>`, `<img .../>`, `</div>`, etc.
/// - HTML comments: `<!-- ... -->`
/// - Horizontal rules: `---`, `***`, `___`, `* * *`, etc.
///
/// Only used to skip contiguous noise at the start of a section body,
/// so mid-section images and links are still rendered normally.
pub(super) fn is_markdown_leading_noise(line: &str) -> bool {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return true;
    }
    // Markdown badge images: [![alt](url)](url) (linked images are always badges)
    if trimmed.starts_with("[![") && trimmed.ends_with(')') {
        return true;
    }
    // Standalone images: ![alt](url) — only skip if alt text is short (badges).
    // Long alt text indicates a screenshot or diagram that has real content value.
    if trimmed.starts_with("![") && trimmed.ends_with(')')
        && let Some(alt_end) = trimmed.find("](")
        && trimmed[2..alt_end].len() <= 30
    {
        return true;
    }
    // Link reference definitions: [label]: URL
    if trimmed.starts_with('[')
        && let Some(pos) = trimmed.find("]: ")
    {
        let after = trimmed[pos + 3..].trim();
        if after.starts_with("http") || after.starts_with('/') || after.starts_with('#') {
            return true;
        }
    }
    // Table of contents links: `- [Title](#anchor)` or `* [Title](#anchor)`
    // These are navigational, not content — they duplicate the heading structure.
    if is_toc_link(trimmed) {
        return true;
    }
    // Block-level HTML tags and comments used for layout/badges, not content.
    // Only matches tags that are clearly structural (div, p, img, br, details,
    // table, etc.) — NOT inline tags like <em>, <strong>, <b>, <a> which wrap content.
    if let Some(rest) = trimmed.strip_prefix('<') {
        let tag_start = rest.strip_prefix('/').unwrap_or(rest);
        let tag_end = tag_start.find(|c: char| !c.is_ascii_alphabetic()).unwrap_or(tag_start.len());
        let tag = &tag_start[..tag_end];
        if rest.starts_with('!')  // HTML comments: <!-- ... -->
            || tag.eq_ignore_ascii_case("div") || tag.eq_ignore_ascii_case("p")
            || tag.eq_ignore_ascii_case("img") || tag.eq_ignore_ascii_case("br")
            || tag.eq_ignore_ascii_case("hr") || tag.eq_ignore_ascii_case("table")
            || tag.eq_ignore_ascii_case("details") || tag.eq_ignore_ascii_case("summary")
            || tag.eq_ignore_ascii_case("picture") || tag.eq_ignore_ascii_case("figure")
            || tag.eq_ignore_ascii_case("center")
        {
            return true;
        }
    }
    // Horizontal rules: 3+ of the same character (-, *, _) with optional spaces
    if is_horizontal_rule(trimmed) {
        return true;
    }
    false
}

/// Check if a line is a table-of-contents link: `- [Title](#anchor)` etc.
pub(super) fn is_toc_link(trimmed: &str) -> bool {
    if trimmed.starts_with("- ") || trimmed.starts_with("* ") || trimmed.starts_with("+ ") {
        let rest = trimmed[2..].trim_start();
        return rest.starts_with('[') && rest.contains("](#") && rest.ends_with(')');
    }
    false
}

/// Trim trailing blank lines from a line range, returning the new exclusive end.
pub(super) fn trim_trailing_blank_lines(lines: &[&str], start: usize, end: usize) -> usize {
    let mut e = end;
    while e > start && lines[e - 1].trim().is_empty() {
        e -= 1;
    }
    e
}

/// Check if a line is trailing noise in config format section bodies.
/// Matches blank lines and lines that are purely closing delimiters
/// (`}`, `]`, with optional trailing commas and whitespace).
pub(super) fn is_config_trailing_noise(line: &str) -> bool {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return true;
    }
    // Closing delimiters with optional trailing comma: }, ], },, ],
    matches!(trimmed, "}" | "}," | "]" | "],")
}

/// Check if a trimmed line is a markdown horizontal rule (thematic break).
/// Matches 3+ of the same character (`-`, `*`, or `_`), optionally
/// separated by spaces.
fn is_horizontal_rule(trimmed: &str) -> bool {
    if trimmed.len() < 3 {
        return false;
    }
    let mut rule_char = None;
    let mut count = 0;
    for b in trimmed.bytes() {
        match b {
            b'-' | b'*' | b'_' => {
                if let Some(rc) = rule_char {
                    if b != rc {
                        return false;
                    }
                } else {
                    rule_char = Some(b);
                }
                count += 1;
            }
            b' ' => {}
            _ => return false,
        }
    }
    count >= 3
}

/// Strip trailing badge/image markdown from a markdown heading line.
///
/// Many README files include CI/coverage/version badges inline in the top-level
/// heading: `# project [![build](url)](url) [![version](url)](url)`.
/// These badge URLs waste token budget while adding no useful information for
/// codebase understanding.
///
/// Returns the prefix of the line before the first trailing ` [![` pattern.
/// Only matches the linked-image pattern (`[![`) which is specifically used
/// for badges; plain `![` images in headings are left intact.
pub(crate) fn strip_heading_badges(line: &str) -> &str {
    // Look for ` [![` — linked image (badge) preceded by a space.
    // Only strip if there's meaningful heading text before the badge.
    if let Some(pos) = line.find(" [![") {
        // Ensure there's at least one non-whitespace char of heading text
        // before the badge (skip the `# ` prefix).
        let before = line[..pos].trim();
        if !before.is_empty() && before != "#" {
            return line[..pos].trim_end();
        }
    }
    line
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leading_noise_badge_vs_screenshot() {
        // Short alt text = badge (noise)
        assert!(is_markdown_leading_noise("![build](https://img.shields.io/badge.svg)"));
        assert!(is_markdown_leading_noise("![npm](https://badge.fury.io/js/pkg.svg)"));
        // Linked badge
        assert!(is_markdown_leading_noise("[![CI](url)](link)"));
        // Long alt text = screenshot (not noise)
        assert!(!is_markdown_leading_noise("![Screenshot showing the dashboard with live metrics](screenshot.png)"));
        // TOC links
        assert!(is_markdown_leading_noise("- [Installation](#installation)"));
        assert!(is_markdown_leading_noise("-   [Usage](#usage)"));
    }

    #[test]
    fn strip_heading_badges_removes_trailing_badges() {
        // Linked images (badges) after heading text
        assert_eq!(
            strip_heading_badges("# color [![build](url)](link)"),
            "# color",
        );
        assert_eq!(
            strip_heading_badges("# Structs [![GoDoc](url)](link) [![Build](url)](link)"),
            "# Structs",
        );
        // Name-only text (without # prefix)
        assert_eq!(
            strip_heading_badges("color [![build](url)](link)"),
            "color",
        );
        // No badges — unchanged
        assert_eq!(strip_heading_badges("# Introduction"), "# Introduction");
        assert_eq!(strip_heading_badges("## API Reference"), "## API Reference");
        // Badge without preceding text — unchanged (just `#` prefix)
        assert_eq!(
            strip_heading_badges("# [![badge](url)](link)"),
            "# [![badge](url)](link)",
        );
        // Plain image (not linked) — not stripped (only [![ is targeted)
        assert_eq!(
            strip_heading_badges("# Project ![icon](url)"),
            "# Project ![icon](url)",
        );
    }

}
