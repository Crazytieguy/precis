use crate::Lang;

use super::{ExtractedItem, ItemKind};

/// Run all post-extraction processing on the item list.
pub(super) fn finalize(items: &mut Vec<ExtractedItem<'_>>, lang: Lang, _source: &str) {
    if lang == Lang::Rust {
        mark_reexports(items);
    }
    dedup_overloads(items, lang);
}

fn mark_reexports(items: &mut [ExtractedItem<'_>]) {
    let module_names: Vec<String> = items
        .iter()
        .filter(|s| s.kind == ItemKind::Module)
        .map(|s| s.name.clone())
        .collect();

    for item in items.iter_mut() {
        if item.kind != ItemKind::Import || !item.is_public {
            continue;
        }
        if item.name.starts_with("self::") {
            item.is_reexport = true;
            continue;
        }
        if let Some(first_segment) = item.name.split("::").next()
            && module_names.iter().any(|m| m == first_segment)
        {
            item.is_reexport = true;
        }
    }
}

fn dedup_overloads(items: &mut Vec<ExtractedItem<'_>>, lang: Lang) {
    if matches!(lang, Lang::Go | Lang::Python | Lang::Java) {
        return;
    }
    let mut seen = std::collections::HashSet::new();
    let mut to_remove = Vec::new();
    let mut prev: Option<(String, ItemKind)> = None;

    for (i, item) in items.iter().enumerate() {
        // Consecutive same-name same-kind: keep the last one
        if let Some((ref pname, pkind)) = prev
            && pname == &item.name && pkind == item.kind {
                to_remove.push(i - 1);
                prev = Some((item.name.clone(), item.kind));
                continue;
            }
        // Non-consecutive same struct/enum/type: skip
        if matches!(
            item.kind,
            ItemKind::Struct | ItemKind::Enum | ItemKind::TypeAlias
        ) {
            let key = (item.name.clone(), item.kind);
            if !seen.insert(key) {
                to_remove.push(i);
                continue;
            }
        }
        prev = Some((item.name.clone(), item.kind));
    }

    // Remove in reverse order to preserve indices
    to_remove.sort_unstable();
    to_remove.dedup();
    for idx in to_remove.into_iter().rev() {
        items.remove(idx);
    }
}
