Score(3000)=0.503 I=0.800 C=0.317 ns_rows≤3K=19/51 (reached=7 partial=1 missing=11)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 47 | 47 | listing of '.' |  |  | 1.000 |
| ns | 47 |  | 47 | Repository root listing | 1.1 |  | 1.000 |
| walker |  | 70 | 23 | listing of 'lib' |  |  | 1.000 |
| walker |  | 85 | 15 | export names surface in index.js |  |  | 1.000 |
| ns | 139 |  | 92 | package.json identity | 1.2 |  | 0.827 |
| walker |  | 148 | 63 | package identity in package.json |  |  | 0.908 |
| walker |  | 156 | 8 | listing of '.github' |  |  | 0.908 |
| ns | 162 |  | 23 | lib/ listing — the entire shipped library | 1.3 |  | 0.908 |
| walker |  | 173 | 17 | listing of '.github/workflows' |  |  | 0.918 |
| walker |  | 181 | 8 | imports in index.js |  |  | 0.918 |
| walker |  | 212 | 31 | package runtime metadata in package.json |  |  | 0.919 |
| walker |  | 262 | 50 | package entrypoints in package.json |  |  | 0.919 |
| ns | 299 |  | 137 | Module export surface (index.js + lib/express.js exports) | 1.4 |  | 0.728 |
| walker |  | 387 | 125 | listing of 'examples' |  |  | 0.731 |
| ns | 446 |  | 147 | Readme tagline + canonical quick-start snippet | 1.5 |  | 0.603 |
| ns | 655 |  | 209 | npm scripts + engines | 1.6 |  | 0.539 |
| ns | 681 |  | 26 | .github listing — all four workflows | 1.7 |  | 0.574 |
| ns | 792 |  | 111 | Prototype bases and module exports of each lib file | 2.1 |  | 0.541 |
| walker |  | 807 | 420 | README headline in Readme.md |  |  | 0.686 |
| walker |  | 935 | 128 | headings outline in Readme.md |  |  | 0.686 |
| ns | 965 |  | 173 | app.* roster — every application method name | 2.2 |  | 0.604 |
| walker |  | 1022 | 87 | Readme.md section #2 |  |  | 0.604 |
| ns | 1204 |  | 239 | res.* roster — every response method name | 2.3 |  | 0.528 |
| walker |  | 1400 | 378 | listing of 'test' |  |  | 0.535 |
| walker |  | 1411 | 11 | listing of 'test/support' |  |  | 0.535 |
| ns | 1468 |  | 264 | req.* roster — methods and defineGetter properties | 2.4 |  | 0.485 |
| walker |  | 1587 | 176 | package scripts in package.json |  |  | 0.555 |
| ns | 1666 |  | 198 | lib/view.js and lib/utils.js symbol rosters | 2.5 |  | 0.520 |
| walker |  | 1698 | 111 | export names surface in lib/request.js |  |  | 0.529 |
| walker |  | 1698 | 0 | export at lib/request.js:37 |  |  | 0.529 |
| walker |  | 1698 | 0 | export at lib/request.js:127 |  |  | 0.529 |
| walker |  | 1698 | 0 | export at lib/request.js:140 |  |  | 0.529 |
| walker |  | 1698 | 0 | export at lib/request.js:171 |  |  | 0.529 |
| walker |  | 1698 | 0 | export at lib/request.js:185 |  |  | 0.529 |
| walker |  | 1698 | 0 | export at lib/request.js:214 |  |  | 0.529 |
| walker |  | 1698 | 0 | export at lib/request.js:269 |  |  | 0.529 |
| walker |  | 1710 | 12 | export at lib/request.js:63 |  |  | 0.531 |
| walker |  | 1734 | 24 | export body at lib/request.js:171 body 172 |  |  | 0.531 |
| walker |  | 1759 | 25 | export body at lib/request.js:127 body 128 |  |  | 0.531 |
| walker |  | 1785 | 26 | export body at lib/request.js:140 body 141 |  |  | 0.531 |
| ns | 1899 |  | 233 | createApplication() body | 2.6 |  | 0.492 |
| walker |  | 1918 | 133 | export names surface in lib/express.js |  |  | 0.565 |
| walker |  | 1918 | 0 | export at lib/express.js:27 |  |  | 0.565 |
| walker |  | 1918 | 0 | export at lib/express.js:36 |  |  | 0.565 |
| walker |  | 1918 | 0 | export at lib/express.js:62 |  |  | 0.565 |
| walker |  | 1918 | 0 | export at lib/express.js:70 |  |  | 0.565 |
| walker |  | 1918 | 0 | export at lib/express.js:77 |  |  | 0.565 |
| walker |  | 1935 | 17 | export doc at lib/express.js:70 |  |  | 0.565 |
| walker |  | 1952 | 17 | export doc at lib/express.js:77 |  |  | 0.565 |
| walker |  | 1972 | 20 | export doc at lib/express.js:62 |  |  | 0.565 |
| walker |  | 1994 | 22 | export doc at lib/express.js:27 |  |  | 0.565 |
| walker |  | 2018 | 24 | imports in lib/request.js |  |  | 0.565 |
| walker |  | 2044 | 26 | export doc at lib/request.js:37 |  |  | 0.565 |
| walker |  | 2069 | 25 | imports in lib/express.js |  |  | 0.565 |
| walker |  | 2096 | 27 | imports in lib/application.js |  |  | 0.565 |
| ns | 2156 |  | 257 | HTTP-verb delegation and app.all() | 2.7 |  | 0.523 |
| ns | 2247 |  | 91 | app.* full signature lines | 2.8 | 2.2 | 0.500 |
| walker |  | 2372 | 276 | export names surface in lib/application.js |  |  | 0.572 |
| walker |  | 2372 | 0 | export at lib/application.js:59 |  |  | 0.572 |
| walker |  | 2372 | 0 | export at lib/application.js:90 |  |  | 0.572 |
| walker |  | 2372 | 0 | export at lib/application.js:152 |  |  | 0.572 |
| walker |  | 2372 | 0 | export at lib/application.js:190 |  |  | 0.572 |
| walker |  | 2372 | 0 | export at lib/application.js:256 |  |  | 0.572 |
| walker |  | 2372 | 0 | export at lib/application.js:294 |  |  | 0.572 |
| walker |  | 2372 | 0 | export at lib/application.js:322 |  |  | 0.572 |
| walker |  | 2372 | 0 | export at lib/application.js:351 |  |  | 0.572 |
| walker |  | 2372 | 0 | export at lib/application.js:399 |  |  | 0.572 |
| walker |  | 2372 | 0 | export at lib/application.js:420 |  |  | 0.572 |
| walker |  | 2372 | 0 | export at lib/application.js:439 |  |  | 0.572 |
| walker |  | 2372 | 0 | export at lib/application.js:451 |  |  | 0.572 |
| walker |  | 2372 | 0 | export at lib/application.js:463 |  |  | 0.572 |
| walker |  | 2372 | 0 | export at lib/application.js:494 |  |  | 0.572 |
| walker |  | 2372 | 0 | export at lib/application.js:522 |  |  | 0.572 |
| walker |  | 2372 | 0 | export at lib/application.js:598 |  |  | 0.572 |
| ns | 2372 |  | 125 | res.* full signature lines | 2.9 | 2.3 | 0.572 |
| walker |  | 2389 | 17 | export doc at lib/application.js:90 |  |  | 0.572 |
| walker |  | 2474 | 85 | listing of 'test/acceptance' |  |  | 0.573 |
| ns | 2481 |  | 109 | Default settings established at boot | 3.1 |  | 0.560 |
| walker |  | 2497 | 23 | Readme.md section #19 |  |  | 0.560 |
| walker |  | 2520 | 23 | Readme.md section #30 |  |  | 0.560 |
| walker |  | 2544 | 24 | Readme.md section #12 |  |  | 0.560 |
| walker |  | 2568 | 24 | Readme.md section #32 |  |  | 0.560 |
| walker |  | 2610 | 42 | export doc at lib/express.js:36 |  |  | 0.560 |
| walker |  | 2635 | 25 | Readme.md section #18 |  |  | 0.560 |
| ns | 2654 |  | 173 | Remaining boot configuration: locals, mountpath, view defaults | 3.2 |  | 0.538 |
| walker |  | 2717 | 82 | Readme.md section #7 |  |  | 0.538 |
| walker |  | 2743 | 26 | Readme.md section #23 |  |  | 0.538 |
| walker |  | 2768 | 25 | Readme.md section #24 |  |  | 0.538 |
| walker |  | 2795 | 27 | Readme.md section #13 |  |  | 0.538 |
| walker |  | 2821 | 26 | Readme.md section #14 |  |  | 0.538 |
| walker |  | 2848 | 27 | Readme.md section #15 |  |  | 0.538 |
| walker |  | 2874 | 26 | Readme.md section #16 |  |  | 0.538 |
| walker |  | 2899 | 25 | Readme.md section #17 |  |  | 0.538 |
| walker |  | 2927 | 28 | Readme.md section #22 |  |  | 0.538 |
| ns | 2958 |  | 304 | app.set() — storage plus the three derived-setting side effects | 3.3 |  | 0.503 |
| walker |  | 2968 | 41 | imports in lib/response.js |  |  | 0.503 |
| ns | 3143 |  | 185 | compileETag — accepted values of the `etag` setting | 3.4 |  | 0.483 |
| ns | 3360 |  | 217 | compileQueryParser — accepted values of `query parser` | 3.5 |  | 0.462 |
| walker |  | 3428 | 460 | package runtime dependencies in package.json |  |  | 0.463 |
| walker |  | 3458 | 30 | Readme.md section #27 |  |  | 0.463 |
| walker |  | 3488 | 30 | Readme.md section #29 |  |  | 0.463 |
| walker |  | 3517 | 29 | Readme.md section #28 |  |  | 0.463 |
| ns | 3559 |  | 199 | compileTrust — accepted values of `trust proxy` | 3.6 |  | 0.449 |
| ns | 3691 |  | 132 | req.ip / req.ips — the consumers of `trust proxy fn` | 3.7 | 2.4 | 0.439 |
| walker |  | 3827 | 310 | export names surface in lib/response.js |  |  | 0.517 |
| walker |  | 3827 | 0 | export at lib/response.js:49 |  |  | 0.517 |
| walker |  | 3827 | 0 | export at lib/response.js:64 |  |  | 0.517 |
| walker |  | 3827 | 0 | export at lib/response.js:97 |  |  | 0.517 |
| walker |  | 3827 | 0 | export at lib/response.js:125 |  |  | 0.517 |
| walker |  | 3827 | 0 | export at lib/response.js:232 |  |  | 0.517 |
| walker |  | 3827 | 0 | export at lib/response.js:260 |  |  | 0.517 |
| walker |  | 3827 | 0 | export at lib/response.js:321 |  |  | 0.517 |
| walker |  | 3827 | 0 | export at lib/response.js:371 |  |  | 0.517 |
| walker |  | 3827 | 0 | export at lib/response.js:433 |  |  | 0.517 |
| walker |  | 3827 | 0 | export at lib/response.js:569 |  |  | 0.517 |
| walker |  | 3827 | 0 | export at lib/response.js:604 |  |  | 0.517 |
| walker |  | 3827 | 0 | export at lib/response.js:629 |  |  | 0.517 |
| walker |  | 3827 | 0 | export at lib/response.js:696 |  |  | 0.517 |
| walker |  | 3827 | 0 | export at lib/response.js:709 |  |  | 0.517 |
| walker |  | 3827 | 0 | export at lib/response.js:742 |  |  | 0.517 |
| walker |  | 3827 | 0 | export at lib/response.js:794 |  |  | 0.517 |
| walker |  | 3827 | 0 | export at lib/response.js:812 |  |  | 0.517 |
| walker |  | 3827 | 0 | export at lib/response.js:875 |  |  | 0.517 |
| walker |  | 3827 | 0 | export at lib/response.js:894 |  |  | 0.517 |
| walker |  | 3840 | 13 | export at lib/response.js:503 |  |  | 0.524 |
| walker |  | 3854 | 14 | export at lib/response.js:664 |  |  | 0.531 |
| walker |  | 3880 | 26 | export doc at lib/response.js:49 |  |  | 0.531 |
| ns | 3984 |  | 293 | Settings read-sites scattered outside defaultConfiguration | 3.8 |  | 0.513 |
| walker |  | 4025 | 145 | export body at lib/response.js:232 body 233 |  |  | 0.514 |
| walker |  | 4082 | 57 | export doc at lib/application.js:451 |  |  | 0.514 |
| ns | 4104 |  | 120 | app.init() — per-app state and the lazy base router | 3.9 |  | 0.503 |
| walker |  | 4139 | 57 | export doc at lib/application.js:463 |  |  | 0.503 |
| walker |  | 4196 | 57 | export doc at lib/response.js:696 |  |  | 0.503 |
| ns | 4335 |  | 231 | Sub-app mounting: the 'mount' event and setting inheritance | 3.10 |  | 0.490 |
| ns | 4481 |  | 146 | JSDoc for app.set() | 3.11 |  | 0.481 |
| walker |  | 4534 | 338 | package identity metadata in package.json |  |  | 0.497 |
| walker |  | 4566 | 32 | Readme.md section #11 |  |  | 0.497 |
| walker |  | 4598 | 32 | Readme.md section #21 |  |  | 0.497 |
| walker |  | 4630 | 32 | Readme.md section #31 |  |  | 0.497 |
| ns | 4738 |  | 257 | app.handle() — the per-request dispatch entry point | 4.1 | 2.8 | 0.481 |
| walker |  | 4761 | 131 | Readme.md section #3 |  |  | 0.481 |
| walker |  | 4822 | 61 | export doc at lib/application.js:59 |  |  | 0.481 |
| walker |  | 4855 | 33 | Readme.md section #26 |  |  | 0.481 |
| walker |  | 4887 | 32 | Readme.md section #25 |  |  | 0.481 |
| walker |  | 4991 | 104 | Readme.md section #10 |  |  | 0.481 |
| ns | 5009 |  | 271 | app.use() — argument/path disambiguation | 4.2 | 2.8 | 0.465 |
| walker |  | 5054 | 63 | export doc at lib/request.js:140 |  |  | 0.465 |
| walker |  | 5178 | 124 | Readme.md section #5 |  |  | 0.465 |
| walker |  | 5244 | 66 | export doc at lib/response.js:604 |  |  | 0.465 |
| ns | 5278 |  | 269 | app.use() — the sub-app mounting branch | 4.3 | 4.2 | 0.452 |
| walker |  | 5315 | 71 | export doc at lib/application.js:152 |  |  | 0.452 |
| walker |  | 5387 | 72 | export doc at lib/response.js:709 |  |  | 0.452 |
| walker |  | 5467 | 80 | export doc at lib/request.js:185 |  |  | 0.452 |
| walker |  | 5553 | 86 | export doc at lib/response.js:875 |  |  | 0.452 |
| ns | 5593 |  | 315 | res.send() body-type dispatch | 4.4 | 2.9 | 0.437 |
| walker |  | 5699 | 146 | Readme.md section #6 |  |  | 0.437 |
| walker |  | 5736 | 37 | Readme.md section #8 |  |  | 0.437 |
| ns | 5774 |  | 181 | View.prototype.lookup() — view file resolution | 4.5 | 2.5 | 0.428 |
| walker |  | 5824 | 88 | export doc at lib/application.js:256 |  |  | 0.428 |
| walker |  | 5916 | 92 | export doc at lib/response.js:232 |  |  | 0.428 |
| walker |  | 6011 | 95 | export doc at lib/application.js:190 |  |  | 0.428 |
| walker |  | 6106 | 95 | export doc at lib/application.js:494 |  |  | 0.428 |
| ns | 6225 |  | 451 | res.send() response finalization | 4.6 | 4.4 | 0.410 |
| walker |  | 6348 | 242 | Readme.md section #1 |  |  | 0.410 |
| walker |  | 6447 | 99 | export doc at lib/response.js:260 |  |  | 0.410 |
| walker |  | 6459 | 12 | imports in lib/view.js |  |  | 0.410 |
| walker |  | 6573 | 114 | export doc at lib/application.js:399 |  |  | 0.410 |
| ns | 6603 |  | 378 | test/ listing — every spec file | 5.1 |  | 0.478 |
| walker |  | 6687 | 114 | export doc at lib/response.js:125 |  |  | 0.478 |
| ns | 6688 |  | 85 | test/acceptance/ listing | 5.2 |  | 0.491 |
| walker |  | 6806 | 119 | export doc at lib/application.js:322 |  |  | 0.491 |
| ns | 6813 |  | 125 | examples/ listing | 5.3 |  | 0.508 |
| walker |  | 6927 | 121 | export doc at lib/application.js:439 |  |  | 0.508 |
| walker |  | 7051 | 124 | export doc at lib/application.js:420 |  |  | 0.508 |
| ns | 7069 |  | 256 | examples/README.md — annotated example index (first half) | 5.4 |  | 0.502 |
| walker |  | 7183 | 132 | export doc at lib/response.js:321 |  |  | 0.502 |
| walker |  | 7317 | 134 | export doc at lib/response.js:812 |  |  | 0.502 |
| ns | 7327 |  | 258 | examples/README.md — annotated example index (second half) | 5.5 | 5.4 | 0.497 |
| ns | 7418 |  | 91 | test/support/ and test/fixtures/ listings | 5.6 |  | 0.490 |
| walker |  | 7534 | 217 | export body at lib/express.js:36 body 37 |  |  | 0.515 |
| ns | 7570 |  | 152 | Test harness: env preamble, assertion helpers, template engine | 5.7 |  | 0.511 |
| walker |  | 7637 | 103 | export names surface in lib/view.js |  |  | 0.514 |
| walker |  | 7637 | 0 | export at lib/view.js:36 |  |  | 0.514 |
| walker |  | 7637 | 0 | export at lib/view.js:52 |  |  | 0.514 |
| walker |  | 7637 | 0 | export at lib/view.js:104 |  |  | 0.514 |
| walker |  | 7637 | 0 | export at lib/view.js:133 |  |  | 0.514 |
| walker |  | 7637 | 0 | export at lib/view.js:169 |  |  | 0.514 |
| ns | 7701 |  | 131 | A complete example app: examples/hello-world/index.js | 5.8 |  | 0.509 |
| walker |  | 7778 | 141 | export doc at lib/application.js:351 |  |  | 0.523 |
| ns | 7862 |  | 161 | Test-writing idiom: head of test/app.js | 5.9 |  | 0.517 |
| walker |  | 7920 | 142 | export doc at lib/response.js:894 |  |  | 0.517 |
| ns | 8148 |  | 286 | Runtime dependencies (all 28) | 6.1 |  | 0.532 |
| walker |  | 8165 | 245 | Readme.md section #4 |  |  | 0.532 |
| walker |  | 8324 | 159 | export doc at lib/application.js:522 |  |  | 0.532 |
| ns | 8374 |  | 226 | ci.yml — jobs and the OS/Node matrix | 6.2 |  | 0.526 |
| walker |  | 8485 | 161 | export doc at lib/response.js:794 |  |  | 0.526 |
| ns | 8561 |  | 187 | .eslintrc.yml — the complete lint rule set | 6.3 |  | 0.521 |
| walker |  | 8649 | 164 | export doc at lib/response.js:629 |  |  | 0.521 |
| walker |  | 8814 | 165 | export doc at lib/response.js:64 |  |  | 0.522 |
| ns | 8835 |  | 274 | Dev dependencies (all 16) | 6.4 |  | 0.516 |
| ns | 8916 |  | 81 | package.json remainder: author, published files | 6.5 |  | 0.521 |
| walker |  | 8996 | 182 | export doc at lib/response.js:97 |  |  | 0.521 |
| walker |  | 9148 | 152 | export names surface in lib/utils.js |  |  | 0.531 |
| walker |  | 9148 | 0 | export at lib/utils.js:29 |  |  | 0.531 |
| walker |  | 9148 | 0 | export at lib/utils.js:40 |  |  | 0.531 |
| walker |  | 9148 | 0 | export at lib/utils.js:51 |  |  | 0.531 |
| walker |  | 9148 | 0 | export at lib/utils.js:61 |  |  | 0.531 |
| walker |  | 9148 | 0 | export at lib/utils.js:75 |  |  | 0.531 |
| walker |  | 9148 | 0 | export at lib/utils.js:130 |  |  | 0.531 |
| walker |  | 9148 | 0 | export at lib/utils.js:162 |  |  | 0.531 |
| walker |  | 9148 | 0 | export at lib/utils.js:194 |  |  | 0.531 |
| walker |  | 9148 | 0 | export at lib/utils.js:225 |  |  | 0.531 |
| ns | 9149 |  | 233 | res.sendFile() option bag (JSDoc) | 7.1 |  | 0.526 |
| walker |  | 9160 | 12 | export body at lib/utils.js:75 body 76 |  |  | 0.526 |
| walker |  | 9205 | 45 | export body at lib/utils.js:61 body 62 |  |  | 0.526 |
| walker |  | 9383 | 178 | export body at lib/request.js:63 body 65 |  |  | 0.526 |
| ns | 9419 |  | 270 | res.cookie() option bag (JSDoc) | 7.2 |  | 0.519 |
| walker |  | 9583 | 200 | export doc at lib/response.js:503 |  |  | 0.519 |
| ns | 9701 |  | 282 | res.status() and res.render() contracts (JSDoc) | 7.3 |  | 0.528 |
| walker |  | 9789 | 206 | export doc at lib/request.js:63 |  |  | 0.528 |
| ns | 9999 |  | 298 | History.md — unreleased section and the 5.2.1 heading | 8.1 |  | 0.524 |
