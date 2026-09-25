Score(3000)=0.459 I=0.775 C=0.271 ns_rows≤3K=18/54 grid(1000/1442/2080/3000/4327/6240/9000)=0.582/0.574/0.543/0.459/0.464/0.538/0.514

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 29 |  | 29 | README title and tagline | 1.1 |  | 0.000 |
| walker |  | 62 | 62 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 66 | 4 | Fs::DirListing { dir: src } |  |  | 0.000 |
| walker |  | 74 | 8 | Fs::DirListing { dir: fuzzer } |  |  | 0.000 |
| walker |  | 83 | 9 | Fs::DirListing { dir: profiler } |  |  | 0.000 |
| walker |  | 86 | 3 | Fs::DirListing { dir: .github } |  |  | 0.000 |
| walker |  | 90 | 4 | Fs::DirListing { dir: .github/workflows } |  |  | 0.000 |
| ns | 91 |  | 62 | Root directory listing (complete) | 1.2 |  | 0.661 |
| walker |  | 116 | 26 | Fs::DirListing { dir: src/tomli } |  |  | 0.692 |
| walker |  | 155 | 39 | Code::CodeKey { rung: ModuleDoc, file: src/tomli/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.696 |
| ns | 172 |  | 81 | The complete public API: src/tomli/__init__.py | 1.3 |  | 0.634 |
| walker |  | 260 | 105 | Toml::Identity { file: pyproject.toml } |  |  | 0.635 |
| ns | 270 |  | 98 | README intro lede: a TOML parser, TOML v1.1.0 as of 2.4.0 | 1.4 |  | 0.593 |
| walker |  | 277 | 17 | Fs::DirListing { dir: scripts } |  |  | 0.594 |
| ns | 300 |  | 30 | Source package listing: src/ and src/tomli/ | 1.5 |  | 0.605 |
| walker |  | 335 | 58 | Markdown::HeadingsOutline { file: tomllib.md } |  |  | 0.605 |
| walker |  | 355 | 20 | Fs::DirListing { dir: benchmark } |  |  | 0.608 |
| walker |  | 385 | 30 | Fs::DirListing { dir: tests } |  |  | 0.610 |
| walker |  | 394 | 9 | Plaintext::DeclSurface { file: profiler/requirements.txt } |  |  | 0.610 |
| ns | 408 |  | 108 | README: the tomllib stdlib relationship | 1.6 |  | 0.554 |
| ns | 496 |  | 88 | Entry points: load() and loads() signatures with docstrings | 1.7 |  | 0.523 |
| ns | 622 |  | 126 | TOMLDecodeError class docstring and its attributes | 1.8 |  | 0.463 |
| ns | 682 |  | 60 | src/tomli/_types.py type aliases (complete file body) | 1.9 |  | 0.434 |
| ns | 766 |  | 84 | README mypyc/pure-Python distribution note | 1.10 |  | 0.418 |
| walker |  | 787 | 393 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.603 |
| walker |  | 824 | 37 | Code::CodeKey { rung: Names, file: src/tomli/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.638 |
| ns | 972 |  | 206 | README section map: every H2 and H3 heading | 1.11 |  | 0.558 |
| walker |  | 1209 | 385 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.706 |
| walker |  | 1218 | 9 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.706 |
| ns | 1220 |  | 248 | Complete roster of module-level functions in _parser.py (names only) | 2.1 |  | 0.612 |
| walker |  | 1243 | 25 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.612 |
| walker |  | 1257 | 14 | Code::CodeKey { rung: Names, file: profiler/profiler_script.py, decl: 0, sub: 0, line: 0 } |  |  | 0.612 |
| walker |  | 1289 | 32 | Code::CodeKey { rung: Decl, file: profiler/profiler_script.py, decl: 1, sub: 0, line: 12 } |  |  | 0.612 |
| walker |  | 1295 | 6 | Fs::DirListing { dir: tests/data } |  |  | 0.612 |
| ns | 1340 |  | 120 | Complete roster of classes in _parser.py, with Output's fields | 2.2 |  | 0.574 |
| walker |  | 1357 | 62 | Code::CodeKey { rung: Names, file: fuzzer/fuzz.py, decl: 0, sub: 0, line: 0 } |  |  | 0.574 |
| walker |  | 1375 | 18 | Code::CodeKey { rung: Doc, file: fuzzer/fuzz.py, decl: 3, sub: 0, line: 59 } |  |  | 0.574 |
| walker |  | 1432 | 57 | Code::CodeKey { rung: Body, file: fuzzer/fuzz.py, decl: 2, sub: 0, line: 53 } |  |  | 0.574 |
| walker |  | 1496 | 64 | Code::CodeKey { rung: Body, file: fuzzer/fuzz.py, decl: 4, sub: 0, line: 71 } |  |  | 0.574 |
| walker |  | 1540 | 44 | Plaintext::DeclSurface { file: fuzzer/requirements.txt } |  |  | 0.574 |
| ns | 1576 |  | 236 | loads(): parse state setup and the statement-loop rule enumeration | 2.3 | 1.7 | 0.524 |
| ns | 1719 |  | 143 | parse_value() signature and the inline-nesting recursion guard | 2.4 | 2.1 | 0.502 |
| walker |  | 1728 | 188 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.502 |
| walker |  | 1858 | 130 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.502 |
| ns | 1924 |  | 205 | MAX_INLINE_NESTING and the mypyc stack-overflow rationale | 2.5 | 2.4 | 0.483 |
| walker |  | 2159 | 301 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.566 |
| walker |  | 2193 | 34 | Code::CodeKey { rung: Names, file: src/tomli/_types.py, decl: 0, sub: 0, line: 0 } |  |  | 0.575 |
| walker |  | 2299 | 106 | Code::CodeKey { rung: Body, file: fuzzer/fuzz.py, decl: 3, sub: 0, line: 59 } |  |  | 0.575 |
| ns | 2357 |  | 433 | loads(): the statement dispatch body and its two top-level errors | 2.6 | 2.3 | 0.503 |
| walker |  | 2541 | 242 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.503 |
| walker |  | 2717 | 176 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.503 |
| ns | 2734 |  | 377 | parse_value(): string, boolean, array and inline-table dispatch | 2.7 | 2.4 | 0.458 |
| walker |  | 2892 | 175 | Code::CodeKey { rung: Names, file: src/tomli/_re.py, decl: 0, sub: 0, line: 0 } |  |  | 0.458 |
| walker |  | 2904 | 12 | Code::CodeKey { rung: Decl, file: src/tomli/_re.py, decl: 7, sub: 0, line: 98 } |  |  | 0.458 |
| walker |  | 3008 | 104 | Code::CodeKey { rung: Decl, file: src/tomli/_re.py, decl: 2, sub: 0, line: 17 } |  |  | 0.459 |
| ns | 3122 |  | 388 | parse_value(): datetime, number and special-float dispatch | 2.8 | 2.4 | 0.428 |
| walker |  | 3167 | 159 | Code::CodeKey { rung: Decl, file: src/tomli/_re.py, decl: 5, sub: 0, line: 46 } |  |  | 0.429 |
| walker |  | 3201 | 34 | Code::CodeKey { rung: Body, file: src/tomli/_re.py, decl: 9, sub: 0, line: 116 } |  |  | 0.429 |
| walker |  | 3438 | 237 | Code::CodeKey { rung: Decl, file: src/tomli/_re.py, decl: 3, sub: 0, line: 26 } |  |  | 0.430 |
| ns | 3453 |  | 331 | Character-class constants: the complete set | 2.9 |  | 0.412 |
| walker |  | 3500 | 62 | Code::CodeKey { rung: Doc, file: src/tomli/_re.py, decl: 6, sub: 0, line: 59 } |  |  | 0.413 |
| ns | 3550 |  | 97 | load() body: the binary-mode requirement | 2.10 | 1.7 | 0.406 |
| ns | 3662 |  | 112 | loads() prologue: CRLF normalisation and the str type check | 2.11 | 1.7 | 0.399 |
| walker |  | 3741 | 241 | Code::CodeKey { rung: Names, file: src/tomli/_parser.py, decl: 0, sub: 0, line: 0 } |  |  | 0.420 |
| ns | 3836 |  | 174 | BASIC_STR_ESCAPE_REPLACEMENTS: the full escape table | 2.12 |  | 0.411 |
| ns | 3872 |  | 36 | tests/ and tests/data/ listings (complete) | 3.1 |  | 0.428 |
| ns | 3931 |  | 59 | tests/__init__.py: the tomli-as-tomllib alias | 3.2 |  | 0.424 |
| walker |  | 3969 | 228 | Code::CodeKey { rung: Names, file: src/tomli/_parser.py, decl: 0, sub: 1, line: 0 } |  |  | 0.445 |
| walker |  | 3980 | 11 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 16, sub: 0, line: 76 } |  |  | 0.445 |
| walker |  | 3994 | 14 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 31, sub: 0, line: 312 } |  |  | 0.447 |
| walker |  | 4017 | 23 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 11, sub: 0, line: 51 } |  |  | 0.455 |
| walker |  | 4067 | 50 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 27, sub: 0, line: 278 } |  |  | 0.456 |
| walker |  | 4113 | 46 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 29, sub: 0, line: 283 } |  |  | 0.457 |
| walker |  | 4126 | 13 | Code::CodeKey { rung: Doc, file: src/tomli/_parser.py, decl: 19, sub: 0, line: 149 } |  |  | 0.461 |
| ns | 4195 |  | 264 | Complete roster of test classes and test methods | 3.3 |  | 0.446 |
| walker |  | 4209 | 83 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 17, sub: 0, line: 87 } |  |  | 0.446 |
| walker |  | 4224 | 15 | Code::CodeKey { rung: Doc, file: src/tomli/_parser.py, decl: 18, sub: 0, line: 137 } |  |  | 0.452 |
| ns | 4371 |  | 176 | tox core configuration: the interpreter matrix and default command | 3.4 |  | 0.443 |
| walker |  | 4380 | 156 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 14, sub: 0, line: 57 } |  |  | 0.474 |
| walker |  | 4412 | 32 | Code::CodeKey { rung: Doc, file: src/tomli/_parser.py, decl: 15, sub: 0, line: 71 } |  |  | 0.483 |
| ns | 4523 |  | 152 | setup.py: the mypyc build path | 3.5 |  | 0.472 |
| walker |  | 4627 | 215 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 20, sub: 0, line: 220 } |  |  | 0.475 |
| walker |  | 4641 | 14 | Code::CodeKey { rung: Doc, file: src/tomli/_parser.py, decl: 20, sub: 0, line: 220 } |  |  | 0.481 |
| ns | 4717 |  | 194 | pyproject.toml: build backend and project metadata | 3.6 |  | 0.477 |
| ns | 4775 |  | 58 | Listings of the helper trees: benchmark, fuzzer, profiler, scripts, workflows | 3.7 |  | 0.494 |
| walker |  | 4851 | 210 | Code::CodeKey { rung: Names, file: src/tomli/_parser.py, decl: 0, sub: 2, line: 0 } |  |  | 0.507 |
| walker |  | 4886 | 35 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 39, sub: 0, line: 413 } |  |  | 0.507 |
| walker |  | 4924 | 38 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 40, sub: 0, line: 447 } |  |  | 0.507 |
| ns | 4958 |  | 183 | test_data.py: how the TOML corpus is discovered and compared | 3.8 | 3.3 | 0.497 |
| walker |  | 4990 | 66 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 34, sub: 0, line: 327 } |  |  | 0.497 |
| ns | 5097 |  | 139 | tests/burntsushi.py: purpose and complete function roster | 3.9 |  | 0.491 |
| walker |  | 5217 | 227 | Code::CodeKey { rung: Names, file: src/tomli/_parser.py, decl: 0, sub: 3, line: 0 } |  |  | 0.531 |
| walker |  | 5249 | 32 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 46, sub: 0, line: 564 } |  |  | 0.531 |
| walker |  | 5285 | 36 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 52, sub: 0, line: 684 } |  |  | 0.533 |
| walker |  | 5322 | 37 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 44, sub: 0, line: 502 } |  |  | 0.533 |
| ns | 5348 |  | 251 | Flags: the two flag constants, the state fields, and the complete method roster | 4.1 | 2.2 | 0.536 |
| walker |  | 5361 | 39 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 45, sub: 0, line: 528 } |  |  | 0.536 |
| walker |  | 5456 | 95 | Code::CodeKey { rung: Doc, file: src/tomli/_parser.py, decl: 54, sub: 0, line: 764 } |  |  | 0.537 |
| ns | 5472 |  | 124 | NestedDict: the parsed-document container and its two methods | 4.2 | 2.2 | 0.538 |
| walker |  | 5566 | 110 | Code::CodeKey { rung: Doc, file: src/tomli/_parser.py, decl: 16, sub: 0, line: 76 } |  |  | 0.564 |
| ns | 5733 |  | 261 | create_dict_rule(): the [table] statement | 4.3 | 2.1 | 0.551 |
| walker |  | 5907 | 341 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.551 |
| ns | 6033 |  | 300 | create_list_rule(): the [[array of tables]] statement | 4.4 | 2.1 | 0.538 |
| walker |  | 6299 | 392 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.538 |
| walker |  | 6483 | 184 | Markdown::Section { file: README.md, section_index: 10, keeps_default_concavity: false } |  |  | 0.538 |
| ns | 6511 |  | 478 | key_value_rule(): dotted keys, pending flags and immutability | 4.5 | 2.1 | 0.518 |
| ns | 6690 |  | 179 | parse_key_part(): bare, literal and basic key forms | 4.6 | 2.1 | 0.511 |
| ns | 6975 |  | 285 | parse_array(): array literals and trailing commas | 4.7 | 2.1 | 0.498 |
| walker |  | 6999 | 516 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.498 |
| walker |  | 7069 | 70 | Code::CodeKey { rung: Body, file: src/tomli/_re.py, decl: 7, sub: 0, line: 98 } |  |  | 0.498 |
| walker |  | 7412 | 343 | Code::CodeKey { rung: Body, file: fuzzer/fuzz.py, decl: 1, sub: 0, line: 20 } |  |  | 0.498 |
| ns | 7464 |  | 489 | parse_inline_table(): inline tables and their local flag scope | 4.8 | 2.1 | 0.482 |
| walker |  | 7495 | 83 | Code::CodeKey { rung: Body, file: src/tomli/_re.py, decl: 8, sub: 0, line: 109 } |  |  | 0.482 |
| walker |  | 7508 | 13 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 22, sub: 0, line: 233 } |  |  | 0.483 |
| walker |  | 7523 | 15 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 47, sub: 0, line: 595 } |  |  | 0.483 |
| walker |  | 7542 | 19 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 32, sub: 0, line: 313 } |  |  | 0.490 |
| walker |  | 7550 | 8 | Plaintext::DeclSurface { file: scripts/requirements.txt } |  |  | 0.490 |
| walker |  | 7574 | 24 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 43, sub: 0, line: 497 } |  |  | 0.490 |
| walker |  | 7639 | 65 | Markdown::HeadingsOutline { file: benchmark/README.md } |  |  | 0.490 |
| ns | 7665 |  | 201 | parse_hex_char() and is_unicode_scalar_value(): escape validation | 4.9 | 2.1 | 0.485 |
| walker |  | 7666 | 27 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 53, sub: 0, line: 760 } |  |  | 0.486 |
| walker |  | 7767 | 101 | Fs::DirListing { dir: tests/data/valid } |  |  | 0.486 |
| walker |  | 7772 | 5 | Fs::DirListing { dir: tests/data/valid/_external } |  |  | 0.486 |
| walker |  | 7778 | 6 | Fs::DirListing { dir: tests/data/valid/_external/toml-test } |  |  | 0.486 |
| walker |  | 7800 | 22 | Fs::DirListing { dir: tests/data/valid/dates-and-times } |  |  | 0.486 |
| ns | 7807 |  | 142 | parse_literal_str() and parse_one_line_basic_str(): the two short string entry points | 4.10 | 2.1 | 0.483 |
| walker |  | 7826 | 26 | Fs::DirListing { dir: tests/data/valid/array } |  |  | 0.483 |
| walker |  | 7852 | 26 | Fs::DirListing { dir: tests/data/valid/inline-table } |  |  | 0.483 |
| walker |  | 7880 | 28 | Fs::DirListing { dir: tests/data/valid/multiline-basic-str } |  |  | 0.483 |
| walker |  | 7908 | 28 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 28, sub: 0, line: 279 } |  |  | 0.490 |
| walker |  | 7946 | 38 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 21, sub: 0, line: 229 } |  |  | 0.496 |
| ns | 8041 |  | 234 | make_safe_parse_float(): the parse_float contract | 4.11 | 2.1 | 0.492 |
| ns | 8133 |  | 92 | TOMLDecodeError.__init__ signature and the deprecated free-form form | 5.1 | 1.8 | 0.497 |
| walker |  | 8283 | 337 | Code::CodeKey { rung: Body, file: src/tomli/_re.py, decl: 6, sub: 0, line: 59 } |  |  | 0.497 |
| ns | 8360 |  | 227 | TOMLDecodeError: line/column computation and message formatting | 5.2 | 5.1 | 0.489 |
| walker |  | 8437 | 154 | Fs::DirListing { dir: tests/data/invalid } |  |  | 0.489 |
| walker |  | 8442 | 5 | Fs::DirListing { dir: tests/data/invalid/_external } |  |  | 0.489 |
| walker |  | 8448 | 6 | Fs::DirListing { dir: tests/data/invalid/dates-and-times } |  |  | 0.489 |
| walker |  | 8454 | 6 | Fs::DirListing { dir: tests/data/invalid/literal-str } |  |  | 0.489 |
| walker |  | 8460 | 6 | Fs::DirListing { dir: tests/data/invalid/_external/toml-test } |  |  | 0.489 |
| walker |  | 8475 | 15 | Fs::DirListing { dir: tests/data/invalid/multiline-literal-str } |  |  | 0.489 |
| walker |  | 8493 | 18 | Fs::DirListing { dir: tests/data/invalid/array-of-tables } |  |  | 0.489 |
| walker |  | 8511 | 18 | Fs::DirListing { dir: tests/data/invalid/boolean } |  |  | 0.489 |
| walker |  | 8532 | 21 | Fs::DirListing { dir: tests/data/invalid/table } |  |  | 0.489 |
| ns | 8535 |  | 175 | Complete roster of src/tomli/_re.py: four regexes and four functions | 5.3 |  | 0.501 |
| walker |  | 8555 | 23 | Fs::DirListing { dir: tests/data/invalid/array } |  |  | 0.501 |
| walker |  | 8588 | 33 | Fs::DirListing { dir: tests/data/invalid/dotted-keys } |  |  | 0.501 |
| walker |  | 8628 | 40 | Fs::DirListing { dir: tests/data/invalid/keys-and-vals } |  |  | 0.501 |
| walker |  | 8670 | 42 | Fs::DirListing { dir: tests/data/invalid/multiline-basic-str } |  |  | 0.501 |
| walker |  | 8759 | 89 | Fs::DirListing { dir: tests/data/invalid/inline-table } |  |  | 0.501 |
| ns | 8772 |  | 237 | RE_NUMBER: the integer and float grammar | 5.4 | 5.3 | 0.514 |
| walker |  | 8776 | 17 | Code::CodeKey { rung: Names, file: tests/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.514 |
| walker |  | 8814 | 38 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 23, sub: 0, line: 236 } |  |  | 0.515 |
| walker |  | 8870 | 56 | Fs::DirListing { dir: tests/data/invalid/_external/toml-test/invalid } |  |  | 0.515 |
| ns | 8901 |  | 129 | match_to_datetime docstring and the cached_tz cache-size note | 5.5 | 5.3 | 0.514 |
| walker |  | 8921 | 51 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 33, sub: 0, line: 318 } |  |  | 0.514 |
| walker |  | 8972 | 51 | Fs::DirListing { dir: tests/data/invalid/_external/toml-test/invalid/local-time } |  |  | 0.514 |
| walker |  | 8999 | 27 | Code::CodeKey { rung: Names, file: benchmark/run.py, decl: 0, sub: 0, line: 0 } |  |  | 0.514 |
| walker |  | 9059 | 60 | Code::CodeKey { rung: Decl, file: benchmark/run.py, decl: 1, sub: 0, line: 15 } |  |  | 0.514 |
| walker |  | 9123 | 64 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 24, sub: 0, line: 241 } |  |  | 0.516 |
| ns | 9133 |  | 232 | Complete roster of tox environments with their descriptions | 6.1 | 3.4 | 0.511 |
| walker |  | 9138 | 15 | Code::CodeKey { rung: Names, file: tests/test_error.py, decl: 0, sub: 0, line: 0 } |  |  | 0.511 |
| walker |  | 9238 | 100 | Code::CodeKey { rung: Decl, file: tests/test_error.py, decl: 1, sub: 0, line: 13 } |  |  | 0.515 |
| walker |  | 9291 | 53 | Code::CodeKey { rung: Body, file: tests/test_error.py, decl: 4, sub: 0, line: 40 } |  |  | 0.515 |
| ns | 9339 |  | 206 | tomllib.md: section map and the CPython sync procedure | 6.2 |  | 0.510 |
| walker |  | 9348 | 57 | Code::CodeKey { rung: Body, file: tests/test_error.py, decl: 3, sub: 0, line: 35 } |  |  | 0.510 |
| walker |  | 9364 | 16 | Code::CodeKey { rung: Names, file: scripts/use_setuptools.py, decl: 0, sub: 0, line: 0 } |  |  | 0.510 |
| walker |  | 9380 | 16 | Code::CodeKey { rung: Names, file: tests/test_misc.py, decl: 0, sub: 0, line: 0 } |  |  | 0.512 |
| ns | 9473 |  | 134 | CHANGELOG.md: the two most recent releases | 6.3 |  | 0.506 |
| walker |  | 9476 | 96 | Code::CodeKey { rung: Decl, file: tests/test_misc.py, decl: 1, sub: 0, line: 17 } |  |  | 0.517 |
| walker |  | 9496 | 20 | Code::CodeKey { rung: Body, file: tests/test_misc.py, decl: 8, sub: 0, line: 133 } |  |  | 0.517 |
| walker |  | 9547 | 51 | Code::CodeKey { rung: Doc, file: tests/test_misc.py, decl: 8, sub: 0, line: 133 } |  |  | 0.517 |
| walker |  | 9615 | 68 | Fs::DirListing { dir: tests/data/invalid/_external/toml-test/invalid/spec-1.1.0 } |  |  | 0.517 |
| ns | 9638 |  | 165 | CI workflow: the complete job list and test matrix | 6.4 |  | 0.513 |
| walker |  | 9701 | 86 | Code::CodeKey { rung: Names, file: tests/burntsushi.py, decl: 0, sub: 0, line: 0 } |  |  | 0.516 |
| walker |  | 9720 | 19 | Code::CodeKey { rung: Body, file: tests/burntsushi.py, decl: 4, sub: 0, line: 92 } |  |  | 0.516 |
| walker |  | 9752 | 32 | Code::CodeKey { rung: Doc, file: tests/burntsushi.py, decl: 2, sub: 0, line: 42 } |  |  | 0.520 |
| ns | 9809 |  | 171 | pre-commit: the complete list of hook ids | 6.5 |  | 0.515 |
| walker |  | 9821 | 69 | Code::CodeKey { rung: Body, file: tests/burntsushi.py, decl: 5, sub: 0, line: 96 } |  |  | 0.515 |
| walker |  | 9891 | 70 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 36, sub: 0, line: 361 } |  |  | 0.515 |
| walker |  | 9992 | 101 | Code::CodeKey { rung: Names, file: tests/test_data.py, decl: 0, sub: 0, line: 0 } |  |  | 0.519 |
| ns | 9994 |  | 185 | Lint and version-bump configuration: .flake8 and .bumpversion.cfg | 6.6 |  | 0.513 |
