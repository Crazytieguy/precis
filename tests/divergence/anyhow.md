Score(3000)=0.599 I=0.840 C=0.427 ns_rows≤3K=20/63 grid(1000/1442/2080/3000/4327/6240/9000)=0.751/0.806/0.679/0.599/0.572/0.473/0.531

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
| walker |  | 1127 | 45 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 1, sub: 0, line: 278 } |  |  | 0.682 |
| walker |  | 1137 | 10 | Code::CodeKey { rung: Body, file: src/lib.rs, decl: 9, sub: 0, line: 648 } |  |  | 0.683 |
| walker |  | 1227 | 90 | Fs::DirListing { dir: tests } |  |  | 0.809 |
| ns | 1244 |  | 235 | Complete roster of exported items in lib.rs | 2.1 |  | 0.804 |
| walker |  | 1263 | 36 | Code::CodeKey { rung: Names, file: src/macros.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.804 |
| walker |  | 1307 | 44 | Code::CodeKey { rung: Names, file: src/fmt.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.804 |
| walker |  | 1342 | 35 | Code::CodeKey { rung: Decl, file: src/fmt.rs, decl: 4, sub: 0, line: 69 } |  |  | 0.805 |
| walker |  | 1354 | 12 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 7, sub: 0, line: 618 } |  |  | 0.806 |
| walker |  | 1405 | 51 | Code::CodeKey { rung: Names, file: src/ensure.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.806 |
| walker |  | 1432 | 27 | Code::CodeKey { rung: Decl, file: src/ensure.rs, decl: 2, sub: 0, line: 58 } |  |  | 0.806 |
| ns | 1452 |  | 208 | impl Error — roster of every public method | 2.2 |  | 0.752 |
| walker |  | 1463 | 31 | Code::CodeKey { rung: Decl, file: src/ensure.rs, decl: 1, sub: 0, line: 35 } |  |  | 0.752 |
| ns | 1612 |  | 160 | Context trait — both method signatures with bounds | 2.3 | 2.1 | 0.742 |
| walker |  | 1629 | 166 | Code::CodeKey { rung: Decl, file: src/macros.rs, decl: 1, sub: 0, line: 56 } |  |  | 0.744 |
| walker |  | 1678 | 49 | Code::CodeKey { rung: Decl, file: src/fmt.rs, decl: 5, sub: 0, line: 75 } |  |  | 0.745 |
| ns | 1778 |  | 166 | Display representations — `{}` and `{:#}`, with sample output | 2.4 | 1.4 | 0.723 |
| walker |  | 1803 | 125 | Toml::Config { file: Cargo.toml } |  |  | 0.723 |
| walker |  | 1866 | 63 | Code::CodeKey { rung: Names, file: src/kind.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.723 |
| walker |  | 1886 | 20 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 7, sub: 0, line: 99 } |  |  | 0.723 |
| walker |  | 1912 | 26 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 2, sub: 0, line: 67 } |  |  | 0.723 |
| ns | 1915 |  | 137 | Debug representations — `{:?}` and `{:#?}` | 2.5 | 2.4 | 0.699 |
| walker |  | 1938 | 26 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 5, sub: 0, line: 89 } |  |  | 0.699 |
| walker |  | 1990 | 52 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 8, sub: 0, line: 114 } |  |  | 0.699 |
| walker |  | 1996 | 6 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 9, sub: 0, line: 116 } |  |  | 0.699 |
| walker |  | 2026 | 30 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 6, sub: 0, line: 90 } |  |  | 0.699 |
| ns | 2031 |  | 116 | Context trait doc — sealed, and outermost-first cause printing | 2.6 | 2.3 | 0.678 |
| walker |  | 2063 | 37 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 3, sub: 0, line: 68 } |  |  | 0.679 |
| walker |  | 2128 | 65 | Code::CodeKey { rung: Decl, file: src/fmt.rs, decl: 1, sub: 0, line: 6 } |  |  | 0.679 |
| ns | 2212 |  | 181 | Context + downcasting — the guarantee, in both directions | 2.7 | 2.6 | 0.650 |
| ns | 2360 |  | 148 | Result alias and Ok() helper semantics | 2.8 | 2.1 | 0.632 |
| walker |  | 2380 | 252 | Code::CodeKey { rung: Decl, file: src/macros.rs, decl: 3, sub: 0, line: 202 } |  |  | 0.636 |
| walker |  | 2384 | 4 | Fs::DirListing { dir: tests/common } |  |  | 0.636 |
| walker |  | 2388 | 4 | Fs::DirListing { dir: tests/drop } |  |  | 0.636 |
| walker |  | 2470 | 82 | Fs::DirListing { dir: tests/ui } |  |  | 0.638 |
| ns | 2536 |  | 176 | no_std support contract | 2.9 |  | 0.620 |
| walker |  | 2755 | 285 | Code::CodeKey { rung: ModuleDoc, file: src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.620 |
| walker |  | 2763 | 8 | Code::CodeKey { rung: Body, file: src/kind.rs, decl: 6, sub: 0, line: 90 } |  |  | 0.620 |
| ns | 2768 |  | 232 | Trait impls on Error — complete list | 2.10 | 2.2 | 0.598 |
| walker |  | 2874 | 111 | Code::CodeKey { rung: Names, file: src/chain.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.598 |
| walker |  | 2894 | 20 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 10, sub: 0, line: 76 } |  |  | 0.598 |
| walker |  | 2924 | 30 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 3, sub: 0, line: 26 } |  |  | 0.599 |
| walker |  | 2930 | 6 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 4, sub: 0, line: 27 } |  |  | 0.599 |
| walker |  | 2964 | 34 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 12, sub: 0, line: 93 } |  |  | 0.599 |
| walker |  | 2999 | 35 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.599 |
| ns | 3002 |  | 234 | What bail! and ensure! mean | 3.1 |  | 0.580 |
| walker |  | 3043 | 44 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 8, sub: 0, line: 56 } |  |  | 0.580 |
| walker |  | 3111 | 68 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 5, sub: 0, line: 35 } |  |  | 0.580 |
| ns | 3183 |  | 181 | What anyhow! constructs | 3.2 | 3.1 | 0.566 |
| walker |  | 3210 | 99 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 2, sub: 0, line: 15 } |  |  | 0.567 |
| ns | 3438 |  | 255 | Every exported macro_rules!, with its export attributes | 3.3 | 3.1 | 0.549 |
| walker |  | 3454 | 244 | Code::CodeKey { rung: Names, file: src/error.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.554 |
| walker |  | 3471 | 17 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 31, sub: 0, line: 731 } |  |  | 0.554 |
| walker |  | 3499 | 28 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 27, sub: 0, line: 719 } |  |  | 0.554 |
| walker |  | 3527 | 28 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 29, sub: 0, line: 725 } |  |  | 0.554 |
| walker |  | 3572 | 45 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 25, sub: 0, line: 712 } |  |  | 0.554 |
| walker |  | 3638 | 66 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 23, sub: 0, line: 703 } |  |  | 0.557 |
| ns | 3660 |  | 222 | anyhow! expansion — the three arms | 3.4 | 3.3 | 0.583 |
| walker |  | 3707 | 69 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 21, sub: 0, line: 691 } |  |  | 0.583 |
| walker |  | 3713 | 6 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 22, sub: 0, line: 696 } |  |  | 0.583 |
| walker |  | 3741 | 28 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 36, sub: 0, line: 775 } |  |  | 0.583 |
| walker |  | 3789 | 48 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 37, sub: 0, line: 787 } |  |  | 0.583 |
| ns | 3796 |  | 136 | bail! body | 3.5 | 3.3 | 0.595 |
| walker |  | 3837 | 48 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 38, sub: 0, line: 798 } |  |  | 0.595 |
| walker |  | 3844 | 7 | Code::CodeKey { rung: Body, file: src/lib.rs, decl: 2, sub: 0, line: 280 } |  |  | 0.595 |
| ns | 3957 |  | 161 | ensure! expansion — the fuel-limited parser call | 3.6 | 3.3 | 0.580 |
| walker |  | 3967 | 123 | Code::CodeKey { rung: Names, file: src/backtrace.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.581 |
| walker |  | 3986 | 19 | Code::CodeKey { rung: Decl, file: src/backtrace.rs, decl: 1, sub: 0, line: 7 } |  |  | 0.581 |
| walker |  | 4026 | 40 | Code::CodeKey { rung: Decl, file: src/backtrace.rs, decl: 2, sub: 0, line: 10 } |  |  | 0.581 |
| walker |  | 4071 | 45 | Code::CodeKey { rung: Decl, file: src/backtrace.rs, decl: 5, sub: 0, line: 31 } |  |  | 0.581 |
| walker |  | 4122 | 51 | Code::CodeKey { rung: Decl, file: src/backtrace.rs, decl: 4, sub: 0, line: 24 } |  |  | 0.581 |
| ns | 4126 |  | 169 | kind.rs — why autoref tagged dispatch | 3.7 | 3.4 | 0.572 |
| walker |  | 4174 | 52 | Code::CodeKey { rung: Decl, file: src/backtrace.rs, decl: 3, sub: 0, line: 17 } |  |  | 0.572 |
| walker |  | 4257 | 83 | Code::CodeKey { rung: Decl, file: src/backtrace.rs, decl: 8, sub: 0, line: 59 } |  |  | 0.572 |
| walker |  | 4344 | 87 | Code::CodeKey { rung: Decl, file: src/backtrace.rs, decl: 6, sub: 0, line: 38 } |  |  | 0.573 |
| ns | 4375 |  | 249 | kind.rs — the three kinds and their new() constructors | 3.8 | 3.7 | 0.565 |
| walker |  | 4430 | 86 | Code::CodeKey { rung: Decl, file: src/backtrace.rs, decl: 7, sub: 0, line: 48 } |  |  | 0.565 |
| ns | 4627 |  | 252 | ensure.rs runtime side — BothDebug / NotBothDebug dispatch | 3.9 | 3.6 | 0.549 |
| walker |  | 4634 | 204 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 1, sub: 0, line: 19 } |  |  | 0.551 |
| walker |  | 4672 | 38 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 4, sub: 0, line: 137 } |  |  | 0.551 |
| walker |  | 4720 | 48 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 3, sub: 0, line: 75 } |  |  | 0.551 |
| walker |  | 4788 | 68 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 2, sub: 0, line: 27 } |  |  | 0.551 |
| ns | 4798 |  | 171 | __parse_ensure! extent, first rule, and catch-all | 3.10 | 3.6 | 0.541 |
| walker |  | 4825 | 37 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 7, sub: 0, line: 197 } |  |  | 0.541 |
| walker |  | 4864 | 39 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 6, sub: 0, line: 169 } |  |  | 0.541 |
| walker |  | 4923 | 59 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 5, sub: 0, line: 145 } |  |  | 0.541 |
| walker |  | 4991 | 68 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 9, sub: 0, line: 258 } |  |  | 0.541 |
| ns | 4995 |  | 197 | lib.rs __private — the macro-facing surface | 3.11 | 3.3 | 0.530 |
| ns | 5156 |  | 161 | test_fmt.rs — the f/g/h fixture and expected `{:#}` strings | 4.1 | 2.4 | 0.523 |
| walker |  | 5163 | 172 | Code::CodeKey { rung: Names, file: src/context.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.523 |
| walker |  | 5211 | 48 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 9, sub: 0, line: 128 } |  |  | 0.523 |
| walker |  | 5263 | 52 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 4, sub: 0, line: 90 } |  |  | 0.523 |
| walker |  | 5292 | 29 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 5, sub: 0, line: 91 } |  |  | 0.523 |
| ns | 5295 |  | 139 | test_fmt.rs — expected `{:?}` strings, including numbered causes | 4.2 | 4.1 | 0.511 |
| walker |  | 5349 | 57 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 7, sub: 0, line: 115 } |  |  | 0.511 |
| walker |  | 5421 | 72 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 14, sub: 0, line: 152 } |  |  | 0.511 |
| walker |  | 5433 | 12 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 16, sub: 0, line: 160 } |  |  | 0.511 |
| ns | 5454 |  | 159 | test_repr.rs — size, null-pointer-optimization, autotrait assertions | 4.3 | 1.5 | 0.501 |
| walker |  | 5475 | 42 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 6, sub: 0, line: 103 } |  |  | 0.501 |
| walker |  | 5559 | 84 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 1, sub: 0, line: 42 } |  |  | 0.501 |
| walker |  | 5588 | 29 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 2, sub: 0, line: 46 } |  |  | 0.501 |
| walker |  | 5630 | 42 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 3, sub: 0, line: 58 } |  |  | 0.501 |
| ns | 5633 |  | 179 | test_downcast.rs — what a bail!'d error downcasts to | 4.4 | 3.4 | 0.489 |
| walker |  | 5715 | 85 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 11, sub: 0, line: 137 } |  |  | 0.489 |
| walker |  | 5727 | 12 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 13, sub: 0, line: 146 } |  |  | 0.489 |
| walker |  | 5756 | 29 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 20, sub: 0, line: 180 } |  |  | 0.489 |
| walker |  | 5804 | 48 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 18, sub: 0, line: 168 } |  |  | 0.489 |
| ns | 5807 |  | 174 | test_chain.rs — cause iteration order | 4.5 |  | 0.481 |
| ns | 6002 |  | 195 | test_context.rs — downcast_ref through a three-level context chain | 4.6 | 2.7 | 0.473 |
| walker |  | 6108 | 304 | Code::CodeKey { rung: ModuleDoc, file: src/lib.rs, decl: 0, sub: 1, line: 0 } |  |  | 0.473 |
| ns | 6252 |  | 250 | ErrorVTable — the hand-rolled vtable layout | 5.1 |  | 0.466 |
| walker |  | 6282 | 174 | Code::CodeKey { rung: Names, file: src/nightly.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.466 |
| walker |  | 6293 | 11 | Code::CodeKey { rung: Decl, file: src/nightly.rs, decl: 2, sub: 0, line: 38 } |  |  | 0.466 |
| walker |  | 6305 | 12 | Code::CodeKey { rung: Body, file: src/nightly.rs, decl: 6, sub: 0, line: 56 } |  |  | 0.466 |
| walker |  | 6318 | 13 | Code::CodeKey { rung: Body, file: src/nightly.rs, decl: 3, sub: 0, line: 41 } |  |  | 0.466 |
| walker |  | 6342 | 24 | Code::CodeKey { rung: Decl, file: src/nightly.rs, decl: 4, sub: 0, line: 45 } |  |  | 0.466 |
| walker |  | 6356 | 14 | Code::CodeKey { rung: Body, file: src/nightly.rs, decl: 5, sub: 0, line: 52 } |  |  | 0.466 |
| walker |  | 6386 | 30 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 8, sub: 0, line: 624 } |  |  | 0.475 |
| walker |  | 6400 | 14 | Code::CodeKey { rung: Body, file: src/nightly.rs, decl: 4, sub: 0, line: 45 } |  |  | 0.475 |
| ns | 6483 |  | 231 | ErrorImpl, ContextError, and the vtable reader | 5.2 | 5.1 | 0.465 |
| walker |  | 6610 | 210 | Code::CodeKey { rung: Names, file: src/wrapper.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.466 |
| ns | 6615 |  | 132 | Error::construct — allocation and type erasure | 5.3 | 5.2 | 0.459 |
| walker |  | 6617 | 7 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 7, sub: 0, line: 33 } |  |  | 0.459 |
| walker |  | 6626 | 9 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.459 |
| walker |  | 6653 | 27 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 13, sub: 0, line: 56 } |  |  | 0.459 |
| walker |  | 6701 | 48 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 8, sub: 0, line: 36 } |  |  | 0.459 |
| ns | 6718 |  | 103 | impl Error — roster of the non-public fns | 5.4 | 5.3 | 0.463 |
| walker |  | 6749 | 48 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 10, sub: 0, line: 45 } |  |  | 0.463 |
| walker |  | 6797 | 48 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 14, sub: 0, line: 60 } |  |  | 0.463 |
| walker |  | 6845 | 48 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 16, sub: 0, line: 67 } |  |  | 0.463 |
| ns | 6889 |  | 171 | Vtable function roster in error.rs | 5.5 | 5.1 | 0.460 |
| walker |  | 6895 | 50 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 2, sub: 0, line: 13 } |  |  | 0.460 |
| walker |  | 6945 | 50 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 4, sub: 0, line: 22 } |  |  | 0.460 |
| walker |  | 7015 | 70 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 18, sub: 0, line: 74 } |  |  | 0.460 |
| walker |  | 7027 | 12 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 20, sub: 0, line: 80 } |  |  | 0.460 |
| ns | 7040 |  | 151 | ErrorImpl methods and trait impls | 5.6 | 5.2 | 0.455 |
| walker |  | 7164 | 137 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.455 |
| walker |  | 7237 | 73 | Code::CodeKey { rung: Body, file: build.rs, decl: 4, sub: 0, line: 199 } |  |  | 0.455 |
| ns | 7275 |  | 235 | ptr.rs — Own / Ref / Mut / CastTo declarations | 5.7 |  | 0.443 |
| ns | 7440 |  | 165 | chain.rs — Chain and ChainState | 6.1 |  | 0.455 |
| walker |  | 7553 | 316 | Code::CodeKey { rung: Names, file: src/ptr.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.457 |
| ns | 7557 |  | 117 | chain.rs — every trait impl on Chain | 6.2 | 6.1 | 0.464 |
| walker |  | 7564 | 11 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 35, sub: 0, line: 181 } |  |  | 0.465 |
| walker |  | 7577 | 13 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 36, sub: 0, line: 185 } |  |  | 0.465 |
| walker |  | 7598 | 21 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 33, sub: 0, line: 174 } |  |  | 0.465 |
| walker |  | 7637 | 39 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 5, sub: 0, line: 19 } |  |  | 0.465 |
| walker |  | 7676 | 39 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 15, sub: 0, line: 74 } |  |  | 0.465 |
| walker |  | 7715 | 39 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 26, sub: 0, line: 135 } |  |  | 0.465 |
| walker |  | 7760 | 45 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 1, sub: 0, line: 5 } |  |  | 0.470 |
| ns | 7782 |  | 225 | wrapper.rs — the three error adapters and their impls | 6.3 |  | 0.483 |
| walker |  | 7819 | 59 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 13, sub: 0, line: 63 } |  |  | 0.494 |
| walker |  | 7879 | 60 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 24, sub: 0, line: 124 } |  |  | 0.511 |
| ns | 7959 |  | 177 | context.rs — where .context() is implemented | 6.4 | 2.3 | 0.506 |
| walker |  | 7995 | 116 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 28, sub: 0, line: 144 } |  |  | 0.506 |
| walker |  | 8118 | 123 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 7, sub: 0, line: 28 } |  |  | 0.506 |
| ns | 8133 |  | 174 | context.rs — ContextError impls, Quoted, and the Sealed trait | 6.5 | 6.4 | 0.506 |
| ns | 8261 |  | 128 | fmt.rs — the rendering entry points | 6.6 | 5.6 | 0.511 |
| walker |  | 8266 | 148 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 17, sub: 0, line: 83 } |  |  | 0.511 |
| ns | 8413 |  | 152 | build.rs — the complete custom-cfg vocabulary | 7.1 |  | 0.507 |
| walker |  | 8512 | 246 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 1, sub: 1, line: 19 } |  |  | 0.527 |
| ns | 8527 |  | 114 | build.rs — which compiler turns each cfg on | 7.2 | 7.1 | 0.524 |
| walker |  | 8531 | 19 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 11, sub: 0, line: 431 } |  |  | 0.524 |
| walker |  | 8560 | 29 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 12, sub: 0, line: 457 } |  |  | 0.524 |
| walker |  | 8591 | 31 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 14, sub: 0, line: 482 } |  |  | 0.524 |
| walker |  | 8622 | 31 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 15, sub: 0, line: 490 } |  |  | 0.524 |
| ns | 8637 |  | 110 | backtrace.rs — the three definitions of Backtrace | 7.3 |  | 0.521 |
| walker |  | 8653 | 31 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 16, sub: 0, line: 554 } |  |  | 0.521 |
| walker |  | 8684 | 31 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 17, sub: 0, line: 568 } |  |  | 0.521 |
| walker |  | 8721 | 37 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 13, sub: 0, line: 468 } |  |  | 0.521 |
| walker |  | 8733 | 12 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 20, sub: 0, line: 674 } |  |  | 0.521 |
| walker |  | 8779 | 46 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 10, sub: 0, line: 370 } |  |  | 0.521 |
| ns | 8813 |  | 176 | backtrace.rs — impl_backtrace! and backtrace!(), both arms each | 7.4 | 7.3 | 0.529 |
| walker |  | 8855 | 76 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 18, sub: 0, line: 598 } |  |  | 0.529 |
| ns | 8954 |  | 141 | backtrace.rs — backtrace_if_absent! and the vendored capture module | 7.5 | 7.3 | 0.531 |
| walker |  | 8959 | 104 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 19, sub: 0, line: 640 } |  |  | 0.531 |
| walker |  | 9074 | 115 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 8, sub: 0, line: 225 } |  |  | 0.531 |
| ns | 9086 |  | 132 | nightly.rs — the build probe and generic-member-access shims | 7.6 | 7.1 | 0.528 |
| ns | 9217 |  | 131 | Toolchain pin and the no_std check-crate | 7.7 |  | 0.522 |
| walker |  | 9327 | 253 | Code::CodeKey { rung: ModuleDoc, file: src/lib.rs, decl: 0, sub: 2, line: 0 } |  |  | 0.522 |
| ns | 9378 |  | 161 | CI job roster and toolchain matrix | 7.8 |  | 0.517 |
| walker |  | 9563 | 236 | Code::CodeKey { rung: Decl, file: src/nightly.rs, decl: 1, sub: 0, line: 12 } |  |  | 0.517 |
| ns | 9570 |  | 192 | CI commands — the canonical build/test/lint invocations | 7.9 | 7.8 | 0.513 |
| ns | 9658 |  | 88 | trybuild UI harness — tests/compiletest.rs in full | 8.1 |  | 0.510 |
| ns | 9740 |  | 82 | tests/ui file listing | 8.2 | 8.1 | 0.518 |
| walker |  | 9787 | 224 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.518 |
| ns | 9849 |  | 109 | One complete compile-fail pair — wrong-interpolation | 8.3 | 8.2 | 0.514 |
| ns | 9990 |  | 141 | Shared test helpers in tests/common and tests/drop | 8.4 | 4.4 | 0.509 |
