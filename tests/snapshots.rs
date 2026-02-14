use std::path::Path;
use symbols::format;

/// Helper to get the path to a test fixture. Returns None if the fixture isn't cloned.
fn fixture_path(name: &str) -> Option<std::path::PathBuf> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("test/fixtures")
        .join(name);
    if path.exists() {
        Some(path)
    } else {
        None
    }
}

#[test]
fn rust_sample_snapshot() {
    let source = r#"
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
"#;
    let output = format::format_file_symbols(Path::new("sample.rs"), Path::new(""), source);
    insta::assert_snapshot!(output);
}

#[test]
fn typescript_sample_snapshot() {
    let source = r#"
export function processItems(items: string[]): number {
    return items.length;
}

function helper(): void {}

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
"#;
    let output = format::format_file_symbols(Path::new("sample.ts"), Path::new(""), source);
    insta::assert_snapshot!(output);
}

#[test]
fn javascript_sample_snapshot() {
    let source = r#"
export function processItems(items) {
    return items.length;
}

function helper() {}

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
"#;
    let output = format::format_file_symbols(Path::new("sample.js"), Path::new(""), source);
    insta::assert_snapshot!(output);
}

// Fixture-based snapshot tests.
// Clone fixtures with: git clone --depth 1 <url> test/fixtures/<name>
// Tests are skipped if the fixture directory is not present.

#[test]
fn fixture_either() {
    let Some(root) = fixture_path("either/src") else {
        eprintln!("skipping fixture_either: clone with `git clone --depth 1 https://github.com/rayon-rs/either.git test/fixtures/either`");
        return;
    };
    let output = format::format_directory(&root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_neverthrow() {
    let Some(root) = fixture_path("neverthrow/src") else {
        eprintln!("skipping fixture_neverthrow: clone with `git clone --depth 1 https://github.com/supermacro/neverthrow.git test/fixtures/neverthrow`");
        return;
    };
    let output = format::format_directory(&root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_semver() {
    let Some(root) = fixture_path("semver/classes") else {
        eprintln!("skipping fixture_semver: clone with `git clone --depth 1 https://github.com/npm/node-semver.git test/fixtures/semver`");
        return;
    };
    let output = format::format_directory(&root);
    insta::assert_snapshot!(output);
}
