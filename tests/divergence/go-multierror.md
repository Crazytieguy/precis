Score(3000)=0.649 I=0.818 C=0.515 ns_rows≤3K=17/54 (reached=8 partial=1 missing=8)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 78 | 78 | listing of '.' |  |  | 1.000 |
| ns | 78 |  | 78 | Root directory listing | 1.1 |  | 1.000 |
| ns | 104 |  | 26 | .github/ and .github/workflows/ listings | 1.2 |  | 0.884 |
| walker |  | 107 | 29 | go module identity in go.mod |  |  | 0.894 |
| walker |  | 107 | 0 | go module file go.mod |  |  | 0.894 |
| walker |  | 125 | 18 | go decl names surface in prefix.go |  |  | 0.894 |
| walker |  | 125 | 0 | go decl at prefix.go:16 |  |  | 0.894 |
| ns | 141 |  | 37 | go.mod + .go-version | 1.3 |  | 0.871 |
| walker |  | 145 | 20 | go decl names surface in append.go |  |  | 0.872 |
| walker |  | 145 | 0 | go decl at append.go:14 |  |  | 0.872 |
| ns | 228 |  | 87 | Public API locations: multierror.go | 1.4 |  | 0.784 |
| walker |  | 303 | 158 | README headline in README.md |  |  | 0.788 |
| walker |  | 352 | 49 | headings outline in README.md |  |  | 0.788 |
| ns | 372 |  | 144 | Public API locations: append/flatten/format/group/prefix/sort | 1.5 |  | 0.677 |
| walker |  | 384 | 32 | go decl names surface in flatten.go |  |  | 0.681 |
| walker |  | 384 | 0 | go decl at flatten.go:8 |  |  | 0.681 |
| walker |  | 384 | 0 | go decl at flatten.go:20 |  |  | 0.681 |
| walker |  | 418 | 34 | go decl names surface in format.go |  |  | 0.707 |
| walker |  | 418 | 0 | go decl at format.go:13 |  |  | 0.707 |
| walker |  | 418 | 0 | go decl at format.go:17 |  |  | 0.707 |
| walker |  | 428 | 10 | go package + imports in append.go |  |  | 0.707 |
| walker |  | 438 | 10 | go package + imports in flatten.go |  |  | 0.707 |
| walker |  | 450 | 12 | go package + imports in sort.go |  |  | 0.707 |
| ns | 468 |  | 96 | Test-function locations: multierror_test.go | 1.6 |  | 0.645 |
| walker |  | 498 | 48 | go decl names surface in group.go |  |  | 0.690 |
| walker |  | 498 | 0 | go decl at group.go:20 |  |  | 0.690 |
| walker |  | 498 | 0 | go decl at group.go:36 |  |  | 0.690 |
| walker |  | 531 | 33 | go decl at group.go:10 |  |  | 0.691 |
| walker |  | 584 | 53 | go decl names surface in sort.go |  |  | 0.758 |
| walker |  | 584 | 0 | go decl at sort.go:7 |  |  | 0.758 |
| walker |  | 584 | 0 | go decl at sort.go:16 |  |  | 0.758 |
| walker |  | 584 | 0 | go decl at sort.go:21 |  |  | 0.758 |
| walker |  | 595 | 11 | go decl doc at sort.go:7 |  |  | 0.758 |
| walker |  | 609 | 14 | go decl doc at sort.go:16 |  |  | 0.759 |
| walker |  | 623 | 14 | go decl doc at sort.go:21 |  |  | 0.759 |
| walker |  | 665 | 42 | go package + imports in multierror.go |  |  | 0.759 |
| ns | 677 |  | 209 | Test-function locations: the other six _test.go files | 1.7 |  | 0.653 |
| walker |  | 837 | 172 | go decl names surface in multierror.go |  |  | 0.720 |
| walker |  | 837 | 0 | go decl at multierror.go:18 |  |  | 0.720 |
| walker |  | 837 | 0 | go decl at multierror.go:31 |  |  | 0.720 |
| walker |  | 837 | 0 | go decl at multierror.go:42 |  |  | 0.720 |
| walker |  | 837 | 0 | go decl at multierror.go:53 |  |  | 0.720 |
| walker |  | 837 | 0 | go decl at multierror.go:71 |  |  | 0.720 |
| walker |  | 837 | 0 | go decl at multierror.go:99 |  |  | 0.720 |
| walker |  | 837 | 0 | go decl at multierror.go:102 |  |  | 0.720 |
| walker |  | 837 | 0 | go decl at multierror.go:108 |  |  | 0.720 |
| walker |  | 837 | 0 | go decl at multierror.go:117 |  |  | 0.720 |
| walker |  | 837 | 0 | go decl at multierror.go:122 |  |  | 0.720 |
| walker |  | 860 | 23 | go decl at multierror.go:13 |  |  | 0.721 |
| walker |  | 875 | 15 | go decl body at multierror.go:42 |  |  | 0.721 |
| walker |  | 884 | 9 | go decl doc at multierror.go:102 |  |  | 0.721 |
| walker |  | 920 | 36 | go decl doc at multierror.go:13 |  |  | 0.724 |
| walker |  | 931 | 11 | go decl body at multierror.go:102 |  |  | 0.724 |
| walker |  | 947 | 16 | go decl doc at multierror.go:122 |  |  | 0.725 |
| ns | 962 |  | 285 | README lede + stdlib-errors.Join deprecation note | 1.8 |  | 0.689 |
| walker |  | 965 | 18 | go decl doc at multierror.go:117 |  |  | 0.690 |
| walker |  | 978 | 13 | go decl body at multierror.go:117 |  |  | 0.690 |
| walker |  | 991 | 13 | go decl body at multierror.go:122 |  |  | 0.690 |
| walker |  | 1022 | 31 | go decl body at multierror.go:53 |  |  | 0.690 |
| walker |  | 1051 | 29 | go decl doc at group.go:10 |  |  | 0.691 |
| walker |  | 1069 | 18 | go decl body at sort.go:21 |  |  | 0.691 |
| walker |  | 1102 | 33 | go decl doc at format.go:13 |  |  | 0.692 |
| walker |  | 1130 | 28 | go decl doc at flatten.go:8 |  |  | 0.692 |
| walker |  | 1151 | 21 | go package + imports in group.go |  |  | 0.692 |
| ns | 1195 |  | 233 | README: package description + stdlib errors compat | 1.9 |  | 0.635 |
| walker |  | 1223 | 72 | go decl doc at multierror.go:31 |  |  | 0.637 |
| walker |  | 1244 | 21 | go decl body at sort.go:16 |  |  | 0.637 |
| walker |  | 1274 | 30 | go decl doc at group.go:36 |  |  | 0.637 |
| walker |  | 1308 | 34 | go decl doc at multierror.go:108 |  |  | 0.638 |
| walker |  | 1342 | 34 | go decl doc at format.go:17 |  |  | 0.638 |
| walker |  | 1392 | 50 | go decl body at multierror.go:18 |  |  | 0.642 |
| walker |  | 1425 | 33 | go package + imports in prefix.go |  |  | 0.642 |
| walker |  | 1487 | 62 | go decl body at multierror.go:31 |  |  | 0.646 |
| ns | 1530 |  | 335 | README: install, go-version floor, compile-error symptom | 1.10 |  | 0.568 |
| ns | 1591 |  | 61 | Error struct fields | 2.1 | 1.4 | 0.587 |
| walker |  | 1605 | 118 | go decl doc at multierror.go:53 |  |  | 0.589 |
| walker |  | 1643 | 38 | go package + imports in format.go |  |  | 0.589 |
| ns | 1648 |  | 57 | Error.Error() — default string formatting | 2.2 | 1.4 | 0.605 |
| walker |  | 1693 | 50 | go decl doc at group.go:20 |  |  | 0.606 |
| walker |  | 1729 | 36 | go decl body at group.go:36 |  |  | 0.606 |
| walker |  | 1767 | 38 | go decl body at sort.go:7 |  |  | 0.607 |
| ns | 1904 |  | 256 | README: Usage intro + accessing the error list | 2.3 |  | 0.557 |
| walker |  | 1942 | 175 | go decl doc at multierror.go:71 |  |  | 0.560 |
| walker |  | 1982 | 40 | go decl body at multierror.go:108 |  |  | 0.561 |
| ns | 2041 |  | 137 | Error.ErrorOrNil() | 2.4 | 1.4 | 0.589 |
| walker |  | 2068 | 86 | go decl doc at prefix.go:16 |  |  | 0.589 |
| walker |  | 2190 | 122 | go decl doc at append.go:14 |  |  | 0.590 |
| ns | 2198 |  | 157 | README: returning a multierror only if there are errors | 2.5 |  | 0.563 |
| walker |  | 2367 | 177 | go decl doc at multierror.go:99 |  |  | 0.569 |
| walker |  | 2521 | 154 | go decl body at multierror.go:71 |  |  | 0.574 |
| ns | 2534 |  | 336 | Error.Unwrap() | 2.6 | 1.4 | 0.618 |
| walker |  | 2620 | 99 | go decl body at flatten.go:8 |  |  | 0.619 |
| walker |  | 2721 | 101 | go decl body at group.go:20 |  |  | 0.621 |
| walker |  | 2863 | 142 | go decl body at format.go:17 |  |  | 0.624 |
| walker |  | 2944 | 81 | go decl body at flatten.go:20 |  |  | 0.625 |
| ns | 2971 |  | 437 | chain type (unexported Unwrap/Is/As helper) | 2.7 |  | 0.649 |
| walker |  | 3142 | 198 | go decl body at prefix.go:16 |  |  | 0.652 |
| ns | 3144 |  | 173 | README: extracting an error via errors.As | 2.8 |  | 0.630 |
| ns | 3308 |  | 164 | README: checking for an exact error value via errors.Is | 2.9 |  | 0.613 |
| walker |  | 3449 | 307 | README.md section #2 |  |  | 0.682 |
| ns | 3463 |  | 155 | Error.WrappedErrors() | 2.10 | 1.4 | 0.689 |
| ns | 3483 |  | 20 | Error.GoString() | 2.11 | 1.4 | 0.689 |
| walker |  | 3769 | 320 | go decl body at append.go:14 |  |  | 0.697 |
| walker |  | 3783 | 14 | listing of '.github' |  |  | 0.705 |
| walker |  | 3795 | 12 | listing of '.github/workflows' |  |  | 0.720 |
| ns | 3834 |  | 351 | Append() doc + *Error branch | 3.1 | 1.5 | 0.738 |
| walker |  | 3851 | 56 | headings outline in CHANGELOG.md |  |  | 0.739 |
| walker |  | 3851 | 0 | CHANGELOG.md section #0 |  |  | 0.739 |
| ns | 3939 |  | 105 | Append() default branch | 3.2 | 3.1 | 0.742 |
| ns | 4140 |  | 201 | README: building a list of errors (Append usage) | 3.3 |  | 0.718 |
| ns | 4352 |  | 212 | README: migrating basic error aggregation to errors.Join | 3.4 |  | 0.693 |
| ns | 4385 |  | 33 | ErrorFormatFunc type | 4.1 | 1.5 | 0.695 |
| ns | 4569 |  | 184 | ListFormatFunc — the default formatter | 4.2 | 1.5 | 0.702 |
| ns | 4712 |  | 143 | README: customizing the formatting of the errors | 4.3 |  | 0.686 |
| walker |  | 4817 | 966 | README.md section #1 |  |  | 0.758 |
| walker |  | 4851 | 34 | headings outline in .github/pull_request_template.md |  |  | 0.758 |
| ns | 4883 |  | 171 | README: migrating custom formatting to errors.Join | 4.4 |  | 0.764 |
| ns | 4945 |  | 62 | Group struct | 5.1 | 1.5 | 0.766 |
| ns | 5106 |  | 161 | Group.Go() | 5.2 | 1.5 | 0.770 |
| ns | 5176 |  | 70 | Group.Wait() | 5.3 | 1.5 | 0.771 |
| ns | 5517 |  | 341 | README: migrating Group to errgroup / a manual mutex | 5.4 |  | 0.779 |
| ns | 5655 |  | 138 | Flatten() | 6.1 | 1.5 | 0.781 |
| ns | 5754 |  | 99 | flatten() recursive helper (unexported) | 6.2 |  | 0.782 |
| walker |  | 5963 | 1112 | README.md section #3 |  |  | 0.912 |
| walker |  | 5980 | 17 | go test names surface in group_test.go |  |  | 0.912 |
| walker |  | 6004 | 24 | .github/pull_request_template.md section #0 |  |  | 0.912 |
| walker |  | 6038 | 34 | go test names surface in flatten_test.go |  |  | 0.913 |
| walker |  | 6072 | 34 | go test names surface in sort_test.go |  |  | 0.914 |
| ns | 6073 |  | 319 | Prefix() | 7.1 | 1.5 | 0.911 |
| walker |  | 6110 | 38 | go test names surface in format_test.go |  |  | 0.914 |
| walker |  | 6133 | 23 | .github/pull_request_template.md section #2 |  |  | 0.915 |
| walker |  | 6187 | 54 | go test names surface in prefix_test.go |  |  | 0.921 |
| ns | 6223 |  | 150 | Len/Swap/Less — sort.Interface | 8.1 | 1.5 | 0.915 |
| walker |  | 6224 | 37 | .github/pull_request_template.md section #1 |  |  | 0.916 |
| walker |  | 6336 | 112 | go test names surface in append_test.go |  |  | 0.936 |
| walker |  | 6472 | 136 | go test names surface in multierror_test.go |  |  | 0.954 |
| ns | 6587 |  | 364 | TestErrorUnwrap | 9.1 | 1.6 | 0.922 |
| ns | 6939 |  | 352 | TestAppend_Error | 9.2 | 1.7 | 0.894 |
| ns | 7396 |  | 457 | TestGroup (whole file) | 9.3 | 1.7 | 0.862 |
| ns | 7642 |  | 246 | TestFlatten | 9.4 | 1.7 | 0.841 |
| ns | 7934 |  | 292 | prefix_test.go (whole file) | 9.5 | 1.7 | 0.820 |
| ns | 8213 |  | 279 | format_test.go (whole file) | 9.6 | 1.7 | 0.797 |
| ns | 8453 |  | 240 | TestSortMultiple (sort_test.go) | 9.7 | 1.7 | 0.780 |
| ns | 8594 |  | 141 | TestErrorErrorOrNil | 9.8 | 1.6 | 0.773 |
| ns | 8982 |  | 388 | Makefile | 10.1 |  | 0.757 |
| ns | 9024 |  | 42 | CI job-name locations (go-multierror.yml) | 10.2 |  | 0.755 |
| ns | 9224 |  | 200 | linux-tests job body | 10.3 | 10.2 | 0.746 |
| ns | 9426 |  | 202 | actionlint.yml | 10.4 |  | 0.737 |
| ns | 9610 |  | 184 | dependabot.yml | 10.5 |  | 0.728 |
| ns | 9690 |  | 80 | CODEOWNERS | 10.6 |  | 0.725 |
| ns | 9843 |  | 153 | PR template | 10.7 |  | 0.725 |
| ns | 9899 |  | 56 | CHANGELOG.md | 10.8 |  | 0.727 |
| ns | 9936 |  | 37 | LICENSE header | 10.9 |  | 0.726 |
