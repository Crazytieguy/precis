Score(3000)=0.614 I=0.802 C=0.470 ns_rows≤3K=21/56 grid(1000/1442/2080/3000/4327/6240/9000)=0.450/0.467/0.518/0.614/0.764/0.846/0.784

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 54 |  | 54 | README title + what the package is | 1.1 |  | 0.000 |
| walker |  | 80 | 80 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 83 |  | 29 | go.mod: module path and language version | 1.2 |  | 0.000 |
| ns | 163 |  | 80 | Complete root directory listing | 1.3 |  | 0.519 |
| walker |  | 187 | 107 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.541 |
| walker |  | 216 | 29 | GoMod::Identity { file: go.mod } |  |  | 0.665 |
| walker |  | 216 | 0 | GoMod::File { file: go.mod } |  |  | 0.665 |
| ns | 240 |  | 77 | README: why returning a list-of-errors as an error works | 1.4 |  | 0.604 |
| walker |  | 265 | 49 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.606 |
| walker |  | 279 | 14 | Fs::DirListing { dir: .github } |  |  | 0.606 |
| walker |  | 291 | 12 | Fs::DirListing { dir: .github/workflows } |  |  | 0.607 |
| ns | 315 |  | 75 | The `Error` type: doc comment and both fields | 1.5 |  | 0.551 |
| ns | 467 |  | 152 | Package-level API roster: every exported func/type outside multierror.go, with full signatures | 1.6 |  | 0.445 |
| ns | 606 |  | 139 | Every method on `Error`, across multierror.go and sort.go | 1.7 |  | 0.385 |
| walker |  | 679 | 388 | Plaintext::Whole { file: Makefile } |  |  | 0.388 |
| ns | 682 |  | 76 | The unexported `chain` type and its four methods | 1.8 |  | 0.362 |
| walker |  | 777 | 98 | Code::CodeKey { rung: Names, file: multierror.go, decl: 0, sub: 0, line: 0 } |  |  | 0.389 |
| walker |  | 800 | 23 | Code::CodeKey { rung: Decl, file: multierror.go, decl: 1, sub: 0, line: 13 } |  |  | 0.400 |
| walker |  | 815 | 15 | Code::CodeKey { rung: Body, file: multierror.go, decl: 4, sub: 0, line: 42 } |  |  | 0.405 |
| walker |  | 853 | 38 | Code::CodeKey { rung: Doc, file: multierror.go, decl: 1, sub: 0, line: 13 } |  |  | 0.414 |
| ns | 853 |  | 171 | README section map: every heading and every bold subsection label | 1.9 |  | 0.414 |
| walker |  | 884 | 31 | Code::CodeKey { rung: Body, file: multierror.go, decl: 5, sub: 0, line: 53 } |  |  | 0.419 |
| walker |  | 904 | 20 | Code::CodeKey { rung: Names, file: append.go, decl: 0, sub: 0, line: 0 } |  |  | 0.420 |
| ns | 946 |  | 93 | README deprecation note: prefer stdlib `errors.Join` | 2.1 |  | 0.423 |
| walker |  | 952 | 48 | Code::CodeKey { rung: Names, file: group.go, decl: 0, sub: 0, line: 0 } |  |  | 0.434 |
| walker |  | 959 | 7 | Code::CodeKey { rung: Decl, file: group.go, decl: 1, sub: 0, line: 10 } |  |  | 0.434 |
| walker |  | 993 | 34 | Code::CodeKey { rung: Names, file: format.go, decl: 0, sub: 0, line: 0 } |  |  | 0.450 |
| walker |  | 1008 | 15 | Code::CodeKey { rung: Names, file: flatten.go, decl: 0, sub: 0, line: 0 } |  |  | 0.459 |
| walker |  | 1026 | 18 | Code::CodeKey { rung: Names, file: prefix.go, decl: 0, sub: 0, line: 0 } |  |  | 0.472 |
| ns | 1070 |  | 124 | `Append` doc comment: nil handling and one-level flattening | 2.2 | 1.6 | 0.450 |
| walker |  | 1081 | 55 | Code::CodeKey { rung: Names, file: sort.go, decl: 0, sub: 0, line: 0 } |  |  | 0.484 |
| walker |  | 1094 | 13 | Code::CodeKey { rung: Doc, file: sort.go, decl: 1, sub: 0, line: 7 } |  |  | 0.484 |
| walker |  | 1108 | 14 | Code::CodeKey { rung: Doc, file: sort.go, decl: 2, sub: 0, line: 16 } |  |  | 0.484 |
| walker |  | 1122 | 14 | Code::CodeKey { rung: Doc, file: sort.go, decl: 3, sub: 0, line: 21 } |  |  | 0.484 |
| walker |  | 1140 | 18 | Code::CodeKey { rung: Body, file: sort.go, decl: 3, sub: 0, line: 21 } |  |  | 0.490 |
| walker |  | 1161 | 21 | Code::CodeKey { rung: Body, file: sort.go, decl: 2, sub: 0, line: 16 } |  |  | 0.496 |
| walker |  | 1190 | 29 | Code::CodeKey { rung: Doc, file: group.go, decl: 1, sub: 0, line: 10 } |  |  | 0.497 |
| walker |  | 1220 | 30 | Code::CodeKey { rung: Doc, file: flatten.go, decl: 1, sub: 0, line: 8 } |  |  | 0.497 |
| walker |  | 1250 | 30 | Code::CodeKey { rung: Doc, file: group.go, decl: 3, sub: 0, line: 36 } |  |  | 0.498 |
| walker |  | 1322 | 72 | Code::CodeKey { rung: Doc, file: multierror.go, decl: 3, sub: 0, line: 31 } |  |  | 0.499 |
| walker |  | 1355 | 33 | Code::CodeKey { rung: Doc, file: format.go, decl: 1, sub: 0, line: 13 } |  |  | 0.499 |
| ns | 1360 |  | 290 | README: stdlib compatibility, install, and the go 1.13 requirement | 2.3 |  | 0.459 |
| walker |  | 1389 | 34 | Code::CodeKey { rung: Doc, file: format.go, decl: 2, sub: 0, line: 17 } |  |  | 0.460 |
| walker |  | 1439 | 50 | Code::CodeKey { rung: Body, file: multierror.go, decl: 2, sub: 0, line: 18 } |  |  | 0.467 |
| walker |  | 1475 | 36 | Code::CodeKey { rung: Body, file: group.go, decl: 3, sub: 0, line: 36 } |  |  | 0.472 |
| walker |  | 1525 | 50 | Code::CodeKey { rung: Doc, file: group.go, decl: 2, sub: 0, line: 20 } |  |  | 0.475 |
| ns | 1580 |  | 220 | README: the canonical accumulate-with-Append recipe | 2.4 |  | 0.436 |
| walker |  | 1643 | 118 | Code::CodeKey { rung: Doc, file: multierror.go, decl: 5, sub: 0, line: 53 } |  |  | 0.441 |
| walker |  | 1681 | 38 | Code::CodeKey { rung: Body, file: sort.go, decl: 1, sub: 0, line: 7 } |  |  | 0.447 |
| ns | 1770 |  | 190 | `ErrorOrNil` and `WrappedErrors` doc comments | 2.5 | 1.7 | 0.468 |
| walker |  | 1931 | 250 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.547 |
| ns | 1945 |  | 175 | `Error.Unwrap` doc comment: ordering, shallow copy, errors.As/Is support | 2.6 | 1.7 | 0.525 |
| ns | 2063 |  | 118 | `Flatten` and `Prefix` doc comments | 2.7 | 1.6 | 0.512 |
| ns | 2174 |  | 111 | `Group` doc comments: Go and Wait | 2.8 | 1.6 | 0.521 |
| walker |  | 2307 | 376 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.537 |
| ns | 2383 |  | 209 | format.go semantics: the formatter hook and the default output strings | 2.9 | 1.6 | 0.516 |
| ns | 2508 |  | 125 | README: customizing the message via `ErrorFormat` | 2.10 |  | 0.495 |
| walker |  | 2648 | 341 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.507 |
| ns | 2687 |  | 179 | README: getting at the individual errors | 2.11 |  | 0.488 |
| walker |  | 2879 | 231 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.630 |
| ns | 2990 |  | 303 | README: `errors.As` extraction and `errors.Is` sentinel checks | 2.12 |  | 0.593 |
| ns | 3125 |  | 135 | README: returning a multierror only if there are errors | 2.13 |  | 0.577 |
| walker |  | 3186 | 307 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.621 |
| ns | 3302 |  | 177 | `chain` doc comment: why the type exists and its precondition | 2.14 | 1.8 | 0.606 |
| walker |  | 3361 | 175 | Code::CodeKey { rung: Doc, file: multierror.go, decl: 6, sub: 0, line: 71 } |  |  | 0.637 |
| ns | 3485 |  | 183 | README migration guide: basic aggregation with `errors.Join` | 2.15 |  | 0.655 |
| ns | 3645 |  | 160 | README migration guide: replacing custom formatting | 2.16 |  | 0.667 |
| walker |  | 3660 | 299 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.722 |
| walker |  | 3748 | 88 | Code::CodeKey { rung: Doc, file: prefix.go, decl: 1, sub: 0, line: 16 } |  |  | 0.739 |
| walker |  | 3810 | 62 | Code::CodeKey { rung: Body, file: multierror.go, decl: 3, sub: 0, line: 31 } |  |  | 0.745 |
| walker |  | 3909 | 99 | Code::CodeKey { rung: Body, file: flatten.go, decl: 1, sub: 0, line: 8 } |  |  | 0.749 |
| ns | 3974 |  | 329 | README migration guide: replacing `Group` with a mutex collector | 2.17 |  | 0.760 |
| walker |  | 4033 | 124 | Code::CodeKey { rung: Doc, file: append.go, decl: 1, sub: 0, line: 14 } |  |  | 0.778 |
| walker |  | 4134 | 101 | Code::CodeKey { rung: Body, file: group.go, decl: 2, sub: 0, line: 20 } |  |  | 0.783 |
| ns | 4294 |  | 320 | `Append` body: the type switch, flattening, and nil filtering | 3.1 | 1.6 | 0.744 |
| ns | 4464 |  | 170 | Bodies of `Error`, `ErrorOrNil`, `GoString`, `WrappedErrors` | 3.2 | 1.7 | 0.745 |
| ns | 4621 |  | 157 | `Error.Unwrap` body: empty/single fast paths and the shallow copy into `chain` | 3.3 | 1.7 | 0.729 |
| walker |  | 4687 | 553 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.810 |
| ns | 4753 |  | 132 | `chain` method bodies: Error/Unwrap/As/Is | 3.4 | 1.8 | 0.793 |
| walker |  | 4829 | 142 | Code::CodeKey { rung: Body, file: format.go, decl: 2, sub: 0, line: 17 } |  |  | 0.819 |
| ns | 4939 |  | 186 | flatten.go bodies: exported `Flatten` and the recursive helper | 3.5 | 1.6 | 0.804 |
| walker |  | 5027 | 198 | Code::CodeKey { rung: Body, file: prefix.go, decl: 1, sub: 0, line: 16 } |  |  | 0.810 |
| ns | 5137 |  | 198 | `Prefix` body: in-place rewrite of each wrapped error | 3.6 | 1.6 | 0.813 |
| walker |  | 5181 | 154 | Code::CodeKey { rung: Body, file: multierror.go, decl: 6, sub: 0, line: 71 } |  |  | 0.833 |
| ns | 5311 |  | 174 | group.go bodies: the struct's unexported fields, `Go`, and `Wait` | 3.7 | 1.6 | 0.826 |
| ns | 5401 |  | 90 | sort.go bodies: Len/Swap/Less | 3.8 | 1.7 | 0.824 |
| ns | 5477 |  | 76 | Every import block in the package: stdlib only | 3.9 |  | 0.813 |
| walker |  | 5501 | 320 | Code::CodeKey { rung: Body, file: append.go, decl: 1, sub: 0, line: 14 } |  |  | 0.853 |
| ns | 5520 |  | 43 | Standard file header: copyright, SPDX tag, package clause | 3.10 |  | 0.850 |
| walker |  | 5637 | 136 | Code::CodeKey { rung: Names, file: multierror_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.850 |
| walker |  | 5648 | 11 | Code::CodeKey { rung: Body, file: multierror_test.go, decl: 1, sub: 0, line: 13 } |  |  | 0.850 |
| walker |  | 5760 | 112 | Code::CodeKey { rung: Names, file: append_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.851 |
| walker |  | 5777 | 17 | Code::CodeKey { rung: Names, file: group_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.851 |
| walker |  | 5811 | 34 | Code::CodeKey { rung: Names, file: flatten_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.851 |
| ns | 5825 |  | 305 | Complete roster of test functions across all seven `*_test.go` files | 4.1 |  | 0.841 |
| walker |  | 5865 | 54 | Code::CodeKey { rung: Names, file: prefix_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.846 |
| walker |  | 5899 | 34 | Code::CodeKey { rung: Names, file: sort_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.849 |
| walker |  | 5937 | 38 | Code::CodeKey { rung: Names, file: format_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.854 |
| walker |  | 5987 | 50 | Code::CodeKey { rung: Body, file: prefix_test.go, decl: 2, sub: 0, line: 22 } |  |  | 0.854 |
| ns | 6011 |  | 186 | Every `t.Run` subtest name in multierror_test.go | 4.2 | 4.1 | 0.846 |
| walker |  | 6042 | 55 | Code::CodeKey { rung: Body, file: flatten_test.go, decl: 2, sub: 0, line: 43 } |  |  | 0.846 |
| walker |  | 6101 | 59 | Code::CodeKey { rung: Body, file: append_test.go, decl: 2, sub: 0, line: 45 } |  |  | 0.846 |
| walker |  | 6164 | 63 | Code::CodeKey { rung: Body, file: append_test.go, decl: 5, sub: 0, line: 71 } |  |  | 0.846 |
| ns | 6246 |  | 235 | The golden formatted output, as asserted in tests | 4.3 | 4.1 | 0.822 |
| walker |  | 6251 | 87 | Code::CodeKey { rung: Body, file: prefix_test.go, decl: 3, sub: 0, line: 30 } |  |  | 0.822 |
| walker |  | 6356 | 105 | Code::CodeKey { rung: Body, file: format_test.go, decl: 1, sub: 0, line: 11 } |  |  | 0.822 |
| walker |  | 6422 | 66 | Code::CodeKey { rung: Body, file: append_test.go, decl: 4, sub: 0, line: 62 } |  |  | 0.822 |
| walker |  | 6488 | 66 | Code::CodeKey { rung: Body, file: append_test.go, decl: 6, sub: 0, line: 79 } |  |  | 0.822 |
| walker |  | 6623 | 135 | Code::CodeKey { rung: Body, file: sort_test.go, decl: 1, sub: 0, line: 13 } |  |  | 0.822 |
| ns | 6658 |  | 412 | group_test.go: the concurrency table and its assertion loop | 4.4 | 4.1 | 0.796 |
| walker |  | 6714 | 91 | Code::CodeKey { rung: Body, file: prefix_test.go, decl: 1, sub: 0, line: 11 } |  |  | 0.796 |
| walker |  | 6781 | 67 | Code::CodeKey { rung: Body, file: append_test.go, decl: 3, sub: 0, line: 53 } |  |  | 0.796 |
| ns | 6837 |  | 179 | Test-only helper `nestedError` and the `errors.As` target pattern | 4.5 | 4.1 | 0.782 |
| walker |  | 6904 | 123 | Code::CodeKey { rung: Body, file: format_test.go, decl: 2, sub: 0, line: 27 } |  |  | 0.782 |
| walker |  | 7031 | 127 | Code::CodeKey { rung: Body, file: multierror_test.go, decl: 2, sub: 0, line: 17 } |  |  | 0.782 |
| walker |  | 7160 | 129 | Code::CodeKey { rung: Body, file: multierror_test.go, decl: 3, sub: 0, line: 33 } |  |  | 0.790 |
| ns | 7254 |  | 417 | append_test.go: the nil / typed-nil / flattening cases in full | 4.6 | 4.1 | 0.764 |
| walker |  | 7351 | 191 | Code::CodeKey { rung: Body, file: sort_test.go, decl: 2, sub: 0, line: 32 } |  |  | 0.764 |
| ns | 7361 |  | 107 | Test file preambles: package clause and imports | 4.7 |  | 0.754 |
| ns | 7387 |  | 26 | Complete `.github/` tree listing | 5.1 |  | 0.755 |
| ns | 7461 |  | 74 | Makefile: every target plus the TEST variable | 5.2 |  | 0.757 |
| walker |  | 7589 | 238 | Code::CodeKey { rung: Body, file: flatten_test.go, decl: 1, sub: 0, line: 13 } |  |  | 0.773 |
| ns | 7710 |  | 249 | Makefile recipes for test, testrace, updatedeps and generate | 5.3 | 5.2 | 0.776 |
| walker |  | 7722 | 133 | Code::CodeKey { rung: Body, file: multierror_test.go, decl: 4, sub: 0, line: 51 } |  |  | 0.776 |
| ns | 8035 |  | 325 | Main CI workflow skeleton: triggers, permissions, all four jobs, the Go matrix | 5.4 |  | 0.755 |
| walker |  | 8134 | 412 | Code::CodeKey { rung: Body, file: group_test.go, decl: 1, sub: 0, line: 12 } |  |  | 0.784 |
| walker |  | 8282 | 148 | Code::CodeKey { rung: Body, file: multierror_test.go, decl: 5, sub: 0, line: 65 } |  |  | 0.784 |
| ns | 8467 |  | 432 | CI steps: the `go fmt` gate and the golangci-lint job | 5.5 | 5.4 | 0.768 |
| walker |  | 8626 | 344 | Code::CodeKey { rung: Body, file: append_test.go, decl: 1, sub: 0, line: 11 } |  |  | 0.797 |
| ns | 8852 |  | 385 | CI: how the linux test job actually runs the suite | 5.6 | 5.4 | 0.783 |
| walker |  | 8982 | 356 | Code::CodeKey { rung: Body, file: multierror_test.go, decl: 6, sub: 0, line: 82 } |  |  | 0.784 |
| ns | 9035 |  | 183 | CI: what the windows job does differently | 5.7 | 5.4 | 0.776 |
| ns | 9237 |  | 202 | actionlint workflow, in full | 5.8 |  | 0.766 |
| walker |  | 9352 | 370 | Code::CodeKey { rung: Body, file: multierror_test.go, decl: 7, sub: 0, line: 118 } |  |  | 0.768 |
| ns | 9388 |  | 151 | Dependabot configuration | 5.9 |  | 0.760 |
| ns | 9477 |  | 89 | CHANGELOG skeleton, CODEOWNERS, and the pinned Go toolchain file | 5.10 |  | 0.754 |
| ns | 9744 |  | 267 | README badge block and the older-Go compile-error hint | 5.11 | 1.1 | 0.749 |
| ns | 9803 |  | 59 | PR template section headings | 5.12 |  | 0.747 |
| ns | 9840 |  | 37 | LICENSE identification lines | 5.13 |  | 0.745 |
| walker |  | 9843 | 491 | Code::CodeKey { rung: Body, file: multierror_test.go, decl: 8, sub: 0, line: 157 } |  |  | 0.756 |
| walker |  | 9899 | 56 | Plaintext::Rest { file: CHANGELOG.md } |  |  | 0.759 |
| walker |  | 9984 | 85 | Plaintext::Rest { file: CODEOWNERS } |  |  | 0.763 |
