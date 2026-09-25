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
| walker |  | 1149 | 10 | Code::CodeKey { rung: Body, file: src/lib.rs, decl: 9, sub: 0, line: 648 } |  |  | 0.684 |
| walker |  | 1179 | 30 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 8, sub: 0, line: 624 } |  |  | 0.686 |
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
| walker |  | 2020 | 52 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 8, sub: 0, line: 114 } |  |  | 0.717 |
| walker |  | 2026 | 6 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 9, sub: 0, line: 116 } |  |  | 0.717 |
| ns | 2031 |  | 116 | Context trait doc — sealed, and outermost-first cause printing | 2.6 | 2.3 | 0.696 |
| walker |  | 2056 | 30 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 6, sub: 0, line: 90 } |  |  | 0.696 |
| walker |  | 2093 | 37 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 3, sub: 0, line: 68 } |  |  | 0.696 |
| walker |  | 2158 | 65 | Code::CodeKey { rung: Decl, file: src/fmt.rs, decl: 1, sub: 0, line: 6 } |  |  | 0.697 |
| ns | 2212 |  | 181 | Context + downcasting — the guarantee, in both directions | 2.7 | 2.6 | 0.667 |
| walker |  | 2348 | 190 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 4, sub: 0, line: 413 } |  |  | 0.667 |
| ns | 2360 |  | 148 | Result alias and Ok() helper semantics | 2.8 | 2.1 | 0.648 |
| ns | 2536 |  | 176 | no_std support contract | 2.9 |  | 0.631 |
| walker |  | 2600 | 252 | Code::CodeKey { rung: Decl, file: src/macros.rs, decl: 3, sub: 0, line: 202 } |  |  | 0.635 |
| walker |  | 2604 | 4 | Fs::DirListing { dir: tests/common } |  |  | 0.635 |
| walker |  | 2608 | 4 | Fs::DirListing { dir: tests/drop } |  |  | 0.635 |
| walker |  | 2690 | 82 | Fs::DirListing { dir: tests/ui } |  |  | 0.636 |
| ns | 2768 |  | 232 | Trait impls on Error — complete list | 2.10 | 2.2 | 0.613 |
| walker |  | 2975 | 285 | Code::CodeKey { rung: ModuleDoc, file: src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.613 |
| ns | 3002 |  | 234 | What bail! and ensure! mean | 3.1 |  | 0.594 |
| ns | 3183 |  | 181 | What anyhow! constructs | 3.2 | 3.1 | 0.579 |
| walker |  | 3279 | 304 | Code::CodeKey { rung: ModuleDoc, file: src/lib.rs, decl: 0, sub: 1, line: 0 } |  |  | 0.579 |
| ns | 3438 |  | 255 | Every exported macro_rules!, with its export attributes | 3.3 | 3.1 | 0.561 |
| walker |  | 3532 | 253 | Code::CodeKey { rung: ModuleDoc, file: src/lib.rs, decl: 0, sub: 2, line: 0 } |  |  | 0.561 |
| ns | 3660 |  | 222 | anyhow! expansion — the three arms | 3.4 | 3.3 | 0.586 |
| walker |  | 3761 | 229 | Code::CodeKey { rung: ModuleDoc, file: src/lib.rs, decl: 0, sub: 3, line: 0 } |  |  | 0.586 |
| ns | 3796 |  | 136 | bail! body | 3.5 | 3.3 | 0.597 |
| ns | 3957 |  | 161 | ensure! expansion — the fuel-limited parser call | 3.6 | 3.3 | 0.583 |
| walker |  | 4029 | 268 | Code::CodeKey { rung: ModuleDoc, file: src/lib.rs, decl: 0, sub: 4, line: 0 } |  |  | 0.583 |
| ns | 4126 |  | 169 | kind.rs — why autoref tagged dispatch | 3.7 | 3.4 | 0.574 |
| walker |  | 4341 | 312 | Code::CodeKey { rung: ModuleDoc, file: src/lib.rs, decl: 0, sub: 5, line: 0 } |  |  | 0.597 |
| walker |  | 4349 | 8 | Code::CodeKey { rung: Body, file: src/kind.rs, decl: 6, sub: 0, line: 90 } |  |  | 0.597 |
| ns | 4375 |  | 249 | kind.rs — the three kinds and their new() constructors | 3.8 | 3.7 | 0.589 |
| walker |  | 4460 | 111 | Code::CodeKey { rung: Names, file: src/chain.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.589 |
| walker |  | 4480 | 20 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 10, sub: 0, line: 76 } |  |  | 0.589 |
| walker |  | 4510 | 30 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 3, sub: 0, line: 26 } |  |  | 0.589 |
| walker |  | 4516 | 6 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 4, sub: 0, line: 27 } |  |  | 0.589 |
| walker |  | 4550 | 34 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 12, sub: 0, line: 93 } |  |  | 0.589 |
| walker |  | 4585 | 35 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.589 |
| ns | 4627 |  | 252 | ensure.rs runtime side — BothDebug / NotBothDebug dispatch | 3.9 | 3.6 | 0.573 |
| walker |  | 4629 | 44 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 8, sub: 0, line: 56 } |  |  | 0.573 |
| walker |  | 4697 | 68 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 5, sub: 0, line: 35 } |  |  | 0.573 |
| walker |  | 4796 | 99 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 2, sub: 0, line: 15 } |  |  | 0.574 |
| ns | 4798 |  | 171 | __parse_ensure! extent, first rule, and catch-all | 3.10 | 3.6 | 0.563 |
| ns | 4995 |  | 197 | lib.rs __private — the macro-facing surface | 3.11 | 3.3 | 0.552 |
| walker |  | 5040 | 244 | Code::CodeKey { rung: Names, file: src/error.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.555 |
| walker |  | 5057 | 17 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 31, sub: 0, line: 731 } |  |  | 0.555 |
| walker |  | 5085 | 28 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 27, sub: 0, line: 719 } |  |  | 0.555 |
| walker |  | 5113 | 28 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 29, sub: 0, line: 725 } |  |  | 0.555 |
| ns | 5156 |  | 161 | test_fmt.rs — the f/g/h fixture and expected `{:#}` strings | 4.1 | 2.4 | 0.548 |
| walker |  | 5158 | 45 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 25, sub: 0, line: 712 } |  |  | 0.548 |
| walker |  | 5224 | 66 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 23, sub: 0, line: 703 } |  |  | 0.550 |
| walker |  | 5293 | 69 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 21, sub: 0, line: 691 } |  |  | 0.550 |
| ns | 5295 |  | 139 | test_fmt.rs — expected `{:?}` strings, including numbered causes | 4.2 | 4.1 | 0.538 |
| walker |  | 5299 | 6 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 22, sub: 0, line: 696 } |  |  | 0.538 |
| walker |  | 5327 | 28 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 36, sub: 0, line: 775 } |  |  | 0.538 |
| walker |  | 5375 | 48 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 37, sub: 0, line: 787 } |  |  | 0.538 |
| walker |  | 5423 | 48 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 38, sub: 0, line: 798 } |  |  | 0.538 |
| ns | 5454 |  | 159 | test_repr.rs — size, null-pointer-optimization, autotrait assertions | 4.3 | 1.5 | 0.527 |
| ns | 5633 |  | 179 | test_downcast.rs — what a bail!'d error downcasts to | 4.4 | 3.4 | 0.514 |
| walker |  | 5707 | 284 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 9, sub: 0, line: 648 } |  |  | 0.517 |
| ns | 5807 |  | 174 | test_chain.rs — cause iteration order | 4.5 |  | 0.509 |
| walker |  | 5830 | 123 | Code::CodeKey { rung: Names, file: src/backtrace.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.510 |
| walker |  | 5849 | 19 | Code::CodeKey { rung: Decl, file: src/backtrace.rs, decl: 1, sub: 0, line: 7 } |  |  | 0.510 |
| walker |  | 5889 | 40 | Code::CodeKey { rung: Decl, file: src/backtrace.rs, decl: 2, sub: 0, line: 10 } |  |  | 0.510 |
| walker |  | 5934 | 45 | Code::CodeKey { rung: Decl, file: src/backtrace.rs, decl: 5, sub: 0, line: 31 } |  |  | 0.510 |
| walker |  | 5985 | 51 | Code::CodeKey { rung: Decl, file: src/backtrace.rs, decl: 4, sub: 0, line: 24 } |  |  | 0.510 |
| ns | 6002 |  | 195 | test_context.rs — downcast_ref through a three-level context chain | 4.6 | 2.7 | 0.501 |
| walker |  | 6037 | 52 | Code::CodeKey { rung: Decl, file: src/backtrace.rs, decl: 3, sub: 0, line: 17 } |  |  | 0.502 |
| walker |  | 6120 | 83 | Code::CodeKey { rung: Decl, file: src/backtrace.rs, decl: 8, sub: 0, line: 59 } |  |  | 0.502 |
| walker |  | 6207 | 87 | Code::CodeKey { rung: Decl, file: src/backtrace.rs, decl: 6, sub: 0, line: 38 } |  |  | 0.502 |
| ns | 6252 |  | 250 | ErrorVTable — the hand-rolled vtable layout | 5.1 |  | 0.494 |
| walker |  | 6293 | 86 | Code::CodeKey { rung: Decl, file: src/backtrace.rs, decl: 7, sub: 0, line: 48 } |  |  | 0.494 |
| ns | 6483 |  | 231 | ErrorImpl, ContextError, and the vtable reader | 5.2 | 5.1 | 0.484 |
| walker |  | 6497 | 204 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 1, sub: 0, line: 19 } |  |  | 0.486 |
| walker |  | 6535 | 38 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 4, sub: 0, line: 137 } |  |  | 0.486 |
| walker |  | 6583 | 48 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 3, sub: 0, line: 75 } |  |  | 0.486 |
| ns | 6615 |  | 132 | Error::construct — allocation and type erasure | 5.3 | 5.2 | 0.479 |
| walker |  | 6651 | 68 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 2, sub: 0, line: 27 } |  |  | 0.479 |
| walker |  | 6688 | 37 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 7, sub: 0, line: 197 } |  |  | 0.479 |
| ns | 6718 |  | 103 | impl Error — roster of the non-public fns | 5.4 | 5.3 | 0.482 |
| walker |  | 6727 | 39 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 6, sub: 0, line: 169 } |  |  | 0.482 |
| walker |  | 6786 | 59 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 5, sub: 0, line: 145 } |  |  | 0.482 |
| walker |  | 6854 | 68 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 9, sub: 0, line: 258 } |  |  | 0.482 |
| ns | 6889 |  | 171 | Vtable function roster in error.rs | 5.5 | 5.1 | 0.479 |
| ns | 7040 |  | 151 | ErrorImpl methods and trait impls | 5.6 | 5.2 | 0.474 |
| walker |  | 7100 | 246 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 1, sub: 1, line: 19 } |  |  | 0.497 |
| walker |  | 7119 | 19 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 11, sub: 0, line: 431 } |  |  | 0.497 |
| walker |  | 7148 | 29 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 12, sub: 0, line: 457 } |  |  | 0.497 |
| walker |  | 7179 | 31 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 14, sub: 0, line: 482 } |  |  | 0.497 |
| walker |  | 7210 | 31 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 15, sub: 0, line: 490 } |  |  | 0.497 |
| walker |  | 7241 | 31 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 16, sub: 0, line: 554 } |  |  | 0.497 |
| walker |  | 7272 | 31 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 17, sub: 0, line: 568 } |  |  | 0.497 |
| ns | 7275 |  | 235 | ptr.rs — Own / Ref / Mut / CastTo declarations | 5.7 |  | 0.484 |
| walker |  | 7309 | 37 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 13, sub: 0, line: 468 } |  |  | 0.484 |
| walker |  | 7321 | 12 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 20, sub: 0, line: 674 } |  |  | 0.484 |
| walker |  | 7367 | 46 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 10, sub: 0, line: 370 } |  |  | 0.484 |
| ns | 7440 |  | 165 | chain.rs — Chain and ChainState | 6.1 |  | 0.495 |
| walker |  | 7443 | 76 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 18, sub: 0, line: 598 } |  |  | 0.495 |
| walker |  | 7458 | 15 | Code::CodeKey { rung: Doc, file: src/error.rs, decl: 17, sub: 0, line: 568 } |  |  | 0.495 |
| ns | 7557 |  | 117 | chain.rs — every trait impl on Chain | 6.2 | 6.1 | 0.502 |
| walker |  | 7562 | 104 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 19, sub: 0, line: 640 } |  |  | 0.502 |
| walker |  | 7580 | 18 | Code::CodeKey { rung: Doc, file: src/error.rs, decl: 15, sub: 0, line: 490 } |  |  | 0.502 |
| ns | 7782 |  | 225 | wrapper.rs — the three error adapters and their impls | 6.3 |  | 0.494 |
| walker |  | 7932 | 352 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 5, sub: 0, line: 468 } |  |  | 0.507 |
| ns | 7959 |  | 177 | context.rs — where .context() is implemented | 6.4 | 2.3 | 0.501 |
| walker |  | 8104 | 172 | Code::CodeKey { rung: Names, file: src/context.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.501 |
| ns | 8133 |  | 174 | context.rs — ContextError impls, Quoted, and the Sealed trait | 6.5 | 6.4 | 0.501 |
| walker |  | 8152 | 48 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 9, sub: 0, line: 128 } |  |  | 0.501 |
| walker |  | 8204 | 52 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 4, sub: 0, line: 90 } |  |  | 0.501 |
| walker |  | 8233 | 29 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 5, sub: 0, line: 91 } |  |  | 0.501 |
| ns | 8261 |  | 128 | fmt.rs — the rendering entry points | 6.6 | 5.6 | 0.507 |
| walker |  | 8290 | 57 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 7, sub: 0, line: 115 } |  |  | 0.507 |
| walker |  | 8362 | 72 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 14, sub: 0, line: 152 } |  |  | 0.507 |
| walker |  | 8374 | 12 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 16, sub: 0, line: 160 } |  |  | 0.507 |
| ns | 8413 |  | 152 | build.rs — the complete custom-cfg vocabulary | 7.1 |  | 0.503 |
| walker |  | 8416 | 42 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 6, sub: 0, line: 103 } |  |  | 0.503 |
| walker |  | 8500 | 84 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 1, sub: 0, line: 42 } |  |  | 0.504 |
| ns | 8527 |  | 114 | build.rs — which compiler turns each cfg on | 7.2 | 7.1 | 0.502 |
| walker |  | 8529 | 29 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 2, sub: 0, line: 46 } |  |  | 0.502 |
| walker |  | 8571 | 42 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 3, sub: 0, line: 58 } |  |  | 0.502 |
| ns | 8637 |  | 110 | backtrace.rs — the three definitions of Backtrace | 7.3 |  | 0.499 |
| walker |  | 8656 | 85 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 11, sub: 0, line: 137 } |  |  | 0.499 |
| walker |  | 8668 | 12 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 13, sub: 0, line: 146 } |  |  | 0.499 |
| walker |  | 8697 | 29 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 20, sub: 0, line: 180 } |  |  | 0.499 |
| walker |  | 8745 | 48 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 18, sub: 0, line: 168 } |  |  | 0.499 |
| ns | 8813 |  | 176 | backtrace.rs — impl_backtrace! and backtrace!(), both arms each | 7.4 | 7.3 | 0.507 |
| walker |  | 8919 | 174 | Code::CodeKey { rung: Names, file: src/nightly.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.507 |
| walker |  | 8930 | 11 | Code::CodeKey { rung: Decl, file: src/nightly.rs, decl: 2, sub: 0, line: 38 } |  |  | 0.507 |
| walker |  | 8942 | 12 | Code::CodeKey { rung: Body, file: src/nightly.rs, decl: 6, sub: 0, line: 56 } |  |  | 0.507 |
| ns | 8954 |  | 141 | backtrace.rs — backtrace_if_absent! and the vendored capture module | 7.5 | 7.3 | 0.510 |
| walker |  | 8955 | 13 | Code::CodeKey { rung: Body, file: src/nightly.rs, decl: 3, sub: 0, line: 41 } |  |  | 0.510 |
| walker |  | 8979 | 24 | Code::CodeKey { rung: Decl, file: src/nightly.rs, decl: 4, sub: 0, line: 45 } |  |  | 0.510 |
| walker |  | 8993 | 14 | Code::CodeKey { rung: Body, file: src/nightly.rs, decl: 5, sub: 0, line: 52 } |  |  | 0.510 |
| walker |  | 9007 | 14 | Code::CodeKey { rung: Body, file: src/nightly.rs, decl: 4, sub: 0, line: 45 } |  |  | 0.510 |
| ns | 9086 |  | 132 | nightly.rs — the build probe and generic-member-access shims | 7.6 | 7.1 | 0.508 |
| walker |  | 9217 | 210 | Code::CodeKey { rung: Names, file: src/wrapper.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.514 |
| ns | 9217 |  | 131 | Toolchain pin and the no_std check-crate | 7.7 |  | 0.514 |
| walker |  | 9224 | 7 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 7, sub: 0, line: 33 } |  |  | 0.515 |
| walker |  | 9233 | 9 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.517 |
| walker |  | 9260 | 27 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 13, sub: 0, line: 56 } |  |  | 0.519 |
| walker |  | 9308 | 48 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 8, sub: 0, line: 36 } |  |  | 0.519 |
| walker |  | 9356 | 48 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 10, sub: 0, line: 45 } |  |  | 0.519 |
| ns | 9378 |  | 161 | CI job roster and toolchain matrix | 7.8 |  | 0.513 |
| walker |  | 9404 | 48 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 14, sub: 0, line: 60 } |  |  | 0.513 |
| walker |  | 9452 | 48 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 16, sub: 0, line: 67 } |  |  | 0.513 |
| walker |  | 9502 | 50 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 2, sub: 0, line: 13 } |  |  | 0.513 |
| walker |  | 9552 | 50 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 4, sub: 0, line: 22 } |  |  | 0.513 |
| ns | 9570 |  | 192 | CI commands — the canonical build/test/lint invocations | 7.9 | 7.8 | 0.509 |
| walker |  | 9622 | 70 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 18, sub: 0, line: 74 } |  |  | 0.509 |
| walker |  | 9634 | 12 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 20, sub: 0, line: 80 } |  |  | 0.509 |
| ns | 9658 |  | 88 | trybuild UI harness — tests/compiletest.rs in full | 8.1 |  | 0.507 |
| ns | 9740 |  | 82 | tests/ui file listing | 8.2 | 8.1 | 0.515 |
| walker |  | 9833 | 199 | Code::CodeKey { rung: Names, file: src/error.rs, decl: 0, sub: 1, line: 0 } |  |  | 0.525 |
| ns | 9849 |  | 109 | One complete compile-fail pair — wrong-interpolation | 8.3 | 8.2 | 0.521 |
| walker |  | 9854 | 21 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 39, sub: 0, line: 809 } |  |  | 0.521 |
| walker |  | 9875 | 21 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 43, sub: 0, line: 876 } |  |  | 0.521 |
| walker |  | 9896 | 21 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 44, sub: 0, line: 892 } |  |  | 0.521 |
| walker |  | 9936 | 40 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 40, sub: 0, line: 828 } |  |  | 0.521 |
| walker |  | 9987 | 51 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 41, sub: 0, line: 838 } |  |  | 0.521 |
| ns | 9990 |  | 141 | Shared test helpers in tests/common and tests/drop | 8.4 | 4.4 | 0.516 |
