Score(3000)=0.497 I=0.538 C=0.459 ns_rows≤3K=16/45 (reached=6 partial=0 missing=10) grid(1000/1442/2080/3000/4327/6240/9000)=0.593/0.714/0.610/0.497/0.401/0.383/0.356

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
| ns | 1836 |  | 175 | `xLSTMLargeConfig` part 1/4 — the four required fields and the norm/bias toggles | 2.3 | 2.2 | 0.606 |
| walker |  | 1897 | 202 | manifest config in pyproject.toml |  |  | 0.607 |
| walker |  | 1978 | 81 | listing of 'xlstm/blocks/slstm/src/util' |  |  | 0.610 |
| walker |  | 2015 | 37 | c whole header in xlstm/blocks/slstm/src/util/util.h |  |  | 0.610 |
| walker |  | 2189 | 174 | python decl at xlstm/blocks/slstm/src/vanilla/__init__.py:77 |  |  | 0.610 |
| ns | 2209 |  | 373 | `xLSTMLargeConfig` part 2/4 — qk/v dim factors and kernel selection (`chunkwise_kernel`, `sequence_kernel`, `step_kernel`, `mode`) | 2.4 | 2.3 | 0.562 |
| walker |  | 2365 | 176 | python decl at xlstm/blocks/slstm/src/vanilla/__init__.py:17 |  |  | 0.562 |
| walker |  | 2379 | 14 | python decl names surface in experiments/metrics.py |  |  | 0.562 |
| walker |  | 2379 | 0 | python decl at experiments/metrics.py:9 |  |  | 0.562 |
| walker |  | 2444 | 65 | python method sigs in experiments/metrics.py |  |  | 0.562 |
| walker |  | 2444 | 0 | python method at experiments/metrics.py:13 |  |  | 0.562 |
| walker |  | 2444 | 0 | python method at experiments/metrics.py:17 |  |  | 0.562 |
| walker |  | 2444 | 0 | python method at experiments/metrics.py:22 |  |  | 0.562 |
| walker |  | 2444 | 0 | python method at experiments/metrics.py:25 |  |  | 0.562 |
| ns | 2452 |  | 243 | `xLSTMLargeConfig` part 3/4 — chunking, state return, and kernel dtypes | 2.5 | 2.4 | 0.536 |
| walker |  | 2453 | 9 | python method body at experiments/metrics.py:22 body 23 |  |  | 0.536 |
| walker |  | 2480 | 27 | python decl names surface in xlstm/xlstm_block_stack.py |  |  | 0.536 |
| walker |  | 2480 | 0 | python decl at xlstm/xlstm_block_stack.py:77 |  |  | 0.536 |
| walker |  | 2492 | 12 | python decl at xlstm/xlstm_block_stack.py:15 |  |  | 0.536 |
| walker |  | 2631 | 139 | python method sigs in xlstm/xlstm_block_stack.py |  |  | 0.536 |
| walker |  | 2631 | 0 | python method at xlstm/xlstm_block_stack.py:40 |  |  | 0.536 |
| walker |  | 2631 | 0 | python method at xlstm/xlstm_block_stack.py:52 |  |  | 0.536 |
| walker |  | 2631 | 0 | python method at xlstm/xlstm_block_stack.py:80 |  |  | 0.536 |
| walker |  | 2631 | 0 | python method at xlstm/xlstm_block_stack.py:90 |  |  | 0.536 |
| walker |  | 2631 | 0 | python method at xlstm/xlstm_block_stack.py:111 |  |  | 0.536 |
| walker |  | 2631 | 0 | python method at xlstm/xlstm_block_stack.py:117 |  |  | 0.536 |
| walker |  | 2639 | 8 | python method at xlstm/xlstm_block_stack.py:36 |  |  | 0.536 |
| walker |  | 2654 | 15 | python method body at xlstm/xlstm_block_stack.py:36 body 38 |  |  | 0.537 |
| walker |  | 2667 | 13 | python class body at xlstm/xlstm_block_stack.py:77 |  |  | 0.537 |
| ns | 2692 |  | 240 | `xLSTMLargeConfig` part 4/4 — feedforward sizing, soft caps, `weight_mode` | 2.6 | 2.5 | 0.516 |
| walker |  | 2723 | 56 | python method at xlstm/xlstm_block_stack.py:126 |  |  | 0.516 |
| walker |  | 2752 | 29 | python decl names surface in xlstm/utils.py |  |  | 0.516 |
| walker |  | 2752 | 0 | python decl at xlstm/utils.py:32 |  |  | 0.516 |
| walker |  | 2762 | 10 | python decl at xlstm/utils.py:11 |  |  | 0.516 |
| walker |  | 2795 | 33 | python class body at experiments/metrics.py:9 |  |  | 0.516 |
| ns | 2885 |  | 193 | `xlstm/xlstm_large/model.py` header: the hard `mlstm_kernels` dependency and the state type aliases | 2.7 |  | 0.497 |
| ns | 3032 |  | 147 | `xLSTMLarge.__init__` — embedding, backbone, lm_head | 2.8 | 2.2 | 0.483 |
| walker |  | 3045 | 250 | python class body at xlstm/xlstm_block_stack.py:15 |  |  | 0.487 |
| walker |  | 3061 | 16 | python imports in xlstm/xlstm_large/utils.py |  |  | 0.487 |
| walker |  | 3150 | 89 | python class body at xlstm/utils.py:11 |  |  | 0.487 |
| walker |  | 3172 | 22 | python method body at experiments/metrics.py:25 body 26 |  |  | 0.487 |
| ns | 3263 |  | 231 | `xLSTMLarge.forward` — signature, shape assert, `soft_cap`, and the conditional state return | 2.9 | 2.8 | 0.467 |
| walker |  | 3320 | 148 | python method sigs in xlstm/utils.py |  |  | 0.467 |
| walker |  | 3320 | 0 | python method at xlstm/utils.py:20 |  |  | 0.467 |
| walker |  | 3320 | 0 | python method at xlstm/utils.py:33 |  |  | 0.467 |
| walker |  | 3320 | 0 | python method at xlstm/utils.py:36 |  |  | 0.467 |
| walker |  | 3320 | 0 | python method at xlstm/utils.py:61 |  |  | 0.467 |
| walker |  | 3320 | 0 | python method at xlstm/utils.py:77 |  |  | 0.467 |
| walker |  | 3330 | 10 | python method body at xlstm/utils.py:33 body 34 |  |  | 0.467 |
| walker |  | 3395 | 65 | python decl names surface in experiments/main.py |  |  | 0.467 |
| walker |  | 3395 | 0 | python decl at experiments/main.py:32 |  |  | 0.467 |
| walker |  | 3395 | 0 | python decl at experiments/main.py:37 |  |  | 0.467 |
| ns | 3411 |  | 148 | `xLSTMLarge.generate` — signature and delegation to `generate_tokens` | 2.10 | 2.8 | 0.454 |
| walker |  | 3412 | 17 | python decl at experiments/main.py:21 |  |  | 0.454 |
| walker |  | 3456 | 44 | python decl at experiments/main.py:25 |  |  | 0.454 |
| walker |  | 3484 | 28 | python decl body at experiments/main.py:32 body 33 |  |  | 0.454 |
| walker |  | 3519 | 35 | python decl names surface in xlstm/xlstm_lm_model.py |  |  | 0.454 |
| walker |  | 3519 | 0 | python decl at xlstm/xlstm_lm_model.py:22 |  |  | 0.454 |
| walker |  | 3538 | 19 | python decl at xlstm/xlstm_lm_model.py:14 |  |  | 0.455 |
| walker |  | 3588 | 50 | python class body at xlstm/xlstm_lm_model.py:14 |  |  | 0.455 |
| ns | 3627 |  | 216 | `xLSTMLargeBlockStack.__init__` — the `mLSTMBlock` list and the `add_out_norm` switch | 2.11 | 2.2 | 0.438 |
| walker |  | 3689 | 101 | python method sigs in xlstm/xlstm_lm_model.py |  |  | 0.438 |
| walker |  | 3689 | 0 | python method at xlstm/xlstm_lm_model.py:25 |  |  | 0.438 |
| walker |  | 3689 | 0 | python method at xlstm/xlstm_lm_model.py:41 |  |  | 0.438 |
| walker |  | 3689 | 0 | python method at xlstm/xlstm_lm_model.py:49 |  |  | 0.438 |
| walker |  | 3689 | 0 | python method at xlstm/xlstm_lm_model.py:65 |  |  | 0.438 |
| walker |  | 3703 | 14 | python class body at xlstm/xlstm_lm_model.py:22 |  |  | 0.438 |
| walker |  | 3762 | 59 | python method at xlstm/xlstm_lm_model.py:56 |  |  | 0.438 |
| walker |  | 3783 | 21 | python method doc at xlstm/xlstm_block_stack.py:40 |  |  | 0.439 |
| walker |  | 3823 | 40 | python decl names surface in experiments/lr_scheduler.py |  |  | 0.439 |
| walker |  | 3823 | 0 | python decl at experiments/lr_scheduler.py:9 |  |  | 0.439 |
| walker |  | 3823 | 0 | python decl at experiments/lr_scheduler.py:24 |  |  | 0.439 |
| ns | 3888 |  | 261 | `xLSTMLargeBlockStack.forward` — the in-place per-layer state update | 2.12 | 2.11 | 0.423 |
| walker |  | 3951 | 128 | python method sigs in experiments/lr_scheduler.py |  |  | 0.423 |
| walker |  | 3951 | 0 | python method at experiments/lr_scheduler.py:10 |  |  | 0.423 |
| walker |  | 3951 | 0 | python method at experiments/lr_scheduler.py:25 |  |  | 0.423 |
| walker |  | 3951 | 0 | python method at experiments/lr_scheduler.py:47 |  |  | 0.423 |
| walker |  | 3959 | 8 | python method at experiments/lr_scheduler.py:33 |  |  | 0.423 |
| walker |  | 3968 | 9 | python method at experiments/lr_scheduler.py:13 |  |  | 0.423 |
| walker |  | 3977 | 9 | python method at experiments/lr_scheduler.py:18 |  |  | 0.423 |
| walker |  | 3991 | 14 | python method doc at experiments/lr_scheduler.py:18 |  |  | 0.423 |
| walker |  | 4007 | 16 | python method doc at experiments/lr_scheduler.py:13 |  |  | 0.423 |
| walker |  | 4023 | 16 | python method doc at experiments/lr_scheduler.py:47 |  |  | 0.423 |
| walker |  | 4031 | 8 | python method body at experiments/lr_scheduler.py:13 body 16 |  |  | 0.423 |
| walker |  | 4085 | 54 | python method doc at xlstm/utils.py:36 |  |  | 0.423 |
| walker |  | 4240 | 155 | README.md section #2 |  |  | 0.423 |
| ns | 4254 |  | 366 | `mLSTMBlock.__init__` (xlstm_large) — the flat-config to nested-config mapping | 2.13 | 2.2 | 0.401 |
| ns | 4415 |  | 161 | `mLSTMBlock.forward` (xlstm_large) — pre-norm + residual around layer and FFN | 2.14 | 2.13 | 0.394 |
| walker |  | 4437 | 197 | README.md section #3 |  |  | 0.399 |
| walker |  | 4445 | 8 | python method body at experiments/lr_scheduler.py:18 body 21 |  |  | 0.399 |
| walker |  | 4501 | 56 | python method doc at xlstm/utils.py:61 |  |  | 0.399 |
| walker |  | 4540 | 39 | python method at xlstm/utils.py:97 |  |  | 0.399 |
| walker |  | 4553 | 13 | c includes in xlstm/blocks/slstm/src/cuda/slstm.h |  |  | 0.399 |
| walker |  | 4576 | 23 | python method body at experiments/metrics.py:13 body 14 |  |  | 0.399 |
| walker |  | 4705 | 129 | c whole header in xlstm/blocks/slstm/src/util/cuda_error.h |  |  | 0.399 |
| walker |  | 4779 | 74 | declaration surface of pytest.ini |  |  | 0.399 |
| walker |  | 4836 | 57 | python imports in experiments/metrics.py |  |  | 0.399 |
| walker |  | 4866 | 30 | python imports in xlstm/xlstm_large/components.py |  |  | 0.399 |
| ns | 4868 |  | 453 | Roster: every top-level class/function of the NeurIPS models and blocks, name + line | 3.1 |  | 0.386 |
| ns | 5181 |  | 313 | Roster: every top-level definition in `xlstm/components/` and the sLSTM kernel's Python layer | 3.2 |  | 0.374 |
| walker |  | 5393 | 527 | plaintext config setup.cfg |  |  | 0.374 |
| ns | 5441 |  | 260 | `xLSTMBlockStackConfig` — the complete field set | 3.3 | 3.1 | 0.394 |
| walker |  | 5453 | 60 | python imports in experiments/lr_scheduler.py |  |  | 0.394 |
| walker |  | 5488 | 35 | python imports in xlstm/xlstm_large/generate.py |  |  | 0.394 |
| walker |  | 5619 | 131 | c decl names surface in xlstm/blocks/slstm/src/util/support.h |  |  | 0.394 |
| walker |  | 5619 | 0 | c decl at xlstm/blocks/slstm/src/util/support.h:48 |  |  | 0.394 |
| walker |  | 5637 | 18 | c decl at xlstm/blocks/slstm/src/util/support.h:28 |  |  | 0.394 |
| ns | 5640 |  | 199 | `block_map` property and `_create_block_map` — how `slstm_at` becomes a per-position block type | 3.4 | 3.3 | 0.390 |
| walker |  | 5659 | 22 | c decl at xlstm/blocks/slstm/src/util/support.h:30 |  |  | 0.390 |
| walker |  | 5682 | 23 | c decl at xlstm/blocks/slstm/src/util/support.h:26 |  |  | 0.390 |
| walker |  | 5693 | 11 | c decl body at xlstm/blocks/slstm/src/util/support.h:48 |  |  | 0.390 |
| walker |  | 5727 | 34 | c decl at xlstm/blocks/slstm/src/util/support.h:40 |  |  | 0.390 |
| ns | 5894 |  | 254 | `xLSTMBlockStackConfig.__post_init__` — the config mutation cascade | 3.5 | 3.3 | 0.381 |
| walker |  | 5929 | 202 | README.md section #6 |  |  | 0.381 |
| walker |  | 6096 | 167 | README.md section #7 |  |  | 0.381 |
| walker |  | 6122 | 26 | python decl names surface in xlstm/blocks/xlstm_block.py |  |  | 0.382 |
| walker |  | 6122 | 0 | python decl at xlstm/blocks/xlstm_block.py:43 |  |  | 0.382 |
| walker |  | 6133 | 11 | python decl at xlstm/blocks/xlstm_block.py:16 |  |  | 0.383 |
| walker |  | 6188 | 55 | python decl doc at xlstm/blocks/xlstm_block.py:43 |  |  | 0.383 |
| walker |  | 6202 | 14 | python class body at xlstm/blocks/xlstm_block.py:43 |  |  | 0.383 |
| ns | 6242 |  | 348 | `xLSTMBlockStack.__init__` and `_create_blocks` — block instantiation and `post_blocks_norm` | 3.6 | 3.1 | 0.371 |
| walker |  | 6312 | 110 | python method sigs in xlstm/blocks/xlstm_block.py |  |  | 0.371 |
| walker |  | 6312 | 0 | python method at xlstm/blocks/xlstm_block.py:27 |  |  | 0.371 |
| walker |  | 6312 | 0 | python method at xlstm/blocks/xlstm_block.py:51 |  |  | 0.371 |
| walker |  | 6312 | 0 | python method at xlstm/blocks/xlstm_block.py:76 |  |  | 0.371 |
| walker |  | 6312 | 0 | python method at xlstm/blocks/xlstm_block.py:82 |  |  | 0.371 |
| walker |  | 6312 | 0 | python method at xlstm/blocks/xlstm_block.py:89 |  |  | 0.371 |
| walker |  | 6422 | 110 | python class body at xlstm/blocks/xlstm_block.py:16 |  |  | 0.371 |
| walker |  | 6448 | 26 | python decl names surface in xlstm/components/linear_headwise.py |  |  | 0.372 |
| walker |  | 6448 | 0 | python decl at xlstm/components/linear_headwise.py:42 |  |  | 0.372 |
| walker |  | 6459 | 11 | python decl at xlstm/components/linear_headwise.py:12 |  |  | 0.373 |
| ns | 6496 |  | 254 | `xLSTMBlockStack.forward` and `step` — the loop and the `block_{i}` state dict | 3.7 | 3.6 | 0.367 |
| walker |  | 6514 | 55 | python decl doc at xlstm/components/linear_headwise.py:42 |  |  | 0.367 |
| walker |  | 6596 | 82 | python method sigs in xlstm/components/linear_headwise.py |  |  | 0.367 |
| walker |  | 6596 | 0 | python method at xlstm/components/linear_headwise.py:31 |  |  | 0.367 |
| walker |  | 6596 | 0 | python method at xlstm/components/linear_headwise.py:49 |  |  | 0.367 |
| walker |  | 6596 | 0 | python method at xlstm/components/linear_headwise.py:75 |  |  | 0.367 |
| walker |  | 6596 | 0 | python method at xlstm/components/linear_headwise.py:84 |  |  | 0.367 |
| walker |  | 6605 | 9 | python method at xlstm/components/linear_headwise.py:67 |  |  | 0.367 |
| walker |  | 6617 | 12 | python class body at xlstm/components/linear_headwise.py:42 |  |  | 0.367 |
| ns | 6828 |  | 332 | `xLSTMLMModelConfig` fields, `xLSTMLMModel.__init__`, and the model's complete method roster | 3.8 | 3.1 | 0.363 |
| walker |  | 6847 | 230 | python class body at xlstm/components/linear_headwise.py:12 |  |  | 0.363 |
| walker |  | 6861 | 14 | python decl names surface in xlstm/xlstm_large/from_pretrained.py |  |  | 0.363 |
| walker |  | 6976 | 115 | python decl at xlstm/xlstm_large/from_pretrained.py:9 |  |  | 0.363 |
| walker |  | 7015 | 39 | python decl names surface in xlstm/components/conv.py |  |  | 0.364 |
| walker |  | 7015 | 0 | python decl at xlstm/components/conv.py:55 |  |  | 0.364 |
| walker |  | 7027 | 12 | python decl at xlstm/components/conv.py:12 |  |  | 0.365 |
| ns | 7054 |  | 226 | `xLSTMBlockConfig` — the mLSTM-xor-sLSTM invariant | 3.9 | 3.1 | 0.366 |
| walker |  | 7090 | 63 | python decl at xlstm/components/conv.py:24 |  |  | 0.366 |
| walker |  | 7177 | 87 | python method sigs in xlstm/components/conv.py |  |  | 0.366 |
| walker |  | 7177 | 0 | python method at xlstm/components/conv.py:20 |  |  | 0.366 |
| walker |  | 7177 | 0 | python method at xlstm/components/conv.py:71 |  |  | 0.366 |
| walker |  | 7177 | 0 | python method at xlstm/components/conv.py:95 |  |  | 0.366 |
| walker |  | 7185 | 8 | python method body at xlstm/components/conv.py:95 body 96 |  |  | 0.366 |
| walker |  | 7235 | 50 | python method at xlstm/components/conv.py:131 |  |  | 0.366 |
| walker |  | 7304 | 69 | python class body at xlstm/components/conv.py:12 |  |  | 0.366 |
| walker |  | 7360 | 56 | python method at xlstm/components/conv.py:110 |  |  | 0.366 |
| walker |  | 7386 | 26 | python method at xlstm/components/conv.py:98 |  |  | 0.366 |
| ns | 7417 |  | 363 | `xLSTMBlock` — class docstring, `__init__` dispatch, and its complete method roster | 3.10 | 3.1 | 0.358 |
| walker |  | 7599 | 213 | python class body at xlstm/components/conv.py:55 |  |  | 0.358 |
| ns | 7668 |  | 251 | Roster: every definition in `xlstm/xlstm_large/`'s support modules | 4.1 |  | 0.352 |
| walker |  | 7676 | 77 | python imports in xlstm/utils.py |  |  | 0.352 |
| walker |  | 7730 | 54 | python decl names surface in experiments/data/utils.py |  |  | 0.352 |
| walker |  | 7730 | 0 | python decl at experiments/data/utils.py:17 |  |  | 0.352 |
| walker |  | 7730 | 0 | python decl at experiments/data/utils.py:39 |  |  | 0.352 |
| walker |  | 7730 | 0 | python decl at experiments/data/utils.py:55 |  |  | 0.352 |
| walker |  | 7730 | 0 | python decl at experiments/data/utils.py:90 |  |  | 0.352 |
| ns | 7848 |  | 180 | `mLSTMLayerConfig` (NeurIPS) — `conv1d_kernel_size`, `qkv_proj_blocksize`, `num_heads`, `proj_factor` | 4.2 |  | 0.347 |
| walker |  | 7917 | 187 | python method sigs in experiments/data/utils.py |  |  | 0.347 |
| walker |  | 7917 | 0 | python method at experiments/data/utils.py:41 |  |  | 0.347 |
| walker |  | 7923 | 6 | python method at experiments/data/utils.py:57 |  |  | 0.347 |
| walker |  | 7931 | 8 | python method at experiments/data/utils.py:46 |  |  | 0.347 |
| walker |  | 7939 | 8 | python method at experiments/data/utils.py:50 |  |  | 0.347 |
| walker |  | 7947 | 8 | python method at experiments/data/utils.py:77 |  |  | 0.347 |
| walker |  | 7955 | 8 | python method at experiments/data/utils.py:85 |  |  | 0.347 |
| walker |  | 7970 | 15 | python method at experiments/data/utils.py:19 |  |  | 0.347 |
| walker |  | 7987 | 17 | python method at experiments/data/utils.py:24 |  |  | 0.347 |
| walker |  | 8004 | 17 | python method at experiments/data/utils.py:29 |  |  | 0.347 |
| walker |  | 8021 | 17 | python method at experiments/data/utils.py:34 |  |  | 0.347 |
| walker |  | 8026 | 5 | python method body at experiments/data/utils.py:19 body 22 |  |  | 0.347 |
| walker |  | 8068 | 42 | python decl names surface in xlstm/components/util.py |  |  | 0.350 |
| walker |  | 8068 | 0 | python decl at xlstm/components/util.py:7 |  |  | 0.350 |
| walker |  | 8068 | 0 | python decl at xlstm/components/util.py:11 |  |  | 0.350 |
| walker |  | 8068 | 0 | python decl at xlstm/components/util.py:26 |  |  | 0.350 |
| walker |  | 8092 | 24 | python decl doc at xlstm/components/util.py:11 |  |  | 0.350 |
| walker |  | 8110 | 18 | python decl body at xlstm/components/util.py:7 body 8 |  |  | 0.350 |
| ns | 8163 |  | 315 | `sLSTMLayerConfig` fields, and the head of `sLSTMCellConfig` (`backend`, `bias_init`, `num_states`) | 4.3 | 3.1 | 0.343 |
| walker |  | 8216 | 106 | python method sigs in xlstm/components/util.py |  |  | 0.343 |
| walker |  | 8216 | 0 | python method at xlstm/components/util.py:58 |  |  | 0.343 |
| walker |  | 8216 | 0 | python method at xlstm/components/util.py:73 |  |  | 0.343 |
| walker |  | 8224 | 8 | python method at xlstm/components/util.py:61 |  |  | 0.343 |
| walker |  | 8232 | 8 | python method at xlstm/components/util.py:65 |  |  | 0.343 |
| walker |  | 8240 | 8 | python method at xlstm/components/util.py:69 |  |  | 0.343 |
| walker |  | 8257 | 17 | python method at xlstm/components/util.py:46 |  |  | 0.343 |
| walker |  | 8275 | 18 | python method at xlstm/components/util.py:51 |  |  | 0.343 |
| walker |  | 8337 | 62 | python method at xlstm/components/util.py:34 |  |  | 0.343 |
| ns | 8394 |  | 231 | sLSTM backend dispatch: `sLSTMCell.__new__`, and the runtime CUDA build in `sLSTMCellCUDA.instance` | 4.4 | 3.1 | 0.338 |
| walker |  | 8432 | 95 | python decl doc at xlstm/components/util.py:26 |  |  | 0.338 |
| walker |  | 8461 | 29 | python decl names surface in xlstm/components/ln.py |  |  | 0.341 |
| walker |  | 8461 | 0 | python decl at xlstm/components/ln.py:8 |  |  | 0.341 |
| walker |  | 8461 | 0 | python decl at xlstm/components/ln.py:51 |  |  | 0.341 |
| walker |  | 8483 | 22 | python decl doc at xlstm/components/ln.py:8 |  |  | 0.341 |
| walker |  | 8558 | 75 | python method sigs in xlstm/components/ln.py |  |  | 0.341 |
| walker |  | 8558 | 0 | python method at xlstm/components/ln.py:36 |  |  | 0.341 |
| walker |  | 8558 | 0 | python method at xlstm/components/ln.py:41 |  |  | 0.341 |
| walker |  | 8558 | 0 | python method at xlstm/components/ln.py:53 |  |  | 0.341 |
| walker |  | 8566 | 8 | python method at xlstm/components/ln.py:27 |  |  | 0.341 |
| ns | 8622 |  | 228 | `_act_fn_registry` and `FeedForwardConfig` — the allowed `act_fn` values | 4.5 |  | 0.336 |
| walker |  | 8640 | 74 | python method at xlstm/components/ln.py:11 |  |  | 0.336 |
| walker |  | 8686 | 46 | python imports in xlstm/components/util.py |  |  | 0.336 |
| walker |  | 8691 | 5 | python method body at experiments/data/utils.py:24 body 27 |  |  | 0.336 |
| walker |  | 8741 | 50 | python imports in xlstm/components/init.py |  |  | 0.336 |
| walker |  | 8753 | 12 | python method body at experiments/lr_scheduler.py:10 body 11 |  |  | 0.336 |
| ns | 8798 |  | 176 | Complete listings of the sLSTM kernel tree: `src/`, `src/cuda/`, `src/util/`, `src/vanilla/` | 4.6 |  | 0.356 |
| walker |  | 8865 | 112 | python decl doc at xlstm/components/conv.py:24 |  |  | 0.356 |
| walker |  | 8891 | 26 | c includes in xlstm/blocks/slstm/src/util/support.h |  |  | 0.356 |
| ns | 8962 |  | 164 | README: the three commands that run the parity experiments | 5.1 | 1.6 | 0.354 |
| walker |  | 8987 | 96 | python decl names surface in xlstm/xlstm_large/generate.py |  |  | 0.356 |
| walker |  | 8987 | 0 | python decl at xlstm/xlstm_large/generate.py:13 |  |  | 0.356 |
| walker |  | 8987 | 0 | python decl at xlstm/xlstm_large/generate.py:23 |  |  | 0.356 |
| walker |  | 9000 | 13 | python decl body at xlstm/xlstm_large/generate.py:23 body 24 |  |  | 0.356 |
| walker |  | 9029 | 29 | python decl at xlstm/xlstm_large/generate.py:8 |  |  | 0.358 |
| walker |  | 9051 | 22 | python decl doc at xlstm/xlstm_large/generate.py:13 |  |  | 0.358 |
| walker |  | 9066 | 15 | python decl body at xlstm/xlstm_large/generate.py:13 body 15 |  |  | 0.358 |
| walker |  | 9081 | 15 | python decl at xlstm/xlstm_large/generate.py:18 |  |  | 0.359 |
| ns | 9133 |  | 171 | `experiments/main.py` — the dataset registry and the `__main__` config entry point | 5.2 |  | 0.356 |
| walker |  | 9178 | 97 | python decl names surface in xlstm/xlstm_large/components.py |  |  | 0.364 |
| walker |  | 9178 | 0 | python decl at xlstm/xlstm_large/components.py:5 |  |  | 0.364 |
| walker |  | 9178 | 0 | python decl at xlstm/xlstm_large/components.py:24 |  |  | 0.364 |
| walker |  | 9178 | 0 | python decl at xlstm/xlstm_large/components.py:70 |  |  | 0.364 |
| walker |  | 9178 | 0 | python decl at xlstm/xlstm_large/components.py:99 |  |  | 0.364 |
| walker |  | 9178 | 0 | python decl at xlstm/xlstm_large/components.py:155 |  |  | 0.364 |
| walker |  | 9178 | 0 | python decl at xlstm/xlstm_large/components.py:188 |  |  | 0.364 |
| walker |  | 9193 | 15 | python decl doc at xlstm/xlstm_large/components.py:188 |  |  | 0.364 |
| walker |  | 9229 | 36 | python decl doc at xlstm/xlstm_large/components.py:99 |  |  | 0.364 |
| walker |  | 9412 | 183 | python method sigs in xlstm/xlstm_large/components.py |  |  | 0.364 |
| walker |  | 9412 | 0 | python method at xlstm/xlstm_large/components.py:58 |  |  | 0.364 |
| walker |  | 9421 | 9 | python method at xlstm/xlstm_large/components.py:65 |  |  | 0.364 |
| ns | 9434 |  | 301 | `experiments/parity_xlstm01.yaml` — the training and model sections of a real config | 5.3 |  | 0.356 |
| walker |  | 9438 | 17 | python method at xlstm/xlstm_large/components.py:93 |  |  | 0.356 |
| walker |  | 9455 | 17 | python method at xlstm/xlstm_large/components.py:182 |  |  | 0.356 |
| walker |  | 9502 | 47 | python method at xlstm/xlstm_large/components.py:142 |  |  | 0.356 |
| walker |  | 9549 | 47 | python method at xlstm/xlstm_large/components.py:231 |  |  | 0.356 |
| walker |  | 9627 | 78 | python method at xlstm/xlstm_large/components.py:35 |  |  | 0.356 |
| ns | 9647 |  | 213 | `tests/conftest.py` in full, plus every test function in the suite | 5.4 |  | 0.352 |
| walker |  | 9715 | 88 | python method at xlstm/xlstm_large/components.py:123 |  |  | 0.352 |
| walker |  | 9803 | 88 | python method at xlstm/xlstm_large/components.py:212 |  |  | 0.352 |
| ns | 9831 |  | 184 | `pyproject.toml` — project metadata, the dependency list marker, and package data | 6.1 |  | 0.361 |
| walker |  | 9840 | 37 | python method at xlstm/xlstm_large/components.py:84 |  |  | 0.361 |
| walker |  | 9877 | 37 | python method at xlstm/xlstm_large/components.py:172 |  |  | 0.361 |
| ns | 9999 |  | 168 | `pytest.ini` in full, and the README install commands | 6.2 | 1.6 | 0.359 |
