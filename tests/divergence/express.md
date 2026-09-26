Score(3000)=0.620 I=0.849 C=0.453 ns_rows≤3K=19/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.712/0.701/0.588/0.620/0.542/0.429/0.487

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
| walker |  | 607 | 61 | Markdown::CommandBlock { file: Readme.md, row: 163 } |  |  | 0.793 |
| ns | 652 |  | 209 | npm scripts + engines | 1.6 |  | 0.708 |
| ns | 678 |  | 26 | .github listing — all four workflows | 1.7 |  | 0.723 |
| walker |  | 694 | 87 | Markdown::Section { file: Readme.md, section_index: 3, keeps_default_concavity: true } |  |  | 0.723 |
| ns | 789 |  | 111 | Prototype bases and module exports of each lib file | 2.1 |  | 0.682 |
| walker |  | 795 | 101 | Fs::DirListing { dir: examples } |  |  | 0.685 |
| walker |  | 928 | 133 | Code::CodeKey { rung: Names, file: lib/express.js, decl: 0, sub: 0, line: 0 } |  |  | 0.787 |
| ns | 962 |  | 173 | app.* roster — every application method name | 2.2 |  | 0.693 |
| walker |  | 1104 | 176 | Json::Scripts { file: package.json } |  |  | 0.776 |
| walker |  | 1119 | 15 | Code::CodeKey { rung: Names, file: index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.796 |
| ns | 1201 |  | 239 | res.* roster — every response method name | 2.3 |  | 0.695 |
| ns | 1465 |  | 264 | req.* roster — methods and defineGetter properties | 2.4 |  | 0.631 |
| walker |  | 1495 | 376 | Fs::DirListing { dir: test } |  |  | 0.639 |
| walker |  | 1507 | 12 | Fs::DirListing { dir: test/support } |  |  | 0.639 |
| walker |  | 1593 | 86 | Fs::DirListing { dir: test/acceptance } |  |  | 0.640 |
| ns | 1663 |  | 198 | lib/view.js and lib/utils.js symbol rosters | 2.5 |  | 0.600 |
| walker |  | 1724 | 131 | Markdown::Section { file: Readme.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.600 |
| walker |  | 1848 | 124 | Markdown::Section { file: Readme.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.600 |
| ns | 1896 |  | 233 | createApplication() body | 2.6 |  | 0.557 |
| ns | 2153 |  | 257 | HTTP-verb delegation and app.all() | 2.7 |  | 0.516 |
| walker |  | 2176 | 328 | Code::CodeKey { rung: Names, file: lib/response.js, decl: 0, sub: 0, line: 0 } |  |  | 0.578 |
| walker |  | 2189 | 13 | Code::CodeKey { rung: Decl, file: lib/response.js, decl: 10, sub: 0, line: 503 } |  |  | 0.583 |
| walker |  | 2203 | 14 | Code::CodeKey { rung: Decl, file: lib/response.js, decl: 14, sub: 0, line: 664 } |  |  | 0.589 |
| walker |  | 2229 | 26 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 1, sub: 0, line: 42 } |  |  | 0.589 |
| walker |  | 2240 | 11 | Code::CodeKey { rung: Body, file: lib/response.js, decl: 15, sub: 0, line: 696 } |  |  | 0.589 |
| ns | 2244 |  | 91 | app.* full signature lines | 2.8 | 2.2 | 0.564 |
| ns | 2369 |  | 125 | res.* full signature lines | 2.9 | 2.3 | 0.600 |
| ns | 2478 |  | 109 | Default settings established at boot | 3.1 |  | 0.586 |
| walker |  | 2493 | 253 | Code::CodeKey { rung: Names, file: lib/application.js, decl: 0, sub: 0, line: 0 } |  |  | 0.675 |
| walker |  | 2621 | 128 | Code::CodeKey { rung: Names, file: lib/request.js, decl: 0, sub: 0, line: 0 } |  |  | 0.687 |
| walker |  | 2633 | 12 | Code::CodeKey { rung: Decl, file: lib/request.js, decl: 2, sub: 0, line: 63 } |  |  | 0.688 |
| walker |  | 2648 | 15 | Code::CodeKey { rung: Body, file: lib/response.js, decl: 18, sub: 0, line: 794 } |  |  | 0.688 |
| ns | 2651 |  | 173 | Remaining boot configuration: locals, mountpath, view defaults | 3.2 |  | 0.661 |
| ns | 2955 |  | 304 | app.set() — storage plus the three derived-setting side effects | 3.3 |  | 0.619 |
| walker |  | 3108 | 460 | Json::Dependencies { file: package.json } |  |  | 0.621 |
| ns | 3140 |  | 185 | compileETag — accepted values of the `etag` setting | 3.4 |  | 0.596 |
| walker |  | 3150 | 42 | Code::CodeKey { rung: Doc, file: lib/express.js, decl: 1, sub: 0, line: 36 } |  |  | 0.596 |
| walker |  | 3168 | 18 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 1, sub: 0, line: 40 } |  |  | 0.596 |
| walker |  | 3335 | 167 | Code::CodeKey { rung: Names, file: lib/utils.js, decl: 0, sub: 0, line: 0 } |  |  | 0.611 |
| ns | 3357 |  | 217 | compileQueryParser — accepted values of `query parser` | 3.5 |  | 0.584 |
| walker |  | 3410 | 75 | Code::CodeKey { rung: Names, file: lib/view.js, decl: 0, sub: 0, line: 0 } |  |  | 0.605 |
| walker |  | 3422 | 12 | Code::CodeKey { rung: Body, file: lib/utils.js, decl: 5, sub: 0, line: 75 } |  |  | 0.605 |
| walker |  | 3435 | 13 | Code::CodeKey { rung: Body, file: lib/request.js, decl: 6, sub: 0, line: 185 } |  |  | 0.605 |
| ns | 3556 |  | 199 | compileTrust — accepted values of `trust proxy` | 3.6 |  | 0.586 |
| walker |  | 3581 | 146 | Markdown::Section { file: Readme.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.586 |
| walker |  | 3592 | 11 | Code::CodeKey { rung: Body, file: lib/application.js, decl: 6, sub: 0, line: 256 } |  |  | 0.586 |
| ns | 3688 |  | 132 | req.ip / req.ips — the consumers of `trust proxy fn` | 3.7 | 2.4 | 0.573 |
| walker |  | 3834 | 242 | Markdown::Section { file: Readme.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.573 |
| walker |  | 3845 | 11 | Code::CodeKey { rung: Body, file: lib/application.js, decl: 11, sub: 0, line: 420 } |  |  | 0.573 |
| walker |  | 3856 | 11 | Code::CodeKey { rung: Body, file: lib/application.js, decl: 12, sub: 0, line: 439 } |  |  | 0.573 |
| walker |  | 3913 | 57 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 15, sub: 0, line: 696 } |  |  | 0.573 |
| walker |  | 3925 | 12 | Code::CodeKey { rung: Body, file: lib/application.js, decl: 13, sub: 0, line: 451 } |  |  | 0.573 |
| walker |  | 3951 | 26 | Code::CodeKey { rung: Doc, file: lib/request.js, decl: 1, sub: 0, line: 30 } |  |  | 0.573 |
| walker |  | 3963 | 12 | Code::CodeKey { rung: Body, file: lib/application.js, decl: 14, sub: 0, line: 463 } |  |  | 0.573 |
| ns | 3981 |  | 293 | Settings read-sites scattered outside defaultConfiguration | 3.8 |  | 0.553 |
| walker |  | 3990 | 27 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 3, sub: 0, line: 90 } |  |  | 0.553 |
| walker |  | 4056 | 66 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 12, sub: 0, line: 604 } |  |  | 0.553 |
| ns | 4101 |  | 120 | app.init() — per-app state and the lazy base router | 3.9 |  | 0.542 |
| walker |  | 4128 | 72 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 16, sub: 0, line: 709 } |  |  | 0.542 |
| ns | 4332 |  | 231 | Sub-app mounting: the 'mount' event and setting inheritance | 3.10 |  | 0.528 |
| walker |  | 4373 | 245 | Markdown::Section { file: Readme.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.528 |
| walker |  | 4459 | 86 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 20, sub: 0, line: 875 } |  |  | 0.528 |
| ns | 4478 |  | 146 | JSDoc for app.set() | 3.11 |  | 0.518 |
| walker |  | 4483 | 24 | Code::CodeKey { rung: Body, file: lib/request.js, decl: 5, sub: 0, line: 171 } |  |  | 0.518 |
| walker |  | 4522 | 39 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 1, sub: 0, line: 29 } |  |  | 0.518 |
| walker |  | 4614 | 92 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 5, sub: 0, line: 232 } |  |  | 0.518 |
| walker |  | 4713 | 99 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 6, sub: 0, line: 260 } |  |  | 0.518 |
| ns | 4735 |  | 257 | app.handle() — the per-request dispatch entry point | 4.1 | 2.8 | 0.501 |
| walker |  | 4738 | 25 | Code::CodeKey { rung: Body, file: lib/request.js, decl: 3, sub: 0, line: 127 } |  |  | 0.501 |
| walker |  | 4764 | 26 | Code::CodeKey { rung: Body, file: lib/request.js, decl: 4, sub: 0, line: 140 } |  |  | 0.501 |
| walker |  | 4812 | 48 | Code::CodeKey { rung: Doc, file: lib/view.js, decl: 2, sub: 0, line: 104 } |  |  | 0.501 |
| walker |  | 4926 | 114 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 4, sub: 0, line: 125 } |  |  | 0.501 |
| ns | 5006 |  | 271 | app.use() — argument/path disambiguation | 4.2 | 2.8 | 0.485 |
| walker |  | 5058 | 132 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 7, sub: 0, line: 321 } |  |  | 0.485 |
| walker |  | 5192 | 134 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 19, sub: 0, line: 812 } |  |  | 0.485 |
| walker |  | 5249 | 57 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 13, sub: 0, line: 451 } |  |  | 0.485 |
| ns | 5275 |  | 269 | app.use() — the sub-app mounting branch | 4.3 | 4.2 | 0.471 |
| walker |  | 5306 | 57 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 14, sub: 0, line: 463 } |  |  | 0.471 |
| walker |  | 5364 | 58 | Code::CodeKey { rung: Doc, file: lib/view.js, decl: 3, sub: 0, line: 133 } |  |  | 0.471 |
| walker |  | 5424 | 60 | Code::CodeKey { rung: Doc, file: lib/view.js, decl: 4, sub: 0, line: 169 } |  |  | 0.471 |
| walker |  | 5566 | 142 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 21, sub: 0, line: 894 } |  |  | 0.472 |
| ns | 5590 |  | 315 | res.send() body-type dispatch | 4.4 | 2.9 | 0.456 |
| ns | 5771 |  | 181 | View.prototype.lookup() — view file resolution | 4.5 | 2.5 | 0.447 |
| walker |  | 5894 | 328 | Markdown::Section { file: Readme.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.447 |
| walker |  | 6077 | 183 | Markdown::Section { file: Readme.md, section_index: 10, keeps_default_concavity: false } |  |  | 0.447 |
| walker |  | 6140 | 63 | Code::CodeKey { rung: Doc, file: lib/request.js, decl: 4, sub: 0, line: 140 } |  |  | 0.447 |
| walker |  | 6203 | 63 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 7, sub: 0, line: 162 } |  |  | 0.447 |
| ns | 6222 |  | 451 | res.send() response finalization | 4.6 | 4.4 | 0.429 |
| walker |  | 6267 | 64 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 6, sub: 0, line: 130 } |  |  | 0.429 |
| walker |  | 6332 | 65 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 4, sub: 0, line: 61 } |  |  | 0.429 |
| walker |  | 6397 | 65 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 5, sub: 0, line: 75 } |  |  | 0.429 |
| walker |  | 6558 | 161 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 18, sub: 0, line: 794 } |  |  | 0.429 |
| ns | 6598 |  | 376 | test/ listing — every spec file | 5.1 |  | 0.491 |
| walker |  | 6627 | 69 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 8, sub: 0, line: 194 } |  |  | 0.491 |
| ns | 6684 |  | 86 | test/acceptance/ listing | 5.2 |  | 0.503 |
| ns | 6785 |  | 101 | examples/ listing | 5.3 |  | 0.519 |
| walker |  | 6981 | 354 | Markdown::Section { file: Readme.md, section_index: 11, keeps_default_concavity: false } |  |  | 0.519 |
| ns | 7041 |  | 256 | examples/README.md — annotated example index (first half) | 5.4 |  | 0.514 |
| walker |  | 7145 | 164 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 13, sub: 0, line: 629 } |  |  | 0.514 |
| ns | 7299 |  | 258 | examples/README.md — annotated example index (second half) | 5.5 | 5.4 | 0.509 |
| walker |  | 7310 | 165 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 2, sub: 0, line: 64 } |  |  | 0.509 |
| walker |  | 7381 | 71 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 2, sub: 0, line: 59 } |  |  | 0.509 |
| ns | 7390 |  | 91 | test/support/ and test/fixtures/ listings | 5.6 |  | 0.502 |
| walker |  | 7452 | 71 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 10, sub: 0, line: 249 } |  |  | 0.502 |
| walker |  | 7524 | 72 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 2, sub: 0, line: 40 } |  |  | 0.502 |
| ns | 7542 |  | 152 | Test harness: env preamble, assertion helpers, template engine | 5.7 |  | 0.499 |
| walker |  | 7596 | 72 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 3, sub: 0, line: 51 } |  |  | 0.499 |
| walker |  | 7669 | 73 | Code::CodeKey { rung: Doc, file: lib/utils.js, decl: 9, sub: 0, line: 225 } |  |  | 0.499 |
| ns | 7673 |  | 131 | A complete example app: examples/hello-world/index.js | 5.8 |  | 0.493 |
| ns | 7834 |  | 161 | Test-writing idiom: head of test/app.js | 5.9 |  | 0.488 |
| walker |  | 7851 | 182 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 3, sub: 0, line: 97 } |  |  | 0.488 |
| walker |  | 7931 | 80 | Code::CodeKey { rung: Doc, file: lib/request.js, decl: 6, sub: 0, line: 185 } |  |  | 0.488 |
| walker |  | 8012 | 81 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 4, sub: 0, line: 152 } |  |  | 0.488 |
| ns | 8120 |  | 286 | Runtime dependencies (all 28) | 6.1 |  | 0.504 |
| walker |  | 8212 | 200 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 10, sub: 0, line: 503 } |  |  | 0.504 |
| walker |  | 8300 | 88 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 6, sub: 0, line: 256 } |  |  | 0.504 |
| ns | 8346 |  | 226 | ci.yml — jobs and the OS/Node matrix | 6.2 |  | 0.498 |
| walker |  | 8512 | 212 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 9, sub: 0, line: 433 } |  |  | 0.498 |
| ns | 8533 |  | 187 | .eslintrc.yml — the complete lint rule set | 6.3 |  | 0.494 |
| walker |  | 8607 | 95 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 5, sub: 0, line: 190 } |  |  | 0.494 |
| walker |  | 8702 | 95 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 15, sub: 0, line: 494 } |  |  | 0.494 |
| ns | 8807 |  | 274 | Dev dependencies (all 16) | 6.4 |  | 0.488 |
| ns | 8888 |  | 81 | package.json remainder: author, published files | 6.5 |  | 0.487 |
| walker |  | 8931 | 229 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 14, sub: 0, line: 664 } |  |  | 0.487 |
| walker |  | 8969 | 38 | Code::CodeKey { rung: Body, file: lib/request.js, decl: 7, sub: 0, line: 214 } |  |  | 0.487 |
| ns | 9121 |  | 233 | res.sendFile() option bag (JSDoc) | 7.1 |  | 0.483 |
| walker |  | 9244 | 275 | Code::CodeKey { rung: Doc, file: lib/response.js, decl: 17, sub: 0, line: 742 } |  |  | 0.484 |
| walker |  | 9363 | 119 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 8, sub: 0, line: 322 } |  |  | 0.484 |
| ns | 9391 |  | 270 | res.cookie() option bag (JSDoc) | 7.2 |  | 0.495 |
| walker |  | 9484 | 121 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 12, sub: 0, line: 439 } |  |  | 0.495 |
| walker |  | 9608 | 124 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 10, sub: 0, line: 399 } |  |  | 0.495 |
| ns | 9673 |  | 282 | res.status() and res.render() contracts (JSDoc) | 7.3 |  | 0.504 |
| walker |  | 9732 | 124 | Code::CodeKey { rung: Doc, file: lib/application.js, decl: 11, sub: 0, line: 420 } |  |  | 0.504 |
| walker |  | 9861 | 129 | Code::CodeKey { rung: Doc, file: lib/view.js, decl: 1, sub: 0, line: 52 } |  |  | 0.504 |
| ns | 9971 |  | 298 | History.md — unreleased section and the 5.2.1 heading | 8.1 |  | 0.500 |
