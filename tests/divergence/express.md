Score(3000)=0.553 I=0.821 C=0.373 ns_rows≤3K=19/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.604/0.535/0.640/0.553/0.544/0.443/0.543

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 43 | 43 | listing of '.' |  |  | 1.000 |
| ns | 43 |  | 43 | Repository root listing | 1.1 |  | 1.000 |
| walker |  | 67 | 24 | listing of 'lib' |  |  | 1.000 |
| walker |  | 82 | 15 | export names surface in index.js |  |  | 1.000 |
| ns | 135 |  | 92 | package.json identity | 1.2 |  | 0.827 |
| walker |  | 145 | 63 | package identity in package.json |  |  | 0.908 |
| walker |  | 153 | 8 | listing of '.github' |  |  | 0.908 |
| ns | 159 |  | 24 | lib/ listing — the entire shipped library | 1.3 |  | 0.908 |
| walker |  | 171 | 18 | listing of '.github/workflows' |  |  | 0.918 |
| walker |  | 179 | 8 | imports in index.js |  |  | 0.918 |
| walker |  | 210 | 31 | package runtime metadata in package.json |  |  | 0.919 |
| walker |  | 260 | 50 | package entrypoints in package.json |  |  | 0.919 |
| ns | 296 |  | 137 | Module export surface (index.js + lib/express.js exports) | 1.4 |  | 0.728 |
| walker |  | 361 | 101 | listing of 'examples' |  |  | 0.731 |
| ns | 443 |  | 147 | Readme tagline + canonical quick-start snippet | 1.5 |  | 0.603 |
| ns | 652 |  | 209 | npm scripts + engines | 1.6 |  | 0.539 |
| ns | 678 |  | 26 | .github listing — all four workflows | 1.7 |  | 0.574 |
| walker |  | 781 | 420 | README headline in Readme.md |  |  | 0.727 |
| ns | 789 |  | 111 | Prototype bases and module exports of each lib file | 2.1 |  | 0.686 |
| walker |  | 909 | 128 | headings outline in Readme.md |  |  | 0.686 |
| ns | 962 |  | 173 | app.* roster — every application method name | 2.2 |  | 0.604 |
| walker |  | 996 | 87 | Readme.md section #2 |  |  | 0.604 |
| ns | 1201 |  | 239 | res.* roster — every response method name | 2.3 |  | 0.528 |
| walker |  | 1372 | 376 | listing of 'test' |  |  | 0.535 |
| walker |  | 1384 | 12 | listing of 'test/support' |  |  | 0.535 |
| ns | 1465 |  | 264 | req.* roster — methods and defineGetter properties | 2.4 |  | 0.485 |
| walker |  | 1560 | 176 | package scripts in package.json |  |  | 0.555 |
| ns | 1663 |  | 198 | lib/view.js and lib/utils.js symbol rosters | 2.5 |  | 0.520 |
| walker |  | 1671 | 111 | export names surface in lib/request.js |  |  | 0.529 |
| walker |  | 1671 | 0 | export at lib/request.js:37 |  |  | 0.529 |
| walker |  | 1671 | 0 | export at lib/request.js:127 |  |  | 0.529 |
| walker |  | 1671 | 0 | export at lib/request.js:140 |  |  | 0.529 |
| walker |  | 1671 | 0 | export at lib/request.js:171 |  |  | 0.529 |
| walker |  | 1671 | 0 | export at lib/request.js:185 |  |  | 0.529 |
| walker |  | 1671 | 0 | export at lib/request.js:214 |  |  | 0.529 |
| walker |  | 1671 | 0 | export at lib/request.js:269 |  |  | 0.529 |
| walker |  | 1683 | 12 | export at lib/request.js:63 |  |  | 0.531 |
| walker |  | 1698 | 15 | export at lib/request.js:469 |  |  | 0.534 |
| walker |  | 1714 | 16 | export at lib/request.js:230 |  |  | 0.539 |
| walker |  | 1730 | 16 | export at lib/request.js:297 |  |  | 0.544 |
| walker |  | 1746 | 16 | export at lib/request.js:326 |  |  | 0.550 |
| walker |  | 1762 | 16 | export at lib/request.js:340 |  |  | 0.555 |
| walker |  | 1778 | 16 | export at lib/request.js:418 |  |  | 0.562 |
| walker |  | 1794 | 16 | export at lib/request.js:444 |  |  | 0.569 |
| walker |  | 1810 | 16 | export at lib/request.js:508 |  |  | 0.577 |
| walker |  | 1827 | 17 | export at lib/request.js:357 |  |  | 0.584 |
| walker |  | 1844 | 17 | export at lib/request.js:403 |  |  | 0.593 |
| walker |  | 1861 | 17 | export at lib/request.js:497 |  |  | 0.602 |
| walker |  | 1880 | 19 | export at lib/request.js:383 |  |  | 0.612 |
| ns | 1896 |  | 233 | createApplication() body | 2.6 |  | 0.568 |
| walker |  | 1897 | 17 | module item at lib/request.js:30 |  |  | 0.570 |
| walker |  | 2030 | 133 | export names surface in lib/express.js |  |  | 0.640 |
| walker |  | 2030 | 0 | export at lib/express.js:27 |  |  | 0.640 |
| walker |  | 2030 | 0 | export at lib/express.js:36 |  |  | 0.640 |
| walker |  | 2030 | 0 | export at lib/express.js:62 |  |  | 0.640 |
| walker |  | 2030 | 0 | export at lib/express.js:70 |  |  | 0.640 |
| walker |  | 2030 | 0 | export at lib/express.js:77 |  |  | 0.640 |
| walker |  | 2047 | 17 | export doc at lib/express.js:70 |  |  | 0.640 |
| walker |  | 2064 | 17 | export doc at lib/express.js:77 |  |  | 0.640 |
| walker |  | 2084 | 20 | export doc at lib/express.js:62 |  |  | 0.640 |
| walker |  | 2106 | 22 | export doc at lib/express.js:27 |  |  | 0.640 |
| walker |  | 2130 | 24 | export doc at lib/request.js:37 |  |  | 0.640 |
| ns | 2153 |  | 257 | HTTP-verb delegation and app.all() | 2.7 |  | 0.592 |
| walker |  | 2154 | 24 | imports in lib/request.js |  |  | 0.592 |
| walker |  | 2179 | 25 | imports in lib/express.js |  |  | 0.592 |
| walker |  | 2206 | 27 | imports in lib/application.js |  |  | 0.592 |
| ns | 2244 |  | 91 | app.* full signature lines | 2.8 | 2.2 | 0.566 |
| ns | 2369 |  | 125 | res.* full signature lines | 2.9 | 2.3 | 0.535 |
| ns | 2478 |  | 109 | Default settings established at boot | 3.1 |  | 0.523 |
| walker |  | 2482 | 276 | export names surface in lib/application.js |  |  | 0.614 |
| walker |  | 2482 | 0 | export at lib/application.js:59 |  |  | 0.614 |
| walker |  | 2482 | 0 | export at lib/application.js:90 |  |  | 0.614 |
| walker |  | 2482 | 0 | export at lib/application.js:152 |  |  | 0.614 |
| walker |  | 2482 | 0 | export at lib/application.js:190 |  |  | 0.614 |
| walker |  | 2482 | 0 | export at lib/application.js:256 |  |  | 0.614 |
| walker |  | 2482 | 0 | export at lib/application.js:294 |  |  | 0.614 |
| walker |  | 2482 | 0 | export at lib/application.js:322 |  |  | 0.614 |
| walker |  | 2482 | 0 | export at lib/application.js:351 |  |  | 0.614 |
| walker |  | 2482 | 0 | export at lib/application.js:399 |  |  | 0.614 |
| walker |  | 2482 | 0 | export at lib/application.js:420 |  |  | 0.614 |
| walker |  | 2482 | 0 | export at lib/application.js:439 |  |  | 0.614 |
| walker |  | 2482 | 0 | export at lib/application.js:451 |  |  | 0.614 |
| walker |  | 2482 | 0 | export at lib/application.js:463 |  |  | 0.614 |
| walker |  | 2482 | 0 | export at lib/application.js:494 |  |  | 0.614 |
| walker |  | 2482 | 0 | export at lib/application.js:522 |  |  | 0.614 |
| walker |  | 2482 | 0 | export at lib/application.js:598 |  |  | 0.614 |
| walker |  | 2499 | 17 | export doc at lib/application.js:90 |  |  | 0.614 |
| walker |  | 2585 | 86 | listing of 'test/acceptance' |  |  | 0.615 |
| walker |  | 2608 | 23 | Readme.md section #19 |  |  | 0.615 |
| walker |  | 2631 | 23 | Readme.md section #30 |  |  | 0.615 |
| ns | 2651 |  | 173 | Remaining boot configuration: locals, mountpath, view defaults | 3.2 |  | 0.591 |
| walker |  | 2655 | 24 | Readme.md section #12 |  |  | 0.591 |
| walker |  | 2679 | 24 | Readme.md section #32 |  |  | 0.591 |
| walker |  | 2721 | 42 | export doc at lib/express.js:36 |  |  | 0.591 |
| walker |  | 2746 | 25 | Readme.md section #18 |  |  | 0.591 |
| walker |  | 2828 | 82 | Readme.md section #7 |  |  | 0.591 |
| walker |  | 2854 | 26 | Readme.md section #23 |  |  | 0.591 |
| walker |  | 2879 | 25 | Readme.md section #24 |  |  | 0.591 |
| walker |  | 2906 | 27 | Readme.md section #13 |  |  | 0.591 |
| walker |  | 2932 | 26 | Readme.md section #14 |  |  | 0.591 |
| ns | 2955 |  | 304 | app.set() — storage plus the three derived-setting side effects | 3.3 |  | 0.553 |
| walker |  | 2959 | 27 | Readme.md section #15 |  |  | 0.553 |
| walker |  | 2985 | 26 | Readme.md section #16 |  |  | 0.553 |
| walker |  | 3010 | 25 | Readme.md section #17 |  |  | 0.553 |
| walker |  | 3059 | 49 | export doc at lib/request.js:403 |  |  | 0.553 |
| walker |  | 3087 | 28 | Readme.md section #22 |  |  | 0.553 |
| walker |  | 3137 | 50 | export doc at lib/request.js:508 |  |  | 0.553 |
| ns | 3140 |  | 185 | compileETag — accepted values of the `etag` setting | 3.4 |  | 0.531 |
| walker |  | 3178 | 41 | imports in lib/response.js |  |  | 0.531 |
| ns | 3357 |  | 217 | compileQueryParser — accepted values of `query parser` | 3.5 |  | 0.508 |
| ns | 3556 |  | 199 | compileTrust — accepted values of `trust proxy` | 3.6 |  | 0.492 |
| walker |  | 3638 | 460 | package runtime dependencies in package.json |  |  | 0.493 |
| walker |  | 3668 | 30 | Readme.md section #27 |  |  | 0.493 |
| ns | 3688 |  | 132 | req.ip / req.ips — the consumers of `trust proxy fn` | 3.7 | 2.4 | 0.484 |
| walker |  | 3698 | 30 | Readme.md section #29 |  |  | 0.484 |
| walker |  | 3727 | 29 | Readme.md section #28 |  |  | 0.484 |
| ns | 3981 |  | 293 | Settings read-sites scattered outside defaultConfiguration | 3.8 |  | 0.467 |
| walker |  | 4037 | 310 | export names surface in lib/response.js |  |  | 0.540 |
| walker |  | 4037 | 0 | export at lib/response.js:49 |  |  | 0.540 |
| walker |  | 4037 | 0 | export at lib/response.js:64 |  |  | 0.540 |
| walker |  | 4037 | 0 | export at lib/response.js:97 |  |  | 0.540 |
| walker |  | 4037 | 0 | export at lib/response.js:125 |  |  | 0.540 |
| walker |  | 4037 | 0 | export at lib/response.js:232 |  |  | 0.540 |
| walker |  | 4037 | 0 | export at lib/response.js:260 |  |  | 0.540 |
| walker |  | 4037 | 0 | export at lib/response.js:321 |  |  | 0.540 |
| walker |  | 4037 | 0 | export at lib/response.js:371 |  |  | 0.540 |
| walker |  | 4037 | 0 | export at lib/response.js:433 |  |  | 0.540 |
| walker |  | 4037 | 0 | export at lib/response.js:569 |  |  | 0.540 |
| walker |  | 4037 | 0 | export at lib/response.js:604 |  |  | 0.540 |
| walker |  | 4037 | 0 | export at lib/response.js:629 |  |  | 0.540 |
| walker |  | 4037 | 0 | export at lib/response.js:696 |  |  | 0.540 |
| walker |  | 4037 | 0 | export at lib/response.js:709 |  |  | 0.540 |
| walker |  | 4037 | 0 | export at lib/response.js:742 |  |  | 0.540 |
| walker |  | 4037 | 0 | export at lib/response.js:794 |  |  | 0.540 |
| walker |  | 4037 | 0 | export at lib/response.js:812 |  |  | 0.540 |
| walker |  | 4037 | 0 | export at lib/response.js:875 |  |  | 0.540 |
| walker |  | 4037 | 0 | export at lib/response.js:894 |  |  | 0.540 |
| walker |  | 4050 | 13 | export at lib/response.js:503 |  |  | 0.546 |
| walker |  | 4064 | 14 | export at lib/response.js:664 |  |  | 0.553 |
| walker |  | 4080 | 16 | module item at lib/response.js:42 |  |  | 0.556 |
| ns | 4101 |  | 120 | app.init() — per-app state and the lazy base router | 3.9 |  | 0.544 |
| walker |  | 4104 | 24 | export doc at lib/response.js:49 |  |  | 0.544 |
| walker |  | 4161 | 57 | export doc at lib/application.js:451 |  |  | 0.544 |
| walker |  | 4218 | 57 | export doc at lib/application.js:463 |  |  | 0.544 |
| walker |  | 4275 | 57 | export doc at lib/response.js:696 |  |  | 0.544 |
| ns | 4332 |  | 231 | Sub-app mounting: the 'mount' event and setting inheritance | 3.10 |  | 0.530 |
| ns | 4478 |  | 146 | JSDoc for app.set() | 3.11 |  | 0.520 |
| walker |  | 4613 | 338 | package identity metadata in package.json |  |  | 0.536 |
| walker |  | 4645 | 32 | Readme.md section #11 |  |  | 0.536 |
| walker |  | 4677 | 32 | Readme.md section #21 |  |  | 0.536 |
| walker |  | 4709 | 32 | Readme.md section #31 |  |  | 0.536 |
| ns | 4735 |  | 257 | app.handle() — the per-request dispatch entry point | 4.1 | 2.8 | 0.519 |
| walker |  | 4840 | 131 | Readme.md section #3 |  |  | 0.519 |
| walker |  | 4899 | 59 | export doc at lib/request.js:326 |  |  | 0.519 |
| walker |  | 4960 | 61 | export doc at lib/application.js:59 |  |  | 0.519 |
| walker |  | 4993 | 33 | Readme.md section #26 |  |  | 0.519 |
| ns | 5006 |  | 271 | app.use() — argument/path disambiguation | 4.2 | 2.8 | 0.502 |
| walker |  | 5025 | 32 | Readme.md section #25 |  |  | 0.502 |
| walker |  | 5129 | 104 | Readme.md section #10 |  |  | 0.502 |
| walker |  | 5192 | 63 | export doc at lib/request.js:140 |  |  | 0.502 |
| ns | 5275 |  | 269 | app.use() — the sub-app mounting branch | 4.3 | 4.2 | 0.487 |
| walker |  | 5316 | 124 | Readme.md section #5 |  |  | 0.487 |
| walker |  | 5382 | 66 | export doc at lib/response.js:604 |  |  | 0.487 |
| walker |  | 5450 | 68 | export doc at lib/request.js:469 |  |  | 0.487 |
| walker |  | 5521 | 71 | export doc at lib/application.js:152 |  |  | 0.487 |
| ns | 5590 |  | 315 | res.send() body-type dispatch | 4.4 | 2.9 | 0.471 |
| walker |  | 5593 | 72 | export doc at lib/response.js:709 |  |  | 0.471 |
| walker |  | 5670 | 77 | export doc at lib/request.js:497 |  |  | 0.471 |
| walker |  | 5749 | 79 | export doc at lib/request.js:340 |  |  | 0.471 |
| ns | 5771 |  | 181 | View.prototype.lookup() — view file resolution | 4.5 | 2.5 | 0.462 |
| walker |  | 5829 | 80 | export doc at lib/request.js:185 |  |  | 0.462 |
| walker |  | 5910 | 81 | export doc at lib/request.js:230 |  |  | 0.462 |
| walker |  | 5996 | 86 | export doc at lib/response.js:875 |  |  | 0.462 |
| walker |  | 6142 | 146 | Readme.md section #6 |  |  | 0.462 |
| walker |  | 6179 | 37 | Readme.md section #8 |  |  | 0.462 |
| ns | 6222 |  | 451 | res.send() response finalization | 4.6 | 4.4 | 0.443 |
| walker |  | 6267 | 88 | export doc at lib/application.js:256 |  |  | 0.443 |
| walker |  | 6359 | 92 | export doc at lib/response.js:232 |  |  | 0.443 |
| walker |  | 6454 | 95 | export doc at lib/application.js:190 |  |  | 0.443 |
| walker |  | 6549 | 95 | export doc at lib/application.js:494 |  |  | 0.443 |
| ns | 6598 |  | 376 | test/ listing — every spec file | 5.1 |  | 0.505 |
| ns | 6684 |  | 86 | test/acceptance/ listing | 5.2 |  | 0.517 |
| ns | 6785 |  | 101 | examples/ listing | 5.3 |  | 0.533 |
| walker |  | 6791 | 242 | Readme.md section #1 |  |  | 0.533 |
| walker |  | 6889 | 98 | export doc at lib/request.js:418 |  |  | 0.533 |
| walker |  | 6988 | 99 | export doc at lib/request.js:444 |  |  | 0.533 |
| ns | 7041 |  | 256 | examples/README.md — annotated example index (first half) | 5.4 |  | 0.527 |
| walker |  | 7087 | 99 | export doc at lib/response.js:260 |  |  | 0.527 |
| walker |  | 7099 | 12 | imports in lib/view.js |  |  | 0.527 |
| walker |  | 7213 | 114 | export doc at lib/application.js:399 |  |  | 0.527 |
| ns | 7299 |  | 258 | examples/README.md — annotated example index (second half) | 5.5 | 5.4 | 0.522 |
| walker |  | 7327 | 114 | export doc at lib/response.js:125 |  |  | 0.522 |
| ns | 7390 |  | 91 | test/support/ and test/fixtures/ listings | 5.6 |  | 0.515 |
| walker |  | 7446 | 119 | export doc at lib/application.js:322 |  |  | 0.515 |
| ns | 7542 |  | 152 | Test harness: env preamble, assertion helpers, template engine | 5.7 |  | 0.511 |
| walker |  | 7567 | 121 | export doc at lib/application.js:439 |  |  | 0.511 |
| ns | 7673 |  | 131 | A complete example app: examples/hello-world/index.js | 5.8 |  | 0.506 |
| walker |  | 7691 | 124 | export doc at lib/application.js:420 |  |  | 0.506 |
| walker |  | 7823 | 132 | export doc at lib/response.js:321 |  |  | 0.506 |
| ns | 7834 |  | 161 | Test-writing idiom: head of test/app.js | 5.9 |  | 0.500 |
| walker |  | 7957 | 134 | export doc at lib/response.js:812 |  |  | 0.500 |
| ns | 8120 |  | 286 | Runtime dependencies (all 28) | 6.1 |  | 0.516 |
| walker |  | 8174 | 217 | export body at lib/express.js:36 body 37 |  |  | 0.539 |
| walker |  | 8277 | 103 | export names surface in lib/view.js |  |  | 0.542 |
| walker |  | 8277 | 0 | export at lib/view.js:36 |  |  | 0.542 |
| walker |  | 8277 | 0 | export at lib/view.js:52 |  |  | 0.542 |
| walker |  | 8277 | 0 | export at lib/view.js:104 |  |  | 0.542 |
| walker |  | 8277 | 0 | export at lib/view.js:133 |  |  | 0.542 |
| walker |  | 8277 | 0 | export at lib/view.js:169 |  |  | 0.542 |
| ns | 8346 |  | 226 | ci.yml — jobs and the OS/Node matrix | 6.2 |  | 0.535 |
| walker |  | 8412 | 135 | export doc at lib/request.js:357 |  |  | 0.535 |
| ns | 8533 |  | 187 | .eslintrc.yml — the complete lint rule set | 6.3 |  | 0.531 |
| walker |  | 8552 | 140 | export doc at lib/request.js:297 |  |  | 0.531 |
| walker |  | 8693 | 141 | export doc at lib/application.js:351 |  |  | 0.543 |
| ns | 8807 |  | 274 | Dev dependencies (all 16) | 6.4 |  | 0.537 |
| walker |  | 8835 | 142 | export doc at lib/response.js:894 |  |  | 0.538 |
| ns | 8888 |  | 81 | package.json remainder: author, published files | 6.5 |  | 0.543 |
| walker |  | 9080 | 245 | Readme.md section #4 |  |  | 0.543 |
| ns | 9121 |  | 233 | res.sendFile() option bag (JSDoc) | 7.1 |  | 0.537 |
| walker |  | 9239 | 159 | export doc at lib/application.js:522 |  |  | 0.537 |
| ns | 9391 |  | 270 | res.cookie() option bag (JSDoc) | 7.2 |  | 0.530 |
| walker |  | 9400 | 161 | export doc at lib/response.js:794 |  |  | 0.530 |
| walker |  | 9564 | 164 | export doc at lib/response.js:629 |  |  | 0.530 |
| ns | 9673 |  | 282 | res.status() and res.render() contracts (JSDoc) | 7.3 |  | 0.528 |
| walker |  | 9729 | 165 | export doc at lib/response.js:64 |  |  | 0.540 |
| walker |  | 9911 | 182 | export doc at lib/response.js:97 |  |  | 0.540 |
| ns | 9971 |  | 298 | History.md — unreleased section and the 5.2.1 heading | 8.1 |  | 0.536 |
