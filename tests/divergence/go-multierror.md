Score(3000)=0.743 I=0.944 C=0.585 ns_rows≤3K=19/39 (reached=15 partial=0 missing=4)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 78 | 78 | listing of '.' |  |  | 1.000 |
| ns | 78 |  | 78 | Top-level fs listing | 1.1 |  | 1.000 |
| walker |  | 102 | 24 | go module file go.mod |  |  | 1.000 |
| walker |  | 118 | 16 | go decl names surface in prefix.go |  |  | 1.000 |
| walker |  | 118 | 0 | go decl at prefix.go:16 |  |  | 1.000 |
| walker |  | 136 | 18 | go decl names surface in append.go |  |  | 1.000 |
| walker |  | 136 | 0 | go decl at append.go:14 |  |  | 1.000 |
| walker |  | 150 | 14 | listing of '.github' |  |  | 1.000 |
| walker |  | 180 | 30 | go decl names surface in flatten.go |  |  | 1.000 |
| walker |  | 180 | 0 | go decl at flatten.go:8 |  |  | 1.000 |
| walker |  | 180 | 0 | go decl at flatten.go:20 |  |  | 1.000 |
| ns | 181 |  | 103 | README lede + deprecation note | 1.2 |  | 0.953 |
| walker |  | 188 | 8 | go package + imports in append.go |  |  | 0.955 |
| walker |  | 196 | 8 | go package + imports in flatten.go |  |  | 0.958 |
| walker |  | 204 | 8 | go package + imports in sort.go |  |  | 0.959 |
| walker |  | 236 | 32 | go decl names surface in format.go |  |  | 0.959 |
| walker |  | 236 | 0 | go decl at format.go:13 |  |  | 0.959 |
| walker |  | 236 | 0 | go decl at format.go:17 |  |  | 0.959 |
| ns | 252 |  | 71 | Error struct definition | 1.3 |  | 0.844 |
| walker |  | 280 | 44 | headings outline in CHANGELOG.md |  |  | 0.844 |
| walker |  | 280 | 0 | CHANGELOG.md section #0 |  |  | 0.844 |
| walker |  | 326 | 46 | go decl names surface in group.go |  |  | 0.848 |
| walker |  | 326 | 0 | go decl at group.go:20 |  |  | 0.848 |
| walker |  | 326 | 0 | go decl at group.go:36 |  |  | 0.848 |
| ns | 337 |  | 85 | Package + import declarations across all .go files | 2.1 |  | 0.757 |
| walker |  | 357 | 31 | go decl at group.go:10 |  |  | 0.763 |
| walker |  | 369 | 12 | listing of '.github/workflows' |  |  | 0.763 |
| ns | 400 |  | 63 | All public function/method first-lines (multierror.go) | 2.2 |  | 0.667 |
| walker |  | 422 | 53 | go decl names surface in sort.go |  |  | 0.671 |
| walker |  | 422 | 0 | go decl at sort.go:7 |  |  | 0.671 |
| walker |  | 422 | 0 | go decl at sort.go:16 |  |  | 0.671 |
| walker |  | 422 | 0 | go decl at sort.go:21 |  |  | 0.671 |
| walker |  | 435 | 13 | go decl doc at sort.go:7 |  |  | 0.672 |
| walker |  | 449 | 14 | go decl doc at sort.go:16 |  |  | 0.673 |
| walker |  | 463 | 14 | go decl doc at sort.go:21 |  |  | 0.674 |
| ns | 564 |  | 164 | Public function names across single-fn files | 2.3 |  | 0.654 |
| ns | 592 |  | 28 | README H2 headings | 2.4 |  | 0.638 |
| walker |  | 612 | 149 | README headline in README.md |  |  | 0.662 |
| walker |  | 663 | 51 | headings outline in README.md |  |  | 0.691 |
| walker |  | 695 | 32 | headings outline in .github/pull_request_template.md |  |  | 0.691 |
| walker |  | 711 | 16 | go package + imports in group.go |  |  | 0.704 |
| ns | 725 |  | 133 | Append doc comment + signature | 3.1 | 2.3 | 0.658 |
| walker |  | 727 | 16 | go decl body at sort.go:21 |  |  | 0.659 |
| walker |  | 756 | 29 | go decl doc at group.go:10 |  |  | 0.671 |
| walker |  | 789 | 33 | go decl doc at format.go:13 |  |  | 0.740 |
| walker |  | 808 | 19 | go decl body at sort.go:16 |  |  | 0.740 |
| walker |  | 838 | 30 | go decl doc at flatten.go:8 |  |  | 0.741 |
| walker |  | 868 | 30 | go decl doc at group.go:36 |  |  | 0.743 |
| walker |  | 894 | 26 | go package + imports in prefix.go |  |  | 0.756 |
| ns | 909 |  | 184 | Error.Unwrap doc + signature | 3.2 | 2.2 | 0.697 |
| walker |  | 930 | 36 | go decl doc at format.go:17 |  |  | 0.699 |
| walker |  | 963 | 33 | go package + imports in format.go |  |  | 0.714 |
| walker |  | 996 | 33 | go package + imports in multierror.go |  |  | 0.729 |
| ns | 1011 |  | 102 | Group struct + Go/Wait signatures with doc comments | 3.3 | 2.3 | 0.708 |
| walker |  | 1017 | 21 | .github/pull_request_template.md section #0 |  |  | 0.708 |
| walker |  | 1048 | 31 | go decl body at sort.go:7 |  |  | 0.708 |
| walker |  | 1100 | 52 | go decl doc at group.go:20 |  |  | 0.748 |
| walker |  | 1134 | 34 | go decl body at group.go:36 |  |  | 0.748 |
| ns | 1140 |  | 129 | Flatten + Prefix doc + signatures | 3.4 | 2.3 | 0.716 |
| ns | 1186 |  | 46 | ErrorFormatFunc + ListFormatFunc docs | 3.5 | 2.3 | 0.727 |
| walker |  | 1306 | 172 | go decl names surface in multierror.go |  |  | 0.795 |
| walker |  | 1306 | 0 | go decl at multierror.go:18 |  |  | 0.795 |
| walker |  | 1306 | 0 | go decl at multierror.go:31 |  |  | 0.795 |
| walker |  | 1306 | 0 | go decl at multierror.go:42 |  |  | 0.795 |
| walker |  | 1306 | 0 | go decl at multierror.go:53 |  |  | 0.795 |
| walker |  | 1306 | 0 | go decl at multierror.go:71 |  |  | 0.795 |
| walker |  | 1306 | 0 | go decl at multierror.go:99 |  |  | 0.795 |
| walker |  | 1306 | 0 | go decl at multierror.go:102 |  |  | 0.795 |
| walker |  | 1306 | 0 | go decl at multierror.go:108 |  |  | 0.795 |
| walker |  | 1306 | 0 | go decl at multierror.go:117 |  |  | 0.795 |
| walker |  | 1306 | 0 | go decl at multierror.go:122 |  |  | 0.795 |
| walker |  | 1329 | 23 | go decl at multierror.go:13 |  |  | 0.804 |
| ns | 1331 |  | 145 | Error.Error + WrappedErrors + GoString docs | 3.6 | 2.2 | 0.770 |
| walker |  | 1342 | 13 | go decl body at multierror.go:42 |  |  | 0.770 |
| walker |  | 1353 | 11 | go decl doc at multierror.go:102 |  |  | 0.770 |
| walker |  | 1362 | 9 | go decl body at multierror.go:102 |  |  | 0.770 |
| walker |  | 1400 | 38 | go decl doc at multierror.go:13 |  |  | 0.804 |
| walker |  | 1411 | 11 | go decl body at multierror.go:117 |  |  | 0.804 |
| ns | 1419 |  | 88 | sort.Interface methods on Error | 3.7 | 2.3 | 0.809 |
| walker |  | 1422 | 11 | go decl body at multierror.go:122 |  |  | 0.809 |
| walker |  | 1438 | 16 | go decl doc at multierror.go:122 |  |  | 0.809 |
| walker |  | 1456 | 18 | go decl doc at multierror.go:117 |  |  | 0.810 |
| walker |  | 1485 | 29 | go decl body at multierror.go:53 |  |  | 0.810 |
| walker |  | 1557 | 72 | go decl doc at multierror.go:31 |  |  | 0.810 |
| ns | 1596 |  | 177 | chain type explainer comment + decl | 3.8 | 3.2 | 0.771 |
| walker |  | 1600 | 43 | go decl body at multierror.go:18 |  |  | 0.771 |
| walker |  | 1634 | 34 | go decl doc at multierror.go:108 |  |  | 0.772 |
| ns | 1718 |  | 122 | README usage section overview lines | 4.1 |  | 0.743 |
| walker |  | 1722 | 88 | go decl doc at prefix.go:16 |  |  | 0.781 |
| walker |  | 1777 | 55 | go decl body at multierror.go:31 |  |  | 0.782 |
| walker |  | 1797 | 20 | .github/pull_request_template.md section #2 |  |  | 0.782 |
| ns | 1908 |  | 190 | README Append usage example | 4.2 | 4.1 | 0.732 |
| walker |  | 1921 | 124 | go decl doc at append.go:14 |  |  | 0.771 |
| walker |  | 2039 | 118 | go decl doc at multierror.go:53 |  |  | 0.807 |
| walker |  | 2072 | 33 | go decl body at multierror.go:108 |  |  | 0.808 |
| walker |  | 2164 | 92 | go decl body at flatten.go:8 |  |  | 0.809 |
| walker |  | 2339 | 175 | go decl doc at multierror.go:71 |  |  | 0.858 |
| ns | 2416 |  | 508 | README errors.Is / errors.As / Unwrap stdlib-compat snippets | 4.3 | 4.1 | 0.753 |
| walker |  | 2428 | 89 | go decl body at group.go:20 |  |  | 0.755 |
| walker |  | 2462 | 34 | .github/pull_request_template.md section #1 |  |  | 0.755 |
| walker |  | 2592 | 130 | go decl body at format.go:17 |  |  | 0.757 |
| ns | 2690 |  | 274 | README ErrorFormat + ErrorOrNil examples | 4.4 | 4.1 | 0.703 |
| walker |  | 2769 | 177 | go decl doc at multierror.go:99 |  |  | 0.739 |
| walker |  | 2911 | 142 | go decl body at multierror.go:71 |  |  | 0.742 |
| walker |  | 2990 | 79 | go decl body at flatten.go:20 |  |  | 0.743 |
| ns | 3075 |  | 385 | README intro paragraph (unwrap + Go-version notes) | 4.5 |  | 0.708 |
| walker |  | 3171 | 181 | go decl body at prefix.go:16 |  |  | 0.710 |
| walker |  | 3447 | 276 | README.md section #2 |  |  | 0.718 |
| ns | 3611 |  | 536 | README migration to errors.Join — basic + Group sections | 4.6 |  | 0.646 |
| walker |  | 3750 | 303 | go decl body at append.go:14 |  |  | 0.651 |
| ns | 3936 |  | 325 | Append body | 5.1 | 3.1 | 0.671 |
| ns | 4319 |  | 383 | Error.Unwrap body + chain methods | 5.2 | 3.2 | 0.680 |
| ns | 4466 |  | 147 | Group.Go and Group.Wait bodies | 5.3 | 3.3 | 0.684 |
| walker |  | 4648 | 898 | README.md section #1 |  |  | 0.794 |
| ns | 4669 |  | 203 | Flatten body (Flatten + flatten recursion) | 5.4 | 3.4 | 0.795 |
| ns | 4872 |  | 203 | Prefix body | 5.5 | 3.4 | 0.795 |
| ns | 5019 |  | 147 | ListFormatFunc body | 5.6 | 3.5 | 0.794 |
| ns | 5197 |  | 178 | Error.Error / ErrorOrNil / WrappedErrors / GoString bodies | 5.7 | 3.6 | 0.792 |
| ns | 5271 |  | 74 | sort.Interface bodies | 5.8 | 3.7 | 0.790 |
| ns | 5514 |  | 243 | Test function name inventory across all _test.go files | 6.1 |  | 0.769 |
| ns | 5622 |  | 108 | Format expected-output strings from tests | 6.2 |  | 0.756 |
| walker |  | 5627 | 979 | README.md section #3 |  |  | 0.866 |
| walker |  | 5642 | 15 | go test names surface in group_test.go |  |  | 0.866 |
| walker |  | 5674 | 32 | go test names surface in flatten_test.go |  |  | 0.866 |
| walker |  | 5706 | 32 | go test names surface in sort_test.go |  |  | 0.867 |
| walker |  | 5742 | 36 | go test names surface in format_test.go |  |  | 0.868 |
| walker |  | 5794 | 52 | go test names surface in prefix_test.go |  |  | 0.870 |
| walker |  | 5904 | 110 | go test names surface in append_test.go |  |  | 0.878 |
| ns | 5988 |  | 366 | TestErrorUnwrap — chain semantics in action | 6.3 | 6.1 | 0.847 |
| walker |  | 6038 | 134 | go test names surface in multierror_test.go |  |  | 0.862 |
| ns | 6733 |  | 745 | TestAppend bodies — Append edge cases | 6.4 | 6.1 | 0.805 |
| ns | 7458 |  | 725 | TestFlatten + TestGroup bodies | 6.5 | 6.1 | 0.759 |
| ns | 8394 |  | 936 | TestErrorIs + TestErrorAs bodies | 6.6 | 6.1 | 0.708 |
| ns | 8988 |  | 594 | Remaining test bodies (sort + prefix) | 6.7 | 6.1 | 0.677 |
| ns | 9527 |  | 539 | .github/ listing + workflow file structure | 6.8 |  | 0.653 |
| ns | 9759 |  | 232 | Makefile targets | 6.9 |  | 0.646 |
| ns | 9864 |  | 105 | Boilerplate metadata | 6.10 |  | 0.643 |
