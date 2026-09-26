Score(3000)=0.496 I=0.807 C=0.304 ns_rows≤3K=18/54 grid(1000/1442/2080/3000/4327/6240/9000)=0.690/0.572/0.558/0.496/0.550/0.530/0.544

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
| walker |  | 415 | 16 | Code::CodeKey { rung: Names, file: src/tomli/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.817 |
| ns | 495 |  | 88 | Entry points: load() and loads() signatures with docstrings | 1.7 |  | 0.772 |
| ns | 621 |  | 126 | TOMLDecodeError class docstring and its attributes | 1.8 |  | 0.683 |
| ns | 681 |  | 60 | src/tomli/_types.py type aliases (complete file body) | 1.9 |  | 0.641 |
| ns | 765 |  | 84 | README mypyc/pure-Python distribution note | 1.10 |  | 0.617 |
| walker |  | 800 | 385 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.634 |
| walker |  | 836 | 36 | Markdown::CommandBlock { file: README.md, row: 135 } |  |  | 0.634 |
| walker |  | 861 | 25 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.634 |
| walker |  | 875 | 14 | Code::CodeKey { rung: Names, file: profiler/profiler_script.py, decl: 0, sub: 0, line: 0 } |  |  | 0.634 |
| walker |  | 907 | 32 | Code::CodeKey { rung: Decl, file: profiler/profiler_script.py, decl: 1, sub: 0, line: 12 } |  |  | 0.634 |
| walker |  | 969 | 62 | Code::CodeKey { rung: Names, file: fuzzer/fuzz.py, decl: 0, sub: 0, line: 0 } |  |  | 0.634 |
| ns | 971 |  | 206 | README section map: every H2 and H3 heading | 1.11 |  | 0.689 |
| walker |  | 975 | 6 | Fs::DirListing { dir: tests/data } |  |  | 0.690 |
| walker |  | 993 | 18 | Code::CodeKey { rung: Doc, file: fuzzer/fuzz.py, decl: 3, sub: 0, line: 59 } |  |  | 0.690 |
| walker |  | 1037 | 44 | Plaintext::DeclSurface { file: fuzzer/requirements.txt } |  |  | 0.690 |
| ns | 1219 |  | 248 | Complete roster of module-level functions in _parser.py (names only) | 2.1 |  | 0.598 |
| walker |  | 1225 | 188 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.598 |
| ns | 1339 |  | 120 | Complete roster of classes in _parser.py, with Output's fields | 2.2 |  | 0.560 |
| walker |  | 1355 | 130 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.560 |
| ns | 1575 |  | 236 | loads(): parse state setup and the statement-loop rule enumeration | 2.3 | 1.7 | 0.512 |
| walker |  | 1656 | 301 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.602 |
| ns | 1718 |  | 143 | parse_value() signature and the inline-nesting recursion guard | 2.4 | 2.1 | 0.577 |
| walker |  | 1826 | 170 | Code::CodeKey { rung: Names, file: src/tomli/_parser.py, decl: 0, sub: 0, line: 0 } |  |  | 0.578 |
| ns | 1923 |  | 205 | MAX_INLINE_NESTING and the mypyc stack-overflow rationale | 2.5 | 2.4 | 0.556 |
| walker |  | 1995 | 169 | Code::CodeKey { rung: Names, file: src/tomli/_parser.py, decl: 0, sub: 1, line: 0 } |  |  | 0.558 |
| walker |  | 2018 | 23 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 11, sub: 0, line: 51 } |  |  | 0.558 |
| walker |  | 2174 | 156 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 14, sub: 0, line: 57 } |  |  | 0.561 |
| ns | 2356 |  | 433 | loads(): the statement dispatch body and its two top-level errors | 2.6 | 2.3 | 0.491 |
| walker |  | 2362 | 188 | Code::CodeKey { rung: Names, file: src/tomli/_parser.py, decl: 0, sub: 2, line: 0 } |  |  | 0.505 |
| walker |  | 2376 | 14 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 31, sub: 0, line: 312 } |  |  | 0.508 |
| walker |  | 2395 | 19 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 16, sub: 0, line: 76 } |  |  | 0.508 |
| walker |  | 2451 | 56 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 34, sub: 0, line: 327 } |  |  | 0.508 |
| walker |  | 2515 | 64 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 27, sub: 0, line: 278 } |  |  | 0.510 |
| walker |  | 2547 | 32 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 29, sub: 0, line: 283 } |  |  | 0.511 |
| walker |  | 2622 | 75 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 17, sub: 0, line: 87 } |  |  | 0.511 |
| ns | 2733 |  | 377 | parse_value(): string, boolean, array and inline-table dispatch | 2.7 | 2.4 | 0.465 |
| walker |  | 2837 | 215 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 20, sub: 0, line: 220 } |  |  | 0.467 |
| walker |  | 2850 | 13 | Code::CodeKey { rung: Doc, file: src/tomli/_parser.py, decl: 19, sub: 0, line: 149 } |  |  | 0.473 |
| walker |  | 2864 | 14 | Code::CodeKey { rung: Doc, file: src/tomli/_parser.py, decl: 20, sub: 0, line: 220 } |  |  | 0.479 |
| walker |  | 2879 | 15 | Code::CodeKey { rung: Doc, file: src/tomli/_parser.py, decl: 18, sub: 0, line: 137 } |  |  | 0.487 |
| walker |  | 3062 | 183 | Code::CodeKey { rung: Names, file: src/tomli/_parser.py, decl: 0, sub: 3, line: 0 } |  |  | 0.506 |
| walker |  | 3084 | 22 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 40, sub: 0, line: 447 } |  |  | 0.506 |
| walker |  | 3109 | 25 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 39, sub: 0, line: 413 } |  |  | 0.506 |
| ns | 3121 |  | 388 | parse_value(): datetime, number and special-float dispatch | 2.8 | 2.4 | 0.472 |
| walker |  | 3396 | 287 | Code::CodeKey { rung: Names, file: src/tomli/_parser.py, decl: 0, sub: 4, line: 0 } |  |  | 0.536 |
| walker |  | 3414 | 18 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 46, sub: 0, line: 564 } |  |  | 0.536 |
| walker |  | 3436 | 22 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 44, sub: 0, line: 502 } |  |  | 0.536 |
| ns | 3452 |  | 331 | Character-class constants: the complete set | 2.9 |  | 0.549 |
| walker |  | 3458 | 22 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 45, sub: 0, line: 528 } |  |  | 0.549 |
| walker |  | 3480 | 22 | Code::CodeKey { rung: Decl, file: src/tomli/_parser.py, decl: 52, sub: 0, line: 684 } |  |  | 0.551 |
| walker |  | 3512 | 32 | Code::CodeKey { rung: Doc, file: src/tomli/_parser.py, decl: 15, sub: 0, line: 71 } |  |  | 0.565 |
| ns | 3549 |  | 97 | load() body: the binary-mode requirement | 2.10 | 1.7 | 0.556 |
| ns | 3661 |  | 112 | loads() prologue: CRLF normalisation and the str type check | 2.11 | 1.7 | 0.547 |
| walker |  | 3754 | 242 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.547 |
| ns | 3835 |  | 174 | BASIC_STR_ESCAPE_REPLACEMENTS: the full escape table | 2.12 |  | 0.564 |
| ns | 3871 |  | 36 | tests/ and tests/data/ listings (complete) | 3.1 |  | 0.574 |
| walker |  | 3930 | 176 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.568 |
| ns | 3930 |  | 59 | tests/__init__.py: the tomli-as-tomllib alias | 3.2 |  | 0.568 |
| walker |  | 4105 | 175 | Code::CodeKey { rung: Names, file: src/tomli/_re.py, decl: 0, sub: 0, line: 0 } |  |  | 0.569 |
| walker |  | 4117 | 12 | Code::CodeKey { rung: Decl, file: src/tomli/_re.py, decl: 7, sub: 0, line: 98 } |  |  | 0.569 |
| ns | 4194 |  | 264 | Complete roster of test classes and test methods | 3.3 |  | 0.550 |
| walker |  | 4221 | 104 | Code::CodeKey { rung: Decl, file: src/tomli/_re.py, decl: 2, sub: 0, line: 17 } |  |  | 0.550 |
| ns | 4370 |  | 176 | tox core configuration: the interpreter matrix and default command | 3.4 |  | 0.540 |
| walker |  | 4380 | 159 | Code::CodeKey { rung: Decl, file: src/tomli/_re.py, decl: 5, sub: 0, line: 46 } |  |  | 0.540 |
| ns | 4522 |  | 152 | setup.py: the mypyc build path | 3.5 |  | 0.527 |
| walker |  | 4617 | 237 | Code::CodeKey { rung: Decl, file: src/tomli/_re.py, decl: 3, sub: 0, line: 26 } |  |  | 0.528 |
| walker |  | 4651 | 34 | Code::CodeKey { rung: Names, file: src/tomli/_types.py, decl: 0, sub: 0, line: 0 } |  |  | 0.533 |
| ns | 4716 |  | 194 | pyproject.toml: build backend and project metadata | 3.6 |  | 0.528 |
| walker |  | 4746 | 95 | Code::CodeKey { rung: Doc, file: src/tomli/_parser.py, decl: 54, sub: 0, line: 764 } |  |  | 0.528 |
| ns | 4774 |  | 58 | Listings of the helper trees: benchmark, fuzzer, profiler, scripts, workflows | 3.7 |  | 0.543 |
| ns | 4957 |  | 183 | test_data.py: how the TOML corpus is discovered and compared | 3.8 | 3.3 | 0.532 |
| walker |  | 5087 | 341 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.532 |
| ns | 5096 |  | 139 | tests/burntsushi.py: purpose and complete function roster | 3.9 |  | 0.525 |
| walker |  | 5197 | 110 | Code::CodeKey { rung: Doc, file: src/tomli/_parser.py, decl: 16, sub: 0, line: 76 } |  |  | 0.553 |
| ns | 5347 |  | 251 | Flags: the two flag constants, the state fields, and the complete method roster | 4.1 | 2.2 | 0.556 |
| ns | 5471 |  | 124 | NestedDict: the parsed-document container and its two methods | 4.2 | 2.2 | 0.556 |
| walker |  | 5589 | 392 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.556 |
| ns | 5732 |  | 261 | create_dict_rule(): the [table] statement | 4.3 | 2.1 | 0.544 |
| walker |  | 5773 | 184 | Markdown::Section { file: README.md, section_index: 10, keeps_default_concavity: false } |  |  | 0.544 |
| ns | 6032 |  | 300 | create_list_rule(): the [[array of tables]] statement | 4.4 | 2.1 | 0.530 |
| walker |  | 6253 | 480 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.530 |
| walker |  | 6310 | 57 | Code::CodeKey { rung: Body, file: fuzzer/fuzz.py, decl: 2, sub: 0, line: 53 } |  |  | 0.530 |
| walker |  | 6374 | 64 | Code::CodeKey { rung: Body, file: fuzzer/fuzz.py, decl: 4, sub: 0, line: 71 } |  |  | 0.530 |
| walker |  | 6436 | 62 | Code::CodeKey { rung: Doc, file: src/tomli/_re.py, decl: 6, sub: 0, line: 59 } |  |  | 0.530 |
| walker |  | 6503 | 67 | Code::CodeKey { rung: Doc, file: src/tomli/_re.py, decl: 7, sub: 0, line: 98 } |  |  | 0.531 |
| ns | 6510 |  | 478 | key_value_rule(): dotted keys, pending flags and immutability | 4.5 | 2.1 | 0.512 |
| walker |  | 6516 | 13 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 22, sub: 0, line: 233 } |  |  | 0.513 |
| walker |  | 6622 | 106 | Code::CodeKey { rung: Body, file: fuzzer/fuzz.py, decl: 3, sub: 0, line: 59 } |  |  | 0.513 |
| walker |  | 6637 | 15 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 47, sub: 0, line: 595 } |  |  | 0.513 |
| walker |  | 6656 | 19 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 32, sub: 0, line: 313 } |  |  | 0.521 |
| walker |  | 6680 | 24 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 43, sub: 0, line: 497 } |  |  | 0.521 |
| walker |  | 6688 | 8 | Plaintext::DeclSurface { file: scripts/requirements.txt } |  |  | 0.521 |
| ns | 6689 |  | 179 | parse_key_part(): bare, literal and basic key forms | 4.6 | 2.1 | 0.513 |
| walker |  | 6715 | 27 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 53, sub: 0, line: 760 } |  |  | 0.513 |
| walker |  | 6749 | 34 | Code::CodeKey { rung: Body, file: src/tomli/_re.py, decl: 9, sub: 0, line: 116 } |  |  | 0.513 |
| walker |  | 6777 | 28 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 28, sub: 0, line: 279 } |  |  | 0.520 |
| walker |  | 6815 | 38 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 21, sub: 0, line: 229 } |  |  | 0.526 |
| walker |  | 6853 | 38 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 23, sub: 0, line: 236 } |  |  | 0.528 |
| walker |  | 6954 | 101 | Fs::DirListing { dir: tests/data/valid } |  |  | 0.528 |
| walker |  | 6964 | 10 | Fs::DirListing { dir: tests/data/valid/_external/toml-test } |  |  | 0.528 |
| ns | 6974 |  | 285 | parse_array(): array literals and trailing commas | 4.7 | 2.1 | 0.515 |
| walker |  | 6986 | 22 | Fs::DirListing { dir: tests/data/valid/dates-and-times } |  |  | 0.515 |
| walker |  | 7012 | 26 | Fs::DirListing { dir: tests/data/valid/array } |  |  | 0.515 |
| walker |  | 7038 | 26 | Fs::DirListing { dir: tests/data/valid/inline-table } |  |  | 0.515 |
| walker |  | 7066 | 28 | Fs::DirListing { dir: tests/data/valid/multiline-basic-str } |  |  | 0.515 |
| walker |  | 7117 | 51 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 33, sub: 0, line: 318 } |  |  | 0.515 |
| walker |  | 7460 | 343 | Code::CodeKey { rung: Body, file: fuzzer/fuzz.py, decl: 1, sub: 0, line: 20 } |  |  | 0.515 |
| ns | 7463 |  | 489 | parse_inline_table(): inline tables and their local flag scope | 4.8 | 2.1 | 0.498 |
| walker |  | 7477 | 17 | Code::CodeKey { rung: Names, file: tests/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.499 |
| walker |  | 7547 | 70 | Code::CodeKey { rung: Body, file: src/tomli/_re.py, decl: 7, sub: 0, line: 98 } |  |  | 0.499 |
| ns | 7664 |  | 201 | parse_hex_char() and is_unicode_scalar_value(): escape validation | 4.9 | 2.1 | 0.494 |
| walker |  | 7701 | 154 | Fs::DirListing { dir: tests/data/invalid } |  |  | 0.494 |
| walker |  | 7707 | 6 | Fs::DirListing { dir: tests/data/invalid/dates-and-times } |  |  | 0.494 |
| walker |  | 7713 | 6 | Fs::DirListing { dir: tests/data/invalid/literal-str } |  |  | 0.494 |
| walker |  | 7728 | 15 | Fs::DirListing { dir: tests/data/invalid/multiline-literal-str } |  |  | 0.494 |
| walker |  | 7738 | 10 | Fs::DirListing { dir: tests/data/invalid/_external/toml-test } |  |  | 0.494 |
| walker |  | 7756 | 18 | Fs::DirListing { dir: tests/data/invalid/array-of-tables } |  |  | 0.494 |
| walker |  | 7774 | 18 | Fs::DirListing { dir: tests/data/invalid/boolean } |  |  | 0.494 |
| walker |  | 7795 | 21 | Fs::DirListing { dir: tests/data/invalid/table } |  |  | 0.494 |
| ns | 7806 |  | 142 | parse_literal_str() and parse_one_line_basic_str(): the two short string entry points | 4.10 | 2.1 | 0.491 |
| walker |  | 7818 | 23 | Fs::DirListing { dir: tests/data/invalid/array } |  |  | 0.491 |
| walker |  | 7851 | 33 | Fs::DirListing { dir: tests/data/invalid/dotted-keys } |  |  | 0.491 |
| walker |  | 7891 | 40 | Fs::DirListing { dir: tests/data/invalid/keys-and-vals } |  |  | 0.491 |
| walker |  | 7933 | 42 | Fs::DirListing { dir: tests/data/invalid/multiline-basic-str } |  |  | 0.491 |
| walker |  | 8022 | 89 | Fs::DirListing { dir: tests/data/invalid/inline-table } |  |  | 0.491 |
| ns | 8040 |  | 234 | make_safe_parse_float(): the parse_float contract | 4.11 | 2.1 | 0.488 |
| walker |  | 8086 | 64 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 24, sub: 0, line: 241 } |  |  | 0.489 |
| ns | 8132 |  | 92 | TOMLDecodeError.__init__ signature and the deprecated free-form form | 5.1 | 1.8 | 0.494 |
| walker |  | 8169 | 83 | Code::CodeKey { rung: Body, file: src/tomli/_re.py, decl: 8, sub: 0, line: 109 } |  |  | 0.494 |
| walker |  | 8239 | 70 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 36, sub: 0, line: 361 } |  |  | 0.494 |
| walker |  | 8295 | 56 | Fs::DirListing { dir: tests/data/invalid/_external/toml-test/invalid } |  |  | 0.494 |
| walker |  | 8332 | 37 | Code::CodeKey { rung: Names, file: benchmark/run.py, decl: 0, sub: 0, line: 0 } |  |  | 0.494 |
| ns | 8359 |  | 227 | TOMLDecodeError: line/column computation and message formatting | 5.2 | 5.1 | 0.486 |
| walker |  | 8382 | 50 | Code::CodeKey { rung: Decl, file: benchmark/run.py, decl: 1, sub: 0, line: 15 } |  |  | 0.486 |
| walker |  | 8472 | 90 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 49, sub: 0, line: 612 } |  |  | 0.497 |
| walker |  | 8523 | 51 | Fs::DirListing { dir: tests/data/invalid/_external/toml-test/invalid/local-time } |  |  | 0.497 |
| ns | 8534 |  | 175 | Complete roster of src/tomli/_re.py: four regexes and four functions | 5.3 |  | 0.508 |
| walker |  | 8539 | 16 | Code::CodeKey { rung: Names, file: scripts/use_setuptools.py, decl: 0, sub: 0, line: 0 } |  |  | 0.508 |
| walker |  | 8636 | 97 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 18, sub: 0, line: 137 } |  |  | 0.519 |
| walker |  | 8722 | 86 | Code::CodeKey { rung: Names, file: tests/burntsushi.py, decl: 0, sub: 0, line: 0 } |  |  | 0.522 |
| ns | 8771 |  | 237 | RE_NUMBER: the integer and float grammar | 5.4 | 5.3 | 0.534 |
| walker |  | 8822 | 100 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 35, sub: 0, line: 349 } |  |  | 0.534 |
| ns | 8900 |  | 129 | match_to_datetime docstring and the cached_tz cache-size note | 5.5 | 5.3 | 0.538 |
| walker |  | 8936 | 114 | Code::CodeKey { rung: Names, file: tests/test_data.py, decl: 0, sub: 0, line: 0 } |  |  | 0.542 |
| walker |  | 8958 | 22 | Code::CodeKey { rung: Decl, file: tests/test_data.py, decl: 6, sub: 0, line: 27 } |  |  | 0.543 |
| walker |  | 8974 | 16 | Code::CodeKey { rung: Names, file: tests/test_misc.py, decl: 0, sub: 0, line: 0 } |  |  | 0.543 |
| walker |  | 9070 | 96 | Code::CodeKey { rung: Decl, file: tests/test_misc.py, decl: 1, sub: 0, line: 17 } |  |  | 0.550 |
| walker |  | 9085 | 15 | Code::CodeKey { rung: Names, file: tests/test_error.py, decl: 0, sub: 0, line: 0 } |  |  | 0.551 |
| ns | 9132 |  | 232 | Complete roster of tox environments with their descriptions | 6.1 | 3.4 | 0.546 |
| walker |  | 9185 | 100 | Code::CodeKey { rung: Decl, file: tests/test_error.py, decl: 1, sub: 0, line: 13 } |  |  | 0.559 |
| walker |  | 9253 | 68 | Fs::DirListing { dir: tests/data/invalid/_external/toml-test/invalid/spec-1.1.0 } |  |  | 0.559 |
| ns | 9338 |  | 206 | tomllib.md: section map and the CPython sync procedure | 6.2 |  | 0.553 |
| walker |  | 9362 | 109 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 30, sub: 0, line: 300 } |  |  | 0.554 |
| walker |  | 9381 | 19 | Code::CodeKey { rung: Body, file: tests/burntsushi.py, decl: 4, sub: 0, line: 92 } |  |  | 0.554 |
| walker |  | 9442 | 61 | Plaintext::DeclSurface { file: benchmark/requirements.txt } |  |  | 0.554 |
| walker |  | 9467 | 25 | Plaintext::Whole { file: benchmark/requirements.txt } |  |  | 0.554 |
| ns | 9472 |  | 134 | CHANGELOG.md: the two most recent releases | 6.3 |  | 0.548 |
| walker |  | 9487 | 20 | Code::CodeKey { rung: Body, file: tests/test_misc.py, decl: 8, sub: 0, line: 133 } |  |  | 0.548 |
| walker |  | 9573 | 86 | Fs::DirListing { dir: tests/data/invalid/_external/toml-test/invalid/local-date } |  |  | 0.548 |
| ns | 9637 |  | 165 | CI workflow: the complete job list and test matrix | 6.4 |  | 0.544 |
| walker |  | 9695 | 122 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 29, sub: 0, line: 283 } |  |  | 0.545 |
| walker |  | 9791 | 96 | Fs::DirListing { dir: tests/data/invalid/_external/toml-test/invalid/local-datetime } |  |  | 0.545 |
| ns | 9808 |  | 171 | pre-commit: the complete list of hook ids | 6.5 |  | 0.540 |
| walker |  | 9823 | 32 | Code::CodeKey { rung: Doc, file: tests/burntsushi.py, decl: 2, sub: 0, line: 42 } |  |  | 0.544 |
| walker |  | 9953 | 130 | Code::CodeKey { rung: Body, file: src/tomli/_parser.py, decl: 54, sub: 0, line: 764 } |  |  | 0.559 |
| ns | 9993 |  | 185 | Lint and version-bump configuration: .flake8 and .bumpversion.cfg | 6.6 |  | 0.553 |
| walker |  | 9995 | 42 | Fs::DirListing { dir: tests/data/invalid/_external/toml-test/invalid/bool } |  |  | 0.553 |
