Score(3000)=0.452 I=0.769 C=0.266 ns_rows≤3K=20/60 grid(1000/1442/2080/3000/4327/6240/9000)=0.719/0.610/0.500/0.452/0.532/0.600/0.578

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
| walker |  | 4267 | 23 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 11, sub: 0, line: 175 } |  |  | 0.527 |
| walker |  | 4290 | 23 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 16, sub: 0, line: 243 } |  |  | 0.527 |
| ns | 4298 |  | 149 | run() signature and per-call context setup | 4.1 | 2.9 | 0.532 |
| walker |  | 4313 | 23 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 4, sub: 0, line: 45 } |  |  | 0.532 |
| walker |  | 4337 | 24 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 1, sub: 0, line: 16 } |  |  | 0.533 |
| walker |  | 4361 | 24 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 3, sub: 0, line: 32 } |  |  | 0.533 |
| walker |  | 4385 | 24 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 6, sub: 0, line: 67 } |  |  | 0.535 |
| walker |  | 4409 | 24 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 9, sub: 0, line: 202 } |  |  | 0.535 |
| ns | 4427 |  | 129 | run() stage 1: coercion, then the struct's own validator | 4.2 | 4.1 | 0.524 |
| walker |  | 4590 | 181 | Code::CodeKey { rung: Names, file: src/utils.ts, decl: 0, sub: 1, line: 0 } |  |  | 0.537 |
| walker |  | 4610 | 20 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 11, sub: 0, line: 218 } |  |  | 0.537 |
| walker |  | 4634 | 24 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 18, sub: 0, line: 283 } |  |  | 0.537 |
| walker |  | 4669 | 35 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 14, sub: 0, line: 242 } |  |  | 0.537 |
| walker |  | 4711 | 42 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 19, sub: 0, line: 291 } |  |  | 0.537 |
| walker |  | 4756 | 45 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 16, sub: 0, line: 267 } |  |  | 0.537 |
| ns | 4759 |  | 332 | run() stage 2: recursive descent over entries | 4.3 | 4.1 | 0.513 |
| walker |  | 4806 | 50 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 13, sub: 0, line: 233 } |  |  | 0.513 |
| ns | 4882 |  | 123 | run() stage 3: refiners and the success yield | 4.4 | 4.1 | 0.504 |
| walker |  | 4918 | 112 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 15, sub: 0, line: 251 } |  |  | 0.504 |
| walker |  | 4937 | 19 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 11, sub: 0, line: 218 } |  |  | 0.504 |
| walker |  | 4956 | 19 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 17, sub: 0, line: 277 } |  |  | 0.504 |
| walker |  | 4977 | 21 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 15, sub: 0, line: 251 } |  |  | 0.504 |
| walker |  | 4998 | 21 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 16, sub: 0, line: 267 } |  |  | 0.504 |
| walker |  | 5020 | 22 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 13, sub: 0, line: 233 } |  |  | 0.504 |
| walker |  | 5042 | 22 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 14, sub: 0, line: 242 } |  |  | 0.504 |
| ns | 5054 |  | 172 | validate() body: how one failure becomes a StructError | 4.5 | 2.1 | 0.491 |
| walker |  | 5064 | 22 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 18, sub: 0, line: 283 } |  |  | 0.491 |
| walker |  | 5245 | 181 | Code::CodeKey { rung: Names, file: src/utils.ts, decl: 0, sub: 2, line: 0 } |  |  | 0.521 |
| walker |  | 5258 | 13 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 20, sub: 0, line: 300 } |  |  | 0.521 |
| ns | 5265 |  | 211 | Struct constructor: defaults and failure wrapping | 4.6 | 1.6 | 0.505 |
| walker |  | 5283 | 25 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 21, sub: 0, line: 307 } |  |  | 0.505 |
| walker |  | 5308 | 25 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 23, sub: 0, line: 324 } |  |  | 0.505 |
| walker |  | 5350 | 42 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 22, sub: 0, line: 315 } |  |  | 0.505 |
| walker |  | 5424 | 74 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 28, sub: 0, line: 394 } |  |  | 0.505 |
| ns | 5456 |  | 191 | StructError constructor: message, cause and failure caching | 4.7 | 2.7 | 0.495 |
| ns | 5596 |  | 140 | Complete docs/ tree listing | 5.1 |  | 0.524 |
| ns | 5649 |  | 53 | Complete test/ and test/api/ listings | 5.2 |  | 0.535 |
| ns | 5774 |  | 125 | All 41 validation-fixture kind directories | 5.3 |  | 0.567 |
| ns | 5829 |  | 55 | Complete examples/ listing | 5.4 |  | 0.574 |
| walker |  | 5848 | 424 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 25, sub: 0, line: 334 } |  |  | 0.574 |
| walker |  | 5867 | 19 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 26, sub: 0, line: 379 } |  |  | 0.574 |
| walker |  | 5888 | 21 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 25, sub: 0, line: 334 } |  |  | 0.574 |
| ns | 5906 |  | 77 | One kind directory listed in full, as the case-naming exemplar | 5.5 |  | 0.566 |
| walker |  | 5910 | 22 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 27, sub: 0, line: 385 } |  |  | 0.566 |
| walker |  | 5933 | 23 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 21, sub: 0, line: 307 } |  |  | 0.566 |
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
| walker |  | 6187 | 27 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 14, sub: 0, line: 231 } |  |  | 0.600 |
| walker |  | 6214 | 27 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 15, sub: 0, line: 237 } |  |  | 0.600 |
| walker |  | 6242 | 28 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 5, sub: 0, line: 83 } |  |  | 0.600 |
| walker |  | 6270 | 28 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 17, sub: 0, line: 253 } |  |  | 0.600 |
| walker |  | 6298 | 28 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 20, sub: 0, line: 300 } |  |  | 0.600 |
| ns | 6316 |  | 227 | Every heading in the six guides | 5.7 |  | 0.588 |
| walker |  | 6327 | 29 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 9, sub: 0, line: 139 } |  |  | 0.588 |
| walker |  | 6357 | 30 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 10, sub: 0, line: 157 } |  |  | 0.588 |
| ns | 6379 |  | 63 | Readme section map | 5.8 |  | 0.592 |
| ns | 6519 |  | 140 | Doc anchors the source rosters do not cover | 5.9 |  | 0.585 |
| walker |  | 6563 | 206 | Markdown::Section { file: Readme.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.585 |
| walker |  | 6596 | 33 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 3, sub: 0, line: 67 } |  |  | 0.585 |
| walker |  | 6629 | 33 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 4, sub: 0, line: 75 } |  |  | 0.585 |
| walker |  | 6665 | 36 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 12, sub: 0, line: 227 } |  |  | 0.585 |
| ns | 6690 |  | 171 | The data-driven fixture runner | 5.10 |  | 0.580 |
| walker |  | 6704 | 39 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 19, sub: 0, line: 266 } |  |  | 0.580 |
| walker |  | 6748 | 44 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 8, sub: 0, line: 130 } |  |  | 0.583 |
| ns | 6891 |  | 201 | Runner assertions: output-style vs failures-style fixtures | 5.11 | 5.10 | 0.574 |
| ns | 7164 |  | 273 | A failures-style and an output-style fixture, in full | 5.12 | 5.10 | 0.559 |
| walker |  | 7215 | 467 | Markdown::Section { file: Readme.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.559 |
| walker |  | 7261 | 46 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 12, sub: 0, line: 185 } |  |  | 0.559 |
| ns | 7295 |  | 131 | The typings harness and one typings test | 5.13 | 5.6 | 0.553 |
| ns | 7481 |  | 186 | toFailure(): result normalisation and default message text | 6.1 | 2.9 | 0.545 |
| walker |  | 7593 | 332 | Markdown::Section { file: Readme.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.545 |
| walker |  | 7778 | 185 | Code::CodeKey { rung: Names, file: src/structs/types.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.548 |
| walker |  | 7794 | 16 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 10, sub: 0, line: 105 } |  |  | 0.548 |
| walker |  | 7820 | 26 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 8, sub: 0, line: 99 } |  |  | 0.549 |
| walker |  | 7846 | 26 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 9, sub: 0, line: 102 } |  |  | 0.550 |
| walker |  | 7868 | 22 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 1, sub: 0, line: 19 } |  |  | 0.550 |
| ns | 7894 |  | 413 | object() implementation, including the mask special case | 6.2 | 3.6 | 0.533 |
| walker |  | 8037 | 169 | Code::CodeKey { rung: Names, file: src/structs/types.ts, decl: 0, sub: 1, line: 0 } |  |  | 0.543 |
| walker |  | 8060 | 23 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 12, sub: 0, line: 144 } |  |  | 0.543 |
| ns | 8085 |  | 191 | refine() and define() bodies - the extension points | 6.3 | 2.4 | 0.536 |
| walker |  | 8100 | 40 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 14, sub: 0, line: 172 } |  |  | 0.536 |
| ns | 8199 |  | 114 | coerce() and trimmed() bodies | 6.4 | 2.5 | 0.532 |
| walker |  | 8294 | 194 | Code::CodeKey { rung: Names, file: src/structs/types.ts, decl: 0, sub: 2, line: 0 } |  |  | 0.547 |
| walker |  | 8313 | 19 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 27, sub: 0, line: 295 } |  |  | 0.549 |
| walker |  | 8344 | 31 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 21, sub: 0, line: 226 } |  |  | 0.553 |
| walker |  | 8367 | 23 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 5, sub: 0, line: 60 } |  |  | 0.553 |
| walker |  | 8390 | 23 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 6, sub: 0, line: 70 } |  |  | 0.554 |
| walker |  | 8413 | 23 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 11, sub: 0, line: 131 } |  |  | 0.554 |
| walker |  | 8436 | 23 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 13, sub: 0, line: 159 } |  |  | 0.555 |
| ns | 8458 |  | 259 | npm scripts | 7.1 | 1.1 | 0.561 |
| walker |  | 8459 | 23 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 23, sub: 0, line: 258 } |  |  | 0.561 |
| walker |  | 8482 | 23 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 25, sub: 0, line: 278 } |  |  | 0.562 |
| ns | 8629 |  | 171 | Changelog: recent release headings and the file's extent | 7.2 |  | 0.558 |
| walker |  | 8657 | 175 | Code::CodeKey { rung: Names, file: src/structs/types.ts, decl: 0, sub: 3, line: 0 } |  |  | 0.580 |
| walker |  | 8679 | 22 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 37, sub: 0, line: 492 } |  |  | 0.580 |
| walker |  | 8713 | 34 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 30, sub: 0, line: 367 } |  |  | 0.580 |
| walker |  | 8747 | 34 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 36, sub: 0, line: 456 } |  |  | 0.580 |
| ns | 8778 |  | 149 | The 2.0.0 release notes: breaking changes and fixes | 7.3 | 7.2 | 0.578 |
| walker |  | 8782 | 35 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 38, sub: 0, line: 522 } |  |  | 0.578 |
| walker |  | 8805 | 23 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 35, sub: 0, line: 442 } |  |  | 0.578 |
| walker |  | 8831 | 26 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 29, sub: 0, line: 351 } |  |  | 0.578 |
| walker |  | 8858 | 27 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 12, sub: 0, line: 144 } |  |  | 0.579 |
| walker |  | 8885 | 27 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 14, sub: 0, line: 172 } |  |  | 0.580 |
| walker |  | 8912 | 27 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 24, sub: 0, line: 266 } |  |  | 0.581 |
| walker |  | 8939 | 27 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 38, sub: 0, line: 522 } |  |  | 0.581 |
| ns | 8957 |  | 179 | TypeScript compiler configuration | 7.4 |  | 0.575 |
| walker |  | 8969 | 30 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 39, sub: 0, line: 574 } |  |  | 0.576 |
| walker |  | 9000 | 31 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 15, sub: 0, line: 200 } |  |  | 0.578 |
| walker |  | 9042 | 42 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 32, sub: 0, line: 413 } |  |  | 0.579 |
| walker |  | 9085 | 43 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 20, sub: 0, line: 225 } |  |  | 0.581 |
| walker |  | 9130 | 45 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 36, sub: 0, line: 456 } |  |  | 0.584 |
| ns | 9207 |  | 250 | Rollup build and JSR publish config | 7.5 |  | 0.575 |
| walker |  | 9350 | 220 | Code::CodeKey { rung: Names, file: src/structs/utilities.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.587 |
| ns | 9368 |  | 161 | CI matrix and dependency automation | 7.6 |  | 0.581 |
| walker |  | 9383 | 33 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 8, sub: 0, line: 106 } |  |  | 0.581 |
| walker |  | 9420 | 37 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 11, sub: 0, line: 197 } |  |  | 0.581 |
| walker |  | 9458 | 38 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 7, sub: 0, line: 80 } |  |  | 0.581 |
| walker |  | 9501 | 43 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 12, sub: 0, line: 221 } |  |  | 0.581 |
| ns | 9521 |  | 153 | Remaining package.json keys: entry points and publish metadata | 7.7 | 1.1 | 0.582 |
| walker |  | 9545 | 44 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 10, sub: 0, line: 171 } |  |  | 0.582 |
| walker |  | 9590 | 45 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 1, sub: 0, line: 17 } |  |  | 0.582 |
| ns | 9599 |  | 78 | Formatting and docs-site configuration | 7.8 |  | 0.579 |
| walker |  | 9692 | 102 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 2, sub: 0, line: 21 } |  |  | 0.579 |
| ns | 9718 |  | 119 | ESLint configuration head | 7.9 |  | 0.575 |
| walker |  | 9842 | 150 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 3, sub: 0, line: 30 } |  |  | 0.575 |
| ns | 9847 |  | 129 | Examples package manifest and run instructions | 7.10 |  | 0.572 |
