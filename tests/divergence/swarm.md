Score(3000)=0.721 I=0.834 C=0.623 ns_rows≤3K=25/66 grid(1000/1442/2080/3000/4327/6240/9000)=0.381/0.584/0.739/0.721/0.743/0.692/0.687

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 37 | 37 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 51 | 14 | Fs::DirListing { dir: assets } |  |  | 0.000 |
| walker |  | 67 | 16 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.000 |
| ns | 87 |  | 87 | README title + deprecation callout | 1.1 |  | 0.116 |
| walker |  | 88 | 21 | Fs::DirListing { dir: swarm } |  |  | 0.123 |
| walker |  | 98 | 10 | Fs::DirListing { dir: swarm/repl } |  |  | 0.129 |
| walker |  | 111 | 13 | Code::CodeKey { rung: Names, file: swarm/repl/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.129 |
| ns | 124 |  | 37 | Repository root listing (complete) | 1.2 |  | 0.572 |
| walker |  | 130 | 19 | Code::CodeKey { rung: ModuleDoc, file: swarm/core.py, decl: 0, sub: 0, line: 0 } |  |  | 0.573 |
| walker |  | 138 | 8 | Code::CodeKey { rung: Names, file: swarm/core.py, decl: 0, sub: 0, line: 0 } |  |  | 0.574 |
| walker |  | 184 | 46 | Code::CodeKey { rung: Names, file: swarm/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.602 |
| ns | 211 |  | 87 | README Overview: agents and handoffs | 1.3 |  | 0.542 |
| ns | 242 |  | 31 | `swarm/` package listing (complete, incl. `repl/`) | 1.4 |  | 0.564 |
| walker |  | 264 | 80 | Code::CodeKey { rung: Decl, file: swarm/core.py, decl: 1, sub: 0, line: 26 } |  |  | 0.579 |
| ns | 301 |  | 59 | Public exports of `swarm` and `swarm.repl` | 1.5 | 1.4 | 0.585 |
| walker |  | 327 | 63 | Code::CodeKey { rung: Decl, file: swarm/core.py, decl: 5, sub: 0, line: 89 } |  |  | 0.590 |
| ns | 399 |  | 98 | README: expressive power, and the statelessness NOTE | 1.6 | 1.3 | 0.558 |
| walker |  | 401 | 74 | Code::CodeKey { rung: Decl, file: swarm/core.py, decl: 3, sub: 0, line: 32 } |  |  | 0.571 |
| ns | 474 |  | 75 | `examples/` and `tests/` directory listings (complete) | 1.7 |  | 0.458 |
| walker |  | 493 | 92 | Code::CodeKey { rung: Decl, file: swarm/core.py, decl: 6, sub: 0, line: 139 } |  |  | 0.468 |
| walker |  | 525 | 32 | Toml::Config { file: pyproject.toml } |  |  | 0.468 |
| walker |  | 631 | 106 | Code::CodeKey { rung: Decl, file: swarm/core.py, decl: 7, sub: 0, line: 231 } |  |  | 0.479 |
| walker |  | 658 | 27 | Fs::DirListing { dir: tests } |  |  | 0.499 |
| ns | 684 |  | 210 | README section-heading roster (all remaining headings) | 1.8 | 1.1 | 0.403 |
| ns | 815 |  | 131 | `Agent` model: every field with its default | 2.1 |  | 0.373 |
| ns | 910 |  | 95 | `Swarm` class: complete method roster | 2.2 |  | 0.403 |
| ns | 998 |  | 88 | `Response` and `Result` models: every field | 2.3 | 2.1 | 0.381 |
| walker |  | 999 | 341 | Fs::DirListing { dir: logs } |  |  | 0.381 |
| walker |  | 1047 | 48 | Fs::DirListing { dir: examples } |  |  | 0.481 |
| walker |  | 1064 | 17 | Fs::DirListing { dir: examples/weather_agent } |  |  | 0.482 |
| ns | 1070 |  | 72 | `swarm/util.py`: complete function roster | 2.4 |  | 0.470 |
| walker |  | 1084 | 20 | Fs::DirListing { dir: examples/personal_shopper } |  |  | 0.471 |
| walker |  | 1107 | 23 | Fs::DirListing { dir: examples/triage_agent } |  |  | 0.472 |
| ns | 1139 |  | 69 | `swarm/repl/repl.py`: complete function roster | 2.5 |  | 0.459 |
| ns | 1245 |  | 106 | `Swarm.run()` full signature with defaults | 2.6 | 2.2 | 0.481 |
| walker |  | 1300 | 193 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.576 |
| ns | 1337 |  | 92 | `run_and_stream()` full signature | 2.7 | 2.2 | 0.584 |
| ns | 1482 |  | 145 | Internal `Swarm` method signatures: completion + tool dispatch | 2.8 | 2.2 | 0.595 |
| walker |  | 1491 | 191 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.765 |
| walker |  | 1517 | 26 | Fs::DirListing { dir: examples/airline } |  |  | 0.766 |
| walker |  | 1520 | 3 | Fs::DirListing { dir: examples/airline/data } |  |  | 0.766 |
| walker |  | 1532 | 12 | Fs::DirListing { dir: examples/airline/data/routines } |  |  | 0.767 |
| walker |  | 1548 | 16 | Fs::DirListing { dir: examples/airline/configs } |  |  | 0.767 |
| ns | 1574 |  | 92 | `swarm/types.py` imports: pydantic + reused OpenAI types | 2.9 |  | 0.741 |
| walker |  | 1575 | 27 | Code::CodeKey { rung: Body, file: swarm/core.py, decl: 2, sub: 0, line: 27 } |  |  | 0.741 |
| walker |  | 1609 | 34 | Fs::DirListing { dir: examples/basic } |  |  | 0.744 |
| walker |  | 1643 | 34 | Fs::DirListing { dir: examples/customer_service_streaming } |  |  | 0.745 |
| walker |  | 1670 | 27 | Fs::DirListing { dir: examples/customer_service_streaming/configs } |  |  | 0.745 |
| walker |  | 1682 | 12 | Fs::DirListing { dir: examples/customer_service_streaming/configs/tools } |  |  | 0.746 |
| walker |  | 1716 | 34 | Fs::DirListing { dir: examples/customer_service_streaming/src } |  |  | 0.746 |
| walker |  | 1735 | 19 | Fs::DirListing { dir: examples/customer_service_streaming/src/swarm } |  |  | 0.747 |
| ns | 1748 |  | 174 | `swarm/core.py` import block | 2.10 |  | 0.693 |
| walker |  | 1749 | 14 | Fs::DirListing { dir: examples/customer_service_streaming/src/swarm/engines } |  |  | 0.693 |
| walker |  | 1752 | 3 | Fs::DirListing { dir: examples/customer_service } |  |  | 0.694 |
| walker |  | 1755 | 3 | Fs::DirListing { dir: examples/customer_service_lite } |  |  | 0.694 |
| walker |  | 1818 | 63 | Code::CodeKey { rung: Names, file: swarm/types.py, decl: 0, sub: 0, line: 0 } |  |  | 0.700 |
| walker |  | 1851 | 33 | Code::CodeKey { rung: Decl, file: swarm/types.py, decl: 3, sub: 0, line: 23 } |  |  | 0.711 |
| walker |  | 1886 | 35 | Code::CodeKey { rung: Decl, file: swarm/types.py, decl: 4, sub: 0, line: 29 } |  |  | 0.731 |
| ns | 1914 |  | 166 | `client.run()` semantics and the five-step loop | 3.1 |  | 0.712 |
| walker |  | 1974 | 88 | Code::CodeKey { rung: Decl, file: swarm/types.py, decl: 2, sub: 0, line: 14 } |  |  | 0.748 |
| walker |  | 2016 | 42 | Fs::DirListing { dir: examples/support_bot } |  |  | 0.751 |
| ns | 2068 |  | 154 | `run()` arguments table, part 1 of 2 | 3.2 |  | 0.739 |
| walker |  | 2088 | 72 | Code::CodeKey { rung: Names, file: swarm/util.py, decl: 0, sub: 0, line: 0 } |  |  | 0.754 |
| ns | 2242 |  | 174 | `run()` arguments table, part 2 of 2 (completes the table) | 3.3 | 3.2 | 0.743 |
| ns | 2398 |  | 156 | `Response` fields table (complete) | 3.4 |  | 0.732 |
| walker |  | 2489 | 401 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.732 |
| walker |  | 2532 | 43 | Code::CodeKey { rung: Names, file: swarm/repl/repl.py, decl: 0, sub: 0, line: 0 } |  |  | 0.738 |
| walker |  | 2558 | 26 | Code::CodeKey { rung: Decl, file: swarm/repl/repl.py, decl: 3, sub: 0, line: 60 } |  |  | 0.750 |
| ns | 2603 |  | 205 | `Agent` fields table (the README's documented subset) | 3.5 |  | 0.734 |
| walker |  | 2633 | 75 | Markdown::Section { file: README.md, section_index: 12, keeps_default_concavity: false } |  |  | 0.735 |
| walker |  | 2694 | 61 | Code::CodeKey { rung: Body, file: swarm/util.py, decl: 2, sub: 0, line: 13 } |  |  | 0.735 |
| ns | 2766 |  | 163 | Function/tool rules: return values, context, errors, ordering | 3.6 |  | 0.723 |
| walker |  | 2776 | 82 | Markdown::Section { file: README.md, section_index: 13, keeps_default_concavity: false } |  |  | 0.725 |
| walker |  | 2863 | 87 | Code::CodeKey { rung: Doc, file: swarm/types.py, decl: 4, sub: 0, line: 29 } |  |  | 0.729 |
| ns | 2897 |  | 131 | Handoffs and `Result`: the documented rules | 3.7 |  | 0.721 |
| ns | 3018 |  | 121 | Function-schema conversion rules | 3.8 |  | 0.709 |
| walker |  | 3131 | 268 | Plaintext::Whole { file: setup.cfg } |  |  | 0.712 |
| ns | 3172 |  | 154 | Streaming: the two Swarm-specific event types | 3.9 |  | 0.702 |
| walker |  | 3221 | 90 | Code::CodeKey { rung: Body, file: swarm/util.py, decl: 1, sub: 0, line: 5 } |  |  | 0.702 |
| walker |  | 3315 | 94 | Code::CodeKey { rung: Body, file: swarm/util.py, decl: 3, sub: 0, line: 21 } |  |  | 0.703 |
| ns | 3369 |  | 197 | Examples index: what each example directory demonstrates | 3.10 |  | 0.689 |
| walker |  | 3423 | 108 | Code::CodeKey { rung: Doc, file: swarm/util.py, decl: 4, sub: 0, line: 31 } |  |  | 0.689 |
| walker |  | 3434 | 11 | Fs::DirListing { dir: tests/test_runs } |  |  | 0.689 |
| walker |  | 3438 | 4 | Fs::DirListing { dir: examples/customer_service_streaming/configs/assistants } |  |  | 0.690 |
| walker |  | 3442 | 4 | Fs::DirListing { dir: examples/customer_service_streaming/src/runs } |  |  | 0.690 |
| walker |  | 3446 | 4 | Fs::DirListing { dir: examples/customer_service_streaming/src/tasks } |  |  | 0.690 |
| walker |  | 3453 | 7 | Fs::DirListing { dir: examples/customer_service_streaming/logs } |  |  | 0.690 |
| ns | 3495 |  | 126 | Returning from `run()` and resuming a conversation | 3.11 |  | 0.688 |
| walker |  | 3636 | 183 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.691 |
| ns | 3670 |  | 175 | `Agent` definition and `instructions` semantics | 3.12 |  | 0.686 |
| ns | 3847 |  | 177 | Install, client construction, and the `run_demo_loop` util | 3.13 | 1.1 | 0.696 |
| ns | 3992 |  | 145 | "Why Swarm" positioning and the evaluations stance | 3.14 |  | 0.694 |
| ns | 4066 |  | 74 | Listings for the three simple examples (complete) | 4.1 |  | 0.705 |
| walker |  | 4071 | 435 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: true } |  |  | 0.754 |
| walker |  | 4223 | 152 | Markdown::Section { file: README.md, section_index: 14, keeps_default_concavity: false } |  |  | 0.754 |
| ns | 4322 |  | 256 | Purpose line of all six example READMEs | 4.2 |  | 0.743 |
| walker |  | 4425 | 202 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.761 |
| walker |  | 4430 | 5 | Fs::DirListing { dir: examples/customer_service_streaming/src/evals } |  |  | 0.762 |
| ns | 4445 |  | 123 | `basic/bare_minimum.py` in full | 4.3 |  | 0.741 |
| walker |  | 4605 | 175 | Code::CodeKey { rung: Body, file: swarm/core.py, decl: 4, sub: 0, line: 71 } |  |  | 0.743 |
| walker |  | 4609 | 4 | Fs::DirListing { dir: examples/airline/data/routines/baggage } |  |  | 0.744 |
| walker |  | 4613 | 4 | Fs::DirListing { dir: examples/airline/data/routines/flight_modification } |  |  | 0.744 |
| walker |  | 4617 | 4 | Fs::DirListing { dir: examples/customer_service_streaming/configs/assistants/user_interface } |  |  | 0.744 |
| walker |  | 4628 | 11 | Fs::DirListing { dir: examples/customer_service_streaming/tests } |  |  | 0.744 |
| ns | 4733 |  | 288 | `triage_agent/agents.py`: three agents and their wiring | 4.4 |  | 0.714 |
| ns | 4845 |  | 112 | `airline` example: complete directory tree | 4.5 |  | 0.703 |
| ns | 4907 |  | 62 | `support_bot` and `personal_shopper` listings (complete) | 4.6 |  | 0.709 |
| ns | 4947 |  | 40 | The three `customer_service*` directories: what is actually there | 4.7 |  | 0.714 |
| walker |  | 4963 | 335 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.744 |
| walker |  | 4970 | 7 | Fs::DirListing { dir: examples/customer_service_streaming/tests/test_runs } |  |  | 0.744 |
| ns | 5149 |  | 202 | `weather_agent/agents.py`: a complete function-calling agent | 4.8 |  | 0.723 |
| walker |  | 5328 | 358 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.741 |
| ns | 5335 |  | 186 | `basic/context_variables.py`: callable instructions + injected context | 4.9 |  | 0.722 |
| walker |  | 5347 | 19 | Fs::DirListing { dir: examples/airline/evals } |  |  | 0.732 |
| walker |  | 5355 | 8 | Fs::DirListing { dir: examples/customer_service_streaming/configs/tools/query_docs } |  |  | 0.732 |
| walker |  | 5363 | 8 | Fs::DirListing { dir: examples/customer_service_streaming/configs/tools/send_email } |  |  | 0.732 |
| walker |  | 5371 | 8 | Fs::DirListing { dir: examples/customer_service_streaming/configs/tools/submit_ticket } |  |  | 0.732 |
| ns | 5576 |  | 241 | `airline` agents: the five agents and their instruction sources | 4.10 |  | 0.712 |
| walker |  | 5610 | 239 | Code::CodeKey { rung: Body, file: swarm/repl/repl.py, decl: 3, sub: 0, line: 60 } |  |  | 0.715 |
| ns | 5765 |  | 189 | `airline` agents: imports and the five transfer functions | 4.11 | 4.10 | 0.698 |
| ns | 5958 |  | 193 | `airline` agents: per-agent tool lists (the handoff graph) | 4.12 | 4.10 | 0.679 |
| ns | 6043 |  | 85 | `airline/configs/tools.py`: complete tool roster | 4.13 |  | 0.673 |
| walker |  | 6049 | 439 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.687 |
| walker |  | 6062 | 13 | Fs::DirListing { dir: examples/airline/evals/eval_cases } |  |  | 0.692 |
| ns | 6286 |  | 243 | `run()` body: state setup and the completion half of the loop | 5.1 | 2.6 | 0.673 |
| walker |  | 6316 | 254 | Code::CodeKey { rung: Body, file: swarm/repl/repl.py, decl: 2, sub: 0, line: 37 } |  |  | 0.673 |
| walker |  | 6331 | 15 | Fs::DirListing { dir: examples/airline/evals/eval_results } |  |  | 0.679 |
| ns | 6479 |  | 193 | `run()` body: tool dispatch, agent switch, and the returned `Response` | 5.2 | 5.1 | 0.665 |
| ns | 6572 |  | 93 | `run()` body: the `stream=True` delegation branch | 5.3 | 2.6 | 0.657 |
| walker |  | 6667 | 336 | Code::CodeKey { rung: Body, file: swarm/core.py, decl: 3, sub: 0, line: 32 } |  |  | 0.660 |
| ns | 6783 |  | 211 | `handle_tool_calls` body: dispatch table and the missing-tool path | 5.4 | 2.8 | 0.647 |
| ns | 7054 |  | 271 | `handle_tool_calls` body: context injection and `Result` merging | 5.5 | 5.4 | 0.631 |
| walker |  | 7223 | 556 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.636 |
| ns | 7265 |  | 211 | `get_chat_completion` body: instructions, tool schemas, context hiding | 5.6 | 2.8 | 0.645 |
| ns | 7390 |  | 125 | `get_chat_completion` body: the Chat Completions request | 5.7 | 5.6 | 0.651 |
| ns | 7565 |  | 175 | `handle_function_result` body: the return-value coercion rules | 5.8 | 2.8 | 0.658 |
| walker |  | 7771 | 548 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.665 |
| ns | 7894 |  | 329 | `function_to_json` body: type map, signature inspection, `required` | 5.9 | 2.4 | 0.645 |
| ns | 8003 |  | 109 | `function_to_json` body: the emitted schema shape | 5.10 | 5.9 | 0.638 |
| ns | 8173 |  | 170 | `swarm/util.py`: the streaming merge helpers | 5.11 | 2.4 | 0.642 |
| walker |  | 8294 | 523 | Markdown::Section { file: README.md, section_index: 10, keeps_default_concavity: false } |  |  | 0.652 |
| ns | 8412 |  | 239 | `run_demo_loop` body: the reference conversation loop | 5.12 | 2.5 | 0.662 |
| walker |  | 8506 | 212 | Markdown::Section { file: README.md, section_index: 11, keeps_default_concavity: false } |  |  | 0.668 |
| ns | 8646 |  | 234 | `run_and_stream` body: the streaming protocol, with elisions marked | 5.13 | 2.7 | 0.654 |
| ns | 8875 |  | 229 | `tests/test_core.py`: complete test roster and the shared fixture | 6.1 |  | 0.645 |
| walker |  | 8947 | 441 | Code::CodeKey { rung: Body, file: swarm/util.py, decl: 4, sub: 0, line: 31 } |  |  | 0.687 |
| ns | 9032 |  | 157 | `tests/mock_client.py`: the fake OpenAI client | 6.2 |  | 0.681 |
| ns | 9134 |  | 102 | `tests/test_util.py`: both schema-conversion tests | 6.3 |  | 0.678 |
| walker |  | 9289 | 342 | Code::CodeKey { rung: Body, file: swarm/repl/repl.py, decl: 1, sub: 0, line: 6 } |  |  | 0.678 |
| walker |  | 9333 | 44 | Fs::DirListing { dir: examples/customer_service_lite/logs } |  |  | 0.678 |
| ns | 9340 |  | 206 | `setup.cfg`: package metadata and the complete dependency list | 6.4 |  | 0.684 |
| ns | 9515 |  | 175 | Build backend and formatting toolchain | 6.5 |  | 0.679 |
| ns | 9649 |  | 134 | `customer_service_streaming/src` and `configs`: complete listings | 7.1 |  | 0.687 |
| walker |  | 9815 | 482 | Code::CodeKey { rung: Body, file: swarm/core.py, decl: 5, sub: 0, line: 89 } |  |  | 0.722 |
| walker |  | 9866 | 51 | Markdown::ReadmeHeadline { file: examples/weather_agent/README.md } |  |  | 0.722 |
| ns | 9869 |  | 220 | The legacy example's own `Swarm` class and its config knobs | 7.2 | 7.1 | 0.712 |
| walker |  | 9883 | 17 | Markdown::HeadingsOutline { file: examples/weather_agent/README.md } |  |  | 0.712 |
| walker |  | 9894 | 11 | Code::CodeKey { rung: Names, file: examples/customer_service_streaming/main.py, decl: 0, sub: 0, line: 0 } |  |  | 0.713 |
| ns | 9894 |  | 25 | Remaining asset and log directories | 7.3 |  | 0.713 |
| walker |  | 9949 | 55 | Fs::DirListing { dir: examples/customer_service/logs } |  |  | 0.713 |
| walker |  | 9961 | 12 | Code::CodeKey { rung: Names, file: examples/airline/main.py, decl: 0, sub: 0, line: 0 } |  |  | 0.713 |
| walker |  | 9985 | 24 | Code::CodeKey { rung: Names, file: tests/test_util.py, decl: 0, sub: 0, line: 0 } |  |  | 0.714 |
