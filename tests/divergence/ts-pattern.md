Score(3000)=0.705 I=0.835 C=0.595 ns_rows≤3K=20/53 grid(1000/1442/2080/3000/4327/6240/9000)=0.738/0.907/0.810/0.705/0.638/0.573/0.546

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
| ns | 639 |  | 126 | README opening example: the match/with/exhaustive expression itself | 1.7 |  | 0.800 |
| walker |  | 715 | 107 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.800 |
| ns | 758 |  | 119 | The discriminated-union types the README's opening example matches on | 1.8 | 1.7 | 0.733 |
| walker |  | 1035 | 320 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.742 |
| ns | 1073 |  | 315 | README Features list — the capability inventory | 1.9 |  | 0.749 |
| ns | 1217 |  | 144 | Complete listings of docs/, examples/ (and their subdirs), benchmarks/, scripts/, .github/ | 1.10 |  | 0.718 |
| walker |  | 1296 | 261 | Markdown::Prelude { file: README.md } |  |  | 0.807 |
| walker |  | 1306 | 10 | Code::CodeKey { rung: Names, file: src/types/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.807 |
| walker |  | 1329 | 23 | Fs::DirListing { dir: examples/gif-fetcher/src } |  |  | 0.851 |
| walker |  | 1386 | 57 | Code::CodeKey { rung: Names, file: src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.906 |
| ns | 1523 |  | 306 | Complete tests/ listing — the feature-named test-file map | 1.11 |  | 0.729 |
| walker |  | 1684 | 298 | Fs::DirListing { dir: tests } |  |  | 0.907 |
| walker |  | 1692 | 8 | Fs::DirListing { dir: tests/types-catalog } |  |  | 0.920 |
| ns | 1751 |  | 228 | README `###` heading locations: Getting Started walkthrough and API Reference | 1.12 |  | 0.864 |
| walker |  | 1965 | 273 | Json::Scripts { file: package.json } |  |  | 0.826 |
| ns | 1965 |  | 214 | README `###` heading locations: the Patterns catalogue | 1.13 |  | 0.826 |
| ns | 2047 |  | 82 | README `###`/`####` heading locations: the Types section | 1.14 |  | 0.810 |
| walker |  | 2072 | 107 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.810 |
| ns | 2103 |  | 56 | src/patterns.ts module docstring | 2.1 |  | 0.796 |
| ns | 2379 |  | 276 | Complete roster of every named export in src/patterns.ts (the `P` namespace) | 2.2 |  | 0.732 |
| ns | 2425 |  | 46 | src/patterns.ts re-export block: `Pattern`, `unstable_Fn`, `matcher` | 2.3 |  | 0.720 |
| walker |  | 2430 | 358 | Json::Entry { file: package.json } |  |  | 0.734 |
| ns | 2555 |  | 130 | `match()` — doc summary, signature and body in src/match.ts | 2.4 |  | 0.721 |
| walker |  | 2569 | 139 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.721 |
| ns | 2720 |  | 165 | `isMatching()` — both overload signatures with their doc summaries | 2.5 |  | 0.707 |
| walker |  | 2794 | 225 | Code::CodeKey { rung: Names, file: src/patterns.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.725 |
| walker |  | 2837 | 43 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 1, sub: 0, line: 81 } |  |  | 0.725 |
| walker |  | 2886 | 49 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 2, sub: 0, line: 100 } |  |  | 0.725 |
| ns | 2887 |  | 167 | Complete method roster of the `Match<>` builder type (src/types/Match.ts) | 2.6 |  | 0.700 |
| ns | 3042 |  | 155 | `chainable()` — the `.optional()/.and()/.or()/.select()` methods every pattern carries | 2.7 |  | 0.685 |
| walker |  | 3114 | 228 | Code::CodeKey { rung: Names, file: src/patterns.ts, decl: 0, sub: 1, line: 0 } |  |  | 0.703 |
| walker |  | 3139 | 25 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 4, sub: 0, line: 131 } |  |  | 0.703 |
| walker |  | 3167 | 28 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 8, sub: 0, line: 246 } |  |  | 0.703 |
| walker |  | 3203 | 36 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 17, sub: 0, line: 445 } |  |  | 0.703 |
| walker |  | 3245 | 42 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 20, sub: 0, line: 611 } |  |  | 0.703 |
| ns | 3269 |  | 227 | The eight wildcard pattern type aliases in src/types/Pattern.ts | 2.8 |  | 0.685 |
| walker |  | 3288 | 43 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 7, sub: 0, line: 242 } |  |  | 0.685 |
| walker |  | 3331 | 43 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 10, sub: 0, line: 295 } |  |  | 0.685 |
| walker |  | 3376 | 45 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 15, sub: 0, line: 433 } |  |  | 0.686 |
| walker |  | 3421 | 45 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 18, sub: 0, line: 536 } |  |  | 0.686 |
| walker |  | 3466 | 45 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 19, sub: 0, line: 572 } |  |  | 0.686 |
| walker |  | 3516 | 50 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 11, sub: 0, line: 299 } |  |  | 0.686 |
| ns | 3536 |  | 267 | `stringChainable` — all seven `P.string.*` refinement methods | 2.9 |  | 0.665 |
| walker |  | 3567 | 51 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 21, sub: 0, line: 637 } |  |  | 0.666 |
| walker |  | 3627 | 60 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 5, sub: 0, line: 187 } |  |  | 0.667 |
| walker |  | 3705 | 78 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 13, sub: 0, line: 357 } |  |  | 0.667 |
| walker |  | 3799 | 94 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 16, sub: 0, line: 437 } |  |  | 0.667 |
| ns | 3821 |  | 285 | `numberChainable` — all nine `P.number.*` refinement methods | 2.10 |  | 0.651 |
| walker |  | 3894 | 95 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 14, sub: 0, line: 362 } |  |  | 0.651 |
| walker |  | 4082 | 188 | Code::CodeKey { rung: Names, file: src/patterns.ts, decl: 0, sub: 2, line: 0 } |  |  | 0.669 |
| ns | 4095 |  | 274 | `bigintChainable` — all seven `P.bigint.*` refinement methods | 2.11 |  | 0.653 |
| walker |  | 4111 | 29 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 22, sub: 0, line: 643 } |  |  | 0.653 |
| walker |  | 4155 | 44 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 27, sub: 0, line: 696 } |  |  | 0.653 |
| walker |  | 4207 | 52 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 23, sub: 0, line: 646 } |  |  | 0.653 |
| ns | 4249 |  | 154 | src/errors.ts in full — `NonExhaustiveError` | 2.12 |  | 0.638 |
| walker |  | 4304 | 97 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 26, sub: 0, line: 686 } |  |  | 0.638 |
| ns | 4436 |  | 187 | src/internals/symbols.ts — the five protocol symbols in full | 3.1 |  | 0.624 |
| walker |  | 4454 | 150 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 25, sub: 0, line: 673 } |  |  | 0.624 |
| walker |  | 4620 | 166 | Code::CodeKey { rung: Names, file: src/patterns.ts, decl: 0, sub: 3, line: 0 } |  |  | 0.650 |
| walker |  | 4650 | 30 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 38, sub: 0, line: 1251 } |  |  | 0.650 |
| ns | 4658 |  | 222 | src/internals/helpers.ts — the three pattern predicates | 3.2 |  | 0.633 |
| walker |  | 4680 | 30 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 39, sub: 0, line: 1271 } |  |  | 0.635 |
| walker |  | 4698 | 18 | Code::CodeKey { rung: Names, file: src/errors.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.635 |
| walker |  | 4715 | 17 | Code::CodeKey { rung: Decl, file: src/errors.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.636 |
| walker |  | 4773 | 58 | Code::CodeKey { rung: Names, file: src/is-matching.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.637 |
| walker |  | 4800 | 27 | Code::CodeKey { rung: Decl, file: src/is-matching.ts, decl: 1, sub: 0, line: 32 } |  |  | 0.639 |
| walker |  | 4833 | 33 | Code::CodeKey { rung: Decl, file: src/is-matching.ts, decl: 3, sub: 0, line: 53 } |  |  | 0.639 |
| ns | 4864 |  | 206 | `matchPattern` signature and the Matcher-Protocol dispatch branch | 3.3 |  | 0.623 |
| walker |  | 4872 | 39 | Code::CodeKey { rung: Decl, file: src/is-matching.ts, decl: 2, sub: 0, line: 48 } |  |  | 0.627 |
| walker |  | 4892 | 20 | Code::CodeKey { rung: Names, file: src/match.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.627 |
| walker |  | 4912 | 20 | Code::CodeKey { rung: Decl, file: src/match.ts, decl: 1, sub: 0, line: 32 } |  |  | 0.628 |
| walker |  | 4928 | 16 | Code::CodeKey { rung: Body, file: src/match.ts, decl: 1, sub: 0, line: 32 } |  |  | 0.630 |
| walker |  | 4932 | 4 | Fs::DirListing { dir: examples/gif-fetcher/public } |  |  | 0.634 |
| ns | 4981 |  | 117 | `matchPattern`'s object-key branch and the primitive fallback | 3.4 |  | 0.624 |
| walker |  | 5152 | 220 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.624 |
| ns | 5188 |  | 207 | `getSelectionKeys` and `flatMap` in full — closing src/internals/helpers.ts | 3.5 |  | 0.610 |
| walker |  | 5298 | 146 | Code::CodeKey { rung: Names, file: src/types/FindSelected.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.610 |
| walker |  | 5311 | 13 | Code::CodeKey { rung: Decl, file: src/types/FindSelected.ts, decl: 1, sub: 0, line: 13 } |  |  | 0.610 |
| walker |  | 5333 | 22 | Code::CodeKey { rung: Decl, file: src/types/FindSelected.ts, decl: 2, sub: 0, line: 16 } |  |  | 0.610 |
| walker |  | 5380 | 47 | Code::CodeKey { rung: Decl, file: src/types/FindSelected.ts, decl: 8, sub: 0, line: 185 } |  |  | 0.610 |
| ns | 5416 |  | 228 | The `MatchExpression` class and its complete method roster | 3.6 |  | 0.592 |
| walker |  | 5441 | 61 | Code::CodeKey { rung: Decl, file: src/types/FindSelected.ts, decl: 6, sub: 0, line: 165 } |  |  | 0.592 |
| walker |  | 5509 | 68 | Code::CodeKey { rung: Decl, file: src/types/FindSelected.ts, decl: 9, sub: 0, line: 191 } |  |  | 0.592 |
| walker |  | 5578 | 69 | Code::CodeKey { rung: Decl, file: src/types/FindSelected.ts, decl: 5, sub: 0, line: 159 } |  |  | 0.592 |
| ns | 5635 |  | 219 | `isMatching`'s runtime implementation — the arity dispatch | 3.7 |  | 0.582 |
| walker |  | 5724 | 146 | Code::CodeKey { rung: Decl, file: src/types/FindSelected.ts, decl: 7, sub: 0, line: 174 } |  |  | 0.582 |
| walker |  | 5764 | 40 | Code::CodeKey { rung: Doc, file: src/errors.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.586 |
| walker |  | 5847 | 83 | Code::CodeKey { rung: Names, file: src/internals/helpers.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.587 |
| walker |  | 5861 | 14 | Code::CodeKey { rung: Decl, file: src/internals/helpers.ts, decl: 1, sub: 0, line: 12 } |  |  | 0.587 |
| walker |  | 5891 | 30 | Code::CodeKey { rung: Decl, file: src/internals/helpers.ts, decl: 2, sub: 0, line: 16 } |  |  | 0.589 |
| walker |  | 5936 | 45 | Code::CodeKey { rung: Decl, file: src/internals/helpers.ts, decl: 3, sub: 0, line: 32 } |  |  | 0.591 |
| walker |  | 5984 | 48 | Code::CodeKey { rung: Decl, file: src/internals/helpers.ts, decl: 5, sub: 0, line: 132 } |  |  | 0.593 |
| ns | 6030 |  | 395 | `MatchExpression.with()` — multi-pattern, guard and selection semantics | 3.8 | 3.6 | 0.572 |
| walker |  | 6151 | 167 | Code::CodeKey { rung: Names, file: src/internals/symbols.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.588 |
| ns | 6201 |  | 171 | `otherwise`, `exhaustive`, `run`, `returnType` and `narrow` bodies, and `defaultCatcher` | 3.9 | 3.6 | 0.573 |
| walker |  | 6244 | 93 | Json::Whole { file: jsr.json } |  |  | 0.573 |
| walker |  | 6336 | 92 | Code::CodeKey { rung: Names, file: src/types/DistributeUnions.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.573 |
| walker |  | 6366 | 30 | Code::CodeKey { rung: Decl, file: src/types/DistributeUnions.ts, decl: 1, sub: 0, line: 41 } |  |  | 0.573 |
| ns | 6370 |  | 169 | The observable rules of `matchPattern`'s tuple/variadic branch | 3.10 | 3.3 | 0.564 |
| walker |  | 6461 | 95 | Code::CodeKey { rung: Decl, file: src/types/DistributeUnions.ts, decl: 4, sub: 0, line: 174 } |  |  | 0.564 |
| ns | 6480 |  | 110 | `MatcherType` — the complete closed set of matcher kinds | 4.1 |  | 0.558 |
| walker |  | 6590 | 129 | Code::CodeKey { rung: Decl, file: src/types/DistributeUnions.ts, decl: 5, sub: 0, line: 183 } |  |  | 0.558 |
| ns | 6710 |  | 230 | `MatcherProtocol` and `MatchResult` — the contract a custom pattern implements | 4.2 |  | 0.547 |
| walker |  | 6720 | 130 | Code::CodeKey { rung: Decl, file: src/types/DistributeUnions.ts, decl: 2, sub: 0, line: 46 } |  |  | 0.547 |
| walker |  | 6827 | 107 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 35, sub: 0, line: 1221 } |  |  | 0.547 |
| ns | 6867 |  | 157 | The `Matcher` interface and its `[symbols.isVariadic]` flag | 4.3 |  | 0.539 |
| walker |  | 6936 | 109 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 28, sub: 0, line: 782 } |  |  | 0.539 |
| walker |  | 7045 | 109 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 34, sub: 0, line: 1211 } |  |  | 0.539 |
| walker |  | 7083 | 38 | Code::CodeKey { rung: Names, file: src/types/Match.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.539 |
| walker |  | 7193 | 110 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 29, sub: 0, line: 793 } |  |  | 0.539 |
| ns | 7263 |  | 396 | Complete roster of the matcher-alias types (`ArrayP`, `SetP`, `SelectP`, …) | 4.4 |  | 0.525 |
| walker |  | 7304 | 111 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 31, sub: 0, line: 928 } |  |  | 0.525 |
| walker |  | 7418 | 114 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 33, sub: 0, line: 1201 } |  |  | 0.525 |
| walker |  | 7535 | 117 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 38, sub: 0, line: 1251 } |  |  | 0.525 |
| ns | 7632 |  | 369 | `Pattern<a>` itself — what shapes are legal as a pattern for a given type | 4.5 |  | 0.512 |
| walker |  | 7653 | 118 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 36, sub: 0, line: 1231 } |  |  | 0.512 |
| walker |  | 7735 | 82 | Code::CodeKey { rung: Names, file: src/types/InvertPattern.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.512 |
| walker |  | 7757 | 22 | Code::CodeKey { rung: Decl, file: src/types/InvertPattern.ts, decl: 1, sub: 0, line: 106 } |  |  | 0.512 |
| walker |  | 7781 | 24 | Code::CodeKey { rung: Decl, file: src/types/InvertPattern.ts, decl: 4, sub: 0, line: 303 } |  |  | 0.512 |
| walker |  | 7880 | 99 | Code::CodeKey { rung: Decl, file: src/types/InvertPattern.ts, decl: 2, sub: 0, line: 180 } |  |  | 0.512 |
| walker |  | 7981 | 101 | Code::CodeKey { rung: Decl, file: src/types/InvertPattern.ts, decl: 3, sub: 0, line: 192 } |  |  | 0.512 |
| walker |  | 8003 | 22 | Code::CodeKey { rung: Doc, file: src/types/InvertPattern.ts, decl: 4, sub: 0, line: 303 } |  |  | 0.512 |
| ns | 8004 |  | 372 | Headline export of every remaining src/types/ module, with its purpose line | 4.6 |  | 0.506 |
| walker |  | 8123 | 120 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 30, sub: 0, line: 805 } |  |  | 0.506 |
| walker |  | 8243 | 120 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 32, sub: 0, line: 1075 } |  |  | 0.506 |
| ns | 8277 |  | 273 | package.json scripts — the complete command set for this repo | 5.1 |  | 0.515 |
| walker |  | 8363 | 120 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 37, sub: 0, line: 1241 } |  |  | 0.515 |
| ns | 8453 |  | 176 | jest.config.cjs and tests/tsconfig.json in full — the test harness | 5.2 |  | 0.509 |
| walker |  | 8580 | 217 | Code::CodeKey { rung: Names, file: src/types/Pattern.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.511 |
| walker |  | 8604 | 24 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 5, sub: 0, line: 74 } |  |  | 0.511 |
| ns | 8607 |  | 154 | tsconfig.json in full — the library's compiler settings | 5.3 |  | 0.505 |
| walker |  | 8630 | 26 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 3, sub: 0, line: 38 } |  |  | 0.506 |
| walker |  | 8660 | 30 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 9, sub: 0, line: 96 } |  |  | 0.506 |
| walker |  | 8743 | 83 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 7, sub: 0, line: 83 } |  |  | 0.506 |
| walker |  | 8842 | 99 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 1, sub: 0, line: 7 } |  |  | 0.519 |
| ns | 8853 |  | 246 | package.json `exports` map — the dual ESM/CJS and `./types` subpaths | 5.4 |  | 0.533 |
| walker |  | 8984 | 142 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 2, sub: 0, line: 23 } |  |  | 0.546 |
| ns | 9028 |  | 175 | docs/roadmap.md — the unimplemented items | 5.5 |  | 0.541 |
| walker |  | 9172 | 188 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 4, sub: 0, line: 50 } |  |  | 0.556 |
| ns | 9178 |  | 150 | Heading locations in the v4-to-v5 migration guide | 5.6 |  | 0.551 |
| ns | 9307 |  | 129 | The internal type-guard predicates behind every wildcard in src/patterns.ts | 5.7 |  | 0.544 |
| walker |  | 9353 | 181 | Code::CodeKey { rung: Names, file: src/types/Pattern.ts, decl: 0, sub: 1, line: 0 } |  |  | 0.552 |
| walker |  | 9392 | 39 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 17, sub: 0, line: 116 } |  |  | 0.552 |
| walker |  | 9439 | 47 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 18, sub: 0, line: 124 } |  |  | 0.553 |
| ns | 9463 |  | 156 | tests/types-catalog/utils.ts — the shared fixture types used across the suite | 5.8 |  | 0.548 |
| walker |  | 9615 | 176 | Code::CodeKey { rung: Names, file: src/types/Pattern.ts, decl: 0, sub: 2, line: 0 } |  |  | 0.555 |
| ns | 9622 |  | 159 | examples/one-file-demo — the demo's table of contents | 5.9 |  | 0.551 |
| walker |  | 9629 | 14 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 20, sub: 0, line: 132 } |  |  | 0.553 |
| walker |  | 9649 | 20 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 23, sub: 0, line: 157 } |  |  | 0.554 |
| walker |  | 9710 | 61 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 22, sub: 0, line: 138 } |  |  | 0.561 |
| ns | 9843 |  | 221 | jsr.json in full, and package.json's devDependencies | 5.10 |  | 0.558 |
| walker |  | 9883 | 173 | Code::CodeKey { rung: Names, file: src/types/Pattern.ts, decl: 0, sub: 3, line: 0 } |  |  | 0.562 |
| walker |  | 9907 | 24 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 30, sub: 0, line: 196 } |  |  | 0.565 |
| ns | 9964 |  | 121 | `.prettierrc` in full, and the benchmark runner scripts | 5.11 |  | 0.561 |
