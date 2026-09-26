Score(3000)=0.590 I=0.638 C=0.546 ns_rows≤3K=15/45 grid(1000/1442/2080/3000/4327/6240/9000)=0.756/0.723/0.642/0.590/0.551/0.494/0.499

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 98 |  | 98 | Package identity: module path, Go version, framework version | 1.1 |  | 0.000 |
| ns | 186 |  | 88 | README lede: what Gin is and what it is for | 1.2 |  | 0.000 |
| walker |  | 229 | 229 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 240 | 11 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.000 |
| walker |  | 244 | 4 | Fs::DirListing { dir: docs } |  |  | 0.000 |
| walker |  | 251 | 7 | Fs::DirListing { dir: internal } |  |  | 0.000 |
| walker |  | 266 | 15 | Fs::DirListing { dir: ginS } |  |  | 0.000 |
| walker |  | 281 | 15 | Fs::DirListing { dir: testdata } |  |  | 0.000 |
| walker |  | 289 | 8 | Fs::DirListing { dir: testdata/certificate } |  |  | 0.000 |
| walker |  | 320 | 31 | GoMod::Identity { file: go.mod } |  |  | 0.164 |
| ns | 324 |  | 138 | doc.go: the canonical hello-world call site | 1.3 | 1.1 | 0.087 |
| walker |  | 329 | 9 | Fs::DirListing { dir: internal/fs } |  |  | 0.088 |
| walker |  | 338 | 9 | Fs::DirListing { dir: testdata/protoexample } |  |  | 0.088 |
| walker |  | 348 | 10 | Fs::DirListing { dir: testdata/template } |  |  | 0.089 |
| walker |  | 359 | 11 | Fs::DirListing { dir: internal/bytesconv } |  |  | 0.090 |
| walker |  | 363 | 4 | Fs::DirListing { dir: examples } |  |  | 0.090 |
| walker |  | 386 | 23 | Fs::DirListing { dir: codec/json } |  |  | 0.092 |
| ns | 444 |  | 120 | README key-feature list (first half) | 1.4 |  | 0.079 |
| walker |  | 461 | 75 | Fs::DirListing { dir: render } |  |  | 0.080 |
| walker |  | 480 | 19 | Fs::DirListing { dir: .github } |  |  | 0.080 |
| walker |  | 502 | 22 | Fs::DirListing { dir: .github/workflows } |  |  | 0.080 |
| ns | 569 |  | 125 | README key-feature list (second half) + Go version prerequisite | 1.5 |  | 0.072 |
| walker |  | 658 | 156 | Fs::DirListing { dir: binding } |  |  | 0.081 |
| ns | 661 |  | 92 | docs/doc.md top-level section map | 1.6 |  | 0.070 |
| walker |  | 859 | 201 | Code::CodeKey { rung: ModuleDoc, file: doc.go, decl: 0, sub: 0, line: 0 } |  |  | 0.603 |
| ns | 890 |  | 229 | Complete repository root listing | 1.7 |  | 0.756 |
| walker |  | 973 | 114 | Plaintext::Whole { file: Makefile } |  |  | 0.756 |
| ns | 1121 |  | 231 | binding/ and render/ directory listings | 1.8 |  | 0.765 |
| walker |  | 1168 | 195 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.765 |
| ns | 1236 |  | 115 | Remaining source/data directory listings (codec, ginS, internal, docs, examples, testdata) | 1.9 |  | 0.765 |
| ns | 1434 |  | 198 | Core public type vocabulary: HandlerFunc, OptionFunc, HandlersChain, RouteInfo | 2.1 |  | 0.723 |
| walker |  | 1541 | 373 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.723 |
| ns | 1684 |  | 250 | Engine struct: complete exported configuration field roster | 2.2 |  | 0.655 |
| walker |  | 1769 | 228 | Code::CodeKey { rung: Names, file: gin.go, decl: 0, sub: 0, line: 0 } |  |  | 0.659 |
| walker |  | 1809 | 40 | Code::CodeKey { rung: Decl, file: gin.go, decl: 5, sub: 0, line: 68 } |  |  | 0.666 |
| walker |  | 1906 | 97 | Code::CodeKey { rung: Decl, file: gin.go, decl: 7, sub: 0, line: 79 } |  |  | 0.666 |
| walker |  | 1912 | 6 | Code::CodeKey { rung: Doc, file: gin.go, decl: 7, sub: 0, line: 79 } |  |  | 0.666 |
| walker |  | 1923 | 11 | Code::CodeKey { rung: Doc, file: gin.go, decl: 6, sub: 0, line: 76 } |  |  | 0.669 |
| walker |  | 1935 | 12 | Code::CodeKey { rung: Doc, file: gin.go, decl: 3, sub: 0, line: 57 } |  |  | 0.672 |
| walker |  | 1949 | 14 | Code::CodeKey { rung: Doc, file: gin.go, decl: 2, sub: 0, line: 54 } |  |  | 0.678 |
| walker |  | 1967 | 18 | Code::CodeKey { rung: Doc, file: gin.go, decl: 1, sub: 0, line: 51 } |  |  | 0.685 |
| walker |  | 1986 | 19 | Code::CodeKey { rung: Doc, file: gin.go, decl: 10, sub: 0, line: 236 } |  |  | 0.685 |
| ns | 2020 |  | 336 | IRoutes / IRouter: the complete route-registration interface | 2.3 |  | 0.642 |
| walker |  | 2157 | 171 | Code::CodeKey { rung: Names, file: gin.go, decl: 0, sub: 1, line: 0 } |  |  | 0.643 |
| walker |  | 2172 | 15 | Code::CodeKey { rung: Doc, file: gin.go, decl: 17, sub: 0, line: 312 } |  |  | 0.643 |
| walker |  | 2190 | 18 | Code::CodeKey { rung: Doc, file: gin.go, decl: 18, sub: 0, line: 321 } |  |  | 0.643 |
| ns | 2441 |  | 421 | Engine constructor New(): every default value | 2.4 | 2.2 | 0.601 |
| walker |  | 2468 | 278 | Code::CodeKey { rung: Names, file: gin.go, decl: 0, sub: 2, line: 0 } |  |  | 0.603 |
| walker |  | 2482 | 14 | Code::CodeKey { rung: Doc, file: gin.go, decl: 31, sub: 0, line: 662 } |  |  | 0.603 |
| walker |  | 2500 | 18 | Code::CodeKey { rung: Doc, file: gin.go, decl: 22, sub: 0, line: 348 } |  |  | 0.603 |
| walker |  | 2520 | 20 | Code::CodeKey { rung: Doc, file: gin.go, decl: 12, sub: 0, line: 259 } |  |  | 0.603 |
| walker |  | 2540 | 20 | Code::CodeKey { rung: Doc, file: gin.go, decl: 13, sub: 0, line: 265 } |  |  | 0.603 |
| walker |  | 2560 | 20 | Code::CodeKey { rung: Doc, file: gin.go, decl: 20, sub: 0, line: 332 } |  |  | 0.603 |
| walker |  | 2582 | 22 | Code::CodeKey { rung: Doc, file: gin.go, decl: 5, sub: 0, line: 68 } |  |  | 0.611 |
| walker |  | 2604 | 22 | Code::CodeKey { rung: Doc, file: gin.go, decl: 19, sub: 0, line: 326 } |  |  | 0.611 |
| walker |  | 2627 | 23 | Code::CodeKey { rung: Doc, file: gin.go, decl: 4, sub: 0, line: 60 } |  |  | 0.611 |
| ns | 2634 |  | 193 | Default(), RouterGroup struct, and the IRouter assertions | 2.5 | 2.3 | 0.591 |
| ns | 2829 |  | 195 | Run* server entry points: all six transports | 2.6 |  | 0.589 |
| walker |  | 2935 | 308 | GoMod::File { file: go.mod } |  |  | 0.589 |
| walker |  | 3119 | 184 | Code::CodeKey { rung: Names, file: routergroup.go, decl: 0, sub: 0, line: 0 } |  |  | 0.590 |
| walker |  | 3136 | 17 | Code::CodeKey { rung: Decl, file: routergroup.go, decl: 3, sub: 0, line: 55 } |  |  | 0.592 |
| walker |  | 3164 | 28 | Code::CodeKey { rung: Decl, file: routergroup.go, decl: 1, sub: 0, line: 27 } |  |  | 0.593 |
| ns | 3333 |  | 504 | gin.go: complete function roster beyond the constructors and Run* | 2.7 | 2.5 | 0.569 |
| walker |  | 3409 | 245 | Code::CodeKey { rung: Decl, file: routergroup.go, decl: 2, sub: 0, line: 33 } |  |  | 0.604 |
| walker |  | 3420 | 11 | Code::CodeKey { rung: Doc, file: routergroup.go, decl: 2, sub: 0, line: 33 } |  |  | 0.608 |
| walker |  | 3438 | 18 | Code::CodeKey { rung: Doc, file: routergroup.go, decl: 1, sub: 0, line: 27 } |  |  | 0.614 |
| walker |  | 3457 | 19 | Code::CodeKey { rung: Doc, file: routergroup.go, decl: 4, sub: 0, line: 65 } |  |  | 0.614 |
| walker |  | 3476 | 19 | Code::CodeKey { rung: Doc, file: routergroup.go, decl: 8, sub: 0, line: 111 } |  |  | 0.614 |
| ns | 3489 |  | 156 | Engine struct: unexported field tail | 2.8 | 2.2 | 0.600 |
| walker |  | 3495 | 19 | Code::CodeKey { rung: Doc, file: routergroup.go, decl: 9, sub: 0, line: 116 } |  |  | 0.600 |
| ns | 3637 |  | 148 | tree.go: Param / Params — the URL-parameter type and its accessors | 2.9 |  | 0.589 |
| walker |  | 3700 | 205 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 0, line: 0 } |  |  | 0.589 |
| walker |  | 3705 | 5 | Code::CodeKey { rung: Decl, file: context.go, decl: 1, sub: 0, line: 31 } |  |  | 0.589 |
| walker |  | 3720 | 15 | Code::CodeKey { rung: Doc, file: context.go, decl: 1, sub: 0, line: 31 } |  |  | 0.589 |
| ns | 3816 |  | 179 | handleHTTPRequest: path resolution before the tree lookup | 2.10 | 2.7 | 0.574 |
| walker |  | 3922 | 202 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 1, line: 0 } |  |  | 0.575 |
| walker |  | 4070 | 148 | Code::CodeKey { rung: Decl, file: context.go, decl: 6, sub: 0, line: 61 } |  |  | 0.575 |
| walker |  | 4081 | 11 | Code::CodeKey { rung: Doc, file: context.go, decl: 10, sub: 0, line: 167 } |  |  | 0.575 |
| walker |  | 4094 | 13 | Code::CodeKey { rung: Doc, file: context.go, decl: 2, sub: 0, line: 47 } |  |  | 0.575 |
| walker |  | 4109 | 15 | Code::CodeKey { rung: Doc, file: context.go, decl: 3, sub: 0, line: 50 } |  |  | 0.575 |
| walker |  | 4126 | 17 | Code::CodeKey { rung: Doc, file: context.go, decl: 13, sub: 0, line: 199 } |  |  | 0.575 |
| ns | 4189 |  | 373 | handleHTTPRequest: the radix lookup, redirects and the 404/405 exits | 2.11 | 2.10 | 0.549 |
| walker |  | 4296 | 170 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 2, line: 0 } |  |  | 0.551 |
| ns | 4438 |  | 249 | tree.go: nodeValue and the complete radix-tree function roster | 2.12 | 2.9 | 0.536 |
| walker |  | 4475 | 179 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 3, line: 0 } |  |  | 0.536 |
| walker |  | 4493 | 18 | Code::CodeKey { rung: Doc, file: context.go, decl: 23, sub: 0, line: 311 } |  |  | 0.536 |
| walker |  | 4511 | 18 | Code::CodeKey { rung: Doc, file: context.go, decl: 24, sub: 0, line: 316 } |  |  | 0.536 |
| walker |  | 4529 | 18 | Code::CodeKey { rung: Doc, file: context.go, decl: 25, sub: 0, line: 321 } |  |  | 0.536 |
| walker |  | 4548 | 19 | Code::CodeKey { rung: Doc, file: context.go, decl: 30, sub: 0, line: 346 } |  |  | 0.536 |
| walker |  | 4569 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 26, sub: 0, line: 326 } |  |  | 0.536 |
| walker |  | 4590 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 27, sub: 0, line: 331 } |  |  | 0.536 |
| walker |  | 4611 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 28, sub: 0, line: 336 } |  |  | 0.536 |
| walker |  | 4632 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 29, sub: 0, line: 341 } |  |  | 0.536 |
| ns | 4658 |  | 220 | path.go: cleanPath semantics and the file's whole function set | 2.13 | 2.11 | 0.528 |
| walker |  | 4817 | 185 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 4, line: 0 } |  |  | 0.528 |
| walker |  | 4834 | 17 | Code::CodeKey { rung: Doc, file: context.go, decl: 37, sub: 0, line: 381 } |  |  | 0.528 |
| walker |  | 4852 | 18 | Code::CodeKey { rung: Doc, file: context.go, decl: 38, sub: 0, line: 386 } |  |  | 0.528 |
| walker |  | 4870 | 18 | Code::CodeKey { rung: Doc, file: context.go, decl: 39, sub: 0, line: 391 } |  |  | 0.528 |
| ns | 4888 |  | 230 | Trusted-platform constants and the SetTrustedProxies contract | 2.14 | 2.7 | 0.521 |
| walker |  | 4890 | 20 | Code::CodeKey { rung: Doc, file: context.go, decl: 35, sub: 0, line: 371 } |  |  | 0.521 |
| walker |  | 4910 | 20 | Code::CodeKey { rung: Doc, file: context.go, decl: 36, sub: 0, line: 376 } |  |  | 0.521 |
| ns | 4977 |  | 89 | Makefile: complete target roster | 3.1 |  | 0.515 |
| ns | 5035 |  | 58 | Makefile variables: which packages are tested and vetted | 3.2 | 3.1 | 0.513 |
| walker |  | 5090 | 180 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 5, line: 0 } |  |  | 0.513 |
| walker |  | 5111 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 40, sub: 0, line: 396 } |  |  | 0.513 |
| walker |  | 5291 | 180 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 6, line: 0 } |  |  | 0.513 |
| walker |  | 5312 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 52, sub: 0, line: 456 } |  |  | 0.513 |
| walker |  | 5333 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 53, sub: 0, line: 461 } |  |  | 0.513 |
| walker |  | 5354 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 54, sub: 0, line: 466 } |  |  | 0.513 |
| walker |  | 5376 | 22 | Code::CodeKey { rung: Doc, file: context.go, decl: 31, sub: 0, line: 351 } |  |  | 0.513 |
| walker |  | 5398 | 22 | Code::CodeKey { rung: Doc, file: context.go, decl: 32, sub: 0, line: 356 } |  |  | 0.513 |
| walker |  | 5420 | 22 | Code::CodeKey { rung: Doc, file: context.go, decl: 33, sub: 0, line: 361 } |  |  | 0.513 |
| walker |  | 5442 | 22 | Code::CodeKey { rung: Doc, file: context.go, decl: 34, sub: 0, line: 366 } |  |  | 0.513 |
| ns | 5457 |  | 422 | Context struct: all fields with their doc comments | 4.1 |  | 0.498 |
| walker |  | 5464 | 22 | Code::CodeKey { rung: Doc, file: context.go, decl: 45, sub: 0, line: 421 } |  |  | 0.498 |
| walker |  | 5486 | 22 | Code::CodeKey { rung: Doc, file: context.go, decl: 55, sub: 0, line: 471 } |  |  | 0.498 |
| walker |  | 5655 | 169 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 7, line: 0 } |  |  | 0.498 |
| ns | 5672 |  | 215 | Context flow control and error attachment: complete roster | 4.2 | 4.1 | 0.504 |
| walker |  | 5826 | 171 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 8, line: 0 } |  |  | 0.505 |
| walker |  | 5842 | 16 | Code::CodeKey { rung: Doc, file: context.go, decl: 65, sub: 0, line: 587 } |  |  | 0.505 |
| walker |  | 6002 | 160 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 9, line: 0 } |  |  | 0.506 |
| walker |  | 6018 | 16 | Code::CodeKey { rung: Doc, file: context.go, decl: 76, sub: 0, line: 719 } |  |  | 0.506 |
| walker |  | 6035 | 17 | Code::CodeKey { rung: Doc, file: context.go, decl: 72, sub: 0, line: 660 } |  |  | 0.506 |
| ns | 6043 |  | 371 | Context response rendering: complete roster | 4.3 | 4.1 | 0.493 |
| walker |  | 6052 | 17 | Code::CodeKey { rung: Doc, file: context.go, decl: 74, sub: 0, line: 698 } |  |  | 0.493 |
| walker |  | 6069 | 17 | Code::CodeKey { rung: Doc, file: context.go, decl: 75, sub: 0, line: 713 } |  |  | 0.493 |
| walker |  | 6266 | 197 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 10, line: 0 } |  |  | 0.494 |
| walker |  | 6286 | 20 | Code::CodeKey { rung: Doc, file: context.go, decl: 78, sub: 0, line: 763 } |  |  | 0.494 |
| walker |  | 6306 | 20 | Code::CodeKey { rung: Doc, file: context.go, decl: 80, sub: 0, line: 773 } |  |  | 0.494 |
| walker |  | 6326 | 20 | Code::CodeKey { rung: Doc, file: context.go, decl: 84, sub: 0, line: 793 } |  |  | 0.494 |
| walker |  | 6347 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 79, sub: 0, line: 768 } |  |  | 0.494 |
| walker |  | 6368 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 83, sub: 0, line: 788 } |  |  | 0.494 |
| walker |  | 6390 | 22 | Code::CodeKey { rung: Doc, file: context.go, decl: 81, sub: 0, line: 778 } |  |  | 0.494 |
| ns | 6468 |  | 425 | Context request binding: complete roster of all 26 entry points | 4.4 | 4.1 | 0.485 |
| walker |  | 6572 | 182 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 11, line: 0 } |  |  | 0.496 |
| walker |  | 6738 | 166 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 12, line: 0 } |  |  | 0.511 |
| walker |  | 6761 | 23 | Code::CodeKey { rung: Doc, file: context.go, decl: 22, sub: 0, line: 296 } |  |  | 0.511 |
| ns | 6809 |  | 341 | Context request input: params, query, form, uploads — complete roster | 4.5 | 4.1 | 0.516 |
| walker |  | 6975 | 214 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 13, line: 0 } |  |  | 0.517 |
| walker |  | 6987 | 12 | Code::CodeKey { rung: Doc, file: context.go, decl: 111, sub: 0, line: 1102 } |  |  | 0.517 |
| walker |  | 7000 | 13 | Code::CodeKey { rung: Doc, file: context.go, decl: 107, sub: 0, line: 1073 } |  |  | 0.517 |
| walker |  | 7013 | 13 | Code::CodeKey { rung: Doc, file: context.go, decl: 110, sub: 0, line: 1094 } |  |  | 0.517 |
| walker |  | 7027 | 14 | Code::CodeKey { rung: Doc, file: context.go, decl: 109, sub: 0, line: 1089 } |  |  | 0.517 |
| ns | 7030 |  | 221 | Context headers, cookies and client IP: complete roster | 4.6 | 4.1 | 0.517 |
| walker |  | 7044 | 17 | Code::CodeKey { rung: Doc, file: context.go, decl: 105, sub: 0, line: 1036 } |  |  | 0.517 |
| ns | 7100 |  | 70 | Context key/value store, with the typed-accessor family marked elided | 4.7 | 4.1 | 0.518 |
| walker |  | 7223 | 179 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 14, line: 0 } |  |  | 0.522 |
| walker |  | 7242 | 19 | Code::CodeKey { rung: Doc, file: context.go, decl: 115, sub: 0, line: 1152 } |  |  | 0.522 |
| ns | 7259 |  | 159 | Content negotiation and Context's context.Context implementation | 4.8 | 4.1 | 0.517 |
| walker |  | 7441 | 199 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 15, line: 0 } |  |  | 0.525 |
| walker |  | 7457 | 16 | Code::CodeKey { rung: Doc, file: context.go, decl: 129, sub: 0, line: 1254 } |  |  | 0.525 |
| walker |  | 7476 | 19 | Code::CodeKey { rung: Doc, file: context.go, decl: 125, sub: 0, line: 1234 } |  |  | 0.525 |
| walker |  | 7495 | 19 | Code::CodeKey { rung: Doc, file: context.go, decl: 128, sub: 0, line: 1249 } |  |  | 0.525 |
| walker |  | 7516 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 126, sub: 0, line: 1239 } |  |  | 0.525 |
| walker |  | 7537 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 127, sub: 0, line: 1244 } |  |  | 0.525 |
| ns | 7557 |  | 298 | Bind vs ShouldBind vs ShouldBindBodyWith: the semantic difference | 4.9 | 4.4 | 0.518 |
| walker |  | 7709 | 172 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 16, line: 0 } |  |  | 0.529 |
| ns | 7714 |  | 157 | Middleware chain mechanics: Next() and the abortIndex sentinel | 4.10 | 4.2 | 0.523 |
| walker |  | 7725 | 16 | Code::CodeKey { rung: Doc, file: context.go, decl: 130, sub: 0, line: 1259 } |  |  | 0.523 |
| walker |  | 7744 | 19 | Code::CodeKey { rung: Doc, file: context.go, decl: 136, sub: 0, line: 1319 } |  |  | 0.523 |
| walker |  | 7764 | 20 | Code::CodeKey { rung: Doc, file: context.go, decl: 131, sub: 0, line: 1268 } |  |  | 0.523 |
| walker |  | 7784 | 20 | Code::CodeKey { rung: Doc, file: context.go, decl: 133, sub: 0, line: 1286 } |  |  | 0.523 |
| walker |  | 7807 | 23 | Code::CodeKey { rung: Doc, file: context.go, decl: 132, sub: 0, line: 1276 } |  |  | 0.523 |
| walker |  | 7831 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 41, sub: 0, line: 401 } |  |  | 0.523 |
| walker |  | 7855 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 42, sub: 0, line: 406 } |  |  | 0.523 |
| walker |  | 7879 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 43, sub: 0, line: 411 } |  |  | 0.523 |
| walker |  | 7903 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 44, sub: 0, line: 416 } |  |  | 0.523 |
| walker |  | 7927 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 46, sub: 0, line: 426 } |  |  | 0.523 |
| walker |  | 7951 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 47, sub: 0, line: 431 } |  |  | 0.523 |
| ns | 7959 |  | 245 | binding: the complete MIME constant table | 5.1 |  | 0.516 |
| walker |  | 7975 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 48, sub: 0, line: 436 } |  |  | 0.516 |
| walker |  | 7999 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 49, sub: 0, line: 441 } |  |  | 0.516 |
| walker |  | 8023 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 50, sub: 0, line: 446 } |  |  | 0.516 |
| walker |  | 8047 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 51, sub: 0, line: 451 } |  |  | 0.516 |
| walker |  | 8071 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 82, sub: 0, line: 783 } |  |  | 0.516 |
| walker |  | 8095 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 98, sub: 0, line: 946 } |  |  | 0.516 |
| walker |  | 8119 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 99, sub: 0, line: 951 } |  |  | 0.516 |
| ns | 8195 |  | 236 | binding: the Binding / BindingBody / BindingUri interfaces | 5.2 | 5.1 | 0.507 |
| walker |  | 8302 | 183 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 17, line: 0 } |  |  | 0.514 |
| walker |  | 8418 | 116 | Code::CodeKey { rung: Decl, file: context.go, decl: 138, sub: 0, line: 1350 } |  |  | 0.515 |
| ns | 8430 |  | 235 | binding: the registry of concrete binder singletons | 5.3 | 5.2 | 0.507 |
| walker |  | 8431 | 13 | Code::CodeKey { rung: Doc, file: context.go, decl: 138, sub: 0, line: 1350 } |  |  | 0.509 |
| walker |  | 8444 | 13 | Code::CodeKey { rung: Doc, file: context.go, decl: 141, sub: 0, line: 1431 } |  |  | 0.509 |
| walker |  | 8459 | 15 | Code::CodeKey { rung: Doc, file: context.go, decl: 139, sub: 0, line: 1364 } |  |  | 0.509 |
| walker |  | 8474 | 15 | Code::CodeKey { rung: Doc, file: context.go, decl: 140, sub: 0, line: 1400 } |  |  | 0.509 |
| walker |  | 8490 | 16 | Code::CodeKey { rung: Doc, file: context.go, decl: 144, sub: 0, line: 1463 } |  |  | 0.509 |
| walker |  | 8513 | 23 | Code::CodeKey { rung: Doc, file: context.go, decl: 143, sub: 0, line: 1455 } |  |  | 0.509 |
| walker |  | 8538 | 25 | Code::CodeKey { rung: Doc, file: context.go, decl: 102, sub: 0, line: 966 } |  |  | 0.509 |
| walker |  | 8563 | 25 | Code::CodeKey { rung: Doc, file: context.go, decl: 142, sub: 0, line: 1447 } |  |  | 0.509 |
| walker |  | 8589 | 26 | Code::CodeKey { rung: Doc, file: context.go, decl: 56, sub: 0, line: 476 } |  |  | 0.509 |
| walker |  | 8615 | 26 | Code::CodeKey { rung: Doc, file: context.go, decl: 100, sub: 0, line: 956 } |  |  | 0.509 |
| walker |  | 8641 | 26 | Code::CodeKey { rung: Doc, file: context.go, decl: 134, sub: 0, line: 1291 } |  |  | 0.509 |
| walker |  | 8668 | 27 | Code::CodeKey { rung: Doc, file: context.go, decl: 104, sub: 0, line: 1027 } |  |  | 0.509 |
| walker |  | 8696 | 28 | Code::CodeKey { rung: Doc, file: context.go, decl: 101, sub: 0, line: 961 } |  |  | 0.509 |
| ns | 8705 |  | 275 | binding.Default(): how a request picks its binder | 5.4 | 5.3 | 0.499 |
| walker |  | 8725 | 29 | Code::CodeKey { rung: Doc, file: context.go, decl: 96, sub: 0, line: 919 } |  |  | 0.499 |
| walker |  | 8757 | 32 | Code::CodeKey { rung: Doc, file: context.go, decl: 137, sub: 0, line: 1328 } |  |  | 0.499 |
| walker |  | 8790 | 33 | Code::CodeKey { rung: Doc, file: context.go, decl: 9, sub: 0, line: 155 } |  |  | 0.499 |
| walker |  | 8824 | 34 | Code::CodeKey { rung: Doc, file: context.go, decl: 106, sub: 0, line: 1042 } |  |  | 0.499 |
| walker |  | 8859 | 35 | Code::CodeKey { rung: Doc, file: context.go, decl: 85, sub: 0, line: 799 } |  |  | 0.499 |
| walker |  | 8894 | 35 | Code::CodeKey { rung: Doc, file: context.go, decl: 124, sub: 0, line: 1229 } |  |  | 0.499 |
| walker |  | 8930 | 36 | Code::CodeKey { rung: Doc, file: context.go, decl: 120, sub: 0, line: 1205 } |  |  | 0.499 |
| walker |  | 8966 | 36 | Code::CodeKey { rung: Doc, file: context.go, decl: 123, sub: 0, line: 1223 } |  |  | 0.499 |
| walker |  | 9003 | 37 | Code::CodeKey { rung: Doc, file: context.go, decl: 95, sub: 0, line: 909 } |  |  | 0.499 |
| walker |  | 9041 | 38 | Code::CodeKey { rung: Doc, file: context.go, decl: 20, sub: 0, line: 276 } |  |  | 0.499 |
| walker |  | 9079 | 38 | Code::CodeKey { rung: Doc, file: context.go, decl: 57, sub: 0, line: 482 } |  |  | 0.499 |
| walker |  | 9117 | 38 | Code::CodeKey { rung: Doc, file: context.go, decl: 66, sub: 0, line: 594 } |  |  | 0.499 |
| ns | 9150 |  | 445 | render: the Render interface and every implementation | 5.5 |  | 0.486 |
| walker |  | 9156 | 39 | Code::CodeKey { rung: Doc, file: context.go, decl: 21, sub: 0, line: 288 } |  |  | 0.486 |
| walker |  | 9195 | 39 | Code::CodeKey { rung: Doc, file: context.go, decl: 63, sub: 0, line: 563 } |  |  | 0.486 |
| walker |  | 9234 | 39 | Code::CodeKey { rung: Doc, file: context.go, decl: 67, sub: 0, line: 601 } |  |  | 0.486 |
| walker |  | 9273 | 39 | Code::CodeKey { rung: Doc, file: context.go, decl: 73, sub: 0, line: 667 } |  |  | 0.486 |
| walker |  | 9312 | 39 | Code::CodeKey { rung: Doc, file: context.go, decl: 94, sub: 0, line: 903 } |  |  | 0.483 |
| ns | 9312 |  | 162 | Every build-tag line in the repository | 5.6 | 5.4 | 0.483 |
| walker |  | 9352 | 40 | Code::CodeKey { rung: Doc, file: context.go, decl: 64, sub: 0, line: 580 } |  |  | 0.483 |
| walker |  | 9392 | 40 | Code::CodeKey { rung: Doc, file: context.go, decl: 70, sub: 0, line: 633 } |  |  | 0.483 |
| walker |  | 9432 | 40 | Code::CodeKey { rung: Doc, file: context.go, decl: 90, sub: 0, line: 879 } |  |  | 0.483 |
| ns | 9452 |  | 140 | binding: StructValidator and the default validator hook | 5.7 | 5.2 | 0.479 |
| walker |  | 9473 | 41 | Code::CodeKey { rung: Doc, file: context.go, decl: 8, sub: 0, line: 149 } |  |  | 0.479 |
| walker |  | 9514 | 41 | Code::CodeKey { rung: Doc, file: context.go, decl: 71, sub: 0, line: 653 } |  |  | 0.479 |
| walker |  | 9555 | 41 | Code::CodeKey { rung: Doc, file: context.go, decl: 89, sub: 0, line: 873 } |  |  | 0.479 |
| walker |  | 9597 | 42 | Code::CodeKey { rung: Doc, file: context.go, decl: 12, sub: 0, line: 188 } |  |  | 0.481 |
| ns | 9624 |  | 172 | Run modes: GIN_MODE, the three mode constants, and the default writers | 6.1 |  | 0.476 |
| walker |  | 9640 | 43 | Code::CodeKey { rung: Doc, file: context.go, decl: 91, sub: 0, line: 885 } |  |  | 0.476 |
| walker |  | 9683 | 43 | Code::CodeKey { rung: Doc, file: context.go, decl: 93, sub: 0, line: 897 } |  |  | 0.476 |
| ns | 9695 |  | 71 | mode.go: complete function roster | 6.2 | 6.1 | 0.474 |
| walker |  | 9726 | 43 | Code::CodeKey { rung: Doc, file: context.go, decl: 121, sub: 0, line: 1211 } |  |  | 0.474 |
| walker |  | 9769 | 43 | Code::CodeKey { rung: Doc, file: context.go, decl: 122, sub: 0, line: 1217 } |  |  | 0.474 |
| walker |  | 9812 | 43 | Code::CodeKey { rung: Doc, file: context.go, decl: 135, sub: 0, line: 1309 } |  |  | 0.474 |
| ns | 9972 |  | 277 | Built-in middleware and helper constructors: complete package-level roster | 6.3 |  | 0.467 |
| walker |  | 9996 | 184 | Code::CodeKey { rung: Names, file: errors.go, decl: 0, sub: 0, line: 0 } |  |  | 0.467 |
