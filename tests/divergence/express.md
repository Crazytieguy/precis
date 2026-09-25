Score(3000)=0.452 I=0.785 C=0.260 ns_rows≤3K=19/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.635/0.657/0.559/0.452/0.389/0.347/0.521

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
| walker |  | 527 | 89 | ts names lib/application.js |  |  | 0.609 |
| ns | 652 |  | 209 | npm scripts + engines | 1.6 |  | 0.544 |
| walker |  | 660 | 133 | ts names lib/express.js |  |  | 0.703 |
| ns | 678 |  | 26 | .github listing — all four workflows | 1.7 |  | 0.720 |
| ns | 789 |  | 111 | Prototype bases and module exports of each lib file | 2.1 |  | 0.717 |
| ns | 962 |  | 173 | app.* roster — every application method name | 2.2 |  | 0.635 |
| walker |  | 1080 | 420 | README headline in Readme.md |  |  | 0.752 |
| ns | 1201 |  | 239 | res.* roster — every response method name | 2.3 |  | 0.657 |
| walker |  | 1208 | 128 | headings outline in Readme.md |  |  | 0.657 |
| walker |  | 1295 | 87 | Readme.md section #2 |  |  | 0.657 |
| walker |  | 1321 | 26 | ts doc lib/request.js:30 |  |  | 0.657 |
| walker |  | 1347 | 26 | ts doc lib/response.js:42 |  |  | 0.657 |
| ns | 1465 |  | 264 | req.* roster — methods and defineGetter properties | 2.4 |  | 0.596 |
| walker |  | 1514 | 167 | ts names lib/utils.js |  |  | 0.612 |
| walker |  | 1526 | 12 | ts body lib/utils.js:75 |  |  | 0.612 |
| ns | 1663 |  | 198 | lib/view.js and lib/utils.js symbol rosters | 2.5 |  | 0.595 |
| ns | 1896 |  | 233 | createApplication() body | 2.6 |  | 0.553 |
| walker |  | 1902 | 376 | listing of 'test' |  |  | 0.559 |
| walker |  | 1914 | 12 | listing of 'test/support' |  |  | 0.559 |
| walker |  | 2090 | 176 | package scripts in package.json |  |  | 0.614 |
| walker |  | 2129 | 39 | ts doc lib/utils.js:29 |  |  | 0.614 |
| ns | 2153 |  | 257 | HTTP-verb delegation and app.all() | 2.7 |  | 0.568 |
| walker |  | 2171 | 42 | ts doc lib/express.js:36 |  |  | 0.568 |
| walker |  | 2216 | 45 | ts body lib/utils.js:61 |  |  | 0.568 |
| walker |  | 2232 | 16 | ts doc lib/application.js:40 |  |  | 0.568 |
| ns | 2244 |  | 91 | app.* full signature lines | 2.8 | 2.2 | 0.543 |
| walker |  | 2295 | 63 | ts doc lib/utils.js:162 |  |  | 0.543 |
| ns | 2369 |  | 125 | res.* full signature lines | 2.9 | 2.3 | 0.514 |
| walker |  | 2381 | 86 | listing of 'test/acceptance' |  |  | 0.515 |
| walker |  | 2404 | 23 | Readme.md section #19 |  |  | 0.515 |
| walker |  | 2427 | 23 | Readme.md section #30 |  |  | 0.515 |
| walker |  | 2451 | 24 | Readme.md section #12 |  |  | 0.515 |
| walker |  | 2475 | 24 | Readme.md section #32 |  |  | 0.515 |
| ns | 2478 |  | 109 | Default settings established at boot | 3.1 |  | 0.503 |
| walker |  | 2500 | 25 | Readme.md section #18 |  |  | 0.503 |
| walker |  | 2582 | 82 | Readme.md section #7 |  |  | 0.503 |
| walker |  | 2608 | 26 | Readme.md section #23 |  |  | 0.503 |
| walker |  | 2633 | 25 | Readme.md section #24 |  |  | 0.503 |
| ns | 2651 |  | 173 | Remaining boot configuration: locals, mountpath, view defaults | 3.2 |  | 0.483 |
| walker |  | 2660 | 27 | Readme.md section #13 |  |  | 0.483 |
| walker |  | 2686 | 26 | Readme.md section #14 |  |  | 0.483 |
| walker |  | 2713 | 27 | Readme.md section #15 |  |  | 0.483 |
| walker |  | 2739 | 26 | Readme.md section #16 |  |  | 0.483 |
| walker |  | 2764 | 25 | Readme.md section #17 |  |  | 0.483 |
| walker |  | 2792 | 28 | Readme.md section #22 |  |  | 0.483 |
| walker |  | 2818 | 26 | ts doc lib/application.js:33 |  |  | 0.483 |
| ns | 2955 |  | 304 | app.set() — storage plus the three derived-setting side effects | 3.3 |  | 0.452 |
| ns | 3140 |  | 185 | compileETag — accepted values of the `etag` setting | 3.4 |  | 0.434 |
| walker |  | 3278 | 460 | package runtime dependencies in package.json |  |  | 0.435 |
| walker |  | 3308 | 30 | Readme.md section #27 |  |  | 0.435 |
| walker |  | 3338 | 30 | Readme.md section #29 |  |  | 0.435 |
| ns | 3357 |  | 217 | compileQueryParser — accepted values of `query parser` | 3.5 |  | 0.416 |
| walker |  | 3367 | 29 | Readme.md section #28 |  |  | 0.416 |
| walker |  | 3431 | 64 | ts doc lib/utils.js:130 |  |  | 0.416 |
| walker |  | 3459 | 28 | ts doc lib/application.js:625 |  |  | 0.416 |
| ns | 3556 |  | 199 | compileTrust — accepted values of `trust proxy` | 3.6 |  | 0.403 |
| ns | 3688 |  | 132 | req.ip / req.ips — the consumers of `trust proxy fn` | 3.7 | 2.4 | 0.394 |
| walker |  | 3797 | 338 | package identity metadata in package.json |  |  | 0.411 |
| walker |  | 3827 | 30 | ts doc lib/application.js:47 |  |  | 0.411 |
| walker |  | 3859 | 32 | Readme.md section #11 |  |  | 0.411 |
| walker |  | 3891 | 32 | Readme.md section #21 |  |  | 0.411 |
| walker |  | 3923 | 32 | Readme.md section #31 |  |  | 0.411 |
| ns | 3981 |  | 293 | Settings read-sites scattered outside defaultConfiguration | 3.8 |  | 0.397 |
| walker |  | 4054 | 131 | Readme.md section #3 |  |  | 0.397 |
| walker |  | 4087 | 33 | Readme.md section #26 |  |  | 0.397 |
| ns | 4101 |  | 120 | app.init() — per-app state and the lazy base router | 3.9 |  | 0.389 |
| walker |  | 4119 | 32 | Readme.md section #25 |  |  | 0.389 |
| walker |  | 4223 | 104 | Readme.md section #10 |  |  | 0.389 |
| ns | 4332 |  | 231 | Sub-app mounting: the 'mount' event and setting inheritance | 3.10 |  | 0.379 |
| walker |  | 4347 | 124 | Readme.md section #5 |  |  | 0.379 |
| walker |  | 4476 | 129 | ts doc lib/view.js:52 |  |  | 0.379 |
| ns | 4478 |  | 146 | JSDoc for app.set() | 3.11 |  | 0.372 |
| walker |  | 4541 | 65 | ts doc lib/utils.js:61 |  |  | 0.372 |
| walker |  | 4687 | 146 | Readme.md section #6 |  |  | 0.372 |
| walker |  | 4724 | 37 | Readme.md section #8 |  |  | 0.372 |
| ns | 4735 |  | 257 | app.handle() — the per-request dispatch entry point | 4.1 | 2.8 | 0.360 |
| walker |  | 4966 | 242 | Readme.md section #1 |  |  | 0.360 |
| ns | 5006 |  | 271 | app.use() — argument/path disambiguation | 4.2 | 2.8 | 0.348 |
| walker |  | 5031 | 65 | ts doc lib/utils.js:75 |  |  | 0.348 |
| walker |  | 5068 | 37 | ts body lib/application.js:615 |  |  | 0.348 |
| walker |  | 5081 | 13 | ts names test/app.render.js |  |  | 0.348 |
| walker |  | 5094 | 13 | ts names test/res.render.js |  |  | 0.348 |
| walker |  | 5108 | 14 | ts names test/req.query.js |  |  | 0.348 |
| ns | 5275 |  | 269 | app.use() — the sub-app mounting branch | 4.3 | 4.2 | 0.338 |
| walker |  | 5325 | 217 | ts body lib/express.js:36 |  |  | 0.381 |
| walker |  | 5340 | 15 | ts names test/app.router.js |  |  | 0.381 |
| walker |  | 5355 | 15 | ts names test/express.raw.js |  |  | 0.381 |
| walker |  | 5370 | 15 | ts names test/express.text.js |  |  | 0.381 |
| ns | 5590 |  | 315 | res.send() body-type dispatch | 4.4 | 2.9 | 0.369 |
| walker |  | 5615 | 245 | Readme.md section #4 |  |  | 0.369 |
| walker |  | 5631 | 16 | ts names test/req.ip.js |  |  | 0.369 |
| walker |  | 5700 | 69 | ts doc lib/utils.js:194 |  |  | 0.369 |
| walker |  | 5717 | 17 | ts names test/app.engine.js |  |  | 0.369 |
| walker |  | 5736 | 19 | ts names test/res.append.js |  |  | 0.369 |
| ns | 5771 |  | 181 | View.prototype.lookup() — view file resolution | 4.5 | 2.5 | 0.361 |
| walker |  | 5779 | 43 | ts body lib/application.js:625 |  |  | 0.361 |
| walker |  | 5850 | 71 | ts doc lib/utils.js:249 |  |  | 0.361 |
| walker |  | 5872 | 22 | ts names test/res.download.js |  |  | 0.361 |
| walker |  | 5956 | 84 | Readme.md section #9 |  |  | 0.361 |
| walker |  | 5970 | 14 | ts names test/acceptance/auth.js |  |  | 0.361 |
| walker |  | 5984 | 14 | ts names test/acceptance/cookie-sessions.js |  |  | 0.361 |
| ns | 6222 |  | 451 | res.send() response finalization | 4.6 | 4.4 | 0.347 |
| walker |  | 6410 | 426 | ts body lib/view.js:52 |  |  | 0.347 |
| walker |  | 6482 | 72 | ts doc lib/utils.js:40 |  |  | 0.347 |
| walker |  | 6528 | 46 | ts doc lib/application.js:615 |  |  | 0.347 |
| walker |  | 6539 | 11 | History.md section #0 |  |  | 0.347 |
| ns | 6598 |  | 376 | test/ listing — every spec file | 5.1 |  | 0.427 |
| ns | 6684 |  | 86 | test/acceptance/ listing | 5.2 |  | 0.442 |
| ns | 6785 |  | 101 | examples/ listing | 5.3 |  | 0.462 |
| walker |  | 6879 | 340 | Readme.md section #34 |  |  | 0.462 |
| walker |  | 6951 | 72 | ts doc lib/utils.js:51 |  |  | 0.462 |
| walker |  | 6986 | 35 | ts names test/res.sendFile.js |  |  | 0.462 |
| walker |  | 7008 | 22 | ts names test/support/tmpl.js |  |  | 0.462 |
| ns | 7041 |  | 256 | examples/README.md — annotated example index (first half) | 5.4 |  | 0.457 |
| walker |  | 7081 | 73 | ts doc lib/utils.js:225 |  |  | 0.457 |
| walker |  | 7124 | 43 | ts names test/express.json.js |  |  | 0.457 |
| walker |  | 7167 | 43 | ts names test/express.urlencoded.js |  |  | 0.457 |
| ns | 7299 |  | 258 | examples/README.md — annotated example index (second half) | 5.5 | 5.4 | 0.453 |
| walker |  | 7324 | 157 | Readme.md section #20 |  |  | 0.453 |
| ns | 7390 |  | 91 | test/support/ and test/fixtures/ listings | 5.6 |  | 0.446 |
| walker |  | 7397 | 73 | ts body lib/utils.js:249 |  |  | 0.446 |
| walker |  | 7470 | 73 | ts names test/res.format.js |  |  | 0.446 |
| ns | 7542 |  | 152 | Test harness: env preamble, assertion helpers, template engine | 5.7 |  | 0.443 |
| walker |  | 7574 | 104 | ts body lib/utils.js:225 |  |  | 0.443 |
| ns | 7673 |  | 131 | A complete example app: examples/hello-world/index.js | 5.8 |  | 0.439 |
| walker |  | 7754 | 180 | ts body lib/utils.js:130 |  |  | 0.462 |
| ns | 7834 |  | 161 | Test-writing idiom: head of test/app.js | 5.9 |  | 0.457 |
| walker |  | 7935 | 181 | ts body lib/utils.js:162 |  |  | 0.476 |
| ns | 8120 |  | 286 | Runtime dependencies (all 28) | 6.1 |  | 0.493 |
| walker |  | 8129 | 194 | ts body lib/utils.js:194 |  |  | 0.511 |
| walker |  | 8207 | 78 | ts names test/express.static.js |  |  | 0.511 |
| walker |  | 8286 | 79 | listing of 'test/fixtures' |  |  | 0.530 |
| walker |  | 8290 | 4 | listing of 'test/fixtures/pets' |  |  | 0.530 |
| walker |  | 8295 | 5 | listing of 'test/fixtures/local_layout' |  |  | 0.530 |
| walker |  | 8302 | 7 | listing of 'test/fixtures/blog' |  |  | 0.530 |
| walker |  | 8309 | 7 | listing of 'test/fixtures/snow ☃' |  |  | 0.530 |
| walker |  | 8314 | 5 | listing of 'test/fixtures/blog/post' |  |  | 0.530 |
| walker |  | 8323 | 9 | listing of 'test/fixtures/users' |  |  | 0.530 |
| walker |  | 8333 | 10 | listing of 'test/fixtures/default_layout' |  |  | 0.530 |
| ns | 8346 |  | 226 | ci.yml — jobs and the OS/Node matrix | 6.2 |  | 0.523 |
| walker |  | 8475 | 142 | ts names test/support/utils.js |  |  | 0.527 |
| walker |  | 8481 | 6 | declaration surface of test/fixtures/% of dogs.txt |  |  | 0.527 |
| walker |  | 8488 | 7 | declaration surface of test/fixtures/name.txt |  |  | 0.527 |
| walker |  | 8495 | 7 | declaration surface of test/fixtures/todo.txt |  |  | 0.527 |
| ns | 8533 |  | 187 | .eslintrc.yml — the complete lint rule set | 6.3 |  | 0.522 |
| ns | 8807 |  | 274 | Dev dependencies (all 16) | 6.4 |  | 0.516 |
| ns | 8888 |  | 81 | package.json remainder: author, published files | 6.5 |  | 0.521 |
| walker |  | 9014 | 519 | README headline in examples/README.md |  |  | 0.544 |
| ns | 9121 |  | 233 | res.sendFile() option bag (JSDoc) | 7.1 |  | 0.539 |
| ns | 9391 |  | 270 | res.cookie() option bag (JSDoc) | 7.2 |  | 0.532 |
| ns | 9673 |  | 282 | res.status() and res.render() contracts (JSDoc) | 7.3 |  | 0.525 |
| walker |  | 9719 | 705 | YAML config at .github/workflows/ci.yml |  |  | 0.536 |
| walker |  | 9727 | 8 | declaration surface of test/fixtures/nums.txt |  |  | 0.536 |
| walker |  | 9761 | 34 | ts body test/support/utils.js:57 |  |  | 0.536 |
| walker |  | 9803 | 42 | ts doc test/support/utils.js:57 |  |  | 0.536 |
| walker |  | 9845 | 42 | ts body test/support/utils.js:45 |  |  | 0.536 |
| ns | 9971 |  | 298 | History.md — unreleased section and the 5.2.1 heading | 8.1 |  | 0.532 |
