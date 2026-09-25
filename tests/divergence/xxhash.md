Score(3000)=0.679 I=0.792 C=0.582 ns_rows≤3K=24/60 grid(1000/1442/2080/3000/4327/6240/9000)=0.574/0.507/0.533/0.679/0.582/0.718/0.741

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
| ns | 275 |  | 77 | README: the complete public API block | 1.4 |  | 0.646 |
| ns | 359 |  | 84 | README: Digest's key methods and hash.Hash64 conformance | 1.5 |  | 0.575 |
| walker |  | 421 | 219 | Code::CodeKey { rung: Names, file: xxhash.go, decl: 0, sub: 0, line: 0 } |  |  | 0.578 |
| walker |  | 432 | 11 | Code::CodeKey { rung: Decl, file: xxhash.go, decl: 1, sub: 0, line: 11 } |  |  | 0.579 |
| ns | 435 |  | 76 | README: pure-Go vs assembly, and the `purego` tag | 1.6 |  | 0.539 |
| walker |  | 443 | 11 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 4, sub: 0, line: 40 } |  |  | 0.540 |
| walker |  | 454 | 11 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 6, sub: 0, line: 53 } |  |  | 0.540 |
| walker |  | 466 | 12 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 8, sub: 0, line: 69 } |  |  | 0.540 |
| ns | 479 |  | 44 | All subdirectory listings (complete) | 1.7 |  | 0.580 |
| ns | 508 |  | 29 | README: remaining H2 section headings | 1.8 |  | 0.562 |
| ns | 539 |  | 31 | Root `go.mod`: module path and Go floor | 1.9 |  | 0.572 |
| walker |  | 545 | 79 | Code::CodeKey { rung: Decl, file: xxhash.go, decl: 3, sub: 0, line: 29 } |  |  | 0.577 |
| walker |  | 580 | 35 | Code::CodeKey { rung: Names, file: xxhash_asm.go, decl: 0, sub: 0, line: 0 } |  |  | 0.578 |
| walker |  | 617 | 37 | Code::CodeKey { rung: Names, file: xxhash_other.go, decl: 0, sub: 0, line: 0 } |  |  | 0.578 |
| walker |  | 630 | 13 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 4, sub: 0, line: 40 } |  |  | 0.579 |
| walker |  | 673 | 43 | Code::CodeKey { rung: Names, file: xxhash_safe.go, decl: 0, sub: 0, line: 0 } |  |  | 0.580 |
| ns | 681 |  | 142 | `Digest` doc comment and all seven struct fields | 2.1 |  | 0.546 |
| walker |  | 739 | 66 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.645 |
| walker |  | 770 | 31 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.673 |
| walker |  | 824 | 54 | Code::CodeKey { rung: Names, file: xxhash_unsafe.go, decl: 0, sub: 0, line: 0 } |  |  | 0.674 |
| walker |  | 841 | 17 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 5, sub: 0, line: 45 } |  |  | 0.675 |
| ns | 849 |  | 168 | `xxhash_asm.go` in full: build tags + the two assembly-backed declarations | 2.2 |  | 0.607 |
| walker |  | 860 | 19 | Code::CodeKey { rung: Decl, file: xxhash_unsafe.go, decl: 3, sub: 0, line: 55 } |  |  | 0.608 |
| walker |  | 895 | 35 | GoMod::Identity { file: xxhashbench/go.mod } |  |  | 0.608 |
| walker |  | 906 | 11 | Code::CodeKey { rung: Body, file: xxhash_safe.go, decl: 1, sub: 0, line: 9 } |  |  | 0.608 |
| walker |  | 917 | 11 | Code::CodeKey { rung: Body, file: xxhash_safe.go, decl: 2, sub: 0, line: 14 } |  |  | 0.608 |
| ns | 970 |  | 121 | `xxhash_other.go`: complementary build tags + pure-Go `Sum64`/`writeBlocks` signatures | 2.3 |  | 0.574 |
| walker |  | 1149 | 232 | Code::CodeKey { rung: Names, file: xxhash.go, decl: 0, sub: 1, line: 0 } |  |  | 0.577 |
| walker |  | 1160 | 11 | Code::CodeKey { rung: Decl, file: xxhash.go, decl: 13, sub: 0, line: 170 } |  |  | 0.578 |
| walker |  | 1171 | 11 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 9, sub: 0, line: 72 } |  |  | 0.578 |
| walker |  | 1217 | 46 | Code::CodeKey { rung: Names, file: xxhsum/xxhsum.go, decl: 0, sub: 0, line: 0 } |  |  | 0.578 |
| walker |  | 1229 | 12 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 12, sub: 0, line: 129 } |  |  | 0.579 |
| walker |  | 1250 | 21 | Code::CodeKey { rung: Doc, file: xxhash_safe.go, decl: 2, sub: 0, line: 14 } |  |  | 0.579 |
| ns | 1252 |  | 282 | `xxhash_unsafe.go`: `!appengine` tag, `Sum64String`/`WriteString` signatures, `sliceHeader` | 2.4 |  | 0.516 |
| walker |  | 1272 | 22 | Code::CodeKey { rung: Doc, file: xxhash_other.go, decl: 1, sub: 0, line: 7 } |  |  | 0.522 |
| walker |  | 1295 | 23 | Code::CodeKey { rung: Doc, file: xxhash_safe.go, decl: 1, sub: 0, line: 9 } |  |  | 0.523 |
| ns | 1432 |  | 180 | `xxhash_safe.go` in full: the `appengine` fallbacks | 2.5 |  | 0.507 |
| ns | 1541 |  | 109 | Constructors: `New` and `NewWithSeed`, with bodies | 2.6 |  | 0.503 |
| walker |  | 1559 | 264 | Code::CodeKey { rung: Names, file: xxhash.go, decl: 0, sub: 2, line: 0 } |  |  | 0.507 |
| walker |  | 1566 | 7 | Code::CodeKey { rung: Doc, file: xxhash_asm.go, decl: 2, sub: 0, line: 15 } |  |  | 0.509 |
| walker |  | 1646 | 80 | Code::CodeKey { rung: Names, file: dynamic/plugin.go, decl: 0, sub: 0, line: 0 } |  |  | 0.510 |
| walker |  | 1655 | 9 | Code::CodeKey { rung: Decl, file: dynamic/plugin.go, decl: 1, sub: 0, line: 14 } |  |  | 0.510 |
| walker |  | 1686 | 31 | Code::CodeKey { rung: Body, file: xxhash_unsafe.go, decl: 1, sub: 0, line: 38 } |  |  | 0.511 |
| ns | 1718 |  | 177 | `Reset`, `ResetWithSeed`, `Size`, `BlockSize` signatures | 2.7 |  | 0.503 |
| walker |  | 1722 | 36 | Code::CodeKey { rung: Doc, file: xxhash_asm.go, decl: 1, sub: 0, line: 12 } |  |  | 0.514 |
| walker |  | 1736 | 14 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 14, sub: 0, line: 176 } |  |  | 0.515 |
| ns | 1830 |  | 112 | `Write`, `Sum`, `Sum64` doc comments + signatures | 2.8 |  | 0.508 |
| walker |  | 1853 | 117 | Plaintext::Whole { file: testall.sh } |  |  | 0.509 |
| walker |  | 1894 | 41 | Code::CodeKey { rung: Doc, file: xxhash_unsafe.go, decl: 2, sub: 0, line: 45 } |  |  | 0.518 |
| walker |  | 1938 | 44 | Code::CodeKey { rung: Doc, file: xxhash_unsafe.go, decl: 1, sub: 0, line: 38 } |  |  | 0.531 |
| ns | 2022 |  | 192 | The five XXH64 primes and the `primes` array | 2.9 |  | 0.526 |
| walker |  | 2051 | 113 | GoMod::File { file: xxhashbench/go.mod } |  |  | 0.527 |
| walker |  | 2069 | 18 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 10, sub: 0, line: 75 } |  |  | 0.533 |
| ns | 2145 |  | 123 | Marshaling: `magic`/`marshaledSize` constants + `MarshalBinary`/`UnmarshalBinary` signatures | 2.10 |  | 0.535 |
| ns | 2217 |  | 72 | Roster: the byte-level helpers `appendUint64`, `consumeUint64`, `u64`, `u32` | 2.11 |  | 0.542 |
| walker |  | 2314 | 245 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.643 |
| walker |  | 2332 | 18 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 15, sub: 0, line: 190 } |  |  | 0.652 |
| walker |  | 2351 | 19 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 11, sub: 0, line: 113 } |  |  | 0.659 |
| walker |  | 2402 | 51 | Code::CodeKey { rung: Body, file: xxhsum/xxhsum.go, decl: 2, sub: 0, line: 34 } |  |  | 0.660 |
| ns | 2436 |  | 219 | Roster: `round`, `mergeRound`, and the complete `rol*` rotate family | 2.12 |  | 0.670 |
| walker |  | 2486 | 84 | Code::CodeKey { rung: Body, file: xxhash_unsafe.go, decl: 2, sub: 0, line: 45 } |  |  | 0.674 |
| ns | 2511 |  | 75 | `ResetWithSeed` body: how a seed becomes the four lanes | 3.1 | 2.7 | 0.661 |
| walker |  | 2512 | 26 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 5, sub: 0, line: 45 } |  |  | 0.673 |
| walker |  | 2568 | 56 | Code::CodeKey { rung: Body, file: dynamic/plugin.go, decl: 2, sub: 0, line: 19 } |  |  | 0.674 |
| ns | 2616 |  | 105 | `round` and `mergeRound` bodies | 3.2 | 2.12 | 0.653 |
| walker |  | 2704 | 136 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.654 |
| walker |  | 2735 | 31 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 6, sub: 0, line: 53 } |  |  | 0.663 |
| ns | 2737 |  | 121 | `xxhash_unsafe.go`: the actual unsafe string-to-slice conversion | 3.3 | 2.4 | 0.666 |
| walker |  | 2810 | 75 | Code::CodeKey { rung: Body, file: xxhsum/xxhsum.go, decl: 3, sub: 0, line: 43 } |  |  | 0.666 |
| walker |  | 2845 | 35 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 7, sub: 0, line: 59 } |  |  | 0.679 |
| walker |  | 2992 | 147 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.679 |
| ns | 3010 |  | 273 | `Write` body, part 1: buffering into `mem` and flushing a partial block | 3.4 | 2.8 | 0.645 |
| walker |  | 3034 | 42 | Code::CodeKey { rung: Doc, file: xxhash_unsafe.go, decl: 3, sub: 0, line: 55 } |  |  | 0.656 |
| walker |  | 3085 | 51 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 3, sub: 0, line: 29 } |  |  | 0.686 |
| ns | 3119 |  | 109 | `Write` body, part 2: full blocks via `writeBlocks`, then store the remainder | 3.5 | 2.8 | 0.668 |
| ns | 3328 |  | 209 | `Sum64` body, part 1: merging the four lanes (and the <32-byte shortcut) | 3.6 | 2.8 | 0.647 |
| walker |  | 3396 | 311 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.648 |
| walker |  | 3430 | 34 | Code::CodeKey { rung: Names, file: xxhash_unsafe_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.648 |
| ns | 3593 |  | 265 | `Sum64` body, part 2: 8/4/1-byte tail loops and the final avalanche | 3.7 | 2.8 | 0.620 |
| walker |  | 3663 | 233 | Code::CodeKey { rung: Body, file: xxhsum/xxhsum.go, decl: 1, sub: 0, line: 11 } |  |  | 0.623 |
| walker |  | 3684 | 21 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 17, sub: 0, line: 214 } |  |  | 0.623 |
| ns | 3704 |  | 111 | `Sum` body: big-endian append of the 64-bit digest | 3.8 | 2.8 | 0.608 |
| ns | 3852 |  | 148 | `MarshalBinary` body: the serialized state layout | 3.9 | 2.10 | 0.597 |
| ns | 4059 |  | 207 | `UnmarshalBinary` body: validation and both error strings | 3.10 | 2.10 | 0.581 |
| walker |  | 4081 | 397 | Code::CodeKey { rung: Body, file: xxhash_other.go, decl: 1, sub: 0, line: 7 } |  |  | 0.587 |
| ns | 4150 |  | 91 | `appendUint64` / `consumeUint64` bodies | 3.11 | 2.11 | 0.582 |
| walker |  | 4357 | 276 | Code::CodeKey { rung: Body, file: dynamic/plugin.go, decl: 3, sub: 0, line: 26 } |  |  | 0.582 |
| ns | 4367 |  | 217 | `xxhash_other.go`: the pure-Go `writeBlocks` body | 3.12 | 2.3 | 0.571 |
| walker |  | 4429 | 72 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 7, sub: 0, line: 59 } |  |  | 0.585 |
| walker |  | 4643 | 214 | Code::CodeKey { rung: Body, file: xxhash_other.go, decl: 2, sub: 0, line: 64 } |  |  | 0.608 |
| ns | 4654 |  | 287 | `xxhash_other.go` `Sum64` body, part 1: why it is not `New/Write/Sum64`, and the block loop | 3.13 | 2.3 | 0.624 |
| walker |  | 4751 | 108 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 11, sub: 0, line: 113 } |  |  | 0.647 |
| walker |  | 4783 | 32 | Code::CodeKey { rung: Names, file: dynamic/dynamic_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.647 |
| ns | 4784 |  | 130 | `xxhash_other.go` `Sum64`, part 2: lane merge, the small-input `prime5` branch, length mix | 3.14 | 2.3 | 0.648 |
| ns | 5049 |  | 265 | `xxhash_other.go` `Sum64`, part 3: tail loops and avalanche | 3.15 | 2.3 | 0.628 |
| walker |  | 5060 | 277 | Code::CodeKey { rung: Body, file: xxhash_other.go, decl: 1, sub: 1, line: 7 } |  |  | 0.667 |
| walker |  | 5139 | 79 | Code::CodeKey { rung: Names, file: bench_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.667 |
| walker |  | 5176 | 37 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 20, sub: 0, line: 222 } |  |  | 0.672 |
| walker |  | 5215 | 39 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 16, sub: 0, line: 208 } |  |  | 0.679 |
| walker |  | 5256 | 41 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 21, sub: 0, line: 229 } |  |  | 0.689 |
| ns | 5393 |  | 344 | `xxhash_unsafe.go`: the inliner-cost design commentary | 3.16 | 2.4 | 0.670 |
| walker |  | 5401 | 145 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 14, sub: 0, line: 176 } |  |  | 0.686 |
| ns | 5510 |  | 117 | `testall.sh` in full: the four tag/arch test combinations | 4.1 |  | 0.690 |
| walker |  | 5569 | 168 | Code::CodeKey { rung: Names, file: xxhash_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.692 |
| ns | 5608 |  | 98 | Roster of every function in `xxhash_test.go` | 4.2 |  | 0.695 |
| walker |  | 5622 | 53 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 2, sub: 0, line: 23 } |  |  | 0.706 |
| walker |  | 5630 | 8 | Plaintext::Whole { file: dynamic/.gitignore } |  |  | 0.706 |
| walker |  | 5727 | 97 | Code::CodeKey { rung: Names, file: xxhashbench/xxhashbench_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.707 |
| walker |  | 5767 | 40 | Code::CodeKey { rung: Doc, file: xxhash_unsafe_test.go, decl: 2, sub: 0, line: 31 } |  |  | 0.707 |
| walker |  | 5776 | 9 | Plaintext::Whole { file: xxhsum/.gitignore } |  |  | 0.707 |
| ns | 5936 |  | 328 | The complete golden-vector table in `TestAll` | 4.3 | 4.2 | 0.692 |
| walker |  | 5980 | 204 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 15, sub: 0, line: 190 } |  |  | 0.714 |
| walker |  | 6068 | 88 | Code::CodeKey { rung: Decl, file: bench_test.go, decl: 1, sub: 0, line: 8 } |  |  | 0.715 |
| ns | 6083 |  | 147 | `bench_test.go`: the input-size table and all four benchmark names | 4.4 |  | 0.720 |
| ns | 6175 |  | 92 | `xxhash_unsafe_test.go`: build tag and both test names | 4.5 |  | 0.718 |
| ns | 6298 |  | 123 | `TestInlining`: which functions must inline, and how it checks | 4.6 | 4.5 | 0.709 |
| walker |  | 6452 | 384 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 10, sub: 0, line: 75 } |  |  | 0.754 |
| ns | 6520 |  | 222 | `xxhsum/`: the CLI's usage text and argument handling | 4.7 |  | 0.747 |
| walker |  | 6560 | 108 | Code::CodeKey { rung: Body, file: bench_test.go, decl: 3, sub: 0, line: 34 } |  |  | 0.747 |
| ns | 6790 |  | 270 | `xxhsum/`: the file loop, `contains`, and the output format | 4.8 | 4.7 | 0.747 |
| ns | 6928 |  | 138 | `dynamic/`: the `plugin` build-tag file and its two exported tests | 4.9 |  | 0.744 |
| walker |  | 7031 | 471 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 12, sub: 0, line: 129 } |  |  | 0.789 |
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
