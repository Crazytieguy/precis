Score(3000)=0.541 I=0.823 C=0.356 ns_rows≤3K=19/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.729/0.704/0.578/0.541/0.516/0.408/0.496

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
| walker |  | 418 | 181 | Markdown::ReadmeHeadline { file: Readme.md } |  |  | 0.755 |
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
| walker |  | 1948 | 119 | Code::CodeKey { rung: Names, file: lib/request.js, decl: 0, sub: 0, line: 0 } |  |  | 0.566 |
| walker |  | 1961 | 13 | Code::CodeKey { rung: Body, file: lib/request.js, decl: 5, sub: 0, line: 185 } |  |  | 0.566 |
| ns | 2153 |  | 257 | HTTP-verb delegation and app.all() | 2.7 |  | 0.523 |
| walker |  | 2214 | 253 | Code::CodeKey { rung: Names, file: lib/application.js, decl: 0, sub: 0, line: 0 } |  |  | 0.581 |
| walker |  | 2225 | 11 | Code::CodeKey { rung: Body, file: lib/application.js, decl: 6, sub: 0, line: 256 } |  |  | 0.581 |
| walker |  | 2236 | 11 | Code::CodeKey { rung: Body, file: lib/application.js, decl: 11, sub: 0, line: 420 } |  |  | 0.581 |
| ns | 2244 |  | 91 | app.* full signature lines | 2.8 | 2.2 | 0.609 |
| walker |  | 2247 | 11 | Code::CodeKey { rung: Body, file: lib/application.js, decl: 12, sub: 0, line: 439 } |  |  | 0.609 |
| walker |  | 2259 | 12 | Code::CodeKey { rung: Body, file: lib/application.js, decl: 13, sub: 0, line: 451 } |  |  | 0.609 |
| walker |  | 2271 | 12 | Code::CodeKey { rung: Body, file: lib/application.js, decl: 14, sub: 0, line: 463 } |  |  | 0.609 |
| walker |  | 2289 | 18 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 1, sub: 0, line: 40 } |  |  | 0.609 |
| walker |  | 2364 | 75 | Code::CodeKey { rung: Names, file: lib/view.js, decl: 0, sub: 0, line: 0 } |  |  | 0.617 |
| ns | 2369 |  | 125 | res.* full signature lines | 2.9 | 2.3 | 0.584 |
| ns | 2478 |  | 109 | Default settings established at boot | 3.1 |  | 0.570 |
| ns | 2651 |  | 173 | Remaining boot configuration: locals, mountpath, view defaults | 3.2 |  | 0.547 |
| walker |  | 2824 | 460 | Json::Dependencies { file: package.json } |  |  | 0.549 |
| ns | 2955 |  | 304 | app.set() — storage plus the three derived-setting side effects | 3.3 |  | 0.514 |
| walker |  | 3133 | 309 | Code::CodeKey { rung: Names, file: lib/response.js, decl: 0, sub: 0, line: 0 } |  |  | 0.596 |
| ns | 3140 |  | 185 | compileETag — accepted values of the `etag` setting | 3.4 |  | 0.572 |
| walker |  | 3144 | 11 | Code::CodeKey { rung: Body, file: lib/response.js, decl: 13, sub: 0, line: 696 } |  |  | 0.572 |
| walker |  | 3159 | 15 | Code::CodeKey { rung: Body, file: lib/response.js, decl: 16, sub: 0, line: 794 } |  |  | 0.572 |
| walker |  | 3182 | 23 | Code::CodeKey { rung: Body, file: lib/response.js, decl: 18, sub: 0, line: 875 } |  |  | 0.572 |
| walker |  | 3208 | 26 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 1, sub: 0, line: 42 } |  |  | 0.572 |
| ns | 3357 |  | 217 | compileQueryParser — accepted values of `query parser` | 3.5 |  | 0.547 |
| walker |  | 3375 | 167 | Code::CodeKey { rung: Names, file: lib/utils.js, decl: 0, sub: 0, line: 0 } |  |  | 0.576 |
| walker |  | 3387 | 12 | Code::CodeKey { rung: Body, file: lib/utils.js, decl: 5, sub: 0, line: 75 } |  |  | 0.576 |
| walker |  | 3533 | 146 | Markdown::Section { file: Readme.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.576 |
| ns | 3556 |  | 199 | compileTrust — accepted values of `trust proxy` | 3.6 |  | 0.557 |
| walker |  | 3557 | 24 | Code::CodeKey { rung: Body, file: lib/request.js, decl: 4, sub: 0, line: 171 } |  |  | 0.557 |
| walker |  | 3614 | 57 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 13, sub: 0, line: 696 } |  |  | 0.557 |
| walker |  | 3639 | 25 | Code::CodeKey { rung: Body, file: lib/request.js, decl: 2, sub: 0, line: 127 } |  |  | 0.557 |
| ns | 3688 |  | 132 | req.ip / req.ips — the consumers of `trust proxy fn` | 3.7 | 2.4 | 0.545 |
| walker |  | 3881 | 242 | Markdown::Section { file: Readme.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.545 |
| walker |  | 3907 | 26 | Code::CodeKey { rung: Doc, file: lib/request.js, decl: 1, sub: 0, line: 30 } |  |  | 0.545 |
| walker |  | 3934 | 27 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 3, sub: 0, line: 90 } |  |  | 0.545 |
| ns | 3981 |  | 293 | Settings read-sites scattered outside defaultConfiguration | 3.8 |  | 0.527 |
| walker |  | 4000 | 66 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 11, sub: 0, line: 604 } |  |  | 0.527 |
| walker |  | 4072 | 72 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 14, sub: 0, line: 709 } |  |  | 0.527 |
| ns | 4101 |  | 120 | app.init() — per-app state and the lazy base router | 3.9 |  | 0.516 |
| walker |  | 4158 | 86 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 18, sub: 0, line: 875 } |  |  | 0.516 |
| walker |  | 4184 | 26 | Code::CodeKey { rung: Body, file: lib/request.js, decl: 3, sub: 0, line: 140 } |  |  | 0.516 |
| walker |  | 4223 | 39 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 1, sub: 0, line: 29 } |  |  | 0.516 |
| walker |  | 4315 | 92 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 5, sub: 0, line: 232 } |  |  | 0.516 |
| ns | 4332 |  | 231 | Sub-app mounting: the 'mount' event and setting inheritance | 3.10 |  | 0.503 |
| ns | 4478 |  | 146 | JSDoc for app.set() | 3.11 |  | 0.493 |
| walker |  | 4560 | 245 | Markdown::Section { file: Readme.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.493 |
| walker |  | 4659 | 99 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 6, sub: 0, line: 260 } |  |  | 0.493 |
| walker |  | 4704 | 45 | Code::CodeKey { rung: Body, file: lib/utils.js, decl: 4, sub: 0, line: 61 } |  |  | 0.493 |
| ns | 4735 |  | 257 | app.handle() — the per-request dispatch entry point | 4.1 | 2.8 | 0.477 |
| walker |  | 4752 | 48 | Code::CodeKey { rung: Doc, file: lib/view.js, decl: 2, sub: 0, line: 104 } |  |  | 0.477 |
| walker |  | 4866 | 114 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 4, sub: 0, line: 125 } |  |  | 0.477 |
| walker |  | 4998 | 132 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 7, sub: 0, line: 321 } |  |  | 0.477 |
| ns | 5006 |  | 271 | app.use() — argument/path disambiguation | 4.2 | 2.8 | 0.461 |
| walker |  | 5132 | 134 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 17, sub: 0, line: 812 } |  |  | 0.461 |
| walker |  | 5189 | 57 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 13, sub: 0, line: 451 } |  |  | 0.461 |
| walker |  | 5246 | 57 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 14, sub: 0, line: 463 } |  |  | 0.461 |
| ns | 5275 |  | 269 | app.use() — the sub-app mounting branch | 4.3 | 4.2 | 0.448 |
| walker |  | 5304 | 58 | Code::CodeKey { rung: Doc, file: lib/view.js, decl: 3, sub: 0, line: 133 } |  |  | 0.448 |
| walker |  | 5364 | 60 | Code::CodeKey { rung: Doc, file: lib/view.js, decl: 4, sub: 0, line: 169 } |  |  | 0.448 |
| walker |  | 5506 | 142 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 19, sub: 0, line: 894 } |  |  | 0.449 |
| walker |  | 5569 | 63 | Code::CodeKey { rung: Doc, file: lib/request.js, decl: 3, sub: 0, line: 140 } |  |  | 0.449 |
| ns | 5590 |  | 315 | res.send() body-type dispatch | 4.4 | 2.9 | 0.434 |
| walker |  | 5632 | 63 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 7, sub: 0, line: 162 } |  |  | 0.434 |
| walker |  | 5696 | 64 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 6, sub: 0, line: 130 } |  |  | 0.434 |
| walker |  | 5761 | 65 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 4, sub: 0, line: 61 } |  |  | 0.434 |
| ns | 5771 |  | 181 | View.prototype.lookup() — view file resolution | 4.5 | 2.5 | 0.425 |
| walker |  | 5826 | 65 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 5, sub: 0, line: 75 } |  |  | 0.425 |
| walker |  | 5987 | 161 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 16, sub: 0, line: 794 } |  |  | 0.425 |
| walker |  | 6056 | 69 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 8, sub: 0, line: 194 } |  |  | 0.425 |
| walker |  | 6220 | 164 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 12, sub: 0, line: 629 } |  |  | 0.425 |
| ns | 6222 |  | 451 | res.send() response finalization | 4.6 | 4.4 | 0.408 |
| walker |  | 6385 | 165 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 2, sub: 0, line: 64 } |  |  | 0.408 |
| walker |  | 6456 | 71 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 2, sub: 0, line: 59 } |  |  | 0.408 |
| walker |  | 6527 | 71 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 10, sub: 0, line: 249 } |  |  | 0.408 |
| ns | 6598 |  | 376 | test/ listing — every spec file | 5.1 |  | 0.475 |
| walker |  | 6599 | 72 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 2, sub: 0, line: 40 } |  |  | 0.475 |
| walker |  | 6671 | 72 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 3, sub: 0, line: 51 } |  |  | 0.475 |
| ns | 6684 |  | 86 | test/acceptance/ listing | 5.2 |  | 0.488 |
| walker |  | 6744 | 73 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 9, sub: 0, line: 225 } |  |  | 0.488 |
| ns | 6785 |  | 101 | examples/ listing | 5.3 |  | 0.504 |
| ns | 7041 |  | 256 | examples/README.md — annotated example index (first half) | 5.4 |  | 0.499 |
| walker |  | 7072 | 328 | Markdown::Section { file: Readme.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.499 |
| walker |  | 7255 | 183 | Markdown::Section { file: Readme.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.499 |
| ns | 7299 |  | 258 | examples/README.md — annotated example index (second half) | 5.5 | 5.4 | 0.494 |
| ns | 7390 |  | 91 | test/support/ and test/fixtures/ listings | 5.6 |  | 0.487 |
| walker |  | 7437 | 182 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 3, sub: 0, line: 97 } |  |  | 0.487 |
| walker |  | 7517 | 80 | Code::CodeKey { rung: Doc, file: lib/request.js, decl: 5, sub: 0, line: 185 } |  |  | 0.487 |
| ns | 7542 |  | 152 | Test harness: env preamble, assertion helpers, template engine | 5.7 |  | 0.484 |
| ns | 7673 |  | 131 | A complete example app: examples/hello-world/index.js | 5.8 |  | 0.479 |
| ns | 7834 |  | 161 | Test-writing idiom: head of test/app.js | 5.9 |  | 0.473 |
| walker |  | 7871 | 354 | Markdown::Section { file: Readme.md, section_index: 10, keeps_default_concavity: false } |  |  | 0.473 |
| walker |  | 7952 | 81 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 4, sub: 0, line: 152 } |  |  | 0.473 |
| walker |  | 8040 | 88 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 6, sub: 0, line: 256 } |  |  | 0.473 |
| ns | 8120 |  | 286 | Runtime dependencies (all 28) | 6.1 |  | 0.490 |
| walker |  | 8252 | 212 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 9, sub: 0, line: 433 } |  |  | 0.490 |
| ns | 8346 |  | 226 | ci.yml — jobs and the OS/Node matrix | 6.2 |  | 0.484 |
| walker |  | 8469 | 217 | Code::CodeKey { rung: Body, file: lib/express.js, decl: 1, sub: 0, line: 36 } |  |  | 0.506 |
| ns | 8533 |  | 187 | .eslintrc.yml — the complete lint rule set | 6.3 |  | 0.502 |
| walker |  | 8564 | 95 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 5, sub: 0, line: 190 } |  |  | 0.502 |
| walker |  | 8659 | 95 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 15, sub: 0, line: 494 } |  |  | 0.502 |
| ns | 8807 |  | 274 | Dev dependencies (all 16) | 6.4 |  | 0.496 |
| ns | 8888 |  | 81 | package.json remainder: author, published files | 6.5 |  | 0.495 |
| walker |  | 8934 | 275 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 15, sub: 0, line: 742 } |  |  | 0.496 |
| walker |  | 9053 | 119 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 8, sub: 0, line: 322 } |  |  | 0.496 |
| ns | 9121 |  | 233 | res.sendFile() option bag (JSDoc) | 7.1 |  | 0.492 |
| walker |  | 9174 | 121 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 12, sub: 0, line: 439 } |  |  | 0.492 |
| walker |  | 9298 | 124 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 10, sub: 0, line: 399 } |  |  | 0.492 |
| ns | 9391 |  | 270 | res.cookie() option bag (JSDoc) | 7.2 |  | 0.502 |
| walker |  | 9422 | 124 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 11, sub: 0, line: 420 } |  |  | 0.502 |
| walker |  | 9551 | 129 | Code::CodeKey { rung: Doc, file: lib/view.js, decl: 1, sub: 0, line: 52 } |  |  | 0.502 |
| ns | 9673 |  | 282 | res.status() and res.render() contracts (JSDoc) | 7.3 |  | 0.511 |
| walker |  | 9692 | 141 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 9, sub: 0, line: 351 } |  |  | 0.523 |
| walker |  | 9730 | 38 | Code::CodeKey { rung: Body, file: lib/request.js, decl: 6, sub: 0, line: 214 } |  |  | 0.523 |
| walker |  | 9889 | 159 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 16, sub: 0, line: 522 } |  |  | 0.523 |
| ns | 9971 |  | 298 | History.md — unreleased section and the 5.2.1 heading | 8.1 |  | 0.519 |
| walker |  | 10000 | 111 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 8, sub: 0, line: 371 } |  |  | 0.522 |
