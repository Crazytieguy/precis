//! Prisma schema walker. Emits a single `Toc` batch listing the
//! opening line of every top-level Prisma declaration in a
//! `schema.prisma` file — `model X { … }`, `enum Y { … }`,
//! `datasource db { … }`, `generator client { … }`. The TOC is
//! analogous to the Rust walker's `PubItemNames` or the C walker's
//! `DeclNames`: it tells the agent every entity that exists in the
//! schema without delivering any of their bodies.
//!
//! Scope: Prisma schemas are the canonical data model of any Node /
//! TypeScript app that uses the Prisma ORM. A coding agent landing
//! in a Prisma-shaped repo almost always needs the model/enum
//! catalog before any other backend question, but the schema's
//! `*.prisma` extension is not covered by any other walker, so the
//! file is currently only reachable via its parent dir listing.
//!
//! The TOC is line-scanned, not parsed — Prisma's grammar isn't
//! pulled in as a tree-sitter dependency and a hand-rolled scan for
//! `^model `, `^enum `, `^datasource `, `^generator ` is sufficient
//! (Prisma's syntax requires these keywords to start the line of a
//! top-level declaration; comments and nested blocks are indented).

use std::path::Path;

use crate::batch::{Batch, BatchKey, PrismaKey};
use crate::value::mix_signals;

use super::{
    FileLines, WalkCtx, fs::files_with_extension, path_depth_factor, single_file_lines_content,
};

/// Cap on TOC entries — budget hedge.
const MAX_TOC_ENTRIES: usize = 80;

pub fn expand_in_dir(dir: &Path, ctx: &WalkCtx) -> Vec<Batch<BatchKey>> {
    let mut out = Vec::new();
    for file in files_with_extension(dir, "prisma") {
        let name = file
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default();
        if !name.eq_ignore_ascii_case("schema.prisma") {
            continue;
        }
        let Some(source) = ctx.read_source(&file) else {
            continue;
        };
        let lines: Vec<usize> = toc_lines(&source);
        if lines.is_empty() || lines.len() > MAX_TOC_ENTRIES {
            continue;
        }
        let Some(content) = single_file_lines_content(&file, &source, FileLines::new(lines)) else {
            continue;
        };
        out.push(Batch {
            key: PrismaKey::Toc { file: file.clone() }.into(),
            predecessor: None,
            content,
            value: toc_value(&file, ctx),
        });
    }
    out
}

/// 1-indexed lines of top-level Prisma declarations
/// (`model`/`enum`/`datasource`/`generator`).
fn toc_lines(source: &str) -> Vec<usize> {
    source
        .lines()
        .enumerate()
        .filter_map(|(i, line)| is_toc_line(line).then_some(i + 1))
        .collect()
}

fn is_toc_line(line: &str) -> bool {
    // Top-level keywords are not indented in Prisma; `starts_with` +
    // whitespace separator is sufficient.
    for kw in ["model ", "enum ", "datasource ", "generator "] {
        if line.starts_with(kw) {
            return true;
        }
    }
    false
}

fn toc_value(file: &Path, ctx: &WalkCtx) -> f64 {
    mix_signals(1.0, 0.7, 0.85, path_depth_factor(file, ctx))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prisma_toc_lines_finds_top_level_decls() {
        let src = "\
generator client {
  provider = \"prisma-client-js\"
}

datasource db {
  provider = \"postgresql\"
  url      = env(\"DB_URL\")
}

model User {
  id    Int     @id @default(autoincrement())
  email String  @unique
}

enum Theme {
  dark
  light
}
";
        assert_eq!(toc_lines(src), vec![1, 5, 10, 15]);
    }

    #[test]
    fn prisma_toc_ignores_indented_and_commented_keywords() {
        let src = "\
// model NotADecl
//enum Also
  model NestedNotReal
model Real {
  id Int @id
}
";
        // Only line 4 is a top-level `model `.
        assert_eq!(toc_lines(src), vec![4]);
    }

    #[test]
    fn prisma_toc_rejects_keyword_prefix_without_separator() {
        // `models` and `enumerable` should not match — the trailing
        // space in each keyword pattern guards against this.
        let src = "\
models User {
enumerable X {
modeling.foo
";
        assert!(toc_lines(src).is_empty());
    }
}
