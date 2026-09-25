Score(3000)=0.547 I=0.559 C=0.536 ns_rows≤3K=21/56 grid(1000/1442/2080/3000/4327/6240/9000)=0.423/0.424/0.468/0.547/0.601/0.876/0.804

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
| walker |  | 1218 | 118 | Code::CodeKey { rung: Doc, file: multierror.go, decl: 5, sub: 0, line: 53 } |  |  | 0.451 |
| walker |  | 1233 | 15 | Code::CodeKey { rung: Body, file: multierror.go, decl: 4, sub: 0, line: 42 } |  |  | 0.455 |
| ns | 1360 |  | 290 | README: stdlib compatibility, install, and the go 1.13 requirement | 2.3 |  | 0.419 |
| walker |  | 1408 | 175 | Code::CodeKey { rung: Doc, file: multierror.go, decl: 6, sub: 0, line: 71 } |  |  | 0.424 |
| ns | 1580 |  | 220 | README: the canonical accumulate-with-Append recipe | 2.4 |  | 0.389 |
| walker |  | 1585 | 177 | Code::CodeKey { rung: Doc, file: multierror.go, decl: 7, sub: 0, line: 99 } |  |  | 0.392 |
| walker |  | 1617 | 32 | Code::CodeKey { rung: Names, file: flatten.go, decl: 0, sub: 0, line: 0 } |  |  | 0.395 |
| walker |  | 1647 | 30 | Code::CodeKey { rung: Doc, file: flatten.go, decl: 1, sub: 0, line: 8 } |  |  | 0.395 |
| walker |  | 1728 | 81 | Code::CodeKey { rung: Body, file: flatten.go, decl: 2, sub: 0, line: 20 } |  |  | 0.397 |
| ns | 1770 |  | 190 | `ErrorOrNil` and `WrappedErrors` doc comments | 2.5 | 1.7 | 0.422 |
| walker |  | 1776 | 48 | Code::CodeKey { rung: Names, file: group.go, decl: 0, sub: 0, line: 0 } |  |  | 0.434 |
| walker |  | 1809 | 33 | Code::CodeKey { rung: Decl, file: group.go, decl: 1, sub: 0, line: 10 } |  |  | 0.437 |
| walker |  | 1838 | 29 | Code::CodeKey { rung: Doc, file: group.go, decl: 1, sub: 0, line: 10 } |  |  | 0.437 |
| walker |  | 1868 | 30 | Code::CodeKey { rung: Doc, file: group.go, decl: 3, sub: 0, line: 36 } |  |  | 0.438 |
| walker |  | 1904 | 36 | Code::CodeKey { rung: Body, file: group.go, decl: 3, sub: 0, line: 36 } |  |  | 0.441 |
| ns | 1945 |  | 175 | `Error.Unwrap` doc comment: ordering, shallow copy, errors.As/Is support | 2.6 | 1.7 | 0.460 |
| walker |  | 1954 | 50 | Code::CodeKey { rung: Doc, file: group.go, decl: 2, sub: 0, line: 20 } |  |  | 0.462 |
| walker |  | 1988 | 34 | Code::CodeKey { rung: Names, file: format.go, decl: 0, sub: 0, line: 0 } |  |  | 0.479 |
| walker |  | 2021 | 33 | Code::CodeKey { rung: Doc, file: format.go, decl: 1, sub: 0, line: 13 } |  |  | 0.479 |
| walker |  | 2055 | 34 | Code::CodeKey { rung: Doc, file: format.go, decl: 2, sub: 0, line: 17 } |  |  | 0.480 |
| ns | 2063 |  | 118 | `Flatten` and `Prefix` doc comments | 2.7 | 1.6 | 0.468 |
| walker |  | 2154 | 99 | Code::CodeKey { rung: Body, file: flatten.go, decl: 1, sub: 0, line: 8 } |  |  | 0.474 |
| walker |  | 2172 | 18 | Code::CodeKey { rung: Names, file: prefix.go, decl: 0, sub: 0, line: 0 } |  |  | 0.485 |
| ns | 2174 |  | 111 | `Group` doc comments: Go and Wait | 2.8 | 1.6 | 0.496 |
| walker |  | 2260 | 88 | Code::CodeKey { rung: Doc, file: prefix.go, decl: 1, sub: 0, line: 16 } |  |  | 0.521 |
| walker |  | 2315 | 55 | Code::CodeKey { rung: Names, file: sort.go, decl: 0, sub: 0, line: 0 } |  |  | 0.541 |
| walker |  | 2328 | 13 | Code::CodeKey { rung: Doc, file: sort.go, decl: 1, sub: 0, line: 7 } |  |  | 0.541 |
| walker |  | 2342 | 14 | Code::CodeKey { rung: Doc, file: sort.go, decl: 2, sub: 0, line: 16 } |  |  | 0.541 |
| walker |  | 2356 | 14 | Code::CodeKey { rung: Doc, file: sort.go, decl: 3, sub: 0, line: 21 } |  |  | 0.541 |
| walker |  | 2374 | 18 | Code::CodeKey { rung: Body, file: sort.go, decl: 3, sub: 0, line: 21 } |  |  | 0.545 |
| ns | 2383 |  | 209 | format.go semantics: the formatter hook and the default output strings | 2.9 | 1.6 | 0.523 |
| walker |  | 2395 | 21 | Code::CodeKey { rung: Body, file: sort.go, decl: 2, sub: 0, line: 16 } |  |  | 0.527 |
| walker |  | 2433 | 38 | Code::CodeKey { rung: Body, file: sort.go, decl: 1, sub: 0, line: 7 } |  |  | 0.532 |
| walker |  | 2453 | 20 | Code::CodeKey { rung: Names, file: append.go, decl: 0, sub: 0, line: 0 } |  |  | 0.544 |
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
| walker |  | 4008 | 299 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: true } |  |  | 0.586 |
| ns | 4294 |  | 320 | `Append` body: the type switch, flattening, and nil filtering | 3.1 | 1.6 | 0.601 |
| walker |  | 4384 | 376 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.663 |
| ns | 4464 |  | 170 | Bodies of `Error`, `ErrorOrNil`, `GoString`, `WrappedErrors` | 3.2 | 1.7 | 0.649 |
| ns | 4621 |  | 157 | `Error.Unwrap` body: empty/single fast paths and the shallow copy into `chain` | 3.3 | 1.7 | 0.635 |
| walker |  | 4725 | 341 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: true } |  |  | 0.684 |
| ns | 4753 |  | 132 | `chain` method bodies: Error/Unwrap/As/Is | 3.4 | 1.8 | 0.681 |
| ns | 4939 |  | 186 | flatten.go bodies: exported `Flatten` and the recursive helper | 3.5 | 1.6 | 0.681 |
| walker |  | 4964 | 239 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.816 |
| ns | 5137 |  | 198 | `Prefix` body: in-place rewrite of each wrapped error | 3.6 | 1.6 | 0.819 |
| walker |  | 5271 | 307 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.844 |
| ns | 5311 |  | 174 | group.go bodies: the struct's unexported fields, `Go`, and `Wait` | 3.7 | 1.6 | 0.843 |
| walker |  | 5333 | 62 | Code::CodeKey { rung: Body, file: multierror.go, decl: 3, sub: 0, line: 31 } |  |  | 0.860 |
| ns | 5401 |  | 90 | sort.go bodies: Len/Swap/Less | 3.8 | 1.7 | 0.857 |
| ns | 5477 |  | 76 | Every import block in the package: stdlib only | 3.9 |  | 0.845 |
| ns | 5520 |  | 43 | Standard file header: copyright, SPDX tag, package clause | 3.10 |  | 0.841 |
| ns | 5825 |  | 305 | Complete roster of test functions across all seven `*_test.go` files | 4.1 |  | 0.819 |
| walker |  | 5886 | 553 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.883 |
| walker |  | 5942 | 56 | Markdown::HeadingsOutline { file: CHANGELOG.md } |  |  | 0.883 |
| ns | 6011 |  | 186 | Every `t.Run` subtest name in multierror_test.go | 4.2 | 4.1 | 0.875 |
| walker |  | 6144 | 202 | Plaintext::Whole { file: .github/workflows/actionlint.yml } |  |  | 0.876 |
| ns | 6246 |  | 235 | The golden formatted output, as asserted in tests | 4.3 | 4.1 | 0.852 |
| walker |  | 6298 | 154 | Code::CodeKey { rung: Body, file: multierror.go, decl: 6, sub: 0, line: 71 } |  |  | 0.869 |
| walker |  | 6332 | 34 | Markdown::HeadingsOutline { file: .github/pull_request_template.md } |  |  | 0.870 |
| walker |  | 6356 | 24 | Markdown::Section { file: .github/pull_request_template.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.870 |
| ns | 6658 |  | 412 | group_test.go: the concurrency table and its assertion loop | 4.4 | 4.1 | 0.841 |
| ns | 6837 |  | 179 | Test-only helper `nestedError` and the `errors.As` target pattern | 4.5 | 4.1 | 0.827 |
| walker |  | 7213 | 857 | Plaintext::Whole { file: .github/workflows/go-multierror.yml } |  |  | 0.831 |
| ns | 7254 |  | 417 | append_test.go: the nil / typed-nil / flattening cases in full | 4.6 | 4.1 | 0.802 |
| ns | 7361 |  | 107 | Test file preambles: package clause and imports | 4.7 |  | 0.791 |
| walker |  | 7379 | 166 | Code::CodeKey { rung: Names, file: multierror_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.794 |
| ns | 7387 |  | 26 | Complete `.github/` tree listing | 5.1 |  | 0.795 |
| walker |  | 7390 | 11 | Code::CodeKey { rung: Body, file: multierror_test.go, decl: 1, sub: 0, line: 13 } |  |  | 0.795 |
| walker |  | 7405 | 15 | Code::CodeKey { rung: Doc, file: multierror_test.go, decl: 9, sub: 0, line: 209 } |  |  | 0.796 |
| walker |  | 7439 | 34 | Code::CodeKey { rung: Names, file: flatten_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.797 |
| ns | 7461 |  | 74 | Makefile: every target plus the TEST variable | 5.2 |  | 0.799 |
| walker |  | 7494 | 55 | Code::CodeKey { rung: Body, file: flatten_test.go, decl: 2, sub: 0, line: 43 } |  |  | 0.799 |
| walker |  | 7511 | 17 | Code::CodeKey { rung: Names, file: group_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.799 |
| walker |  | 7545 | 34 | Code::CodeKey { rung: Names, file: sort_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.801 |
| walker |  | 7599 | 54 | Code::CodeKey { rung: Names, file: prefix_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.804 |
| walker |  | 7649 | 50 | Code::CodeKey { rung: Body, file: prefix_test.go, decl: 2, sub: 0, line: 22 } |  |  | 0.804 |
| ns | 7710 |  | 249 | Makefile recipes for test, testrace, updatedeps and generate | 5.3 | 5.2 | 0.807 |
| walker |  | 7736 | 87 | Code::CodeKey { rung: Body, file: prefix_test.go, decl: 3, sub: 0, line: 30 } |  |  | 0.807 |
| walker |  | 7827 | 91 | Code::CodeKey { rung: Body, file: prefix_test.go, decl: 1, sub: 0, line: 11 } |  |  | 0.807 |
| walker |  | 7939 | 112 | Code::CodeKey { rung: Names, file: append_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.815 |
| walker |  | 7998 | 59 | Code::CodeKey { rung: Body, file: append_test.go, decl: 2, sub: 0, line: 45 } |  |  | 0.815 |
| ns | 8035 |  | 325 | Main CI workflow skeleton: triggers, permissions, all four jobs, the Go matrix | 5.4 |  | 0.804 |
| walker |  | 8061 | 63 | Code::CodeKey { rung: Body, file: append_test.go, decl: 5, sub: 0, line: 71 } |  |  | 0.804 |
| walker |  | 8127 | 66 | Code::CodeKey { rung: Body, file: append_test.go, decl: 4, sub: 0, line: 62 } |  |  | 0.804 |
| walker |  | 8193 | 66 | Code::CodeKey { rung: Body, file: append_test.go, decl: 6, sub: 0, line: 79 } |  |  | 0.804 |
| walker |  | 8260 | 67 | Code::CodeKey { rung: Body, file: append_test.go, decl: 3, sub: 0, line: 53 } |  |  | 0.805 |
| walker |  | 8298 | 38 | Code::CodeKey { rung: Names, file: format_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.809 |
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
