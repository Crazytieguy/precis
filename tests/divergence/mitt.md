Score(3000)=0.842 I=0.944 C=0.751 ns_rows≤3K=19/47 (reached=14 partial=2 missing=3)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 32 | 32 | listing of '.' |  |  | 1.000 |
| ns | 32 |  | 32 | Root directory listing | 1.1 |  | 1.000 |
| walker |  | 35 | 3 | listing of 'src' |  |  | 1.000 |
| ns | 65 |  | 33 | src/, test/, and .github/ contents | 1.2 |  | 0.714 |
| walker |  | 99 | 64 | package identity in package.json |  |  | 0.715 |
| walker |  | 109 | 10 | listing of '.github' |  |  | 0.765 |
| walker |  | 117 | 8 | listing of '.github/workflows' |  |  | 0.860 |
| ns | 128 |  | 63 | README identity: title, tagline, runtime claims | 1.3 |  | 0.751 |
| ns | 242 |  | 114 | README feature bullets | 1.4 |  | 0.657 |
| walker |  | 334 | 217 | YAML config at .github/workflows/main.yml |  |  | 0.661 |
| ns | 336 |  | 94 | README table of contents | 1.5 |  | 0.559 |
| walker |  | 345 | 11 | listing of 'test' |  |  | 0.650 |
| ns | 425 |  | 89 | src/index.ts export roster (all 8 exports, names only) | 2.1 |  | 0.571 |
| ns | 507 |  | 82 | Type aliases: EventType, Handler, WildcardHandler | 2.2 | 2.1 | 0.507 |
| walker |  | 532 | 187 | README headline in README.md |  |  | 0.663 |
| ns | 619 |  | 112 | Type aliases: EventHandlerList, WildCardEventHandlerList, EventHandlerMap | 2.3 | 2.1 | 0.589 |
| walker |  | 692 | 160 | export names surface in src/index.ts |  |  | 0.735 |
| walker |  | 706 | 14 | export at src/index.ts:13 |  |  | 0.748 |
| walker |  | 729 | 23 | export at src/index.ts:46 |  |  | 0.749 |
| walker |  | 757 | 28 | export at src/index.ts:6 |  |  | 0.779 |
| walker |  | 792 | 35 | export at src/index.ts:18 |  |  | 0.813 |
| ns | 825 |  | 206 | Emitter<Events> interface (full) | 2.4 | 2.1 | 0.704 |
| ns | 903 |  | 78 | mitt() factory: JSDoc + signature | 2.5 | 2.1 | 0.677 |
| walker |  | 989 | 197 | export at src/index.ts:23 |  |  | 0.812 |
| ns | 997 |  | 94 | mitt() body: GenericEventHandler, all init, return-object open | 2.6 | 2.5 | 0.761 |
| walker |  | 1035 | 46 | export doc at src/index.ts:46 |  |  | 0.804 |
| walker |  | 1179 | 144 | headings outline in README.md |  |  | 0.807 |
| ns | 1198 |  | 201 | on() implementation | 2.7 | 2.6 | 0.744 |
| walker |  | 1210 | 31 | README.md section #18 |  |  | 0.745 |
| walker |  | 1248 | 38 | README.md section #15 |  |  | 0.745 |
| walker |  | 1261 | 13 | README.md section #8 |  |  | 0.745 |
| walker |  | 1344 | 83 | README.md section #1 |  |  | 0.806 |
| ns | 1438 |  | 240 | off() implementation | 2.8 | 2.6 | 0.741 |
| walker |  | 1459 | 115 | package identity metadata in package.json |  |  | 0.742 |
| walker |  | 1671 | 212 | package entrypoints in package.json |  |  | 0.750 |
| walker |  | 1690 | 19 | README.md section #13 |  |  | 0.750 |
| ns | 1789 |  | 351 | emit() implementation | 2.9 | 2.6 | 0.660 |
| walker |  | 1923 | 233 | package scripts in package.json |  |  | 0.664 |
| walker |  | 1940 | 17 | README.md section #7 |  |  | 0.664 |
| ns | 2022 |  | 233 | package.json: identity + entry points | 3.1 |  | 0.687 |
| ns | 2257 |  | 235 | package.json: scripts | 3.2 |  | 0.702 |
| ns | 2536 |  | 279 | index_test.ts: imports + top-level mitt() factory tests | 4.1 |  | 0.656 |
| ns | 2735 |  | 199 | index_test.ts: shared Events type + beforeEach instance setup | 4.2 |  | 0.624 |
| ns | 2796 |  | 61 | index_test.ts: properties block | 4.3 |  | 0.616 |
| walker |  | 2838 | 898 | export body at src/index.ts:46 body 49 |  |  | 0.842 |
| walker |  | 2983 | 145 | README.md section #5 |  |  | 0.842 |
| ns | 3097 |  | 301 | index_test.ts: on() - registration and append semantics | 4.4 |  | 0.791 |
| walker |  | 3263 | 280 | README.md section #2 |  |  | 0.793 |
| walker |  | 3293 | 30 | README.md section #12 |  |  | 0.793 |
| walker |  | 3442 | 149 | json config tsconfig.json |  |  | 0.795 |
| ns | 3461 |  | 364 | index_test.ts: on() - case sensitivity, symbols, duplicates | 4.5 |  | 0.750 |
| walker |  | 3475 | 33 | README.md section #10 |  |  | 0.750 |
| walker |  | 3586 | 111 | README.md section #4 |  |  | 0.750 |
| walker |  | 3621 | 35 | README.md section #6 |  |  | 0.750 |
| walker |  | 3676 | 55 | README.md section #16 |  |  | 0.750 |
| walker |  | 3775 | 99 | README.md section #14 |  |  | 0.750 |
| ns | 3806 |  | 345 | index_test.ts: off() - single-handler removal, case sensitivity | 4.6 |  | 0.710 |
| ns | 4069 |  | 263 | index_test.ts: off() - first-match-only and type-wide removal | 4.7 |  | 0.684 |
| walker |  | 4143 | 368 | package dev/peer dependencies in package.json |  |  | 0.687 |
| walker |  | 4259 | 116 | README.md section #11 |  |  | 0.687 |
| walker |  | 4379 | 120 | README.md section #9 |  |  | 0.687 |
| ns | 4407 |  | 338 | index_test.ts: emit() - dispatch and case sensitivity | 4.8 |  | 0.656 |
| ns | 4586 |  | 179 | index_test.ts: emit() - wildcard ('*') handler invocation | 4.9 |  | 0.641 |
| ns | 4788 |  | 202 | test-types-compilation.ts: setup | 4.10 |  | 0.622 |
| walker |  | 4912 | 533 | README.md section #3 |  |  | 0.626 |
| ns | 4984 |  | 196 | test-types-compilation.ts: on() type inference checks | 4.11 |  | 0.611 |
| walker |  | 5032 | 120 | YAML config at .github/workflows/compressed-size.yml |  |  | 0.612 |
| ns | 5180 |  | 196 | test-types-compilation.ts: off() type inference checks | 4.12 |  | 0.597 |
| walker |  | 5224 | 192 | README.md section #17 |  |  | 0.598 |
| walker |  | 5243 | 19 | .github/PULL_REQUEST_TEMPLATE.md section #2 |  |  | 0.598 |
| walker |  | 5259 | 16 | .github/PULL_REQUEST_TEMPLATE.md section #1 |  |  | 0.598 |
| ns | 5376 |  | 196 | test-types-compilation.ts: emit() type inference checks | 4.13 |  | 0.584 |
| ns | 5535 |  | 159 | package.json: repository/keywords/homepage/authors/license/files | 5.1 |  | 0.598 |
| walker |  | 5633 | 374 | test names surface in test/index_test.ts |  |  | 0.604 |
| walker |  | 5661 | 28 | .github/PULL_REQUEST_TEMPLATE.md section #3 |  |  | 0.605 |
| ns | 5675 |  | 140 | package.json: mocha and prettier config blocks | 5.2 |  | 0.593 |
| walker |  | 5771 | 110 | .github/PULL_REQUEST_TEMPLATE.md section #0 |  |  | 0.594 |
| walker |  | 5784 | 13 | imports in test/test-types-compilation.ts |  |  | 0.594 |
| walker |  | 5858 | 74 | plaintext config .gitignore |  |  | 0.595 |
| walker |  | 6023 | 165 | plaintext config .editorconfig |  |  | 0.596 |
| ns | 6044 |  | 369 | package.json: devDependencies (full list) | 5.3 |  | 0.613 |
| ns | 6193 |  | 149 | tsconfig.json (full) | 6.1 |  | 0.623 |
| walker |  | 6332 | 309 | plaintext config LICENSE |  |  | 0.625 |
| ns | 6418 |  | 225 | .eslintrc: ignores, extends, parser, env, globals | 6.2 |  | 0.609 |
| ns | 6693 |  | 275 | .eslintrc: rule overrides | 6.3 |  | 0.592 |
| walker |  | 6832 | 500 | plaintext config .eslintrc |  |  | 0.660 |
| ns | 6858 |  | 165 | .editorconfig (full) | 6.4 |  | 0.668 |
| walker |  | 6892 | 60 | imports in test/index_test.ts |  |  | 0.671 |
| ns | 6932 |  | 74 | .gitignore (full) | 6.5 |  | 0.675 |
| ns | 7149 |  | 217 | CI workflow: main.yml (full) | 6.6 |  | 0.686 |
| ns | 7269 |  | 120 | CI workflow: compressed-size.yml (full) | 6.7 |  | 0.690 |
| ns | 7558 |  | 289 | README: Install section | 7.1 |  | 0.699 |
| ns | 7792 |  | 234 | README: Usage section (core example) | 7.2 |  | 0.706 |
| ns | 8105 |  | 313 | README: TypeScript usage section | 7.3 |  | 0.715 |
| ns | 8216 |  | 111 | README: Examples & Demos | 7.4 |  | 0.717 |
| ns | 8545 |  | 329 | README: Contribute section | 7.5 |  | 0.721 |
| ns | 8757 |  | 212 | PULL_REQUEST_TEMPLATE.md (full) | 7.6 |  | 0.720 |
| ns | 9066 |  | 309 | LICENSE (full) | 7.7 |  | 0.724 |
| ns | 9104 |  | 38 | README: License section | 7.8 |  | 0.725 |
