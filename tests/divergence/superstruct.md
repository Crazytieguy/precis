Score(3000)=0.528 I=0.795 C=0.351 ns_rows≤3K=20/60 grid(1000/1442/2080/3000/4327/6240/9000)=0.647/0.608/0.535/0.528/0.531/0.626/0.583

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
| walker |  | 710 | 19 | Markdown::Section { file: Readme.md, section_index: 10, keeps_default_concavity: false } |  |  | 0.725 |
| walker |  | 738 | 28 | Fs::DirListing { dir: test } |  |  | 0.726 |
| walker |  | 814 | 76 | Json::Entry { file: package.json } |  |  | 0.727 |
| ns | 816 |  | 178 | Core API signatures: assert / create / is / mask / validate | 1.7 |  | 0.658 |
| ns | 971 |  | 155 | Readme canonical usage snippet | 1.8 |  | 0.577 |
| walker |  | 1047 | 233 | Markdown::Section { file: Readme.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.719 |
| ns | 1066 |  | 95 | src/struct.ts symbol roster: methods and top-level helpers | 2.1 |  | 0.671 |
| walker |  | 1102 | 55 | Fs::DirListing { dir: examples } |  |  | 0.673 |
| walker |  | 1127 | 25 | Fs::DirListing { dir: test/api } |  |  | 0.674 |
| walker |  | 1206 | 79 | Code::CodeKey { rung: Names, file: src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.727 |
| ns | 1318 |  | 252 | Complete roster of the 25 type structs | 2.2 |  | 0.630 |
| walker |  | 1331 | 125 | Fs::DirListing { dir: test/validation } |  |  | 0.636 |
| ns | 1420 |  | 102 | Complete roster of src/structs/utilities.ts | 2.3 |  | 0.606 |
| ns | 1503 |  | 83 | Complete roster of src/structs/refinements.ts | 2.4 |  | 0.584 |
| walker |  | 1514 | 183 | Fs::DirListing { dir: test/typings } |  |  | 0.589 |
| ns | 1537 |  | 34 | Complete roster of src/structs/coercions.ts | 2.5 |  | 0.582 |
| walker |  | 1625 | 111 | Markdown::Section { file: Readme.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.582 |
| ns | 1651 |  | 114 | Failure - the shape of every validation failure | 2.6 |  | 0.556 |
| ns | 1829 |  | 178 | StructError class: doc, fields, and its early-exit contract | 2.7 |  | 0.526 |
| walker |  | 1951 | 326 | Json::Scripts { file: package.json } |  |  | 0.527 |
| walker |  | 1978 | 27 | Code::CodeKey { rung: Names, file: src/error.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.528 |
| ns | 2037 |  | 208 | Exported type vocabulary of src/struct.ts | 2.8 |  | 0.501 |
| walker |  | 2059 | 81 | Code::CodeKey { rung: Decl, file: src/error.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.534 |
| ns | 2140 |  | 103 | Function roster of src/utils.ts | 2.9 |  | 0.519 |
| walker |  | 2172 | 113 | Code::CodeKey { rung: Decl, file: src/error.ts, decl: 2, sub: 0, line: 25 } |  |  | 0.536 |
| walker |  | 2198 | 26 | Code::CodeKey { rung: Doc, file: src/error.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.554 |
| walker |  | 2374 | 176 | Markdown::Section { file: Readme.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.554 |
| ns | 2378 |  | 238 | Type-level utility roster of src/utils.ts | 2.10 |  | 0.521 |
| walker |  | 2569 | 195 | Markdown::Section { file: Readme.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.521 |
| ns | 2574 |  | 196 | What each type struct does, part 1 (any ... literal) | 3.1 | 2.2 | 0.505 |
| walker |  | 2704 | 135 | Markdown::Section { file: Readme.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.505 |
| ns | 2916 |  | 342 | What each type struct does, part 2 (map ... unknown) | 3.2 | 2.2 | 0.481 |
| walker |  | 2937 | 233 | Code::CodeKey { rung: Names, file: src/struct.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.514 |
| walker |  | 2971 | 34 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 13, sub: 0, line: 221 } |  |  | 0.528 |
| walker |  | 3008 | 37 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 9, sub: 0, line: 139 } |  |  | 0.528 |
| walker |  | 3045 | 37 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 10, sub: 0, line: 157 } |  |  | 0.528 |
| walker |  | 3085 | 40 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 8, sub: 0, line: 123 } |  |  | 0.528 |
| walker |  | 3126 | 41 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 16, sub: 0, line: 243 } |  |  | 0.549 |
| ns | 3144 |  | 228 | What each utility struct does | 3.3 | 2.3 | 0.530 |
| walker |  | 3209 | 83 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 12, sub: 0, line: 185 } |  |  | 0.530 |
| walker |  | 3232 | 23 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 11, sub: 0, line: 175 } |  |  | 0.530 |
| walker |  | 3255 | 23 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 16, sub: 0, line: 243 } |  |  | 0.530 |
| walker |  | 3280 | 25 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 18, sub: 0, line: 259 } |  |  | 0.530 |
| ns | 3300 |  | 156 | What each refinement does | 3.4 | 2.4 | 0.521 |
| walker |  | 3307 | 27 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 14, sub: 0, line: 231 } |  |  | 0.521 |
| walker |  | 3334 | 27 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 15, sub: 0, line: 237 } |  |  | 0.521 |
| walker |  | 3362 | 28 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 8, sub: 0, line: 123 } |  |  | 0.521 |
| walker |  | 3390 | 28 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 17, sub: 0, line: 253 } |  |  | 0.521 |
| ns | 3411 |  | 111 | What each coercion does - and the create() gotcha | 3.5 | 2.5 | 0.516 |
| walker |  | 3419 | 29 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 9, sub: 0, line: 139 } |  |  | 0.516 |
| walker |  | 3449 | 30 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 10, sub: 0, line: 157 } |  |  | 0.516 |
| walker |  | 3488 | 39 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 19, sub: 0, line: 266 } |  |  | 0.516 |
| ns | 3716 |  | 305 | Public overload declarations of the six overloaded factories | 3.6 | 2.2 | 0.491 |
| walker |  | 3730 | 242 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 1, sub: 0, line: 10 } |  |  | 0.541 |
| walker |  | 3802 | 72 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 2, sub: 0, line: 22 } |  |  | 0.541 |
| walker |  | 3874 | 72 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 7, sub: 0, line: 107 } |  |  | 0.541 |
| walker |  | 3902 | 28 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 5, sub: 0, line: 83 } |  |  | 0.541 |
| walker |  | 3935 | 33 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 3, sub: 0, line: 67 } |  |  | 0.541 |
| ns | 3959 |  | 243 | Struct method semantics: mask recursion and validate options | 3.7 | 2.1 | 0.527 |
| walker |  | 3968 | 33 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 4, sub: 0, line: 75 } |  |  | 0.527 |
| walker |  | 4014 | 46 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 12, sub: 0, line: 185 } |  |  | 0.527 |
| walker |  | 4083 | 69 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 1, sub: 0, line: 10 } |  |  | 0.553 |
| ns | 4149 |  | 190 | What each src/utils.ts helper does | 3.8 | 2.9 | 0.542 |
| walker |  | 4152 | 69 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 6, sub: 0, line: 93 } |  |  | 0.544 |
| ns | 4298 |  | 149 | run() signature and per-call context setup | 4.1 | 2.9 | 0.531 |
| ns | 4427 |  | 129 | run() stage 1: coercion, then the struct's own validator | 4.2 | 4.1 | 0.520 |
| walker |  | 4434 | 282 | Markdown::Section { file: Readme.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.520 |
| walker |  | 4672 | 238 | Code::CodeKey { rung: Names, file: src/utils.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.541 |
| walker |  | 4692 | 20 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 11, sub: 0, line: 218 } |  |  | 0.541 |
| walker |  | 4742 | 50 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 7, sub: 0, line: 106 } |  |  | 0.541 |
| ns | 4759 |  | 332 | run() stage 2: recursive descent over entries | 4.3 | 4.1 | 0.517 |
| walker |  | 4792 | 50 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 9, sub: 0, line: 202 } |  |  | 0.517 |
| walker |  | 4846 | 54 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 6, sub: 0, line: 67 } |  |  | 0.517 |
| ns | 4882 |  | 123 | run() stage 3: refiners and the success yield | 4.4 | 4.1 | 0.507 |
| walker |  | 4948 | 102 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 8, sub: 0, line: 130 } |  |  | 0.523 |
| walker |  | 4967 | 19 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 11, sub: 0, line: 218 } |  |  | 0.523 |
| walker |  | 4990 | 23 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 4, sub: 0, line: 45 } |  |  | 0.523 |
| walker |  | 5014 | 24 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 1, sub: 0, line: 16 } |  |  | 0.524 |
| walker |  | 5038 | 24 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 3, sub: 0, line: 32 } |  |  | 0.524 |
| ns | 5054 |  | 172 | validate() body: how one failure becomes a StructError | 4.5 | 2.1 | 0.511 |
| walker |  | 5062 | 24 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 6, sub: 0, line: 67 } |  |  | 0.513 |
| walker |  | 5086 | 24 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 9, sub: 0, line: 202 } |  |  | 0.513 |
| walker |  | 5111 | 25 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 2, sub: 0, line: 24 } |  |  | 0.514 |
| walker |  | 5136 | 25 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 7, sub: 0, line: 106 } |  |  | 0.516 |
| walker |  | 5161 | 25 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 10, sub: 0, line: 212 } |  |  | 0.516 |
| walker |  | 5197 | 36 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 12, sub: 0, line: 227 } |  |  | 0.516 |
| walker |  | 5241 | 44 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 8, sub: 0, line: 130 } |  |  | 0.522 |
| ns | 5265 |  | 211 | Struct constructor: defaults and failure wrapping | 4.6 | 1.6 | 0.505 |
| walker |  | 5291 | 50 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 5, sub: 0, line: 58 } |  |  | 0.513 |
| ns | 5456 |  | 191 | StructError constructor: message, cause and failure caching | 4.7 | 2.7 | 0.504 |
| ns | 5596 |  | 140 | Complete docs/ tree listing | 5.1 |  | 0.532 |
| walker |  | 5600 | 309 | Code::CodeKey { rung: Names, file: src/utils.ts, decl: 0, sub: 1, line: 0 } |  |  | 0.566 |
| walker |  | 5613 | 13 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 20, sub: 0, line: 300 } |  |  | 0.566 |
| walker |  | 5637 | 24 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 18, sub: 0, line: 283 } |  |  | 0.566 |
| ns | 5649 |  | 53 | Complete test/ and test/api/ listings | 5.2 |  | 0.576 |
| walker |  | 5662 | 25 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 21, sub: 0, line: 307 } |  |  | 0.576 |
| walker |  | 5687 | 25 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 23, sub: 0, line: 324 } |  |  | 0.576 |
| walker |  | 5722 | 35 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 14, sub: 0, line: 242 } |  |  | 0.576 |
| walker |  | 5764 | 42 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 19, sub: 0, line: 291 } |  |  | 0.576 |
| ns | 5774 |  | 125 | All 41 validation-fixture kind directories | 5.3 |  | 0.604 |
| walker |  | 5806 | 42 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 22, sub: 0, line: 315 } |  |  | 0.604 |
| ns | 5829 |  | 55 | Complete examples/ listing | 5.4 |  | 0.611 |
| walker |  | 5851 | 45 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 16, sub: 0, line: 267 } |  |  | 0.611 |
| walker |  | 5901 | 50 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 13, sub: 0, line: 233 } |  |  | 0.611 |
| ns | 5906 |  | 77 | One kind directory listed in full, as the case-naming exemplar | 5.5 |  | 0.602 |
| walker |  | 5975 | 74 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 28, sub: 0, line: 394 } |  |  | 0.602 |
| walker |  | 6087 | 112 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 15, sub: 0, line: 251 } |  |  | 0.602 |
| ns | 6089 |  | 183 | The 45 type-level test files | 5.6 |  | 0.626 |
| walker |  | 6106 | 19 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 17, sub: 0, line: 277 } |  |  | 0.626 |
| walker |  | 6127 | 21 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 15, sub: 0, line: 251 } |  |  | 0.626 |
| walker |  | 6148 | 21 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 16, sub: 0, line: 267 } |  |  | 0.626 |
| walker |  | 6169 | 21 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 26, sub: 0, line: 379 } |  |  | 0.626 |
| walker |  | 6191 | 22 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 13, sub: 0, line: 233 } |  |  | 0.626 |
| walker |  | 6213 | 22 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 14, sub: 0, line: 242 } |  |  | 0.626 |
| walker |  | 6235 | 22 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 18, sub: 0, line: 283 } |  |  | 0.626 |
| walker |  | 6257 | 22 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 27, sub: 0, line: 385 } |  |  | 0.626 |
| walker |  | 6280 | 23 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 21, sub: 0, line: 307 } |  |  | 0.626 |
| walker |  | 6304 | 24 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 23, sub: 0, line: 324 } |  |  | 0.626 |
| ns | 6316 |  | 227 | Every heading in the six guides | 5.7 |  | 0.613 |
| walker |  | 6329 | 25 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 22, sub: 0, line: 315 } |  |  | 0.613 |
| walker |  | 6355 | 26 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 19, sub: 0, line: 291 } |  |  | 0.613 |
| ns | 6379 |  | 63 | Readme section map | 5.8 |  | 0.616 |
| walker |  | 6383 | 28 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 20, sub: 0, line: 300 } |  |  | 0.616 |
| walker |  | 6446 | 63 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 28, sub: 0, line: 394 } |  |  | 0.616 |
| ns | 6519 |  | 140 | Doc anchors the source rosters do not cover | 5.9 |  | 0.610 |
| ns | 6690 |  | 171 | The data-driven fixture runner | 5.10 |  | 0.604 |
| walker |  | 6868 | 422 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 25, sub: 0, line: 334 } |  |  | 0.604 |
| walker |  | 6889 | 21 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 25, sub: 0, line: 334 } |  |  | 0.604 |
| ns | 6891 |  | 201 | Runner assertions: output-style vs failures-style fixtures | 5.11 | 5.10 | 0.595 |
| walker |  | 6986 | 97 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 13, sub: 0, line: 221 } |  |  | 0.600 |
| walker |  | 7093 | 107 | Code::CodeKey { rung: Doc, file: src/error.ts, decl: 2, sub: 0, line: 25 } |  |  | 0.614 |
| ns | 7164 |  | 273 | A failures-style and an output-style fixture, in full | 5.12 | 5.10 | 0.598 |
| ns | 7295 |  | 131 | The typings harness and one typings test | 5.13 | 5.6 | 0.592 |
| walker |  | 7299 | 206 | Markdown::Section { file: Readme.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.592 |
| walker |  | 7400 | 101 | Code::CodeKey { rung: Names, file: src/structs/refinements.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.600 |
| walker |  | 7433 | 33 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 5, sub: 0, line: 93 } |  |  | 0.600 |
| walker |  | 7477 | 44 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 7, sub: 0, line: 146 } |  |  | 0.600 |
| ns | 7481 |  | 186 | toFailure(): result normalisation and default message text | 6.1 | 2.9 | 0.592 |
| walker |  | 7525 | 48 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 1, sub: 0, line: 8 } |  |  | 0.592 |
| walker |  | 7573 | 48 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 4, sub: 0, line: 77 } |  |  | 0.592 |
| walker |  | 7630 | 57 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 2, sub: 0, line: 33 } |  |  | 0.592 |
| walker |  | 7687 | 57 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 3, sub: 0, line: 55 } |  |  | 0.592 |
| walker |  | 7749 | 62 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 6, sub: 0, line: 109 } |  |  | 0.592 |
| walker |  | 7773 | 24 | Code::CodeKey { rung: Doc, file: src/structs/refinements.ts, decl: 5, sub: 0, line: 93 } |  |  | 0.592 |
| walker |  | 7799 | 26 | Code::CodeKey { rung: Doc, file: src/structs/refinements.ts, decl: 2, sub: 0, line: 33 } |  |  | 0.592 |
| walker |  | 7825 | 26 | Code::CodeKey { rung: Doc, file: src/structs/refinements.ts, decl: 3, sub: 0, line: 55 } |  |  | 0.593 |
| walker |  | 7854 | 29 | Code::CodeKey { rung: Doc, file: src/structs/refinements.ts, decl: 1, sub: 0, line: 8 } |  |  | 0.594 |
| walker |  | 7883 | 29 | Code::CodeKey { rung: Doc, file: src/structs/refinements.ts, decl: 4, sub: 0, line: 77 } |  |  | 0.595 |
| ns | 7894 |  | 413 | object() implementation, including the mask special case | 6.2 | 3.6 | 0.577 |
| walker |  | 7931 | 48 | Code::CodeKey { rung: Doc, file: src/structs/refinements.ts, decl: 6, sub: 0, line: 109 } |  |  | 0.579 |
| walker |  | 8066 | 135 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 7, sub: 0, line: 107 } |  |  | 0.588 |
| ns | 8085 |  | 191 | refine() and define() bodies - the extension points | 6.3 | 2.4 | 0.581 |
| ns | 8199 |  | 114 | coerce() and trimmed() bodies | 6.4 | 2.5 | 0.576 |
| walker |  | 8286 | 220 | Code::CodeKey { rung: Names, file: src/structs/utilities.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.589 |
| walker |  | 8319 | 33 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 8, sub: 0, line: 106 } |  |  | 0.589 |
| walker |  | 8356 | 37 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 11, sub: 0, line: 197 } |  |  | 0.589 |
| walker |  | 8394 | 38 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 7, sub: 0, line: 80 } |  |  | 0.589 |
| walker |  | 8437 | 43 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 12, sub: 0, line: 221 } |  |  | 0.589 |
| ns | 8458 |  | 259 | npm scripts | 7.1 | 1.1 | 0.595 |
| walker |  | 8481 | 44 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 10, sub: 0, line: 171 } |  |  | 0.595 |
| walker |  | 8526 | 45 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 1, sub: 0, line: 17 } |  |  | 0.595 |
| walker |  | 8628 | 102 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 2, sub: 0, line: 21 } |  |  | 0.595 |
| ns | 8629 |  | 171 | Changelog: recent release headings and the file's extent | 7.2 |  | 0.591 |
| walker |  | 8778 | 150 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 3, sub: 0, line: 30 } |  |  | 0.589 |
| ns | 8778 |  | 149 | The 2.0.0 release notes: breaking changes and fixes | 7.3 | 7.2 | 0.589 |
| walker |  | 8804 | 26 | Code::CodeKey { rung: Doc, file: src/structs/utilities.ts, decl: 6, sub: 0, line: 71 } |  |  | 0.589 |
| ns | 8957 |  | 179 | TypeScript compiler configuration | 7.4 |  | 0.583 |
| walker |  | 8986 | 182 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 4, sub: 0, line: 44 } |  |  | 0.583 |
| walker |  | 9033 | 47 | Code::CodeKey { rung: Doc, file: src/structs/utilities.ts, decl: 13, sub: 0, line: 247 } |  |  | 0.583 |
| walker |  | 9088 | 55 | Code::CodeKey { rung: Doc, file: src/structs/utilities.ts, decl: 7, sub: 0, line: 80 } |  |  | 0.584 |
| walker |  | 9145 | 57 | Code::CodeKey { rung: Doc, file: src/structs/utilities.ts, decl: 10, sub: 0, line: 171 } |  |  | 0.585 |
| walker |  | 9202 | 57 | Code::CodeKey { rung: Doc, file: src/structs/utilities.ts, decl: 12, sub: 0, line: 221 } |  |  | 0.588 |
| ns | 9207 |  | 250 | Rollup build and JSR publish config | 7.5 |  | 0.578 |
| walker |  | 9265 | 63 | Code::CodeKey { rung: Doc, file: src/structs/utilities.ts, decl: 11, sub: 0, line: 197 } |  |  | 0.581 |
| walker |  | 9333 | 68 | Code::CodeKey { rung: Doc, file: src/structs/utilities.ts, decl: 1, sub: 0, line: 17 } |  |  | 0.585 |
| ns | 9368 |  | 161 | CI matrix and dependency automation | 7.6 |  | 0.580 |
| walker |  | 9414 | 81 | Code::CodeKey { rung: Doc, file: src/structs/utilities.ts, decl: 8, sub: 0, line: 106 } |  |  | 0.581 |
| ns | 9521 |  | 153 | Remaining package.json keys: entry points and publish metadata | 7.7 | 1.1 | 0.581 |
| ns | 9599 |  | 78 | Formatting and docs-site configuration | 7.8 |  | 0.579 |
| ns | 9718 |  | 119 | ESLint configuration head | 7.9 |  | 0.575 |
| ns | 9847 |  | 129 | Examples package manifest and run instructions | 7.10 |  | 0.572 |
| walker |  | 9881 | 467 | Markdown::Section { file: Readme.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.572 |
| walker |  | 9996 | 115 | Markdown::Section { file: Readme.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.572 |
