Score(3000)=0.624 I=0.848 C=0.459 ns_rows≤3K=19/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.603/0.631/0.622/0.624/0.516/0.420/0.532

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
| walker |  | 1328 | 87 | Readme.md section #2 |  |  | 0.631 |
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
| ns | 2153 |  | 257 | HTTP-verb delegation and app.all() | 2.7 |  | 0.575 |
| walker |  | 2201 | 309 | ts names lib/response.js |  |  | 0.635 |
| walker |  | 2212 | 11 | ts body lib/response.js:696 |  |  | 0.635 |
| walker |  | 2227 | 15 | ts body lib/response.js:794 |  |  | 0.635 |
| ns | 2244 |  | 91 | app.* full signature lines | 2.8 | 2.2 | 0.656 |
| walker |  | 2250 | 23 | ts body lib/response.js:875 |  |  | 0.656 |
| walker |  | 2276 | 26 | ts doc lib/response.js:42 |  |  | 0.656 |
| ns | 2369 |  | 125 | res.* full signature lines | 2.9 | 2.3 | 0.663 |
| walker |  | 2452 | 176 | package scripts in package.json |  |  | 0.703 |
| walker |  | 2470 | 18 | ts doc lib/application.js:40 |  |  | 0.703 |
| ns | 2478 |  | 109 | Default settings established at boot | 3.1 |  | 0.687 |
| walker |  | 2497 | 27 | ts doc lib/application.js:90 |  |  | 0.687 |
| walker |  | 2523 | 26 | ts body lib/request.js:140 |  |  | 0.687 |
| ns | 2651 |  | 173 | Remaining boot configuration: locals, mountpath, view defaults | 3.2 |  | 0.659 |
| walker |  | 2899 | 376 | listing of 'test' |  |  | 0.666 |
| walker |  | 2911 | 12 | listing of 'test/support' |  |  | 0.666 |
| walker |  | 2950 | 39 | ts doc lib/utils.js:29 |  |  | 0.666 |
| ns | 2955 |  | 304 | app.set() — storage plus the three derived-setting side effects | 3.3 |  | 0.624 |
| ns | 3140 |  | 185 | compileETag — accepted values of the `etag` setting | 3.4 |  | 0.599 |
| ns | 3357 |  | 217 | compileQueryParser — accepted values of `query parser` | 3.5 |  | 0.572 |
| walker |  | 3410 | 460 | package runtime dependencies in package.json |  |  | 0.574 |
| walker |  | 3452 | 42 | ts doc lib/express.js:36 |  |  | 0.574 |
| walker |  | 3497 | 45 | ts body lib/utils.js:61 |  |  | 0.574 |
| walker |  | 3545 | 48 | ts doc lib/view.js:104 |  |  | 0.574 |
| ns | 3556 |  | 199 | compileTrust — accepted values of `trust proxy` | 3.6 |  | 0.556 |
| walker |  | 3603 | 58 | ts doc lib/view.js:133 |  |  | 0.556 |
| walker |  | 3663 | 60 | ts doc lib/view.js:169 |  |  | 0.556 |
| ns | 3688 |  | 132 | req.ip / req.ips — the consumers of `trust proxy fn` | 3.7 | 2.4 | 0.544 |
| walker |  | 3726 | 63 | ts doc lib/utils.js:162 |  |  | 0.544 |
| walker |  | 3812 | 86 | listing of 'test/acceptance' |  |  | 0.545 |
| walker |  | 3835 | 23 | Readme.md section #19 |  |  | 0.545 |
| walker |  | 3858 | 23 | Readme.md section #30 |  |  | 0.545 |
| walker |  | 3882 | 24 | Readme.md section #12 |  |  | 0.545 |
| walker |  | 3906 | 24 | Readme.md section #32 |  |  | 0.545 |
| walker |  | 3931 | 25 | Readme.md section #18 |  |  | 0.545 |
| ns | 3981 |  | 293 | Settings read-sites scattered outside defaultConfiguration | 3.8 |  | 0.527 |
| walker |  | 4013 | 82 | Readme.md section #7 |  |  | 0.527 |
| ns | 4101 |  | 120 | app.init() — per-app state and the lazy base router | 3.9 |  | 0.516 |
| ns | 4332 |  | 231 | Sub-app mounting: the 'mount' event and setting inheritance | 3.10 |  | 0.503 |
| walker |  | 4351 | 338 | package identity metadata in package.json |  |  | 0.519 |
| walker |  | 4377 | 26 | Readme.md section #23 |  |  | 0.519 |
| walker |  | 4402 | 25 | Readme.md section #24 |  |  | 0.519 |
| walker |  | 4429 | 27 | Readme.md section #13 |  |  | 0.519 |
| walker |  | 4455 | 26 | Readme.md section #14 |  |  | 0.519 |
| ns | 4478 |  | 146 | JSDoc for app.set() | 3.11 |  | 0.509 |
| walker |  | 4482 | 27 | Readme.md section #15 |  |  | 0.509 |
| walker |  | 4508 | 26 | Readme.md section #16 |  |  | 0.509 |
| walker |  | 4533 | 25 | Readme.md section #17 |  |  | 0.509 |
| walker |  | 4571 | 38 | ts body lib/request.js:214 |  |  | 0.509 |
| walker |  | 4628 | 57 | ts doc lib/response.js:696 |  |  | 0.509 |
| walker |  | 4656 | 28 | Readme.md section #22 |  |  | 0.509 |
| walker |  | 4686 | 30 | Readme.md section #27 |  |  | 0.509 |
| walker |  | 4716 | 30 | Readme.md section #29 |  |  | 0.509 |
| ns | 4735 |  | 257 | app.handle() — the per-request dispatch entry point | 4.1 | 2.8 | 0.492 |
| walker |  | 4745 | 29 | Readme.md section #28 |  |  | 0.492 |
| walker |  | 4809 | 64 | ts doc lib/utils.js:130 |  |  | 0.492 |
| walker |  | 4841 | 32 | Readme.md section #11 |  |  | 0.492 |
| walker |  | 4873 | 32 | Readme.md section #21 |  |  | 0.492 |
| walker |  | 4905 | 32 | Readme.md section #31 |  |  | 0.492 |
| ns | 5006 |  | 271 | app.use() — argument/path disambiguation | 4.2 | 2.8 | 0.476 |
| walker |  | 5036 | 131 | Readme.md section #3 |  |  | 0.476 |
| walker |  | 5069 | 33 | Readme.md section #26 |  |  | 0.476 |
| walker |  | 5101 | 32 | Readme.md section #25 |  |  | 0.476 |
| walker |  | 5205 | 104 | Readme.md section #10 |  |  | 0.476 |
| ns | 5275 |  | 269 | app.use() — the sub-app mounting branch | 4.3 | 4.2 | 0.463 |
| walker |  | 5329 | 124 | Readme.md section #5 |  |  | 0.463 |
| walker |  | 5360 | 31 | ts body lib/application.js:399 |  |  | 0.463 |
| walker |  | 5489 | 129 | ts doc lib/view.js:52 |  |  | 0.463 |
| walker |  | 5548 | 59 | ts body lib/response.js:321 |  |  | 0.463 |
| ns | 5590 |  | 315 | res.send() body-type dispatch | 4.4 | 2.9 | 0.447 |
| walker |  | 5613 | 65 | ts doc lib/utils.js:61 |  |  | 0.447 |
| walker |  | 5759 | 146 | Readme.md section #6 |  |  | 0.447 |
| ns | 5771 |  | 181 | View.prototype.lookup() — view file resolution | 4.5 | 2.5 | 0.439 |
| walker |  | 5796 | 37 | Readme.md section #8 |  |  | 0.439 |
| walker |  | 6038 | 242 | Readme.md section #1 |  |  | 0.439 |
| walker |  | 6098 | 60 | ts body lib/response.js:604 |  |  | 0.439 |
| walker |  | 6161 | 63 | ts doc lib/request.js:140 |  |  | 0.439 |
| ns | 6222 |  | 451 | res.send() response finalization | 4.6 | 4.4 | 0.420 |
| walker |  | 6226 | 65 | ts doc lib/utils.js:75 |  |  | 0.420 |
| walker |  | 6443 | 217 | ts body lib/express.js:36 |  |  | 0.454 |
| walker |  | 6509 | 66 | ts doc lib/response.js:604 |  |  | 0.454 |
| ns | 6598 |  | 376 | test/ listing — every spec file | 5.1 |  | 0.514 |
| ns | 6684 |  | 86 | test/acceptance/ listing | 5.2 |  | 0.526 |
| walker |  | 6754 | 245 | Readme.md section #4 |  |  | 0.526 |
| ns | 6785 |  | 101 | examples/ listing | 5.3 |  | 0.542 |
| walker |  | 6923 | 169 | ts body lib/view.js:169 |  |  | 0.542 |
| walker |  | 6992 | 69 | ts doc lib/utils.js:194 |  |  | 0.542 |
| ns | 7041 |  | 256 | examples/README.md — annotated example index (first half) | 5.4 |  | 0.536 |
| walker |  | 7049 | 57 | ts doc lib/application.js:451 |  |  | 0.536 |
| walker |  | 7129 | 80 | ts doc lib/request.js:185 |  |  | 0.536 |
| walker |  | 7200 | 71 | ts doc lib/utils.js:249 |  |  | 0.536 |
| walker |  | 7272 | 72 | ts doc lib/response.js:709 |  |  | 0.536 |
| ns | 7299 |  | 258 | examples/README.md — annotated example index (second half) | 5.5 | 5.4 | 0.530 |
| walker |  | 7329 | 57 | ts doc lib/application.js:463 |  |  | 0.530 |
| ns | 7390 |  | 91 | test/support/ and test/fixtures/ listings | 5.6 |  | 0.523 |
| walker |  | 7505 | 176 | ts body lib/view.js:104 |  |  | 0.542 |
| ns | 7542 |  | 152 | Test harness: env preamble, assertion helpers, template engine | 5.7 |  | 0.538 |
| walker |  | 7589 | 84 | Readme.md section #9 |  |  | 0.538 |
| walker |  | 7661 | 72 | ts doc lib/utils.js:40 |  |  | 0.538 |
| ns | 7673 |  | 131 | A complete example app: examples/hello-world/index.js | 5.8 |  | 0.532 |
| walker |  | 7740 | 79 | ts body lib/response.js:709 |  |  | 0.532 |
| ns | 7834 |  | 161 | Test-writing idiom: head of test/app.js | 5.9 |  | 0.527 |
| walker |  | 8080 | 340 | Readme.md section #34 |  |  | 0.527 |
| ns | 8120 |  | 286 | Runtime dependencies (all 28) | 6.1 |  | 0.541 |
| walker |  | 8151 | 71 | ts doc lib/application.js:59 |  |  | 0.541 |
| walker |  | 8223 | 72 | ts doc lib/utils.js:51 |  |  | 0.541 |
| walker |  | 8296 | 73 | ts doc lib/utils.js:225 |  |  | 0.541 |
| ns | 8346 |  | 226 | ci.yml — jobs and the OS/Node matrix | 6.2 |  | 0.535 |
| walker |  | 8369 | 73 | ts body lib/utils.js:249 |  |  | 0.535 |
| walker |  | 8450 | 81 | ts doc lib/application.js:152 |  |  | 0.535 |
| ns | 8533 |  | 187 | .eslintrc.yml — the complete lint rule set | 6.3 |  | 0.530 |
| walker |  | 8536 | 86 | ts doc lib/response.js:875 |  |  | 0.530 |
| walker |  | 8624 | 88 | ts doc lib/application.js:256 |  |  | 0.530 |
| walker |  | 8711 | 87 | ts body lib/application.js:494 |  |  | 0.533 |
| walker |  | 8803 | 92 | ts doc lib/response.js:232 |  |  | 0.533 |
| ns | 8807 |  | 274 | Dev dependencies (all 16) | 6.4 |  | 0.527 |
| ns | 8888 |  | 81 | package.json remainder: author, published files | 6.5 |  | 0.532 |
| walker |  | 8898 | 95 | ts doc lib/application.js:190 |  |  | 0.532 |
| walker |  | 8993 | 95 | ts doc lib/application.js:494 |  |  | 0.532 |
| walker |  | 9092 | 99 | ts doc lib/response.js:260 |  |  | 0.532 |
| ns | 9121 |  | 233 | res.sendFile() option bag (JSDoc) | 7.1 |  | 0.527 |
| walker |  | 9193 | 101 | ts body lib/application.js:322 |  |  | 0.527 |
| walker |  | 9296 | 103 | ts body lib/application.js:598 |  |  | 0.527 |
| ns | 9391 |  | 270 | res.cookie() option bag (JSDoc) | 7.2 |  | 0.520 |
| walker |  | 9400 | 104 | ts body lib/utils.js:225 |  |  | 0.520 |
| walker |  | 9514 | 114 | ts doc lib/response.js:125 |  |  | 0.520 |
| walker |  | 9626 | 112 | ts body lib/request.js:269 |  |  | 0.520 |
| ns | 9673 |  | 282 | res.status() and res.render() contracts (JSDoc) | 7.3 |  | 0.514 |
| walker |  | 9745 | 119 | ts doc lib/application.js:322 |  |  | 0.514 |
| walker |  | 9866 | 121 | ts doc lib/application.js:439 |  |  | 0.514 |
| ns | 9971 |  | 298 | History.md — unreleased section and the 5.2.1 heading | 8.1 |  | 0.510 |
| walker |  | 9990 | 124 | ts doc lib/application.js:399 |  |  | 0.510 |
