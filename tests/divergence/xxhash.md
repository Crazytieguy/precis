Score(3000)=0.664 I=0.809 C=0.545 ns_rows≤3K=24/60 grid(1000/1442/2080/3000/4327/6240/9000)=0.733/0.659/0.671/0.664/0.674/0.704/0.669

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
| walker |  | 578 | 245 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.934 |
| ns | 681 |  | 142 | `Digest` doc comment and all seven struct fields | 2.1 |  | 0.832 |
| walker |  | 798 | 220 | Code::CodeKey { rung: Names, file: xxhash.go, decl: 0, sub: 0, line: 0 } |  |  | 0.834 |
| ns | 849 |  | 168 | `xxhash_asm.go` in full: build tags + the two assembly-backed declarations | 2.2 |  | 0.747 |
| walker |  | 877 | 79 | Code::CodeKey { rung: Decl, file: xxhash.go, decl: 1, sub: 0, line: 29 } |  |  | 0.776 |
| walker |  | 888 | 11 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 7, sub: 0, line: 72 } |  |  | 0.777 |
| walker |  | 900 | 12 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 6, sub: 0, line: 69 } |  |  | 0.777 |
| walker |  | 912 | 12 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 10, sub: 0, line: 129 } |  |  | 0.778 |
| walker |  | 925 | 13 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 2, sub: 0, line: 40 } |  |  | 0.778 |
| walker |  | 941 | 16 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 11, sub: 0, line: 176 } |  |  | 0.779 |
| walker |  | 958 | 17 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 3, sub: 0, line: 45 } |  |  | 0.780 |
| ns | 970 |  | 121 | `xxhash_other.go`: complementary build tags + pure-Go `Sum64`/`writeBlocks` signatures | 2.3 |  | 0.732 |
| walker |  | 976 | 18 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 8, sub: 0, line: 75 } |  |  | 0.732 |
| walker |  | 994 | 18 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 12, sub: 0, line: 190 } |  |  | 0.733 |
| walker |  | 1013 | 19 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 9, sub: 0, line: 113 } |  |  | 0.734 |
| walker |  | 1044 | 31 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 4, sub: 0, line: 53 } |  |  | 0.736 |
| walker |  | 1079 | 35 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 5, sub: 0, line: 59 } |  |  | 0.738 |
| walker |  | 1090 | 11 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 2, sub: 0, line: 40 } |  |  | 0.738 |
| walker |  | 1101 | 11 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 4, sub: 0, line: 53 } |  |  | 0.739 |
| walker |  | 1237 | 136 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.740 |
| ns | 1252 |  | 282 | `xxhash_unsafe.go`: `!appengine` tag, `Sum64String`/`WriteString` signatures, `sliceHeader` | 2.4 |  | 0.651 |
| walker |  | 1292 | 55 | Code::CodeKey { rung: Names, file: xxhash_unsafe.go, decl: 0, sub: 0, line: 0 } |  |  | 0.654 |
| walker |  | 1345 | 53 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 1, sub: 0, line: 29 } |  |  | 0.708 |
| ns | 1432 |  | 180 | `xxhash_safe.go` in full: the `appengine` fallbacks | 2.5 |  | 0.659 |
| walker |  | 1458 | 113 | GoMod::File { file: xxhashbench/go.mod } |  |  | 0.660 |
| walker |  | 1502 | 44 | Code::CodeKey { rung: Names, file: xxhash_other.go, decl: 0, sub: 0, line: 0 } |  |  | 0.663 |
| walker |  | 1524 | 22 | Code::CodeKey { rung: Doc, file: xxhash_other.go, decl: 1, sub: 0, line: 7 } |  |  | 0.669 |
| ns | 1541 |  | 109 | Constructors: `New` and `NewWithSeed`, with bodies | 2.6 |  | 0.656 |
| walker |  | 1568 | 44 | Code::CodeKey { rung: Names, file: xxhash_asm.go, decl: 0, sub: 0, line: 0 } |  |  | 0.659 |
| walker |  | 1577 | 9 | Code::CodeKey { rung: Decl, file: xxhash_asm.go, decl: 1, sub: 0, line: 11 } |  |  | 0.661 |
| walker |  | 1631 | 54 | Code::CodeKey { rung: Names, file: xxhash_safe.go, decl: 0, sub: 0, line: 0 } |  |  | 0.665 |
| walker |  | 1642 | 11 | Code::CodeKey { rung: Body, file: xxhash_safe.go, decl: 1, sub: 0, line: 9 } |  |  | 0.667 |
| walker |  | 1653 | 11 | Code::CodeKey { rung: Body, file: xxhash_safe.go, decl: 2, sub: 0, line: 14 } |  |  | 0.670 |
| walker |  | 1674 | 21 | Code::CodeKey { rung: Doc, file: xxhash_safe.go, decl: 2, sub: 0, line: 14 } |  |  | 0.676 |
| walker |  | 1697 | 23 | Code::CodeKey { rung: Doc, file: xxhash_safe.go, decl: 1, sub: 0, line: 9 } |  |  | 0.684 |
| ns | 1718 |  | 177 | `Reset`, `ResetWithSeed`, `Size`, `BlockSize` signatures | 2.7 |  | 0.692 |
| walker |  | 1724 | 27 | Code::CodeKey { rung: Doc, file: xxhash_asm.go, decl: 1, sub: 0, line: 11 } |  |  | 0.700 |
| ns | 1830 |  | 112 | `Write`, `Sum`, `Sum64` doc comments + signatures | 2.8 |  | 0.699 |
| walker |  | 1871 | 147 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.699 |
| walker |  | 1902 | 31 | Code::CodeKey { rung: Body, file: xxhash_unsafe.go, decl: 1, sub: 0, line: 38 } |  |  | 0.700 |
| walker |  | 1948 | 46 | Code::CodeKey { rung: Names, file: xxhsum/xxhsum.go, decl: 0, sub: 0, line: 0 } |  |  | 0.700 |
| ns | 2022 |  | 192 | The five XXH64 primes and the `primes` array | 2.9 |  | 0.670 |
| ns | 2145 |  | 123 | Marshaling: `magic`/`marshaledSize` constants + `MarshalBinary`/`UnmarshalBinary` signatures | 2.10 |  | 0.660 |
| walker |  | 2184 | 236 | Code::CodeKey { rung: Decl, file: xxhsum/xxhsum.go, decl: 1, sub: 0, line: 11 } |  |  | 0.662 |
| ns | 2217 |  | 72 | Roster: the byte-level helpers `appendUint64`, `consumeUint64`, `u64`, `u32` | 2.11 |  | 0.654 |
| walker |  | 2225 | 41 | Code::CodeKey { rung: Doc, file: xxhash_unsafe.go, decl: 2, sub: 0, line: 45 } |  |  | 0.659 |
| walker |  | 2251 | 26 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 3, sub: 0, line: 45 } |  |  | 0.673 |
| walker |  | 2295 | 44 | Code::CodeKey { rung: Doc, file: xxhash_unsafe.go, decl: 1, sub: 0, line: 38 } |  |  | 0.681 |
| walker |  | 2406 | 111 | Plaintext::DeclSurface { file: testall.sh } |  |  | 0.682 |
| walker |  | 2412 | 6 | Plaintext::Whole { file: testall.sh } |  |  | 0.682 |
| ns | 2436 |  | 219 | Roster: `round`, `mergeRound`, and the complete `rol*` rotate family | 2.12 |  | 0.663 |
| ns | 2511 |  | 75 | `ResetWithSeed` body: how a seed becomes the four lanes | 3.1 | 2.7 | 0.650 |
| ns | 2616 |  | 105 | `round` and `mergeRound` bodies | 3.2 | 2.12 | 0.628 |
| walker |  | 2723 | 311 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.630 |
| ns | 2737 |  | 121 | `xxhash_unsafe.go`: the actual unsafe string-to-slice conversion | 3.3 | 2.4 | 0.617 |
| walker |  | 2795 | 72 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 5, sub: 0, line: 59 } |  |  | 0.638 |
| walker |  | 2879 | 84 | Code::CodeKey { rung: Body, file: xxhash_unsafe.go, decl: 2, sub: 0, line: 45 } |  |  | 0.659 |
| walker |  | 2987 | 108 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 9, sub: 0, line: 113 } |  |  | 0.664 |
| ns | 3010 |  | 273 | `Write` body, part 1: buffering into `mem` and flushing a partial block | 3.4 | 2.8 | 0.631 |
| walker |  | 3038 | 51 | Code::CodeKey { rung: Body, file: xxhsum/xxhsum.go, decl: 2, sub: 0, line: 34 } |  |  | 0.631 |
| ns | 3119 |  | 109 | `Write` body, part 2: full blocks via `writeBlocks`, then store the remainder | 3.5 | 2.8 | 0.615 |
| walker |  | 3183 | 145 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 11, sub: 0, line: 176 } |  |  | 0.619 |
| walker |  | 3258 | 75 | Code::CodeKey { rung: Body, file: xxhsum/xxhsum.go, decl: 3, sub: 0, line: 43 } |  |  | 0.620 |
| ns | 3328 |  | 209 | `Sum64` body, part 1: merging the four lanes (and the <32-byte shortcut) | 3.6 | 2.8 | 0.600 |
| walker |  | 3462 | 204 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 12, sub: 0, line: 190 } |  |  | 0.605 |
| ns | 3593 |  | 265 | `Sum64` body, part 2: 8/4/1-byte tail loops and the final avalanche | 3.7 | 2.8 | 0.579 |
| ns | 3704 |  | 111 | `Sum` body: big-endian append of the 64-bit digest | 3.8 | 2.8 | 0.590 |
| ns | 3852 |  | 148 | `MarshalBinary` body: the serialized state layout | 3.9 | 2.10 | 0.599 |
| walker |  | 3859 | 397 | Code::CodeKey { rung: Body, file: xxhash_other.go, decl: 1, sub: 0, line: 7 } |  |  | 0.604 |
| ns | 4059 |  | 207 | `UnmarshalBinary` body: validation and both error strings | 3.10 | 2.10 | 0.616 |
| ns | 4150 |  | 91 | `appendUint64` / `consumeUint64` bodies | 3.11 | 2.11 | 0.606 |
| walker |  | 4243 | 384 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 8, sub: 0, line: 75 } |  |  | 0.673 |
| walker |  | 4289 | 46 | Code::CodeKey { rung: Names, file: xxhash_unsafe_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.674 |
| ns | 4367 |  | 217 | `xxhash_other.go`: the pure-Go `writeBlocks` body | 3.12 | 2.3 | 0.661 |
| walker |  | 4372 | 83 | Code::CodeKey { rung: Names, file: xxhash_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.661 |
| walker |  | 4439 | 67 | Code::CodeKey { rung: Names, file: bench_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.662 |
| ns | 4654 |  | 287 | `xxhash_other.go` `Sum64` body, part 1: why it is not `New/Write/Sum64`, and the block loop | 3.13 | 2.3 | 0.673 |
| walker |  | 4716 | 277 | Code::CodeKey { rung: Body, file: xxhash_other.go, decl: 1, sub: 1, line: 7 } |  |  | 0.677 |
| ns | 4784 |  | 130 | `xxhash_other.go` `Sum64`, part 2: lane merge, the small-input `prime5` branch, length mix | 3.14 | 2.3 | 0.681 |
| ns | 5049 |  | 265 | `xxhash_other.go` `Sum64`, part 3: tail loops and avalanche | 3.15 | 2.3 | 0.690 |
| walker |  | 5187 | 471 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 10, sub: 0, line: 129 } |  |  | 0.750 |
| walker |  | 5232 | 45 | Code::CodeKey { rung: Names, file: dynamic/dynamic_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.750 |
| walker |  | 5272 | 40 | Code::CodeKey { rung: Doc, file: xxhash_unsafe_test.go, decl: 2, sub: 0, line: 31 } |  |  | 0.751 |
| walker |  | 5289 | 17 | Code::CodeKey { rung: Names, file: xxhashbench/xxhashbench_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.751 |
| ns | 5393 |  | 344 | `xxhash_unsafe.go`: the inliner-cost design commentary | 3.16 | 2.4 | 0.730 |
| walker |  | 5397 | 108 | Code::CodeKey { rung: Body, file: bench_test.go, decl: 2, sub: 0, line: 34 } |  |  | 0.730 |
| ns | 5510 |  | 117 | `testall.sh` in full: the four tag/arch test combinations | 4.1 |  | 0.733 |
| walker |  | 5548 | 151 | Code::CodeKey { rung: Body, file: xxhash_unsafe_test.go, decl: 1, sub: 0, line: 13 } |  |  | 0.733 |
| ns | 5608 |  | 98 | Roster of every function in `xxhash_test.go` | 4.2 |  | 0.728 |
| walker |  | 5674 | 126 | Code::CodeKey { rung: Body, file: bench_test.go, decl: 4, sub: 0, line: 63 } |  |  | 0.728 |
| walker |  | 5841 | 167 | Code::CodeKey { rung: Body, file: xxhash_test.go, decl: 2, sub: 0, line: 97 } |  |  | 0.728 |
| ns | 5936 |  | 328 | The complete golden-vector table in `TestAll` | 4.3 | 4.2 | 0.712 |
| walker |  | 5975 | 134 | Code::CodeKey { rung: Body, file: bench_test.go, decl: 1, sub: 0, line: 19 } |  |  | 0.712 |
| ns | 6083 |  | 147 | `bench_test.go`: the input-size table and all four benchmark names | 4.4 |  | 0.703 |
| walker |  | 6150 | 175 | Code::CodeKey { rung: Body, file: xxhash_test.go, decl: 3, sub: 0, line: 114 } |  |  | 0.703 |
| ns | 6175 |  | 92 | `xxhash_unsafe_test.go`: build tag and both test names | 4.5 |  | 0.703 |
| walker |  | 6297 | 147 | Code::CodeKey { rung: Body, file: dynamic/dynamic_test.go, decl: 1, sub: 0, line: 17 } |  |  | 0.704 |
| ns | 6298 |  | 123 | `TestInlining`: which functions must inline, and how it checks | 4.6 | 4.5 | 0.694 |
| walker |  | 6449 | 152 | Code::CodeKey { rung: Body, file: bench_test.go, decl: 3, sub: 0, line: 46 } |  |  | 0.694 |
| ns | 6520 |  | 222 | `xxhsum/`: the CLI's usage text and argument handling | 4.7 |  | 0.688 |
| walker |  | 6748 | 299 | Code::CodeKey { rung: Body, file: xxhash_unsafe_test.go, decl: 2, sub: 0, line: 31 } |  |  | 0.703 |
| ns | 6790 |  | 270 | `xxhsum/`: the file loop, `contains`, and the output format | 4.8 | 4.7 | 0.706 |
| walker |  | 6897 | 149 | Code::CodeKey { rung: Body, file: dynamic/dynamic_test.go, decl: 2, sub: 0, line: 33 } |  |  | 0.708 |
| ns | 6928 |  | 138 | `dynamic/`: the `plugin` build-tag file and its two exported tests | 4.9 |  | 0.698 |
| walker |  | 7157 | 260 | Code::CodeKey { rung: Body, file: xxhash_test.go, decl: 5, sub: 0, line: 170 } |  |  | 0.698 |
| ns | 7158 |  | 230 | `dynamic/dynamic_test.go`: building the plugin and calling into it | 4.10 |  | 0.697 |
| ns | 7367 |  | 209 | `xxhashbench/`: separate module, `replace ../`, and its deprecation TODO | 4.11 |  | 0.693 |
| ns | 7566 |  | 199 | `xxhashbench/`: the comparison table's shape and every hash it compares | 4.12 | 4.11 | 0.680 |
| walker |  | 7679 | 522 | Code::CodeKey { rung: Body, file: xxhashbench/xxhashbench_test.go, decl: 1, sub: 0, line: 108 } |  |  | 0.680 |
| ns | 7899 |  | 333 | CI: the test matrix and the exact commands run | 4.13 |  | 0.666 |
| ns | 8030 |  | 131 | README: Compatibility section body | 4.14 | 1.8 | 0.669 |
| walker |  | 8083 | 404 | Code::CodeKey { rung: Body, file: xxhash_test.go, decl: 4, sub: 0, line: 131 } |  |  | 0.669 |
| ns | 8336 |  | 306 | README: measured purego-vs-asm throughput table and how it was produced | 4.15 | 1.8 | 0.674 |
| ns | 8364 |  | 28 | License identification | 4.16 |  | 0.672 |
| ns | 8514 |  | 150 | `xxhash_amd64.s`: build tags and both `TEXT` symbol definitions | 5.1 |  | 0.666 |
| ns | 8664 |  | 150 | `xxhash_arm64.s`: build tags and both `TEXT` symbol definitions | 5.2 |  | 0.659 |
| walker |  | 8741 | 658 | Code::CodeKey { rung: Body, file: xxhash_test.go, decl: 1, sub: 0, line: 12 } |  |  | 0.676 |
| ns | 8821 |  | 157 | `xxhash_amd64.s`: the complete register-allocation map | 5.3 | 5.1 | 0.669 |
| ns | 9036 |  | 215 | `xxhash_arm64.s`: the complete register-allocation map | 5.4 | 5.2 | 0.659 |
| walker |  | 9076 | 335 | Plaintext::Rest { file: xxhashbench/go.sum } |  |  | 0.659 |
| ns | 9176 |  | 140 | `xxhash_amd64.s`: `round` and `round0` macro bodies | 5.5 | 5.3 | 0.654 |
| ns | 9348 |  | 172 | `xxhash_amd64.s`: `mergeRound` and `blockLoop` macros | 5.6 | 5.5 | 0.647 |
| ns | 9400 |  | 52 | `xxhash_arm64.s`: the complete macro roster | 5.7 | 5.4 | 0.643 |
| walker |  | 9605 | 529 | Plaintext::Rest { file: dynamic/plugin.go } |  |  | 0.655 |
| walker |  | 9996 | 391 | Plaintext::Rest { file: xxhash_arm64.s } |  |  | 0.674 |
