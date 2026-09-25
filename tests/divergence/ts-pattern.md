Score(3000)=0.636 I=0.789 C=0.513 ns_rows≤3K=20/53 grid(1000/1442/2080/3000/4327/6240/9000)=0.684/0.713/0.714/0.636/0.627/0.532/0.536

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
| walker |  | 645 | 57 | Code::CodeKey { rung: Names, file: src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.737 |
| ns | 758 |  | 119 | The discriminated-union types the README's opening example matches on | 1.8 | 1.7 | 0.675 |
| walker |  | 974 | 329 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.684 |
| ns | 1073 |  | 315 | README Features list — the capability inventory | 1.9 |  | 0.701 |
| ns | 1217 |  | 144 | Complete listings of docs/, examples/ (and their subdirs), benchmarks/, scripts/, .github/ | 1.10 |  | 0.685 |
| walker |  | 1272 | 298 | Fs::DirListing { dir: tests } |  |  | 0.713 |
| ns | 1523 |  | 306 | Complete tests/ listing — the feature-named test-file map | 1.11 |  | 0.744 |
| walker |  | 1533 | 261 | Markdown::Prelude { file: README.md } |  |  | 0.812 |
| walker |  | 1681 | 148 | Markdown::HeadingsOutline { file: docs/v3-to-v4-migration-guide.md } |  |  | 0.812 |
| ns | 1751 |  | 228 | README `###` heading locations: Getting Started walkthrough and API Reference | 1.12 |  | 0.762 |
| walker |  | 1831 | 150 | Markdown::HeadingsOutline { file: docs/v4-to-v5-migration-guide.md } |  |  | 0.763 |
| walker |  | 1860 | 29 | Markdown::Section { file: docs/v4-to-v5-migration-guide.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.764 |
| ns | 1965 |  | 214 | README `###` heading locations: the Patterns catalogue | 1.13 |  | 0.728 |
| ns | 2047 |  | 82 | README `###`/`####` heading locations: the Types section | 1.14 |  | 0.714 |
| ns | 2103 |  | 56 | src/patterns.ts module docstring | 2.1 |  | 0.702 |
| walker |  | 2133 | 273 | Json::Scripts { file: package.json } |  |  | 0.704 |
| walker |  | 2339 | 206 | Json::IdentityMeta { file: package.json } |  |  | 0.704 |
| ns | 2379 |  | 276 | Complete roster of every named export in src/patterns.ts (the `P` namespace) | 2.2 |  | 0.646 |
| ns | 2425 |  | 46 | src/patterns.ts re-export block: `Pattern`, `unstable_Fn`, `matcher` | 2.3 |  | 0.636 |
| ns | 2555 |  | 130 | `match()` — doc summary, signature and body in src/match.ts | 2.4 |  | 0.625 |
| walker |  | 2695 | 356 | Json::Entry { file: package.json } |  |  | 0.638 |
| ns | 2720 |  | 165 | `isMatching()` — both overload signatures with their doc summaries | 2.5 |  | 0.626 |
| walker |  | 2751 | 56 | Code::CodeKey { rung: ModuleDoc, file: src/patterns.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.642 |
| walker |  | 2787 | 36 | Code::CodeKey { rung: ModuleDoc, file: src/internals/helpers.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.642 |
| walker |  | 2791 | 4 | Fs::DirListing { dir: examples/gif-fetcher/public } |  |  | 0.646 |
| walker |  | 2799 | 8 | Fs::DirListing { dir: tests/types-catalog } |  |  | 0.656 |
| ns | 2887 |  | 167 | Complete method roster of the `Match<>` builder type (src/types/Match.ts) | 2.6 |  | 0.634 |
| walker |  | 2923 | 124 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.636 |
| walker |  | 2941 | 18 | Code::CodeKey { rung: Names, file: src/errors.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.636 |
| walker |  | 2958 | 17 | Code::CodeKey { rung: Decl, file: src/errors.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.636 |
| walker |  | 2998 | 40 | Code::CodeKey { rung: Doc, file: src/errors.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.636 |
| ns | 3042 |  | 155 | `chainable()` — the `.optional()/.and()/.or()/.select()` methods every pattern carries | 2.7 |  | 0.623 |
| walker |  | 3073 | 75 | Code::CodeKey { rung: Body, file: src/errors.ts, decl: 2, sub: 0, line: 6 } |  |  | 0.625 |
| walker |  | 3083 | 10 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.634 |
| walker |  | 3141 | 58 | Code::CodeKey { rung: Names, file: src/is-matching.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.635 |
| walker |  | 3168 | 27 | Code::CodeKey { rung: Decl, file: src/is-matching.ts, decl: 1, sub: 0, line: 32 } |  |  | 0.637 |
| walker |  | 3201 | 33 | Code::CodeKey { rung: Decl, file: src/is-matching.ts, decl: 3, sub: 0, line: 53 } |  |  | 0.637 |
| walker |  | 3240 | 39 | Code::CodeKey { rung: Decl, file: src/is-matching.ts, decl: 2, sub: 0, line: 48 } |  |  | 0.643 |
| walker |  | 3260 | 20 | Code::CodeKey { rung: Names, file: src/match.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.643 |
| ns | 3269 |  | 227 | The eight wildcard pattern type aliases in src/types/Pattern.ts | 2.8 |  | 0.627 |
| walker |  | 3280 | 20 | Code::CodeKey { rung: Decl, file: src/match.ts, decl: 1, sub: 0, line: 32 } |  |  | 0.628 |
| walker |  | 3296 | 16 | Code::CodeKey { rung: Body, file: src/match.ts, decl: 1, sub: 0, line: 32 } |  |  | 0.630 |
| walker |  | 3444 | 148 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.631 |
| ns | 3536 |  | 267 | `stringChainable` — all seven `P.string.*` refinement methods | 2.9 |  | 0.612 |
| walker |  | 3595 | 151 | Code::CodeKey { rung: Doc, file: src/is-matching.ts, decl: 2, sub: 0, line: 48 } |  |  | 0.616 |
| walker |  | 3688 | 93 | Json::Whole { file: jsr.json } |  |  | 0.616 |
| walker |  | 3703 | 15 | Fs::DirListing { dir: examples/one-file-demo } |  |  | 0.631 |
| ns | 3821 |  | 285 | `numberChainable` — all nine `P.number.*` refinement methods | 2.10 |  | 0.615 |
| walker |  | 3976 | 273 | Code::CodeKey { rung: Names, file: src/patterns.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.632 |
| walker |  | 4001 | 25 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 4, sub: 0, line: 131 } |  |  | 0.632 |
| walker |  | 4044 | 43 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 1, sub: 0, line: 81 } |  |  | 0.632 |
| walker |  | 4093 | 49 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 2, sub: 0, line: 100 } |  |  | 0.632 |
| ns | 4095 |  | 274 | `bigintChainable` — all seven `P.bigint.*` refinement methods | 2.11 |  | 0.617 |
| walker |  | 4153 | 60 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 5, sub: 0, line: 187 } |  |  | 0.618 |
| ns | 4249 |  | 154 | src/errors.ts in full — `NonExhaustiveError` | 2.12 |  | 0.627 |
| walker |  | 4329 | 176 | Code::CodeKey { rung: Doc, file: src/is-matching.ts, decl: 1, sub: 0, line: 32 } |  |  | 0.632 |
| ns | 4436 |  | 187 | src/internals/symbols.ts — the five protocol symbols in full | 3.1 |  | 0.618 |
| walker |  | 4559 | 230 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.621 |
| ns | 4658 |  | 222 | src/internals/helpers.ts — the three pattern predicates | 3.2 |  | 0.605 |
| walker |  | 4768 | 209 | Code::CodeKey { rung: Doc, file: src/match.ts, decl: 1, sub: 0, line: 32 } |  |  | 0.614 |
| ns | 4864 |  | 206 | `matchPattern` signature and the Matcher-Protocol dispatch branch | 3.3 |  | 0.599 |
| walker |  | 4976 | 208 | Markdown::Section { file: docs/v3-to-v4-migration-guide.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.599 |
| ns | 4981 |  | 117 | `matchPattern`'s object-key branch and the primitive fallback | 3.4 |  | 0.589 |
| walker |  | 5130 | 154 | Json::Whole { file: tsconfig.json } |  |  | 0.590 |
| ns | 5188 |  | 207 | `getSelectionKeys` and `flatMap` in full — closing src/internals/helpers.ts | 3.5 |  | 0.578 |
| walker |  | 5293 | 163 | Code::CodeKey { rung: ModuleDoc, file: src/internals/symbols.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.578 |
| ns | 5416 |  | 228 | The `MatchExpression` class and its complete method roster | 3.6 |  | 0.561 |
| walker |  | 5556 | 263 | Code::CodeKey { rung: Names, file: src/patterns.ts, decl: 0, sub: 1, line: 0 } |  |  | 0.575 |
| walker |  | 5584 | 28 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 8, sub: 0, line: 246 } |  |  | 0.575 |
| walker |  | 5613 | 29 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 22, sub: 0, line: 643 } |  |  | 0.575 |
| ns | 5635 |  | 219 | `isMatching`'s runtime implementation — the arity dispatch | 3.7 |  | 0.565 |
| walker |  | 5649 | 36 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 17, sub: 0, line: 445 } |  |  | 0.565 |
| walker |  | 5691 | 42 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 20, sub: 0, line: 611 } |  |  | 0.565 |
| walker |  | 5734 | 43 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 7, sub: 0, line: 242 } |  |  | 0.565 |
| walker |  | 5777 | 43 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 10, sub: 0, line: 295 } |  |  | 0.565 |
| walker |  | 5822 | 45 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 15, sub: 0, line: 433 } |  |  | 0.566 |
| walker |  | 5867 | 45 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 18, sub: 0, line: 536 } |  |  | 0.566 |
| walker |  | 5912 | 45 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 19, sub: 0, line: 572 } |  |  | 0.566 |
| walker |  | 5961 | 49 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 21, sub: 0, line: 637 } |  |  | 0.567 |
| walker |  | 6011 | 50 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 11, sub: 0, line: 299 } |  |  | 0.567 |
| ns | 6030 |  | 395 | `MatchExpression.with()` — multi-pattern, guard and selection semantics | 3.8 | 3.6 | 0.547 |
| walker |  | 6063 | 52 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 23, sub: 0, line: 646 } |  |  | 0.547 |
| walker |  | 6141 | 78 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 13, sub: 0, line: 357 } |  |  | 0.547 |
| ns | 6201 |  | 171 | `otherwise`, `exhaustive`, `run`, `returnType` and `narrow` bodies, and `defaultCatcher` | 3.9 | 3.6 | 0.532 |
| walker |  | 6235 | 94 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 16, sub: 0, line: 437 } |  |  | 0.532 |
| walker |  | 6330 | 95 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 14, sub: 0, line: 362 } |  |  | 0.532 |
| ns | 6370 |  | 169 | The observable rules of `matchPattern`'s tuple/variadic branch | 3.10 | 3.3 | 0.524 |
| walker |  | 6429 | 99 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 26, sub: 0, line: 686 } |  |  | 0.524 |
| ns | 6480 |  | 110 | `MatcherType` — the complete closed set of matcher kinds | 4.1 |  | 0.518 |
| walker |  | 6579 | 150 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 25, sub: 0, line: 673 } |  |  | 0.518 |
| ns | 6710 |  | 230 | `MatcherProtocol` and `MatchResult` — the contract a custom pattern implements | 4.2 |  | 0.508 |
| walker |  | 6848 | 269 | Code::CodeKey { rung: Names, file: src/patterns.ts, decl: 0, sub: 2, line: 0 } |  |  | 0.537 |
| ns | 6867 |  | 157 | The `Matcher` interface and its `[symbols.isVariadic]` flag | 4.3 |  | 0.530 |
| walker |  | 6878 | 30 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 38, sub: 0, line: 1251 } |  |  | 0.530 |
| walker |  | 6908 | 30 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 39, sub: 0, line: 1271 } |  |  | 0.531 |
| walker |  | 6952 | 44 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 27, sub: 0, line: 696 } |  |  | 0.531 |
| walker |  | 7059 | 107 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 35, sub: 0, line: 1221 } |  |  | 0.531 |
| walker |  | 7168 | 109 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 28, sub: 0, line: 782 } |  |  | 0.531 |
| ns | 7263 |  | 396 | Complete roster of the matcher-alias types (`ArrayP`, `SetP`, `SelectP`, …) | 4.4 |  | 0.517 |
| walker |  | 7277 | 109 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 34, sub: 0, line: 1211 } |  |  | 0.517 |
| walker |  | 7387 | 110 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 29, sub: 0, line: 793 } |  |  | 0.517 |
| walker |  | 7498 | 111 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 31, sub: 0, line: 928 } |  |  | 0.517 |
| walker |  | 7612 | 114 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 33, sub: 0, line: 1201 } |  |  | 0.517 |
| ns | 7632 |  | 369 | `Pattern<a>` itself — what shapes are legal as a pattern for a given type | 4.5 |  | 0.504 |
| walker |  | 7729 | 117 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 38, sub: 0, line: 1251 } |  |  | 0.504 |
| walker |  | 7847 | 118 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 36, sub: 0, line: 1231 } |  |  | 0.504 |
| walker |  | 7967 | 120 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 30, sub: 0, line: 805 } |  |  | 0.504 |
| ns | 8004 |  | 372 | Headline export of every remaining src/types/ module, with its purpose line | 4.6 |  | 0.495 |
| walker |  | 8087 | 120 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 32, sub: 0, line: 1075 } |  |  | 0.495 |
| walker |  | 8207 | 120 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 37, sub: 0, line: 1241 } |  |  | 0.495 |
| ns | 8277 |  | 273 | package.json scripts — the complete command set for this repo | 5.1 |  | 0.504 |
| walker |  | 8328 | 121 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 3, sub: 0, line: 116 } |  |  | 0.504 |
| ns | 8453 |  | 176 | jest.config.cjs and tests/tsconfig.json in full — the test harness | 5.2 |  | 0.498 |
| walker |  | 8496 | 168 | Code::CodeKey { rung: Body, file: src/is-matching.ts, decl: 3, sub: 0, line: 53 } |  |  | 0.514 |
| ns | 8607 |  | 154 | tsconfig.json in full — the library's compiler settings | 5.3 |  | 0.522 |
| walker |  | 8627 | 131 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 9, sub: 0, line: 294 } |  |  | 0.522 |
| walker |  | 8759 | 132 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 2, sub: 0, line: 100 } |  |  | 0.522 |
| ns | 8853 |  | 246 | package.json `exports` map — the dual ESM/CJS and `./types` subpaths | 5.4 |  | 0.536 |
| ns | 9028 |  | 175 | docs/roadmap.md — the unimplemented items | 5.5 |  | 0.532 |
| walker |  | 9128 | 369 | Markdown::Section { file: docs/v3-to-v4-migration-guide.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.532 |
| ns | 9178 |  | 150 | Heading locations in the v4-to-v5 migration guide | 5.6 |  | 0.538 |
| walker |  | 9270 | 142 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 6, sub: 0, line: 241 } |  |  | 0.538 |
| ns | 9307 |  | 129 | The internal type-guard predicates behind every wildcard in src/patterns.ts | 5.7 |  | 0.530 |
| walker |  | 9413 | 143 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 24, sub: 0, line: 672 } |  |  | 0.530 |
| ns | 9463 |  | 156 | tests/types-catalog/utils.ts — the shared fixture types used across the suite | 5.8 |  | 0.526 |
| walker |  | 9557 | 144 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 20, sub: 0, line: 611 } |  |  | 0.526 |
| ns | 9622 |  | 159 | examples/one-file-demo — the demo's table of contents | 5.9 |  | 0.522 |
| walker |  | 9703 | 146 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 5, sub: 0, line: 187 } |  |  | 0.522 |
| walker |  | 9759 | 56 | Json::Identity { file: benchmarks/package.json } |  |  | 0.522 |
| ns | 9843 |  | 221 | jsr.json in full, and package.json's devDependencies | 5.10 |  | 0.519 |
| walker |  | 9912 | 153 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 1, sub: 0, line: 81 } |  |  | 0.519 |
| ns | 9964 |  | 121 | `.prettierrc` in full, and the benchmark runner scripts | 5.11 |  | 0.516 |
