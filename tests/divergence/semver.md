Score(3000)=0.686 I=0.898 C=0.524 ns_rows≤3K=16/48 grid(1000/1442/2080/3000/4327/6240/9000)=0.653/0.951/0.785/0.686/0.643/0.610/0.521

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 79 |  | 79 | Package identity: README title + package.json name/version/description/main | 1.1 |  | 0.000 |
| walker |  | 103 | 103 | listing of '.' |  |  | 0.000 |
| walker |  | 106 | 3 | listing of 'tap-snapshots' |  |  | 0.000 |
| walker |  | 111 | 5 | listing of 'bin' |  |  | 0.000 |
| walker |  | 123 | 12 | ts names index.js |  |  | 0.000 |
| walker |  | 140 | 17 | listing of 'classes' |  |  | 0.000 |
| walker |  | 166 | 26 | README headline in README.md |  |  | 0.219 |
| ns | 182 |  | 103 | Complete root directory listing | 1.2 |  | 0.661 |
| walker |  | 193 | 27 | listing of 'internal' |  |  | 0.687 |
| walker |  | 254 | 61 | package identity in package.json |  |  | 0.945 |
| ns | 307 |  | 125 | package.json `bin` + `files`: CLI entry point and published surface | 1.3 |  | 0.756 |
| walker |  | 310 | 56 | listing of 'ranges' |  |  | 0.768 |
| walker |  | 340 | 30 | package runtime metadata in package.json |  |  | 0.768 |
| ns | 356 |  | 49 | Complete listings of classes/, internal/ and bin/ | 1.4 |  | 0.786 |
| walker |  | 445 | 105 | listing of 'functions' |  |  | 0.820 |
| walker |  | 459 | 14 | ts names preload.js |  |  | 0.820 |
| ns | 461 |  | 105 | Complete listing of functions/ (24 version-level modules) | 1.5 |  | 0.834 |
| walker |  | 487 | 28 | listing of '.github' |  |  | 0.834 |
| ns | 517 |  | 56 | Complete listing of ranges/ (11 range-level modules) | 1.6 |  | 0.835 |
| walker |  | 528 | 41 | listing of '.github/workflows' |  |  | 0.836 |
| walker |  | 549 | 21 | ts names map.js |  |  | 0.836 |
| walker |  | 582 | 33 | listing of 'test' |  |  | 0.837 |
| walker |  | 620 | 38 | listing of 'benchmarks' |  |  | 0.838 |
| walker |  | 680 | 60 | package identity metadata in package.json |  |  | 0.838 |
| ns | 712 |  | 195 | index.js aggregate export object, first half (parse..Range) | 1.7 |  | 0.719 |
| ns | 909 |  | 197 | index.js aggregate export object, remainder (satisfies..rcompareIdentifiers) | 1.8 | 1.7 | 0.653 |
| walker |  | 1060 | 380 | ts decl index.js:45 |  |  | 0.873 |
| ns | 1230 |  | 321 | README section map: every heading, no bodies | 1.9 |  | 0.795 |
| walker |  | 1240 | 180 | headings outline in README.md |  |  | 0.831 |
| walker |  | 1265 | 25 | README.md section #1 |  |  | 0.831 |
| walker |  | 1282 | 17 | listing of 'test/classes' |  |  | 0.831 |
| walker |  | 1417 | 135 | package entrypoints in package.json |  |  | 0.951 |
| ns | 1491 |  | 261 | range.bnf: the complete formal grammar of range syntax | 1.10 |  | 0.900 |
| walker |  | 1702 | 285 | README.md section #2 |  |  | 0.903 |
| ns | 1744 |  | 253 | SemVer class: requires + complete method roster | 2.1 |  | 0.834 |
| walker |  | 1846 | 144 | package scripts in package.json |  |  | 0.835 |
| walker |  | 1873 | 27 | listing of 'test/internal' |  |  | 0.835 |
| walker |  | 1876 | 3 | listing of 'tap-snapshots/test' |  |  | 0.835 |
| ns | 1950 |  | 206 | Comparator class: ANY sentinel, complete method roster, requires | 2.2 |  | 0.785 |
| walker |  | 1981 | 105 | listing of 'test/functions' |  |  | 0.785 |
| walker |  | 2037 | 56 | listing of 'test/ranges' |  |  | 0.785 |
| ns | 2216 |  | 266 | Range class: complete method roster + hoisted require block and LRU cache | 2.3 |  | 0.732 |
| ns | 2537 |  | 321 | range.js: complete roster of module-level range-desugaring helpers | 2.4 |  | 0.686 |
| ns | 2602 |  | 65 | classes/index.js barrel (whole file) | 2.5 |  | 0.676 |
| ns | 2857 |  | 255 | README usage: canonical calls against the aggregate export | 3.1 |  | 0.686 |
| walker |  | 2904 | 867 | README.md section #3 |  |  | 0.686 |
| walker |  | 2908 | 4 | listing of '.github/matchers' |  |  | 0.686 |
| walker |  | 2912 | 4 | listing of 'test/integration' |  |  | 0.686 |
| ns | 3065 |  | 208 | README: the options object (`loose`, `includePrerelease`) | 3.2 |  | 0.670 |
| ns | 3171 |  | 106 | README: what counts as a version | 3.3 |  | 0.661 |
| ns | 3331 |  | 160 | README: comparators and the complete primitive operator set | 3.4 |  | 0.648 |
| ns | 3445 |  | 114 | README: comparator sets intersect, `\|\|` unions them | 3.5 |  | 0.640 |
| walker |  | 3475 | 563 | README.md section #4 |  |  | 0.640 |
| walker |  | 3581 | 106 | README.md section #5 |  |  | 0.655 |
| ns | 3611 |  | 166 | README: the prerelease-tag matching rule | 3.6 |  | 0.643 |
| walker |  | 3665 | 84 | listing of 'test/fixtures' |  |  | 0.645 |
| walker |  | 3670 | 5 | listing of 'test/bin' |  |  | 0.645 |
| walker |  | 3678 | 8 | listing of '.github/ISSUE_TEMPLATE' |  |  | 0.645 |
| walker |  | 3689 | 11 | listing of '.github/actions' |  |  | 0.645 |
| walker |  | 3693 | 4 | listing of '.github/actions/create-check' |  |  | 0.645 |
| walker |  | 3697 | 4 | listing of '.github/actions/install-latest-npm' |  |  | 0.645 |
| ns | 3849 |  | 238 | README: the `inc` contract and the eight release types | 3.7 |  | 0.632 |
| walker |  | 3910 | 213 | README.md section #14 |  |  | 0.658 |
| ns | 4177 |  | 328 | README: caret ranges (the left-most non-zero rule) and its desugaring table | 3.8 |  | 0.643 |
| walker |  | 4440 | 530 | README.md section #15 |  |  | 0.668 |
| walker |  | 4448 | 8 | listing of 'tap-snapshots/test/bin' |  |  | 0.668 |
| ns | 4467 |  | 290 | README: tilde ranges and their desugaring table | 3.9 |  | 0.659 |
| walker |  | 4710 | 262 | README.md section #20 |  |  | 0.659 |
| ns | 4731 |  | 264 | README: X-ranges and bare partial versions | 3.10 |  | 0.648 |
| ns | 4922 |  | 191 | README: hyphen ranges | 3.11 |  | 0.638 |
| ns | 5101 |  | 179 | README: ranges can be non-contiguous (the gtr/ltr/satisfies gotcha) | 3.12 |  | 0.631 |
| walker |  | 5141 | 431 | README.md section #6 |  |  | 0.664 |
| ns | 5314 |  | 213 | README: coercion limits | 3.13 |  | 0.656 |
| walker |  | 5582 | 441 | README.md section #7 |  |  | 0.659 |
| ns | 5694 |  | 380 | internal/constants.js (whole file): every tunable limit and flag | 4.1 |  | 0.631 |
| walker |  | 5932 | 350 | json config release-please-config.json |  |  | 0.631 |
| ns | 5956 |  | 262 | internal/parse-options.js and internal/debug.js (whole files) | 4.2 |  | 0.610 |
| ns | 6262 |  | 306 | internal/identifiers.js (whole file): the prerelease ordering rule | 4.3 |  | 0.591 |
| ns | 6359 |  | 97 | internal/lrucache.js: class shape and the 1000-entry bound | 4.4 |  | 0.582 |
| walker |  | 6403 | 471 | README.md section #8 |  |  | 0.598 |
| ns | 6593 |  | 234 | internal/re.js: the four exported arrays and the createToken registrar | 4.5 |  | 0.587 |
| ns | 6761 |  | 168 | internal/re.js: the ReDoS-safe regex construction | 4.6 |  | 0.580 |
| walker |  | 6872 | 469 | README.md section #9 |  |  | 0.584 |
| walker |  | 7310 | 438 | README.md section #10 |  |  | 0.607 |
| ns | 7436 |  | 675 | internal/re.js: complete roster of all 43 regex token names | 4.7 |  | 0.581 |
| ns | 7755 |  | 319 | bin/semver.js: complete option-flag roster | 5.1 |  | 0.563 |
| walker |  | 7830 | 520 | README.md section #16 |  |  | 0.563 |
| ns | 7931 |  | 176 | bin/semver.js: usage line, the `-n` contract, and exit semantics | 5.2 |  | 0.556 |
| ns | 8268 |  | 337 | bin/semver.js: the main() output pipeline | 5.3 |  | 0.544 |
| ns | 8414 |  | 146 | SemVer.inc: complete roster of handled release types | 6.1 | 2.1 | 0.535 |
| walker |  | 8605 | 775 | README.md section #21 |  |  | 0.535 |
| ns | 8644 |  | 230 | functions/cmp.js: the complete operator dispatch table | 6.2 |  | 0.525 |
| ns | 8886 |  | 242 | functions/coerce.js: the right-to-left scanning rule | 6.3 |  | 0.521 |
| walker |  | 9032 | 427 | plaintext config .github/workflows/codeql-analysis.yml |  |  | 0.521 |
| ns | 9133 |  | 247 | functions/diff.js: the prerelease-to-release special cases | 6.4 |  | 0.513 |
| ns | 9250 |  | 117 | Complete listings of test/ and test/fixtures/ | 7.1 |  | 0.529 |
| ns | 9400 |  | 150 | map.js + test/map.js: the enforced source-to-test mirror | 7.2 |  | 0.525 |
| ns | 9615 |  | 215 | package.json: npm scripts and tap configuration | 7.3 |  | 0.526 |
| walker |  | 9695 | 663 | README.md section #17 |  |  | 0.526 |
| ns | 9722 |  | 107 | Complete listings of benchmarks/, .github/ and .github/workflows/ | 7.4 |  | 0.538 |
| ns | 9812 |  | 90 | CONTRIBUTING.md: the rules that would silently fail a PR | 7.5 |  | 0.537 |
| ns | 9951 |  | 139 | .eslintrc.local.js: the constraints on published source | 7.6 |  | 0.532 |
