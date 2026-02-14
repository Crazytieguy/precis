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

; Constants
(const_spec
  name: (identifier) @name) @symbol

; Package-level variables
(var_spec
  name: (identifier) @name) @symbol
