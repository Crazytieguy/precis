; Global and local function definitions
(function_declaration
  name: (identifier) @name) @symbol

; Local function definitions
(function_declaration
  name: (dot_index_expression) @name) @symbol

; Variable assignments (module-level locals, constants)
(variable_declaration
  (assignment_statement
    (variable_list
      name: (identifier) @name))) @symbol

; Function assignments: M.foo = function(...)
(assignment_statement
  (variable_list
    name: (dot_index_expression) @name)
  (expression_list
    value: (function_definition))) @symbol
