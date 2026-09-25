Score(3000)=0.497 I=0.538 C=0.459 ns_rows≤3K=16/45 grid(1000/1442/2080/3000/4327/6240/9000)=0.593/0.714/0.610/0.497/0.401/0.384/0.354

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 65 | 65 | listing of '.' |  |  | 0.000 |
| walker |  | 74 | 9 | listing of 'notebooks' |  |  | 0.000 |
| walker |  | 77 | 3 | listing of '.github' |  |  | 0.000 |
| walker |  | 85 | 8 | listing of '.github/workflows' |  |  | 0.000 |
| ns | 96 |  | 96 | What xLSTM is, from the README lede | 1.1 |  | 0.000 |
| walker |  | 113 | 28 | listing of 'res' |  |  | 0.000 |
| walker |  | 128 | 15 | listing of 'notebooks/xlstm_large' |  |  | 0.000 |
| ns | 161 |  | 65 | Complete repository-root listing | 1.2 |  | 0.594 |
| walker |  | 164 | 36 | listing of 'xlstm' |  |  | 0.608 |
| walker |  | 186 | 22 | listing of 'xlstm/blocks' |  |  | 0.625 |
| walker |  | 209 | 23 | listing of 'xlstm/blocks/mlstm' |  |  | 0.633 |
| ns | 225 |  | 64 | README: the 7B model and the name "xLSTM Large" | 1.3 |  | 0.582 |
| walker |  | 232 | 23 | listing of 'xlstm/blocks/slstm' |  |  | 0.592 |
| walker |  | 254 | 22 | listing of 'xlstm/blocks/slstm/src' |  |  | 0.592 |
| walker |  | 270 | 16 | listing of 'xlstm/blocks/slstm/src/vanilla' |  |  | 0.592 |
| walker |  | 298 | 28 | listing of 'xlstm/xlstm_large' |  |  | 0.621 |
| walker |  | 333 | 35 | listing of 'xlstm/components' |  |  | 0.669 |
| ns | 346 |  | 121 | Complete listings of `xlstm/`, `xlstm/blocks/`, `xlstm/components/`, `xlstm/xlstm_large/` | 1.4 |  | 0.676 |
| walker |  | 379 | 46 | listing of 'experiments' |  |  | 0.679 |
| walker |  | 395 | 16 | listing of 'experiments/data' |  |  | 0.682 |
| walker |  | 421 | 26 | listing of 'experiments/data/formal_language' |  |  | 0.689 |
| walker |  | 449 | 28 | listing of 'experiments/data/formal_language/tasks' |  |  | 0.698 |
| walker |  | 491 | 42 | python imports in xlstm/xlstm_large/__init__.py |  |  | 0.698 |
| walker |  | 524 | 33 | listing of 'notebooks/xlstm' |  |  | 0.704 |
| ns | 528 |  | 182 | `xlstm/__init__.py` in full — version plus the entire public export block | 1.5 |  | 0.634 |
| walker |  | 627 | 103 | README headline in README.md |  |  | 0.647 |
| walker |  | 672 | 45 | python decl names surface in xlstm/blocks/slstm/src/vanilla/__init__.py |  |  | 0.648 |
| ns | 712 |  | 184 | Every README heading (H1/H2/H3), line-located | 1.6 | 1.1 | 0.568 |
| walker |  | 774 | 102 | python imports in xlstm/blocks/slstm/src/vanilla/__init__.py |  |  | 0.568 |
| walker |  | 811 | 37 | python decl at xlstm/blocks/slstm/src/vanilla/__init__.py:11 |  |  | 0.568 |
| ns | 968 |  | 256 | README 75-85: the 7B code is `xlstm/xlstm_large`, standalone on `mlstm_kernels` | 1.7 | 1.6 | 0.521 |
| walker |  | 993 | 182 | python imports in xlstm/__init__.py |  |  | 0.593 |
| ns | 1139 |  | 171 | Complete listings of `tests/`, `experiments/` and the `experiments/data/` tree | 1.8 |  | 0.570 |
| walker |  | 1179 | 186 | headings outline in README.md |  |  | 0.651 |
| walker |  | 1234 | 55 | listing of 'tests' |  |  | 0.714 |
| ns | 1278 |  | 139 | Complete listings of `xlstm/blocks/mlstm/`, `xlstm/blocks/slstm/`, `notebooks/`, `res/`, `.github/workflows/` | 1.9 |  | 0.713 |
| walker |  | 1319 | 85 | [package] in pyproject.toml |  |  | 0.714 |
| walker |  | 1506 | 187 | [dependencies] in pyproject.toml |  |  | 0.714 |
| ns | 1548 |  | 270 | README quickstart: instantiate `xLSTMLargeConfig` + `xLSTMLarge` and run a forward pass | 2.1 |  | 0.657 |
| walker |  | 1644 | 138 | package metadata in pyproject.toml |  |  | 0.657 |
| ns | 1667 |  | 119 | Every top-level class in `xlstm/xlstm_large/model.py`, name + line | 2.2 |  | 0.635 |
| walker |  | 1700 | 56 | listing of 'xlstm/blocks/slstm/src/cuda' |  |  | 0.636 |
| walker |  | 1712 | 12 | c names xlstm/blocks/slstm/src/cuda/slstm.h |  |  | 0.636 |
| ns | 1842 |  | 175 | `xLSTMLargeConfig` part 1/4 — the four required fields and the norm/bias toggles | 2.3 | 2.2 | 0.606 |
| walker |  | 1914 | 202 | manifest config in pyproject.toml |  |  | 0.607 |
| walker |  | 1996 | 82 | listing of 'xlstm/blocks/slstm/src/util' |  |  | 0.610 |
| walker |  | 2009 | 13 | c names xlstm/blocks/slstm/src/util/util.h |  |  | 0.610 |
| walker |  | 2034 | 25 | c names xlstm/blocks/slstm/src/util/cuda_error.h |  |  | 0.610 |
| walker |  | 2054 | 20 | c decl xlstm/blocks/slstm/src/util/cuda_error.h:9 |  |  | 0.610 |
| walker |  | 2085 | 31 | c module doc xlstm/blocks/slstm/src/util/cuda_error.h |  |  | 0.610 |
| ns | 2215 |  | 373 | `xLSTMLargeConfig` part 2/4 — qk/v dim factors and kernel selection (`chunkwise_kernel`, `sequence_kernel`, `step_kernel`, `mode`) | 2.4 | 2.3 | 0.562 |
| walker |  | 2259 | 174 | python decl at xlstm/blocks/slstm/src/vanilla/__init__.py:77 |  |  | 0.562 |
| walker |  | 2435 | 176 | python decl at xlstm/blocks/slstm/src/vanilla/__init__.py:17 |  |  | 0.562 |
| walker |  | 2449 | 14 | python decl names surface in experiments/metrics.py |  |  | 0.562 |
| walker |  | 2449 | 0 | python decl at experiments/metrics.py:9 |  |  | 0.562 |
| ns | 2458 |  | 243 | `xLSTMLargeConfig` part 3/4 — chunking, state return, and kernel dtypes | 2.5 | 2.4 | 0.536 |
| walker |  | 2514 | 65 | python method sigs in experiments/metrics.py |  |  | 0.536 |
| walker |  | 2514 | 0 | python method at experiments/metrics.py:13 |  |  | 0.536 |
| walker |  | 2514 | 0 | python method at experiments/metrics.py:17 |  |  | 0.536 |
| walker |  | 2514 | 0 | python method at experiments/metrics.py:22 |  |  | 0.536 |
| walker |  | 2514 | 0 | python method at experiments/metrics.py:25 |  |  | 0.536 |
| walker |  | 2523 | 9 | python method body at experiments/metrics.py:22 body 23 |  |  | 0.536 |
| walker |  | 2550 | 27 | python decl names surface in xlstm/xlstm_block_stack.py |  |  | 0.536 |
| walker |  | 2550 | 0 | python decl at xlstm/xlstm_block_stack.py:77 |  |  | 0.536 |
| walker |  | 2562 | 12 | python decl at xlstm/xlstm_block_stack.py:15 |  |  | 0.536 |
| ns | 2698 |  | 240 | `xLSTMLargeConfig` part 4/4 — feedforward sizing, soft caps, `weight_mode` | 2.6 | 2.5 | 0.516 |
| walker |  | 2701 | 139 | python method sigs in xlstm/xlstm_block_stack.py |  |  | 0.516 |
| walker |  | 2701 | 0 | python method at xlstm/xlstm_block_stack.py:40 |  |  | 0.516 |
| walker |  | 2701 | 0 | python method at xlstm/xlstm_block_stack.py:52 |  |  | 0.516 |
| walker |  | 2701 | 0 | python method at xlstm/xlstm_block_stack.py:80 |  |  | 0.516 |
| walker |  | 2701 | 0 | python method at xlstm/xlstm_block_stack.py:90 |  |  | 0.516 |
| walker |  | 2701 | 0 | python method at xlstm/xlstm_block_stack.py:111 |  |  | 0.516 |
| walker |  | 2701 | 0 | python method at xlstm/xlstm_block_stack.py:117 |  |  | 0.516 |
| walker |  | 2709 | 8 | python method at xlstm/xlstm_block_stack.py:36 |  |  | 0.516 |
| walker |  | 2724 | 15 | python method body at xlstm/xlstm_block_stack.py:36 body 38 |  |  | 0.516 |
| walker |  | 2737 | 13 | python class body at xlstm/xlstm_block_stack.py:77 |  |  | 0.516 |
| walker |  | 2793 | 56 | python method at xlstm/xlstm_block_stack.py:126 |  |  | 0.516 |
| walker |  | 2822 | 29 | python decl names surface in xlstm/utils.py |  |  | 0.516 |
| walker |  | 2822 | 0 | python decl at xlstm/utils.py:32 |  |  | 0.516 |
| walker |  | 2832 | 10 | python decl at xlstm/utils.py:11 |  |  | 0.516 |
| walker |  | 2865 | 33 | python class body at experiments/metrics.py:9 |  |  | 0.516 |
| ns | 2891 |  | 193 | `xlstm/xlstm_large/model.py` header: the hard `mlstm_kernels` dependency and the state type aliases | 2.7 |  | 0.497 |
| ns | 3038 |  | 147 | `xLSTMLarge.__init__` — embedding, backbone, lm_head | 2.8 | 2.2 | 0.483 |
| walker |  | 3115 | 250 | python class body at xlstm/xlstm_block_stack.py:15 |  |  | 0.487 |
| walker |  | 3131 | 16 | python imports in xlstm/xlstm_large/utils.py |  |  | 0.487 |
| walker |  | 3220 | 89 | python class body at xlstm/utils.py:11 |  |  | 0.487 |
| ns | 3269 |  | 231 | `xLSTMLarge.forward` — signature, shape assert, `soft_cap`, and the conditional state return | 2.9 | 2.8 | 0.467 |
| walker |  | 3278 | 58 | c names xlstm/blocks/slstm/src/util/device_assert.h |  |  | 0.467 |
| walker |  | 3291 | 13 | c decl xlstm/blocks/slstm/src/util/device_assert.h:23 |  |  | 0.467 |
| walker |  | 3314 | 23 | c decl xlstm/blocks/slstm/src/util/device_assert.h:27 |  |  | 0.467 |
| walker |  | 3336 | 22 | python method body at experiments/metrics.py:25 body 26 |  |  | 0.467 |
| ns | 3417 |  | 148 | `xLSTMLarge.generate` — signature and delegation to `generate_tokens` | 2.10 | 2.8 | 0.454 |
| walker |  | 3484 | 148 | python method sigs in xlstm/utils.py |  |  | 0.454 |
| walker |  | 3484 | 0 | python method at xlstm/utils.py:20 |  |  | 0.454 |
| walker |  | 3484 | 0 | python method at xlstm/utils.py:33 |  |  | 0.454 |
| walker |  | 3484 | 0 | python method at xlstm/utils.py:36 |  |  | 0.454 |
| walker |  | 3484 | 0 | python method at xlstm/utils.py:61 |  |  | 0.454 |
| walker |  | 3484 | 0 | python method at xlstm/utils.py:77 |  |  | 0.454 |
| walker |  | 3494 | 10 | python method body at xlstm/utils.py:33 body 34 |  |  | 0.454 |
| walker |  | 3559 | 65 | python decl names surface in experiments/main.py |  |  | 0.454 |
| walker |  | 3559 | 0 | python decl at experiments/main.py:32 |  |  | 0.454 |
| walker |  | 3559 | 0 | python decl at experiments/main.py:37 |  |  | 0.454 |
| walker |  | 3576 | 17 | python decl at experiments/main.py:21 |  |  | 0.454 |
| walker |  | 3620 | 44 | python decl at experiments/main.py:25 |  |  | 0.454 |
| ns | 3633 |  | 216 | `xLSTMLargeBlockStack.__init__` — the `mLSTMBlock` list and the `add_out_norm` switch | 2.11 | 2.2 | 0.437 |
| walker |  | 3648 | 28 | python decl body at experiments/main.py:32 body 33 |  |  | 0.437 |
| walker |  | 3683 | 35 | python decl names surface in xlstm/xlstm_lm_model.py |  |  | 0.437 |
| walker |  | 3683 | 0 | python decl at xlstm/xlstm_lm_model.py:22 |  |  | 0.437 |
| walker |  | 3702 | 19 | python decl at xlstm/xlstm_lm_model.py:14 |  |  | 0.438 |
| walker |  | 3752 | 50 | python class body at xlstm/xlstm_lm_model.py:14 |  |  | 0.438 |
| walker |  | 3853 | 101 | python method sigs in xlstm/xlstm_lm_model.py |  |  | 0.438 |
| walker |  | 3853 | 0 | python method at xlstm/xlstm_lm_model.py:25 |  |  | 0.438 |
| walker |  | 3853 | 0 | python method at xlstm/xlstm_lm_model.py:41 |  |  | 0.438 |
| walker |  | 3853 | 0 | python method at xlstm/xlstm_lm_model.py:49 |  |  | 0.438 |
| walker |  | 3853 | 0 | python method at xlstm/xlstm_lm_model.py:65 |  |  | 0.438 |
| walker |  | 3867 | 14 | python class body at xlstm/xlstm_lm_model.py:22 |  |  | 0.438 |
| ns | 3894 |  | 261 | `xLSTMLargeBlockStack.forward` — the in-place per-layer state update | 2.12 | 2.11 | 0.423 |
| walker |  | 3926 | 59 | python method at xlstm/xlstm_lm_model.py:56 |  |  | 0.423 |
| walker |  | 3947 | 21 | python method doc at xlstm/xlstm_block_stack.py:40 |  |  | 0.423 |
| walker |  | 3987 | 40 | python decl names surface in experiments/lr_scheduler.py |  |  | 0.423 |
| walker |  | 3987 | 0 | python decl at experiments/lr_scheduler.py:9 |  |  | 0.423 |
| walker |  | 3987 | 0 | python decl at experiments/lr_scheduler.py:24 |  |  | 0.423 |
| walker |  | 4115 | 128 | python method sigs in experiments/lr_scheduler.py |  |  | 0.423 |
| walker |  | 4115 | 0 | python method at experiments/lr_scheduler.py:10 |  |  | 0.423 |
| walker |  | 4115 | 0 | python method at experiments/lr_scheduler.py:25 |  |  | 0.423 |
| walker |  | 4115 | 0 | python method at experiments/lr_scheduler.py:47 |  |  | 0.423 |
| walker |  | 4123 | 8 | python method at experiments/lr_scheduler.py:33 |  |  | 0.423 |
| walker |  | 4132 | 9 | python method at experiments/lr_scheduler.py:13 |  |  | 0.423 |
| walker |  | 4141 | 9 | python method at experiments/lr_scheduler.py:18 |  |  | 0.423 |
| walker |  | 4155 | 14 | python method doc at experiments/lr_scheduler.py:18 |  |  | 0.423 |
| walker |  | 4171 | 16 | python method doc at experiments/lr_scheduler.py:13 |  |  | 0.423 |
| walker |  | 4187 | 16 | python method doc at experiments/lr_scheduler.py:47 |  |  | 0.423 |
| walker |  | 4195 | 8 | python method body at experiments/lr_scheduler.py:13 body 16 |  |  | 0.423 |
| walker |  | 4249 | 54 | python method doc at xlstm/utils.py:36 |  |  | 0.423 |
| ns | 4260 |  | 366 | `mLSTMBlock.__init__` (xlstm_large) — the flat-config to nested-config mapping | 2.13 | 2.2 | 0.401 |
| walker |  | 4404 | 155 | README.md section #2 |  |  | 0.401 |
| ns | 4421 |  | 161 | `mLSTMBlock.forward` (xlstm_large) — pre-norm + residual around layer and FFN | 2.14 | 2.13 | 0.394 |
| walker |  | 4601 | 197 | README.md section #3 |  |  | 0.399 |
| walker |  | 4609 | 8 | python method body at experiments/lr_scheduler.py:18 body 21 |  |  | 0.399 |
| walker |  | 4665 | 56 | python method doc at xlstm/utils.py:61 |  |  | 0.399 |
| walker |  | 4704 | 39 | python method at xlstm/utils.py:97 |  |  | 0.399 |
| walker |  | 4727 | 23 | python method body at experiments/metrics.py:13 body 14 |  |  | 0.399 |
| walker |  | 4801 | 74 | declaration surface of pytest.ini |  |  | 0.399 |
| walker |  | 4858 | 57 | python imports in experiments/metrics.py |  |  | 0.399 |
| ns | 4874 |  | 453 | Roster: every top-level class/function of the NeurIPS models and blocks, name + line | 3.1 |  | 0.386 |
| walker |  | 4888 | 30 | python imports in xlstm/xlstm_large/components.py |  |  | 0.386 |
| ns | 5187 |  | 313 | Roster: every top-level definition in `xlstm/components/` and the sLSTM kernel's Python layer | 3.2 |  | 0.374 |
| walker |  | 5415 | 527 | plaintext config setup.cfg |  |  | 0.374 |
| ns | 5447 |  | 260 | `xLSTMBlockStackConfig` — the complete field set | 3.3 | 3.1 | 0.394 |
| walker |  | 5475 | 60 | python imports in experiments/lr_scheduler.py |  |  | 0.394 |
| walker |  | 5510 | 35 | python imports in xlstm/xlstm_large/generate.py |  |  | 0.394 |
| ns | 5646 |  | 199 | `block_map` property and `_create_block_map` — how `slstm_at` becomes a per-position block type | 3.4 | 3.3 | 0.390 |
| walker |  | 5712 | 202 | README.md section #6 |  |  | 0.390 |
| walker |  | 5879 | 167 | README.md section #7 |  |  | 0.390 |
| ns | 5900 |  | 254 | `xLSTMBlockStackConfig.__post_init__` — the config mutation cascade | 3.5 | 3.3 | 0.381 |
| walker |  | 5905 | 26 | python decl names surface in xlstm/blocks/xlstm_block.py |  |  | 0.382 |
| walker |  | 5905 | 0 | python decl at xlstm/blocks/xlstm_block.py:43 |  |  | 0.382 |
| walker |  | 5916 | 11 | python decl at xlstm/blocks/xlstm_block.py:16 |  |  | 0.383 |
| walker |  | 5971 | 55 | python decl doc at xlstm/blocks/xlstm_block.py:43 |  |  | 0.383 |
| walker |  | 5985 | 14 | python class body at xlstm/blocks/xlstm_block.py:43 |  |  | 0.383 |
| walker |  | 6095 | 110 | python method sigs in xlstm/blocks/xlstm_block.py |  |  | 0.383 |
| walker |  | 6095 | 0 | python method at xlstm/blocks/xlstm_block.py:27 |  |  | 0.383 |
| walker |  | 6095 | 0 | python method at xlstm/blocks/xlstm_block.py:51 |  |  | 0.383 |
| walker |  | 6095 | 0 | python method at xlstm/blocks/xlstm_block.py:76 |  |  | 0.383 |
| walker |  | 6095 | 0 | python method at xlstm/blocks/xlstm_block.py:82 |  |  | 0.383 |
| walker |  | 6095 | 0 | python method at xlstm/blocks/xlstm_block.py:89 |  |  | 0.383 |
| walker |  | 6205 | 110 | python class body at xlstm/blocks/xlstm_block.py:16 |  |  | 0.383 |
| walker |  | 6231 | 26 | python decl names surface in xlstm/components/linear_headwise.py |  |  | 0.384 |
| walker |  | 6231 | 0 | python decl at xlstm/components/linear_headwise.py:42 |  |  | 0.384 |
| walker |  | 6242 | 11 | python decl at xlstm/components/linear_headwise.py:12 |  |  | 0.385 |
| ns | 6248 |  | 348 | `xLSTMBlockStack.__init__` and `_create_blocks` — block instantiation and `post_blocks_norm` | 3.6 | 3.1 | 0.373 |
| walker |  | 6297 | 55 | python decl doc at xlstm/components/linear_headwise.py:42 |  |  | 0.373 |
| walker |  | 6379 | 82 | python method sigs in xlstm/components/linear_headwise.py |  |  | 0.373 |
| walker |  | 6379 | 0 | python method at xlstm/components/linear_headwise.py:31 |  |  | 0.373 |
| walker |  | 6379 | 0 | python method at xlstm/components/linear_headwise.py:49 |  |  | 0.373 |
| walker |  | 6379 | 0 | python method at xlstm/components/linear_headwise.py:75 |  |  | 0.373 |
| walker |  | 6379 | 0 | python method at xlstm/components/linear_headwise.py:84 |  |  | 0.373 |
| walker |  | 6388 | 9 | python method at xlstm/components/linear_headwise.py:67 |  |  | 0.373 |
| walker |  | 6400 | 12 | python class body at xlstm/components/linear_headwise.py:42 |  |  | 0.373 |
| ns | 6502 |  | 254 | `xLSTMBlockStack.forward` and `step` — the loop and the `block_{i}` state dict | 3.7 | 3.6 | 0.367 |
| walker |  | 6630 | 230 | python class body at xlstm/components/linear_headwise.py:12 |  |  | 0.367 |
| walker |  | 6644 | 14 | python decl names surface in xlstm/xlstm_large/from_pretrained.py |  |  | 0.367 |
| walker |  | 6759 | 115 | python decl at xlstm/xlstm_large/from_pretrained.py:9 |  |  | 0.367 |
| ns | 6834 |  | 332 | `xLSTMLMModelConfig` fields, `xLSTMLMModel.__init__`, and the model's complete method roster | 3.8 | 3.1 | 0.363 |
| walker |  | 6890 | 131 | c names xlstm/blocks/slstm/src/util/support.h |  |  | 0.363 |
| walker |  | 6908 | 18 | c decl xlstm/blocks/slstm/src/util/support.h:28 |  |  | 0.363 |
| walker |  | 6930 | 22 | c decl xlstm/blocks/slstm/src/util/support.h:30 |  |  | 0.363 |
| walker |  | 6953 | 23 | c decl xlstm/blocks/slstm/src/util/support.h:26 |  |  | 0.363 |
| walker |  | 6987 | 34 | c decl xlstm/blocks/slstm/src/util/support.h:40 |  |  | 0.363 |
| ns | 7060 |  | 226 | `xLSTMBlockConfig` — the mLSTM-xor-sLSTM invariant | 3.9 | 3.1 | 0.364 |
| walker |  | 7073 | 86 | c decl xlstm/blocks/slstm/src/util/support.h:34 |  |  | 0.364 |
| walker |  | 7112 | 39 | python decl names surface in xlstm/components/conv.py |  |  | 0.365 |
| walker |  | 7112 | 0 | python decl at xlstm/components/conv.py:55 |  |  | 0.365 |
| walker |  | 7124 | 12 | python decl at xlstm/components/conv.py:12 |  |  | 0.366 |
| walker |  | 7187 | 63 | python decl at xlstm/components/conv.py:24 |  |  | 0.366 |
| walker |  | 7274 | 87 | python method sigs in xlstm/components/conv.py |  |  | 0.366 |
| walker |  | 7274 | 0 | python method at xlstm/components/conv.py:20 |  |  | 0.366 |
| walker |  | 7274 | 0 | python method at xlstm/components/conv.py:71 |  |  | 0.366 |
| walker |  | 7274 | 0 | python method at xlstm/components/conv.py:95 |  |  | 0.366 |
| walker |  | 7282 | 8 | python method body at xlstm/components/conv.py:95 body 96 |  |  | 0.366 |
| walker |  | 7332 | 50 | python method at xlstm/components/conv.py:131 |  |  | 0.366 |
| walker |  | 7401 | 69 | python class body at xlstm/components/conv.py:12 |  |  | 0.366 |
| ns | 7423 |  | 363 | `xLSTMBlock` — class docstring, `__init__` dispatch, and its complete method roster | 3.10 | 3.1 | 0.358 |
| walker |  | 7457 | 56 | python method at xlstm/components/conv.py:110 |  |  | 0.358 |
| walker |  | 7483 | 26 | python method at xlstm/components/conv.py:98 |  |  | 0.358 |
| ns | 7674 |  | 251 | Roster: every definition in `xlstm/xlstm_large/`'s support modules | 4.1 |  | 0.352 |
| walker |  | 7696 | 213 | python class body at xlstm/components/conv.py:55 |  |  | 0.352 |
| walker |  | 7773 | 77 | python imports in xlstm/utils.py |  |  | 0.352 |
| walker |  | 7827 | 54 | python decl names surface in experiments/data/utils.py |  |  | 0.352 |
| walker |  | 7827 | 0 | python decl at experiments/data/utils.py:17 |  |  | 0.352 |
| walker |  | 7827 | 0 | python decl at experiments/data/utils.py:39 |  |  | 0.352 |
| walker |  | 7827 | 0 | python decl at experiments/data/utils.py:55 |  |  | 0.352 |
| walker |  | 7827 | 0 | python decl at experiments/data/utils.py:90 |  |  | 0.352 |
| ns | 7854 |  | 180 | `mLSTMLayerConfig` (NeurIPS) — `conv1d_kernel_size`, `qkv_proj_blocksize`, `num_heads`, `proj_factor` | 4.2 |  | 0.347 |
| walker |  | 8014 | 187 | python method sigs in experiments/data/utils.py |  |  | 0.347 |
| walker |  | 8014 | 0 | python method at experiments/data/utils.py:41 |  |  | 0.347 |
| walker |  | 8020 | 6 | python method at experiments/data/utils.py:57 |  |  | 0.347 |
| walker |  | 8028 | 8 | python method at experiments/data/utils.py:46 |  |  | 0.347 |
| walker |  | 8036 | 8 | python method at experiments/data/utils.py:50 |  |  | 0.347 |
| walker |  | 8044 | 8 | python method at experiments/data/utils.py:77 |  |  | 0.347 |
| walker |  | 8052 | 8 | python method at experiments/data/utils.py:85 |  |  | 0.347 |
| walker |  | 8067 | 15 | python method at experiments/data/utils.py:19 |  |  | 0.347 |
| walker |  | 8084 | 17 | python method at experiments/data/utils.py:24 |  |  | 0.347 |
| walker |  | 8101 | 17 | python method at experiments/data/utils.py:29 |  |  | 0.347 |
| walker |  | 8118 | 17 | python method at experiments/data/utils.py:34 |  |  | 0.347 |
| walker |  | 8123 | 5 | python method body at experiments/data/utils.py:19 body 22 |  |  | 0.347 |
| walker |  | 8165 | 42 | python decl names surface in xlstm/components/util.py |  |  | 0.350 |
| walker |  | 8165 | 0 | python decl at xlstm/components/util.py:7 |  |  | 0.350 |
| walker |  | 8165 | 0 | python decl at xlstm/components/util.py:11 |  |  | 0.350 |
| walker |  | 8165 | 0 | python decl at xlstm/components/util.py:26 |  |  | 0.350 |
| ns | 8169 |  | 315 | `sLSTMLayerConfig` fields, and the head of `sLSTMCellConfig` (`backend`, `bias_init`, `num_states`) | 4.3 | 3.1 | 0.343 |
| walker |  | 8189 | 24 | python decl doc at xlstm/components/util.py:11 |  |  | 0.343 |
| walker |  | 8207 | 18 | python decl body at xlstm/components/util.py:7 body 8 |  |  | 0.343 |
| walker |  | 8313 | 106 | python method sigs in xlstm/components/util.py |  |  | 0.343 |
| walker |  | 8313 | 0 | python method at xlstm/components/util.py:58 |  |  | 0.343 |
| walker |  | 8313 | 0 | python method at xlstm/components/util.py:73 |  |  | 0.343 |
| walker |  | 8321 | 8 | python method at xlstm/components/util.py:61 |  |  | 0.343 |
| walker |  | 8329 | 8 | python method at xlstm/components/util.py:65 |  |  | 0.343 |
| walker |  | 8337 | 8 | python method at xlstm/components/util.py:69 |  |  | 0.343 |
| walker |  | 8354 | 17 | python method at xlstm/components/util.py:46 |  |  | 0.343 |
| walker |  | 8372 | 18 | python method at xlstm/components/util.py:51 |  |  | 0.343 |
| ns | 8400 |  | 231 | sLSTM backend dispatch: `sLSTMCell.__new__`, and the runtime CUDA build in `sLSTMCellCUDA.instance` | 4.4 | 3.1 | 0.338 |
| walker |  | 8434 | 62 | python method at xlstm/components/util.py:34 |  |  | 0.338 |
| walker |  | 8529 | 95 | python decl doc at xlstm/components/util.py:26 |  |  | 0.338 |
| walker |  | 8558 | 29 | python decl names surface in xlstm/components/ln.py |  |  | 0.341 |
| walker |  | 8558 | 0 | python decl at xlstm/components/ln.py:8 |  |  | 0.341 |
| walker |  | 8558 | 0 | python decl at xlstm/components/ln.py:51 |  |  | 0.341 |
| walker |  | 8580 | 22 | python decl doc at xlstm/components/ln.py:8 |  |  | 0.341 |
| ns | 8628 |  | 228 | `_act_fn_registry` and `FeedForwardConfig` — the allowed `act_fn` values | 4.5 |  | 0.336 |
| walker |  | 8655 | 75 | python method sigs in xlstm/components/ln.py |  |  | 0.336 |
| walker |  | 8655 | 0 | python method at xlstm/components/ln.py:36 |  |  | 0.336 |
| walker |  | 8655 | 0 | python method at xlstm/components/ln.py:41 |  |  | 0.336 |
| walker |  | 8655 | 0 | python method at xlstm/components/ln.py:53 |  |  | 0.336 |
| walker |  | 8663 | 8 | python method at xlstm/components/ln.py:27 |  |  | 0.336 |
| walker |  | 8737 | 74 | python method at xlstm/components/ln.py:11 |  |  | 0.336 |
| walker |  | 8748 | 11 | c body xlstm/blocks/slstm/src/util/support.h:48 |  |  | 0.336 |
| ns | 8804 |  | 176 | Complete listings of the sLSTM kernel tree: `src/`, `src/cuda/`, `src/util/`, `src/vanilla/` | 4.6 |  | 0.356 |
| walker |  | 8968 | 220 | c names xlstm/blocks/slstm/src/util/blas.h |  |  | 0.354 |
| ns | 8968 |  | 164 | README: the three commands that run the parity experiments | 5.1 | 1.6 | 0.354 |
| walker |  | 9027 | 59 | c decl xlstm/blocks/slstm/src/util/blas.h:75 |  |  | 0.354 |
| walker |  | 9088 | 61 | c decl xlstm/blocks/slstm/src/util/blas.h:80 |  |  | 0.354 |
| ns | 9139 |  | 171 | `experiments/main.py` — the dataset registry and the `__main__` config entry point | 5.2 |  | 0.350 |
| walker |  | 9153 | 65 | c decl xlstm/blocks/slstm/src/util/blas.h:39 |  |  | 0.350 |
| walker |  | 9249 | 96 | c decl xlstm/blocks/slstm/src/util/blas.h:60 |  |  | 0.350 |
| walker |  | 9350 | 101 | c decl xlstm/blocks/slstm/src/util/blas.h:67 |  |  | 0.350 |
| ns | 9440 |  | 301 | `experiments/parity_xlstm01.yaml` — the training and model sections of a real config | 5.3 |  | 0.343 |
| walker |  | 9456 | 106 | c decl xlstm/blocks/slstm/src/util/blas.h:52 |  |  | 0.343 |
| walker |  | 9502 | 46 | python imports in xlstm/components/util.py |  |  | 0.343 |
| walker |  | 9507 | 5 | python method body at experiments/data/utils.py:24 body 27 |  |  | 0.343 |
| walker |  | 9557 | 50 | python imports in xlstm/components/init.py |  |  | 0.343 |
| walker |  | 9569 | 12 | python method body at experiments/lr_scheduler.py:10 body 11 |  |  | 0.343 |
| ns | 9653 |  | 213 | `tests/conftest.py` in full, plus every test function in the suite | 5.4 |  | 0.340 |
| walker |  | 9681 | 112 | python decl doc at xlstm/components/conv.py:24 |  |  | 0.340 |
| walker |  | 9777 | 96 | python decl names surface in xlstm/xlstm_large/generate.py |  |  | 0.341 |
| walker |  | 9777 | 0 | python decl at xlstm/xlstm_large/generate.py:13 |  |  | 0.341 |
| walker |  | 9777 | 0 | python decl at xlstm/xlstm_large/generate.py:23 |  |  | 0.341 |
| walker |  | 9790 | 13 | python decl body at xlstm/xlstm_large/generate.py:23 body 24 |  |  | 0.341 |
| walker |  | 9819 | 29 | python decl at xlstm/xlstm_large/generate.py:8 |  |  | 0.343 |
| ns | 9837 |  | 184 | `pyproject.toml` — project metadata, the dependency list marker, and package data | 6.1 |  | 0.352 |
| walker |  | 9841 | 22 | python decl doc at xlstm/xlstm_large/generate.py:13 |  |  | 0.352 |
| walker |  | 9856 | 15 | python decl body at xlstm/xlstm_large/generate.py:13 body 15 |  |  | 0.352 |
| walker |  | 9871 | 15 | python decl at xlstm/xlstm_large/generate.py:18 |  |  | 0.354 |
| walker |  | 9968 | 97 | python decl names surface in xlstm/xlstm_large/components.py |  |  | 0.361 |
| walker |  | 9968 | 0 | python decl at xlstm/xlstm_large/components.py:5 |  |  | 0.361 |
| walker |  | 9968 | 0 | python decl at xlstm/xlstm_large/components.py:24 |  |  | 0.361 |
| walker |  | 9968 | 0 | python decl at xlstm/xlstm_large/components.py:70 |  |  | 0.361 |
| walker |  | 9968 | 0 | python decl at xlstm/xlstm_large/components.py:99 |  |  | 0.361 |
| walker |  | 9968 | 0 | python decl at xlstm/xlstm_large/components.py:155 |  |  | 0.361 |
| walker |  | 9968 | 0 | python decl at xlstm/xlstm_large/components.py:188 |  |  | 0.361 |
| walker |  | 9983 | 15 | python decl doc at xlstm/xlstm_large/components.py:188 |  |  | 0.361 |
| ns | 10005 |  | 168 | `pytest.ini` in full, and the README install commands | 6.2 | 1.6 | 0.359 |
