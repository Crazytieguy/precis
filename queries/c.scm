; Function definitions
(function_definition
  declarator: (function_declarator
    declarator: (identifier) @name)) @symbol

; Function definitions returning pointers
(function_definition
  declarator: (pointer_declarator
    declarator: (function_declarator
      declarator: (identifier) @name))) @symbol

; Function declarations (prototypes) — direct
(declaration
  declarator: (function_declarator
    declarator: (identifier) @name)) @symbol

; Function declarations (prototypes) — returning pointer
(declaration
  declarator: (pointer_declarator
    declarator: (function_declarator
      declarator: (identifier) @name))) @symbol

; Struct definitions (with body only, not forward declarations)
(struct_specifier
  name: (type_identifier) @name
  body: (field_declaration_list)) @symbol

; Union definitions
(union_specifier
  name: (type_identifier) @name
  body: (field_declaration_list)) @symbol

; Enum definitions
(enum_specifier
  name: (type_identifier) @name
  body: (enumerator_list)) @symbol

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
