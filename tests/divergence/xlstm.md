Score(3000)=0.500 I=0.806 C=0.310 ns_rows≤3K=18/68 (reached=6 partial=0 missing=12)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 63 | 63 | listing of '.' |  |  | 1.000 |
| ns | 63 |  | 63 | Repo root listing | 1.1 |  | 1.000 |
| walker |  | 72 | 9 | listing of 'notebooks' |  |  | 1.000 |
| ns | 122 |  | 59 | pyproject.toml - package name/version/description | 1.2 |  | 0.874 |
| ns | 141 |  | 19 | README - title line | 1.3 |  | 0.849 |
| walker |  | 175 | 103 | README headline in README.md |  |  | 0.879 |
| walker |  | 178 | 3 | listing of '.github' |  |  | 0.879 |
| walker |  | 186 | 8 | listing of '.github/workflows' |  |  | 0.879 |
| walker |  | 214 | 28 | listing of 'res' |  |  | 0.879 |
| walker |  | 250 | 36 | listing of 'xlstm' |  |  | 0.889 |
| ns | 258 |  | 117 | xlstm/ package-root + subpackage listings | 1.4 |  | 0.593 |
| walker |  | 270 | 20 | listing of 'xlstm/blocks' |  |  | 0.645 |
| walker |  | 291 | 21 | listing of 'xlstm/blocks/slstm' |  |  | 0.645 |
| walker |  | 311 | 20 | listing of 'xlstm/blocks/slstm/src' |  |  | 0.645 |
| walker |  | 327 | 16 | listing of 'xlstm/blocks/slstm/src/vanilla' |  |  | 0.645 |
| ns | 335 |  | 77 | README - About paragraph (the xLSTM idea) | 1.5 |  | 0.621 |
| walker |  | 350 | 23 | listing of 'xlstm/blocks/mlstm' |  |  | 0.621 |
| walker |  | 378 | 28 | listing of 'xlstm/xlstm_large' |  |  | 0.728 |
| walker |  | 411 | 33 | listing of 'xlstm/components' |  |  | 0.898 |
| walker |  | 453 | 42 | python imports in xlstm/xlstm_large/__init__.py |  |  | 0.898 |
| ns | 459 |  | 124 | README - xLSTM 7B / xLSTM Large callout | 1.6 |  | 0.837 |
| walker |  | 468 | 15 | listing of 'notebooks/xlstm_large' |  |  | 0.838 |
| walker |  | 514 | 46 | listing of 'experiments' |  |  | 0.838 |
| walker |  | 528 | 14 | listing of 'experiments/data' |  |  | 0.839 |
| walker |  | 552 | 24 | listing of 'experiments/data/formal_language' |  |  | 0.839 |
| walker |  | 578 | 26 | listing of 'experiments/data/formal_language/tasks' |  |  | 0.841 |
| walker |  | 623 | 45 | python decl names surface in xlstm/blocks/slstm/src/vanilla/__init__.py |  |  | 0.841 |
| ns | 641 |  | 182 | xlstm/__init__.py - public package API | 2.1 |  | 0.767 |
| walker |  | 656 | 33 | listing of 'notebooks/xlstm' |  |  | 0.768 |
| walker |  | 758 | 102 | python imports in xlstm/blocks/slstm/src/vanilla/__init__.py |  |  | 0.768 |
| walker |  | 772 | 14 | python decl names surface in experiments/metrics.py |  |  | 0.768 |
| walker |  | 772 | 0 | python decl at experiments/metrics.py:9 |  |  | 0.768 |
| walker |  | 809 | 37 | python decl at xlstm/blocks/slstm/src/vanilla/__init__.py:11 |  |  | 0.768 |
| ns | 825 |  | 184 | xlstm_block_stack.py - class/def locations | 2.2 |  | 0.692 |
| ns | 989 |  | 164 | xlstm_lm_model.py - class/def locations | 2.3 |  | 0.647 |
| walker |  | 991 | 182 | python imports in xlstm/__init__.py |  |  | 0.726 |
| walker |  | 1018 | 27 | python decl names surface in xlstm/xlstm_block_stack.py |  |  | 0.726 |
| walker |  | 1018 | 0 | python decl at xlstm/xlstm_block_stack.py:77 |  |  | 0.726 |
| walker |  | 1030 | 12 | python decl at xlstm/xlstm_block_stack.py:15 |  |  | 0.728 |
| walker |  | 1045 | 15 | python class body at xlstm/xlstm_block_stack.py:77 |  |  | 0.728 |
| walker |  | 1074 | 29 | python decl names surface in xlstm/utils.py |  |  | 0.728 |
| walker |  | 1074 | 0 | python decl at xlstm/utils.py:32 |  |  | 0.728 |
| walker |  | 1084 | 10 | python decl at xlstm/utils.py:11 |  |  | 0.729 |
| ns | 1110 |  | 121 | xlstm/utils.py - class/def locations | 2.4 |  | 0.694 |
| ns | 1250 |  | 140 | blocks/xlstm_block.py - class/def locations | 2.5 |  | 0.649 |
| walker |  | 1270 | 186 | headings outline in README.md |  |  | 0.649 |
| walker |  | 1355 | 85 | [package] in pyproject.toml |  |  | 0.689 |
| walker |  | 1390 | 35 | python class body at experiments/metrics.py:9 |  |  | 0.689 |
| walker |  | 1425 | 35 | python decl names surface in xlstm/xlstm_lm_model.py |  |  | 0.690 |
| walker |  | 1425 | 0 | python decl at xlstm/xlstm_lm_model.py:22 |  |  | 0.690 |
| walker |  | 1444 | 19 | python decl at xlstm/xlstm_lm_model.py:14 |  |  | 0.693 |
| walker |  | 1460 | 16 | python class body at xlstm/xlstm_lm_model.py:22 |  |  | 0.693 |
| walker |  | 1474 | 14 | python decl names surface in xlstm/xlstm_large/from_pretrained.py |  |  | 0.693 |
| walker |  | 1514 | 40 | python decl names surface in experiments/lr_scheduler.py |  |  | 0.693 |
| walker |  | 1514 | 0 | python decl at experiments/lr_scheduler.py:9 |  |  | 0.693 |
| walker |  | 1514 | 0 | python decl at experiments/lr_scheduler.py:24 |  |  | 0.693 |
| ns | 1640 |  | 390 | blocks/mlstm/{block,layer,cell}.py - class/def locations | 2.6 |  | 0.599 |
| walker |  | 1701 | 187 | [dependencies] in pyproject.toml |  |  | 0.599 |
| ns | 1722 |  | 82 | blocks/mlstm/backends.py - function locations | 2.7 |  | 0.586 |
| walker |  | 1764 | 63 | python method sigs in experiments/metrics.py |  |  | 0.586 |
| walker |  | 1764 | 0 | python method at experiments/metrics.py:13 |  |  | 0.586 |
| walker |  | 1764 | 0 | python method at experiments/metrics.py:17 |  |  | 0.586 |
| walker |  | 1764 | 0 | python method at experiments/metrics.py:22 |  |  | 0.586 |
| walker |  | 1764 | 0 | python method at experiments/metrics.py:25 |  |  | 0.586 |
| walker |  | 1773 | 9 | python method body at experiments/metrics.py:22 body 23 |  |  | 0.586 |
| walker |  | 1823 | 50 | python class body at xlstm/xlstm_lm_model.py:14 |  |  | 0.586 |
| walker |  | 1876 | 53 | listing of 'tests' |  |  | 0.587 |
| walker |  | 1941 | 65 | python decl names surface in experiments/main.py |  |  | 0.587 |
| walker |  | 1941 | 0 | python decl at experiments/main.py:32 |  |  | 0.587 |
| walker |  | 1941 | 0 | python decl at experiments/main.py:37 |  |  | 0.587 |
| walker |  | 1958 | 17 | python decl at experiments/main.py:21 |  |  | 0.587 |
| walker |  | 2002 | 44 | python decl at experiments/main.py:25 |  |  | 0.587 |
| ns | 2004 |  | 282 | blocks/slstm/{block,layer}.py - class/def locations | 2.8 |  | 0.536 |
| walker |  | 2030 | 28 | python decl body at experiments/main.py:32 body 33 |  |  | 0.536 |
| walker |  | 2234 | 204 | manifest config in pyproject.toml |  |  | 0.536 |
| walker |  | 2260 | 26 | python decl names surface in xlstm/blocks/xlstm_block.py |  |  | 0.536 |
| walker |  | 2260 | 0 | python decl at xlstm/blocks/xlstm_block.py:43 |  |  | 0.536 |
| walker |  | 2271 | 11 | python decl at xlstm/blocks/xlstm_block.py:16 |  |  | 0.538 |
| walker |  | 2287 | 16 | python class body at xlstm/blocks/xlstm_block.py:43 |  |  | 0.538 |
| walker |  | 2313 | 26 | python decl names surface in xlstm/components/linear_headwise.py |  |  | 0.538 |
| walker |  | 2313 | 0 | python decl at xlstm/components/linear_headwise.py:42 |  |  | 0.538 |
| ns | 2314 |  | 310 | blocks/slstm/cell.py - sLSTMCellConfig + sLSTMCellBase - class/def locations | 2.9 |  | 0.496 |
| walker |  | 2324 | 11 | python decl at xlstm/components/linear_headwise.py:12 |  |  | 0.496 |
| walker |  | 2340 | 16 | python class body at xlstm/components/linear_headwise.py:42 |  |  | 0.496 |
| walker |  | 2439 | 99 | python method sigs in xlstm/xlstm_lm_model.py |  |  | 0.512 |
| walker |  | 2439 | 0 | python method at xlstm/xlstm_lm_model.py:25 |  |  | 0.512 |
| walker |  | 2439 | 0 | python method at xlstm/xlstm_lm_model.py:41 |  |  | 0.512 |
| walker |  | 2439 | 0 | python method at xlstm/xlstm_lm_model.py:49 |  |  | 0.512 |
| walker |  | 2439 | 0 | python method at xlstm/xlstm_lm_model.py:65 |  |  | 0.512 |
| ns | 2568 |  | 254 | blocks/slstm/cell.py - CUDA/vanilla backend classes - class/def locations | 2.10 |  | 0.487 |
| walker |  | 2576 | 137 | python method sigs in xlstm/xlstm_block_stack.py |  |  | 0.513 |
| walker |  | 2576 | 0 | python method at xlstm/xlstm_block_stack.py:40 |  |  | 0.513 |
| walker |  | 2576 | 0 | python method at xlstm/xlstm_block_stack.py:52 |  |  | 0.513 |
| walker |  | 2576 | 0 | python method at xlstm/xlstm_block_stack.py:80 |  |  | 0.513 |
| walker |  | 2576 | 0 | python method at xlstm/xlstm_block_stack.py:90 |  |  | 0.513 |
| walker |  | 2576 | 0 | python method at xlstm/xlstm_block_stack.py:111 |  |  | 0.513 |
| walker |  | 2576 | 0 | python method at xlstm/xlstm_block_stack.py:117 |  |  | 0.513 |
| walker |  | 2584 | 8 | python method at xlstm/xlstm_block_stack.py:36 |  |  | 0.513 |
| walker |  | 2595 | 11 | python decl names surface in experiments/data/formal_language/tasks/parity.py |  |  | 0.513 |
| walker |  | 2651 | 56 | python method at xlstm/xlstm_block_stack.py:126 |  |  | 0.541 |
| walker |  | 2680 | 29 | python decl names surface in xlstm/components/ln.py |  |  | 0.541 |
| walker |  | 2680 | 0 | python decl at xlstm/components/ln.py:8 |  |  | 0.541 |
| walker |  | 2680 | 0 | python decl at xlstm/components/ln.py:51 |  |  | 0.541 |
| walker |  | 2702 | 22 | python decl doc at xlstm/components/ln.py:8 |  |  | 0.541 |
| ns | 2842 |  | 274 | components/{conv,feedforward,linear_headwise}.py - class/def locations | 2.11 |  | 0.511 |
| walker |  | 2876 | 174 | python decl at xlstm/blocks/slstm/src/vanilla/__init__.py:77 |  |  | 0.511 |
| ns | 2976 |  | 134 | components/{ln,init,util}.py - class/def locations | 2.12 |  | 0.500 |
| walker |  | 3052 | 176 | python decl at xlstm/blocks/slstm/src/vanilla/__init__.py:17 |  |  | 0.500 |
| ns | 3222 |  | 246 | xlstm_large/{model,components}.py - class/def locations | 2.13 |  | 0.477 |
| ns | 3342 |  | 120 | xlstm_large/{generate,from_pretrained,utils}.py - class/def locations + __init__.py | 2.14 |  | 0.473 |
| walker |  | 3568 | 516 | README.md section #0 |  |  | 0.492 |
| walker |  | 3580 | 12 | python decl names surface in experiments/data/formal_language/tasks/even_pairs.py |  |  | 0.492 |
| ns | 3653 |  | 311 | README - xLSTM Large quickstart code | 3.1 |  | 0.467 |
| walker |  | 3669 | 89 | python class body at xlstm/utils.py:11 |  |  | 0.467 |
| walker |  | 3728 | 59 | python method at xlstm/xlstm_lm_model.py:56 |  |  | 0.485 |
| walker |  | 3856 | 128 | python method sigs in experiments/lr_scheduler.py |  |  | 0.485 |
| walker |  | 3856 | 0 | python method at experiments/lr_scheduler.py:10 |  |  | 0.485 |
| walker |  | 3856 | 0 | python method at experiments/lr_scheduler.py:25 |  |  | 0.485 |
| walker |  | 3856 | 0 | python method at experiments/lr_scheduler.py:47 |  |  | 0.485 |
| walker |  | 3864 | 8 | python method at experiments/lr_scheduler.py:33 |  |  | 0.485 |
| walker |  | 3873 | 9 | python method at experiments/lr_scheduler.py:13 |  |  | 0.485 |
| walker |  | 3882 | 9 | python method at experiments/lr_scheduler.py:18 |  |  | 0.485 |
| walker |  | 3896 | 14 | python method doc at experiments/lr_scheduler.py:18 |  |  | 0.485 |
| walker |  | 3912 | 16 | python method doc at experiments/lr_scheduler.py:13 |  |  | 0.485 |
| walker |  | 3928 | 16 | python method doc at experiments/lr_scheduler.py:47 |  |  | 0.485 |
| ns | 3974 |  | 321 | README - non-CUDA hardware recommendation + sLSTM CUDA build env var | 3.2 |  | 0.474 |
| walker |  | 4178 | 250 | python class body at xlstm/xlstm_block_stack.py:15 |  |  | 0.474 |
| walker |  | 4194 | 16 | python imports in xlstm/xlstm_large/utils.py |  |  | 0.474 |
| walker |  | 4250 | 56 | listing of 'xlstm/blocks/slstm/src/cuda' |  |  | 0.474 |
| walker |  | 4262 | 12 | c decl names surface in xlstm/blocks/slstm/src/cuda/slstm.h |  |  | 0.474 |
| walker |  | 4262 | 0 | c decl at xlstm/blocks/slstm/src/cuda/slstm.h:44 |  |  | 0.474 |
| walker |  | 4275 | 13 | c includes in xlstm/blocks/slstm/src/cuda/slstm.h |  |  | 0.474 |
| ns | 4420 |  | 446 | README - xLSTMBlockStack usage code (NeurIPS architecture) | 3.3 |  | 0.439 |
| walker |  | 4528 | 253 | c header banner in xlstm/blocks/slstm/src/cuda/slstm.h |  |  | 0.439 |
| walker |  | 4603 | 75 | python method sigs in xlstm/components/ln.py |  |  | 0.439 |
| walker |  | 4603 | 0 | python method at xlstm/components/ln.py:36 |  |  | 0.439 |
| walker |  | 4603 | 0 | python method at xlstm/components/ln.py:41 |  |  | 0.439 |
| walker |  | 4603 | 0 | python method at xlstm/components/ln.py:53 |  |  | 0.439 |
| walker |  | 4611 | 8 | python method at xlstm/components/ln.py:27 |  |  | 0.439 |
| walker |  | 4649 | 38 | python decl names surface in xlstm/xlstm_large/utils.py |  |  | 0.446 |
| walker |  | 4649 | 0 | python decl at xlstm/xlstm_large/utils.py:5 |  |  | 0.446 |
| walker |  | 4677 | 28 | python decl at xlstm/xlstm_large/utils.py:10 |  |  | 0.446 |
| walker |  | 4694 | 17 | python decl doc at xlstm/xlstm_large/utils.py:5 |  |  | 0.446 |
| ns | 4702 |  | 282 | README - xLSTMLMModel usage code + yaml/dacite config pattern | 3.4 |  | 0.432 |
| walker |  | 4716 | 22 | python decl body at xlstm/xlstm_large/utils.py:5 body 7 |  |  | 0.432 |
| walker |  | 4738 | 22 | python method body at experiments/metrics.py:25 body 26 |  |  | 0.432 |
| ns | 4775 |  | 73 | xLSTMBlockStackConfig.__post_init__ - slstm_at resolution + per-block config propagation | 4.1 | 2.2 | 0.428 |
| walker |  | 4886 | 148 | python method sigs in xlstm/utils.py |  |  | 0.450 |
| walker |  | 4886 | 0 | python method at xlstm/utils.py:20 |  |  | 0.450 |
| walker |  | 4886 | 0 | python method at xlstm/utils.py:33 |  |  | 0.450 |
| walker |  | 4886 | 0 | python method at xlstm/utils.py:36 |  |  | 0.450 |
| walker |  | 4886 | 0 | python method at xlstm/utils.py:61 |  |  | 0.450 |
| walker |  | 4886 | 0 | python method at xlstm/utils.py:77 |  |  | 0.450 |
| walker |  | 4896 | 10 | python method body at xlstm/utils.py:33 body 34 |  |  | 0.450 |
| walker |  | 4935 | 39 | python decl names surface in xlstm/components/conv.py |  |  | 0.452 |
| walker |  | 4935 | 0 | python decl at xlstm/components/conv.py:55 |  |  | 0.452 |
| walker |  | 4947 | 12 | python decl at xlstm/components/conv.py:12 |  |  | 0.453 |
| walker |  | 5010 | 63 | python decl at xlstm/components/conv.py:24 |  |  | 0.439 |
| ns | 5010 |  | 235 | xLSTMBlockStack._create_blocks - block_map-driven mLSTM/sLSTM dispatch | 4.2 | 2.2 | 0.439 |
| walker |  | 5018 | 8 | python method body at experiments/lr_scheduler.py:13 body 16 |  |  | 0.439 |
| walker |  | 5098 | 80 | python method sigs in xlstm/components/linear_headwise.py |  |  | 0.445 |
| walker |  | 5098 | 0 | python method at xlstm/components/linear_headwise.py:31 |  |  | 0.445 |
| walker |  | 5098 | 0 | python method at xlstm/components/linear_headwise.py:49 |  |  | 0.445 |
| walker |  | 5098 | 0 | python method at xlstm/components/linear_headwise.py:75 |  |  | 0.445 |
| walker |  | 5098 | 0 | python method at xlstm/components/linear_headwise.py:84 |  |  | 0.445 |
| walker |  | 5107 | 9 | python method at xlstm/components/linear_headwise.py:67 |  |  | 0.445 |
| walker |  | 5128 | 21 | python method doc at xlstm/xlstm_block_stack.py:40 |  |  | 0.445 |
| walker |  | 5170 | 42 | python decl names surface in xlstm/components/util.py |  |  | 0.450 |
| walker |  | 5170 | 0 | python decl at xlstm/components/util.py:7 |  |  | 0.450 |
| walker |  | 5170 | 0 | python decl at xlstm/components/util.py:11 |  |  | 0.450 |
| walker |  | 5170 | 0 | python decl at xlstm/components/util.py:26 |  |  | 0.450 |
| walker |  | 5194 | 24 | python decl doc at xlstm/components/util.py:11 |  |  | 0.450 |
| ns | 5196 |  | 186 | xLSTMLMModel.__init__ - embedding/lm_head/tie_weights | 4.3 | 2.3 | 0.441 |
| walker |  | 5212 | 18 | python decl body at xlstm/components/util.py:7 body 8 |  |  | 0.441 |
| walker |  | 5318 | 106 | python method sigs in xlstm/components/util.py |  |  | 0.448 |
| walker |  | 5318 | 0 | python method at xlstm/components/util.py:58 |  |  | 0.448 |
| walker |  | 5318 | 0 | python method at xlstm/components/util.py:73 |  |  | 0.448 |
| walker |  | 5326 | 8 | python method at xlstm/components/util.py:61 |  |  | 0.448 |
| walker |  | 5334 | 8 | python method at xlstm/components/util.py:65 |  |  | 0.448 |
| ns | 5339 |  | 143 | xLSTMLMModel.forward + step | 4.4 | 2.3 | 0.442 |
| walker |  | 5342 | 8 | python method at xlstm/components/util.py:69 |  |  | 0.442 |
| walker |  | 5359 | 17 | python method at xlstm/components/util.py:46 |  |  | 0.442 |
| walker |  | 5377 | 18 | python method at xlstm/components/util.py:51 |  |  | 0.442 |
| walker |  | 5403 | 26 | python decl names surface in xlstm/blocks/mlstm/cell.py |  |  | 0.442 |
| walker |  | 5403 | 0 | python decl at xlstm/blocks/mlstm/cell.py:20 |  |  | 0.442 |
| walker |  | 5414 | 11 | python decl at xlstm/blocks/mlstm/cell.py:13 |  |  | 0.442 |
| walker |  | 5428 | 14 | python class body at xlstm/blocks/mlstm/cell.py:20 |  |  | 0.442 |
| walker |  | 5465 | 37 | python class body at xlstm/blocks/mlstm/cell.py:13 |  |  | 0.442 |
| walker |  | 5552 | 87 | python method sigs in xlstm/components/conv.py |  |  | 0.454 |
| walker |  | 5552 | 0 | python method at xlstm/components/conv.py:20 |  |  | 0.454 |
| walker |  | 5552 | 0 | python method at xlstm/components/conv.py:71 |  |  | 0.454 |
| walker |  | 5552 | 0 | python method at xlstm/components/conv.py:95 |  |  | 0.454 |
| ns | 5555 |  | 216 | xLSTMLMModel._create_weight_decay_optim_groups - embedding weight-decay override | 4.5 | 2.3 | 0.445 |
| walker |  | 5560 | 8 | python method body at xlstm/components/conv.py:95 body 96 |  |  | 0.445 |
| walker |  | 5629 | 69 | python class body at xlstm/components/conv.py:12 |  |  | 0.445 |
| walker |  | 5656 | 27 | python decl names surface in xlstm/blocks/mlstm/layer.py |  |  | 0.445 |
| walker |  | 5656 | 0 | python decl at xlstm/blocks/mlstm/layer.py:39 |  |  | 0.445 |
| walker |  | 5673 | 17 | python decl at xlstm/blocks/mlstm/layer.py:18 |  |  | 0.446 |
| walker |  | 5688 | 15 | python class body at xlstm/blocks/mlstm/layer.py:39 |  |  | 0.446 |
| ns | 5702 |  | 147 | UpProjConfigMixin._set_proj_up_dim - proj-up dimension rounding rule | 4.6 | 2.4 | 0.440 |
| walker |  | 5715 | 27 | python decl names surface in xlstm/blocks/slstm/layer.py |  |  | 0.440 |
| walker |  | 5715 | 0 | python decl at xlstm/blocks/slstm/layer.py:33 |  |  | 0.440 |
| walker |  | 5732 | 17 | python decl at xlstm/blocks/slstm/layer.py:18 |  |  | 0.441 |
| walker |  | 5747 | 15 | python class body at xlstm/blocks/slstm/layer.py:33 |  |  | 0.441 |
| walker |  | 5800 | 53 | python decl doc at xlstm/blocks/xlstm_block.py:43 |  |  | 0.450 |
| walker |  | 5853 | 53 | python decl doc at xlstm/components/linear_headwise.py:42 |  |  | 0.450 |
| walker |  | 5881 | 28 | python decl names surface in xlstm/blocks/mlstm/block.py |  |  | 0.450 |
| walker |  | 5881 | 0 | python decl at xlstm/blocks/mlstm/block.py:22 |  |  | 0.450 |
| walker |  | 5892 | 11 | python decl at xlstm/blocks/mlstm/block.py:9 |  |  | 0.451 |
| walker |  | 5906 | 14 | python class body at xlstm/blocks/mlstm/block.py:22 |  |  | 0.451 |
| walker |  | 5942 | 36 | python method sigs in xlstm/blocks/mlstm/block.py |  |  | 0.453 |
| walker |  | 5942 | 0 | python method at xlstm/blocks/mlstm/block.py:17 |  |  | 0.453 |
| walker |  | 5942 | 0 | python method at xlstm/blocks/mlstm/block.py:25 |  |  | 0.453 |
| ns | 5968 |  | 266 | WeightDecayOptimGroupMixin._create_weight_decay_optim_groups - default decay/no-decay heuristic | 4.7 | 2.4 | 0.442 |
| walker |  | 5970 | 28 | python decl names surface in xlstm/blocks/slstm/block.py |  |  | 0.443 |
| walker |  | 5970 | 0 | python decl at xlstm/blocks/slstm/block.py:29 |  |  | 0.443 |
| walker |  | 5981 | 11 | python decl at xlstm/blocks/slstm/block.py:12 |  |  | 0.444 |
| walker |  | 5995 | 14 | python class body at xlstm/blocks/slstm/block.py:29 |  |  | 0.444 |
| walker |  | 6028 | 33 | python method sigs in xlstm/blocks/slstm/block.py |  |  | 0.445 |
| walker |  | 6028 | 0 | python method at xlstm/blocks/slstm/block.py:21 |  |  | 0.445 |
| walker |  | 6028 | 0 | python method at xlstm/blocks/slstm/block.py:32 |  |  | 0.445 |
| ns | 6030 |  | 62 | xLSTMBlockConfig.__post_init__ - mutual-exclusion invariant | 5.1 | 2.5 | 0.444 |
| walker |  | 6078 | 50 | python method at xlstm/components/conv.py:131 |  |  | 0.444 |
| ns | 6106 |  | 76 | xLSTMBlock.forward - pre-norm residual pattern | 5.2 | 2.5 | 0.441 |
| walker |  | 6132 | 54 | python method doc at xlstm/utils.py:36 |  |  | 0.441 |
| walker |  | 6140 | 8 | python method body at experiments/lr_scheduler.py:18 body 21 |  |  | 0.441 |
| walker |  | 6196 | 56 | python method doc at xlstm/utils.py:61 |  |  | 0.441 |
| ns | 6207 |  | 101 | mLSTMLayer.__init__ - q/k/v projected via headwise num_proj_heads | 5.3 | 2.6 | 0.437 |
| walker |  | 6278 | 82 | listing of 'xlstm/blocks/slstm/src/util' |  |  | 0.438 |
| ns | 6305 |  | 98 | mLSTMCell.forward - backend_fn dispatch call | 5.4 | 2.6 | 0.434 |
| walker |  | 6315 | 37 | c whole header in xlstm/blocks/slstm/src/util/util.h |  |  | 0.434 |
| walker |  | 6444 | 129 | c whole header in xlstm/blocks/slstm/src/util/cuda_error.h |  |  | 0.434 |
| ns | 6461 |  | 156 | mLSTMCell.step - S==1 assertion + zero-state shapes | 5.5 | 2.6 | 0.431 |
| walker |  | 6575 | 131 | c decl names surface in xlstm/blocks/slstm/src/util/support.h |  |  | 0.431 |
| walker |  | 6575 | 0 | c decl at xlstm/blocks/slstm/src/util/support.h:48 |  |  | 0.431 |
| walker |  | 6593 | 18 | c decl at xlstm/blocks/slstm/src/util/support.h:28 |  |  | 0.431 |
| walker |  | 6615 | 22 | c decl at xlstm/blocks/slstm/src/util/support.h:30 |  |  | 0.431 |
| walker |  | 6638 | 23 | c decl at xlstm/blocks/slstm/src/util/support.h:26 |  |  | 0.431 |
| walker |  | 6649 | 11 | c decl body at xlstm/blocks/slstm/src/util/support.h:48 |  |  | 0.431 |
| walker |  | 6683 | 34 | c decl at xlstm/blocks/slstm/src/util/support.h:40 |  |  | 0.431 |
| walker |  | 6709 | 26 | c includes in xlstm/blocks/slstm/src/util/support.h |  |  | 0.431 |
| ns | 6741 |  | 280 | parallel_stabilized_simple - stabilized log-D-matrix core equation | 5.6 | 2.7 | 0.425 |
| ns | 6948 |  | 207 | recurrent_step_stabilized_simple - (c,n,m) update rule | 5.7 | 2.7 | 0.421 |
| ns | 7046 |  | 98 | sLSTMBlockConfig + sLSTMLayerConfig.__post_init__ - propagation rules | 6.1 | 2.8 | 0.417 |
| walker |  | 7072 | 363 | c whole header in xlstm/blocks/slstm/src/util/device_assert.h |  |  | 0.417 |
| walker |  | 7158 | 86 | c decl at xlstm/blocks/slstm/src/util/support.h:34 |  |  | 0.417 |
| ns | 7267 |  | 221 | sLSTMCellConfig - backend/function/bias_init/dtype knobs | 6.2 | 2.9 | 0.411 |
| ns | 7435 |  | 168 | sLSTMCellBase.reset_parameters - powerlaw_blockdependent forget-gate init | 6.3 | 2.9 | 0.405 |
| ns | 7519 |  | 84 | sLSTMCellBase.step/forward - dispatch to backend-specific _impl/_impl_step | 6.4 | 2.9 | 0.401 |
| walker |  | 7565 | 407 | c decl names surface in xlstm/blocks/slstm/src/util/blas.h |  |  | 0.401 |
| walker |  | 7565 | 0 | c decl at xlstm/blocks/slstm/src/util/blas.h:108 |  |  | 0.401 |
| walker |  | 7565 | 0 | c decl at xlstm/blocks/slstm/src/util/blas.h:115 |  |  | 0.401 |
| walker |  | 7565 | 0 | c decl at xlstm/blocks/slstm/src/util/blas.h:122 |  |  | 0.401 |
| walker |  | 7624 | 59 | c decl at xlstm/blocks/slstm/src/util/blas.h:75 |  |  | 0.401 |
| ns | 7643 |  | 124 | sLSTMCellCUDA JIT cache key + sLSTMCell.__new__ backend factory | 6.5 | 2.10 | 0.397 |
| walker |  | 7683 | 59 | c decl at xlstm/blocks/slstm/src/util/blas.h:80 |  |  | 0.397 |
| ns | 7729 |  | 86 | vanilla pointwise gate equations - slstm vs plain lstm variant | 6.6 |  | 0.395 |
| walker |  | 7742 | 59 | c decl at xlstm/blocks/slstm/src/util/blas.h:85 |  |  | 0.395 |
| walker |  | 7781 | 39 | c includes in xlstm/blocks/slstm/src/util/blas.h |  |  | 0.395 |
| ns | 7818 |  | 89 | cuda_init.py - load() JIT entry point + XLSTM_EXTRA_INCLUDE_PATHS env var | 6.7 |  | 0.393 |
| walker |  | 7873 | 92 | c decl at xlstm/blocks/slstm/src/util/blas.h:39 |  |  | 0.393 |
| ns | 7956 |  | 138 | sLSTM CUDA/C++ source tree listing | 6.8 |  | 0.418 |
| walker |  | 7964 | 91 | c decl at xlstm/blocks/slstm/src/util/blas.h:90 |  |  | 0.418 |
| ns | 8021 |  | 65 | slstm.h - ForwardPass/BackwardPass/BackwardPassCut class + Run() locations | 6.9 |  | 0.416 |
| walker |  | 8055 | 91 | c decl at xlstm/blocks/slstm/src/util/blas.h:97 |  |  | 0.416 |
| ns | 8118 |  | 97 | CausalConv1d.step - rolling-buffer single-step convolution | 7.1 | 2.11 | 0.414 |
| walker |  | 8151 | 96 | c decl at xlstm/blocks/slstm/src/util/blas.h:60 |  |  | 0.414 |
| ns | 8232 |  | 114 | LinearHeadwiseExpand.forward - per-head einsum projection | 7.2 | 2.11 | 0.411 |
| walker |  | 8252 | 101 | c decl at xlstm/blocks/slstm/src/util/blas.h:67 |  |  | 0.411 |
| ns | 8270 |  | 38 | init.py - small_init_ / wang_init_ std-dev formulas | 7.3 | 2.12 | 0.411 |
| walker |  | 8358 | 106 | c decl at xlstm/blocks/slstm/src/util/blas.h:52 |  |  | 0.411 |
| ns | 8415 |  | 145 | xLSTMLargeConfig - chunkwise_kernel/mode/weight_mode doc'd fields | 8.1 | 2.13 | 0.408 |
| ns | 8532 |  | 117 | xLSTMLarge.forward + mLSTMLayer.forward - soft_cap gate/logit capping | 8.2 | 2.13 | 0.405 |
| ns | 8568 |  | 36 | components.py - soft_cap formula | 8.3 | 2.13 | 0.404 |
| walker |  | 8606 | 248 | c header banner in xlstm/blocks/slstm/src/util/blas.h |  |  | 0.404 |
| ns | 8826 |  | 258 | from_pretrained.py - sharded safetensors loading + forced single weight_mode | 8.4 | 2.14 | 0.397 |
| walker |  | 8854 | 248 | c header banner in xlstm/blocks/slstm/src/util/support.h |  |  | 0.397 |
| walker |  | 8910 | 56 | python method at xlstm/components/conv.py:110 |  |  | 0.397 |
| ns | 8936 |  | 110 | experiments/ + data/formal_language/{,tasks}/ directory listings | 9.1 |  | 0.419 |
| walker |  | 8964 | 54 | python decl names surface in experiments/data/utils.py |  |  | 0.419 |
| walker |  | 8964 | 0 | python decl at experiments/data/utils.py:17 |  |  | 0.419 |
| walker |  | 8964 | 0 | python decl at experiments/data/utils.py:39 |  |  | 0.419 |
| walker |  | 8964 | 0 | python decl at experiments/data/utils.py:55 |  |  | 0.419 |
| walker |  | 8964 | 0 | python decl at experiments/data/utils.py:90 |  |  | 0.419 |
| ns | 9029 |  | 93 | main.py - CLI entry point + dataset registry | 9.2 |  | 0.418 |
| walker |  | 9074 | 110 | python method sigs in xlstm/blocks/xlstm_block.py |  |  | 0.430 |
| walker |  | 9074 | 0 | python method at xlstm/blocks/xlstm_block.py:27 |  |  | 0.430 |
| walker |  | 9074 | 0 | python method at xlstm/blocks/xlstm_block.py:51 |  |  | 0.430 |
| walker |  | 9074 | 0 | python method at xlstm/blocks/xlstm_block.py:76 |  |  | 0.430 |
| walker |  | 9074 | 0 | python method at xlstm/blocks/xlstm_block.py:82 |  |  | 0.430 |
| walker |  | 9074 | 0 | python method at xlstm/blocks/xlstm_block.py:89 |  |  | 0.430 |
| walker |  | 9113 | 39 | python method at xlstm/utils.py:97 |  |  | 0.430 |
| walker |  | 9128 | 15 | python decl names surface in xlstm/blocks/slstm/src/vanilla/lstm.py |  |  | 0.430 |
| walker |  | 9143 | 15 | python decl names surface in xlstm/blocks/slstm/src/vanilla/slstm.py |  |  | 0.430 |
| ns | 9209 |  | 180 | lr_scheduler.py - LinearWarmupCosineAnnealing.compute_lr formula | 9.3 |  | 0.426 |
| walker |  | 9330 | 187 | python method sigs in experiments/data/utils.py |  |  | 0.426 |
| walker |  | 9330 | 0 | python method at experiments/data/utils.py:41 |  |  | 0.426 |
| ns | 9331 |  | 122 | FormLangDataset.__getitem__ - mask-based causal LM framing | 9.4 |  | 0.423 |
| walker |  | 9336 | 6 | python method at experiments/data/utils.py:57 |  |  | 0.423 |
| walker |  | 9344 | 8 | python method at experiments/data/utils.py:46 |  |  | 0.423 |
| walker |  | 9352 | 8 | python method at experiments/data/utils.py:50 |  |  | 0.423 |
| walker |  | 9360 | 8 | python method at experiments/data/utils.py:77 |  |  | 0.423 |
| walker |  | 9368 | 8 | python method at experiments/data/utils.py:85 |  |  | 0.423 |
| ns | 9379 |  | 48 | tasks/{parity,cycle_navigation,even_pairs,modular_arithmetic}.py - function locations | 9.5 |  | 0.423 |
| walker |  | 9383 | 15 | python method at experiments/data/utils.py:19 |  |  | 0.423 |
| walker |  | 9400 | 17 | python method at experiments/data/utils.py:24 |  |  | 0.423 |
| walker |  | 9417 | 17 | python method at experiments/data/utils.py:29 |  |  | 0.423 |
| walker |  | 9434 | 17 | python method at experiments/data/utils.py:34 |  |  | 0.423 |
| walker |  | 9500 | 66 | python method sigs in xlstm/blocks/slstm/layer.py |  |  | 0.428 |
| walker |  | 9500 | 0 | python method at xlstm/blocks/slstm/layer.py:28 |  |  | 0.428 |
| walker |  | 9500 | 0 | python method at xlstm/blocks/slstm/layer.py:36 |  |  | 0.428 |
| walker |  | 9500 | 0 | python method at xlstm/blocks/slstm/layer.py:84 |  |  | 0.428 |
| walker |  | 9523 | 23 | python decl names surface in experiments/data/formal_language/tasks/cycle_navigation.py |  |  | 0.429 |
| walker |  | 9549 | 26 | python decl at experiments/data/formal_language/tasks/cycle_navigation.py:51 |  |  | 0.429 |
| ns | 9553 |  | 174 | parity_xlstm{01,10,11}.yaml - the mLSTM-only / sLSTM-only / mixed variants | 9.6 |  | 0.424 |
| walker |  | 9572 | 23 | python method body at experiments/metrics.py:13 body 14 |  |  | 0.424 |
| ns | 9606 |  | 53 | tests/ directory listing | 10.1 |  | 0.430 |
| walker |  | 9634 | 62 | python method at xlstm/components/util.py:34 |  |  | 0.430 |
| ns | 9653 |  | 47 | conftest.py - CUDA-required skip-all guard | 10.2 |  | 0.429 |
| walker |  | 9708 | 74 | python method sigs in xlstm/blocks/mlstm/cell.py |  |  | 0.432 |
| walker |  | 9708 | 0 | python method at xlstm/blocks/mlstm/cell.py:23 |  |  | 0.432 |
| walker |  | 9708 | 0 | python method at xlstm/blocks/mlstm/cell.py:43 |  |  | 0.432 |
| walker |  | 9708 | 0 | python method at xlstm/blocks/mlstm/cell.py:133 |  |  | 0.432 |
| walker |  | 9734 | 26 | python method at xlstm/components/conv.py:98 |  |  | 0.432 |
| ns | 9788 |  | 135 | Test-function locations across the pytest suite | 10.3 |  | 0.429 |
| ns | 9864 |  | 76 | res/ + notebooks/ directory listings | 11.1 |  | 0.435 |
| ns | 9872 |  | 8 | .github/workflows/ - no test/lint CI, only CLA + repo-sync | 11.2 |  | 0.437 |
| ns | 9887 |  | 15 | LICENSE header | 11.3 |  | 0.436 |
| walker |  | 9947 | 213 | python class body at xlstm/components/conv.py:55 |  |  | 0.436 |
| ns | 9961 |  | 74 | pytest.ini - full | 11.4 |  | 0.434 |
| ns | 9996 |  | 35 | README - Citation BibTeX keys | 11.5 |  | 0.434 |
