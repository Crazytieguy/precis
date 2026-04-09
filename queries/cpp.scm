; Function definitions (standalone)
(function_definition
  declarator: (function_declarator
    declarator: (identifier) @name)) @symbol

; Function definitions returning pointers
(function_definition
  declarator: (pointer_declarator
    declarator: (function_declarator
      declarator: (identifier) @name))) @symbol

; Function definitions with qualified names (methods defined outside class)
(function_definition
  declarator: (function_declarator
    declarator: (qualified_identifier) @name)) @symbol

; Out-of-class methods returning pointers
(function_definition
  declarator: (pointer_declarator
    declarator: (function_declarator
      declarator: (qualified_identifier) @name))) @symbol

; Function declarations (prototypes) — direct
(declaration
  declarator: (function_declarator
    declarator: (identifier) @name)) @symbol

; Function declarations (prototypes) — returning pointer
(declaration
  declarator: (pointer_declarator
    declarator: (function_declarator
      declarator: (identifier) @name))) @symbol

; Class definitions (with body only, not forward declarations)
(class_specifier
  name: (type_identifier) @name
  body: (field_declaration_list)) @symbol

; Struct definitions (with body)
(struct_specifier
  name: (type_identifier) @name
  body: (field_declaration_list)) @symbol

; Union definitions
(union_specifier
  name: (type_identifier) @name
  body: (field_declaration_list)) @symbol

; Enum definitions (both scoped and unscoped)
(enum_specifier
  name: (type_identifier) @name
  body: (enumerator_list)) @symbol

; Namespace definitions
(namespace_definition
  name: (namespace_identifier) @name) @symbol

; Using type alias: using Foo = Bar;
(alias_declaration
  name: (type_identifier) @name) @symbol

; Typedef declarations (name extracted in code)
(type_definition) @symbol

; Macro definitions
(preproc_def
  name: (identifier) @name) @symbol

; Function-like macro definitions
(preproc_function_def
  name: (identifier) @name) @symbol

; Include directives
(preproc_include) @symbol
