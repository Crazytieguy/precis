Score(3000)=0.638 I=0.792 C=0.514 ns_rows≤3K=20/53 grid(1000/1442/2080/3000/4327/6240/9000)=0.626/0.738/0.715/0.638/0.608/0.520/0.479

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
| walker |  | 415 | 150 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.796 |
| walker |  | 441 | 26 | Fs::DirListing { dir: benchmarks } |  |  | 0.800 |
| ns | 513 |  | 156 | All top-level (# / ##) README headings with line numbers | 1.6 |  | 0.691 |
| walker |  | 550 | 109 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.691 |
| walker |  | 565 | 15 | Fs::DirListing { dir: examples/gif-fetcher } |  |  | 0.695 |
| walker |  | 588 | 23 | Fs::DirListing { dir: examples/gif-fetcher/src } |  |  | 0.702 |
| ns | 639 |  | 126 | README opening example: the match/with/exhaustive expression itself | 1.7 |  | 0.671 |
| ns | 758 |  | 119 | The discriminated-union types the README's opening example matches on | 1.8 | 1.7 | 0.615 |
| walker |  | 917 | 329 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.624 |
| ns | 1073 |  | 315 | README Features list — the capability inventory | 1.9 |  | 0.643 |
| walker |  | 1215 | 298 | Fs::DirListing { dir: tests } |  |  | 0.672 |
| ns | 1217 |  | 144 | Complete listings of docs/, examples/ (and their subdirs), benchmarks/, scripts/, .github/ | 1.10 |  | 0.662 |
| walker |  | 1476 | 261 | Markdown::Prelude { file: README.md } |  |  | 0.756 |
| ns | 1523 |  | 306 | Complete tests/ listing — the feature-named test-file map | 1.11 |  | 0.764 |
| walker |  | 1533 | 57 | Code::CodeKey { rung: Names, file: src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.812 |
| walker |  | 1681 | 148 | Markdown::HeadingsOutline { file: docs/v3-to-v4-migration-guide.md } |  |  | 0.812 |
| ns | 1751 |  | 228 | README `###` heading locations: Getting Started walkthrough and API Reference | 1.12 |  | 0.762 |
| walker |  | 1831 | 150 | Markdown::HeadingsOutline { file: docs/v4-to-v5-migration-guide.md } |  |  | 0.763 |
| walker |  | 1860 | 29 | Markdown::Section { file: docs/v4-to-v5-migration-guide.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.764 |
| ns | 1965 |  | 214 | README `###` heading locations: the Patterns catalogue | 1.13 |  | 0.728 |
| ns | 2047 |  | 82 | README `###`/`####` heading locations: the Types section | 1.14 |  | 0.714 |
| ns | 2103 |  | 56 | src/patterns.ts module docstring | 2.1 |  | 0.702 |
| walker |  | 2133 | 273 | Json::Scripts { file: package.json } |  |  | 0.704 |
| ns | 2379 |  | 276 | Complete roster of every named export in src/patterns.ts (the `P` namespace) | 2.2 |  | 0.646 |
| ns | 2425 |  | 46 | src/patterns.ts re-export block: `Pattern`, `unstable_Fn`, `matcher` | 2.3 |  | 0.636 |
| walker |  | 2491 | 358 | Json::Entry { file: package.json } |  |  | 0.650 |
| walker |  | 2509 | 18 | Code::CodeKey { rung: Names, file: src/errors.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.650 |
| walker |  | 2526 | 17 | Code::CodeKey { rung: Decl, file: src/errors.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.650 |
| ns | 2555 |  | 130 | `match()` — doc summary, signature and body in src/match.ts | 2.4 |  | 0.638 |
| walker |  | 2566 | 40 | Code::CodeKey { rung: Doc, file: src/errors.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.639 |
| walker |  | 2641 | 75 | Code::CodeKey { rung: Body, file: src/errors.ts, decl: 2, sub: 0, line: 6 } |  |  | 0.641 |
| walker |  | 2645 | 4 | Fs::DirListing { dir: examples/gif-fetcher/public } |  |  | 0.646 |
| walker |  | 2653 | 8 | Fs::DirListing { dir: tests/types-catalog } |  |  | 0.656 |
| walker |  | 2663 | 10 | Code::CodeKey { rung: Names, file: src/types/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.656 |
| ns | 2720 |  | 165 | `isMatching()` — both overload signatures with their doc summaries | 2.5 |  | 0.644 |
| walker |  | 2721 | 58 | Code::CodeKey { rung: Names, file: src/is-matching.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.645 |
| walker |  | 2748 | 27 | Code::CodeKey { rung: Decl, file: src/is-matching.ts, decl: 1, sub: 0, line: 32 } |  |  | 0.648 |
| walker |  | 2781 | 33 | Code::CodeKey { rung: Decl, file: src/is-matching.ts, decl: 3, sub: 0, line: 53 } |  |  | 0.648 |
| walker |  | 2820 | 39 | Code::CodeKey { rung: Decl, file: src/is-matching.ts, decl: 2, sub: 0, line: 48 } |  |  | 0.654 |
| walker |  | 2840 | 20 | Code::CodeKey { rung: Names, file: src/match.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.654 |
| walker |  | 2860 | 20 | Code::CodeKey { rung: Decl, file: src/match.ts, decl: 1, sub: 0, line: 32 } |  |  | 0.656 |
| walker |  | 2876 | 16 | Code::CodeKey { rung: Body, file: src/match.ts, decl: 1, sub: 0, line: 32 } |  |  | 0.658 |
| ns | 2887 |  | 167 | Complete method roster of the `Match<>` builder type (src/types/Match.ts) | 2.6 |  | 0.635 |
| ns | 3042 |  | 155 | `chainable()` — the `.optional()/.and()/.or()/.select()` methods every pattern carries | 2.7 |  | 0.622 |
| walker |  | 3151 | 275 | Code::CodeKey { rung: Names, file: src/patterns.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.642 |
| walker |  | 3176 | 25 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 4, sub: 0, line: 131 } |  |  | 0.642 |
| walker |  | 3219 | 43 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 1, sub: 0, line: 81 } |  |  | 0.642 |
| walker |  | 3268 | 49 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 2, sub: 0, line: 100 } |  |  | 0.642 |
| ns | 3269 |  | 227 | The eight wildcard pattern type aliases in src/types/Pattern.ts | 2.8 |  | 0.625 |
| walker |  | 3328 | 60 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 5, sub: 0, line: 187 } |  |  | 0.626 |
| ns | 3536 |  | 267 | `stringChainable` — all seven `P.string.*` refinement methods | 2.9 |  | 0.606 |
| walker |  | 3591 | 263 | Code::CodeKey { rung: Names, file: src/patterns.ts, decl: 0, sub: 1, line: 0 } |  |  | 0.626 |
| walker |  | 3619 | 28 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 8, sub: 0, line: 246 } |  |  | 0.626 |
| walker |  | 3648 | 29 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 22, sub: 0, line: 643 } |  |  | 0.626 |
| walker |  | 3684 | 36 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 17, sub: 0, line: 445 } |  |  | 0.626 |
| walker |  | 3726 | 42 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 20, sub: 0, line: 611 } |  |  | 0.626 |
| walker |  | 3769 | 43 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 7, sub: 0, line: 242 } |  |  | 0.626 |
| walker |  | 3812 | 43 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 10, sub: 0, line: 295 } |  |  | 0.626 |
| ns | 3821 |  | 285 | `numberChainable` — all nine `P.number.*` refinement methods | 2.10 |  | 0.610 |
| walker |  | 3857 | 45 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 15, sub: 0, line: 433 } |  |  | 0.611 |
| walker |  | 3902 | 45 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 18, sub: 0, line: 536 } |  |  | 0.611 |
| walker |  | 3947 | 45 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 19, sub: 0, line: 572 } |  |  | 0.611 |
| walker |  | 3996 | 49 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 21, sub: 0, line: 637 } |  |  | 0.613 |
| walker |  | 4046 | 50 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 11, sub: 0, line: 299 } |  |  | 0.613 |
| ns | 4095 |  | 274 | `bigintChainable` — all seven `P.bigint.*` refinement methods | 2.11 |  | 0.598 |
| walker |  | 4098 | 52 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 23, sub: 0, line: 646 } |  |  | 0.598 |
| walker |  | 4176 | 78 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 13, sub: 0, line: 357 } |  |  | 0.598 |
| ns | 4249 |  | 154 | src/errors.ts in full — `NonExhaustiveError` | 2.12 |  | 0.608 |
| walker |  | 4270 | 94 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 16, sub: 0, line: 437 } |  |  | 0.608 |
| walker |  | 4365 | 95 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 14, sub: 0, line: 362 } |  |  | 0.608 |
| ns | 4436 |  | 187 | src/internals/symbols.ts — the five protocol symbols in full | 3.1 |  | 0.594 |
| walker |  | 4464 | 99 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 26, sub: 0, line: 686 } |  |  | 0.594 |
| walker |  | 4614 | 150 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 25, sub: 0, line: 673 } |  |  | 0.594 |
| ns | 4658 |  | 222 | src/internals/helpers.ts — the three pattern predicates | 3.2 |  | 0.579 |
| ns | 4864 |  | 206 | `matchPattern` signature and the Matcher-Protocol dispatch branch | 3.3 |  | 0.565 |
| walker |  | 4883 | 269 | Code::CodeKey { rung: Names, file: src/patterns.ts, decl: 0, sub: 2, line: 0 } |  |  | 0.601 |
| walker |  | 4913 | 30 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 38, sub: 0, line: 1251 } |  |  | 0.601 |
| walker |  | 4943 | 30 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 39, sub: 0, line: 1271 } |  |  | 0.603 |
| ns | 4981 |  | 117 | `matchPattern`'s object-key branch and the primitive fallback | 3.4 |  | 0.594 |
| walker |  | 4987 | 44 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 27, sub: 0, line: 696 } |  |  | 0.594 |
| walker |  | 5094 | 107 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 35, sub: 0, line: 1221 } |  |  | 0.594 |
| ns | 5188 |  | 207 | `getSelectionKeys` and `flatMap` in full — closing src/internals/helpers.ts | 3.5 |  | 0.581 |
| walker |  | 5203 | 109 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 28, sub: 0, line: 782 } |  |  | 0.581 |
| walker |  | 5312 | 109 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 34, sub: 0, line: 1211 } |  |  | 0.581 |
| ns | 5416 |  | 228 | The `MatchExpression` class and its complete method roster | 3.6 |  | 0.564 |
| walker |  | 5422 | 110 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 29, sub: 0, line: 793 } |  |  | 0.564 |
| walker |  | 5533 | 111 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 31, sub: 0, line: 928 } |  |  | 0.564 |
| ns | 5635 |  | 219 | `isMatching`'s runtime implementation — the arity dispatch | 3.7 |  | 0.554 |
| walker |  | 5647 | 114 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 33, sub: 0, line: 1201 } |  |  | 0.554 |
| walker |  | 5764 | 117 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 38, sub: 0, line: 1251 } |  |  | 0.554 |
| walker |  | 5882 | 118 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 36, sub: 0, line: 1231 } |  |  | 0.554 |
| walker |  | 6002 | 120 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 30, sub: 0, line: 805 } |  |  | 0.554 |
| ns | 6030 |  | 395 | `MatchExpression.with()` — multi-pattern, guard and selection semantics | 3.8 | 3.6 | 0.534 |
| walker |  | 6122 | 120 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 32, sub: 0, line: 1075 } |  |  | 0.534 |
| ns | 6201 |  | 171 | `otherwise`, `exhaustive`, `run`, `returnType` and `narrow` bodies, and `defaultCatcher` | 3.9 | 3.6 | 0.520 |
| walker |  | 6242 | 120 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 37, sub: 0, line: 1241 } |  |  | 0.520 |
| walker |  | 6363 | 121 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 3, sub: 0, line: 116 } |  |  | 0.520 |
| ns | 6370 |  | 169 | The observable rules of `matchPattern`'s tuple/variadic branch | 3.10 | 3.3 | 0.512 |
| ns | 6480 |  | 110 | `MatcherType` — the complete closed set of matcher kinds | 4.1 |  | 0.506 |
| walker |  | 6494 | 131 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 9, sub: 0, line: 294 } |  |  | 0.506 |
| walker |  | 6626 | 132 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 2, sub: 0, line: 100 } |  |  | 0.506 |
| ns | 6710 |  | 230 | `MatcherProtocol` and `MatchResult` — the contract a custom pattern implements | 4.2 |  | 0.496 |
| walker |  | 6768 | 142 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 6, sub: 0, line: 241 } |  |  | 0.496 |
| ns | 6867 |  | 157 | The `Matcher` interface and its `[symbols.isVariadic]` flag | 4.3 |  | 0.489 |
| walker |  | 6911 | 143 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 24, sub: 0, line: 672 } |  |  | 0.489 |
| walker |  | 7055 | 144 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 20, sub: 0, line: 611 } |  |  | 0.489 |
| walker |  | 7201 | 146 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 5, sub: 0, line: 187 } |  |  | 0.489 |
| ns | 7263 |  | 396 | Complete roster of the matcher-alias types (`ArrayP`, `SetP`, `SelectP`, …) | 4.4 |  | 0.476 |
| walker |  | 7354 | 153 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 1, sub: 0, line: 81 } |  |  | 0.476 |
| walker |  | 7508 | 154 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 21, sub: 0, line: 637 } |  |  | 0.476 |
| ns | 7632 |  | 369 | `Pattern<a>` itself — what shapes are legal as a pattern for a given type | 4.5 |  | 0.464 |
| walker |  | 7679 | 171 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 15, sub: 0, line: 433 } |  |  | 0.464 |
| walker |  | 7853 | 174 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 12, sub: 0, line: 356 } |  |  | 0.464 |
| ns | 8004 |  | 372 | Headline export of every remaining src/types/ module, with its purpose line | 4.6 |  | 0.456 |
| walker |  | 8028 | 175 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 19, sub: 0, line: 572 } |  |  | 0.456 |
| walker |  | 8220 | 192 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 4, sub: 0, line: 131 } |  |  | 0.456 |
| ns | 8277 |  | 273 | package.json scripts — the complete command set for this repo | 5.1 |  | 0.466 |
| walker |  | 8413 | 193 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 39, sub: 0, line: 1271 } |  |  | 0.466 |
| ns | 8453 |  | 176 | jest.config.cjs and tests/tsconfig.json in full — the test harness | 5.2 |  | 0.460 |
| ns | 8607 |  | 154 | tsconfig.json in full — the library's compiler settings | 5.3 |  | 0.455 |
| walker |  | 8645 | 232 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 18, sub: 0, line: 536 } |  |  | 0.455 |
| walker |  | 8769 | 124 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.456 |
| walker |  | 8779 | 10 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.461 |
| ns | 8853 |  | 246 | package.json `exports` map — the dual ESM/CJS and `./types` subpaths | 5.4 |  | 0.478 |
| walker |  | 8927 | 148 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.479 |
| ns | 9028 |  | 175 | docs/roadmap.md — the unimplemented items | 5.5 |  | 0.475 |
| walker |  | 9073 | 146 | Code::CodeKey { rung: Names, file: src/types/FindSelected.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.475 |
| walker |  | 9086 | 13 | Code::CodeKey { rung: Decl, file: src/types/FindSelected.ts, decl: 1, sub: 0, line: 13 } |  |  | 0.475 |
| walker |  | 9108 | 22 | Code::CodeKey { rung: Decl, file: src/types/FindSelected.ts, decl: 2, sub: 0, line: 16 } |  |  | 0.475 |
| walker |  | 9155 | 47 | Code::CodeKey { rung: Decl, file: src/types/FindSelected.ts, decl: 8, sub: 0, line: 185 } |  |  | 0.475 |
| ns | 9178 |  | 150 | Heading locations in the v4-to-v5 migration guide | 5.6 |  | 0.482 |
| walker |  | 9216 | 61 | Code::CodeKey { rung: Decl, file: src/types/FindSelected.ts, decl: 6, sub: 0, line: 165 } |  |  | 0.482 |
| walker |  | 9284 | 68 | Code::CodeKey { rung: Decl, file: src/types/FindSelected.ts, decl: 9, sub: 0, line: 191 } |  |  | 0.482 |
| ns | 9307 |  | 129 | The internal type-guard predicates behind every wildcard in src/patterns.ts | 5.7 |  | 0.476 |
| walker |  | 9353 | 69 | Code::CodeKey { rung: Decl, file: src/types/FindSelected.ts, decl: 5, sub: 0, line: 159 } |  |  | 0.476 |
| ns | 9463 |  | 156 | tests/types-catalog/utils.ts — the shared fixture types used across the suite | 5.8 |  | 0.472 |
| walker |  | 9499 | 146 | Code::CodeKey { rung: Decl, file: src/types/FindSelected.ts, decl: 7, sub: 0, line: 174 } |  |  | 0.472 |
| walker |  | 9582 | 83 | Code::CodeKey { rung: Names, file: src/internals/helpers.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.472 |
| walker |  | 9596 | 14 | Code::CodeKey { rung: Decl, file: src/internals/helpers.ts, decl: 1, sub: 0, line: 12 } |  |  | 0.473 |
| ns | 9622 |  | 159 | examples/one-file-demo — the demo's table of contents | 5.9 |  | 0.470 |
| walker |  | 9626 | 30 | Code::CodeKey { rung: Decl, file: src/internals/helpers.ts, decl: 2, sub: 0, line: 16 } |  |  | 0.471 |
| walker |  | 9671 | 45 | Code::CodeKey { rung: Decl, file: src/internals/helpers.ts, decl: 3, sub: 0, line: 32 } |  |  | 0.472 |
| walker |  | 9719 | 48 | Code::CodeKey { rung: Decl, file: src/internals/helpers.ts, decl: 5, sub: 0, line: 132 } |  |  | 0.474 |
| walker |  | 9756 | 37 | Code::CodeKey { rung: Body, file: src/internals/helpers.ts, decl: 2, sub: 0, line: 16 } |  |  | 0.476 |
| ns | 9843 |  | 221 | jsr.json in full, and package.json's devDependencies | 5.10 |  | 0.471 |
| walker |  | 9923 | 167 | Code::CodeKey { rung: Names, file: src/internals/symbols.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.482 |
| ns | 9964 |  | 121 | `.prettierrc` in full, and the benchmark runner scripts | 5.11 |  | 0.479 |
| walker |  | 9993 | 70 | Code::CodeKey { rung: Doc, file: src/is-matching.ts, decl: 2, sub: 0, line: 48 } |  |  | 0.481 |
