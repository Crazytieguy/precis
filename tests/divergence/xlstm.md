Score(3000)=0.479 I=0.797 C=0.288 ns_rows≤3K=18/68 (reached=6 partial=0 missing=12)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 69 | 69 | listing of '.' |  |  | 1.000 |
| ns | 69 |  | 69 | Repo root listing | 1.1 |  | 1.000 |
| walker |  | 79 | 10 | listing of 'notebooks' |  |  | 1.000 |
| walker |  | 82 | 3 | listing of '.github' |  |  | 1.000 |
| walker |  | 89 | 7 | listing of '.github/workflows' |  |  | 1.000 |
| walker |  | 116 | 27 | listing of 'res' |  |  | 1.000 |
| ns | 128 |  | 59 | pyproject.toml - package name/version/description | 1.2 |  | 0.874 |
| walker |  | 130 | 14 | listing of 'notebooks/xlstm_large' |  |  | 0.875 |
| ns | 147 |  | 19 | README - title line | 1.3 |  | 0.849 |
| walker |  | 168 | 38 | listing of 'xlstm' |  |  | 0.859 |
| walker |  | 189 | 21 | listing of 'xlstm/blocks' |  |  | 0.872 |
| walker |  | 210 | 21 | listing of 'xlstm/blocks/slstm' |  |  | 0.872 |
| walker |  | 232 | 22 | listing of 'xlstm/blocks/mlstm' |  |  | 0.872 |
| walker |  | 254 | 22 | listing of 'xlstm/blocks/slstm/src' |  |  | 0.872 |
| ns | 268 |  | 121 | xlstm/ package-root + subpackage listings | 1.4 |  | 0.627 |
| walker |  | 269 | 15 | listing of 'xlstm/blocks/slstm/src/vanilla' |  |  | 0.627 |
| walker |  | 296 | 27 | listing of 'xlstm/xlstm_large' |  |  | 0.740 |
| walker |  | 328 | 32 | listing of 'xlstm/components' |  |  | 0.918 |
| ns | 345 |  | 77 | README - About paragraph (the xLSTM idea) | 1.5 |  | 0.883 |
| walker |  | 374 | 46 | listing of 'experiments' |  |  | 0.883 |
| walker |  | 388 | 14 | listing of 'experiments/data' |  |  | 0.884 |
| walker |  | 412 | 24 | listing of 'experiments/data/formal_language' |  |  | 0.885 |
| walker |  | 437 | 25 | listing of 'experiments/data/formal_language/tasks' |  |  | 0.886 |
| ns | 469 |  | 124 | README - xLSTM 7B / xLSTM Large callout | 1.6 |  | 0.826 |
| walker |  | 479 | 42 | python imports in xlstm/xlstm_large/__init__.py |  |  | 0.826 |
| walker |  | 582 | 103 | README headline in README.md |  |  | 0.841 |
| walker |  | 614 | 32 | listing of 'notebooks/xlstm' |  |  | 0.841 |
| ns | 651 |  | 182 | xlstm/__init__.py - public package API | 2.1 |  | 0.768 |
| walker |  | 659 | 45 | python decl names surface in xlstm/blocks/slstm/src/vanilla/__init__.py |  |  | 0.768 |
| walker |  | 761 | 102 | python imports in xlstm/blocks/slstm/src/vanilla/__init__.py |  |  | 0.768 |
| walker |  | 798 | 37 | python decl at xlstm/blocks/slstm/src/vanilla/__init__.py:11 |  |  | 0.768 |
| ns | 835 |  | 184 | xlstm_block_stack.py - class/def locations | 2.2 |  | 0.692 |
| walker |  | 980 | 182 | python imports in xlstm/__init__.py |  |  | 0.776 |
| ns | 999 |  | 164 | xlstm_lm_model.py - class/def locations | 2.3 |  | 0.726 |
| ns | 1120 |  | 121 | xlstm/utils.py - class/def locations | 2.4 |  | 0.688 |
| walker |  | 1166 | 186 | headings outline in README.md |  |  | 0.688 |
| walker |  | 1251 | 85 | [package] in pyproject.toml |  |  | 0.730 |
| ns | 1260 |  | 140 | blocks/xlstm_block.py - class/def locations | 2.5 |  | 0.683 |
| walker |  | 1303 | 52 | listing of 'tests' |  |  | 0.684 |
| walker |  | 1490 | 187 | [dependencies] in pyproject.toml |  |  | 0.684 |
| walker |  | 1545 | 55 | listing of 'xlstm/blocks/slstm/src/cuda' |  |  | 0.684 |
| walker |  | 1557 | 12 | c decl names surface in xlstm/blocks/slstm/src/cuda/slstm.h |  |  | 0.684 |
| walker |  | 1557 | 0 | c decl at xlstm/blocks/slstm/src/cuda/slstm.h:44 |  |  | 0.684 |
| walker |  | 1570 | 13 | c includes in xlstm/blocks/slstm/src/cuda/slstm.h |  |  | 0.684 |
| ns | 1650 |  | 390 | blocks/mlstm/{block,layer,cell}.py - class/def locations | 2.6 |  | 0.592 |
| ns | 1732 |  | 82 | blocks/mlstm/backends.py - function locations | 2.7 |  | 0.579 |
| walker |  | 1823 | 253 | c header banner in xlstm/blocks/slstm/src/cuda/slstm.h |  |  | 0.579 |
| ns | 2014 |  | 282 | blocks/slstm/{block,layer}.py - class/def locations | 2.8 |  | 0.529 |
| walker |  | 2027 | 204 | manifest config in pyproject.toml |  |  | 0.529 |
| walker |  | 2041 | 14 | python decl names surface in experiments/metrics.py |  |  | 0.529 |
| walker |  | 2041 | 0 | python decl at experiments/metrics.py:9 |  |  | 0.529 |
| walker |  | 2106 | 65 | python method sigs in experiments/metrics.py |  |  | 0.529 |
| walker |  | 2106 | 0 | python method at experiments/metrics.py:13 |  |  | 0.529 |
| walker |  | 2106 | 0 | python method at experiments/metrics.py:17 |  |  | 0.529 |
| walker |  | 2106 | 0 | python method at experiments/metrics.py:22 |  |  | 0.529 |
| walker |  | 2106 | 0 | python method at experiments/metrics.py:25 |  |  | 0.529 |
| walker |  | 2115 | 9 | python method body at experiments/metrics.py:22 body 23 |  |  | 0.529 |
| walker |  | 2142 | 27 | python decl names surface in xlstm/xlstm_block_stack.py |  |  | 0.529 |
| walker |  | 2142 | 0 | python decl at xlstm/xlstm_block_stack.py:77 |  |  | 0.529 |
| walker |  | 2154 | 12 | python decl at xlstm/xlstm_block_stack.py:15 |  |  | 0.530 |
| walker |  | 2293 | 139 | python method sigs in xlstm/xlstm_block_stack.py |  |  | 0.561 |
| walker |  | 2293 | 0 | python method at xlstm/xlstm_block_stack.py:40 |  |  | 0.561 |
| walker |  | 2293 | 0 | python method at xlstm/xlstm_block_stack.py:52 |  |  | 0.561 |
| walker |  | 2293 | 0 | python method at xlstm/xlstm_block_stack.py:80 |  |  | 0.561 |
| walker |  | 2293 | 0 | python method at xlstm/xlstm_block_stack.py:90 |  |  | 0.561 |
| walker |  | 2293 | 0 | python method at xlstm/xlstm_block_stack.py:111 |  |  | 0.561 |
| walker |  | 2293 | 0 | python method at xlstm/xlstm_block_stack.py:117 |  |  | 0.561 |
| walker |  | 2301 | 8 | python method at xlstm/xlstm_block_stack.py:36 |  |  | 0.561 |
| walker |  | 2316 | 15 | python method body at xlstm/xlstm_block_stack.py:36 body 38 |  |  | 0.561 |
| ns | 2324 |  | 310 | blocks/slstm/cell.py - sLSTMCellConfig + sLSTMCellBase - class/def locations | 2.9 |  | 0.517 |
| walker |  | 2329 | 13 | python class body at xlstm/xlstm_block_stack.py:77 |  |  | 0.517 |
| walker |  | 2385 | 56 | python method at xlstm/xlstm_block_stack.py:126 |  |  | 0.548 |
| walker |  | 2559 | 174 | python decl at xlstm/blocks/slstm/src/vanilla/__init__.py:77 |  |  | 0.548 |
| ns | 2578 |  | 254 | blocks/slstm/cell.py - CUDA/vanilla backend classes - class/def locations | 2.10 |  | 0.520 |
| walker |  | 2735 | 176 | python decl at xlstm/blocks/slstm/src/vanilla/__init__.py:17 |  |  | 0.520 |
| walker |  | 2764 | 29 | python decl names surface in xlstm/utils.py |  |  | 0.521 |
| walker |  | 2764 | 0 | python decl at xlstm/utils.py:32 |  |  | 0.521 |
| walker |  | 2774 | 10 | python decl at xlstm/utils.py:11 |  |  | 0.522 |
| walker |  | 2807 | 33 | python class body at experiments/metrics.py:9 |  |  | 0.522 |
| ns | 2852 |  | 274 | components/{conv,feedforward,linear_headwise}.py - class/def locations | 2.11 |  | 0.492 |
| ns | 2986 |  | 134 | components/{ln,init,util}.py - class/def locations | 2.12 |  | 0.479 |
| walker |  | 3057 | 250 | python class body at xlstm/xlstm_block_stack.py:15 |  |  | 0.479 |
| walker |  | 3122 | 65 | python decl names surface in experiments/main.py |  |  | 0.479 |
| walker |  | 3122 | 0 | python decl at experiments/main.py:32 |  |  | 0.479 |
| walker |  | 3122 | 0 | python decl at experiments/main.py:37 |  |  | 0.479 |
| walker |  | 3139 | 17 | python decl at experiments/main.py:21 |  |  | 0.479 |
| walker |  | 3183 | 44 | python decl at experiments/main.py:25 |  |  | 0.479 |
| walker |  | 3211 | 28 | python decl body at experiments/main.py:32 body 33 |  |  | 0.479 |
| walker |  | 3227 | 16 | python imports in xlstm/xlstm_large/utils.py |  |  | 0.479 |
| ns | 3232 |  | 246 | xlstm_large/{model,components}.py - class/def locations | 2.13 |  | 0.458 |
| walker |  | 3308 | 81 | listing of 'xlstm/blocks/slstm/src/util' |  |  | 0.460 |
| walker |  | 3345 | 37 | c whole header in xlstm/blocks/slstm/src/util/util.h |  |  | 0.460 |
| ns | 3352 |  | 120 | xlstm_large/{generate,from_pretrained,utils}.py - class/def locations + __init__.py | 2.14 |  | 0.454 |
| walker |  | 3474 | 129 | c whole header in xlstm/blocks/slstm/src/util/cuda_error.h |  |  | 0.454 |
| walker |  | 3605 | 131 | c decl names surface in xlstm/blocks/slstm/src/util/support.h |  |  | 0.454 |
| walker |  | 3605 | 0 | c decl at xlstm/blocks/slstm/src/util/support.h:48 |  |  | 0.454 |
| walker |  | 3623 | 18 | c decl at xlstm/blocks/slstm/src/util/support.h:28 |  |  | 0.454 |
| walker |  | 3645 | 22 | c decl at xlstm/blocks/slstm/src/util/support.h:30 |  |  | 0.454 |
| ns | 3663 |  | 311 | README - xLSTM Large quickstart code | 3.1 |  | 0.430 |
| walker |  | 3668 | 23 | c decl at xlstm/blocks/slstm/src/util/support.h:26 |  |  | 0.430 |
| walker |  | 3679 | 11 | c decl body at xlstm/blocks/slstm/src/util/support.h:48 |  |  | 0.430 |
| walker |  | 3713 | 34 | c decl at xlstm/blocks/slstm/src/util/support.h:40 |  |  | 0.430 |
| walker |  | 3739 | 26 | c includes in xlstm/blocks/slstm/src/util/support.h |  |  | 0.430 |
| ns | 3984 |  | 321 | README - non-CUDA hardware recommendation + sLSTM CUDA build env var | 3.2 |  | 0.420 |
| walker |  | 4102 | 363 | c whole header in xlstm/blocks/slstm/src/util/device_assert.h |  |  | 0.420 |
| walker |  | 4188 | 86 | c decl at xlstm/blocks/slstm/src/util/support.h:34 |  |  | 0.420 |
| ns | 4430 |  | 446 | README - xLSTMBlockStack usage code (NeurIPS architecture) | 3.3 |  | 0.389 |
| walker |  | 4595 | 407 | c decl names surface in xlstm/blocks/slstm/src/util/blas.h |  |  | 0.389 |
| walker |  | 4595 | 0 | c decl at xlstm/blocks/slstm/src/util/blas.h:108 |  |  | 0.389 |
| walker |  | 4595 | 0 | c decl at xlstm/blocks/slstm/src/util/blas.h:115 |  |  | 0.389 |
| walker |  | 4595 | 0 | c decl at xlstm/blocks/slstm/src/util/blas.h:122 |  |  | 0.389 |
| walker |  | 4654 | 59 | c decl at xlstm/blocks/slstm/src/util/blas.h:75 |  |  | 0.389 |
| ns | 4712 |  | 282 | README - xLSTMLMModel usage code + yaml/dacite config pattern | 3.4 |  | 0.377 |
| walker |  | 4713 | 59 | c decl at xlstm/blocks/slstm/src/util/blas.h:80 |  |  | 0.377 |
| walker |  | 4772 | 59 | c decl at xlstm/blocks/slstm/src/util/blas.h:85 |  |  | 0.377 |
| ns | 4785 |  | 73 | xLSTMBlockStackConfig.__post_init__ - slstm_at resolution + per-block config propagation | 4.1 | 2.2 | 0.374 |
| walker |  | 4811 | 39 | c includes in xlstm/blocks/slstm/src/util/blas.h |  |  | 0.374 |
| walker |  | 4903 | 92 | c decl at xlstm/blocks/slstm/src/util/blas.h:39 |  |  | 0.374 |
| walker |  | 4994 | 91 | c decl at xlstm/blocks/slstm/src/util/blas.h:90 |  |  | 0.374 |
| ns | 5020 |  | 235 | xLSTMBlockStack._create_blocks - block_map-driven mLSTM/sLSTM dispatch | 4.2 | 2.2 | 0.363 |
| walker |  | 5085 | 91 | c decl at xlstm/blocks/slstm/src/util/blas.h:97 |  |  | 0.363 |
| walker |  | 5181 | 96 | c decl at xlstm/blocks/slstm/src/util/blas.h:60 |  |  | 0.363 |
| ns | 5206 |  | 186 | xLSTMLMModel.__init__ - embedding/lm_head/tie_weights | 4.3 | 2.3 | 0.355 |
| walker |  | 5282 | 101 | c decl at xlstm/blocks/slstm/src/util/blas.h:67 |  |  | 0.355 |
| ns | 5349 |  | 143 | xLSTMLMModel.forward + step | 4.4 | 2.3 | 0.347 |
| walker |  | 5388 | 106 | c decl at xlstm/blocks/slstm/src/util/blas.h:52 |  |  | 0.347 |
| ns | 5565 |  | 216 | xLSTMLMModel._create_weight_decay_optim_groups - embedding weight-decay override | 4.5 | 2.3 | 0.340 |
| walker |  | 5636 | 248 | c header banner in xlstm/blocks/slstm/src/util/blas.h |  |  | 0.340 |
| ns | 5712 |  | 147 | UpProjConfigMixin._set_proj_up_dim - proj-up dimension rounding rule | 4.6 | 2.4 | 0.336 |
| walker |  | 5884 | 248 | c header banner in xlstm/blocks/slstm/src/util/support.h |  |  | 0.336 |
| walker |  | 5919 | 35 | python decl names surface in xlstm/xlstm_lm_model.py |  |  | 0.336 |
| walker |  | 5919 | 0 | python decl at xlstm/xlstm_lm_model.py:22 |  |  | 0.336 |
| walker |  | 5938 | 19 | python decl at xlstm/xlstm_lm_model.py:14 |  |  | 0.337 |
| ns | 5978 |  | 266 | WeightDecayOptimGroupMixin._create_weight_decay_optim_groups - default decay/no-decay heuristic | 4.7 | 2.4 | 0.329 |
| walker |  | 5988 | 50 | python class body at xlstm/xlstm_lm_model.py:14 |  |  | 0.329 |
| ns | 6040 |  | 62 | xLSTMBlockConfig.__post_init__ - mutual-exclusion invariant | 5.1 | 2.5 | 0.328 |
| walker |  | 6089 | 101 | python method sigs in xlstm/xlstm_lm_model.py |  |  | 0.339 |
| walker |  | 6089 | 0 | python method at xlstm/xlstm_lm_model.py:25 |  |  | 0.339 |
| walker |  | 6089 | 0 | python method at xlstm/xlstm_lm_model.py:41 |  |  | 0.339 |
| walker |  | 6089 | 0 | python method at xlstm/xlstm_lm_model.py:49 |  |  | 0.339 |
| walker |  | 6089 | 0 | python method at xlstm/xlstm_lm_model.py:65 |  |  | 0.339 |
| walker |  | 6103 | 14 | python class body at xlstm/xlstm_lm_model.py:22 |  |  | 0.339 |
| ns | 6116 |  | 76 | xLSTMBlock.forward - pre-norm residual pattern | 5.2 | 2.5 | 0.336 |
| walker |  | 6162 | 59 | python method at xlstm/xlstm_lm_model.py:56 |  |  | 0.354 |
| ns | 6217 |  | 101 | mLSTMLayer.__init__ - q/k/v projected via headwise num_proj_heads | 5.3 | 2.6 | 0.350 |
| walker |  | 6251 | 89 | python class body at xlstm/utils.py:11 |  |  | 0.350 |
| walker |  | 6273 | 22 | python method body at experiments/metrics.py:25 body 26 |  |  | 0.350 |
| ns | 6315 |  | 98 | mLSTMCell.forward - backend_fn dispatch call | 5.4 | 2.6 | 0.347 |
| walker |  | 6421 | 148 | python method sigs in xlstm/utils.py |  |  | 0.367 |
| walker |  | 6421 | 0 | python method at xlstm/utils.py:20 |  |  | 0.367 |
| walker |  | 6421 | 0 | python method at xlstm/utils.py:33 |  |  | 0.367 |
| walker |  | 6421 | 0 | python method at xlstm/utils.py:36 |  |  | 0.367 |
| walker |  | 6421 | 0 | python method at xlstm/utils.py:61 |  |  | 0.367 |
| walker |  | 6421 | 0 | python method at xlstm/utils.py:77 |  |  | 0.367 |
| walker |  | 6431 | 10 | python method body at xlstm/utils.py:33 body 34 |  |  | 0.367 |
| walker |  | 6452 | 21 | python method doc at xlstm/xlstm_block_stack.py:40 |  |  | 0.367 |
| ns | 6471 |  | 156 | mLSTMCell.step - S==1 assertion + zero-state shapes | 5.5 | 2.6 | 0.364 |
| walker |  | 6492 | 40 | python decl names surface in experiments/lr_scheduler.py |  |  | 0.364 |
| walker |  | 6492 | 0 | python decl at experiments/lr_scheduler.py:9 |  |  | 0.364 |
| walker |  | 6492 | 0 | python decl at experiments/lr_scheduler.py:24 |  |  | 0.364 |
| walker |  | 6620 | 128 | python method sigs in experiments/lr_scheduler.py |  |  | 0.364 |
| walker |  | 6620 | 0 | python method at experiments/lr_scheduler.py:10 |  |  | 0.364 |
| walker |  | 6620 | 0 | python method at experiments/lr_scheduler.py:25 |  |  | 0.364 |
| walker |  | 6620 | 0 | python method at experiments/lr_scheduler.py:47 |  |  | 0.364 |
| walker |  | 6628 | 8 | python method at experiments/lr_scheduler.py:33 |  |  | 0.364 |
| walker |  | 6637 | 9 | python method at experiments/lr_scheduler.py:13 |  |  | 0.364 |
| walker |  | 6646 | 9 | python method at experiments/lr_scheduler.py:18 |  |  | 0.364 |
| walker |  | 6660 | 14 | python method doc at experiments/lr_scheduler.py:18 |  |  | 0.364 |
| walker |  | 6676 | 16 | python method doc at experiments/lr_scheduler.py:13 |  |  | 0.364 |
| walker |  | 6692 | 16 | python method doc at experiments/lr_scheduler.py:47 |  |  | 0.364 |
| walker |  | 6700 | 8 | python method body at experiments/lr_scheduler.py:13 body 16 |  |  | 0.364 |
| ns | 6751 |  | 280 | parallel_stabilized_simple - stabilized log-D-matrix core equation | 5.6 | 2.7 | 0.359 |
| walker |  | 6754 | 54 | python method doc at xlstm/utils.py:36 |  |  | 0.359 |
| walker |  | 6762 | 8 | python method body at experiments/lr_scheduler.py:18 body 21 |  |  | 0.359 |
| walker |  | 6818 | 56 | python method doc at xlstm/utils.py:61 |  |  | 0.359 |
| walker |  | 6857 | 39 | python method at xlstm/utils.py:97 |  |  | 0.359 |
| ns | 6958 |  | 207 | recurrent_step_stabilized_simple - (c,n,m) update rule | 5.7 | 2.7 | 0.355 |
| ns | 7056 |  | 98 | sLSTMBlockConfig + sLSTMLayerConfig.__post_init__ - propagation rules | 6.1 | 2.8 | 0.352 |
| walker |  | 7231 | 374 | README.md section #5 |  |  | 0.361 |
| walker |  | 7254 | 23 | python method body at experiments/metrics.py:13 body 14 |  |  | 0.361 |
| ns | 7277 |  | 221 | sLSTMCellConfig - backend/function/bias_init/dtype knobs | 6.2 | 2.9 | 0.356 |
| walker |  | 7311 | 57 | python imports in experiments/metrics.py |  |  | 0.356 |
| walker |  | 7341 | 30 | python imports in xlstm/xlstm_large/components.py |  |  | 0.356 |
| ns | 7445 |  | 168 | sLSTMCellBase.reset_parameters - powerlaw_blockdependent forget-gate init | 6.3 | 2.9 | 0.351 |
| ns | 7529 |  | 84 | sLSTMCellBase.step/forward - dispatch to backend-specific _impl/_impl_step | 6.4 | 2.9 | 0.348 |
| ns | 7653 |  | 124 | sLSTMCellCUDA JIT cache key + sLSTMCell.__new__ backend factory | 6.5 | 2.10 | 0.344 |
| ns | 7739 |  | 86 | vanilla pointwise gate equations - slstm vs plain lstm variant | 6.6 |  | 0.342 |
| ns | 7828 |  | 89 | cuda_init.py - load() JIT entry point + XLSTM_EXTRA_INCLUDE_PATHS env var | 6.7 |  | 0.341 |
| walker |  | 7868 | 527 | plaintext config setup.cfg |  |  | 0.341 |
| walker |  | 7928 | 60 | python imports in experiments/lr_scheduler.py |  |  | 0.341 |
| walker |  | 7954 | 26 | python decl names surface in xlstm/blocks/xlstm_block.py |  |  | 0.341 |
| walker |  | 7954 | 0 | python decl at xlstm/blocks/xlstm_block.py:43 |  |  | 0.341 |
| walker |  | 7965 | 11 | python decl at xlstm/blocks/xlstm_block.py:16 |  |  | 0.341 |
| ns | 7966 |  | 138 | sLSTM CUDA/C++ source tree listing | 6.8 |  | 0.371 |
| walker |  | 8020 | 55 | python decl doc at xlstm/blocks/xlstm_block.py:43 |  |  | 0.378 |
| ns | 8031 |  | 65 | slstm.h - ForwardPass/BackwardPass/BackwardPassCut class + Run() locations | 6.9 |  | 0.376 |
| walker |  | 8034 | 14 | python class body at xlstm/blocks/xlstm_block.py:43 |  |  | 0.376 |
| ns | 8128 |  | 97 | CausalConv1d.step - rolling-buffer single-step convolution | 7.1 | 2.11 | 0.374 |
| walker |  | 8144 | 110 | python method sigs in xlstm/blocks/xlstm_block.py |  |  | 0.388 |
| walker |  | 8144 | 0 | python method at xlstm/blocks/xlstm_block.py:27 |  |  | 0.388 |
| walker |  | 8144 | 0 | python method at xlstm/blocks/xlstm_block.py:51 |  |  | 0.388 |
| walker |  | 8144 | 0 | python method at xlstm/blocks/xlstm_block.py:76 |  |  | 0.388 |
| walker |  | 8144 | 0 | python method at xlstm/blocks/xlstm_block.py:82 |  |  | 0.388 |
| walker |  | 8144 | 0 | python method at xlstm/blocks/xlstm_block.py:89 |  |  | 0.388 |
| ns | 8242 |  | 114 | LinearHeadwiseExpand.forward - per-head einsum projection | 7.2 | 2.11 | 0.385 |
| walker |  | 8254 | 110 | python class body at xlstm/blocks/xlstm_block.py:16 |  |  | 0.385 |
| walker |  | 8280 | 26 | python decl names surface in xlstm/components/linear_headwise.py |  |  | 0.385 |
| walker |  | 8280 | 0 | python decl at xlstm/components/linear_headwise.py:42 |  |  | 0.385 |
| ns | 8280 |  | 38 | init.py - small_init_ / wang_init_ std-dev formulas | 7.3 | 2.12 | 0.385 |
| walker |  | 8291 | 11 | python decl at xlstm/components/linear_headwise.py:12 |  |  | 0.385 |
| walker |  | 8346 | 55 | python decl doc at xlstm/components/linear_headwise.py:42 |  |  | 0.385 |
| ns | 8425 |  | 145 | xLSTMLargeConfig - chunkwise_kernel/mode/weight_mode doc'd fields | 8.1 | 2.13 | 0.382 |
| walker |  | 8428 | 82 | python method sigs in xlstm/components/linear_headwise.py |  |  | 0.385 |
| walker |  | 8428 | 0 | python method at xlstm/components/linear_headwise.py:31 |  |  | 0.385 |
| walker |  | 8428 | 0 | python method at xlstm/components/linear_headwise.py:49 |  |  | 0.385 |
| walker |  | 8428 | 0 | python method at xlstm/components/linear_headwise.py:75 |  |  | 0.385 |
| walker |  | 8428 | 0 | python method at xlstm/components/linear_headwise.py:84 |  |  | 0.385 |
| walker |  | 8437 | 9 | python method at xlstm/components/linear_headwise.py:67 |  |  | 0.385 |
| walker |  | 8449 | 12 | python class body at xlstm/components/linear_headwise.py:42 |  |  | 0.385 |
| ns | 8542 |  | 117 | xLSTMLarge.forward + mLSTMLayer.forward - soft_cap gate/logit capping | 8.2 | 2.13 | 0.382 |
| ns | 8578 |  | 36 | components.py - soft_cap formula | 8.3 | 2.13 | 0.381 |
| walker |  | 8679 | 230 | python class body at xlstm/components/linear_headwise.py:12 |  |  | 0.381 |
| walker |  | 8714 | 35 | python imports in xlstm/xlstm_large/generate.py |  |  | 0.381 |
| walker |  | 8753 | 39 | python decl names surface in xlstm/components/conv.py |  |  | 0.383 |
| walker |  | 8753 | 0 | python decl at xlstm/components/conv.py:55 |  |  | 0.383 |
| walker |  | 8765 | 12 | python decl at xlstm/components/conv.py:12 |  |  | 0.385 |
| walker |  | 8828 | 63 | python decl at xlstm/components/conv.py:24 |  |  | 0.385 |
| ns | 8836 |  | 258 | from_pretrained.py - sharded safetensors loading + forced single weight_mode | 8.4 | 2.14 | 0.379 |
| walker |  | 8915 | 87 | python method sigs in xlstm/components/conv.py |  |  | 0.387 |
| walker |  | 8915 | 0 | python method at xlstm/components/conv.py:20 |  |  | 0.387 |
| walker |  | 8915 | 0 | python method at xlstm/components/conv.py:71 |  |  | 0.387 |
| walker |  | 8915 | 0 | python method at xlstm/components/conv.py:95 |  |  | 0.387 |
| walker |  | 8923 | 8 | python method body at xlstm/components/conv.py:95 body 96 |  |  | 0.387 |
| ns | 8948 |  | 112 | experiments/ + data/formal_language/{,tasks}/ directory listings | 9.1 |  | 0.409 |
| walker |  | 8973 | 50 | python method at xlstm/components/conv.py:131 |  |  | 0.409 |
| ns | 9041 |  | 93 | main.py - CLI entry point + dataset registry | 9.2 |  | 0.408 |
| walker |  | 9042 | 69 | python class body at xlstm/components/conv.py:12 |  |  | 0.408 |
| walker |  | 9098 | 56 | python method at xlstm/components/conv.py:110 |  |  | 0.408 |
| walker |  | 9124 | 26 | python method at xlstm/components/conv.py:98 |  |  | 0.408 |
| ns | 9221 |  | 180 | lr_scheduler.py - LinearWarmupCosineAnnealing.compute_lr formula | 9.3 |  | 0.405 |
| walker |  | 9337 | 213 | python class body at xlstm/components/conv.py:55 |  |  | 0.405 |
| ns | 9343 |  | 122 | FormLangDataset.__getitem__ - mask-based causal LM framing | 9.4 |  | 0.402 |
| walker |  | 9351 | 14 | python decl names surface in xlstm/xlstm_large/from_pretrained.py |  |  | 0.403 |
| ns | 9391 |  | 48 | tasks/{parity,cycle_navigation,even_pairs,modular_arithmetic}.py - function locations | 9.5 |  | 0.402 |
| walker |  | 9466 | 115 | python decl at xlstm/xlstm_large/from_pretrained.py:9 |  |  | 0.402 |
| walker |  | 9520 | 54 | python decl names surface in experiments/data/utils.py |  |  | 0.402 |
| walker |  | 9520 | 0 | python decl at experiments/data/utils.py:17 |  |  | 0.402 |
| walker |  | 9520 | 0 | python decl at experiments/data/utils.py:39 |  |  | 0.402 |
| walker |  | 9520 | 0 | python decl at experiments/data/utils.py:55 |  |  | 0.402 |
| walker |  | 9520 | 0 | python decl at experiments/data/utils.py:90 |  |  | 0.402 |
| ns | 9565 |  | 174 | parity_xlstm{01,10,11}.yaml - the mLSTM-only / sLSTM-only / mixed variants | 9.6 |  | 0.397 |
| ns | 9617 |  | 52 | tests/ directory listing | 10.1 |  | 0.403 |
| ns | 9664 |  | 47 | conftest.py - CUDA-required skip-all guard | 10.2 |  | 0.402 |
| walker |  | 9707 | 187 | python method sigs in experiments/data/utils.py |  |  | 0.402 |
| walker |  | 9707 | 0 | python method at experiments/data/utils.py:41 |  |  | 0.402 |
| walker |  | 9713 | 6 | python method at experiments/data/utils.py:57 |  |  | 0.402 |
| walker |  | 9721 | 8 | python method at experiments/data/utils.py:46 |  |  | 0.402 |
| walker |  | 9729 | 8 | python method at experiments/data/utils.py:50 |  |  | 0.402 |
| walker |  | 9737 | 8 | python method at experiments/data/utils.py:77 |  |  | 0.402 |
| walker |  | 9745 | 8 | python method at experiments/data/utils.py:85 |  |  | 0.402 |
| walker |  | 9760 | 15 | python method at experiments/data/utils.py:19 |  |  | 0.402 |
| walker |  | 9777 | 17 | python method at experiments/data/utils.py:24 |  |  | 0.402 |
| walker |  | 9794 | 17 | python method at experiments/data/utils.py:29 |  |  | 0.402 |
| ns | 9799 |  | 135 | Test-function locations across the pytest suite | 10.3 |  | 0.399 |
| walker |  | 9811 | 17 | python method at experiments/data/utils.py:34 |  |  | 0.399 |
| walker |  | 9853 | 42 | python decl names surface in xlstm/components/util.py |  |  | 0.400 |
| walker |  | 9853 | 0 | python decl at xlstm/components/util.py:7 |  |  | 0.400 |
| walker |  | 9853 | 0 | python decl at xlstm/components/util.py:11 |  |  | 0.400 |
| walker |  | 9853 | 0 | python decl at xlstm/components/util.py:26 |  |  | 0.400 |
| ns | 9874 |  | 75 | res/ + notebooks/ directory listings | 11.1 |  | 0.407 |
| walker |  | 9877 | 24 | python decl doc at xlstm/components/util.py:11 |  |  | 0.407 |
| ns | 9882 |  | 8 | .github/workflows/ - no test/lint CI, only CLA + repo-sync | 11.2 |  | 0.409 |
| walker |  | 9895 | 18 | python decl body at xlstm/components/util.py:7 body 8 |  |  | 0.409 |
| ns | 9897 |  | 15 | LICENSE header | 11.3 |  | 0.408 |
| ns | 9971 |  | 74 | pytest.ini - full | 11.4 |  | 0.407 |
| ns | 10006 |  | 35 | README - Citation BibTeX keys | 11.5 |  | 0.406 |
