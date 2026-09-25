Score(3000)=0.706 I=0.904 C=0.551 ns_rows≤3K=16/48 grid(1000/1442/2080/3000/4327/6240/9000)=0.659/0.758/0.797/0.706/0.649/0.606/0.555

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
| walker |  | 245 | 3 | Fs::DirListing { dir: tap-snapshots/test } |  |  | 0.945 |
| walker |  | 301 | 56 | Fs::DirListing { dir: ranges } |  |  | 0.961 |
| ns | 307 |  | 125 | package.json `bin` + `files`: CLI entry point and published surface | 1.3 |  | 0.768 |
| walker |  | 331 | 30 | Json::Runtime { file: package.json } |  |  | 0.768 |
| ns | 356 |  | 49 | Complete listings of classes/, internal/ and bin/ | 1.4 |  | 0.786 |
| walker |  | 436 | 105 | Fs::DirListing { dir: functions } |  |  | 0.820 |
| walker |  | 444 | 8 | Fs::DirListing { dir: tap-snapshots/test/bin } |  |  | 0.820 |
| ns | 461 |  | 105 | Complete listing of functions/ (24 version-level modules) | 1.5 |  | 0.834 |
| walker |  | 472 | 28 | Fs::DirListing { dir: .github } |  |  | 0.834 |
| walker |  | 513 | 41 | Fs::DirListing { dir: .github/workflows } |  |  | 0.835 |
| ns | 517 |  | 56 | Complete listing of ranges/ (11 range-level modules) | 1.6 |  | 0.836 |
| walker |  | 546 | 33 | Fs::DirListing { dir: test } |  |  | 0.837 |
| walker |  | 550 | 4 | Fs::DirListing { dir: test/integration } |  |  | 0.837 |
| walker |  | 555 | 5 | Fs::DirListing { dir: test/bin } |  |  | 0.837 |
| walker |  | 593 | 38 | Fs::DirListing { dir: benchmarks } |  |  | 0.838 |
| walker |  | 610 | 17 | Fs::DirListing { dir: test/classes } |  |  | 0.838 |
| ns | 712 |  | 195 | index.js aggregate export object, first half (parse..Range) | 1.7 |  | 0.719 |
| walker |  | 790 | 180 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.724 |
| walker |  | 815 | 25 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.724 |
| ns | 909 |  | 197 | index.js aggregate export object, remainder (satisfies..rcompareIdentifiers) | 1.8 | 1.7 | 0.658 |
| walker |  | 1100 | 285 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: true } |  |  | 0.661 |
| walker |  | 1127 | 27 | Fs::DirListing { dir: test/internal } |  |  | 0.661 |
| walker |  | 1149 | 22 | Code::CodeKey { rung: Names, file: classes/semver.js, decl: 0, sub: 0, line: 0 } |  |  | 0.661 |
| ns | 1230 |  | 321 | README section map: every heading, no bodies | 1.9 |  | 0.639 |
| walker |  | 1255 | 106 | Code::CodeKey { rung: Decl, file: classes/semver.js, decl: 1, sub: 0, line: 9 } |  |  | 0.641 |
| walker |  | 1264 | 9 | Code::CodeKey { rung: Body, file: classes/semver.js, decl: 4, sub: 0, line: 89 } |  |  | 0.641 |
| walker |  | 1401 | 137 | Json::Entry { file: package.json } |  |  | 0.758 |
| ns | 1491 |  | 261 | range.bnf: the complete formal grammar of range syntax | 1.10 |  | 0.718 |
| walker |  | 1545 | 144 | Json::Scripts { file: package.json } |  |  | 0.718 |
| walker |  | 1557 | 12 | Code::CodeKey { rung: Names, file: classes/index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.718 |
| walker |  | 1599 | 42 | Code::CodeKey { rung: Decl, file: classes/index.js, decl: 1, sub: 0, line: 3 } |  |  | 0.719 |
| walker |  | 1611 | 12 | Code::CodeKey { rung: Names, file: index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.720 |
| ns | 1744 |  | 253 | SemVer class: requires + complete method roster | 2.1 |  | 0.677 |
| ns | 1950 |  | 206 | Comparator class: ANY sentinel, complete method roster, requires | 2.2 |  | 0.637 |
| walker |  | 1991 | 380 | Code::CodeKey { rung: Decl, file: index.js, decl: 1, sub: 0, line: 45 } |  |  | 0.797 |
| walker |  | 2005 | 14 | Code::CodeKey { rung: Names, file: preload.js, decl: 0, sub: 0, line: 0 } |  |  | 0.797 |
| walker |  | 2110 | 105 | Fs::DirListing { dir: test/functions } |  |  | 0.797 |
| ns | 2216 |  | 266 | Range class: complete method roster + hoisted require block and LRU cache | 2.3 |  | 0.744 |
| ns | 2537 |  | 321 | range.js: complete roster of module-level range-desugaring helpers | 2.4 |  | 0.696 |
| ns | 2602 |  | 65 | classes/index.js barrel (whole file) | 2.5 |  | 0.697 |
| ns | 2857 |  | 255 | README usage: canonical calls against the aggregate export | 3.1 |  | 0.706 |
| walker |  | 2977 | 867 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: true } |  |  | 0.706 |
| ns | 3065 |  | 208 | README: the options object (`loose`, `includePrerelease`) | 3.2 |  | 0.689 |
| ns | 3171 |  | 106 | README: what counts as a version | 3.3 |  | 0.680 |
| ns | 3331 |  | 160 | README: comparators and the complete primitive operator set | 3.4 |  | 0.666 |
| ns | 3445 |  | 114 | README: comparator sets intersect, `\|\|` unions them | 3.5 |  | 0.658 |
| walker |  | 3540 | 563 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: true } |  |  | 0.658 |
| walker |  | 3561 | 21 | Code::CodeKey { rung: Names, file: map.js, decl: 0, sub: 0, line: 0 } |  |  | 0.658 |
| walker |  | 3565 | 4 | Fs::DirListing { dir: .github/matchers } |  |  | 0.658 |
| ns | 3611 |  | 166 | README: the prerelease-tag matching rule | 3.6 |  | 0.647 |
| walker |  | 3621 | 56 | Fs::DirListing { dir: test/ranges } |  |  | 0.647 |
| walker |  | 3727 | 106 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.661 |
| walker |  | 3748 | 21 | Code::CodeKey { rung: Names, file: internal/debug.js, decl: 0, sub: 0, line: 0 } |  |  | 0.661 |
| walker |  | 3821 | 73 | Code::CodeKey { rung: Decl, file: internal/debug.js, decl: 1, sub: 0, line: 3 } |  |  | 0.662 |
| walker |  | 3843 | 22 | Code::CodeKey { rung: Names, file: classes/comparator.js, decl: 0, sub: 0, line: 0 } |  |  | 0.662 |
| ns | 3849 |  | 238 | README: the `inc` contract and the eight release types | 3.7 |  | 0.648 |
| walker |  | 3921 | 78 | Code::CodeKey { rung: Decl, file: classes/comparator.js, decl: 1, sub: 0, line: 5 } |  |  | 0.654 |
| walker |  | 3929 | 8 | Code::CodeKey { rung: Body, file: classes/comparator.js, decl: 2, sub: 0, line: 6 } |  |  | 0.655 |
| walker |  | 3938 | 9 | Code::CodeKey { rung: Body, file: classes/comparator.js, decl: 5, sub: 0, line: 57 } |  |  | 0.655 |
| walker |  | 3960 | 22 | Code::CodeKey { rung: Names, file: classes/range.js, decl: 0, sub: 0, line: 0 } |  |  | 0.656 |
| walker |  | 4048 | 88 | Code::CodeKey { rung: Decl, file: classes/range.js, decl: 1, sub: 0, line: 6 } |  |  | 0.661 |
| walker |  | 4057 | 9 | Code::CodeKey { rung: Body, file: classes/range.js, decl: 4, sub: 0, line: 92 } |  |  | 0.661 |
| walker |  | 4066 | 9 | Code::CodeKey { rung: Body, file: classes/range.js, decl: 5, sub: 0, line: 96 } |  |  | 0.662 |
| walker |  | 4150 | 84 | Fs::DirListing { dir: test/fixtures } |  |  | 0.663 |
| walker |  | 4174 | 24 | Code::CodeKey { rung: Names, file: internal/lrucache.js, decl: 0, sub: 0, line: 0 } |  |  | 0.663 |
| ns | 4177 |  | 328 | README: caret ranges (the left-most non-zero rule) and its desugaring table | 3.8 |  | 0.648 |
| walker |  | 4225 | 51 | Code::CodeKey { rung: Decl, file: internal/lrucache.js, decl: 1, sub: 0, line: 3 } |  |  | 0.649 |
| walker |  | 4236 | 11 | Code::CodeKey { rung: Body, file: internal/lrucache.js, decl: 4, sub: 0, line: 21 } |  |  | 0.649 |
| walker |  | 4261 | 25 | Code::CodeKey { rung: Names, file: internal/parse-options.js, decl: 0, sub: 0, line: 0 } |  |  | 0.649 |
| walker |  | 4336 | 75 | Code::CodeKey { rung: Names, file: internal/identifiers.js, decl: 0, sub: 0, line: 0 } |  |  | 0.649 |
| walker |  | 4363 | 27 | Code::CodeKey { rung: Names, file: functions/clean.js, decl: 0, sub: 0, line: 0 } |  |  | 0.649 |
| walker |  | 4390 | 27 | Code::CodeKey { rung: Names, file: functions/valid.js, decl: 0, sub: 0, line: 0 } |  |  | 0.649 |
| ns | 4467 |  | 290 | README: tilde ranges and their desugaring table | 3.9 |  | 0.641 |
| walker |  | 4516 | 126 | Code::CodeKey { rung: Names, file: internal/re.js, decl: 0, sub: 0, line: 0 } |  |  | 0.641 |
| walker |  | 4544 | 28 | Code::CodeKey { rung: Names, file: functions/compare.js, decl: 0, sub: 0, line: 0 } |  |  | 0.641 |
| walker |  | 4562 | 18 | Code::CodeKey { rung: Decl, file: functions/compare.js, decl: 1, sub: 0, line: 4 } |  |  | 0.641 |
| walker |  | 4591 | 29 | Code::CodeKey { rung: Names, file: functions/coerce.js, decl: 0, sub: 0, line: 0 } |  |  | 0.641 |
| walker |  | 4620 | 29 | Code::CodeKey { rung: Names, file: functions/diff.js, decl: 0, sub: 0, line: 0 } |  |  | 0.641 |
| walker |  | 4649 | 29 | Code::CodeKey { rung: Names, file: functions/prerelease.js, decl: 0, sub: 0, line: 0 } |  |  | 0.641 |
| walker |  | 4678 | 29 | Code::CodeKey { rung: Names, file: functions/satisfies.js, decl: 0, sub: 0, line: 0 } |  |  | 0.641 |
| walker |  | 4707 | 29 | Code::CodeKey { rung: Names, file: ranges/min-version.js, decl: 0, sub: 0, line: 0 } |  |  | 0.641 |
| ns | 4731 |  | 264 | README: X-ranges and bare partial versions | 3.10 |  | 0.630 |
| walker |  | 4736 | 29 | Code::CodeKey { rung: Names, file: ranges/valid.js, decl: 0, sub: 0, line: 0 } |  |  | 0.630 |
| walker |  | 4766 | 30 | Code::CodeKey { rung: Names, file: ranges/subset.js, decl: 0, sub: 0, line: 0 } |  |  | 0.630 |
| walker |  | 4796 | 30 | Code::CodeKey { rung: Names, file: ranges/to-comparators.js, decl: 0, sub: 0, line: 0 } |  |  | 0.630 |
| walker |  | 4831 | 35 | Code::CodeKey { rung: Decl, file: ranges/to-comparators.js, decl: 1, sub: 0, line: 6 } |  |  | 0.630 |
| ns | 4922 |  | 191 | README: hyphen ranges | 3.11 |  | 0.620 |
| walker |  | 4939 | 108 | Code::CodeKey { rung: Names, file: internal/constants.js, decl: 0, sub: 0, line: 0 } |  |  | 0.621 |
| walker |  | 4958 | 19 | Code::CodeKey { rung: Decl, file: internal/constants.js, decl: 3, sub: 0, line: 8 } |  |  | 0.621 |
| walker |  | 5022 | 64 | Code::CodeKey { rung: Decl, file: internal/constants.js, decl: 6, sub: 0, line: 18 } |  |  | 0.621 |
| ns | 5101 |  | 179 | README: ranges can be non-contiguous (the gtr/ltr/satisfies gotcha) | 3.12 |  | 0.614 |
| walker |  | 5110 | 88 | Code::CodeKey { rung: Decl, file: internal/constants.js, decl: 7, sub: 0, line: 28 } |  |  | 0.615 |
| walker |  | 5141 | 31 | Code::CodeKey { rung: Names, file: functions/cmp.js, decl: 0, sub: 0, line: 0 } |  |  | 0.615 |
| walker |  | 5172 | 31 | Code::CodeKey { rung: Names, file: functions/compare-build.js, decl: 0, sub: 0, line: 0 } |  |  | 0.615 |
| walker |  | 5203 | 31 | Code::CodeKey { rung: Names, file: ranges/intersects.js, decl: 0, sub: 0, line: 0 } |  |  | 0.615 |
| walker |  | 5234 | 31 | Code::CodeKey { rung: Names, file: ranges/outside.js, decl: 0, sub: 0, line: 0 } |  |  | 0.615 |
| walker |  | 5266 | 32 | Code::CodeKey { rung: Names, file: functions/parse.js, decl: 0, sub: 0, line: 0 } |  |  | 0.615 |
| walker |  | 5299 | 33 | Code::CodeKey { rung: Names, file: functions/compare-loose.js, decl: 0, sub: 0, line: 0 } |  |  | 0.615 |
| ns | 5314 |  | 213 | README: coercion limits | 3.13 |  | 0.607 |
| walker |  | 5332 | 33 | Code::CodeKey { rung: Names, file: functions/major.js, decl: 0, sub: 0, line: 0 } |  |  | 0.607 |
| walker |  | 5365 | 33 | Code::CodeKey { rung: Names, file: functions/minor.js, decl: 0, sub: 0, line: 0 } |  |  | 0.607 |
| walker |  | 5398 | 33 | Code::CodeKey { rung: Names, file: functions/patch.js, decl: 0, sub: 0, line: 0 } |  |  | 0.607 |
| walker |  | 5432 | 34 | Code::CodeKey { rung: Names, file: functions/inc.js, decl: 0, sub: 0, line: 0 } |  |  | 0.607 |
| walker |  | 5467 | 35 | Code::CodeKey { rung: Names, file: functions/rcompare.js, decl: 0, sub: 0, line: 0 } |  |  | 0.607 |
| walker |  | 5502 | 35 | Code::CodeKey { rung: Names, file: ranges/max-satisfying.js, decl: 0, sub: 0, line: 0 } |  |  | 0.607 |
| walker |  | 5537 | 35 | Code::CodeKey { rung: Names, file: ranges/min-satisfying.js, decl: 0, sub: 0, line: 0 } |  |  | 0.607 |
| walker |  | 5560 | 23 | Code::CodeKey { rung: Body, file: internal/lrucache.js, decl: 2, sub: 0, line: 4 } |  |  | 0.608 |
| walker |  | 5597 | 37 | Code::CodeKey { rung: Names, file: functions/eq.js, decl: 0, sub: 0, line: 0 } |  |  | 0.608 |
| walker |  | 5634 | 37 | Code::CodeKey { rung: Names, file: functions/gt.js, decl: 0, sub: 0, line: 0 } |  |  | 0.608 |
| walker |  | 5671 | 37 | Code::CodeKey { rung: Names, file: functions/lt.js, decl: 0, sub: 0, line: 0 } |  |  | 0.608 |
| ns | 5694 |  | 380 | internal/constants.js (whole file): every tunable limit and flag | 4.1 |  | 0.608 |
| walker |  | 5708 | 37 | Code::CodeKey { rung: Names, file: functions/neq.js, decl: 0, sub: 0, line: 0 } |  |  | 0.608 |
| walker |  | 5745 | 37 | Code::CodeKey { rung: Names, file: ranges/gtr.js, decl: 0, sub: 0, line: 0 } |  |  | 0.608 |
| walker |  | 5782 | 37 | Code::CodeKey { rung: Names, file: ranges/ltr.js, decl: 0, sub: 0, line: 0 } |  |  | 0.608 |
| walker |  | 5790 | 8 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.608 |
| walker |  | 5829 | 39 | Code::CodeKey { rung: Names, file: functions/gte.js, decl: 0, sub: 0, line: 0 } |  |  | 0.608 |
| walker |  | 5868 | 39 | Code::CodeKey { rung: Names, file: functions/lte.js, decl: 0, sub: 0, line: 0 } |  |  | 0.608 |
| walker |  | 5908 | 40 | Code::CodeKey { rung: Names, file: functions/sort.js, decl: 0, sub: 0, line: 0 } |  |  | 0.608 |
| walker |  | 5928 | 20 | Code::CodeKey { rung: Names, file: ranges/simplify.js, decl: 0, sub: 0, line: 0 } |  |  | 0.608 |
| walker |  | 5954 | 26 | Code::CodeKey { rung: Body, file: functions/valid.js, decl: 1, sub: 0, line: 4 } |  |  | 0.608 |
| ns | 5956 |  | 262 | internal/parse-options.js and internal/debug.js (whole files) | 4.2 |  | 0.596 |
| walker |  | 5996 | 42 | Code::CodeKey { rung: Names, file: functions/rsort.js, decl: 0, sub: 0, line: 0 } |  |  | 0.596 |
| walker |  | 6061 | 65 | Code::CodeKey { rung: Body, file: classes/semver.js, decl: 3, sub: 0, line: 81 } |  |  | 0.596 |
| walker |  | 6072 | 11 | Fs::DirListing { dir: .github/actions } |  |  | 0.596 |
| walker |  | 6076 | 4 | Fs::DirListing { dir: .github/actions/create-check } |  |  | 0.596 |
| walker |  | 6080 | 4 | Fs::DirListing { dir: .github/actions/install-latest-npm } |  |  | 0.596 |
| ns | 6262 |  | 306 | internal/identifiers.js (whole file): the prerelease ordering rule | 4.3 |  | 0.581 |
| walker |  | 6293 | 213 | Markdown::Section { file: README.md, section_index: 14, keeps_default_concavity: false } |  |  | 0.601 |
| walker |  | 6329 | 36 | Code::CodeKey { rung: Body, file: functions/clean.js, decl: 1, sub: 0, line: 4 } |  |  | 0.601 |
| ns | 6359 |  | 97 | internal/lrucache.js: class shape and the 1000-entry bound | 4.4 |  | 0.604 |
| walker |  | 6365 | 36 | Code::CodeKey { rung: Body, file: functions/prerelease.js, decl: 1, sub: 0, line: 4 } |  |  | 0.604 |
| walker |  | 6410 | 45 | Code::CodeKey { rung: Body, file: ranges/intersects.js, decl: 1, sub: 0, line: 4 } |  |  | 0.604 |
| ns | 6593 |  | 234 | internal/re.js: the four exported arrays and the createToken registrar | 4.5 |  | 0.594 |
| walker |  | 6672 | 262 | Markdown::Section { file: README.md, section_index: 20, keeps_default_concavity: false } |  |  | 0.594 |
| walker |  | 6724 | 52 | Code::CodeKey { rung: Body, file: functions/compare-build.js, decl: 1, sub: 0, line: 4 } |  |  | 0.594 |
| ns | 6761 |  | 168 | internal/re.js: the ReDoS-safe regex construction | 4.6 |  | 0.588 |
| walker |  | 6779 | 55 | Code::CodeKey { rung: Body, file: functions/satisfies.js, decl: 1, sub: 0, line: 4 } |  |  | 0.588 |
| walker |  | 6923 | 144 | Code::CodeKey { rung: Body, file: classes/semver.js, decl: 5, sub: 0, line: 93 } |  |  | 0.588 |
| walker |  | 6995 | 72 | Code::CodeKey { rung: Body, file: internal/parse-options.js, decl: 1, sub: 0, line: 6 } |  |  | 0.600 |
| walker |  | 7165 | 170 | Code::CodeKey { rung: Body, file: classes/range.js, decl: 8, sub: 0, line: 193 } |  |  | 0.601 |
| walker |  | 7244 | 79 | Code::CodeKey { rung: Body, file: ranges/valid.js, decl: 1, sub: 0, line: 4 } |  |  | 0.601 |
| ns | 7436 |  | 675 | internal/re.js: complete roster of all 43 regex token names | 4.7 |  | 0.575 |
| walker |  | 7675 | 431 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.600 |
| ns | 7755 |  | 319 | bin/semver.js: complete option-flag roster | 5.1 |  | 0.581 |
| ns | 7931 |  | 176 | bin/semver.js: usage line, the `-n` contract, and exit semantics | 5.2 |  | 0.575 |
| walker |  | 8116 | 441 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.577 |
| ns | 8268 |  | 337 | bin/semver.js: the main() output pipeline | 5.3 |  | 0.565 |
| ns | 8414 |  | 146 | SemVer.inc: complete roster of handled release types | 6.1 | 2.1 | 0.555 |
| walker |  | 8587 | 471 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.567 |
| ns | 8644 |  | 230 | functions/cmp.js: the complete operator dispatch table | 6.2 |  | 0.557 |
| ns | 8886 |  | 242 | functions/coerce.js: the right-to-left scanning rule | 6.3 |  | 0.553 |
| walker |  | 9056 | 469 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.555 |
| ns | 9133 |  | 247 | functions/diff.js: the prerelease-to-release special cases | 6.4 |  | 0.548 |
| ns | 9250 |  | 117 | Complete listings of test/ and test/fixtures/ | 7.1 |  | 0.561 |
| ns | 9400 |  | 150 | map.js + test/map.js: the enforced source-to-test mirror | 7.2 |  | 0.557 |
| walker |  | 9494 | 438 | Markdown::Section { file: README.md, section_index: 10, keeps_default_concavity: false } |  |  | 0.575 |
| walker |  | 9590 | 96 | Code::CodeKey { rung: Body, file: internal/lrucache.js, decl: 3, sub: 0, line: 9 } |  |  | 0.576 |
| ns | 9615 |  | 215 | package.json: npm scripts and tap configuration | 7.3 |  | 0.576 |
| walker |  | 9687 | 97 | Code::CodeKey { rung: Body, file: functions/parse.js, decl: 1, sub: 0, line: 4 } |  |  | 0.576 |
| ns | 9722 |  | 107 | Complete listings of benchmarks/, .github/ and .github/workflows/ | 7.4 |  | 0.585 |
| ns | 9812 |  | 90 | CONTRIBUTING.md: the rules that would silently fail a PR | 7.5 |  | 0.585 |
| ns | 9951 |  | 139 | .eslintrc.local.js: the constraints on published source | 7.6 |  | 0.580 |
| walker |  | 10000 | 313 | Markdown::Section { file: README.md, section_index: 15, keeps_default_concavity: false } |  |  | 0.592 |
