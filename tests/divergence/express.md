Score(3000)=0.512 I=0.805 C=0.326 ns_rows≤3K=19/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.604/0.696/0.557/0.512/0.490/0.408/0.509

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 43 | 43 | Fs::DirListing { dir: . } |  |  | 1.000 |
| ns | 43 |  | 43 | Repository root listing | 1.1 |  | 1.000 |
| walker |  | 67 | 24 | Fs::DirListing { dir: lib } |  |  | 1.000 |
| walker |  | 130 | 63 | Json::Identity { file: package.json } |  |  | 1.000 |
| ns | 135 |  | 92 | package.json identity | 1.2 |  | 0.908 |
| walker |  | 138 | 8 | Fs::DirListing { dir: .github } |  |  | 0.908 |
| walker |  | 156 | 18 | Fs::DirListing { dir: .github/workflows } |  |  | 0.908 |
| ns | 159 |  | 24 | lib/ listing — the entire shipped library | 1.3 |  | 0.918 |
| walker |  | 187 | 31 | Json::Runtime { file: package.json } |  |  | 0.918 |
| walker |  | 237 | 50 | Json::Entry { file: package.json } |  |  | 0.919 |
| ns | 296 |  | 137 | Module export surface (index.js + lib/express.js exports) | 1.4 |  | 0.725 |
| walker |  | 338 | 101 | Fs::DirListing { dir: examples } |  |  | 0.729 |
| ns | 443 |  | 147 | Readme tagline + canonical quick-start snippet | 1.5 |  | 0.601 |
| ns | 652 |  | 209 | npm scripts + engines | 1.6 |  | 0.538 |
| ns | 678 |  | 26 | .github listing — all four workflows | 1.7 |  | 0.572 |
| walker |  | 758 | 420 | Markdown::ReadmeHeadline { file: Readme.md } |  |  | 0.726 |
| ns | 789 |  | 111 | Prototype bases and module exports of each lib file | 2.1 |  | 0.685 |
| walker |  | 886 | 128 | Markdown::HeadingsOutline { file: Readme.md } |  |  | 0.685 |
| ns | 962 |  | 173 | app.* roster — every application method name | 2.2 |  | 0.603 |
| walker |  | 973 | 87 | Markdown::Section { file: Readme.md, section_index: 3, keeps_default_concavity: true } |  |  | 0.603 |
| walker |  | 1106 | 133 | Code::CodeKey { rung: Names, file: lib/express.js, decl: 0, sub: 0, line: 0 } |  |  | 0.693 |
| walker |  | 1148 | 42 | Code::CodeKey { rung: Doc, file: lib/express.js, decl: 1, sub: 0, line: 36 } |  |  | 0.693 |
| ns | 1201 |  | 239 | res.* roster — every response method name | 2.3 |  | 0.605 |
| walker |  | 1324 | 176 | Json::Scripts { file: package.json } |  |  | 0.678 |
| walker |  | 1339 | 15 | Code::CodeKey { rung: Names, file: index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.695 |
| ns | 1465 |  | 264 | req.* roster — methods and defineGetter properties | 2.4 |  | 0.631 |
| ns | 1663 |  | 198 | lib/view.js and lib/utils.js symbol rosters | 2.5 |  | 0.592 |
| walker |  | 1715 | 376 | Fs::DirListing { dir: test } |  |  | 0.599 |
| walker |  | 1727 | 12 | Fs::DirListing { dir: test/support } |  |  | 0.599 |
| walker |  | 1813 | 86 | Fs::DirListing { dir: test/acceptance } |  |  | 0.600 |
| ns | 1896 |  | 233 | createApplication() body | 2.6 |  | 0.557 |
| walker |  | 1944 | 131 | Markdown::Section { file: Readme.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.557 |
| walker |  | 2068 | 124 | Markdown::Section { file: Readme.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.557 |
| ns | 2153 |  | 257 | HTTP-verb delegation and app.all() | 2.7 |  | 0.516 |
| walker |  | 2187 | 119 | Code::CodeKey { rung: Names, file: lib/request.js, decl: 0, sub: 0, line: 0 } |  |  | 0.523 |
| walker |  | 2200 | 13 | Code::CodeKey { rung: Body, file: lib/request.js, decl: 5, sub: 0, line: 185 } |  |  | 0.523 |
| walker |  | 2224 | 24 | Code::CodeKey { rung: Body, file: lib/request.js, decl: 4, sub: 0, line: 171 } |  |  | 0.523 |
| ns | 2244 |  | 91 | app.* full signature lines | 2.8 | 2.2 | 0.500 |
| walker |  | 2249 | 25 | Code::CodeKey { rung: Body, file: lib/request.js, decl: 2, sub: 0, line: 127 } |  |  | 0.500 |
| walker |  | 2275 | 26 | Code::CodeKey { rung: Doc, file: lib/request.js, decl: 1, sub: 0, line: 30 } |  |  | 0.500 |
| walker |  | 2301 | 26 | Code::CodeKey { rung: Body, file: lib/request.js, decl: 3, sub: 0, line: 140 } |  |  | 0.500 |
| walker |  | 2364 | 63 | Code::CodeKey { rung: Doc, file: lib/request.js, decl: 3, sub: 0, line: 140 } |  |  | 0.500 |
| ns | 2369 |  | 125 | res.* full signature lines | 2.9 | 2.3 | 0.473 |
| ns | 2478 |  | 109 | Default settings established at boot | 3.1 |  | 0.462 |
| walker |  | 2617 | 253 | Code::CodeKey { rung: Names, file: lib/application.js, decl: 0, sub: 0, line: 0 } |  |  | 0.562 |
| walker |  | 2628 | 11 | Code::CodeKey { rung: Body, file: lib/application.js, decl: 6, sub: 0, line: 256 } |  |  | 0.562 |
| walker |  | 2639 | 11 | Code::CodeKey { rung: Body, file: lib/application.js, decl: 11, sub: 0, line: 420 } |  |  | 0.562 |
| walker |  | 2650 | 11 | Code::CodeKey { rung: Body, file: lib/application.js, decl: 12, sub: 0, line: 439 } |  |  | 0.562 |
| ns | 2651 |  | 173 | Remaining boot configuration: locals, mountpath, view defaults | 3.2 |  | 0.540 |
| walker |  | 2662 | 12 | Code::CodeKey { rung: Body, file: lib/application.js, decl: 13, sub: 0, line: 451 } |  |  | 0.540 |
| walker |  | 2674 | 12 | Code::CodeKey { rung: Body, file: lib/application.js, decl: 14, sub: 0, line: 463 } |  |  | 0.540 |
| walker |  | 2692 | 18 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 1, sub: 0, line: 40 } |  |  | 0.540 |
| walker |  | 2719 | 27 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 3, sub: 0, line: 90 } |  |  | 0.540 |
| walker |  | 2776 | 57 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 13, sub: 0, line: 451 } |  |  | 0.540 |
| walker |  | 2833 | 57 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 14, sub: 0, line: 463 } |  |  | 0.540 |
| walker |  | 2908 | 75 | Code::CodeKey { rung: Names, file: lib/view.js, decl: 0, sub: 0, line: 0 } |  |  | 0.547 |
| ns | 2955 |  | 304 | app.set() — storage plus the three derived-setting side effects | 3.3 |  | 0.512 |
| walker |  | 2956 | 48 | Code::CodeKey { rung: Doc, file: lib/view.js, decl: 2, sub: 0, line: 104 } |  |  | 0.512 |
| walker |  | 3014 | 58 | Code::CodeKey { rung: Doc, file: lib/view.js, decl: 3, sub: 0, line: 133 } |  |  | 0.512 |
| walker |  | 3074 | 60 | Code::CodeKey { rung: Doc, file: lib/view.js, decl: 4, sub: 0, line: 169 } |  |  | 0.512 |
| ns | 3140 |  | 185 | compileETag — accepted values of the `etag` setting | 3.4 |  | 0.492 |
| walker |  | 3145 | 71 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 2, sub: 0, line: 59 } |  |  | 0.492 |
| ns | 3357 |  | 217 | compileQueryParser — accepted values of `query parser` | 3.5 |  | 0.470 |
| ns | 3556 |  | 199 | compileTrust — accepted values of `trust proxy` | 3.6 |  | 0.455 |
| walker |  | 3605 | 460 | Json::Dependencies { file: package.json } |  |  | 0.457 |
| ns | 3688 |  | 132 | req.ip / req.ips — the consumers of `trust proxy fn` | 3.7 | 2.4 | 0.447 |
| walker |  | 3914 | 309 | Code::CodeKey { rung: Names, file: lib/response.js, decl: 0, sub: 0, line: 0 } |  |  | 0.518 |
| walker |  | 3925 | 11 | Code::CodeKey { rung: Body, file: lib/response.js, decl: 13, sub: 0, line: 696 } |  |  | 0.518 |
| walker |  | 3940 | 15 | Code::CodeKey { rung: Body, file: lib/response.js, decl: 16, sub: 0, line: 794 } |  |  | 0.518 |
| walker |  | 3963 | 23 | Code::CodeKey { rung: Body, file: lib/response.js, decl: 18, sub: 0, line: 875 } |  |  | 0.518 |
| ns | 3981 |  | 293 | Settings read-sites scattered outside defaultConfiguration | 3.8 |  | 0.500 |
| walker |  | 3989 | 26 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 1, sub: 0, line: 42 } |  |  | 0.500 |
| walker |  | 4046 | 57 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 13, sub: 0, line: 696 } |  |  | 0.500 |
| ns | 4101 |  | 120 | app.init() — per-app state and the lazy base router | 3.9 |  | 0.490 |
| walker |  | 4112 | 66 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 11, sub: 0, line: 604 } |  |  | 0.490 |
| walker |  | 4184 | 72 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 14, sub: 0, line: 709 } |  |  | 0.490 |
| walker |  | 4270 | 86 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 18, sub: 0, line: 875 } |  |  | 0.490 |
| ns | 4332 |  | 231 | Sub-app mounting: the 'mount' event and setting inheritance | 3.10 |  | 0.477 |
| walker |  | 4362 | 92 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 5, sub: 0, line: 232 } |  |  | 0.477 |
| walker |  | 4461 | 99 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 6, sub: 0, line: 260 } |  |  | 0.477 |
| ns | 4478 |  | 146 | JSDoc for app.set() | 3.11 |  | 0.468 |
| walker |  | 4575 | 114 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 4, sub: 0, line: 125 } |  |  | 0.468 |
| walker |  | 4707 | 132 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 7, sub: 0, line: 321 } |  |  | 0.468 |
| ns | 4735 |  | 257 | app.handle() — the per-request dispatch entry point | 4.1 | 2.8 | 0.453 |
| walker |  | 4841 | 134 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 17, sub: 0, line: 812 } |  |  | 0.453 |
| walker |  | 4983 | 142 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 19, sub: 0, line: 894 } |  |  | 0.453 |
| ns | 5006 |  | 271 | app.use() — argument/path disambiguation | 4.2 | 2.8 | 0.439 |
| walker |  | 5144 | 161 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 16, sub: 0, line: 794 } |  |  | 0.439 |
| ns | 5275 |  | 269 | app.use() — the sub-app mounting branch | 4.3 | 4.2 | 0.426 |
| walker |  | 5308 | 164 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 12, sub: 0, line: 629 } |  |  | 0.426 |
| walker |  | 5473 | 165 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 2, sub: 0, line: 64 } |  |  | 0.427 |
| ns | 5590 |  | 315 | res.send() body-type dispatch | 4.4 | 2.9 | 0.412 |
| walker |  | 5655 | 182 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 3, sub: 0, line: 97 } |  |  | 0.412 |
| ns | 5771 |  | 181 | View.prototype.lookup() — view file resolution | 4.5 | 2.5 | 0.405 |
| walker |  | 5822 | 167 | Code::CodeKey { rung: Names, file: lib/utils.js, decl: 0, sub: 0, line: 0 } |  |  | 0.426 |
| walker |  | 5834 | 12 | Code::CodeKey { rung: Body, file: lib/utils.js, decl: 5, sub: 0, line: 75 } |  |  | 0.426 |
| walker |  | 5873 | 39 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 1, sub: 0, line: 29 } |  |  | 0.426 |
| walker |  | 5918 | 45 | Code::CodeKey { rung: Body, file: lib/utils.js, decl: 4, sub: 0, line: 61 } |  |  | 0.426 |
| walker |  | 5981 | 63 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 7, sub: 0, line: 162 } |  |  | 0.426 |
| walker |  | 6045 | 64 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 6, sub: 0, line: 130 } |  |  | 0.426 |
| walker |  | 6110 | 65 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 4, sub: 0, line: 61 } |  |  | 0.426 |
| walker |  | 6175 | 65 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 5, sub: 0, line: 75 } |  |  | 0.426 |
| ns | 6222 |  | 451 | res.send() response finalization | 4.6 | 4.4 | 0.408 |
| walker |  | 6244 | 69 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 8, sub: 0, line: 194 } |  |  | 0.408 |
| walker |  | 6315 | 71 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 10, sub: 0, line: 249 } |  |  | 0.408 |
| walker |  | 6387 | 72 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 2, sub: 0, line: 40 } |  |  | 0.408 |
| walker |  | 6459 | 72 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 3, sub: 0, line: 51 } |  |  | 0.408 |
| walker |  | 6532 | 73 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 9, sub: 0, line: 225 } |  |  | 0.408 |
| ns | 6598 |  | 376 | test/ listing — every spec file | 5.1 |  | 0.475 |
| walker |  | 6612 | 80 | Code::CodeKey { rung: Doc, file: lib/request.js, decl: 5, sub: 0, line: 185 } |  |  | 0.475 |
| ns | 6684 |  | 86 | test/acceptance/ listing | 5.2 |  | 0.488 |
| walker |  | 6693 | 81 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 4, sub: 0, line: 152 } |  |  | 0.488 |
| ns | 6785 |  | 101 | examples/ listing | 5.3 |  | 0.504 |
| walker |  | 6839 | 146 | Markdown::Section { file: Readme.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.504 |
| walker |  | 6927 | 88 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 6, sub: 0, line: 256 } |  |  | 0.504 |
| ns | 7041 |  | 256 | examples/README.md — annotated example index (first half) | 5.4 |  | 0.499 |
| walker |  | 7139 | 212 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 9, sub: 0, line: 433 } |  |  | 0.499 |
| ns | 7299 |  | 258 | examples/README.md — annotated example index (second half) | 5.5 | 5.4 | 0.494 |
| walker |  | 7356 | 217 | Code::CodeKey { rung: Body, file: lib/express.js, decl: 1, sub: 0, line: 36 } |  |  | 0.519 |
| ns | 7390 |  | 91 | test/support/ and test/fixtures/ listings | 5.6 |  | 0.511 |
| walker |  | 7451 | 95 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 5, sub: 0, line: 190 } |  |  | 0.511 |
| ns | 7542 |  | 152 | Test harness: env preamble, assertion helpers, template engine | 5.7 |  | 0.508 |
| walker |  | 7546 | 95 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 15, sub: 0, line: 494 } |  |  | 0.508 |
| ns | 7673 |  | 131 | A complete example app: examples/hello-world/index.js | 5.8 |  | 0.502 |
| walker |  | 7788 | 242 | Markdown::Section { file: Readme.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.502 |
| ns | 7834 |  | 161 | Test-writing idiom: head of test/app.js | 5.9 |  | 0.497 |
| walker |  | 8063 | 275 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 15, sub: 0, line: 742 } |  |  | 0.498 |
| ns | 8120 |  | 286 | Runtime dependencies (all 28) | 6.1 |  | 0.514 |
| walker |  | 8182 | 119 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 8, sub: 0, line: 322 } |  |  | 0.514 |
| walker |  | 8303 | 121 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 12, sub: 0, line: 439 } |  |  | 0.514 |
| ns | 8346 |  | 226 | ci.yml — jobs and the OS/Node matrix | 6.2 |  | 0.507 |
| walker |  | 8427 | 124 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 10, sub: 0, line: 399 } |  |  | 0.507 |
| ns | 8533 |  | 187 | .eslintrc.yml — the complete lint rule set | 6.3 |  | 0.503 |
| walker |  | 8551 | 124 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 11, sub: 0, line: 420 } |  |  | 0.503 |
| walker |  | 8680 | 129 | Code::CodeKey { rung: Doc, file: lib/view.js, decl: 1, sub: 0, line: 52 } |  |  | 0.503 |
| ns | 8807 |  | 274 | Dev dependencies (all 16) | 6.4 |  | 0.497 |
| walker |  | 8821 | 141 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 9, sub: 0, line: 351 } |  |  | 0.510 |
| ns | 8888 |  | 81 | package.json remainder: author, published files | 6.5 |  | 0.509 |
| walker |  | 9066 | 245 | Markdown::Section { file: Readme.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.509 |
| walker |  | 9104 | 38 | Code::CodeKey { rung: Body, file: lib/request.js, decl: 6, sub: 0, line: 214 } |  |  | 0.509 |
| ns | 9121 |  | 233 | res.sendFile() option bag (JSDoc) | 7.1 |  | 0.504 |
| walker |  | 9263 | 159 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 16, sub: 0, line: 522 } |  |  | 0.504 |
| ns | 9391 |  | 270 | res.cookie() option bag (JSDoc) | 7.2 |  | 0.514 |
| ns | 9673 |  | 282 | res.status() and res.render() contracts (JSDoc) | 7.3 |  | 0.523 |
| walker |  | 9764 | 501 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 8, sub: 0, line: 371 } |  |  | 0.535 |
| ns | 9971 |  | 298 | History.md — unreleased section and the 5.2.1 heading | 8.1 |  | 0.531 |
| walker |  | 9987 | 223 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 17, sub: 0, line: 598 } |  |  | 0.531 |
| walker |  | 9994 | 7 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 10, sub: 0, line: 569 } |  |  | 0.531 |
