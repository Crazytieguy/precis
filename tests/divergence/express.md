Score(3000)=0.586 I=0.837 C=0.410 ns_rows≤3K=19/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.729/0.704/0.558/0.586/0.516/0.408/0.475

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 43 | 43 | Fs::DirListing { dir: . } |  |  | 1.000 |
| ns | 43 |  | 43 | Repository root listing | 1.1 |  | 1.000 |
| ns | 135 |  | 92 | package.json identity | 1.2 |  | 0.795 |
| ns | 159 |  | 24 | lib/ listing — the entire shipped library | 1.3 |  | 0.674 |
| walker |  | 224 | 181 | Markdown::ReadmeHeadline { file: Readme.md } |  |  | 0.705 |
| walker |  | 248 | 24 | Fs::DirListing { dir: lib } |  |  | 0.873 |
| ns | 296 |  | 137 | Module export surface (index.js + lib/express.js exports) | 1.4 |  | 0.689 |
| walker |  | 311 | 63 | Json::Identity { file: package.json } |  |  | 0.746 |
| walker |  | 319 | 8 | Fs::DirListing { dir: .github } |  |  | 0.747 |
| walker |  | 337 | 18 | Fs::DirListing { dir: .github/workflows } |  |  | 0.754 |
| walker |  | 368 | 31 | Json::Runtime { file: package.json } |  |  | 0.754 |
| walker |  | 418 | 50 | Json::Entry { file: package.json } |  |  | 0.755 |
| ns | 443 |  | 147 | Readme tagline + canonical quick-start snippet | 1.5 |  | 0.793 |
| walker |  | 546 | 128 | Markdown::HeadingsOutline { file: Readme.md } |  |  | 0.793 |
| walker |  | 633 | 87 | Markdown::Section { file: Readme.md, section_index: 3, keeps_default_concavity: true } |  |  | 0.793 |
| ns | 652 |  | 209 | npm scripts + engines | 1.6 |  | 0.708 |
| ns | 678 |  | 26 | .github listing — all four workflows | 1.7 |  | 0.723 |
| walker |  | 734 | 101 | Fs::DirListing { dir: examples } |  |  | 0.726 |
| ns | 789 |  | 111 | Prototype bases and module exports of each lib file | 2.1 |  | 0.685 |
| walker |  | 867 | 133 | Code::CodeKey { rung: Names, file: lib/express.js, decl: 0, sub: 0, line: 0 } |  |  | 0.787 |
| ns | 962 |  | 173 | app.* roster — every application method name | 2.2 |  | 0.693 |
| walker |  | 1043 | 176 | Json::Scripts { file: package.json } |  |  | 0.776 |
| walker |  | 1058 | 15 | Code::CodeKey { rung: Names, file: index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.796 |
| ns | 1201 |  | 239 | res.* roster — every response method name | 2.3 |  | 0.695 |
| walker |  | 1434 | 376 | Fs::DirListing { dir: test } |  |  | 0.704 |
| walker |  | 1446 | 12 | Fs::DirListing { dir: test/support } |  |  | 0.704 |
| ns | 1465 |  | 264 | req.* roster — methods and defineGetter properties | 2.4 |  | 0.639 |
| walker |  | 1532 | 86 | Fs::DirListing { dir: test/acceptance } |  |  | 0.640 |
| walker |  | 1663 | 131 | Markdown::Section { file: Readme.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.600 |
| ns | 1663 |  | 198 | lib/view.js and lib/utils.js symbol rosters | 2.5 |  | 0.600 |
| walker |  | 1787 | 124 | Markdown::Section { file: Readme.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.600 |
| walker |  | 1829 | 42 | Code::CodeKey { rung: Doc, file: lib/express.js, decl: 1, sub: 0, line: 36 } |  |  | 0.600 |
| ns | 1896 |  | 233 | createApplication() body | 2.6 |  | 0.557 |
| ns | 2153 |  | 257 | HTTP-verb delegation and app.all() | 2.7 |  | 0.516 |
| ns | 2244 |  | 91 | app.* full signature lines | 2.8 | 2.2 | 0.493 |
| walker |  | 2289 | 460 | Json::Dependencies { file: package.json } |  |  | 0.495 |
| ns | 2369 |  | 125 | res.* full signature lines | 2.9 | 2.3 | 0.468 |
| ns | 2478 |  | 109 | Default settings established at boot | 3.1 |  | 0.457 |
| walker |  | 2598 | 309 | Code::CodeKey { rung: Names, file: lib/response.js, decl: 0, sub: 0, line: 0 } |  |  | 0.551 |
| walker |  | 2609 | 11 | Code::CodeKey { rung: Body, file: lib/response.js, decl: 13, sub: 0, line: 696 } |  |  | 0.551 |
| walker |  | 2635 | 26 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 1, sub: 0, line: 42 } |  |  | 0.551 |
| walker |  | 2650 | 15 | Code::CodeKey { rung: Body, file: lib/response.js, decl: 16, sub: 0, line: 794 } |  |  | 0.551 |
| ns | 2651 |  | 173 | Remaining boot configuration: locals, mountpath, view defaults | 3.2 |  | 0.529 |
| walker |  | 2903 | 253 | Code::CodeKey { rung: Names, file: lib/application.js, decl: 0, sub: 0, line: 0 } |  |  | 0.617 |
| walker |  | 2921 | 18 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 1, sub: 0, line: 40 } |  |  | 0.617 |
| ns | 2955 |  | 304 | app.set() — storage plus the three derived-setting side effects | 3.3 |  | 0.578 |
| walker |  | 3040 | 119 | Code::CodeKey { rung: Names, file: lib/request.js, decl: 0, sub: 0, line: 0 } |  |  | 0.588 |
| walker |  | 3053 | 13 | Code::CodeKey { rung: Body, file: lib/request.js, decl: 5, sub: 0, line: 185 } |  |  | 0.588 |
| ns | 3140 |  | 185 | compileETag — accepted values of the `etag` setting | 3.4 |  | 0.564 |
| walker |  | 3220 | 167 | Code::CodeKey { rung: Names, file: lib/utils.js, decl: 0, sub: 0, line: 0 } |  |  | 0.580 |
| walker |  | 3232 | 12 | Code::CodeKey { rung: Body, file: lib/utils.js, decl: 5, sub: 0, line: 75 } |  |  | 0.580 |
| walker |  | 3307 | 75 | Code::CodeKey { rung: Names, file: lib/view.js, decl: 0, sub: 0, line: 0 } |  |  | 0.602 |
| walker |  | 3318 | 11 | Code::CodeKey { rung: Body, file: lib/application.js, decl: 6, sub: 0, line: 256 } |  |  | 0.602 |
| walker |  | 3329 | 11 | Code::CodeKey { rung: Body, file: lib/application.js, decl: 11, sub: 0, line: 420 } |  |  | 0.602 |
| walker |  | 3340 | 11 | Code::CodeKey { rung: Body, file: lib/application.js, decl: 12, sub: 0, line: 439 } |  |  | 0.602 |
| ns | 3357 |  | 217 | compileQueryParser — accepted values of `query parser` | 3.5 |  | 0.576 |
| walker |  | 3486 | 146 | Markdown::Section { file: Readme.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.576 |
| walker |  | 3509 | 23 | Code::CodeKey { rung: Body, file: lib/response.js, decl: 18, sub: 0, line: 875 } |  |  | 0.576 |
| ns | 3556 |  | 199 | compileTrust — accepted values of `trust proxy` | 3.6 |  | 0.557 |
| walker |  | 3566 | 57 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 13, sub: 0, line: 696 } |  |  | 0.557 |
| walker |  | 3578 | 12 | Code::CodeKey { rung: Body, file: lib/application.js, decl: 13, sub: 0, line: 451 } |  |  | 0.557 |
| ns | 3688 |  | 132 | req.ip / req.ips — the consumers of `trust proxy fn` | 3.7 | 2.4 | 0.545 |
| walker |  | 3820 | 242 | Markdown::Section { file: Readme.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.545 |
| walker |  | 3846 | 26 | Code::CodeKey { rung: Doc, file: lib/request.js, decl: 1, sub: 0, line: 30 } |  |  | 0.545 |
| walker |  | 3858 | 12 | Code::CodeKey { rung: Body, file: lib/application.js, decl: 14, sub: 0, line: 463 } |  |  | 0.545 |
| walker |  | 3885 | 27 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 3, sub: 0, line: 90 } |  |  | 0.545 |
| walker |  | 3951 | 66 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 11, sub: 0, line: 604 } |  |  | 0.545 |
| ns | 3981 |  | 293 | Settings read-sites scattered outside defaultConfiguration | 3.8 |  | 0.527 |
| walker |  | 4023 | 72 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 14, sub: 0, line: 709 } |  |  | 0.527 |
| ns | 4101 |  | 120 | app.init() — per-app state and the lazy base router | 3.9 |  | 0.516 |
| walker |  | 4109 | 86 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 18, sub: 0, line: 875 } |  |  | 0.516 |
| walker |  | 4133 | 24 | Code::CodeKey { rung: Body, file: lib/request.js, decl: 4, sub: 0, line: 171 } |  |  | 0.516 |
| walker |  | 4172 | 39 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 1, sub: 0, line: 29 } |  |  | 0.516 |
| walker |  | 4264 | 92 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 5, sub: 0, line: 232 } |  |  | 0.516 |
| ns | 4332 |  | 231 | Sub-app mounting: the 'mount' event and setting inheritance | 3.10 |  | 0.503 |
| ns | 4478 |  | 146 | JSDoc for app.set() | 3.11 |  | 0.493 |
| walker |  | 4509 | 245 | Markdown::Section { file: Readme.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.493 |
| walker |  | 4534 | 25 | Code::CodeKey { rung: Body, file: lib/request.js, decl: 2, sub: 0, line: 127 } |  |  | 0.493 |
| walker |  | 4633 | 99 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 6, sub: 0, line: 260 } |  |  | 0.493 |
| walker |  | 4659 | 26 | Code::CodeKey { rung: Body, file: lib/request.js, decl: 3, sub: 0, line: 140 } |  |  | 0.493 |
| walker |  | 4707 | 48 | Code::CodeKey { rung: Doc, file: lib/view.js, decl: 2, sub: 0, line: 104 } |  |  | 0.493 |
| ns | 4735 |  | 257 | app.handle() — the per-request dispatch entry point | 4.1 | 2.8 | 0.477 |
| walker |  | 4821 | 114 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 4, sub: 0, line: 125 } |  |  | 0.477 |
| walker |  | 4953 | 132 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 7, sub: 0, line: 321 } |  |  | 0.477 |
| ns | 5006 |  | 271 | app.use() — argument/path disambiguation | 4.2 | 2.8 | 0.461 |
| walker |  | 5087 | 134 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 17, sub: 0, line: 812 } |  |  | 0.461 |
| walker |  | 5144 | 57 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 13, sub: 0, line: 451 } |  |  | 0.461 |
| walker |  | 5201 | 57 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 14, sub: 0, line: 463 } |  |  | 0.461 |
| walker |  | 5259 | 58 | Code::CodeKey { rung: Doc, file: lib/view.js, decl: 3, sub: 0, line: 133 } |  |  | 0.461 |
| ns | 5275 |  | 269 | app.use() — the sub-app mounting branch | 4.3 | 4.2 | 0.448 |
| walker |  | 5319 | 60 | Code::CodeKey { rung: Doc, file: lib/view.js, decl: 4, sub: 0, line: 169 } |  |  | 0.448 |
| walker |  | 5461 | 142 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 19, sub: 0, line: 894 } |  |  | 0.449 |
| walker |  | 5524 | 63 | Code::CodeKey { rung: Doc, file: lib/request.js, decl: 3, sub: 0, line: 140 } |  |  | 0.449 |
| walker |  | 5587 | 63 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 7, sub: 0, line: 162 } |  |  | 0.449 |
| ns | 5590 |  | 315 | res.send() body-type dispatch | 4.4 | 2.9 | 0.434 |
| walker |  | 5651 | 64 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 6, sub: 0, line: 130 } |  |  | 0.434 |
| walker |  | 5716 | 65 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 4, sub: 0, line: 61 } |  |  | 0.434 |
| ns | 5771 |  | 181 | View.prototype.lookup() — view file resolution | 4.5 | 2.5 | 0.425 |
| walker |  | 5781 | 65 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 5, sub: 0, line: 75 } |  |  | 0.425 |
| walker |  | 5942 | 161 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 16, sub: 0, line: 794 } |  |  | 0.425 |
| walker |  | 6011 | 69 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 8, sub: 0, line: 194 } |  |  | 0.425 |
| walker |  | 6175 | 164 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 12, sub: 0, line: 629 } |  |  | 0.425 |
| ns | 6222 |  | 451 | res.send() response finalization | 4.6 | 4.4 | 0.408 |
| walker |  | 6340 | 165 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 2, sub: 0, line: 64 } |  |  | 0.408 |
| walker |  | 6411 | 71 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 2, sub: 0, line: 59 } |  |  | 0.408 |
| walker |  | 6482 | 71 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 10, sub: 0, line: 249 } |  |  | 0.408 |
| walker |  | 6554 | 72 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 2, sub: 0, line: 40 } |  |  | 0.408 |
| ns | 6598 |  | 376 | test/ listing — every spec file | 5.1 |  | 0.475 |
| walker |  | 6626 | 72 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 3, sub: 0, line: 51 } |  |  | 0.475 |
| ns | 6684 |  | 86 | test/acceptance/ listing | 5.2 |  | 0.488 |
| walker |  | 6699 | 73 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 9, sub: 0, line: 225 } |  |  | 0.488 |
| ns | 6785 |  | 101 | examples/ listing | 5.3 |  | 0.504 |
| walker |  | 7027 | 328 | Markdown::Section { file: Readme.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.504 |
| ns | 7041 |  | 256 | examples/README.md — annotated example index (first half) | 5.4 |  | 0.499 |
| walker |  | 7210 | 183 | Markdown::Section { file: Readme.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.499 |
| ns | 7299 |  | 258 | examples/README.md — annotated example index (second half) | 5.5 | 5.4 | 0.494 |
| ns | 7390 |  | 91 | test/support/ and test/fixtures/ listings | 5.6 |  | 0.487 |
| walker |  | 7392 | 182 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 3, sub: 0, line: 97 } |  |  | 0.487 |
| walker |  | 7430 | 38 | Code::CodeKey { rung: Body, file: lib/request.js, decl: 6, sub: 0, line: 214 } |  |  | 0.487 |
| walker |  | 7510 | 80 | Code::CodeKey { rung: Doc, file: lib/request.js, decl: 5, sub: 0, line: 185 } |  |  | 0.487 |
| ns | 7542 |  | 152 | Test harness: env preamble, assertion helpers, template engine | 5.7 |  | 0.484 |
| ns | 7673 |  | 131 | A complete example app: examples/hello-world/index.js | 5.8 |  | 0.479 |
| ns | 7834 |  | 161 | Test-writing idiom: head of test/app.js | 5.9 |  | 0.473 |
| walker |  | 7864 | 354 | Markdown::Section { file: Readme.md, section_index: 10, keeps_default_concavity: false } |  |  | 0.473 |
| walker |  | 7945 | 81 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 4, sub: 0, line: 152 } |  |  | 0.473 |
| walker |  | 8033 | 88 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 6, sub: 0, line: 256 } |  |  | 0.473 |
| ns | 8120 |  | 286 | Runtime dependencies (all 28) | 6.1 |  | 0.490 |
| walker |  | 8245 | 212 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 9, sub: 0, line: 433 } |  |  | 0.490 |
| walker |  | 8340 | 95 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 5, sub: 0, line: 190 } |  |  | 0.490 |
| ns | 8346 |  | 226 | ci.yml — jobs and the OS/Node matrix | 6.2 |  | 0.484 |
| walker |  | 8435 | 95 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 15, sub: 0, line: 494 } |  |  | 0.484 |
| ns | 8533 |  | 187 | .eslintrc.yml — the complete lint rule set | 6.3 |  | 0.480 |
| walker |  | 8710 | 275 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 15, sub: 0, line: 742 } |  |  | 0.481 |
| ns | 8807 |  | 274 | Dev dependencies (all 16) | 6.4 |  | 0.475 |
| walker |  | 8829 | 119 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 8, sub: 0, line: 322 } |  |  | 0.475 |
| ns | 8888 |  | 81 | package.json remainder: author, published files | 6.5 |  | 0.475 |
| walker |  | 8950 | 121 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 12, sub: 0, line: 439 } |  |  | 0.475 |
| walker |  | 9074 | 124 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 10, sub: 0, line: 399 } |  |  | 0.475 |
| ns | 9121 |  | 233 | res.sendFile() option bag (JSDoc) | 7.1 |  | 0.470 |
| walker |  | 9198 | 124 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 11, sub: 0, line: 420 } |  |  | 0.470 |
| walker |  | 9327 | 129 | Code::CodeKey { rung: Doc, file: lib/view.js, decl: 1, sub: 0, line: 52 } |  |  | 0.470 |
| ns | 9391 |  | 270 | res.cookie() option bag (JSDoc) | 7.2 |  | 0.482 |
| walker |  | 9468 | 141 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 9, sub: 0, line: 351 } |  |  | 0.494 |
| ns | 9673 |  | 282 | res.status() and res.render() contracts (JSDoc) | 7.3 |  | 0.503 |
| walker |  | 9685 | 217 | Code::CodeKey { rung: Body, file: lib/express.js, decl: 1, sub: 0, line: 36 } |  |  | 0.523 |
| walker |  | 9844 | 159 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 16, sub: 0, line: 522 } |  |  | 0.523 |
| ns | 9971 |  | 298 | History.md — unreleased section and the 5.2.1 heading | 8.1 |  | 0.519 |
