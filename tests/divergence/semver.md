Score(3000)=0.706 I=0.904 C=0.552 ns_rows≤3K=16/48 grid(1000/1442/2080/3000/4327/6240/9000)=0.659/0.758/0.798/0.706/0.650/0.611/0.542

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
| ns | 909 |  | 197 | index.js aggregate export object, remainder (satisfies..rcompareIdentifiers) | 1.8 | 1.7 | 0.658 |
| walker |  | 1080 | 285 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: true } |  |  | 0.661 |
| walker |  | 1107 | 27 | Fs::DirListing { dir: test/internal } |  |  | 0.661 |
| walker |  | 1129 | 22 | Code::CodeKey { rung: Names, file: classes/semver.js, decl: 0, sub: 0, line: 0 } |  |  | 0.661 |
| ns | 1230 |  | 321 | README section map: every heading, no bodies | 1.9 |  | 0.639 |
| walker |  | 1235 | 106 | Code::CodeKey { rung: Decl, file: classes/semver.js, decl: 1, sub: 0, line: 9 } |  |  | 0.641 |
| walker |  | 1244 | 9 | Code::CodeKey { rung: Body, file: classes/semver.js, decl: 4, sub: 0, line: 89 } |  |  | 0.641 |
| walker |  | 1381 | 137 | Json::Entry { file: package.json } |  |  | 0.758 |
| ns | 1491 |  | 261 | range.bnf: the complete formal grammar of range syntax | 1.10 |  | 0.718 |
| walker |  | 1525 | 144 | Json::Scripts { file: package.json } |  |  | 0.718 |
| walker |  | 1537 | 12 | Code::CodeKey { rung: Names, file: classes/index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.718 |
| walker |  | 1579 | 42 | Code::CodeKey { rung: Decl, file: classes/index.js, decl: 1, sub: 0, line: 3 } |  |  | 0.719 |
| walker |  | 1591 | 12 | Code::CodeKey { rung: Names, file: index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.720 |
| ns | 1744 |  | 253 | SemVer class: requires + complete method roster | 2.1 |  | 0.677 |
| ns | 1950 |  | 206 | Comparator class: ANY sentinel, complete method roster, requires | 2.2 |  | 0.637 |
| walker |  | 1971 | 380 | Code::CodeKey { rung: Decl, file: index.js, decl: 1, sub: 0, line: 45 } |  |  | 0.797 |
| walker |  | 2036 | 65 | Code::CodeKey { rung: Body, file: classes/semver.js, decl: 3, sub: 0, line: 81 } |  |  | 0.798 |
| walker |  | 2050 | 14 | Code::CodeKey { rung: Names, file: preload.js, decl: 0, sub: 0, line: 0 } |  |  | 0.798 |
| walker |  | 2053 | 3 | Fs::DirListing { dir: tap-snapshots/test } |  |  | 0.798 |
| walker |  | 2158 | 105 | Fs::DirListing { dir: test/functions } |  |  | 0.798 |
| ns | 2216 |  | 266 | Range class: complete method roster + hoisted require block and LRU cache | 2.3 |  | 0.745 |
| ns | 2537 |  | 321 | range.js: complete roster of module-level range-desugaring helpers | 2.4 |  | 0.697 |
| ns | 2602 |  | 65 | classes/index.js barrel (whole file) | 2.5 |  | 0.698 |
| ns | 2857 |  | 255 | README usage: canonical calls against the aggregate export | 3.1 |  | 0.706 |
| walker |  | 3025 | 867 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: true } |  |  | 0.706 |
| ns | 3065 |  | 208 | README: the options object (`loose`, `includePrerelease`) | 3.2 |  | 0.690 |
| ns | 3171 |  | 106 | README: what counts as a version | 3.3 |  | 0.681 |
| ns | 3331 |  | 160 | README: comparators and the complete primitive operator set | 3.4 |  | 0.667 |
| ns | 3445 |  | 114 | README: comparator sets intersect, `\|\|` unions them | 3.5 |  | 0.659 |
| walker |  | 3588 | 563 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: true } |  |  | 0.659 |
| walker |  | 3609 | 21 | Code::CodeKey { rung: Names, file: map.js, decl: 0, sub: 0, line: 0 } |  |  | 0.659 |
| ns | 3611 |  | 166 | README: the prerelease-tag matching rule | 3.6 |  | 0.647 |
| walker |  | 3613 | 4 | Fs::DirListing { dir: .github/matchers } |  |  | 0.647 |
| walker |  | 3617 | 4 | Fs::DirListing { dir: test/integration } |  |  | 0.647 |
| walker |  | 3673 | 56 | Fs::DirListing { dir: test/ranges } |  |  | 0.647 |
| walker |  | 3779 | 106 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.662 |
| walker |  | 3800 | 21 | Code::CodeKey { rung: Names, file: internal/debug.js, decl: 0, sub: 0, line: 0 } |  |  | 0.662 |
| ns | 3849 |  | 238 | README: the `inc` contract and the eight release types | 3.7 |  | 0.648 |
| walker |  | 3873 | 73 | Code::CodeKey { rung: Decl, file: internal/debug.js, decl: 1, sub: 0, line: 3 } |  |  | 0.649 |
| walker |  | 3895 | 22 | Code::CodeKey { rung: Names, file: classes/comparator.js, decl: 0, sub: 0, line: 0 } |  |  | 0.649 |
| walker |  | 3973 | 78 | Code::CodeKey { rung: Decl, file: classes/comparator.js, decl: 1, sub: 0, line: 5 } |  |  | 0.655 |
| walker |  | 3981 | 8 | Code::CodeKey { rung: Body, file: classes/comparator.js, decl: 2, sub: 0, line: 6 } |  |  | 0.655 |
| walker |  | 3990 | 9 | Code::CodeKey { rung: Body, file: classes/comparator.js, decl: 5, sub: 0, line: 57 } |  |  | 0.656 |
| walker |  | 4012 | 22 | Code::CodeKey { rung: Names, file: classes/range.js, decl: 0, sub: 0, line: 0 } |  |  | 0.656 |
| walker |  | 4100 | 88 | Code::CodeKey { rung: Decl, file: classes/range.js, decl: 1, sub: 0, line: 6 } |  |  | 0.661 |
| walker |  | 4109 | 9 | Code::CodeKey { rung: Body, file: classes/range.js, decl: 4, sub: 0, line: 92 } |  |  | 0.662 |
| walker |  | 4118 | 9 | Code::CodeKey { rung: Body, file: classes/range.js, decl: 5, sub: 0, line: 96 } |  |  | 0.663 |
| ns | 4177 |  | 328 | README: caret ranges (the left-most non-zero rule) and its desugaring table | 3.8 |  | 0.648 |
| walker |  | 4202 | 84 | Fs::DirListing { dir: test/fixtures } |  |  | 0.649 |
| walker |  | 4207 | 5 | Fs::DirListing { dir: test/bin } |  |  | 0.649 |
| walker |  | 4231 | 24 | Code::CodeKey { rung: Names, file: internal/lrucache.js, decl: 0, sub: 0, line: 0 } |  |  | 0.649 |
| walker |  | 4282 | 51 | Code::CodeKey { rung: Decl, file: internal/lrucache.js, decl: 1, sub: 0, line: 3 } |  |  | 0.649 |
| walker |  | 4293 | 11 | Code::CodeKey { rung: Body, file: internal/lrucache.js, decl: 4, sub: 0, line: 21 } |  |  | 0.650 |
| walker |  | 4316 | 23 | Code::CodeKey { rung: Body, file: internal/lrucache.js, decl: 2, sub: 0, line: 4 } |  |  | 0.650 |
| walker |  | 4341 | 25 | Code::CodeKey { rung: Names, file: internal/parse-options.js, decl: 0, sub: 0, line: 0 } |  |  | 0.650 |
| walker |  | 4416 | 75 | Code::CodeKey { rung: Names, file: internal/identifiers.js, decl: 0, sub: 0, line: 0 } |  |  | 0.650 |
| ns | 4467 |  | 290 | README: tilde ranges and their desugaring table | 3.9 |  | 0.642 |
| walker |  | 4560 | 144 | Code::CodeKey { rung: Body, file: classes/semver.js, decl: 5, sub: 0, line: 93 } |  |  | 0.643 |
| walker |  | 4587 | 27 | Code::CodeKey { rung: Names, file: functions/clean.js, decl: 0, sub: 0, line: 0 } |  |  | 0.643 |
| walker |  | 4623 | 36 | Code::CodeKey { rung: Body, file: functions/clean.js, decl: 1, sub: 0, line: 4 } |  |  | 0.643 |
| walker |  | 4650 | 27 | Code::CodeKey { rung: Names, file: functions/valid.js, decl: 0, sub: 0, line: 0 } |  |  | 0.643 |
| walker |  | 4676 | 26 | Code::CodeKey { rung: Body, file: functions/valid.js, decl: 1, sub: 0, line: 4 } |  |  | 0.643 |
| ns | 4731 |  | 264 | README: X-ranges and bare partial versions | 3.10 |  | 0.632 |
| walker |  | 4802 | 126 | Code::CodeKey { rung: Names, file: internal/re.js, decl: 0, sub: 0, line: 0 } |  |  | 0.632 |
| walker |  | 4830 | 28 | Code::CodeKey { rung: Names, file: functions/compare.js, decl: 0, sub: 0, line: 0 } |  |  | 0.632 |
| walker |  | 4848 | 18 | Code::CodeKey { rung: Decl, file: functions/compare.js, decl: 1, sub: 0, line: 4 } |  |  | 0.632 |
| walker |  | 4877 | 29 | Code::CodeKey { rung: Names, file: functions/coerce.js, decl: 0, sub: 0, line: 0 } |  |  | 0.632 |
| walker |  | 4906 | 29 | Code::CodeKey { rung: Names, file: functions/diff.js, decl: 0, sub: 0, line: 0 } |  |  | 0.632 |
| ns | 4922 |  | 191 | README: hyphen ranges | 3.11 |  | 0.622 |
| walker |  | 4935 | 29 | Code::CodeKey { rung: Names, file: functions/prerelease.js, decl: 0, sub: 0, line: 0 } |  |  | 0.622 |
| walker |  | 4971 | 36 | Code::CodeKey { rung: Body, file: functions/prerelease.js, decl: 1, sub: 0, line: 4 } |  |  | 0.622 |
| walker |  | 5000 | 29 | Code::CodeKey { rung: Names, file: functions/satisfies.js, decl: 0, sub: 0, line: 0 } |  |  | 0.622 |
| walker |  | 5055 | 55 | Code::CodeKey { rung: Body, file: functions/satisfies.js, decl: 1, sub: 0, line: 4 } |  |  | 0.622 |
| walker |  | 5084 | 29 | Code::CodeKey { rung: Names, file: ranges/min-version.js, decl: 0, sub: 0, line: 0 } |  |  | 0.622 |
| ns | 5101 |  | 179 | README: ranges can be non-contiguous (the gtr/ltr/satisfies gotcha) | 3.12 |  | 0.615 |
| walker |  | 5113 | 29 | Code::CodeKey { rung: Names, file: ranges/valid.js, decl: 0, sub: 0, line: 0 } |  |  | 0.615 |
| walker |  | 5143 | 30 | Code::CodeKey { rung: Names, file: ranges/subset.js, decl: 0, sub: 0, line: 0 } |  |  | 0.615 |
| walker |  | 5173 | 30 | Code::CodeKey { rung: Names, file: ranges/to-comparators.js, decl: 0, sub: 0, line: 0 } |  |  | 0.615 |
| walker |  | 5208 | 35 | Code::CodeKey { rung: Decl, file: ranges/to-comparators.js, decl: 1, sub: 0, line: 6 } |  |  | 0.615 |
| ns | 5314 |  | 213 | README: coercion limits | 3.13 |  | 0.607 |
| walker |  | 5316 | 108 | Code::CodeKey { rung: Names, file: internal/constants.js, decl: 0, sub: 0, line: 0 } |  |  | 0.607 |
| walker |  | 5335 | 19 | Code::CodeKey { rung: Decl, file: internal/constants.js, decl: 3, sub: 0, line: 8 } |  |  | 0.607 |
| walker |  | 5399 | 64 | Code::CodeKey { rung: Decl, file: internal/constants.js, decl: 6, sub: 0, line: 18 } |  |  | 0.608 |
| walker |  | 5487 | 88 | Code::CodeKey { rung: Decl, file: internal/constants.js, decl: 7, sub: 0, line: 28 } |  |  | 0.609 |
| walker |  | 5559 | 72 | Code::CodeKey { rung: Body, file: internal/parse-options.js, decl: 1, sub: 0, line: 6 } |  |  | 0.610 |
| ns | 5694 |  | 380 | internal/constants.js (whole file): every tunable limit and flag | 4.1 |  | 0.611 |
| walker |  | 5729 | 170 | Code::CodeKey { rung: Body, file: classes/range.js, decl: 8, sub: 0, line: 193 } |  |  | 0.611 |
| walker |  | 5760 | 31 | Code::CodeKey { rung: Names, file: functions/cmp.js, decl: 0, sub: 0, line: 0 } |  |  | 0.611 |
| walker |  | 5791 | 31 | Code::CodeKey { rung: Names, file: functions/compare-build.js, decl: 0, sub: 0, line: 0 } |  |  | 0.611 |
| walker |  | 5843 | 52 | Code::CodeKey { rung: Body, file: functions/compare-build.js, decl: 1, sub: 0, line: 4 } |  |  | 0.611 |
| walker |  | 5874 | 31 | Code::CodeKey { rung: Names, file: ranges/intersects.js, decl: 0, sub: 0, line: 0 } |  |  | 0.611 |
| walker |  | 5919 | 45 | Code::CodeKey { rung: Body, file: ranges/intersects.js, decl: 1, sub: 0, line: 4 } |  |  | 0.611 |
| walker |  | 5950 | 31 | Code::CodeKey { rung: Names, file: ranges/outside.js, decl: 0, sub: 0, line: 0 } |  |  | 0.611 |
| ns | 5956 |  | 262 | internal/parse-options.js and internal/debug.js (whole files) | 4.2 |  | 0.611 |
| walker |  | 5982 | 32 | Code::CodeKey { rung: Names, file: functions/parse.js, decl: 0, sub: 0, line: 0 } |  |  | 0.611 |
| walker |  | 6015 | 33 | Code::CodeKey { rung: Names, file: functions/compare-loose.js, decl: 0, sub: 0, line: 0 } |  |  | 0.611 |
| walker |  | 6048 | 33 | Code::CodeKey { rung: Names, file: functions/major.js, decl: 0, sub: 0, line: 0 } |  |  | 0.611 |
| walker |  | 6081 | 33 | Code::CodeKey { rung: Names, file: functions/minor.js, decl: 0, sub: 0, line: 0 } |  |  | 0.611 |
| walker |  | 6114 | 33 | Code::CodeKey { rung: Names, file: functions/patch.js, decl: 0, sub: 0, line: 0 } |  |  | 0.611 |
| walker |  | 6193 | 79 | Code::CodeKey { rung: Body, file: ranges/valid.js, decl: 1, sub: 0, line: 4 } |  |  | 0.611 |
| walker |  | 6227 | 34 | Code::CodeKey { rung: Names, file: functions/inc.js, decl: 0, sub: 0, line: 0 } |  |  | 0.611 |
| walker |  | 6262 | 35 | Code::CodeKey { rung: Names, file: functions/rcompare.js, decl: 0, sub: 0, line: 0 } |  |  | 0.594 |
| ns | 6262 |  | 306 | internal/identifiers.js (whole file): the prerelease ordering rule | 4.3 |  | 0.594 |
| walker |  | 6297 | 35 | Code::CodeKey { rung: Names, file: ranges/max-satisfying.js, decl: 0, sub: 0, line: 0 } |  |  | 0.594 |
| walker |  | 6332 | 35 | Code::CodeKey { rung: Names, file: ranges/min-satisfying.js, decl: 0, sub: 0, line: 0 } |  |  | 0.594 |
| ns | 6359 |  | 97 | internal/lrucache.js: class shape and the 1000-entry bound | 4.4 |  | 0.598 |
| walker |  | 6369 | 37 | Code::CodeKey { rung: Names, file: functions/eq.js, decl: 0, sub: 0, line: 0 } |  |  | 0.598 |
| walker |  | 6406 | 37 | Code::CodeKey { rung: Names, file: functions/gt.js, decl: 0, sub: 0, line: 0 } |  |  | 0.598 |
| walker |  | 6443 | 37 | Code::CodeKey { rung: Names, file: functions/lt.js, decl: 0, sub: 0, line: 0 } |  |  | 0.598 |
| walker |  | 6480 | 37 | Code::CodeKey { rung: Names, file: functions/neq.js, decl: 0, sub: 0, line: 0 } |  |  | 0.598 |
| walker |  | 6517 | 37 | Code::CodeKey { rung: Names, file: ranges/gtr.js, decl: 0, sub: 0, line: 0 } |  |  | 0.598 |
| walker |  | 6554 | 37 | Code::CodeKey { rung: Names, file: ranges/ltr.js, decl: 0, sub: 0, line: 0 } |  |  | 0.598 |
| walker |  | 6562 | 8 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.598 |
| ns | 6593 |  | 234 | internal/re.js: the four exported arrays and the createToken registrar | 4.5 |  | 0.589 |
| walker |  | 6601 | 39 | Code::CodeKey { rung: Names, file: functions/gte.js, decl: 0, sub: 0, line: 0 } |  |  | 0.589 |
| walker |  | 6640 | 39 | Code::CodeKey { rung: Names, file: functions/lte.js, decl: 0, sub: 0, line: 0 } |  |  | 0.589 |
| walker |  | 6680 | 40 | Code::CodeKey { rung: Names, file: functions/sort.js, decl: 0, sub: 0, line: 0 } |  |  | 0.589 |
| walker |  | 6700 | 20 | Code::CodeKey { rung: Names, file: ranges/simplify.js, decl: 0, sub: 0, line: 0 } |  |  | 0.589 |
| ns | 6761 |  | 168 | internal/re.js: the ReDoS-safe regex construction | 4.6 |  | 0.582 |
| walker |  | 6796 | 96 | Code::CodeKey { rung: Body, file: internal/lrucache.js, decl: 3, sub: 0, line: 9 } |  |  | 0.583 |
| walker |  | 6893 | 97 | Code::CodeKey { rung: Body, file: functions/parse.js, decl: 1, sub: 0, line: 4 } |  |  | 0.583 |
| walker |  | 6935 | 42 | Code::CodeKey { rung: Names, file: functions/rsort.js, decl: 0, sub: 0, line: 0 } |  |  | 0.583 |
| walker |  | 7120 | 185 | Code::CodeKey { rung: Body, file: classes/range.js, decl: 3, sub: 0, line: 73 } |  |  | 0.584 |
| walker |  | 7131 | 11 | Fs::DirListing { dir: .github/actions } |  |  | 0.584 |
| walker |  | 7135 | 4 | Fs::DirListing { dir: .github/actions/create-check } |  |  | 0.584 |
| walker |  | 7139 | 4 | Fs::DirListing { dir: .github/actions/install-latest-npm } |  |  | 0.584 |
| walker |  | 7352 | 213 | Markdown::Section { file: README.md, section_index: 14, keeps_default_concavity: false } |  |  | 0.602 |
| ns | 7436 |  | 675 | internal/re.js: complete roster of all 43 regex token names | 4.7 |  | 0.576 |
| walker |  | 7488 | 136 | Code::CodeKey { rung: Body, file: functions/inc.js, decl: 1, sub: 0, line: 5 } |  |  | 0.576 |
| walker |  | 7713 | 225 | Code::CodeKey { rung: Body, file: classes/semver.js, decl: 8, sub: 0, line: 168 } |  |  | 0.577 |
| ns | 7755 |  | 319 | bin/semver.js: complete option-flag roster | 5.1 |  | 0.559 |
| walker |  | 7863 | 150 | Code::CodeKey { rung: Body, file: classes/comparator.js, decl: 6, sub: 0, line: 61 } |  |  | 0.560 |
| walker |  | 7871 | 8 | Fs::DirListing { dir: tap-snapshots/test/bin } |  |  | 0.560 |
| ns | 7931 |  | 176 | bin/semver.js: usage line, the `-n` contract, and exit semantics | 5.2 |  | 0.553 |
| walker |  | 8068 | 197 | Code::CodeKey { rung: Body, file: classes/range.js, decl: 7, sub: 0, line: 170 } |  |  | 0.554 |
| ns | 8268 |  | 337 | bin/semver.js: the main() output pipeline | 5.3 |  | 0.542 |
| walker |  | 8330 | 262 | Markdown::Section { file: README.md, section_index: 20, keeps_default_concavity: false } |  |  | 0.542 |
| ns | 8414 |  | 146 | SemVer.inc: complete roster of handled release types | 6.1 | 2.1 | 0.532 |
| walker |  | 8528 | 198 | Code::CodeKey { rung: Body, file: internal/identifiers.js, decl: 1, sub: 0, line: 4 } |  |  | 0.555 |
| ns | 8644 |  | 230 | functions/cmp.js: the complete operator dispatch table | 6.2 |  | 0.545 |
| walker |  | 8661 | 133 | Code::CodeKey { rung: Body, file: internal/lrucache.js, decl: 5, sub: 0, line: 25 } |  |  | 0.546 |
| walker |  | 8860 | 199 | Code::CodeKey { rung: Body, file: ranges/max-satisfying.js, decl: 1, sub: 0, line: 6 } |  |  | 0.546 |
| ns | 8886 |  | 242 | functions/coerce.js: the right-to-left scanning rule | 6.3 |  | 0.542 |
| walker |  | 9059 | 199 | Code::CodeKey { rung: Body, file: ranges/min-satisfying.js, decl: 1, sub: 0, line: 5 } |  |  | 0.542 |
| ns | 9133 |  | 247 | functions/diff.js: the prerelease-to-release special cases | 6.4 |  | 0.534 |
| ns | 9250 |  | 117 | Complete listings of test/ and test/fixtures/ | 7.1 |  | 0.548 |
| walker |  | 9285 | 226 | Code::CodeKey { rung: Body, file: classes/semver.js, decl: 6, sub: 0, line: 109 } |  |  | 0.549 |
| ns | 9400 |  | 150 | map.js + test/map.js: the enforced source-to-test mirror | 7.2 |  | 0.545 |
| walker |  | 9580 | 295 | Code::CodeKey { rung: Body, file: ranges/subset.js, decl: 1, sub: 0, line: 45 } |  |  | 0.545 |
| ns | 9615 |  | 215 | package.json: npm scripts and tap configuration | 7.3 |  | 0.545 |
| ns | 9722 |  | 107 | Complete listings of benchmarks/, .github/ and .github/workflows/ | 7.4 |  | 0.556 |
| ns | 9812 |  | 90 | CONTRIBUTING.md: the rules that would silently fail a PR | 7.5 |  | 0.555 |
| ns | 9951 |  | 139 | .eslintrc.local.js: the constraints on published source | 7.6 |  | 0.551 |
| walker |  | 9990 | 410 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.567 |
