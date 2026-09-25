Score(3000)=0.483 I=0.783 C=0.297 ns_rows≤3K=20/60 grid(1000/1442/2080/3000/4327/6240/9000)=0.577/0.556/0.500/0.483/0.473/0.642/0.592

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
| walker |  | 404 | 30 | Markdown::HeadingsOutline { file: docs/summary.md } |  |  | 0.871 |
| ns | 416 |  | 166 | Readme lede: what Superstruct is and why it exists | 1.5 |  | 0.845 |
| walker |  | 422 | 18 | Markdown::HeadingsOutline { file: docs/resources/links.md } |  |  | 0.845 |
| walker |  | 619 | 197 | Markdown::ReadmeHeadline { file: Readme.md } |  |  | 0.854 |
| ns | 638 |  | 222 | The Struct class: doc comment and its six fields | 1.6 |  | 0.705 |
| walker |  | 673 | 54 | Markdown::HeadingsOutline { file: Readme.md } |  |  | 0.706 |
| walker |  | 739 | 66 | Markdown::Prelude { file: Readme.md } |  |  | 0.725 |
| walker |  | 767 | 28 | Fs::DirListing { dir: test } |  |  | 0.726 |
| walker |  | 786 | 19 | Markdown::Section { file: Readme.md, section_index: 10, keeps_default_concavity: false } |  |  | 0.726 |
| ns | 816 |  | 178 | Core API signatures: assert / create / is / mask / validate | 1.7 |  | 0.657 |
| walker |  | 862 | 76 | Json::Entry { file: package.json } |  |  | 0.658 |
| ns | 971 |  | 155 | Readme canonical usage snippet | 1.8 |  | 0.577 |
| walker |  | 993 | 131 | Markdown::ReadmeHeadline { file: docs/readme.md } |  |  | 0.577 |
| walker |  | 1059 | 66 | Markdown::Prelude { file: docs/readme.md } |  |  | 0.577 |
| ns | 1066 |  | 95 | src/struct.ts symbol roster: methods and top-level helpers | 2.1 |  | 0.539 |
| walker |  | 1114 | 55 | Fs::DirListing { dir: examples } |  |  | 0.540 |
| walker |  | 1139 | 25 | Fs::DirListing { dir: test/api } |  |  | 0.542 |
| ns | 1318 |  | 252 | Complete roster of the 25 type structs | 2.2 |  | 0.469 |
| walker |  | 1372 | 233 | Markdown::Section { file: Readme.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.584 |
| ns | 1420 |  | 102 | Complete roster of src/structs/utilities.ts | 2.3 |  | 0.556 |
| walker |  | 1451 | 79 | Code::CodeKey { rung: Names, file: src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.600 |
| ns | 1503 |  | 83 | Complete roster of src/structs/refinements.ts | 2.4 |  | 0.579 |
| ns | 1537 |  | 34 | Complete roster of src/structs/coercions.ts | 2.5 |  | 0.571 |
| walker |  | 1576 | 125 | Fs::DirListing { dir: test/validation } |  |  | 0.577 |
| ns | 1651 |  | 114 | Failure - the shape of every validation failure | 2.6 |  | 0.551 |
| walker |  | 1759 | 183 | Fs::DirListing { dir: test/typings } |  |  | 0.556 |
| walker |  | 1778 | 19 | Markdown::Section { file: docs/resources/links.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.556 |
| walker |  | 1810 | 32 | Markdown::HeadingsOutline { file: docs/guides/05-handling-errors.md } |  |  | 0.556 |
| ns | 1829 |  | 178 | StructError class: doc, fields, and its early-exit contract | 2.7 |  | 0.526 |
| walker |  | 1844 | 34 | Markdown::HeadingsOutline { file: docs/guides/03-coercing-data.md } |  |  | 0.526 |
| walker |  | 1878 | 34 | Markdown::HeadingsOutline { file: docs/guides/04-refining-validation.md } |  |  | 0.526 |
| walker |  | 1914 | 36 | Markdown::HeadingsOutline { file: docs/guides/01-getting-started.md } |  |  | 0.526 |
| ns | 2037 |  | 208 | Exported type vocabulary of src/struct.ts | 2.8 |  | 0.500 |
| ns | 2140 |  | 103 | Function roster of src/utils.ts | 2.9 |  | 0.486 |
| walker |  | 2240 | 326 | Json::Scripts { file: package.json } |  |  | 0.487 |
| ns | 2378 |  | 238 | Type-level utility roster of src/utils.ts | 2.10 |  | 0.457 |
| walker |  | 2473 | 233 | Markdown::Section { file: docs/readme.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.457 |
| walker |  | 2517 | 44 | Markdown::HeadingsOutline { file: docs/guides/06-using-typescript.md } |  |  | 0.458 |
| walker |  | 2544 | 27 | Code::CodeKey { rung: Names, file: src/error.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.459 |
| ns | 2574 |  | 196 | What each type struct does, part 1 (any ... literal) | 3.1 | 2.2 | 0.445 |
| walker |  | 2625 | 81 | Code::CodeKey { rung: Decl, file: src/error.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.474 |
| walker |  | 2738 | 113 | Code::CodeKey { rung: Decl, file: src/error.ts, decl: 2, sub: 0, line: 25 } |  |  | 0.490 |
| walker |  | 2764 | 26 | Code::CodeKey { rung: Doc, file: src/error.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.506 |
| walker |  | 2804 | 40 | Markdown::HeadingsOutline { file: docs/reference/errors.md } |  |  | 0.506 |
| ns | 2916 |  | 342 | What each type struct does, part 2 (map ... unknown) | 3.2 | 2.2 | 0.483 |
| walker |  | 3105 | 301 | Json::IdentityMeta { file: package.json } |  |  | 0.497 |
| ns | 3144 |  | 228 | What each utility struct does | 3.3 | 2.3 | 0.480 |
| walker |  | 3157 | 52 | Markdown::HeadingsOutline { file: docs/guides/02-validating-data.md } |  |  | 0.481 |
| walker |  | 3199 | 42 | Markdown::HeadingsOutline { file: docs/reference/typescript.md } |  |  | 0.481 |
| walker |  | 3226 | 27 | Markdown::Section { file: docs/reference/typescript.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.481 |
| walker |  | 3271 | 45 | Markdown::HeadingsOutline { file: docs/reference/coercions.md } |  |  | 0.481 |
| ns | 3300 |  | 156 | What each refinement does | 3.4 | 2.4 | 0.473 |
| walker |  | 3308 | 37 | Markdown::Section { file: docs/guides/02-validating-data.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.473 |
| walker |  | 3346 | 38 | Markdown::Section { file: docs/reference/errors.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.473 |
| ns | 3411 |  | 111 | What each coercion does - and the create() gotcha | 3.5 | 2.5 | 0.468 |
| walker |  | 3457 | 111 | Markdown::Section { file: Readme.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.468 |
| walker |  | 3690 | 233 | Code::CodeKey { rung: Names, file: src/struct.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.499 |
| ns | 3716 |  | 305 | Public overload declarations of the six overloaded factories | 3.6 | 2.2 | 0.476 |
| walker |  | 3724 | 34 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 13, sub: 0, line: 221 } |  |  | 0.489 |
| walker |  | 3761 | 37 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 9, sub: 0, line: 139 } |  |  | 0.489 |
| walker |  | 3798 | 37 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 10, sub: 0, line: 157 } |  |  | 0.489 |
| walker |  | 3838 | 40 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 8, sub: 0, line: 123 } |  |  | 0.489 |
| walker |  | 3879 | 41 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 16, sub: 0, line: 243 } |  |  | 0.507 |
| ns | 3959 |  | 243 | Struct method semantics: mask recursion and validate options | 3.7 | 2.1 | 0.495 |
| walker |  | 3962 | 83 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 12, sub: 0, line: 185 } |  |  | 0.495 |
| walker |  | 3985 | 23 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 11, sub: 0, line: 175 } |  |  | 0.495 |
| walker |  | 4008 | 23 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 16, sub: 0, line: 243 } |  |  | 0.495 |
| walker |  | 4033 | 25 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 18, sub: 0, line: 259 } |  |  | 0.495 |
| walker |  | 4060 | 27 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 14, sub: 0, line: 231 } |  |  | 0.495 |
| walker |  | 4087 | 27 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 15, sub: 0, line: 237 } |  |  | 0.495 |
| walker |  | 4115 | 28 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 8, sub: 0, line: 123 } |  |  | 0.495 |
| walker |  | 4143 | 28 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 17, sub: 0, line: 253 } |  |  | 0.495 |
| ns | 4149 |  | 190 | What each src/utils.ts helper does | 3.8 | 2.9 | 0.484 |
| walker |  | 4172 | 29 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 9, sub: 0, line: 139 } |  |  | 0.484 |
| walker |  | 4202 | 30 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 10, sub: 0, line: 157 } |  |  | 0.484 |
| walker |  | 4241 | 39 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 19, sub: 0, line: 266 } |  |  | 0.484 |
| ns | 4298 |  | 149 | run() signature and per-call context setup | 4.1 | 2.9 | 0.473 |
| ns | 4427 |  | 129 | run() stage 1: coercion, then the struct's own validator | 4.2 | 4.1 | 0.463 |
| walker |  | 4483 | 242 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 1, sub: 0, line: 10 } |  |  | 0.509 |
| walker |  | 4555 | 72 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 2, sub: 0, line: 22 } |  |  | 0.509 |
| walker |  | 4627 | 72 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 7, sub: 0, line: 107 } |  |  | 0.509 |
| walker |  | 4655 | 28 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 5, sub: 0, line: 83 } |  |  | 0.509 |
| walker |  | 4688 | 33 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 3, sub: 0, line: 67 } |  |  | 0.509 |
| walker |  | 4721 | 33 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 4, sub: 0, line: 75 } |  |  | 0.509 |
| ns | 4759 |  | 332 | run() stage 2: recursive descent over entries | 4.3 | 4.1 | 0.486 |
| walker |  | 4767 | 46 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 12, sub: 0, line: 185 } |  |  | 0.486 |
| walker |  | 4836 | 69 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 1, sub: 0, line: 10 } |  |  | 0.509 |
| ns | 4882 |  | 123 | run() stage 3: refiners and the success yield | 4.4 | 4.1 | 0.500 |
| walker |  | 4905 | 69 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 6, sub: 0, line: 93 } |  |  | 0.502 |
| walker |  | 4936 | 31 | Markdown::Section { file: docs/summary.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.502 |
| ns | 5054 |  | 172 | validate() body: how one failure becomes a StructError | 4.5 | 2.1 | 0.489 |
| walker |  | 5174 | 238 | Code::CodeKey { rung: Names, file: src/utils.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.509 |
| walker |  | 5194 | 20 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 11, sub: 0, line: 218 } |  |  | 0.509 |
| walker |  | 5244 | 50 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 7, sub: 0, line: 106 } |  |  | 0.509 |
| ns | 5265 |  | 211 | Struct constructor: defaults and failure wrapping | 4.6 | 1.6 | 0.492 |
| walker |  | 5294 | 50 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 9, sub: 0, line: 202 } |  |  | 0.492 |
| walker |  | 5348 | 54 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 6, sub: 0, line: 67 } |  |  | 0.492 |
| walker |  | 5450 | 102 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 8, sub: 0, line: 130 } |  |  | 0.508 |
| ns | 5456 |  | 191 | StructError constructor: message, cause and failure caching | 4.7 | 2.7 | 0.498 |
| walker |  | 5469 | 19 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 11, sub: 0, line: 218 } |  |  | 0.498 |
| walker |  | 5492 | 23 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 4, sub: 0, line: 45 } |  |  | 0.499 |
| walker |  | 5516 | 24 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 1, sub: 0, line: 16 } |  |  | 0.499 |
| walker |  | 5540 | 24 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 3, sub: 0, line: 32 } |  |  | 0.500 |
| walker |  | 5564 | 24 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 6, sub: 0, line: 67 } |  |  | 0.501 |
| walker |  | 5588 | 24 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 9, sub: 0, line: 202 } |  |  | 0.501 |
| ns | 5596 |  | 140 | Complete docs/ tree listing | 5.1 |  | 0.532 |
| walker |  | 5613 | 25 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 2, sub: 0, line: 24 } |  |  | 0.533 |
| walker |  | 5638 | 25 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 7, sub: 0, line: 106 } |  |  | 0.535 |
| ns | 5649 |  | 53 | Complete test/ and test/api/ listings | 5.2 |  | 0.547 |
| walker |  | 5663 | 25 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 10, sub: 0, line: 212 } |  |  | 0.547 |
| walker |  | 5699 | 36 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 12, sub: 0, line: 227 } |  |  | 0.547 |
| walker |  | 5743 | 44 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 8, sub: 0, line: 130 } |  |  | 0.551 |
| ns | 5774 |  | 125 | All 41 validation-fixture kind directories | 5.3 |  | 0.585 |
| walker |  | 5793 | 50 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 5, sub: 0, line: 58 } |  |  | 0.590 |
| ns | 5829 |  | 55 | Complete examples/ listing | 5.4 |  | 0.598 |
| ns | 5906 |  | 77 | One kind directory listed in full, as the case-naming exemplar | 5.5 |  | 0.589 |
| ns | 6089 |  | 183 | The 45 type-level test files | 5.6 |  | 0.616 |
| walker |  | 6102 | 309 | Code::CodeKey { rung: Names, file: src/utils.ts, decl: 0, sub: 1, line: 0 } |  |  | 0.642 |
| walker |  | 6115 | 13 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 20, sub: 0, line: 300 } |  |  | 0.642 |
| walker |  | 6139 | 24 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 18, sub: 0, line: 283 } |  |  | 0.642 |
| walker |  | 6164 | 25 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 21, sub: 0, line: 307 } |  |  | 0.642 |
| walker |  | 6189 | 25 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 23, sub: 0, line: 324 } |  |  | 0.642 |
| walker |  | 6224 | 35 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 14, sub: 0, line: 242 } |  |  | 0.642 |
| walker |  | 6266 | 42 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 19, sub: 0, line: 291 } |  |  | 0.642 |
| walker |  | 6308 | 42 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 22, sub: 0, line: 315 } |  |  | 0.642 |
| ns | 6316 |  | 227 | Every heading in the six guides | 5.7 |  | 0.651 |
| walker |  | 6353 | 45 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 16, sub: 0, line: 267 } |  |  | 0.651 |
| ns | 6379 |  | 63 | Readme section map | 5.8 |  | 0.654 |
| walker |  | 6403 | 50 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 13, sub: 0, line: 233 } |  |  | 0.654 |
| walker |  | 6477 | 74 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 28, sub: 0, line: 394 } |  |  | 0.654 |
| ns | 6519 |  | 140 | Doc anchors the source rosters do not cover | 5.9 |  | 0.650 |
| walker |  | 6589 | 112 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 15, sub: 0, line: 251 } |  |  | 0.650 |
| walker |  | 6608 | 19 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 17, sub: 0, line: 277 } |  |  | 0.650 |
| walker |  | 6629 | 21 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 15, sub: 0, line: 251 } |  |  | 0.650 |
| walker |  | 6650 | 21 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 16, sub: 0, line: 267 } |  |  | 0.650 |
| walker |  | 6671 | 21 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 26, sub: 0, line: 379 } |  |  | 0.650 |
| ns | 6690 |  | 171 | The data-driven fixture runner | 5.10 |  | 0.644 |
| walker |  | 6693 | 22 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 13, sub: 0, line: 233 } |  |  | 0.644 |
| walker |  | 6715 | 22 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 14, sub: 0, line: 242 } |  |  | 0.644 |
| walker |  | 6737 | 22 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 18, sub: 0, line: 283 } |  |  | 0.644 |
| walker |  | 6759 | 22 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 27, sub: 0, line: 385 } |  |  | 0.644 |
| walker |  | 6782 | 23 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 21, sub: 0, line: 307 } |  |  | 0.644 |
| walker |  | 6806 | 24 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 23, sub: 0, line: 324 } |  |  | 0.644 |
| walker |  | 6831 | 25 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 22, sub: 0, line: 315 } |  |  | 0.644 |
| walker |  | 6857 | 26 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 19, sub: 0, line: 291 } |  |  | 0.644 |
| walker |  | 6885 | 28 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 20, sub: 0, line: 300 } |  |  | 0.644 |
| ns | 6891 |  | 201 | Runner assertions: output-style vs failures-style fixtures | 5.11 | 5.10 | 0.634 |
| walker |  | 6948 | 63 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 28, sub: 0, line: 394 } |  |  | 0.634 |
| ns | 7164 |  | 273 | A failures-style and an output-style fixture, in full | 5.12 | 5.10 | 0.617 |
| ns | 7295 |  | 131 | The typings harness and one typings test | 5.13 | 5.6 | 0.611 |
| walker |  | 7370 | 422 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 25, sub: 0, line: 334 } |  |  | 0.611 |
| walker |  | 7391 | 21 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 25, sub: 0, line: 334 } |  |  | 0.611 |
| ns | 7481 |  | 186 | toFailure(): result normalisation and default message text | 6.1 | 2.9 | 0.602 |
| walker |  | 7488 | 97 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 13, sub: 0, line: 221 } |  |  | 0.607 |
| walker |  | 7550 | 62 | Markdown::HeadingsOutline { file: docs/reference/core.md } |  |  | 0.608 |
| walker |  | 7657 | 107 | Code::CodeKey { rung: Doc, file: src/error.ts, decl: 2, sub: 0, line: 25 } |  |  | 0.622 |
| walker |  | 7833 | 176 | Markdown::Section { file: Readme.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.622 |
| ns | 7894 |  | 413 | object() implementation, including the mask special case | 6.2 | 3.6 | 0.603 |
| ns | 8085 |  | 191 | refine() and define() bodies - the extension points | 6.3 | 2.4 | 0.594 |
| walker |  | 8090 | 257 | Markdown::Section { file: License.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.594 |
| ns | 8199 |  | 114 | coerce() and trimmed() bodies | 6.4 | 2.5 | 0.590 |
| walker |  | 8285 | 195 | Markdown::Section { file: Readme.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.590 |
| walker |  | 8420 | 135 | Markdown::Section { file: Readme.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.590 |
| ns | 8458 |  | 259 | npm scripts | 7.1 | 1.1 | 0.596 |
| walker |  | 8483 | 63 | Markdown::Section { file: docs/reference/coercions.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.596 |
| walker |  | 8562 | 79 | Markdown::HeadingsOutline { file: docs/reference/utilities.md } |  |  | 0.596 |
| walker |  | 8592 | 30 | Markdown::Section { file: docs/reference/utilities.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.596 |
| ns | 8629 |  | 171 | Changelog: recent release headings and the file's extent | 7.2 |  | 0.592 |
| walker |  | 8693 | 101 | Code::CodeKey { rung: Names, file: src/structs/refinements.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.600 |
| walker |  | 8726 | 33 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 5, sub: 0, line: 93 } |  |  | 0.600 |
| walker |  | 8770 | 44 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 7, sub: 0, line: 146 } |  |  | 0.601 |
| ns | 8778 |  | 149 | The 2.0.0 release notes: breaking changes and fixes | 7.3 | 7.2 | 0.598 |
| walker |  | 8818 | 48 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 1, sub: 0, line: 8 } |  |  | 0.598 |
| walker |  | 8866 | 48 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 4, sub: 0, line: 77 } |  |  | 0.598 |
| walker |  | 8923 | 57 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 2, sub: 0, line: 33 } |  |  | 0.598 |
| ns | 8957 |  | 179 | TypeScript compiler configuration | 7.4 |  | 0.592 |
| walker |  | 8980 | 57 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 3, sub: 0, line: 55 } |  |  | 0.592 |
| walker |  | 9042 | 62 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 6, sub: 0, line: 109 } |  |  | 0.592 |
| walker |  | 9066 | 24 | Code::CodeKey { rung: Doc, file: src/structs/refinements.ts, decl: 5, sub: 0, line: 93 } |  |  | 0.592 |
| walker |  | 9092 | 26 | Code::CodeKey { rung: Doc, file: src/structs/refinements.ts, decl: 2, sub: 0, line: 33 } |  |  | 0.593 |
| walker |  | 9118 | 26 | Code::CodeKey { rung: Doc, file: src/structs/refinements.ts, decl: 3, sub: 0, line: 55 } |  |  | 0.593 |
| walker |  | 9147 | 29 | Code::CodeKey { rung: Doc, file: src/structs/refinements.ts, decl: 1, sub: 0, line: 8 } |  |  | 0.594 |
| walker |  | 9176 | 29 | Code::CodeKey { rung: Doc, file: src/structs/refinements.ts, decl: 4, sub: 0, line: 77 } |  |  | 0.595 |
| ns | 9207 |  | 250 | Rollup build and JSR publish config | 7.5 |  | 0.586 |
| walker |  | 9224 | 48 | Code::CodeKey { rung: Doc, file: src/structs/refinements.ts, decl: 6, sub: 0, line: 109 } |  |  | 0.588 |
| walker |  | 9349 | 125 | Markdown::Section { file: docs/summary.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.588 |
| ns | 9368 |  | 161 | CI matrix and dependency automation | 7.6 |  | 0.583 |
| walker |  | 9431 | 82 | Markdown::HeadingsOutline { file: docs/reference/refinements.md } |  |  | 0.584 |
| walker |  | 9484 | 53 | Markdown::Section { file: docs/reference/refinements.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.584 |
| ns | 9521 |  | 153 | Remaining package.json keys: entry points and publish metadata | 7.7 | 1.1 | 0.590 |
| ns | 9599 |  | 78 | Formatting and docs-site configuration | 7.8 |  | 0.587 |
| walker |  | 9619 | 135 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 7, sub: 0, line: 107 } |  |  | 0.594 |
| ns | 9718 |  | 119 | ESLint configuration head | 7.9 |  | 0.590 |
| ns | 9847 |  | 129 | Examples package manifest and run instructions | 7.10 |  | 0.588 |
| walker |  | 9901 | 282 | Markdown::Section { file: Readme.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.588 |
