; Functions (including methods inside classes)
(function_definition
  name: (identifier) @name) @symbol

; Classes
(class_definition
  name: (identifier) @name) @symbol

; Module-level assignments (constants, type variables, dunder attributes)
(expression_statement
  (assignment
    left: (identifier) @name)) @symbol
