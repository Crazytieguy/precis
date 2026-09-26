//! What a language module's `extract` returns: one [`FileModel`] per
//! source file. The engine turns it into the file's batch ladder
//! (`ModuleDoc`, `Names`, then `Decl` / `Doc` / `Body` per declaration);
//! a language module never builds a batch, picks a value or names a key.
//!
//! # Rows
//!
//! Every row is a **1-based source line number**: tree-sitter's 0-based
//! `row + 1`, the numbering of [`crate::content::Span`]. Use
//! [`SourceFile::node_rows`](super::SourceFile::node_rows) to turn a node
//! into rows; it ignores a final row the node covers only with whitespace.
//! Row lists may be unsorted, may repeat a row and may include blank rows:
//! the engine sorts, dedups and drops blank rows from every part (interior
//! blanks still render, bridged between the kept rows around them).
//!
//! # Parts of a declaration
//!
//! A [`DeclInfo`] splits a declaration's rows into `doc`, `head` and
//! `body`, plus `name_rows`:
//!
//! - **`doc`**: the doc comment rows directly above the declaration (no
//!   blank row between them and the head), or a docstring inside it (the
//!   Python first-statement string, whose rows are then not in `body`). One
//!   [`Item`] per paragraph (per block for TS/JS JSDoc).
//! - **`head`** starts at the declaration's first row: its first leading
//!   attribute / decorator / annotation / modifier row, never a doc
//!   comment. Where it ends depends on [`Shape`]:
//!   - `Callable`: the last row before the first body statement starts,
//!     and at least the last name row; the whole declaration when the body
//!     has no statement. So a multi-line signature and the row that opens
//!     the body (`) -> T {`, `def f():`) are head, and a statement sharing
//!     a row with the signature (`fn f() { x }`) is head only.
//!   - `Whole`: the row that opens the body (`struct A {`, `class A:`,
//!     `const (`), plus the closing row (`}`, `};`, `)`) when it holds
//!     nothing else. A declaration without a body (`type A = B;`,
//!     `const X = [ … ];`) is all head. Rows between body items that belong
//!     to no item (a comment between two methods) are in no part.
//! - **`body`**, by [`Shape`]:
//!   - `Callable`: one [`Item`] per top-level statement of the body block,
//!     leading comments included, in source order, excluding rows already
//!     in `head` or `doc`. The closing row (`}`, `end`) is in no part.
//!   - `Whole`: one [`Item`] per field / variant / spec / entry (its own
//!     attributes and comments included), and, for a container, one [`Item`]
//!     per member holding exactly that member's `name_rows`, all in source
//!     order.
//! - **`name_rows`**: the rows the file's roster lists for the declaration.
//!   Usually the one row holding the declared name: the `def` / `fn` /
//!   `class` row. Several rows when one row doesn't name it (a Go grouped
//!   declaration lists its `const (` row and one per spec; a C anonymous
//!   `typedef struct {…} Name;` lists its first and its `} Name;` rows) or
//!   leaves out what it is (Python lists one-row decorators and a multi-row
//!   signature's closing row). Must be non-empty and inside `head` for a
//!   `Callable`, inside `head ∪ body` for a `Whole`.
//!
//! The `Decl` batch renders `head` (plus `body` for `Whole`), `Doc` renders
//! `doc`, `Body` renders a `Callable`'s `body`. An [`Item`] is the unit
//! neither the chunker nor the budget's cut of the last batch splits.
//!
//! # Members
//!
//! A container (class, Rust `impl` / `trait`, TS class) is a `Whole`
//! declaration whose `members` are the methods / functions it defines, each
//! itself a [`DeclInfo`] (usually `Callable`) whose own `members` is empty:
//! nesting stops at one level. A container's `body` lists each member's
//! `name_rows` as one [`Item`], so the container's `Decl` is its member
//! roster and each member's `Decl` hangs under the chunk that lists it.
//! Every other row of a member (the rest of its signature, its doc, its
//! body) must be absent from the container's `head` and `body`. Fields are
//! body [`Item`]s, not members.
//!
//! # Hidden declarations
//!
//! The language module decides which declarations to leave out of the
//! model entirely (not in `decls`, not in a container's `members`, their
//! name rows not in a container's `body`): test code, `#[doc(hidden)]`,
//! unexported TS/JS and Go declarations, C non-`inline` `static` in a
//! header, and members the language enforces as private to their
//! container. Every declaration in the model is priced alike.
//!
//! # Ownership
//!
//! Within a file, no row may belong to two of `module_doc`, `reexports`
//! and the declarations. A row in no part never renders, including
//! context wrapping several declarations (a C `#ifdef … #endif`, a TS
//! `declare namespace X {` line). A row two batches claim outside one
//! predecessor chain is dropped from the later batch, so a violation
//! loses content rather than failing the run.
//!
//! # What the engine does, so `extract` doesn't
//!
//! - Orders declarations (and each container's members) by first row, the
//!   smallest row in any part, `doc` included.
//! - Merges siblings whose first rows are equal (Go `var a = 1; var b = 2`
//!   on one row, two TS members on one row): the union of each part, the
//!   first one's shape.
//! - **Trims at the next sibling**: drops every row at or past the next
//!   sibling's first row. Tree-sitter sometimes extends a node into the
//!   next declaration (a C attribute-like macro, a trailing comment);
//!   untrimmed, the earlier `Decl` would claim the next one's name row
//!   through their shared `Names` ancestor and become its predecessor. The
//!   trim is load-bearing, not redundant with the ownership ledger.
//! - Strips `module_doc` rows from every other part, then makes each
//!   declaration's parts disjoint (`doc` over `head` over `body`), so a row
//!   listed in two parts renders once, in the earlier rung.
//! - Chunks oversize parts, values batches and gates each batch on its
//!   predecessor.

/// The extraction result for one source file.
#[derive(Debug, Default, Clone)]
pub(crate) struct FileModel {
    /// File-level documentation, one [`Item`] per paragraph: crate / module
    /// doc comments, the package comment, the module docstring, and
    /// language-specific identity blocks (Lua `_VERSION` table, Python
    /// module dunders other than `__all__`).
    pub module_doc: Vec<Item>,
    /// What the file exposes without declaring it here, one [`Item`] per
    /// statement: `pub use`, `pub mod name;`, `export … from`,
    /// `export * from`, `__init__.py` from-imports, `__all__`. Listed in the
    /// file's `Names` roster alongside the declarations. Ordinary imports
    /// are not modeled.
    pub reexports: Vec<Item>,
    /// Admitted top-level declarations, in any order.
    pub decls: Vec<DeclInfo>,
}

/// One declaration: its rows split into parts (see the module docs).
#[derive(Debug, Clone)]
pub(crate) struct DeclInfo {
    /// Rows the `Names` roster (or, for a member, its container's `Decl`)
    /// lists. Non-empty; inside `head` for `Callable`, inside
    /// `head ∪ body` for `Whole`.
    pub name_rows: Vec<usize>,
    /// Leading attributes / decorators through the signature (`Callable`),
    /// or through the row opening the body, plus the closing row (`Whole`).
    /// Non-empty.
    pub head: Vec<usize>,
    /// Doc comment or docstring, one [`Item`] per paragraph.
    pub doc: Vec<Item>,
    /// Statements (`Callable`); fields, variants, specs and member name
    /// rows (`Whole`).
    pub body: Vec<Item>,
    pub shape: Shape,
    /// A container's admitted members. Always empty on a member.
    pub members: Vec<DeclInfo>,
}

impl DeclInfo {
    /// A declaration without doc, body or members.
    pub(crate) fn new(name_rows: Vec<usize>, head: Vec<usize>, shape: Shape) -> Self {
        Self {
            name_rows,
            head,
            doc: Vec::new(),
            body: Vec::new(),
            shape,
            members: Vec::new(),
        }
    }
}

/// A group of rows the chunker keeps together: one paragraph, statement,
/// field or re-export.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Item {
    pub rows: Vec<usize>,
}

impl Item {
    pub(crate) fn new(rows: impl IntoIterator<Item = usize>) -> Self {
        Self {
            rows: rows.into_iter().collect(),
        }
    }
}

/// How a declaration's body relates to its `Decl` batch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Shape {
    /// Function, method, arrow-function const, C function definition: the
    /// `Decl` is the signature; the body is a separate `Body` follow-up.
    Callable,
    /// Type, struct, enum, interface, alias, typedef, constant, macro,
    /// every container, and a program's flow functions: the `Decl` is head
    /// plus body.
    Whole,
}
