Score(3000)=0.548 I=0.622 C=0.482 ns_rows≤3K=15/45 grid(1000/1442/2080/3000/4327/6240/9000)=0.756/0.723/0.616/0.548/0.461/0.449/0.443

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
| walker |  | 1566 | 25 | Code::CodeKey { rung: Names, file: context_appengine.go, decl: 0, sub: 0, line: 0 } |  |  | 0.723 |
| walker |  | 1578 | 12 | Code::CodeKey { rung: Body, file: context_appengine.go, decl: 1, sub: 0, line: 9 } |  |  | 0.723 |
| ns | 1684 |  | 250 | Engine struct: complete exported configuration field roster | 2.2 |  | 0.655 |
| walker |  | 1793 | 215 | Code::CodeKey { rung: Names, file: mode.go, decl: 0, sub: 0, line: 0 } |  |  | 0.656 |
| walker |  | 1800 | 7 | Code::CodeKey { rung: Decl, file: mode.go, decl: 6, sub: 0, line: 47 } |  |  | 0.656 |
| walker |  | 1811 | 11 | Code::CodeKey { rung: Decl, file: mode.go, decl: 3, sub: 0, line: 28 } |  |  | 0.656 |
| walker |  | 1856 | 45 | Code::CodeKey { rung: Decl, file: mode.go, decl: 2, sub: 0, line: 19 } |  |  | 0.656 |
| walker |  | 1866 | 10 | Code::CodeKey { rung: Body, file: mode.go, decl: 9, sub: 0, line: 81 } |  |  | 0.656 |
| walker |  | 1877 | 11 | Code::CodeKey { rung: Doc, file: mode.go, decl: 12, sub: 0, line: 98 } |  |  | 0.656 |
| walker |  | 1890 | 13 | Code::CodeKey { rung: Doc, file: mode.go, decl: 9, sub: 0, line: 81 } |  |  | 0.656 |
| walker |  | 1905 | 15 | Code::CodeKey { rung: Doc, file: mode.go, decl: 1, sub: 0, line: 17 } |  |  | 0.657 |
| walker |  | 1920 | 15 | Code::CodeKey { rung: Doc, file: mode.go, decl: 8, sub: 0, line: 58 } |  |  | 0.657 |
| walker |  | 1938 | 18 | Code::CodeKey { rung: Doc, file: mode.go, decl: 5, sub: 0, line: 45 } |  |  | 0.657 |
| ns | 2020 |  | 336 | IRoutes / IRouter: the complete route-registration interface | 2.3 |  | 0.616 |
| walker |  | 2126 | 188 | Code::CodeKey { rung: Names, file: logger.go, decl: 0, sub: 0, line: 0 } |  |  | 0.616 |
| walker |  | 2135 | 9 | Code::CodeKey { rung: Decl, file: logger.go, decl: 2, sub: 0, line: 19 } |  |  | 0.616 |
| walker |  | 2142 | 7 | Code::CodeKey { rung: Decl, file: logger.go, decl: 3, sub: 0, line: 25 } |  |  | 0.616 |
| walker |  | 2438 | 296 | Code::CodeKey { rung: Names, file: logger.go, decl: 0, sub: 1, line: 0 } |  |  | 0.616 |
| ns | 2441 |  | 421 | Engine constructor New(): every default value | 2.4 | 2.2 | 0.576 |
| walker |  | 2445 | 7 | Code::CodeKey { rung: Body, file: logger.go, decl: 12, sub: 0, line: 157 } |  |  | 0.576 |
| ns | 2634 |  | 193 | Default(), RouterGroup struct, and the IRouter assertions | 2.5 | 2.3 | 0.556 |
| walker |  | 2667 | 222 | Code::CodeKey { rung: Decl, file: logger.go, decl: 5, sub: 0, line: 39 } |  |  | 0.556 |
| walker |  | 2679 | 12 | Code::CodeKey { rung: Doc, file: logger.go, decl: 5, sub: 0, line: 39 } |  |  | 0.556 |
| walker |  | 2691 | 12 | Code::CodeKey { rung: Doc, file: logger.go, decl: 12, sub: 0, line: 157 } |  |  | 0.556 |
| ns | 2829 |  | 195 | Run* server entry points: all six transports | 2.6 |  | 0.548 |
| walker |  | 2963 | 272 | Code::CodeKey { rung: Decl, file: logger.go, decl: 8, sub: 0, line: 68 } |  |  | 0.548 |
| walker |  | 2978 | 15 | Code::CodeKey { rung: Doc, file: logger.go, decl: 10, sub: 0, line: 112 } |  |  | 0.548 |
| walker |  | 2993 | 15 | Code::CodeKey { rung: Doc, file: logger.go, decl: 15, sub: 0, line: 197 } |  |  | 0.548 |
| walker |  | 3008 | 15 | Code::CodeKey { rung: Doc, file: logger.go, decl: 16, sub: 0, line: 202 } |  |  | 0.548 |
| walker |  | 3023 | 15 | Code::CodeKey { rung: Doc, file: logger.go, decl: 22, sub: 0, line: 245 } |  |  | 0.548 |
| ns | 3333 |  | 504 | gin.go: complete function roster beyond the constructors and Run* | 2.7 | 2.5 | 0.516 |
| walker |  | 3380 | 357 | Code::CodeKey { rung: Decl, file: logger.go, decl: 14, sub: 0, line: 167 } |  |  | 0.516 |
| walker |  | 3396 | 16 | Code::CodeKey { rung: Doc, file: logger.go, decl: 6, sub: 0, line: 62 } |  |  | 0.516 |
| walker |  | 3412 | 16 | Code::CodeKey { rung: Doc, file: logger.go, decl: 17, sub: 0, line: 207 } |  |  | 0.516 |
| walker |  | 3430 | 18 | Code::CodeKey { rung: Doc, file: logger.go, decl: 7, sub: 0, line: 65 } |  |  | 0.516 |
| walker |  | 3448 | 18 | Code::CodeKey { rung: Doc, file: logger.go, decl: 14, sub: 0, line: 167 } |  |  | 0.516 |
| walker |  | 3466 | 18 | Code::CodeKey { rung: Doc, file: logger.go, decl: 18, sub: 0, line: 212 } |  |  | 0.516 |
| walker |  | 3485 | 19 | Code::CodeKey { rung: Doc, file: logger.go, decl: 13, sub: 0, line: 162 } |  |  | 0.516 |
| ns | 3489 |  | 156 | Engine struct: unexported field tail | 2.8 | 2.2 | 0.504 |
| walker |  | 3504 | 19 | Code::CodeKey { rung: Doc, file: logger.go, decl: 20, sub: 0, line: 229 } |  |  | 0.504 |
| walker |  | 3524 | 20 | Code::CodeKey { rung: Doc, file: logger.go, decl: 8, sub: 0, line: 68 } |  |  | 0.504 |
| walker |  | 3544 | 20 | Code::CodeKey { rung: Doc, file: logger.go, decl: 9, sub: 0, line: 94 } |  |  | 0.504 |
| walker |  | 3564 | 20 | Code::CodeKey { rung: Doc, file: logger.go, decl: 11, sub: 0, line: 133 } |  |  | 0.504 |
| ns | 3637 |  | 148 | tree.go: Param / Params — the URL-parameter type and its accessors | 2.9 |  | 0.495 |
| walker |  | 3778 | 214 | Code::CodeKey { rung: Names, file: recovery.go, decl: 0, sub: 0, line: 0 } |  |  | 0.495 |
| walker |  | 3789 | 11 | Code::CodeKey { rung: Decl, file: recovery.go, decl: 1, sub: 0, line: 26 } |  |  | 0.495 |
| walker |  | 3801 | 12 | Code::CodeKey { rung: Body, file: recovery.go, decl: 3, sub: 0, line: 35 } |  |  | 0.495 |
| walker |  | 3815 | 14 | Code::CodeKey { rung: Doc, file: recovery.go, decl: 2, sub: 0, line: 32 } |  |  | 0.495 |
| ns | 3816 |  | 179 | handleHTTPRequest: path resolution before the tree lookup | 2.10 | 2.7 | 0.483 |
| walker |  | 3829 | 14 | Code::CodeKey { rung: Body, file: recovery.go, decl: 4, sub: 0, line: 40 } |  |  | 0.483 |
| walker |  | 3844 | 15 | Code::CodeKey { rung: Doc, file: recovery.go, decl: 12, sub: 0, line: 197 } |  |  | 0.483 |
| walker |  | 3861 | 17 | Code::CodeKey { rung: Doc, file: recovery.go, decl: 9, sub: 0, line: 114 } |  |  | 0.483 |
| walker |  | 3881 | 20 | Code::CodeKey { rung: Doc, file: recovery.go, decl: 11, sub: 0, line: 172 } |  |  | 0.483 |
| ns | 4189 |  | 373 | handleHTTPRequest: the radix lookup, redirects and the 404/405 exits | 2.11 | 2.10 | 0.461 |
| walker |  | 4195 | 314 | Code::CodeKey { rung: Names, file: utils.go, decl: 0, sub: 0, line: 0 } |  |  | 0.461 |
| walker |  | 4207 | 12 | Code::CodeKey { rung: Doc, file: utils.go, decl: 2, sub: 0, line: 23 } |  |  | 0.461 |
| walker |  | 4220 | 13 | Code::CodeKey { rung: Doc, file: utils.go, decl: 1, sub: 0, line: 20 } |  |  | 0.461 |
| walker |  | 4234 | 14 | Code::CodeKey { rung: Doc, file: utils.go, decl: 3, sub: 0, line: 26 } |  |  | 0.461 |
| walker |  | 4249 | 15 | Code::CodeKey { rung: Doc, file: utils.go, decl: 7, sub: 0, line: 61 } |  |  | 0.461 |
| walker |  | 4264 | 15 | Code::CodeKey { rung: Doc, file: utils.go, decl: 8, sub: 0, line: 64 } |  |  | 0.461 |
| walker |  | 4282 | 18 | Code::CodeKey { rung: Doc, file: utils.go, decl: 4, sub: 0, line: 29 } |  |  | 0.461 |
| walker |  | 4303 | 21 | Code::CodeKey { rung: Doc, file: utils.go, decl: 6, sub: 0, line: 54 } |  |  | 0.461 |
| ns | 4438 |  | 249 | tree.go: nodeValue and the complete radix-tree function roster | 2.12 | 2.9 | 0.449 |
| walker |  | 4487 | 184 | Code::CodeKey { rung: Names, file: auth.go, decl: 0, sub: 0, line: 0 } |  |  | 0.449 |
| walker |  | 4505 | 18 | Code::CodeKey { rung: Decl, file: auth.go, decl: 4, sub: 0, line: 25 } |  |  | 0.449 |
| walker |  | 4517 | 12 | Code::CodeKey { rung: Body, file: auth.go, decl: 8, sub: 0, line: 72 } |  |  | 0.449 |
| walker |  | 4534 | 17 | Code::CodeKey { rung: Doc, file: auth.go, decl: 3, sub: 0, line: 23 } |  |  | 0.449 |
| walker |  | 4553 | 19 | Code::CodeKey { rung: Doc, file: auth.go, decl: 1, sub: 0, line: 17 } |  |  | 0.449 |
| walker |  | 4574 | 21 | Code::CodeKey { rung: Doc, file: auth.go, decl: 2, sub: 0, line: 20 } |  |  | 0.449 |
| walker |  | 4591 | 17 | Code::CodeKey { rung: Names, file: version.go, decl: 0, sub: 0, line: 0 } |  |  | 0.472 |
| walker |  | 4605 | 14 | Code::CodeKey { rung: Doc, file: version.go, decl: 1, sub: 0, line: 8 } |  |  | 0.512 |
| walker |  | 4627 | 22 | Code::CodeKey { rung: Doc, file: utils.go, decl: 5, sub: 0, line: 47 } |  |  | 0.512 |
| ns | 4658 |  | 220 | path.go: cleanPath semantics and the file's whole function set | 2.13 | 2.11 | 0.504 |
| ns | 4888 |  | 230 | Trusted-platform constants and the SetTrustedProxies contract | 2.14 | 2.7 | 0.494 |
| ns | 4977 |  | 89 | Makefile: complete target roster | 3.1 |  | 0.488 |
| walker |  | 5011 | 384 | Code::CodeKey { rung: Names, file: errors.go, decl: 0, sub: 0, line: 0 } |  |  | 0.488 |
| ns | 5035 |  | 58 | Makefile variables: which packages are tested and vetted | 3.2 | 3.1 | 0.487 |
| walker |  | 5038 | 27 | Code::CodeKey { rung: Decl, file: errors.go, decl: 3, sub: 0, line: 32 } |  |  | 0.487 |
| walker |  | 5115 | 77 | Code::CodeKey { rung: Decl, file: errors.go, decl: 2, sub: 0, line: 18 } |  |  | 0.487 |
| walker |  | 5123 | 8 | Code::CodeKey { rung: Body, file: errors.go, decl: 12, sub: 0, line: 92 } |  |  | 0.487 |
| walker |  | 5132 | 9 | Code::CodeKey { rung: Body, file: errors.go, decl: 10, sub: 0, line: 82 } |  |  | 0.487 |
| walker |  | 5142 | 10 | Code::CodeKey { rung: Doc, file: errors.go, decl: 3, sub: 0, line: 32 } |  |  | 0.487 |
| walker |  | 5153 | 11 | Code::CodeKey { rung: Doc, file: errors.go, decl: 6, sub: 0, line: 43 } |  |  | 0.487 |
| walker |  | 5164 | 11 | Code::CodeKey { rung: Doc, file: errors.go, decl: 10, sub: 0, line: 82 } |  |  | 0.487 |
| walker |  | 5175 | 11 | Code::CodeKey { rung: Doc, file: errors.go, decl: 11, sub: 0, line: 87 } |  |  | 0.487 |
| walker |  | 5187 | 12 | Code::CodeKey { rung: Doc, file: errors.go, decl: 8, sub: 0, line: 55 } |  |  | 0.487 |
| walker |  | 5201 | 14 | Code::CodeKey { rung: Doc, file: errors.go, decl: 7, sub: 0, line: 49 } |  |  | 0.487 |
| walker |  | 5215 | 14 | Code::CodeKey { rung: Doc, file: errors.go, decl: 9, sub: 0, line: 77 } |  |  | 0.487 |
| walker |  | 5229 | 14 | Code::CodeKey { rung: Doc, file: errors.go, decl: 17, sub: 0, line: 157 } |  |  | 0.487 |
| walker |  | 5251 | 22 | Code::CodeKey { rung: Doc, file: errors.go, decl: 1, sub: 0, line: 16 } |  |  | 0.487 |
| ns | 5457 |  | 422 | Context struct: all fields with their doc comments | 4.1 |  | 0.467 |
| walker |  | 5478 | 227 | Code::CodeKey { rung: Names, file: debug.go, decl: 0, sub: 0, line: 0 } |  |  | 0.467 |
| walker |  | 5490 | 12 | Code::CodeKey { rung: Doc, file: debug.go, decl: 5, sub: 0, line: 30 } |  |  | 0.467 |
| walker |  | 5505 | 15 | Code::CodeKey { rung: Doc, file: debug.go, decl: 4, sub: 0, line: 27 } |  |  | 0.467 |
| walker |  | 5522 | 17 | Code::CodeKey { rung: Body, file: debug.go, decl: 3, sub: 0, line: 22 } |  |  | 0.467 |
| walker |  | 5593 | 71 | Code::CodeKey { rung: Names, file: path.go, decl: 0, sub: 0, line: 0 } |  |  | 0.467 |
| walker |  | 5616 | 23 | Code::CodeKey { rung: Doc, file: utils.go, decl: 18, sub: 0, line: 174 } |  |  | 0.467 |
| walker |  | 5639 | 23 | Code::CodeKey { rung: Doc, file: utils.go, decl: 19, sub: 0, line: 182 } |  |  | 0.467 |
| ns | 5672 |  | 215 | Context flow control and error attachment: complete roster | 4.2 | 4.1 | 0.460 |
| walker |  | 5999 | 360 | Code::CodeKey { rung: Names, file: response_writer.go, decl: 0, sub: 0, line: 0 } |  |  | 0.460 |
| walker |  | 6008 | 9 | Code::CodeKey { rung: Decl, file: response_writer.go, decl: 1, sub: 0, line: 15 } |  |  | 0.460 |
| walker |  | 6034 | 26 | Code::CodeKey { rung: Decl, file: response_writer.go, decl: 4, sub: 0, line: 49 } |  |  | 0.460 |
| walker |  | 6042 | 8 | Code::CodeKey { rung: Body, file: response_writer.go, decl: 12, sub: 0, line: 98 } |  |  | 0.460 |
| ns | 6043 |  | 371 | Context response rendering: complete roster | 4.3 | 4.1 | 0.449 |
| walker |  | 6050 | 8 | Code::CodeKey { rung: Body, file: response_writer.go, decl: 13, sub: 0, line: 102 } |  |  | 0.449 |
| walker |  | 6292 | 242 | Code::CodeKey { rung: Decl, file: response_writer.go, decl: 3, sub: 0, line: 23 } |  |  | 0.449 |
| walker |  | 6298 | 6 | Code::CodeKey { rung: Doc, file: response_writer.go, decl: 3, sub: 0, line: 23 } |  |  | 0.449 |
| walker |  | 6311 | 13 | Code::CodeKey { rung: Doc, file: response_writer.go, decl: 17, sub: 0, line: 129 } |  |  | 0.449 |
| walker |  | 6325 | 14 | Code::CodeKey { rung: Doc, file: response_writer.go, decl: 16, sub: 0, line: 124 } |  |  | 0.449 |
| walker |  | 6340 | 15 | Code::CodeKey { rung: Doc, file: response_writer.go, decl: 15, sub: 0, line: 111 } |  |  | 0.449 |
| walker |  | 6364 | 24 | Code::CodeKey { rung: Doc, file: recovery.go, decl: 3, sub: 0, line: 35 } |  |  | 0.449 |
| walker |  | 6463 | 99 | Code::CodeKey { rung: Names, file: fs.go, decl: 0, sub: 0, line: 0 } |  |  | 0.449 |
| ns | 6468 |  | 425 | Context request binding: complete roster of all 26 entry points | 4.4 | 4.1 | 0.438 |
| walker |  | 6475 | 12 | Code::CodeKey { rung: Decl, file: fs.go, decl: 3, sub: 0, line: 28 } |  |  | 0.438 |
| walker |  | 6490 | 15 | Code::CodeKey { rung: Decl, file: fs.go, decl: 1, sub: 0, line: 13 } |  |  | 0.438 |
| walker |  | 6506 | 16 | Code::CodeKey { rung: Doc, file: fs.go, decl: 4, sub: 0, line: 33 } |  |  | 0.438 |
| walker |  | 6525 | 19 | Code::CodeKey { rung: Doc, file: fs.go, decl: 2, sub: 0, line: 18 } |  |  | 0.438 |
| walker |  | 6545 | 20 | Code::CodeKey { rung: Doc, file: fs.go, decl: 1, sub: 0, line: 13 } |  |  | 0.438 |
| walker |  | 6567 | 22 | Code::CodeKey { rung: Doc, file: fs.go, decl: 3, sub: 0, line: 28 } |  |  | 0.438 |
| walker |  | 6593 | 26 | Code::CodeKey { rung: Doc, file: utils.go, decl: 17, sub: 0, line: 164 } |  |  | 0.438 |
| walker |  | 6620 | 27 | Code::CodeKey { rung: Doc, file: errors.go, decl: 12, sub: 0, line: 92 } |  |  | 0.438 |
| walker |  | 6647 | 27 | Code::CodeKey { rung: Doc, file: recovery.go, decl: 4, sub: 0, line: 40 } |  |  | 0.438 |
| ns | 6809 |  | 341 | Context request input: params, query, form, uploads — complete roster | 4.5 | 4.1 | 0.429 |
| walker |  | 6846 | 199 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 0, line: 0 } |  |  | 0.429 |
| walker |  | 6857 | 11 | Code::CodeKey { rung: Decl, file: context.go, decl: 1, sub: 0, line: 31 } |  |  | 0.429 |
| walker |  | 6872 | 15 | Code::CodeKey { rung: Doc, file: context.go, decl: 1, sub: 0, line: 31 } |  |  | 0.429 |
| ns | 7030 |  | 221 | Context headers, cookies and client IP: complete roster | 4.6 | 4.1 | 0.423 |
| walker |  | 7076 | 204 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 1, line: 0 } |  |  | 0.426 |
| walker |  | 7085 | 9 | Code::CodeKey { rung: Body, file: context.go, decl: 12, sub: 0, line: 167 } |  |  | 0.426 |
| walker |  | 7096 | 11 | Code::CodeKey { rung: Doc, file: context.go, decl: 12, sub: 0, line: 167 } |  |  | 0.426 |
| ns | 7100 |  | 70 | Context key/value store, with the typed-accessor family marked elided | 4.7 | 4.1 | 0.424 |
| walker |  | 7109 | 13 | Code::CodeKey { rung: Doc, file: context.go, decl: 2, sub: 0, line: 47 } |  |  | 0.424 |
| walker |  | 7123 | 14 | Code::CodeKey { rung: Doc, file: context.go, decl: 6, sub: 0, line: 57 } |  |  | 0.424 |
| walker |  | 7138 | 15 | Code::CodeKey { rung: Doc, file: context.go, decl: 3, sub: 0, line: 50 } |  |  | 0.424 |
| ns | 7259 |  | 159 | Content negotiation and Context's context.Context implementation | 4.8 | 4.1 | 0.420 |
| walker |  | 7525 | 387 | Code::CodeKey { rung: Decl, file: context.go, decl: 7, sub: 0, line: 61 } |  |  | 0.465 |
| ns | 7557 |  | 298 | Bind vs ShouldBind vs ShouldBindBodyWith: the semantic difference | 4.9 | 4.4 | 0.460 |
| walker |  | 7704 | 179 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 2, line: 0 } |  |  | 0.475 |
| ns | 7714 |  | 157 | Middleware chain mechanics: Next() and the abortIndex sentinel | 4.10 | 4.2 | 0.469 |
| walker |  | 7721 | 17 | Code::CodeKey { rung: Doc, file: context.go, decl: 15, sub: 0, line: 199 } |  |  | 0.470 |
| walker |  | 7908 | 187 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 3, line: 0 } |  |  | 0.474 |
| walker |  | 7926 | 18 | Code::CodeKey { rung: Doc, file: context.go, decl: 26, sub: 0, line: 311 } |  |  | 0.474 |
| walker |  | 7944 | 18 | Code::CodeKey { rung: Doc, file: context.go, decl: 27, sub: 0, line: 316 } |  |  | 0.474 |
| ns | 7959 |  | 245 | binding: the complete MIME constant table | 5.1 |  | 0.468 |
| walker |  | 7962 | 18 | Code::CodeKey { rung: Doc, file: context.go, decl: 28, sub: 0, line: 321 } |  |  | 0.468 |
| walker |  | 7983 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 29, sub: 0, line: 326 } |  |  | 0.468 |
| walker |  | 8004 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 30, sub: 0, line: 331 } |  |  | 0.468 |
| walker |  | 8025 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 31, sub: 0, line: 336 } |  |  | 0.468 |
| walker |  | 8048 | 23 | Code::CodeKey { rung: Doc, file: context.go, decl: 24, sub: 0, line: 296 } |  |  | 0.468 |
| ns | 8195 |  | 236 | binding: the Binding / BindingBody / BindingUri interfaces | 5.2 | 5.1 | 0.460 |
| walker |  | 8234 | 186 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 4, line: 0 } |  |  | 0.460 |
| walker |  | 8251 | 17 | Code::CodeKey { rung: Doc, file: context.go, decl: 40, sub: 0, line: 381 } |  |  | 0.460 |
| walker |  | 8270 | 19 | Code::CodeKey { rung: Doc, file: context.go, decl: 33, sub: 0, line: 346 } |  |  | 0.460 |
| walker |  | 8290 | 20 | Code::CodeKey { rung: Doc, file: context.go, decl: 38, sub: 0, line: 371 } |  |  | 0.460 |
| walker |  | 8310 | 20 | Code::CodeKey { rung: Doc, file: context.go, decl: 39, sub: 0, line: 376 } |  |  | 0.460 |
| walker |  | 8331 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 32, sub: 0, line: 341 } |  |  | 0.460 |
| walker |  | 8353 | 22 | Code::CodeKey { rung: Doc, file: context.go, decl: 34, sub: 0, line: 351 } |  |  | 0.460 |
| walker |  | 8375 | 22 | Code::CodeKey { rung: Doc, file: context.go, decl: 35, sub: 0, line: 356 } |  |  | 0.460 |
| walker |  | 8397 | 22 | Code::CodeKey { rung: Doc, file: context.go, decl: 36, sub: 0, line: 361 } |  |  | 0.460 |
| walker |  | 8419 | 22 | Code::CodeKey { rung: Doc, file: context.go, decl: 37, sub: 0, line: 366 } |  |  | 0.460 |
| ns | 8430 |  | 235 | binding: the registry of concrete binder singletons | 5.3 | 5.2 | 0.453 |
| walker |  | 8592 | 173 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 5, line: 0 } |  |  | 0.453 |
| walker |  | 8610 | 18 | Code::CodeKey { rung: Doc, file: context.go, decl: 41, sub: 0, line: 386 } |  |  | 0.453 |
| walker |  | 8628 | 18 | Code::CodeKey { rung: Doc, file: context.go, decl: 42, sub: 0, line: 391 } |  |  | 0.453 |
| walker |  | 8649 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 43, sub: 0, line: 396 } |  |  | 0.453 |
| walker |  | 8671 | 22 | Code::CodeKey { rung: Doc, file: context.go, decl: 48, sub: 0, line: 421 } |  |  | 0.453 |
| walker |  | 8695 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 44, sub: 0, line: 401 } |  |  | 0.453 |
| ns | 8705 |  | 275 | binding.Default(): how a request picks its binder | 5.4 | 5.3 | 0.443 |
| walker |  | 8719 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 45, sub: 0, line: 406 } |  |  | 0.443 |
| walker |  | 8743 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 46, sub: 0, line: 411 } |  |  | 0.443 |
| walker |  | 8767 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 47, sub: 0, line: 416 } |  |  | 0.443 |
| walker |  | 8947 | 180 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 6, line: 0 } |  |  | 0.443 |
| walker |  | 8968 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 55, sub: 0, line: 456 } |  |  | 0.443 |
| walker |  | 8989 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 56, sub: 0, line: 461 } |  |  | 0.443 |
| walker |  | 9013 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 49, sub: 0, line: 426 } |  |  | 0.443 |
| walker |  | 9037 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 50, sub: 0, line: 431 } |  |  | 0.443 |
| walker |  | 9061 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 51, sub: 0, line: 436 } |  |  | 0.443 |
| walker |  | 9085 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 52, sub: 0, line: 441 } |  |  | 0.443 |
| walker |  | 9109 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 53, sub: 0, line: 446 } |  |  | 0.443 |
| walker |  | 9133 | 24 | Code::CodeKey { rung: Doc, file: context.go, decl: 54, sub: 0, line: 451 } |  |  | 0.443 |
| ns | 9150 |  | 445 | render: the Render interface and every implementation | 5.5 |  | 0.432 |
| walker |  | 9302 | 169 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 7, line: 0 } |  |  | 0.435 |
| ns | 9312 |  | 162 | Every build-tag line in the repository | 5.6 | 5.4 | 0.433 |
| walker |  | 9323 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 57, sub: 0, line: 466 } |  |  | 0.433 |
| walker |  | 9345 | 22 | Code::CodeKey { rung: Doc, file: context.go, decl: 58, sub: 0, line: 471 } |  |  | 0.433 |
| ns | 9452 |  | 140 | binding: StructValidator and the default validator hook | 5.7 | 5.2 | 0.429 |
| walker |  | 9508 | 163 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 8, line: 0 } |  |  | 0.434 |
| walker |  | 9524 | 16 | Code::CodeKey { rung: Doc, file: context.go, decl: 69, sub: 0, line: 587 } |  |  | 0.434 |
| ns | 9624 |  | 172 | Run modes: GIN_MODE, the three mode constants, and the default writers | 6.1 |  | 0.442 |
| walker |  | 9693 | 169 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 9, line: 0 } |  |  | 0.452 |
| ns | 9695 |  | 71 | mode.go: complete function roster | 6.2 | 6.1 | 0.456 |
| walker |  | 9710 | 17 | Code::CodeKey { rung: Doc, file: context.go, decl: 77, sub: 0, line: 660 } |  |  | 0.456 |
| walker |  | 9874 | 164 | Code::CodeKey { rung: Names, file: context.go, decl: 0, sub: 10, line: 0 } |  |  | 0.463 |
| walker |  | 9890 | 16 | Code::CodeKey { rung: Doc, file: context.go, decl: 82, sub: 0, line: 719 } |  |  | 0.463 |
| walker |  | 9907 | 17 | Code::CodeKey { rung: Doc, file: context.go, decl: 80, sub: 0, line: 698 } |  |  | 0.463 |
| walker |  | 9924 | 17 | Code::CodeKey { rung: Doc, file: context.go, decl: 81, sub: 0, line: 713 } |  |  | 0.463 |
| walker |  | 9944 | 20 | Code::CodeKey { rung: Doc, file: context.go, decl: 84, sub: 0, line: 763 } |  |  | 0.463 |
| walker |  | 9965 | 21 | Code::CodeKey { rung: Doc, file: context.go, decl: 85, sub: 0, line: 768 } |  |  | 0.463 |
| ns | 9972 |  | 277 | Built-in middleware and helper constructors: complete package-level roster | 6.3 |  | 0.473 |
| walker |  | 9991 | 26 | Code::CodeKey { rung: Doc, file: context.go, decl: 59, sub: 0, line: 476 } |  |  | 0.473 |
