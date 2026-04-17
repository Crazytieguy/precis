//! Shared DocFirst / DocRest kinds for every symbol family with doc comments.
//!
//! Nine families (Function, Struct, Enum, Class, Interface, Trait, TypeAlias,
//! Const, Macro) each have a DocFirst (renders the first doc line + spawns
//! DocRest) and DocRest (renders the remaining lines) kind. The impls are
//! byte-identical modulo type names, so they're generated here by one
//! `doc_pair!` invocation per family rather than scattered across the
//! family files.
//!
//! Module is the exception: `ModuleDocFirst` / `ModuleDocRest` have
//! noise-stripping render helpers and live in `module.rs`.

use super::doc_pair;

doc_pair!(FunctionDocFirst, FunctionDocRest);
doc_pair!(StructDocFirst, StructDocRest);
doc_pair!(EnumDocFirst, EnumDocRest);
doc_pair!(ClassDocFirst, ClassDocRest);
doc_pair!(InterfaceDocFirst, InterfaceDocRest);
doc_pair!(TraitDocFirst, TraitDocRest);
doc_pair!(TypeAliasDocFirst, TypeAliasDocRest);
doc_pair!(ConstDocFirst, ConstDocRest);
doc_pair!(MacroDocFirst, MacroDocRest);
