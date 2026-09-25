Score(3000)=0.552 I=0.564 C=0.540 ns_rows≤3K=21/56 grid(1000/1442/2080/3000/4327/6240/9000)=0.438/0.427/0.478/0.552/0.647/0.893/0.806

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 54 |  | 54 | README title + what the package is | 1.1 |  | 0.000 |
| walker |  | 80 | 80 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 83 |  | 29 | go.mod: module path and language version | 1.2 |  | 0.000 |
| walker |  | 109 | 29 | GoMod::Identity { file: go.mod } |  |  | 0.562 |
| walker |  | 109 | 0 | GoMod::File { file: go.mod } |  |  | 0.562 |
| walker |  | 123 | 14 | Fs::DirListing { dir: .github } |  |  | 0.563 |
| walker |  | 135 | 12 | Fs::DirListing { dir: .github/workflows } |  |  | 0.563 |
| ns | 163 |  | 80 | Complete root directory listing | 1.3 |  | 0.645 |
| ns | 240 |  | 77 | README: why returning a list-of-errors as an error works | 1.4 |  | 0.586 |
| walker |  | 242 | 107 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.605 |
| walker |  | 291 | 49 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.607 |
| ns | 315 |  | 75 | The `Error` type: doc comment and both fields | 1.5 |  | 0.551 |
| ns | 467 |  | 152 | Package-level API roster: every exported func/type outside multierror.go, with full signatures | 1.6 |  | 0.445 |
| ns | 606 |  | 139 | Every method on `Error`, across multierror.go and sort.go | 1.7 |  | 0.385 |
| walker |  | 679 | 388 | Plaintext::Whole { file: Makefile } |  |  | 0.388 |
| ns | 682 |  | 76 | The unexported `chain` type and its four methods | 1.8 |  | 0.362 |
| walker |  | 853 | 174 | Code::CodeKey { rung: Names, file: multierror.go, decl: 0, sub: 0, line: 0 } |  |  | 0.396 |
| ns | 853 |  | 171 | README section map: every heading and every bold subsection label | 1.9 |  | 0.396 |
| walker |  | 876 | 23 | Code::CodeKey { rung: Decl, file: multierror.go, decl: 1, sub: 0, line: 13 } |  |  | 0.406 |
| walker |  | 885 | 9 | Code::CodeKey { rung: Doc, file: multierror.go, decl: 8, sub: 0, line: 102 } |  |  | 0.406 |
| walker |  | 896 | 11 | Code::CodeKey { rung: Body, file: multierror.go, decl: 8, sub: 0, line: 102 } |  |  | 0.413 |
| walker |  | 909 | 13 | Code::CodeKey { rung: Body, file: multierror.go, decl: 10, sub: 0, line: 117 } |  |  | 0.419 |
| walker |  | 925 | 16 | Code::CodeKey { rung: Doc, file: multierror.go, decl: 11, sub: 0, line: 122 } |  |  | 0.419 |
| walker |  | 943 | 18 | Code::CodeKey { rung: Doc, file: multierror.go, decl: 10, sub: 0, line: 117 } |  |  | 0.419 |
| ns | 946 |  | 93 | README deprecation note: prefer stdlib `errors.Join` | 2.1 |  | 0.422 |
| walker |  | 977 | 34 | Code::CodeKey { rung: Doc, file: multierror.go, decl: 9, sub: 0, line: 108 } |  |  | 0.423 |
| walker |  | 1015 | 38 | Code::CodeKey { rung: Doc, file: multierror.go, decl: 1, sub: 0, line: 13 } |  |  | 0.459 |
| walker |  | 1028 | 13 | Code::CodeKey { rung: Body, file: multierror.go, decl: 11, sub: 0, line: 122 } |  |  | 0.466 |
| ns | 1070 |  | 124 | `Append` doc comment: nil handling and one-level flattening | 2.2 | 1.6 | 0.444 |
| walker |  | 1100 | 72 | Code::CodeKey { rung: Doc, file: multierror.go, decl: 3, sub: 0, line: 31 } |  |  | 0.445 |
| walker |  | 1115 | 15 | Code::CodeKey { rung: Body, file: multierror.go, decl: 4, sub: 0, line: 42 } |  |  | 0.449 |
| walker |  | 1233 | 118 | Code::CodeKey { rung: Doc, file: multierror.go, decl: 5, sub: 0, line: 53 } |  |  | 0.455 |
| ns | 1360 |  | 290 | README: stdlib compatibility, install, and the go 1.13 requirement | 2.3 |  | 0.419 |
| walker |  | 1408 | 175 | Code::CodeKey { rung: Doc, file: multierror.go, decl: 6, sub: 0, line: 71 } |  |  | 0.424 |
| walker |  | 1440 | 32 | Code::CodeKey { rung: Names, file: flatten.go, decl: 0, sub: 0, line: 0 } |  |  | 0.427 |
| walker |  | 1470 | 30 | Code::CodeKey { rung: Doc, file: flatten.go, decl: 1, sub: 0, line: 8 } |  |  | 0.427 |
| walker |  | 1518 | 48 | Code::CodeKey { rung: Names, file: group.go, decl: 0, sub: 0, line: 0 } |  |  | 0.439 |
| walker |  | 1551 | 33 | Code::CodeKey { rung: Decl, file: group.go, decl: 1, sub: 0, line: 10 } |  |  | 0.442 |
| walker |  | 1580 | 29 | Code::CodeKey { rung: Doc, file: group.go, decl: 1, sub: 0, line: 10 } |  |  | 0.406 |
| ns | 1580 |  | 220 | README: the canonical accumulate-with-Append recipe | 2.4 |  | 0.406 |
| walker |  | 1610 | 30 | Code::CodeKey { rung: Doc, file: group.go, decl: 3, sub: 0, line: 36 } |  |  | 0.407 |
| walker |  | 1646 | 36 | Code::CodeKey { rung: Body, file: group.go, decl: 3, sub: 0, line: 36 } |  |  | 0.410 |
| walker |  | 1696 | 50 | Code::CodeKey { rung: Doc, file: group.go, decl: 2, sub: 0, line: 20 } |  |  | 0.413 |
| ns | 1770 |  | 190 | `ErrorOrNil` and `WrappedErrors` doc comments | 2.5 | 1.7 | 0.437 |
| walker |  | 1873 | 177 | Code::CodeKey { rung: Doc, file: multierror.go, decl: 7, sub: 0, line: 99 } |  |  | 0.440 |
| walker |  | 1907 | 34 | Code::CodeKey { rung: Names, file: format.go, decl: 0, sub: 0, line: 0 } |  |  | 0.456 |
| walker |  | 1940 | 33 | Code::CodeKey { rung: Doc, file: format.go, decl: 1, sub: 0, line: 13 } |  |  | 0.457 |
| ns | 1945 |  | 175 | `Error.Unwrap` doc comment: ordering, shallow copy, errors.As/Is support | 2.6 | 1.7 | 0.475 |
| walker |  | 1974 | 34 | Code::CodeKey { rung: Doc, file: format.go, decl: 2, sub: 0, line: 17 } |  |  | 0.475 |
| walker |  | 2055 | 81 | Code::CodeKey { rung: Body, file: flatten.go, decl: 2, sub: 0, line: 20 } |  |  | 0.480 |
| ns | 2063 |  | 118 | `Flatten` and `Prefix` doc comments | 2.7 | 1.6 | 0.468 |
| walker |  | 2073 | 18 | Code::CodeKey { rung: Names, file: prefix.go, decl: 0, sub: 0, line: 0 } |  |  | 0.478 |
| walker |  | 2128 | 55 | Code::CodeKey { rung: Names, file: sort.go, decl: 0, sub: 0, line: 0 } |  |  | 0.500 |
| walker |  | 2141 | 13 | Code::CodeKey { rung: Doc, file: sort.go, decl: 1, sub: 0, line: 7 } |  |  | 0.500 |
| walker |  | 2155 | 14 | Code::CodeKey { rung: Doc, file: sort.go, decl: 2, sub: 0, line: 16 } |  |  | 0.500 |
| walker |  | 2169 | 14 | Code::CodeKey { rung: Doc, file: sort.go, decl: 3, sub: 0, line: 21 } |  |  | 0.500 |
| ns | 2174 |  | 111 | `Group` doc comments: Go and Wait | 2.8 | 1.6 | 0.509 |
| walker |  | 2187 | 18 | Code::CodeKey { rung: Body, file: sort.go, decl: 3, sub: 0, line: 21 } |  |  | 0.514 |
| walker |  | 2208 | 21 | Code::CodeKey { rung: Body, file: sort.go, decl: 2, sub: 0, line: 16 } |  |  | 0.518 |
| walker |  | 2246 | 38 | Code::CodeKey { rung: Body, file: sort.go, decl: 1, sub: 0, line: 7 } |  |  | 0.523 |
| walker |  | 2334 | 88 | Code::CodeKey { rung: Doc, file: prefix.go, decl: 1, sub: 0, line: 16 } |  |  | 0.548 |
| walker |  | 2354 | 20 | Code::CodeKey { rung: Names, file: append.go, decl: 0, sub: 0, line: 0 } |  |  | 0.559 |
| ns | 2383 |  | 209 | format.go semantics: the formatter hook and the default output strings | 2.9 | 1.6 | 0.536 |
| walker |  | 2453 | 99 | Code::CodeKey { rung: Body, file: flatten.go, decl: 1, sub: 0, line: 8 } |  |  | 0.544 |
| ns | 2508 |  | 125 | README: customizing the message via `ErrorFormat` | 2.10 |  | 0.522 |
| walker |  | 2577 | 124 | Code::CodeKey { rung: Doc, file: append.go, decl: 1, sub: 0, line: 14 } |  |  | 0.548 |
| ns | 2687 |  | 179 | README: getting at the individual errors | 2.11 |  | 0.527 |
| walker |  | 2719 | 142 | Code::CodeKey { rung: Body, file: format.go, decl: 2, sub: 0, line: 17 } |  |  | 0.570 |
| walker |  | 2917 | 198 | Code::CodeKey { rung: Body, file: prefix.go, decl: 1, sub: 0, line: 16 } |  |  | 0.577 |
| walker |  | 2948 | 31 | Code::CodeKey { rung: Body, file: multierror.go, decl: 5, sub: 0, line: 53 } |  |  | 0.581 |
| ns | 2990 |  | 303 | README: `errors.As` extraction and `errors.Is` sentinel checks | 2.12 |  | 0.547 |
| walker |  | 3049 | 101 | Code::CodeKey { rung: Body, file: group.go, decl: 2, sub: 0, line: 20 } |  |  | 0.554 |
| ns | 3125 |  | 135 | README: returning a multierror only if there are errors | 2.13 |  | 0.539 |
| ns | 3302 |  | 177 | `chain` doc comment: why the type exists and its precondition | 2.14 | 1.8 | 0.547 |
| walker |  | 3369 | 320 | Code::CodeKey { rung: Body, file: append.go, decl: 1, sub: 0, line: 14 } |  |  | 0.558 |
| walker |  | 3409 | 40 | Code::CodeKey { rung: Body, file: multierror.go, decl: 9, sub: 0, line: 108 } |  |  | 0.562 |
| walker |  | 3459 | 50 | Code::CodeKey { rung: Body, file: multierror.go, decl: 2, sub: 0, line: 18 } |  |  | 0.566 |
| ns | 3485 |  | 183 | README migration guide: basic aggregation with `errors.Join` | 2.15 |  | 0.543 |
| ns | 3645 |  | 160 | README migration guide: replacing custom formatting | 2.16 |  | 0.527 |
| walker |  | 3709 | 250 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.572 |
| ns | 3974 |  | 329 | README migration guide: replacing `Group` with a mutex collector | 2.17 |  | 0.540 |
| walker |  | 4085 | 376 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.609 |
| ns | 4294 |  | 320 | `Append` body: the type switch, flattening, and nil filtering | 3.1 | 1.6 | 0.621 |
| walker |  | 4426 | 341 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.673 |
| ns | 4464 |  | 170 | Bodies of `Error`, `ErrorOrNil`, `GoString`, `WrappedErrors` | 3.2 | 1.7 | 0.659 |
| ns | 4621 |  | 157 | `Error.Unwrap` body: empty/single fast paths and the shallow copy into `chain` | 3.3 | 1.7 | 0.645 |
| walker |  | 4665 | 239 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.778 |
| ns | 4753 |  | 132 | `chain` method bodies: Error/Unwrap/As/Is | 3.4 | 1.8 | 0.775 |
| ns | 4939 |  | 186 | flatten.go bodies: exported `Flatten` and the recursive helper | 3.5 | 1.6 | 0.777 |
| walker |  | 4972 | 307 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.803 |
| ns | 5137 |  | 198 | `Prefix` body: in-place rewrite of each wrapped error | 3.6 | 1.6 | 0.806 |
| walker |  | 5271 | 299 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.844 |
| ns | 5311 |  | 174 | group.go bodies: the struct's unexported fields, `Go`, and `Wait` | 3.7 | 1.6 | 0.843 |
| walker |  | 5333 | 62 | Code::CodeKey { rung: Body, file: multierror.go, decl: 3, sub: 0, line: 31 } |  |  | 0.860 |
| ns | 5401 |  | 90 | sort.go bodies: Len/Swap/Less | 3.8 | 1.7 | 0.857 |
| ns | 5477 |  | 76 | Every import block in the package: stdlib only | 3.9 |  | 0.845 |
| ns | 5520 |  | 43 | Standard file header: copyright, SPDX tag, package clause | 3.10 |  | 0.841 |
| ns | 5825 |  | 305 | Complete roster of test functions across all seven `*_test.go` files | 4.1 |  | 0.819 |
| walker |  | 5886 | 553 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.883 |
| walker |  | 5942 | 56 | Markdown::HeadingsOutline { file: CHANGELOG.md } |  |  | 0.883 |
| ns | 6011 |  | 186 | Every `t.Run` subtest name in multierror_test.go | 4.2 | 4.1 | 0.875 |
| walker |  | 6096 | 154 | Code::CodeKey { rung: Body, file: multierror.go, decl: 6, sub: 0, line: 71 } |  |  | 0.892 |
| walker |  | 6130 | 34 | Markdown::HeadingsOutline { file: .github/pull_request_template.md } |  |  | 0.893 |
| walker |  | 6154 | 24 | Markdown::Section { file: .github/pull_request_template.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.893 |
| ns | 6246 |  | 235 | The golden formatted output, as asserted in tests | 4.3 | 4.1 | 0.868 |
| walker |  | 6320 | 166 | Code::CodeKey { rung: Names, file: multierror_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.871 |
| walker |  | 6331 | 11 | Code::CodeKey { rung: Body, file: multierror_test.go, decl: 1, sub: 0, line: 13 } |  |  | 0.871 |
| walker |  | 6346 | 15 | Code::CodeKey { rung: Doc, file: multierror_test.go, decl: 9, sub: 0, line: 209 } |  |  | 0.871 |
| walker |  | 6380 | 34 | Code::CodeKey { rung: Names, file: flatten_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.872 |
| walker |  | 6435 | 55 | Code::CodeKey { rung: Body, file: flatten_test.go, decl: 2, sub: 0, line: 43 } |  |  | 0.872 |
| walker |  | 6452 | 17 | Code::CodeKey { rung: Names, file: group_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.873 |
| walker |  | 6486 | 34 | Code::CodeKey { rung: Names, file: sort_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.875 |
| walker |  | 6540 | 54 | Code::CodeKey { rung: Names, file: prefix_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.878 |
| walker |  | 6590 | 50 | Code::CodeKey { rung: Body, file: prefix_test.go, decl: 2, sub: 0, line: 22 } |  |  | 0.878 |
| ns | 6658 |  | 412 | group_test.go: the concurrency table and its assertion loop | 4.4 | 4.1 | 0.850 |
| walker |  | 6677 | 87 | Code::CodeKey { rung: Body, file: prefix_test.go, decl: 3, sub: 0, line: 30 } |  |  | 0.850 |
| walker |  | 6789 | 112 | Code::CodeKey { rung: Names, file: append_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.860 |
| ns | 6837 |  | 179 | Test-only helper `nestedError` and the `errors.As` target pattern | 4.5 | 4.1 | 0.846 |
| walker |  | 6848 | 59 | Code::CodeKey { rung: Body, file: append_test.go, decl: 2, sub: 0, line: 45 } |  |  | 0.846 |
| walker |  | 6911 | 63 | Code::CodeKey { rung: Body, file: append_test.go, decl: 5, sub: 0, line: 71 } |  |  | 0.846 |
| walker |  | 6977 | 66 | Code::CodeKey { rung: Body, file: append_test.go, decl: 4, sub: 0, line: 62 } |  |  | 0.846 |
| walker |  | 7043 | 66 | Code::CodeKey { rung: Body, file: append_test.go, decl: 6, sub: 0, line: 79 } |  |  | 0.846 |
| walker |  | 7081 | 38 | Code::CodeKey { rung: Names, file: format_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.851 |
| walker |  | 7172 | 91 | Code::CodeKey { rung: Body, file: prefix_test.go, decl: 1, sub: 0, line: 11 } |  |  | 0.851 |
| walker |  | 7239 | 67 | Code::CodeKey { rung: Body, file: append_test.go, decl: 3, sub: 0, line: 53 } |  |  | 0.851 |
| ns | 7254 |  | 417 | append_test.go: the nil / typed-nil / flattening cases in full | 4.6 | 4.1 | 0.822 |
| walker |  | 7344 | 105 | Code::CodeKey { rung: Body, file: format_test.go, decl: 1, sub: 0, line: 11 } |  |  | 0.822 |
| ns | 7361 |  | 107 | Test file preambles: package clause and imports | 4.7 |  | 0.811 |
| ns | 7387 |  | 26 | Complete `.github/` tree listing | 5.1 |  | 0.812 |
| ns | 7461 |  | 74 | Makefile: every target plus the TEST variable | 5.2 |  | 0.813 |
| walker |  | 7467 | 123 | Code::CodeKey { rung: Body, file: format_test.go, decl: 2, sub: 0, line: 27 } |  |  | 0.813 |
| walker |  | 7594 | 127 | Code::CodeKey { rung: Body, file: multierror_test.go, decl: 2, sub: 0, line: 17 } |  |  | 0.813 |
| ns | 7710 |  | 249 | Makefile recipes for test, testrace, updatedeps and generate | 5.3 | 5.2 | 0.815 |
| walker |  | 7723 | 129 | Code::CodeKey { rung: Body, file: multierror_test.go, decl: 3, sub: 0, line: 33 } |  |  | 0.822 |
| walker |  | 7858 | 135 | Code::CodeKey { rung: Body, file: sort_test.go, decl: 1, sub: 0, line: 13 } |  |  | 0.822 |
| walker |  | 7881 | 23 | Markdown::Section { file: .github/pull_request_template.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.822 |
| ns | 8035 |  | 325 | Main CI workflow skeleton: triggers, permissions, all four jobs, the Go matrix | 5.4 |  | 0.800 |
| walker |  | 8072 | 191 | Code::CodeKey { rung: Body, file: sort_test.go, decl: 2, sub: 0, line: 32 } |  |  | 0.800 |
| walker |  | 8205 | 133 | Code::CodeKey { rung: Body, file: multierror_test.go, decl: 4, sub: 0, line: 51 } |  |  | 0.800 |
| walker |  | 8443 | 238 | Code::CodeKey { rung: Body, file: flatten_test.go, decl: 1, sub: 0, line: 13 } |  |  | 0.815 |
| ns | 8467 |  | 432 | CI steps: the `go fmt` gate and the golangci-lint job | 5.5 | 5.4 | 0.799 |
| walker |  | 8480 | 37 | Markdown::Section { file: .github/pull_request_template.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.799 |
| walker |  | 8628 | 148 | Code::CodeKey { rung: Body, file: multierror_test.go, decl: 5, sub: 0, line: 65 } |  |  | 0.799 |
| ns | 8852 |  | 385 | CI: how the linux test job actually runs the suite | 5.6 | 5.4 | 0.785 |
| ns | 9035 |  | 183 | CI: what the windows job does differently | 5.7 | 5.4 | 0.777 |
| walker |  | 9040 | 412 | Code::CodeKey { rung: Body, file: group_test.go, decl: 1, sub: 0, line: 12 } |  |  | 0.804 |
| ns | 9237 |  | 202 | actionlint workflow, in full | 5.8 |  | 0.794 |
| walker |  | 9384 | 344 | Code::CodeKey { rung: Body, file: append_test.go, decl: 1, sub: 0, line: 11 } |  |  | 0.821 |
| ns | 9388 |  | 151 | Dependabot configuration | 5.9 |  | 0.812 |
| ns | 9477 |  | 89 | CHANGELOG skeleton, CODEOWNERS, and the pinned Go toolchain file | 5.10 |  | 0.809 |
| walker |  | 9740 | 356 | Code::CodeKey { rung: Body, file: multierror_test.go, decl: 6, sub: 0, line: 82 } |  |  | 0.810 |
| ns | 9744 |  | 267 | README badge block and the older-Go compile-error hint | 5.11 | 1.1 | 0.804 |
| ns | 9803 |  | 59 | PR template section headings | 5.12 |  | 0.803 |
| ns | 9840 |  | 37 | LICENSE identification lines | 5.13 |  | 0.802 |
| walker |  | 9995 | 255 | Code::CodeKey { rung: Body, file: multierror_test.go, decl: 7, sub: 0, line: 118 } |  |  | 0.803 |
