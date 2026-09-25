Score(3000)=0.725 I=0.917 C=0.574 ns_rows≤3K=16/48 grid(1000/1442/2080/3000/4327/6240/9000)=0.659/0.758/0.786/0.725/0.637/0.610/0.541

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 79 |  | 79 | Package identity: README title + package.json name/version/description/main | 1.1 |  | 0.000 |
| walker |  | 103 | 103 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 106 | 3 | Fs::DirListing { dir: tap-snapshots } |  |  | 0.000 |
| walker |  | 111 | 5 | Fs::DirListing { dir: bin } |  |  | 0.000 |
| walker |  | 128 | 17 | Fs::DirListing { dir: classes } |  |  | 0.000 |
| walker |  | 154 | 26 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.219 |
| walker |  | 181 | 27 | Fs::DirListing { dir: internal } |  |  | 0.227 |
| ns | 182 |  | 103 | Complete root directory listing | 1.2 |  | 0.687 |
| walker |  | 242 | 61 | Json::Identity { file: package.json } |  |  | 0.945 |
| walker |  | 298 | 56 | Fs::DirListing { dir: ranges } |  |  | 0.961 |
| ns | 307 |  | 125 | package.json `bin` + `files`: CLI entry point and published surface | 1.3 |  | 0.768 |
| walker |  | 328 | 30 | Json::Runtime { file: package.json } |  |  | 0.768 |
| ns | 356 |  | 49 | Complete listings of classes/, internal/ and bin/ | 1.4 |  | 0.786 |
| walker |  | 433 | 105 | Fs::DirListing { dir: functions } |  |  | 0.820 |
| walker |  | 461 | 28 | Fs::DirListing { dir: .github } |  |  | 0.834 |
| ns | 461 |  | 105 | Complete listing of functions/ (24 version-level modules) | 1.5 |  | 0.834 |
| walker |  | 502 | 41 | Fs::DirListing { dir: .github/workflows } |  |  | 0.835 |
| ns | 517 |  | 56 | Complete listing of ranges/ (11 range-level modules) | 1.6 |  | 0.836 |
| walker |  | 535 | 33 | Fs::DirListing { dir: test } |  |  | 0.837 |
| walker |  | 573 | 38 | Fs::DirListing { dir: benchmarks } |  |  | 0.838 |
| walker |  | 590 | 17 | Fs::DirListing { dir: test/classes } |  |  | 0.838 |
| ns | 712 |  | 195 | index.js aggregate export object, first half (parse..Range) | 1.7 |  | 0.719 |
| walker |  | 770 | 180 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.724 |
| walker |  | 795 | 25 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.724 |
| walker |  | 822 | 27 | Fs::DirListing { dir: test/internal } |  |  | 0.724 |
| ns | 909 |  | 197 | index.js aggregate export object, remainder (satisfies..rcompareIdentifiers) | 1.8 | 1.7 | 0.658 |
| walker |  | 1107 | 285 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: true } |  |  | 0.661 |
| ns | 1230 |  | 321 | README section map: every heading, no bodies | 1.9 |  | 0.639 |
| walker |  | 1244 | 137 | Json::Entry { file: package.json } |  |  | 0.757 |
| walker |  | 1388 | 144 | Json::Scripts { file: package.json } |  |  | 0.757 |
| walker |  | 1400 | 12 | Code::CodeKey { rung: Names, file: classes/index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.757 |
| walker |  | 1442 | 42 | Code::CodeKey { rung: Decl, file: classes/index.js, decl: 1, sub: 0, line: 3 } |  |  | 0.758 |
| walker |  | 1454 | 12 | Code::CodeKey { rung: Names, file: index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.759 |
| ns | 1491 |  | 261 | range.bnf: the complete formal grammar of range syntax | 1.10 |  | 0.718 |
| walker |  | 1512 | 58 | Json::IdentityMeta { file: package.json } |  |  | 0.718 |
| ns | 1744 |  | 253 | SemVer class: requires + complete method roster | 2.1 |  | 0.664 |
| walker |  | 1892 | 380 | Code::CodeKey { rung: Decl, file: index.js, decl: 1, sub: 0, line: 45 } |  |  | 0.836 |
| walker |  | 1906 | 14 | Code::CodeKey { rung: Names, file: preload.js, decl: 0, sub: 0, line: 0 } |  |  | 0.836 |
| walker |  | 1909 | 3 | Fs::DirListing { dir: tap-snapshots/test } |  |  | 0.836 |
| ns | 1950 |  | 206 | Comparator class: ANY sentinel, complete method roster, requires | 2.2 |  | 0.786 |
| walker |  | 2014 | 105 | Fs::DirListing { dir: test/functions } |  |  | 0.786 |
| walker |  | 2035 | 21 | Code::CodeKey { rung: Names, file: map.js, decl: 0, sub: 0, line: 0 } |  |  | 0.786 |
| walker |  | 2039 | 4 | Fs::DirListing { dir: .github/matchers } |  |  | 0.786 |
| walker |  | 2043 | 4 | Fs::DirListing { dir: test/integration } |  |  | 0.786 |
| walker |  | 2099 | 56 | Fs::DirListing { dir: test/ranges } |  |  | 0.786 |
| walker |  | 2120 | 21 | Code::CodeKey { rung: Names, file: internal/debug.js, decl: 0, sub: 0, line: 0 } |  |  | 0.786 |
| walker |  | 2193 | 73 | Code::CodeKey { rung: Decl, file: internal/debug.js, decl: 1, sub: 0, line: 3 } |  |  | 0.787 |
| walker |  | 2215 | 22 | Code::CodeKey { rung: Names, file: classes/comparator.js, decl: 0, sub: 0, line: 0 } |  |  | 0.787 |
| ns | 2216 |  | 266 | Range class: complete method roster + hoisted require block and LRU cache | 2.3 |  | 0.734 |
| walker |  | 2293 | 78 | Code::CodeKey { rung: Decl, file: classes/comparator.js, decl: 1, sub: 0, line: 5 } |  |  | 0.741 |
| walker |  | 2301 | 8 | Code::CodeKey { rung: Body, file: classes/comparator.js, decl: 2, sub: 0, line: 6 } |  |  | 0.742 |
| walker |  | 2310 | 9 | Code::CodeKey { rung: Body, file: classes/comparator.js, decl: 5, sub: 0, line: 57 } |  |  | 0.743 |
| walker |  | 2332 | 22 | Code::CodeKey { rung: Names, file: classes/range.js, decl: 0, sub: 0, line: 0 } |  |  | 0.744 |
| walker |  | 2420 | 88 | Code::CodeKey { rung: Decl, file: classes/range.js, decl: 1, sub: 0, line: 6 } |  |  | 0.750 |
| walker |  | 2429 | 9 | Code::CodeKey { rung: Body, file: classes/range.js, decl: 4, sub: 0, line: 92 } |  |  | 0.751 |
| walker |  | 2438 | 9 | Code::CodeKey { rung: Body, file: classes/range.js, decl: 5, sub: 0, line: 96 } |  |  | 0.752 |
| walker |  | 2460 | 22 | Code::CodeKey { rung: Names, file: classes/semver.js, decl: 0, sub: 0, line: 0 } |  |  | 0.752 |
| ns | 2537 |  | 321 | range.js: complete roster of module-level range-desugaring helpers | 2.4 |  | 0.704 |
| walker |  | 2566 | 106 | Code::CodeKey { rung: Decl, file: classes/semver.js, decl: 1, sub: 0, line: 9 } |  |  | 0.713 |
| walker |  | 2575 | 9 | Code::CodeKey { rung: Body, file: classes/semver.js, decl: 4, sub: 0, line: 89 } |  |  | 0.714 |
| ns | 2602 |  | 65 | classes/index.js barrel (whole file) | 2.5 |  | 0.714 |
| walker |  | 2659 | 84 | Fs::DirListing { dir: test/fixtures } |  |  | 0.715 |
| walker |  | 2664 | 5 | Fs::DirListing { dir: test/bin } |  |  | 0.715 |
| walker |  | 2688 | 24 | Code::CodeKey { rung: Names, file: internal/lrucache.js, decl: 0, sub: 0, line: 0 } |  |  | 0.715 |
| walker |  | 2739 | 51 | Code::CodeKey { rung: Decl, file: internal/lrucache.js, decl: 1, sub: 0, line: 3 } |  |  | 0.716 |
| walker |  | 2750 | 11 | Code::CodeKey { rung: Body, file: internal/lrucache.js, decl: 4, sub: 0, line: 21 } |  |  | 0.716 |
| walker |  | 2773 | 23 | Code::CodeKey { rung: Body, file: internal/lrucache.js, decl: 2, sub: 0, line: 4 } |  |  | 0.716 |
| walker |  | 2798 | 25 | Code::CodeKey { rung: Names, file: internal/parse-options.js, decl: 0, sub: 0, line: 0 } |  |  | 0.716 |
| walker |  | 2825 | 27 | Code::CodeKey { rung: Names, file: functions/clean.js, decl: 0, sub: 0, line: 0 } |  |  | 0.716 |
| ns | 2857 |  | 255 | README usage: canonical calls against the aggregate export | 3.1 |  | 0.724 |
| walker |  | 2861 | 36 | Code::CodeKey { rung: Body, file: functions/clean.js, decl: 1, sub: 0, line: 4 } |  |  | 0.724 |
| walker |  | 2888 | 27 | Code::CodeKey { rung: Names, file: functions/valid.js, decl: 0, sub: 0, line: 0 } |  |  | 0.724 |
| walker |  | 2914 | 26 | Code::CodeKey { rung: Body, file: functions/valid.js, decl: 1, sub: 0, line: 4 } |  |  | 0.724 |
| walker |  | 2979 | 65 | Code::CodeKey { rung: Body, file: classes/semver.js, decl: 3, sub: 0, line: 81 } |  |  | 0.725 |
| ns | 3065 |  | 208 | README: the options object (`loose`, `includePrerelease`) | 3.2 |  | 0.708 |
| walker |  | 3105 | 126 | Code::CodeKey { rung: Names, file: internal/re.js, decl: 0, sub: 0, line: 0 } |  |  | 0.708 |
| walker |  | 3133 | 28 | Code::CodeKey { rung: Names, file: functions/compare.js, decl: 0, sub: 0, line: 0 } |  |  | 0.708 |
| walker |  | 3151 | 18 | Code::CodeKey { rung: Decl, file: functions/compare.js, decl: 1, sub: 0, line: 4 } |  |  | 0.708 |
| ns | 3171 |  | 106 | README: what counts as a version | 3.3 |  | 0.700 |
| walker |  | 3180 | 29 | Code::CodeKey { rung: Names, file: functions/coerce.js, decl: 0, sub: 0, line: 0 } |  |  | 0.700 |
| walker |  | 3209 | 29 | Code::CodeKey { rung: Names, file: functions/diff.js, decl: 0, sub: 0, line: 0 } |  |  | 0.700 |
| walker |  | 3238 | 29 | Code::CodeKey { rung: Names, file: functions/prerelease.js, decl: 0, sub: 0, line: 0 } |  |  | 0.700 |
| walker |  | 3274 | 36 | Code::CodeKey { rung: Body, file: functions/prerelease.js, decl: 1, sub: 0, line: 4 } |  |  | 0.700 |
| walker |  | 3303 | 29 | Code::CodeKey { rung: Names, file: functions/satisfies.js, decl: 0, sub: 0, line: 0 } |  |  | 0.700 |
| ns | 3331 |  | 160 | README: comparators and the complete primitive operator set | 3.4 |  | 0.685 |
| walker |  | 3358 | 55 | Code::CodeKey { rung: Body, file: functions/satisfies.js, decl: 1, sub: 0, line: 4 } |  |  | 0.685 |
| walker |  | 3387 | 29 | Code::CodeKey { rung: Names, file: ranges/min-version.js, decl: 0, sub: 0, line: 0 } |  |  | 0.685 |
| walker |  | 3416 | 29 | Code::CodeKey { rung: Names, file: ranges/valid.js, decl: 0, sub: 0, line: 0 } |  |  | 0.685 |
| ns | 3445 |  | 114 | README: comparator sets intersect, `\|\|` unions them | 3.5 |  | 0.677 |
| ns | 3611 |  | 166 | README: the prerelease-tag matching rule | 3.6 |  | 0.665 |
| ns | 3849 |  | 238 | README: the `inc` contract and the eight release types | 3.7 |  | 0.651 |
| ns | 4177 |  | 328 | README: caret ranges (the left-most non-zero rule) and its desugaring table | 3.8 |  | 0.637 |
| walker |  | 4283 | 867 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: true } |  |  | 0.637 |
| ns | 4467 |  | 290 | README: tilde ranges and their desugaring table | 3.9 |  | 0.628 |
| ns | 4731 |  | 264 | README: X-ranges and bare partial versions | 3.10 |  | 0.617 |
| walker |  | 4846 | 563 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: true } |  |  | 0.617 |
| walker |  | 4876 | 30 | Code::CodeKey { rung: Names, file: ranges/subset.js, decl: 0, sub: 0, line: 0 } |  |  | 0.617 |
| walker |  | 4906 | 30 | Code::CodeKey { rung: Names, file: ranges/to-comparators.js, decl: 0, sub: 0, line: 0 } |  |  | 0.617 |
| ns | 4922 |  | 191 | README: hyphen ranges | 3.11 |  | 0.608 |
| walker |  | 4941 | 35 | Code::CodeKey { rung: Decl, file: ranges/to-comparators.js, decl: 1, sub: 0, line: 6 } |  |  | 0.608 |
| walker |  | 5049 | 108 | Code::CodeKey { rung: Names, file: internal/constants.js, decl: 0, sub: 0, line: 0 } |  |  | 0.608 |
| walker |  | 5068 | 19 | Code::CodeKey { rung: Decl, file: internal/constants.js, decl: 3, sub: 0, line: 8 } |  |  | 0.608 |
| ns | 5101 |  | 179 | README: ranges can be non-contiguous (the gtr/ltr/satisfies gotcha) | 3.12 |  | 0.602 |
| walker |  | 5132 | 64 | Code::CodeKey { rung: Decl, file: internal/constants.js, decl: 6, sub: 0, line: 18 } |  |  | 0.602 |
| walker |  | 5220 | 88 | Code::CodeKey { rung: Decl, file: internal/constants.js, decl: 7, sub: 0, line: 28 } |  |  | 0.603 |
| walker |  | 5292 | 72 | Code::CodeKey { rung: Body, file: internal/parse-options.js, decl: 1, sub: 0, line: 6 } |  |  | 0.604 |
| ns | 5314 |  | 213 | README: coercion limits | 3.13 |  | 0.596 |
| walker |  | 5462 | 170 | Code::CodeKey { rung: Body, file: classes/range.js, decl: 8, sub: 0, line: 193 } |  |  | 0.597 |
| walker |  | 5493 | 31 | Code::CodeKey { rung: Names, file: functions/cmp.js, decl: 0, sub: 0, line: 0 } |  |  | 0.597 |
| walker |  | 5524 | 31 | Code::CodeKey { rung: Names, file: functions/compare-build.js, decl: 0, sub: 0, line: 0 } |  |  | 0.597 |
| walker |  | 5576 | 52 | Code::CodeKey { rung: Body, file: functions/compare-build.js, decl: 1, sub: 0, line: 4 } |  |  | 0.597 |
| walker |  | 5607 | 31 | Code::CodeKey { rung: Names, file: ranges/intersects.js, decl: 0, sub: 0, line: 0 } |  |  | 0.597 |
| walker |  | 5652 | 45 | Code::CodeKey { rung: Body, file: ranges/intersects.js, decl: 1, sub: 0, line: 4 } |  |  | 0.597 |
| walker |  | 5683 | 31 | Code::CodeKey { rung: Names, file: ranges/outside.js, decl: 0, sub: 0, line: 0 } |  |  | 0.597 |
| ns | 5694 |  | 380 | internal/constants.js (whole file): every tunable limit and flag | 4.1 |  | 0.598 |
| walker |  | 5715 | 32 | Code::CodeKey { rung: Names, file: functions/parse.js, decl: 0, sub: 0, line: 0 } |  |  | 0.598 |
| walker |  | 5821 | 106 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.610 |
| walker |  | 5854 | 33 | Code::CodeKey { rung: Names, file: functions/compare-loose.js, decl: 0, sub: 0, line: 0 } |  |  | 0.610 |
| walker |  | 5887 | 33 | Code::CodeKey { rung: Names, file: functions/major.js, decl: 0, sub: 0, line: 0 } |  |  | 0.610 |
| walker |  | 5920 | 33 | Code::CodeKey { rung: Names, file: functions/minor.js, decl: 0, sub: 0, line: 0 } |  |  | 0.610 |
| walker |  | 5953 | 33 | Code::CodeKey { rung: Names, file: functions/patch.js, decl: 0, sub: 0, line: 0 } |  |  | 0.610 |
| ns | 5956 |  | 262 | internal/parse-options.js and internal/debug.js (whole files) | 4.2 |  | 0.610 |
| walker |  | 6032 | 79 | Code::CodeKey { rung: Body, file: ranges/valid.js, decl: 1, sub: 0, line: 4 } |  |  | 0.610 |
| walker |  | 6066 | 34 | Code::CodeKey { rung: Names, file: functions/inc.js, decl: 0, sub: 0, line: 0 } |  |  | 0.610 |
| walker |  | 6101 | 35 | Code::CodeKey { rung: Names, file: functions/rcompare.js, decl: 0, sub: 0, line: 0 } |  |  | 0.610 |
| walker |  | 6136 | 35 | Code::CodeKey { rung: Names, file: ranges/max-satisfying.js, decl: 0, sub: 0, line: 0 } |  |  | 0.610 |
| walker |  | 6171 | 35 | Code::CodeKey { rung: Names, file: ranges/min-satisfying.js, decl: 0, sub: 0, line: 0 } |  |  | 0.610 |
| walker |  | 6208 | 37 | Code::CodeKey { rung: Names, file: functions/eq.js, decl: 0, sub: 0, line: 0 } |  |  | 0.610 |
| walker |  | 6245 | 37 | Code::CodeKey { rung: Names, file: functions/gt.js, decl: 0, sub: 0, line: 0 } |  |  | 0.610 |
| ns | 6262 |  | 306 | internal/identifiers.js (whole file): the prerelease ordering rule | 4.3 |  | 0.590 |
| walker |  | 6282 | 37 | Code::CodeKey { rung: Names, file: functions/lt.js, decl: 0, sub: 0, line: 0 } |  |  | 0.590 |
| walker |  | 6319 | 37 | Code::CodeKey { rung: Names, file: functions/neq.js, decl: 0, sub: 0, line: 0 } |  |  | 0.590 |
| walker |  | 6356 | 37 | Code::CodeKey { rung: Names, file: ranges/gtr.js, decl: 0, sub: 0, line: 0 } |  |  | 0.590 |
| ns | 6359 |  | 97 | internal/lrucache.js: class shape and the 1000-entry bound | 4.4 |  | 0.594 |
| walker |  | 6393 | 37 | Code::CodeKey { rung: Names, file: ranges/ltr.js, decl: 0, sub: 0, line: 0 } |  |  | 0.594 |
| walker |  | 6401 | 8 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.594 |
| walker |  | 6440 | 39 | Code::CodeKey { rung: Names, file: functions/gte.js, decl: 0, sub: 0, line: 0 } |  |  | 0.594 |
| walker |  | 6479 | 39 | Code::CodeKey { rung: Names, file: functions/lte.js, decl: 0, sub: 0, line: 0 } |  |  | 0.594 |
| walker |  | 6519 | 40 | Code::CodeKey { rung: Names, file: functions/sort.js, decl: 0, sub: 0, line: 0 } |  |  | 0.594 |
| walker |  | 6539 | 20 | Code::CodeKey { rung: Names, file: ranges/simplify.js, decl: 0, sub: 0, line: 0 } |  |  | 0.594 |
| ns | 6593 |  | 234 | internal/re.js: the four exported arrays and the createToken registrar | 4.5 |  | 0.585 |
| walker |  | 6635 | 96 | Code::CodeKey { rung: Body, file: internal/lrucache.js, decl: 3, sub: 0, line: 9 } |  |  | 0.586 |
| walker |  | 6732 | 97 | Code::CodeKey { rung: Body, file: functions/parse.js, decl: 1, sub: 0, line: 4 } |  |  | 0.586 |
| ns | 6761 |  | 168 | internal/re.js: the ReDoS-safe regex construction | 4.6 |  | 0.580 |
| walker |  | 6774 | 42 | Code::CodeKey { rung: Names, file: functions/rsort.js, decl: 0, sub: 0, line: 0 } |  |  | 0.580 |
| walker |  | 6849 | 75 | Code::CodeKey { rung: Names, file: internal/identifiers.js, decl: 0, sub: 0, line: 0 } |  |  | 0.583 |
| walker |  | 7034 | 185 | Code::CodeKey { rung: Body, file: classes/range.js, decl: 3, sub: 0, line: 73 } |  |  | 0.583 |
| walker |  | 7045 | 11 | Fs::DirListing { dir: .github/actions } |  |  | 0.583 |
| walker |  | 7049 | 4 | Fs::DirListing { dir: .github/actions/create-check } |  |  | 0.583 |
| walker |  | 7053 | 4 | Fs::DirListing { dir: .github/actions/install-latest-npm } |  |  | 0.583 |
| walker |  | 7189 | 136 | Code::CodeKey { rung: Body, file: functions/inc.js, decl: 1, sub: 0, line: 5 } |  |  | 0.583 |
| walker |  | 7333 | 144 | Code::CodeKey { rung: Body, file: classes/semver.js, decl: 5, sub: 0, line: 93 } |  |  | 0.584 |
| ns | 7436 |  | 675 | internal/re.js: complete roster of all 43 regex token names | 4.7 |  | 0.559 |
| walker |  | 7483 | 150 | Code::CodeKey { rung: Body, file: classes/comparator.js, decl: 6, sub: 0, line: 61 } |  |  | 0.559 |
| walker |  | 7491 | 8 | Fs::DirListing { dir: tap-snapshots/test/bin } |  |  | 0.559 |
| walker |  | 7688 | 197 | Code::CodeKey { rung: Body, file: classes/range.js, decl: 7, sub: 0, line: 170 } |  |  | 0.560 |
| ns | 7755 |  | 319 | bin/semver.js: complete option-flag roster | 5.1 |  | 0.542 |
| walker |  | 7886 | 198 | Code::CodeKey { rung: Body, file: internal/identifiers.js, decl: 1, sub: 0, line: 4 } |  |  | 0.567 |
| ns | 7931 |  | 176 | bin/semver.js: usage line, the `-n` contract, and exit semantics | 5.2 |  | 0.561 |
| walker |  | 8019 | 133 | Code::CodeKey { rung: Body, file: internal/lrucache.js, decl: 5, sub: 0, line: 25 } |  |  | 0.562 |
| walker |  | 8218 | 199 | Code::CodeKey { rung: Body, file: ranges/max-satisfying.js, decl: 1, sub: 0, line: 6 } |  |  | 0.562 |
| ns | 8268 |  | 337 | bin/semver.js: the main() output pipeline | 5.3 |  | 0.550 |
| ns | 8414 |  | 146 | SemVer.inc: complete roster of handled release types | 6.1 | 2.1 | 0.540 |
| walker |  | 8417 | 199 | Code::CodeKey { rung: Body, file: ranges/min-satisfying.js, decl: 1, sub: 0, line: 5 } |  |  | 0.540 |
| walker |  | 8630 | 213 | Markdown::Section { file: README.md, section_index: 14, keeps_default_concavity: false } |  |  | 0.556 |
| ns | 8644 |  | 230 | functions/cmp.js: the complete operator dispatch table | 6.2 |  | 0.546 |
| ns | 8886 |  | 242 | functions/coerce.js: the right-to-left scanning rule | 6.3 |  | 0.541 |
| walker |  | 8925 | 295 | Code::CodeKey { rung: Body, file: ranges/subset.js, decl: 1, sub: 0, line: 45 } |  |  | 0.541 |
| ns | 9133 |  | 247 | functions/diff.js: the prerelease-to-release special cases | 6.4 |  | 0.534 |
| walker |  | 9187 | 262 | Markdown::Section { file: README.md, section_index: 20, keeps_default_concavity: false } |  |  | 0.534 |
| ns | 9250 |  | 117 | Complete listings of test/ and test/fixtures/ | 7.1 |  | 0.548 |
| walker |  | 9398 | 211 | Code::CodeKey { rung: Body, file: classes/comparator.js, decl: 4, sub: 0, line: 36 } |  |  | 0.548 |
| ns | 9400 |  | 150 | map.js + test/map.js: the enforced source-to-test mirror | 7.2 |  | 0.545 |
| ns | 9615 |  | 215 | package.json: npm scripts and tap configuration | 7.3 |  | 0.545 |
| walker |  | 9623 | 225 | Code::CodeKey { rung: Body, file: classes/semver.js, decl: 8, sub: 0, line: 168 } |  |  | 0.545 |
| ns | 9722 |  | 107 | Complete listings of benchmarks/, .github/ and .github/workflows/ | 7.4 |  | 0.556 |
| ns | 9812 |  | 90 | CONTRIBUTING.md: the rules that would silently fail a PR | 7.5 |  | 0.555 |
| ns | 9951 |  | 139 | .eslintrc.local.js: the constraints on published source | 7.6 |  | 0.551 |
| walker |  | 9985 | 362 | Code::CodeKey { rung: Body, file: functions/cmp.js, decl: 1, sub: 0, line: 10 } |  |  | 0.567 |
