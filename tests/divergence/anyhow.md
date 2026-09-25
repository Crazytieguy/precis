Score(3000)=0.583 I=0.815 C=0.418 ns_rows≤3K=20/63 grid(1000/1442/2080/3000/4327/6240/9000)=0.677/0.670/0.650/0.583/0.507/0.421/0.535

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
| walker |  | 315 | 126 | rust names src/lib.rs |  |  | 0.491 |
| walker |  | 333 | 18 | rust decl src/lib.rs:648 |  |  | 0.453 |
| ns | 333 |  | 74 | …and the other two guarantees: always a backtrace, one word wide | 1.5 | 1.4 | 0.453 |
| walker |  | 359 | 26 | rust decl src/lib.rs:389 |  |  | 0.457 |
| walker |  | 408 | 49 | rust decl src/lib.rs:413 |  |  | 0.464 |
| ns | 417 |  | 84 | Cargo.toml package tail — MSRV, edition, license, links | 1.6 |  | 0.425 |
| walker |  | 464 | 56 | rust decl src/lib.rs:616 |  |  | 0.426 |
| walker |  | 487 | 23 | rust decl src/lib.rs:618 |  |  | 0.427 |
| ns | 507 |  | 90 | tests/ inventory | 1.7 |  | 0.348 |
| walker |  | 521 | 34 | rust decl src/lib.rs:624 |  |  | 0.350 |
| walker |  | 590 | 69 | rust names build.rs |  |  | 0.350 |
| walker |  | 634 | 44 | headings outline in README.md |  |  | 0.355 |
| ns | 645 |  | 138 | Cargo features — std default, optional backtrace | 1.8 |  | 0.327 |
| walker |  | 670 | 36 | rust names src/macros.rs |  |  | 0.327 |
| ns | 793 |  | 148 | README lede, install snippet, and section map | 1.9 |  | 0.335 |
| walker |  | 835 | 165 | [package] in Cargo.toml |  |  | 0.635 |
| walker |  | 880 | 45 | rust decl src/lib.rs:278 |  |  | 0.635 |
| walker |  | 924 | 44 | rust names src/fmt.rs |  |  | 0.636 |
| walker |  | 998 | 74 | README.md section #0 |  |  | 0.677 |
| ns | 1009 |  | 216 | lib.rs crate attributes and module declarations | 1.10 |  | 0.587 |
| walker |  | 1049 | 51 | rust names src/ensure.rs |  |  | 0.587 |
| walker |  | 1076 | 27 | rust decl src/ensure.rs:58 |  |  | 0.587 |
| walker |  | 1107 | 31 | rust decl src/ensure.rs:35 |  |  | 0.587 |
| walker |  | 1117 | 10 | rust body src/lib.rs:648 |  |  | 0.588 |
| walker |  | 1227 | 110 | [dependencies] in Cargo.toml |  |  | 0.651 |
| ns | 1244 |  | 235 | Complete roster of exported items in lib.rs | 2.1 |  | 0.670 |
| walker |  | 1290 | 63 | rust names src/kind.rs |  |  | 0.670 |
| walker |  | 1310 | 20 | rust decl src/kind.rs:99 |  |  | 0.670 |
| walker |  | 1336 | 26 | rust decl src/kind.rs:67 |  |  | 0.670 |
| walker |  | 1362 | 26 | rust decl src/kind.rs:89 |  |  | 0.670 |
| walker |  | 1414 | 52 | rust decl src/kind.rs:114 |  |  | 0.670 |
| walker |  | 1420 | 6 | rust decl src/kind.rs:116 |  |  | 0.670 |
| walker |  | 1450 | 30 | rust decl src/kind.rs:90 |  |  | 0.671 |
| ns | 1452 |  | 208 | impl Error — roster of every public method | 2.2 |  | 0.625 |
| walker |  | 1487 | 37 | rust decl src/kind.rs:68 |  |  | 0.626 |
| walker |  | 1577 | 90 | listing of 'tests' |  |  | 0.728 |
| ns | 1612 |  | 160 | Context trait — both method signatures with bounds | 2.3 | 2.1 | 0.713 |
| walker |  | 1688 | 111 | rust names src/chain.rs |  |  | 0.713 |
| walker |  | 1708 | 20 | rust decl src/chain.rs:76 |  |  | 0.713 |
| walker |  | 1738 | 30 | rust decl src/chain.rs:26 |  |  | 0.713 |
| walker |  | 1744 | 6 | rust decl src/chain.rs:27 |  |  | 0.713 |
| ns | 1778 |  | 166 | Display representations — `{}` and `{:#}`, with sample output | 2.4 | 1.4 | 0.693 |
| ns | 1915 |  | 137 | Debug representations — `{:?}` and `{:#?}` | 2.5 | 2.4 | 0.669 |
| walker |  | 1988 | 244 | rust names src/error.rs |  |  | 0.670 |
| walker |  | 2005 | 17 | rust decl src/error.rs:731 |  |  | 0.670 |
| ns | 2031 |  | 116 | Context trait doc — sealed, and outermost-first cause printing | 2.6 | 2.3 | 0.650 |
| walker |  | 2033 | 28 | rust decl src/error.rs:719 |  |  | 0.650 |
| walker |  | 2061 | 28 | rust decl src/error.rs:725 |  |  | 0.650 |
| walker |  | 2106 | 45 | rust decl src/error.rs:712 |  |  | 0.650 |
| walker |  | 2172 | 66 | rust decl src/error.rs:703 |  |  | 0.651 |
| ns | 2212 |  | 181 | Context + downcasting — the guarantee, in both directions | 2.7 | 2.6 | 0.623 |
| walker |  | 2241 | 69 | rust decl src/error.rs:691 |  |  | 0.623 |
| walker |  | 2247 | 6 | rust decl src/error.rs:696 |  |  | 0.623 |
| walker |  | 2275 | 28 | rust decl src/error.rs:775 |  |  | 0.623 |
| walker |  | 2309 | 34 | rust decl src/chain.rs:93 |  |  | 0.623 |
| ns | 2360 |  | 148 | Result alias and Ok() helper semantics | 2.8 | 2.1 | 0.605 |
| walker |  | 2432 | 123 | rust names src/backtrace.rs |  |  | 0.606 |
| walker |  | 2451 | 19 | rust decl src/backtrace.rs:7 |  |  | 0.606 |
| walker |  | 2486 | 35 | rust decl src/chain.rs:10 |  |  | 0.606 |
| walker |  | 2521 | 35 | rust decl src/fmt.rs:69 |  |  | 0.606 |
| ns | 2536 |  | 176 | no_std support contract | 2.9 |  | 0.589 |
| walker |  | 2561 | 40 | rust decl src/backtrace.rs:10 |  |  | 0.590 |
| walker |  | 2573 | 12 | rust doc src/lib.rs:618 |  |  | 0.596 |
| walker |  | 2617 | 44 | rust decl src/chain.rs:56 |  |  | 0.596 |
| walker |  | 2662 | 45 | rust decl src/backtrace.rs:31 |  |  | 0.596 |
| ns | 2768 |  | 232 | Trait impls on Error — complete list | 2.10 | 2.2 | 0.583 |
| walker |  | 2834 | 172 | rust names src/context.rs |  |  | 0.583 |
| walker |  | 2882 | 48 | rust decl src/context.rs:128 |  |  | 0.583 |
| walker |  | 2934 | 52 | rust decl src/context.rs:90 |  |  | 0.583 |
| walker |  | 2963 | 29 | rust decl src/context.rs:91 |  |  | 0.583 |
| ns | 3002 |  | 234 | What bail! and ensure! mean | 3.1 |  | 0.565 |
| walker |  | 3020 | 57 | rust decl src/context.rs:115 |  |  | 0.565 |
| walker |  | 3092 | 72 | rust decl src/context.rs:152 |  |  | 0.565 |
| walker |  | 3104 | 12 | rust decl src/context.rs:160 |  |  | 0.565 |
| walker |  | 3146 | 42 | rust decl src/context.rs:103 |  |  | 0.565 |
| ns | 3183 |  | 181 | What anyhow! constructs | 3.2 | 3.1 | 0.551 |
| walker |  | 3230 | 84 | rust decl src/context.rs:42 |  |  | 0.551 |
| walker |  | 3259 | 29 | rust decl src/context.rs:46 |  |  | 0.551 |
| walker |  | 3301 | 42 | rust decl src/context.rs:58 |  |  | 0.551 |
| walker |  | 3386 | 85 | rust decl src/context.rs:137 |  |  | 0.551 |
| walker |  | 3398 | 12 | rust decl src/context.rs:146 |  |  | 0.551 |
| walker |  | 3427 | 29 | rust decl src/context.rs:180 |  |  | 0.551 |
| ns | 3438 |  | 255 | Every exported macro_rules!, with its export attributes | 3.3 | 3.1 | 0.527 |
| walker |  | 3475 | 48 | rust decl src/context.rs:168 |  |  | 0.527 |
| walker |  | 3523 | 48 | rust decl src/error.rs:787 |  |  | 0.527 |
| walker |  | 3571 | 48 | rust decl src/error.rs:798 |  |  | 0.527 |
| ns | 3660 |  | 222 | anyhow! expansion — the three arms | 3.4 | 3.3 | 0.507 |
| walker |  | 3745 | 174 | rust names src/nightly.rs |  |  | 0.507 |
| walker |  | 3756 | 11 | rust decl src/nightly.rs:38 |  |  | 0.507 |
| walker |  | 3768 | 12 | rust body src/nightly.rs:56 |  |  | 0.507 |
| walker |  | 3781 | 13 | rust body src/nightly.rs:41 |  |  | 0.507 |
| ns | 3796 |  | 136 | bail! body | 3.5 | 3.3 | 0.497 |
| walker |  | 3805 | 24 | rust decl src/nightly.rs:45 |  |  | 0.497 |
| walker |  | 3819 | 14 | rust body src/nightly.rs:52 |  |  | 0.497 |
| ns | 3957 |  | 161 | ensure! expansion — the fuel-limited parser call | 3.6 | 3.3 | 0.486 |
| walker |  | 3985 | 166 | rust decl src/macros.rs:56 |  |  | 0.514 |
| walker |  | 4034 | 49 | rust decl src/fmt.rs:75 |  |  | 0.514 |
| ns | 4126 |  | 169 | kind.rs — why autoref tagged dispatch | 3.7 | 3.4 | 0.506 |
| walker |  | 4159 | 125 | manifest config in Cargo.toml |  |  | 0.506 |
| walker |  | 4210 | 51 | rust decl src/backtrace.rs:24 |  |  | 0.506 |
| walker |  | 4262 | 52 | rust decl src/backtrace.rs:17 |  |  | 0.507 |
| ns | 4375 |  | 249 | kind.rs — the three kinds and their new() constructors | 3.8 | 3.7 | 0.502 |
| walker |  | 4472 | 210 | rust names src/wrapper.rs |  |  | 0.503 |
| walker |  | 4479 | 7 | rust decl src/wrapper.rs:33 |  |  | 0.503 |
| walker |  | 4488 | 9 | rust decl src/wrapper.rs:10 |  |  | 0.503 |
| walker |  | 4515 | 27 | rust decl src/wrapper.rs:56 |  |  | 0.503 |
| walker |  | 4563 | 48 | rust decl src/wrapper.rs:36 |  |  | 0.503 |
| walker |  | 4611 | 48 | rust decl src/wrapper.rs:45 |  |  | 0.503 |
| ns | 4627 |  | 252 | ensure.rs runtime side — BothDebug / NotBothDebug dispatch | 3.9 | 3.6 | 0.489 |
| walker |  | 4659 | 48 | rust decl src/wrapper.rs:60 |  |  | 0.489 |
| walker |  | 4707 | 48 | rust decl src/wrapper.rs:67 |  |  | 0.489 |
| walker |  | 4757 | 50 | rust decl src/wrapper.rs:13 |  |  | 0.489 |
| ns | 4798 |  | 171 | __parse_ensure! extent, first rule, and catch-all | 3.10 | 3.6 | 0.480 |
| walker |  | 4807 | 50 | rust decl src/wrapper.rs:22 |  |  | 0.480 |
| walker |  | 4877 | 70 | rust decl src/wrapper.rs:74 |  |  | 0.480 |
| walker |  | 4889 | 12 | rust decl src/wrapper.rs:80 |  |  | 0.480 |
| ns | 4995 |  | 197 | lib.rs __private — the macro-facing surface | 3.11 | 3.3 | 0.470 |
| ns | 5156 |  | 161 | test_fmt.rs — the f/g/h fixture and expected `{:#}` strings | 4.1 | 2.4 | 0.463 |
| walker |  | 5205 | 316 | rust names src/ptr.rs |  |  | 0.464 |
| walker |  | 5216 | 11 | rust decl src/ptr.rs:181 |  |  | 0.464 |
| walker |  | 5229 | 13 | rust decl src/ptr.rs:185 |  |  | 0.464 |
| walker |  | 5250 | 21 | rust decl src/ptr.rs:174 |  |  | 0.464 |
| walker |  | 5289 | 39 | rust decl src/ptr.rs:19 |  |  | 0.464 |
| ns | 5295 |  | 139 | test_fmt.rs — expected `{:?}` strings, including numbered causes | 4.2 | 4.1 | 0.453 |
| walker |  | 5328 | 39 | rust decl src/ptr.rs:74 |  |  | 0.453 |
| walker |  | 5367 | 39 | rust decl src/ptr.rs:135 |  |  | 0.453 |
| walker |  | 5412 | 45 | rust decl src/ptr.rs:5 |  |  | 0.453 |
| ns | 5454 |  | 159 | test_repr.rs — size, null-pointer-optimization, autotrait assertions | 4.3 | 1.5 | 0.444 |
| walker |  | 5471 | 59 | rust decl src/ptr.rs:63 |  |  | 0.445 |
| walker |  | 5531 | 60 | rust decl src/ptr.rs:124 |  |  | 0.446 |
| ns | 5633 |  | 179 | test_downcast.rs — what a bail!'d error downcasts to | 4.4 | 3.4 | 0.435 |
| walker |  | 5647 | 116 | rust decl src/ptr.rs:144 |  |  | 0.435 |
| walker |  | 5770 | 123 | rust decl src/ptr.rs:28 |  |  | 0.435 |
| ns | 5807 |  | 174 | test_chain.rs — cause iteration order | 4.5 |  | 0.428 |
| walker |  | 5918 | 148 | rust decl src/ptr.rs:83 |  |  | 0.428 |
| walker |  | 5983 | 65 | rust decl src/fmt.rs:6 |  |  | 0.429 |
| ns | 6002 |  | 195 | test_context.rs — downcast_ref through a three-level context chain | 4.6 | 2.7 | 0.421 |
| walker |  | 6051 | 68 | rust decl src/chain.rs:35 |  |  | 0.421 |
| ns | 6252 |  | 250 | ErrorVTable — the hand-rolled vtable layout | 5.1 |  | 0.415 |
| walker |  | 6303 | 252 | rust decl src/macros.rs:202 |  |  | 0.455 |
| walker |  | 6386 | 83 | rust decl src/backtrace.rs:59 |  |  | 0.455 |
| walker |  | 6473 | 87 | rust decl src/backtrace.rs:38 |  |  | 0.455 |
| ns | 6483 |  | 231 | ErrorImpl, ContextError, and the vtable reader | 5.2 | 5.1 | 0.446 |
| walker |  | 6559 | 86 | rust decl src/backtrace.rs:48 |  |  | 0.446 |
| ns | 6615 |  | 132 | Error::construct — allocation and type erasure | 5.3 | 5.2 | 0.439 |
| ns | 6718 |  | 103 | impl Error — roster of the non-public fns | 5.4 | 5.3 | 0.436 |
| walker |  | 6758 | 199 | rust names src/error.rs #1 |  |  | 0.437 |
| walker |  | 6779 | 21 | rust decl src/error.rs:809 |  |  | 0.437 |
| walker |  | 6800 | 21 | rust decl src/error.rs:876 |  |  | 0.437 |
| walker |  | 6821 | 21 | rust decl src/error.rs:892 |  |  | 0.437 |
| walker |  | 6861 | 40 | rust decl src/error.rs:828 |  |  | 0.437 |
| ns | 6889 |  | 171 | Vtable function roster in error.rs | 5.5 | 5.1 | 0.450 |
| walker |  | 6912 | 51 | rust decl src/error.rs:838 |  |  | 0.450 |
| walker |  | 6963 | 51 | rust decl src/error.rs:858 |  |  | 0.450 |
| walker |  | 7038 | 75 | rust decl src/error.rs:915 |  |  | 0.450 |
| ns | 7040 |  | 151 | ErrorImpl methods and trait impls | 5.6 | 5.2 | 0.445 |
| walker |  | 7137 | 99 | rust decl src/chain.rs:15 |  |  | 0.446 |
| ns | 7275 |  | 235 | ptr.rs — Own / Ref / Mut / CastTo declarations | 5.7 |  | 0.472 |
| walker |  | 7422 | 285 | rust module doc src/lib.rs |  |  | 0.472 |
| walker |  | 7430 | 8 | rust body src/kind.rs:90 |  |  | 0.472 |
| walker |  | 7437 | 7 | rust body src/lib.rs:280 |  |  | 0.472 |
| ns | 7440 |  | 165 | chain.rs — Chain and ChainState | 6.1 |  | 0.483 |
| walker |  | 7441 | 4 | listing of 'tests/common' |  |  | 0.483 |
| walker |  | 7445 | 4 | listing of 'tests/drop' |  |  | 0.483 |
| ns | 7557 |  | 117 | chain.rs — every trait impl on Chain | 6.2 | 6.1 | 0.489 |
| walker |  | 7649 | 204 | rust decl src/error.rs:19 |  |  | 0.496 |
| walker |  | 7687 | 38 | rust decl src/error.rs:137 |  |  | 0.496 |
| walker |  | 7735 | 48 | rust decl src/error.rs:75 |  |  | 0.496 |
| ns | 7782 |  | 225 | wrapper.rs — the three error adapters and their impls | 6.3 |  | 0.508 |
| walker |  | 7803 | 68 | rust decl src/error.rs:27 |  |  | 0.508 |
| walker |  | 7840 | 37 | rust decl src/error.rs:197 |  |  | 0.508 |
| walker |  | 7879 | 39 | rust decl src/error.rs:169 |  |  | 0.508 |
| walker |  | 7938 | 59 | rust decl src/error.rs:145 |  |  | 0.508 |
| ns | 7959 |  | 177 | context.rs — where .context() is implemented | 6.4 | 2.3 | 0.503 |
| walker |  | 8006 | 68 | rust decl src/error.rs:258 |  |  | 0.503 |
| walker |  | 8088 | 82 | listing of 'tests/ui' |  |  | 0.504 |
| ns | 8133 |  | 174 | context.rs — ContextError impls, Quoted, and the Sealed trait | 6.5 | 6.4 | 0.504 |
| ns | 8261 |  | 128 | fmt.rs — the rendering entry points | 6.6 | 5.6 | 0.509 |
| walker |  | 8392 | 304 | rust module doc src/lib.rs #1 |  |  | 0.509 |
| ns | 8413 |  | 152 | build.rs — the complete custom-cfg vocabulary | 7.1 |  | 0.505 |
| walker |  | 8422 | 30 | rust doc src/lib.rs:624 |  |  | 0.512 |
| ns | 8527 |  | 114 | build.rs — which compiler turns each cfg on | 7.2 | 7.1 | 0.509 |
| ns | 8637 |  | 110 | backtrace.rs — the three definitions of Backtrace | 7.3 |  | 0.506 |
| walker |  | 8680 | 258 | rust names src/error.rs #2 |  |  | 0.524 |
| walker |  | 8726 | 46 | rust decl src/error.rs:1047 |  |  | 0.524 |
| walker |  | 8733 | 7 | rust decl src/error.rs:1049 |  |  | 0.524 |
| walker |  | 8777 | 44 | rust decl src/error.rs:1055 |  |  | 0.524 |
| walker |  | 8784 | 7 | rust decl src/error.rs:1057 |  |  | 0.524 |
| ns | 8813 |  | 176 | backtrace.rs — impl_backtrace! and backtrace!(), both arms each | 7.4 | 7.3 | 0.531 |
| walker |  | 8828 | 44 | rust decl src/error.rs:1063 |  |  | 0.531 |
| walker |  | 8835 | 7 | rust decl src/error.rs:1065 |  |  | 0.531 |
| walker |  | 8884 | 49 | rust decl src/error.rs:1078 |  |  | 0.531 |
| walker |  | 8935 | 51 | rust decl src/error.rs:1071 |  |  | 0.531 |
| ns | 8954 |  | 141 | backtrace.rs — backtrace_if_absent! and the vendored capture module | 7.5 | 7.3 | 0.533 |
| walker |  | 8968 | 33 | rust decl src/error.rs:951 |  |  | 0.535 |
| walker |  | 9021 | 53 | rust decl src/error.rs:1029 |  |  | 0.535 |
| walker |  | 9074 | 53 | rust decl src/error.rs:1038 |  |  | 0.535 |
| ns | 9086 |  | 132 | nightly.rs — the build probe and generic-member-access shims | 7.6 | 7.1 | 0.532 |
| walker |  | 9153 | 79 | rust decl src/error.rs:1015 |  |  | 0.532 |
| walker |  | 9166 | 13 | rust decl src/error.rs:1023 |  |  | 0.532 |
| ns | 9217 |  | 131 | Toolchain pin and the no_std check-crate | 7.7 |  | 0.526 |
| walker |  | 9249 | 83 | rust decl src/error.rs:933 |  |  | 0.535 |
| walker |  | 9366 | 117 | rust decl src/error.rs:966 |  |  | 0.540 |
| walker |  | 9375 | 9 | rust decl src/error.rs:1009 |  |  | 0.540 |
| ns | 9378 |  | 161 | CI job roster and toolchain matrix | 7.8 |  | 0.534 |
| walker |  | 9394 | 19 | rust decl src/error.rs:984 |  |  | 0.534 |
| walker |  | 9415 | 21 | rust decl src/error.rs:973 |  |  | 0.534 |
| walker |  | 9429 | 14 | rust body src/nightly.rs:45 |  |  | 0.534 |
| walker |  | 9566 | 137 | README.md section #8 |  |  | 0.534 |
| ns | 9570 |  | 192 | CI commands — the canonical build/test/lint invocations | 7.9 | 7.8 | 0.530 |
| walker |  | 9639 | 73 | rust body build.rs:199 |  |  | 0.530 |
| ns | 9658 |  | 88 | trybuild UI harness — tests/compiletest.rs in full | 8.1 |  | 0.527 |
| ns | 9740 |  | 82 | tests/ui file listing | 8.2 | 8.1 | 0.535 |
| ns | 9849 |  | 109 | One complete compile-fail pair — wrong-interpolation | 8.3 | 8.2 | 0.531 |
| walker |  | 9885 | 246 | rust decl src/error.rs:19 #1 |  |  | 0.548 |
| walker |  | 9904 | 19 | rust decl src/error.rs:431 |  |  | 0.548 |
| walker |  | 9933 | 29 | rust decl src/error.rs:457 |  |  | 0.548 |
| walker |  | 9964 | 31 | rust decl src/error.rs:482 |  |  | 0.548 |
| ns | 9990 |  | 141 | Shared test helpers in tests/common and tests/drop | 8.4 | 4.4 | 0.542 |
| walker |  | 9995 | 31 | rust decl src/error.rs:490 |  |  | 0.542 |
