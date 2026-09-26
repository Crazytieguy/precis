Score(3000)=0.452 I=0.769 C=0.266 ns_rows≤3K=20/60 grid(1000/1442/2080/3000/4327/6240/9000)=0.719/0.607/0.500/0.452/0.541/0.600/0.580

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 65 | 65 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 69 |  | 69 | Package identity: name, description, version, license | 1.1 |  | 0.000 |
| walker |  | 84 | 19 | Fs::DirListing { dir: src } |  |  | 0.000 |
| walker |  | 102 | 18 | Fs::DirListing { dir: src/structs } |  |  | 0.000 |
| ns | 134 |  | 65 | Complete root directory listing | 1.2 |  | 0.615 |
| ns | 171 |  | 37 | Complete src/ and src/structs/ listings | 1.3 |  | 0.612 |
| ns | 250 |  | 79 | src/index.ts - the entire public barrel | 1.4 |  | 0.552 |
| walker |  | 268 | 166 | Markdown::ReadmeHeadline { file: Readme.md } |  |  | 0.554 |
| walker |  | 289 | 21 | Fs::DirListing { dir: docs } |  |  | 0.555 |
| walker |  | 297 | 8 | Fs::DirListing { dir: docs/resources } |  |  | 0.555 |
| walker |  | 301 | 4 | Fs::DirListing { dir: .vscode } |  |  | 0.555 |
| walker |  | 383 | 82 | Json::Identity { file: package.json } |  |  | 0.866 |
| walker |  | 391 | 8 | Fs::DirListing { dir: .github } |  |  | 0.866 |
| walker |  | 395 | 4 | Fs::DirListing { dir: .github/workflows } |  |  | 0.866 |
| ns | 416 |  | 166 | Readme lede: what Superstruct is and why it exists | 1.5 |  | 0.847 |
| walker |  | 424 | 29 | Fs::DirListing { dir: docs/images } |  |  | 0.849 |
| walker |  | 455 | 31 | Fs::DirListing { dir: docs/reference } |  |  | 0.851 |
| walker |  | 521 | 66 | Markdown::Prelude { file: Readme.md } |  |  | 0.874 |
| walker |  | 555 | 34 | Json::Runtime { file: package.json } |  |  | 0.874 |
| walker |  | 609 | 54 | Markdown::HeadingsOutline { file: Readme.md } |  |  | 0.875 |
| ns | 638 |  | 222 | The Struct class: doc comment and its six fields | 1.6 |  | 0.723 |
| walker |  | 660 | 51 | Fs::DirListing { dir: docs/guides } |  |  | 0.725 |
| walker |  | 688 | 28 | Fs::DirListing { dir: test } |  |  | 0.726 |
| walker |  | 764 | 76 | Json::Entry { file: package.json } |  |  | 0.727 |
| ns | 816 |  | 178 | Core API signatures: assert / create / is / mask / validate | 1.7 |  | 0.658 |
| ns | 971 |  | 155 | Readme canonical usage snippet | 1.8 |  | 0.577 |
| walker |  | 997 | 233 | Markdown::Section { file: Readme.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.719 |
| walker |  | 1054 | 57 | Markdown::Section { file: Readme.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.719 |
| ns | 1066 |  | 95 | src/struct.ts symbol roster: methods and top-level helpers | 2.1 |  | 0.671 |
| walker |  | 1109 | 55 | Fs::DirListing { dir: examples } |  |  | 0.673 |
| walker |  | 1134 | 25 | Fs::DirListing { dir: test/api } |  |  | 0.674 |
| walker |  | 1213 | 79 | Code::CodeKey { rung: Names, file: src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.727 |
| ns | 1318 |  | 252 | Complete roster of the 25 type structs | 2.2 |  | 0.630 |
| walker |  | 1338 | 125 | Fs::DirListing { dir: test/validation } |  |  | 0.636 |
| ns | 1420 |  | 102 | Complete roster of src/structs/utilities.ts | 2.3 |  | 0.606 |
| ns | 1503 |  | 83 | Complete roster of src/structs/refinements.ts | 2.4 |  | 0.584 |
| walker |  | 1521 | 183 | Fs::DirListing { dir: test/typings } |  |  | 0.589 |
| ns | 1537 |  | 34 | Complete roster of src/structs/coercions.ts | 2.5 |  | 0.582 |
| ns | 1651 |  | 114 | Failure - the shape of every validation failure | 2.6 |  | 0.556 |
| ns | 1829 |  | 178 | StructError class: doc, fields, and its early-exit contract | 2.7 |  | 0.526 |
| walker |  | 1847 | 326 | Json::Scripts { file: package.json } |  |  | 0.527 |
| walker |  | 2023 | 176 | Markdown::Section { file: Readme.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.527 |
| ns | 2037 |  | 208 | Exported type vocabulary of src/struct.ts | 2.8 |  | 0.500 |
| ns | 2140 |  | 103 | Function roster of src/utils.ts | 2.9 |  | 0.486 |
| walker |  | 2218 | 195 | Markdown::Section { file: Readme.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.486 |
| walker |  | 2343 | 125 | Markdown::Section { file: Readme.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.486 |
| ns | 2378 |  | 238 | Type-level utility roster of src/utils.ts | 2.10 |  | 0.457 |
| ns | 2574 |  | 196 | What each type struct does, part 1 (any ... literal) | 3.1 | 2.2 | 0.443 |
| walker |  | 2625 | 282 | Markdown::Section { file: Readme.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.443 |
| walker |  | 2810 | 185 | Code::CodeKey { rung: Names, file: src/utils.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.474 |
| walker |  | 2860 | 50 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 7, sub: 0, line: 106 } |  |  | 0.474 |
| walker |  | 2910 | 50 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 9, sub: 0, line: 202 } |  |  | 0.474 |
| ns | 2916 |  | 342 | What each type struct does, part 2 (map ... unknown) | 3.2 | 2.2 | 0.452 |
| walker |  | 2964 | 54 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 6, sub: 0, line: 67 } |  |  | 0.452 |
| walker |  | 3066 | 102 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 8, sub: 0, line: 130 } |  |  | 0.453 |
| ns | 3144 |  | 228 | What each utility struct does | 3.3 | 2.3 | 0.437 |
| walker |  | 3299 | 233 | Code::CodeKey { rung: Names, file: src/struct.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.470 |
| ns | 3300 |  | 156 | What each refinement does | 3.4 | 2.4 | 0.463 |
| walker |  | 3333 | 34 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 13, sub: 0, line: 221 } |  |  | 0.477 |
| walker |  | 3370 | 37 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 9, sub: 0, line: 139 } |  |  | 0.477 |
| walker |  | 3407 | 37 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 10, sub: 0, line: 157 } |  |  | 0.477 |
| ns | 3411 |  | 111 | What each coercion does - and the create() gotcha | 3.5 | 2.5 | 0.471 |
| walker |  | 3447 | 40 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 8, sub: 0, line: 123 } |  |  | 0.471 |
| walker |  | 3488 | 41 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 16, sub: 0, line: 243 } |  |  | 0.491 |
| walker |  | 3571 | 83 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 12, sub: 0, line: 185 } |  |  | 0.491 |
| ns | 3716 |  | 305 | Public overload declarations of the six overloaded factories | 3.6 | 2.2 | 0.468 |
| walker |  | 3815 | 244 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 1, sub: 0, line: 10 } |  |  | 0.519 |
| walker |  | 3887 | 72 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 2, sub: 0, line: 22 } |  |  | 0.519 |
| walker |  | 3959 | 72 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 7, sub: 0, line: 107 } |  |  | 0.506 |
| ns | 3959 |  | 243 | Struct method semantics: mask recursion and validate options | 3.7 | 2.1 | 0.506 |
| walker |  | 3986 | 27 | Code::CodeKey { rung: Names, file: src/error.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.506 |
| walker |  | 4067 | 81 | Code::CodeKey { rung: Decl, file: src/error.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.527 |
| ns | 4149 |  | 190 | What each src/utils.ts helper does | 3.8 | 2.9 | 0.516 |
| walker |  | 4180 | 113 | Code::CodeKey { rung: Decl, file: src/error.ts, decl: 2, sub: 0, line: 25 } |  |  | 0.527 |
| ns | 4298 |  | 149 | run() signature and per-call context setup | 4.1 | 2.9 | 0.532 |
| walker |  | 4361 | 181 | Code::CodeKey { rung: Names, file: src/utils.ts, decl: 0, sub: 1, line: 0 } |  |  | 0.546 |
| walker |  | 4381 | 20 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 11, sub: 0, line: 218 } |  |  | 0.546 |
| walker |  | 4405 | 24 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 18, sub: 0, line: 283 } |  |  | 0.546 |
| ns | 4427 |  | 129 | run() stage 1: coercion, then the struct's own validator | 4.2 | 4.1 | 0.535 |
| walker |  | 4440 | 35 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 14, sub: 0, line: 242 } |  |  | 0.535 |
| walker |  | 4482 | 42 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 19, sub: 0, line: 291 } |  |  | 0.535 |
| walker |  | 4527 | 45 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 16, sub: 0, line: 267 } |  |  | 0.535 |
| walker |  | 4577 | 50 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 13, sub: 0, line: 233 } |  |  | 0.535 |
| walker |  | 4689 | 112 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 15, sub: 0, line: 251 } |  |  | 0.535 |
| walker |  | 4708 | 19 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 11, sub: 0, line: 218 } |  |  | 0.535 |
| walker |  | 4727 | 19 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 17, sub: 0, line: 277 } |  |  | 0.535 |
| ns | 4759 |  | 332 | run() stage 2: recursive descent over entries | 4.3 | 4.1 | 0.511 |
| ns | 4882 |  | 123 | run() stage 3: refiners and the success yield | 4.4 | 4.1 | 0.501 |
| walker |  | 4908 | 181 | Code::CodeKey { rung: Names, file: src/utils.ts, decl: 0, sub: 2, line: 0 } |  |  | 0.532 |
| walker |  | 4921 | 13 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 20, sub: 0, line: 300 } |  |  | 0.532 |
| walker |  | 4946 | 25 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 21, sub: 0, line: 307 } |  |  | 0.532 |
| walker |  | 4971 | 25 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 23, sub: 0, line: 324 } |  |  | 0.532 |
| walker |  | 5013 | 42 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 22, sub: 0, line: 315 } |  |  | 0.532 |
| ns | 5054 |  | 172 | validate() body: how one failure becomes a StructError | 4.5 | 2.1 | 0.519 |
| walker |  | 5087 | 74 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 28, sub: 0, line: 394 } |  |  | 0.519 |
| ns | 5265 |  | 211 | Struct constructor: defaults and failure wrapping | 4.6 | 1.6 | 0.502 |
| ns | 5456 |  | 191 | StructError constructor: message, cause and failure caching | 4.7 | 2.7 | 0.493 |
| walker |  | 5511 | 424 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 25, sub: 0, line: 334 } |  |  | 0.493 |
| walker |  | 5530 | 19 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 26, sub: 0, line: 379 } |  |  | 0.493 |
| walker |  | 5551 | 21 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 15, sub: 0, line: 251 } |  |  | 0.493 |
| walker |  | 5572 | 21 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 16, sub: 0, line: 267 } |  |  | 0.493 |
| walker |  | 5593 | 21 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 25, sub: 0, line: 334 } |  |  | 0.493 |
| ns | 5596 |  | 140 | Complete docs/ tree listing | 5.1 |  | 0.522 |
| walker |  | 5615 | 22 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 13, sub: 0, line: 233 } |  |  | 0.522 |
| walker |  | 5637 | 22 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 14, sub: 0, line: 242 } |  |  | 0.522 |
| ns | 5649 |  | 53 | Complete test/ and test/api/ listings | 5.2 |  | 0.533 |
| walker |  | 5659 | 22 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 18, sub: 0, line: 283 } |  |  | 0.533 |
| walker |  | 5681 | 22 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 27, sub: 0, line: 385 } |  |  | 0.533 |
| walker |  | 5704 | 23 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 11, sub: 0, line: 175 } |  |  | 0.533 |
| walker |  | 5727 | 23 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 16, sub: 0, line: 243 } |  |  | 0.533 |
| walker |  | 5750 | 23 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 4, sub: 0, line: 45 } |  |  | 0.533 |
| walker |  | 5773 | 23 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 21, sub: 0, line: 307 } |  |  | 0.533 |
| ns | 5774 |  | 125 | All 41 validation-fixture kind directories | 5.3 |  | 0.565 |
| walker |  | 5797 | 24 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 1, sub: 0, line: 16 } |  |  | 0.566 |
| walker |  | 5821 | 24 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 3, sub: 0, line: 32 } |  |  | 0.566 |
| ns | 5829 |  | 55 | Complete examples/ listing | 5.4 |  | 0.573 |
| walker |  | 5845 | 24 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 6, sub: 0, line: 67 } |  |  | 0.574 |
| walker |  | 5869 | 24 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 9, sub: 0, line: 202 } |  |  | 0.574 |
| walker |  | 5893 | 24 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 23, sub: 0, line: 324 } |  |  | 0.574 |
| ns | 5906 |  | 77 | One kind directory listed in full, as the case-naming exemplar | 5.5 |  | 0.566 |
| walker |  | 5918 | 25 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 18, sub: 0, line: 259 } |  |  | 0.566 |
| walker |  | 5943 | 25 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 2, sub: 0, line: 24 } |  |  | 0.567 |
| walker |  | 5968 | 25 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 7, sub: 0, line: 106 } |  |  | 0.569 |
| walker |  | 5993 | 25 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 10, sub: 0, line: 212 } |  |  | 0.569 |
| walker |  | 6018 | 25 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 22, sub: 0, line: 315 } |  |  | 0.569 |
| ns | 6089 |  | 183 | The 45 type-level test files | 5.6 |  | 0.594 |
| walker |  | 6214 | 196 | Markdown::Section { file: Readme.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.594 |
| walker |  | 6240 | 26 | Code::CodeKey { rung: Doc, file: src/error.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.600 |
| walker |  | 6266 | 26 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 8, sub: 0, line: 123 } |  |  | 0.600 |
| walker |  | 6292 | 26 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 19, sub: 0, line: 291 } |  |  | 0.600 |
| ns | 6316 |  | 227 | Every heading in the six guides | 5.7 |  | 0.588 |
| walker |  | 6319 | 27 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 14, sub: 0, line: 231 } |  |  | 0.588 |
| walker |  | 6346 | 27 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 15, sub: 0, line: 237 } |  |  | 0.588 |
| walker |  | 6374 | 28 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 5, sub: 0, line: 83 } |  |  | 0.588 |
| ns | 6379 |  | 63 | Readme section map | 5.8 |  | 0.592 |
| walker |  | 6402 | 28 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 17, sub: 0, line: 253 } |  |  | 0.592 |
| walker |  | 6430 | 28 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 20, sub: 0, line: 300 } |  |  | 0.592 |
| walker |  | 6459 | 29 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 9, sub: 0, line: 139 } |  |  | 0.592 |
| walker |  | 6489 | 30 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 10, sub: 0, line: 157 } |  |  | 0.592 |
| ns | 6519 |  | 140 | Doc anchors the source rosters do not cover | 5.9 |  | 0.585 |
| walker |  | 6522 | 33 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 3, sub: 0, line: 67 } |  |  | 0.585 |
| walker |  | 6555 | 33 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 4, sub: 0, line: 75 } |  |  | 0.585 |
| walker |  | 6591 | 36 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 12, sub: 0, line: 227 } |  |  | 0.585 |
| ns | 6690 |  | 171 | The data-driven fixture runner | 5.10 |  | 0.580 |
| ns | 6891 |  | 201 | Runner assertions: output-style vs failures-style fixtures | 5.11 | 5.10 | 0.571 |
| walker |  | 7048 | 457 | Markdown::Section { file: Readme.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.571 |
| ns | 7164 |  | 273 | A failures-style and an output-style fixture, in full | 5.12 | 5.10 | 0.555 |
| ns | 7295 |  | 131 | The typings harness and one typings test | 5.13 | 5.6 | 0.550 |
| walker |  | 7370 | 322 | Markdown::Section { file: Readme.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.550 |
| walker |  | 7409 | 39 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 19, sub: 0, line: 266 } |  |  | 0.550 |
| ns | 7481 |  | 186 | toFailure(): result normalisation and default message text | 6.1 | 2.9 | 0.542 |
| walker |  | 7594 | 185 | Code::CodeKey { rung: Names, file: src/structs/types.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.545 |
| walker |  | 7610 | 16 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 10, sub: 0, line: 105 } |  |  | 0.545 |
| walker |  | 7636 | 26 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 8, sub: 0, line: 99 } |  |  | 0.546 |
| walker |  | 7662 | 26 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 9, sub: 0, line: 102 } |  |  | 0.547 |
| walker |  | 7831 | 169 | Code::CodeKey { rung: Names, file: src/structs/types.ts, decl: 0, sub: 1, line: 0 } |  |  | 0.558 |
| walker |  | 7854 | 23 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 12, sub: 0, line: 144 } |  |  | 0.558 |
| walker |  | 7894 | 40 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 14, sub: 0, line: 172 } |  |  | 0.540 |
| ns | 7894 |  | 413 | object() implementation, including the mask special case | 6.2 | 3.6 | 0.540 |
| ns | 8085 |  | 191 | refine() and define() bodies - the extension points | 6.3 | 2.4 | 0.533 |
| walker |  | 8088 | 194 | Code::CodeKey { rung: Names, file: src/structs/types.ts, decl: 0, sub: 2, line: 0 } |  |  | 0.548 |
| walker |  | 8107 | 19 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 27, sub: 0, line: 295 } |  |  | 0.551 |
| walker |  | 8138 | 31 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 21, sub: 0, line: 226 } |  |  | 0.555 |
| ns | 8199 |  | 114 | coerce() and trimmed() bodies | 6.4 | 2.5 | 0.550 |
| walker |  | 8313 | 175 | Code::CodeKey { rung: Names, file: src/structs/types.ts, decl: 0, sub: 3, line: 0 } |  |  | 0.573 |
| walker |  | 8335 | 22 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 37, sub: 0, line: 492 } |  |  | 0.573 |
| walker |  | 8369 | 34 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 30, sub: 0, line: 367 } |  |  | 0.573 |
| walker |  | 8403 | 34 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 36, sub: 0, line: 456 } |  |  | 0.573 |
| walker |  | 8438 | 35 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 38, sub: 0, line: 522 } |  |  | 0.573 |
| ns | 8458 |  | 259 | npm scripts | 7.1 | 1.1 | 0.579 |
| walker |  | 8460 | 22 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 1, sub: 0, line: 19 } |  |  | 0.579 |
| walker |  | 8483 | 23 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 5, sub: 0, line: 60 } |  |  | 0.579 |
| walker |  | 8506 | 23 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 6, sub: 0, line: 70 } |  |  | 0.579 |
| walker |  | 8529 | 23 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 11, sub: 0, line: 131 } |  |  | 0.580 |
| walker |  | 8552 | 23 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 13, sub: 0, line: 159 } |  |  | 0.581 |
| walker |  | 8575 | 23 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 23, sub: 0, line: 258 } |  |  | 0.581 |
| walker |  | 8598 | 23 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 25, sub: 0, line: 278 } |  |  | 0.581 |
| walker |  | 8621 | 23 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 35, sub: 0, line: 442 } |  |  | 0.581 |
| ns | 8629 |  | 171 | Changelog: recent release headings and the file's extent | 7.2 |  | 0.577 |
| walker |  | 8647 | 26 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 29, sub: 0, line: 351 } |  |  | 0.577 |
| walker |  | 8674 | 27 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 12, sub: 0, line: 144 } |  |  | 0.579 |
| walker |  | 8701 | 27 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 14, sub: 0, line: 172 } |  |  | 0.580 |
| walker |  | 8728 | 27 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 24, sub: 0, line: 266 } |  |  | 0.580 |
| walker |  | 8755 | 27 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 38, sub: 0, line: 522 } |  |  | 0.581 |
| ns | 8778 |  | 149 | The 2.0.0 release notes: breaking changes and fixes | 7.3 | 7.2 | 0.579 |
| walker |  | 8785 | 30 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 39, sub: 0, line: 574 } |  |  | 0.580 |
| walker |  | 8816 | 31 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 15, sub: 0, line: 200 } |  |  | 0.581 |
| walker |  | 8858 | 42 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 32, sub: 0, line: 413 } |  |  | 0.583 |
| walker |  | 8901 | 43 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 20, sub: 0, line: 225 } |  |  | 0.585 |
| ns | 8957 |  | 179 | TypeScript compiler configuration | 7.4 |  | 0.579 |
| walker |  | 9121 | 220 | Code::CodeKey { rung: Names, file: src/structs/utilities.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.591 |
| walker |  | 9154 | 33 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 8, sub: 0, line: 106 } |  |  | 0.591 |
| walker |  | 9191 | 37 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 11, sub: 0, line: 197 } |  |  | 0.591 |
| ns | 9207 |  | 250 | Rollup build and JSR publish config | 7.5 |  | 0.582 |
| walker |  | 9229 | 38 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 7, sub: 0, line: 80 } |  |  | 0.582 |
| walker |  | 9272 | 43 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 12, sub: 0, line: 221 } |  |  | 0.582 |
| walker |  | 9316 | 44 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 10, sub: 0, line: 171 } |  |  | 0.582 |
| walker |  | 9361 | 45 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 1, sub: 0, line: 17 } |  |  | 0.582 |
| ns | 9368 |  | 161 | CI matrix and dependency automation | 7.6 |  | 0.576 |
| walker |  | 9463 | 102 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 2, sub: 0, line: 21 } |  |  | 0.576 |
| ns | 9521 |  | 153 | Remaining package.json keys: entry points and publish metadata | 7.7 | 1.1 | 0.577 |
| ns | 9599 |  | 78 | Formatting and docs-site configuration | 7.8 |  | 0.574 |
| walker |  | 9613 | 150 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 3, sub: 0, line: 30 } |  |  | 0.574 |
| ns | 9718 |  | 119 | ESLint configuration head | 7.9 |  | 0.570 |
| walker |  | 9795 | 182 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 4, sub: 0, line: 44 } |  |  | 0.570 |
| ns | 9847 |  | 129 | Examples package manifest and run instructions | 7.10 |  | 0.568 |
| walker |  | 9896 | 101 | Code::CodeKey { rung: Names, file: src/structs/refinements.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.575 |
| walker |  | 9929 | 33 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 5, sub: 0, line: 93 } |  |  | 0.575 |
| walker |  | 9973 | 44 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 7, sub: 0, line: 146 } |  |  | 0.576 |
