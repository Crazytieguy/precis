Score(3000)=0.688 I=0.836 C=0.565 ns_rows≤3K=20/53 grid(1000/1442/2080/3000/4327/6240/9000)=0.737/0.906/0.810/0.688/0.633/0.573/0.593

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 48 | 48 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 60 |  | 60 | README title and one-line description of the library | 1.1 |  | 0.000 |
| walker |  | 77 | 29 | Fs::DirListing { dir: src } |  |  | 0.000 |
| ns | 132 |  | 72 | src/index.ts in full — the complete public export surface | 1.2 |  | 0.000 |
| walker |  | 133 | 56 | Fs::DirListing { dir: src/types } |  |  | 0.000 |
| ns | 180 |  | 48 | Complete repository root listing | 1.3 |  | 0.494 |
| walker |  | 206 | 73 | Json::Identity { file: package.json } |  |  | 0.504 |
| walker |  | 214 | 8 | Fs::DirListing { dir: src/internals } |  |  | 0.523 |
| ns | 273 |  | 93 | Complete listings of src/, src/internals/ and src/types/ | 1.4 |  | 0.544 |
| walker |  | 344 | 130 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.826 |
| ns | 357 |  | 84 | package.json identity: name, version, description, module type, entry source | 1.5 |  | 0.794 |
| walker |  | 370 | 26 | Fs::DirListing { dir: docs } |  |  | 0.794 |
| walker |  | 376 | 6 | Fs::DirListing { dir: scripts } |  |  | 0.794 |
| walker |  | 385 | 9 | Fs::DirListing { dir: .github } |  |  | 0.795 |
| walker |  | 395 | 10 | Fs::DirListing { dir: examples } |  |  | 0.796 |
| walker |  | 405 | 10 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.798 |
| walker |  | 431 | 26 | Fs::DirListing { dir: benchmarks } |  |  | 0.802 |
| walker |  | 446 | 15 | Fs::DirListing { dir: examples/gif-fetcher } |  |  | 0.808 |
| walker |  | 461 | 15 | Fs::DirListing { dir: examples/one-file-demo } |  |  | 0.813 |
| ns | 513 |  | 156 | All top-level (# / ##) README headings with line numbers | 1.6 |  | 0.702 |
| walker |  | 552 | 91 | Json::Scripts { file: package.json } |  |  | 0.702 |
| ns | 639 |  | 126 | README opening example: the match/with/exhaustive expression itself | 1.7 |  | 0.671 |
| walker |  | 699 | 147 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.800 |
| walker |  | 699 | 0 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.800 |
| walker |  | 753 | 54 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.800 |
| ns | 758 |  | 119 | The discriminated-union types the README's opening example matches on | 1.8 | 1.7 | 0.733 |
| walker |  | 1073 | 320 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.750 |
| ns | 1073 |  | 315 | README Features list — the capability inventory | 1.9 |  | 0.750 |
| ns | 1217 |  | 144 | Complete listings of docs/, examples/ (and their subdirs), benchmarks/, scripts/, .github/ | 1.10 |  | 0.718 |
| walker |  | 1334 | 261 | Markdown::Prelude { file: README.md } |  |  | 0.807 |
| walker |  | 1344 | 10 | Code::CodeKey { rung: Names, file: src/types/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.807 |
| walker |  | 1367 | 23 | Fs::DirListing { dir: examples/gif-fetcher/src } |  |  | 0.851 |
| walker |  | 1424 | 57 | Code::CodeKey { rung: Names, file: src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.906 |
| ns | 1523 |  | 306 | Complete tests/ listing — the feature-named test-file map | 1.11 |  | 0.730 |
| walker |  | 1606 | 182 | Json::ScriptsTail { file: package.json } |  |  | 0.731 |
| ns | 1751 |  | 228 | README `###` heading locations: Getting Started walkthrough and API Reference | 1.12 |  | 0.687 |
| walker |  | 1904 | 298 | Fs::DirListing { dir: tests } |  |  | 0.853 |
| walker |  | 1912 | 8 | Fs::DirListing { dir: tests/types-catalog } |  |  | 0.866 |
| walker |  | 1932 | 20 | Code::CodeKey { rung: Names, file: src/match.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.866 |
| walker |  | 1952 | 20 | Code::CodeKey { rung: Decl, file: src/match.ts, decl: 1, sub: 0, line: 32 } |  |  | 0.866 |
| ns | 1965 |  | 214 | README `###` heading locations: the Patterns catalogue | 1.13 |  | 0.826 |
| ns | 2047 |  | 82 | README `###`/`####` heading locations: the Types section | 1.14 |  | 0.810 |
| ns | 2103 |  | 56 | src/patterns.ts module docstring | 2.1 |  | 0.797 |
| walker |  | 2169 | 217 | Code::CodeKey { rung: Names, file: src/types/Pattern.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.797 |
| walker |  | 2193 | 24 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 5, sub: 0, line: 74 } |  |  | 0.797 |
| walker |  | 2219 | 26 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 3, sub: 0, line: 38 } |  |  | 0.797 |
| walker |  | 2249 | 30 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 9, sub: 0, line: 96 } |  |  | 0.797 |
| walker |  | 2332 | 83 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 7, sub: 0, line: 83 } |  |  | 0.797 |
| ns | 2379 |  | 276 | Complete roster of every named export in src/patterns.ts (the `P` namespace) | 2.2 |  | 0.732 |
| ns | 2425 |  | 46 | src/patterns.ts re-export block: `Pattern`, `unstable_Fn`, `matcher` | 2.3 |  | 0.721 |
| walker |  | 2431 | 99 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 1, sub: 0, line: 7 } |  |  | 0.722 |
| ns | 2555 |  | 130 | `match()` — doc summary, signature and body in src/match.ts | 2.4 |  | 0.711 |
| walker |  | 2573 | 142 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 2, sub: 0, line: 23 } |  |  | 0.712 |
| walker |  | 2680 | 107 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.712 |
| ns | 2720 |  | 165 | `isMatching()` — both overload signatures with their doc summaries | 2.5 |  | 0.699 |
| ns | 2887 |  | 167 | Complete method roster of the `Match<>` builder type (src/types/Match.ts) | 2.6 |  | 0.675 |
| walker |  | 3038 | 358 | Json::Entry { file: package.json } |  |  | 0.688 |
| ns | 3042 |  | 155 | `chainable()` — the `.optional()/.and()/.or()/.select()` methods every pattern carries | 2.7 |  | 0.673 |
| walker |  | 3226 | 188 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 4, sub: 0, line: 50 } |  |  | 0.675 |
| ns | 3269 |  | 227 | The eight wildcard pattern type aliases in src/types/Pattern.ts | 2.8 |  | 0.657 |
| walker |  | 3365 | 139 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.657 |
| walker |  | 3381 | 16 | Code::CodeKey { rung: Body, file: src/match.ts, decl: 1, sub: 0, line: 32 } |  |  | 0.659 |
| walker |  | 3439 | 58 | Code::CodeKey { rung: Names, file: src/is-matching.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.660 |
| walker |  | 3466 | 27 | Code::CodeKey { rung: Decl, file: src/is-matching.ts, decl: 1, sub: 0, line: 32 } |  |  | 0.663 |
| walker |  | 3499 | 33 | Code::CodeKey { rung: Decl, file: src/is-matching.ts, decl: 3, sub: 0, line: 53 } |  |  | 0.663 |
| ns | 3536 |  | 267 | `stringChainable` — all seven `P.string.*` refinement methods | 2.9 |  | 0.642 |
| walker |  | 3538 | 39 | Code::CodeKey { rung: Decl, file: src/is-matching.ts, decl: 2, sub: 0, line: 48 } |  |  | 0.647 |
| walker |  | 3556 | 18 | Code::CodeKey { rung: Names, file: src/errors.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.647 |
| walker |  | 3573 | 17 | Code::CodeKey { rung: Decl, file: src/errors.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.647 |
| walker |  | 3798 | 225 | Code::CodeKey { rung: Names, file: src/patterns.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.663 |
| ns | 3821 |  | 285 | `numberChainable` — all nine `P.number.*` refinement methods | 2.10 |  | 0.646 |
| walker |  | 3841 | 43 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 1, sub: 0, line: 81 } |  |  | 0.646 |
| walker |  | 3890 | 49 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 2, sub: 0, line: 100 } |  |  | 0.646 |
| ns | 4095 |  | 274 | `bigintChainable` — all seven `P.bigint.*` refinement methods | 2.11 |  | 0.631 |
| walker |  | 4118 | 228 | Code::CodeKey { rung: Names, file: src/patterns.ts, decl: 0, sub: 1, line: 0 } |  |  | 0.647 |
| walker |  | 4143 | 25 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 4, sub: 0, line: 131 } |  |  | 0.647 |
| walker |  | 4171 | 28 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 8, sub: 0, line: 246 } |  |  | 0.647 |
| walker |  | 4207 | 36 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 17, sub: 0, line: 445 } |  |  | 0.647 |
| walker |  | 4249 | 42 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 20, sub: 0, line: 611 } |  |  | 0.633 |
| ns | 4249 |  | 154 | src/errors.ts in full — `NonExhaustiveError` | 2.12 |  | 0.633 |
| walker |  | 4292 | 43 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 7, sub: 0, line: 242 } |  |  | 0.633 |
| walker |  | 4335 | 43 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 10, sub: 0, line: 295 } |  |  | 0.633 |
| walker |  | 4380 | 45 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 15, sub: 0, line: 433 } |  |  | 0.634 |
| walker |  | 4425 | 45 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 18, sub: 0, line: 536 } |  |  | 0.634 |
| ns | 4436 |  | 187 | src/internals/symbols.ts — the five protocol symbols in full | 3.1 |  | 0.620 |
| walker |  | 4470 | 45 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 19, sub: 0, line: 572 } |  |  | 0.620 |
| walker |  | 4520 | 50 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 11, sub: 0, line: 299 } |  |  | 0.620 |
| walker |  | 4571 | 51 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 21, sub: 0, line: 637 } |  |  | 0.622 |
| walker |  | 4631 | 60 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 5, sub: 0, line: 187 } |  |  | 0.623 |
| ns | 4658 |  | 222 | src/internals/helpers.ts — the three pattern predicates | 3.2 |  | 0.607 |
| walker |  | 4709 | 78 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 13, sub: 0, line: 357 } |  |  | 0.607 |
| walker |  | 4803 | 94 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 16, sub: 0, line: 437 } |  |  | 0.607 |
| ns | 4864 |  | 206 | `matchPattern` signature and the Matcher-Protocol dispatch branch | 3.3 |  | 0.591 |
| walker |  | 4898 | 95 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 14, sub: 0, line: 362 } |  |  | 0.591 |
| ns | 4981 |  | 117 | `matchPattern`'s object-key branch and the primitive fallback | 3.4 |  | 0.582 |
| walker |  | 5086 | 188 | Code::CodeKey { rung: Names, file: src/patterns.ts, decl: 0, sub: 2, line: 0 } |  |  | 0.598 |
| walker |  | 5115 | 29 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 22, sub: 0, line: 643 } |  |  | 0.598 |
| walker |  | 5159 | 44 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 27, sub: 0, line: 696 } |  |  | 0.598 |
| ns | 5188 |  | 207 | `getSelectionKeys` and `flatMap` in full — closing src/internals/helpers.ts | 3.5 |  | 0.585 |
| walker |  | 5211 | 52 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 23, sub: 0, line: 646 } |  |  | 0.585 |
| walker |  | 5308 | 97 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 26, sub: 0, line: 686 } |  |  | 0.585 |
| ns | 5416 |  | 228 | The `MatchExpression` class and its complete method roster | 3.6 |  | 0.568 |
| walker |  | 5458 | 150 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 25, sub: 0, line: 673 } |  |  | 0.568 |
| walker |  | 5624 | 166 | Code::CodeKey { rung: Names, file: src/patterns.ts, decl: 0, sub: 3, line: 0 } |  |  | 0.591 |
| ns | 5635 |  | 219 | `isMatching`'s runtime implementation — the arity dispatch | 3.7 |  | 0.580 |
| walker |  | 5654 | 30 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 38, sub: 0, line: 1251 } |  |  | 0.580 |
| walker |  | 5684 | 30 | Code::CodeKey { rung: Decl, file: src/patterns.ts, decl: 39, sub: 0, line: 1271 } |  |  | 0.582 |
| walker |  | 5688 | 4 | Fs::DirListing { dir: examples/gif-fetcher/public } |  |  | 0.585 |
| walker |  | 5908 | 220 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.585 |
| ns | 6030 |  | 395 | `MatchExpression.with()` — multi-pattern, guard and selection semantics | 3.8 | 3.6 | 0.564 |
| walker |  | 6075 | 167 | Code::CodeKey { rung: Names, file: src/internals/symbols.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.581 |
| walker |  | 6090 | 15 | Code::CodeKey { rung: Doc, file: src/internals/symbols.ts, decl: 7, sub: 0, line: 26 } |  |  | 0.584 |
| walker |  | 6130 | 40 | Code::CodeKey { rung: Doc, file: src/errors.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.588 |
| walker |  | 6152 | 22 | Code::CodeKey { rung: Doc, file: src/types/Pattern.ts, decl: 5, sub: 0, line: 74 } |  |  | 0.588 |
| ns | 6201 |  | 171 | `otherwise`, `exhaustive`, `run`, `returnType` and `narrow` bodies, and `defaultCatcher` | 3.9 | 3.6 | 0.573 |
| walker |  | 6234 | 82 | Code::CodeKey { rung: Names, file: src/types/InvertPattern.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.573 |
| walker |  | 6256 | 22 | Code::CodeKey { rung: Decl, file: src/types/InvertPattern.ts, decl: 1, sub: 0, line: 106 } |  |  | 0.573 |
| walker |  | 6280 | 24 | Code::CodeKey { rung: Decl, file: src/types/InvertPattern.ts, decl: 4, sub: 0, line: 303 } |  |  | 0.573 |
| ns | 6370 |  | 169 | The observable rules of `matchPattern`'s tuple/variadic branch | 3.10 | 3.3 | 0.564 |
| walker |  | 6379 | 99 | Code::CodeKey { rung: Decl, file: src/types/InvertPattern.ts, decl: 2, sub: 0, line: 180 } |  |  | 0.564 |
| walker |  | 6480 | 101 | Code::CodeKey { rung: Decl, file: src/types/InvertPattern.ts, decl: 3, sub: 0, line: 192 } |  |  | 0.572 |
| ns | 6480 |  | 110 | `MatcherType` — the complete closed set of matcher kinds | 4.1 |  | 0.572 |
| walker |  | 6502 | 22 | Code::CodeKey { rung: Doc, file: src/types/InvertPattern.ts, decl: 4, sub: 0, line: 303 } |  |  | 0.572 |
| walker |  | 6585 | 83 | Code::CodeKey { rung: Names, file: src/types/Match.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.572 |
| walker |  | 6614 | 29 | Code::CodeKey { rung: Decl, file: src/types/Match.ts, decl: 4, sub: 0, line: 251 } |  |  | 0.572 |
| walker |  | 6663 | 49 | Code::CodeKey { rung: Decl, file: src/types/Match.ts, decl: 3, sub: 0, line: 245 } |  |  | 0.572 |
| ns | 6710 |  | 230 | `MatcherProtocol` and `MatchResult` — the contract a custom pattern implements | 4.2 |  | 0.576 |
| walker |  | 6844 | 181 | Code::CodeKey { rung: Names, file: src/types/Pattern.ts, decl: 0, sub: 1, line: 0 } |  |  | 0.576 |
| ns | 6867 |  | 157 | The `Matcher` interface and its `[symbols.isVariadic]` flag | 4.3 |  | 0.585 |
| walker |  | 6883 | 39 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 17, sub: 0, line: 116 } |  |  | 0.585 |
| walker |  | 6930 | 47 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 18, sub: 0, line: 124 } |  |  | 0.585 |
| walker |  | 7106 | 176 | Code::CodeKey { rung: Names, file: src/types/Pattern.ts, decl: 0, sub: 2, line: 0 } |  |  | 0.589 |
| walker |  | 7120 | 14 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 20, sub: 0, line: 132 } |  |  | 0.589 |
| walker |  | 7140 | 20 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 23, sub: 0, line: 157 } |  |  | 0.589 |
| walker |  | 7201 | 61 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 22, sub: 0, line: 138 } |  |  | 0.590 |
| walker |  | 7222 | 21 | Code::CodeKey { rung: Doc, file: src/types/Pattern.ts, decl: 24, sub: 0, line: 190 } |  |  | 0.591 |
| ns | 7263 |  | 396 | Complete roster of the matcher-alias types (`ArrayP`, `SetP`, `SelectP`, …) | 4.4 |  | 0.605 |
| walker |  | 7426 | 204 | Code::CodeKey { rung: Names, file: src/types/Pattern.ts, decl: 0, sub: 3, line: 0 } |  |  | 0.610 |
| walker |  | 7450 | 24 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 30, sub: 0, line: 196 } |  |  | 0.614 |
| walker |  | 7520 | 70 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 32, sub: 0, line: 203 } |  |  | 0.614 |
| ns | 7632 |  | 369 | `Pattern<a>` itself — what shapes are legal as a pattern for a given type | 4.5 |  | 0.598 |
| walker |  | 7702 | 182 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 38, sub: 0, line: 645 } |  |  | 0.598 |
| walker |  | 7991 | 289 | Code::CodeKey { rung: Decl, file: src/types/Pattern.ts, decl: 38, sub: 1, line: 645 } |  |  | 0.598 |
| ns | 8004 |  | 372 | Headline export of every remaining src/types/ module, with its purpose line | 4.6 |  | 0.589 |
| walker |  | 8193 | 202 | Code::CodeKey { rung: Names, file: src/types/helpers.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.589 |
| walker |  | 8232 | 39 | Code::CodeKey { rung: Decl, file: src/types/helpers.ts, decl: 9, sub: 0, line: 49 } |  |  | 0.589 |
| ns | 8277 |  | 273 | package.json scripts — the complete command set for this repo | 5.1 |  | 0.595 |
| walker |  | 8280 | 48 | Code::CodeKey { rung: Decl, file: src/types/helpers.ts, decl: 5, sub: 0, line: 25 } |  |  | 0.595 |
| walker |  | 8345 | 65 | Code::CodeKey { rung: Decl, file: src/types/helpers.ts, decl: 8, sub: 0, line: 42 } |  |  | 0.595 |
| walker |  | 8419 | 74 | Code::CodeKey { rung: Decl, file: src/types/helpers.ts, decl: 4, sub: 0, line: 16 } |  |  | 0.595 |
| ns | 8453 |  | 176 | jest.config.cjs and tests/tsconfig.json in full — the test harness | 5.2 |  | 0.588 |
| walker |  | 8508 | 89 | Code::CodeKey { rung: Decl, file: src/types/helpers.ts, decl: 7, sub: 0, line: 33 } |  |  | 0.588 |
| ns | 8607 |  | 154 | tsconfig.json in full — the library's compiler settings | 5.3 |  | 0.581 |
| walker |  | 8654 | 146 | Code::CodeKey { rung: Names, file: src/types/FindSelected.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.582 |
| walker |  | 8667 | 13 | Code::CodeKey { rung: Decl, file: src/types/FindSelected.ts, decl: 1, sub: 0, line: 13 } |  |  | 0.582 |
| walker |  | 8689 | 22 | Code::CodeKey { rung: Decl, file: src/types/FindSelected.ts, decl: 2, sub: 0, line: 16 } |  |  | 0.582 |
| walker |  | 8736 | 47 | Code::CodeKey { rung: Decl, file: src/types/FindSelected.ts, decl: 8, sub: 0, line: 185 } |  |  | 0.582 |
| walker |  | 8797 | 61 | Code::CodeKey { rung: Decl, file: src/types/FindSelected.ts, decl: 6, sub: 0, line: 165 } |  |  | 0.582 |
| ns | 8853 |  | 246 | package.json `exports` map — the dual ESM/CJS and `./types` subpaths | 5.4 |  | 0.593 |
| walker |  | 8865 | 68 | Code::CodeKey { rung: Decl, file: src/types/FindSelected.ts, decl: 9, sub: 0, line: 191 } |  |  | 0.593 |
| walker |  | 8934 | 69 | Code::CodeKey { rung: Decl, file: src/types/FindSelected.ts, decl: 5, sub: 0, line: 159 } |  |  | 0.593 |
| ns | 9028 |  | 175 | docs/roadmap.md — the unimplemented items | 5.5 |  | 0.589 |
| walker |  | 9080 | 146 | Code::CodeKey { rung: Decl, file: src/types/FindSelected.ts, decl: 7, sub: 0, line: 174 } |  |  | 0.589 |
| walker |  | 9172 | 92 | Code::CodeKey { rung: Names, file: src/types/DistributeUnions.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.590 |
| ns | 9178 |  | 150 | Heading locations in the v4-to-v5 migration guide | 5.6 |  | 0.584 |
| walker |  | 9202 | 30 | Code::CodeKey { rung: Decl, file: src/types/DistributeUnions.ts, decl: 1, sub: 0, line: 41 } |  |  | 0.584 |
| walker |  | 9297 | 95 | Code::CodeKey { rung: Decl, file: src/types/DistributeUnions.ts, decl: 4, sub: 0, line: 174 } |  |  | 0.584 |
| ns | 9307 |  | 129 | The internal type-guard predicates behind every wildcard in src/patterns.ts | 5.7 |  | 0.577 |
| walker |  | 9426 | 129 | Code::CodeKey { rung: Decl, file: src/types/DistributeUnions.ts, decl: 5, sub: 0, line: 183 } |  |  | 0.577 |
| ns | 9463 |  | 156 | tests/types-catalog/utils.ts — the shared fixture types used across the suite | 5.8 |  | 0.572 |
| walker |  | 9556 | 130 | Code::CodeKey { rung: Decl, file: src/types/DistributeUnions.ts, decl: 2, sub: 0, line: 46 } |  |  | 0.572 |
| walker |  | 9573 | 17 | Code::CodeKey { rung: Doc, file: src/types/DistributeUnions.ts, decl: 5, sub: 0, line: 183 } |  |  | 0.572 |
| walker |  | 9594 | 21 | Code::CodeKey { rung: Doc, file: src/types/DistributeUnions.ts, decl: 2, sub: 0, line: 46 } |  |  | 0.572 |
| ns | 9622 |  | 159 | examples/one-file-demo — the demo's table of contents | 5.9 |  | 0.568 |
| walker |  | 9701 | 107 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 35, sub: 0, line: 1221 } |  |  | 0.568 |
| walker |  | 9810 | 109 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 28, sub: 0, line: 782 } |  |  | 0.568 |
| ns | 9843 |  | 221 | jsr.json in full, and package.json's devDependencies | 5.10 |  | 0.561 |
| walker |  | 9919 | 109 | Code::CodeKey { rung: Doc, file: src/patterns.ts, decl: 34, sub: 0, line: 1211 } |  |  | 0.561 |
| ns | 9964 |  | 121 | `.prettierrc` in full, and the benchmark runner scripts | 5.11 |  | 0.558 |
