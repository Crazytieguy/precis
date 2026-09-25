Score(3000)=0.615 I=0.846 C=0.446 ns_rows≤3K=20/63 grid(1000/1442/2080/3000/4327/6240/9000)=0.895/0.807/0.699/0.615/0.580/0.501/0.559

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
| walker |  | 1599 | 117 | Code::CodeKey { rung: Body, file: build.rs, decl: 3, sub: 0, line: 188 } |  |  | 0.753 |
| ns | 1612 |  | 160 | Context trait — both method signatures with bounds | 2.3 | 2.1 | 0.760 |
| walker |  | 1681 | 82 | Fs::DirListing { dir: tests/ui } |  |  | 0.762 |
| walker |  | 1717 | 36 | Code::CodeKey { rung: Names, file: src/macros.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.762 |
| ns | 1778 |  | 166 | Display representations — `{}` and `{:#}`, with sample output | 2.4 | 1.4 | 0.740 |
| walker |  | 1883 | 166 | Code::CodeKey { rung: Decl, file: src/macros.rs, decl: 1, sub: 0, line: 56 } |  |  | 0.742 |
| ns | 1915 |  | 137 | Debug representations — `{:?}` and `{:#?}` | 2.5 | 2.4 | 0.717 |
| ns | 2031 |  | 116 | Context trait doc — sealed, and outermost-first cause printing | 2.6 | 2.3 | 0.696 |
| walker |  | 2135 | 252 | Code::CodeKey { rung: Decl, file: src/macros.rs, decl: 3, sub: 0, line: 202 } |  |  | 0.701 |
| ns | 2212 |  | 181 | Context + downcasting — the guarantee, in both directions | 2.7 | 2.6 | 0.671 |
| walker |  | 2258 | 123 | Code::CodeKey { rung: Names, file: src/backtrace.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.672 |
| walker |  | 2277 | 19 | Code::CodeKey { rung: Decl, file: src/backtrace.rs, decl: 1, sub: 0, line: 7 } |  |  | 0.672 |
| walker |  | 2317 | 40 | Code::CodeKey { rung: Decl, file: src/backtrace.rs, decl: 2, sub: 0, line: 10 } |  |  | 0.672 |
| ns | 2360 |  | 148 | Result alias and Ok() helper semantics | 2.8 | 2.1 | 0.653 |
| walker |  | 2362 | 45 | Code::CodeKey { rung: Decl, file: src/backtrace.rs, decl: 5, sub: 0, line: 31 } |  |  | 0.653 |
| walker |  | 2413 | 51 | Code::CodeKey { rung: Decl, file: src/backtrace.rs, decl: 4, sub: 0, line: 24 } |  |  | 0.653 |
| walker |  | 2465 | 52 | Code::CodeKey { rung: Decl, file: src/backtrace.rs, decl: 3, sub: 0, line: 17 } |  |  | 0.654 |
| ns | 2536 |  | 176 | no_std support contract | 2.9 |  | 0.636 |
| walker |  | 2548 | 83 | Code::CodeKey { rung: Decl, file: src/backtrace.rs, decl: 8, sub: 0, line: 59 } |  |  | 0.636 |
| walker |  | 2635 | 87 | Code::CodeKey { rung: Decl, file: src/backtrace.rs, decl: 6, sub: 0, line: 38 } |  |  | 0.636 |
| walker |  | 2721 | 86 | Code::CodeKey { rung: Decl, file: src/backtrace.rs, decl: 7, sub: 0, line: 48 } |  |  | 0.636 |
| walker |  | 2765 | 44 | Code::CodeKey { rung: Names, file: src/fmt.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.636 |
| ns | 2768 |  | 232 | Trait impls on Error — complete list | 2.10 | 2.2 | 0.614 |
| walker |  | 2800 | 35 | Code::CodeKey { rung: Decl, file: src/fmt.rs, decl: 4, sub: 0, line: 69 } |  |  | 0.614 |
| walker |  | 2849 | 49 | Code::CodeKey { rung: Decl, file: src/fmt.rs, decl: 5, sub: 0, line: 75 } |  |  | 0.614 |
| walker |  | 2914 | 65 | Code::CodeKey { rung: Decl, file: src/fmt.rs, decl: 1, sub: 0, line: 6 } |  |  | 0.614 |
| ns | 3002 |  | 234 | What bail! and ensure! mean | 3.1 |  | 0.595 |
| walker |  | 3025 | 111 | Code::CodeKey { rung: Names, file: src/chain.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.595 |
| walker |  | 3045 | 20 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 10, sub: 0, line: 76 } |  |  | 0.596 |
| walker |  | 3075 | 30 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 3, sub: 0, line: 26 } |  |  | 0.596 |
| walker |  | 3081 | 6 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 4, sub: 0, line: 27 } |  |  | 0.596 |
| walker |  | 3115 | 34 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 12, sub: 0, line: 93 } |  |  | 0.596 |
| walker |  | 3150 | 35 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.596 |
| ns | 3183 |  | 181 | What anyhow! constructs | 3.2 | 3.1 | 0.581 |
| walker |  | 3194 | 44 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 8, sub: 0, line: 56 } |  |  | 0.581 |
| walker |  | 3262 | 68 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 5, sub: 0, line: 35 } |  |  | 0.581 |
| walker |  | 3361 | 99 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 2, sub: 0, line: 15 } |  |  | 0.582 |
| ns | 3438 |  | 255 | Every exported macro_rules!, with its export attributes | 3.3 | 3.1 | 0.564 |
| walker |  | 3505 | 144 | Code::CodeKey { rung: Names, file: src/kind.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.564 |
| walker |  | 3525 | 20 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 9, sub: 0, line: 99 } |  |  | 0.564 |
| walker |  | 3545 | 20 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 10, sub: 0, line: 111 } |  |  | 0.564 |
| walker |  | 3571 | 26 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 3, sub: 0, line: 67 } |  |  | 0.565 |
| walker |  | 3597 | 26 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 7, sub: 0, line: 89 } |  |  | 0.565 |
| walker |  | 3627 | 30 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 8, sub: 0, line: 90 } |  |  | 0.565 |
| ns | 3660 |  | 222 | anyhow! expansion — the three arms | 3.4 | 3.3 | 0.590 |
| walker |  | 3664 | 37 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 4, sub: 0, line: 68 } |  |  | 0.590 |
| walker |  | 3714 | 50 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 11, sub: 0, line: 114 } |  |  | 0.591 |
| walker |  | 3720 | 6 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 12, sub: 0, line: 116 } |  |  | 0.591 |
| walker |  | 3728 | 8 | Code::CodeKey { rung: Body, file: src/kind.rs, decl: 8, sub: 0, line: 90 } |  |  | 0.591 |
| ns | 3796 |  | 136 | bail! body | 3.5 | 3.3 | 0.602 |
| walker |  | 3918 | 190 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 4, sub: 0, line: 413 } |  |  | 0.602 |
| ns | 3957 |  | 161 | ensure! expansion — the fuel-limited parser call | 3.6 | 3.3 | 0.588 |
| ns | 4126 |  | 169 | kind.rs — why autoref tagged dispatch | 3.7 | 3.4 | 0.578 |
| walker |  | 4128 | 210 | Code::CodeKey { rung: Names, file: src/wrapper.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.579 |
| walker |  | 4135 | 7 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 7, sub: 0, line: 33 } |  |  | 0.579 |
| walker |  | 4144 | 9 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.580 |
| walker |  | 4171 | 27 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 13, sub: 0, line: 56 } |  |  | 0.580 |
| walker |  | 4219 | 48 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 8, sub: 0, line: 36 } |  |  | 0.580 |
| walker |  | 4267 | 48 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 10, sub: 0, line: 45 } |  |  | 0.580 |
| walker |  | 4315 | 48 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 14, sub: 0, line: 60 } |  |  | 0.580 |
| walker |  | 4363 | 48 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 16, sub: 0, line: 67 } |  |  | 0.580 |
| ns | 4375 |  | 249 | kind.rs — the three kinds and their new() constructors | 3.8 | 3.7 | 0.584 |
| walker |  | 4413 | 50 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 2, sub: 0, line: 13 } |  |  | 0.584 |
| walker |  | 4463 | 50 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 4, sub: 0, line: 22 } |  |  | 0.584 |
| walker |  | 4533 | 70 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 18, sub: 0, line: 74 } |  |  | 0.584 |
| walker |  | 4545 | 12 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 20, sub: 0, line: 80 } |  |  | 0.584 |
| ns | 4627 |  | 252 | ensure.rs runtime side — BothDebug / NotBothDebug dispatch | 3.9 | 3.6 | 0.567 |
| walker |  | 4636 | 91 | Code::CodeKey { rung: Names, file: src/ensure.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.568 |
| walker |  | 4662 | 26 | Code::CodeKey { rung: Decl, file: src/ensure.rs, decl: 3, sub: 0, line: 29 } |  |  | 0.570 |
| walker |  | 4689 | 27 | Code::CodeKey { rung: Decl, file: src/ensure.rs, decl: 6, sub: 0, line: 58 } |  |  | 0.570 |
| walker |  | 4720 | 31 | Code::CodeKey { rung: Decl, file: src/ensure.rs, decl: 5, sub: 0, line: 35 } |  |  | 0.570 |
| walker |  | 4777 | 57 | Code::CodeKey { rung: Decl, file: src/ensure.rs, decl: 1, sub: 0, line: 14 } |  |  | 0.577 |
| walker |  | 4787 | 10 | Code::CodeKey { rung: Body, file: src/ensure.rs, decl: 4, sub: 0, line: 30 } |  |  | 0.579 |
| ns | 4798 |  | 171 | __parse_ensure! extent, first rule, and catch-all | 3.10 | 3.6 | 0.568 |
| walker |  | 4805 | 18 | Code::CodeKey { rung: Body, file: src/ensure.rs, decl: 2, sub: 0, line: 19 } |  |  | 0.570 |
| ns | 4995 |  | 197 | lib.rs __private — the macro-facing surface | 3.11 | 3.3 | 0.558 |
| walker |  | 5121 | 316 | Code::CodeKey { rung: Names, file: src/ptr.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.558 |
| walker |  | 5132 | 11 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 35, sub: 0, line: 181 } |  |  | 0.559 |
| walker |  | 5145 | 13 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 36, sub: 0, line: 185 } |  |  | 0.559 |
| ns | 5156 |  | 161 | test_fmt.rs — the f/g/h fixture and expected `{:#}` strings | 4.1 | 2.4 | 0.551 |
| walker |  | 5166 | 21 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 33, sub: 0, line: 174 } |  |  | 0.551 |
| walker |  | 5205 | 39 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 5, sub: 0, line: 19 } |  |  | 0.551 |
| walker |  | 5244 | 39 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 15, sub: 0, line: 74 } |  |  | 0.551 |
| walker |  | 5283 | 39 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 26, sub: 0, line: 135 } |  |  | 0.551 |
| ns | 5295 |  | 139 | test_fmt.rs — expected `{:?}` strings, including numbered causes | 4.2 | 4.1 | 0.539 |
| walker |  | 5328 | 45 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 1, sub: 0, line: 5 } |  |  | 0.539 |
| walker |  | 5387 | 59 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 13, sub: 0, line: 63 } |  |  | 0.540 |
| walker |  | 5447 | 60 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 24, sub: 0, line: 124 } |  |  | 0.541 |
| ns | 5454 |  | 159 | test_repr.rs — size, null-pointer-optimization, autotrait assertions | 4.3 | 1.5 | 0.530 |
| walker |  | 5563 | 116 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 28, sub: 0, line: 144 } |  |  | 0.530 |
| ns | 5633 |  | 179 | test_downcast.rs — what a bail!'d error downcasts to | 4.4 | 3.4 | 0.517 |
| walker |  | 5686 | 123 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 7, sub: 0, line: 28 } |  |  | 0.517 |
| ns | 5807 |  | 174 | test_chain.rs — cause iteration order | 4.5 |  | 0.509 |
| walker |  | 5834 | 148 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 17, sub: 0, line: 83 } |  |  | 0.509 |
| ns | 6002 |  | 195 | test_context.rs — downcast_ref through a three-level context chain | 4.6 | 2.7 | 0.500 |
| walker |  | 6006 | 172 | Code::CodeKey { rung: Names, file: src/context.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.501 |
| walker |  | 6035 | 29 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 20, sub: 0, line: 180 } |  |  | 0.501 |
| walker |  | 6083 | 48 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 9, sub: 0, line: 128 } |  |  | 0.501 |
| walker |  | 6131 | 48 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 18, sub: 0, line: 168 } |  |  | 0.501 |
| walker |  | 6183 | 52 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 4, sub: 0, line: 90 } |  |  | 0.501 |
| walker |  | 6212 | 29 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 5, sub: 0, line: 91 } |  |  | 0.501 |
| ns | 6252 |  | 250 | ErrorVTable — the hand-rolled vtable layout | 5.1 |  | 0.493 |
| walker |  | 6254 | 42 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 6, sub: 0, line: 103 } |  |  | 0.493 |
| walker |  | 6311 | 57 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 7, sub: 0, line: 115 } |  |  | 0.493 |
| walker |  | 6383 | 72 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 14, sub: 0, line: 152 } |  |  | 0.493 |
| walker |  | 6395 | 12 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 16, sub: 0, line: 160 } |  |  | 0.493 |
| walker |  | 6479 | 84 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 1, sub: 0, line: 42 } |  |  | 0.493 |
| ns | 6483 |  | 231 | ErrorImpl, ContextError, and the vtable reader | 5.2 | 5.1 | 0.483 |
| walker |  | 6508 | 29 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 2, sub: 0, line: 46 } |  |  | 0.483 |
| walker |  | 6550 | 42 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 3, sub: 0, line: 58 } |  |  | 0.483 |
| ns | 6615 |  | 132 | Error::construct — allocation and type erasure | 5.3 | 5.2 | 0.476 |
| walker |  | 6635 | 85 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 11, sub: 0, line: 137 } |  |  | 0.476 |
| walker |  | 6647 | 12 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 13, sub: 0, line: 146 } |  |  | 0.476 |
| walker |  | 6655 | 8 | Code::CodeKey { rung: Body, file: src/ptr.rs, decl: 6, sub: 0, line: 23 } |  |  | 0.476 |
| walker |  | 6665 | 10 | Code::CodeKey { rung: Body, file: src/wrapper.rs, decl: 19, sub: 0, line: 76 } |  |  | 0.476 |
| ns | 6718 |  | 103 | impl Error — roster of the non-public fns | 5.4 | 5.3 | 0.473 |
| ns | 6889 |  | 171 | Vtable function roster in error.rs | 5.5 | 5.1 | 0.467 |
| walker |  | 6909 | 244 | Code::CodeKey { rung: Names, file: src/error.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.473 |
| walker |  | 6926 | 17 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 31, sub: 0, line: 731 } |  |  | 0.473 |
| walker |  | 6954 | 28 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 27, sub: 0, line: 719 } |  |  | 0.473 |
| walker |  | 6982 | 28 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 29, sub: 0, line: 725 } |  |  | 0.473 |
| walker |  | 7010 | 28 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 36, sub: 0, line: 775 } |  |  | 0.473 |
| ns | 7040 |  | 151 | ErrorImpl methods and trait impls | 5.6 | 5.2 | 0.467 |
| walker |  | 7055 | 45 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 25, sub: 0, line: 712 } |  |  | 0.467 |
| walker |  | 7103 | 48 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 37, sub: 0, line: 787 } |  |  | 0.467 |
| walker |  | 7151 | 48 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 38, sub: 0, line: 798 } |  |  | 0.467 |
| walker |  | 7217 | 66 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 23, sub: 0, line: 703 } |  |  | 0.469 |
| ns | 7275 |  | 235 | ptr.rs — Own / Ref / Mut / CastTo declarations | 5.7 |  | 0.493 |
| walker |  | 7286 | 69 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 21, sub: 0, line: 691 } |  |  | 0.493 |
| walker |  | 7292 | 6 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 22, sub: 0, line: 696 } |  |  | 0.493 |
| ns | 7440 |  | 165 | chain.rs — Chain and ChainState | 6.1 |  | 0.503 |
| ns | 7557 |  | 117 | chain.rs — every trait impl on Chain | 6.2 | 6.1 | 0.510 |
| walker |  | 7563 | 271 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 33, sub: 0, line: 740 } |  |  | 0.527 |
| walker |  | 7767 | 204 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 1, sub: 0, line: 19 } |  |  | 0.534 |
| ns | 7782 |  | 225 | wrapper.rs — the three error adapters and their impls | 6.3 |  | 0.544 |
| walker |  | 7804 | 37 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 7, sub: 0, line: 197 } |  |  | 0.544 |
| walker |  | 7842 | 38 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 4, sub: 0, line: 137 } |  |  | 0.544 |
| walker |  | 7881 | 39 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 6, sub: 0, line: 169 } |  |  | 0.544 |
| walker |  | 7929 | 48 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 3, sub: 0, line: 75 } |  |  | 0.544 |
| ns | 7959 |  | 177 | context.rs — where .context() is implemented | 6.4 | 2.3 | 0.539 |
| walker |  | 7988 | 59 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 5, sub: 0, line: 145 } |  |  | 0.539 |
| walker |  | 8056 | 68 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 2, sub: 0, line: 27 } |  |  | 0.539 |
| walker |  | 8124 | 68 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 9, sub: 0, line: 258 } |  |  | 0.539 |
| ns | 8133 |  | 174 | context.rs — ContextError impls, Quoted, and the Sealed trait | 6.5 | 6.4 | 0.538 |
| walker |  | 8239 | 115 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 8, sub: 0, line: 225 } |  |  | 0.538 |
| ns | 8261 |  | 128 | fmt.rs — the rendering entry points | 6.6 | 5.6 | 0.542 |
| ns | 8413 |  | 152 | build.rs — the complete custom-cfg vocabulary | 7.1 |  | 0.538 |
| walker |  | 8485 | 246 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 1, sub: 1, line: 19 } |  |  | 0.557 |
| walker |  | 8497 | 12 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 20, sub: 0, line: 674 } |  |  | 0.557 |
| walker |  | 8516 | 19 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 11, sub: 0, line: 431 } |  |  | 0.557 |
| ns | 8527 |  | 114 | build.rs — which compiler turns each cfg on | 7.2 | 7.1 | 0.555 |
| walker |  | 8545 | 29 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 12, sub: 0, line: 457 } |  |  | 0.555 |
| walker |  | 8576 | 31 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 14, sub: 0, line: 482 } |  |  | 0.555 |
| walker |  | 8607 | 31 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 15, sub: 0, line: 490 } |  |  | 0.555 |
| ns | 8637 |  | 110 | backtrace.rs — the three definitions of Backtrace | 7.3 |  | 0.551 |
| walker |  | 8638 | 31 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 16, sub: 0, line: 554 } |  |  | 0.551 |
| walker |  | 8669 | 31 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 17, sub: 0, line: 568 } |  |  | 0.551 |
| walker |  | 8706 | 37 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 13, sub: 0, line: 468 } |  |  | 0.551 |
| walker |  | 8752 | 46 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 10, sub: 0, line: 370 } |  |  | 0.551 |
| ns | 8813 |  | 176 | backtrace.rs — impl_backtrace! and backtrace!(), both arms each | 7.4 | 7.3 | 0.558 |
| walker |  | 8828 | 76 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 18, sub: 0, line: 598 } |  |  | 0.558 |
| walker |  | 8843 | 15 | Code::CodeKey { rung: Doc, file: src/error.rs, decl: 17, sub: 0, line: 568 } |  |  | 0.558 |
| walker |  | 8947 | 104 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 19, sub: 0, line: 640 } |  |  | 0.558 |
| ns | 8954 |  | 141 | backtrace.rs — backtrace_if_absent! and the vendored capture module | 7.5 | 7.3 | 0.559 |
| walker |  | 8965 | 18 | Code::CodeKey { rung: Doc, file: src/error.rs, decl: 15, sub: 0, line: 490 } |  |  | 0.559 |
| walker |  | 9037 | 72 | Code::CodeKey { rung: Doc, file: src/error.rs, decl: 13, sub: 0, line: 468 } |  |  | 0.559 |
| ns | 9086 |  | 132 | nightly.rs — the build probe and generic-member-access shims | 7.6 | 7.1 | 0.556 |
| ns | 9217 |  | 131 | Toolchain pin and the no_std check-crate | 7.7 |  | 0.549 |
| walker |  | 9236 | 199 | Code::CodeKey { rung: Names, file: src/error.rs, decl: 0, sub: 1, line: 0 } |  |  | 0.560 |
| walker |  | 9257 | 21 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 39, sub: 0, line: 809 } |  |  | 0.560 |
| walker |  | 9278 | 21 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 43, sub: 0, line: 876 } |  |  | 0.560 |
| walker |  | 9299 | 21 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 44, sub: 0, line: 892 } |  |  | 0.560 |
| walker |  | 9339 | 40 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 40, sub: 0, line: 828 } |  |  | 0.560 |
| ns | 9378 |  | 161 | CI job roster and toolchain matrix | 7.8 |  | 0.554 |
| walker |  | 9390 | 51 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 41, sub: 0, line: 838 } |  |  | 0.554 |
| walker |  | 9441 | 51 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 42, sub: 0, line: 858 } |  |  | 0.554 |
| walker |  | 9516 | 75 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 45, sub: 0, line: 915 } |  |  | 0.554 |
| ns | 9570 |  | 192 | CI commands — the canonical build/test/lint invocations | 7.9 | 7.8 | 0.550 |
| walker |  | 9615 | 99 | Code::CodeKey { rung: Doc, file: src/error.rs, decl: 2, sub: 0, line: 27 } |  |  | 0.551 |
| ns | 9658 |  | 88 | trybuild UI harness — tests/compiletest.rs in full | 8.1 |  | 0.548 |
| walker |  | 9718 | 103 | Code::CodeKey { rung: Body, file: src/fmt.rs, decl: 2, sub: 0, line: 7 } |  |  | 0.548 |
| ns | 9740 |  | 82 | tests/ui file listing | 8.2 | 8.1 | 0.555 |
| ns | 9849 |  | 109 | One complete compile-fail pair — wrong-interpolation | 8.3 | 8.2 | 0.551 |
| walker |  | 9976 | 258 | Code::CodeKey { rung: Names, file: src/error.rs, decl: 0, sub: 2, line: 0 } |  |  | 0.566 |
| ns | 9990 |  | 141 | Shared test helpers in tests/common and tests/drop | 8.4 | 4.4 | 0.560 |
| walker |  | 9994 | 18 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 48, sub: 0, line: 951 } |  |  | 0.560 |
