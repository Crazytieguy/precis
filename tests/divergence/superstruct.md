Score(3000)=0.459 I=0.763 C=0.276 ns_rows≤3K=20/60 (reached=6 partial=1 missing=13)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 69 |  | 69 | Package identity: name, description, version, license | 1.1 |  | 0.000 |
| walker |  | 71 | 71 | listing of '.' |  |  | 0.000 |
| walker |  | 90 | 19 | listing of 'src' |  |  | 0.000 |
| walker |  | 107 | 17 | listing of 'src/structs' |  |  | 0.000 |
| walker |  | 110 | 3 | listing of '.vscode' |  |  | 0.000 |
| walker |  | 134 | 24 | listing of 'docs' |  |  | 0.000 |
| ns | 140 |  | 71 | Complete root directory listing | 1.2 |  | 0.616 |
| walker |  | 141 | 7 | listing of 'docs/resources' |  |  | 0.616 |
| ns | 177 |  | 37 | Complete src/ and src/structs/ listings | 1.3 |  | 0.613 |
| walker |  | 195 | 54 | listing of 'examples' |  |  | 0.617 |
| walker |  | 203 | 8 | listing of '.github' |  |  | 0.617 |
| walker |  | 206 | 3 | listing of '.github/workflows' |  |  | 0.617 |
| ns | 256 |  | 79 | src/index.ts - the entire public barrel | 1.4 |  | 0.556 |
| walker |  | 381 | 175 | YAML config at .github/workflows/ci.yml |  |  | 0.559 |
| ns | 422 |  | 166 | Readme lede: what Superstruct is and why it exists | 1.5 |  | 0.542 |
| walker |  | 463 | 82 | package identity in package.json |  |  | 0.843 |
| walker |  | 491 | 28 | listing of 'docs/images' |  |  | 0.844 |
| walker |  | 521 | 30 | listing of 'docs/reference' |  |  | 0.846 |
| walker |  | 600 | 79 | imports in src/index.ts |  |  | 0.941 |
| ns | 644 |  | 222 | The Struct class: doc comment and its six fields | 1.6 |  | 0.778 |
| walker |  | 650 | 50 | listing of 'docs/guides' |  |  | 0.780 |
| walker |  | 684 | 34 | package runtime metadata in package.json |  |  | 0.780 |
| walker |  | 711 | 27 | export names surface in src/error.ts |  |  | 0.780 |
| ns | 822 |  | 178 | Core API signatures: assert / create / is / mask / validate | 1.7 |  | 0.706 |
| walker |  | 893 | 182 | README headline in Readme.md |  |  | 0.711 |
| walker |  | 959 | 66 | README prelude in Readme.md |  |  | 0.725 |
| ns | 977 |  | 155 | Readme canonical usage snippet | 1.8 |  | 0.636 |
| walker |  | 1013 | 54 | headings outline in Readme.md |  |  | 0.637 |
| walker |  | 1031 | 18 | headings outline in docs/resources/links.md |  |  | 0.637 |
| walker |  | 1061 | 30 | listing of 'test' |  |  | 0.638 |
| ns | 1072 |  | 95 | src/struct.ts symbol roster: methods and top-level helpers | 2.1 |  | 0.595 |
| walker |  | 1080 | 19 | Readme.md section #11 |  |  | 0.595 |
| walker |  | 1156 | 76 | package entrypoints in package.json |  |  | 0.596 |
| walker |  | 1272 | 116 | README headline in docs/readme.md |  |  | 0.596 |
| ns | 1324 |  | 252 | Complete roster of the 25 type structs | 2.2 |  | 0.517 |
| walker |  | 1338 | 66 | README prelude in docs/readme.md |  |  | 0.517 |
| walker |  | 1419 | 81 | export at src/error.ts:5 |  |  | 0.519 |
| ns | 1426 |  | 102 | Complete roster of src/structs/utilities.ts | 2.3 |  | 0.495 |
| ns | 1509 |  | 83 | Complete roster of src/structs/refinements.ts | 2.4 |  | 0.477 |
| walker |  | 1512 | 93 | export at src/error.ts:25 |  |  | 0.478 |
| ns | 1543 |  | 34 | Complete roster of src/structs/coercions.ts | 2.5 |  | 0.472 |
| walker |  | 1544 | 32 | headings outline in docs/guides/05-handling-errors.md |  |  | 0.472 |
| walker |  | 1578 | 34 | headings outline in docs/guides/03-coercing-data.md |  |  | 0.472 |
| walker |  | 1612 | 34 | headings outline in docs/guides/04-refining-validation.md |  |  | 0.473 |
| walker |  | 1648 | 36 | headings outline in docs/guides/01-getting-started.md |  |  | 0.473 |
| walker |  | 1648 | 0 | docs/guides/01-getting-started.md section #0 |  |  | 0.473 |
| ns | 1657 |  | 114 | Failure - the shape of every validation failure | 2.6 |  | 0.490 |
| walker |  | 1672 | 24 | listing of 'test/api' |  |  | 0.491 |
| walker |  | 1716 | 44 | headings outline in docs/guides/06-using-typescript.md |  |  | 0.492 |
| walker |  | 1756 | 40 | headings outline in docs/reference/errors.md |  |  | 0.492 |
| walker |  | 1782 | 26 | export doc at src/error.ts:5 |  |  | 0.514 |
| walker |  | 1834 | 52 | headings outline in docs/guides/02-validating-data.md |  |  | 0.515 |
| ns | 1835 |  | 178 | StructError class: doc, fields, and its early-exit contract | 2.7 |  | 0.503 |
| walker |  | 1876 | 42 | headings outline in docs/reference/typescript.md |  |  | 0.503 |
| walker |  | 1921 | 45 | headings outline in docs/reference/coercions.md |  |  | 0.503 |
| walker |  | 1979 | 58 | export names surface in src/structs/coercions.ts |  |  | 0.518 |
| walker |  | 1979 | 0 | export at src/structs/coercions.ts:79 |  |  | 0.518 |
| walker |  | 1998 | 19 | export body at src/structs/coercions.ts:79 body 80 |  |  | 0.518 |
| ns | 2043 |  | 208 | Exported type vocabulary of src/struct.ts | 2.8 |  | 0.492 |
| walker |  | 2045 | 47 | export at src/structs/coercions.ts:16 |  |  | 0.492 |
| walker |  | 2102 | 57 | export at src/structs/coercions.ts:38 |  |  | 0.492 |
| ns | 2146 |  | 103 | Function roster of src/utils.ts | 2.9 |  | 0.478 |
| ns | 2384 |  | 238 | Type-level utility roster of src/utils.ts | 2.10 |  | 0.449 |
| walker |  | 2419 | 317 | mdBook SUMMARY at docs/summary.md |  |  | 0.449 |
| ns | 2580 |  | 196 | What each type struct does, part 1 (any ... literal) | 3.1 | 2.2 | 0.436 |
| walker |  | 2584 | 165 | listing of 'test/validation' |  |  | 0.440 |
| walker |  | 2766 | 182 | listing of 'test/typings' |  |  | 0.444 |
| ns | 2922 |  | 342 | What each type struct does, part 2 (map ... unknown) | 3.2 | 2.2 | 0.423 |
| walker |  | 2999 | 233 | export names surface in src/struct.ts |  |  | 0.459 |
| walker |  | 2999 | 0 | export at src/struct.ts:175 |  |  | 0.459 |
| walker |  | 2999 | 0 | export at src/struct.ts:231 |  |  | 0.459 |
| walker |  | 2999 | 0 | export at src/struct.ts:237 |  |  | 0.459 |
| walker |  | 2999 | 0 | export at src/struct.ts:253 |  |  | 0.459 |
| walker |  | 2999 | 0 | export at src/struct.ts:259 |  |  | 0.459 |
| walker |  | 2999 | 0 | export at src/struct.ts:266 |  |  | 0.459 |
| walker |  | 3033 | 34 | export at src/struct.ts:221 |  |  | 0.474 |
| walker |  | 3074 | 41 | export at src/struct.ts:243 |  |  | 0.495 |
| walker |  | 3111 | 37 | export at src/struct.ts:139 |  |  | 0.495 |
| walker |  | 3148 | 37 | export at src/struct.ts:157 |  |  | 0.495 |
| ns | 3150 |  | 228 | What each utility struct does | 3.3 | 2.3 | 0.478 |
| walker |  | 3188 | 40 | export at src/struct.ts:123 |  |  | 0.478 |
| walker |  | 3271 | 83 | export at src/struct.ts:185 |  |  | 0.478 |
| walker |  | 3295 | 24 | export body at src/struct.ts:175 body 176 |  |  | 0.478 |
| ns | 3306 |  | 156 | What each refinement does | 3.4 | 2.4 | 0.470 |
| walker |  | 3318 | 23 | export doc at src/struct.ts:243 |  |  | 0.470 |
| walker |  | 3343 | 25 | export doc at src/struct.ts:259 |  |  | 0.470 |
| walker |  | 3366 | 23 | export doc at src/struct.ts:175 |  |  | 0.470 |
| walker |  | 3393 | 27 | export doc at src/struct.ts:231 |  |  | 0.470 |
| ns | 3417 |  | 111 | What each coercion does - and the create() gotcha | 3.5 | 2.5 | 0.465 |
| walker |  | 3420 | 27 | export doc at src/struct.ts:237 |  |  | 0.465 |
| walker |  | 3448 | 28 | export doc at src/struct.ts:253 |  |  | 0.465 |
| walker |  | 3476 | 28 | export doc at src/struct.ts:123 |  |  | 0.465 |
| walker |  | 3505 | 29 | export doc at src/struct.ts:139 |  |  | 0.465 |
| walker |  | 3535 | 30 | export doc at src/struct.ts:157 |  |  | 0.465 |
| ns | 3722 |  | 305 | Public overload declarations of the six overloaded factories | 3.6 | 2.2 | 0.443 |
| walker |  | 3762 | 227 | export at src/struct.ts:10 |  |  | 0.473 |
| walker |  | 3824 | 62 | headings outline in docs/reference/core.md |  |  | 0.476 |
| walker |  | 3824 | 0 | docs/reference/core.md section #0 |  |  | 0.476 |
| walker |  | 3863 | 39 | export doc at src/struct.ts:266 |  |  | 0.476 |
| walker |  | 3882 | 19 | docs/resources/links.md section #0 |  |  | 0.476 |
| ns | 3965 |  | 243 | Struct method semantics: mask recursion and validate options | 3.7 | 2.1 | 0.464 |
| walker |  | 4047 | 165 | export body at src/error.ts:25 body 36 |  |  | 0.465 |
| walker |  | 4126 | 79 | headings outline in docs/reference/utilities.md |  |  | 0.465 |
| ns | 4155 |  | 190 | What each src/utils.ts helper does | 3.8 | 2.9 | 0.456 |
| walker |  | 4172 | 46 | export doc at src/struct.ts:185 |  |  | 0.456 |
| walker |  | 4254 | 82 | headings outline in docs/reference/refinements.md |  |  | 0.456 |
| walker |  | 4292 | 38 | imports in src/struct.ts |  |  | 0.456 |
| ns | 4304 |  | 149 | run() signature and per-call context setup | 4.1 | 2.9 | 0.445 |
| walker |  | 4377 | 85 | export body at src/structs/coercions.ts:16 body 21 |  |  | 0.446 |
| ns | 4433 |  | 129 | run() stage 1: coercion, then the struct's own validator | 4.2 | 4.1 | 0.437 |
| ns | 4765 |  | 332 | run() stage 2: recursive descent over entries | 4.3 | 4.1 | 0.417 |
| ns | 4888 |  | 123 | run() stage 3: refiners and the success yield | 4.4 | 4.1 | 0.409 |
| walker |  | 4897 | 520 | Readme.md section #0 |  |  | 0.456 |
| walker |  | 4924 | 27 | docs/reference/typescript.md section #0 |  |  | 0.456 |
| walker |  | 5025 | 101 | export names surface in src/structs/refinements.ts |  |  | 0.472 |
| walker |  | 5058 | 33 | export at src/structs/refinements.ts:93 |  |  | 0.472 |
| ns | 5060 |  | 172 | validate() body: how one failure becomes a StructError | 4.5 | 2.1 | 0.461 |
| walker |  | 5102 | 44 | export at src/structs/refinements.ts:146 |  |  | 0.461 |
| walker |  | 5150 | 48 | export at src/structs/refinements.ts:8 |  |  | 0.461 |
| walker |  | 5198 | 48 | export at src/structs/refinements.ts:77 |  |  | 0.461 |
| walker |  | 5255 | 57 | export at src/structs/refinements.ts:33 |  |  | 0.461 |
| ns | 5271 |  | 211 | Struct constructor: defaults and failure wrapping | 4.6 | 1.6 | 0.446 |
| walker |  | 5312 | 57 | export at src/structs/refinements.ts:55 |  |  | 0.446 |
| walker |  | 5374 | 62 | export at src/structs/refinements.ts:109 |  |  | 0.446 |
| walker |  | 5398 | 24 | export doc at src/structs/refinements.ts:93 |  |  | 0.446 |
| walker |  | 5424 | 26 | export doc at src/structs/refinements.ts:33 |  |  | 0.447 |
| walker |  | 5450 | 26 | export doc at src/structs/refinements.ts:55 |  |  | 0.448 |
| ns | 5462 |  | 191 | StructError constructor: message, cause and failure caching | 4.7 | 2.7 | 0.466 |
| walker |  | 5479 | 29 | export doc at src/structs/refinements.ts:8 |  |  | 0.467 |
| walker |  | 5508 | 29 | export doc at src/structs/refinements.ts:77 |  |  | 0.470 |
| walker |  | 5575 | 67 | export doc at src/struct.ts:10 |  |  | 0.490 |
| ns | 5605 |  | 143 | Complete docs/ tree listing | 5.1 |  | 0.520 |
| ns | 5660 |  | 55 | Complete test/ and test/api/ listings | 5.2 |  | 0.532 |
| ns | 5825 |  | 165 | All 41 validation-fixture kind directories | 5.3 |  | 0.565 |
| ns | 5879 |  | 54 | Complete examples/ listing | 5.4 |  | 0.573 |
| walker |  | 5901 | 326 | package scripts in package.json |  |  | 0.574 |
| walker |  | 5931 | 30 | docs/reference/utilities.md section #0 |  |  | 0.574 |
| ns | 5955 |  | 76 | One kind directory listed in full, as the case-naming exemplar | 5.5 |  | 0.566 |
| walker |  | 5959 | 28 | imports in src/structs/refinements.ts |  |  | 0.566 |
| walker |  | 5996 | 37 | docs/guides/02-validating-data.md section #0 |  |  | 0.566 |
| walker |  | 6034 | 38 | docs/reference/errors.md section #0 |  |  | 0.566 |
| ns | 6137 |  | 182 | The 45 type-level test files | 5.6 |  | 0.593 |
| walker |  | 6145 | 111 | Readme.md section #8 |  |  | 0.593 |
| ns | 6364 |  | 227 | Every heading in the six guides | 5.7 |  | 0.603 |
| ns | 6427 |  | 63 | Readme section map | 5.8 |  | 0.607 |
| walker |  | 6446 | 301 | package identity metadata in package.json |  |  | 0.620 |
| walker |  | 6470 | 24 | module-doc lede in test/index.ts |  |  | 0.620 |
| walker |  | 6567 | 97 | export doc at src/struct.ts:221 |  |  | 0.621 |
| ns | 6567 |  | 140 | Doc anchors the source rosters do not cover | 5.9 |  | 0.621 |
| walker |  | 6674 | 107 | export doc at src/error.ts:25 |  |  | 0.635 |
| walker |  | 6719 | 45 | Readme.md section #5 |  |  | 0.635 |
| ns | 6738 |  | 171 | The data-driven fixture runner | 5.10 |  | 0.628 |
| walker |  | 6763 | 44 | Readme.md section #6 |  |  | 0.628 |
| walker |  | 6811 | 48 | export doc at src/structs/refinements.ts:109 |  |  | 0.631 |
| ns | 6939 |  | 201 | Runner assertions: output-style vs failures-style fixtures | 5.11 | 5.10 | 0.621 |
| walker |  | 6996 | 185 | json config tsconfig.json |  |  | 0.622 |
| walker |  | 7049 | 53 | docs/reference/refinements.md section #0 |  |  | 0.622 |
| walker |  | 7099 | 50 | Readme.md section #4 |  |  | 0.622 |
| walker |  | 7117 | 18 | export names surface in test/index.ts |  |  | 0.622 |
| walker |  | 7170 | 53 | Readme.md section #3 |  |  | 0.622 |
| ns | 7212 |  | 273 | A failures-style and an output-style fixture, in full | 5.12 | 5.10 | 0.606 |
| walker |  | 7218 | 48 | imports in src/structs/coercions.ts |  |  | 0.606 |
| walker |  | 7281 | 63 | docs/reference/coercions.md section #0 |  |  | 0.606 |
| ns | 7343 |  | 131 | The typings harness and one typings test | 5.13 | 5.6 | 0.600 |
| walker |  | 7519 | 238 | export names surface in src/structs/utilities.ts |  |  | 0.614 |
| walker |  | 7519 | 0 | export at src/structs/utilities.ts:60 |  |  | 0.614 |
| walker |  | 7519 | 0 | export at src/structs/utilities.ts:71 |  |  | 0.614 |
| walker |  | 7519 | 0 | export at src/structs/utilities.ts:140 |  |  | 0.614 |
| walker |  | 7519 | 0 | export at src/structs/utilities.ts:247 |  |  | 0.614 |
| ns | 7529 |  | 186 | toFailure(): result normalisation and default message text | 6.1 | 2.9 | 0.605 |
| walker |  | 7538 | 19 | export body at src/structs/utilities.ts:71 body 72 |  |  | 0.605 |
| walker |  | 7571 | 33 | export at src/structs/utilities.ts:106 |  |  | 0.605 |
| walker |  | 7608 | 37 | export at src/structs/utilities.ts:197 |  |  | 0.605 |
| walker |  | 7646 | 38 | export at src/structs/utilities.ts:80 |  |  | 0.605 |
| walker |  | 7689 | 43 | export at src/structs/utilities.ts:221 |  |  | 0.605 |
| walker |  | 7733 | 44 | export at src/structs/utilities.ts:171 |  |  | 0.605 |
| walker |  | 7778 | 45 | export at src/structs/utilities.ts:17 |  |  | 0.605 |
| walker |  | 7880 | 102 | export at src/structs/utilities.ts:21 |  |  | 0.605 |
| walker |  | 7906 | 26 | export doc at src/structs/utilities.ts:71 |  |  | 0.605 |
| ns | 7942 |  | 413 | object() implementation, including the mask special case | 6.2 | 3.6 | 0.587 |
| walker |  | 8056 | 150 | export at src/structs/utilities.ts:30 |  |  | 0.587 |
| walker |  | 8085 | 29 | export doc at src/structs/utilities.ts:247 |  |  | 0.587 |
| ns | 8133 |  | 191 | refine() and define() bodies - the extension points | 6.3 | 2.4 | 0.581 |
| ns | 8247 |  | 114 | coerce() and trimmed() bodies | 6.4 | 2.5 | 0.584 |
| walker |  | 8267 | 182 | export at src/structs/utilities.ts:44 |  |  | 0.584 |
| walker |  | 8322 | 55 | export doc at src/structs/utilities.ts:80 |  |  | 0.585 |
| walker |  | 8379 | 57 | export doc at src/structs/utilities.ts:171 |  |  | 0.587 |
| walker |  | 8436 | 57 | export doc at src/structs/utilities.ts:221 |  |  | 0.589 |
| walker |  | 8499 | 63 | export doc at src/structs/utilities.ts:197 |  |  | 0.592 |
| ns | 8506 |  | 259 | npm scripts | 7.1 | 1.1 | 0.598 |
| walker |  | 8567 | 68 | export doc at src/structs/utilities.ts:17 |  |  | 0.602 |
| walker |  | 8644 | 77 | export doc at src/structs/coercions.ts:79 |  |  | 0.603 |
| ns | 8677 |  | 171 | Changelog: recent release headings and the file's extent | 7.2 |  | 0.599 |
| ns | 8826 |  | 149 | The 2.0.0 release notes: breaking changes and fixes | 7.3 | 7.2 | 0.597 |
| walker |  | 8915 | 271 | headings outline in docs/reference/types.md |  |  | 0.598 |
| walker |  | 8957 | 42 | docs/reference/types.md section #0 |  |  | 0.598 |
| ns | 9005 |  | 179 | TypeScript compiler configuration | 7.4 |  | 0.604 |
| walker |  | 9038 | 81 | export doc at src/structs/utilities.ts:106 |  |  | 0.604 |
| walker |  | 9120 | 82 | export doc at src/structs/coercions.ts:38 |  |  | 0.605 |
| walker |  | 9211 | 91 | export doc at src/structs/refinements.ts:146 |  |  | 0.607 |
| ns | 9255 |  | 250 | Rollup build and JSR publish config | 7.5 |  | 0.598 |
| walker |  | 9321 | 110 | json config jsr.json |  |  | 0.600 |
| ns | 9416 |  | 161 | CI matrix and dependency automation | 7.6 |  | 0.605 |
| walker |  | 9527 | 206 | Readme.md section #9 |  |  | 0.605 |
| ns | 9569 |  | 153 | Remaining package.json keys: entry points and publish metadata | 7.7 | 1.1 | 0.610 |
| walker |  | 9607 | 80 | imports in src/structs/utilities.ts |  |  | 0.610 |
| walker |  | 9633 | 26 | package identity in examples/package.json |  |  | 0.610 |
| ns | 9647 |  | 78 | Formatting and docs-site configuration | 7.8 |  | 0.608 |
| walker |  | 9736 | 103 | export doc at src/structs/utilities.ts:140 |  |  | 0.610 |
| ns | 9766 |  | 119 | ESLint configuration head | 7.9 |  | 0.605 |
| walker |  | 9844 | 108 | docs/guides/03-coercing-data.md section #0 |  |  | 0.605 |
| ns | 9895 |  | 129 | Examples package manifest and run instructions | 7.10 |  | 0.603 |
| walker |  | 9955 | 111 | docs/guides/04-refining-validation.md section #0 |  |  | 0.603 |
