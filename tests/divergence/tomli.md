Score(3000)=0.459 I=0.778 C=0.271 ns_rows≤3K=18/54 grid(1000/1442/2080/3000/4327/6240/9000)=0.559/0.574/0.493/0.459/0.493/0.546/0.520

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
| walker |  | 364 | 9 | Plaintext::Whole { file: profiler/requirements.txt } |  |  | 0.608 |
| walker |  | 394 | 30 | Fs::DirListing { dir: tests } |  |  | 0.610 |
| ns | 408 |  | 108 | README: the tomllib stdlib relationship | 1.6 |  | 0.554 |
| walker |  | 454 | 60 | Toml::PackageMetadata { file: pyproject.toml } |  |  | 0.555 |
| ns | 496 |  | 88 | Entry points: load() and loads() signatures with docstrings | 1.7 |  | 0.525 |
| ns | 622 |  | 126 | TOMLDecodeError class docstring and its attributes | 1.8 |  | 0.464 |
| ns | 682 |  | 60 | src/tomli/_types.py type aliases (complete file body) | 1.9 |  | 0.436 |
| ns | 766 |  | 84 | README mypyc/pure-Python distribution note | 1.10 |  | 0.419 |
| walker |  | 847 | 393 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.604 |
| walker |  | 891 | 44 | Plaintext::Whole { file: fuzzer/requirements.txt } |  |  | 0.604 |
| walker |  | 928 | 37 | Code::CodeKey { rung: Names, file: src/tomli/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.639 |
| ns | 972 |  | 206 | README section map: every H2 and H3 heading | 1.11 |  | 0.559 |
| ns | 1220 |  | 248 | Complete roster of module-level functions in _parser.py (names only) | 2.1 |  | 0.485 |
| walker |  | 1313 | 385 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.613 |
| walker |  | 1322 | 9 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.613 |
| ns | 1340 |  | 120 | Complete roster of classes in _parser.py, with Output's fields | 2.2 |  | 0.574 |
| walker |  | 1347 | 25 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.574 |
| walker |  | 1361 | 14 | Code::CodeKey { rung: Names, file: profiler/profiler_script.py, decl: 0, sub: 0, line: 0 } |  |  | 0.574 |
| walker |  | 1393 | 32 | Code::CodeKey { rung: Decl, file: profiler/profiler_script.py, decl: 1, sub: 0, line: 12 } |  |  | 0.574 |
| walker |  | 1399 | 6 | Fs::DirListing { dir: tests/data } |  |  | 0.574 |
| walker |  | 1461 | 62 | Code::CodeKey { rung: Names, file: fuzzer/fuzz.py, decl: 0, sub: 0, line: 0 } |  |  | 0.574 |
| walker |  | 1479 | 18 | Code::CodeKey { rung: Doc, file: fuzzer/fuzz.py, decl: 3, sub: 0, line: 59 } |  |  | 0.574 |
| walker |  | 1536 | 57 | Code::CodeKey { rung: Body, file: fuzzer/fuzz.py, decl: 2, sub: 0, line: 53 } |  |  | 0.574 |
| ns | 1576 |  | 236 | loads(): parse state setup and the statement-loop rule enumeration | 2.3 | 1.7 | 0.525 |
| walker |  | 1600 | 64 | Code::CodeKey { rung: Body, file: fuzzer/fuzz.py, decl: 4, sub: 0, line: 71 } |  |  | 0.525 |
| ns | 1719 |  | 143 | parse_value() signature and the inline-nesting recursion guard | 2.4 | 2.1 | 0.503 |
| walker |  | 1775 | 175 | Markdown::Section { file: tomllib.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.503 |
| walker |  | 1784 | 9 | Markdown::ReadmeHeadline { file: benchmark/README.md } |  |  | 0.503 |
| walker |  | 1818 | 34 | Code::CodeKey { rung: Names, file: src/tomli/_types.py, decl: 0, sub: 0, line: 0 } |  |  | 0.513 |
| walker |  | 1924 | 106 | Code::CodeKey { rung: Body, file: fuzzer/fuzz.py, decl: 3, sub: 0, line: 59 } |  |  | 0.493 |
| ns | 1924 |  | 205 | MAX_INLINE_NESTING and the mypyc stack-overflow rationale | 2.5 | 2.4 | 0.493 |
| walker |  | 2112 | 188 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.493 |
| walker |  | 2242 | 130 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.493 |
| ns | 2357 |  | 433 | loads(): the statement dispatch body and its two top-level errors | 2.6 | 2.3 | 0.432 |
| walker |  | 2543 | 301 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.504 |
| walker |  | 2718 | 175 | Code::CodeKey { rung: Names, file: src/tomli/_re.py, decl: 0, sub: 0, line: 0 } |  |  | 0.505 |
| walker |  | 2730 | 12 | Code::CodeKey { rung: Decl, file: src/tomli/_re.py, decl: 7, sub: 0, line: 98 } |  |  | 0.505 |
| ns | 2734 |  | 377 | parse_value(): string, boolean, array and inline-table dispatch | 2.7 | 2.4 | 0.459 |
| walker |  | 2834 | 104 | Code::CodeKey { rung: Decl, file: src/tomli/_re.py, decl: 2, sub: 0, line: 17 } |  |  | 0.459 |
| walker |  | 2993 | 159 | Code::CodeKey { rung: Decl, file: src/tomli/_re.py, decl: 5, sub: 0, line: 46 } |  |  | 0.459 |
| walker |  | 3027 | 34 | Code::CodeKey { rung: Body, file: src/tomli/_re.py, decl: 9, sub: 0, line: 116 } |  |  | 0.459 |
| ns | 3122 |  | 388 | parse_value(): datetime, number and special-float dispatch | 2.8 | 2.4 | 0.429 |
| walker |  | 3264 | 237 | Code::CodeKey { rung: Decl, file: src/tomli/_re.py, decl: 3, sub: 0, line: 26 } |  |  | 0.431 |
| walker |  | 3326 | 62 | Code::CodeKey { rung: Doc, file: src/tomli/_re.py, decl: 6, sub: 0, line: 59 } |  |  | 0.431 |
| ns | 3453 |  | 331 | Character-class constants: the complete set | 2.9 |  | 0.413 |
| ns | 3550 |  | 97 | load() body: the binary-mode requirement | 2.10 | 1.7 | 0.406 |
| walker |  | 3567 | 241 | Code::CodeKey { rung: Names, file: src/tomli/_parser.py, decl: 0, sub: 0, line: 0 } |  |  | 0.428 |
| ns | 3662 |  | 112 | loads() prologue: CRLF normalisation and the str type check | 2.11 | 1.7 | 0.421 |
| walker |  | 3795 | 228 | Code::CodeKey { rung: Names, file: src/tomli/_parser.py, decl: 0, sub: 1, line: 0 } |  |  | 0.443 |
| walker |  | 3806 | 11 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 16, sub: 0, line: 76 } |  |  | 0.443 |
| walker |  | 3820 | 14 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 31, sub: 0, line: 312 } |  |  | 0.446 |
| ns | 3836 |  | 174 | BASIC_STR_ESCAPE_REPLACEMENTS: the full escape table | 2.12 |  | 0.436 |
| walker |  | 3843 | 23 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 11, sub: 0, line: 51 } |  |  | 0.445 |
| ns | 3872 |  | 36 | tests/ and tests/data/ listings (complete) | 3.1 |  | 0.460 |
| walker |  | 3893 | 50 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 27, sub: 0, line: 278 } |  |  | 0.461 |
| ns | 3931 |  | 59 | tests/__init__.py: the tomli-as-tomllib alias | 3.2 |  | 0.457 |
| walker |  | 3939 | 46 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 29, sub: 0, line: 283 } |  |  | 0.457 |
| walker |  | 3952 | 13 | Code::CodeKey { rung: Doc, file: src/tomli/_parser.py, decl: 19, sub: 0, line: 149 } |  |  | 0.462 |
| walker |  | 4035 | 83 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 17, sub: 0, line: 87 } |  |  | 0.462 |
| walker |  | 4050 | 15 | Code::CodeKey { rung: Doc, file: src/tomli/_parser.py, decl: 18, sub: 0, line: 137 } |  |  | 0.468 |
| ns | 4195 |  | 264 | Complete roster of test classes and test methods | 3.3 |  | 0.452 |
| walker |  | 4206 | 156 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 14, sub: 0, line: 57 } |  |  | 0.483 |
| walker |  | 4238 | 32 | Code::CodeKey { rung: Doc, file: src/tomli/_parser.py, decl: 15, sub: 0, line: 71 } |  |  | 0.493 |
| ns | 4371 |  | 176 | tox core configuration: the interpreter matrix and default command | 3.4 |  | 0.484 |
| walker |  | 4453 | 215 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 20, sub: 0, line: 220 } |  |  | 0.487 |
| walker |  | 4467 | 14 | Code::CodeKey { rung: Doc, file: src/tomli/_parser.py, decl: 20, sub: 0, line: 220 } |  |  | 0.493 |
| ns | 4523 |  | 152 | setup.py: the mypyc build path | 3.5 |  | 0.481 |
| walker |  | 4677 | 210 | Code::CodeKey { rung: Names, file: src/tomli/_parser.py, decl: 0, sub: 2, line: 0 } |  |  | 0.495 |
| walker |  | 4712 | 35 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 39, sub: 0, line: 413 } |  |  | 0.495 |
| ns | 4717 |  | 194 | pyproject.toml: build backend and project metadata | 3.6 |  | 0.502 |
| walker |  | 4750 | 38 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 40, sub: 0, line: 447 } |  |  | 0.502 |
| ns | 4775 |  | 58 | Listings of the helper trees: benchmark, fuzzer, profiler, scripts, workflows | 3.7 |  | 0.518 |
| walker |  | 4816 | 66 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 34, sub: 0, line: 327 } |  |  | 0.518 |
| ns | 4958 |  | 183 | test_data.py: how the TOML corpus is discovered and compared | 3.8 | 3.3 | 0.508 |
| walker |  | 5043 | 227 | Code::CodeKey { rung: Names, file: src/tomli/_parser.py, decl: 0, sub: 3, line: 0 } |  |  | 0.548 |
| walker |  | 5075 | 32 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 46, sub: 0, line: 564 } |  |  | 0.548 |
| ns | 5097 |  | 139 | tests/burntsushi.py: purpose and complete function roster | 3.9 |  | 0.541 |
| walker |  | 5111 | 36 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 52, sub: 0, line: 684 } |  |  | 0.543 |
| walker |  | 5148 | 37 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 44, sub: 0, line: 502 } |  |  | 0.543 |
| walker |  | 5187 | 39 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 45, sub: 0, line: 528 } |  |  | 0.543 |
| walker |  | 5282 | 95 | Code::CodeKey { rung: Doc, file: src/tomli/_parser.py, decl: 54, sub: 0, line: 764 } |  |  | 0.543 |
| ns | 5348 |  | 251 | Flags: the two flag constants, the state fields, and the complete method roster | 4.1 | 2.2 | 0.546 |
| walker |  | 5392 | 110 | Code::CodeKey { rung: Doc, file: src/tomli/_parser.py, decl: 16, sub: 0, line: 76 } |  |  | 0.573 |
| ns | 5472 |  | 124 | NestedDict: the parsed-document container and its two methods | 4.2 | 2.2 | 0.573 |
| walker |  | 5634 | 242 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.573 |
| ns | 5733 |  | 261 | create_dict_rule(): the [table] statement | 4.3 | 2.1 | 0.560 |
| walker |  | 5810 | 176 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.560 |
| ns | 6033 |  | 300 | create_list_rule(): the [[array of tables]] statement | 4.4 | 2.1 | 0.546 |
| walker |  | 6151 | 341 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.546 |
| walker |  | 6325 | 174 | Markdown::Section { file: tomllib.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.546 |
| walker |  | 6395 | 70 | Code::CodeKey { rung: Body, file: src/tomli/_re.py, decl: 7, sub: 0, line: 98 } |  |  | 0.547 |
| ns | 6511 |  | 478 | key_value_rule(): dotted keys, pending flags and immutability | 4.5 | 2.1 | 0.527 |
| ns | 6690 |  | 179 | parse_key_part(): bare, literal and basic key forms | 4.6 | 2.1 | 0.519 |
| ns | 6975 |  | 285 | parse_array(): array literals and trailing commas | 4.7 | 2.1 | 0.507 |
| ns | 7464 |  | 489 | parse_inline_table(): inline tables and their local flag scope | 4.8 | 2.1 | 0.490 |
| ns | 7665 |  | 201 | parse_hex_char() and is_unicode_scalar_value(): escape validation | 4.9 | 2.1 | 0.484 |
| ns | 7807 |  | 142 | parse_literal_str() and parse_one_line_basic_str(): the two short string entry points | 4.10 | 2.1 | 0.481 |
| ns | 8041 |  | 234 | make_safe_parse_float(): the parse_float contract | 4.11 | 2.1 | 0.477 |
| ns | 8133 |  | 92 | TOMLDecodeError.__init__ signature and the deprecated free-form form | 5.1 | 1.8 | 0.483 |
| walker |  | 8205 | 1810 | Toml::Config { file: pyproject.toml } |  |  | 0.506 |
| ns | 8360 |  | 227 | TOMLDecodeError: line/column computation and message formatting | 5.2 | 5.1 | 0.498 |
| ns | 8535 |  | 175 | Complete roster of src/tomli/_re.py: four regexes and four functions | 5.3 |  | 0.508 |
| walker |  | 8597 | 392 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.508 |
| ns | 8772 |  | 237 | RE_NUMBER: the integer and float grammar | 5.4 | 5.3 | 0.520 |
| walker |  | 8781 | 184 | Markdown::Section { file: README.md, section_index: 10, keeps_default_concavity: false } |  |  | 0.520 |
| ns | 8901 |  | 129 | match_to_datetime docstring and the cached_tz cache-size note | 5.5 | 5.3 | 0.520 |
| ns | 9133 |  | 232 | Complete roster of tox environments with their descriptions | 6.1 | 3.4 | 0.527 |
| walker |  | 9297 | 516 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.527 |
| ns | 9339 |  | 206 | tomllib.md: section map and the CPython sync procedure | 6.2 |  | 0.526 |
| ns | 9473 |  | 134 | CHANGELOG.md: the two most recent releases | 6.3 |  | 0.520 |
| ns | 9638 |  | 165 | CI workflow: the complete job list and test matrix | 6.4 |  | 0.516 |
| walker |  | 9640 | 343 | Code::CodeKey { rung: Body, file: fuzzer/fuzz.py, decl: 1, sub: 0, line: 20 } |  |  | 0.516 |
| walker |  | 9648 | 8 | Plaintext::Whole { file: scripts/requirements.txt } |  |  | 0.516 |
| walker |  | 9731 | 83 | Code::CodeKey { rung: Body, file: src/tomli/_re.py, decl: 8, sub: 0, line: 109 } |  |  | 0.517 |
| walker |  | 9744 | 13 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 22, sub: 0, line: 233 } |  |  | 0.518 |
| walker |  | 9759 | 15 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 47, sub: 0, line: 595 } |  |  | 0.518 |
| ns | 9809 |  | 171 | pre-commit: the complete list of hook ids | 6.5 |  | 0.513 |
| ns | 9994 |  | 185 | Lint and version-bump configuration: .flake8 and .bumpversion.cfg | 6.6 |  | 0.507 |
