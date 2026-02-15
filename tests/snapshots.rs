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
| `--level` | Granularity level (0-5) |

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

// Regression test: inline comments after `{` should not prevent signature detection.
// Previously, `interface Foo { // comment` would miss the `{` because
// `trimmed.ends_with('{')` was false, causing body content to leak into signature output.
// Tests at level 4 (all signatures) where the regression would be visible.

#[test]
fn interface_with_inline_comment_level2() {
    let source = r#"
export interface Options extends Base { // eslint-disable-line
    method?: string;
    headers?: Record<string, string>;
    body?: BodyInit;
}

export interface Config {
    timeout: number;
    retries: number;
}

export function fetch(url: string): Promise<Response> { // main entry
    return globalThis.fetch(url);
}
"#;
    let output = format::render_file(4, Path::new("api.ts"), Path::new(""), source);
    insta::assert_snapshot!(output);
}

// Level 0: file paths only (same output for all languages)

#[test]
fn rust_sample_level0() {
    let output = format::render_file(0, Path::new("sample.rs"), Path::new(""), rust_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn typescript_sample_level0() {
    let output = format::render_file(
        0,
        Path::new("sample.ts"),
        Path::new(""),
        typescript_sample(),
    );
    insta::assert_snapshot!(output);
}

#[test]
fn javascript_sample_level0() {
    let output = format::render_file(
        0,
        Path::new("sample.js"),
        Path::new(""),
        javascript_sample(),
    );
    insta::assert_snapshot!(output);
}

#[test]
fn tsx_sample_level0() {
    let output = format::render_file(0, Path::new("sample.tsx"), Path::new(""), tsx_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn go_sample_level0() {
    let output = format::render_file(0, Path::new("sample.go"), Path::new(""), go_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn python_sample_level0() {
    let output = format::render_file(0, Path::new("sample.py"), Path::new(""), python_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn markdown_sample_level0() {
    let output = format::render_file(0, Path::new("README.md"), Path::new(""), markdown_sample());
    insta::assert_snapshot!(output);
}

// Level 1: public symbol names only (truncated at identifier)

#[test]
fn rust_sample_level1() {
    let output = format::render_file(1, Path::new("sample.rs"), Path::new(""), rust_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn typescript_sample_level1() {
    let output = format::render_file(
        1,
        Path::new("sample.ts"),
        Path::new(""),
        typescript_sample(),
    );
    insta::assert_snapshot!(output);
}

#[test]
fn javascript_sample_level1() {
    let output = format::render_file(
        1,
        Path::new("sample.js"),
        Path::new(""),
        javascript_sample(),
    );
    insta::assert_snapshot!(output);
}

#[test]
fn tsx_sample_level1() {
    let output = format::render_file(1, Path::new("sample.tsx"), Path::new(""), tsx_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn go_sample_level1() {
    let output = format::render_file(1, Path::new("sample.go"), Path::new(""), go_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn python_sample_level1() {
    let output = format::render_file(1, Path::new("sample.py"), Path::new(""), python_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn markdown_sample_level1() {
    let output =
        format::render_file(1, Path::new("README.md"), Path::new(""), markdown_sample());
    insta::assert_snapshot!(output);
}

// Level 2: all symbol names (truncated at identifier)

#[test]
fn rust_sample_level2() {
    let output = format::render_file(2, Path::new("sample.rs"), Path::new(""), rust_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn typescript_sample_level2() {
    let output = format::render_file(
        2,
        Path::new("sample.ts"),
        Path::new(""),
        typescript_sample(),
    );
    insta::assert_snapshot!(output);
}

#[test]
fn javascript_sample_level2() {
    let output = format::render_file(
        2,
        Path::new("sample.js"),
        Path::new(""),
        javascript_sample(),
    );
    insta::assert_snapshot!(output);
}

#[test]
fn tsx_sample_level2() {
    let output = format::render_file(2, Path::new("sample.tsx"), Path::new(""), tsx_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn go_sample_level2() {
    let output = format::render_file(2, Path::new("sample.go"), Path::new(""), go_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn python_sample_level2() {
    let output = format::render_file(2, Path::new("sample.py"), Path::new(""), python_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn markdown_sample_level2() {
    let output =
        format::render_file(2, Path::new("README.md"), Path::new(""), markdown_sample());
    insta::assert_snapshot!(output);
}

// Level 3: public signatures + private names (visibility-gated)

#[test]
fn rust_sample_level3() {
    let output = format::render_file(3, Path::new("sample.rs"), Path::new(""), rust_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn typescript_sample_level3() {
    let output = format::render_file(
        3,
        Path::new("sample.ts"),
        Path::new(""),
        typescript_sample(),
    );
    insta::assert_snapshot!(output);
}

#[test]
fn javascript_sample_level3() {
    let output = format::render_file(
        3,
        Path::new("sample.js"),
        Path::new(""),
        javascript_sample(),
    );
    insta::assert_snapshot!(output);
}

#[test]
fn tsx_sample_level3() {
    let output = format::render_file(3, Path::new("sample.tsx"), Path::new(""), tsx_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn go_sample_level3() {
    let output = format::render_file(3, Path::new("sample.go"), Path::new(""), go_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn python_sample_level3() {
    let output = format::render_file(3, Path::new("sample.py"), Path::new(""), python_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn markdown_sample_level3() {
    let output =
        format::render_file(3, Path::new("README.md"), Path::new(""), markdown_sample());
    insta::assert_snapshot!(output);
}

// Level 4: full signatures for all symbols

#[test]
fn rust_sample_level4() {
    let output = format::render_file(4, Path::new("sample.rs"), Path::new(""), rust_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn typescript_sample_level4() {
    let output = format::render_file(
        4,
        Path::new("sample.ts"),
        Path::new(""),
        typescript_sample(),
    );
    insta::assert_snapshot!(output);
}

#[test]
fn javascript_sample_level4() {
    let output = format::render_file(
        4,
        Path::new("sample.js"),
        Path::new(""),
        javascript_sample(),
    );
    insta::assert_snapshot!(output);
}

#[test]
fn tsx_sample_level4() {
    let output = format::render_file(4, Path::new("sample.tsx"), Path::new(""), tsx_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn go_sample_level4() {
    let output = format::render_file(4, Path::new("sample.go"), Path::new(""), go_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn python_sample_level4() {
    let output = format::render_file(4, Path::new("sample.py"), Path::new(""), python_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn markdown_sample_level4() {
    let output =
        format::render_file(4, Path::new("README.md"), Path::new(""), markdown_sample());
    insta::assert_snapshot!(output);
}

// Level 5: first-line doc comments (public symbols only)

#[test]
fn rust_sample_level5() {
    let output = format::render_file(5, Path::new("sample.rs"), Path::new(""), rust_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn typescript_sample_level5() {
    let output = format::render_file(
        5,
        Path::new("sample.ts"),
        Path::new(""),
        typescript_sample(),
    );
    insta::assert_snapshot!(output);
}

#[test]
fn javascript_sample_level5() {
    let output = format::render_file(
        5,
        Path::new("sample.js"),
        Path::new(""),
        javascript_sample(),
    );
    insta::assert_snapshot!(output);
}

#[test]
fn tsx_sample_level5() {
    let output = format::render_file(5, Path::new("sample.tsx"), Path::new(""), tsx_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn go_sample_level5() {
    let output = format::render_file(5, Path::new("sample.go"), Path::new(""), go_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn python_sample_level5() {
    let output = format::render_file(5, Path::new("sample.py"), Path::new(""), python_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn markdown_sample_level5() {
    let output =
        format::render_file(5, Path::new("README.md"), Path::new(""), markdown_sample());
    insta::assert_snapshot!(output);
}

// Level 6: full doc comments (public symbols only)

#[test]
fn rust_sample_level6() {
    let output = format::render_file(6, Path::new("sample.rs"), Path::new(""), rust_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn typescript_sample_level6() {
    let output = format::render_file(
        6,
        Path::new("sample.ts"),
        Path::new(""),
        typescript_sample(),
    );
    insta::assert_snapshot!(output);
}

#[test]
fn javascript_sample_level6() {
    let output = format::render_file(
        6,
        Path::new("sample.js"),
        Path::new(""),
        javascript_sample(),
    );
    insta::assert_snapshot!(output);
}

#[test]
fn tsx_sample_level6() {
    let output = format::render_file(6, Path::new("sample.tsx"), Path::new(""), tsx_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn go_sample_level6() {
    let output = format::render_file(6, Path::new("sample.go"), Path::new(""), go_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn python_sample_level6() {
    let output = format::render_file(6, Path::new("sample.py"), Path::new(""), python_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn markdown_sample_level6() {
    let output =
        format::render_file(6, Path::new("README.md"), Path::new(""), markdown_sample());
    insta::assert_snapshot!(output);
}

// Level 7: full doc comments (all symbols)

#[test]
fn rust_sample_level7() {
    let output = format::render_file(7, Path::new("sample.rs"), Path::new(""), rust_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn typescript_sample_level7() {
    let output = format::render_file(
        7,
        Path::new("sample.ts"),
        Path::new(""),
        typescript_sample(),
    );
    insta::assert_snapshot!(output);
}

#[test]
fn javascript_sample_level7() {
    let output = format::render_file(
        7,
        Path::new("sample.js"),
        Path::new(""),
        javascript_sample(),
    );
    insta::assert_snapshot!(output);
}

#[test]
fn tsx_sample_level7() {
    let output = format::render_file(7, Path::new("sample.tsx"), Path::new(""), tsx_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn go_sample_level7() {
    let output = format::render_file(7, Path::new("sample.go"), Path::new(""), go_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn python_sample_level7() {
    let output = format::render_file(7, Path::new("sample.py"), Path::new(""), python_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn markdown_sample_level7() {
    let output =
        format::render_file(7, Path::new("README.md"), Path::new(""), markdown_sample());
    insta::assert_snapshot!(output);
}

// Level 8: type body expansion (public types only)

#[test]
fn rust_sample_level8() {
    let output = format::render_file(8, Path::new("sample.rs"), Path::new(""), rust_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn typescript_sample_level8() {
    let output = format::render_file(
        8,
        Path::new("sample.ts"),
        Path::new(""),
        typescript_sample(),
    );
    insta::assert_snapshot!(output);
}

#[test]
fn javascript_sample_level8() {
    let output = format::render_file(
        8,
        Path::new("sample.js"),
        Path::new(""),
        javascript_sample(),
    );
    insta::assert_snapshot!(output);
}

#[test]
fn tsx_sample_level8() {
    let output = format::render_file(8, Path::new("sample.tsx"), Path::new(""), tsx_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn go_sample_level8() {
    let output = format::render_file(8, Path::new("sample.go"), Path::new(""), go_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn python_sample_level8() {
    let output = format::render_file(8, Path::new("sample.py"), Path::new(""), python_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn markdown_sample_level8() {
    let output =
        format::render_file(8, Path::new("README.md"), Path::new(""), markdown_sample());
    insta::assert_snapshot!(output);
}

// Level 9: type body expansion (all types)

#[test]
fn rust_sample_level9() {
    let output = format::render_file(9, Path::new("sample.rs"), Path::new(""), rust_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn typescript_sample_level9() {
    let output = format::render_file(
        9,
        Path::new("sample.ts"),
        Path::new(""),
        typescript_sample(),
    );
    insta::assert_snapshot!(output);
}

#[test]
fn javascript_sample_level9() {
    let output = format::render_file(
        9,
        Path::new("sample.js"),
        Path::new(""),
        javascript_sample(),
    );
    insta::assert_snapshot!(output);
}

#[test]
fn tsx_sample_level9() {
    let output = format::render_file(9, Path::new("sample.tsx"), Path::new(""), tsx_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn go_sample_level9() {
    let output = format::render_file(9, Path::new("sample.go"), Path::new(""), go_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn python_sample_level9() {
    let output = format::render_file(9, Path::new("sample.py"), Path::new(""), python_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn markdown_sample_level9() {
    let output =
        format::render_file(9, Path::new("README.md"), Path::new(""), markdown_sample());
    insta::assert_snapshot!(output);
}

// Level 10: full source

#[test]
fn rust_sample_level10() {
    let output = format::render_file(10, Path::new("sample.rs"), Path::new(""), rust_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn typescript_sample_level10() {
    let output = format::render_file(
        10,
        Path::new("sample.ts"),
        Path::new(""),
        typescript_sample(),
    );
    insta::assert_snapshot!(output);
}

#[test]
fn javascript_sample_level10() {
    let output = format::render_file(
        10,
        Path::new("sample.js"),
        Path::new(""),
        javascript_sample(),
    );
    insta::assert_snapshot!(output);
}

#[test]
fn tsx_sample_level10() {
    let output = format::render_file(10, Path::new("sample.tsx"), Path::new(""), tsx_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn go_sample_level10() {
    let output = format::render_file(10, Path::new("sample.go"), Path::new(""), go_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn python_sample_level10() {
    let output = format::render_file(10, Path::new("sample.py"), Path::new(""), python_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn markdown_sample_level10() {
    let output =
        format::render_file(10, Path::new("README.md"), Path::new(""), markdown_sample());
    insta::assert_snapshot!(output);
}

// Subdirectory budget tests: running on a subdirectory within a fixture tests
// that path display and file discovery work correctly at deeper nesting levels.

budget_test!(budget_ts_pattern_types_subdir1, "ts-pattern/src/types", 15);
budget_test!(budget_ts_pattern_types_subdir2, "ts-pattern/src/types", 500);
budget_test!(budget_react_hot_toast_components_subdir1, "react-hot-toast/src/components", 10);
budget_test!(budget_react_hot_toast_components_subdir2, "react-hot-toast/src/components", 100);
budget_test!(budget_superstruct_structs_subdir1, "superstruct/src/structs", 5);
budget_test!(budget_superstruct_structs_subdir2, "superstruct/src/structs", 150);
budget_test!(budget_log_kv_subdir1, "log/src/kv", 8);
budget_test!(budget_log_kv_subdir2, "log/src/kv", 1500);
budget_test!(budget_semver_functions_subdir1, "semver/functions", 30);
budget_test!(budget_semver_functions_subdir2, "semver/functions", 100);
budget_test!(budget_neverthrow_internals_subdir1, "neverthrow/src/_internals", 5);
budget_test!(budget_neverthrow_internals_subdir2, "neverthrow/src/_internals", 70);
budget_test!(budget_ky_errors_subdir1, "ky/source/errors", 5);
budget_test!(budget_ky_errors_subdir2, "ky/source/errors", 40);
budget_test!(budget_thiserror_impl_subdir1, "thiserror/impl/src", 5);
budget_test!(budget_thiserror_impl_subdir2, "thiserror/impl/src", 700);
budget_test!(budget_semver_internal_subdir1, "semver/internal", 8);
budget_test!(budget_semver_internal_subdir2, "semver/internal", 100);
budget_test!(budget_mdbook_cli_subdir1, "mdbook/guide/src/cli", 12);
budget_test!(budget_mdbook_cli_subdir2, "mdbook/guide/src/cli", 130);
budget_test!(budget_mdbook_format_subdir1, "mdbook/guide/src/format", 20);
budget_test!(budget_mdbook_format_subdir2, "mdbook/guide/src/format", 270);
budget_test!(budget_xxhash_xxhsum_subdir1, "xxhash/xxhsum", 10);
budget_test!(budget_xxhash_xxhsum_subdir2, "xxhash/xxhsum", 20);

// Single-file rendering tests (precis accepts individual files, not just directories).

#[test]
fn single_file_rust_level1() {
    let Some(root) = fixture_path("either/src") else {
        eprintln!("skipping single_file_rust_level1: clone either fixture");
        return;
    };
    let file = root.join("lib.rs");
    let source = std::fs::read_to_string(&file).unwrap();
    let output = format::render_file(1, &file, &root, &source);
    insta::assert_snapshot!(output);
}

#[test]
fn single_file_rust_level2() {
    let Some(root) = fixture_path("either/src") else {
        eprintln!("skipping single_file_rust_level2: clone either fixture");
        return;
    };
    let file = root.join("lib.rs");
    let source = std::fs::read_to_string(&file).unwrap();
    let output = format::render_file(2, &file, &root, &source);
    insta::assert_snapshot!(output);
}

#[test]
fn single_file_typescript_level1() {
    let Some(root) = fixture_path("neverthrow/src") else {
        eprintln!("skipping single_file_typescript_level1: clone neverthrow fixture");
        return;
    };
    let file = root.join("result.ts");
    let source = std::fs::read_to_string(&file).unwrap();
    let output = format::render_file(1, &file, &root, &source);
    insta::assert_snapshot!(output);
}

#[test]
fn single_file_budget() {
    let Some(root) = fixture_path("either/src") else {
        eprintln!("skipping single_file_budget: clone either fixture");
        return;
    };
    let file = root.join("lib.rs");
    let source = std::fs::read_to_string(&file).unwrap();

    // Very large budget should give MAX_LEVEL
    let (level, _) = format::budget_level_file(usize::MAX, &file, &root, &source);
    assert_eq!(level, format::MAX_LEVEL);

    // Budget of 0 should give level 0
    let (level, _) = format::budget_level_file(0, &file, &root, &source);
    assert_eq!(level, 0);

    // Monotonicity: each level's word count <= next level's word count
    let mut prev_words = 0;
    for l in 0..=format::MAX_LEVEL {
        let output = format::render_file(l, &file, &root, &source);
        let words = format::count_words(&output);
        assert!(
            words >= prev_words,
            "Level {} ({} words) < previous ({} words)",
            l,
            words,
            prev_words,
        );
        prev_words = words;
    }
}

/// Test per-file monotonicity for a file with no extractable symbols.
/// Previously, levels 1 and 2 would return empty string for such files,
/// violating monotonicity vs level 0 which always shows the file path.
#[test]
fn monotonicity_no_symbols() {
    let source = r#"
use std::collections::HashMap;
// Just imports and comments, no symbols.
"#;
    let path = Path::new("imports_only.rs");
    let root = Path::new("");

    let mut prev_words = 0;
    for level in 0..=format::MAX_LEVEL {
        let output = format::render_file(level, path, root, source);
        let words = format::count_words(&output);
        assert!(
            words >= prev_words,
            "No-symbols file: level {} ({} words) < level {} ({} words)",
            level,
            words,
            level.saturating_sub(1),
            prev_words,
        );
        prev_words = words;
    }
}

/// Test directory-level monotonicity when some files have unreadable source (None).
/// Previously, unreadable files were included at level 0 but silently skipped at
/// levels 1+, which could cause the total word count to decrease.
#[test]
fn monotonicity_unreadable_files() {
    let readable = Path::new("lib.rs");
    let unreadable = Path::new("binary.rs");
    let root = Path::new("");
    let files = vec![readable.to_path_buf(), unreadable.to_path_buf()];
    // Second file has None source (simulating unreadable/non-UTF-8 file)
    let sources: Vec<Option<String>> = vec![
        Some("pub fn hello() {}\n".to_string()),
        None,
    ];

    let mut prev_words = 0;
    for level in 0..=format::MAX_LEVEL {
        let output = format::render_files(level, root, &files, &sources);
        let words = format::count_words(&output);
        assert!(
            words >= prev_words,
            "Unreadable files: level {} ({} words) < level {} ({} words)\nOutput:\n{}",
            level,
            words,
            level.saturating_sub(1),
            prev_words,
            output,
        );
        prev_words = words;
    }
}

/// Test the monotonicity invariant: for any file, a higher level must never
/// produce fewer words than a lower level. Tests one representative fixture per language.
#[test]
fn monotonicity_invariant() {
    let fixtures: &[(&str, &str)] = &[
        ("either", "either/src"),                        // Rust (5 files)
        ("thiserror", "thiserror/src"),                   // Rust (6 files)
        ("neverthrow", "neverthrow/src"),                 // TypeScript (3 files)
        ("ts-pattern", "ts-pattern/src"),                 // TypeScript (5 files)
        ("commander", "commander/lib"),                   // JavaScript (6 files)
        ("react-hot-toast", "react-hot-toast/src"),       // TSX (13 files)
        ("pluggy", "pluggy/src/pluggy"),                  // Python (7 files)
        ("typeguard", "typeguard/src/typeguard"),         // Python (12 files)
        ("xxhash", "xxhash"),                             // Go (8 files)
        ("go-multierror", "go-multierror"),               // Go (14 files)
        ("mdbook", "mdbook/guide/src"),                   // Markdown (35 files)
        // Root-level targets exercise depth penalties and multi-language discovery
        ("either-root", "either"),                        // Rust + Markdown (6 files)
        ("neverthrow-root", "neverthrow"),                // TS + JS + Markdown (9 files)
        ("pluggy-root", "pluggy"),                        // Python + Markdown (14 files)
        ("sonner-root", "sonner"),                        // TSX + TS + JS (26 files)
        ("commander-root", "commander"),                  // JS + Markdown (27 files)
        ("anyhow-root", "anyhow"),                        // Rust + Markdown (8 files)
        ("log-root", "log"),                              // Rust + Markdown (6 files)
        ("ts-pattern-root", "ts-pattern"),                // TS + JS + Markdown (8 files)
        ("typeguard-root", "typeguard"),                  // Python + Markdown (16 files)
        ("mdbook-root", "mdbook"),                        // Markdown + TOML (40+ files)
        ("once_cell-root", "once_cell"),                  // Rust + Markdown (10 files)
        ("thiserror-root", "thiserror"),                  // Rust + Markdown (9 files)
        ("react-hot-toast-root", "react-hot-toast"),      // TSX + Markdown (15 files)
        ("humanize-root", "humanize"),                    // Python + Markdown (18 files)
        ("tomli-root", "tomli"),                          // Python + Markdown (12 files)
        ("cmdk-root", "cmdk"),                            // TSX + Markdown (6 files)
        ("debug-root", "debug"),                          // JS + Markdown (6 files)
        ("dotenv-root", "dotenv"),                        // JS + Markdown (5 files)
        ("ini-root", "ini"),                              // JS + Markdown (5 files)
        ("input-otp-root", "input-otp"),                  // TSX + TS + Markdown (16 files)
        ("ky-root", "ky"),                                // TS + Markdown (8 files)
        ("mitt-root", "mitt"),                            // TS + Markdown (5 files)
        ("python-dotenv-root", "python-dotenv"),          // Python + Markdown (10 files)
        ("semver-root", "semver"),                        // Go + Markdown (12 files)
        ("superstruct-root", "superstruct"),              // TS + Markdown (16 files)
        ("vaul-root", "vaul"),                            // TSX + Markdown (12 files)
    ];
    let mut tested_files = 0;
    for (name, subpath) in fixtures {
        let Some(root) = fixture_path(subpath) else {
            continue;
        };
        let files = precis::walk::discover_source_files(&root);
        for file in &files {
            let source = std::fs::read_to_string(file).unwrap();
            let relative = file.strip_prefix(&root).unwrap_or(file);
            tested_files += 1;
            let mut prev_words = 0;
            for level in 0..=format::MAX_LEVEL {
                let output = format::render_file(level, file, &root, &source);
                let words = format::count_words(&output);
                assert!(
                    words >= prev_words,
                    "Monotonicity violation in {} file {}: level {} ({} words) < level {} ({} words)",
                    name,
                    relative.display(),
                    level,
                    words,
                    level.saturating_sub(1),
                    prev_words,
                );
                prev_words = words;
            }
        }
    }
    assert!(
        tested_files > 0,
        "No fixture files available for monotonicity test"
    );
}

/// Test the budget binary search with synthetic cost functions (no parsing).
#[test]
fn budget_algorithm() {
    let costs: [usize; 11] = [5, 8, 10, 25, 40, 60, 90, 120, 200, 350, 500];
    let cost = |level: u8| costs[level as usize];

    // Extremes
    assert_eq!(format::search_level(0, cost), 0);
    assert_eq!(format::search_level(usize::MAX, cost), format::MAX_LEVEL);

    // Exact boundaries: budget matching level N's cost should select >= N
    for level in 0..=format::MAX_LEVEL {
        let selected = format::search_level(costs[level as usize], cost);
        assert!(
            selected >= level,
            "budget={} selected level {} (expected >= {})",
            costs[level as usize],
            selected,
            level
        );
    }

    // Off-by-one: budget one below level N's cost should select < N
    for level in 1..=format::MAX_LEVEL {
        let selected = format::search_level(costs[level as usize] - 1, cost);
        assert!(
            selected < level,
            "budget={} selected level {} (expected < {})",
            costs[level as usize] - 1,
            selected,
            level
        );
    }

    // Flat region: levels with equal cost should all be reachable
    let flat_costs: [usize; 11] = [5, 10, 10, 10, 10, 10, 10, 10, 10, 10, 50];
    let flat_cost = |level: u8| flat_costs[level as usize];
    assert_eq!(format::search_level(10, flat_cost), 9);
    assert_eq!(format::search_level(9, flat_cost), 0);
}

/// Sanity check: directory-level budget_level works at extremes.
#[test]
fn budget_level_sanity() {
    let fixtures: &[&str] = &["either/src", "neverthrow/src"];
    let mut tested = 0;
    for subpath in fixtures {
        let Some(root) = fixture_path(subpath) else {
            continue;
        };
        tested += 1;
        let files = walk::discover_source_files(&root);
        let sources = format::read_sources(&files);
        assert_eq!(format::budget_level(0, &root, &files, &sources).0, 0);
        assert_eq!(
            format::budget_level(usize::MAX, &root, &files, &sources).0,
            format::MAX_LEVEL
        );
    }
    assert!(tested > 0, "No fixtures available for budget sanity test");
}

// Budget-based snapshot tests: given a word budget, snapshot the end-to-end output.
// These test the full pipeline: budget -> level selection -> rendering -> output.

/// Helper: render a fixture with a word budget and return output with metadata header.
fn render_with_budget(subpath: &str, budget: usize) -> Option<String> {
    let root = fixture_path(subpath)?;
    let files = walk::discover_source_files(&root);
    let sources = format::read_sources(&files);
    let (level, all_symbols) = format::budget_level(budget, &root, &files, &sources);
    let output =
        format::render_files_with_symbols(level, &root, &files, &sources, &all_symbols);
    let words = format::count_words(&output);
    Some(format!(
        "budget: {} → level {} ({} words)\n\n{}",
        budget, level, words, output
    ))
}

budget_test!(budget_mitt_level0, "mitt/src", 10);
budget_test!(budget_mitt_level1, "mitt/src", 50);
budget_test!(budget_mitt_level2, "mitt/src", 100);
budget_test!(budget_mitt_level3, "mitt/src", 300);
budget_test!(budget_mitt_level5, "mitt/src", 5000);
budget_test!(budget_ini_level0, "ini/lib", 5);
budget_test!(budget_ini_level1, "ini/lib", 20);
budget_test!(budget_ini_level3, "ini/lib", 50);
budget_test!(budget_neverthrow_level0, "neverthrow/src", 100);
budget_test!(budget_neverthrow_level1, "neverthrow/src", 500);

budget_test!(budget_either_level0, "either/src", 3);
budget_test!(budget_either_level1, "either/src", 1000);
budget_test!(budget_either_level3, "either/src", 5000);
budget_test!(budget_either_level4, "either/src", 6000);

budget_test!(budget_pluggy_level0, "pluggy/src/pluggy", 5);
budget_test!(budget_pluggy_level1, "pluggy/src/pluggy", 300);
budget_test!(budget_pluggy_level2, "pluggy/src/pluggy", 1000);
budget_test!(budget_pluggy_level3, "pluggy/src/pluggy", 2000);

budget_test!(budget_sonner_level0, "sonner/src", 3);
budget_test!(budget_sonner_level1, "sonner/src", 180);
budget_test!(budget_sonner_level3, "sonner/src", 500);
budget_test!(budget_sonner_level4, "sonner/src", 1900);

budget_test!(budget_mdbook_level0, "mdbook/guide/src", 30);
budget_test!(budget_mdbook_level1, "mdbook/guide/src", 555);
budget_test!(budget_mdbook_level3, "mdbook/guide/src", 6000);
budget_test!(budget_mdbook_level4, "mdbook/guide/src", 17200);
budget_test!(budget_mdbook_level5, "mdbook/guide/src", 20000);

budget_test!(budget_go_multierror_level0, "go-multierror", 5);
budget_test!(budget_go_multierror_level1, "go-multierror", 120);
budget_test!(budget_go_multierror_level3, "go-multierror", 900);
budget_test!(budget_go_multierror_level4, "go-multierror", 1800);

budget_test!(budget_xxhash_level0, "xxhash", 5);
budget_test!(budget_xxhash_level1, "xxhash", 150);
budget_test!(budget_xxhash_level3, "xxhash", 750);
budget_test!(budget_xxhash_level4, "xxhash", 1100);

budget_test!(budget_color_level0, "color", 3);
budget_test!(budget_color_level1, "color", 420);
budget_test!(budget_color_level3, "color", 2800);
budget_test!(budget_color_level4, "color", 3250);

budget_test!(budget_go_version_level0, "go-version", 3);
budget_test!(budget_go_version_level1, "go-version", 300);
budget_test!(budget_go_version_level3, "go-version", 1350);
budget_test!(budget_go_version_level4, "go-version", 1850);

budget_test!(budget_structs_level0, "structs", 3);
budget_test!(budget_structs_level1, "structs", 180);
budget_test!(budget_structs_level3, "structs", 2100);
budget_test!(budget_structs_level4, "structs", 2700);

budget_test!(budget_typeguard_level0, "typeguard/src/typeguard", 10);
budget_test!(budget_typeguard_level1, "typeguard/src/typeguard", 400);
budget_test!(budget_typeguard_level3, "typeguard/src/typeguard", 3100);
budget_test!(budget_typeguard_level4, "typeguard/src/typeguard", 7100);

budget_test!(budget_anyhow_level0, "anyhow/src", 5);
budget_test!(budget_anyhow_level1, "anyhow/src", 1200);
budget_test!(budget_anyhow_level3, "anyhow/src", 5000);
budget_test!(budget_anyhow_level4, "anyhow/src", 6500);

budget_test!(budget_ts_pattern_level0, "ts-pattern/src", 5);
budget_test!(budget_ts_pattern_level1, "ts-pattern/src", 800);
budget_test!(budget_ts_pattern_level2, "ts-pattern/src", 11000);
budget_test!(budget_ts_pattern_level4, "ts-pattern/src", 14000);

budget_test!(budget_tomli_level0, "tomli/src/tomli", 3);
budget_test!(budget_tomli_level1, "tomli/src/tomli", 200);
budget_test!(budget_tomli_level3, "tomli/src/tomli", 920);
budget_test!(budget_tomli_level5, "tomli/src/tomli", 1500);

budget_test!(budget_log_level0, "log/src", 3);
budget_test!(budget_log_level1, "log/src", 2000);
budget_test!(budget_log_level3, "log/src", 5000);
budget_test!(budget_log_level4, "log/src", 10000);

budget_test!(budget_thiserror_level0, "thiserror/src", 3);
budget_test!(budget_thiserror_level1, "thiserror/src", 280);
budget_test!(budget_thiserror_level4, "thiserror/src", 440);
budget_test!(budget_thiserror_level6, "thiserror/src", 1900);

budget_test!(budget_once_cell_level0, "once_cell/src", 3);
budget_test!(budget_once_cell_level1, "once_cell/src", 1000);
budget_test!(budget_once_cell_level3, "once_cell/src", 3000);
budget_test!(budget_once_cell_level5, "once_cell/src", 6200);

budget_test!(budget_humanize_level0, "humanize/src/humanize", 3);
budget_test!(budget_humanize_level1, "humanize/src/humanize", 100);
budget_test!(budget_humanize_level2, "humanize/src/humanize", 500);
budget_test!(budget_humanize_level5, "humanize/src/humanize", 3400);

budget_test!(budget_python_dotenv_level0, "python-dotenv/src/dotenv", 5);
budget_test!(budget_python_dotenv_level1, "python-dotenv/src/dotenv", 200);
budget_test!(budget_python_dotenv_level3, "python-dotenv/src/dotenv", 1300);
budget_test!(budget_python_dotenv_level5, "python-dotenv/src/dotenv", 2000);

budget_test!(budget_semver_level0, "semver/classes", 3);
budget_test!(budget_semver_level1, "semver/classes", 100);
budget_test!(budget_semver_level4, "semver/classes", 260);
budget_test!(budget_semver_level6, "semver/classes", 4600);

budget_test!(budget_cmdk_level0, "cmdk/cmdk/src", 3);
budget_test!(budget_cmdk_level1, "cmdk/cmdk/src", 120);
budget_test!(budget_cmdk_level4, "cmdk/cmdk/src", 1100);
budget_test!(budget_cmdk_level6, "cmdk/cmdk/src", 2200);

budget_test!(budget_react_hot_toast_level0, "react-hot-toast/src", 5);
budget_test!(budget_react_hot_toast_level1, "react-hot-toast/src", 200);
budget_test!(budget_react_hot_toast_level4, "react-hot-toast/src", 700);
budget_test!(budget_react_hot_toast_level5, "react-hot-toast/src", 1000);

budget_test!(budget_superstruct_level0, "superstruct/src", 5);
budget_test!(budget_superstruct_level1, "superstruct/src", 300);
budget_test!(budget_superstruct_level4, "superstruct/src", 3600);
budget_test!(budget_superstruct_level5, "superstruct/src", 4000);

budget_test!(budget_dotenv_level0, "dotenv/lib", 3);
budget_test!(budget_dotenv_level1, "dotenv/lib", 80);
budget_test!(budget_dotenv_level4, "dotenv/lib", 160);
budget_test!(budget_dotenv_level5, "dotenv/lib", 600);

budget_test!(budget_commander_level0, "commander/lib", 3);
budget_test!(budget_commander_level1, "commander/lib", 400);
budget_test!(budget_commander_level3, "commander/lib", 2500);
budget_test!(budget_commander_level4, "commander/lib", 7500);

budget_test!(budget_ky_level0, "ky/source", 10);
budget_test!(budget_ky_level1, "ky/source", 350);
budget_test!(budget_ky_level4, "ky/source", 7000);
budget_test!(budget_ky_level6, "ky/source", 12500);

budget_test!(budget_vaul_level0, "vaul/src", 5);
budget_test!(budget_vaul_level1, "vaul/src", 250);
budget_test!(budget_vaul_level4, "vaul/src", 1300);
budget_test!(budget_vaul_level5, "vaul/src", 1500);

budget_test!(budget_debug_level0, "debug/src", 3);
budget_test!(budget_debug_level1, "debug/src", 35);
budget_test!(budget_debug_level3, "debug/src", 50);
budget_test!(budget_debug_level5, "debug/src", 300);

budget_test!(budget_input_otp_level0, "input-otp/packages/input-otp/src", 5);
budget_test!(budget_input_otp_level1, "input-otp/packages/input-otp/src", 60);
budget_test!(budget_input_otp_level4, "input-otp/packages/input-otp/src", 200);
budget_test!(budget_input_otp_level5, "input-otp/packages/input-otp/src", 250);

// Root-level budget tests: exercise depth penalties, multi-language discovery,
// and file filtering at the repo root — the most common real-world use case.

budget_test!(budget_either_root_level0, "either", 6);
budget_test!(budget_either_root_level1, "either", 2000);
budget_test!(budget_either_root_level3, "either", 5200);
budget_test!(budget_either_root_level5, "either", 6200);

budget_test!(budget_neverthrow_root_level0, "neverthrow", 10);
budget_test!(budget_neverthrow_root_level1, "neverthrow", 1500);
budget_test!(budget_neverthrow_root_level3, "neverthrow", 4000);
budget_test!(budget_neverthrow_root_level4, "neverthrow", 6000);

budget_test!(budget_pluggy_root_level0, "pluggy", 50);
budget_test!(budget_pluggy_root_level1, "pluggy", 100);
budget_test!(budget_pluggy_root_level3, "pluggy", 2000);
budget_test!(budget_pluggy_root_level4, "pluggy", 3200);

budget_test!(budget_sonner_root_level0, "sonner", 30);
budget_test!(budget_sonner_root_level3, "sonner", 800);
budget_test!(budget_sonner_root_level4, "sonner", 2000);
budget_test!(budget_sonner_root_level5, "sonner", 5000);

budget_test!(budget_commander_root_level0, "commander", 30);
budget_test!(budget_commander_root_level1, "commander", 2500);
budget_test!(budget_commander_root_level2, "commander", 5000);
budget_test!(budget_commander_root_level4, "commander", 20000);

budget_test!(budget_anyhow_root_level0, "anyhow", 15);
budget_test!(budget_anyhow_root_level1, "anyhow", 1300);
budget_test!(budget_anyhow_root_level3, "anyhow", 5500);
budget_test!(budget_anyhow_root_level4, "anyhow", 7000);

budget_test!(budget_log_root_level0, "log", 15);
budget_test!(budget_log_root_level1, "log", 2000);
budget_test!(budget_log_root_level3, "log", 7000);
budget_test!(budget_log_root_level4, "log", 14000);

budget_test!(budget_ts_pattern_root_level0, "ts-pattern", 25);
budget_test!(budget_ts_pattern_root_level1, "ts-pattern", 800);
budget_test!(budget_ts_pattern_root_level2, "ts-pattern", 3000);
budget_test!(budget_ts_pattern_root_level4, "ts-pattern", 18000);

budget_test!(budget_typeguard_root_level0, "typeguard", 15);
budget_test!(budget_typeguard_root_level2, "typeguard", 500);
budget_test!(budget_typeguard_root_level3, "typeguard", 2000);
budget_test!(budget_typeguard_root_level4, "typeguard", 3000);

budget_test!(budget_mdbook_root_level0, "mdbook", 130);
budget_test!(budget_mdbook_root_level1, "mdbook", 1000);
budget_test!(budget_mdbook_root_level3, "mdbook", 5000);
budget_test!(budget_mdbook_root_level4, "mdbook", 22000);

budget_test!(budget_once_cell_root_level0, "once_cell", 15);
budget_test!(budget_once_cell_root_level1, "once_cell", 1500);
budget_test!(budget_once_cell_root_level2, "once_cell", 3000);
budget_test!(budget_once_cell_root_level3, "once_cell", 6000);
budget_test!(budget_once_cell_root_level5, "once_cell", 10000);

budget_test!(budget_thiserror_root_level0, "thiserror", 30);
budget_test!(budget_thiserror_root_level1, "thiserror", 500);
budget_test!(budget_thiserror_root_level2, "thiserror", 1500);
budget_test!(budget_thiserror_root_level4, "thiserror", 2500);
budget_test!(budget_thiserror_root_level5, "thiserror", 5000);

budget_test!(budget_react_hot_toast_root_level0, "react-hot-toast", 50);
budget_test!(budget_react_hot_toast_root_level1, "react-hot-toast", 200);
budget_test!(budget_react_hot_toast_root_level2, "react-hot-toast", 1000);
budget_test!(budget_react_hot_toast_root_level5, "react-hot-toast", 2000);

budget_test!(budget_humanize_root_level0, "humanize", 20);
budget_test!(budget_humanize_root_level1, "humanize", 120);
budget_test!(budget_humanize_root_level2, "humanize", 600);
budget_test!(budget_humanize_root_level4, "humanize", 3500);

budget_test!(budget_tomli_root_level0, "tomli", 15);
budget_test!(budget_tomli_root_level1, "tomli", 250);
budget_test!(budget_tomli_root_level2, "tomli", 1500);
budget_test!(budget_tomli_root_level4, "tomli", 3000);

budget_test!(budget_cmdk_root_level0, "cmdk", 25);
budget_test!(budget_cmdk_root_level1, "cmdk", 200);
budget_test!(budget_cmdk_root_level2, "cmdk", 1000);
budget_test!(budget_cmdk_root_level6, "cmdk", 4300);

budget_test!(budget_debug_root_level0, "debug", 15);
budget_test!(budget_debug_root_level2, "debug", 200);
budget_test!(budget_debug_root_level5, "debug", 1500);
budget_test!(budget_debug_root_level7, "debug", 5000);

budget_test!(budget_dotenv_root_level0, "dotenv", 15);
budget_test!(budget_dotenv_root_level1, "dotenv", 650);
budget_test!(budget_dotenv_root_level5, "dotenv", 3000);
budget_test!(budget_dotenv_root_level7, "dotenv", 10000);

budget_test!(budget_ini_root_level0, "ini", 10);
budget_test!(budget_ini_root_level1, "ini", 160);
budget_test!(budget_ini_root_level5, "ini", 1000);
budget_test!(budget_ini_root_level7, "ini", 2000);

budget_test!(budget_input_otp_root_level0, "input-otp", 60);
budget_test!(budget_input_otp_root_level1, "input-otp", 200);
budget_test!(budget_input_otp_root_level3, "input-otp", 1200);
budget_test!(budget_input_otp_root_level6, "input-otp", 3700);

budget_test!(budget_ky_root_level0, "ky", 30);
budget_test!(budget_ky_root_level2, "ky", 600);
budget_test!(budget_ky_root_level4, "ky", 7400);
budget_test!(budget_ky_root_level7, "ky", 16200);

budget_test!(budget_mitt_root_level0, "mitt", 5);
budget_test!(budget_mitt_root_level1, "mitt", 100);
budget_test!(budget_mitt_root_level5, "mitt", 540);
budget_test!(budget_mitt_root_level7, "mitt", 1000);

budget_test!(budget_python_dotenv_root_level0, "python-dotenv", 15);
budget_test!(budget_python_dotenv_root_level2, "python-dotenv", 500);
budget_test!(budget_python_dotenv_root_level5, "python-dotenv", 2000);
budget_test!(budget_python_dotenv_root_level6, "python-dotenv", 4500);

budget_test!(budget_semver_root_level0, "semver", 60);
budget_test!(budget_semver_root_level1, "semver", 700);
budget_test!(budget_semver_root_level2, "semver", 2000);
budget_test!(budget_semver_root_level6, "semver", 7000);

budget_test!(budget_superstruct_root_level0, "superstruct", 35);
budget_test!(budget_superstruct_root_level1, "superstruct", 500);
budget_test!(budget_superstruct_root_level3, "superstruct", 5000);
budget_test!(budget_superstruct_root_level5, "superstruct", 7000);

budget_test!(budget_vaul_root_level0, "vaul", 20);
budget_test!(budget_vaul_root_level1, "vaul", 250);
budget_test!(budget_vaul_root_level2, "vaul", 1100);
budget_test!(budget_vaul_root_level7, "vaul", 2000);
