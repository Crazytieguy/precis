//! TypeScript / JavaScript extraction for the code engine.
//!
//! A file's declarations are its API:
//! - exported declarations and the locals an `export { … }` clause,
//!   `export default X`, `export = X` or a CommonJS `module.exports` /
//!   `exports.x` assignment names; in a file that is one AMD `define` or
//!   UMD wrapper call, the factory body is the top level and its `return`
//!   exports;
//! - every top-level declaration of a `.d.ts` (ambient declarations are
//!   implicitly exported);
//! - every top-level declaration and control-flow statement of an
//!   entrypoint that exports nothing (an application's startup file).
//!
//! Everything else is hidden. `export … from` and the statements that
//! export a name without declaring it are re-exports, listed on the roster.
//!
//! Classes and object-literal values (including the object a call ends
//! its arguments with, `X.extend({ … })`) are containers: methods (and
//! function-valued fields or entries) are members, other fields and
//! entries are body items, and `#name` / `private` / `protected` members
//! are hidden. Interfaces, enums, array literals and namespaces are
//! `Whole` declarations whose entries are body items. Imports and
//! `require` declarations are not modeled.

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;
use std::path::{Path, PathBuf};

use tree_sitter::Node;

use super::model::{DeclInfo, FileModel, Item, Shape};
use super::{
    Language, SourceFile, block_head, file_name, file_stem, is_named_after, named_children,
};
use crate::walker::WalkCtx;

pub(super) const LANGUAGE: Language = Language {
    extensions: &["ts", "tsx", "mts", "cts", "js", "jsx", "mjs", "cjs"],
    grammar,
    extract,
    is_entrypoint: Some(is_entrypoint),
    file_weight: Some(file_weight),
    sibling_mentions: None,
    sibling_names: None,
};

const ENTRYPOINT_STEMS: &[&str] = &["index", "main", "mod", "esm"];

/// The TypeScript grammar for `.ts` / `.mts` / `.cts`; the TSX grammar,
/// which also parses JSX, for everything else.
fn grammar(path: &Path) -> tree_sitter::Language {
    let is_typescript = path
        .extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| {
            ["ts", "mts", "cts"]
                .iter()
                .any(|candidate| extension.eq_ignore_ascii_case(candidate))
        });
    if is_typescript {
        tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()
    } else {
        tree_sitter_typescript::LANGUAGE_TSX.into()
    }
}

fn extract(file: &SourceFile, ctx: &WalkCtx) -> FileModel {
    let root = file.tree.root_node();
    let scope = module_factory_body(file, root).unwrap_or(root);
    let mut cursor = scope.walk();
    let statements: Vec<Node> = scope.named_children(&mut cursor).collect();

    let mut scan = ExportScan::default();
    let classified: Vec<(Node, TopLevel)> = statements
        .iter()
        .map(|&statement| (statement, scan.classify(file, statement)))
        .collect();
    scan.follow_local_factories(file, &classified);
    scan.follow_signature_types(file, &classified);

    let exports_nothing = scan.public_names.is_empty()
        && classified
            .iter()
            .all(|(_, top_level)| !matches!(top_level, TopLevel::Exported(_) | TopLevel::Reexport));
    let is_ambient_file = is_declaration_file(&file.path);
    let is_entry_program = exports_nothing && is_entrypoint(&file.path, ctx);

    let mut model = FileModel::default();
    for (statement, top_level) in classified {
        let node = match top_level {
            TopLevel::Reexport => {
                let comments = doc_items(file, statement)
                    .into_iter()
                    .filter(|doc| file.line(doc.rows[0]).trim_start().starts_with("//"))
                    .flat_map(|doc| doc.rows);
                model
                    .reexports
                    .push(Item::new(comments.chain(file.node_rows(statement))));
                continue;
            }
            TopLevel::Exported(node) => node,
            TopLevel::Local(node) => {
                let exported = declared_names(file, node)
                    .iter()
                    .any(|name| scan.public_names.contains(*name));
                if !exported && !is_ambient_file && !is_entry_program {
                    continue;
                }
                node
            }
            TopLevel::Method { receiver, value }
                if scan.public_names.contains(file.text(receiver)) =>
            {
                value
            }
            TopLevel::Skip if is_entry_program && is_script_statement(statement) => statement,
            TopLevel::Method { .. } | TopLevel::Skip => continue,
        };
        let mut decl = declaration(file, statement, node);
        decl.doc = doc_items(file, statement);
        if !is_internal(file, &decl.doc) {
            model.decls.push(decl);
        }
    }
    model
}

/// The body of the factory function a file that is one AMD
/// `define([…], function (…) { … })` or UMD
/// `(function (root, factory) { … })(this, function (…) { … })` call wraps
/// its code in: the module's real top level, whose `return` exports.
fn module_factory_body<'tree>(file: &SourceFile, root: Node<'tree>) -> Option<Node<'tree>> {
    let mut cursor = root.walk();
    let mut statements = root
        .named_children(&mut cursor)
        .filter(|statement| statement.kind() != "comment" && !is_directive(*statement));
    let (Some(statement), None) = (statements.next(), statements.next()) else {
        return None;
    };
    if statement.kind() != "expression_statement" {
        return None;
    }
    let mut call = statement.named_child(0)?;
    while call.kind() == "parenthesized_expression" {
        call = call.named_child(0)?;
    }
    if call.kind() != "call_expression" {
        return None;
    }
    let mut callee = call.child_by_field_name("function")?;
    while callee.kind() == "parenthesized_expression" {
        callee = callee.named_child(0)?;
    }
    let is_wrapper = (callee.kind() == "identifier" && file.text(callee) == "define")
        || is_function_kind(callee.kind());
    let arguments = call.child_by_field_name("arguments")?;
    let mut cursor = arguments.walk();
    let factory = arguments.named_children(&mut cursor).last()?;
    let body = factory.child_by_field_name("body")?;
    (is_wrapper && is_function_kind(factory.kind()) && body.kind() == "statement_block")
        .then_some(body)
}

/// `'use strict'` and other string-literal statements.
fn is_directive(statement: Node) -> bool {
    statement.kind() == "expression_statement"
        && statement
            .named_child(0)
            .is_some_and(|expression| expression.kind() == "string")
}

/// `index` / `main` / `mod` / `esm` sources within two directories of
/// their package root (the nearest `package.json` directory, else the
/// walk root): `index.ts`, `lib/main.js`, `src/node/index.ts`; and,
/// outside `bin/`, the source named after that package (`lib/express.js`).
fn is_entrypoint(path: &Path, ctx: &WalkCtx) -> bool {
    let (Some(stem), Some(dir)) = (file_stem(path), path.parent()) else {
        return false;
    };
    let package_dir = ctx.code.typescript.package_dir(dir, ctx.root());
    let Ok(relative) = path.strip_prefix(&package_dir) else {
        return false;
    };
    relative.components().count() <= 3
        && (ENTRYPOINT_STEMS.contains(&stem)
            || (is_named_after(path, &package_dir) && !relative.starts_with("bin")))
}

/// Tooling config (`vite.config.ts`, `.eslintrc.js`) is not the
/// project's code.
fn file_weight(path: &Path, _ctx: &WalkCtx) -> f64 {
    if is_config_file(path) {
        CONFIG_FILE_WEIGHT
    } else {
        1.0
    }
}

const CONFIG_FILE_WEIGHT: f64 = 0.001;

fn is_config_file(path: &Path) -> bool {
    file_name(path).is_some_and(|name| name.starts_with('.') || name.contains(".config."))
}

#[derive(Default)]
pub(crate) struct RunState {
    /// Nearest enclosing package directory of each directory asked about.
    package_dirs: RefCell<HashMap<PathBuf, PathBuf>>,
}

impl RunState {
    /// The nearest directory at or above `dir` holding a `package.json`,
    /// stopping at `root`; `root` when there is none.
    fn package_dir(&self, dir: &Path, root: &Path) -> PathBuf {
        if let Some(hit) = self.package_dirs.borrow().get(dir) {
            return hit.clone();
        }
        let package_dir = if dir == root || !dir.starts_with(root) {
            root.to_path_buf()
        } else if dir.join("package.json").is_file() {
            dir.to_path_buf()
        } else {
            dir.parent().map_or_else(
                || root.to_path_buf(),
                |parent| self.package_dir(parent, root),
            )
        };
        self.package_dirs
            .borrow_mut()
            .insert(dir.to_path_buf(), package_dir.clone());
        package_dir
    }
}

fn is_declaration_file(path: &Path) -> bool {
    file_name(path).is_some_and(|name| {
        let name = name.to_ascii_lowercase();
        [".d.ts", ".d.mts", ".d.cts"]
            .iter()
            .any(|suffix| name.ends_with(suffix))
    })
}

/// What one top-level statement contributes to the model.
enum TopLevel<'tree> {
    /// Exposes names without declaring them: `export … from`, an export
    /// clause, `export default X`, `module.exports = X`.
    Reexport,
    /// An exported declaration or exported value, with the node that
    /// decides its shape.
    Exported(Node<'tree>),
    /// A declaration that is API only if something exports its name.
    Local(Node<'tree>),
    /// A value assigned onto a local (`app.use = function …`,
    /// `Router.prototype.route = …`, `Store.VERSION = '1.0'`): part of
    /// that local's API when the local is published.
    Method {
        receiver: Node<'tree>,
        value: Node<'tree>,
    },
    Skip,
}

/// Export facts gathered over the file's top-level statements before any
/// declaration is built.
#[derive(Default)]
struct ExportScan<'source> {
    /// Local names some export statement or CommonJS assignment publishes.
    public_names: HashSet<&'source str>,
}

impl<'source> ExportScan<'source> {
    fn classify<'tree>(
        &mut self,
        file: &'source SourceFile,
        statement: Node<'tree>,
    ) -> TopLevel<'tree> {
        match statement.kind() {
            "export_statement" => {
                if let Some(declaration) = statement.child_by_field_name("declaration") {
                    let declaration = unwrap_ambient(declaration);
                    self.public_names.extend(declared_names(file, declaration));
                    return TopLevel::Exported(declaration);
                }
                if let Some(value) = statement.child_by_field_name("value") {
                    return self.exported_value(file, value);
                }
                if statement.child_by_field_name("source").is_none() {
                    let mut cursor = statement.walk();
                    for child in statement.named_children(&mut cursor) {
                        match child.kind() {
                            "export_clause" => self.mark_clause(file, child),
                            // `export = X`
                            "identifier" => self.mark(file, child),
                            _ => {}
                        }
                    }
                }
                TopLevel::Reexport
            }
            "expression_statement" => {
                if let Some(value) = statement
                    .named_child(0)
                    .and_then(|expression| commonjs_assignment_value(file, expression))
                {
                    return self.exported_value(file, value);
                }
                match statement.named_child(0) {
                    Some(namespace) if namespace.kind() == "internal_module" => {
                        TopLevel::Local(namespace)
                    }
                    Some(assignment) => method_assignment(file, assignment),
                    None => TopLevel::Skip,
                }
            }
            "ambient_declaration" => TopLevel::Local(unwrap_ambient(statement)),
            // A module factory's `return X`.
            "return_statement" => match statement.named_child(0) {
                Some(value) => self.exported_value(file, value),
                None => TopLevel::Skip,
            },
            "lexical_declaration" | "variable_declaration"
                if is_require_declaration(file, statement) =>
            {
                TopLevel::Skip
            }
            kind if is_declaration_kind(kind) => {
                if let Some(declarator) = single_declarator(statement)
                    && let Some(name) = declarator.child_by_field_name("name")
                    && declarator
                        .child_by_field_name("value")
                        .is_some_and(|value| {
                            value.kind() == "assignment_expression"
                                && commonjs_assignment_value(file, value).is_some()
                        })
                {
                    // `var app = exports = module.exports = {}`
                    self.mark(file, name);
                }
                TopLevel::Local(statement)
            }
            _ => TopLevel::Skip,
        }
    }

    /// `export default <value>` or a CommonJS export's right-hand side.
    /// A reference (`x`, `x.y`, `require('x')`), or an object of bare
    /// names, re-exports; anything else is an exported value in its own
    /// right.
    fn exported_value<'tree>(
        &mut self,
        file: &'source SourceFile,
        value: Node<'tree>,
    ) -> TopLevel<'tree> {
        if is_reference(file, value) {
            if value.kind() == "identifier" {
                self.mark(file, value);
            }
            return TopLevel::Reexport;
        }
        if let Some(callee) = callee_identifier(value) {
            self.mark(file, callee);
        }
        if value.kind() == "object" {
            let mut all_names = true;
            let mut cursor = value.walk();
            for entry in value.named_children(&mut cursor) {
                match object_entry_name(entry) {
                    Some(name) => self.mark(file, name),
                    None => all_names &= entry.kind() == "comment",
                }
            }
            if all_names {
                return TopLevel::Reexport;
            }
        }
        TopLevel::Exported(value)
    }

    fn mark_clause(&mut self, file: &'source SourceFile, clause: Node) {
        let mut cursor = clause.walk();
        for specifier in clause.named_children(&mut cursor) {
            if let Some(name) = specifier.child_by_field_name("name") {
                self.mark(file, name);
            }
        }
    }

    fn mark(&mut self, file: &'source SourceFile, name: Node) {
        self.public_names.insert(file.text(name));
    }

    /// A published `const x = make()` / `new X()` is a handle on the
    /// local implementation it calls, so that implementation is
    /// published too (one hop).
    fn follow_local_factories(
        &mut self,
        file: &'source SourceFile,
        classified: &[(Node, TopLevel)],
    ) {
        let mut callees = Vec::new();
        for (_, top_level) in classified {
            let TopLevel::Local(node) = top_level else {
                continue;
            };
            let Some(declarator) = single_declarator(*node) else {
                continue;
            };
            let published = declarator
                .child_by_field_name("name")
                .is_some_and(|name| self.public_names.contains(file.text(name)));
            if let Some(callee) = declarator
                .child_by_field_name("value")
                .and_then(callee_identifier)
                .filter(|_| published)
            {
                callees.push(file.text(callee));
            }
        }
        self.public_names.extend(callees);
    }

    /// A published declaration's signature names the local types a caller
    /// has to know (`props: CardProps`, `extends StateType<T>`), so those
    /// are published too (one hop).
    fn follow_signature_types(
        &mut self,
        file: &'source SourceFile,
        classified: &[(Node, TopLevel)],
    ) {
        let mut types = Vec::new();
        for (_, top_level) in classified {
            let node = match top_level {
                TopLevel::Exported(node) => *node,
                TopLevel::Local(node)
                    if declared_names(file, *node)
                        .iter()
                        .any(|name| self.public_names.contains(name)) =>
                {
                    *node
                }
                _ => continue,
            };
            signature_types(file, node, &mut types);
        }
        self.public_names.extend(types);
    }
}

/// The type names `node`'s parameter lists, return types and
/// `extends` / `implements` clauses mention, outside function and class
/// bodies.
fn signature_types<'source>(file: &'source SourceFile, node: Node, types: &mut Vec<&'source str>) {
    fn walk<'source>(
        file: &'source SourceFile,
        node: Node,
        in_signature: bool,
        types: &mut Vec<&'source str>,
    ) {
        if matches!(node.kind(), "statement_block" | "class_body") {
            return;
        }
        if in_signature && node.kind() == "type_identifier" {
            types.push(file.text(node));
        }
        let in_signature = in_signature
            || matches!(
                node.kind(),
                "formal_parameters"
                    | "extends_clause"
                    | "extends_type_clause"
                    | "implements_clause"
            );
        let return_type = node.child_by_field_name("return_type");
        let mut cursor = node.walk();
        for child in node.named_children(&mut cursor) {
            walk(
                file,
                child,
                in_signature || Some(child) == return_type,
                types,
            );
        }
    }
    walk(file, node, false, types);
}

/// A top-level statement that runs rather than declares: an entry
/// script's control flow (`if (…) …`, `main().catch(…)`), not a
/// directive (`'use strict'`) or an import.
fn is_script_statement(statement: Node) -> bool {
    match statement.kind() {
        "import_statement" | "export_statement" | "empty_statement" => false,
        "expression_statement" => statement
            .named_child(0)
            .is_some_and(|expression| expression.kind() != "string"),
        kind => kind.ends_with("_statement"),
    }
}

fn is_declaration_kind(kind: &str) -> bool {
    matches!(
        kind,
        "function_declaration"
            | "generator_function_declaration"
            | "function_signature"
            | "class_declaration"
            | "abstract_class_declaration"
            | "interface_declaration"
            | "type_alias_declaration"
            | "enum_declaration"
            | "lexical_declaration"
            | "variable_declaration"
    )
}

fn is_function_kind(kind: &str) -> bool {
    matches!(
        kind,
        "function_declaration"
            | "generator_function_declaration"
            | "function_signature"
            | "function_expression"
            | "generator_function"
            | "arrow_function"
    )
}

fn is_class_kind(kind: &str) -> bool {
    matches!(
        kind,
        "class_declaration" | "abstract_class_declaration" | "class"
    )
}

/// The declaration inside `declare …`: a function signature, class,
/// const, namespace or `declare module 'x'` block, or the statement block
/// of `declare global`.
fn unwrap_ambient(node: Node) -> Node {
    if node.kind() != "ambient_declaration" {
        return node;
    }
    let mut cursor = node.walk();
    let inner = node
        .named_children(&mut cursor)
        .find(|child| child.kind() != "comment");
    inner.unwrap_or(node)
}

/// The names a top-level declaration binds.
fn declared_names<'source>(file: &'source SourceFile, node: Node) -> Vec<&'source str> {
    if matches!(node.kind(), "lexical_declaration" | "variable_declaration") {
        let mut cursor = node.walk();
        return node
            .named_children(&mut cursor)
            .filter_map(|declarator| declarator.child_by_field_name("name"))
            .filter(|name| name.kind() == "identifier")
            .map(|name| file.text(name))
            .collect();
    }
    node.child_by_field_name("name")
        .map(|name| file.text(name))
        .into_iter()
        .collect()
}

fn single_declarator(node: Node) -> Option<Node> {
    if !matches!(node.kind(), "lexical_declaration" | "variable_declaration") {
        return None;
    }
    let mut cursor = node.walk();
    let mut declarators = node
        .named_children(&mut cursor)
        .filter(|child| child.kind() == "variable_declarator");
    let first = declarators.next()?;
    declarators.next().is_none().then_some(first)
}

/// `f` in `f(…)` or `new f(…)`.
fn callee_identifier(value: Node) -> Option<Node> {
    let callee = match value.kind() {
        "call_expression" => value.child_by_field_name("function"),
        "new_expression" => value.child_by_field_name("constructor"),
        _ => None,
    }?;
    (callee.kind() == "identifier").then_some(callee)
}

/// The local an object-literal entry names: `a` in `{ a }` or `{ key: a }`.
fn object_entry_name(entry: Node) -> Option<Node> {
    match entry.kind() {
        "shorthand_property_identifier" => Some(entry),
        "pair" => entry
            .child_by_field_name("value")
            .filter(|value| value.kind() == "identifier"),
        _ => None,
    }
}

/// A name, a property path off a name, or a `require` chain.
fn is_reference(file: &SourceFile, value: Node) -> bool {
    match value.kind() {
        "identifier" => true,
        "member_expression" => value
            .child_by_field_name("object")
            .is_some_and(|object| is_reference(file, object)),
        _ => is_require_rooted(file, value),
    }
}

/// `const x = require('y')`, including `require('y').z` and
/// `require('y')(…)` chains.
fn is_require_declaration(file: &SourceFile, node: Node) -> bool {
    let mut cursor = node.walk();
    node.named_children(&mut cursor)
        .filter_map(|declarator| declarator.child_by_field_name("value"))
        .any(|value| is_require_rooted(file, value))
}

fn is_require_rooted(file: &SourceFile, node: Node) -> bool {
    match node.kind() {
        "call_expression" => node.child_by_field_name("function").is_some_and(|callee| {
            (callee.kind() == "identifier" && file.text(callee) == "require")
                || is_require_rooted(file, callee)
        }),
        "member_expression" | "subscript_expression" => node
            .child_by_field_name("object")
            .is_some_and(|object| is_require_rooted(file, object)),
        _ => false,
    }
}

/// The exported value of a CommonJS export assignment: `module.exports =
/// X`, `exports.x = X`, `module.exports.x = X`, `exports['x'] = X`, and
/// chains through a bare `exports` (`exports = module.exports = X`).
fn commonjs_assignment_value<'tree>(
    file: &SourceFile,
    mut expression: Node<'tree>,
) -> Option<Node<'tree>> {
    loop {
        if expression.kind() != "assignment_expression" {
            return None;
        }
        let left = expression.child_by_field_name("left")?;
        let right = expression.child_by_field_name("right")?;
        if is_commonjs_target(file, left) {
            let mut value = right;
            while value.kind() == "assignment_expression" {
                value = value.child_by_field_name("right")?;
            }
            return Some(value);
        }
        if !(left.kind() == "identifier" && file.text(left) == "exports") {
            return None;
        }
        expression = right;
    }
}

/// `R.name = …`, `R.prototype.name = …` or `R.prototype = …`, with `R` a
/// plain name.
fn method_assignment<'tree>(file: &SourceFile, assignment: Node<'tree>) -> TopLevel<'tree> {
    let (Some(left), Some(value)) = (
        assignment.child_by_field_name("left"),
        assignment.child_by_field_name("right"),
    ) else {
        return TopLevel::Skip;
    };
    if assignment.kind() != "assignment_expression"
        || left.kind() != "member_expression"
        || left
            .child_by_field_name("property")
            .is_some_and(|property| file.text(property).starts_with('_'))
    {
        return TopLevel::Skip;
    }
    let mut object = left.child_by_field_name("object");
    if let Some(prototype) = object.filter(|object| {
        object.kind() == "member_expression"
            && object
                .child_by_field_name("property")
                .is_some_and(|property| file.text(property) == "prototype")
    }) {
        object = prototype.child_by_field_name("object");
    }
    match object {
        Some(receiver) if receiver.kind() == "identifier" => TopLevel::Method { receiver, value },
        _ => TopLevel::Skip,
    }
}

fn is_commonjs_target(file: &SourceFile, left: Node) -> bool {
    let is_identifier =
        |node: Node, name: &str| node.kind() == "identifier" && file.text(node) == name;
    let is_module_exports = |node: Node| {
        node.kind() == "member_expression"
            && node
                .child_by_field_name("object")
                .is_some_and(|object| is_identifier(object, "module"))
            && node
                .child_by_field_name("property")
                .is_some_and(|property| file.text(property) == "exports")
    };
    match left.kind() {
        "member_expression" | "subscript_expression" => {
            is_module_exports(left)
                || left.child_by_field_name("object").is_some_and(|object| {
                    is_identifier(object, "exports") || is_module_exports(object)
                })
        }
        _ => false,
    }
}

/// The parts other than the doc of the declaration `statement`
/// introduces, shaped by `node` (the declaration or exported value inside
/// it). The head starts at `statement`, so it carries `export`, `declare`
/// and decorators.
fn declaration(file: &SourceFile, statement: Node, node: Node) -> DeclInfo {
    let rows = file.node_rows(statement);
    let name_row = name_row(node).unwrap_or(*rows.start());
    let kind = node.kind();
    if is_function_kind(kind) {
        return function_callable(file, rows, name_row, node);
    }
    if is_class_kind(kind) {
        return class(file, rows, name_row, node.child_by_field_name("body"));
    }
    match kind {
        "interface_declaration" | "enum_declaration" | "internal_module" | "module" => {
            file.whole(vec![name_row], rows, node.child_by_field_name("body"))
        }
        "type_alias_declaration" => {
            let object = node
                .child_by_field_name("value")
                .filter(|value| value.kind() == "object_type");
            file.whole(vec![name_row], rows, object)
        }
        // `declare global { … }`
        "statement_block" => file.whole(vec![*rows.start()], rows, Some(node)),
        "lexical_declaration" | "variable_declaration" => match single_declarator(node) {
            Some(declarator) => match declarator.child_by_field_name("value") {
                Some(value) => value_declaration(file, rows, name_row, value),
                None => file.whole(vec![name_row], rows, None),
            },
            None => {
                let name_rows = named_children(Some(node))
                    .into_iter()
                    .filter_map(|declarator| declarator.child_by_field_name("name"))
                    .map(|name| name.start_position().row + 1)
                    .collect();
                file.whole(name_rows, rows, None)
            }
        },
        _ => value_declaration(file, rows, name_row, node),
    }
}

/// A declaration shaped by the value it binds: a function (possibly
/// wrapped, `memo(forwardRef(() => { … }))`) is `Callable`; a class is a
/// container, as is an object literal, bare or ending a call's
/// arguments; an array literal lists its entries; anything else is all
/// head. An assignment chain (`var X = exports.X = function …`) is shaped
/// by its final value.
fn value_declaration(
    file: &SourceFile,
    rows: RangeInclusive<usize>,
    name_row: usize,
    value: Node,
) -> DeclInfo {
    let mut value = value;
    while value.kind() == "assignment_expression"
        && let Some(right) = value.child_by_field_name("right")
    {
        value = right;
    }
    if is_class_kind(value.kind()) {
        return class(file, rows, name_row, value.child_by_field_name("body"));
    }
    if value.kind() == "object" {
        return class(file, rows, name_row, Some(value));
    }
    if value.kind() == "array" {
        return file.whole(vec![name_row], rows, Some(value));
    }
    if let Some(function) = wrapped_function(value) {
        return function_callable(file, rows, name_row, function);
    }
    match last_object_argument(value) {
        Some(object) => class(file, rows, name_row, Some(object)),
        None => file.whole(vec![name_row], rows, None),
    }
}

/// The object literal a call or `new` ends its arguments with:
/// `z.object({ … })`, `createTheme(base, { … })`, `new Store({ … })`,
/// also through chained calls (`z.object({ … }).strict()`).
fn last_object_argument(value: Node) -> Option<Node> {
    let mut call = value;
    loop {
        let arguments = match call.kind() {
            "call_expression" | "new_expression" => call.child_by_field_name("arguments")?,
            _ => return None,
        };
        let mut cursor = arguments.walk();
        let last = arguments
            .named_children(&mut cursor)
            .filter(|argument| argument.kind() != "comment")
            .last();
        if let Some(object) = last.filter(|last| last.kind() == "object") {
            return Some(object);
        }
        call = call
            .child_by_field_name("function")
            .filter(|callee| callee.kind() == "member_expression")?
            .child_by_field_name("object")?;
    }
}

/// The statement block of a function value, seen through [`wrapped_function`].
fn wrapped_function_block(value: Node) -> Option<Node> {
    wrapped_function(value)?
        .child_by_field_name("body")
        .filter(|body| body.kind() == "statement_block")
}

/// A function value, seen through parentheses and the first argument of
/// wrapper calls (`memo(forwardRef((p, r) => {…}))`).
fn wrapped_function(value: Node) -> Option<Node> {
    let mut node = value;
    loop {
        if is_function_kind(node.kind()) {
            return Some(node);
        }
        node = match node.kind() {
            "call_expression" => node.child_by_field_name("arguments")?.named_child(0)?,
            "parenthesized_expression" => node.named_child(0)?,
            _ => return None,
        };
    }
}

fn name_row(node: Node) -> Option<usize> {
    let name = node.child_by_field_name("name").or_else(|| {
        single_declarator(node).and_then(|declarator| declarator.child_by_field_name("name"))
    })?;
    Some(name.start_position().row + 1)
}

/// A function's parts: [`callable`] over its statement block or, for an
/// arrow function whose body is an expression, the head through the `=>`
/// row and the rest of the expression as one body item.
fn function_callable(
    file: &SourceFile,
    rows: RangeInclusive<usize>,
    name_row: usize,
    function: Node,
) -> DeclInfo {
    let body = function.child_by_field_name("body");
    if body.is_none_or(|body| body.kind() == "statement_block") {
        return callable(file, rows, name_row, body);
    }
    let (start, end) = (*rows.start(), *rows.end());
    let arrow_row = function
        .children(&mut function.walk())
        .find(|child| child.kind() == "=>")
        .map_or(end, |arrow| arrow.start_position().row + 1);
    let head_end = arrow_row.max(name_row);
    DeclInfo {
        body: vec![Item::new(head_end + 1..=end)],
        ..DeclInfo::new(
            vec![name_row],
            (start..=head_end).collect(),
            Shape::Callable,
        )
    }
}

/// Head through the row before the first statement (at least through the
/// row opening the block and the name row); one body item per statement.
fn callable(
    file: &SourceFile,
    rows: RangeInclusive<usize>,
    name_row: usize,
    block: Option<Node>,
) -> DeclInfo {
    let floor = block.map_or(name_row, |block| {
        (block.start_position().row + 1).max(name_row)
    });
    file.callable(vec![name_row], rows, named_children(block), floor)
}

/// A class or object literal as a container: the header and closing row
/// as head, each visible field or entry (with its comments) as a body
/// item, and each visible method or function-valued field or entry as a
/// member listed by its name row.
fn class(
    file: &SourceFile,
    rows: RangeInclusive<usize>,
    class_name_row: usize,
    block: Option<Node>,
) -> DeclInfo {
    let Some(block) = block else {
        return file.whole(vec![class_name_row], rows, None);
    };
    let open_row = block.start_position().row + 1;
    let mut body = Vec::new();
    let mut members = Vec::new();
    let mut last_row = open_row;
    let mut comment_start = None;
    // A method's decorators are its preceding siblings, not its children.
    let mut first_decorator: Option<Node> = None;
    let mut cursor = block.walk();
    for child in block.named_children(&mut cursor) {
        let child_rows = file.node_rows(child);
        match child.kind() {
            "comment" => {
                if *child_rows.start() > last_row && first_decorator.is_none() {
                    comment_start.get_or_insert(*child_rows.start());
                }
                continue;
            }
            "decorator" => {
                first_decorator.get_or_insert(child);
                continue;
            }
            _ => {}
        }
        let leading_comment = comment_start.take();
        let anchor = first_decorator.take().unwrap_or(child);
        let (start, end) = (anchor.start_position().row + 1, *child_rows.end());
        let is_member = matches!(
            child.kind(),
            "method_definition" | "method_signature" | "abstract_method_signature"
        );
        let is_field = matches!(
            child.kind(),
            "public_field_definition"
                | "property_signature"
                | "index_signature"
                | "pair"
                | "shorthand_property_identifier"
                | "spread_element"
        );
        let function_block = child
            .child_by_field_name("value")
            .and_then(wrapped_function_block);
        let is_data_entry = child.kind() == "pair" && function_block.is_none();
        if (is_member || is_field) && !is_hidden_member(file, child, is_data_entry) {
            if is_member || function_block.is_some() {
                let member_name_row = name_row(child).unwrap_or(start);
                let block = function_block.or_else(|| child.child_by_field_name("body"));
                let mut member = callable(file, start..=end, member_name_row, block);
                member.doc = doc_items(file, anchor);
                if !is_internal(file, &member.doc) {
                    body.push(Item::new(member.name_rows.iter().copied()));
                    members.push(member);
                }
            } else {
                let start = leading_comment.unwrap_or(start).max(last_row + 1);
                body.push(Item::new(start..=end));
            }
        }
        last_row = last_row.max(end);
    }
    let close_row = *file.node_rows(block).end();
    let head = block_head(rows, open_row, (close_row - 1).max(last_row));
    DeclInfo {
        body,
        members,
        ..DeclInfo::new(vec![class_name_row], head, Shape::Whole)
    }
}

/// `#name`, `private` and `protected` members, and `_name` members other
/// than an object literal's data entries (`_id`, `__typename`, whose
/// underscore is part of the data's shape): not part of the container's
/// API.
fn is_hidden_member(file: &SourceFile, member: Node, is_data_entry: bool) -> bool {
    if member
        .child_by_field_name("name")
        .or_else(|| member.child_by_field_name("key"))
        .is_some_and(|name| {
            name.kind() == "private_property_identifier"
                || (!is_data_entry && file.text(name).starts_with('_'))
        })
    {
        return true;
    }
    let mut cursor = member.walk();
    member.children(&mut cursor).any(|child| {
        child.kind() == "accessibility_modifier"
            && matches!(file.text(child), "private" | "protected")
    })
}

/// A doc tagged `@internal` or `@ignore`: public to the compiler, but
/// not part of the documented API.
fn is_internal(file: &SourceFile, doc: &[Item]) -> bool {
    doc.iter().flat_map(|item| &item.rows).any(|&row| {
        let line = file.line(row);
        line.contains("@internal") || line.contains("@ignore")
    })
}

/// The comments directly above `node`: one item per block comment and
/// per run of `//` rows. JSDoc is often separated from what it documents
/// by one blank row (`/** … */`, blank, `function f`), so one blank row
/// still attaches, and by tool directives (`// eslint-disable-next-line`),
/// which are skipped. A file's `@license` / `@fileoverview` header is not
/// the doc of the declaration under it.
fn doc_items(file: &SourceFile, node: Node) -> Vec<Item> {
    let is_line_comment = |row: usize| file.line(row).trim_start().starts_with("//");
    let mut items: Vec<Item> = Vec::new();
    for rows in file.comments_above(node, 2, |comment| {
        !is_file_header(comment, file.text(comment))
    }) {
        let start = *rows.start();
        if is_tool_directive(file.line(start).trim_start()) {
            continue;
        }
        match items.last_mut() {
            Some(run)
                if is_line_comment(start)
                    && run.rows.last() == Some(&(start - 1))
                    && is_line_comment(start - 1) =>
            {
                run.rows.extend(rows);
            }
            _ => items.push(Item::new(rows)),
        }
    }
    items
}

const TOOL_DIRECTIVES: &[&str] = &[
    "eslint-",
    "istanbul ",
    "tslint:",
    "prettier-ignore",
    "@ts-",
    "deno-lint-",
    "oxlint-",
    "biome-ignore",
];

/// A comment addressed to a linter, type checker or coverage tool.
fn is_tool_directive(comment: &str) -> bool {
    let Some(body) = comment
        .strip_prefix("//")
        .or_else(|| comment.strip_prefix("/*"))
    else {
        return false;
    };
    let body = body.trim_start();
    TOOL_DIRECTIVES
        .iter()
        .any(|directive| body.starts_with(directive))
}

const FILE_HEADER_TAGS: &[&str] = &[
    "@license",
    "@fileoverview",
    "@module",
    "@packagedocumentation",
];

/// A comment at the top of the file tagged as describing the file.
fn is_file_header(comment: Node, text: &str) -> bool {
    let text = text.to_ascii_lowercase();
    comment.start_position().row == 0 && FILE_HEADER_TAGS.iter().any(|tag| text.contains(tag))
}

#[cfg(test)]
mod tests {
    use super::super::test_support::{describe, rows};
    use super::*;

    fn extract_in(files: &[(&str, &str)], target: &str) -> FileModel {
        super::super::test_support::extract_in(&LANGUAGE, files, target).1
    }

    fn extract_source(relative: &str, source: &str) -> FileModel {
        extract_in(&[(relative, source)], relative)
    }

    #[test]
    fn typescript_exported_function_splits_signature_and_statements() {
        let model = extract_source(
            "src/util.ts",
            "\
/**
 * Adds.
 *
 * @example add(1, 2)
 */
export function add(
  a: number,
  b: number,
): number {
  // sum them
  const sum = a + b;
  return sum;
}
",
        );
        assert_eq!(
            describe(&model),
            ["Callable name [6] head [6, 7, 8, 9] doc [[1, 2, 3, 4, 5]] body [[10, 11], [12]]"]
        );
    }

    #[test]
    fn typescript_statement_on_the_signature_row_stays_head() {
        let model = extract_source(
            "a.ts",
            "export function one() { return 1; }\nexport const two = () => { return 2;\n};\n",
        );
        assert_eq!(
            describe(&model),
            [
                "Callable name [1] head [1] doc [] body []",
                "Callable name [2] head [2] doc [] body []",
            ]
        );
    }

    #[test]
    fn typescript_expression_bodied_arrow_keeps_its_body_out_of_the_head() {
        let model = extract_source(
            "src/card.tsx",
            "\
export const Card = (p: CardProps) => (
  <div>
    <button>OK</button>
  </div>
);
export const validate = (options) =>
  check(options, rules);
export const Row = memo((p: RowProps) => (
  <tr />
));
export const one = () => 1;
",
        );
        assert_eq!(
            describe(&model),
            [
                "Callable name [1] head [1] doc [] body [[2, 3, 4, 5]]",
                "Callable name [6] head [6] doc [] body [[7]]",
                "Callable name [8] head [8] doc [] body [[9, 10]]",
                "Callable name [11] head [11] doc [] body []",
            ]
        );
    }

    #[test]
    fn typescript_class_is_a_container_of_its_public_members() {
        let model = extract_source(
            "src/queue.ts",
            "\
export class Queue<T>
  extends Base {
  /** Pending count. */
  size = 0;
  #items: T[] = [];
  private cache = new Map();
  protected limit = 10;
  static { init(); }

  constructor(private readonly name: string) {
    super();
  }

  /** Adds an item. */
  @traced()
  add(item: T): void {
    this.#items.push(item);
  }

  private drain() {
    return [];
  }

  get length() { return this.size; }

  onDone = () => {
    notify();
  };
}
",
        );
        assert_eq!(
            describe(&model),
            [
                "Whole name [1] head [1, 2, 29] doc [] body [[3, 4], [10], [16], [24], [26]]",
                "  Callable name [10] head [10] doc [] body [[11]]",
                "  Callable name [16] head [15, 16] doc [[14]] body [[17]]",
                "  Callable name [24] head [24] doc [] body []",
                "  Callable name [26] head [26] doc [] body [[27]]",
            ]
        );
    }

    #[test]
    fn typescript_underscore_members_are_private() {
        let model = extract_source(
            "lib/graph.js",
            "\
export class Graph {
  _cache = {};
  _update() {}
  get() {}
}
export const store = {
  _id: 1,
  _normalize: function (key) {},
  load: function () {},
};
function Provider() {}
Provider.prototype._execute = function () {};
Provider.prototype.run = function () {};
export { Provider };
",
        );
        assert_eq!(
            describe(&model),
            [
                "Whole name [1] head [1, 5] doc [] body [[4]]",
                "  Callable name [4] head [4] doc [] body []",
                "Whole name [6] head [6, 10] doc [] body [[7], [9]]",
                "  Callable name [9] head [9] doc [] body []",
                "Callable name [11] head [11] doc [] body []",
                "Callable name [13] head [13] doc [] body []",
            ]
        );
    }

    #[test]
    fn typescript_underscore_data_entries_stay_public() {
        let model = extract_source(
            "src/schema.ts",
            "\
export const documentSchema = z.object({
  _id: z.string(),
  __typename: z.literal(\"Document\"),
  title: z.string(),
});
",
        );
        assert_eq!(
            describe(&model),
            ["Whole name [1] head [1, 5] doc [] body [[2], [3], [4]]"]
        );
    }

    #[test]
    fn typescript_internal_tagged_declarations_are_hidden() {
        let model = extract_source(
            "src/proxy.ts",
            "\
/** @internal */
export class HttpProxy {}
/**
 * Public client.
 */
export class Client {
  /** @ignore */
  reset() {}
  send() {}
}
",
        );
        assert_eq!(
            describe(&model),
            [
                "Whole name [6] head [6, 10] doc [[3, 4, 5]] body [[9]]",
                "  Callable name [9] head [9] doc [] body []",
            ]
        );
    }

    #[test]
    fn typescript_field_keeps_its_doc_across_a_blank_row() {
        let model = extract_source(
            "src/options.ts",
            "\
export class Options {
  /** Maximum time to wait, in milliseconds. */

  timeout = 1000;
}
",
        );
        assert_eq!(
            describe(&model),
            ["Whole name [1] head [1, 5] doc [] body [[2, 4]]"]
        );
    }

    #[test]
    fn typescript_abstract_and_overload_method_signatures_are_members() {
        let model = extract_source(
            "src/shape.ts",
            "\
export abstract class Shape {
  abstract area(): number;
  scale(by: number): this;
  scale(by: string): this;
  scale(by: unknown) {
    return this;
  }
}
",
        );
        assert_eq!(
            describe(&model),
            [
                "Whole name [1] head [1, 8] doc [] body [[2], [3], [4], [5]]",
                "  Callable name [2] head [2] doc [] body []",
                "  Callable name [3] head [3] doc [] body []",
                "  Callable name [4] head [4] doc [] body []",
                "  Callable name [5] head [5] doc [] body [[6]]",
            ]
        );
    }

    #[test]
    fn typescript_member_on_the_class_row_shares_the_container_head() {
        let model = extract_source("a.ts", "export class A { run() {\n  go();\n} }\n");
        assert_eq!(
            describe(&model),
            [
                "Whole name [1] head [1] doc [] body []",
                "  Callable name [1] head [1] doc [] body [[2]]",
            ]
        );
    }

    #[test]
    fn typescript_interface_enum_and_object_alias_list_their_entries() {
        let model = extract_source(
            "src/types.ts",
            "\
export interface Options {
  /** Retry count. */
  retries: number;
  onError(error: Error): void;
}
export enum Mode { Fast, Slow }
export type Point = {
  x: number;
  y: number;
};
export type Id = string | number;
export type Props = {
  open: boolean;
} & (A | B);
export type Picked = {
  open: boolean;
} & Pick<
  Base,
  'id'
>;
",
        );
        assert_eq!(
            describe(&model),
            [
                "Whole name [1] head [1, 5] doc [] body [[2, 3], [4]]",
                "Whole name [6] head [6] doc [] body []",
                "Whole name [7] head [7, 10] doc [] body [[8], [9]]",
                "Whole name [11] head [11] doc [] body []",
                "Whole name [12] head [12, 13, 14] doc [] body []",
                "Whole name [15] head [15, 16, 17, 18, 19, 20] doc [] body []",
            ]
        );
    }

    #[test]
    fn typescript_object_literal_const_lists_entries_and_methods() {
        let model = extract_source(
            "src/config.js",
            "\
export const config = {
  port: 80,
  // The host name.
  host: 'localhost',
  start() {
    listen();
  },
};
",
        );
        assert_eq!(
            describe(&model),
            [
                "Whole name [1] head [1, 8] doc [] body [[2], [3, 4], [5]]",
                "  Callable name [5] head [5] doc [] body [[6]]",
            ]
        );
    }

    #[test]
    fn typescript_call_ending_in_an_object_lists_its_entries() {
        let model = extract_source(
            "src/schema.ts",
            "\
export const userSchema = z.object({
  id: z.string(),
  name: z.string(),
});
export const theme = createTheme(base, {
  color: 'red',
}).strict();
export const store = new Store({
  path: '/tmp',
});
export const plain = z.object(shape);
module.exports = Area = Base.extend({
  init: function(id) {
    this.id = id;
  },
});
export const strict = z.object({
  id: z.string(),
})
  .strict();
",
        );
        assert_eq!(
            describe(&model),
            [
                "Whole name [1] head [1, 4] doc [] body [[2], [3]]",
                "Whole name [5] head [5, 7] doc [] body [[6]]",
                "Whole name [8] head [8, 10] doc [] body [[9]]",
                "Whole name [11] head [11] doc [] body []",
                "Whole name [12] head [12, 16] doc [] body [[13]]",
                "  Callable name [13] head [13] doc [] body [[14]]",
                "Whole name [17] head [17, 19, 20] doc [] body [[18]]",
            ]
        );
    }

    #[test]
    fn typescript_local_types_in_published_signatures_are_published() {
        let model = extract_source(
            "src/timeline.tsx",
            "\
type RoomTimelineProps = { room: Room };
type Hidden = { secret: string };
type StateType<T> = Merged<T>;
export function RoomTimeline({ room }: RoomTimelineProps) {
  const x: Hidden = load();
}
export interface TreeState<T> extends StateType<T> {}
",
        );
        let name_rows: Vec<Vec<usize>> = model
            .decls
            .iter()
            .map(|decl| decl.name_rows.clone())
            .collect();
        assert_eq!(name_rows, [vec![1], vec![3], vec![4], vec![7]]);
    }

    #[test]
    fn typescript_reexport_keeps_its_line_comment_label() {
        let model = extract_source(
            "src/index.ts",
            "\
// Base
export * from './base';
/** Errors. */
export * from './errors';
",
        );
        assert_eq!(rows(&model.reexports), [vec![1, 2], vec![4]]);
    }

    #[test]
    fn typescript_reexports_and_clauses_publish_without_declaring() {
        let model = extract_source(
            "src/api.ts",
            "\
export * from './a';
export { b, c as d } from './b';
function local() {}
function hidden() {}
interface Shape {}
export { local, type Shape };
",
        );
        assert_eq!(rows(&model.reexports), [vec![1], vec![2], vec![6]]);
        assert_eq!(
            describe(&model),
            [
                "Callable name [3] head [3] doc [] body []",
                "Whole name [5] head [5] doc [] body []",
            ]
        );
    }

    #[test]
    fn typescript_unexported_declarations_are_hidden_unless_an_entrypoint_exports_nothing() {
        let source = "import x from 'x';\nconst helper = 1;\nexport const api = 2;\n";
        let listed = |relative| extract_source(relative, source).decls.len();
        assert_eq!(listed("src/other.ts"), 1);
        assert_eq!(listed("src/index.ts"), 1);
        let application = extract_source(
            "src/main.ts",
            "'use strict';\nimport { mount } from 'ui';\nfunction App() {}\nfunction start() {\n  mount(App);\n}\nif (ready) {\n  start();\n}\n",
        );
        let heads: Vec<_> = application
            .decls
            .iter()
            .map(|decl| decl.head.clone())
            .collect();
        assert_eq!(heads, [vec![3], vec![4], vec![7, 8, 9]]);
        let script = extract_source(
            "scripts/build.js",
            "const fs = require('fs');\nfunction main() {}\nmain();\n",
        );
        assert!(script.decls.is_empty());
    }

    /// A published `f()` or `new C()` publishes the local it calls.
    #[test]
    fn typescript_published_factory_call_publishes_its_callee() {
        let cases: [(&str, &str, &[&str]); 3] = [
            (
                "src/client.ts",
                "function createInstance() {\n  return new Client();\n}\nconst instance = createInstance();\nfunction unrelated() {}\nexport default instance;\n",
                &[
                    "Callable name [1] head [1] doc [] body [[2]]",
                    "Whole name [4] head [4] doc [] body []",
                ],
            ),
            (
                "src/client.js",
                "class Client {\n  request() {}\n}\nconst client = new Client();\nexport default client;\n",
                &[
                    "Whole name [1] head [1, 3] doc [] body [[2]]",
                    "  Callable name [2] head [2] doc [] body []",
                    "Whole name [4] head [4] doc [] body []",
                ],
            ),
            (
                "lib/cli.js",
                "function Cli() {\n  this.opts = [];\n}\nfunction other() {}\nmodule.exports = new Cli();\n",
                &[
                    "Callable name [1] head [1] doc [] body [[2]]",
                    "Whole name [5] head [5] doc [] body []",
                ],
            ),
        ];
        for (relative, source, expected) in cases {
            assert_eq!(
                describe(&extract_source(relative, source)),
                expected,
                "{source}"
            );
        }
    }

    #[test]
    fn typescript_default_export_values_take_their_shape() {
        let model = extract_source(
            "src/component.tsx",
            "export default function () {\n  return <div />;\n}\n",
        );
        assert_eq!(
            describe(&model),
            ["Callable name [1] head [1] doc [] body [[2]]"]
        );
        let object = extract_source(
            "src/plugin.js",
            "export default {\n  name: 'x',\n  setup,\n};\n",
        );
        assert_eq!(
            describe(&object),
            ["Whole name [1] head [1, 4] doc [] body [[2], [3]]"]
        );
    }

    #[test]
    fn typescript_wrapped_component_in_jsx_is_callable() {
        let model = extract_source(
            "src/item.jsx",
            "\
export const Item = React.memo(React.forwardRef((props, ref) => {
  const [open, setOpen] = useState(false);
  return <li ref={ref}>{props.label}</li>;
}));
",
        );
        assert_eq!(
            describe(&model),
            ["Callable name [1] head [1] doc [] body [[2], [3]]"]
        );
    }

    #[test]
    fn typescript_commonjs_exports_publish_their_targets() {
        let model = extract_source(
            "lib/router.js",
            "\
'use strict';
var debug = require('debug')('router');
var EventEmitter = require('events').EventEmitter;
function Router() {}
function helper() {}
function internal() {}
exports.helper = helper;
exports.create = function create(options) {
  return new Router(options);
};
module.exports.VERSION = '1.0';
module.exports = { Router };
exports.json = parsers.json;
exports.static = require('serve-static');
",
        );
        assert_eq!(
            rows(&model.reexports),
            [vec![7], vec![12], vec![13], vec![14]]
        );
        assert_eq!(
            describe(&model),
            [
                "Callable name [4] head [4] doc [] body []",
                "Callable name [5] head [5] doc [] body []",
                "Callable name [8] head [8] doc [] body [[9]]",
                "Whole name [11] head [11] doc [] body []",
            ]
        );
    }

    #[test]
    fn typescript_values_assigned_onto_a_published_local_are_its_api() {
        let model = extract_source(
            "lib/application.js",
            "\
var app = exports = module.exports = {};
app.use = function use(fn) {
  return this;
};
function Router() {}
Router.prototype.route = function route(path) {};
var other = {};
other.run = function run() {};
app.name = 'app';
",
        );
        assert_eq!(
            describe(&model),
            [
                "Whole name [1] head [1] doc [] body []",
                "Callable name [2] head [2] doc [] body [[3]]",
                "Whole name [9] head [9] doc [] body []",
            ]
        );
        let router = extract_source(
            "lib/router.js",
            "function Router() {}\nRouter.prototype.route = () => {};\nmodule.exports = Router;\n",
        );
        assert_eq!(
            describe(&router),
            [
                "Callable name [1] head [1] doc [] body []",
                "Callable name [2] head [2] doc [] body []",
            ]
        );
        for source in [
            "export function Router() {}\nRouter.prototype.route = () => {};\nRouter.create = () => {};\n",
            "function Router() {}\nRouter.prototype.route = () => {};\nRouter.create = () => {};\nexport { Router };\n",
            "export default function Router() {}\nRouter.prototype.route = () => {};\nRouter.create = () => {};\n",
        ] {
            assert_eq!(
                describe(&extract_source("lib/router.js", source))[..3],
                [
                    "Callable name [1] head [1] doc [] body []",
                    "Callable name [2] head [2] doc [] body []",
                    "Callable name [3] head [3] doc [] body []",
                ],
                "{source}"
            );
        }
    }

    #[test]
    fn typescript_assignment_chain_is_shaped_by_its_final_value() {
        let model = extract_source(
            "lib/stores/file.js",
            "\
var File = exports.File = function (options) {
  this.type = 'file';
  this.file = options.file;
};
",
        );
        assert_eq!(
            describe(&model),
            ["Callable name [1] head [1] doc [] body [[2], [3]]"]
        );
    }

    #[test]
    fn typescript_prototype_object_lists_its_methods() {
        let model = extract_source(
            "lib/store.js",
            "\
function Store(opts) {}
Store.prototype = {
  get: function (key) {
    return this.data[key];
  },
  set: function (key, value) {},
};
exports.Store = Store;
",
        );
        assert_eq!(
            describe(&model),
            [
                "Callable name [1] head [1] doc [] body []",
                "Whole name [2] head [2, 7] doc [] body [[3], [6]]",
                "  Callable name [3] head [3] doc [] body [[4]]",
                "  Callable name [6] head [6] doc [] body []",
            ]
        );
    }

    #[test]
    fn typescript_amd_and_umd_factories_export_what_they_return() {
        let amd = extract_source(
            "client/js/area.js",
            "\
define(['lib/class'], function(Class) {
    function helper() {}
    var Area = Class.extend({
        init: function(x) {
            this.x = x;
        },
    });
    return Area;
});
",
        );
        assert_eq!(rows(&amd.reexports), [vec![8]]);
        assert_eq!(
            describe(&amd),
            [
                "Whole name [3] head [3, 7] doc [] body [[4]]",
                "  Callable name [4] head [4] doc [] body [[5]]",
            ]
        );
        let umd = extract_source(
            "src/lib.js",
            "\
/** License. */
(function (root, factory) {
  if (typeof define === 'function') define(factory);
  else root.Lib = factory();
}(this, function () {
  'use strict';
  /** Makes one. */
  function Lib() {}
  return Lib;
}));
",
        );
        assert_eq!(rows(&umd.reexports), [vec![9]]);
        assert_eq!(
            describe(&umd),
            ["Callable name [8] head [8] doc [[7]] body []"]
        );
    }

    #[test]
    fn typescript_overloads_are_separate_declarations() {
        let model = extract_source(
            "src/parse.ts",
            "\
export function parse(input: string): Ast;
export function parse(input: Buffer): Ast;
export function parse(input: unknown): Ast {
  return build(input);
}
",
        );
        assert_eq!(
            describe(&model),
            [
                "Callable name [1] head [1] doc [] body []",
                "Callable name [2] head [2] doc [] body []",
                "Callable name [3] head [3] doc [] body [[4]]",
            ]
        );
    }

    #[test]
    fn typescript_declaration_file_publishes_every_ambient_declaration() {
        let model = extract_source(
            "types/index.d.ts",
            "\
declare function greet(name: string): void;
declare const VERSION: string;
interface Options {
  loud: boolean;
}
declare namespace Greeter {
  function reset(): void;
  const count: number;
}
",
        );
        assert_eq!(
            describe(&model),
            [
                "Callable name [1] head [1] doc [] body []",
                "Whole name [2] head [2] doc [] body []",
                "Whole name [3] head [3, 5] doc [] body [[4]]",
                "Whole name [6] head [6, 9] doc [] body [[7], [8]]",
            ]
        );
    }

    #[test]
    fn typescript_doc_sits_at_most_one_blank_row_above() {
        let model = extract_source(
            "a.ts",
            "\
/** Kept. */

export const a = 1;
/** Lost. */


export const b = 2;
let x; /** Trailing. */
export const c = 3;
",
        );
        let docs: Vec<Vec<Vec<usize>>> = model.decls.iter().map(|decl| rows(&decl.doc)).collect();
        assert_eq!(docs, [vec![vec![1]], vec![], vec![]]);
    }

    #[test]
    fn typescript_line_comment_runs_are_docs() {
        let model = extract_source(
            "lib/provider.js",
            "\
//
// ### function get (key, callback)
// Retrieves the value for the key.
//
export function get(key, callback) {}
// Implicit operators have no location.
/** Token. */
export type Token = string;
",
        );
        let docs: Vec<Vec<Vec<usize>>> = model.decls.iter().map(|decl| rows(&decl.doc)).collect();
        assert_eq!(docs, [vec![vec![1, 2, 3, 4]], vec![vec![6], vec![7]]]);
    }

    #[test]
    fn typescript_doc_skips_tool_directives_and_the_file_header() {
        let model = extract_source(
            "a.ts",
            "\
/**
 * @license MIT
 */
export function first() {}
/** Parses. */
// eslint-disable-next-line complexity
export function parse() {}
export class A {
  /** Handles. */
  /* istanbul ignore next */
  handle() {}
}
",
        );
        assert_eq!(
            describe(&model),
            [
                "Callable name [4] head [4] doc [] body []",
                "Callable name [7] head [7] doc [[5]] body []",
                "Whole name [8] head [8, 12] doc [] body [[11]]",
                "  Callable name [11] head [11] doc [[9]] body []",
            ]
        );
    }

    #[test]
    fn typescript_entrypoints_sit_near_their_package_root() {
        let files = [
            ("index.ts", ""),
            ("src/index.ts", ""),
            ("src/deep/index.ts", ""),
            ("src/deep/er/index.ts", ""),
            ("packages/core/package.json", "{}"),
            ("packages/core/src/main.js", ""),
            ("packages/core/src/lib/mod.ts", ""),
            ("packages/core/src/core.ts", ""),
            ("packages/core/bin/core.js", ""),
            ("index.d.ts", ""),
            ("src/app.ts", ""),
        ];
        let dir = tempfile::tempdir().unwrap();
        for (relative, content) in files {
            let path = dir.path().join(relative);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, content).unwrap();
        }
        let ctx = WalkCtx::new(dir.path().to_path_buf());
        let entrypoints: Vec<&str> = files
            .iter()
            .map(|(relative, _)| *relative)
            .filter(|relative| is_entrypoint(&dir.path().join(relative), &ctx))
            .collect();
        assert_eq!(
            entrypoints,
            [
                "index.ts",
                "src/index.ts",
                "src/deep/index.ts",
                "packages/core/src/main.js",
                "packages/core/src/lib/mod.ts",
                "packages/core/src/core.ts"
            ]
        );
    }

    #[test]
    fn typescript_config_weighs_less() {
        let ctx = WalkCtx::new(PathBuf::from("/repo"));
        let weight = |relative: &str| file_weight(&Path::new("/repo").join(relative), &ctx);
        for primary in [
            "index.js",
            "lib/router.js",
            "server/app.ts",
            "src/irc.js",
            "arc.ts",
        ] {
            assert_eq!(weight(primary), 1.0, "{primary}");
        }
        for config in [
            ".eslintrc.js",
            "vite.config.ts",
            "src/jest.config.cjs",
            ".prettierrc.js",
        ] {
            assert_eq!(weight(config), CONFIG_FILE_WEIGHT, "{config}");
        }
    }
}
