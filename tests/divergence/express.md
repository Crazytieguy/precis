Score(3000)=0.621 I=0.850 C=0.453 ns_rows≤3K=19/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.729/0.704/0.608/0.621/0.542/0.429/0.487

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
| ns | 1896 |  | 233 | createApplication() body | 2.6 |  | 0.557 |
| walker |  | 2115 | 328 | Code::CodeKey { rung: Names, file: lib/response.js, decl: 0, sub: 0, line: 0 } |  |  | 0.625 |
| walker |  | 2128 | 13 | Code::CodeKey { rung: Decl, file: lib/response.js, decl: 10, sub: 0, line: 503 } |  |  | 0.630 |
| walker |  | 2142 | 14 | Code::CodeKey { rung: Decl, file: lib/response.js, decl: 14, sub: 0, line: 664 } |  |  | 0.637 |
| ns | 2153 |  | 257 | HTTP-verb delegation and app.all() | 2.7 |  | 0.589 |
| walker |  | 2168 | 26 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 1, sub: 0, line: 42 } |  |  | 0.589 |
| walker |  | 2179 | 11 | Code::CodeKey { rung: Body, file: lib/response.js, decl: 15, sub: 0, line: 696 } |  |  | 0.589 |
| ns | 2244 |  | 91 | app.* full signature lines | 2.8 | 2.2 | 0.564 |
| ns | 2369 |  | 125 | res.* full signature lines | 2.9 | 2.3 | 0.600 |
| walker |  | 2432 | 253 | Code::CodeKey { rung: Names, file: lib/application.js, decl: 0, sub: 0, line: 0 } |  |  | 0.691 |
| ns | 2478 |  | 109 | Default settings established at boot | 3.1 |  | 0.675 |
| walker |  | 2560 | 128 | Code::CodeKey { rung: Names, file: lib/request.js, decl: 0, sub: 0, line: 0 } |  |  | 0.687 |
| walker |  | 2572 | 12 | Code::CodeKey { rung: Decl, file: lib/request.js, decl: 2, sub: 0, line: 63 } |  |  | 0.688 |
| walker |  | 2587 | 15 | Code::CodeKey { rung: Body, file: lib/response.js, decl: 18, sub: 0, line: 794 } |  |  | 0.688 |
| ns | 2651 |  | 173 | Remaining boot configuration: locals, mountpath, view defaults | 3.2 |  | 0.661 |
| ns | 2955 |  | 304 | app.set() — storage plus the three derived-setting side effects | 3.3 |  | 0.619 |
| walker |  | 3047 | 460 | Json::Dependencies { file: package.json } |  |  | 0.621 |
| walker |  | 3089 | 42 | Code::CodeKey { rung: Doc, file: lib/express.js, decl: 1, sub: 0, line: 36 } |  |  | 0.621 |
| walker |  | 3107 | 18 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 1, sub: 0, line: 40 } |  |  | 0.621 |
| ns | 3140 |  | 185 | compileETag — accepted values of the `etag` setting | 3.4 |  | 0.596 |
| walker |  | 3274 | 167 | Code::CodeKey { rung: Names, file: lib/utils.js, decl: 0, sub: 0, line: 0 } |  |  | 0.611 |
| walker |  | 3349 | 75 | Code::CodeKey { rung: Names, file: lib/view.js, decl: 0, sub: 0, line: 0 } |  |  | 0.633 |
| ns | 3357 |  | 217 | compileQueryParser — accepted values of `query parser` | 3.5 |  | 0.605 |
| walker |  | 3361 | 12 | Code::CodeKey { rung: Body, file: lib/utils.js, decl: 5, sub: 0, line: 75 } |  |  | 0.605 |
| walker |  | 3374 | 13 | Code::CodeKey { rung: Body, file: lib/request.js, decl: 6, sub: 0, line: 185 } |  |  | 0.605 |
| walker |  | 3520 | 146 | Markdown::Section { file: Readme.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.605 |
| walker |  | 3531 | 11 | Code::CodeKey { rung: Body, file: lib/application.js, decl: 6, sub: 0, line: 256 } |  |  | 0.605 |
| ns | 3556 |  | 199 | compileTrust — accepted values of `trust proxy` | 3.6 |  | 0.586 |
| ns | 3688 |  | 132 | req.ip / req.ips — the consumers of `trust proxy fn` | 3.7 | 2.4 | 0.573 |
| walker |  | 3773 | 242 | Markdown::Section { file: Readme.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.573 |
| walker |  | 3784 | 11 | Code::CodeKey { rung: Body, file: lib/application.js, decl: 11, sub: 0, line: 420 } |  |  | 0.573 |
| walker |  | 3795 | 11 | Code::CodeKey { rung: Body, file: lib/application.js, decl: 12, sub: 0, line: 439 } |  |  | 0.573 |
| walker |  | 3852 | 57 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 15, sub: 0, line: 696 } |  |  | 0.573 |
| walker |  | 3864 | 12 | Code::CodeKey { rung: Body, file: lib/application.js, decl: 13, sub: 0, line: 451 } |  |  | 0.573 |
| walker |  | 3890 | 26 | Code::CodeKey { rung: Doc, file: lib/request.js, decl: 1, sub: 0, line: 30 } |  |  | 0.573 |
| walker |  | 3902 | 12 | Code::CodeKey { rung: Body, file: lib/application.js, decl: 14, sub: 0, line: 463 } |  |  | 0.573 |
| walker |  | 3929 | 27 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 3, sub: 0, line: 90 } |  |  | 0.573 |
| ns | 3981 |  | 293 | Settings read-sites scattered outside defaultConfiguration | 3.8 |  | 0.553 |
| walker |  | 3995 | 66 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 12, sub: 0, line: 604 } |  |  | 0.553 |
| walker |  | 4067 | 72 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 16, sub: 0, line: 709 } |  |  | 0.553 |
| ns | 4101 |  | 120 | app.init() — per-app state and the lazy base router | 3.9 |  | 0.542 |
| walker |  | 4312 | 245 | Markdown::Section { file: Readme.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.542 |
| ns | 4332 |  | 231 | Sub-app mounting: the 'mount' event and setting inheritance | 3.10 |  | 0.528 |
| walker |  | 4398 | 86 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 20, sub: 0, line: 875 } |  |  | 0.528 |
| walker |  | 4422 | 24 | Code::CodeKey { rung: Body, file: lib/request.js, decl: 5, sub: 0, line: 171 } |  |  | 0.528 |
| walker |  | 4461 | 39 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 1, sub: 0, line: 29 } |  |  | 0.528 |
| ns | 4478 |  | 146 | JSDoc for app.set() | 3.11 |  | 0.518 |
| walker |  | 4553 | 92 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 5, sub: 0, line: 232 } |  |  | 0.518 |
| walker |  | 4652 | 99 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 6, sub: 0, line: 260 } |  |  | 0.518 |
| walker |  | 4677 | 25 | Code::CodeKey { rung: Body, file: lib/request.js, decl: 3, sub: 0, line: 127 } |  |  | 0.518 |
| walker |  | 4703 | 26 | Code::CodeKey { rung: Body, file: lib/request.js, decl: 4, sub: 0, line: 140 } |  |  | 0.518 |
| ns | 4735 |  | 257 | app.handle() — the per-request dispatch entry point | 4.1 | 2.8 | 0.501 |
| walker |  | 4751 | 48 | Code::CodeKey { rung: Doc, file: lib/view.js, decl: 2, sub: 0, line: 104 } |  |  | 0.501 |
| walker |  | 4865 | 114 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 4, sub: 0, line: 125 } |  |  | 0.501 |
| walker |  | 4997 | 132 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 7, sub: 0, line: 321 } |  |  | 0.501 |
| ns | 5006 |  | 271 | app.use() — argument/path disambiguation | 4.2 | 2.8 | 0.485 |
| walker |  | 5131 | 134 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 19, sub: 0, line: 812 } |  |  | 0.485 |
| walker |  | 5188 | 57 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 13, sub: 0, line: 451 } |  |  | 0.485 |
| walker |  | 5245 | 57 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 14, sub: 0, line: 463 } |  |  | 0.485 |
| ns | 5275 |  | 269 | app.use() — the sub-app mounting branch | 4.3 | 4.2 | 0.471 |
| walker |  | 5303 | 58 | Code::CodeKey { rung: Doc, file: lib/view.js, decl: 3, sub: 0, line: 133 } |  |  | 0.471 |
| walker |  | 5363 | 60 | Code::CodeKey { rung: Doc, file: lib/view.js, decl: 4, sub: 0, line: 169 } |  |  | 0.471 |
| walker |  | 5505 | 142 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 21, sub: 0, line: 894 } |  |  | 0.472 |
| ns | 5590 |  | 315 | res.send() body-type dispatch | 4.4 | 2.9 | 0.456 |
| ns | 5771 |  | 181 | View.prototype.lookup() — view file resolution | 4.5 | 2.5 | 0.447 |
| walker |  | 5833 | 328 | Markdown::Section { file: Readme.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.447 |
| walker |  | 6016 | 183 | Markdown::Section { file: Readme.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.447 |
| walker |  | 6079 | 63 | Code::CodeKey { rung: Doc, file: lib/request.js, decl: 4, sub: 0, line: 140 } |  |  | 0.447 |
| walker |  | 6142 | 63 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 7, sub: 0, line: 162 } |  |  | 0.447 |
| walker |  | 6206 | 64 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 6, sub: 0, line: 130 } |  |  | 0.447 |
| ns | 6222 |  | 451 | res.send() response finalization | 4.6 | 4.4 | 0.429 |
| walker |  | 6271 | 65 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 4, sub: 0, line: 61 } |  |  | 0.429 |
| walker |  | 6336 | 65 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 5, sub: 0, line: 75 } |  |  | 0.429 |
| walker |  | 6497 | 161 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 18, sub: 0, line: 794 } |  |  | 0.429 |
| walker |  | 6566 | 69 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 8, sub: 0, line: 194 } |  |  | 0.429 |
| ns | 6598 |  | 376 | test/ listing — every spec file | 5.1 |  | 0.491 |
| ns | 6684 |  | 86 | test/acceptance/ listing | 5.2 |  | 0.503 |
| ns | 6785 |  | 101 | examples/ listing | 5.3 |  | 0.519 |
| walker |  | 6920 | 354 | Markdown::Section { file: Readme.md, section_index: 10, keeps_default_concavity: false } |  |  | 0.519 |
| ns | 7041 |  | 256 | examples/README.md — annotated example index (first half) | 5.4 |  | 0.514 |
| walker |  | 7084 | 164 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 13, sub: 0, line: 629 } |  |  | 0.514 |
| walker |  | 7249 | 165 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 2, sub: 0, line: 64 } |  |  | 0.514 |
| ns | 7299 |  | 258 | examples/README.md — annotated example index (second half) | 5.5 | 5.4 | 0.509 |
| walker |  | 7320 | 71 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 2, sub: 0, line: 59 } |  |  | 0.509 |
| ns | 7390 |  | 91 | test/support/ and test/fixtures/ listings | 5.6 |  | 0.502 |
| walker |  | 7391 | 71 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 10, sub: 0, line: 249 } |  |  | 0.502 |
| walker |  | 7463 | 72 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 2, sub: 0, line: 40 } |  |  | 0.502 |
| walker |  | 7535 | 72 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 3, sub: 0, line: 51 } |  |  | 0.502 |
| ns | 7542 |  | 152 | Test harness: env preamble, assertion helpers, template engine | 5.7 |  | 0.499 |
| walker |  | 7608 | 73 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 9, sub: 0, line: 225 } |  |  | 0.499 |
| ns | 7673 |  | 131 | A complete example app: examples/hello-world/index.js | 5.8 |  | 0.493 |
| walker |  | 7790 | 182 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 3, sub: 0, line: 97 } |  |  | 0.493 |
| ns | 7834 |  | 161 | Test-writing idiom: head of test/app.js | 5.9 |  | 0.488 |
| walker |  | 7870 | 80 | Code::CodeKey { rung: Doc, file: lib/request.js, decl: 6, sub: 0, line: 185 } |  |  | 0.488 |
| walker |  | 7951 | 81 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 4, sub: 0, line: 152 } |  |  | 0.488 |
| ns | 8120 |  | 286 | Runtime dependencies (all 28) | 6.1 |  | 0.504 |
| walker |  | 8151 | 200 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 10, sub: 0, line: 503 } |  |  | 0.504 |
| walker |  | 8239 | 88 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 6, sub: 0, line: 256 } |  |  | 0.504 |
| ns | 8346 |  | 226 | ci.yml — jobs and the OS/Node matrix | 6.2 |  | 0.498 |
| walker |  | 8451 | 212 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 9, sub: 0, line: 433 } |  |  | 0.498 |
| ns | 8533 |  | 187 | .eslintrc.yml — the complete lint rule set | 6.3 |  | 0.494 |
| walker |  | 8546 | 95 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 5, sub: 0, line: 190 } |  |  | 0.494 |
| walker |  | 8641 | 95 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 15, sub: 0, line: 494 } |  |  | 0.494 |
| ns | 8807 |  | 274 | Dev dependencies (all 16) | 6.4 |  | 0.488 |
| walker |  | 8870 | 229 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 14, sub: 0, line: 664 } |  |  | 0.488 |
| ns | 8888 |  | 81 | package.json remainder: author, published files | 6.5 |  | 0.487 |
| walker |  | 8908 | 38 | Code::CodeKey { rung: Body, file: lib/request.js, decl: 7, sub: 0, line: 214 } |  |  | 0.487 |
| ns | 9121 |  | 233 | res.sendFile() option bag (JSDoc) | 7.1 |  | 0.483 |
| walker |  | 9183 | 275 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 17, sub: 0, line: 742 } |  |  | 0.484 |
| walker |  | 9302 | 119 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 8, sub: 0, line: 322 } |  |  | 0.484 |
| ns | 9391 |  | 270 | res.cookie() option bag (JSDoc) | 7.2 |  | 0.495 |
| walker |  | 9423 | 121 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 12, sub: 0, line: 439 } |  |  | 0.495 |
| walker |  | 9547 | 124 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 10, sub: 0, line: 399 } |  |  | 0.495 |
| walker |  | 9671 | 124 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 11, sub: 0, line: 420 } |  |  | 0.495 |
| ns | 9673 |  | 282 | res.status() and res.render() contracts (JSDoc) | 7.3 |  | 0.504 |
| walker |  | 9800 | 129 | Code::CodeKey { rung: Doc, file: lib/view.js, decl: 1, sub: 0, line: 52 } |  |  | 0.504 |
| walker |  | 9941 | 141 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 9, sub: 0, line: 351 } |  |  | 0.516 |
| ns | 9971 |  | 298 | History.md — unreleased section and the 5.2.1 heading | 8.1 |  | 0.512 |
| walker |  | 9996 | 55 | Code::CodeKey { rung: Body, file: lib/express.js, decl: 1, sub: 0, line: 36 } |  |  | 0.513 |
