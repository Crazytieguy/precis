//! `go.mod` / `go.work` batches: the module's identity directives, then
//! the file's directives without comments or indirect requires.

use std::path::Path;

use crate::batch::{Batch, BatchKey, GoModKey};
use crate::value::mix_signals;

use super::{WalkCtx, fs::list_dir, path_depth_factor, single_file_lines_content};

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
                value: identity_value(&path, ctx),
            });
            key
        });
        if let Some(content) = single_file_lines_content(&path, &source, kept_rows) {
            out.push(Batch {
                key: GoModKey::File { file: path.clone() }.into(),
                predecessor: identity,
                content,
                value: file_value(&path, ctx),
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
        if let Some((keyword, open_row)) = open_block {
            if trimmed == ")" {
                if !block_rows.is_empty() {
                    kept.push(open_row);
                    kept.append(&mut block_rows);
                    kept.push(row);
                }
                open_block = None;
            } else if keep_block_entry(keyword, trimmed) {
                block_rows.push(row);
            }
            continue;
        }
        if let Some(keyword) = block_start(trimmed) {
            open_block = Some((keyword, row));
            continue;
        }
        let first = trimmed.split_whitespace().next().unwrap_or("");
        if matches!(first, "module" | "go" | "toolchain") {
            identity.push(row);
        }
        if keep_directive_line(first, trimmed) {
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

fn block_start(trimmed: &str) -> Option<&str> {
    let (first, rest) = trimmed.split_once(char::is_whitespace)?;
    (rest.trim() == "(" && matches!(first, "require" | "replace" | "exclude" | "retract" | "use"))
        .then_some(first)
}

fn keep_directive_line(first: &str, trimmed: &str) -> bool {
    matches!(
        first,
        "module" | "go" | "toolchain" | "replace" | "exclude" | "retract" | "use"
    ) || (first == "require" && !trimmed.contains("// indirect"))
}

fn keep_block_entry(block: &str, trimmed: &str) -> bool {
    if trimmed.is_empty() || trimmed.starts_with("//") {
        return false;
    }
    block != "require" || !trimmed.contains("// indirect")
}

fn file_value(file: &Path, ctx: &WalkCtx) -> f64 {
    mix_signals(0.80, 0.60, 0.5, path_depth_factor(file, ctx))
}

fn identity_value(file: &Path, ctx: &WalkCtx) -> f64 {
    mix_signals(0.90, 0.75, 0.35, path_depth_factor(file, ctx))
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
}
