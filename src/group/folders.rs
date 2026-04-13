use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::classify::{self, FileRole};
use crate::heuristics;
use crate::schedule::ScheduleCtx;

use super::{FilesGroup, FoldersGroup, Group};

/// Walk one directory level (honoring .gitignore) and classify entries
/// into sub-folders and files-by-role. Returns absolute paths.
pub fn walk_dir_entries(
    abs_dir: &Path,
    root: &Path,
) -> (Vec<PathBuf>, HashMap<FileRole, Vec<PathBuf>>) {
    let walker = ignore::WalkBuilder::new(abs_dir)
        .max_depth(Some(1))
        .sort_by_file_path(|a, b| a.cmp(b))
        .build();

    let mut subdirs: Vec<PathBuf> = Vec::new();
    let mut files_by_role: HashMap<FileRole, Vec<PathBuf>> = HashMap::new();

    for entry in walker.flatten() {
        let abs_path = entry.into_path();
        if abs_path == abs_dir {
            continue;
        }

        let relative = abs_path.strip_prefix(root).unwrap_or(&abs_path);

        if abs_path.is_dir() {
            if !classify::is_vendored_or_fixture(relative) {
                subdirs.push(abs_path);
            }
        } else if abs_path.is_file()
            && classify::is_source_file(&abs_path)
            && !classify::is_vendored_or_fixture(relative)
        {
            let role = FileRole::from_path(relative);
            files_by_role.entry(role).or_default().push(abs_path);
        }
    }

    subdirs.sort();
    (subdirs, files_by_role)
}

/// Create FilesGroups from a files-by-role map, partitioning by per-file
/// modifier properties (D7: split when items would be prioritized differently).
pub fn create_files_groups<'s>(
    dir: &Path,
    files_by_role: HashMap<FileRole, Vec<PathBuf>>,
    inherited_modifier: f64,
    root: &Path,
) -> Vec<Group<'s>> {
    let mut result: Vec<Group<'s>> = Vec::new();
    let mut sorted_roles: Vec<_> = files_by_role.into_iter().collect();
    sorted_roles.sort_by_key(|(role, _)| *role);
    let is_root_dir = dir == root;

    let dir_has_headers = sorted_roles.iter().any(|(_, files)| {
        files.iter().any(|f| classify::is_header_file(f))
    });

    for (role, files) in sorted_roles {
        let mut partitions: HashMap<(bool, bool, bool, bool), Vec<PathBuf>> = HashMap::new();
        for file_path in files {
            let relative = file_path.strip_prefix(root).unwrap_or(&file_path);
            let props = classify::file_modifier_properties(relative);
            partitions.entry(props).or_default().push(file_path);
        }

        let mut sorted_partitions: Vec<_> = partitions.into_iter().collect();
        sorted_partitions.sort_by_key(|e| e.0);

        for ((is_deprioritized, is_type_declaration, is_header, is_test_file), mut part_files) in
            sorted_partitions
        {
            part_files.sort();

            let has_companion_header = !is_header
                && dir_has_headers
                && part_files.iter().any(|f| classify::is_c_implementation_file(f));

            let contribution = heuristics::files_contribution(
                role,
                is_root_dir,
                is_deprioritized,
                is_type_declaration,
                is_header,
                is_test_file,
                has_companion_header,
            );

            result.push(Group::Files(FilesGroup {
                parent_dir: dir.to_path_buf(),
                role,
                items: part_files,
                inherited_modifier: inherited_modifier * contribution,
                is_deprioritized,
                is_type_declaration,
                is_header,
                is_test_file,
            }));
        }
    }
    result
}

/// Produce children when a FoldersGroup is scheduled.
/// Walks each item directory one level and classifies entries.
pub fn children<'s>(g: &mut FoldersGroup, ctx: &ScheduleCtx<'s>) -> Vec<Group<'s>> {
    let mut result: Vec<Group<'s>> = Vec::new();

    for item_dir in &g.items {
        let rel_dir = ctx.rel_path(item_dir);
        let category = classify::classify_dir(rel_dir);
        let contribution = heuristics::folders_contribution(rel_dir, category);
        let child_modifier = g.inherited_modifier * contribution;

        let (subdirs, files_by_role) = walk_dir_entries(item_dir, &ctx.root);

        if !subdirs.is_empty() {
            result.push(Group::Folders(FoldersGroup {
                parent_dir: item_dir.clone(),
                items: subdirs,
                inherited_modifier: child_modifier,
                cached_item_costs: None,
            }));
        }

        result.extend(create_files_groups(
            item_dir,
            files_by_role,
            child_modifier,
            &ctx.root,
        ));
    }

    debug_assert!(
        result.iter().all(|g| match g {
            Group::Folders(f) => !f.items.is_empty(),
            Group::Files(f) => !f.items.is_empty(),
            Group::Ts(t) => !t.items.is_empty(),
        }),
        "D3: children() produced an empty group"
    );

    result
}
