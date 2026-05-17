Score(3000)=0.600 I=0.805 C=0.448 ns_rows≤3K=22/48 (reached=11 partial=1 missing=10)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 63 | 63 | listing of '.' |  |  | 1.000 |
| ns | 63 |  | 63 | Repository root listing | 1.1 |  | 1.000 |
| walker |  | 78 | 15 | README headline in README.md |  |  | 1.000 |
| ns | 78 |  | 15 | README H1 — project name | 1.2 |  | 1.000 |
| walker |  | 87 | 9 | listing of 'notebooks' |  |  | 1.000 |
| ns | 114 |  | 36 | xlstm/ package directory listing | 1.3 |  | 0.812 |
| walker |  | 123 | 36 | listing of 'xlstm' |  |  | 1.000 |
| walker |  | 143 | 20 | listing of 'xlstm/blocks' |  |  | 1.000 |
| walker |  | 164 | 21 | listing of 'xlstm/blocks/slstm' |  |  | 1.000 |
| walker |  | 184 | 20 | listing of 'xlstm/blocks/slstm/src' |  |  | 1.000 |
| walker |  | 200 | 16 | listing of 'xlstm/blocks/slstm/src/vanilla' |  |  | 1.000 |
| ns | 213 |  | 99 | Top-level __init__ — re-export names only | 1.4 |  | 0.853 |
| walker |  | 223 | 23 | listing of 'xlstm/blocks/mlstm' |  |  | 0.863 |
| walker |  | 251 | 28 | listing of 'xlstm/xlstm_large' |  |  | 0.870 |
| walker |  | 293 | 42 | python imports in xlstm/xlstm_large/__init__.py |  |  | 0.870 |
| walker |  | 326 | 33 | listing of 'xlstm/components' |  |  | 0.870 |
| ns | 369 |  | 156 | README — About paragraph | 1.5 |  | 0.760 |
| walker |  | 372 | 46 | listing of 'experiments' |  |  | 0.761 |
| walker |  | 386 | 14 | listing of 'experiments/data' |  |  | 0.762 |
| walker |  | 410 | 24 | listing of 'experiments/data/formal_language' |  |  | 0.765 |
| walker |  | 436 | 26 | listing of 'experiments/data/formal_language/tasks' |  |  | 0.768 |
| ns | 447 |  | 78 | Top-level __init__ — full content (refines 1.4) | 1.6 | 1.4 | 0.699 |
| walker |  | 448 | 12 | python decl names surface in experiments/metrics.py |  |  | 0.699 |
| walker |  | 448 | 0 | python decl at experiments/metrics.py:9 |  |  | 0.699 |
| walker |  | 489 | 41 | python decl names surface in xlstm/blocks/slstm/src/vanilla/__init__.py |  |  | 0.699 |
| walker |  | 583 | 94 | python imports in xlstm/blocks/slstm/src/vanilla/__init__.py |  |  | 0.699 |
| walker |  | 611 | 28 | listing of 'res' |  |  | 0.658 |
| ns | 611 |  | 164 | README — xLSTM Large signpost | 1.7 |  | 0.658 |
| walker |  | 650 | 39 | python decl at xlstm/blocks/slstm/src/vanilla/__init__.py:11 |  |  | 0.659 |
| ns | 727 |  | 116 | README — NeurIPS-paper-models signpost | 1.8 |  | 0.612 |
| ns | 747 |  | 20 | xlstm/blocks/ directory listing | 2.1 |  | 0.634 |
| ns | 780 |  | 33 | xlstm/components/ directory listing | 2.2 |  | 0.664 |
| ns | 808 |  | 28 | xlstm/xlstm_large/ directory listing | 2.3 |  | 0.684 |
| walker |  | 827 | 177 | python imports in xlstm/__init__.py |  |  | 0.835 |
| walker |  | 852 | 25 | python decl names surface in xlstm/xlstm_block_stack.py |  |  | 0.842 |
| walker |  | 852 | 0 | python decl at xlstm/xlstm_block_stack.py:77 |  |  | 0.842 |
| ns | 852 |  | 44 | xlstm/blocks/mlstm/ + slstm/ listings | 2.4 |  | 0.842 |
| walker |  | 862 | 10 | python decl at xlstm/xlstm_block_stack.py:15 |  |  | 0.842 |
| walker |  | 875 | 13 | python class body at xlstm/xlstm_block_stack.py:77 |  |  | 0.842 |
| walker |  | 902 | 27 | python decl names surface in xlstm/utils.py |  |  | 0.842 |
| walker |  | 902 | 0 | python decl at xlstm/utils.py:32 |  |  | 0.842 |
| walker |  | 910 | 8 | python decl at xlstm/utils.py:11 |  |  | 0.842 |
| walker |  | 922 | 12 | python decl names surface in xlstm/xlstm_large/from_pretrained.py |  |  | 0.842 |
| walker |  | 955 | 33 | python class body at experiments/metrics.py:9 |  |  | 0.842 |
| walker |  | 988 | 33 | python decl names surface in xlstm/xlstm_lm_model.py |  |  | 0.843 |
| walker |  | 988 | 0 | python decl at xlstm/xlstm_lm_model.py:22 |  |  | 0.843 |
| walker |  | 1005 | 17 | python decl at xlstm/xlstm_lm_model.py:14 |  |  | 0.843 |
| walker |  | 1019 | 14 | python class body at xlstm/xlstm_lm_model.py:22 |  |  | 0.844 |
| walker |  | 1096 | 77 | [package] in pyproject.toml |  |  | 0.844 |
| ns | 1124 |  | 272 | xLSTMBlockStackConfig field list | 2.5 |  | 0.748 |
| ns | 1250 |  | 126 | xLSTMLMModelConfig — LM-specific fields | 2.6 |  | 0.720 |
| walker |  | 1284 | 188 | headings outline in README.md |  |  | 0.728 |
| walker |  | 1322 | 38 | python decl names surface in experiments/lr_scheduler.py |  |  | 0.728 |
| walker |  | 1322 | 0 | python decl at experiments/lr_scheduler.py:9 |  |  | 0.728 |
| walker |  | 1322 | 0 | python decl at experiments/lr_scheduler.py:24 |  |  | 0.728 |
| walker |  | 1374 | 52 | python class body at xlstm/xlstm_lm_model.py:14 |  |  | 0.762 |
| ns | 1409 |  | 159 | xLSTMLargeConfig — required + key fields | 2.7 |  | 0.720 |
| walker |  | 1559 | 185 | [dependencies] in pyproject.toml |  |  | 0.720 |
| ns | 1583 |  | 174 | xlstm/blocks/slstm/src/ directory listings | 2.8 |  | 0.648 |
| walker |  | 1624 | 65 | python method sigs in experiments/metrics.py |  |  | 0.648 |
| walker |  | 1624 | 0 | python method at experiments/metrics.py:13 |  |  | 0.648 |
| walker |  | 1624 | 0 | python method at experiments/metrics.py:17 |  |  | 0.648 |
| walker |  | 1624 | 0 | python method at experiments/metrics.py:22 |  |  | 0.648 |
| walker |  | 1624 | 0 | python method at experiments/metrics.py:25 |  |  | 0.648 |
| walker |  | 1633 | 9 | python method body at experiments/metrics.py:22 body 23 |  |  | 0.648 |
| walker |  | 1642 | 9 | python decl names surface in experiments/data/formal_language/tasks/parity.py |  |  | 0.648 |
| walker |  | 1652 | 10 | python decl names surface in experiments/data/formal_language/tasks/even_pairs.py |  |  | 0.648 |
| walker |  | 1711 | 59 | python decl names surface in experiments/main.py |  |  | 0.648 |
| walker |  | 1711 | 0 | python decl at experiments/main.py:32 |  |  | 0.648 |
| walker |  | 1711 | 0 | python decl at experiments/main.py:37 |  |  | 0.648 |
| walker |  | 1730 | 19 | python decl at experiments/main.py:21 |  |  | 0.648 |
| walker |  | 1776 | 46 | python decl at experiments/main.py:25 |  |  | 0.648 |
| ns | 1803 |  | 220 | experiments/ + tests/ + notebooks/ listings | 2.9 |  | 0.630 |
| walker |  | 1804 | 28 | python decl body at experiments/main.py:32 body 33 |  |  | 0.630 |
| walker |  | 1828 | 24 | python decl names surface in xlstm/blocks/xlstm_block.py |  |  | 0.630 |
| walker |  | 1828 | 0 | python decl at xlstm/blocks/xlstm_block.py:43 |  |  | 0.630 |
| walker |  | 1837 | 9 | python decl at xlstm/blocks/xlstm_block.py:16 |  |  | 0.630 |
| walker |  | 1851 | 14 | python class body at xlstm/blocks/xlstm_block.py:43 |  |  | 0.630 |
| walker |  | 1875 | 24 | python decl names surface in xlstm/components/linear_headwise.py |  |  | 0.630 |
| walker |  | 1875 | 0 | python decl at xlstm/components/linear_headwise.py:42 |  |  | 0.630 |
| walker |  | 1884 | 9 | python decl at xlstm/components/linear_headwise.py:12 |  |  | 0.630 |
| walker |  | 1898 | 14 | python class body at xlstm/components/linear_headwise.py:42 |  |  | 0.630 |
| walker |  | 1951 | 53 | listing of 'tests' |  |  | 0.669 |
| ns | 1954 |  | 151 | xLSTMBlockStack — class headers + fn signatures | 3.1 |  | 0.653 |
| walker |  | 1978 | 27 | python decl names surface in xlstm/components/ln.py |  |  | 0.653 |
| walker |  | 1978 | 0 | python decl at xlstm/components/ln.py:8 |  |  | 0.653 |
| walker |  | 1978 | 0 | python decl at xlstm/components/ln.py:51 |  |  | 0.653 |
| walker |  | 1998 | 20 | python decl doc at xlstm/components/ln.py:8 |  |  | 0.653 |
| walker |  | 2082 | 84 | python class body at xlstm/utils.py:11 |  |  | 0.654 |
| ns | 2104 |  | 150 | xLSTMLMModel — class header + fn signatures | 3.2 |  | 0.641 |
| walker |  | 2181 | 99 | python method sigs in experiments/lr_scheduler.py |  |  | 0.641 |
| walker |  | 2181 | 0 | python method at experiments/lr_scheduler.py:10 |  |  | 0.641 |
| walker |  | 2181 | 0 | python method at experiments/lr_scheduler.py:25 |  |  | 0.641 |
| walker |  | 2181 | 0 | python method at experiments/lr_scheduler.py:47 |  |  | 0.641 |
| walker |  | 2194 | 13 | python method at experiments/lr_scheduler.py:13 |  |  | 0.641 |
| walker |  | 2208 | 14 | python method at experiments/lr_scheduler.py:18 |  |  | 0.641 |
| walker |  | 2230 | 22 | python method at experiments/lr_scheduler.py:33 |  |  | 0.641 |
| walker |  | 2244 | 14 | python method doc at experiments/lr_scheduler.py:18 |  |  | 0.641 |
| walker |  | 2258 | 14 | python method doc at experiments/lr_scheduler.py:47 |  |  | 0.641 |
| walker |  | 2268 | 10 | python method body at experiments/lr_scheduler.py:13 body 16 |  |  | 0.641 |
| walker |  | 2278 | 10 | python method body at experiments/lr_scheduler.py:18 body 21 |  |  | 0.641 |
| walker |  | 2294 | 16 | python method doc at experiments/lr_scheduler.py:13 |  |  | 0.641 |
| walker |  | 2306 | 12 | python method body at experiments/lr_scheduler.py:10 body 11 |  |  | 0.641 |
| walker |  | 2320 | 14 | python imports in xlstm/xlstm_large/utils.py |  |  | 0.641 |
| ns | 2420 |  | 316 | xLSTMBlockConfig + xLSTMBlock — the shared parent block | 3.3 |  | 0.613 |
| walker |  | 2421 | 101 | python method sigs in xlstm/xlstm_lm_model.py |  |  | 0.623 |
| walker |  | 2421 | 0 | python method at xlstm/xlstm_lm_model.py:25 |  |  | 0.623 |
| walker |  | 2421 | 0 | python method at xlstm/xlstm_lm_model.py:41 |  |  | 0.623 |
| walker |  | 2421 | 0 | python method at xlstm/xlstm_lm_model.py:49 |  |  | 0.623 |
| walker |  | 2421 | 0 | python method at xlstm/xlstm_lm_model.py:65 |  |  | 0.623 |
| walker |  | 2478 | 57 | python method at xlstm/xlstm_lm_model.py:56 |  |  | 0.635 |
| walker |  | 2650 | 172 | python decl at xlstm/blocks/slstm/src/vanilla/__init__.py:77 |  |  | 0.635 |
| ns | 2673 |  | 253 | mLSTMBlock + sLSTMBlock — thin subclass shells | 3.4 |  | 0.606 |
| walker |  | 2824 | 174 | python decl at xlstm/blocks/slstm/src/vanilla/__init__.py:17 |  |  | 0.606 |
| walker |  | 2846 | 22 | python method body at experiments/metrics.py:25 body 26 |  |  | 0.606 |
| walker |  | 2879 | 33 | listing of 'notebooks/xlstm' |  |  | 0.623 |
| ns | 2889 |  | 216 | mLSTMLayerConfig — field declarations | 3.5 |  | 0.600 |
| walker |  | 2916 | 37 | python decl names surface in xlstm/components/conv.py |  |  | 0.600 |
| walker |  | 2916 | 0 | python decl at xlstm/components/conv.py:55 |  |  | 0.600 |
| walker |  | 2926 | 10 | python decl at xlstm/components/conv.py:12 |  |  | 0.600 |
| walker |  | 2987 | 61 | python decl at xlstm/components/conv.py:24 |  |  | 0.600 |
| walker |  | 3011 | 24 | python decl names surface in xlstm/blocks/mlstm/cell.py |  |  | 0.600 |
| walker |  | 3011 | 0 | python decl at xlstm/blocks/mlstm/cell.py:20 |  |  | 0.600 |
| walker |  | 3020 | 9 | python decl at xlstm/blocks/mlstm/cell.py:13 |  |  | 0.601 |
| walker |  | 3032 | 12 | python class body at xlstm/blocks/mlstm/cell.py:20 |  |  | 0.601 |
| ns | 3040 |  | 151 | mLSTMLayer — class header + fn signatures | 3.6 |  | 0.588 |
| walker |  | 3055 | 23 | python method body at experiments/metrics.py:13 body 14 |  |  | 0.588 |
| walker |  | 3128 | 73 | python method sigs in xlstm/components/ln.py |  |  | 0.588 |
| walker |  | 3128 | 0 | python method at xlstm/components/ln.py:36 |  |  | 0.588 |
| walker |  | 3128 | 0 | python method at xlstm/components/ln.py:41 |  |  | 0.588 |
| walker |  | 3128 | 0 | python method at xlstm/components/ln.py:53 |  |  | 0.588 |
| walker |  | 3140 | 12 | python method at xlstm/components/ln.py:27 |  |  | 0.588 |
| walker |  | 3188 | 48 | python decl doc at xlstm/blocks/xlstm_block.py:43 |  |  | 0.589 |
| ns | 3237 |  | 197 | mLSTMCell + mLSTMCellConfig | 3.7 |  | 0.572 |
| walker |  | 3321 | 133 | python method sigs in xlstm/xlstm_block_stack.py |  |  | 0.581 |
| walker |  | 3321 | 0 | python method at xlstm/xlstm_block_stack.py:40 |  |  | 0.581 |
| walker |  | 3321 | 0 | python method at xlstm/xlstm_block_stack.py:52 |  |  | 0.581 |
| walker |  | 3321 | 0 | python method at xlstm/xlstm_block_stack.py:80 |  |  | 0.581 |
| walker |  | 3321 | 0 | python method at xlstm/xlstm_block_stack.py:90 |  |  | 0.581 |
| walker |  | 3321 | 0 | python method at xlstm/xlstm_block_stack.py:111 |  |  | 0.581 |
| walker |  | 3321 | 0 | python method at xlstm/xlstm_block_stack.py:117 |  |  | 0.581 |
| walker |  | 3333 | 12 | python method at xlstm/xlstm_block_stack.py:36 |  |  | 0.581 |
| walker |  | 3350 | 17 | python method body at xlstm/xlstm_block_stack.py:36 body 38 |  |  | 0.581 |
| walker |  | 3404 | 54 | python method at xlstm/xlstm_block_stack.py:126 |  |  | 0.593 |
| walker |  | 3443 | 39 | python class body at xlstm/blocks/mlstm/cell.py:13 |  |  | 0.596 |
| walker |  | 3468 | 25 | python decl names surface in xlstm/blocks/mlstm/layer.py |  |  | 0.596 |
| walker |  | 3468 | 0 | python decl at xlstm/blocks/mlstm/layer.py:39 |  |  | 0.596 |
| walker |  | 3483 | 15 | python decl at xlstm/blocks/mlstm/layer.py:18 |  |  | 0.597 |
| walker |  | 3496 | 13 | python class body at xlstm/blocks/mlstm/layer.py:39 |  |  | 0.597 |
| walker |  | 3521 | 25 | python decl names surface in xlstm/blocks/slstm/layer.py |  |  | 0.597 |
| walker |  | 3521 | 0 | python decl at xlstm/blocks/slstm/layer.py:33 |  |  | 0.597 |
| walker |  | 3536 | 15 | python decl at xlstm/blocks/slstm/layer.py:18 |  |  | 0.597 |
| walker |  | 3549 | 13 | python class body at xlstm/blocks/slstm/layer.py:33 |  |  | 0.597 |
| walker |  | 3589 | 40 | python decl names surface in xlstm/components/util.py |  |  | 0.597 |
| walker |  | 3589 | 0 | python decl at xlstm/components/util.py:7 |  |  | 0.597 |
| walker |  | 3589 | 0 | python decl at xlstm/components/util.py:11 |  |  | 0.597 |
| walker |  | 3589 | 0 | python decl at xlstm/components/util.py:26 |  |  | 0.597 |
| walker |  | 3611 | 22 | python decl doc at xlstm/components/util.py:11 |  |  | 0.597 |
| walker |  | 3629 | 18 | python decl body at xlstm/components/util.py:7 body 8 |  |  | 0.597 |
| ns | 3657 |  | 420 | mlstm/backends.py — all three backend fn signatures | 3.8 |  | 0.565 |
| walker |  | 3669 | 40 | python decl names surface in xlstm/xlstm_large/utils.py |  |  | 0.565 |
| walker |  | 3669 | 0 | python decl at xlstm/xlstm_large/utils.py:5 |  |  | 0.565 |
| walker |  | 3695 | 26 | python decl at xlstm/xlstm_large/utils.py:10 |  |  | 0.565 |
| walker |  | 3710 | 15 | python decl doc at xlstm/xlstm_large/utils.py:5 |  |  | 0.565 |
| walker |  | 3734 | 24 | python decl body at xlstm/xlstm_large/utils.py:5 body 7 |  |  | 0.565 |
| walker |  | 3753 | 19 | python method doc at xlstm/xlstm_block_stack.py:40 |  |  | 0.565 |
| walker |  | 3779 | 26 | python decl names surface in xlstm/blocks/mlstm/block.py |  |  | 0.565 |
| walker |  | 3779 | 0 | python decl at xlstm/blocks/mlstm/block.py:22 |  |  | 0.565 |
| walker |  | 3788 | 9 | python decl at xlstm/blocks/mlstm/block.py:9 |  |  | 0.566 |
| walker |  | 3800 | 12 | python class body at xlstm/blocks/mlstm/block.py:22 |  |  | 0.566 |
| walker |  | 3838 | 38 | python method sigs in xlstm/blocks/mlstm/block.py |  |  | 0.566 |
| walker |  | 3838 | 0 | python method at xlstm/blocks/mlstm/block.py:17 |  |  | 0.566 |
| walker |  | 3838 | 0 | python method at xlstm/blocks/mlstm/block.py:25 |  |  | 0.566 |
| walker |  | 3864 | 26 | python decl names surface in xlstm/blocks/slstm/block.py |  |  | 0.567 |
| walker |  | 3864 | 0 | python decl at xlstm/blocks/slstm/block.py:29 |  |  | 0.567 |
| walker |  | 3873 | 9 | python decl at xlstm/blocks/slstm/block.py:12 |  |  | 0.568 |
| walker |  | 3885 | 12 | python class body at xlstm/blocks/slstm/block.py:29 |  |  | 0.568 |
| walker |  | 3920 | 35 | python method sigs in xlstm/blocks/slstm/block.py |  |  | 0.568 |
| walker |  | 3920 | 0 | python method at xlstm/blocks/slstm/block.py:21 |  |  | 0.568 |
| walker |  | 3920 | 0 | python method at xlstm/blocks/slstm/block.py:32 |  |  | 0.568 |
| walker |  | 3973 | 53 | python decl doc at xlstm/components/linear_headwise.py:42 |  |  | 0.544 |
| ns | 3973 |  | 316 | sLSTMLayerConfig + sLSTMLayer — config fields + method signatures | 3.9 |  | 0.544 |
| walker |  | 4055 | 82 | python method sigs in xlstm/components/linear_headwise.py |  |  | 0.544 |
| walker |  | 4055 | 0 | python method at xlstm/components/linear_headwise.py:31 |  |  | 0.544 |
| walker |  | 4055 | 0 | python method at xlstm/components/linear_headwise.py:49 |  |  | 0.544 |
| walker |  | 4055 | 0 | python method at xlstm/components/linear_headwise.py:75 |  |  | 0.544 |
| walker |  | 4055 | 0 | python method at xlstm/components/linear_headwise.py:84 |  |  | 0.544 |
| walker |  | 4062 | 7 | python method at xlstm/components/linear_headwise.py:67 |  |  | 0.544 |
| walker |  | 4075 | 13 | python decl names surface in xlstm/blocks/slstm/src/vanilla/lstm.py |  |  | 0.544 |
| walker |  | 4088 | 13 | python decl names surface in xlstm/blocks/slstm/src/vanilla/slstm.py |  |  | 0.544 |
| walker |  | 4238 | 150 | python method sigs in xlstm/utils.py |  |  | 0.545 |
| walker |  | 4238 | 0 | python method at xlstm/utils.py:20 |  |  | 0.545 |
| walker |  | 4238 | 0 | python method at xlstm/utils.py:33 |  |  | 0.545 |
| walker |  | 4238 | 0 | python method at xlstm/utils.py:36 |  |  | 0.545 |
| walker |  | 4238 | 0 | python method at xlstm/utils.py:61 |  |  | 0.545 |
| walker |  | 4238 | 0 | python method at xlstm/utils.py:77 |  |  | 0.545 |
| walker |  | 4248 | 10 | python method body at xlstm/utils.py:33 body 34 |  |  | 0.545 |
| walker |  | 4319 | 71 | python class body at xlstm/components/conv.py:12 |  |  | 0.546 |
| walker |  | 4338 | 19 | python decl names surface in experiments/data/formal_language/tasks/cycle_navigation.py |  |  | 0.546 |
| walker |  | 4425 | 87 | python method sigs in xlstm/components/conv.py |  |  | 0.546 |
| walker |  | 4425 | 0 | python method at xlstm/components/conv.py:20 |  |  | 0.546 |
| walker |  | 4425 | 0 | python method at xlstm/components/conv.py:71 |  |  | 0.546 |
| walker |  | 4425 | 0 | python method at xlstm/components/conv.py:95 |  |  | 0.546 |
| ns | 4430 |  | 457 | sLSTMCellConfig — full field declarations | 3.10 |  | 0.523 |
| walker |  | 4433 | 8 | python method body at xlstm/components/conv.py:95 body 96 |  |  | 0.523 |
| walker |  | 4481 | 48 | python method at xlstm/components/conv.py:131 |  |  | 0.524 |
| walker |  | 4509 | 28 | python decl at experiments/data/formal_language/tasks/cycle_navigation.py:51 |  |  | 0.524 |
| walker |  | 4561 | 52 | python method doc at xlstm/utils.py:36 |  |  | 0.524 |
| walker |  | 4615 | 54 | python method doc at xlstm/utils.py:61 |  |  | 0.524 |
| walker |  | 4669 | 54 | python method at xlstm/components/conv.py:110 |  |  | 0.525 |
| ns | 4693 |  | 263 | sLSTMCellConfig — derived properties + defines | 3.11 | 3.10 | 0.504 |
| walker |  | 4721 | 52 | python decl names surface in experiments/data/utils.py |  |  | 0.504 |
| walker |  | 4721 | 0 | python decl at experiments/data/utils.py:17 |  |  | 0.504 |
| walker |  | 4721 | 0 | python decl at experiments/data/utils.py:39 |  |  | 0.504 |
| walker |  | 4721 | 0 | python decl at experiments/data/utils.py:55 |  |  | 0.504 |
| walker |  | 4721 | 0 | python decl at experiments/data/utils.py:90 |  |  | 0.504 |
| walker |  | 4743 | 22 | python decl names surface in experiments/data/formal_language/tasks/modular_arithmetic.py |  |  | 0.504 |
| walker |  | 4846 | 103 | python method sigs in xlstm/components/util.py |  |  | 0.504 |
| walker |  | 4846 | 0 | python method at xlstm/components/util.py:58 |  |  | 0.504 |
| walker |  | 4846 | 0 | python method at xlstm/components/util.py:73 |  |  | 0.504 |
| walker |  | 4853 | 7 | python method at xlstm/components/util.py:61 |  |  | 0.504 |
| walker |  | 4860 | 7 | python method at xlstm/components/util.py:65 |  |  | 0.504 |
| walker |  | 4867 | 7 | python method at xlstm/components/util.py:69 |  |  | 0.504 |
| walker |  | 4882 | 15 | python method at xlstm/components/util.py:46 |  |  | 0.504 |
| walker |  | 4898 | 16 | python method at xlstm/components/util.py:51 |  |  | 0.504 |
| walker |  | 4914 | 16 | python method body at xlstm/components/util.py:73 body 74 |  |  | 0.504 |
| walker |  | 4933 | 19 | python method body at xlstm/components/util.py:58 body 59 |  |  | 0.504 |
| ns | 5075 |  | 382 | sLSTMCell classes — locations of all 4 + dispatcher | 3.12 |  | 0.482 |
| ns | 5363 |  | 288 | components/feedforward.py — config + class + factory | 3.13 |  | 0.470 |
| ns | 5662 |  | 299 | components/conv.py — CausalConv1d config + class + step | 3.14 |  | 0.497 |
| walker |  | 5750 | 817 | README.md section #0 |  |  | 0.518 |
| walker |  | 5753 | 3 | listing of '.github' |  |  | 0.518 |
| walker |  | 5761 | 8 | listing of '.github/workflows' |  |  | 0.518 |
| walker |  | 5798 | 37 | python method at xlstm/utils.py:97 |  |  | 0.518 |
| walker |  | 5818 | 20 | python method body at xlstm/components/conv.py:20 body 21 |  |  | 0.518 |
| walker |  | 5833 | 15 | listing of 'notebooks/xlstm_large' |  |  | 0.525 |
| walker |  | 5861 | 28 | python imports in xlstm/xlstm_large/components.py |  |  | 0.525 |
| walker |  | 5889 | 28 | python imports in xlstm/xlstm_large/generate.py |  |  | 0.525 |
| ns | 5934 |  | 272 | components/ln.py + linear_headwise.py — class signatures | 3.15 |  | 0.517 |
| walker |  | 5957 | 68 | python method sigs in xlstm/blocks/slstm/layer.py |  |  | 0.519 |
| walker |  | 5957 | 0 | python method at xlstm/blocks/slstm/layer.py:28 |  |  | 0.519 |
| walker |  | 5957 | 0 | python method at xlstm/blocks/slstm/layer.py:36 |  |  | 0.519 |
| walker |  | 5957 | 0 | python method at xlstm/blocks/slstm/layer.py:84 |  |  | 0.519 |
| walker |  | 5981 | 24 | python method at xlstm/components/conv.py:98 |  |  | 0.519 |
| ns | 6015 |  | 81 | components/init.py — full file (weight init helpers) | 3.16 |  | 0.517 |
| walker |  | 6093 | 112 | python method sigs in xlstm/blocks/xlstm_block.py |  |  | 0.523 |
| walker |  | 6093 | 0 | python method at xlstm/blocks/xlstm_block.py:27 |  |  | 0.523 |
| walker |  | 6093 | 0 | python method at xlstm/blocks/xlstm_block.py:51 |  |  | 0.523 |
| walker |  | 6093 | 0 | python method at xlstm/blocks/xlstm_block.py:76 |  |  | 0.523 |
| walker |  | 6093 | 0 | python method at xlstm/blocks/xlstm_block.py:82 |  |  | 0.523 |
| walker |  | 6093 | 0 | python method at xlstm/blocks/xlstm_block.py:89 |  |  | 0.523 |
| walker |  | 6153 | 60 | python method at xlstm/components/util.py:34 |  |  | 0.523 |
| walker |  | 6191 | 38 | python decl names surface in xlstm/blocks/mlstm/backends.py |  |  | 0.523 |
| ns | 6195 |  | 180 | components/util.py — ParameterProxy + helpers | 3.17 |  | 0.519 |
| walker |  | 6212 | 21 | python method body at xlstm/components/util.py:61 body 63 |  |  | 0.519 |
| walker |  | 6233 | 21 | python method body at xlstm/components/util.py:69 body 71 |  |  | 0.519 |
| walker |  | 6287 | 54 | python imports in experiments/metrics.py |  |  | 0.519 |
| walker |  | 6389 | 102 | python class body at xlstm/blocks/xlstm_block.py:16 |  |  | 0.531 |
| walker |  | 6428 | 39 | python decl names surface in experiments/data/formal_language/online_generate.py |  |  | 0.531 |
| walker |  | 6428 | 0 | python decl at experiments/data/formal_language/online_generate.py:9 |  |  | 0.531 |
| walker |  | 6428 | 0 | python decl at experiments/data/formal_language/online_generate.py:29 |  |  | 0.531 |
| walker |  | 6428 | 0 | python decl at experiments/data/formal_language/online_generate.py:41 |  |  | 0.531 |
| walker |  | 6476 | 48 | python decl doc at experiments/data/formal_language/online_generate.py:41 |  |  | 0.531 |
| ns | 6487 |  | 292 | xlstm/utils.py — config mixin + optim-group mixin | 3.18 |  | 0.542 |
| walker |  | 6599 | 123 | python method sigs in experiments/data/utils.py |  |  | 0.542 |
| walker |  | 6599 | 0 | python method at experiments/data/utils.py:41 |  |  | 0.542 |
| walker |  | 6608 | 9 | python method at experiments/data/utils.py:85 |  |  | 0.542 |
| walker |  | 6618 | 10 | python method at experiments/data/utils.py:77 |  |  | 0.542 |
| walker |  | 6629 | 11 | python method at experiments/data/utils.py:50 |  |  | 0.542 |
| walker |  | 6641 | 12 | python method at experiments/data/utils.py:46 |  |  | 0.542 |
| walker |  | 6658 | 17 | python method at experiments/data/utils.py:57 |  |  | 0.542 |
| walker |  | 6681 | 23 | python method at experiments/data/utils.py:19 |  |  | 0.542 |
| walker |  | 6688 | 7 | python method body at experiments/data/utils.py:19 body 22 |  |  | 0.542 |
| ns | 6691 |  | 204 | xLSTMLargeConfig — all remaining fields (refines 2.7) | 4.1 | 2.7 | 0.535 |
| walker |  | 6712 | 24 | python method at experiments/data/utils.py:29 |  |  | 0.535 |
| walker |  | 6719 | 7 | python method body at experiments/data/utils.py:29 body 32 |  |  | 0.535 |
| walker |  | 6743 | 24 | python method at experiments/data/utils.py:34 |  |  | 0.535 |
| walker |  | 6750 | 7 | python method body at experiments/data/utils.py:34 body 37 |  |  | 0.535 |
| walker |  | 6776 | 26 | python method at experiments/data/utils.py:24 |  |  | 0.535 |
| walker |  | 6783 | 7 | python method body at experiments/data/utils.py:24 body 27 |  |  | 0.535 |
| walker |  | 6794 | 11 | python method body at experiments/data/utils.py:50 body 52 |  |  | 0.535 |
| walker |  | 6806 | 12 | python method body at experiments/data/utils.py:46 body 48 |  |  | 0.535 |
| walker |  | 6825 | 19 | python method body at experiments/data/utils.py:85 body 87 |  |  | 0.535 |
| walker |  | 6847 | 22 | python method body at xlstm/components/util.py:65 body 67 |  |  | 0.535 |
| walker |  | 6904 | 57 | python imports in experiments/lr_scheduler.py |  |  | 0.535 |
| walker |  | 6980 | 76 | python method sigs in xlstm/blocks/mlstm/cell.py |  |  | 0.542 |
| walker |  | 6980 | 0 | python method at xlstm/blocks/mlstm/cell.py:23 |  |  | 0.542 |
| walker |  | 6980 | 0 | python method at xlstm/blocks/mlstm/cell.py:43 |  |  | 0.542 |
| walker |  | 6980 | 0 | python method at xlstm/blocks/mlstm/cell.py:133 |  |  | 0.542 |
| walker |  | 6997 | 17 | python decl names surface #1 in xlstm/blocks/slstm/cell.py |  |  | 0.542 |
| walker |  | 6997 | 0 | python decl at xlstm/blocks/slstm/cell.py:777 |  |  | 0.542 |
| walker |  | 7009 | 12 | python class body at xlstm/blocks/slstm/cell.py:777 |  |  | 0.542 |
| walker |  | 7038 | 29 | python method sigs #1 in xlstm/blocks/slstm/cell.py |  |  | 0.543 |
| walker |  | 7038 | 0 | python method at xlstm/blocks/slstm/cell.py:780 |  |  | 0.543 |
| ns | 7076 |  | 385 | xlstm_large/model.py — class skeleton (4 classes) | 4.2 |  | 0.530 |
| walker |  | 7119 | 81 | python method sigs in xlstm/blocks/mlstm/layer.py |  |  | 0.533 |
| walker |  | 7119 | 0 | python method at xlstm/blocks/mlstm/layer.py:34 |  |  | 0.533 |
| walker |  | 7119 | 0 | python method at xlstm/blocks/mlstm/layer.py:42 |  |  | 0.533 |
| walker |  | 7119 | 0 | python method at xlstm/blocks/mlstm/layer.py:101 |  |  | 0.533 |
| walker |  | 7127 | 8 | python method at xlstm/blocks/mlstm/layer.py:158 |  |  | 0.533 |
| walker |  | 7199 | 72 | python class body at xlstm/blocks/mlstm/block.py:9 |  |  | 0.539 |
| ns | 7248 |  | 172 | xlstm_large/model.py — mLSTMLayerConfig fields | 4.3 |  | 0.533 |
| walker |  | 7271 | 72 | python method at xlstm/components/ln.py:11 |  |  | 0.542 |
| walker |  | 7366 | 95 | python decl doc at xlstm/components/util.py:26 |  |  | 0.554 |
| ns | 7388 |  | 140 | xlstm_large/components.py — soft_cap + Norm classes | 4.4 |  | 0.548 |
| walker |  | 7406 | 40 | python method body at experiments/metrics.py:17 body 18 |  |  | 0.548 |
| walker |  | 7643 | 237 | python class body at xlstm/xlstm_block_stack.py:15 |  |  | 0.577 |
| ns | 7648 |  | 260 | xlstm_large/from_pretrained.py — HF checkpoint loader | 4.5 |  | 0.566 |
| walker |  | 7722 | 79 | python decl names surface in xlstm/components/feedforward.py |  |  | 0.568 |
| walker |  | 7722 | 0 | python decl at xlstm/components/feedforward.py:22 |  |  | 0.568 |
| walker |  | 7722 | 0 | python decl at xlstm/components/feedforward.py:49 |  |  | 0.568 |
| walker |  | 7722 | 0 | python decl at xlstm/components/feedforward.py:90 |  |  | 0.568 |
| walker |  | 7734 | 12 | python decl at xlstm/components/feedforward.py:31 |  |  | 0.569 |
| walker |  | 7744 | 10 | python class body at xlstm/components/feedforward.py:49 |  |  | 0.569 |
| walker |  | 7812 | 68 | python method sigs in xlstm/components/feedforward.py |  |  | 0.570 |
| walker |  | 7812 | 0 | python method at xlstm/components/feedforward.py:42 |  |  | 0.570 |
| walker |  | 7812 | 0 | python method at xlstm/components/feedforward.py:52 |  |  | 0.570 |
| walker |  | 7812 | 0 | python method at xlstm/components/feedforward.py:72 |  |  | 0.570 |
| walker |  | 7812 | 0 | python method at xlstm/components/feedforward.py:77 |  |  | 0.570 |
| walker |  | 7915 | 103 | python class body at xlstm/components/feedforward.py:31 |  |  | 0.578 |
| walker |  | 7969 | 54 | python method at xlstm/blocks/slstm/layer.py:92 |  |  | 0.582 |
| ns | 7975 |  | 327 | xlstm_large/generate.py — generate_tokens + sampling registry | 4.6 |  | 0.569 |
| walker |  | 8043 | 74 | python imports in xlstm/utils.py |  |  | 0.569 |
| walker |  | 8095 | 52 | python decl body at xlstm/components/feedforward.py:90 body 91 |  |  | 0.569 |
| walker |  | 8138 | 43 | python imports in xlstm/components/init.py |  |  | 0.569 |
| walker |  | 8266 | 128 | python decl at xlstm/xlstm_large/from_pretrained.py:9 |  |  | 0.575 |
| ns | 8293 |  | 318 | xlstm_large/utils.py — single→fused weight conversion | 4.7 |  | 0.564 |
| walker |  | 8353 | 87 | python decl names surface in xlstm/components/init.py |  |  | 0.568 |
| walker |  | 8353 | 0 | python decl at xlstm/components/init.py:8 |  |  | 0.568 |
| walker |  | 8353 | 0 | python decl at xlstm/components/init.py:18 |  |  | 0.568 |
| walker |  | 8353 | 0 | python decl at xlstm/components/init.py:28 |  |  | 0.568 |
| walker |  | 8365 | 12 | python decl doc at xlstm/components/init.py:8 |  |  | 0.568 |
| walker |  | 8395 | 30 | python decl doc at xlstm/components/init.py:28 |  |  | 0.568 |
| walker |  | 8487 | 92 | python decl doc at xlstm/components/init.py:18 |  |  | 0.568 |
| walker |  | 8532 | 45 | python method body at xlstm/xlstm_block_stack.py:117 body 119 |  |  | 0.568 |
| walker |  | 8621 | 89 | python decl names surface in xlstm/xlstm_large/generate.py |  |  | 0.569 |
| walker |  | 8621 | 0 | python decl at xlstm/xlstm_large/generate.py:13 |  |  | 0.569 |
| walker |  | 8621 | 0 | python decl at xlstm/xlstm_large/generate.py:23 |  |  | 0.569 |
| walker |  | 8634 | 13 | python decl body at xlstm/xlstm_large/generate.py:23 body 24 |  |  | 0.569 |
| walker |  | 8649 | 15 | python decl body at xlstm/xlstm_large/generate.py:13 body 15 |  |  | 0.570 |
| walker |  | 8669 | 20 | python decl doc at xlstm/xlstm_large/generate.py:13 |  |  | 0.571 |
| ns | 8698 |  | 405 | experiments/main.py — training entry point | 4.8 |  | 0.556 |
| walker |  | 8700 | 31 | python decl at xlstm/xlstm_large/generate.py:8 |  |  | 0.556 |
| walker |  | 8717 | 17 | python decl at xlstm/xlstm_large/generate.py:18 |  |  | 0.557 |
| walker |  | 8763 | 46 | python method body at xlstm/xlstm_block_stack.py:111 body 112 |  |  | 0.557 |
| walker |  | 8858 | 95 | python class body at xlstm/blocks/slstm/block.py:12 |  |  | 0.569 |
| walker |  | 8914 | 56 | listing of 'xlstm/blocks/slstm/src/cuda' |  |  | 0.578 |
| walker |  | 8924 | 10 | c decl names surface in xlstm/blocks/slstm/src/cuda/slstm.h |  |  | 0.578 |
| walker |  | 8924 | 0 | c decl at xlstm/blocks/slstm/src/cuda/slstm.h:44 |  |  | 0.578 |
| walker |  | 8935 | 11 | c includes in xlstm/blocks/slstm/src/cuda/slstm.h |  |  | 0.578 |
| ns | 9065 |  | 367 | CUDA bindings — slstm.cc PYBIND11 + sLSTMFunc class | 4.9 |  | 0.567 |
| walker |  | 9183 | 248 | c header banner in xlstm/blocks/slstm/src/cuda/slstm.h |  |  | 0.567 |
| walker |  | 9231 | 48 | python imports in xlstm/components/util.py |  |  | 0.567 |
| walker |  | 9330 | 99 | python decl names surface in xlstm/xlstm_large/components.py |  |  | 0.571 |
| walker |  | 9330 | 0 | python decl at xlstm/xlstm_large/components.py:24 |  |  | 0.571 |
| walker |  | 9330 | 0 | python decl at xlstm/xlstm_large/components.py:70 |  |  | 0.571 |
| walker |  | 9343 | 13 | python decl at xlstm/xlstm_large/components.py:188 |  |  | 0.571 |
| walker |  | 9356 | 13 | python decl at xlstm/xlstm_large/components.py:5 |  |  | 0.571 |
| ns | 9375 |  | 310 | cuda_init.py — torch.utils.cpp_extension.load wrapper | 4.10 |  | 0.562 |
| walker |  | 9388 | 32 | python decl at xlstm/xlstm_large/components.py:155 |  |  | 0.562 |
| walker |  | 9422 | 34 | python decl at xlstm/xlstm_large/components.py:99 |  |  | 0.562 |
| walker |  | 9596 | 174 | python method sigs in xlstm/xlstm_large/components.py |  |  | 0.564 |
| walker |  | 9596 | 0 | python method at xlstm/xlstm_large/components.py:58 |  |  | 0.564 |
| walker |  | 9611 | 15 | python method at xlstm/xlstm_large/components.py:93 |  |  | 0.564 |
| walker |  | 9626 | 15 | python method at xlstm/xlstm_large/components.py:182 |  |  | 0.564 |
| walker |  | 9642 | 16 | python method at xlstm/xlstm_large/components.py:65 |  |  | 0.566 |
| walker |  | 9652 | 10 | python method body at xlstm/xlstm_large/components.py:65 body 67 |  |  | 0.568 |
| ns | 9663 |  | 288 | vanilla sLSTM/LSTM pointwise + slstm_forward driver | 4.11 |  | 0.563 |
| walker |  | 9697 | 45 | python method at xlstm/xlstm_large/components.py:142 |  |  | 0.563 |
| walker |  | 9742 | 45 | python method at xlstm/xlstm_large/components.py:231 |  |  | 0.563 |
| walker |  | 9818 | 76 | python method at xlstm/xlstm_large/components.py:35 |  |  | 0.565 |
| ns | 9841 |  | 178 | tests/ — test fn locations only | 4.12 |  | 0.559 |
| walker |  | 9904 | 86 | python method at xlstm/xlstm_large/components.py:123 |  |  | 0.559 |
| ns | 9989 |  | 148 | experiments support — task fns + dataset/scheduler entry points | 4.13 |  | 0.556 |
| walker |  | 9990 | 86 | python method at xlstm/xlstm_large/components.py:212 |  |  | 0.556 |
