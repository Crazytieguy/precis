Score(3000)=0.490 I=0.528 C=0.455 ns_rows≤3K=16/45 grid(1000/1442/2080/3000/4327/6240/9000)=0.521/0.543/0.543/0.490/0.394/0.396/0.384

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 65 | 65 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 74 | 9 | Fs::DirListing { dir: notebooks } |  |  | 0.000 |
| walker |  | 77 | 3 | Fs::DirListing { dir: .github } |  |  | 0.000 |
| walker |  | 85 | 8 | Fs::DirListing { dir: .github/workflows } |  |  | 0.000 |
| ns | 96 |  | 96 | What xLSTM is, from the README lede | 1.1 |  | 0.000 |
| walker |  | 113 | 28 | Fs::DirListing { dir: res } |  |  | 0.000 |
| walker |  | 128 | 15 | Fs::DirListing { dir: notebooks/xlstm_large } |  |  | 0.000 |
| ns | 161 |  | 65 | Complete repository-root listing | 1.2 |  | 0.594 |
| walker |  | 164 | 36 | Fs::DirListing { dir: xlstm } |  |  | 0.608 |
| walker |  | 181 | 17 | Code::CodeKey { rung: ModuleDoc, file: xlstm/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.608 |
| walker |  | 203 | 22 | Fs::DirListing { dir: xlstm/blocks } |  |  | 0.626 |
| ns | 225 |  | 64 | README: the 7B model and the name "xLSTM Large" | 1.3 |  | 0.575 |
| walker |  | 226 | 23 | Fs::DirListing { dir: xlstm/blocks/mlstm } |  |  | 0.582 |
| walker |  | 249 | 23 | Fs::DirListing { dir: xlstm/blocks/slstm } |  |  | 0.592 |
| walker |  | 271 | 22 | Fs::DirListing { dir: xlstm/blocks/slstm/src } |  |  | 0.592 |
| walker |  | 287 | 16 | Fs::DirListing { dir: xlstm/blocks/slstm/src/vanilla } |  |  | 0.593 |
| walker |  | 315 | 28 | Fs::DirListing { dir: xlstm/xlstm_large } |  |  | 0.621 |
| ns | 346 |  | 121 | Complete listings of `xlstm/`, `xlstm/blocks/`, `xlstm/components/`, `xlstm/xlstm_large/` | 1.4 |  | 0.528 |
| walker |  | 350 | 35 | Fs::DirListing { dir: xlstm/components } |  |  | 0.676 |
| walker |  | 396 | 46 | Fs::DirListing { dir: experiments } |  |  | 0.679 |
| walker |  | 412 | 16 | Fs::DirListing { dir: experiments/data } |  |  | 0.682 |
| walker |  | 438 | 26 | Fs::DirListing { dir: experiments/data/formal_language } |  |  | 0.689 |
| walker |  | 466 | 28 | Fs::DirListing { dir: experiments/data/formal_language/tasks } |  |  | 0.698 |
| ns | 528 |  | 182 | `xlstm/__init__.py` in full — version plus the entire public export block | 1.5 |  | 0.629 |
| walker |  | 551 | 85 | Toml::Identity { file: pyproject.toml } |  |  | 0.629 |
| walker |  | 584 | 33 | Fs::DirListing { dir: notebooks/xlstm } |  |  | 0.635 |
| walker |  | 687 | 103 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.648 |
| ns | 712 |  | 184 | Every README heading (H1/H2/H3), line-located | 1.6 | 1.1 | 0.568 |
| walker |  | 729 | 42 | Code::CodeKey { rung: Names, file: xlstm/xlstm_large/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.568 |
| walker |  | 743 | 14 | Code::CodeKey { rung: Names, file: experiments/metrics.py, decl: 0, sub: 0, line: 0 } |  |  | 0.568 |
| walker |  | 855 | 112 | Code::CodeKey { rung: Names, file: xlstm/blocks/slstm/src/vanilla/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.568 |
| walker |  | 892 | 37 | Code::CodeKey { rung: Decl, file: xlstm/blocks/slstm/src/vanilla/__init__.py, decl: 1, sub: 0, line: 11 } |  |  | 0.568 |
| ns | 968 |  | 256 | README 75-85: the 7B code is `xlstm/xlstm_large`, standalone on `mlstm_kernels` | 1.7 | 1.6 | 0.521 |
| walker |  | 1066 | 174 | Code::CodeKey { rung: Decl, file: xlstm/blocks/slstm/src/vanilla/__init__.py, decl: 3, sub: 0, line: 77 } |  |  | 0.521 |
| ns | 1139 |  | 171 | Complete listings of `tests/`, `experiments/` and the `experiments/data/` tree | 1.8 |  | 0.514 |
| walker |  | 1242 | 176 | Code::CodeKey { rung: Decl, file: xlstm/blocks/slstm/src/vanilla/__init__.py, decl: 2, sub: 0, line: 17 } |  |  | 0.514 |
| walker |  | 1256 | 14 | Code::CodeKey { rung: Names, file: xlstm/xlstm_large/from_pretrained.py, decl: 0, sub: 0, line: 0 } |  |  | 0.514 |
| ns | 1278 |  | 139 | Complete listings of `xlstm/blocks/mlstm/`, `xlstm/blocks/slstm/`, `notebooks/`, `res/`, `.github/workflows/` | 1.9 |  | 0.543 |
| walker |  | 1443 | 187 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.544 |
| walker |  | 1474 | 31 | Code::CodeKey { rung: Names, file: xlstm/utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.544 |
| walker |  | 1505 | 31 | Code::CodeKey { rung: Names, file: xlstm/xlstm_block_stack.py, decl: 0, sub: 0, line: 0 } |  |  | 0.544 |
| ns | 1548 |  | 270 | README quickstart: instantiate `xLSTMLargeConfig` + `xLSTMLarge` and run a forward pass | 2.1 |  | 0.500 |
| walker |  | 1608 | 103 | Code::CodeKey { rung: Decl, file: experiments/metrics.py, decl: 1, sub: 0, line: 9 } |  |  | 0.500 |
| walker |  | 1617 | 9 | Code::CodeKey { rung: Body, file: experiments/metrics.py, decl: 4, sub: 0, line: 22 } |  |  | 0.500 |
| ns | 1667 |  | 119 | Every top-level class in `xlstm/xlstm_large/model.py`, name + line | 2.2 |  | 0.484 |
| walker |  | 1727 | 110 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_block_stack.py, decl: 5, sub: 0, line: 77 } |  |  | 0.484 |
| walker |  | 1783 | 56 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_block_stack.py, decl: 10, sub: 0, line: 126 } |  |  | 0.484 |
| walker |  | 1838 | 55 | Fs::DirListing { dir: tests } |  |  | 0.533 |
| ns | 1842 |  | 175 | `xLSTMLargeConfig` part 1/4 — the four required fields and the norm/bias toggles | 2.3 | 2.2 | 0.508 |
| walker |  | 1998 | 160 | Code::CodeKey { rung: Names, file: xlstm/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.543 |
| walker |  | 2184 | 186 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.601 |
| ns | 2215 |  | 373 | `xLSTMLargeConfig` part 2/4 — qk/v dim factors and kernel selection (`chunkwise_kernel`, `sequence_kernel`, `step_kernel`, `mode`) | 2.4 | 2.3 | 0.554 |
| walker |  | 2307 | 123 | Code::CodeKey { rung: Decl, file: xlstm/utils.py, decl: 1, sub: 0, line: 11 } |  |  | 0.554 |
| walker |  | 2434 | 127 | Code::CodeKey { rung: Decl, file: xlstm/utils.py, decl: 3, sub: 0, line: 32 } |  |  | 0.554 |
| walker |  | 2444 | 10 | Code::CodeKey { rung: Body, file: xlstm/utils.py, decl: 4, sub: 0, line: 33 } |  |  | 0.554 |
| ns | 2458 |  | 243 | `xLSTMLargeConfig` part 3/4 — chunking, state return, and kernel dtypes | 2.5 | 2.4 | 0.528 |
| walker |  | 2484 | 40 | Code::CodeKey { rung: Names, file: experiments/lr_scheduler.py, decl: 0, sub: 0, line: 0 } |  |  | 0.528 |
| walker |  | 2537 | 53 | Code::CodeKey { rung: Decl, file: experiments/lr_scheduler.py, decl: 1, sub: 0, line: 9 } |  |  | 0.528 |
| walker |  | 2546 | 9 | Code::CodeKey { rung: Decl, file: experiments/lr_scheduler.py, decl: 3, sub: 0, line: 13 } |  |  | 0.528 |
| walker |  | 2555 | 9 | Code::CodeKey { rung: Decl, file: experiments/lr_scheduler.py, decl: 4, sub: 0, line: 18 } |  |  | 0.528 |
| walker |  | 2630 | 75 | Code::CodeKey { rung: Decl, file: experiments/lr_scheduler.py, decl: 5, sub: 0, line: 24 } |  |  | 0.528 |
| walker |  | 2638 | 8 | Code::CodeKey { rung: Decl, file: experiments/lr_scheduler.py, decl: 7, sub: 0, line: 33 } |  |  | 0.528 |
| walker |  | 2684 | 46 | Code::CodeKey { rung: Names, file: xlstm/xlstm_lm_model.py, decl: 0, sub: 0, line: 0 } |  |  | 0.529 |
| ns | 2698 |  | 240 | `xLSTMLargeConfig` part 4/4 — feedforward sizing, soft caps, `weight_mode` | 2.6 | 2.5 | 0.508 |
| walker |  | 2742 | 58 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_lm_model.py, decl: 1, sub: 0, line: 14 } |  |  | 0.509 |
| walker |  | 2862 | 120 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_lm_model.py, decl: 2, sub: 0, line: 22 } |  |  | 0.509 |
| ns | 2891 |  | 193 | `xlstm/xlstm_large/model.py` header: the hard `mlstm_kernels` dependency and the state type aliases | 2.7 |  | 0.490 |
| walker |  | 2921 | 59 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_lm_model.py, decl: 6, sub: 0, line: 56 } |  |  | 0.490 |
| walker |  | 2935 | 14 | Code::CodeKey { rung: Doc, file: experiments/lr_scheduler.py, decl: 4, sub: 0, line: 18 } |  |  | 0.490 |
| walker |  | 2951 | 16 | Code::CodeKey { rung: Doc, file: experiments/lr_scheduler.py, decl: 3, sub: 0, line: 13 } |  |  | 0.490 |
| walker |  | 2967 | 16 | Code::CodeKey { rung: Doc, file: experiments/lr_scheduler.py, decl: 8, sub: 0, line: 47 } |  |  | 0.490 |
| walker |  | 2996 | 29 | Code::CodeKey { rung: Names, file: xlstm/blocks/xlstm_block.py, decl: 0, sub: 0, line: 0 } |  |  | 0.490 |
| walker |  | 3025 | 29 | Code::CodeKey { rung: Names, file: xlstm/components/linear_headwise.py, decl: 0, sub: 0, line: 0 } |  |  | 0.490 |
| ns | 3038 |  | 147 | `xLSTMLarge.__init__` — embedding, backbone, lm_head | 2.8 | 2.2 | 0.477 |
| walker |  | 3112 | 87 | Code::CodeKey { rung: Decl, file: xlstm/components/linear_headwise.py, decl: 3, sub: 0, line: 42 } |  |  | 0.477 |
| walker |  | 3141 | 29 | Code::CodeKey { rung: Names, file: xlstm/components/ln.py, decl: 0, sub: 0, line: 0 } |  |  | 0.477 |
| walker |  | 3164 | 23 | Code::CodeKey { rung: Decl, file: xlstm/components/ln.py, decl: 6, sub: 0, line: 51 } |  |  | 0.477 |
| walker |  | 3223 | 59 | Code::CodeKey { rung: Decl, file: xlstm/components/ln.py, decl: 1, sub: 0, line: 8 } |  |  | 0.477 |
| walker |  | 3231 | 8 | Code::CodeKey { rung: Decl, file: xlstm/components/ln.py, decl: 3, sub: 0, line: 27 } |  |  | 0.477 |
| walker |  | 3242 | 11 | Code::CodeKey { rung: Names, file: experiments/data/formal_language/tasks/parity.py, decl: 0, sub: 0, line: 0 } |  |  | 0.477 |
| ns | 3269 |  | 231 | `xLSTMLarge.forward` — signature, shape assert, `soft_cap`, and the conditional state return | 2.9 | 2.8 | 0.458 |
| walker |  | 3380 | 138 | Toml::PackageMetadata { file: pyproject.toml } |  |  | 0.458 |
| ns | 3417 |  | 148 | `xLSTMLarge.generate` — signature and delegation to `generate_tokens` | 2.10 | 2.8 | 0.446 |
| walker |  | 3436 | 56 | Fs::DirListing { dir: xlstm/blocks/slstm/src/cuda } |  |  | 0.446 |
| walker |  | 3448 | 12 | Code::CodeKey { rung: Names, file: experiments/data/formal_language/tasks/even_pairs.py, decl: 0, sub: 0, line: 0 } |  |  | 0.446 |
| walker |  | 3513 | 65 | Code::CodeKey { rung: Names, file: experiments/main.py, decl: 0, sub: 0, line: 0 } |  |  | 0.446 |
| walker |  | 3530 | 17 | Code::CodeKey { rung: Decl, file: experiments/main.py, decl: 1, sub: 0, line: 21 } |  |  | 0.446 |
| walker |  | 3574 | 44 | Code::CodeKey { rung: Decl, file: experiments/main.py, decl: 2, sub: 0, line: 25 } |  |  | 0.446 |
| walker |  | 3602 | 28 | Code::CodeKey { rung: Body, file: experiments/main.py, decl: 3, sub: 0, line: 32 } |  |  | 0.446 |
| ns | 3633 |  | 216 | `xLSTMLargeBlockStack.__init__` — the `mLSTMBlock` list and the `add_out_norm` switch | 2.11 | 2.2 | 0.430 |
| walker |  | 3717 | 115 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/from_pretrained.py, decl: 1, sub: 0, line: 9 } |  |  | 0.430 |
| walker |  | 3737 | 20 | Code::CodeKey { rung: Doc, file: xlstm/components/ln.py, decl: 1, sub: 0, line: 8 } |  |  | 0.430 |
| walker |  | 3854 | 117 | Code::CodeKey { rung: Decl, file: xlstm/blocks/xlstm_block.py, decl: 3, sub: 0, line: 43 } |  |  | 0.430 |
| walker |  | 3893 | 39 | Code::CodeKey { rung: Decl, file: xlstm/utils.py, decl: 8, sub: 0, line: 97 } |  |  | 0.430 |
| ns | 3894 |  | 261 | `xLSTMLargeBlockStack.forward` — the in-place per-layer state update | 2.12 | 2.11 | 0.415 |
| walker |  | 3915 | 22 | Code::CodeKey { rung: Body, file: experiments/metrics.py, decl: 5, sub: 0, line: 25 } |  |  | 0.415 |
| walker |  | 3957 | 42 | Code::CodeKey { rung: Names, file: xlstm/components/util.py, decl: 0, sub: 0, line: 0 } |  |  | 0.415 |
| walker |  | 4063 | 106 | Code::CodeKey { rung: Decl, file: xlstm/components/util.py, decl: 3, sub: 0, line: 26 } |  |  | 0.415 |
| walker |  | 4071 | 8 | Code::CodeKey { rung: Decl, file: xlstm/components/util.py, decl: 8, sub: 0, line: 61 } |  |  | 0.415 |
| walker |  | 4079 | 8 | Code::CodeKey { rung: Decl, file: xlstm/components/util.py, decl: 9, sub: 0, line: 65 } |  |  | 0.415 |
| walker |  | 4087 | 8 | Code::CodeKey { rung: Decl, file: xlstm/components/util.py, decl: 10, sub: 0, line: 69 } |  |  | 0.415 |
| walker |  | 4149 | 62 | Code::CodeKey { rung: Decl, file: xlstm/components/util.py, decl: 4, sub: 0, line: 34 } |  |  | 0.415 |
| walker |  | 4191 | 42 | Code::CodeKey { rung: Names, file: xlstm/xlstm_large/utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.415 |
| walker |  | 4219 | 28 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/utils.py, decl: 2, sub: 0, line: 10 } |  |  | 0.415 |
| walker |  | 4236 | 17 | Code::CodeKey { rung: Doc, file: xlstm/xlstm_large/utils.py, decl: 1, sub: 0, line: 5 } |  |  | 0.415 |
| walker |  | 4258 | 22 | Code::CodeKey { rung: Body, file: xlstm/xlstm_large/utils.py, decl: 1, sub: 0, line: 5 } |  |  | 0.415 |
| ns | 4260 |  | 366 | `mLSTMBlock.__init__` (xlstm_large) — the flat-config to nested-config mapping | 2.13 | 2.2 | 0.394 |
| walker |  | 4395 | 137 | Code::CodeKey { rung: Decl, file: xlstm/blocks/xlstm_block.py, decl: 1, sub: 0, line: 16 } |  |  | 0.394 |
| walker |  | 4418 | 23 | Code::CodeKey { rung: Body, file: experiments/metrics.py, decl: 2, sub: 0, line: 13 } |  |  | 0.394 |
| ns | 4421 |  | 161 | `mLSTMBlock.forward` (xlstm_large) — pre-norm + residual around layer and FFN | 2.14 | 2.13 | 0.387 |
| walker |  | 4461 | 43 | Code::CodeKey { rung: Names, file: xlstm/components/conv.py, decl: 0, sub: 0, line: 0 } |  |  | 0.387 |
| walker |  | 4524 | 63 | Code::CodeKey { rung: Decl, file: xlstm/components/conv.py, decl: 3, sub: 0, line: 24 } |  |  | 0.387 |
| walker |  | 4620 | 96 | Code::CodeKey { rung: Decl, file: xlstm/components/conv.py, decl: 1, sub: 0, line: 12 } |  |  | 0.387 |
| walker |  | 4644 | 24 | Code::CodeKey { rung: Doc, file: xlstm/components/util.py, decl: 2, sub: 0, line: 11 } |  |  | 0.387 |
| walker |  | 4718 | 74 | Code::CodeKey { rung: Decl, file: xlstm/components/ln.py, decl: 2, sub: 0, line: 11 } |  |  | 0.387 |
| ns | 4874 |  | 453 | Roster: every top-level class/function of the NeurIPS models and blocks, name + line | 3.1 |  | 0.376 |
| walker |  | 4920 | 202 | Toml::Config { file: pyproject.toml } |  |  | 0.377 |
| walker |  | 5002 | 82 | Fs::DirListing { dir: xlstm/blocks/slstm/src/util } |  |  | 0.378 |
| walker |  | 5014 | 12 | Code::CodeKey { rung: Names, file: xlstm/blocks/slstm/src/cuda/slstm.h, decl: 0, sub: 0, line: 0 } |  |  | 0.378 |
| ns | 5187 |  | 313 | Roster: every top-level definition in `xlstm/components/` and the sLSTM kernel's Python layer | 3.2 |  | 0.379 |
| walker |  | 5321 | 307 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_block_stack.py, decl: 1, sub: 0, line: 15 } |  |  | 0.382 |
| walker |  | 5327 | 6 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_block_stack.py, decl: 2, sub: 0, line: 36 } |  |  | 0.382 |
| walker |  | 5342 | 15 | Code::CodeKey { rung: Body, file: xlstm/xlstm_block_stack.py, decl: 2, sub: 0, line: 36 } |  |  | 0.382 |
| walker |  | 5371 | 29 | Code::CodeKey { rung: Names, file: xlstm/blocks/mlstm/cell.py, decl: 0, sub: 0, line: 0 } |  |  | 0.384 |
| walker |  | 5416 | 45 | Code::CodeKey { rung: Decl, file: xlstm/blocks/mlstm/cell.py, decl: 1, sub: 0, line: 13 } |  |  | 0.384 |
| ns | 5447 |  | 260 | `xLSTMBlockStackConfig` — the complete field set | 3.3 | 3.1 | 0.403 |
| walker |  | 5509 | 93 | Code::CodeKey { rung: Decl, file: xlstm/blocks/mlstm/cell.py, decl: 2, sub: 0, line: 20 } |  |  | 0.403 |
| walker |  | 5522 | 13 | Code::CodeKey { rung: Names, file: xlstm/blocks/slstm/src/util/util.h, decl: 0, sub: 0, line: 0 } |  |  | 0.403 |
| walker |  | 5535 | 13 | Code::CodeKey { rung: Decl, file: xlstm/blocks/slstm/src/util/util.h, decl: 1, sub: 0, line: 3 } |  |  | 0.403 |
| walker |  | 5566 | 31 | Code::CodeKey { rung: Names, file: xlstm/blocks/mlstm/block.py, decl: 0, sub: 0, line: 0 } |  |  | 0.405 |
| walker |  | 5607 | 41 | Code::CodeKey { rung: Decl, file: xlstm/blocks/mlstm/block.py, decl: 3, sub: 0, line: 22 } |  |  | 0.405 |
| walker |  | 5638 | 31 | Code::CodeKey { rung: Names, file: xlstm/blocks/slstm/block.py, decl: 0, sub: 0, line: 0 } |  |  | 0.408 |
| ns | 5646 |  | 199 | `block_map` property and `_create_block_map` — how `slstm_at` becomes a per-position block type | 3.4 | 3.3 | 0.401 |
| walker |  | 5676 | 38 | Code::CodeKey { rung: Decl, file: xlstm/blocks/slstm/block.py, decl: 3, sub: 0, line: 29 } |  |  | 0.401 |
| walker |  | 5778 | 102 | Code::CodeKey { rung: Decl, file: xlstm/blocks/mlstm/block.py, decl: 1, sub: 0, line: 9 } |  |  | 0.401 |
| walker |  | 5832 | 54 | Code::CodeKey { rung: Names, file: experiments/data/utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.401 |
| walker |  | 5880 | 48 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 10, sub: 0, line: 55 } |  |  | 0.401 |
| walker |  | 5886 | 6 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 11, sub: 0, line: 57 } |  |  | 0.401 |
| walker |  | 5894 | 8 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 12, sub: 0, line: 77 } |  |  | 0.401 |
| ns | 5900 |  | 254 | `xLSTMBlockStackConfig.__post_init__` — the config mutation cascade | 3.5 | 3.3 | 0.392 |
| walker |  | 5902 | 8 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 13, sub: 0, line: 85 } |  |  | 0.392 |
| walker |  | 5969 | 67 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 6, sub: 0, line: 39 } |  |  | 0.392 |
| walker |  | 5977 | 8 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 8, sub: 0, line: 46 } |  |  | 0.392 |
| walker |  | 5985 | 8 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 9, sub: 0, line: 50 } |  |  | 0.392 |
| walker |  | 6062 | 77 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 1, sub: 0, line: 17 } |  |  | 0.392 |
| walker |  | 6077 | 15 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 2, sub: 0, line: 19 } |  |  | 0.392 |
| walker |  | 6094 | 17 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 3, sub: 0, line: 24 } |  |  | 0.392 |
| walker |  | 6111 | 17 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 4, sub: 0, line: 29 } |  |  | 0.392 |
| walker |  | 6128 | 17 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 5, sub: 0, line: 34 } |  |  | 0.392 |
| walker |  | 6202 | 74 | Plaintext::DeclSurface { file: pytest.ini } |  |  | 0.392 |
| walker |  | 6217 | 15 | Code::CodeKey { rung: Names, file: xlstm/blocks/slstm/src/vanilla/lstm.py, decl: 0, sub: 0, line: 0 } |  |  | 0.394 |
| walker |  | 6232 | 15 | Code::CodeKey { rung: Names, file: xlstm/blocks/slstm/src/vanilla/slstm.py, decl: 0, sub: 0, line: 0 } |  |  | 0.396 |
| ns | 6248 |  | 348 | `xLSTMBlockStack.__init__` and `_create_blocks` — block instantiation and `post_blocks_norm` | 3.6 | 3.1 | 0.384 |
| walker |  | 6268 | 36 | Code::CodeKey { rung: Names, file: xlstm/blocks/mlstm/layer.py, decl: 0, sub: 0, line: 0 } |  |  | 0.387 |
| walker |  | 6353 | 85 | Code::CodeKey { rung: Decl, file: xlstm/blocks/mlstm/layer.py, decl: 3, sub: 0, line: 39 } |  |  | 0.387 |
| walker |  | 6389 | 36 | Code::CodeKey { rung: Names, file: xlstm/blocks/slstm/layer.py, decl: 0, sub: 0, line: 0 } |  |  | 0.391 |
| walker |  | 6461 | 72 | Code::CodeKey { rung: Decl, file: xlstm/blocks/slstm/layer.py, decl: 3, sub: 0, line: 33 } |  |  | 0.391 |
| ns | 6502 |  | 254 | `xLSTMBlockStack.forward` and `step` — the loop and the `block_{i}` state dict | 3.7 | 3.6 | 0.384 |
| walker |  | 6517 | 56 | Code::CodeKey { rung: Decl, file: xlstm/blocks/slstm/layer.py, decl: 6, sub: 0, line: 92 } |  |  | 0.384 |
| walker |  | 6540 | 23 | Code::CodeKey { rung: Names, file: experiments/data/formal_language/tasks/cycle_navigation.py, decl: 0, sub: 0, line: 0 } |  |  | 0.384 |
| walker |  | 6566 | 26 | Code::CodeKey { rung: Decl, file: experiments/data/formal_language/tasks/cycle_navigation.py, decl: 2, sub: 0, line: 51 } |  |  | 0.384 |
| walker |  | 6691 | 125 | Code::CodeKey { rung: Decl, file: xlstm/blocks/slstm/block.py, decl: 1, sub: 0, line: 12 } |  |  | 0.384 |
| walker |  | 6731 | 40 | Code::CodeKey { rung: Names, file: xlstm/blocks/mlstm/backends.py, decl: 0, sub: 0, line: 0 } |  |  | 0.389 |
| ns | 6834 |  | 332 | `xLSTMLMModelConfig` fields, `xLSTMLMModel.__init__`, and the model's complete method roster | 3.8 | 3.1 | 0.383 |
| walker |  | 6847 | 116 | Code::CodeKey { rung: Decl, file: xlstm/blocks/mlstm/backends.py, decl: 1, sub: 0, line: 9 } |  |  | 0.383 |
| walker |  | 6978 | 131 | Code::CodeKey { rung: Decl, file: xlstm/blocks/mlstm/backends.py, decl: 2, sub: 0, line: 93 } |  |  | 0.383 |
| walker |  | 7004 | 26 | Code::CodeKey { rung: Names, file: experiments/data/formal_language/tasks/modular_arithmetic.py, decl: 0, sub: 0, line: 0 } |  |  | 0.383 |
| walker |  | 7058 | 54 | Code::CodeKey { rung: Decl, file: experiments/data/formal_language/tasks/modular_arithmetic.py, decl: 2, sub: 0, line: 62 } |  |  | 0.383 |
| ns | 7060 |  | 226 | `xLSTMBlockConfig` — the mLSTM-xor-sLSTM invariant | 3.9 | 3.1 | 0.384 |
| walker |  | 7099 | 41 | Code::CodeKey { rung: Names, file: experiments/data/formal_language/online_generate.py, decl: 0, sub: 0, line: 0 } |  |  | 0.384 |
| walker |  | 7151 | 52 | Code::CodeKey { rung: Decl, file: experiments/data/formal_language/online_generate.py, decl: 1, sub: 0, line: 9 } |  |  | 0.384 |
| walker |  | 7238 | 87 | Code::CodeKey { rung: Decl, file: experiments/data/formal_language/online_generate.py, decl: 9, sub: 0, line: 41 } |  |  | 0.384 |
| walker |  | 7246 | 8 | Code::CodeKey { rung: Decl, file: experiments/data/formal_language/online_generate.py, decl: 14, sub: 0, line: 67 } |  |  | 0.384 |
| walker |  | 7254 | 8 | Code::CodeKey { rung: Decl, file: experiments/data/formal_language/online_generate.py, decl: 15, sub: 0, line: 71 } |  |  | 0.384 |
| walker |  | 7387 | 133 | Code::CodeKey { rung: Decl, file: xlstm/blocks/slstm/layer.py, decl: 1, sub: 0, line: 18 } |  |  | 0.384 |
| walker |  | 7407 | 20 | Code::CodeKey { rung: Body, file: xlstm/components/conv.py, decl: 2, sub: 0, line: 20 } |  |  | 0.384 |
| walker |  | 7415 | 8 | Code::CodeKey { rung: Body, file: experiments/data/formal_language/online_generate.py, decl: 4, sub: 0, line: 25 } |  |  | 0.384 |
| walker |  | 7423 | 8 | Code::CodeKey { rung: Body, file: experiments/lr_scheduler.py, decl: 3, sub: 0, line: 13 } |  |  | 0.373 |
| ns | 7423 |  | 363 | `xLSTMBlock` — class docstring, `__init__` dispatch, and its complete method roster | 3.10 | 3.1 | 0.373 |
| walker |  | 7501 | 78 | Code::CodeKey { rung: Decl, file: xlstm/blocks/slstm/layer.py, decl: 7, sub: 0, line: 123 } |  |  | 0.373 |
| walker |  | 7580 | 79 | Code::CodeKey { rung: Decl, file: xlstm/blocks/mlstm/layer.py, decl: 6, sub: 0, line: 127 } |  |  | 0.373 |
| ns | 7674 |  | 251 | Roster: every definition in `xlstm/xlstm_large/`'s support modules | 4.1 |  | 0.367 |
| walker |  | 7837 | 257 | Code::CodeKey { rung: Decl, file: xlstm/components/linear_headwise.py, decl: 1, sub: 0, line: 12 } |  |  | 0.367 |
| ns | 7854 |  | 180 | `mLSTMLayerConfig` (NeurIPS) — `conv1d_kernel_size`, `qkv_proj_blocksize`, `num_heads`, `proj_factor` | 4.2 |  | 0.362 |
| walker |  | 7885 | 48 | Code::CodeKey { rung: Decl, file: experiments/data/formal_language/online_generate.py, decl: 5, sub: 0, line: 29 } |  |  | 0.362 |
| walker |  | 7992 | 107 | Code::CodeKey { rung: Decl, file: experiments/data/formal_language/tasks/even_pairs.py, decl: 1, sub: 0, line: 8 } |  |  | 0.362 |
| walker |  | 8081 | 89 | Code::CodeKey { rung: Names, file: xlstm/components/feedforward.py, decl: 0, sub: 0, line: 0 } |  |  | 0.369 |
| walker |  | 8150 | 69 | Code::CodeKey { rung: Decl, file: xlstm/components/feedforward.py, decl: 5, sub: 0, line: 49 } |  |  | 0.369 |
| ns | 8169 |  | 315 | `sLSTMLayerConfig` fields, and the head of `sLSTMCellConfig` (`backend`, `bias_init`, `num_states`) | 4.3 | 3.1 | 0.365 |
| walker |  | 8283 | 133 | Code::CodeKey { rung: Decl, file: xlstm/components/feedforward.py, decl: 3, sub: 0, line: 31 } |  |  | 0.366 |
| walker |  | 8372 | 89 | Code::CodeKey { rung: Names, file: xlstm/components/init.py, decl: 0, sub: 0, line: 0 } |  |  | 0.371 |
| walker |  | 8386 | 14 | Code::CodeKey { rung: Doc, file: xlstm/components/init.py, decl: 1, sub: 0, line: 8 } |  |  | 0.371 |
| ns | 8400 |  | 231 | sLSTM backend dispatch: `sLSTMCell.__new__`, and the runtime CUDA build in `sLSTMCellCUDA.instance` | 4.4 | 3.1 | 0.366 |
| walker |  | 8418 | 32 | Code::CodeKey { rung: Doc, file: xlstm/components/init.py, decl: 3, sub: 0, line: 28 } |  |  | 0.366 |
| walker |  | 8463 | 45 | Code::CodeKey { rung: Body, file: xlstm/components/init.py, decl: 3, sub: 0, line: 28 } |  |  | 0.366 |
| walker |  | 8510 | 47 | Code::CodeKey { rung: Body, file: xlstm/components/init.py, decl: 2, sub: 0, line: 18 } |  |  | 0.366 |
| ns | 8628 |  | 228 | `_act_fn_registry` and `FeedForwardConfig` — the allowed `act_fn` values | 4.5 |  | 0.365 |
| walker |  | 8665 | 155 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.365 |
| ns | 8804 |  | 176 | Complete listings of the sLSTM kernel tree: `src/`, `src/cuda/`, `src/util/`, `src/vanilla/` | 4.6 |  | 0.383 |
| walker |  | 8862 | 197 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: true } |  |  | 0.386 |
| ns | 8968 |  | 164 | README: the three commands that run the parity experiments | 5.1 | 1.6 | 0.384 |
| ns | 9139 |  | 171 | `experiments/main.py` — the dataset registry and the `__main__` config entry point | 5.2 |  | 0.380 |
| walker |  | 9153 | 291 | Code::CodeKey { rung: Decl, file: xlstm/components/conv.py, decl: 4, sub: 0, line: 55 } |  |  | 0.380 |
| walker |  | 9203 | 50 | Code::CodeKey { rung: Decl, file: xlstm/components/conv.py, decl: 9, sub: 0, line: 131 } |  |  | 0.380 |
| walker |  | 9259 | 56 | Code::CodeKey { rung: Decl, file: xlstm/components/conv.py, decl: 8, sub: 0, line: 110 } |  |  | 0.380 |
| walker |  | 9285 | 26 | Code::CodeKey { rung: Decl, file: xlstm/components/conv.py, decl: 7, sub: 0, line: 98 } |  |  | 0.380 |
| walker |  | 9376 | 91 | Code::CodeKey { rung: Decl, file: xlstm/blocks/mlstm/cell.py, decl: 5, sub: 0, line: 75 } |  |  | 0.380 |
| walker |  | 9384 | 8 | Code::CodeKey { rung: Body, file: experiments/lr_scheduler.py, decl: 4, sub: 0, line: 18 } |  |  | 0.380 |
| ns | 9440 |  | 301 | `experiments/parity_xlstm01.yaml` — the training and model sections of a real config | 5.3 |  | 0.372 |
| walker |  | 9497 | 113 | Code::CodeKey { rung: Decl, file: experiments/data/formal_language/tasks/parity.py, decl: 1, sub: 0, line: 9 } |  |  | 0.372 |
| walker |  | 9551 | 54 | Code::CodeKey { rung: Doc, file: xlstm/utils.py, decl: 5, sub: 0, line: 36 } |  |  | 0.372 |
| walker |  | 9604 | 53 | Code::CodeKey { rung: Doc, file: xlstm/blocks/xlstm_block.py, decl: 3, sub: 0, line: 43 } |  |  | 0.374 |
| ns | 9653 |  | 213 | `tests/conftest.py` in full, plus every test function in the suite | 5.4 |  | 0.369 |
| walker |  | 9657 | 53 | Code::CodeKey { rung: Doc, file: xlstm/components/linear_headwise.py, decl: 3, sub: 0, line: 42 } |  |  | 0.369 |
| walker |  | 9748 | 91 | Code::CodeKey { rung: Decl, file: xlstm/components/feedforward.py, decl: 1, sub: 0, line: 12 } |  |  | 0.380 |
| walker |  | 9800 | 52 | Code::CodeKey { rung: Body, file: xlstm/components/feedforward.py, decl: 9, sub: 0, line: 90 } |  |  | 0.380 |
| walker |  | 9818 | 18 | Code::CodeKey { rung: Body, file: xlstm/components/util.py, decl: 1, sub: 0, line: 7 } |  |  | 0.380 |
| ns | 9837 |  | 184 | `pyproject.toml` — project metadata, the dependency list marker, and package data | 6.1 |  | 0.388 |
| walker |  | 9843 | 25 | Code::CodeKey { rung: Names, file: xlstm/blocks/slstm/src/util/cuda_error.h, decl: 0, sub: 0, line: 0 } |  |  | 0.388 |
| walker |  | 9863 | 20 | Code::CodeKey { rung: Decl, file: xlstm/blocks/slstm/src/util/cuda_error.h, decl: 1, sub: 0, line: 9 } |  |  | 0.388 |
| walker |  | 9919 | 56 | Code::CodeKey { rung: Doc, file: xlstm/utils.py, decl: 6, sub: 0, line: 61 } |  |  | 0.388 |
| ns | 10005 |  | 168 | `pytest.ini` in full, and the README install commands | 6.2 | 1.6 | 0.385 |
