Score(3000)=0.652 I=0.837 C=0.507 ns_rows≤3K=20/63 grid(1000/1442/2080/3000/4327/6240/9000)=0.752/0.799/0.696/0.652/0.552/0.530/0.574

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
| walker |  | 268 | 35 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.504 |
| ns | 333 |  | 74 | …and the other two guarantees: always a backtrace, one word wide | 1.5 | 1.4 | 0.463 |
| ns | 417 |  | 84 | Cargo.toml package tail — MSRV, edition, license, links | 1.6 |  | 0.424 |
| walker |  | 433 | 165 | Toml::Identity { file: Cargo.toml } |  |  | 0.850 |
| ns | 507 |  | 90 | tests/ inventory | 1.7 |  | 0.693 |
| walker |  | 543 | 110 | Toml::Dependencies { file: Cargo.toml } |  |  | 0.703 |
| ns | 645 |  | 138 | Cargo features — std default, optional backtrace | 1.8 |  | 0.723 |
| walker |  | 768 | 225 | Code::CodeKey { rung: Names, file: src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.730 |
| walker |  | 786 | 18 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 9, sub: 0, line: 648 } |  |  | 0.731 |
| ns | 793 |  | 148 | README lede, install snippet, and section map | 1.9 |  | 0.744 |
| walker |  | 812 | 26 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 3, sub: 0, line: 389 } |  |  | 0.746 |
| walker |  | 857 | 45 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 1, sub: 0, line: 278 } |  |  | 0.746 |
| walker |  | 906 | 49 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 4, sub: 0, line: 413 } |  |  | 0.751 |
| walker |  | 962 | 56 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 6, sub: 0, line: 616 } |  |  | 0.751 |
| walker |  | 985 | 23 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 7, sub: 0, line: 618 } |  |  | 0.752 |
| ns | 1009 |  | 216 | lib.rs crate attributes and module declarations | 1.10 |  | 0.681 |
| walker |  | 1019 | 34 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 8, sub: 0, line: 624 } |  |  | 0.682 |
| walker |  | 1109 | 90 | Fs::DirListing { dir: tests } |  |  | 0.808 |
| walker |  | 1113 | 4 | Fs::DirListing { dir: tests/common } |  |  | 0.808 |
| walker |  | 1117 | 4 | Fs::DirListing { dir: tests/drop } |  |  | 0.808 |
| walker |  | 1131 | 14 | Fs::DirListing { dir: tests/crate } |  |  | 0.808 |
| walker |  | 1143 | 12 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 7, sub: 0, line: 618 } |  |  | 0.809 |
| walker |  | 1212 | 69 | Code::CodeKey { rung: Names, file: build.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.809 |
| ns | 1244 |  | 235 | Complete roster of exported items in lib.rs | 2.1 |  | 0.797 |
| walker |  | 1303 | 91 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.797 |
| walker |  | 1392 | 89 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.797 |
| walker |  | 1399 | 7 | Code::CodeKey { rung: Body, file: src/lib.rs, decl: 2, sub: 0, line: 280 } |  |  | 0.797 |
| walker |  | 1429 | 30 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 8, sub: 0, line: 624 } |  |  | 0.799 |
| ns | 1452 |  | 208 | impl Error — roster of every public method | 2.2 |  | 0.745 |
| walker |  | 1511 | 82 | Fs::DirListing { dir: tests/ui } |  |  | 0.747 |
| walker |  | 1521 | 10 | Code::CodeKey { rung: Body, file: src/lib.rs, decl: 9, sub: 0, line: 648 } |  |  | 0.755 |
| ns | 1612 |  | 160 | Context trait — both method signatures with bounds | 2.3 | 2.1 | 0.762 |
| walker |  | 1621 | 100 | Code::CodeKey { rung: Names, file: src/ensure.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.762 |
| walker |  | 1647 | 26 | Code::CodeKey { rung: Decl, file: src/ensure.rs, decl: 3, sub: 0, line: 29 } |  |  | 0.762 |
| walker |  | 1674 | 27 | Code::CodeKey { rung: Decl, file: src/ensure.rs, decl: 9, sub: 0, line: 58 } |  |  | 0.762 |
| walker |  | 1703 | 29 | Code::CodeKey { rung: Decl, file: src/ensure.rs, decl: 5, sub: 0, line: 35 } |  |  | 0.762 |
| walker |  | 1736 | 33 | Code::CodeKey { rung: Decl, file: src/ensure.rs, decl: 6, sub: 0, line: 40 } |  |  | 0.762 |
| ns | 1778 |  | 166 | Display representations — `{}` and `{:#}`, with sample output | 2.4 | 1.4 | 0.740 |
| walker |  | 1793 | 57 | Code::CodeKey { rung: Decl, file: src/ensure.rs, decl: 1, sub: 0, line: 14 } |  |  | 0.740 |
| walker |  | 1803 | 10 | Code::CodeKey { rung: Body, file: src/ensure.rs, decl: 4, sub: 0, line: 30 } |  |  | 0.741 |
| walker |  | 1821 | 18 | Code::CodeKey { rung: Body, file: src/ensure.rs, decl: 2, sub: 0, line: 19 } |  |  | 0.741 |
| ns | 1915 |  | 137 | Debug representations — `{:?}` and `{:#?}` | 2.5 | 2.4 | 0.716 |
| ns | 2031 |  | 116 | Context trait doc — sealed, and outermost-first cause printing | 2.6 | 2.3 | 0.695 |
| walker |  | 2149 | 328 | Code::CodeKey { rung: Names, file: src/error.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.699 |
| walker |  | 2166 | 17 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 31, sub: 0, line: 731 } |  |  | 0.699 |
| walker |  | 2194 | 28 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 27, sub: 0, line: 719 } |  |  | 0.699 |
| ns | 2212 |  | 181 | Context + downcasting — the guarantee, in both directions | 2.7 | 2.6 | 0.669 |
| walker |  | 2222 | 28 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 29, sub: 0, line: 725 } |  |  | 0.669 |
| walker |  | 2255 | 33 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 35, sub: 0, line: 951 } |  |  | 0.669 |
| walker |  | 2300 | 45 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 25, sub: 0, line: 712 } |  |  | 0.669 |
| walker |  | 2346 | 46 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 48, sub: 0, line: 1047 } |  |  | 0.669 |
| walker |  | 2353 | 7 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 49, sub: 0, line: 1049 } |  |  | 0.669 |
| ns | 2360 |  | 148 | Result alias and Ok() helper semantics | 2.8 | 2.1 | 0.650 |
| walker |  | 2397 | 44 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 50, sub: 0, line: 1055 } |  |  | 0.650 |
| walker |  | 2404 | 7 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 51, sub: 0, line: 1057 } |  |  | 0.650 |
| walker |  | 2448 | 44 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 52, sub: 0, line: 1063 } |  |  | 0.650 |
| walker |  | 2455 | 7 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 53, sub: 0, line: 1065 } |  |  | 0.650 |
| walker |  | 2504 | 49 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 56, sub: 0, line: 1078 } |  |  | 0.650 |
| ns | 2536 |  | 176 | no_std support contract | 2.9 |  | 0.633 |
| walker |  | 2555 | 51 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 54, sub: 0, line: 1071 } |  |  | 0.633 |
| walker |  | 2608 | 53 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 44, sub: 0, line: 1029 } |  |  | 0.633 |
| walker |  | 2661 | 53 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 46, sub: 0, line: 1038 } |  |  | 0.633 |
| walker |  | 2727 | 66 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 23, sub: 0, line: 703 } |  |  | 0.633 |
| ns | 2768 |  | 232 | Trait impls on Error — complete list | 2.10 | 2.2 | 0.651 |
| walker |  | 2796 | 69 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 21, sub: 0, line: 691 } |  |  | 0.651 |
| walker |  | 2802 | 6 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 22, sub: 0, line: 696 } |  |  | 0.651 |
| walker |  | 2881 | 79 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 41, sub: 0, line: 1015 } |  |  | 0.651 |
| walker |  | 2894 | 13 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 43, sub: 0, line: 1023 } |  |  | 0.651 |
| walker |  | 2977 | 83 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 34, sub: 0, line: 933 } |  |  | 0.652 |
| ns | 3002 |  | 234 | What bail! and ensure! mean | 3.1 |  | 0.631 |
| walker |  | 3094 | 117 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 36, sub: 0, line: 966 } |  |  | 0.632 |
| walker |  | 3103 | 9 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 40, sub: 0, line: 1009 } |  |  | 0.632 |
| walker |  | 3122 | 19 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 39, sub: 0, line: 984 } |  |  | 0.632 |
| walker |  | 3143 | 21 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 38, sub: 0, line: 973 } |  |  | 0.632 |
| ns | 3183 |  | 181 | What anyhow! constructs | 3.2 | 3.1 | 0.616 |
| walker |  | 3308 | 165 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 1, sub: 0, line: 19 } |  |  | 0.618 |
| walker |  | 3345 | 37 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 7, sub: 0, line: 197 } |  |  | 0.618 |
| walker |  | 3383 | 38 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 4, sub: 0, line: 137 } |  |  | 0.618 |
| walker |  | 3422 | 39 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 6, sub: 0, line: 169 } |  |  | 0.618 |
| ns | 3438 |  | 255 | Every exported macro_rules!, with its export attributes | 3.3 | 3.1 | 0.590 |
| walker |  | 3470 | 48 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 3, sub: 0, line: 75 } |  |  | 0.590 |
| walker |  | 3529 | 59 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 5, sub: 0, line: 145 } |  |  | 0.590 |
| walker |  | 3597 | 68 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 2, sub: 0, line: 27 } |  |  | 0.590 |
| ns | 3660 |  | 222 | anyhow! expansion — the three arms | 3.4 | 3.3 | 0.568 |
| ns | 3796 |  | 136 | bail! body | 3.5 | 3.3 | 0.557 |
| walker |  | 3868 | 271 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 33, sub: 0, line: 740 } |  |  | 0.559 |
| ns | 3957 |  | 161 | ensure! expansion — the fuel-limited parser call | 3.6 | 3.3 | 0.545 |
| walker |  | 4046 | 178 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 1, sub: 1, line: 19 } |  |  | 0.561 |
| walker |  | 4065 | 19 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 11, sub: 0, line: 431 } |  |  | 0.561 |
| walker |  | 4094 | 29 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 12, sub: 0, line: 457 } |  |  | 0.561 |
| walker |  | 4125 | 31 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 14, sub: 0, line: 482 } |  |  | 0.561 |
| ns | 4126 |  | 169 | kind.rs — why autoref tagged dispatch | 3.7 | 3.4 | 0.552 |
| walker |  | 4156 | 31 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 15, sub: 0, line: 490 } |  |  | 0.552 |
| walker |  | 4187 | 31 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 16, sub: 0, line: 554 } |  |  | 0.552 |
| walker |  | 4224 | 37 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 13, sub: 0, line: 468 } |  |  | 0.552 |
| walker |  | 4270 | 46 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 10, sub: 0, line: 370 } |  |  | 0.552 |
| walker |  | 4338 | 68 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 9, sub: 0, line: 258 } |  |  | 0.552 |
| ns | 4375 |  | 249 | kind.rs — the three kinds and their new() constructors | 3.8 | 3.7 | 0.537 |
| walker |  | 4453 | 115 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 8, sub: 0, line: 225 } |  |  | 0.537 |
| walker |  | 4560 | 107 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 1, sub: 2, line: 19 } |  |  | 0.553 |
| walker |  | 4572 | 12 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 20, sub: 0, line: 674 } |  |  | 0.553 |
| walker |  | 4603 | 31 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 17, sub: 0, line: 568 } |  |  | 0.553 |
| ns | 4627 |  | 252 | ensure.rs runtime side — BothDebug / NotBothDebug dispatch | 3.9 | 3.6 | 0.551 |
| walker |  | 4679 | 76 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 18, sub: 0, line: 598 } |  |  | 0.551 |
| walker |  | 4783 | 104 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 19, sub: 0, line: 640 } |  |  | 0.551 |
| walker |  | 4798 | 15 | Code::CodeKey { rung: Doc, file: src/error.rs, decl: 17, sub: 0, line: 568 } |  |  | 0.541 |
| ns | 4798 |  | 171 | __parse_ensure! extent, first rule, and catch-all | 3.10 | 3.6 | 0.541 |
| walker |  | 4841 | 43 | Code::CodeKey { rung: Names, file: src/backtrace.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.541 |
| walker |  | 4860 | 19 | Code::CodeKey { rung: Decl, file: src/backtrace.rs, decl: 1, sub: 0, line: 7 } |  |  | 0.541 |
| walker |  | 4884 | 24 | Code::CodeKey { rung: Names, file: src/macros.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.541 |
| ns | 4995 |  | 197 | lib.rs __private — the macro-facing surface | 3.11 | 3.3 | 0.530 |
| walker |  | 5052 | 168 | Code::CodeKey { rung: Decl, file: src/macros.rs, decl: 1, sub: 0, line: 56 } |  |  | 0.552 |
| ns | 5156 |  | 161 | test_fmt.rs — the f/g/h fixture and expected `{:#}` strings | 4.1 | 2.4 | 0.545 |
| ns | 5295 |  | 139 | test_fmt.rs — expected `{:?}` strings, including numbered causes | 4.2 | 4.1 | 0.533 |
| walker |  | 5304 | 252 | Code::CodeKey { rung: Decl, file: src/macros.rs, decl: 2, sub: 0, line: 202 } |  |  | 0.572 |
| walker |  | 5322 | 18 | Code::CodeKey { rung: Doc, file: src/error.rs, decl: 15, sub: 0, line: 490 } |  |  | 0.572 |
| ns | 5454 |  | 159 | test_repr.rs — size, null-pointer-optimization, autotrait assertions | 4.3 | 1.5 | 0.560 |
| walker |  | 5573 | 251 | Code::CodeKey { rung: Names, file: src/context.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.561 |
| walker |  | 5600 | 27 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 20, sub: 0, line: 180 } |  |  | 0.561 |
| ns | 5633 |  | 179 | test_downcast.rs — what a bail!'d error downcasts to | 4.4 | 3.4 | 0.547 |
| walker |  | 5648 | 48 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 9, sub: 0, line: 128 } |  |  | 0.547 |
| walker |  | 5696 | 48 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 18, sub: 0, line: 168 } |  |  | 0.547 |
| walker |  | 5748 | 52 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 4, sub: 0, line: 90 } |  |  | 0.547 |
| walker |  | 5777 | 29 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 5, sub: 0, line: 91 } |  |  | 0.547 |
| ns | 5807 |  | 174 | test_chain.rs — cause iteration order | 4.5 |  | 0.539 |
| walker |  | 5819 | 42 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 6, sub: 0, line: 103 } |  |  | 0.539 |
| walker |  | 5876 | 57 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 7, sub: 0, line: 115 } |  |  | 0.539 |
| walker |  | 5948 | 72 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 14, sub: 0, line: 152 } |  |  | 0.539 |
| walker |  | 5960 | 12 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 16, sub: 0, line: 160 } |  |  | 0.539 |
| ns | 6002 |  | 195 | test_context.rs — downcast_ref through a three-level context chain | 4.6 | 2.7 | 0.530 |
| walker |  | 6044 | 84 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 1, sub: 0, line: 42 } |  |  | 0.530 |
| walker |  | 6073 | 29 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 2, sub: 0, line: 46 } |  |  | 0.530 |
| walker |  | 6115 | 42 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 3, sub: 0, line: 58 } |  |  | 0.530 |
| walker |  | 6200 | 85 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 11, sub: 0, line: 137 } |  |  | 0.530 |
| walker |  | 6212 | 12 | Code::CodeKey { rung: Decl, file: src/context.rs, decl: 13, sub: 0, line: 146 } |  |  | 0.530 |
| ns | 6252 |  | 250 | ErrorVTable — the hand-rolled vtable layout | 5.1 |  | 0.541 |
| walker |  | 6256 | 44 | Code::CodeKey { rung: Names, file: src/fmt.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.542 |
| walker |  | 6291 | 35 | Code::CodeKey { rung: Decl, file: src/fmt.rs, decl: 4, sub: 0, line: 69 } |  |  | 0.542 |
| walker |  | 6340 | 49 | Code::CodeKey { rung: Decl, file: src/fmt.rs, decl: 5, sub: 0, line: 75 } |  |  | 0.542 |
| walker |  | 6405 | 65 | Code::CodeKey { rung: Decl, file: src/fmt.rs, decl: 1, sub: 0, line: 6 } |  |  | 0.542 |
| ns | 6483 |  | 231 | ErrorImpl, ContextError, and the vtable reader | 5.2 | 5.1 | 0.542 |
| ns | 6615 |  | 132 | Error::construct — allocation and type erasure | 5.3 | 5.2 | 0.535 |
| ns | 6718 |  | 103 | impl Error — roster of the non-public fns | 5.4 | 5.3 | 0.539 |
| walker |  | 6721 | 316 | Code::CodeKey { rung: Names, file: src/ptr.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.540 |
| walker |  | 6732 | 11 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 35, sub: 0, line: 181 } |  |  | 0.540 |
| walker |  | 6745 | 13 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 36, sub: 0, line: 185 } |  |  | 0.540 |
| walker |  | 6766 | 21 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 33, sub: 0, line: 174 } |  |  | 0.540 |
| walker |  | 6805 | 39 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 5, sub: 0, line: 19 } |  |  | 0.540 |
| walker |  | 6844 | 39 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 15, sub: 0, line: 74 } |  |  | 0.540 |
| walker |  | 6883 | 39 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 26, sub: 0, line: 135 } |  |  | 0.540 |
| ns | 6889 |  | 171 | Vtable function roster in error.rs | 5.5 | 5.1 | 0.533 |
| walker |  | 6928 | 45 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 1, sub: 0, line: 5 } |  |  | 0.533 |
| walker |  | 6987 | 59 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 13, sub: 0, line: 63 } |  |  | 0.534 |
| ns | 7040 |  | 151 | ErrorImpl methods and trait impls | 5.6 | 5.2 | 0.536 |
| walker |  | 7047 | 60 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 24, sub: 0, line: 124 } |  |  | 0.537 |
| walker |  | 7163 | 116 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 28, sub: 0, line: 144 } |  |  | 0.537 |
| ns | 7275 |  | 235 | ptr.rs — Own / Ref / Mut / CastTo declarations | 5.7 |  | 0.556 |
| walker |  | 7286 | 123 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 7, sub: 0, line: 28 } |  |  | 0.556 |
| walker |  | 7434 | 148 | Code::CodeKey { rung: Decl, file: src/ptr.rs, decl: 17, sub: 0, line: 83 } |  |  | 0.556 |
| ns | 7440 |  | 165 | chain.rs — Chain and ChainState | 6.1 |  | 0.548 |
| ns | 7557 |  | 117 | chain.rs — every trait impl on Chain | 6.2 | 6.1 | 0.544 |
| walker |  | 7578 | 144 | Code::CodeKey { rung: Names, file: src/kind.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.548 |
| walker |  | 7598 | 20 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 9, sub: 0, line: 99 } |  |  | 0.549 |
| walker |  | 7618 | 20 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 10, sub: 0, line: 111 } |  |  | 0.549 |
| walker |  | 7644 | 26 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 3, sub: 0, line: 67 } |  |  | 0.550 |
| walker |  | 7670 | 26 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 7, sub: 0, line: 89 } |  |  | 0.552 |
| walker |  | 7700 | 30 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 8, sub: 0, line: 90 } |  |  | 0.553 |
| walker |  | 7737 | 37 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 4, sub: 0, line: 68 } |  |  | 0.555 |
| ns | 7782 |  | 225 | wrapper.rs — the three error adapters and their impls | 6.3 |  | 0.547 |
| walker |  | 7787 | 50 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 11, sub: 0, line: 114 } |  |  | 0.549 |
| walker |  | 7793 | 6 | Code::CodeKey { rung: Decl, file: src/kind.rs, decl: 12, sub: 0, line: 116 } |  |  | 0.549 |
| walker |  | 7904 | 111 | Code::CodeKey { rung: Names, file: src/chain.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.552 |
| walker |  | 7924 | 20 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 10, sub: 0, line: 76 } |  |  | 0.553 |
| walker |  | 7954 | 30 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 3, sub: 0, line: 26 } |  |  | 0.554 |
| ns | 7959 |  | 177 | context.rs — where .context() is implemented | 6.4 | 2.3 | 0.550 |
| walker |  | 7960 | 6 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 4, sub: 0, line: 27 } |  |  | 0.550 |
| walker |  | 7994 | 34 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 12, sub: 0, line: 93 } |  |  | 0.550 |
| walker |  | 8029 | 35 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.550 |
| walker |  | 8073 | 44 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 8, sub: 0, line: 56 } |  |  | 0.553 |
| ns | 8133 |  | 174 | context.rs — ContextError impls, Quoted, and the Sealed trait | 6.5 | 6.4 | 0.559 |
| walker |  | 8141 | 68 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 5, sub: 0, line: 35 } |  |  | 0.562 |
| walker |  | 8240 | 99 | Code::CodeKey { rung: Decl, file: src/chain.rs, decl: 2, sub: 0, line: 15 } |  |  | 0.575 |
| ns | 8261 |  | 128 | fmt.rs — the rendering entry points | 6.6 | 5.6 | 0.579 |
| ns | 8413 |  | 152 | build.rs — the complete custom-cfg vocabulary | 7.1 |  | 0.575 |
| walker |  | 8450 | 210 | Code::CodeKey { rung: Names, file: src/wrapper.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.586 |
| walker |  | 8457 | 7 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 7, sub: 0, line: 33 } |  |  | 0.588 |
| walker |  | 8466 | 9 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.590 |
| walker |  | 8493 | 27 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 13, sub: 0, line: 56 } |  |  | 0.591 |
| ns | 8527 |  | 114 | build.rs — which compiler turns each cfg on | 7.2 | 7.1 | 0.588 |
| walker |  | 8541 | 48 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 8, sub: 0, line: 36 } |  |  | 0.588 |
| walker |  | 8589 | 48 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 10, sub: 0, line: 45 } |  |  | 0.588 |
| walker |  | 8637 | 48 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 14, sub: 0, line: 60 } |  |  | 0.585 |
| ns | 8637 |  | 110 | backtrace.rs — the three definitions of Backtrace | 7.3 |  | 0.585 |
| walker |  | 8685 | 48 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 16, sub: 0, line: 67 } |  |  | 0.585 |
| walker |  | 8735 | 50 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 2, sub: 0, line: 13 } |  |  | 0.585 |
| walker |  | 8785 | 50 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 4, sub: 0, line: 22 } |  |  | 0.585 |
| ns | 8813 |  | 176 | backtrace.rs — impl_backtrace! and backtrace!(), both arms each | 7.4 | 7.3 | 0.579 |
| walker |  | 8855 | 70 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 18, sub: 0, line: 74 } |  |  | 0.579 |
| walker |  | 8867 | 12 | Code::CodeKey { rung: Decl, file: src/wrapper.rs, decl: 20, sub: 0, line: 80 } |  |  | 0.579 |
| walker |  | 8875 | 8 | Code::CodeKey { rung: Body, file: src/kind.rs, decl: 8, sub: 0, line: 90 } |  |  | 0.579 |
| ns | 8954 |  | 141 | backtrace.rs — backtrace_if_absent! and the vendored capture module | 7.5 | 7.3 | 0.574 |
| ns | 9086 |  | 132 | nightly.rs — the build probe and generic-member-access shims | 7.6 | 7.1 | 0.570 |
| walker |  | 9099 | 224 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.570 |
| ns | 9217 |  | 131 | Toolchain pin and the no_std check-crate | 7.7 |  | 0.564 |
| walker |  | 9307 | 208 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.564 |
| ns | 9378 |  | 161 | CI job roster and toolchain matrix | 7.8 |  | 0.558 |
| ns | 9570 |  | 192 | CI commands — the canonical build/test/lint invocations | 7.9 | 7.8 | 0.553 |
| walker |  | 9573 | 266 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.553 |
| ns | 9658 |  | 88 | trybuild UI harness — tests/compiletest.rs in full | 8.1 |  | 0.550 |
| walker |  | 9678 | 105 | Code::CodeKey { rung: Names, file: src/nightly.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.552 |
| walker |  | 9690 | 12 | Code::CodeKey { rung: Body, file: src/nightly.rs, decl: 3, sub: 0, line: 56 } |  |  | 0.552 |
| walker |  | 9703 | 13 | Code::CodeKey { rung: Body, file: src/nightly.rs, decl: 1, sub: 0, line: 41 } |  |  | 0.552 |
| walker |  | 9717 | 14 | Code::CodeKey { rung: Body, file: src/nightly.rs, decl: 2, sub: 0, line: 52 } |  |  | 0.552 |
| ns | 9740 |  | 82 | tests/ui file listing | 8.2 | 8.1 | 0.558 |
| ns | 9849 |  | 109 | One complete compile-fail pair — wrong-interpolation | 8.3 | 8.2 | 0.554 |
| walker |  | 9918 | 201 | Code::CodeKey { rung: ModuleDoc, file: src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.554 |
| walker |  | 9959 | 41 | Code::CodeKey { rung: Body, file: src/ensure.rs, decl: 7, sub: 0, line: 41 } |  |  | 0.554 |
| ns | 9990 |  | 141 | Shared test helpers in tests/common and tests/drop | 8.4 | 4.4 | 0.548 |
