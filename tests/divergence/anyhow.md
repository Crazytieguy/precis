Score(3000)=0.703 I=0.901 C=0.548 ns_rows≤3K=20/63 (reached=11 partial=1 missing=8)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 37 | 37 | listing of '.' |  |  | 0.000 |
| walker |  | 85 | 48 | listing of 'src' |  |  | 0.000 |
| ns | 85 |  | 85 | Crate identity — name, version, description | 1.1 |  | 0.000 |
| walker |  | 112 | 27 | [features] in Cargo.toml |  |  | 0.000 |
| walker |  | 120 | 8 | listing of '.github' |  |  | 0.000 |
| ns | 122 |  | 37 | Repository root listing | 1.2 |  | 0.523 |
| walker |  | 123 | 3 | listing of '.github/workflows' |  |  | 0.523 |
| ns | 170 |  | 48 | src/ module inventory | 1.3 |  | 0.545 |
| walker |  | 190 | 67 | README headline in README.md |  |  | 0.549 |
| ns | 261 |  | 91 | What `anyhow::Error` is — a Box<dyn Error> that must be Send + Sync | 1.4 |  | 0.486 |
| walker |  | 280 | 90 | pub-item names surface in src/lib.rs |  |  | 0.489 |
| walker |  | 280 | 0 | pub item at src/lib.rs:468 |  |  | 0.489 |
| walker |  | 293 | 13 | pub item at src/lib.rs:650 |  |  | 0.490 |
| walker |  | 319 | 26 | pub item at src/lib.rs:390 |  |  | 0.493 |
| walker |  | 329 | 10 | pub item body at src/lib.rs:650 body 651 |  |  | 0.494 |
| ns | 335 |  | 74 | …and the other two guarantees: always a backtrace, one word wide | 1.5 | 1.4 | 0.454 |
| walker |  | 378 | 49 | pub item at src/lib.rs:415 |  |  | 0.461 |
| ns | 419 |  | 84 | Cargo.toml package tail — MSRV, edition, license, links | 1.6 |  | 0.422 |
| walker |  | 422 | 44 | headings outline in README.md |  |  | 0.429 |
| ns | 512 |  | 93 | tests/ inventory | 1.7 |  | 0.349 |
| walker |  | 582 | 160 | pub item at src/lib.rs:616 |  |  | 0.358 |
| ns | 650 |  | 138 | Cargo features — std default, optional backtrace | 1.8 |  | 0.330 |
| walker |  | 747 | 165 | [package] in Cargo.toml |  |  | 0.646 |
| ns | 798 |  | 148 | README lede, install snippet, and section map | 1.9 |  | 0.637 |
| walker |  | 821 | 74 | README.md section #0 |  |  | 0.678 |
| walker |  | 931 | 110 | [dependencies] in Cargo.toml |  |  | 0.752 |
| ns | 1014 |  | 216 | lib.rs crate attributes and module declarations | 1.10 |  | 0.652 |
| walker |  | 1205 | 274 | crate-doc lede in src/lib.rs |  |  | 0.652 |
| ns | 1249 |  | 235 | Complete roster of exported items in lib.rs | 2.1 |  | 0.649 |
| walker |  | 1327 | 122 | macro_export names across src |  |  | 0.649 |
| ns | 1457 |  | 208 | impl Error — roster of every public method | 2.2 |  | 0.606 |
| walker |  | 1591 | 264 | mod/use plumbing in src/lib.rs |  |  | 0.651 |
| ns | 1617 |  | 160 | Context trait — both method signatures with bounds | 2.3 | 2.1 | 0.670 |
| walker |  | 1684 | 93 | listing of 'tests' |  |  | 0.763 |
| ns | 1783 |  | 166 | Display representations — `{}` and `{:#}`, with sample output | 2.4 | 1.4 | 0.741 |
| walker |  | 1809 | 125 | manifest config in Cargo.toml |  |  | 0.741 |
| walker |  | 1812 | 3 | listing of 'tests/common' |  |  | 0.741 |
| walker |  | 1853 | 41 | pub-item doc lede at src/lib.rs:415 |  |  | 0.741 |
| walker |  | 1918 | 65 | pub-item doc lede at src/lib.rs:616 |  |  | 0.742 |
| ns | 1920 |  | 137 | Debug representations — `{:?}` and `{:#?}` | 2.5 | 2.4 | 0.716 |
| walker |  | 1921 | 3 | listing of 'tests/drop' |  |  | 0.716 |
| walker |  | 2002 | 81 | listing of 'tests/ui' |  |  | 0.718 |
| ns | 2036 |  | 116 | Context trait doc — sealed, and outermost-first cause printing | 2.6 | 2.3 | 0.703 |
| walker |  | 2140 | 138 | macro_export body at src/macros.rs:58 |  |  | 0.705 |
| ns | 2217 |  | 181 | Context + downcasting — the guarantee, in both directions | 2.7 | 2.6 | 0.675 |
| walker |  | 2314 | 174 | pub-item doc lede at src/lib.rs:390 |  |  | 0.732 |
| ns | 2365 |  | 148 | Result alias and Ok() helper semantics | 2.8 | 2.1 | 0.711 |
| walker |  | 2480 | 166 | impl method sigs in src/context.rs |  |  | 0.712 |
| ns | 2541 |  | 176 | no_std support contract | 2.9 |  | 0.692 |
| ns | 2773 |  | 232 | Trait impls on Error — complete list | 2.10 | 2.2 | 0.667 |
| walker |  | 2881 | 401 | crate attributes in src/lib.rs |  |  | 0.703 |
| walker |  | 2915 | 34 | pub item at src/backtrace.rs:8 |  |  | 0.703 |
| ns | 3007 |  | 234 | What bail! and ensure! mean | 3.1 |  | 0.681 |
| walker |  | 3137 | 222 | macro_export body at src/macros.rs:204 |  |  | 0.685 |
| walker |  | 3174 | 37 | impl method sigs in src/ensure.rs |  |  | 0.685 |
| walker |  | 3174 | 0 | impl method at src/ensure.rs:41 |  |  | 0.685 |
| walker |  | 3174 | 0 | impl method at src/ensure.rs:48 |  |  | 0.685 |
| ns | 3188 |  | 181 | What anyhow! constructs | 3.2 | 3.1 | 0.667 |
| walker |  | 3207 | 33 | pub-item names surface in src/chain.rs |  |  | 0.667 |
| walker |  | 3242 | 35 | pub item at src/chain.rs:11 |  |  | 0.667 |
| ns | 3443 |  | 255 | Every exported macro_rules!, with its export attributes | 3.3 | 3.1 | 0.643 |
| walker |  | 3468 | 226 | pub-item doc lede at src/lib.rs:468 |  |  | 0.652 |
| ns | 3665 |  | 222 | anyhow! expansion — the three arms | 3.4 | 3.3 | 0.671 |
| ns | 3801 |  | 136 | bail! body | 3.5 | 3.3 | 0.679 |
| ns | 3962 |  | 161 | ensure! expansion — the fuel-limited parser call | 3.6 | 3.3 | 0.663 |
| ns | 4131 |  | 169 | kind.rs — why autoref tagged dispatch | 3.7 | 3.4 | 0.652 |
| walker |  | 4157 | 689 | impl method sigs in src/error.rs |  |  | 0.690 |
| walker |  | 4157 | 0 | impl method at src/error.rs:958 |  |  | 0.690 |
| walker |  | 4157 | 0 | impl method at src/error.rs:967 |  |  | 0.690 |
| walker |  | 4157 | 0 | impl method at src/error.rs:1010 |  |  | 0.690 |
| walker |  | 4191 | 34 | pub-item names surface in src/error.rs |  |  | 0.690 |
| walker |  | 4222 | 31 | pub item at src/error.rs:952 |  |  | 0.690 |
| ns | 4380 |  | 249 | kind.rs — the three kinds and their new() constructors | 3.8 | 3.7 | 0.672 |
| walker |  | 4506 | 284 | pub-item doc lede at src/lib.rs:650 |  |  | 0.686 |
| walker |  | 4546 | 40 | impl method sigs in src/chain.rs |  |  | 0.686 |
| walker |  | 4546 | 0 | impl method at src/chain.rs:28 |  |  | 0.686 |
| ns | 4632 |  | 252 | ensure.rs runtime side — BothDebug / NotBothDebug dispatch | 3.9 | 3.6 | 0.666 |
| ns | 4803 |  | 171 | __parse_ensure! extent, first rule, and catch-all | 3.10 | 3.6 | 0.654 |
| walker |  | 4992 | 446 | impl method sigs in src/ptr.rs |  |  | 0.654 |
| walker |  | 4992 | 0 | impl method at src/ptr.rs:32 |  |  | 0.654 |
| walker |  | 4992 | 0 | impl method at src/ptr.rs:38 |  |  | 0.654 |
| walker |  | 4992 | 0 | impl method at src/ptr.rs:44 |  |  | 0.654 |
| walker |  | 4992 | 0 | impl method at src/ptr.rs:48 |  |  | 0.654 |
| walker |  | 4992 | 0 | impl method at src/ptr.rs:55 |  |  | 0.654 |
| walker |  | 4992 | 0 | impl method at src/ptr.rs:87 |  |  | 0.654 |
| walker |  | 4992 | 0 | impl method at src/ptr.rs:94 |  |  | 0.654 |
| walker |  | 4992 | 0 | impl method at src/ptr.rs:101 |  |  | 0.654 |
| walker |  | 4992 | 0 | impl method at src/ptr.rs:108 |  |  | 0.654 |
| walker |  | 4992 | 0 | impl method at src/ptr.rs:115 |  |  | 0.654 |
| walker |  | 4992 | 0 | impl method at src/ptr.rs:119 |  |  | 0.654 |
| walker |  | 4992 | 0 | impl method at src/ptr.rs:148 |  |  | 0.654 |
| walker |  | 4992 | 0 | impl method at src/ptr.rs:155 |  |  | 0.654 |
| walker |  | 4992 | 0 | impl method at src/ptr.rs:162 |  |  | 0.654 |
| walker |  | 4992 | 0 | impl method at src/ptr.rs:169 |  |  | 0.654 |
| walker |  | 4992 | 0 | impl method at src/ptr.rs:175 |  |  | 0.654 |
| ns | 5000 |  | 197 | lib.rs __private — the macro-facing surface | 3.11 | 3.3 | 0.640 |
| ns | 5161 |  | 161 | test_fmt.rs — the f/g/h fixture and expected `{:#}` strings | 4.1 | 2.4 | 0.632 |
| ns | 5300 |  | 139 | test_fmt.rs — expected `{:?}` strings, including numbered causes | 4.2 | 4.1 | 0.618 |
| ns | 5459 |  | 159 | test_repr.rs — size, null-pointer-optimization, autotrait assertions | 4.3 | 1.5 | 0.605 |
| walker |  | 5520 | 528 | crate-doc body in src/lib.rs |  |  | 0.605 |
| walker |  | 5559 | 39 | pub-item names surface in src/ensure.rs |  |  | 0.605 |
| ns | 5638 |  | 179 | test_downcast.rs — what a bail!'d error downcasts to | 4.4 | 3.4 | 0.590 |
| walker |  | 5696 | 137 | README.md section #8 |  |  | 0.590 |
| walker |  | 5746 | 50 | pub-item names surface in src/ptr.rs |  |  | 0.590 |
| walker |  | 5759 | 13 | pub item at src/ptr.rs:181 |  |  | 0.591 |
| walker |  | 5806 | 47 | pub item at src/ptr.rs:6 |  |  | 0.591 |
| ns | 5812 |  | 174 | test_chain.rs — cause iteration order | 4.5 |  | 0.582 |
| walker |  | 5818 | 12 | impl method at src/error.rs:675 |  |  | 0.582 |
| walker |  | 5879 | 61 | pub item at src/ptr.rs:64 |  |  | 0.583 |
| walker |  | 5902 | 23 | pub-item names surface in tests/drop/mod.rs |  |  | 0.583 |
| walker |  | 5926 | 24 | pub item at tests/drop/mod.rs:26 |  |  | 0.583 |
| walker |  | 5951 | 25 | pub item at tests/drop/mod.rs:9 |  |  | 0.583 |
| ns | 6007 |  | 195 | test_context.rs — downcast_ref through a three-level context chain | 4.6 | 2.7 | 0.573 |
| walker |  | 6013 | 62 | pub item at src/ptr.rs:125 |  |  | 0.574 |
| walker |  | 6067 | 54 | pub-item names surface in src/wrapper.rs |  |  | 0.574 |
| walker |  | 6076 | 9 | pub item at src/wrapper.rs:11 |  |  | 0.574 |
| walker |  | 6085 | 9 | pub item at src/wrapper.rs:34 |  |  | 0.574 |
| walker |  | 6114 | 29 | pub item at src/wrapper.rs:58 |  |  | 0.574 |
| walker |  | 6127 | 13 | listing of 'tests/crate' |  |  | 0.574 |
| walker |  | 6140 | 13 | impl method at src/error.rs:1002 |  |  | 0.574 |
| walker |  | 6153 | 13 | impl method body at src/ptr.rs:115 body 116 |  |  | 0.574 |
| walker |  | 6225 | 72 | impl method sigs in src/fmt.rs |  |  | 0.574 |
| walker |  | 6225 | 0 | impl method at src/fmt.rs:7 |  |  | 0.574 |
| walker |  | 6225 | 0 | impl method at src/fmt.rs:20 |  |  | 0.574 |
| ns | 6257 |  | 250 | ErrorVTable — the hand-rolled vtable layout | 5.1 |  | 0.565 |
| walker |  | 6443 | 218 | README.md section #7 |  |  | 0.565 |
| ns | 6488 |  | 231 | ErrorImpl, ContextError, and the vtable reader | 5.2 | 5.1 | 0.556 |
| walker |  | 6542 | 99 | pub item at src/chain.rs:16 |  |  | 0.557 |
| ns | 6620 |  | 132 | Error::construct — allocation and type erasure | 5.3 | 5.2 | 0.549 |
| ns | 6723 |  | 103 | impl Error — roster of the non-public fns | 5.4 | 5.3 | 0.554 |
| ns | 6894 |  | 171 | Vtable function roster in error.rs | 5.5 | 5.1 | 0.547 |
| walker |  | 6911 | 369 | crate-doc tail at src/lib.rs:95 |  |  | 0.547 |
| ns | 7045 |  | 151 | ErrorImpl methods and trait impls | 5.6 | 5.2 | 0.548 |
| ns | 7280 |  | 235 | ptr.rs — Own / Ref / Mut / CastTo declarations | 5.7 |  | 0.568 |
| walker |  | 7292 | 381 | crate-doc tail at src/lib.rs:142 |  |  | 0.568 |
| ns | 7445 |  | 165 | chain.rs — Chain and ChainState | 6.1 |  | 0.576 |
| ns | 7562 |  | 117 | chain.rs — every trait impl on Chain | 6.2 | 6.1 | 0.571 |
| walker |  | 7602 | 310 | crate-doc tail at src/lib.rs:179 |  |  | 0.586 |
| walker |  | 7685 | 83 | pub item at src/error.rs:934 |  |  | 0.594 |
| walker |  | 7700 | 15 | impl method body at src/ptr.rs:119 body 120 |  |  | 0.594 |
| walker |  | 7715 | 15 | impl method body at src/ptr.rs:175 body 176 |  |  | 0.594 |
| ns | 7787 |  | 225 | wrapper.rs — the three error adapters and their impls | 6.3 |  | 0.588 |
| walker |  | 7942 | 227 | README.md section #4 |  |  | 0.588 |
| walker |  | 7958 | 16 | impl method body at src/ptr.rs:169 body 170 |  |  | 0.588 |
| ns | 7964 |  | 177 | context.rs — where .context() is implemented | 6.4 | 2.3 | 0.583 |
| walker |  | 7975 | 17 | impl method body at src/ptr.rs:44 body 45 |  |  | 0.583 |
| walker |  | 8075 | 100 | impl method sigs in src/kind.rs |  |  | 0.583 |
| walker |  | 8075 | 0 | impl method at src/kind.rs:117 |  |  | 0.583 |
| ns | 8138 |  | 174 | context.rs — ContextError impls, Quoted, and the Sealed trait | 6.5 | 6.4 | 0.578 |
| walker |  | 8166 | 91 | pub-item names surface in src/nightly.rs |  |  | 0.578 |
| walker |  | 8166 | 0 | pub item at src/nightly.rs:41 |  |  | 0.578 |
| walker |  | 8166 | 0 | pub item at src/nightly.rs:52 |  |  | 0.578 |
| walker |  | 8166 | 0 | pub item at src/nightly.rs:56 |  |  | 0.578 |
| walker |  | 8178 | 12 | pub item body at src/nightly.rs:56 body 57 |  |  | 0.578 |
| walker |  | 8191 | 13 | pub item body at src/nightly.rs:41 body 42 |  |  | 0.578 |
| walker |  | 8205 | 14 | pub item body at src/nightly.rs:52 body 53 |  |  | 0.578 |
| walker |  | 8223 | 18 | impl method body at src/error.rs:1010 body 1011 |  |  | 0.578 |
| walker |  | 8242 | 19 | impl method at src/error.rs:432 |  |  | 0.578 |
| walker |  | 8260 | 18 | impl method body at src/error.rs:432 body 433 |  |  | 0.578 |
| ns | 8266 |  | 128 | fmt.rs — the rendering entry points | 6.6 | 5.6 | 0.573 |
| walker |  | 8279 | 19 | impl method at src/error.rs:985 |  |  | 0.573 |
| walker |  | 8298 | 19 | impl method body at src/error.rs:675 body 676 |  |  | 0.573 |
| walker |  | 8401 | 103 | pub-item names surface in src/kind.rs |  |  | 0.577 |
| ns | 8418 |  | 152 | build.rs — the complete custom-cfg vocabulary | 7.1 |  | 0.573 |
| walker |  | 8421 | 20 | pub item at src/kind.rs:100 |  |  | 0.574 |
| walker |  | 8442 | 21 | impl method at src/error.rs:974 |  |  | 0.574 |
| walker |  | 8489 | 47 | pub-item names surface in tests/common/mod.rs |  |  | 0.575 |
| walker |  | 8489 | 0 | pub item at tests/common/mod.rs:4 |  |  | 0.575 |
| walker |  | 8489 | 0 | pub item at tests/common/mod.rs:8 |  |  | 0.575 |
| walker |  | 8489 | 0 | pub item at tests/common/mod.rs:12 |  |  | 0.575 |
| walker |  | 8499 | 10 | pub item body at tests/common/mod.rs:4 body 5 |  |  | 0.575 |
| walker |  | 8515 | 16 | pub item body at tests/common/mod.rs:8 body 9 |  |  | 0.575 |
| ns | 8532 |  | 114 | build.rs — which compiler turns each cfg on | 7.2 | 7.1 | 0.572 |
| ns | 8642 |  | 110 | backtrace.rs — the three definitions of Backtrace | 7.3 |  | 0.568 |
| walker |  | 8807 | 292 | README.md section #9 |  |  | 0.571 |
| ns | 8818 |  | 176 | backtrace.rs — impl_backtrace! and backtrace!(), both arms each | 7.4 | 7.3 | 0.565 |
| walker |  | 8830 | 23 | impl method at src/error.rs:140 |  |  | 0.565 |
| walker |  | 8853 | 23 | impl method at src/error.rs:459 |  |  | 0.565 |
| walker |  | 8870 | 17 | impl method body at src/error.rs:459 body 460 |  |  | 0.565 |
| walker |  | 8893 | 23 | impl method at src/error.rs:622 |  |  | 0.565 |
| walker |  | 8917 | 24 | impl method at src/kind.rs:91 |  |  | 0.566 |
| walker |  | 8925 | 8 | impl method body at src/kind.rs:91 body 95 |  |  | 0.566 |
| ns | 8959 |  | 141 | backtrace.rs — backtrace_if_absent! and the vendored capture module | 7.5 | 7.3 | 0.561 |
| walker |  | 9062 | 137 | README.md section #6 |  |  | 0.561 |
| walker |  | 9086 | 24 | impl method body at src/ptr.rs:38 body 39 |  |  | 0.561 |
| ns | 9091 |  | 132 | nightly.rs — the build probe and generic-member-access shims | 7.6 | 7.1 | 0.558 |
| walker |  | 9110 | 24 | pub item body at tests/common/mod.rs:12 body 13 |  |  | 0.558 |
| ns | 9222 |  | 131 | Toolchain pin and the no_std check-crate | 7.7 |  | 0.552 |
| walker |  | 9253 | 143 | README.md section #3 |  |  | 0.552 |
| walker |  | 9272 | 19 | mod/use plumbing in tests/common/mod.rs |  |  | 0.552 |
| walker |  | 9301 | 29 | impl method at src/context.rs:46 |  |  | 0.552 |
| walker |  | 9330 | 29 | impl method at src/context.rs:91 |  |  | 0.552 |
| walker |  | 9359 | 29 | impl method at src/error.rs:198 |  |  | 0.552 |
| ns | 9383 |  | 161 | CI job roster and toolchain matrix | 7.8 |  | 0.546 |
| walker |  | 9388 | 29 | impl method at src/error.rs:372 |  |  | 0.546 |
| walker |  | 9419 | 31 | impl method at src/error.rs:77 |  |  | 0.546 |
| walker |  | 9437 | 18 | impl method body at src/error.rs:77 body 81 |  |  | 0.546 |
| walker |  | 9468 | 31 | impl method at src/error.rs:170 |  |  | 0.546 |
| walker |  | 9499 | 31 | impl method at src/error.rs:482 |  |  | 0.546 |
| walker |  | 9514 | 15 | impl method body at src/error.rs:482 body 486 |  |  | 0.546 |
| walker |  | 9545 | 31 | impl method at src/error.rs:490 |  |  | 0.546 |
| ns | 9575 |  | 192 | CI commands — the canonical build/test/lint invocations | 7.9 | 7.8 | 0.542 |
| walker |  | 9576 | 31 | impl method at src/error.rs:554 |  |  | 0.542 |
| walker |  | 9607 | 31 | impl method at src/error.rs:568 |  |  | 0.542 |
| walker |  | 9638 | 31 | impl method at src/kind.rs:69 |  |  | 0.544 |
| walker |  | 9656 | 18 | impl method body at src/kind.rs:69 body 73 |  |  | 0.544 |
| ns | 9663 |  | 88 | trybuild UI harness — tests/compiletest.rs in full | 8.1 |  | 0.541 |
| walker |  | 9686 | 30 | impl method body at src/ptr.rs:94 body 95 |  |  | 0.541 |
| walker |  | 9717 | 31 | impl method body at src/chain.rs:28 body 29 |  |  | 0.541 |
| ns | 9744 |  | 81 | tests/ui file listing | 8.2 | 8.1 | 0.549 |
| walker |  | 9750 | 33 | impl method body at src/ptr.rs:48 body 49 |  |  | 0.549 |
| walker |  | 9783 | 33 | impl method body at src/ptr.rs:55 body 56 |  |  | 0.549 |
| walker |  | 9816 | 33 | impl method body at src/ptr.rs:108 body 109 |  |  | 0.549 |
| walker |  | 9849 | 33 | impl method body at src/ptr.rs:155 body 156 |  |  | 0.549 |
| ns | 9853 |  | 109 | One complete compile-fail pair — wrong-interpolation | 8.3 | 8.2 | 0.545 |
| walker |  | 9882 | 33 | impl method body at src/ptr.rs:162 body 163 |  |  | 0.545 |
| walker |  | 9916 | 34 | impl method body at src/ptr.rs:101 body 102 |  |  | 0.545 |
| walker |  | 9950 | 34 | impl method body at src/ptr.rs:148 body 149 |  |  | 0.545 |
| walker |  | 9985 | 35 | impl method body at src/kind.rs:117 body 118 |  |  | 0.545 |
| ns | 9994 |  | 141 | Shared test helpers in tests/common and tests/drop | 8.4 | 4.4 | 0.544 |
