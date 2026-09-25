Score(3000)=0.482 I=0.781 C=0.297 ns_rows≤3K=20/60 grid(1000/1442/2080/3000/4327/6240/9000)=0.639/0.499/0.564/0.482/0.428/0.622/0.643

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
| walker |  | 5230 | 231 | Code::CodeKey { rung: Names, file: src/structs/types.ts, decl: 0, sub: 1, line: 0 } |  |  | 0.444 |
| walker |  | 5261 | 31 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 21, sub: 0, line: 226 } |  |  | 0.451 |
| ns | 5265 |  | 211 | Struct constructor: defaults and failure wrapping | 4.6 | 1.6 | 0.437 |
| walker |  | 5301 | 40 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 14, sub: 0, line: 172 } |  |  | 0.437 |
| walker |  | 5324 | 23 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 13, sub: 0, line: 159 } |  |  | 0.440 |
| walker |  | 5347 | 23 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 23, sub: 0, line: 258 } |  |  | 0.440 |
| walker |  | 5374 | 27 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 14, sub: 0, line: 172 } |  |  | 0.444 |
| walker |  | 5401 | 27 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 24, sub: 0, line: 266 } |  |  | 0.444 |
| walker |  | 5432 | 31 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 15, sub: 0, line: 200 } |  |  | 0.450 |
| ns | 5456 |  | 191 | StructError constructor: message, cause and failure caching | 4.7 | 2.7 | 0.441 |
| walker |  | 5475 | 43 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 20, sub: 0, line: 225 } |  |  | 0.443 |
| ns | 5596 |  | 140 | Complete docs/ tree listing | 5.1 |  | 0.478 |
| ns | 5649 |  | 53 | Complete test/ and test/api/ listings | 5.2 |  | 0.491 |
| walker |  | 5744 | 269 | Code::CodeKey { rung: Names, file: src/structs/types.ts, decl: 0, sub: 2, line: 0 } |  |  | 0.540 |
| walker |  | 5763 | 19 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 27, sub: 0, line: 295 } |  |  | 0.545 |
| ns | 5774 |  | 125 | All 41 validation-fixture kind directories | 5.3 |  | 0.577 |
| walker |  | 5785 | 22 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 37, sub: 0, line: 492 } |  |  | 0.577 |
| walker |  | 5819 | 34 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 30, sub: 0, line: 367 } |  |  | 0.577 |
| ns | 5829 |  | 55 | Complete examples/ listing | 5.4 |  | 0.585 |
| walker |  | 5853 | 34 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 36, sub: 0, line: 456 } |  |  | 0.585 |
| walker |  | 5888 | 35 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 38, sub: 0, line: 522 } |  |  | 0.585 |
| ns | 5906 |  | 77 | One kind directory listed in full, as the case-naming exemplar | 5.5 |  | 0.576 |
| walker |  | 5911 | 23 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 25, sub: 0, line: 278 } |  |  | 0.577 |
| walker |  | 5934 | 23 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 35, sub: 0, line: 442 } |  |  | 0.578 |
| walker |  | 5960 | 26 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 29, sub: 0, line: 351 } |  |  | 0.578 |
| walker |  | 5987 | 27 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 38, sub: 0, line: 522 } |  |  | 0.580 |
| walker |  | 6017 | 30 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 39, sub: 0, line: 574 } |  |  | 0.581 |
| walker |  | 6059 | 42 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 32, sub: 0, line: 413 } |  |  | 0.584 |
| ns | 6089 |  | 183 | The 45 type-level test files | 5.6 |  | 0.609 |
| walker |  | 6104 | 45 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 36, sub: 0, line: 456 } |  |  | 0.613 |
| walker |  | 6168 | 64 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 30, sub: 0, line: 367 } |  |  | 0.617 |
| walker |  | 6233 | 65 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 26, sub: 0, line: 294 } |  |  | 0.622 |
| walker |  | 6299 | 66 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 31, sub: 0, line: 402 } |  |  | 0.624 |
| ns | 6316 |  | 227 | Every heading in the six guides | 5.7 |  | 0.633 |
| walker |  | 6366 | 67 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 37, sub: 0, line: 492 } |  |  | 0.636 |
| ns | 6379 |  | 63 | Readme section map | 5.8 |  | 0.639 |
| walker |  | 6473 | 107 | Code::CodeKey { rung: Doc, file: src/error.ts, decl: 2, sub: 0, line: 25 } |  |  | 0.654 |
| ns | 6519 |  | 140 | Doc anchors the source rosters do not cover | 5.9 |  | 0.650 |
| walker |  | 6574 | 101 | Code::CodeKey { rung: Names, file: src/structs/refinements.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.659 |
| walker |  | 6607 | 33 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 5, sub: 0, line: 93 } |  |  | 0.659 |
| walker |  | 6651 | 44 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 7, sub: 0, line: 146 } |  |  | 0.660 |
| ns | 6690 |  | 171 | The data-driven fixture runner | 5.10 |  | 0.653 |
| walker |  | 6699 | 48 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 1, sub: 0, line: 8 } |  |  | 0.653 |
| walker |  | 6747 | 48 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 4, sub: 0, line: 77 } |  |  | 0.653 |
| walker |  | 6804 | 57 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 2, sub: 0, line: 33 } |  |  | 0.653 |
| walker |  | 6861 | 57 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 3, sub: 0, line: 55 } |  |  | 0.653 |
| ns | 6891 |  | 201 | Runner assertions: output-style vs failures-style fixtures | 5.11 | 5.10 | 0.643 |
| walker |  | 6923 | 62 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 6, sub: 0, line: 109 } |  |  | 0.643 |
| walker |  | 6947 | 24 | Code::CodeKey { rung: Doc, file: src/structs/refinements.ts, decl: 5, sub: 0, line: 93 } |  |  | 0.643 |
| walker |  | 6973 | 26 | Code::CodeKey { rung: Doc, file: src/structs/refinements.ts, decl: 2, sub: 0, line: 33 } |  |  | 0.644 |
| walker |  | 6999 | 26 | Code::CodeKey { rung: Doc, file: src/structs/refinements.ts, decl: 3, sub: 0, line: 55 } |  |  | 0.644 |
| walker |  | 7028 | 29 | Code::CodeKey { rung: Doc, file: src/structs/refinements.ts, decl: 1, sub: 0, line: 8 } |  |  | 0.645 |
| walker |  | 7057 | 29 | Code::CodeKey { rung: Doc, file: src/structs/refinements.ts, decl: 4, sub: 0, line: 77 } |  |  | 0.647 |
| walker |  | 7105 | 48 | Code::CodeKey { rung: Doc, file: src/structs/refinements.ts, decl: 6, sub: 0, line: 109 } |  |  | 0.649 |
| ns | 7164 |  | 273 | A failures-style and an output-style fixture, in full | 5.12 | 5.10 | 0.631 |
| ns | 7295 |  | 131 | The typings harness and one typings test | 5.13 | 5.6 | 0.625 |
| walker |  | 7338 | 233 | Code::CodeKey { rung: Names, file: src/struct.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.637 |
| walker |  | 7372 | 34 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 13, sub: 0, line: 221 } |  |  | 0.642 |
| walker |  | 7409 | 37 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 9, sub: 0, line: 139 } |  |  | 0.642 |
| walker |  | 7446 | 37 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 10, sub: 0, line: 157 } |  |  | 0.642 |
| ns | 7481 |  | 186 | toFailure(): result normalisation and default message text | 6.1 | 2.9 | 0.633 |
| walker |  | 7486 | 40 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 8, sub: 0, line: 123 } |  |  | 0.633 |
| walker |  | 7527 | 41 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 16, sub: 0, line: 243 } |  |  | 0.640 |
| walker |  | 7610 | 83 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 12, sub: 0, line: 185 } |  |  | 0.640 |
| walker |  | 7633 | 23 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 11, sub: 0, line: 175 } |  |  | 0.640 |
| walker |  | 7656 | 23 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 16, sub: 0, line: 243 } |  |  | 0.640 |
| walker |  | 7681 | 25 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 18, sub: 0, line: 259 } |  |  | 0.640 |
| walker |  | 7708 | 27 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 14, sub: 0, line: 231 } |  |  | 0.640 |
| walker |  | 7735 | 27 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 15, sub: 0, line: 237 } |  |  | 0.640 |
| walker |  | 7763 | 28 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 8, sub: 0, line: 123 } |  |  | 0.640 |
| walker |  | 7791 | 28 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 17, sub: 0, line: 253 } |  |  | 0.640 |
| walker |  | 7820 | 29 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 9, sub: 0, line: 139 } |  |  | 0.640 |
| walker |  | 7850 | 30 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 10, sub: 0, line: 157 } |  |  | 0.640 |
| walker |  | 7889 | 39 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 19, sub: 0, line: 266 } |  |  | 0.640 |
| ns | 7894 |  | 413 | object() implementation, including the mask special case | 6.2 | 3.6 | 0.620 |
| ns | 8085 |  | 191 | refine() and define() bodies - the extension points | 6.3 | 2.4 | 0.613 |
| walker |  | 8131 | 242 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 1, sub: 0, line: 10 } |  |  | 0.637 |
| ns | 8199 |  | 114 | coerce() and trimmed() bodies | 6.4 | 2.5 | 0.632 |
| walker |  | 8203 | 72 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 2, sub: 0, line: 22 } |  |  | 0.632 |
| walker |  | 8275 | 72 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 7, sub: 0, line: 107 } |  |  | 0.632 |
| walker |  | 8303 | 28 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 5, sub: 0, line: 83 } |  |  | 0.632 |
| walker |  | 8336 | 33 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 3, sub: 0, line: 67 } |  |  | 0.632 |
| walker |  | 8369 | 33 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 4, sub: 0, line: 75 } |  |  | 0.632 |
| walker |  | 8415 | 46 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 12, sub: 0, line: 185 } |  |  | 0.632 |
| ns | 8458 |  | 259 | npm scripts | 7.1 | 1.1 | 0.637 |
| walker |  | 8484 | 69 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 1, sub: 0, line: 10 } |  |  | 0.651 |
| walker |  | 8553 | 69 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 6, sub: 0, line: 93 } |  |  | 0.652 |
| ns | 8629 |  | 171 | Changelog: recent release headings and the file's extent | 7.2 |  | 0.648 |
| walker |  | 8650 | 97 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 13, sub: 0, line: 221 } |  |  | 0.652 |
| ns | 8778 |  | 149 | The 2.0.0 release notes: breaking changes and fixes | 7.3 | 7.2 | 0.650 |
| walker |  | 8826 | 176 | Markdown::Section { file: Readme.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.650 |
| ns | 8957 |  | 179 | TypeScript compiler configuration | 7.4 |  | 0.643 |
| walker |  | 9021 | 195 | Markdown::Section { file: Readme.md, section_index: 4, keeps_default_concavity: true } |  |  | 0.643 |
| walker |  | 9156 | 135 | Markdown::Section { file: Readme.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.643 |
| ns | 9207 |  | 250 | Rollup build and JSR publish config | 7.5 |  | 0.633 |
| ns | 9368 |  | 161 | CI matrix and dependency automation | 7.6 |  | 0.627 |
| walker |  | 9413 | 257 | Markdown::Section { file: License.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.627 |
| walker |  | 9476 | 63 | Markdown::Section { file: docs/reference/coercions.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.627 |
| ns | 9521 |  | 153 | Remaining package.json keys: entry points and publish metadata | 7.7 | 1.1 | 0.632 |
| walker |  | 9555 | 79 | Markdown::HeadingsOutline { file: docs/reference/utilities.md } |  |  | 0.632 |
| walker |  | 9585 | 30 | Markdown::Section { file: docs/reference/utilities.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.632 |
| ns | 9599 |  | 78 | Formatting and docs-site configuration | 7.8 |  | 0.630 |
| ns | 9718 |  | 119 | ESLint configuration head | 7.9 |  | 0.625 |
| walker |  | 9720 | 135 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 7, sub: 0, line: 107 } |  |  | 0.632 |
| walker |  | 9845 | 125 | Markdown::Section { file: docs/summary.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.632 |
| ns | 9847 |  | 129 | Examples package manifest and run instructions | 7.10 |  | 0.630 |
| walker |  | 9927 | 82 | Markdown::HeadingsOutline { file: docs/reference/refinements.md } |  |  | 0.631 |
| walker |  | 9980 | 53 | Markdown::Section { file: docs/reference/refinements.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.631 |
