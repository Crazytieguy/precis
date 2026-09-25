Score(3000)=0.727 I=0.823 C=0.642 ns_rows≤3K=24/60 grid(1000/1442/2080/3000/4327/6240/9000)=0.732/0.607/0.705/0.727/0.614/0.731/0.735

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 47 |  | 47 | Package doc lede | 1.1 |  | 0.000 |
| walker |  | 85 | 85 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 113 |  | 66 | README title + positioning sentence | 1.2 |  | 0.000 |
| walker |  | 151 | 66 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.667 |
| walker |  | 162 | 11 | Fs::DirListing { dir: xxhsum } |  |  | 0.669 |
| walker |  | 176 | 14 | Fs::DirListing { dir: dynamic } |  |  | 0.677 |
| walker |  | 191 | 15 | Fs::DirListing { dir: xxhashbench } |  |  | 0.690 |
| ns | 198 |  | 85 | Complete root directory listing | 1.3 |  | 0.704 |
| walker |  | 222 | 31 | GoMod::Identity { file: go.mod } |  |  | 0.713 |
| walker |  | 222 | 0 | GoMod::File { file: go.mod } |  |  | 0.713 |
| walker |  | 225 | 3 | Fs::DirListing { dir: .github } |  |  | 0.713 |
| walker |  | 229 | 4 | Fs::DirListing { dir: .github/workflows } |  |  | 0.718 |
| walker |  | 268 | 39 | Code::CodeKey { rung: ModuleDoc, file: xxhash.go, decl: 0, sub: 0, line: 0 } |  |  | 0.939 |
| ns | 275 |  | 77 | README: the complete public API block | 1.4 |  | 0.795 |
| walker |  | 303 | 35 | GoMod::Identity { file: xxhashbench/go.mod } |  |  | 0.795 |
| walker |  | 334 | 31 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.800 |
| ns | 359 |  | 84 | README: Digest's key methods and hash.Hash64 conformance | 1.5 |  | 0.713 |
| ns | 435 |  | 76 | README: pure-Go vs assembly, and the `purego` tag | 1.6 |  | 0.665 |
| ns | 479 |  | 44 | All subdirectory listings (complete) | 1.7 |  | 0.696 |
| ns | 508 |  | 29 | README: remaining H2 section headings | 1.8 |  | 0.703 |
| ns | 539 |  | 31 | Root `go.mod`: module path and Go floor | 1.9 |  | 0.709 |
| walker |  | 579 | 245 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.934 |
| ns | 681 |  | 142 | `Digest` doc comment and all seven struct fields | 2.1 |  | 0.832 |
| walker |  | 758 | 179 | Code::CodeKey { rung: Names, file: xxhash.go, decl: 0, sub: 0, line: 0 } |  |  | 0.835 |
| walker |  | 769 | 11 | Code::CodeKey { rung: Decl, file: xxhash.go, decl: 1, sub: 0, line: 11 } |  |  | 0.835 |
| walker |  | 848 | 79 | Code::CodeKey { rung: Decl, file: xxhash.go, decl: 3, sub: 0, line: 29 } |  |  | 0.868 |
| ns | 849 |  | 168 | `xxhash_asm.go` in full: build tags + the two assembly-backed declarations | 2.2 |  | 0.777 |
| walker |  | 859 | 11 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 4, sub: 0, line: 40 } |  |  | 0.777 |
| walker |  | 870 | 11 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 6, sub: 0, line: 53 } |  |  | 0.778 |
| walker |  | 883 | 13 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 4, sub: 0, line: 40 } |  |  | 0.778 |
| walker |  | 900 | 17 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 5, sub: 0, line: 45 } |  |  | 0.779 |
| ns | 970 |  | 121 | `xxhash_other.go`: complementary build tags + pure-Go `Sum64`/`writeBlocks` signatures | 2.3 |  | 0.731 |
| walker |  | 1083 | 183 | Code::CodeKey { rung: Names, file: xxhash.go, decl: 0, sub: 1, line: 0 } |  |  | 0.733 |
| walker |  | 1094 | 11 | Code::CodeKey { rung: Decl, file: xxhash.go, decl: 13, sub: 0, line: 170 } |  |  | 0.734 |
| walker |  | 1105 | 11 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 9, sub: 0, line: 72 } |  |  | 0.734 |
| walker |  | 1117 | 12 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 8, sub: 0, line: 69 } |  |  | 0.735 |
| walker |  | 1129 | 12 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 12, sub: 0, line: 129 } |  |  | 0.735 |
| walker |  | 1143 | 14 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 14, sub: 0, line: 176 } |  |  | 0.736 |
| walker |  | 1161 | 18 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 10, sub: 0, line: 75 } |  |  | 0.737 |
| walker |  | 1180 | 19 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 11, sub: 0, line: 113 } |  |  | 0.738 |
| ns | 1252 |  | 282 | `xxhash_unsafe.go`: `!appengine` tag, `Sum64String`/`WriteString` signatures, `sliceHeader` | 2.4 |  | 0.649 |
| walker |  | 1359 | 179 | Code::CodeKey { rung: Names, file: xxhash.go, decl: 0, sub: 2, line: 0 } |  |  | 0.651 |
| walker |  | 1377 | 18 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 15, sub: 0, line: 190 } |  |  | 0.652 |
| ns | 1432 |  | 180 | `xxhash_safe.go` in full: the `appengine` fallbacks | 2.5 |  | 0.607 |
| ns | 1541 |  | 109 | Constructors: `New` and `NewWithSeed`, with bodies | 2.6 |  | 0.598 |
| walker |  | 1551 | 174 | Code::CodeKey { rung: Names, file: xxhash.go, decl: 0, sub: 3, line: 0 } |  |  | 0.601 |
| walker |  | 1582 | 31 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 6, sub: 0, line: 53 } |  |  | 0.602 |
| walker |  | 1617 | 35 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 7, sub: 0, line: 59 } |  |  | 0.604 |
| walker |  | 1683 | 66 | Code::CodeKey { rung: Names, file: xxhash_unsafe.go, decl: 0, sub: 0, line: 0 } |  |  | 0.608 |
| walker |  | 1702 | 19 | Code::CodeKey { rung: Decl, file: xxhash_unsafe.go, decl: 3, sub: 0, line: 55 } |  |  | 0.612 |
| ns | 1718 |  | 177 | `Reset`, `ResetWithSeed`, `Size`, `BlockSize` signatures | 2.7 |  | 0.627 |
| walker |  | 1753 | 51 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 3, sub: 0, line: 29 } |  |  | 0.671 |
| ns | 1830 |  | 112 | `Write`, `Sum`, `Sum64` doc comments + signatures | 2.8 |  | 0.672 |
| walker |  | 1889 | 136 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.673 |
| walker |  | 1942 | 53 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 2, sub: 0, line: 23 } |  |  | 0.675 |
| walker |  | 1996 | 54 | Code::CodeKey { rung: Names, file: xxhash_safe.go, decl: 0, sub: 0, line: 0 } |  |  | 0.679 |
| walker |  | 2007 | 11 | Code::CodeKey { rung: Body, file: xxhash_safe.go, decl: 1, sub: 0, line: 9 } |  |  | 0.681 |
| walker |  | 2018 | 11 | Code::CodeKey { rung: Body, file: xxhash_safe.go, decl: 2, sub: 0, line: 14 } |  |  | 0.683 |
| ns | 2022 |  | 192 | The five XXH64 primes and the `primes` array | 2.9 |  | 0.694 |
| walker |  | 2039 | 21 | Code::CodeKey { rung: Doc, file: xxhash_safe.go, decl: 2, sub: 0, line: 14 } |  |  | 0.699 |
| walker |  | 2062 | 23 | Code::CodeKey { rung: Doc, file: xxhash_safe.go, decl: 1, sub: 0, line: 9 } |  |  | 0.705 |
| walker |  | 2123 | 61 | Code::CodeKey { rung: Names, file: xxhash_asm.go, decl: 0, sub: 0, line: 0 } |  |  | 0.710 |
| walker |  | 2130 | 7 | Code::CodeKey { rung: Doc, file: xxhash_asm.go, decl: 2, sub: 0, line: 15 } |  |  | 0.713 |
| ns | 2145 |  | 123 | Marshaling: `magic`/`marshaledSize` constants + `MarshalBinary`/`UnmarshalBinary` signatures | 2.10 |  | 0.715 |
| walker |  | 2193 | 63 | Code::CodeKey { rung: Names, file: xxhash_other.go, decl: 0, sub: 0, line: 0 } |  |  | 0.720 |
| walker |  | 2215 | 22 | Code::CodeKey { rung: Doc, file: xxhash_other.go, decl: 1, sub: 0, line: 7 } |  |  | 0.726 |
| ns | 2217 |  | 72 | Roster: the byte-level helpers `appendUint64`, `consumeUint64`, `u64`, `u32` | 2.11 |  | 0.729 |
| walker |  | 2246 | 31 | Code::CodeKey { rung: Body, file: xxhash_unsafe.go, decl: 1, sub: 0, line: 38 } |  |  | 0.730 |
| walker |  | 2282 | 36 | Code::CodeKey { rung: Doc, file: xxhash_asm.go, decl: 1, sub: 0, line: 12 } |  |  | 0.742 |
| walker |  | 2429 | 147 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.742 |
| ns | 2436 |  | 219 | Roster: `round`, `mergeRound`, and the complete `rol*` rotate family | 2.12 |  | 0.747 |
| walker |  | 2470 | 41 | Code::CodeKey { rung: Doc, file: xxhash_unsafe.go, decl: 2, sub: 0, line: 45 } |  |  | 0.755 |
| ns | 2511 |  | 75 | `ResetWithSeed` body: how a seed becomes the four lanes | 3.1 | 2.7 | 0.740 |
| walker |  | 2512 | 42 | Code::CodeKey { rung: Doc, file: xxhash_unsafe.go, decl: 3, sub: 0, line: 55 } |  |  | 0.751 |
| walker |  | 2556 | 44 | Code::CodeKey { rung: Doc, file: xxhash_unsafe.go, decl: 1, sub: 0, line: 38 } |  |  | 0.763 |
| walker |  | 2602 | 46 | Code::CodeKey { rung: Names, file: xxhsum/xxhsum.go, decl: 0, sub: 0, line: 0 } |  |  | 0.763 |
| ns | 2616 |  | 105 | `round` and `mergeRound` bodies | 3.2 | 2.12 | 0.739 |
| ns | 2737 |  | 121 | `xxhash_unsafe.go`: the actual unsafe string-to-slice conversion | 3.3 | 2.4 | 0.724 |
| walker |  | 2838 | 236 | Code::CodeKey { rung: Decl, file: xxhsum/xxhsum.go, decl: 1, sub: 0, line: 11 } |  |  | 0.726 |
| walker |  | 2928 | 90 | Code::CodeKey { rung: Names, file: dynamic/plugin.go, decl: 0, sub: 0, line: 0 } |  |  | 0.726 |
| walker |  | 2937 | 9 | Code::CodeKey { rung: Decl, file: dynamic/plugin.go, decl: 1, sub: 0, line: 14 } |  |  | 0.727 |
| ns | 3010 |  | 273 | `Write` body, part 1: buffering into `mem` and flushing a partial block | 3.4 | 2.8 | 0.690 |
| walker |  | 3048 | 111 | Plaintext::DeclSurface { file: testall.sh } |  |  | 0.692 |
| walker |  | 3054 | 6 | Plaintext::Whole { file: testall.sh } |  |  | 0.692 |
| ns | 3119 |  | 109 | `Write` body, part 2: full blocks via `writeBlocks`, then store the remainder | 3.5 | 2.8 | 0.674 |
| walker |  | 3167 | 113 | GoMod::File { file: xxhashbench/go.mod } |  |  | 0.674 |
| ns | 3328 |  | 209 | `Sum64` body, part 1: merging the four lanes (and the <32-byte shortcut) | 3.6 | 2.8 | 0.652 |
| walker |  | 3478 | 311 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.654 |
| walker |  | 3529 | 51 | Code::CodeKey { rung: Body, file: xxhsum/xxhsum.go, decl: 2, sub: 0, line: 34 } |  |  | 0.655 |
| walker |  | 3585 | 56 | Code::CodeKey { rung: Body, file: dynamic/plugin.go, decl: 2, sub: 0, line: 19 } |  |  | 0.655 |
| ns | 3593 |  | 265 | `Sum64` body, part 2: 8/4/1-byte tail loops and the final avalanche | 3.7 | 2.8 | 0.627 |
| walker |  | 3660 | 75 | Code::CodeKey { rung: Body, file: xxhsum/xxhsum.go, decl: 3, sub: 0, line: 43 } |  |  | 0.628 |
| ns | 3704 |  | 111 | `Sum` body: big-endian append of the 64-bit digest | 3.8 | 2.8 | 0.613 |
| walker |  | 3744 | 84 | Code::CodeKey { rung: Body, file: xxhash_unsafe.go, decl: 2, sub: 0, line: 45 } |  |  | 0.630 |
| ns | 3852 |  | 148 | `MarshalBinary` body: the serialized state layout | 3.9 | 2.10 | 0.618 |
| walker |  | 3958 | 214 | Code::CodeKey { rung: Body, file: xxhash_other.go, decl: 2, sub: 0, line: 64 } |  |  | 0.622 |
| walker |  | 3979 | 21 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 17, sub: 0, line: 214 } |  |  | 0.622 |
| walker |  | 4005 | 26 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 5, sub: 0, line: 45 } |  |  | 0.631 |
| ns | 4059 |  | 207 | `UnmarshalBinary` body: validation and both error strings | 3.10 | 2.10 | 0.614 |
| ns | 4150 |  | 91 | `appendUint64` / `consumeUint64` bodies | 3.11 | 2.11 | 0.609 |
| ns | 4367 |  | 217 | `xxhash_other.go`: the pure-Go `writeBlocks` body | 3.12 | 2.3 | 0.617 |
| walker |  | 4402 | 397 | Code::CodeKey { rung: Body, file: xxhash_other.go, decl: 1, sub: 0, line: 7 } |  |  | 0.623 |
| ns | 4654 |  | 287 | `xxhash_other.go` `Sum64` body, part 1: why it is not `New/Write/Sum64`, and the block loop | 3.13 | 2.3 | 0.638 |
| walker |  | 4678 | 276 | Code::CodeKey { rung: Body, file: dynamic/plugin.go, decl: 3, sub: 0, line: 26 } |  |  | 0.638 |
| ns | 4784 |  | 130 | `xxhash_other.go` `Sum64`, part 2: lane merge, the small-input `prime5` branch, length mix | 3.14 | 2.3 | 0.639 |
| walker |  | 4955 | 277 | Code::CodeKey { rung: Body, file: xxhash_other.go, decl: 1, sub: 1, line: 7 } |  |  | 0.647 |
| walker |  | 4992 | 37 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 20, sub: 0, line: 222 } |  |  | 0.653 |
| walker |  | 5031 | 39 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 16, sub: 0, line: 208 } |  |  | 0.661 |
| ns | 5049 |  | 265 | `xxhash_other.go` `Sum64`, part 3: tail loops and avalanche | 3.15 | 2.3 | 0.671 |
| walker |  | 5072 | 41 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 21, sub: 0, line: 229 } |  |  | 0.681 |
| walker |  | 5118 | 46 | Code::CodeKey { rung: Names, file: xxhash_unsafe_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.681 |
| walker |  | 5197 | 79 | Code::CodeKey { rung: Names, file: bench_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.682 |
| walker |  | 5285 | 88 | Code::CodeKey { rung: Decl, file: bench_test.go, decl: 1, sub: 0, line: 8 } |  |  | 0.683 |
| walker |  | 5357 | 72 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 7, sub: 0, line: 59 } |  |  | 0.695 |
| ns | 5393 |  | 344 | `xxhash_unsafe.go`: the inliner-cost design commentary | 3.16 | 2.4 | 0.675 |
| ns | 5510 |  | 117 | `testall.sh` in full: the four tag/arch test combinations | 4.1 |  | 0.680 |
| walker |  | 5525 | 168 | Code::CodeKey { rung: Names, file: xxhash_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.681 |
| ns | 5608 |  | 98 | Roster of every function in `xxhash_test.go` | 4.2 |  | 0.685 |
| walker |  | 5633 | 108 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 11, sub: 0, line: 113 } |  |  | 0.704 |
| walker |  | 5673 | 40 | Code::CodeKey { rung: Doc, file: xxhash_unsafe_test.go, decl: 2, sub: 0, line: 31 } |  |  | 0.704 |
| walker |  | 5718 | 45 | Code::CodeKey { rung: Names, file: dynamic/dynamic_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.704 |
| walker |  | 5863 | 145 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 14, sub: 0, line: 176 } |  |  | 0.720 |
| ns | 5936 |  | 328 | The complete golden-vector table in `TestAll` | 4.3 | 4.2 | 0.705 |
| walker |  | 5960 | 97 | Code::CodeKey { rung: Names, file: xxhashbench/xxhashbench_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.705 |
| walker |  | 6020 | 60 | Code::CodeKey { rung: Body, file: xxhash_test.go, decl: 9, sub: 0, line: 193 } |  |  | 0.705 |
| ns | 6083 |  | 147 | `bench_test.go`: the input-size table and all four benchmark names | 4.4 |  | 0.710 |
| ns | 6175 |  | 92 | `xxhash_unsafe_test.go`: build tag and both test names | 4.5 |  | 0.710 |
| walker |  | 6224 | 204 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 15, sub: 0, line: 190 } |  |  | 0.731 |
| ns | 6298 |  | 123 | `TestInlining`: which functions must inline, and how it checks | 4.6 | 4.5 | 0.721 |
| walker |  | 6324 | 100 | Code::CodeKey { rung: Body, file: xxhash_test.go, decl: 3, sub: 0, line: 88 } |  |  | 0.721 |
| ns | 6520 |  | 222 | `xxhsum/`: the CLI's usage text and argument handling | 4.7 |  | 0.715 |
| ns | 6790 |  | 270 | `xxhsum/`: the file loop, `contains`, and the output format | 4.8 | 4.7 | 0.718 |
| ns | 6928 |  | 138 | `dynamic/`: the `plugin` build-tag file and its two exported tests | 4.9 |  | 0.717 |
| ns | 7158 |  | 230 | `dynamic/dynamic_test.go`: building the plugin and calling into it | 4.10 |  | 0.702 |
| walker |  | 7279 | 955 | Code::CodeKey { rung: Decl, file: xxhashbench/xxhashbench_test.go, decl: 2, sub: 0, line: 20 } |  |  | 0.704 |
| ns | 7367 |  | 209 | `xxhashbench/`: separate module, `replace ../`, and its deprecation TODO | 4.11 |  | 0.700 |
| walker |  | 7387 | 108 | Code::CodeKey { rung: Body, file: bench_test.go, decl: 3, sub: 0, line: 34 } |  |  | 0.700 |
| walker |  | 7434 | 47 | Code::CodeKey { rung: Body, file: xxhashbench/xxhashbench_test.go, decl: 4, sub: 0, line: 152 } |  |  | 0.700 |
| walker |  | 7481 | 47 | Code::CodeKey { rung: Body, file: xxhashbench/xxhashbench_test.go, decl: 5, sub: 0, line: 159 } |  |  | 0.700 |
| ns | 7566 |  | 199 | `xxhashbench/`: the comparison table's shape and every hash it compares | 4.12 | 4.11 | 0.706 |
| walker |  | 7607 | 126 | Code::CodeKey { rung: Body, file: bench_test.go, decl: 5, sub: 0, line: 63 } |  |  | 0.706 |
| walker |  | 7741 | 134 | Code::CodeKey { rung: Body, file: bench_test.go, decl: 2, sub: 0, line: 19 } |  |  | 0.706 |
| walker |  | 7892 | 151 | Code::CodeKey { rung: Body, file: xxhash_unsafe_test.go, decl: 1, sub: 0, line: 13 } |  |  | 0.706 |
| ns | 7899 |  | 333 | CI: the test matrix and the exact commands run | 4.13 |  | 0.691 |
| ns | 8030 |  | 131 | README: Compatibility section body | 4.14 | 1.8 | 0.693 |
| walker |  | 8059 | 167 | Code::CodeKey { rung: Body, file: xxhash_test.go, decl: 4, sub: 0, line: 97 } |  |  | 0.693 |
| ns | 8336 |  | 306 | README: measured purego-vs-asm throughput table and how it was produced | 4.15 | 1.8 | 0.698 |
| ns | 8364 |  | 28 | License identification | 4.16 |  | 0.696 |
| walker |  | 8443 | 384 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 10, sub: 0, line: 75 } |  |  | 0.732 |
| ns | 8514 |  | 150 | `xxhash_amd64.s`: build tags and both `TEXT` symbol definitions | 5.1 |  | 0.725 |
| walker |  | 8618 | 175 | Code::CodeKey { rung: Body, file: xxhash_test.go, decl: 5, sub: 0, line: 114 } |  |  | 0.725 |
| ns | 8664 |  | 150 | `xxhash_arm64.s`: build tags and both `TEXT` symbol definitions | 5.2 |  | 0.718 |
| ns | 8821 |  | 157 | `xxhash_amd64.s`: the complete register-allocation map | 5.3 | 5.1 | 0.710 |
| ns | 9036 |  | 215 | `xxhash_arm64.s`: the complete register-allocation map | 5.4 | 5.2 | 0.700 |
| walker |  | 9089 | 471 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 12, sub: 0, line: 129 } |  |  | 0.737 |
| ns | 9176 |  | 140 | `xxhash_amd64.s`: `round` and `round0` macro bodies | 5.5 | 5.3 | 0.731 |
| walker |  | 9241 | 152 | Code::CodeKey { rung: Body, file: bench_test.go, decl: 4, sub: 0, line: 46 } |  |  | 0.731 |
| ns | 9348 |  | 172 | `xxhash_amd64.s`: `mergeRound` and `blockLoop` macros | 5.6 | 5.5 | 0.724 |
| ns | 9400 |  | 52 | `xxhash_arm64.s`: the complete macro roster | 5.7 | 5.4 | 0.720 |
| walker |  | 9540 | 299 | Code::CodeKey { rung: Body, file: xxhash_unsafe_test.go, decl: 2, sub: 0, line: 31 } |  |  | 0.730 |
| walker |  | 9687 | 147 | Code::CodeKey { rung: Body, file: dynamic/dynamic_test.go, decl: 1, sub: 0, line: 17 } |  |  | 0.734 |
| walker |  | 9836 | 149 | Code::CodeKey { rung: Body, file: dynamic/dynamic_test.go, decl: 2, sub: 0, line: 33 } |  |  | 0.742 |
| walker |  | 9998 | 162 | Code::CodeKey { rung: Body, file: xxhash_test.go, decl: 8, sub: 0, line: 170 } |  |  | 0.742 |
