Score(3000)=0.624 I=0.869 C=0.448 ns_rows≤3K=16/40 (reached=10 partial=1 missing=5)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 31 | 31 | listing of '.' |  |  | 1.000 |
| ns | 31 |  | 31 | Root file listing | 1.1 |  | 1.000 |
| ns | 96 |  | 65 | README heading map, part 1 (top matter through 'Obtaining the string length') | 1.2 |  | 0.715 |
| walker |  | 140 | 109 | README headline in README.md |  |  | 0.730 |
| ns | 183 |  | 87 | sds typedef, SDS_MAX_PREALLOC, SDS_NOINIT | 1.3 |  | 0.555 |
| walker |  | 257 | 117 | plaintext config Makefile |  |  | 0.559 |
| ns | 281 |  | 98 | struct sdshdr5 layout | 1.4 |  | 0.483 |
| walker |  | 290 | 33 | c decl names surface in sdsalloc.h |  |  | 0.484 |
| ns | 364 |  | 83 | struct sdshdr8 layout | 1.5 |  | 0.430 |
| ns | 447 |  | 83 | struct sdshdr16 layout | 1.6 |  | 0.390 |
| ns | 530 |  | 83 | struct sdshdr32 layout | 1.7 |  | 0.359 |
| ns | 613 |  | 83 | struct sdshdr64 layout | 1.8 |  | 0.333 |
| walker |  | 689 | 399 | headings outline in README.md |  |  | 0.464 |
| walker |  | 747 | 58 | c decl names surface in testhelp.h |  |  | 0.464 |
| ns | 794 |  | 181 | SDS_TYPE_* constants and SDS_HDR/SDS_HDR_VAR macros | 1.9 |  | 0.414 |
| ns | 1025 |  | 231 | README lede: what SDS is, v1-vs-v2 compatibility | 1.10 | 1.2 | 0.394 |
| walker |  | 1092 | 345 | README.md section #0 |  |  | 0.539 |
| ns | 1348 |  | 323 | README: how SDS strings work (header-prefix design + diagram) | 1.11 | 1.2 | 0.546 |
| walker |  | 1535 | 443 | c decl names surface in sds.h |  |  | 0.657 |
| walker |  | 1535 | 0 | c decl at sds.h:87 |  |  | 0.657 |
| walker |  | 1535 | 0 | c decl at sds.h:104 |  |  | 0.657 |
| walker |  | 1535 | 0 | c decl at sds.h:130 |  |  | 0.657 |
| walker |  | 1535 | 0 | c decl at sds.h:154 |  |  | 0.657 |
| walker |  | 1535 | 0 | c decl at sds.h:180 |  |  | 0.657 |
| walker |  | 1535 | 0 | c decl at sds.h:197 |  |  | 0.657 |
| walker |  | 1572 | 37 | c decl at sds.h:47 |  |  | 0.672 |
| walker |  | 1637 | 65 | c decl at sds.h:51 |  |  | 0.718 |
| walker |  | 1702 | 65 | c decl at sds.h:57 |  |  | 0.761 |
| walker |  | 1767 | 65 | c decl at sds.h:63 |  |  | 0.802 |
| ns | 1806 |  | 458 | sdslen() / sdsavail() inline accessors | 1.12 |  | 0.660 |
| walker |  | 1832 | 65 | c decl at sds.h:69 |  |  | 0.693 |
| walker |  | 1862 | 30 | c includes in sds.h |  |  | 0.729 |
| walker |  | 1882 | 20 | c decl doc at sds.h:180 |  |  | 0.729 |
| ns | 1969 |  | 163 | sdssetlen() inline mutator (TYPE_16/32/64 cases elided) | 1.13 |  | 0.690 |
| ns | 2149 |  | 180 | sdsinclen() inline mutator (TYPE_16/32/64 cases elided) | 1.14 |  | 0.654 |
| walker |  | 2333 | 451 | c decl names surface #1 in sds.h |  |  | 0.658 |
| walker |  | 2351 | 18 | c decl at sds.h:232 |  |  | 0.658 |
| ns | 2570 |  | 421 | README: disadvantages of the single-allocation design | 1.15 | 1.2 | 0.605 |
| walker |  | 2631 | 280 | c decl names surface #2 in sds.h |  |  | 0.610 |
| walker |  | 2631 | 0 | c decl at sds.h:256 |  |  | 0.610 |
| walker |  | 2631 | 0 | c decl at sds.h:266 |  |  | 0.610 |
| walker |  | 2643 | 12 | c decl doc at sds.h:256 |  |  | 0.611 |
| ns | 2687 |  | 117 | Makefile | 2.1 |  | 0.624 |
| walker |  | 3136 | 493 | c header banner in sds.h |  |  | 0.624 |
| walker |  | 3195 | 59 | c decl at testhelp.h:44 |  |  | 0.624 |
| walker |  | 3261 | 66 | c decl doc at sds.h:266 |  |  | 0.626 |
| walker |  | 3302 | 41 | c decl doc at sds.h:47 |  |  | 0.648 |
| ns | 3370 |  | 683 | sds.h public function declarations, part 1 (creation through low-level MakeRoomFor/IncrLen) | 2.2 |  | 0.683 |
| walker |  | 3401 | 99 | c decl at testhelp.h:48 |  |  | 0.683 |
| ns | 3566 |  | 196 | sds.h public function declarations, part 2 (RemoveFreeSpace/AllocSize/allocator wrappers/sdsTest) | 2.3 |  | 0.690 |
| ns | 3690 |  | 124 | sdsalloc.h allocator macros | 2.4 |  | 0.677 |
| ns | 3819 |  | 129 | Changelog | 2.5 |  | 0.661 |
| walker |  | 3877 | 476 | c decl names surface in sds.c |  |  | 0.663 |
| walker |  | 3877 | 0 | c decl at sds.c:89 |  |  | 0.663 |
| walker |  | 3877 | 0 | c decl at sds.c:149 |  |  | 0.663 |
| walker |  | 3877 | 0 | c decl at sds.c:154 |  |  | 0.663 |
| walker |  | 3877 | 0 | c decl at sds.c:160 |  |  | 0.663 |
| walker |  | 3877 | 0 | c decl at sds.c:165 |  |  | 0.663 |
| walker |  | 3877 | 0 | c decl at sds.c:184 |  |  | 0.663 |
| walker |  | 3877 | 0 | c decl at sds.c:193 |  |  | 0.663 |
| walker |  | 3877 | 0 | c decl at sds.c:204 |  |  | 0.663 |
| walker |  | 3877 | 0 | c decl at sds.c:256 |  |  | 0.663 |
| walker |  | 3877 | 0 | c decl at sds.c:300 |  |  | 0.663 |
| walker |  | 3877 | 0 | c decl at sds.c:307 |  |  | 0.663 |
| walker |  | 3877 | 0 | c decl at sds.c:334 |  |  | 0.663 |
| walker |  | 3877 | 0 | c decl at sds.c:380 |  |  | 0.663 |
| walker |  | 3877 | 0 | c decl at sds.c:398 |  |  | 0.663 |
| walker |  | 3877 | 0 | c decl at sds.c:413 |  |  | 0.663 |
| walker |  | 3877 | 0 | c decl at sds.c:421 |  |  | 0.663 |
| walker |  | 3877 | 0 | c decl at sds.c:427 |  |  | 0.663 |
| walker |  | 3877 | 0 | c decl at sds.c:440 |  |  | 0.663 |
| walker |  | 3877 | 0 | c decl at sds.c:450 |  |  | 0.663 |
| walker |  | 3877 | 0 | c decl at sds.c:451 |  |  | 0.663 |
| walker |  | 3877 | 0 | c decl at sds.c:494 |  |  | 0.663 |
| walker |  | 3877 | 0 | c decl at sds.c:526 |  |  | 0.663 |
| walker |  | 3877 | 0 | c decl at sds.c:534 |  |  | 0.663 |
| walker |  | 3890 | 13 | c decl body at sds.c:149 |  |  | 0.663 |
| walker |  | 3902 | 12 | c decl doc at sds.c:160 |  |  | 0.663 |
| walker |  | 3919 | 17 | c decl body at sds.c:160 |  |  | 0.663 |
| walker |  | 3936 | 17 | c decl body at sds.c:413 |  |  | 0.663 |
| walker |  | 3953 | 17 | c decl body at sds.c:440 |  |  | 0.663 |
| walker |  | 3972 | 19 | c decl body at sds.c:307 |  |  | 0.663 |
| walker |  | 3991 | 19 | c decl body at sds.c:421 |  |  | 0.663 |
| walker |  | 4011 | 20 | c decl doc at sds.c:154 |  |  | 0.663 |
| walker |  | 4038 | 27 | c decl body at sds.c:184 |  |  | 0.663 |
| walker |  | 4065 | 27 | c decl body at sds.c:193 |  |  | 0.663 |
| walker |  | 4086 | 21 | c decl doc at sds.c:494 |  |  | 0.663 |
| walker |  | 4108 | 22 | c decl doc at sds.c:534 |  |  | 0.663 |
| walker |  | 4131 | 23 | c decl doc at sds.c:165 |  |  | 0.663 |
| walker |  | 4163 | 32 | c decl body at sds.c:165 |  |  | 0.663 |
| walker |  | 4195 | 32 | c decl body at sds.c:300 |  |  | 0.663 |
| walker |  | 4232 | 37 | c decl body at sds.c:154 |  |  | 0.663 |
| walker |  | 4266 | 34 | c decl doc at sds.c:307 |  |  | 0.663 |
| walker |  | 4302 | 36 | c decl doc at sds.c:149 |  |  | 0.663 |
| walker |  | 4349 | 47 | c decl body at sds.c:526 |  |  | 0.663 |
| ns | 4389 |  | 570 | sds.c function-definition location catalog | 2.6 |  | 0.631 |
| walker |  | 4432 | 83 | c includes in sds.c |  |  | 0.631 |
| walker |  | 4473 | 41 | c decl doc at sds.c:440 |  |  | 0.631 |
| ns | 4493 |  | 104 | README heading map, part 2 (Destroying strings through String joining) | 3.1 |  | 0.640 |
| walker |  | 4515 | 42 | c decl doc at sds.c:427 |  |  | 0.640 |
| ns | 4608 |  | 115 | README heading map, part 3 (Error handling through Credits and license) | 3.2 |  | 0.648 |
| walker |  | 4680 | 165 | c decl body at sds.h:87 |  |  | 0.664 |
| ns | 4711 |  | 103 | README: error handling convention | 3.3 | 3.2 | 0.655 |
| walker |  | 4845 | 165 | c decl body at sds.h:180 |  |  | 0.655 |
| ns | 4856 |  | 145 | README: interactions with heap checkers | 3.4 | 3.2 | 0.647 |
| walker |  | 4896 | 51 | c decl doc at sds.c:526 |  |  | 0.647 |
| ns | 4941 |  | 85 | README: sharing SDS strings (refcounting advice, intro) | 3.5 | 3.2 | 0.640 |
| ns | 5100 |  | 159 | README: sharing SDS strings (refcounting advice, increment/decrement contract) | 3.6 |  | 0.633 |
| walker |  | 5101 | 205 | c decl body at sds.h:197 |  |  | 0.633 |
| walker |  | 5168 | 67 | c decl doc at sds.c:421 |  |  | 0.633 |
| walker |  | 5405 | 237 | c decl body at sds.h:130 |  |  | 0.662 |
| ns | 5424 |  | 324 | sdsHdrSize() / sdsReqType() bodies | 4.1 | 2.6 | 0.633 |
| walker |  | 5473 | 68 | c decl doc at sds.c:413 |  |  | 0.633 |
| walker |  | 5542 | 69 | c decl doc at sds.c:193 |  |  | 0.633 |
| walker |  | 5786 | 244 | c decl body at sds.h:104 |  |  | 0.695 |
| ns | 6014 |  | 590 | sdsnewlen() body, part 1 (doc comment through TYPE_8 case) | 4.2 | 2.6 | 0.658 |
| walker |  | 6038 | 252 | c decl body at sds.h:154 |  |  | 0.683 |
| ns | 6074 |  | 60 | sdsnewlen() body, part 2 (memcpy/null-term tail; TYPE_16/32/64 cases elided) | 4.3 | 2.6 | 0.677 |
| walker |  | 6111 | 73 | c decl doc at sds.c:380 |  |  | 0.678 |
| walker |  | 6211 | 100 | c decl body at sds.c:427 |  |  | 0.678 |
| walker |  | 6315 | 104 | c decl body at sds.c:398 |  |  | 0.678 |
| walker |  | 6755 | 440 | c decl names surface #1 in sds.c |  |  | 0.718 |
| walker |  | 6755 | 0 | c decl at sds.c:591 |  |  | 0.718 |
| walker |  | 6755 | 0 | c decl at sds.c:616 |  |  | 0.718 |
| walker |  | 6755 | 0 | c decl at sds.c:725 |  |  | 0.718 |
| walker |  | 6755 | 0 | c decl at sds.c:756 |  |  | 0.718 |
| walker |  | 6755 | 0 | c decl at sds.c:783 |  |  | 0.718 |
| walker |  | 6755 | 0 | c decl at sds.c:790 |  |  | 0.718 |
| walker |  | 6755 | 0 | c decl at sds.c:807 |  |  | 0.718 |
| walker |  | 6755 | 0 | c decl at sds.c:835 |  |  | 0.718 |
| walker |  | 6755 | 0 | c decl at sds.c:885 |  |  | 0.718 |
| walker |  | 6755 | 0 | c decl at sds.c:898 |  |  | 0.718 |
| walker |  | 6755 | 0 | c decl at sds.c:925 |  |  | 0.718 |
| walker |  | 6755 | 0 | c decl at sds.c:932 |  |  | 0.718 |
| walker |  | 6755 | 0 | c decl at sds.c:973 |  |  | 0.718 |
| walker |  | 6755 | 0 | c decl at sds.c:1092 |  |  | 0.718 |
| walker |  | 6755 | 0 | c decl at sds.c:1108 |  |  | 0.718 |
| walker |  | 6755 | 0 | c decl at sds.c:1120 |  |  | 0.718 |
| walker |  | 6755 | 0 | c decl at sds.c:1136 |  |  | 0.718 |
| walker |  | 6775 | 20 | c decl doc at sds.c:1120 |  |  | 0.718 |
| ns | 6795 |  | 721 | sdsMakeRoomFor() body | 4.4 | 2.6 | 0.677 |
| walker |  | 6796 | 21 | c decl doc at sds.c:783 |  |  | 0.677 |
| walker |  | 6817 | 21 | c decl doc at sds.c:790 |  |  | 0.677 |
| walker |  | 6843 | 26 | c decl doc at sds.c:885 |  |  | 0.677 |
| walker |  | 6882 | 39 | c decl body at sds.c:885 |  |  | 0.677 |
| walker |  | 6915 | 33 | c decl doc at sds.c:925 |  |  | 0.677 |
| walker |  | 6949 | 34 | c decl doc at sds.c:932 |  |  | 0.677 |
| walker |  | 6997 | 48 | c decl body at sds.c:783 |  |  | 0.677 |
| walker |  | 7045 | 48 | c decl body at sds.c:790 |  |  | 0.677 |
| walker |  | 7094 | 49 | c decl body at sds.c:925 |  |  | 0.677 |
| ns | 7115 |  | 320 | sdsIncrLen() body (TYPE_5/TYPE_8 cases shown; TYPE_16/32/64 elided) | 4.5 | 2.6 | 0.660 |
| walker |  | 7132 | 38 | c decl doc at sds.c:1108 |  |  | 0.660 |
| walker |  | 7196 | 64 | c decl body at sds.c:591 |  |  | 0.660 |
| walker |  | 7300 | 104 | c decl body at sds.c:1108 |  |  | 0.660 |
| ns | 7334 |  | 219 | sdsgrowzero() body | 4.6 | 2.6 | 0.652 |
| walker |  | 7408 | 108 | c decl body at sds.c:1120 |  |  | 0.652 |
| walker |  | 7493 | 85 | c decl doc at sds.c:398 |  |  | 0.653 |
| ns | 7539 |  | 205 | sdscatlen() body | 5.1 | 2.6 | 0.659 |
| walker |  | 7581 | 88 | c decl doc at sds.c:300 |  |  | 0.659 |
| walker |  | 7667 | 86 | c decl doc at sds.c:450 |  |  | 0.659 |
| walker |  | 7765 | 98 | c decl doc at sds.c:256 |  |  | 0.659 |
| ns | 7888 |  | 349 | sdstrim() body | 5.2 | 2.6 | 0.642 |
| walker |  | 7898 | 133 | c decl body at sds.c:1092 |  |  | 0.642 |
| walker |  | 8001 | 103 | c decl doc at sds.c:1136 |  |  | 0.642 |
| ns | 8393 |  | 505 | sdsrange() body | 5.3 | 2.6 | 0.618 |
| walker |  | 8426 | 425 | README.md section #1 |  |  | 0.641 |
| walker |  | 8844 | 418 | README.md section #2 |  |  | 0.672 |
| walker |  | 9252 | 408 | README.md section #3 |  |  | 0.672 |
| ns | 9283 |  | 890 | sdscatfmt() body, part 1 (doc comment, setup, %s/%S and %i/%I cases) | 6.1 | 2.6 | 0.632 |
| walker |  | 9387 | 135 | c decl body at sds.c:380 |  |  | 0.646 |
| ns | 9451 |  | 168 | sdscatfmt() body, part 2 (default case and null-term tail; %u/%U case elided) | 6.2 | 6.1 | 0.636 |
| walker |  | 9525 | 138 | c decl body at sds.c:807 |  |  | 0.636 |
| walker |  | 9635 | 110 | c decl doc at sds.c:898 |  |  | 0.636 |
| walker |  | 9746 | 111 | c decl doc at sds.c:204 |  |  | 0.637 |
| ns | 9886 |  | 435 | sdscatrepr() body | 7.1 | 2.6 | 0.626 |
| walker |  | 9903 | 157 | c decl body at sds.c:725 |  |  | 0.631 |
| ns | 9997 |  | 111 | SDS_TEST_MAIN / main() harness wrapper | 8.1 | 2.6 | 0.624 |
