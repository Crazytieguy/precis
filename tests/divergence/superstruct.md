Score(3000)=0.482 I=0.781 C=0.297 ns_rows≤3K=20/60 grid(1000/1442/2080/3000/4327/6240/9000)=0.639/0.499/0.564/0.482/0.428/0.591/0.593

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
| ns | 816 |  | 178 | Core API signatures: assert / create / is / mask / validate | 1.7 |  | 0.657 |
| walker |  | 818 | 79 | Code::CodeKey { rung: Names, file: src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.727 |
| walker |  | 846 | 28 | Fs::DirListing { dir: test } |  |  | 0.727 |
| walker |  | 865 | 19 | Markdown::Section { file: Readme.md, section_index: 10, keeps_default_concavity: false } |  |  | 0.727 |
| walker |  | 941 | 76 | Json::Entry { file: package.json } |  |  | 0.728 |
| ns | 971 |  | 155 | Readme canonical usage snippet | 1.8 |  | 0.639 |
| ns | 1066 |  | 95 | src/struct.ts symbol roster: methods and top-level helpers | 2.1 |  | 0.597 |
| walker |  | 1072 | 131 | Markdown::ReadmeHeadline { file: docs/readme.md } |  |  | 0.597 |
| walker |  | 1099 | 27 | Code::CodeKey { rung: Names, file: src/error.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.597 |
| walker |  | 1180 | 81 | Code::CodeKey { rung: Decl, file: src/error.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.600 |
| walker |  | 1246 | 66 | Markdown::Prelude { file: docs/readme.md } |  |  | 0.600 |
| ns | 1318 |  | 252 | Complete roster of the 25 type structs | 2.2 |  | 0.520 |
| walker |  | 1359 | 113 | Code::CodeKey { rung: Decl, file: src/error.ts, decl: 2, sub: 0, line: 25 } |  |  | 0.522 |
| walker |  | 1414 | 55 | Fs::DirListing { dir: examples } |  |  | 0.523 |
| ns | 1420 |  | 102 | Complete roster of src/structs/utilities.ts | 2.3 |  | 0.498 |
| walker |  | 1439 | 25 | Fs::DirListing { dir: test/api } |  |  | 0.499 |
| ns | 1503 |  | 83 | Complete roster of src/structs/refinements.ts | 2.4 |  | 0.481 |
| ns | 1537 |  | 34 | Complete roster of src/structs/coercions.ts | 2.5 |  | 0.475 |
| ns | 1651 |  | 114 | Failure - the shape of every validation failure | 2.6 |  | 0.492 |
| walker |  | 1672 | 233 | Markdown::Section { file: Readme.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.584 |
| walker |  | 1698 | 26 | Code::CodeKey { rung: Doc, file: src/error.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.604 |
| walker |  | 1823 | 125 | Fs::DirListing { dir: test/validation } |  |  | 0.610 |
| ns | 1829 |  | 178 | StructError class: doc, fields, and its early-exit contract | 2.7 |  | 0.594 |
| ns | 2037 |  | 208 | Exported type vocabulary of src/struct.ts | 2.8 |  | 0.564 |
| walker |  | 2105 | 282 | Markdown::Section { file: Readme.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.564 |
| ns | 2140 |  | 103 | Function roster of src/utils.ts | 2.9 |  | 0.548 |
| walker |  | 2288 | 183 | Fs::DirListing { dir: test/typings } |  |  | 0.553 |
| walker |  | 2307 | 19 | Markdown::Section { file: docs/resources/links.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.553 |
| walker |  | 2339 | 32 | Markdown::HeadingsOutline { file: docs/guides/05-handling-errors.md } |  |  | 0.553 |
| walker |  | 2373 | 34 | Markdown::HeadingsOutline { file: docs/guides/03-coercing-data.md } |  |  | 0.553 |
| ns | 2378 |  | 238 | Type-level utility roster of src/utils.ts | 2.10 |  | 0.520 |
| walker |  | 2407 | 34 | Markdown::HeadingsOutline { file: docs/guides/04-refining-validation.md } |  |  | 0.520 |
| walker |  | 2443 | 36 | Markdown::HeadingsOutline { file: docs/guides/01-getting-started.md } |  |  | 0.520 |
| ns | 2574 |  | 196 | What each type struct does, part 1 (any ... literal) | 3.1 | 2.2 | 0.505 |
| walker |  | 2769 | 326 | Json::Scripts { file: package.json } |  |  | 0.506 |
| ns | 2916 |  | 342 | What each type struct does, part 2 (map ... unknown) | 3.2 | 2.2 | 0.482 |
| walker |  | 3002 | 233 | Markdown::Section { file: docs/readme.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.482 |
| walker |  | 3046 | 44 | Markdown::HeadingsOutline { file: docs/guides/06-using-typescript.md } |  |  | 0.482 |
| walker |  | 3104 | 58 | Code::CodeKey { rung: Names, file: src/structs/coercions.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.494 |
| ns | 3144 |  | 228 | What each utility struct does | 3.3 | 2.3 | 0.476 |
| walker |  | 3151 | 47 | Code::CodeKey { rung: Decl, file: src/structs/coercions.ts, decl: 1, sub: 0, line: 16 } |  |  | 0.476 |
| walker |  | 3208 | 57 | Code::CodeKey { rung: Decl, file: src/structs/coercions.ts, decl: 2, sub: 0, line: 38 } |  |  | 0.476 |
| walker |  | 3227 | 19 | Code::CodeKey { rung: Body, file: src/structs/coercions.ts, decl: 3, sub: 0, line: 79 } |  |  | 0.477 |
| walker |  | 3267 | 40 | Markdown::HeadingsOutline { file: docs/reference/errors.md } |  |  | 0.477 |
| ns | 3300 |  | 156 | What each refinement does | 3.4 | 2.4 | 0.469 |
| ns | 3411 |  | 111 | What each coercion does - and the create() gotcha | 3.5 | 2.5 | 0.464 |
| walker |  | 3568 | 301 | Json::IdentityMeta { file: package.json } |  |  | 0.478 |
| walker |  | 3620 | 52 | Markdown::HeadingsOutline { file: docs/guides/02-validating-data.md } |  |  | 0.479 |
| walker |  | 3662 | 42 | Markdown::HeadingsOutline { file: docs/reference/typescript.md } |  |  | 0.479 |
| walker |  | 3689 | 27 | Markdown::Section { file: docs/reference/typescript.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.479 |
| ns | 3716 |  | 305 | Public overload declarations of the six overloaded factories | 3.6 | 2.2 | 0.456 |
| ns | 3959 |  | 243 | Struct method semantics: mask recursion and validate options | 3.7 | 2.1 | 0.445 |
| walker |  | 3971 | 282 | Markdown::Section { file: docs/readme.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.445 |
| walker |  | 4016 | 45 | Markdown::HeadingsOutline { file: docs/reference/coercions.md } |  |  | 0.445 |
| walker |  | 4053 | 37 | Markdown::Section { file: docs/guides/02-validating-data.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.445 |
| walker |  | 4091 | 38 | Markdown::Section { file: docs/reference/errors.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.445 |
| walker |  | 4115 | 24 | Code::CodeKey { rung: ModuleDoc, file: test/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.445 |
| ns | 4149 |  | 190 | What each src/utils.ts helper does | 3.8 | 2.9 | 0.436 |
| walker |  | 4226 | 111 | Markdown::Section { file: Readme.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.436 |
| walker |  | 4257 | 31 | Markdown::Section { file: docs/summary.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.436 |
| ns | 4298 |  | 149 | run() signature and per-call context setup | 4.1 | 2.9 | 0.426 |
| walker |  | 4319 | 62 | Markdown::HeadingsOutline { file: docs/reference/core.md } |  |  | 0.428 |
| ns | 4427 |  | 129 | run() stage 1: coercion, then the struct's own validator | 4.2 | 4.1 | 0.419 |
| walker |  | 4542 | 223 | Code::CodeKey { rung: Names, file: src/structs/types.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.430 |
| walker |  | 4558 | 16 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 10, sub: 0, line: 105 } |  |  | 0.430 |
| walker |  | 4581 | 23 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 12, sub: 0, line: 144 } |  |  | 0.430 |
| walker |  | 4607 | 26 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 8, sub: 0, line: 99 } |  |  | 0.432 |
| walker |  | 4633 | 26 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 9, sub: 0, line: 102 } |  |  | 0.435 |
| walker |  | 4647 | 14 | Code::CodeKey { rung: Body, file: src/structs/types.ts, decl: 1, sub: 0, line: 19 } |  |  | 0.435 |
| walker |  | 4669 | 22 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 1, sub: 0, line: 19 } |  |  | 0.435 |
| walker |  | 4692 | 23 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 5, sub: 0, line: 60 } |  |  | 0.436 |
| walker |  | 4715 | 23 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 6, sub: 0, line: 70 } |  |  | 0.437 |
| walker |  | 4738 | 23 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 11, sub: 0, line: 131 } |  |  | 0.438 |
| ns | 4759 |  | 332 | run() stage 2: recursive descent over entries | 4.3 | 4.1 | 0.419 |
| walker |  | 4765 | 27 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 12, sub: 0, line: 144 } |  |  | 0.421 |
| walker |  | 4830 | 65 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 8, sub: 0, line: 99 } |  |  | 0.424 |
| ns | 4882 |  | 123 | run() stage 3: refiners and the success yield | 4.4 | 4.1 | 0.416 |
| walker |  | 4904 | 74 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 7, sub: 0, line: 83 } |  |  | 0.419 |
| walker |  | 4999 | 95 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 2, sub: 0, line: 31 } |  |  | 0.423 |
| ns | 5054 |  | 172 | validate() body: how one failure becomes a StructError | 4.5 | 2.1 | 0.412 |
| walker |  | 5106 | 107 | Code::CodeKey { rung: Doc, file: src/error.ts, decl: 2, sub: 0, line: 25 } |  |  | 0.441 |
| walker |  | 5207 | 101 | Code::CodeKey { rung: Names, file: src/structs/refinements.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.457 |
| walker |  | 5240 | 33 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 5, sub: 0, line: 93 } |  |  | 0.457 |
| ns | 5265 |  | 211 | Struct constructor: defaults and failure wrapping | 4.6 | 1.6 | 0.442 |
| walker |  | 5284 | 44 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 7, sub: 0, line: 146 } |  |  | 0.442 |
| walker |  | 5332 | 48 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 1, sub: 0, line: 8 } |  |  | 0.442 |
| walker |  | 5380 | 48 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 4, sub: 0, line: 77 } |  |  | 0.442 |
| walker |  | 5437 | 57 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 2, sub: 0, line: 33 } |  |  | 0.442 |
| ns | 5456 |  | 191 | StructError constructor: message, cause and failure caching | 4.7 | 2.7 | 0.434 |
| walker |  | 5494 | 57 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 3, sub: 0, line: 55 } |  |  | 0.434 |
| walker |  | 5556 | 62 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 6, sub: 0, line: 109 } |  |  | 0.434 |
| walker |  | 5580 | 24 | Code::CodeKey { rung: Doc, file: src/structs/refinements.ts, decl: 5, sub: 0, line: 93 } |  |  | 0.434 |
| ns | 5596 |  | 140 | Complete docs/ tree listing | 5.1 |  | 0.471 |
| walker |  | 5606 | 26 | Code::CodeKey { rung: Doc, file: src/structs/refinements.ts, decl: 2, sub: 0, line: 33 } |  |  | 0.471 |
| walker |  | 5632 | 26 | Code::CodeKey { rung: Doc, file: src/structs/refinements.ts, decl: 3, sub: 0, line: 55 } |  |  | 0.473 |
| ns | 5649 |  | 53 | Complete test/ and test/api/ listings | 5.2 |  | 0.486 |
| walker |  | 5661 | 29 | Code::CodeKey { rung: Doc, file: src/structs/refinements.ts, decl: 1, sub: 0, line: 8 } |  |  | 0.488 |
| walker |  | 5690 | 29 | Code::CodeKey { rung: Doc, file: src/structs/refinements.ts, decl: 4, sub: 0, line: 77 } |  |  | 0.490 |
| walker |  | 5738 | 48 | Code::CodeKey { rung: Doc, file: src/structs/refinements.ts, decl: 6, sub: 0, line: 109 } |  |  | 0.494 |
| ns | 5774 |  | 125 | All 41 validation-fixture kind directories | 5.3 |  | 0.532 |
| ns | 5829 |  | 55 | Complete examples/ listing | 5.4 |  | 0.540 |
| ns | 5906 |  | 77 | One kind directory listed in full, as the case-naming exemplar | 5.5 |  | 0.533 |
| walker |  | 5971 | 233 | Code::CodeKey { rung: Names, file: src/struct.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.548 |
| walker |  | 6005 | 34 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 13, sub: 0, line: 221 } |  |  | 0.555 |
| walker |  | 6042 | 37 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 9, sub: 0, line: 139 } |  |  | 0.555 |
| walker |  | 6079 | 37 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 10, sub: 0, line: 157 } |  |  | 0.555 |
| ns | 6089 |  | 183 | The 45 type-level test files | 5.6 |  | 0.583 |
| walker |  | 6119 | 40 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 8, sub: 0, line: 123 } |  |  | 0.583 |
| walker |  | 6160 | 41 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 16, sub: 0, line: 243 } |  |  | 0.591 |
| walker |  | 6243 | 83 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 12, sub: 0, line: 185 } |  |  | 0.591 |
| walker |  | 6266 | 23 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 11, sub: 0, line: 175 } |  |  | 0.591 |
| walker |  | 6289 | 23 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 16, sub: 0, line: 243 } |  |  | 0.591 |
| walker |  | 6314 | 25 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 18, sub: 0, line: 259 } |  |  | 0.591 |
| ns | 6316 |  | 227 | Every heading in the six guides | 5.7 |  | 0.602 |
| walker |  | 6341 | 27 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 14, sub: 0, line: 231 } |  |  | 0.602 |
| walker |  | 6368 | 27 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 15, sub: 0, line: 237 } |  |  | 0.602 |
| ns | 6379 |  | 63 | Readme section map | 5.8 |  | 0.606 |
| walker |  | 6396 | 28 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 8, sub: 0, line: 123 } |  |  | 0.606 |
| walker |  | 6424 | 28 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 17, sub: 0, line: 253 } |  |  | 0.606 |
| walker |  | 6453 | 29 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 9, sub: 0, line: 139 } |  |  | 0.606 |
| walker |  | 6483 | 30 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 10, sub: 0, line: 157 } |  |  | 0.606 |
| ns | 6519 |  | 140 | Doc anchors the source rosters do not cover | 5.9 |  | 0.603 |
| walker |  | 6522 | 39 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 19, sub: 0, line: 266 } |  |  | 0.603 |
| ns | 6690 |  | 171 | The data-driven fixture runner | 5.10 |  | 0.597 |
| walker |  | 6764 | 242 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 1, sub: 0, line: 10 } |  |  | 0.624 |
| walker |  | 6836 | 72 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 2, sub: 0, line: 22 } |  |  | 0.624 |
| ns | 6891 |  | 201 | Runner assertions: output-style vs failures-style fixtures | 5.11 | 5.10 | 0.614 |
| walker |  | 6908 | 72 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 7, sub: 0, line: 107 } |  |  | 0.614 |
| walker |  | 6954 | 46 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 12, sub: 0, line: 185 } |  |  | 0.614 |
| walker |  | 6982 | 28 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 5, sub: 0, line: 83 } |  |  | 0.614 |
| walker |  | 7015 | 33 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 3, sub: 0, line: 67 } |  |  | 0.614 |
| walker |  | 7048 | 33 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 4, sub: 0, line: 75 } |  |  | 0.614 |
| walker |  | 7117 | 69 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 1, sub: 0, line: 10 } |  |  | 0.630 |
| ns | 7164 |  | 273 | A failures-style and an output-style fixture, in full | 5.12 | 5.10 | 0.612 |
| walker |  | 7214 | 97 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 13, sub: 0, line: 221 } |  |  | 0.615 |
| ns | 7295 |  | 131 | The typings harness and one typings test | 5.13 | 5.6 | 0.609 |
| walker |  | 7390 | 176 | Markdown::Section { file: Readme.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.609 |
| ns | 7481 |  | 186 | toFailure(): result normalisation and default message text | 6.1 | 2.9 | 0.600 |
| walker |  | 7585 | 195 | Markdown::Section { file: Readme.md, section_index: 4, keeps_default_concavity: true } |  |  | 0.600 |
| walker |  | 7720 | 135 | Markdown::Section { file: Readme.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.600 |
| ns | 7894 |  | 413 | object() implementation, including the mask special case | 6.2 | 3.6 | 0.581 |
| walker |  | 7977 | 257 | Markdown::Section { file: License.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.581 |
| walker |  | 8046 | 69 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 6, sub: 0, line: 93 } |  |  | 0.585 |
| ns | 8085 |  | 191 | refine() and define() bodies - the extension points | 6.3 | 2.4 | 0.578 |
| walker |  | 8109 | 63 | Markdown::Section { file: docs/reference/coercions.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.578 |
| walker |  | 8188 | 79 | Markdown::HeadingsOutline { file: docs/reference/utilities.md } |  |  | 0.578 |
| ns | 8199 |  | 114 | coerce() and trimmed() bodies | 6.4 | 2.5 | 0.574 |
| walker |  | 8218 | 30 | Markdown::Section { file: docs/reference/utilities.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.574 |
| walker |  | 8343 | 125 | Markdown::Section { file: docs/summary.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.574 |
| walker |  | 8425 | 82 | Markdown::HeadingsOutline { file: docs/reference/refinements.md } |  |  | 0.575 |
| ns | 8458 |  | 259 | npm scripts | 7.1 | 1.1 | 0.581 |
| walker |  | 8478 | 53 | Markdown::Section { file: docs/reference/refinements.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.581 |
| ns | 8629 |  | 171 | Changelog: recent release headings and the file's extent | 7.2 |  | 0.577 |
| walker |  | 8709 | 231 | Code::CodeKey { rung: Names, file: src/structs/types.ts, decl: 0, sub: 1, line: 0 } |  |  | 0.592 |
| walker |  | 8740 | 31 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 21, sub: 0, line: 226 } |  |  | 0.595 |
| ns | 8778 |  | 149 | The 2.0.0 release notes: breaking changes and fixes | 7.3 | 7.2 | 0.593 |
| walker |  | 8780 | 40 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 14, sub: 0, line: 172 } |  |  | 0.593 |
| walker |  | 8803 | 23 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 13, sub: 0, line: 159 } |  |  | 0.594 |
| walker |  | 8826 | 23 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 23, sub: 0, line: 258 } |  |  | 0.594 |
| walker |  | 8853 | 27 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 14, sub: 0, line: 172 } |  |  | 0.596 |
| walker |  | 8880 | 27 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 24, sub: 0, line: 266 } |  |  | 0.596 |
| walker |  | 8911 | 31 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 15, sub: 0, line: 200 } |  |  | 0.599 |
| walker |  | 8954 | 43 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 20, sub: 0, line: 225 } |  |  | 0.599 |
| ns | 8957 |  | 179 | TypeScript compiler configuration | 7.4 |  | 0.593 |
| walker |  | 9031 | 77 | Code::CodeKey { rung: Doc, file: src/structs/coercions.ts, decl: 3, sub: 0, line: 79 } |  |  | 0.593 |
| walker |  | 9113 | 82 | Code::CodeKey { rung: Doc, file: src/structs/coercions.ts, decl: 2, sub: 0, line: 38 } |  |  | 0.594 |
| walker |  | 9204 | 91 | Code::CodeKey { rung: Doc, file: src/structs/refinements.ts, decl: 7, sub: 0, line: 146 } |  |  | 0.596 |
| ns | 9207 |  | 250 | Rollup build and JSR publish config | 7.5 |  | 0.586 |
| walker |  | 9314 | 110 | Json::Whole { file: jsr.json } |  |  | 0.589 |
| ns | 9368 |  | 161 | CI matrix and dependency automation | 7.6 |  | 0.583 |
| ns | 9521 |  | 153 | Remaining package.json keys: entry points and publish metadata | 7.7 | 1.1 | 0.589 |
| walker |  | 9534 | 220 | Code::CodeKey { rung: Names, file: src/structs/utilities.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.601 |
| walker |  | 9567 | 33 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 8, sub: 0, line: 106 } |  |  | 0.601 |
| ns | 9599 |  | 78 | Formatting and docs-site configuration | 7.8 |  | 0.599 |
| walker |  | 9604 | 37 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 11, sub: 0, line: 197 } |  |  | 0.599 |
| walker |  | 9642 | 38 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 7, sub: 0, line: 80 } |  |  | 0.599 |
| walker |  | 9685 | 43 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 12, sub: 0, line: 221 } |  |  | 0.599 |
| ns | 9718 |  | 119 | ESLint configuration head | 7.9 |  | 0.594 |
| walker |  | 9729 | 44 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 10, sub: 0, line: 171 } |  |  | 0.594 |
| walker |  | 9774 | 45 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 1, sub: 0, line: 17 } |  |  | 0.594 |
| ns | 9847 |  | 129 | Examples package manifest and run instructions | 7.10 |  | 0.592 |
| walker |  | 9876 | 102 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 2, sub: 0, line: 21 } |  |  | 0.592 |
