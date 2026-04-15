//! Per-kind traits for tree-sitter groups.
//!
//! Each variant of `TsGroupKey` wraps a per-family struct (in
//! `src/group/ts/<family>.rs`). `TsGroupKindMethods` is implemented on each
//! of those structs, and `TsGroupKey` delegates through a central match.
//!
//! `TsGroupKindParse` is an associated-function trait (no `&self`) called by
//! the parse dispatcher (`crate::parse::dispatch_kinds`) on concrete struct
//! types. It is not enum-dispatched.

use tree_sitter::Node;

use crate::Lang;
use crate::group::{Group, TsGroup, TsGroupKey, TsItem};
use crate::parse::file_ctx::FileCtx;
use crate::render::LineEntry;

/// Receiver methods on tree-sitter group kinds. Each per-family struct
/// implements this trait; the central `TsGroupKey` impl matches on the
/// variant and forwards to the inner struct.
pub trait TsGroupKindMethods {
    /// Render this kind's items into `LineEntry` values.
    fn render_item<'s>(&self, item: &TsItem<'s>) -> Vec<LineEntry<'s>>;

    /// Spawn child groups when this group is committed.
    ///
    /// The outer `ts::children` drains `parent.dependent_siblings` before
    /// calling this method, so impls see an immutable `&TsGroup` and only
    /// need to return the kind-specific spawn output. Default: no children.
    fn children<'s>(&self, _parent: &TsGroup<'s>) -> Vec<Group<'s>> {
        vec![]
    }
}

/// How a kind extracts items from a file.
pub enum KindParseStrategy {
    /// This kind doesn't apply to this language.
    None,
    /// This kind reads source/lines directly without a tree-sitter query.
    /// `from_parse` receives an empty match slice.
    SourceOnly,
    /// Standard query-based extraction. The dispatcher compiles the query
    /// (cached per `(LanguageConfig, kind)`) and walks every match, applying
    /// `is_inside_function` and language-specific filters before passing the
    /// matches to `from_parse`.
    Query(&'static str),
}

/// Cursor-independent wrapper around a tree-sitter `QueryMatch`. Tree-sitter's
/// raw `QueryMatch` borrows from the streaming cursor and is invalidated on
/// every `.next()`, so we materialize the fields we care about up front.
pub struct OwnedQueryMatch<'s> {
    /// The node captured by `@symbol`. Every per-kind query must declare a
    /// `@symbol` capture.
    pub symbol: Node<'s>,
    /// The "range" node used for end-line / doc detection. Equals `symbol`
    /// for most kinds; differs only for C++ `template_declaration`, which
    /// wraps the inner item but is the desired rendered range.
    pub range_node: Node<'s>,
}

impl<'s> OwnedQueryMatch<'s> {
    /// Whether this match carries a doc comment. Computed on demand so
    /// kinds that don't need it (heading, impl_block) don't pay, and kinds
    /// that reject the match before consulting it don't pay either.
    pub fn is_documented(&self, ctx: &FileCtx<'_>) -> bool {
        crate::parse::ast::is_documented(self.range_node, ctx.source, ctx.lang)
    }

    /// Whether this match is publicly visible. Mirrors `is_documented` in
    /// shape so per-kind `from_parse` impls don't have to thread
    /// `m.symbol, ctx` through a module-level helper.
    pub fn is_public(&self, ctx: &FileCtx<'_>) -> bool {
        crate::parse::symbol_is_public(self.symbol, ctx)
    }
}

/// Parse-time constructors for a kind. Not enum-dispatched — called on
/// concrete struct types by the dispatcher.
pub trait TsGroupKindParse {
    /// How does this kind parse for the given language?
    fn parse_strategy(lang: Lang) -> KindParseStrategy
    where
        Self: Sized;

    /// Walk the (already top-level-filtered) query matches and produce zero
    /// or more groups. For `SourceOnly` strategies, `matches` is empty.
    fn from_parse<'s>(
        matches: &[OwnedQueryMatch<'s>],
        ctx: &FileCtx<'s>,
    ) -> Vec<ParseTsGroup<'s>>
    where
        Self: Sized;
}

/// Pre-calibration tree-sitter group. Produced by per-kind `from_parse`
/// impls before cross-file aggregation and calibration.
#[allow(dead_code)]
pub struct ParseTsGroup<'s> {
    pub key: TsGroupKey,
    pub items: Vec<TsItem<'s>>,
    /// Sub-groups attached to this group. Populated inside `from_parse` for
    /// in-file relationships (heading nesting, private/public function
    /// gating within a single file). Calibration recurses into these.
    pub dependent_siblings: Vec<ParseTsGroup<'s>>,
}
