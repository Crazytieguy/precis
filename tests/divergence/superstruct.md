Score(3000)=0.638 I=0.894 C=0.455 ns_rows≤3K=14/40 (reached=6 partial=1 missing=7)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 65 | 65 | listing of '.' |  |  | 1.000 |
| ns | 65 |  | 65 | Repo root listing | 1.1 |  | 1.000 |
| walker |  | 84 | 19 | listing of 'src' |  |  | 1.000 |
| walker |  | 102 | 18 | listing of 'src/structs' |  |  | 1.000 |
| ns | 102 |  | 37 | src/ and src/structs/ listings | 1.2 |  | 1.000 |
| walker |  | 123 | 21 | listing of 'docs' |  |  | 1.000 |
| ns | 130 |  | 28 | test/ top-level listing | 1.3 |  | 0.873 |
| walker |  | 131 | 8 | listing of 'docs/resources' |  |  | 0.875 |
| walker |  | 135 | 4 | listing of '.vscode' |  |  | 0.875 |
| ns | 185 |  | 55 | examples/ listing | 1.4 |  | 0.740 |
| walker |  | 217 | 82 | package identity in package.json |  |  | 0.743 |
| walker |  | 272 | 55 | listing of 'examples' |  |  | 0.906 |
| walker |  | 280 | 8 | listing of '.github' |  |  | 0.906 |
| walker |  | 284 | 4 | listing of '.github/workflows' |  |  | 0.906 |
| ns | 325 |  | 140 | docs/ listing (all levels) | 1.5 |  | 0.705 |
| ns | 420 |  | 95 | package.json identity (name/desc/version) | 1.6 |  | 0.695 |
| walker |  | 459 | 175 | YAML config at .github/workflows/ci.yml |  |  | 0.697 |
| walker |  | 538 | 79 | imports in src/index.ts |  |  | 0.697 |
| walker |  | 567 | 29 | listing of 'docs/images' |  |  | 0.741 |
| ns | 572 |  | 152 | package.json entry points, files, engines | 1.7 | 1.6 | 0.672 |
| walker |  | 598 | 31 | listing of 'docs/reference' |  |  | 0.744 |
| walker |  | 632 | 34 | package runtime metadata in package.json |  |  | 0.750 |
| walker |  | 659 | 27 | export names surface in src/error.ts |  |  | 0.750 |
| walker |  | 710 | 51 | listing of 'docs/guides' |  |  | 0.832 |
| ns | 743 |  | 171 | Readme concept lede | 1.8 |  | 0.816 |
| walker |  | 892 | 182 | README headline in Readme.md |  |  | 0.819 |
| walker |  | 946 | 54 | headings outline in Readme.md |  |  | 0.819 |
| walker |  | 964 | 18 | headings outline in docs/resources/links.md |  |  | 0.819 |
| walker |  | 983 | 19 | Readme.md section #11 |  |  | 0.819 |
| walker |  | 1059 | 76 | package entrypoints in package.json |  |  | 0.868 |
| ns | 1060 |  | 317 | docs/summary.md (site nav) | 1.9 |  | 0.756 |
| walker |  | 1087 | 28 | listing of 'test' |  |  | 0.808 |
| walker |  | 1203 | 116 | README headline in docs/readme.md |  |  | 0.808 |
| walker |  | 1284 | 81 | export at src/error.ts:5 |  |  | 0.808 |
| walker |  | 1377 | 93 | export at src/error.ts:25 |  |  | 0.809 |
| ns | 1391 |  | 331 | Readme design principles | 1.10 |  | 0.768 |
| walker |  | 1409 | 32 | headings outline in docs/guides/05-handling-errors.md |  |  | 0.768 |
| walker |  | 1443 | 34 | headings outline in docs/guides/03-coercing-data.md |  |  | 0.768 |
| walker |  | 1477 | 34 | headings outline in docs/guides/04-refining-validation.md |  |  | 0.768 |
| walker |  | 1513 | 36 | headings outline in docs/guides/01-getting-started.md |  |  | 0.768 |
| walker |  | 1513 | 0 | docs/guides/01-getting-started.md section #0 |  |  | 0.768 |
| ns | 1531 |  | 140 | struct.ts symbol locations | 2.1 |  | 0.730 |
| walker |  | 1557 | 44 | headings outline in docs/guides/06-using-typescript.md |  |  | 0.730 |
| walker |  | 1597 | 40 | headings outline in docs/reference/errors.md |  |  | 0.730 |
| walker |  | 1623 | 26 | export doc at src/error.ts:5 |  |  | 0.730 |
| walker |  | 1675 | 52 | headings outline in docs/guides/02-validating-data.md |  |  | 0.697 |
| ns | 1675 |  | 144 | Struct class fields | 2.2 | 2.1 | 0.697 |
| walker |  | 1717 | 42 | headings outline in docs/reference/typescript.md |  |  | 0.697 |
| walker |  | 1742 | 25 | listing of 'test/api' |  |  | 0.697 |
| walker |  | 1787 | 45 | headings outline in docs/reference/coercions.md |  |  | 0.697 |
| walker |  | 1845 | 58 | export names surface in src/structs/coercions.ts |  |  | 0.697 |
| walker |  | 1845 | 0 | export at src/structs/coercions.ts:79 |  |  | 0.697 |
| walker |  | 1864 | 19 | export body at src/structs/coercions.ts:79 body 80 |  |  | 0.697 |
| walker |  | 1911 | 47 | export at src/structs/coercions.ts:16 |  |  | 0.697 |
| walker |  | 1968 | 57 | export at src/structs/coercions.ts:38 |  |  | 0.697 |
| ns | 2058 |  | 383 | Struct constructor body | 2.3 | 2.1 | 0.610 |
| walker |  | 2285 | 317 | mdBook SUMMARY at docs/summary.md |  |  | 0.710 |
| walker |  | 2410 | 125 | listing of 'test/validation' |  |  | 0.714 |
| ns | 2598 |  | 540 | Struct instance methods (assert/create/is/mask/validate) | 2.4 | 2.1 | 0.618 |
| walker |  | 2643 | 233 | export names surface in src/struct.ts |  |  | 0.634 |
| walker |  | 2643 | 0 | export at src/struct.ts:175 |  |  | 0.634 |
| walker |  | 2643 | 0 | export at src/struct.ts:231 |  |  | 0.634 |
| walker |  | 2643 | 0 | export at src/struct.ts:237 |  |  | 0.634 |
| walker |  | 2643 | 0 | export at src/struct.ts:253 |  |  | 0.634 |
| walker |  | 2643 | 0 | export at src/struct.ts:259 |  |  | 0.634 |
| walker |  | 2643 | 0 | export at src/struct.ts:266 |  |  | 0.634 |
| walker |  | 2677 | 34 | export at src/struct.ts:221 |  |  | 0.634 |
| walker |  | 2718 | 41 | export at src/struct.ts:243 |  |  | 0.635 |
| walker |  | 2755 | 37 | export at src/struct.ts:139 |  |  | 0.635 |
| walker |  | 2792 | 37 | export at src/struct.ts:157 |  |  | 0.635 |
| walker |  | 2832 | 40 | export at src/struct.ts:123 |  |  | 0.636 |
| walker |  | 2915 | 83 | export at src/struct.ts:185 |  |  | 0.637 |
| walker |  | 2939 | 24 | export body at src/struct.ts:175 body 176 |  |  | 0.637 |
| walker |  | 2962 | 23 | export doc at src/struct.ts:243 |  |  | 0.637 |
| walker |  | 2987 | 25 | export doc at src/struct.ts:259 |  |  | 0.638 |
| walker |  | 3010 | 23 | export doc at src/struct.ts:175 |  |  | 0.638 |
| walker |  | 3037 | 27 | export doc at src/struct.ts:231 |  |  | 0.639 |
| walker |  | 3064 | 27 | export doc at src/struct.ts:237 |  |  | 0.640 |
| walker |  | 3092 | 28 | export doc at src/struct.ts:253 |  |  | 0.640 |
| walker |  | 3120 | 28 | export doc at src/struct.ts:123 |  |  | 0.641 |
| walker |  | 3149 | 29 | export doc at src/struct.ts:139 |  |  | 0.641 |
| walker |  | 3179 | 30 | export doc at src/struct.ts:157 |  |  | 0.642 |
| walker |  | 3406 | 227 | export at src/struct.ts:10 |  |  | 0.688 |
| ns | 3416 |  | 818 | top-level assert/create/mask/is/validate | 2.5 | 2.1 | 0.616 |
| walker |  | 3468 | 62 | headings outline in docs/reference/core.md |  |  | 0.616 |
| walker |  | 3468 | 0 | docs/reference/core.md section #0 |  |  | 0.616 |
| walker |  | 3507 | 39 | export doc at src/struct.ts:266 |  |  | 0.617 |
| walker |  | 3526 | 19 | docs/resources/links.md section #0 |  |  | 0.617 |
| walker |  | 3691 | 165 | export body at src/error.ts:25 body 36 |  |  | 0.619 |
| walker |  | 3770 | 79 | headings outline in docs/reference/utilities.md |  |  | 0.619 |
| walker |  | 3816 | 46 | export doc at src/struct.ts:185 |  |  | 0.630 |
| ns | 3853 |  | 437 | struct.ts supporting types (Context/Infer/Describe/Result/Coercer/Validator/Refiner) | 2.6 |  | 0.654 |
| walker |  | 3898 | 82 | headings outline in docs/reference/refinements.md |  |  | 0.654 |
| walker |  | 4081 | 183 | listing of 'test/typings' |  |  | 0.658 |
| walker |  | 4119 | 38 | imports in src/struct.ts |  |  | 0.658 |
| walker |  | 4204 | 85 | export body at src/structs/coercions.ts:16 body 21 |  |  | 0.658 |
| ns | 4391 |  | 538 | error.ts full (Failure type + StructError class) | 3.1 |  | 0.651 |
| ns | 4712 |  | 321 | types.ts factory locations (all 25) | 4.1 |  | 0.631 |
| walker |  | 4724 | 520 | Readme.md section #0 |  |  | 0.631 |
| walker |  | 4751 | 27 | docs/reference/typescript.md section #0 |  |  | 0.631 |
| walker |  | 4852 | 101 | export names surface in src/structs/refinements.ts |  |  | 0.631 |
| walker |  | 4885 | 33 | export at src/structs/refinements.ts:93 |  |  | 0.631 |
| walker |  | 4929 | 44 | export at src/structs/refinements.ts:146 |  |  | 0.631 |
| walker |  | 4977 | 48 | export at src/structs/refinements.ts:8 |  |  | 0.631 |
| walker |  | 5025 | 48 | export at src/structs/refinements.ts:77 |  |  | 0.631 |
| walker |  | 5082 | 57 | export at src/structs/refinements.ts:33 |  |  | 0.631 |
| walker |  | 5139 | 57 | export at src/structs/refinements.ts:55 |  |  | 0.631 |
| walker |  | 5201 | 62 | export at src/structs/refinements.ts:109 |  |  | 0.631 |
| walker |  | 5225 | 24 | export doc at src/structs/refinements.ts:93 |  |  | 0.631 |
| walker |  | 5251 | 26 | export doc at src/structs/refinements.ts:33 |  |  | 0.631 |
| walker |  | 5277 | 26 | export doc at src/structs/refinements.ts:55 |  |  | 0.631 |
| walker |  | 5306 | 29 | export doc at src/structs/refinements.ts:8 |  |  | 0.631 |
| walker |  | 5335 | 29 | export doc at src/structs/refinements.ts:77 |  |  | 0.631 |
| walker |  | 5402 | 67 | export doc at src/struct.ts:10 |  |  | 0.631 |
| ns | 5497 |  | 785 | array + object factory bodies | 4.2 | 4.1 | 0.579 |
| ns | 5543 |  | 46 | coercions.ts symbol locations | 5.1 |  | 0.581 |
| walker |  | 5728 | 326 | package scripts in package.json |  |  | 0.582 |
| walker |  | 5758 | 30 | docs/reference/utilities.md section #0 |  |  | 0.582 |
| ns | 5928 |  | 385 | defaulted() body | 5.2 | 5.1 | 0.560 |
| walker |  | 5964 | 206 | Readme.md section #9 |  |  | 0.560 |
| walker |  | 5992 | 28 | imports in src/structs/refinements.ts |  |  | 0.560 |
| walker |  | 6029 | 37 | docs/guides/02-validating-data.md section #0 |  |  | 0.560 |
| ns | 6056 |  | 128 | refinements.ts symbol locations | 6.1 |  | 0.563 |
| walker |  | 6067 | 38 | docs/reference/errors.md section #0 |  |  | 0.563 |
| walker |  | 6178 | 111 | Readme.md section #8 |  |  | 0.563 |
| ns | 6325 |  | 269 | refine() body | 6.2 | 6.1 | 0.550 |
| ns | 6456 |  | 131 | utilities.ts symbol locations | 7.1 |  | 0.545 |
| walker |  | 6479 | 301 | package identity metadata in package.json |  |  | 0.559 |
| walker |  | 6503 | 24 | module-doc lede in test/index.ts |  |  | 0.559 |
| ns | 6519 |  | 63 | define() body | 7.2 | 7.1 | 0.556 |
| walker |  | 6600 | 97 | export doc at src/struct.ts:221 |  |  | 0.556 |
| ns | 6685 |  | 166 | utils.ts value-helper locations | 8.1 |  | 0.552 |
| walker |  | 6707 | 107 | export doc at src/error.ts:25 |  |  | 0.574 |
| walker |  | 6752 | 45 | Readme.md section #5 |  |  | 0.575 |
| walker |  | 6796 | 44 | Readme.md section #6 |  |  | 0.576 |
| walker |  | 6844 | 48 | export doc at src/structs/refinements.ts:109 |  |  | 0.576 |
| walker |  | 7029 | 185 | json config tsconfig.json |  |  | 0.577 |
| walker |  | 7082 | 53 | docs/reference/refinements.md section #0 |  |  | 0.577 |
| walker |  | 7132 | 50 | Readme.md section #4 |  |  | 0.579 |
| walker |  | 7150 | 18 | export names surface in test/index.ts |  |  | 0.579 |
| walker |  | 7203 | 53 | Readme.md section #3 |  |  | 0.581 |
| walker |  | 7251 | 48 | imports in src/structs/coercions.ts |  |  | 0.581 |
| walker |  | 7314 | 63 | docs/reference/coercions.md section #0 |  |  | 0.581 |
| ns | 7474 |  | 789 | run() traversal body | 8.2 | 8.1 | 0.547 |
| walker |  | 7534 | 220 | export names surface in src/structs/utilities.ts |  |  | 0.558 |
| walker |  | 7534 | 0 | export at src/structs/utilities.ts:60 |  |  | 0.558 |
| walker |  | 7534 | 0 | export at src/structs/utilities.ts:71 |  |  | 0.558 |
| walker |  | 7534 | 0 | export at src/structs/utilities.ts:140 |  |  | 0.558 |
| walker |  | 7534 | 0 | export at src/structs/utilities.ts:247 |  |  | 0.558 |
| walker |  | 7553 | 19 | export body at src/structs/utilities.ts:71 body 72 |  |  | 0.559 |
| walker |  | 7586 | 33 | export at src/structs/utilities.ts:106 |  |  | 0.559 |
| walker |  | 7623 | 37 | export at src/structs/utilities.ts:197 |  |  | 0.559 |
| walker |  | 7661 | 38 | export at src/structs/utilities.ts:80 |  |  | 0.559 |
| walker |  | 7704 | 43 | export at src/structs/utilities.ts:221 |  |  | 0.559 |
| walker |  | 7748 | 44 | export at src/structs/utilities.ts:171 |  |  | 0.559 |
| walker |  | 7793 | 45 | export at src/structs/utilities.ts:17 |  |  | 0.559 |
| walker |  | 7895 | 102 | export at src/structs/utilities.ts:21 |  |  | 0.559 |
| walker |  | 7921 | 26 | export doc at src/structs/utilities.ts:71 |  |  | 0.562 |
| ns | 7929 |  | 455 | StructError property table | 9.1 |  | 0.556 |
| ns | 8054 |  | 125 | test/validation/ kind roster | 10.1 |  | 0.579 |
| walker |  | 8071 | 150 | export at src/structs/utilities.ts:30 |  |  | 0.579 |
| ns | 8237 |  | 183 | test/typings/ file roster | 10.2 |  | 0.599 |
| walker |  | 8253 | 182 | export at src/structs/utilities.ts:44 |  |  | 0.599 |
| walker |  | 8300 | 47 | export doc at src/structs/utilities.ts:247 |  |  | 0.599 |
| walker |  | 8355 | 55 | export doc at src/structs/utilities.ts:80 |  |  | 0.599 |
| ns | 8369 |  | 132 | test/validation/object/valid.ts (fixture shape) | 10.3 |  | 0.593 |
| walker |  | 8412 | 57 | export doc at src/structs/utilities.ts:171 |  |  | 0.593 |
| walker |  | 8469 | 57 | export doc at src/structs/utilities.ts:221 |  |  | 0.593 |
| walker |  | 8532 | 63 | export doc at src/structs/utilities.ts:197 |  |  | 0.593 |
| ns | 8645 |  | 276 | examples/basic-validation.js | 11.1 |  | 0.581 |
| ns | 8971 |  | 326 | package.json scripts | 12.1 |  | 0.587 |
| walker |  | 9048 | 516 | Readme.md section #2 |  |  | 0.587 |
| walker |  | 9116 | 68 | export doc at src/structs/utilities.ts:17 |  |  | 0.587 |
| ns | 9156 |  | 185 | tsconfig.json | 12.2 |  | 0.594 |
| walker |  | 9193 | 77 | export doc at src/structs/coercions.ts:79 |  |  | 0.594 |
| ns | 9331 |  | 175 | .github/workflows/ci.yml | 12.3 |  | 0.600 |
| ns | 9441 |  | 110 | jsr.json | 12.4 |  | 0.597 |
| walker |  | 9464 | 271 | headings outline in docs/reference/types.md |  |  | 0.597 |
| walker |  | 9506 | 42 | docs/reference/types.md section #0 |  |  | 0.597 |
| walker |  | 9587 | 81 | export doc at src/structs/utilities.ts:106 |  |  | 0.597 |
| ns | 9628 |  | 187 | rollup.config.js | 12.5 |  | 0.590 |
| walker |  | 9669 | 82 | export doc at src/structs/coercions.ts:38 |  |  | 0.593 |
| walker |  | 9760 | 91 | export doc at src/structs/refinements.ts:146 |  |  | 0.598 |
| ns | 9765 |  | 137 | editor/format config (.editorconfig, .prettierrc) | 12.6 |  | 0.593 |
| ns | 9831 |  | 66 | editor + gitbook publish config (.vscode/settings.json, .gitbook.yaml) | 12.7 |  | 0.590 |
| walker |  | 9870 | 110 | json config jsr.json |  |  | 0.597 |
| walker |  | 9950 | 80 | imports in src/structs/utilities.ts |  |  | 0.597 |
| walker |  | 9976 | 26 | package identity in examples/package.json |  |  | 0.597 |
| ns | 9979 |  | 148 | Changelog.md — 2.0.0 breaking changes | 13.1 |  | 0.595 |
