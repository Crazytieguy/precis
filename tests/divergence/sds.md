Score(3000)=0.541 I=0.847 C=0.346 ns_rows≤3K=16/40 (reached=10 partial=0 missing=6)

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
| walker |  | 1190 | 443 | c decl names surface in sds.h |  |  | 0.554 |
| walker |  | 1190 | 0 | c decl at sds.h:87 |  |  | 0.554 |
| walker |  | 1190 | 0 | c decl at sds.h:104 |  |  | 0.554 |
| walker |  | 1190 | 0 | c decl at sds.h:130 |  |  | 0.554 |
| walker |  | 1190 | 0 | c decl at sds.h:154 |  |  | 0.554 |
| walker |  | 1190 | 0 | c decl at sds.h:180 |  |  | 0.554 |
| walker |  | 1190 | 0 | c decl at sds.h:197 |  |  | 0.554 |
| walker |  | 1227 | 37 | c decl at sds.h:47 |  |  | 0.574 |
| walker |  | 1292 | 65 | c decl at sds.h:51 |  |  | 0.635 |
| ns | 1348 |  | 323 | README: how SDS strings work (header-prefix design + diagram) | 1.11 | 1.2 | 0.538 |
| walker |  | 1357 | 65 | c decl at sds.h:57 |  |  | 0.586 |
| walker |  | 1422 | 65 | c decl at sds.h:63 |  |  | 0.631 |
| walker |  | 1487 | 65 | c decl at sds.h:69 |  |  | 0.674 |
| walker |  | 1517 | 30 | c includes in sds.h |  |  | 0.720 |
| walker |  | 1537 | 20 | c decl doc at sds.h:180 |  |  | 0.720 |
| walker |  | 1578 | 41 | c decl doc at sds.h:47 |  |  | 0.755 |
| ns | 1806 |  | 458 | sdslen() / sdsavail() inline accessors | 1.12 |  | 0.621 |
| ns | 1969 |  | 163 | sdssetlen() inline mutator (TYPE_16/32/64 cases elided) | 1.13 |  | 0.588 |
| walker |  | 2029 | 451 | c decl names surface #1 in sds.h |  |  | 0.592 |
| walker |  | 2047 | 18 | c decl at sds.h:232 |  |  | 0.592 |
| ns | 2149 |  | 180 | sdsinclen() inline mutator (TYPE_16/32/64 cases elided) | 1.14 |  | 0.561 |
| walker |  | 2327 | 280 | c decl names surface #2 in sds.h |  |  | 0.566 |
| walker |  | 2327 | 0 | c decl at sds.h:256 |  |  | 0.566 |
| walker |  | 2327 | 0 | c decl at sds.h:266 |  |  | 0.566 |
| walker |  | 2339 | 12 | c decl doc at sds.h:256 |  |  | 0.567 |
| ns | 2570 |  | 421 | README: disadvantages of the single-allocation design | 1.15 | 1.2 | 0.521 |
| ns | 2687 |  | 117 | Makefile | 2.1 |  | 0.539 |
| walker |  | 2832 | 493 | c header banner in sds.h |  |  | 0.539 |
| walker |  | 2891 | 59 | c decl at testhelp.h:44 |  |  | 0.539 |
| walker |  | 2957 | 66 | c decl doc at sds.h:266 |  |  | 0.541 |
| walker |  | 3122 | 165 | c decl body at sds.h:87 |  |  | 0.570 |
| walker |  | 3287 | 165 | c decl body at sds.h:180 |  |  | 0.570 |
| ns | 3370 |  | 683 | sds.h public function declarations, part 1 (creation through low-level MakeRoomFor/IncrLen) | 2.2 |  | 0.619 |
| walker |  | 3386 | 99 | c decl at testhelp.h:48 |  |  | 0.619 |
| ns | 3566 |  | 196 | sds.h public function declarations, part 2 (RemoveFreeSpace/AllocSize/allocator wrappers/sdsTest) | 2.3 |  | 0.630 |
| ns | 3690 |  | 124 | sdsalloc.h allocator macros | 2.4 |  | 0.618 |
| ns | 3819 |  | 129 | Changelog | 2.5 |  | 0.603 |
| walker |  | 3862 | 476 | c decl names surface in sds.c |  |  | 0.605 |
| walker |  | 3862 | 0 | c decl at sds.c:89 |  |  | 0.605 |
| walker |  | 3862 | 0 | c decl at sds.c:149 |  |  | 0.605 |
| walker |  | 3862 | 0 | c decl at sds.c:154 |  |  | 0.605 |
| walker |  | 3862 | 0 | c decl at sds.c:160 |  |  | 0.605 |
| walker |  | 3862 | 0 | c decl at sds.c:165 |  |  | 0.605 |
| walker |  | 3862 | 0 | c decl at sds.c:184 |  |  | 0.605 |
| walker |  | 3862 | 0 | c decl at sds.c:193 |  |  | 0.605 |
| walker |  | 3862 | 0 | c decl at sds.c:204 |  |  | 0.605 |
| walker |  | 3862 | 0 | c decl at sds.c:256 |  |  | 0.605 |
| walker |  | 3862 | 0 | c decl at sds.c:300 |  |  | 0.605 |
| walker |  | 3862 | 0 | c decl at sds.c:307 |  |  | 0.605 |
| walker |  | 3862 | 0 | c decl at sds.c:334 |  |  | 0.605 |
| walker |  | 3862 | 0 | c decl at sds.c:380 |  |  | 0.605 |
| walker |  | 3862 | 0 | c decl at sds.c:398 |  |  | 0.605 |
| walker |  | 3862 | 0 | c decl at sds.c:413 |  |  | 0.605 |
| walker |  | 3862 | 0 | c decl at sds.c:421 |  |  | 0.605 |
| walker |  | 3862 | 0 | c decl at sds.c:427 |  |  | 0.605 |
| walker |  | 3862 | 0 | c decl at sds.c:440 |  |  | 0.605 |
| walker |  | 3862 | 0 | c decl at sds.c:450 |  |  | 0.605 |
| walker |  | 3862 | 0 | c decl at sds.c:451 |  |  | 0.605 |
| walker |  | 3862 | 0 | c decl at sds.c:494 |  |  | 0.605 |
| walker |  | 3862 | 0 | c decl at sds.c:526 |  |  | 0.605 |
| walker |  | 3862 | 0 | c decl at sds.c:534 |  |  | 0.605 |
| walker |  | 3875 | 13 | c decl body at sds.c:149 |  |  | 0.605 |
| walker |  | 3887 | 12 | c decl doc at sds.c:160 |  |  | 0.605 |
| walker |  | 3904 | 17 | c decl body at sds.c:160 |  |  | 0.605 |
| walker |  | 3921 | 17 | c decl body at sds.c:413 |  |  | 0.605 |
| walker |  | 3938 | 17 | c decl body at sds.c:440 |  |  | 0.605 |
| walker |  | 3957 | 19 | c decl body at sds.c:307 |  |  | 0.605 |
| walker |  | 3976 | 19 | c decl body at sds.c:421 |  |  | 0.605 |
| walker |  | 3996 | 20 | c decl doc at sds.c:154 |  |  | 0.605 |
| walker |  | 4023 | 27 | c decl body at sds.c:184 |  |  | 0.605 |
| walker |  | 4050 | 27 | c decl body at sds.c:193 |  |  | 0.605 |
| walker |  | 4071 | 21 | c decl doc at sds.c:494 |  |  | 0.605 |
| walker |  | 4093 | 22 | c decl doc at sds.c:534 |  |  | 0.605 |
| walker |  | 4116 | 23 | c decl doc at sds.c:165 |  |  | 0.605 |
| walker |  | 4148 | 32 | c decl body at sds.c:165 |  |  | 0.605 |
| walker |  | 4180 | 32 | c decl body at sds.c:300 |  |  | 0.605 |
| walker |  | 4217 | 37 | c decl body at sds.c:154 |  |  | 0.605 |
| walker |  | 4251 | 34 | c decl doc at sds.c:307 |  |  | 0.605 |
| walker |  | 4287 | 36 | c decl doc at sds.c:149 |  |  | 0.605 |
| walker |  | 4334 | 47 | c decl body at sds.c:526 |  |  | 0.605 |
| ns | 4389 |  | 570 | sds.c function-definition location catalog | 2.6 |  | 0.579 |
| walker |  | 4417 | 83 | c includes in sds.c |  |  | 0.579 |
| walker |  | 4458 | 41 | c decl doc at sds.c:440 |  |  | 0.579 |
| ns | 4493 |  | 104 | README heading map, part 2 (Destroying strings through String joining) | 3.1 |  | 0.589 |
| walker |  | 4500 | 42 | c decl doc at sds.c:427 |  |  | 0.589 |
| walker |  | 4551 | 51 | c decl doc at sds.c:526 |  |  | 0.589 |
| ns | 4608 |  | 115 | README heading map, part 3 (Error handling through Credits and license) | 3.2 |  | 0.598 |
| ns | 4711 |  | 103 | README: error handling convention | 3.3 | 3.2 | 0.590 |
| walker |  | 4756 | 205 | c decl body at sds.h:197 |  |  | 0.590 |
| walker |  | 4823 | 67 | c decl doc at sds.c:421 |  |  | 0.590 |
| ns | 4856 |  | 145 | README: interactions with heap checkers | 3.4 | 3.2 | 0.582 |
| ns | 4941 |  | 85 | README: sharing SDS strings (refcounting advice, intro) | 3.5 | 3.2 | 0.576 |
| walker |  | 5060 | 237 | c decl body at sds.h:130 |  |  | 0.606 |
| ns | 5100 |  | 159 | README: sharing SDS strings (refcounting advice, increment/decrement contract) | 3.6 |  | 0.599 |
| walker |  | 5128 | 68 | c decl doc at sds.c:413 |  |  | 0.599 |
| walker |  | 5197 | 69 | c decl doc at sds.c:193 |  |  | 0.599 |
| ns | 5424 |  | 324 | sdsHdrSize() / sdsReqType() bodies | 4.1 | 2.6 | 0.573 |
| walker |  | 5441 | 244 | c decl body at sds.h:104 |  |  | 0.637 |
| walker |  | 5693 | 252 | c decl body at sds.h:154 |  |  | 0.665 |
| walker |  | 5766 | 73 | c decl doc at sds.c:380 |  |  | 0.665 |
| walker |  | 5866 | 100 | c decl body at sds.c:427 |  |  | 0.665 |
| walker |  | 5970 | 104 | c decl body at sds.c:398 |  |  | 0.665 |
| ns | 6014 |  | 590 | sdsnewlen() body, part 1 (doc comment through TYPE_8 case) | 4.2 | 2.6 | 0.629 |
| ns | 6074 |  | 60 | sdsnewlen() body, part 2 (memcpy/null-term tail; TYPE_16/32/64 cases elided) | 4.3 | 2.6 | 0.624 |
| walker |  | 6410 | 440 | c decl names surface #1 in sds.c |  |  | 0.664 |
| walker |  | 6410 | 0 | c decl at sds.c:591 |  |  | 0.664 |
| walker |  | 6410 | 0 | c decl at sds.c:616 |  |  | 0.664 |
| walker |  | 6410 | 0 | c decl at sds.c:725 |  |  | 0.664 |
| walker |  | 6410 | 0 | c decl at sds.c:756 |  |  | 0.664 |
| walker |  | 6410 | 0 | c decl at sds.c:783 |  |  | 0.664 |
| walker |  | 6410 | 0 | c decl at sds.c:790 |  |  | 0.664 |
| walker |  | 6410 | 0 | c decl at sds.c:807 |  |  | 0.664 |
| walker |  | 6410 | 0 | c decl at sds.c:835 |  |  | 0.664 |
| walker |  | 6410 | 0 | c decl at sds.c:885 |  |  | 0.664 |
| walker |  | 6410 | 0 | c decl at sds.c:898 |  |  | 0.664 |
| walker |  | 6410 | 0 | c decl at sds.c:925 |  |  | 0.664 |
| walker |  | 6410 | 0 | c decl at sds.c:932 |  |  | 0.664 |
| walker |  | 6410 | 0 | c decl at sds.c:973 |  |  | 0.664 |
| walker |  | 6410 | 0 | c decl at sds.c:1092 |  |  | 0.664 |
| walker |  | 6410 | 0 | c decl at sds.c:1108 |  |  | 0.664 |
| walker |  | 6410 | 0 | c decl at sds.c:1120 |  |  | 0.664 |
| walker |  | 6410 | 0 | c decl at sds.c:1136 |  |  | 0.664 |
| walker |  | 6430 | 20 | c decl doc at sds.c:1120 |  |  | 0.664 |
| walker |  | 6451 | 21 | c decl doc at sds.c:783 |  |  | 0.664 |
| walker |  | 6472 | 21 | c decl doc at sds.c:790 |  |  | 0.664 |
| walker |  | 6498 | 26 | c decl doc at sds.c:885 |  |  | 0.664 |
| walker |  | 6537 | 39 | c decl body at sds.c:885 |  |  | 0.664 |
| walker |  | 6570 | 33 | c decl doc at sds.c:925 |  |  | 0.664 |
| walker |  | 6604 | 34 | c decl doc at sds.c:932 |  |  | 0.664 |
| walker |  | 6652 | 48 | c decl body at sds.c:783 |  |  | 0.664 |
| walker |  | 6700 | 48 | c decl body at sds.c:790 |  |  | 0.664 |
| walker |  | 6749 | 49 | c decl body at sds.c:925 |  |  | 0.664 |
| walker |  | 6787 | 38 | c decl doc at sds.c:1108 |  |  | 0.664 |
| ns | 6795 |  | 721 | sdsMakeRoomFor() body | 4.4 | 2.6 | 0.626 |
| walker |  | 6851 | 64 | c decl body at sds.c:591 |  |  | 0.626 |
| walker |  | 6955 | 104 | c decl body at sds.c:1108 |  |  | 0.626 |
| walker |  | 7063 | 108 | c decl body at sds.c:1120 |  |  | 0.626 |
| ns | 7115 |  | 320 | sdsIncrLen() body (TYPE_5/TYPE_8 cases shown; TYPE_16/32/64 elided) | 4.5 | 2.6 | 0.611 |
| walker |  | 7148 | 85 | c decl doc at sds.c:398 |  |  | 0.612 |
| walker |  | 7236 | 88 | c decl doc at sds.c:300 |  |  | 0.612 |
| walker |  | 7322 | 86 | c decl doc at sds.c:450 |  |  | 0.612 |
| ns | 7334 |  | 219 | sdsgrowzero() body | 4.6 | 2.6 | 0.605 |
| walker |  | 7420 | 98 | c decl doc at sds.c:256 |  |  | 0.605 |
| ns | 7539 |  | 205 | sdscatlen() body | 5.1 | 2.6 | 0.612 |
| walker |  | 7553 | 133 | c decl body at sds.c:1092 |  |  | 0.612 |
| walker |  | 7656 | 103 | c decl doc at sds.c:1136 |  |  | 0.612 |
| walker |  | 7791 | 135 | c decl body at sds.c:380 |  |  | 0.628 |
| ns | 7888 |  | 349 | sdstrim() body | 5.2 | 2.6 | 0.612 |
| walker |  | 7929 | 138 | c decl body at sds.c:807 |  |  | 0.612 |
| walker |  | 8039 | 110 | c decl doc at sds.c:898 |  |  | 0.612 |
| walker |  | 8150 | 111 | c decl doc at sds.c:204 |  |  | 0.614 |
| walker |  | 8307 | 157 | c decl body at sds.c:725 |  |  | 0.620 |
| ns | 8393 |  | 505 | sdsrange() body | 5.3 | 2.6 | 0.597 |
| walker |  | 8841 | 534 | c header banner in testhelp.h |  |  | 0.597 |
| walker |  | 8981 | 140 | c decl doc at sds.c:807 |  |  | 0.597 |
| ns | 9283 |  | 890 | sdscatfmt() body, part 1 (doc comment, setup, %s/%S and %i/%I cases) | 6.1 | 2.6 | 0.562 |
| ns | 9451 |  | 168 | sdscatfmt() body, part 2 (default case and null-term tail; %u/%U case elided) | 6.2 | 6.1 | 0.553 |
| walker |  | 9563 | 582 | c header banner in sdsalloc.h |  |  | 0.562 |
| walker |  | 9712 | 149 | c decl doc at sds.c:1092 |  |  | 0.562 |
| ns | 9886 |  | 435 | sdscatrepr() body | 7.1 | 2.6 | 0.552 |
| walker |  | 9889 | 177 | c decl doc at sds.c:725 |  |  | 0.574 |
| ns | 9997 |  | 111 | SDS_TEST_MAIN / main() harness wrapper | 8.1 | 2.6 | 0.568 |
