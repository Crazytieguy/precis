Score(3000)=0.706 I=0.904 C=0.551 ns_rows≤3K=16/48 grid(1000/1442/2080/3000/4327/6240/9000)=0.659/0.758/0.797/0.706/0.656/0.617/0.583

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 79 |  | 79 | Package identity: README title + package.json name/version/description/main | 1.1 |  | 0.000 |
| walker |  | 103 | 103 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 129 | 26 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.216 |
| walker |  | 134 | 5 | Fs::DirListing { dir: bin } |  |  | 0.216 |
| walker |  | 151 | 17 | Fs::DirListing { dir: classes } |  |  | 0.219 |
| walker |  | 178 | 27 | Fs::DirListing { dir: internal } |  |  | 0.227 |
| ns | 182 |  | 103 | Complete root directory listing | 1.2 |  | 0.687 |
| walker |  | 239 | 61 | Json::Identity { file: package.json } |  |  | 0.945 |
| walker |  | 295 | 56 | Fs::DirListing { dir: ranges } |  |  | 0.961 |
| ns | 307 |  | 125 | package.json `bin` + `files`: CLI entry point and published surface | 1.3 |  | 0.768 |
| walker |  | 325 | 30 | Json::Runtime { file: package.json } |  |  | 0.768 |
| ns | 356 |  | 49 | Complete listings of classes/, internal/ and bin/ | 1.4 |  | 0.786 |
| walker |  | 430 | 105 | Fs::DirListing { dir: functions } |  |  | 0.820 |
| walker |  | 440 | 10 | Fs::DirListing { dir: tap-snapshots/test/bin } |  |  | 0.820 |
| ns | 461 |  | 105 | Complete listing of functions/ (24 version-level modules) | 1.5 |  | 0.834 |
| walker |  | 468 | 28 | Fs::DirListing { dir: .github } |  |  | 0.834 |
| walker |  | 509 | 41 | Fs::DirListing { dir: .github/workflows } |  |  | 0.835 |
| ns | 517 |  | 56 | Complete listing of ranges/ (11 range-level modules) | 1.6 |  | 0.836 |
| walker |  | 542 | 33 | Fs::DirListing { dir: test } |  |  | 0.837 |
| walker |  | 546 | 4 | Fs::DirListing { dir: test/integration } |  |  | 0.837 |
| walker |  | 551 | 5 | Fs::DirListing { dir: test/bin } |  |  | 0.837 |
| walker |  | 589 | 38 | Fs::DirListing { dir: benchmarks } |  |  | 0.838 |
| walker |  | 606 | 17 | Fs::DirListing { dir: test/classes } |  |  | 0.838 |
| ns | 712 |  | 195 | index.js aggregate export object, first half (parse..Range) | 1.7 |  | 0.719 |
| walker |  | 786 | 180 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.724 |
| walker |  | 811 | 25 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.724 |
| ns | 909 |  | 197 | index.js aggregate export object, remainder (satisfies..rcompareIdentifiers) | 1.8 | 1.7 | 0.658 |
| walker |  | 1096 | 285 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: true } |  |  | 0.661 |
| walker |  | 1118 | 22 | Code::CodeKey { rung: Names, file: classes/semver.js, decl: 0, sub: 0, line: 0 } |  |  | 0.661 |
| walker |  | 1224 | 106 | Code::CodeKey { rung: Decl, file: classes/semver.js, decl: 1, sub: 0, line: 9 } |  |  | 0.662 |
| ns | 1230 |  | 321 | README section map: every heading, no bodies | 1.9 |  | 0.641 |
| walker |  | 1361 | 137 | Json::Entry { file: package.json } |  |  | 0.758 |
| walker |  | 1370 | 9 | Code::CodeKey { rung: Body, file: classes/semver.js, decl: 4, sub: 0, line: 89 } |  |  | 0.758 |
| ns | 1491 |  | 261 | range.bnf: the complete formal grammar of range syntax | 1.10 |  | 0.718 |
| walker |  | 1514 | 144 | Json::Scripts { file: package.json } |  |  | 0.718 |
| walker |  | 1526 | 12 | Code::CodeKey { rung: Names, file: classes/index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.718 |
| walker |  | 1568 | 42 | Code::CodeKey { rung: Decl, file: classes/index.js, decl: 1, sub: 0, line: 3 } |  |  | 0.719 |
| walker |  | 1580 | 12 | Code::CodeKey { rung: Names, file: index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.720 |
| ns | 1744 |  | 253 | SemVer class: requires + complete method roster | 2.1 |  | 0.677 |
| ns | 1950 |  | 206 | Comparator class: ANY sentinel, complete method roster, requires | 2.2 |  | 0.637 |
| walker |  | 1960 | 380 | Code::CodeKey { rung: Decl, file: index.js, decl: 1, sub: 0, line: 45 } |  |  | 0.797 |
| walker |  | 1987 | 27 | Fs::DirListing { dir: test/internal } |  |  | 0.797 |
| walker |  | 2014 | 27 | Code::CodeKey { rung: Names, file: preload.js, decl: 0, sub: 0, line: 0 } |  |  | 0.797 |
| ns | 2216 |  | 266 | Range class: complete method roster + hoisted require block and LRU cache | 2.3 |  | 0.744 |
| ns | 2537 |  | 321 | range.js: complete roster of module-level range-desugaring helpers | 2.4 |  | 0.696 |
| ns | 2602 |  | 65 | classes/index.js barrel (whole file) | 2.5 |  | 0.697 |
| ns | 2857 |  | 255 | README usage: canonical calls against the aggregate export | 3.1 |  | 0.706 |
| walker |  | 2881 | 867 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: true } |  |  | 0.706 |
| ns | 3065 |  | 208 | README: the options object (`loose`, `includePrerelease`) | 3.2 |  | 0.689 |
| ns | 3171 |  | 106 | README: what counts as a version | 3.3 |  | 0.680 |
| ns | 3331 |  | 160 | README: comparators and the complete primitive operator set | 3.4 |  | 0.666 |
| walker |  | 3444 | 563 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: true } |  |  | 0.666 |
| ns | 3445 |  | 114 | README: comparator sets intersect, `\|\|` unions them | 3.5 |  | 0.658 |
| walker |  | 3465 | 21 | Code::CodeKey { rung: Names, file: map.js, decl: 0, sub: 0, line: 0 } |  |  | 0.658 |
| walker |  | 3571 | 106 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.673 |
| ns | 3611 |  | 166 | README: the prerelease-tag matching rule | 3.6 |  | 0.661 |
| walker |  | 3676 | 105 | Fs::DirListing { dir: test/functions } |  |  | 0.661 |
| walker |  | 3680 | 4 | Fs::DirListing { dir: .github/matchers } |  |  | 0.661 |
| walker |  | 3702 | 22 | Code::CodeKey { rung: Names, file: classes/range.js, decl: 0, sub: 0, line: 0 } |  |  | 0.661 |
| walker |  | 3790 | 88 | Code::CodeKey { rung: Decl, file: classes/range.js, decl: 1, sub: 0, line: 6 } |  |  | 0.667 |
| walker |  | 3802 | 12 | Code::CodeKey { rung: Doc, file: classes/range.js, decl: 1, sub: 0, line: 6 } |  |  | 0.668 |
| walker |  | 3811 | 9 | Code::CodeKey { rung: Body, file: classes/range.js, decl: 4, sub: 0, line: 92 } |  |  | 0.669 |
| walker |  | 3820 | 9 | Code::CodeKey { rung: Body, file: classes/range.js, decl: 5, sub: 0, line: 96 } |  |  | 0.670 |
| walker |  | 3841 | 21 | Code::CodeKey { rung: Doc, file: classes/range.js, decl: 8, sub: 0, line: 193 } |  |  | 0.673 |
| ns | 3849 |  | 238 | README: the `inc` contract and the eight release types | 3.7 |  | 0.660 |
| walker |  | 3863 | 22 | Code::CodeKey { rung: Names, file: classes/comparator.js, decl: 0, sub: 0, line: 0 } |  |  | 0.660 |
| walker |  | 3941 | 78 | Code::CodeKey { rung: Decl, file: classes/comparator.js, decl: 1, sub: 0, line: 5 } |  |  | 0.665 |
| walker |  | 3949 | 8 | Code::CodeKey { rung: Body, file: classes/comparator.js, decl: 2, sub: 0, line: 6 } |  |  | 0.666 |
| walker |  | 3961 | 12 | Code::CodeKey { rung: Doc, file: classes/comparator.js, decl: 1, sub: 0, line: 5 } |  |  | 0.668 |
| walker |  | 4017 | 56 | Fs::DirListing { dir: test/ranges } |  |  | 0.668 |
| walker |  | 4026 | 9 | Code::CodeKey { rung: Body, file: classes/comparator.js, decl: 5, sub: 0, line: 57 } |  |  | 0.669 |
| walker |  | 4110 | 84 | Fs::DirListing { dir: test/fixtures } |  |  | 0.671 |
| ns | 4177 |  | 328 | README: caret ranges (the left-most non-zero rule) and its desugaring table | 3.8 |  | 0.655 |
| walker |  | 4236 | 126 | Code::CodeKey { rung: Names, file: internal/re.js, decl: 0, sub: 0, line: 0 } |  |  | 0.656 |
| walker |  | 4248 | 12 | Code::CodeKey { rung: Doc, file: internal/re.js, decl: 1, sub: 0, line: 12 } |  |  | 0.656 |
| walker |  | 4277 | 29 | Code::CodeKey { rung: Names, file: functions/coerce.js, decl: 0, sub: 0, line: 0 } |  |  | 0.656 |
| walker |  | 4306 | 29 | Code::CodeKey { rung: Names, file: functions/diff.js, decl: 0, sub: 0, line: 0 } |  |  | 0.656 |
| walker |  | 4336 | 30 | Code::CodeKey { rung: Names, file: ranges/subset.js, decl: 0, sub: 0, line: 0 } |  |  | 0.656 |
| walker |  | 4444 | 108 | Code::CodeKey { rung: Names, file: internal/constants.js, decl: 0, sub: 0, line: 0 } |  |  | 0.656 |
| walker |  | 4463 | 19 | Code::CodeKey { rung: Decl, file: internal/constants.js, decl: 3, sub: 0, line: 8 } |  |  | 0.656 |
| ns | 4467 |  | 290 | README: tilde ranges and their desugaring table | 3.9 |  | 0.647 |
| walker |  | 4527 | 64 | Code::CodeKey { rung: Decl, file: internal/constants.js, decl: 6, sub: 0, line: 18 } |  |  | 0.648 |
| walker |  | 4615 | 88 | Code::CodeKey { rung: Decl, file: internal/constants.js, decl: 7, sub: 0, line: 28 } |  |  | 0.649 |
| walker |  | 4626 | 11 | Code::CodeKey { rung: Doc, file: internal/constants.js, decl: 4, sub: 0, line: 12 } |  |  | 0.649 |
| walker |  | 4650 | 24 | Code::CodeKey { rung: Names, file: internal/lrucache.js, decl: 0, sub: 0, line: 0 } |  |  | 0.649 |
| walker |  | 4701 | 51 | Code::CodeKey { rung: Decl, file: internal/lrucache.js, decl: 1, sub: 0, line: 3 } |  |  | 0.650 |
| walker |  | 4712 | 11 | Code::CodeKey { rung: Body, file: internal/lrucache.js, decl: 4, sub: 0, line: 21 } |  |  | 0.650 |
| ns | 4731 |  | 264 | README: X-ranges and bare partial versions | 3.10 |  | 0.639 |
| walker |  | 4787 | 75 | Code::CodeKey { rung: Names, file: internal/identifiers.js, decl: 0, sub: 0, line: 0 } |  |  | 0.639 |
| walker |  | 4812 | 25 | Code::CodeKey { rung: Names, file: internal/parse-options.js, decl: 0, sub: 0, line: 0 } |  |  | 0.639 |
| walker |  | 4833 | 21 | Code::CodeKey { rung: Names, file: internal/debug.js, decl: 0, sub: 0, line: 0 } |  |  | 0.639 |
| walker |  | 4906 | 73 | Code::CodeKey { rung: Decl, file: internal/debug.js, decl: 1, sub: 0, line: 3 } |  |  | 0.640 |
| ns | 4922 |  | 191 | README: hyphen ranges | 3.11 |  | 0.630 |
| walker |  | 4937 | 31 | Code::CodeKey { rung: Names, file: functions/cmp.js, decl: 0, sub: 0, line: 0 } |  |  | 0.630 |
| walker |  | 4968 | 31 | Code::CodeKey { rung: Names, file: ranges/outside.js, decl: 0, sub: 0, line: 0 } |  |  | 0.630 |
| walker |  | 4997 | 29 | Code::CodeKey { rung: Names, file: ranges/min-version.js, decl: 0, sub: 0, line: 0 } |  |  | 0.630 |
| walker |  | 5040 | 43 | Code::CodeKey { rung: Doc, file: classes/semver.js, decl: 9, sub: 0, line: 194 } |  |  | 0.637 |
| walker |  | 5074 | 34 | Code::CodeKey { rung: Names, file: functions/inc.js, decl: 0, sub: 0, line: 0 } |  |  | 0.637 |
| ns | 5101 |  | 179 | README: ranges can be non-contiguous (the gtr/ltr/satisfies gotcha) | 3.12 |  | 0.630 |
| walker |  | 5106 | 32 | Code::CodeKey { rung: Names, file: functions/parse.js, decl: 0, sub: 0, line: 0 } |  |  | 0.630 |
| walker |  | 5137 | 31 | Code::CodeKey { rung: Names, file: functions/compare-build.js, decl: 0, sub: 0, line: 0 } |  |  | 0.630 |
| walker |  | 5166 | 29 | Code::CodeKey { rung: Names, file: functions/satisfies.js, decl: 0, sub: 0, line: 0 } |  |  | 0.630 |
| walker |  | 5195 | 29 | Code::CodeKey { rung: Names, file: functions/prerelease.js, decl: 0, sub: 0, line: 0 } |  |  | 0.630 |
| walker |  | 5222 | 27 | Code::CodeKey { rung: Names, file: functions/clean.js, decl: 0, sub: 0, line: 0 } |  |  | 0.630 |
| walker |  | 5249 | 27 | Code::CodeKey { rung: Names, file: functions/valid.js, decl: 0, sub: 0, line: 0 } |  |  | 0.630 |
| walker |  | 5277 | 28 | Code::CodeKey { rung: Names, file: functions/compare.js, decl: 0, sub: 0, line: 0 } |  |  | 0.630 |
| walker |  | 5295 | 18 | Code::CodeKey { rung: Body, file: functions/compare.js, decl: 1, sub: 0, line: 4 } |  |  | 0.630 |
| ns | 5314 |  | 213 | README: coercion limits | 3.13 |  | 0.621 |
| walker |  | 5315 | 20 | Code::CodeKey { rung: Names, file: ranges/simplify.js, decl: 0, sub: 0, line: 0 } |  |  | 0.621 |
| walker |  | 5350 | 35 | Code::CodeKey { rung: Names, file: ranges/max-satisfying.js, decl: 0, sub: 0, line: 0 } |  |  | 0.621 |
| walker |  | 5385 | 35 | Code::CodeKey { rung: Names, file: ranges/min-satisfying.js, decl: 0, sub: 0, line: 0 } |  |  | 0.621 |
| walker |  | 5414 | 29 | Code::CodeKey { rung: Names, file: ranges/valid.js, decl: 0, sub: 0, line: 0 } |  |  | 0.621 |
| walker |  | 5444 | 30 | Code::CodeKey { rung: Names, file: ranges/to-comparators.js, decl: 0, sub: 0, line: 0 } |  |  | 0.621 |
| walker |  | 5458 | 14 | Code::CodeKey { rung: Doc, file: ranges/to-comparators.js, decl: 1, sub: 0, line: 6 } |  |  | 0.621 |
| walker |  | 5495 | 37 | Code::CodeKey { rung: Names, file: ranges/gtr.js, decl: 0, sub: 0, line: 0 } |  |  | 0.621 |
| walker |  | 5532 | 37 | Code::CodeKey { rung: Names, file: ranges/ltr.js, decl: 0, sub: 0, line: 0 } |  |  | 0.621 |
| walker |  | 5563 | 31 | Code::CodeKey { rung: Names, file: ranges/intersects.js, decl: 0, sub: 0, line: 0 } |  |  | 0.621 |
| walker |  | 5582 | 19 | Code::CodeKey { rung: Doc, file: ranges/ltr.js, decl: 1, sub: 0, line: 5 } |  |  | 0.621 |
| walker |  | 5624 | 42 | Code::CodeKey { rung: Names, file: functions/rsort.js, decl: 0, sub: 0, line: 0 } |  |  | 0.621 |
| walker |  | 5664 | 40 | Code::CodeKey { rung: Names, file: functions/sort.js, decl: 0, sub: 0, line: 0 } |  |  | 0.621 |
| ns | 5694 |  | 380 | internal/constants.js (whole file): every tunable limit and flag | 4.1 |  | 0.624 |
| walker |  | 5697 | 33 | Code::CodeKey { rung: Names, file: functions/major.js, decl: 0, sub: 0, line: 0 } |  |  | 0.624 |
| walker |  | 5730 | 33 | Code::CodeKey { rung: Names, file: functions/minor.js, decl: 0, sub: 0, line: 0 } |  |  | 0.624 |
| walker |  | 5763 | 33 | Code::CodeKey { rung: Names, file: functions/patch.js, decl: 0, sub: 0, line: 0 } |  |  | 0.624 |
| walker |  | 5796 | 33 | Code::CodeKey { rung: Names, file: functions/compare-loose.js, decl: 0, sub: 0, line: 0 } |  |  | 0.624 |
| walker |  | 5831 | 35 | Code::CodeKey { rung: Names, file: functions/rcompare.js, decl: 0, sub: 0, line: 0 } |  |  | 0.624 |
| walker |  | 5868 | 37 | Code::CodeKey { rung: Names, file: functions/neq.js, decl: 0, sub: 0, line: 0 } |  |  | 0.624 |
| walker |  | 5907 | 39 | Code::CodeKey { rung: Names, file: functions/gte.js, decl: 0, sub: 0, line: 0 } |  |  | 0.624 |
| walker |  | 5946 | 39 | Code::CodeKey { rung: Names, file: functions/lte.js, decl: 0, sub: 0, line: 0 } |  |  | 0.624 |
| ns | 5956 |  | 262 | internal/parse-options.js and internal/debug.js (whole files) | 4.2 |  | 0.611 |
| walker |  | 5983 | 37 | Code::CodeKey { rung: Names, file: functions/eq.js, decl: 0, sub: 0, line: 0 } |  |  | 0.611 |
| walker |  | 6020 | 37 | Code::CodeKey { rung: Names, file: functions/gt.js, decl: 0, sub: 0, line: 0 } |  |  | 0.611 |
| walker |  | 6057 | 37 | Code::CodeKey { rung: Names, file: functions/lt.js, decl: 0, sub: 0, line: 0 } |  |  | 0.611 |
| walker |  | 6065 | 8 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.611 |
| walker |  | 6091 | 26 | Code::CodeKey { rung: Body, file: functions/valid.js, decl: 1, sub: 0, line: 4 } |  |  | 0.611 |
| walker |  | 6114 | 23 | Code::CodeKey { rung: Body, file: internal/lrucache.js, decl: 2, sub: 0, line: 4 } |  |  | 0.611 |
| ns | 6262 |  | 306 | internal/identifiers.js (whole file): the prerelease ordering rule | 4.3 |  | 0.595 |
| walker |  | 6327 | 213 | Markdown::Section { file: README.md, section_index: 14, keeps_default_concavity: false } |  |  | 0.615 |
| walker |  | 6338 | 11 | Fs::DirListing { dir: .github/actions } |  |  | 0.615 |
| walker |  | 6342 | 4 | Fs::DirListing { dir: .github/actions/create-check } |  |  | 0.615 |
| walker |  | 6346 | 4 | Fs::DirListing { dir: .github/actions/install-latest-npm } |  |  | 0.615 |
| ns | 6359 |  | 97 | internal/lrucache.js: class shape and the 1000-entry bound | 4.4 |  | 0.617 |
| walker |  | 6381 | 35 | Code::CodeKey { rung: Doc, file: internal/constants.js, decl: 1, sub: 0, line: 5 } |  |  | 0.625 |
| walker |  | 6417 | 36 | Code::CodeKey { rung: Body, file: functions/clean.js, decl: 1, sub: 0, line: 4 } |  |  | 0.625 |
| walker |  | 6453 | 36 | Code::CodeKey { rung: Body, file: functions/prerelease.js, decl: 1, sub: 0, line: 4 } |  |  | 0.625 |
| walker |  | 6488 | 35 | Code::CodeKey { rung: Body, file: ranges/to-comparators.js, decl: 1, sub: 0, line: 6 } |  |  | 0.625 |
| walker |  | 6528 | 40 | Code::CodeKey { rung: Doc, file: internal/constants.js, decl: 5, sub: 0, line: 16 } |  |  | 0.635 |
| ns | 6593 |  | 234 | internal/re.js: the four exported arrays and the createToken registrar | 4.5 |  | 0.626 |
| ns | 6761 |  | 168 | internal/re.js: the ReDoS-safe regex construction | 4.6 |  | 0.619 |
| walker |  | 6790 | 262 | Markdown::Section { file: README.md, section_index: 20, keeps_default_concavity: false } |  |  | 0.619 |
| walker |  | 6855 | 65 | Code::CodeKey { rung: Body, file: classes/semver.js, decl: 3, sub: 0, line: 81 } |  |  | 0.620 |
| walker |  | 6900 | 45 | Code::CodeKey { rung: Body, file: ranges/intersects.js, decl: 1, sub: 0, line: 4 } |  |  | 0.620 |
| walker |  | 6952 | 52 | Code::CodeKey { rung: Body, file: functions/compare-build.js, decl: 1, sub: 0, line: 4 } |  |  | 0.620 |
| walker |  | 7007 | 55 | Code::CodeKey { rung: Body, file: functions/satisfies.js, decl: 1, sub: 0, line: 4 } |  |  | 0.620 |
| ns | 7436 |  | 675 | internal/re.js: complete roster of all 43 regex token names | 4.7 |  | 0.593 |
| walker |  | 7438 | 431 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.618 |
| ns | 7755 |  | 319 | bin/semver.js: complete option-flag roster | 5.1 |  | 0.598 |
| walker |  | 7879 | 441 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.601 |
| ns | 7931 |  | 176 | bin/semver.js: usage line, the `-n` contract, and exit semantics | 5.2 |  | 0.594 |
| walker |  | 7951 | 72 | Code::CodeKey { rung: Body, file: internal/parse-options.js, decl: 1, sub: 0, line: 6 } |  |  | 0.604 |
| ns | 8268 |  | 337 | bin/semver.js: the main() output pipeline | 5.3 |  | 0.591 |
| ns | 8414 |  | 146 | SemVer.inc: complete roster of handled release types | 6.1 | 2.1 | 0.581 |
| walker |  | 8422 | 471 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.593 |
| ns | 8644 |  | 230 | functions/cmp.js: the complete operator dispatch table | 6.2 |  | 0.583 |
| ns | 8886 |  | 242 | functions/coerce.js: the right-to-left scanning rule | 6.3 |  | 0.578 |
| walker |  | 8891 | 469 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.580 |
| ns | 9133 |  | 247 | functions/diff.js: the prerelease-to-release special cases | 6.4 |  | 0.572 |
| ns | 9250 |  | 117 | Complete listings of test/ and test/fixtures/ | 7.1 |  | 0.585 |
| walker |  | 9329 | 438 | Markdown::Section { file: README.md, section_index: 10, keeps_default_concavity: false } |  |  | 0.602 |
| ns | 9400 |  | 150 | map.js + test/map.js: the enforced source-to-test mirror | 7.2 |  | 0.598 |
| walker |  | 9408 | 79 | Code::CodeKey { rung: Body, file: ranges/valid.js, decl: 1, sub: 0, line: 4 } |  |  | 0.598 |
| ns | 9615 |  | 215 | package.json: npm scripts and tap configuration | 7.3 |  | 0.597 |
| ns | 9722 |  | 107 | Complete listings of benchmarks/, .github/ and .github/workflows/ | 7.4 |  | 0.606 |
| ns | 9812 |  | 90 | CONTRIBUTING.md: the rules that would silently fail a PR | 7.5 |  | 0.605 |
| walker |  | 9938 | 530 | Markdown::Section { file: README.md, section_index: 15, keeps_default_concavity: false } |  |  | 0.618 |
| ns | 9951 |  | 139 | .eslintrc.local.js: the constraints on published source | 7.6 |  | 0.613 |
| walker |  | 9985 | 47 | Markdown::Section { file: README.md, section_index: 16, keeps_default_concavity: false } |  |  | 0.613 |
