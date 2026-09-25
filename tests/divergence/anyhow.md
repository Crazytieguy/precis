Score(3000)=0.703 I=0.901 C=0.548 ns_rows≤3K=20/63 grid(1000/1442/2080/3000/4327/6240/9000)=0.752/0.649/0.695/0.703/0.690/0.573/0.575

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 34 | 34 | listing of '.' |  |  | 0.000 |
| walker |  | 83 | 49 | listing of 'src' |  |  | 0.000 |
| ns | 85 |  | 85 | Crate identity — name, version, description | 1.1 |  | 0.000 |
| walker |  | 110 | 27 | [features] in Cargo.toml |  |  | 0.000 |
| walker |  | 118 | 8 | listing of '.github' |  |  | 0.000 |
| ns | 119 |  | 34 | Repository root listing | 1.2 |  | 0.523 |
| walker |  | 122 | 4 | listing of '.github/workflows' |  |  | 0.523 |
| ns | 168 |  | 49 | src/ module inventory | 1.3 |  | 0.545 |
| walker |  | 189 | 67 | README headline in README.md |  |  | 0.549 |
| ns | 259 |  | 91 | What `anyhow::Error` is — a Box<dyn Error> that must be Send + Sync | 1.4 |  | 0.486 |
| walker |  | 279 | 90 | pub-item names surface in src/lib.rs |  |  | 0.489 |
| walker |  | 279 | 0 | pub item at src/lib.rs:468 |  |  | 0.489 |
| walker |  | 292 | 13 | pub item at src/lib.rs:650 |  |  | 0.490 |
| walker |  | 318 | 26 | pub item at src/lib.rs:390 |  |  | 0.493 |
| walker |  | 328 | 10 | pub item body at src/lib.rs:650 body 651 |  |  | 0.494 |
| ns | 333 |  | 74 | …and the other two guarantees: always a backtrace, one word wide | 1.5 | 1.4 | 0.454 |
| walker |  | 377 | 49 | pub item at src/lib.rs:415 |  |  | 0.461 |
| ns | 417 |  | 84 | Cargo.toml package tail — MSRV, edition, license, links | 1.6 |  | 0.422 |
| walker |  | 421 | 44 | headings outline in README.md |  |  | 0.429 |
| ns | 507 |  | 90 | tests/ inventory | 1.7 |  | 0.349 |
| walker |  | 581 | 160 | pub item at src/lib.rs:616 |  |  | 0.358 |
| ns | 645 |  | 138 | Cargo features — std default, optional backtrace | 1.8 |  | 0.330 |
| walker |  | 746 | 165 | [package] in Cargo.toml |  |  | 0.646 |
| ns | 793 |  | 148 | README lede, install snippet, and section map | 1.9 |  | 0.637 |
| walker |  | 820 | 74 | README.md section #0 |  |  | 0.678 |
| walker |  | 930 | 110 | [dependencies] in Cargo.toml |  |  | 0.752 |
| ns | 1009 |  | 216 | lib.rs crate attributes and module declarations | 1.10 |  | 0.652 |
| walker |  | 1204 | 274 | crate-doc lede in src/lib.rs |  |  | 0.652 |
| ns | 1244 |  | 235 | Complete roster of exported items in lib.rs | 2.1 |  | 0.649 |
| walker |  | 1326 | 122 | macro_export names across src |  |  | 0.649 |
| ns | 1452 |  | 208 | impl Error — roster of every public method | 2.2 |  | 0.606 |
| walker |  | 1590 | 264 | mod/use plumbing in src/lib.rs |  |  | 0.651 |
| ns | 1612 |  | 160 | Context trait — both method signatures with bounds | 2.3 | 2.1 | 0.670 |
| walker |  | 1680 | 90 | listing of 'tests' |  |  | 0.763 |
| ns | 1778 |  | 166 | Display representations — `{}` and `{:#}`, with sample output | 2.4 | 1.4 | 0.741 |
| walker |  | 1805 | 125 | manifest config in Cargo.toml |  |  | 0.741 |
| ns | 1915 |  | 137 | Debug representations — `{:?}` and `{:#?}` | 2.5 | 2.4 | 0.715 |
| walker |  | 1971 | 166 | impl method sigs in src/context.rs |  |  | 0.716 |
| ns | 2031 |  | 116 | Context trait doc — sealed, and outermost-first cause printing | 2.6 | 2.3 | 0.695 |
| ns | 2212 |  | 181 | Context + downcasting — the guarantee, in both directions | 2.7 | 2.6 | 0.665 |
| ns | 2360 |  | 148 | Result alias and Ok() helper semantics | 2.8 | 2.1 | 0.646 |
| walker |  | 2372 | 401 | crate attributes in src/lib.rs |  |  | 0.684 |
| walker |  | 2406 | 34 | pub item at src/backtrace.rs:8 |  |  | 0.684 |
| walker |  | 2410 | 4 | listing of 'tests/common' |  |  | 0.684 |
| walker |  | 2414 | 4 | listing of 'tests/drop' |  |  | 0.684 |
| walker |  | 2455 | 41 | pub-item doc lede at src/lib.rs:415 |  |  | 0.684 |
| walker |  | 2520 | 65 | pub-item doc lede at src/lib.rs:616 |  |  | 0.690 |
| ns | 2536 |  | 176 | no_std support contract | 2.9 |  | 0.672 |
| walker |  | 2658 | 138 | macro_export body at src/macros.rs:58 |  |  | 0.674 |
| ns | 2768 |  | 232 | Trait impls on Error — complete list | 2.10 | 2.2 | 0.650 |
| walker |  | 2832 | 174 | pub-item doc lede at src/lib.rs:390 |  |  | 0.701 |
| walker |  | 2914 | 82 | listing of 'tests/ui' |  |  | 0.703 |
| ns | 3002 |  | 234 | What bail! and ensure! mean | 3.1 |  | 0.681 |
| walker |  | 3136 | 222 | macro_export body at src/macros.rs:204 |  |  | 0.685 |
| walker |  | 3173 | 37 | impl method sigs in src/ensure.rs |  |  | 0.685 |
| walker |  | 3173 | 0 | impl method at src/ensure.rs:41 |  |  | 0.685 |
| walker |  | 3173 | 0 | impl method at src/ensure.rs:48 |  |  | 0.685 |
| ns | 3183 |  | 181 | What anyhow! constructs | 3.2 | 3.1 | 0.667 |
| walker |  | 3212 | 39 | pub-item names surface in src/ensure.rs |  |  | 0.667 |
| walker |  | 3245 | 33 | pub-item names surface in src/chain.rs |  |  | 0.667 |
| walker |  | 3280 | 35 | pub item at src/chain.rs:11 |  |  | 0.667 |
| ns | 3438 |  | 255 | Every exported macro_rules!, with its export attributes | 3.3 | 3.1 | 0.643 |
| walker |  | 3506 | 226 | pub-item doc lede at src/lib.rs:468 |  |  | 0.652 |
| ns | 3660 |  | 222 | anyhow! expansion — the three arms | 3.4 | 3.3 | 0.671 |
| ns | 3796 |  | 136 | bail! body | 3.5 | 3.3 | 0.679 |
| ns | 3957 |  | 161 | ensure! expansion — the fuel-limited parser call | 3.6 | 3.3 | 0.663 |
| ns | 4126 |  | 169 | kind.rs — why autoref tagged dispatch | 3.7 | 3.4 | 0.652 |
| walker |  | 4195 | 689 | impl method sigs in src/error.rs |  |  | 0.690 |
| walker |  | 4195 | 0 | impl method at src/error.rs:958 |  |  | 0.690 |
| walker |  | 4195 | 0 | impl method at src/error.rs:967 |  |  | 0.690 |
| walker |  | 4195 | 0 | impl method at src/error.rs:1010 |  |  | 0.690 |
| walker |  | 4229 | 34 | pub-item names surface in src/error.rs |  |  | 0.690 |
| walker |  | 4260 | 31 | pub item at src/error.rs:952 |  |  | 0.690 |
| ns | 4375 |  | 249 | kind.rs — the three kinds and their new() constructors | 3.8 | 3.7 | 0.672 |
| walker |  | 4544 | 284 | pub-item doc lede at src/lib.rs:650 |  |  | 0.686 |
| walker |  | 4584 | 40 | impl method sigs in src/chain.rs |  |  | 0.687 |
| walker |  | 4584 | 0 | impl method at src/chain.rs:28 |  |  | 0.687 |
| ns | 4627 |  | 252 | ensure.rs runtime side — BothDebug / NotBothDebug dispatch | 3.9 | 3.6 | 0.667 |
| ns | 4798 |  | 171 | __parse_ensure! extent, first rule, and catch-all | 3.10 | 3.6 | 0.654 |
| ns | 4995 |  | 197 | lib.rs __private — the macro-facing surface | 3.11 | 3.3 | 0.641 |
| walker |  | 5030 | 446 | impl method sigs in src/ptr.rs |  |  | 0.641 |
| walker |  | 5030 | 0 | impl method at src/ptr.rs:32 |  |  | 0.641 |
| walker |  | 5030 | 0 | impl method at src/ptr.rs:38 |  |  | 0.641 |
| walker |  | 5030 | 0 | impl method at src/ptr.rs:44 |  |  | 0.641 |
| walker |  | 5030 | 0 | impl method at src/ptr.rs:48 |  |  | 0.641 |
| walker |  | 5030 | 0 | impl method at src/ptr.rs:55 |  |  | 0.641 |
| walker |  | 5030 | 0 | impl method at src/ptr.rs:87 |  |  | 0.641 |
| walker |  | 5030 | 0 | impl method at src/ptr.rs:94 |  |  | 0.641 |
| walker |  | 5030 | 0 | impl method at src/ptr.rs:101 |  |  | 0.641 |
| walker |  | 5030 | 0 | impl method at src/ptr.rs:108 |  |  | 0.641 |
| walker |  | 5030 | 0 | impl method at src/ptr.rs:115 |  |  | 0.641 |
| walker |  | 5030 | 0 | impl method at src/ptr.rs:119 |  |  | 0.641 |
| walker |  | 5030 | 0 | impl method at src/ptr.rs:148 |  |  | 0.641 |
| walker |  | 5030 | 0 | impl method at src/ptr.rs:155 |  |  | 0.641 |
| walker |  | 5030 | 0 | impl method at src/ptr.rs:162 |  |  | 0.641 |
| walker |  | 5030 | 0 | impl method at src/ptr.rs:169 |  |  | 0.641 |
| walker |  | 5030 | 0 | impl method at src/ptr.rs:175 |  |  | 0.641 |
| ns | 5156 |  | 161 | test_fmt.rs — the f/g/h fixture and expected `{:#}` strings | 4.1 | 2.4 | 0.632 |
| ns | 5295 |  | 139 | test_fmt.rs — expected `{:?}` strings, including numbered causes | 4.2 | 4.1 | 0.618 |
| walker |  | 5403 | 373 | private-fn names surface in src/error.rs |  |  | 0.619 |
| ns | 5454 |  | 159 | test_repr.rs — size, null-pointer-optimization, autotrait assertions | 4.3 | 1.5 | 0.607 |
| ns | 5633 |  | 179 | test_downcast.rs — what a bail!'d error downcasts to | 4.4 | 3.4 | 0.592 |
| ns | 5807 |  | 174 | test_chain.rs — cause iteration order | 4.5 |  | 0.583 |
| walker |  | 5931 | 528 | crate-doc body in src/lib.rs |  |  | 0.583 |
| ns | 6002 |  | 195 | test_context.rs — downcast_ref through a three-level context chain | 4.6 | 2.7 | 0.573 |
| walker |  | 6068 | 137 | README.md section #8 |  |  | 0.573 |
| walker |  | 6118 | 50 | pub-item names surface in src/ptr.rs |  |  | 0.573 |
| walker |  | 6131 | 13 | pub item at src/ptr.rs:181 |  |  | 0.573 |
| walker |  | 6178 | 47 | pub item at src/ptr.rs:6 |  |  | 0.573 |
| walker |  | 6190 | 12 | impl method at src/error.rs:675 |  |  | 0.573 |
| walker |  | 6251 | 61 | pub item at src/ptr.rs:64 |  |  | 0.574 |
| ns | 6252 |  | 250 | ErrorVTable — the hand-rolled vtable layout | 5.1 |  | 0.565 |
| walker |  | 6274 | 23 | pub-item names surface in tests/drop/mod.rs |  |  | 0.565 |
| walker |  | 6298 | 24 | pub item at tests/drop/mod.rs:26 |  |  | 0.565 |
| walker |  | 6323 | 25 | pub item at tests/drop/mod.rs:9 |  |  | 0.565 |
| walker |  | 6385 | 62 | pub item at src/ptr.rs:125 |  |  | 0.566 |
| walker |  | 6439 | 54 | pub-item names surface in src/wrapper.rs |  |  | 0.566 |
| walker |  | 6448 | 9 | pub item at src/wrapper.rs:11 |  |  | 0.566 |
| walker |  | 6457 | 9 | pub item at src/wrapper.rs:34 |  |  | 0.566 |
| ns | 6483 |  | 231 | ErrorImpl, ContextError, and the vtable reader | 5.2 | 5.1 | 0.559 |
| walker |  | 6486 | 29 | pub item at src/wrapper.rs:58 |  |  | 0.559 |
| walker |  | 6499 | 13 | impl method at src/error.rs:1002 |  |  | 0.559 |
| walker |  | 6512 | 13 | impl method body at src/ptr.rs:115 body 116 |  |  | 0.559 |
| walker |  | 6584 | 72 | impl method sigs in src/fmt.rs |  |  | 0.559 |
| walker |  | 6584 | 0 | impl method at src/fmt.rs:7 |  |  | 0.559 |
| walker |  | 6584 | 0 | impl method at src/fmt.rs:20 |  |  | 0.559 |
| ns | 6615 |  | 132 | Error::construct — allocation and type erasure | 5.3 | 5.2 | 0.551 |
| ns | 6718 |  | 103 | impl Error — roster of the non-public fns | 5.4 | 5.3 | 0.556 |
| walker |  | 6802 | 218 | README.md section #7 |  |  | 0.556 |
| ns | 6889 |  | 171 | Vtable function roster in error.rs | 5.5 | 5.1 | 0.566 |
| walker |  | 6901 | 99 | pub item at src/chain.rs:16 |  |  | 0.567 |
| ns | 7040 |  | 151 | ErrorImpl methods and trait impls | 5.6 | 5.2 | 0.567 |
| walker |  | 7270 | 369 | crate-doc tail at src/lib.rs:95 |  |  | 0.567 |
| ns | 7275 |  | 235 | ptr.rs — Own / Ref / Mut / CastTo declarations | 5.7 |  | 0.586 |
| ns | 7440 |  | 165 | chain.rs — Chain and ChainState | 6.1 |  | 0.593 |
| ns | 7557 |  | 117 | chain.rs — every trait impl on Chain | 6.2 | 6.1 | 0.588 |
| walker |  | 7651 | 381 | crate-doc tail at src/lib.rs:142 |  |  | 0.588 |
| ns | 7782 |  | 225 | wrapper.rs — the three error adapters and their impls | 6.3 |  | 0.583 |
| ns | 7959 |  | 177 | context.rs — where .context() is implemented | 6.4 | 2.3 | 0.577 |
| walker |  | 7961 | 310 | crate-doc tail at src/lib.rs:179 |  |  | 0.591 |
| walker |  | 8044 | 83 | pub item at src/error.rs:934 |  |  | 0.601 |
| walker |  | 8059 | 15 | impl method body at src/ptr.rs:119 body 120 |  |  | 0.601 |
| walker |  | 8074 | 15 | impl method body at src/ptr.rs:175 body 176 |  |  | 0.601 |
| walker |  | 8090 | 16 | impl method body at src/ptr.rs:169 body 170 |  |  | 0.601 |
| walker |  | 8107 | 17 | impl method body at src/ptr.rs:44 body 45 |  |  | 0.601 |
| ns | 8133 |  | 174 | context.rs — ContextError impls, Quoted, and the Sealed trait | 6.5 | 6.4 | 0.595 |
| walker |  | 8207 | 100 | impl method sigs in src/kind.rs |  |  | 0.595 |
| walker |  | 8207 | 0 | impl method at src/kind.rs:117 |  |  | 0.595 |
| ns | 8261 |  | 128 | fmt.rs — the rendering entry points | 6.6 | 5.6 | 0.590 |
| walker |  | 8298 | 91 | pub-item names surface in src/nightly.rs |  |  | 0.590 |
| walker |  | 8298 | 0 | pub item at src/nightly.rs:41 |  |  | 0.590 |
| walker |  | 8298 | 0 | pub item at src/nightly.rs:52 |  |  | 0.590 |
| walker |  | 8298 | 0 | pub item at src/nightly.rs:56 |  |  | 0.590 |
| walker |  | 8310 | 12 | pub item body at src/nightly.rs:56 body 57 |  |  | 0.590 |
| walker |  | 8323 | 13 | pub item body at src/nightly.rs:41 body 42 |  |  | 0.590 |
| walker |  | 8337 | 14 | pub item body at src/nightly.rs:52 body 53 |  |  | 0.590 |
| walker |  | 8355 | 18 | impl method body at src/error.rs:1010 body 1011 |  |  | 0.590 |
| walker |  | 8381 | 26 | pub item at src/ensure.rs:10 |  |  | 0.591 |
| walker |  | 8407 | 26 | pub item at src/ensure.rs:25 |  |  | 0.592 |
| ns | 8413 |  | 152 | build.rs — the complete custom-cfg vocabulary | 7.1 |  | 0.587 |
| walker |  | 8421 | 14 | listing of 'tests/crate' |  |  | 0.587 |
| walker |  | 8440 | 19 | impl method at src/error.rs:432 |  |  | 0.587 |
| walker |  | 8458 | 18 | impl method body at src/error.rs:432 body 433 |  |  | 0.587 |
| walker |  | 8477 | 19 | impl method at src/error.rs:985 |  |  | 0.587 |
| walker |  | 8496 | 19 | impl method body at src/error.rs:675 body 676 |  |  | 0.587 |
| ns | 8527 |  | 114 | build.rs — which compiler turns each cfg on | 7.2 | 7.1 | 0.584 |
| walker |  | 8599 | 103 | pub-item names surface in src/kind.rs |  |  | 0.588 |
| walker |  | 8619 | 20 | pub item at src/kind.rs:100 |  |  | 0.590 |
| ns | 8637 |  | 110 | backtrace.rs — the three definitions of Backtrace | 7.3 |  | 0.586 |
| walker |  | 8640 | 21 | impl method at src/error.rs:974 |  |  | 0.586 |
| walker |  | 8687 | 47 | pub-item names surface in tests/common/mod.rs |  |  | 0.586 |
| walker |  | 8687 | 0 | pub item at tests/common/mod.rs:4 |  |  | 0.586 |
| walker |  | 8687 | 0 | pub item at tests/common/mod.rs:8 |  |  | 0.586 |
| walker |  | 8687 | 0 | pub item at tests/common/mod.rs:12 |  |  | 0.586 |
| walker |  | 8697 | 10 | pub item body at tests/common/mod.rs:4 body 5 |  |  | 0.586 |
| walker |  | 8713 | 16 | pub item body at tests/common/mod.rs:8 body 9 |  |  | 0.586 |
| ns | 8813 |  | 176 | backtrace.rs — impl_backtrace! and backtrace!(), both arms each | 7.4 | 7.3 | 0.580 |
| ns | 8954 |  | 141 | backtrace.rs — backtrace_if_absent! and the vendored capture module | 7.5 | 7.3 | 0.575 |
| walker |  | 9005 | 292 | README.md section #9 |  |  | 0.577 |
| walker |  | 9028 | 23 | impl method at src/error.rs:140 |  |  | 0.577 |
| walker |  | 9051 | 23 | impl method at src/error.rs:459 |  |  | 0.577 |
| walker |  | 9068 | 17 | impl method body at src/error.rs:459 body 460 |  |  | 0.577 |
| ns | 9086 |  | 132 | nightly.rs — the build probe and generic-member-access shims | 7.6 | 7.1 | 0.574 |
| walker |  | 9091 | 23 | impl method at src/error.rs:622 |  |  | 0.574 |
| walker |  | 9115 | 24 | impl method at src/kind.rs:91 |  |  | 0.575 |
| walker |  | 9123 | 8 | impl method body at src/kind.rs:91 body 95 |  |  | 0.575 |
| ns | 9217 |  | 131 | Toolchain pin and the no_std check-crate | 7.7 |  | 0.569 |
| walker |  | 9260 | 137 | README.md section #6 |  |  | 0.569 |
| walker |  | 9284 | 24 | impl method body at src/ptr.rs:38 body 39 |  |  | 0.569 |
| walker |  | 9308 | 24 | pub item body at tests/common/mod.rs:12 body 13 |  |  | 0.569 |
| ns | 9378 |  | 161 | CI job roster and toolchain matrix | 7.8 |  | 0.563 |
| walker |  | 9453 | 145 | README.md section #3 |  |  | 0.563 |
| walker |  | 9472 | 19 | mod/use plumbing in tests/common/mod.rs |  |  | 0.563 |
| walker |  | 9501 | 29 | impl method at src/context.rs:46 |  |  | 0.563 |
| walker |  | 9530 | 29 | impl method at src/context.rs:91 |  |  | 0.563 |
| walker |  | 9559 | 29 | impl method at src/error.rs:198 |  |  | 0.563 |
| ns | 9570 |  | 192 | CI commands — the canonical build/test/lint invocations | 7.9 | 7.8 | 0.559 |
| walker |  | 9588 | 29 | impl method at src/error.rs:372 |  |  | 0.559 |
| walker |  | 9619 | 31 | impl method at src/error.rs:77 |  |  | 0.559 |
| walker |  | 9637 | 18 | impl method body at src/error.rs:77 body 81 |  |  | 0.559 |
| ns | 9658 |  | 88 | trybuild UI harness — tests/compiletest.rs in full | 8.1 |  | 0.556 |
| walker |  | 9668 | 31 | impl method at src/error.rs:170 |  |  | 0.556 |
| walker |  | 9699 | 31 | impl method at src/error.rs:482 |  |  | 0.556 |
| walker |  | 9714 | 15 | impl method body at src/error.rs:482 body 486 |  |  | 0.556 |
| ns | 9740 |  | 82 | tests/ui file listing | 8.2 | 8.1 | 0.563 |
| walker |  | 9745 | 31 | impl method at src/error.rs:490 |  |  | 0.563 |
| walker |  | 9776 | 31 | impl method at src/error.rs:554 |  |  | 0.563 |
| walker |  | 9807 | 31 | impl method at src/error.rs:568 |  |  | 0.563 |
| walker |  | 9838 | 31 | impl method at src/kind.rs:69 |  |  | 0.565 |
| ns | 9849 |  | 109 | One complete compile-fail pair — wrong-interpolation | 8.3 | 8.2 | 0.561 |
| walker |  | 9856 | 18 | impl method body at src/kind.rs:69 body 73 |  |  | 0.561 |
| walker |  | 9886 | 30 | impl method body at src/ptr.rs:94 body 95 |  |  | 0.561 |
| walker |  | 9917 | 31 | impl method body at src/chain.rs:28 body 29 |  |  | 0.561 |
| walker |  | 9950 | 33 | impl method body at src/ptr.rs:48 body 49 |  |  | 0.561 |
| walker |  | 9983 | 33 | impl method body at src/ptr.rs:55 body 56 |  |  | 0.561 |
| ns | 9990 |  | 141 | Shared test helpers in tests/common and tests/drop | 8.4 | 4.4 | 0.560 |
