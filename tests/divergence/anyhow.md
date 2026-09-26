Score(3000)=0.618 I=0.827 C=0.461 ns_rows≤3K=20/63 grid(1000/1442/2080/3000/4327/6240/9000)=0.751/0.809/0.695/0.618/0.539/0.510/0.556

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 34 | 34 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 85 |  | 85 | Crate identity — name, version, description | 1.1 |  | 0.000 |
| walker |  | 101 | 67 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.000 |
| ns | 119 |  | 34 | Repository root listing | 1.2 |  | 0.420 |
| walker |  | 150 | 49 | Fs::DirListing { dir: src } |  |  | 0.527 |
| walker |  | 158 | 8 | Fs::DirListing { dir: .github } |  |  | 0.527 |
| walker |  | 162 | 4 | Fs::DirListing { dir: .github/workflows } |  |  | 0.527 |
| ns | 168 |  | 49 | src/ module inventory | 1.3 |  | 0.548 |
| walker |  | 189 | 27 | Toml::Operational { file: Cargo.toml } |  |  | 0.549 |
| walker |  | 233 | 44 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.558 |
| ns | 259 |  | 91 | What `anyhow::Error` is — a Box<dyn Error> that must be Send + Sync | 1.4 |  | 0.494 |
| walker |  | 307 | 74 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.504 |
| ns | 333 |  | 74 | …and the other two guarantees: always a backtrace, one word wide | 1.5 | 1.4 | 0.463 |
| ns | 417 |  | 84 | Cargo.toml package tail — MSRV, edition, license, links | 1.6 |  | 0.424 |
| walker |  | 472 | 165 | Toml::Identity { file: Cargo.toml } |  |  | 0.850 |
| ns | 507 |  | 90 | tests/ inventory | 1.7 |  | 0.693 |
| walker |  | 582 | 110 | Toml::Dependencies { file: Cargo.toml } |  |  | 0.703 |
| ns | 645 |  | 138 | Cargo features — std default, optional backtrace | 1.8 |  | 0.723 |
| ns | 793 |  | 148 | README lede, install snippet, and section map | 1.9 |  | 0.737 |
| walker |  | 807 | 225 | Code::CodeKey { rung: Names, file: src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.743 |
| walker |  | 825 | 18 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 9, sub: 0, line: 648 } |  |  | 0.744 |
| walker |  | 851 | 26 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 3, sub: 0, line: 389 } |  |  | 0.746 |
| walker |  | 896 | 45 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 1, sub: 0, line: 278 } |  |  | 0.746 |
| walker |  | 945 | 49 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 4, sub: 0, line: 413 } |  |  | 0.751 |
| walker |  | 1001 | 56 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 6, sub: 0, line: 616 } |  |  | 0.751 |
| ns | 1009 |  | 216 | lib.rs crate attributes and module declarations | 1.10 |  | 0.680 |
| walker |  | 1024 | 23 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 7, sub: 0, line: 618 } |  |  | 0.681 |
| walker |  | 1058 | 34 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 8, sub: 0, line: 624 } |  |  | 0.682 |
| walker |  | 1070 | 12 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 7, sub: 0, line: 618 } |  |  | 0.683 |
| walker |  | 1160 | 90 | Fs::DirListing { dir: tests } |  |  | 0.809 |
| walker |  | 1164 | 4 | Fs::DirListing { dir: tests/common } |  |  | 0.809 |
| walker |  | 1168 | 4 | Fs::DirListing { dir: tests/drop } |  |  | 0.809 |
| walker |  | 1182 | 14 | Fs::DirListing { dir: tests/crate } |  |  | 0.809 |
| walker |  | 1189 | 7 | Code::CodeKey { rung: Body, file: src/lib.rs, decl: 2, sub: 0, line: 280 } |  |  | 0.809 |
| ns | 1244 |  | 235 | Complete roster of exported items in lib.rs | 2.1 |  | 0.797 |
| walker |  | 1258 | 69 | Code::CodeKey { rung: Names, file: build.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.797 |
| walker |  | 1288 | 30 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 8, sub: 0, line: 624 } |  |  | 0.799 |
| walker |  | 1298 | 10 | Code::CodeKey { rung: Body, file: src/lib.rs, decl: 9, sub: 0, line: 648 } |  |  | 0.807 |
| walker |  | 1380 | 82 | Fs::DirListing { dir: tests/ui } |  |  | 0.809 |
| ns | 1452 |  | 208 | impl Error — roster of every public method | 2.2 |  | 0.755 |
| walker |  | 1505 | 125 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.755 |
| ns | 1612 |  | 160 | Context trait — both method signatures with bounds | 2.3 | 2.1 | 0.762 |
| walker |  | 1642 | 137 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.762 |
| walker |  | 1733 | 91 | Code::CodeKey { rung: Names, file: src/ensure.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.762 |
| walker |  | 1759 | 26 | Code::CodeKey { rung: Decl, file: src/ensure.rs, decl: 3, sub: 0, line: 29 } |  |  | 0.762 |
| ns | 1778 |  | 166 | Display representations — `{}` and `{:#}`, with sample output | 2.4 | 1.4 | 0.740 |
| walker |  | 1786 | 27 | Code::CodeKey { rung: Decl, file: src/ensure.rs, decl: 6, sub: 0, line: 58 } |  |  | 0.740 |
| walker |  | 1817 | 31 | Code::CodeKey { rung: Decl, file: src/ensure.rs, decl: 5, sub: 0, line: 35 } |  |  | 0.740 |
| walker |  | 1874 | 57 | Code::CodeKey { rung: Decl, file: src/ensure.rs, decl: 1, sub: 0, line: 14 } |  |  | 0.740 |
| walker |  | 1884 | 10 | Code::CodeKey { rung: Body, file: src/ensure.rs, decl: 4, sub: 0, line: 30 } |  |  | 0.741 |
| walker |  | 1902 | 18 | Code::CodeKey { rung: Body, file: src/ensure.rs, decl: 2, sub: 0, line: 19 } |  |  | 0.741 |
| ns | 1915 |  | 137 | Debug representations — `{:?}` and `{:#?}` | 2.5 | 2.4 | 0.716 |
| ns | 2031 |  | 116 | Context trait doc — sealed, and outermost-first cause printing | 2.6 | 2.3 | 0.695 |
| walker |  | 2126 | 224 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.695 |
| ns | 2212 |  | 181 | Context + downcasting — the guarantee, in both directions | 2.7 | 2.6 | 0.665 |
| walker |  | 2344 | 218 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.665 |
| ns | 2360 |  | 148 | Result alias and Ok() helper semantics | 2.8 | 2.1 | 0.646 |
| ns | 2536 |  | 176 | no_std support contract | 2.9 |  | 0.629 |
| walker |  | 2552 | 208 | Code::CodeKey { rung: Names, file: src/error.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.630 |
| walker |  | 2569 | 17 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 31, sub: 0, line: 731 } |  |  | 0.630 |
| walker |  | 2597 | 28 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 27, sub: 0, line: 719 } |  |  | 0.630 |
| walker |  | 2625 | 28 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 29, sub: 0, line: 725 } |  |  | 0.630 |
| walker |  | 2653 | 28 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 36, sub: 0, line: 775 } |  |  | 0.630 |
| walker |  | 2698 | 45 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 25, sub: 0, line: 712 } |  |  | 0.630 |
| walker |  | 2746 | 48 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 37, sub: 0, line: 787 } |  |  | 0.630 |
| ns | 2768 |  | 232 | Trait impls on Error — complete list | 2.10 | 2.2 | 0.612 |
| walker |  | 2812 | 66 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 23, sub: 0, line: 703 } |  |  | 0.615 |
| walker |  | 2881 | 69 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 21, sub: 0, line: 691 } |  |  | 0.615 |
| walker |  | 2887 | 6 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 22, sub: 0, line: 696 } |  |  | 0.615 |
| ns | 3002 |  | 234 | What bail! and ensure! mean | 3.1 |  | 0.596 |
| walker |  | 3057 | 170 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 1, sub: 0, line: 19 } |  |  | 0.598 |
| walker |  | 3094 | 37 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 7, sub: 0, line: 197 } |  |  | 0.598 |
| walker |  | 3132 | 38 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 4, sub: 0, line: 137 } |  |  | 0.598 |
| walker |  | 3171 | 39 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 6, sub: 0, line: 169 } |  |  | 0.598 |
| ns | 3183 |  | 181 | What anyhow! constructs | 3.2 | 3.1 | 0.583 |
| walker |  | 3219 | 48 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 3, sub: 0, line: 75 } |  |  | 0.583 |
| walker |  | 3278 | 59 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 5, sub: 0, line: 145 } |  |  | 0.583 |
| walker |  | 3346 | 68 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 2, sub: 0, line: 27 } |  |  | 0.583 |
| ns | 3438 |  | 255 | Every exported macro_rules!, with its export attributes | 3.3 | 3.1 | 0.557 |
| walker |  | 3617 | 271 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 33, sub: 0, line: 740 } |  |  | 0.558 |
| ns | 3660 |  | 222 | anyhow! expansion — the three arms | 3.4 | 3.3 | 0.537 |
| walker |  | 3795 | 178 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 1, sub: 1, line: 19 } |  |  | 0.554 |
| ns | 3796 |  | 136 | bail! body | 3.5 | 3.3 | 0.543 |
| walker |  | 3814 | 19 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 11, sub: 0, line: 431 } |  |  | 0.543 |
| walker |  | 3843 | 29 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 12, sub: 0, line: 457 } |  |  | 0.543 |
| walker |  | 3874 | 31 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 14, sub: 0, line: 482 } |  |  | 0.543 |
| walker |  | 3905 | 31 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 15, sub: 0, line: 490 } |  |  | 0.543 |
| walker |  | 3936 | 31 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 16, sub: 0, line: 554 } |  |  | 0.543 |
| ns | 3957 |  | 161 | ensure! expansion — the fuel-limited parser call | 3.6 | 3.3 | 0.530 |
| walker |  | 3973 | 37 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 13, sub: 0, line: 468 } |  |  | 0.530 |
| walker |  | 4019 | 46 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 10, sub: 0, line: 370 } |  |  | 0.530 |
| walker |  | 4087 | 68 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 9, sub: 0, line: 258 } |  |  | 0.530 |
| ns | 4126 |  | 169 | kind.rs — why autoref tagged dispatch | 3.7 | 3.4 | 0.522 |
| walker |  | 4202 | 115 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 8, sub: 0, line: 225 } |  |  | 0.522 |
| walker |  | 4304 | 102 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 1, sub: 2, line: 19 } |  |  | 0.539 |
| walker |  | 4316 | 12 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 20, sub: 0, line: 674 } |  |  | 0.539 |
| walker |  | 4347 | 31 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 17, sub: 0, line: 568 } |  |  | 0.539 |
| ns | 4375 |  | 249 | kind.rs — the three kinds and their new() constructors | 3.8 | 3.7 | 0.524 |
| walker |  | 4423 | 76 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 18, sub: 0, line: 598 } |  |  | 0.524 |
| walker |  | 4527 | 104 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 19, sub: 0, line: 640 } |  |  | 0.524 |
| walker |  | 4542 | 15 | Code::CodeKey { rung: Doc, file: src/error.rs, decl: 17, sub: 0, line: 568 } |  |  | 0.524 |
| ns | 4627 |  | 252 | ensure.rs runtime side — BothDebug / NotBothDebug dispatch | 3.9 | 3.6 | 0.524 |
| walker |  | 4665 | 123 | Code::CodeKey { rung: Names, file: src/backtrace.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.524 |
| walker |  | 4684 | 19 | Code::CodeKey { rung: Decl, file: src/backtrace.rs, decl: 1, sub: 0, line: 7 } |  |  | 0.524 |
| walker |  | 4724 | 40 | Code::CodeKey { rung: Decl, file: src/backtrace.rs, decl: 2, sub: 0, line: 10 } |  |  | 0.524 |
| walker |  | 4769 | 45 | Code::CodeKey { rung: Decl, file: src/backtrace.rs, decl: 5, sub: 0, line: 31 } |  |  | 0.524 |
| ns | 4798 |  | 171 | __parse_ensure! extent, first rule, and catch-all | 3.10 | 3.6 | 0.514 |
| walker |  | 4820 | 51 | Code::CodeKey { rung: Decl, file: src/backtrace.rs, decl: 4, sub: 0, line: 24 } |  |  | 0.515 |
| walker |  | 4872 | 52 | Code::CodeKey { rung: Decl, file: src/backtrace.rs, decl: 3, sub: 0, line: 17 } |  |  | 0.515 |
| walker |  | 4955 | 83 | Code::CodeKey { rung: Decl, file: src/backtrace.rs, decl: 8, sub: 0, line: 59 } |  |  | 0.515 |
| ns | 4995 |  | 197 | lib.rs __private — the macro-facing surface | 3.11 | 3.3 | 0.504 |
| walker |  | 5042 | 87 | Code::CodeKey { rung: Decl, file: src/backtrace.rs, decl: 6, sub: 0, line: 38 } |  |  | 0.505 |
| walker |  | 5128 | 86 | Code::CodeKey { rung: Decl, file: src/backtrace.rs, decl: 7, sub: 0, line: 48 } |  |  | 0.505 |
| ns | 5156 |  | 161 | test_fmt.rs — the f/g/h fixture and expected `{:#}` strings | 4.1 | 2.4 | 0.498 |
| walker |  | 5164 | 36 | Code::CodeKey { rung: Names, file: src/macros.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.499 |
| ns | 5295 |  | 139 | test_fmt.rs — expected `{:?}` strings, including numbered causes | 4.2 | 4.1 | 0.487 |
| walker |  | 5330 | 166 | Code::CodeKey { rung: Decl, file: src/macros.rs, decl: 1, sub: 0, line: 56 } |  |  | 0.510 |
| ns | 5454 |  | 159 | test_repr.rs — size, null-pointer-optimization, autotrait assertions | 4.3 | 1.5 | 0.500 |
| walker |  | 5582 | 252 | Code::CodeKey { rung: Decl, file: src/macros.rs, decl: 3, sub: 0, line: 202 } |  |  | 0.540 |
| walker |  | 5600 | 18 | Code::CodeKey { rung: Doc, file: src/error.rs, decl: 15, sub: 0, line: 490 } |  |  | 0.540 |
| ns | 5633 |  | 179 | test_downcast.rs — what a bail!'d error downcasts to | 4.4 | 3.4 | 0.526 |
| walker |  | 5751 | 151 | Code::CodeKey { rung: Names, file: src/error.rs, decl: 0, sub: 1, line: 0 } |  |  | 0.527 |
| walker |  | 5772 | 21 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 39, sub: 0, line: 809 } |  |  | 0.527 |
| ns | 5807 |  | 174 | test_chain.rs — cause iteration order | 4.5 |  | 0.519 |
| walker |  | 5812 | 40 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 40, sub: 0, line: 828 } |  |  | 0.519 |
| walker |  | 5860 | 48 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 38, sub: 0, line: 798 } |  |  | 0.519 |
| walker |  | 5911 | 51 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 41, sub: 0, line: 838 } |  |  | 0.519 |
| walker |  | 5962 | 51 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 42, sub: 0, line: 858 } |  |  | 0.519 |
| ns | 6002 |  | 195 | test_context.rs — downcast_ref through a three-level context chain | 4.6 | 2.7 | 0.510 |
| walker |  | 6134 | 172 | Code::CodeKey { rung: Names, file: src/context.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.510 |
| walker |  | 6163 | 29 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 20, sub: 0, line: 180 } |  |  | 0.510 |
| walker |  | 6211 | 48 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 9, sub: 0, line: 128 } |  |  | 0.510 |
| ns | 6252 |  | 250 | ErrorVTable — the hand-rolled vtable layout | 5.1 |  | 0.523 |
| walker |  | 6259 | 48 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 18, sub: 0, line: 168 } |  |  | 0.523 |
| walker |  | 6311 | 52 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 4, sub: 0, line: 90 } |  |  | 0.523 |
| walker |  | 6340 | 29 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 5, sub: 0, line: 91 } |  |  | 0.523 |
| walker |  | 6382 | 42 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 6, sub: 0, line: 103 } |  |  | 0.523 |
| walker |  | 6439 | 57 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 7, sub: 0, line: 115 } |  |  | 0.523 |
| ns | 6483 |  | 231 | ErrorImpl, ContextError, and the vtable reader | 5.2 | 5.1 | 0.512 |
| walker |  | 6511 | 72 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 14, sub: 0, line: 152 } |  |  | 0.512 |
| walker |  | 6523 | 12 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 16, sub: 0, line: 160 } |  |  | 0.512 |
| walker |  | 6607 | 84 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 1, sub: 0, line: 42 } |  |  | 0.512 |
| ns | 6615 |  | 132 | Error::construct — allocation and type erasure | 5.3 | 5.2 | 0.505 |
| walker |  | 6636 | 29 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 2, sub: 0, line: 46 } |  |  | 0.505 |
| walker |  | 6678 | 42 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 3, sub: 0, line: 58 } |  |  | 0.505 |
| ns | 6718 |  | 103 | impl Error — roster of the non-public fns | 5.4 | 5.3 | 0.510 |
| walker |  | 6763 | 85 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 11, sub: 0, line: 137 } |  |  | 0.510 |
| walker |  | 6775 | 12 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 13, sub: 0, line: 146 } |  |  | 0.510 |
| walker |  | 6819 | 44 | Code::CodeKey { rung: Names, file: src/fmt.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.510 |
| walker |  | 6854 | 35 | Code::CodeKey { rung: Decl, file: src/fmt.rs, decl: 4, sub: 0, line: 69 } |  |  | 0.510 |
| ns | 6889 |  | 171 | Vtable function roster in error.rs | 5.5 | 5.1 | 0.513 |
| walker |  | 6903 | 49 | Code::CodeKey { rung: Decl, file: src/fmt.rs, decl: 5, sub: 0, line: 75 } |  |  | 0.513 |
| walker |  | 6968 | 65 | Code::CodeKey { rung: Decl, file: src/fmt.rs, decl: 1, sub: 0, line: 6 } |  |  | 0.513 |
| ns | 7040 |  | 151 | ErrorImpl methods and trait impls | 5.6 | 5.2 | 0.507 |
| ns | 7275 |  | 235 | ptr.rs — Own / Ref / Mut / CastTo declarations | 5.7 |  | 0.494 |
| walker |  | 7284 | 316 | Code::CodeKey { rung: Names, file: src/ptr.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.495 |
| walker |  | 7295 | 11 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 35, sub: 0, line: 181 } |  |  | 0.496 |
| walker |  | 7308 | 13 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 36, sub: 0, line: 185 } |  |  | 0.496 |
| walker |  | 7329 | 21 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 33, sub: 0, line: 174 } |  |  | 0.496 |
| walker |  | 7368 | 39 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 5, sub: 0, line: 19 } |  |  | 0.496 |
| walker |  | 7407 | 39 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 15, sub: 0, line: 74 } |  |  | 0.496 |
| ns | 7440 |  | 165 | chain.rs — Chain and ChainState | 6.1 |  | 0.489 |
| walker |  | 7446 | 39 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 26, sub: 0, line: 135 } |  |  | 0.489 |
| walker |  | 7491 | 45 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 1, sub: 0, line: 5 } |  |  | 0.495 |
| walker |  | 7550 | 59 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 13, sub: 0, line: 63 } |  |  | 0.506 |
| ns | 7557 |  | 117 | chain.rs — every trait impl on Chain | 6.2 | 6.1 | 0.502 |
| walker |  | 7610 | 60 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 24, sub: 0, line: 124 } |  |  | 0.519 |
| walker |  | 7726 | 116 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 28, sub: 0, line: 144 } |  |  | 0.519 |
| ns | 7782 |  | 225 | wrapper.rs — the three error adapters and their impls | 6.3 |  | 0.511 |
| walker |  | 7849 | 123 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 7, sub: 0, line: 28 } |  |  | 0.511 |
| ns | 7959 |  | 177 | context.rs — where .context() is implemented | 6.4 | 2.3 | 0.507 |
| walker |  | 7997 | 148 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 17, sub: 0, line: 83 } |  |  | 0.507 |
| ns | 8133 |  | 174 | context.rs — ContextError impls, Quoted, and the Sealed trait | 6.5 | 6.4 | 0.507 |
| walker |  | 8141 | 144 | Code::CodeKey { rung: Names, file: src/kind.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.511 |
| walker |  | 8161 | 20 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 9, sub: 0, line: 99 } |  |  | 0.512 |
| walker |  | 8181 | 20 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 10, sub: 0, line: 111 } |  |  | 0.512 |
| walker |  | 8207 | 26 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 3, sub: 0, line: 67 } |  |  | 0.513 |
| walker |  | 8233 | 26 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 7, sub: 0, line: 89 } |  |  | 0.514 |
| ns | 8261 |  | 128 | fmt.rs — the rendering entry points | 6.6 | 5.6 | 0.519 |
| walker |  | 8263 | 30 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 8, sub: 0, line: 90 } |  |  | 0.521 |
| walker |  | 8300 | 37 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 4, sub: 0, line: 68 } |  |  | 0.523 |
| walker |  | 8350 | 50 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 11, sub: 0, line: 114 } |  |  | 0.524 |
| walker |  | 8356 | 6 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 12, sub: 0, line: 116 } |  |  | 0.524 |
| walker |  | 8364 | 8 | Code::CodeKey { rung: Body, file: src/kind.rs, decl: 8, sub: 0, line: 90 } |  |  | 0.524 |
| ns | 8413 |  | 152 | build.rs — the complete custom-cfg vocabulary | 7.1 |  | 0.521 |
| walker |  | 8475 | 111 | Code::CodeKey { rung: Names, file: src/chain.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.524 |
| walker |  | 8495 | 20 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 10, sub: 0, line: 76 } |  |  | 0.525 |
| walker |  | 8525 | 30 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 3, sub: 0, line: 26 } |  |  | 0.526 |
| ns | 8527 |  | 114 | build.rs — which compiler turns each cfg on | 7.2 | 7.1 | 0.523 |
| walker |  | 8531 | 6 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 4, sub: 0, line: 27 } |  |  | 0.524 |
| walker |  | 8565 | 34 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 12, sub: 0, line: 93 } |  |  | 0.524 |
| walker |  | 8600 | 35 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.524 |
| ns | 8637 |  | 110 | backtrace.rs — the three definitions of Backtrace | 7.3 |  | 0.521 |
| walker |  | 8644 | 44 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 8, sub: 0, line: 56 } |  |  | 0.523 |
| walker |  | 8712 | 68 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 5, sub: 0, line: 35 } |  |  | 0.526 |
| walker |  | 8811 | 99 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 2, sub: 0, line: 15 } |  |  | 0.539 |
| ns | 8813 |  | 176 | backtrace.rs — impl_backtrace! and backtrace!(), both arms each | 7.4 | 7.3 | 0.546 |
| ns | 8954 |  | 141 | backtrace.rs — backtrace_if_absent! and the vendored capture module | 7.5 | 7.3 | 0.548 |
| walker |  | 9021 | 210 | Code::CodeKey { rung: Names, file: src/wrapper.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.560 |
| walker |  | 9028 | 7 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 7, sub: 0, line: 33 } |  |  | 0.561 |
| walker |  | 9037 | 9 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.563 |
| walker |  | 9064 | 27 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 13, sub: 0, line: 56 } |  |  | 0.564 |
| ns | 9086 |  | 132 | nightly.rs — the build probe and generic-member-access shims | 7.6 | 7.1 | 0.560 |
| walker |  | 9112 | 48 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 8, sub: 0, line: 36 } |  |  | 0.560 |
| walker |  | 9160 | 48 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 10, sub: 0, line: 45 } |  |  | 0.560 |
| walker |  | 9208 | 48 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 14, sub: 0, line: 60 } |  |  | 0.560 |
| ns | 9217 |  | 131 | Toolchain pin and the no_std check-crate | 7.7 |  | 0.554 |
| walker |  | 9256 | 48 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 16, sub: 0, line: 67 } |  |  | 0.554 |
| walker |  | 9306 | 50 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 2, sub: 0, line: 13 } |  |  | 0.554 |
| walker |  | 9356 | 50 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 4, sub: 0, line: 22 } |  |  | 0.554 |
| ns | 9378 |  | 161 | CI job roster and toolchain matrix | 7.8 |  | 0.548 |
| walker |  | 9426 | 70 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 18, sub: 0, line: 74 } |  |  | 0.548 |
| walker |  | 9438 | 12 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 20, sub: 0, line: 80 } |  |  | 0.548 |
| ns | 9570 |  | 192 | CI commands — the canonical build/test/lint invocations | 7.9 | 7.8 | 0.544 |
| walker |  | 9608 | 170 | Code::CodeKey { rung: Names, file: src/error.rs, decl: 0, sub: 2, line: 0 } |  |  | 0.551 |
| walker |  | 9629 | 21 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 43, sub: 0, line: 876 } |  |  | 0.551 |
| walker |  | 9650 | 21 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 44, sub: 0, line: 892 } |  |  | 0.551 |
| ns | 9658 |  | 88 | trybuild UI harness — tests/compiletest.rs in full | 8.1 |  | 0.548 |
| walker |  | 9683 | 33 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 48, sub: 0, line: 951 } |  |  | 0.550 |
| ns | 9740 |  | 82 | tests/ui file listing | 8.2 | 8.1 | 0.557 |
| walker |  | 9758 | 75 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 45, sub: 0, line: 915 } |  |  | 0.557 |
| walker |  | 9839 | 81 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 54, sub: 0, line: 1015 } |  |  | 0.557 |
| ns | 9849 |  | 109 | One complete compile-fail pair — wrong-interpolation | 8.3 | 8.2 | 0.553 |
| walker |  | 9852 | 13 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 56, sub: 0, line: 1023 } |  |  | 0.553 |
| walker |  | 9935 | 83 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 46, sub: 0, line: 933 } |  |  | 0.560 |
| walker |  | 9971 | 36 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 49, sub: 0, line: 966 } |  |  | 0.561 |
| ns | 9990 |  | 141 | Shared test helpers in tests/common and tests/drop | 8.4 | 4.4 | 0.555 |
