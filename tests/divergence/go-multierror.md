Score(3000)=0.517 I=0.569 C=0.470 ns_rows≤3K=21/56 grid(1000/1442/2080/3000/4327/6240/9000)=0.554/0.513/0.518/0.517/0.627/0.883/0.804

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 54 |  | 54 | README title + what the package is | 1.1 |  | 0.000 |
| walker |  | 80 | 80 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 83 |  | 29 | go.mod: module path and language version | 1.2 |  | 0.000 |
| walker |  | 98 | 18 | Code::CodeKey { rung: Names, file: prefix.go, decl: 0, sub: 0, line: 0 } |  |  | 0.000 |
| walker |  | 118 | 20 | Code::CodeKey { rung: Names, file: append.go, decl: 0, sub: 0, line: 0 } |  |  | 0.000 |
| walker |  | 147 | 29 | GoMod::Identity { file: go.mod } |  |  | 0.564 |
| walker |  | 147 | 0 | GoMod::File { file: go.mod } |  |  | 0.564 |
| ns | 163 |  | 80 | Complete root directory listing | 1.3 |  | 0.646 |
| walker |  | 179 | 32 | Code::CodeKey { rung: Names, file: flatten.go, decl: 0, sub: 0, line: 0 } |  |  | 0.651 |
| walker |  | 213 | 34 | Code::CodeKey { rung: Names, file: format.go, decl: 0, sub: 0, line: 0 } |  |  | 0.659 |
| ns | 240 |  | 77 | README: why returning a list-of-errors as an error works | 1.4 |  | 0.599 |
| walker |  | 261 | 48 | Code::CodeKey { rung: Names, file: group.go, decl: 0, sub: 0, line: 0 } |  |  | 0.612 |
| walker |  | 294 | 33 | Code::CodeKey { rung: Decl, file: group.go, decl: 1, sub: 0, line: 10 } |  |  | 0.615 |
| ns | 315 |  | 75 | The `Error` type: doc comment and both fields | 1.5 |  | 0.558 |
| walker |  | 349 | 55 | Code::CodeKey { rung: Names, file: sort.go, decl: 0, sub: 0, line: 0 } |  |  | 0.560 |
| walker |  | 363 | 14 | Fs::DirListing { dir: .github } |  |  | 0.560 |
| walker |  | 375 | 12 | Fs::DirListing { dir: .github/workflows } |  |  | 0.561 |
| ns | 467 |  | 152 | Package-level API roster: every exported func/type outside multierror.go, with full signatures | 1.6 |  | 0.532 |
| walker |  | 482 | 107 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.548 |
| walker |  | 531 | 49 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.549 |
| walker |  | 544 | 13 | Code::CodeKey { rung: Doc, file: sort.go, decl: 1, sub: 0, line: 7 } |  |  | 0.549 |
| walker |  | 558 | 14 | Code::CodeKey { rung: Doc, file: sort.go, decl: 2, sub: 0, line: 16 } |  |  | 0.549 |
| walker |  | 572 | 14 | Code::CodeKey { rung: Doc, file: sort.go, decl: 3, sub: 0, line: 21 } |  |  | 0.549 |
| ns | 606 |  | 139 | Every method on `Error`, across multierror.go and sort.go | 1.7 |  | 0.481 |
| ns | 682 |  | 76 | The unexported `chain` type and its four methods | 1.8 |  | 0.449 |
| walker |  | 746 | 174 | Code::CodeKey { rung: Names, file: multierror.go, decl: 0, sub: 0, line: 0 } |  |  | 0.534 |
| walker |  | 769 | 23 | Code::CodeKey { rung: Decl, file: multierror.go, decl: 1, sub: 0, line: 13 } |  |  | 0.544 |
| walker |  | 784 | 15 | Code::CodeKey { rung: Body, file: multierror.go, decl: 4, sub: 0, line: 42 } |  |  | 0.551 |
| walker |  | 793 | 9 | Code::CodeKey { rung: Doc, file: multierror.go, decl: 8, sub: 0, line: 102 } |  |  | 0.551 |
| walker |  | 824 | 31 | Code::CodeKey { rung: Body, file: multierror.go, decl: 5, sub: 0, line: 53 } |  |  | 0.559 |
| ns | 853 |  | 171 | README section map: every heading and every bold subsection label | 1.9 |  | 0.517 |
| walker |  | 862 | 38 | Code::CodeKey { rung: Doc, file: multierror.go, decl: 1, sub: 0, line: 13 } |  |  | 0.552 |
| ns | 946 |  | 93 | README deprecation note: prefer stdlib `errors.Join` | 2.1 |  | 0.554 |
| ns | 1070 |  | 124 | `Append` doc comment: nil handling and one-level flattening | 2.2 | 1.6 | 0.528 |
| walker |  | 1250 | 388 | Plaintext::Whole { file: Makefile } |  |  | 0.532 |
| walker |  | 1261 | 11 | Code::CodeKey { rung: Body, file: multierror.go, decl: 8, sub: 0, line: 102 } |  |  | 0.538 |
| walker |  | 1279 | 18 | Code::CodeKey { rung: Body, file: sort.go, decl: 3, sub: 0, line: 21 } |  |  | 0.544 |
| walker |  | 1308 | 29 | Code::CodeKey { rung: Doc, file: group.go, decl: 1, sub: 0, line: 10 } |  |  | 0.545 |
| walker |  | 1338 | 30 | Code::CodeKey { rung: Doc, file: flatten.go, decl: 1, sub: 0, line: 8 } |  |  | 0.545 |
| ns | 1360 |  | 290 | README: stdlib compatibility, install, and the go 1.13 requirement | 2.3 |  | 0.501 |
| walker |  | 1368 | 30 | Code::CodeKey { rung: Doc, file: group.go, decl: 3, sub: 0, line: 36 } |  |  | 0.502 |
| walker |  | 1381 | 13 | Code::CodeKey { rung: Body, file: multierror.go, decl: 10, sub: 0, line: 117 } |  |  | 0.507 |
| walker |  | 1402 | 21 | Code::CodeKey { rung: Body, file: sort.go, decl: 2, sub: 0, line: 16 } |  |  | 0.513 |
| walker |  | 1435 | 33 | Code::CodeKey { rung: Doc, file: format.go, decl: 1, sub: 0, line: 13 } |  |  | 0.513 |
| walker |  | 1469 | 34 | Code::CodeKey { rung: Doc, file: format.go, decl: 2, sub: 0, line: 17 } |  |  | 0.514 |
| walker |  | 1505 | 36 | Code::CodeKey { rung: Body, file: group.go, decl: 3, sub: 0, line: 36 } |  |  | 0.520 |
| walker |  | 1518 | 13 | Code::CodeKey { rung: Body, file: multierror.go, decl: 11, sub: 0, line: 122 } |  |  | 0.526 |
| walker |  | 1568 | 50 | Code::CodeKey { rung: Body, file: multierror.go, decl: 2, sub: 0, line: 18 } |  |  | 0.533 |
| ns | 1580 |  | 220 | README: the canonical accumulate-with-Append recipe | 2.4 |  | 0.489 |
| walker |  | 1618 | 50 | Code::CodeKey { rung: Doc, file: group.go, decl: 2, sub: 0, line: 20 } |  |  | 0.492 |
| walker |  | 1634 | 16 | Code::CodeKey { rung: Doc, file: multierror.go, decl: 11, sub: 0, line: 122 } |  |  | 0.492 |
| walker |  | 1672 | 38 | Code::CodeKey { rung: Body, file: sort.go, decl: 1, sub: 0, line: 7 } |  |  | 0.498 |
| walker |  | 1760 | 88 | Code::CodeKey { rung: Doc, file: prefix.go, decl: 1, sub: 0, line: 16 } |  |  | 0.502 |
| ns | 1770 |  | 190 | `ErrorOrNil` and `WrappedErrors` doc comments | 2.5 | 1.7 | 0.480 |
| walker |  | 1859 | 99 | Code::CodeKey { rung: Body, file: flatten.go, decl: 1, sub: 0, line: 8 } |  |  | 0.487 |
| walker |  | 1877 | 18 | Code::CodeKey { rung: Doc, file: multierror.go, decl: 10, sub: 0, line: 117 } |  |  | 0.487 |
| ns | 1945 |  | 175 | `Error.Unwrap` doc comment: ordering, shallow copy, errors.As/Is support | 2.6 | 1.7 | 0.467 |
| walker |  | 2001 | 124 | Code::CodeKey { rung: Doc, file: append.go, decl: 1, sub: 0, line: 14 } |  |  | 0.500 |
| walker |  | 2063 | 62 | Code::CodeKey { rung: Body, file: multierror.go, decl: 3, sub: 0, line: 31 } |  |  | 0.518 |
| ns | 2063 |  | 118 | `Flatten` and `Prefix` doc comments | 2.7 | 1.6 | 0.518 |
| ns | 2174 |  | 111 | `Group` doc comments: Go and Wait | 2.8 | 1.6 | 0.528 |
| walker |  | 2205 | 142 | Code::CodeKey { rung: Body, file: format.go, decl: 2, sub: 0, line: 17 } |  |  | 0.539 |
| walker |  | 2277 | 72 | Code::CodeKey { rung: Doc, file: multierror.go, decl: 3, sub: 0, line: 31 } |  |  | 0.544 |
| ns | 2383 |  | 209 | format.go semantics: the formatter hook and the default output strings | 2.9 | 1.6 | 0.559 |
| walker |  | 2475 | 198 | Code::CodeKey { rung: Body, file: prefix.go, decl: 1, sub: 0, line: 16 } |  |  | 0.567 |
| ns | 2508 |  | 125 | README: customizing the message via `ErrorFormat` | 2.10 |  | 0.545 |
| walker |  | 2576 | 101 | Code::CodeKey { rung: Body, file: group.go, decl: 2, sub: 0, line: 20 } |  |  | 0.552 |
| walker |  | 2593 | 17 | Code::CodeKey { rung: Names, file: group_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.552 |
| walker |  | 2674 | 81 | Code::CodeKey { rung: Body, file: flatten.go, decl: 2, sub: 0, line: 20 } |  |  | 0.558 |
| ns | 2687 |  | 179 | README: getting at the individual errors | 2.11 |  | 0.537 |
| ns | 2990 |  | 303 | README: `errors.As` extraction and `errors.Is` sentinel checks | 2.12 |  | 0.506 |
| walker |  | 2994 | 320 | Code::CodeKey { rung: Body, file: append.go, decl: 1, sub: 0, line: 14 } |  |  | 0.517 |
| walker |  | 3028 | 34 | Code::CodeKey { rung: Doc, file: multierror.go, decl: 9, sub: 0, line: 108 } |  |  | 0.517 |
| ns | 3125 |  | 135 | README: returning a multierror only if there are errors | 2.13 |  | 0.503 |
| walker |  | 3146 | 118 | Code::CodeKey { rung: Doc, file: multierror.go, decl: 5, sub: 0, line: 53 } |  |  | 0.528 |
| ns | 3302 |  | 177 | `chain` doc comment: why the type exists and its precondition | 2.14 | 1.8 | 0.516 |
| walker |  | 3396 | 250 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.566 |
| ns | 3485 |  | 183 | README migration guide: basic aggregation with `errors.Join` | 2.15 |  | 0.543 |
| ns | 3645 |  | 160 | README migration guide: replacing custom formatting | 2.16 |  | 0.527 |
| walker |  | 3695 | 299 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: true } |  |  | 0.577 |
| ns | 3974 |  | 329 | README migration guide: replacing `Group` with a mutex collector | 2.17 |  | 0.545 |
| walker |  | 4071 | 376 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.615 |
| ns | 4294 |  | 320 | `Append` body: the type switch, flattening, and nil filtering | 3.1 | 1.6 | 0.627 |
| walker |  | 4412 | 341 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: true } |  |  | 0.680 |
| ns | 4464 |  | 170 | Bodies of `Error`, `ErrorOrNil`, `GoString`, `WrappedErrors` | 3.2 | 1.7 | 0.679 |
| ns | 4621 |  | 157 | `Error.Unwrap` body: empty/single fast paths and the shallow copy into `chain` | 3.3 | 1.7 | 0.664 |
| walker |  | 4651 | 239 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.800 |
| ns | 4753 |  | 132 | `chain` method bodies: Error/Unwrap/As/Is | 3.4 | 1.8 | 0.788 |
| ns | 4939 |  | 186 | flatten.go bodies: exported `Flatten` and the recursive helper | 3.5 | 1.6 | 0.790 |
| walker |  | 4958 | 307 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.815 |
| walker |  | 4992 | 34 | Code::CodeKey { rung: Names, file: flatten_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.815 |
| walker |  | 5026 | 34 | Code::CodeKey { rung: Names, file: sort_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.816 |
| walker |  | 5066 | 40 | Code::CodeKey { rung: Body, file: multierror.go, decl: 9, sub: 0, line: 108 } |  |  | 0.826 |
| walker |  | 5104 | 38 | Code::CodeKey { rung: Names, file: format_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.826 |
| ns | 5137 |  | 198 | `Prefix` body: in-place rewrite of each wrapped error | 3.6 | 1.6 | 0.828 |
| walker |  | 5258 | 154 | Code::CodeKey { rung: Body, file: multierror.go, decl: 6, sub: 0, line: 71 } |  |  | 0.849 |
| ns | 5311 |  | 174 | group.go bodies: the struct's unexported fields, `Go`, and `Wait` | 3.7 | 1.6 | 0.848 |
| walker |  | 5312 | 54 | Code::CodeKey { rung: Names, file: prefix_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.848 |
| ns | 5401 |  | 90 | sort.go bodies: Len/Swap/Less | 3.8 | 1.7 | 0.846 |
| ns | 5477 |  | 76 | Every import block in the package: stdlib only | 3.9 |  | 0.834 |
| walker |  | 5487 | 175 | Code::CodeKey { rung: Doc, file: multierror.go, decl: 6, sub: 0, line: 71 } |  |  | 0.850 |
| ns | 5520 |  | 43 | Standard file header: copyright, SPDX tag, package clause | 3.10 |  | 0.847 |
| ns | 5825 |  | 305 | Complete roster of test functions across all seven `*_test.go` files | 4.1 |  | 0.828 |
| ns | 6011 |  | 186 | Every `t.Run` subtest name in multierror_test.go | 4.2 | 4.1 | 0.820 |
| walker |  | 6040 | 553 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.883 |
| walker |  | 6096 | 56 | Markdown::HeadingsOutline { file: CHANGELOG.md } |  |  | 0.883 |
| ns | 6246 |  | 235 | The golden formatted output, as asserted in tests | 4.3 | 4.1 | 0.859 |
| walker |  | 6298 | 202 | Plaintext::Whole { file: .github/workflows/actionlint.yml } |  |  | 0.861 |
| walker |  | 6410 | 112 | Code::CodeKey { rung: Names, file: append_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.868 |
| walker |  | 6444 | 34 | Markdown::HeadingsOutline { file: .github/pull_request_template.md } |  |  | 0.868 |
| walker |  | 6610 | 166 | Code::CodeKey { rung: Names, file: multierror_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.882 |
| walker |  | 6621 | 11 | Code::CodeKey { rung: Body, file: multierror_test.go, decl: 1, sub: 0, line: 13 } |  |  | 0.882 |
| ns | 6658 |  | 412 | group_test.go: the concurrency table and its assertion loop | 4.4 | 4.1 | 0.853 |
| walker |  | 6798 | 177 | Code::CodeKey { rung: Doc, file: multierror.go, decl: 7, sub: 0, line: 99 } |  |  | 0.866 |
| walker |  | 6813 | 15 | Code::CodeKey { rung: Doc, file: multierror_test.go, decl: 9, sub: 0, line: 209 } |  |  | 0.866 |
| ns | 6837 |  | 179 | Test-only helper `nestedError` and the `errors.As` target pattern | 4.5 | 4.1 | 0.852 |
| walker |  | 6863 | 50 | Code::CodeKey { rung: Body, file: prefix_test.go, decl: 2, sub: 0, line: 22 } |  |  | 0.852 |
| walker |  | 6918 | 55 | Code::CodeKey { rung: Body, file: flatten_test.go, decl: 2, sub: 0, line: 43 } |  |  | 0.852 |
| walker |  | 6977 | 59 | Code::CodeKey { rung: Body, file: append_test.go, decl: 2, sub: 0, line: 45 } |  |  | 0.852 |
| walker |  | 7001 | 24 | Markdown::Section { file: .github/pull_request_template.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.852 |
| walker |  | 7064 | 63 | Code::CodeKey { rung: Body, file: append_test.go, decl: 5, sub: 0, line: 71 } |  |  | 0.852 |
| walker |  | 7130 | 66 | Code::CodeKey { rung: Body, file: append_test.go, decl: 4, sub: 0, line: 62 } |  |  | 0.852 |
| walker |  | 7196 | 66 | Code::CodeKey { rung: Body, file: append_test.go, decl: 6, sub: 0, line: 79 } |  |  | 0.852 |
| ns | 7254 |  | 417 | append_test.go: the nil / typed-nil / flattening cases in full | 4.6 | 4.1 | 0.823 |
| ns | 7361 |  | 107 | Test file preambles: package clause and imports | 4.7 |  | 0.812 |
| ns | 7387 |  | 26 | Complete `.github/` tree listing | 5.1 |  | 0.813 |
| ns | 7461 |  | 74 | Makefile: every target plus the TEST variable | 5.2 |  | 0.814 |
| ns | 7710 |  | 249 | Makefile recipes for test, testrace, updatedeps and generate | 5.3 | 5.2 | 0.816 |
| ns | 8035 |  | 325 | Main CI workflow skeleton: triggers, permissions, all four jobs, the Go matrix | 5.4 |  | 0.794 |
| walker |  | 8053 | 857 | Plaintext::Whole { file: .github/workflows/go-multierror.yml } |  |  | 0.808 |
| walker |  | 8140 | 87 | Code::CodeKey { rung: Body, file: prefix_test.go, decl: 3, sub: 0, line: 30 } |  |  | 0.808 |
| walker |  | 8231 | 91 | Code::CodeKey { rung: Body, file: prefix_test.go, decl: 1, sub: 0, line: 11 } |  |  | 0.808 |
| walker |  | 8298 | 67 | Code::CodeKey { rung: Body, file: append_test.go, decl: 3, sub: 0, line: 53 } |  |  | 0.809 |
| walker |  | 8403 | 105 | Code::CodeKey { rung: Body, file: format_test.go, decl: 1, sub: 0, line: 11 } |  |  | 0.809 |
| ns | 8467 |  | 432 | CI steps: the `go fmt` gate and the golangci-lint job | 5.5 | 5.4 | 0.812 |
| walker |  | 8526 | 123 | Code::CodeKey { rung: Body, file: format_test.go, decl: 2, sub: 0, line: 27 } |  |  | 0.812 |
| walker |  | 8653 | 127 | Code::CodeKey { rung: Body, file: multierror_test.go, decl: 2, sub: 0, line: 17 } |  |  | 0.812 |
| walker |  | 8782 | 129 | Code::CodeKey { rung: Body, file: multierror_test.go, decl: 3, sub: 0, line: 33 } |  |  | 0.818 |
| ns | 8852 |  | 385 | CI: how the linux test job actually runs the suite | 5.6 | 5.4 | 0.804 |
| walker |  | 8917 | 135 | Code::CodeKey { rung: Body, file: sort_test.go, decl: 1, sub: 0, line: 13 } |  |  | 0.804 |
| walker |  | 8940 | 23 | Markdown::Section { file: .github/pull_request_template.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.804 |
| ns | 9035 |  | 183 | CI: what the windows job does differently | 5.7 | 5.4 | 0.796 |
| walker |  | 9131 | 191 | Code::CodeKey { rung: Body, file: sort_test.go, decl: 2, sub: 0, line: 32 } |  |  | 0.796 |
| ns | 9237 |  | 202 | actionlint workflow, in full | 5.8 |  | 0.798 |
| walker |  | 9264 | 133 | Code::CodeKey { rung: Body, file: multierror_test.go, decl: 4, sub: 0, line: 51 } |  |  | 0.798 |
| ns | 9388 |  | 151 | Dependabot configuration | 5.9 |  | 0.789 |
| ns | 9477 |  | 89 | CHANGELOG skeleton, CODEOWNERS, and the pinned Go toolchain file | 5.10 |  | 0.786 |
| walker |  | 9502 | 238 | Code::CodeKey { rung: Body, file: flatten_test.go, decl: 1, sub: 0, line: 13 } |  |  | 0.800 |
| walker |  | 9539 | 37 | Markdown::Section { file: .github/pull_request_template.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.800 |
| walker |  | 9687 | 148 | Code::CodeKey { rung: Body, file: multierror_test.go, decl: 5, sub: 0, line: 65 } |  |  | 0.800 |
| ns | 9744 |  | 267 | README badge block and the older-Go compile-error hint | 5.11 | 1.1 | 0.795 |
| ns | 9803 |  | 59 | PR template section headings | 5.12 |  | 0.793 |
| ns | 9840 |  | 37 | LICENSE identification lines | 5.13 |  | 0.792 |
