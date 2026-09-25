Score(3000)=0.762 I=0.930 C=0.625 ns_rows≤3K=25/66 grid(1000/1442/2080/3000/4327/6240/9000)=0.698/0.643/0.742/0.762/0.821/0.723/0.704

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 37 | 37 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 87 |  | 87 | README title + deprecation callout | 1.1 |  | 0.000 |
| ns | 124 |  | 37 | Repository root listing (complete) | 1.2 |  | 0.487 |
| walker |  | 147 | 110 | Markdown::ReadmeHeadline { file: README.md } |  |  | 1.000 |
| walker |  | 161 | 14 | Fs::DirListing { dir: assets } |  |  | 1.000 |
| walker |  | 182 | 21 | Fs::DirListing { dir: swarm } |  |  | 1.000 |
| walker |  | 192 | 10 | Fs::DirListing { dir: swarm/repl } |  |  | 1.000 |
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
| walker |  | 1054 | 13 | Code::CodeKey { rung: Names, file: swarm/repl/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.703 |
| ns | 1070 |  | 72 | `swarm/util.py`: complete function roster | 2.4 |  | 0.686 |
| walker |  | 1100 | 46 | Code::CodeKey { rung: Names, file: swarm/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.727 |
| ns | 1139 |  | 69 | `swarm/repl/repl.py`: complete function roster | 2.5 |  | 0.707 |
| walker |  | 1175 | 75 | Markdown::Section { file: README.md, section_index: 12, keeps_default_concavity: false } |  |  | 0.707 |
| walker |  | 1209 | 34 | Fs::DirListing { dir: examples/basic } |  |  | 0.710 |
| walker |  | 1243 | 34 | Fs::DirListing { dir: examples/customer_service_streaming } |  |  | 0.712 |
| ns | 1245 |  | 106 | `Swarm.run()` full signature with defaults | 2.6 | 2.2 | 0.671 |
| walker |  | 1270 | 27 | Fs::DirListing { dir: examples/customer_service_streaming/configs } |  |  | 0.672 |
| walker |  | 1282 | 12 | Fs::DirListing { dir: examples/customer_service_streaming/configs/tools } |  |  | 0.672 |
| walker |  | 1290 | 8 | Fs::DirListing { dir: examples/customer_service_streaming/configs/tools/query_docs } |  |  | 0.672 |
| walker |  | 1298 | 8 | Fs::DirListing { dir: examples/customer_service_streaming/configs/tools/send_email } |  |  | 0.672 |
| walker |  | 1306 | 8 | Fs::DirListing { dir: examples/customer_service_streaming/configs/tools/submit_ticket } |  |  | 0.672 |
| ns | 1337 |  | 92 | `run_and_stream()` full signature | 2.7 | 2.2 | 0.640 |
| walker |  | 1340 | 34 | Fs::DirListing { dir: examples/customer_service_streaming/src } |  |  | 0.641 |
| walker |  | 1344 | 4 | Fs::DirListing { dir: examples/customer_service_streaming/src/runs } |  |  | 0.641 |
| walker |  | 1348 | 4 | Fs::DirListing { dir: examples/customer_service_streaming/src/tasks } |  |  | 0.641 |
| walker |  | 1353 | 5 | Fs::DirListing { dir: examples/customer_service_streaming/src/evals } |  |  | 0.641 |
| walker |  | 1372 | 19 | Fs::DirListing { dir: examples/customer_service_streaming/src/swarm } |  |  | 0.642 |
| walker |  | 1386 | 14 | Fs::DirListing { dir: examples/customer_service_streaming/src/swarm/engines } |  |  | 0.642 |
| walker |  | 1468 | 82 | Markdown::Section { file: README.md, section_index: 13, keeps_default_concavity: false } |  |  | 0.644 |
| ns | 1482 |  | 145 | Internal `Swarm` method signatures: completion + tool dispatch | 2.8 | 2.2 | 0.598 |
| walker |  | 1510 | 42 | Fs::DirListing { dir: examples/support_bot } |  |  | 0.601 |
| walker |  | 1573 | 63 | Code::CodeKey { rung: Names, file: swarm/types.py, decl: 0, sub: 0, line: 0 } |  |  | 0.609 |
| ns | 1574 |  | 92 | `swarm/types.py` imports: pydantic + reused OpenAI types | 2.9 |  | 0.588 |
| walker |  | 1606 | 33 | Code::CodeKey { rung: Decl, file: swarm/types.py, decl: 3, sub: 0, line: 23 } |  |  | 0.602 |
| walker |  | 1641 | 35 | Code::CodeKey { rung: Decl, file: swarm/types.py, decl: 4, sub: 0, line: 29 } |  |  | 0.627 |
| walker |  | 1729 | 88 | Code::CodeKey { rung: Decl, file: swarm/types.py, decl: 2, sub: 0, line: 14 } |  |  | 0.674 |
| ns | 1748 |  | 174 | `swarm/core.py` import block | 2.10 |  | 0.626 |
| walker |  | 1766 | 37 | Code::CodeKey { rung: Names, file: swarm/core.py, decl: 0, sub: 0, line: 0 } |  |  | 0.628 |
| walker |  | 1846 | 80 | Code::CodeKey { rung: Decl, file: swarm/core.py, decl: 2, sub: 0, line: 26 } |  |  | 0.666 |
| walker |  | 1909 | 63 | Code::CodeKey { rung: Decl, file: swarm/core.py, decl: 6, sub: 0, line: 89 } |  |  | 0.687 |
| ns | 1914 |  | 166 | `client.run()` semantics and the five-step loop | 3.1 |  | 0.669 |
| walker |  | 1983 | 74 | Code::CodeKey { rung: Decl, file: swarm/core.py, decl: 4, sub: 0, line: 32 } |  |  | 0.715 |
| ns | 2068 |  | 154 | `run()` arguments table, part 1 of 2 | 3.2 |  | 0.704 |
| walker |  | 2075 | 92 | Code::CodeKey { rung: Decl, file: swarm/core.py, decl: 7, sub: 0, line: 139 } |  |  | 0.742 |
| walker |  | 2181 | 106 | Code::CodeKey { rung: Decl, file: swarm/core.py, decl: 8, sub: 0, line: 231 } |  |  | 0.784 |
| ns | 2242 |  | 174 | `run()` arguments table, part 2 of 2 (completes the table) | 3.3 | 3.2 | 0.772 |
| walker |  | 2271 | 90 | Plaintext::DeclSurface { file: setup.cfg } |  |  | 0.773 |
| walker |  | 2298 | 27 | Code::CodeKey { rung: Body, file: swarm/core.py, decl: 3, sub: 0, line: 27 } |  |  | 0.773 |
| walker |  | 2341 | 43 | Code::CodeKey { rung: Names, file: swarm/repl/repl.py, decl: 0, sub: 0, line: 0 } |  |  | 0.779 |
| walker |  | 2367 | 26 | Code::CodeKey { rung: Decl, file: swarm/repl/repl.py, decl: 3, sub: 0, line: 60 } |  |  | 0.791 |
| ns | 2398 |  | 156 | `Response` fields table (complete) | 3.4 |  | 0.780 |
| walker |  | 2439 | 72 | Code::CodeKey { rung: Names, file: swarm/util.py, decl: 0, sub: 0, line: 0 } |  |  | 0.795 |
| ns | 2603 |  | 205 | `Agent` fields table (the README's documented subset) | 3.5 |  | 0.779 |
| walker |  | 2622 | 183 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.781 |
| ns | 2766 |  | 163 | Function/tool rules: return values, context, errors, ordering | 3.6 |  | 0.768 |
| walker |  | 2774 | 152 | Markdown::Section { file: README.md, section_index: 14, keeps_default_concavity: false } |  |  | 0.768 |
| ns | 2897 |  | 131 | Handoffs and `Result`: the documented rules | 3.7 |  | 0.760 |
| walker |  | 2976 | 202 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.762 |
| ns | 3018 |  | 121 | Function-schema conversion rules | 3.8 |  | 0.750 |
| ns | 3172 |  | 154 | Streaming: the two Swarm-specific event types | 3.9 |  | 0.741 |
| ns | 3369 |  | 197 | Examples index: what each example directory demonstrates | 3.10 |  | 0.747 |
| walker |  | 3377 | 401 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.747 |
| ns | 3481 |  | 112 | Returning from `run()` and resuming a conversation | 3.11 |  | 0.745 |
| walker |  | 3555 | 178 | Plaintext::Whole { file: setup.cfg } |  |  | 0.747 |
| walker |  | 3566 | 11 | Fs::DirListing { dir: tests/test_runs } |  |  | 0.747 |
| ns | 3656 |  | 175 | `Agent` definition and `instructions` semantics | 3.12 |  | 0.741 |
| ns | 3833 |  | 177 | Install, client construction, and the `run_demo_loop` util | 3.13 | 1.1 | 0.751 |
| walker |  | 3901 | 335 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.783 |
| walker |  | 3908 | 7 | Fs::DirListing { dir: examples/customer_service_streaming/logs } |  |  | 0.783 |
| ns | 3978 |  | 145 | "Why Swarm" positioning and the evaluations stance | 3.14 |  | 0.784 |
| walker |  | 3995 | 87 | Code::CodeKey { rung: Doc, file: swarm/types.py, decl: 4, sub: 0, line: 29 } |  |  | 0.787 |
| walker |  | 4006 | 11 | Fs::DirListing { dir: examples/customer_service_streaming/tests } |  |  | 0.787 |
| ns | 4052 |  | 74 | Listings for the three simple examples (complete) | 4.1 |  | 0.795 |
| ns | 4308 |  | 256 | Purpose line of all six example READMEs | 4.2 |  | 0.783 |
| ns | 4431 |  | 123 | `basic/bare_minimum.py` in full | 4.3 |  | 0.763 |
| walker |  | 4441 | 435 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.811 |
| ns | 4719 |  | 288 | `triage_agent/agents.py`: three agents and their wiring | 4.4 |  | 0.779 |
| walker |  | 4785 | 344 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.800 |
| ns | 4830 |  | 111 | `airline` example: complete directory tree | 4.5 |  | 0.796 |
| walker |  | 4846 | 61 | Code::CodeKey { rung: Body, file: swarm/util.py, decl: 2, sub: 0, line: 13 } |  |  | 0.796 |
| ns | 4892 |  | 62 | `support_bot` and `personal_shopper` listings (complete) | 4.6 |  | 0.801 |
| ns | 4930 |  | 38 | The three `customer_service*` directories: what is actually there | 4.7 |  | 0.799 |
| ns | 5132 |  | 202 | `weather_agent/agents.py`: a complete function-calling agent | 4.8 |  | 0.776 |
| walker |  | 5285 | 439 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.792 |
| walker |  | 5292 | 7 | Fs::DirListing { dir: examples/customer_service_streaming/tests/test_runs } |  |  | 0.792 |
| ns | 5318 |  | 186 | `basic/context_variables.py`: callable instructions + injected context | 4.9 |  | 0.772 |
| ns | 5559 |  | 241 | `airline` agents: the five agents and their instruction sources | 4.10 |  | 0.751 |
| ns | 5748 |  | 189 | `airline` agents: imports and the five transfer functions | 4.11 | 4.10 | 0.734 |
| walker |  | 5848 | 556 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.740 |
| ns | 5941 |  | 193 | `airline` agents: per-agent tool lists (the handoff graph) | 4.12 | 4.10 | 0.720 |
| ns | 6026 |  | 85 | `airline/configs/tools.py`: complete tool roster | 4.13 |  | 0.714 |
| ns | 6269 |  | 243 | `run()` body: state setup and the completion half of the loop | 5.1 | 2.6 | 0.695 |
| walker |  | 6396 | 548 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.703 |
| ns | 6462 |  | 193 | `run()` body: tool dispatch, agent switch, and the returned `Response` | 5.2 | 5.1 | 0.688 |
| ns | 6555 |  | 93 | `run()` body: the `stream=True` delegation branch | 5.3 | 2.6 | 0.681 |
| ns | 6766 |  | 211 | `handle_tool_calls` body: dispatch table and the missing-tool path | 5.4 | 2.8 | 0.667 |
| walker |  | 6919 | 523 | Markdown::Section { file: README.md, section_index: 10, keeps_default_concavity: false } |  |  | 0.680 |
| ns | 7037 |  | 271 | `handle_tool_calls` body: context injection and `Result` merging | 5.5 | 5.4 | 0.663 |
| walker |  | 7131 | 212 | Markdown::Section { file: README.md, section_index: 11, keeps_default_concavity: false } |  |  | 0.670 |
| walker |  | 7137 | 6 | Fs::DirListing { dir: examples/customer_service_streaming/configs/assistants/user_interface } |  |  | 0.670 |
| walker |  | 7245 | 108 | Code::CodeKey { rung: Doc, file: swarm/util.py, decl: 4, sub: 0, line: 31 } |  |  | 0.670 |
| ns | 7248 |  | 211 | `get_chat_completion` body: instructions, tool schemas, context hiding | 5.6 | 2.8 | 0.659 |
| ns | 7373 |  | 125 | `get_chat_completion` body: the Chat Completions request | 5.7 | 5.6 | 0.651 |
| walker |  | 7420 | 175 | Code::CodeKey { rung: Body, file: swarm/core.py, decl: 5, sub: 0, line: 71 } |  |  | 0.652 |
| walker |  | 7433 | 13 | Fs::DirListing { dir: examples/airline/evals/eval_cases } |  |  | 0.657 |
| walker |  | 7523 | 90 | Code::CodeKey { rung: Body, file: swarm/util.py, decl: 1, sub: 0, line: 5 } |  |  | 0.657 |
| walker |  | 7538 | 15 | Fs::DirListing { dir: examples/airline/evals/eval_results } |  |  | 0.662 |
| ns | 7548 |  | 175 | `handle_function_result` body: the return-value coercion rules | 5.8 | 2.8 | 0.670 |
| walker |  | 7632 | 94 | Code::CodeKey { rung: Body, file: swarm/util.py, decl: 3, sub: 0, line: 21 } |  |  | 0.671 |
| walker |  | 7678 | 46 | Fs::DirListing { dir: examples/customer_service_lite/logs } |  |  | 0.673 |
| ns | 7877 |  | 329 | `function_to_json` body: type map, signature inspection, `required` | 5.9 | 2.4 | 0.653 |
| walker |  | 7917 | 239 | Code::CodeKey { rung: Body, file: swarm/repl/repl.py, decl: 3, sub: 0, line: 60 } |  |  | 0.655 |
| ns | 7986 |  | 109 | `function_to_json` body: the emitted schema shape | 5.10 | 5.9 | 0.648 |
| ns | 8156 |  | 170 | `swarm/util.py`: the streaming merge helpers | 5.11 | 2.4 | 0.652 |
| walker |  | 8253 | 336 | Code::CodeKey { rung: Body, file: swarm/core.py, decl: 4, sub: 0, line: 32 } |  |  | 0.685 |
| walker |  | 8310 | 57 | Fs::DirListing { dir: examples/customer_service/logs } |  |  | 0.687 |
| ns | 8395 |  | 239 | `run_demo_loop` body: the reference conversation loop | 5.12 | 2.5 | 0.697 |
| walker |  | 8564 | 254 | Code::CodeKey { rung: Body, file: swarm/repl/repl.py, decl: 2, sub: 0, line: 37 } |  |  | 0.697 |
| walker |  | 8573 | 9 | Plaintext::DeclSurface { file: examples/support_bot/requirements.txt } |  |  | 0.697 |
| ns | 8629 |  | 234 | `run_and_stream` body: the streaming protocol, with elisions marked | 5.13 | 2.7 | 0.683 |
| ns | 8858 |  | 229 | `tests/test_core.py`: complete test roster and the shared fixture | 6.1 |  | 0.673 |
| ns | 9015 |  | 157 | `tests/mock_client.py`: the fake OpenAI client | 6.2 |  | 0.668 |
| walker |  | 9055 | 482 | Code::CodeKey { rung: Body, file: swarm/core.py, decl: 6, sub: 0, line: 89 } |  |  | 0.709 |
| walker |  | 9114 | 59 | Plaintext::Whole { file: examples/support_bot/Makefile } |  |  | 0.709 |
| ns | 9117 |  | 102 | `tests/test_util.py`: both schema-conversion tests | 6.3 |  | 0.706 |
| ns | 9323 |  | 206 | `setup.cfg`: package metadata and the complete dependency list | 6.4 |  | 0.712 |
| ns | 9498 |  | 175 | Build backend and formatting toolchain | 6.5 |  | 0.705 |
| walker |  | 9555 | 441 | Code::CodeKey { rung: Body, file: swarm/util.py, decl: 4, sub: 0, line: 31 } |  |  | 0.744 |
| ns | 9630 |  | 132 | `customer_service_streaming/src` and `configs`: complete listings | 7.1 |  | 0.750 |
| walker |  | 9632 | 77 | Plaintext::Whole { file: examples/support_bot/docker-compose.yaml } |  |  | 0.750 |
| ns | 9850 |  | 220 | The legacy example's own `Swarm` class and its config knobs | 7.2 | 7.1 | 0.740 |
| ns | 9875 |  | 25 | Remaining asset and log directories | 7.3 |  | 0.741 |
| walker |  | 9974 | 342 | Code::CodeKey { rung: Body, file: swarm/repl/repl.py, decl: 1, sub: 0, line: 6 } |  |  | 0.741 |
| walker |  | 9993 | 19 | Plaintext::Whole { file: examples/customer_service_streaming/docker-compose.yaml } |  |  | 0.741 |
