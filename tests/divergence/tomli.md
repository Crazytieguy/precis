Score(3000)=0.458 I=0.774 C=0.271 ns_rows≤3K=18/54 grid(1000/1442/2080/3000/4327/6240/9000)=0.582/0.574/0.485/0.458/0.446/0.538/0.514

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
| ns | 1576 |  | 236 | loads(): parse state setup and the statement-loop rule enumeration | 2.3 | 1.7 | 0.524 |
| walker |  | 1671 | 175 | Markdown::Section { file: tomllib.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.524 |
| walker |  | 1715 | 44 | Plaintext::DeclSurface { file: fuzzer/requirements.txt } |  |  | 0.524 |
| ns | 1719 |  | 143 | parse_value() signature and the inline-nesting recursion guard | 2.4 | 2.1 | 0.502 |
| walker |  | 1903 | 188 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.502 |
| ns | 1924 |  | 205 | MAX_INLINE_NESTING and the mypyc stack-overflow rationale | 2.5 | 2.4 | 0.483 |
| walker |  | 2033 | 130 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.483 |
| walker |  | 2334 | 301 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.566 |
| ns | 2357 |  | 433 | loads(): the statement dispatch body and its two top-level errors | 2.6 | 2.3 | 0.495 |
| walker |  | 2368 | 34 | Code::CodeKey { rung: Names, file: src/tomli/_types.py, decl: 0, sub: 0, line: 0 } |  |  | 0.503 |
| walker |  | 2474 | 106 | Code::CodeKey { rung: Body, file: fuzzer/fuzz.py, decl: 3, sub: 0, line: 59 } |  |  | 0.503 |
| walker |  | 2716 | 242 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.503 |
| ns | 2734 |  | 377 | parse_value(): string, boolean, array and inline-table dispatch | 2.7 | 2.4 | 0.458 |
| walker |  | 2892 | 176 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.458 |
| walker |  | 3067 | 175 | Code::CodeKey { rung: Names, file: src/tomli/_re.py, decl: 0, sub: 0, line: 0 } |  |  | 0.458 |
| walker |  | 3079 | 12 | Code::CodeKey { rung: Decl, file: src/tomli/_re.py, decl: 7, sub: 0, line: 98 } |  |  | 0.458 |
| ns | 3122 |  | 388 | parse_value(): datetime, number and special-float dispatch | 2.8 | 2.4 | 0.428 |
| walker |  | 3183 | 104 | Code::CodeKey { rung: Decl, file: src/tomli/_re.py, decl: 2, sub: 0, line: 17 } |  |  | 0.428 |
| walker |  | 3342 | 159 | Code::CodeKey { rung: Decl, file: src/tomli/_re.py, decl: 5, sub: 0, line: 46 } |  |  | 0.429 |
| walker |  | 3376 | 34 | Code::CodeKey { rung: Body, file: src/tomli/_re.py, decl: 9, sub: 0, line: 116 } |  |  | 0.429 |
| ns | 3453 |  | 331 | Character-class constants: the complete set | 2.9 |  | 0.411 |
| ns | 3550 |  | 97 | load() body: the binary-mode requirement | 2.10 | 1.7 | 0.404 |
| walker |  | 3613 | 237 | Code::CodeKey { rung: Decl, file: src/tomli/_re.py, decl: 3, sub: 0, line: 26 } |  |  | 0.405 |
| ns | 3662 |  | 112 | loads() prologue: CRLF normalisation and the str type check | 2.11 | 1.7 | 0.399 |
| walker |  | 3675 | 62 | Code::CodeKey { rung: Doc, file: src/tomli/_re.py, decl: 6, sub: 0, line: 59 } |  |  | 0.399 |
| ns | 3836 |  | 174 | BASIC_STR_ESCAPE_REPLACEMENTS: the full escape table | 2.12 |  | 0.390 |
| ns | 3872 |  | 36 | tests/ and tests/data/ listings (complete) | 3.1 |  | 0.408 |
| walker |  | 3916 | 241 | Code::CodeKey { rung: Names, file: src/tomli/_parser.py, decl: 0, sub: 0, line: 0 } |  |  | 0.428 |
| ns | 3931 |  | 59 | tests/__init__.py: the tomli-as-tomllib alias | 3.2 |  | 0.424 |
| walker |  | 4144 | 228 | Code::CodeKey { rung: Names, file: src/tomli/_parser.py, decl: 0, sub: 1, line: 0 } |  |  | 0.445 |
| walker |  | 4155 | 11 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 16, sub: 0, line: 76 } |  |  | 0.445 |
| walker |  | 4169 | 14 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 31, sub: 0, line: 312 } |  |  | 0.447 |
| walker |  | 4192 | 23 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 11, sub: 0, line: 51 } |  |  | 0.455 |
| ns | 4195 |  | 264 | Complete roster of test classes and test methods | 3.3 |  | 0.440 |
| walker |  | 4242 | 50 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 27, sub: 0, line: 278 } |  |  | 0.441 |
| walker |  | 4288 | 46 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 29, sub: 0, line: 283 } |  |  | 0.442 |
| walker |  | 4301 | 13 | Code::CodeKey { rung: Doc, file: src/tomli/_parser.py, decl: 19, sub: 0, line: 149 } |  |  | 0.446 |
| ns | 4371 |  | 176 | tox core configuration: the interpreter matrix and default command | 3.4 |  | 0.437 |
| walker |  | 4384 | 83 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 17, sub: 0, line: 87 } |  |  | 0.438 |
| walker |  | 4399 | 15 | Code::CodeKey { rung: Doc, file: src/tomli/_parser.py, decl: 18, sub: 0, line: 137 } |  |  | 0.443 |
| ns | 4523 |  | 152 | setup.py: the mypyc build path | 3.5 |  | 0.433 |
| walker |  | 4555 | 156 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 14, sub: 0, line: 57 } |  |  | 0.462 |
| walker |  | 4587 | 32 | Code::CodeKey { rung: Doc, file: src/tomli/_parser.py, decl: 15, sub: 0, line: 71 } |  |  | 0.472 |
| ns | 4717 |  | 194 | pyproject.toml: build backend and project metadata | 3.6 |  | 0.468 |
| ns | 4775 |  | 58 | Listings of the helper trees: benchmark, fuzzer, profiler, scripts, workflows | 3.7 |  | 0.486 |
| walker |  | 4802 | 215 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 20, sub: 0, line: 220 } |  |  | 0.489 |
| walker |  | 4816 | 14 | Code::CodeKey { rung: Doc, file: src/tomli/_parser.py, decl: 20, sub: 0, line: 220 } |  |  | 0.494 |
| ns | 4958 |  | 183 | test_data.py: how the TOML corpus is discovered and compared | 3.8 | 3.3 | 0.484 |
| walker |  | 5026 | 210 | Code::CodeKey { rung: Names, file: src/tomli/_parser.py, decl: 0, sub: 2, line: 0 } |  |  | 0.497 |
| walker |  | 5061 | 35 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 39, sub: 0, line: 413 } |  |  | 0.497 |
| ns | 5097 |  | 139 | tests/burntsushi.py: purpose and complete function roster | 3.9 |  | 0.491 |
| walker |  | 5099 | 38 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 40, sub: 0, line: 447 } |  |  | 0.491 |
| walker |  | 5165 | 66 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 34, sub: 0, line: 327 } |  |  | 0.491 |
| ns | 5348 |  | 251 | Flags: the two flag constants, the state fields, and the complete method roster | 4.1 | 2.2 | 0.496 |
| walker |  | 5392 | 227 | Code::CodeKey { rung: Names, file: src/tomli/_parser.py, decl: 0, sub: 3, line: 0 } |  |  | 0.535 |
| walker |  | 5424 | 32 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 46, sub: 0, line: 564 } |  |  | 0.535 |
| walker |  | 5460 | 36 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 52, sub: 0, line: 684 } |  |  | 0.536 |
| ns | 5472 |  | 124 | NestedDict: the parsed-document container and its two methods | 4.2 | 2.2 | 0.537 |
| walker |  | 5497 | 37 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 44, sub: 0, line: 502 } |  |  | 0.537 |
| walker |  | 5536 | 39 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 45, sub: 0, line: 528 } |  |  | 0.537 |
| walker |  | 5631 | 95 | Code::CodeKey { rung: Doc, file: src/tomli/_parser.py, decl: 54, sub: 0, line: 764 } |  |  | 0.538 |
| ns | 5733 |  | 261 | create_dict_rule(): the [table] statement | 4.3 | 2.1 | 0.526 |
| walker |  | 5741 | 110 | Code::CodeKey { rung: Doc, file: src/tomli/_parser.py, decl: 16, sub: 0, line: 76 } |  |  | 0.551 |
| ns | 6033 |  | 300 | create_list_rule(): the [[array of tables]] statement | 4.4 | 2.1 | 0.538 |
| walker |  | 6082 | 341 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.538 |
| walker |  | 6474 | 392 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.538 |
| ns | 6511 |  | 478 | key_value_rule(): dotted keys, pending flags and immutability | 4.5 | 2.1 | 0.518 |
| walker |  | 6658 | 184 | Markdown::Section { file: README.md, section_index: 10, keeps_default_concavity: false } |  |  | 0.518 |
| ns | 6690 |  | 179 | parse_key_part(): bare, literal and basic key forms | 4.6 | 2.1 | 0.511 |
| ns | 6975 |  | 285 | parse_array(): array literals and trailing commas | 4.7 | 2.1 | 0.498 |
| walker |  | 7174 | 516 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.498 |
| walker |  | 7348 | 174 | Markdown::Section { file: tomllib.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.499 |
| walker |  | 7418 | 70 | Code::CodeKey { rung: Body, file: src/tomli/_re.py, decl: 7, sub: 0, line: 98 } |  |  | 0.499 |
| ns | 7464 |  | 489 | parse_inline_table(): inline tables and their local flag scope | 4.8 | 2.1 | 0.482 |
| ns | 7665 |  | 201 | parse_hex_char() and is_unicode_scalar_value(): escape validation | 4.9 | 2.1 | 0.477 |
| walker |  | 7761 | 343 | Code::CodeKey { rung: Body, file: fuzzer/fuzz.py, decl: 1, sub: 0, line: 20 } |  |  | 0.477 |
| ns | 7807 |  | 142 | parse_literal_str() and parse_one_line_basic_str(): the two short string entry points | 4.10 | 2.1 | 0.473 |
| walker |  | 7844 | 83 | Code::CodeKey { rung: Body, file: src/tomli/_re.py, decl: 8, sub: 0, line: 109 } |  |  | 0.473 |
| walker |  | 7857 | 13 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 22, sub: 0, line: 233 } |  |  | 0.474 |
| walker |  | 7872 | 15 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 47, sub: 0, line: 595 } |  |  | 0.474 |
| ns | 8041 |  | 234 | make_safe_parse_float(): the parse_float contract | 4.11 | 2.1 | 0.471 |
| ns | 8133 |  | 92 | TOMLDecodeError.__init__ signature and the deprecated free-form form | 5.1 | 1.8 | 0.477 |
| walker |  | 8225 | 353 | Markdown::Section { file: tomllib.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.477 |
| walker |  | 8244 | 19 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 32, sub: 0, line: 313 } |  |  | 0.484 |
| walker |  | 8252 | 8 | Plaintext::DeclSurface { file: scripts/requirements.txt } |  |  | 0.484 |
| walker |  | 8276 | 24 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 43, sub: 0, line: 497 } |  |  | 0.485 |
| walker |  | 8341 | 65 | Markdown::HeadingsOutline { file: benchmark/README.md } |  |  | 0.485 |
| ns | 8360 |  | 227 | TOMLDecodeError: line/column computation and message formatting | 5.2 | 5.1 | 0.477 |
| walker |  | 8368 | 27 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 53, sub: 0, line: 760 } |  |  | 0.478 |
| walker |  | 8469 | 101 | Fs::DirListing { dir: tests/data/valid } |  |  | 0.478 |
| walker |  | 8474 | 5 | Fs::DirListing { dir: tests/data/valid/_external } |  |  | 0.478 |
| walker |  | 8480 | 6 | Fs::DirListing { dir: tests/data/valid/_external/toml-test } |  |  | 0.478 |
| walker |  | 8502 | 22 | Fs::DirListing { dir: tests/data/valid/dates-and-times } |  |  | 0.478 |
| walker |  | 8528 | 26 | Fs::DirListing { dir: tests/data/valid/array } |  |  | 0.478 |
| ns | 8535 |  | 175 | Complete roster of src/tomli/_re.py: four regexes and four functions | 5.3 |  | 0.491 |
| walker |  | 8554 | 26 | Fs::DirListing { dir: tests/data/valid/inline-table } |  |  | 0.491 |
| walker |  | 8582 | 28 | Fs::DirListing { dir: tests/data/valid/multiline-basic-str } |  |  | 0.491 |
| walker |  | 8610 | 28 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 28, sub: 0, line: 279 } |  |  | 0.496 |
| walker |  | 8648 | 38 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 21, sub: 0, line: 229 } |  |  | 0.502 |
| ns | 8772 |  | 237 | RE_NUMBER: the integer and float grammar | 5.4 | 5.3 | 0.514 |
| ns | 8901 |  | 129 | match_to_datetime docstring and the cached_tz cache-size note | 5.5 | 5.3 | 0.514 |
| walker |  | 8985 | 337 | Code::CodeKey { rung: Body, file: src/tomli/_re.py, decl: 6, sub: 0, line: 59 } |  |  | 0.514 |
| ns | 9133 |  | 232 | Complete roster of tox environments with their descriptions | 6.1 | 3.4 | 0.509 |
| walker |  | 9139 | 154 | Fs::DirListing { dir: tests/data/invalid } |  |  | 0.509 |
| walker |  | 9144 | 5 | Fs::DirListing { dir: tests/data/invalid/_external } |  |  | 0.509 |
| walker |  | 9150 | 6 | Fs::DirListing { dir: tests/data/invalid/dates-and-times } |  |  | 0.509 |
| walker |  | 9156 | 6 | Fs::DirListing { dir: tests/data/invalid/literal-str } |  |  | 0.509 |
| walker |  | 9162 | 6 | Fs::DirListing { dir: tests/data/invalid/_external/toml-test } |  |  | 0.509 |
| walker |  | 9177 | 15 | Fs::DirListing { dir: tests/data/invalid/multiline-literal-str } |  |  | 0.509 |
| walker |  | 9195 | 18 | Fs::DirListing { dir: tests/data/invalid/array-of-tables } |  |  | 0.509 |
| walker |  | 9213 | 18 | Fs::DirListing { dir: tests/data/invalid/boolean } |  |  | 0.509 |
| walker |  | 9234 | 21 | Fs::DirListing { dir: tests/data/invalid/table } |  |  | 0.509 |
| walker |  | 9257 | 23 | Fs::DirListing { dir: tests/data/invalid/array } |  |  | 0.509 |
| walker |  | 9290 | 33 | Fs::DirListing { dir: tests/data/invalid/dotted-keys } |  |  | 0.509 |
| walker |  | 9330 | 40 | Fs::DirListing { dir: tests/data/invalid/keys-and-vals } |  |  | 0.509 |
| ns | 9339 |  | 206 | tomllib.md: section map and the CPython sync procedure | 6.2 |  | 0.516 |
| walker |  | 9372 | 42 | Fs::DirListing { dir: tests/data/invalid/multiline-basic-str } |  |  | 0.516 |
| walker |  | 9461 | 89 | Fs::DirListing { dir: tests/data/invalid/inline-table } |  |  | 0.516 |
| ns | 9473 |  | 134 | CHANGELOG.md: the two most recent releases | 6.3 |  | 0.510 |
| walker |  | 9478 | 17 | Code::CodeKey { rung: Names, file: tests/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.510 |
| walker |  | 9516 | 38 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 23, sub: 0, line: 236 } |  |  | 0.511 |
| walker |  | 9572 | 56 | Fs::DirListing { dir: tests/data/invalid/_external/toml-test/invalid } |  |  | 0.511 |
| walker |  | 9623 | 51 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 33, sub: 0, line: 318 } |  |  | 0.511 |
| ns | 9638 |  | 165 | CI workflow: the complete job list and test matrix | 6.4 |  | 0.507 |
| walker |  | 9674 | 51 | Fs::DirListing { dir: tests/data/invalid/_external/toml-test/invalid/local-time } |  |  | 0.507 |
| walker |  | 9701 | 27 | Code::CodeKey { rung: Names, file: benchmark/run.py, decl: 0, sub: 0, line: 0 } |  |  | 0.507 |
| walker |  | 9761 | 60 | Code::CodeKey { rung: Decl, file: benchmark/run.py, decl: 1, sub: 0, line: 15 } |  |  | 0.507 |
| ns | 9809 |  | 171 | pre-commit: the complete list of hook ids | 6.5 |  | 0.503 |
| walker |  | 9825 | 64 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 24, sub: 0, line: 241 } |  |  | 0.504 |
| walker |  | 9840 | 15 | Code::CodeKey { rung: Names, file: tests/test_error.py, decl: 0, sub: 0, line: 0 } |  |  | 0.504 |
| walker |  | 9940 | 100 | Code::CodeKey { rung: Decl, file: tests/test_error.py, decl: 1, sub: 0, line: 13 } |  |  | 0.508 |
| walker |  | 9993 | 53 | Code::CodeKey { rung: Body, file: tests/test_error.py, decl: 4, sub: 0, line: 40 } |  |  | 0.508 |
| ns | 9994 |  | 185 | Lint and version-bump configuration: .flake8 and .bumpversion.cfg | 6.6 |  | 0.502 |
