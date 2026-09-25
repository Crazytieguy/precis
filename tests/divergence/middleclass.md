Score(3000)=0.844 I=0.893 C=0.798 ns_rows≤3K=24/47 grid(1000/1442/2080/3000/4327/6240/9000)=0.620/0.751/0.794/0.844/0.731/0.616/0.572

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 33 | 33 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 41 | 8 | Fs::DirListing { dir: performance } |  |  | 0.000 |
| ns | 65 |  | 65 | Module identity table (_VERSION / _DESCRIPTION / _URL) | 1.1 |  | 0.000 |
| walker |  | 88 | 47 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.000 |
| ns | 98 |  | 33 | Repository root listing (complete) | 1.2 |  | 0.479 |
| ns | 145 |  | 47 | README title and one-line pitch | 1.3 |  | 0.470 |
| walker |  | 146 | 58 | Fs::DirListing { dir: spec } |  |  | 0.510 |
| walker |  | 226 | 80 | Fs::DirListing { dir: rockspecs } |  |  | 0.561 |
| ns | 248 |  | 103 | The public entry point: middleclass.class + callable-module metatable | 1.4 |  | 0.468 |
| walker |  | 291 | 65 | Code::CodeKey { rung: ModuleDoc, file: middleclass.lua, decl: 0, sub: 0, line: 0 } |  |  | 0.810 |
| walker |  | 337 | 46 | Markdown::HeadingsOutline { file: UPDATING.md } |  |  | 0.811 |
| ns | 394 |  | 146 | Complete listings of spec/, performance/ and rockspecs/ | 1.5 |  | 0.835 |
| walker |  | 418 | 81 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.850 |
| ns | 442 |  | 48 | All README section headings | 1.6 |  | 0.851 |
| ns | 572 |  | 130 | DefaultMixin member roster — every default instance and static method name | 2.1 |  | 0.689 |
| ns | 699 |  | 127 | Bodies of Class:allocate and Class:new | 2.2 | 2.1 | 0.627 |
| walker |  | 734 | 316 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.643 |
| ns | 930 |  | 231 | Body of Class:subclass | 2.3 | 2.1 | 0.566 |
| walker |  | 1040 | 306 | Code::CodeKey { rung: Names, file: middleclass.lua, decl: 0, sub: 0, line: 0 } |  |  | 0.649 |
| ns | 1074 |  | 144 | Bodies of subclassed, isSubclassOf and include | 2.4 | 2.1 | 0.607 |
| walker |  | 1090 | 50 | Code::CodeKey { rung: Body, file: middleclass.lua, decl: 12, sub: 0, line: 139 } |  |  | 0.627 |
| walker |  | 1140 | 50 | Code::CodeKey { rung: Body, file: middleclass.lua, decl: 16, sub: 0, line: 172 } |  |  | 0.654 |
| walker |  | 1192 | 52 | Code::CodeKey { rung: Body, file: middleclass.lua, decl: 18, sub: 0, line: 186 } |  |  | 0.672 |
| ns | 1195 |  | 121 | Bodies of the instance-level defaults __tostring, initialize, isInstanceOf | 2.5 | 2.1 | 0.641 |
| walker |  | 1251 | 59 | Code::CodeKey { rung: Body, file: middleclass.lua, decl: 13, sub: 0, line: 144 } |  |  | 0.683 |
| walker |  | 1315 | 64 | Code::CodeKey { rung: Body, file: middleclass.lua, decl: 17, sub: 0, line: 178 } |  |  | 0.723 |
| walker |  | 1340 | 25 | Code::CodeKey { rung: Names, file: performance/run.lua, decl: 0, sub: 0, line: 0 } |  |  | 0.740 |
| ns | 1340 |  | 145 | README Quick Look, part 1: defining a class, initializer, class variable, method | 3.1 |  | 0.740 |
| walker |  | 1349 | 9 | Code::CodeKey { rung: Body, file: performance/run.lua, decl: 1, sub: 0, line: 19 } |  |  | 0.740 |
| walker |  | 1358 | 9 | Code::CodeKey { rung: Body, file: performance/run.lua, decl: 2, sub: 0, line: 37 } |  |  | 0.740 |
| ns | 1434 |  | 94 | README Quick Look, part 2: subclassing and calling the superclass initializer | 3.2 |  | 0.749 |
| walker |  | 1438 | 80 | Code::CodeKey { rung: Body, file: middleclass.lua, decl: 3, sub: 0, line: 68 } |  |  | 0.751 |
| ns | 1569 |  | 135 | README Specs and Performance tests sections — how to run everything | 3.3 |  | 0.719 |
| walker |  | 1707 | 269 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.769 |
| ns | 1737 |  | 168 | README Documentation, Installation and License bodies | 3.4 |  | 0.773 |
| walker |  | 1745 | 38 | Code::CodeKey { rung: Names, file: spec/metamethods_spec.lua, decl: 0, sub: 0, line: 0 } |  |  | 0.773 |
| walker |  | 1759 | 14 | Code::CodeKey { rung: Body, file: spec/metamethods_spec.lua, decl: 1, sub: 0, line: 3 } |  |  | 0.773 |
| walker |  | 1773 | 14 | Code::CodeKey { rung: Body, file: spec/metamethods_spec.lua, decl: 2, sub: 0, line: 7 } |  |  | 0.773 |
| walker |  | 1858 | 85 | Code::CodeKey { rung: Body, file: middleclass.lua, decl: 10, sub: 0, line: 129 } |  |  | 0.813 |
| ns | 1864 |  | 127 | CHANGELOG version heading roster (all eight releases) | 3.5 |  | 0.788 |
| walker |  | 1971 | 113 | Code::CodeKey { rung: Body, file: middleclass.lua, decl: 2, sub: 0, line: 57 } |  |  | 0.792 |
| ns | 2090 |  | 226 | CHANGELOG entries for the 4.x line | 3.6 |  | 0.764 |
| ns | 2122 |  | 32 | UPDATING.md section headings | 3.7 |  | 0.765 |
| walker |  | 2129 | 158 | Code::CodeKey { rung: Body, file: middleclass.lua, decl: 7, sub: 0, line: 109 } |  |  | 0.770 |
| walker |  | 2352 | 223 | Code::CodeKey { rung: Body, file: middleclass.lua, decl: 14, sub: 0, line: 151 } |  |  | 0.838 |
| ns | 2371 |  | 249 | UPDATING 3.x to 4.x migration body | 3.8 | 3.7 | 0.799 |
| ns | 2472 |  | 101 | Roster of every internal local function in middleclass.lua | 4.1 |  | 0.798 |
| ns | 2562 |  | 90 | _createClass: the shape of a class table | 4.2 | 4.1 | 0.780 |
| walker |  | 2578 | 226 | Code::CodeKey { rung: Body, file: middleclass.lua, decl: 1, sub: 0, line: 31 } |  |  | 0.789 |
| ns | 2629 |  | 67 | _createClass: the class metatable (__index/__tostring/__call/__newindex) | 4.3 | 4.1 | 0.779 |
| ns | 2765 |  | 136 | _createClass: the static-inheritance metatable | 4.4 | 4.2 | 0.754 |
| walker |  | 2869 | 291 | Code::CodeKey { rung: Body, file: middleclass.lua, decl: 6, sub: 0, line: 81 } |  |  | 0.825 |
| ns | 2932 |  | 167 | _includeMixin body | 4.5 | 4.1 | 0.826 |
| walker |  | 3005 | 136 | Markdown::HeadingsOutline { file: CHANGELOG.md } |  |  | 0.849 |
| ns | 3145 |  | 213 | _declareInstanceMethod and _propagateInstanceMethod bodies | 4.6 | 4.1 | 0.847 |
| ns | 3379 |  | 234 | _createIndexWrapper body | 4.7 | 4.1 | 0.847 |
| walker |  | 3384 | 379 | Plaintext::Whole { file: .travis.yml } |  |  | 0.851 |
| ns | 3515 |  | 136 | Top-level describe block for each of the eight spec files | 5.1 |  | 0.826 |
| ns | 3700 |  | 185 | The Lua-version gate that conditionally requires the 5.2/5.3 metamethod specs | 5.2 |  | 0.804 |
| ns | 4000 |  | 300 | spec/class_spec.lua in full | 5.3 | 5.1 | 0.767 |
| ns | 4227 |  | 227 | Every describe block in default_methods_spec.lua | 5.4 |  | 0.731 |
| ns | 4360 |  | 133 | Every describe block in metamethods_spec.lua | 5.5 | 5.1 | 0.713 |
| ns | 4505 |  | 145 | Every describe block in classes_spec.lua and instances_spec.lua | 5.6 |  | 0.692 |
| ns | 4771 |  | 266 | mixins_spec.lua setup: what a mixin looks like in practice | 5.7 |  | 0.673 |
| ns | 5128 |  | 357 | metamethods_spec Vector fixture, part 1: arithmetic and comparison metamethods | 5.8 | 5.5 | 0.661 |
| ns | 5328 |  | 200 | metamethods_spec Vector fixture, part 2: __pow, __mul, and the non-function __metatable/__mode fields | 5.9 | 5.8 | 0.651 |
| ns | 5768 |  | 440 | Lua 5.3 metamethod fixture: the bitwise and __gc set | 5.10 |  | 0.633 |
| ns | 6061 |  | 293 | Lua 5.2 metamethod fixture: __len, __pairs, __ipairs | 5.11 |  | 0.616 |
| ns | 6392 |  | 331 | metamethods_spec: the __index/__newindex getter-setter fixture | 5.12 | 5.5 | 0.599 |
| ns | 6684 |  | 292 | UPDATING 2.x to 3.x migration prose | 6.1 | 3.7 | 0.592 |
| ns | 7024 |  | 340 | CHANGELOG entries for the 3.x and 2.0 releases | 6.2 | 3.5 | 0.583 |
| ns | 7263 |  | 239 | Current rockspec in full (middleclass-4.1.1-0) | 6.3 |  | 0.571 |
| ns | 7409 |  | 146 | .travis.yml Lua version matrix and test script | 6.4 |  | 0.578 |
| ns | 7494 |  | 85 | Version line of each of the five older rockspecs | 6.5 |  | 0.575 |
| ns | 7644 |  | 150 | performance/time.lua in full plus run.lua's harness header | 6.6 |  | 0.568 |
| ns | 7734 |  | 90 | The six operations benchmarked by performance/run.lua | 6.7 | 6.6 | 0.562 |
| ns | 7952 |  | 218 | .travis.yml remainder: toolchain install, coverage upload, branch and mail rules | 6.8 | 6.4 | 0.573 |
| ns | 7970 |  | 18 | MIT license copyright line | 6.9 |  | 0.572 |
