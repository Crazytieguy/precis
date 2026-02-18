use precis::{format, walk};
use std::path::Path;

/// Helper to get the path to a test fixture. Returns None if the fixture isn't cloned.
fn fixture_path(name: &str) -> Option<std::path::PathBuf> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("test/fixtures")
        .join(name);
    if path.exists() { Some(path) } else { None }
}

// Clone fixtures with:
//   git clone --depth 1 https://github.com/rayon-rs/either.git test/fixtures/either
//   git clone --depth 1 https://github.com/supermacro/neverthrow.git test/fixtures/neverthrow
//   git clone --depth 1 https://github.com/npm/node-semver.git test/fixtures/semver
//   git clone --depth 1 https://github.com/pacocoursey/cmdk.git test/fixtures/cmdk
//   git clone --depth 1 https://github.com/gvergnaud/ts-pattern.git test/fixtures/ts-pattern
//   git clone --depth 1 https://github.com/dtolnay/anyhow.git test/fixtures/anyhow
//   git clone --depth 1 https://github.com/matklad/once_cell.git test/fixtures/once_cell
//   git clone --depth 1 https://github.com/timolins/react-hot-toast.git test/fixtures/react-hot-toast
//   git clone --depth 1 https://github.com/ianstormtaylor/superstruct.git test/fixtures/superstruct
//   git clone --depth 1 https://github.com/motdotla/dotenv.git test/fixtures/dotenv
//   git clone --depth 1 https://github.com/tj/commander.js.git test/fixtures/commander
//   git clone --depth 1 https://github.com/dtolnay/thiserror.git test/fixtures/thiserror
//   git clone --depth 1 https://github.com/emilkowalski/sonner.git test/fixtures/sonner
//   git clone --depth 1 https://github.com/developit/mitt.git test/fixtures/mitt
//   git clone --depth 1 https://github.com/debug-js/debug.git test/fixtures/debug
//   git clone --depth 1 https://github.com/rust-lang/log.git test/fixtures/log
//   git clone --depth 1 https://github.com/sindresorhus/ky.git test/fixtures/ky
//   git clone --depth 1 https://github.com/npm/ini.git test/fixtures/ini
//   git clone --depth 1 https://github.com/emilkowalski/vaul.git test/fixtures/vaul
//   git clone --depth 1 https://github.com/guilhermerodz/input-otp.git test/fixtures/input-otp
//   git clone --depth 1 https://github.com/pytest-dev/pluggy.git test/fixtures/pluggy
//   git clone --depth 1 https://github.com/hukkin/tomli.git test/fixtures/tomli
//   git clone --depth 1 https://github.com/python-humanize/humanize.git test/fixtures/humanize
//   git clone --depth 1 https://github.com/theskumar/python-dotenv.git test/fixtures/python-dotenv
//   git clone --depth 1 https://github.com/agronholm/typeguard.git test/fixtures/typeguard
//   git clone --depth 1 https://github.com/rust-lang/mdBook.git test/fixtures/mdbook
//   git clone --depth 1 https://github.com/hashicorp/go-multierror.git test/fixtures/go-multierror
//   git clone --depth 1 https://github.com/cespare/xxhash.git test/fixtures/xxhash
//   git clone --depth 1 https://github.com/fatih/color.git test/fixtures/color
//   git clone --depth 1 https://github.com/hashicorp/go-version.git test/fixtures/go-version
//   git clone --depth 1 https://github.com/fatih/structs.git test/fixtures/structs
//   git clone --depth 1 https://github.com/antirez/sds.git test/fixtures/sds

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

// Budget monotonicity: more budget should never produce fewer words.
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
    ];
    let budgets = [0, 10, 20, 50, 100, 200, 500, 1000, 10000];

    for (filename, source) in samples {
        let mut prev_words = 0;
        for &budget in &budgets {
            let output = format::render_file_with_budget(
                budget,
                Path::new(filename),
                Path::new(""),
                source,
            );
            let words = format::count_words(&output);
            assert!(
                words >= prev_words,
                "Budget monotonicity violation in {}: budget {} ({} words) < previous ({} words)",
                filename,
                budget,
                words,
                prev_words,
            );
            prev_words = words;
        }
    }
}

// Single-file rendering tests (precis accepts individual files, not just directories).

#[test]
fn single_file_budget_rust() {
    let Some(root) = fixture_path("either/src") else {
        eprintln!("skipping single_file_budget_rust: clone either fixture");
        return;
    };
    let file = root.join("lib.rs");
    let source = std::fs::read_to_string(&file).unwrap();

    // Budget monotonicity across a range
    let mut prev_words = 0;
    for budget in [0, 50, 100, 200, 500, 1000, 10000] {
        let output = format::render_file_with_budget(budget, &file, &root, &source);
        let words = format::count_words(&output);
        assert!(
            words >= prev_words,
            "Single file budget {} ({} words) < previous ({} words)",
            budget,
            words,
            prev_words,
        );
        prev_words = words;
    }
}

// Budget-based fixture snapshot tests.
// Most are commented out — re-introduce as performance and determinism improve.

/// Helper: render a fixture with a word budget and return output with metadata header.
fn render_with_budget(subpath: &str, budget: usize) -> Option<String> {
    let root = fixture_path(subpath)?;
    let files = walk::discover_source_files(&root);
    let sources = format::read_sources(&files);
    let output = format::render_with_budget(budget, &root, &files, &sources);
    let words = format::count_words(&output);
    Some(format!(
        "budget: {} ({} words)\n\n{}",
        budget, words, output
    ))
}

// Small fixture tests: one per language family, at two budget levels.
budget_test!(budget_either_src_500, "either/src", 500);
budget_test!(budget_either_src_2000, "either/src", 2000);
budget_test!(budget_go_multierror_500, "go-multierror", 500);
budget_test!(budget_go_multierror_2000, "go-multierror", 2000);
budget_test!(budget_mitt_src_500, "mitt/src", 500);
budget_test!(budget_mitt_src_2000, "mitt/src", 2000);
budget_test!(budget_sds_500, "sds", 500);
budget_test!(budget_sds_2000, "sds", 2000);

// Subdirectory budget tests: running on a subdirectory within a fixture tests
// that path display and file discovery work correctly at deeper nesting levels.

budget_test!(budget_ts_pattern_src_types_500, "ts-pattern/src/types", 500);
budget_test!(budget_ts_pattern_src_types_1000, "ts-pattern/src/types", 1000);
budget_test!(budget_ts_pattern_src_types_2000, "ts-pattern/src/types", 2000);
budget_test!(budget_ts_pattern_src_types_4000, "ts-pattern/src/types", 4000);
budget_test!(budget_react_hot_toast_src_components_500, "react-hot-toast/src/components", 500);
budget_test!(budget_react_hot_toast_src_components_1000, "react-hot-toast/src/components", 1000);
budget_test!(budget_react_hot_toast_src_components_2000, "react-hot-toast/src/components", 2000);
budget_test!(budget_react_hot_toast_src_components_4000, "react-hot-toast/src/components", 4000);
budget_test!(budget_superstruct_src_structs_500, "superstruct/src/structs", 500);
budget_test!(budget_superstruct_src_structs_1000, "superstruct/src/structs", 1000);
budget_test!(budget_superstruct_src_structs_2000, "superstruct/src/structs", 2000);
budget_test!(budget_superstruct_src_structs_4000, "superstruct/src/structs", 4000);
budget_test!(budget_log_src_kv_500, "log/src/kv", 500);
budget_test!(budget_log_src_kv_1000, "log/src/kv", 1000);
budget_test!(budget_log_src_kv_2000, "log/src/kv", 2000);
budget_test!(budget_log_src_kv_4000, "log/src/kv", 4000);
budget_test!(budget_semver_functions_500, "semver/functions", 500);
budget_test!(budget_semver_functions_1000, "semver/functions", 1000);
budget_test!(budget_semver_functions_2000, "semver/functions", 2000);
budget_test!(budget_semver_functions_4000, "semver/functions", 4000);
budget_test!(budget_neverthrow_src_internals_500, "neverthrow/src/_internals", 500);
budget_test!(budget_neverthrow_src_internals_1000, "neverthrow/src/_internals", 1000);
budget_test!(budget_neverthrow_src_internals_2000, "neverthrow/src/_internals", 2000);
budget_test!(budget_neverthrow_src_internals_4000, "neverthrow/src/_internals", 4000);
budget_test!(budget_ky_source_errors_500, "ky/source/errors", 500);
budget_test!(budget_ky_source_errors_1000, "ky/source/errors", 1000);
budget_test!(budget_ky_source_errors_2000, "ky/source/errors", 2000);
budget_test!(budget_ky_source_errors_4000, "ky/source/errors", 4000);
budget_test!(budget_thiserror_impl_src_500, "thiserror/impl/src", 500);
budget_test!(budget_thiserror_impl_src_1000, "thiserror/impl/src", 1000);
budget_test!(budget_thiserror_impl_src_2000, "thiserror/impl/src", 2000);
budget_test!(budget_thiserror_impl_src_4000, "thiserror/impl/src", 4000);
budget_test!(budget_semver_internal_500, "semver/internal", 500);
budget_test!(budget_semver_internal_1000, "semver/internal", 1000);
budget_test!(budget_semver_internal_2000, "semver/internal", 2000);
budget_test!(budget_semver_internal_4000, "semver/internal", 4000);
budget_test!(budget_mdbook_guide_src_cli_500, "mdbook/guide/src/cli", 500);
budget_test!(budget_mdbook_guide_src_cli_1000, "mdbook/guide/src/cli", 1000);
budget_test!(budget_mdbook_guide_src_cli_2000, "mdbook/guide/src/cli", 2000);
budget_test!(budget_mdbook_guide_src_cli_4000, "mdbook/guide/src/cli", 4000);
budget_test!(budget_mdbook_guide_src_format_500, "mdbook/guide/src/format", 500);
budget_test!(budget_mdbook_guide_src_format_1000, "mdbook/guide/src/format", 1000);
budget_test!(budget_mdbook_guide_src_format_2000, "mdbook/guide/src/format", 2000);
budget_test!(budget_mdbook_guide_src_format_4000, "mdbook/guide/src/format", 4000);
budget_test!(budget_xxhash_xxhsum_500, "xxhash/xxhsum", 500);
budget_test!(budget_xxhash_xxhsum_1000, "xxhash/xxhsum", 1000);
budget_test!(budget_xxhash_xxhsum_2000, "xxhash/xxhsum", 2000);
budget_test!(budget_xxhash_xxhsum_4000, "xxhash/xxhsum", 4000);

budget_test!(budget_mitt_src_1000, "mitt/src", 1000);
budget_test!(budget_mitt_src_4000, "mitt/src", 4000);
budget_test!(budget_ini_lib_500, "ini/lib", 500);
budget_test!(budget_ini_lib_1000, "ini/lib", 1000);
budget_test!(budget_ini_lib_2000, "ini/lib", 2000);
budget_test!(budget_ini_lib_4000, "ini/lib", 4000);
budget_test!(budget_neverthrow_src_500, "neverthrow/src", 500);
budget_test!(budget_neverthrow_src_1000, "neverthrow/src", 1000);
budget_test!(budget_neverthrow_src_2000, "neverthrow/src", 2000);
budget_test!(budget_neverthrow_src_4000, "neverthrow/src", 4000);
budget_test!(budget_either_src_1000, "either/src", 1000);
budget_test!(budget_either_src_4000, "either/src", 4000);
budget_test!(budget_pluggy_src_pluggy_500, "pluggy/src/pluggy", 500);
budget_test!(budget_pluggy_src_pluggy_1000, "pluggy/src/pluggy", 1000);
budget_test!(budget_pluggy_src_pluggy_2000, "pluggy/src/pluggy", 2000);
budget_test!(budget_pluggy_src_pluggy_4000, "pluggy/src/pluggy", 4000);
budget_test!(budget_sonner_src_500, "sonner/src", 500);
budget_test!(budget_sonner_src_1000, "sonner/src", 1000);
budget_test!(budget_sonner_src_2000, "sonner/src", 2000);
budget_test!(budget_sonner_src_4000, "sonner/src", 4000);
budget_test!(budget_mdbook_guide_src_500, "mdbook/guide/src", 500);
budget_test!(budget_mdbook_guide_src_1000, "mdbook/guide/src", 1000);
budget_test!(budget_mdbook_guide_src_2000, "mdbook/guide/src", 2000);
budget_test!(budget_mdbook_guide_src_4000, "mdbook/guide/src", 4000);
budget_test!(budget_go_multierror_1000, "go-multierror", 1000);
budget_test!(budget_go_multierror_4000, "go-multierror", 4000);
budget_test!(budget_xxhash_500, "xxhash", 500);
budget_test!(budget_xxhash_1000, "xxhash", 1000);
budget_test!(budget_xxhash_2000, "xxhash", 2000);
budget_test!(budget_xxhash_4000, "xxhash", 4000);
budget_test!(budget_color_500, "color", 500);
budget_test!(budget_color_1000, "color", 1000);
budget_test!(budget_color_2000, "color", 2000);
budget_test!(budget_color_4000, "color", 4000);
budget_test!(budget_go_version_500, "go-version", 500);
budget_test!(budget_go_version_1000, "go-version", 1000);
budget_test!(budget_go_version_2000, "go-version", 2000);
budget_test!(budget_go_version_4000, "go-version", 4000);
budget_test!(budget_structs_500, "structs", 500);
budget_test!(budget_structs_1000, "structs", 1000);
budget_test!(budget_structs_2000, "structs", 2000);
budget_test!(budget_structs_4000, "structs", 4000);
budget_test!(budget_typeguard_src_typeguard_500, "typeguard/src/typeguard", 500);
budget_test!(budget_typeguard_src_typeguard_1000, "typeguard/src/typeguard", 1000);
budget_test!(budget_typeguard_src_typeguard_2000, "typeguard/src/typeguard", 2000);
budget_test!(budget_typeguard_src_typeguard_4000, "typeguard/src/typeguard", 4000);
budget_test!(budget_anyhow_src_500, "anyhow/src", 500);
budget_test!(budget_anyhow_src_1000, "anyhow/src", 1000);
budget_test!(budget_anyhow_src_2000, "anyhow/src", 2000);
budget_test!(budget_anyhow_src_4000, "anyhow/src", 4000);
budget_test!(budget_ts_pattern_src_500, "ts-pattern/src", 500);
budget_test!(budget_ts_pattern_src_1000, "ts-pattern/src", 1000);
budget_test!(budget_ts_pattern_src_2000, "ts-pattern/src", 2000);
budget_test!(budget_ts_pattern_src_4000, "ts-pattern/src", 4000);
budget_test!(budget_tomli_src_tomli_500, "tomli/src/tomli", 500);
budget_test!(budget_tomli_src_tomli_1000, "tomli/src/tomli", 1000);
budget_test!(budget_tomli_src_tomli_2000, "tomli/src/tomli", 2000);
budget_test!(budget_tomli_src_tomli_4000, "tomli/src/tomli", 4000);
budget_test!(budget_log_src_500, "log/src", 500);
budget_test!(budget_log_src_1000, "log/src", 1000);
budget_test!(budget_log_src_2000, "log/src", 2000);
budget_test!(budget_log_src_4000, "log/src", 4000);
budget_test!(budget_thiserror_src_500, "thiserror/src", 500);
budget_test!(budget_thiserror_src_1000, "thiserror/src", 1000);
budget_test!(budget_thiserror_src_2000, "thiserror/src", 2000);
budget_test!(budget_thiserror_src_4000, "thiserror/src", 4000);
budget_test!(budget_once_cell_src_500, "once_cell/src", 500);
budget_test!(budget_once_cell_src_1000, "once_cell/src", 1000);
budget_test!(budget_once_cell_src_2000, "once_cell/src", 2000);
budget_test!(budget_once_cell_src_4000, "once_cell/src", 4000);
budget_test!(budget_humanize_src_humanize_500, "humanize/src/humanize", 500);
budget_test!(budget_humanize_src_humanize_1000, "humanize/src/humanize", 1000);
budget_test!(budget_humanize_src_humanize_2000, "humanize/src/humanize", 2000);
budget_test!(budget_humanize_src_humanize_4000, "humanize/src/humanize", 4000);
budget_test!(budget_python_dotenv_src_dotenv_500, "python-dotenv/src/dotenv", 500);
budget_test!(budget_python_dotenv_src_dotenv_1000, "python-dotenv/src/dotenv", 1000);
budget_test!(budget_python_dotenv_src_dotenv_2000, "python-dotenv/src/dotenv", 2000);
budget_test!(budget_python_dotenv_src_dotenv_4000, "python-dotenv/src/dotenv", 4000);
budget_test!(budget_semver_classes_500, "semver/classes", 500);
budget_test!(budget_semver_classes_1000, "semver/classes", 1000);
budget_test!(budget_semver_classes_2000, "semver/classes", 2000);
budget_test!(budget_semver_classes_4000, "semver/classes", 4000);
budget_test!(budget_cmdk_cmdk_src_500, "cmdk/cmdk/src", 500);
budget_test!(budget_cmdk_cmdk_src_1000, "cmdk/cmdk/src", 1000);
budget_test!(budget_cmdk_cmdk_src_2000, "cmdk/cmdk/src", 2000);
budget_test!(budget_cmdk_cmdk_src_4000, "cmdk/cmdk/src", 4000);
budget_test!(budget_react_hot_toast_src_500, "react-hot-toast/src", 500);
budget_test!(budget_react_hot_toast_src_1000, "react-hot-toast/src", 1000);
budget_test!(budget_react_hot_toast_src_2000, "react-hot-toast/src", 2000);
budget_test!(budget_react_hot_toast_src_4000, "react-hot-toast/src", 4000);
budget_test!(budget_superstruct_src_500, "superstruct/src", 500);
budget_test!(budget_superstruct_src_1000, "superstruct/src", 1000);
budget_test!(budget_superstruct_src_2000, "superstruct/src", 2000);
budget_test!(budget_superstruct_src_4000, "superstruct/src", 4000);
budget_test!(budget_dotenv_lib_500, "dotenv/lib", 500);
budget_test!(budget_dotenv_lib_1000, "dotenv/lib", 1000);
budget_test!(budget_dotenv_lib_2000, "dotenv/lib", 2000);
budget_test!(budget_dotenv_lib_4000, "dotenv/lib", 4000);
budget_test!(budget_commander_lib_500, "commander/lib", 500);
budget_test!(budget_commander_lib_1000, "commander/lib", 1000);
budget_test!(budget_commander_lib_2000, "commander/lib", 2000);
budget_test!(budget_commander_lib_4000, "commander/lib", 4000);
budget_test!(budget_ky_source_500, "ky/source", 500);
budget_test!(budget_ky_source_1000, "ky/source", 1000);
budget_test!(budget_ky_source_2000, "ky/source", 2000);
budget_test!(budget_ky_source_4000, "ky/source", 4000);
budget_test!(budget_vaul_src_500, "vaul/src", 500);
budget_test!(budget_vaul_src_1000, "vaul/src", 1000);
budget_test!(budget_vaul_src_2000, "vaul/src", 2000);
budget_test!(budget_vaul_src_4000, "vaul/src", 4000);
budget_test!(budget_debug_src_500, "debug/src", 500);
budget_test!(budget_debug_src_1000, "debug/src", 1000);
budget_test!(budget_debug_src_2000, "debug/src", 2000);
budget_test!(budget_debug_src_4000, "debug/src", 4000);
budget_test!(budget_input_otp_packages_input_otp_src_500, "input-otp/packages/input-otp/src", 500);
budget_test!(budget_input_otp_packages_input_otp_src_1000, "input-otp/packages/input-otp/src", 1000);
budget_test!(budget_input_otp_packages_input_otp_src_2000, "input-otp/packages/input-otp/src", 2000);
budget_test!(budget_input_otp_packages_input_otp_src_4000, "input-otp/packages/input-otp/src", 4000);

// Root-level budget tests: exercise depth factors, multi-language discovery,
// and file filtering at the repo root.

budget_test!(budget_either_500, "either", 500);
budget_test!(budget_either_1000, "either", 1000);
budget_test!(budget_either_2000, "either", 2000);
budget_test!(budget_either_4000, "either", 4000);
budget_test!(budget_neverthrow_500, "neverthrow", 500);
budget_test!(budget_neverthrow_1000, "neverthrow", 1000);
budget_test!(budget_neverthrow_2000, "neverthrow", 2000);
budget_test!(budget_neverthrow_4000, "neverthrow", 4000);
budget_test!(budget_pluggy_500, "pluggy", 500);
budget_test!(budget_pluggy_1000, "pluggy", 1000);
budget_test!(budget_pluggy_2000, "pluggy", 2000);
budget_test!(budget_pluggy_4000, "pluggy", 4000);
budget_test!(budget_sonner_500, "sonner", 500);
budget_test!(budget_sonner_1000, "sonner", 1000);
budget_test!(budget_sonner_2000, "sonner", 2000);
budget_test!(budget_sonner_4000, "sonner", 4000);
budget_test!(budget_commander_500, "commander", 500);
budget_test!(budget_commander_1000, "commander", 1000);
budget_test!(budget_commander_2000, "commander", 2000);
budget_test!(budget_commander_4000, "commander", 4000);
budget_test!(budget_anyhow_500, "anyhow", 500);
budget_test!(budget_anyhow_1000, "anyhow", 1000);
budget_test!(budget_anyhow_2000, "anyhow", 2000);
budget_test!(budget_anyhow_4000, "anyhow", 4000);
budget_test!(budget_log_500, "log", 500);
budget_test!(budget_log_1000, "log", 1000);
budget_test!(budget_log_2000, "log", 2000);
budget_test!(budget_log_4000, "log", 4000);
budget_test!(budget_ts_pattern_500, "ts-pattern", 500);
budget_test!(budget_ts_pattern_1000, "ts-pattern", 1000);
budget_test!(budget_ts_pattern_2000, "ts-pattern", 2000);
budget_test!(budget_ts_pattern_4000, "ts-pattern", 4000);
budget_test!(budget_typeguard_500, "typeguard", 500);
budget_test!(budget_typeguard_1000, "typeguard", 1000);
budget_test!(budget_typeguard_2000, "typeguard", 2000);
budget_test!(budget_typeguard_4000, "typeguard", 4000);
budget_test!(budget_mdbook_500, "mdbook", 500);
budget_test!(budget_mdbook_1000, "mdbook", 1000);
budget_test!(budget_mdbook_2000, "mdbook", 2000);
budget_test!(budget_mdbook_4000, "mdbook", 4000);
budget_test!(budget_once_cell_500, "once_cell", 500);
budget_test!(budget_once_cell_1000, "once_cell", 1000);
budget_test!(budget_once_cell_2000, "once_cell", 2000);
budget_test!(budget_once_cell_4000, "once_cell", 4000);
budget_test!(budget_thiserror_500, "thiserror", 500);
budget_test!(budget_thiserror_1000, "thiserror", 1000);
budget_test!(budget_thiserror_2000, "thiserror", 2000);
budget_test!(budget_thiserror_4000, "thiserror", 4000);
budget_test!(budget_react_hot_toast_500, "react-hot-toast", 500);
budget_test!(budget_react_hot_toast_1000, "react-hot-toast", 1000);
budget_test!(budget_react_hot_toast_2000, "react-hot-toast", 2000);
budget_test!(budget_react_hot_toast_4000, "react-hot-toast", 4000);
budget_test!(budget_humanize_500, "humanize", 500);
budget_test!(budget_humanize_1000, "humanize", 1000);
budget_test!(budget_humanize_2000, "humanize", 2000);
budget_test!(budget_humanize_4000, "humanize", 4000);
budget_test!(budget_tomli_500, "tomli", 500);
budget_test!(budget_tomli_1000, "tomli", 1000);
budget_test!(budget_tomli_2000, "tomli", 2000);
budget_test!(budget_tomli_4000, "tomli", 4000);
budget_test!(budget_cmdk_500, "cmdk", 500);
budget_test!(budget_cmdk_1000, "cmdk", 1000);
budget_test!(budget_cmdk_2000, "cmdk", 2000);
budget_test!(budget_cmdk_4000, "cmdk", 4000);
budget_test!(budget_debug_500, "debug", 500);
budget_test!(budget_debug_1000, "debug", 1000);
budget_test!(budget_debug_2000, "debug", 2000);
budget_test!(budget_debug_4000, "debug", 4000);
budget_test!(budget_dotenv_500, "dotenv", 500);
budget_test!(budget_dotenv_1000, "dotenv", 1000);
budget_test!(budget_dotenv_2000, "dotenv", 2000);
budget_test!(budget_dotenv_4000, "dotenv", 4000);
budget_test!(budget_ini_500, "ini", 500);
budget_test!(budget_ini_1000, "ini", 1000);
budget_test!(budget_ini_2000, "ini", 2000);
budget_test!(budget_ini_4000, "ini", 4000);
budget_test!(budget_input_otp_500, "input-otp", 500);
budget_test!(budget_input_otp_1000, "input-otp", 1000);
budget_test!(budget_input_otp_2000, "input-otp", 2000);
budget_test!(budget_input_otp_4000, "input-otp", 4000);
budget_test!(budget_ky_500, "ky", 500);
budget_test!(budget_ky_1000, "ky", 1000);
budget_test!(budget_ky_2000, "ky", 2000);
budget_test!(budget_ky_4000, "ky", 4000);
budget_test!(budget_mitt_500, "mitt", 500);
budget_test!(budget_mitt_1000, "mitt", 1000);
budget_test!(budget_mitt_2000, "mitt", 2000);
budget_test!(budget_mitt_4000, "mitt", 4000);
budget_test!(budget_python_dotenv_500, "python-dotenv", 500);
budget_test!(budget_python_dotenv_1000, "python-dotenv", 1000);
budget_test!(budget_python_dotenv_2000, "python-dotenv", 2000);
budget_test!(budget_python_dotenv_4000, "python-dotenv", 4000);
budget_test!(budget_semver_500, "semver", 500);
budget_test!(budget_semver_1000, "semver", 1000);
budget_test!(budget_semver_2000, "semver", 2000);
budget_test!(budget_semver_4000, "semver", 4000);
budget_test!(budget_superstruct_500, "superstruct", 500);
budget_test!(budget_superstruct_1000, "superstruct", 1000);
budget_test!(budget_superstruct_2000, "superstruct", 2000);
budget_test!(budget_superstruct_4000, "superstruct", 4000);
budget_test!(budget_vaul_500, "vaul", 500);
budget_test!(budget_vaul_1000, "vaul", 1000);
budget_test!(budget_vaul_2000, "vaul", 2000);
budget_test!(budget_vaul_4000, "vaul", 4000);
