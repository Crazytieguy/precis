Score(3000)=0.624 I=0.848 C=0.459 ns_rows≤3K=19/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.603/0.631/0.622/0.624/0.532/0.454/0.529

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 43 | 43 | Fs::DirListing { dir: . } |  |  | 1.000 |
| ns | 43 |  | 43 | Repository root listing | 1.1 |  | 1.000 |
| walker |  | 58 | 15 | Code::CodeKey { rung: Names, file: index.js, decl: 0, sub: 0, line: 0 } |  |  | 1.000 |
| walker |  | 82 | 24 | Fs::DirListing { dir: lib } |  |  | 1.000 |
| ns | 135 |  | 92 | package.json identity | 1.2 |  | 0.827 |
| walker |  | 145 | 63 | Json::Identity { file: package.json } |  |  | 0.908 |
| walker |  | 153 | 8 | Fs::DirListing { dir: .github } |  |  | 0.908 |
| ns | 159 |  | 24 | lib/ listing — the entire shipped library | 1.3 |  | 0.908 |
| walker |  | 171 | 18 | Fs::DirListing { dir: .github/workflows } |  |  | 0.918 |
| walker |  | 202 | 31 | Json::Runtime { file: package.json } |  |  | 0.919 |
| walker |  | 252 | 50 | Json::Entry { file: package.json } |  |  | 0.919 |
| ns | 296 |  | 137 | Module export surface (index.js + lib/express.js exports) | 1.4 |  | 0.728 |
| walker |  | 327 | 75 | Code::CodeKey { rung: Names, file: lib/view.js, decl: 0, sub: 0, line: 0 } |  |  | 0.729 |
| walker |  | 428 | 101 | Fs::DirListing { dir: examples } |  |  | 0.732 |
| ns | 443 |  | 147 | Readme tagline + canonical quick-start snippet | 1.5 |  | 0.604 |
| walker |  | 547 | 119 | Code::CodeKey { rung: Names, file: lib/request.js, decl: 0, sub: 0, line: 0 } |  |  | 0.606 |
| walker |  | 560 | 13 | Code::CodeKey { rung: Body, file: lib/request.js, decl: 5, sub: 0, line: 185 } |  |  | 0.606 |
| ns | 652 |  | 209 | npm scripts + engines | 1.6 |  | 0.542 |
| ns | 678 |  | 26 | .github listing — all four workflows | 1.7 |  | 0.576 |
| walker |  | 693 | 133 | Code::CodeKey { rung: Names, file: lib/express.js, decl: 0, sub: 0, line: 0 } |  |  | 0.717 |
| ns | 789 |  | 111 | Prototype bases and module exports of each lib file | 2.1 |  | 0.685 |
| ns | 962 |  | 173 | app.* roster — every application method name | 2.2 |  | 0.603 |
| walker |  | 1113 | 420 | Markdown::ReadmeHeadline { file: Readme.md } |  |  | 0.723 |
| ns | 1201 |  | 239 | res.* roster — every response method name | 2.3 |  | 0.631 |
| walker |  | 1241 | 128 | Markdown::HeadingsOutline { file: Readme.md } |  |  | 0.631 |
| walker |  | 1328 | 87 | Markdown::Section { file: Readme.md, section_index: 3, keeps_default_concavity: true } |  |  | 0.631 |
| walker |  | 1352 | 24 | Code::CodeKey { rung: Body, file: lib/request.js, decl: 4, sub: 0, line: 171 } |  |  | 0.631 |
| walker |  | 1378 | 26 | Code::CodeKey { rung: Doc, file: lib/request.js, decl: 1, sub: 0, line: 30 } |  |  | 0.631 |
| walker |  | 1403 | 25 | Code::CodeKey { rung: Body, file: lib/request.js, decl: 2, sub: 0, line: 127 } |  |  | 0.631 |
| ns | 1465 |  | 264 | req.* roster — methods and defineGetter properties | 2.4 |  | 0.579 |
| walker |  | 1656 | 253 | Code::CodeKey { rung: Names, file: lib/application.js, decl: 0, sub: 0, line: 0 } |  |  | 0.653 |
| ns | 1663 |  | 198 | lib/view.js and lib/utils.js symbol rosters | 2.5 |  | 0.620 |
| walker |  | 1667 | 11 | Code::CodeKey { rung: Body, file: lib/application.js, decl: 6, sub: 0, line: 256 } |  |  | 0.620 |
| walker |  | 1678 | 11 | Code::CodeKey { rung: Body, file: lib/application.js, decl: 11, sub: 0, line: 420 } |  |  | 0.620 |
| walker |  | 1689 | 11 | Code::CodeKey { rung: Body, file: lib/application.js, decl: 12, sub: 0, line: 439 } |  |  | 0.620 |
| walker |  | 1701 | 12 | Code::CodeKey { rung: Body, file: lib/application.js, decl: 13, sub: 0, line: 451 } |  |  | 0.620 |
| walker |  | 1713 | 12 | Code::CodeKey { rung: Body, file: lib/application.js, decl: 14, sub: 0, line: 463 } |  |  | 0.620 |
| walker |  | 1880 | 167 | Code::CodeKey { rung: Names, file: lib/utils.js, decl: 0, sub: 0, line: 0 } |  |  | 0.670 |
| walker |  | 1892 | 12 | Code::CodeKey { rung: Body, file: lib/utils.js, decl: 5, sub: 0, line: 75 } |  |  | 0.670 |
| ns | 1896 |  | 233 | createApplication() body | 2.6 |  | 0.622 |
| ns | 2153 |  | 257 | HTTP-verb delegation and app.all() | 2.7 |  | 0.575 |
| walker |  | 2201 | 309 | Code::CodeKey { rung: Names, file: lib/response.js, decl: 0, sub: 0, line: 0 } |  |  | 0.635 |
| walker |  | 2212 | 11 | Code::CodeKey { rung: Body, file: lib/response.js, decl: 13, sub: 0, line: 696 } |  |  | 0.635 |
| walker |  | 2227 | 15 | Code::CodeKey { rung: Body, file: lib/response.js, decl: 16, sub: 0, line: 794 } |  |  | 0.635 |
| ns | 2244 |  | 91 | app.* full signature lines | 2.8 | 2.2 | 0.656 |
| walker |  | 2250 | 23 | Code::CodeKey { rung: Body, file: lib/response.js, decl: 18, sub: 0, line: 875 } |  |  | 0.656 |
| walker |  | 2276 | 26 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 1, sub: 0, line: 42 } |  |  | 0.656 |
| ns | 2369 |  | 125 | res.* full signature lines | 2.9 | 2.3 | 0.663 |
| walker |  | 2452 | 176 | Json::Scripts { file: package.json } |  |  | 0.703 |
| ns | 2478 |  | 109 | Default settings established at boot | 3.1 |  | 0.687 |
| ns | 2651 |  | 173 | Remaining boot configuration: locals, mountpath, view defaults | 3.2 |  | 0.659 |
| walker |  | 2828 | 376 | Fs::DirListing { dir: test } |  |  | 0.666 |
| walker |  | 2840 | 12 | Fs::DirListing { dir: test/support } |  |  | 0.666 |
| walker |  | 2897 | 57 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 13, sub: 0, line: 696 } |  |  | 0.666 |
| walker |  | 2915 | 18 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 1, sub: 0, line: 40 } |  |  | 0.666 |
| walker |  | 2954 | 39 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 1, sub: 0, line: 29 } |  |  | 0.666 |
| ns | 2955 |  | 304 | app.set() — storage plus the three derived-setting side effects | 3.3 |  | 0.624 |
| ns | 3140 |  | 185 | compileETag — accepted values of the `etag` setting | 3.4 |  | 0.599 |
| ns | 3357 |  | 217 | compileQueryParser — accepted values of `query parser` | 3.5 |  | 0.572 |
| walker |  | 3414 | 460 | Json::Dependencies { file: package.json } |  |  | 0.574 |
| walker |  | 3440 | 26 | Code::CodeKey { rung: Body, file: lib/request.js, decl: 3, sub: 0, line: 140 } |  |  | 0.574 |
| walker |  | 3526 | 86 | Fs::DirListing { dir: test/acceptance } |  |  | 0.576 |
| ns | 3556 |  | 199 | compileTrust — accepted values of `trust proxy` | 3.6 |  | 0.557 |
| walker |  | 3568 | 42 | Code::CodeKey { rung: Doc, file: lib/express.js, decl: 1, sub: 0, line: 36 } |  |  | 0.557 |
| walker |  | 3613 | 45 | Code::CodeKey { rung: Body, file: lib/utils.js, decl: 4, sub: 0, line: 61 } |  |  | 0.557 |
| walker |  | 3661 | 48 | Code::CodeKey { rung: Doc, file: lib/view.js, decl: 2, sub: 0, line: 104 } |  |  | 0.557 |
| ns | 3688 |  | 132 | req.ip / req.ips — the consumers of `trust proxy fn` | 3.7 | 2.4 | 0.545 |
| walker |  | 3720 | 59 | Code::CodeKey { rung: Body, file: lib/response.js, decl: 7, sub: 0, line: 321 } |  |  | 0.545 |
| walker |  | 3778 | 58 | Code::CodeKey { rung: Doc, file: lib/view.js, decl: 3, sub: 0, line: 133 } |  |  | 0.545 |
| walker |  | 3838 | 60 | Code::CodeKey { rung: Doc, file: lib/view.js, decl: 4, sub: 0, line: 169 } |  |  | 0.545 |
| walker |  | 3901 | 63 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 7, sub: 0, line: 162 } |  |  | 0.545 |
| walker |  | 3961 | 60 | Code::CodeKey { rung: Body, file: lib/response.js, decl: 11, sub: 0, line: 604 } |  |  | 0.545 |
| ns | 3981 |  | 293 | Settings read-sites scattered outside defaultConfiguration | 3.8 |  | 0.527 |
| walker |  | 3988 | 27 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 3, sub: 0, line: 90 } |  |  | 0.527 |
| ns | 4101 |  | 120 | app.init() — per-app state and the lazy base router | 3.9 |  | 0.516 |
| walker |  | 4326 | 338 | Json::IdentityMeta { file: package.json } |  |  | 0.532 |
| ns | 4332 |  | 231 | Sub-app mounting: the 'mount' event and setting inheritance | 3.10 |  | 0.519 |
| walker |  | 4364 | 38 | Code::CodeKey { rung: Body, file: lib/request.js, decl: 6, sub: 0, line: 214 } |  |  | 0.519 |
| walker |  | 4428 | 64 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 6, sub: 0, line: 130 } |  |  | 0.519 |
| ns | 4478 |  | 146 | JSDoc for app.set() | 3.11 |  | 0.509 |
| walker |  | 4559 | 131 | Markdown::Section { file: Readme.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.509 |
| walker |  | 4625 | 66 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 11, sub: 0, line: 604 } |  |  | 0.509 |
| ns | 4735 |  | 257 | app.handle() — the per-request dispatch entry point | 4.1 | 2.8 | 0.492 |
| walker |  | 4749 | 124 | Markdown::Section { file: Readme.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.492 |
| walker |  | 4780 | 31 | Code::CodeKey { rung: Body, file: lib/application.js, decl: 10, sub: 0, line: 399 } |  |  | 0.492 |
| walker |  | 4909 | 129 | Code::CodeKey { rung: Doc, file: lib/view.js, decl: 1, sub: 0, line: 52 } |  |  | 0.492 |
| walker |  | 4974 | 65 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 4, sub: 0, line: 61 } |  |  | 0.492 |
| ns | 5006 |  | 271 | app.use() — argument/path disambiguation | 4.2 | 2.8 | 0.476 |
| walker |  | 5120 | 146 | Markdown::Section { file: Readme.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.476 |
| walker |  | 5192 | 72 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 14, sub: 0, line: 709 } |  |  | 0.476 |
| ns | 5275 |  | 269 | app.use() — the sub-app mounting branch | 4.3 | 4.2 | 0.463 |
| walker |  | 5434 | 242 | Markdown::Section { file: Readme.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.463 |
| walker |  | 5497 | 63 | Code::CodeKey { rung: Doc, file: lib/request.js, decl: 3, sub: 0, line: 140 } |  |  | 0.463 |
| walker |  | 5562 | 65 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 5, sub: 0, line: 75 } |  |  | 0.463 |
| ns | 5590 |  | 315 | res.send() body-type dispatch | 4.4 | 2.9 | 0.447 |
| walker |  | 5641 | 79 | Code::CodeKey { rung: Body, file: lib/response.js, decl: 14, sub: 0, line: 709 } |  |  | 0.447 |
| ns | 5771 |  | 181 | View.prototype.lookup() — view file resolution | 4.5 | 2.5 | 0.439 |
| walker |  | 5858 | 217 | Code::CodeKey { rung: Body, file: lib/express.js, decl: 1, sub: 0, line: 36 } |  |  | 0.473 |
| walker |  | 6103 | 245 | Markdown::Section { file: Readme.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.473 |
| ns | 6222 |  | 451 | res.send() response finalization | 4.6 | 4.4 | 0.454 |
| walker |  | 6316 | 213 | Markdown::Section { file: Readme.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.454 |
| walker |  | 6485 | 169 | Code::CodeKey { rung: Body, file: lib/view.js, decl: 4, sub: 0, line: 169 } |  |  | 0.454 |
| walker |  | 6554 | 69 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 8, sub: 0, line: 194 } |  |  | 0.454 |
| ns | 6598 |  | 376 | test/ listing — every spec file | 5.1 |  | 0.514 |
| walker |  | 6611 | 57 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 13, sub: 0, line: 451 } |  |  | 0.514 |
| ns | 6684 |  | 86 | test/acceptance/ listing | 5.2 |  | 0.526 |
| walker |  | 6697 | 86 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 18, sub: 0, line: 875 } |  |  | 0.526 |
| walker |  | 6777 | 80 | Code::CodeKey { rung: Doc, file: lib/request.js, decl: 5, sub: 0, line: 185 } |  |  | 0.526 |
| ns | 6785 |  | 101 | examples/ listing | 5.3 |  | 0.542 |
| walker |  | 6848 | 71 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 10, sub: 0, line: 249 } |  |  | 0.542 |
| walker |  | 6905 | 57 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 14, sub: 0, line: 463 } |  |  | 0.542 |
| walker |  | 6997 | 92 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 5, sub: 0, line: 232 } |  |  | 0.542 |
| ns | 7041 |  | 256 | examples/README.md — annotated example index (first half) | 5.4 |  | 0.536 |
| walker |  | 7173 | 176 | Code::CodeKey { rung: Body, file: lib/view.js, decl: 2, sub: 0, line: 104 } |  |  | 0.555 |
| walker |  | 7245 | 72 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 2, sub: 0, line: 40 } |  |  | 0.555 |
| ns | 7299 |  | 258 | examples/README.md — annotated example index (second half) | 5.5 | 5.4 | 0.550 |
| ns | 7390 |  | 91 | test/support/ and test/fixtures/ listings | 5.6 |  | 0.542 |
| ns | 7542 |  | 152 | Test harness: env preamble, assertion helpers, template engine | 5.7 |  | 0.538 |
| walker |  | 7573 | 328 | Markdown::Section { file: Readme.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.538 |
| ns | 7673 |  | 131 | A complete example app: examples/hello-world/index.js | 5.8 |  | 0.532 |
| walker |  | 7756 | 183 | Markdown::Section { file: Readme.md, section_index: 10, keeps_default_concavity: true } |  |  | 0.532 |
| ns | 7834 |  | 161 | Test-writing idiom: head of test/app.js | 5.9 |  | 0.527 |
| walker |  | 7855 | 99 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 6, sub: 0, line: 260 } |  |  | 0.527 |
| ns | 8120 |  | 286 | Runtime dependencies (all 28) | 6.1 |  | 0.541 |
| walker |  | 8195 | 340 | Markdown::Section { file: Readme.md, section_index: 13, keeps_default_concavity: false } |  |  | 0.541 |
| ns | 8346 |  | 226 | ci.yml — jobs and the OS/Node matrix | 6.2 |  | 0.535 |
| ns | 8533 |  | 187 | .eslintrc.yml — the complete lint rule set | 6.3 |  | 0.530 |
| walker |  | 8549 | 354 | Markdown::Section { file: Readme.md, section_index: 11, keeps_default_concavity: false } |  |  | 0.530 |
| walker |  | 8620 | 71 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 2, sub: 0, line: 59 } |  |  | 0.530 |
| walker |  | 8692 | 72 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 3, sub: 0, line: 51 } |  |  | 0.530 |
| walker |  | 8804 | 112 | Code::CodeKey { rung: Body, file: lib/request.js, decl: 7, sub: 0, line: 269 } |  |  | 0.530 |
| ns | 8807 |  | 274 | Dev dependencies (all 16) | 6.4 |  | 0.524 |
| ns | 8888 |  | 81 | package.json remainder: author, published files | 6.5 |  | 0.529 |
| walker |  | 8918 | 114 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 4, sub: 0, line: 125 } |  |  | 0.529 |
| walker |  | 8991 | 73 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 9, sub: 0, line: 225 } |  |  | 0.529 |
| ns | 9121 |  | 233 | res.sendFile() option bag (JSDoc) | 7.1 |  | 0.524 |
| walker |  | 9235 | 244 | Code::CodeKey { rung: Body, file: lib/view.js, decl: 3, sub: 0, line: 133 } |  |  | 0.524 |
| walker |  | 9316 | 81 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 4, sub: 0, line: 152 } |  |  | 0.524 |
| ns | 9391 |  | 270 | res.cookie() option bag (JSDoc) | 7.2 |  | 0.517 |
| walker |  | 9435 | 119 | Code::CodeKey { rung: Body, file: lib/response.js, decl: 12, sub: 0, line: 629 } |  |  | 0.517 |
| walker |  | 9508 | 73 | Code::CodeKey { rung: Body, file: lib/utils.js, decl: 10, sub: 0, line: 249 } |  |  | 0.517 |
| walker |  | 9587 | 79 | Fs::DirListing { dir: test/fixtures } |  |  | 0.534 |
| walker |  | 9591 | 4 | Fs::DirListing { dir: test/fixtures/pets } |  |  | 0.534 |
| walker |  | 9596 | 5 | Fs::DirListing { dir: test/fixtures/local_layout } |  |  | 0.534 |
| walker |  | 9603 | 7 | Fs::DirListing { dir: test/fixtures/blog } |  |  | 0.534 |
| walker |  | 9610 | 7 | Fs::DirListing { dir: test/fixtures/snow ☃ } |  |  | 0.534 |
| walker |  | 9615 | 5 | Fs::DirListing { dir: test/fixtures/blog/post } |  |  | 0.534 |
| walker |  | 9624 | 9 | Fs::DirListing { dir: test/fixtures/users } |  |  | 0.534 |
| walker |  | 9634 | 10 | Fs::DirListing { dir: test/fixtures/default_layout } |  |  | 0.534 |
| ns | 9673 |  | 282 | res.status() and res.render() contracts (JSDoc) | 7.3 |  | 0.528 |
| walker |  | 9722 | 88 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 6, sub: 0, line: 256 } |  |  | 0.528 |
| walker |  | 9854 | 132 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 7, sub: 0, line: 321 } |  |  | 0.528 |
| walker |  | 9941 | 87 | Code::CodeKey { rung: Body, file: lib/application.js, decl: 15, sub: 0, line: 494 } |  |  | 0.530 |
| ns | 9971 |  | 298 | History.md — unreleased section and the 5.2.1 heading | 8.1 |  | 0.527 |
