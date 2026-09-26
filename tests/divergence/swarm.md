Score(3000)=0.739 I=0.901 C=0.606 ns_rows≤3K=25/66 grid(1000/1442/2080/3000/4327/6240/9000)=0.698/0.613/0.698/0.739/0.820/0.713/0.707

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 37 | 37 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 58 | 21 | Fs::DirListing { dir: swarm } |  |  | 0.000 |
| walker |  | 68 | 10 | Fs::DirListing { dir: swarm/repl } |  |  | 0.000 |
| ns | 87 |  | 87 | README title + deprecation callout | 1.1 |  | 0.000 |
| ns | 124 |  | 37 | Repository root listing (complete) | 1.2 |  | 0.545 |
| walker |  | 178 | 110 | Markdown::ReadmeHeadline { file: README.md } |  |  | 1.000 |
| walker |  | 192 | 14 | Fs::DirListing { dir: assets } |  |  | 1.000 |
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
| ns | 684 |  | 210 | README section-heading roster (all remaining headings) | 1.8 | 1.1 | 0.681 |
| ns | 815 |  | 131 | `Agent` model: every field with its default | 2.1 |  | 0.630 |
| walker |  | 836 | 191 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.779 |
| ns | 910 |  | 95 | `Swarm` class: complete method roster | 2.2 |  | 0.736 |
| walker |  | 935 | 99 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.737 |
| walker |  | 958 | 23 | Fs::DirListing { dir: examples/triage_agent } |  |  | 0.738 |
| walker |  | 984 | 26 | Fs::DirListing { dir: examples/airline } |  |  | 0.739 |
| walker |  | 998 | 14 | Fs::DirListing { dir: examples/airline/data/routines } |  |  | 0.698 |
| ns | 998 |  | 88 | `Response` and `Result` models: every field | 2.3 | 2.1 | 0.698 |
| walker |  | 1002 | 4 | Fs::DirListing { dir: examples/airline/data/routines/baggage } |  |  | 0.698 |
| walker |  | 1006 | 4 | Fs::DirListing { dir: examples/airline/data/routines/flight_modification } |  |  | 0.698 |
| walker |  | 1022 | 16 | Fs::DirListing { dir: examples/airline/configs } |  |  | 0.699 |
| walker |  | 1041 | 19 | Fs::DirListing { dir: examples/airline/evals } |  |  | 0.701 |
| walker |  | 1066 | 25 | Code::CodeKey { rung: Names, file: swarm/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.709 |
| ns | 1070 |  | 72 | `swarm/util.py`: complete function roster | 2.4 |  | 0.692 |
| ns | 1139 |  | 69 | `swarm/repl/repl.py`: complete function roster | 2.5 |  | 0.673 |
| walker |  | 1141 | 75 | Markdown::Section { file: README.md, section_index: 12, keeps_default_concavity: false } |  |  | 0.673 |
| walker |  | 1175 | 34 | Fs::DirListing { dir: examples/basic } |  |  | 0.676 |
| walker |  | 1209 | 34 | Fs::DirListing { dir: examples/customer_service_streaming } |  |  | 0.677 |
| walker |  | 1236 | 27 | Fs::DirListing { dir: examples/customer_service_streaming/configs } |  |  | 0.677 |
| ns | 1245 |  | 106 | `Swarm.run()` full signature with defaults | 2.6 | 2.2 | 0.639 |
| walker |  | 1248 | 12 | Fs::DirListing { dir: examples/customer_service_streaming/configs/tools } |  |  | 0.639 |
| walker |  | 1256 | 8 | Fs::DirListing { dir: examples/customer_service_streaming/configs/tools/query_docs } |  |  | 0.639 |
| walker |  | 1264 | 8 | Fs::DirListing { dir: examples/customer_service_streaming/configs/tools/send_email } |  |  | 0.639 |
| walker |  | 1272 | 8 | Fs::DirListing { dir: examples/customer_service_streaming/configs/tools/submit_ticket } |  |  | 0.639 |
| walker |  | 1306 | 34 | Fs::DirListing { dir: examples/customer_service_streaming/src } |  |  | 0.640 |
| walker |  | 1310 | 4 | Fs::DirListing { dir: examples/customer_service_streaming/src/runs } |  |  | 0.640 |
| walker |  | 1314 | 4 | Fs::DirListing { dir: examples/customer_service_streaming/src/tasks } |  |  | 0.640 |
| walker |  | 1319 | 5 | Fs::DirListing { dir: examples/customer_service_streaming/src/evals } |  |  | 0.640 |
| ns | 1337 |  | 92 | `run_and_stream()` full signature | 2.7 | 2.2 | 0.610 |
| walker |  | 1338 | 19 | Fs::DirListing { dir: examples/customer_service_streaming/src/swarm } |  |  | 0.610 |
| walker |  | 1352 | 14 | Fs::DirListing { dir: examples/customer_service_streaming/src/swarm/engines } |  |  | 0.611 |
| walker |  | 1434 | 82 | Markdown::Section { file: README.md, section_index: 13, keeps_default_concavity: false } |  |  | 0.613 |
| walker |  | 1476 | 42 | Fs::DirListing { dir: examples/support_bot } |  |  | 0.615 |
| ns | 1482 |  | 145 | Internal `Swarm` method signatures: completion + tool dispatch | 2.8 | 2.2 | 0.572 |
| walker |  | 1513 | 37 | Code::CodeKey { rung: Names, file: swarm/core.py, decl: 0, sub: 0, line: 0 } |  |  | 0.575 |
| ns | 1574 |  | 92 | `swarm/types.py` imports: pydantic + reused OpenAI types | 2.9 |  | 0.555 |
| walker |  | 1636 | 123 | Code::CodeKey { rung: Decl, file: swarm/core.py, decl: 2, sub: 0, line: 26 } |  |  | 0.606 |
| walker |  | 1688 | 52 | Code::CodeKey { rung: Decl, file: swarm/core.py, decl: 6, sub: 0, line: 89 } |  |  | 0.631 |
| ns | 1748 |  | 174 | `swarm/core.py` import block | 2.10 |  | 0.585 |
| walker |  | 1749 | 61 | Code::CodeKey { rung: Decl, file: swarm/core.py, decl: 4, sub: 0, line: 32 } |  |  | 0.630 |
| walker |  | 1833 | 84 | Code::CodeKey { rung: Decl, file: swarm/core.py, decl: 7, sub: 0, line: 139 } |  |  | 0.672 |
| ns | 1914 |  | 166 | `client.run()` semantics and the five-step loop | 3.1 |  | 0.655 |
| walker |  | 1928 | 95 | Code::CodeKey { rung: Decl, file: swarm/core.py, decl: 8, sub: 0, line: 231 } |  |  | 0.699 |
| walker |  | 2018 | 90 | Plaintext::DeclSurface { file: setup.cfg } |  |  | 0.699 |
| ns | 2068 |  | 154 | `run()` arguments table, part 1 of 2 | 3.2 |  | 0.688 |
| walker |  | 2071 | 53 | Code::CodeKey { rung: Names, file: swarm/repl/repl.py, decl: 0, sub: 0, line: 0 } |  |  | 0.698 |
| walker |  | 2087 | 16 | Code::CodeKey { rung: Decl, file: swarm/repl/repl.py, decl: 3, sub: 0, line: 60 } |  |  | 0.708 |
| ns | 2242 |  | 174 | `run()` arguments table, part 2 of 2 (completes the table) | 3.3 | 3.2 | 0.697 |
| walker |  | 2246 | 159 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.700 |
| walker |  | 2318 | 72 | Code::CodeKey { rung: Names, file: swarm/util.py, decl: 0, sub: 0, line: 0 } |  |  | 0.716 |
| walker |  | 2381 | 63 | Code::CodeKey { rung: Names, file: swarm/types.py, decl: 0, sub: 0, line: 0 } |  |  | 0.722 |
| ns | 2398 |  | 156 | `Response` fields table (complete) | 3.4 |  | 0.711 |
| walker |  | 2414 | 33 | Code::CodeKey { rung: Decl, file: swarm/types.py, decl: 3, sub: 0, line: 23 } |  |  | 0.722 |
| walker |  | 2449 | 35 | Code::CodeKey { rung: Decl, file: swarm/types.py, decl: 4, sub: 0, line: 29 } |  |  | 0.740 |
| walker |  | 2537 | 88 | Code::CodeKey { rung: Decl, file: swarm/types.py, decl: 2, sub: 0, line: 14 } |  |  | 0.775 |
| walker |  | 2564 | 27 | Code::CodeKey { rung: Body, file: swarm/core.py, decl: 3, sub: 0, line: 27 } |  |  | 0.775 |
| ns | 2603 |  | 205 | `Agent` fields table (the README's documented subset) | 3.5 |  | 0.760 |
| ns | 2766 |  | 163 | Function/tool rules: return values, context, errors, ordering | 3.6 |  | 0.747 |
| walker |  | 2852 | 288 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.747 |
| ns | 2897 |  | 131 | Handoffs and `Result`: the documented rules | 3.7 |  | 0.739 |
| walker |  | 3004 | 152 | Markdown::Section { file: README.md, section_index: 14, keeps_default_concavity: false } |  |  | 0.739 |
| ns | 3018 |  | 121 | Function-schema conversion rules | 3.8 |  | 0.727 |
| ns | 3172 |  | 154 | Streaming: the two Swarm-specific event types | 3.9 |  | 0.718 |
| walker |  | 3206 | 202 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.720 |
| walker |  | 3219 | 13 | Code::CodeKey { rung: Names, file: swarm/repl/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.726 |
| ns | 3369 |  | 197 | Examples index: what each example directory demonstrates | 3.10 |  | 0.733 |
| walker |  | 3397 | 178 | Plaintext::Whole { file: setup.cfg } |  |  | 0.734 |
| walker |  | 3408 | 11 | Fs::DirListing { dir: tests/test_runs } |  |  | 0.734 |
| ns | 3481 |  | 112 | Returning from `run()` and resuming a conversation | 3.11 |  | 0.733 |
| ns | 3656 |  | 175 | `Agent` definition and `instructions` semantics | 3.12 |  | 0.727 |
| walker |  | 3743 | 335 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.760 |
| walker |  | 3750 | 7 | Fs::DirListing { dir: examples/customer_service_streaming/logs } |  |  | 0.760 |
| ns | 3833 |  | 177 | Install, client construction, and the `run_demo_loop` util | 3.13 | 1.1 | 0.769 |
| walker |  | 3837 | 87 | Code::CodeKey { rung: Doc, file: swarm/types.py, decl: 4, sub: 0, line: 29 } |  |  | 0.772 |
| walker |  | 3848 | 11 | Fs::DirListing { dir: examples/customer_service_streaming/tests } |  |  | 0.772 |
| ns | 3978 |  | 145 | "Why Swarm" positioning and the evaluations stance | 3.14 |  | 0.774 |
| ns | 4052 |  | 74 | Listings for the three simple examples (complete) | 4.1 |  | 0.782 |
| walker |  | 4283 | 435 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.832 |
| ns | 4308 |  | 256 | Purpose line of all six example READMEs | 4.2 |  | 0.820 |
| ns | 4431 |  | 123 | `basic/bare_minimum.py` in full | 4.3 |  | 0.798 |
| walker |  | 4627 | 344 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.820 |
| ns | 4719 |  | 288 | `triage_agent/agents.py`: three agents and their wiring | 4.4 |  | 0.788 |
| ns | 4830 |  | 111 | `airline` example: complete directory tree | 4.5 |  | 0.784 |
| ns | 4892 |  | 62 | `support_bot` and `personal_shopper` listings (complete) | 4.6 |  | 0.789 |
| ns | 4930 |  | 38 | The three `customer_service*` directories: what is actually there | 4.7 |  | 0.787 |
| walker |  | 5066 | 439 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.803 |
| walker |  | 5073 | 7 | Fs::DirListing { dir: examples/customer_service_streaming/tests/test_runs } |  |  | 0.803 |
| ns | 5132 |  | 202 | `weather_agent/agents.py`: a complete function-calling agent | 4.8 |  | 0.780 |
| walker |  | 5134 | 61 | Code::CodeKey { rung: Body, file: swarm/util.py, decl: 2, sub: 0, line: 13 } |  |  | 0.781 |
| ns | 5318 |  | 186 | `basic/context_variables.py`: callable instructions + injected context | 4.9 |  | 0.761 |
| ns | 5559 |  | 241 | `airline` agents: the five agents and their instruction sources | 4.10 |  | 0.740 |
| walker |  | 5690 | 556 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.748 |
| ns | 5748 |  | 189 | `airline` agents: imports and the five transfer functions | 4.11 | 4.10 | 0.730 |
| ns | 5941 |  | 193 | `airline` agents: per-agent tool lists (the handoff graph) | 4.12 | 4.10 | 0.710 |
| ns | 6026 |  | 85 | `airline/configs/tools.py`: complete tool roster | 4.13 |  | 0.704 |
| walker |  | 6238 | 548 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.713 |
| ns | 6269 |  | 243 | `run()` body: state setup and the completion half of the loop | 5.1 | 2.6 | 0.694 |
| ns | 6462 |  | 193 | `run()` body: tool dispatch, agent switch, and the returned `Response` | 5.2 | 5.1 | 0.679 |
| ns | 6555 |  | 93 | `run()` body: the `stream=True` delegation branch | 5.3 | 2.6 | 0.671 |
| walker |  | 6761 | 523 | Markdown::Section { file: README.md, section_index: 10, keeps_default_concavity: false } |  |  | 0.685 |
| ns | 6766 |  | 211 | `handle_tool_calls` body: dispatch table and the missing-tool path | 5.4 | 2.8 | 0.671 |
| walker |  | 6973 | 212 | Markdown::Section { file: README.md, section_index: 11, keeps_default_concavity: false } |  |  | 0.678 |
| walker |  | 6979 | 6 | Fs::DirListing { dir: examples/customer_service_streaming/configs/assistants/user_interface } |  |  | 0.679 |
| ns | 7037 |  | 271 | `handle_tool_calls` body: context injection and `Result` merging | 5.5 | 5.4 | 0.661 |
| walker |  | 7087 | 108 | Code::CodeKey { rung: Doc, file: swarm/util.py, decl: 4, sub: 0, line: 31 } |  |  | 0.661 |
| walker |  | 7100 | 13 | Fs::DirListing { dir: examples/airline/evals/eval_cases } |  |  | 0.666 |
| ns | 7248 |  | 211 | `get_chat_completion` body: instructions, tool schemas, context hiding | 5.6 | 2.8 | 0.655 |
| walker |  | 7275 | 175 | Code::CodeKey { rung: Body, file: swarm/core.py, decl: 5, sub: 0, line: 71 } |  |  | 0.656 |
| walker |  | 7290 | 15 | Fs::DirListing { dir: examples/airline/evals/eval_results } |  |  | 0.661 |
| ns | 7373 |  | 125 | `get_chat_completion` body: the Chat Completions request | 5.7 | 5.6 | 0.653 |
| walker |  | 7380 | 90 | Code::CodeKey { rung: Body, file: swarm/util.py, decl: 1, sub: 0, line: 5 } |  |  | 0.653 |
| walker |  | 7474 | 94 | Code::CodeKey { rung: Body, file: swarm/util.py, decl: 3, sub: 0, line: 21 } |  |  | 0.654 |
| walker |  | 7520 | 46 | Fs::DirListing { dir: examples/customer_service_lite/logs } |  |  | 0.656 |
| ns | 7548 |  | 175 | `handle_function_result` body: the return-value coercion rules | 5.8 | 2.8 | 0.664 |
| walker |  | 7577 | 57 | Fs::DirListing { dir: examples/customer_service/logs } |  |  | 0.667 |
| walker |  | 7816 | 239 | Code::CodeKey { rung: Body, file: swarm/repl/repl.py, decl: 3, sub: 0, line: 60 } |  |  | 0.669 |
| ns | 7877 |  | 329 | `function_to_json` body: type map, signature inspection, `required` | 5.9 | 2.4 | 0.649 |
| ns | 7986 |  | 109 | `function_to_json` body: the emitted schema shape | 5.10 | 5.9 | 0.642 |
| walker |  | 8152 | 336 | Code::CodeKey { rung: Body, file: swarm/core.py, decl: 4, sub: 0, line: 32 } |  |  | 0.675 |
| ns | 8156 |  | 170 | `swarm/util.py`: the streaming merge helpers | 5.11 | 2.4 | 0.679 |
| ns | 8395 |  | 239 | `run_demo_loop` body: the reference conversation loop | 5.12 | 2.5 | 0.688 |
| walker |  | 8406 | 254 | Code::CodeKey { rung: Body, file: swarm/repl/repl.py, decl: 2, sub: 0, line: 37 } |  |  | 0.688 |
| walker |  | 8454 | 48 | Plaintext::Whole { file: examples/support_bot/Makefile } |  |  | 0.688 |
| walker |  | 8463 | 9 | Plaintext::DeclSurface { file: examples/support_bot/requirements.txt } |  |  | 0.688 |
| ns | 8629 |  | 234 | `run_and_stream` body: the streaming protocol, with elisions marked | 5.13 | 2.7 | 0.675 |
| ns | 8858 |  | 229 | `tests/test_core.py`: complete test roster and the shared fixture | 6.1 |  | 0.665 |
| walker |  | 8945 | 482 | Code::CodeKey { rung: Body, file: swarm/core.py, decl: 6, sub: 0, line: 89 } |  |  | 0.707 |
| ns | 9015 |  | 157 | `tests/mock_client.py`: the fake OpenAI client | 6.2 |  | 0.701 |
| walker |  | 9022 | 77 | Plaintext::Whole { file: examples/support_bot/docker-compose.yaml } |  |  | 0.701 |
| ns | 9117 |  | 102 | `tests/test_util.py`: both schema-conversion tests | 6.3 |  | 0.698 |
| ns | 9323 |  | 206 | `setup.cfg`: package metadata and the complete dependency list | 6.4 |  | 0.704 |
| walker |  | 9463 | 441 | Code::CodeKey { rung: Body, file: swarm/util.py, decl: 4, sub: 0, line: 31 } |  |  | 0.744 |
| ns | 9498 |  | 175 | Build backend and formatting toolchain | 6.5 |  | 0.736 |
| walker |  | 9553 | 90 | Plaintext::Whole { file: examples/customer_service_streaming/docker-compose.yaml } |  |  | 0.736 |
| ns | 9630 |  | 132 | `customer_service_streaming/src` and `configs`: complete listings | 7.1 |  | 0.743 |
| ns | 9850 |  | 220 | The legacy example's own `Swarm` class and its config knobs | 7.2 | 7.1 | 0.732 |
| ns | 9875 |  | 25 | Remaining asset and log directories | 7.3 |  | 0.733 |
| walker |  | 9895 | 342 | Code::CodeKey { rung: Body, file: swarm/repl/repl.py, decl: 1, sub: 0, line: 6 } |  |  | 0.733 |
| walker |  | 10000 | 105 | Code::CodeKey { rung: Body, file: swarm/core.py, decl: 8, sub: 0, line: 231 } |  |  | 0.742 |
