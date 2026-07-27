Score(3000)=0.638 I=0.893 C=0.455 ns_rows≤3K=14/40 (reached=6 partial=1 missing=7)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 71 | 71 | listing of '.' |  |  | 1.000 |
| ns | 71 |  | 71 | Repo root listing | 1.1 |  | 1.000 |
| walker |  | 90 | 19 | listing of 'src' |  |  | 1.000 |
| walker |  | 107 | 17 | listing of 'src/structs' |  |  | 1.000 |
| ns | 108 |  | 37 | src/ and src/structs/ listings | 1.2 |  | 1.000 |
| walker |  | 110 | 3 | listing of '.vscode' |  |  | 1.000 |
| walker |  | 134 | 24 | listing of 'docs' |  |  | 1.000 |
| ns | 138 |  | 30 | test/ top-level listing | 1.3 |  | 0.873 |
| walker |  | 141 | 7 | listing of 'docs/resources' |  |  | 0.875 |
| ns | 192 |  | 54 | examples/ listing | 1.4 |  | 0.740 |
| walker |  | 223 | 82 | package identity in package.json |  |  | 0.743 |
| walker |  | 277 | 54 | listing of 'examples' |  |  | 0.906 |
| walker |  | 285 | 8 | listing of '.github' |  |  | 0.906 |
| walker |  | 288 | 3 | listing of '.github/workflows' |  |  | 0.906 |
| ns | 335 |  | 143 | docs/ listing (all levels) | 1.5 |  | 0.705 |
| ns | 430 |  | 95 | package.json identity (name/desc/version) | 1.6 |  | 0.695 |
| walker |  | 463 | 175 | YAML config at .github/workflows/ci.yml |  |  | 0.697 |
| walker |  | 542 | 79 | imports in src/index.ts |  |  | 0.697 |
| walker |  | 570 | 28 | listing of 'docs/images' |  |  | 0.741 |
| ns | 582 |  | 152 | package.json entry points, files, engines | 1.7 | 1.6 | 0.672 |
| walker |  | 600 | 30 | listing of 'docs/reference' |  |  | 0.744 |
| walker |  | 634 | 34 | package runtime metadata in package.json |  |  | 0.750 |
| walker |  | 661 | 27 | export names surface in src/error.ts |  |  | 0.750 |
| walker |  | 711 | 50 | listing of 'docs/guides' |  |  | 0.832 |
| ns | 753 |  | 171 | Readme concept lede | 1.8 |  | 0.816 |
| walker |  | 893 | 182 | README headline in Readme.md |  |  | 0.819 |
| walker |  | 947 | 54 | headings outline in Readme.md |  |  | 0.819 |
| walker |  | 965 | 18 | headings outline in docs/resources/links.md |  |  | 0.819 |
| walker |  | 984 | 19 | Readme.md section #11 |  |  | 0.819 |
| walker |  | 1060 | 76 | package entrypoints in package.json |  |  | 0.868 |
| ns | 1070 |  | 317 | docs/summary.md (site nav) | 1.9 |  | 0.756 |
| walker |  | 1176 | 116 | README headline in docs/readme.md |  |  | 0.756 |
| walker |  | 1257 | 81 | export at src/error.ts:5 |  |  | 0.757 |
| walker |  | 1350 | 93 | export at src/error.ts:25 |  |  | 0.758 |
| walker |  | 1380 | 30 | listing of 'test' |  |  | 0.809 |
| ns | 1401 |  | 331 | Readme design principles | 1.10 |  | 0.768 |
| walker |  | 1412 | 32 | headings outline in docs/guides/05-handling-errors.md |  |  | 0.768 |
| walker |  | 1446 | 34 | headings outline in docs/guides/03-coercing-data.md |  |  | 0.768 |
| walker |  | 1480 | 34 | headings outline in docs/guides/04-refining-validation.md |  |  | 0.768 |
| walker |  | 1516 | 36 | headings outline in docs/guides/01-getting-started.md |  |  | 0.768 |
| walker |  | 1516 | 0 | docs/guides/01-getting-started.md section #0 |  |  | 0.768 |
| ns | 1541 |  | 140 | struct.ts symbol locations | 2.1 |  | 0.730 |
| walker |  | 1560 | 44 | headings outline in docs/guides/06-using-typescript.md |  |  | 0.730 |
| walker |  | 1600 | 40 | headings outline in docs/reference/errors.md |  |  | 0.730 |
| walker |  | 1626 | 26 | export doc at src/error.ts:5 |  |  | 0.730 |
| walker |  | 1678 | 52 | headings outline in docs/guides/02-validating-data.md |  |  | 0.730 |
| ns | 1685 |  | 144 | Struct class fields | 2.2 | 2.1 | 0.697 |
| walker |  | 1720 | 42 | headings outline in docs/reference/typescript.md |  |  | 0.697 |
| walker |  | 1765 | 45 | headings outline in docs/reference/coercions.md |  |  | 0.697 |
| walker |  | 1789 | 24 | listing of 'test/api' |  |  | 0.697 |
| walker |  | 1847 | 58 | export names surface in src/structs/coercions.ts |  |  | 0.697 |
| walker |  | 1847 | 0 | export at src/structs/coercions.ts:79 |  |  | 0.697 |
| walker |  | 1866 | 19 | export body at src/structs/coercions.ts:79 body 80 |  |  | 0.697 |
| walker |  | 1913 | 47 | export at src/structs/coercions.ts:16 |  |  | 0.697 |
| walker |  | 1970 | 57 | export at src/structs/coercions.ts:38 |  |  | 0.697 |
| ns | 2068 |  | 383 | Struct constructor body | 2.3 | 2.1 | 0.610 |
| walker |  | 2287 | 317 | mdBook SUMMARY at docs/summary.md |  |  | 0.710 |
| walker |  | 2520 | 233 | export names surface in src/struct.ts |  |  | 0.729 |
| walker |  | 2520 | 0 | export at src/struct.ts:175 |  |  | 0.729 |
| walker |  | 2520 | 0 | export at src/struct.ts:231 |  |  | 0.729 |
| walker |  | 2520 | 0 | export at src/struct.ts:237 |  |  | 0.729 |
| walker |  | 2520 | 0 | export at src/struct.ts:253 |  |  | 0.729 |
| walker |  | 2520 | 0 | export at src/struct.ts:259 |  |  | 0.729 |
| walker |  | 2520 | 0 | export at src/struct.ts:266 |  |  | 0.729 |
| walker |  | 2554 | 34 | export at src/struct.ts:221 |  |  | 0.729 |
| walker |  | 2595 | 41 | export at src/struct.ts:243 |  |  | 0.730 |
| ns | 2608 |  | 540 | Struct instance methods (assert/create/is/mask/validate) | 2.4 | 2.1 | 0.632 |
| walker |  | 2632 | 37 | export at src/struct.ts:139 |  |  | 0.632 |
| walker |  | 2669 | 37 | export at src/struct.ts:157 |  |  | 0.632 |
| walker |  | 2709 | 40 | export at src/struct.ts:123 |  |  | 0.632 |
| walker |  | 2792 | 83 | export at src/struct.ts:185 |  |  | 0.633 |
| walker |  | 2816 | 24 | export body at src/struct.ts:175 body 176 |  |  | 0.634 |
| walker |  | 2839 | 23 | export doc at src/struct.ts:243 |  |  | 0.634 |
| walker |  | 2864 | 25 | export doc at src/struct.ts:259 |  |  | 0.635 |
| walker |  | 2887 | 23 | export doc at src/struct.ts:175 |  |  | 0.635 |
| walker |  | 2914 | 27 | export doc at src/struct.ts:231 |  |  | 0.636 |
| walker |  | 2941 | 27 | export doc at src/struct.ts:237 |  |  | 0.636 |
| walker |  | 2969 | 28 | export doc at src/struct.ts:253 |  |  | 0.637 |
| walker |  | 2997 | 28 | export doc at src/struct.ts:123 |  |  | 0.638 |
| walker |  | 3026 | 29 | export doc at src/struct.ts:139 |  |  | 0.638 |
| walker |  | 3056 | 30 | export doc at src/struct.ts:157 |  |  | 0.639 |
| walker |  | 3283 | 227 | export at src/struct.ts:10 |  |  | 0.685 |
| walker |  | 3345 | 62 | headings outline in docs/reference/core.md |  |  | 0.685 |
| walker |  | 3345 | 0 | docs/reference/core.md section #0 |  |  | 0.685 |
| walker |  | 3384 | 39 | export doc at src/struct.ts:266 |  |  | 0.686 |
| walker |  | 3403 | 19 | docs/resources/links.md section #0 |  |  | 0.686 |
| ns | 3426 |  | 818 | top-level assert/create/mask/is/validate | 2.5 | 2.1 | 0.614 |
| walker |  | 3568 | 165 | export body at src/error.ts:25 body 36 |  |  | 0.616 |
| walker |  | 3647 | 79 | headings outline in docs/reference/utilities.md |  |  | 0.616 |
| walker |  | 3693 | 46 | export doc at src/struct.ts:185 |  |  | 0.627 |
| walker |  | 3775 | 82 | headings outline in docs/reference/refinements.md |  |  | 0.627 |
| walker |  | 3813 | 38 | imports in src/struct.ts |  |  | 0.627 |
| ns | 3863 |  | 437 | struct.ts supporting types (Context/Infer/Describe/Result/Coercer/Validator/Refiner) | 2.6 |  | 0.651 |
| walker |  | 3898 | 85 | export body at src/structs/coercions.ts:16 body 21 |  |  | 0.651 |
| walker |  | 4063 | 165 | listing of 'test/validation' |  |  | 0.654 |
| ns | 4401 |  | 538 | error.ts full (Failure type + StructError class) | 3.1 |  | 0.648 |
| walker |  | 4583 | 520 | Readme.md section #0 |  |  | 0.648 |
| walker |  | 4610 | 27 | docs/reference/typescript.md section #0 |  |  | 0.648 |
| walker |  | 4711 | 101 | export names surface in src/structs/refinements.ts |  |  | 0.649 |
| ns | 4722 |  | 321 | types.ts factory locations (all 25) | 4.1 |  | 0.628 |
| walker |  | 4744 | 33 | export at src/structs/refinements.ts:93 |  |  | 0.628 |
| walker |  | 4788 | 44 | export at src/structs/refinements.ts:146 |  |  | 0.628 |
| walker |  | 4836 | 48 | export at src/structs/refinements.ts:8 |  |  | 0.628 |
| walker |  | 4884 | 48 | export at src/structs/refinements.ts:77 |  |  | 0.628 |
| walker |  | 4941 | 57 | export at src/structs/refinements.ts:33 |  |  | 0.628 |
| walker |  | 4998 | 57 | export at src/structs/refinements.ts:55 |  |  | 0.628 |
| walker |  | 5060 | 62 | export at src/structs/refinements.ts:109 |  |  | 0.628 |
| walker |  | 5084 | 24 | export doc at src/structs/refinements.ts:93 |  |  | 0.628 |
| walker |  | 5110 | 26 | export doc at src/structs/refinements.ts:33 |  |  | 0.628 |
| walker |  | 5136 | 26 | export doc at src/structs/refinements.ts:55 |  |  | 0.628 |
| walker |  | 5318 | 182 | listing of 'test/typings' |  |  | 0.631 |
| walker |  | 5347 | 29 | export doc at src/structs/refinements.ts:8 |  |  | 0.631 |
| walker |  | 5376 | 29 | export doc at src/structs/refinements.ts:77 |  |  | 0.631 |
| walker |  | 5443 | 67 | export doc at src/struct.ts:10 |  |  | 0.631 |
| ns | 5507 |  | 785 | array + object factory bodies | 4.2 | 4.1 | 0.579 |
| ns | 5553 |  | 46 | coercions.ts symbol locations | 5.1 |  | 0.581 |
| walker |  | 5769 | 326 | package scripts in package.json |  |  | 0.582 |
| walker |  | 5799 | 30 | docs/reference/utilities.md section #0 |  |  | 0.582 |
| ns | 5938 |  | 385 | defaulted() body | 5.2 | 5.1 | 0.560 |
| walker |  | 6005 | 206 | Readme.md section #9 |  |  | 0.560 |
| walker |  | 6033 | 28 | imports in src/structs/refinements.ts |  |  | 0.560 |
| ns | 6066 |  | 128 | refinements.ts symbol locations | 6.1 |  | 0.563 |
| walker |  | 6070 | 37 | docs/guides/02-validating-data.md section #0 |  |  | 0.563 |
| walker |  | 6108 | 38 | docs/reference/errors.md section #0 |  |  | 0.563 |
| walker |  | 6219 | 111 | Readme.md section #8 |  |  | 0.563 |
| ns | 6335 |  | 269 | refine() body | 6.2 | 6.1 | 0.550 |
| ns | 6466 |  | 131 | utilities.ts symbol locations | 7.1 |  | 0.545 |
| walker |  | 6520 | 301 | package identity metadata in package.json |  |  | 0.559 |
| ns | 6529 |  | 63 | define() body | 7.2 | 7.1 | 0.556 |
| walker |  | 6544 | 24 | module-doc lede in test/index.ts |  |  | 0.556 |
| walker |  | 6641 | 97 | export doc at src/struct.ts:221 |  |  | 0.556 |
| ns | 6695 |  | 166 | utils.ts value-helper locations | 8.1 |  | 0.552 |
| walker |  | 6748 | 107 | export doc at src/error.ts:25 |  |  | 0.574 |
| walker |  | 6793 | 45 | Readme.md section #5 |  |  | 0.575 |
| walker |  | 6837 | 44 | Readme.md section #6 |  |  | 0.576 |
| walker |  | 6885 | 48 | export doc at src/structs/refinements.ts:109 |  |  | 0.576 |
| walker |  | 7070 | 185 | json config tsconfig.json |  |  | 0.577 |
| walker |  | 7123 | 53 | docs/reference/refinements.md section #0 |  |  | 0.577 |
| walker |  | 7173 | 50 | Readme.md section #4 |  |  | 0.579 |
| walker |  | 7191 | 18 | export names surface in test/index.ts |  |  | 0.579 |
| walker |  | 7244 | 53 | Readme.md section #3 |  |  | 0.581 |
| walker |  | 7292 | 48 | imports in src/structs/coercions.ts |  |  | 0.581 |
| walker |  | 7355 | 63 | docs/reference/coercions.md section #0 |  |  | 0.581 |
| ns | 7484 |  | 789 | run() traversal body | 8.2 | 8.1 | 0.547 |
| walker |  | 7575 | 220 | export names surface in src/structs/utilities.ts |  |  | 0.558 |
| walker |  | 7575 | 0 | export at src/structs/utilities.ts:60 |  |  | 0.558 |
| walker |  | 7575 | 0 | export at src/structs/utilities.ts:71 |  |  | 0.558 |
| walker |  | 7575 | 0 | export at src/structs/utilities.ts:140 |  |  | 0.558 |
| walker |  | 7575 | 0 | export at src/structs/utilities.ts:247 |  |  | 0.558 |
| walker |  | 7594 | 19 | export body at src/structs/utilities.ts:71 body 72 |  |  | 0.559 |
| walker |  | 7627 | 33 | export at src/structs/utilities.ts:106 |  |  | 0.559 |
| walker |  | 7664 | 37 | export at src/structs/utilities.ts:197 |  |  | 0.559 |
| walker |  | 7702 | 38 | export at src/structs/utilities.ts:80 |  |  | 0.559 |
| walker |  | 7745 | 43 | export at src/structs/utilities.ts:221 |  |  | 0.559 |
| walker |  | 7789 | 44 | export at src/structs/utilities.ts:171 |  |  | 0.559 |
| walker |  | 7834 | 45 | export at src/structs/utilities.ts:17 |  |  | 0.559 |
| walker |  | 7936 | 102 | export at src/structs/utilities.ts:21 |  |  | 0.559 |
| ns | 7939 |  | 455 | StructError property table | 9.1 |  | 0.553 |
| walker |  | 7962 | 26 | export doc at src/structs/utilities.ts:71 |  |  | 0.556 |
| ns | 8104 |  | 165 | test/validation/ kind roster | 10.1 |  | 0.579 |
| walker |  | 8112 | 150 | export at src/structs/utilities.ts:30 |  |  | 0.579 |
| ns | 8286 |  | 182 | test/typings/ file roster | 10.2 |  | 0.599 |
| walker |  | 8294 | 182 | export at src/structs/utilities.ts:44 |  |  | 0.599 |
| walker |  | 8341 | 47 | export doc at src/structs/utilities.ts:247 |  |  | 0.599 |
| walker |  | 8396 | 55 | export doc at src/structs/utilities.ts:80 |  |  | 0.599 |
| ns | 8418 |  | 132 | test/validation/object/valid.ts (fixture shape) | 10.3 |  | 0.593 |
| walker |  | 8453 | 57 | export doc at src/structs/utilities.ts:171 |  |  | 0.593 |
| walker |  | 8510 | 57 | export doc at src/structs/utilities.ts:221 |  |  | 0.593 |
| walker |  | 8573 | 63 | export doc at src/structs/utilities.ts:197 |  |  | 0.593 |
| ns | 8694 |  | 276 | examples/basic-validation.js | 11.1 |  | 0.581 |
| ns | 9020 |  | 326 | package.json scripts | 12.1 |  | 0.587 |
| walker |  | 9089 | 516 | Readme.md section #2 |  |  | 0.587 |
| walker |  | 9157 | 68 | export doc at src/structs/utilities.ts:17 |  |  | 0.587 |
| ns | 9205 |  | 185 | tsconfig.json | 12.2 |  | 0.594 |
| walker |  | 9234 | 77 | export doc at src/structs/coercions.ts:79 |  |  | 0.594 |
| ns | 9380 |  | 175 | .github/workflows/ci.yml | 12.3 |  | 0.600 |
| ns | 9490 |  | 110 | jsr.json | 12.4 |  | 0.597 |
| walker |  | 9505 | 271 | headings outline in docs/reference/types.md |  |  | 0.597 |
| walker |  | 9547 | 42 | docs/reference/types.md section #0 |  |  | 0.597 |
| walker |  | 9628 | 81 | export doc at src/structs/utilities.ts:106 |  |  | 0.597 |
| ns | 9677 |  | 187 | rollup.config.js | 12.5 |  | 0.590 |
| walker |  | 9710 | 82 | export doc at src/structs/coercions.ts:38 |  |  | 0.593 |
| walker |  | 9801 | 91 | export doc at src/structs/refinements.ts:146 |  |  | 0.598 |
| ns | 9814 |  | 137 | editor/format config (.editorconfig, .prettierrc) | 12.6 |  | 0.593 |
| ns | 9880 |  | 66 | editor + gitbook publish config (.vscode/settings.json, .gitbook.yaml) | 12.7 |  | 0.590 |
| walker |  | 9911 | 110 | json config jsr.json |  |  | 0.597 |
| walker |  | 9991 | 80 | imports in src/structs/utilities.ts |  |  | 0.597 |
| ns | 10028 |  | 148 | Changelog.md — 2.0.0 breaking changes | 13.1 |  | 0.595 |
