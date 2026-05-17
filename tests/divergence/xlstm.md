Score(3000)=0.600 I=0.804 C=0.448 ns_rows≤3K=22/48 (reached=11 partial=1 missing=10)

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
| walker |  | 498 | 62 | python imports in xlstm/blocks/slstm/src/vanilla/__init__.py |  |  | 0.699 |
| walker |  | 510 | 12 | python decl names surface in experiments/metrics.py |  |  | 0.699 |
| walker |  | 510 | 0 | python decl at experiments/metrics.py:9 |  |  | 0.699 |
| walker |  | 551 | 41 | python decl names surface in xlstm/blocks/slstm/src/vanilla/__init__.py |  |  | 0.699 |
| walker |  | 579 | 28 | listing of 'res' |  |  | 0.699 |
| ns | 611 |  | 164 | README — xLSTM Large signpost | 1.7 |  | 0.658 |
| walker |  | 618 | 39 | python decl at xlstm/blocks/slstm/src/vanilla/__init__.py:11 |  |  | 0.659 |
| ns | 727 |  | 116 | README — NeurIPS-paper-models signpost | 1.8 |  | 0.612 |
| ns | 747 |  | 20 | xlstm/blocks/ directory listing | 2.1 |  | 0.634 |
| ns | 780 |  | 33 | xlstm/components/ directory listing | 2.2 |  | 0.664 |
| walker |  | 795 | 177 | python imports in xlstm/__init__.py |  |  | 0.829 |
| ns | 808 |  | 28 | xlstm/xlstm_large/ directory listing | 2.3 |  | 0.835 |
| walker |  | 820 | 25 | python decl names surface in xlstm/xlstm_block_stack.py |  |  | 0.835 |
| walker |  | 820 | 0 | python decl at xlstm/xlstm_block_stack.py:77 |  |  | 0.835 |
| walker |  | 830 | 10 | python decl at xlstm/xlstm_block_stack.py:15 |  |  | 0.835 |
| walker |  | 843 | 13 | python class body at xlstm/xlstm_block_stack.py:77 |  |  | 0.835 |
| ns | 852 |  | 44 | xlstm/blocks/mlstm/ + slstm/ listings | 2.4 |  | 0.842 |
| walker |  | 870 | 27 | python decl names surface in xlstm/utils.py |  |  | 0.842 |
| walker |  | 870 | 0 | python decl at xlstm/utils.py:32 |  |  | 0.842 |
| walker |  | 878 | 8 | python decl at xlstm/utils.py:11 |  |  | 0.842 |
| walker |  | 890 | 12 | python decl names surface in xlstm/xlstm_large/from_pretrained.py |  |  | 0.842 |
| walker |  | 923 | 33 | python class body at experiments/metrics.py:9 |  |  | 0.842 |
| walker |  | 956 | 33 | python decl names surface in xlstm/xlstm_lm_model.py |  |  | 0.843 |
| walker |  | 956 | 0 | python decl at xlstm/xlstm_lm_model.py:22 |  |  | 0.843 |
| walker |  | 973 | 17 | python decl at xlstm/xlstm_lm_model.py:14 |  |  | 0.843 |
| walker |  | 987 | 14 | python class body at xlstm/xlstm_lm_model.py:22 |  |  | 0.844 |
| walker |  | 1064 | 77 | [package] in pyproject.toml |  |  | 0.844 |
| ns | 1124 |  | 272 | xLSTMBlockStackConfig field list | 2.5 |  | 0.748 |
| ns | 1250 |  | 126 | xLSTMLMModelConfig — LM-specific fields | 2.6 |  | 0.720 |
| walker |  | 1252 | 188 | headings outline in README.md |  |  | 0.728 |
| walker |  | 1290 | 38 | python decl names surface in experiments/lr_scheduler.py |  |  | 0.728 |
| walker |  | 1290 | 0 | python decl at experiments/lr_scheduler.py:9 |  |  | 0.728 |
| walker |  | 1290 | 0 | python decl at experiments/lr_scheduler.py:24 |  |  | 0.728 |
| walker |  | 1342 | 52 | python class body at xlstm/xlstm_lm_model.py:14 |  |  | 0.762 |
| ns | 1409 |  | 159 | xLSTMLargeConfig — required + key fields | 2.7 |  | 0.720 |
| walker |  | 1527 | 185 | [dependencies] in pyproject.toml |  |  | 0.720 |
| ns | 1583 |  | 174 | xlstm/blocks/slstm/src/ directory listings | 2.8 |  | 0.648 |
| walker |  | 1592 | 65 | python method sigs in experiments/metrics.py |  |  | 0.648 |
| walker |  | 1592 | 0 | python method at experiments/metrics.py:13 |  |  | 0.648 |
| walker |  | 1592 | 0 | python method at experiments/metrics.py:17 |  |  | 0.648 |
| walker |  | 1592 | 0 | python method at experiments/metrics.py:22 |  |  | 0.648 |
| walker |  | 1592 | 0 | python method at experiments/metrics.py:25 |  |  | 0.648 |
| walker |  | 1601 | 9 | python method body at experiments/metrics.py:22 body 23 |  |  | 0.648 |
| walker |  | 1610 | 9 | python decl names surface in experiments/data/formal_language/tasks/parity.py |  |  | 0.648 |
| walker |  | 1620 | 10 | python decl names surface in experiments/data/formal_language/tasks/even_pairs.py |  |  | 0.648 |
| walker |  | 1679 | 59 | python decl names surface in experiments/main.py |  |  | 0.648 |
| walker |  | 1679 | 0 | python decl at experiments/main.py:32 |  |  | 0.648 |
| walker |  | 1679 | 0 | python decl at experiments/main.py:37 |  |  | 0.648 |
| walker |  | 1698 | 19 | python decl at experiments/main.py:21 |  |  | 0.648 |
| walker |  | 1744 | 46 | python decl at experiments/main.py:25 |  |  | 0.648 |
| walker |  | 1772 | 28 | python decl body at experiments/main.py:32 body 33 |  |  | 0.648 |
| walker |  | 1796 | 24 | python decl names surface in xlstm/blocks/xlstm_block.py |  |  | 0.648 |
| walker |  | 1796 | 0 | python decl at xlstm/blocks/xlstm_block.py:43 |  |  | 0.648 |
| ns | 1803 |  | 220 | experiments/ + tests/ + notebooks/ listings | 2.9 |  | 0.630 |
| walker |  | 1805 | 9 | python decl at xlstm/blocks/xlstm_block.py:16 |  |  | 0.630 |
| walker |  | 1819 | 14 | python class body at xlstm/blocks/xlstm_block.py:43 |  |  | 0.630 |
| walker |  | 1843 | 24 | python decl names surface in xlstm/components/linear_headwise.py |  |  | 0.630 |
| walker |  | 1843 | 0 | python decl at xlstm/components/linear_headwise.py:42 |  |  | 0.630 |
| walker |  | 1852 | 9 | python decl at xlstm/components/linear_headwise.py:12 |  |  | 0.630 |
| walker |  | 1866 | 14 | python class body at xlstm/components/linear_headwise.py:42 |  |  | 0.630 |
| walker |  | 1919 | 53 | listing of 'tests' |  |  | 0.669 |
| walker |  | 1946 | 27 | python decl names surface in xlstm/components/ln.py |  |  | 0.669 |
| walker |  | 1946 | 0 | python decl at xlstm/components/ln.py:8 |  |  | 0.669 |
| walker |  | 1946 | 0 | python decl at xlstm/components/ln.py:51 |  |  | 0.669 |
| ns | 1954 |  | 151 | xLSTMBlockStack — class headers + fn signatures | 3.1 |  | 0.653 |
| walker |  | 1966 | 20 | python decl doc at xlstm/components/ln.py:8 |  |  | 0.653 |
| walker |  | 2050 | 84 | python class body at xlstm/utils.py:11 |  |  | 0.654 |
| ns | 2104 |  | 150 | xLSTMLMModel — class header + fn signatures | 3.2 |  | 0.641 |
| walker |  | 2149 | 99 | python method sigs in experiments/lr_scheduler.py |  |  | 0.641 |
| walker |  | 2149 | 0 | python method at experiments/lr_scheduler.py:10 |  |  | 0.641 |
| walker |  | 2149 | 0 | python method at experiments/lr_scheduler.py:25 |  |  | 0.641 |
| walker |  | 2149 | 0 | python method at experiments/lr_scheduler.py:47 |  |  | 0.641 |
| walker |  | 2162 | 13 | python method at experiments/lr_scheduler.py:13 |  |  | 0.641 |
| walker |  | 2176 | 14 | python method at experiments/lr_scheduler.py:18 |  |  | 0.641 |
| walker |  | 2198 | 22 | python method at experiments/lr_scheduler.py:33 |  |  | 0.641 |
| walker |  | 2212 | 14 | python method doc at experiments/lr_scheduler.py:18 |  |  | 0.641 |
| walker |  | 2226 | 14 | python method doc at experiments/lr_scheduler.py:47 |  |  | 0.641 |
| walker |  | 2236 | 10 | python method body at experiments/lr_scheduler.py:13 body 16 |  |  | 0.641 |
| walker |  | 2246 | 10 | python method body at experiments/lr_scheduler.py:18 body 21 |  |  | 0.641 |
| walker |  | 2262 | 16 | python method doc at experiments/lr_scheduler.py:13 |  |  | 0.641 |
| walker |  | 2274 | 12 | python method body at experiments/lr_scheduler.py:10 body 11 |  |  | 0.641 |
| walker |  | 2288 | 14 | python imports in xlstm/components/init.py |  |  | 0.641 |
| walker |  | 2302 | 14 | python imports in xlstm/xlstm_large/utils.py |  |  | 0.641 |
| walker |  | 2403 | 101 | python method sigs in xlstm/xlstm_lm_model.py |  |  | 0.651 |
| walker |  | 2403 | 0 | python method at xlstm/xlstm_lm_model.py:25 |  |  | 0.651 |
| walker |  | 2403 | 0 | python method at xlstm/xlstm_lm_model.py:41 |  |  | 0.651 |
| walker |  | 2403 | 0 | python method at xlstm/xlstm_lm_model.py:49 |  |  | 0.651 |
| walker |  | 2403 | 0 | python method at xlstm/xlstm_lm_model.py:65 |  |  | 0.651 |
| ns | 2420 |  | 316 | xLSTMBlockConfig + xLSTMBlock — the shared parent block | 3.3 |  | 0.623 |
| walker |  | 2460 | 57 | python method at xlstm/xlstm_lm_model.py:56 |  |  | 0.635 |
| walker |  | 2488 | 28 | python imports in experiments/lr_scheduler.py |  |  | 0.635 |
| walker |  | 2504 | 16 | python imports in xlstm/components/util.py |  |  | 0.635 |
| ns | 2673 |  | 253 | mLSTMBlock + sLSTMBlock — thin subclass shells | 3.4 |  | 0.606 |
| walker |  | 2676 | 172 | python decl at xlstm/blocks/slstm/src/vanilla/__init__.py:77 |  |  | 0.606 |
| walker |  | 2850 | 174 | python decl at xlstm/blocks/slstm/src/vanilla/__init__.py:17 |  |  | 0.606 |
| walker |  | 2872 | 22 | python method body at experiments/metrics.py:25 body 26 |  |  | 0.606 |
| ns | 2889 |  | 216 | mLSTMLayerConfig — field declarations | 3.5 |  | 0.584 |
| walker |  | 2905 | 33 | listing of 'notebooks/xlstm' |  |  | 0.600 |
| walker |  | 2942 | 37 | python decl names surface in xlstm/components/conv.py |  |  | 0.600 |
| walker |  | 2942 | 0 | python decl at xlstm/components/conv.py:55 |  |  | 0.600 |
| walker |  | 2952 | 10 | python decl at xlstm/components/conv.py:12 |  |  | 0.600 |
| walker |  | 3013 | 61 | python decl at xlstm/components/conv.py:24 |  |  | 0.600 |
| walker |  | 3037 | 24 | python decl names surface in xlstm/blocks/mlstm/cell.py |  |  | 0.600 |
| walker |  | 3037 | 0 | python decl at xlstm/blocks/mlstm/cell.py:20 |  |  | 0.600 |
| ns | 3040 |  | 151 | mLSTMLayer — class header + fn signatures | 3.6 |  | 0.588 |
| walker |  | 3046 | 9 | python decl at xlstm/blocks/mlstm/cell.py:13 |  |  | 0.588 |
| walker |  | 3058 | 12 | python class body at xlstm/blocks/mlstm/cell.py:20 |  |  | 0.588 |
| walker |  | 3081 | 23 | python method body at experiments/metrics.py:13 body 14 |  |  | 0.588 |
| walker |  | 3154 | 73 | python method sigs in xlstm/components/ln.py |  |  | 0.588 |
| walker |  | 3154 | 0 | python method at xlstm/components/ln.py:36 |  |  | 0.588 |
| walker |  | 3154 | 0 | python method at xlstm/components/ln.py:41 |  |  | 0.588 |
| walker |  | 3154 | 0 | python method at xlstm/components/ln.py:53 |  |  | 0.588 |
| walker |  | 3166 | 12 | python method at xlstm/components/ln.py:27 |  |  | 0.588 |
| walker |  | 3214 | 48 | python decl doc at xlstm/blocks/xlstm_block.py:43 |  |  | 0.589 |
| ns | 3237 |  | 197 | mLSTMCell + mLSTMCellConfig | 3.7 |  | 0.572 |
| walker |  | 3347 | 133 | python method sigs in xlstm/xlstm_block_stack.py |  |  | 0.581 |
| walker |  | 3347 | 0 | python method at xlstm/xlstm_block_stack.py:40 |  |  | 0.581 |
| walker |  | 3347 | 0 | python method at xlstm/xlstm_block_stack.py:52 |  |  | 0.581 |
| walker |  | 3347 | 0 | python method at xlstm/xlstm_block_stack.py:80 |  |  | 0.581 |
| walker |  | 3347 | 0 | python method at xlstm/xlstm_block_stack.py:90 |  |  | 0.581 |
| walker |  | 3347 | 0 | python method at xlstm/xlstm_block_stack.py:111 |  |  | 0.581 |
| walker |  | 3347 | 0 | python method at xlstm/xlstm_block_stack.py:117 |  |  | 0.581 |
| walker |  | 3359 | 12 | python method at xlstm/xlstm_block_stack.py:36 |  |  | 0.581 |
| walker |  | 3376 | 17 | python method body at xlstm/xlstm_block_stack.py:36 body 38 |  |  | 0.581 |
| walker |  | 3430 | 54 | python method at xlstm/xlstm_block_stack.py:126 |  |  | 0.593 |
| walker |  | 3464 | 34 | python imports in experiments/metrics.py |  |  | 0.593 |
| walker |  | 3503 | 39 | python class body at xlstm/blocks/mlstm/cell.py:13 |  |  | 0.596 |
| walker |  | 3528 | 25 | python decl names surface in xlstm/blocks/mlstm/layer.py |  |  | 0.596 |
| walker |  | 3528 | 0 | python decl at xlstm/blocks/mlstm/layer.py:39 |  |  | 0.596 |
| walker |  | 3543 | 15 | python decl at xlstm/blocks/mlstm/layer.py:18 |  |  | 0.597 |
| walker |  | 3556 | 13 | python class body at xlstm/blocks/mlstm/layer.py:39 |  |  | 0.597 |
| walker |  | 3581 | 25 | python decl names surface in xlstm/blocks/slstm/layer.py |  |  | 0.597 |
| walker |  | 3581 | 0 | python decl at xlstm/blocks/slstm/layer.py:33 |  |  | 0.597 |
| walker |  | 3596 | 15 | python decl at xlstm/blocks/slstm/layer.py:18 |  |  | 0.597 |
| walker |  | 3609 | 13 | python class body at xlstm/blocks/slstm/layer.py:33 |  |  | 0.597 |
| walker |  | 3649 | 40 | python decl names surface in xlstm/components/util.py |  |  | 0.597 |
| walker |  | 3649 | 0 | python decl at xlstm/components/util.py:7 |  |  | 0.597 |
| walker |  | 3649 | 0 | python decl at xlstm/components/util.py:11 |  |  | 0.597 |
| walker |  | 3649 | 0 | python decl at xlstm/components/util.py:26 |  |  | 0.597 |
| ns | 3657 |  | 420 | mlstm/backends.py — all three backend fn signatures | 3.8 |  | 0.564 |
| walker |  | 3671 | 22 | python decl doc at xlstm/components/util.py:11 |  |  | 0.565 |
| walker |  | 3689 | 18 | python decl body at xlstm/components/util.py:7 body 8 |  |  | 0.565 |
| walker |  | 3729 | 40 | python decl names surface in xlstm/xlstm_large/utils.py |  |  | 0.565 |
| walker |  | 3729 | 0 | python decl at xlstm/xlstm_large/utils.py:5 |  |  | 0.565 |
| walker |  | 3755 | 26 | python decl at xlstm/xlstm_large/utils.py:10 |  |  | 0.565 |
| walker |  | 3770 | 15 | python decl doc at xlstm/xlstm_large/utils.py:5 |  |  | 0.565 |
| walker |  | 3794 | 24 | python decl body at xlstm/xlstm_large/utils.py:5 body 7 |  |  | 0.565 |
| walker |  | 3813 | 19 | python method doc at xlstm/xlstm_block_stack.py:40 |  |  | 0.565 |
| walker |  | 3839 | 26 | python decl names surface in xlstm/blocks/mlstm/block.py |  |  | 0.565 |
| walker |  | 3839 | 0 | python decl at xlstm/blocks/mlstm/block.py:22 |  |  | 0.565 |
| walker |  | 3848 | 9 | python decl at xlstm/blocks/mlstm/block.py:9 |  |  | 0.566 |
| walker |  | 3860 | 12 | python class body at xlstm/blocks/mlstm/block.py:22 |  |  | 0.566 |
| walker |  | 3898 | 38 | python method sigs in xlstm/blocks/mlstm/block.py |  |  | 0.566 |
| walker |  | 3898 | 0 | python method at xlstm/blocks/mlstm/block.py:17 |  |  | 0.566 |
| walker |  | 3898 | 0 | python method at xlstm/blocks/mlstm/block.py:25 |  |  | 0.566 |
| walker |  | 3924 | 26 | python decl names surface in xlstm/blocks/slstm/block.py |  |  | 0.567 |
| walker |  | 3924 | 0 | python decl at xlstm/blocks/slstm/block.py:29 |  |  | 0.567 |
| walker |  | 3933 | 9 | python decl at xlstm/blocks/slstm/block.py:12 |  |  | 0.568 |
| walker |  | 3945 | 12 | python class body at xlstm/blocks/slstm/block.py:29 |  |  | 0.568 |
| ns | 3973 |  | 316 | sLSTMLayerConfig + sLSTMLayer — config fields + method signatures | 3.9 |  | 0.544 |
| walker |  | 3980 | 35 | python method sigs in xlstm/blocks/slstm/block.py |  |  | 0.544 |
| walker |  | 3980 | 0 | python method at xlstm/blocks/slstm/block.py:21 |  |  | 0.544 |
| walker |  | 3980 | 0 | python method at xlstm/blocks/slstm/block.py:32 |  |  | 0.544 |
| walker |  | 4033 | 53 | python decl doc at xlstm/components/linear_headwise.py:42 |  |  | 0.544 |
| walker |  | 4115 | 82 | python method sigs in xlstm/components/linear_headwise.py |  |  | 0.544 |
| walker |  | 4115 | 0 | python method at xlstm/components/linear_headwise.py:31 |  |  | 0.544 |
| walker |  | 4115 | 0 | python method at xlstm/components/linear_headwise.py:49 |  |  | 0.544 |
| walker |  | 4115 | 0 | python method at xlstm/components/linear_headwise.py:75 |  |  | 0.544 |
| walker |  | 4115 | 0 | python method at xlstm/components/linear_headwise.py:84 |  |  | 0.544 |
| walker |  | 4122 | 7 | python method at xlstm/components/linear_headwise.py:67 |  |  | 0.544 |
| walker |  | 4135 | 13 | python decl names surface in xlstm/blocks/slstm/src/vanilla/lstm.py |  |  | 0.544 |
| walker |  | 4148 | 13 | python decl names surface in xlstm/blocks/slstm/src/vanilla/slstm.py |  |  | 0.544 |
| walker |  | 4298 | 150 | python method sigs in xlstm/utils.py |  |  | 0.545 |
| walker |  | 4298 | 0 | python method at xlstm/utils.py:20 |  |  | 0.545 |
| walker |  | 4298 | 0 | python method at xlstm/utils.py:33 |  |  | 0.545 |
| walker |  | 4298 | 0 | python method at xlstm/utils.py:36 |  |  | 0.545 |
| walker |  | 4298 | 0 | python method at xlstm/utils.py:61 |  |  | 0.545 |
| walker |  | 4298 | 0 | python method at xlstm/utils.py:77 |  |  | 0.545 |
| walker |  | 4308 | 10 | python method body at xlstm/utils.py:33 body 34 |  |  | 0.545 |
| walker |  | 4379 | 71 | python class body at xlstm/components/conv.py:12 |  |  | 0.546 |
| walker |  | 4398 | 19 | python decl names surface in experiments/data/formal_language/tasks/cycle_navigation.py |  |  | 0.546 |
| ns | 4430 |  | 457 | sLSTMCellConfig — full field declarations | 3.10 |  | 0.523 |
| walker |  | 4485 | 87 | python method sigs in xlstm/components/conv.py |  |  | 0.523 |
| walker |  | 4485 | 0 | python method at xlstm/components/conv.py:20 |  |  | 0.523 |
| walker |  | 4485 | 0 | python method at xlstm/components/conv.py:71 |  |  | 0.523 |
| walker |  | 4485 | 0 | python method at xlstm/components/conv.py:95 |  |  | 0.523 |
| walker |  | 4493 | 8 | python method body at xlstm/components/conv.py:95 body 96 |  |  | 0.523 |
| walker |  | 4541 | 48 | python method at xlstm/components/conv.py:131 |  |  | 0.524 |
| walker |  | 4569 | 28 | python decl at experiments/data/formal_language/tasks/cycle_navigation.py:51 |  |  | 0.524 |
| walker |  | 4621 | 52 | python method doc at xlstm/utils.py:36 |  |  | 0.524 |
| walker |  | 4675 | 54 | python method doc at xlstm/utils.py:61 |  |  | 0.524 |
| ns | 4693 |  | 263 | sLSTMCellConfig — derived properties + defines | 3.11 | 3.10 | 0.503 |
| walker |  | 4720 | 45 | python imports in xlstm/utils.py |  |  | 0.503 |
| walker |  | 4774 | 54 | python method at xlstm/components/conv.py:110 |  |  | 0.504 |
| walker |  | 4781 | 7 | python imports in xlstm/blocks/slstm/src/vanilla/lstm.py |  |  | 0.504 |
| walker |  | 4833 | 52 | python decl names surface in experiments/data/utils.py |  |  | 0.504 |
| walker |  | 4833 | 0 | python decl at experiments/data/utils.py:17 |  |  | 0.504 |
| walker |  | 4833 | 0 | python decl at experiments/data/utils.py:39 |  |  | 0.504 |
| walker |  | 4833 | 0 | python decl at experiments/data/utils.py:55 |  |  | 0.504 |
| walker |  | 4833 | 0 | python decl at experiments/data/utils.py:90 |  |  | 0.504 |
| walker |  | 4855 | 22 | python decl names surface in experiments/data/formal_language/tasks/modular_arithmetic.py |  |  | 0.504 |
| walker |  | 4958 | 103 | python method sigs in xlstm/components/util.py |  |  | 0.504 |
| walker |  | 4958 | 0 | python method at xlstm/components/util.py:58 |  |  | 0.504 |
| walker |  | 4958 | 0 | python method at xlstm/components/util.py:73 |  |  | 0.504 |
| walker |  | 4965 | 7 | python method at xlstm/components/util.py:61 |  |  | 0.504 |
| walker |  | 4972 | 7 | python method at xlstm/components/util.py:65 |  |  | 0.504 |
| walker |  | 4979 | 7 | python method at xlstm/components/util.py:69 |  |  | 0.504 |
| walker |  | 4994 | 15 | python method at xlstm/components/util.py:46 |  |  | 0.504 |
| walker |  | 5010 | 16 | python method at xlstm/components/util.py:51 |  |  | 0.504 |
| walker |  | 5026 | 16 | python method body at xlstm/components/util.py:73 body 74 |  |  | 0.504 |
| walker |  | 5045 | 19 | python method body at xlstm/components/util.py:58 body 59 |  |  | 0.504 |
| ns | 5075 |  | 382 | sLSTMCell classes — locations of all 4 + dispatcher | 3.12 |  | 0.482 |
| ns | 5363 |  | 288 | components/feedforward.py — config + class + factory | 3.13 |  | 0.470 |
| ns | 5662 |  | 299 | components/conv.py — CausalConv1d config + class + step | 3.14 |  | 0.497 |
| walker |  | 5862 | 817 | README.md section #0 |  |  | 0.518 |
| walker |  | 5865 | 3 | listing of '.github' |  |  | 0.518 |
| walker |  | 5873 | 8 | listing of '.github/workflows' |  |  | 0.518 |
| walker |  | 5900 | 27 | python imports in xlstm/components/ln.py |  |  | 0.518 |
| ns | 5934 |  | 272 | components/ln.py + linear_headwise.py — class signatures | 3.15 |  | 0.509 |
| walker |  | 5937 | 37 | python method at xlstm/utils.py:97 |  |  | 0.510 |
| walker |  | 5957 | 20 | python method body at xlstm/components/conv.py:20 body 21 |  |  | 0.510 |
| walker |  | 5972 | 15 | listing of 'notebooks/xlstm_large' |  |  | 0.517 |
| walker |  | 6000 | 28 | python imports in xlstm/xlstm_large/components.py |  |  | 0.517 |
| ns | 6015 |  | 81 | components/init.py — full file (weight init helpers) | 3.16 |  | 0.515 |
| walker |  | 6028 | 28 | python imports in xlstm/xlstm_large/generate.py |  |  | 0.515 |
| walker |  | 6096 | 68 | python method sigs in xlstm/blocks/slstm/layer.py |  |  | 0.517 |
| walker |  | 6096 | 0 | python method at xlstm/blocks/slstm/layer.py:28 |  |  | 0.517 |
| walker |  | 6096 | 0 | python method at xlstm/blocks/slstm/layer.py:36 |  |  | 0.517 |
| walker |  | 6096 | 0 | python method at xlstm/blocks/slstm/layer.py:84 |  |  | 0.517 |
| walker |  | 6120 | 24 | python method at xlstm/components/conv.py:98 |  |  | 0.517 |
| ns | 6195 |  | 180 | components/util.py — ParameterProxy + helpers | 3.17 |  | 0.513 |
| walker |  | 6232 | 112 | python method sigs in xlstm/blocks/xlstm_block.py |  |  | 0.519 |
| walker |  | 6232 | 0 | python method at xlstm/blocks/xlstm_block.py:27 |  |  | 0.519 |
| walker |  | 6232 | 0 | python method at xlstm/blocks/xlstm_block.py:51 |  |  | 0.519 |
| walker |  | 6232 | 0 | python method at xlstm/blocks/xlstm_block.py:76 |  |  | 0.519 |
| walker |  | 6232 | 0 | python method at xlstm/blocks/xlstm_block.py:82 |  |  | 0.519 |
| walker |  | 6232 | 0 | python method at xlstm/blocks/xlstm_block.py:89 |  |  | 0.519 |
| walker |  | 6292 | 60 | python method at xlstm/components/util.py:34 |  |  | 0.519 |
| walker |  | 6330 | 38 | python decl names surface in xlstm/blocks/mlstm/backends.py |  |  | 0.519 |
| walker |  | 6351 | 21 | python method body at xlstm/components/util.py:61 body 63 |  |  | 0.519 |
| walker |  | 6372 | 21 | python method body at xlstm/components/util.py:69 body 71 |  |  | 0.519 |
| walker |  | 6474 | 102 | python class body at xlstm/blocks/xlstm_block.py:16 |  |  | 0.531 |
| ns | 6487 |  | 292 | xlstm/utils.py — config mixin + optim-group mixin | 3.18 |  | 0.542 |
| walker |  | 6513 | 39 | python decl names surface in experiments/data/formal_language/online_generate.py |  |  | 0.542 |
| walker |  | 6513 | 0 | python decl at experiments/data/formal_language/online_generate.py:9 |  |  | 0.542 |
| walker |  | 6513 | 0 | python decl at experiments/data/formal_language/online_generate.py:29 |  |  | 0.542 |
| walker |  | 6513 | 0 | python decl at experiments/data/formal_language/online_generate.py:41 |  |  | 0.542 |
| walker |  | 6561 | 48 | python decl doc at experiments/data/formal_language/online_generate.py:41 |  |  | 0.542 |
| walker |  | 6684 | 123 | python method sigs in experiments/data/utils.py |  |  | 0.542 |
| walker |  | 6684 | 0 | python method at experiments/data/utils.py:41 |  |  | 0.542 |
| ns | 6691 |  | 204 | xLSTMLargeConfig — all remaining fields (refines 2.7) | 4.1 | 2.7 | 0.535 |
| walker |  | 6693 | 9 | python method at experiments/data/utils.py:85 |  |  | 0.535 |
| walker |  | 6703 | 10 | python method at experiments/data/utils.py:77 |  |  | 0.535 |
| walker |  | 6714 | 11 | python method at experiments/data/utils.py:50 |  |  | 0.535 |
| walker |  | 6726 | 12 | python method at experiments/data/utils.py:46 |  |  | 0.535 |
| walker |  | 6743 | 17 | python method at experiments/data/utils.py:57 |  |  | 0.535 |
| walker |  | 6766 | 23 | python method at experiments/data/utils.py:19 |  |  | 0.535 |
| walker |  | 6773 | 7 | python method body at experiments/data/utils.py:19 body 22 |  |  | 0.535 |
| walker |  | 6797 | 24 | python method at experiments/data/utils.py:29 |  |  | 0.535 |
| walker |  | 6804 | 7 | python method body at experiments/data/utils.py:29 body 32 |  |  | 0.535 |
| walker |  | 6828 | 24 | python method at experiments/data/utils.py:34 |  |  | 0.535 |
| walker |  | 6835 | 7 | python method body at experiments/data/utils.py:34 body 37 |  |  | 0.535 |
| walker |  | 6861 | 26 | python method at experiments/data/utils.py:24 |  |  | 0.535 |
| walker |  | 6868 | 7 | python method body at experiments/data/utils.py:24 body 27 |  |  | 0.535 |
| walker |  | 6879 | 11 | python method body at experiments/data/utils.py:50 body 52 |  |  | 0.535 |
| walker |  | 6891 | 12 | python method body at experiments/data/utils.py:46 body 48 |  |  | 0.535 |
| walker |  | 6910 | 19 | python method body at experiments/data/utils.py:85 body 87 |  |  | 0.535 |
| walker |  | 6932 | 22 | python method body at xlstm/components/util.py:65 body 67 |  |  | 0.535 |
| walker |  | 7008 | 76 | python method sigs in xlstm/blocks/mlstm/cell.py |  |  | 0.542 |
| walker |  | 7008 | 0 | python method at xlstm/blocks/mlstm/cell.py:23 |  |  | 0.542 |
| walker |  | 7008 | 0 | python method at xlstm/blocks/mlstm/cell.py:43 |  |  | 0.542 |
| walker |  | 7008 | 0 | python method at xlstm/blocks/mlstm/cell.py:133 |  |  | 0.542 |
| walker |  | 7025 | 17 | python decl names surface #1 in xlstm/blocks/slstm/cell.py |  |  | 0.542 |
| walker |  | 7025 | 0 | python decl at xlstm/blocks/slstm/cell.py:777 |  |  | 0.542 |
| walker |  | 7037 | 12 | python class body at xlstm/blocks/slstm/cell.py:777 |  |  | 0.542 |
| walker |  | 7066 | 29 | python method sigs #1 in xlstm/blocks/slstm/cell.py |  |  | 0.543 |
| walker |  | 7066 | 0 | python method at xlstm/blocks/slstm/cell.py:780 |  |  | 0.543 |
| ns | 7076 |  | 385 | xlstm_large/model.py — class skeleton (4 classes) | 4.2 |  | 0.530 |
| walker |  | 7147 | 81 | python method sigs in xlstm/blocks/mlstm/layer.py |  |  | 0.533 |
| walker |  | 7147 | 0 | python method at xlstm/blocks/mlstm/layer.py:34 |  |  | 0.533 |
| walker |  | 7147 | 0 | python method at xlstm/blocks/mlstm/layer.py:42 |  |  | 0.533 |
| walker |  | 7147 | 0 | python method at xlstm/blocks/mlstm/layer.py:101 |  |  | 0.533 |
| walker |  | 7155 | 8 | python method at xlstm/blocks/mlstm/layer.py:158 |  |  | 0.533 |
| walker |  | 7227 | 72 | python class body at xlstm/blocks/mlstm/block.py:9 |  |  | 0.539 |
| ns | 7248 |  | 172 | xlstm_large/model.py — mLSTMLayerConfig fields | 4.3 |  | 0.533 |
| walker |  | 7263 | 36 | python imports in xlstm/components/linear_headwise.py |  |  | 0.533 |
| walker |  | 7335 | 72 | python method at xlstm/components/ln.py:11 |  |  | 0.542 |
| ns | 7388 |  | 140 | xlstm_large/components.py — soft_cap + Norm classes | 4.4 |  | 0.536 |
| walker |  | 7430 | 95 | python decl doc at xlstm/components/util.py:26 |  |  | 0.548 |
| walker |  | 7470 | 40 | python method body at experiments/metrics.py:17 body 18 |  |  | 0.548 |
| walker |  | 7493 | 23 | python imports in xlstm/blocks/mlstm/backends.py |  |  | 0.548 |
| walker |  | 7531 | 38 | python imports in xlstm/components/conv.py |  |  | 0.548 |
| ns | 7648 |  | 260 | xlstm_large/from_pretrained.py — HF checkpoint loader | 4.5 |  | 0.538 |
| walker |  | 7768 | 237 | python class body at xlstm/xlstm_block_stack.py:15 |  |  | 0.566 |
| walker |  | 7847 | 79 | python decl names surface in xlstm/components/feedforward.py |  |  | 0.568 |
| walker |  | 7847 | 0 | python decl at xlstm/components/feedforward.py:22 |  |  | 0.568 |
| walker |  | 7847 | 0 | python decl at xlstm/components/feedforward.py:49 |  |  | 0.568 |
| walker |  | 7847 | 0 | python decl at xlstm/components/feedforward.py:90 |  |  | 0.568 |
| walker |  | 7859 | 12 | python decl at xlstm/components/feedforward.py:31 |  |  | 0.569 |
| walker |  | 7869 | 10 | python class body at xlstm/components/feedforward.py:49 |  |  | 0.569 |
| walker |  | 7937 | 68 | python method sigs in xlstm/components/feedforward.py |  |  | 0.570 |
| walker |  | 7937 | 0 | python method at xlstm/components/feedforward.py:42 |  |  | 0.570 |
| walker |  | 7937 | 0 | python method at xlstm/components/feedforward.py:52 |  |  | 0.570 |
| walker |  | 7937 | 0 | python method at xlstm/components/feedforward.py:72 |  |  | 0.570 |
| walker |  | 7937 | 0 | python method at xlstm/components/feedforward.py:77 |  |  | 0.570 |
| ns | 7975 |  | 327 | xlstm_large/generate.py — generate_tokens + sampling registry | 4.6 |  | 0.558 |
| walker |  | 8040 | 103 | python class body at xlstm/components/feedforward.py:31 |  |  | 0.565 |
| walker |  | 8094 | 54 | python method at xlstm/blocks/slstm/layer.py:92 |  |  | 0.569 |
| walker |  | 8146 | 52 | python decl body at xlstm/components/feedforward.py:90 body 91 |  |  | 0.569 |
| walker |  | 8274 | 128 | python decl at xlstm/xlstm_large/from_pretrained.py:9 |  |  | 0.575 |
| ns | 8293 |  | 318 | xlstm_large/utils.py — single→fused weight conversion | 4.7 |  | 0.564 |
| walker |  | 8361 | 87 | python decl names surface in xlstm/components/init.py |  |  | 0.568 |
| walker |  | 8361 | 0 | python decl at xlstm/components/init.py:8 |  |  | 0.568 |
| walker |  | 8361 | 0 | python decl at xlstm/components/init.py:18 |  |  | 0.568 |
| walker |  | 8361 | 0 | python decl at xlstm/components/init.py:28 |  |  | 0.568 |
| walker |  | 8373 | 12 | python decl doc at xlstm/components/init.py:8 |  |  | 0.568 |
| walker |  | 8403 | 30 | python decl doc at xlstm/components/init.py:28 |  |  | 0.568 |
| walker |  | 8495 | 92 | python decl doc at xlstm/components/init.py:18 |  |  | 0.568 |
| walker |  | 8540 | 45 | python method body at xlstm/xlstm_block_stack.py:117 body 119 |  |  | 0.568 |
| walker |  | 8629 | 89 | python decl names surface in xlstm/xlstm_large/generate.py |  |  | 0.569 |
| walker |  | 8629 | 0 | python decl at xlstm/xlstm_large/generate.py:13 |  |  | 0.569 |
| walker |  | 8629 | 0 | python decl at xlstm/xlstm_large/generate.py:23 |  |  | 0.569 |
| walker |  | 8642 | 13 | python decl body at xlstm/xlstm_large/generate.py:23 body 24 |  |  | 0.569 |
| walker |  | 8657 | 15 | python decl body at xlstm/xlstm_large/generate.py:13 body 15 |  |  | 0.570 |
| walker |  | 8677 | 20 | python decl doc at xlstm/xlstm_large/generate.py:13 |  |  | 0.571 |
| ns | 8698 |  | 405 | experiments/main.py — training entry point | 4.8 |  | 0.556 |
| walker |  | 8708 | 31 | python decl at xlstm/xlstm_large/generate.py:8 |  |  | 0.556 |
| walker |  | 8725 | 17 | python decl at xlstm/xlstm_large/generate.py:18 |  |  | 0.557 |
| walker |  | 8743 | 18 | python imports in experiments/data/formal_language/tasks/cycle_navigation.py |  |  | 0.557 |
| walker |  | 8761 | 18 | python imports in experiments/data/formal_language/tasks/even_pairs.py |  |  | 0.557 |
| walker |  | 8779 | 18 | python imports in experiments/data/formal_language/tasks/modular_arithmetic.py |  |  | 0.557 |
| walker |  | 8797 | 18 | python imports in experiments/data/formal_language/tasks/parity.py |  |  | 0.557 |
| walker |  | 8843 | 46 | python method body at xlstm/xlstm_block_stack.py:111 body 112 |  |  | 0.557 |
| walker |  | 8938 | 95 | python class body at xlstm/blocks/slstm/block.py:12 |  |  | 0.569 |
| walker |  | 8994 | 56 | listing of 'xlstm/blocks/slstm/src/cuda' |  |  | 0.578 |
| walker |  | 9004 | 10 | c decl names surface in xlstm/blocks/slstm/src/cuda/slstm.h |  |  | 0.578 |
| walker |  | 9004 | 0 | c decl at xlstm/blocks/slstm/src/cuda/slstm.h:44 |  |  | 0.578 |
| walker |  | 9015 | 11 | c includes in xlstm/blocks/slstm/src/cuda/slstm.h |  |  | 0.578 |
| ns | 9065 |  | 367 | CUDA bindings — slstm.cc PYBIND11 + sLSTMFunc class | 4.9 |  | 0.567 |
| walker |  | 9263 | 248 | c header banner in xlstm/blocks/slstm/src/cuda/slstm.h |  |  | 0.567 |
| walker |  | 9350 | 87 | python imports in xlstm/xlstm_lm_model.py |  |  | 0.567 |
| ns | 9375 |  | 310 | cuda_init.py — torch.utils.cpp_extension.load wrapper | 4.10 |  | 0.558 |
| walker |  | 9380 | 30 | python imports in experiments/data/formal_language/online_generate.py |  |  | 0.558 |
| walker |  | 9479 | 99 | python decl names surface in xlstm/xlstm_large/components.py |  |  | 0.562 |
| walker |  | 9479 | 0 | python decl at xlstm/xlstm_large/components.py:24 |  |  | 0.562 |
| walker |  | 9479 | 0 | python decl at xlstm/xlstm_large/components.py:70 |  |  | 0.562 |
| walker |  | 9492 | 13 | python decl at xlstm/xlstm_large/components.py:188 |  |  | 0.562 |
| walker |  | 9505 | 13 | python decl at xlstm/xlstm_large/components.py:5 |  |  | 0.562 |
| walker |  | 9537 | 32 | python decl at xlstm/xlstm_large/components.py:155 |  |  | 0.562 |
| walker |  | 9571 | 34 | python decl at xlstm/xlstm_large/components.py:99 |  |  | 0.562 |
| ns | 9663 |  | 288 | vanilla sLSTM/LSTM pointwise + slstm_forward driver | 4.11 |  | 0.557 |
| walker |  | 9745 | 174 | python method sigs in xlstm/xlstm_large/components.py |  |  | 0.559 |
| walker |  | 9745 | 0 | python method at xlstm/xlstm_large/components.py:58 |  |  | 0.559 |
| walker |  | 9760 | 15 | python method at xlstm/xlstm_large/components.py:93 |  |  | 0.559 |
| walker |  | 9775 | 15 | python method at xlstm/xlstm_large/components.py:182 |  |  | 0.559 |
| walker |  | 9791 | 16 | python method at xlstm/xlstm_large/components.py:65 |  |  | 0.562 |
| walker |  | 9801 | 10 | python method body at xlstm/xlstm_large/components.py:65 body 67 |  |  | 0.563 |
| ns | 9841 |  | 178 | tests/ — test fn locations only | 4.12 |  | 0.557 |
| walker |  | 9846 | 45 | python method at xlstm/xlstm_large/components.py:142 |  |  | 0.557 |
| walker |  | 9891 | 45 | python method at xlstm/xlstm_large/components.py:231 |  |  | 0.557 |
| walker |  | 9967 | 76 | python method at xlstm/xlstm_large/components.py:35 |  |  | 0.559 |
| ns | 9989 |  | 148 | experiments support — task fns + dataset/scheduler entry points | 4.13 |  | 0.556 |
