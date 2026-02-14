use std::path::Path;
use symbols::format;

#[test]
fn self_snapshot() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let output = format::format_directory(&root);
    insta::assert_snapshot!(output);
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
