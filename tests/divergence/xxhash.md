Score(3000)=0.733 I=0.827 C=0.649 ns_rows≤3K=24/60 grid(1000/1442/2080/3000/4327/6240/9000)=0.733/0.662/0.722/0.733/0.614/0.731/0.762

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
| walker |  | 163 | 31 | GoMod::Identity { file: go.mod } |  |  | 0.000 |
| walker |  | 163 | 0 | GoMod::File { file: go.mod } |  |  | 0.000 |
| ns | 198 |  | 85 | Complete root directory listing | 1.3 |  | 0.531 |
| walker |  | 202 | 39 | Code::CodeKey { rung: ModuleDoc, file: xxhash.go, decl: 0, sub: 0, line: 0 } |  |  | 0.763 |
| walker |  | 237 | 35 | GoMod::Identity { file: xxhashbench/go.mod } |  |  | 0.763 |
| ns | 275 |  | 77 | README: the complete public API block | 1.4 |  | 0.646 |
| walker |  | 303 | 66 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.795 |
| walker |  | 334 | 31 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.800 |
| ns | 359 |  | 84 | README: Digest's key methods and hash.Hash64 conformance | 1.5 |  | 0.713 |
| ns | 435 |  | 76 | README: pure-Go vs assembly, and the `purego` tag | 1.6 |  | 0.665 |
| ns | 479 |  | 44 | All subdirectory listings (complete) | 1.7 |  | 0.696 |
| ns | 508 |  | 29 | README: remaining H2 section headings | 1.8 |  | 0.703 |
| ns | 539 |  | 31 | Root `go.mod`: module path and Go floor | 1.9 |  | 0.709 |
| walker |  | 579 | 245 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.934 |
| ns | 681 |  | 142 | `Digest` doc comment and all seven struct fields | 2.1 |  | 0.832 |
| walker |  | 798 | 219 | Code::CodeKey { rung: Names, file: xxhash.go, decl: 0, sub: 0, line: 0 } |  |  | 0.835 |
| walker |  | 809 | 11 | Code::CodeKey { rung: Decl, file: xxhash.go, decl: 1, sub: 0, line: 11 } |  |  | 0.836 |
| walker |  | 820 | 11 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 4, sub: 0, line: 40 } |  |  | 0.836 |
| walker |  | 831 | 11 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 6, sub: 0, line: 53 } |  |  | 0.836 |
| walker |  | 843 | 12 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 8, sub: 0, line: 69 } |  |  | 0.837 |
| ns | 849 |  | 168 | `xxhash_asm.go` in full: build tags + the two assembly-backed declarations | 2.2 |  | 0.749 |
| walker |  | 922 | 79 | Code::CodeKey { rung: Decl, file: xxhash.go, decl: 3, sub: 0, line: 29 } |  |  | 0.778 |
| walker |  | 935 | 13 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 4, sub: 0, line: 40 } |  |  | 0.779 |
| walker |  | 952 | 17 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 5, sub: 0, line: 45 } |  |  | 0.780 |
| ns | 970 |  | 121 | `xxhash_other.go`: complementary build tags + pure-Go `Sum64`/`writeBlocks` signatures | 2.3 |  | 0.732 |
| walker |  | 983 | 31 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 6, sub: 0, line: 53 } |  |  | 0.733 |
| walker |  | 1018 | 35 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 7, sub: 0, line: 59 } |  |  | 0.735 |
| walker |  | 1069 | 51 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 3, sub: 0, line: 29 } |  |  | 0.797 |
| walker |  | 1122 | 53 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 2, sub: 0, line: 23 } |  |  | 0.801 |
| ns | 1252 |  | 282 | `xxhash_unsafe.go`: `!appengine` tag, `Sum64String`/`WriteString` signatures, `sliceHeader` | 2.4 |  | 0.704 |
| walker |  | 1354 | 232 | Code::CodeKey { rung: Names, file: xxhash.go, decl: 0, sub: 1, line: 0 } |  |  | 0.707 |
| walker |  | 1365 | 11 | Code::CodeKey { rung: Decl, file: xxhash.go, decl: 13, sub: 0, line: 170 } |  |  | 0.708 |
| walker |  | 1376 | 11 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 9, sub: 0, line: 72 } |  |  | 0.709 |
| walker |  | 1388 | 12 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 12, sub: 0, line: 129 } |  |  | 0.709 |
| walker |  | 1402 | 14 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 14, sub: 0, line: 176 } |  |  | 0.710 |
| walker |  | 1420 | 18 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 10, sub: 0, line: 75 } |  |  | 0.711 |
| ns | 1432 |  | 180 | `xxhash_safe.go` in full: the `appengine` fallbacks | 2.5 |  | 0.661 |
| walker |  | 1438 | 18 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 15, sub: 0, line: 190 } |  |  | 0.662 |
| walker |  | 1457 | 19 | Code::CodeKey { rung: Doc, file: xxhash.go, decl: 11, sub: 0, line: 113 } |  |  | 0.663 |
| ns | 1541 |  | 109 | Constructors: `New` and `NewWithSeed`, with bodies | 2.6 |  | 0.651 |
| ns | 1718 |  | 177 | `Reset`, `ResetWithSeed`, `Size`, `BlockSize` signatures | 2.7 |  | 0.663 |
| walker |  | 1721 | 264 | Code::CodeKey { rung: Names, file: xxhash.go, decl: 0, sub: 2, line: 0 } |  |  | 0.667 |
| walker |  | 1787 | 66 | Code::CodeKey { rung: Names, file: xxhash_unsafe.go, decl: 0, sub: 0, line: 0 } |  |  | 0.671 |
| walker |  | 1806 | 19 | Code::CodeKey { rung: Decl, file: xxhash_unsafe.go, decl: 3, sub: 0, line: 55 } |  |  | 0.674 |
| ns | 1830 |  | 112 | `Write`, `Sum`, `Sum64` doc comments + signatures | 2.8 |  | 0.674 |
| walker |  | 1837 | 31 | Code::CodeKey { rung: Body, file: xxhash_unsafe.go, decl: 1, sub: 0, line: 38 } |  |  | 0.676 |
| walker |  | 1878 | 41 | Code::CodeKey { rung: Doc, file: xxhash_unsafe.go, decl: 2, sub: 0, line: 45 } |  |  | 0.685 |
| walker |  | 1920 | 42 | Code::CodeKey { rung: Doc, file: xxhash_unsafe.go, decl: 3, sub: 0, line: 55 } |  |  | 0.698 |
| walker |  | 1964 | 44 | Code::CodeKey { rung: Doc, file: xxhash_unsafe.go, decl: 1, sub: 0, line: 38 } |  |  | 0.714 |
| ns | 2022 |  | 192 | The five XXH64 primes and the `primes` array | 2.9 |  | 0.722 |
| walker |  | 2100 | 136 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.723 |
| ns | 2145 |  | 123 | Marshaling: `magic`/`marshaledSize` constants + `MarshalBinary`/`UnmarshalBinary` signatures | 2.10 |  | 0.724 |
| walker |  | 2154 | 54 | Code::CodeKey { rung: Names, file: xxhash_safe.go, decl: 0, sub: 0, line: 0 } |  |  | 0.727 |
| walker |  | 2165 | 11 | Code::CodeKey { rung: Body, file: xxhash_safe.go, decl: 1, sub: 0, line: 9 } |  |  | 0.729 |
| walker |  | 2176 | 11 | Code::CodeKey { rung: Body, file: xxhash_safe.go, decl: 2, sub: 0, line: 14 } |  |  | 0.731 |
| walker |  | 2197 | 21 | Code::CodeKey { rung: Doc, file: xxhash_safe.go, decl: 2, sub: 0, line: 14 } |  |  | 0.736 |
| ns | 2217 |  | 72 | Roster: the byte-level helpers `appendUint64`, `consumeUint64`, `u64`, `u32` | 2.11 |  | 0.738 |
| walker |  | 2220 | 23 | Code::CodeKey { rung: Doc, file: xxhash_safe.go, decl: 1, sub: 0, line: 9 } |  |  | 0.744 |
| walker |  | 2281 | 61 | Code::CodeKey { rung: Names, file: xxhash_asm.go, decl: 0, sub: 0, line: 0 } |  |  | 0.749 |
| walker |  | 2288 | 7 | Code::CodeKey { rung: Doc, file: xxhash_asm.go, decl: 2, sub: 0, line: 15 } |  |  | 0.751 |
| walker |  | 2324 | 36 | Code::CodeKey { rung: Doc, file: xxhash_asm.go, decl: 1, sub: 0, line: 12 } |  |  | 0.763 |
| walker |  | 2387 | 63 | Code::CodeKey { rung: Names, file: xxhash_other.go, decl: 0, sub: 0, line: 0 } |  |  | 0.768 |
| walker |  | 2409 | 22 | Code::CodeKey { rung: Doc, file: xxhash_other.go, decl: 1, sub: 0, line: 7 } |  |  | 0.774 |
| ns | 2436 |  | 219 | Roster: `round`, `mergeRound`, and the complete `rol*` rotate family | 2.12 |  | 0.778 |
| ns | 2511 |  | 75 | `ResetWithSeed` body: how a seed becomes the four lanes | 3.1 | 2.7 | 0.763 |
| walker |  | 2556 | 147 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.763 |
| walker |  | 2602 | 46 | Code::CodeKey { rung: Names, file: xxhsum/xxhsum.go, decl: 0, sub: 0, line: 0 } |  |  | 0.763 |
| ns | 2616 |  | 105 | `round` and `mergeRound` bodies | 3.2 | 2.12 | 0.739 |
| ns | 2737 |  | 121 | `xxhash_unsafe.go`: the actual unsafe string-to-slice conversion | 3.3 | 2.4 | 0.724 |
| walker |  | 2838 | 236 | Code::CodeKey { rung: Decl, file: xxhsum/xxhsum.go, decl: 1, sub: 0, line: 11 } |  |  | 0.726 |
| walker |  | 2889 | 51 | Code::CodeKey { rung: Body, file: xxhsum/xxhsum.go, decl: 2, sub: 0, line: 34 } |  |  | 0.727 |
| walker |  | 2964 | 75 | Code::CodeKey { rung: Body, file: xxhsum/xxhsum.go, decl: 3, sub: 0, line: 43 } |  |  | 0.728 |
| ns | 3010 |  | 273 | `Write` body, part 1: buffering into `mem` and flushing a partial block | 3.4 | 2.8 | 0.692 |
| walker |  | 3048 | 84 | Code::CodeKey { rung: Body, file: xxhash_unsafe.go, decl: 2, sub: 0, line: 45 } |  |  | 0.711 |
| ns | 3119 |  | 109 | `Write` body, part 2: full blocks via `writeBlocks`, then store the remainder | 3.5 | 2.8 | 0.692 |
| walker |  | 3138 | 90 | Code::CodeKey { rung: Names, file: dynamic/plugin.go, decl: 0, sub: 0, line: 0 } |  |  | 0.693 |
| walker |  | 3147 | 9 | Code::CodeKey { rung: Decl, file: dynamic/plugin.go, decl: 1, sub: 0, line: 14 } |  |  | 0.693 |
| walker |  | 3203 | 56 | Code::CodeKey { rung: Body, file: dynamic/plugin.go, decl: 2, sub: 0, line: 19 } |  |  | 0.693 |
| walker |  | 3314 | 111 | Plaintext::DeclSurface { file: testall.sh } |  |  | 0.694 |
| walker |  | 3320 | 6 | Plaintext::Whole { file: testall.sh } |  |  | 0.695 |
| ns | 3328 |  | 209 | `Sum64` body, part 1: merging the four lanes (and the <32-byte shortcut) | 3.6 | 2.8 | 0.672 |
| walker |  | 3534 | 214 | Code::CodeKey { rung: Body, file: xxhash_other.go, decl: 2, sub: 0, line: 64 } |  |  | 0.676 |
| ns | 3593 |  | 265 | `Sum64` body, part 2: 8/4/1-byte tail loops and the final avalanche | 3.7 | 2.8 | 0.647 |
| walker |  | 3647 | 113 | GoMod::File { file: xxhashbench/go.mod } |  |  | 0.647 |
| ns | 3704 |  | 111 | `Sum` body: big-endian append of the 64-bit digest | 3.8 | 2.8 | 0.632 |
| ns | 3852 |  | 148 | `MarshalBinary` body: the serialized state layout | 3.9 | 2.10 | 0.620 |
| walker |  | 3958 | 311 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.622 |
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
| walker |  | 5144 | 72 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 7, sub: 0, line: 59 } |  |  | 0.693 |
| walker |  | 5252 | 108 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 11, sub: 0, line: 113 } |  |  | 0.713 |
| ns | 5393 |  | 344 | `xxhash_unsafe.go`: the inliner-cost design commentary | 3.16 | 2.4 | 0.693 |
| walker |  | 5397 | 145 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 14, sub: 0, line: 176 } |  |  | 0.710 |
| walker |  | 5443 | 46 | Code::CodeKey { rung: Names, file: xxhash_unsafe_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.710 |
| walker |  | 5483 | 40 | Code::CodeKey { rung: Doc, file: xxhash_unsafe_test.go, decl: 2, sub: 0, line: 31 } |  |  | 0.710 |
| ns | 5510 |  | 117 | `testall.sh` in full: the four tag/arch test combinations | 4.1 |  | 0.714 |
| walker |  | 5562 | 79 | Code::CodeKey { rung: Names, file: bench_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.714 |
| ns | 5608 |  | 98 | Roster of every function in `xxhash_test.go` | 4.2 |  | 0.706 |
| walker |  | 5650 | 88 | Code::CodeKey { rung: Decl, file: bench_test.go, decl: 1, sub: 0, line: 8 } |  |  | 0.708 |
| walker |  | 5854 | 204 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 15, sub: 0, line: 190 } |  |  | 0.730 |
| ns | 5936 |  | 328 | The complete golden-vector table in `TestAll` | 4.3 | 4.2 | 0.715 |
| walker |  | 6022 | 168 | Code::CodeKey { rung: Names, file: xxhash_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.727 |
| walker |  | 6082 | 60 | Code::CodeKey { rung: Body, file: xxhash_test.go, decl: 9, sub: 0, line: 193 } |  |  | 0.727 |
| ns | 6083 |  | 147 | `bench_test.go`: the input-size table and all four benchmark names | 4.4 |  | 0.731 |
| ns | 6175 |  | 92 | `xxhash_unsafe_test.go`: build tag and both test names | 4.5 |  | 0.731 |
| walker |  | 6182 | 100 | Code::CodeKey { rung: Body, file: xxhash_test.go, decl: 3, sub: 0, line: 88 } |  |  | 0.731 |
| walker |  | 6290 | 108 | Code::CodeKey { rung: Body, file: bench_test.go, decl: 3, sub: 0, line: 34 } |  |  | 0.731 |
| ns | 6298 |  | 123 | `TestInlining`: which functions must inline, and how it checks | 4.6 | 4.5 | 0.721 |
| walker |  | 6416 | 126 | Code::CodeKey { rung: Body, file: bench_test.go, decl: 5, sub: 0, line: 63 } |  |  | 0.721 |
| ns | 6520 |  | 222 | `xxhsum/`: the CLI's usage text and argument handling | 4.7 |  | 0.714 |
| walker |  | 6550 | 134 | Code::CodeKey { rung: Body, file: bench_test.go, decl: 2, sub: 0, line: 19 } |  |  | 0.714 |
| walker |  | 6701 | 151 | Code::CodeKey { rung: Body, file: xxhash_unsafe_test.go, decl: 1, sub: 0, line: 13 } |  |  | 0.714 |
| ns | 6790 |  | 270 | `xxhsum/`: the file loop, `contains`, and the output format | 4.8 | 4.7 | 0.718 |
| ns | 6928 |  | 138 | `dynamic/`: the `plugin` build-tag file and its two exported tests | 4.9 |  | 0.717 |
| walker |  | 7085 | 384 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 10, sub: 0, line: 75 } |  |  | 0.758 |
| walker |  | 7130 | 45 | Code::CodeKey { rung: Names, file: dynamic/dynamic_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.758 |
| ns | 7158 |  | 230 | `dynamic/dynamic_test.go`: building the plugin and calling into it | 4.10 |  | 0.742 |
| walker |  | 7297 | 167 | Code::CodeKey { rung: Body, file: xxhash_test.go, decl: 4, sub: 0, line: 97 } |  |  | 0.742 |
| ns | 7367 |  | 209 | `xxhashbench/`: separate module, `replace ../`, and its deprecation TODO | 4.11 |  | 0.737 |
| walker |  | 7472 | 175 | Code::CodeKey { rung: Body, file: xxhash_test.go, decl: 5, sub: 0, line: 114 } |  |  | 0.737 |
| ns | 7566 |  | 199 | `xxhashbench/`: the comparison table's shape and every hash it compares | 4.12 | 4.11 | 0.723 |
| ns | 7899 |  | 333 | CI: the test matrix and the exact commands run | 4.13 |  | 0.708 |
| walker |  | 7943 | 471 | Code::CodeKey { rung: Body, file: xxhash.go, decl: 12, sub: 0, line: 129 } |  |  | 0.750 |
| ns | 8030 |  | 131 | README: Compatibility section body | 4.14 | 1.8 | 0.751 |
| walker |  | 8040 | 97 | Code::CodeKey { rung: Names, file: xxhashbench/xxhashbench_test.go, decl: 0, sub: 0, line: 0 } |  |  | 0.753 |
| walker |  | 8087 | 47 | Code::CodeKey { rung: Body, file: xxhashbench/xxhashbench_test.go, decl: 4, sub: 0, line: 152 } |  |  | 0.753 |
| walker |  | 8134 | 47 | Code::CodeKey { rung: Body, file: xxhashbench/xxhashbench_test.go, decl: 5, sub: 0, line: 159 } |  |  | 0.753 |
| walker |  | 8286 | 152 | Code::CodeKey { rung: Body, file: bench_test.go, decl: 4, sub: 0, line: 46 } |  |  | 0.753 |
| ns | 8336 |  | 306 | README: measured purego-vs-asm throughput table and how it was produced | 4.15 | 1.8 | 0.756 |
| ns | 8364 |  | 28 | License identification | 4.16 |  | 0.754 |
| ns | 8514 |  | 150 | `xxhash_amd64.s`: build tags and both `TEXT` symbol definitions | 5.1 |  | 0.747 |
| walker |  | 8585 | 299 | Code::CodeKey { rung: Body, file: xxhash_unsafe_test.go, decl: 2, sub: 0, line: 31 } |  |  | 0.757 |
| ns | 8664 |  | 150 | `xxhash_arm64.s`: build tags and both `TEXT` symbol definitions | 5.2 |  | 0.750 |
| walker |  | 8732 | 147 | Code::CodeKey { rung: Body, file: dynamic/dynamic_test.go, decl: 1, sub: 0, line: 17 } |  |  | 0.754 |
| ns | 8821 |  | 157 | `xxhash_amd64.s`: the complete register-allocation map | 5.3 | 5.1 | 0.746 |
| walker |  | 8881 | 149 | Code::CodeKey { rung: Body, file: dynamic/dynamic_test.go, decl: 2, sub: 0, line: 33 } |  |  | 0.755 |
| ns | 9036 |  | 215 | `xxhash_arm64.s`: the complete register-allocation map | 5.4 | 5.2 | 0.744 |
| ns | 9176 |  | 140 | `xxhash_amd64.s`: `round` and `round0` macro bodies | 5.5 | 5.3 | 0.738 |
| ns | 9348 |  | 172 | `xxhash_amd64.s`: `mergeRound` and `blockLoop` macros | 5.6 | 5.5 | 0.731 |
| ns | 9400 |  | 52 | `xxhash_arm64.s`: the complete macro roster | 5.7 | 5.4 | 0.726 |
| walker |  | 9836 | 955 | Code::CodeKey { rung: Decl, file: xxhashbench/xxhashbench_test.go, decl: 2, sub: 0, line: 20 } |  |  | 0.742 |
| walker |  | 9998 | 162 | Code::CodeKey { rung: Body, file: xxhash_test.go, decl: 8, sub: 0, line: 170 } |  |  | 0.742 |
