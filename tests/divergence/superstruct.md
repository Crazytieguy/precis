Score(3000)=0.528 I=0.795 C=0.351 ns_rows≤3K=20/60 grid(1000/1442/2080/3000/4327/6240/9000)=0.686/0.608/0.539/0.528/0.531/0.626/0.583

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 65 | 65 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 69 |  | 69 | Package identity: name, description, version, license | 1.1 |  | 0.000 |
| walker |  | 84 | 19 | Fs::DirListing { dir: src } |  |  | 0.000 |
| walker |  | 102 | 18 | Fs::DirListing { dir: src/structs } |  |  | 0.000 |
| walker |  | 123 | 21 | Fs::DirListing { dir: docs } |  |  | 0.000 |
| walker |  | 131 | 8 | Fs::DirListing { dir: docs/resources } |  |  | 0.000 |
| ns | 134 |  | 65 | Complete root directory listing | 1.2 |  | 0.616 |
| walker |  | 135 | 4 | Fs::DirListing { dir: .vscode } |  |  | 0.616 |
| walker |  | 143 | 8 | Fs::DirListing { dir: .github } |  |  | 0.616 |
| walker |  | 147 | 4 | Fs::DirListing { dir: .github/workflows } |  |  | 0.616 |
| ns | 171 |  | 37 | Complete src/ and src/structs/ listings | 1.3 |  | 0.613 |
| walker |  | 229 | 82 | Json::Identity { file: package.json } |  |  | 0.959 |
| ns | 250 |  | 79 | src/index.ts - the entire public barrel | 1.4 |  | 0.865 |
| walker |  | 258 | 29 | Fs::DirListing { dir: docs/images } |  |  | 0.866 |
| walker |  | 289 | 31 | Fs::DirListing { dir: docs/reference } |  |  | 0.868 |
| walker |  | 323 | 34 | Json::Runtime { file: package.json } |  |  | 0.869 |
| walker |  | 374 | 51 | Fs::DirListing { dir: docs/guides } |  |  | 0.871 |
| ns | 416 |  | 166 | Readme lede: what Superstruct is and why it exists | 1.5 |  | 0.845 |
| walker |  | 571 | 197 | Markdown::ReadmeHeadline { file: Readme.md } |  |  | 0.854 |
| walker |  | 637 | 66 | Markdown::Prelude { file: Readme.md } |  |  | 0.876 |
| ns | 638 |  | 222 | The Struct class: doc comment and its six fields | 1.6 |  | 0.724 |
| walker |  | 691 | 54 | Markdown::HeadingsOutline { file: Readme.md } |  |  | 0.725 |
| walker |  | 719 | 28 | Fs::DirListing { dir: test } |  |  | 0.726 |
| walker |  | 795 | 76 | Json::Entry { file: package.json } |  |  | 0.727 |
| ns | 816 |  | 178 | Core API signatures: assert / create / is / mask / validate | 1.7 |  | 0.658 |
| ns | 971 |  | 155 | Readme canonical usage snippet | 1.8 |  | 0.577 |
| walker |  | 1028 | 233 | Markdown::Section { file: Readme.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.719 |
| ns | 1066 |  | 95 | src/struct.ts symbol roster: methods and top-level helpers | 2.1 |  | 0.671 |
| walker |  | 1083 | 55 | Fs::DirListing { dir: examples } |  |  | 0.673 |
| walker |  | 1108 | 25 | Fs::DirListing { dir: test/api } |  |  | 0.674 |
| walker |  | 1187 | 79 | Code::CodeKey { rung: Names, file: src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.727 |
| walker |  | 1312 | 125 | Fs::DirListing { dir: test/validation } |  |  | 0.734 |
| ns | 1318 |  | 252 | Complete roster of the 25 type structs | 2.2 |  | 0.636 |
| ns | 1420 |  | 102 | Complete roster of src/structs/utilities.ts | 2.3 |  | 0.606 |
| walker |  | 1495 | 183 | Fs::DirListing { dir: test/typings } |  |  | 0.611 |
| ns | 1503 |  | 83 | Complete roster of src/structs/refinements.ts | 2.4 |  | 0.589 |
| ns | 1537 |  | 34 | Complete roster of src/structs/coercions.ts | 2.5 |  | 0.582 |
| walker |  | 1606 | 111 | Markdown::Section { file: Readme.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.582 |
| ns | 1651 |  | 114 | Failure - the shape of every validation failure | 2.6 |  | 0.556 |
| ns | 1829 |  | 178 | StructError class: doc, fields, and its early-exit contract | 2.7 |  | 0.526 |
| walker |  | 1932 | 326 | Json::Scripts { file: package.json } |  |  | 0.527 |
| walker |  | 1959 | 27 | Code::CodeKey { rung: Names, file: src/error.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.528 |
| ns | 2037 |  | 208 | Exported type vocabulary of src/struct.ts | 2.8 |  | 0.501 |
| walker |  | 2040 | 81 | Code::CodeKey { rung: Decl, file: src/error.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.534 |
| ns | 2140 |  | 103 | Function roster of src/utils.ts | 2.9 |  | 0.519 |
| walker |  | 2153 | 113 | Code::CodeKey { rung: Decl, file: src/error.ts, decl: 2, sub: 0, line: 25 } |  |  | 0.536 |
| walker |  | 2179 | 26 | Code::CodeKey { rung: Doc, file: src/error.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.554 |
| walker |  | 2355 | 176 | Markdown::Section { file: Readme.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.554 |
| ns | 2378 |  | 238 | Type-level utility roster of src/utils.ts | 2.10 |  | 0.521 |
| walker |  | 2550 | 195 | Markdown::Section { file: Readme.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.521 |
| ns | 2574 |  | 196 | What each type struct does, part 1 (any ... literal) | 3.1 | 2.2 | 0.505 |
| walker |  | 2685 | 135 | Markdown::Section { file: Readme.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.505 |
| ns | 2916 |  | 342 | What each type struct does, part 2 (map ... unknown) | 3.2 | 2.2 | 0.481 |
| walker |  | 2918 | 233 | Code::CodeKey { rung: Names, file: src/struct.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.514 |
| walker |  | 2952 | 34 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 13, sub: 0, line: 221 } |  |  | 0.528 |
| walker |  | 2989 | 37 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 9, sub: 0, line: 139 } |  |  | 0.528 |
| walker |  | 3026 | 37 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 10, sub: 0, line: 157 } |  |  | 0.528 |
| walker |  | 3066 | 40 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 8, sub: 0, line: 123 } |  |  | 0.528 |
| walker |  | 3107 | 41 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 16, sub: 0, line: 243 } |  |  | 0.549 |
| ns | 3144 |  | 228 | What each utility struct does | 3.3 | 2.3 | 0.530 |
| walker |  | 3190 | 83 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 12, sub: 0, line: 185 } |  |  | 0.530 |
| walker |  | 3213 | 23 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 11, sub: 0, line: 175 } |  |  | 0.530 |
| walker |  | 3236 | 23 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 16, sub: 0, line: 243 } |  |  | 0.530 |
| walker |  | 3261 | 25 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 18, sub: 0, line: 259 } |  |  | 0.530 |
| walker |  | 3288 | 27 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 14, sub: 0, line: 231 } |  |  | 0.530 |
| ns | 3300 |  | 156 | What each refinement does | 3.4 | 2.4 | 0.521 |
| walker |  | 3315 | 27 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 15, sub: 0, line: 237 } |  |  | 0.521 |
| walker |  | 3343 | 28 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 8, sub: 0, line: 123 } |  |  | 0.521 |
| walker |  | 3371 | 28 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 17, sub: 0, line: 253 } |  |  | 0.521 |
| walker |  | 3400 | 29 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 9, sub: 0, line: 139 } |  |  | 0.521 |
| ns | 3411 |  | 111 | What each coercion does - and the create() gotcha | 3.5 | 2.5 | 0.516 |
| walker |  | 3430 | 30 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 10, sub: 0, line: 157 } |  |  | 0.516 |
| walker |  | 3469 | 39 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 19, sub: 0, line: 266 } |  |  | 0.516 |
| walker |  | 3711 | 242 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 1, sub: 0, line: 10 } |  |  | 0.568 |
| ns | 3716 |  | 305 | Public overload declarations of the six overloaded factories | 3.6 | 2.2 | 0.541 |
| walker |  | 3783 | 72 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 2, sub: 0, line: 22 } |  |  | 0.541 |
| walker |  | 3855 | 72 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 7, sub: 0, line: 107 } |  |  | 0.541 |
| walker |  | 3883 | 28 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 5, sub: 0, line: 83 } |  |  | 0.541 |
| walker |  | 3916 | 33 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 3, sub: 0, line: 67 } |  |  | 0.541 |
| walker |  | 3949 | 33 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 4, sub: 0, line: 75 } |  |  | 0.541 |
| ns | 3959 |  | 243 | Struct method semantics: mask recursion and validate options | 3.7 | 2.1 | 0.527 |
| walker |  | 3995 | 46 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 12, sub: 0, line: 185 } |  |  | 0.527 |
| walker |  | 4064 | 69 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 1, sub: 0, line: 10 } |  |  | 0.553 |
| walker |  | 4133 | 69 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 6, sub: 0, line: 93 } |  |  | 0.555 |
| ns | 4149 |  | 190 | What each src/utils.ts helper does | 3.8 | 2.9 | 0.544 |
| ns | 4298 |  | 149 | run() signature and per-call context setup | 4.1 | 2.9 | 0.531 |
| walker |  | 4415 | 282 | Markdown::Section { file: Readme.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.531 |
| ns | 4427 |  | 129 | run() stage 1: coercion, then the struct's own validator | 4.2 | 4.1 | 0.520 |
| walker |  | 4653 | 238 | Code::CodeKey { rung: Names, file: src/utils.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.541 |
| walker |  | 4673 | 20 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 11, sub: 0, line: 218 } |  |  | 0.541 |
| walker |  | 4723 | 50 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 7, sub: 0, line: 106 } |  |  | 0.541 |
| ns | 4759 |  | 332 | run() stage 2: recursive descent over entries | 4.3 | 4.1 | 0.517 |
| walker |  | 4773 | 50 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 9, sub: 0, line: 202 } |  |  | 0.517 |
| walker |  | 4827 | 54 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 6, sub: 0, line: 67 } |  |  | 0.517 |
| ns | 4882 |  | 123 | run() stage 3: refiners and the success yield | 4.4 | 4.1 | 0.507 |
| walker |  | 4929 | 102 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 8, sub: 0, line: 130 } |  |  | 0.523 |
| walker |  | 4948 | 19 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 11, sub: 0, line: 218 } |  |  | 0.523 |
| walker |  | 4971 | 23 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 4, sub: 0, line: 45 } |  |  | 0.523 |
| walker |  | 4995 | 24 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 1, sub: 0, line: 16 } |  |  | 0.524 |
| walker |  | 5019 | 24 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 3, sub: 0, line: 32 } |  |  | 0.524 |
| walker |  | 5043 | 24 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 6, sub: 0, line: 67 } |  |  | 0.526 |
| ns | 5054 |  | 172 | validate() body: how one failure becomes a StructError | 4.5 | 2.1 | 0.513 |
| walker |  | 5067 | 24 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 9, sub: 0, line: 202 } |  |  | 0.513 |
| walker |  | 5092 | 25 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 2, sub: 0, line: 24 } |  |  | 0.514 |
| walker |  | 5117 | 25 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 7, sub: 0, line: 106 } |  |  | 0.516 |
| walker |  | 5142 | 25 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 10, sub: 0, line: 212 } |  |  | 0.516 |
| walker |  | 5178 | 36 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 12, sub: 0, line: 227 } |  |  | 0.516 |
| walker |  | 5222 | 44 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 8, sub: 0, line: 130 } |  |  | 0.522 |
| ns | 5265 |  | 211 | Struct constructor: defaults and failure wrapping | 4.6 | 1.6 | 0.505 |
| walker |  | 5272 | 50 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 5, sub: 0, line: 58 } |  |  | 0.513 |
| ns | 5456 |  | 191 | StructError constructor: message, cause and failure caching | 4.7 | 2.7 | 0.504 |
| walker |  | 5581 | 309 | Code::CodeKey { rung: Names, file: src/utils.ts, decl: 0, sub: 1, line: 0 } |  |  | 0.541 |
| walker |  | 5594 | 13 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 20, sub: 0, line: 300 } |  |  | 0.541 |
| ns | 5596 |  | 140 | Complete docs/ tree listing | 5.1 |  | 0.566 |
| walker |  | 5618 | 24 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 18, sub: 0, line: 283 } |  |  | 0.566 |
| walker |  | 5643 | 25 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 21, sub: 0, line: 307 } |  |  | 0.566 |
| ns | 5649 |  | 53 | Complete test/ and test/api/ listings | 5.2 |  | 0.576 |
| walker |  | 5668 | 25 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 23, sub: 0, line: 324 } |  |  | 0.576 |
| walker |  | 5703 | 35 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 14, sub: 0, line: 242 } |  |  | 0.576 |
| walker |  | 5745 | 42 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 19, sub: 0, line: 291 } |  |  | 0.576 |
| ns | 5774 |  | 125 | All 41 validation-fixture kind directories | 5.3 |  | 0.604 |
| walker |  | 5787 | 42 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 22, sub: 0, line: 315 } |  |  | 0.604 |
| ns | 5829 |  | 55 | Complete examples/ listing | 5.4 |  | 0.611 |
| walker |  | 5832 | 45 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 16, sub: 0, line: 267 } |  |  | 0.611 |
| walker |  | 5882 | 50 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 13, sub: 0, line: 233 } |  |  | 0.611 |
| ns | 5906 |  | 77 | One kind directory listed in full, as the case-naming exemplar | 5.5 |  | 0.602 |
| walker |  | 5956 | 74 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 28, sub: 0, line: 394 } |  |  | 0.602 |
| walker |  | 6068 | 112 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 15, sub: 0, line: 251 } |  |  | 0.602 |
| walker |  | 6087 | 19 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 17, sub: 0, line: 277 } |  |  | 0.602 |
| ns | 6089 |  | 183 | The 45 type-level test files | 5.6 |  | 0.626 |
| walker |  | 6108 | 21 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 15, sub: 0, line: 251 } |  |  | 0.626 |
| walker |  | 6129 | 21 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 16, sub: 0, line: 267 } |  |  | 0.626 |
| walker |  | 6150 | 21 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 26, sub: 0, line: 379 } |  |  | 0.626 |
| walker |  | 6172 | 22 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 13, sub: 0, line: 233 } |  |  | 0.626 |
| walker |  | 6194 | 22 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 14, sub: 0, line: 242 } |  |  | 0.626 |
| walker |  | 6216 | 22 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 18, sub: 0, line: 283 } |  |  | 0.626 |
| walker |  | 6238 | 22 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 27, sub: 0, line: 385 } |  |  | 0.626 |
| walker |  | 6261 | 23 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 21, sub: 0, line: 307 } |  |  | 0.626 |
| walker |  | 6285 | 24 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 23, sub: 0, line: 324 } |  |  | 0.626 |
| walker |  | 6310 | 25 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 22, sub: 0, line: 315 } |  |  | 0.626 |
| ns | 6316 |  | 227 | Every heading in the six guides | 5.7 |  | 0.613 |
| walker |  | 6336 | 26 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 19, sub: 0, line: 291 } |  |  | 0.613 |
| walker |  | 6364 | 28 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 20, sub: 0, line: 300 } |  |  | 0.613 |
| ns | 6379 |  | 63 | Readme section map | 5.8 |  | 0.616 |
| walker |  | 6427 | 63 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 28, sub: 0, line: 394 } |  |  | 0.616 |
| ns | 6519 |  | 140 | Doc anchors the source rosters do not cover | 5.9 |  | 0.610 |
| ns | 6690 |  | 171 | The data-driven fixture runner | 5.10 |  | 0.604 |
| walker |  | 6849 | 422 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 25, sub: 0, line: 334 } |  |  | 0.604 |
| walker |  | 6870 | 21 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 25, sub: 0, line: 334 } |  |  | 0.604 |
| ns | 6891 |  | 201 | Runner assertions: output-style vs failures-style fixtures | 5.11 | 5.10 | 0.595 |
| walker |  | 6967 | 97 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 13, sub: 0, line: 221 } |  |  | 0.600 |
| walker |  | 7074 | 107 | Code::CodeKey { rung: Doc, file: src/error.ts, decl: 2, sub: 0, line: 25 } |  |  | 0.614 |
| ns | 7164 |  | 273 | A failures-style and an output-style fixture, in full | 5.12 | 5.10 | 0.598 |
| walker |  | 7280 | 206 | Markdown::Section { file: Readme.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.598 |
| ns | 7295 |  | 131 | The typings harness and one typings test | 5.13 | 5.6 | 0.592 |
| walker |  | 7381 | 101 | Code::CodeKey { rung: Names, file: src/structs/refinements.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.600 |
| walker |  | 7414 | 33 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 5, sub: 0, line: 93 } |  |  | 0.600 |
| walker |  | 7458 | 44 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 7, sub: 0, line: 146 } |  |  | 0.600 |
| ns | 7481 |  | 186 | toFailure(): result normalisation and default message text | 6.1 | 2.9 | 0.592 |
| walker |  | 7506 | 48 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 1, sub: 0, line: 8 } |  |  | 0.592 |
| walker |  | 7554 | 48 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 4, sub: 0, line: 77 } |  |  | 0.592 |
| walker |  | 7611 | 57 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 2, sub: 0, line: 33 } |  |  | 0.592 |
| walker |  | 7668 | 57 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 3, sub: 0, line: 55 } |  |  | 0.592 |
| walker |  | 7730 | 62 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 6, sub: 0, line: 109 } |  |  | 0.592 |
| walker |  | 7754 | 24 | Code::CodeKey { rung: Doc, file: src/structs/refinements.ts, decl: 5, sub: 0, line: 93 } |  |  | 0.592 |
| walker |  | 7780 | 26 | Code::CodeKey { rung: Doc, file: src/structs/refinements.ts, decl: 2, sub: 0, line: 33 } |  |  | 0.592 |
| walker |  | 7806 | 26 | Code::CodeKey { rung: Doc, file: src/structs/refinements.ts, decl: 3, sub: 0, line: 55 } |  |  | 0.593 |
| walker |  | 7835 | 29 | Code::CodeKey { rung: Doc, file: src/structs/refinements.ts, decl: 1, sub: 0, line: 8 } |  |  | 0.594 |
| walker |  | 7864 | 29 | Code::CodeKey { rung: Doc, file: src/structs/refinements.ts, decl: 4, sub: 0, line: 77 } |  |  | 0.595 |
| ns | 7894 |  | 413 | object() implementation, including the mask special case | 6.2 | 3.6 | 0.577 |
| walker |  | 7912 | 48 | Code::CodeKey { rung: Doc, file: src/structs/refinements.ts, decl: 6, sub: 0, line: 109 } |  |  | 0.579 |
| walker |  | 8047 | 135 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 7, sub: 0, line: 107 } |  |  | 0.588 |
| ns | 8085 |  | 191 | refine() and define() bodies - the extension points | 6.3 | 2.4 | 0.581 |
| ns | 8199 |  | 114 | coerce() and trimmed() bodies | 6.4 | 2.5 | 0.576 |
| walker |  | 8267 | 220 | Code::CodeKey { rung: Names, file: src/structs/utilities.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.589 |
| walker |  | 8300 | 33 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 8, sub: 0, line: 106 } |  |  | 0.589 |
| walker |  | 8337 | 37 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 11, sub: 0, line: 197 } |  |  | 0.589 |
| walker |  | 8375 | 38 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 7, sub: 0, line: 80 } |  |  | 0.589 |
| walker |  | 8418 | 43 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 12, sub: 0, line: 221 } |  |  | 0.589 |
| ns | 8458 |  | 259 | npm scripts | 7.1 | 1.1 | 0.595 |
| walker |  | 8462 | 44 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 10, sub: 0, line: 171 } |  |  | 0.595 |
| walker |  | 8507 | 45 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 1, sub: 0, line: 17 } |  |  | 0.595 |
| walker |  | 8609 | 102 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 2, sub: 0, line: 21 } |  |  | 0.595 |
| ns | 8629 |  | 171 | Changelog: recent release headings and the file's extent | 7.2 |  | 0.591 |
| walker |  | 8759 | 150 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 3, sub: 0, line: 30 } |  |  | 0.591 |
| ns | 8778 |  | 149 | The 2.0.0 release notes: breaking changes and fixes | 7.3 | 7.2 | 0.589 |
| walker |  | 8785 | 26 | Code::CodeKey { rung: Doc, file: src/structs/utilities.ts, decl: 6, sub: 0, line: 71 } |  |  | 0.589 |
| ns | 8957 |  | 179 | TypeScript compiler configuration | 7.4 |  | 0.583 |
| walker |  | 8967 | 182 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 4, sub: 0, line: 44 } |  |  | 0.583 |
| walker |  | 9014 | 47 | Code::CodeKey { rung: Doc, file: src/structs/utilities.ts, decl: 13, sub: 0, line: 247 } |  |  | 0.583 |
| walker |  | 9069 | 55 | Code::CodeKey { rung: Doc, file: src/structs/utilities.ts, decl: 7, sub: 0, line: 80 } |  |  | 0.584 |
| walker |  | 9126 | 57 | Code::CodeKey { rung: Doc, file: src/structs/utilities.ts, decl: 10, sub: 0, line: 171 } |  |  | 0.585 |
| walker |  | 9183 | 57 | Code::CodeKey { rung: Doc, file: src/structs/utilities.ts, decl: 12, sub: 0, line: 221 } |  |  | 0.588 |
| ns | 9207 |  | 250 | Rollup build and JSR publish config | 7.5 |  | 0.578 |
| walker |  | 9246 | 63 | Code::CodeKey { rung: Doc, file: src/structs/utilities.ts, decl: 11, sub: 0, line: 197 } |  |  | 0.581 |
| walker |  | 9314 | 68 | Code::CodeKey { rung: Doc, file: src/structs/utilities.ts, decl: 1, sub: 0, line: 17 } |  |  | 0.585 |
| ns | 9368 |  | 161 | CI matrix and dependency automation | 7.6 |  | 0.580 |
| walker |  | 9395 | 81 | Code::CodeKey { rung: Doc, file: src/structs/utilities.ts, decl: 8, sub: 0, line: 106 } |  |  | 0.581 |
| ns | 9521 |  | 153 | Remaining package.json keys: entry points and publish metadata | 7.7 | 1.1 | 0.581 |
| ns | 9599 |  | 78 | Formatting and docs-site configuration | 7.8 |  | 0.579 |
| ns | 9718 |  | 119 | ESLint configuration head | 7.9 |  | 0.575 |
| ns | 9847 |  | 129 | Examples package manifest and run instructions | 7.10 |  | 0.572 |
| walker |  | 9862 | 467 | Markdown::Section { file: Readme.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.572 |
| walker |  | 9982 | 120 | Markdown::Section { file: Readme.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.572 |
