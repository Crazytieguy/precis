Score(3000)=0.697 I=0.826 C=0.588 ns_rows≤3K=25/66 (reached=11 partial=2 missing=12)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 42 | 42 | listing of '.' |  |  | 0.000 |
| walker |  | 55 | 13 | listing of 'assets' |  |  | 0.000 |
| walker |  | 71 | 16 | README headline in README.md |  |  | 0.000 |
| ns | 87 |  | 87 | README title + deprecation callout | 1.1 |  | 0.116 |
| walker |  | 92 | 21 | listing of 'swarm' |  |  | 0.123 |
| ns | 129 |  | 42 | Repository root listing (complete) | 1.2 |  | 0.545 |
| walker |  | 138 | 46 | python imports in swarm/__init__.py |  |  | 0.565 |
| walker |  | 147 | 9 | listing of 'swarm/repl' |  |  | 0.589 |
| walker |  | 160 | 13 | python imports in swarm/repl/__init__.py |  |  | 0.601 |
| walker |  | 192 | 32 | manifest config in pyproject.toml |  |  | 0.601 |
| ns | 216 |  | 87 | README Overview: agents and handoffs | 1.3 |  | 0.542 |
| walker |  | 217 | 25 | listing of 'tests' |  |  | 0.547 |
| ns | 247 |  | 31 | `swarm/` package listing (complete, incl. `repl/`) | 1.4 |  | 0.568 |
| ns | 306 |  | 59 | Public exports of `swarm` and `swarm.repl` | 1.5 | 1.4 | 0.574 |
| ns | 404 |  | 98 | README: expressive power, and the statelessness NOTE | 1.6 | 1.3 | 0.543 |
| ns | 483 |  | 79 | `examples/` and `tests/` directory listings (complete) | 1.7 |  | 0.451 |
| walker |  | 557 | 340 | listing of 'logs' |  |  | 0.451 |
| walker |  | 611 | 54 | listing of 'examples' |  |  | 0.603 |
| walker |  | 627 | 16 | listing of 'examples/weather_agent' |  |  | 0.604 |
| ns | 693 |  | 210 | README section-heading roster (all remaining headings) | 1.8 | 1.1 | 0.488 |
| walker |  | 820 | 193 | headings outline in README.md |  |  | 0.622 |
| ns | 824 |  | 131 | `Agent` model: every field with its default | 2.1 |  | 0.575 |
| walker |  | 837 | 17 | listing of 'examples/personal_shopper' |  |  | 0.575 |
| walker |  | 859 | 22 | listing of 'examples/triage_agent' |  |  | 0.577 |
| walker |  | 885 | 26 | listing of 'examples/airline' |  |  | 0.578 |
| walker |  | 888 | 3 | listing of 'examples/airline/data' |  |  | 0.578 |
| walker |  | 901 | 13 | listing of 'examples/airline/data/routines' |  |  | 0.579 |
| walker |  | 914 | 13 | listing of 'examples/airline/configs' |  |  | 0.580 |
| ns | 919 |  | 95 | `Swarm` class: complete method roster | 2.2 |  | 0.548 |
| walker |  | 932 | 18 | python imports in swarm/util.py |  |  | 0.548 |
| walker |  | 965 | 33 | listing of 'examples/basic' |  |  | 0.552 |
| walker |  | 1003 | 38 | listing of 'examples/customer_service_streaming' |  |  | 0.555 |
| ns | 1007 |  | 88 | `Response` and `Result` models: every field | 2.3 | 2.1 | 0.524 |
| walker |  | 1029 | 26 | listing of 'examples/customer_service_streaming/configs' |  |  | 0.524 |
| walker |  | 1043 | 14 | listing of 'examples/customer_service_streaming/configs/tools' |  |  | 0.524 |
| walker |  | 1055 | 12 | python decl names surface in swarm/core.py |  |  | 0.524 |
| walker |  | 1055 | 0 | python decl at swarm/core.py:26 |  |  | 0.524 |
| ns | 1079 |  | 72 | `swarm/util.py`: complete function roster | 2.4 |  | 0.512 |
| walker |  | 1135 | 80 | python method sigs in swarm/core.py |  |  | 0.544 |
| walker |  | 1135 | 0 | python method at swarm/core.py:27 |  |  | 0.544 |
| walker |  | 1135 | 0 | python method at swarm/core.py:71 |  |  | 0.544 |
| ns | 1148 |  | 69 | `swarm/repl/repl.py`: complete function roster | 2.5 |  | 0.529 |
| walker |  | 1198 | 63 | python method at swarm/core.py:89 |  |  | 0.532 |
| ns | 1254 |  | 106 | `Swarm.run()` full signature with defaults | 2.6 | 2.2 | 0.502 |
| walker |  | 1272 | 74 | python method at swarm/core.py:32 |  |  | 0.510 |
| ns | 1346 |  | 92 | `run_and_stream()` full signature | 2.7 | 2.2 | 0.487 |
| walker |  | 1364 | 92 | python method at swarm/core.py:139 |  |  | 0.532 |
| walker |  | 1399 | 35 | listing of 'examples/customer_service_streaming/src' |  |  | 0.533 |
| walker |  | 1418 | 19 | listing of 'examples/customer_service_streaming/src/swarm' |  |  | 0.534 |
| walker |  | 1431 | 13 | listing of 'examples/customer_service_streaming/src/swarm/engines' |  |  | 0.534 |
| ns | 1491 |  | 145 | Internal `Swarm` method signatures: completion + tool dispatch | 2.8 | 2.2 | 0.552 |
| walker |  | 1537 | 106 | python method at swarm/core.py:231 |  |  | 0.595 |
| walker |  | 1540 | 3 | listing of 'examples/customer_service' |  |  | 0.596 |
| walker |  | 1543 | 3 | listing of 'examples/customer_service_lite' |  |  | 0.597 |
| ns | 1583 |  | 92 | `swarm/types.py` imports: pydantic + reused OpenAI types | 2.9 |  | 0.576 |
| walker |  | 1618 | 75 | README.md section #11 |  |  | 0.576 |
| walker |  | 1658 | 40 | listing of 'examples/support_bot' |  |  | 0.579 |
| ns | 1757 |  | 174 | `swarm/core.py` import block | 2.10 |  | 0.537 |
| ns | 1923 |  | 166 | `client.run()` semantics and the five-step loop | 3.1 |  | 0.524 |
| ns | 2077 |  | 154 | `run()` arguments table, part 1 of 2 | 3.2 |  | 0.515 |
| ns | 2251 |  | 174 | `run()` arguments table, part 2 of 2 (completes the table) | 3.3 | 3.2 | 0.507 |
| walker |  | 2255 | 597 | README.md section #0 |  |  | 0.648 |
| ns | 2407 |  | 156 | `Response` fields table (complete) | 3.4 |  | 0.639 |
| walker |  | 2523 | 268 | plaintext config setup.cfg |  |  | 0.641 |
| walker |  | 2605 | 82 | README.md section #12 |  |  | 0.643 |
| ns | 2612 |  | 205 | `Agent` fields table (the README's documented subset) | 3.5 |  | 0.630 |
| walker |  | 2668 | 63 | python decl names surface in swarm/types.py |  |  | 0.636 |
| walker |  | 2668 | 0 | python decl at swarm/types.py:14 |  |  | 0.636 |
| walker |  | 2668 | 0 | python decl at swarm/types.py:23 |  |  | 0.636 |
| walker |  | 2668 | 0 | python decl at swarm/types.py:29 |  |  | 0.636 |
| walker |  | 2701 | 33 | python class body at swarm/types.py:23 |  |  | 0.646 |
| walker |  | 2736 | 35 | python class body at swarm/types.py:29 |  |  | 0.664 |
| ns | 2775 |  | 163 | Function/tool rules: return values, context, errors, ordering | 3.6 |  | 0.653 |
| walker |  | 2824 | 88 | python class body at swarm/types.py:14 |  |  | 0.687 |
| ns | 2906 |  | 131 | Handoffs and `Result`: the documented rules | 3.7 |  | 0.679 |
| walker |  | 2911 | 87 | python decl doc at swarm/types.py:29 |  |  | 0.683 |
| walker |  | 2979 | 68 | python decl names surface in swarm/util.py |  |  | 0.697 |
| walker |  | 2979 | 0 | python decl at swarm/util.py:5 |  |  | 0.697 |
| walker |  | 2979 | 0 | python decl at swarm/util.py:13 |  |  | 0.697 |
| walker |  | 2979 | 0 | python decl at swarm/util.py:21 |  |  | 0.697 |
| walker |  | 2979 | 0 | python decl at swarm/util.py:31 |  |  | 0.697 |
| walker |  | 3003 | 24 | python imports in swarm/repl/repl.py |  |  | 0.697 |
| ns | 3027 |  | 121 | Function-schema conversion rules | 3.8 |  | 0.686 |
| walker |  | 3111 | 108 | python decl doc at swarm/util.py:31 |  |  | 0.686 |
| walker |  | 3172 | 61 | python decl body at swarm/util.py:13 body 14 |  |  | 0.686 |
| walker |  | 3176 | 4 | listing of 'examples/customer_service_streaming/logs' |  |  | 0.686 |
| ns | 3181 |  | 154 | Streaming: the two Swarm-specific event types | 3.9 |  | 0.677 |
| walker |  | 3342 | 166 | python imports in swarm/core.py |  |  | 0.729 |
| ns | 3378 |  | 197 | Examples index: what each example directory demonstrates | 3.10 |  | 0.714 |
| ns | 3504 |  | 126 | Returning from `run()` and resuming a conversation | 3.11 |  | 0.713 |
| walker |  | 3525 | 183 | README.md section #3 |  |  | 0.715 |
| ns | 3679 |  | 175 | `Agent` definition and `instructions` semantics | 3.12 |  | 0.710 |
| ns | 3856 |  | 177 | Install, client construction, and the `run_demo_loop` util | 3.13 | 1.1 | 0.719 |
| walker |  | 3960 | 435 | README.md section #4 |  |  | 0.771 |
| ns | 4001 |  | 145 | "Why Swarm" positioning and the evaluations stance | 3.14 |  | 0.768 |
| ns | 4072 |  | 71 | Listings for the three simple examples (complete) | 4.1 |  | 0.774 |
| walker |  | 4112 | 152 | README.md section #13 |  |  | 0.774 |
| walker |  | 4314 | 202 | README.md section #2 |  |  | 0.793 |
| ns | 4328 |  | 256 | Purpose line of all six example READMEs | 4.2 |  | 0.781 |
| walker |  | 4353 | 39 | python decl names surface in swarm/repl/repl.py |  |  | 0.786 |
| walker |  | 4353 | 0 | python decl at swarm/repl/repl.py:6 |  |  | 0.786 |
| walker |  | 4353 | 0 | python decl at swarm/repl/repl.py:37 |  |  | 0.786 |
| walker |  | 4379 | 26 | python decl at swarm/repl/repl.py:60 |  |  | 0.794 |
| ns | 4451 |  | 123 | `basic/bare_minimum.py` in full | 4.3 |  | 0.773 |
| walker |  | 4465 | 86 | YAML config at .pre-commit-config.yaml |  |  | 0.774 |
| walker |  | 4545 | 80 | python imports in swarm/types.py |  |  | 0.790 |
| walker |  | 4548 | 3 | listing of 'examples/customer_service_streaming/src/runs' |  |  | 0.790 |
| walker |  | 4551 | 3 | listing of 'examples/customer_service_streaming/src/tasks' |  |  | 0.790 |
| walker |  | 4561 | 10 | listing of 'tests/test_runs' |  |  | 0.790 |
| walker |  | 4651 | 90 | python decl body at swarm/util.py:5 body 6 |  |  | 0.790 |
| walker |  | 4678 | 27 | python method body at swarm/core.py:27 body 28 |  |  | 0.790 |
| ns | 4739 |  | 288 | `triage_agent/agents.py`: three agents and their wiring | 4.4 |  | 0.758 |
| walker |  | 4772 | 94 | python decl body at swarm/util.py:21 body 22 |  |  | 0.760 |
| walker |  | 4776 | 4 | listing of 'examples/customer_service_streaming/configs/assistants' |  |  | 0.760 |
| walker |  | 4780 | 4 | listing of 'examples/customer_service_streaming/src/evals' |  |  | 0.760 |
| ns | 4854 |  | 115 | `airline` example: complete directory tree | 4.5 |  | 0.742 |
| ns | 4911 |  | 57 | `support_bot` and `personal_shopper` listings (complete) | 4.6 |  | 0.747 |
| ns | 4955 |  | 44 | The three `customer_service*` directories: what is actually there | 4.7 |  | 0.750 |
| walker |  | 5115 | 335 | README.md section #1 |  |  | 0.780 |
| walker |  | 5118 | 3 | listing of 'examples/airline/data/routines/baggage' |  |  | 0.782 |
| walker |  | 5121 | 3 | listing of 'examples/airline/data/routines/flight_modification' |  |  | 0.784 |
| walker |  | 5124 | 3 | listing of 'examples/customer_service_streaming/configs/assistants/user_interface' |  |  | 0.784 |
| walker |  | 5132 | 8 | python decl body at swarm/repl/repl.py:6 body 7 |  |  | 0.784 |
| ns | 5157 |  | 202 | `weather_agent/agents.py`: a complete function-calling agent | 4.8 |  | 0.762 |
| walker |  | 5227 | 95 | json config logs/session_20240425-135655.json |  |  | 0.762 |
| walker |  | 5238 | 11 | listing of 'examples/customer_service_streaming/tests' |  |  | 0.762 |
| walker |  | 5242 | 4 | listing of 'examples/customer_service_streaming/tests/test_runs' |  |  | 0.762 |
| walker |  | 5339 | 97 | json config logs/session_20240402-112456.json |  |  | 0.762 |
| ns | 5343 |  | 186 | `basic/context_variables.py`: callable instructions + injected context | 4.9 |  | 0.743 |
| walker |  | 5436 | 97 | json config logs/session_20240425-135657.json |  |  | 0.743 |
| walker |  | 5535 | 99 | json config logs/session_20240425-140516.json |  |  | 0.743 |
| ns | 5584 |  | 241 | `airline` agents: the five agents and their instruction sources | 4.10 |  | 0.723 |
| walker |  | 5634 | 99 | json config logs/session_20240425-145907.json |  |  | 0.723 |
| walker |  | 5733 | 99 | json config logs/session_20240425-211732.json |  |  | 0.723 |
| ns | 5773 |  | 189 | `airline` agents: imports and the five transfer functions | 4.11 | 4.10 | 0.706 |
| walker |  | 5834 | 101 | json config logs/session_20240402-112443.json |  |  | 0.706 |
| walker |  | 5935 | 101 | json config logs/session_20240402-112501.json |  |  | 0.706 |
| ns | 5966 |  | 193 | `airline` agents: per-agent tool lists (the handoff graph) | 4.12 | 4.10 | 0.687 |
| walker |  | 6036 | 101 | json config logs/session_20240425-140502.json |  |  | 0.687 |
| ns | 6051 |  | 85 | `airline/configs/tools.py`: complete tool roster | 4.13 |  | 0.681 |
| walker |  | 6137 | 101 | json config logs/session_20240425-141509.json |  |  | 0.681 |
| walker |  | 6238 | 101 | json config logs/session_20240425-211942.json |  |  | 0.681 |
| ns | 6294 |  | 243 | `run()` body: state setup and the completion half of the loop | 5.1 | 2.6 | 0.663 |
| walker |  | 6340 | 102 | json config logs/session_20240402-113222.json |  |  | 0.663 |
| walker |  | 6443 | 103 | json config logs/session_20240425-135728.json |  |  | 0.663 |
| ns | 6487 |  | 193 | `run()` body: tool dispatch, agent switch, and the returned `Response` | 5.2 | 5.1 | 0.649 |
| walker |  | 6546 | 103 | json config logs/session_20240425-141709.json |  |  | 0.649 |
| ns | 6580 |  | 93 | `run()` body: the `stream=True` delegation branch | 5.3 | 2.6 | 0.642 |
| walker |  | 6649 | 103 | json config logs/session_20240425-145129.json |  |  | 0.642 |
| walker |  | 6752 | 103 | json config logs/session_20240425-145930.json |  |  | 0.642 |
| ns | 6791 |  | 211 | `handle_tool_calls` body: dispatch table and the missing-tool path | 5.4 | 2.8 | 0.629 |
| walker |  | 6855 | 103 | json config logs/session_20240425-212431.json |  |  | 0.629 |
| walker |  | 6960 | 105 | json config logs/session_20240425-212748.json |  |  | 0.629 |
| ns | 7062 |  | 271 | `handle_tool_calls` body: context injection and `Result` merging | 5.5 | 5.4 | 0.613 |
| walker |  | 7065 | 105 | json config logs/session_20240425-213023.json |  |  | 0.613 |
| walker |  | 7074 | 9 | python decl body at swarm/repl/repl.py:6 body 8 |  |  | 0.613 |
| walker |  | 7181 | 107 | json config logs/session_20240425-145324.json |  |  | 0.613 |
| ns | 7273 |  | 211 | `get_chat_completion` body: instructions, tool schemas, context hiding | 5.6 | 2.8 | 0.602 |
| walker |  | 7288 | 107 | json config logs/session_20240425-212341.json |  |  | 0.602 |
| ns | 7398 |  | 125 | `get_chat_completion` body: the Chat Completions request | 5.7 | 5.6 | 0.595 |
| walker |  | 7399 | 111 | json config logs/session_20240425-140427.json |  |  | 0.595 |
| walker |  | 7511 | 112 | json config logs/session_20240425-155814.json |  |  | 0.595 |
| ns | 7573 |  | 175 | `handle_function_result` body: the return-value coercion rules | 5.8 | 2.8 | 0.586 |
| walker |  | 7625 | 114 | json config logs/session_20240402-112114.json |  |  | 0.586 |
| walker |  | 7740 | 115 | json config logs/session_20240425-150004.json |  |  | 0.586 |
| walker |  | 7857 | 117 | json config logs/session_20240425-211813.json |  |  | 0.586 |
| ns | 7902 |  | 329 | `function_to_json` body: type map, signature inspection, `required` | 5.9 | 2.4 | 0.568 |
| walker |  | 7975 | 118 | json config logs/session_20240425-140553.json |  |  | 0.568 |
| ns | 8011 |  | 109 | `function_to_json` body: the emitted schema shape | 5.10 | 5.9 | 0.562 |
| walker |  | 8093 | 118 | json config logs/session_20240425-172809.json |  |  | 0.562 |
| ns | 8181 |  | 170 | `swarm/util.py`: the streaming merge helpers | 5.11 | 2.4 | 0.568 |
| walker |  | 8213 | 120 | json config logs/session_20240425-141416.json |  |  | 0.568 |
| walker |  | 8334 | 121 | json config logs/session_20240425-150040.json |  |  | 0.568 |
| ns | 8420 |  | 239 | `run_demo_loop` body: the reference conversation loop | 5.12 | 2.5 | 0.556 |
| ns | 8654 |  | 234 | `run_and_stream` body: the streaming protocol, with elisions marked | 5.13 | 2.7 | 0.545 |
| walker |  | 8692 | 358 | README.md section #5 |  |  | 0.558 |
| walker |  | 8699 | 7 | listing of 'examples/customer_service_streaming/configs/tools/query_docs' |  |  | 0.558 |
| walker |  | 8706 | 7 | listing of 'examples/customer_service_streaming/configs/tools/send_email' |  |  | 0.558 |
| walker |  | 8713 | 7 | listing of 'examples/customer_service_streaming/configs/tools/submit_ticket' |  |  | 0.558 |
| walker |  | 8733 | 20 | listing of 'examples/airline/evals' |  |  | 0.564 |
| walker |  | 8745 | 12 | listing of 'examples/airline/evals/eval_cases' |  |  | 0.568 |
| ns | 8883 |  | 229 | `tests/test_core.py`: complete test roster and the shared fixture | 6.1 |  | 0.560 |
| ns | 9040 |  | 157 | `tests/mock_client.py`: the fake OpenAI client | 6.2 |  | 0.556 |
| ns | 9142 |  | 102 | `tests/test_util.py`: both schema-conversion tests | 6.3 |  | 0.553 |
| walker |  | 9184 | 439 | README.md section #6 |  |  | 0.563 |
| ns | 9348 |  | 206 | `setup.cfg`: package metadata and the complete dependency list | 6.4 |  | 0.574 |
| walker |  | 9355 | 171 | json config logs/session_20240402-113415.json |  |  | 0.574 |
| walker |  | 9369 | 14 | listing of 'examples/airline/evals/eval_results' |  |  | 0.578 |
| ns | 9523 |  | 175 | Build backend and formatting toolchain | 6.5 |  | 0.586 |
| ns | 9662 |  | 139 | `customer_service_streaming/src` and `configs`: complete listings | 7.1 |  | 0.599 |
| ns | 9882 |  | 220 | The legacy example's own `Swarm` class and its config knobs | 7.2 | 7.1 | 0.591 |
| ns | 9905 |  | 23 | Remaining asset and log directories | 7.3 |  | 0.592 |
| walker |  | 9925 | 556 | README.md section #7 |  |  | 0.596 |
