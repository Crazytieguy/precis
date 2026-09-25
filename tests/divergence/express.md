Score(3000)=0.624 I=0.848 C=0.459 ns_rows≤3K=19/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.603/0.631/0.622/0.624/0.516/0.425/0.507

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 43 | 43 | listing of '.' |  |  | 1.000 |
| ns | 43 |  | 43 | Repository root listing | 1.1 |  | 1.000 |
| walker |  | 58 | 15 | ts names index.js |  |  | 1.000 |
| walker |  | 82 | 24 | listing of 'lib' |  |  | 1.000 |
| ns | 135 |  | 92 | package.json identity | 1.2 |  | 0.827 |
| walker |  | 145 | 63 | package identity in package.json |  |  | 0.908 |
| walker |  | 153 | 8 | listing of '.github' |  |  | 0.908 |
| ns | 159 |  | 24 | lib/ listing — the entire shipped library | 1.3 |  | 0.908 |
| walker |  | 171 | 18 | listing of '.github/workflows' |  |  | 0.918 |
| walker |  | 202 | 31 | package runtime metadata in package.json |  |  | 0.919 |
| walker |  | 252 | 50 | package entrypoints in package.json |  |  | 0.919 |
| ns | 296 |  | 137 | Module export surface (index.js + lib/express.js exports) | 1.4 |  | 0.728 |
| walker |  | 327 | 75 | ts names lib/view.js |  |  | 0.729 |
| walker |  | 428 | 101 | listing of 'examples' |  |  | 0.732 |
| ns | 443 |  | 147 | Readme tagline + canonical quick-start snippet | 1.5 |  | 0.604 |
| walker |  | 547 | 119 | ts names lib/request.js |  |  | 0.606 |
| walker |  | 560 | 13 | ts body lib/request.js:185 |  |  | 0.606 |
| ns | 652 |  | 209 | npm scripts + engines | 1.6 |  | 0.542 |
| ns | 678 |  | 26 | .github listing — all four workflows | 1.7 |  | 0.576 |
| walker |  | 693 | 133 | ts names lib/express.js |  |  | 0.717 |
| ns | 789 |  | 111 | Prototype bases and module exports of each lib file | 2.1 |  | 0.685 |
| ns | 962 |  | 173 | app.* roster — every application method name | 2.2 |  | 0.603 |
| walker |  | 1113 | 420 | README headline in Readme.md |  |  | 0.723 |
| ns | 1201 |  | 239 | res.* roster — every response method name | 2.3 |  | 0.631 |
| walker |  | 1241 | 128 | headings outline in Readme.md |  |  | 0.631 |
| walker |  | 1328 | 87 | Readme.md section #3 |  |  | 0.631 |
| walker |  | 1352 | 24 | ts body lib/request.js:171 |  |  | 0.631 |
| walker |  | 1378 | 26 | ts doc lib/request.js:30 |  |  | 0.631 |
| walker |  | 1403 | 25 | ts body lib/request.js:127 |  |  | 0.631 |
| ns | 1465 |  | 264 | req.* roster — methods and defineGetter properties | 2.4 |  | 0.579 |
| walker |  | 1656 | 253 | ts names lib/application.js |  |  | 0.653 |
| ns | 1663 |  | 198 | lib/view.js and lib/utils.js symbol rosters | 2.5 |  | 0.620 |
| walker |  | 1667 | 11 | ts body lib/application.js:256 |  |  | 0.620 |
| walker |  | 1678 | 11 | ts body lib/application.js:420 |  |  | 0.620 |
| walker |  | 1689 | 11 | ts body lib/application.js:439 |  |  | 0.620 |
| walker |  | 1701 | 12 | ts body lib/application.js:451 |  |  | 0.620 |
| walker |  | 1713 | 12 | ts body lib/application.js:463 |  |  | 0.620 |
| walker |  | 1880 | 167 | ts names lib/utils.js |  |  | 0.670 |
| walker |  | 1892 | 12 | ts body lib/utils.js:75 |  |  | 0.670 |
| ns | 1896 |  | 233 | createApplication() body | 2.6 |  | 0.622 |
| walker |  | 1910 | 18 | ts doc lib/application.js:40 |  |  | 0.622 |
| walker |  | 1937 | 27 | ts doc lib/application.js:90 |  |  | 0.622 |
| walker |  | 1963 | 26 | ts body lib/request.js:140 |  |  | 0.622 |
| ns | 2153 |  | 257 | HTTP-verb delegation and app.all() | 2.7 |  | 0.575 |
| ns | 2244 |  | 91 | app.* full signature lines | 2.8 | 2.2 | 0.602 |
| walker |  | 2272 | 309 | ts names lib/response.js |  |  | 0.656 |
| walker |  | 2283 | 11 | ts body lib/response.js:696 |  |  | 0.656 |
| walker |  | 2298 | 15 | ts body lib/response.js:794 |  |  | 0.656 |
| walker |  | 2321 | 23 | ts body lib/response.js:875 |  |  | 0.656 |
| walker |  | 2347 | 26 | ts doc lib/response.js:42 |  |  | 0.656 |
| ns | 2369 |  | 125 | res.* full signature lines | 2.9 | 2.3 | 0.663 |
| ns | 2478 |  | 109 | Default settings established at boot | 3.1 |  | 0.647 |
| walker |  | 2523 | 176 | package scripts in package.json |  |  | 0.687 |
| ns | 2651 |  | 173 | Remaining boot configuration: locals, mountpath, view defaults | 3.2 |  | 0.659 |
| walker |  | 2899 | 376 | listing of 'test' |  |  | 0.666 |
| walker |  | 2911 | 12 | listing of 'test/support' |  |  | 0.666 |
| walker |  | 2942 | 31 | ts body lib/application.js:399 |  |  | 0.666 |
| ns | 2955 |  | 304 | app.set() — storage plus the three derived-setting side effects | 3.3 |  | 0.624 |
| walker |  | 2981 | 39 | ts doc lib/utils.js:29 |  |  | 0.624 |
| ns | 3140 |  | 185 | compileETag — accepted values of the `etag` setting | 3.4 |  | 0.599 |
| ns | 3357 |  | 217 | compileQueryParser — accepted values of `query parser` | 3.5 |  | 0.572 |
| walker |  | 3441 | 460 | package runtime dependencies in package.json |  |  | 0.574 |
| walker |  | 3479 | 38 | ts body lib/request.js:214 |  |  | 0.574 |
| walker |  | 3521 | 42 | ts doc lib/express.js:36 |  |  | 0.574 |
| ns | 3556 |  | 199 | compileTrust — accepted values of `trust proxy` | 3.6 |  | 0.556 |
| walker |  | 3566 | 45 | ts body lib/utils.js:61 |  |  | 0.556 |
| walker |  | 3614 | 48 | ts doc lib/view.js:104 |  |  | 0.556 |
| walker |  | 3671 | 57 | ts doc lib/application.js:451 |  |  | 0.556 |
| ns | 3688 |  | 132 | req.ip / req.ips — the consumers of `trust proxy fn` | 3.7 | 2.4 | 0.544 |
| walker |  | 3728 | 57 | ts doc lib/application.js:463 |  |  | 0.544 |
| walker |  | 3785 | 57 | ts doc lib/response.js:696 |  |  | 0.544 |
| walker |  | 3843 | 58 | ts doc lib/view.js:133 |  |  | 0.544 |
| walker |  | 3903 | 60 | ts doc lib/view.js:169 |  |  | 0.544 |
| walker |  | 3962 | 59 | ts body lib/response.js:321 |  |  | 0.544 |
| ns | 3981 |  | 293 | Settings read-sites scattered outside defaultConfiguration | 3.8 |  | 0.525 |
| walker |  | 4022 | 60 | ts body lib/response.js:604 |  |  | 0.525 |
| walker |  | 4085 | 63 | ts doc lib/request.js:140 |  |  | 0.525 |
| ns | 4101 |  | 120 | app.init() — per-app state and the lazy base router | 3.9 |  | 0.514 |
| walker |  | 4148 | 63 | ts doc lib/utils.js:162 |  |  | 0.514 |
| walker |  | 4212 | 64 | ts doc lib/utils.js:130 |  |  | 0.514 |
| walker |  | 4298 | 86 | listing of 'test/acceptance' |  |  | 0.516 |
| ns | 4332 |  | 231 | Sub-app mounting: the 'mount' event and setting inheritance | 3.10 |  | 0.503 |
| walker |  | 4363 | 65 | ts doc lib/utils.js:61 |  |  | 0.503 |
| walker |  | 4428 | 65 | ts doc lib/utils.js:75 |  |  | 0.503 |
| ns | 4478 |  | 146 | JSDoc for app.set() | 3.11 |  | 0.493 |
| walker |  | 4494 | 66 | ts doc lib/response.js:604 |  |  | 0.493 |
| walker |  | 4563 | 69 | ts doc lib/utils.js:194 |  |  | 0.493 |
| walker |  | 4634 | 71 | ts doc lib/application.js:59 |  |  | 0.493 |
| walker |  | 4705 | 71 | ts doc lib/utils.js:249 |  |  | 0.493 |
| ns | 4735 |  | 257 | app.handle() — the per-request dispatch entry point | 4.1 | 2.8 | 0.477 |
| walker |  | 4777 | 72 | ts doc lib/response.js:709 |  |  | 0.477 |
| walker |  | 4849 | 72 | ts doc lib/utils.js:40 |  |  | 0.477 |
| walker |  | 4921 | 72 | ts doc lib/utils.js:51 |  |  | 0.477 |
| walker |  | 4994 | 73 | ts doc lib/utils.js:225 |  |  | 0.477 |
| ns | 5006 |  | 271 | app.use() — argument/path disambiguation | 4.2 | 2.8 | 0.461 |
| walker |  | 5067 | 73 | ts body lib/utils.js:249 |  |  | 0.461 |
| ns | 5275 |  | 269 | app.use() — the sub-app mounting branch | 4.3 | 4.2 | 0.448 |
| walker |  | 5405 | 338 | package identity metadata in package.json |  |  | 0.463 |
| walker |  | 5485 | 80 | ts doc lib/request.js:185 |  |  | 0.463 |
| walker |  | 5566 | 81 | ts doc lib/application.js:152 |  |  | 0.463 |
| ns | 5590 |  | 315 | res.send() body-type dispatch | 4.4 | 2.9 | 0.447 |
| walker |  | 5645 | 79 | ts body lib/response.js:709 |  |  | 0.447 |
| walker |  | 5731 | 86 | ts doc lib/response.js:875 |  |  | 0.447 |
| ns | 5771 |  | 181 | View.prototype.lookup() — view file resolution | 4.5 | 2.5 | 0.439 |
| walker |  | 5819 | 88 | ts doc lib/application.js:256 |  |  | 0.439 |
| walker |  | 5906 | 87 | ts body lib/application.js:494 |  |  | 0.443 |
| walker |  | 5998 | 92 | ts doc lib/response.js:232 |  |  | 0.443 |
| walker |  | 6093 | 95 | ts doc lib/application.js:190 |  |  | 0.443 |
| walker |  | 6188 | 95 | ts doc lib/application.js:494 |  |  | 0.443 |
| ns | 6222 |  | 451 | res.send() response finalization | 4.6 | 4.4 | 0.425 |
| walker |  | 6287 | 99 | ts doc lib/response.js:260 |  |  | 0.425 |
| walker |  | 6418 | 131 | Readme.md section #4 |  |  | 0.425 |
| walker |  | 6519 | 101 | ts body lib/application.js:322 |  |  | 0.425 |
| ns | 6598 |  | 376 | test/ listing — every spec file | 5.1 |  | 0.490 |
| walker |  | 6622 | 103 | ts body lib/application.js:598 |  |  | 0.490 |
| ns | 6684 |  | 86 | test/acceptance/ listing | 5.2 |  | 0.503 |
| walker |  | 6726 | 104 | ts body lib/utils.js:225 |  |  | 0.503 |
| ns | 6785 |  | 101 | examples/ listing | 5.3 |  | 0.520 |
| walker |  | 6840 | 114 | ts doc lib/response.js:125 |  |  | 0.520 |
| walker |  | 6964 | 124 | Readme.md section #6 |  |  | 0.520 |
| ns | 7041 |  | 256 | examples/README.md — annotated example index (first half) | 5.4 |  | 0.514 |
| walker |  | 7076 | 112 | ts body lib/request.js:269 |  |  | 0.514 |
| walker |  | 7195 | 119 | ts doc lib/application.js:322 |  |  | 0.514 |
| ns | 7299 |  | 258 | examples/README.md — annotated example index (second half) | 5.5 | 5.4 | 0.509 |
| walker |  | 7316 | 121 | ts doc lib/application.js:439 |  |  | 0.509 |
| ns | 7390 |  | 91 | test/support/ and test/fixtures/ listings | 5.6 |  | 0.502 |
| walker |  | 7440 | 124 | ts doc lib/application.js:399 |  |  | 0.502 |
| ns | 7542 |  | 152 | Test harness: env preamble, assertion helpers, template engine | 5.7 |  | 0.498 |
| walker |  | 7564 | 124 | ts doc lib/application.js:420 |  |  | 0.498 |
| ns | 7673 |  | 131 | A complete example app: examples/hello-world/index.js | 5.8 |  | 0.493 |
| walker |  | 7683 | 119 | ts body lib/response.js:629 |  |  | 0.493 |
| walker |  | 7805 | 122 | ts body lib/application.js:294 |  |  | 0.493 |
| ns | 7834 |  | 161 | Test-writing idiom: head of test/app.js | 5.9 |  | 0.488 |
| walker |  | 7934 | 129 | ts doc lib/view.js:52 |  |  | 0.488 |
| walker |  | 8066 | 132 | ts doc lib/response.js:321 |  |  | 0.488 |
| ns | 8120 |  | 286 | Runtime dependencies (all 28) | 6.1 |  | 0.504 |
| walker |  | 8200 | 134 | ts doc lib/response.js:812 |  |  | 0.504 |
| walker |  | 8341 | 141 | ts doc lib/application.js:351 |  |  | 0.518 |
| ns | 8346 |  | 226 | ci.yml — jobs and the OS/Node matrix | 6.2 |  | 0.511 |
| walker |  | 8483 | 142 | ts doc lib/response.js:894 |  |  | 0.511 |
| ns | 8533 |  | 187 | .eslintrc.yml — the complete lint rule set | 6.3 |  | 0.507 |
| walker |  | 8629 | 146 | Readme.md section #7 |  |  | 0.507 |
| walker |  | 8774 | 145 | ts body lib/response.js:232 |  |  | 0.507 |
| ns | 8807 |  | 274 | Dev dependencies (all 16) | 6.4 |  | 0.502 |
| ns | 8888 |  | 81 | package.json remainder: author, published files | 6.5 |  | 0.507 |
| walker |  | 8933 | 159 | ts doc lib/application.js:522 |  |  | 0.507 |
| walker |  | 9094 | 161 | ts doc lib/response.js:794 |  |  | 0.507 |
| ns | 9121 |  | 233 | res.sendFile() option bag (JSDoc) | 7.1 |  | 0.502 |
| walker |  | 9258 | 164 | ts doc lib/response.js:629 |  |  | 0.502 |
| ns | 9391 |  | 270 | res.cookie() option bag (JSDoc) | 7.2 |  | 0.496 |
| walker |  | 9423 | 165 | ts doc lib/response.js:64 |  |  | 0.497 |
| walker |  | 9665 | 242 | Readme.md section #2 |  |  | 0.497 |
| ns | 9673 |  | 282 | res.status() and res.render() contracts (JSDoc) | 7.3 |  | 0.506 |
| walker |  | 9829 | 164 | ts body lib/response.js:64 |  |  | 0.506 |
| ns | 9971 |  | 298 | History.md — unreleased section and the 5.2.1 heading | 8.1 |  | 0.502 |
| walker |  | 9998 | 169 | ts body lib/view.js:169 |  |  | 0.502 |
