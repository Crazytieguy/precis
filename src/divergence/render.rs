use std::collections::HashMap;
use std::fmt::Write as _;

use super::*;

pub(super) fn format_report(scores: &Scores, ctx: &BuildCtx) -> String {
    let mut out = String::new();
    writeln!(out, "{} {}", scores.headline(), scores.grid()).unwrap();
    format_schedule_table(&mut out, ctx);
    out
}

/// Interleaved NS + walker timeline, sorted by cumulative tokens —
/// the divergence report's primary surface.
fn format_schedule_table(out: &mut String, ctx: &BuildCtx) {
    out.push('\n');
    out.push_str(
        "| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |\n",
    );
    out.push_str(
        "|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|\n",
    );

    let mut walker_cum: HashMap<&Atom, usize> = HashMap::new();
    let mut a_b_atoms: usize = 0;
    let mut prev_ns_t: usize = 0;

    let mut ns_iter = ctx.ns_rows.iter().enumerate().peekable();
    let mut walker_iter = ctx.walker_rows.iter().peekable();

    // Process all rows sharing a `cum` value as one tied group: advance
    // every row's state into `walker_cum` / `a_b_atoms` first, then
    // compute `Score(B = cum)` ONCE and print that single value on every
    // row in the group. Otherwise tied walker + NS rows would show two
    // different Score values for the same `B`, which is order-dependent
    // garbage — `Score(B)` is a function of state, not of iteration order.
    //
    // Each pending row carries the formatted prefix (everything except
    // the trailing Score column) so the Score can be appended after the
    // group's state advance.
    let mut pending: Vec<String> = Vec::new();
    loop {
        let next_cum = match (
            ns_iter.peek().map(|(_, r)| r.exp_t),
            walker_iter.peek().map(|wr| wr.seen_t),
        ) {
            (None, None) => break,
            (Some(n), None) => n,
            (None, Some(w)) => w,
            (Some(n), Some(w)) => n.min(w),
        };
        pending.clear();
        // Walker rows for this cum first (arbitrary within-group order —
        // they all carry the same Score after state advance).
        while walker_iter.peek().is_some_and(|wr| wr.seen_t == next_cum) {
            let wr = walker_iter.next().unwrap();
            fold_walker_atoms(&mut walker_cum, &wr.atoms);
            pending.push(format!(
                "| walker |  | {} | {} | {} |  |  |",
                wr.seen_t,
                wr.batch.cost_tokens,
                escape_cell(&wr.batch.descriptor),
            ));
        }
        while ns_iter.peek().is_some_and(|(_, r)| r.exp_t == next_cum) {
            let (i, ns) = ns_iter.next().unwrap();
            a_b_atoms += ns.atoms.len();
            let marginal = ns.exp_t.saturating_sub(prev_ns_t);
            prev_ns_t = ns.exp_t;
            let nsb = &ctx.ns.batches[i];
            pending.push(format!(
                "| ns | {} |  | {} | {} | {} | {} |",
                ns.exp_t,
                marginal,
                escape_cell(&nsb.descriptor),
                escape_cell(&nsb.id),
                escape_cell(nsb.predecessor.as_deref().unwrap_or("")),
            ));
        }
        let score = compute_score_at_running(ctx, next_cum, &walker_cum, a_b_atoms).score;
        for prefix in &pending {
            writeln!(out, "{prefix} {score:.3} |").unwrap();
        }
    }
}

/// Markdown table-cell escape. Backslashes are doubled BEFORE pipes
/// are escaped — in GFM, only an odd-length backslash run escapes a pipe.
fn escape_cell(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('|', "\\|")
        .replace(['\n', '\r'], " ")
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::content::BatchContent;
    use crate::north_star::{NorthStar, NsBatch};
    use crate::schedule_types::{Atom, ScheduledBatch};

    use super::super::{BuildCtx, GradedAtom, NsRow, WalkerRow};
    use super::format_schedule_table;

    fn line_atom(line: usize) -> GradedAtom {
        GradedAtom {
            atom: Atom::Line {
                path: PathBuf::from("src/x.rs"),
                line,
            },
            bytes: 10,
        }
    }

    /// Rows that share a `cum` value must show the same `Score(B=cum)`.
    /// Codex caught the earlier walker-first one-at-a-time advance
    /// producing `0.569` on a walker row and `0.550` on a tied NS row
    /// at B=458 in anyhow.md — order-dependent garbage. Fix groups
    /// tied rows and computes Score once.
    #[test]
    fn divergence_render_tied_cum_shares_score() {
        let ns = NorthStar {
            fixture: String::new(),
            revision_pin: String::new(),
            summary: String::new(),
            batches: vec![NsBatch {
                id: "1.1".into(),
                descriptor: "row at 100".into(),
                justification: String::new(),
                predecessor: None,
                content: BatchContent::Lines { spans: vec![] },
            }],
        };
        // Walker delivers nothing the NS wants — Score(100) ends up 0.
        // What matters: both rows at cum=100 print the *same* Score.
        let walker_batch = ScheduledBatch {
            position: 1,
            key: String::new(),
            descriptor: "walker at 100".into(),
            cost_tokens: 100,
            cum_tokens: 100,
            content: BatchContent::Lines { spans: vec![] },
        };
        let ns_rows = vec![NsRow {
            atoms: vec![line_atom(1)],
            exp_t: 100,
            rank_start: 1,
        }];
        let walker_rows = vec![WalkerRow {
            atoms: vec![line_atom(99)],
            seen_t: 100,
            batch: &walker_batch,
        }];
        let ctx = BuildCtx {
            ns_rows,
            walker_rows,
            ns: &ns,
        };
        let mut out = String::new();
        format_schedule_table(&mut out, &ctx);
        let scores: Vec<&str> = out
            .lines()
            .filter(|l| l.starts_with("| walker |") || l.starts_with("| ns |"))
            .map(|l| l.rsplit('|').nth(1).unwrap().trim())
            .collect();
        assert_eq!(scores.len(), 2, "two rows at the same cum: {out}");
        assert_eq!(
            scores[0], scores[1],
            "tied-cum rows must share Score(B=cum): {out}"
        );
    }

    /// NS descriptors / ids / predecessors containing `|` must be
    /// cell-escaped so they don't split the row into extra columns.
    /// Three input shapes the escaper has to handle correctly: bare
    /// `|`, an already-escaped `\|` (e.g. a regex fragment authored as
    /// `\|`), and a literal backslash `\\` followed by `|`. The
    /// invariant: the number of *unescaped* pipes in a rendered row
    /// equals 9 (the 8-column row's 9 separators), regardless of how
    /// many `|`s appear inside cells.
    #[test]
    fn divergence_render_escapes_pipe_in_descriptor() {
        for input in [
            "Ranges (a || b)",
            "regex \\| operator",
            "double backslash \\\\| pipe",
        ] {
            let ns = NorthStar {
                fixture: String::new(),
                revision_pin: String::new(),
                summary: String::new(),
                batches: vec![NsBatch {
                    id: "1.1".into(),
                    descriptor: input.into(),
                    justification: String::new(),
                    predecessor: None,
                    content: BatchContent::Lines { spans: vec![] },
                }],
            };
            let ns_rows = vec![NsRow {
                atoms: vec![line_atom(1)],
                exp_t: 100,
                rank_start: 1,
            }];
            let ctx = BuildCtx {
                ns_rows,
                walker_rows: vec![],
                ns: &ns,
            };
            let mut out = String::new();
            format_schedule_table(&mut out, &ctx);
            let ns_row = out
                .lines()
                .find(|l| l.starts_with("| ns |"))
                .expect("ns row");
            // Count unescaped pipes = total `|` minus pipes preceded by
            // an odd-length run of backslashes. Cheap approximation: for
            // each position of `|`, walk back counting `\`. An odd count
            // means escaped.
            let bytes = ns_row.as_bytes();
            let unescaped = (0..bytes.len())
                .filter(|&i| bytes[i] == b'|')
                .filter(|&i| {
                    let mut k = i;
                    let mut bs = 0usize;
                    while k > 0 && bytes[k - 1] == b'\\' {
                        bs += 1;
                        k -= 1;
                    }
                    bs.is_multiple_of(2)
                })
                .count();
            assert_eq!(
                unescaped, 9,
                "input {input:?} row {ns_row:?} should have 9 unescaped pipes"
            );
        }
    }
}
