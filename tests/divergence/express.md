Score(3000)=0.507 I=0.807 C=0.318 ns_rows≤3K=19/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.603/0.604/0.485/0.507/0.490/0.421/0.506

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
| walker |  | 1149 | 176 | Json::Scripts { file: package.json } |  |  | 0.690 |
| walker |  | 1164 | 15 | Code::CodeKey { rung: Names, file: index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.691 |
| ns | 1201 |  | 239 | res.* roster — every response method name | 2.3 |  | 0.604 |
| ns | 1465 |  | 264 | req.* roster — methods and defineGetter properties | 2.4 |  | 0.548 |
| walker |  | 1540 | 376 | Fs::DirListing { dir: test } |  |  | 0.555 |
| walker |  | 1552 | 12 | Fs::DirListing { dir: test/support } |  |  | 0.555 |
| ns | 1663 |  | 198 | lib/view.js and lib/utils.js symbol rosters | 2.5 |  | 0.520 |
| ns | 1896 |  | 233 | createApplication() body | 2.6 |  | 0.483 |
| walker |  | 2012 | 460 | Json::Dependencies { file: package.json } |  |  | 0.485 |
| walker |  | 2098 | 86 | Fs::DirListing { dir: test/acceptance } |  |  | 0.486 |
| ns | 2153 |  | 257 | HTTP-verb delegation and app.all() | 2.7 |  | 0.450 |
| walker |  | 2231 | 133 | Code::CodeKey { rung: Names, file: lib/express.js, decl: 0, sub: 0, line: 0 } |  |  | 0.518 |
| ns | 2244 |  | 91 | app.* full signature lines | 2.8 | 2.2 | 0.495 |
| walker |  | 2273 | 42 | Code::CodeKey { rung: Doc, file: lib/express.js, decl: 1, sub: 0, line: 36 } |  |  | 0.495 |
| ns | 2369 |  | 125 | res.* full signature lines | 2.9 | 2.3 | 0.468 |
| walker |  | 2392 | 119 | Code::CodeKey { rung: Names, file: lib/request.js, decl: 0, sub: 0, line: 0 } |  |  | 0.475 |
| walker |  | 2405 | 13 | Code::CodeKey { rung: Body, file: lib/request.js, decl: 5, sub: 0, line: 185 } |  |  | 0.475 |
| walker |  | 2429 | 24 | Code::CodeKey { rung: Body, file: lib/request.js, decl: 4, sub: 0, line: 171 } |  |  | 0.475 |
| walker |  | 2454 | 25 | Code::CodeKey { rung: Body, file: lib/request.js, decl: 2, sub: 0, line: 127 } |  |  | 0.475 |
| ns | 2478 |  | 109 | Default settings established at boot | 3.1 |  | 0.463 |
| walker |  | 2480 | 26 | Code::CodeKey { rung: Doc, file: lib/request.js, decl: 1, sub: 0, line: 30 } |  |  | 0.463 |
| walker |  | 2506 | 26 | Code::CodeKey { rung: Body, file: lib/request.js, decl: 3, sub: 0, line: 140 } |  |  | 0.463 |
| walker |  | 2569 | 63 | Code::CodeKey { rung: Doc, file: lib/request.js, decl: 3, sub: 0, line: 140 } |  |  | 0.463 |
| ns | 2651 |  | 173 | Remaining boot configuration: locals, mountpath, view defaults | 3.2 |  | 0.445 |
| walker |  | 2822 | 253 | Code::CodeKey { rung: Names, file: lib/application.js, decl: 0, sub: 0, line: 0 } |  |  | 0.541 |
| walker |  | 2833 | 11 | Code::CodeKey { rung: Body, file: lib/application.js, decl: 6, sub: 0, line: 256 } |  |  | 0.541 |
| walker |  | 2844 | 11 | Code::CodeKey { rung: Body, file: lib/application.js, decl: 11, sub: 0, line: 420 } |  |  | 0.541 |
| walker |  | 2855 | 11 | Code::CodeKey { rung: Body, file: lib/application.js, decl: 12, sub: 0, line: 439 } |  |  | 0.541 |
| walker |  | 2867 | 12 | Code::CodeKey { rung: Body, file: lib/application.js, decl: 13, sub: 0, line: 451 } |  |  | 0.541 |
| walker |  | 2879 | 12 | Code::CodeKey { rung: Body, file: lib/application.js, decl: 14, sub: 0, line: 463 } |  |  | 0.541 |
| walker |  | 2897 | 18 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 1, sub: 0, line: 40 } |  |  | 0.541 |
| walker |  | 2924 | 27 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 3, sub: 0, line: 90 } |  |  | 0.541 |
| ns | 2955 |  | 304 | app.set() — storage plus the three derived-setting side effects | 3.3 |  | 0.507 |
| walker |  | 2981 | 57 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 13, sub: 0, line: 451 } |  |  | 0.507 |
| walker |  | 3038 | 57 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 14, sub: 0, line: 463 } |  |  | 0.507 |
| walker |  | 3113 | 75 | Code::CodeKey { rung: Names, file: lib/view.js, decl: 0, sub: 0, line: 0 } |  |  | 0.514 |
| ns | 3140 |  | 185 | compileETag — accepted values of the `etag` setting | 3.4 |  | 0.493 |
| walker |  | 3161 | 48 | Code::CodeKey { rung: Doc, file: lib/view.js, decl: 2, sub: 0, line: 104 } |  |  | 0.493 |
| walker |  | 3219 | 58 | Code::CodeKey { rung: Doc, file: lib/view.js, decl: 3, sub: 0, line: 133 } |  |  | 0.493 |
| walker |  | 3279 | 60 | Code::CodeKey { rung: Doc, file: lib/view.js, decl: 4, sub: 0, line: 169 } |  |  | 0.493 |
| walker |  | 3350 | 71 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 2, sub: 0, line: 59 } |  |  | 0.493 |
| ns | 3357 |  | 217 | compileQueryParser — accepted values of `query parser` | 3.5 |  | 0.472 |
| ns | 3556 |  | 199 | compileTrust — accepted values of `trust proxy` | 3.6 |  | 0.457 |
| walker |  | 3659 | 309 | Code::CodeKey { rung: Names, file: lib/response.js, decl: 0, sub: 0, line: 0 } |  |  | 0.529 |
| walker |  | 3670 | 11 | Code::CodeKey { rung: Body, file: lib/response.js, decl: 13, sub: 0, line: 696 } |  |  | 0.529 |
| walker |  | 3685 | 15 | Code::CodeKey { rung: Body, file: lib/response.js, decl: 16, sub: 0, line: 794 } |  |  | 0.529 |
| ns | 3688 |  | 132 | req.ip / req.ips — the consumers of `trust proxy fn` | 3.7 | 2.4 | 0.518 |
| walker |  | 3708 | 23 | Code::CodeKey { rung: Body, file: lib/response.js, decl: 18, sub: 0, line: 875 } |  |  | 0.518 |
| walker |  | 3734 | 26 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 1, sub: 0, line: 42 } |  |  | 0.518 |
| walker |  | 3791 | 57 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 13, sub: 0, line: 696 } |  |  | 0.518 |
| walker |  | 3857 | 66 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 11, sub: 0, line: 604 } |  |  | 0.518 |
| walker |  | 3929 | 72 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 14, sub: 0, line: 709 } |  |  | 0.518 |
| ns | 3981 |  | 293 | Settings read-sites scattered outside defaultConfiguration | 3.8 |  | 0.500 |
| walker |  | 4015 | 86 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 18, sub: 0, line: 875 } |  |  | 0.500 |
| ns | 4101 |  | 120 | app.init() — per-app state and the lazy base router | 3.9 |  | 0.490 |
| walker |  | 4107 | 92 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 5, sub: 0, line: 232 } |  |  | 0.490 |
| walker |  | 4206 | 99 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 6, sub: 0, line: 260 } |  |  | 0.490 |
| walker |  | 4320 | 114 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 4, sub: 0, line: 125 } |  |  | 0.490 |
| ns | 4332 |  | 231 | Sub-app mounting: the 'mount' event and setting inheritance | 3.10 |  | 0.477 |
| walker |  | 4452 | 132 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 7, sub: 0, line: 321 } |  |  | 0.477 |
| ns | 4478 |  | 146 | JSDoc for app.set() | 3.11 |  | 0.468 |
| walker |  | 4586 | 134 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 17, sub: 0, line: 812 } |  |  | 0.468 |
| walker |  | 4728 | 142 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 19, sub: 0, line: 894 } |  |  | 0.469 |
| ns | 4735 |  | 257 | app.handle() — the per-request dispatch entry point | 4.1 | 2.8 | 0.453 |
| walker |  | 4889 | 161 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 16, sub: 0, line: 794 } |  |  | 0.453 |
| ns | 5006 |  | 271 | app.use() — argument/path disambiguation | 4.2 | 2.8 | 0.439 |
| walker |  | 5053 | 164 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 12, sub: 0, line: 629 } |  |  | 0.439 |
| walker |  | 5218 | 165 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 2, sub: 0, line: 64 } |  |  | 0.439 |
| ns | 5275 |  | 269 | app.use() — the sub-app mounting branch | 4.3 | 4.2 | 0.427 |
| walker |  | 5556 | 338 | Json::IdentityMeta { file: package.json } |  |  | 0.441 |
| ns | 5590 |  | 315 | res.send() body-type dispatch | 4.4 | 2.9 | 0.426 |
| walker |  | 5738 | 182 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 3, sub: 0, line: 97 } |  |  | 0.426 |
| ns | 5771 |  | 181 | View.prototype.lookup() — view file resolution | 4.5 | 2.5 | 0.418 |
| walker |  | 5905 | 167 | Code::CodeKey { rung: Names, file: lib/utils.js, decl: 0, sub: 0, line: 0 } |  |  | 0.439 |
| walker |  | 5917 | 12 | Code::CodeKey { rung: Body, file: lib/utils.js, decl: 5, sub: 0, line: 75 } |  |  | 0.439 |
| walker |  | 5956 | 39 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 1, sub: 0, line: 29 } |  |  | 0.439 |
| walker |  | 6001 | 45 | Code::CodeKey { rung: Body, file: lib/utils.js, decl: 4, sub: 0, line: 61 } |  |  | 0.439 |
| walker |  | 6064 | 63 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 7, sub: 0, line: 162 } |  |  | 0.439 |
| walker |  | 6128 | 64 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 6, sub: 0, line: 130 } |  |  | 0.439 |
| walker |  | 6193 | 65 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 4, sub: 0, line: 61 } |  |  | 0.439 |
| ns | 6222 |  | 451 | res.send() response finalization | 4.6 | 4.4 | 0.421 |
| walker |  | 6258 | 65 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 5, sub: 0, line: 75 } |  |  | 0.421 |
| walker |  | 6327 | 69 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 8, sub: 0, line: 194 } |  |  | 0.421 |
| walker |  | 6398 | 71 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 10, sub: 0, line: 249 } |  |  | 0.421 |
| walker |  | 6470 | 72 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 2, sub: 0, line: 40 } |  |  | 0.421 |
| walker |  | 6542 | 72 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 3, sub: 0, line: 51 } |  |  | 0.421 |
| ns | 6598 |  | 376 | test/ listing — every spec file | 5.1 |  | 0.488 |
| walker |  | 6615 | 73 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 9, sub: 0, line: 225 } |  |  | 0.488 |
| ns | 6684 |  | 86 | test/acceptance/ listing | 5.2 |  | 0.500 |
| walker |  | 6695 | 80 | Code::CodeKey { rung: Doc, file: lib/request.js, decl: 5, sub: 0, line: 185 } |  |  | 0.500 |
| walker |  | 6776 | 81 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 4, sub: 0, line: 152 } |  |  | 0.500 |
| ns | 6785 |  | 101 | examples/ listing | 5.3 |  | 0.517 |
| walker |  | 6864 | 88 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 6, sub: 0, line: 256 } |  |  | 0.517 |
| ns | 7041 |  | 256 | examples/README.md — annotated example index (first half) | 5.4 |  | 0.512 |
| walker |  | 7076 | 212 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 9, sub: 0, line: 433 } |  |  | 0.512 |
| walker |  | 7171 | 95 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 5, sub: 0, line: 190 } |  |  | 0.512 |
| walker |  | 7266 | 95 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 15, sub: 0, line: 494 } |  |  | 0.512 |
| ns | 7299 |  | 258 | examples/README.md — annotated example index (second half) | 5.5 | 5.4 | 0.507 |
| ns | 7390 |  | 91 | test/support/ and test/fixtures/ listings | 5.6 |  | 0.499 |
| walker |  | 7397 | 131 | Markdown::Section { file: Readme.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.499 |
| walker |  | 7521 | 124 | Markdown::Section { file: Readme.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.499 |
| ns | 7542 |  | 152 | Test harness: env preamble, assertion helpers, template engine | 5.7 |  | 0.496 |
| ns | 7673 |  | 131 | A complete example app: examples/hello-world/index.js | 5.8 |  | 0.491 |
| walker |  | 7796 | 275 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 15, sub: 0, line: 742 } |  |  | 0.492 |
| ns | 7834 |  | 161 | Test-writing idiom: head of test/app.js | 5.9 |  | 0.486 |
| walker |  | 7915 | 119 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 8, sub: 0, line: 322 } |  |  | 0.486 |
| walker |  | 8036 | 121 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 12, sub: 0, line: 439 } |  |  | 0.486 |
| ns | 8120 |  | 286 | Runtime dependencies (all 28) | 6.1 |  | 0.503 |
| walker |  | 8160 | 124 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 10, sub: 0, line: 399 } |  |  | 0.503 |
| walker |  | 8284 | 124 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 11, sub: 0, line: 420 } |  |  | 0.503 |
| ns | 8346 |  | 226 | ci.yml — jobs and the OS/Node matrix | 6.2 |  | 0.497 |
| walker |  | 8413 | 129 | Code::CodeKey { rung: Doc, file: lib/view.js, decl: 1, sub: 0, line: 52 } |  |  | 0.497 |
| ns | 8533 |  | 187 | .eslintrc.yml — the complete lint rule set | 6.3 |  | 0.493 |
| walker |  | 8554 | 141 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 9, sub: 0, line: 351 } |  |  | 0.506 |
| walker |  | 8700 | 146 | Markdown::Section { file: Readme.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.506 |
| walker |  | 8738 | 38 | Code::CodeKey { rung: Body, file: lib/request.js, decl: 6, sub: 0, line: 214 } |  |  | 0.506 |
| ns | 8807 |  | 274 | Dev dependencies (all 16) | 6.4 |  | 0.500 |
| ns | 8888 |  | 81 | package.json remainder: author, published files | 6.5 |  | 0.506 |
| walker |  | 8897 | 159 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 16, sub: 0, line: 522 } |  |  | 0.506 |
| ns | 9121 |  | 233 | res.sendFile() option bag (JSDoc) | 7.1 |  | 0.501 |
| walker |  | 9139 | 242 | Markdown::Section { file: Readme.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.501 |
| ns | 9391 |  | 270 | res.cookie() option bag (JSDoc) | 7.2 |  | 0.512 |
| walker |  | 9640 | 501 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 8, sub: 0, line: 371 } |  |  | 0.525 |
| ns | 9673 |  | 282 | res.status() and res.render() contracts (JSDoc) | 7.3 |  | 0.533 |
| walker |  | 9857 | 217 | Code::CodeKey { rung: Body, file: lib/express.js, decl: 1, sub: 0, line: 36 } |  |  | 0.553 |
| ns | 9971 |  | 298 | History.md — unreleased section and the 5.2.1 heading | 8.1 |  | 0.549 |
