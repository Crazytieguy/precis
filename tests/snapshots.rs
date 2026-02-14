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

/// Generate a snapshot test that renders a fixture directory at a given level.
macro_rules! fixture_test {
    ($name:ident, $path:expr, $level:expr) => {
        #[test]
        fn $name() {
            let Some(root) = fixture_path($path) else {
                eprintln!(
                    "skipping {}: fixture not present at {}",
                    stringify!($name),
                    $path
                );
                return;
            };
            let files = walk::discover_source_files(&root);
            let sources = format::read_sources(&files);
            let output = format::render_files($level, &root, &files, &sources);
            insta::assert_snapshot!(output);
        }
    };
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
// `trimmed.ends_with('{')` was false, causing body content to leak into level 2/3 output.

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
    let output = format::render_file(2, Path::new("api.ts"), Path::new(""), source);
    insta::assert_snapshot!(output);
}

// Level 0: file paths only

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

// Level 1: symbol names (truncated)

#[test]
fn rust_sample_snapshot() {
    let output = format::render_file(1, Path::new("sample.rs"), Path::new(""), rust_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn typescript_sample_snapshot() {
    let output = format::render_file(
        1,
        Path::new("sample.ts"),
        Path::new(""),
        typescript_sample(),
    );
    insta::assert_snapshot!(output);
}

#[test]
fn javascript_sample_snapshot() {
    let output = format::render_file(
        1,
        Path::new("sample.js"),
        Path::new(""),
        javascript_sample(),
    );
    insta::assert_snapshot!(output);
}

#[test]
fn tsx_sample_snapshot() {
    let output = format::render_file(1, Path::new("sample.tsx"), Path::new(""), tsx_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn go_sample_snapshot() {
    let output = format::render_file(1, Path::new("sample.go"), Path::new(""), go_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn python_sample_snapshot() {
    let output = format::render_file(1, Path::new("sample.py"), Path::new(""), python_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn markdown_sample_snapshot() {
    let output =
        format::render_file(1, Path::new("README.md"), Path::new(""), markdown_sample());
    insta::assert_snapshot!(output);
}

// Level 2: full signature lines

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

// Level 3: signature lines with doc comments

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

// Level 4: signatures with doc comments + type bodies (struct/enum/trait/interface)

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

// Level 5: full source

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

// Fixture-based snapshot tests (level 1: symbol names).

fixture_test!(fixture_either, "either/src", 1);
fixture_test!(fixture_neverthrow, "neverthrow/src", 1);
fixture_test!(fixture_semver, "semver/classes", 1);
fixture_test!(fixture_cmdk, "cmdk/cmdk/src", 1);
fixture_test!(fixture_ts_pattern, "ts-pattern/src", 1);
fixture_test!(fixture_anyhow, "anyhow/src", 1);
fixture_test!(fixture_once_cell, "once_cell/src", 1);
fixture_test!(fixture_react_hot_toast, "react-hot-toast/src", 1);
fixture_test!(fixture_superstruct, "superstruct/src", 1);
fixture_test!(fixture_dotenv, "dotenv/lib", 1);
fixture_test!(fixture_commander, "commander/lib", 1);
fixture_test!(fixture_thiserror, "thiserror/src", 1);
fixture_test!(fixture_sonner, "sonner/src", 1);
fixture_test!(fixture_mitt, "mitt/src", 1);
fixture_test!(fixture_debug, "debug/src", 1);
fixture_test!(fixture_log, "log/src", 1);
fixture_test!(fixture_ky, "ky/source", 1);
fixture_test!(fixture_ini, "ini/lib", 1);
fixture_test!(fixture_vaul, "vaul/src", 1);
fixture_test!(fixture_input_otp, "input-otp/packages/input-otp/src", 1);
fixture_test!(fixture_pluggy, "pluggy/src/pluggy", 1);
fixture_test!(fixture_tomli, "tomli/src/tomli", 1);
fixture_test!(fixture_humanize, "humanize/src/humanize", 1);
fixture_test!(fixture_python_dotenv, "python-dotenv/src/dotenv", 1);
fixture_test!(fixture_typeguard, "typeguard/src/typeguard", 1);
fixture_test!(fixture_mdbook, "mdbook/guide/src", 1);
fixture_test!(fixture_go_multierror, "go-multierror", 1);
fixture_test!(fixture_xxhash, "xxhash", 1);

// Fixture-based snapshot tests (level 2: full signature lines).

fixture_test!(fixture_either_level2, "either/src", 2);
fixture_test!(fixture_neverthrow_level2, "neverthrow/src", 2);
fixture_test!(fixture_semver_level2, "semver/classes", 2);
fixture_test!(fixture_cmdk_level2, "cmdk/cmdk/src", 2);
fixture_test!(fixture_ts_pattern_level2, "ts-pattern/src", 2);
fixture_test!(fixture_anyhow_level2, "anyhow/src", 2);
fixture_test!(fixture_once_cell_level2, "once_cell/src", 2);
fixture_test!(fixture_react_hot_toast_level2, "react-hot-toast/src", 2);
fixture_test!(fixture_superstruct_level2, "superstruct/src", 2);
fixture_test!(fixture_dotenv_level2, "dotenv/lib", 2);
fixture_test!(fixture_commander_level2, "commander/lib", 2);
fixture_test!(fixture_thiserror_level2, "thiserror/src", 2);
fixture_test!(fixture_sonner_level2, "sonner/src", 2);
fixture_test!(fixture_mitt_level2, "mitt/src", 2);
fixture_test!(fixture_debug_level2, "debug/src", 2);
fixture_test!(fixture_log_level2, "log/src", 2);
fixture_test!(fixture_ky_level2, "ky/source", 2);
fixture_test!(fixture_ini_level2, "ini/lib", 2);
fixture_test!(fixture_vaul_level2, "vaul/src", 2);
fixture_test!(
    fixture_input_otp_level2,
    "input-otp/packages/input-otp/src",
    2
);
fixture_test!(fixture_pluggy_level2, "pluggy/src/pluggy", 2);
fixture_test!(fixture_tomli_level2, "tomli/src/tomli", 2);
fixture_test!(fixture_humanize_level2, "humanize/src/humanize", 2);
fixture_test!(fixture_python_dotenv_level2, "python-dotenv/src/dotenv", 2);
fixture_test!(fixture_typeguard_level2, "typeguard/src/typeguard", 2);
fixture_test!(fixture_mdbook_level2, "mdbook/guide/src", 2);
fixture_test!(fixture_go_multierror_level2, "go-multierror", 2);
fixture_test!(fixture_xxhash_level2, "xxhash", 2);

// Fixture-based snapshot tests (level 3: signatures with doc comments).

fixture_test!(fixture_either_level3, "either/src", 3);
fixture_test!(fixture_neverthrow_level3, "neverthrow/src", 3);
fixture_test!(fixture_semver_level3, "semver/classes", 3);
fixture_test!(fixture_cmdk_level3, "cmdk/cmdk/src", 3);
fixture_test!(fixture_ts_pattern_level3, "ts-pattern/src", 3);
fixture_test!(fixture_anyhow_level3, "anyhow/src", 3);
fixture_test!(fixture_once_cell_level3, "once_cell/src", 3);
fixture_test!(fixture_react_hot_toast_level3, "react-hot-toast/src", 3);
fixture_test!(fixture_superstruct_level3, "superstruct/src", 3);
fixture_test!(fixture_dotenv_level3, "dotenv/lib", 3);
fixture_test!(fixture_commander_level3, "commander/lib", 3);
fixture_test!(fixture_thiserror_level3, "thiserror/src", 3);
fixture_test!(fixture_sonner_level3, "sonner/src", 3);
fixture_test!(fixture_mitt_level3, "mitt/src", 3);
fixture_test!(fixture_debug_level3, "debug/src", 3);
fixture_test!(fixture_log_level3, "log/src", 3);
fixture_test!(fixture_ky_level3, "ky/source", 3);
fixture_test!(fixture_ini_level3, "ini/lib", 3);
fixture_test!(fixture_vaul_level3, "vaul/src", 3);
fixture_test!(
    fixture_input_otp_level3,
    "input-otp/packages/input-otp/src",
    3
);
fixture_test!(fixture_pluggy_level3, "pluggy/src/pluggy", 3);
fixture_test!(fixture_tomli_level3, "tomli/src/tomli", 3);
fixture_test!(fixture_humanize_level3, "humanize/src/humanize", 3);
fixture_test!(fixture_python_dotenv_level3, "python-dotenv/src/dotenv", 3);
fixture_test!(fixture_typeguard_level3, "typeguard/src/typeguard", 3);
fixture_test!(fixture_mdbook_level3, "mdbook/guide/src", 3);
fixture_test!(fixture_go_multierror_level3, "go-multierror", 3);
fixture_test!(fixture_xxhash_level3, "xxhash", 3);

// Fixture-based snapshot tests (level 4: type bodies expanded).

fixture_test!(fixture_either_level4, "either/src", 4);
fixture_test!(fixture_neverthrow_level4, "neverthrow/src", 4);
fixture_test!(fixture_semver_level4, "semver/classes", 4);
fixture_test!(fixture_cmdk_level4, "cmdk/cmdk/src", 4);
fixture_test!(fixture_ts_pattern_level4, "ts-pattern/src", 4);
fixture_test!(fixture_anyhow_level4, "anyhow/src", 4);
fixture_test!(fixture_once_cell_level4, "once_cell/src", 4);
fixture_test!(fixture_react_hot_toast_level4, "react-hot-toast/src", 4);
fixture_test!(fixture_superstruct_level4, "superstruct/src", 4);
fixture_test!(fixture_dotenv_level4, "dotenv/lib", 4);
fixture_test!(fixture_commander_level4, "commander/lib", 4);
fixture_test!(fixture_thiserror_level4, "thiserror/src", 4);
fixture_test!(fixture_sonner_level4, "sonner/src", 4);
fixture_test!(fixture_mitt_level4, "mitt/src", 4);
fixture_test!(fixture_debug_level4, "debug/src", 4);
fixture_test!(fixture_log_level4, "log/src", 4);
fixture_test!(fixture_ky_level4, "ky/source", 4);
fixture_test!(fixture_ini_level4, "ini/lib", 4);
fixture_test!(fixture_vaul_level4, "vaul/src", 4);
fixture_test!(
    fixture_input_otp_level4,
    "input-otp/packages/input-otp/src",
    4
);
fixture_test!(fixture_pluggy_level4, "pluggy/src/pluggy", 4);
fixture_test!(fixture_tomli_level4, "tomli/src/tomli", 4);
fixture_test!(fixture_humanize_level4, "humanize/src/humanize", 4);
fixture_test!(fixture_python_dotenv_level4, "python-dotenv/src/dotenv", 4);
fixture_test!(fixture_typeguard_level4, "typeguard/src/typeguard", 4);
fixture_test!(fixture_mdbook_level4, "mdbook/guide/src", 4);
fixture_test!(fixture_go_multierror_level4, "go-multierror", 4);
fixture_test!(fixture_xxhash_level4, "xxhash", 4);

// Subdirectory tests: running on a subdirectory within a fixture tests
// that path display and file discovery work correctly at deeper nesting levels.

fixture_test!(fixture_ts_pattern_types_subdir, "ts-pattern/src/types", 1);
fixture_test!(
    fixture_ts_pattern_types_subdir_level2,
    "ts-pattern/src/types",
    2
);
fixture_test!(
    fixture_react_hot_toast_components_subdir,
    "react-hot-toast/src/components",
    1
);
fixture_test!(
    fixture_react_hot_toast_components_subdir_level2,
    "react-hot-toast/src/components",
    2
);
fixture_test!(
    fixture_superstruct_structs_subdir,
    "superstruct/src/structs",
    1
);
fixture_test!(
    fixture_superstruct_structs_subdir_level2,
    "superstruct/src/structs",
    2
);
fixture_test!(fixture_log_kv_subdir, "log/src/kv", 1);
fixture_test!(fixture_log_kv_subdir_level2, "log/src/kv", 2);
fixture_test!(fixture_semver_functions_subdir, "semver/functions", 1);
fixture_test!(
    fixture_semver_functions_subdir_level2,
    "semver/functions",
    2
);
fixture_test!(
    fixture_neverthrow_internals_subdir,
    "neverthrow/src/_internals",
    1
);
fixture_test!(
    fixture_neverthrow_internals_subdir_level2,
    "neverthrow/src/_internals",
    2
);
fixture_test!(fixture_ky_errors_subdir, "ky/source/errors", 1);
fixture_test!(fixture_ky_errors_subdir_level2, "ky/source/errors", 2);
fixture_test!(fixture_thiserror_impl_subdir, "thiserror/impl/src", 1);
fixture_test!(
    fixture_thiserror_impl_subdir_level2,
    "thiserror/impl/src",
    2
);
fixture_test!(fixture_semver_internal_subdir, "semver/internal", 1);
fixture_test!(fixture_semver_internal_subdir_level2, "semver/internal", 2);
fixture_test!(fixture_mdbook_cli_subdir, "mdbook/guide/src/cli", 1);
fixture_test!(fixture_mdbook_cli_subdir_level2, "mdbook/guide/src/cli", 2);
fixture_test!(fixture_mdbook_format_subdir, "mdbook/guide/src/format", 1);
fixture_test!(
    fixture_mdbook_format_subdir_level2,
    "mdbook/guide/src/format",
    2
);
fixture_test!(fixture_xxhash_xxhsum_subdir, "xxhash/xxhsum", 1);
fixture_test!(fixture_xxhash_xxhsum_subdir_level2, "xxhash/xxhsum", 2);

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
/// produce fewer words than a lower level. Tests per-file across all fixtures.
#[test]
fn monotonicity_invariant() {
    let fixtures: &[(&str, &str)] = &[
        ("either", "either/src"),
        ("anyhow", "anyhow/src"),
        ("once_cell", "once_cell/src"),
        ("thiserror", "thiserror/src"),
        ("log", "log/src"),
        ("neverthrow", "neverthrow/src"),
        ("ts-pattern", "ts-pattern/src"),
        ("superstruct", "superstruct/src"),
        ("mitt", "mitt/src"),
        ("ky", "ky/source"),
        ("semver", "semver/classes"),
        ("dotenv", "dotenv/lib"),
        ("commander", "commander/lib"),
        ("debug", "debug/src"),
        ("ini", "ini/lib"),
        ("cmdk", "cmdk/cmdk/src"),
        ("react-hot-toast", "react-hot-toast/src"),
        ("sonner", "sonner/src"),
        ("vaul", "vaul/src"),
        ("input-otp", "input-otp/packages/input-otp/src"),
        ("pluggy", "pluggy/src/pluggy"),
        ("tomli", "tomli/src/tomli"),
        ("humanize", "humanize/src/humanize"),
        ("python-dotenv", "python-dotenv/src/dotenv"),
        ("typeguard", "typeguard/src/typeguard"),
        ("mdbook", "mdbook/guide/src"),
        ("go-multierror", "go-multierror"),
        ("xxhash", "xxhash"),
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
    let costs: [usize; 6] = [5, 10, 25, 60, 120, 200];
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
    let flat_costs: [usize; 6] = [5, 10, 10, 10, 10, 50];
    let flat_cost = |level: u8| flat_costs[level as usize];
    assert_eq!(format::search_level(10, flat_cost), 4);
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

// Rust budget tests (either: 0→5, 1→692, 2→1314, 3→4955, 4→5025, 5→8621 words)
budget_test!(budget_either_level0, "either/src", 3);
budget_test!(budget_either_level1, "either/src", 1000);
budget_test!(budget_either_level3, "either/src", 5000);
budget_test!(budget_either_level4, "either/src", 6000);

// Python budget tests (pluggy: 0→7, 1→293, 2→650, 3→1753, 4→5957, 5→7397 words)
budget_test!(budget_pluggy_level0, "pluggy/src/pluggy", 5);
budget_test!(budget_pluggy_level1, "pluggy/src/pluggy", 300);
budget_test!(budget_pluggy_level2, "pluggy/src/pluggy", 1000);
budget_test!(budget_pluggy_level3, "pluggy/src/pluggy", 2000);

// TSX budget tests (sonner: 0→5, 1→174, 2→495, 3→495, 4→1837, 5→5929 words)
budget_test!(budget_sonner_level0, "sonner/src", 3);
budget_test!(budget_sonner_level1, "sonner/src", 180);
budget_test!(budget_sonner_level3, "sonner/src", 500);
budget_test!(budget_sonner_level4, "sonner/src", 1900);

// Markdown budget tests (mdbook: 0→37, 1→553, 2→560, 3→5061, 4→17096, 5→17253 words)
budget_test!(budget_mdbook_level0, "mdbook/guide/src", 30);
budget_test!(budget_mdbook_level1, "mdbook/guide/src", 555);
budget_test!(budget_mdbook_level3, "mdbook/guide/src", 6000);
budget_test!(budget_mdbook_level4, "mdbook/guide/src", 17200);
budget_test!(budget_mdbook_level5, "mdbook/guide/src", 20000);

// Go budget tests (go-multierror: 0→9, 1→109, 2→169, 3→890, 4→1787, 5→2543 words)
budget_test!(budget_go_multierror_level0, "go-multierror", 5);
budget_test!(budget_go_multierror_level1, "go-multierror", 120);
budget_test!(budget_go_multierror_level3, "go-multierror", 900);
budget_test!(budget_go_multierror_level4, "go-multierror", 1800);

// Go budget tests (xxhash: 0→8, 1→141, 2→353, 3→728, 4→1025, 5→2598 words)
budget_test!(budget_xxhash_level0, "xxhash", 5);
budget_test!(budget_xxhash_level1, "xxhash", 150);
budget_test!(budget_xxhash_level3, "xxhash", 750);
budget_test!(budget_xxhash_level4, "xxhash", 1100);
