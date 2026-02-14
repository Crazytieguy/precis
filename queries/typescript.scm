; Functions
(function_declaration
  name: (identifier) @name) @symbol

; Classes
(class_declaration
  name: (type_identifier) @name) @symbol

; Class methods
(method_definition
  name: (property_identifier) @name) @symbol

; Interfaces
(interface_declaration
  name: (type_identifier) @name) @symbol

; Interface method signatures
(method_signature
  name: (property_identifier) @name) @symbol

; Enums
(enum_declaration
  name: (identifier) @name) @symbol

; Type aliases
(type_alias_declaration
  name: (type_identifier) @name) @symbol

; Const/let/var declarations
(lexical_declaration
  (variable_declarator
    name: (identifier) @name)) @symbol

; Namespaces
(internal_module
  name: (identifier) @name) @symbol
