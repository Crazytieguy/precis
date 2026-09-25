Score(3000)=0.516 I=0.767 C=0.347 ns_rows≤3K=20/60 grid(1000/1442/2080/3000/4327/6240/9000)=0.640/0.498/0.571/0.516/0.578/0.611/0.617

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 65 | 65 | listing of '.' |  |  | 0.000 |
| ns | 69 |  | 69 | Package identity: name, description, version, license | 1.1 |  | 0.000 |
| walker |  | 84 | 19 | listing of 'src' |  |  | 0.000 |
| walker |  | 102 | 18 | listing of 'src/structs' |  |  | 0.000 |
| walker |  | 123 | 21 | listing of 'docs' |  |  | 0.000 |
| walker |  | 131 | 8 | listing of 'docs/resources' |  |  | 0.000 |
| ns | 134 |  | 65 | Complete root directory listing | 1.2 |  | 0.616 |
| walker |  | 135 | 4 | listing of '.vscode' |  |  | 0.616 |
| ns | 171 |  | 37 | Complete src/ and src/structs/ listings | 1.3 |  | 0.613 |
| walker |  | 190 | 55 | listing of 'examples' |  |  | 0.617 |
| walker |  | 198 | 8 | listing of '.github' |  |  | 0.617 |
| walker |  | 202 | 4 | listing of '.github/workflows' |  |  | 0.617 |
| ns | 250 |  | 79 | src/index.ts - the entire public barrel | 1.4 |  | 0.556 |
| walker |  | 281 | 79 | ts names src/index.ts |  |  | 0.640 |
| walker |  | 363 | 82 | package identity in package.json |  |  | 0.965 |
| walker |  | 392 | 29 | listing of 'docs/images' |  |  | 0.966 |
| ns | 416 |  | 166 | Readme lede: what Superstruct is and why it exists | 1.5 |  | 0.937 |
| walker |  | 423 | 31 | listing of 'docs/reference' |  |  | 0.939 |
| walker |  | 450 | 27 | ts names src/error.ts |  |  | 0.939 |
| walker |  | 484 | 34 | package runtime metadata in package.json |  |  | 0.940 |
| walker |  | 535 | 51 | listing of 'docs/guides' |  |  | 0.942 |
| walker |  | 565 | 30 | headings outline in docs/summary.md |  |  | 0.942 |
| walker |  | 583 | 18 | headings outline in docs/resources/links.md |  |  | 0.942 |
| walker |  | 611 | 28 | listing of 'test' |  |  | 0.943 |
| ns | 638 |  | 222 | The Struct class: doc comment and its six fields | 1.6 |  | 0.780 |
| walker |  | 808 | 197 | README headline in Readme.md |  |  | 0.787 |
| ns | 816 |  | 178 | Core API signatures: assert / create / is / mask / validate | 1.7 |  | 0.712 |
| walker |  | 862 | 54 | headings outline in Readme.md |  |  | 0.713 |
| walker |  | 928 | 66 | README prelude in Readme.md |  |  | 0.729 |
| walker |  | 947 | 19 | Readme.md section #10 |  |  | 0.729 |
| ns | 971 |  | 155 | Readme canonical usage snippet | 1.8 |  | 0.640 |
| walker |  | 1023 | 76 | package entrypoints in package.json |  |  | 0.641 |
| ns | 1066 |  | 95 | src/struct.ts symbol roster: methods and top-level helpers | 2.1 |  | 0.598 |
| walker |  | 1104 | 81 | ts decl src/error.ts:5 |  |  | 0.602 |
| walker |  | 1235 | 131 | README headline in docs/readme.md |  |  | 0.602 |
| walker |  | 1301 | 66 | README prelude in docs/readme.md |  |  | 0.602 |
| ns | 1318 |  | 252 | Complete roster of the 25 type structs | 2.2 |  | 0.521 |
| walker |  | 1359 | 58 | ts names src/structs/coercions.ts |  |  | 0.523 |
| walker |  | 1406 | 47 | ts decl src/structs/coercions.ts:16 |  |  | 0.523 |
| ns | 1420 |  | 102 | Complete roster of src/structs/utilities.ts | 2.3 |  | 0.498 |
| walker |  | 1463 | 57 | ts decl src/structs/coercions.ts:38 |  |  | 0.498 |
| ns | 1503 |  | 83 | Complete roster of src/structs/refinements.ts | 2.4 |  | 0.480 |
| ns | 1537 |  | 34 | Complete roster of src/structs/coercions.ts | 2.5 |  | 0.491 |
| walker |  | 1576 | 113 | ts decl src/error.ts:25 |  |  | 0.493 |
| ns | 1651 |  | 114 | Failure - the shape of every validation failure | 2.6 |  | 0.508 |
| walker |  | 1809 | 233 | Readme.md section #0 |  |  | 0.598 |
| ns | 1829 |  | 178 | StructError class: doc, fields, and its early-exit contract | 2.7 |  | 0.583 |
| walker |  | 1835 | 26 | ts doc src/error.ts:5 |  |  | 0.601 |
| ns | 2037 |  | 208 | Exported type vocabulary of src/struct.ts | 2.8 |  | 0.571 |
| walker |  | 2117 | 282 | Readme.md section #1 |  |  | 0.571 |
| ns | 2140 |  | 103 | Function roster of src/utils.ts | 2.9 |  | 0.555 |
| walker |  | 2142 | 25 | listing of 'test/api' |  |  | 0.556 |
| walker |  | 2365 | 223 | ts names src/structs/types.ts |  |  | 0.567 |
| ns | 2378 |  | 238 | Type-level utility roster of src/utils.ts | 2.10 |  | 0.533 |
| walker |  | 2381 | 16 | ts decl src/structs/types.ts:105 |  |  | 0.533 |
| walker |  | 2404 | 23 | ts decl src/structs/types.ts:144 |  |  | 0.533 |
| walker |  | 2430 | 26 | ts decl src/structs/types.ts:99 |  |  | 0.533 |
| walker |  | 2456 | 26 | ts decl src/structs/types.ts:102 |  |  | 0.533 |
| walker |  | 2557 | 101 | ts names src/structs/refinements.ts |  |  | 0.558 |
| ns | 2574 |  | 196 | What each type struct does, part 1 (any ... literal) | 3.1 | 2.2 | 0.541 |
| walker |  | 2590 | 33 | ts decl src/structs/refinements.ts:93 |  |  | 0.541 |
| walker |  | 2634 | 44 | ts decl src/structs/refinements.ts:146 |  |  | 0.541 |
| walker |  | 2682 | 48 | ts decl src/structs/refinements.ts:8 |  |  | 0.541 |
| walker |  | 2730 | 48 | ts decl src/structs/refinements.ts:77 |  |  | 0.541 |
| walker |  | 2787 | 57 | ts decl src/structs/refinements.ts:33 |  |  | 0.541 |
| walker |  | 2844 | 57 | ts decl src/structs/refinements.ts:55 |  |  | 0.541 |
| walker |  | 2906 | 62 | ts decl src/structs/refinements.ts:109 |  |  | 0.541 |
| ns | 2916 |  | 342 | What each type struct does, part 2 (map ... unknown) | 3.2 | 2.2 | 0.516 |
| walker |  | 3139 | 233 | ts names src/struct.ts |  |  | 0.547 |
| ns | 3144 |  | 228 | What each utility struct does | 3.3 | 2.3 | 0.528 |
| walker |  | 3173 | 34 | ts decl src/struct.ts:221 |  |  | 0.540 |
| walker |  | 3210 | 37 | ts decl src/struct.ts:139 |  |  | 0.540 |
| walker |  | 3247 | 37 | ts decl src/struct.ts:157 |  |  | 0.540 |
| walker |  | 3287 | 40 | ts decl src/struct.ts:123 |  |  | 0.540 |
| ns | 3300 |  | 156 | What each refinement does | 3.4 | 2.4 | 0.532 |
| walker |  | 3328 | 41 | ts decl src/struct.ts:243 |  |  | 0.550 |
| walker |  | 3411 | 83 | ts decl src/struct.ts:185 |  |  | 0.544 |
| ns | 3411 |  | 111 | What each coercion does - and the create() gotcha | 3.5 | 2.5 | 0.544 |
| walker |  | 3430 | 19 | ts body src/structs/coercions.ts:79 |  |  | 0.544 |
| walker |  | 3449 | 19 | docs/resources/links.md section #0 |  |  | 0.544 |
| walker |  | 3574 | 125 | listing of 'test/validation' |  |  | 0.549 |
| ns | 3716 |  | 305 | Public overload declarations of the six overloaded factories | 3.6 | 2.2 | 0.530 |
| walker |  | 3805 | 231 | ts names src/structs/types.ts #1 |  |  | 0.565 |
| walker |  | 3836 | 31 | ts decl src/structs/types.ts:226 |  |  | 0.573 |
| walker |  | 3876 | 40 | ts decl src/structs/types.ts:172 |  |  | 0.573 |
| ns | 3959 |  | 243 | Struct method semantics: mask recursion and validate options | 3.7 | 2.1 | 0.559 |
| walker |  | 4120 | 244 | ts decl src/struct.ts:10 |  |  | 0.604 |
| ns | 4149 |  | 190 | What each src/utils.ts helper does | 3.8 | 2.9 | 0.592 |
| walker |  | 4192 | 72 | ts decl src/struct.ts:22 |  |  | 0.592 |
| walker |  | 4264 | 72 | ts decl src/struct.ts:107 |  |  | 0.592 |
| walker |  | 4296 | 32 | headings outline in docs/guides/05-handling-errors.md |  |  | 0.592 |
| ns | 4298 |  | 149 | run() signature and per-call context setup | 4.1 | 2.9 | 0.578 |
| walker |  | 4330 | 34 | headings outline in docs/guides/03-coercing-data.md |  |  | 0.578 |
| walker |  | 4364 | 34 | headings outline in docs/guides/04-refining-validation.md |  |  | 0.578 |
| walker |  | 4400 | 36 | headings outline in docs/guides/01-getting-started.md |  |  | 0.579 |
| ns | 4427 |  | 129 | run() stage 1: coercion, then the struct's own validator | 4.2 | 4.1 | 0.567 |
| walker |  | 4726 | 326 | package scripts in package.json |  |  | 0.568 |
| ns | 4759 |  | 332 | run() stage 2: recursive descent over entries | 4.3 | 4.1 | 0.542 |
| ns | 4882 |  | 123 | run() stage 3: refiners and the success yield | 4.4 | 4.1 | 0.532 |
| walker |  | 4946 | 220 | ts names src/structs/utilities.ts |  |  | 0.556 |
| walker |  | 4979 | 33 | ts decl src/structs/utilities.ts:106 |  |  | 0.556 |
| walker |  | 5016 | 37 | ts decl src/structs/utilities.ts:197 |  |  | 0.556 |
| walker |  | 5054 | 38 | ts decl src/structs/utilities.ts:80 |  |  | 0.542 |
| ns | 5054 |  | 172 | validate() body: how one failure becomes a StructError | 4.5 | 2.1 | 0.542 |
| walker |  | 5097 | 43 | ts decl src/structs/utilities.ts:221 |  |  | 0.542 |
| walker |  | 5141 | 44 | ts decl src/structs/utilities.ts:171 |  |  | 0.542 |
| walker |  | 5186 | 45 | ts decl src/structs/utilities.ts:17 |  |  | 0.542 |
| ns | 5265 |  | 211 | Struct constructor: defaults and failure wrapping | 4.6 | 1.6 | 0.525 |
| walker |  | 5288 | 102 | ts decl src/structs/utilities.ts:21 |  |  | 0.525 |
| walker |  | 5438 | 150 | ts decl src/structs/utilities.ts:30 |  |  | 0.525 |
| ns | 5456 |  | 191 | StructError constructor: message, cause and failure caching | 4.7 | 2.7 | 0.515 |
| ns | 5596 |  | 140 | Complete docs/ tree listing | 5.1 |  | 0.542 |
| walker |  | 5621 | 183 | listing of 'test/typings' |  |  | 0.546 |
| ns | 5649 |  | 53 | Complete test/ and test/api/ listings | 5.2 |  | 0.557 |
| ns | 5774 |  | 125 | All 41 validation-fixture kind directories | 5.3 |  | 0.587 |
| ns | 5829 |  | 55 | Complete examples/ listing | 5.4 |  | 0.594 |
| walker |  | 5854 | 233 | docs/readme.md section #0 |  |  | 0.594 |
| walker |  | 5898 | 44 | headings outline in docs/guides/06-using-typescript.md |  |  | 0.595 |
| ns | 5906 |  | 77 | One kind directory listed in full, as the case-naming exemplar | 5.5 |  | 0.586 |
| walker |  | 6080 | 182 | ts decl src/structs/utilities.ts:44 |  |  | 0.586 |
| ns | 6089 |  | 183 | The 45 type-level test files | 5.6 |  | 0.611 |
| walker |  | 6120 | 40 | headings outline in docs/reference/errors.md |  |  | 0.611 |
| ns | 6316 |  | 227 | Every heading in the six guides | 5.7 |  | 0.612 |
| ns | 6379 |  | 63 | Readme section map | 5.8 |  | 0.615 |
| walker |  | 6421 | 301 | package identity metadata in package.json |  |  | 0.629 |
| ns | 6519 |  | 140 | Doc anchors the source rosters do not cover | 5.9 |  | 0.623 |
| walker |  | 6690 | 269 | ts names src/structs/types.ts #2 |  |  | 0.652 |
| ns | 6690 |  | 171 | The data-driven fixture runner | 5.10 |  | 0.652 |
| walker |  | 6709 | 19 | ts decl src/structs/types.ts:295 |  |  | 0.655 |
| walker |  | 6731 | 22 | ts decl src/structs/types.ts:492 |  |  | 0.655 |
| walker |  | 6765 | 34 | ts decl src/structs/types.ts:367 |  |  | 0.655 |
| walker |  | 6799 | 34 | ts decl src/structs/types.ts:456 |  |  | 0.655 |
| walker |  | 6834 | 35 | ts decl src/structs/types.ts:522 |  |  | 0.655 |
| walker |  | 6886 | 52 | headings outline in docs/guides/02-validating-data.md |  |  | 0.665 |
| ns | 6891 |  | 201 | Runner assertions: output-style vs failures-style fixtures | 5.11 | 5.10 | 0.655 |
| walker |  | 6928 | 42 | headings outline in docs/reference/typescript.md |  |  | 0.656 |
| walker |  | 6955 | 27 | docs/reference/typescript.md section #0 |  |  | 0.656 |
| ns | 7164 |  | 273 | A failures-style and an output-style fixture, in full | 5.12 | 5.10 | 0.639 |
| walker |  | 7237 | 282 | docs/readme.md section #1 |  |  | 0.639 |
| walker |  | 7282 | 45 | headings outline in docs/reference/coercions.md |  |  | 0.640 |
| ns | 7295 |  | 131 | The typings harness and one typings test | 5.13 | 5.6 | 0.633 |
| walker |  | 7319 | 37 | docs/guides/02-validating-data.md section #0 |  |  | 0.633 |
| walker |  | 7341 | 22 | ts names test/index.ts |  |  | 0.633 |
| walker |  | 7361 | 20 | ts module doc test/index.ts |  |  | 0.634 |
| walker |  | 7399 | 38 | docs/reference/errors.md section #0 |  |  | 0.634 |
| ns | 7481 |  | 186 | toFailure(): result normalisation and default message text | 6.1 | 2.9 | 0.625 |
| walker |  | 7510 | 111 | Readme.md section #7 |  |  | 0.625 |
| walker |  | 7541 | 31 | docs/summary.md section #2 |  |  | 0.625 |
| walker |  | 7603 | 62 | headings outline in docs/reference/core.md |  |  | 0.626 |
| walker |  | 7710 | 107 | ts doc src/error.ts:25 |  |  | 0.640 |
| walker |  | 7886 | 176 | Readme.md section #3 |  |  | 0.640 |
| ns | 7894 |  | 413 | object() implementation, including the mask special case | 6.2 | 3.6 | 0.620 |
| walker |  | 8081 | 195 | Readme.md section #4 |  |  | 0.620 |
| ns | 8085 |  | 191 | refine() and define() bodies - the extension points | 6.3 | 2.4 | 0.613 |
| ns | 8199 |  | 114 | coerce() and trimmed() bodies | 6.4 | 2.5 | 0.608 |
| walker |  | 8216 | 135 | Readme.md section #5 |  |  | 0.608 |
| walker |  | 8454 | 238 | ts names src/utils.ts |  |  | 0.617 |
| ns | 8458 |  | 259 | npm scripts | 7.1 | 1.1 | 0.623 |
| walker |  | 8474 | 20 | ts decl src/utils.ts:218 |  |  | 0.623 |
| walker |  | 8524 | 50 | ts decl src/utils.ts:106 |  |  | 0.623 |
| walker |  | 8574 | 50 | ts decl src/utils.ts:202 |  |  | 0.623 |
| walker |  | 8628 | 54 | ts decl src/utils.ts:67 |  |  | 0.623 |
| ns | 8629 |  | 171 | Changelog: recent release headings and the file's extent | 7.2 |  | 0.619 |
| ns | 8778 |  | 149 | The 2.0.0 release notes: breaking changes and fixes | 7.3 | 7.2 | 0.616 |
| walker |  | 8885 | 257 | License.md section #0 |  |  | 0.616 |
| ns | 8957 |  | 179 | TypeScript compiler configuration | 7.4 |  | 0.610 |
| walker |  | 8987 | 102 | ts decl src/utils.ts:130 |  |  | 0.617 |
| walker |  | 9050 | 63 | docs/reference/coercions.md section #0 |  |  | 0.617 |
| walker |  | 9129 | 79 | headings outline in docs/reference/utilities.md |  |  | 0.617 |
| walker |  | 9159 | 30 | docs/reference/utilities.md section #0 |  |  | 0.617 |
| walker |  | 9170 | 11 | ts body src/struct.ts:83 |  |  | 0.617 |
| ns | 9207 |  | 250 | Rollup build and JSR publish config | 7.5 |  | 0.607 |
| walker |  | 9295 | 125 | docs/summary.md section #0 |  |  | 0.607 |
| ns | 9368 |  | 161 | CI matrix and dependency automation | 7.6 |  | 0.602 |
| walker |  | 9377 | 82 | headings outline in docs/reference/refinements.md |  |  | 0.603 |
| walker |  | 9400 | 23 | ts doc src/struct.ts:175 |  |  | 0.603 |
| walker |  | 9423 | 23 | ts doc src/struct.ts:243 |  |  | 0.603 |
| walker |  | 9448 | 25 | ts doc src/struct.ts:259 |  |  | 0.603 |
| walker |  | 9472 | 24 | ts body src/struct.ts:175 |  |  | 0.603 |
| walker |  | 9485 | 13 | ts body src/struct.ts:67 |  |  | 0.603 |
| walker |  | 9498 | 13 | ts body src/struct.ts:75 |  |  | 0.603 |
| walker |  | 9511 | 13 | ts body src/struct.ts:93 |  |  | 0.603 |
| ns | 9521 |  | 153 | Remaining package.json keys: entry points and publish metadata | 7.7 | 1.1 | 0.608 |
| walker |  | 9524 | 13 | ts body src/struct.ts:107 |  |  | 0.608 |
| walker |  | 9550 | 26 | ts doc src/struct.ts:123 |  |  | 0.608 |
| walker |  | 9577 | 27 | ts doc src/struct.ts:231 |  |  | 0.608 |
| ns | 9599 |  | 78 | Formatting and docs-site configuration | 7.8 |  | 0.606 |
| walker |  | 9604 | 27 | ts doc src/struct.ts:237 |  |  | 0.606 |
| walker |  | 9632 | 28 | ts doc src/struct.ts:253 |  |  | 0.606 |
| walker |  | 9646 | 14 | ts body src/structs/types.ts:19 |  |  | 0.606 |
| walker |  | 9660 | 14 | ts body src/structs/types.ts:258 |  |  | 0.606 |
| walker |  | 9674 | 14 | ts body src/structs/types.ts:574 |  |  | 0.606 |
| walker |  | 9703 | 29 | ts doc src/struct.ts:139 |  |  | 0.606 |
| ns | 9718 |  | 119 | ESLint configuration head | 7.9 |  | 0.601 |
| walker |  | 9733 | 30 | ts doc src/struct.ts:157 |  |  | 0.601 |
| walker |  | 9752 | 19 | ts body src/structs/utilities.ts:71 |  |  | 0.602 |
| walker |  | 9791 | 39 | ts doc src/struct.ts:266 |  |  | 0.602 |
| walker |  | 9813 | 22 | ts doc src/structs/types.ts:19 |  |  | 0.602 |
| walker |  | 9836 | 23 | ts doc src/structs/types.ts:60 |  |  | 0.602 |
| ns | 9847 |  | 129 | Examples package manifest and run instructions | 7.10 |  | 0.600 |
| walker |  | 9859 | 23 | ts doc src/structs/types.ts:70 |  |  | 0.600 |
| walker |  | 9882 | 23 | ts doc src/structs/types.ts:131 |  |  | 0.601 |
| walker |  | 9905 | 23 | ts doc src/structs/types.ts:159 |  |  | 0.601 |
| walker |  | 9928 | 23 | ts doc src/structs/types.ts:258 |  |  | 0.601 |
| walker |  | 9951 | 23 | ts doc src/structs/types.ts:278 |  |  | 0.601 |
| walker |  | 9974 | 23 | ts doc src/structs/types.ts:442 |  |  | 0.602 |
