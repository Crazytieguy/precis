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

    if path.is_file() {
        let group_ctx = group::GroupCtx {
            store: &store,
            root: path.parent().unwrap_or(Path::new("")).to_path_buf(),
        };
        let seed = build_file_seed(path, &group_ctx);
        let ctx = schedule::ScheduleCtx {
            groups: group_ctx,
            budget,
            char_budget,
        };
        return schedule::schedule(seed, &ctx);
    }

    let root = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    let group_ctx = group::GroupCtx {
        store: &store,
        root: root.clone(),
    };
    let seed = build_dir_seed(&root, &group_ctx);
    let ctx = schedule::ScheduleCtx {
        groups: group_ctx,
        budget,
        char_budget,
    };

    schedule::schedule(seed, &ctx)
}

/// Build the seed frontier for a single-file input (design §5.2).
fn build_file_seed<'s>(path: &Path, ctx: &group::GroupCtx<'s>) -> Vec<group::Group<'s>> {
    let rel = ctx.rel_path(path);
    let role = classify::FileRole::from_path(rel);
    let items = [path.to_path_buf()];
    group::files::children_for_files(&ctx.root, role, &items, 1.0, ctx)
}

/// Build the seed frontier for a directory input (design §5.2).
/// Walks one level to produce a FoldersGroup (if sub-folders exist) and FilesGroups.
fn build_dir_seed<'s>(abs_path: &Path, ctx: &group::GroupCtx<'s>) -> Vec<group::Group<'s>> {
    let (subdirs, files_by_role) = group::folders::walk_dir_entries(abs_path, &ctx.root);

    let mut seed = Vec::new();

    if !subdirs.is_empty() {
        seed.push(group::Group::Folders(group::FoldersGroup {
            parent_dir: abs_path.to_path_buf(),
            items: subdirs,
            inherited_modifier: 1.0,
            cached_item_costs: None,
        }));
    }

    seed.extend(group::folders::create_files_groups(
        abs_path,
        files_by_role,
        1.0,
        &ctx.root,
    ));

    seed
}
