Score(3000)=0.721 I=0.834 C=0.623 ns_rows≤3K=25/66 grid(1000/1442/2080/3000/4327/6240/9000)=0.446/0.618/0.620/0.721/0.742/0.676/0.654

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 37 | 37 | listing of '.' |  |  | 0.000 |
| walker |  | 51 | 14 | listing of 'assets' |  |  | 0.000 |
| walker |  | 67 | 16 | README headline in README.md |  |  | 0.000 |
| ns | 87 |  | 87 | README title + deprecation callout | 1.1 |  | 0.116 |
| walker |  | 88 | 21 | listing of 'swarm' |  |  | 0.123 |
| walker |  | 98 | 10 | listing of 'swarm/repl' |  |  | 0.129 |
| walker |  | 111 | 13 | python names swarm/repl/__init__.py |  |  | 0.129 |
| ns | 124 |  | 37 | Repository root listing (complete) | 1.2 |  | 0.572 |
| walker |  | 157 | 46 | python names swarm/__init__.py |  |  | 0.601 |
| walker |  | 169 | 12 | python names swarm/core.py |  |  | 0.601 |
| walker |  | 184 | 15 | python module doc swarm/core.py |  |  | 0.602 |
| ns | 211 |  | 87 | README Overview: agents and handoffs | 1.3 |  | 0.542 |
| ns | 242 |  | 31 | `swarm/` package listing (complete, incl. `repl/`) | 1.4 |  | 0.564 |
| walker |  | 264 | 80 | python decl swarm/core.py:26 |  |  | 0.579 |
| walker |  | 296 | 32 | manifest config in pyproject.toml |  |  | 0.579 |
| ns | 301 |  | 59 | Public exports of `swarm` and `swarm.repl` | 1.5 | 1.4 | 0.585 |
| walker |  | 359 | 63 | python decl swarm/core.py:89 |  |  | 0.590 |
| ns | 399 |  | 98 | README: expressive power, and the statelessness NOTE | 1.6 | 1.3 | 0.558 |
| walker |  | 433 | 74 | python decl swarm/core.py:32 |  |  | 0.571 |
| ns | 474 |  | 75 | `examples/` and `tests/` directory listings (complete) | 1.7 |  | 0.458 |
| walker |  | 496 | 63 | python names swarm/types.py |  |  | 0.461 |
| walker |  | 529 | 33 | python decl swarm/types.py:23 |  |  | 0.464 |
| walker |  | 564 | 35 | python decl swarm/types.py:29 |  |  | 0.470 |
| walker |  | 591 | 27 | listing of 'tests' |  |  | 0.490 |
| walker |  | 663 | 72 | python names swarm/util.py |  |  | 0.495 |
| ns | 684 |  | 210 | README section-heading roster (all remaining headings) | 1.8 | 1.1 | 0.400 |
| walker |  | 755 | 92 | python decl swarm/core.py:139 |  |  | 0.407 |
| walker |  | 798 | 43 | python names swarm/repl/repl.py |  |  | 0.409 |
| ns | 815 |  | 131 | `Agent` model: every field with its default | 2.1 |  | 0.385 |
| walker |  | 824 | 26 | python decl swarm/repl/repl.py:60 |  |  | 0.388 |
| ns | 910 |  | 95 | `Swarm` class: complete method roster | 2.2 |  | 0.418 |
| walker |  | 930 | 106 | python decl swarm/core.py:231 |  |  | 0.427 |
| ns | 998 |  | 88 | `Response` and `Result` models: every field | 2.3 | 2.1 | 0.446 |
| walker |  | 1018 | 88 | python decl swarm/types.py:14 |  |  | 0.503 |
| ns | 1070 |  | 72 | `swarm/util.py`: complete function roster | 2.4 |  | 0.510 |
| ns | 1139 |  | 69 | `swarm/repl/repl.py`: complete function roster | 2.5 |  | 0.517 |
| ns | 1245 |  | 106 | `Swarm.run()` full signature with defaults | 2.6 | 2.2 | 0.530 |
| ns | 1337 |  | 92 | `run_and_stream()` full signature | 2.7 | 2.2 | 0.539 |
| walker |  | 1359 | 341 | listing of 'logs' |  |  | 0.539 |
| walker |  | 1407 | 48 | listing of 'examples' |  |  | 0.618 |
| walker |  | 1424 | 17 | listing of 'examples/weather_agent' |  |  | 0.618 |
| ns | 1482 |  | 145 | Internal `Swarm` method signatures: completion + tool dispatch | 2.8 | 2.2 | 0.624 |
| ns | 1574 |  | 92 | `swarm/types.py` imports: pydantic + reused OpenAI types | 2.9 |  | 0.602 |
| walker |  | 1617 | 193 | headings outline in README.md |  |  | 0.677 |
| walker |  | 1640 | 23 | listing of 'examples/triage_agent' |  |  | 0.679 |
| walker |  | 1660 | 20 | listing of 'examples/personal_shopper' |  |  | 0.679 |
| walker |  | 1686 | 26 | listing of 'examples/airline' |  |  | 0.680 |
| walker |  | 1689 | 3 | listing of 'examples/airline/data' |  |  | 0.680 |
| walker |  | 1701 | 12 | listing of 'examples/airline/data/routines' |  |  | 0.681 |
| walker |  | 1717 | 16 | listing of 'examples/airline/configs' |  |  | 0.682 |
| ns | 1748 |  | 174 | `swarm/core.py` import block | 2.10 |  | 0.633 |
| walker |  | 1751 | 34 | listing of 'examples/basic' |  |  | 0.636 |
| walker |  | 1785 | 34 | listing of 'examples/customer_service_streaming' |  |  | 0.638 |
| walker |  | 1812 | 27 | listing of 'examples/customer_service_streaming/configs' |  |  | 0.639 |
| walker |  | 1824 | 12 | listing of 'examples/customer_service_streaming/configs/tools' |  |  | 0.639 |
| walker |  | 1858 | 34 | listing of 'examples/customer_service_streaming/src' |  |  | 0.640 |
| walker |  | 1877 | 19 | listing of 'examples/customer_service_streaming/src/swarm' |  |  | 0.640 |
| walker |  | 1891 | 14 | listing of 'examples/customer_service_streaming/src/swarm/engines' |  |  | 0.641 |
| walker |  | 1894 | 3 | listing of 'examples/customer_service' |  |  | 0.642 |
| walker |  | 1897 | 3 | listing of 'examples/customer_service_lite' |  |  | 0.642 |
| ns | 1914 |  | 166 | `client.run()` semantics and the five-step loop | 3.1 |  | 0.625 |
| walker |  | 1972 | 75 | README.md section #11 |  |  | 0.626 |
| walker |  | 2014 | 42 | listing of 'examples/support_bot' |  |  | 0.629 |
| ns | 2068 |  | 154 | `run()` arguments table, part 1 of 2 | 3.2 |  | 0.619 |
| walker |  | 2075 | 61 | python body swarm/util.py:13 |  |  | 0.620 |
| ns | 2242 |  | 174 | `run()` arguments table, part 2 of 2 (completes the table) | 3.3 | 3.2 | 0.610 |
| ns | 2398 |  | 156 | `Response` fields table (complete) | 3.4 |  | 0.601 |
| ns | 2603 |  | 205 | `Agent` fields table (the README's documented subset) | 3.5 |  | 0.589 |
| walker |  | 2672 | 597 | README.md section #0 |  |  | 0.735 |
| walker |  | 2754 | 82 | README.md section #12 |  |  | 0.737 |
| ns | 2766 |  | 163 | Function/tool rules: return values, context, errors, ordering | 3.6 |  | 0.725 |
| walker |  | 2841 | 87 | python doc swarm/types.py:29 |  |  | 0.729 |
| ns | 2897 |  | 131 | Handoffs and `Result`: the documented rules | 3.7 |  | 0.721 |
| ns | 3018 |  | 121 | Function-schema conversion rules | 3.8 |  | 0.709 |
| walker |  | 3109 | 268 | plaintext config setup.cfg |  |  | 0.712 |
| ns | 3172 |  | 154 | Streaming: the two Swarm-specific event types | 3.9 |  | 0.702 |
| walker |  | 3199 | 90 | python body swarm/util.py:5 |  |  | 0.702 |
| walker |  | 3293 | 94 | python body swarm/util.py:21 |  |  | 0.703 |
| ns | 3369 |  | 197 | Examples index: what each example directory demonstrates | 3.10 |  | 0.689 |
| walker |  | 3401 | 108 | python doc swarm/util.py:31 |  |  | 0.689 |
| walker |  | 3428 | 27 | python body swarm/core.py:27 |  |  | 0.689 |
| ns | 3495 |  | 126 | Returning from `run()` and resuming a conversation | 3.11 |  | 0.688 |
| walker |  | 3611 | 183 | README.md section #3 |  |  | 0.690 |
| ns | 3670 |  | 175 | `Agent` definition and `instructions` semantics | 3.12 |  | 0.685 |
| ns | 3847 |  | 177 | Install, client construction, and the `run_demo_loop` util | 3.13 | 1.1 | 0.696 |
| ns | 3992 |  | 145 | "Why Swarm" positioning and the evaluations stance | 3.14 |  | 0.694 |
| walker |  | 4046 | 435 | README.md section #4 |  |  | 0.746 |
| ns | 4066 |  | 74 | Listings for the three simple examples (complete) | 4.1 |  | 0.753 |
| walker |  | 4198 | 152 | README.md section #13 |  |  | 0.753 |
| ns | 4322 |  | 256 | Purpose line of all six example READMEs | 4.2 |  | 0.742 |
| walker |  | 4400 | 202 | README.md section #2 |  |  | 0.761 |
| walker |  | 4411 | 11 | listing of 'tests/test_runs' |  |  | 0.761 |
| walker |  | 4415 | 4 | listing of 'examples/customer_service_streaming/configs/assistants' |  |  | 0.761 |
| walker |  | 4419 | 4 | listing of 'examples/customer_service_streaming/src/runs' |  |  | 0.761 |
| walker |  | 4423 | 4 | listing of 'examples/customer_service_streaming/src/tasks' |  |  | 0.761 |
| walker |  | 4430 | 7 | listing of 'examples/customer_service_streaming/logs' |  |  | 0.761 |
| ns | 4445 |  | 123 | `basic/bare_minimum.py` in full | 4.3 |  | 0.741 |
| ns | 4733 |  | 288 | `triage_agent/agents.py`: three agents and their wiring | 4.4 |  | 0.712 |
| walker |  | 4765 | 335 | README.md section #1 |  |  | 0.743 |
| walker |  | 4770 | 5 | listing of 'examples/customer_service_streaming/src/evals' |  |  | 0.743 |
| walker |  | 4781 | 11 | python names examples/customer_service_streaming/main.py |  |  | 0.743 |
| walker |  | 4785 | 4 | listing of 'examples/airline/data/routines/baggage' |  |  | 0.743 |
| walker |  | 4789 | 4 | listing of 'examples/airline/data/routines/flight_modification' |  |  | 0.744 |
| walker |  | 4793 | 4 | listing of 'examples/customer_service_streaming/configs/assistants/user_interface' |  |  | 0.744 |
| walker |  | 4805 | 12 | python names examples/airline/main.py |  |  | 0.744 |
| walker |  | 4816 | 11 | listing of 'examples/customer_service_streaming/tests' |  |  | 0.744 |
| walker |  | 4840 | 24 | python names tests/test_util.py |  |  | 0.744 |
| ns | 4845 |  | 112 | `airline` example: complete directory tree | 4.5 |  | 0.731 |
| walker |  | 4847 | 7 | listing of 'examples/customer_service_streaming/tests/test_runs' |  |  | 0.731 |
| ns | 4907 |  | 62 | `support_bot` and `personal_shopper` listings (complete) | 4.6 |  | 0.737 |
| ns | 4947 |  | 40 | The three `customer_service*` directories: what is actually there | 4.7 |  | 0.742 |
| walker |  | 5086 | 239 | python body swarm/repl/repl.py:60 |  |  | 0.744 |
| ns | 5149 |  | 202 | `weather_agent/agents.py`: a complete function-calling agent | 4.8 |  | 0.724 |
| ns | 5335 |  | 186 | `basic/context_variables.py`: callable instructions + injected context | 4.9 |  | 0.705 |
| walker |  | 5340 | 254 | python body swarm/repl/repl.py:37 |  |  | 0.705 |
| ns | 5576 |  | 241 | `airline` agents: the five agents and their instruction sources | 4.10 |  | 0.686 |
| walker |  | 5698 | 358 | README.md section #5 |  |  | 0.703 |
| walker |  | 5710 | 12 | python names examples/customer_service_streaming/src/arg_parser.py |  |  | 0.703 |
| walker |  | 5729 | 19 | listing of 'examples/airline/evals' |  |  | 0.713 |
| ns | 5765 |  | 189 | `airline` agents: imports and the five transfer functions | 4.11 | 4.10 | 0.697 |
| ns | 5958 |  | 193 | `airline` agents: per-agent tool lists (the handoff graph) | 4.12 | 4.10 | 0.678 |
| ns | 6043 |  | 85 | `airline/configs/tools.py`: complete tool roster | 4.13 |  | 0.672 |
| walker |  | 6170 | 441 | python body swarm/util.py:31 |  |  | 0.676 |
| ns | 6286 |  | 243 | `run()` body: state setup and the completion half of the loop | 5.1 | 2.6 | 0.658 |
| ns | 6479 |  | 193 | `run()` body: tool dispatch, agent switch, and the returned `Response` | 5.2 | 5.1 | 0.644 |
| walker |  | 6512 | 342 | python body swarm/repl/repl.py:6 |  |  | 0.644 |
| walker |  | 6520 | 8 | listing of 'examples/customer_service_streaming/configs/tools/query_docs' |  |  | 0.644 |
| walker |  | 6528 | 8 | listing of 'examples/customer_service_streaming/configs/tools/send_email' |  |  | 0.644 |
| walker |  | 6536 | 8 | listing of 'examples/customer_service_streaming/configs/tools/submit_ticket' |  |  | 0.644 |
| walker |  | 6545 | 9 | python names examples/customer_service_streaming/src/swarm/conversation.py |  |  | 0.644 |
| ns | 6572 |  | 93 | `run()` body: the `stream=True` delegation branch | 5.3 | 2.6 | 0.637 |
| ns | 6783 |  | 211 | `handle_tool_calls` body: dispatch table and the missing-tool path | 5.4 | 2.8 | 0.624 |
| walker |  | 6984 | 439 | README.md section #6 |  |  | 0.636 |
| walker |  | 6997 | 13 | listing of 'examples/airline/evals/eval_cases' |  |  | 0.641 |
| walker |  | 7012 | 15 | listing of 'examples/airline/evals/eval_results' |  |  | 0.647 |
| walker |  | 7023 | 11 | python names examples/customer_service_streaming/src/runs/run.py |  |  | 0.647 |
| ns | 7054 |  | 271 | `handle_tool_calls` body: context injection and `Result` merging | 5.5 | 5.4 | 0.630 |
| walker |  | 7198 | 175 | python body swarm/core.py:71 |  |  | 0.632 |
| walker |  | 7210 | 12 | python names examples/customer_service_streaming/src/evals/eval_function.py |  |  | 0.632 |
| walker |  | 7222 | 12 | python names examples/customer_service_streaming/src/swarm/swarm.py |  |  | 0.632 |
| ns | 7265 |  | 211 | `get_chat_completion` body: instructions, tool schemas, context hiding | 5.6 | 2.8 | 0.621 |
| ns | 7390 |  | 125 | `get_chat_completion` body: the Chat Completions request | 5.7 | 5.6 | 0.613 |
| ns | 7565 |  | 175 | `handle_function_result` body: the return-value coercion rules | 5.8 | 2.8 | 0.622 |
| walker |  | 7778 | 556 | README.md section #7 |  |  | 0.628 |
| ns | 7894 |  | 329 | `function_to_json` body: type map, signature inspection, `required` | 5.9 | 2.4 | 0.643 |
| ns | 8003 |  | 109 | `function_to_json` body: the emitted schema shape | 5.10 | 5.9 | 0.648 |
| ns | 8173 |  | 170 | `swarm/util.py`: the streaming merge helpers | 5.11 | 2.4 | 0.651 |
| walker |  | 8326 | 548 | README.md section #8 |  |  | 0.657 |
| ns | 8412 |  | 239 | `run_demo_loop` body: the reference conversation loop | 5.12 | 2.5 | 0.667 |
| ns | 8646 |  | 234 | `run_and_stream` body: the streaming protocol, with elisions marked | 5.13 | 2.7 | 0.653 |
| walker |  | 8849 | 523 | README.md section #9 |  |  | 0.663 |
| ns | 8875 |  | 229 | `tests/test_core.py`: complete test roster and the shared fixture | 6.1 |  | 0.654 |
| ns | 9032 |  | 157 | `tests/mock_client.py`: the fake OpenAI client | 6.2 |  | 0.649 |
| walker |  | 9061 | 212 | README.md section #10 |  |  | 0.654 |
| walker |  | 9074 | 13 | python names examples/customer_service_streaming/src/swarm/assistants.py |  |  | 0.654 |
| walker |  | 9125 | 51 | README headline in examples/weather_agent/README.md |  |  | 0.654 |
| ns | 9134 |  | 102 | `tests/test_util.py`: both schema-conversion tests | 6.3 |  | 0.651 |
| walker |  | 9142 | 17 | headings outline in examples/weather_agent/README.md |  |  | 0.651 |
| walker |  | 9165 | 23 | python names examples/customer_service_streaming/src/utils.py |  |  | 0.651 |
| walker |  | 9226 | 61 | README headline in examples/triage_agent/README.md |  |  | 0.652 |
| walker |  | 9243 | 17 | headings outline in examples/triage_agent/README.md |  |  | 0.652 |
| walker |  | 9325 | 82 | python names tests/mock_client.py |  |  | 0.652 |
| ns | 9340 |  | 206 | `setup.cfg`: package metadata and the complete dependency list | 6.4 |  | 0.659 |
| walker |  | 9392 | 67 | python decl tests/mock_client.py:44 |  |  | 0.662 |
| walker |  | 9403 | 11 | python names examples/customer_service_streaming/src/swarm/engines/engine.py |  |  | 0.662 |
| walker |  | 9412 | 9 | plaintext config examples/support_bot/requirements.txt |  |  | 0.662 |
| walker |  | 9456 | 44 | python names examples/weather_agent/agents.py |  |  | 0.662 |
| walker |  | 9495 | 39 | python decl examples/weather_agent/agents.py:19 |  |  | 0.664 |
| ns | 9515 |  | 175 | Build backend and formatting toolchain | 6.5 |  | 0.660 |
| walker |  | 9562 | 67 | README headline in examples/support_bot/README.md |  |  | 0.661 |
| walker |  | 9578 | 16 | headings outline in examples/support_bot/README.md |  |  | 0.661 |
| walker |  | 9590 | 12 | python names examples/customer_service_streaming/src/swarm/engines/local_engine.py |  |  | 0.661 |
| walker |  | 9634 | 44 | listing of 'examples/customer_service_lite/logs' |  |  | 0.661 |
| ns | 9649 |  | 134 | `customer_service_streaming/src` and `configs`: complete listings | 7.1 |  | 0.670 |
| walker |  | 9707 | 73 | README headline in examples/basic/README.md |  |  | 0.671 |
| walker |  | 9725 | 18 | headings outline in examples/basic/README.md |  |  | 0.671 |
| walker |  | 9738 | 13 | python names examples/customer_service_streaming/configs/tools/send_email/handler.py |  |  | 0.671 |
| walker |  | 9751 | 13 | python names examples/customer_service_streaming/src/swarm/engines/assistants_engine.py |  |  | 0.671 |
| walker |  | 9808 | 57 | python names examples/basic/simple_loop_no_helpers.py |  |  | 0.671 |
| walker |  | 9833 | 25 | python decl examples/basic/simple_loop_no_helpers.py:5 |  |  | 0.671 |
| ns | 9869 |  | 220 | The legacy example's own `Swarm` class and its config knobs | 7.2 | 7.1 | 0.662 |
| ns | 9894 |  | 25 | Remaining asset and log directories | 7.3 |  | 0.663 |
| walker |  | 9945 | 112 | python names tests/test_core.py |  |  | 0.666 |
| walker |  | 9951 | 6 | python decl tests/test_core.py:10 |  |  | 0.666 |
| walker |  | 9973 | 22 | python names examples/customer_service_streaming/src/tasks/task.py |  |  | 0.666 |
