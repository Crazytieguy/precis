Score(3000)=0.584 I=0.634 C=0.538 ns_rows≤3K=15/45 grid(1000/1442/2080/3000/4327/6240/9000)=0.756/0.723/0.630/0.584/0.546/0.559/0.532

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
| walker |  | 1796 | 255 | Code::CodeKey { rung: Names, file: gin.go, decl: 0, sub: 0, line: 0 } |  |  | 0.658 |
| walker |  | 1805 | 9 | Code::CodeKey { rung: Decl, file: gin.go, decl: 2, sub: 0, line: 32 } |  |  | 0.658 |
| walker |  | 1814 | 9 | Code::CodeKey { rung: Decl, file: gin.go, decl: 1, sub: 0, line: 25 } |  |  | 0.658 |
| walker |  | 1854 | 40 | Code::CodeKey { rung: Decl, file: gin.go, decl: 9, sub: 0, line: 68 } |  |  | 0.665 |
| walker |  | 1959 | 105 | Code::CodeKey { rung: Decl, file: gin.go, decl: 11, sub: 0, line: 79 } |  |  | 0.665 |
| walker |  | 1965 | 6 | Code::CodeKey { rung: Doc, file: gin.go, decl: 11, sub: 0, line: 79 } |  |  | 0.666 |
| walker |  | 1976 | 11 | Code::CodeKey { rung: Doc, file: gin.go, decl: 10, sub: 0, line: 76 } |  |  | 0.669 |
| walker |  | 1988 | 12 | Code::CodeKey { rung: Doc, file: gin.go, decl: 7, sub: 0, line: 57 } |  |  | 0.672 |
| ns | 2020 |  | 336 | IRoutes / IRouter: the complete route-registration interface | 2.3 |  | 0.630 |
| walker |  | 2284 | 296 | Code::CodeKey { rung: Decl, file: gin.go, decl: 4, sub: 0, line: 39 } |  |  | 0.630 |
| walker |  | 2298 | 14 | Code::CodeKey { rung: Doc, file: gin.go, decl: 6, sub: 0, line: 54 } |  |  | 0.636 |
| walker |  | 2314 | 16 | Code::CodeKey { rung: Doc, file: gin.go, decl: 5, sub: 0, line: 51 } |  |  | 0.642 |
| walker |  | 2336 | 22 | Code::CodeKey { rung: Doc, file: gin.go, decl: 9, sub: 0, line: 68 } |  |  | 0.650 |
| walker |  | 2359 | 23 | Code::CodeKey { rung: Doc, file: gin.go, decl: 8, sub: 0, line: 60 } |  |  | 0.650 |
| ns | 2441 |  | 421 | Engine constructor New(): every default value | 2.4 | 2.2 | 0.608 |
| walker |  | 2563 | 204 | Code::CodeKey { rung: Names, file: gin.go, decl: 0, sub: 1, line: 0 } |  |  | 0.608 |
| walker |  | 2582 | 19 | Code::CodeKey { rung: Doc, file: gin.go, decl: 15, sub: 0, line: 236 } |  |  | 0.608 |
| walker |  | 2602 | 20 | Code::CodeKey { rung: Doc, file: gin.go, decl: 18, sub: 0, line: 259 } |  |  | 0.608 |
| walker |  | 2622 | 20 | Code::CodeKey { rung: Doc, file: gin.go, decl: 19, sub: 0, line: 265 } |  |  | 0.608 |
| ns | 2634 |  | 193 | Default(), RouterGroup struct, and the IRouter assertions | 2.5 | 2.3 | 0.589 |
| walker |  | 2814 | 192 | Code::CodeKey { rung: Decl, file: gin.go, decl: 12, sub: 0, line: 92 } |  |  | 0.591 |
| ns | 2829 |  | 195 | Run* server entry points: all six transports | 2.6 |  | 0.583 |
| walker |  | 2995 | 181 | Code::CodeKey { rung: Decl, file: gin.go, decl: 12, sub: 1, line: 92 } |  |  | 0.584 |
| walker |  | 3212 | 217 | Code::CodeKey { rung: Decl, file: gin.go, decl: 12, sub: 2, line: 92 } |  |  | 0.589 |
| ns | 3333 |  | 504 | gin.go: complete function roster beyond the constructors and Run* | 2.7 | 2.5 | 0.556 |
| walker |  | 3390 | 178 | Code::CodeKey { rung: Decl, file: gin.go, decl: 12, sub: 3, line: 92 } |  |  | 0.561 |
| ns | 3489 |  | 156 | Engine struct: unexported field tail | 2.8 | 2.2 | 0.548 |
| walker |  | 3582 | 192 | Code::CodeKey { rung: Decl, file: gin.go, decl: 12, sub: 4, line: 92 } |  |  | 0.558 |
| ns | 3637 |  | 148 | tree.go: Param / Params — the URL-parameter type and its accessors | 2.9 |  | 0.547 |
| walker |  | 3770 | 188 | Code::CodeKey { rung: Names, file: gin.go, decl: 0, sub: 2, line: 0 } |  |  | 0.558 |
| walker |  | 3785 | 15 | Code::CodeKey { rung: Doc, file: gin.go, decl: 23, sub: 0, line: 312 } |  |  | 0.558 |
| walker |  | 3803 | 18 | Code::CodeKey { rung: Doc, file: gin.go, decl: 24, sub: 0, line: 321 } |  |  | 0.558 |
| ns | 3816 |  | 179 | handleHTTPRequest: path resolution before the tree lookup | 2.10 | 2.7 | 0.544 |
| walker |  | 3821 | 18 | Code::CodeKey { rung: Doc, file: gin.go, decl: 28, sub: 0, line: 348 } |  |  | 0.544 |
| walker |  | 3841 | 20 | Code::CodeKey { rung: Doc, file: gin.go, decl: 26, sub: 0, line: 332 } |  |  | 0.544 |
| walker |  | 3863 | 22 | Code::CodeKey { rung: Doc, file: gin.go, decl: 25, sub: 0, line: 326 } |  |  | 0.544 |
| walker |  | 4044 | 181 | Code::CodeKey { rung: Names, file: gin.go, decl: 0, sub: 3, line: 0 } |  |  | 0.561 |
| walker |  | 4066 | 22 | Code::CodeKey { rung: Doc, file: gin.go, decl: 37, sub: 0, line: 462 } |  |  | 0.561 |
| ns | 4189 |  | 373 | handleHTTPRequest: the radix lookup, redirects and the 404/405 exits | 2.11 | 2.10 | 0.535 |
| walker |  | 4236 | 170 | Code::CodeKey { rung: Names, file: gin.go, decl: 0, sub: 4, line: 0 } |  |  | 0.546 |
| walker |  | 4251 | 15 | Code::CodeKey { rung: Doc, file: gin.go, decl: 41, sub: 0, line: 517 } |  |  | 0.546 |
| walker |  | 4267 | 16 | Code::CodeKey { rung: Doc, file: gin.go, decl: 40, sub: 0, line: 504 } |  |  | 0.546 |
| ns | 4438 |  | 249 | tree.go: nodeValue and the complete radix-tree function roster | 2.12 | 2.9 | 0.532 |
| walker |  | 4474 | 207 | Code::CodeKey { rung: Decl, file: gin.go, decl: 12, sub: 5, line: 92 } |  |  | 0.543 |
| ns | 4658 |  | 220 | path.go: cleanPath semantics and the file's whole function set | 2.13 | 2.11 | 0.534 |
| walker |  | 4670 | 196 | Code::CodeKey { rung: Names, file: gin.go, decl: 0, sub: 5, line: 0 } |  |  | 0.555 |
| walker |  | 4684 | 14 | Code::CodeKey { rung: Doc, file: gin.go, decl: 49, sub: 0, line: 662 } |  |  | 0.555 |
| walker |  | 4708 | 24 | Code::CodeKey { rung: Doc, file: gin.go, decl: 39, sub: 0, line: 482 } |  |  | 0.555 |
| ns | 4888 |  | 230 | Trusted-platform constants and the SetTrustedProxies contract | 2.14 | 2.7 | 0.547 |
| walker |  | 4945 | 237 | Code::CodeKey { rung: Decl, file: gin.go, decl: 12, sub: 6, line: 92 } |  |  | 0.575 |
| walker |  | 4973 | 28 | Code::CodeKey { rung: Doc, file: gin.go, decl: 21, sub: 0, line: 288 } |  |  | 0.575 |
| ns | 4977 |  | 89 | Makefile: complete target roster | 3.1 |  | 0.568 |
| walker |  | 5001 | 28 | Code::CodeKey { rung: Doc, file: gin.go, decl: 38, sub: 0, line: 469 } |  |  | 0.568 |
| walker |  | 5030 | 29 | Code::CodeKey { rung: Doc, file: gin.go, decl: 20, sub: 0, line: 272 } |  |  | 0.568 |
| ns | 5035 |  | 58 | Makefile variables: which packages are tested and vetted | 3.2 | 3.1 | 0.567 |
| walker |  | 5338 | 308 | GoMod::File { file: go.mod } |  |  | 0.567 |
| walker |  | 5370 | 32 | Code::CodeKey { rung: Doc, file: gin.go, decl: 22, sub: 0, line: 300 } |  |  | 0.567 |
| walker |  | 5402 | 32 | Code::CodeKey { rung: Doc, file: gin.go, decl: 36, sub: 0, line: 457 } |  |  | 0.567 |
| ns | 5457 |  | 422 | Context struct: all fields with their doc comments | 4.1 |  | 0.543 |
| walker |  | 5583 | 181 | Code::CodeKey { rung: Names, file: routergroup.go, decl: 0, sub: 0, line: 0 } |  |  | 0.545 |
| walker |  | 5611 | 28 | Code::CodeKey { rung: Decl, file: routergroup.go, decl: 2, sub: 0, line: 27 } |  |  | 0.545 |
| walker |  | 5649 | 38 | Code::CodeKey { rung: Decl, file: routergroup.go, decl: 4, sub: 0, line: 55 } |  |  | 0.550 |
| ns | 5672 |  | 215 | Context flow control and error attachment: complete roster | 4.2 | 4.1 | 0.541 |
| walker |  | 5751 | 102 | Code::CodeKey { rung: Decl, file: routergroup.go, decl: 1, sub: 0, line: 14 } |  |  | 0.541 |
| walker |  | 5996 | 245 | Code::CodeKey { rung: Decl, file: routergroup.go, decl: 3, sub: 0, line: 33 } |  |  | 0.566 |
| walker |  | 6007 | 11 | Code::CodeKey { rung: Doc, file: routergroup.go, decl: 3, sub: 0, line: 33 } |  |  | 0.569 |
| walker |  | 6023 | 16 | Code::CodeKey { rung: Doc, file: routergroup.go, decl: 2, sub: 0, line: 27 } |  |  | 0.573 |
| walker |  | 6040 | 17 | Code::CodeKey { rung: Doc, file: routergroup.go, decl: 6, sub: 0, line: 65 } |  |  | 0.573 |
| ns | 6043 |  | 371 | Context response rendering: complete roster | 4.3 | 4.1 | 0.559 |
| walker |  | 6239 | 199 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 0, line: 0 } |  |  | 0.559 |
| walker |  | 6250 | 11 | Code::CodeKey { rung: Decl, file: context.go, decl: 1, sub: 0, line: 31 } |  |  | 0.559 |
| walker |  | 6265 | 15 | Code::CodeKey { rung: Doc, file: context.go, decl: 1, sub: 0, line: 31 } |  |  | 0.559 |
| ns | 6468 |  | 425 | Context request binding: complete roster of all 26 entry points | 4.4 | 4.1 | 0.545 |
| walker |  | 6469 | 204 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 1, line: 0 } |  |  | 0.548 |
| walker |  | 6480 | 11 | Code::CodeKey { rung: Doc, file: context.go, decl: 12, sub: 0, line: 167 } |  |  | 0.548 |
| ns | 6809 |  | 341 | Context request input: params, query, form, uploads — complete roster | 4.5 | 4.1 | 0.536 |
| walker |  | 6867 | 387 | Code::CodeKey { rung: Decl, file: context.go, decl: 7, sub: 0, line: 61 } |  |  | 0.572 |
| walker |  | 6880 | 13 | Code::CodeKey { rung: Doc, file: context.go, decl: 2, sub: 0, line: 47 } |  |  | 0.572 |
| walker |  | 6894 | 14 | Code::CodeKey { rung: Doc, file: context.go, decl: 6, sub: 0, line: 57 } |  |  | 0.572 |
| walker |  | 6909 | 15 | Code::CodeKey { rung: Doc, file: context.go, decl: 3, sub: 0, line: 50 } |  |  | 0.572 |
| ns | 7030 |  | 221 | Context headers, cookies and client IP: complete roster | 4.6 | 4.1 | 0.565 |
| walker |  | 7088 | 179 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 2, line: 0 } |  |  | 0.577 |
| ns | 7100 |  | 70 | Context key/value store, with the typed-accessor family marked elided | 4.7 | 4.1 | 0.574 |
| walker |  | 7105 | 17 | Code::CodeKey { rung: Doc, file: context.go, decl: 15, sub: 0, line: 199 } |  |  | 0.574 |
| ns | 7259 |  | 159 | Content negotiation and Context's context.Context implementation | 4.8 | 4.1 | 0.568 |
| walker |  | 7292 | 187 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 3, line: 0 } |  |  | 0.571 |
| walker |  | 7310 | 18 | Code::CodeKey { rung: Doc, file: context.go, decl: 26, sub: 0, line: 311 } |  |  | 0.571 |
| walker |  | 7328 | 18 | Code::CodeKey { rung: Doc, file: context.go, decl: 27, sub: 0, line: 316 } |  |  | 0.571 |
| walker |  | 7346 | 18 | Code::CodeKey { rung: Doc, file: context.go, decl: 28, sub: 0, line: 321 } |  |  | 0.571 |
| walker |  | 7367 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 29, sub: 0, line: 326 } |  |  | 0.571 |
| walker |  | 7388 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 30, sub: 0, line: 331 } |  |  | 0.571 |
| walker |  | 7409 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 31, sub: 0, line: 336 } |  |  | 0.571 |
| walker |  | 7432 | 23 | Code::CodeKey { rung: Doc, file: context.go, decl: 24, sub: 0, line: 296 } |  |  | 0.571 |
| ns | 7557 |  | 298 | Bind vs ShouldBind vs ShouldBindBodyWith: the semantic difference | 4.9 | 4.4 | 0.564 |
| walker |  | 7618 | 186 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 4, line: 0 } |  |  | 0.564 |
| walker |  | 7635 | 17 | Code::CodeKey { rung: Doc, file: context.go, decl: 40, sub: 0, line: 381 } |  |  | 0.564 |
| walker |  | 7654 | 19 | Code::CodeKey { rung: Doc, file: context.go, decl: 33, sub: 0, line: 346 } |  |  | 0.564 |
| walker |  | 7674 | 20 | Code::CodeKey { rung: Doc, file: context.go, decl: 38, sub: 0, line: 371 } |  |  | 0.564 |
| walker |  | 7694 | 20 | Code::CodeKey { rung: Doc, file: context.go, decl: 39, sub: 0, line: 376 } |  |  | 0.564 |
| ns | 7714 |  | 157 | Middleware chain mechanics: Next() and the abortIndex sentinel | 4.10 | 4.2 | 0.558 |
| walker |  | 7715 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 32, sub: 0, line: 341 } |  |  | 0.558 |
| walker |  | 7737 | 22 | Code::CodeKey { rung: Doc, file: context.go, decl: 34, sub: 0, line: 351 } |  |  | 0.558 |
| walker |  | 7759 | 22 | Code::CodeKey { rung: Doc, file: context.go, decl: 35, sub: 0, line: 356 } |  |  | 0.558 |
| walker |  | 7781 | 22 | Code::CodeKey { rung: Doc, file: context.go, decl: 36, sub: 0, line: 361 } |  |  | 0.558 |
| walker |  | 7803 | 22 | Code::CodeKey { rung: Doc, file: context.go, decl: 37, sub: 0, line: 366 } |  |  | 0.558 |
| ns | 7959 |  | 245 | binding: the complete MIME constant table | 5.1 |  | 0.550 |
| walker |  | 7976 | 173 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 5, line: 0 } |  |  | 0.550 |
| walker |  | 7994 | 18 | Code::CodeKey { rung: Doc, file: context.go, decl: 41, sub: 0, line: 386 } |  |  | 0.550 |
| walker |  | 8012 | 18 | Code::CodeKey { rung: Doc, file: context.go, decl: 42, sub: 0, line: 391 } |  |  | 0.550 |
| walker |  | 8033 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 43, sub: 0, line: 396 } |  |  | 0.550 |
| walker |  | 8055 | 22 | Code::CodeKey { rung: Doc, file: context.go, decl: 48, sub: 0, line: 421 } |  |  | 0.550 |
| walker |  | 8079 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 44, sub: 0, line: 401 } |  |  | 0.550 |
| walker |  | 8103 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 45, sub: 0, line: 406 } |  |  | 0.550 |
| walker |  | 8127 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 46, sub: 0, line: 411 } |  |  | 0.550 |
| walker |  | 8151 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 47, sub: 0, line: 416 } |  |  | 0.550 |
| ns | 8195 |  | 236 | binding: the Binding / BindingBody / BindingUri interfaces | 5.2 | 5.1 | 0.541 |
| walker |  | 8331 | 180 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 6, line: 0 } |  |  | 0.541 |
| walker |  | 8352 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 55, sub: 0, line: 456 } |  |  | 0.541 |
| walker |  | 8373 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 56, sub: 0, line: 461 } |  |  | 0.541 |
| walker |  | 8397 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 49, sub: 0, line: 426 } |  |  | 0.541 |
| walker |  | 8421 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 50, sub: 0, line: 431 } |  |  | 0.541 |
| ns | 8430 |  | 235 | binding: the registry of concrete binder singletons | 5.3 | 5.2 | 0.533 |
| walker |  | 8445 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 51, sub: 0, line: 436 } |  |  | 0.533 |
| walker |  | 8469 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 52, sub: 0, line: 441 } |  |  | 0.533 |
| walker |  | 8493 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 53, sub: 0, line: 446 } |  |  | 0.533 |
| walker |  | 8517 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 54, sub: 0, line: 451 } |  |  | 0.533 |
| walker |  | 8686 | 169 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 7, line: 0 } |  |  | 0.535 |
| ns | 8705 |  | 275 | binding.Default(): how a request picks its binder | 5.4 | 5.3 | 0.524 |
| walker |  | 8707 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 57, sub: 0, line: 466 } |  |  | 0.524 |
| walker |  | 8729 | 22 | Code::CodeKey { rung: Doc, file: context.go, decl: 58, sub: 0, line: 471 } |  |  | 0.524 |
| walker |  | 8892 | 163 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 8, line: 0 } |  |  | 0.528 |
| walker |  | 8908 | 16 | Code::CodeKey { rung: Doc, file: context.go, decl: 69, sub: 0, line: 587 } |  |  | 0.528 |
| walker |  | 9077 | 169 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 9, line: 0 } |  |  | 0.536 |
| walker |  | 9094 | 17 | Code::CodeKey { rung: Doc, file: context.go, decl: 77, sub: 0, line: 660 } |  |  | 0.536 |
| ns | 9150 |  | 445 | render: the Render interface and every implementation | 5.5 |  | 0.523 |
| walker |  | 9258 | 164 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 10, line: 0 } |  |  | 0.530 |
| walker |  | 9274 | 16 | Code::CodeKey { rung: Doc, file: context.go, decl: 82, sub: 0, line: 719 } |  |  | 0.530 |
| walker |  | 9291 | 17 | Code::CodeKey { rung: Doc, file: context.go, decl: 80, sub: 0, line: 698 } |  |  | 0.530 |
| walker |  | 9308 | 17 | Code::CodeKey { rung: Doc, file: context.go, decl: 81, sub: 0, line: 713 } |  |  | 0.530 |
| ns | 9312 |  | 162 | Every build-tag line in the repository | 5.6 | 5.4 | 0.526 |
| walker |  | 9328 | 20 | Code::CodeKey { rung: Doc, file: context.go, decl: 84, sub: 0, line: 763 } |  |  | 0.526 |
| walker |  | 9349 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 85, sub: 0, line: 768 } |  |  | 0.526 |
| walker |  | 9375 | 26 | Code::CodeKey { rung: Doc, file: context.go, decl: 59, sub: 0, line: 476 } |  |  | 0.526 |
| ns | 9452 |  | 140 | binding: StructValidator and the default validator hook | 5.7 | 5.2 | 0.521 |
| walker |  | 9555 | 180 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 11, line: 0 } |  |  | 0.525 |
| walker |  | 9575 | 20 | Code::CodeKey { rung: Doc, file: context.go, decl: 86, sub: 0, line: 773 } |  |  | 0.525 |
| walker |  | 9595 | 20 | Code::CodeKey { rung: Doc, file: context.go, decl: 90, sub: 0, line: 793 } |  |  | 0.525 |
| walker |  | 9616 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 89, sub: 0, line: 788 } |  |  | 0.525 |
| ns | 9624 |  | 172 | Run modes: GIN_MODE, the three mode constants, and the default writers | 6.1 |  | 0.520 |
| walker |  | 9638 | 22 | Code::CodeKey { rung: Doc, file: context.go, decl: 87, sub: 0, line: 778 } |  |  | 0.520 |
| walker |  | 9662 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 88, sub: 0, line: 783 } |  |  | 0.520 |
| ns | 9695 |  | 71 | mode.go: complete function roster | 6.2 | 6.1 | 0.518 |
| walker |  | 9858 | 196 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 12, line: 0 } |  |  | 0.527 |
| ns | 9972 |  | 277 | Built-in middleware and helper constructors: complete package-level roster | 6.3 |  | 0.518 |
| walker |  | 9988 | 130 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 13, line: 0 } |  |  | 0.527 |
