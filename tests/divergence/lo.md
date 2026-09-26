Score(3000)=0.638 I=0.731 C=0.557 ns_rows≤3K=16/55 grid(1000/1442/2080/3000/4327/6240/9000)=0.753/0.626/0.614/0.638/0.595/0.570/0.521

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 50 |  | 50 | What lo is: README title and one-line pitch | 1.1 |  | 0.000 |
| ns | 125 |  | 75 | Scope statement: what the library actually covers | 1.2 |  | 0.000 |
| ns | 160 |  | 35 | The helper taxonomy: all fifteen category headers | 1.3 |  | 0.000 |
| walker |  | 175 | 175 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 184 | 9 | Fs::DirListing { dir: parallel } |  |  | 0.000 |
| walker |  | 195 | 11 | Fs::DirListing { dir: internal } |  |  | 0.000 |
| walker |  | 210 | 15 | Fs::DirListing { dir: mutable } |  |  | 0.000 |
| ns | 306 |  | 146 | The four public import paths and a call example | 1.4 |  | 0.000 |
| walker |  | 340 | 130 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.358 |
| walker |  | 369 | 29 | GoMod::Identity { file: go.mod } |  |  | 0.358 |
| walker |  | 381 | 12 | Fs::DirListing { dir: internal/xrand } |  |  | 0.359 |
| walker |  | 401 | 20 | Fs::DirListing { dir: internal/constraints } |  |  | 0.359 |
| walker |  | 422 | 21 | Fs::DirListing { dir: internal/xtime } |  |  | 0.361 |
| walker |  | 435 | 13 | Code::CodeKey { rung: ModuleDoc, file: internal/xtime/fake.go, decl: 0, sub: 0, line: 0 } |  |  | 0.361 |
| walker |  | 448 | 13 | Code::CodeKey { rung: ModuleDoc, file: internal/xtime/real.go, decl: 0, sub: 0, line: 0 } |  |  | 0.361 |
| walker |  | 461 | 13 | Code::CodeKey { rung: ModuleDoc, file: internal/xtime/time.go, decl: 0, sub: 0, line: 0 } |  |  | 0.361 |
| ns | 481 |  | 175 | Complete root directory listing | 1.5 |  | 0.700 |
| walker |  | 595 | 134 | Fs::DirListing { dir: it } |  |  | 0.739 |
| walker |  | 614 | 19 | Fs::DirListing { dir: .github } |  |  | 0.739 |
| ns | 639 |  | 158 | Sibling helper packages: it/, mutable/, parallel/ listings | 1.6 |  | 0.747 |
| walker |  | 640 | 26 | Fs::DirListing { dir: .github/workflows } |  |  | 0.748 |
| walker |  | 670 | 30 | Code::CodeKey { rung: ModuleDoc, file: internal/constraints/constraints.go, decl: 0, sub: 0, line: 0 } |  |  | 0.748 |
| walker |  | 767 | 97 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.752 |
| walker |  | 788 | 21 | Markdown::Section { file: README.md, section_index: 108, keeps_default_concavity: false } |  |  | 0.752 |
| ns | 829 |  | 190 | internal/ and exp/ listings | 1.7 |  | 0.651 |
| walker |  | 871 | 83 | GoMod::File { file: go.mod } |  |  | 0.651 |
| walker |  | 886 | 15 | Fs::DirListing { dir: .github/PULL_REQUEST_TEMPLATE } |  |  | 0.652 |
| walker |  | 1012 | 126 | Fs::DirListing { dir: exp/simd } |  |  | 0.776 |
| walker |  | 1047 | 35 | GoMod::Identity { file: exp/simd/go.mod } |  |  | 0.777 |
| ns | 1086 |  | 257 | Non-Go trees: docs/, benchmark/, .github/ listings | 1.8 |  | 0.667 |
| ns | 1264 |  | 178 | Scope caveat, versioning contract, zero dependencies, Go floor | 1.9 |  | 0.641 |
| ns | 1376 |  | 112 | Makefile: every target name | 1.10 |  | 0.617 |
| ns | 1469 |  | 93 | README section skeleton: all H2 headings | 1.11 |  | 0.623 |
| ns | 1609 |  | 140 | Anatomy of a helper: Filter and FilterErr in full | 2.1 |  | 0.614 |
| walker |  | 1984 | 937 | Plaintext::Whole { file: Makefile } |  |  | 0.651 |
| walker |  | 2043 | 59 | Fs::DirListing { dir: docs } |  |  | 0.682 |
| ns | 2049 |  | 440 | slice.go roster 1/2: transform, group, build (L12-L709) | 2.2 | 2.1 | 0.614 |
| walker |  | 2052 | 9 | Fs::DirListing { dir: docs/plugins/helpers-pages } |  |  | 0.614 |
| walker |  | 2072 | 20 | Fs::DirListing { dir: docs/static } |  |  | 0.614 |
| walker |  | 2093 | 21 | Fs::DirListing { dir: docs/src } |  |  | 0.614 |
| walker |  | 2099 | 6 | Fs::DirListing { dir: docs/src/clientModules } |  |  | 0.614 |
| walker |  | 2119 | 20 | Fs::DirListing { dir: docs/src/pages } |  |  | 0.614 |
| walker |  | 2144 | 25 | Fs::DirListing { dir: docs/src/theme } |  |  | 0.614 |
| walker |  | 2149 | 5 | Fs::DirListing { dir: docs/src/theme/ColorModeToggle } |  |  | 0.614 |
| walker |  | 2154 | 5 | Fs::DirListing { dir: docs/src/theme/DocSidebar } |  |  | 0.614 |
| walker |  | 2159 | 5 | Fs::DirListing { dir: docs/src/theme/NotFound } |  |  | 0.614 |
| walker |  | 2169 | 10 | Fs::DirListing { dir: docs/src/theme/CodeBlock/Buttons/CopyButton } |  |  | 0.614 |
| walker |  | 2244 | 75 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.614 |
| walker |  | 2265 | 21 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.614 |
| walker |  | 2311 | 46 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.614 |
| walker |  | 2331 | 20 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.614 |
| walker |  | 2359 | 28 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.620 |
| walker |  | 2377 | 18 | Markdown::Section { file: README.md, section_index: 10, keeps_default_concavity: false } |  |  | 0.657 |
| walker |  | 2414 | 37 | GoMod::File { file: exp/simd/go.mod } |  |  | 0.657 |
| ns | 2416 |  | 367 | slice.go roster 2/2: take/drop, reject, count, trim (L721-L1297) | 2.3 |  | 0.611 |
| walker |  | 2445 | 31 | Fs::DirListing { dir: docs/plugins/helpers-pages/components } |  |  | 0.611 |
| ns | 2577 |  | 161 | find.go roster 1/2: index, find, uniques (L13-L270) | 2.4 |  | 0.594 |
| walker |  | 2598 | 153 | Fs::DirListing { dir: benchmark } |  |  | 0.661 |
| walker |  | 2637 | 39 | Fs::DirListing { dir: docs/docs } |  |  | 0.661 |
| walker |  | 2647 | 10 | Fs::DirListing { dir: docs/docs/mutable } |  |  | 0.661 |
| walker |  | 2657 | 10 | Fs::DirListing { dir: docs/docs/parallel } |  |  | 0.661 |
| walker |  | 2668 | 11 | Fs::DirListing { dir: docs/docs/experimental } |  |  | 0.662 |
| walker |  | 2817 | 149 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.671 |
| walker |  | 2859 | 42 | Fs::DirListing { dir: docs/docs/iter } |  |  | 0.671 |
| ns | 2909 |  | 332 | find.go roster 2/2: min/max, first/last/nth, sampling (L352-L988) | 2.5 |  | 0.634 |
| walker |  | 2927 | 68 | Fs::DirListing { dir: docs/docs/core } |  |  | 0.636 |
| walker |  | 3099 | 172 | Code::CodeKey { rung: Names, file: slice.go, decl: 0, sub: 0, line: 0 } |  |  | 0.639 |
| ns | 3282 |  | 373 | map.go roster: every map helper | 2.6 |  | 0.604 |
| walker |  | 3286 | 187 | Code::CodeKey { rung: Names, file: type_manipulation.go, decl: 0, sub: 0, line: 0 } |  |  | 0.604 |
| walker |  | 3293 | 7 | Code::CodeKey { rung: Body, file: type_manipulation.go, decl: 4, sub: 0, line: 34 } |  |  | 0.604 |
| walker |  | 3301 | 8 | Code::CodeKey { rung: Body, file: type_manipulation.go, decl: 3, sub: 0, line: 28 } |  |  | 0.604 |
| walker |  | 3311 | 10 | Code::CodeKey { rung: Body, file: type_manipulation.go, decl: 2, sub: 0, line: 22 } |  |  | 0.604 |
| ns | 3489 |  | 207 | condition.go complete: Ternary plus the If/Switch builder types | 2.7 |  | 0.591 |
| walker |  | 3553 | 242 | Code::CodeKey { rung: Names, file: type_manipulation.go, decl: 0, sub: 1, line: 0 } |  |  | 0.594 |
| ns | 3720 |  | 231 | type_manipulation.go roster: pointer, zero-value, coalesce | 2.8 |  | 0.607 |
| walker |  | 3730 | 177 | Code::CodeKey { rung: Names, file: find.go, decl: 0, sub: 0, line: 0 } |  |  | 0.612 |
| walker |  | 3882 | 152 | Code::CodeKey { rung: Names, file: find.go, decl: 0, sub: 1, line: 0 } |  |  | 0.621 |
| ns | 3908 |  | 188 | intersect.go roster: set predicates and set algebra | 2.9 |  | 0.606 |
| walker |  | 4034 | 152 | Code::CodeKey { rung: Names, file: find.go, decl: 0, sub: 2, line: 0 } |  |  | 0.615 |
| ns | 4083 |  | 175 | errors.go roster: Must, Try, Validate, and the Assert vars | 2.10 |  | 0.600 |
| walker |  | 4190 | 156 | Code::CodeKey { rung: Names, file: find.go, decl: 0, sub: 3, line: 0 } |  |  | 0.602 |
| ns | 4262 |  | 179 | channel.go roster: dispatcher, buffering, fan-in/fan-out | 2.11 |  | 0.590 |
| walker |  | 4376 | 186 | Code::CodeKey { rung: Names, file: find.go, decl: 0, sub: 4, line: 0 } |  |  | 0.597 |
| ns | 4525 |  | 263 | tuples.go: eleven arity families, head and tail with elision | 2.12 |  | 0.574 |
| walker |  | 4545 | 169 | Code::CodeKey { rung: Names, file: find.go, decl: 0, sub: 5, line: 0 } |  |  | 0.583 |
| ns | 4777 |  | 252 | math.go and string.go rosters | 2.13 |  | 0.567 |
| walker |  | 4829 | 284 | Code::CodeKey { rung: Names, file: find.go, decl: 0, sub: 6, line: 0 } |  |  | 0.593 |
| ns | 4914 |  | 137 | concurrency.go roster: Async, Synchronize, WaitFor | 2.14 |  | 0.585 |
| walker |  | 4977 | 148 | Code::CodeKey { rung: Names, file: condition.go, decl: 0, sub: 0, line: 0 } |  |  | 0.586 |
| ns | 5076 |  | 162 | retry.go roster: attempt, debounce, throttle, transaction | 2.15 |  | 0.577 |
| walker |  | 5161 | 184 | Code::CodeKey { rung: Names, file: slice.go, decl: 0, sub: 1, line: 0 } |  |  | 0.579 |
| ns | 5186 |  | 110 | time.go, func.go and constraints.go: complete small files | 2.16 |  | 0.572 |
| walker |  | 5321 | 160 | Code::CodeKey { rung: Names, file: slice.go, decl: 0, sub: 2, line: 0 } |  |  | 0.575 |
| ns | 5327 |  | 141 | types.go: Entry and the Tuple2..Tuple9 family | 2.17 |  | 0.569 |
| ns | 5390 |  | 63 | Full signatures for the three flagship slice helpers | 2.18 | 2.2 | 0.570 |
| ns | 5495 |  | 105 | it/ package header: the go1.23 build tag and its dependencies | 3.1 |  | 0.563 |
| walker |  | 5501 | 180 | Code::CodeKey { rung: Names, file: slice.go, decl: 0, sub: 3, line: 0 } |  |  | 0.568 |
| walker |  | 5684 | 183 | Code::CodeKey { rung: Names, file: slice.go, decl: 0, sub: 4, line: 0 } |  |  | 0.574 |
| walker |  | 5751 | 67 | Fs::DirListing { dir: docs/scripts } |  |  | 0.575 |
| walker |  | 5786 | 35 | Code::CodeKey { rung: Doc, file: find.go, decl: 44, sub: 0, line: 966 } |  |  | 0.575 |
| walker |  | 5822 | 36 | Code::CodeKey { rung: Doc, file: find.go, decl: 4, sub: 0, line: 56 } |  |  | 0.575 |
| walker |  | 5858 | 36 | Code::CodeKey { rung: Doc, file: find.go, decl: 46, sub: 0, line: 982 } |  |  | 0.575 |
| walker |  | 5894 | 36 | Code::CodeKey { rung: Doc, file: type_manipulation.go, decl: 18, sub: 0, line: 182 } |  |  | 0.575 |
| walker |  | 5930 | 36 | Code::CodeKey { rung: Doc, file: type_manipulation.go, decl: 20, sub: 0, line: 204 } |  |  | 0.575 |
| ns | 5950 |  | 455 | it/seq.go roster 1/2: sequence transforms (L16-L615) | 3.2 |  | 0.552 |
| walker |  | 6092 | 162 | Code::CodeKey { rung: Names, file: slice.go, decl: 0, sub: 5, line: 0 } |  |  | 0.562 |
| walker |  | 6272 | 180 | Code::CodeKey { rung: Names, file: slice.go, decl: 0, sub: 6, line: 0 } |  |  | 0.572 |
| ns | 6285 |  | 335 | it/seq.go roster 2/2: take/drop, count, trim, buffer (L627-L1169) | 3.3 |  | 0.556 |
| walker |  | 6428 | 156 | Code::CodeKey { rung: Names, file: slice.go, decl: 0, sub: 7, line: 0 } |  |  | 0.564 |
| walker |  | 6589 | 161 | Code::CodeKey { rung: Names, file: slice.go, decl: 0, sub: 8, line: 0 } |  |  | 0.572 |
| ns | 6666 |  | 381 | it/find.go roster: the iterator search surface | 3.4 |  | 0.555 |
| walker |  | 6763 | 174 | Code::CodeKey { rung: Names, file: slice.go, decl: 0, sub: 9, line: 0 } |  |  | 0.557 |
| walker |  | 6778 | 15 | Code::CodeKey { rung: Doc, file: slice.go, decl: 50, sub: 0, line: 779 } |  |  | 0.557 |
| walker |  | 6938 | 160 | Code::CodeKey { rung: Names, file: slice.go, decl: 0, sub: 10, line: 0 } |  |  | 0.560 |
| ns | 6969 |  | 303 | it/map.go and it/type_manipulation.go rosters | 3.5 |  | 0.548 |
| walker |  | 7106 | 168 | Code::CodeKey { rung: Names, file: slice.go, decl: 0, sub: 11, line: 0 } |  |  | 0.554 |
| ns | 7273 |  | 304 | it/intersect.go, it/math.go, it/channel.go, it/string.go rosters | 3.6 |  | 0.542 |
| walker |  | 7300 | 194 | Code::CodeKey { rung: Names, file: slice.go, decl: 0, sub: 12, line: 0 } |  |  | 0.551 |
| walker |  | 7319 | 19 | Code::CodeKey { rung: Doc, file: slice.go, decl: 70, sub: 0, line: 1140 } |  |  | 0.551 |
| ns | 7363 |  | 90 | it/tuples.go: four zip/cross-join families with elision | 3.7 |  | 0.546 |
| ns | 7489 |  | 126 | mutable/ and parallel/: both packages in full | 3.8 |  | 0.542 |
| walker |  | 7570 | 251 | Code::CodeKey { rung: Names, file: slice.go, decl: 0, sub: 13, line: 0 } |  |  | 0.558 |
| ns | 7603 |  | 114 | internal/constraints: the numeric constraint set | 3.9 |  | 0.553 |
| walker |  | 7606 | 36 | Code::CodeKey { rung: Doc, file: slice.go, decl: 36, sub: 0, line: 591 } |  |  | 0.553 |
| walker |  | 7643 | 37 | Code::CodeKey { rung: Doc, file: condition.go, decl: 3, sub: 0, line: 33 } |  |  | 0.553 |
| walker |  | 7680 | 37 | Code::CodeKey { rung: Doc, file: condition.go, decl: 5, sub: 0, line: 105 } |  |  | 0.553 |
| walker |  | 7717 | 37 | Code::CodeKey { rung: Doc, file: slice.go, decl: 4, sub: 0, line: 57 } |  |  | 0.553 |
| walker |  | 7754 | 37 | Code::CodeKey { rung: Doc, file: type_manipulation.go, decl: 4, sub: 0, line: 34 } |  |  | 0.553 |
| walker |  | 7792 | 38 | Code::CodeKey { rung: Doc, file: find.go, decl: 3, sub: 0, line: 40 } |  |  | 0.553 |
| walker |  | 7830 | 38 | Code::CodeKey { rung: Doc, file: slice.go, decl: 12, sub: 0, line: 177 } |  |  | 0.553 |
| ns | 7835 |  | 232 | internal/xtime: the swappable Clock, and internal/xrand | 3.10 |  | 0.546 |
| walker |  | 7868 | 38 | Code::CodeKey { rung: Doc, file: slice.go, decl: 46, sub: 0, line: 721 } |  |  | 0.546 |
| ns | 7885 |  | 50 | TestMain: goleak plus the fake clock | 4.1 |  | 0.544 |
| walker |  | 7906 | 38 | Code::CodeKey { rung: Doc, file: type_manipulation.go, decl: 3, sub: 0, line: 28 } |  |  | 0.544 |
| ns | 7938 |  | 53 | Unit-test conventions: testify, t.Parallel, `is := assert.New(t)` | 4.2 |  | 0.542 |
| walker |  | 7944 | 38 | Code::CodeKey { rung: Doc, file: type_manipulation.go, decl: 19, sub: 0, line: 193 } |  |  | 0.542 |
| walker |  | 7982 | 38 | Code::CodeKey { rung: Doc, file: type_manipulation.go, decl: 21, sub: 0, line: 215 } |  |  | 0.542 |
| ns | 8001 |  | 63 | Example tests: the godoc `// Output:` convention | 4.3 |  | 0.540 |
| walker |  | 8021 | 39 | Code::CodeKey { rung: Doc, file: find.go, decl: 10, sub: 0, line: 148 } |  |  | 0.540 |
| walker |  | 8060 | 39 | Code::CodeKey { rung: Doc, file: slice.go, decl: 34, sub: 0, line: 567 } |  |  | 0.540 |
| walker |  | 8099 | 39 | Code::CodeKey { rung: Doc, file: slice.go, decl: 68, sub: 0, line: 1113 } |  |  | 0.540 |
| ns | 8118 |  | 117 | benchmark/: shared generators and the parametric bench shape | 4.4 |  | 0.536 |
| walker |  | 8138 | 39 | Code::CodeKey { rung: Doc, file: slice.go, decl: 69, sub: 0, line: 1129 } |  |  | 0.536 |
| walker |  | 8177 | 39 | Code::CodeKey { rung: Doc, file: type_manipulation.go, decl: 6, sub: 0, line: 53 } |  |  | 0.536 |
| walker |  | 8217 | 40 | Code::CodeKey { rung: Doc, file: slice.go, decl: 33, sub: 0, line: 555 } |  |  | 0.536 |
| walker |  | 8257 | 40 | Code::CodeKey { rung: Doc, file: slice.go, decl: 47, sub: 0, line: 737 } |  |  | 0.536 |
| ns | 8269 |  | 151 | exp/simd: what it requires and why it is a separate module | 5.1 |  | 0.534 |
| walker |  | 8297 | 40 | Code::CodeKey { rung: Doc, file: slice.go, decl: 58, sub: 0, line: 958 } |  |  | 0.534 |
| walker |  | 8337 | 40 | Code::CodeKey { rung: Doc, file: slice.go, decl: 67, sub: 0, line: 1098 } |  |  | 0.534 |
| walker |  | 8377 | 40 | Code::CodeKey { rung: Doc, file: slice.go, decl: 80, sub: 0, line: 1297 } |  |  | 0.534 |
| walker |  | 8417 | 40 | Code::CodeKey { rung: Doc, file: type_manipulation.go, decl: 14, sub: 0, line: 147 } |  |  | 0.534 |
| ns | 8432 |  | 163 | exp/simd/math.go: the seven dispatcher families with elision | 5.2 |  | 0.526 |
| walker |  | 8457 | 40 | Code::CodeKey { rung: Doc, file: type_manipulation.go, decl: 16, sub: 0, line: 161 } |  |  | 0.526 |
| walker |  | 8498 | 41 | Code::CodeKey { rung: Doc, file: find.go, decl: 38, sub: 0, line: 888 } |  |  | 0.526 |
| walker |  | 8539 | 41 | Code::CodeKey { rung: Doc, file: slice.go, decl: 13, sub: 0, line: 192 } |  |  | 0.526 |
| walker |  | 8580 | 41 | Code::CodeKey { rung: Doc, file: slice.go, decl: 35, sub: 0, line: 579 } |  |  | 0.526 |
| walker |  | 8621 | 41 | Code::CodeKey { rung: Doc, file: slice.go, decl: 61, sub: 0, line: 1005 } |  |  | 0.526 |
| walker |  | 8662 | 41 | Code::CodeKey { rung: Doc, file: slice.go, decl: 71, sub: 0, line: 1155 } |  |  | 0.526 |
| ns | 8669 |  | 237 | exp/simd: runtime feature detection and the fallback path | 5.3 | 5.2 | 0.519 |
| walker |  | 8703 | 41 | Code::CodeKey { rung: Doc, file: slice.go, decl: 77, sub: 0, line: 1261 } |  |  | 0.519 |
| walker |  | 8744 | 41 | Code::CodeKey { rung: Doc, file: slice.go, decl: 78, sub: 0, line: 1272 } |  |  | 0.519 |
| walker |  | 8785 | 41 | Code::CodeKey { rung: Doc, file: type_manipulation.go, decl: 8, sub: 0, line: 73 } |  |  | 0.519 |
| ns | 8787 |  | 118 | exp/simd width-specific files: the <Op><Type>x<Lanes> naming rule | 5.4 |  | 0.515 |
| ns | 8887 |  | 100 | docs/ npm scripts: every documentation checker | 6.1 |  | 0.512 |
| walker |  | 8989 | 204 | Code::CodeKey { rung: Names, file: errors.go, decl: 0, sub: 0, line: 0 } |  |  | 0.513 |
| ns | 8993 |  | 106 | docs/scripts and docs/docs listings | 6.2 |  | 0.521 |
| ns | 9134 |  | 141 | docs/docs category pages and the sidebar | 6.3 |  | 0.533 |
| walker |  | 9228 | 239 | Code::CodeKey { rung: Decl, file: errors.go, decl: 2, sub: 0, line: 34 } |  |  | 0.533 |
| walker |  | 9244 | 16 | Code::CodeKey { rung: Doc, file: errors.go, decl: 2, sub: 0, line: 34 } |  |  | 0.533 |
| ns | 9280 |  | 146 | Makefile recipes: race tests and the SIMD guard | 7.1 | 1.10 | 0.535 |
| walker |  | 9423 | 179 | Code::CodeKey { rung: Names, file: intersect.go, decl: 0, sub: 0, line: 0 } |  |  | 0.537 |
| walker |  | 9592 | 169 | Code::CodeKey { rung: Names, file: intersect.go, decl: 0, sub: 1, line: 0 } |  |  | 0.543 |
| ns | 9606 |  | 326 | golangci-lint: the enabled linter set | 7.2 |  | 0.532 |
| walker |  | 9746 | 154 | Code::CodeKey { rung: Names, file: intersect.go, decl: 0, sub: 2, line: 0 } |  |  | 0.539 |
| walker |  | 9754 | 8 | Code::CodeKey { rung: Body, file: intersect.go, decl: 16, sub: 0, line: 308 } |  |  | 0.539 |
| ns | 9807 |  | 201 | golangci-lint: thresholds and exclusions | 7.3 |  | 0.534 |
| walker |  | 9908 | 154 | Code::CodeKey { rung: Names, file: retry.go, decl: 0, sub: 0, line: 0 } |  |  | 0.535 |
| ns | 9975 |  | 168 | CI: the Go version matrix | 7.4 |  | 0.530 |
| walker |  | 9978 | 70 | Code::CodeKey { rung: Names, file: retry.go, decl: 0, sub: 1, line: 0 } |  |  | 0.532 |
| ns | 9990 |  | 15 | Contribution surface: PR templates and the contributor loop | 7.5 |  | 0.533 |
