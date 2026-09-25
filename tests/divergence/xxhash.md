Score(3000)=0.742 I=0.826 C=0.666 ns_rows≤3K=24/60 grid(1000/1442/2080/3000/4327/6240/9000)=0.650/0.567/0.634/0.742/0.594/0.753/0.741

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 47 |  | 47 | Package doc lede | 1.1 |  | 0.000 |
| walker |  | 85 | 85 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 96 | 11 | Fs::DirListing { dir: xxhsum } |  |  | 0.000 |
| walker |  | 110 | 14 | Fs::DirListing { dir: dynamic } |  |  | 0.000 |
| ns | 113 |  | 66 | README title + positioning sentence | 1.2 |  | 0.000 |
| walker |  | 125 | 15 | Fs::DirListing { dir: xxhashbench } |  |  | 0.000 |
| walker |  | 128 | 3 | Fs::DirListing { dir: .github } |  |  | 0.000 |
| walker |  | 132 | 4 | Fs::DirListing { dir: .github/workflows } |  |  | 0.000 |
| walker |  | 171 | 39 | Code::CodeKey { rung: ModuleDoc, file: xxhash.go, decl: 0, sub: 0, line: 0 } |  |  | 0.502 |
| ns | 198 |  | 85 | Complete root directory listing | 1.3 |  | 0.756 |
| walker |  | 202 | 31 | GoMod::Identity { file: go.mod } |  |  | 0.763 |
| walker |  | 202 | 0 | GoMod::File { file: go.mod } |  |  | 0.763 |
| walker |  | 268 | 66 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.939 |
| ns | 275 |  | 77 | README: the complete public API block | 1.4 |  | 0.795 |
| walker |  | 299 | 31 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.800 |
| walker |  | 334 | 35 | GoMod::Identity { file: xxhashbench/go.mod } |  |  | 0.800 |
| ns | 359 |  | 84 | README: Digest's key methods and hash.Hash64 conformance | 1.5 |  | 0.713 |
| ns | 435 |  | 76 | README: pure-Go vs assembly, and the `purego` tag | 1.6 |  | 0.665 |
| ns | 479 |  | 44 | All subdirectory listings (complete) | 1.7 |  | 0.696 |
| ns | 508 |  | 29 | README: remaining H2 section headings | 1.8 |  | 0.703 |
| ns | 539 |  | 31 | Root `go.mod`: module path and Go floor | 1.9 |  | 0.709 |
| walker |  | 553 | 219 | Code::CodeKey { rung: Names, file: xxhash.go, decl: 0, sub: 0, line: 0 } |  |  | 0.712 |
| walker |  | 564 | 11 | Code::CodeKey { rung: Decl, file: xxhash.go, decl: 1, sub: 0, line: 11 } |  |  | 0.713 |
| walker |  | 575 | 11 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 4, sub: 0, line: 40 } |  |  | 0.713 |
| walker |  | 586 | 11 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 6, sub: 0, line: 53 } |  |  | 0.713 |
| walker |  | 598 | 12 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 8, sub: 0, line: 69 } |  |  | 0.714 |
| walker |  | 677 | 79 | Code::CodeKey { rung: Decl, file: xxhash.go, decl: 3, sub: 0, line: 29 } |  |  | 0.719 |
| ns | 681 |  | 142 | `Digest` doc comment and all seven struct fields | 2.1 |  | 0.672 |
| walker |  | 690 | 13 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 4, sub: 0, line: 40 } |  |  | 0.672 |
| walker |  | 707 | 17 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 5, sub: 0, line: 45 } |  |  | 0.673 |
| walker |  | 738 | 31 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 6, sub: 0, line: 53 } |  |  | 0.675 |
| walker |  | 773 | 35 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 7, sub: 0, line: 59 } |  |  | 0.677 |
| walker |  | 824 | 51 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 3, sub: 0, line: 29 } |  |  | 0.756 |
| ns | 849 |  | 168 | `xxhash_asm.go` in full: build tags + the two assembly-backed declarations | 2.2 |  | 0.677 |
| walker |  | 859 | 35 | Code::CodeKey { rung: Names, file: xxhash_asm.go, decl: 0, sub: 0, line: 0 } |  |  | 0.680 |
| walker |  | 896 | 37 | Code::CodeKey { rung: Names, file: xxhash_other.go, decl: 0, sub: 0, line: 0 } |  |  | 0.680 |
| walker |  | 918 | 22 | Code::CodeKey { rung: Doc, file: xxhash_other.go, decl: 1, sub: 0, line: 7 } |  |  | 0.681 |
| walker |  | 961 | 43 | Code::CodeKey { rung: Names, file: xxhash_safe.go, decl: 0, sub: 0, line: 0 } |  |  | 0.682 |
| ns | 970 |  | 121 | `xxhash_other.go`: complementary build tags + pure-Go `Sum64`/`writeBlocks` signatures | 2.3 |  | 0.649 |
| walker |  | 972 | 11 | Code::CodeKey { rung: Body, file: xxhash_safe.go, decl: 1, sub: 0, line: 9 } |  |  | 0.649 |
| walker |  | 983 | 11 | Code::CodeKey { rung: Body, file: xxhash_safe.go, decl: 2, sub: 0, line: 14 } |  |  | 0.650 |
| walker |  | 1004 | 21 | Code::CodeKey { rung: Doc, file: xxhash_safe.go, decl: 2, sub: 0, line: 14 } |  |  | 0.650 |
| walker |  | 1027 | 23 | Code::CodeKey { rung: Doc, file: xxhash_safe.go, decl: 1, sub: 0, line: 9 } |  |  | 0.651 |
| walker |  | 1034 | 7 | Code::CodeKey { rung: Doc, file: xxhash_asm.go, decl: 2, sub: 0, line: 15 } |  |  | 0.654 |
| walker |  | 1088 | 54 | Code::CodeKey { rung: Names, file: xxhash_unsafe.go, decl: 0, sub: 0, line: 0 } |  |  | 0.654 |
| walker |  | 1107 | 19 | Code::CodeKey { rung: Decl, file: xxhash_unsafe.go, decl: 3, sub: 0, line: 55 } |  |  | 0.655 |
| walker |  | 1138 | 31 | Code::CodeKey { rung: Body, file: xxhash_unsafe.go, decl: 1, sub: 0, line: 38 } |  |  | 0.655 |
| ns | 1252 |  | 282 | `xxhash_unsafe.go`: `!appengine` tag, `Sum64String`/`WriteString` signatures, `sliceHeader` | 2.4 |  | 0.583 |
| walker |  | 1370 | 232 | Code::CodeKey { rung: Names, file: xxhash.go, decl: 0, sub: 1, line: 0 } |  |  | 0.586 |
| walker |  | 1381 | 11 | Code::CodeKey { rung: Decl, file: xxhash.go, decl: 13, sub: 0, line: 170 } |  |  | 0.586 |
| walker |  | 1392 | 11 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 9, sub: 0, line: 72 } |  |  | 0.587 |
| walker |  | 1404 | 12 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 12, sub: 0, line: 129 } |  |  | 0.588 |
| walker |  | 1418 | 14 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 14, sub: 0, line: 176 } |  |  | 0.588 |
| ns | 1432 |  | 180 | `xxhash_safe.go` in full: the `appengine` fallbacks | 2.5 |  | 0.566 |
| walker |  | 1436 | 18 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 10, sub: 0, line: 75 } |  |  | 0.567 |
| walker |  | 1454 | 18 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 15, sub: 0, line: 190 } |  |  | 0.568 |
| walker |  | 1473 | 19 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 11, sub: 0, line: 113 } |  |  | 0.569 |
| ns | 1541 |  | 109 | Constructors: `New` and `NewWithSeed`, with bodies | 2.6 |  | 0.562 |
| ns | 1718 |  | 177 | `Reset`, `ResetWithSeed`, `Size`, `BlockSize` signatures | 2.7 |  | 0.578 |
| walker |  | 1737 | 264 | Code::CodeKey { rung: Names, file: xxhash.go, decl: 0, sub: 2, line: 0 } |  |  | 0.583 |
| walker |  | 1773 | 36 | Code::CodeKey { rung: Doc, file: xxhash_asm.go, decl: 1, sub: 0, line: 12 } |  |  | 0.594 |
| ns | 1830 |  | 112 | `Write`, `Sum`, `Sum64` doc comments + signatures | 2.8 |  | 0.597 |
| walker |  | 1890 | 117 | Plaintext::Whole { file: testall.sh } |  |  | 0.598 |
| walker |  | 1943 | 53 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 2, sub: 0, line: 23 } |  |  | 0.601 |
| walker |  | 1984 | 41 | Code::CodeKey { rung: Doc, file: xxhash_unsafe.go, decl: 2, sub: 0, line: 45 } |  |  | 0.610 |
| ns | 2022 |  | 192 | The five XXH64 primes and the `primes` array | 2.9 |  | 0.623 |
| walker |  | 2028 | 44 | Code::CodeKey { rung: Doc, file: xxhash_unsafe.go, decl: 1, sub: 0, line: 38 } |  |  | 0.634 |
| walker |  | 2141 | 113 | GoMod::File { file: xxhashbench/go.mod } |  |  | 0.635 |
| ns | 2145 |  | 123 | Marshaling: `magic`/`marshaledSize` constants + `MarshalBinary`/`UnmarshalBinary` signatures | 2.10 |  | 0.639 |
| walker |  | 2187 | 46 | Code::CodeKey { rung: Names, file: xxhsum/xxhsum.go, decl: 0, sub: 0, line: 0 } |  |  | 0.639 |
| ns | 2217 |  | 72 | Roster: the byte-level helpers `appendUint64`, `consumeUint64`, `u64`, `u32` | 2.11 |  | 0.643 |
| walker |  | 2432 | 245 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.743 |
| ns | 2436 |  | 219 | Roster: `round`, `mergeRound`, and the complete `rol*` rotate family | 2.12 |  | 0.748 |
| ns | 2511 |  | 75 | `ResetWithSeed` body: how a seed becomes the four lanes | 3.1 | 2.7 | 0.734 |
| walker |  | 2512 | 80 | Code::CodeKey { rung: Names, file: dynamic/plugin.go, decl: 0, sub: 0, line: 0 } |  |  | 0.734 |
| walker |  | 2521 | 9 | Code::CodeKey { rung: Decl, file: dynamic/plugin.go, decl: 1, sub: 0, line: 14 } |  |  | 0.735 |
| walker |  | 2572 | 51 | Code::CodeKey { rung: Body, file: xxhsum/xxhsum.go, decl: 2, sub: 0, line: 34 } |  |  | 0.735 |
| ns | 2616 |  | 105 | `round` and `mergeRound` bodies | 3.2 | 2.12 | 0.712 |
| walker |  | 2656 | 84 | Code::CodeKey { rung: Body, file: xxhash_unsafe.go, decl: 2, sub: 0, line: 45 } |  |  | 0.716 |
| walker |  | 2712 | 56 | Code::CodeKey { rung: Body, file: dynamic/plugin.go, decl: 2, sub: 0, line: 19 } |  |  | 0.716 |
| ns | 2737 |  | 121 | `xxhash_unsafe.go`: the actual unsafe string-to-slice conversion | 3.3 | 2.4 | 0.717 |
| walker |  | 2848 | 136 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.718 |
| walker |  | 2890 | 42 | Code::CodeKey { rung: Doc, file: xxhash_unsafe.go, decl: 3, sub: 0, line: 55 } |  |  | 0.730 |
| walker |  | 2965 | 75 | Code::CodeKey { rung: Body, file: xxhsum/xxhsum.go, decl: 3, sub: 0, line: 43 } |  |  | 0.731 |
| walker |  | 2991 | 26 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 5, sub: 0, line: 45 } |  |  | 0.742 |
| ns | 3010 |  | 273 | `Write` body, part 1: buffering into `mem` and flushing a partial block | 3.4 | 2.8 | 0.705 |
| ns | 3119 |  | 109 | `Write` body, part 2: full blocks via `writeBlocks`, then store the remainder | 3.5 | 2.8 | 0.686 |
| walker |  | 3138 | 147 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.686 |
| ns | 3328 |  | 209 | `Sum64` body, part 1: merging the four lanes (and the <32-byte shortcut) | 3.6 | 2.8 | 0.664 |
| walker |  | 3449 | 311 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.666 |
| ns | 3593 |  | 265 | `Sum64` body, part 2: 8/4/1-byte tail loops and the final avalanche | 3.7 | 2.8 | 0.637 |
| walker |  | 3682 | 233 | Code::CodeKey { rung: Body, file: xxhsum/xxhsum.go, decl: 1, sub: 0, line: 11 } |  |  | 0.640 |
| ns | 3704 |  | 111 | `Sum` body: big-endian append of the 64-bit digest | 3.8 | 2.8 | 0.625 |
| ns | 3852 |  | 148 | `MarshalBinary` body: the serialized state layout | 3.9 | 2.10 | 0.613 |
| ns | 4059 |  | 207 | `UnmarshalBinary` body: validation and both error strings | 3.10 | 2.10 | 0.596 |
| walker |  | 4079 | 397 | Code::CodeKey { rung: Body, file: xxhash_other.go, decl: 1, sub: 0, line: 7 } |  |  | 0.602 |
| ns | 4150 |  | 91 | `appendUint64` / `consumeUint64` bodies | 3.11 | 2.11 | 0.594 |
| walker |  | 4355 | 276 | Code::CodeKey { rung: Body, file: dynamic/plugin.go, decl: 3, sub: 0, line: 26 } |  |  | 0.594 |
| ns | 4367 |  | 217 | `xxhash_other.go`: the pure-Go `writeBlocks` body | 3.12 | 2.3 | 0.583 |
| walker |  | 4376 | 21 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 17, sub: 0, line: 214 } |  |  | 0.586 |
| walker |  | 4590 | 214 | Code::CodeKey { rung: Body, file: xxhash_other.go, decl: 2, sub: 0, line: 64 } |  |  | 0.608 |
| ns | 4654 |  | 287 | `xxhash_other.go` `Sum64` body, part 1: why it is not `New/Write/Sum64`, and the block loop | 3.13 | 2.3 | 0.624 |
| ns | 4784 |  | 130 | `xxhash_other.go` `Sum64`, part 2: lane merge, the small-input `prime5` branch, length mix | 3.14 | 2.3 | 0.626 |
| walker |  | 4867 | 277 | Code::CodeKey { rung: Body, file: xxhash_other.go, decl: 1, sub: 1, line: 7 } |  |  | 0.634 |
| walker |  | 4939 | 72 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 7, sub: 0, line: 59 } |  |  | 0.646 |
| walker |  | 5047 | 108 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 11, sub: 0, line: 113 } |  |  | 0.669 |
| ns | 5049 |  | 265 | `xxhash_other.go` `Sum64`, part 3: tail loops and avalanche | 3.15 | 2.3 | 0.679 |
| walker |  | 5084 | 37 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 20, sub: 0, line: 222 } |  |  | 0.684 |
| walker |  | 5118 | 34 | Code::CodeKey { rung: Names, file: xxhash_unsafe_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.684 |
| walker |  | 5157 | 39 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 16, sub: 0, line: 208 } |  |  | 0.691 |
| walker |  | 5198 | 41 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 21, sub: 0, line: 229 } |  |  | 0.701 |
| walker |  | 5343 | 145 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 14, sub: 0, line: 176 } |  |  | 0.718 |
| walker |  | 5351 | 8 | Plaintext::Whole { file: dynamic/.gitignore } |  |  | 0.718 |
| walker |  | 5391 | 40 | Code::CodeKey { rung: Doc, file: xxhash_unsafe_test.go, decl: 2, sub: 0, line: 31 } |  |  | 0.718 |
| ns | 5393 |  | 344 | `xxhash_unsafe.go`: the inliner-cost design commentary | 3.16 | 2.4 | 0.698 |
| walker |  | 5400 | 9 | Plaintext::Whole { file: xxhsum/.gitignore } |  |  | 0.698 |
| ns | 5510 |  | 117 | `testall.sh` in full: the four tag/arch test combinations | 4.1 |  | 0.702 |
| walker |  | 5604 | 204 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 15, sub: 0, line: 190 } |  |  | 0.725 |
| ns | 5608 |  | 98 | Roster of every function in `xxhash_test.go` | 4.2 |  | 0.717 |
| walker |  | 5636 | 32 | Code::CodeKey { rung: Names, file: dynamic/dynamic_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.717 |
| walker |  | 5715 | 79 | Code::CodeKey { rung: Names, file: bench_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.717 |
| walker |  | 5803 | 88 | Code::CodeKey { rung: Decl, file: bench_test.go, decl: 1, sub: 0, line: 8 } |  |  | 0.719 |
| ns | 5936 |  | 328 | The complete golden-vector table in `TestAll` | 4.3 | 4.2 | 0.703 |
| ns | 6083 |  | 147 | `bench_test.go`: the input-size table and all four benchmark names | 4.4 |  | 0.708 |
| ns | 6175 |  | 92 | `xxhash_unsafe_test.go`: build tag and both test names | 4.5 |  | 0.707 |
| walker |  | 6187 | 384 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 10, sub: 0, line: 75 } |  |  | 0.753 |
| ns | 6298 |  | 123 | `TestInlining`: which functions must inline, and how it checks | 4.6 | 4.5 | 0.743 |
| walker |  | 6355 | 168 | Code::CodeKey { rung: Names, file: xxhash_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.754 |
| walker |  | 6463 | 108 | Code::CodeKey { rung: Body, file: bench_test.go, decl: 3, sub: 0, line: 34 } |  |  | 0.754 |
| ns | 6520 |  | 222 | `xxhsum/`: the CLI's usage text and argument handling | 4.7 |  | 0.746 |
| ns | 6790 |  | 270 | `xxhsum/`: the file loop, `contains`, and the output format | 4.8 | 4.7 | 0.747 |
| ns | 6928 |  | 138 | `dynamic/`: the `plugin` build-tag file and its two exported tests | 4.9 |  | 0.744 |
| walker |  | 6934 | 471 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 12, sub: 0, line: 129 } |  |  | 0.789 |
| walker |  | 7031 | 97 | Code::CodeKey { rung: Names, file: xxhashbench/xxhashbench_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.789 |
| walker |  | 7157 | 126 | Code::CodeKey { rung: Body, file: bench_test.go, decl: 5, sub: 0, line: 63 } |  |  | 0.789 |
| ns | 7158 |  | 230 | `dynamic/dynamic_test.go`: building the plugin and calling into it | 4.10 |  | 0.772 |
| walker |  | 7291 | 134 | Code::CodeKey { rung: Body, file: bench_test.go, decl: 2, sub: 0, line: 19 } |  |  | 0.772 |
| ns | 7367 |  | 209 | `xxhashbench/`: separate module, `replace ../`, and its deprecation TODO | 4.11 |  | 0.767 |
| walker |  | 7442 | 151 | Code::CodeKey { rung: Body, file: xxhash_unsafe_test.go, decl: 1, sub: 0, line: 13 } |  |  | 0.767 |
| ns | 7566 |  | 199 | `xxhashbench/`: the comparison table's shape and every hash it compares | 4.12 | 4.11 | 0.754 |
| walker |  | 7609 | 167 | Code::CodeKey { rung: Body, file: xxhash_test.go, decl: 4, sub: 0, line: 97 } |  |  | 0.754 |
| walker |  | 7784 | 175 | Code::CodeKey { rung: Body, file: xxhash_test.go, decl: 5, sub: 0, line: 114 } |  |  | 0.754 |
| walker |  | 7844 | 60 | Code::CodeKey { rung: Body, file: xxhash_test.go, decl: 9, sub: 0, line: 193 } |  |  | 0.754 |
| ns | 7899 |  | 333 | CI: the test matrix and the exact commands run | 4.13 |  | 0.738 |
| walker |  | 7996 | 152 | Code::CodeKey { rung: Body, file: bench_test.go, decl: 4, sub: 0, line: 46 } |  |  | 0.738 |
| ns | 8030 |  | 131 | README: Compatibility section body | 4.14 | 1.8 | 0.740 |
| walker |  | 8256 | 260 | Code::CodeKey { rung: Body, file: xxhash_test.go, decl: 8, sub: 0, line: 170 } |  |  | 0.740 |
| ns | 8336 |  | 306 | README: measured purego-vs-asm throughput table and how it was produced | 4.15 | 1.8 | 0.743 |
| ns | 8364 |  | 28 | License identification | 4.16 |  | 0.742 |
| ns | 8514 |  | 150 | `xxhash_amd64.s`: build tags and both `TEXT` symbol definitions | 5.1 |  | 0.734 |
| walker |  | 8555 | 299 | Code::CodeKey { rung: Body, file: xxhash_unsafe_test.go, decl: 2, sub: 0, line: 31 } |  |  | 0.745 |
| ns | 8664 |  | 150 | `xxhash_arm64.s`: build tags and both `TEXT` symbol definitions | 5.2 |  | 0.738 |
| walker |  | 8702 | 147 | Code::CodeKey { rung: Body, file: dynamic/dynamic_test.go, decl: 1, sub: 0, line: 17 } |  |  | 0.741 |
| ns | 8821 |  | 157 | `xxhash_amd64.s`: the complete register-allocation map | 5.3 | 5.1 | 0.733 |
| walker |  | 8851 | 149 | Code::CodeKey { rung: Body, file: dynamic/dynamic_test.go, decl: 2, sub: 0, line: 33 } |  |  | 0.741 |
| walker |  | 8898 | 47 | Code::CodeKey { rung: Body, file: xxhashbench/xxhashbench_test.go, decl: 4, sub: 0, line: 152 } |  |  | 0.741 |
| walker |  | 8945 | 47 | Code::CodeKey { rung: Body, file: xxhashbench/xxhashbench_test.go, decl: 5, sub: 0, line: 159 } |  |  | 0.741 |
| ns | 9036 |  | 215 | `xxhash_arm64.s`: the complete register-allocation map | 5.4 | 5.2 | 0.731 |
| ns | 9176 |  | 140 | `xxhash_amd64.s`: `round` and `round0` macro bodies | 5.5 | 5.3 | 0.725 |
| walker |  | 9257 | 312 | Plaintext::Whole { file: LICENSE.txt } |  |  | 0.727 |
| ns | 9348 |  | 172 | `xxhash_amd64.s`: `mergeRound` and `blockLoop` macros | 5.6 | 5.5 | 0.720 |
| walker |  | 9357 | 100 | Code::CodeKey { rung: Body, file: xxhash_test.go, decl: 3, sub: 0, line: 88 } |  |  | 0.720 |
| ns | 9400 |  | 52 | `xxhash_arm64.s`: the complete macro roster | 5.7 | 5.4 | 0.716 |
| walker |  | 9761 | 404 | Code::CodeKey { rung: Body, file: xxhash_test.go, decl: 6, sub: 0, line: 131 } |  |  | 0.716 |
