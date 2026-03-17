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

; Import declarations (grouped `import (...)` blocks)
; Note: the Names/Signatures stage model is a poor fit for imports —
; the "name" of a grouped import is just the keyword "import", which
; is useless. The full import list only appears at Signatures stage.
; This is an architectural limitation: the stage progression assumes
; every symbol has a meaningful name, which imports don't.
(import_declaration) @symbol
