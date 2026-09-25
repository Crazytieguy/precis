Score(3000)=0.443 I=0.780 C=0.252 ns_rows≤3K=19/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.736/0.659/0.602/0.443/0.382/0.341/0.548

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
| walker |  | 197 | 26 | ts names lib/view.js |  |  | 0.918 |
| walker |  | 226 | 29 | ts names lib/response.js |  |  | 0.920 |
| walker |  | 256 | 30 | ts names lib/request.js |  |  | 0.923 |
| walker |  | 287 | 31 | package runtime metadata in package.json |  |  | 0.924 |
| ns | 296 |  | 137 | Module export surface (index.js + lib/express.js exports) | 1.4 |  | 0.731 |
| walker |  | 337 | 50 | package entrypoints in package.json |  |  | 0.732 |
| walker |  | 438 | 101 | listing of 'examples' |  |  | 0.735 |
| ns | 443 |  | 147 | Readme tagline + canonical quick-start snippet | 1.5 |  | 0.607 |
| walker |  | 571 | 133 | ts names lib/express.js |  |  | 0.785 |
| ns | 652 |  | 209 | npm scripts + engines | 1.6 |  | 0.700 |
| ns | 678 |  | 26 | .github listing — all four workflows | 1.7 |  | 0.718 |
| ns | 789 |  | 111 | Prototype bases and module exports of each lib file | 2.1 |  | 0.703 |
| ns | 962 |  | 173 | app.* roster — every application method name | 2.2 |  | 0.619 |
| walker |  | 991 | 420 | README headline in Readme.md |  |  | 0.736 |
| walker |  | 1119 | 128 | headings outline in Readme.md |  |  | 0.736 |
| ns | 1201 |  | 239 | res.* roster — every response method name | 2.3 |  | 0.643 |
| walker |  | 1206 | 87 | Readme.md section #2 |  |  | 0.643 |
| walker |  | 1232 | 26 | ts doc lib/request.js:30 |  |  | 0.643 |
| walker |  | 1258 | 26 | ts doc lib/response.js:42 |  |  | 0.643 |
| walker |  | 1425 | 167 | ts names lib/utils.js |  |  | 0.659 |
| walker |  | 1437 | 12 | ts body lib/utils.js:75 |  |  | 0.659 |
| ns | 1465 |  | 264 | req.* roster — methods and defineGetter properties | 2.4 |  | 0.598 |
| ns | 1663 |  | 198 | lib/view.js and lib/utils.js symbol rosters | 2.5 |  | 0.582 |
| walker |  | 1813 | 376 | listing of 'test' |  |  | 0.589 |
| walker |  | 1825 | 12 | listing of 'test/support' |  |  | 0.589 |
| ns | 1896 |  | 233 | createApplication() body | 2.6 |  | 0.547 |
| walker |  | 2001 | 176 | package scripts in package.json |  |  | 0.602 |
| walker |  | 2040 | 39 | ts doc lib/utils.js:29 |  |  | 0.602 |
| walker |  | 2082 | 42 | ts doc lib/express.js:36 |  |  | 0.602 |
| walker |  | 2127 | 45 | ts body lib/utils.js:61 |  |  | 0.602 |
| ns | 2153 |  | 257 | HTTP-verb delegation and app.all() | 2.7 |  | 0.557 |
| walker |  | 2190 | 63 | ts doc lib/utils.js:162 |  |  | 0.557 |
| ns | 2244 |  | 91 | app.* full signature lines | 2.8 | 2.2 | 0.533 |
| walker |  | 2276 | 86 | listing of 'test/acceptance' |  |  | 0.534 |
| walker |  | 2299 | 23 | Readme.md section #19 |  |  | 0.534 |
| walker |  | 2322 | 23 | Readme.md section #30 |  |  | 0.534 |
| walker |  | 2346 | 24 | Readme.md section #12 |  |  | 0.534 |
| ns | 2369 |  | 125 | res.* full signature lines | 2.9 | 2.3 | 0.505 |
| walker |  | 2370 | 24 | Readme.md section #32 |  |  | 0.505 |
| walker |  | 2395 | 25 | Readme.md section #18 |  |  | 0.505 |
| walker |  | 2477 | 82 | Readme.md section #7 |  |  | 0.505 |
| ns | 2478 |  | 109 | Default settings established at boot | 3.1 |  | 0.493 |
| walker |  | 2503 | 26 | Readme.md section #23 |  |  | 0.493 |
| walker |  | 2528 | 25 | Readme.md section #24 |  |  | 0.493 |
| walker |  | 2555 | 27 | Readme.md section #13 |  |  | 0.493 |
| walker |  | 2581 | 26 | Readme.md section #14 |  |  | 0.493 |
| walker |  | 2608 | 27 | Readme.md section #15 |  |  | 0.493 |
| walker |  | 2634 | 26 | Readme.md section #16 |  |  | 0.493 |
| ns | 2651 |  | 173 | Remaining boot configuration: locals, mountpath, view defaults | 3.2 |  | 0.473 |
| walker |  | 2659 | 25 | Readme.md section #17 |  |  | 0.473 |
| walker |  | 2687 | 28 | Readme.md section #22 |  |  | 0.473 |
| ns | 2955 |  | 304 | app.set() — storage plus the three derived-setting side effects | 3.3 |  | 0.443 |
| ns | 3140 |  | 185 | compileETag — accepted values of the `etag` setting | 3.4 |  | 0.426 |
| walker |  | 3147 | 460 | package runtime dependencies in package.json |  |  | 0.427 |
| walker |  | 3177 | 30 | Readme.md section #27 |  |  | 0.427 |
| walker |  | 3207 | 30 | Readme.md section #29 |  |  | 0.427 |
| walker |  | 3236 | 29 | Readme.md section #28 |  |  | 0.427 |
| walker |  | 3300 | 64 | ts doc lib/utils.js:130 |  |  | 0.427 |
| ns | 3357 |  | 217 | compileQueryParser — accepted values of `query parser` | 3.5 |  | 0.408 |
| ns | 3556 |  | 199 | compileTrust — accepted values of `trust proxy` | 3.6 |  | 0.395 |
| walker |  | 3638 | 338 | package identity metadata in package.json |  |  | 0.413 |
| walker |  | 3670 | 32 | Readme.md section #11 |  |  | 0.413 |
| ns | 3688 |  | 132 | req.ip / req.ips — the consumers of `trust proxy fn` | 3.7 | 2.4 | 0.404 |
| walker |  | 3702 | 32 | Readme.md section #21 |  |  | 0.404 |
| walker |  | 3734 | 32 | Readme.md section #31 |  |  | 0.404 |
| walker |  | 3865 | 131 | Readme.md section #3 |  |  | 0.404 |
| walker |  | 3898 | 33 | Readme.md section #26 |  |  | 0.404 |
| walker |  | 3930 | 32 | Readme.md section #25 |  |  | 0.404 |
| ns | 3981 |  | 293 | Settings read-sites scattered outside defaultConfiguration | 3.8 |  | 0.390 |
| walker |  | 4034 | 104 | Readme.md section #10 |  |  | 0.390 |
| ns | 4101 |  | 120 | app.init() — per-app state and the lazy base router | 3.9 |  | 0.382 |
| walker |  | 4158 | 124 | Readme.md section #5 |  |  | 0.382 |
| walker |  | 4287 | 129 | ts doc lib/view.js:52 |  |  | 0.382 |
| ns | 4332 |  | 231 | Sub-app mounting: the 'mount' event and setting inheritance | 3.10 |  | 0.372 |
| walker |  | 4352 | 65 | ts doc lib/utils.js:61 |  |  | 0.372 |
| ns | 4478 |  | 146 | JSDoc for app.set() | 3.11 |  | 0.365 |
| walker |  | 4498 | 146 | Readme.md section #6 |  |  | 0.365 |
| walker |  | 4535 | 37 | Readme.md section #8 |  |  | 0.365 |
| ns | 4735 |  | 257 | app.handle() — the per-request dispatch entry point | 4.1 | 2.8 | 0.353 |
| walker |  | 4777 | 242 | Readme.md section #1 |  |  | 0.353 |
| walker |  | 4842 | 65 | ts doc lib/utils.js:75 |  |  | 0.353 |
| ns | 5006 |  | 271 | app.use() — argument/path disambiguation | 4.2 | 2.8 | 0.341 |
| walker |  | 5059 | 217 | ts body lib/express.js:36 |  |  | 0.387 |
| ns | 5275 |  | 269 | app.use() — the sub-app mounting branch | 4.3 | 4.2 | 0.376 |
| walker |  | 5304 | 245 | Readme.md section #4 |  |  | 0.376 |
| walker |  | 5373 | 69 | ts doc lib/utils.js:194 |  |  | 0.376 |
| walker |  | 5444 | 71 | ts doc lib/utils.js:249 |  |  | 0.376 |
| walker |  | 5528 | 84 | Readme.md section #9 |  |  | 0.376 |
| ns | 5590 |  | 315 | res.send() body-type dispatch | 4.4 | 2.9 | 0.363 |
| ns | 5771 |  | 181 | View.prototype.lookup() — view file resolution | 4.5 | 2.5 | 0.356 |
| walker |  | 5954 | 426 | ts body lib/view.js:52 |  |  | 0.356 |
| walker |  | 6026 | 72 | ts doc lib/utils.js:40 |  |  | 0.356 |
| walker |  | 6037 | 11 | History.md section #0 |  |  | 0.356 |
| ns | 6222 |  | 451 | res.send() response finalization | 4.6 | 4.4 | 0.341 |
| walker |  | 6377 | 340 | Readme.md section #34 |  |  | 0.341 |
| walker |  | 6449 | 72 | ts doc lib/utils.js:51 |  |  | 0.341 |
| walker |  | 6522 | 73 | ts doc lib/utils.js:225 |  |  | 0.341 |
| walker |  | 6595 | 73 | ts body lib/utils.js:249 |  |  | 0.341 |
| ns | 6598 |  | 376 | test/ listing — every spec file | 5.1 |  | 0.423 |
| ns | 6684 |  | 86 | test/acceptance/ listing | 5.2 |  | 0.438 |
| walker |  | 6699 | 104 | ts body lib/utils.js:225 |  |  | 0.438 |
| ns | 6785 |  | 101 | examples/ listing | 5.3 |  | 0.458 |
| walker |  | 6879 | 180 | ts body lib/utils.js:130 |  |  | 0.483 |
| ns | 7041 |  | 256 | examples/README.md — annotated example index (first half) | 5.4 |  | 0.477 |
| walker |  | 7060 | 181 | ts body lib/utils.js:162 |  |  | 0.498 |
| walker |  | 7254 | 194 | ts body lib/utils.js:194 |  |  | 0.519 |
| ns | 7299 |  | 258 | examples/README.md — annotated example index (second half) | 5.5 | 5.4 | 0.514 |
| ns | 7390 |  | 91 | test/support/ and test/fixtures/ listings | 5.6 |  | 0.506 |
| walker |  | 7411 | 157 | Readme.md section #20 |  |  | 0.506 |
| walker |  | 7490 | 79 | listing of 'test/fixtures' |  |  | 0.526 |
| walker |  | 7494 | 4 | listing of 'test/fixtures/pets' |  |  | 0.526 |
| walker |  | 7499 | 5 | listing of 'test/fixtures/local_layout' |  |  | 0.526 |
| walker |  | 7506 | 7 | listing of 'test/fixtures/blog' |  |  | 0.526 |
| walker |  | 7513 | 7 | listing of 'test/fixtures/snow ☃' |  |  | 0.526 |
| walker |  | 7518 | 5 | listing of 'test/fixtures/blog/post' |  |  | 0.526 |
| walker |  | 7527 | 9 | listing of 'test/fixtures/users' |  |  | 0.526 |
| walker |  | 7537 | 10 | listing of 'test/fixtures/default_layout' |  |  | 0.526 |
| ns | 7542 |  | 152 | Test harness: env preamble, assertion helpers, template engine | 5.7 |  | 0.523 |
| walker |  | 7543 | 6 | declaration surface of test/fixtures/% of dogs.txt |  |  | 0.523 |
| walker |  | 7550 | 7 | declaration surface of test/fixtures/name.txt |  |  | 0.523 |
| walker |  | 7557 | 7 | declaration surface of test/fixtures/todo.txt |  |  | 0.523 |
| ns | 7673 |  | 131 | A complete example app: examples/hello-world/index.js | 5.8 |  | 0.517 |
| ns | 7834 |  | 161 | Test-writing idiom: head of test/app.js | 5.9 |  | 0.511 |
| walker |  | 8076 | 519 | README headline in examples/README.md |  |  | 0.537 |
| ns | 8120 |  | 286 | Runtime dependencies (all 28) | 6.1 |  | 0.550 |
| ns | 8346 |  | 226 | ci.yml — jobs and the OS/Node matrix | 6.2 |  | 0.543 |
| ns | 8533 |  | 187 | .eslintrc.yml — the complete lint rule set | 6.3 |  | 0.538 |
| walker |  | 8781 | 705 | YAML config at .github/workflows/ci.yml |  |  | 0.549 |
| walker |  | 8789 | 8 | declaration surface of test/fixtures/nums.txt |  |  | 0.549 |
| ns | 8807 |  | 274 | Dev dependencies (all 16) | 6.4 |  | 0.543 |
| ns | 8888 |  | 81 | package.json remainder: author, published files | 6.5 |  | 0.548 |
| ns | 9121 |  | 233 | res.sendFile() option bag (JSDoc) | 7.1 |  | 0.542 |
| ns | 9391 |  | 270 | res.cookie() option bag (JSDoc) | 7.2 |  | 0.535 |
| ns | 9673 |  | 282 | res.status() and res.render() contracts (JSDoc) | 7.3 |  | 0.529 |
| walker |  | 9790 | 1001 | Readme.md section #33 |  |  | 0.529 |
| ns | 9971 |  | 298 | History.md — unreleased section and the 5.2.1 heading | 8.1 |  | 0.525 |
