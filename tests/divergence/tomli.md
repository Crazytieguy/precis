Score(3000)=0.482 I=0.800 C=0.290 ns_rows≤3K=18/54 grid(1000/1442/2080/3000/4327/6240/9000)=0.706/0.573/0.575/0.482/0.561/0.536/0.534

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
| walker |  | 220 | 65 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.930 |
| ns | 270 |  | 98 | README intro lede: a TOML parser, TOML v1.1.0 as of 2.4.0 | 1.4 |  | 0.874 |
| ns | 300 |  | 30 | Source package listing: src/ and src/tomli/ | 1.5 |  | 0.875 |
| walker |  | 325 | 105 | Toml::Identity { file: pyproject.toml } |  |  | 0.876 |
| walker |  | 342 | 17 | Fs::DirListing { dir: scripts } |  |  | 0.877 |
| walker |  | 362 | 20 | Fs::DirListing { dir: benchmark } |  |  | 0.879 |
| walker |  | 392 | 30 | Fs::DirListing { dir: tests } |  |  | 0.881 |
| walker |  | 401 | 9 | Plaintext::DeclSurface { file: profiler/requirements.txt } |  |  | 0.881 |
| ns | 408 |  | 108 | README: the tomllib stdlib relationship | 1.6 |  | 0.799 |
| walker |  | 438 | 37 | Code::CodeKey { rung: Names, file: src/tomli/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.844 |
| ns | 496 |  | 88 | Entry points: load() and loads() signatures with docstrings | 1.7 |  | 0.797 |
| ns | 622 |  | 126 | TOMLDecodeError class docstring and its attributes | 1.8 |  | 0.705 |
| ns | 682 |  | 60 | src/tomli/_types.py type aliases (complete file body) | 1.9 |  | 0.662 |
| ns | 766 |  | 84 | README mypyc/pure-Python distribution note | 1.10 |  | 0.637 |
| walker |  | 823 | 385 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.654 |
| walker |  | 848 | 25 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.654 |
| ns | 972 |  | 206 | README section map: every H2 and H3 heading | 1.11 |  | 0.706 |
| walker |  | 1218 | 370 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.706 |
| ns | 1220 |  | 248 | Complete roster of module-level functions in _parser.py (names only) | 2.1 |  | 0.611 |
| walker |  | 1232 | 14 | Code::CodeKey { rung: Names, file: profiler/profiler_script.py, decl: 0, sub: 0, line: 0 } |  |  | 0.611 |
| walker |  | 1264 | 32 | Code::CodeKey { rung: Decl, file: profiler/profiler_script.py, decl: 1, sub: 0, line: 12 } |  |  | 0.611 |
| walker |  | 1326 | 62 | Code::CodeKey { rung: Names, file: fuzzer/fuzz.py, decl: 0, sub: 0, line: 0 } |  |  | 0.611 |
| ns | 1340 |  | 120 | Complete roster of classes in _parser.py, with Output's fields | 2.2 |  | 0.573 |
| walker |  | 1344 | 18 | Code::CodeKey { rung: Doc, file: fuzzer/fuzz.py, decl: 3, sub: 0, line: 59 } |  |  | 0.573 |
| walker |  | 1350 | 6 | Fs::DirListing { dir: tests/data } |  |  | 0.573 |
| walker |  | 1394 | 44 | Plaintext::DeclSurface { file: fuzzer/requirements.txt } |  |  | 0.573 |
| ns | 1576 |  | 236 | loads(): parse state setup and the statement-loop rule enumeration | 2.3 | 1.7 | 0.524 |
| walker |  | 1582 | 188 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.524 |
| walker |  | 1712 | 130 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.524 |
| ns | 1719 |  | 143 | parse_value() signature and the inline-nesting recursion guard | 2.4 | 2.1 | 0.502 |
| ns | 1924 |  | 205 | MAX_INLINE_NESTING and the mypyc stack-overflow rationale | 2.5 | 2.4 | 0.483 |
| walker |  | 2013 | 301 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.566 |
| walker |  | 2047 | 34 | Code::CodeKey { rung: Names, file: src/tomli/_types.py, decl: 0, sub: 0, line: 0 } |  |  | 0.575 |
| walker |  | 2217 | 170 | Code::CodeKey { rung: Names, file: src/tomli/_parser.py, decl: 0, sub: 0, line: 0 } |  |  | 0.575 |
| ns | 2357 |  | 433 | loads(): the statement dispatch body and its two top-level errors | 2.6 | 2.3 | 0.504 |
| walker |  | 2386 | 169 | Code::CodeKey { rung: Names, file: src/tomli/_parser.py, decl: 0, sub: 1, line: 0 } |  |  | 0.506 |
| walker |  | 2409 | 23 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 11, sub: 0, line: 51 } |  |  | 0.507 |
| walker |  | 2565 | 156 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 14, sub: 0, line: 57 } |  |  | 0.509 |
| ns | 2734 |  | 377 | parse_value(): string, boolean, array and inline-table dispatch | 2.7 | 2.4 | 0.463 |
| walker |  | 2772 | 207 | Code::CodeKey { rung: Names, file: src/tomli/_parser.py, decl: 0, sub: 2, line: 0 } |  |  | 0.477 |
| walker |  | 2783 | 11 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 16, sub: 0, line: 76 } |  |  | 0.477 |
| walker |  | 2797 | 14 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 31, sub: 0, line: 312 } |  |  | 0.480 |
| walker |  | 2847 | 50 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 27, sub: 0, line: 278 } |  |  | 0.481 |
| walker |  | 2893 | 46 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 29, sub: 0, line: 283 } |  |  | 0.482 |
| walker |  | 2959 | 66 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 34, sub: 0, line: 327 } |  |  | 0.482 |
| walker |  | 3042 | 83 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 17, sub: 0, line: 87 } |  |  | 0.482 |
| ns | 3122 |  | 388 | parse_value(): datetime, number and special-float dispatch | 2.8 | 2.4 | 0.451 |
| walker |  | 3257 | 215 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 20, sub: 0, line: 220 } |  |  | 0.453 |
| walker |  | 3270 | 13 | Code::CodeKey { rung: Doc, file: src/tomli/_parser.py, decl: 19, sub: 0, line: 149 } |  |  | 0.458 |
| walker |  | 3284 | 14 | Code::CodeKey { rung: Doc, file: src/tomli/_parser.py, decl: 20, sub: 0, line: 220 } |  |  | 0.464 |
| walker |  | 3299 | 15 | Code::CodeKey { rung: Doc, file: src/tomli/_parser.py, decl: 18, sub: 0, line: 137 } |  |  | 0.471 |
| ns | 3453 |  | 331 | Character-class constants: the complete set | 2.9 |  | 0.491 |
| walker |  | 3488 | 189 | Code::CodeKey { rung: Names, file: src/tomli/_parser.py, decl: 0, sub: 3, line: 0 } |  |  | 0.523 |
| walker |  | 3520 | 32 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 46, sub: 0, line: 564 } |  |  | 0.523 |
| ns | 3550 |  | 97 | load() body: the binary-mode requirement | 2.10 | 1.7 | 0.514 |
| walker |  | 3555 | 35 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 39, sub: 0, line: 413 } |  |  | 0.514 |
| walker |  | 3592 | 37 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 44, sub: 0, line: 502 } |  |  | 0.514 |
| walker |  | 3630 | 38 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 40, sub: 0, line: 447 } |  |  | 0.514 |
| ns | 3662 |  | 112 | loads() prologue: CRLF normalisation and the str type check | 2.11 | 1.7 | 0.506 |
| walker |  | 3669 | 39 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 45, sub: 0, line: 528 } |  |  | 0.506 |
| walker |  | 3835 | 166 | Code::CodeKey { rung: Names, file: src/tomli/_parser.py, decl: 0, sub: 4, line: 0 } |  |  | 0.544 |
| ns | 3836 |  | 174 | BASIC_STR_ESCAPE_REPLACEMENTS: the full escape table | 2.12 |  | 0.561 |
| walker |  | 3871 | 36 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 52, sub: 0, line: 684 } |  |  | 0.563 |
| ns | 3872 |  | 36 | tests/ and tests/data/ listings (complete) | 3.1 |  | 0.573 |
| walker |  | 3903 | 32 | Code::CodeKey { rung: Doc, file: src/tomli/_parser.py, decl: 15, sub: 0, line: 71 } |  |  | 0.586 |
| ns | 3931 |  | 59 | tests/__init__.py: the tomli-as-tomllib alias | 3.2 |  | 0.580 |
| walker |  | 4145 | 242 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.580 |
| ns | 4195 |  | 264 | Complete roster of test classes and test methods | 3.3 |  | 0.561 |
| walker |  | 4321 | 176 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.561 |
| ns | 4371 |  | 176 | tox core configuration: the interpreter matrix and default command | 3.4 |  | 0.550 |
| walker |  | 4416 | 95 | Code::CodeKey { rung: Doc, file: src/tomli/_parser.py, decl: 54, sub: 0, line: 764 } |  |  | 0.551 |
| ns | 4523 |  | 152 | setup.py: the mypyc build path | 3.5 |  | 0.537 |
| walker |  | 4591 | 175 | Code::CodeKey { rung: Names, file: src/tomli/_re.py, decl: 0, sub: 0, line: 0 } |  |  | 0.538 |
| walker |  | 4603 | 12 | Code::CodeKey { rung: Decl, file: src/tomli/_re.py, decl: 7, sub: 0, line: 98 } |  |  | 0.538 |
| walker |  | 4707 | 104 | Code::CodeKey { rung: Decl, file: src/tomli/_re.py, decl: 2, sub: 0, line: 17 } |  |  | 0.538 |
| ns | 4717 |  | 194 | pyproject.toml: build backend and project metadata | 3.6 |  | 0.533 |
| ns | 4775 |  | 58 | Listings of the helper trees: benchmark, fuzzer, profiler, scripts, workflows | 3.7 |  | 0.547 |
| walker |  | 4866 | 159 | Code::CodeKey { rung: Decl, file: src/tomli/_re.py, decl: 5, sub: 0, line: 46 } |  |  | 0.547 |
| ns | 4958 |  | 183 | test_data.py: how the TOML corpus is discovered and compared | 3.8 | 3.3 | 0.536 |
| ns | 5097 |  | 139 | tests/burntsushi.py: purpose and complete function roster | 3.9 |  | 0.530 |
| walker |  | 5103 | 237 | Code::CodeKey { rung: Decl, file: src/tomli/_re.py, decl: 3, sub: 0, line: 26 } |  |  | 0.531 |
| walker |  | 5213 | 110 | Code::CodeKey { rung: Doc, file: src/tomli/_parser.py, decl: 16, sub: 0, line: 76 } |  |  | 0.559 |
| ns | 5348 |  | 251 | Flags: the two flag constants, the state fields, and the complete method roster | 4.1 | 2.2 | 0.562 |
| ns | 5472 |  | 124 | NestedDict: the parsed-document container and its two methods | 4.2 | 2.2 | 0.562 |
| walker |  | 5554 | 341 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.562 |
| walker |  | 5611 | 57 | Code::CodeKey { rung: Body, file: fuzzer/fuzz.py, decl: 2, sub: 0, line: 53 } |  |  | 0.562 |
| walker |  | 5675 | 64 | Code::CodeKey { rung: Body, file: fuzzer/fuzz.py, decl: 4, sub: 0, line: 71 } |  |  | 0.562 |
| ns | 5733 |  | 261 | create_dict_rule(): the [table] statement | 4.3 | 2.1 | 0.549 |
| ns | 6033 |  | 300 | create_list_rule(): the [[array of tables]] statement | 4.4 | 2.1 | 0.536 |
| walker |  | 6067 | 392 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.536 |
| walker |  | 6251 | 184 | Markdown::Section { file: README.md, section_index: 10, keeps_default_concavity: false } |  |  | 0.536 |
| ns | 6511 |  | 478 | key_value_rule(): dotted keys, pending flags and immutability | 4.5 | 2.1 | 0.517 |
| ns | 6690 |  | 179 | parse_key_part(): bare, literal and basic key forms | 4.6 | 2.1 | 0.509 |
| walker |  | 6767 | 516 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.509 |
| walker |  | 6801 | 34 | Code::CodeKey { rung: Body, file: src/tomli/_re.py, decl: 9, sub: 0, line: 116 } |  |  | 0.509 |
| walker |  | 6907 | 106 | Code::CodeKey { rung: Body, file: fuzzer/fuzz.py, decl: 3, sub: 0, line: 59 } |  |  | 0.509 |
| walker |  | 6969 | 62 | Code::CodeKey { rung: Doc, file: src/tomli/_re.py, decl: 6, sub: 0, line: 59 } |  |  | 0.509 |
| ns | 6975 |  | 285 | parse_array(): array literals and trailing commas | 4.7 | 2.1 | 0.497 |
| walker |  | 6977 | 8 | Plaintext::DeclSurface { file: scripts/requirements.txt } |  |  | 0.497 |
| walker |  | 7047 | 70 | Code::CodeKey { rung: Body, file: src/tomli/_re.py, decl: 7, sub: 0, line: 98 } |  |  | 0.497 |
| walker |  | 7060 | 13 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 22, sub: 0, line: 233 } |  |  | 0.498 |
| walker |  | 7403 | 343 | Code::CodeKey { rung: Body, file: fuzzer/fuzz.py, decl: 1, sub: 0, line: 20 } |  |  | 0.498 |
| walker |  | 7418 | 15 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 47, sub: 0, line: 595 } |  |  | 0.498 |
| ns | 7464 |  | 489 | parse_inline_table(): inline tables and their local flag scope | 4.8 | 2.1 | 0.482 |
| walker |  | 7501 | 83 | Code::CodeKey { rung: Body, file: src/tomli/_re.py, decl: 8, sub: 0, line: 109 } |  |  | 0.482 |
| walker |  | 7602 | 101 | Fs::DirListing { dir: tests/data/valid } |  |  | 0.482 |
| walker |  | 7607 | 5 | Fs::DirListing { dir: tests/data/valid/_external } |  |  | 0.482 |
| walker |  | 7613 | 6 | Fs::DirListing { dir: tests/data/valid/_external/toml-test } |  |  | 0.482 |
| walker |  | 7635 | 22 | Fs::DirListing { dir: tests/data/valid/dates-and-times } |  |  | 0.482 |
| walker |  | 7661 | 26 | Fs::DirListing { dir: tests/data/valid/array } |  |  | 0.482 |
| ns | 7665 |  | 201 | parse_hex_char() and is_unicode_scalar_value(): escape validation | 4.9 | 2.1 | 0.476 |
| walker |  | 7687 | 26 | Fs::DirListing { dir: tests/data/valid/inline-table } |  |  | 0.476 |
| walker |  | 7715 | 28 | Fs::DirListing { dir: tests/data/valid/multiline-basic-str } |  |  | 0.476 |
| walker |  | 7734 | 19 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 32, sub: 0, line: 313 } |  |  | 0.483 |
| walker |  | 7751 | 17 | Code::CodeKey { rung: Names, file: tests/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.484 |
| walker |  | 7775 | 24 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 43, sub: 0, line: 497 } |  |  | 0.484 |
| ns | 7807 |  | 142 | parse_literal_str() and parse_one_line_basic_str(): the two short string entry points | 4.10 | 2.1 | 0.482 |
| walker |  | 7929 | 154 | Fs::DirListing { dir: tests/data/invalid } |  |  | 0.482 |
| walker |  | 7934 | 5 | Fs::DirListing { dir: tests/data/invalid/_external } |  |  | 0.482 |
| walker |  | 7940 | 6 | Fs::DirListing { dir: tests/data/invalid/dates-and-times } |  |  | 0.482 |
| walker |  | 7946 | 6 | Fs::DirListing { dir: tests/data/invalid/literal-str } |  |  | 0.482 |
| walker |  | 7952 | 6 | Fs::DirListing { dir: tests/data/invalid/_external/toml-test } |  |  | 0.482 |
| walker |  | 7967 | 15 | Fs::DirListing { dir: tests/data/invalid/multiline-literal-str } |  |  | 0.482 |
| walker |  | 7985 | 18 | Fs::DirListing { dir: tests/data/invalid/array-of-tables } |  |  | 0.482 |
| walker |  | 8003 | 18 | Fs::DirListing { dir: tests/data/invalid/boolean } |  |  | 0.482 |
| walker |  | 8024 | 21 | Fs::DirListing { dir: tests/data/invalid/table } |  |  | 0.482 |
| ns | 8041 |  | 234 | make_safe_parse_float(): the parse_float contract | 4.11 | 2.1 | 0.478 |
| walker |  | 8047 | 23 | Fs::DirListing { dir: tests/data/invalid/array } |  |  | 0.478 |
| walker |  | 8080 | 33 | Fs::DirListing { dir: tests/data/invalid/dotted-keys } |  |  | 0.478 |
| walker |  | 8120 | 40 | Fs::DirListing { dir: tests/data/invalid/keys-and-vals } |  |  | 0.478 |
| ns | 8133 |  | 92 | TOMLDecodeError.__init__ signature and the deprecated free-form form | 5.1 | 1.8 | 0.484 |
| walker |  | 8162 | 42 | Fs::DirListing { dir: tests/data/invalid/multiline-basic-str } |  |  | 0.484 |
| walker |  | 8251 | 89 | Fs::DirListing { dir: tests/data/invalid/inline-table } |  |  | 0.484 |
| walker |  | 8278 | 27 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 53, sub: 0, line: 760 } |  |  | 0.484 |
| walker |  | 8306 | 28 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 28, sub: 0, line: 279 } |  |  | 0.490 |
| ns | 8360 |  | 227 | TOMLDecodeError: line/column computation and message formatting | 5.2 | 5.1 | 0.482 |
| walker |  | 8362 | 56 | Fs::DirListing { dir: tests/data/invalid/_external/toml-test/invalid } |  |  | 0.482 |
| walker |  | 8389 | 27 | Code::CodeKey { rung: Names, file: benchmark/run.py, decl: 0, sub: 0, line: 0 } |  |  | 0.482 |
| walker |  | 8449 | 60 | Code::CodeKey { rung: Decl, file: benchmark/run.py, decl: 1, sub: 0, line: 15 } |  |  | 0.482 |
| walker |  | 8464 | 15 | Code::CodeKey { rung: Names, file: tests/test_error.py, decl: 0, sub: 0, line: 0 } |  |  | 0.482 |
| ns | 8535 |  | 175 | Complete roster of src/tomli/_re.py: four regexes and four functions | 5.3 |  | 0.495 |
| walker |  | 8564 | 100 | Code::CodeKey { rung: Decl, file: tests/test_error.py, decl: 1, sub: 0, line: 13 } |  |  | 0.499 |
| walker |  | 8615 | 51 | Fs::DirListing { dir: tests/data/invalid/_external/toml-test/invalid/local-time } |  |  | 0.499 |
| walker |  | 8631 | 16 | Code::CodeKey { rung: Names, file: scripts/use_setuptools.py, decl: 0, sub: 0, line: 0 } |  |  | 0.499 |
| walker |  | 8647 | 16 | Code::CodeKey { rung: Names, file: tests/test_misc.py, decl: 0, sub: 0, line: 0 } |  |  | 0.501 |
| walker |  | 8743 | 96 | Code::CodeKey { rung: Decl, file: tests/test_misc.py, decl: 1, sub: 0, line: 17 } |  |  | 0.513 |
| walker |  | 8763 | 20 | Code::CodeKey { rung: Body, file: tests/test_misc.py, decl: 8, sub: 0, line: 133 } |  |  | 0.513 |
| ns | 8772 |  | 237 | RE_NUMBER: the integer and float grammar | 5.4 | 5.3 | 0.525 |
| walker |  | 8801 | 38 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 21, sub: 0, line: 229 } |  |  | 0.530 |
| walker |  | 8887 | 86 | Code::CodeKey { rung: Names, file: tests/burntsushi.py, decl: 0, sub: 0, line: 0 } |  |  | 0.533 |
| ns | 8901 |  | 129 | match_to_datetime docstring and the cached_tz cache-size note | 5.5 | 5.3 | 0.532 |
| walker |  | 8906 | 19 | Code::CodeKey { rung: Body, file: tests/burntsushi.py, decl: 4, sub: 0, line: 92 } |  |  | 0.532 |
| walker |  | 8944 | 38 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 23, sub: 0, line: 236 } |  |  | 0.533 |
| walker |  | 9058 | 114 | Code::CodeKey { rung: Names, file: tests/test_data.py, decl: 0, sub: 0, line: 0 } |  |  | 0.539 |
| walker |  | 9080 | 22 | Code::CodeKey { rung: Decl, file: tests/test_data.py, decl: 6, sub: 0, line: 27 } |  |  | 0.543 |
| ns | 9133 |  | 232 | Complete roster of tox environments with their descriptions | 6.1 | 3.4 | 0.538 |
| walker |  | 9148 | 68 | Fs::DirListing { dir: tests/data/invalid/_external/toml-test/invalid/spec-1.1.0 } |  |  | 0.538 |
| walker |  | 9209 | 61 | Plaintext::DeclSurface { file: benchmark/requirements.txt } |  |  | 0.538 |
| walker |  | 9234 | 25 | Plaintext::Whole { file: benchmark/requirements.txt } |  |  | 0.538 |
| walker |  | 9266 | 32 | Code::CodeKey { rung: Doc, file: tests/burntsushi.py, decl: 2, sub: 0, line: 42 } |  |  | 0.542 |
| ns | 9339 |  | 206 | tomllib.md: section map and the CPython sync procedure | 6.2 |  | 0.537 |
| walker |  | 9352 | 86 | Fs::DirListing { dir: tests/data/invalid/_external/toml-test/invalid/local-date } |  |  | 0.537 |
| ns | 9473 |  | 134 | CHANGELOG.md: the two most recent releases | 6.3 |  | 0.530 |
| ns | 9638 |  | 165 | CI workflow: the complete job list and test matrix | 6.4 |  | 0.526 |
| walker |  | 9689 | 337 | Code::CodeKey { rung: Body, file: src/tomli/_re.py, decl: 6, sub: 0, line: 59 } |  |  | 0.526 |
| walker |  | 9740 | 51 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 33, sub: 0, line: 318 } |  |  | 0.526 |
| ns | 9809 |  | 171 | pre-commit: the complete list of hook ids | 6.5 |  | 0.522 |
| walker |  | 9836 | 96 | Fs::DirListing { dir: tests/data/invalid/_external/toml-test/invalid/local-datetime } |  |  | 0.522 |
| walker |  | 9952 | 116 | Fs::DirListing { dir: tests/data/invalid/_external/toml-test/invalid/bool } |  |  | 0.522 |
| walker |  | 9993 | 41 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 24, sub: 0, line: 241 } |  |  | 0.523 |
| ns | 9994 |  | 185 | Lint and version-bump configuration: .flake8 and .bumpversion.cfg | 6.6 |  | 0.517 |
