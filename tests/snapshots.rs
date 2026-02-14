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

#[test]
fn tsx_sample_snapshot() {
    let source = r#"
import React, { useState, forwardRef } from "react";

export interface ButtonProps {
    label: string;
    onClick: () => void;
    disabled?: boolean;
}

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
"#;
    let output = format::format_file_symbols(Path::new("sample.tsx"), Path::new(""), source);
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

#[test]
fn fixture_cmdk() {
    let Some(root) = fixture_path("cmdk/cmdk/src") else {
        eprintln!("skipping fixture_cmdk: clone with `git clone --depth 1 https://github.com/pacocoursey/cmdk.git test/fixtures/cmdk`");
        return;
    };
    let output = format::format_directory(&root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_ts_pattern() {
    let Some(root) = fixture_path("ts-pattern/src") else {
        eprintln!("skipping fixture_ts_pattern: clone with `git clone --depth 1 https://github.com/gvergnaud/ts-pattern.git test/fixtures/ts-pattern`");
        return;
    };
    let output = format::format_directory(&root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_anyhow() {
    let Some(root) = fixture_path("anyhow/src") else {
        eprintln!("skipping fixture_anyhow: clone with `git clone --depth 1 https://github.com/dtolnay/anyhow.git test/fixtures/anyhow`");
        return;
    };
    let output = format::format_directory(&root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_once_cell() {
    let Some(root) = fixture_path("once_cell/src") else {
        eprintln!("skipping fixture_once_cell: clone with `git clone --depth 1 https://github.com/matklad/once_cell.git test/fixtures/once_cell`");
        return;
    };
    let output = format::format_directory(&root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_react_hot_toast() {
    let Some(root) = fixture_path("react-hot-toast/src") else {
        eprintln!("skipping fixture_react_hot_toast: clone with `git clone --depth 1 https://github.com/timolins/react-hot-toast.git test/fixtures/react-hot-toast`");
        return;
    };
    let output = format::format_directory(&root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_superstruct() {
    let Some(root) = fixture_path("superstruct/src") else {
        eprintln!("skipping fixture_superstruct: clone with `git clone --depth 1 https://github.com/ianstormtaylor/superstruct.git test/fixtures/superstruct`");
        return;
    };
    let output = format::format_directory(&root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_dotenv() {
    let Some(root) = fixture_path("dotenv/lib") else {
        eprintln!("skipping fixture_dotenv: clone with `git clone --depth 1 https://github.com/motdotla/dotenv.git test/fixtures/dotenv`");
        return;
    };
    let output = format::format_directory(&root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_commander() {
    let Some(root) = fixture_path("commander/lib") else {
        eprintln!("skipping fixture_commander: clone with `git clone --depth 1 https://github.com/tj/commander.js.git test/fixtures/commander`");
        return;
    };
    let output = format::format_directory(&root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_thiserror() {
    let Some(root) = fixture_path("thiserror/src") else {
        eprintln!("skipping fixture_thiserror: clone with `git clone --depth 1 https://github.com/dtolnay/thiserror.git test/fixtures/thiserror`");
        return;
    };
    let output = format::format_directory(&root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_sonner() {
    let Some(root) = fixture_path("sonner/src") else {
        eprintln!("skipping fixture_sonner: clone with `git clone --depth 1 https://github.com/emilkowalski/sonner.git test/fixtures/sonner`");
        return;
    };
    let output = format::format_directory(&root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_mitt() {
    let Some(root) = fixture_path("mitt/src") else {
        eprintln!("skipping fixture_mitt: clone with `git clone --depth 1 https://github.com/developit/mitt.git test/fixtures/mitt`");
        return;
    };
    let output = format::format_directory(&root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_debug() {
    let Some(root) = fixture_path("debug/src") else {
        eprintln!("skipping fixture_debug: clone with `git clone --depth 1 https://github.com/debug-js/debug.git test/fixtures/debug`");
        return;
    };
    let output = format::format_directory(&root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_log() {
    let Some(root) = fixture_path("log/src") else {
        eprintln!("skipping fixture_log: clone with `git clone --depth 1 https://github.com/rust-lang/log.git test/fixtures/log`");
        return;
    };
    let output = format::format_directory(&root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_ky() {
    let Some(root) = fixture_path("ky/source") else {
        eprintln!("skipping fixture_ky: clone with `git clone --depth 1 https://github.com/sindresorhus/ky.git test/fixtures/ky`");
        return;
    };
    let output = format::format_directory(&root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_ini() {
    let Some(root) = fixture_path("ini/lib") else {
        eprintln!("skipping fixture_ini: clone with `git clone --depth 1 https://github.com/npm/ini.git test/fixtures/ini`");
        return;
    };
    let output = format::format_directory(&root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_vaul() {
    let Some(root) = fixture_path("vaul/src") else {
        eprintln!("skipping fixture_vaul: clone with `git clone --depth 1 https://github.com/emilkowalski/vaul.git test/fixtures/vaul`");
        return;
    };
    let output = format::format_directory(&root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_input_otp() {
    let Some(root) = fixture_path("input-otp/packages/input-otp/src") else {
        eprintln!("skipping fixture_input_otp: clone with `git clone --depth 1 https://github.com/guilhermerodz/input-otp.git test/fixtures/input-otp`");
        return;
    };
    let output = format::format_directory(&root);
    insta::assert_snapshot!(output);
}
