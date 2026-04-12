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

        let relative = ctx.rel_path(&abs_path).to_path_buf();

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

    // Create FilesGroups per role, partitioned by per-file properties that
    // affect the modifier (is_config, is_type_declaration, is_header).
    // Files with different properties get different modifiers, so they must
    // be in separate groups (D7: split when items would be prioritized differently).
    let mut sorted_roles: Vec<_> = files_by_role.into_iter().collect();
    sorted_roles.sort_by_key(|(role, _)| *role);
    let is_root_dir = abs_dir == ctx.root;
    for (role, files) in sorted_roles {
        if files.is_empty() {
            continue;
        }

        let mut partitions: HashMap<(bool, bool, bool), Vec<PathBuf>> = HashMap::new();
        for file_path in files {
            let relative = ctx.rel_path(&file_path);
            let props = classify::file_modifier_properties(relative);
            partitions.entry(props).or_default().push(file_path);
        }

        let mut sorted_partitions: Vec<_> = partitions.into_iter().collect();
        sorted_partitions.sort_by_key(|e| e.0);

        for ((is_config, is_type_declaration, is_header), mut part_files) in sorted_partitions {
            part_files.sort();

            let contribution = heuristics::files_contribution(
                role,
                is_root_dir,
                is_config,
                is_type_declaration,
                is_header,
            );

            result.push(Group::Files(FilesGroup {
                parent_dir: g.parent_dir.clone(),
                role,
                items: part_files,
                inherited_modifier: g.inherited_modifier * contribution,
                is_config,
                is_type_declaration,
                is_header,
            }));
        }
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
