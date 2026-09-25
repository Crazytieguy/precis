Score(3000)=0.452 I=0.774 C=0.264 ns_rows≤3K=20/60 grid(1000/1442/2080/3000/4327/6240/9000)=0.577/0.556/0.499/0.452/0.456/0.607/0.584

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
| ns | 1829 |  | 178 | StructError class: doc, fields, and its early-exit contract | 2.7 |  | 0.521 |
| walker |  | 1858 | 282 | Markdown::Section { file: Readme.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.521 |
| ns | 2037 |  | 208 | Exported type vocabulary of src/struct.ts | 2.8 |  | 0.495 |
| walker |  | 2041 | 183 | Fs::DirListing { dir: test/typings } |  |  | 0.499 |
| walker |  | 2060 | 19 | Markdown::Section { file: docs/resources/links.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.499 |
| walker |  | 2092 | 32 | Markdown::HeadingsOutline { file: docs/guides/05-handling-errors.md } |  |  | 0.499 |
| walker |  | 2126 | 34 | Markdown::HeadingsOutline { file: docs/guides/03-coercing-data.md } |  |  | 0.499 |
| ns | 2140 |  | 103 | Function roster of src/utils.ts | 2.9 |  | 0.485 |
| walker |  | 2160 | 34 | Markdown::HeadingsOutline { file: docs/guides/04-refining-validation.md } |  |  | 0.485 |
| walker |  | 2196 | 36 | Markdown::HeadingsOutline { file: docs/guides/01-getting-started.md } |  |  | 0.486 |
| ns | 2378 |  | 238 | Type-level utility roster of src/utils.ts | 2.10 |  | 0.457 |
| walker |  | 2522 | 326 | Json::Scripts { file: package.json } |  |  | 0.457 |
| ns | 2574 |  | 196 | What each type struct does, part 1 (any ... literal) | 3.1 | 2.2 | 0.444 |
| walker |  | 2755 | 233 | Markdown::Section { file: docs/readme.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.444 |
| walker |  | 2799 | 44 | Markdown::HeadingsOutline { file: docs/guides/06-using-typescript.md } |  |  | 0.444 |
| walker |  | 2826 | 27 | Code::CodeKey { rung: Names, file: src/error.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.445 |
| walker |  | 2907 | 81 | Code::CodeKey { rung: Decl, file: src/error.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.474 |
| ns | 2916 |  | 342 | What each type struct does, part 2 (map ... unknown) | 3.2 | 2.2 | 0.452 |
| walker |  | 3020 | 113 | Code::CodeKey { rung: Decl, file: src/error.ts, decl: 2, sub: 0, line: 25 } |  |  | 0.467 |
| walker |  | 3046 | 26 | Code::CodeKey { rung: Doc, file: src/error.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.482 |
| walker |  | 3086 | 40 | Markdown::HeadingsOutline { file: docs/reference/errors.md } |  |  | 0.483 |
| ns | 3144 |  | 228 | What each utility struct does | 3.3 | 2.3 | 0.466 |
| ns | 3300 |  | 156 | What each refinement does | 3.4 | 2.4 | 0.458 |
| walker |  | 3387 | 301 | Json::IdentityMeta { file: package.json } |  |  | 0.472 |
| ns | 3411 |  | 111 | What each coercion does - and the create() gotcha | 3.5 | 2.5 | 0.467 |
| walker |  | 3439 | 52 | Markdown::HeadingsOutline { file: docs/guides/02-validating-data.md } |  |  | 0.468 |
| walker |  | 3481 | 42 | Markdown::HeadingsOutline { file: docs/reference/typescript.md } |  |  | 0.468 |
| walker |  | 3508 | 27 | Markdown::Section { file: docs/reference/typescript.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.468 |
| ns | 3716 |  | 305 | Public overload declarations of the six overloaded factories | 3.6 | 2.2 | 0.446 |
| walker |  | 3790 | 282 | Markdown::Section { file: docs/readme.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.446 |
| walker |  | 3835 | 45 | Markdown::HeadingsOutline { file: docs/reference/coercions.md } |  |  | 0.446 |
| walker |  | 3872 | 37 | Markdown::Section { file: docs/guides/02-validating-data.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.446 |
| walker |  | 3910 | 38 | Markdown::Section { file: docs/reference/errors.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.446 |
| ns | 3959 |  | 243 | Struct method semantics: mask recursion and validate options | 3.7 | 2.1 | 0.435 |
| walker |  | 4021 | 111 | Markdown::Section { file: Readme.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.435 |
| ns | 4149 |  | 190 | What each src/utils.ts helper does | 3.8 | 2.9 | 0.426 |
| walker |  | 4254 | 233 | Code::CodeKey { rung: Names, file: src/struct.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.454 |
| walker |  | 4288 | 34 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 13, sub: 0, line: 221 } |  |  | 0.467 |
| ns | 4298 |  | 149 | run() signature and per-call context setup | 4.1 | 2.9 | 0.456 |
| walker |  | 4325 | 37 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 9, sub: 0, line: 139 } |  |  | 0.456 |
| walker |  | 4362 | 37 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 10, sub: 0, line: 157 } |  |  | 0.456 |
| walker |  | 4402 | 40 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 8, sub: 0, line: 123 } |  |  | 0.456 |
| ns | 4427 |  | 129 | run() stage 1: coercion, then the struct's own validator | 4.2 | 4.1 | 0.446 |
| walker |  | 4443 | 41 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 16, sub: 0, line: 243 } |  |  | 0.463 |
| walker |  | 4526 | 83 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 12, sub: 0, line: 185 } |  |  | 0.463 |
| walker |  | 4549 | 23 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 11, sub: 0, line: 175 } |  |  | 0.463 |
| walker |  | 4572 | 23 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 16, sub: 0, line: 243 } |  |  | 0.463 |
| walker |  | 4597 | 25 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 18, sub: 0, line: 259 } |  |  | 0.463 |
| walker |  | 4624 | 27 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 14, sub: 0, line: 231 } |  |  | 0.463 |
| walker |  | 4651 | 27 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 15, sub: 0, line: 237 } |  |  | 0.463 |
| walker |  | 4679 | 28 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 8, sub: 0, line: 123 } |  |  | 0.463 |
| walker |  | 4707 | 28 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 17, sub: 0, line: 253 } |  |  | 0.463 |
| walker |  | 4736 | 29 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 9, sub: 0, line: 139 } |  |  | 0.463 |
| ns | 4759 |  | 332 | run() stage 2: recursive descent over entries | 4.3 | 4.1 | 0.443 |
| walker |  | 4766 | 30 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 10, sub: 0, line: 157 } |  |  | 0.443 |
| walker |  | 4805 | 39 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 19, sub: 0, line: 266 } |  |  | 0.443 |
| ns | 4882 |  | 123 | run() stage 3: refiners and the success yield | 4.4 | 4.1 | 0.434 |
| walker |  | 5047 | 242 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 1, sub: 0, line: 10 } |  |  | 0.477 |
| ns | 5054 |  | 172 | validate() body: how one failure becomes a StructError | 4.5 | 2.1 | 0.465 |
| walker |  | 5119 | 72 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 2, sub: 0, line: 22 } |  |  | 0.465 |
| walker |  | 5191 | 72 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 7, sub: 0, line: 107 } |  |  | 0.465 |
| walker |  | 5219 | 28 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 5, sub: 0, line: 83 } |  |  | 0.465 |
| walker |  | 5252 | 33 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 3, sub: 0, line: 67 } |  |  | 0.465 |
| ns | 5265 |  | 211 | Struct constructor: defaults and failure wrapping | 4.6 | 1.6 | 0.450 |
| walker |  | 5285 | 33 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 4, sub: 0, line: 75 } |  |  | 0.450 |
| walker |  | 5331 | 46 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 12, sub: 0, line: 185 } |  |  | 0.450 |
| walker |  | 5400 | 69 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 1, sub: 0, line: 10 } |  |  | 0.472 |
| ns | 5456 |  | 191 | StructError constructor: message, cause and failure caching | 4.7 | 2.7 | 0.463 |
| walker |  | 5469 | 69 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 6, sub: 0, line: 93 } |  |  | 0.465 |
| walker |  | 5500 | 31 | Markdown::Section { file: docs/summary.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.465 |
| ns | 5596 |  | 140 | Complete docs/ tree listing | 5.1 |  | 0.500 |
| ns | 5649 |  | 53 | Complete test/ and test/api/ listings | 5.2 |  | 0.513 |
| walker |  | 5738 | 238 | Code::CodeKey { rung: Names, file: src/utils.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.529 |
| walker |  | 5758 | 20 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 11, sub: 0, line: 218 } |  |  | 0.529 |
| ns | 5774 |  | 125 | All 41 validation-fixture kind directories | 5.3 |  | 0.565 |
| walker |  | 5808 | 50 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 7, sub: 0, line: 106 } |  |  | 0.565 |
| ns | 5829 |  | 55 | Complete examples/ listing | 5.4 |  | 0.573 |
| walker |  | 5858 | 50 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 9, sub: 0, line: 202 } |  |  | 0.573 |
| ns | 5906 |  | 77 | One kind directory listed in full, as the case-naming exemplar | 5.5 |  | 0.565 |
| walker |  | 5912 | 54 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 6, sub: 0, line: 67 } |  |  | 0.565 |
| walker |  | 6014 | 102 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 8, sub: 0, line: 130 } |  |  | 0.576 |
| walker |  | 6033 | 19 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 11, sub: 0, line: 218 } |  |  | 0.576 |
| walker |  | 6056 | 23 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 4, sub: 0, line: 45 } |  |  | 0.576 |
| walker |  | 6080 | 24 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 1, sub: 0, line: 16 } |  |  | 0.576 |
| ns | 6089 |  | 183 | The 45 type-level test files | 5.6 |  | 0.604 |
| walker |  | 6104 | 24 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 3, sub: 0, line: 32 } |  |  | 0.604 |
| walker |  | 6128 | 24 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 6, sub: 0, line: 67 } |  |  | 0.605 |
| walker |  | 6152 | 24 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 9, sub: 0, line: 202 } |  |  | 0.605 |
| walker |  | 6177 | 25 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 2, sub: 0, line: 24 } |  |  | 0.606 |
| walker |  | 6202 | 25 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 7, sub: 0, line: 106 } |  |  | 0.607 |
| walker |  | 6227 | 25 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 10, sub: 0, line: 212 } |  |  | 0.607 |
| walker |  | 6263 | 36 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 12, sub: 0, line: 227 } |  |  | 0.607 |
| walker |  | 6307 | 44 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 8, sub: 0, line: 130 } |  |  | 0.611 |
| ns | 6316 |  | 227 | Every heading in the six guides | 5.7 |  | 0.621 |
| walker |  | 6357 | 50 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 5, sub: 0, line: 58 } |  |  | 0.626 |
| ns | 6379 |  | 63 | Readme section map | 5.8 |  | 0.629 |
| ns | 6519 |  | 140 | Doc anchors the source rosters do not cover | 5.9 |  | 0.626 |
| walker |  | 6666 | 309 | Code::CodeKey { rung: Names, file: src/utils.ts, decl: 0, sub: 1, line: 0 } |  |  | 0.650 |
| walker |  | 6679 | 13 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 20, sub: 0, line: 300 } |  |  | 0.650 |
| ns | 6690 |  | 171 | The data-driven fixture runner | 5.10 |  | 0.644 |
| walker |  | 6703 | 24 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 18, sub: 0, line: 283 } |  |  | 0.644 |
| walker |  | 6728 | 25 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 21, sub: 0, line: 307 } |  |  | 0.644 |
| walker |  | 6753 | 25 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 23, sub: 0, line: 324 } |  |  | 0.644 |
| walker |  | 6788 | 35 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 14, sub: 0, line: 242 } |  |  | 0.644 |
| walker |  | 6830 | 42 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 19, sub: 0, line: 291 } |  |  | 0.644 |
| walker |  | 6872 | 42 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 22, sub: 0, line: 315 } |  |  | 0.644 |
| ns | 6891 |  | 201 | Runner assertions: output-style vs failures-style fixtures | 5.11 | 5.10 | 0.634 |
| walker |  | 6917 | 45 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 16, sub: 0, line: 267 } |  |  | 0.634 |
| walker |  | 6967 | 50 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 13, sub: 0, line: 233 } |  |  | 0.634 |
| walker |  | 7041 | 74 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 28, sub: 0, line: 394 } |  |  | 0.634 |
| walker |  | 7153 | 112 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 15, sub: 0, line: 251 } |  |  | 0.634 |
| ns | 7164 |  | 273 | A failures-style and an output-style fixture, in full | 5.12 | 5.10 | 0.617 |
| walker |  | 7172 | 19 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 17, sub: 0, line: 277 } |  |  | 0.617 |
| walker |  | 7193 | 21 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 15, sub: 0, line: 251 } |  |  | 0.617 |
| walker |  | 7214 | 21 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 16, sub: 0, line: 267 } |  |  | 0.617 |
| walker |  | 7235 | 21 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 26, sub: 0, line: 379 } |  |  | 0.617 |
| walker |  | 7257 | 22 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 13, sub: 0, line: 233 } |  |  | 0.617 |
| walker |  | 7279 | 22 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 14, sub: 0, line: 242 } |  |  | 0.617 |
| ns | 7295 |  | 131 | The typings harness and one typings test | 5.13 | 5.6 | 0.611 |
| walker |  | 7301 | 22 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 18, sub: 0, line: 283 } |  |  | 0.611 |
| walker |  | 7323 | 22 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 27, sub: 0, line: 385 } |  |  | 0.611 |
| walker |  | 7346 | 23 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 21, sub: 0, line: 307 } |  |  | 0.611 |
| walker |  | 7370 | 24 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 23, sub: 0, line: 324 } |  |  | 0.611 |
| walker |  | 7395 | 25 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 22, sub: 0, line: 315 } |  |  | 0.611 |
| walker |  | 7421 | 26 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 19, sub: 0, line: 291 } |  |  | 0.611 |
| walker |  | 7449 | 28 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 20, sub: 0, line: 300 } |  |  | 0.611 |
| ns | 7481 |  | 186 | toFailure(): result normalisation and default message text | 6.1 | 2.9 | 0.602 |
| walker |  | 7512 | 63 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 28, sub: 0, line: 394 } |  |  | 0.602 |
| ns | 7894 |  | 413 | object() implementation, including the mask special case | 6.2 | 3.6 | 0.583 |
| walker |  | 7934 | 422 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 25, sub: 0, line: 334 } |  |  | 0.583 |
| walker |  | 7955 | 21 | Code::CodeKey { rung: Doc, file: src/utils.ts, decl: 25, sub: 0, line: 334 } |  |  | 0.583 |
| walker |  | 8052 | 97 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 13, sub: 0, line: 221 } |  |  | 0.588 |
| ns | 8085 |  | 191 | refine() and define() bodies - the extension points | 6.3 | 2.4 | 0.580 |
| walker |  | 8114 | 62 | Markdown::HeadingsOutline { file: docs/reference/core.md } |  |  | 0.581 |
| ns | 8199 |  | 114 | coerce() and trimmed() bodies | 6.4 | 2.5 | 0.577 |
| walker |  | 8221 | 107 | Code::CodeKey { rung: Doc, file: src/error.ts, decl: 2, sub: 0, line: 25 } |  |  | 0.590 |
| walker |  | 8397 | 176 | Markdown::Section { file: Readme.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.590 |
| ns | 8458 |  | 259 | npm scripts | 7.1 | 1.1 | 0.596 |
| walker |  | 8592 | 195 | Markdown::Section { file: Readme.md, section_index: 4, keeps_default_concavity: true } |  |  | 0.596 |
| ns | 8629 |  | 171 | Changelog: recent release headings and the file's extent | 7.2 |  | 0.592 |
| walker |  | 8727 | 135 | Markdown::Section { file: Readme.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.592 |
| ns | 8778 |  | 149 | The 2.0.0 release notes: breaking changes and fixes | 7.3 | 7.2 | 0.590 |
| ns | 8957 |  | 179 | TypeScript compiler configuration | 7.4 |  | 0.584 |
| walker |  | 8984 | 257 | Markdown::Section { file: License.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.584 |
| walker |  | 9047 | 63 | Markdown::Section { file: docs/reference/coercions.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.584 |
| walker |  | 9126 | 79 | Markdown::HeadingsOutline { file: docs/reference/utilities.md } |  |  | 0.584 |
| walker |  | 9156 | 30 | Markdown::Section { file: docs/reference/utilities.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.584 |
| ns | 9207 |  | 250 | Rollup build and JSR publish config | 7.5 |  | 0.575 |
| walker |  | 9257 | 101 | Code::CodeKey { rung: Names, file: src/structs/refinements.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.582 |
| walker |  | 9290 | 33 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 5, sub: 0, line: 93 } |  |  | 0.582 |
| walker |  | 9334 | 44 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 7, sub: 0, line: 146 } |  |  | 0.583 |
| ns | 9368 |  | 161 | CI matrix and dependency automation | 7.6 |  | 0.578 |
| walker |  | 9382 | 48 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 1, sub: 0, line: 8 } |  |  | 0.578 |
| walker |  | 9430 | 48 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 4, sub: 0, line: 77 } |  |  | 0.578 |
| walker |  | 9487 | 57 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 2, sub: 0, line: 33 } |  |  | 0.578 |
| ns | 9521 |  | 153 | Remaining package.json keys: entry points and publish metadata | 7.7 | 1.1 | 0.584 |
| walker |  | 9544 | 57 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 3, sub: 0, line: 55 } |  |  | 0.584 |
| ns | 9599 |  | 78 | Formatting and docs-site configuration | 7.8 |  | 0.581 |
| walker |  | 9606 | 62 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 6, sub: 0, line: 109 } |  |  | 0.581 |
| walker |  | 9630 | 24 | Code::CodeKey { rung: Doc, file: src/structs/refinements.ts, decl: 5, sub: 0, line: 93 } |  |  | 0.581 |
| walker |  | 9656 | 26 | Code::CodeKey { rung: Doc, file: src/structs/refinements.ts, decl: 2, sub: 0, line: 33 } |  |  | 0.582 |
| walker |  | 9682 | 26 | Code::CodeKey { rung: Doc, file: src/structs/refinements.ts, decl: 3, sub: 0, line: 55 } |  |  | 0.582 |
| walker |  | 9711 | 29 | Code::CodeKey { rung: Doc, file: src/structs/refinements.ts, decl: 1, sub: 0, line: 8 } |  |  | 0.583 |
| ns | 9718 |  | 119 | ESLint configuration head | 7.9 |  | 0.579 |
| walker |  | 9740 | 29 | Code::CodeKey { rung: Doc, file: src/structs/refinements.ts, decl: 4, sub: 0, line: 77 } |  |  | 0.580 |
| walker |  | 9788 | 48 | Code::CodeKey { rung: Doc, file: src/structs/refinements.ts, decl: 6, sub: 0, line: 109 } |  |  | 0.582 |
| ns | 9847 |  | 129 | Examples package manifest and run instructions | 7.10 |  | 0.579 |
| walker |  | 9913 | 125 | Markdown::Section { file: docs/summary.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.579 |
| walker |  | 9995 | 82 | Markdown::HeadingsOutline { file: docs/reference/refinements.md } |  |  | 0.580 |
