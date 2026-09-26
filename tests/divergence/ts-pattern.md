Score(3000)=0.709 I=0.836 C=0.601 ns_rows≤3K=20/53 grid(1000/1442/2080/3000/4327/6240/9000)=0.742/0.910/0.822/0.709/0.638/0.573/0.594

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 48 | 48 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 60 |  | 60 | README title and one-line description of the library | 1.1 |  | 0.000 |
| ns | 132 |  | 72 | src/index.ts in full — the complete public export surface | 1.2 |  | 0.000 |
| walker |  | 178 | 130 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.577 |
| ns | 180 |  | 48 | Complete repository root listing | 1.3 |  | 0.750 |
| walker |  | 204 | 26 | Fs::DirListing { dir: docs } |  |  | 0.750 |
| walker |  | 233 | 29 | Fs::DirListing { dir: src } |  |  | 0.761 |
| walker |  | 241 | 8 | Fs::DirListing { dir: src/internals } |  |  | 0.768 |
| walker |  | 247 | 6 | Fs::DirListing { dir: scripts } |  |  | 0.768 |
| ns | 273 |  | 93 | Complete listings of src/, src/internals/ and src/types/ | 1.4 |  | 0.588 |
| walker |  | 320 | 73 | Json::Identity { file: package.json } |  |  | 0.594 |
| ns | 357 |  | 84 | package.json identity: name, version, description, module type, entry source | 1.5 |  | 0.584 |
| walker |  | 376 | 56 | Fs::DirListing { dir: src/types } |  |  | 0.794 |
| walker |  | 385 | 9 | Fs::DirListing { dir: .github } |  |  | 0.795 |
| walker |  | 395 | 10 | Fs::DirListing { dir: examples } |  |  | 0.796 |
| walker |  | 405 | 10 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.798 |
| walker |  | 431 | 26 | Fs::DirListing { dir: benchmarks } |  |  | 0.802 |
| walker |  | 446 | 15 | Fs::DirListing { dir: examples/gif-fetcher } |  |  | 0.808 |
| walker |  | 461 | 15 | Fs::DirListing { dir: examples/one-file-demo } |  |  | 0.813 |
| ns | 513 |  | 156 | All top-level (# / ##) README headings with line numbers | 1.6 |  | 0.702 |
| walker |  | 608 | 147 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.837 |
| walker |  | 608 | 0 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.837 |
| ns | 639 |  | 126 | README opening example: the match/with/exhaustive expression itself | 1.7 |  | 0.800 |
| walker |  | 662 | 54 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.800 |
| ns | 758 |  | 119 | The discriminated-union types the README's opening example matches on | 1.8 | 1.7 | 0.733 |
| walker |  | 982 | 320 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.742 |
| ns | 1073 |  | 315 | README Features list — the capability inventory | 1.9 |  | 0.749 |
| ns | 1217 |  | 144 | Complete listings of docs/, examples/ (and their subdirs), benchmarks/, scripts/, .github/ | 1.10 |  | 0.718 |
| walker |  | 1243 | 261 | Markdown::Prelude { file: README.md } |  |  | 0.807 |
| walker |  | 1253 | 10 | Code::CodeKey { rung: Names, file: src/types/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.807 |
| walker |  | 1276 | 23 | Fs::DirListing { dir: examples/gif-fetcher/src } |  |  | 0.851 |
| walker |  | 1333 | 57 | Code::CodeKey { rung: Names, file: src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.906 |
| ns | 1523 |  | 306 | Complete tests/ listing — the feature-named test-file map | 1.11 |  | 0.729 |
| walker |  | 1631 | 298 | Fs::DirListing { dir: tests } |  |  | 0.907 |
| walker |  | 1639 | 8 | Fs::DirListing { dir: tests/types-catalog } |  |  | 0.920 |
| ns | 1751 |  | 228 | README `###` heading locations: Getting Started walkthrough and API Reference | 1.12 |  | 0.864 |
| walker |  | 1912 | 273 | Json::Scripts { file: package.json } |  |  | 0.866 |
| ns | 1965 |  | 214 | README `###` heading locations: the Patterns catalogue | 1.13 |  | 0.826 |
| walker |  | 2019 | 107 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.826 |
| ns | 2047 |  | 82 | README `###`/`####` heading locations: the Types section | 1.14 |  | 0.810 |
| ns | 2103 |  | 56 | src/patterns.ts module docstring | 2.1 |  | 0.796 |
| walker |  | 2377 | 358 | Json::Entry { file: package.json } |  |  | 0.811 |
| ns | 2379 |  | 276 | Complete roster of every named export in src/patterns.ts (the `P` namespace) | 2.2 |  | 0.745 |
| ns | 2425 |  | 46 | src/patterns.ts re-export block: `Pattern`, `unstable_Fn`, `matcher` | 2.3 |  | 0.734 |
| walker |  | 2516 | 139 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.734 |
| ns | 2555 |  | 130 | `match()` — doc summary, signature and body in src/match.ts | 2.4 |  | 0.721 |
| ns | 2720 |  | 165 | `isMatching()` — both overload signatures with their doc summaries | 2.5 |  | 0.707 |
| walker |  | 2741 | 225 | Code::CodeKey { rung: Names, file: src/patterns.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.725 |
| walker |  | 2784 | 43 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 1, sub: 0, line: 81 } |  |  | 0.725 |
| walker |  | 2833 | 49 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 2, sub: 0, line: 100 } |  |  | 0.725 |
| ns | 2887 |  | 167 | Complete method roster of the `Match<>` builder type (src/types/Match.ts) | 2.6 |  | 0.700 |
| ns | 3042 |  | 155 | `chainable()` — the `.optional()/.and()/.or()/.select()` methods every pattern carries | 2.7 |  | 0.685 |
| walker |  | 3061 | 228 | Code::CodeKey { rung: Names, file: src/patterns.ts, decl: 0, sub: 1, line: 0 } |  |  | 0.703 |
| walker |  | 3086 | 25 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 4, sub: 0, line: 131 } |  |  | 0.703 |
| walker |  | 3114 | 28 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 8, sub: 0, line: 246 } |  |  | 0.703 |
| walker |  | 3150 | 36 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 17, sub: 0, line: 445 } |  |  | 0.703 |
| walker |  | 3192 | 42 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 20, sub: 0, line: 611 } |  |  | 0.703 |
| walker |  | 3235 | 43 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 7, sub: 0, line: 242 } |  |  | 0.703 |
| ns | 3269 |  | 227 | The eight wildcard pattern type aliases in src/types/Pattern.ts | 2.8 |  | 0.685 |
| walker |  | 3278 | 43 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 10, sub: 0, line: 295 } |  |  | 0.685 |
| walker |  | 3323 | 45 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 15, sub: 0, line: 433 } |  |  | 0.686 |
| walker |  | 3368 | 45 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 18, sub: 0, line: 536 } |  |  | 0.686 |
| walker |  | 3413 | 45 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 19, sub: 0, line: 572 } |  |  | 0.686 |
| walker |  | 3463 | 50 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 11, sub: 0, line: 299 } |  |  | 0.686 |
| walker |  | 3514 | 51 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 21, sub: 0, line: 637 } |  |  | 0.688 |
| ns | 3536 |  | 267 | `stringChainable` — all seven `P.string.*` refinement methods | 2.9 |  | 0.666 |
| walker |  | 3574 | 60 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 5, sub: 0, line: 187 } |  |  | 0.667 |
| walker |  | 3652 | 78 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 13, sub: 0, line: 357 } |  |  | 0.667 |
| walker |  | 3746 | 94 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 16, sub: 0, line: 437 } |  |  | 0.667 |
| ns | 3821 |  | 285 | `numberChainable` — all nine `P.number.*` refinement methods | 2.10 |  | 0.651 |
| walker |  | 3841 | 95 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 14, sub: 0, line: 362 } |  |  | 0.651 |
| walker |  | 4029 | 188 | Code::CodeKey { rung: Names, file: src/patterns.ts, decl: 0, sub: 2, line: 0 } |  |  | 0.669 |
| walker |  | 4058 | 29 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 22, sub: 0, line: 643 } |  |  | 0.669 |
| ns | 4095 |  | 274 | `bigintChainable` — all seven `P.bigint.*` refinement methods | 2.11 |  | 0.653 |
| walker |  | 4102 | 44 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 27, sub: 0, line: 696 } |  |  | 0.653 |
| walker |  | 4154 | 52 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 23, sub: 0, line: 646 } |  |  | 0.653 |
| ns | 4249 |  | 154 | src/errors.ts in full — `NonExhaustiveError` | 2.12 |  | 0.638 |
| walker |  | 4251 | 97 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 26, sub: 0, line: 686 } |  |  | 0.638 |
| walker |  | 4401 | 150 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 25, sub: 0, line: 673 } |  |  | 0.638 |
| ns | 4436 |  | 187 | src/internals/symbols.ts — the five protocol symbols in full | 3.1 |  | 0.624 |
| walker |  | 4567 | 166 | Code::CodeKey { rung: Names, file: src/patterns.ts, decl: 0, sub: 3, line: 0 } |  |  | 0.650 |
| walker |  | 4597 | 30 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 38, sub: 0, line: 1251 } |  |  | 0.650 |
| walker |  | 4627 | 30 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 39, sub: 0, line: 1271 } |  |  | 0.652 |
| walker |  | 4647 | 20 | Code::CodeKey { rung: Names, file: src/match.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.652 |
| ns | 4658 |  | 222 | src/internals/helpers.ts — the three pattern predicates | 3.2 |  | 0.635 |
| walker |  | 4667 | 20 | Code::CodeKey { rung: Decl, file: src/match.ts, decl: 1, sub: 0, line: 32 } |  |  | 0.636 |
| walker |  | 4683 | 16 | Code::CodeKey { rung: Body, file: src/match.ts, decl: 1, sub: 0, line: 32 } |  |  | 0.638 |
| walker |  | 4741 | 58 | Code::CodeKey { rung: Names, file: src/is-matching.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.639 |
| walker |  | 4768 | 27 | Code::CodeKey { rung: Decl, file: src/is-matching.ts, decl: 1, sub: 0, line: 32 } |  |  | 0.641 |
| walker |  | 4801 | 33 | Code::CodeKey { rung: Decl, file: src/is-matching.ts, decl: 3, sub: 0, line: 53 } |  |  | 0.641 |
| walker |  | 4840 | 39 | Code::CodeKey { rung: Decl, file: src/is-matching.ts, decl: 2, sub: 0, line: 48 } |  |  | 0.645 |
| walker |  | 4858 | 18 | Code::CodeKey { rung: Names, file: src/errors.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.645 |
| ns | 4864 |  | 206 | `matchPattern` signature and the Matcher-Protocol dispatch branch | 3.3 |  | 0.629 |
| walker |  | 4875 | 17 | Code::CodeKey { rung: Decl, file: src/errors.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.630 |
| walker |  | 4879 | 4 | Fs::DirListing { dir: examples/gif-fetcher/public } |  |  | 0.634 |
| ns | 4981 |  | 117 | `matchPattern`'s object-key branch and the primitive fallback | 3.4 |  | 0.624 |
| walker |  | 5099 | 220 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.624 |
| ns | 5188 |  | 207 | `getSelectionKeys` and `flatMap` in full — closing src/internals/helpers.ts | 3.5 |  | 0.610 |
| walker |  | 5266 | 167 | Code::CodeKey { rung: Names, file: src/internals/symbols.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.629 |
| walker |  | 5281 | 15 | Code::CodeKey { rung: Doc, file: src/internals/symbols.ts, decl: 7, sub: 0, line: 26 } |  |  | 0.632 |
| walker |  | 5374 | 93 | Json::Whole { file: jsr.json } |  |  | 0.632 |
| walker |  | 5414 | 40 | Code::CodeKey { rung: Doc, file: src/errors.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.637 |
| ns | 5416 |  | 228 | The `MatchExpression` class and its complete method roster | 3.6 |  | 0.618 |
| walker |  | 5631 | 217 | Code::CodeKey { rung: Names, file: src/types/Pattern.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.618 |
| ns | 5635 |  | 219 | `isMatching`'s runtime implementation — the arity dispatch | 3.7 |  | 0.607 |
| walker |  | 5655 | 24 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 5, sub: 0, line: 74 } |  |  | 0.607 |
| walker |  | 5681 | 26 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 3, sub: 0, line: 38 } |  |  | 0.607 |
| walker |  | 5711 | 30 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 9, sub: 0, line: 96 } |  |  | 0.607 |
| walker |  | 5794 | 83 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 7, sub: 0, line: 83 } |  |  | 0.607 |
| walker |  | 5893 | 99 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 1, sub: 0, line: 7 } |  |  | 0.608 |
| ns | 6030 |  | 395 | `MatchExpression.with()` — multi-pattern, guard and selection semantics | 3.8 | 3.6 | 0.586 |
| walker |  | 6035 | 142 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 2, sub: 0, line: 23 } |  |  | 0.587 |
| ns | 6201 |  | 171 | `otherwise`, `exhaustive`, `run`, `returnType` and `narrow` bodies, and `defaultCatcher` | 3.9 | 3.6 | 0.572 |
| walker |  | 6223 | 188 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 4, sub: 0, line: 50 } |  |  | 0.573 |
| walker |  | 6245 | 22 | Code::CodeKey { rung: Doc, file: src/types/Pattern.ts, decl: 5, sub: 0, line: 74 } |  |  | 0.573 |
| walker |  | 6327 | 82 | Code::CodeKey { rung: Names, file: src/types/InvertPattern.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.573 |
| walker |  | 6349 | 22 | Code::CodeKey { rung: Decl, file: src/types/InvertPattern.ts, decl: 1, sub: 0, line: 106 } |  |  | 0.573 |
| ns | 6370 |  | 169 | The observable rules of `matchPattern`'s tuple/variadic branch | 3.10 | 3.3 | 0.564 |
| walker |  | 6373 | 24 | Code::CodeKey { rung: Decl, file: src/types/InvertPattern.ts, decl: 4, sub: 0, line: 303 } |  |  | 0.564 |
| walker |  | 6472 | 99 | Code::CodeKey { rung: Decl, file: src/types/InvertPattern.ts, decl: 2, sub: 0, line: 180 } |  |  | 0.564 |
| ns | 6480 |  | 110 | `MatcherType` — the complete closed set of matcher kinds | 4.1 |  | 0.572 |
| walker |  | 6573 | 101 | Code::CodeKey { rung: Decl, file: src/types/InvertPattern.ts, decl: 3, sub: 0, line: 192 } |  |  | 0.572 |
| walker |  | 6595 | 22 | Code::CodeKey { rung: Doc, file: src/types/InvertPattern.ts, decl: 4, sub: 0, line: 303 } |  |  | 0.572 |
| walker |  | 6678 | 83 | Code::CodeKey { rung: Names, file: src/types/Match.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.572 |
| walker |  | 6707 | 29 | Code::CodeKey { rung: Decl, file: src/types/Match.ts, decl: 4, sub: 0, line: 251 } |  |  | 0.572 |
| ns | 6710 |  | 230 | `MatcherProtocol` and `MatchResult` — the contract a custom pattern implements | 4.2 |  | 0.576 |
| walker |  | 6756 | 49 | Code::CodeKey { rung: Decl, file: src/types/Match.ts, decl: 3, sub: 0, line: 245 } |  |  | 0.576 |
| ns | 6867 |  | 157 | The `Matcher` interface and its `[symbols.isVariadic]` flag | 4.3 |  | 0.584 |
| walker |  | 6937 | 181 | Code::CodeKey { rung: Names, file: src/types/Pattern.ts, decl: 0, sub: 1, line: 0 } |  |  | 0.585 |
| walker |  | 6976 | 39 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 17, sub: 0, line: 116 } |  |  | 0.585 |
| walker |  | 7023 | 47 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 18, sub: 0, line: 124 } |  |  | 0.585 |
| walker |  | 7199 | 176 | Code::CodeKey { rung: Names, file: src/types/Pattern.ts, decl: 0, sub: 2, line: 0 } |  |  | 0.589 |
| walker |  | 7213 | 14 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 20, sub: 0, line: 132 } |  |  | 0.589 |
| walker |  | 7233 | 20 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 23, sub: 0, line: 157 } |  |  | 0.589 |
| ns | 7263 |  | 396 | Complete roster of the matcher-alias types (`ArrayP`, `SetP`, `SelectP`, …) | 4.4 |  | 0.594 |
| walker |  | 7294 | 61 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 22, sub: 0, line: 138 } |  |  | 0.603 |
| walker |  | 7315 | 21 | Code::CodeKey { rung: Doc, file: src/types/Pattern.ts, decl: 24, sub: 0, line: 190 } |  |  | 0.605 |
| walker |  | 7519 | 204 | Code::CodeKey { rung: Names, file: src/types/Pattern.ts, decl: 0, sub: 3, line: 0 } |  |  | 0.610 |
| walker |  | 7543 | 24 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 30, sub: 0, line: 196 } |  |  | 0.614 |
| walker |  | 7613 | 70 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 32, sub: 0, line: 203 } |  |  | 0.614 |
| ns | 7632 |  | 369 | `Pattern<a>` itself — what shapes are legal as a pattern for a given type | 4.5 |  | 0.599 |
| walker |  | 7795 | 182 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 38, sub: 0, line: 645 } |  |  | 0.599 |
| ns | 8004 |  | 372 | Headline export of every remaining src/types/ module, with its purpose line | 4.6 |  | 0.589 |
| walker |  | 8084 | 289 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 38, sub: 1, line: 645 } |  |  | 0.589 |
| ns | 8277 |  | 273 | package.json scripts — the complete command set for this repo | 5.1 |  | 0.596 |
| walker |  | 8286 | 202 | Code::CodeKey { rung: Names, file: src/types/helpers.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.596 |
| walker |  | 8325 | 39 | Code::CodeKey { rung: Decl, file: src/types/helpers.ts, decl: 9, sub: 0, line: 49 } |  |  | 0.596 |
| walker |  | 8373 | 48 | Code::CodeKey { rung: Decl, file: src/types/helpers.ts, decl: 5, sub: 0, line: 25 } |  |  | 0.596 |
| walker |  | 8438 | 65 | Code::CodeKey { rung: Decl, file: src/types/helpers.ts, decl: 8, sub: 0, line: 42 } |  |  | 0.596 |
| ns | 8453 |  | 176 | jest.config.cjs and tests/tsconfig.json in full — the test harness | 5.2 |  | 0.588 |
| walker |  | 8512 | 74 | Code::CodeKey { rung: Decl, file: src/types/helpers.ts, decl: 4, sub: 0, line: 16 } |  |  | 0.588 |
| walker |  | 8601 | 89 | Code::CodeKey { rung: Decl, file: src/types/helpers.ts, decl: 7, sub: 0, line: 33 } |  |  | 0.588 |
| ns | 8607 |  | 154 | tsconfig.json in full — the library's compiler settings | 5.3 |  | 0.582 |
| walker |  | 8747 | 146 | Code::CodeKey { rung: Names, file: src/types/FindSelected.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.583 |
| walker |  | 8760 | 13 | Code::CodeKey { rung: Decl, file: src/types/FindSelected.ts, decl: 1, sub: 0, line: 13 } |  |  | 0.583 |
| walker |  | 8782 | 22 | Code::CodeKey { rung: Decl, file: src/types/FindSelected.ts, decl: 2, sub: 0, line: 16 } |  |  | 0.583 |
| walker |  | 8829 | 47 | Code::CodeKey { rung: Decl, file: src/types/FindSelected.ts, decl: 8, sub: 0, line: 185 } |  |  | 0.583 |
| ns | 8853 |  | 246 | package.json `exports` map — the dual ESM/CJS and `./types` subpaths | 5.4 |  | 0.594 |
| walker |  | 8890 | 61 | Code::CodeKey { rung: Decl, file: src/types/FindSelected.ts, decl: 6, sub: 0, line: 165 } |  |  | 0.594 |
| walker |  | 8958 | 68 | Code::CodeKey { rung: Decl, file: src/types/FindSelected.ts, decl: 9, sub: 0, line: 191 } |  |  | 0.594 |
| walker |  | 9027 | 69 | Code::CodeKey { rung: Decl, file: src/types/FindSelected.ts, decl: 5, sub: 0, line: 159 } |  |  | 0.594 |
| ns | 9028 |  | 175 | docs/roadmap.md — the unimplemented items | 5.5 |  | 0.589 |
| walker |  | 9173 | 146 | Code::CodeKey { rung: Decl, file: src/types/FindSelected.ts, decl: 7, sub: 0, line: 174 } |  |  | 0.589 |
| ns | 9178 |  | 150 | Heading locations in the v4-to-v5 migration guide | 5.6 |  | 0.584 |
| walker |  | 9265 | 92 | Code::CodeKey { rung: Names, file: src/types/DistributeUnions.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.585 |
| walker |  | 9295 | 30 | Code::CodeKey { rung: Decl, file: src/types/DistributeUnions.ts, decl: 1, sub: 0, line: 41 } |  |  | 0.585 |
| ns | 9307 |  | 129 | The internal type-guard predicates behind every wildcard in src/patterns.ts | 5.7 |  | 0.577 |
| walker |  | 9390 | 95 | Code::CodeKey { rung: Decl, file: src/types/DistributeUnions.ts, decl: 4, sub: 0, line: 174 } |  |  | 0.577 |
| ns | 9463 |  | 156 | tests/types-catalog/utils.ts — the shared fixture types used across the suite | 5.8 |  | 0.572 |
| walker |  | 9519 | 129 | Code::CodeKey { rung: Decl, file: src/types/DistributeUnions.ts, decl: 5, sub: 0, line: 183 } |  |  | 0.572 |
| ns | 9622 |  | 159 | examples/one-file-demo — the demo's table of contents | 5.9 |  | 0.568 |
| walker |  | 9649 | 130 | Code::CodeKey { rung: Decl, file: src/types/DistributeUnions.ts, decl: 2, sub: 0, line: 46 } |  |  | 0.568 |
| walker |  | 9666 | 17 | Code::CodeKey { rung: Doc, file: src/types/DistributeUnions.ts, decl: 5, sub: 0, line: 183 } |  |  | 0.568 |
| walker |  | 9687 | 21 | Code::CodeKey { rung: Doc, file: src/types/DistributeUnions.ts, decl: 2, sub: 0, line: 46 } |  |  | 0.568 |
| walker |  | 9794 | 107 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 35, sub: 0, line: 1221 } |  |  | 0.568 |
| ns | 9843 |  | 221 | jsr.json in full, and package.json's devDependencies | 5.10 |  | 0.564 |
| walker |  | 9903 | 109 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 28, sub: 0, line: 782 } |  |  | 0.564 |
| ns | 9964 |  | 121 | `.prettierrc` in full, and the benchmark runner scripts | 5.11 |  | 0.561 |
