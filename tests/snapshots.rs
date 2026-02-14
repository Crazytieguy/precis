use std::path::Path;
use precis::format;

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

fn rust_sample() -> &'static str {
    r#"
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
"#
}

fn typescript_sample() -> &'static str {
    r#"
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
"#
}

fn javascript_sample() -> &'static str {
    r#"
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
"#
}

fn tsx_sample() -> &'static str {
    r#"
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
"#
}

// Level 0: file paths only

#[test]
fn rust_sample_level0() {
    let output = format::render_file(0, Path::new("sample.rs"), Path::new(""), rust_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn typescript_sample_level0() {
    let output = format::render_file(0, Path::new("sample.ts"), Path::new(""), typescript_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn javascript_sample_level0() {
    let output = format::render_file(0, Path::new("sample.js"), Path::new(""), javascript_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn tsx_sample_level0() {
    let output = format::render_file(0, Path::new("sample.tsx"), Path::new(""), tsx_sample());
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
    let output = format::render_file(1, Path::new("sample.ts"), Path::new(""), typescript_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn javascript_sample_snapshot() {
    let output = format::render_file(1, Path::new("sample.js"), Path::new(""), javascript_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn tsx_sample_snapshot() {
    let output = format::render_file(1, Path::new("sample.tsx"), Path::new(""), tsx_sample());
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
    let output = format::render_file(2, Path::new("sample.ts"), Path::new(""), typescript_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn javascript_sample_level2() {
    let output = format::render_file(2, Path::new("sample.js"), Path::new(""), javascript_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn tsx_sample_level2() {
    let output = format::render_file(2, Path::new("sample.tsx"), Path::new(""), tsx_sample());
    insta::assert_snapshot!(output);
}

// Level 3: full source

#[test]
fn rust_sample_level3() {
    let output = format::render_file(3, Path::new("sample.rs"), Path::new(""), rust_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn typescript_sample_level3() {
    let output = format::render_file(3, Path::new("sample.ts"), Path::new(""), typescript_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn javascript_sample_level3() {
    let output = format::render_file(3, Path::new("sample.js"), Path::new(""), javascript_sample());
    insta::assert_snapshot!(output);
}

#[test]
fn tsx_sample_level3() {
    let output = format::render_file(3, Path::new("sample.tsx"), Path::new(""), tsx_sample());
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
    let output = format::render_directory(1, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_neverthrow() {
    let Some(root) = fixture_path("neverthrow/src") else {
        eprintln!("skipping fixture_neverthrow: clone with `git clone --depth 1 https://github.com/supermacro/neverthrow.git test/fixtures/neverthrow`");
        return;
    };
    let output = format::render_directory(1, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_semver() {
    let Some(root) = fixture_path("semver/classes") else {
        eprintln!("skipping fixture_semver: clone with `git clone --depth 1 https://github.com/npm/node-semver.git test/fixtures/semver`");
        return;
    };
    let output = format::render_directory(1, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_cmdk() {
    let Some(root) = fixture_path("cmdk/cmdk/src") else {
        eprintln!("skipping fixture_cmdk: clone with `git clone --depth 1 https://github.com/pacocoursey/cmdk.git test/fixtures/cmdk`");
        return;
    };
    let output = format::render_directory(1, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_ts_pattern() {
    let Some(root) = fixture_path("ts-pattern/src") else {
        eprintln!("skipping fixture_ts_pattern: clone with `git clone --depth 1 https://github.com/gvergnaud/ts-pattern.git test/fixtures/ts-pattern`");
        return;
    };
    let output = format::render_directory(1, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_anyhow() {
    let Some(root) = fixture_path("anyhow/src") else {
        eprintln!("skipping fixture_anyhow: clone with `git clone --depth 1 https://github.com/dtolnay/anyhow.git test/fixtures/anyhow`");
        return;
    };
    let output = format::render_directory(1, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_once_cell() {
    let Some(root) = fixture_path("once_cell/src") else {
        eprintln!("skipping fixture_once_cell: clone with `git clone --depth 1 https://github.com/matklad/once_cell.git test/fixtures/once_cell`");
        return;
    };
    let output = format::render_directory(1, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_react_hot_toast() {
    let Some(root) = fixture_path("react-hot-toast/src") else {
        eprintln!("skipping fixture_react_hot_toast: clone with `git clone --depth 1 https://github.com/timolins/react-hot-toast.git test/fixtures/react-hot-toast`");
        return;
    };
    let output = format::render_directory(1, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_superstruct() {
    let Some(root) = fixture_path("superstruct/src") else {
        eprintln!("skipping fixture_superstruct: clone with `git clone --depth 1 https://github.com/ianstormtaylor/superstruct.git test/fixtures/superstruct`");
        return;
    };
    let output = format::render_directory(1, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_dotenv() {
    let Some(root) = fixture_path("dotenv/lib") else {
        eprintln!("skipping fixture_dotenv: clone with `git clone --depth 1 https://github.com/motdotla/dotenv.git test/fixtures/dotenv`");
        return;
    };
    let output = format::render_directory(1, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_commander() {
    let Some(root) = fixture_path("commander/lib") else {
        eprintln!("skipping fixture_commander: clone with `git clone --depth 1 https://github.com/tj/commander.js.git test/fixtures/commander`");
        return;
    };
    let output = format::render_directory(1, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_thiserror() {
    let Some(root) = fixture_path("thiserror/src") else {
        eprintln!("skipping fixture_thiserror: clone with `git clone --depth 1 https://github.com/dtolnay/thiserror.git test/fixtures/thiserror`");
        return;
    };
    let output = format::render_directory(1, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_sonner() {
    let Some(root) = fixture_path("sonner/src") else {
        eprintln!("skipping fixture_sonner: clone with `git clone --depth 1 https://github.com/emilkowalski/sonner.git test/fixtures/sonner`");
        return;
    };
    let output = format::render_directory(1, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_mitt() {
    let Some(root) = fixture_path("mitt/src") else {
        eprintln!("skipping fixture_mitt: clone with `git clone --depth 1 https://github.com/developit/mitt.git test/fixtures/mitt`");
        return;
    };
    let output = format::render_directory(1, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_debug() {
    let Some(root) = fixture_path("debug/src") else {
        eprintln!("skipping fixture_debug: clone with `git clone --depth 1 https://github.com/debug-js/debug.git test/fixtures/debug`");
        return;
    };
    let output = format::render_directory(1, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_log() {
    let Some(root) = fixture_path("log/src") else {
        eprintln!("skipping fixture_log: clone with `git clone --depth 1 https://github.com/rust-lang/log.git test/fixtures/log`");
        return;
    };
    let output = format::render_directory(1, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_ky() {
    let Some(root) = fixture_path("ky/source") else {
        eprintln!("skipping fixture_ky: clone with `git clone --depth 1 https://github.com/sindresorhus/ky.git test/fixtures/ky`");
        return;
    };
    let output = format::render_directory(1, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_ini() {
    let Some(root) = fixture_path("ini/lib") else {
        eprintln!("skipping fixture_ini: clone with `git clone --depth 1 https://github.com/npm/ini.git test/fixtures/ini`");
        return;
    };
    let output = format::render_directory(1, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_vaul() {
    let Some(root) = fixture_path("vaul/src") else {
        eprintln!("skipping fixture_vaul: clone with `git clone --depth 1 https://github.com/emilkowalski/vaul.git test/fixtures/vaul`");
        return;
    };
    let output = format::render_directory(1, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_input_otp() {
    let Some(root) = fixture_path("input-otp/packages/input-otp/src") else {
        eprintln!("skipping fixture_input_otp: clone with `git clone --depth 1 https://github.com/guilhermerodz/input-otp.git test/fixtures/input-otp`");
        return;
    };
    let output = format::render_directory(1, &root);
    insta::assert_snapshot!(output);
}

// Level 2 fixture-based snapshot tests (full signature lines).

#[test]
fn fixture_either_level2() {
    let Some(root) = fixture_path("either/src") else {
        eprintln!("skipping fixture_either_level2: clone with `git clone --depth 1 https://github.com/rayon-rs/either.git test/fixtures/either`");
        return;
    };
    let output = format::render_directory(2, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_neverthrow_level2() {
    let Some(root) = fixture_path("neverthrow/src") else {
        eprintln!("skipping fixture_neverthrow_level2: clone with `git clone --depth 1 https://github.com/supermacro/neverthrow.git test/fixtures/neverthrow`");
        return;
    };
    let output = format::render_directory(2, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_semver_level2() {
    let Some(root) = fixture_path("semver/classes") else {
        eprintln!("skipping fixture_semver_level2: clone with `git clone --depth 1 https://github.com/npm/node-semver.git test/fixtures/semver`");
        return;
    };
    let output = format::render_directory(2, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_cmdk_level2() {
    let Some(root) = fixture_path("cmdk/cmdk/src") else {
        eprintln!("skipping fixture_cmdk_level2: clone with `git clone --depth 1 https://github.com/pacocoursey/cmdk.git test/fixtures/cmdk`");
        return;
    };
    let output = format::render_directory(2, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_ts_pattern_level2() {
    let Some(root) = fixture_path("ts-pattern/src") else {
        eprintln!("skipping fixture_ts_pattern_level2: clone with `git clone --depth 1 https://github.com/gvergnaud/ts-pattern.git test/fixtures/ts-pattern`");
        return;
    };
    let output = format::render_directory(2, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_anyhow_level2() {
    let Some(root) = fixture_path("anyhow/src") else {
        eprintln!("skipping fixture_anyhow_level2: clone with `git clone --depth 1 https://github.com/dtolnay/anyhow.git test/fixtures/anyhow`");
        return;
    };
    let output = format::render_directory(2, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_once_cell_level2() {
    let Some(root) = fixture_path("once_cell/src") else {
        eprintln!("skipping fixture_once_cell_level2: clone with `git clone --depth 1 https://github.com/matklad/once_cell.git test/fixtures/once_cell`");
        return;
    };
    let output = format::render_directory(2, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_react_hot_toast_level2() {
    let Some(root) = fixture_path("react-hot-toast/src") else {
        eprintln!("skipping fixture_react_hot_toast_level2: clone with `git clone --depth 1 https://github.com/timolins/react-hot-toast.git test/fixtures/react-hot-toast`");
        return;
    };
    let output = format::render_directory(2, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_superstruct_level2() {
    let Some(root) = fixture_path("superstruct/src") else {
        eprintln!("skipping fixture_superstruct_level2: clone with `git clone --depth 1 https://github.com/ianstormtaylor/superstruct.git test/fixtures/superstruct`");
        return;
    };
    let output = format::render_directory(2, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_dotenv_level2() {
    let Some(root) = fixture_path("dotenv/lib") else {
        eprintln!("skipping fixture_dotenv_level2: clone with `git clone --depth 1 https://github.com/motdotla/dotenv.git test/fixtures/dotenv`");
        return;
    };
    let output = format::render_directory(2, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_commander_level2() {
    let Some(root) = fixture_path("commander/lib") else {
        eprintln!("skipping fixture_commander_level2: clone with `git clone --depth 1 https://github.com/tj/commander.js.git test/fixtures/commander`");
        return;
    };
    let output = format::render_directory(2, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_thiserror_level2() {
    let Some(root) = fixture_path("thiserror/src") else {
        eprintln!("skipping fixture_thiserror_level2: clone with `git clone --depth 1 https://github.com/dtolnay/thiserror.git test/fixtures/thiserror`");
        return;
    };
    let output = format::render_directory(2, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_sonner_level2() {
    let Some(root) = fixture_path("sonner/src") else {
        eprintln!("skipping fixture_sonner_level2: clone with `git clone --depth 1 https://github.com/emilkowalski/sonner.git test/fixtures/sonner`");
        return;
    };
    let output = format::render_directory(2, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_mitt_level2() {
    let Some(root) = fixture_path("mitt/src") else {
        eprintln!("skipping fixture_mitt_level2: clone with `git clone --depth 1 https://github.com/developit/mitt.git test/fixtures/mitt`");
        return;
    };
    let output = format::render_directory(2, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_debug_level2() {
    let Some(root) = fixture_path("debug/src") else {
        eprintln!("skipping fixture_debug_level2: clone with `git clone --depth 1 https://github.com/debug-js/debug.git test/fixtures/debug`");
        return;
    };
    let output = format::render_directory(2, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_log_level2() {
    let Some(root) = fixture_path("log/src") else {
        eprintln!("skipping fixture_log_level2: clone with `git clone --depth 1 https://github.com/rust-lang/log.git test/fixtures/log`");
        return;
    };
    let output = format::render_directory(2, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_ky_level2() {
    let Some(root) = fixture_path("ky/source") else {
        eprintln!("skipping fixture_ky_level2: clone with `git clone --depth 1 https://github.com/sindresorhus/ky.git test/fixtures/ky`");
        return;
    };
    let output = format::render_directory(2, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_ini_level2() {
    let Some(root) = fixture_path("ini/lib") else {
        eprintln!("skipping fixture_ini_level2: clone with `git clone --depth 1 https://github.com/npm/ini.git test/fixtures/ini`");
        return;
    };
    let output = format::render_directory(2, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_vaul_level2() {
    let Some(root) = fixture_path("vaul/src") else {
        eprintln!("skipping fixture_vaul_level2: clone with `git clone --depth 1 https://github.com/emilkowalski/vaul.git test/fixtures/vaul`");
        return;
    };
    let output = format::render_directory(2, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_input_otp_level2() {
    let Some(root) = fixture_path("input-otp/packages/input-otp/src") else {
        eprintln!("skipping fixture_input_otp_level2: clone with `git clone --depth 1 https://github.com/guilhermerodz/input-otp.git test/fixtures/input-otp`");
        return;
    };
    let output = format::render_directory(2, &root);
    insta::assert_snapshot!(output);
}

// Nested subdirectory tests: running on a subdirectory within a fixture tests
// that path display and file discovery work correctly at deeper nesting levels.

#[test]
fn fixture_ts_pattern_types_subdir() {
    let Some(root) = fixture_path("ts-pattern/src/types") else {
        eprintln!("skipping fixture_ts_pattern_types_subdir: clone with `git clone --depth 1 https://github.com/gvergnaud/ts-pattern.git test/fixtures/ts-pattern`");
        return;
    };
    let output = format::render_directory(1, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_ts_pattern_types_subdir_level2() {
    let Some(root) = fixture_path("ts-pattern/src/types") else {
        eprintln!("skipping fixture_ts_pattern_types_subdir_level2: clone with `git clone --depth 1 https://github.com/gvergnaud/ts-pattern.git test/fixtures/ts-pattern`");
        return;
    };
    let output = format::render_directory(2, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_react_hot_toast_components_subdir() {
    let Some(root) = fixture_path("react-hot-toast/src/components") else {
        eprintln!("skipping fixture_react_hot_toast_components_subdir: clone with `git clone --depth 1 https://github.com/timolins/react-hot-toast.git test/fixtures/react-hot-toast`");
        return;
    };
    let output = format::render_directory(1, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_react_hot_toast_components_subdir_level2() {
    let Some(root) = fixture_path("react-hot-toast/src/components") else {
        eprintln!("skipping fixture_react_hot_toast_components_subdir_level2: clone with `git clone --depth 1 https://github.com/timolins/react-hot-toast.git test/fixtures/react-hot-toast`");
        return;
    };
    let output = format::render_directory(2, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_superstruct_structs_subdir() {
    let Some(root) = fixture_path("superstruct/src/structs") else {
        eprintln!("skipping fixture_superstruct_structs_subdir: clone with `git clone --depth 1 https://github.com/ianstormtaylor/superstruct.git test/fixtures/superstruct`");
        return;
    };
    let output = format::render_directory(1, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_log_kv_subdir() {
    let Some(root) = fixture_path("log/src/kv") else {
        eprintln!("skipping fixture_log_kv_subdir: clone with `git clone --depth 1 https://github.com/rust-lang/log.git test/fixtures/log`");
        return;
    };
    let output = format::render_directory(1, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_log_kv_subdir_level2() {
    let Some(root) = fixture_path("log/src/kv") else {
        eprintln!("skipping fixture_log_kv_subdir_level2: clone with `git clone --depth 1 https://github.com/rust-lang/log.git test/fixtures/log`");
        return;
    };
    let output = format::render_directory(2, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_semver_functions_subdir() {
    let Some(root) = fixture_path("semver/functions") else {
        eprintln!("skipping fixture_semver_functions_subdir: clone with `git clone --depth 1 https://github.com/npm/node-semver.git test/fixtures/semver`");
        return;
    };
    let output = format::render_directory(1, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_semver_functions_subdir_level2() {
    let Some(root) = fixture_path("semver/functions") else {
        eprintln!("skipping fixture_semver_functions_subdir_level2: clone with `git clone --depth 1 https://github.com/npm/node-semver.git test/fixtures/semver`");
        return;
    };
    let output = format::render_directory(2, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_superstruct_structs_subdir_level2() {
    let Some(root) = fixture_path("superstruct/src/structs") else {
        eprintln!("skipping fixture_superstruct_structs_subdir_level2: clone with `git clone --depth 1 https://github.com/ianstormtaylor/superstruct.git test/fixtures/superstruct`");
        return;
    };
    let output = format::render_directory(2, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_neverthrow_internals_subdir() {
    let Some(root) = fixture_path("neverthrow/src/_internals") else {
        eprintln!("skipping fixture_neverthrow_internals_subdir: clone with `git clone --depth 1 https://github.com/supermacro/neverthrow.git test/fixtures/neverthrow`");
        return;
    };
    let output = format::render_directory(1, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_neverthrow_internals_subdir_level2() {
    let Some(root) = fixture_path("neverthrow/src/_internals") else {
        eprintln!("skipping fixture_neverthrow_internals_subdir_level2: clone with `git clone --depth 1 https://github.com/supermacro/neverthrow.git test/fixtures/neverthrow`");
        return;
    };
    let output = format::render_directory(2, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_ky_errors_subdir() {
    let Some(root) = fixture_path("ky/source/errors") else {
        eprintln!("skipping fixture_ky_errors_subdir: clone with `git clone --depth 1 https://github.com/sindresorhus/ky.git test/fixtures/ky`");
        return;
    };
    let output = format::render_directory(1, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_ky_errors_subdir_level2() {
    let Some(root) = fixture_path("ky/source/errors") else {
        eprintln!("skipping fixture_ky_errors_subdir_level2: clone with `git clone --depth 1 https://github.com/sindresorhus/ky.git test/fixtures/ky`");
        return;
    };
    let output = format::render_directory(2, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_thiserror_impl_subdir() {
    let Some(root) = fixture_path("thiserror/impl/src") else {
        eprintln!("skipping fixture_thiserror_impl_subdir: clone with `git clone --depth 1 https://github.com/dtolnay/thiserror.git test/fixtures/thiserror`");
        return;
    };
    let output = format::render_directory(1, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_thiserror_impl_subdir_level2() {
    let Some(root) = fixture_path("thiserror/impl/src") else {
        eprintln!("skipping fixture_thiserror_impl_subdir_level2: clone with `git clone --depth 1 https://github.com/dtolnay/thiserror.git test/fixtures/thiserror`");
        return;
    };
    let output = format::render_directory(2, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_commander_lib_subdir() {
    let Some(root) = fixture_path("commander/lib") else {
        eprintln!("skipping fixture_commander_lib_subdir: clone with `git clone --depth 1 https://github.com/tj/commander.js.git test/fixtures/commander`");
        return;
    };
    let output = format::render_directory(1, &root);
    insta::assert_snapshot!(output);
}

#[test]
fn fixture_commander_lib_subdir_level2() {
    let Some(root) = fixture_path("commander/lib") else {
        eprintln!("skipping fixture_commander_lib_subdir_level2: clone with `git clone --depth 1 https://github.com/tj/commander.js.git test/fixtures/commander`");
        return;
    };
    let output = format::render_directory(2, &root);
    insta::assert_snapshot!(output);
}

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
    let level = format::budget_level_file(usize::MAX, &file, &root, &source);
    assert_eq!(level, format::MAX_LEVEL);

    // Budget of 0 should give level 0
    let level = format::budget_level_file(0, &file, &root, &source);
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

/// Test the monotonicity invariant: for any file, a higher level must never
/// produce fewer words than a lower level.
#[test]
fn monotonicity_invariant() {
    // Test against all available fixtures
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
    ];

    let mut tested = 0;
    for (name, subpath) in fixtures {
        let Some(root) = fixture_path(subpath) else {
            continue;
        };
        tested += 1;
        for level in 0..format::MAX_LEVEL {
            let lower = format::render_directory(level, &root);
            let upper = format::render_directory(level + 1, &root);
            let lower_words: usize = lower.split_whitespace().count();
            let upper_words: usize = upper.split_whitespace().count();
            assert!(
                upper_words >= lower_words,
                "Monotonicity violation in {}: level {} ({} words) > level {} ({} words)",
                name,
                level,
                lower_words,
                level + 1,
                upper_words,
            );
        }
    }
    assert!(tested > 0, "No fixtures available for monotonicity test");
}

/// Test the budget algorithm: budget_level should return the highest level
/// whose output fits within the word budget.
#[test]
fn budget_algorithm() {
    let fixtures: &[&str] = &[
        "either/src",
        "anyhow/src",
        "neverthrow/src",
        "mitt/src",
        "ini/lib",
    ];

    let mut tested = 0;
    for subpath in fixtures {
        let Some(root) = fixture_path(subpath) else {
            continue;
        };
        tested += 1;

        // Compute word counts at each level
        let mut word_counts = Vec::new();
        for level in 0..=format::MAX_LEVEL {
            let output = format::render_directory(level, &root);
            word_counts.push(format::count_words(&output));
        }

        // Budget of 0 should give level 0 (file paths have at least 1 word)
        // unless even level 0 is empty
        let level0 = format::budget_level(0, &root);
        assert_eq!(level0, 0, "Budget 0 should yield level 0");

        // Very large budget should give MAX_LEVEL
        let level_max = format::budget_level(usize::MAX, &root);
        assert_eq!(
            level_max,
            format::MAX_LEVEL,
            "Huge budget should yield MAX_LEVEL"
        );

        // Budget exactly matching each level's word count should select that level
        for level in 0..=format::MAX_LEVEL {
            let selected = format::budget_level(word_counts[level as usize], &root);
            assert!(
                selected >= level,
                "Budget of {} words (level {} count) selected level {} (expected >= {})",
                word_counts[level as usize],
                level,
                selected,
                level,
            );
        }

        // Budget just below a level's word count should select the previous level
        for level in 1..=format::MAX_LEVEL {
            if word_counts[level as usize] > word_counts[(level - 1) as usize] {
                let budget = word_counts[level as usize] - 1;
                let selected = format::budget_level(budget, &root);
                assert!(
                    selected < level,
                    "Budget of {} (one below level {} count {}) selected level {} (expected < {})",
                    budget,
                    level,
                    word_counts[level as usize],
                    selected,
                    level,
                );
            }
        }
    }
    assert!(tested > 0, "No fixtures available for budget test");
}
