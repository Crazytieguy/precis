Score(3000)=0.452 I=0.769 C=0.266 ns_rows≤3K=20/60 grid(1000/1442/2080/3000/4327/6240/9000)=0.719/0.610/0.500/0.452/0.534/0.600/0.579

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
| walker |  | 1052 | 55 | Fs::DirListing { dir: examples } |  |  | 0.721 |
| ns | 1066 |  | 95 | src/struct.ts symbol roster: methods and top-level helpers | 2.1 |  | 0.673 |
| walker |  | 1077 | 25 | Fs::DirListing { dir: test/api } |  |  | 0.674 |
| walker |  | 1156 | 79 | Code::CodeKey { rung: Names, file: src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.727 |
| walker |  | 1281 | 125 | Fs::DirListing { dir: test/validation } |  |  | 0.734 |
| ns | 1318 |  | 252 | Complete roster of the 25 type structs | 2.2 |  | 0.636 |
| ns | 1420 |  | 102 | Complete roster of src/structs/utilities.ts | 2.3 |  | 0.606 |
| walker |  | 1464 | 183 | Fs::DirListing { dir: test/typings } |  |  | 0.611 |
| ns | 1503 |  | 83 | Complete roster of src/structs/refinements.ts | 2.4 |  | 0.589 |
| ns | 1537 |  | 34 | Complete roster of src/structs/coercions.ts | 2.5 |  | 0.582 |
| walker |  | 1575 | 111 | Markdown::Section { file: Readme.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.582 |
| ns | 1651 |  | 114 | Failure - the shape of every validation failure | 2.6 |  | 0.556 |
| ns | 1829 |  | 178 | StructError class: doc, fields, and its early-exit contract | 2.7 |  | 0.526 |
| walker |  | 1901 | 326 | Json::Scripts { file: package.json } |  |  | 0.527 |
| ns | 2037 |  | 208 | Exported type vocabulary of src/struct.ts | 2.8 |  | 0.500 |
| walker |  | 2077 | 176 | Markdown::Section { file: Readme.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.500 |
| ns | 2140 |  | 103 | Function roster of src/utils.ts | 2.9 |  | 0.486 |
| walker |  | 2272 | 195 | Markdown::Section { file: Readme.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.486 |
| ns | 2378 |  | 238 | Type-level utility roster of src/utils.ts | 2.10 |  | 0.457 |
| walker |  | 2407 | 135 | Markdown::Section { file: Readme.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.457 |
| ns | 2574 |  | 196 | What each type struct does, part 1 (any ... literal) | 3.1 | 2.2 | 0.443 |
| walker |  | 2689 | 282 | Markdown::Section { file: Readme.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.443 |
| walker |  | 2874 | 185 | Code::CodeKey { rung: Names, file: src/utils.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.474 |
| ns | 2916 |  | 342 | What each type struct does, part 2 (map ... unknown) | 3.2 | 2.2 | 0.452 |
| walker |  | 2924 | 50 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 7, sub: 0, line: 106 } |  |  | 0.452 |
| walker |  | 2974 | 50 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 9, sub: 0, line: 202 } |  |  | 0.452 |
| walker |  | 3028 | 54 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 6, sub: 0, line: 67 } |  |  | 0.452 |
| walker |  | 3130 | 102 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 8, sub: 0, line: 130 } |  |  | 0.453 |
| ns | 3144 |  | 228 | What each utility struct does | 3.3 | 2.3 | 0.437 |
| ns | 3300 |  | 156 | What each refinement does | 3.4 | 2.4 | 0.430 |
| walker |  | 3363 | 233 | Code::CodeKey { rung: Names, file: src/struct.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.463 |
| walker |  | 3397 | 34 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 13, sub: 0, line: 221 } |  |  | 0.477 |
| ns | 3411 |  | 111 | What each coercion does - and the create() gotcha | 3.5 | 2.5 | 0.471 |
| walker |  | 3434 | 37 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 9, sub: 0, line: 139 } |  |  | 0.471 |
| walker |  | 3471 | 37 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 10, sub: 0, line: 157 } |  |  | 0.471 |
| walker |  | 3511 | 40 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 8, sub: 0, line: 123 } |  |  | 0.471 |
| walker |  | 3552 | 41 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 16, sub: 0, line: 243 } |  |  | 0.491 |
| walker |  | 3635 | 83 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 12, sub: 0, line: 185 } |  |  | 0.491 |
| ns | 3716 |  | 305 | Public overload declarations of the six overloaded factories | 3.6 | 2.2 | 0.468 |
| walker |  | 3879 | 244 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 1, sub: 0, line: 10 } |  |  | 0.519 |
| walker |  | 3951 | 72 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 2, sub: 0, line: 22 } |  |  | 0.519 |
| ns | 3959 |  | 243 | Struct method semantics: mask recursion and validate options | 3.7 | 2.1 | 0.506 |
| walker |  | 4023 | 72 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 7, sub: 0, line: 107 } |  |  | 0.506 |
| walker |  | 4050 | 27 | Code::CodeKey { rung: Names, file: src/error.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.506 |
| walker |  | 4131 | 81 | Code::CodeKey { rung: Decl, file: src/error.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.527 |
| ns | 4149 |  | 190 | What each src/utils.ts helper does | 3.8 | 2.9 | 0.516 |
| walker |  | 4244 | 113 | Code::CodeKey { rung: Decl, file: src/error.ts, decl: 2, sub: 0, line: 25 } |  |  | 0.527 |
| ns | 4298 |  | 149 | run() signature and per-call context setup | 4.1 | 2.9 | 0.532 |
| walker |  | 4425 | 181 | Code::CodeKey { rung: Names, file: src/utils.ts, decl: 0, sub: 1, line: 0 } |  |  | 0.546 |
| ns | 4427 |  | 129 | run() stage 1: coercion, then the struct's own validator | 4.2 | 4.1 | 0.535 |
| walker |  | 4445 | 20 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 11, sub: 0, line: 218 } |  |  | 0.535 |
| walker |  | 4469 | 24 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 18, sub: 0, line: 283 } |  |  | 0.535 |
| walker |  | 4504 | 35 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 14, sub: 0, line: 242 } |  |  | 0.535 |
| walker |  | 4546 | 42 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 19, sub: 0, line: 291 } |  |  | 0.535 |
| walker |  | 4591 | 45 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 16, sub: 0, line: 267 } |  |  | 0.535 |
| walker |  | 4641 | 50 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 13, sub: 0, line: 233 } |  |  | 0.535 |
| walker |  | 4753 | 112 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 15, sub: 0, line: 251 } |  |  | 0.535 |
| ns | 4759 |  | 332 | run() stage 2: recursive descent over entries | 4.3 | 4.1 | 0.511 |
| walker |  | 4772 | 19 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 11, sub: 0, line: 218 } |  |  | 0.511 |
| walker |  | 4791 | 19 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 17, sub: 0, line: 277 } |  |  | 0.511 |
| ns | 4882 |  | 123 | run() stage 3: refiners and the success yield | 4.4 | 4.1 | 0.501 |
| walker |  | 4972 | 181 | Code::CodeKey { rung: Names, file: src/utils.ts, decl: 0, sub: 2, line: 0 } |  |  | 0.532 |
| walker |  | 4985 | 13 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 20, sub: 0, line: 300 } |  |  | 0.532 |
| walker |  | 5010 | 25 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 21, sub: 0, line: 307 } |  |  | 0.532 |
| walker |  | 5035 | 25 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 23, sub: 0, line: 324 } |  |  | 0.532 |
| ns | 5054 |  | 172 | validate() body: how one failure becomes a StructError | 4.5 | 2.1 | 0.519 |
| walker |  | 5077 | 42 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 22, sub: 0, line: 315 } |  |  | 0.519 |
| walker |  | 5151 | 74 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 28, sub: 0, line: 394 } |  |  | 0.519 |
| ns | 5265 |  | 211 | Struct constructor: defaults and failure wrapping | 4.6 | 1.6 | 0.502 |
| ns | 5456 |  | 191 | StructError constructor: message, cause and failure caching | 4.7 | 2.7 | 0.493 |
| walker |  | 5575 | 424 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 25, sub: 0, line: 334 } |  |  | 0.493 |
| walker |  | 5594 | 19 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 26, sub: 0, line: 379 } |  |  | 0.493 |
| ns | 5596 |  | 140 | Complete docs/ tree listing | 5.1 |  | 0.522 |
| walker |  | 5615 | 21 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 15, sub: 0, line: 251 } |  |  | 0.522 |
| walker |  | 5636 | 21 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 16, sub: 0, line: 267 } |  |  | 0.522 |
| ns | 5649 |  | 53 | Complete test/ and test/api/ listings | 5.2 |  | 0.533 |
| walker |  | 5657 | 21 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 25, sub: 0, line: 334 } |  |  | 0.533 |
| walker |  | 5679 | 22 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 13, sub: 0, line: 233 } |  |  | 0.533 |
| walker |  | 5701 | 22 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 14, sub: 0, line: 242 } |  |  | 0.533 |
| walker |  | 5723 | 22 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 18, sub: 0, line: 283 } |  |  | 0.533 |
| walker |  | 5745 | 22 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 27, sub: 0, line: 385 } |  |  | 0.533 |
| walker |  | 5768 | 23 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 11, sub: 0, line: 175 } |  |  | 0.533 |
| ns | 5774 |  | 125 | All 41 validation-fixture kind directories | 5.3 |  | 0.565 |
| walker |  | 5791 | 23 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 16, sub: 0, line: 243 } |  |  | 0.565 |
| walker |  | 5814 | 23 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 4, sub: 0, line: 45 } |  |  | 0.565 |
| ns | 5829 |  | 55 | Complete examples/ listing | 5.4 |  | 0.573 |
| walker |  | 5837 | 23 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 21, sub: 0, line: 307 } |  |  | 0.573 |
| walker |  | 5861 | 24 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 1, sub: 0, line: 16 } |  |  | 0.573 |
| walker |  | 5885 | 24 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 3, sub: 0, line: 32 } |  |  | 0.573 |
| ns | 5906 |  | 77 | One kind directory listed in full, as the case-naming exemplar | 5.5 |  | 0.565 |
| walker |  | 5909 | 24 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 6, sub: 0, line: 67 } |  |  | 0.566 |
| walker |  | 5933 | 24 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 9, sub: 0, line: 202 } |  |  | 0.566 |
| walker |  | 5957 | 24 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 23, sub: 0, line: 324 } |  |  | 0.566 |
| walker |  | 5982 | 25 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 18, sub: 0, line: 259 } |  |  | 0.566 |
| walker |  | 6007 | 25 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 2, sub: 0, line: 24 } |  |  | 0.567 |
| walker |  | 6032 | 25 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 7, sub: 0, line: 106 } |  |  | 0.569 |
| walker |  | 6057 | 25 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 10, sub: 0, line: 212 } |  |  | 0.569 |
| walker |  | 6082 | 25 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 22, sub: 0, line: 315 } |  |  | 0.569 |
| ns | 6089 |  | 183 | The 45 type-level test files | 5.6 |  | 0.594 |
| walker |  | 6108 | 26 | Code::CodeKey { rung: Doc, file: src/error.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.600 |
| walker |  | 6134 | 26 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 8, sub: 0, line: 123 } |  |  | 0.600 |
| walker |  | 6160 | 26 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 19, sub: 0, line: 291 } |  |  | 0.600 |
| ns | 6316 |  | 227 | Every heading in the six guides | 5.7 |  | 0.588 |
| walker |  | 6366 | 206 | Markdown::Section { file: Readme.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.588 |
| ns | 6379 |  | 63 | Readme section map | 5.8 |  | 0.592 |
| walker |  | 6393 | 27 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 14, sub: 0, line: 231 } |  |  | 0.592 |
| walker |  | 6420 | 27 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 15, sub: 0, line: 237 } |  |  | 0.592 |
| walker |  | 6448 | 28 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 5, sub: 0, line: 83 } |  |  | 0.592 |
| walker |  | 6476 | 28 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 17, sub: 0, line: 253 } |  |  | 0.592 |
| walker |  | 6504 | 28 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 20, sub: 0, line: 300 } |  |  | 0.592 |
| ns | 6519 |  | 140 | Doc anchors the source rosters do not cover | 5.9 |  | 0.585 |
| walker |  | 6533 | 29 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 9, sub: 0, line: 139 } |  |  | 0.585 |
| walker |  | 6563 | 30 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 10, sub: 0, line: 157 } |  |  | 0.585 |
| walker |  | 6596 | 33 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 3, sub: 0, line: 67 } |  |  | 0.585 |
| walker |  | 6629 | 33 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 4, sub: 0, line: 75 } |  |  | 0.585 |
| walker |  | 6665 | 36 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 12, sub: 0, line: 227 } |  |  | 0.585 |
| ns | 6690 |  | 171 | The data-driven fixture runner | 5.10 |  | 0.580 |
| ns | 6891 |  | 201 | Runner assertions: output-style vs failures-style fixtures | 5.11 | 5.10 | 0.571 |
| walker |  | 7132 | 467 | Markdown::Section { file: Readme.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.571 |
| ns | 7164 |  | 273 | A failures-style and an output-style fixture, in full | 5.12 | 5.10 | 0.555 |
| walker |  | 7171 | 39 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 19, sub: 0, line: 266 } |  |  | 0.555 |
| ns | 7295 |  | 131 | The typings harness and one typings test | 5.13 | 5.6 | 0.550 |
| ns | 7481 |  | 186 | toFailure(): result normalisation and default message text | 6.1 | 2.9 | 0.542 |
| walker |  | 7503 | 332 | Markdown::Section { file: Readme.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.542 |
| walker |  | 7688 | 185 | Code::CodeKey { rung: Names, file: src/structs/types.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.545 |
| walker |  | 7704 | 16 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 10, sub: 0, line: 105 } |  |  | 0.545 |
| walker |  | 7730 | 26 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 8, sub: 0, line: 99 } |  |  | 0.546 |
| walker |  | 7756 | 26 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 9, sub: 0, line: 102 } |  |  | 0.547 |
| ns | 7894 |  | 413 | object() implementation, including the mask special case | 6.2 | 3.6 | 0.530 |
| walker |  | 7925 | 169 | Code::CodeKey { rung: Names, file: src/structs/types.ts, decl: 0, sub: 1, line: 0 } |  |  | 0.540 |
| walker |  | 7948 | 23 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 12, sub: 0, line: 144 } |  |  | 0.540 |
| walker |  | 7988 | 40 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 14, sub: 0, line: 172 } |  |  | 0.540 |
| ns | 8085 |  | 191 | refine() and define() bodies - the extension points | 6.3 | 2.4 | 0.533 |
| walker |  | 8182 | 194 | Code::CodeKey { rung: Names, file: src/structs/types.ts, decl: 0, sub: 2, line: 0 } |  |  | 0.548 |
| ns | 8199 |  | 114 | coerce() and trimmed() bodies | 6.4 | 2.5 | 0.544 |
| walker |  | 8201 | 19 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 27, sub: 0, line: 295 } |  |  | 0.546 |
| walker |  | 8232 | 31 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 21, sub: 0, line: 226 } |  |  | 0.550 |
| walker |  | 8407 | 175 | Code::CodeKey { rung: Names, file: src/structs/types.ts, decl: 0, sub: 3, line: 0 } |  |  | 0.573 |
| walker |  | 8429 | 22 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 37, sub: 0, line: 492 } |  |  | 0.573 |
| ns | 8458 |  | 259 | npm scripts | 7.1 | 1.1 | 0.579 |
| walker |  | 8463 | 34 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 30, sub: 0, line: 367 } |  |  | 0.579 |
| walker |  | 8497 | 34 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 36, sub: 0, line: 456 } |  |  | 0.579 |
| walker |  | 8532 | 35 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 38, sub: 0, line: 522 } |  |  | 0.579 |
| walker |  | 8554 | 22 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 1, sub: 0, line: 19 } |  |  | 0.579 |
| walker |  | 8577 | 23 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 5, sub: 0, line: 60 } |  |  | 0.579 |
| walker |  | 8600 | 23 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 6, sub: 0, line: 70 } |  |  | 0.579 |
| walker |  | 8623 | 23 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 11, sub: 0, line: 131 } |  |  | 0.580 |
| ns | 8629 |  | 171 | Changelog: recent release headings and the file's extent | 7.2 |  | 0.576 |
| walker |  | 8646 | 23 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 13, sub: 0, line: 159 } |  |  | 0.577 |
| walker |  | 8669 | 23 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 23, sub: 0, line: 258 } |  |  | 0.577 |
| walker |  | 8692 | 23 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 25, sub: 0, line: 278 } |  |  | 0.577 |
| walker |  | 8715 | 23 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 35, sub: 0, line: 442 } |  |  | 0.577 |
| walker |  | 8741 | 26 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 29, sub: 0, line: 351 } |  |  | 0.577 |
| walker |  | 8768 | 27 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 12, sub: 0, line: 144 } |  |  | 0.579 |
| ns | 8778 |  | 149 | The 2.0.0 release notes: breaking changes and fixes | 7.3 | 7.2 | 0.576 |
| walker |  | 8795 | 27 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 14, sub: 0, line: 172 } |  |  | 0.578 |
| walker |  | 8822 | 27 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 24, sub: 0, line: 266 } |  |  | 0.578 |
| walker |  | 8849 | 27 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 38, sub: 0, line: 522 } |  |  | 0.579 |
| walker |  | 8879 | 30 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 39, sub: 0, line: 574 } |  |  | 0.580 |
| walker |  | 8910 | 31 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 15, sub: 0, line: 200 } |  |  | 0.581 |
| walker |  | 8952 | 42 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 32, sub: 0, line: 413 } |  |  | 0.583 |
| ns | 8957 |  | 179 | TypeScript compiler configuration | 7.4 |  | 0.577 |
| walker |  | 8995 | 43 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 20, sub: 0, line: 225 } |  |  | 0.579 |
| ns | 9207 |  | 250 | Rollup build and JSR publish config | 7.5 |  | 0.570 |
| walker |  | 9215 | 220 | Code::CodeKey { rung: Names, file: src/structs/utilities.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.582 |
| walker |  | 9248 | 33 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 8, sub: 0, line: 106 } |  |  | 0.582 |
| walker |  | 9285 | 37 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 11, sub: 0, line: 197 } |  |  | 0.582 |
| walker |  | 9323 | 38 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 7, sub: 0, line: 80 } |  |  | 0.582 |
| walker |  | 9366 | 43 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 12, sub: 0, line: 221 } |  |  | 0.582 |
| ns | 9368 |  | 161 | CI matrix and dependency automation | 7.6 |  | 0.576 |
| walker |  | 9410 | 44 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 10, sub: 0, line: 171 } |  |  | 0.576 |
| walker |  | 9455 | 45 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 1, sub: 0, line: 17 } |  |  | 0.576 |
| ns | 9521 |  | 153 | Remaining package.json keys: entry points and publish metadata | 7.7 | 1.1 | 0.577 |
| walker |  | 9557 | 102 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 2, sub: 0, line: 21 } |  |  | 0.577 |
| ns | 9599 |  | 78 | Formatting and docs-site configuration | 7.8 |  | 0.574 |
| walker |  | 9707 | 150 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 3, sub: 0, line: 30 } |  |  | 0.574 |
| ns | 9718 |  | 119 | ESLint configuration head | 7.9 |  | 0.570 |
| ns | 9847 |  | 129 | Examples package manifest and run instructions | 7.10 |  | 0.568 |
| walker |  | 9889 | 182 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 4, sub: 0, line: 44 } |  |  | 0.568 |
| walker |  | 9990 | 101 | Code::CodeKey { rung: Names, file: src/structs/refinements.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.575 |
