Score(3000)=0.633 I=0.783 C=0.512 ns_rows≤3K=20/53 grid(1000/1442/2080/3000/4327/6240/9000)=0.666/0.748/0.729/0.633/0.578/0.482/0.480

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
| ns | 513 |  | 156 | All top-level (# / ##) README headings with line numbers | 1.6 |  | 0.688 |
| walker |  | 524 | 109 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.688 |
| walker |  | 550 | 26 | Fs::DirListing { dir: benchmarks } |  |  | 0.691 |
| walker |  | 565 | 15 | Fs::DirListing { dir: examples/gif-fetcher } |  |  | 0.695 |
| ns | 639 |  | 126 | README opening example: the match/with/exhaustive expression itself | 1.7 |  | 0.665 |
| ns | 758 |  | 119 | The discriminated-union types the README's opening example matches on | 1.8 | 1.7 | 0.609 |
| walker |  | 894 | 329 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.618 |
| ns | 1073 |  | 315 | README Features list — the capability inventory | 1.9 |  | 0.637 |
| walker |  | 1155 | 261 | Markdown::Prelude { file: README.md } |  |  | 0.753 |
| walker |  | 1178 | 23 | Fs::DirListing { dir: examples/gif-fetcher/src } |  |  | 0.760 |
| walker |  | 1188 | 10 | Code::CodeKey { rung: Names, file: src/types/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.760 |
| ns | 1217 |  | 144 | Complete listings of docs/, examples/ (and their subdirs), benchmarks/, scripts/, .github/ | 1.10 |  | 0.725 |
| walker |  | 1486 | 298 | Fs::DirListing { dir: tests } |  |  | 0.756 |
| ns | 1523 |  | 306 | Complete tests/ listing — the feature-named test-file map | 1.11 |  | 0.764 |
| walker |  | 1543 | 57 | Code::CodeKey { rung: Names, file: src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.812 |
| ns | 1751 |  | 228 | README `###` heading locations: Getting Started walkthrough and API Reference | 1.12 |  | 0.762 |
| walker |  | 1816 | 273 | Json::Scripts { file: package.json } |  |  | 0.764 |
| ns | 1965 |  | 214 | README `###` heading locations: the Patterns catalogue | 1.13 |  | 0.728 |
| ns | 2047 |  | 82 | README `###`/`####` heading locations: the Types section | 1.14 |  | 0.714 |
| ns | 2103 |  | 56 | src/patterns.ts module docstring | 2.1 |  | 0.703 |
| walker |  | 2174 | 358 | Json::Entry { file: package.json } |  |  | 0.718 |
| walker |  | 2298 | 124 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.720 |
| ns | 2379 |  | 276 | Complete roster of every named export in src/patterns.ts (the `P` namespace) | 2.2 |  | 0.661 |
| ns | 2425 |  | 46 | src/patterns.ts re-export block: `Pattern`, `unstable_Fn`, `matcher` | 2.3 |  | 0.651 |
| walker |  | 2446 | 148 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.652 |
| ns | 2555 |  | 130 | `match()` — doc summary, signature and body in src/match.ts | 2.4 |  | 0.641 |
| ns | 2720 |  | 165 | `isMatching()` — both overload signatures with their doc summaries | 2.5 |  | 0.628 |
| walker |  | 2721 | 275 | Code::CodeKey { rung: Names, file: src/patterns.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.650 |
| walker |  | 2746 | 25 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 4, sub: 0, line: 131 } |  |  | 0.650 |
| walker |  | 2789 | 43 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 1, sub: 0, line: 81 } |  |  | 0.650 |
| walker |  | 2838 | 49 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 2, sub: 0, line: 100 } |  |  | 0.650 |
| ns | 2887 |  | 167 | Complete method roster of the `Match<>` builder type (src/types/Match.ts) | 2.6 |  | 0.628 |
| walker |  | 2898 | 60 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 5, sub: 0, line: 187 } |  |  | 0.628 |
| ns | 3042 |  | 155 | `chainable()` — the `.optional()/.and()/.or()/.select()` methods every pattern carries | 2.7 |  | 0.615 |
| walker |  | 3161 | 263 | Code::CodeKey { rung: Names, file: src/patterns.ts, decl: 0, sub: 1, line: 0 } |  |  | 0.636 |
| walker |  | 3189 | 28 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 8, sub: 0, line: 246 } |  |  | 0.636 |
| walker |  | 3218 | 29 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 22, sub: 0, line: 643 } |  |  | 0.636 |
| walker |  | 3254 | 36 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 17, sub: 0, line: 445 } |  |  | 0.636 |
| ns | 3269 |  | 227 | The eight wildcard pattern type aliases in src/types/Pattern.ts | 2.8 |  | 0.620 |
| walker |  | 3296 | 42 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 20, sub: 0, line: 611 } |  |  | 0.620 |
| walker |  | 3339 | 43 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 7, sub: 0, line: 242 } |  |  | 0.620 |
| walker |  | 3382 | 43 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 10, sub: 0, line: 295 } |  |  | 0.620 |
| walker |  | 3427 | 45 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 15, sub: 0, line: 433 } |  |  | 0.621 |
| walker |  | 3472 | 45 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 18, sub: 0, line: 536 } |  |  | 0.621 |
| walker |  | 3517 | 45 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 19, sub: 0, line: 572 } |  |  | 0.621 |
| ns | 3536 |  | 267 | `stringChainable` — all seven `P.string.*` refinement methods | 2.9 |  | 0.601 |
| walker |  | 3566 | 49 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 21, sub: 0, line: 637 } |  |  | 0.603 |
| walker |  | 3616 | 50 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 11, sub: 0, line: 299 } |  |  | 0.603 |
| walker |  | 3668 | 52 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 23, sub: 0, line: 646 } |  |  | 0.603 |
| walker |  | 3746 | 78 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 13, sub: 0, line: 357 } |  |  | 0.603 |
| ns | 3821 |  | 285 | `numberChainable` — all nine `P.number.*` refinement methods | 2.10 |  | 0.588 |
| walker |  | 3840 | 94 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 16, sub: 0, line: 437 } |  |  | 0.588 |
| walker |  | 3935 | 95 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 14, sub: 0, line: 362 } |  |  | 0.588 |
| walker |  | 4034 | 99 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 26, sub: 0, line: 686 } |  |  | 0.588 |
| ns | 4095 |  | 274 | `bigintChainable` — all seven `P.bigint.*` refinement methods | 2.11 |  | 0.574 |
| walker |  | 4184 | 150 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 25, sub: 0, line: 673 } |  |  | 0.574 |
| ns | 4249 |  | 154 | src/errors.ts in full — `NonExhaustiveError` | 2.12 |  | 0.560 |
| ns | 4436 |  | 187 | src/internals/symbols.ts — the five protocol symbols in full | 3.1 |  | 0.548 |
| walker |  | 4453 | 269 | Code::CodeKey { rung: Names, file: src/patterns.ts, decl: 0, sub: 2, line: 0 } |  |  | 0.588 |
| walker |  | 4483 | 30 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 38, sub: 0, line: 1251 } |  |  | 0.588 |
| walker |  | 4513 | 30 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 39, sub: 0, line: 1271 } |  |  | 0.590 |
| walker |  | 4557 | 44 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 27, sub: 0, line: 696 } |  |  | 0.590 |
| ns | 4658 |  | 222 | src/internals/helpers.ts — the three pattern predicates | 3.2 |  | 0.575 |
| walker |  | 4664 | 107 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 35, sub: 0, line: 1221 } |  |  | 0.575 |
| walker |  | 4773 | 109 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 28, sub: 0, line: 782 } |  |  | 0.575 |
| ns | 4864 |  | 206 | `matchPattern` signature and the Matcher-Protocol dispatch branch | 3.3 |  | 0.561 |
| walker |  | 4882 | 109 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 34, sub: 0, line: 1211 } |  |  | 0.561 |
| ns | 4981 |  | 117 | `matchPattern`'s object-key branch and the primitive fallback | 3.4 |  | 0.552 |
| walker |  | 4992 | 110 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 29, sub: 0, line: 793 } |  |  | 0.552 |
| walker |  | 5103 | 111 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 31, sub: 0, line: 928 } |  |  | 0.552 |
| ns | 5188 |  | 207 | `getSelectionKeys` and `flatMap` in full — closing src/internals/helpers.ts | 3.5 |  | 0.540 |
| walker |  | 5217 | 114 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 33, sub: 0, line: 1201 } |  |  | 0.540 |
| walker |  | 5334 | 117 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 38, sub: 0, line: 1251 } |  |  | 0.540 |
| ns | 5416 |  | 228 | The `MatchExpression` class and its complete method roster | 3.6 |  | 0.524 |
| walker |  | 5452 | 118 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 36, sub: 0, line: 1231 } |  |  | 0.524 |
| walker |  | 5572 | 120 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 30, sub: 0, line: 805 } |  |  | 0.524 |
| ns | 5635 |  | 219 | `isMatching`'s runtime implementation — the arity dispatch | 3.7 |  | 0.514 |
| walker |  | 5692 | 120 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 32, sub: 0, line: 1075 } |  |  | 0.514 |
| walker |  | 5812 | 120 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 37, sub: 0, line: 1241 } |  |  | 0.514 |
| walker |  | 5933 | 121 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 3, sub: 0, line: 116 } |  |  | 0.514 |
| ns | 6030 |  | 395 | `MatchExpression.with()` — multi-pattern, guard and selection semantics | 3.8 | 3.6 | 0.495 |
| walker |  | 6064 | 131 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 9, sub: 0, line: 294 } |  |  | 0.495 |
| walker |  | 6196 | 132 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 2, sub: 0, line: 100 } |  |  | 0.495 |
| ns | 6201 |  | 171 | `otherwise`, `exhaustive`, `run`, `returnType` and `narrow` bodies, and `defaultCatcher` | 3.9 | 3.6 | 0.482 |
| walker |  | 6338 | 142 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 6, sub: 0, line: 241 } |  |  | 0.482 |
| ns | 6370 |  | 169 | The observable rules of `matchPattern`'s tuple/variadic branch | 3.10 | 3.3 | 0.475 |
| ns | 6480 |  | 110 | `MatcherType` — the complete closed set of matcher kinds | 4.1 |  | 0.469 |
| walker |  | 6481 | 143 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 24, sub: 0, line: 672 } |  |  | 0.469 |
| walker |  | 6625 | 144 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 20, sub: 0, line: 611 } |  |  | 0.469 |
| ns | 6710 |  | 230 | `MatcherProtocol` and `MatchResult` — the contract a custom pattern implements | 4.2 |  | 0.460 |
| walker |  | 6771 | 146 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 5, sub: 0, line: 187 } |  |  | 0.460 |
| ns | 6867 |  | 157 | The `Matcher` interface and its `[symbols.isVariadic]` flag | 4.3 |  | 0.454 |
| walker |  | 6924 | 153 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 1, sub: 0, line: 81 } |  |  | 0.454 |
| walker |  | 7078 | 154 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 21, sub: 0, line: 637 } |  |  | 0.454 |
| walker |  | 7249 | 171 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 15, sub: 0, line: 433 } |  |  | 0.454 |
| ns | 7263 |  | 396 | Complete roster of the matcher-alias types (`ArrayP`, `SetP`, `SelectP`, …) | 4.4 |  | 0.442 |
| walker |  | 7423 | 174 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 12, sub: 0, line: 356 } |  |  | 0.442 |
| walker |  | 7598 | 175 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 19, sub: 0, line: 572 } |  |  | 0.442 |
| ns | 7632 |  | 369 | `Pattern<a>` itself — what shapes are legal as a pattern for a given type | 4.5 |  | 0.431 |
| walker |  | 7790 | 192 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 4, sub: 0, line: 131 } |  |  | 0.431 |
| walker |  | 7983 | 193 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 39, sub: 0, line: 1271 } |  |  | 0.431 |
| walker |  | 8001 | 18 | Code::CodeKey { rung: Names, file: src/errors.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.431 |
| ns | 8004 |  | 372 | Headline export of every remaining src/types/ module, with its purpose line | 4.6 |  | 0.423 |
| walker |  | 8018 | 17 | Code::CodeKey { rung: Decl, file: src/errors.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.424 |
| walker |  | 8058 | 40 | Code::CodeKey { rung: Doc, file: src/errors.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.428 |
| walker |  | 8133 | 75 | Code::CodeKey { rung: Body, file: src/errors.ts, decl: 2, sub: 0, line: 6 } |  |  | 0.441 |
| walker |  | 8137 | 4 | Fs::DirListing { dir: examples/gif-fetcher/public } |  |  | 0.444 |
| walker |  | 8145 | 8 | Fs::DirListing { dir: tests/types-catalog } |  |  | 0.449 |
| walker |  | 8203 | 58 | Code::CodeKey { rung: Names, file: src/is-matching.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.450 |
| walker |  | 8230 | 27 | Code::CodeKey { rung: Decl, file: src/is-matching.ts, decl: 1, sub: 0, line: 32 } |  |  | 0.451 |
| walker |  | 8263 | 33 | Code::CodeKey { rung: Decl, file: src/is-matching.ts, decl: 3, sub: 0, line: 53 } |  |  | 0.452 |
| ns | 8277 |  | 273 | package.json scripts — the complete command set for this repo | 5.1 |  | 0.462 |
| walker |  | 8302 | 39 | Code::CodeKey { rung: Decl, file: src/is-matching.ts, decl: 2, sub: 0, line: 48 } |  |  | 0.465 |
| walker |  | 8322 | 20 | Code::CodeKey { rung: Names, file: src/match.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.466 |
| walker |  | 8342 | 20 | Code::CodeKey { rung: Decl, file: src/match.ts, decl: 1, sub: 0, line: 32 } |  |  | 0.466 |
| walker |  | 8358 | 16 | Code::CodeKey { rung: Body, file: src/match.ts, decl: 1, sub: 0, line: 32 } |  |  | 0.467 |
| ns | 8453 |  | 176 | jest.config.cjs and tests/tsconfig.json in full — the test harness | 5.2 |  | 0.461 |
| walker |  | 8590 | 232 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 18, sub: 0, line: 536 } |  |  | 0.461 |
| ns | 8607 |  | 154 | tsconfig.json in full — the library's compiler settings | 5.3 |  | 0.456 |
| walker |  | 8820 | 230 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.458 |
| walker |  | 8830 | 10 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.463 |
| ns | 8853 |  | 246 | package.json `exports` map — the dual ESM/CJS and `./types` subpaths | 5.4 |  | 0.479 |
| walker |  | 8976 | 146 | Code::CodeKey { rung: Names, file: src/types/FindSelected.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.480 |
| walker |  | 8989 | 13 | Code::CodeKey { rung: Decl, file: src/types/FindSelected.ts, decl: 1, sub: 0, line: 13 } |  |  | 0.480 |
| walker |  | 9011 | 22 | Code::CodeKey { rung: Decl, file: src/types/FindSelected.ts, decl: 2, sub: 0, line: 16 } |  |  | 0.480 |
| ns | 9028 |  | 175 | docs/roadmap.md — the unimplemented items | 5.5 |  | 0.476 |
| walker |  | 9058 | 47 | Code::CodeKey { rung: Decl, file: src/types/FindSelected.ts, decl: 8, sub: 0, line: 185 } |  |  | 0.476 |
| walker |  | 9119 | 61 | Code::CodeKey { rung: Decl, file: src/types/FindSelected.ts, decl: 6, sub: 0, line: 165 } |  |  | 0.476 |
| ns | 9178 |  | 150 | Heading locations in the v4-to-v5 migration guide | 5.6 |  | 0.472 |
| walker |  | 9187 | 68 | Code::CodeKey { rung: Decl, file: src/types/FindSelected.ts, decl: 9, sub: 0, line: 191 } |  |  | 0.472 |
| walker |  | 9256 | 69 | Code::CodeKey { rung: Decl, file: src/types/FindSelected.ts, decl: 5, sub: 0, line: 159 } |  |  | 0.472 |
| ns | 9307 |  | 129 | The internal type-guard predicates behind every wildcard in src/patterns.ts | 5.7 |  | 0.466 |
| walker |  | 9402 | 146 | Code::CodeKey { rung: Decl, file: src/types/FindSelected.ts, decl: 7, sub: 0, line: 174 } |  |  | 0.466 |
| ns | 9463 |  | 156 | tests/types-catalog/utils.ts — the shared fixture types used across the suite | 5.8 |  | 0.462 |
| walker |  | 9485 | 83 | Code::CodeKey { rung: Names, file: src/internals/helpers.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.462 |
| walker |  | 9499 | 14 | Code::CodeKey { rung: Decl, file: src/internals/helpers.ts, decl: 1, sub: 0, line: 12 } |  |  | 0.463 |
| walker |  | 9529 | 30 | Code::CodeKey { rung: Decl, file: src/internals/helpers.ts, decl: 2, sub: 0, line: 16 } |  |  | 0.464 |
| walker |  | 9574 | 45 | Code::CodeKey { rung: Decl, file: src/internals/helpers.ts, decl: 3, sub: 0, line: 32 } |  |  | 0.465 |
| walker |  | 9622 | 48 | Code::CodeKey { rung: Decl, file: src/internals/helpers.ts, decl: 5, sub: 0, line: 132 } |  |  | 0.463 |
| ns | 9622 |  | 159 | examples/one-file-demo — the demo's table of contents | 5.9 |  | 0.463 |
| walker |  | 9659 | 37 | Code::CodeKey { rung: Body, file: src/internals/helpers.ts, decl: 2, sub: 0, line: 16 } |  |  | 0.466 |
| walker |  | 9826 | 167 | Code::CodeKey { rung: Names, file: src/internals/symbols.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.478 |
| ns | 9843 |  | 221 | jsr.json in full, and package.json's devDependencies | 5.10 |  | 0.472 |
| ns | 9964 |  | 121 | `.prettierrc` in full, and the benchmark runner scripts | 5.11 |  | 0.469 |
| walker |  | 9977 | 151 | Code::CodeKey { rung: Doc, file: src/is-matching.ts, decl: 2, sub: 0, line: 48 } |  |  | 0.472 |
| walker |  | 9999 | 22 | Json::Whole { file: jsr.json } |  |  | 0.472 |
