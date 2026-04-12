use crate::Lang;

use super::{ExtractedItem, ItemKind};

pub(super) fn finalize(items: &mut Vec<ExtractedItem<'_>>, lang: Lang, source: &str) {
    if lang == Lang::Rust {
        mark_reexports(items, source);
    }
    dedup_overloads(items, lang, source);
}

fn mark_reexports(items: &mut [ExtractedItem<'_>], source: &str) {
    let module_names: Vec<&str> = items
        .iter()
        .filter(|s| s.kind == ItemKind::Module)
        .filter_map(|s| {
            s.node
                .child_by_field_name("name")
                .and_then(|n| n.utf8_text(source.as_bytes()).ok())
        })
        .collect();

    for item in items.iter_mut() {
        if item.kind != ItemKind::Import || !item.is_public {
            continue;
        }
        let text = item.node.utf8_text(source.as_bytes()).unwrap_or("").trim();
        let path = text
            .strip_prefix("pub")
            .map(|s| s.trim_start())
            .unwrap_or(text);
        let path = path
            .strip_prefix("use")
            .map(|s| s.trim_start())
            .unwrap_or(path);
        let path = path.trim_end_matches(';').trim();

        if path.starts_with("crate::")
            || path.starts_with("self::")
            || path.starts_with("super::")
        {
            item.is_reexport = true;
            continue;
        }
        if let Some(first_segment) = path.split("::").next()
            && module_names.contains(&first_segment)
        {
            item.is_reexport = true;
        }
    }
}

fn dedup_overloads(items: &mut Vec<ExtractedItem<'_>>, lang: Lang, source: &str) {
    if !matches!(lang, Lang::Rust | Lang::C | Lang::JsTs) {
        return;
    }
    let mut seen = std::collections::HashSet::new();
    let mut to_remove = Vec::new();
    let mut prev: Option<(&str, ItemKind)> = None;

    for (i, item) in items.iter().enumerate() {
        let ident = item_identifier(item, source);

        if let Some((pname, pkind)) = prev
            && pname == ident && pkind == item.kind {
                to_remove.push(i - 1);
                prev = Some((ident, item.kind));
                continue;
            }
        if matches!(
            item.kind,
            ItemKind::Struct | ItemKind::Enum | ItemKind::TypeAlias
        ) && !seen.insert((ident, item.kind))
        {
            to_remove.push(i);
            continue;
        }
        prev = Some((ident, item.kind));
    }

    to_remove.sort_unstable();
    to_remove.dedup();
    for idx in to_remove.into_iter().rev() {
        items.remove(idx);
    }
}

fn item_identifier<'a>(item: &ExtractedItem<'_>, source: &'a str) -> &'a str {
    if let Some(name_node) = item.node.child_by_field_name("name")
        && let Ok(text) = name_node.utf8_text(source.as_bytes())
    {
        return text.trim();
    }
    if let Some(decl) = item.node.child_by_field_name("declarator") {
        let found = super::ast::find_descendant_of_kind(decl, "type_identifier")
            .or_else(|| super::ast::find_descendant_of_kind(decl, "identifier"));
        if let Some(n) = found
            && let Ok(text) = n.utf8_text(source.as_bytes())
        {
            return text.trim();
        }
    }
    if item.node.kind() == "lexical_declaration" {
        let mut cursor = item.node.walk();
        if let Some(vd) = item.node.children(&mut cursor).find(|c| c.kind() == "variable_declarator")
            && let Some(name_node) = vd.child_by_field_name("name")
            && let Ok(text) = name_node.utf8_text(source.as_bytes())
        {
            return text.trim();
        }
    }
    if item.node.kind() == "impl_item"
        && let Ok(text) = item.node.utf8_text(source.as_bytes())
    {
        return text.lines().next().unwrap_or("").trim();
    }
    item.node
        .utf8_text(source.as_bytes())
        .map(|s| s.lines().next().unwrap_or("").trim())
        .unwrap_or("")
}
