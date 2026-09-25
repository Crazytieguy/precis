Score(3000)=0.520 I=0.781 C=0.347 ns_rows≤3K=20/60 grid(1000/1442/2080/3000/4327/6240/9000)=0.639/0.498/0.577/0.520/0.539/0.611/0.610

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
| walker |  | 226 | 79 | Code::CodeKey { rung: Names, file: src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.640 |
| ns | 250 |  | 79 | src/index.ts - the entire public barrel | 1.4 |  | 0.636 |
| walker |  | 308 | 82 | Json::Identity { file: package.json } |  |  | 0.962 |
| walker |  | 337 | 29 | Fs::DirListing { dir: docs/images } |  |  | 0.964 |
| walker |  | 368 | 31 | Fs::DirListing { dir: docs/reference } |  |  | 0.966 |
| walker |  | 395 | 27 | Code::CodeKey { rung: Names, file: src/error.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.966 |
| ns | 416 |  | 166 | Readme lede: what Superstruct is and why it exists | 1.5 |  | 0.937 |
| walker |  | 429 | 34 | Json::Runtime { file: package.json } |  |  | 0.937 |
| walker |  | 480 | 51 | Fs::DirListing { dir: docs/guides } |  |  | 0.940 |
| walker |  | 510 | 30 | Markdown::HeadingsOutline { file: docs/summary.md } |  |  | 0.940 |
| walker |  | 528 | 18 | Markdown::HeadingsOutline { file: docs/resources/links.md } |  |  | 0.940 |
| walker |  | 556 | 28 | Fs::DirListing { dir: test } |  |  | 0.941 |
| ns | 638 |  | 222 | The Struct class: doc comment and its six fields | 1.6 |  | 0.778 |
| walker |  | 753 | 197 | Markdown::ReadmeHeadline { file: Readme.md } |  |  | 0.785 |
| walker |  | 807 | 54 | Markdown::HeadingsOutline { file: Readme.md } |  |  | 0.786 |
| ns | 816 |  | 178 | Core API signatures: assert / create / is / mask / validate | 1.7 |  | 0.711 |
| walker |  | 873 | 66 | Markdown::Prelude { file: Readme.md } |  |  | 0.728 |
| walker |  | 892 | 19 | Markdown::Section { file: Readme.md, section_index: 10, keeps_default_concavity: false } |  |  | 0.728 |
| walker |  | 968 | 76 | Json::Entry { file: package.json } |  |  | 0.728 |
| ns | 971 |  | 155 | Readme canonical usage snippet | 1.8 |  | 0.639 |
| walker |  | 1049 | 81 | Code::CodeKey { rung: Decl, file: src/error.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.643 |
| ns | 1066 |  | 95 | src/struct.ts symbol roster: methods and top-level helpers | 2.1 |  | 0.600 |
| walker |  | 1180 | 131 | Markdown::ReadmeHeadline { file: docs/readme.md } |  |  | 0.600 |
| walker |  | 1246 | 66 | Markdown::Prelude { file: docs/readme.md } |  |  | 0.600 |
| walker |  | 1301 | 55 | Fs::DirListing { dir: examples } |  |  | 0.602 |
| ns | 1318 |  | 252 | Complete roster of the 25 type structs | 2.2 |  | 0.521 |
| walker |  | 1359 | 58 | Code::CodeKey { rung: Names, file: src/structs/coercions.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.523 |
| walker |  | 1406 | 47 | Code::CodeKey { rung: Decl, file: src/structs/coercions.ts, decl: 1, sub: 0, line: 16 } |  |  | 0.523 |
| ns | 1420 |  | 102 | Complete roster of src/structs/utilities.ts | 2.3 |  | 0.498 |
| walker |  | 1463 | 57 | Code::CodeKey { rung: Decl, file: src/structs/coercions.ts, decl: 2, sub: 0, line: 38 } |  |  | 0.498 |
| walker |  | 1488 | 25 | Fs::DirListing { dir: test/api } |  |  | 0.499 |
| ns | 1503 |  | 83 | Complete roster of src/structs/refinements.ts | 2.4 |  | 0.481 |
| ns | 1537 |  | 34 | Complete roster of src/structs/coercions.ts | 2.5 |  | 0.492 |
| walker |  | 1601 | 113 | Code::CodeKey { rung: Decl, file: src/error.ts, decl: 2, sub: 0, line: 25 } |  |  | 0.494 |
| ns | 1651 |  | 114 | Failure - the shape of every validation failure | 2.6 |  | 0.509 |
| ns | 1829 |  | 178 | StructError class: doc, fields, and its early-exit contract | 2.7 |  | 0.501 |
| walker |  | 1834 | 233 | Markdown::Section { file: Readme.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.584 |
| walker |  | 1860 | 26 | Code::CodeKey { rung: Doc, file: src/error.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.602 |
| walker |  | 1985 | 125 | Fs::DirListing { dir: test/validation } |  |  | 0.608 |
| ns | 2037 |  | 208 | Exported type vocabulary of src/struct.ts | 2.8 |  | 0.577 |
| ns | 2140 |  | 103 | Function roster of src/utils.ts | 2.9 |  | 0.561 |
| walker |  | 2267 | 282 | Markdown::Section { file: Readme.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.561 |
| ns | 2378 |  | 238 | Type-level utility roster of src/utils.ts | 2.10 |  | 0.527 |
| walker |  | 2490 | 223 | Code::CodeKey { rung: Names, file: src/structs/types.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.538 |
| walker |  | 2506 | 16 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 10, sub: 0, line: 105 } |  |  | 0.538 |
| walker |  | 2529 | 23 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 12, sub: 0, line: 144 } |  |  | 0.538 |
| walker |  | 2555 | 26 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 8, sub: 0, line: 99 } |  |  | 0.538 |
| ns | 2574 |  | 196 | What each type struct does, part 1 (any ... literal) | 3.1 | 2.2 | 0.522 |
| walker |  | 2581 | 26 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 9, sub: 0, line: 102 } |  |  | 0.522 |
| walker |  | 2595 | 14 | Code::CodeKey { rung: Body, file: src/structs/types.ts, decl: 1, sub: 0, line: 19 } |  |  | 0.522 |
| walker |  | 2696 | 101 | Code::CodeKey { rung: Names, file: src/structs/refinements.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.546 |
| walker |  | 2729 | 33 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 5, sub: 0, line: 93 } |  |  | 0.546 |
| walker |  | 2773 | 44 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 7, sub: 0, line: 146 } |  |  | 0.546 |
| walker |  | 2821 | 48 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 1, sub: 0, line: 8 } |  |  | 0.546 |
| walker |  | 2869 | 48 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 4, sub: 0, line: 77 } |  |  | 0.546 |
| ns | 2916 |  | 342 | What each type struct does, part 2 (map ... unknown) | 3.2 | 2.2 | 0.520 |
| walker |  | 2926 | 57 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 2, sub: 0, line: 33 } |  |  | 0.520 |
| walker |  | 2983 | 57 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 3, sub: 0, line: 55 } |  |  | 0.520 |
| walker |  | 3045 | 62 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 6, sub: 0, line: 109 } |  |  | 0.520 |
| ns | 3144 |  | 228 | What each utility struct does | 3.3 | 2.3 | 0.502 |
| walker |  | 3278 | 233 | Code::CodeKey { rung: Names, file: src/struct.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.532 |
| ns | 3300 |  | 156 | What each refinement does | 3.4 | 2.4 | 0.524 |
| walker |  | 3312 | 34 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 13, sub: 0, line: 221 } |  |  | 0.537 |
| walker |  | 3349 | 37 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 9, sub: 0, line: 139 } |  |  | 0.537 |
| walker |  | 3386 | 37 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 10, sub: 0, line: 157 } |  |  | 0.537 |
| ns | 3411 |  | 111 | What each coercion does - and the create() gotcha | 3.5 | 2.5 | 0.531 |
| walker |  | 3426 | 40 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 8, sub: 0, line: 123 } |  |  | 0.531 |
| walker |  | 3467 | 41 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 16, sub: 0, line: 243 } |  |  | 0.549 |
| walker |  | 3550 | 83 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 12, sub: 0, line: 185 } |  |  | 0.549 |
| ns | 3716 |  | 305 | Public overload declarations of the six overloaded factories | 3.6 | 2.2 | 0.530 |
| walker |  | 3733 | 183 | Fs::DirListing { dir: test/typings } |  |  | 0.535 |
| walker |  | 3755 | 22 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 1, sub: 0, line: 19 } |  |  | 0.535 |
| walker |  | 3774 | 19 | Code::CodeKey { rung: Body, file: src/structs/coercions.ts, decl: 3, sub: 0, line: 79 } |  |  | 0.535 |
| walker |  | 3793 | 19 | Markdown::Section { file: docs/resources/links.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.535 |
| ns | 3959 |  | 243 | Struct method semantics: mask recursion and validate options | 3.7 | 2.1 | 0.522 |
| walker |  | 4024 | 231 | Code::CodeKey { rung: Names, file: src/structs/types.ts, decl: 0, sub: 1, line: 0 } |  |  | 0.556 |
| walker |  | 4055 | 31 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 21, sub: 0, line: 226 } |  |  | 0.563 |
| walker |  | 4095 | 40 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 14, sub: 0, line: 172 } |  |  | 0.563 |
| ns | 4149 |  | 190 | What each src/utils.ts helper does | 3.8 | 2.9 | 0.552 |
| ns | 4298 |  | 149 | run() signature and per-call context setup | 4.1 | 2.9 | 0.539 |
| walker |  | 4339 | 244 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 1, sub: 0, line: 10 } |  |  | 0.583 |
| walker |  | 4411 | 72 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 2, sub: 0, line: 22 } |  |  | 0.583 |
| ns | 4427 |  | 129 | run() stage 1: coercion, then the struct's own validator | 4.2 | 4.1 | 0.571 |
| walker |  | 4483 | 72 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 7, sub: 0, line: 107 } |  |  | 0.571 |
| walker |  | 4515 | 32 | Markdown::HeadingsOutline { file: docs/guides/05-handling-errors.md } |  |  | 0.571 |
| walker |  | 4549 | 34 | Markdown::HeadingsOutline { file: docs/guides/03-coercing-data.md } |  |  | 0.571 |
| walker |  | 4583 | 34 | Markdown::HeadingsOutline { file: docs/guides/04-refining-validation.md } |  |  | 0.571 |
| walker |  | 4619 | 36 | Markdown::HeadingsOutline { file: docs/guides/01-getting-started.md } |  |  | 0.571 |
| ns | 4759 |  | 332 | run() stage 2: recursive descent over entries | 4.3 | 4.1 | 0.546 |
| ns | 4882 |  | 123 | run() stage 3: refiners and the success yield | 4.4 | 4.1 | 0.536 |
| walker |  | 4945 | 326 | Json::Scripts { file: package.json } |  |  | 0.537 |
| ns | 5054 |  | 172 | validate() body: how one failure becomes a StructError | 4.5 | 2.1 | 0.523 |
| walker |  | 5165 | 220 | Code::CodeKey { rung: Names, file: src/structs/utilities.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.546 |
| walker |  | 5198 | 33 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 8, sub: 0, line: 106 } |  |  | 0.546 |
| walker |  | 5235 | 37 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 11, sub: 0, line: 197 } |  |  | 0.546 |
| ns | 5265 |  | 211 | Struct constructor: defaults and failure wrapping | 4.6 | 1.6 | 0.529 |
| walker |  | 5273 | 38 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 7, sub: 0, line: 80 } |  |  | 0.529 |
| walker |  | 5316 | 43 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 12, sub: 0, line: 221 } |  |  | 0.529 |
| walker |  | 5360 | 44 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 10, sub: 0, line: 171 } |  |  | 0.529 |
| walker |  | 5405 | 45 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 1, sub: 0, line: 17 } |  |  | 0.529 |
| ns | 5456 |  | 191 | StructError constructor: message, cause and failure caching | 4.7 | 2.7 | 0.519 |
| walker |  | 5507 | 102 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 2, sub: 0, line: 21 } |  |  | 0.519 |
| ns | 5596 |  | 140 | Complete docs/ tree listing | 5.1 |  | 0.546 |
| ns | 5649 |  | 53 | Complete test/ and test/api/ listings | 5.2 |  | 0.557 |
| walker |  | 5657 | 150 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 3, sub: 0, line: 30 } |  |  | 0.557 |
| ns | 5774 |  | 125 | All 41 validation-fixture kind directories | 5.3 |  | 0.587 |
| ns | 5829 |  | 55 | Complete examples/ listing | 5.4 |  | 0.594 |
| walker |  | 5890 | 233 | Markdown::Section { file: docs/readme.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.594 |
| ns | 5906 |  | 77 | One kind directory listed in full, as the case-naming exemplar | 5.5 |  | 0.586 |
| walker |  | 5934 | 44 | Markdown::HeadingsOutline { file: docs/guides/06-using-typescript.md } |  |  | 0.586 |
| ns | 6089 |  | 183 | The 45 type-level test files | 5.6 |  | 0.611 |
| walker |  | 6116 | 182 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 4, sub: 0, line: 44 } |  |  | 0.611 |
| walker |  | 6156 | 40 | Markdown::HeadingsOutline { file: docs/reference/errors.md } |  |  | 0.611 |
| ns | 6316 |  | 227 | Every heading in the six guides | 5.7 |  | 0.612 |
| ns | 6379 |  | 63 | Readme section map | 5.8 |  | 0.615 |
| walker |  | 6457 | 301 | Json::IdentityMeta { file: package.json } |  |  | 0.629 |
| ns | 6519 |  | 140 | Doc anchors the source rosters do not cover | 5.9 |  | 0.623 |
| ns | 6690 |  | 171 | The data-driven fixture runner | 5.10 |  | 0.617 |
| walker |  | 6726 | 269 | Code::CodeKey { rung: Names, file: src/structs/types.ts, decl: 0, sub: 2, line: 0 } |  |  | 0.652 |
| walker |  | 6745 | 19 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 27, sub: 0, line: 295 } |  |  | 0.656 |
| walker |  | 6767 | 22 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 37, sub: 0, line: 492 } |  |  | 0.656 |
| walker |  | 6801 | 34 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 30, sub: 0, line: 367 } |  |  | 0.656 |
| walker |  | 6835 | 34 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 36, sub: 0, line: 456 } |  |  | 0.656 |
| walker |  | 6870 | 35 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 38, sub: 0, line: 522 } |  |  | 0.656 |
| ns | 6891 |  | 201 | Runner assertions: output-style vs failures-style fixtures | 5.11 | 5.10 | 0.646 |
| walker |  | 6922 | 52 | Markdown::HeadingsOutline { file: docs/guides/02-validating-data.md } |  |  | 0.655 |
| walker |  | 6964 | 42 | Markdown::HeadingsOutline { file: docs/reference/typescript.md } |  |  | 0.656 |
| walker |  | 6991 | 27 | Markdown::Section { file: docs/reference/typescript.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.656 |
| ns | 7164 |  | 273 | A failures-style and an output-style fixture, in full | 5.12 | 5.10 | 0.639 |
| walker |  | 7273 | 282 | Markdown::Section { file: docs/readme.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.639 |
| ns | 7295 |  | 131 | The typings harness and one typings test | 5.13 | 5.6 | 0.632 |
| walker |  | 7318 | 45 | Markdown::HeadingsOutline { file: docs/reference/coercions.md } |  |  | 0.633 |
| walker |  | 7355 | 37 | Markdown::Section { file: docs/guides/02-validating-data.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.633 |
| walker |  | 7377 | 22 | Code::CodeKey { rung: Names, file: test/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.633 |
| walker |  | 7397 | 20 | Code::CodeKey { rung: ModuleDoc, file: test/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.634 |
| walker |  | 7435 | 38 | Markdown::Section { file: docs/reference/errors.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.634 |
| ns | 7481 |  | 186 | toFailure(): result normalisation and default message text | 6.1 | 2.9 | 0.625 |
| walker |  | 7546 | 111 | Markdown::Section { file: Readme.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.625 |
| walker |  | 7577 | 31 | Markdown::Section { file: docs/summary.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.625 |
| walker |  | 7639 | 62 | Markdown::HeadingsOutline { file: docs/reference/core.md } |  |  | 0.626 |
| walker |  | 7746 | 107 | Code::CodeKey { rung: Doc, file: src/error.ts, decl: 2, sub: 0, line: 25 } |  |  | 0.640 |
| ns | 7894 |  | 413 | object() implementation, including the mask special case | 6.2 | 3.6 | 0.620 |
| walker |  | 7922 | 176 | Markdown::Section { file: Readme.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.620 |
| ns | 8085 |  | 191 | refine() and define() bodies - the extension points | 6.3 | 2.4 | 0.613 |
| walker |  | 8117 | 195 | Markdown::Section { file: Readme.md, section_index: 4, keeps_default_concavity: true } |  |  | 0.613 |
| ns | 8199 |  | 114 | coerce() and trimmed() bodies | 6.4 | 2.5 | 0.608 |
| walker |  | 8252 | 135 | Markdown::Section { file: Readme.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.608 |
| ns | 8458 |  | 259 | npm scripts | 7.1 | 1.1 | 0.614 |
| walker |  | 8490 | 238 | Code::CodeKey { rung: Names, file: src/utils.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.623 |
| walker |  | 8510 | 20 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 11, sub: 0, line: 218 } |  |  | 0.623 |
| walker |  | 8560 | 50 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 7, sub: 0, line: 106 } |  |  | 0.623 |
| walker |  | 8610 | 50 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 9, sub: 0, line: 202 } |  |  | 0.623 |
| ns | 8629 |  | 171 | Changelog: recent release headings and the file's extent | 7.2 |  | 0.619 |
| walker |  | 8664 | 54 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 6, sub: 0, line: 67 } |  |  | 0.619 |
| ns | 8778 |  | 149 | The 2.0.0 release notes: breaking changes and fixes | 7.3 | 7.2 | 0.616 |
| walker |  | 8921 | 257 | Markdown::Section { file: License.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.616 |
| ns | 8957 |  | 179 | TypeScript compiler configuration | 7.4 |  | 0.610 |
| walker |  | 9023 | 102 | Code::CodeKey { rung: Decl, file: src/utils.ts, decl: 8, sub: 0, line: 130 } |  |  | 0.617 |
| walker |  | 9086 | 63 | Markdown::Section { file: docs/reference/coercions.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.617 |
| walker |  | 9165 | 79 | Markdown::HeadingsOutline { file: docs/reference/utilities.md } |  |  | 0.617 |
| walker |  | 9195 | 30 | Markdown::Section { file: docs/reference/utilities.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.617 |
| walker |  | 9206 | 11 | Code::CodeKey { rung: Body, file: src/struct.ts, decl: 5, sub: 0, line: 83 } |  |  | 0.617 |
| ns | 9207 |  | 250 | Rollup build and JSR publish config | 7.5 |  | 0.607 |
| walker |  | 9331 | 125 | Markdown::Section { file: docs/summary.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.607 |
| ns | 9368 |  | 161 | CI matrix and dependency automation | 7.6 |  | 0.602 |
| walker |  | 9413 | 82 | Markdown::HeadingsOutline { file: docs/reference/refinements.md } |  |  | 0.603 |
| walker |  | 9466 | 53 | Markdown::Section { file: docs/reference/refinements.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.603 |
| ns | 9521 |  | 153 | Remaining package.json keys: entry points and publish metadata | 7.7 | 1.1 | 0.608 |
| walker |  | 9543 | 77 | Code::CodeKey { rung: Doc, file: src/structs/coercions.ts, decl: 3, sub: 0, line: 79 } |  |  | 0.609 |
| walker |  | 9567 | 24 | Code::CodeKey { rung: Doc, file: src/structs/refinements.ts, decl: 5, sub: 0, line: 93 } |  |  | 0.609 |
| walker |  | 9590 | 23 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 11, sub: 0, line: 175 } |  |  | 0.609 |
| ns | 9599 |  | 78 | Formatting and docs-site configuration | 7.8 |  | 0.606 |
| walker |  | 9700 | 110 | Json::Whole { file: jsr.json } |  |  | 0.608 |
| ns | 9718 |  | 119 | ESLint configuration head | 7.9 |  | 0.604 |
| ns | 9847 |  | 129 | Examples package manifest and run instructions | 7.10 |  | 0.601 |
| walker |  | 9906 | 206 | Markdown::Section { file: Readme.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.601 |
| walker |  | 9932 | 26 | Json::Identity { file: examples/package.json } |  |  | 0.601 |
| walker |  | 9955 | 23 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 16, sub: 0, line: 243 } |  |  | 0.601 |
| walker |  | 9969 | 14 | Code::CodeKey { rung: Body, file: src/structs/types.ts, decl: 23, sub: 0, line: 258 } |  |  | 0.601 |
