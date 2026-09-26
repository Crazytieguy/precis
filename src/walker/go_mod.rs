//! `go.mod` / `go.work` batches: the module's identity directives, then
//! the file's directives without comments or indirect requires.

use std::path::Path;

use crate::batch::{Batch, BatchKey, GoModKey};
use crate::fs_util::list_dir;
use crate::value::{dependency_roster_value, manifest_identity_value};

use super::{WalkCtx, path_depth_factor, single_file_lines_content};

pub fn expand_in_dir(dir: &Path, ctx: &WalkCtx) -> Vec<Batch> {
    let mut out = Vec::new();
    for (name, kind) in list_dir(dir, ctx.dir_filter()).iter() {
        if !matches!(kind, crate::fs_util::EntryKind::File) {
            continue;
        }
        if !matches!(name.as_str(), "go.mod" | "go.work") {
            continue;
        }
        let path = dir.join(name);
        let depth = path_depth_factor(&path, ctx);
        let Some(source) = ctx.read_source(&path) else {
            continue;
        };
        let (identity_rows, kept_rows) = scan(&source);
        let identity = single_file_lines_content(&path, &source, identity_rows).map(|content| {
            let key: BatchKey = GoModKey::Identity { file: path.clone() }.into();
            out.push(Batch {
                key: key.clone(),
                predecessor: None,
                content,
                value: manifest_identity_value(1.0, depth),
            });
            key
        });
        if let Some(content) = single_file_lines_content(&path, &source, kept_rows) {
            out.push(Batch {
                key: GoModKey::File { file: path.clone() }.into(),
                predecessor: identity,
                content,
                value: dependency_roster_value(depth),
            });
        }
    }
    out
}

/// `(identity rows, kept rows)`: the top-level `module` / `go` /
/// `toolchain` directives, and every directive and block entry except
/// comments and indirect requires (a block keeps its parentheses only
/// when it keeps an entry).
fn scan(source: &str) -> (Vec<usize>, Vec<usize>) {
    let mut identity = Vec::new();
    let mut kept = Vec::new();
    let mut open_block: Option<(&str, usize)> = None;
    let mut block_rows = Vec::new();
    for (index, line) in source.lines().enumerate() {
        let row = index + 1;
        let trimmed = line.trim();
        let code = strip_comment(trimmed).trim_end();
        if let Some((keyword, open_row)) = open_block {
            if code == ")" {
                if !block_rows.is_empty() {
                    kept.push(open_row);
                    kept.append(&mut block_rows);
                    kept.push(row);
                }
                open_block = None;
            } else if !code.is_empty() && is_kept(keyword, trimmed) {
                block_rows.push(row);
            }
            continue;
        }
        if let Some(keyword) = block_start(code) {
            open_block = Some((keyword, row));
            continue;
        }
        let first = code.split_whitespace().next().unwrap_or("");
        if matches!(first, "module" | "go" | "toolchain") {
            identity.push(row);
        }
        if DIRECTIVES.contains(&first) && is_kept(first, trimmed) {
            kept.push(row);
        }
    }
    if let Some((_, open_row)) = open_block
        && !block_rows.is_empty()
    {
        kept.push(open_row);
        kept.append(&mut block_rows);
    }
    (identity, kept)
}

/// `line` up to its `//` comment, if any. go.mod strings are `"…"` with
/// backslash escapes or backquoted raw strings; a `//` inside one is text.
fn strip_comment(line: &str) -> &str {
    let mut quote = None;
    let mut escaped = false;
    let mut previous_slash = false;
    for (index, c) in line.char_indices() {
        match quote {
            Some('"') if escaped => escaped = false,
            Some('"') if c == '\\' => escaped = true,
            Some(open) if c == open => quote = None,
            Some(_) => {}
            None if c == '/' && previous_slash => return &line[..index - 1],
            None if c == '"' || c == '`' => quote = Some(c),
            None => {}
        }
        previous_slash = quote.is_none() && c == '/';
    }
    line
}

/// The directives of go.mod and go.work.
#[rustfmt::skip]
const DIRECTIVES: &[&str] = &[
    "module", "go", "toolchain", "godebug", "require", "replace", "exclude", "retract", "use",
    "tool", "ignore",
];

fn block_start(code: &str) -> Option<&str> {
    let (first, rest) = code.split_once(char::is_whitespace)?;
    (rest.trim() == "(" && DIRECTIVES.contains(&first)).then_some(first)
}

/// Every directive line but an indirect `require`.
fn is_kept(directive: &str, trimmed: &str) -> bool {
    directive != "require" || !trimmed.contains("// indirect")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn go_mod_keeps_direct_requires_replace_and_retract() {
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
        assert_eq!(scan(src).1, vec![1, 3, 5, 6, 8, 10, 12]);
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
        assert_eq!(scan(src).0, vec![1, 3, 5]);
    }

    /// A trailing comment ending in `(` does not open a block.
    #[test]
    fn go_mod_identity_ignores_a_paren_in_a_trailing_comment() {
        let src = "module example.com/foo\n\nrequire example.com/x v1.0.0 // pinned (\n\ngo 1.22\n";
        assert_eq!(scan(src).0, vec![1, 5]);
    }

    /// A block whose every entry is an indirect require drops its
    /// parentheses too.
    #[test]
    fn go_mod_drops_a_block_of_only_indirect_requires() {
        let mut src = String::from("module example.com/foo\n\nrequire (\n");
        for i in 0..65 {
            src.push_str(&format!("\tgithub.com/indirect/{i} v0.0.1 // indirect\n"));
        }
        src.push_str(")\n\nexclude github.com/bad/module v1.0.0\n");
        let exclude_row = src.lines().count();
        assert_eq!(scan(&src).1, vec![1, exclude_row]);
    }

    #[test]
    fn go_mod_recognizes_blocks_with_trailing_comments() {
        let src = "\
module example.com/foo

require ( // direct dependencies
\tgithub.com/x/y v1.0.0
\tgithub.com/x/z v0.5.0 // indirect
) // end
";
        assert_eq!(scan(src).1, vec![1, 3, 4, 6]);
    }

    #[test]
    fn go_mod_keeps_tool_godebug_and_ignore_directives() {
        let src = "\
module example.com/foo

godebug default=go1.21

tool (
\tgithub.com/gogo/protobuf/protoc-gen-gogo
\texample.com/foo/internal/gen
)

tool golang.org/x/tools/cmd/stringer

ignore ./node_modules
";
        assert_eq!(scan(src).1, vec![1, 3, 5, 6, 7, 8, 10, 12]);
    }

    #[test]
    fn go_mod_keeps_go_work_members_under_a_commented_use_block() {
        let src = "go 1.22\n\nuse ( // workspace packages\n\t./a\n\t\"./b//c\"\n)\n";
        assert_eq!(scan(src).1, vec![1, 3, 4, 5, 6]);
    }
}
