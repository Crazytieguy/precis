Score(3000)=0.578 I=0.761 C=0.439 ns_rows≤3K=16/48 grid(1000/1442/2080/3000/4327/6240/9000)=0.653/0.595/0.492/0.578/0.637/0.610/0.535

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 79 |  | 79 | Package identity: README title + package.json name/version/description/main | 1.1 |  | 0.000 |
| walker |  | 103 | 103 | listing of '.' |  |  | 0.000 |
| walker |  | 106 | 3 | listing of 'tap-snapshots' |  |  | 0.000 |
| walker |  | 111 | 5 | listing of 'bin' |  |  | 0.000 |
| walker |  | 123 | 12 | ts names index.js |  |  | 0.000 |
| walker |  | 140 | 17 | listing of 'classes' |  |  | 0.000 |
| walker |  | 152 | 12 | ts names classes/index.js |  |  | 0.000 |
| walker |  | 166 | 14 | ts names preload.js |  |  | 0.000 |
| ns | 182 |  | 103 | Complete root directory listing | 1.2 |  | 0.615 |
| walker |  | 192 | 26 | README headline in README.md |  |  | 0.661 |
| walker |  | 219 | 27 | listing of 'internal' |  |  | 0.687 |
| walker |  | 240 | 21 | ts names map.js |  |  | 0.687 |
| walker |  | 282 | 42 | ts decl classes/index.js:3 |  |  | 0.690 |
| ns | 307 |  | 125 | package.json `bin` + `files`: CLI entry point and published surface | 1.3 |  | 0.551 |
| walker |  | 343 | 61 | package identity in package.json |  |  | 0.757 |
| ns | 356 |  | 49 | Complete listings of classes/, internal/ and bin/ | 1.4 |  | 0.775 |
| walker |  | 399 | 56 | listing of 'ranges' |  |  | 0.788 |
| walker |  | 419 | 20 | ts names ranges/simplify.js |  |  | 0.788 |
| walker |  | 440 | 21 | ts names internal/debug.js |  |  | 0.788 |
| ns | 461 |  | 105 | Complete listing of functions/ (24 version-level modules) | 1.5 |  | 0.637 |
| walker |  | 462 | 22 | ts names classes/comparator.js |  |  | 0.637 |
| walker |  | 484 | 22 | ts names classes/range.js |  |  | 0.637 |
| walker |  | 506 | 22 | ts names classes/semver.js |  |  | 0.637 |
| ns | 517 |  | 56 | Complete listing of ranges/ (11 range-level modules) | 1.6 |  | 0.664 |
| walker |  | 530 | 24 | ts names internal/lrucache.js |  |  | 0.664 |
| walker |  | 555 | 25 | ts names internal/parse-options.js |  |  | 0.665 |
| walker |  | 584 | 29 | ts names ranges/min-version.js |  |  | 0.665 |
| walker |  | 613 | 29 | ts names ranges/valid.js |  |  | 0.665 |
| walker |  | 643 | 30 | ts names ranges/subset.js |  |  | 0.665 |
| walker |  | 673 | 30 | ts names ranges/to-comparators.js |  |  | 0.665 |
| walker |  | 704 | 31 | ts names ranges/intersects.js |  |  | 0.665 |
| ns | 712 |  | 195 | index.js aggregate export object, first half (parse..Range) | 1.7 |  | 0.571 |
| walker |  | 735 | 31 | ts names ranges/outside.js |  |  | 0.571 |
| walker |  | 840 | 105 | listing of 'functions' |  |  | 0.718 |
| walker |  | 867 | 27 | ts names functions/clean.js |  |  | 0.718 |
| walker |  | 894 | 27 | ts names functions/valid.js |  |  | 0.718 |
| ns | 909 |  | 197 | index.js aggregate export object, remainder (satisfies..rcompareIdentifiers) | 1.8 | 1.7 | 0.653 |
| walker |  | 922 | 28 | ts names functions/compare.js |  |  | 0.653 |
| walker |  | 940 | 18 | ts decl functions/compare.js:4 |  |  | 0.653 |
| walker |  | 969 | 29 | ts names functions/coerce.js |  |  | 0.653 |
| walker |  | 998 | 29 | ts names functions/diff.js |  |  | 0.653 |
| walker |  | 1027 | 29 | ts names functions/prerelease.js |  |  | 0.653 |
| walker |  | 1056 | 29 | ts names functions/satisfies.js |  |  | 0.653 |
| walker |  | 1087 | 31 | ts names functions/cmp.js |  |  | 0.653 |
| walker |  | 1118 | 31 | ts names functions/compare-build.js |  |  | 0.653 |
| walker |  | 1150 | 32 | ts names functions/parse.js |  |  | 0.653 |
| walker |  | 1180 | 30 | package runtime metadata in package.json |  |  | 0.653 |
| walker |  | 1213 | 33 | ts names functions/compare-loose.js |  |  | 0.653 |
| ns | 1230 |  | 321 | README section map: every heading, no bodies | 1.9 |  | 0.595 |
| walker |  | 1246 | 33 | ts names functions/major.js |  |  | 0.595 |
| walker |  | 1279 | 33 | ts names functions/minor.js |  |  | 0.595 |
| walker |  | 1312 | 33 | ts names functions/patch.js |  |  | 0.595 |
| walker |  | 1346 | 34 | ts names functions/inc.js |  |  | 0.595 |
| walker |  | 1381 | 35 | ts names functions/rcompare.js |  |  | 0.595 |
| walker |  | 1416 | 35 | ts names ranges/max-satisfying.js |  |  | 0.595 |
| walker |  | 1451 | 35 | ts names ranges/min-satisfying.js |  |  | 0.595 |
| walker |  | 1486 | 35 | ts decl ranges/to-comparators.js:6 |  |  | 0.595 |
| ns | 1491 |  | 261 | range.bnf: the complete formal grammar of range syntax | 1.10 |  | 0.563 |
| walker |  | 1523 | 37 | ts names functions/eq.js |  |  | 0.563 |
| walker |  | 1560 | 37 | ts names functions/gt.js |  |  | 0.563 |
| walker |  | 1597 | 37 | ts names functions/lt.js |  |  | 0.563 |
| walker |  | 1634 | 37 | ts names functions/neq.js |  |  | 0.563 |
| walker |  | 1671 | 37 | ts names ranges/gtr.js |  |  | 0.563 |
| walker |  | 1708 | 37 | ts names ranges/ltr.js |  |  | 0.563 |
| ns | 1744 |  | 253 | SemVer class: requires + complete method roster | 2.1 |  | 0.521 |
| walker |  | 1747 | 39 | ts names functions/gte.js |  |  | 0.521 |
| walker |  | 1786 | 39 | ts names functions/lte.js |  |  | 0.521 |
| walker |  | 1874 | 88 | ts decl classes/range.js:6 |  |  | 0.521 |
| walker |  | 1914 | 40 | ts names functions/sort.js |  |  | 0.521 |
| ns | 1950 |  | 206 | Comparator class: ANY sentinel, complete method roster, requires | 2.2 |  | 0.490 |
| walker |  | 1956 | 42 | ts names functions/rsort.js |  |  | 0.490 |
| walker |  | 2007 | 51 | ts decl internal/lrucache.js:3 |  |  | 0.491 |
| walker |  | 2035 | 28 | listing of '.github' |  |  | 0.491 |
| walker |  | 2076 | 41 | listing of '.github/workflows' |  |  | 0.492 |
| walker |  | 2141 | 65 | ts names internal/re.js |  |  | 0.492 |
| walker |  | 2174 | 33 | listing of 'test' |  |  | 0.492 |
| ns | 2216 |  | 266 | Range class: complete method roster + hoisted require block and LRU cache | 2.3 |  | 0.466 |
| walker |  | 2249 | 75 | ts names internal/identifiers.js |  |  | 0.467 |
| walker |  | 2287 | 38 | listing of 'benchmarks' |  |  | 0.467 |
| walker |  | 2360 | 73 | ts decl internal/debug.js:3 |  |  | 0.468 |
| walker |  | 2438 | 78 | ts decl classes/comparator.js:5 |  |  | 0.476 |
| ns | 2537 |  | 321 | range.js: complete roster of module-level range-desugaring helpers | 2.4 |  | 0.446 |
| ns | 2602 |  | 65 | classes/index.js barrel (whole file) | 2.5 |  | 0.452 |
| walker |  | 2818 | 380 | ts decl index.js:45 |  |  | 0.589 |
| walker |  | 2826 | 8 | ts body classes/comparator.js:6 |  |  | 0.590 |
| ns | 2857 |  | 255 | README usage: canonical calls against the aggregate export | 3.1 |  | 0.578 |
| walker |  | 2934 | 108 | ts names internal/constants.js |  |  | 0.578 |
| walker |  | 2953 | 19 | ts decl internal/constants.js:8 |  |  | 0.578 |
| walker |  | 3017 | 64 | ts decl internal/constants.js:18 |  |  | 0.579 |
| ns | 3065 |  | 208 | README: the options object (`loose`, `includePrerelease`) | 3.2 |  | 0.565 |
| walker |  | 3105 | 88 | ts decl internal/constants.js:28 |  |  | 0.566 |
| walker |  | 3114 | 9 | ts body classes/comparator.js:57 |  |  | 0.567 |
| walker |  | 3123 | 9 | ts body classes/range.js:92 |  |  | 0.568 |
| walker |  | 3132 | 9 | ts body classes/range.js:96 |  |  | 0.569 |
| ns | 3171 |  | 106 | README: what counts as a version | 3.3 |  | 0.561 |
| walker |  | 3192 | 60 | package identity metadata in package.json |  |  | 0.561 |
| walker |  | 3298 | 106 | ts decl classes/semver.js:9 |  |  | 0.569 |
| walker |  | 3307 | 9 | ts body classes/semver.js:89 |  |  | 0.570 |
| ns | 3331 |  | 160 | README: comparators and the complete primitive operator set | 3.4 |  | 0.558 |
| ns | 3445 |  | 114 | README: comparator sets intersect, `\|\|` unions them | 3.5 |  | 0.552 |
| walker |  | 3487 | 180 | headings outline in README.md |  |  | 0.575 |
| walker |  | 3512 | 25 | README.md section #1 |  |  | 0.575 |
| walker |  | 3539 | 27 | README.md section #55 |  |  | 0.575 |
| walker |  | 3556 | 17 | listing of 'test/classes' |  |  | 0.575 |
| walker |  | 3567 | 11 | ts body internal/lrucache.js:21 |  |  | 0.575 |
| ns | 3611 |  | 166 | README: the prerelease-tag matching rule | 3.6 |  | 0.564 |
| walker |  | 3702 | 135 | package entrypoints in package.json |  |  | 0.641 |
| ns | 3849 |  | 238 | README: the `inc` contract and the eight release types | 3.7 |  | 0.628 |
| walker |  | 3987 | 285 | README.md section #2 |  |  | 0.650 |
| walker |  | 4013 | 26 | ts body functions/valid.js:4 |  |  | 0.650 |
| walker |  | 4157 | 144 | package scripts in package.json |  |  | 0.651 |
| ns | 4177 |  | 328 | README: caret ranges (the left-most non-zero rule) and its desugaring table | 3.8 |  | 0.636 |
| walker |  | 4184 | 27 | listing of 'test/internal' |  |  | 0.636 |
| walker |  | 4187 | 3 | listing of 'tap-snapshots/test' |  |  | 0.636 |
| walker |  | 4223 | 36 | ts body functions/clean.js:4 |  |  | 0.636 |
| walker |  | 4259 | 36 | ts body functions/prerelease.js:4 |  |  | 0.636 |
| walker |  | 4282 | 23 | ts body internal/lrucache.js:4 |  |  | 0.637 |
| walker |  | 4327 | 45 | ts body ranges/intersects.js:4 |  |  | 0.637 |
| walker |  | 4348 | 21 | README.md section #21 |  |  | 0.637 |
| walker |  | 4369 | 21 | README.md section #22 |  |  | 0.637 |
| walker |  | 4390 | 21 | README.md section #23 |  |  | 0.637 |
| walker |  | 4411 | 21 | README.md section #24 |  |  | 0.637 |
| walker |  | 4433 | 22 | README.md section #35 |  |  | 0.637 |
| walker |  | 4455 | 22 | README.md section #52 |  |  | 0.637 |
| ns | 4467 |  | 290 | README: tilde ranges and their desugaring table | 3.9 |  | 0.628 |
| walker |  | 4507 | 52 | ts body functions/compare-build.js:4 |  |  | 0.628 |
| walker |  | 4562 | 55 | ts body functions/satisfies.js:4 |  |  | 0.628 |
| walker |  | 4587 | 25 | README.md section #13 |  |  | 0.628 |
| walker |  | 4692 | 105 | listing of 'test/functions' |  |  | 0.628 |
| walker |  | 4716 | 24 | README.md section #43 |  |  | 0.628 |
| ns | 4731 |  | 264 | README: X-ranges and bare partial versions | 3.10 |  | 0.617 |
| walker |  | 4740 | 24 | README.md section #48 |  |  | 0.617 |
| walker |  | 4796 | 56 | listing of 'test/ranges' |  |  | 0.617 |
| walker |  | 4825 | 29 | README.md section #14 |  |  | 0.617 |
| walker |  | 4848 | 23 | README.md section #34 |  |  | 0.617 |
| walker |  | 4852 | 4 | listing of '.github/matchers' |  |  | 0.617 |
| walker |  | 4856 | 4 | listing of 'test/integration' |  |  | 0.617 |
| ns | 4922 |  | 191 | README: hyphen ranges | 3.11 |  | 0.608 |
| walker |  | 4928 | 72 | ts body internal/parse-options.js:6 |  |  | 0.609 |
| walker |  | 4957 | 29 | README.md section #26 |  |  | 0.609 |
| walker |  | 4986 | 29 | README.md section #36 |  |  | 0.609 |
| walker |  | 5092 | 106 | README.md section #4 |  |  | 0.622 |
| ns | 5101 |  | 179 | README: ranges can be non-contiguous (the gtr/ltr/satisfies gotcha) | 3.12 |  | 0.615 |
| walker |  | 5122 | 30 | README.md section #39 |  |  | 0.615 |
| walker |  | 5206 | 84 | listing of 'test/fixtures' |  |  | 0.617 |
| walker |  | 5218 | 12 | ts names .commitlintrc.js |  |  | 0.617 |
| walker |  | 5230 | 12 | ts names .eslintrc.js |  |  | 0.617 |
| walker |  | 5242 | 12 | ts names .eslintrc.local.js |  |  | 0.617 |
| ns | 5314 |  | 213 | README: coercion limits | 3.13 |  | 0.608 |
| walker |  | 5321 | 79 | ts body ranges/valid.js:4 |  |  | 0.608 |
| walker |  | 5354 | 33 | README.md section #31 |  |  | 0.608 |
| walker |  | 5359 | 5 | listing of 'test/bin' |  |  | 0.608 |
| walker |  | 5456 | 97 | ts body functions/parse.js:4 |  |  | 0.608 |
| walker |  | 5493 | 37 | README.md section #40 |  |  | 0.608 |
| walker |  | 5529 | 36 | README.md section #41 |  |  | 0.608 |
| walker |  | 5566 | 37 | README.md section #47 |  |  | 0.608 |
| walker |  | 5631 | 65 | ts body classes/semver.js:81 |  |  | 0.609 |
| walker |  | 5675 | 44 | README.md section #37 |  |  | 0.609 |
| ns | 5694 |  | 380 | internal/constants.js (whole file): every tunable limit and flag | 4.1 |  | 0.610 |
| walker |  | 5717 | 42 | README.md section #38 |  |  | 0.610 |
| walker |  | 5761 | 44 | README.md section #45 |  |  | 0.610 |
| walker |  | 5806 | 45 | README.md section #53 |  |  | 0.610 |
| walker |  | 5853 | 47 | README.md section #29 |  |  | 0.610 |
| ns | 5956 |  | 262 | internal/parse-options.js and internal/debug.js (whole files) | 4.2 |  | 0.610 |
| walker |  | 5989 | 136 | ts body functions/inc.js:5 |  |  | 0.610 |
| walker |  | 5997 | 8 | listing of '.github/ISSUE_TEMPLATE' |  |  | 0.610 |
| walker |  | 6047 | 50 | README.md section #30 |  |  | 0.610 |
| walker |  | 6102 | 55 | README.md section #25 |  |  | 0.610 |
| walker |  | 6198 | 96 | ts body internal/lrucache.js:9 |  |  | 0.610 |
| walker |  | 6209 | 11 | listing of '.github/actions' |  |  | 0.610 |
| walker |  | 6213 | 4 | listing of '.github/actions/create-check' |  |  | 0.610 |
| walker |  | 6217 | 4 | listing of '.github/actions/install-latest-npm' |  |  | 0.610 |
| walker |  | 6227 | 10 | CHANGELOG.md section #0 |  |  | 0.610 |
| ns | 6262 |  | 306 | internal/identifiers.js (whole file): the prerelease ordering rule | 4.3 |  | 0.593 |
| ns | 6359 |  | 97 | internal/lrucache.js: class shape and the 1000-entry bound | 4.4 |  | 0.598 |
| walker |  | 6425 | 198 | ts body internal/identifiers.js:4 |  |  | 0.626 |
| ns | 6593 |  | 234 | internal/re.js: the four exported arrays and the createToken registrar | 4.5 |  | 0.614 |
| walker |  | 6624 | 199 | ts body ranges/max-satisfying.js:6 |  |  | 0.614 |
| ns | 6761 |  | 168 | internal/re.js: the ReDoS-safe regex construction | 4.6 |  | 0.607 |
| walker |  | 6823 | 199 | ts body ranges/min-satisfying.js:5 |  |  | 0.607 |
| walker |  | 6893 | 70 | README.md section #32 |  |  | 0.607 |
| walker |  | 6964 | 71 | README.md section #28 |  |  | 0.607 |
| walker |  | 7041 | 77 | README.md section #6 |  |  | 0.610 |
| walker |  | 7117 | 76 | README.md section #27 |  |  | 0.610 |
| walker |  | 7199 | 82 | README.md section #8 |  |  | 0.610 |
| walker |  | 7207 | 8 | listing of 'tap-snapshots/test/bin' |  |  | 0.610 |
| walker |  | 7351 | 144 | ts body classes/semver.js:93 |  |  | 0.610 |
| ns | 7436 |  | 675 | internal/re.js: complete roster of all 43 regex token names | 4.7 |  | 0.584 |
| walker |  | 7501 | 150 | ts body classes/comparator.js:61 |  |  | 0.584 |
| walker |  | 7586 | 85 | README.md section #51 |  |  | 0.584 |
| walker |  | 7677 | 91 | README.md section #10 |  |  | 0.595 |
| ns | 7755 |  | 319 | bin/semver.js: complete option-flag roster | 5.1 |  | 0.577 |
| walker |  | 7763 | 86 | README.md section #42 |  |  | 0.577 |
| ns | 7931 |  | 176 | bin/semver.js: usage line, the `-n` contract, and exit semantics | 5.2 |  | 0.570 |
| walker |  | 8058 | 295 | ts body ranges/subset.js:45 |  |  | 0.570 |
| walker |  | 8134 | 76 | README.md section #33 |  |  | 0.570 |
| ns | 8268 |  | 337 | bin/semver.js: the main() output pipeline | 5.3 |  | 0.558 |
| walker |  | 8304 | 170 | ts body classes/range.js:193 |  |  | 0.558 |
| walker |  | 8377 | 73 | README.md section #57 |  |  | 0.558 |
| walker |  | 8389 | 12 | ts names test/fixtures/comparator-intersection.js |  |  | 0.558 |
| walker |  | 8401 | 12 | ts names test/fixtures/comparisons.js |  |  | 0.558 |
| walker |  | 8413 | 12 | ts names test/fixtures/equality.js |  |  | 0.558 |
| ns | 8414 |  | 146 | SemVer.inc: complete roster of handled release types | 6.1 | 2.1 | 0.548 |
| walker |  | 8425 | 12 | ts names test/fixtures/increments.js |  |  | 0.548 |
| walker |  | 8437 | 12 | ts names test/fixtures/invalid-versions.js |  |  | 0.548 |
| walker |  | 8449 | 12 | ts names test/fixtures/range-exclude.js |  |  | 0.548 |
| walker |  | 8461 | 12 | ts names test/fixtures/range-include.js |  |  | 0.548 |
| walker |  | 8473 | 12 | ts names test/fixtures/range-intersection.js |  |  | 0.548 |
| walker |  | 8485 | 12 | ts names test/fixtures/range-parse.js |  |  | 0.548 |
| walker |  | 8497 | 12 | ts names test/fixtures/valid-versions.js |  |  | 0.548 |
| walker |  | 8509 | 12 | ts names test/fixtures/version-gt-range.js |  |  | 0.548 |
| walker |  | 8521 | 12 | ts names test/fixtures/version-lt-range.js |  |  | 0.548 |
| walker |  | 8533 | 12 | ts names test/fixtures/version-not-gt-range.js |  |  | 0.548 |
| walker |  | 8545 | 12 | ts names test/fixtures/version-not-lt-range.js |  |  | 0.548 |
| ns | 8644 |  | 230 | functions/cmp.js: the complete operator dispatch table | 6.2 |  | 0.539 |
| walker |  | 8678 | 133 | ts body internal/lrucache.js:25 |  |  | 0.540 |
| walker |  | 8790 | 112 | README.md section #9 |  |  | 0.540 |
| ns | 8886 |  | 242 | functions/coerce.js: the right-to-left scanning rule | 6.3 |  | 0.535 |
| ns | 9133 |  | 247 | functions/diff.js: the prerelease-to-release special cases | 6.4 |  | 0.528 |
| walker |  | 9174 | 384 | ts body functions/cmp.js:10 |  |  | 0.552 |
| ns | 9250 |  | 117 | Complete listings of test/ and test/fixtures/ | 7.1 |  | 0.565 |
| ns | 9400 |  | 150 | map.js + test/map.js: the enforced source-to-test mirror | 7.2 |  | 0.561 |
| walker |  | 9608 | 434 | ts body ranges/simplify.js:8 |  |  | 0.561 |
| ns | 9615 |  | 215 | package.json: npm scripts and tap configuration | 7.3 |  | 0.561 |
| walker |  | 9678 | 70 | ts decl .eslintrc.js:11 |  |  | 0.561 |
| ns | 9722 |  | 107 | Complete listings of benchmarks/, .github/ and .github/workflows/ | 7.4 |  | 0.571 |
| walker |  | 9811 | 133 | README.md section #50 |  |  | 0.571 |
| ns | 9812 |  | 90 | CONTRIBUTING.md: the rules that would silently fail a PR | 7.5 |  | 0.571 |
| ns | 9951 |  | 139 | .eslintrc.local.js: the constraints on published source | 7.6 |  | 0.566 |
| walker |  | 9996 | 185 | ts body classes/range.js:73 |  |  | 0.566 |
