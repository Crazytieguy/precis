; Class declarations
(class_declaration
  name: (identifier) @name) @symbol

; Interface declarations
(interface_declaration
  name: (identifier) @name) @symbol

; Enum declarations
(enum_declaration
  name: (identifier) @name) @symbol

; Record declarations
(record_declaration
  name: (identifier) @name) @symbol

; Annotation type declarations
(annotation_type_declaration
  name: (identifier) @name) @symbol

; Method declarations (including abstract, default, static)
(method_declaration
  name: (identifier) @name) @symbol

; Constructor declarations
(constructor_declaration
  name: (identifier) @name) @symbol

; Field declarations (class-level; classified in code based on context)
(field_declaration) @symbol

; Interface constant declarations
(constant_declaration) @symbol

; Annotation type element declarations (e.g., String value();)
(annotation_type_element_declaration
  name: (identifier) @name) @symbol

; Import declarations
(import_declaration) @symbol

; Module declarations (module-info.java / JPMS)
(module_declaration) @symbol
