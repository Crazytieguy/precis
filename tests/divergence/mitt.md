Score(3000)=0.677 I=0.828 C=0.554 ns_rows≤3K=22/43 (reached=13 partial=4 missing=5)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 29 | 29 | listing of '.' |  |  | 1.000 |
| ns | 29 |  | 29 | Top-level directory listing | 1.1 |  | 1.000 |
| walker |  | 33 | 4 | listing of 'src' |  |  | 1.000 |
| walker |  | 43 | 10 | listing of '.github' |  |  | 1.000 |
| walker |  | 52 | 9 | listing of '.github/workflows' |  |  | 1.000 |
| ns | 57 |  | 28 | README title + tagline | 1.2 |  | 0.845 |
| ns | 104 |  | 47 | package.json name + version + description | 1.3 |  | 0.738 |
| ns | 120 |  | 16 | src/ and test/ listings | 1.4 |  | 0.672 |
| ns | 212 |  | 92 | package.json entrypoint + source fields | 1.5 |  | 0.565 |
| walker |  | 225 | 173 | package identity in package.json |  |  | 0.663 |
| ns | 362 |  | 150 | README feature bullets | 1.6 |  | 0.569 |
| walker |  | 393 | 168 | README headline in README.md |  |  | 0.760 |
| ns | 470 |  | 108 | src/index.ts public exports — name-only locations | 2.1 |  | 0.662 |
| walker |  | 553 | 160 | export names surface in src/index.ts |  |  | 0.805 |
| walker |  | 565 | 12 | export at src/index.ts:13 |  |  | 0.807 |
| walker |  | 588 | 23 | export at src/index.ts:46 |  |  | 0.810 |
| walker |  | 614 | 26 | export at src/index.ts:6 |  |  | 0.813 |
| walker |  | 649 | 35 | export at src/index.ts:18 |  |  | 0.816 |
| ns | 667 |  | 197 | Emitter<Events> interface — full | 2.2 | 2.1 | 0.659 |
| walker |  | 829 | 180 | export at src/index.ts:23 |  |  | 0.827 |
| walker |  | 877 | 48 | export doc at src/index.ts:46 |  |  | 0.827 |
| ns | 889 |  | 222 | README quickstart code example | 2.3 |  | 0.679 |
| ns | 912 |  | 23 | mitt() default-export signature | 2.4 | 2.1 | 0.686 |
| ns | 989 |  | 77 | Handler / WildcardHandler type aliases | 2.5 | 2.1 | 0.678 |
| walker |  | 1023 | 146 | headings outline in README.md |  |  | 0.679 |
| walker |  | 1051 | 28 | README.md section #18 |  |  | 0.679 |
| walker |  | 1086 | 35 | README.md section #15 |  |  | 0.679 |
| ns | 1096 |  | 107 | EventHandlerMap type | 2.6 | 2.1 | 0.669 |
| walker |  | 1099 | 13 | README.md section #8 |  |  | 0.670 |
| walker |  | 1179 | 80 | README.md section #1 |  |  | 0.670 |
| walker |  | 1194 | 15 | .github/PULL_REQUEST_TEMPLATE.md section #2 |  |  | 0.670 |
| walker |  | 1210 | 16 | .github/PULL_REQUEST_TEMPLATE.md section #1 |  |  | 0.670 |
| walker |  | 1227 | 17 | README.md section #13 |  |  | 0.671 |
| ns | 1302 |  | 206 | README API one-line method descriptions | 2.7 |  | 0.618 |
| ns | 1401 |  | 99 | mitt() body — Map default + return-shape skeleton | 3.1 | 2.4 | 0.587 |
| walker |  | 1439 | 212 | package entrypoints in package.json |  |  | 0.635 |
| ns | 1606 |  | 205 | emit() body — the only non-trivial method | 3.2 | 2.2 | 0.587 |
| ns | 1717 |  | 111 | on() body | 3.3 | 2.2 | 0.570 |
| ns | 1844 |  | 127 | off() body | 3.4 | 2.2 | 0.550 |
| ns | 1981 |  | 137 | emit() JSDoc | 3.5 | 2.2 | 0.532 |
| ns | 2294 |  | 313 | README TypeScript usage section | 3.6 |  | 0.485 |
| walker |  | 2310 | 871 | export body at src/index.ts:46 body 49 |  |  | 0.687 |
| walker |  | 2545 | 235 | package scripts in package.json |  |  | 0.689 |
| walker |  | 2559 | 14 | README.md section #7 |  |  | 0.695 |
| ns | 2626 |  | 332 | test/index_test.ts test labels — all describe + it titles | 3.7 |  | 0.651 |
| walker |  | 2662 | 103 | README.md section #4 |  |  | 0.652 |
| walker |  | 2811 | 149 | json config tsconfig.json |  |  | 0.655 |
| ns | 2829 |  | 203 | on() / off() JSDoc | 3.8 | 2.2 | 0.668 |
| walker |  | 2841 | 30 | .github/PULL_REQUEST_TEMPLATE.md section #3 |  |  | 0.668 |
| walker |  | 2873 | 32 | README.md section #12 |  |  | 0.680 |
| walker |  | 2906 | 33 | README.md section #10 |  |  | 0.696 |
| ns | 2949 |  | 120 | compressed-size CI workflow | 4.1 |  | 0.677 |
| ns | 3089 |  | 140 | package.json mocha + prettier blocks | 4.2 |  | 0.654 |
| ns | 3238 |  | 149 | tsconfig.json — full | 4.3 |  | 0.667 |
| walker |  | 3272 | 366 | package dependencies in package.json |  |  | 0.669 |
| walker |  | 3377 | 105 | .github/PULL_REQUEST_TEMPLATE.md section #0 |  |  | 0.669 |
| ns | 3455 |  | 217 | CI workflow (main.yml) | 4.4 |  | 0.638 |
| walker |  | 3509 | 132 | README.md section #5 |  |  | 0.638 |
| walker |  | 3583 | 74 | plaintext config .gitignore |  |  | 0.639 |
| walker |  | 3610 | 27 | README.md section #6 |  |  | 0.650 |
| ns | 3690 |  | 235 | package.json scripts | 4.5 |  | 0.659 |
| walker |  | 3760 | 150 | plaintext config .editorconfig |  |  | 0.661 |
| ns | 3890 |  | 200 | test-types-compilation.ts preamble — Events type + handler decls | 4.6 |  | 0.639 |
| walker |  | 4002 | 242 | README.md section #2 |  |  | 0.641 |
| walker |  | 4098 | 96 | README.md section #14 |  |  | 0.641 |
| walker |  | 4110 | 12 | listing of 'test' |  |  | 0.655 |
| walker |  | 4162 | 52 | README.md section #16 |  |  | 0.655 |
| walker |  | 4275 | 113 | README.md section #11 |  |  | 0.656 |
| ns | 4287 |  | 397 | test-types-compilation.ts on()/off() blocks | 4.7 | 4.6 | 0.619 |
| walker |  | 4392 | 117 | README.md section #9 |  |  | 0.619 |
| ns | 4485 |  | 198 | test-types-compilation.ts emit() block | 4.8 | 4.6 | 0.602 |
| ns | 4724 |  | 239 | test/index_test.ts imports + outer-block tests | 4.9 | 3.7 | 0.583 |
| walker |  | 4839 | 447 | README.md section #3 |  |  | 0.661 |
| ns | 4955 |  | 231 | test/index_test.ts mitt# Events type + beforeEach | 4.10 | 3.7 | 0.640 |
| ns | 5244 |  | 289 | README install section | 5.1 |  | 0.646 |
| walker |  | 5339 | 500 | plaintext config .eslintrc |  |  | 0.650 |
| walker |  | 5348 | 9 | imports in test/test-types-compilation.ts |  |  | 0.650 |
| ns | 5401 |  | 157 | Test body: wildcard '*' invocation | 5.2 | 3.7 | 0.640 |
| walker |  | 5532 | 184 | README.md section #17 |  |  | 0.641 |
| walker |  | 5592 | 60 | imports in test/index_test.ts |  |  | 0.643 |
| walker |  | 5881 | 289 | plaintext config LICENSE |  |  | 0.643 |
| ns | 5926 |  | 525 | Test bodies: on() registration semantics | 5.3 | 3.7 | 0.607 |
| ns | 6411 |  | 485 | Test bodies: off() removal semantics | 5.4 | 3.7 | 0.580 |
| ns | 6664 |  | 253 | Test bodies: emit() typed dispatch + case sensitivity | 5.5 | 3.7 | 0.568 |
| ns | 7020 |  | 356 | README API parameter tables | 5.6 | 2.7 | 0.571 |
| ns | 7520 |  | 500 | .eslintrc — full | 5.7 |  | 0.600 |
| ns | 8020 |  | 500 | README Examples / Contribute / License sections | 5.8 |  | 0.612 |
| ns | 8386 |  | 366 | package.json devDependencies | 5.9 |  | 0.622 |
| ns | 8625 |  | 239 | .editorconfig + .gitignore | 5.10 |  | 0.630 |
| ns | 8837 |  | 212 | .github/PULL_REQUEST_TEMPLATE.md | 5.11 |  | 0.629 |
| ns | 8863 |  | 26 | LICENSE — MIT preamble | 5.12 |  | 0.630 |
