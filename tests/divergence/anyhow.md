Score(3000)=0.614 I=0.846 C=0.446 ns_rows≤3K=20/63 grid(1000/1442/2080/3000/4327/6240/9000)=0.895/0.807/0.694/0.614/0.578/0.478/0.545

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
| walker |  | 672 | 90 | Fs::DirListing { dir: tests } |  |  | 0.894 |
| ns | 793 |  | 148 | README lede, install snippet, and section map | 1.9 |  | 0.884 |
| walker |  | 897 | 225 | Code::CodeKey { rung: Names, file: src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.891 |
| walker |  | 915 | 18 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 9, sub: 0, line: 648 } |  |  | 0.892 |
| walker |  | 941 | 26 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 3, sub: 0, line: 389 } |  |  | 0.895 |
| walker |  | 986 | 45 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 1, sub: 0, line: 278 } |  |  | 0.895 |
| walker |  | 993 | 7 | Code::CodeKey { rung: Body, file: src/lib.rs, decl: 2, sub: 0, line: 280 } |  |  | 0.895 |
| ns | 1009 |  | 216 | lib.rs crate attributes and module declarations | 1.10 |  | 0.801 |
| walker |  | 1042 | 49 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 4, sub: 0, line: 413 } |  |  | 0.806 |
| walker |  | 1098 | 56 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 6, sub: 0, line: 616 } |  |  | 0.806 |
| walker |  | 1121 | 23 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 7, sub: 0, line: 618 } |  |  | 0.807 |
| walker |  | 1155 | 34 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 8, sub: 0, line: 624 } |  |  | 0.808 |
| walker |  | 1167 | 12 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 7, sub: 0, line: 618 } |  |  | 0.809 |
| walker |  | 1197 | 30 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 8, sub: 0, line: 624 } |  |  | 0.812 |
| ns | 1244 |  | 235 | Complete roster of exported items in lib.rs | 2.1 |  | 0.799 |
| walker |  | 1322 | 125 | Toml::Config { file: Cargo.toml } |  |  | 0.799 |
| walker |  | 1332 | 10 | Code::CodeKey { rung: Body, file: src/lib.rs, decl: 9, sub: 0, line: 648 } |  |  | 0.807 |
| walker |  | 1401 | 69 | Code::CodeKey { rung: Names, file: build.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.807 |
| ns | 1452 |  | 208 | impl Error — roster of every public method | 2.2 |  | 0.753 |
| walker |  | 1474 | 73 | Code::CodeKey { rung: Body, file: build.rs, decl: 4, sub: 0, line: 199 } |  |  | 0.753 |
| walker |  | 1478 | 4 | Fs::DirListing { dir: tests/common } |  |  | 0.753 |
| walker |  | 1482 | 4 | Fs::DirListing { dir: tests/drop } |  |  | 0.753 |
| walker |  | 1545 | 63 | Code::CodeKey { rung: Names, file: src/kind.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.753 |
| walker |  | 1565 | 20 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 7, sub: 0, line: 99 } |  |  | 0.753 |
| walker |  | 1591 | 26 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 2, sub: 0, line: 67 } |  |  | 0.753 |
| ns | 1612 |  | 160 | Context trait — both method signatures with bounds | 2.3 | 2.1 | 0.760 |
| walker |  | 1617 | 26 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 5, sub: 0, line: 89 } |  |  | 0.760 |
| walker |  | 1647 | 30 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 6, sub: 0, line: 90 } |  |  | 0.761 |
| walker |  | 1684 | 37 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 3, sub: 0, line: 68 } |  |  | 0.761 |
| walker |  | 1736 | 52 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 8, sub: 0, line: 114 } |  |  | 0.761 |
| walker |  | 1742 | 6 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 9, sub: 0, line: 116 } |  |  | 0.761 |
| walker |  | 1750 | 8 | Code::CodeKey { rung: Body, file: src/kind.rs, decl: 6, sub: 0, line: 90 } |  |  | 0.761 |
| ns | 1778 |  | 166 | Display representations — `{}` and `{:#}`, with sample output | 2.4 | 1.4 | 0.739 |
| walker |  | 1867 | 117 | Code::CodeKey { rung: Body, file: build.rs, decl: 3, sub: 0, line: 188 } |  |  | 0.739 |
| ns | 1915 |  | 137 | Debug representations — `{:?}` and `{:#?}` | 2.5 | 2.4 | 0.714 |
| walker |  | 1949 | 82 | Fs::DirListing { dir: tests/ui } |  |  | 0.715 |
| walker |  | 1985 | 36 | Code::CodeKey { rung: Names, file: src/macros.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.715 |
| ns | 2031 |  | 116 | Context trait doc — sealed, and outermost-first cause printing | 2.6 | 2.3 | 0.694 |
| walker |  | 2151 | 166 | Code::CodeKey { rung: Decl, file: src/macros.rs, decl: 1, sub: 0, line: 56 } |  |  | 0.697 |
| ns | 2212 |  | 181 | Context + downcasting — the guarantee, in both directions | 2.7 | 2.6 | 0.667 |
| ns | 2360 |  | 148 | Result alias and Ok() helper semantics | 2.8 | 2.1 | 0.648 |
| walker |  | 2403 | 252 | Code::CodeKey { rung: Decl, file: src/macros.rs, decl: 3, sub: 0, line: 202 } |  |  | 0.653 |
| walker |  | 2526 | 123 | Code::CodeKey { rung: Names, file: src/backtrace.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.653 |
| ns | 2536 |  | 176 | no_std support contract | 2.9 |  | 0.636 |
| walker |  | 2545 | 19 | Code::CodeKey { rung: Decl, file: src/backtrace.rs, decl: 1, sub: 0, line: 7 } |  |  | 0.636 |
| walker |  | 2585 | 40 | Code::CodeKey { rung: Decl, file: src/backtrace.rs, decl: 2, sub: 0, line: 10 } |  |  | 0.636 |
| walker |  | 2630 | 45 | Code::CodeKey { rung: Decl, file: src/backtrace.rs, decl: 5, sub: 0, line: 31 } |  |  | 0.636 |
| walker |  | 2681 | 51 | Code::CodeKey { rung: Decl, file: src/backtrace.rs, decl: 4, sub: 0, line: 24 } |  |  | 0.636 |
| walker |  | 2733 | 52 | Code::CodeKey { rung: Decl, file: src/backtrace.rs, decl: 3, sub: 0, line: 17 } |  |  | 0.637 |
| ns | 2768 |  | 232 | Trait impls on Error — complete list | 2.10 | 2.2 | 0.614 |
| walker |  | 2816 | 83 | Code::CodeKey { rung: Decl, file: src/backtrace.rs, decl: 8, sub: 0, line: 59 } |  |  | 0.614 |
| walker |  | 2903 | 87 | Code::CodeKey { rung: Decl, file: src/backtrace.rs, decl: 6, sub: 0, line: 38 } |  |  | 0.614 |
| walker |  | 2989 | 86 | Code::CodeKey { rung: Decl, file: src/backtrace.rs, decl: 7, sub: 0, line: 48 } |  |  | 0.614 |
| ns | 3002 |  | 234 | What bail! and ensure! mean | 3.1 |  | 0.595 |
| walker |  | 3033 | 44 | Code::CodeKey { rung: Names, file: src/fmt.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.595 |
| walker |  | 3068 | 35 | Code::CodeKey { rung: Decl, file: src/fmt.rs, decl: 4, sub: 0, line: 69 } |  |  | 0.595 |
| walker |  | 3117 | 49 | Code::CodeKey { rung: Decl, file: src/fmt.rs, decl: 5, sub: 0, line: 75 } |  |  | 0.595 |
| walker |  | 3182 | 65 | Code::CodeKey { rung: Decl, file: src/fmt.rs, decl: 1, sub: 0, line: 6 } |  |  | 0.596 |
| ns | 3183 |  | 181 | What anyhow! constructs | 3.2 | 3.1 | 0.581 |
| walker |  | 3293 | 111 | Code::CodeKey { rung: Names, file: src/chain.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.581 |
| walker |  | 3313 | 20 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 10, sub: 0, line: 76 } |  |  | 0.581 |
| walker |  | 3343 | 30 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 3, sub: 0, line: 26 } |  |  | 0.581 |
| walker |  | 3349 | 6 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 4, sub: 0, line: 27 } |  |  | 0.581 |
| walker |  | 3383 | 34 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 12, sub: 0, line: 93 } |  |  | 0.581 |
| walker |  | 3418 | 35 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.581 |
| ns | 3438 |  | 255 | Every exported macro_rules!, with its export attributes | 3.3 | 3.1 | 0.563 |
| walker |  | 3462 | 44 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 8, sub: 0, line: 56 } |  |  | 0.563 |
| walker |  | 3530 | 68 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 5, sub: 0, line: 35 } |  |  | 0.563 |
| walker |  | 3629 | 99 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 2, sub: 0, line: 15 } |  |  | 0.564 |
| ns | 3660 |  | 222 | anyhow! expansion — the three arms | 3.4 | 3.3 | 0.590 |
| walker |  | 3680 | 51 | Code::CodeKey { rung: Names, file: src/ensure.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.590 |
| walker |  | 3707 | 27 | Code::CodeKey { rung: Decl, file: src/ensure.rs, decl: 2, sub: 0, line: 58 } |  |  | 0.590 |
| walker |  | 3738 | 31 | Code::CodeKey { rung: Decl, file: src/ensure.rs, decl: 1, sub: 0, line: 35 } |  |  | 0.590 |
| ns | 3796 |  | 136 | bail! body | 3.5 | 3.3 | 0.601 |
| walker |  | 3925 | 187 | Code::CodeKey { rung: Body, file: src/ensure.rs, decl: 3, sub: 0, line: 59 } |  |  | 0.601 |
| ns | 3957 |  | 161 | ensure! expansion — the fuel-limited parser call | 3.6 | 3.3 | 0.587 |
| walker |  | 4115 | 190 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 4, sub: 0, line: 413 } |  |  | 0.587 |
| ns | 4126 |  | 169 | kind.rs — why autoref tagged dispatch | 3.7 | 3.4 | 0.577 |
| walker |  | 4325 | 210 | Code::CodeKey { rung: Names, file: src/wrapper.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.578 |
| walker |  | 4332 | 7 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 7, sub: 0, line: 33 } |  |  | 0.578 |
| walker |  | 4341 | 9 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.579 |
| walker |  | 4368 | 27 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 13, sub: 0, line: 56 } |  |  | 0.579 |
| ns | 4375 |  | 249 | kind.rs — the three kinds and their new() constructors | 3.8 | 3.7 | 0.571 |
| walker |  | 4416 | 48 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 8, sub: 0, line: 36 } |  |  | 0.571 |
| walker |  | 4464 | 48 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 10, sub: 0, line: 45 } |  |  | 0.571 |
| walker |  | 4512 | 48 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 14, sub: 0, line: 60 } |  |  | 0.571 |
| walker |  | 4560 | 48 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 16, sub: 0, line: 67 } |  |  | 0.571 |
| walker |  | 4610 | 50 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 2, sub: 0, line: 13 } |  |  | 0.571 |
| ns | 4627 |  | 252 | ensure.rs runtime side — BothDebug / NotBothDebug dispatch | 3.9 | 3.6 | 0.555 |
| walker |  | 4660 | 50 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 4, sub: 0, line: 22 } |  |  | 0.555 |
| walker |  | 4730 | 70 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 18, sub: 0, line: 74 } |  |  | 0.555 |
| walker |  | 4742 | 12 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 20, sub: 0, line: 80 } |  |  | 0.555 |
| ns | 4798 |  | 171 | __parse_ensure! extent, first rule, and catch-all | 3.10 | 3.6 | 0.545 |
| ns | 4995 |  | 197 | lib.rs __private — the macro-facing surface | 3.11 | 3.3 | 0.533 |
| walker |  | 5058 | 316 | Code::CodeKey { rung: Names, file: src/ptr.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.533 |
| walker |  | 5069 | 11 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 35, sub: 0, line: 181 } |  |  | 0.533 |
| walker |  | 5082 | 13 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 36, sub: 0, line: 185 } |  |  | 0.533 |
| walker |  | 5103 | 21 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 33, sub: 0, line: 174 } |  |  | 0.533 |
| walker |  | 5142 | 39 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 5, sub: 0, line: 19 } |  |  | 0.533 |
| ns | 5156 |  | 161 | test_fmt.rs — the f/g/h fixture and expected `{:#}` strings | 4.1 | 2.4 | 0.526 |
| walker |  | 5181 | 39 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 15, sub: 0, line: 74 } |  |  | 0.526 |
| walker |  | 5220 | 39 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 26, sub: 0, line: 135 } |  |  | 0.526 |
| walker |  | 5265 | 45 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 1, sub: 0, line: 5 } |  |  | 0.527 |
| ns | 5295 |  | 139 | test_fmt.rs — expected `{:?}` strings, including numbered causes | 4.2 | 4.1 | 0.515 |
| walker |  | 5324 | 59 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 13, sub: 0, line: 63 } |  |  | 0.515 |
| walker |  | 5384 | 60 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 24, sub: 0, line: 124 } |  |  | 0.516 |
| ns | 5454 |  | 159 | test_repr.rs — size, null-pointer-optimization, autotrait assertions | 4.3 | 1.5 | 0.506 |
| walker |  | 5500 | 116 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 28, sub: 0, line: 144 } |  |  | 0.506 |
| walker |  | 5623 | 123 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 7, sub: 0, line: 28 } |  |  | 0.506 |
| ns | 5633 |  | 179 | test_downcast.rs — what a bail!'d error downcasts to | 4.4 | 3.4 | 0.493 |
| walker |  | 5771 | 148 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 17, sub: 0, line: 83 } |  |  | 0.493 |
| walker |  | 5789 | 18 | Code::CodeKey { rung: Body, file: src/kind.rs, decl: 3, sub: 0, line: 68 } |  |  | 0.493 |
| ns | 5807 |  | 174 | test_chain.rs — cause iteration order | 4.5 |  | 0.486 |
| walker |  | 5961 | 172 | Code::CodeKey { rung: Names, file: src/context.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.486 |
| walker |  | 5990 | 29 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 20, sub: 0, line: 180 } |  |  | 0.486 |
| ns | 6002 |  | 195 | test_context.rs — downcast_ref through a three-level context chain | 4.6 | 2.7 | 0.478 |
| walker |  | 6038 | 48 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 9, sub: 0, line: 128 } |  |  | 0.478 |
| walker |  | 6086 | 48 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 18, sub: 0, line: 168 } |  |  | 0.478 |
| walker |  | 6138 | 52 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 4, sub: 0, line: 90 } |  |  | 0.478 |
| walker |  | 6167 | 29 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 5, sub: 0, line: 91 } |  |  | 0.478 |
| walker |  | 6209 | 42 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 6, sub: 0, line: 103 } |  |  | 0.478 |
| ns | 6252 |  | 250 | ErrorVTable — the hand-rolled vtable layout | 5.1 |  | 0.471 |
| walker |  | 6266 | 57 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 7, sub: 0, line: 115 } |  |  | 0.471 |
| walker |  | 6338 | 72 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 14, sub: 0, line: 152 } |  |  | 0.471 |
| walker |  | 6350 | 12 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 16, sub: 0, line: 160 } |  |  | 0.471 |
| walker |  | 6434 | 84 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 1, sub: 0, line: 42 } |  |  | 0.471 |
| walker |  | 6463 | 29 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 2, sub: 0, line: 46 } |  |  | 0.471 |
| ns | 6483 |  | 231 | ErrorImpl, ContextError, and the vtable reader | 5.2 | 5.1 | 0.461 |
| walker |  | 6505 | 42 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 3, sub: 0, line: 58 } |  |  | 0.461 |
| walker |  | 6590 | 85 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 11, sub: 0, line: 137 } |  |  | 0.461 |
| walker |  | 6602 | 12 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 13, sub: 0, line: 146 } |  |  | 0.461 |
| walker |  | 6610 | 8 | Code::CodeKey { rung: Body, file: src/ptr.rs, decl: 6, sub: 0, line: 23 } |  |  | 0.461 |
| ns | 6615 |  | 132 | Error::construct — allocation and type erasure | 5.3 | 5.2 | 0.454 |
| walker |  | 6620 | 10 | Code::CodeKey { rung: Body, file: src/wrapper.rs, decl: 19, sub: 0, line: 76 } |  |  | 0.454 |
| ns | 6718 |  | 103 | impl Error — roster of the non-public fns | 5.4 | 5.3 | 0.451 |
| walker |  | 6864 | 244 | Code::CodeKey { rung: Names, file: src/error.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.454 |
| walker |  | 6881 | 17 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 31, sub: 0, line: 731 } |  |  | 0.454 |
| ns | 6889 |  | 171 | Vtable function roster in error.rs | 5.5 | 5.1 | 0.452 |
| walker |  | 6909 | 28 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 27, sub: 0, line: 719 } |  |  | 0.452 |
| walker |  | 6937 | 28 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 29, sub: 0, line: 725 } |  |  | 0.452 |
| walker |  | 6965 | 28 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 36, sub: 0, line: 775 } |  |  | 0.452 |
| walker |  | 7010 | 45 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 25, sub: 0, line: 712 } |  |  | 0.452 |
| ns | 7040 |  | 151 | ErrorImpl methods and trait impls | 5.6 | 5.2 | 0.446 |
| walker |  | 7058 | 48 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 37, sub: 0, line: 787 } |  |  | 0.446 |
| walker |  | 7106 | 48 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 38, sub: 0, line: 798 } |  |  | 0.446 |
| walker |  | 7172 | 66 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 23, sub: 0, line: 703 } |  |  | 0.448 |
| walker |  | 7241 | 69 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 21, sub: 0, line: 691 } |  |  | 0.448 |
| walker |  | 7247 | 6 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 22, sub: 0, line: 696 } |  |  | 0.448 |
| ns | 7275 |  | 235 | ptr.rs — Own / Ref / Mut / CastTo declarations | 5.7 |  | 0.474 |
| ns | 7440 |  | 165 | chain.rs — Chain and ChainState | 6.1 |  | 0.485 |
| walker |  | 7518 | 271 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 33, sub: 0, line: 740 } |  |  | 0.504 |
| ns | 7557 |  | 117 | chain.rs — every trait impl on Chain | 6.2 | 6.1 | 0.510 |
| walker |  | 7722 | 204 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 1, sub: 0, line: 19 } |  |  | 0.517 |
| walker |  | 7759 | 37 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 7, sub: 0, line: 197 } |  |  | 0.517 |
| ns | 7782 |  | 225 | wrapper.rs — the three error adapters and their impls | 6.3 |  | 0.527 |
| walker |  | 7797 | 38 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 4, sub: 0, line: 137 } |  |  | 0.527 |
| walker |  | 7836 | 39 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 6, sub: 0, line: 169 } |  |  | 0.527 |
| walker |  | 7884 | 48 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 3, sub: 0, line: 75 } |  |  | 0.527 |
| walker |  | 7943 | 59 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 5, sub: 0, line: 145 } |  |  | 0.527 |
| ns | 7959 |  | 177 | context.rs — where .context() is implemented | 6.4 | 2.3 | 0.523 |
| walker |  | 8011 | 68 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 2, sub: 0, line: 27 } |  |  | 0.523 |
| walker |  | 8079 | 68 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 9, sub: 0, line: 258 } |  |  | 0.523 |
| ns | 8133 |  | 174 | context.rs — ContextError impls, Quoted, and the Sealed trait | 6.5 | 6.4 | 0.522 |
| walker |  | 8194 | 115 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 8, sub: 0, line: 225 } |  |  | 0.522 |
| ns | 8261 |  | 128 | fmt.rs — the rendering entry points | 6.6 | 5.6 | 0.527 |
| ns | 8413 |  | 152 | build.rs — the complete custom-cfg vocabulary | 7.1 |  | 0.523 |
| walker |  | 8440 | 246 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 1, sub: 1, line: 19 } |  |  | 0.542 |
| walker |  | 8452 | 12 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 20, sub: 0, line: 674 } |  |  | 0.542 |
| walker |  | 8471 | 19 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 11, sub: 0, line: 431 } |  |  | 0.542 |
| walker |  | 8500 | 29 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 12, sub: 0, line: 457 } |  |  | 0.542 |
| ns | 8527 |  | 114 | build.rs — which compiler turns each cfg on | 7.2 | 7.1 | 0.540 |
| walker |  | 8531 | 31 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 14, sub: 0, line: 482 } |  |  | 0.540 |
| walker |  | 8562 | 31 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 15, sub: 0, line: 490 } |  |  | 0.540 |
| walker |  | 8593 | 31 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 16, sub: 0, line: 554 } |  |  | 0.540 |
| walker |  | 8624 | 31 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 17, sub: 0, line: 568 } |  |  | 0.540 |
| ns | 8637 |  | 110 | backtrace.rs — the three definitions of Backtrace | 7.3 |  | 0.536 |
| walker |  | 8661 | 37 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 13, sub: 0, line: 468 } |  |  | 0.536 |
| walker |  | 8707 | 46 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 10, sub: 0, line: 370 } |  |  | 0.536 |
| walker |  | 8783 | 76 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 18, sub: 0, line: 598 } |  |  | 0.536 |
| walker |  | 8798 | 15 | Code::CodeKey { rung: Doc, file: src/error.rs, decl: 17, sub: 0, line: 568 } |  |  | 0.536 |
| ns | 8813 |  | 176 | backtrace.rs — impl_backtrace! and backtrace!(), both arms each | 7.4 | 7.3 | 0.543 |
| walker |  | 8902 | 104 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 19, sub: 0, line: 640 } |  |  | 0.543 |
| walker |  | 8920 | 18 | Code::CodeKey { rung: Doc, file: src/error.rs, decl: 15, sub: 0, line: 490 } |  |  | 0.543 |
| ns | 8954 |  | 141 | backtrace.rs — backtrace_if_absent! and the vendored capture module | 7.5 | 7.3 | 0.545 |
| walker |  | 8992 | 72 | Code::CodeKey { rung: Doc, file: src/error.rs, decl: 13, sub: 0, line: 468 } |  |  | 0.545 |
| ns | 9086 |  | 132 | nightly.rs — the build probe and generic-member-access shims | 7.6 | 7.1 | 0.541 |
| walker |  | 9191 | 199 | Code::CodeKey { rung: Names, file: src/error.rs, decl: 0, sub: 1, line: 0 } |  |  | 0.552 |
| walker |  | 9212 | 21 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 39, sub: 0, line: 809 } |  |  | 0.552 |
| ns | 9217 |  | 131 | Toolchain pin and the no_std check-crate | 7.7 |  | 0.546 |
| walker |  | 9233 | 21 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 43, sub: 0, line: 876 } |  |  | 0.546 |
| walker |  | 9254 | 21 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 44, sub: 0, line: 892 } |  |  | 0.546 |
| walker |  | 9294 | 40 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 40, sub: 0, line: 828 } |  |  | 0.546 |
| walker |  | 9345 | 51 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 41, sub: 0, line: 838 } |  |  | 0.546 |
| ns | 9378 |  | 161 | CI job roster and toolchain matrix | 7.8 |  | 0.541 |
| walker |  | 9396 | 51 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 42, sub: 0, line: 858 } |  |  | 0.541 |
| walker |  | 9471 | 75 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 45, sub: 0, line: 915 } |  |  | 0.541 |
| walker |  | 9570 | 99 | Code::CodeKey { rung: Doc, file: src/error.rs, decl: 2, sub: 0, line: 27 } |  |  | 0.538 |
| ns | 9570 |  | 192 | CI commands — the canonical build/test/lint invocations | 7.9 | 7.8 | 0.538 |
| ns | 9658 |  | 88 | trybuild UI harness — tests/compiletest.rs in full | 8.1 |  | 0.535 |
| walker |  | 9673 | 103 | Code::CodeKey { rung: Body, file: src/fmt.rs, decl: 2, sub: 0, line: 7 } |  |  | 0.535 |
| ns | 9740 |  | 82 | tests/ui file listing | 8.2 | 8.1 | 0.542 |
| ns | 9849 |  | 109 | One complete compile-fail pair — wrong-interpolation | 8.3 | 8.2 | 0.538 |
| walker |  | 9931 | 258 | Code::CodeKey { rung: Names, file: src/error.rs, decl: 0, sub: 2, line: 0 } |  |  | 0.553 |
| walker |  | 9964 | 33 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 48, sub: 0, line: 951 } |  |  | 0.555 |
| ns | 9990 |  | 141 | Shared test helpers in tests/common and tests/drop | 8.4 | 4.4 | 0.549 |
