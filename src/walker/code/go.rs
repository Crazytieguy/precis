//! Go extraction. Functions and methods are `Callable`, except the
//! program flow of `package main` (see `show_program_flow`); `type`,
//! `const` and `var` declarations are `Whole`, and a grouped `( … )`
//! declaration lists one roster row per spec. The module doc is the
//! package comment. A `//go:build` constraint joins the roster as a
//! re-export row, so a platform variant never lists its declarations
//! without their condition.

use tree_sitter::Node;

use super::model::{DeclInfo, FileModel, Item, Shape};
use super::{Language, ProgramFunction, SourceFile, show_program_flow};
use crate::walker::WalkCtx;

pub(super) const LANGUAGE: Language = Language {
    extensions: &["go"],
    grammar: |_| tree_sitter_go::LANGUAGE.into(),
    extract,
    is_entrypoint: None,
    file_weight: None,
};

fn extract(file: &SourceFile, _ctx: &WalkCtx) -> FileModel {
    let root = file.tree.root_node();
    let mut model = FileModel::default();
    let mut is_program = false;
    let mut functions = Vec::new();
    let mut cursor = root.walk();
    for child in root.named_children(&mut cursor) {
        let decl = match child.kind() {
            "comment" if file.text(child).starts_with("//go:build") => {
                model.reexports.push(Item::new(file.node_rows(child)));
                continue;
            }
            "package_clause" => {
                model.module_doc = doc_items(child, file);
                let mut inner = child.walk();
                is_program = child
                    .named_children(&mut inner)
                    .any(|name| name.kind() == "package_identifier" && file.text(name) == "main");
                continue;
            }
            "function_declaration" if is_program => {
                let decl = callable(child, file);
                if decl.is_some() {
                    functions.push(ProgramFunction::new(model.decls.len(), child, file));
                }
                decl
            }
            "function_declaration" | "method_declaration" => callable(child, file),
            "type_declaration" | "const_declaration" | "var_declaration" => whole(child, file),
            _ => continue,
        };
        model.decls.extend(decl);
    }
    show_program_flow(&mut model.decls, &functions);
    model
}

fn doc_items(node: Node, file: &SourceFile) -> Vec<Item> {
    file.paragraphs(file.comment_rows_above(node))
}

/// A function or method: the head runs through the row opening its
/// body, or is every row when the body has no statement.
fn callable(node: Node, file: &SourceFile) -> Option<DeclInfo> {
    node.child_by_field_name("name")?;
    let rows = file.node_rows(node);
    let start = *rows.start();
    let block = node.child_by_field_name("body");
    let open_row = block.map_or(start, |block| *file.node_rows(block).start());
    let mut statements = Vec::new();
    if let Some(block) = block {
        let mut cursor = block.walk();
        for child in block.named_children(&mut cursor) {
            if child.kind() == "statement_list" {
                let mut inner = child.walk();
                statements.extend(child.named_children(&mut inner));
            } else {
                statements.push(child);
            }
        }
    }
    let body = file.node_items(statements, open_row);
    let head_end = if body.is_empty() {
        *rows.end()
    } else {
        open_row
    };
    Some(DeclInfo {
        name_rows: vec![start],
        head: (start..=head_end).collect(),
        doc: doc_items(node, file),
        body,
        shape: Shape::Callable,
        members: Vec::new(),
    })
}

/// `type`, `const` or `var`: a grouped declaration's body is its specs;
/// a single struct or interface type's body is its fields / methods. The
/// head is every row outside the span of the body items.
fn whole(node: Node, file: &SourceFile) -> Option<DeclInfo> {
    let rows = file.node_rows(node);
    let start = *rows.start();
    let mut specs = Vec::new();
    let mut group = None;
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        match child.kind() {
            "(" => group = Some(node),
            "var_spec_list" => {
                group = Some(child);
                let mut inner = child.walk();
                specs.extend(child.named_children(&mut inner).filter(is_spec));
            }
            _ if is_spec(&child) => specs.push(child),
            _ => {}
        }
    }
    let (name_rows, body) = match group {
        Some(group) => {
            let mut inner = group.walk();
            let body = file.node_items(group.named_children(&mut inner), start);
            let name_rows = specs
                .iter()
                .map(|spec| *file.node_rows(*spec).start())
                .collect();
            (name_rows, body)
        }
        None => {
            let field_list = specs
                .first()
                .and_then(|spec| spec.child_by_field_name("type"))
                .and_then(|ty| match ty.kind() {
                    "struct_type" => ty
                        .named_children(&mut ty.walk())
                        .find(|child| child.kind() == "field_declaration_list"),
                    "interface_type" => Some(ty),
                    _ => None,
                });
            let body = field_list.map_or_else(Vec::new, |list| {
                let mut inner = list.walk();
                file.node_items(list.named_children(&mut inner), start)
            });
            (vec![start], body)
        }
    };
    if name_rows.is_empty() {
        return None;
    }
    let head = match (body.first(), body.last()) {
        (Some(first), Some(last)) => (start..first.rows[0])
            .chain(last.rows[last.rows.len() - 1] + 1..=*rows.end())
            .collect(),
        _ => rows.collect(),
    };
    Some(DeclInfo {
        name_rows,
        head,
        doc: doc_items(node, file),
        body,
        shape: Shape::Whole,
        members: Vec::new(),
    })
}

fn is_spec(node: &Node) -> bool {
    matches!(
        node.kind(),
        "type_spec" | "type_alias" | "const_spec" | "var_spec"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn extract_source(source: &str) -> FileModel {
        super::super::test_support::extract_source(&LANGUAGE, "a.go", source)
    }

    fn body_rows(decl: &DeclInfo) -> Vec<Vec<usize>> {
        super::super::test_support::rows(&decl.body)
    }

    #[test]
    fn go_splits_functions_into_signature_doc_and_statements() {
        let model = extract_source(
            "\
// Package foo does things.
package foo

// Run runs.
func (s *Server) Run(
\tctx context.Context,
) error {
\t// start
\ts.start() // now
\treturn nil
}

func helper() int { return 1 }

func Exported(
\tx int,
) int { return x
\t_ = x
}
",
        );
        assert_eq!(model.module_doc, [Item::new([1])]);
        let run = &model.decls[0];
        assert_eq!(
            (run.head.clone(), run.name_rows.clone()),
            (vec![5, 6, 7], vec![5])
        );
        assert_eq!(run.doc, [Item::new([4])]);
        assert_eq!(body_rows(run), [vec![8, 9], vec![10]]);
        let helper = &model.decls[1];
        assert_eq!((helper.head.as_slice(), helper.body.len()), (&[13][..], 0));
        let shared_open_row = &model.decls[2];
        assert_eq!(shared_open_row.head, [15, 16, 17]);
        assert_eq!(body_rows(shared_open_row), [vec![18]]);
    }

    #[test]
    fn go_grouped_declarations_list_one_row_per_spec() {
        let model = extract_source(
            "\
package foo

const (
\t// A is a.
\tA = iota
\tb
)

var x, Y = 1, 2
",
        );
        let group = &model.decls[0];
        assert_eq!(group.name_rows, [5, 6]);
        assert_eq!(group.head, [3, 7]);
        assert_eq!(body_rows(group), [vec![4, 5], vec![6]]);
        let single = &model.decls[1];
        assert_eq!((single.head.as_slice(), single.body.len()), (&[9][..], 0));
    }

    #[test]
    fn go_build_constraint_joins_the_roster() {
        let model = extract_source(
            "\
//go:build linux && !purego

// Package foo does things.
package foo

func Sum() {}
",
        );
        assert_eq!(model.reexports, [Item::new([1])]);
        assert_eq!(model.module_doc, [Item::new([3])]);
    }

    #[test]
    fn go_struct_body_is_its_fields() {
        let model = extract_source(
            "\
package main

type config struct {
\tName string

\t// Port to bind.
\tPort int
}

func (c *config) load() {}
",
        );
        let config = &model.decls[0];
        assert_eq!(
            (config.head.clone(), config.shape),
            (vec![3, 8], Shape::Whole)
        );
        assert_eq!(body_rows(config), [vec![4], vec![6, 7]]);
    }

    #[test]
    fn go_program_main_renders_with_its_body() {
        let model = extract_source(
            "\
package main

func main() {
\tflag.Parse()
\tsetup()
\tserve()
}

func setup() {}
",
        );
        let shapes: Vec<Shape> = model.decls.iter().map(|decl| decl.shape).collect();
        assert_eq!(shapes, [Shape::Whole, Shape::Callable]);
        assert_eq!(model.decls[0].head, [3, 7]);
        assert_eq!(body_rows(&model.decls[0]), [vec![4], vec![5], vec![6]]);
    }
}
