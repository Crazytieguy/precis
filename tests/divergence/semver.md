Score(3000)=0.687 I=0.902 C=0.524 ns_rows≤3K=16/48 grid(1000/1442/2080/3000/4327/6240/9000)=0.653/0.951/0.785/0.687/0.643/0.627/0.521

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
| walker |  | 633 | 60 | Json::IdentityMeta { file: package.json } |  |  | 0.838 |
| walker |  | 650 | 17 | Fs::DirListing { dir: test/classes } |  |  | 0.838 |
| walker |  | 662 | 12 | Code::CodeKey { rung: Names, file: index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.838 |
| ns | 712 |  | 195 | index.js aggregate export object, first half (parse..Range) | 1.7 |  | 0.719 |
| ns | 909 |  | 197 | index.js aggregate export object, remainder (satisfies..rcompareIdentifiers) | 1.8 | 1.7 | 0.653 |
| walker |  | 1042 | 380 | Code::CodeKey { rung: Decl, file: index.js, decl: 1, sub: 0, line: 45 } |  |  | 0.873 |
| walker |  | 1222 | 180 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.878 |
| ns | 1230 |  | 321 | README section map: every heading, no bodies | 1.9 |  | 0.831 |
| walker |  | 1247 | 25 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.831 |
| walker |  | 1274 | 27 | Fs::DirListing { dir: test/internal } |  |  | 0.831 |
| walker |  | 1409 | 135 | Json::Entry { file: package.json } |  |  | 0.951 |
| ns | 1491 |  | 261 | range.bnf: the complete formal grammar of range syntax | 1.10 |  | 0.900 |
| walker |  | 1694 | 285 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: true } |  |  | 0.903 |
| ns | 1744 |  | 253 | SemVer class: requires + complete method roster | 2.1 |  | 0.834 |
| walker |  | 1838 | 144 | Json::Scripts { file: package.json } |  |  | 0.835 |
| walker |  | 1841 | 3 | Fs::DirListing { dir: tap-snapshots/test } |  |  | 0.835 |
| walker |  | 1946 | 105 | Fs::DirListing { dir: test/functions } |  |  | 0.835 |
| walker |  | 1950 | 4 | Fs::DirListing { dir: .github/matchers } |  |  | 0.785 |
| ns | 1950 |  | 206 | Comparator class: ANY sentinel, complete method roster, requires | 2.2 |  | 0.785 |
| walker |  | 1954 | 4 | Fs::DirListing { dir: test/integration } |  |  | 0.785 |
| walker |  | 2010 | 56 | Fs::DirListing { dir: test/ranges } |  |  | 0.785 |
| walker |  | 2094 | 84 | Fs::DirListing { dir: test/fixtures } |  |  | 0.787 |
| walker |  | 2099 | 5 | Fs::DirListing { dir: test/bin } |  |  | 0.787 |
| ns | 2216 |  | 266 | Range class: complete method roster + hoisted require block and LRU cache | 2.3 |  | 0.734 |
| ns | 2537 |  | 321 | range.js: complete roster of module-level range-desugaring helpers | 2.4 |  | 0.687 |
| ns | 2602 |  | 65 | classes/index.js barrel (whole file) | 2.5 |  | 0.678 |
| ns | 2857 |  | 255 | README usage: canonical calls against the aggregate export | 3.1 |  | 0.687 |
| walker |  | 2966 | 867 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: true } |  |  | 0.687 |
| ns | 3065 |  | 208 | README: the options object (`loose`, `includePrerelease`) | 3.2 |  | 0.671 |
| ns | 3171 |  | 106 | README: what counts as a version | 3.3 |  | 0.663 |
| ns | 3331 |  | 160 | README: comparators and the complete primitive operator set | 3.4 |  | 0.649 |
| ns | 3445 |  | 114 | README: comparator sets intersect, `\|\|` unions them | 3.5 |  | 0.642 |
| walker |  | 3529 | 563 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: true } |  |  | 0.642 |
| ns | 3611 |  | 166 | README: the prerelease-tag matching rule | 3.6 |  | 0.630 |
| walker |  | 3635 | 106 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.645 |
| walker |  | 3643 | 8 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.645 |
| walker |  | 3654 | 11 | Fs::DirListing { dir: .github/actions } |  |  | 0.645 |
| walker |  | 3658 | 4 | Fs::DirListing { dir: .github/actions/create-check } |  |  | 0.645 |
| walker |  | 3662 | 4 | Fs::DirListing { dir: .github/actions/install-latest-npm } |  |  | 0.645 |
| walker |  | 3670 | 8 | Fs::DirListing { dir: tap-snapshots/test/bin } |  |  | 0.645 |
| ns | 3849 |  | 238 | README: the `inc` contract and the eight release types | 3.7 |  | 0.632 |
| walker |  | 3883 | 213 | Markdown::Section { file: README.md, section_index: 14, keeps_default_concavity: false } |  |  | 0.658 |
| ns | 4177 |  | 328 | README: caret ranges (the left-most non-zero rule) and its desugaring table | 3.8 |  | 0.643 |
| walker |  | 4413 | 530 | Markdown::Section { file: README.md, section_index: 15, keeps_default_concavity: true } |  |  | 0.668 |
| walker |  | 4427 | 14 | Code::CodeKey { rung: Names, file: preload.js, decl: 0, sub: 0, line: 0 } |  |  | 0.668 |
| ns | 4467 |  | 290 | README: tilde ranges and their desugaring table | 3.9 |  | 0.659 |
| walker |  | 4689 | 262 | Markdown::Section { file: README.md, section_index: 20, keeps_default_concavity: false } |  |  | 0.659 |
| walker |  | 4710 | 21 | Code::CodeKey { rung: Names, file: map.js, decl: 0, sub: 0, line: 0 } |  |  | 0.659 |
| ns | 4731 |  | 264 | README: X-ranges and bare partial versions | 3.10 |  | 0.648 |
| ns | 4922 |  | 191 | README: hyphen ranges | 3.11 |  | 0.638 |
| ns | 5101 |  | 179 | README: ranges can be non-contiguous (the gtr/ltr/satisfies gotcha) | 3.12 |  | 0.631 |
| walker |  | 5141 | 431 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.664 |
| ns | 5314 |  | 213 | README: coercion limits | 3.13 |  | 0.656 |
| walker |  | 5582 | 441 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.659 |
| ns | 5694 |  | 380 | internal/constants.js (whole file): every tunable limit and flag | 4.1 |  | 0.631 |
| ns | 5956 |  | 262 | internal/parse-options.js and internal/debug.js (whole files) | 4.2 |  | 0.610 |
| walker |  | 6053 | 471 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.627 |
| ns | 6262 |  | 306 | internal/identifiers.js (whole file): the prerelease ordering rule | 4.3 |  | 0.607 |
| ns | 6359 |  | 97 | internal/lrucache.js: class shape and the 1000-entry bound | 4.4 |  | 0.598 |
| walker |  | 6522 | 469 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.601 |
| ns | 6593 |  | 234 | internal/re.js: the four exported arrays and the createToken registrar | 4.5 |  | 0.590 |
| ns | 6761 |  | 168 | internal/re.js: the ReDoS-safe regex construction | 4.6 |  | 0.584 |
| walker |  | 6960 | 438 | Markdown::Section { file: README.md, section_index: 10, keeps_default_concavity: false } |  |  | 0.607 |
| ns | 7436 |  | 675 | internal/re.js: complete roster of all 43 regex token names | 4.7 |  | 0.581 |
| walker |  | 7480 | 520 | Markdown::Section { file: README.md, section_index: 16, keeps_default_concavity: false } |  |  | 0.581 |
| ns | 7755 |  | 319 | bin/semver.js: complete option-flag roster | 5.1 |  | 0.563 |
| walker |  | 7830 | 350 | Json::Whole { file: release-please-config.json } |  |  | 0.563 |
| ns | 7931 |  | 176 | bin/semver.js: usage line, the `-n` contract, and exit semantics | 5.2 |  | 0.556 |
| ns | 8268 |  | 337 | bin/semver.js: the main() output pipeline | 5.3 |  | 0.544 |
| ns | 8414 |  | 146 | SemVer.inc: complete roster of handled release types | 6.1 | 2.1 | 0.535 |
| walker |  | 8493 | 663 | Markdown::Section { file: README.md, section_index: 17, keeps_default_concavity: false } |  |  | 0.535 |
| ns | 8644 |  | 230 | functions/cmp.js: the complete operator dispatch table | 6.2 |  | 0.525 |
| ns | 8886 |  | 242 | functions/coerce.js: the right-to-left scanning rule | 6.3 |  | 0.521 |
| walker |  | 9039 | 546 | Markdown::Section { file: README.md, section_index: 18, keeps_default_concavity: false } |  |  | 0.541 |
| ns | 9133 |  | 247 | functions/diff.js: the prerelease-to-release special cases | 6.4 |  | 0.534 |
| ns | 9250 |  | 117 | Complete listings of test/ and test/fixtures/ | 7.1 |  | 0.548 |
| ns | 9400 |  | 150 | map.js + test/map.js: the enforced source-to-test mirror | 7.2 |  | 0.544 |
| walker |  | 9569 | 530 | Markdown::Section { file: README.md, section_index: 19, keeps_default_concavity: false } |  |  | 0.544 |
| ns | 9615 |  | 215 | package.json: npm scripts and tap configuration | 7.3 |  | 0.545 |
| ns | 9722 |  | 107 | Complete listings of benchmarks/, .github/ and .github/workflows/ | 7.4 |  | 0.556 |
| ns | 9812 |  | 90 | CONTRIBUTING.md: the rules that would silently fail a PR | 7.5 |  | 0.555 |
| ns | 9951 |  | 139 | .eslintrc.local.js: the constraints on published source | 7.6 |  | 0.550 |
