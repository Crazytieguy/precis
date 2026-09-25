Score(3000)=0.497 I=0.538 C=0.459 ns_rows≤3K=16/45 grid(1000/1442/2080/3000/4327/6240/9000)=0.593/0.714/0.610/0.497/0.401/0.385/0.354

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
| walker |  | 2022 | 13 | c decl xlstm/blocks/slstm/src/util/util.h:3 |  |  | 0.610 |
| walker |  | 2047 | 25 | c names xlstm/blocks/slstm/src/util/cuda_error.h |  |  | 0.610 |
| walker |  | 2067 | 20 | c decl xlstm/blocks/slstm/src/util/cuda_error.h:9 |  |  | 0.610 |
| ns | 2215 |  | 373 | `xLSTMLargeConfig` part 2/4 — qk/v dim factors and kernel selection (`chunkwise_kernel`, `sequence_kernel`, `step_kernel`, `mode`) | 2.4 | 2.3 | 0.562 |
| walker |  | 2241 | 174 | python decl at xlstm/blocks/slstm/src/vanilla/__init__.py:77 |  |  | 0.562 |
| walker |  | 2417 | 176 | python decl at xlstm/blocks/slstm/src/vanilla/__init__.py:17 |  |  | 0.562 |
| walker |  | 2431 | 14 | python decl names surface in experiments/metrics.py |  |  | 0.562 |
| walker |  | 2431 | 0 | python decl at experiments/metrics.py:9 |  |  | 0.562 |
| ns | 2458 |  | 243 | `xLSTMLargeConfig` part 3/4 — chunking, state return, and kernel dtypes | 2.5 | 2.4 | 0.536 |
| walker |  | 2496 | 65 | python method sigs in experiments/metrics.py |  |  | 0.536 |
| walker |  | 2496 | 0 | python method at experiments/metrics.py:13 |  |  | 0.536 |
| walker |  | 2496 | 0 | python method at experiments/metrics.py:17 |  |  | 0.536 |
| walker |  | 2496 | 0 | python method at experiments/metrics.py:22 |  |  | 0.536 |
| walker |  | 2496 | 0 | python method at experiments/metrics.py:25 |  |  | 0.536 |
| walker |  | 2505 | 9 | python method body at experiments/metrics.py:22 body 23 |  |  | 0.536 |
| walker |  | 2532 | 27 | python decl names surface in xlstm/xlstm_block_stack.py |  |  | 0.536 |
| walker |  | 2532 | 0 | python decl at xlstm/xlstm_block_stack.py:77 |  |  | 0.536 |
| walker |  | 2544 | 12 | python decl at xlstm/xlstm_block_stack.py:15 |  |  | 0.536 |
| walker |  | 2683 | 139 | python method sigs in xlstm/xlstm_block_stack.py |  |  | 0.536 |
| walker |  | 2683 | 0 | python method at xlstm/xlstm_block_stack.py:40 |  |  | 0.536 |
| walker |  | 2683 | 0 | python method at xlstm/xlstm_block_stack.py:52 |  |  | 0.536 |
| walker |  | 2683 | 0 | python method at xlstm/xlstm_block_stack.py:80 |  |  | 0.536 |
| walker |  | 2683 | 0 | python method at xlstm/xlstm_block_stack.py:90 |  |  | 0.536 |
| walker |  | 2683 | 0 | python method at xlstm/xlstm_block_stack.py:111 |  |  | 0.536 |
| walker |  | 2683 | 0 | python method at xlstm/xlstm_block_stack.py:117 |  |  | 0.536 |
| walker |  | 2691 | 8 | python method at xlstm/xlstm_block_stack.py:36 |  |  | 0.536 |
| ns | 2698 |  | 240 | `xLSTMLargeConfig` part 4/4 — feedforward sizing, soft caps, `weight_mode` | 2.6 | 2.5 | 0.516 |
| walker |  | 2706 | 15 | python method body at xlstm/xlstm_block_stack.py:36 body 38 |  |  | 0.516 |
| walker |  | 2719 | 13 | python class body at xlstm/xlstm_block_stack.py:77 |  |  | 0.516 |
| walker |  | 2775 | 56 | python method at xlstm/xlstm_block_stack.py:126 |  |  | 0.516 |
| walker |  | 2804 | 29 | python decl names surface in xlstm/utils.py |  |  | 0.516 |
| walker |  | 2804 | 0 | python decl at xlstm/utils.py:32 |  |  | 0.516 |
| walker |  | 2814 | 10 | python decl at xlstm/utils.py:11 |  |  | 0.516 |
| walker |  | 2847 | 33 | python class body at experiments/metrics.py:9 |  |  | 0.516 |
| ns | 2891 |  | 193 | `xlstm/xlstm_large/model.py` header: the hard `mlstm_kernels` dependency and the state type aliases | 2.7 |  | 0.497 |
| ns | 3038 |  | 147 | `xLSTMLarge.__init__` — embedding, backbone, lm_head | 2.8 | 2.2 | 0.483 |
| walker |  | 3097 | 250 | python class body at xlstm/xlstm_block_stack.py:15 |  |  | 0.487 |
| walker |  | 3113 | 16 | python imports in xlstm/xlstm_large/utils.py |  |  | 0.487 |
| walker |  | 3202 | 89 | python class body at xlstm/utils.py:11 |  |  | 0.487 |
| walker |  | 3260 | 58 | c names xlstm/blocks/slstm/src/util/device_assert.h |  |  | 0.487 |
| ns | 3269 |  | 231 | `xLSTMLarge.forward` — signature, shape assert, `soft_cap`, and the conditional state return | 2.9 | 2.8 | 0.467 |
| walker |  | 3273 | 13 | c decl xlstm/blocks/slstm/src/util/device_assert.h:23 |  |  | 0.467 |
| walker |  | 3296 | 23 | c decl xlstm/blocks/slstm/src/util/device_assert.h:27 |  |  | 0.467 |
| walker |  | 3318 | 22 | python method body at experiments/metrics.py:25 body 26 |  |  | 0.467 |
| ns | 3417 |  | 148 | `xLSTMLarge.generate` — signature and delegation to `generate_tokens` | 2.10 | 2.8 | 0.454 |
| walker |  | 3466 | 148 | python method sigs in xlstm/utils.py |  |  | 0.454 |
| walker |  | 3466 | 0 | python method at xlstm/utils.py:20 |  |  | 0.454 |
| walker |  | 3466 | 0 | python method at xlstm/utils.py:33 |  |  | 0.454 |
| walker |  | 3466 | 0 | python method at xlstm/utils.py:36 |  |  | 0.454 |
| walker |  | 3466 | 0 | python method at xlstm/utils.py:61 |  |  | 0.454 |
| walker |  | 3466 | 0 | python method at xlstm/utils.py:77 |  |  | 0.454 |
| walker |  | 3476 | 10 | python method body at xlstm/utils.py:33 body 34 |  |  | 0.454 |
| walker |  | 3541 | 65 | python decl names surface in experiments/main.py |  |  | 0.454 |
| walker |  | 3541 | 0 | python decl at experiments/main.py:32 |  |  | 0.454 |
| walker |  | 3541 | 0 | python decl at experiments/main.py:37 |  |  | 0.454 |
| walker |  | 3558 | 17 | python decl at experiments/main.py:21 |  |  | 0.454 |
| walker |  | 3602 | 44 | python decl at experiments/main.py:25 |  |  | 0.454 |
| walker |  | 3630 | 28 | python decl body at experiments/main.py:32 body 33 |  |  | 0.454 |
| ns | 3633 |  | 216 | `xLSTMLargeBlockStack.__init__` — the `mLSTMBlock` list and the `add_out_norm` switch | 2.11 | 2.2 | 0.437 |
| walker |  | 3665 | 35 | python decl names surface in xlstm/xlstm_lm_model.py |  |  | 0.437 |
| walker |  | 3665 | 0 | python decl at xlstm/xlstm_lm_model.py:22 |  |  | 0.437 |
| walker |  | 3684 | 19 | python decl at xlstm/xlstm_lm_model.py:14 |  |  | 0.438 |
| walker |  | 3734 | 50 | python class body at xlstm/xlstm_lm_model.py:14 |  |  | 0.438 |
| walker |  | 3835 | 101 | python method sigs in xlstm/xlstm_lm_model.py |  |  | 0.438 |
| walker |  | 3835 | 0 | python method at xlstm/xlstm_lm_model.py:25 |  |  | 0.438 |
| walker |  | 3835 | 0 | python method at xlstm/xlstm_lm_model.py:41 |  |  | 0.438 |
| walker |  | 3835 | 0 | python method at xlstm/xlstm_lm_model.py:49 |  |  | 0.438 |
| walker |  | 3835 | 0 | python method at xlstm/xlstm_lm_model.py:65 |  |  | 0.438 |
| walker |  | 3849 | 14 | python class body at xlstm/xlstm_lm_model.py:22 |  |  | 0.438 |
| ns | 3894 |  | 261 | `xLSTMLargeBlockStack.forward` — the in-place per-layer state update | 2.12 | 2.11 | 0.423 |
| walker |  | 3908 | 59 | python method at xlstm/xlstm_lm_model.py:56 |  |  | 0.423 |
| walker |  | 3929 | 21 | python method doc at xlstm/xlstm_block_stack.py:40 |  |  | 0.423 |
| walker |  | 3969 | 40 | python decl names surface in experiments/lr_scheduler.py |  |  | 0.423 |
| walker |  | 3969 | 0 | python decl at experiments/lr_scheduler.py:9 |  |  | 0.423 |
| walker |  | 3969 | 0 | python decl at experiments/lr_scheduler.py:24 |  |  | 0.423 |
| walker |  | 4097 | 128 | python method sigs in experiments/lr_scheduler.py |  |  | 0.423 |
| walker |  | 4097 | 0 | python method at experiments/lr_scheduler.py:10 |  |  | 0.423 |
| walker |  | 4097 | 0 | python method at experiments/lr_scheduler.py:25 |  |  | 0.423 |
| walker |  | 4097 | 0 | python method at experiments/lr_scheduler.py:47 |  |  | 0.423 |
| walker |  | 4105 | 8 | python method at experiments/lr_scheduler.py:33 |  |  | 0.423 |
| walker |  | 4114 | 9 | python method at experiments/lr_scheduler.py:13 |  |  | 0.423 |
| walker |  | 4123 | 9 | python method at experiments/lr_scheduler.py:18 |  |  | 0.423 |
| walker |  | 4137 | 14 | python method doc at experiments/lr_scheduler.py:18 |  |  | 0.423 |
| walker |  | 4153 | 16 | python method doc at experiments/lr_scheduler.py:13 |  |  | 0.423 |
| walker |  | 4169 | 16 | python method doc at experiments/lr_scheduler.py:47 |  |  | 0.423 |
| walker |  | 4177 | 8 | python method body at experiments/lr_scheduler.py:13 body 16 |  |  | 0.423 |
| walker |  | 4231 | 54 | python method doc at xlstm/utils.py:36 |  |  | 0.423 |
| ns | 4260 |  | 366 | `mLSTMBlock.__init__` (xlstm_large) — the flat-config to nested-config mapping | 2.13 | 2.2 | 0.401 |
| walker |  | 4386 | 155 | README.md section #2 |  |  | 0.401 |
| ns | 4421 |  | 161 | `mLSTMBlock.forward` (xlstm_large) — pre-norm + residual around layer and FFN | 2.14 | 2.13 | 0.394 |
| walker |  | 4583 | 197 | README.md section #3 |  |  | 0.399 |
| walker |  | 4591 | 8 | python method body at experiments/lr_scheduler.py:18 body 21 |  |  | 0.399 |
| walker |  | 4647 | 56 | python method doc at xlstm/utils.py:61 |  |  | 0.399 |
| walker |  | 4686 | 39 | python method at xlstm/utils.py:97 |  |  | 0.399 |
| walker |  | 4709 | 23 | python method body at experiments/metrics.py:13 body 14 |  |  | 0.399 |
| walker |  | 4783 | 74 | declaration surface of pytest.ini |  |  | 0.399 |
| walker |  | 4840 | 57 | python imports in experiments/metrics.py |  |  | 0.399 |
| walker |  | 4870 | 30 | python imports in xlstm/xlstm_large/components.py |  |  | 0.399 |
| ns | 4874 |  | 453 | Roster: every top-level class/function of the NeurIPS models and blocks, name + line | 3.1 |  | 0.386 |
| ns | 5187 |  | 313 | Roster: every top-level definition in `xlstm/components/` and the sLSTM kernel's Python layer | 3.2 |  | 0.374 |
| walker |  | 5397 | 527 | plaintext config setup.cfg |  |  | 0.374 |
| ns | 5447 |  | 260 | `xLSTMBlockStackConfig` — the complete field set | 3.3 | 3.1 | 0.394 |
| walker |  | 5457 | 60 | python imports in experiments/lr_scheduler.py |  |  | 0.394 |
| walker |  | 5492 | 35 | python imports in xlstm/xlstm_large/generate.py |  |  | 0.394 |
| ns | 5646 |  | 199 | `block_map` property and `_create_block_map` — how `slstm_at` becomes a per-position block type | 3.4 | 3.3 | 0.390 |
| walker |  | 5694 | 202 | README.md section #6 |  |  | 0.390 |
| walker |  | 5861 | 167 | README.md section #7 |  |  | 0.390 |
| walker |  | 5887 | 26 | python decl names surface in xlstm/blocks/xlstm_block.py |  |  | 0.391 |
| walker |  | 5887 | 0 | python decl at xlstm/blocks/xlstm_block.py:43 |  |  | 0.391 |
| walker |  | 5898 | 11 | python decl at xlstm/blocks/xlstm_block.py:16 |  |  | 0.391 |
| ns | 5900 |  | 254 | `xLSTMBlockStackConfig.__post_init__` — the config mutation cascade | 3.5 | 3.3 | 0.383 |
| walker |  | 5953 | 55 | python decl doc at xlstm/blocks/xlstm_block.py:43 |  |  | 0.383 |
| walker |  | 5967 | 14 | python class body at xlstm/blocks/xlstm_block.py:43 |  |  | 0.383 |
| walker |  | 6077 | 110 | python method sigs in xlstm/blocks/xlstm_block.py |  |  | 0.383 |
| walker |  | 6077 | 0 | python method at xlstm/blocks/xlstm_block.py:27 |  |  | 0.383 |
| walker |  | 6077 | 0 | python method at xlstm/blocks/xlstm_block.py:51 |  |  | 0.383 |
| walker |  | 6077 | 0 | python method at xlstm/blocks/xlstm_block.py:76 |  |  | 0.383 |
| walker |  | 6077 | 0 | python method at xlstm/blocks/xlstm_block.py:82 |  |  | 0.383 |
| walker |  | 6077 | 0 | python method at xlstm/blocks/xlstm_block.py:89 |  |  | 0.383 |
| walker |  | 6187 | 110 | python class body at xlstm/blocks/xlstm_block.py:16 |  |  | 0.383 |
| walker |  | 6213 | 26 | python decl names surface in xlstm/components/linear_headwise.py |  |  | 0.384 |
| walker |  | 6213 | 0 | python decl at xlstm/components/linear_headwise.py:42 |  |  | 0.384 |
| walker |  | 6224 | 11 | python decl at xlstm/components/linear_headwise.py:12 |  |  | 0.385 |
| ns | 6248 |  | 348 | `xLSTMBlockStack.__init__` and `_create_blocks` — block instantiation and `post_blocks_norm` | 3.6 | 3.1 | 0.373 |
| walker |  | 6279 | 55 | python decl doc at xlstm/components/linear_headwise.py:42 |  |  | 0.373 |
| walker |  | 6361 | 82 | python method sigs in xlstm/components/linear_headwise.py |  |  | 0.373 |
| walker |  | 6361 | 0 | python method at xlstm/components/linear_headwise.py:31 |  |  | 0.373 |
| walker |  | 6361 | 0 | python method at xlstm/components/linear_headwise.py:49 |  |  | 0.373 |
| walker |  | 6361 | 0 | python method at xlstm/components/linear_headwise.py:75 |  |  | 0.373 |
| walker |  | 6361 | 0 | python method at xlstm/components/linear_headwise.py:84 |  |  | 0.373 |
| walker |  | 6370 | 9 | python method at xlstm/components/linear_headwise.py:67 |  |  | 0.373 |
| walker |  | 6382 | 12 | python class body at xlstm/components/linear_headwise.py:42 |  |  | 0.373 |
| ns | 6502 |  | 254 | `xLSTMBlockStack.forward` and `step` — the loop and the `block_{i}` state dict | 3.7 | 3.6 | 0.367 |
| walker |  | 6612 | 230 | python class body at xlstm/components/linear_headwise.py:12 |  |  | 0.367 |
| walker |  | 6626 | 14 | python decl names surface in xlstm/xlstm_large/from_pretrained.py |  |  | 0.367 |
| walker |  | 6741 | 115 | python decl at xlstm/xlstm_large/from_pretrained.py:9 |  |  | 0.367 |
| ns | 6834 |  | 332 | `xLSTMLMModelConfig` fields, `xLSTMLMModel.__init__`, and the model's complete method roster | 3.8 | 3.1 | 0.363 |
| walker |  | 6872 | 131 | c names xlstm/blocks/slstm/src/util/support.h |  |  | 0.363 |
| walker |  | 6890 | 18 | c decl xlstm/blocks/slstm/src/util/support.h:28 |  |  | 0.363 |
| walker |  | 6912 | 22 | c decl xlstm/blocks/slstm/src/util/support.h:30 |  |  | 0.363 |
| walker |  | 6935 | 23 | c decl xlstm/blocks/slstm/src/util/support.h:26 |  |  | 0.363 |
| walker |  | 6969 | 34 | c decl xlstm/blocks/slstm/src/util/support.h:40 |  |  | 0.363 |
| walker |  | 7055 | 86 | c decl xlstm/blocks/slstm/src/util/support.h:34 |  |  | 0.363 |
| ns | 7060 |  | 226 | `xLSTMBlockConfig` — the mLSTM-xor-sLSTM invariant | 3.9 | 3.1 | 0.364 |
| walker |  | 7094 | 39 | python decl names surface in xlstm/components/conv.py |  |  | 0.365 |
| walker |  | 7094 | 0 | python decl at xlstm/components/conv.py:55 |  |  | 0.365 |
| walker |  | 7106 | 12 | python decl at xlstm/components/conv.py:12 |  |  | 0.366 |
| walker |  | 7169 | 63 | python decl at xlstm/components/conv.py:24 |  |  | 0.366 |
| walker |  | 7256 | 87 | python method sigs in xlstm/components/conv.py |  |  | 0.366 |
| walker |  | 7256 | 0 | python method at xlstm/components/conv.py:20 |  |  | 0.366 |
| walker |  | 7256 | 0 | python method at xlstm/components/conv.py:71 |  |  | 0.366 |
| walker |  | 7256 | 0 | python method at xlstm/components/conv.py:95 |  |  | 0.366 |
| walker |  | 7264 | 8 | python method body at xlstm/components/conv.py:95 body 96 |  |  | 0.366 |
| walker |  | 7314 | 50 | python method at xlstm/components/conv.py:131 |  |  | 0.366 |
| walker |  | 7383 | 69 | python class body at xlstm/components/conv.py:12 |  |  | 0.366 |
| ns | 7423 |  | 363 | `xLSTMBlock` — class docstring, `__init__` dispatch, and its complete method roster | 3.10 | 3.1 | 0.358 |
| walker |  | 7439 | 56 | python method at xlstm/components/conv.py:110 |  |  | 0.358 |
| walker |  | 7465 | 26 | python method at xlstm/components/conv.py:98 |  |  | 0.358 |
| ns | 7674 |  | 251 | Roster: every definition in `xlstm/xlstm_large/`'s support modules | 4.1 |  | 0.352 |
| walker |  | 7678 | 213 | python class body at xlstm/components/conv.py:55 |  |  | 0.352 |
| walker |  | 7755 | 77 | python imports in xlstm/utils.py |  |  | 0.352 |
| walker |  | 7809 | 54 | python decl names surface in experiments/data/utils.py |  |  | 0.352 |
| walker |  | 7809 | 0 | python decl at experiments/data/utils.py:17 |  |  | 0.352 |
| walker |  | 7809 | 0 | python decl at experiments/data/utils.py:39 |  |  | 0.352 |
| walker |  | 7809 | 0 | python decl at experiments/data/utils.py:55 |  |  | 0.352 |
| walker |  | 7809 | 0 | python decl at experiments/data/utils.py:90 |  |  | 0.352 |
| ns | 7854 |  | 180 | `mLSTMLayerConfig` (NeurIPS) — `conv1d_kernel_size`, `qkv_proj_blocksize`, `num_heads`, `proj_factor` | 4.2 |  | 0.347 |
| walker |  | 7996 | 187 | python method sigs in experiments/data/utils.py |  |  | 0.347 |
| walker |  | 7996 | 0 | python method at experiments/data/utils.py:41 |  |  | 0.347 |
| walker |  | 8002 | 6 | python method at experiments/data/utils.py:57 |  |  | 0.347 |
| walker |  | 8010 | 8 | python method at experiments/data/utils.py:46 |  |  | 0.347 |
| walker |  | 8018 | 8 | python method at experiments/data/utils.py:50 |  |  | 0.347 |
| walker |  | 8026 | 8 | python method at experiments/data/utils.py:77 |  |  | 0.347 |
| walker |  | 8034 | 8 | python method at experiments/data/utils.py:85 |  |  | 0.347 |
| walker |  | 8049 | 15 | python method at experiments/data/utils.py:19 |  |  | 0.347 |
| walker |  | 8066 | 17 | python method at experiments/data/utils.py:24 |  |  | 0.347 |
| walker |  | 8083 | 17 | python method at experiments/data/utils.py:29 |  |  | 0.347 |
| walker |  | 8100 | 17 | python method at experiments/data/utils.py:34 |  |  | 0.347 |
| walker |  | 8105 | 5 | python method body at experiments/data/utils.py:19 body 22 |  |  | 0.347 |
| walker |  | 8147 | 42 | python decl names surface in xlstm/components/util.py |  |  | 0.350 |
| walker |  | 8147 | 0 | python decl at xlstm/components/util.py:7 |  |  | 0.350 |
| walker |  | 8147 | 0 | python decl at xlstm/components/util.py:11 |  |  | 0.350 |
| walker |  | 8147 | 0 | python decl at xlstm/components/util.py:26 |  |  | 0.350 |
| ns | 8169 |  | 315 | `sLSTMLayerConfig` fields, and the head of `sLSTMCellConfig` (`backend`, `bias_init`, `num_states`) | 4.3 | 3.1 | 0.343 |
| walker |  | 8171 | 24 | python decl doc at xlstm/components/util.py:11 |  |  | 0.343 |
| walker |  | 8189 | 18 | python decl body at xlstm/components/util.py:7 body 8 |  |  | 0.343 |
| walker |  | 8295 | 106 | python method sigs in xlstm/components/util.py |  |  | 0.343 |
| walker |  | 8295 | 0 | python method at xlstm/components/util.py:58 |  |  | 0.343 |
| walker |  | 8295 | 0 | python method at xlstm/components/util.py:73 |  |  | 0.343 |
| walker |  | 8303 | 8 | python method at xlstm/components/util.py:61 |  |  | 0.343 |
| walker |  | 8311 | 8 | python method at xlstm/components/util.py:65 |  |  | 0.343 |
| walker |  | 8319 | 8 | python method at xlstm/components/util.py:69 |  |  | 0.343 |
| walker |  | 8336 | 17 | python method at xlstm/components/util.py:46 |  |  | 0.343 |
| walker |  | 8354 | 18 | python method at xlstm/components/util.py:51 |  |  | 0.343 |
| ns | 8400 |  | 231 | sLSTM backend dispatch: `sLSTMCell.__new__`, and the runtime CUDA build in `sLSTMCellCUDA.instance` | 4.4 | 3.1 | 0.338 |
| walker |  | 8416 | 62 | python method at xlstm/components/util.py:34 |  |  | 0.338 |
| walker |  | 8511 | 95 | python decl doc at xlstm/components/util.py:26 |  |  | 0.338 |
| walker |  | 8540 | 29 | python decl names surface in xlstm/components/ln.py |  |  | 0.341 |
| walker |  | 8540 | 0 | python decl at xlstm/components/ln.py:8 |  |  | 0.341 |
| walker |  | 8540 | 0 | python decl at xlstm/components/ln.py:51 |  |  | 0.341 |
| walker |  | 8562 | 22 | python decl doc at xlstm/components/ln.py:8 |  |  | 0.341 |
| ns | 8628 |  | 228 | `_act_fn_registry` and `FeedForwardConfig` — the allowed `act_fn` values | 4.5 |  | 0.336 |
| walker |  | 8637 | 75 | python method sigs in xlstm/components/ln.py |  |  | 0.336 |
| walker |  | 8637 | 0 | python method at xlstm/components/ln.py:36 |  |  | 0.336 |
| walker |  | 8637 | 0 | python method at xlstm/components/ln.py:41 |  |  | 0.336 |
| walker |  | 8637 | 0 | python method at xlstm/components/ln.py:53 |  |  | 0.336 |
| walker |  | 8645 | 8 | python method at xlstm/components/ln.py:27 |  |  | 0.336 |
| walker |  | 8719 | 74 | python method at xlstm/components/ln.py:11 |  |  | 0.336 |
| walker |  | 8730 | 11 | c body xlstm/blocks/slstm/src/util/support.h:48 |  |  | 0.336 |
| ns | 8804 |  | 176 | Complete listings of the sLSTM kernel tree: `src/`, `src/cuda/`, `src/util/`, `src/vanilla/` | 4.6 |  | 0.356 |
| walker |  | 8950 | 220 | c names xlstm/blocks/slstm/src/util/blas.h |  |  | 0.356 |
| ns | 8968 |  | 164 | README: the three commands that run the parity experiments | 5.1 | 1.6 | 0.354 |
| walker |  | 9009 | 59 | c decl xlstm/blocks/slstm/src/util/blas.h:75 |  |  | 0.354 |
| walker |  | 9070 | 61 | c decl xlstm/blocks/slstm/src/util/blas.h:80 |  |  | 0.354 |
| walker |  | 9135 | 65 | c decl xlstm/blocks/slstm/src/util/blas.h:39 |  |  | 0.354 |
| ns | 9139 |  | 171 | `experiments/main.py` — the dataset registry and the `__main__` config entry point | 5.2 |  | 0.350 |
| walker |  | 9231 | 96 | c decl xlstm/blocks/slstm/src/util/blas.h:60 |  |  | 0.350 |
| walker |  | 9332 | 101 | c decl xlstm/blocks/slstm/src/util/blas.h:67 |  |  | 0.350 |
| walker |  | 9438 | 106 | c decl xlstm/blocks/slstm/src/util/blas.h:52 |  |  | 0.350 |
| ns | 9440 |  | 301 | `experiments/parity_xlstm01.yaml` — the training and model sections of a real config | 5.3 |  | 0.343 |
| walker |  | 9484 | 46 | python imports in xlstm/components/util.py |  |  | 0.343 |
| walker |  | 9489 | 5 | python method body at experiments/data/utils.py:24 body 27 |  |  | 0.343 |
| walker |  | 9539 | 50 | python imports in xlstm/components/init.py |  |  | 0.343 |
| walker |  | 9551 | 12 | python method body at experiments/lr_scheduler.py:10 body 11 |  |  | 0.343 |
| ns | 9653 |  | 213 | `tests/conftest.py` in full, plus every test function in the suite | 5.4 |  | 0.340 |
| walker |  | 9663 | 112 | python decl doc at xlstm/components/conv.py:24 |  |  | 0.340 |
| walker |  | 9759 | 96 | python decl names surface in xlstm/xlstm_large/generate.py |  |  | 0.341 |
| walker |  | 9759 | 0 | python decl at xlstm/xlstm_large/generate.py:13 |  |  | 0.341 |
| walker |  | 9759 | 0 | python decl at xlstm/xlstm_large/generate.py:23 |  |  | 0.341 |
| walker |  | 9772 | 13 | python decl body at xlstm/xlstm_large/generate.py:23 body 24 |  |  | 0.341 |
| walker |  | 9801 | 29 | python decl at xlstm/xlstm_large/generate.py:8 |  |  | 0.343 |
| walker |  | 9823 | 22 | python decl doc at xlstm/xlstm_large/generate.py:13 |  |  | 0.343 |
| ns | 9837 |  | 184 | `pyproject.toml` — project metadata, the dependency list marker, and package data | 6.1 |  | 0.352 |
| walker |  | 9838 | 15 | python decl body at xlstm/xlstm_large/generate.py:13 body 15 |  |  | 0.352 |
| walker |  | 9853 | 15 | python decl at xlstm/xlstm_large/generate.py:18 |  |  | 0.354 |
| walker |  | 9950 | 97 | python decl names surface in xlstm/xlstm_large/components.py |  |  | 0.361 |
| walker |  | 9950 | 0 | python decl at xlstm/xlstm_large/components.py:5 |  |  | 0.361 |
| walker |  | 9950 | 0 | python decl at xlstm/xlstm_large/components.py:24 |  |  | 0.361 |
| walker |  | 9950 | 0 | python decl at xlstm/xlstm_large/components.py:70 |  |  | 0.361 |
| walker |  | 9950 | 0 | python decl at xlstm/xlstm_large/components.py:99 |  |  | 0.361 |
| walker |  | 9950 | 0 | python decl at xlstm/xlstm_large/components.py:155 |  |  | 0.361 |
| walker |  | 9950 | 0 | python decl at xlstm/xlstm_large/components.py:188 |  |  | 0.361 |
| walker |  | 9965 | 15 | python decl doc at xlstm/xlstm_large/components.py:188 |  |  | 0.361 |
| ns | 10005 |  | 168 | `pytest.ini` in full, and the README install commands | 6.2 | 1.6 | 0.359 |
