; Functions
(function_declaration
  name: (identifier) @name) @symbol

; Methods (with receiver)
(method_declaration
  name: (field_identifier) @name) @symbol

; Type definitions (struct, interface, etc.)
(type_spec
  name: (type_identifier) @name) @symbol

; Type aliases
(type_alias
  name: (type_identifier) @name) @symbol

; Grouped const/var blocks (const (...) / var (...))
; Individual specs are also captured below; the grouped declaration
; provides the opening context line (e.g. `const (`) at levels 1+.
(const_declaration) @symbol
(var_declaration) @symbol

; Constants
(const_spec
  name: (identifier) @name) @symbol

; Package-level variables
(var_spec
  name: (identifier) @name) @symbol
