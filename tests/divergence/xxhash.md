Score(3000)=0.625 I=0.781 C=0.500 ns_rows≤3K=24/60 grid(1000/1442/2080/3000/4327/6240/9000)=0.691/0.602/0.635/0.625/0.643/0.679/0.648

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
| walker |  | 228 | 6 | Fs::DirListing { dir: .github/workflows } |  |  | 0.718 |
| walker |  | 267 | 39 | Code::CodeKey { rung: ModuleDoc, file: xxhash.go, decl: 0, sub: 0, line: 0 } |  |  | 0.939 |
| ns | 275 |  | 77 | README: the complete public API block | 1.4 |  | 0.795 |
| walker |  | 302 | 35 | GoMod::Identity { file: xxhashbench/go.mod } |  |  | 0.795 |
| walker |  | 333 | 31 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.800 |
| ns | 359 |  | 84 | README: Digest's key methods and hash.Hash64 conformance | 1.5 |  | 0.713 |
| ns | 435 |  | 76 | README: pure-Go vs assembly, and the `purego` tag | 1.6 |  | 0.665 |
| ns | 479 |  | 44 | All subdirectory listings (complete) | 1.7 |  | 0.696 |
| ns | 508 |  | 29 | README: remaining H2 section headings | 1.8 |  | 0.703 |
| ns | 539 |  | 31 | Root `go.mod`: module path and Go floor | 1.9 |  | 0.709 |
| walker |  | 561 | 228 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.910 |
| ns | 681 |  | 142 | `Digest` doc comment and all seven struct fields | 2.1 |  | 0.810 |
| walker |  | 781 | 220 | Code::CodeKey { rung: Names, file: xxhash.go, decl: 0, sub: 0, line: 0 } |  |  | 0.813 |
| walker |  | 788 | 7 | Code::CodeKey { rung: Decl, file: xxhash.go, decl: 1, sub: 0, line: 29 } |  |  | 0.813 |
| walker |  | 799 | 11 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 7, sub: 0, line: 72 } |  |  | 0.814 |
| walker |  | 811 | 12 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 6, sub: 0, line: 69 } |  |  | 0.814 |
| walker |  | 823 | 12 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 10, sub: 0, line: 129 } |  |  | 0.815 |
| walker |  | 836 | 13 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 2, sub: 0, line: 40 } |  |  | 0.815 |
| ns | 849 |  | 168 | `xxhash_asm.go` in full: build tags + the two assembly-backed declarations | 2.2 |  | 0.730 |
| walker |  | 852 | 16 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 11, sub: 0, line: 176 } |  |  | 0.730 |
| walker |  | 869 | 17 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 3, sub: 0, line: 45 } |  |  | 0.731 |
| walker |  | 887 | 18 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 8, sub: 0, line: 75 } |  |  | 0.732 |
| walker |  | 905 | 18 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 12, sub: 0, line: 190 } |  |  | 0.733 |
| walker |  | 924 | 19 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 9, sub: 0, line: 113 } |  |  | 0.734 |
| walker |  | 935 | 11 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 2, sub: 0, line: 40 } |  |  | 0.734 |
| walker |  | 946 | 11 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 4, sub: 0, line: 53 } |  |  | 0.735 |
| ns | 970 |  | 121 | `xxhash_other.go`: complementary build tags + pure-Go `Sum64`/`writeBlocks` signatures | 2.3 |  | 0.690 |
| walker |  | 977 | 31 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 4, sub: 0, line: 53 } |  |  | 0.691 |
| walker |  | 1012 | 35 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 5, sub: 0, line: 59 } |  |  | 0.693 |
| walker |  | 1148 | 136 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.694 |
| walker |  | 1203 | 55 | Code::CodeKey { rung: Names, file: xxhash_unsafe.go, decl: 0, sub: 0, line: 0 } |  |  | 0.695 |
| ns | 1252 |  | 282 | `xxhash_unsafe.go`: `!appengine` tag, `Sum64String`/`WriteString` signatures, `sliceHeader` | 2.4 |  | 0.613 |
| walker |  | 1256 | 53 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 1, sub: 0, line: 29 } |  |  | 0.636 |
| walker |  | 1369 | 113 | GoMod::File { file: xxhashbench/go.mod } |  |  | 0.637 |
| walker |  | 1413 | 44 | Code::CodeKey { rung: Names, file: xxhash_other.go, decl: 0, sub: 0, line: 0 } |  |  | 0.641 |
| ns | 1432 |  | 180 | `xxhash_safe.go` in full: the `appengine` fallbacks | 2.5 |  | 0.596 |
| walker |  | 1435 | 22 | Code::CodeKey { rung: Doc, file: xxhash_other.go, decl: 1, sub: 0, line: 7 } |  |  | 0.602 |
| walker |  | 1479 | 44 | Code::CodeKey { rung: Names, file: xxhash_asm.go, decl: 0, sub: 0, line: 0 } |  |  | 0.605 |
| walker |  | 1488 | 9 | Code::CodeKey { rung: Decl, file: xxhash_asm.go, decl: 1, sub: 0, line: 11 } |  |  | 0.607 |
| ns | 1541 |  | 109 | Constructors: `New` and `NewWithSeed`, with bodies | 2.6 |  | 0.598 |
| walker |  | 1542 | 54 | Code::CodeKey { rung: Names, file: xxhash_safe.go, decl: 0, sub: 0, line: 0 } |  |  | 0.602 |
| walker |  | 1553 | 11 | Code::CodeKey { rung: Body, file: xxhash_safe.go, decl: 1, sub: 0, line: 9 } |  |  | 0.605 |
| walker |  | 1564 | 11 | Code::CodeKey { rung: Body, file: xxhash_safe.go, decl: 2, sub: 0, line: 14 } |  |  | 0.608 |
| walker |  | 1585 | 21 | Code::CodeKey { rung: Doc, file: xxhash_safe.go, decl: 2, sub: 0, line: 14 } |  |  | 0.614 |
| walker |  | 1608 | 23 | Code::CodeKey { rung: Doc, file: xxhash_safe.go, decl: 1, sub: 0, line: 9 } |  |  | 0.623 |
| walker |  | 1635 | 27 | Code::CodeKey { rung: Doc, file: xxhash_asm.go, decl: 1, sub: 0, line: 11 } |  |  | 0.632 |
| ns | 1718 |  | 177 | `Reset`, `ResetWithSeed`, `Size`, `BlockSize` signatures | 2.7 |  | 0.644 |
| walker |  | 1782 | 147 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.644 |
| walker |  | 1813 | 31 | Code::CodeKey { rung: Body, file: xxhash_unsafe.go, decl: 1, sub: 0, line: 38 } |  |  | 0.645 |
| ns | 1830 |  | 112 | `Write`, `Sum`, `Sum64` doc comments + signatures | 2.8 |  | 0.646 |
| walker |  | 1839 | 26 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 3, sub: 0, line: 45 } |  |  | 0.661 |
| walker |  | 1885 | 46 | Code::CodeKey { rung: Names, file: xxhsum/xxhsum.go, decl: 0, sub: 0, line: 0 } |  |  | 0.661 |
| ns | 2022 |  | 192 | The five XXH64 primes and the `primes` array | 2.9 |  | 0.634 |
| walker |  | 2121 | 236 | Code::CodeKey { rung: Decl, file: xxhsum/xxhsum.go, decl: 1, sub: 0, line: 11 } |  |  | 0.636 |
| ns | 2145 |  | 123 | Marshaling: `magic`/`marshaledSize` constants + `MarshalBinary`/`UnmarshalBinary` signatures | 2.10 |  | 0.626 |
| walker |  | 2162 | 41 | Code::CodeKey { rung: Doc, file: xxhash_unsafe.go, decl: 2, sub: 0, line: 45 } |  |  | 0.632 |
| walker |  | 2206 | 44 | Code::CodeKey { rung: Doc, file: xxhash_unsafe.go, decl: 1, sub: 0, line: 38 } |  |  | 0.640 |
| ns | 2217 |  | 72 | Roster: the byte-level helpers `appendUint64`, `consumeUint64`, `u64`, `u32` | 2.11 |  | 0.632 |
| walker |  | 2317 | 111 | Plaintext::DeclSurface { file: testall.sh } |  |  | 0.633 |
| walker |  | 2323 | 6 | Plaintext::Whole { file: testall.sh } |  |  | 0.634 |
| ns | 2436 |  | 219 | Roster: `round`, `mergeRound`, and the complete `rol*` rotate family | 2.12 |  | 0.616 |
| ns | 2511 |  | 75 | `ResetWithSeed` body: how a seed becomes the four lanes | 3.1 | 2.7 | 0.604 |
| ns | 2616 |  | 105 | `round` and `mergeRound` bodies | 3.2 | 2.12 | 0.584 |
| walker |  | 2634 | 311 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.585 |
| walker |  | 2706 | 72 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 5, sub: 0, line: 59 } |  |  | 0.607 |
| ns | 2737 |  | 121 | `xxhash_unsafe.go`: the actual unsafe string-to-slice conversion | 3.3 | 2.4 | 0.595 |
| walker |  | 2790 | 84 | Code::CodeKey { rung: Body, file: xxhash_unsafe.go, decl: 2, sub: 0, line: 45 } |  |  | 0.616 |
| walker |  | 2898 | 108 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 9, sub: 0, line: 113 } |  |  | 0.621 |
| walker |  | 2949 | 51 | Code::CodeKey { rung: Body, file: xxhsum/xxhsum.go, decl: 2, sub: 0, line: 34 } |  |  | 0.622 |
| ns | 3010 |  | 273 | `Write` body, part 1: buffering into `mem` and flushing a partial block | 3.4 | 2.8 | 0.591 |
| walker |  | 3094 | 145 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 11, sub: 0, line: 176 } |  |  | 0.595 |
| ns | 3119 |  | 109 | `Write` body, part 2: full blocks via `writeBlocks`, then store the remainder | 3.5 | 2.8 | 0.580 |
| walker |  | 3169 | 75 | Code::CodeKey { rung: Body, file: xxhsum/xxhsum.go, decl: 3, sub: 0, line: 43 } |  |  | 0.581 |
| ns | 3328 |  | 209 | `Sum64` body, part 1: merging the four lanes (and the <32-byte shortcut) | 3.6 | 2.8 | 0.562 |
| walker |  | 3373 | 204 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 12, sub: 0, line: 190 } |  |  | 0.567 |
| ns | 3593 |  | 265 | `Sum64` body, part 2: 8/4/1-byte tail loops and the final avalanche | 3.7 | 2.8 | 0.542 |
| ns | 3704 |  | 111 | `Sum` body: big-endian append of the 64-bit digest | 3.8 | 2.8 | 0.555 |
| walker |  | 3770 | 397 | Code::CodeKey { rung: Body, file: xxhash_other.go, decl: 1, sub: 0, line: 7 } |  |  | 0.561 |
| ns | 3852 |  | 148 | `MarshalBinary` body: the serialized state layout | 3.9 | 2.10 | 0.570 |
| ns | 4059 |  | 207 | `UnmarshalBinary` body: validation and both error strings | 3.10 | 2.10 | 0.584 |
| ns | 4150 |  | 91 | `appendUint64` / `consumeUint64` bodies | 3.11 | 2.11 | 0.574 |
| walker |  | 4154 | 384 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 8, sub: 0, line: 75 } |  |  | 0.642 |
| walker |  | 4200 | 46 | Code::CodeKey { rung: Names, file: xxhash_unsafe_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.642 |
| walker |  | 4283 | 83 | Code::CodeKey { rung: Names, file: xxhash_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.642 |
| walker |  | 4350 | 67 | Code::CodeKey { rung: Names, file: bench_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.643 |
| ns | 4367 |  | 217 | `xxhash_other.go`: the pure-Go `writeBlocks` body | 3.12 | 2.3 | 0.631 |
| walker |  | 4627 | 277 | Code::CodeKey { rung: Body, file: xxhash_other.go, decl: 1, sub: 1, line: 7 } |  |  | 0.634 |
| ns | 4654 |  | 287 | `xxhash_other.go` `Sum64` body, part 1: why it is not `New/Write/Sum64`, and the block loop | 3.13 | 2.3 | 0.648 |
| ns | 4784 |  | 130 | `xxhash_other.go` `Sum64`, part 2: lane merge, the small-input `prime5` branch, length mix | 3.14 | 2.3 | 0.652 |
| ns | 5049 |  | 265 | `xxhash_other.go` `Sum64`, part 3: tail loops and avalanche | 3.15 | 2.3 | 0.662 |
| walker |  | 5098 | 471 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 10, sub: 0, line: 129 } |  |  | 0.723 |
| walker |  | 5143 | 45 | Code::CodeKey { rung: Names, file: dynamic/dynamic_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.723 |
| walker |  | 5183 | 40 | Code::CodeKey { rung: Doc, file: xxhash_unsafe_test.go, decl: 2, sub: 0, line: 31 } |  |  | 0.723 |
| walker |  | 5200 | 17 | Code::CodeKey { rung: Names, file: xxhashbench/xxhashbench_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.723 |
| walker |  | 5308 | 108 | Code::CodeKey { rung: Body, file: bench_test.go, decl: 2, sub: 0, line: 34 } |  |  | 0.723 |
| ns | 5393 |  | 344 | `xxhash_unsafe.go`: the inliner-cost design commentary | 3.16 | 2.4 | 0.703 |
| walker |  | 5459 | 151 | Code::CodeKey { rung: Body, file: xxhash_unsafe_test.go, decl: 1, sub: 0, line: 13 } |  |  | 0.703 |
| ns | 5510 |  | 117 | `testall.sh` in full: the four tag/arch test combinations | 4.1 |  | 0.706 |
| walker |  | 5585 | 126 | Code::CodeKey { rung: Body, file: bench_test.go, decl: 4, sub: 0, line: 63 } |  |  | 0.706 |
| ns | 5608 |  | 98 | Roster of every function in `xxhash_test.go` | 4.2 |  | 0.702 |
| walker |  | 5752 | 167 | Code::CodeKey { rung: Body, file: xxhash_test.go, decl: 2, sub: 0, line: 97 } |  |  | 0.702 |
| walker |  | 5886 | 134 | Code::CodeKey { rung: Body, file: bench_test.go, decl: 1, sub: 0, line: 19 } |  |  | 0.702 |
| ns | 5936 |  | 328 | The complete golden-vector table in `TestAll` | 4.3 | 4.2 | 0.687 |
| walker |  | 6061 | 175 | Code::CodeKey { rung: Body, file: xxhash_test.go, decl: 3, sub: 0, line: 114 } |  |  | 0.687 |
| ns | 6083 |  | 147 | `bench_test.go`: the input-size table and all four benchmark names | 4.4 |  | 0.678 |
| ns | 6175 |  | 92 | `xxhash_unsafe_test.go`: build tag and both test names | 4.5 |  | 0.678 |
| walker |  | 6208 | 147 | Code::CodeKey { rung: Body, file: dynamic/dynamic_test.go, decl: 1, sub: 0, line: 17 } |  |  | 0.679 |
| ns | 6298 |  | 123 | `TestInlining`: which functions must inline, and how it checks | 4.6 | 4.5 | 0.670 |
| walker |  | 6360 | 152 | Code::CodeKey { rung: Body, file: bench_test.go, decl: 3, sub: 0, line: 46 } |  |  | 0.670 |
| ns | 6520 |  | 222 | `xxhsum/`: the CLI's usage text and argument handling | 4.7 |  | 0.664 |
| walker |  | 6659 | 299 | Code::CodeKey { rung: Body, file: xxhash_unsafe_test.go, decl: 2, sub: 0, line: 31 } |  |  | 0.678 |
| ns | 6790 |  | 270 | `xxhsum/`: the file loop, `contains`, and the output format | 4.8 | 4.7 | 0.683 |
| walker |  | 6808 | 149 | Code::CodeKey { rung: Body, file: dynamic/dynamic_test.go, decl: 2, sub: 0, line: 33 } |  |  | 0.684 |
| ns | 6928 |  | 138 | `dynamic/`: the `plugin` build-tag file and its two exported tests | 4.9 |  | 0.675 |
| walker |  | 7068 | 260 | Code::CodeKey { rung: Body, file: xxhash_test.go, decl: 5, sub: 0, line: 170 } |  |  | 0.675 |
| ns | 7158 |  | 230 | `dynamic/dynamic_test.go`: building the plugin and calling into it | 4.10 |  | 0.674 |
| walker |  | 7275 | 207 | Code::CodeKey { rung: Body, file: xxhashbench/xxhashbench_test.go, decl: 1, sub: 0, line: 108 } |  |  | 0.674 |
| ns | 7367 |  | 209 | `xxhashbench/`: separate module, `replace ../`, and its deprecation TODO | 4.11 |  | 0.671 |
| ns | 7566 |  | 199 | `xxhashbench/`: the comparison table's shape and every hash it compares | 4.12 | 4.11 | 0.658 |
| walker |  | 7679 | 404 | Code::CodeKey { rung: Body, file: xxhash_test.go, decl: 4, sub: 0, line: 131 } |  |  | 0.658 |
| walker |  | 7849 | 170 | Code::CodeKey { rung: Body, file: xxhashbench/xxhashbench_test.go, decl: 1, sub: 1, line: 108 } |  |  | 0.658 |
| ns | 7899 |  | 333 | CI: the test matrix and the exact commands run | 4.13 |  | 0.645 |
| walker |  | 7994 | 145 | Code::CodeKey { rung: Body, file: xxhashbench/xxhashbench_test.go, decl: 1, sub: 2, line: 108 } |  |  | 0.645 |
| ns | 8030 |  | 131 | README: Compatibility section body | 4.14 | 1.8 | 0.647 |
| walker |  | 8159 | 165 | Code::CodeKey { rung: Body, file: xxhash_test.go, decl: 1, sub: 0, line: 12 } |  |  | 0.653 |
| walker |  | 8315 | 156 | Code::CodeKey { rung: Body, file: xxhash_test.go, decl: 1, sub: 1, line: 12 } |  |  | 0.664 |
| ns | 8336 |  | 306 | README: measured purego-vs-asm throughput table and how it was produced | 4.15 | 1.8 | 0.669 |
| ns | 8364 |  | 28 | License identification | 4.16 |  | 0.667 |
| walker |  | 8497 | 182 | Code::CodeKey { rung: Body, file: xxhash_test.go, decl: 1, sub: 2, line: 12 } |  |  | 0.668 |
| ns | 8514 |  | 150 | `xxhash_amd64.s`: build tags and both `TEXT` symbol definitions | 5.1 |  | 0.662 |
| walker |  | 8652 | 155 | Code::CodeKey { rung: Body, file: xxhash_test.go, decl: 1, sub: 3, line: 12 } |  |  | 0.662 |
| ns | 8664 |  | 150 | `xxhash_arm64.s`: build tags and both `TEXT` symbol definitions | 5.2 |  | 0.655 |
| ns | 8821 |  | 157 | `xxhash_amd64.s`: the complete register-allocation map | 5.3 | 5.1 | 0.648 |
| walker |  | 8987 | 335 | Plaintext::Rest { file: xxhashbench/go.sum } |  |  | 0.648 |
| ns | 9036 |  | 215 | `xxhash_arm64.s`: the complete register-allocation map | 5.4 | 5.2 | 0.639 |
| ns | 9176 |  | 140 | `xxhash_amd64.s`: `round` and `round0` macro bodies | 5.5 | 5.3 | 0.633 |
| ns | 9348 |  | 172 | `xxhash_amd64.s`: `mergeRound` and `blockLoop` macros | 5.6 | 5.5 | 0.627 |
| ns | 9400 |  | 52 | `xxhash_arm64.s`: the complete macro roster | 5.7 | 5.4 | 0.624 |
| walker |  | 9516 | 529 | Plaintext::Rest { file: dynamic/plugin.go } |  |  | 0.636 |
| walker |  | 9993 | 477 | Plaintext::Rest { file: xxhash_arm64.s } |  |  | 0.657 |
