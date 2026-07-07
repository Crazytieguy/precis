Score(3000)=0.521 I=0.817 C=0.332 ns_rows≤3K=18/68 (reached=7 partial=0 missing=11)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 63 | 63 | listing of '.' |  |  | 1.000 |
| ns | 63 |  | 63 | Repo root listing | 1.1 |  | 1.000 |
| ns | 122 |  | 59 | pyproject.toml - package name/version/description | 1.2 |  | 0.874 |
| ns | 141 |  | 19 | README - title line | 1.3 |  | 0.849 |
| walker |  | 166 | 103 | README headline in README.md |  |  | 0.879 |
| walker |  | 175 | 9 | listing of 'notebooks' |  |  | 0.879 |
| walker |  | 211 | 36 | listing of 'xlstm' |  |  | 0.889 |
| walker |  | 231 | 20 | listing of 'xlstm/blocks' |  |  | 0.902 |
| walker |  | 252 | 21 | listing of 'xlstm/blocks/slstm' |  |  | 0.902 |
| ns | 258 |  | 117 | xlstm/ package-root + subpackage listings | 1.4 |  | 0.645 |
| walker |  | 272 | 20 | listing of 'xlstm/blocks/slstm/src' |  |  | 0.645 |
| walker |  | 288 | 16 | listing of 'xlstm/blocks/slstm/src/vanilla' |  |  | 0.645 |
| walker |  | 311 | 23 | listing of 'xlstm/blocks/mlstm' |  |  | 0.645 |
| ns | 335 |  | 77 | README - About paragraph (the xLSTM idea) | 1.5 |  | 0.620 |
| walker |  | 339 | 28 | listing of 'xlstm/xlstm_large' |  |  | 0.727 |
| walker |  | 372 | 33 | listing of 'xlstm/components' |  |  | 0.898 |
| walker |  | 414 | 42 | python imports in xlstm/xlstm_large/__init__.py |  |  | 0.898 |
| ns | 459 |  | 124 | README - xLSTM 7B / xLSTM Large callout | 1.6 |  | 0.837 |
| walker |  | 460 | 46 | listing of 'experiments' |  |  | 0.837 |
| walker |  | 474 | 14 | listing of 'experiments/data' |  |  | 0.838 |
| walker |  | 498 | 24 | listing of 'experiments/data/formal_language' |  |  | 0.839 |
| walker |  | 524 | 26 | listing of 'experiments/data/formal_language/tasks' |  |  | 0.840 |
| walker |  | 527 | 3 | listing of '.github' |  |  | 0.840 |
| walker |  | 535 | 8 | listing of '.github/workflows' |  |  | 0.840 |
| walker |  | 580 | 45 | python decl names surface in xlstm/blocks/slstm/src/vanilla/__init__.py |  |  | 0.840 |
| walker |  | 594 | 14 | python decl names surface in experiments/metrics.py |  |  | 0.840 |
| walker |  | 594 | 0 | python decl at experiments/metrics.py:9 |  |  | 0.840 |
| walker |  | 622 | 28 | listing of 'res' |  |  | 0.840 |
| ns | 641 |  | 182 | xlstm/__init__.py - public package API | 2.1 |  | 0.767 |
| walker |  | 724 | 102 | python imports in xlstm/blocks/slstm/src/vanilla/__init__.py |  |  | 0.767 |
| walker |  | 739 | 15 | listing of 'notebooks/xlstm_large' |  |  | 0.767 |
| walker |  | 776 | 37 | python decl at xlstm/blocks/slstm/src/vanilla/__init__.py:11 |  |  | 0.767 |
| ns | 825 |  | 184 | xlstm_block_stack.py - class/def locations | 2.2 |  | 0.691 |
| walker |  | 958 | 182 | python imports in xlstm/__init__.py |  |  | 0.775 |
| walker |  | 985 | 27 | python decl names surface in xlstm/xlstm_block_stack.py |  |  | 0.776 |
| walker |  | 985 | 0 | python decl at xlstm/xlstm_block_stack.py:77 |  |  | 0.776 |
| ns | 989 |  | 164 | xlstm_lm_model.py - class/def locations | 2.3 |  | 0.726 |
| walker |  | 997 | 12 | python decl at xlstm/xlstm_block_stack.py:15 |  |  | 0.728 |
| walker |  | 1012 | 15 | python class body at xlstm/xlstm_block_stack.py:77 |  |  | 0.728 |
| walker |  | 1041 | 29 | python decl names surface in xlstm/utils.py |  |  | 0.728 |
| walker |  | 1041 | 0 | python decl at xlstm/utils.py:32 |  |  | 0.728 |
| walker |  | 1051 | 10 | python decl at xlstm/utils.py:11 |  |  | 0.728 |
| walker |  | 1084 | 33 | listing of 'notebooks/xlstm' |  |  | 0.729 |
| ns | 1110 |  | 121 | xlstm/utils.py - class/def locations | 2.4 |  | 0.694 |
| walker |  | 1119 | 35 | python class body at experiments/metrics.py:9 |  |  | 0.694 |
| walker |  | 1154 | 35 | python decl names surface in xlstm/xlstm_lm_model.py |  |  | 0.695 |
| walker |  | 1154 | 0 | python decl at xlstm/xlstm_lm_model.py:22 |  |  | 0.695 |
| walker |  | 1173 | 19 | python decl at xlstm/xlstm_lm_model.py:14 |  |  | 0.698 |
| walker |  | 1189 | 16 | python class body at xlstm/xlstm_lm_model.py:22 |  |  | 0.698 |
| ns | 1250 |  | 140 | blocks/xlstm_block.py - class/def locations | 2.5 |  | 0.653 |
| walker |  | 1375 | 186 | headings outline in README.md |  |  | 0.653 |
| walker |  | 1389 | 14 | python decl names surface in xlstm/xlstm_large/from_pretrained.py |  |  | 0.653 |
| walker |  | 1474 | 85 | [package] in pyproject.toml |  |  | 0.693 |
| walker |  | 1514 | 40 | python decl names surface in experiments/lr_scheduler.py |  |  | 0.693 |
| walker |  | 1514 | 0 | python decl at experiments/lr_scheduler.py:9 |  |  | 0.693 |
| walker |  | 1514 | 0 | python decl at experiments/lr_scheduler.py:24 |  |  | 0.693 |
| walker |  | 1577 | 63 | python method sigs in experiments/metrics.py |  |  | 0.693 |
| walker |  | 1577 | 0 | python method at experiments/metrics.py:13 |  |  | 0.693 |
| walker |  | 1577 | 0 | python method at experiments/metrics.py:17 |  |  | 0.693 |
| walker |  | 1577 | 0 | python method at experiments/metrics.py:22 |  |  | 0.693 |
| walker |  | 1577 | 0 | python method at experiments/metrics.py:25 |  |  | 0.693 |
| walker |  | 1586 | 9 | python method body at experiments/metrics.py:22 body 23 |  |  | 0.693 |
| walker |  | 1636 | 50 | python class body at xlstm/xlstm_lm_model.py:14 |  |  | 0.693 |
| ns | 1640 |  | 390 | blocks/mlstm/{block,layer,cell}.py - class/def locations | 2.6 |  | 0.599 |
| ns | 1722 |  | 82 | blocks/mlstm/backends.py - function locations | 2.7 |  | 0.586 |
| walker |  | 1823 | 187 | [dependencies] in pyproject.toml |  |  | 0.586 |
| walker |  | 1876 | 53 | listing of 'tests' |  |  | 0.587 |
| walker |  | 1941 | 65 | python decl names surface in experiments/main.py |  |  | 0.587 |
| walker |  | 1941 | 0 | python decl at experiments/main.py:32 |  |  | 0.587 |
| walker |  | 1941 | 0 | python decl at experiments/main.py:37 |  |  | 0.587 |
| walker |  | 1958 | 17 | python decl at experiments/main.py:21 |  |  | 0.587 |
| walker |  | 2002 | 44 | python decl at experiments/main.py:25 |  |  | 0.587 |
| ns | 2004 |  | 282 | blocks/slstm/{block,layer}.py - class/def locations | 2.8 |  | 0.536 |
| walker |  | 2030 | 28 | python decl body at experiments/main.py:32 body 33 |  |  | 0.536 |
| walker |  | 2056 | 26 | python decl names surface in xlstm/blocks/xlstm_block.py |  |  | 0.536 |
| walker |  | 2056 | 0 | python decl at xlstm/blocks/xlstm_block.py:43 |  |  | 0.536 |
| walker |  | 2067 | 11 | python decl at xlstm/blocks/xlstm_block.py:16 |  |  | 0.538 |
| walker |  | 2083 | 16 | python class body at xlstm/blocks/xlstm_block.py:43 |  |  | 0.538 |
| walker |  | 2109 | 26 | python decl names surface in xlstm/components/linear_headwise.py |  |  | 0.538 |
| walker |  | 2109 | 0 | python decl at xlstm/components/linear_headwise.py:42 |  |  | 0.538 |
| walker |  | 2120 | 11 | python decl at xlstm/components/linear_headwise.py:12 |  |  | 0.538 |
| walker |  | 2136 | 16 | python class body at xlstm/components/linear_headwise.py:42 |  |  | 0.538 |
| walker |  | 2147 | 11 | python decl names surface in experiments/data/formal_language/tasks/parity.py |  |  | 0.538 |
| walker |  | 2246 | 99 | python method sigs in xlstm/xlstm_lm_model.py |  |  | 0.556 |
| walker |  | 2246 | 0 | python method at xlstm/xlstm_lm_model.py:25 |  |  | 0.556 |
| walker |  | 2246 | 0 | python method at xlstm/xlstm_lm_model.py:41 |  |  | 0.556 |
| walker |  | 2246 | 0 | python method at xlstm/xlstm_lm_model.py:49 |  |  | 0.556 |
| walker |  | 2246 | 0 | python method at xlstm/xlstm_lm_model.py:65 |  |  | 0.556 |
| walker |  | 2258 | 12 | python decl names surface in experiments/data/formal_language/tasks/even_pairs.py |  |  | 0.556 |
| ns | 2314 |  | 310 | blocks/slstm/cell.py - sLSTMCellConfig + sLSTMCellBase - class/def locations | 2.9 |  | 0.512 |
| walker |  | 2395 | 137 | python method sigs in xlstm/xlstm_block_stack.py |  |  | 0.540 |
| walker |  | 2395 | 0 | python method at xlstm/xlstm_block_stack.py:40 |  |  | 0.540 |
| walker |  | 2395 | 0 | python method at xlstm/xlstm_block_stack.py:52 |  |  | 0.540 |
| walker |  | 2395 | 0 | python method at xlstm/xlstm_block_stack.py:80 |  |  | 0.540 |
| walker |  | 2395 | 0 | python method at xlstm/xlstm_block_stack.py:90 |  |  | 0.540 |
| walker |  | 2395 | 0 | python method at xlstm/xlstm_block_stack.py:111 |  |  | 0.540 |
| walker |  | 2395 | 0 | python method at xlstm/xlstm_block_stack.py:117 |  |  | 0.540 |
| walker |  | 2403 | 8 | python method at xlstm/xlstm_block_stack.py:36 |  |  | 0.540 |
| walker |  | 2432 | 29 | python decl names surface in xlstm/components/ln.py |  |  | 0.540 |
| walker |  | 2432 | 0 | python decl at xlstm/components/ln.py:8 |  |  | 0.540 |
| walker |  | 2432 | 0 | python decl at xlstm/components/ln.py:51 |  |  | 0.540 |
| walker |  | 2454 | 22 | python decl doc at xlstm/components/ln.py:8 |  |  | 0.540 |
| ns | 2568 |  | 254 | blocks/slstm/cell.py - CUDA/vanilla backend classes - class/def locations | 2.10 |  | 0.513 |
| walker |  | 2658 | 204 | manifest config in pyproject.toml |  |  | 0.513 |
| walker |  | 2714 | 56 | python method at xlstm/xlstm_block_stack.py:126 |  |  | 0.542 |
| walker |  | 2803 | 89 | python class body at xlstm/utils.py:11 |  |  | 0.542 |
| ns | 2842 |  | 274 | components/{conv,feedforward,linear_headwise}.py - class/def locations | 2.11 |  | 0.511 |
| walker |  | 2862 | 59 | python method at xlstm/xlstm_lm_model.py:56 |  |  | 0.532 |
| ns | 2976 |  | 134 | components/{ln,init,util}.py - class/def locations | 2.12 |  | 0.521 |
| walker |  | 3036 | 174 | python decl at xlstm/blocks/slstm/src/vanilla/__init__.py:77 |  |  | 0.521 |
| walker |  | 3212 | 176 | python decl at xlstm/blocks/slstm/src/vanilla/__init__.py:17 |  |  | 0.521 |
| ns | 3222 |  | 246 | xlstm_large/{model,components}.py - class/def locations | 2.13 |  | 0.498 |
| ns | 3342 |  | 120 | xlstm_large/{generate,from_pretrained,utils}.py - class/def locations + __init__.py | 2.14 |  | 0.493 |
| ns | 3653 |  | 311 | README - xLSTM Large quickstart code | 3.1 |  | 0.468 |
| walker |  | 3728 | 516 | README.md section #0 |  |  | 0.485 |
| walker |  | 3744 | 16 | python imports in xlstm/xlstm_large/utils.py |  |  | 0.485 |
| walker |  | 3872 | 128 | python method sigs in experiments/lr_scheduler.py |  |  | 0.485 |
| walker |  | 3872 | 0 | python method at experiments/lr_scheduler.py:10 |  |  | 0.485 |
| walker |  | 3872 | 0 | python method at experiments/lr_scheduler.py:25 |  |  | 0.485 |
| walker |  | 3872 | 0 | python method at experiments/lr_scheduler.py:47 |  |  | 0.485 |
| walker |  | 3880 | 8 | python method at experiments/lr_scheduler.py:33 |  |  | 0.485 |
| walker |  | 3889 | 9 | python method at experiments/lr_scheduler.py:13 |  |  | 0.485 |
| walker |  | 3898 | 9 | python method at experiments/lr_scheduler.py:18 |  |  | 0.485 |
| walker |  | 3912 | 14 | python method doc at experiments/lr_scheduler.py:18 |  |  | 0.485 |
| walker |  | 3928 | 16 | python method doc at experiments/lr_scheduler.py:13 |  |  | 0.485 |
| walker |  | 3944 | 16 | python method doc at experiments/lr_scheduler.py:47 |  |  | 0.485 |
| ns | 3974 |  | 321 | README - non-CUDA hardware recommendation + sLSTM CUDA build env var | 3.2 |  | 0.474 |
| walker |  | 4194 | 250 | python class body at xlstm/xlstm_block_stack.py:15 |  |  | 0.474 |
| walker |  | 4269 | 75 | python method sigs in xlstm/components/ln.py |  |  | 0.474 |
| walker |  | 4269 | 0 | python method at xlstm/components/ln.py:36 |  |  | 0.474 |
| walker |  | 4269 | 0 | python method at xlstm/components/ln.py:41 |  |  | 0.474 |
| walker |  | 4269 | 0 | python method at xlstm/components/ln.py:53 |  |  | 0.474 |
| walker |  | 4277 | 8 | python method at xlstm/components/ln.py:27 |  |  | 0.474 |
| walker |  | 4315 | 38 | python decl names surface in xlstm/xlstm_large/utils.py |  |  | 0.481 |
| walker |  | 4315 | 0 | python decl at xlstm/xlstm_large/utils.py:5 |  |  | 0.481 |
| walker |  | 4343 | 28 | python decl at xlstm/xlstm_large/utils.py:10 |  |  | 0.481 |
| walker |  | 4360 | 17 | python decl doc at xlstm/xlstm_large/utils.py:5 |  |  | 0.481 |
| walker |  | 4382 | 22 | python decl body at xlstm/xlstm_large/utils.py:5 body 7 |  |  | 0.481 |
| ns | 4420 |  | 446 | README - xLSTMBlockStack usage code (NeurIPS architecture) | 3.3 |  | 0.446 |
| walker |  | 4421 | 39 | python decl names surface in xlstm/components/conv.py |  |  | 0.448 |
| walker |  | 4421 | 0 | python decl at xlstm/components/conv.py:55 |  |  | 0.448 |
| walker |  | 4433 | 12 | python decl at xlstm/components/conv.py:12 |  |  | 0.449 |
| walker |  | 4496 | 63 | python decl at xlstm/components/conv.py:24 |  |  | 0.449 |
| walker |  | 4576 | 80 | python method sigs in xlstm/components/linear_headwise.py |  |  | 0.455 |
| walker |  | 4576 | 0 | python method at xlstm/components/linear_headwise.py:31 |  |  | 0.455 |
| walker |  | 4576 | 0 | python method at xlstm/components/linear_headwise.py:49 |  |  | 0.455 |
| walker |  | 4576 | 0 | python method at xlstm/components/linear_headwise.py:75 |  |  | 0.455 |
| walker |  | 4576 | 0 | python method at xlstm/components/linear_headwise.py:84 |  |  | 0.455 |
| walker |  | 4585 | 9 | python method at xlstm/components/linear_headwise.py:67 |  |  | 0.455 |
| walker |  | 4607 | 22 | python method body at experiments/metrics.py:25 body 26 |  |  | 0.455 |
| ns | 4702 |  | 282 | README - xLSTMLMModel usage code + yaml/dacite config pattern | 3.4 |  | 0.441 |
| walker |  | 4755 | 148 | python method sigs in xlstm/utils.py |  |  | 0.463 |
| walker |  | 4755 | 0 | python method at xlstm/utils.py:20 |  |  | 0.463 |
| walker |  | 4755 | 0 | python method at xlstm/utils.py:33 |  |  | 0.463 |
| walker |  | 4755 | 0 | python method at xlstm/utils.py:36 |  |  | 0.463 |
| walker |  | 4755 | 0 | python method at xlstm/utils.py:61 |  |  | 0.463 |
| walker |  | 4755 | 0 | python method at xlstm/utils.py:77 |  |  | 0.463 |
| walker |  | 4765 | 10 | python method body at xlstm/utils.py:33 body 34 |  |  | 0.463 |
| ns | 4775 |  | 73 | xLSTMBlockStackConfig.__post_init__ - slstm_at resolution + per-block config propagation | 4.1 | 2.2 | 0.459 |
| walker |  | 4791 | 26 | python decl names surface in xlstm/blocks/mlstm/cell.py |  |  | 0.459 |
| walker |  | 4791 | 0 | python decl at xlstm/blocks/mlstm/cell.py:20 |  |  | 0.459 |
| walker |  | 4802 | 11 | python decl at xlstm/blocks/mlstm/cell.py:13 |  |  | 0.459 |
| walker |  | 4816 | 14 | python class body at xlstm/blocks/mlstm/cell.py:20 |  |  | 0.459 |
| walker |  | 4853 | 37 | python class body at xlstm/blocks/mlstm/cell.py:13 |  |  | 0.459 |
| walker |  | 4895 | 42 | python decl names surface in xlstm/components/util.py |  |  | 0.464 |
| walker |  | 4895 | 0 | python decl at xlstm/components/util.py:7 |  |  | 0.464 |
| walker |  | 4895 | 0 | python decl at xlstm/components/util.py:11 |  |  | 0.464 |
| walker |  | 4895 | 0 | python decl at xlstm/components/util.py:26 |  |  | 0.464 |
| walker |  | 4919 | 24 | python decl doc at xlstm/components/util.py:11 |  |  | 0.464 |
| walker |  | 4937 | 18 | python decl body at xlstm/components/util.py:7 body 8 |  |  | 0.464 |
| ns | 5010 |  | 235 | xLSTMBlockStack._create_blocks - block_map-driven mLSTM/sLSTM dispatch | 4.2 | 2.2 | 0.451 |
| walker |  | 5043 | 106 | python method sigs in xlstm/components/util.py |  |  | 0.458 |
| walker |  | 5043 | 0 | python method at xlstm/components/util.py:58 |  |  | 0.458 |
| walker |  | 5043 | 0 | python method at xlstm/components/util.py:73 |  |  | 0.458 |
| walker |  | 5051 | 8 | python method at xlstm/components/util.py:61 |  |  | 0.458 |
| walker |  | 5059 | 8 | python method at xlstm/components/util.py:65 |  |  | 0.458 |
| walker |  | 5067 | 8 | python method at xlstm/components/util.py:69 |  |  | 0.458 |
| walker |  | 5084 | 17 | python method at xlstm/components/util.py:46 |  |  | 0.458 |
| walker |  | 5102 | 18 | python method at xlstm/components/util.py:51 |  |  | 0.458 |
| walker |  | 5110 | 8 | python method body at experiments/lr_scheduler.py:13 body 16 |  |  | 0.458 |
| walker |  | 5137 | 27 | python decl names surface in xlstm/blocks/mlstm/layer.py |  |  | 0.458 |
| walker |  | 5137 | 0 | python decl at xlstm/blocks/mlstm/layer.py:39 |  |  | 0.458 |
| walker |  | 5154 | 17 | python decl at xlstm/blocks/mlstm/layer.py:18 |  |  | 0.459 |
| walker |  | 5169 | 15 | python class body at xlstm/blocks/mlstm/layer.py:39 |  |  | 0.459 |
| walker |  | 5196 | 27 | python decl names surface in xlstm/blocks/slstm/layer.py |  |  | 0.449 |
| walker |  | 5196 | 0 | python decl at xlstm/blocks/slstm/layer.py:33 |  |  | 0.449 |
| ns | 5196 |  | 186 | xLSTMLMModel.__init__ - embedding/lm_head/tie_weights | 4.3 | 2.3 | 0.449 |
| walker |  | 5213 | 17 | python decl at xlstm/blocks/slstm/layer.py:18 |  |  | 0.450 |
| walker |  | 5228 | 15 | python class body at xlstm/blocks/slstm/layer.py:33 |  |  | 0.450 |
| walker |  | 5315 | 87 | python method sigs in xlstm/components/conv.py |  |  | 0.462 |
| walker |  | 5315 | 0 | python method at xlstm/components/conv.py:20 |  |  | 0.462 |
| walker |  | 5315 | 0 | python method at xlstm/components/conv.py:71 |  |  | 0.462 |
| walker |  | 5315 | 0 | python method at xlstm/components/conv.py:95 |  |  | 0.462 |
| walker |  | 5323 | 8 | python method body at xlstm/components/conv.py:95 body 96 |  |  | 0.462 |
| ns | 5339 |  | 143 | xLSTMLMModel.forward + step | 4.4 | 2.3 | 0.455 |
| walker |  | 5344 | 21 | python method doc at xlstm/xlstm_block_stack.py:40 |  |  | 0.455 |
| walker |  | 5372 | 28 | python decl names surface in xlstm/blocks/mlstm/block.py |  |  | 0.456 |
| walker |  | 5372 | 0 | python decl at xlstm/blocks/mlstm/block.py:22 |  |  | 0.456 |
| walker |  | 5383 | 11 | python decl at xlstm/blocks/mlstm/block.py:9 |  |  | 0.457 |
| walker |  | 5397 | 14 | python class body at xlstm/blocks/mlstm/block.py:22 |  |  | 0.457 |
| walker |  | 5433 | 36 | python method sigs in xlstm/blocks/mlstm/block.py |  |  | 0.459 |
| walker |  | 5433 | 0 | python method at xlstm/blocks/mlstm/block.py:17 |  |  | 0.459 |
| walker |  | 5433 | 0 | python method at xlstm/blocks/mlstm/block.py:25 |  |  | 0.459 |
| walker |  | 5461 | 28 | python decl names surface in xlstm/blocks/slstm/block.py |  |  | 0.460 |
| walker |  | 5461 | 0 | python decl at xlstm/blocks/slstm/block.py:29 |  |  | 0.460 |
| walker |  | 5472 | 11 | python decl at xlstm/blocks/slstm/block.py:12 |  |  | 0.460 |
| walker |  | 5486 | 14 | python class body at xlstm/blocks/slstm/block.py:29 |  |  | 0.460 |
| walker |  | 5519 | 33 | python method sigs in xlstm/blocks/slstm/block.py |  |  | 0.462 |
| walker |  | 5519 | 0 | python method at xlstm/blocks/slstm/block.py:21 |  |  | 0.462 |
| walker |  | 5519 | 0 | python method at xlstm/blocks/slstm/block.py:32 |  |  | 0.462 |
| ns | 5555 |  | 216 | xLSTMLMModel._create_weight_decay_optim_groups - embedding weight-decay override | 4.5 | 2.3 | 0.452 |
| walker |  | 5588 | 69 | python class body at xlstm/components/conv.py:12 |  |  | 0.452 |
| walker |  | 5641 | 53 | python decl doc at xlstm/blocks/xlstm_block.py:43 |  |  | 0.462 |
| walker |  | 5694 | 53 | python decl doc at xlstm/components/linear_headwise.py:42 |  |  | 0.462 |
| ns | 5702 |  | 147 | UpProjConfigMixin._set_proj_up_dim - proj-up dimension rounding rule | 4.6 | 2.4 | 0.456 |
| walker |  | 5744 | 50 | python method at xlstm/components/conv.py:131 |  |  | 0.456 |
| walker |  | 5759 | 15 | python decl names surface in xlstm/blocks/slstm/src/vanilla/lstm.py |  |  | 0.456 |
| walker |  | 5774 | 15 | python decl names surface in xlstm/blocks/slstm/src/vanilla/slstm.py |  |  | 0.456 |
| walker |  | 5828 | 54 | python method doc at xlstm/utils.py:36 |  |  | 0.456 |
| walker |  | 5894 | 66 | python method sigs in xlstm/blocks/slstm/layer.py |  |  | 0.463 |
| walker |  | 5894 | 0 | python method at xlstm/blocks/slstm/layer.py:28 |  |  | 0.463 |
| walker |  | 5894 | 0 | python method at xlstm/blocks/slstm/layer.py:36 |  |  | 0.463 |
| walker |  | 5894 | 0 | python method at xlstm/blocks/slstm/layer.py:84 |  |  | 0.463 |
| walker |  | 5950 | 56 | python method at xlstm/components/conv.py:110 |  |  | 0.463 |
| walker |  | 5958 | 8 | python method body at experiments/lr_scheduler.py:18 body 21 |  |  | 0.463 |
| ns | 5968 |  | 266 | WeightDecayOptimGroupMixin._create_weight_decay_optim_groups - default decay/no-decay heuristic | 4.7 | 2.4 | 0.452 |
| walker |  | 6012 | 54 | python decl names surface in experiments/data/utils.py |  |  | 0.452 |
| walker |  | 6012 | 0 | python decl at experiments/data/utils.py:17 |  |  | 0.452 |
| walker |  | 6012 | 0 | python decl at experiments/data/utils.py:39 |  |  | 0.452 |
| walker |  | 6012 | 0 | python decl at experiments/data/utils.py:55 |  |  | 0.452 |
| walker |  | 6012 | 0 | python decl at experiments/data/utils.py:90 |  |  | 0.452 |
| ns | 6030 |  | 62 | xLSTMBlockConfig.__post_init__ - mutual-exclusion invariant | 5.1 | 2.5 | 0.451 |
| ns | 6106 |  | 76 | xLSTMBlock.forward - pre-norm residual pattern | 5.2 | 2.5 | 0.448 |
| walker |  | 6122 | 110 | python method sigs in xlstm/blocks/xlstm_block.py |  |  | 0.464 |
| walker |  | 6122 | 0 | python method at xlstm/blocks/xlstm_block.py:27 |  |  | 0.464 |
| walker |  | 6122 | 0 | python method at xlstm/blocks/xlstm_block.py:51 |  |  | 0.464 |
| walker |  | 6122 | 0 | python method at xlstm/blocks/xlstm_block.py:76 |  |  | 0.464 |
| walker |  | 6122 | 0 | python method at xlstm/blocks/xlstm_block.py:82 |  |  | 0.464 |
| walker |  | 6122 | 0 | python method at xlstm/blocks/xlstm_block.py:89 |  |  | 0.464 |
| walker |  | 6178 | 56 | python method doc at xlstm/utils.py:61 |  |  | 0.464 |
| walker |  | 6201 | 23 | python decl names surface in experiments/data/formal_language/tasks/cycle_navigation.py |  |  | 0.464 |
| ns | 6207 |  | 101 | mLSTMLayer.__init__ - q/k/v projected via headwise num_proj_heads | 5.3 | 2.6 | 0.460 |
| walker |  | 6227 | 26 | python decl at experiments/data/formal_language/tasks/cycle_navigation.py:51 |  |  | 0.460 |
| ns | 6305 |  | 98 | mLSTMCell.forward - backend_fn dispatch call | 5.4 | 2.6 | 0.455 |
| walker |  | 6414 | 187 | python method sigs in experiments/data/utils.py |  |  | 0.455 |
| walker |  | 6414 | 0 | python method at experiments/data/utils.py:41 |  |  | 0.455 |
| walker |  | 6420 | 6 | python method at experiments/data/utils.py:57 |  |  | 0.455 |
| walker |  | 6428 | 8 | python method at experiments/data/utils.py:46 |  |  | 0.455 |
| walker |  | 6436 | 8 | python method at experiments/data/utils.py:50 |  |  | 0.455 |
| walker |  | 6444 | 8 | python method at experiments/data/utils.py:77 |  |  | 0.455 |
| walker |  | 6452 | 8 | python method at experiments/data/utils.py:85 |  |  | 0.455 |
| ns | 6461 |  | 156 | mLSTMCell.step - S==1 assertion + zero-state shapes | 5.5 | 2.6 | 0.452 |
| walker |  | 6467 | 15 | python method at experiments/data/utils.py:19 |  |  | 0.452 |
| walker |  | 6484 | 17 | python method at experiments/data/utils.py:24 |  |  | 0.452 |
| walker |  | 6501 | 17 | python method at experiments/data/utils.py:29 |  |  | 0.452 |
| walker |  | 6518 | 17 | python method at experiments/data/utils.py:34 |  |  | 0.452 |
| walker |  | 6557 | 39 | python method at xlstm/utils.py:97 |  |  | 0.452 |
| walker |  | 6631 | 74 | python method sigs in xlstm/blocks/mlstm/cell.py |  |  | 0.456 |
| walker |  | 6631 | 0 | python method at xlstm/blocks/mlstm/cell.py:23 |  |  | 0.456 |
| walker |  | 6631 | 0 | python method at xlstm/blocks/mlstm/cell.py:43 |  |  | 0.456 |
| walker |  | 6631 | 0 | python method at xlstm/blocks/mlstm/cell.py:133 |  |  | 0.456 |
| walker |  | 6693 | 62 | python method at xlstm/components/util.py:34 |  |  | 0.456 |
| walker |  | 6719 | 26 | python decl names surface in experiments/data/formal_language/tasks/modular_arithmetic.py |  |  | 0.457 |
| ns | 6741 |  | 280 | parallel_stabilized_simple - stabilized log-D-matrix core equation | 5.6 | 2.7 | 0.450 |
| walker |  | 6745 | 26 | python method at xlstm/components/conv.py:98 |  |  | 0.450 |
| walker |  | 6768 | 23 | python method body at experiments/metrics.py:13 body 14 |  |  | 0.450 |
| ns | 6948 |  | 207 | recurrent_step_stabilized_simple - (c,n,m) update rule | 5.7 | 2.7 | 0.446 |
| walker |  | 6981 | 213 | python class body at xlstm/components/conv.py:55 |  |  | 0.446 |
| ns | 7046 |  | 98 | sLSTMBlockConfig + sLSTMLayerConfig.__post_init__ - propagation rules | 6.1 | 2.8 | 0.442 |
| walker |  | 7211 | 230 | python class body at xlstm/components/linear_headwise.py:12 |  |  | 0.442 |
| ns | 7267 |  | 221 | sLSTMCellConfig - backend/function/bias_init/dtype knobs | 6.2 | 2.9 | 0.436 |
| walker |  | 7290 | 79 | python method sigs in xlstm/blocks/mlstm/layer.py |  |  | 0.443 |
| walker |  | 7290 | 0 | python method at xlstm/blocks/mlstm/layer.py:34 |  |  | 0.443 |
| walker |  | 7290 | 0 | python method at xlstm/blocks/mlstm/layer.py:42 |  |  | 0.443 |
| walker |  | 7290 | 0 | python method at xlstm/blocks/mlstm/layer.py:101 |  |  | 0.443 |
| walker |  | 7300 | 10 | python method at xlstm/blocks/mlstm/layer.py:158 |  |  | 0.443 |
| walker |  | 7340 | 40 | python decl names surface in xlstm/blocks/mlstm/backends.py |  |  | 0.445 |
| walker |  | 7370 | 30 | python imports in xlstm/xlstm_large/components.py |  |  | 0.445 |
| walker |  | 7411 | 41 | python decl names surface in experiments/data/formal_language/online_generate.py |  |  | 0.445 |
| walker |  | 7411 | 0 | python decl at experiments/data/formal_language/online_generate.py:9 |  |  | 0.445 |
| walker |  | 7411 | 0 | python decl at experiments/data/formal_language/online_generate.py:29 |  |  | 0.445 |
| walker |  | 7411 | 0 | python decl at experiments/data/formal_language/online_generate.py:41 |  |  | 0.445 |
| ns | 7435 |  | 168 | sLSTMCellBase.reset_parameters - powerlaw_blockdependent forget-gate init | 6.3 | 2.9 | 0.439 |
| walker |  | 7468 | 57 | python imports in experiments/metrics.py |  |  | 0.439 |
| walker |  | 7518 | 50 | python decl doc at experiments/data/formal_language/online_generate.py:41 |  |  | 0.439 |
| ns | 7519 |  | 84 | sLSTMCellBase.step/forward - dispatch to backend-specific _impl/_impl_step | 6.4 | 2.9 | 0.435 |
| walker |  | 7628 | 110 | python class body at xlstm/blocks/xlstm_block.py:16 |  |  | 0.435 |
| ns | 7643 |  | 124 | sLSTMCellCUDA JIT cache key + sLSTMCell.__new__ backend factory | 6.5 | 2.10 | 0.430 |
| walker |  | 7688 | 60 | python imports in experiments/lr_scheduler.py |  |  | 0.430 |
| ns | 7729 |  | 86 | vanilla pointwise gate equations - slstm vs plain lstm variant | 6.6 |  | 0.428 |
| ns | 7818 |  | 89 | cuda_init.py - load() JIT entry point + XLSTM_EXTRA_INCLUDE_PATHS env var | 6.7 |  | 0.426 |
| walker |  | 7868 | 180 | python method sigs in experiments/data/formal_language/online_generate.py |  |  | 0.426 |
| walker |  | 7868 | 0 | python method at experiments/data/formal_language/online_generate.py:11 |  |  | 0.426 |
| walker |  | 7868 | 0 | python method at experiments/data/formal_language/online_generate.py:19 |  |  | 0.426 |
| walker |  | 7868 | 0 | python method at experiments/data/formal_language/online_generate.py:25 |  |  | 0.426 |
| walker |  | 7868 | 0 | python method at experiments/data/formal_language/online_generate.py:30 |  |  | 0.426 |
| walker |  | 7868 | 0 | python method at experiments/data/formal_language/online_generate.py:34 |  |  | 0.426 |
| walker |  | 7868 | 0 | python method at experiments/data/formal_language/online_generate.py:37 |  |  | 0.426 |
| walker |  | 7868 | 0 | python method at experiments/data/formal_language/online_generate.py:47 |  |  | 0.426 |
| walker |  | 7868 | 0 | python method at experiments/data/formal_language/online_generate.py:54 |  |  | 0.426 |
| walker |  | 7868 | 0 | python method at experiments/data/formal_language/online_generate.py:59 |  |  | 0.426 |
| walker |  | 7868 | 0 | python method at experiments/data/formal_language/online_generate.py:63 |  |  | 0.426 |
| walker |  | 7876 | 8 | python method at experiments/data/formal_language/online_generate.py:67 |  |  | 0.426 |
| walker |  | 7884 | 8 | python method at experiments/data/formal_language/online_generate.py:71 |  |  | 0.426 |
| walker |  | 7892 | 8 | python method body at experiments/data/formal_language/online_generate.py:25 body 26 |  |  | 0.426 |
| walker |  | 7927 | 35 | python imports in xlstm/xlstm_large/generate.py |  |  | 0.426 |
| ns | 7956 |  | 138 | sLSTM CUDA/C++ source tree listing | 6.8 |  | 0.417 |
| ns | 8021 |  | 65 | slstm.h - ForwardPass/BackwardPass/BackwardPassCut class + Run() locations | 6.9 |  | 0.415 |
| walker |  | 8042 | 115 | python decl at xlstm/xlstm_large/from_pretrained.py:9 |  |  | 0.415 |
| walker |  | 8116 | 74 | python method at xlstm/components/ln.py:11 |  |  | 0.415 |
| ns | 8118 |  | 97 | CausalConv1d.step - rolling-buffer single-step convolution | 7.1 | 2.11 | 0.413 |
| walker |  | 8191 | 75 | python class body at xlstm/blocks/mlstm/block.py:9 |  |  | 0.413 |
| ns | 8232 |  | 114 | LinearHeadwiseExpand.forward - per-head einsum projection | 7.2 | 2.11 | 0.410 |
| ns | 8270 |  | 38 | init.py - small_init_ / wang_init_ std-dev formulas | 7.3 | 2.12 | 0.409 |
| ns | 8415 |  | 145 | xLSTMLargeConfig - chunkwise_kernel/mode/weight_mode doc'd fields | 8.1 | 2.13 | 0.406 |
| ns | 8532 |  | 117 | xLSTMLarge.forward + mLSTMLayer.forward - soft_cap gate/logit capping | 8.2 | 2.13 | 0.403 |
| ns | 8568 |  | 36 | components.py - soft_cap formula | 8.3 | 2.13 | 0.402 |
| walker |  | 8718 | 527 | plaintext config setup.cfg |  |  | 0.402 |
| walker |  | 8727 | 9 | python method body at experiments/data/formal_language/online_generate.py:37 body 38 |  |  | 0.402 |
| walker |  | 8822 | 95 | python decl doc at xlstm/components/util.py:26 |  |  | 0.402 |
| ns | 8826 |  | 258 | from_pretrained.py - sharded safetensors loading + forced single weight_mode | 8.4 | 2.14 | 0.396 |
| walker |  | 8827 | 5 | python method body at experiments/data/utils.py:19 body 22 |  |  | 0.396 |
| walker |  | 8910 | 83 | python decl names surface in xlstm/components/feedforward.py |  |  | 0.405 |
| walker |  | 8910 | 0 | python decl at xlstm/components/feedforward.py:22 |  |  | 0.405 |
| walker |  | 8910 | 0 | python decl at xlstm/components/feedforward.py:49 |  |  | 0.405 |
| walker |  | 8910 | 0 | python decl at xlstm/components/feedforward.py:90 |  |  | 0.405 |
| walker |  | 8924 | 14 | python decl at xlstm/components/feedforward.py:31 |  |  | 0.409 |
| walker |  | 8936 | 12 | python class body at xlstm/components/feedforward.py:49 |  |  | 0.431 |
| ns | 8936 |  | 110 | experiments/ + data/formal_language/{,tasks}/ directory listings | 9.1 |  | 0.431 |
| walker |  | 9002 | 66 | python method sigs in xlstm/components/feedforward.py |  |  | 0.438 |
| walker |  | 9002 | 0 | python method at xlstm/components/feedforward.py:42 |  |  | 0.438 |
| walker |  | 9002 | 0 | python method at xlstm/components/feedforward.py:52 |  |  | 0.438 |
| walker |  | 9002 | 0 | python method at xlstm/components/feedforward.py:72 |  |  | 0.438 |
| walker |  | 9002 | 0 | python method at xlstm/components/feedforward.py:77 |  |  | 0.438 |
| ns | 9029 |  | 93 | main.py - CLI entry point + dataset registry | 9.2 |  | 0.437 |
| walker |  | 9108 | 106 | python class body at xlstm/components/feedforward.py:31 |  |  | 0.437 |
| walker |  | 9164 | 56 | python method at xlstm/blocks/slstm/layer.py:92 |  |  | 0.446 |
| ns | 9209 |  | 180 | lr_scheduler.py - LinearWarmupCosineAnnealing.compute_lr formula | 9.3 |  | 0.442 |
| walker |  | 9241 | 77 | python imports in xlstm/utils.py |  |  | 0.442 |
| walker |  | 9330 | 89 | python decl names surface in xlstm/components/init.py |  |  | 0.449 |
| walker |  | 9330 | 0 | python decl at xlstm/components/init.py:8 |  |  | 0.449 |
| walker |  | 9330 | 0 | python decl at xlstm/components/init.py:18 |  |  | 0.449 |
| walker |  | 9330 | 0 | python decl at xlstm/components/init.py:28 |  |  | 0.449 |
| ns | 9331 |  | 122 | FormLangDataset.__getitem__ - mask-based causal LM framing | 9.4 |  | 0.446 |
| walker |  | 9344 | 14 | python decl doc at xlstm/components/init.py:8 |  |  | 0.446 |
| walker |  | 9376 | 32 | python decl doc at xlstm/components/init.py:28 |  |  | 0.446 |
| ns | 9379 |  | 48 | tasks/{parity,cycle_navigation,even_pairs,modular_arithmetic}.py - function locations | 9.5 |  | 0.449 |
| walker |  | 9537 | 161 | python class body at xlstm/blocks/mlstm/layer.py:18 |  |  | 0.449 |
| ns | 9553 |  | 174 | parity_xlstm{01,10,11}.yaml - the mLSTM-only / sLSTM-only / mixed variants | 9.6 |  | 0.444 |
| walker |  | 9593 | 56 | listing of 'xlstm/blocks/slstm/src/cuda' |  |  | 0.448 |
| walker |  | 9605 | 12 | c decl names surface in xlstm/blocks/slstm/src/cuda/slstm.h |  |  | 0.448 |
| walker |  | 9605 | 0 | c decl at xlstm/blocks/slstm/src/cuda/slstm.h:44 |  |  | 0.448 |
| ns | 9606 |  | 53 | tests/ directory listing | 10.1 |  | 0.453 |
| walker |  | 9618 | 13 | c includes in xlstm/blocks/slstm/src/cuda/slstm.h |  |  | 0.453 |
| ns | 9653 |  | 47 | conftest.py - CUDA-required skip-all guard | 10.2 |  | 0.452 |
| ns | 9788 |  | 135 | Test-function locations across the pytest suite | 10.3 |  | 0.448 |
| ns | 9864 |  | 76 | res/ + notebooks/ directory listings | 11.1 |  | 0.455 |
| walker |  | 9871 | 253 | c header banner in xlstm/blocks/slstm/src/cuda/slstm.h |  |  | 0.455 |
| ns | 9872 |  | 8 | .github/workflows/ - no test/lint CI, only CLA + repo-sync | 11.2 |  | 0.456 |
| walker |  | 9886 | 15 | python method body at xlstm/xlstm_block_stack.py:36 body 38 |  |  | 0.456 |
| ns | 9887 |  | 15 | LICENSE header | 11.3 |  | 0.456 |
| ns | 9961 |  | 74 | pytest.ini - full | 11.4 |  | 0.454 |
| walker |  | 9980 | 94 | python decl doc at xlstm/components/init.py:18 |  |  | 0.454 |
| ns | 9996 |  | 35 | README - Citation BibTeX keys | 11.5 |  | 0.453 |
