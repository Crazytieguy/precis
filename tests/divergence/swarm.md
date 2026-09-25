Score(3000)=0.721 I=0.834 C=0.623 ns_rows≤3K=25/66 grid(1000/1442/2080/3000/4327/6240/9000)=0.446/0.618/0.602/0.721/0.725/0.659/0.544

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
| ns | 1748 |  | 174 | `swarm/core.py` import block | 2.10 |  | 0.628 |
| ns | 1914 |  | 166 | `client.run()` semantics and the five-step loop | 3.1 |  | 0.612 |
| ns | 2068 |  | 154 | `run()` arguments table, part 1 of 2 | 3.2 |  | 0.602 |
| walker |  | 2214 | 597 | README.md section #0 |  |  | 0.759 |
| walker |  | 2237 | 23 | listing of 'examples/triage_agent' |  |  | 0.760 |
| ns | 2242 |  | 174 | `run()` arguments table, part 2 of 2 (completes the table) | 3.3 | 3.2 | 0.749 |
| walker |  | 2257 | 20 | listing of 'examples/personal_shopper' |  |  | 0.749 |
| walker |  | 2283 | 26 | listing of 'examples/airline' |  |  | 0.749 |
| walker |  | 2286 | 3 | listing of 'examples/airline/data' |  |  | 0.750 |
| walker |  | 2298 | 12 | listing of 'examples/airline/data/routines' |  |  | 0.750 |
| walker |  | 2314 | 16 | listing of 'examples/airline/configs' |  |  | 0.751 |
| walker |  | 2348 | 34 | listing of 'examples/basic' |  |  | 0.754 |
| walker |  | 2382 | 34 | listing of 'examples/customer_service_streaming' |  |  | 0.755 |
| ns | 2398 |  | 156 | `Response` fields table (complete) | 3.4 |  | 0.744 |
| walker |  | 2409 | 27 | listing of 'examples/customer_service_streaming/configs' |  |  | 0.744 |
| walker |  | 2421 | 12 | listing of 'examples/customer_service_streaming/configs/tools' |  |  | 0.744 |
| walker |  | 2455 | 34 | listing of 'examples/customer_service_streaming/src' |  |  | 0.745 |
| walker |  | 2474 | 19 | listing of 'examples/customer_service_streaming/src/swarm' |  |  | 0.746 |
| walker |  | 2488 | 14 | listing of 'examples/customer_service_streaming/src/swarm/engines' |  |  | 0.746 |
| walker |  | 2491 | 3 | listing of 'examples/customer_service' |  |  | 0.746 |
| walker |  | 2494 | 3 | listing of 'examples/customer_service_lite' |  |  | 0.747 |
| walker |  | 2536 | 42 | listing of 'examples/support_bot' |  |  | 0.750 |
| walker |  | 2597 | 61 | python body swarm/util.py:13 |  |  | 0.750 |
| ns | 2603 |  | 205 | `Agent` fields table (the README's documented subset) | 3.5 |  | 0.735 |
| ns | 2766 |  | 163 | Function/tool rules: return values, context, errors, ordering | 3.6 |  | 0.723 |
| walker |  | 2865 | 268 | plaintext config setup.cfg |  |  | 0.725 |
| ns | 2897 |  | 131 | Handoffs and `Result`: the documented rules | 3.7 |  | 0.717 |
| walker |  | 2952 | 87 | python doc swarm/types.py:29 |  |  | 0.721 |
| ns | 3018 |  | 121 | Function-schema conversion rules | 3.8 |  | 0.709 |
| walker |  | 3042 | 90 | python body swarm/util.py:5 |  |  | 0.709 |
| walker |  | 3136 | 94 | python body swarm/util.py:21 |  |  | 0.710 |
| ns | 3172 |  | 154 | Streaming: the two Swarm-specific event types | 3.9 |  | 0.701 |
| walker |  | 3244 | 108 | python doc swarm/util.py:31 |  |  | 0.701 |
| walker |  | 3271 | 27 | python body swarm/core.py:27 |  |  | 0.701 |
| ns | 3369 |  | 197 | Examples index: what each example directory demonstrates | 3.10 |  | 0.687 |
| ns | 3495 |  | 126 | Returning from `run()` and resuming a conversation | 3.11 |  | 0.685 |
| walker |  | 3606 | 335 | README.md section #1 |  |  | 0.718 |
| ns | 3670 |  | 175 | `Agent` definition and `instructions` semantics | 3.12 |  | 0.713 |
| walker |  | 3692 | 86 | YAML config at .pre-commit-config.yaml |  |  | 0.714 |
| ns | 3847 |  | 177 | Install, client construction, and the `run_demo_loop` util | 3.13 | 1.1 | 0.697 |
| walker |  | 3894 | 202 | README.md section #2 |  |  | 0.719 |
| walker |  | 3905 | 11 | listing of 'tests/test_runs' |  |  | 0.719 |
| walker |  | 3980 | 75 | README.md section #11 |  |  | 0.719 |
| walker |  | 3984 | 4 | listing of 'examples/customer_service_streaming/configs/assistants' |  |  | 0.719 |
| walker |  | 3988 | 4 | listing of 'examples/customer_service_streaming/src/runs' |  |  | 0.719 |
| walker |  | 3992 | 4 | listing of 'examples/customer_service_streaming/src/tasks' |  |  | 0.721 |
| ns | 3992 |  | 145 | "Why Swarm" positioning and the evaluations stance | 3.14 |  | 0.721 |
| walker |  | 3999 | 7 | listing of 'examples/customer_service_streaming/logs' |  |  | 0.721 |
| ns | 4066 |  | 74 | Listings for the three simple examples (complete) | 4.1 |  | 0.731 |
| walker |  | 4182 | 183 | README.md section #3 |  |  | 0.736 |
| ns | 4322 |  | 256 | Purpose line of all six example READMEs | 4.2 |  | 0.725 |
| ns | 4445 |  | 123 | `basic/bare_minimum.py` in full | 4.3 |  | 0.706 |
| walker |  | 4617 | 435 | README.md section #4 |  |  | 0.754 |
| ns | 4733 |  | 288 | `triage_agent/agents.py`: three agents and their wiring | 4.4 |  | 0.724 |
| walker |  | 4769 | 152 | README.md section #13 |  |  | 0.724 |
| walker |  | 4774 | 5 | listing of 'examples/customer_service_streaming/src/evals' |  |  | 0.724 |
| ns | 4845 |  | 112 | `airline` example: complete directory tree | 4.5 |  | 0.708 |
| ns | 4907 |  | 62 | `support_bot` and `personal_shopper` listings (complete) | 4.6 |  | 0.715 |
| ns | 4947 |  | 40 | The three `customer_service*` directories: what is actually there | 4.7 |  | 0.720 |
| walker |  | 5132 | 358 | README.md section #5 |  |  | 0.739 |
| ns | 5149 |  | 202 | `weather_agent/agents.py`: a complete function-calling agent | 4.8 |  | 0.718 |
| ns | 5335 |  | 186 | `basic/context_variables.py`: callable instructions + injected context | 4.9 |  | 0.700 |
| walker |  | 5571 | 439 | README.md section #6 |  |  | 0.715 |
| ns | 5576 |  | 241 | `airline` agents: the five agents and their instruction sources | 4.10 |  | 0.696 |
| walker |  | 5582 | 11 | python names examples/customer_service_streaming/main.py |  |  | 0.696 |
| walker |  | 5586 | 4 | listing of 'examples/airline/data/routines/baggage' |  |  | 0.698 |
| walker |  | 5590 | 4 | listing of 'examples/airline/data/routines/flight_modification' |  |  | 0.700 |
| walker |  | 5594 | 4 | listing of 'examples/customer_service_streaming/configs/assistants/user_interface' |  |  | 0.700 |
| walker |  | 5606 | 12 | python names examples/airline/main.py |  |  | 0.700 |
| walker |  | 5701 | 95 | json config logs/session_20240425-135655.json |  |  | 0.700 |
| walker |  | 5712 | 11 | listing of 'examples/customer_service_streaming/tests' |  |  | 0.700 |
| ns | 5765 |  | 189 | `airline` agents: imports and the five transfer functions | 4.11 | 4.10 | 0.684 |
| walker |  | 5809 | 97 | json config logs/session_20240402-112456.json |  |  | 0.684 |
| walker |  | 5906 | 97 | json config logs/session_20240425-135657.json |  |  | 0.684 |
| ns | 5958 |  | 193 | `airline` agents: per-agent tool lists (the handoff graph) | 4.12 | 4.10 | 0.665 |
| walker |  | 6005 | 99 | json config logs/session_20240425-140516.json |  |  | 0.665 |
| ns | 6043 |  | 85 | `airline/configs/tools.py`: complete tool roster | 4.13 |  | 0.659 |
| walker |  | 6104 | 99 | json config logs/session_20240425-145907.json |  |  | 0.659 |
| walker |  | 6203 | 99 | json config logs/session_20240425-211732.json |  |  | 0.659 |
| ns | 6286 |  | 243 | `run()` body: state setup and the completion half of the loop | 5.1 | 2.6 | 0.642 |
| walker |  | 6304 | 101 | json config logs/session_20240402-112443.json |  |  | 0.642 |
| walker |  | 6405 | 101 | json config logs/session_20240402-112501.json |  |  | 0.642 |
| ns | 6479 |  | 193 | `run()` body: tool dispatch, agent switch, and the returned `Response` | 5.2 | 5.1 | 0.628 |
| walker |  | 6506 | 101 | json config logs/session_20240425-140502.json |  |  | 0.628 |
| ns | 6572 |  | 93 | `run()` body: the `stream=True` delegation branch | 5.3 | 2.6 | 0.621 |
| walker |  | 6607 | 101 | json config logs/session_20240425-141509.json |  |  | 0.621 |
| walker |  | 6708 | 101 | json config logs/session_20240425-211942.json |  |  | 0.621 |
| walker |  | 6715 | 7 | listing of 'examples/customer_service_streaming/tests/test_runs' |  |  | 0.621 |
| ns | 6783 |  | 211 | `handle_tool_calls` body: dispatch table and the missing-tool path | 5.4 | 2.8 | 0.609 |
| walker |  | 6817 | 102 | json config logs/session_20240402-113222.json |  |  | 0.609 |
| walker |  | 6899 | 82 | README.md section #12 |  |  | 0.623 |
| walker |  | 7002 | 103 | json config logs/session_20240425-135728.json |  |  | 0.623 |
| ns | 7054 |  | 271 | `handle_tool_calls` body: context injection and `Result` merging | 5.5 | 5.4 | 0.607 |
| walker |  | 7105 | 103 | json config logs/session_20240425-141709.json |  |  | 0.607 |
| walker |  | 7208 | 103 | json config logs/session_20240425-145129.json |  |  | 0.607 |
| ns | 7265 |  | 211 | `get_chat_completion` body: instructions, tool schemas, context hiding | 5.6 | 2.8 | 0.596 |
| walker |  | 7311 | 103 | json config logs/session_20240425-145930.json |  |  | 0.596 |
| ns | 7390 |  | 125 | `get_chat_completion` body: the Chat Completions request | 5.7 | 5.6 | 0.589 |
| walker |  | 7414 | 103 | json config logs/session_20240425-212431.json |  |  | 0.589 |
| walker |  | 7519 | 105 | json config logs/session_20240425-212748.json |  |  | 0.589 |
| ns | 7565 |  | 175 | `handle_function_result` body: the return-value coercion rules | 5.8 | 2.8 | 0.580 |
| walker |  | 7624 | 105 | json config logs/session_20240425-213023.json |  |  | 0.580 |
| ns | 7894 |  | 329 | `function_to_json` body: type map, signature inspection, `required` | 5.9 | 2.4 | 0.563 |
| ns | 8003 |  | 109 | `function_to_json` body: the emitted schema shape | 5.10 | 5.9 | 0.557 |
| ns | 8173 |  | 170 | `swarm/util.py`: the streaming merge helpers | 5.11 | 2.4 | 0.563 |
| walker |  | 8180 | 556 | README.md section #7 |  |  | 0.568 |
| ns | 8412 |  | 239 | `run_demo_loop` body: the reference conversation loop | 5.12 | 2.5 | 0.556 |
| ns | 8646 |  | 234 | `run_and_stream` body: the streaming protocol, with elisions marked | 5.13 | 2.7 | 0.545 |
| walker |  | 8728 | 548 | README.md section #8 |  |  | 0.552 |
| ns | 8875 |  | 229 | `tests/test_core.py`: complete test roster and the shared fixture | 6.1 |  | 0.544 |
| ns | 9032 |  | 157 | `tests/mock_client.py`: the fake OpenAI client | 6.2 |  | 0.540 |
| ns | 9134 |  | 102 | `tests/test_util.py`: both schema-conversion tests | 6.3 |  | 0.537 |
| walker |  | 9251 | 523 | README.md section #9 |  |  | 0.547 |
| ns | 9340 |  | 206 | `setup.cfg`: package metadata and the complete dependency list | 6.4 |  | 0.559 |
| walker |  | 9463 | 212 | README.md section #10 |  |  | 0.564 |
| ns | 9515 |  | 175 | Build backend and formatting toolchain | 6.5 |  | 0.573 |
| walker |  | 9570 | 107 | json config logs/session_20240425-145324.json |  |  | 0.573 |
| ns | 9649 |  | 134 | `customer_service_streaming/src` and `configs`: complete listings | 7.1 |  | 0.586 |
| walker |  | 9677 | 107 | json config logs/session_20240425-212341.json |  |  | 0.586 |
| walker |  | 9788 | 111 | json config logs/session_20240425-140427.json |  |  | 0.586 |
| ns | 9869 |  | 220 | The legacy example's own `Swarm` class and its config knobs | 7.2 | 7.1 | 0.578 |
| ns | 9894 |  | 25 | Remaining asset and log directories | 7.3 |  | 0.580 |
| walker |  | 9900 | 112 | json config logs/session_20240425-155814.json |  |  | 0.580 |
