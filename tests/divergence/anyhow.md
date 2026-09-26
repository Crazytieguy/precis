Score(3000)=0.636 I=0.831 C=0.486 ns_rows≤3K=20/63 grid(1000/1442/2080/3000/4327/6240/9000)=0.897/0.784/0.679/0.636/0.538/0.520/0.567

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 34 | 34 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 83 | 49 | Fs::DirListing { dir: src } |  |  | 0.000 |
| ns | 85 |  | 85 | Crate identity — name, version, description | 1.1 |  | 0.000 |
| ns | 119 |  | 34 | Repository root listing | 1.2 |  | 0.523 |
| walker |  | 150 | 67 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.527 |
| walker |  | 158 | 8 | Fs::DirListing { dir: .github } |  |  | 0.527 |
| walker |  | 162 | 4 | Fs::DirListing { dir: .github/workflows } |  |  | 0.527 |
| ns | 168 |  | 49 | src/ module inventory | 1.3 |  | 0.548 |
| walker |  | 189 | 27 | Toml::Operational { file: Cargo.toml } |  |  | 0.549 |
| walker |  | 233 | 44 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.558 |
| ns | 259 |  | 91 | What `anyhow::Error` is — a Box<dyn Error> that must be Send + Sync | 1.4 |  | 0.494 |
| walker |  | 268 | 35 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.504 |
| ns | 333 |  | 74 | …and the other two guarantees: always a backtrace, one word wide | 1.5 | 1.4 | 0.463 |
| ns | 417 |  | 84 | Cargo.toml package tail — MSRV, edition, license, links | 1.6 |  | 0.424 |
| walker |  | 433 | 165 | Toml::Identity { file: Cargo.toml } |  |  | 0.850 |
| ns | 507 |  | 90 | tests/ inventory | 1.7 |  | 0.693 |
| walker |  | 543 | 110 | Toml::Dependencies { file: Cargo.toml } |  |  | 0.703 |
| walker |  | 633 | 90 | Fs::DirListing { dir: tests } |  |  | 0.903 |
| walker |  | 637 | 4 | Fs::DirListing { dir: tests/common } |  |  | 0.903 |
| walker |  | 641 | 4 | Fs::DirListing { dir: tests/drop } |  |  | 0.903 |
| ns | 645 |  | 138 | Cargo features — std default, optional backtrace | 1.8 |  | 0.894 |
| walker |  | 655 | 14 | Fs::DirListing { dir: tests/crate } |  |  | 0.894 |
| walker |  | 781 | 126 | Code::CodeKey { rung: Names, file: src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.897 |
| ns | 793 |  | 148 | README lede, install snippet, and section map | 1.9 |  | 0.887 |
| walker |  | 799 | 18 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 9, sub: 0, line: 648 } |  |  | 0.888 |
| walker |  | 825 | 26 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 3, sub: 0, line: 389 } |  |  | 0.890 |
| walker |  | 870 | 45 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 1, sub: 0, line: 278 } |  |  | 0.890 |
| walker |  | 919 | 49 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 4, sub: 0, line: 413 } |  |  | 0.895 |
| walker |  | 975 | 56 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 6, sub: 0, line: 616 } |  |  | 0.896 |
| walker |  | 998 | 23 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 7, sub: 0, line: 618 } |  |  | 0.897 |
| ns | 1009 |  | 216 | lib.rs crate attributes and module declarations | 1.10 |  | 0.777 |
| walker |  | 1032 | 34 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 8, sub: 0, line: 624 } |  |  | 0.779 |
| walker |  | 1044 | 12 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 7, sub: 0, line: 618 } |  |  | 0.780 |
| walker |  | 1051 | 7 | Code::CodeKey { rung: Body, file: src/lib.rs, decl: 2, sub: 0, line: 280 } |  |  | 0.780 |
| walker |  | 1120 | 69 | Code::CodeKey { rung: Names, file: build.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.780 |
| walker |  | 1211 | 91 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.780 |
| ns | 1244 |  | 235 | Complete roster of exported items in lib.rs | 2.1 |  | 0.772 |
| walker |  | 1300 | 89 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.772 |
| walker |  | 1310 | 10 | Code::CodeKey { rung: Body, file: src/lib.rs, decl: 9, sub: 0, line: 648 } |  |  | 0.780 |
| walker |  | 1340 | 30 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 8, sub: 0, line: 624 } |  |  | 0.782 |
| walker |  | 1422 | 82 | Fs::DirListing { dir: tests/ui } |  |  | 0.784 |
| ns | 1452 |  | 208 | impl Error — roster of every public method | 2.2 |  | 0.731 |
| walker |  | 1522 | 100 | Code::CodeKey { rung: Names, file: src/ensure.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.731 |
| walker |  | 1548 | 26 | Code::CodeKey { rung: Decl, file: src/ensure.rs, decl: 3, sub: 0, line: 29 } |  |  | 0.731 |
| walker |  | 1575 | 27 | Code::CodeKey { rung: Decl, file: src/ensure.rs, decl: 9, sub: 0, line: 58 } |  |  | 0.731 |
| walker |  | 1604 | 29 | Code::CodeKey { rung: Decl, file: src/ensure.rs, decl: 5, sub: 0, line: 35 } |  |  | 0.731 |
| ns | 1612 |  | 160 | Context trait — both method signatures with bounds | 2.3 | 2.1 | 0.740 |
| walker |  | 1637 | 33 | Code::CodeKey { rung: Decl, file: src/ensure.rs, decl: 6, sub: 0, line: 40 } |  |  | 0.740 |
| walker |  | 1694 | 57 | Code::CodeKey { rung: Decl, file: src/ensure.rs, decl: 1, sub: 0, line: 14 } |  |  | 0.741 |
| walker |  | 1704 | 10 | Code::CodeKey { rung: Body, file: src/ensure.rs, decl: 4, sub: 0, line: 30 } |  |  | 0.741 |
| walker |  | 1722 | 18 | Code::CodeKey { rung: Body, file: src/ensure.rs, decl: 2, sub: 0, line: 19 } |  |  | 0.741 |
| ns | 1778 |  | 166 | Display representations — `{}` and `{:#}`, with sample output | 2.4 | 1.4 | 0.720 |
| ns | 1915 |  | 137 | Debug representations — `{:?}` and `{:#?}` | 2.5 | 2.4 | 0.695 |
| ns | 2031 |  | 116 | Context trait doc — sealed, and outermost-first cause printing | 2.6 | 2.3 | 0.675 |
| walker |  | 2050 | 328 | Code::CodeKey { rung: Names, file: src/error.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.679 |
| walker |  | 2067 | 17 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 31, sub: 0, line: 731 } |  |  | 0.679 |
| walker |  | 2095 | 28 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 27, sub: 0, line: 719 } |  |  | 0.679 |
| walker |  | 2123 | 28 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 29, sub: 0, line: 725 } |  |  | 0.679 |
| walker |  | 2156 | 33 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 35, sub: 0, line: 951 } |  |  | 0.679 |
| walker |  | 2201 | 45 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 25, sub: 0, line: 712 } |  |  | 0.679 |
| ns | 2212 |  | 181 | Context + downcasting — the guarantee, in both directions | 2.7 | 2.6 | 0.650 |
| walker |  | 2247 | 46 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 48, sub: 0, line: 1047 } |  |  | 0.650 |
| walker |  | 2254 | 7 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 49, sub: 0, line: 1049 } |  |  | 0.650 |
| walker |  | 2298 | 44 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 50, sub: 0, line: 1055 } |  |  | 0.650 |
| walker |  | 2305 | 7 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 51, sub: 0, line: 1057 } |  |  | 0.650 |
| walker |  | 2349 | 44 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 52, sub: 0, line: 1063 } |  |  | 0.650 |
| walker |  | 2356 | 7 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 53, sub: 0, line: 1065 } |  |  | 0.650 |
| ns | 2360 |  | 148 | Result alias and Ok() helper semantics | 2.8 | 2.1 | 0.632 |
| walker |  | 2405 | 49 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 56, sub: 0, line: 1078 } |  |  | 0.632 |
| walker |  | 2456 | 51 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 54, sub: 0, line: 1071 } |  |  | 0.632 |
| walker |  | 2509 | 53 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 44, sub: 0, line: 1029 } |  |  | 0.632 |
| ns | 2536 |  | 176 | no_std support contract | 2.9 |  | 0.615 |
| walker |  | 2562 | 53 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 46, sub: 0, line: 1038 } |  |  | 0.615 |
| walker |  | 2628 | 66 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 23, sub: 0, line: 703 } |  |  | 0.615 |
| walker |  | 2697 | 69 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 21, sub: 0, line: 691 } |  |  | 0.615 |
| walker |  | 2703 | 6 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 22, sub: 0, line: 696 } |  |  | 0.615 |
| ns | 2768 |  | 232 | Trait impls on Error — complete list | 2.10 | 2.2 | 0.634 |
| walker |  | 2782 | 79 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 41, sub: 0, line: 1015 } |  |  | 0.634 |
| walker |  | 2795 | 13 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 43, sub: 0, line: 1023 } |  |  | 0.634 |
| walker |  | 2878 | 83 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 34, sub: 0, line: 933 } |  |  | 0.635 |
| walker |  | 2995 | 117 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 36, sub: 0, line: 966 } |  |  | 0.636 |
| ns | 3002 |  | 234 | What bail! and ensure! mean | 3.1 |  | 0.616 |
| walker |  | 3004 | 9 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 40, sub: 0, line: 1009 } |  |  | 0.616 |
| walker |  | 3023 | 19 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 39, sub: 0, line: 984 } |  |  | 0.616 |
| walker |  | 3044 | 21 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 38, sub: 0, line: 973 } |  |  | 0.616 |
| ns | 3183 |  | 181 | What anyhow! constructs | 3.2 | 3.1 | 0.600 |
| walker |  | 3214 | 170 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 1, sub: 0, line: 19 } |  |  | 0.603 |
| walker |  | 3251 | 37 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 7, sub: 0, line: 197 } |  |  | 0.603 |
| walker |  | 3289 | 38 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 4, sub: 0, line: 137 } |  |  | 0.603 |
| walker |  | 3328 | 39 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 6, sub: 0, line: 169 } |  |  | 0.603 |
| walker |  | 3376 | 48 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 3, sub: 0, line: 75 } |  |  | 0.603 |
| walker |  | 3435 | 59 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 5, sub: 0, line: 145 } |  |  | 0.603 |
| ns | 3438 |  | 255 | Every exported macro_rules!, with its export attributes | 3.3 | 3.1 | 0.575 |
| walker |  | 3503 | 68 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 2, sub: 0, line: 27 } |  |  | 0.575 |
| ns | 3660 |  | 222 | anyhow! expansion — the three arms | 3.4 | 3.3 | 0.554 |
| walker |  | 3774 | 271 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 33, sub: 0, line: 740 } |  |  | 0.555 |
| ns | 3796 |  | 136 | bail! body | 3.5 | 3.3 | 0.545 |
| walker |  | 3952 | 178 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 1, sub: 1, line: 19 } |  |  | 0.561 |
| ns | 3957 |  | 161 | ensure! expansion — the fuel-limited parser call | 3.6 | 3.3 | 0.547 |
| walker |  | 3971 | 19 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 11, sub: 0, line: 431 } |  |  | 0.547 |
| walker |  | 4000 | 29 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 12, sub: 0, line: 457 } |  |  | 0.547 |
| walker |  | 4031 | 31 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 14, sub: 0, line: 482 } |  |  | 0.547 |
| walker |  | 4062 | 31 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 15, sub: 0, line: 490 } |  |  | 0.547 |
| walker |  | 4093 | 31 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 16, sub: 0, line: 554 } |  |  | 0.547 |
| ns | 4126 |  | 169 | kind.rs — why autoref tagged dispatch | 3.7 | 3.4 | 0.538 |
| walker |  | 4130 | 37 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 13, sub: 0, line: 468 } |  |  | 0.538 |
| walker |  | 4176 | 46 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 10, sub: 0, line: 370 } |  |  | 0.538 |
| walker |  | 4244 | 68 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 9, sub: 0, line: 258 } |  |  | 0.538 |
| walker |  | 4359 | 115 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 8, sub: 0, line: 225 } |  |  | 0.538 |
| ns | 4375 |  | 249 | kind.rs — the three kinds and their new() constructors | 3.8 | 3.7 | 0.524 |
| walker |  | 4461 | 102 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 1, sub: 2, line: 19 } |  |  | 0.540 |
| walker |  | 4473 | 12 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 20, sub: 0, line: 674 } |  |  | 0.540 |
| walker |  | 4504 | 31 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 17, sub: 0, line: 568 } |  |  | 0.540 |
| walker |  | 4580 | 76 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 18, sub: 0, line: 598 } |  |  | 0.540 |
| ns | 4627 |  | 252 | ensure.rs runtime side — BothDebug / NotBothDebug dispatch | 3.9 | 3.6 | 0.539 |
| walker |  | 4684 | 104 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 19, sub: 0, line: 640 } |  |  | 0.539 |
| walker |  | 4699 | 15 | Code::CodeKey { rung: Doc, file: src/error.rs, decl: 17, sub: 0, line: 568 } |  |  | 0.539 |
| walker |  | 4742 | 43 | Code::CodeKey { rung: Names, file: src/backtrace.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.539 |
| walker |  | 4761 | 19 | Code::CodeKey { rung: Decl, file: src/backtrace.rs, decl: 1, sub: 0, line: 7 } |  |  | 0.539 |
| walker |  | 4785 | 24 | Code::CodeKey { rung: Names, file: src/macros.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.539 |
| ns | 4798 |  | 171 | __parse_ensure! extent, first rule, and catch-all | 3.10 | 3.6 | 0.529 |
| walker |  | 4953 | 168 | Code::CodeKey { rung: Decl, file: src/macros.rs, decl: 1, sub: 0, line: 56 } |  |  | 0.552 |
| ns | 4995 |  | 197 | lib.rs __private — the macro-facing surface | 3.11 | 3.3 | 0.541 |
| ns | 5156 |  | 161 | test_fmt.rs — the f/g/h fixture and expected `{:#}` strings | 4.1 | 2.4 | 0.533 |
| walker |  | 5205 | 252 | Code::CodeKey { rung: Decl, file: src/macros.rs, decl: 2, sub: 0, line: 202 } |  |  | 0.574 |
| walker |  | 5223 | 18 | Code::CodeKey { rung: Doc, file: src/error.rs, decl: 15, sub: 0, line: 490 } |  |  | 0.574 |
| ns | 5295 |  | 139 | test_fmt.rs — expected `{:?}` strings, including numbered causes | 4.2 | 4.1 | 0.561 |
| ns | 5454 |  | 159 | test_repr.rs — size, null-pointer-optimization, autotrait assertions | 4.3 | 1.5 | 0.550 |
| walker |  | 5474 | 251 | Code::CodeKey { rung: Names, file: src/context.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.551 |
| walker |  | 5501 | 27 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 20, sub: 0, line: 180 } |  |  | 0.551 |
| walker |  | 5549 | 48 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 9, sub: 0, line: 128 } |  |  | 0.551 |
| walker |  | 5597 | 48 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 18, sub: 0, line: 168 } |  |  | 0.551 |
| ns | 5633 |  | 179 | test_downcast.rs — what a bail!'d error downcasts to | 4.4 | 3.4 | 0.537 |
| walker |  | 5649 | 52 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 4, sub: 0, line: 90 } |  |  | 0.537 |
| walker |  | 5678 | 29 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 5, sub: 0, line: 91 } |  |  | 0.537 |
| walker |  | 5720 | 42 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 6, sub: 0, line: 103 } |  |  | 0.537 |
| walker |  | 5777 | 57 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 7, sub: 0, line: 115 } |  |  | 0.537 |
| ns | 5807 |  | 174 | test_chain.rs — cause iteration order | 4.5 |  | 0.529 |
| walker |  | 5849 | 72 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 14, sub: 0, line: 152 } |  |  | 0.529 |
| walker |  | 5861 | 12 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 16, sub: 0, line: 160 } |  |  | 0.529 |
| walker |  | 5945 | 84 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 1, sub: 0, line: 42 } |  |  | 0.529 |
| walker |  | 5974 | 29 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 2, sub: 0, line: 46 } |  |  | 0.529 |
| ns | 6002 |  | 195 | test_context.rs — downcast_ref through a three-level context chain | 4.6 | 2.7 | 0.520 |
| walker |  | 6016 | 42 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 3, sub: 0, line: 58 } |  |  | 0.520 |
| walker |  | 6101 | 85 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 11, sub: 0, line: 137 } |  |  | 0.520 |
| walker |  | 6113 | 12 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 13, sub: 0, line: 146 } |  |  | 0.520 |
| walker |  | 6157 | 44 | Code::CodeKey { rung: Names, file: src/fmt.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.520 |
| walker |  | 6192 | 35 | Code::CodeKey { rung: Decl, file: src/fmt.rs, decl: 4, sub: 0, line: 69 } |  |  | 0.520 |
| walker |  | 6241 | 49 | Code::CodeKey { rung: Decl, file: src/fmt.rs, decl: 5, sub: 0, line: 75 } |  |  | 0.520 |
| ns | 6252 |  | 250 | ErrorVTable — the hand-rolled vtable layout | 5.1 |  | 0.532 |
| walker |  | 6306 | 65 | Code::CodeKey { rung: Decl, file: src/fmt.rs, decl: 1, sub: 0, line: 6 } |  |  | 0.533 |
| ns | 6483 |  | 231 | ErrorImpl, ContextError, and the vtable reader | 5.2 | 5.1 | 0.533 |
| ns | 6615 |  | 132 | Error::construct — allocation and type erasure | 5.3 | 5.2 | 0.525 |
| walker |  | 6622 | 316 | Code::CodeKey { rung: Names, file: src/ptr.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.525 |
| walker |  | 6633 | 11 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 35, sub: 0, line: 181 } |  |  | 0.526 |
| walker |  | 6646 | 13 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 36, sub: 0, line: 185 } |  |  | 0.526 |
| walker |  | 6667 | 21 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 33, sub: 0, line: 174 } |  |  | 0.526 |
| walker |  | 6706 | 39 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 5, sub: 0, line: 19 } |  |  | 0.526 |
| ns | 6718 |  | 103 | impl Error — roster of the non-public fns | 5.4 | 5.3 | 0.531 |
| walker |  | 6745 | 39 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 15, sub: 0, line: 74 } |  |  | 0.531 |
| walker |  | 6784 | 39 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 26, sub: 0, line: 135 } |  |  | 0.531 |
| walker |  | 6829 | 45 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 1, sub: 0, line: 5 } |  |  | 0.531 |
| walker |  | 6888 | 59 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 13, sub: 0, line: 63 } |  |  | 0.532 |
| ns | 6889 |  | 171 | Vtable function roster in error.rs | 5.5 | 5.1 | 0.525 |
| walker |  | 6948 | 60 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 24, sub: 0, line: 124 } |  |  | 0.526 |
| ns | 7040 |  | 151 | ErrorImpl methods and trait impls | 5.6 | 5.2 | 0.528 |
| walker |  | 7064 | 116 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 28, sub: 0, line: 144 } |  |  | 0.528 |
| walker |  | 7187 | 123 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 7, sub: 0, line: 28 } |  |  | 0.528 |
| ns | 7275 |  | 235 | ptr.rs — Own / Ref / Mut / CastTo declarations | 5.7 |  | 0.547 |
| walker |  | 7335 | 148 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 17, sub: 0, line: 83 } |  |  | 0.547 |
| ns | 7440 |  | 165 | chain.rs — Chain and ChainState | 6.1 |  | 0.540 |
| walker |  | 7479 | 144 | Code::CodeKey { rung: Names, file: src/kind.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.544 |
| walker |  | 7499 | 20 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 9, sub: 0, line: 99 } |  |  | 0.545 |
| walker |  | 7519 | 20 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 10, sub: 0, line: 111 } |  |  | 0.545 |
| walker |  | 7545 | 26 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 3, sub: 0, line: 67 } |  |  | 0.546 |
| ns | 7557 |  | 117 | chain.rs — every trait impl on Chain | 6.2 | 6.1 | 0.542 |
| walker |  | 7571 | 26 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 7, sub: 0, line: 89 } |  |  | 0.543 |
| walker |  | 7601 | 30 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 8, sub: 0, line: 90 } |  |  | 0.545 |
| walker |  | 7638 | 37 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 4, sub: 0, line: 68 } |  |  | 0.547 |
| walker |  | 7688 | 50 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 11, sub: 0, line: 114 } |  |  | 0.549 |
| walker |  | 7694 | 6 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 12, sub: 0, line: 116 } |  |  | 0.549 |
| ns | 7782 |  | 225 | wrapper.rs — the three error adapters and their impls | 6.3 |  | 0.541 |
| walker |  | 7805 | 111 | Code::CodeKey { rung: Names, file: src/chain.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.544 |
| walker |  | 7825 | 20 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 10, sub: 0, line: 76 } |  |  | 0.545 |
| walker |  | 7855 | 30 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 3, sub: 0, line: 26 } |  |  | 0.546 |
| walker |  | 7861 | 6 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 4, sub: 0, line: 27 } |  |  | 0.547 |
| walker |  | 7895 | 34 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 12, sub: 0, line: 93 } |  |  | 0.547 |
| walker |  | 7930 | 35 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.547 |
| ns | 7959 |  | 177 | context.rs — where .context() is implemented | 6.4 | 2.3 | 0.543 |
| walker |  | 7974 | 44 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 8, sub: 0, line: 56 } |  |  | 0.545 |
| walker |  | 8042 | 68 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 5, sub: 0, line: 35 } |  |  | 0.548 |
| ns | 8133 |  | 174 | context.rs — ContextError impls, Quoted, and the Sealed trait | 6.5 | 6.4 | 0.554 |
| walker |  | 8141 | 99 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 2, sub: 0, line: 15 } |  |  | 0.568 |
| ns | 8261 |  | 128 | fmt.rs — the rendering entry points | 6.6 | 5.6 | 0.571 |
| walker |  | 8351 | 210 | Code::CodeKey { rung: Names, file: src/wrapper.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.583 |
| walker |  | 8358 | 7 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 7, sub: 0, line: 33 } |  |  | 0.585 |
| walker |  | 8367 | 9 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.586 |
| walker |  | 8394 | 27 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 13, sub: 0, line: 56 } |  |  | 0.588 |
| ns | 8413 |  | 152 | build.rs — the complete custom-cfg vocabulary | 7.1 |  | 0.584 |
| walker |  | 8442 | 48 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 8, sub: 0, line: 36 } |  |  | 0.584 |
| walker |  | 8490 | 48 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 10, sub: 0, line: 45 } |  |  | 0.584 |
| ns | 8527 |  | 114 | build.rs — which compiler turns each cfg on | 7.2 | 7.1 | 0.581 |
| walker |  | 8538 | 48 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 14, sub: 0, line: 60 } |  |  | 0.581 |
| walker |  | 8586 | 48 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 16, sub: 0, line: 67 } |  |  | 0.581 |
| walker |  | 8636 | 50 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 2, sub: 0, line: 13 } |  |  | 0.581 |
| ns | 8637 |  | 110 | backtrace.rs — the three definitions of Backtrace | 7.3 |  | 0.577 |
| walker |  | 8686 | 50 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 4, sub: 0, line: 22 } |  |  | 0.577 |
| walker |  | 8756 | 70 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 18, sub: 0, line: 74 } |  |  | 0.577 |
| walker |  | 8768 | 12 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 20, sub: 0, line: 80 } |  |  | 0.577 |
| walker |  | 8776 | 8 | Code::CodeKey { rung: Body, file: src/kind.rs, decl: 8, sub: 0, line: 90 } |  |  | 0.577 |
| ns | 8813 |  | 176 | backtrace.rs — impl_backtrace! and backtrace!(), both arms each | 7.4 | 7.3 | 0.571 |
| ns | 8954 |  | 141 | backtrace.rs — backtrace_if_absent! and the vendored capture module | 7.5 | 7.3 | 0.567 |
| walker |  | 9000 | 224 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.567 |
| ns | 9086 |  | 132 | nightly.rs — the build probe and generic-member-access shims | 7.6 | 7.1 | 0.563 |
| walker |  | 9208 | 208 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.563 |
| ns | 9217 |  | 131 | Toolchain pin and the no_std check-crate | 7.7 |  | 0.557 |
| ns | 9378 |  | 161 | CI job roster and toolchain matrix | 7.8 |  | 0.551 |
| walker |  | 9474 | 266 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.551 |
| ns | 9570 |  | 192 | CI commands — the canonical build/test/lint invocations | 7.9 | 7.8 | 0.547 |
| walker |  | 9616 | 142 | Code::CodeKey { rung: Names, file: src/nightly.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.548 |
| walker |  | 9627 | 11 | Code::CodeKey { rung: Decl, file: src/nightly.rs, decl: 2, sub: 0, line: 38 } |  |  | 0.548 |
| ns | 9658 |  | 88 | trybuild UI harness — tests/compiletest.rs in full | 8.1 |  | 0.545 |
| ns | 9740 |  | 82 | tests/ui file listing | 8.2 | 8.1 | 0.552 |
| ns | 9849 |  | 109 | One complete compile-fail pair — wrong-interpolation | 8.3 | 8.2 | 0.548 |
| walker |  | 9863 | 236 | Code::CodeKey { rung: Decl, file: src/nightly.rs, decl: 1, sub: 0, line: 12 } |  |  | 0.548 |
| walker |  | 9875 | 12 | Code::CodeKey { rung: Body, file: src/nightly.rs, decl: 5, sub: 0, line: 56 } |  |  | 0.548 |
| walker |  | 9940 | 65 | Code::CodeKey { rung: ModuleDoc, file: src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.548 |
| ns | 9990 |  | 141 | Shared test helpers in tests/common and tests/drop | 8.4 | 4.4 | 0.542 |
