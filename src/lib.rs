pub mod classify;
pub mod format;
pub mod group;
pub mod heuristics;
pub mod parse;
pub mod render;
pub mod schedule;
pub mod store;

use std::path::Path;

/// Language family for rendering and parsing heuristics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Lang {
    Rust,
    Python,
    Go,
    C,
    Java,
    Lua,
    Markdown,
    JsTs,
    Json,
    Toml,
    Yaml,
}

impl Lang {
    pub fn from_extension(ext: &str) -> Option<Lang> {
        match ext {
            "rs" => Some(Lang::Rust),
            "py" => Some(Lang::Python),
            "go" => Some(Lang::Go),
            "c" | "h" | "cpp" | "cc" | "cxx" | "hpp" | "hxx" | "hh" => Some(Lang::C),
            "java" => Some(Lang::Java),
            "lua" => Some(Lang::Lua),
            "md" | "mdx" => Some(Lang::Markdown),
            "ts" | "tsx" | "js" | "jsx" | "mts" | "cts" | "mjs" | "cjs" => Some(Lang::JsTs),
            "json" => Some(Lang::Json),
            "toml" => Some(Lang::Toml),
            "yaml" | "yml" => Some(Lang::Yaml),
            _ => None,
        }
    }

    pub fn from_path(path: &Path) -> Option<Lang> {
        path.extension()
            .and_then(|e| e.to_str())
            .and_then(|ext| Lang::from_extension(&ext.to_ascii_lowercase()))
    }
}

/// The single public entry point (plan: "Public library surface").
/// Renders a path (file or directory) within token and optional character budgets.
pub fn render(path: &Path, budget: usize, char_budget: Option<usize>) -> String {
    let store = store::ParseStore::new();

    let seed = if path.is_file() {
        let display_path = path
            .file_name()
            .map(|n| Path::new(n).to_path_buf())
            .unwrap_or_else(|| path.to_path_buf());
        let ctx_temp = schedule::ScheduleCtx {
            store: &store,
            root: path.parent().unwrap_or(Path::new("")).to_path_buf(),
            budget,
            char_budget,
        };
        let seed = build_file_seed(path, &display_path, &store);
        return schedule::schedule(seed, &ctx_temp);
    } else {
        build_dir_seed(path)
    };

    let root = path
        .canonicalize()
        .unwrap_or_else(|_| path.to_path_buf());
    let ctx = schedule::ScheduleCtx {
        store: &store,
        root: root.clone(),
        budget,
        char_budget,
    };

    schedule::schedule(seed, &ctx)
}

/// Build the seed frontier for a single-file input (design §5.2).
fn build_file_seed<'s>(
    path: &Path,
    display_path: &Path,
    store: &'s store::ParseStore,
) -> Vec<group::Group<'s>> {
    // Read and store the source
    let source = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => return vec![],
    };
    store.store_source(path, source);

    // Parse the stored source
    let (source, tree) = match store.parse_stored(path) {
        Some(pair) => pair,
        None => {
            // Unsupported language — return empty (file header only)
            return vec![];
        }
    };

    let config = match store.config_for(path) {
        Some(c) => c,
        None => return vec![],
    };

    let items = parse::extract_items(path, source, tree, config);
    let lines: Vec<&str> = source.lines().collect();
    let lang = Lang::from_path(path);

    // Build TsGroups from extracted items (same logic as files::children)
    let mut buckets: std::collections::HashMap<group::TsGroupKey, Vec<group::TsItem<'s>>> =
        std::collections::HashMap::new();

    for item in &items {
        let keys = group::files::item_to_group_keys(item, &lines, lang);
        for key in keys {
            let ts_item = group::TsItem {
                path: display_path.to_path_buf(),
                source,
                node: item.node,
                name: item.name.clone(),
                start_line: item.start_line,
                end_line: item.end_line,
            };
            buckets.entry(key).or_default().push(ts_item);
        }
    }

    let mut groups: Vec<group::Group<'s>> = Vec::new();
    let mut sorted_buckets: Vec<_> = buckets.into_iter().collect();
    sorted_buckets.sort_by(|a, b| a.0.cmp(&b.0));
    for (key, items) in sorted_buckets {
        if items.is_empty() {
            continue;
        }
        let modifier = group::files::compute_item_modifier(&key, 1.0);
        groups.push(group::Group::Ts(group::TsGroup {
            key,
            items,
            inherited_modifier: modifier,
            dependent_siblings: vec![],
            cached_render: None,
        }));
    }
    groups
}

/// Build the seed frontier for a directory input (design §5.2).
fn build_dir_seed(path: &Path) -> Vec<group::Group<'static>> {
    let abs_path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    vec![group::Group::Folders(group::FoldersGroup {
        parent_dir: abs_path,
        inherited_modifier: 1.0,
        category: classify::FileCategory::Source,
    })]
}

