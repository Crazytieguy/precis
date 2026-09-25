Score(3000)=0.619 I=0.781 C=0.491 ns_rows≤3K=20/53 grid(1000/1442/2080/3000/4327/6240/9000)=0.675/0.713/0.715/0.619/0.632/0.551/0.538

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
| walker |  | 322 | 57 | Code::CodeKey { rung: Names, file: src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.643 |
| ns | 357 |  | 84 | package.json identity: name, version, description, module type, entry source | 1.5 |  | 0.618 |
| walker |  | 472 | 150 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.876 |
| walker |  | 498 | 26 | Fs::DirListing { dir: benchmarks } |  |  | 0.880 |
| ns | 513 |  | 156 | All top-level (# / ##) README headings with line numbers | 1.6 |  | 0.760 |
| walker |  | 516 | 18 | Code::CodeKey { rung: Names, file: src/errors.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.760 |
| walker |  | 533 | 17 | Code::CodeKey { rung: Decl, file: src/errors.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.760 |
| walker |  | 553 | 20 | Code::CodeKey { rung: Names, file: src/match.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.760 |
| walker |  | 573 | 20 | Code::CodeKey { rung: Decl, file: src/match.ts, decl: 1, sub: 0, line: 32 } |  |  | 0.760 |
| ns | 639 |  | 126 | README opening example: the match/with/exhaustive expression itself | 1.7 |  | 0.727 |
| walker |  | 682 | 109 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.727 |
| walker |  | 697 | 15 | Fs::DirListing { dir: examples/gif-fetcher } |  |  | 0.731 |
| walker |  | 713 | 16 | Code::CodeKey { rung: Body, file: src/match.ts, decl: 1, sub: 0, line: 32 } |  |  | 0.731 |
| walker |  | 736 | 23 | Fs::DirListing { dir: examples/gif-fetcher/src } |  |  | 0.737 |
| ns | 758 |  | 119 | The discriminated-union types the README's opening example matches on | 1.8 | 1.7 | 0.675 |
| walker |  | 1065 | 329 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.684 |
| ns | 1073 |  | 315 | README Features list — the capability inventory | 1.9 |  | 0.702 |
| ns | 1217 |  | 144 | Complete listings of docs/, examples/ (and their subdirs), benchmarks/, scripts/, .github/ | 1.10 |  | 0.686 |
| walker |  | 1363 | 298 | Fs::DirListing { dir: tests } |  |  | 0.713 |
| ns | 1523 |  | 306 | Complete tests/ listing — the feature-named test-file map | 1.11 |  | 0.745 |
| walker |  | 1624 | 261 | Markdown::Prelude { file: README.md } |  |  | 0.812 |
| walker |  | 1682 | 58 | Code::CodeKey { rung: Names, file: src/is-matching.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.812 |
| walker |  | 1709 | 27 | Code::CodeKey { rung: Decl, file: src/is-matching.ts, decl: 1, sub: 0, line: 32 } |  |  | 0.813 |
| walker |  | 1742 | 33 | Code::CodeKey { rung: Decl, file: src/is-matching.ts, decl: 3, sub: 0, line: 53 } |  |  | 0.813 |
| ns | 1751 |  | 228 | README `###` heading locations: Getting Started walkthrough and API Reference | 1.12 |  | 0.764 |
| walker |  | 1781 | 39 | Code::CodeKey { rung: Decl, file: src/is-matching.ts, decl: 2, sub: 0, line: 48 } |  |  | 0.764 |
| walker |  | 1929 | 148 | Markdown::HeadingsOutline { file: docs/v3-to-v4-migration-guide.md } |  |  | 0.764 |
| ns | 1965 |  | 214 | README `###` heading locations: the Patterns catalogue | 1.13 |  | 0.729 |
| ns | 2047 |  | 82 | README `###`/`####` heading locations: the Types section | 1.14 |  | 0.714 |
| walker |  | 2079 | 150 | Markdown::HeadingsOutline { file: docs/v4-to-v5-migration-guide.md } |  |  | 0.715 |
| ns | 2103 |  | 56 | src/patterns.ts module docstring | 2.1 |  | 0.704 |
| walker |  | 2108 | 29 | Markdown::Section { file: docs/v4-to-v5-migration-guide.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.704 |
| walker |  | 2148 | 40 | Code::CodeKey { rung: Doc, file: src/errors.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.705 |
| ns | 2379 |  | 276 | Complete roster of every named export in src/patterns.ts (the `P` namespace) | 2.2 |  | 0.647 |
| walker |  | 2421 | 273 | Json::Scripts { file: package.json } |  |  | 0.649 |
| ns | 2425 |  | 46 | src/patterns.ts re-export block: `Pattern`, `unstable_Fn`, `matcher` | 2.3 |  | 0.638 |
| ns | 2555 |  | 130 | `match()` — doc summary, signature and body in src/match.ts | 2.4 |  | 0.631 |
| walker |  | 2627 | 206 | Json::IdentityMeta { file: package.json } |  |  | 0.631 |
| ns | 2720 |  | 165 | `isMatching()` — both overload signatures with their doc summaries | 2.5 |  | 0.628 |
| ns | 2887 |  | 167 | Complete method roster of the `Match<>` builder type (src/types/Match.ts) | 2.6 |  | 0.606 |
| walker |  | 2983 | 356 | Json::Entry { file: package.json } |  |  | 0.619 |
| walker |  | 3039 | 56 | Code::CodeKey { rung: ModuleDoc, file: src/patterns.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.634 |
| ns | 3042 |  | 155 | `chainable()` — the `.optional()/.and()/.or()/.select()` methods every pattern carries | 2.7 |  | 0.620 |
| walker |  | 3075 | 36 | Code::CodeKey { rung: ModuleDoc, file: src/internals/helpers.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.620 |
| walker |  | 3085 | 10 | Code::CodeKey { rung: Names, file: src/types/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.620 |
| walker |  | 3089 | 4 | Fs::DirListing { dir: examples/gif-fetcher/public } |  |  | 0.625 |
| walker |  | 3097 | 8 | Fs::DirListing { dir: tests/types-catalog } |  |  | 0.634 |
| walker |  | 3221 | 124 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.636 |
| walker |  | 3231 | 10 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.645 |
| ns | 3269 |  | 227 | The eight wildcard pattern type aliases in src/types/Pattern.ts | 2.8 |  | 0.628 |
| walker |  | 3379 | 148 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.630 |
| walker |  | 3454 | 75 | Code::CodeKey { rung: Body, file: src/errors.ts, decl: 2, sub: 0, line: 6 } |  |  | 0.631 |
| ns | 3536 |  | 267 | `stringChainable` — all seven `P.string.*` refinement methods | 2.9 |  | 0.612 |
| walker |  | 3605 | 151 | Code::CodeKey { rung: Doc, file: src/is-matching.ts, decl: 2, sub: 0, line: 48 } |  |  | 0.616 |
| walker |  | 3698 | 93 | Json::Whole { file: jsr.json } |  |  | 0.616 |
| walker |  | 3713 | 15 | Fs::DirListing { dir: examples/one-file-demo } |  |  | 0.631 |
| ns | 3821 |  | 285 | `numberChainable` — all nine `P.number.*` refinement methods | 2.10 |  | 0.615 |
| walker |  | 3889 | 176 | Code::CodeKey { rung: Doc, file: src/is-matching.ts, decl: 1, sub: 0, line: 32 } |  |  | 0.621 |
| ns | 4095 |  | 274 | `bigintChainable` — all seven `P.bigint.*` refinement methods | 2.11 |  | 0.606 |
| walker |  | 4162 | 273 | Code::CodeKey { rung: Names, file: src/patterns.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.623 |
| walker |  | 4187 | 25 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 4, sub: 0, line: 131 } |  |  | 0.623 |
| walker |  | 4230 | 43 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 1, sub: 0, line: 81 } |  |  | 0.623 |
| ns | 4249 |  | 154 | src/errors.ts in full — `NonExhaustiveError` | 2.12 |  | 0.632 |
| walker |  | 4279 | 49 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 2, sub: 0, line: 100 } |  |  | 0.632 |
| walker |  | 4339 | 60 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 5, sub: 0, line: 187 } |  |  | 0.632 |
| ns | 4436 |  | 187 | src/internals/symbols.ts — the five protocol symbols in full | 3.1 |  | 0.618 |
| walker |  | 4569 | 230 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.621 |
| ns | 4658 |  | 222 | src/internals/helpers.ts — the three pattern predicates | 3.2 |  | 0.605 |
| walker |  | 4778 | 209 | Code::CodeKey { rung: Doc, file: src/match.ts, decl: 1, sub: 0, line: 32 } |  |  | 0.614 |
| walker |  | 4805 | 27 | Code::CodeKey { rung: Names, file: src/types/ExtractPreciseValue.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.614 |
| walker |  | 4833 | 28 | Code::CodeKey { rung: Names, file: src/types/IsMatching.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.614 |
| walker |  | 4862 | 29 | Code::CodeKey { rung: Names, file: src/types/DeepExclude.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.614 |
| ns | 4864 |  | 206 | `matchPattern` signature and the Matcher-Protocol dispatch branch | 3.3 |  | 0.599 |
| ns | 4981 |  | 117 | `matchPattern`'s object-key branch and the primitive fallback | 3.4 |  | 0.589 |
| walker |  | 5070 | 208 | Markdown::Section { file: docs/v3-to-v4-migration-guide.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.589 |
| walker |  | 5090 | 20 | Code::CodeKey { rung: Names, file: examples/gif-fetcher/src/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.589 |
| walker |  | 5128 | 38 | Code::CodeKey { rung: Names, file: src/types/Match.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.590 |
| ns | 5188 |  | 207 | `getSelectionKeys` and `flatMap` in full — closing src/internals/helpers.ts | 3.5 |  | 0.577 |
| walker |  | 5282 | 154 | Json::Whole { file: tsconfig.json } |  |  | 0.578 |
| ns | 5416 |  | 228 | The `MatchExpression` class and its complete method roster | 3.6 |  | 0.561 |
| walker |  | 5445 | 163 | Code::CodeKey { rung: ModuleDoc, file: src/internals/symbols.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.561 |
| walker |  | 5490 | 45 | Code::CodeKey { rung: Names, file: src/types/BuildMany.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.561 |
| walker |  | 5510 | 20 | Code::CodeKey { rung: Decl, file: src/types/BuildMany.ts, decl: 1, sub: 0, line: 4 } |  |  | 0.561 |
| walker |  | 5631 | 121 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 3, sub: 0, line: 116 } |  |  | 0.561 |
| ns | 5635 |  | 219 | `isMatching`'s runtime implementation — the arity dispatch | 3.7 |  | 0.551 |
| walker |  | 5799 | 168 | Code::CodeKey { rung: Body, file: src/is-matching.ts, decl: 3, sub: 0, line: 53 } |  |  | 0.573 |
| ns | 6030 |  | 395 | `MatchExpression.with()` — multi-pattern, guard and selection semantics | 3.8 | 3.6 | 0.553 |
| walker |  | 6062 | 263 | Code::CodeKey { rung: Names, file: src/patterns.ts, decl: 0, sub: 1, line: 0 } |  |  | 0.566 |
| walker |  | 6090 | 28 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 8, sub: 0, line: 246 } |  |  | 0.566 |
| walker |  | 6119 | 29 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 22, sub: 0, line: 643 } |  |  | 0.566 |
| walker |  | 6155 | 36 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 17, sub: 0, line: 445 } |  |  | 0.566 |
| walker |  | 6197 | 42 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 20, sub: 0, line: 611 } |  |  | 0.566 |
| ns | 6201 |  | 171 | `otherwise`, `exhaustive`, `run`, `returnType` and `narrow` bodies, and `defaultCatcher` | 3.9 | 3.6 | 0.551 |
| walker |  | 6240 | 43 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 7, sub: 0, line: 242 } |  |  | 0.551 |
| walker |  | 6283 | 43 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 10, sub: 0, line: 295 } |  |  | 0.551 |
| walker |  | 6328 | 45 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 15, sub: 0, line: 433 } |  |  | 0.552 |
| ns | 6370 |  | 169 | The observable rules of `matchPattern`'s tuple/variadic branch | 3.10 | 3.3 | 0.544 |
| walker |  | 6373 | 45 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 18, sub: 0, line: 536 } |  |  | 0.544 |
| walker |  | 6418 | 45 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 19, sub: 0, line: 572 } |  |  | 0.544 |
| walker |  | 6467 | 49 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 21, sub: 0, line: 637 } |  |  | 0.545 |
| ns | 6480 |  | 110 | `MatcherType` — the complete closed set of matcher kinds | 4.1 |  | 0.538 |
| walker |  | 6517 | 50 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 11, sub: 0, line: 299 } |  |  | 0.538 |
| walker |  | 6569 | 52 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 23, sub: 0, line: 646 } |  |  | 0.538 |
| walker |  | 6647 | 78 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 13, sub: 0, line: 357 } |  |  | 0.538 |
| ns | 6710 |  | 230 | `MatcherProtocol` and `MatchResult` — the contract a custom pattern implements | 4.2 |  | 0.528 |
| walker |  | 6741 | 94 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 16, sub: 0, line: 437 } |  |  | 0.528 |
| walker |  | 6836 | 95 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 14, sub: 0, line: 362 } |  |  | 0.528 |
| ns | 6867 |  | 157 | The `Matcher` interface and its `[symbols.isVariadic]` flag | 4.3 |  | 0.520 |
| walker |  | 6935 | 99 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 26, sub: 0, line: 686 } |  |  | 0.520 |
| walker |  | 7085 | 150 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 25, sub: 0, line: 673 } |  |  | 0.520 |
| ns | 7263 |  | 396 | Complete roster of the matcher-alias types (`ArrayP`, `SetP`, `SelectP`, …) | 4.4 |  | 0.507 |
| walker |  | 7354 | 269 | Code::CodeKey { rung: Names, file: src/patterns.ts, decl: 0, sub: 2, line: 0 } |  |  | 0.535 |
| walker |  | 7384 | 30 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 38, sub: 0, line: 1251 } |  |  | 0.535 |
| walker |  | 7414 | 30 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 39, sub: 0, line: 1271 } |  |  | 0.536 |
| walker |  | 7458 | 44 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 27, sub: 0, line: 696 } |  |  | 0.536 |
| walker |  | 7565 | 107 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 35, sub: 0, line: 1221 } |  |  | 0.536 |
| ns | 7632 |  | 369 | `Pattern<a>` itself — what shapes are legal as a pattern for a given type | 4.5 |  | 0.522 |
| walker |  | 7674 | 109 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 28, sub: 0, line: 782 } |  |  | 0.522 |
| walker |  | 7783 | 109 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 34, sub: 0, line: 1211 } |  |  | 0.522 |
| walker |  | 7893 | 110 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 29, sub: 0, line: 793 } |  |  | 0.522 |
| walker |  | 8004 | 111 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 31, sub: 0, line: 928 } |  |  | 0.515 |
| ns | 8004 |  | 372 | Headline export of every remaining src/types/ module, with its purpose line | 4.6 |  | 0.515 |
| walker |  | 8118 | 114 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 33, sub: 0, line: 1201 } |  |  | 0.515 |
| walker |  | 8235 | 117 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 38, sub: 0, line: 1251 } |  |  | 0.515 |
| ns | 8277 |  | 273 | package.json scripts — the complete command set for this repo | 5.1 |  | 0.524 |
| walker |  | 8353 | 118 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 36, sub: 0, line: 1231 } |  |  | 0.524 |
| ns | 8453 |  | 176 | jest.config.cjs and tests/tsconfig.json in full — the test harness | 5.2 |  | 0.517 |
| walker |  | 8473 | 120 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 30, sub: 0, line: 805 } |  |  | 0.517 |
| walker |  | 8593 | 120 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 32, sub: 0, line: 1075 } |  |  | 0.517 |
| ns | 8607 |  | 154 | tsconfig.json in full — the library's compiler settings | 5.3 |  | 0.525 |
| walker |  | 8713 | 120 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 37, sub: 0, line: 1241 } |  |  | 0.525 |
| walker |  | 8844 | 131 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 9, sub: 0, line: 294 } |  |  | 0.525 |
| ns | 8853 |  | 246 | package.json `exports` map — the dual ESM/CJS and `./types` subpaths | 5.4 |  | 0.538 |
| walker |  | 8976 | 132 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 2, sub: 0, line: 100 } |  |  | 0.538 |
| ns | 9028 |  | 175 | docs/roadmap.md — the unimplemented items | 5.5 |  | 0.534 |
| ns | 9178 |  | 150 | Heading locations in the v4-to-v5 migration guide | 5.6 |  | 0.540 |
| ns | 9307 |  | 129 | The internal type-guard predicates behind every wildcard in src/patterns.ts | 5.7 |  | 0.533 |
| walker |  | 9345 | 369 | Markdown::Section { file: docs/v3-to-v4-migration-guide.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.533 |
| ns | 9463 |  | 156 | tests/types-catalog/utils.ts — the shared fixture types used across the suite | 5.8 |  | 0.528 |
| walker |  | 9487 | 142 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 6, sub: 0, line: 241 } |  |  | 0.528 |
| ns | 9622 |  | 159 | examples/one-file-demo — the demo's table of contents | 5.9 |  | 0.524 |
| walker |  | 9630 | 143 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 24, sub: 0, line: 672 } |  |  | 0.524 |
| walker |  | 9774 | 144 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 20, sub: 0, line: 611 } |  |  | 0.524 |
| ns | 9843 |  | 221 | jsr.json in full, and package.json's devDependencies | 5.10 |  | 0.521 |
| walker |  | 9920 | 146 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 5, sub: 0, line: 187 } |  |  | 0.521 |
| ns | 9964 |  | 121 | `.prettierrc` in full, and the benchmark runner scripts | 5.11 |  | 0.518 |
| walker |  | 9976 | 56 | Json::Identity { file: benchmarks/package.json } |  |  | 0.518 |
