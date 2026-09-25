Score(3000)=0.746 I=0.917 C=0.607 ns_rows≤3K=25/66 grid(1000/1442/2080/3000/4327/6240/9000)=0.526/0.766/0.716/0.746/0.784/0.713/0.716

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 37 | 37 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 51 | 14 | Fs::DirListing { dir: assets } |  |  | 0.000 |
| walker |  | 72 | 21 | Fs::DirListing { dir: swarm } |  |  | 0.000 |
| walker |  | 82 | 10 | Fs::DirListing { dir: swarm/repl } |  |  | 0.000 |
| ns | 87 |  | 87 | README title + deprecation callout | 1.1 |  | 0.000 |
| walker |  | 101 | 19 | Code::CodeKey { rung: ModuleDoc, file: swarm/core.py, decl: 0, sub: 0, line: 0 } |  |  | 0.000 |
| ns | 124 |  | 37 | Repository root listing (complete) | 1.2 |  | 0.546 |
| walker |  | 211 | 110 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.922 |
| ns | 211 |  | 87 | README Overview: agents and handoffs | 1.3 |  | 0.922 |
| walker |  | 238 | 27 | Fs::DirListing { dir: tests } |  |  | 0.922 |
| ns | 242 |  | 31 | `swarm/` package listing (complete, incl. `repl/`) | 1.4 |  | 0.929 |
| walker |  | 246 | 8 | Code::CodeKey { rung: Names, file: swarm/core.py, decl: 0, sub: 0, line: 0 } |  |  | 0.929 |
| ns | 301 |  | 59 | Public exports of `swarm` and `swarm.repl` | 1.5 | 1.4 | 0.836 |
| walker |  | 326 | 80 | Code::CodeKey { rung: Decl, file: swarm/core.py, decl: 1, sub: 0, line: 26 } |  |  | 0.846 |
| walker |  | 389 | 63 | Code::CodeKey { rung: Decl, file: swarm/core.py, decl: 5, sub: 0, line: 89 } |  |  | 0.850 |
| ns | 399 |  | 98 | README: expressive power, and the statelessness NOTE | 1.6 | 1.3 | 0.804 |
| walker |  | 463 | 74 | Code::CodeKey { rung: Decl, file: swarm/core.py, decl: 3, sub: 0, line: 32 } |  |  | 0.812 |
| ns | 474 |  | 75 | `examples/` and `tests/` directory listings (complete) | 1.7 |  | 0.675 |
| walker |  | 555 | 92 | Code::CodeKey { rung: Decl, file: swarm/core.py, decl: 6, sub: 0, line: 139 } |  |  | 0.682 |
| walker |  | 661 | 106 | Code::CodeKey { rung: Decl, file: swarm/core.py, decl: 7, sub: 0, line: 231 } |  |  | 0.690 |
| ns | 684 |  | 210 | README section-heading roster (all remaining headings) | 1.8 | 1.1 | 0.558 |
| ns | 815 |  | 131 | `Agent` model: every field with its default | 2.1 |  | 0.515 |
| ns | 910 |  | 95 | `Swarm` class: complete method roster | 2.2 |  | 0.558 |
| ns | 998 |  | 88 | `Response` and `Result` models: every field | 2.3 | 2.1 | 0.526 |
| walker |  | 1002 | 341 | Fs::DirListing { dir: logs } |  |  | 0.526 |
| walker |  | 1050 | 48 | Fs::DirListing { dir: examples } |  |  | 0.645 |
| walker |  | 1067 | 17 | Fs::DirListing { dir: examples/weather_agent } |  |  | 0.646 |
| ns | 1070 |  | 72 | `swarm/util.py`: complete function roster | 2.4 |  | 0.630 |
| walker |  | 1087 | 20 | Fs::DirListing { dir: examples/personal_shopper } |  |  | 0.631 |
| walker |  | 1110 | 23 | Fs::DirListing { dir: examples/triage_agent } |  |  | 0.632 |
| ns | 1139 |  | 69 | `swarm/repl/repl.py`: complete function roster | 2.5 |  | 0.614 |
| ns | 1245 |  | 106 | `Swarm.run()` full signature with defaults | 2.6 | 2.2 | 0.644 |
| walker |  | 1301 | 191 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.754 |
| ns | 1337 |  | 92 | `run_and_stream()` full signature | 2.7 | 2.2 | 0.765 |
| walker |  | 1400 | 99 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.765 |
| walker |  | 1426 | 26 | Fs::DirListing { dir: examples/airline } |  |  | 0.766 |
| walker |  | 1429 | 3 | Fs::DirListing { dir: examples/airline/data } |  |  | 0.766 |
| walker |  | 1441 | 12 | Fs::DirListing { dir: examples/airline/data/routines } |  |  | 0.766 |
| walker |  | 1457 | 16 | Fs::DirListing { dir: examples/airline/configs } |  |  | 0.767 |
| ns | 1482 |  | 145 | Internal `Swarm` method signatures: completion + tool dispatch | 2.8 | 2.2 | 0.781 |
| walker |  | 1484 | 27 | Code::CodeKey { rung: Body, file: swarm/core.py, decl: 2, sub: 0, line: 27 } |  |  | 0.781 |
| walker |  | 1497 | 13 | Code::CodeKey { rung: Names, file: swarm/repl/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.782 |
| walker |  | 1531 | 34 | Fs::DirListing { dir: examples/basic } |  |  | 0.785 |
| walker |  | 1565 | 34 | Fs::DirListing { dir: examples/customer_service_streaming } |  |  | 0.787 |
| ns | 1574 |  | 92 | `swarm/types.py` imports: pydantic + reused OpenAI types | 2.9 |  | 0.759 |
| walker |  | 1592 | 27 | Fs::DirListing { dir: examples/customer_service_streaming/configs } |  |  | 0.760 |
| walker |  | 1604 | 12 | Fs::DirListing { dir: examples/customer_service_streaming/configs/tools } |  |  | 0.760 |
| walker |  | 1638 | 34 | Fs::DirListing { dir: examples/customer_service_streaming/src } |  |  | 0.760 |
| walker |  | 1657 | 19 | Fs::DirListing { dir: examples/customer_service_streaming/src/swarm } |  |  | 0.761 |
| walker |  | 1671 | 14 | Fs::DirListing { dir: examples/customer_service_streaming/src/swarm/engines } |  |  | 0.761 |
| walker |  | 1717 | 46 | Code::CodeKey { rung: Names, file: swarm/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.792 |
| ns | 1748 |  | 174 | `swarm/core.py` import block | 2.10 |  | 0.734 |
| walker |  | 1792 | 75 | Markdown::Section { file: README.md, section_index: 12, keeps_default_concavity: false } |  |  | 0.734 |
| walker |  | 1795 | 3 | Fs::DirListing { dir: examples/customer_service } |  |  | 0.735 |
| walker |  | 1798 | 3 | Fs::DirListing { dir: examples/customer_service_lite } |  |  | 0.735 |
| walker |  | 1840 | 42 | Fs::DirListing { dir: examples/support_bot } |  |  | 0.738 |
| ns | 1914 |  | 166 | `client.run()` semantics and the five-step loop | 3.1 |  | 0.719 |
| walker |  | 1922 | 82 | Markdown::Section { file: README.md, section_index: 13, keeps_default_concavity: false } |  |  | 0.721 |
| walker |  | 2012 | 90 | Plaintext::DeclSurface { file: setup.cfg } |  |  | 0.721 |
| ns | 2068 |  | 154 | `run()` arguments table, part 1 of 2 | 3.2 |  | 0.710 |
| walker |  | 2075 | 63 | Code::CodeKey { rung: Names, file: swarm/types.py, decl: 0, sub: 0, line: 0 } |  |  | 0.716 |
| walker |  | 2108 | 33 | Code::CodeKey { rung: Decl, file: swarm/types.py, decl: 3, sub: 0, line: 23 } |  |  | 0.727 |
| walker |  | 2143 | 35 | Code::CodeKey { rung: Decl, file: swarm/types.py, decl: 4, sub: 0, line: 29 } |  |  | 0.746 |
| walker |  | 2231 | 88 | Code::CodeKey { rung: Decl, file: swarm/types.py, decl: 2, sub: 0, line: 14 } |  |  | 0.783 |
| ns | 2242 |  | 174 | `run()` arguments table, part 2 of 2 (completes the table) | 3.3 | 3.2 | 0.771 |
| walker |  | 2303 | 72 | Code::CodeKey { rung: Names, file: swarm/util.py, decl: 0, sub: 0, line: 0 } |  |  | 0.786 |
| walker |  | 2364 | 61 | Code::CodeKey { rung: Body, file: swarm/util.py, decl: 2, sub: 0, line: 13 } |  |  | 0.787 |
| ns | 2398 |  | 156 | `Response` fields table (complete) | 3.4 |  | 0.775 |
| walker |  | 2451 | 87 | Code::CodeKey { rung: Doc, file: swarm/types.py, decl: 4, sub: 0, line: 29 } |  |  | 0.779 |
| ns | 2603 |  | 205 | `Agent` fields table (the README's documented subset) | 3.5 |  | 0.763 |
| walker |  | 2634 | 183 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.766 |
| walker |  | 2724 | 90 | Code::CodeKey { rung: Body, file: swarm/util.py, decl: 1, sub: 0, line: 5 } |  |  | 0.766 |
| ns | 2766 |  | 163 | Function/tool rules: return values, context, errors, ordering | 3.6 |  | 0.753 |
| walker |  | 2876 | 152 | Markdown::Section { file: README.md, section_index: 14, keeps_default_concavity: false } |  |  | 0.753 |
| ns | 2897 |  | 131 | Handoffs and `Result`: the documented rules | 3.7 |  | 0.745 |
| ns | 3018 |  | 121 | Function-schema conversion rules | 3.8 |  | 0.733 |
| walker |  | 3078 | 202 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.735 |
| walker |  | 3172 | 94 | Code::CodeKey { rung: Body, file: swarm/util.py, decl: 3, sub: 0, line: 21 } |  |  | 0.727 |
| ns | 3172 |  | 154 | Streaming: the two Swarm-specific event types | 3.9 |  | 0.727 |
| walker |  | 3280 | 108 | Code::CodeKey { rung: Doc, file: swarm/util.py, decl: 4, sub: 0, line: 31 } |  |  | 0.727 |
| walker |  | 3291 | 11 | Fs::DirListing { dir: tests/test_runs } |  |  | 0.727 |
| walker |  | 3334 | 43 | Code::CodeKey { rung: Names, file: swarm/repl/repl.py, decl: 0, sub: 0, line: 0 } |  |  | 0.733 |
| walker |  | 3360 | 26 | Code::CodeKey { rung: Decl, file: swarm/repl/repl.py, decl: 3, sub: 0, line: 60 } |  |  | 0.744 |
| ns | 3369 |  | 197 | Examples index: what each example directory demonstrates | 3.10 |  | 0.750 |
| ns | 3495 |  | 126 | Returning from `run()` and resuming a conversation | 3.11 |  | 0.749 |
| ns | 3670 |  | 175 | `Agent` definition and `instructions` semantics | 3.12 |  | 0.743 |
| walker |  | 3761 | 401 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.743 |
| ns | 3847 |  | 177 | Install, client construction, and the `run_demo_loop` util | 3.13 | 1.1 | 0.752 |
| walker |  | 3939 | 178 | Plaintext::Whole { file: setup.cfg } |  |  | 0.754 |
| walker |  | 3943 | 4 | Fs::DirListing { dir: examples/customer_service_streaming/configs/assistants } |  |  | 0.754 |
| walker |  | 3947 | 4 | Fs::DirListing { dir: examples/customer_service_streaming/src/runs } |  |  | 0.754 |
| walker |  | 3951 | 4 | Fs::DirListing { dir: examples/customer_service_streaming/src/tasks } |  |  | 0.754 |
| ns | 3992 |  | 145 | "Why Swarm" positioning and the evaluations stance | 3.14 |  | 0.752 |
| ns | 4066 |  | 74 | Listings for the three simple examples (complete) | 4.1 |  | 0.761 |
| walker |  | 4286 | 335 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.795 |
| walker |  | 4293 | 7 | Fs::DirListing { dir: examples/customer_service_streaming/logs } |  |  | 0.795 |
| walker |  | 4298 | 5 | Fs::DirListing { dir: examples/customer_service_streaming/src/evals } |  |  | 0.795 |
| ns | 4322 |  | 256 | Purpose line of all six example READMEs | 4.2 |  | 0.784 |
| ns | 4445 |  | 123 | `basic/bare_minimum.py` in full | 4.3 |  | 0.763 |
| walker |  | 4473 | 175 | Code::CodeKey { rung: Body, file: swarm/core.py, decl: 4, sub: 0, line: 71 } |  |  | 0.765 |
| walker |  | 4477 | 4 | Fs::DirListing { dir: examples/airline/data/routines/baggage } |  |  | 0.765 |
| walker |  | 4481 | 4 | Fs::DirListing { dir: examples/airline/data/routines/flight_modification } |  |  | 0.765 |
| walker |  | 4485 | 4 | Fs::DirListing { dir: examples/customer_service_streaming/configs/assistants/user_interface } |  |  | 0.765 |
| walker |  | 4496 | 11 | Fs::DirListing { dir: examples/customer_service_streaming/tests } |  |  | 0.765 |
| walker |  | 4503 | 7 | Fs::DirListing { dir: examples/customer_service_streaming/tests/test_runs } |  |  | 0.765 |
| ns | 4733 |  | 288 | `triage_agent/agents.py`: three agents and their wiring | 4.4 |  | 0.735 |
| ns | 4845 |  | 112 | `airline` example: complete directory tree | 4.5 |  | 0.724 |
| ns | 4907 |  | 62 | `support_bot` and `personal_shopper` listings (complete) | 4.6 |  | 0.732 |
| walker |  | 4938 | 435 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.774 |
| ns | 4947 |  | 40 | The three `customer_service*` directories: what is actually there | 4.7 |  | 0.779 |
| ns | 5149 |  | 202 | `weather_agent/agents.py`: a complete function-calling agent | 4.8 |  | 0.757 |
| walker |  | 5296 | 358 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.775 |
| ns | 5335 |  | 186 | `basic/context_variables.py`: callable instructions + injected context | 4.9 |  | 0.756 |
| ns | 5576 |  | 241 | `airline` agents: the five agents and their instruction sources | 4.10 |  | 0.735 |
| walker |  | 5735 | 439 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.750 |
| ns | 5765 |  | 189 | `airline` agents: imports and the five transfer functions | 4.11 | 4.10 | 0.733 |
| ns | 5958 |  | 193 | `airline` agents: per-agent tool lists (the handoff graph) | 4.12 | 4.10 | 0.713 |
| ns | 6043 |  | 85 | `airline/configs/tools.py`: complete tool roster | 4.13 |  | 0.706 |
| ns | 6286 |  | 243 | `run()` body: state setup and the completion half of the loop | 5.1 | 2.6 | 0.687 |
| walker |  | 6291 | 556 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.694 |
| ns | 6479 |  | 193 | `run()` body: tool dispatch, agent switch, and the returned `Response` | 5.2 | 5.1 | 0.679 |
| ns | 6572 |  | 93 | `run()` body: the `stream=True` delegation branch | 5.3 | 2.6 | 0.672 |
| ns | 6783 |  | 211 | `handle_tool_calls` body: dispatch table and the missing-tool path | 5.4 | 2.8 | 0.658 |
| walker |  | 6839 | 548 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.666 |
| ns | 7054 |  | 271 | `handle_tool_calls` body: context injection and `Result` merging | 5.5 | 5.4 | 0.649 |
| ns | 7265 |  | 211 | `get_chat_completion` body: instructions, tool schemas, context hiding | 5.6 | 2.8 | 0.638 |
| walker |  | 7362 | 523 | Markdown::Section { file: README.md, section_index: 10, keeps_default_concavity: false } |  |  | 0.650 |
| ns | 7390 |  | 125 | `get_chat_completion` body: the Chat Completions request | 5.7 | 5.6 | 0.643 |
| ns | 7565 |  | 175 | `handle_function_result` body: the return-value coercion rules | 5.8 | 2.8 | 0.651 |
| walker |  | 7574 | 212 | Markdown::Section { file: README.md, section_index: 11, keeps_default_concavity: false } |  |  | 0.658 |
| walker |  | 7593 | 19 | Fs::DirListing { dir: examples/airline/evals } |  |  | 0.666 |
| walker |  | 7601 | 8 | Fs::DirListing { dir: examples/customer_service_streaming/configs/tools/query_docs } |  |  | 0.666 |
| walker |  | 7609 | 8 | Fs::DirListing { dir: examples/customer_service_streaming/configs/tools/send_email } |  |  | 0.666 |
| walker |  | 7617 | 8 | Fs::DirListing { dir: examples/customer_service_streaming/configs/tools/submit_ticket } |  |  | 0.666 |
| walker |  | 7856 | 239 | Code::CodeKey { rung: Body, file: swarm/repl/repl.py, decl: 3, sub: 0, line: 60 } |  |  | 0.668 |
| walker |  | 7869 | 13 | Fs::DirListing { dir: examples/airline/evals/eval_cases } |  |  | 0.673 |
| ns | 7894 |  | 329 | `function_to_json` body: type map, signature inspection, `required` | 5.9 | 2.4 | 0.653 |
| ns | 8003 |  | 109 | `function_to_json` body: the emitted schema shape | 5.10 | 5.9 | 0.646 |
| walker |  | 8123 | 254 | Code::CodeKey { rung: Body, file: swarm/repl/repl.py, decl: 2, sub: 0, line: 37 } |  |  | 0.646 |
| walker |  | 8138 | 15 | Fs::DirListing { dir: examples/airline/evals/eval_results } |  |  | 0.650 |
| ns | 8173 |  | 170 | `swarm/util.py`: the streaming merge helpers | 5.11 | 2.4 | 0.655 |
| ns | 8412 |  | 239 | `run_demo_loop` body: the reference conversation loop | 5.12 | 2.5 | 0.666 |
| walker |  | 8474 | 336 | Code::CodeKey { rung: Body, file: swarm/core.py, decl: 3, sub: 0, line: 32 } |  |  | 0.697 |
| ns | 8646 |  | 234 | `run_and_stream` body: the streaming protocol, with elisions marked | 5.13 | 2.7 | 0.683 |
| ns | 8875 |  | 229 | `tests/test_core.py`: complete test roster and the shared fixture | 6.1 |  | 0.673 |
| walker |  | 8915 | 441 | Code::CodeKey { rung: Body, file: swarm/util.py, decl: 4, sub: 0, line: 31 } |  |  | 0.716 |
| ns | 9032 |  | 157 | `tests/mock_client.py`: the fake OpenAI client | 6.2 |  | 0.711 |
| ns | 9134 |  | 102 | `tests/test_util.py`: both schema-conversion tests | 6.3 |  | 0.707 |
| walker |  | 9257 | 342 | Code::CodeKey { rung: Body, file: swarm/repl/repl.py, decl: 1, sub: 0, line: 6 } |  |  | 0.707 |
| walker |  | 9301 | 44 | Fs::DirListing { dir: examples/customer_service_lite/logs } |  |  | 0.707 |
| ns | 9340 |  | 206 | `setup.cfg`: package metadata and the complete dependency list | 6.4 |  | 0.713 |
| ns | 9515 |  | 175 | Build backend and formatting toolchain | 6.5 |  | 0.706 |
| ns | 9649 |  | 134 | `customer_service_streaming/src` and `configs`: complete listings | 7.1 |  | 0.714 |
| walker |  | 9783 | 482 | Code::CodeKey { rung: Body, file: swarm/core.py, decl: 5, sub: 0, line: 89 } |  |  | 0.750 |
| walker |  | 9838 | 55 | Fs::DirListing { dir: examples/customer_service/logs } |  |  | 0.750 |
| ns | 9869 |  | 220 | The legacy example's own `Swarm` class and its config knobs | 7.2 | 7.1 | 0.740 |
| ns | 9894 |  | 25 | Remaining asset and log directories | 7.3 |  | 0.741 |
| walker |  | 9983 | 145 | Code::CodeKey { rung: Body, file: swarm/core.py, decl: 7, sub: 0, line: 231 } |  |  | 0.750 |
