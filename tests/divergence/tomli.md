Score(3000)=0.501 I=0.816 C=0.308 ns_rows≤3K=18/54 grid(1000/1442/2080/3000/4327/6240/9000)=0.707/0.576/0.569/0.501/0.557/0.536/0.548

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 29 |  | 29 | README title and tagline | 1.1 |  | 0.000 |
| walker |  | 62 | 62 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 91 |  | 62 | Root directory listing (complete) | 1.2 |  | 0.659 |
| walker |  | 127 | 65 | Markdown::ReadmeHeadline { file: README.md } |  |  | 1.000 |
| walker |  | 135 | 8 | Fs::DirListing { dir: fuzzer } |  |  | 1.000 |
| walker |  | 144 | 9 | Fs::DirListing { dir: profiler } |  |  | 1.000 |
| ns | 172 |  | 81 | The complete public API: src/tomli/__init__.py | 1.3 |  | 0.901 |
| walker |  | 173 | 29 | Fs::DirListing { dir: src/tomli } |  |  | 0.920 |
| walker |  | 179 | 6 | Fs::DirListing { dir: .github/workflows } |  |  | 0.920 |
| walker |  | 218 | 39 | Code::CodeKey { rung: ModuleDoc, file: src/tomli/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.930 |
| ns | 270 |  | 98 | README intro lede: a TOML parser, TOML v1.1.0 as of 2.4.0 | 1.4 |  | 0.874 |
| ns | 299 |  | 29 | Source package listing: src/ and src/tomli/ | 1.5 |  | 0.875 |
| walker |  | 323 | 105 | Toml::Identity { file: pyproject.toml } |  |  | 0.876 |
| walker |  | 340 | 17 | Fs::DirListing { dir: scripts } |  |  | 0.877 |
| walker |  | 360 | 20 | Fs::DirListing { dir: benchmark } |  |  | 0.879 |
| walker |  | 390 | 30 | Fs::DirListing { dir: tests } |  |  | 0.881 |
| walker |  | 399 | 9 | Plaintext::DeclSurface { file: profiler/requirements.txt } |  |  | 0.881 |
| ns | 407 |  | 108 | README: the tomllib stdlib relationship | 1.6 |  | 0.799 |
| walker |  | 436 | 37 | Code::CodeKey { rung: Names, file: src/tomli/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.844 |
| ns | 495 |  | 88 | Entry points: load() and loads() signatures with docstrings | 1.7 |  | 0.797 |
| ns | 621 |  | 126 | TOMLDecodeError class docstring and its attributes | 1.8 |  | 0.705 |
| ns | 681 |  | 60 | src/tomli/_types.py type aliases (complete file body) | 1.9 |  | 0.662 |
| ns | 765 |  | 84 | README mypyc/pure-Python distribution note | 1.10 |  | 0.637 |
| walker |  | 821 | 385 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.654 |
| walker |  | 857 | 36 | Markdown::CommandBlock { file: README.md, row: 135 } |  |  | 0.654 |
| walker |  | 882 | 25 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.654 |
| walker |  | 896 | 14 | Code::CodeKey { rung: Names, file: profiler/profiler_script.py, decl: 0, sub: 0, line: 0 } |  |  | 0.654 |
| walker |  | 928 | 32 | Code::CodeKey { rung: Decl, file: profiler/profiler_script.py, decl: 1, sub: 0, line: 12 } |  |  | 0.654 |
| ns | 971 |  | 206 | README section map: every H2 and H3 heading | 1.11 |  | 0.706 |
| walker |  | 990 | 62 | Code::CodeKey { rung: Names, file: fuzzer/fuzz.py, decl: 0, sub: 0, line: 0 } |  |  | 0.706 |
| walker |  | 996 | 6 | Fs::DirListing { dir: tests/data } |  |  | 0.707 |
| walker |  | 1014 | 18 | Code::CodeKey { rung: Doc, file: fuzzer/fuzz.py, decl: 3, sub: 0, line: 59 } |  |  | 0.707 |
| walker |  | 1058 | 44 | Plaintext::DeclSurface { file: fuzzer/requirements.txt } |  |  | 0.707 |
| ns | 1219 |  | 248 | Complete roster of module-level functions in _parser.py (names only) | 2.1 |  | 0.612 |
| walker |  | 1246 | 188 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.612 |
| ns | 1339 |  | 120 | Complete roster of classes in _parser.py, with Output's fields | 2.2 |  | 0.573 |
| walker |  | 1376 | 130 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.573 |
| ns | 1575 |  | 236 | loads(): parse state setup and the statement-loop rule enumeration | 2.3 | 1.7 | 0.524 |
| walker |  | 1677 | 301 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.614 |
| ns | 1718 |  | 143 | parse_value() signature and the inline-nesting recursion guard | 2.4 | 2.1 | 0.588 |
| walker |  | 1847 | 170 | Code::CodeKey { rung: Names, file: src/tomli/_parser.py, decl: 0, sub: 0, line: 0 } |  |  | 0.589 |
| ns | 1923 |  | 205 | MAX_INLINE_NESTING and the mypyc stack-overflow rationale | 2.5 | 2.4 | 0.567 |
| walker |  | 2016 | 169 | Code::CodeKey { rung: Names, file: src/tomli/_parser.py, decl: 0, sub: 1, line: 0 } |  |  | 0.569 |
| walker |  | 2039 | 23 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 11, sub: 0, line: 51 } |  |  | 0.569 |
| walker |  | 2195 | 156 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 14, sub: 0, line: 57 } |  |  | 0.572 |
| ns | 2356 |  | 433 | loads(): the statement dispatch body and its two top-level errors | 2.6 | 2.3 | 0.501 |
| walker |  | 2383 | 188 | Code::CodeKey { rung: Names, file: src/tomli/_parser.py, decl: 0, sub: 2, line: 0 } |  |  | 0.515 |
| walker |  | 2397 | 14 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 31, sub: 0, line: 312 } |  |  | 0.518 |
| walker |  | 2416 | 19 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 16, sub: 0, line: 76 } |  |  | 0.518 |
| walker |  | 2472 | 56 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 34, sub: 0, line: 327 } |  |  | 0.518 |
| walker |  | 2536 | 64 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 27, sub: 0, line: 278 } |  |  | 0.520 |
| walker |  | 2568 | 32 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 29, sub: 0, line: 283 } |  |  | 0.520 |
| walker |  | 2643 | 75 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 17, sub: 0, line: 87 } |  |  | 0.521 |
| ns | 2733 |  | 377 | parse_value(): string, boolean, array and inline-table dispatch | 2.7 | 2.4 | 0.474 |
| walker |  | 2858 | 215 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 20, sub: 0, line: 220 } |  |  | 0.476 |
| walker |  | 2871 | 13 | Code::CodeKey { rung: Doc, file: src/tomli/_parser.py, decl: 19, sub: 0, line: 149 } |  |  | 0.481 |
| walker |  | 2885 | 14 | Code::CodeKey { rung: Doc, file: src/tomli/_parser.py, decl: 20, sub: 0, line: 220 } |  |  | 0.488 |
| walker |  | 2900 | 15 | Code::CodeKey { rung: Doc, file: src/tomli/_parser.py, decl: 18, sub: 0, line: 137 } |  |  | 0.496 |
| walker |  | 3083 | 183 | Code::CodeKey { rung: Names, file: src/tomli/_parser.py, decl: 0, sub: 3, line: 0 } |  |  | 0.514 |
| walker |  | 3105 | 22 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 40, sub: 0, line: 447 } |  |  | 0.514 |
| ns | 3121 |  | 388 | parse_value(): datetime, number and special-float dispatch | 2.8 | 2.4 | 0.480 |
| walker |  | 3130 | 25 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 39, sub: 0, line: 413 } |  |  | 0.480 |
| walker |  | 3417 | 287 | Code::CodeKey { rung: Names, file: src/tomli/_parser.py, decl: 0, sub: 4, line: 0 } |  |  | 0.544 |
| walker |  | 3435 | 18 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 46, sub: 0, line: 564 } |  |  | 0.544 |
| ns | 3452 |  | 331 | Character-class constants: the complete set | 2.9 |  | 0.557 |
| walker |  | 3457 | 22 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 44, sub: 0, line: 502 } |  |  | 0.557 |
| walker |  | 3479 | 22 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 45, sub: 0, line: 528 } |  |  | 0.557 |
| walker |  | 3501 | 22 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 52, sub: 0, line: 684 } |  |  | 0.558 |
| walker |  | 3533 | 32 | Code::CodeKey { rung: Doc, file: src/tomli/_parser.py, decl: 15, sub: 0, line: 71 } |  |  | 0.573 |
| ns | 3549 |  | 97 | load() body: the binary-mode requirement | 2.10 | 1.7 | 0.563 |
| ns | 3661 |  | 112 | loads() prologue: CRLF normalisation and the str type check | 2.11 | 1.7 | 0.554 |
| walker |  | 3775 | 242 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.554 |
| ns | 3835 |  | 174 | BASIC_STR_ESCAPE_REPLACEMENTS: the full escape table | 2.12 |  | 0.571 |
| ns | 3871 |  | 36 | tests/ and tests/data/ listings (complete) | 3.1 |  | 0.581 |
| ns | 3930 |  | 59 | tests/__init__.py: the tomli-as-tomllib alias | 3.2 |  | 0.575 |
| walker |  | 3951 | 176 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.575 |
| walker |  | 4138 | 187 | Code::CodeKey { rung: Names, file: src/tomli/_re.py, decl: 0, sub: 0, line: 0 } |  |  | 0.576 |
| ns | 4194 |  | 264 | Complete roster of test classes and test methods | 3.3 |  | 0.557 |
| walker |  | 4242 | 104 | Code::CodeKey { rung: Decl, file: src/tomli/_re.py, decl: 2, sub: 0, line: 17 } |  |  | 0.557 |
| ns | 4370 |  | 176 | tox core configuration: the interpreter matrix and default command | 3.4 |  | 0.546 |
| walker |  | 4401 | 159 | Code::CodeKey { rung: Decl, file: src/tomli/_re.py, decl: 5, sub: 0, line: 46 } |  |  | 0.547 |
| ns | 4522 |  | 152 | setup.py: the mypyc build path | 3.5 |  | 0.533 |
| walker |  | 4638 | 237 | Code::CodeKey { rung: Decl, file: src/tomli/_re.py, decl: 3, sub: 0, line: 26 } |  |  | 0.535 |
| walker |  | 4672 | 34 | Code::CodeKey { rung: Names, file: src/tomli/_types.py, decl: 0, sub: 0, line: 0 } |  |  | 0.539 |
| ns | 4716 |  | 194 | pyproject.toml: build backend and project metadata | 3.6 |  | 0.534 |
| walker |  | 4767 | 95 | Code::CodeKey { rung: Doc, file: src/tomli/_parser.py, decl: 54, sub: 0, line: 764 } |  |  | 0.534 |
| ns | 4774 |  | 58 | Listings of the helper trees: benchmark, fuzzer, profiler, scripts, workflows | 3.7 |  | 0.549 |
| ns | 4957 |  | 183 | test_data.py: how the TOML corpus is discovered and compared | 3.8 | 3.3 | 0.538 |
| ns | 5096 |  | 139 | tests/burntsushi.py: purpose and complete function roster | 3.9 |  | 0.531 |
| walker |  | 5108 | 341 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.531 |
| walker |  | 5218 | 110 | Code::CodeKey { rung: Doc, file: src/tomli/_parser.py, decl: 16, sub: 0, line: 76 } |  |  | 0.559 |
| ns | 5347 |  | 251 | Flags: the two flag constants, the state fields, and the complete method roster | 4.1 | 2.2 | 0.562 |
| ns | 5471 |  | 124 | NestedDict: the parsed-document container and its two methods | 4.2 | 2.2 | 0.562 |
| walker |  | 5610 | 392 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.562 |
| ns | 5732 |  | 261 | create_dict_rule(): the [table] statement | 4.3 | 2.1 | 0.549 |
| walker |  | 5794 | 184 | Markdown::Section { file: README.md, section_index: 10, keeps_default_concavity: false } |  |  | 0.549 |
| ns | 6032 |  | 300 | create_list_rule(): the [[array of tables]] statement | 4.4 | 2.1 | 0.536 |
| walker |  | 6274 | 480 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.536 |
| walker |  | 6331 | 57 | Code::CodeKey { rung: Body, file: fuzzer/fuzz.py, decl: 2, sub: 0, line: 53 } |  |  | 0.536 |
| walker |  | 6395 | 64 | Code::CodeKey { rung: Body, file: fuzzer/fuzz.py, decl: 4, sub: 0, line: 71 } |  |  | 0.536 |
| walker |  | 6457 | 62 | Code::CodeKey { rung: Doc, file: src/tomli/_re.py, decl: 6, sub: 0, line: 59 } |  |  | 0.536 |
| ns | 6510 |  | 478 | key_value_rule(): dotted keys, pending flags and immutability | 4.5 | 2.1 | 0.517 |
| walker |  | 6524 | 67 | Code::CodeKey { rung: Doc, file: src/tomli/_re.py, decl: 7, sub: 0, line: 98 } |  |  | 0.517 |
| walker |  | 6537 | 13 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 22, sub: 0, line: 233 } |  |  | 0.519 |
| walker |  | 6643 | 106 | Code::CodeKey { rung: Body, file: fuzzer/fuzz.py, decl: 3, sub: 0, line: 59 } |  |  | 0.519 |
| walker |  | 6658 | 15 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 47, sub: 0, line: 595 } |  |  | 0.519 |
| walker |  | 6677 | 19 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 32, sub: 0, line: 313 } |  |  | 0.526 |
| ns | 6689 |  | 179 | parse_key_part(): bare, literal and basic key forms | 4.6 | 2.1 | 0.518 |
| walker |  | 6701 | 24 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 43, sub: 0, line: 497 } |  |  | 0.518 |
| walker |  | 6709 | 8 | Plaintext::DeclSurface { file: scripts/requirements.txt } |  |  | 0.518 |
| walker |  | 6736 | 27 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 53, sub: 0, line: 760 } |  |  | 0.518 |
| walker |  | 6770 | 34 | Code::CodeKey { rung: Body, file: src/tomli/_re.py, decl: 9, sub: 0, line: 116 } |  |  | 0.518 |
| walker |  | 6798 | 28 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 28, sub: 0, line: 279 } |  |  | 0.525 |
| walker |  | 6836 | 38 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 21, sub: 0, line: 229 } |  |  | 0.532 |
| walker |  | 6874 | 38 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 23, sub: 0, line: 236 } |  |  | 0.533 |
| ns | 6974 |  | 285 | parse_array(): array literals and trailing commas | 4.7 | 2.1 | 0.520 |
| walker |  | 6975 | 101 | Fs::DirListing { dir: tests/data/valid } |  |  | 0.520 |
| walker |  | 6985 | 10 | Fs::DirListing { dir: tests/data/valid/_external/toml-test } |  |  | 0.520 |
| walker |  | 7007 | 22 | Fs::DirListing { dir: tests/data/valid/dates-and-times } |  |  | 0.520 |
| walker |  | 7033 | 26 | Fs::DirListing { dir: tests/data/valid/array } |  |  | 0.520 |
| walker |  | 7059 | 26 | Fs::DirListing { dir: tests/data/valid/inline-table } |  |  | 0.520 |
| walker |  | 7087 | 28 | Fs::DirListing { dir: tests/data/valid/multiline-basic-str } |  |  | 0.520 |
| walker |  | 7138 | 51 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 33, sub: 0, line: 318 } |  |  | 0.520 |
| ns | 7463 |  | 489 | parse_inline_table(): inline tables and their local flag scope | 4.8 | 2.1 | 0.503 |
| walker |  | 7481 | 343 | Code::CodeKey { rung: Body, file: fuzzer/fuzz.py, decl: 1, sub: 0, line: 20 } |  |  | 0.503 |
| walker |  | 7498 | 17 | Code::CodeKey { rung: Names, file: tests/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.503 |
| walker |  | 7568 | 70 | Code::CodeKey { rung: Body, file: src/tomli/_re.py, decl: 7, sub: 0, line: 98 } |  |  | 0.503 |
| ns | 7664 |  | 201 | parse_hex_char() and is_unicode_scalar_value(): escape validation | 4.9 | 2.1 | 0.499 |
| walker |  | 7722 | 154 | Fs::DirListing { dir: tests/data/invalid } |  |  | 0.499 |
| walker |  | 7728 | 6 | Fs::DirListing { dir: tests/data/invalid/dates-and-times } |  |  | 0.499 |
| walker |  | 7734 | 6 | Fs::DirListing { dir: tests/data/invalid/literal-str } |  |  | 0.499 |
| walker |  | 7749 | 15 | Fs::DirListing { dir: tests/data/invalid/multiline-literal-str } |  |  | 0.499 |
| walker |  | 7759 | 10 | Fs::DirListing { dir: tests/data/invalid/_external/toml-test } |  |  | 0.499 |
| walker |  | 7777 | 18 | Fs::DirListing { dir: tests/data/invalid/array-of-tables } |  |  | 0.499 |
| walker |  | 7795 | 18 | Fs::DirListing { dir: tests/data/invalid/boolean } |  |  | 0.499 |
| ns | 7806 |  | 142 | parse_literal_str() and parse_one_line_basic_str(): the two short string entry points | 4.10 | 2.1 | 0.496 |
| walker |  | 7816 | 21 | Fs::DirListing { dir: tests/data/invalid/table } |  |  | 0.496 |
| walker |  | 7839 | 23 | Fs::DirListing { dir: tests/data/invalid/array } |  |  | 0.496 |
| walker |  | 7872 | 33 | Fs::DirListing { dir: tests/data/invalid/dotted-keys } |  |  | 0.496 |
| walker |  | 7912 | 40 | Fs::DirListing { dir: tests/data/invalid/keys-and-vals } |  |  | 0.496 |
| walker |  | 7954 | 42 | Fs::DirListing { dir: tests/data/invalid/multiline-basic-str } |  |  | 0.496 |
| ns | 8040 |  | 234 | make_safe_parse_float(): the parse_float contract | 4.11 | 2.1 | 0.493 |
| walker |  | 8043 | 89 | Fs::DirListing { dir: tests/data/invalid/inline-table } |  |  | 0.493 |
| walker |  | 8107 | 64 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 24, sub: 0, line: 241 } |  |  | 0.494 |
| ns | 8132 |  | 92 | TOMLDecodeError.__init__ signature and the deprecated free-form form | 5.1 | 1.8 | 0.499 |
| walker |  | 8190 | 83 | Code::CodeKey { rung: Body, file: src/tomli/_re.py, decl: 8, sub: 0, line: 109 } |  |  | 0.499 |
| walker |  | 8260 | 70 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 36, sub: 0, line: 361 } |  |  | 0.499 |
| walker |  | 8316 | 56 | Fs::DirListing { dir: tests/data/invalid/_external/toml-test/invalid } |  |  | 0.499 |
| walker |  | 8353 | 37 | Code::CodeKey { rung: Names, file: benchmark/run.py, decl: 0, sub: 0, line: 0 } |  |  | 0.499 |
| ns | 8359 |  | 227 | TOMLDecodeError: line/column computation and message formatting | 5.2 | 5.1 | 0.491 |
| walker |  | 8403 | 50 | Code::CodeKey { rung: Decl, file: benchmark/run.py, decl: 1, sub: 0, line: 15 } |  |  | 0.491 |
| walker |  | 8493 | 90 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 49, sub: 0, line: 612 } |  |  | 0.501 |
| ns | 8534 |  | 175 | Complete roster of src/tomli/_re.py: four regexes and four functions | 5.3 |  | 0.513 |
| walker |  | 8544 | 51 | Fs::DirListing { dir: tests/data/invalid/_external/toml-test/invalid/local-time } |  |  | 0.513 |
| walker |  | 8560 | 16 | Code::CodeKey { rung: Names, file: scripts/use_setuptools.py, decl: 0, sub: 0, line: 0 } |  |  | 0.513 |
| walker |  | 8657 | 97 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 18, sub: 0, line: 137 } |  |  | 0.523 |
| walker |  | 8743 | 86 | Code::CodeKey { rung: Names, file: tests/burntsushi.py, decl: 0, sub: 0, line: 0 } |  |  | 0.527 |
| ns | 8771 |  | 237 | RE_NUMBER: the integer and float grammar | 5.4 | 5.3 | 0.538 |
| walker |  | 8843 | 100 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 35, sub: 0, line: 349 } |  |  | 0.538 |
| ns | 8900 |  | 129 | match_to_datetime docstring and the cached_tz cache-size note | 5.5 | 5.3 | 0.543 |
| walker |  | 8957 | 114 | Code::CodeKey { rung: Names, file: tests/test_data.py, decl: 0, sub: 0, line: 0 } |  |  | 0.547 |
| walker |  | 8979 | 22 | Code::CodeKey { rung: Decl, file: tests/test_data.py, decl: 6, sub: 0, line: 27 } |  |  | 0.547 |
| walker |  | 8995 | 16 | Code::CodeKey { rung: Names, file: tests/test_misc.py, decl: 0, sub: 0, line: 0 } |  |  | 0.548 |
| walker |  | 9091 | 96 | Code::CodeKey { rung: Decl, file: tests/test_misc.py, decl: 1, sub: 0, line: 17 } |  |  | 0.554 |
| walker |  | 9106 | 15 | Code::CodeKey { rung: Names, file: tests/test_error.py, decl: 0, sub: 0, line: 0 } |  |  | 0.556 |
| ns | 9132 |  | 232 | Complete roster of tox environments with their descriptions | 6.1 | 3.4 | 0.550 |
| walker |  | 9206 | 100 | Code::CodeKey { rung: Decl, file: tests/test_error.py, decl: 1, sub: 0, line: 13 } |  |  | 0.564 |
| walker |  | 9274 | 68 | Fs::DirListing { dir: tests/data/invalid/_external/toml-test/invalid/spec-1.1.0 } |  |  | 0.564 |
| ns | 9338 |  | 206 | tomllib.md: section map and the CPython sync procedure | 6.2 |  | 0.558 |
| walker |  | 9383 | 109 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 30, sub: 0, line: 300 } |  |  | 0.559 |
| walker |  | 9402 | 19 | Code::CodeKey { rung: Body, file: tests/burntsushi.py, decl: 4, sub: 0, line: 92 } |  |  | 0.559 |
| walker |  | 9463 | 61 | Plaintext::DeclSurface { file: benchmark/requirements.txt } |  |  | 0.559 |
| ns | 9472 |  | 134 | CHANGELOG.md: the two most recent releases | 6.3 |  | 0.552 |
| walker |  | 9488 | 25 | Plaintext::Whole { file: benchmark/requirements.txt } |  |  | 0.552 |
| walker |  | 9508 | 20 | Code::CodeKey { rung: Body, file: tests/test_misc.py, decl: 8, sub: 0, line: 133 } |  |  | 0.552 |
| walker |  | 9594 | 86 | Fs::DirListing { dir: tests/data/invalid/_external/toml-test/invalid/local-date } |  |  | 0.552 |
| ns | 9637 |  | 165 | CI workflow: the complete job list and test matrix | 6.4 |  | 0.548 |
| walker |  | 9716 | 122 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 29, sub: 0, line: 283 } |  |  | 0.549 |
| ns | 9808 |  | 171 | pre-commit: the complete list of hook ids | 6.5 |  | 0.544 |
| walker |  | 9812 | 96 | Fs::DirListing { dir: tests/data/invalid/_external/toml-test/invalid/local-datetime } |  |  | 0.544 |
| walker |  | 9844 | 32 | Code::CodeKey { rung: Doc, file: tests/burntsushi.py, decl: 2, sub: 0, line: 42 } |  |  | 0.548 |
| walker |  | 9974 | 130 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 54, sub: 0, line: 764 } |  |  | 0.564 |
| walker |  | 9992 | 18 | Fs::DirListing { dir: tests/data/invalid/_external/toml-test/invalid/bool } |  |  | 0.564 |
| ns | 9993 |  | 185 | Lint and version-bump configuration: .flake8 and .bumpversion.cfg | 6.6 |  | 0.557 |
