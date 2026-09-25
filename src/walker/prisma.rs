//! Prisma schema walker. Emits a `Toc` batch listing the opening line
//! of every top-level Prisma declaration in a `schema.prisma` file —
//! `model X { … }`, `enum Y { … }`, `datasource db { … }`,
//! `generator client { … }` — plus a per-declaration `Decl` body batch
//! delivering the brace block itself.
//!
//! The TOC is analogous to the Rust walker's `PubItemNames` or the C
//! walker's `DeclNames`: it tells the agent every entity that exists in
//! the schema without delivering any of their bodies. The `Decl` bodies
//! then mirror the code engine's roster → `Decl` ladder — each top-level
//! declaration's body schedules independently, predecessor = the `Toc`,
//! so the agent gets a catalog surface first and the concrete model /
//! enum fields as budget allows.
//!
//! Scope: Prisma schemas are the canonical data model of any Node /
//! TypeScript app that uses the Prisma ORM. A coding agent landing in a
//! Prisma-shaped repo almost always needs the model/enum catalog — and,
//! right after it, the actual field/enum-value lists — before any other
//! backend question, but the schema's `*.prisma` extension is not
//! covered by any other walker, so the file is otherwise only reachable
//! via its parent dir listing.
//!
//! The schema is line-scanned, not parsed — Prisma's grammar isn't
//! pulled in as a tree-sitter dependency. Top-level declarations are
//! detected by a `^model `/`^enum `/`^datasource `/`^generator ` scan
//! (Prisma requires these keywords to start the line of a top-level
//! declaration; comments and nested blocks are indented). The body span
//! of each declaration is found by brace-depth counting from its opener
//! to the matching close brace.

use std::path::Path;

use crate::batch::{Batch, BatchKey, PrismaKey};
use crate::render::Source;
use crate::value::mix_signals;

use super::{FileLines, WalkCtx, fs::files_with_extension, single_file_lines_content};

/// Cap on TOC entries — budget hedge. Also caps the per-decl body
/// batches emitted (a schema with more top-level decls than this is
/// treated as too large to body-expand within budget).
const MAX_TOC_ENTRIES: usize = 80;

/// Field count at which a declaration body earns full base value. Wider
/// models (User/Link/Collection) carry the schema's load-bearing
/// relations and field semantics the NS wants. Scaling value by body
/// size also neutralizes the scheduler's small-batch bias
/// (`value / cost^0.35`), so a thin model doesn't out-rank a wide one
/// purely on cost.
const FULL_VALUE_FIELD_ROWS: f64 = 24.0;

/// Minimum field rows for a `model` body to be worth a per-decl batch.
/// Field-rich models (entities with relations + many columns) answer
/// "what fields / relations does X have" directly; narrow bookkeeping
/// tables (join rows, token rows, single-purpose lookup models) are
/// already covered by the TOC opener and aren't worth the budget they'd
/// displace from orientation surfaces. Enums always emit regardless of
/// size — their value lists are what "what values can field X take"
/// resolves against, and individually they're cheap.
const MIN_MODEL_FIELDS: usize = 18;

/// A `model` body longer than this many rows is split at its row
/// midpoint into a head `Decl` + a `DeclTail`, so the high-value
/// identity / relation fields at the top schedule ahead of the
/// archival-default fields that trail a wide model.
///
/// The split is an affordability carve of one declaration, not a value
/// judgement on its halves: both hold the same model's fields, and half
/// a model's field list is a misleading answer to "what fields does X
/// have". So the tail carries the head's value and orders behind it
/// only through the scheduler's cost term.
const MODEL_SPLIT_MIN_ROWS: usize = 32;

/// Kind of a top-level Prisma declaration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DeclKind {
    Model,
    Enum,
    /// `datasource` / `generator` header block.
    Header,
}

/// One top-level Prisma declaration: its opener line and the inclusive
/// row range of its brace block.
struct Decl {
    /// 1-indexed opener line (`model X {` etc.).
    open_line: usize,
    /// 1-indexed last line of the block (the `}`).
    close_line: usize,
    kind: DeclKind,
}

impl Decl {
    /// Body rows excluding the opener and closer brace lines.
    fn field_rows(&self) -> usize {
        (self.close_line.saturating_sub(self.open_line) + 1).saturating_sub(2)
    }

    /// Whether this declaration is worth its own body batch. Enums and
    /// header blocks (the datasource/generator config the agent needs to
    /// see the DB binding) always qualify; models must clear the
    /// field-richness floor.
    fn warrants_body(&self) -> bool {
        match self.kind {
            DeclKind::Enum | DeclKind::Header => true,
            DeclKind::Model => self.field_rows() >= MIN_MODEL_FIELDS,
        }
    }
}

pub fn expand_in_dir(dir: &Path, ctx: &WalkCtx) -> Vec<Batch<BatchKey>> {
    let mut out = Vec::new();
    for file in files_with_extension(dir, "prisma", ctx) {
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
        let decls = decls(&source);
        if decls.is_empty() || decls.len() > MAX_TOC_ENTRIES {
            continue;
        }
        let toc_line_nums: Vec<usize> = decls.iter().map(|d| d.open_line).collect();
        let Some(content) =
            single_file_lines_content(&file, &source, FileLines::new(toc_line_nums))
        else {
            continue;
        };
        // Multi-file Prisma layouts and generated copies ship
        // `schema.prisma` files holding only `datasource` / `generator`
        // config. Those are build wiring, not the application's data
        // model, and earn no root pin.
        let declares_data_model = decls
            .iter()
            .any(|d| matches!(d.kind, DeclKind::Model | DeclKind::Enum));
        let depth = schema_depth_factor(&file, ctx, declares_data_model);

        let toc_key: BatchKey = PrismaKey::Toc { file: file.clone() }.into();
        out.push(Batch {
            key: toc_key.clone(),
            predecessor: None,
            content,
            value: toc_value(depth),
        });

        for decl in decls.iter().filter(|decl| decl.warrants_body()) {
            push_decl_batches(&mut out, &file, &source, decl, depth, &toc_key);
        }
    }
    out
}

/// Emit the body batch(es) for one declaration. A wide model splits
/// into a head `Decl` + a `DeclTail`; everything else emits a single
/// whole-block `Decl`.
fn push_decl_batches(
    out: &mut Vec<Batch<BatchKey>>,
    file: &Path,
    source: &Source,
    decl: &Decl,
    depth: f64,
    toc_key: &BatchKey,
) {
    let body_rows = decl.close_line.saturating_sub(decl.open_line) + 1;
    let value = decl_value(decl.field_rows(), depth);
    let head_key: BatchKey = PrismaKey::Decl {
        file: file.to_path_buf(),
        start_line: decl.open_line,
    }
    .into();

    let split_at = (decl.kind == DeclKind::Model && body_rows > MODEL_SPLIT_MIN_ROWS)
        .then(|| model_split_line(decl))
        .flatten();

    let head_end = split_at.map_or(decl.close_line, |s| s - 1);
    let Some(head_content) = block_content(file, source, decl.open_line, head_end) else {
        return;
    };
    out.push(Batch {
        key: head_key.clone(),
        predecessor: Some(toc_key.clone()),
        content: head_content,
        value,
    });

    let Some(tail_start) = split_at else {
        return;
    };
    let Some(tail_content) = block_content(file, source, tail_start, decl.close_line) else {
        return;
    };
    out.push(Batch {
        key: PrismaKey::DeclTail {
            file: file.to_path_buf(),
            start_line: decl.open_line,
            tail_start_line: tail_start,
        }
        .into(),
        predecessor: Some(head_key),
        value,
        content: tail_content,
    });
}

/// Split line for a wide model: the midpoint row of the block, kept
/// strictly interior. No field-line awareness — the split can land on a
/// blank/comment/continuation row, which is harmless because
/// `build_file_spans` trims blank rows at span edges, so each half
/// still starts and ends on content. Returns `None` only when the block has no strictly
/// interior row (< 3 rows), which the `MODEL_SPLIT_MIN_ROWS` gate at the
/// sole call site makes unreachable in production.
fn model_split_line(decl: &Decl) -> Option<usize> {
    let mid = decl.open_line + (decl.close_line - decl.open_line) / 2;
    // Keep the split strictly interior to the block.
    (mid > decl.open_line && mid < decl.close_line).then_some(mid)
}

fn block_content(
    file: &Path,
    source: &Source,
    start_line: usize,
    end_line: usize,
) -> Option<crate::content::BatchContent> {
    // `start_line`/`end_line` are already 1-based (from `decls`), so the
    // span rows are the inclusive range directly. Routing through `push_rows`
    // (0-based-in → 1-based-out) would add a second +1 and shift every body
    // down a line, dropping the `model X {` opener. FileLines is 1-based,
    // matching how the Toc consumes `open_line`.
    let rows: Vec<usize> = (start_line..=end_line).collect();
    single_file_lines_content(file, source, FileLines::new(rows))
}

/// Top-level Prisma declarations with their brace-block row ranges,
/// in source order.
fn decls(source: &str) -> Vec<Decl> {
    let lines: Vec<&str> = source.lines().collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        if let Some(kind) = decl_keyword(lines[i]) {
            let open_line = i + 1;
            // Brace-depth scan to the matching close brace. Prisma blocks
            // open with `{` on the keyword line; nested `{ }` (e.g. in
            // `@default`) stay balanced within a line.
            let mut depth = 0i32;
            let mut close_line = open_line;
            for (j, line) in lines.iter().enumerate().skip(i) {
                depth += line.matches('{').count() as i32;
                depth -= line.matches('}').count() as i32;
                if depth <= 0 {
                    close_line = j + 1;
                    break;
                }
            }
            out.push(Decl {
                open_line,
                close_line,
                kind,
            });
            i = close_line; // resume after the block
        } else {
            i += 1;
        }
    }
    out
}

/// `Some(kind)` if `line` opens a top-level Prisma declaration. Top-level
/// keywords are not indented in Prisma; `starts_with` + a whitespace
/// separator is sufficient (guards `models`/`enumerable`).
fn decl_keyword(line: &str) -> Option<DeclKind> {
    if line.starts_with("model ") {
        Some(DeclKind::Model)
    } else if line.starts_with("enum ") {
        Some(DeclKind::Enum)
    } else if line.starts_with("datasource ") || line.starts_with("generator ") {
        Some(DeclKind::Header)
    } else {
        None
    }
}

fn toc_value(depth: f64) -> f64 {
    mix_signals(1.0, 0.7, 0.85, depth)
}

/// Depth factor for a data-model definition file, pinned to root tier
/// when it actually declares a data model.
///
/// A schema that declares the application's persistent entities is
/// application spine: every backend question resolves against it, and
/// the NS ranks its catalog beside the root manifest. Where it sits in
/// the tree records only which workspace package owns the ORM client
/// (`packages/prisma/`, `db/`, `server/prisma/`), so the generic
/// depth damp reads that packaging choice as a centrality signal and
/// pushes the catalog behind hundreds of directory listings. Pinning is
/// the same clamp [`super::file_depth_factor`] already grants an
/// entrypoint, for the same reason.
///
/// The pin is bound to the *data model*, not to configuration in
/// general: deploy / CI / tool config (compose files, workflows,
/// Makefiles, tsconfig) describes how the project is built and run, and
/// its depth genuinely tracks its scope — a workflow under
/// `apps/web/.github/` governs only that app. Nothing outside this
/// walker's `schema.prisma` gate is affected. That binding is enforced
/// by `declares_data_model` rather than by the filename: a
/// `schema.prisma` holding only `datasource` / `generator` blocks — a
/// multi-file layout's config half, a generated client's copy — keeps
/// ordinary path-depth pricing.
fn schema_depth_factor(file: &Path, ctx: &WalkCtx, declares_data_model: bool) -> f64 {
    super::file_depth_factor(file, ctx, declares_data_model)
}

/// Per-decl body value. Below the TOC cat (so the catalog surface
/// schedules first) but above incidental orientation noise (deep dir
/// listings, README body sections), so the concrete model / enum
/// fields land right after the catalog. Scaled by body size so wide
/// load-bearing models out-rank narrow bookkeeping blocks (and the
/// scheduler's small-batch cost bias is neutralized).
fn decl_value(field_rows: usize, depth: f64) -> f64 {
    let size = (field_rows as f64 / FULL_VALUE_FIELD_ROWS)
        .powf(crate::value::DEFAULT_CONCAVITY_EXPONENT)
        .min(1.0);
    mix_signals(0.85, 0.75, 0.7, depth) * size
}

#[cfg(test)]
mod tests {
    use super::*;

    const SCHEMA: &str = "\
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

    #[test]
    fn prisma_decls_finds_top_level_blocks() {
        let d = decls(SCHEMA);
        let openers: Vec<usize> = d.iter().map(|x| x.open_line).collect();
        assert_eq!(openers, vec![1, 5, 10, 15]);
        // Brace-block close lines.
        let closers: Vec<usize> = d.iter().map(|x| x.close_line).collect();
        assert_eq!(closers, vec![3, 8, 13, 18]);
        let kinds: Vec<DeclKind> = d.iter().map(|x| x.kind).collect();
        assert_eq!(
            kinds,
            vec![
                DeclKind::Header,
                DeclKind::Header,
                DeclKind::Model,
                DeclKind::Enum,
            ]
        );
    }

    #[test]
    fn prisma_warrants_body_filters_narrow_models_not_enums() {
        // Enum and header blocks always warrant a body, regardless of size.
        let enum_decl = Decl {
            open_line: 1,
            close_line: 4,
            kind: DeclKind::Enum,
        };
        assert!(enum_decl.warrants_body());
        let header = Decl {
            open_line: 1,
            close_line: 4,
            kind: DeclKind::Header,
        };
        assert!(header.warrants_body());
        // A narrow model (below the field floor) does not.
        let narrow = Decl {
            open_line: 1,
            close_line: 6, // 4 field rows
            kind: DeclKind::Model,
        };
        assert!(!narrow.warrants_body());
        // A wide model does.
        let wide = Decl {
            open_line: 1,
            close_line: 1 + MIN_MODEL_FIELDS + 1, // MIN_MODEL_FIELDS field rows
            kind: DeclKind::Model,
        };
        assert!(wide.warrants_body());
    }

    #[test]
    fn prisma_decls_ignores_indented_and_commented_keywords() {
        let src = "\
// model NotADecl
//enum Also
  model NestedNotReal
model Real {
  id Int @id
}
";
        let d = decls(src);
        assert_eq!(d.len(), 1);
        assert_eq!(d[0].open_line, 4);
        assert_eq!(d[0].close_line, 6);
    }

    #[test]
    fn prisma_decl_keyword_rejects_prefix_without_separator() {
        // `models` and `enumerable` should not match — the trailing
        // space in each keyword pattern guards against this.
        assert_eq!(decl_keyword("models User {"), None);
        assert_eq!(decl_keyword("enumerable X {"), None);
        assert_eq!(decl_keyword("modeling.foo"), None);
        assert_eq!(decl_keyword("model User {"), Some(DeclKind::Model));
        assert_eq!(decl_keyword("enum Theme {"), Some(DeclKind::Enum));
        assert_eq!(decl_keyword("datasource db {"), Some(DeclKind::Header));
        assert_eq!(decl_keyword("generator client {"), Some(DeclKind::Header));
    }

    /// The root pin belongs to the application's data model. A nested
    /// `schema.prisma` carrying only `datasource` / `generator` config —
    /// a multi-file layout's config half, a generated client's copy — is
    /// build wiring and keeps ordinary path-depth pricing.
    #[test]
    fn prisma_root_pin_requires_a_model_or_enum_declaration() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let nested = root.join("packages/db/prisma");
        std::fs::create_dir_all(&nested).unwrap();
        std::fs::write(root.join("README.md"), "# demo\n").unwrap();

        let config_only = nested.join("schema.prisma");
        std::fs::write(
            &config_only,
            "generator client {\n  provider = \"prisma-client-js\"\n}\n\ndatasource db {\n  provider = \"postgresql\"\n  url      = env(\"DB_URL\")\n}\n",
        )
        .unwrap();

        let ctx = WalkCtx::new(root.to_path_buf());
        let unpinned = schema_depth_factor(&config_only, &ctx, false);
        let pinned = schema_depth_factor(&config_only, &ctx, true);
        assert!(
            unpinned < pinned,
            "a nested config-only schema must not reach root tier: {unpinned} vs {pinned}"
        );

        let batches = expand_in_dir(&nested, &ctx);
        let toc = batches
            .iter()
            .find(|b| matches!(b.key, BatchKey::Prisma(PrismaKey::Toc { .. })))
            .expect("toc batch");
        assert!((toc.value - toc_value(unpinned)).abs() < 1e-9);

        // Adding one model turns the pin back on at the same path.
        std::fs::write(
            &config_only,
            "generator client {\n  provider = \"prisma-client-js\"\n}\n\nmodel User {\n  id    Int    @id\n  email String @unique\n}\n",
        )
        .unwrap();
        let ctx = WalkCtx::new(root.to_path_buf());
        let toc = expand_in_dir(&nested, &ctx)
            .into_iter()
            .find(|b| matches!(b.key, BatchKey::Prisma(PrismaKey::Toc { .. })))
            .expect("toc batch");
        assert!((toc.value - toc_value(pinned)).abs() < 1e-9);
    }

    #[test]
    fn prisma_wide_model_splits_strictly_interior() {
        // 40-row model: open at 1, close at 40.
        let decl = Decl {
            open_line: 1,
            close_line: 40,
            kind: DeclKind::Model,
        };
        let split = model_split_line(&decl).expect("wide model splits");
        assert!(split > 1 && split < 40);
    }
}
