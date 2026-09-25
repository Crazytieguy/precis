Score(3000)=0.423 I=0.766 C=0.234 ns_rows≤3K=20/60 grid(1000/1442/2080/3000/4327/6240/9000)=0.577/0.495/0.499/0.423/0.452/0.606/0.577

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
| walker |  | 1138 | 79 | Code::CodeKey { rung: Names, file: src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.597 |
| walker |  | 1193 | 55 | Fs::DirListing { dir: examples } |  |  | 0.598 |
| walker |  | 1218 | 25 | Fs::DirListing { dir: test/api } |  |  | 0.599 |
| ns | 1318 |  | 252 | Complete roster of the 25 type structs | 2.2 |  | 0.519 |
| ns | 1420 |  | 102 | Complete roster of src/structs/utilities.ts | 2.3 |  | 0.495 |
| walker |  | 1451 | 233 | Markdown::Section { file: Readme.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.600 |
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
| walker |  | 2839 | 40 | Markdown::HeadingsOutline { file: docs/reference/errors.md } |  |  | 0.444 |
| ns | 2916 |  | 342 | What each type struct does, part 2 (map ... unknown) | 3.2 | 2.2 | 0.423 |
| walker |  | 3140 | 301 | Json::IdentityMeta { file: package.json } |  |  | 0.437 |
| ns | 3144 |  | 228 | What each utility struct does | 3.3 | 2.3 | 0.422 |
| walker |  | 3192 | 52 | Markdown::HeadingsOutline { file: docs/guides/02-validating-data.md } |  |  | 0.423 |
| walker |  | 3234 | 42 | Markdown::HeadingsOutline { file: docs/reference/typescript.md } |  |  | 0.423 |
| walker |  | 3261 | 27 | Markdown::Section { file: docs/reference/typescript.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.423 |
| ns | 3300 |  | 156 | What each refinement does | 3.4 | 2.4 | 0.416 |
| ns | 3411 |  | 111 | What each coercion does - and the create() gotcha | 3.5 | 2.5 | 0.412 |
| walker |  | 3543 | 282 | Markdown::Section { file: docs/readme.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.412 |
| walker |  | 3588 | 45 | Markdown::HeadingsOutline { file: docs/reference/coercions.md } |  |  | 0.412 |
| walker |  | 3625 | 37 | Markdown::Section { file: docs/guides/02-validating-data.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.412 |
| walker |  | 3663 | 38 | Markdown::Section { file: docs/reference/errors.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.412 |
| walker |  | 3687 | 24 | Code::CodeKey { rung: ModuleDoc, file: test/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.412 |
| walker |  | 3714 | 27 | Code::CodeKey { rung: Names, file: src/error.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.413 |
| ns | 3716 |  | 305 | Public overload declarations of the six overloaded factories | 3.6 | 2.2 | 0.393 |
| walker |  | 3795 | 81 | Code::CodeKey { rung: Decl, file: src/error.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.418 |
| walker |  | 3908 | 113 | Code::CodeKey { rung: Decl, file: src/error.ts, decl: 2, sub: 0, line: 25 } |  |  | 0.432 |
| walker |  | 3934 | 26 | Code::CodeKey { rung: Doc, file: src/error.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.446 |
| ns | 3959 |  | 243 | Struct method semantics: mask recursion and validate options | 3.7 | 2.1 | 0.435 |
| walker |  | 4045 | 111 | Markdown::Section { file: Readme.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.435 |
| walker |  | 4076 | 31 | Markdown::Section { file: docs/summary.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.435 |
| walker |  | 4138 | 62 | Markdown::HeadingsOutline { file: docs/reference/core.md } |  |  | 0.437 |
| ns | 4149 |  | 190 | What each src/utils.ts helper does | 3.8 | 2.9 | 0.428 |
| walker |  | 4245 | 107 | Code::CodeKey { rung: Doc, file: src/error.ts, decl: 2, sub: 0, line: 25 } |  |  | 0.462 |
| ns | 4298 |  | 149 | run() signature and per-call context setup | 4.1 | 2.9 | 0.452 |
| ns | 4427 |  | 129 | run() stage 1: coercion, then the struct's own validator | 4.2 | 4.1 | 0.442 |
| walker |  | 4478 | 233 | Code::CodeKey { rung: Names, file: src/struct.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.468 |
| walker |  | 4512 | 34 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 13, sub: 0, line: 221 } |  |  | 0.480 |
| walker |  | 4549 | 37 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 9, sub: 0, line: 139 } |  |  | 0.480 |
| walker |  | 4586 | 37 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 10, sub: 0, line: 157 } |  |  | 0.480 |
| walker |  | 4626 | 40 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 8, sub: 0, line: 123 } |  |  | 0.480 |
| walker |  | 4667 | 41 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 16, sub: 0, line: 243 } |  |  | 0.496 |
| walker |  | 4750 | 83 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 12, sub: 0, line: 185 } |  |  | 0.496 |
| ns | 4759 |  | 332 | run() stage 2: recursive descent over entries | 4.3 | 4.1 | 0.473 |
| walker |  | 4773 | 23 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 11, sub: 0, line: 175 } |  |  | 0.473 |
| walker |  | 4796 | 23 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 16, sub: 0, line: 243 } |  |  | 0.473 |
| walker |  | 4821 | 25 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 18, sub: 0, line: 259 } |  |  | 0.473 |
| walker |  | 4848 | 27 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 14, sub: 0, line: 231 } |  |  | 0.473 |
| walker |  | 4875 | 27 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 15, sub: 0, line: 237 } |  |  | 0.473 |
| ns | 4882 |  | 123 | run() stage 3: refiners and the success yield | 4.4 | 4.1 | 0.465 |
| walker |  | 4903 | 28 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 8, sub: 0, line: 123 } |  |  | 0.465 |
| walker |  | 4931 | 28 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 17, sub: 0, line: 253 } |  |  | 0.465 |
| walker |  | 4960 | 29 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 9, sub: 0, line: 139 } |  |  | 0.465 |
| walker |  | 4990 | 30 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 10, sub: 0, line: 157 } |  |  | 0.465 |
| walker |  | 5029 | 39 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 19, sub: 0, line: 266 } |  |  | 0.465 |
| ns | 5054 |  | 172 | validate() body: how one failure becomes a StructError | 4.5 | 2.1 | 0.453 |
| ns | 5265 |  | 211 | Struct constructor: defaults and failure wrapping | 4.6 | 1.6 | 0.439 |
| walker |  | 5271 | 242 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 1, sub: 0, line: 10 } |  |  | 0.478 |
| walker |  | 5343 | 72 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 2, sub: 0, line: 22 } |  |  | 0.478 |
| walker |  | 5415 | 72 | Code::CodeKey { rung: Decl, file: src/struct.ts, decl: 7, sub: 0, line: 107 } |  |  | 0.478 |
| walker |  | 5443 | 28 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 5, sub: 0, line: 83 } |  |  | 0.478 |
| ns | 5456 |  | 191 | StructError constructor: message, cause and failure caching | 4.7 | 2.7 | 0.469 |
| walker |  | 5476 | 33 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 3, sub: 0, line: 67 } |  |  | 0.469 |
| walker |  | 5509 | 33 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 4, sub: 0, line: 75 } |  |  | 0.469 |
| walker |  | 5555 | 46 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 12, sub: 0, line: 185 } |  |  | 0.469 |
| ns | 5596 |  | 140 | Complete docs/ tree listing | 5.1 |  | 0.503 |
| walker |  | 5624 | 69 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 1, sub: 0, line: 10 } |  |  | 0.522 |
| ns | 5649 |  | 53 | Complete test/ and test/api/ listings | 5.2 |  | 0.535 |
| walker |  | 5693 | 69 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 6, sub: 0, line: 93 } |  |  | 0.536 |
| ns | 5774 |  | 125 | All 41 validation-fixture kind directories | 5.3 |  | 0.572 |
| walker |  | 5790 | 97 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 13, sub: 0, line: 221 } |  |  | 0.578 |
| ns | 5829 |  | 55 | Complete examples/ listing | 5.4 |  | 0.586 |
| ns | 5906 |  | 77 | One kind directory listed in full, as the case-naming exemplar | 5.5 |  | 0.578 |
| walker |  | 5966 | 176 | Markdown::Section { file: Readme.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.578 |
| ns | 6089 |  | 183 | The 45 type-level test files | 5.6 |  | 0.606 |
| walker |  | 6161 | 195 | Markdown::Section { file: Readme.md, section_index: 4, keeps_default_concavity: true } |  |  | 0.606 |
| walker |  | 6296 | 135 | Markdown::Section { file: Readme.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.606 |
| ns | 6316 |  | 227 | Every heading in the six guides | 5.7 |  | 0.617 |
| ns | 6379 |  | 63 | Readme section map | 5.8 |  | 0.620 |
| ns | 6519 |  | 140 | Doc anchors the source rosters do not cover | 5.9 |  | 0.617 |
| walker |  | 6553 | 257 | Markdown::Section { file: License.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.617 |
| walker |  | 6616 | 63 | Markdown::Section { file: docs/reference/coercions.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.617 |
| ns | 6690 |  | 171 | The data-driven fixture runner | 5.10 |  | 0.611 |
| walker |  | 6695 | 79 | Markdown::HeadingsOutline { file: docs/reference/utilities.md } |  |  | 0.611 |
| walker |  | 6725 | 30 | Markdown::Section { file: docs/reference/utilities.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.611 |
| walker |  | 6860 | 135 | Code::CodeKey { rung: Doc, file: src/struct.ts, decl: 7, sub: 0, line: 107 } |  |  | 0.620 |
| ns | 6891 |  | 201 | Runner assertions: output-style vs failures-style fixtures | 5.11 | 5.10 | 0.611 |
| walker |  | 6985 | 125 | Markdown::Section { file: docs/summary.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.611 |
| walker |  | 7067 | 82 | Markdown::HeadingsOutline { file: docs/reference/refinements.md } |  |  | 0.612 |
| walker |  | 7120 | 53 | Markdown::Section { file: docs/reference/refinements.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.612 |
| ns | 7164 |  | 273 | A failures-style and an output-style fixture, in full | 5.12 | 5.10 | 0.596 |
| walker |  | 7221 | 101 | Code::CodeKey { rung: Names, file: src/structs/refinements.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.604 |
| walker |  | 7254 | 33 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 5, sub: 0, line: 93 } |  |  | 0.604 |
| ns | 7295 |  | 131 | The typings harness and one typings test | 5.13 | 5.6 | 0.599 |
| walker |  | 7298 | 44 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 7, sub: 0, line: 146 } |  |  | 0.599 |
| walker |  | 7346 | 48 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 1, sub: 0, line: 8 } |  |  | 0.599 |
| walker |  | 7394 | 48 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 4, sub: 0, line: 77 } |  |  | 0.599 |
| walker |  | 7451 | 57 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 2, sub: 0, line: 33 } |  |  | 0.599 |
| ns | 7481 |  | 186 | toFailure(): result normalisation and default message text | 6.1 | 2.9 | 0.590 |
| walker |  | 7508 | 57 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 3, sub: 0, line: 55 } |  |  | 0.590 |
| walker |  | 7570 | 62 | Code::CodeKey { rung: Decl, file: src/structs/refinements.ts, decl: 6, sub: 0, line: 109 } |  |  | 0.590 |
| walker |  | 7594 | 24 | Code::CodeKey { rung: Doc, file: src/structs/refinements.ts, decl: 5, sub: 0, line: 93 } |  |  | 0.590 |
| walker |  | 7620 | 26 | Code::CodeKey { rung: Doc, file: src/structs/refinements.ts, decl: 2, sub: 0, line: 33 } |  |  | 0.591 |
| walker |  | 7646 | 26 | Code::CodeKey { rung: Doc, file: src/structs/refinements.ts, decl: 3, sub: 0, line: 55 } |  |  | 0.591 |
| walker |  | 7675 | 29 | Code::CodeKey { rung: Doc, file: src/structs/refinements.ts, decl: 1, sub: 0, line: 8 } |  |  | 0.592 |
| walker |  | 7704 | 29 | Code::CodeKey { rung: Doc, file: src/structs/refinements.ts, decl: 4, sub: 0, line: 77 } |  |  | 0.593 |
| walker |  | 7752 | 48 | Code::CodeKey { rung: Doc, file: src/structs/refinements.ts, decl: 6, sub: 0, line: 109 } |  |  | 0.596 |
| walker |  | 7843 | 91 | Code::CodeKey { rung: Doc, file: src/structs/refinements.ts, decl: 7, sub: 0, line: 146 } |  |  | 0.598 |
| ns | 7894 |  | 413 | object() implementation, including the mask special case | 6.2 | 3.6 | 0.580 |
| walker |  | 7953 | 110 | Json::Whole { file: jsr.json } |  |  | 0.580 |
| ns | 8085 |  | 191 | refine() and define() bodies - the extension points | 6.3 | 2.4 | 0.573 |
| walker |  | 8173 | 220 | Code::CodeKey { rung: Names, file: src/structs/utilities.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.586 |
| ns | 8199 |  | 114 | coerce() and trimmed() bodies | 6.4 | 2.5 | 0.581 |
| walker |  | 8206 | 33 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 8, sub: 0, line: 106 } |  |  | 0.581 |
| walker |  | 8243 | 37 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 11, sub: 0, line: 197 } |  |  | 0.581 |
| walker |  | 8281 | 38 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 7, sub: 0, line: 80 } |  |  | 0.581 |
| walker |  | 8324 | 43 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 12, sub: 0, line: 221 } |  |  | 0.581 |
| walker |  | 8368 | 44 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 10, sub: 0, line: 171 } |  |  | 0.581 |
| walker |  | 8413 | 45 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 1, sub: 0, line: 17 } |  |  | 0.581 |
| ns | 8458 |  | 259 | npm scripts | 7.1 | 1.1 | 0.588 |
| walker |  | 8515 | 102 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 2, sub: 0, line: 21 } |  |  | 0.588 |
| ns | 8629 |  | 171 | Changelog: recent release headings and the file's extent | 7.2 |  | 0.584 |
| walker |  | 8665 | 150 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 3, sub: 0, line: 30 } |  |  | 0.584 |
| walker |  | 8691 | 26 | Code::CodeKey { rung: Doc, file: src/structs/utilities.ts, decl: 6, sub: 0, line: 71 } |  |  | 0.584 |
| ns | 8778 |  | 149 | The 2.0.0 release notes: breaking changes and fixes | 7.3 | 7.2 | 0.582 |
| walker |  | 8873 | 182 | Code::CodeKey { rung: Decl, file: src/structs/utilities.ts, decl: 4, sub: 0, line: 44 } |  |  | 0.582 |
| walker |  | 8920 | 47 | Code::CodeKey { rung: Doc, file: src/structs/utilities.ts, decl: 13, sub: 0, line: 247 } |  |  | 0.582 |
| ns | 8957 |  | 179 | TypeScript compiler configuration | 7.4 |  | 0.576 |
| walker |  | 8975 | 55 | Code::CodeKey { rung: Doc, file: src/structs/utilities.ts, decl: 7, sub: 0, line: 80 } |  |  | 0.577 |
| walker |  | 9032 | 57 | Code::CodeKey { rung: Doc, file: src/structs/utilities.ts, decl: 10, sub: 0, line: 171 } |  |  | 0.579 |
| walker |  | 9089 | 57 | Code::CodeKey { rung: Doc, file: src/structs/utilities.ts, decl: 12, sub: 0, line: 221 } |  |  | 0.581 |
| walker |  | 9152 | 63 | Code::CodeKey { rung: Doc, file: src/structs/utilities.ts, decl: 11, sub: 0, line: 197 } |  |  | 0.584 |
| ns | 9207 |  | 250 | Rollup build and JSR publish config | 7.5 |  | 0.577 |
| walker |  | 9220 | 68 | Code::CodeKey { rung: Doc, file: src/structs/utilities.ts, decl: 1, sub: 0, line: 17 } |  |  | 0.581 |
| walker |  | 9301 | 81 | Code::CodeKey { rung: Doc, file: src/structs/utilities.ts, decl: 8, sub: 0, line: 106 } |  |  | 0.582 |
| ns | 9368 |  | 161 | CI matrix and dependency automation | 7.6 |  | 0.577 |
| walker |  | 9404 | 103 | Code::CodeKey { rung: Doc, file: src/structs/utilities.ts, decl: 9, sub: 0, line: 140 } |  |  | 0.579 |
| ns | 9521 |  | 153 | Remaining package.json keys: entry points and publish metadata | 7.7 | 1.1 | 0.585 |
| ns | 9599 |  | 78 | Formatting and docs-site configuration | 7.8 |  | 0.582 |
| walker |  | 9610 | 206 | Markdown::Section { file: Readme.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.582 |
| walker |  | 9636 | 26 | Json::Identity { file: examples/package.json } |  |  | 0.582 |
| ns | 9718 |  | 119 | ESLint configuration head | 7.9 |  | 0.578 |
| ns | 9847 |  | 129 | Examples package manifest and run instructions | 7.10 |  | 0.576 |
| walker |  | 9859 | 223 | Code::CodeKey { rung: Names, file: src/structs/types.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.580 |
| walker |  | 9875 | 16 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 10, sub: 0, line: 105 } |  |  | 0.580 |
| walker |  | 9898 | 23 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 12, sub: 0, line: 144 } |  |  | 0.580 |
| walker |  | 9924 | 26 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 8, sub: 0, line: 99 } |  |  | 0.581 |
| walker |  | 9950 | 26 | Code::CodeKey { rung: Decl, file: src/structs/types.ts, decl: 9, sub: 0, line: 102 } |  |  | 0.582 |
| walker |  | 9964 | 14 | Code::CodeKey { rung: Body, file: src/structs/types.ts, decl: 1, sub: 0, line: 19 } |  |  | 0.582 |
| walker |  | 9986 | 22 | Code::CodeKey { rung: Doc, file: src/structs/types.ts, decl: 1, sub: 0, line: 19 } |  |  | 0.582 |
