//! Root SQL contract walker.
//!
//! SQL trees are dominated by migrations and generated schema history, so
//! extension alone is not a useful signal. This walker admits only `.sql`
//! files at the walk root or nested SQL files whose exact relative path is
//! cited by a root README/build file. It then emits semicolon-aligned DDL and
//! a bounded set of representative `SELECT ... WHERE` contracts.

use std::collections::HashSet;
use std::ops::Range;
use std::path::{Path, PathBuf};

use crate::batch::{Batch, BatchKey, SqlKey};
use crate::render::Source;
use crate::value::{DEFAULT_CONCAVITY_EXPONENT, conserved_catalog_chunk_factors, mix_signals};

use super::{
    FileLines, WalkCtx, budget_chunk_ranges, fs, gap_ellipses, gated_read_source,
    path_depth_factor, single_file_lines_content,
};

const SQL_BYTE_GATE: usize = 1024 * 1024;
const SQL_SCHEMA_CHUNK_TARGET_TOKENS: usize = 180;
const SQL_SCHEMA_CHUNK_MIN_TOKENS: usize = 100;
const SQL_QUERY_CHUNK_TARGET_TOKENS: usize = 120;
const SQL_QUERY_CHUNK_MIN_TOKENS: usize = 70;
const REPRESENTATIVE_QUERY_LIMIT: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ContractKind {
    Schema,
    Query,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Contract {
    lines: Range<usize>,
    kind: ContractKind,
    richness: usize,
}

pub fn expand_in_dir(dir: &Path, ctx: &WalkCtx) -> Vec<Batch<BatchKey>> {
    let mut out = Vec::new();
    for file in fs::files_with_extension(dir, "sql", ctx) {
        if dir != ctx.root() && !ctx.is_sql_cited(&file) {
            continue;
        }
        let Some(source) = gated_read_source(&file, ctx, SQL_BYTE_GATE) else {
            continue;
        };
        let contracts = collect_contracts(&source);
        if contracts.is_empty() {
            continue;
        }

        let mut schema = merge_overlapping_contracts(
            contracts
                .iter()
                .filter(|contract| contract.kind == ContractKind::Schema)
                .cloned()
                .collect(),
        );
        let mut queries = merge_overlapping_contracts(
            contracts
                .into_iter()
                .filter(|contract| contract.kind == ContractKind::Query)
                .collect(),
        );
        // Rendering is line-granular. If DDL and a query share a physical
        // line, the schema batch already carries the query text; emitting a
        // second owner for that line would violate the overlap invariant.
        queries.retain(|query| {
            !schema
                .iter()
                .any(|ddl| ranges_overlap(&query.lines, &ddl.lines))
        });
        queries.sort_unstable_by(|left, right| {
            right
                .richness
                .cmp(&left.richness)
                .then_with(|| left.lines.start.cmp(&right.lines.start))
        });
        queries.truncate(REPRESENTATIVE_QUERY_LIMIT);
        schema.sort_unstable_by_key(|contract| contract.lines.start);

        // Gap markers are assigned once across both surfaces. This keeps a
        // schema chunk's ellipsis from landing on a query line (or vice
        // versa), which would create non-ancestor overlap at render time.
        let mut all_lines: Vec<usize> = schema
            .iter()
            .chain(&queries)
            .flat_map(|contract| contract.lines.clone())
            .collect();
        all_lines.sort_unstable();
        all_lines.dedup();
        let line_count = source.lines().count();
        let ellipses = gap_ellipses(&all_lines, line_count);

        out.extend(contract_chunks(
            &file,
            &source,
            &schema,
            &all_lines,
            &ellipses,
            ContractKind::Schema,
            ctx,
        ));
        out.extend(contract_chunks(
            &file,
            &source,
            &queries,
            &all_lines,
            &ellipses,
            ContractKind::Query,
            ctx,
        ));
    }
    out
}

fn contract_chunks(
    file: &Path,
    source: &Source,
    contracts: &[Contract],
    all_lines: &[usize],
    ellipses: &[usize],
    kind: ContractKind,
    ctx: &WalkCtx,
) -> Vec<Batch<BatchKey>> {
    if contracts.is_empty() {
        return Vec::new();
    }
    let global_first = all_lines.first().copied();
    let lines_for = |range: Range<usize>| {
        let mut selected: Vec<usize> = contracts[range]
            .iter()
            .flat_map(|contract| contract.lines.clone())
            .collect();
        selected.sort_unstable();
        selected.dedup();
        let owned_ellipses = ellipses
            .iter()
            .copied()
            .filter(|ellipsis| {
                (*ellipsis == 1 && selected.first().copied() == global_first)
                    || ellipsis
                        .checked_sub(1)
                        .is_some_and(|previous| selected.binary_search(&previous).is_ok())
            })
            .collect();
        FileLines::new(selected).with_ellipses(owned_ellipses)
    };
    let cost = |range: Range<usize>| {
        single_file_lines_content(file, source, lines_for(range))
            .map(|content| ctx.marginal_tokens(&content))
            .unwrap_or(0)
    };
    let (target, min_tail) = match kind {
        ContractKind::Schema => (SQL_SCHEMA_CHUNK_TARGET_TOKENS, SQL_SCHEMA_CHUNK_MIN_TOKENS),
        ContractKind::Query => (SQL_QUERY_CHUNK_TARGET_TOKENS, SQL_QUERY_CHUNK_MIN_TOKENS),
    };
    let contents: Vec<_> = budget_chunk_ranges(
        contracts.len(),
        cost,
        target,
        min_tail,
        |_| true,
        |range| cost(range) <= target + 60,
    )
    .into_iter()
    .filter_map(|range| single_file_lines_content(file, source, lines_for(range)))
    .collect();
    let costs: Vec<usize> = contents
        .iter()
        .map(|content| ctx.marginal_tokens(content))
        .collect();
    let factors = conserved_catalog_chunk_factors(&costs, DEFAULT_CONCAVITY_EXPONENT);
    let depth = path_depth_factor(file, ctx);
    let aggregate_value = match kind {
        ContractKind::Schema => mix_signals(0.65, 0.60, 0.65, depth),
        ContractKind::Query => mix_signals(0.50, 0.60, 0.55, depth),
    };
    contents
        .into_iter()
        .zip(factors)
        .enumerate()
        .map(|(chunk_index, (content, factor))| {
            let key = match kind {
                ContractKind::Schema => SqlKey::SchemaChunk {
                    file: file.to_path_buf(),
                    chunk_index,
                },
                ContractKind::Query => SqlKey::QueryChunk {
                    file: file.to_path_buf(),
                    chunk_index,
                },
            };
            Batch {
                key: key.into(),
                predecessor: None,
                content,
                value: aggregate_value * factor,
            }
        })
        .collect()
}

/// Exact nested SQL paths mentioned by root README/build files. Matching the
/// full relative path (not a basename such as `migration.sql`) is what keeps
/// migration forests excluded.
pub(crate) fn collect_root_cited_sql_paths(root: &Path, ctx: &WalkCtx) -> HashSet<PathBuf> {
    let Ok(entries) = std::fs::read_dir(root) else {
        return HashSet::new();
    };
    let mut citation_text = String::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        let lower = name.to_ascii_lowercase();
        let is_readme = matches!(lower.as_str(), "readme.md" | "readme.rst" | "readme.txt");
        let is_build = matches!(
            super::plaintext::classify_plaintext(name),
            Some(
                super::plaintext::Class::BuildEntrypoint
                    | super::plaintext::Class::Dockerfile
                    | super::plaintext::Class::BuildScript
            )
        );
        if (is_readme || is_build)
            && let Ok(text) = std::fs::read_to_string(path)
        {
            citation_text.push_str(&text.replace('\\', "/"));
            citation_text.push('\n');
        }
    }
    if citation_text.is_empty() {
        return HashSet::new();
    }

    fs::files_with_extension_recursive(root, "sql", ctx)
        .into_iter()
        .filter(|file| file.parent().is_some_and(|parent| parent != root))
        .filter(|file| {
            let relative = file.strip_prefix(root).unwrap_or(file);
            citation_text.contains(&relative.to_string_lossy().replace('\\', "/"))
        })
        .filter_map(|file| file.canonicalize().ok())
        .collect()
}

fn collect_contracts(source: &str) -> Vec<Contract> {
    statement_ranges(source)
        .into_iter()
        .filter_map(|(bytes, segment_start_line)| {
            classify_statement(&source[bytes], segment_start_line)
        })
        .collect()
}

fn merge_overlapping_contracts(mut contracts: Vec<Contract>) -> Vec<Contract> {
    contracts.sort_unstable_by_key(|contract| contract.lines.start);
    let mut merged: Vec<Contract> = Vec::new();
    for contract in contracts {
        if let Some(previous) = merged.last_mut()
            && previous.kind == contract.kind
            && ranges_overlap(&previous.lines, &contract.lines)
        {
            previous.lines.end = previous.lines.end.max(contract.lines.end);
            previous.richness = previous.richness.max(contract.richness);
        } else {
            merged.push(contract);
        }
    }
    merged
}

fn ranges_overlap(left: &Range<usize>, right: &Range<usize>) -> bool {
    left.start < right.end && right.start < left.end
}

fn classify_statement(raw: &str, segment_start_line: usize) -> Option<Contract> {
    let (sql, leading_lines) = sql_suffix(raw)?;
    let tokens = sql_tokens(sql);
    if tokens.is_empty() {
        return None;
    }
    let kind = if is_schema_tokens(&tokens) {
        ContractKind::Schema
    } else if is_query_tokens(&tokens) {
        ContractKind::Query
    } else {
        return None;
    };
    let start = segment_start_line + leading_lines;
    let end = segment_start_line + raw.lines().count().max(1) - 1;
    let richness = if kind == ContractKind::Query {
        query_richness(&tokens, end.saturating_sub(start) + 1)
    } else {
        0
    };
    Some(Contract {
        lines: start..end + 1,
        kind,
        richness,
    })
}

/// Drop leading blank/comment/sqlite-shell lines from one semicolon-delimited
/// segment and return the first SQL-looking suffix plus its line offset.
fn sql_suffix(raw: &str) -> Option<(&str, usize)> {
    let mut byte_offset = 0;
    let mut in_block_comment = false;
    for (line_index, line) in raw.split_inclusive('\n').enumerate() {
        let mut remaining = line.trim_start();
        loop {
            if in_block_comment {
                let Some(end) = remaining.find("*/") else {
                    remaining = "";
                    break;
                };
                remaining = remaining[end + 2..].trim_start();
                in_block_comment = false;
                continue;
            }
            if remaining.starts_with("/*") {
                in_block_comment = true;
                remaining = &remaining[2..];
                continue;
            }
            break;
        }
        let sql_start = remaining.split_whitespace().next().is_some_and(|word| {
            matches!(
                word.to_ascii_lowercase().as_str(),
                "create" | "alter" | "drop" | "truncate" | "select" | "with"
            )
        });
        if sql_start {
            let indent = line.len() - line.trim_start().len();
            let comment_trim = line.trim_start().len() - remaining.len();
            return Some((&raw[byte_offset + indent + comment_trim..], line_index));
        }
        byte_offset += line.len();
    }
    None
}

fn is_schema_tokens(tokens: &[String]) -> bool {
    let words: Vec<&str> = tokens.iter().map(String::as_str).collect();
    match words.as_slice() {
        ["create", rest @ ..] => {
            let rest = match rest {
                ["or", "replace", tail @ ..] => tail,
                ["temp", tail @ ..] | ["temporary", tail @ ..] => tail,
                _ => rest,
            };
            matches!(
                rest,
                ["table", ..]
                    | ["virtual", "table", ..]
                    | ["view", ..]
                    | ["index", ..]
                    | ["unique", "index", ..]
                    | ["trigger", ..]
                    | ["schema", ..]
                    | ["database", ..]
            )
        }
        ["alter", "table" | "view" | "schema" | "database", ..]
        | [
            "drop",
            "table" | "view" | "index" | "trigger" | "schema" | "database",
            ..,
        ]
        | ["truncate", "table", ..] => true,
        _ => false,
    }
}

fn is_query_tokens(tokens: &[String]) -> bool {
    matches!(tokens.first().map(String::as_str), Some("select" | "with"))
        && tokens.iter().any(|token| token == "select")
        && tokens.iter().any(|token| token == "where")
}

fn query_richness(tokens: &[String], line_count: usize) -> usize {
    let count = |needle: &str| {
        tokens
            .iter()
            .filter(|token| token.as_str() == needle)
            .count()
    };
    8 * count("where")
        + 3 * (count("and") + count("or"))
        + 2 * (count("join") + count("match") + count("between") + count("having"))
        + line_count.min(20)
}

/// Lowercase SQL identifiers while replacing comments and quoted bodies with
/// separators, so keywords inside strings cannot classify a statement.
fn sql_tokens(sql: &str) -> Vec<String> {
    let bytes = sql.as_bytes();
    let mut out = String::with_capacity(sql.len());
    let mut index = 0;
    let mut state = ScanState::Normal;
    while index < bytes.len() {
        match &state {
            ScanState::Normal => match bytes[index] {
                b'\'' => {
                    state = ScanState::Single;
                    out.push(' ');
                }
                b'"' => {
                    state = ScanState::Double;
                    out.push(' ');
                }
                b'`' => {
                    state = ScanState::Backtick;
                    out.push(' ');
                }
                b'[' => {
                    state = ScanState::Bracket;
                    out.push(' ');
                }
                b'-' if bytes.get(index + 1) == Some(&b'-') => {
                    state = ScanState::LineComment;
                    index += 1;
                    out.push(' ');
                }
                b'/' if bytes.get(index + 1) == Some(&b'*') => {
                    state = ScanState::BlockComment;
                    index += 1;
                    out.push(' ');
                }
                b'$' => {
                    if let Some(tag) = dollar_tag(&bytes[index..]) {
                        index += tag.len() - 1;
                        state = ScanState::Dollar(tag);
                    }
                    out.push(' ');
                }
                byte if byte.is_ascii_alphanumeric() || byte == b'_' => {
                    out.push((byte as char).to_ascii_lowercase())
                }
                _ => out.push(' '),
            },
            ScanState::Single => {
                if bytes[index] == b'\'' {
                    if bytes.get(index + 1) == Some(&b'\'') {
                        index += 1;
                    } else {
                        state = ScanState::Normal;
                    }
                }
                out.push(' ');
            }
            ScanState::Double => {
                if bytes[index] == b'"' {
                    if bytes.get(index + 1) == Some(&b'"') {
                        index += 1;
                    } else {
                        state = ScanState::Normal;
                    }
                }
                out.push(' ');
            }
            ScanState::Backtick => {
                if bytes[index] == b'`' {
                    state = ScanState::Normal;
                }
                out.push(' ');
            }
            ScanState::Bracket => {
                if bytes[index] == b']' {
                    state = ScanState::Normal;
                }
                out.push(' ');
            }
            ScanState::LineComment => {
                if bytes[index] == b'\n' {
                    state = ScanState::Normal;
                }
                out.push(' ');
            }
            ScanState::BlockComment => {
                if bytes[index] == b'*' && bytes.get(index + 1) == Some(&b'/') {
                    state = ScanState::Normal;
                    index += 1;
                }
                out.push(' ');
            }
            ScanState::Dollar(tag) => {
                if bytes[index..].starts_with(tag) {
                    index += tag.len() - 1;
                    state = ScanState::Normal;
                }
                out.push(' ');
            }
        }
        index += 1;
    }
    out.split_whitespace().map(str::to_owned).collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ScanState {
    Normal,
    Single,
    Double,
    Backtick,
    Bracket,
    LineComment,
    BlockComment,
    Dollar(Vec<u8>),
}

/// Byte ranges of semicolon-terminated statements plus their one-based
/// segment start line. Semicolons inside comments, quoted identifiers,
/// strings, and PostgreSQL dollar-quoted bodies are ignored.
fn statement_ranges(source: &str) -> Vec<(Range<usize>, usize)> {
    let bytes = source.as_bytes();
    let mut out = Vec::new();
    let mut state = ScanState::Normal;
    let mut index = 0;
    let mut start = 0;
    let mut start_line = 1;
    let mut line = 1;
    while index < bytes.len() {
        match &mut state {
            ScanState::Normal => match bytes[index] {
                b'\'' => state = ScanState::Single,
                b'"' => state = ScanState::Double,
                b'`' => state = ScanState::Backtick,
                b'[' => state = ScanState::Bracket,
                b'-' if bytes.get(index + 1) == Some(&b'-') => {
                    state = ScanState::LineComment;
                    index += 1;
                }
                b'/' if bytes.get(index + 1) == Some(&b'*') => {
                    state = ScanState::BlockComment;
                    index += 1;
                }
                b'$' => {
                    if let Some(tag) = dollar_tag(&bytes[index..]) {
                        index += tag.len() - 1;
                        state = ScanState::Dollar(tag);
                    }
                }
                b';' => {
                    out.push((start..index + 1, start_line));
                    start = index + 1;
                    start_line = line;
                }
                _ => {}
            },
            ScanState::Single => {
                if bytes[index] == b'\'' {
                    if bytes.get(index + 1) == Some(&b'\'') {
                        index += 1;
                    } else {
                        state = ScanState::Normal;
                    }
                }
            }
            ScanState::Double => {
                if bytes[index] == b'"' {
                    if bytes.get(index + 1) == Some(&b'"') {
                        index += 1;
                    } else {
                        state = ScanState::Normal;
                    }
                }
            }
            ScanState::Backtick => {
                if bytes[index] == b'`' {
                    state = ScanState::Normal;
                }
            }
            ScanState::Bracket => {
                if bytes[index] == b']' {
                    state = ScanState::Normal;
                }
            }
            ScanState::LineComment => {
                if bytes[index] == b'\n' {
                    state = ScanState::Normal;
                }
            }
            ScanState::BlockComment => {
                if bytes[index] == b'*' && bytes.get(index + 1) == Some(&b'/') {
                    state = ScanState::Normal;
                    index += 1;
                }
            }
            ScanState::Dollar(tag) => {
                if bytes[index..].starts_with(tag) {
                    index += tag.len() - 1;
                    state = ScanState::Normal;
                }
            }
        }
        if bytes[index] == b'\n' {
            line += 1;
            if start == index + 1 {
                start_line = line;
            }
        }
        index += 1;
    }
    if start < bytes.len() && !source[start..].trim().is_empty() {
        out.push((start..bytes.len(), start_line));
    }
    out
}

fn dollar_tag(bytes: &[u8]) -> Option<Vec<u8>> {
    if bytes.first() != Some(&b'$') {
        return None;
    }
    let end = bytes[1..].iter().position(|byte| *byte == b'$')? + 1;
    bytes[1..end]
        .iter()
        .all(|byte| byte.is_ascii_alphanumeric() || *byte == b'_')
        .then(|| bytes[..=end].to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::batch::BatchKey;
    use crate::scheduler::Scheduler;
    use crate::walker::FsWalker;

    #[test]
    fn sql_scanner_respects_quoted_and_commented_semicolons() {
        let source = "-- ;\ncreate table users(id int, note text default ';');\n\
                      select * from users where note = 'it''s; fine';\n\
                      /* ; */ insert into users values (1, 'skip');\n";
        let contracts = collect_contracts(source);
        assert_eq!(contracts.len(), 2);
        assert_eq!(contracts[0].kind, ContractKind::Schema);
        assert_eq!(contracts[1].kind, ContractKind::Query);
    }

    #[test]
    fn sql_scanner_keeps_dollar_quoted_function_body_together() {
        let source = "create function f() returns void as $$ begin; end; $$ language plpgsql;\n\
                      create table kept(id int);\n";
        let ranges = statement_ranges(source);
        assert_eq!(ranges.len(), 2);
        let contracts = collect_contracts(source);
        assert_eq!(contracts.len(), 1);
        assert_eq!(contracts[0].kind, ContractKind::Schema);
    }

    #[test]
    fn sql_classifier_ignores_where_inside_dollar_quote() {
        let source = "select $$not a where clause;$$;\n";
        assert!(collect_contracts(source).is_empty());
    }

    #[test]
    fn sql_gate_admits_root_and_exactly_cited_nested_files_only() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::create_dir(root.join("schema")).unwrap();
        std::fs::create_dir(root.join("migrations")).unwrap();
        std::fs::write(root.join("root.sql"), "create table root_table(id int);\n").unwrap();
        std::fs::write(
            root.join("schema/contract.sql"),
            "create table cited_table(id int);\n",
        )
        .unwrap();
        std::fs::write(
            root.join("migrations/migration.sql"),
            "create table excluded_table(id int);\n",
        )
        .unwrap();
        std::fs::write(root.join("README.md"), "See `schema/contract.sql`.\n").unwrap();

        let report =
            Scheduler::new(root.to_path_buf(), FsWalker, 1_000_000, None).run_with_report();
        let sql_files: Vec<PathBuf> = report
            .scheduled
            .iter()
            .filter_map(|record| match &record.key {
                BatchKey::Sql(SqlKey::SchemaChunk { file, .. }) => Some(file.clone()),
                _ => None,
            })
            .collect();
        assert!(sql_files.contains(&root.join("root.sql")));
        assert!(sql_files.contains(&root.join("schema/contract.sql")));
        assert!(!sql_files.contains(&root.join("migrations/migration.sql")));
    }

    #[test]
    fn sql_query_selection_prefers_rich_where_contracts() {
        let mut source = String::from("create table t(id int, kind text);\n");
        for index in 0..8 {
            source.push_str(&format!("select * from t where id = {index};\n"));
        }
        source.push_str(
            "select id, kind from t\nwhere id between 1 and 9\n  and kind = 'kept'\n  and id > 2;\n",
        );
        let contracts = collect_contracts(&source);
        let mut queries: Vec<_> = contracts
            .into_iter()
            .filter(|contract| contract.kind == ContractKind::Query)
            .collect();
        queries.sort_unstable_by_key(|contract| std::cmp::Reverse(contract.richness));
        queries.truncate(REPRESENTATIVE_QUERY_LIMIT);
        assert!(queries.iter().any(|contract| contract.richness > 20));
    }

    #[test]
    fn sql_same_line_contracts_have_one_render_owner() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::write(
            root.join("contract.sql"),
            "create table t(id int); select * from t where id = 1;\n",
        )
        .unwrap();

        let report =
            Scheduler::new(root.to_path_buf(), FsWalker, 1_000_000, None).run_with_report();
        let sql_batches = report
            .scheduled
            .iter()
            .filter(|record| matches!(record.key, BatchKey::Sql(_)))
            .count();
        assert_eq!(sql_batches, 1);
    }
}
