Score(3000)=0.601 I=0.823 C=0.438 ns_rows≤3K=20/63 grid(1000/1442/2080/3000/4327/6240/9000)=0.639/0.697/0.669/0.601/0.520/0.432/0.540

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
| walker |  | 258 | 69 | rust names build.rs |  |  | 0.549 |
| ns | 259 |  | 91 | What `anyhow::Error` is — a Box<dyn Error> that must be Send + Sync | 1.4 |  | 0.486 |
| walker |  | 302 | 44 | headings outline in README.md |  |  | 0.494 |
| ns | 333 |  | 74 | …and the other two guarantees: always a backtrace, one word wide | 1.5 | 1.4 | 0.454 |
| ns | 417 |  | 84 | Cargo.toml package tail — MSRV, edition, license, links | 1.6 |  | 0.416 |
| ns | 507 |  | 90 | tests/ inventory | 1.7 |  | 0.339 |
| walker |  | 527 | 225 | rust names src/lib.rs |  |  | 0.347 |
| walker |  | 545 | 18 | rust decl src/lib.rs:648 |  |  | 0.349 |
| walker |  | 571 | 26 | rust decl src/lib.rs:389 |  |  | 0.351 |
| walker |  | 620 | 49 | rust decl src/lib.rs:413 |  |  | 0.357 |
| ns | 645 |  | 138 | Cargo features — std default, optional backtrace | 1.8 |  | 0.329 |
| walker |  | 676 | 56 | rust decl src/lib.rs:616 |  |  | 0.329 |
| walker |  | 699 | 23 | rust decl src/lib.rs:618 |  |  | 0.330 |
| walker |  | 733 | 34 | rust decl src/lib.rs:624 |  |  | 0.332 |
| walker |  | 769 | 36 | rust names src/macros.rs |  |  | 0.332 |
| ns | 793 |  | 148 | README lede, install snippet, and section map | 1.9 |  | 0.340 |
| walker |  | 934 | 165 | [package] in Cargo.toml |  |  | 0.639 |
| walker |  | 979 | 45 | rust decl src/lib.rs:278 |  |  | 0.639 |
| ns | 1009 |  | 216 | lib.rs crate attributes and module declarations | 1.10 |  | 0.586 |
| walker |  | 1023 | 44 | rust names src/fmt.rs |  |  | 0.586 |
| walker |  | 1097 | 74 | README.md section #0 |  |  | 0.621 |
| walker |  | 1148 | 51 | rust names src/ensure.rs |  |  | 0.621 |
| walker |  | 1175 | 27 | rust decl src/ensure.rs:58 |  |  | 0.621 |
| walker |  | 1206 | 31 | rust decl src/ensure.rs:35 |  |  | 0.621 |
| walker |  | 1216 | 10 | rust body src/lib.rs:648 |  |  | 0.622 |
| ns | 1244 |  | 235 | Complete roster of exported items in lib.rs | 2.1 |  | 0.645 |
| walker |  | 1326 | 110 | [dependencies] in Cargo.toml |  |  | 0.696 |
| walker |  | 1389 | 63 | rust names src/kind.rs |  |  | 0.696 |
| walker |  | 1409 | 20 | rust decl src/kind.rs:99 |  |  | 0.697 |
| walker |  | 1435 | 26 | rust decl src/kind.rs:67 |  |  | 0.697 |
| ns | 1452 |  | 208 | impl Error — roster of every public method | 2.2 |  | 0.650 |
| walker |  | 1461 | 26 | rust decl src/kind.rs:89 |  |  | 0.650 |
| walker |  | 1513 | 52 | rust decl src/kind.rs:114 |  |  | 0.650 |
| walker |  | 1519 | 6 | rust decl src/kind.rs:116 |  |  | 0.650 |
| walker |  | 1549 | 30 | rust decl src/kind.rs:90 |  |  | 0.650 |
| walker |  | 1586 | 37 | rust decl src/kind.rs:68 |  |  | 0.650 |
| ns | 1612 |  | 160 | Context trait — both method signatures with bounds | 2.3 | 2.1 | 0.640 |
| walker |  | 1676 | 90 | listing of 'tests' |  |  | 0.735 |
| ns | 1778 |  | 166 | Display representations — `{}` and `{:#}`, with sample output | 2.4 | 1.4 | 0.713 |
| walker |  | 1787 | 111 | rust names src/chain.rs |  |  | 0.714 |
| walker |  | 1807 | 20 | rust decl src/chain.rs:76 |  |  | 0.714 |
| walker |  | 1837 | 30 | rust decl src/chain.rs:26 |  |  | 0.714 |
| walker |  | 1843 | 6 | rust decl src/chain.rs:27 |  |  | 0.714 |
| ns | 1915 |  | 137 | Debug representations — `{:?}` and `{:#?}` | 2.5 | 2.4 | 0.690 |
| ns | 2031 |  | 116 | Context trait doc — sealed, and outermost-first cause printing | 2.6 | 2.3 | 0.669 |
| walker |  | 2087 | 244 | rust names src/error.rs |  |  | 0.670 |
| walker |  | 2104 | 17 | rust decl src/error.rs:731 |  |  | 0.670 |
| walker |  | 2132 | 28 | rust decl src/error.rs:719 |  |  | 0.670 |
| walker |  | 2160 | 28 | rust decl src/error.rs:725 |  |  | 0.670 |
| walker |  | 2205 | 45 | rust decl src/error.rs:712 |  |  | 0.670 |
| ns | 2212 |  | 181 | Context + downcasting — the guarantee, in both directions | 2.7 | 2.6 | 0.642 |
| walker |  | 2271 | 66 | rust decl src/error.rs:703 |  |  | 0.642 |
| walker |  | 2340 | 69 | rust decl src/error.rs:691 |  |  | 0.642 |
| walker |  | 2346 | 6 | rust decl src/error.rs:696 |  |  | 0.642 |
| ns | 2360 |  | 148 | Result alias and Ok() helper semantics | 2.8 | 2.1 | 0.624 |
| walker |  | 2374 | 28 | rust decl src/error.rs:775 |  |  | 0.624 |
| walker |  | 2408 | 34 | rust decl src/chain.rs:93 |  |  | 0.624 |
| walker |  | 2531 | 123 | rust names src/backtrace.rs |  |  | 0.624 |
| ns | 2536 |  | 176 | no_std support contract | 2.9 |  | 0.607 |
| walker |  | 2550 | 19 | rust decl src/backtrace.rs:7 |  |  | 0.607 |
| walker |  | 2585 | 35 | rust decl src/chain.rs:10 |  |  | 0.607 |
| walker |  | 2620 | 35 | rust decl src/fmt.rs:69 |  |  | 0.608 |
| walker |  | 2660 | 40 | rust decl src/backtrace.rs:10 |  |  | 0.608 |
| walker |  | 2672 | 12 | rust doc src/lib.rs:618 |  |  | 0.614 |
| walker |  | 2716 | 44 | rust decl src/chain.rs:56 |  |  | 0.614 |
| walker |  | 2761 | 45 | rust decl src/backtrace.rs:31 |  |  | 0.614 |
| ns | 2768 |  | 232 | Trait impls on Error — complete list | 2.10 | 2.2 | 0.600 |
| walker |  | 2933 | 172 | rust names src/context.rs |  |  | 0.601 |
| walker |  | 2981 | 48 | rust decl src/context.rs:128 |  |  | 0.601 |
| ns | 3002 |  | 234 | What bail! and ensure! mean | 3.1 |  | 0.582 |
| walker |  | 3033 | 52 | rust decl src/context.rs:90 |  |  | 0.582 |
| walker |  | 3062 | 29 | rust decl src/context.rs:91 |  |  | 0.582 |
| walker |  | 3119 | 57 | rust decl src/context.rs:115 |  |  | 0.582 |
| ns | 3183 |  | 181 | What anyhow! constructs | 3.2 | 3.1 | 0.567 |
| walker |  | 3191 | 72 | rust decl src/context.rs:152 |  |  | 0.567 |
| walker |  | 3203 | 12 | rust decl src/context.rs:160 |  |  | 0.567 |
| walker |  | 3245 | 42 | rust decl src/context.rs:103 |  |  | 0.567 |
| walker |  | 3329 | 84 | rust decl src/context.rs:42 |  |  | 0.567 |
| walker |  | 3358 | 29 | rust decl src/context.rs:46 |  |  | 0.567 |
| walker |  | 3400 | 42 | rust decl src/context.rs:58 |  |  | 0.567 |
| ns | 3438 |  | 255 | Every exported macro_rules!, with its export attributes | 3.3 | 3.1 | 0.543 |
| walker |  | 3485 | 85 | rust decl src/context.rs:137 |  |  | 0.543 |
| walker |  | 3497 | 12 | rust decl src/context.rs:146 |  |  | 0.543 |
| walker |  | 3526 | 29 | rust decl src/context.rs:180 |  |  | 0.543 |
| walker |  | 3574 | 48 | rust decl src/context.rs:168 |  |  | 0.543 |
| walker |  | 3622 | 48 | rust decl src/error.rs:787 |  |  | 0.543 |
| ns | 3660 |  | 222 | anyhow! expansion — the three arms | 3.4 | 3.3 | 0.522 |
| walker |  | 3670 | 48 | rust decl src/error.rs:798 |  |  | 0.522 |
| ns | 3796 |  | 136 | bail! body | 3.5 | 3.3 | 0.512 |
| walker |  | 3844 | 174 | rust names src/nightly.rs |  |  | 0.512 |
| walker |  | 3855 | 11 | rust decl src/nightly.rs:38 |  |  | 0.512 |
| walker |  | 3867 | 12 | rust body src/nightly.rs:56 |  |  | 0.512 |
| walker |  | 3880 | 13 | rust body src/nightly.rs:41 |  |  | 0.512 |
| walker |  | 3904 | 24 | rust decl src/nightly.rs:45 |  |  | 0.512 |
| walker |  | 3918 | 14 | rust body src/nightly.rs:52 |  |  | 0.512 |
| ns | 3957 |  | 161 | ensure! expansion — the fuel-limited parser call | 3.6 | 3.3 | 0.500 |
| walker |  | 4084 | 166 | rust decl src/macros.rs:56 |  |  | 0.528 |
| ns | 4126 |  | 169 | kind.rs — why autoref tagged dispatch | 3.7 | 3.4 | 0.520 |
| walker |  | 4133 | 49 | rust decl src/fmt.rs:75 |  |  | 0.520 |
| walker |  | 4258 | 125 | manifest config in Cargo.toml |  |  | 0.520 |
| walker |  | 4309 | 51 | rust decl src/backtrace.rs:24 |  |  | 0.520 |
| walker |  | 4361 | 52 | rust decl src/backtrace.rs:17 |  |  | 0.520 |
| ns | 4375 |  | 249 | kind.rs — the three kinds and their new() constructors | 3.8 | 3.7 | 0.515 |
| walker |  | 4571 | 210 | rust names src/wrapper.rs |  |  | 0.516 |
| walker |  | 4578 | 7 | rust decl src/wrapper.rs:33 |  |  | 0.516 |
| walker |  | 4587 | 9 | rust decl src/wrapper.rs:10 |  |  | 0.516 |
| walker |  | 4614 | 27 | rust decl src/wrapper.rs:56 |  |  | 0.516 |
| ns | 4627 |  | 252 | ensure.rs runtime side — BothDebug / NotBothDebug dispatch | 3.9 | 3.6 | 0.502 |
| walker |  | 4662 | 48 | rust decl src/wrapper.rs:36 |  |  | 0.502 |
| walker |  | 4710 | 48 | rust decl src/wrapper.rs:45 |  |  | 0.502 |
| walker |  | 4758 | 48 | rust decl src/wrapper.rs:60 |  |  | 0.502 |
| ns | 4798 |  | 171 | __parse_ensure! extent, first rule, and catch-all | 3.10 | 3.6 | 0.492 |
| walker |  | 4806 | 48 | rust decl src/wrapper.rs:67 |  |  | 0.492 |
| walker |  | 4856 | 50 | rust decl src/wrapper.rs:13 |  |  | 0.492 |
| walker |  | 4906 | 50 | rust decl src/wrapper.rs:22 |  |  | 0.492 |
| walker |  | 4976 | 70 | rust decl src/wrapper.rs:74 |  |  | 0.492 |
| walker |  | 4988 | 12 | rust decl src/wrapper.rs:80 |  |  | 0.492 |
| ns | 4995 |  | 197 | lib.rs __private — the macro-facing surface | 3.11 | 3.3 | 0.482 |
| ns | 5156 |  | 161 | test_fmt.rs — the f/g/h fixture and expected `{:#}` strings | 4.1 | 2.4 | 0.476 |
| ns | 5295 |  | 139 | test_fmt.rs — expected `{:?}` strings, including numbered causes | 4.2 | 4.1 | 0.465 |
| walker |  | 5304 | 316 | rust names src/ptr.rs |  |  | 0.465 |
| walker |  | 5315 | 11 | rust decl src/ptr.rs:181 |  |  | 0.465 |
| walker |  | 5328 | 13 | rust decl src/ptr.rs:185 |  |  | 0.465 |
| walker |  | 5349 | 21 | rust decl src/ptr.rs:174 |  |  | 0.465 |
| walker |  | 5388 | 39 | rust decl src/ptr.rs:19 |  |  | 0.465 |
| walker |  | 5427 | 39 | rust decl src/ptr.rs:74 |  |  | 0.465 |
| ns | 5454 |  | 159 | test_repr.rs — size, null-pointer-optimization, autotrait assertions | 4.3 | 1.5 | 0.455 |
| walker |  | 5466 | 39 | rust decl src/ptr.rs:135 |  |  | 0.455 |
| walker |  | 5511 | 45 | rust decl src/ptr.rs:5 |  |  | 0.456 |
| walker |  | 5570 | 59 | rust decl src/ptr.rs:63 |  |  | 0.456 |
| walker |  | 5630 | 60 | rust decl src/ptr.rs:124 |  |  | 0.457 |
| ns | 5633 |  | 179 | test_downcast.rs — what a bail!'d error downcasts to | 4.4 | 3.4 | 0.446 |
| walker |  | 5746 | 116 | rust decl src/ptr.rs:144 |  |  | 0.446 |
| ns | 5807 |  | 174 | test_chain.rs — cause iteration order | 4.5 |  | 0.439 |
| walker |  | 5869 | 123 | rust decl src/ptr.rs:28 |  |  | 0.439 |
| ns | 6002 |  | 195 | test_context.rs — downcast_ref through a three-level context chain | 4.6 | 2.7 | 0.432 |
| walker |  | 6017 | 148 | rust decl src/ptr.rs:83 |  |  | 0.432 |
| walker |  | 6082 | 65 | rust decl src/fmt.rs:6 |  |  | 0.432 |
| walker |  | 6150 | 68 | rust decl src/chain.rs:35 |  |  | 0.432 |
| ns | 6252 |  | 250 | ErrorVTable — the hand-rolled vtable layout | 5.1 |  | 0.426 |
| walker |  | 6402 | 252 | rust decl src/macros.rs:202 |  |  | 0.465 |
| ns | 6483 |  | 231 | ErrorImpl, ContextError, and the vtable reader | 5.2 | 5.1 | 0.455 |
| walker |  | 6485 | 83 | rust decl src/backtrace.rs:59 |  |  | 0.455 |
| walker |  | 6572 | 87 | rust decl src/backtrace.rs:38 |  |  | 0.456 |
| ns | 6615 |  | 132 | Error::construct — allocation and type erasure | 5.3 | 5.2 | 0.449 |
| walker |  | 6658 | 86 | rust decl src/backtrace.rs:48 |  |  | 0.449 |
| ns | 6718 |  | 103 | impl Error — roster of the non-public fns | 5.4 | 5.3 | 0.446 |
| walker |  | 6857 | 199 | rust names src/error.rs #1 |  |  | 0.447 |
| walker |  | 6878 | 21 | rust decl src/error.rs:809 |  |  | 0.447 |
| ns | 6889 |  | 171 | Vtable function roster in error.rs | 5.5 | 5.1 | 0.460 |
| walker |  | 6899 | 21 | rust decl src/error.rs:876 |  |  | 0.460 |
| walker |  | 6920 | 21 | rust decl src/error.rs:892 |  |  | 0.460 |
| walker |  | 6960 | 40 | rust decl src/error.rs:828 |  |  | 0.460 |
| walker |  | 7011 | 51 | rust decl src/error.rs:838 |  |  | 0.460 |
| ns | 7040 |  | 151 | ErrorImpl methods and trait impls | 5.6 | 5.2 | 0.455 |
| walker |  | 7062 | 51 | rust decl src/error.rs:858 |  |  | 0.455 |
| walker |  | 7137 | 75 | rust decl src/error.rs:915 |  |  | 0.455 |
| walker |  | 7236 | 99 | rust decl src/chain.rs:15 |  |  | 0.456 |
| ns | 7275 |  | 235 | ptr.rs — Own / Ref / Mut / CastTo declarations | 5.7 |  | 0.481 |
| ns | 7440 |  | 165 | chain.rs — Chain and ChainState | 6.1 |  | 0.491 |
| walker |  | 7521 | 285 | rust module doc src/lib.rs |  |  | 0.491 |
| walker |  | 7529 | 8 | rust body src/kind.rs:90 |  |  | 0.491 |
| walker |  | 7536 | 7 | rust body src/lib.rs:280 |  |  | 0.491 |
| walker |  | 7540 | 4 | listing of 'tests/common' |  |  | 0.491 |
| walker |  | 7544 | 4 | listing of 'tests/drop' |  |  | 0.491 |
| ns | 7557 |  | 117 | chain.rs — every trait impl on Chain | 6.2 | 6.1 | 0.498 |
| walker |  | 7748 | 204 | rust decl src/error.rs:19 |  |  | 0.505 |
| ns | 7782 |  | 225 | wrapper.rs — the three error adapters and their impls | 6.3 |  | 0.516 |
| walker |  | 7786 | 38 | rust decl src/error.rs:137 |  |  | 0.516 |
| walker |  | 7834 | 48 | rust decl src/error.rs:75 |  |  | 0.516 |
| walker |  | 7902 | 68 | rust decl src/error.rs:27 |  |  | 0.516 |
| walker |  | 7939 | 37 | rust decl src/error.rs:197 |  |  | 0.516 |
| ns | 7959 |  | 177 | context.rs — where .context() is implemented | 6.4 | 2.3 | 0.511 |
| walker |  | 7978 | 39 | rust decl src/error.rs:169 |  |  | 0.511 |
| walker |  | 8037 | 59 | rust decl src/error.rs:145 |  |  | 0.511 |
| walker |  | 8105 | 68 | rust decl src/error.rs:258 |  |  | 0.511 |
| ns | 8133 |  | 174 | context.rs — ContextError impls, Quoted, and the Sealed trait | 6.5 | 6.4 | 0.511 |
| walker |  | 8187 | 82 | listing of 'tests/ui' |  |  | 0.512 |
| ns | 8261 |  | 128 | fmt.rs — the rendering entry points | 6.6 | 5.6 | 0.517 |
| ns | 8413 |  | 152 | build.rs — the complete custom-cfg vocabulary | 7.1 |  | 0.513 |
| walker |  | 8491 | 304 | rust module doc src/lib.rs #1 |  |  | 0.513 |
| walker |  | 8521 | 30 | rust doc src/lib.rs:624 |  |  | 0.519 |
| ns | 8527 |  | 114 | build.rs — which compiler turns each cfg on | 7.2 | 7.1 | 0.517 |
| ns | 8637 |  | 110 | backtrace.rs — the three definitions of Backtrace | 7.3 |  | 0.514 |
| walker |  | 8779 | 258 | rust names src/error.rs #2 |  |  | 0.531 |
| ns | 8813 |  | 176 | backtrace.rs — impl_backtrace! and backtrace!(), both arms each | 7.4 | 7.3 | 0.538 |
| walker |  | 8825 | 46 | rust decl src/error.rs:1047 |  |  | 0.538 |
| walker |  | 8832 | 7 | rust decl src/error.rs:1049 |  |  | 0.538 |
| walker |  | 8876 | 44 | rust decl src/error.rs:1055 |  |  | 0.538 |
| walker |  | 8883 | 7 | rust decl src/error.rs:1057 |  |  | 0.538 |
| walker |  | 8927 | 44 | rust decl src/error.rs:1063 |  |  | 0.538 |
| walker |  | 8934 | 7 | rust decl src/error.rs:1065 |  |  | 0.538 |
| ns | 8954 |  | 141 | backtrace.rs — backtrace_if_absent! and the vendored capture module | 7.5 | 7.3 | 0.540 |
| walker |  | 8983 | 49 | rust decl src/error.rs:1078 |  |  | 0.540 |
| walker |  | 9034 | 51 | rust decl src/error.rs:1071 |  |  | 0.540 |
| walker |  | 9067 | 33 | rust decl src/error.rs:951 |  |  | 0.542 |
| ns | 9086 |  | 132 | nightly.rs — the build probe and generic-member-access shims | 7.6 | 7.1 | 0.539 |
| walker |  | 9120 | 53 | rust decl src/error.rs:1029 |  |  | 0.539 |
| walker |  | 9173 | 53 | rust decl src/error.rs:1038 |  |  | 0.539 |
| ns | 9217 |  | 131 | Toolchain pin and the no_std check-crate | 7.7 |  | 0.533 |
| walker |  | 9252 | 79 | rust decl src/error.rs:1015 |  |  | 0.533 |
| walker |  | 9265 | 13 | rust decl src/error.rs:1023 |  |  | 0.533 |
| walker |  | 9348 | 83 | rust decl src/error.rs:933 |  |  | 0.542 |
| ns | 9378 |  | 161 | CI job roster and toolchain matrix | 7.8 |  | 0.536 |
| walker |  | 9465 | 117 | rust decl src/error.rs:966 |  |  | 0.541 |
| walker |  | 9474 | 9 | rust decl src/error.rs:1009 |  |  | 0.541 |
| walker |  | 9493 | 19 | rust decl src/error.rs:984 |  |  | 0.541 |
| walker |  | 9514 | 21 | rust decl src/error.rs:973 |  |  | 0.541 |
| walker |  | 9528 | 14 | rust body src/nightly.rs:45 |  |  | 0.541 |
| ns | 9570 |  | 192 | CI commands — the canonical build/test/lint invocations | 7.9 | 7.8 | 0.537 |
| ns | 9658 |  | 88 | trybuild UI harness — tests/compiletest.rs in full | 8.1 |  | 0.534 |
| walker |  | 9665 | 137 | README.md section #8 |  |  | 0.534 |
| walker |  | 9738 | 73 | rust body build.rs:199 |  |  | 0.534 |
| ns | 9740 |  | 82 | tests/ui file listing | 8.2 | 8.1 | 0.542 |
| ns | 9849 |  | 109 | One complete compile-fail pair — wrong-interpolation | 8.3 | 8.2 | 0.537 |
| walker |  | 9984 | 246 | rust decl src/error.rs:19 #1 |  |  | 0.554 |
| ns | 9990 |  | 141 | Shared test helpers in tests/common and tests/drop | 8.4 | 4.4 | 0.548 |
