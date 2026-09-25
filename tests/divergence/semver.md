Score(3000)=0.726 I=0.918 C=0.575 ns_rows≤3K=16/48 grid(1000/1442/2080/3000/4327/6240/9000)=0.659/0.758/0.798/0.726/0.638/0.611/0.538

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
| walker |  | 844 | 22 | Code::CodeKey { rung: Names, file: classes/semver.js, decl: 0, sub: 0, line: 0 } |  |  | 0.724 |
| ns | 909 |  | 197 | index.js aggregate export object, remainder (satisfies..rcompareIdentifiers) | 1.8 | 1.7 | 0.658 |
| walker |  | 950 | 106 | Code::CodeKey { rung: Decl, file: classes/semver.js, decl: 1, sub: 0, line: 9 } |  |  | 0.659 |
| walker |  | 959 | 9 | Code::CodeKey { rung: Body, file: classes/semver.js, decl: 4, sub: 0, line: 89 } |  |  | 0.659 |
| ns | 1230 |  | 321 | README section map: every heading, no bodies | 1.9 |  | 0.638 |
| walker |  | 1244 | 285 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: true } |  |  | 0.641 |
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
| walker |  | 2179 | 21 | Code::CodeKey { rung: Names, file: map.js, decl: 0, sub: 0, line: 0 } |  |  | 0.798 |
| walker |  | 2183 | 4 | Fs::DirListing { dir: .github/matchers } |  |  | 0.798 |
| walker |  | 2187 | 4 | Fs::DirListing { dir: test/integration } |  |  | 0.798 |
| ns | 2216 |  | 266 | Range class: complete method roster + hoisted require block and LRU cache | 2.3 |  | 0.745 |
| walker |  | 2243 | 56 | Fs::DirListing { dir: test/ranges } |  |  | 0.745 |
| walker |  | 2264 | 21 | Code::CodeKey { rung: Names, file: internal/debug.js, decl: 0, sub: 0, line: 0 } |  |  | 0.745 |
| walker |  | 2337 | 73 | Code::CodeKey { rung: Decl, file: internal/debug.js, decl: 1, sub: 0, line: 3 } |  |  | 0.745 |
| walker |  | 2359 | 22 | Code::CodeKey { rung: Names, file: classes/comparator.js, decl: 0, sub: 0, line: 0 } |  |  | 0.746 |
| walker |  | 2437 | 78 | Code::CodeKey { rung: Decl, file: classes/comparator.js, decl: 1, sub: 0, line: 5 } |  |  | 0.753 |
| walker |  | 2445 | 8 | Code::CodeKey { rung: Body, file: classes/comparator.js, decl: 2, sub: 0, line: 6 } |  |  | 0.754 |
| walker |  | 2454 | 9 | Code::CodeKey { rung: Body, file: classes/comparator.js, decl: 5, sub: 0, line: 57 } |  |  | 0.755 |
| walker |  | 2476 | 22 | Code::CodeKey { rung: Names, file: classes/range.js, decl: 0, sub: 0, line: 0 } |  |  | 0.755 |
| ns | 2537 |  | 321 | range.js: complete roster of module-level range-desugaring helpers | 2.4 |  | 0.707 |
| walker |  | 2564 | 88 | Code::CodeKey { rung: Decl, file: classes/range.js, decl: 1, sub: 0, line: 6 } |  |  | 0.713 |
| walker |  | 2573 | 9 | Code::CodeKey { rung: Body, file: classes/range.js, decl: 4, sub: 0, line: 92 } |  |  | 0.714 |
| walker |  | 2582 | 9 | Code::CodeKey { rung: Body, file: classes/range.js, decl: 5, sub: 0, line: 96 } |  |  | 0.714 |
| ns | 2602 |  | 65 | classes/index.js barrel (whole file) | 2.5 |  | 0.714 |
| walker |  | 2666 | 84 | Fs::DirListing { dir: test/fixtures } |  |  | 0.716 |
| walker |  | 2671 | 5 | Fs::DirListing { dir: test/bin } |  |  | 0.716 |
| walker |  | 2695 | 24 | Code::CodeKey { rung: Names, file: internal/lrucache.js, decl: 0, sub: 0, line: 0 } |  |  | 0.716 |
| walker |  | 2746 | 51 | Code::CodeKey { rung: Decl, file: internal/lrucache.js, decl: 1, sub: 0, line: 3 } |  |  | 0.717 |
| walker |  | 2757 | 11 | Code::CodeKey { rung: Body, file: internal/lrucache.js, decl: 4, sub: 0, line: 21 } |  |  | 0.717 |
| walker |  | 2780 | 23 | Code::CodeKey { rung: Body, file: internal/lrucache.js, decl: 2, sub: 0, line: 4 } |  |  | 0.717 |
| walker |  | 2805 | 25 | Code::CodeKey { rung: Names, file: internal/parse-options.js, decl: 0, sub: 0, line: 0 } |  |  | 0.717 |
| ns | 2857 |  | 255 | README usage: canonical calls against the aggregate export | 3.1 |  | 0.725 |
| walker |  | 2880 | 75 | Code::CodeKey { rung: Names, file: internal/identifiers.js, decl: 0, sub: 0, line: 0 } |  |  | 0.726 |
| walker |  | 3024 | 144 | Code::CodeKey { rung: Body, file: classes/semver.js, decl: 5, sub: 0, line: 93 } |  |  | 0.726 |
| walker |  | 3051 | 27 | Code::CodeKey { rung: Names, file: functions/clean.js, decl: 0, sub: 0, line: 0 } |  |  | 0.726 |
| ns | 3065 |  | 208 | README: the options object (`loose`, `includePrerelease`) | 3.2 |  | 0.709 |
| walker |  | 3087 | 36 | Code::CodeKey { rung: Body, file: functions/clean.js, decl: 1, sub: 0, line: 4 } |  |  | 0.709 |
| walker |  | 3114 | 27 | Code::CodeKey { rung: Names, file: functions/valid.js, decl: 0, sub: 0, line: 0 } |  |  | 0.709 |
| walker |  | 3140 | 26 | Code::CodeKey { rung: Body, file: functions/valid.js, decl: 1, sub: 0, line: 4 } |  |  | 0.709 |
| ns | 3171 |  | 106 | README: what counts as a version | 3.3 |  | 0.700 |
| walker |  | 3266 | 126 | Code::CodeKey { rung: Names, file: internal/re.js, decl: 0, sub: 0, line: 0 } |  |  | 0.701 |
| walker |  | 3294 | 28 | Code::CodeKey { rung: Names, file: functions/compare.js, decl: 0, sub: 0, line: 0 } |  |  | 0.701 |
| walker |  | 3312 | 18 | Code::CodeKey { rung: Decl, file: functions/compare.js, decl: 1, sub: 0, line: 4 } |  |  | 0.701 |
| ns | 3331 |  | 160 | README: comparators and the complete primitive operator set | 3.4 |  | 0.686 |
| walker |  | 3341 | 29 | Code::CodeKey { rung: Names, file: functions/coerce.js, decl: 0, sub: 0, line: 0 } |  |  | 0.686 |
| walker |  | 3370 | 29 | Code::CodeKey { rung: Names, file: functions/diff.js, decl: 0, sub: 0, line: 0 } |  |  | 0.686 |
| walker |  | 3399 | 29 | Code::CodeKey { rung: Names, file: functions/prerelease.js, decl: 0, sub: 0, line: 0 } |  |  | 0.686 |
| walker |  | 3435 | 36 | Code::CodeKey { rung: Body, file: functions/prerelease.js, decl: 1, sub: 0, line: 4 } |  |  | 0.686 |
| ns | 3445 |  | 114 | README: comparator sets intersect, `\|\|` unions them | 3.5 |  | 0.678 |
| walker |  | 3464 | 29 | Code::CodeKey { rung: Names, file: functions/satisfies.js, decl: 0, sub: 0, line: 0 } |  |  | 0.678 |
| walker |  | 3519 | 55 | Code::CodeKey { rung: Body, file: functions/satisfies.js, decl: 1, sub: 0, line: 4 } |  |  | 0.678 |
| walker |  | 3548 | 29 | Code::CodeKey { rung: Names, file: ranges/min-version.js, decl: 0, sub: 0, line: 0 } |  |  | 0.678 |
| walker |  | 3577 | 29 | Code::CodeKey { rung: Names, file: ranges/valid.js, decl: 0, sub: 0, line: 0 } |  |  | 0.678 |
| ns | 3611 |  | 166 | README: the prerelease-tag matching rule | 3.6 |  | 0.666 |
| ns | 3849 |  | 238 | README: the `inc` contract and the eight release types | 3.7 |  | 0.652 |
| ns | 4177 |  | 328 | README: caret ranges (the left-most non-zero rule) and its desugaring table | 3.8 |  | 0.638 |
| walker |  | 4444 | 867 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: true } |  |  | 0.638 |
| ns | 4467 |  | 290 | README: tilde ranges and their desugaring table | 3.9 |  | 0.629 |
| ns | 4731 |  | 264 | README: X-ranges and bare partial versions | 3.10 |  | 0.618 |
| ns | 4922 |  | 191 | README: hyphen ranges | 3.11 |  | 0.609 |
| walker |  | 5007 | 563 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: true } |  |  | 0.609 |
| walker |  | 5037 | 30 | Code::CodeKey { rung: Names, file: ranges/subset.js, decl: 0, sub: 0, line: 0 } |  |  | 0.609 |
| walker |  | 5067 | 30 | Code::CodeKey { rung: Names, file: ranges/to-comparators.js, decl: 0, sub: 0, line: 0 } |  |  | 0.609 |
| ns | 5101 |  | 179 | README: ranges can be non-contiguous (the gtr/ltr/satisfies gotcha) | 3.12 |  | 0.602 |
| walker |  | 5102 | 35 | Code::CodeKey { rung: Decl, file: ranges/to-comparators.js, decl: 1, sub: 0, line: 6 } |  |  | 0.602 |
| walker |  | 5210 | 108 | Code::CodeKey { rung: Names, file: internal/constants.js, decl: 0, sub: 0, line: 0 } |  |  | 0.603 |
| walker |  | 5229 | 19 | Code::CodeKey { rung: Decl, file: internal/constants.js, decl: 3, sub: 0, line: 8 } |  |  | 0.603 |
| walker |  | 5293 | 64 | Code::CodeKey { rung: Decl, file: internal/constants.js, decl: 6, sub: 0, line: 18 } |  |  | 0.603 |
| ns | 5314 |  | 213 | README: coercion limits | 3.13 |  | 0.595 |
| walker |  | 5381 | 88 | Code::CodeKey { rung: Decl, file: internal/constants.js, decl: 7, sub: 0, line: 28 } |  |  | 0.596 |
| walker |  | 5453 | 72 | Code::CodeKey { rung: Body, file: internal/parse-options.js, decl: 1, sub: 0, line: 6 } |  |  | 0.597 |
| walker |  | 5623 | 170 | Code::CodeKey { rung: Body, file: classes/range.js, decl: 8, sub: 0, line: 193 } |  |  | 0.598 |
| walker |  | 5654 | 31 | Code::CodeKey { rung: Names, file: functions/cmp.js, decl: 0, sub: 0, line: 0 } |  |  | 0.598 |
| walker |  | 5685 | 31 | Code::CodeKey { rung: Names, file: functions/compare-build.js, decl: 0, sub: 0, line: 0 } |  |  | 0.598 |
| ns | 5694 |  | 380 | internal/constants.js (whole file): every tunable limit and flag | 4.1 |  | 0.599 |
| walker |  | 5737 | 52 | Code::CodeKey { rung: Body, file: functions/compare-build.js, decl: 1, sub: 0, line: 4 } |  |  | 0.599 |
| walker |  | 5768 | 31 | Code::CodeKey { rung: Names, file: ranges/intersects.js, decl: 0, sub: 0, line: 0 } |  |  | 0.599 |
| walker |  | 5813 | 45 | Code::CodeKey { rung: Body, file: ranges/intersects.js, decl: 1, sub: 0, line: 4 } |  |  | 0.599 |
| walker |  | 5844 | 31 | Code::CodeKey { rung: Names, file: ranges/outside.js, decl: 0, sub: 0, line: 0 } |  |  | 0.599 |
| walker |  | 5876 | 32 | Code::CodeKey { rung: Names, file: functions/parse.js, decl: 0, sub: 0, line: 0 } |  |  | 0.599 |
| ns | 5956 |  | 262 | internal/parse-options.js and internal/debug.js (whole files) | 4.2 |  | 0.600 |
| walker |  | 5982 | 106 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.611 |
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
| walker |  | 7275 | 136 | Code::CodeKey { rung: Body, file: functions/inc.js, decl: 1, sub: 0, line: 5 } |  |  | 0.584 |
| ns | 7436 |  | 675 | internal/re.js: complete roster of all 43 regex token names | 4.7 |  | 0.559 |
| walker |  | 7500 | 225 | Code::CodeKey { rung: Body, file: classes/semver.js, decl: 8, sub: 0, line: 168 } |  |  | 0.559 |
| walker |  | 7650 | 150 | Code::CodeKey { rung: Body, file: classes/comparator.js, decl: 6, sub: 0, line: 61 } |  |  | 0.560 |
| walker |  | 7658 | 8 | Fs::DirListing { dir: tap-snapshots/test/bin } |  |  | 0.560 |
| ns | 7755 |  | 319 | bin/semver.js: complete option-flag roster | 5.1 |  | 0.542 |
| walker |  | 7855 | 197 | Code::CodeKey { rung: Body, file: classes/range.js, decl: 7, sub: 0, line: 170 } |  |  | 0.543 |
| ns | 7931 |  | 176 | bin/semver.js: usage line, the `-n` contract, and exit semantics | 5.2 |  | 0.537 |
| walker |  | 8053 | 198 | Code::CodeKey { rung: Body, file: internal/identifiers.js, decl: 1, sub: 0, line: 4 } |  |  | 0.561 |
| walker |  | 8186 | 133 | Code::CodeKey { rung: Body, file: internal/lrucache.js, decl: 5, sub: 0, line: 25 } |  |  | 0.562 |
| ns | 8268 |  | 337 | bin/semver.js: the main() output pipeline | 5.3 |  | 0.550 |
| walker |  | 8385 | 199 | Code::CodeKey { rung: Body, file: ranges/max-satisfying.js, decl: 1, sub: 0, line: 6 } |  |  | 0.550 |
| ns | 8414 |  | 146 | SemVer.inc: complete roster of handled release types | 6.1 | 2.1 | 0.540 |
| walker |  | 8584 | 199 | Code::CodeKey { rung: Body, file: ranges/min-satisfying.js, decl: 1, sub: 0, line: 5 } |  |  | 0.540 |
| ns | 8644 |  | 230 | functions/cmp.js: the complete operator dispatch table | 6.2 |  | 0.531 |
| walker |  | 8810 | 226 | Code::CodeKey { rung: Body, file: classes/semver.js, decl: 6, sub: 0, line: 109 } |  |  | 0.531 |
| ns | 8886 |  | 242 | functions/coerce.js: the right-to-left scanning rule | 6.3 |  | 0.527 |
| walker |  | 9023 | 213 | Markdown::Section { file: README.md, section_index: 14, keeps_default_concavity: false } |  |  | 0.542 |
| ns | 9133 |  | 247 | functions/diff.js: the prerelease-to-release special cases | 6.4 |  | 0.535 |
| ns | 9250 |  | 117 | Complete listings of test/ and test/fixtures/ | 7.1 |  | 0.549 |
| walker |  | 9318 | 295 | Code::CodeKey { rung: Body, file: ranges/subset.js, decl: 1, sub: 0, line: 45 } |  |  | 0.549 |
| ns | 9400 |  | 150 | map.js + test/map.js: the enforced source-to-test mirror | 7.2 |  | 0.545 |
| walker |  | 9580 | 262 | Markdown::Section { file: README.md, section_index: 20, keeps_default_concavity: false } |  |  | 0.545 |
| ns | 9615 |  | 215 | package.json: npm scripts and tap configuration | 7.3 |  | 0.545 |
| ns | 9722 |  | 107 | Complete listings of benchmarks/, .github/ and .github/workflows/ | 7.4 |  | 0.556 |
| walker |  | 9791 | 211 | Code::CodeKey { rung: Body, file: classes/comparator.js, decl: 4, sub: 0, line: 36 } |  |  | 0.557 |
| ns | 9812 |  | 90 | CONTRIBUTING.md: the rules that would silently fail a PR | 7.5 |  | 0.556 |
| ns | 9951 |  | 139 | .eslintrc.local.js: the constraints on published source | 7.6 |  | 0.551 |
| walker |  | 9993 | 202 | Code::CodeKey { rung: Body, file: functions/cmp.js, decl: 1, sub: 0, line: 10 } |  |  | 0.553 |
