Score(3000)=0.574 I=0.846 C=0.390 ns_rows≤3K=16/40 (reached=10 partial=0 missing=6)

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
| ns | 794 |  | 181 | SDS_TYPE_* constants and SDS_HDR/SDS_HDR_VAR macros | 1.9 |  | 0.414 |
| walker |  | 855 | 166 | README.md section #0 |  |  | 0.424 |
| walker |  | 913 | 58 | c decl names surface in testhelp.h |  |  | 0.424 |
| ns | 1025 |  | 231 | README lede: what SDS is, v1-vs-v2 compatibility | 1.10 | 1.2 | 0.529 |
| ns | 1348 |  | 323 | README: how SDS strings work (header-prefix design + diagram) | 1.11 | 1.2 | 0.453 |
| walker |  | 1356 | 443 | c decl names surface in sds.h |  |  | 0.572 |
| walker |  | 1356 | 0 | c decl at sds.h:87 |  |  | 0.572 |
| walker |  | 1356 | 0 | c decl at sds.h:104 |  |  | 0.572 |
| walker |  | 1356 | 0 | c decl at sds.h:130 |  |  | 0.572 |
| walker |  | 1356 | 0 | c decl at sds.h:154 |  |  | 0.572 |
| walker |  | 1356 | 0 | c decl at sds.h:180 |  |  | 0.572 |
| walker |  | 1356 | 0 | c decl at sds.h:197 |  |  | 0.572 |
| walker |  | 1393 | 37 | c decl at sds.h:47 |  |  | 0.588 |
| walker |  | 1458 | 65 | c decl at sds.h:51 |  |  | 0.636 |
| walker |  | 1523 | 65 | c decl at sds.h:57 |  |  | 0.681 |
| walker |  | 1588 | 65 | c decl at sds.h:63 |  |  | 0.724 |
| walker |  | 1653 | 65 | c decl at sds.h:69 |  |  | 0.765 |
| walker |  | 1683 | 30 | c includes in sds.h |  |  | 0.809 |
| walker |  | 1703 | 20 | c decl doc at sds.h:180 |  |  | 0.809 |
| ns | 1806 |  | 458 | sdslen() / sdsavail() inline accessors | 1.12 |  | 0.666 |
| ns | 1969 |  | 163 | sdssetlen() inline mutator (TYPE_16/32/64 cases elided) | 1.13 |  | 0.630 |
| ns | 2149 |  | 180 | sdsinclen() inline mutator (TYPE_16/32/64 cases elided) | 1.14 |  | 0.597 |
| walker |  | 2154 | 451 | c decl names surface #1 in sds.h |  |  | 0.601 |
| walker |  | 2172 | 18 | c decl at sds.h:232 |  |  | 0.602 |
| walker |  | 2452 | 280 | c decl names surface #2 in sds.h |  |  | 0.607 |
| walker |  | 2452 | 0 | c decl at sds.h:256 |  |  | 0.607 |
| walker |  | 2452 | 0 | c decl at sds.h:266 |  |  | 0.607 |
| walker |  | 2464 | 12 | c decl doc at sds.h:256 |  |  | 0.607 |
| ns | 2570 |  | 421 | README: disadvantages of the single-allocation design | 1.15 | 1.2 | 0.558 |
| ns | 2687 |  | 117 | Makefile | 2.1 |  | 0.574 |
| walker |  | 2957 | 493 | c header banner in sds.h |  |  | 0.574 |
| walker |  | 3016 | 59 | c decl at testhelp.h:44 |  |  | 0.574 |
| walker |  | 3082 | 66 | c decl doc at sds.h:266 |  |  | 0.576 |
| walker |  | 3123 | 41 | c decl doc at sds.h:47 |  |  | 0.598 |
| walker |  | 3222 | 99 | c decl at testhelp.h:48 |  |  | 0.598 |
| ns | 3370 |  | 683 | sds.h public function declarations, part 1 (creation through low-level MakeRoomFor/IncrLen) | 2.2 |  | 0.642 |
| ns | 3566 |  | 196 | sds.h public function declarations, part 2 (RemoveFreeSpace/AllocSize/allocator wrappers/sdsTest) | 2.3 |  | 0.652 |
| ns | 3690 |  | 124 | sdsalloc.h allocator macros | 2.4 |  | 0.640 |
| walker |  | 3698 | 476 | c decl names surface in sds.c |  |  | 0.642 |
| walker |  | 3698 | 0 | c decl at sds.c:89 |  |  | 0.642 |
| walker |  | 3698 | 0 | c decl at sds.c:149 |  |  | 0.642 |
| walker |  | 3698 | 0 | c decl at sds.c:154 |  |  | 0.642 |
| walker |  | 3698 | 0 | c decl at sds.c:160 |  |  | 0.642 |
| walker |  | 3698 | 0 | c decl at sds.c:165 |  |  | 0.642 |
| walker |  | 3698 | 0 | c decl at sds.c:184 |  |  | 0.642 |
| walker |  | 3698 | 0 | c decl at sds.c:193 |  |  | 0.642 |
| walker |  | 3698 | 0 | c decl at sds.c:204 |  |  | 0.642 |
| walker |  | 3698 | 0 | c decl at sds.c:256 |  |  | 0.642 |
| walker |  | 3698 | 0 | c decl at sds.c:300 |  |  | 0.642 |
| walker |  | 3698 | 0 | c decl at sds.c:307 |  |  | 0.642 |
| walker |  | 3698 | 0 | c decl at sds.c:334 |  |  | 0.642 |
| walker |  | 3698 | 0 | c decl at sds.c:380 |  |  | 0.642 |
| walker |  | 3698 | 0 | c decl at sds.c:398 |  |  | 0.642 |
| walker |  | 3698 | 0 | c decl at sds.c:413 |  |  | 0.642 |
| walker |  | 3698 | 0 | c decl at sds.c:421 |  |  | 0.642 |
| walker |  | 3698 | 0 | c decl at sds.c:427 |  |  | 0.642 |
| walker |  | 3698 | 0 | c decl at sds.c:440 |  |  | 0.642 |
| walker |  | 3698 | 0 | c decl at sds.c:450 |  |  | 0.642 |
| walker |  | 3698 | 0 | c decl at sds.c:451 |  |  | 0.642 |
| walker |  | 3698 | 0 | c decl at sds.c:494 |  |  | 0.642 |
| walker |  | 3698 | 0 | c decl at sds.c:526 |  |  | 0.642 |
| walker |  | 3698 | 0 | c decl at sds.c:534 |  |  | 0.642 |
| walker |  | 3711 | 13 | c decl body at sds.c:149 |  |  | 0.642 |
| walker |  | 3723 | 12 | c decl doc at sds.c:160 |  |  | 0.642 |
| walker |  | 3740 | 17 | c decl body at sds.c:160 |  |  | 0.642 |
| walker |  | 3757 | 17 | c decl body at sds.c:413 |  |  | 0.642 |
| walker |  | 3774 | 17 | c decl body at sds.c:440 |  |  | 0.642 |
| walker |  | 3793 | 19 | c decl body at sds.c:307 |  |  | 0.642 |
| walker |  | 3812 | 19 | c decl body at sds.c:421 |  |  | 0.642 |
| ns | 3819 |  | 129 | Changelog | 2.5 |  | 0.627 |
| walker |  | 3832 | 20 | c decl doc at sds.c:154 |  |  | 0.627 |
| walker |  | 3859 | 27 | c decl body at sds.c:184 |  |  | 0.627 |
| walker |  | 3886 | 27 | c decl body at sds.c:193 |  |  | 0.627 |
| walker |  | 3907 | 21 | c decl doc at sds.c:494 |  |  | 0.627 |
| walker |  | 3929 | 22 | c decl doc at sds.c:534 |  |  | 0.627 |
| walker |  | 3952 | 23 | c decl doc at sds.c:165 |  |  | 0.627 |
| walker |  | 3984 | 32 | c decl body at sds.c:165 |  |  | 0.627 |
| walker |  | 4016 | 32 | c decl body at sds.c:300 |  |  | 0.627 |
| walker |  | 4053 | 37 | c decl body at sds.c:154 |  |  | 0.627 |
| walker |  | 4087 | 34 | c decl doc at sds.c:307 |  |  | 0.627 |
| walker |  | 4123 | 36 | c decl doc at sds.c:149 |  |  | 0.627 |
| walker |  | 4170 | 47 | c decl body at sds.c:526 |  |  | 0.627 |
| walker |  | 4253 | 83 | c includes in sds.c |  |  | 0.627 |
| walker |  | 4294 | 41 | c decl doc at sds.c:440 |  |  | 0.627 |
| walker |  | 4336 | 42 | c decl doc at sds.c:427 |  |  | 0.627 |
| ns | 4389 |  | 570 | sds.c function-definition location catalog | 2.6 |  | 0.598 |
| ns | 4493 |  | 104 | README heading map, part 2 (Destroying strings through String joining) | 3.1 |  | 0.608 |
| walker |  | 4501 | 165 | c decl body at sds.h:87 |  |  | 0.625 |
| ns | 4608 |  | 115 | README heading map, part 3 (Error handling through Credits and license) | 3.2 |  | 0.634 |
| walker |  | 4666 | 165 | c decl body at sds.h:180 |  |  | 0.634 |
| ns | 4711 |  | 103 | README: error handling convention | 3.3 | 3.2 | 0.625 |
| walker |  | 4717 | 51 | c decl doc at sds.c:526 |  |  | 0.625 |
| ns | 4856 |  | 145 | README: interactions with heap checkers | 3.4 | 3.2 | 0.617 |
| walker |  | 4922 | 205 | c decl body at sds.h:197 |  |  | 0.617 |
| ns | 4941 |  | 85 | README: sharing SDS strings (refcounting advice, intro) | 3.5 | 3.2 | 0.610 |
| walker |  | 4989 | 67 | c decl doc at sds.c:421 |  |  | 0.610 |
| ns | 5100 |  | 159 | README: sharing SDS strings (refcounting advice, increment/decrement contract) | 3.6 |  | 0.604 |
| walker |  | 5226 | 237 | c decl body at sds.h:130 |  |  | 0.633 |
| walker |  | 5294 | 68 | c decl doc at sds.c:413 |  |  | 0.633 |
| walker |  | 5363 | 69 | c decl doc at sds.c:193 |  |  | 0.633 |
| ns | 5424 |  | 324 | sdsHdrSize() / sdsReqType() bodies | 4.1 | 2.6 | 0.605 |
| walker |  | 5607 | 244 | c decl body at sds.h:104 |  |  | 0.668 |
| walker |  | 5859 | 252 | c decl body at sds.h:154 |  |  | 0.696 |
| walker |  | 5932 | 73 | c decl doc at sds.c:380 |  |  | 0.696 |
| ns | 6014 |  | 590 | sdsnewlen() body, part 1 (doc comment through TYPE_8 case) | 4.2 | 2.6 | 0.658 |
| walker |  | 6032 | 100 | c decl body at sds.c:427 |  |  | 0.658 |
| ns | 6074 |  | 60 | sdsnewlen() body, part 2 (memcpy/null-term tail; TYPE_16/32/64 cases elided) | 4.3 | 2.6 | 0.653 |
| walker |  | 6136 | 104 | c decl body at sds.c:398 |  |  | 0.653 |
| walker |  | 6576 | 440 | c decl names surface #1 in sds.c |  |  | 0.693 |
| walker |  | 6576 | 0 | c decl at sds.c:591 |  |  | 0.693 |
| walker |  | 6576 | 0 | c decl at sds.c:616 |  |  | 0.693 |
| walker |  | 6576 | 0 | c decl at sds.c:725 |  |  | 0.693 |
| walker |  | 6576 | 0 | c decl at sds.c:756 |  |  | 0.693 |
| walker |  | 6576 | 0 | c decl at sds.c:783 |  |  | 0.693 |
| walker |  | 6576 | 0 | c decl at sds.c:790 |  |  | 0.693 |
| walker |  | 6576 | 0 | c decl at sds.c:807 |  |  | 0.693 |
| walker |  | 6576 | 0 | c decl at sds.c:835 |  |  | 0.693 |
| walker |  | 6576 | 0 | c decl at sds.c:885 |  |  | 0.693 |
| walker |  | 6576 | 0 | c decl at sds.c:898 |  |  | 0.693 |
| walker |  | 6576 | 0 | c decl at sds.c:925 |  |  | 0.693 |
| walker |  | 6576 | 0 | c decl at sds.c:932 |  |  | 0.693 |
| walker |  | 6576 | 0 | c decl at sds.c:973 |  |  | 0.693 |
| walker |  | 6576 | 0 | c decl at sds.c:1092 |  |  | 0.693 |
| walker |  | 6576 | 0 | c decl at sds.c:1108 |  |  | 0.693 |
| walker |  | 6576 | 0 | c decl at sds.c:1120 |  |  | 0.693 |
| walker |  | 6576 | 0 | c decl at sds.c:1136 |  |  | 0.693 |
| walker |  | 6596 | 20 | c decl doc at sds.c:1120 |  |  | 0.693 |
| walker |  | 6617 | 21 | c decl doc at sds.c:783 |  |  | 0.693 |
| walker |  | 6638 | 21 | c decl doc at sds.c:790 |  |  | 0.693 |
| walker |  | 6664 | 26 | c decl doc at sds.c:885 |  |  | 0.693 |
| walker |  | 6703 | 39 | c decl body at sds.c:885 |  |  | 0.693 |
| walker |  | 6736 | 33 | c decl doc at sds.c:925 |  |  | 0.693 |
| walker |  | 6770 | 34 | c decl doc at sds.c:932 |  |  | 0.693 |
| ns | 6795 |  | 721 | sdsMakeRoomFor() body | 4.4 | 2.6 | 0.653 |
| walker |  | 6818 | 48 | c decl body at sds.c:783 |  |  | 0.653 |
| walker |  | 6866 | 48 | c decl body at sds.c:790 |  |  | 0.653 |
| walker |  | 6915 | 49 | c decl body at sds.c:925 |  |  | 0.653 |
| walker |  | 6953 | 38 | c decl doc at sds.c:1108 |  |  | 0.653 |
| walker |  | 7017 | 64 | c decl body at sds.c:591 |  |  | 0.653 |
| ns | 7115 |  | 320 | sdsIncrLen() body (TYPE_5/TYPE_8 cases shown; TYPE_16/32/64 elided) | 4.5 | 2.6 | 0.638 |
| walker |  | 7121 | 104 | c decl body at sds.c:1108 |  |  | 0.638 |
| walker |  | 7229 | 108 | c decl body at sds.c:1120 |  |  | 0.638 |
| walker |  | 7314 | 85 | c decl doc at sds.c:398 |  |  | 0.639 |
| ns | 7334 |  | 219 | sdsgrowzero() body | 4.6 | 2.6 | 0.631 |
| ns | 7539 |  | 205 | sdscatlen() body | 5.1 | 2.6 | 0.638 |
| walker |  | 7671 | 357 | README.md section #1 |  |  | 0.679 |
| ns | 7888 |  | 349 | sdstrim() body | 5.2 | 2.6 | 0.661 |
| walker |  | 8023 | 352 | README.md section #2 |  |  | 0.675 |
| walker |  | 8281 | 258 | README.md section #3 |  |  | 0.698 |
| ns | 8393 |  | 505 | sdsrange() body | 5.3 | 2.6 | 0.672 |
| walker |  | 8636 | 355 | README.md section #4 |  |  | 0.672 |
| walker |  | 8929 | 293 | README.md section #5 |  |  | 0.672 |
| walker |  | 9261 | 332 | README.md section #6 |  |  | 0.672 |
| ns | 9283 |  | 890 | sdscatfmt() body, part 1 (doc comment, setup, %s/%S and %i/%I cases) | 6.1 | 2.6 | 0.632 |
| ns | 9451 |  | 168 | sdscatfmt() body, part 2 (default case and null-term tail; %u/%U case elided) | 6.2 | 6.1 | 0.622 |
| walker |  | 9614 | 353 | README.md section #7 |  |  | 0.622 |
| walker |  | 9886 | 272 | README.md section #8 |  |  | 0.608 |
| ns | 9886 |  | 435 | sdscatrepr() body | 7.1 | 2.6 | 0.608 |
| ns | 9997 |  | 111 | SDS_TEST_MAIN / main() harness wrapper | 8.1 | 2.6 | 0.602 |
