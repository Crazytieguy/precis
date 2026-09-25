Score(3000)=0.619 I=0.781 C=0.491 ns_rows≤3K=20/53 grid(1000/1442/2080/3000/4327/6240/9000)=0.671/0.647/0.714/0.619/0.607/0.516/0.548

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 48 | 48 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 60 |  | 60 | README title and one-line description of the library | 1.1 |  | 0.000 |
| walker |  | 74 | 26 | Fs::DirListing { dir: docs } |  |  | 0.000 |
| walker |  | 103 | 29 | Fs::DirListing { dir: src } |  |  | 0.000 |
| walker |  | 111 | 8 | Fs::DirListing { dir: src/internals } |  |  | 0.000 |
| ns | 132 |  | 72 | src/index.ts in full — the complete public export surface | 1.2 |  | 0.000 |
| walker |  | 168 | 57 | Code::CodeKey { rung: Names, file: src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.442 |
| walker |  | 174 | 6 | Fs::DirListing { dir: scripts } |  |  | 0.443 |
| ns | 180 |  | 48 | Complete repository root listing | 1.3 |  | 0.561 |
| walker |  | 230 | 56 | Fs::DirListing { dir: src/types } |  |  | 0.635 |
| ns | 273 |  | 93 | Complete listings of src/, src/internals/ and src/types/ | 1.4 |  | 0.631 |
| walker |  | 303 | 73 | Json::Identity { file: package.json } |  |  | 0.640 |
| walker |  | 321 | 18 | Code::CodeKey { rung: Names, file: src/errors.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.640 |
| walker |  | 338 | 17 | Code::CodeKey { rung: Decl, file: src/errors.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.641 |
| walker |  | 347 | 9 | Fs::DirListing { dir: .github } |  |  | 0.642 |
| ns | 357 |  | 84 | package.json identity: name, version, description, module type, entry source | 1.5 |  | 0.616 |
| walker |  | 367 | 20 | Code::CodeKey { rung: Names, file: src/match.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.616 |
| walker |  | 377 | 10 | Fs::DirListing { dir: examples } |  |  | 0.618 |
| walker |  | 397 | 20 | Code::CodeKey { rung: Decl, file: src/match.ts, decl: 1, sub: 0, line: 32 } |  |  | 0.618 |
| ns | 513 |  | 156 | All top-level (# / ##) README headings with line numbers | 1.6 |  | 0.534 |
| walker |  | 547 | 150 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.757 |
| walker |  | 573 | 26 | Fs::DirListing { dir: benchmarks } |  |  | 0.760 |
| walker |  | 631 | 58 | Code::CodeKey { rung: Names, file: src/is-matching.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.761 |
| ns | 639 |  | 126 | README opening example: the match/with/exhaustive expression itself | 1.7 |  | 0.727 |
| walker |  | 658 | 27 | Code::CodeKey { rung: Decl, file: src/is-matching.ts, decl: 1, sub: 0, line: 32 } |  |  | 0.728 |
| walker |  | 691 | 33 | Code::CodeKey { rung: Decl, file: src/is-matching.ts, decl: 3, sub: 0, line: 53 } |  |  | 0.728 |
| walker |  | 730 | 39 | Code::CodeKey { rung: Decl, file: src/is-matching.ts, decl: 2, sub: 0, line: 48 } |  |  | 0.729 |
| ns | 758 |  | 119 | The discriminated-union types the README's opening example matches on | 1.8 | 1.7 | 0.667 |
| walker |  | 839 | 109 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.667 |
| walker |  | 855 | 16 | Code::CodeKey { rung: Body, file: src/match.ts, decl: 1, sub: 0, line: 32 } |  |  | 0.668 |
| walker |  | 870 | 15 | Fs::DirListing { dir: examples/gif-fetcher } |  |  | 0.671 |
| ns | 1073 |  | 315 | README Features list — the capability inventory | 1.9 |  | 0.634 |
| walker |  | 1199 | 329 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.697 |
| walker |  | 1209 | 10 | Code::CodeKey { rung: Names, file: src/types/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.697 |
| ns | 1217 |  | 144 | Complete listings of docs/, examples/ (and their subdirs), benchmarks/, scripts/, .github/ | 1.10 |  | 0.647 |
| walker |  | 1507 | 298 | Fs::DirListing { dir: tests } |  |  | 0.674 |
| ns | 1523 |  | 306 | Complete tests/ listing — the feature-named test-file map | 1.11 |  | 0.718 |
| ns | 1751 |  | 228 | README `###` heading locations: Getting Started walkthrough and API Reference | 1.12 |  | 0.675 |
| walker |  | 1768 | 261 | Markdown::Prelude { file: README.md } |  |  | 0.739 |
| walker |  | 1791 | 23 | Fs::DirListing { dir: examples/gif-fetcher/src } |  |  | 0.764 |
| walker |  | 1939 | 148 | Markdown::HeadingsOutline { file: docs/v3-to-v4-migration-guide.md } |  |  | 0.764 |
| ns | 1965 |  | 214 | README `###` heading locations: the Patterns catalogue | 1.13 |  | 0.729 |
| ns | 2047 |  | 82 | README `###`/`####` heading locations: the Types section | 1.14 |  | 0.714 |
| walker |  | 2089 | 150 | Markdown::HeadingsOutline { file: docs/v4-to-v5-migration-guide.md } |  |  | 0.715 |
| ns | 2103 |  | 56 | src/patterns.ts module docstring | 2.1 |  | 0.704 |
| walker |  | 2118 | 29 | Markdown::Section { file: docs/v4-to-v5-migration-guide.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.704 |
| walker |  | 2158 | 40 | Code::CodeKey { rung: Doc, file: src/errors.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.705 |
| ns | 2379 |  | 276 | Complete roster of every named export in src/patterns.ts (the `P` namespace) | 2.2 |  | 0.647 |
| ns | 2425 |  | 46 | src/patterns.ts re-export block: `Pattern`, `unstable_Fn`, `matcher` | 2.3 |  | 0.637 |
| walker |  | 2431 | 273 | Json::Scripts { file: package.json } |  |  | 0.638 |
| ns | 2555 |  | 130 | `match()` — doc summary, signature and body in src/match.ts | 2.4 |  | 0.631 |
| walker |  | 2637 | 206 | Json::IdentityMeta { file: package.json } |  |  | 0.631 |
| ns | 2720 |  | 165 | `isMatching()` — both overload signatures with their doc summaries | 2.5 |  | 0.628 |
| ns | 2887 |  | 167 | Complete method roster of the `Match<>` builder type (src/types/Match.ts) | 2.6 |  | 0.606 |
| walker |  | 2993 | 356 | Json::Entry { file: package.json } |  |  | 0.619 |
| ns | 3042 |  | 155 | `chainable()` — the `.optional()/.and()/.or()/.select()` methods every pattern carries | 2.7 |  | 0.606 |
| walker |  | 3049 | 56 | Code::CodeKey { rung: ModuleDoc, file: src/patterns.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.620 |
| ns | 3269 |  | 227 | The eight wildcard pattern type aliases in src/types/Pattern.ts | 2.8 |  | 0.604 |
| walker |  | 3322 | 273 | Code::CodeKey { rung: Names, file: src/patterns.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.623 |
| walker |  | 3347 | 25 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 4, sub: 0, line: 131 } |  |  | 0.623 |
| walker |  | 3390 | 43 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 1, sub: 0, line: 81 } |  |  | 0.623 |
| walker |  | 3439 | 49 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 2, sub: 0, line: 100 } |  |  | 0.623 |
| walker |  | 3499 | 60 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 5, sub: 0, line: 187 } |  |  | 0.624 |
| walker |  | 3526 | 27 | Code::CodeKey { rung: Names, file: src/types/ExtractPreciseValue.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.624 |
| ns | 3536 |  | 267 | `stringChainable` — all seven `P.string.*` refinement methods | 2.9 |  | 0.604 |
| walker |  | 3554 | 28 | Code::CodeKey { rung: Names, file: src/types/IsMatching.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.604 |
| walker |  | 3583 | 29 | Code::CodeKey { rung: Names, file: src/types/DeepExclude.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.604 |
| walker |  | 3603 | 20 | Code::CodeKey { rung: Names, file: examples/gif-fetcher/src/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.604 |
| walker |  | 3639 | 36 | Code::CodeKey { rung: ModuleDoc, file: src/internals/helpers.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.604 |
| walker |  | 3677 | 38 | Code::CodeKey { rung: Names, file: src/types/Match.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.605 |
| walker |  | 3722 | 45 | Code::CodeKey { rung: Names, file: src/types/BuildMany.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.605 |
| walker |  | 3742 | 20 | Code::CodeKey { rung: Decl, file: src/types/BuildMany.ts, decl: 1, sub: 0, line: 4 } |  |  | 0.605 |
| ns | 3821 |  | 285 | `numberChainable` — all nine `P.number.*` refinement methods | 2.10 |  | 0.590 |
| walker |  | 3866 | 124 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.591 |
| walker |  | 4014 | 148 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.592 |
| walker |  | 4018 | 4 | Fs::DirListing { dir: examples/gif-fetcher/public } |  |  | 0.596 |
| walker |  | 4026 | 8 | Fs::DirListing { dir: tests/types-catalog } |  |  | 0.605 |
| ns | 4095 |  | 274 | `bigintChainable` — all seven `P.bigint.*` refinement methods | 2.11 |  | 0.591 |
| walker |  | 4101 | 75 | Code::CodeKey { rung: Body, file: src/errors.ts, decl: 2, sub: 0, line: 6 } |  |  | 0.592 |
| ns | 4249 |  | 154 | src/errors.ts in full — `NonExhaustiveError` | 2.12 |  | 0.603 |
| walker |  | 4252 | 151 | Code::CodeKey { rung: Doc, file: src/is-matching.ts, decl: 2, sub: 0, line: 48 } |  |  | 0.607 |
| walker |  | 4345 | 93 | Json::Whole { file: jsr.json } |  |  | 0.607 |
| ns | 4436 |  | 187 | src/internals/symbols.ts — the five protocol symbols in full | 3.1 |  | 0.593 |
| walker |  | 4608 | 263 | Code::CodeKey { rung: Names, file: src/patterns.ts, decl: 0, sub: 1, line: 0 } |  |  | 0.610 |
| walker |  | 4636 | 28 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 8, sub: 0, line: 246 } |  |  | 0.610 |
| ns | 4658 |  | 222 | src/internals/helpers.ts — the three pattern predicates | 3.2 |  | 0.595 |
| walker |  | 4665 | 29 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 22, sub: 0, line: 643 } |  |  | 0.595 |
| walker |  | 4701 | 36 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 17, sub: 0, line: 445 } |  |  | 0.595 |
| walker |  | 4743 | 42 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 20, sub: 0, line: 611 } |  |  | 0.595 |
| walker |  | 4786 | 43 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 7, sub: 0, line: 242 } |  |  | 0.595 |
| walker |  | 4829 | 43 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 10, sub: 0, line: 295 } |  |  | 0.595 |
| ns | 4864 |  | 206 | `matchPattern` signature and the Matcher-Protocol dispatch branch | 3.3 |  | 0.580 |
| walker |  | 4874 | 45 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 15, sub: 0, line: 433 } |  |  | 0.581 |
| walker |  | 4919 | 45 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 18, sub: 0, line: 536 } |  |  | 0.581 |
| walker |  | 4964 | 45 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 19, sub: 0, line: 572 } |  |  | 0.581 |
| ns | 4981 |  | 117 | `matchPattern`'s object-key branch and the primitive fallback | 3.4 |  | 0.572 |
| walker |  | 5013 | 49 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 21, sub: 0, line: 637 } |  |  | 0.573 |
| walker |  | 5063 | 50 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 11, sub: 0, line: 299 } |  |  | 0.573 |
| walker |  | 5115 | 52 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 23, sub: 0, line: 646 } |  |  | 0.573 |
| ns | 5188 |  | 207 | `getSelectionKeys` and `flatMap` in full — closing src/internals/helpers.ts | 3.5 |  | 0.561 |
| walker |  | 5193 | 78 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 13, sub: 0, line: 357 } |  |  | 0.561 |
| walker |  | 5287 | 94 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 16, sub: 0, line: 437 } |  |  | 0.561 |
| walker |  | 5382 | 95 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 14, sub: 0, line: 362 } |  |  | 0.561 |
| ns | 5416 |  | 228 | The `MatchExpression` class and its complete method roster | 3.6 |  | 0.544 |
| walker |  | 5481 | 99 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 26, sub: 0, line: 686 } |  |  | 0.544 |
| walker |  | 5631 | 150 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 25, sub: 0, line: 673 } |  |  | 0.544 |
| ns | 5635 |  | 219 | `isMatching`'s runtime implementation — the arity dispatch | 3.7 |  | 0.534 |
| walker |  | 5641 | 10 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.541 |
| walker |  | 5722 | 81 | Code::CodeKey { rung: Names, file: src/internals/helpers.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.542 |
| walker |  | 5736 | 14 | Code::CodeKey { rung: Decl, file: src/internals/helpers.ts, decl: 1, sub: 0, line: 12 } |  |  | 0.542 |
| walker |  | 5766 | 30 | Code::CodeKey { rung: Decl, file: src/internals/helpers.ts, decl: 2, sub: 0, line: 16 } |  |  | 0.544 |
| walker |  | 5811 | 45 | Code::CodeKey { rung: Decl, file: src/internals/helpers.ts, decl: 3, sub: 0, line: 32 } |  |  | 0.546 |
| walker |  | 5859 | 48 | Code::CodeKey { rung: Decl, file: src/internals/helpers.ts, decl: 5, sub: 0, line: 132 } |  |  | 0.548 |
| walker |  | 5941 | 82 | Code::CodeKey { rung: Names, file: src/types/InvertPattern.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.548 |
| walker |  | 5963 | 22 | Code::CodeKey { rung: Decl, file: src/types/InvertPattern.ts, decl: 1, sub: 0, line: 106 } |  |  | 0.548 |
| walker |  | 5987 | 24 | Code::CodeKey { rung: Decl, file: src/types/InvertPattern.ts, decl: 4, sub: 0, line: 303 } |  |  | 0.548 |
| ns | 6030 |  | 395 | `MatchExpression.with()` — multi-pattern, guard and selection semantics | 3.8 | 3.6 | 0.529 |
| ns | 6201 |  | 171 | `otherwise`, `exhaustive`, `run`, `returnType` and `narrow` bodies, and `defaultCatcher` | 3.9 | 3.6 | 0.515 |
| walker |  | 6217 | 230 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.516 |
| ns | 6370 |  | 169 | The observable rules of `matchPattern`'s tuple/variadic branch | 3.10 | 3.3 | 0.509 |
| walker |  | 6426 | 209 | Code::CodeKey { rung: Doc, file: src/match.ts, decl: 1, sub: 0, line: 32 } |  |  | 0.516 |
| ns | 6480 |  | 110 | `MatcherType` — the complete closed set of matcher kinds | 4.1 |  | 0.510 |
| walker |  | 6518 | 92 | Code::CodeKey { rung: Names, file: src/types/DistributeUnions.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.510 |
| walker |  | 6548 | 30 | Code::CodeKey { rung: Decl, file: src/types/DistributeUnions.ts, decl: 1, sub: 0, line: 41 } |  |  | 0.510 |
| walker |  | 6643 | 95 | Code::CodeKey { rung: Decl, file: src/types/DistributeUnions.ts, decl: 4, sub: 0, line: 174 } |  |  | 0.510 |
| ns | 6710 |  | 230 | `MatcherProtocol` and `MatchResult` — the contract a custom pattern implements | 4.2 |  | 0.500 |
| walker |  | 6851 | 208 | Markdown::Section { file: docs/v3-to-v4-migration-guide.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.500 |
| ns | 6867 |  | 157 | The `Matcher` interface and its `[symbols.isVariadic]` flag | 4.3 |  | 0.493 |
| walker |  | 6950 | 99 | Code::CodeKey { rung: Decl, file: src/types/InvertPattern.ts, decl: 2, sub: 0, line: 180 } |  |  | 0.493 |
| walker |  | 7051 | 101 | Code::CodeKey { rung: Decl, file: src/types/InvertPattern.ts, decl: 3, sub: 0, line: 192 } |  |  | 0.493 |
| ns | 7263 |  | 396 | Complete roster of the matcher-alias types (`ArrayP`, `SetP`, `SelectP`, …) | 4.4 |  | 0.481 |
| walker |  | 7320 | 269 | Code::CodeKey { rung: Names, file: src/patterns.ts, decl: 0, sub: 2, line: 0 } |  |  | 0.509 |
| walker |  | 7350 | 30 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 38, sub: 0, line: 1251 } |  |  | 0.509 |
| walker |  | 7380 | 30 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 39, sub: 0, line: 1271 } |  |  | 0.510 |
| walker |  | 7424 | 44 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 27, sub: 0, line: 696 } |  |  | 0.510 |
| ns | 7632 |  | 369 | `Pattern<a>` itself — what shapes are legal as a pattern for a given type | 4.5 |  | 0.497 |
| walker |  | 7668 | 244 | Code::CodeKey { rung: Names, file: src/types/Pattern.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.500 |
| walker |  | 7692 | 24 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 5, sub: 0, line: 74 } |  |  | 0.500 |
| walker |  | 7718 | 26 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 3, sub: 0, line: 38 } |  |  | 0.500 |
| walker |  | 7748 | 30 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 9, sub: 0, line: 96 } |  |  | 0.501 |
| walker |  | 7831 | 83 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 7, sub: 0, line: 83 } |  |  | 0.501 |
| walker |  | 7930 | 99 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 1, sub: 0, line: 7 } |  |  | 0.515 |
| ns | 8004 |  | 372 | Headline export of every remaining src/types/ module, with its purpose line | 4.6 |  | 0.512 |
| walker |  | 8106 | 176 | Code::CodeKey { rung: Doc, file: src/is-matching.ts, decl: 1, sub: 0, line: 32 } |  |  | 0.515 |
| walker |  | 8121 | 15 | Fs::DirListing { dir: examples/one-file-demo } |  |  | 0.524 |
| walker |  | 8275 | 154 | Json::Whole { file: tsconfig.json } |  |  | 0.525 |
| ns | 8277 |  | 273 | package.json scripts — the complete command set for this repo | 5.1 |  | 0.533 |
| walker |  | 8404 | 129 | Code::CodeKey { rung: Decl, file: src/types/DistributeUnions.ts, decl: 5, sub: 0, line: 183 } |  |  | 0.533 |
| ns | 8453 |  | 176 | jest.config.cjs and tests/tsconfig.json in full — the test harness | 5.2 |  | 0.526 |
| walker |  | 8534 | 130 | Code::CodeKey { rung: Decl, file: src/types/DistributeUnions.ts, decl: 2, sub: 0, line: 46 } |  |  | 0.526 |
| ns | 8607 |  | 154 | tsconfig.json in full — the library's compiler settings | 5.3 |  | 0.533 |
| walker |  | 8780 | 246 | Code::CodeKey { rung: Names, file: src/types/helpers.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.535 |
| walker |  | 8817 | 37 | Code::CodeKey { rung: Decl, file: src/types/helpers.ts, decl: 9, sub: 0, line: 49 } |  |  | 0.535 |
| ns | 8853 |  | 246 | package.json `exports` map — the dual ESM/CJS and `./types` subpaths | 5.4 |  | 0.548 |
| walker |  | 8865 | 48 | Code::CodeKey { rung: Decl, file: src/types/helpers.ts, decl: 5, sub: 0, line: 25 } |  |  | 0.548 |
| walker |  | 8930 | 65 | Code::CodeKey { rung: Decl, file: src/types/helpers.ts, decl: 8, sub: 0, line: 42 } |  |  | 0.548 |
| walker |  | 9004 | 74 | Code::CodeKey { rung: Decl, file: src/types/helpers.ts, decl: 4, sub: 0, line: 16 } |  |  | 0.548 |
| ns | 9028 |  | 175 | docs/roadmap.md — the unimplemented items | 5.5 |  | 0.544 |
| walker |  | 9093 | 89 | Code::CodeKey { rung: Decl, file: src/types/helpers.ts, decl: 7, sub: 0, line: 33 } |  |  | 0.544 |
| ns | 9178 |  | 150 | Heading locations in the v4-to-v5 migration guide | 5.6 |  | 0.549 |
| walker |  | 9239 | 146 | Code::CodeKey { rung: Names, file: src/types/FindSelected.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.552 |
| walker |  | 9252 | 13 | Code::CodeKey { rung: Decl, file: src/types/FindSelected.ts, decl: 1, sub: 0, line: 13 } |  |  | 0.552 |
| walker |  | 9274 | 22 | Code::CodeKey { rung: Decl, file: src/types/FindSelected.ts, decl: 2, sub: 0, line: 16 } |  |  | 0.552 |
| ns | 9307 |  | 129 | The internal type-guard predicates behind every wildcard in src/patterns.ts | 5.7 |  | 0.544 |
| walker |  | 9321 | 47 | Code::CodeKey { rung: Decl, file: src/types/FindSelected.ts, decl: 8, sub: 0, line: 185 } |  |  | 0.544 |
| walker |  | 9382 | 61 | Code::CodeKey { rung: Decl, file: src/types/FindSelected.ts, decl: 6, sub: 0, line: 165 } |  |  | 0.544 |
| walker |  | 9450 | 68 | Code::CodeKey { rung: Decl, file: src/types/FindSelected.ts, decl: 9, sub: 0, line: 191 } |  |  | 0.544 |
| ns | 9463 |  | 156 | tests/types-catalog/utils.ts — the shared fixture types used across the suite | 5.8 |  | 0.539 |
| walker |  | 9519 | 69 | Code::CodeKey { rung: Decl, file: src/types/FindSelected.ts, decl: 5, sub: 0, line: 159 } |  |  | 0.539 |
| ns | 9622 |  | 159 | examples/one-file-demo — the demo's table of contents | 5.9 |  | 0.536 |
| walker |  | 9661 | 142 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 2, sub: 0, line: 23 } |  |  | 0.547 |
| walker |  | 9824 | 163 | Code::CodeKey { rung: ModuleDoc, file: src/internals/symbols.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.547 |
| ns | 9843 |  | 221 | jsr.json in full, and package.json's devDependencies | 5.10 |  | 0.544 |
| ns | 9964 |  | 121 | `.prettierrc` in full, and the benchmark runner scripts | 5.11 |  | 0.540 |
| walker |  | 9970 | 146 | Code::CodeKey { rung: Decl, file: src/types/FindSelected.ts, decl: 7, sub: 0, line: 174 } |  |  | 0.540 |
