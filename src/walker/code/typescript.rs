//! TypeScript / JavaScript extraction for the code engine.
//!
//! A file's API is what it exports: ESM `export` declarations, the
//! locals an `export { … }` clause, `export default X` or `export = X`
//! names, and CommonJS `module.exports` / `exports.x` targets. Every
//! top-level declaration of a `.d.ts` file is API (ambient declarations
//! are implicitly exported). Other top-level declarations are `Private`
//! in entrypoint files and hidden elsewhere. `export … from` and the statements that export a
//! name without declaring it are re-exports, listed on the roster.
//!
//! Classes are containers: methods (and arrow-function fields) are
//! members, other fields are body items, and `#name` / `private` /
//! `protected` members are hidden. Interfaces, enums, object-literal
//! values and namespaces are `Whole` declarations whose entries are body
//! items. Imports and `require` declarations are not modeled.

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use tree_sitter::Node;

use super::SourceFile;
use super::model::{DeclInfo, FileModel, Item, Shape, Visibility};
use crate::walker::WalkCtx;

pub(super) const EXTENSIONS: &[&str] = &["ts", "tsx", "mts", "cts", "js", "jsx", "mjs", "cjs"];

const ENTRYPOINT_STEMS: &[&str] = &["index", "main", "mod", "esm"];

/// JSDoc tags that mark a leading comment as the file's own doc rather
/// than the next declaration's.
const MODULE_DOC_TAGS: &[&str] = &["@module", "@packageDocumentation", "@file", "@fileoverview"];

/// The TypeScript grammar for `.ts` / `.mts` / `.cts`; the TSX grammar,
/// which also parses JSX, for everything else.
pub(super) fn grammar(path: &Path) -> tree_sitter::Language {
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

pub(super) fn extract(file: &SourceFile, ctx: &WalkCtx) -> FileModel {
    let root = file.tree.root_node();
    let mut cursor = root.walk();
    let statements: Vec<Node> = root.children(&mut cursor).collect();
    let entrypoint = is_entrypoint(&file.path, ctx);

    let mut scan = ExportScan::default();
    let classified: Vec<(Node, TopLevel)> = statements
        .iter()
        .map(|&statement| (statement, scan.classify(file, statement)))
        .collect();
    scan.follow_local_factories(file, &classified);

    let module_doc = module_doc(file, &statements, entrypoint);
    let module_doc_rows: HashSet<usize> = module_doc
        .iter()
        .flat_map(|item| item.rows.iter().copied())
        .collect();
    let unexported = if is_declaration_file(&file.path) {
        Some(Visibility::Public)
    } else if entrypoint {
        Some(Visibility::Private)
    } else {
        None
    };

    let mut model = FileModel {
        module_doc,
        ..FileModel::default()
    };
    for (statement, top_level) in classified {
        let (node, visibility) = match top_level {
            TopLevel::Reexport => {
                model.reexports.push(Item::new(file.node_rows(statement)));
                continue;
            }
            TopLevel::Exported(node) => (node, Visibility::Public),
            TopLevel::Local(node) => {
                let exported = declared_names(file, node)
                    .iter()
                    .any(|name| scan.public_names.contains(*name));
                let visibility = if exported {
                    Some(Visibility::Public)
                } else {
                    unexported
                };
                let Some(visibility) = visibility else {
                    continue;
                };
                (node, visibility)
            }
            TopLevel::Method { receiver, value }
                if scan.public_names.contains(file.text(receiver)) =>
            {
                (value, Visibility::Public)
            }
            TopLevel::Method { .. } | TopLevel::Skip => continue,
        };
        let parts = declaration_parts(file, statement, node, visibility);
        model.decls.push(DeclInfo {
            name_rows: parts.name_rows,
            head: parts.head,
            doc: doc_items(file, statement, &module_doc_rows),
            body: parts.body,
            shape: parts.shape,
            visibility,
            members: parts.members,
        });
    }
    model
}

/// `index` / `main` / `mod` / `esm` sources within one directory of
/// their package root (the nearest `package.json` directory, else the
/// walk root): `index.ts`, `src/index.ts`, `lib/main.js`.
pub(super) fn is_entrypoint(path: &Path, ctx: &WalkCtx) -> bool {
    let Some((stem, extension)) = path
        .file_name()
        .and_then(|name| name.to_str())
        .and_then(|name| name.rsplit_once('.'))
    else {
        return false;
    };
    let named_entrypoint = ENTRYPOINT_STEMS.contains(&stem)
        && EXTENSIONS
            .iter()
            .any(|candidate| extension.eq_ignore_ascii_case(candidate));
    let Some(dir) = path.parent().filter(|_| named_entrypoint) else {
        return false;
    };
    let package_dir = ctx.code.typescript.package_dir(dir, ctx.root());
    path.strip_prefix(package_dir)
        .is_ok_and(|relative| relative.components().count() <= 2)
}

/// Tooling config (`vite.config.ts`, `.eslintrc.js`) is not the
/// project's code.
pub(super) fn file_weight(path: &Path, _ctx: &WalkCtx) -> f64 {
    if is_config_file(path) {
        CONFIG_FILE_WEIGHT
    } else {
        1.0
    }
}

const CONFIG_FILE_WEIGHT: f64 = 0.001;

fn is_config_file(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
        return false;
    };
    let stem = name.split('.').next().unwrap_or_default();
    name.starts_with('.') || name.contains(".config.") || stem.ends_with("rc")
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
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| {
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
    /// A function assigned onto a local (`app.use = function …`,
    /// `Router.prototype.route = …`): a method of that local's API when
    /// the local is published.
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
                if let Some(value) = commonjs_export_value(file, statement) {
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

/// The exported value of a CommonJS export statement: `module.exports =
/// X`, `exports.x = X`, `module.exports.x = X`, `exports['x'] = X`, and
/// chains through a bare `exports` (`exports = module.exports = X`).
fn commonjs_export_value<'tree>(file: &SourceFile, statement: Node<'tree>) -> Option<Node<'tree>> {
    commonjs_assignment_value(file, statement.named_child(0)?)
}

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

/// `R.name = <function>` or `R.prototype.name = <function>`, with `R` a
/// plain name.
fn method_assignment<'tree>(file: &SourceFile, assignment: Node<'tree>) -> TopLevel<'tree> {
    let (Some(left), Some(value)) = (
        assignment.child_by_field_name("left"),
        assignment.child_by_field_name("right"),
    ) else {
        return TopLevel::Skip;
    };
    let is_function = is_function_kind(value.kind()) || wrapped_function_block(value).is_some();
    if assignment.kind() != "assignment_expression"
        || left.kind() != "member_expression"
        || !is_function
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

/// A declaration's rows other than its doc.
struct Parts {
    name_rows: Vec<usize>,
    head: Vec<usize>,
    body: Vec<Item>,
    shape: Shape,
    members: Vec<DeclInfo>,
}

/// The parts of the declaration `statement` introduces, shaped by `node`
/// (the declaration or exported value inside it). The head starts at
/// `statement`, so it carries `export`, `declare` and decorators.
fn declaration_parts(
    file: &SourceFile,
    statement: Node,
    node: Node,
    visibility: Visibility,
) -> Parts {
    let span = Span::of(file, statement);
    let name_row = name_row(node).unwrap_or(span.start);
    let kind = node.kind();
    if is_function_kind(kind) {
        return callable(file, span, name_row, node.child_by_field_name("body"));
    }
    if is_class_kind(kind) {
        return class(
            file,
            span,
            name_row,
            node.child_by_field_name("body"),
            visibility,
        );
    }
    match kind {
        "interface_declaration" | "enum_declaration" | "internal_module" | "module" => {
            whole(file, span, vec![name_row], node.child_by_field_name("body"))
        }
        "type_alias_declaration" => {
            let object = node.child_by_field_name("value").and_then(own_object_type);
            whole(file, span, vec![name_row], object)
        }
        // `declare global { … }`
        "statement_block" => whole(file, span, vec![span.start], Some(node)),
        "lexical_declaration" | "variable_declaration" => match single_declarator(node) {
            Some(declarator) => match declarator.child_by_field_name("value") {
                Some(value) => value_parts(file, span, name_row, value, visibility),
                None => whole(file, span, vec![name_row], None),
            },
            None => {
                let mut cursor = node.walk();
                let name_rows = node
                    .named_children(&mut cursor)
                    .filter_map(|declarator| declarator.child_by_field_name("name"))
                    .map(|name| name.start_position().row + 1)
                    .collect();
                whole(file, span, name_rows, None)
            }
        },
        _ => value_parts(file, span, span.start, node, visibility),
    }
}

/// The member block an alias declares itself: `{ … }`, or the last
/// object literal of an intersection (`Base & { … }`).
fn own_object_type(value: Node) -> Option<Node> {
    match value.kind() {
        "object_type" => Some(value),
        "intersection_type" => {
            let mut cursor = value.walk();
            value
                .named_children(&mut cursor)
                .filter(|operand| operand.kind() == "object_type")
                .last()
        }
        _ => None,
    }
}

/// A declaration shaped by the value it binds: a function (possibly
/// wrapped, `memo(forwardRef(() => { … }))`) is `Callable`; a class is a
/// container; an object or array literal lists its entries; anything
/// else is all head.
fn value_parts(
    file: &SourceFile,
    span: Span,
    name_row: usize,
    value: Node,
    visibility: Visibility,
) -> Parts {
    if is_class_kind(value.kind()) {
        return class(
            file,
            span,
            name_row,
            value.child_by_field_name("body"),
            visibility,
        );
    }
    if matches!(value.kind(), "object" | "array") {
        return whole(file, span, vec![name_row], Some(value));
    }
    if let Some(block) = wrapped_function_block(value) {
        return callable(file, span, name_row, Some(block));
    }
    if is_function_kind(value.kind()) {
        return callable(file, span, name_row, None);
    }
    whole(file, span, vec![name_row], None)
}

/// How deep [`wrapped_function_block`] looks through call wrappers.
const FUNCTION_WRAPPER_DEPTH: usize = 6;

/// The statement block of a function value, seen through parentheses and
/// the first argument of wrapper calls (`memo(forwardRef((p, r) => {…}))`).
fn wrapped_function_block(value: Node) -> Option<Node> {
    let mut node = value;
    for _ in 0..FUNCTION_WRAPPER_DEPTH {
        if is_function_kind(node.kind()) {
            return node
                .child_by_field_name("body")
                .filter(|body| body.kind() == "statement_block");
        }
        node = match node.kind() {
            "call_expression" => node.child_by_field_name("arguments")?.named_child(0)?,
            "parenthesized_expression" => node.named_child(0)?,
            _ => return None,
        };
    }
    None
}

/// First and last row of a node, as [`SourceFile::node_rows`] counts them.
#[derive(Clone, Copy)]
struct Span {
    start: usize,
    end: usize,
}

impl Span {
    fn of(file: &SourceFile, node: Node) -> Self {
        let rows = file.node_rows(node);
        Self {
            start: *rows.start(),
            end: *rows.end(),
        }
    }
}

fn name_row(node: Node) -> Option<usize> {
    let name = node.child_by_field_name("name").or_else(|| {
        single_declarator(node).and_then(|declarator| declarator.child_by_field_name("name"))
    })?;
    Some(name.start_position().row + 1)
}

/// Head through the row before the first statement (at least through the
/// row opening the block and the name row); one body item per statement.
fn callable(file: &SourceFile, span: Span, name_row: usize, block: Option<Node>) -> Parts {
    let first_statement = block.and_then(|block| block.named_child(0));
    let (head_end, body) = match (block, first_statement) {
        (Some(block), Some(first)) => {
            let head_end = (block.start_position().row + 1)
                .max(first.start_position().row)
                .max(name_row);
            (head_end, entry_items(file, block, head_end))
        }
        _ => (span.end, Vec::new()),
    };
    Parts {
        name_rows: vec![name_row],
        head: (span.start..=head_end).collect(),
        body,
        shape: Shape::Callable,
        members: Vec::new(),
    }
}

/// Head through the row opening `block`, plus the rows from its closing
/// row through the declaration's end; one body item per entry of `block`.
/// Without a block, all head.
fn whole(file: &SourceFile, span: Span, name_rows: Vec<usize>, block: Option<Node>) -> Parts {
    let Some(block) = block else {
        return Parts {
            name_rows,
            head: (span.start..=span.end).collect(),
            body: Vec::new(),
            shape: Shape::Whole,
            members: Vec::new(),
        };
    };
    let open_row = block.start_position().row + 1;
    let body = entry_items(file, block, open_row);
    let last_body_row = body
        .iter()
        .flat_map(|item| item.rows.iter().copied())
        .max()
        .unwrap_or(open_row);
    let suffix_start = Span::of(file, block).end.max(last_body_row + 1);
    let head: Vec<usize> = (span.start..=open_row)
        .chain(suffix_start.max(open_row + 1)..=span.end)
        .collect();
    Parts {
        name_rows,
        head,
        body,
        shape: Shape::Whole,
        members: Vec::new(),
    }
}

/// One item per named child of `block` (statements, fields, entries),
/// each with the comments directly above it, keeping only rows past
/// `after_row` and past the previous item. A comment ending the block is
/// an item of its own.
fn entry_items(file: &SourceFile, block: Node, after_row: usize) -> Vec<Item> {
    let mut items = Vec::new();
    let mut last_row = after_row;
    let mut comments: Option<Span> = None;
    let mut cursor = block.walk();
    for child in block.named_children(&mut cursor) {
        let span = Span::of(file, child);
        if child.kind() == "comment" {
            if span.start > last_row {
                comments = Some(Span {
                    start: comments.map_or(span.start, |comments| comments.start),
                    end: span.end,
                });
            }
            continue;
        }
        let start = comments
            .take()
            .map_or(span.start, |comments| comments.start)
            .max(last_row + 1);
        if start <= span.end {
            items.push(Item::new(start..=span.end));
            last_row = span.end;
        }
    }
    if let Some(comments) = comments {
        items.push(Item::new(comments.start..=comments.end));
    }
    items
}

/// A class as a container: the header and closing row as head, each
/// visible field (with its comments) as a body item, and each visible
/// method or arrow-function field as a member listed by its name row.
fn class(
    file: &SourceFile,
    span: Span,
    name_row: usize,
    block: Option<Node>,
    visibility: Visibility,
) -> Parts {
    let Some(block) = block else {
        return whole(file, span, vec![name_row], None);
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
        let child_span = Span::of(file, child);
        match child.kind() {
            "comment" => {
                if child_span.start > last_row && first_decorator.is_none() {
                    comment_start.get_or_insert(child_span.start);
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
        let span = Span {
            start: anchor.start_position().row + 1,
            end: child_span.end,
        };
        let is_member = matches!(
            child.kind(),
            "method_definition" | "method_signature" | "abstract_method_signature"
        );
        let is_field = matches!(
            child.kind(),
            "public_field_definition" | "property_signature" | "index_signature"
        );
        if (is_member || is_field) && !is_hidden_member(file, child) {
            let function_block = child
                .child_by_field_name("value")
                .and_then(wrapped_function_block);
            if is_member || function_block.is_some() {
                let member_name_row = name_row_of_member(child).unwrap_or(span.start);
                let block = function_block.or_else(|| child.child_by_field_name("body"));
                let parts = callable(file, span, member_name_row, block);
                body.push(Item::new(parts.name_rows.iter().copied()));
                members.push(DeclInfo {
                    name_rows: parts.name_rows,
                    head: parts.head,
                    doc: doc_items(file, anchor, &HashSet::new()),
                    body: parts.body,
                    shape: Shape::Callable,
                    visibility,
                    members: Vec::new(),
                });
            } else {
                let start = leading_comment.unwrap_or(span.start).max(last_row + 1);
                if start <= span.end {
                    body.push(Item::new(start..=span.end));
                }
            }
        }
        last_row = last_row.max(span.end);
    }
    let mut head: Vec<usize> = (span.start..=open_row).collect();
    if span.end > last_row {
        head.push(span.end);
    }
    Parts {
        name_rows: vec![name_row],
        head,
        body,
        shape: Shape::Whole,
        members,
    }
}

fn name_row_of_member(member: Node) -> Option<usize> {
    member
        .child_by_field_name("name")
        .map(|name| name.start_position().row + 1)
}

/// `#name`, `private` and `protected` members: not part of the class's
/// API.
fn is_hidden_member(file: &SourceFile, member: Node) -> bool {
    if member
        .child_by_field_name("name")
        .is_some_and(|name| name.kind() == "private_property_identifier")
    {
        return true;
    }
    let mut cursor = member.walk();
    member.children(&mut cursor).any(|child| {
        child.kind() == "accessibility_modifier"
            && matches!(file.text(child), "private" | "protected")
    })
}

/// The `/** … */` blocks directly above `node`, one item per paragraph.
/// JSDoc is often separated from what it documents by one blank row
/// (`/** … */`, blank, `function f`), so one blank row still attaches.
/// A block in `module_doc_rows` is the file's doc and ends the walk.
fn doc_items(file: &SourceFile, node: Node, module_doc_rows: &HashSet<usize>) -> Vec<Item> {
    let mut blocks = Vec::new();
    let mut next_start = node.start_position().row + 1;
    let mut previous = node.prev_sibling();
    while let Some(comment) = previous {
        let span = Span::of(file, comment);
        let is_attached_jsdoc = comment.kind() == "comment"
            && file.text(comment).starts_with("/**")
            && span.end < next_start
            && next_start - span.end <= 2
            && starts_its_row(file, comment)
            && !module_doc_rows.contains(&span.start);
        if !is_attached_jsdoc {
            break;
        }
        blocks.push(comment);
        next_start = span.start;
        previous = comment.prev_sibling();
    }
    blocks
        .into_iter()
        .rev()
        .flat_map(|comment| jsdoc_paragraphs(file, comment))
        .collect()
}

fn starts_its_row(file: &SourceFile, node: Node) -> bool {
    let position = node.start_position();
    file.line(position.row + 1)
        .get(..position.column)
        .is_some_and(|before| before.trim().is_empty())
}

/// A JSDoc block split after each bare ` *` separator row.
fn jsdoc_paragraphs(file: &SourceFile, comment: Node) -> Vec<Item> {
    let span = Span::of(file, comment);
    let mut paragraphs = Vec::new();
    let mut paragraph = Vec::new();
    for row in span.start..=span.end {
        paragraph.push(row);
        let separator = file
            .line(row)
            .trim_start()
            .trim_start_matches('*')
            .trim()
            .is_empty();
        if separator && row != span.start && row != span.end {
            paragraphs.push(Item::new(std::mem::take(&mut paragraph)));
        }
    }
    if !paragraph.is_empty() {
        paragraphs.push(Item::new(paragraph));
    }
    paragraphs
}

/// The file's leading `/** … */` block (after a shebang and plain
/// comments) when it documents the file: it carries a file-level tag, or
/// it opens an entrypoint and a blank row separates it from what follows.
fn module_doc(file: &SourceFile, statements: &[Node], entrypoint: bool) -> Vec<Item> {
    let lede = statements
        .iter()
        .find(|statement| {
            !(statement.kind() == "hash_bang_line"
                || statement.kind() == "comment" && !file.text(**statement).starts_with("/**"))
        })
        .filter(|statement| statement.kind() == "comment");
    let Some(&lede) = lede else {
        return Vec::new();
    };
    let text = file.text(lede);
    let tagged = MODULE_DOC_TAGS.iter().any(|tag| {
        text.match_indices(tag)
            .any(|(at, _)| !text[at + tag.len()..].starts_with(|c: char| c.is_ascii_alphanumeric()))
    });
    let blank_after = file.line(Span::of(file, lede).end + 1).trim().is_empty();
    if tagged || (entrypoint && blank_after) {
        jsdoc_paragraphs(file, lede)
    } else {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::walker::code::Language;

    /// Writes `files` under a fresh root and extracts `target`.
    fn extract_in(files: &[(&str, &str)], target: &str) -> FileModel {
        let dir = tempfile::tempdir().unwrap();
        for (relative, content) in files {
            let path = dir.path().join(relative);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, content).unwrap();
        }
        let ctx = WalkCtx::new(dir.path().to_path_buf());
        let file = SourceFile::parse(&dir.path().join(target), Language::TypeScript, &ctx).unwrap();
        extract(&file, &ctx)
    }

    fn extract_source(relative: &str, source: &str) -> FileModel {
        extract_in(&[(relative, source)], relative)
    }

    fn rows(items: &[Item]) -> Vec<Vec<usize>> {
        items.iter().map(|item| item.rows.clone()).collect()
    }

    /// One line per declaration (members indented): visibility, shape,
    /// name rows, head, doc and body.
    fn describe(model: &FileModel) -> Vec<String> {
        fn line(decl: &DeclInfo, indent: &str) -> String {
            format!(
                "{indent}{:?} {:?} name {:?} head {:?} doc {:?} body {:?}",
                decl.visibility,
                decl.shape,
                decl.name_rows,
                decl.head,
                rows(&decl.doc),
                rows(&decl.body),
            )
        }
        model
            .decls
            .iter()
            .flat_map(|decl| {
                std::iter::once(line(decl, "")).chain(decl.members.iter().map(|m| line(m, "  ")))
            })
            .collect()
    }

    #[test]
    fn code_typescript_exported_function_splits_doc_signature_and_statements() {
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
            [
                "Public Callable name [6] head [6, 7, 8, 9] doc [[1, 2, 3], [4, 5]] body [[10, 11], [12]]"
            ]
        );
    }

    #[test]
    fn code_typescript_statement_on_the_signature_row_stays_head() {
        let model = extract_source(
            "a.ts",
            "export function one() { return 1; }\nexport const two = () => { return 2;\n};\n",
        );
        assert_eq!(
            describe(&model),
            [
                "Public Callable name [1] head [1] doc [] body []",
                "Public Callable name [2] head [2] doc [] body []",
            ]
        );
    }

    #[test]
    fn code_typescript_class_is_a_container_of_its_public_members() {
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
                "Public Whole name [1] head [1, 2, 29] doc [] body [[3, 4], [10], [16], [24], [26]]",
                "  Public Callable name [10] head [10] doc [] body [[11]]",
                "  Public Callable name [16] head [15, 16] doc [[14]] body [[17]]",
                "  Public Callable name [24] head [24] doc [] body []",
                "  Public Callable name [26] head [26] doc [] body [[27]]",
            ]
        );
    }

    #[test]
    fn code_typescript_abstract_and_overload_method_signatures_are_members() {
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
                "Public Whole name [1] head [1, 8] doc [] body [[2], [3], [4], [5]]",
                "  Public Callable name [2] head [2] doc [] body []",
                "  Public Callable name [3] head [3] doc [] body []",
                "  Public Callable name [4] head [4] doc [] body []",
                "  Public Callable name [5] head [5] doc [] body [[6]]",
            ]
        );
    }

    #[test]
    fn code_typescript_member_on_the_class_row_shares_the_container_head() {
        let model = extract_source("a.ts", "export class A { run() {\n  go();\n} }\n");
        assert_eq!(
            describe(&model),
            [
                "Public Whole name [1] head [1] doc [] body [[1]]",
                "  Public Callable name [1] head [1] doc [] body [[2]]",
            ]
        );
    }

    #[test]
    fn code_typescript_interface_enum_and_object_alias_list_their_entries() {
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
                "Public Whole name [1] head [1, 5] doc [] body [[2, 3], [4]]",
                "Public Whole name [6] head [6] doc [] body []",
                "Public Whole name [7] head [7, 10] doc [] body [[8], [9]]",
                "Public Whole name [11] head [11] doc [] body []",
                "Public Whole name [12] head [12, 14] doc [] body [[13]]",
                "Public Whole name [15] head [15, 17, 18, 19, 20] doc [] body [[16]]",
            ]
        );
    }

    #[test]
    fn code_typescript_object_literal_const_lists_entries_with_their_comments() {
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
            ["Public Whole name [1] head [1, 8] doc [] body [[2], [3, 4], [5, 6, 7]]"]
        );
    }

    #[test]
    fn code_typescript_reexports_and_clauses_publish_without_declaring() {
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
                "Public Callable name [3] head [3] doc [] body []",
                "Public Whole name [5] head [5] doc [] body []",
            ]
        );
    }

    #[test]
    fn code_typescript_unexported_declarations_are_private_only_in_entrypoints() {
        let source = "import x from 'x';\nconst helper = 1;\nexport const api = 2;\n";
        let visibilities = |relative| {
            extract_source(relative, source)
                .decls
                .iter()
                .map(|decl| decl.visibility)
                .collect::<Vec<_>>()
        };
        assert_eq!(visibilities("src/other.ts"), [Visibility::Public]);
        assert_eq!(
            visibilities("src/index.ts"),
            [Visibility::Private, Visibility::Public]
        );
        let script = extract_source(
            "scripts/build.js",
            "const fs = require('fs');\nfunction main() {}\nmain();\n",
        );
        assert!(script.decls.is_empty());
    }

    #[test]
    fn code_typescript_default_export_resolves_to_the_local_implementation() {
        let model = extract_source(
            "src/client.ts",
            "\
function createInstance() {
  return new Client();
}
const instance = createInstance();
function unrelated() {}
export default instance;
",
        );
        assert_eq!(rows(&model.reexports), [vec![6]]);
        assert_eq!(
            describe(&model),
            [
                "Public Callable name [1] head [1] doc [] body [[2]]",
                "Public Whole name [4] head [4] doc [] body []",
            ]
        );
    }

    #[test]
    fn code_typescript_default_export_values_take_their_shape() {
        let model = extract_source(
            "src/component.tsx",
            "export default function () {\n  return <div />;\n}\n",
        );
        assert_eq!(
            describe(&model),
            ["Public Callable name [1] head [1] doc [] body [[2]]"]
        );
        let object = extract_source(
            "src/plugin.js",
            "export default {\n  name: 'x',\n  setup,\n};\n",
        );
        assert_eq!(
            describe(&object),
            ["Public Whole name [1] head [1, 4] doc [] body [[2], [3]]"]
        );
    }

    #[test]
    fn code_typescript_wrapped_component_in_jsx_is_callable() {
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
            ["Public Callable name [1] head [1] doc [] body [[2], [3]]"]
        );
    }

    #[test]
    fn code_typescript_commonjs_exports_publish_their_targets() {
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
                "Public Callable name [4] head [4] doc [] body []",
                "Public Callable name [5] head [5] doc [] body []",
                "Public Callable name [8] head [8] doc [] body [[9]]",
                "Public Whole name [11] head [11] doc [] body []",
            ]
        );
    }

    #[test]
    fn code_typescript_functions_assigned_onto_a_published_local_are_its_methods() {
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
                "Public Whole name [1] head [1] doc [] body []",
                "Public Callable name [2] head [2] doc [] body [[3]]",
            ]
        );
        let router = extract_source(
            "lib/router.js",
            "function Router() {}\nRouter.prototype.route = () => {};\nmodule.exports = Router;\n",
        );
        assert_eq!(
            describe(&router),
            [
                "Public Callable name [1] head [1] doc [] body []",
                "Public Callable name [2] head [2] doc [] body []",
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
                    "Public Callable name [1] head [1] doc [] body []",
                    "Public Callable name [2] head [2] doc [] body []",
                    "Public Callable name [3] head [3] doc [] body []",
                ],
                "{source}"
            );
        }
    }

    #[test]
    fn code_typescript_commonjs_published_constructor_call_publishes_the_constructor() {
        let model = extract_source(
            "lib/cli.js",
            "function Cli() {\n  this.opts = [];\n}\nfunction other() {}\nmodule.exports = new Cli();\n",
        );
        assert_eq!(
            describe(&model),
            [
                "Public Callable name [1] head [1] doc [] body [[2]]",
                "Public Whole name [5] head [5] doc [] body []",
            ]
        );
    }

    #[test]
    fn code_typescript_overloads_are_separate_declarations() {
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
                "Public Callable name [1] head [1] doc [] body []",
                "Public Callable name [2] head [2] doc [] body []",
                "Public Callable name [3] head [3] doc [] body [[4]]",
            ]
        );
    }

    #[test]
    fn code_typescript_declaration_file_publishes_every_ambient_declaration() {
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
                "Public Callable name [1] head [1] doc [] body []",
                "Public Whole name [2] head [2] doc [] body []",
                "Public Whole name [3] head [3, 5] doc [] body [[4]]",
                "Public Whole name [6] head [6, 9] doc [] body [[7], [8]]",
            ]
        );
    }

    #[test]
    fn code_typescript_module_doc_needs_a_tag_or_an_entrypoint_gap() {
        let separated = "#!/usr/bin/env node\n/** The CLI. */\n\nexport function run() {}\n";
        assert_eq!(
            rows(&extract_source("index.js", separated).module_doc),
            [vec![2]]
        );
        assert!(
            extract_source("src/deep/run.js", separated)
                .module_doc
                .is_empty()
        );

        let attached = "/** Runs. */\nexport function run() {}\n";
        let model = extract_source("index.js", attached);
        assert!(model.module_doc.is_empty());
        assert_eq!(rows(&model.decls[0].doc), [vec![1]]);

        let tagged = "/**\n * @module run\n */\nexport function run() {}\n";
        let model = extract_source("src/deep/run.js", tagged);
        assert_eq!(rows(&model.module_doc), [vec![1, 2, 3]]);
        assert!(model.decls[0].doc.is_empty());
    }

    #[test]
    fn code_typescript_doc_sits_at_most_one_blank_row_above() {
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
    fn code_typescript_entrypoints_sit_near_their_package_root() {
        let files = [
            ("index.ts", ""),
            ("src/index.ts", ""),
            ("src/deep/index.ts", ""),
            ("packages/core/package.json", "{}"),
            ("packages/core/src/main.js", ""),
            ("packages/core/src/lib/mod.ts", ""),
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
            ["index.ts", "src/index.ts", "packages/core/src/main.js"]
        );
    }

    #[test]
    fn code_typescript_config_weighs_less() {
        let ctx = WalkCtx::new(PathBuf::from("/repo"));
        let weight = |relative: &str| file_weight(&Path::new("/repo").join(relative), &ctx);
        for primary in ["index.js", "lib/router.js", "server/app.ts"] {
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
