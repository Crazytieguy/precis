use std::collections::HashMap;
use std::path::PathBuf;

use crate::classify::{self, FileRole};
use crate::heuristics;
use crate::schedule::ScheduleCtx;

use super::{FilesGroup, FoldersGroup, Group};

/// Produce children when a FoldersGroup is scheduled.
/// Walks one directory level and classifies entries.
pub fn children<'s>(g: &mut FoldersGroup, ctx: &ScheduleCtx<'s>) -> Vec<Group<'s>> {
    let mut result: Vec<Group<'s>> = Vec::new();

    // Walk one level with the ignore crate (honors .gitignore)
    // Use the absolute path for FS walking, but store relative paths for display.
    let abs_dir = if g.parent_dir.is_relative() {
        ctx.root.join(&g.parent_dir)
    } else {
        g.parent_dir.clone()
    };
    let walker = ignore::WalkBuilder::new(&abs_dir)
        .max_depth(Some(1))
        .sort_by_file_path(|a, b| a.cmp(b))
        .build();

    let mut subdirs: Vec<(PathBuf, PathBuf)> = Vec::new(); // (abs, relative)
    let mut files_by_role: HashMap<FileRole, Vec<PathBuf>> = HashMap::new(); // abs paths

    for entry in walker.flatten() {
        let abs_path = entry.into_path();
        if abs_path == abs_dir {
            continue;
        }

        let relative = abs_path
            .strip_prefix(&ctx.root)
            .unwrap_or(&abs_path)
            .to_path_buf();

        if abs_path.is_dir() {
            if !classify::is_vendored_or_fixture(&relative) {
                subdirs.push((abs_path, relative));
            }
        } else if abs_path.is_file()
            && classify::is_source_file(&abs_path)
                && !classify::is_vendored_or_fixture(&relative)
            {
                let role = FileRole::from_path(&relative);
                files_by_role.entry(role).or_default().push(abs_path);
            }
    }

    subdirs.sort_by(|a, b| a.1.cmp(&b.1));

    // Create sub-FoldersGroups
    for (abs_dir, rel_dir) in subdirs {
        let category = classify::classify_file(&rel_dir);
        let contribution = heuristics::folders_contribution(&rel_dir, category);
        result.push(Group::Folders(FoldersGroup {
            parent_dir: abs_dir,
            inherited_modifier: g.inherited_modifier * contribution,
            category,
        }));
    }

    // Create FilesGroups per role (sorted for determinism)
    let mut sorted_roles: Vec<_> = files_by_role.into_iter().collect();
    sorted_roles.sort_by_key(|(role, _)| *role);
    for (role, mut files) in sorted_roles {
        if files.is_empty() {
            continue;
        }
        files.sort();

        let is_root_dir = abs_dir == ctx.root;

        // Compute per-file properties from first file (relative path for classification)
        let sample_path = &files[0];
        let relative = sample_path.strip_prefix(&ctx.root).unwrap_or(sample_path);
        let filename = relative.file_name().and_then(|n| n.to_str()).unwrap_or("");
        let is_config = classify::is_config_file(relative, filename);
        let is_type_declaration = classify::is_type_declaration_file(relative);
        let is_header = relative
            .extension()
            .and_then(|e| e.to_str())
            .is_some_and(|ext| classify::is_header_extension(&ext.to_ascii_lowercase()));

        let contribution = heuristics::files_contribution(
            role,
            is_root_dir,
            is_config,
            is_type_declaration,
            is_header,
            false, // is_generated checked after source read
        );

        result.push(Group::Files(FilesGroup {
            parent_dir: g.parent_dir.clone(),
            role,
            items: files,
            inherited_modifier: g.inherited_modifier * contribution,
            is_config,
            is_type_declaration,
            is_header,
            is_generated: false,
        }));
    }

    debug_assert!(
        result.iter().all(|g| match g {
            Group::Folders(f) => !f.parent_dir.as_os_str().is_empty(),
            Group::Files(f) => !f.items.is_empty(),
            Group::Ts(t) => !t.items.is_empty(),
        }),
        "D3: children() produced an empty group"
    );

    result
}
