Score(3000)=0.695 I=0.821 C=0.588 ns_rows≤3K=25/66 grid(1000/1442/2080/3000/4327/6240/9000)=0.513/0.591/0.613/0.695/0.773/0.695/0.571

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 37 | 37 | listing of '.' |  |  | 0.000 |
| walker |  | 51 | 14 | listing of 'assets' |  |  | 0.000 |
| walker |  | 67 | 16 | README headline in README.md |  |  | 0.000 |
| ns | 87 |  | 87 | README title + deprecation callout | 1.1 |  | 0.116 |
| walker |  | 88 | 21 | listing of 'swarm' |  |  | 0.123 |
| ns | 124 |  | 37 | Repository root listing (complete) | 1.2 |  | 0.545 |
| walker |  | 134 | 46 | python imports in swarm/__init__.py |  |  | 0.565 |
| walker |  | 144 | 10 | listing of 'swarm/repl' |  |  | 0.589 |
| walker |  | 157 | 13 | python imports in swarm/repl/__init__.py |  |  | 0.601 |
| walker |  | 189 | 32 | manifest config in pyproject.toml |  |  | 0.601 |
| ns | 211 |  | 87 | README Overview: agents and handoffs | 1.3 |  | 0.542 |
| walker |  | 216 | 27 | listing of 'tests' |  |  | 0.547 |
| ns | 242 |  | 31 | `swarm/` package listing (complete, incl. `repl/`) | 1.4 |  | 0.568 |
| ns | 301 |  | 59 | Public exports of `swarm` and `swarm.repl` | 1.5 | 1.4 | 0.574 |
| ns | 399 |  | 98 | README: expressive power, and the statelessness NOTE | 1.6 | 1.3 | 0.543 |
| ns | 474 |  | 75 | `examples/` and `tests/` directory listings (complete) | 1.7 |  | 0.451 |
| walker |  | 557 | 341 | listing of 'logs' |  |  | 0.451 |
| walker |  | 605 | 48 | listing of 'examples' |  |  | 0.603 |
| walker |  | 622 | 17 | listing of 'examples/weather_agent' |  |  | 0.604 |
| ns | 684 |  | 210 | README section-heading roster (all remaining headings) | 1.8 | 1.1 | 0.488 |
| walker |  | 815 | 193 | headings outline in README.md |  |  | 0.575 |
| ns | 815 |  | 131 | `Agent` model: every field with its default | 2.1 |  | 0.575 |
| ns | 910 |  | 95 | `Swarm` class: complete method roster | 2.2 |  | 0.543 |
| ns | 998 |  | 88 | `Response` and `Result` models: every field | 2.3 | 2.1 | 0.513 |
| ns | 1070 |  | 72 | `swarm/util.py`: complete function roster | 2.4 |  | 0.501 |
| ns | 1139 |  | 69 | `swarm/repl/repl.py`: complete function roster | 2.5 |  | 0.487 |
| ns | 1245 |  | 106 | `Swarm.run()` full signature with defaults | 2.6 | 2.2 | 0.459 |
| ns | 1337 |  | 92 | `run_and_stream()` full signature | 2.7 | 2.2 | 0.438 |
| walker |  | 1412 | 597 | README.md section #0 |  |  | 0.590 |
| walker |  | 1435 | 23 | listing of 'examples/triage_agent' |  |  | 0.591 |
| walker |  | 1455 | 20 | listing of 'examples/personal_shopper' |  |  | 0.591 |
| walker |  | 1481 | 26 | listing of 'examples/airline' |  |  | 0.592 |
| ns | 1482 |  | 145 | Internal `Swarm` method signatures: completion + tool dispatch | 2.8 | 2.2 | 0.550 |
| walker |  | 1484 | 3 | listing of 'examples/airline/data' |  |  | 0.550 |
| walker |  | 1496 | 12 | listing of 'examples/airline/data/routines' |  |  | 0.550 |
| walker |  | 1512 | 16 | listing of 'examples/airline/configs' |  |  | 0.551 |
| walker |  | 1530 | 18 | python imports in swarm/util.py |  |  | 0.551 |
| walker |  | 1564 | 34 | listing of 'examples/basic' |  |  | 0.553 |
| ns | 1574 |  | 92 | `swarm/types.py` imports: pydantic + reused OpenAI types | 2.9 |  | 0.534 |
| walker |  | 1598 | 34 | listing of 'examples/customer_service_streaming' |  |  | 0.535 |
| walker |  | 1625 | 27 | listing of 'examples/customer_service_streaming/configs' |  |  | 0.536 |
| walker |  | 1637 | 12 | listing of 'examples/customer_service_streaming/configs/tools' |  |  | 0.536 |
| walker |  | 1649 | 12 | python decl names surface in swarm/core.py |  |  | 0.536 |
| walker |  | 1649 | 0 | python decl at swarm/core.py:26 |  |  | 0.536 |
| walker |  | 1729 | 80 | python method sigs in swarm/core.py |  |  | 0.569 |
| walker |  | 1729 | 0 | python method at swarm/core.py:27 |  |  | 0.569 |
| walker |  | 1729 | 0 | python method at swarm/core.py:71 |  |  | 0.569 |
| ns | 1748 |  | 174 | `swarm/core.py` import block | 2.10 |  | 0.528 |
| walker |  | 1792 | 63 | python method at swarm/core.py:89 |  |  | 0.548 |
| walker |  | 1866 | 74 | python method at swarm/core.py:32 |  |  | 0.597 |
| ns | 1914 |  | 166 | `client.run()` semantics and the five-step loop | 3.1 |  | 0.581 |
| walker |  | 1958 | 92 | python method at swarm/core.py:139 |  |  | 0.621 |
| walker |  | 1992 | 34 | listing of 'examples/customer_service_streaming/src' |  |  | 0.621 |
| walker |  | 2011 | 19 | listing of 'examples/customer_service_streaming/src/swarm' |  |  | 0.622 |
| walker |  | 2025 | 14 | listing of 'examples/customer_service_streaming/src/swarm/engines' |  |  | 0.622 |
| ns | 2068 |  | 154 | `run()` arguments table, part 1 of 2 | 3.2 |  | 0.613 |
| walker |  | 2131 | 106 | python method at swarm/core.py:231 |  |  | 0.655 |
| walker |  | 2134 | 3 | listing of 'examples/customer_service' |  |  | 0.655 |
| walker |  | 2137 | 3 | listing of 'examples/customer_service_lite' |  |  | 0.656 |
| walker |  | 2179 | 42 | listing of 'examples/support_bot' |  |  | 0.658 |
| ns | 2242 |  | 174 | `run()` arguments table, part 2 of 2 (completes the table) | 3.3 | 3.2 | 0.648 |
| ns | 2398 |  | 156 | `Response` fields table (complete) | 3.4 |  | 0.639 |
| walker |  | 2447 | 268 | plaintext config setup.cfg |  |  | 0.641 |
| walker |  | 2510 | 63 | python decl names surface in swarm/types.py |  |  | 0.647 |
| walker |  | 2510 | 0 | python decl at swarm/types.py:14 |  |  | 0.647 |
| walker |  | 2510 | 0 | python decl at swarm/types.py:23 |  |  | 0.647 |
| walker |  | 2510 | 0 | python decl at swarm/types.py:29 |  |  | 0.647 |
| walker |  | 2543 | 33 | python class body at swarm/types.py:23 |  |  | 0.657 |
| walker |  | 2578 | 35 | python class body at swarm/types.py:29 |  |  | 0.675 |
| ns | 2603 |  | 205 | `Agent` fields table (the README's documented subset) | 3.5 |  | 0.662 |
| walker |  | 2666 | 88 | python class body at swarm/types.py:14 |  |  | 0.696 |
| walker |  | 2753 | 87 | python decl doc at swarm/types.py:29 |  |  | 0.700 |
| ns | 2766 |  | 163 | Function/tool rules: return values, context, errors, ordering | 3.6 |  | 0.688 |
| walker |  | 2821 | 68 | python decl names surface in swarm/util.py |  |  | 0.702 |
| walker |  | 2821 | 0 | python decl at swarm/util.py:5 |  |  | 0.702 |
| walker |  | 2821 | 0 | python decl at swarm/util.py:13 |  |  | 0.702 |
| walker |  | 2821 | 0 | python decl at swarm/util.py:21 |  |  | 0.702 |
| walker |  | 2821 | 0 | python decl at swarm/util.py:31 |  |  | 0.702 |
| walker |  | 2845 | 24 | python imports in swarm/repl/repl.py |  |  | 0.702 |
| ns | 2897 |  | 131 | Handoffs and `Result`: the documented rules | 3.7 |  | 0.695 |
| walker |  | 2953 | 108 | python decl doc at swarm/util.py:31 |  |  | 0.695 |
| walker |  | 3014 | 61 | python decl body at swarm/util.py:13 body 14 |  |  | 0.695 |
| ns | 3018 |  | 121 | Function-schema conversion rules | 3.8 |  | 0.684 |
| ns | 3172 |  | 154 | Streaming: the two Swarm-specific event types | 3.9 |  | 0.675 |
| walker |  | 3349 | 335 | README.md section #1 |  |  | 0.708 |
| ns | 3369 |  | 197 | Examples index: what each example directory demonstrates | 3.10 |  | 0.694 |
| ns | 3495 |  | 126 | Returning from `run()` and resuming a conversation | 3.11 |  | 0.692 |
| walker |  | 3515 | 166 | python imports in swarm/core.py |  |  | 0.743 |
| walker |  | 3554 | 39 | python decl names surface in swarm/repl/repl.py |  |  | 0.749 |
| walker |  | 3554 | 0 | python decl at swarm/repl/repl.py:6 |  |  | 0.749 |
| walker |  | 3554 | 0 | python decl at swarm/repl/repl.py:37 |  |  | 0.749 |
| walker |  | 3580 | 26 | python decl at swarm/repl/repl.py:60 |  |  | 0.759 |
| walker |  | 3666 | 86 | YAML config at .pre-commit-config.yaml |  |  | 0.760 |
| ns | 3670 |  | 175 | `Agent` definition and `instructions` semantics | 3.12 |  | 0.754 |
| walker |  | 3746 | 80 | python imports in swarm/types.py |  |  | 0.773 |
| walker |  | 3836 | 90 | python decl body at swarm/util.py:5 body 6 |  |  | 0.773 |
| ns | 3847 |  | 177 | Install, client construction, and the `run_demo_loop` util | 3.13 | 1.1 | 0.754 |
| ns | 3992 |  | 145 | "Why Swarm" positioning and the evaluations stance | 3.14 |  | 0.752 |
| walker |  | 4038 | 202 | README.md section #2 |  |  | 0.772 |
| walker |  | 4049 | 11 | listing of 'tests/test_runs' |  |  | 0.773 |
| ns | 4066 |  | 74 | Listings for the three simple examples (complete) | 4.1 |  | 0.779 |
| walker |  | 4076 | 27 | python method body at swarm/core.py:27 body 28 |  |  | 0.779 |
| walker |  | 4170 | 94 | python decl body at swarm/util.py:21 body 22 |  |  | 0.780 |
| walker |  | 4245 | 75 | README.md section #11 |  |  | 0.784 |
| walker |  | 4249 | 4 | listing of 'examples/customer_service_streaming/configs/assistants' |  |  | 0.784 |
| walker |  | 4253 | 4 | listing of 'examples/customer_service_streaming/src/runs' |  |  | 0.784 |
| walker |  | 4257 | 4 | listing of 'examples/customer_service_streaming/src/tasks' |  |  | 0.784 |
| walker |  | 4264 | 7 | listing of 'examples/customer_service_streaming/logs' |  |  | 0.784 |
| ns | 4322 |  | 256 | Purpose line of all six example READMEs | 4.2 |  | 0.773 |
| ns | 4445 |  | 123 | `basic/bare_minimum.py` in full | 4.3 |  | 0.752 |
| walker |  | 4447 | 183 | README.md section #3 |  |  | 0.757 |
| ns | 4733 |  | 288 | `triage_agent/agents.py`: three agents and their wiring | 4.4 |  | 0.727 |
| ns | 4845 |  | 112 | `airline` example: complete directory tree | 4.5 |  | 0.711 |
| walker |  | 4882 | 435 | README.md section #4 |  |  | 0.753 |
| ns | 4907 |  | 62 | `support_bot` and `personal_shopper` listings (complete) | 4.6 |  | 0.758 |
| ns | 4947 |  | 40 | The three `customer_service*` directories: what is actually there | 4.7 |  | 0.762 |
| walker |  | 5034 | 152 | README.md section #13 |  |  | 0.762 |
| walker |  | 5039 | 5 | listing of 'examples/customer_service_streaming/src/evals' |  |  | 0.762 |
| ns | 5149 |  | 202 | `weather_agent/agents.py`: a complete function-calling agent | 4.8 |  | 0.741 |
| ns | 5335 |  | 186 | `basic/context_variables.py`: callable instructions + injected context | 4.9 |  | 0.722 |
| walker |  | 5397 | 358 | README.md section #5 |  |  | 0.740 |
| ns | 5576 |  | 241 | `airline` agents: the five agents and their instruction sources | 4.10 |  | 0.720 |
| ns | 5765 |  | 189 | `airline` agents: imports and the five transfer functions | 4.11 | 4.10 | 0.703 |
| walker |  | 5836 | 439 | README.md section #6 |  |  | 0.717 |
| walker |  | 5840 | 4 | listing of 'examples/airline/data/routines/baggage' |  |  | 0.719 |
| walker |  | 5844 | 4 | listing of 'examples/airline/data/routines/flight_modification' |  |  | 0.721 |
| walker |  | 5848 | 4 | listing of 'examples/customer_service_streaming/configs/assistants/user_interface' |  |  | 0.721 |
| walker |  | 5856 | 8 | python decl body at swarm/repl/repl.py:6 body 7 |  |  | 0.721 |
| walker |  | 5951 | 95 | json config logs/session_20240425-135655.json |  |  | 0.721 |
| ns | 5958 |  | 193 | `airline` agents: per-agent tool lists (the handoff graph) | 4.12 | 4.10 | 0.701 |
| walker |  | 5962 | 11 | listing of 'examples/customer_service_streaming/tests' |  |  | 0.701 |
| ns | 6043 |  | 85 | `airline/configs/tools.py`: complete tool roster | 4.13 |  | 0.695 |
| walker |  | 6059 | 97 | json config logs/session_20240402-112456.json |  |  | 0.695 |
| walker |  | 6156 | 97 | json config logs/session_20240425-135657.json |  |  | 0.695 |
| walker |  | 6255 | 99 | json config logs/session_20240425-140516.json |  |  | 0.695 |
| ns | 6286 |  | 243 | `run()` body: state setup and the completion half of the loop | 5.1 | 2.6 | 0.677 |
| walker |  | 6354 | 99 | json config logs/session_20240425-145907.json |  |  | 0.677 |
| walker |  | 6453 | 99 | json config logs/session_20240425-211732.json |  |  | 0.677 |
| ns | 6479 |  | 193 | `run()` body: tool dispatch, agent switch, and the returned `Response` | 5.2 | 5.1 | 0.662 |
| walker |  | 6554 | 101 | json config logs/session_20240402-112443.json |  |  | 0.662 |
| ns | 6572 |  | 93 | `run()` body: the `stream=True` delegation branch | 5.3 | 2.6 | 0.655 |
| walker |  | 6655 | 101 | json config logs/session_20240402-112501.json |  |  | 0.655 |
| walker |  | 6756 | 101 | json config logs/session_20240425-140502.json |  |  | 0.655 |
| ns | 6783 |  | 211 | `handle_tool_calls` body: dispatch table and the missing-tool path | 5.4 | 2.8 | 0.642 |
| walker |  | 6857 | 101 | json config logs/session_20240425-141509.json |  |  | 0.642 |
| walker |  | 6958 | 101 | json config logs/session_20240425-211942.json |  |  | 0.642 |
| walker |  | 6965 | 7 | listing of 'examples/customer_service_streaming/tests/test_runs' |  |  | 0.642 |
| ns | 7054 |  | 271 | `handle_tool_calls` body: context injection and `Result` merging | 5.5 | 5.4 | 0.626 |
| walker |  | 7067 | 102 | json config logs/session_20240402-113222.json |  |  | 0.626 |
| walker |  | 7149 | 82 | README.md section #12 |  |  | 0.639 |
| walker |  | 7252 | 103 | json config logs/session_20240425-135728.json |  |  | 0.639 |
| ns | 7265 |  | 211 | `get_chat_completion` body: instructions, tool schemas, context hiding | 5.6 | 2.8 | 0.628 |
| walker |  | 7355 | 103 | json config logs/session_20240425-141709.json |  |  | 0.628 |
| ns | 7390 |  | 125 | `get_chat_completion` body: the Chat Completions request | 5.7 | 5.6 | 0.620 |
| walker |  | 7458 | 103 | json config logs/session_20240425-145129.json |  |  | 0.620 |
| walker |  | 7561 | 103 | json config logs/session_20240425-145930.json |  |  | 0.620 |
| ns | 7565 |  | 175 | `handle_function_result` body: the return-value coercion rules | 5.8 | 2.8 | 0.611 |
| walker |  | 7664 | 103 | json config logs/session_20240425-212431.json |  |  | 0.611 |
| walker |  | 7769 | 105 | json config logs/session_20240425-212748.json |  |  | 0.611 |
| walker |  | 7874 | 105 | json config logs/session_20240425-213023.json |  |  | 0.611 |
| walker |  | 7883 | 9 | python decl body at swarm/repl/repl.py:6 body 8 |  |  | 0.611 |
| ns | 7894 |  | 329 | `function_to_json` body: type map, signature inspection, `required` | 5.9 | 2.4 | 0.593 |
| ns | 8003 |  | 109 | `function_to_json` body: the emitted schema shape | 5.10 | 5.9 | 0.586 |
| ns | 8173 |  | 170 | `swarm/util.py`: the streaming merge helpers | 5.11 | 2.4 | 0.592 |
| ns | 8412 |  | 239 | `run_demo_loop` body: the reference conversation loop | 5.12 | 2.5 | 0.579 |
| walker |  | 8439 | 556 | README.md section #7 |  |  | 0.584 |
| ns | 8646 |  | 234 | `run_and_stream` body: the streaming protocol, with elisions marked | 5.13 | 2.7 | 0.573 |
| ns | 8875 |  | 229 | `tests/test_core.py`: complete test roster and the shared fixture | 6.1 |  | 0.564 |
| walker |  | 8987 | 548 | README.md section #8 |  |  | 0.571 |
| ns | 9032 |  | 157 | `tests/mock_client.py`: the fake OpenAI client | 6.2 |  | 0.566 |
| ns | 9134 |  | 102 | `tests/test_util.py`: both schema-conversion tests | 6.3 |  | 0.563 |
| ns | 9340 |  | 206 | `setup.cfg`: package metadata and the complete dependency list | 6.4 |  | 0.574 |
| walker |  | 9510 | 523 | README.md section #9 |  |  | 0.584 |
| ns | 9515 |  | 175 | Build backend and formatting toolchain | 6.5 |  | 0.592 |
| ns | 9649 |  | 134 | `customer_service_streaming/src` and `configs`: complete listings | 7.1 |  | 0.605 |
| walker |  | 9722 | 212 | README.md section #10 |  |  | 0.610 |
| walker |  | 9829 | 107 | json config logs/session_20240425-145324.json |  |  | 0.610 |
| ns | 9869 |  | 220 | The legacy example's own `Swarm` class and its config knobs | 7.2 | 7.1 | 0.601 |
| ns | 9894 |  | 25 | Remaining asset and log directories | 7.3 |  | 0.603 |
| walker |  | 9936 | 107 | json config logs/session_20240425-212341.json |  |  | 0.603 |
