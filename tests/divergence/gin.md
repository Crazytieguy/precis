Score(3000)=0.585 I=0.651 C=0.525 ns_rows≤3K=15/45 grid(1000/1442/2080/3000/4327/6240/9000)=0.751/0.715/0.636/0.585/0.547/0.491/0.498

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 98 |  | 98 | Package identity: module path, Go version, framework version | 1.1 |  | 0.000 |
| ns | 186 |  | 88 | README lede: what Gin is and what it is for | 1.2 |  | 0.000 |
| walker |  | 229 | 229 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 233 | 4 | Fs::DirListing { dir: docs } |  |  | 0.000 |
| walker |  | 240 | 7 | Fs::DirListing { dir: internal } |  |  | 0.000 |
| ns | 324 |  | 138 | doc.go: the canonical hello-world call site | 1.3 | 1.1 | 0.000 |
| walker |  | 369 | 129 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.000 |
| walker |  | 384 | 15 | Fs::DirListing { dir: ginS } |  |  | 0.000 |
| walker |  | 415 | 31 | GoMod::Identity { file: go.mod } |  |  | 0.086 |
| walker |  | 424 | 9 | Fs::DirListing { dir: internal/fs } |  |  | 0.087 |
| walker |  | 435 | 11 | Fs::DirListing { dir: internal/bytesconv } |  |  | 0.087 |
| walker |  | 439 | 4 | Fs::DirListing { dir: examples } |  |  | 0.087 |
| ns | 444 |  | 120 | README key-feature list (first half) | 1.4 |  | 0.074 |
| walker |  | 462 | 23 | Fs::DirListing { dir: codec/json } |  |  | 0.076 |
| walker |  | 537 | 75 | Fs::DirListing { dir: render } |  |  | 0.077 |
| walker |  | 552 | 15 | Fs::DirListing { dir: testdata } |  |  | 0.078 |
| ns | 569 |  | 125 | README key-feature list (second half) + Go version prerequisite | 1.5 |  | 0.070 |
| walker |  | 571 | 19 | Fs::DirListing { dir: .github } |  |  | 0.070 |
| walker |  | 593 | 22 | Fs::DirListing { dir: .github/workflows } |  |  | 0.070 |
| walker |  | 602 | 9 | Fs::DirListing { dir: testdata/protoexample } |  |  | 0.071 |
| ns | 661 |  | 92 | docs/doc.md top-level section map | 1.6 |  | 0.061 |
| walker |  | 758 | 156 | Fs::DirListing { dir: binding } |  |  | 0.069 |
| ns | 890 |  | 229 | Complete repository root listing | 1.7 |  | 0.425 |
| walker |  | 959 | 201 | Code::CodeKey { rung: ModuleDoc, file: doc.go, decl: 0, sub: 0, line: 0 } |  |  | 0.751 |
| ns | 1121 |  | 231 | binding/ and render/ directory listings | 1.8 |  | 0.761 |
| walker |  | 1134 | 175 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.761 |
| walker |  | 1220 | 86 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.776 |
| ns | 1236 |  | 115 | Remaining source/data directory listings (codec, ginS, internal, docs, examples, testdata) | 1.9 |  | 0.757 |
| ns | 1434 |  | 198 | Core public type vocabulary: HandlerFunc, OptionFunc, HandlersChain, RouteInfo | 2.1 |  | 0.715 |
| walker |  | 1553 | 333 | Plaintext::Whole { file: Makefile } |  |  | 0.715 |
| ns | 1684 |  | 250 | Engine struct: complete exported configuration field roster | 2.2 |  | 0.648 |
| walker |  | 1781 | 228 | Code::CodeKey { rung: Names, file: gin.go, decl: 0, sub: 0, line: 0 } |  |  | 0.651 |
| walker |  | 1821 | 40 | Code::CodeKey { rung: Decl, file: gin.go, decl: 5, sub: 0, line: 68 } |  |  | 0.659 |
| walker |  | 1918 | 97 | Code::CodeKey { rung: Decl, file: gin.go, decl: 7, sub: 0, line: 79 } |  |  | 0.659 |
| walker |  | 1924 | 6 | Code::CodeKey { rung: Doc, file: gin.go, decl: 7, sub: 0, line: 79 } |  |  | 0.659 |
| walker |  | 1935 | 11 | Code::CodeKey { rung: Doc, file: gin.go, decl: 6, sub: 0, line: 76 } |  |  | 0.662 |
| walker |  | 1947 | 12 | Code::CodeKey { rung: Doc, file: gin.go, decl: 3, sub: 0, line: 57 } |  |  | 0.666 |
| walker |  | 1961 | 14 | Code::CodeKey { rung: Doc, file: gin.go, decl: 2, sub: 0, line: 54 } |  |  | 0.671 |
| walker |  | 1979 | 18 | Code::CodeKey { rung: Doc, file: gin.go, decl: 1, sub: 0, line: 51 } |  |  | 0.678 |
| walker |  | 1998 | 19 | Code::CodeKey { rung: Doc, file: gin.go, decl: 10, sub: 0, line: 236 } |  |  | 0.678 |
| ns | 2020 |  | 336 | IRoutes / IRouter: the complete route-registration interface | 2.3 |  | 0.636 |
| walker |  | 2169 | 171 | Code::CodeKey { rung: Names, file: gin.go, decl: 0, sub: 1, line: 0 } |  |  | 0.636 |
| walker |  | 2184 | 15 | Code::CodeKey { rung: Doc, file: gin.go, decl: 17, sub: 0, line: 312 } |  |  | 0.636 |
| walker |  | 2202 | 18 | Code::CodeKey { rung: Doc, file: gin.go, decl: 18, sub: 0, line: 321 } |  |  | 0.636 |
| ns | 2441 |  | 421 | Engine constructor New(): every default value | 2.4 | 2.2 | 0.595 |
| walker |  | 2480 | 278 | Code::CodeKey { rung: Names, file: gin.go, decl: 0, sub: 2, line: 0 } |  |  | 0.597 |
| walker |  | 2494 | 14 | Code::CodeKey { rung: Doc, file: gin.go, decl: 31, sub: 0, line: 662 } |  |  | 0.597 |
| walker |  | 2512 | 18 | Code::CodeKey { rung: Doc, file: gin.go, decl: 22, sub: 0, line: 348 } |  |  | 0.597 |
| walker |  | 2532 | 20 | Code::CodeKey { rung: Doc, file: gin.go, decl: 12, sub: 0, line: 259 } |  |  | 0.597 |
| walker |  | 2552 | 20 | Code::CodeKey { rung: Doc, file: gin.go, decl: 13, sub: 0, line: 265 } |  |  | 0.597 |
| walker |  | 2572 | 20 | Code::CodeKey { rung: Doc, file: gin.go, decl: 20, sub: 0, line: 332 } |  |  | 0.597 |
| walker |  | 2594 | 22 | Code::CodeKey { rung: Doc, file: gin.go, decl: 5, sub: 0, line: 68 } |  |  | 0.605 |
| walker |  | 2616 | 22 | Code::CodeKey { rung: Doc, file: gin.go, decl: 19, sub: 0, line: 326 } |  |  | 0.605 |
| ns | 2634 |  | 193 | Default(), RouterGroup struct, and the IRouter assertions | 2.5 | 2.3 | 0.585 |
| walker |  | 2639 | 23 | Code::CodeKey { rung: Doc, file: gin.go, decl: 4, sub: 0, line: 60 } |  |  | 0.585 |
| ns | 2829 |  | 195 | Run* server entry points: all six transports | 2.6 |  | 0.584 |
| walker |  | 2947 | 308 | GoMod::File { file: go.mod } |  |  | 0.584 |
| walker |  | 3131 | 184 | Code::CodeKey { rung: Names, file: routergroup.go, decl: 0, sub: 0, line: 0 } |  |  | 0.585 |
| walker |  | 3148 | 17 | Code::CodeKey { rung: Decl, file: routergroup.go, decl: 3, sub: 0, line: 55 } |  |  | 0.586 |
| walker |  | 3176 | 28 | Code::CodeKey { rung: Decl, file: routergroup.go, decl: 1, sub: 0, line: 27 } |  |  | 0.587 |
| ns | 3333 |  | 504 | gin.go: complete function roster beyond the constructors and Run* | 2.7 | 2.5 | 0.565 |
| walker |  | 3421 | 245 | Code::CodeKey { rung: Decl, file: routergroup.go, decl: 2, sub: 0, line: 33 } |  |  | 0.600 |
| walker |  | 3432 | 11 | Code::CodeKey { rung: Doc, file: routergroup.go, decl: 2, sub: 0, line: 33 } |  |  | 0.604 |
| walker |  | 3450 | 18 | Code::CodeKey { rung: Doc, file: routergroup.go, decl: 1, sub: 0, line: 27 } |  |  | 0.610 |
| walker |  | 3469 | 19 | Code::CodeKey { rung: Doc, file: routergroup.go, decl: 4, sub: 0, line: 65 } |  |  | 0.610 |
| walker |  | 3488 | 19 | Code::CodeKey { rung: Doc, file: routergroup.go, decl: 8, sub: 0, line: 111 } |  |  | 0.610 |
| ns | 3489 |  | 156 | Engine struct: unexported field tail | 2.8 | 2.2 | 0.596 |
| walker |  | 3507 | 19 | Code::CodeKey { rung: Doc, file: routergroup.go, decl: 9, sub: 0, line: 116 } |  |  | 0.596 |
| ns | 3637 |  | 148 | tree.go: Param / Params — the URL-parameter type and its accessors | 2.9 |  | 0.585 |
| walker |  | 3712 | 205 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 0, line: 0 } |  |  | 0.585 |
| walker |  | 3717 | 5 | Code::CodeKey { rung: Decl, file: context.go, decl: 1, sub: 0, line: 31 } |  |  | 0.585 |
| walker |  | 3732 | 15 | Code::CodeKey { rung: Doc, file: context.go, decl: 1, sub: 0, line: 31 } |  |  | 0.585 |
| ns | 3816 |  | 179 | handleHTTPRequest: path resolution before the tree lookup | 2.10 | 2.7 | 0.571 |
| walker |  | 3934 | 202 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 1, line: 0 } |  |  | 0.571 |
| walker |  | 4082 | 148 | Code::CodeKey { rung: Decl, file: context.go, decl: 6, sub: 0, line: 61 } |  |  | 0.572 |
| walker |  | 4093 | 11 | Code::CodeKey { rung: Doc, file: context.go, decl: 10, sub: 0, line: 167 } |  |  | 0.572 |
| walker |  | 4106 | 13 | Code::CodeKey { rung: Doc, file: context.go, decl: 2, sub: 0, line: 47 } |  |  | 0.572 |
| walker |  | 4121 | 15 | Code::CodeKey { rung: Doc, file: context.go, decl: 3, sub: 0, line: 50 } |  |  | 0.572 |
| walker |  | 4138 | 17 | Code::CodeKey { rung: Doc, file: context.go, decl: 13, sub: 0, line: 199 } |  |  | 0.572 |
| ns | 4189 |  | 373 | handleHTTPRequest: the radix lookup, redirects and the 404/405 exits | 2.11 | 2.10 | 0.546 |
| walker |  | 4308 | 170 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 2, line: 0 } |  |  | 0.547 |
| ns | 4438 |  | 249 | tree.go: nodeValue and the complete radix-tree function roster | 2.12 | 2.9 | 0.533 |
| walker |  | 4487 | 179 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 3, line: 0 } |  |  | 0.533 |
| walker |  | 4505 | 18 | Code::CodeKey { rung: Doc, file: context.go, decl: 23, sub: 0, line: 311 } |  |  | 0.533 |
| walker |  | 4523 | 18 | Code::CodeKey { rung: Doc, file: context.go, decl: 24, sub: 0, line: 316 } |  |  | 0.533 |
| walker |  | 4541 | 18 | Code::CodeKey { rung: Doc, file: context.go, decl: 25, sub: 0, line: 321 } |  |  | 0.533 |
| walker |  | 4560 | 19 | Code::CodeKey { rung: Doc, file: context.go, decl: 30, sub: 0, line: 346 } |  |  | 0.533 |
| walker |  | 4581 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 26, sub: 0, line: 326 } |  |  | 0.533 |
| walker |  | 4602 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 27, sub: 0, line: 331 } |  |  | 0.533 |
| walker |  | 4623 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 28, sub: 0, line: 336 } |  |  | 0.533 |
| walker |  | 4644 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 29, sub: 0, line: 341 } |  |  | 0.533 |
| ns | 4658 |  | 220 | path.go: cleanPath semantics and the file's whole function set | 2.13 | 2.11 | 0.524 |
| walker |  | 4829 | 185 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 4, line: 0 } |  |  | 0.524 |
| walker |  | 4846 | 17 | Code::CodeKey { rung: Doc, file: context.go, decl: 37, sub: 0, line: 381 } |  |  | 0.524 |
| walker |  | 4864 | 18 | Code::CodeKey { rung: Doc, file: context.go, decl: 38, sub: 0, line: 386 } |  |  | 0.524 |
| walker |  | 4882 | 18 | Code::CodeKey { rung: Doc, file: context.go, decl: 39, sub: 0, line: 391 } |  |  | 0.524 |
| ns | 4888 |  | 230 | Trusted-platform constants and the SetTrustedProxies contract | 2.14 | 2.7 | 0.518 |
| walker |  | 4902 | 20 | Code::CodeKey { rung: Doc, file: context.go, decl: 35, sub: 0, line: 371 } |  |  | 0.518 |
| walker |  | 4922 | 20 | Code::CodeKey { rung: Doc, file: context.go, decl: 36, sub: 0, line: 376 } |  |  | 0.518 |
| ns | 4977 |  | 89 | Makefile: complete target roster | 3.1 |  | 0.512 |
| ns | 5035 |  | 58 | Makefile variables: which packages are tested and vetted | 3.2 | 3.1 | 0.511 |
| walker |  | 5102 | 180 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 5, line: 0 } |  |  | 0.511 |
| walker |  | 5123 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 40, sub: 0, line: 396 } |  |  | 0.511 |
| walker |  | 5303 | 180 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 6, line: 0 } |  |  | 0.511 |
| walker |  | 5324 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 52, sub: 0, line: 456 } |  |  | 0.511 |
| walker |  | 5345 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 53, sub: 0, line: 461 } |  |  | 0.511 |
| walker |  | 5366 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 54, sub: 0, line: 466 } |  |  | 0.511 |
| walker |  | 5388 | 22 | Code::CodeKey { rung: Doc, file: context.go, decl: 31, sub: 0, line: 351 } |  |  | 0.511 |
| walker |  | 5410 | 22 | Code::CodeKey { rung: Doc, file: context.go, decl: 32, sub: 0, line: 356 } |  |  | 0.511 |
| walker |  | 5432 | 22 | Code::CodeKey { rung: Doc, file: context.go, decl: 33, sub: 0, line: 361 } |  |  | 0.511 |
| walker |  | 5454 | 22 | Code::CodeKey { rung: Doc, file: context.go, decl: 34, sub: 0, line: 366 } |  |  | 0.511 |
| ns | 5457 |  | 422 | Context struct: all fields with their doc comments | 4.1 |  | 0.495 |
| walker |  | 5476 | 22 | Code::CodeKey { rung: Doc, file: context.go, decl: 45, sub: 0, line: 421 } |  |  | 0.495 |
| walker |  | 5498 | 22 | Code::CodeKey { rung: Doc, file: context.go, decl: 55, sub: 0, line: 471 } |  |  | 0.495 |
| walker |  | 5667 | 169 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 7, line: 0 } |  |  | 0.496 |
| ns | 5672 |  | 215 | Context flow control and error attachment: complete roster | 4.2 | 4.1 | 0.502 |
| walker |  | 5838 | 171 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 8, line: 0 } |  |  | 0.503 |
| walker |  | 5854 | 16 | Code::CodeKey { rung: Doc, file: context.go, decl: 65, sub: 0, line: 587 } |  |  | 0.503 |
| walker |  | 6014 | 160 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 9, line: 0 } |  |  | 0.504 |
| walker |  | 6030 | 16 | Code::CodeKey { rung: Doc, file: context.go, decl: 76, sub: 0, line: 719 } |  |  | 0.504 |
| ns | 6043 |  | 371 | Context response rendering: complete roster | 4.3 | 4.1 | 0.491 |
| walker |  | 6047 | 17 | Code::CodeKey { rung: Doc, file: context.go, decl: 72, sub: 0, line: 660 } |  |  | 0.491 |
| walker |  | 6064 | 17 | Code::CodeKey { rung: Doc, file: context.go, decl: 74, sub: 0, line: 698 } |  |  | 0.491 |
| walker |  | 6081 | 17 | Code::CodeKey { rung: Doc, file: context.go, decl: 75, sub: 0, line: 713 } |  |  | 0.491 |
| walker |  | 6278 | 197 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 10, line: 0 } |  |  | 0.492 |
| walker |  | 6298 | 20 | Code::CodeKey { rung: Doc, file: context.go, decl: 78, sub: 0, line: 763 } |  |  | 0.492 |
| walker |  | 6318 | 20 | Code::CodeKey { rung: Doc, file: context.go, decl: 80, sub: 0, line: 773 } |  |  | 0.492 |
| walker |  | 6338 | 20 | Code::CodeKey { rung: Doc, file: context.go, decl: 84, sub: 0, line: 793 } |  |  | 0.492 |
| walker |  | 6359 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 79, sub: 0, line: 768 } |  |  | 0.492 |
| walker |  | 6380 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 83, sub: 0, line: 788 } |  |  | 0.492 |
| walker |  | 6402 | 22 | Code::CodeKey { rung: Doc, file: context.go, decl: 81, sub: 0, line: 778 } |  |  | 0.492 |
| ns | 6468 |  | 425 | Context request binding: complete roster of all 26 entry points | 4.4 | 4.1 | 0.483 |
| walker |  | 6584 | 182 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 11, line: 0 } |  |  | 0.494 |
| walker |  | 6750 | 166 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 12, line: 0 } |  |  | 0.509 |
| walker |  | 6773 | 23 | Code::CodeKey { rung: Doc, file: context.go, decl: 22, sub: 0, line: 296 } |  |  | 0.509 |
| ns | 6809 |  | 341 | Context request input: params, query, form, uploads — complete roster | 4.5 | 4.1 | 0.515 |
| walker |  | 6987 | 214 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 13, line: 0 } |  |  | 0.515 |
| walker |  | 6999 | 12 | Code::CodeKey { rung: Doc, file: context.go, decl: 111, sub: 0, line: 1102 } |  |  | 0.515 |
| walker |  | 7012 | 13 | Code::CodeKey { rung: Doc, file: context.go, decl: 107, sub: 0, line: 1073 } |  |  | 0.515 |
| walker |  | 7025 | 13 | Code::CodeKey { rung: Doc, file: context.go, decl: 110, sub: 0, line: 1094 } |  |  | 0.515 |
| ns | 7030 |  | 221 | Context headers, cookies and client IP: complete roster | 4.6 | 4.1 | 0.516 |
| walker |  | 7039 | 14 | Code::CodeKey { rung: Doc, file: context.go, decl: 109, sub: 0, line: 1089 } |  |  | 0.516 |
| walker |  | 7056 | 17 | Code::CodeKey { rung: Doc, file: context.go, decl: 105, sub: 0, line: 1036 } |  |  | 0.516 |
| ns | 7100 |  | 70 | Context key/value store, with the typed-accessor family marked elided | 4.7 | 4.1 | 0.517 |
| walker |  | 7235 | 179 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 14, line: 0 } |  |  | 0.522 |
| walker |  | 7254 | 19 | Code::CodeKey { rung: Doc, file: context.go, decl: 115, sub: 0, line: 1152 } |  |  | 0.522 |
| ns | 7259 |  | 159 | Content negotiation and Context's context.Context implementation | 4.8 | 4.1 | 0.516 |
| walker |  | 7453 | 199 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 15, line: 0 } |  |  | 0.524 |
| walker |  | 7469 | 16 | Code::CodeKey { rung: Doc, file: context.go, decl: 129, sub: 0, line: 1254 } |  |  | 0.524 |
| walker |  | 7488 | 19 | Code::CodeKey { rung: Doc, file: context.go, decl: 125, sub: 0, line: 1234 } |  |  | 0.524 |
| walker |  | 7507 | 19 | Code::CodeKey { rung: Doc, file: context.go, decl: 128, sub: 0, line: 1249 } |  |  | 0.524 |
| walker |  | 7528 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 126, sub: 0, line: 1239 } |  |  | 0.524 |
| walker |  | 7549 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 127, sub: 0, line: 1244 } |  |  | 0.524 |
| ns | 7557 |  | 298 | Bind vs ShouldBind vs ShouldBindBodyWith: the semantic difference | 4.9 | 4.4 | 0.518 |
| ns | 7714 |  | 157 | Middleware chain mechanics: Next() and the abortIndex sentinel | 4.10 | 4.2 | 0.511 |
| walker |  | 7721 | 172 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 16, line: 0 } |  |  | 0.523 |
| walker |  | 7737 | 16 | Code::CodeKey { rung: Doc, file: context.go, decl: 130, sub: 0, line: 1259 } |  |  | 0.523 |
| walker |  | 7756 | 19 | Code::CodeKey { rung: Doc, file: context.go, decl: 136, sub: 0, line: 1319 } |  |  | 0.523 |
| walker |  | 7776 | 20 | Code::CodeKey { rung: Doc, file: context.go, decl: 131, sub: 0, line: 1268 } |  |  | 0.523 |
| walker |  | 7796 | 20 | Code::CodeKey { rung: Doc, file: context.go, decl: 133, sub: 0, line: 1286 } |  |  | 0.523 |
| walker |  | 7819 | 23 | Code::CodeKey { rung: Doc, file: context.go, decl: 132, sub: 0, line: 1276 } |  |  | 0.523 |
| walker |  | 7843 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 41, sub: 0, line: 401 } |  |  | 0.523 |
| walker |  | 7867 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 42, sub: 0, line: 406 } |  |  | 0.523 |
| walker |  | 7891 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 43, sub: 0, line: 411 } |  |  | 0.523 |
| walker |  | 7915 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 44, sub: 0, line: 416 } |  |  | 0.523 |
| walker |  | 7939 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 46, sub: 0, line: 426 } |  |  | 0.523 |
| ns | 7959 |  | 245 | binding: the complete MIME constant table | 5.1 |  | 0.515 |
| walker |  | 7963 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 47, sub: 0, line: 431 } |  |  | 0.515 |
| walker |  | 7987 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 48, sub: 0, line: 436 } |  |  | 0.515 |
| walker |  | 8011 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 49, sub: 0, line: 441 } |  |  | 0.515 |
| walker |  | 8035 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 50, sub: 0, line: 446 } |  |  | 0.515 |
| walker |  | 8059 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 51, sub: 0, line: 451 } |  |  | 0.515 |
| walker |  | 8083 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 82, sub: 0, line: 783 } |  |  | 0.515 |
| walker |  | 8107 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 98, sub: 0, line: 946 } |  |  | 0.515 |
| walker |  | 8131 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 99, sub: 0, line: 951 } |  |  | 0.515 |
| ns | 8195 |  | 236 | binding: the Binding / BindingBody / BindingUri interfaces | 5.2 | 5.1 | 0.506 |
| walker |  | 8314 | 183 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 17, line: 0 } |  |  | 0.514 |
| walker |  | 8430 | 116 | Code::CodeKey { rung: Decl, file: context.go, decl: 138, sub: 0, line: 1350 } |  |  | 0.507 |
| ns | 8430 |  | 235 | binding: the registry of concrete binder singletons | 5.3 | 5.2 | 0.507 |
| walker |  | 8443 | 13 | Code::CodeKey { rung: Doc, file: context.go, decl: 138, sub: 0, line: 1350 } |  |  | 0.509 |
| walker |  | 8456 | 13 | Code::CodeKey { rung: Doc, file: context.go, decl: 141, sub: 0, line: 1431 } |  |  | 0.509 |
| walker |  | 8471 | 15 | Code::CodeKey { rung: Doc, file: context.go, decl: 139, sub: 0, line: 1364 } |  |  | 0.509 |
| walker |  | 8486 | 15 | Code::CodeKey { rung: Doc, file: context.go, decl: 140, sub: 0, line: 1400 } |  |  | 0.509 |
| walker |  | 8502 | 16 | Code::CodeKey { rung: Doc, file: context.go, decl: 144, sub: 0, line: 1463 } |  |  | 0.509 |
| walker |  | 8525 | 23 | Code::CodeKey { rung: Doc, file: context.go, decl: 143, sub: 0, line: 1455 } |  |  | 0.509 |
| walker |  | 8550 | 25 | Code::CodeKey { rung: Doc, file: context.go, decl: 102, sub: 0, line: 966 } |  |  | 0.509 |
| walker |  | 8575 | 25 | Code::CodeKey { rung: Doc, file: context.go, decl: 142, sub: 0, line: 1447 } |  |  | 0.509 |
| walker |  | 8601 | 26 | Code::CodeKey { rung: Doc, file: context.go, decl: 56, sub: 0, line: 476 } |  |  | 0.509 |
| walker |  | 8627 | 26 | Code::CodeKey { rung: Doc, file: context.go, decl: 100, sub: 0, line: 956 } |  |  | 0.509 |
| walker |  | 8653 | 26 | Code::CodeKey { rung: Doc, file: context.go, decl: 134, sub: 0, line: 1291 } |  |  | 0.509 |
| walker |  | 8680 | 27 | Code::CodeKey { rung: Doc, file: context.go, decl: 104, sub: 0, line: 1027 } |  |  | 0.509 |
| ns | 8705 |  | 275 | binding.Default(): how a request picks its binder | 5.4 | 5.3 | 0.498 |
| walker |  | 8708 | 28 | Code::CodeKey { rung: Doc, file: context.go, decl: 101, sub: 0, line: 961 } |  |  | 0.498 |
| walker |  | 8737 | 29 | Code::CodeKey { rung: Doc, file: context.go, decl: 96, sub: 0, line: 919 } |  |  | 0.498 |
| walker |  | 8769 | 32 | Code::CodeKey { rung: Doc, file: context.go, decl: 137, sub: 0, line: 1328 } |  |  | 0.498 |
| walker |  | 8802 | 33 | Code::CodeKey { rung: Doc, file: context.go, decl: 9, sub: 0, line: 155 } |  |  | 0.498 |
| walker |  | 8836 | 34 | Code::CodeKey { rung: Doc, file: context.go, decl: 106, sub: 0, line: 1042 } |  |  | 0.498 |
| walker |  | 8871 | 35 | Code::CodeKey { rung: Doc, file: context.go, decl: 85, sub: 0, line: 799 } |  |  | 0.498 |
| walker |  | 8906 | 35 | Code::CodeKey { rung: Doc, file: context.go, decl: 124, sub: 0, line: 1229 } |  |  | 0.498 |
| walker |  | 8942 | 36 | Code::CodeKey { rung: Doc, file: context.go, decl: 120, sub: 0, line: 1205 } |  |  | 0.498 |
| walker |  | 8978 | 36 | Code::CodeKey { rung: Doc, file: context.go, decl: 123, sub: 0, line: 1223 } |  |  | 0.498 |
| walker |  | 9015 | 37 | Code::CodeKey { rung: Doc, file: context.go, decl: 95, sub: 0, line: 909 } |  |  | 0.498 |
| walker |  | 9053 | 38 | Code::CodeKey { rung: Doc, file: context.go, decl: 20, sub: 0, line: 276 } |  |  | 0.498 |
| walker |  | 9091 | 38 | Code::CodeKey { rung: Doc, file: context.go, decl: 57, sub: 0, line: 482 } |  |  | 0.498 |
| walker |  | 9129 | 38 | Code::CodeKey { rung: Doc, file: context.go, decl: 66, sub: 0, line: 594 } |  |  | 0.498 |
| ns | 9150 |  | 445 | render: the Render interface and every implementation | 5.5 |  | 0.486 |
| walker |  | 9168 | 39 | Code::CodeKey { rung: Doc, file: context.go, decl: 21, sub: 0, line: 288 } |  |  | 0.486 |
| walker |  | 9207 | 39 | Code::CodeKey { rung: Doc, file: context.go, decl: 63, sub: 0, line: 563 } |  |  | 0.486 |
| walker |  | 9246 | 39 | Code::CodeKey { rung: Doc, file: context.go, decl: 67, sub: 0, line: 601 } |  |  | 0.486 |
| walker |  | 9285 | 39 | Code::CodeKey { rung: Doc, file: context.go, decl: 73, sub: 0, line: 667 } |  |  | 0.486 |
| ns | 9312 |  | 162 | Every build-tag line in the repository | 5.6 | 5.4 | 0.483 |
| walker |  | 9324 | 39 | Code::CodeKey { rung: Doc, file: context.go, decl: 94, sub: 0, line: 903 } |  |  | 0.483 |
| walker |  | 9364 | 40 | Code::CodeKey { rung: Doc, file: context.go, decl: 64, sub: 0, line: 580 } |  |  | 0.483 |
| walker |  | 9404 | 40 | Code::CodeKey { rung: Doc, file: context.go, decl: 70, sub: 0, line: 633 } |  |  | 0.483 |
| walker |  | 9444 | 40 | Code::CodeKey { rung: Doc, file: context.go, decl: 90, sub: 0, line: 879 } |  |  | 0.483 |
| ns | 9452 |  | 140 | binding: StructValidator and the default validator hook | 5.7 | 5.2 | 0.478 |
| walker |  | 9485 | 41 | Code::CodeKey { rung: Doc, file: context.go, decl: 8, sub: 0, line: 149 } |  |  | 0.478 |
| walker |  | 9526 | 41 | Code::CodeKey { rung: Doc, file: context.go, decl: 71, sub: 0, line: 653 } |  |  | 0.478 |
| walker |  | 9567 | 41 | Code::CodeKey { rung: Doc, file: context.go, decl: 89, sub: 0, line: 873 } |  |  | 0.478 |
| walker |  | 9609 | 42 | Code::CodeKey { rung: Doc, file: context.go, decl: 12, sub: 0, line: 188 } |  |  | 0.481 |
| ns | 9624 |  | 172 | Run modes: GIN_MODE, the three mode constants, and the default writers | 6.1 |  | 0.476 |
| walker |  | 9652 | 43 | Code::CodeKey { rung: Doc, file: context.go, decl: 91, sub: 0, line: 885 } |  |  | 0.476 |
| walker |  | 9695 | 43 | Code::CodeKey { rung: Doc, file: context.go, decl: 93, sub: 0, line: 897 } |  |  | 0.474 |
| ns | 9695 |  | 71 | mode.go: complete function roster | 6.2 | 6.1 | 0.474 |
| walker |  | 9738 | 43 | Code::CodeKey { rung: Doc, file: context.go, decl: 121, sub: 0, line: 1211 } |  |  | 0.474 |
| walker |  | 9781 | 43 | Code::CodeKey { rung: Doc, file: context.go, decl: 122, sub: 0, line: 1217 } |  |  | 0.474 |
| walker |  | 9824 | 43 | Code::CodeKey { rung: Doc, file: context.go, decl: 135, sub: 0, line: 1309 } |  |  | 0.474 |
| ns | 9972 |  | 277 | Built-in middleware and helper constructors: complete package-level roster | 6.3 |  | 0.466 |
| walker |  | 9992 | 168 | Code::CodeKey { rung: Names, file: errors.go, decl: 0, sub: 0, line: 0 } |  |  | 0.466 |
