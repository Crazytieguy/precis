//! Per-file context passed into per-kind `from_parse` impls.

use std::collections::HashSet;
use std::path::Path;

use tree_sitter::Node;

use crate::Lang;
use crate::store::LanguageConfig;

pub struct FileCtx<'s> {
    pub config: &'s LanguageConfig,
    pub lang: Lang,
    pub display_path: &'s Path,
    pub source: &'s str,
    pub root: Node<'s>,
    pub lines: Vec<&'s str>,
    pub mod_names: Vec<&'s str>,
    pub export_names: HashSet<&'s str>,
}
