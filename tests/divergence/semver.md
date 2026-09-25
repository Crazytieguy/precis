Score(3000)=0.704 I=0.909 C=0.545 ns_rows≤3K=16/48 grid(1000/1442/2080/3000/4327/6240/9000)=0.658/0.757/0.785/0.704/0.651/0.589/0.505

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 79 |  | 79 | Package identity: README title + package.json name/version/description/main | 1.1 |  | 0.000 |
| walker |  | 112 | 112 | listing of '.' |  |  | 0.000 |
| walker |  | 115 | 3 | listing of 'tap-snapshots' |  |  | 0.000 |
| walker |  | 119 | 4 | listing of 'bin' |  |  | 0.000 |
| walker |  | 135 | 16 | listing of 'classes' |  |  | 0.000 |
| walker |  | 147 | 12 | export names surface in index.js |  |  | 0.000 |
| walker |  | 173 | 26 | README headline in README.md |  |  | 0.219 |
| ns | 191 |  | 112 | Complete root directory listing | 1.2 |  | 0.661 |
| walker |  | 199 | 26 | listing of 'internal' |  |  | 0.687 |
| walker |  | 260 | 61 | package identity in package.json |  |  | 0.945 |
| walker |  | 315 | 55 | listing of 'ranges' |  |  | 0.961 |
| ns | 316 |  | 125 | package.json `bin` + `files`: CLI entry point and published surface | 1.3 |  | 0.768 |
| ns | 362 |  | 46 | Complete listings of classes/, internal/ and bin/ | 1.4 |  | 0.786 |
| walker |  | 419 | 104 | listing of 'functions' |  |  | 0.820 |
| walker |  | 449 | 30 | package runtime metadata in package.json |  |  | 0.820 |
| ns | 466 |  | 104 | Complete listing of functions/ (24 version-level modules) | 1.5 |  | 0.834 |
| walker |  | 480 | 31 | listing of '.github' |  |  | 0.834 |
| walker |  | 520 | 40 | listing of '.github/workflows' |  |  | 0.835 |
| ns | 521 |  | 55 | Complete listing of ranges/ (11 range-level modules) | 1.6 |  | 0.836 |
| walker |  | 557 | 37 | listing of 'benchmarks' |  |  | 0.837 |
| walker |  | 569 | 12 | export names surface in classes/index.js |  |  | 0.837 |
| walker |  | 608 | 39 | listing of 'test' |  |  | 0.838 |
| walker |  | 668 | 60 | package identity metadata in package.json |  |  | 0.838 |
| walker |  | 684 | 16 | listing of 'test/classes' |  |  | 0.838 |
| ns | 716 |  | 195 | index.js aggregate export object, first half (parse..Range) | 1.7 |  | 0.719 |
| walker |  | 864 | 180 | headings outline in README.md |  |  | 0.725 |
| walker |  | 889 | 25 | README.md section #1 |  |  | 0.725 |
| ns | 913 |  | 197 | index.js aggregate export object, remainder (satisfies..rcompareIdentifiers) | 1.8 | 1.7 | 0.658 |
| walker |  | 916 | 27 | README.md section #55 |  |  | 0.658 |
| walker |  | 1051 | 135 | package entrypoints in package.json |  |  | 0.787 |
| ns | 1234 |  | 321 | README section map: every heading, no bodies | 1.9 |  | 0.754 |
| walker |  | 1336 | 285 | README.md section #2 |  |  | 0.757 |
| walker |  | 1480 | 144 | package scripts in package.json |  |  | 0.758 |
| ns | 1495 |  | 261 | range.bnf: the complete formal grammar of range syntax | 1.10 |  | 0.717 |
| walker |  | 1506 | 26 | listing of 'test/internal' |  |  | 0.717 |
| walker |  | 1509 | 3 | listing of 'tap-snapshots/test' |  |  | 0.717 |
| ns | 1748 |  | 253 | SemVer class: requires + complete method roster | 2.1 |  | 0.663 |
| walker |  | 1889 | 380 | export at index.js:45 |  |  | 0.835 |
| walker |  | 1895 | 6 | imports in classes/index.js |  |  | 0.835 |
| walker |  | 1906 | 11 | export names surface in functions/compare.js |  |  | 0.835 |
| walker |  | 1917 | 11 | export names surface in functions/eq.js |  |  | 0.835 |
| walker |  | 1928 | 11 | export names surface in functions/gt.js |  |  | 0.835 |
| walker |  | 1939 | 11 | export names surface in functions/lt.js |  |  | 0.835 |
| walker |  | 1950 | 11 | export names surface in functions/major.js |  |  | 0.835 |
| ns | 1954 |  | 206 | Comparator class: ANY sentinel, complete method roster, requires | 2.2 |  | 0.785 |
| walker |  | 1961 | 11 | export names surface in functions/minor.js |  |  | 0.785 |
| walker |  | 1972 | 11 | export names surface in functions/neq.js |  |  | 0.785 |
| walker |  | 1983 | 11 | export names surface in functions/patch.js |  |  | 0.785 |
| walker |  | 1994 | 11 | export names surface in functions/sort.js |  |  | 0.785 |
| walker |  | 2006 | 12 | export names surface in functions/compare-loose.js |  |  | 0.785 |
| walker |  | 2018 | 12 | export names surface in functions/gte.js |  |  | 0.785 |
| walker |  | 2030 | 12 | export names surface in functions/lte.js |  |  | 0.785 |
| walker |  | 2042 | 12 | export names surface in functions/rcompare.js |  |  | 0.785 |
| walker |  | 2054 | 12 | export names surface in functions/rsort.js |  |  | 0.785 |
| walker |  | 2066 | 12 | export names surface in internal/constants.js |  |  | 0.785 |
| walker |  | 2078 | 12 | export names surface in ranges/gtr.js |  |  | 0.785 |
| walker |  | 2090 | 12 | export names surface in ranges/ltr.js |  |  | 0.785 |
| walker |  | 2132 | 42 | export at classes/index.js:3 |  |  | 0.786 |
| walker |  | 2153 | 21 | README.md section #21 |  |  | 0.786 |
| walker |  | 2174 | 21 | README.md section #22 |  |  | 0.786 |
| walker |  | 2195 | 21 | README.md section #23 |  |  | 0.786 |
| walker |  | 2216 | 21 | README.md section #24 |  |  | 0.786 |
| ns | 2220 |  | 266 | Range class: complete method roster + hoisted require block and LRU cache | 2.3 |  | 0.734 |
| walker |  | 2229 | 13 | export names surface in ranges/to-comparators.js |  |  | 0.734 |
| walker |  | 2251 | 22 | README.md section #35 |  |  | 0.734 |
| walker |  | 2273 | 22 | README.md section #52 |  |  | 0.734 |
| walker |  | 2276 | 3 | listing of '.github/matchers' |  |  | 0.734 |
| walker |  | 2279 | 3 | listing of 'test/integration' |  |  | 0.734 |
| walker |  | 2304 | 25 | README.md section #13 |  |  | 0.734 |
| walker |  | 2408 | 104 | listing of 'test/functions' |  |  | 0.734 |
| walker |  | 2432 | 24 | README.md section #43 |  |  | 0.734 |
| walker |  | 2456 | 24 | README.md section #48 |  |  | 0.734 |
| walker |  | 2511 | 55 | listing of 'test/ranges' |  |  | 0.734 |
| walker |  | 2540 | 29 | README.md section #14 |  |  | 0.734 |
| ns | 2541 |  | 321 | range.js: complete roster of module-level range-desugaring helpers | 2.4 |  | 0.687 |
| walker |  | 2563 | 23 | README.md section #34 |  |  | 0.687 |
| walker |  | 2567 | 4 | listing of 'test/bin' |  |  | 0.687 |
| walker |  | 2596 | 29 | README.md section #26 |  |  | 0.687 |
| ns | 2606 |  | 65 | classes/index.js barrel (whole file) | 2.5 |  | 0.691 |
| walker |  | 2625 | 29 | README.md section #36 |  |  | 0.691 |
| walker |  | 2731 | 106 | README.md section #4 |  |  | 0.692 |
| walker |  | 2761 | 30 | README.md section #39 |  |  | 0.692 |
| walker |  | 2781 | 20 | export names surface in ranges/simplify.js |  |  | 0.692 |
| walker |  | 2781 | 0 | export at ranges/simplify.js:8 |  |  | 0.692 |
| ns | 2861 |  | 255 | README usage: canonical calls against the aggregate export | 3.1 |  | 0.701 |
| walker |  | 2864 | 83 | listing of 'test/fixtures' |  |  | 0.703 |
| walker |  | 2886 | 22 | export names surface in classes/comparator.js |  |  | 0.703 |
| walker |  | 2908 | 22 | export names surface in classes/range.js |  |  | 0.704 |
| walker |  | 2930 | 22 | export names surface in classes/semver.js |  |  | 0.704 |
| walker |  | 2963 | 33 | README.md section #31 |  |  | 0.704 |
| walker |  | 2971 | 8 | imports in classes/range.js |  |  | 0.704 |
| walker |  | 3008 | 37 | README.md section #40 |  |  | 0.704 |
| walker |  | 3044 | 36 | README.md section #41 |  |  | 0.704 |
| ns | 3069 |  | 208 | README: the options object (`loose`, `includePrerelease`) | 3.2 |  | 0.687 |
| walker |  | 3081 | 37 | README.md section #47 |  |  | 0.687 |
| walker |  | 3164 | 83 | export at classes/range.js:6 |  |  | 0.698 |
| ns | 3175 |  | 106 | README: what counts as a version | 3.3 |  | 0.704 |
| walker |  | 3191 | 27 | export names surface in functions/clean.js |  |  | 0.704 |
| walker |  | 3191 | 0 | export at functions/clean.js:4 |  |  | 0.704 |
| walker |  | 3218 | 27 | export names surface in functions/valid.js |  |  | 0.704 |
| walker |  | 3218 | 0 | export at functions/valid.js:4 |  |  | 0.704 |
| walker |  | 3247 | 29 | export names surface in functions/coerce.js |  |  | 0.704 |
| walker |  | 3247 | 0 | export at functions/coerce.js:7 |  |  | 0.704 |
| walker |  | 3276 | 29 | export names surface in functions/diff.js |  |  | 0.704 |
| walker |  | 3276 | 0 | export at functions/diff.js:5 |  |  | 0.704 |
| walker |  | 3305 | 29 | export names surface in functions/prerelease.js |  |  | 0.704 |
| walker |  | 3305 | 0 | export at functions/prerelease.js:4 |  |  | 0.704 |
| walker |  | 3334 | 29 | export names surface in functions/satisfies.js |  |  | 0.704 |
| walker |  | 3334 | 0 | export at functions/satisfies.js:4 |  |  | 0.704 |
| ns | 3335 |  | 160 | README: comparators and the complete primitive operator set | 3.4 |  | 0.689 |
| walker |  | 3363 | 29 | export names surface in ranges/min-version.js |  |  | 0.689 |
| walker |  | 3363 | 0 | export at ranges/min-version.js:7 |  |  | 0.689 |
| walker |  | 3392 | 29 | export names surface in ranges/valid.js |  |  | 0.689 |
| walker |  | 3392 | 0 | export at ranges/valid.js:4 |  |  | 0.689 |
| walker |  | 3422 | 30 | export names surface in ranges/subset.js |  |  | 0.689 |
| walker |  | 3422 | 0 | export at ranges/subset.js:45 |  |  | 0.689 |
| ns | 3449 |  | 114 | README: comparator sets intersect, `\|\|` unions them | 3.5 |  | 0.681 |
| walker |  | 3453 | 31 | export names surface in functions/cmp.js |  |  | 0.681 |
| walker |  | 3453 | 0 | export at functions/cmp.js:10 |  |  | 0.681 |
| walker |  | 3484 | 31 | export names surface in functions/compare-build.js |  |  | 0.681 |
| walker |  | 3484 | 0 | export at functions/compare-build.js:4 |  |  | 0.681 |
| walker |  | 3515 | 31 | export names surface in ranges/intersects.js |  |  | 0.681 |
| walker |  | 3515 | 0 | export at ranges/intersects.js:4 |  |  | 0.681 |
| walker |  | 3546 | 31 | export names surface in ranges/outside.js |  |  | 0.681 |
| walker |  | 3546 | 0 | export at ranges/outside.js:13 |  |  | 0.681 |
| walker |  | 3553 | 7 | listing of '.github/ISSUE_TEMPLATE' |  |  | 0.681 |
| walker |  | 3585 | 32 | export names surface in functions/parse.js |  |  | 0.681 |
| walker |  | 3585 | 0 | export at functions/parse.js:4 |  |  | 0.681 |
| ns | 3615 |  | 166 | README: the prerelease-tag matching rule | 3.6 |  | 0.669 |
| walker |  | 3629 | 44 | README.md section #37 |  |  | 0.669 |
| walker |  | 3671 | 42 | README.md section #38 |  |  | 0.669 |
| walker |  | 3715 | 44 | README.md section #45 |  |  | 0.669 |
| walker |  | 3760 | 45 | README.md section #53 |  |  | 0.669 |
| walker |  | 3794 | 34 | export names surface in functions/inc.js |  |  | 0.669 |
| walker |  | 3794 | 0 | export at functions/inc.js:5 |  |  | 0.669 |
| walker |  | 3829 | 35 | export names surface in ranges/max-satisfying.js |  |  | 0.669 |
| walker |  | 3829 | 0 | export at ranges/max-satisfying.js:6 |  |  | 0.669 |
| ns | 3853 |  | 238 | README: the `inc` contract and the eight release types | 3.7 |  | 0.655 |
| walker |  | 3864 | 35 | export names surface in ranges/min-satisfying.js |  |  | 0.655 |
| walker |  | 3864 | 0 | export at ranges/min-satisfying.js:5 |  |  | 0.655 |
| walker |  | 3911 | 47 | README.md section #29 |  |  | 0.655 |
| walker |  | 3961 | 50 | README.md section #30 |  |  | 0.655 |
| walker |  | 4016 | 55 | README.md section #25 |  |  | 0.655 |
| walker |  | 4026 | 10 | CHANGELOG.md section #0 |  |  | 0.655 |
| walker |  | 4052 | 26 | export body at functions/valid.js:4 body 5 |  |  | 0.655 |
| walker |  | 4064 | 12 | listing of '.github/actions' |  |  | 0.655 |
| walker |  | 4067 | 3 | listing of '.github/actions/create-check' |  |  | 0.655 |
| walker |  | 4070 | 3 | listing of '.github/actions/install-latest-npm' |  |  | 0.655 |
| walker |  | 4143 | 73 | export at classes/comparator.js:5 |  |  | 0.666 |
| walker |  | 4151 | 8 | imports in classes/comparator.js |  |  | 0.666 |
| walker |  | 4159 | 8 | imports in internal/constants.js |  |  | 0.666 |
| ns | 4181 |  | 328 | README: caret ranges (the left-most non-zero rule) and its desugaring table | 3.8 |  | 0.651 |
| walker |  | 4229 | 70 | README.md section #32 |  |  | 0.651 |
| walker |  | 4300 | 71 | README.md section #28 |  |  | 0.651 |
| walker |  | 4307 | 7 | listing of 'tap-snapshots/test/bin' |  |  | 0.651 |
| walker |  | 4384 | 77 | README.md section #6 |  |  | 0.654 |
| walker |  | 4460 | 76 | README.md section #27 |  |  | 0.654 |
| ns | 4471 |  | 290 | README: tilde ranges and their desugaring table | 3.9 |  | 0.645 |
| walker |  | 4542 | 82 | README.md section #8 |  |  | 0.645 |
| walker |  | 4578 | 36 | export body at functions/clean.js:4 body 5 |  |  | 0.645 |
| walker |  | 4614 | 36 | export body at functions/prerelease.js:4 body 5 |  |  | 0.645 |
| walker |  | 4699 | 85 | README.md section #51 |  |  | 0.645 |
| ns | 4735 |  | 264 | README: X-ranges and bare partial versions | 3.10 |  | 0.634 |
| ns | 4926 |  | 191 | README: hyphen ranges | 3.11 |  | 0.625 |
| ns | 5105 |  | 179 | README: ranges can be non-contiguous (the gtr/ltr/satisfies gotcha) | 3.12 |  | 0.618 |
| walker |  | 5275 | 576 | imports in index.js |  |  | 0.618 |
| walker |  | 5289 | 14 | export names surface in preload.js |  |  | 0.618 |
| walker |  | 5310 | 21 | export names surface in map.js |  |  | 0.618 |
| walker |  | 5316 | 6 | imports in map.js |  |  | 0.618 |
| ns | 5318 |  | 213 | README: coercion limits | 3.13 |  | 0.610 |
| walker |  | 5324 | 8 | imports in preload.js |  |  | 0.610 |
| walker |  | 5415 | 91 | README.md section #10 |  |  | 0.624 |
| walker |  | 5516 | 101 | export at classes/semver.js:9 |  |  | 0.636 |
| walker |  | 5602 | 86 | README.md section #42 |  |  | 0.636 |
| walker |  | 5613 | 11 | export names surface in internal/debug.js |  |  | 0.636 |
| walker |  | 5689 | 76 | README.md section #33 |  |  | 0.636 |
| ns | 5698 |  | 380 | internal/constants.js (whole file): every tunable limit and flag | 4.1 |  | 0.608 |
| walker |  | 5762 | 73 | README.md section #57 |  |  | 0.608 |
| walker |  | 5783 | 21 | module item at functions/compare-loose.js:4 |  |  | 0.608 |
| walker |  | 5805 | 22 | module item at functions/major.js:4 |  |  | 0.608 |
| walker |  | 5827 | 22 | module item at functions/minor.js:4 |  |  | 0.608 |
| walker |  | 5849 | 22 | module item at functions/patch.js:4 |  |  | 0.608 |
| walker |  | 5894 | 45 | export body at ranges/intersects.js:4 body 5 |  |  | 0.608 |
| walker |  | 5917 | 23 | module item at functions/rcompare.js:4 |  |  | 0.608 |
| ns | 5960 |  | 262 | internal/parse-options.js and internal/debug.js (whole files) | 4.2 |  | 0.589 |
| walker |  | 6029 | 112 | README.md section #9 |  |  | 0.589 |
| walker |  | 6054 | 25 | module item at ranges/gtr.js:5 |  |  | 0.589 |
| walker |  | 6079 | 25 | module item at ranges/ltr.js:5 |  |  | 0.589 |
| walker |  | 6131 | 52 | export body at functions/compare-build.js:4 body 5 |  |  | 0.589 |
| walker |  | 6157 | 26 | module item at functions/eq.js:4 |  |  | 0.589 |
| walker |  | 6183 | 26 | module item at functions/gt.js:4 |  |  | 0.589 |
| walker |  | 6209 | 26 | module item at functions/lt.js:4 |  |  | 0.589 |
| walker |  | 6235 | 26 | module item at functions/neq.js:4 |  |  | 0.589 |
| ns | 6266 |  | 306 | internal/identifiers.js (whole file): the prerelease ordering rule | 4.3 |  | 0.570 |
| walker |  | 6323 | 88 | export at internal/constants.js:28 |  |  | 0.573 |
| walker |  | 6350 | 27 | module item at functions/gte.js:4 |  |  | 0.573 |
| ns | 6363 |  | 97 | internal/lrucache.js: class shape and the 1000-entry bound | 4.4 |  | 0.565 |
| walker |  | 6377 | 27 | module item at functions/lte.js:4 |  |  | 0.565 |
| walker |  | 6432 | 55 | export body at functions/satisfies.js:4 body 5 |  |  | 0.565 |
| walker |  | 6461 | 29 | module item at functions/sort.js:4 |  |  | 0.565 |
| walker |  | 6491 | 30 | module item at functions/rsort.js:4 |  |  | 0.565 |
| walker |  | 6510 | 19 | imports in ranges/gtr.js |  |  | 0.565 |
| ns | 6597 |  | 234 | internal/re.js: the four exported arrays and the createToken registrar | 4.5 |  | 0.555 |
| walker |  | 6643 | 133 | README.md section #50 |  |  | 0.555 |
| walker |  | 6678 | 35 | module item at functions/compare.js:4 |  |  | 0.555 |
| walker |  | 6700 | 22 | imports in functions/clean.js |  |  | 0.555 |
| walker |  | 6722 | 22 | imports in functions/compare-loose.js |  |  | 0.555 |
| walker |  | 6744 | 22 | imports in functions/eq.js |  |  | 0.555 |
| ns | 6765 |  | 168 | internal/re.js: the ReDoS-safe regex construction | 4.6 |  | 0.549 |
| walker |  | 6766 | 22 | imports in functions/gt.js |  |  | 0.549 |
| walker |  | 6788 | 22 | imports in functions/gte.js |  |  | 0.549 |
| walker |  | 6810 | 22 | imports in functions/lt.js |  |  | 0.549 |
| walker |  | 6832 | 22 | imports in functions/lte.js |  |  | 0.549 |
| walker |  | 6854 | 22 | imports in functions/neq.js |  |  | 0.549 |
| walker |  | 6876 | 22 | imports in functions/prerelease.js |  |  | 0.549 |
| walker |  | 6898 | 22 | imports in functions/rcompare.js |  |  | 0.549 |
| walker |  | 6920 | 22 | imports in functions/valid.js |  |  | 0.549 |
| walker |  | 6943 | 23 | imports in functions/diff.js |  |  | 0.549 |
| walker |  | 7022 | 79 | export body at ranges/valid.js:4 body 5 |  |  | 0.549 |
| walker |  | 7177 | 155 | README.md section #44 |  |  | 0.549 |
| walker |  | 7201 | 24 | imports in functions/rsort.js |  |  | 0.549 |
| walker |  | 7225 | 24 | imports in functions/satisfies.js |  |  | 0.549 |
| walker |  | 7249 | 24 | imports in functions/sort.js |  |  | 0.549 |
| walker |  | 7273 | 24 | imports in ranges/intersects.js |  |  | 0.549 |
| walker |  | 7297 | 24 | imports in ranges/ltr.js |  |  | 0.549 |
| walker |  | 7321 | 24 | imports in ranges/valid.js |  |  | 0.549 |
| walker |  | 7345 | 24 | export names surface in internal/lrucache.js |  |  | 0.550 |
| walker |  | 7371 | 26 | imports in functions/compare-build.js |  |  | 0.550 |
| walker |  | 7397 | 26 | imports in functions/compare.js |  |  | 0.550 |
| walker |  | 7423 | 26 | imports in functions/inc.js |  |  | 0.550 |
| ns | 7440 |  | 675 | internal/re.js: complete roster of all 43 regex token names | 4.7 |  | 0.526 |
| walker |  | 7449 | 26 | imports in functions/major.js |  |  | 0.526 |
| walker |  | 7475 | 26 | imports in functions/minor.js |  |  | 0.526 |
| walker |  | 7501 | 26 | imports in functions/parse.js |  |  | 0.526 |
| walker |  | 7527 | 26 | imports in functions/patch.js |  |  | 0.526 |
| walker |  | 7553 | 26 | imports in ranges/to-comparators.js |  |  | 0.526 |
| walker |  | 7578 | 25 | export names surface in internal/parse-options.js |  |  | 0.526 |
| walker |  | 7578 | 0 | export at internal/parse-options.js:6 |  |  | 0.526 |
| walker |  | 7748 | 170 | README.md section #46 |  |  | 0.536 |
| ns | 7759 |  | 319 | bin/semver.js: complete option-flag roster | 5.1 |  | 0.519 |
| walker |  | 7933 | 185 | README.md section #7 |  |  | 0.519 |
| ns | 7935 |  | 176 | bin/semver.js: usage line, the `-n` contract, and exit semantics | 5.2 |  | 0.513 |
| walker |  | 8030 | 97 | export body at functions/parse.js:4 body 5 |  |  | 0.513 |
| ns | 8272 |  | 337 | bin/semver.js: the main() output pipeline | 5.3 |  | 0.502 |
| ns | 8418 |  | 146 | SemVer.inc: complete roster of handled release types | 6.1 | 2.1 | 0.493 |
| walker |  | 8628 | 598 | README.md section #5 |  |  | 0.518 |
| ns | 8648 |  | 230 | functions/cmp.js: the complete operator dispatch table | 6.2 |  | 0.508 |
| walker |  | 8657 | 29 | export names surface in internal/identifiers.js |  |  | 0.509 |
| walker |  | 8657 | 0 | export at internal/identifiers.js:4 |  |  | 0.509 |
| walker |  | 8677 | 20 | export at internal/identifiers.js:26 |  |  | 0.510 |
| walker |  | 8729 | 52 | module item at ranges/to-comparators.js:6 |  |  | 0.510 |
| ns | 8890 |  | 242 | functions/coerce.js: the right-to-left scanning rule | 6.3 |  | 0.505 |
| walker |  | 9079 | 350 | json config release-please-config.json |  |  | 0.505 |
| ns | 9137 |  | 247 | functions/diff.js: the prerelease-to-release special cases | 6.4 |  | 0.498 |
| walker |  | 9231 | 152 | README.md section #56 |  |  | 0.498 |
| ns | 9260 |  | 123 | Complete listings of test/ and test/fixtures/ | 7.1 |  | 0.515 |
| walker |  | 9277 | 46 | export at internal/lrucache.js:3 |  |  | 0.521 |
| walker |  | 9314 | 37 | imports in ranges/simplify.js |  |  | 0.521 |
| ns | 9410 |  | 150 | map.js + test/map.js: the enforced source-to-test mirror | 7.2 |  | 0.518 |
| walker |  | 9536 | 222 | README.md section #54 |  |  | 0.518 |
| walker |  | 9575 | 39 | imports in ranges/max-satisfying.js |  |  | 0.518 |
| walker |  | 9614 | 39 | imports in ranges/min-satisfying.js |  |  | 0.518 |
| ns | 9625 |  | 215 | package.json: npm scripts and tap configuration | 7.3 |  | 0.518 |
| ns | 9734 |  | 109 | Complete listings of benchmarks/, .github/ and .github/workflows/ | 7.4 |  | 0.530 |
| walker |  | 9750 | 136 | export body at functions/inc.js:5 body 6 |  |  | 0.530 |
| walker |  | 9791 | 41 | imports in ranges/outside.js |  |  | 0.530 |
| ns | 9824 |  | 90 | CONTRIBUTING.md: the rules that would silently fail a PR | 7.5 |  | 0.530 |
| walker |  | 9832 | 41 | imports in ranges/subset.js |  |  | 0.530 |
| ns | 9963 |  | 139 | .eslintrc.local.js: the constraints on published source | 7.6 |  | 0.525 |
