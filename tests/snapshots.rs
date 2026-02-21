use precis::{format, walk};
use std::path::Path;

/// Helper to get the path to a test fixture. Returns None if the fixture isn't cloned.
fn fixture_path(name: &str) -> Option<std::path::PathBuf> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("test/fixtures")
        .join(name);
    if path.exists() { Some(path) } else { None }
}

/// Generate a snapshot test that renders a fixture with a word budget.
macro_rules! budget_test {
    ($name:ident, $path:expr, $budget:expr) => {
        #[test]
        fn $name() {
            let Some(output) = render_with_budget($path, $budget) else {
                eprintln!(
                    "skipping {}: fixture not present at {}",
                    stringify!($name),
                    $path
                );
                return;
            };
            insta::assert_snapshot!(output);
        }
    };
}

// Run `cargo run --bin clone_fixtures` to clone all missing fixtures.
// Fixture/entry data is defined in test/fixtures.rs (shared with clone_fixtures bin).
macro_rules! with_fixtures { ($($tt:tt)*) => {} }
macro_rules! with_entries {
    ($(($name:ident, $path:expr, $budget:expr)),* $(,)?) => {
        $(budget_test!($name, $path, $budget);)*
    };
}
include!("../test/fixtures.rs");

fn rust_sample() -> &'static str {
    r#"
/// Process the input and return a list of tokens.
pub fn process(input: &str) -> Result<Vec<Token>, Error> {
    todo!()
}

fn helper() {}

/// A lexical token with its kind and source span.
pub struct Token {
    kind: TokenKind,
    span: Span,
}

/// The kind of a lexical token.
pub enum TokenKind {
    Ident,
    Number,
    Symbol,
}

/// Trait for visiting tokens in a token stream.
pub trait Visitor {
    fn visit_token(&mut self, token: &Token);
    fn visit_all(&mut self, tokens: &[Token]) {
        for t in tokens {
            self.visit_token(t);
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.kind)
    }
}

impl Token {
    /// Create a new token with the given kind and span.
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }
}

pub type Span = (usize, usize);

const MAX_TOKENS: usize = 1024;

pub static VERSION: &str = "0.1.0";

macro_rules! token {
    ($kind:expr) => {
        Token { kind: $kind, span: (0, 0) }
    };
}

pub mod lexer;
"#
}

fn typescript_sample() -> &'static str {
    r#"
/**
 * Process all items and return the count.
 */
export function processItems(items: string[]): number {
    return items.length;
}

function helper(): void {}

/**
 * Fetch data from the given URL.
 * @param url - The endpoint to fetch from.
 * @returns The fetch response.
 */
export async function fetchData(url: string): Promise<Response> {
    return fetch(url);
}

export class TokenParser {
    private tokens: Token[];

    constructor(input: string) {
        this.tokens = [];
    }

    public parse(): Token[] {
        return this.tokens;
    }

    private advance(): void {}
}

export interface Visitor {
    visitToken(token: Token): void;
    visitAll(tokens: Token[]): void;
}

export enum TokenKind {
    Ident = "ident",
    Number = "number",
    Symbol = "symbol",
}

export type Span = [number, number];

export const MAX_TOKENS = 1024;

export default class DefaultExport {
    name: string = "";
}

export namespace Utils {
    export function format(s: string): string {
        return s;
    }
}
"#
}

fn javascript_sample() -> &'static str {
    r#"
/**
 * Process all items and return the total count.
 * @param {Array} items - The items to process.
 * @returns {number} The count of items.
 */
export function processItems(items) {
    return items.length;
}

function helper() {}

/**
 * A parser that tokenizes input strings.
 */
export class TokenParser {
    constructor(input) {
        this.tokens = [];
    }

    parse() {
        return this.tokens;
    }

    #advance() {}
}

export const MAX_TOKENS = 1024;

export default class DefaultExport {
    name = "";
}
"#
}

fn tsx_sample() -> &'static str {
    r#"
import React, { useState, forwardRef } from "react";

/** Props for the Button component. */
export interface ButtonProps {
    label: string;
    onClick: () => void;
    disabled?: boolean;
}

/**
 * A clickable button component.
 * @param props - The button props.
 */
export function Button({ label, onClick, disabled }: ButtonProps) {
    return <button onClick={onClick} disabled={disabled}>{label}</button>;
}

export const IconButton = ({ icon, ...rest }: { icon: string } & ButtonProps) => {
    return <Button label={icon} {...rest} />;
};

function useToggle(initial: boolean): [boolean, () => void] {
    const [value, setValue] = useState(initial);
    return [value, () => setValue(v => !v)];
}

/** A forwarded-ref input component. */
export const Input = forwardRef<HTMLInputElement, React.InputHTMLAttributes<HTMLInputElement>>(
    (props, ref) => {
        return <input ref={ref} {...props} />;
    }
);

export type Theme = "light" | "dark";

export const MemoizedList = React.memo(function MemoizedList({ items }: { items: string[] }) {
    return <ul>{items.map(i => <li key={i}>{i}</li>)}</ul>;
});

export default function App() {
    const [on, toggle] = useToggle(false);
    return (
        <div>
            <Button label={on ? "On" : "Off"} onClick={toggle} />
        </div>
    );
}
"#
}

fn python_sample() -> &'static str {
    r#"
from typing import Optional, Generic, TypeVar
from dataclasses import dataclass

T = TypeVar("T")

# Process the input items and return the total count.
def process_items(items: list[str]) -> int:
    return len(items)

def _helper() -> None:
    pass

@dataclass
class Token:
    kind: str
    span: tuple[int, int]
    value: Optional[str] = None

# A visitor that walks over tokens.
class Visitor:
    def visit_token(self, token: Token) -> None:
        pass

    def visit_all(self, tokens: list[Token]) -> None:
        for t in tokens:
            self.visit_token(t)

class TokenParser(Generic[T]):
    """A generic token parser.

    Parses source text into a list of tokens.
    """

    MAX_TOKENS: int = 1024

    def __init__(self, source: str) -> None:
        """Initialize the parser with source text."""
        self._tokens: list[Token] = []
        self._source = source

    def parse(self) -> list[Token]:
        """Parse and return the token list."""
        return self._tokens

    @classmethod
    def from_file(cls, path: str) -> "TokenParser[T]":
        with open(path) as f:
            return cls(f.read())

    def _advance(self) -> None:
        pass

VERSION: str = "0.1.0"
"#
}

fn go_sample() -> &'static str {
    r#"
package token

import "fmt"

// Token represents a lexical token with its kind and source span.
type Token struct {
	Kind TokenKind
	Span Span
}

// TokenKind represents the kind of a lexical token.
type TokenKind int

const (
	Ident  TokenKind = iota
	Number
	Symbol
)

// Visitor walks over tokens in a stream.
type Visitor interface {
	VisitToken(token *Token)
	VisitAll(tokens []*Token)
}

// Process processes the input and returns a list of tokens.
func Process(input string) ([]Token, error) {
	return nil, nil
}

func helper() {}

// TokenParser parses source text into tokens.
type TokenParser[T any] struct {
	tokens []Token
	source string
}

// NewTokenParser creates a new parser.
func NewTokenParser[T any](source string) *TokenParser[T] {
	return &TokenParser[T]{source: source}
}

// Parse parses and returns the token list.
func (p *TokenParser[T]) Parse() []Token {
	return p.tokens
}

func (p *TokenParser[T]) advance() {}

// Span represents a source location as [start, end].
type Span = [2]int

const MaxTokens = 1024

var Version = "0.1.0"

// String implements fmt.Stringer for Token.
func (t Token) String() string {
	return fmt.Sprintf("%d:%v", t.Kind, t.Span)
}
"#
}

fn markdown_sample() -> &'static str {
    r#"# precis

A CLI tool that extracts a token-efficient summary of a codebase.

## Installation

```bash
cargo install precis
```

## Usage

### Basic Usage

Run on a directory to get a summary:

```
precis ./src
```

### Options

| Flag | Description |
|------|-------------|
| `--budget` | Word budget for output |
| `--json` | Output as JSON |

## Supported Languages

- Rust
- TypeScript / JavaScript
- Python
- Markdown

### Adding a New Language

1. Add a tree-sitter grammar dependency
2. Create a query file in `queries/`
3. Update `walk.rs` and `parse.rs`

## Contributing

Pull requests welcome! Please run tests before submitting.

## License

MIT
"#
}

fn c_sample() -> &'static str {
    r#"
#include <stdio.h>
#include <stdlib.h>
#include "token.h"

#define MAX_TOKENS 1024
#define MIN(a, b) ((a) < (b) ? (a) : (b))

typedef struct Token {
    int kind;
    int start;
    int end;
} Token;

/* The kind of a lexical token. */
enum TokenKind {
    TOKEN_IDENT,
    TOKEN_NUMBER,
    TOKEN_SYMBOL,
};

typedef int (*Comparator)(const void *, const void *);

/**
 * Process the input and return a list of tokens.
 * @param input The source text to tokenize.
 * @param len Length of the input.
 * @return Number of tokens extracted, or -1 on error.
 */
int process(const char *input, int len) {
    return len;
}

static void helper(void) {}

// Create a new token with the given kind and span.
Token *token_new(int kind, int start, int end) {
    Token *t = malloc(sizeof(Token));
    t->kind = kind;
    t->start = start;
    t->end = end;
    return t;
}

void token_free(Token *t) {
    free(t);
}

/* Internal helper for parsing. */
static int _parse_next(const char *input, int pos) {
    return pos + 1;
}

extern int global_count;

void *alloc_node(size_t size);
"#
}

fn toml_sample() -> &'static str {
    r#"[package]
name = "example"
version = "0.1.0"
edition = "2021"
description = "An example Rust crate"

[dependencies]
serde = { version = "1", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }
clap = "4"

[dev-dependencies]
insta = "1.0"
tempfile = "3"

[profile.release]
opt-level = 3
lto = true
strip = true

[[bin]]
name = "example"
path = "src/main.rs"

[workspace.metadata.release]
publish = false
"#
}

fn yaml_sample() -> &'static str {
    r#"name: CI

on:
  push:
    branches: [main]
  pull_request:

env:
  CARGO_TERM_COLOR: always
  RUST_BACKTRACE: 1

jobs:
  test:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        rust: [stable, nightly]
    steps:
      - uses: actions/checkout@v4
      - name: Install Rust
        uses: dtolnay/rust-toolchain@master
        with:
          toolchain: ${{ matrix.rust }}
      - name: Run tests
        run: cargo test --all-features

  lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Clippy
        run: cargo clippy -- -D warnings
      - name: Format
        run: cargo fmt -- --check

  release:
    needs: [test, lint]
    runs-on: ubuntu-latest
    if: github.ref == 'refs/heads/main'
    steps:
      - uses: actions/checkout@v4
      - name: Publish
        run: cargo publish
"#
}

fn json_sample() -> &'static str {
    r#"{
  "name": "@example/toolkit",
  "version": "2.0.0",
  "description": "A toolkit for building CLI applications",
  "main": "dist/index.js",
  "types": "dist/index.d.ts",
  "scripts": {
    "build": "tsc",
    "test": "jest --coverage",
    "lint": "eslint src/",
    "prepare": "npm run build"
  },
  "dependencies": {
    "chalk": "^5.3.0",
    "commander": "^12.0.0",
    "fs-extra": "^11.0.0"
  },
  "devDependencies": {
    "@types/node": "^20.0.0",
    "jest": "^29.0.0",
    "typescript": "^5.0.0",
    "eslint": "^9.0.0"
  },
  "repository": {
    "type": "git",
    "url": "https://github.com/example/toolkit.git"
  },
  "keywords": ["cli", "toolkit", "typescript"],
  "license": "MIT"
}
"#
}

// Budget-based inline sample tests: test each language at small and large budgets.

macro_rules! sample_test {
    ($name:ident, $filename:expr, $sample_fn:expr, $budget:expr) => {
        #[test]
        fn $name() {
            let output = format::render_file_with_budget(
                $budget,
                Path::new($filename),
                Path::new(""),
                $sample_fn(),
            );
            insta::assert_snapshot!(output);
        }
    };
}

sample_test!(rust_sample_budget_20, "sample.rs", rust_sample, 20);
sample_test!(rust_sample_budget_50, "sample.rs", rust_sample, 50);
sample_test!(rust_sample_budget_200, "sample.rs", rust_sample, 200);
sample_test!(rust_sample_budget_10000, "sample.rs", rust_sample, 10000);

sample_test!(typescript_sample_budget_20, "sample.ts", typescript_sample, 20);
sample_test!(typescript_sample_budget_50, "sample.ts", typescript_sample, 50);
sample_test!(typescript_sample_budget_200, "sample.ts", typescript_sample, 200);
sample_test!(typescript_sample_budget_10000, "sample.ts", typescript_sample, 10000);

sample_test!(javascript_sample_budget_20, "sample.js", javascript_sample, 20);
sample_test!(javascript_sample_budget_50, "sample.js", javascript_sample, 50);
sample_test!(javascript_sample_budget_200, "sample.js", javascript_sample, 200);
sample_test!(javascript_sample_budget_10000, "sample.js", javascript_sample, 10000);

sample_test!(tsx_sample_budget_20, "sample.tsx", tsx_sample, 20);
sample_test!(tsx_sample_budget_50, "sample.tsx", tsx_sample, 50);
sample_test!(tsx_sample_budget_200, "sample.tsx", tsx_sample, 200);
sample_test!(tsx_sample_budget_10000, "sample.tsx", tsx_sample, 10000);

sample_test!(python_sample_budget_20, "sample.py", python_sample, 20);
sample_test!(python_sample_budget_50, "sample.py", python_sample, 50);
sample_test!(python_sample_budget_200, "sample.py", python_sample, 200);
sample_test!(python_sample_budget_10000, "sample.py", python_sample, 10000);

sample_test!(go_sample_budget_20, "sample.go", go_sample, 20);
sample_test!(go_sample_budget_50, "sample.go", go_sample, 50);
sample_test!(go_sample_budget_200, "sample.go", go_sample, 200);
sample_test!(go_sample_budget_10000, "sample.go", go_sample, 10000);

sample_test!(c_sample_budget_20, "sample.c", c_sample, 20);
sample_test!(c_sample_budget_50, "sample.c", c_sample, 50);
sample_test!(c_sample_budget_200, "sample.c", c_sample, 200);
sample_test!(c_sample_budget_10000, "sample.c", c_sample, 10000);

sample_test!(markdown_sample_budget_20, "README.md", markdown_sample, 20);
sample_test!(markdown_sample_budget_50, "README.md", markdown_sample, 50);
sample_test!(markdown_sample_budget_200, "README.md", markdown_sample, 200);
sample_test!(markdown_sample_budget_10000, "README.md", markdown_sample, 10000);

sample_test!(toml_sample_budget_20, "Cargo.toml", toml_sample, 20);
sample_test!(toml_sample_budget_50, "Cargo.toml", toml_sample, 50);
sample_test!(toml_sample_budget_200, "Cargo.toml", toml_sample, 200);
sample_test!(toml_sample_budget_10000, "Cargo.toml", toml_sample, 10000);

sample_test!(yaml_sample_budget_20, "ci.yml", yaml_sample, 20);
sample_test!(yaml_sample_budget_50, "ci.yml", yaml_sample, 50);
sample_test!(yaml_sample_budget_200, "ci.yml", yaml_sample, 200);
sample_test!(yaml_sample_budget_10000, "ci.yml", yaml_sample, 10000);

sample_test!(json_sample_budget_20, "package.json", json_sample, 20);
sample_test!(json_sample_budget_50, "package.json", json_sample, 50);
sample_test!(json_sample_budget_200, "package.json", json_sample, 200);
sample_test!(json_sample_budget_10000, "package.json", json_sample, 10000);

// Budget monotonicity: more budget should never produce fewer tokens.
#[test]
fn budget_monotonicity_inline() {
    let samples: &[(&str, &str)] = &[
        ("sample.rs", rust_sample()),
        ("sample.ts", typescript_sample()),
        ("sample.js", javascript_sample()),
        ("sample.tsx", tsx_sample()),
        ("sample.py", python_sample()),
        ("sample.go", go_sample()),
        ("sample.c", c_sample()),
        ("README.md", markdown_sample()),
        ("Cargo.toml", toml_sample()),
        ("ci.yml", yaml_sample()),
        ("package.json", json_sample()),
    ];
    let budgets = [0, 10, 20, 50, 100, 200, 500, 1000, 10000];

    for (filename, source) in samples {
        let mut prev_tokens = 0;
        for &budget in &budgets {
            let output = format::render_file_with_budget(
                budget,
                Path::new(filename),
                Path::new(""),
                source,
            );
            let tokens = format::count_tokens(&output);
            assert!(
                tokens >= prev_tokens,
                "Budget monotonicity violation in {}: budget {} ({} tokens) < previous ({} tokens)",
                filename,
                budget,
                tokens,
                prev_tokens,
            );
            prev_tokens = tokens;
        }
    }
}

// Single-file rendering tests (precis accepts individual files, not just directories).

#[test]
fn single_file_budget_rust() {
    let Some(root) = fixture_path("anyhow/src") else {
        eprintln!("skipping single_file_budget_rust: clone anyhow fixture");
        return;
    };
    let file = root.join("lib.rs");
    let source = std::fs::read_to_string(&file).unwrap();

    // Budget monotonicity across a range
    let mut prev_tokens = 0;
    for budget in [0, 50, 100, 200, 500, 1000, 10000] {
        let output = format::render_file_with_budget(budget, &file, &root, &source);
        let tokens = format::count_tokens(&output);
        assert!(
            tokens >= prev_tokens,
            "Single file budget {} ({} tokens) < previous ({} tokens)",
            budget,
            tokens,
            prev_tokens,
        );
        prev_tokens = tokens;
    }
}

// Budget-based fixture snapshot tests.

/// Helper: render a fixture with a token budget and return output with metadata header.
fn render_with_budget(subpath: &str, budget: usize) -> Option<String> {
    let root = fixture_path(subpath)?;
    let files = walk::discover_source_files(&root);
    let sources = format::read_sources(&files);
    let output = format::render_with_budget(budget, &root, &files, &sources);
    let tokens = format::count_tokens(&output);
    Some(format!(
        "budget: {} ({} tokens)\n\n{}",
        budget, tokens, output
    ))
}

// Snapshot tests are generated from entries in test/fixtures.rs via the
// with_entries! callback macro defined above.