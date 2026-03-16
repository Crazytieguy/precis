; Function definitions (global and dotted: function foo() / function M.foo())
(function_declaration
  name: (identifier) @name) @symbol

(function_declaration
  name: (dot_index_expression) @name) @symbol

; Method definitions (colon syntax: function M:method())
(function_declaration
  name: (method_index_expression) @name) @symbol

; Variable declarations (local M = {}, local x = require "...")
(variable_declaration
  (assignment_statement
    (variable_list
      name: (identifier) @name))) @symbol

; Module method assignments: M.foo = function(...)
(assignment_statement
  (variable_list
    name: (dot_index_expression) @name)
  (expression_list
    value: (function_definition))) @symbol

; Module property assignments: M.CONSTANT = value
(assignment_statement
  (variable_list
    name: (dot_index_expression) @name)
  (expression_list
    value: (_))) @symbol
