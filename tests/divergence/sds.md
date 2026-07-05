Score(3000)=0.509 I=0.841 C=0.308 ns_rows≤3K=16/40 (reached=9 partial=0 missing=7)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 31 | 31 | listing of '.' |  |  | 1.000 |
| ns | 31 |  | 31 | Root file listing | 1.1 |  | 1.000 |
| ns | 96 |  | 65 | README heading map, part 1 (top matter through 'Obtaining the string length') | 1.2 |  | 0.715 |
| walker |  | 140 | 109 | README headline in README.md |  |  | 0.730 |
| walker |  | 173 | 33 | c decl names surface in sdsalloc.h |  |  | 0.730 |
| ns | 183 |  | 87 | sds typedef, SDS_MAX_PREALLOC, SDS_NOINIT | 1.3 |  | 0.556 |
| ns | 281 |  | 98 | struct sdshdr5 layout | 1.4 |  | 0.480 |
| ns | 364 |  | 83 | struct sdshdr8 layout | 1.5 |  | 0.427 |
| ns | 447 |  | 83 | struct sdshdr16 layout | 1.6 |  | 0.388 |
| ns | 530 |  | 83 | struct sdshdr32 layout | 1.7 |  | 0.357 |
| walker |  | 572 | 399 | headings outline in README.md |  |  | 0.496 |
| ns | 613 |  | 83 | struct sdshdr64 layout | 1.8 |  | 0.461 |
| walker |  | 630 | 58 | c decl names surface in testhelp.h |  |  | 0.461 |
| ns | 794 |  | 181 | SDS_TYPE_* constants and SDS_HDR/SDS_HDR_VAR macros | 1.9 |  | 0.411 |
| ns | 1025 |  | 231 | README lede: what SDS is, v1-vs-v2 compatibility | 1.10 | 1.2 | 0.392 |
| walker |  | 1073 | 443 | c decl names surface in sds.h |  |  | 0.552 |
| walker |  | 1073 | 0 | c decl at sds.h:87 |  |  | 0.552 |
| walker |  | 1073 | 0 | c decl at sds.h:104 |  |  | 0.552 |
| walker |  | 1073 | 0 | c decl at sds.h:130 |  |  | 0.552 |
| walker |  | 1073 | 0 | c decl at sds.h:154 |  |  | 0.552 |
| walker |  | 1073 | 0 | c decl at sds.h:180 |  |  | 0.552 |
| walker |  | 1073 | 0 | c decl at sds.h:197 |  |  | 0.552 |
| walker |  | 1110 | 37 | c decl at sds.h:47 |  |  | 0.572 |
| walker |  | 1175 | 65 | c decl at sds.h:51 |  |  | 0.632 |
| walker |  | 1240 | 65 | c decl at sds.h:57 |  |  | 0.688 |
| walker |  | 1305 | 65 | c decl at sds.h:63 |  |  | 0.742 |
| ns | 1348 |  | 323 | README: how SDS strings work (header-prefix design + diagram) | 1.11 | 1.2 | 0.629 |
| walker |  | 1370 | 65 | c decl at sds.h:69 |  |  | 0.672 |
| walker |  | 1400 | 30 | c includes in sds.h |  |  | 0.717 |
| walker |  | 1420 | 20 | c decl doc at sds.h:180 |  |  | 0.717 |
| walker |  | 1461 | 41 | c decl doc at sds.h:47 |  |  | 0.752 |
| ns | 1806 |  | 458 | sdslen() / sdsavail() inline accessors | 1.12 |  | 0.619 |
| walker |  | 1912 | 451 | c decl names surface #1 in sds.h |  |  | 0.623 |
| walker |  | 1930 | 18 | c decl at sds.h:232 |  |  | 0.624 |
| ns | 1969 |  | 163 | sdssetlen() inline mutator (TYPE_16/32/64 cases elided) | 1.13 |  | 0.590 |
| ns | 2149 |  | 180 | sdsinclen() inline mutator (TYPE_16/32/64 cases elided) | 1.14 |  | 0.559 |
| walker |  | 2210 | 280 | c decl names surface #2 in sds.h |  |  | 0.564 |
| walker |  | 2210 | 0 | c decl at sds.h:256 |  |  | 0.564 |
| walker |  | 2210 | 0 | c decl at sds.h:266 |  |  | 0.564 |
| walker |  | 2222 | 12 | c decl doc at sds.h:256 |  |  | 0.564 |
| ns | 2570 |  | 421 | README: disadvantages of the single-allocation design | 1.15 | 1.2 | 0.519 |
| ns | 2687 |  | 117 | Makefile | 2.1 |  | 0.508 |
| walker |  | 2715 | 493 | c header banner in sds.h |  |  | 0.508 |
| walker |  | 2781 | 66 | c decl doc at sds.h:266 |  |  | 0.509 |
| walker |  | 2840 | 59 | c decl at testhelp.h:44 |  |  | 0.509 |
| walker |  | 3005 | 165 | c decl body at sds.h:87 |  |  | 0.540 |
| walker |  | 3170 | 165 | c decl body at sds.h:180 |  |  | 0.540 |
| ns | 3370 |  | 683 | sds.h public function declarations, part 1 (creation through low-level MakeRoomFor/IncrLen) | 2.2 |  | 0.595 |
| ns | 3566 |  | 196 | sds.h public function declarations, part 2 (RemoveFreeSpace/AllocSize/allocator wrappers/sdsTest) | 2.3 |  | 0.608 |
| walker |  | 3646 | 476 | c decl names surface in sds.c |  |  | 0.610 |
| walker |  | 3646 | 0 | c decl at sds.c:89 |  |  | 0.610 |
| walker |  | 3646 | 0 | c decl at sds.c:149 |  |  | 0.610 |
| walker |  | 3646 | 0 | c decl at sds.c:154 |  |  | 0.610 |
| walker |  | 3646 | 0 | c decl at sds.c:160 |  |  | 0.610 |
| walker |  | 3646 | 0 | c decl at sds.c:165 |  |  | 0.610 |
| walker |  | 3646 | 0 | c decl at sds.c:184 |  |  | 0.610 |
| walker |  | 3646 | 0 | c decl at sds.c:193 |  |  | 0.610 |
| walker |  | 3646 | 0 | c decl at sds.c:204 |  |  | 0.610 |
| walker |  | 3646 | 0 | c decl at sds.c:256 |  |  | 0.610 |
| walker |  | 3646 | 0 | c decl at sds.c:300 |  |  | 0.610 |
| walker |  | 3646 | 0 | c decl at sds.c:307 |  |  | 0.610 |
| walker |  | 3646 | 0 | c decl at sds.c:334 |  |  | 0.610 |
| walker |  | 3646 | 0 | c decl at sds.c:380 |  |  | 0.610 |
| walker |  | 3646 | 0 | c decl at sds.c:398 |  |  | 0.610 |
| walker |  | 3646 | 0 | c decl at sds.c:413 |  |  | 0.610 |
| walker |  | 3646 | 0 | c decl at sds.c:421 |  |  | 0.610 |
| walker |  | 3646 | 0 | c decl at sds.c:427 |  |  | 0.610 |
| walker |  | 3646 | 0 | c decl at sds.c:440 |  |  | 0.610 |
| walker |  | 3646 | 0 | c decl at sds.c:450 |  |  | 0.610 |
| walker |  | 3646 | 0 | c decl at sds.c:451 |  |  | 0.610 |
| walker |  | 3646 | 0 | c decl at sds.c:494 |  |  | 0.610 |
| walker |  | 3646 | 0 | c decl at sds.c:526 |  |  | 0.610 |
| walker |  | 3646 | 0 | c decl at sds.c:534 |  |  | 0.610 |
| walker |  | 3659 | 13 | c decl body at sds.c:149 |  |  | 0.610 |
| walker |  | 3671 | 12 | c decl doc at sds.c:160 |  |  | 0.610 |
| walker |  | 3688 | 17 | c decl body at sds.c:160 |  |  | 0.610 |
| ns | 3690 |  | 124 | sdsalloc.h allocator macros | 2.4 |  | 0.598 |
| walker |  | 3705 | 17 | c decl body at sds.c:413 |  |  | 0.598 |
| walker |  | 3722 | 17 | c decl body at sds.c:440 |  |  | 0.598 |
| walker |  | 3741 | 19 | c decl body at sds.c:307 |  |  | 0.598 |
| walker |  | 3760 | 19 | c decl body at sds.c:421 |  |  | 0.598 |
| walker |  | 3780 | 20 | c decl doc at sds.c:154 |  |  | 0.598 |
| walker |  | 3807 | 27 | c decl body at sds.c:184 |  |  | 0.598 |
| ns | 3819 |  | 129 | Changelog | 2.5 |  | 0.584 |
| walker |  | 3834 | 27 | c decl body at sds.c:193 |  |  | 0.584 |
| walker |  | 3855 | 21 | c decl doc at sds.c:494 |  |  | 0.584 |
| walker |  | 3877 | 22 | c decl doc at sds.c:534 |  |  | 0.584 |
| walker |  | 3900 | 23 | c decl doc at sds.c:165 |  |  | 0.584 |
| walker |  | 3932 | 32 | c decl body at sds.c:165 |  |  | 0.584 |
| walker |  | 3964 | 32 | c decl body at sds.c:300 |  |  | 0.584 |
| walker |  | 4001 | 37 | c decl body at sds.c:154 |  |  | 0.584 |
| walker |  | 4084 | 83 | c includes in sds.c |  |  | 0.584 |
| walker |  | 4118 | 34 | c decl doc at sds.c:307 |  |  | 0.584 |
| walker |  | 4165 | 47 | c decl body at sds.c:526 |  |  | 0.584 |
| walker |  | 4201 | 36 | c decl doc at sds.c:149 |  |  | 0.584 |
| walker |  | 4242 | 41 | c decl doc at sds.c:440 |  |  | 0.584 |
| walker |  | 4284 | 42 | c decl doc at sds.c:427 |  |  | 0.584 |
| walker |  | 4335 | 51 | c decl doc at sds.c:526 |  |  | 0.584 |
| ns | 4389 |  | 570 | sds.c function-definition location catalog | 2.6 |  | 0.560 |
| walker |  | 4434 | 99 | c decl at testhelp.h:48 |  |  | 0.560 |
| ns | 4493 |  | 104 | README heading map, part 2 (Destroying strings through String joining) | 3.1 |  | 0.571 |
| ns | 4608 |  | 115 | README heading map, part 3 (Error handling through Credits and license) | 3.2 |  | 0.581 |
| walker |  | 4639 | 205 | c decl body at sds.h:197 |  |  | 0.581 |
| ns | 4711 |  | 103 | README: error handling convention | 3.3 | 3.2 | 0.573 |
| ns | 4856 |  | 145 | README: interactions with heap checkers | 3.4 | 3.2 | 0.565 |
| walker |  | 4876 | 237 | c decl body at sds.h:130 |  |  | 0.595 |
| ns | 4941 |  | 85 | README: sharing SDS strings (refcounting advice, intro) | 3.5 | 3.2 | 0.589 |
| ns | 5100 |  | 159 | README: sharing SDS strings (refcounting advice, increment/decrement contract) | 3.6 |  | 0.583 |
| walker |  | 5120 | 244 | c decl body at sds.h:104 |  |  | 0.650 |
| walker |  | 5372 | 252 | c decl body at sds.h:154 |  |  | 0.680 |
| ns | 5424 |  | 324 | sdsHdrSize() / sdsReqType() bodies | 4.1 | 2.6 | 0.650 |
| walker |  | 5439 | 67 | c decl doc at sds.c:421 |  |  | 0.650 |
| walker |  | 5507 | 68 | c decl doc at sds.c:413 |  |  | 0.650 |
| walker |  | 5576 | 69 | c decl doc at sds.c:193 |  |  | 0.650 |
| walker |  | 5649 | 73 | c decl doc at sds.c:380 |  |  | 0.651 |
| walker |  | 5749 | 100 | c decl body at sds.c:427 |  |  | 0.651 |
| walker |  | 5853 | 104 | c decl body at sds.c:398 |  |  | 0.651 |
| ns | 6014 |  | 590 | sdsnewlen() body, part 1 (doc comment through TYPE_8 case) | 4.2 | 2.6 | 0.616 |
| ns | 6074 |  | 60 | sdsnewlen() body, part 2 (memcpy/null-term tail; TYPE_16/32/64 cases elided) | 4.3 | 2.6 | 0.610 |
| walker |  | 6293 | 440 | c decl names surface #1 in sds.c |  |  | 0.651 |
| walker |  | 6293 | 0 | c decl at sds.c:591 |  |  | 0.651 |
| walker |  | 6293 | 0 | c decl at sds.c:616 |  |  | 0.651 |
| walker |  | 6293 | 0 | c decl at sds.c:725 |  |  | 0.651 |
| walker |  | 6293 | 0 | c decl at sds.c:756 |  |  | 0.651 |
| walker |  | 6293 | 0 | c decl at sds.c:783 |  |  | 0.651 |
| walker |  | 6293 | 0 | c decl at sds.c:790 |  |  | 0.651 |
| walker |  | 6293 | 0 | c decl at sds.c:807 |  |  | 0.651 |
| walker |  | 6293 | 0 | c decl at sds.c:835 |  |  | 0.651 |
| walker |  | 6293 | 0 | c decl at sds.c:885 |  |  | 0.651 |
| walker |  | 6293 | 0 | c decl at sds.c:898 |  |  | 0.651 |
| walker |  | 6293 | 0 | c decl at sds.c:925 |  |  | 0.651 |
| walker |  | 6293 | 0 | c decl at sds.c:932 |  |  | 0.651 |
| walker |  | 6293 | 0 | c decl at sds.c:973 |  |  | 0.651 |
| walker |  | 6293 | 0 | c decl at sds.c:1092 |  |  | 0.651 |
| walker |  | 6293 | 0 | c decl at sds.c:1108 |  |  | 0.651 |
| walker |  | 6293 | 0 | c decl at sds.c:1120 |  |  | 0.651 |
| walker |  | 6293 | 0 | c decl at sds.c:1136 |  |  | 0.651 |
| walker |  | 6313 | 20 | c decl doc at sds.c:1120 |  |  | 0.651 |
| walker |  | 6334 | 21 | c decl doc at sds.c:783 |  |  | 0.651 |
| walker |  | 6355 | 21 | c decl doc at sds.c:790 |  |  | 0.651 |
| walker |  | 6381 | 26 | c decl doc at sds.c:885 |  |  | 0.651 |
| walker |  | 6420 | 39 | c decl body at sds.c:885 |  |  | 0.651 |
| walker |  | 6453 | 33 | c decl doc at sds.c:925 |  |  | 0.651 |
| walker |  | 6487 | 34 | c decl doc at sds.c:932 |  |  | 0.651 |
| walker |  | 6535 | 48 | c decl body at sds.c:783 |  |  | 0.651 |
| walker |  | 6583 | 48 | c decl body at sds.c:790 |  |  | 0.651 |
| walker |  | 6632 | 49 | c decl body at sds.c:925 |  |  | 0.651 |
| walker |  | 6670 | 38 | c decl doc at sds.c:1108 |  |  | 0.651 |
| walker |  | 6734 | 64 | c decl body at sds.c:591 |  |  | 0.651 |
| ns | 6795 |  | 721 | sdsMakeRoomFor() body | 4.4 | 2.6 | 0.614 |
| walker |  | 6838 | 104 | c decl body at sds.c:1108 |  |  | 0.614 |
| walker |  | 6946 | 108 | c decl body at sds.c:1120 |  |  | 0.614 |
| walker |  | 7031 | 85 | c decl doc at sds.c:398 |  |  | 0.615 |
| ns | 7115 |  | 320 | sdsIncrLen() body (TYPE_5/TYPE_8 cases shown; TYPE_16/32/64 elided) | 4.5 | 2.6 | 0.600 |
| walker |  | 7119 | 88 | c decl doc at sds.c:300 |  |  | 0.600 |
| walker |  | 7205 | 86 | c decl doc at sds.c:450 |  |  | 0.600 |
| walker |  | 7303 | 98 | c decl doc at sds.c:256 |  |  | 0.600 |
| ns | 7334 |  | 219 | sdsgrowzero() body | 4.6 | 2.6 | 0.593 |
| walker |  | 7436 | 133 | c decl body at sds.c:1092 |  |  | 0.593 |
| ns | 7539 |  | 205 | sdscatlen() body | 5.1 | 2.6 | 0.601 |
| walker |  | 7571 | 135 | c decl body at sds.c:380 |  |  | 0.617 |
| walker |  | 7709 | 138 | c decl body at sds.c:807 |  |  | 0.617 |
| walker |  | 7812 | 103 | c decl doc at sds.c:1136 |  |  | 0.617 |
| ns | 7888 |  | 349 | sdstrim() body | 5.2 | 2.6 | 0.601 |
| walker |  | 7922 | 110 | c decl doc at sds.c:898 |  |  | 0.601 |
| walker |  | 8033 | 111 | c decl doc at sds.c:204 |  |  | 0.603 |
| walker |  | 8190 | 157 | c decl body at sds.c:725 |  |  | 0.609 |
| walker |  | 8330 | 140 | c decl doc at sds.c:807 |  |  | 0.609 |
| ns | 8393 |  | 505 | sdsrange() body | 5.3 | 2.6 | 0.587 |
| walker |  | 8864 | 534 | c header banner in testhelp.h |  |  | 0.587 |
| walker |  | 9013 | 149 | c decl doc at sds.c:1092 |  |  | 0.587 |
| ns | 9283 |  | 890 | sdscatfmt() body, part 1 (doc comment, setup, %s/%S and %i/%I cases) | 6.1 | 2.6 | 0.552 |
| ns | 9451 |  | 168 | sdscatfmt() body, part 2 (default case and null-term tail; %u/%U case elided) | 6.2 | 6.1 | 0.543 |
| walker |  | 9595 | 582 | c header banner in sdsalloc.h |  |  | 0.553 |
| ns | 9886 |  | 435 | sdscatrepr() body | 7.1 | 2.6 | 0.543 |
| ns | 9997 |  | 111 | SDS_TEST_MAIN / main() harness wrapper | 8.1 | 2.6 | 0.538 |
