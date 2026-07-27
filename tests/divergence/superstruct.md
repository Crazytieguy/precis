Score(3000)=0.639 I=0.887 C=0.460 ns_rows≤3K=14/40 (reached=6 partial=2 missing=6)

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
| walker |  | 225 | 8 | listing of '.github' |  |  | 0.743 |
| walker |  | 229 | 4 | listing of '.github/workflows' |  |  | 0.743 |
| ns | 325 |  | 140 | docs/ listing (all levels) | 1.5 |  | 0.585 |
| walker |  | 404 | 175 | YAML config at .github/workflows/ci.yml |  |  | 0.586 |
| ns | 420 |  | 95 | package.json identity (name/desc/version) | 1.6 |  | 0.584 |
| walker |  | 459 | 55 | listing of 'examples' |  |  | 0.697 |
| walker |  | 538 | 79 | imports in src/index.ts |  |  | 0.697 |
| walker |  | 567 | 29 | listing of 'docs/images' |  |  | 0.741 |
| ns | 572 |  | 152 | package.json entry points, files, engines | 1.7 | 1.6 | 0.672 |
| walker |  | 598 | 31 | listing of 'docs/reference' |  |  | 0.744 |
| walker |  | 632 | 34 | package runtime metadata in package.json |  |  | 0.750 |
| walker |  | 659 | 27 | export names surface in src/error.ts |  |  | 0.750 |
| walker |  | 710 | 51 | listing of 'docs/guides' |  |  | 0.832 |
| ns | 743 |  | 171 | Readme concept lede | 1.8 |  | 0.816 |
| walker |  | 892 | 182 | README headline in Readme.md |  |  | 0.819 |
| walker |  | 958 | 66 | README prelude in Readme.md |  |  | 0.827 |
| walker |  | 1012 | 54 | headings outline in Readme.md |  |  | 0.827 |
| walker |  | 1030 | 18 | headings outline in docs/resources/links.md |  |  | 0.827 |
| walker |  | 1049 | 19 | Readme.md section #11 |  |  | 0.827 |
| ns | 1060 |  | 317 | docs/summary.md (site nav) | 1.9 |  | 0.721 |
| walker |  | 1125 | 76 | package entrypoints in package.json |  |  | 0.764 |
| walker |  | 1241 | 116 | README headline in docs/readme.md |  |  | 0.764 |
| walker |  | 1307 | 66 | README prelude in docs/readme.md |  |  | 0.764 |
| walker |  | 1388 | 81 | export at src/error.ts:5 |  |  | 0.764 |
| ns | 1391 |  | 331 | Readme design principles | 1.10 |  | 0.726 |
| walker |  | 1481 | 93 | export at src/error.ts:25 |  |  | 0.726 |
| walker |  | 1509 | 28 | listing of 'test' |  |  | 0.775 |
| ns | 1531 |  | 140 | struct.ts symbol locations | 2.1 |  | 0.736 |
| walker |  | 1541 | 32 | headings outline in docs/guides/05-handling-errors.md |  |  | 0.736 |
| walker |  | 1575 | 34 | headings outline in docs/guides/03-coercing-data.md |  |  | 0.736 |
| walker |  | 1609 | 34 | headings outline in docs/guides/04-refining-validation.md |  |  | 0.736 |
| walker |  | 1645 | 36 | headings outline in docs/guides/01-getting-started.md |  |  | 0.736 |
| walker |  | 1645 | 0 | docs/guides/01-getting-started.md section #0 |  |  | 0.736 |
| ns | 1675 |  | 144 | Struct class fields | 2.2 | 2.1 | 0.702 |
| walker |  | 1689 | 44 | headings outline in docs/guides/06-using-typescript.md |  |  | 0.702 |
| walker |  | 1729 | 40 | headings outline in docs/reference/errors.md |  |  | 0.702 |
| walker |  | 1755 | 26 | export doc at src/error.ts:5 |  |  | 0.703 |
| walker |  | 1807 | 52 | headings outline in docs/guides/02-validating-data.md |  |  | 0.703 |
| walker |  | 1849 | 42 | headings outline in docs/reference/typescript.md |  |  | 0.703 |
| walker |  | 1894 | 45 | headings outline in docs/reference/coercions.md |  |  | 0.703 |
| walker |  | 1919 | 25 | listing of 'test/api' |  |  | 0.703 |
| walker |  | 1977 | 58 | export names surface in src/structs/coercions.ts |  |  | 0.703 |
| walker |  | 1977 | 0 | export at src/structs/coercions.ts:79 |  |  | 0.703 |
| walker |  | 1996 | 19 | export body at src/structs/coercions.ts:79 body 80 |  |  | 0.703 |
| walker |  | 2043 | 47 | export at src/structs/coercions.ts:16 |  |  | 0.703 |
| ns | 2058 |  | 383 | Struct constructor body | 2.3 | 2.1 | 0.615 |
| walker |  | 2100 | 57 | export at src/structs/coercions.ts:38 |  |  | 0.615 |
| walker |  | 2417 | 317 | mdBook SUMMARY at docs/summary.md |  |  | 0.715 |
| ns | 2598 |  | 540 | Struct instance methods (assert/create/is/mask/validate) | 2.4 | 2.1 | 0.619 |
| walker |  | 2650 | 233 | export names surface in src/struct.ts |  |  | 0.635 |
| walker |  | 2650 | 0 | export at src/struct.ts:175 |  |  | 0.635 |
| walker |  | 2650 | 0 | export at src/struct.ts:231 |  |  | 0.635 |
| walker |  | 2650 | 0 | export at src/struct.ts:237 |  |  | 0.635 |
| walker |  | 2650 | 0 | export at src/struct.ts:253 |  |  | 0.635 |
| walker |  | 2650 | 0 | export at src/struct.ts:259 |  |  | 0.635 |
| walker |  | 2650 | 0 | export at src/struct.ts:266 |  |  | 0.635 |
| walker |  | 2684 | 34 | export at src/struct.ts:221 |  |  | 0.635 |
| walker |  | 2725 | 41 | export at src/struct.ts:243 |  |  | 0.636 |
| walker |  | 2762 | 37 | export at src/struct.ts:139 |  |  | 0.636 |
| walker |  | 2799 | 37 | export at src/struct.ts:157 |  |  | 0.636 |
| walker |  | 2839 | 40 | export at src/struct.ts:123 |  |  | 0.637 |
| walker |  | 2922 | 83 | export at src/struct.ts:185 |  |  | 0.638 |
| walker |  | 2946 | 24 | export body at src/struct.ts:175 body 176 |  |  | 0.638 |
| walker |  | 2969 | 23 | export doc at src/struct.ts:243 |  |  | 0.638 |
| walker |  | 2994 | 25 | export doc at src/struct.ts:259 |  |  | 0.639 |
| walker |  | 3017 | 23 | export doc at src/struct.ts:175 |  |  | 0.639 |
| walker |  | 3044 | 27 | export doc at src/struct.ts:231 |  |  | 0.640 |
| walker |  | 3071 | 27 | export doc at src/struct.ts:237 |  |  | 0.641 |
| walker |  | 3099 | 28 | export doc at src/struct.ts:253 |  |  | 0.641 |
| walker |  | 3127 | 28 | export doc at src/struct.ts:123 |  |  | 0.642 |
| walker |  | 3156 | 29 | export doc at src/struct.ts:139 |  |  | 0.643 |
| walker |  | 3186 | 30 | export doc at src/struct.ts:157 |  |  | 0.643 |
| walker |  | 3413 | 227 | export at src/struct.ts:10 |  |  | 0.689 |
| ns | 3416 |  | 818 | top-level assert/create/mask/is/validate | 2.5 | 2.1 | 0.616 |
| walker |  | 3475 | 62 | headings outline in docs/reference/core.md |  |  | 0.616 |
| walker |  | 3475 | 0 | docs/reference/core.md section #0 |  |  | 0.616 |
| walker |  | 3514 | 39 | export doc at src/struct.ts:266 |  |  | 0.617 |
| walker |  | 3533 | 19 | docs/resources/links.md section #0 |  |  | 0.617 |
| walker |  | 3658 | 125 | listing of 'test/validation' |  |  | 0.620 |
| walker |  | 3823 | 165 | export body at src/error.ts:25 body 36 |  |  | 0.622 |
| ns | 3853 |  | 437 | struct.ts supporting types (Context/Infer/Describe/Result/Coercer/Validator/Refiner) | 2.6 |  | 0.647 |
| walker |  | 3902 | 79 | headings outline in docs/reference/utilities.md |  |  | 0.647 |
| walker |  | 3948 | 46 | export doc at src/struct.ts:185 |  |  | 0.657 |
| walker |  | 4030 | 82 | headings outline in docs/reference/refinements.md |  |  | 0.657 |
| walker |  | 4068 | 38 | imports in src/struct.ts |  |  | 0.657 |
| walker |  | 4153 | 85 | export body at src/structs/coercions.ts:16 body 21 |  |  | 0.657 |
| ns | 4391 |  | 538 | error.ts full (Failure type + StructError class) | 3.1 |  | 0.651 |
| walker |  | 4673 | 520 | Readme.md section #0 |  |  | 0.651 |
| walker |  | 4700 | 27 | docs/reference/typescript.md section #0 |  |  | 0.651 |
| ns | 4712 |  | 321 | types.ts factory locations (all 25) | 4.1 |  | 0.630 |
| walker |  | 4801 | 101 | export names surface in src/structs/refinements.ts |  |  | 0.631 |
| walker |  | 4834 | 33 | export at src/structs/refinements.ts:93 |  |  | 0.631 |
| walker |  | 4878 | 44 | export at src/structs/refinements.ts:146 |  |  | 0.631 |
| walker |  | 4926 | 48 | export at src/structs/refinements.ts:8 |  |  | 0.631 |
| walker |  | 4974 | 48 | export at src/structs/refinements.ts:77 |  |  | 0.631 |
| walker |  | 5031 | 57 | export at src/structs/refinements.ts:33 |  |  | 0.631 |
| walker |  | 5088 | 57 | export at src/structs/refinements.ts:55 |  |  | 0.631 |
| walker |  | 5150 | 62 | export at src/structs/refinements.ts:109 |  |  | 0.631 |
| walker |  | 5174 | 24 | export doc at src/structs/refinements.ts:93 |  |  | 0.631 |
| walker |  | 5200 | 26 | export doc at src/structs/refinements.ts:33 |  |  | 0.631 |
| walker |  | 5226 | 26 | export doc at src/structs/refinements.ts:55 |  |  | 0.631 |
| walker |  | 5409 | 183 | listing of 'test/typings' |  |  | 0.634 |
| walker |  | 5438 | 29 | export doc at src/structs/refinements.ts:8 |  |  | 0.634 |
| walker |  | 5467 | 29 | export doc at src/structs/refinements.ts:77 |  |  | 0.634 |
| ns | 5497 |  | 785 | array + object factory bodies | 4.2 | 4.1 | 0.581 |
| walker |  | 5534 | 67 | export doc at src/struct.ts:10 |  |  | 0.581 |
| ns | 5543 |  | 46 | coercions.ts symbol locations | 5.1 |  | 0.583 |
| walker |  | 5860 | 326 | package scripts in package.json |  |  | 0.584 |
| walker |  | 5890 | 30 | docs/reference/utilities.md section #0 |  |  | 0.584 |
| ns | 5928 |  | 385 | defaulted() body | 5.2 | 5.1 | 0.562 |
| ns | 6056 |  | 128 | refinements.ts symbol locations | 6.1 |  | 0.565 |
| walker |  | 6096 | 206 | Readme.md section #9 |  |  | 0.565 |
| walker |  | 6124 | 28 | imports in src/structs/refinements.ts |  |  | 0.565 |
| walker |  | 6161 | 37 | docs/guides/02-validating-data.md section #0 |  |  | 0.565 |
| walker |  | 6199 | 38 | docs/reference/errors.md section #0 |  |  | 0.565 |
| walker |  | 6310 | 111 | Readme.md section #8 |  |  | 0.565 |
| ns | 6325 |  | 269 | refine() body | 6.2 | 6.1 | 0.552 |
| ns | 6456 |  | 131 | utilities.ts symbol locations | 7.1 |  | 0.548 |
| ns | 6519 |  | 63 | define() body | 7.2 | 7.1 | 0.545 |
| walker |  | 6611 | 301 | package identity metadata in package.json |  |  | 0.559 |
| walker |  | 6635 | 24 | module-doc lede in test/index.ts |  |  | 0.559 |
| ns | 6685 |  | 166 | utils.ts value-helper locations | 8.1 |  | 0.554 |
| walker |  | 6732 | 97 | export doc at src/struct.ts:221 |  |  | 0.554 |
| walker |  | 6839 | 107 | export doc at src/error.ts:25 |  |  | 0.576 |
| walker |  | 6884 | 45 | Readme.md section #5 |  |  | 0.577 |
| walker |  | 6928 | 44 | Readme.md section #6 |  |  | 0.578 |
| walker |  | 6976 | 48 | export doc at src/structs/refinements.ts:109 |  |  | 0.578 |
| walker |  | 7161 | 185 | json config tsconfig.json |  |  | 0.579 |
| walker |  | 7214 | 53 | docs/reference/refinements.md section #0 |  |  | 0.579 |
| walker |  | 7264 | 50 | Readme.md section #4 |  |  | 0.581 |
| walker |  | 7282 | 18 | export names surface in test/index.ts |  |  | 0.581 |
| walker |  | 7335 | 53 | Readme.md section #3 |  |  | 0.583 |
| walker |  | 7383 | 48 | imports in src/structs/coercions.ts |  |  | 0.583 |
| walker |  | 7446 | 63 | docs/reference/coercions.md section #0 |  |  | 0.583 |
| ns | 7474 |  | 789 | run() traversal body | 8.2 | 8.1 | 0.549 |
| walker |  | 7666 | 220 | export names surface in src/structs/utilities.ts |  |  | 0.560 |
| walker |  | 7666 | 0 | export at src/structs/utilities.ts:60 |  |  | 0.560 |
| walker |  | 7666 | 0 | export at src/structs/utilities.ts:71 |  |  | 0.560 |
| walker |  | 7666 | 0 | export at src/structs/utilities.ts:140 |  |  | 0.560 |
| walker |  | 7666 | 0 | export at src/structs/utilities.ts:247 |  |  | 0.560 |
| walker |  | 7685 | 19 | export body at src/structs/utilities.ts:71 body 72 |  |  | 0.561 |
| walker |  | 7718 | 33 | export at src/structs/utilities.ts:106 |  |  | 0.561 |
| walker |  | 7755 | 37 | export at src/structs/utilities.ts:197 |  |  | 0.561 |
| walker |  | 7793 | 38 | export at src/structs/utilities.ts:80 |  |  | 0.561 |
| walker |  | 7836 | 43 | export at src/structs/utilities.ts:221 |  |  | 0.561 |
| walker |  | 7880 | 44 | export at src/structs/utilities.ts:171 |  |  | 0.561 |
| walker |  | 7925 | 45 | export at src/structs/utilities.ts:17 |  |  | 0.561 |
| ns | 7929 |  | 455 | StructError property table | 9.1 |  | 0.555 |
| walker |  | 8027 | 102 | export at src/structs/utilities.ts:21 |  |  | 0.555 |
| walker |  | 8053 | 26 | export doc at src/structs/utilities.ts:71 |  |  | 0.558 |
| ns | 8054 |  | 125 | test/validation/ kind roster | 10.1 |  | 0.580 |
| walker |  | 8203 | 150 | export at src/structs/utilities.ts:30 |  |  | 0.580 |
| ns | 8237 |  | 183 | test/typings/ file roster | 10.2 |  | 0.601 |
| ns | 8369 |  | 132 | test/validation/object/valid.ts (fixture shape) | 10.3 |  | 0.595 |
| walker |  | 8385 | 182 | export at src/structs/utilities.ts:44 |  |  | 0.595 |
| walker |  | 8432 | 47 | export doc at src/structs/utilities.ts:247 |  |  | 0.595 |
| walker |  | 8487 | 55 | export doc at src/structs/utilities.ts:80 |  |  | 0.595 |
| walker |  | 8544 | 57 | export doc at src/structs/utilities.ts:171 |  |  | 0.595 |
| walker |  | 8601 | 57 | export doc at src/structs/utilities.ts:221 |  |  | 0.595 |
| ns | 8645 |  | 276 | examples/basic-validation.js | 11.1 |  | 0.582 |
| walker |  | 8664 | 63 | export doc at src/structs/utilities.ts:197 |  |  | 0.582 |
| ns | 8971 |  | 326 | package.json scripts | 12.1 |  | 0.589 |
| ns | 9156 |  | 185 | tsconfig.json | 12.2 |  | 0.596 |
| walker |  | 9180 | 516 | Readme.md section #2 |  |  | 0.596 |
| walker |  | 9248 | 68 | export doc at src/structs/utilities.ts:17 |  |  | 0.596 |
| walker |  | 9325 | 77 | export doc at src/structs/coercions.ts:79 |  |  | 0.596 |
| ns | 9331 |  | 175 | .github/workflows/ci.yml | 12.3 |  | 0.602 |
| ns | 9441 |  | 110 | jsr.json | 12.4 |  | 0.598 |
| walker |  | 9596 | 271 | headings outline in docs/reference/types.md |  |  | 0.598 |
| ns | 9628 |  | 187 | rollup.config.js | 12.5 |  | 0.591 |
| walker |  | 9638 | 42 | docs/reference/types.md section #0 |  |  | 0.591 |
| walker |  | 9719 | 81 | export doc at src/structs/utilities.ts:106 |  |  | 0.591 |
| ns | 9765 |  | 137 | editor/format config (.editorconfig, .prettierrc) | 12.6 |  | 0.586 |
| walker |  | 9801 | 82 | export doc at src/structs/coercions.ts:38 |  |  | 0.589 |
| ns | 9831 |  | 66 | editor + gitbook publish config (.vscode/settings.json, .gitbook.yaml) | 12.7 |  | 0.587 |
| walker |  | 9892 | 91 | export doc at src/structs/refinements.ts:146 |  |  | 0.592 |
| ns | 9979 |  | 148 | Changelog.md — 2.0.0 breaking changes | 13.1 |  | 0.590 |
