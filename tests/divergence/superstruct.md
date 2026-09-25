Score(3000)=0.545 I=0.775 C=0.384 ns_rows≤3K=20/60 grid(1000/1442/2080/3000/4327/6240/9000)=0.636/0.495/0.555/0.545/0.489/0.605/0.600

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
| walker |  | 1080 | 19 | Readme.md section #12 |  |  | 0.595 |
| walker |  | 1156 | 76 | package entrypoints in package.json |  |  | 0.596 |
| walker |  | 1272 | 116 | README headline in docs/readme.md |  |  | 0.596 |
| ns | 1324 |  | 252 | Complete roster of the 25 type structs | 2.2 |  | 0.517 |
| walker |  | 1338 | 66 | README prelude in docs/readme.md |  |  | 0.517 |
| walker |  | 1419 | 81 | export at src/error.ts:5 |  |  | 0.519 |
| ns | 1426 |  | 102 | Complete roster of src/structs/utilities.ts | 2.3 |  | 0.495 |
| ns | 1509 |  | 83 | Complete roster of src/structs/refinements.ts | 2.4 |  | 0.477 |
| walker |  | 1512 | 93 | export at src/error.ts:25 |  |  | 0.478 |
| ns | 1543 |  | 34 | Complete roster of src/structs/coercions.ts | 2.5 |  | 0.472 |
| ns | 1657 |  | 114 | Failure - the shape of every validation failure | 2.6 |  | 0.490 |
| walker |  | 1745 | 233 | Readme.md section #0 |  |  | 0.582 |
| walker |  | 1771 | 26 | export doc at src/error.ts:5 |  |  | 0.602 |
| walker |  | 1795 | 24 | listing of 'test/api' |  |  | 0.603 |
| ns | 1835 |  | 178 | StructError class: doc, fields, and its early-exit contract | 2.7 |  | 0.584 |
| ns | 2043 |  | 208 | Exported type vocabulary of src/struct.ts | 2.8 |  | 0.555 |
| walker |  | 2077 | 282 | Readme.md section #1 |  |  | 0.555 |
| walker |  | 2135 | 58 | export names surface in src/structs/coercions.ts |  |  | 0.568 |
| walker |  | 2135 | 0 | export at src/structs/coercions.ts:79 |  |  | 0.568 |
| ns | 2146 |  | 103 | Function roster of src/utils.ts | 2.9 |  | 0.552 |
| walker |  | 2154 | 19 | export body at src/structs/coercions.ts:79 body 80 |  |  | 0.552 |
| walker |  | 2201 | 47 | export at src/structs/coercions.ts:16 |  |  | 0.552 |
| walker |  | 2258 | 57 | export at src/structs/coercions.ts:38 |  |  | 0.552 |
| ns | 2384 |  | 238 | Type-level utility roster of src/utils.ts | 2.10 |  | 0.519 |
| walker |  | 2491 | 233 | export names surface in src/struct.ts |  |  | 0.554 |
| walker |  | 2491 | 0 | export at src/struct.ts:175 |  |  | 0.554 |
| walker |  | 2491 | 0 | export at src/struct.ts:231 |  |  | 0.554 |
| walker |  | 2491 | 0 | export at src/struct.ts:237 |  |  | 0.554 |
| walker |  | 2491 | 0 | export at src/struct.ts:253 |  |  | 0.554 |
| walker |  | 2491 | 0 | export at src/struct.ts:259 |  |  | 0.554 |
| walker |  | 2491 | 0 | export at src/struct.ts:266 |  |  | 0.554 |
| walker |  | 2525 | 34 | export at src/struct.ts:221 |  |  | 0.569 |
| walker |  | 2566 | 41 | export at src/struct.ts:243 |  |  | 0.590 |
| ns | 2580 |  | 196 | What each type struct does, part 1 (any ... literal) | 3.1 | 2.2 | 0.572 |
| walker |  | 2603 | 37 | export at src/struct.ts:139 |  |  | 0.572 |
| walker |  | 2640 | 37 | export at src/struct.ts:157 |  |  | 0.572 |
| walker |  | 2680 | 40 | export at src/struct.ts:123 |  |  | 0.572 |
| walker |  | 2763 | 83 | export at src/struct.ts:185 |  |  | 0.572 |
| walker |  | 2787 | 24 | export body at src/struct.ts:175 body 176 |  |  | 0.572 |
| walker |  | 2810 | 23 | export doc at src/struct.ts:243 |  |  | 0.572 |
| walker |  | 2835 | 25 | export doc at src/struct.ts:259 |  |  | 0.572 |
| walker |  | 2858 | 23 | export doc at src/struct.ts:175 |  |  | 0.572 |
| walker |  | 2885 | 27 | export doc at src/struct.ts:231 |  |  | 0.572 |
| walker |  | 2912 | 27 | export doc at src/struct.ts:237 |  |  | 0.572 |
| ns | 2922 |  | 342 | What each type struct does, part 2 (map ... unknown) | 3.2 | 2.2 | 0.545 |
| walker |  | 2940 | 28 | export doc at src/struct.ts:253 |  |  | 0.545 |
| walker |  | 2968 | 28 | export doc at src/struct.ts:123 |  |  | 0.545 |
| walker |  | 2997 | 29 | export doc at src/struct.ts:139 |  |  | 0.545 |
| walker |  | 3027 | 30 | export doc at src/struct.ts:157 |  |  | 0.545 |
| ns | 3150 |  | 228 | What each utility struct does | 3.3 | 2.3 | 0.527 |
| walker |  | 3254 | 227 | export at src/struct.ts:10 |  |  | 0.558 |
| walker |  | 3293 | 39 | export doc at src/struct.ts:266 |  |  | 0.558 |
| ns | 3306 |  | 156 | What each refinement does | 3.4 | 2.4 | 0.549 |
| walker |  | 3312 | 19 | docs/resources/links.md section #0 |  |  | 0.549 |
| ns | 3417 |  | 111 | What each coercion does - and the create() gotcha | 3.5 | 2.5 | 0.543 |
| walker |  | 3477 | 165 | export body at src/error.ts:25 body 36 |  |  | 0.544 |
| walker |  | 3509 | 32 | headings outline in docs/guides/05-handling-errors.md |  |  | 0.545 |
| walker |  | 3543 | 34 | headings outline in docs/guides/03-coercing-data.md |  |  | 0.545 |
| walker |  | 3577 | 34 | headings outline in docs/guides/04-refining-validation.md |  |  | 0.545 |
| walker |  | 3613 | 36 | headings outline in docs/guides/01-getting-started.md |  |  | 0.545 |
| walker |  | 3613 | 0 | docs/guides/01-getting-started.md section #0 |  |  | 0.545 |
| walker |  | 3659 | 46 | export doc at src/struct.ts:185 |  |  | 0.545 |
| ns | 3722 |  | 305 | Public overload declarations of the six overloaded factories | 3.6 | 2.2 | 0.520 |
| ns | 3965 |  | 243 | Struct method semantics: mask recursion and validate options | 3.7 | 2.1 | 0.506 |
| walker |  | 3976 | 317 | mdBook SUMMARY at docs/summary.md |  |  | 0.506 |
| walker |  | 4014 | 38 | imports in src/struct.ts |  |  | 0.506 |
| walker |  | 4099 | 85 | export body at src/structs/coercions.ts:16 body 21 |  |  | 0.507 |
| ns | 4155 |  | 190 | What each src/utils.ts helper does | 3.8 | 2.9 | 0.497 |
| walker |  | 4264 | 165 | listing of 'test/validation' |  |  | 0.501 |
| ns | 4304 |  | 149 | run() signature and per-call context setup | 4.1 | 2.9 | 0.489 |
| walker |  | 4365 | 101 | export names surface in src/structs/refinements.ts |  |  | 0.507 |
| walker |  | 4398 | 33 | export at src/structs/refinements.ts:93 |  |  | 0.507 |
| ns | 4433 |  | 129 | run() stage 1: coercion, then the struct's own validator | 4.2 | 4.1 | 0.496 |
| walker |  | 4442 | 44 | export at src/structs/refinements.ts:146 |  |  | 0.497 |
| walker |  | 4490 | 48 | export at src/structs/refinements.ts:8 |  |  | 0.497 |
| walker |  | 4538 | 48 | export at src/structs/refinements.ts:77 |  |  | 0.497 |
| walker |  | 4595 | 57 | export at src/structs/refinements.ts:33 |  |  | 0.497 |
| walker |  | 4652 | 57 | export at src/structs/refinements.ts:55 |  |  | 0.497 |
| walker |  | 4714 | 62 | export at src/structs/refinements.ts:109 |  |  | 0.497 |
| walker |  | 4738 | 24 | export doc at src/structs/refinements.ts:93 |  |  | 0.497 |
| walker |  | 4764 | 26 | export doc at src/structs/refinements.ts:33 |  |  | 0.498 |
| ns | 4765 |  | 332 | run() stage 2: recursive descent over entries | 4.3 | 4.1 | 0.475 |
| walker |  | 4790 | 26 | export doc at src/structs/refinements.ts:55 |  |  | 0.477 |
| ns | 4888 |  | 123 | run() stage 3: refiners and the success yield | 4.4 | 4.1 | 0.468 |
| walker |  | 4972 | 182 | listing of 'test/typings' |  |  | 0.472 |
| walker |  | 5016 | 44 | headings outline in docs/guides/06-using-typescript.md |  |  | 0.472 |
| walker |  | 5045 | 29 | export doc at src/structs/refinements.ts:8 |  |  | 0.474 |
| ns | 5060 |  | 172 | validate() body: how one failure becomes a StructError | 4.5 | 2.1 | 0.462 |
| walker |  | 5074 | 29 | export doc at src/structs/refinements.ts:77 |  |  | 0.465 |
| walker |  | 5141 | 67 | export doc at src/struct.ts:10 |  |  | 0.487 |
| ns | 5271 |  | 211 | Struct constructor: defaults and failure wrapping | 4.6 | 1.6 | 0.471 |
| ns | 5462 |  | 191 | StructError constructor: message, cause and failure caching | 4.7 | 2.7 | 0.487 |
| walker |  | 5467 | 326 | package scripts in package.json |  |  | 0.488 |
| walker |  | 5507 | 40 | headings outline in docs/reference/errors.md |  |  | 0.488 |
| walker |  | 5559 | 52 | headings outline in docs/guides/02-validating-data.md |  |  | 0.489 |
| walker |  | 5601 | 42 | headings outline in docs/reference/typescript.md |  |  | 0.489 |
| ns | 5605 |  | 143 | Complete docs/ tree listing | 5.1 |  | 0.519 |
| walker |  | 5628 | 27 | docs/reference/typescript.md section #0 |  |  | 0.519 |
| walker |  | 5656 | 28 | imports in src/structs/refinements.ts |  |  | 0.519 |
| ns | 5660 |  | 55 | Complete test/ and test/api/ listings | 5.2 |  | 0.531 |
| walker |  | 5701 | 45 | headings outline in docs/reference/coercions.md |  |  | 0.531 |
| walker |  | 5738 | 37 | docs/guides/02-validating-data.md section #0 |  |  | 0.531 |
| walker |  | 5776 | 38 | docs/reference/errors.md section #0 |  |  | 0.531 |
| ns | 5825 |  | 165 | All 41 validation-fixture kind directories | 5.3 |  | 0.564 |
| ns | 5879 |  | 54 | Complete examples/ listing | 5.4 |  | 0.572 |
| walker |  | 5887 | 111 | Readme.md section #9 |  |  | 0.572 |
| ns | 5955 |  | 76 | One kind directory listed in full, as the case-naming exemplar | 5.5 |  | 0.564 |
| ns | 6137 |  | 182 | The 45 type-level test files | 5.6 |  | 0.591 |
| walker |  | 6188 | 301 | package identity metadata in package.json |  |  | 0.605 |
| walker |  | 6212 | 24 | module-doc lede in test/index.ts |  |  | 0.605 |
| walker |  | 6309 | 97 | export doc at src/struct.ts:221 |  |  | 0.607 |
| ns | 6364 |  | 227 | Every heading in the six guides | 5.7 |  | 0.618 |
| walker |  | 6416 | 107 | export doc at src/error.ts:25 |  |  | 0.632 |
| ns | 6427 |  | 63 | Readme section map | 5.8 |  | 0.635 |
| walker |  | 6461 | 45 | Readme.md section #6 |  |  | 0.635 |
| walker |  | 6505 | 44 | Readme.md section #7 |  |  | 0.635 |
| walker |  | 6553 | 48 | export doc at src/structs/refinements.ts:109 |  |  | 0.638 |
| ns | 6567 |  | 140 | Doc anchors the source rosters do not cover | 5.9 |  | 0.635 |
| walker |  | 6615 | 62 | headings outline in docs/reference/core.md |  |  | 0.636 |
| walker |  | 6615 | 0 | docs/reference/core.md section #0 |  |  | 0.636 |
| ns | 6738 |  | 171 | The data-driven fixture runner | 5.10 |  | 0.630 |
| walker |  | 6800 | 185 | json config tsconfig.json |  |  | 0.631 |
| walker |  | 6850 | 50 | Readme.md section #5 |  |  | 0.631 |
| walker |  | 6868 | 18 | export names surface in test/index.ts |  |  | 0.631 |
| walker |  | 6921 | 53 | Readme.md section #4 |  |  | 0.631 |
| ns | 6939 |  | 201 | Runner assertions: output-style vs failures-style fixtures | 5.11 | 5.10 | 0.621 |
| walker |  | 6969 | 48 | imports in src/structs/coercions.ts |  |  | 0.621 |
| walker |  | 7032 | 63 | docs/reference/coercions.md section #0 |  |  | 0.621 |
| walker |  | 7111 | 79 | headings outline in docs/reference/utilities.md |  |  | 0.621 |
| walker |  | 7141 | 30 | docs/reference/utilities.md section #0 |  |  | 0.621 |
| ns | 7212 |  | 273 | A failures-style and an output-style fixture, in full | 5.12 | 5.10 | 0.604 |
| walker |  | 7223 | 82 | headings outline in docs/reference/refinements.md |  |  | 0.606 |
| walker |  | 7276 | 53 | docs/reference/refinements.md section #0 |  |  | 0.606 |
| ns | 7343 |  | 131 | The typings harness and one typings test | 5.13 | 5.6 | 0.600 |
| walker |  | 7514 | 238 | export names surface in src/structs/utilities.ts |  |  | 0.614 |
| walker |  | 7514 | 0 | export at src/structs/utilities.ts:60 |  |  | 0.614 |
| walker |  | 7514 | 0 | export at src/structs/utilities.ts:71 |  |  | 0.614 |
| walker |  | 7514 | 0 | export at src/structs/utilities.ts:140 |  |  | 0.614 |
| walker |  | 7514 | 0 | export at src/structs/utilities.ts:247 |  |  | 0.614 |
| ns | 7529 |  | 186 | toFailure(): result normalisation and default message text | 6.1 | 2.9 | 0.605 |
| walker |  | 7533 | 19 | export body at src/structs/utilities.ts:71 body 72 |  |  | 0.605 |
| walker |  | 7566 | 33 | export at src/structs/utilities.ts:106 |  |  | 0.605 |
| walker |  | 7603 | 37 | export at src/structs/utilities.ts:197 |  |  | 0.605 |
| walker |  | 7641 | 38 | export at src/structs/utilities.ts:80 |  |  | 0.605 |
| walker |  | 7684 | 43 | export at src/structs/utilities.ts:221 |  |  | 0.605 |
| walker |  | 7728 | 44 | export at src/structs/utilities.ts:171 |  |  | 0.605 |
| walker |  | 7773 | 45 | export at src/structs/utilities.ts:17 |  |  | 0.605 |
| walker |  | 7875 | 102 | export at src/structs/utilities.ts:21 |  |  | 0.605 |
| walker |  | 7901 | 26 | export doc at src/structs/utilities.ts:71 |  |  | 0.605 |
| ns | 7942 |  | 413 | object() implementation, including the mask special case | 6.2 | 3.6 | 0.587 |
| walker |  | 8051 | 150 | export at src/structs/utilities.ts:30 |  |  | 0.587 |
| walker |  | 8080 | 29 | export doc at src/structs/utilities.ts:247 |  |  | 0.587 |
| ns | 8133 |  | 191 | refine() and define() bodies - the extension points | 6.3 | 2.4 | 0.581 |
| ns | 8247 |  | 114 | coerce() and trimmed() bodies | 6.4 | 2.5 | 0.584 |
| walker |  | 8262 | 182 | export at src/structs/utilities.ts:44 |  |  | 0.584 |
| walker |  | 8317 | 55 | export doc at src/structs/utilities.ts:80 |  |  | 0.585 |
| walker |  | 8374 | 57 | export doc at src/structs/utilities.ts:171 |  |  | 0.587 |
| walker |  | 8431 | 57 | export doc at src/structs/utilities.ts:221 |  |  | 0.589 |
| walker |  | 8494 | 63 | export doc at src/structs/utilities.ts:197 |  |  | 0.592 |
| ns | 8506 |  | 259 | npm scripts | 7.1 | 1.1 | 0.598 |
| walker |  | 8562 | 68 | export doc at src/structs/utilities.ts:17 |  |  | 0.602 |
| walker |  | 8639 | 77 | export doc at src/structs/coercions.ts:79 |  |  | 0.603 |
| ns | 8677 |  | 171 | Changelog: recent release headings and the file's extent | 7.2 |  | 0.599 |
| walker |  | 8720 | 81 | export doc at src/structs/utilities.ts:106 |  |  | 0.600 |
| walker |  | 8802 | 82 | export doc at src/structs/coercions.ts:38 |  |  | 0.600 |
| ns | 8826 |  | 149 | The 2.0.0 release notes: breaking changes and fixes | 7.3 | 7.2 | 0.598 |
| walker |  | 8893 | 91 | export doc at src/structs/refinements.ts:146 |  |  | 0.600 |
| walker |  | 9003 | 110 | json config jsr.json |  |  | 0.600 |
| ns | 9005 |  | 179 | TypeScript compiler configuration | 7.4 |  | 0.606 |
| walker |  | 9209 | 206 | Readme.md section #10 |  |  | 0.606 |
| ns | 9255 |  | 250 | Rollup build and JSR publish config | 7.5 |  | 0.599 |
| walker |  | 9289 | 80 | imports in src/structs/utilities.ts |  |  | 0.599 |
| walker |  | 9315 | 26 | package identity in examples/package.json |  |  | 0.599 |
| ns | 9416 |  | 161 | CI matrix and dependency automation | 7.6 |  | 0.604 |
| walker |  | 9418 | 103 | export doc at src/structs/utilities.ts:140 |  |  | 0.606 |
| walker |  | 9526 | 108 | docs/guides/03-coercing-data.md section #0 |  |  | 0.606 |
| ns | 9569 |  | 153 | Remaining package.json keys: entry points and publish metadata | 7.7 | 1.1 | 0.612 |
| walker |  | 9637 | 111 | docs/guides/04-refining-validation.md section #0 |  |  | 0.612 |
| ns | 9647 |  | 78 | Formatting and docs-site configuration | 7.8 |  | 0.609 |
| ns | 9766 |  | 119 | ESLint configuration head | 7.9 |  | 0.605 |
| ns | 9895 |  | 129 | Examples package manifest and run instructions | 7.10 |  | 0.602 |
