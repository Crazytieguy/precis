Score(3000)=0.721 I=0.834 C=0.623 ns_rows≤3K=25/66 grid(1000/1442/2080/3000/4327/6240/9000)=0.446/0.618/0.767/0.721/0.742/0.677/0.652

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
| walker |  | 157 | 46 | Code::CodeKey { rung: Names, file: swarm/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.601 |
| walker |  | 169 | 12 | Code::CodeKey { rung: Names, file: swarm/core.py, decl: 0, sub: 0, line: 0 } |  |  | 0.601 |
| walker |  | 184 | 15 | Code::CodeKey { rung: ModuleDoc, file: swarm/core.py, decl: 0, sub: 0, line: 0 } |  |  | 0.602 |
| ns | 211 |  | 87 | README Overview: agents and handoffs | 1.3 |  | 0.542 |
| ns | 242 |  | 31 | `swarm/` package listing (complete, incl. `repl/`) | 1.4 |  | 0.564 |
| walker |  | 264 | 80 | Code::CodeKey { rung: Decl, file: swarm/core.py, decl: 1, sub: 0, line: 26 } |  |  | 0.579 |
| walker |  | 296 | 32 | Toml::Config { file: pyproject.toml } |  |  | 0.579 |
| ns | 301 |  | 59 | Public exports of `swarm` and `swarm.repl` | 1.5 | 1.4 | 0.585 |
| walker |  | 359 | 63 | Code::CodeKey { rung: Decl, file: swarm/core.py, decl: 5, sub: 0, line: 89 } |  |  | 0.590 |
| ns | 399 |  | 98 | README: expressive power, and the statelessness NOTE | 1.6 | 1.3 | 0.558 |
| walker |  | 433 | 74 | Code::CodeKey { rung: Decl, file: swarm/core.py, decl: 3, sub: 0, line: 32 } |  |  | 0.571 |
| ns | 474 |  | 75 | `examples/` and `tests/` directory listings (complete) | 1.7 |  | 0.458 |
| walker |  | 496 | 63 | Code::CodeKey { rung: Names, file: swarm/types.py, decl: 0, sub: 0, line: 0 } |  |  | 0.461 |
| walker |  | 529 | 33 | Code::CodeKey { rung: Decl, file: swarm/types.py, decl: 3, sub: 0, line: 23 } |  |  | 0.464 |
| walker |  | 564 | 35 | Code::CodeKey { rung: Decl, file: swarm/types.py, decl: 4, sub: 0, line: 29 } |  |  | 0.470 |
| walker |  | 591 | 27 | Fs::DirListing { dir: tests } |  |  | 0.490 |
| walker |  | 663 | 72 | Code::CodeKey { rung: Names, file: swarm/util.py, decl: 0, sub: 0, line: 0 } |  |  | 0.495 |
| ns | 684 |  | 210 | README section-heading roster (all remaining headings) | 1.8 | 1.1 | 0.400 |
| walker |  | 755 | 92 | Code::CodeKey { rung: Decl, file: swarm/core.py, decl: 6, sub: 0, line: 139 } |  |  | 0.407 |
| walker |  | 798 | 43 | Code::CodeKey { rung: Names, file: swarm/repl/repl.py, decl: 0, sub: 0, line: 0 } |  |  | 0.409 |
| ns | 815 |  | 131 | `Agent` model: every field with its default | 2.1 |  | 0.385 |
| walker |  | 824 | 26 | Code::CodeKey { rung: Decl, file: swarm/repl/repl.py, decl: 3, sub: 0, line: 60 } |  |  | 0.388 |
| ns | 910 |  | 95 | `Swarm` class: complete method roster | 2.2 |  | 0.418 |
| walker |  | 930 | 106 | Code::CodeKey { rung: Decl, file: swarm/core.py, decl: 7, sub: 0, line: 231 } |  |  | 0.427 |
| ns | 998 |  | 88 | `Response` and `Result` models: every field | 2.3 | 2.1 | 0.446 |
| walker |  | 1018 | 88 | Code::CodeKey { rung: Decl, file: swarm/types.py, decl: 2, sub: 0, line: 14 } |  |  | 0.503 |
| ns | 1070 |  | 72 | `swarm/util.py`: complete function roster | 2.4 |  | 0.510 |
| ns | 1139 |  | 69 | `swarm/repl/repl.py`: complete function roster | 2.5 |  | 0.517 |
| ns | 1245 |  | 106 | `Swarm.run()` full signature with defaults | 2.6 | 2.2 | 0.530 |
| ns | 1337 |  | 92 | `run_and_stream()` full signature | 2.7 | 2.2 | 0.539 |
| walker |  | 1359 | 341 | Fs::DirListing { dir: logs } |  |  | 0.539 |
| walker |  | 1407 | 48 | Fs::DirListing { dir: examples } |  |  | 0.618 |
| walker |  | 1424 | 17 | Fs::DirListing { dir: examples/weather_agent } |  |  | 0.618 |
| ns | 1482 |  | 145 | Internal `Swarm` method signatures: completion + tool dispatch | 2.8 | 2.2 | 0.624 |
| ns | 1574 |  | 92 | `swarm/types.py` imports: pydantic + reused OpenAI types | 2.9 |  | 0.602 |
| walker |  | 1617 | 193 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.677 |
| ns | 1748 |  | 174 | `swarm/core.py` import block | 2.10 |  | 0.628 |
| walker |  | 1808 | 191 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.792 |
| walker |  | 1831 | 23 | Fs::DirListing { dir: examples/triage_agent } |  |  | 0.793 |
| walker |  | 1851 | 20 | Fs::DirListing { dir: examples/personal_shopper } |  |  | 0.793 |
| walker |  | 1877 | 26 | Fs::DirListing { dir: examples/airline } |  |  | 0.794 |
| walker |  | 1880 | 3 | Fs::DirListing { dir: examples/airline/data } |  |  | 0.794 |
| walker |  | 1892 | 12 | Fs::DirListing { dir: examples/airline/data/routines } |  |  | 0.794 |
| walker |  | 1908 | 16 | Fs::DirListing { dir: examples/airline/configs } |  |  | 0.795 |
| ns | 1914 |  | 166 | `client.run()` semantics and the five-step loop | 3.1 |  | 0.775 |
| walker |  | 1942 | 34 | Fs::DirListing { dir: examples/basic } |  |  | 0.778 |
| walker |  | 1976 | 34 | Fs::DirListing { dir: examples/customer_service_streaming } |  |  | 0.779 |
| walker |  | 2003 | 27 | Fs::DirListing { dir: examples/customer_service_streaming/configs } |  |  | 0.779 |
| walker |  | 2015 | 12 | Fs::DirListing { dir: examples/customer_service_streaming/configs/tools } |  |  | 0.780 |
| ns | 2068 |  | 154 | `run()` arguments table, part 1 of 2 | 3.2 |  | 0.767 |
| ns | 2242 |  | 174 | `run()` arguments table, part 2 of 2 (completes the table) | 3.3 | 3.2 | 0.756 |
| ns | 2398 |  | 156 | `Response` fields table (complete) | 3.4 |  | 0.744 |
| walker |  | 2416 | 401 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.744 |
| walker |  | 2443 | 27 | Code::CodeKey { rung: Body, file: swarm/core.py, decl: 2, sub: 0, line: 27 } |  |  | 0.744 |
| walker |  | 2477 | 34 | Fs::DirListing { dir: examples/customer_service_streaming/src } |  |  | 0.745 |
| walker |  | 2496 | 19 | Fs::DirListing { dir: examples/customer_service_streaming/src/swarm } |  |  | 0.746 |
| walker |  | 2510 | 14 | Fs::DirListing { dir: examples/customer_service_streaming/src/swarm/engines } |  |  | 0.746 |
| walker |  | 2513 | 3 | Fs::DirListing { dir: examples/customer_service } |  |  | 0.746 |
| walker |  | 2516 | 3 | Fs::DirListing { dir: examples/customer_service_lite } |  |  | 0.747 |
| walker |  | 2591 | 75 | Markdown::Section { file: README.md, section_index: 12, keeps_default_concavity: false } |  |  | 0.747 |
| ns | 2603 |  | 205 | `Agent` fields table (the README's documented subset) | 3.5 |  | 0.732 |
| walker |  | 2633 | 42 | Fs::DirListing { dir: examples/support_bot } |  |  | 0.735 |
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
| ns | 3495 |  | 126 | Returning from `run()` and resuming a conversation | 3.11 |  | 0.688 |
| walker |  | 3606 | 183 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.690 |
| ns | 3670 |  | 175 | `Agent` definition and `instructions` semantics | 3.12 |  | 0.685 |
| ns | 3847 |  | 177 | Install, client construction, and the `run_demo_loop` util | 3.13 | 1.1 | 0.696 |
| ns | 3992 |  | 145 | "Why Swarm" positioning and the evaluations stance | 3.14 |  | 0.694 |
| walker |  | 4041 | 435 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: true } |  |  | 0.746 |
| ns | 4066 |  | 74 | Listings for the three simple examples (complete) | 4.1 |  | 0.753 |
| walker |  | 4193 | 152 | Markdown::Section { file: README.md, section_index: 14, keeps_default_concavity: false } |  |  | 0.753 |
| ns | 4322 |  | 256 | Purpose line of all six example READMEs | 4.2 |  | 0.742 |
| walker |  | 4395 | 202 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.761 |
| walker |  | 4406 | 11 | Fs::DirListing { dir: tests/test_runs } |  |  | 0.761 |
| walker |  | 4410 | 4 | Fs::DirListing { dir: examples/customer_service_streaming/configs/assistants } |  |  | 0.761 |
| walker |  | 4414 | 4 | Fs::DirListing { dir: examples/customer_service_streaming/src/runs } |  |  | 0.761 |
| walker |  | 4418 | 4 | Fs::DirListing { dir: examples/customer_service_streaming/src/tasks } |  |  | 0.761 |
| walker |  | 4425 | 7 | Fs::DirListing { dir: examples/customer_service_streaming/logs } |  |  | 0.761 |
| ns | 4445 |  | 123 | `basic/bare_minimum.py` in full | 4.3 |  | 0.741 |
| ns | 4733 |  | 288 | `triage_agent/agents.py`: three agents and their wiring | 4.4 |  | 0.712 |
| walker |  | 4760 | 335 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.743 |
| walker |  | 4765 | 5 | Fs::DirListing { dir: examples/customer_service_streaming/src/evals } |  |  | 0.743 |
| walker |  | 4776 | 11 | Code::CodeKey { rung: Names, file: examples/customer_service_streaming/main.py, decl: 0, sub: 0, line: 0 } |  |  | 0.743 |
| walker |  | 4780 | 4 | Fs::DirListing { dir: examples/airline/data/routines/baggage } |  |  | 0.743 |
| walker |  | 4784 | 4 | Fs::DirListing { dir: examples/airline/data/routines/flight_modification } |  |  | 0.744 |
| walker |  | 4788 | 4 | Fs::DirListing { dir: examples/customer_service_streaming/configs/assistants/user_interface } |  |  | 0.744 |
| walker |  | 4800 | 12 | Code::CodeKey { rung: Names, file: examples/airline/main.py, decl: 0, sub: 0, line: 0 } |  |  | 0.744 |
| ns | 4845 |  | 112 | `airline` example: complete directory tree | 4.5 |  | 0.731 |
| ns | 4907 |  | 62 | `support_bot` and `personal_shopper` listings (complete) | 4.6 |  | 0.737 |
| ns | 4947 |  | 40 | The three `customer_service*` directories: what is actually there | 4.7 |  | 0.741 |
| ns | 5149 |  | 202 | `weather_agent/agents.py`: a complete function-calling agent | 4.8 |  | 0.721 |
| walker |  | 5158 | 358 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.739 |
| walker |  | 5169 | 11 | Fs::DirListing { dir: examples/customer_service_streaming/tests } |  |  | 0.739 |
| walker |  | 5193 | 24 | Code::CodeKey { rung: Names, file: tests/test_util.py, decl: 0, sub: 0, line: 0 } |  |  | 0.739 |
| walker |  | 5200 | 7 | Fs::DirListing { dir: examples/customer_service_streaming/tests/test_runs } |  |  | 0.739 |
| ns | 5335 |  | 186 | `basic/context_variables.py`: callable instructions + injected context | 4.9 |  | 0.720 |
| walker |  | 5375 | 175 | Code::CodeKey { rung: Body, file: swarm/core.py, decl: 4, sub: 0, line: 71 } |  |  | 0.722 |
| ns | 5576 |  | 241 | `airline` agents: the five agents and their instruction sources | 4.10 |  | 0.703 |
| walker |  | 5614 | 239 | Code::CodeKey { rung: Body, file: swarm/repl/repl.py, decl: 3, sub: 0, line: 60 } |  |  | 0.705 |
| ns | 5765 |  | 189 | `airline` agents: imports and the five transfer functions | 4.11 | 4.10 | 0.689 |
| ns | 5958 |  | 193 | `airline` agents: per-agent tool lists (the handoff graph) | 4.12 | 4.10 | 0.670 |
| ns | 6043 |  | 85 | `airline/configs/tools.py`: complete tool roster | 4.13 |  | 0.664 |
| walker |  | 6053 | 439 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.677 |
| ns | 6286 |  | 243 | `run()` body: state setup and the completion half of the loop | 5.1 | 2.6 | 0.659 |
| walker |  | 6307 | 254 | Code::CodeKey { rung: Body, file: swarm/repl/repl.py, decl: 2, sub: 0, line: 37 } |  |  | 0.659 |
| walker |  | 6319 | 12 | Code::CodeKey { rung: Names, file: examples/customer_service_streaming/src/arg_parser.py, decl: 0, sub: 0, line: 0 } |  |  | 0.659 |
| ns | 6479 |  | 193 | `run()` body: tool dispatch, agent switch, and the returned `Response` | 5.2 | 5.1 | 0.645 |
| ns | 6572 |  | 93 | `run()` body: the `stream=True` delegation branch | 5.3 | 2.6 | 0.638 |
| ns | 6783 |  | 211 | `handle_tool_calls` body: dispatch table and the missing-tool path | 5.4 | 2.8 | 0.625 |
| walker |  | 6875 | 556 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.631 |
| ns | 7054 |  | 271 | `handle_tool_calls` body: context injection and `Result` merging | 5.5 | 5.4 | 0.615 |
| ns | 7265 |  | 211 | `get_chat_completion` body: instructions, tool schemas, context hiding | 5.6 | 2.8 | 0.605 |
| ns | 7390 |  | 125 | `get_chat_completion` body: the Chat Completions request | 5.7 | 5.6 | 0.597 |
| walker |  | 7423 | 548 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.605 |
| ns | 7565 |  | 175 | `handle_function_result` body: the return-value coercion rules | 5.8 | 2.8 | 0.614 |
| ns | 7894 |  | 329 | `function_to_json` body: type map, signature inspection, `required` | 5.9 | 2.4 | 0.596 |
| walker |  | 7946 | 523 | Markdown::Section { file: README.md, section_index: 10, keeps_default_concavity: false } |  |  | 0.607 |
| ns | 8003 |  | 109 | `function_to_json` body: the emitted schema shape | 5.10 | 5.9 | 0.600 |
| walker |  | 8158 | 212 | Markdown::Section { file: README.md, section_index: 11, keeps_default_concavity: false } |  |  | 0.607 |
| ns | 8173 |  | 170 | `swarm/util.py`: the streaming merge helpers | 5.11 | 2.4 | 0.612 |
| walker |  | 8177 | 19 | Fs::DirListing { dir: examples/airline/evals } |  |  | 0.619 |
| ns | 8412 |  | 239 | `run_demo_loop` body: the reference conversation loop | 5.12 | 2.5 | 0.630 |
| walker |  | 8618 | 441 | Code::CodeKey { rung: Body, file: swarm/util.py, decl: 4, sub: 0, line: 31 } |  |  | 0.674 |
| ns | 8646 |  | 234 | `run_and_stream` body: the streaming protocol, with elisions marked | 5.13 | 2.7 | 0.661 |
| ns | 8875 |  | 229 | `tests/test_core.py`: complete test roster and the shared fixture | 6.1 |  | 0.652 |
| walker |  | 8960 | 342 | Code::CodeKey { rung: Body, file: swarm/repl/repl.py, decl: 1, sub: 0, line: 6 } |  |  | 0.652 |
| walker |  | 8968 | 8 | Fs::DirListing { dir: examples/customer_service_streaming/configs/tools/query_docs } |  |  | 0.652 |
| walker |  | 8976 | 8 | Fs::DirListing { dir: examples/customer_service_streaming/configs/tools/send_email } |  |  | 0.652 |
| walker |  | 8984 | 8 | Fs::DirListing { dir: examples/customer_service_streaming/configs/tools/submit_ticket } |  |  | 0.652 |
| walker |  | 8993 | 9 | Code::CodeKey { rung: Names, file: examples/customer_service_streaming/src/swarm/conversation.py, decl: 0, sub: 0, line: 0 } |  |  | 0.652 |
| walker |  | 9006 | 13 | Fs::DirListing { dir: examples/airline/evals/eval_cases } |  |  | 0.655 |
| walker |  | 9021 | 15 | Fs::DirListing { dir: examples/airline/evals/eval_results } |  |  | 0.659 |
| walker |  | 9032 | 11 | Code::CodeKey { rung: Names, file: examples/customer_service_streaming/src/runs/run.py, decl: 0, sub: 0, line: 0 } |  |  | 0.654 |
| ns | 9032 |  | 157 | `tests/mock_client.py`: the fake OpenAI client | 6.2 |  | 0.654 |
| walker |  | 9044 | 12 | Code::CodeKey { rung: Names, file: examples/customer_service_streaming/src/evals/eval_function.py, decl: 0, sub: 0, line: 0 } |  |  | 0.654 |
| walker |  | 9056 | 12 | Code::CodeKey { rung: Names, file: examples/customer_service_streaming/src/swarm/swarm.py, decl: 0, sub: 0, line: 0 } |  |  | 0.654 |
| ns | 9134 |  | 102 | `tests/test_util.py`: both schema-conversion tests | 6.3 |  | 0.651 |
| ns | 9340 |  | 206 | `setup.cfg`: package metadata and the complete dependency list | 6.4 |  | 0.658 |
| walker |  | 9392 | 336 | Code::CodeKey { rung: Body, file: swarm/core.py, decl: 3, sub: 0, line: 32 } |  |  | 0.684 |
| walker |  | 9405 | 13 | Code::CodeKey { rung: Names, file: examples/customer_service_streaming/src/swarm/assistants.py, decl: 0, sub: 0, line: 0 } |  |  | 0.684 |
| walker |  | 9456 | 51 | Markdown::ReadmeHeadline { file: examples/weather_agent/README.md } |  |  | 0.685 |
| walker |  | 9473 | 17 | Markdown::HeadingsOutline { file: examples/weather_agent/README.md } |  |  | 0.685 |
| walker |  | 9496 | 23 | Code::CodeKey { rung: Names, file: examples/customer_service_streaming/src/utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.685 |
| ns | 9515 |  | 175 | Build backend and formatting toolchain | 6.5 |  | 0.680 |
| walker |  | 9557 | 61 | Markdown::ReadmeHeadline { file: examples/triage_agent/README.md } |  |  | 0.680 |
| walker |  | 9574 | 17 | Markdown::HeadingsOutline { file: examples/triage_agent/README.md } |  |  | 0.680 |
| ns | 9649 |  | 134 | `customer_service_streaming/src` and `configs`: complete listings | 7.1 |  | 0.689 |
| walker |  | 9656 | 82 | Code::CodeKey { rung: Names, file: tests/mock_client.py, decl: 0, sub: 0, line: 0 } |  |  | 0.689 |
| walker |  | 9723 | 67 | Code::CodeKey { rung: Decl, file: tests/mock_client.py, decl: 2, sub: 0, line: 44 } |  |  | 0.691 |
| walker |  | 9734 | 11 | Code::CodeKey { rung: Names, file: examples/customer_service_streaming/src/swarm/engines/engine.py, decl: 0, sub: 0, line: 0 } |  |  | 0.691 |
| walker |  | 9743 | 9 | Plaintext::Whole { file: examples/support_bot/requirements.txt } |  |  | 0.691 |
| walker |  | 9787 | 44 | Code::CodeKey { rung: Names, file: examples/weather_agent/agents.py, decl: 0, sub: 0, line: 0 } |  |  | 0.691 |
| walker |  | 9826 | 39 | Code::CodeKey { rung: Decl, file: examples/weather_agent/agents.py, decl: 3, sub: 0, line: 19 } |  |  | 0.693 |
| ns | 9869 |  | 220 | The legacy example's own `Swarm` class and its config knobs | 7.2 | 7.1 | 0.684 |
| walker |  | 9893 | 67 | Markdown::ReadmeHeadline { file: examples/support_bot/README.md } |  |  | 0.684 |
| ns | 9894 |  | 25 | Remaining asset and log directories | 7.3 |  | 0.685 |
| walker |  | 9909 | 16 | Markdown::HeadingsOutline { file: examples/support_bot/README.md } |  |  | 0.685 |
| walker |  | 9921 | 12 | Code::CodeKey { rung: Names, file: examples/customer_service_streaming/src/swarm/engines/local_engine.py, decl: 0, sub: 0, line: 0 } |  |  | 0.685 |
| walker |  | 9965 | 44 | Fs::DirListing { dir: examples/customer_service_lite/logs } |  |  | 0.685 |
