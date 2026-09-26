//! Go extraction. Functions and methods are `Callable`, except the
//! program flow of `package main` (see `show_program_flow`); `type`,
//! `const` and `var` declarations are `Whole`, and a grouped `( … )`
//! declaration lists its opening row and one roster row per spec, so a
//! spec never shows without its keyword. The module doc is the
//! package comment. A `//go:build` constraint joins the roster as a
//! re-export row, so a platform variant never lists its declarations
//! without their condition. A file carrying the generated-code banner,
//! or excluded from every build by `//go:build ignore` (a generator or
//! demo program), yields no declarations. Outside `package main`, what
//! no importer can name (a lower-case declaration, spec, field or
//! interface method, or a method on a lower-case type no exported
//! function returns) is hidden, unless its file, or its struct, exports
//! nothing.

use std::collections::HashSet;
use std::sync::LazyLock;

use regex::Regex;
use tree_sitter::Node;

use super::model::{DeclInfo, FileModel, Item};
use super::{Language, ProgramFunction, SourceFile, named_children, show_program_flow};
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
    let mut names = Vec::new();
    for declaration in top_level_declarations(file.tree.root_node()) {
        names.extend(
            declaration
                .children_by_field_name("name", &mut declaration.walk())
                .map(|name| file.text(name).to_owned()),
        );
    }
    names
}

/// The file's top-level functions, methods and specs.
fn top_level_declarations(root: Node) -> Vec<Node> {
    let mut declarations = Vec::new();
    for child in root.named_children(&mut root.walk()) {
        match child.kind() {
            "function_declaration" | "method_declaration" => declarations.push(child),
            "type_declaration" | "const_declaration" | "var_declaration" => {
                declarations.extend(specs(child).0);
            }
            _ => {}
        }
    }
    declarations
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
    let declarations = top_level_declarations(root);
    let handed_out = handed_out_types(&declarations, file);
    let exports_something = declarations.iter().any(|declaration| {
        if matches!(
            declaration.kind(),
            "function_declaration" | "method_declaration"
        ) {
            is_reachable_callable(*declaration, file, &handed_out)
        } else {
            declares_exported(*declaration, file)
        }
    });
    for child in root.named_children(&mut root.walk()) {
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
                is_program = child
                    .named_children(&mut child.walk())
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
                if !api_only || is_reachable_callable(child, file, &handed_out) =>
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
    root.named_children(&mut root.walk())
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

/// An exported function, or an exported method whose receiver type an
/// importer can reach: an exported type, or one in `handed_out`.
fn is_reachable_callable(node: Node, file: &SourceFile, handed_out: &[&str]) -> bool {
    declares_exported(node, file)
        && node.child_by_field_name("receiver").is_none_or(|receiver| {
            let parameter = receiver.named_children(&mut receiver.walk()).next();
            parameter
                .and_then(|parameter| base_type_name(parameter.child_by_field_name("type")?))
                .is_some_and(|name| {
                    let name = file.text(name);
                    is_exported(name) || handed_out.contains(&name)
                })
        })
}

/// The type names in the results of the file's exported functions and
/// methods: an importer calls methods on what a constructor returns even
/// when it cannot name the type.
fn handed_out_types<'a>(declarations: &[Node], file: &'a SourceFile) -> Vec<&'a str> {
    let mut names = Vec::new();
    for declaration in declarations {
        if !matches!(
            declaration.kind(),
            "function_declaration" | "method_declaration"
        ) || !declares_exported(*declaration, file)
        {
            continue;
        }
        let mut pending: Vec<Node> = declaration
            .child_by_field_name("result")
            .into_iter()
            .collect();
        while let Some(node) = pending.pop() {
            if node.kind() == "type_identifier" {
                names.push(file.text(node));
            }
            pending.extend(node.named_children(&mut node.walk()));
        }
    }
    names
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

/// Whether a function, method, spec or struct field declares an exported
/// name, or a field embeds an exported type.
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
    let (directives, doc): (Vec<usize>, Vec<usize>) = file
        .comments_above(node, 1, |_| true)
        .into_iter()
        .flatten()
        .partition(|&row| is_directive(file.line(row).trim()));
    let doc = file.paragraphs_by(doc, |line| matches!(line.trim(), "" | "//"));
    (doc, directives)
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
    let statements = named_children(block).into_iter().flat_map(|child| {
        if child.kind() == "statement_list" {
            named_children(Some(child))
        } else {
            vec![child]
        }
    });
    let mut decl = file.callable(vec![start], rows, statements, open_row);
    let (doc, directives) = doc_and_directives(node, file);
    decl.head.extend(directives);
    Some(DeclInfo { doc, ..decl })
}

/// `type`, `const` or `var`: a grouped declaration's body is its specs;
/// a single struct or interface type's body is its fields / methods. The
/// head is every row outside the span of the body items. With
/// `api_only`, unexported specs, fields and interface methods are left
/// out, and so is a declaration with nothing exported.
fn whole(node: Node, file: &SourceFile, api_only: bool) -> Option<DeclInfo> {
    let rows = file.node_rows(node);
    let start = *rows.start();
    let (specs, group) = specs(node);
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
    let mut decl = file.whole(name_rows, rows, entries);
    if let Some(list) = entries {
        let is_entry = |entry: &Node| {
            is_spec(entry) || matches!(entry.kind(), "field_declaration" | "method_elem")
        };
        let shows_some = list
            .named_children(&mut list.walk())
            .any(|entry| is_entry(&entry) && visible(&entry));
        decl.body = file.admitted_items(list, start, |entry| {
            !is_entry(&entry) || !shows_some || visible(&entry)
        });
    }
    let (doc, directives) = doc_and_directives(node, file);
    decl.head.extend(directives);
    Some(DeclInfo { doc, ..decl })
}

/// The specs of a `type`, `const` or `var` declaration, and the node
/// holding them when they are grouped in parentheses.
fn specs(node: Node) -> (Vec<Node>, Option<Node>) {
    let mut specs = Vec::new();
    let mut group = None;
    for child in node.children(&mut node.walk()) {
        match child.kind() {
            "(" => group = Some(node),
            "var_spec_list" => {
                group = Some(child);
                specs.extend(child.named_children(&mut child.walk()).filter(is_spec));
            }
            _ if is_spec(&child) => specs.push(child),
            _ => {}
        }
    }
    (specs, group)
}

fn is_spec(node: &Node) -> bool {
    matches!(
        node.kind(),
        "type_spec" | "type_alias" | "const_spec" | "var_spec"
    )
}

#[cfg(test)]
mod tests {
    use super::super::test_support::describe;
    use super::*;

    fn extract_source(source: &str) -> FileModel {
        super::super::test_support::extract_source(&LANGUAGE, "a.go", source)
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
        assert_eq!(
            describe(&model),
            [
                "Callable name [5] head [5, 6, 7] doc [[4]] body [[8, 9], [10]]",
                "Callable name [13] head [13] doc [] body []",
                "Callable name [15] head [15, 16, 17] doc [] body [[18]]",
            ]
        );
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
        assert_eq!(
            describe(&model),
            [
                "Whole name [3, 5, 6] head [3, 7] doc [] body [[4, 5], [6]]",
                "Whole name [9] head [9] doc [] body []",
            ]
        );
    }

    /// Outside `package main`, a declaration, spec, field or interface
    /// method no importer can name is hidden, unless its file (or struct)
    /// exports nothing. An exported method on an unexported type stays
    /// when an exported function returns that type.
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

type pool struct{}

func (p *pool) Get() {}

func NewPool() (*pool, error) { return nil, nil }
",
        );
        assert_eq!(
            describe(&model),
            [
                "Whole name [7] head [7, 11] doc [] body [[8], [10]]",
                "Whole name [13] head [13, 16] doc [] body [[14]]",
                "Whole name [18] head [18, 20] doc [] body [[19]]",
                "Whole name [22, 23] head [22, 25] doc [] body [[23]]",
                "Callable name [31] head [31] doc [] body []",
                "Callable name [35] head [35] doc [] body []",
                "Callable name [41] head [41] doc [] body []",
                "Callable name [43] head [43] doc [] body []",
            ]
        );
        let internal = extract_source("package cmd\n\nvar rootCmd = 1\n\nfunc run() {}\n");
        assert_eq!(internal.decls.len(), 2);
        let methods_on_internal_type = extract_source(
            "//go:build !disabled\n\npackage ops\n\ntype op struct{}\n\nfunc (o *op) Evaluate() bool { return true }\n",
        );
        assert_eq!(methods_on_internal_type.decls.len(), 2);
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
        assert_eq!(
            describe(&model),
            ["Callable name [10] head [10] doc [[6, 7], [8, 9]] body []"]
        );
    }

    /// A directive stays with its declaration even when the doc is hidden;
    /// a build constraint joins the roster.
    #[test]
    fn go_directives_are_head_not_doc() {
        let model = extract_source(
            "\
//go:build linux && !purego

// Package ov does things.
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
        assert_eq!(model.reexports, [Item::new([1])]);
        assert_eq!(model.module_doc, [Item::new([3])]);
        assert_eq!(
            describe(&model),
            [
                "Whole name [10] head [9, 10] doc [[7, 8]] body []",
                "Callable name [13] head [12, 13] doc [] body [[14]]",
            ]
        );
        assert!(is_directive("//nolint:errcheck") && is_directive("//export Add"));
        assert!(!is_directive("// Note: prose") && !is_directive("//TODO: fix"));
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
        assert_eq!(
            describe(&model),
            [
                "Whole name [3] head [3, 8] doc [] body [[4], [6, 7]]",
                "Callable name [10] head [10] doc [] body []",
            ]
        );
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
        assert_eq!(
            describe(&model),
            [
                "Whole name [3] head [3, 7] doc [] body [[4], [5], [6]]",
                "Callable name [9] head [9] doc [] body []",
            ]
        );
    }
}
