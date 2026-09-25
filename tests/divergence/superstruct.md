Score(3000)=0.516 I=0.767 C=0.347 ns_rows≤3K=20/60 grid(1000/1442/2080/3000/4327/6240/9000)=0.640/0.498/0.571/0.516/0.578/0.611/0.610

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
| ns | 171 |  | 37 | Complete src/ and src/structs/ listings | 1.3 |  | 0.613 |
| walker |  | 190 | 55 | Fs::DirListing { dir: examples } |  |  | 0.617 |
| walker |  | 198 | 8 | Fs::DirListing { dir: .github } |  |  | 0.617 |
| walker |  | 202 | 4 | Fs::DirListing { dir: .github/workflows } |  |  | 0.617 |
| ns | 250 |  | 79 | src/index.ts - the entire public barrel | 1.4 |  | 0.556 |
| walker |  | 281 | 79 | Code::CodeKey { rung: Names, file: src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.640 |
| walker |  | 363 | 82 | Json::Identity { file: package.json } |  |  | 0.965 |
| walker |  | 392 | 29 | Fs::DirListing { dir: docs/images } |  |  | 0.966 |
| ns | 416 |  | 166 | Readme lede: what Superstruct is and why it exists | 1.5 |  | 0.937 |
| walker |  | 423 | 31 | Fs::DirListing { dir: docs/reference } |  |  | 0.939 |
| walker |  | 450 | 27 | Code::CodeKey { rung: Names, file: src/error.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.939 |
| walker |  | 484 | 34 | Json::Runtime { file: package.json } |  |  | 0.940 |
| walker |  | 535 | 51 | Fs::DirListing { dir: docs/guides } |  |  | 0.942 |
| walker |  | 565 | 30 | Markdown::HeadingsOutline { file: docs/summary.md } |  |  | 0.942 |
| walker |  | 583 | 18 | Markdown::HeadingsOutline { file: docs/resources/links.md } |  |  | 0.942 |
| walker |  | 611 | 28 | Fs::DirListing { dir: test } |  |  | 0.943 |
| ns | 638 |  | 222 | The Struct class: doc comment and its six fields | 1.6 |  | 0.780 |
| walker |  | 808 | 197 | Markdown::ReadmeHeadline { file: Readme.md } |  |  | 0.787 |
| ns | 816 |  | 178 | Core API signatures: assert / create / is / mask / validate | 1.7 |  | 0.712 |
| walker |  | 862 | 54 | Markdown::HeadingsOutline { file: Readme.md } |  |  | 0.713 |
| walker |  | 928 | 66 | Markdown::Prelude { file: Readme.md } |  |  | 0.729 |
| walker |  | 947 | 19 | Markdown::Section { file: Readme.md, section_index: 10, keeps_default_concavity: false } |  |  | 0.729 |
| ns | 971 |  | 155 | Readme canonical usage snippet | 1.8 |  | 0.640 |
| walker |  | 1023 | 76 | Json::Entry { file: package.json } |  |  | 0.641 |
| ns | 1066 |  | 95 | src/struct.ts symbol roster: methods and top-level helpers | 2.1 |  | 0.598 |
| walker |  | 1104 | 81 | Code::CodeKey { rung: Decl, file: src/error.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.602 |
| walker |  | 1235 | 131 | Markdown::ReadmeHeadline { file: docs/readme.md } |  |  | 0.602 |
| walker |  | 1301 | 66 | Markdown::Prelude { file: docs/readme.md } |  |  | 0.602 |
| ns | 1318 |  | 252 | Complete roster of the 25 type structs | 2.2 |  | 0.521 |
| walker |  | 1359 | 58 | Code::CodeKey { rung: Names, file: src/structs/coercions.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.523 |
| walker |  | 1406 | 47 | Code::CodeKey { rung: Decl, file: src/structs/coercions.ts, decl: 1, sub: 0, line: 16 } |  |  | 0.523 |
| ns | 1420 |  | 102 | Complete roster of src/structs/utilities.ts | 2.3 |  | 0.498 |
| walker |  | 1463 | 57 | Code::CodeKey { rung: Decl, file: src/structs/coercions.ts, decl: 2, sub: 0, line: 38 } |  |  | 0.498 |
| ns | 1503 |  | 83 | Complete roster of src/structs/refinements.ts | 2.4 |  | 0.480 |
| ns | 1537 |  | 34 | Complete roster of src/structs/coercions.ts | 2.5 |  | 0.491 |
| walker |  | 1576 | 113 | Code::CodeKey { rung: Decl, file: src/error.ts, decl: 2, sub: 0, line: 25 } |  |  | 0.493 |
| ns | 1651 |  | 114 | Failure - the shape of every validation failure | 2.6 |  | 0.508 |
| walker |  | 1809 | 233 | Markdown::Section { file: Readme.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.598 |
| ns | 1829 |  | 178 | StructError class: doc, fields, and its early-exit contract | 2.7 |  | 0.583 |
| walker |  | 1835 | 26 | Code::CodeKey { rung: Doc, file: src/error.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.601 |
| ns | 2037 |  | 208 | Exported type vocabulary of src/struct.ts | 2.8 |  | 0.571 |
| walker |  | 2117 | 282 | Markdown::Section { file: Readme.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.571 |
| ns | 2140 |  | 103 | Function roster of src/utils.ts | 2.9 |  | 0.555 |
| walker |  | 2142 | 25 | Fs::DirListing { dir: test/api } |  |  | 0.556 |
| walker |  | 2365 | 223 | Code::CodeKey { rung: Names, file: src/structs/types.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.567 |
| ns | 2378 |  | 238 | Type-level utility roster of src/utils.ts | 2.10 |  | 0.533 |
| walker |  | 2381 | 16 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 10, sub: 0, line: 105 } |  |  | 0.533 |
| walker |  | 2404 | 23 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 12, sub: 0, line: 144 } |  |  | 0.533 |
| walker |  | 2430 | 26 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 8, sub: 0, line: 99 } |  |  | 0.533 |
| walker |  | 2456 | 26 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 9, sub: 0, line: 102 } |  |  | 0.533 |
| walker |  | 2470 | 14 | Code::CodeKey { rung: Body, file: src/structs/types.ts, decl: 1, sub: 0, line: 19 } |  |  | 0.533 |
| walker |  | 2571 | 101 | Code::CodeKey { rung: Names, file: src/structs/refinements.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.558 |
| ns | 2574 |  | 196 | What each type struct does, part 1 (any ... literal) | 3.1 | 2.2 | 0.541 |
| walker |  | 2604 | 33 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 5, sub: 0, line: 93 } |  |  | 0.541 |
| walker |  | 2648 | 44 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 7, sub: 0, line: 146 } |  |  | 0.541 |
| walker |  | 2696 | 48 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 1, sub: 0, line: 8 } |  |  | 0.541 |
| walker |  | 2744 | 48 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 4, sub: 0, line: 77 } |  |  | 0.541 |
| walker |  | 2801 | 57 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 2, sub: 0, line: 33 } |  |  | 0.541 |
| walker |  | 2858 | 57 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 3, sub: 0, line: 55 } |  |  | 0.541 |
| ns | 2916 |  | 342 | What each type struct does, part 2 (map ... unknown) | 3.2 | 2.2 | 0.516 |
| walker |  | 2920 | 62 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 6, sub: 0, line: 109 } |  |  | 0.516 |
| ns | 3144 |  | 228 | What each utility struct does | 3.3 | 2.3 | 0.498 |
| walker |  | 3153 | 233 | Code::CodeKey { rung: Names, file: src/struct.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.528 |
| walker |  | 3187 | 34 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 13, sub: 0, line: 221 } |  |  | 0.540 |
| walker |  | 3224 | 37 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 9, sub: 0, line: 139 } |  |  | 0.540 |
| walker |  | 3261 | 37 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 10, sub: 0, line: 157 } |  |  | 0.540 |
| ns | 3300 |  | 156 | What each refinement does | 3.4 | 2.4 | 0.532 |
| walker |  | 3301 | 40 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 8, sub: 0, line: 123 } |  |  | 0.532 |
| walker |  | 3342 | 41 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 16, sub: 0, line: 243 } |  |  | 0.550 |
| ns | 3411 |  | 111 | What each coercion does - and the create() gotcha | 3.5 | 2.5 | 0.544 |
| walker |  | 3425 | 83 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 12, sub: 0, line: 185 } |  |  | 0.544 |
| walker |  | 3447 | 22 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 1, sub: 0, line: 19 } |  |  | 0.544 |
| walker |  | 3466 | 19 | Code::CodeKey { rung: Body, file: src/structs/coercions.ts, decl: 3, sub: 0, line: 79 } |  |  | 0.544 |
| walker |  | 3485 | 19 | Markdown::Section { file: docs/resources/links.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.544 |
| walker |  | 3610 | 125 | Fs::DirListing { dir: test/validation } |  |  | 0.549 |
| ns | 3716 |  | 305 | Public overload declarations of the six overloaded factories | 3.6 | 2.2 | 0.531 |
| walker |  | 3841 | 231 | Code::CodeKey { rung: Names, file: src/structs/types.ts, decl: 0, sub: 1, line: 0 } |  |  | 0.565 |
| walker |  | 3872 | 31 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 21, sub: 0, line: 226 } |  |  | 0.573 |
| walker |  | 3912 | 40 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 14, sub: 0, line: 172 } |  |  | 0.573 |
| ns | 3959 |  | 243 | Struct method semantics: mask recursion and validate options | 3.7 | 2.1 | 0.559 |
| ns | 4149 |  | 190 | What each src/utils.ts helper does | 3.8 | 2.9 | 0.547 |
| walker |  | 4156 | 244 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 1, sub: 0, line: 10 } |  |  | 0.592 |
| walker |  | 4228 | 72 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 2, sub: 0, line: 22 } |  |  | 0.592 |
| ns | 4298 |  | 149 | run() signature and per-call context setup | 4.1 | 2.9 | 0.578 |
| walker |  | 4300 | 72 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 7, sub: 0, line: 107 } |  |  | 0.578 |
| walker |  | 4332 | 32 | Markdown::HeadingsOutline { file: docs/guides/05-handling-errors.md } |  |  | 0.578 |
| walker |  | 4366 | 34 | Markdown::HeadingsOutline { file: docs/guides/03-coercing-data.md } |  |  | 0.578 |
| walker |  | 4400 | 34 | Markdown::HeadingsOutline { file: docs/guides/04-refining-validation.md } |  |  | 0.579 |
| ns | 4427 |  | 129 | run() stage 1: coercion, then the struct's own validator | 4.2 | 4.1 | 0.567 |
| walker |  | 4436 | 36 | Markdown::HeadingsOutline { file: docs/guides/01-getting-started.md } |  |  | 0.567 |
| ns | 4759 |  | 332 | run() stage 2: recursive descent over entries | 4.3 | 4.1 | 0.542 |
| walker |  | 4762 | 326 | Json::Scripts { file: package.json } |  |  | 0.542 |
| ns | 4882 |  | 123 | run() stage 3: refiners and the success yield | 4.4 | 4.1 | 0.532 |
| walker |  | 4982 | 220 | Code::CodeKey { rung: Names, file: src/structs/utilities.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.556 |
| walker |  | 5015 | 33 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 8, sub: 0, line: 106 } |  |  | 0.556 |
| walker |  | 5052 | 37 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 11, sub: 0, line: 197 } |  |  | 0.556 |
| ns | 5054 |  | 172 | validate() body: how one failure becomes a StructError | 4.5 | 2.1 | 0.542 |
| walker |  | 5090 | 38 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 7, sub: 0, line: 80 } |  |  | 0.542 |
| walker |  | 5133 | 43 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 12, sub: 0, line: 221 } |  |  | 0.542 |
| walker |  | 5177 | 44 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 10, sub: 0, line: 171 } |  |  | 0.542 |
| walker |  | 5222 | 45 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 1, sub: 0, line: 17 } |  |  | 0.542 |
| ns | 5265 |  | 211 | Struct constructor: defaults and failure wrapping | 4.6 | 1.6 | 0.525 |
| walker |  | 5324 | 102 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 2, sub: 0, line: 21 } |  |  | 0.525 |
| ns | 5456 |  | 191 | StructError constructor: message, cause and failure caching | 4.7 | 2.7 | 0.515 |
| walker |  | 5474 | 150 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 3, sub: 0, line: 30 } |  |  | 0.515 |
| ns | 5596 |  | 140 | Complete docs/ tree listing | 5.1 |  | 0.542 |
| ns | 5649 |  | 53 | Complete test/ and test/api/ listings | 5.2 |  | 0.553 |
| walker |  | 5657 | 183 | Fs::DirListing { dir: test/typings } |  |  | 0.557 |
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
