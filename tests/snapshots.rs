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

#[test]
fn rust_sample_level11() {
    let output = format::render_file(11, Path::new("sample.rs"), Path::new(""), rust_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn typescript_sample_level11() {
    let output = format::render_file(
        11,
        Path::new("sample.ts"),
        Path::new(""),
        typescript_sample(),
    );
    insta::assert_snapshot!(output);
}

#[test]
fn javascript_sample_level11() {
    let output = format::render_file(
        11,
        Path::new("sample.js"),
        Path::new(""),
        javascript_sample(),
    );
    insta::assert_snapshot!(output);
}

#[test]
fn tsx_sample_level11() {
    let output = format::render_file(11, Path::new("sample.tsx"), Path::new(""), tsx_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn go_sample_level11() {
    let output = format::render_file(11, Path::new("sample.go"), Path::new(""), go_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn python_sample_level11() {
    let output = format::render_file(11, Path::new("sample.py"), Path::new(""), python_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn markdown_sample_level11() {
    let output =
        format::render_file(11, Path::new("README.md"), Path::new(""), markdown_sample());
    insta::assert_snapshot!(output);
}

// Level 12: full source (same as 11 for small samples without sig_count_penalty)

#[test]
fn rust_sample_level12() {
    let output = format::render_file(12, Path::new("sample.rs"), Path::new(""), rust_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn typescript_sample_level12() {
    let output = format::render_file(
        12,
        Path::new("sample.ts"),
        Path::new(""),
        typescript_sample(),
    );
    insta::assert_snapshot!(output);
}

#[test]
fn javascript_sample_level12() {
    let output = format::render_file(
        12,
        Path::new("sample.js"),
        Path::new(""),
        javascript_sample(),
    );
    insta::assert_snapshot!(output);
}

#[test]
fn tsx_sample_level12() {
    let output = format::render_file(12, Path::new("sample.tsx"), Path::new(""), tsx_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn go_sample_level12() {
    let output = format::render_file(12, Path::new("sample.go"), Path::new(""), go_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn python_sample_level12() {
    let output = format::render_file(12, Path::new("sample.py"), Path::new(""), python_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn markdown_sample_level12() {
    let output =
        format::render_file(12, Path::new("README.md"), Path::new(""), markdown_sample());
    insta::assert_snapshot!(output);
}

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
            let symbols = precis::parse::extract_symbols(file, &source);
            tested_files += 1;
            let mut prev_words = 0;
            for level in 0..=format::MAX_LEVEL {
                let output = format::render_file_with_symbols(level, file, &root, &source, &symbols);
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
    let costs: [usize; 13] = [5, 8, 10, 25, 40, 60, 90, 120, 200, 350, 500, 700, 900];
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
    let flat_costs: [usize; 13] = [5, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 50];
    let flat_cost = |level: u8| flat_costs[level as usize];
    assert_eq!(format::search_level(10, flat_cost), 11);
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

budget_test!(budget_mitt_src_500, "mitt/src", 500);
budget_test!(budget_mitt_src_1000, "mitt/src", 1000);
budget_test!(budget_mitt_src_2000, "mitt/src", 2000);
budget_test!(budget_mitt_src_4000, "mitt/src", 4000);
budget_test!(budget_ini_lib_500, "ini/lib", 500);
budget_test!(budget_ini_lib_1000, "ini/lib", 1000);
budget_test!(budget_ini_lib_2000, "ini/lib", 2000);
budget_test!(budget_ini_lib_4000, "ini/lib", 4000);
budget_test!(budget_neverthrow_src_500, "neverthrow/src", 500);
budget_test!(budget_neverthrow_src_1000, "neverthrow/src", 1000);
budget_test!(budget_neverthrow_src_2000, "neverthrow/src", 2000);
budget_test!(budget_neverthrow_src_4000, "neverthrow/src", 4000);
budget_test!(budget_either_src_500, "either/src", 500);
budget_test!(budget_either_src_1000, "either/src", 1000);
budget_test!(budget_either_src_2000, "either/src", 2000);
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
budget_test!(budget_go_multierror_500, "go-multierror", 500);
budget_test!(budget_go_multierror_1000, "go-multierror", 1000);
budget_test!(budget_go_multierror_2000, "go-multierror", 2000);
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

// Root-level budget tests: exercise depth penalties, multi-language discovery,
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
