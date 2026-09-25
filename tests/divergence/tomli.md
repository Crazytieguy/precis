Score(3000)=0.422 I=0.758 C=0.235 ns_rows≤3K=18/54 grid(1000/1442/2080/3000/4327/6240/9000)=0.575/0.468/0.515/0.422/0.499/0.509/0.490

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 29 |  | 29 | README title and tagline | 1.1 |  | 0.000 |
| walker |  | 62 | 62 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 66 | 4 | Fs::DirListing { dir: src } |  |  | 0.000 |
| walker |  | 69 | 3 | Fs::DirListing { dir: .github } |  |  | 0.000 |
| walker |  | 73 | 4 | Fs::DirListing { dir: .github/workflows } |  |  | 0.000 |
| ns | 91 |  | 62 | Root directory listing (complete) | 1.2 |  | 0.660 |
| walker |  | 99 | 26 | Fs::DirListing { dir: src/tomli } |  |  | 0.691 |
| walker |  | 138 | 39 | Code::CodeKey { rung: ModuleDoc, file: src/tomli/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.695 |
| ns | 172 |  | 81 | The complete public API: src/tomli/__init__.py | 1.3 |  | 0.633 |
| walker |  | 175 | 37 | Code::CodeKey { rung: Names, file: src/tomli/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.689 |
| walker |  | 183 | 8 | Fs::DirListing { dir: fuzzer } |  |  | 0.690 |
| walker |  | 192 | 9 | Fs::DirListing { dir: profiler } |  |  | 0.690 |
| ns | 270 |  | 98 | README intro lede: a TOML parser, TOML v1.1.0 as of 2.4.0 | 1.4 |  | 0.644 |
| walker |  | 297 | 105 | Toml::Identity { file: pyproject.toml } |  |  | 0.646 |
| ns | 300 |  | 30 | Source package listing: src/ and src/tomli/ | 1.5 |  | 0.648 |
| walker |  | 314 | 17 | Fs::DirListing { dir: scripts } |  |  | 0.649 |
| walker |  | 372 | 58 | Markdown::HeadingsOutline { file: tomllib.md } |  |  | 0.650 |
| walker |  | 392 | 20 | Fs::DirListing { dir: benchmark } |  |  | 0.653 |
| ns | 408 |  | 108 | README: the tomllib stdlib relationship | 1.6 |  | 0.592 |
| walker |  | 426 | 34 | Code::CodeKey { rung: Names, file: src/tomli/_types.py, decl: 0, sub: 0, line: 0 } |  |  | 0.596 |
| walker |  | 456 | 30 | Fs::DirListing { dir: tests } |  |  | 0.598 |
| ns | 496 |  | 88 | Entry points: load() and loads() signatures with docstrings | 1.7 |  | 0.564 |
| walker |  | 516 | 60 | Toml::PackageMetadata { file: pyproject.toml } |  |  | 0.566 |
| ns | 622 |  | 126 | TOMLDecodeError class docstring and its attributes | 1.8 |  | 0.501 |
| ns | 682 |  | 60 | src/tomli/_types.py type aliases (complete file body) | 1.9 |  | 0.483 |
| ns | 766 |  | 84 | README mypyc/pure-Python distribution note | 1.10 |  | 0.465 |
| walker |  | 909 | 393 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.657 |
| ns | 972 |  | 206 | README section map: every H2 and H3 heading | 1.11 |  | 0.575 |
| walker |  | 1150 | 241 | Code::CodeKey { rung: Names, file: src/tomli/_parser.py, decl: 0, sub: 0, line: 0 } |  |  | 0.577 |
| ns | 1220 |  | 248 | Complete roster of module-level functions in _parser.py (names only) | 2.1 |  | 0.500 |
| ns | 1340 |  | 120 | Complete roster of classes in _parser.py, with Output's fields | 2.2 |  | 0.468 |
| walker |  | 1535 | 385 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.587 |
| walker |  | 1544 | 9 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.587 |
| walker |  | 1569 | 25 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.587 |
| ns | 1576 |  | 236 | loads(): parse state setup and the statement-loop rule enumeration | 2.3 | 1.7 | 0.536 |
| ns | 1719 |  | 143 | parse_value() signature and the inline-nesting recursion guard | 2.4 | 2.1 | 0.513 |
| walker |  | 1797 | 228 | Code::CodeKey { rung: Names, file: src/tomli/_parser.py, decl: 0, sub: 1, line: 0 } |  |  | 0.529 |
| walker |  | 1808 | 11 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 16, sub: 0, line: 76 } |  |  | 0.529 |
| walker |  | 1822 | 14 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 31, sub: 0, line: 312 } |  |  | 0.533 |
| walker |  | 1845 | 23 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 11, sub: 0, line: 51 } |  |  | 0.533 |
| walker |  | 1895 | 50 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 27, sub: 0, line: 278 } |  |  | 0.535 |
| ns | 1924 |  | 205 | MAX_INLINE_NESTING and the mypyc stack-overflow rationale | 2.5 | 2.4 | 0.515 |
| walker |  | 1941 | 46 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 29, sub: 0, line: 283 } |  |  | 0.515 |
| walker |  | 2097 | 156 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 14, sub: 0, line: 57 } |  |  | 0.518 |
| walker |  | 2180 | 83 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 17, sub: 0, line: 87 } |  |  | 0.518 |
| ns | 2357 |  | 433 | loads(): the statement dispatch body and its two top-level errors | 2.6 | 2.3 | 0.454 |
| walker |  | 2395 | 215 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 20, sub: 0, line: 220 } |  |  | 0.457 |
| walker |  | 2570 | 175 | Code::CodeKey { rung: Names, file: src/tomli/_re.py, decl: 0, sub: 0, line: 0 } |  |  | 0.457 |
| walker |  | 2582 | 12 | Code::CodeKey { rung: Decl, file: src/tomli/_re.py, decl: 7, sub: 0, line: 98 } |  |  | 0.458 |
| ns | 2734 |  | 377 | parse_value(): string, boolean, array and inline-table dispatch | 2.7 | 2.4 | 0.416 |
| walker |  | 2741 | 159 | Code::CodeKey { rung: Decl, file: src/tomli/_re.py, decl: 5, sub: 0, line: 46 } |  |  | 0.416 |
| walker |  | 2758 | 17 | Code::CodeKey { rung: Names, file: tests/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.416 |
| walker |  | 2771 | 13 | Code::CodeKey { rung: Doc, file: src/tomli/_parser.py, decl: 19, sub: 0, line: 149 } |  |  | 0.422 |
| walker |  | 2805 | 34 | Code::CodeKey { rung: Body, file: src/tomli/_re.py, decl: 9, sub: 0, line: 116 } |  |  | 0.422 |
| walker |  | 3042 | 237 | Code::CodeKey { rung: Decl, file: src/tomli/_re.py, decl: 3, sub: 0, line: 26 } |  |  | 0.423 |
| ns | 3122 |  | 388 | parse_value(): datetime, number and special-float dispatch | 2.8 | 2.4 | 0.395 |
| walker |  | 3252 | 210 | Code::CodeKey { rung: Names, file: src/tomli/_parser.py, decl: 0, sub: 2, line: 0 } |  |  | 0.417 |
| walker |  | 3287 | 35 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 39, sub: 0, line: 413 } |  |  | 0.417 |
| walker |  | 3325 | 38 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 40, sub: 0, line: 447 } |  |  | 0.417 |
| walker |  | 3391 | 66 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 34, sub: 0, line: 327 } |  |  | 0.417 |
| ns | 3453 |  | 331 | Character-class constants: the complete set | 2.9 |  | 0.443 |
| ns | 3550 |  | 97 | load() body: the binary-mode requirement | 2.10 | 1.7 | 0.436 |
| walker |  | 3566 | 175 | Markdown::Section { file: tomllib.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.436 |
| walker |  | 3575 | 9 | Markdown::ReadmeHeadline { file: benchmark/README.md } |  |  | 0.436 |
| walker |  | 3581 | 6 | Fs::DirListing { dir: tests/data } |  |  | 0.437 |
| ns | 3662 |  | 112 | loads() prologue: CRLF normalisation and the str type check | 2.11 | 1.7 | 0.430 |
| walker |  | 3685 | 104 | Code::CodeKey { rung: Decl, file: src/tomli/_re.py, decl: 2, sub: 0, line: 17 } |  |  | 0.430 |
| ns | 3836 |  | 174 | BASIC_STR_ESCAPE_REPLACEMENTS: the full escape table | 2.12 |  | 0.453 |
| ns | 3872 |  | 36 | tests/ and tests/data/ listings (complete) | 3.1 |  | 0.467 |
| walker |  | 3912 | 227 | Code::CodeKey { rung: Names, file: src/tomli/_parser.py, decl: 0, sub: 3, line: 0 } |  |  | 0.518 |
| ns | 3931 |  | 59 | tests/__init__.py: the tomli-as-tomllib alias | 3.2 |  | 0.514 |
| walker |  | 3944 | 32 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 46, sub: 0, line: 564 } |  |  | 0.514 |
| walker |  | 3980 | 36 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 52, sub: 0, line: 684 } |  |  | 0.516 |
| walker |  | 4017 | 37 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 44, sub: 0, line: 502 } |  |  | 0.516 |
| walker |  | 4056 | 39 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 45, sub: 0, line: 528 } |  |  | 0.516 |
| ns | 4195 |  | 264 | Complete roster of test classes and test methods | 3.3 |  | 0.499 |
| walker |  | 4244 | 188 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.499 |
| ns | 4371 |  | 176 | tox core configuration: the interpreter matrix and default command | 3.4 |  | 0.489 |
| walker |  | 4374 | 130 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: true } |  |  | 0.489 |
| ns | 4523 |  | 152 | setup.py: the mypyc build path | 3.5 |  | 0.478 |
| walker |  | 4675 | 301 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.522 |
| ns | 4717 |  | 194 | pyproject.toml: build backend and project metadata | 3.6 |  | 0.527 |
| ns | 4775 |  | 58 | Listings of the helper trees: benchmark, fuzzer, profiler, scripts, workflows | 3.7 |  | 0.542 |
| walker |  | 4917 | 242 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.542 |
| ns | 4958 |  | 183 | test_data.py: how the TOML corpus is discovered and compared | 3.8 | 3.3 | 0.531 |
| walker |  | 5093 | 176 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: true } |  |  | 0.531 |
| ns | 5097 |  | 139 | tests/burntsushi.py: purpose and complete function roster | 3.9 |  | 0.524 |
| walker |  | 5107 | 14 | Code::CodeKey { rung: Names, file: profiler/profiler_script.py, decl: 0, sub: 0, line: 0 } |  |  | 0.524 |
| walker |  | 5122 | 15 | Code::CodeKey { rung: Names, file: tests/test_error.py, decl: 0, sub: 0, line: 0 } |  |  | 0.525 |
| walker |  | 5184 | 62 | Code::CodeKey { rung: Doc, file: src/tomli/_re.py, decl: 6, sub: 0, line: 59 } |  |  | 0.525 |
| walker |  | 5200 | 16 | Code::CodeKey { rung: Names, file: scripts/use_setuptools.py, decl: 0, sub: 0, line: 0 } |  |  | 0.525 |
| walker |  | 5216 | 16 | Code::CodeKey { rung: Names, file: tests/test_misc.py, decl: 0, sub: 0, line: 0 } |  |  | 0.525 |
| ns | 5348 |  | 251 | Flags: the two flag constants, the state fields, and the complete method roster | 4.1 | 2.2 | 0.529 |
| ns | 5472 |  | 124 | NestedDict: the parsed-document container and its two methods | 4.2 | 2.2 | 0.530 |
| walker |  | 5557 | 341 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.530 |
| walker |  | 5731 | 174 | Markdown::Section { file: tomllib.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.530 |
| ns | 5733 |  | 261 | create_dict_rule(): the [table] statement | 4.3 | 2.1 | 0.518 |
| ns | 6033 |  | 300 | create_list_rule(): the [[array of tables]] statement | 4.4 | 2.1 | 0.505 |
| walker |  | 6084 | 353 | Markdown::Section { file: tomllib.md, section_index: 2, keeps_default_concavity: true } |  |  | 0.506 |
| walker |  | 6098 | 14 | Code::CodeKey { rung: Doc, file: src/tomli/_parser.py, decl: 20, sub: 0, line: 220 } |  |  | 0.509 |
| walker |  | 6168 | 70 | Code::CodeKey { rung: Body, file: src/tomli/_re.py, decl: 7, sub: 0, line: 98 } |  |  | 0.509 |
| ns | 6511 |  | 478 | key_value_rule(): dotted keys, pending flags and immutability | 4.5 | 2.1 | 0.491 |
| ns | 6690 |  | 179 | parse_key_part(): bare, literal and basic key forms | 4.6 | 2.1 | 0.484 |
| ns | 6975 |  | 285 | parse_array(): array literals and trailing commas | 4.7 | 2.1 | 0.472 |
| ns | 7464 |  | 489 | parse_inline_table(): inline tables and their local flag scope | 4.8 | 2.1 | 0.457 |
| ns | 7665 |  | 201 | parse_hex_char() and is_unicode_scalar_value(): escape validation | 4.9 | 2.1 | 0.451 |
| ns | 7807 |  | 142 | parse_literal_str() and parse_one_line_basic_str(): the two short string entry points | 4.10 | 2.1 | 0.448 |
| walker |  | 7978 | 1810 | Toml::Config { file: pyproject.toml } |  |  | 0.473 |
| walker |  | 8005 | 27 | Code::CodeKey { rung: Names, file: benchmark/run.py, decl: 0, sub: 0, line: 0 } |  |  | 0.473 |
| walker |  | 8020 | 15 | Code::CodeKey { rung: Doc, file: src/tomli/_parser.py, decl: 18, sub: 0, line: 137 } |  |  | 0.476 |
| ns | 8041 |  | 234 | make_safe_parse_float(): the parse_float contract | 4.11 | 2.1 | 0.468 |
| ns | 8133 |  | 92 | TOMLDecodeError.__init__ signature and the deprecated free-form form | 5.1 | 1.8 | 0.474 |
| ns | 8360 |  | 227 | TOMLDecodeError: line/column computation and message formatting | 5.2 | 5.1 | 0.466 |
| walker |  | 8412 | 392 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.466 |
| ns | 8535 |  | 175 | Complete roster of src/tomli/_re.py: four regexes and four functions | 5.3 |  | 0.477 |
| walker |  | 8596 | 184 | Markdown::Section { file: README.md, section_index: 10, keeps_default_concavity: false } |  |  | 0.477 |
| ns | 8772 |  | 237 | RE_NUMBER: the integer and float grammar | 5.4 | 5.3 | 0.490 |
| ns | 8901 |  | 129 | match_to_datetime docstring and the cached_tz cache-size note | 5.5 | 5.3 | 0.490 |
| walker |  | 9112 | 516 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.490 |
| walker |  | 9127 | 15 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 47, sub: 0, line: 595 } |  |  | 0.490 |
| ns | 9133 |  | 232 | Complete roster of tox environments with their descriptions | 6.1 | 3.4 | 0.498 |
| walker |  | 9159 | 32 | Code::CodeKey { rung: Decl, file: profiler/profiler_script.py, decl: 1, sub: 0, line: 12 } |  |  | 0.498 |
| walker |  | 9167 | 8 | Plaintext::Whole { file: scripts/requirements.txt } |  |  | 0.498 |
| walker |  | 9250 | 83 | Code::CodeKey { rung: Body, file: src/tomli/_re.py, decl: 8, sub: 0, line: 109 } |  |  | 0.499 |
| walker |  | 9259 | 9 | Plaintext::Whole { file: profiler/requirements.txt } |  |  | 0.499 |
| walker |  | 9283 | 24 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 43, sub: 0, line: 497 } |  |  | 0.500 |
| ns | 9339 |  | 206 | tomllib.md: section map and the CPython sync procedure | 6.2 |  | 0.507 |
| walker |  | 9345 | 62 | Code::CodeKey { rung: Names, file: fuzzer/fuzz.py, decl: 0, sub: 0, line: 0 } |  |  | 0.507 |
| walker |  | 9405 | 60 | Code::CodeKey { rung: Decl, file: benchmark/run.py, decl: 1, sub: 0, line: 15 } |  |  | 0.507 |
| walker |  | 9418 | 13 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 22, sub: 0, line: 233 } |  |  | 0.508 |
| walker |  | 9469 | 51 | Markdown::HeadingsOutline { file: benchmark/README.md } |  |  | 0.508 |
| ns | 9473 |  | 134 | CHANGELOG.md: the two most recent releases | 6.3 |  | 0.503 |
| walker |  | 9496 | 27 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 53, sub: 0, line: 760 } |  |  | 0.503 |
| walker |  | 9582 | 86 | Code::CodeKey { rung: Names, file: tests/burntsushi.py, decl: 0, sub: 0, line: 0 } |  |  | 0.506 |
| ns | 9638 |  | 165 | CI workflow: the complete job list and test matrix | 6.4 |  | 0.502 |
| walker |  | 9678 | 96 | Code::CodeKey { rung: Decl, file: tests/test_misc.py, decl: 1, sub: 0, line: 17 } |  |  | 0.507 |
| walker |  | 9710 | 32 | Code::CodeKey { rung: Doc, file: src/tomli/_parser.py, decl: 15, sub: 0, line: 71 } |  |  | 0.513 |
| ns | 9809 |  | 171 | pre-commit: the complete list of hook ids | 6.5 |  | 0.509 |
| walker |  | 9810 | 100 | Code::CodeKey { rung: Decl, file: tests/test_error.py, decl: 1, sub: 0, line: 13 } |  |  | 0.519 |
| walker |  | 9828 | 18 | Code::CodeKey { rung: Doc, file: fuzzer/fuzz.py, decl: 3, sub: 0, line: 59 } |  |  | 0.519 |
| walker |  | 9942 | 114 | Code::CodeKey { rung: Names, file: tests/test_data.py, decl: 0, sub: 0, line: 0 } |  |  | 0.525 |
| walker |  | 9964 | 22 | Code::CodeKey { rung: Decl, file: tests/test_data.py, decl: 6, sub: 0, line: 27 } |  |  | 0.528 |
| walker |  | 9983 | 19 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 32, sub: 0, line: 313 } |  |  | 0.534 |
| ns | 9994 |  | 185 | Lint and version-bump configuration: .flake8 and .bumpversion.cfg | 6.6 |  | 0.528 |
