Score(3000)=0.496 I=0.537 C=0.459 ns_rows≤3K=16/45 (reached=6 partial=0 missing=10)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 69 | 69 | listing of '.' |  |  | 0.000 |
| walker |  | 79 | 10 | listing of 'notebooks' |  |  | 0.000 |
| walker |  | 82 | 3 | listing of '.github' |  |  | 0.000 |
| walker |  | 89 | 7 | listing of '.github/workflows' |  |  | 0.000 |
| ns | 96 |  | 96 | What xLSTM is, from the README lede | 1.1 |  | 0.000 |
| walker |  | 116 | 27 | listing of 'res' |  |  | 0.000 |
| walker |  | 130 | 14 | listing of 'notebooks/xlstm_large' |  |  | 0.000 |
| ns | 165 |  | 69 | Complete repository-root listing | 1.2 |  | 0.594 |
| walker |  | 168 | 38 | listing of 'xlstm' |  |  | 0.608 |
| walker |  | 189 | 21 | listing of 'xlstm/blocks' |  |  | 0.625 |
| walker |  | 210 | 21 | listing of 'xlstm/blocks/slstm' |  |  | 0.633 |
| ns | 229 |  | 64 | README: the 7B model and the name "xLSTM Large" | 1.3 |  | 0.582 |
| walker |  | 232 | 22 | listing of 'xlstm/blocks/mlstm' |  |  | 0.592 |
| walker |  | 254 | 22 | listing of 'xlstm/blocks/slstm/src' |  |  | 0.592 |
| walker |  | 269 | 15 | listing of 'xlstm/blocks/slstm/src/vanilla' |  |  | 0.592 |
| walker |  | 296 | 27 | listing of 'xlstm/xlstm_large' |  |  | 0.621 |
| walker |  | 328 | 32 | listing of 'xlstm/components' |  |  | 0.669 |
| ns | 350 |  | 121 | Complete listings of `xlstm/`, `xlstm/blocks/`, `xlstm/components/`, `xlstm/xlstm_large/` | 1.4 |  | 0.676 |
| walker |  | 374 | 46 | listing of 'experiments' |  |  | 0.679 |
| walker |  | 388 | 14 | listing of 'experiments/data' |  |  | 0.682 |
| walker |  | 412 | 24 | listing of 'experiments/data/formal_language' |  |  | 0.689 |
| walker |  | 437 | 25 | listing of 'experiments/data/formal_language/tasks' |  |  | 0.698 |
| walker |  | 479 | 42 | python imports in xlstm/xlstm_large/__init__.py |  |  | 0.698 |
| walker |  | 511 | 32 | listing of 'notebooks/xlstm' |  |  | 0.704 |
| ns | 532 |  | 182 | `xlstm/__init__.py` in full — version plus the entire public export block | 1.5 |  | 0.634 |
| walker |  | 614 | 103 | README headline in README.md |  |  | 0.647 |
| walker |  | 659 | 45 | python decl names surface in xlstm/blocks/slstm/src/vanilla/__init__.py |  |  | 0.648 |
| ns | 716 |  | 184 | Every README heading (H1/H2/H3), line-located | 1.6 | 1.1 | 0.568 |
| walker |  | 761 | 102 | python imports in xlstm/blocks/slstm/src/vanilla/__init__.py |  |  | 0.568 |
| walker |  | 798 | 37 | python decl at xlstm/blocks/slstm/src/vanilla/__init__.py:11 |  |  | 0.568 |
| ns | 972 |  | 256 | README 75-85: the 7B code is `xlstm/xlstm_large`, standalone on `mlstm_kernels` | 1.7 | 1.6 | 0.521 |
| walker |  | 980 | 182 | python imports in xlstm/__init__.py |  |  | 0.593 |
| walker |  | 1032 | 52 | listing of 'tests' |  |  | 0.606 |
| ns | 1136 |  | 164 | Complete listings of `tests/`, `experiments/` and the `experiments/data/` tree | 1.8 |  | 0.635 |
| walker |  | 1218 | 186 | headings outline in README.md |  |  | 0.714 |
| ns | 1272 |  | 136 | Complete listings of `xlstm/blocks/mlstm/`, `xlstm/blocks/slstm/`, `notebooks/`, `res/`, `.github/workflows/` | 1.9 |  | 0.713 |
| walker |  | 1303 | 85 | [package] in pyproject.toml |  |  | 0.714 |
| walker |  | 1490 | 187 | [dependencies] in pyproject.toml |  |  | 0.714 |
| ns | 1542 |  | 270 | README quickstart: instantiate `xLSTMLargeConfig` + `xLSTMLarge` and run a forward pass | 2.1 |  | 0.657 |
| walker |  | 1628 | 138 | package metadata in pyproject.toml |  |  | 0.657 |
| ns | 1661 |  | 119 | Every top-level class in `xlstm/xlstm_large/model.py`, name + line | 2.2 |  | 0.635 |
| walker |  | 1683 | 55 | listing of 'xlstm/blocks/slstm/src/cuda' |  |  | 0.636 |
| walker |  | 1695 | 12 | c decl names surface in xlstm/blocks/slstm/src/cuda/slstm.h |  |  | 0.636 |
| walker |  | 1695 | 0 | c decl at xlstm/blocks/slstm/src/cuda/slstm.h:44 |  |  | 0.636 |
| walker |  | 1708 | 13 | c includes in xlstm/blocks/slstm/src/cuda/slstm.h |  |  | 0.636 |
| ns | 1836 |  | 175 | `xLSTMLargeConfig` part 1/4 — the four required fields and the norm/bias toggles | 2.3 | 2.2 | 0.606 |
| walker |  | 1961 | 253 | c header banner in xlstm/blocks/slstm/src/cuda/slstm.h |  |  | 0.606 |
| walker |  | 2163 | 202 | manifest config in pyproject.toml |  |  | 0.607 |
| ns | 2209 |  | 373 | `xLSTMLargeConfig` part 2/4 — qk/v dim factors and kernel selection (`chunkwise_kernel`, `sequence_kernel`, `step_kernel`, `mode`) | 2.4 | 2.3 | 0.560 |
| walker |  | 2244 | 81 | listing of 'xlstm/blocks/slstm/src/util' |  |  | 0.562 |
| walker |  | 2281 | 37 | c whole header in xlstm/blocks/slstm/src/util/util.h |  |  | 0.562 |
| walker |  | 2410 | 129 | c whole header in xlstm/blocks/slstm/src/util/cuda_error.h |  |  | 0.562 |
| ns | 2452 |  | 243 | `xLSTMLargeConfig` part 3/4 — chunking, state return, and kernel dtypes | 2.5 | 2.4 | 0.536 |
| walker |  | 2541 | 131 | c decl names surface in xlstm/blocks/slstm/src/util/support.h |  |  | 0.536 |
| walker |  | 2541 | 0 | c decl at xlstm/blocks/slstm/src/util/support.h:48 |  |  | 0.536 |
| walker |  | 2559 | 18 | c decl at xlstm/blocks/slstm/src/util/support.h:28 |  |  | 0.536 |
| walker |  | 2581 | 22 | c decl at xlstm/blocks/slstm/src/util/support.h:30 |  |  | 0.536 |
| walker |  | 2604 | 23 | c decl at xlstm/blocks/slstm/src/util/support.h:26 |  |  | 0.536 |
| walker |  | 2615 | 11 | c decl body at xlstm/blocks/slstm/src/util/support.h:48 |  |  | 0.536 |
| walker |  | 2649 | 34 | c decl at xlstm/blocks/slstm/src/util/support.h:40 |  |  | 0.536 |
| walker |  | 2675 | 26 | c includes in xlstm/blocks/slstm/src/util/support.h |  |  | 0.536 |
| ns | 2692 |  | 240 | `xLSTMLargeConfig` part 4/4 — feedforward sizing, soft caps, `weight_mode` | 2.6 | 2.5 | 0.516 |
| ns | 2885 |  | 193 | `xlstm/xlstm_large/model.py` header: the hard `mlstm_kernels` dependency and the state type aliases | 2.7 |  | 0.496 |
| ns | 3032 |  | 147 | `xLSTMLarge.__init__` — embedding, backbone, lm_head | 2.8 | 2.2 | 0.483 |
| walker |  | 3038 | 363 | c whole header in xlstm/blocks/slstm/src/util/device_assert.h |  |  | 0.483 |
| walker |  | 3124 | 86 | c decl at xlstm/blocks/slstm/src/util/support.h:34 |  |  | 0.483 |
| ns | 3263 |  | 231 | `xLSTMLarge.forward` — signature, shape assert, `soft_cap`, and the conditional state return | 2.9 | 2.8 | 0.463 |
| ns | 3411 |  | 148 | `xLSTMLarge.generate` — signature and delegation to `generate_tokens` | 2.10 | 2.8 | 0.451 |
| walker |  | 3531 | 407 | c decl names surface in xlstm/blocks/slstm/src/util/blas.h |  |  | 0.451 |
| walker |  | 3531 | 0 | c decl at xlstm/blocks/slstm/src/util/blas.h:108 |  |  | 0.451 |
| walker |  | 3531 | 0 | c decl at xlstm/blocks/slstm/src/util/blas.h:115 |  |  | 0.451 |
| walker |  | 3531 | 0 | c decl at xlstm/blocks/slstm/src/util/blas.h:122 |  |  | 0.451 |
| walker |  | 3590 | 59 | c decl at xlstm/blocks/slstm/src/util/blas.h:75 |  |  | 0.451 |
| ns | 3627 |  | 216 | `xLSTMLargeBlockStack.__init__` — the `mLSTMBlock` list and the `add_out_norm` switch | 2.11 | 2.2 | 0.434 |
| walker |  | 3649 | 59 | c decl at xlstm/blocks/slstm/src/util/blas.h:80 |  |  | 0.434 |
| walker |  | 3708 | 59 | c decl at xlstm/blocks/slstm/src/util/blas.h:85 |  |  | 0.434 |
| walker |  | 3747 | 39 | c includes in xlstm/blocks/slstm/src/util/blas.h |  |  | 0.434 |
| walker |  | 3839 | 92 | c decl at xlstm/blocks/slstm/src/util/blas.h:39 |  |  | 0.434 |
| ns | 3888 |  | 261 | `xLSTMLargeBlockStack.forward` — the in-place per-layer state update | 2.12 | 2.11 | 0.418 |
| walker |  | 3930 | 91 | c decl at xlstm/blocks/slstm/src/util/blas.h:90 |  |  | 0.418 |
| walker |  | 4021 | 91 | c decl at xlstm/blocks/slstm/src/util/blas.h:97 |  |  | 0.418 |
| walker |  | 4117 | 96 | c decl at xlstm/blocks/slstm/src/util/blas.h:60 |  |  | 0.418 |
| walker |  | 4218 | 101 | c decl at xlstm/blocks/slstm/src/util/blas.h:67 |  |  | 0.418 |
| ns | 4254 |  | 366 | `mLSTMBlock.__init__` (xlstm_large) — the flat-config to nested-config mapping | 2.13 | 2.2 | 0.397 |
| walker |  | 4324 | 106 | c decl at xlstm/blocks/slstm/src/util/blas.h:52 |  |  | 0.397 |
| ns | 4415 |  | 161 | `mLSTMBlock.forward` (xlstm_large) — pre-norm + residual around layer and FFN | 2.14 | 2.13 | 0.390 |
| walker |  | 4572 | 248 | c header banner in xlstm/blocks/slstm/src/util/blas.h |  |  | 0.390 |
| walker |  | 4820 | 248 | c header banner in xlstm/blocks/slstm/src/util/support.h |  |  | 0.390 |
| walker |  | 4834 | 14 | python decl names surface in experiments/metrics.py |  |  | 0.390 |
| walker |  | 4834 | 0 | python decl at experiments/metrics.py:9 |  |  | 0.390 |
| ns | 4868 |  | 453 | Roster: every top-level class/function of the NeurIPS models and blocks, name + line | 3.1 |  | 0.374 |
| walker |  | 4899 | 65 | python method sigs in experiments/metrics.py |  |  | 0.374 |
| walker |  | 4899 | 0 | python method at experiments/metrics.py:13 |  |  | 0.374 |
| walker |  | 4899 | 0 | python method at experiments/metrics.py:17 |  |  | 0.374 |
| walker |  | 4899 | 0 | python method at experiments/metrics.py:22 |  |  | 0.374 |
| walker |  | 4899 | 0 | python method at experiments/metrics.py:25 |  |  | 0.374 |
| walker |  | 4908 | 9 | python method body at experiments/metrics.py:22 body 23 |  |  | 0.374 |
| walker |  | 4935 | 27 | python decl names surface in xlstm/xlstm_block_stack.py |  |  | 0.374 |
| walker |  | 4935 | 0 | python decl at xlstm/xlstm_block_stack.py:77 |  |  | 0.374 |
| walker |  | 4947 | 12 | python decl at xlstm/xlstm_block_stack.py:15 |  |  | 0.374 |
| walker |  | 5086 | 139 | python method sigs in xlstm/xlstm_block_stack.py |  |  | 0.374 |
| walker |  | 5086 | 0 | python method at xlstm/xlstm_block_stack.py:40 |  |  | 0.374 |
| walker |  | 5086 | 0 | python method at xlstm/xlstm_block_stack.py:52 |  |  | 0.374 |
| walker |  | 5086 | 0 | python method at xlstm/xlstm_block_stack.py:80 |  |  | 0.374 |
| walker |  | 5086 | 0 | python method at xlstm/xlstm_block_stack.py:90 |  |  | 0.374 |
| walker |  | 5086 | 0 | python method at xlstm/xlstm_block_stack.py:111 |  |  | 0.374 |
| walker |  | 5086 | 0 | python method at xlstm/xlstm_block_stack.py:117 |  |  | 0.374 |
| walker |  | 5094 | 8 | python method at xlstm/xlstm_block_stack.py:36 |  |  | 0.374 |
| walker |  | 5109 | 15 | python method body at xlstm/xlstm_block_stack.py:36 body 38 |  |  | 0.374 |
| walker |  | 5122 | 13 | python class body at xlstm/xlstm_block_stack.py:77 |  |  | 0.374 |
| walker |  | 5178 | 56 | python method at xlstm/xlstm_block_stack.py:126 |  |  | 0.375 |
| ns | 5181 |  | 313 | Roster: every top-level definition in `xlstm/components/` and the sLSTM kernel's Python layer | 3.2 |  | 0.364 |
| walker |  | 5352 | 174 | python decl at xlstm/blocks/slstm/src/vanilla/__init__.py:77 |  |  | 0.364 |
| ns | 5441 |  | 260 | `xLSTMBlockStackConfig` — the complete field set | 3.3 | 3.1 | 0.354 |
| walker |  | 5528 | 176 | python decl at xlstm/blocks/slstm/src/vanilla/__init__.py:17 |  |  | 0.354 |
| walker |  | 5557 | 29 | python decl names surface in xlstm/utils.py |  |  | 0.355 |
| walker |  | 5557 | 0 | python decl at xlstm/utils.py:32 |  |  | 0.355 |
| walker |  | 5567 | 10 | python decl at xlstm/utils.py:11 |  |  | 0.355 |
| walker |  | 5600 | 33 | python class body at experiments/metrics.py:9 |  |  | 0.355 |
| ns | 5640 |  | 199 | `block_map` property and `_create_block_map` — how `slstm_at` becomes a per-position block type | 3.4 | 3.3 | 0.350 |
| walker |  | 5850 | 250 | python class body at xlstm/xlstm_block_stack.py:15 |  |  | 0.381 |
| ns | 5894 |  | 254 | `xLSTMBlockStackConfig.__post_init__` — the config mutation cascade | 3.5 | 3.3 | 0.373 |
| walker |  | 5915 | 65 | python decl names surface in experiments/main.py |  |  | 0.373 |
| walker |  | 5915 | 0 | python decl at experiments/main.py:32 |  |  | 0.373 |
| walker |  | 5915 | 0 | python decl at experiments/main.py:37 |  |  | 0.373 |
| walker |  | 5932 | 17 | python decl at experiments/main.py:21 |  |  | 0.373 |
| walker |  | 5976 | 44 | python decl at experiments/main.py:25 |  |  | 0.373 |
| walker |  | 6004 | 28 | python decl body at experiments/main.py:32 body 33 |  |  | 0.373 |
| walker |  | 6020 | 16 | python imports in xlstm/xlstm_large/utils.py |  |  | 0.373 |
| walker |  | 6055 | 35 | python decl names surface in xlstm/xlstm_lm_model.py |  |  | 0.374 |
| walker |  | 6055 | 0 | python decl at xlstm/xlstm_lm_model.py:22 |  |  | 0.374 |
| walker |  | 6074 | 19 | python decl at xlstm/xlstm_lm_model.py:14 |  |  | 0.374 |
| walker |  | 6124 | 50 | python class body at xlstm/xlstm_lm_model.py:14 |  |  | 0.375 |
| walker |  | 6225 | 101 | python method sigs in xlstm/xlstm_lm_model.py |  |  | 0.375 |
| walker |  | 6225 | 0 | python method at xlstm/xlstm_lm_model.py:25 |  |  | 0.375 |
| walker |  | 6225 | 0 | python method at xlstm/xlstm_lm_model.py:41 |  |  | 0.375 |
| walker |  | 6225 | 0 | python method at xlstm/xlstm_lm_model.py:49 |  |  | 0.375 |
| walker |  | 6225 | 0 | python method at xlstm/xlstm_lm_model.py:65 |  |  | 0.375 |
| walker |  | 6239 | 14 | python class body at xlstm/xlstm_lm_model.py:22 |  |  | 0.375 |
| ns | 6242 |  | 348 | `xLSTMBlockStack.__init__` and `_create_blocks` — block instantiation and `post_blocks_norm` | 3.6 | 3.1 | 0.364 |
| walker |  | 6298 | 59 | python method at xlstm/xlstm_lm_model.py:56 |  |  | 0.364 |
| walker |  | 6387 | 89 | python class body at xlstm/utils.py:11 |  |  | 0.364 |
| walker |  | 6409 | 22 | python method body at experiments/metrics.py:25 body 26 |  |  | 0.364 |
| ns | 6496 |  | 254 | `xLSTMBlockStack.forward` and `step` — the loop and the `block_{i}` state dict | 3.7 | 3.6 | 0.358 |
| walker |  | 6557 | 148 | python method sigs in xlstm/utils.py |  |  | 0.358 |
| walker |  | 6557 | 0 | python method at xlstm/utils.py:20 |  |  | 0.358 |
| walker |  | 6557 | 0 | python method at xlstm/utils.py:33 |  |  | 0.358 |
| walker |  | 6557 | 0 | python method at xlstm/utils.py:36 |  |  | 0.358 |
| walker |  | 6557 | 0 | python method at xlstm/utils.py:61 |  |  | 0.358 |
| walker |  | 6557 | 0 | python method at xlstm/utils.py:77 |  |  | 0.358 |
| walker |  | 6567 | 10 | python method body at xlstm/utils.py:33 body 34 |  |  | 0.358 |
| walker |  | 6588 | 21 | python method doc at xlstm/xlstm_block_stack.py:40 |  |  | 0.359 |
| walker |  | 6628 | 40 | python decl names surface in experiments/lr_scheduler.py |  |  | 0.359 |
| walker |  | 6628 | 0 | python decl at experiments/lr_scheduler.py:9 |  |  | 0.359 |
| walker |  | 6628 | 0 | python decl at experiments/lr_scheduler.py:24 |  |  | 0.359 |
| walker |  | 6756 | 128 | python method sigs in experiments/lr_scheduler.py |  |  | 0.359 |
| walker |  | 6756 | 0 | python method at experiments/lr_scheduler.py:10 |  |  | 0.359 |
| walker |  | 6756 | 0 | python method at experiments/lr_scheduler.py:25 |  |  | 0.359 |
| walker |  | 6756 | 0 | python method at experiments/lr_scheduler.py:47 |  |  | 0.359 |
| walker |  | 6764 | 8 | python method at experiments/lr_scheduler.py:33 |  |  | 0.359 |
| walker |  | 6773 | 9 | python method at experiments/lr_scheduler.py:13 |  |  | 0.359 |
| walker |  | 6782 | 9 | python method at experiments/lr_scheduler.py:18 |  |  | 0.359 |
| walker |  | 6796 | 14 | python method doc at experiments/lr_scheduler.py:18 |  |  | 0.359 |
| walker |  | 6812 | 16 | python method doc at experiments/lr_scheduler.py:13 |  |  | 0.359 |
| walker |  | 6828 | 16 | python method doc at experiments/lr_scheduler.py:47 |  |  | 0.356 |
| ns | 6828 |  | 332 | `xLSTMLMModelConfig` fields, `xLSTMLMModel.__init__`, and the model's complete method roster | 3.8 | 3.1 | 0.356 |
| walker |  | 6836 | 8 | python method body at experiments/lr_scheduler.py:13 body 16 |  |  | 0.356 |
| walker |  | 6890 | 54 | python method doc at xlstm/utils.py:36 |  |  | 0.356 |
| walker |  | 6898 | 8 | python method body at experiments/lr_scheduler.py:18 body 21 |  |  | 0.356 |
| walker |  | 6954 | 56 | python method doc at xlstm/utils.py:61 |  |  | 0.356 |
| walker |  | 6993 | 39 | python method at xlstm/utils.py:97 |  |  | 0.356 |
| ns | 7054 |  | 226 | `xLSTMBlockConfig` — the mLSTM-xor-sLSTM invariant | 3.9 | 3.1 | 0.350 |
| walker |  | 7367 | 374 | README.md section #5 |  |  | 0.350 |
| walker |  | 7390 | 23 | python method body at experiments/metrics.py:13 body 14 |  |  | 0.350 |
| ns | 7417 |  | 363 | `xLSTMBlock` — class docstring, `__init__` dispatch, and its complete method roster | 3.10 | 3.1 | 0.340 |
| walker |  | 7464 | 74 | declaration surface of pytest.ini |  |  | 0.340 |
| walker |  | 7521 | 57 | python imports in experiments/metrics.py |  |  | 0.340 |
| walker |  | 7551 | 30 | python imports in xlstm/xlstm_large/components.py |  |  | 0.340 |
| ns | 7668 |  | 251 | Roster: every definition in `xlstm/xlstm_large/`'s support modules | 4.1 |  | 0.334 |
| ns | 7848 |  | 180 | `mLSTMLayerConfig` (NeurIPS) — `conv1d_kernel_size`, `qkv_proj_blocksize`, `num_heads`, `proj_factor` | 4.2 |  | 0.329 |
| walker |  | 8078 | 527 | plaintext config setup.cfg |  |  | 0.329 |
| walker |  | 8138 | 60 | python imports in experiments/lr_scheduler.py |  |  | 0.329 |
| ns | 8163 |  | 315 | `sLSTMLayerConfig` fields, and the head of `sLSTMCellConfig` (`backend`, `bias_init`, `num_states`) | 4.3 | 3.1 | 0.323 |
| walker |  | 8164 | 26 | python decl names surface in xlstm/blocks/xlstm_block.py |  |  | 0.324 |
| walker |  | 8164 | 0 | python decl at xlstm/blocks/xlstm_block.py:43 |  |  | 0.324 |
| walker |  | 8175 | 11 | python decl at xlstm/blocks/xlstm_block.py:16 |  |  | 0.324 |
| walker |  | 8230 | 55 | python decl doc at xlstm/blocks/xlstm_block.py:43 |  |  | 0.325 |
| walker |  | 8244 | 14 | python class body at xlstm/blocks/xlstm_block.py:43 |  |  | 0.325 |
| walker |  | 8354 | 110 | python method sigs in xlstm/blocks/xlstm_block.py |  |  | 0.328 |
| walker |  | 8354 | 0 | python method at xlstm/blocks/xlstm_block.py:27 |  |  | 0.328 |
| walker |  | 8354 | 0 | python method at xlstm/blocks/xlstm_block.py:51 |  |  | 0.328 |
| walker |  | 8354 | 0 | python method at xlstm/blocks/xlstm_block.py:76 |  |  | 0.328 |
| walker |  | 8354 | 0 | python method at xlstm/blocks/xlstm_block.py:82 |  |  | 0.328 |
| walker |  | 8354 | 0 | python method at xlstm/blocks/xlstm_block.py:89 |  |  | 0.328 |
| ns | 8394 |  | 231 | sLSTM backend dispatch: `sLSTMCell.__new__`, and the runtime CUDA build in `sLSTMCellCUDA.instance` | 4.4 | 3.1 | 0.323 |
| walker |  | 8464 | 110 | python class body at xlstm/blocks/xlstm_block.py:16 |  |  | 0.329 |
| walker |  | 8490 | 26 | python decl names surface in xlstm/components/linear_headwise.py |  |  | 0.330 |
| walker |  | 8490 | 0 | python decl at xlstm/components/linear_headwise.py:42 |  |  | 0.330 |
| walker |  | 8501 | 11 | python decl at xlstm/components/linear_headwise.py:12 |  |  | 0.330 |
| walker |  | 8556 | 55 | python decl doc at xlstm/components/linear_headwise.py:42 |  |  | 0.330 |
| ns | 8622 |  | 228 | `_act_fn_registry` and `FeedForwardConfig` — the allowed `act_fn` values | 4.5 |  | 0.325 |
| walker |  | 8638 | 82 | python method sigs in xlstm/components/linear_headwise.py |  |  | 0.325 |
| walker |  | 8638 | 0 | python method at xlstm/components/linear_headwise.py:31 |  |  | 0.325 |
| walker |  | 8638 | 0 | python method at xlstm/components/linear_headwise.py:49 |  |  | 0.325 |
| walker |  | 8638 | 0 | python method at xlstm/components/linear_headwise.py:75 |  |  | 0.325 |
| walker |  | 8638 | 0 | python method at xlstm/components/linear_headwise.py:84 |  |  | 0.325 |
| walker |  | 8647 | 9 | python method at xlstm/components/linear_headwise.py:67 |  |  | 0.325 |
| walker |  | 8659 | 12 | python class body at xlstm/components/linear_headwise.py:42 |  |  | 0.325 |
| ns | 8798 |  | 176 | Complete listings of the sLSTM kernel tree: `src/`, `src/cuda/`, `src/util/`, `src/vanilla/` | 4.6 |  | 0.346 |
| walker |  | 8889 | 230 | python class body at xlstm/components/linear_headwise.py:12 |  |  | 0.346 |
| walker |  | 8924 | 35 | python imports in xlstm/xlstm_large/generate.py |  |  | 0.346 |
| ns | 8962 |  | 164 | README: the three commands that run the parity experiments | 5.1 | 1.6 | 0.344 |
| walker |  | 8963 | 39 | python decl names surface in xlstm/components/conv.py |  |  | 0.345 |
| walker |  | 8963 | 0 | python decl at xlstm/components/conv.py:55 |  |  | 0.345 |
| walker |  | 8975 | 12 | python decl at xlstm/components/conv.py:12 |  |  | 0.346 |
| walker |  | 9038 | 63 | python decl at xlstm/components/conv.py:24 |  |  | 0.346 |
| walker |  | 9125 | 87 | python method sigs in xlstm/components/conv.py |  |  | 0.346 |
| walker |  | 9125 | 0 | python method at xlstm/components/conv.py:20 |  |  | 0.346 |
| walker |  | 9125 | 0 | python method at xlstm/components/conv.py:71 |  |  | 0.346 |
| walker |  | 9125 | 0 | python method at xlstm/components/conv.py:95 |  |  | 0.346 |
| walker |  | 9133 | 8 | python method body at xlstm/components/conv.py:95 body 96 |  |  | 0.342 |
| ns | 9133 |  | 171 | `experiments/main.py` — the dataset registry and the `__main__` config entry point | 5.2 |  | 0.342 |
| walker |  | 9183 | 50 | python method at xlstm/components/conv.py:131 |  |  | 0.342 |
| walker |  | 9252 | 69 | python class body at xlstm/components/conv.py:12 |  |  | 0.342 |
| walker |  | 9308 | 56 | python method at xlstm/components/conv.py:110 |  |  | 0.342 |
| walker |  | 9334 | 26 | python method at xlstm/components/conv.py:98 |  |  | 0.342 |
| ns | 9434 |  | 301 | `experiments/parity_xlstm01.yaml` — the training and model sections of a real config | 5.3 |  | 0.336 |
| walker |  | 9547 | 213 | python class body at xlstm/components/conv.py:55 |  |  | 0.336 |
| walker |  | 9561 | 14 | python decl names surface in xlstm/xlstm_large/from_pretrained.py |  |  | 0.336 |
| ns | 9647 |  | 213 | `tests/conftest.py` in full, plus every test function in the suite | 5.4 |  | 0.332 |
| walker |  | 9676 | 115 | python decl at xlstm/xlstm_large/from_pretrained.py:9 |  |  | 0.332 |
| walker |  | 9730 | 54 | python decl names surface in experiments/data/utils.py |  |  | 0.332 |
| walker |  | 9730 | 0 | python decl at experiments/data/utils.py:17 |  |  | 0.332 |
| walker |  | 9730 | 0 | python decl at experiments/data/utils.py:39 |  |  | 0.332 |
| walker |  | 9730 | 0 | python decl at experiments/data/utils.py:55 |  |  | 0.332 |
| walker |  | 9730 | 0 | python decl at experiments/data/utils.py:90 |  |  | 0.332 |
| ns | 9831 |  | 184 | `pyproject.toml` — project metadata, the dependency list marker, and package data | 6.1 |  | 0.341 |
| walker |  | 9917 | 187 | python method sigs in experiments/data/utils.py |  |  | 0.341 |
| walker |  | 9917 | 0 | python method at experiments/data/utils.py:41 |  |  | 0.341 |
| walker |  | 9923 | 6 | python method at experiments/data/utils.py:57 |  |  | 0.341 |
| walker |  | 9931 | 8 | python method at experiments/data/utils.py:46 |  |  | 0.341 |
| walker |  | 9939 | 8 | python method at experiments/data/utils.py:50 |  |  | 0.341 |
| walker |  | 9947 | 8 | python method at experiments/data/utils.py:77 |  |  | 0.341 |
| walker |  | 9955 | 8 | python method at experiments/data/utils.py:85 |  |  | 0.341 |
| walker |  | 9970 | 15 | python method at experiments/data/utils.py:19 |  |  | 0.341 |
| walker |  | 9987 | 17 | python method at experiments/data/utils.py:24 |  |  | 0.341 |
| ns | 9999 |  | 168 | `pytest.ini` in full, and the README install commands | 6.2 | 1.6 | 0.340 |
