Score(3000)=0.613 I=0.843 C=0.446 ns_rows≤3K=20/63 grid(1000/1442/2080/3000/4327/6240/9000)=0.751/0.808/0.696/0.613/0.574/0.502/0.510

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 34 | 34 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 83 | 49 | Fs::DirListing { dir: src } |  |  | 0.000 |
| ns | 85 |  | 85 | Crate identity — name, version, description | 1.1 |  | 0.000 |
| walker |  | 91 | 8 | Fs::DirListing { dir: .github } |  |  | 0.000 |
| walker |  | 95 | 4 | Fs::DirListing { dir: .github/workflows } |  |  | 0.000 |
| ns | 119 |  | 34 | Repository root listing | 1.2 |  | 0.523 |
| walker |  | 162 | 67 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.527 |
| ns | 168 |  | 49 | src/ module inventory | 1.3 |  | 0.548 |
| walker |  | 189 | 27 | Toml::Operational { file: Cargo.toml } |  |  | 0.549 |
| walker |  | 233 | 44 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.558 |
| ns | 259 |  | 91 | What `anyhow::Error` is — a Box<dyn Error> that must be Send + Sync | 1.4 |  | 0.494 |
| ns | 333 |  | 74 | …and the other two guarantees: always a backtrace, one word wide | 1.5 | 1.4 | 0.454 |
| walker |  | 398 | 165 | Toml::Identity { file: Cargo.toml } |  |  | 0.837 |
| ns | 417 |  | 84 | Cargo.toml package tail — MSRV, edition, license, links | 1.6 |  | 0.844 |
| walker |  | 472 | 74 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.850 |
| ns | 507 |  | 90 | tests/ inventory | 1.7 |  | 0.693 |
| walker |  | 582 | 110 | Toml::Dependencies { file: Cargo.toml } |  |  | 0.703 |
| ns | 645 |  | 138 | Cargo features — std default, optional backtrace | 1.8 |  | 0.723 |
| walker |  | 651 | 69 | Code::CodeKey { rung: Names, file: build.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.723 |
| ns | 793 |  | 148 | README lede, install snippet, and section map | 1.9 |  | 0.737 |
| walker |  | 876 | 225 | Code::CodeKey { rung: Names, file: src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.743 |
| walker |  | 894 | 18 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 9, sub: 0, line: 648 } |  |  | 0.744 |
| walker |  | 920 | 26 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 3, sub: 0, line: 389 } |  |  | 0.746 |
| walker |  | 969 | 49 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 4, sub: 0, line: 413 } |  |  | 0.751 |
| ns | 1009 |  | 216 | lib.rs crate attributes and module declarations | 1.10 |  | 0.680 |
| walker |  | 1025 | 56 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 6, sub: 0, line: 616 } |  |  | 0.680 |
| walker |  | 1048 | 23 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 7, sub: 0, line: 618 } |  |  | 0.681 |
| walker |  | 1082 | 34 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 8, sub: 0, line: 624 } |  |  | 0.682 |
| walker |  | 1094 | 12 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 7, sub: 0, line: 618 } |  |  | 0.683 |
| walker |  | 1139 | 45 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 1, sub: 0, line: 278 } |  |  | 0.683 |
| walker |  | 1169 | 30 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 8, sub: 0, line: 624 } |  |  | 0.685 |
| walker |  | 1179 | 10 | Code::CodeKey { rung: Body, file: src/lib.rs, decl: 9, sub: 0, line: 648 } |  |  | 0.686 |
| ns | 1244 |  | 235 | Complete roster of exported items in lib.rs | 2.1 |  | 0.699 |
| walker |  | 1269 | 90 | Fs::DirListing { dir: tests } |  |  | 0.807 |
| walker |  | 1305 | 36 | Code::CodeKey { rung: Names, file: src/macros.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.808 |
| walker |  | 1349 | 44 | Code::CodeKey { rung: Names, file: src/fmt.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.808 |
| walker |  | 1384 | 35 | Code::CodeKey { rung: Decl, file: src/fmt.rs, decl: 4, sub: 0, line: 69 } |  |  | 0.808 |
| walker |  | 1435 | 51 | Code::CodeKey { rung: Names, file: src/ensure.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.808 |
| ns | 1452 |  | 208 | impl Error — roster of every public method | 2.2 |  | 0.754 |
| walker |  | 1462 | 27 | Code::CodeKey { rung: Decl, file: src/ensure.rs, decl: 2, sub: 0, line: 58 } |  |  | 0.754 |
| walker |  | 1493 | 31 | Code::CodeKey { rung: Decl, file: src/ensure.rs, decl: 1, sub: 0, line: 35 } |  |  | 0.754 |
| ns | 1612 |  | 160 | Context trait — both method signatures with bounds | 2.3 | 2.1 | 0.761 |
| walker |  | 1659 | 166 | Code::CodeKey { rung: Decl, file: src/macros.rs, decl: 1, sub: 0, line: 56 } |  |  | 0.764 |
| walker |  | 1708 | 49 | Code::CodeKey { rung: Decl, file: src/fmt.rs, decl: 5, sub: 0, line: 75 } |  |  | 0.764 |
| ns | 1778 |  | 166 | Display representations — `{}` and `{:#}`, with sample output | 2.4 | 1.4 | 0.742 |
| walker |  | 1833 | 125 | Toml::Config { file: Cargo.toml } |  |  | 0.742 |
| walker |  | 1896 | 63 | Code::CodeKey { rung: Names, file: src/kind.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.742 |
| ns | 1915 |  | 137 | Debug representations — `{:?}` and `{:#?}` | 2.5 | 2.4 | 0.716 |
| walker |  | 1916 | 20 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 7, sub: 0, line: 99 } |  |  | 0.716 |
| walker |  | 1942 | 26 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 2, sub: 0, line: 67 } |  |  | 0.716 |
| walker |  | 1968 | 26 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 5, sub: 0, line: 89 } |  |  | 0.717 |
| walker |  | 1998 | 30 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 6, sub: 0, line: 90 } |  |  | 0.717 |
| ns | 2031 |  | 116 | Context trait doc — sealed, and outermost-first cause printing | 2.6 | 2.3 | 0.696 |
| walker |  | 2035 | 37 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 3, sub: 0, line: 68 } |  |  | 0.696 |
| walker |  | 2087 | 52 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 8, sub: 0, line: 114 } |  |  | 0.696 |
| walker |  | 2093 | 6 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 9, sub: 0, line: 116 } |  |  | 0.696 |
| walker |  | 2101 | 8 | Code::CodeKey { rung: Body, file: src/kind.rs, decl: 6, sub: 0, line: 90 } |  |  | 0.696 |
| walker |  | 2166 | 65 | Code::CodeKey { rung: Decl, file: src/fmt.rs, decl: 1, sub: 0, line: 6 } |  |  | 0.697 |
| ns | 2212 |  | 181 | Context + downcasting — the guarantee, in both directions | 2.7 | 2.6 | 0.667 |
| walker |  | 2356 | 190 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 4, sub: 0, line: 413 } |  |  | 0.667 |
| ns | 2360 |  | 148 | Result alias and Ok() helper semantics | 2.8 | 2.1 | 0.648 |
| ns | 2536 |  | 176 | no_std support contract | 2.9 |  | 0.631 |
| walker |  | 2608 | 252 | Code::CodeKey { rung: Decl, file: src/macros.rs, decl: 3, sub: 0, line: 202 } |  |  | 0.635 |
| walker |  | 2612 | 4 | Fs::DirListing { dir: tests/common } |  |  | 0.635 |
| walker |  | 2616 | 4 | Fs::DirListing { dir: tests/drop } |  |  | 0.635 |
| walker |  | 2623 | 7 | Code::CodeKey { rung: Body, file: src/lib.rs, decl: 2, sub: 0, line: 280 } |  |  | 0.635 |
| walker |  | 2705 | 82 | Fs::DirListing { dir: tests/ui } |  |  | 0.636 |
| ns | 2768 |  | 232 | Trait impls on Error — complete list | 2.10 | 2.2 | 0.613 |
| walker |  | 2990 | 285 | Code::CodeKey { rung: ModuleDoc, file: src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.613 |
| ns | 3002 |  | 234 | What bail! and ensure! mean | 3.1 |  | 0.594 |
| ns | 3183 |  | 181 | What anyhow! constructs | 3.2 | 3.1 | 0.579 |
| walker |  | 3294 | 304 | Code::CodeKey { rung: ModuleDoc, file: src/lib.rs, decl: 0, sub: 1, line: 0 } |  |  | 0.579 |
| ns | 3438 |  | 255 | Every exported macro_rules!, with its export attributes | 3.3 | 3.1 | 0.561 |
| walker |  | 3547 | 253 | Code::CodeKey { rung: ModuleDoc, file: src/lib.rs, decl: 0, sub: 2, line: 0 } |  |  | 0.561 |
| ns | 3660 |  | 222 | anyhow! expansion — the three arms | 3.4 | 3.3 | 0.586 |
| walker |  | 3776 | 229 | Code::CodeKey { rung: ModuleDoc, file: src/lib.rs, decl: 0, sub: 3, line: 0 } |  |  | 0.586 |
| ns | 3796 |  | 136 | bail! body | 3.5 | 3.3 | 0.597 |
| ns | 3957 |  | 161 | ensure! expansion — the fuel-limited parser call | 3.6 | 3.3 | 0.583 |
| walker |  | 4044 | 268 | Code::CodeKey { rung: ModuleDoc, file: src/lib.rs, decl: 0, sub: 4, line: 0 } |  |  | 0.583 |
| ns | 4126 |  | 169 | kind.rs — why autoref tagged dispatch | 3.7 | 3.4 | 0.574 |
| walker |  | 4356 | 312 | Code::CodeKey { rung: ModuleDoc, file: src/lib.rs, decl: 0, sub: 5, line: 0 } |  |  | 0.597 |
| ns | 4375 |  | 249 | kind.rs — the three kinds and their new() constructors | 3.8 | 3.7 | 0.589 |
| walker |  | 4467 | 111 | Code::CodeKey { rung: Names, file: src/chain.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.589 |
| walker |  | 4487 | 20 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 10, sub: 0, line: 76 } |  |  | 0.589 |
| walker |  | 4517 | 30 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 3, sub: 0, line: 26 } |  |  | 0.589 |
| walker |  | 4523 | 6 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 4, sub: 0, line: 27 } |  |  | 0.589 |
| walker |  | 4557 | 34 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 12, sub: 0, line: 93 } |  |  | 0.589 |
| walker |  | 4592 | 35 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.589 |
| ns | 4627 |  | 252 | ensure.rs runtime side — BothDebug / NotBothDebug dispatch | 3.9 | 3.6 | 0.573 |
| walker |  | 4636 | 44 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 8, sub: 0, line: 56 } |  |  | 0.573 |
| walker |  | 4704 | 68 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 5, sub: 0, line: 35 } |  |  | 0.573 |
| ns | 4798 |  | 171 | __parse_ensure! extent, first rule, and catch-all | 3.10 | 3.6 | 0.562 |
| walker |  | 4803 | 99 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 2, sub: 0, line: 15 } |  |  | 0.563 |
| ns | 4995 |  | 197 | lib.rs __private — the macro-facing surface | 3.11 | 3.3 | 0.552 |
| walker |  | 5047 | 244 | Code::CodeKey { rung: Names, file: src/error.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.555 |
| walker |  | 5064 | 17 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 31, sub: 0, line: 731 } |  |  | 0.555 |
| walker |  | 5092 | 28 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 27, sub: 0, line: 719 } |  |  | 0.555 |
| walker |  | 5120 | 28 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 29, sub: 0, line: 725 } |  |  | 0.555 |
| ns | 5156 |  | 161 | test_fmt.rs — the f/g/h fixture and expected `{:#}` strings | 4.1 | 2.4 | 0.548 |
| walker |  | 5165 | 45 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 25, sub: 0, line: 712 } |  |  | 0.548 |
| walker |  | 5231 | 66 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 23, sub: 0, line: 703 } |  |  | 0.550 |
| ns | 5295 |  | 139 | test_fmt.rs — expected `{:?}` strings, including numbered causes | 4.2 | 4.1 | 0.538 |
| walker |  | 5300 | 69 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 21, sub: 0, line: 691 } |  |  | 0.538 |
| walker |  | 5306 | 6 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 22, sub: 0, line: 696 } |  |  | 0.538 |
| walker |  | 5334 | 28 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 36, sub: 0, line: 775 } |  |  | 0.538 |
| walker |  | 5382 | 48 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 37, sub: 0, line: 787 } |  |  | 0.538 |
| walker |  | 5430 | 48 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 38, sub: 0, line: 798 } |  |  | 0.538 |
| ns | 5454 |  | 159 | test_repr.rs — size, null-pointer-optimization, autotrait assertions | 4.3 | 1.5 | 0.527 |
| ns | 5633 |  | 179 | test_downcast.rs — what a bail!'d error downcasts to | 4.4 | 3.4 | 0.514 |
| walker |  | 5714 | 284 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 9, sub: 0, line: 648 } |  |  | 0.517 |
| ns | 5807 |  | 174 | test_chain.rs — cause iteration order | 4.5 |  | 0.509 |
| walker |  | 5837 | 123 | Code::CodeKey { rung: Names, file: src/backtrace.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.510 |
| walker |  | 5856 | 19 | Code::CodeKey { rung: Decl, file: src/backtrace.rs, decl: 1, sub: 0, line: 7 } |  |  | 0.510 |
| walker |  | 5896 | 40 | Code::CodeKey { rung: Decl, file: src/backtrace.rs, decl: 2, sub: 0, line: 10 } |  |  | 0.510 |
| walker |  | 5941 | 45 | Code::CodeKey { rung: Decl, file: src/backtrace.rs, decl: 5, sub: 0, line: 31 } |  |  | 0.510 |
| walker |  | 5992 | 51 | Code::CodeKey { rung: Decl, file: src/backtrace.rs, decl: 4, sub: 0, line: 24 } |  |  | 0.510 |
| ns | 6002 |  | 195 | test_context.rs — downcast_ref through a three-level context chain | 4.6 | 2.7 | 0.501 |
| walker |  | 6044 | 52 | Code::CodeKey { rung: Decl, file: src/backtrace.rs, decl: 3, sub: 0, line: 17 } |  |  | 0.502 |
| walker |  | 6127 | 83 | Code::CodeKey { rung: Decl, file: src/backtrace.rs, decl: 8, sub: 0, line: 59 } |  |  | 0.502 |
| walker |  | 6214 | 87 | Code::CodeKey { rung: Decl, file: src/backtrace.rs, decl: 6, sub: 0, line: 38 } |  |  | 0.502 |
| ns | 6252 |  | 250 | ErrorVTable — the hand-rolled vtable layout | 5.1 |  | 0.494 |
| walker |  | 6300 | 86 | Code::CodeKey { rung: Decl, file: src/backtrace.rs, decl: 7, sub: 0, line: 48 } |  |  | 0.494 |
| ns | 6483 |  | 231 | ErrorImpl, ContextError, and the vtable reader | 5.2 | 5.1 | 0.484 |
| walker |  | 6504 | 204 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 1, sub: 0, line: 19 } |  |  | 0.486 |
| walker |  | 6542 | 38 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 4, sub: 0, line: 137 } |  |  | 0.486 |
| walker |  | 6590 | 48 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 3, sub: 0, line: 75 } |  |  | 0.486 |
| ns | 6615 |  | 132 | Error::construct — allocation and type erasure | 5.3 | 5.2 | 0.479 |
| walker |  | 6658 | 68 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 2, sub: 0, line: 27 } |  |  | 0.479 |
| walker |  | 6695 | 37 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 7, sub: 0, line: 197 } |  |  | 0.479 |
| ns | 6718 |  | 103 | impl Error — roster of the non-public fns | 5.4 | 5.3 | 0.482 |
| walker |  | 6734 | 39 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 6, sub: 0, line: 169 } |  |  | 0.482 |
| walker |  | 6793 | 59 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 5, sub: 0, line: 145 } |  |  | 0.482 |
| walker |  | 6861 | 68 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 9, sub: 0, line: 258 } |  |  | 0.482 |
| ns | 6889 |  | 171 | Vtable function roster in error.rs | 5.5 | 5.1 | 0.479 |
| walker |  | 6976 | 115 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 8, sub: 0, line: 225 } |  |  | 0.479 |
| ns | 7040 |  | 151 | ErrorImpl methods and trait impls | 5.6 | 5.2 | 0.474 |
| walker |  | 7222 | 246 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 1, sub: 1, line: 19 } |  |  | 0.497 |
| walker |  | 7241 | 19 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 11, sub: 0, line: 431 } |  |  | 0.497 |
| walker |  | 7270 | 29 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 12, sub: 0, line: 457 } |  |  | 0.497 |
| ns | 7275 |  | 235 | ptr.rs — Own / Ref / Mut / CastTo declarations | 5.7 |  | 0.484 |
| walker |  | 7301 | 31 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 14, sub: 0, line: 482 } |  |  | 0.484 |
| walker |  | 7332 | 31 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 15, sub: 0, line: 490 } |  |  | 0.484 |
| walker |  | 7363 | 31 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 16, sub: 0, line: 554 } |  |  | 0.484 |
| walker |  | 7394 | 31 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 17, sub: 0, line: 568 } |  |  | 0.484 |
| walker |  | 7431 | 37 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 13, sub: 0, line: 468 } |  |  | 0.484 |
| ns | 7440 |  | 165 | chain.rs — Chain and ChainState | 6.1 |  | 0.495 |
| walker |  | 7443 | 12 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 20, sub: 0, line: 674 } |  |  | 0.495 |
| walker |  | 7489 | 46 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 10, sub: 0, line: 370 } |  |  | 0.495 |
| ns | 7557 |  | 117 | chain.rs — every trait impl on Chain | 6.2 | 6.1 | 0.502 |
| walker |  | 7565 | 76 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 18, sub: 0, line: 598 } |  |  | 0.502 |
| walker |  | 7580 | 15 | Code::CodeKey { rung: Doc, file: src/error.rs, decl: 17, sub: 0, line: 568 } |  |  | 0.502 |
| walker |  | 7684 | 104 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 19, sub: 0, line: 640 } |  |  | 0.502 |
| walker |  | 7702 | 18 | Code::CodeKey { rung: Doc, file: src/error.rs, decl: 15, sub: 0, line: 490 } |  |  | 0.502 |
| walker |  | 7774 | 72 | Code::CodeKey { rung: Doc, file: src/error.rs, decl: 13, sub: 0, line: 468 } |  |  | 0.502 |
| ns | 7782 |  | 225 | wrapper.rs — the three error adapters and their impls | 6.3 |  | 0.494 |
| ns | 7959 |  | 177 | context.rs — where .context() is implemented | 6.4 | 2.3 | 0.489 |
| walker |  | 8126 | 352 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 5, sub: 0, line: 468 } |  |  | 0.501 |
| ns | 8133 |  | 174 | context.rs — ContextError impls, Quoted, and the Sealed trait | 6.5 | 6.4 | 0.496 |
| walker |  | 8144 | 18 | Code::CodeKey { rung: Body, file: src/kind.rs, decl: 3, sub: 0, line: 68 } |  |  | 0.496 |
| ns | 8261 |  | 128 | fmt.rs — the rendering entry points | 6.6 | 5.6 | 0.501 |
| walker |  | 8316 | 172 | Code::CodeKey { rung: Names, file: src/context.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.507 |
| walker |  | 8364 | 48 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 9, sub: 0, line: 128 } |  |  | 0.507 |
| ns | 8413 |  | 152 | build.rs — the complete custom-cfg vocabulary | 7.1 |  | 0.503 |
| walker |  | 8416 | 52 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 4, sub: 0, line: 90 } |  |  | 0.503 |
| walker |  | 8445 | 29 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 5, sub: 0, line: 91 } |  |  | 0.503 |
| walker |  | 8487 | 42 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 6, sub: 0, line: 103 } |  |  | 0.503 |
| ns | 8527 |  | 114 | build.rs — which compiler turns each cfg on | 7.2 | 7.1 | 0.500 |
| walker |  | 8544 | 57 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 7, sub: 0, line: 115 } |  |  | 0.500 |
| walker |  | 8616 | 72 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 14, sub: 0, line: 152 } |  |  | 0.500 |
| walker |  | 8628 | 12 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 16, sub: 0, line: 160 } |  |  | 0.500 |
| ns | 8637 |  | 110 | backtrace.rs — the three definitions of Backtrace | 7.3 |  | 0.498 |
| walker |  | 8712 | 84 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 1, sub: 0, line: 42 } |  |  | 0.499 |
| walker |  | 8741 | 29 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 2, sub: 0, line: 46 } |  |  | 0.499 |
| walker |  | 8783 | 42 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 3, sub: 0, line: 58 } |  |  | 0.499 |
| ns | 8813 |  | 176 | backtrace.rs — impl_backtrace! and backtrace!(), both arms each | 7.4 | 7.3 | 0.507 |
| walker |  | 8868 | 85 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 11, sub: 0, line: 137 } |  |  | 0.507 |
| walker |  | 8880 | 12 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 13, sub: 0, line: 146 } |  |  | 0.507 |
| walker |  | 8909 | 29 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 20, sub: 0, line: 180 } |  |  | 0.507 |
| ns | 8954 |  | 141 | backtrace.rs — backtrace_if_absent! and the vendored capture module | 7.5 | 7.3 | 0.510 |
| walker |  | 8957 | 48 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 18, sub: 0, line: 168 } |  |  | 0.510 |
| ns | 9086 |  | 132 | nightly.rs — the build probe and generic-member-access shims | 7.6 | 7.1 | 0.506 |
| walker |  | 9131 | 174 | Code::CodeKey { rung: Names, file: src/nightly.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.508 |
| walker |  | 9142 | 11 | Code::CodeKey { rung: Decl, file: src/nightly.rs, decl: 2, sub: 0, line: 38 } |  |  | 0.508 |
| walker |  | 9154 | 12 | Code::CodeKey { rung: Body, file: src/nightly.rs, decl: 6, sub: 0, line: 56 } |  |  | 0.508 |
| walker |  | 9167 | 13 | Code::CodeKey { rung: Body, file: src/nightly.rs, decl: 3, sub: 0, line: 41 } |  |  | 0.508 |
| walker |  | 9191 | 24 | Code::CodeKey { rung: Decl, file: src/nightly.rs, decl: 4, sub: 0, line: 45 } |  |  | 0.508 |
| walker |  | 9205 | 14 | Code::CodeKey { rung: Body, file: src/nightly.rs, decl: 5, sub: 0, line: 52 } |  |  | 0.508 |
| ns | 9217 |  | 131 | Toolchain pin and the no_std check-crate | 7.7 |  | 0.502 |
| walker |  | 9304 | 99 | Code::CodeKey { rung: Doc, file: src/error.rs, decl: 2, sub: 0, line: 27 } |  |  | 0.503 |
| walker |  | 9318 | 14 | Code::CodeKey { rung: Body, file: src/nightly.rs, decl: 4, sub: 0, line: 45 } |  |  | 0.503 |
| ns | 9378 |  | 161 | CI job roster and toolchain matrix | 7.8 |  | 0.498 |
| walker |  | 9528 | 210 | Code::CodeKey { rung: Names, file: src/wrapper.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.510 |
| walker |  | 9535 | 7 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 7, sub: 0, line: 33 } |  |  | 0.511 |
| walker |  | 9544 | 9 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.513 |
| ns | 9570 |  | 192 | CI commands — the canonical build/test/lint invocations | 7.9 | 7.8 | 0.509 |
| walker |  | 9571 | 27 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 13, sub: 0, line: 56 } |  |  | 0.511 |
| walker |  | 9619 | 48 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 8, sub: 0, line: 36 } |  |  | 0.511 |
| ns | 9658 |  | 88 | trybuild UI harness — tests/compiletest.rs in full | 8.1 |  | 0.508 |
| walker |  | 9667 | 48 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 10, sub: 0, line: 45 } |  |  | 0.508 |
| walker |  | 9715 | 48 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 14, sub: 0, line: 60 } |  |  | 0.508 |
| ns | 9740 |  | 82 | tests/ui file listing | 8.2 | 8.1 | 0.516 |
| walker |  | 9763 | 48 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 16, sub: 0, line: 67 } |  |  | 0.516 |
| walker |  | 9813 | 50 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 2, sub: 0, line: 13 } |  |  | 0.516 |
| ns | 9849 |  | 109 | One complete compile-fail pair — wrong-interpolation | 8.3 | 8.2 | 0.512 |
| walker |  | 9863 | 50 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 4, sub: 0, line: 22 } |  |  | 0.512 |
| walker |  | 9933 | 70 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 18, sub: 0, line: 74 } |  |  | 0.512 |
| walker |  | 9945 | 12 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 20, sub: 0, line: 80 } |  |  | 0.512 |
| walker |  | 9955 | 10 | Code::CodeKey { rung: Body, file: src/wrapper.rs, decl: 19, sub: 0, line: 76 } |  |  | 0.512 |
| ns | 9990 |  | 141 | Shared test helpers in tests/common and tests/drop | 8.4 | 4.4 | 0.507 |
