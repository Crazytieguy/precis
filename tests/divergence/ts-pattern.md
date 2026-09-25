Score(3000)=0.675 I=0.803 C=0.567 ns_rows≤3K=20/53 grid(1000/1442/2080/3000/4327/6240/9000)=0.672/0.837/0.770/0.675/0.603/0.547/0.534

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 48 | 48 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 60 |  | 60 | README title and one-line description of the library | 1.1 |  | 0.000 |
| walker |  | 74 | 26 | Fs::DirListing { dir: docs } |  |  | 0.000 |
| walker |  | 103 | 29 | Fs::DirListing { dir: src } |  |  | 0.000 |
| walker |  | 111 | 8 | Fs::DirListing { dir: src/internals } |  |  | 0.000 |
| walker |  | 117 | 6 | Fs::DirListing { dir: scripts } |  |  | 0.000 |
| ns | 132 |  | 72 | src/index.ts in full — the complete public export surface | 1.2 |  | 0.000 |
| ns | 180 |  | 48 | Complete repository root listing | 1.3 |  | 0.435 |
| walker |  | 190 | 73 | Json::Identity { file: package.json } |  |  | 0.447 |
| walker |  | 246 | 56 | Fs::DirListing { dir: src/types } |  |  | 0.523 |
| walker |  | 255 | 9 | Fs::DirListing { dir: .github } |  |  | 0.525 |
| walker |  | 265 | 10 | Fs::DirListing { dir: examples } |  |  | 0.526 |
| ns | 273 |  | 93 | Complete listings of src/, src/internals/ and src/types/ | 1.4 |  | 0.548 |
| ns | 357 |  | 84 | package.json identity: name, version, description, module type, entry source | 1.5 |  | 0.528 |
| walker |  | 395 | 130 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.796 |
| walker |  | 504 | 109 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.796 |
| ns | 513 |  | 156 | All top-level (# / ##) README headings with line numbers | 1.6 |  | 0.688 |
| walker |  | 514 | 10 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.689 |
| walker |  | 540 | 26 | Fs::DirListing { dir: benchmarks } |  |  | 0.693 |
| ns | 639 |  | 126 | README opening example: the match/with/exhaustive expression itself | 1.7 |  | 0.663 |
| ns | 758 |  | 119 | The discriminated-union types the README's opening example matches on | 1.8 | 1.7 | 0.607 |
| walker |  | 869 | 329 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.616 |
| walker |  | 884 | 15 | Fs::DirListing { dir: examples/gif-fetcher } |  |  | 0.620 |
| walker |  | 899 | 15 | Fs::DirListing { dir: examples/one-file-demo } |  |  | 0.624 |
| ns | 1073 |  | 315 | README Features list — the capability inventory | 1.9 |  | 0.643 |
| walker |  | 1160 | 261 | Markdown::Prelude { file: README.md } |  |  | 0.760 |
| walker |  | 1170 | 10 | Code::CodeKey { rung: Names, file: src/types/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.760 |
| walker |  | 1193 | 23 | Fs::DirListing { dir: examples/gif-fetcher/src } |  |  | 0.768 |
| ns | 1217 |  | 144 | Complete listings of docs/, examples/ (and their subdirs), benchmarks/, scripts/, .github/ | 1.10 |  | 0.771 |
| walker |  | 1250 | 57 | Code::CodeKey { rung: Names, file: src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.825 |
| ns | 1523 |  | 306 | Complete tests/ listing — the feature-named test-file map | 1.11 |  | 0.664 |
| walker |  | 1548 | 298 | Fs::DirListing { dir: tests } |  |  | 0.845 |
| walker |  | 1556 | 8 | Fs::DirListing { dir: tests/types-catalog } |  |  | 0.859 |
| ns | 1751 |  | 228 | README `###` heading locations: Getting Started walkthrough and API Reference | 1.12 |  | 0.807 |
| walker |  | 1829 | 273 | Json::Scripts { file: package.json } |  |  | 0.808 |
| ns | 1965 |  | 214 | README `###` heading locations: the Patterns catalogue | 1.13 |  | 0.771 |
| ns | 2047 |  | 82 | README `###`/`####` heading locations: the Types section | 1.14 |  | 0.756 |
| ns | 2103 |  | 56 | src/patterns.ts module docstring | 2.1 |  | 0.743 |
| walker |  | 2187 | 358 | Json::Entry { file: package.json } |  |  | 0.758 |
| walker |  | 2311 | 124 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.760 |
| ns | 2379 |  | 276 | Complete roster of every named export in src/patterns.ts (the `P` namespace) | 2.2 |  | 0.699 |
| ns | 2425 |  | 46 | src/patterns.ts re-export block: `Pattern`, `unstable_Fn`, `matcher` | 2.3 |  | 0.687 |
| walker |  | 2459 | 148 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.689 |
| ns | 2555 |  | 130 | `match()` — doc summary, signature and body in src/match.ts | 2.4 |  | 0.677 |
| walker |  | 2684 | 225 | Code::CodeKey { rung: Names, file: src/patterns.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.695 |
| ns | 2720 |  | 165 | `isMatching()` — both overload signatures with their doc summaries | 2.5 |  | 0.682 |
| walker |  | 2727 | 43 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 1, sub: 0, line: 81 } |  |  | 0.682 |
| walker |  | 2776 | 49 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 2, sub: 0, line: 100 } |  |  | 0.682 |
| ns | 2887 |  | 167 | Complete method roster of the `Match<>` builder type (src/types/Match.ts) | 2.6 |  | 0.659 |
| walker |  | 3004 | 228 | Code::CodeKey { rung: Names, file: src/patterns.ts, decl: 0, sub: 1, line: 0 } |  |  | 0.678 |
| walker |  | 3029 | 25 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 4, sub: 0, line: 131 } |  |  | 0.678 |
| ns | 3042 |  | 155 | `chainable()` — the `.optional()/.and()/.or()/.select()` methods every pattern carries | 2.7 |  | 0.663 |
| walker |  | 3057 | 28 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 8, sub: 0, line: 246 } |  |  | 0.663 |
| walker |  | 3093 | 36 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 17, sub: 0, line: 445 } |  |  | 0.663 |
| walker |  | 3135 | 42 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 20, sub: 0, line: 611 } |  |  | 0.663 |
| walker |  | 3178 | 43 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 7, sub: 0, line: 242 } |  |  | 0.663 |
| walker |  | 3221 | 43 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 10, sub: 0, line: 295 } |  |  | 0.663 |
| walker |  | 3266 | 45 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 15, sub: 0, line: 433 } |  |  | 0.664 |
| ns | 3269 |  | 227 | The eight wildcard pattern type aliases in src/types/Pattern.ts | 2.8 |  | 0.647 |
| walker |  | 3311 | 45 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 18, sub: 0, line: 536 } |  |  | 0.647 |
| walker |  | 3356 | 45 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 19, sub: 0, line: 572 } |  |  | 0.647 |
| walker |  | 3406 | 50 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 11, sub: 0, line: 299 } |  |  | 0.647 |
| walker |  | 3457 | 51 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 21, sub: 0, line: 637 } |  |  | 0.649 |
| walker |  | 3517 | 60 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 5, sub: 0, line: 187 } |  |  | 0.650 |
| ns | 3536 |  | 267 | `stringChainable` — all seven `P.string.*` refinement methods | 2.9 |  | 0.629 |
| walker |  | 3595 | 78 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 13, sub: 0, line: 357 } |  |  | 0.629 |
| walker |  | 3689 | 94 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 16, sub: 0, line: 437 } |  |  | 0.629 |
| walker |  | 3784 | 95 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 14, sub: 0, line: 362 } |  |  | 0.629 |
| ns | 3821 |  | 285 | `numberChainable` — all nine `P.number.*` refinement methods | 2.10 |  | 0.614 |
| walker |  | 3972 | 188 | Code::CodeKey { rung: Names, file: src/patterns.ts, decl: 0, sub: 2, line: 0 } |  |  | 0.632 |
| walker |  | 4001 | 29 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 22, sub: 0, line: 643 } |  |  | 0.632 |
| walker |  | 4045 | 44 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 27, sub: 0, line: 696 } |  |  | 0.632 |
| ns | 4095 |  | 274 | `bigintChainable` — all seven `P.bigint.*` refinement methods | 2.11 |  | 0.617 |
| walker |  | 4097 | 52 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 23, sub: 0, line: 646 } |  |  | 0.617 |
| walker |  | 4194 | 97 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 26, sub: 0, line: 686 } |  |  | 0.617 |
| ns | 4249 |  | 154 | src/errors.ts in full — `NonExhaustiveError` | 2.12 |  | 0.603 |
| walker |  | 4344 | 150 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 25, sub: 0, line: 673 } |  |  | 0.603 |
| ns | 4436 |  | 187 | src/internals/symbols.ts — the five protocol symbols in full | 3.1 |  | 0.589 |
| walker |  | 4510 | 166 | Code::CodeKey { rung: Names, file: src/patterns.ts, decl: 0, sub: 3, line: 0 } |  |  | 0.616 |
| walker |  | 4540 | 30 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 38, sub: 0, line: 1251 } |  |  | 0.616 |
| walker |  | 4570 | 30 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 39, sub: 0, line: 1271 } |  |  | 0.618 |
| walker |  | 4588 | 18 | Code::CodeKey { rung: Names, file: src/errors.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.618 |
| walker |  | 4605 | 17 | Code::CodeKey { rung: Decl, file: src/errors.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.619 |
| ns | 4658 |  | 222 | src/internals/helpers.ts — the three pattern predicates | 3.2 |  | 0.603 |
| walker |  | 4663 | 58 | Code::CodeKey { rung: Names, file: src/is-matching.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.604 |
| walker |  | 4690 | 27 | Code::CodeKey { rung: Decl, file: src/is-matching.ts, decl: 1, sub: 0, line: 32 } |  |  | 0.606 |
| walker |  | 4723 | 33 | Code::CodeKey { rung: Decl, file: src/is-matching.ts, decl: 3, sub: 0, line: 53 } |  |  | 0.606 |
| walker |  | 4762 | 39 | Code::CodeKey { rung: Decl, file: src/is-matching.ts, decl: 2, sub: 0, line: 48 } |  |  | 0.610 |
| walker |  | 4782 | 20 | Code::CodeKey { rung: Names, file: src/match.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.611 |
| walker |  | 4802 | 20 | Code::CodeKey { rung: Decl, file: src/match.ts, decl: 1, sub: 0, line: 32 } |  |  | 0.612 |
| walker |  | 4818 | 16 | Code::CodeKey { rung: Body, file: src/match.ts, decl: 1, sub: 0, line: 32 } |  |  | 0.613 |
| walker |  | 4822 | 4 | Fs::DirListing { dir: examples/gif-fetcher/public } |  |  | 0.617 |
| ns | 4864 |  | 206 | `matchPattern` signature and the Matcher-Protocol dispatch branch | 3.3 |  | 0.602 |
| ns | 4981 |  | 117 | `matchPattern`'s object-key branch and the primitive fallback | 3.4 |  | 0.592 |
| walker |  | 5052 | 230 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.594 |
| ns | 5188 |  | 207 | `getSelectionKeys` and `flatMap` in full — closing src/internals/helpers.ts | 3.5 |  | 0.581 |
| walker |  | 5198 | 146 | Code::CodeKey { rung: Names, file: src/types/FindSelected.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.581 |
| walker |  | 5211 | 13 | Code::CodeKey { rung: Decl, file: src/types/FindSelected.ts, decl: 1, sub: 0, line: 13 } |  |  | 0.581 |
| walker |  | 5233 | 22 | Code::CodeKey { rung: Decl, file: src/types/FindSelected.ts, decl: 2, sub: 0, line: 16 } |  |  | 0.581 |
| walker |  | 5280 | 47 | Code::CodeKey { rung: Decl, file: src/types/FindSelected.ts, decl: 8, sub: 0, line: 185 } |  |  | 0.581 |
| walker |  | 5341 | 61 | Code::CodeKey { rung: Decl, file: src/types/FindSelected.ts, decl: 6, sub: 0, line: 165 } |  |  | 0.581 |
| walker |  | 5409 | 68 | Code::CodeKey { rung: Decl, file: src/types/FindSelected.ts, decl: 9, sub: 0, line: 191 } |  |  | 0.581 |
| ns | 5416 |  | 228 | The `MatchExpression` class and its complete method roster | 3.6 |  | 0.564 |
| walker |  | 5478 | 69 | Code::CodeKey { rung: Decl, file: src/types/FindSelected.ts, decl: 5, sub: 0, line: 159 } |  |  | 0.564 |
| walker |  | 5624 | 146 | Code::CodeKey { rung: Decl, file: src/types/FindSelected.ts, decl: 7, sub: 0, line: 174 } |  |  | 0.564 |
| ns | 5635 |  | 219 | `isMatching`'s runtime implementation — the arity dispatch | 3.7 |  | 0.554 |
| walker |  | 5664 | 40 | Code::CodeKey { rung: Doc, file: src/errors.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.558 |
| walker |  | 5747 | 83 | Code::CodeKey { rung: Names, file: src/internals/helpers.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.559 |
| walker |  | 5761 | 14 | Code::CodeKey { rung: Decl, file: src/internals/helpers.ts, decl: 1, sub: 0, line: 12 } |  |  | 0.560 |
| walker |  | 5791 | 30 | Code::CodeKey { rung: Decl, file: src/internals/helpers.ts, decl: 2, sub: 0, line: 16 } |  |  | 0.562 |
| walker |  | 5836 | 45 | Code::CodeKey { rung: Decl, file: src/internals/helpers.ts, decl: 3, sub: 0, line: 32 } |  |  | 0.563 |
| walker |  | 5884 | 48 | Code::CodeKey { rung: Decl, file: src/internals/helpers.ts, decl: 5, sub: 0, line: 132 } |  |  | 0.566 |
| ns | 6030 |  | 395 | `MatchExpression.with()` — multi-pattern, guard and selection semantics | 3.8 | 3.6 | 0.545 |
| walker |  | 6051 | 167 | Code::CodeKey { rung: Names, file: src/internals/symbols.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.562 |
| walker |  | 6144 | 93 | Json::Whole { file: jsr.json } |  |  | 0.562 |
| ns | 6201 |  | 171 | `otherwise`, `exhaustive`, `run`, `returnType` and `narrow` bodies, and `defaultCatcher` | 3.9 | 3.6 | 0.547 |
| walker |  | 6236 | 92 | Code::CodeKey { rung: Names, file: src/types/DistributeUnions.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.547 |
| walker |  | 6266 | 30 | Code::CodeKey { rung: Decl, file: src/types/DistributeUnions.ts, decl: 1, sub: 0, line: 41 } |  |  | 0.547 |
| walker |  | 6361 | 95 | Code::CodeKey { rung: Decl, file: src/types/DistributeUnions.ts, decl: 4, sub: 0, line: 174 } |  |  | 0.547 |
| ns | 6370 |  | 169 | The observable rules of `matchPattern`'s tuple/variadic branch | 3.10 | 3.3 | 0.539 |
| ns | 6480 |  | 110 | `MatcherType` — the complete closed set of matcher kinds | 4.1 |  | 0.533 |
| walker |  | 6490 | 129 | Code::CodeKey { rung: Decl, file: src/types/DistributeUnions.ts, decl: 5, sub: 0, line: 183 } |  |  | 0.533 |
| walker |  | 6620 | 130 | Code::CodeKey { rung: Decl, file: src/types/DistributeUnions.ts, decl: 2, sub: 0, line: 46 } |  |  | 0.533 |
| ns | 6710 |  | 230 | `MatcherProtocol` and `MatchResult` — the contract a custom pattern implements | 4.2 |  | 0.522 |
| walker |  | 6727 | 107 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 35, sub: 0, line: 1221 } |  |  | 0.522 |
| walker |  | 6836 | 109 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 28, sub: 0, line: 782 } |  |  | 0.522 |
| ns | 6867 |  | 157 | The `Matcher` interface and its `[symbols.isVariadic]` flag | 4.3 |  | 0.515 |
| walker |  | 6945 | 109 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 34, sub: 0, line: 1211 } |  |  | 0.515 |
| walker |  | 6983 | 38 | Code::CodeKey { rung: Names, file: src/types/Match.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.515 |
| walker |  | 7093 | 110 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 29, sub: 0, line: 793 } |  |  | 0.515 |
| walker |  | 7204 | 111 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 31, sub: 0, line: 928 } |  |  | 0.515 |
| ns | 7263 |  | 396 | Complete roster of the matcher-alias types (`ArrayP`, `SetP`, `SelectP`, …) | 4.4 |  | 0.502 |
| walker |  | 7318 | 114 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 33, sub: 0, line: 1201 } |  |  | 0.502 |
| walker |  | 7435 | 117 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 38, sub: 0, line: 1251 } |  |  | 0.502 |
| walker |  | 7553 | 118 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 36, sub: 0, line: 1231 } |  |  | 0.502 |
| ns | 7632 |  | 369 | `Pattern<a>` itself — what shapes are legal as a pattern for a given type | 4.5 |  | 0.489 |
| walker |  | 7635 | 82 | Code::CodeKey { rung: Names, file: src/types/InvertPattern.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.489 |
| walker |  | 7657 | 22 | Code::CodeKey { rung: Decl, file: src/types/InvertPattern.ts, decl: 1, sub: 0, line: 106 } |  |  | 0.489 |
| walker |  | 7681 | 24 | Code::CodeKey { rung: Decl, file: src/types/InvertPattern.ts, decl: 4, sub: 0, line: 303 } |  |  | 0.489 |
| walker |  | 7780 | 99 | Code::CodeKey { rung: Decl, file: src/types/InvertPattern.ts, decl: 2, sub: 0, line: 180 } |  |  | 0.489 |
| walker |  | 7881 | 101 | Code::CodeKey { rung: Decl, file: src/types/InvertPattern.ts, decl: 3, sub: 0, line: 192 } |  |  | 0.489 |
| walker |  | 7903 | 22 | Code::CodeKey { rung: Doc, file: src/types/InvertPattern.ts, decl: 4, sub: 0, line: 303 } |  |  | 0.489 |
| ns | 8004 |  | 372 | Headline export of every remaining src/types/ module, with its purpose line | 4.6 |  | 0.484 |
| walker |  | 8023 | 120 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 30, sub: 0, line: 805 } |  |  | 0.484 |
| walker |  | 8143 | 120 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 32, sub: 0, line: 1075 } |  |  | 0.484 |
| walker |  | 8263 | 120 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 37, sub: 0, line: 1241 } |  |  | 0.484 |
| ns | 8277 |  | 273 | package.json scripts — the complete command set for this repo | 5.1 |  | 0.493 |
| ns | 8453 |  | 176 | jest.config.cjs and tests/tsconfig.json in full — the test harness | 5.2 |  | 0.487 |
| walker |  | 8480 | 217 | Code::CodeKey { rung: Names, file: src/types/Pattern.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.489 |
| walker |  | 8504 | 24 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 5, sub: 0, line: 74 } |  |  | 0.489 |
| walker |  | 8530 | 26 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 3, sub: 0, line: 38 } |  |  | 0.489 |
| walker |  | 8560 | 30 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 9, sub: 0, line: 96 } |  |  | 0.490 |
| ns | 8607 |  | 154 | tsconfig.json in full — the library's compiler settings | 5.3 |  | 0.485 |
| walker |  | 8643 | 83 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 7, sub: 0, line: 83 } |  |  | 0.485 |
| walker |  | 8742 | 99 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 1, sub: 0, line: 7 } |  |  | 0.498 |
| ns | 8853 |  | 246 | package.json `exports` map — the dual ESM/CJS and `./types` subpaths | 5.4 |  | 0.512 |
| walker |  | 8884 | 142 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 2, sub: 0, line: 23 } |  |  | 0.525 |
| ns | 9028 |  | 175 | docs/roadmap.md — the unimplemented items | 5.5 |  | 0.521 |
| walker |  | 9072 | 188 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 4, sub: 0, line: 50 } |  |  | 0.535 |
| ns | 9178 |  | 150 | Heading locations in the v4-to-v5 migration guide | 5.6 |  | 0.530 |
| walker |  | 9253 | 181 | Code::CodeKey { rung: Names, file: src/types/Pattern.ts, decl: 0, sub: 1, line: 0 } |  |  | 0.538 |
| walker |  | 9292 | 39 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 17, sub: 0, line: 116 } |  |  | 0.539 |
| ns | 9307 |  | 129 | The internal type-guard predicates behind every wildcard in src/patterns.ts | 5.7 |  | 0.532 |
| walker |  | 9339 | 47 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 18, sub: 0, line: 124 } |  |  | 0.533 |
| ns | 9463 |  | 156 | tests/types-catalog/utils.ts — the shared fixture types used across the suite | 5.8 |  | 0.528 |
| walker |  | 9515 | 176 | Code::CodeKey { rung: Names, file: src/types/Pattern.ts, decl: 0, sub: 2, line: 0 } |  |  | 0.535 |
| walker |  | 9529 | 14 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 20, sub: 0, line: 132 } |  |  | 0.537 |
| walker |  | 9549 | 20 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 23, sub: 0, line: 157 } |  |  | 0.537 |
| walker |  | 9610 | 61 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 22, sub: 0, line: 138 } |  |  | 0.545 |
| ns | 9622 |  | 159 | examples/one-file-demo — the demo's table of contents | 5.9 |  | 0.541 |
| walker |  | 9783 | 173 | Code::CodeKey { rung: Names, file: src/types/Pattern.ts, decl: 0, sub: 3, line: 0 } |  |  | 0.545 |
| walker |  | 9807 | 24 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 30, sub: 0, line: 196 } |  |  | 0.548 |
| ns | 9843 |  | 221 | jsr.json in full, and package.json's devDependencies | 5.10 |  | 0.545 |
| ns | 9964 |  | 121 | `.prettierrc` in full, and the benchmark runner scripts | 5.11 |  | 0.542 |
| walker |  | 9995 | 188 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 37, sub: 0, line: 645 } |  |  | 0.542 |
