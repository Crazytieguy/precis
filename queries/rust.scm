; Functions
(function_item
  name: (identifier) @name) @symbol

; Structs
(struct_item
  name: (type_identifier) @name) @symbol

; Enums
(enum_item
  name: (type_identifier) @name) @symbol

; Traits
(trait_item
  name: (type_identifier) @name) @symbol

; Impl blocks
(impl_item) @symbol

; Type aliases
(type_item
  name: (type_identifier) @name) @symbol

; Constants
(const_item
  name: (identifier) @name) @symbol

; Statics
(static_item
  name: (identifier) @name) @symbol

; Macro definitions
(macro_definition
  name: (identifier) @name) @symbol

; Module declarations
(mod_item
  name: (identifier) @name) @symbol
