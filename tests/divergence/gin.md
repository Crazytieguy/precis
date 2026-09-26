Score(3000)=0.574 I=0.631 C=0.522 ns_rows≤3K=15/45 grid(1000/1442/2080/3000/4327/6240/9000)=0.751/0.701/0.624/0.574/0.538/0.483/0.490

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 98 |  | 98 | Package identity: module path, Go version, framework version | 1.1 |  | 0.000 |
| ns | 186 |  | 88 | README lede: what Gin is and what it is for | 1.2 |  | 0.000 |
| walker |  | 229 | 229 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 240 | 11 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.000 |
| walker |  | 244 | 4 | Fs::DirListing { dir: docs } |  |  | 0.000 |
| walker |  | 251 | 7 | Fs::DirListing { dir: internal } |  |  | 0.000 |
| walker |  | 266 | 15 | Fs::DirListing { dir: ginS } |  |  | 0.000 |
| walker |  | 297 | 31 | GoMod::Identity { file: go.mod } |  |  | 0.162 |
| walker |  | 306 | 9 | Fs::DirListing { dir: internal/fs } |  |  | 0.162 |
| walker |  | 317 | 11 | Fs::DirListing { dir: internal/bytesconv } |  |  | 0.163 |
| walker |  | 321 | 4 | Fs::DirListing { dir: examples } |  |  | 0.163 |
| ns | 324 |  | 138 | doc.go: the canonical hello-world call site | 1.3 | 1.1 | 0.087 |
| walker |  | 344 | 23 | Fs::DirListing { dir: codec/json } |  |  | 0.089 |
| walker |  | 419 | 75 | Fs::DirListing { dir: render } |  |  | 0.090 |
| walker |  | 434 | 15 | Fs::DirListing { dir: testdata } |  |  | 0.092 |
| ns | 444 |  | 120 | README key-feature list (first half) | 1.4 |  | 0.078 |
| walker |  | 453 | 19 | Fs::DirListing { dir: .github } |  |  | 0.078 |
| walker |  | 475 | 22 | Fs::DirListing { dir: .github/workflows } |  |  | 0.078 |
| walker |  | 484 | 9 | Fs::DirListing { dir: testdata/protoexample } |  |  | 0.079 |
| ns | 569 |  | 125 | README key-feature list (second half) + Go version prerequisite | 1.5 |  | 0.071 |
| walker |  | 640 | 156 | Fs::DirListing { dir: binding } |  |  | 0.080 |
| ns | 661 |  | 92 | docs/doc.md top-level section map | 1.6 |  | 0.069 |
| walker |  | 841 | 201 | Code::CodeKey { rung: ModuleDoc, file: doc.go, decl: 0, sub: 0, line: 0 } |  |  | 0.599 |
| ns | 890 |  | 229 | Complete repository root listing | 1.7 |  | 0.751 |
| walker |  | 955 | 114 | Plaintext::Whole { file: Makefile } |  |  | 0.751 |
| ns | 1121 |  | 231 | binding/ and render/ directory listings | 1.8 |  | 0.761 |
| walker |  | 1150 | 195 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.761 |
| ns | 1236 |  | 115 | Remaining source/data directory listings (codec, ginS, internal, docs, examples, testdata) | 1.9 |  | 0.742 |
| ns | 1434 |  | 198 | Core public type vocabulary: HandlerFunc, OptionFunc, HandlersChain, RouteInfo | 2.1 |  | 0.701 |
| walker |  | 1523 | 373 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.701 |
| ns | 1684 |  | 250 | Engine struct: complete exported configuration field roster | 2.2 |  | 0.635 |
| walker |  | 1751 | 228 | Code::CodeKey { rung: Names, file: gin.go, decl: 0, sub: 0, line: 0 } |  |  | 0.639 |
| walker |  | 1791 | 40 | Code::CodeKey { rung: Decl, file: gin.go, decl: 5, sub: 0, line: 68 } |  |  | 0.646 |
| walker |  | 1888 | 97 | Code::CodeKey { rung: Decl, file: gin.go, decl: 7, sub: 0, line: 79 } |  |  | 0.646 |
| walker |  | 1894 | 6 | Code::CodeKey { rung: Doc, file: gin.go, decl: 7, sub: 0, line: 79 } |  |  | 0.646 |
| walker |  | 1905 | 11 | Code::CodeKey { rung: Doc, file: gin.go, decl: 6, sub: 0, line: 76 } |  |  | 0.649 |
| walker |  | 1917 | 12 | Code::CodeKey { rung: Doc, file: gin.go, decl: 3, sub: 0, line: 57 } |  |  | 0.653 |
| walker |  | 1931 | 14 | Code::CodeKey { rung: Doc, file: gin.go, decl: 2, sub: 0, line: 54 } |  |  | 0.659 |
| walker |  | 1949 | 18 | Code::CodeKey { rung: Doc, file: gin.go, decl: 1, sub: 0, line: 51 } |  |  | 0.665 |
| walker |  | 1968 | 19 | Code::CodeKey { rung: Doc, file: gin.go, decl: 10, sub: 0, line: 236 } |  |  | 0.665 |
| ns | 2020 |  | 336 | IRoutes / IRouter: the complete route-registration interface | 2.3 |  | 0.624 |
| walker |  | 2139 | 171 | Code::CodeKey { rung: Names, file: gin.go, decl: 0, sub: 1, line: 0 } |  |  | 0.625 |
| walker |  | 2154 | 15 | Code::CodeKey { rung: Doc, file: gin.go, decl: 17, sub: 0, line: 312 } |  |  | 0.625 |
| walker |  | 2172 | 18 | Code::CodeKey { rung: Doc, file: gin.go, decl: 18, sub: 0, line: 321 } |  |  | 0.625 |
| ns | 2441 |  | 421 | Engine constructor New(): every default value | 2.4 | 2.2 | 0.584 |
| walker |  | 2450 | 278 | Code::CodeKey { rung: Names, file: gin.go, decl: 0, sub: 2, line: 0 } |  |  | 0.586 |
| walker |  | 2464 | 14 | Code::CodeKey { rung: Doc, file: gin.go, decl: 31, sub: 0, line: 662 } |  |  | 0.586 |
| walker |  | 2482 | 18 | Code::CodeKey { rung: Doc, file: gin.go, decl: 22, sub: 0, line: 348 } |  |  | 0.586 |
| walker |  | 2502 | 20 | Code::CodeKey { rung: Doc, file: gin.go, decl: 12, sub: 0, line: 259 } |  |  | 0.586 |
| walker |  | 2522 | 20 | Code::CodeKey { rung: Doc, file: gin.go, decl: 13, sub: 0, line: 265 } |  |  | 0.586 |
| walker |  | 2542 | 20 | Code::CodeKey { rung: Doc, file: gin.go, decl: 20, sub: 0, line: 332 } |  |  | 0.586 |
| walker |  | 2564 | 22 | Code::CodeKey { rung: Doc, file: gin.go, decl: 5, sub: 0, line: 68 } |  |  | 0.594 |
| walker |  | 2586 | 22 | Code::CodeKey { rung: Doc, file: gin.go, decl: 19, sub: 0, line: 326 } |  |  | 0.594 |
| walker |  | 2609 | 23 | Code::CodeKey { rung: Doc, file: gin.go, decl: 4, sub: 0, line: 60 } |  |  | 0.594 |
| ns | 2634 |  | 193 | Default(), RouterGroup struct, and the IRouter assertions | 2.5 | 2.3 | 0.574 |
| ns | 2829 |  | 195 | Run* server entry points: all six transports | 2.6 |  | 0.573 |
| walker |  | 2917 | 308 | GoMod::File { file: go.mod } |  |  | 0.573 |
| walker |  | 3101 | 184 | Code::CodeKey { rung: Names, file: routergroup.go, decl: 0, sub: 0, line: 0 } |  |  | 0.574 |
| walker |  | 3118 | 17 | Code::CodeKey { rung: Decl, file: routergroup.go, decl: 3, sub: 0, line: 55 } |  |  | 0.576 |
| walker |  | 3146 | 28 | Code::CodeKey { rung: Decl, file: routergroup.go, decl: 1, sub: 0, line: 27 } |  |  | 0.577 |
| ns | 3333 |  | 504 | gin.go: complete function roster beyond the constructors and Run* | 2.7 | 2.5 | 0.555 |
| walker |  | 3391 | 245 | Code::CodeKey { rung: Decl, file: routergroup.go, decl: 2, sub: 0, line: 33 } |  |  | 0.589 |
| walker |  | 3402 | 11 | Code::CodeKey { rung: Doc, file: routergroup.go, decl: 2, sub: 0, line: 33 } |  |  | 0.593 |
| walker |  | 3420 | 18 | Code::CodeKey { rung: Doc, file: routergroup.go, decl: 1, sub: 0, line: 27 } |  |  | 0.599 |
| walker |  | 3439 | 19 | Code::CodeKey { rung: Doc, file: routergroup.go, decl: 4, sub: 0, line: 65 } |  |  | 0.599 |
| walker |  | 3458 | 19 | Code::CodeKey { rung: Doc, file: routergroup.go, decl: 8, sub: 0, line: 111 } |  |  | 0.599 |
| walker |  | 3477 | 19 | Code::CodeKey { rung: Doc, file: routergroup.go, decl: 9, sub: 0, line: 116 } |  |  | 0.599 |
| ns | 3489 |  | 156 | Engine struct: unexported field tail | 2.8 | 2.2 | 0.586 |
| ns | 3637 |  | 148 | tree.go: Param / Params — the URL-parameter type and its accessors | 2.9 |  | 0.575 |
| walker |  | 3682 | 205 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 0, line: 0 } |  |  | 0.575 |
| walker |  | 3687 | 5 | Code::CodeKey { rung: Decl, file: context.go, decl: 1, sub: 0, line: 31 } |  |  | 0.575 |
| walker |  | 3702 | 15 | Code::CodeKey { rung: Doc, file: context.go, decl: 1, sub: 0, line: 31 } |  |  | 0.575 |
| ns | 3816 |  | 179 | handleHTTPRequest: path resolution before the tree lookup | 2.10 | 2.7 | 0.561 |
| walker |  | 3904 | 202 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 1, line: 0 } |  |  | 0.561 |
| walker |  | 4052 | 148 | Code::CodeKey { rung: Decl, file: context.go, decl: 6, sub: 0, line: 61 } |  |  | 0.562 |
| walker |  | 4063 | 11 | Code::CodeKey { rung: Doc, file: context.go, decl: 10, sub: 0, line: 167 } |  |  | 0.562 |
| walker |  | 4076 | 13 | Code::CodeKey { rung: Doc, file: context.go, decl: 2, sub: 0, line: 47 } |  |  | 0.562 |
| walker |  | 4091 | 15 | Code::CodeKey { rung: Doc, file: context.go, decl: 3, sub: 0, line: 50 } |  |  | 0.562 |
| walker |  | 4108 | 17 | Code::CodeKey { rung: Doc, file: context.go, decl: 13, sub: 0, line: 199 } |  |  | 0.562 |
| ns | 4189 |  | 373 | handleHTTPRequest: the radix lookup, redirects and the 404/405 exits | 2.11 | 2.10 | 0.537 |
| walker |  | 4278 | 170 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 2, line: 0 } |  |  | 0.538 |
| ns | 4438 |  | 249 | tree.go: nodeValue and the complete radix-tree function roster | 2.12 | 2.9 | 0.524 |
| walker |  | 4457 | 179 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 3, line: 0 } |  |  | 0.524 |
| walker |  | 4475 | 18 | Code::CodeKey { rung: Doc, file: context.go, decl: 23, sub: 0, line: 311 } |  |  | 0.524 |
| walker |  | 4493 | 18 | Code::CodeKey { rung: Doc, file: context.go, decl: 24, sub: 0, line: 316 } |  |  | 0.524 |
| walker |  | 4511 | 18 | Code::CodeKey { rung: Doc, file: context.go, decl: 25, sub: 0, line: 321 } |  |  | 0.524 |
| walker |  | 4530 | 19 | Code::CodeKey { rung: Doc, file: context.go, decl: 30, sub: 0, line: 346 } |  |  | 0.524 |
| walker |  | 4551 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 26, sub: 0, line: 326 } |  |  | 0.524 |
| walker |  | 4572 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 27, sub: 0, line: 331 } |  |  | 0.524 |
| walker |  | 4593 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 28, sub: 0, line: 336 } |  |  | 0.524 |
| walker |  | 4614 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 29, sub: 0, line: 341 } |  |  | 0.524 |
| ns | 4658 |  | 220 | path.go: cleanPath semantics and the file's whole function set | 2.13 | 2.11 | 0.515 |
| walker |  | 4799 | 185 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 4, line: 0 } |  |  | 0.515 |
| walker |  | 4816 | 17 | Code::CodeKey { rung: Doc, file: context.go, decl: 37, sub: 0, line: 381 } |  |  | 0.515 |
| walker |  | 4834 | 18 | Code::CodeKey { rung: Doc, file: context.go, decl: 38, sub: 0, line: 386 } |  |  | 0.515 |
| walker |  | 4852 | 18 | Code::CodeKey { rung: Doc, file: context.go, decl: 39, sub: 0, line: 391 } |  |  | 0.515 |
| walker |  | 4872 | 20 | Code::CodeKey { rung: Doc, file: context.go, decl: 35, sub: 0, line: 371 } |  |  | 0.515 |
| ns | 4888 |  | 230 | Trusted-platform constants and the SetTrustedProxies contract | 2.14 | 2.7 | 0.509 |
| walker |  | 4892 | 20 | Code::CodeKey { rung: Doc, file: context.go, decl: 36, sub: 0, line: 376 } |  |  | 0.509 |
| ns | 4977 |  | 89 | Makefile: complete target roster | 3.1 |  | 0.503 |
| ns | 5035 |  | 58 | Makefile variables: which packages are tested and vetted | 3.2 | 3.1 | 0.502 |
| walker |  | 5072 | 180 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 5, line: 0 } |  |  | 0.502 |
| walker |  | 5093 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 40, sub: 0, line: 396 } |  |  | 0.502 |
| walker |  | 5273 | 180 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 6, line: 0 } |  |  | 0.502 |
| walker |  | 5294 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 52, sub: 0, line: 456 } |  |  | 0.502 |
| walker |  | 5315 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 53, sub: 0, line: 461 } |  |  | 0.502 |
| walker |  | 5336 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 54, sub: 0, line: 466 } |  |  | 0.502 |
| walker |  | 5358 | 22 | Code::CodeKey { rung: Doc, file: context.go, decl: 31, sub: 0, line: 351 } |  |  | 0.502 |
| walker |  | 5380 | 22 | Code::CodeKey { rung: Doc, file: context.go, decl: 32, sub: 0, line: 356 } |  |  | 0.502 |
| walker |  | 5402 | 22 | Code::CodeKey { rung: Doc, file: context.go, decl: 33, sub: 0, line: 361 } |  |  | 0.502 |
| walker |  | 5424 | 22 | Code::CodeKey { rung: Doc, file: context.go, decl: 34, sub: 0, line: 366 } |  |  | 0.502 |
| walker |  | 5446 | 22 | Code::CodeKey { rung: Doc, file: context.go, decl: 45, sub: 0, line: 421 } |  |  | 0.502 |
| ns | 5457 |  | 422 | Context struct: all fields with their doc comments | 4.1 |  | 0.486 |
| walker |  | 5468 | 22 | Code::CodeKey { rung: Doc, file: context.go, decl: 55, sub: 0, line: 471 } |  |  | 0.486 |
| walker |  | 5637 | 169 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 7, line: 0 } |  |  | 0.487 |
| ns | 5672 |  | 215 | Context flow control and error attachment: complete roster | 4.2 | 4.1 | 0.493 |
| walker |  | 5808 | 171 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 8, line: 0 } |  |  | 0.494 |
| walker |  | 5824 | 16 | Code::CodeKey { rung: Doc, file: context.go, decl: 65, sub: 0, line: 587 } |  |  | 0.494 |
| walker |  | 5984 | 160 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 9, line: 0 } |  |  | 0.495 |
| walker |  | 6000 | 16 | Code::CodeKey { rung: Doc, file: context.go, decl: 76, sub: 0, line: 719 } |  |  | 0.495 |
| walker |  | 6017 | 17 | Code::CodeKey { rung: Doc, file: context.go, decl: 72, sub: 0, line: 660 } |  |  | 0.495 |
| walker |  | 6034 | 17 | Code::CodeKey { rung: Doc, file: context.go, decl: 74, sub: 0, line: 698 } |  |  | 0.495 |
| ns | 6043 |  | 371 | Context response rendering: complete roster | 4.3 | 4.1 | 0.483 |
| walker |  | 6051 | 17 | Code::CodeKey { rung: Doc, file: context.go, decl: 75, sub: 0, line: 713 } |  |  | 0.483 |
| walker |  | 6248 | 197 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 10, line: 0 } |  |  | 0.483 |
| walker |  | 6268 | 20 | Code::CodeKey { rung: Doc, file: context.go, decl: 78, sub: 0, line: 763 } |  |  | 0.483 |
| walker |  | 6288 | 20 | Code::CodeKey { rung: Doc, file: context.go, decl: 80, sub: 0, line: 773 } |  |  | 0.483 |
| walker |  | 6308 | 20 | Code::CodeKey { rung: Doc, file: context.go, decl: 84, sub: 0, line: 793 } |  |  | 0.483 |
| walker |  | 6329 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 79, sub: 0, line: 768 } |  |  | 0.483 |
| walker |  | 6350 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 83, sub: 0, line: 788 } |  |  | 0.483 |
| walker |  | 6372 | 22 | Code::CodeKey { rung: Doc, file: context.go, decl: 81, sub: 0, line: 778 } |  |  | 0.483 |
| ns | 6468 |  | 425 | Context request binding: complete roster of all 26 entry points | 4.4 | 4.1 | 0.474 |
| walker |  | 6554 | 182 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 11, line: 0 } |  |  | 0.485 |
| walker |  | 6720 | 166 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 12, line: 0 } |  |  | 0.500 |
| walker |  | 6743 | 23 | Code::CodeKey { rung: Doc, file: context.go, decl: 22, sub: 0, line: 296 } |  |  | 0.500 |
| ns | 6809 |  | 341 | Context request input: params, query, form, uploads — complete roster | 4.5 | 4.1 | 0.506 |
| walker |  | 6957 | 214 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 13, line: 0 } |  |  | 0.507 |
| walker |  | 6969 | 12 | Code::CodeKey { rung: Doc, file: context.go, decl: 111, sub: 0, line: 1102 } |  |  | 0.507 |
| walker |  | 6982 | 13 | Code::CodeKey { rung: Doc, file: context.go, decl: 107, sub: 0, line: 1073 } |  |  | 0.507 |
| walker |  | 6995 | 13 | Code::CodeKey { rung: Doc, file: context.go, decl: 110, sub: 0, line: 1094 } |  |  | 0.507 |
| walker |  | 7009 | 14 | Code::CodeKey { rung: Doc, file: context.go, decl: 109, sub: 0, line: 1089 } |  |  | 0.507 |
| walker |  | 7026 | 17 | Code::CodeKey { rung: Doc, file: context.go, decl: 105, sub: 0, line: 1036 } |  |  | 0.507 |
| ns | 7030 |  | 221 | Context headers, cookies and client IP: complete roster | 4.6 | 4.1 | 0.507 |
| ns | 7100 |  | 70 | Context key/value store, with the typed-accessor family marked elided | 4.7 | 4.1 | 0.508 |
| walker |  | 7205 | 179 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 14, line: 0 } |  |  | 0.513 |
| walker |  | 7224 | 19 | Code::CodeKey { rung: Doc, file: context.go, decl: 115, sub: 0, line: 1152 } |  |  | 0.513 |
| ns | 7259 |  | 159 | Content negotiation and Context's context.Context implementation | 4.8 | 4.1 | 0.507 |
| walker |  | 7423 | 199 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 15, line: 0 } |  |  | 0.515 |
| walker |  | 7439 | 16 | Code::CodeKey { rung: Doc, file: context.go, decl: 129, sub: 0, line: 1254 } |  |  | 0.515 |
| walker |  | 7458 | 19 | Code::CodeKey { rung: Doc, file: context.go, decl: 125, sub: 0, line: 1234 } |  |  | 0.515 |
| walker |  | 7477 | 19 | Code::CodeKey { rung: Doc, file: context.go, decl: 128, sub: 0, line: 1249 } |  |  | 0.515 |
| walker |  | 7498 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 126, sub: 0, line: 1239 } |  |  | 0.515 |
| walker |  | 7519 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 127, sub: 0, line: 1244 } |  |  | 0.515 |
| ns | 7557 |  | 298 | Bind vs ShouldBind vs ShouldBindBodyWith: the semantic difference | 4.9 | 4.4 | 0.509 |
| walker |  | 7691 | 172 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 16, line: 0 } |  |  | 0.520 |
| walker |  | 7707 | 16 | Code::CodeKey { rung: Doc, file: context.go, decl: 130, sub: 0, line: 1259 } |  |  | 0.520 |
| ns | 7714 |  | 157 | Middleware chain mechanics: Next() and the abortIndex sentinel | 4.10 | 4.2 | 0.514 |
| walker |  | 7726 | 19 | Code::CodeKey { rung: Doc, file: context.go, decl: 136, sub: 0, line: 1319 } |  |  | 0.514 |
| walker |  | 7746 | 20 | Code::CodeKey { rung: Doc, file: context.go, decl: 131, sub: 0, line: 1268 } |  |  | 0.514 |
| walker |  | 7766 | 20 | Code::CodeKey { rung: Doc, file: context.go, decl: 133, sub: 0, line: 1286 } |  |  | 0.514 |
| walker |  | 7789 | 23 | Code::CodeKey { rung: Doc, file: context.go, decl: 132, sub: 0, line: 1276 } |  |  | 0.514 |
| walker |  | 7813 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 41, sub: 0, line: 401 } |  |  | 0.514 |
| walker |  | 7837 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 42, sub: 0, line: 406 } |  |  | 0.514 |
| walker |  | 7861 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 43, sub: 0, line: 411 } |  |  | 0.514 |
| walker |  | 7885 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 44, sub: 0, line: 416 } |  |  | 0.514 |
| walker |  | 7909 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 46, sub: 0, line: 426 } |  |  | 0.514 |
| walker |  | 7933 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 47, sub: 0, line: 431 } |  |  | 0.514 |
| walker |  | 7957 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 48, sub: 0, line: 436 } |  |  | 0.514 |
| ns | 7959 |  | 245 | binding: the complete MIME constant table | 5.1 |  | 0.507 |
| walker |  | 7981 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 49, sub: 0, line: 441 } |  |  | 0.507 |
| walker |  | 8005 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 50, sub: 0, line: 446 } |  |  | 0.507 |
| walker |  | 8029 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 51, sub: 0, line: 451 } |  |  | 0.507 |
| walker |  | 8053 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 82, sub: 0, line: 783 } |  |  | 0.507 |
| walker |  | 8077 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 98, sub: 0, line: 946 } |  |  | 0.507 |
| walker |  | 8101 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 99, sub: 0, line: 951 } |  |  | 0.507 |
| ns | 8195 |  | 236 | binding: the Binding / BindingBody / BindingUri interfaces | 5.2 | 5.1 | 0.498 |
| walker |  | 8284 | 183 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 17, line: 0 } |  |  | 0.505 |
| walker |  | 8400 | 116 | Code::CodeKey { rung: Decl, file: context.go, decl: 138, sub: 0, line: 1350 } |  |  | 0.506 |
| walker |  | 8413 | 13 | Code::CodeKey { rung: Doc, file: context.go, decl: 138, sub: 0, line: 1350 } |  |  | 0.508 |
| walker |  | 8426 | 13 | Code::CodeKey { rung: Doc, file: context.go, decl: 141, sub: 0, line: 1431 } |  |  | 0.508 |
| ns | 8430 |  | 235 | binding: the registry of concrete binder singletons | 5.3 | 5.2 | 0.501 |
| walker |  | 8441 | 15 | Code::CodeKey { rung: Doc, file: context.go, decl: 139, sub: 0, line: 1364 } |  |  | 0.501 |
| walker |  | 8456 | 15 | Code::CodeKey { rung: Doc, file: context.go, decl: 140, sub: 0, line: 1400 } |  |  | 0.501 |
| walker |  | 8472 | 16 | Code::CodeKey { rung: Doc, file: context.go, decl: 144, sub: 0, line: 1463 } |  |  | 0.501 |
| walker |  | 8495 | 23 | Code::CodeKey { rung: Doc, file: context.go, decl: 143, sub: 0, line: 1455 } |  |  | 0.501 |
| walker |  | 8520 | 25 | Code::CodeKey { rung: Doc, file: context.go, decl: 102, sub: 0, line: 966 } |  |  | 0.501 |
| walker |  | 8545 | 25 | Code::CodeKey { rung: Doc, file: context.go, decl: 142, sub: 0, line: 1447 } |  |  | 0.501 |
| walker |  | 8571 | 26 | Code::CodeKey { rung: Doc, file: context.go, decl: 56, sub: 0, line: 476 } |  |  | 0.501 |
| walker |  | 8597 | 26 | Code::CodeKey { rung: Doc, file: context.go, decl: 100, sub: 0, line: 956 } |  |  | 0.501 |
| walker |  | 8623 | 26 | Code::CodeKey { rung: Doc, file: context.go, decl: 134, sub: 0, line: 1291 } |  |  | 0.501 |
| walker |  | 8650 | 27 | Code::CodeKey { rung: Doc, file: context.go, decl: 104, sub: 0, line: 1027 } |  |  | 0.501 |
| walker |  | 8678 | 28 | Code::CodeKey { rung: Doc, file: context.go, decl: 101, sub: 0, line: 961 } |  |  | 0.501 |
| ns | 8705 |  | 275 | binding.Default(): how a request picks its binder | 5.4 | 5.3 | 0.490 |
| walker |  | 8707 | 29 | Code::CodeKey { rung: Doc, file: context.go, decl: 96, sub: 0, line: 919 } |  |  | 0.490 |
| walker |  | 8739 | 32 | Code::CodeKey { rung: Doc, file: context.go, decl: 137, sub: 0, line: 1328 } |  |  | 0.490 |
| walker |  | 8772 | 33 | Code::CodeKey { rung: Doc, file: context.go, decl: 9, sub: 0, line: 155 } |  |  | 0.490 |
| walker |  | 8806 | 34 | Code::CodeKey { rung: Doc, file: context.go, decl: 106, sub: 0, line: 1042 } |  |  | 0.490 |
| walker |  | 8841 | 35 | Code::CodeKey { rung: Doc, file: context.go, decl: 85, sub: 0, line: 799 } |  |  | 0.490 |
| walker |  | 8876 | 35 | Code::CodeKey { rung: Doc, file: context.go, decl: 124, sub: 0, line: 1229 } |  |  | 0.490 |
| walker |  | 8912 | 36 | Code::CodeKey { rung: Doc, file: context.go, decl: 120, sub: 0, line: 1205 } |  |  | 0.490 |
| walker |  | 8948 | 36 | Code::CodeKey { rung: Doc, file: context.go, decl: 123, sub: 0, line: 1223 } |  |  | 0.490 |
| walker |  | 8985 | 37 | Code::CodeKey { rung: Doc, file: context.go, decl: 95, sub: 0, line: 909 } |  |  | 0.490 |
| walker |  | 9023 | 38 | Code::CodeKey { rung: Doc, file: context.go, decl: 20, sub: 0, line: 276 } |  |  | 0.490 |
| walker |  | 9061 | 38 | Code::CodeKey { rung: Doc, file: context.go, decl: 57, sub: 0, line: 482 } |  |  | 0.490 |
| walker |  | 9099 | 38 | Code::CodeKey { rung: Doc, file: context.go, decl: 66, sub: 0, line: 594 } |  |  | 0.490 |
| walker |  | 9138 | 39 | Code::CodeKey { rung: Doc, file: context.go, decl: 21, sub: 0, line: 288 } |  |  | 0.490 |
| ns | 9150 |  | 445 | render: the Render interface and every implementation | 5.5 |  | 0.478 |
| walker |  | 9177 | 39 | Code::CodeKey { rung: Doc, file: context.go, decl: 63, sub: 0, line: 563 } |  |  | 0.478 |
| walker |  | 9216 | 39 | Code::CodeKey { rung: Doc, file: context.go, decl: 67, sub: 0, line: 601 } |  |  | 0.478 |
| walker |  | 9255 | 39 | Code::CodeKey { rung: Doc, file: context.go, decl: 73, sub: 0, line: 667 } |  |  | 0.478 |
| walker |  | 9294 | 39 | Code::CodeKey { rung: Doc, file: context.go, decl: 94, sub: 0, line: 903 } |  |  | 0.478 |
| ns | 9312 |  | 162 | Every build-tag line in the repository | 5.6 | 5.4 | 0.475 |
| walker |  | 9334 | 40 | Code::CodeKey { rung: Doc, file: context.go, decl: 64, sub: 0, line: 580 } |  |  | 0.475 |
| walker |  | 9374 | 40 | Code::CodeKey { rung: Doc, file: context.go, decl: 70, sub: 0, line: 633 } |  |  | 0.475 |
| walker |  | 9414 | 40 | Code::CodeKey { rung: Doc, file: context.go, decl: 90, sub: 0, line: 879 } |  |  | 0.475 |
| ns | 9452 |  | 140 | binding: StructValidator and the default validator hook | 5.7 | 5.2 | 0.471 |
| walker |  | 9455 | 41 | Code::CodeKey { rung: Doc, file: context.go, decl: 8, sub: 0, line: 149 } |  |  | 0.471 |
| walker |  | 9496 | 41 | Code::CodeKey { rung: Doc, file: context.go, decl: 71, sub: 0, line: 653 } |  |  | 0.471 |
| walker |  | 9537 | 41 | Code::CodeKey { rung: Doc, file: context.go, decl: 89, sub: 0, line: 873 } |  |  | 0.471 |
| walker |  | 9579 | 42 | Code::CodeKey { rung: Doc, file: context.go, decl: 12, sub: 0, line: 188 } |  |  | 0.473 |
| walker |  | 9622 | 43 | Code::CodeKey { rung: Doc, file: context.go, decl: 91, sub: 0, line: 885 } |  |  | 0.473 |
| ns | 9624 |  | 172 | Run modes: GIN_MODE, the three mode constants, and the default writers | 6.1 |  | 0.468 |
| walker |  | 9665 | 43 | Code::CodeKey { rung: Doc, file: context.go, decl: 93, sub: 0, line: 897 } |  |  | 0.468 |
| ns | 9695 |  | 71 | mode.go: complete function roster | 6.2 | 6.1 | 0.466 |
| walker |  | 9708 | 43 | Code::CodeKey { rung: Doc, file: context.go, decl: 121, sub: 0, line: 1211 } |  |  | 0.466 |
| walker |  | 9751 | 43 | Code::CodeKey { rung: Doc, file: context.go, decl: 122, sub: 0, line: 1217 } |  |  | 0.466 |
| walker |  | 9794 | 43 | Code::CodeKey { rung: Doc, file: context.go, decl: 135, sub: 0, line: 1309 } |  |  | 0.466 |
| ns | 9972 |  | 277 | Built-in middleware and helper constructors: complete package-level roster | 6.3 |  | 0.459 |
| walker |  | 9999 | 205 | Code::CodeKey { rung: Names, file: errors.go, decl: 0, sub: 0, line: 0 } |  |  | 0.459 |
