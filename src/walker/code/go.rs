//! Go extraction. Functions and methods are `Callable`, except the
//! program flow of `package main` (see `show_program_flow`); `type`,
//! `const` and `var` declarations are `Whole`, and a grouped `( … )`
//! declaration lists its opening row and one roster row per spec, so a
//! spec never shows without its keyword. The module doc is the
//! package comment. A `//go:build` constraint joins the roster as a
//! re-export row, so a platform variant never lists its declarations
//! without their condition. A file carrying the generated-code banner,
//! or excluded from every build by `//go:build ignore` (a generator or
//! demo program), yields no declarations. Outside `package main`, what no importer can
//! name (a lower-case declaration, spec, field or interface method, or a
//! method on a lower-case type) is hidden, unless its file, or its
//! struct, exports nothing.

use std::collections::HashSet;
use std::sync::LazyLock;

use regex::Regex;
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
    sibling_mentions: Some(sibling_mentions),
    sibling_names: Some(declared_names),
};

/// Every identifier the file uses: files of one package share a
/// namespace, so a file depends on a sibling by naming what it declares.
fn sibling_mentions(file: &SourceFile) -> HashSet<String> {
    static IDENTIFIER: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\b[A-Za-z_]\w*").unwrap());
    IDENTIFIER
        .find_iter(&file.source)
        .map(|word| word.as_str().to_owned())
        .collect()
}

/// The top-level functions, methods, types, constants and variables the
/// file declares.
fn declared_names(file: &SourceFile) -> Vec<String> {
    let root = file.tree.root_node();
    let mut specs = Vec::new();
    let mut cursor = root.walk();
    for child in root.named_children(&mut cursor) {
        match child.kind() {
            "function_declaration" | "method_declaration" => specs.push(child),
            "type_declaration" | "const_declaration" | "var_declaration" => {
                let mut inner = child.walk();
                for spec in child.named_children(&mut inner) {
                    if spec.kind().ends_with("_list") {
                        let mut list = spec.walk();
                        specs.extend(spec.named_children(&mut list));
                    } else {
                        specs.push(spec);
                    }
                }
            }
            _ => {}
        }
    }
    let mut names = Vec::new();
    for spec in specs {
        let mut inner = spec.walk();
        names.extend(
            spec.children_by_field_name("name", &mut inner)
                .map(|name| file.text(name))
                .map(str::to_owned),
        );
    }
    names
}

fn extract(file: &SourceFile, _ctx: &WalkCtx) -> FileModel {
    let root = file.tree.root_node();
    let mut model = FileModel::default();
    if is_generated(root, file) {
        return model;
    }
    let mut is_program = false;
    let mut api_only = false;
    let mut functions = Vec::new();
    let exports_something = root
        .named_children(&mut root.walk())
        .any(|child| match child.kind() {
            "function_declaration" | "method_declaration" => is_exported_callable(child, file),
            "type_declaration" | "const_declaration" | "var_declaration" => {
                whole(child, file, true).is_some()
            }
            _ => false,
        });
    let mut cursor = root.walk();
    for child in root.named_children(&mut cursor) {
        let decl = match child.kind() {
            "comment" if file.text(child).starts_with("//go:build") => {
                if file.text(child)["//go:build".len()..].trim() == "ignore" {
                    return FileModel::default();
                }
                model.reexports.push(Item::new(file.node_rows(child)));
                continue;
            }
            "package_clause" => {
                (model.module_doc, _) = doc_and_directives(child, file);
                let mut inner = child.walk();
                is_program = child
                    .named_children(&mut inner)
                    .any(|name| name.kind() == "package_identifier" && file.text(name) == "main");
                api_only = !is_program && exports_something;
                continue;
            }
            "function_declaration" if is_program => {
                let decl = callable(child, file);
                if decl.is_some() {
                    functions.push(ProgramFunction::new(model.decls.len(), child, file));
                }
                decl
            }
            "function_declaration" | "method_declaration"
                if !api_only || is_exported_callable(child, file) =>
            {
                callable(child, file)
            }
            "type_declaration" | "const_declaration" | "var_declaration" => {
                whole(child, file, api_only)
            }
            _ => continue,
        };
        model.decls.extend(decl);
    }
    show_program_flow(&mut model.decls, &functions);
    model
}

/// Whether a comment before the package clause is Go's generated-code
/// banner, `// Code generated … DO NOT EDIT.`: machine output such as
/// parser tables and conversion code, which nobody reads as the
/// package's source.
fn is_generated(root: Node, file: &SourceFile) -> bool {
    let mut cursor = root.walk();
    root.named_children(&mut cursor)
        .take_while(|child| child.kind() == "comment")
        .any(|comment| {
            let text = file.text(comment).trim_end();
            text.starts_with("// Code generated ") && text.ends_with(" DO NOT EDIT.")
        })
}

/// Whether an identifier is visible outside its package: it starts with an
/// upper-case letter.
fn is_exported(name: &str) -> bool {
    name.chars().next().is_some_and(char::is_uppercase)
}

/// A function is exported by its name; a method also needs an exported
/// receiver type, or no importer can name it.
fn is_exported_callable(node: Node, file: &SourceFile) -> bool {
    let named = |node: Option<Node>| node.is_some_and(|name| is_exported(file.text(name)));
    named(node.child_by_field_name("name"))
        && node.child_by_field_name("receiver").is_none_or(|receiver| {
            let mut cursor = receiver.walk();
            let parameter = receiver.named_children(&mut cursor).next();
            named(
                parameter
                    .and_then(|parameter| base_type_name(parameter.child_by_field_name("type")?)),
            )
        })
}

/// The type name under pointers, parentheses, type arguments and a
/// package qualifier: `T` of `*pkg.T[K]`.
fn base_type_name(node: Node) -> Option<Node> {
    match node.kind() {
        "type_identifier" => Some(node),
        "qualified_type" => base_type_name(node.child_by_field_name("name")?),
        "generic_type" => base_type_name(node.child_by_field_name("type")?),
        "pointer_type" | "parenthesized_type" => base_type_name(node.named_child(0)?),
        _ => None,
    }
}

/// Whether a spec or struct field declares an exported name, or embeds
/// an exported type.
fn declares_exported(node: Node, file: &SourceFile) -> bool {
    let mut cursor = node.walk();
    let mut names = node.children_by_field_name("name", &mut cursor).peekable();
    if names.peek().is_none() {
        return node
            .child_by_field_name("type")
            .and_then(base_type_name)
            .is_some_and(|name| is_exported(file.text(name)));
    }
    names.any(|name| is_exported(file.text(name)))
}

/// The comments directly above `node`: its doc, one [`Item`] per
/// paragraph, and the rows of its directives. A Go doc separates
/// paragraphs with a bare `//` row, which stays with the paragraph above
/// it. A directive (`//go:embed f`, `//go:linkname x`, `//export f`)
/// instructs the toolchain and belongs with the declaration's head, as
/// an attribute would.
fn doc_and_directives(node: Node, file: &SourceFile) -> (Vec<Item>, Vec<usize>) {
    let mut items: Vec<Item> = Vec::new();
    let mut directives = Vec::new();
    let mut after_break = true;
    for row in file.comments_above(node, 1, |_| true).into_iter().flatten() {
        let text = file.line(row).trim();
        if is_directive(text) {
            directives.push(row);
            continue;
        }
        let is_break = text.is_empty() || text == "//";
        match items.last_mut() {
            Some(item) if is_break || !after_break => item.rows.push(row),
            _ => items.push(Item::new([row])),
        }
        after_break = is_break;
    }
    (items, directives)
}

/// Go's directive comment syntax: `//line `, `//extern `, `//export `,
/// or `//` then `[a-z0-9]+:[a-z0-9]` (`//go:noinline`, `//nolint:errcheck`).
fn is_directive(text: &str) -> bool {
    let Some(rest) = text.strip_prefix("//") else {
        return false;
    };
    if ["line ", "extern ", "export "]
        .iter()
        .any(|prefix| rest.starts_with(prefix))
    {
        return true;
    }
    let is_word = |c: char| c.is_ascii_lowercase() || c.is_ascii_digit();
    rest.split_once(':').is_some_and(|(namespace, after)| {
        !namespace.is_empty() && namespace.chars().all(is_word) && after.starts_with(is_word)
    })
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
    let (doc, directives) = doc_and_directives(node, file);
    Some(DeclInfo {
        name_rows: vec![start],
        head: directives.into_iter().chain(start..=head_end).collect(),
        doc,
        body,
        shape: Shape::Callable,
        members: Vec::new(),
    })
}

/// `type`, `const` or `var`: a grouped declaration's body is its specs;
/// a single struct or interface type's body is its fields / methods. The
/// head is every row outside the span of the body items. With
/// `api_only`, unexported specs, fields and interface methods are left
/// out, and so is a declaration with nothing exported.
fn whole(node: Node, file: &SourceFile, api_only: bool) -> Option<DeclInfo> {
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
    let visible = |node: &Node| !api_only || declares_exported(*node, file);
    let (name_rows, entries) = match group {
        Some(group) => {
            let spec_rows: Vec<usize> = specs
                .iter()
                .filter(|spec| visible(spec))
                .map(|spec| *file.node_rows(*spec).start())
                .collect();
            let name_rows = if spec_rows.is_empty() {
                spec_rows
            } else {
                [vec![start], spec_rows].concat()
            };
            (name_rows, Some(group))
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
            let name_rows = if specs.first().is_some_and(visible) {
                vec![start]
            } else {
                Vec::new()
            };
            (name_rows, field_list)
        }
    };
    if name_rows.is_empty() {
        return None;
    }
    let entries: Vec<Node> = entries.map_or_else(Vec::new, |list| {
        let mut inner = list.walk();
        list.named_children(&mut inner).collect()
    });
    let mut body = file.node_items(entries.iter().copied(), start);
    let (doc, mut head) = doc_and_directives(node, file);
    head.extend(match (body.first(), body.last()) {
        (Some(first), Some(last)) => (start..first.rows[0])
            .chain(last.rows[last.rows.len() - 1] + 1..=*rows.end())
            .collect::<Vec<_>>(),
        _ => rows.clone().collect(),
    });
    let end_rows = |shown: bool| -> Vec<usize> {
        entries
            .iter()
            .filter(|entry| {
                is_spec(entry) || matches!(entry.kind(), "field_declaration" | "method_elem")
            })
            .filter(|entry| visible(entry) == shown)
            .map(|entry| *file.node_rows(*entry).end())
            .collect()
    };
    let (hidden, shown) = (end_rows(false), end_rows(true));
    let hidden = if shown.is_empty() { Vec::new() } else { hidden };
    body.retain(|item| {
        let last = item.rows[item.rows.len() - 1];
        !hidden.contains(&last) || shown.contains(&last)
    });
    Some(DeclInfo {
        name_rows,
        head,
        doc,
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

func Helper() int { return 1 }

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
\tB
)

var x, Y = 1, 2
",
        );
        let group = &model.decls[0];
        assert_eq!(group.name_rows, [3, 5, 6]);
        assert_eq!(group.head, [3, 7]);
        assert_eq!(body_rows(group), [vec![4, 5], vec![6]]);
        let single = &model.decls[1];
        assert_eq!((single.head.as_slice(), single.body.len()), (&[9][..], 0));
    }

    /// Outside `package main`, a declaration, spec, field or interface
    /// method no importer can name is hidden, unless its file (or struct)
    /// exports nothing.
    #[test]
    fn go_library_hides_unexported_declarations() {
        let model = extract_source(
            "\
package http

type conn struct {
\tserver *Server
}

type Server struct {
\tAddr string
\tmu   sync.Mutex
\t*Logger
}

type Handler interface {
\tServeHTTP(ResponseWriter, *Request)
\tprivate()
}

type Digest struct {
\tv1 uint64
}

var (
\tErrClosed = errors.New(\"closed\")
\terrShort  = errors.New(\"short\")
)

var _ Handler = (*Server)(nil)

func (c *conn) Serve() {}

func (s *Server) Close() error { return nil }

func (s *Server) listen() {}

func ListenAndServe() {}

func newConn() *conn { return nil }
",
        );
        let roster: Vec<Vec<usize>> = model
            .decls
            .iter()
            .map(|decl| decl.name_rows.clone())
            .collect();
        assert_eq!(
            roster,
            [
                vec![7],
                vec![13],
                vec![18],
                vec![22, 23],
                vec![31],
                vec![35]
            ]
        );
        assert_eq!(body_rows(&model.decls[0]), [vec![8], vec![10]]);
        assert_eq!(body_rows(&model.decls[1]), [vec![14]]);
        assert_eq!(body_rows(&model.decls[2]), [vec![19]]);
        assert_eq!(body_rows(&model.decls[3]), [vec![23]]);
        let internal = extract_source("package cmd\n\nvar rootCmd = 1\n\nfunc run() {}\n");
        assert_eq!(internal.decls.len(), 2);
    }

    /// A bare `//` row splits a doc, so its summary can show alone.
    #[test]
    fn go_doc_splits_into_paragraphs_at_bare_comment_rows() {
        let model = extract_source(
            "\
// Package p does X.
//
// More about p.
package p

// Foo does X.
//
// Long second paragraph
// continues here.
func Foo() {}
",
        );
        assert_eq!(model.module_doc, [Item::new([1, 2]), Item::new([3])]);
        assert_eq!(model.decls[0].doc, [Item::new([6, 7]), Item::new([8, 9])]);
    }

    /// A directive stays with its declaration even when the doc is hidden.
    #[test]
    fn go_directives_are_head_not_doc() {
        let model = extract_source(
            "\
//go:generate mockery
package ov

// configTemplate is the default configuration.
//
//go:embed ov.yaml.template
var ConfigTemplate string

//go:nosplit
func Gosched() {
\tmcall(gosched_m)
}
",
        );
        assert!(model.module_doc.is_empty());
        let template = &model.decls[0];
        assert_eq!(template.doc, [Item::new([4, 5])]);
        assert_eq!(template.head, [6, 7]);
        let gosched = &model.decls[1];
        assert!(gosched.doc.is_empty());
        assert_eq!(gosched.head, [9, 10]);
        assert!(is_directive("//nolint:errcheck") && is_directive("//export Add"));
        assert!(!is_directive("// Note: prose") && !is_directive("//TODO: fix"));
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
    fn go_generated_file_yields_nothing() {
        let model = extract_source(
            "\
// Code generated by goyacc -l -o parser.go parser.go.y. DO NOT EDIT.

package gojq

const tokAltOp = 57346

var yyExca = [...]int16{ -1, 1, 1, -1, }
",
        );
        assert!(model.module_doc.is_empty() && model.decls.is_empty());
        let after_license = extract_source(
            "// Copyright 2024 The Authors.\n\n// Code generated by conversion-gen. DO NOT EDIT.\n\npackage v1\n\nfunc Convert() {}\n",
        );
        assert!(after_license.decls.is_empty());
        let mentioned_in_doc = extract_source(
            "package p\n\n// Code generated by hand. DO NOT EDIT.\nfunc Hand() {}\n",
        );
        assert_eq!(mentioned_in_doc.decls.len(), 1);
    }

    #[test]
    fn go_build_ignored_program_yields_nothing() {
        let model =
            extract_source("//go:build ignore\n\npackage main\n\nfunc main() {\n\tgenerate()\n}\n");
        assert!(model.decls.is_empty() && model.reexports.is_empty());
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
