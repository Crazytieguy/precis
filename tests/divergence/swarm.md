Score(3000)=0.746 I=0.917 C=0.607 ns_rows≤3K=25/66 grid(1000/1442/2080/3000/4327/6240/9000)=0.698/0.644/0.674/0.746/0.784/0.713/0.716

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 37 | 37 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 51 | 14 | Fs::DirListing { dir: assets } |  |  | 0.000 |
| walker |  | 72 | 21 | Fs::DirListing { dir: swarm } |  |  | 0.000 |
| walker |  | 82 | 10 | Fs::DirListing { dir: swarm/repl } |  |  | 0.000 |
| ns | 87 |  | 87 | README title + deprecation callout | 1.1 |  | 0.000 |
| ns | 124 |  | 37 | Repository root listing (complete) | 1.2 |  | 0.546 |
| walker |  | 192 | 110 | Markdown::ReadmeHeadline { file: README.md } |  |  | 1.000 |
| ns | 211 |  | 87 | README Overview: agents and handoffs | 1.3 |  | 0.922 |
| walker |  | 219 | 27 | Fs::DirListing { dir: tests } |  |  | 0.922 |
| ns | 242 |  | 31 | `swarm/` package listing (complete, incl. `repl/`) | 1.4 |  | 0.928 |
| ns | 301 |  | 59 | Public exports of `swarm` and `swarm.repl` | 1.5 | 1.4 | 0.835 |
| ns | 399 |  | 98 | README: expressive power, and the statelessness NOTE | 1.6 | 1.3 | 0.790 |
| ns | 474 |  | 75 | `examples/` and `tests/` directory listings (complete) | 1.7 |  | 0.656 |
| walker |  | 560 | 341 | Fs::DirListing { dir: logs } |  |  | 0.656 |
| walker |  | 608 | 48 | Fs::DirListing { dir: examples } |  |  | 0.842 |
| walker |  | 625 | 17 | Fs::DirListing { dir: examples/weather_agent } |  |  | 0.842 |
| walker |  | 645 | 20 | Fs::DirListing { dir: examples/personal_shopper } |  |  | 0.843 |
| walker |  | 668 | 23 | Fs::DirListing { dir: examples/triage_agent } |  |  | 0.844 |
| ns | 684 |  | 210 | README section-heading roster (all remaining headings) | 1.8 | 1.1 | 0.683 |
| ns | 815 |  | 131 | `Agent` model: every field with its default | 2.1 |  | 0.631 |
| walker |  | 859 | 191 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.780 |
| ns | 910 |  | 95 | `Swarm` class: complete method roster | 2.2 |  | 0.738 |
| walker |  | 958 | 99 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.738 |
| walker |  | 984 | 26 | Fs::DirListing { dir: examples/airline } |  |  | 0.739 |
| walker |  | 987 | 3 | Fs::DirListing { dir: examples/airline/data } |  |  | 0.739 |
| ns | 998 |  | 88 | `Response` and `Result` models: every field | 2.3 | 2.1 | 0.697 |
| walker |  | 999 | 12 | Fs::DirListing { dir: examples/airline/data/routines } |  |  | 0.698 |
| walker |  | 1015 | 16 | Fs::DirListing { dir: examples/airline/configs } |  |  | 0.699 |
| walker |  | 1028 | 13 | Code::CodeKey { rung: Names, file: swarm/repl/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.701 |
| walker |  | 1062 | 34 | Fs::DirListing { dir: examples/basic } |  |  | 0.704 |
| ns | 1070 |  | 72 | `swarm/util.py`: complete function roster | 2.4 |  | 0.687 |
| walker |  | 1096 | 34 | Fs::DirListing { dir: examples/customer_service_streaming } |  |  | 0.689 |
| walker |  | 1123 | 27 | Fs::DirListing { dir: examples/customer_service_streaming/configs } |  |  | 0.689 |
| walker |  | 1135 | 12 | Fs::DirListing { dir: examples/customer_service_streaming/configs/tools } |  |  | 0.689 |
| ns | 1139 |  | 69 | `swarm/repl/repl.py`: complete function roster | 2.5 |  | 0.670 |
| walker |  | 1169 | 34 | Fs::DirListing { dir: examples/customer_service_streaming/src } |  |  | 0.670 |
| walker |  | 1188 | 19 | Fs::DirListing { dir: examples/customer_service_streaming/src/swarm } |  |  | 0.671 |
| walker |  | 1202 | 14 | Fs::DirListing { dir: examples/customer_service_streaming/src/swarm/engines } |  |  | 0.671 |
| ns | 1245 |  | 106 | `Swarm.run()` full signature with defaults | 2.6 | 2.2 | 0.633 |
| walker |  | 1248 | 46 | Code::CodeKey { rung: Names, file: swarm/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.671 |
| walker |  | 1323 | 75 | Markdown::Section { file: README.md, section_index: 12, keeps_default_concavity: false } |  |  | 0.671 |
| walker |  | 1326 | 3 | Fs::DirListing { dir: examples/customer_service } |  |  | 0.672 |
| walker |  | 1329 | 3 | Fs::DirListing { dir: examples/customer_service_lite } |  |  | 0.672 |
| ns | 1337 |  | 92 | `run_and_stream()` full signature | 2.7 | 2.2 | 0.641 |
| walker |  | 1371 | 42 | Fs::DirListing { dir: examples/support_bot } |  |  | 0.643 |
| walker |  | 1453 | 82 | Markdown::Section { file: README.md, section_index: 13, keeps_default_concavity: false } |  |  | 0.645 |
| ns | 1482 |  | 145 | Internal `Swarm` method signatures: completion + tool dispatch | 2.8 | 2.2 | 0.599 |
| walker |  | 1543 | 90 | Plaintext::DeclSurface { file: setup.cfg } |  |  | 0.600 |
| ns | 1574 |  | 92 | `swarm/types.py` imports: pydantic + reused OpenAI types | 2.9 |  | 0.579 |
| walker |  | 1606 | 63 | Code::CodeKey { rung: Names, file: swarm/types.py, decl: 0, sub: 0, line: 0 } |  |  | 0.587 |
| walker |  | 1639 | 33 | Code::CodeKey { rung: Decl, file: swarm/types.py, decl: 3, sub: 0, line: 23 } |  |  | 0.601 |
| walker |  | 1674 | 35 | Code::CodeKey { rung: Decl, file: swarm/types.py, decl: 4, sub: 0, line: 29 } |  |  | 0.626 |
| ns | 1748 |  | 174 | `swarm/core.py` import block | 2.10 |  | 0.581 |
| walker |  | 1762 | 88 | Code::CodeKey { rung: Decl, file: swarm/types.py, decl: 2, sub: 0, line: 14 } |  |  | 0.625 |
| walker |  | 1834 | 72 | Code::CodeKey { rung: Names, file: swarm/util.py, decl: 0, sub: 0, line: 0 } |  |  | 0.643 |
| walker |  | 1895 | 61 | Code::CodeKey { rung: Body, file: swarm/util.py, decl: 2, sub: 0, line: 13 } |  |  | 0.643 |
| ns | 1914 |  | 166 | `client.run()` semantics and the five-step loop | 3.1 |  | 0.626 |
| walker |  | 1932 | 37 | Code::CodeKey { rung: Names, file: swarm/core.py, decl: 0, sub: 0, line: 0 } |  |  | 0.629 |
| walker |  | 2012 | 80 | Code::CodeKey { rung: Decl, file: swarm/core.py, decl: 2, sub: 0, line: 26 } |  |  | 0.666 |
| ns | 2068 |  | 154 | `run()` arguments table, part 1 of 2 | 3.2 |  | 0.655 |
| walker |  | 2075 | 63 | Code::CodeKey { rung: Decl, file: swarm/core.py, decl: 6, sub: 0, line: 89 } |  |  | 0.674 |
| walker |  | 2149 | 74 | Code::CodeKey { rung: Decl, file: swarm/core.py, decl: 4, sub: 0, line: 32 } |  |  | 0.719 |
| walker |  | 2241 | 92 | Code::CodeKey { rung: Decl, file: swarm/core.py, decl: 7, sub: 0, line: 139 } |  |  | 0.757 |
| ns | 2242 |  | 174 | `run()` arguments table, part 2 of 2 (completes the table) | 3.3 | 3.2 | 0.746 |
| walker |  | 2347 | 106 | Code::CodeKey { rung: Decl, file: swarm/core.py, decl: 8, sub: 0, line: 231 } |  |  | 0.787 |
| walker |  | 2374 | 27 | Code::CodeKey { rung: Body, file: swarm/core.py, decl: 3, sub: 0, line: 27 } |  |  | 0.787 |
| ns | 2398 |  | 156 | `Response` fields table (complete) | 3.4 |  | 0.775 |
| walker |  | 2461 | 87 | Code::CodeKey { rung: Doc, file: swarm/types.py, decl: 4, sub: 0, line: 29 } |  |  | 0.779 |
| ns | 2603 |  | 205 | `Agent` fields table (the README's documented subset) | 3.5 |  | 0.763 |
| walker |  | 2644 | 183 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.766 |
| walker |  | 2734 | 90 | Code::CodeKey { rung: Body, file: swarm/util.py, decl: 1, sub: 0, line: 5 } |  |  | 0.766 |
| ns | 2766 |  | 163 | Function/tool rules: return values, context, errors, ordering | 3.6 |  | 0.753 |
| walker |  | 2886 | 152 | Markdown::Section { file: README.md, section_index: 14, keeps_default_concavity: false } |  |  | 0.753 |
| ns | 2897 |  | 131 | Handoffs and `Result`: the documented rules | 3.7 |  | 0.745 |
| ns | 3018 |  | 121 | Function-schema conversion rules | 3.8 |  | 0.733 |
| walker |  | 3088 | 202 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.735 |
| ns | 3172 |  | 154 | Streaming: the two Swarm-specific event types | 3.9 |  | 0.726 |
| walker |  | 3182 | 94 | Code::CodeKey { rung: Body, file: swarm/util.py, decl: 3, sub: 0, line: 21 } |  |  | 0.727 |
| walker |  | 3290 | 108 | Code::CodeKey { rung: Doc, file: swarm/util.py, decl: 4, sub: 0, line: 31 } |  |  | 0.727 |
| walker |  | 3301 | 11 | Fs::DirListing { dir: tests/test_runs } |  |  | 0.727 |
| walker |  | 3344 | 43 | Code::CodeKey { rung: Names, file: swarm/repl/repl.py, decl: 0, sub: 0, line: 0 } |  |  | 0.733 |
| ns | 3369 |  | 197 | Examples index: what each example directory demonstrates | 3.10 |  | 0.740 |
| walker |  | 3370 | 26 | Code::CodeKey { rung: Decl, file: swarm/repl/repl.py, decl: 3, sub: 0, line: 60 } |  |  | 0.750 |
| ns | 3495 |  | 126 | Returning from `run()` and resuming a conversation | 3.11 |  | 0.749 |
| ns | 3670 |  | 175 | `Agent` definition and `instructions` semantics | 3.12 |  | 0.743 |
| walker |  | 3771 | 401 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.743 |
| ns | 3847 |  | 177 | Install, client construction, and the `run_demo_loop` util | 3.13 | 1.1 | 0.752 |
| walker |  | 3949 | 178 | Plaintext::Whole { file: setup.cfg } |  |  | 0.754 |
| walker |  | 3953 | 4 | Fs::DirListing { dir: examples/customer_service_streaming/configs/assistants } |  |  | 0.754 |
| walker |  | 3957 | 4 | Fs::DirListing { dir: examples/customer_service_streaming/src/runs } |  |  | 0.754 |
| walker |  | 3961 | 4 | Fs::DirListing { dir: examples/customer_service_streaming/src/tasks } |  |  | 0.754 |
| ns | 3992 |  | 145 | "Why Swarm" positioning and the evaluations stance | 3.14 |  | 0.752 |
| ns | 4066 |  | 74 | Listings for the three simple examples (complete) | 4.1 |  | 0.761 |
| walker |  | 4296 | 335 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.795 |
| walker |  | 4303 | 7 | Fs::DirListing { dir: examples/customer_service_streaming/logs } |  |  | 0.795 |
| walker |  | 4308 | 5 | Fs::DirListing { dir: examples/customer_service_streaming/src/evals } |  |  | 0.795 |
| ns | 4322 |  | 256 | Purpose line of all six example READMEs | 4.2 |  | 0.784 |
| ns | 4445 |  | 123 | `basic/bare_minimum.py` in full | 4.3 |  | 0.763 |
| walker |  | 4483 | 175 | Code::CodeKey { rung: Body, file: swarm/core.py, decl: 5, sub: 0, line: 71 } |  |  | 0.765 |
| walker |  | 4487 | 4 | Fs::DirListing { dir: examples/airline/data/routines/baggage } |  |  | 0.765 |
| walker |  | 4491 | 4 | Fs::DirListing { dir: examples/airline/data/routines/flight_modification } |  |  | 0.765 |
| walker |  | 4495 | 4 | Fs::DirListing { dir: examples/customer_service_streaming/configs/assistants/user_interface } |  |  | 0.765 |
| walker |  | 4506 | 11 | Fs::DirListing { dir: examples/customer_service_streaming/tests } |  |  | 0.765 |
| walker |  | 4513 | 7 | Fs::DirListing { dir: examples/customer_service_streaming/tests/test_runs } |  |  | 0.765 |
| ns | 4733 |  | 288 | `triage_agent/agents.py`: three agents and their wiring | 4.4 |  | 0.735 |
| ns | 4845 |  | 112 | `airline` example: complete directory tree | 4.5 |  | 0.724 |
| ns | 4907 |  | 62 | `support_bot` and `personal_shopper` listings (complete) | 4.6 |  | 0.732 |
| ns | 4947 |  | 40 | The three `customer_service*` directories: what is actually there | 4.7 |  | 0.738 |
| walker |  | 4948 | 435 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.779 |
| ns | 5149 |  | 202 | `weather_agent/agents.py`: a complete function-calling agent | 4.8 |  | 0.757 |
| walker |  | 5306 | 358 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.775 |
| ns | 5335 |  | 186 | `basic/context_variables.py`: callable instructions + injected context | 4.9 |  | 0.756 |
| ns | 5576 |  | 241 | `airline` agents: the five agents and their instruction sources | 4.10 |  | 0.735 |
| walker |  | 5745 | 439 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.750 |
| ns | 5765 |  | 189 | `airline` agents: imports and the five transfer functions | 4.11 | 4.10 | 0.733 |
| ns | 5958 |  | 193 | `airline` agents: per-agent tool lists (the handoff graph) | 4.12 | 4.10 | 0.713 |
| ns | 6043 |  | 85 | `airline/configs/tools.py`: complete tool roster | 4.13 |  | 0.706 |
| ns | 6286 |  | 243 | `run()` body: state setup and the completion half of the loop | 5.1 | 2.6 | 0.687 |
| walker |  | 6301 | 556 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.694 |
| ns | 6479 |  | 193 | `run()` body: tool dispatch, agent switch, and the returned `Response` | 5.2 | 5.1 | 0.679 |
| ns | 6572 |  | 93 | `run()` body: the `stream=True` delegation branch | 5.3 | 2.6 | 0.672 |
| ns | 6783 |  | 211 | `handle_tool_calls` body: dispatch table and the missing-tool path | 5.4 | 2.8 | 0.658 |
| walker |  | 6849 | 548 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.666 |
| ns | 7054 |  | 271 | `handle_tool_calls` body: context injection and `Result` merging | 5.5 | 5.4 | 0.649 |
| ns | 7265 |  | 211 | `get_chat_completion` body: instructions, tool schemas, context hiding | 5.6 | 2.8 | 0.638 |
| walker |  | 7372 | 523 | Markdown::Section { file: README.md, section_index: 10, keeps_default_concavity: false } |  |  | 0.650 |
| ns | 7390 |  | 125 | `get_chat_completion` body: the Chat Completions request | 5.7 | 5.6 | 0.643 |
| ns | 7565 |  | 175 | `handle_function_result` body: the return-value coercion rules | 5.8 | 2.8 | 0.651 |
| walker |  | 7584 | 212 | Markdown::Section { file: README.md, section_index: 11, keeps_default_concavity: false } |  |  | 0.658 |
| walker |  | 7603 | 19 | Fs::DirListing { dir: examples/airline/evals } |  |  | 0.666 |
| walker |  | 7611 | 8 | Fs::DirListing { dir: examples/customer_service_streaming/configs/tools/query_docs } |  |  | 0.666 |
| walker |  | 7619 | 8 | Fs::DirListing { dir: examples/customer_service_streaming/configs/tools/send_email } |  |  | 0.666 |
| walker |  | 7627 | 8 | Fs::DirListing { dir: examples/customer_service_streaming/configs/tools/submit_ticket } |  |  | 0.666 |
| walker |  | 7866 | 239 | Code::CodeKey { rung: Body, file: swarm/repl/repl.py, decl: 3, sub: 0, line: 60 } |  |  | 0.668 |
| walker |  | 7879 | 13 | Fs::DirListing { dir: examples/airline/evals/eval_cases } |  |  | 0.673 |
| ns | 7894 |  | 329 | `function_to_json` body: type map, signature inspection, `required` | 5.9 | 2.4 | 0.653 |
| ns | 8003 |  | 109 | `function_to_json` body: the emitted schema shape | 5.10 | 5.9 | 0.646 |
| walker |  | 8133 | 254 | Code::CodeKey { rung: Body, file: swarm/repl/repl.py, decl: 2, sub: 0, line: 37 } |  |  | 0.646 |
| walker |  | 8148 | 15 | Fs::DirListing { dir: examples/airline/evals/eval_results } |  |  | 0.650 |
| ns | 8173 |  | 170 | `swarm/util.py`: the streaming merge helpers | 5.11 | 2.4 | 0.655 |
| ns | 8412 |  | 239 | `run_demo_loop` body: the reference conversation loop | 5.12 | 2.5 | 0.666 |
| walker |  | 8484 | 336 | Code::CodeKey { rung: Body, file: swarm/core.py, decl: 4, sub: 0, line: 32 } |  |  | 0.697 |
| ns | 8646 |  | 234 | `run_and_stream` body: the streaming protocol, with elisions marked | 5.13 | 2.7 | 0.683 |
| ns | 8875 |  | 229 | `tests/test_core.py`: complete test roster and the shared fixture | 6.1 |  | 0.673 |
| walker |  | 8925 | 441 | Code::CodeKey { rung: Body, file: swarm/util.py, decl: 4, sub: 0, line: 31 } |  |  | 0.716 |
| ns | 9032 |  | 157 | `tests/mock_client.py`: the fake OpenAI client | 6.2 |  | 0.711 |
| ns | 9134 |  | 102 | `tests/test_util.py`: both schema-conversion tests | 6.3 |  | 0.707 |
| walker |  | 9267 | 342 | Code::CodeKey { rung: Body, file: swarm/repl/repl.py, decl: 1, sub: 0, line: 6 } |  |  | 0.707 |
| walker |  | 9311 | 44 | Fs::DirListing { dir: examples/customer_service_lite/logs } |  |  | 0.707 |
| ns | 9340 |  | 206 | `setup.cfg`: package metadata and the complete dependency list | 6.4 |  | 0.713 |
| ns | 9515 |  | 175 | Build backend and formatting toolchain | 6.5 |  | 0.706 |
| ns | 9649 |  | 134 | `customer_service_streaming/src` and `configs`: complete listings | 7.1 |  | 0.714 |
| walker |  | 9793 | 482 | Code::CodeKey { rung: Body, file: swarm/core.py, decl: 6, sub: 0, line: 89 } |  |  | 0.750 |
| walker |  | 9848 | 55 | Fs::DirListing { dir: examples/customer_service/logs } |  |  | 0.750 |
| ns | 9869 |  | 220 | The legacy example's own `Swarm` class and its config knobs | 7.2 | 7.1 | 0.740 |
| ns | 9894 |  | 25 | Remaining asset and log directories | 7.3 |  | 0.741 |
| walker |  | 9993 | 145 | Code::CodeKey { rung: Body, file: swarm/core.py, decl: 8, sub: 0, line: 231 } |  |  | 0.750 |
