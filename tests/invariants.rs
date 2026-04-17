//! Structural invariant tests (design §10).
//!
//! Verifies R1 (source fidelity), line-number monotonicity, and R4
//! (override resolution) using proper assertions that run in release mode.
//!
//! D4 (non-overlapping items) used to be checked at `extract_items`, but
//! stage 7 moved overlap handling into the per-file chainer in
//! `src/group/files.rs`: overlapping items become `dependent_siblings` of
//! their chain root instead of being dropped, so the post-`extract_items`
//! stream is allowed to contain overlaps. The effective D4 guarantee is
//! observed at the output level via `check_monotonicity`.

use std::path::{Path, PathBuf};

// ---------------------------------------------------------------------------
// Output parsing
// ---------------------------------------------------------------------------

struct FileSection {
    header: String,
    lines: Vec<ContentLine>,
}

struct ContentLine {
    line_num: u32, // 1-indexed as displayed
    text: String,
}

fn parse_output(output: &str) -> Vec<FileSection> {
    let mut sections = Vec::new();
    let mut current_header: Option<String> = None;
    let mut current_lines: Vec<ContentLine> = Vec::new();

    for raw_line in output.lines() {
        if raw_line.is_empty() {
            if let Some(header) = current_header.take() {
                sections.push(FileSection {
                    header,
                    lines: std::mem::take(&mut current_lines),
                });
            }
            continue;
        }

        if let Some(arrow_pos) = raw_line.find('→') {
            let prefix = &raw_line[..arrow_pos];
            let after_arrow = &raw_line[arrow_pos + '→'.len_utf8()..];

            if after_arrow == "…" && prefix.trim().is_empty() {
                continue;
            }

            if let Ok(num) = prefix.trim().parse::<u32>() {
                let text = match after_arrow.strip_suffix(" …") {
                    Some(stripped) => stripped.to_string(),
                    None => after_arrow.to_string(),
                };
                current_lines.push(ContentLine {
                    line_num: num,
                    text,
                });
                continue;
            }
        }

        if let Some(header) = current_header.take() {
            sections.push(FileSection {
                header,
                lines: std::mem::take(&mut current_lines),
            });
        }
        current_header = Some(raw_line.to_string());
    }

    if let Some(header) = current_header {
        sections.push(FileSection {
            header,
            lines: current_lines,
        });
    }

    sections
}

// ---------------------------------------------------------------------------
// Invariant checks
// ---------------------------------------------------------------------------

/// R1: every content line in the output is a substring of the actual source line.
fn check_r1(sections: &[FileSection], root: &Path) {
    for section in sections {
        if section.header.ends_with('/') || section.lines.is_empty() {
            continue;
        }

        let file_path = root.join(&section.header);
        let source = match std::fs::read_to_string(&file_path) {
            Ok(s) => s,
            Err(_) => continue,
        };
        let src_lines: Vec<&str> = source.lines().collect();

        for cl in &section.lines {
            let idx = cl.line_num as usize - 1;
            assert!(
                idx < src_lines.len(),
                "R1: line {} out of range in {} (source has {} lines)",
                cl.line_num,
                section.header,
                src_lines.len(),
            );
            let actual = src_lines[idx];
            assert!(
                actual.contains(&cl.text),
                "R1: line {} in {}: output {:?} not found in source {:?}",
                cl.line_num,
                section.header,
                cl.text,
                actual,
            );
        }
    }
}

/// Line-number monotonicity: line numbers strictly increase within each file section.
fn check_monotonicity(sections: &[FileSection]) {
    for section in sections {
        let mut prev: Option<u32> = None;
        for cl in &section.lines {
            if let Some(p) = prev {
                assert!(
                    cl.line_num > p,
                    "monotonicity: line {} follows {} in {}",
                    cl.line_num,
                    p,
                    section.header,
                );
            }
            prev = Some(cl.line_num);
        }
    }
}

/// Run all output-level invariant checks (R1, monotonicity) for a given render.
fn check_output_invariants(output: &str, root: &Path) {
    let sections = parse_output(output);
    check_monotonicity(&sections);
    check_r1(&sections, root);
}

// ---------------------------------------------------------------------------
// Inline samples
// ---------------------------------------------------------------------------

const RUST_SAMPLE: &str = r#"
/// Process the input and return a list of tokens.
pub fn process(input: &str) -> Result<Vec<Token>, Error> {
    todo!()
}

fn helper() {}

pub struct Token {
    kind: TokenKind,
    span: Span,
}

pub enum TokenKind {
    Ident,
    Number,
    Symbol,
}

pub trait Visitor {
    fn visit_token(&mut self, token: &Token);
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.kind)
    }
}

pub type Span = (usize, usize);
const MAX_TOKENS: usize = 1024;
"#;

const PYTHON_SAMPLE: &str = r#"
from typing import Optional
from dataclasses import dataclass

def process_items(items: list[str]) -> int:
    return len(items)

def _helper() -> None:
    pass

@dataclass
class Token:
    kind: str
    span: tuple[int, int]
    value: Optional[str] = None

class Visitor:
    def visit_token(self, token: Token) -> None:
        pass
"#;

const MARKDOWN_SAMPLE: &str = "# Title\n\n\
Introduction paragraph.\n\n\
## Section One\n\n\
Content for section one.\n\n\
### Subsection\n\n\
More content.\n\n\
## Section Two\n\n\
Content for section two.\n";

const TOML_SAMPLE: &str = "[package]\n\
name = \"example\"\n\
version = \"0.1.0\"\n\n\
[dependencies]\n\
serde = \"1\"\n\n\
[dev-dependencies]\n\
insta = \"1.0\"\n";

const GO_SAMPLE: &str = r#"
package token

import "fmt"

// Token represents a lexical token.
type Token struct {
	Kind int
	Span [2]int
}

// Process returns a list of tokens.
func Process(input string) ([]Token, error) {
	return nil, nil
}

func helper() {}

type Visitor interface {
	VisitToken(token *Token)
}
"#;

const TYPESCRIPT_SAMPLE: &str = r#"
export function processItems(items: string[]): number {
    return items.length;
}

function helper(): void {}

export class TokenParser {
    private tokens: string[];
    constructor(input: string) {
        this.tokens = [];
    }
    public parse(): string[] {
        return this.tokens;
    }
}

export interface Visitor {
    visitToken(token: string): void;
}

export enum TokenKind {
    Ident = "ident",
    Number = "number",
}

export type Span = [number, number];
export const MAX_TOKENS = 1024;
"#;

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

fn samples() -> Vec<(&'static str, &'static str)> {
    vec![
        ("sample.rs", RUST_SAMPLE),
        ("sample.py", PYTHON_SAMPLE),
        ("README.md", MARKDOWN_SAMPLE),
        ("Cargo.toml", TOML_SAMPLE),
        ("sample.go", GO_SAMPLE),
        ("sample.ts", TYPESCRIPT_SAMPLE),
    ]
}

/// R1 + monotonicity on inline samples at various budgets.
#[test]
fn r1_and_monotonicity_inline() {
    for (filename, source) in samples() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join(filename);
        std::fs::write(&file_path, source).unwrap();

        for budget in [10, 50, 200, 1000, 10000] {
            let output = precis::render(&file_path, budget, None);
            check_output_invariants(&output, dir.path());
        }
    }
}

/// R1 + monotonicity on all available fixture directories.
#[test]
fn invariants_on_fixtures() {
    let fixtures_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("test/fixtures");
    if !fixtures_dir.exists() {
        return;
    }

    let mut fixture_dirs: Vec<PathBuf> = std::fs::read_dir(&fixtures_dir)
        .unwrap()
        .flatten()
        .filter(|e| e.path().is_dir())
        .map(|e| e.path())
        .collect();
    fixture_dirs.sort();

    for fixture_path in &fixture_dirs {
        let output = precis::render(fixture_path, 4000, None);
        check_output_invariants(&output, fixture_path);
    }
}

/// R4 survival: render at many budgets without panicking.
/// The assert! in render.rs (converted from debug_assert!) catches
/// override violations during the scheduling loop.
#[test]
fn r4_no_panic_across_budgets() {
    let budgets = [0, 1, 10, 50, 100, 200, 500, 1000, 5000, 10000];

    for (filename, source) in samples() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join(filename);
        std::fs::write(&file_path, source).unwrap();

        for &budget in &budgets {
            let _ = precis::render(&file_path, budget, None);
        }
    }

    // Also exercise char budget path
    for (filename, source) in samples() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join(filename);
        std::fs::write(&file_path, source).unwrap();

        for &budget in &[100, 500, 2000] {
            let _ = precis::render(&file_path, budget, Some(budget * 4));
        }
    }
}
