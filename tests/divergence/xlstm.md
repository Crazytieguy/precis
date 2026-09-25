Score(3000)=0.492 I=0.532 C=0.454 ns_rows≤3K=16/45 grid(1000/1442/2080/3000/4327/6240/9000)=0.641/0.668/0.604/0.492/0.397/0.383/0.375

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
| walker |  | 186 | 22 | Fs::DirListing { dir: xlstm/blocks } |  |  | 0.625 |
| walker |  | 209 | 23 | Fs::DirListing { dir: xlstm/blocks/mlstm } |  |  | 0.633 |
| ns | 225 |  | 64 | README: the 7B model and the name "xLSTM Large" | 1.3 |  | 0.582 |
| walker |  | 232 | 23 | Fs::DirListing { dir: xlstm/blocks/slstm } |  |  | 0.592 |
| walker |  | 254 | 22 | Fs::DirListing { dir: xlstm/blocks/slstm/src } |  |  | 0.592 |
| walker |  | 270 | 16 | Fs::DirListing { dir: xlstm/blocks/slstm/src/vanilla } |  |  | 0.592 |
| walker |  | 287 | 17 | Code::CodeKey { rung: ModuleDoc, file: xlstm/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.593 |
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
| walker |  | 742 | 55 | Fs::DirListing { dir: tests } |  |  | 0.582 |
| walker |  | 928 | 186 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.699 |
| ns | 968 |  | 256 | README 75-85: the 7B code is `xlstm/xlstm_large`, standalone on `mlstm_kernels` | 1.7 | 1.6 | 0.641 |
| walker |  | 1115 | 187 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.641 |
| ns | 1139 |  | 171 | Complete listings of `tests/`, `experiments/` and the `experiments/data/` tree | 1.8 |  | 0.661 |
| walker |  | 1171 | 56 | Fs::DirListing { dir: xlstm/blocks/slstm/src/cuda } |  |  | 0.662 |
| ns | 1278 |  | 139 | Complete listings of `xlstm/blocks/mlstm/`, `xlstm/blocks/slstm/`, `notebooks/`, `res/`, `.github/workflows/` | 1.9 |  | 0.668 |
| walker |  | 1283 | 112 | Code::CodeKey { rung: Names, file: xlstm/blocks/slstm/src/vanilla/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.668 |
| walker |  | 1320 | 37 | Code::CodeKey { rung: Decl, file: xlstm/blocks/slstm/src/vanilla/__init__.py, decl: 1, sub: 0, line: 11 } |  |  | 0.668 |
| walker |  | 1494 | 174 | Code::CodeKey { rung: Decl, file: xlstm/blocks/slstm/src/vanilla/__init__.py, decl: 3, sub: 0, line: 77 } |  |  | 0.668 |
| ns | 1548 |  | 270 | README quickstart: instantiate `xLSTMLargeConfig` + `xLSTMLarge` and run a forward pass | 2.1 |  | 0.615 |
| ns | 1667 |  | 119 | Every top-level class in `xlstm/xlstm_large/model.py`, name + line | 2.2 |  | 0.594 |
| walker |  | 1670 | 176 | Code::CodeKey { rung: Decl, file: xlstm/blocks/slstm/src/vanilla/__init__.py, decl: 2, sub: 0, line: 17 } |  |  | 0.594 |
| walker |  | 1712 | 42 | Code::CodeKey { rung: Names, file: xlstm/xlstm_large/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.594 |
| ns | 1842 |  | 175 | `xLSTMLargeConfig` part 1/4 — the four required fields and the norm/bias toggles | 2.3 | 2.2 | 0.566 |
| walker |  | 1872 | 160 | Code::CodeKey { rung: Names, file: xlstm/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.601 |
| walker |  | 1954 | 82 | Fs::DirListing { dir: xlstm/blocks/slstm/src/util } |  |  | 0.603 |
| walker |  | 2028 | 74 | Plaintext::DeclSurface { file: pytest.ini } |  |  | 0.604 |
| walker |  | 2166 | 138 | Toml::PackageMetadata { file: pyproject.toml } |  |  | 0.604 |
| walker |  | 2180 | 14 | Code::CodeKey { rung: Names, file: experiments/metrics.py, decl: 0, sub: 0, line: 0 } |  |  | 0.604 |
| ns | 2215 |  | 373 | `xLSTMLargeConfig` part 2/4 — qk/v dim factors and kernel selection (`chunkwise_kernel`, `sequence_kernel`, `step_kernel`, `mode`) | 2.4 | 2.3 | 0.557 |
| walker |  | 2283 | 103 | Code::CodeKey { rung: Decl, file: experiments/metrics.py, decl: 1, sub: 0, line: 9 } |  |  | 0.557 |
| walker |  | 2292 | 9 | Code::CodeKey { rung: Body, file: experiments/metrics.py, decl: 4, sub: 0, line: 22 } |  |  | 0.557 |
| walker |  | 2314 | 22 | Code::CodeKey { rung: Body, file: experiments/metrics.py, decl: 5, sub: 0, line: 25 } |  |  | 0.557 |
| walker |  | 2337 | 23 | Code::CodeKey { rung: Body, file: experiments/metrics.py, decl: 2, sub: 0, line: 13 } |  |  | 0.557 |
| walker |  | 2377 | 40 | Code::CodeKey { rung: Body, file: experiments/metrics.py, decl: 3, sub: 0, line: 17 } |  |  | 0.557 |
| ns | 2458 |  | 243 | `xLSTMLargeConfig` part 3/4 — chunking, state return, and kernel dtypes | 2.5 | 2.4 | 0.531 |
| walker |  | 2467 | 90 | Plaintext::DeclSurface { file: setup.cfg } |  |  | 0.531 |
| walker |  | 2498 | 31 | Code::CodeKey { rung: Names, file: xlstm/utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.531 |
| walker |  | 2621 | 123 | Code::CodeKey { rung: Decl, file: xlstm/utils.py, decl: 1, sub: 0, line: 11 } |  |  | 0.531 |
| ns | 2698 |  | 240 | `xLSTMLargeConfig` part 4/4 — feedforward sizing, soft caps, `weight_mode` | 2.6 | 2.5 | 0.511 |
| walker |  | 2748 | 127 | Code::CodeKey { rung: Decl, file: xlstm/utils.py, decl: 3, sub: 0, line: 32 } |  |  | 0.511 |
| walker |  | 2787 | 39 | Code::CodeKey { rung: Decl, file: xlstm/utils.py, decl: 8, sub: 0, line: 97 } |  |  | 0.511 |
| walker |  | 2797 | 10 | Code::CodeKey { rung: Body, file: xlstm/utils.py, decl: 4, sub: 0, line: 33 } |  |  | 0.511 |
| walker |  | 2851 | 54 | Code::CodeKey { rung: Doc, file: xlstm/utils.py, decl: 5, sub: 0, line: 36 } |  |  | 0.511 |
| ns | 2891 |  | 193 | `xlstm/xlstm_large/model.py` header: the hard `mlstm_kernels` dependency and the state type aliases | 2.7 |  | 0.491 |
| walker |  | 2907 | 56 | Code::CodeKey { rung: Doc, file: xlstm/utils.py, decl: 6, sub: 0, line: 61 } |  |  | 0.491 |
| walker |  | 2938 | 31 | Code::CodeKey { rung: Names, file: xlstm/xlstm_block_stack.py, decl: 0, sub: 0, line: 0 } |  |  | 0.492 |
| ns | 3038 |  | 147 | `xLSTMLarge.__init__` — embedding, backbone, lm_head | 2.8 | 2.2 | 0.478 |
| walker |  | 3048 | 110 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_block_stack.py, decl: 5, sub: 0, line: 77 } |  |  | 0.478 |
| walker |  | 3104 | 56 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_block_stack.py, decl: 10, sub: 0, line: 126 } |  |  | 0.479 |
| walker |  | 3150 | 46 | Code::CodeKey { rung: Body, file: xlstm/xlstm_block_stack.py, decl: 8, sub: 0, line: 111 } |  |  | 0.479 |
| ns | 3269 |  | 231 | `xLSTMLarge.forward` — signature, shape assert, `soft_cap`, and the conditional state return | 2.9 | 2.8 | 0.459 |
| ns | 3417 |  | 148 | `xLSTMLarge.generate` — signature and delegation to `generate_tokens` | 2.10 | 2.8 | 0.447 |
| walker |  | 3457 | 307 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_block_stack.py, decl: 1, sub: 0, line: 15 } |  |  | 0.450 |
| walker |  | 3463 | 6 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_block_stack.py, decl: 2, sub: 0, line: 36 } |  |  | 0.450 |
| walker |  | 3484 | 21 | Code::CodeKey { rung: Doc, file: xlstm/xlstm_block_stack.py, decl: 3, sub: 0, line: 40 } |  |  | 0.450 |
| walker |  | 3499 | 15 | Code::CodeKey { rung: Body, file: xlstm/xlstm_block_stack.py, decl: 2, sub: 0, line: 36 } |  |  | 0.450 |
| walker |  | 3564 | 65 | Code::CodeKey { rung: Names, file: experiments/main.py, decl: 0, sub: 0, line: 0 } |  |  | 0.450 |
| walker |  | 3581 | 17 | Code::CodeKey { rung: Decl, file: experiments/main.py, decl: 1, sub: 0, line: 21 } |  |  | 0.450 |
| walker |  | 3625 | 44 | Code::CodeKey { rung: Decl, file: experiments/main.py, decl: 2, sub: 0, line: 25 } |  |  | 0.450 |
| ns | 3633 |  | 216 | `xLSTMLargeBlockStack.__init__` — the `mLSTMBlock` list and the `add_out_norm` switch | 2.11 | 2.2 | 0.433 |
| walker |  | 3653 | 28 | Code::CodeKey { rung: Body, file: experiments/main.py, decl: 3, sub: 0, line: 32 } |  |  | 0.433 |
| walker |  | 3736 | 83 | Code::CodeKey { rung: Doc, file: xlstm/utils.py, decl: 7, sub: 0, line: 77 } |  |  | 0.433 |
| walker |  | 3891 | 155 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.433 |
| ns | 3894 |  | 261 | `xLSTMLargeBlockStack.forward` — the in-place per-layer state update | 2.12 | 2.11 | 0.418 |
| walker |  | 3931 | 40 | Code::CodeKey { rung: Names, file: experiments/lr_scheduler.py, decl: 0, sub: 0, line: 0 } |  |  | 0.418 |
| walker |  | 3984 | 53 | Code::CodeKey { rung: Decl, file: experiments/lr_scheduler.py, decl: 1, sub: 0, line: 9 } |  |  | 0.418 |
| walker |  | 3993 | 9 | Code::CodeKey { rung: Decl, file: experiments/lr_scheduler.py, decl: 3, sub: 0, line: 13 } |  |  | 0.418 |
| walker |  | 4002 | 9 | Code::CodeKey { rung: Decl, file: experiments/lr_scheduler.py, decl: 4, sub: 0, line: 18 } |  |  | 0.418 |
| walker |  | 4012 | 10 | Code::CodeKey { rung: Body, file: experiments/lr_scheduler.py, decl: 3, sub: 0, line: 13 } |  |  | 0.418 |
| walker |  | 4087 | 75 | Code::CodeKey { rung: Decl, file: experiments/lr_scheduler.py, decl: 5, sub: 0, line: 24 } |  |  | 0.418 |
| walker |  | 4095 | 8 | Code::CodeKey { rung: Decl, file: experiments/lr_scheduler.py, decl: 7, sub: 0, line: 33 } |  |  | 0.418 |
| walker |  | 4109 | 14 | Code::CodeKey { rung: Doc, file: experiments/lr_scheduler.py, decl: 3, sub: 0, line: 13 } |  |  | 0.418 |
| walker |  | 4123 | 14 | Code::CodeKey { rung: Doc, file: experiments/lr_scheduler.py, decl: 4, sub: 0, line: 18 } |  |  | 0.418 |
| walker |  | 4139 | 16 | Code::CodeKey { rung: Doc, file: experiments/lr_scheduler.py, decl: 8, sub: 0, line: 47 } |  |  | 0.418 |
| walker |  | 4147 | 8 | Code::CodeKey { rung: Body, file: experiments/lr_scheduler.py, decl: 4, sub: 0, line: 18 } |  |  | 0.418 |
| walker |  | 4159 | 12 | Code::CodeKey { rung: Body, file: experiments/lr_scheduler.py, decl: 2, sub: 0, line: 10 } |  |  | 0.418 |
| walker |  | 4205 | 46 | Code::CodeKey { rung: Names, file: xlstm/xlstm_lm_model.py, decl: 0, sub: 0, line: 0 } |  |  | 0.418 |
| ns | 4260 |  | 366 | `mLSTMBlock.__init__` (xlstm_large) — the flat-config to nested-config mapping | 2.13 | 2.2 | 0.397 |
| walker |  | 4263 | 58 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_lm_model.py, decl: 1, sub: 0, line: 14 } |  |  | 0.397 |
| walker |  | 4383 | 120 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_lm_model.py, decl: 2, sub: 0, line: 22 } |  |  | 0.397 |
| ns | 4421 |  | 161 | `mLSTMBlock.forward` (xlstm_large) — pre-norm + residual around layer and FFN | 2.14 | 2.13 | 0.390 |
| walker |  | 4442 | 59 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_lm_model.py, decl: 6, sub: 0, line: 56 } |  |  | 0.390 |
| walker |  | 4500 | 58 | Code::CodeKey { rung: Body, file: xlstm/xlstm_lm_model.py, decl: 5, sub: 0, line: 49 } |  |  | 0.390 |
| walker |  | 4570 | 70 | Code::CodeKey { rung: Body, file: xlstm/xlstm_lm_model.py, decl: 6, sub: 0, line: 56 } |  |  | 0.390 |
| walker |  | 4624 | 54 | Code::CodeKey { rung: Names, file: experiments/data/utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.390 |
| walker |  | 4672 | 48 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 10, sub: 0, line: 55 } |  |  | 0.390 |
| walker |  | 4678 | 6 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 11, sub: 0, line: 57 } |  |  | 0.390 |
| walker |  | 4686 | 8 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 12, sub: 0, line: 77 } |  |  | 0.390 |
| walker |  | 4694 | 8 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 13, sub: 0, line: 85 } |  |  | 0.390 |
| walker |  | 4761 | 67 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 6, sub: 0, line: 39 } |  |  | 0.390 |
| walker |  | 4769 | 8 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 8, sub: 0, line: 46 } |  |  | 0.390 |
| walker |  | 4777 | 8 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 9, sub: 0, line: 50 } |  |  | 0.390 |
| walker |  | 4854 | 77 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 1, sub: 0, line: 17 } |  |  | 0.390 |
| walker |  | 4869 | 15 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 2, sub: 0, line: 19 } |  |  | 0.390 |
| ns | 4874 |  | 453 | Roster: every top-level class/function of the NeurIPS models and blocks, name + line | 3.1 |  | 0.377 |
| walker |  | 4886 | 17 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 3, sub: 0, line: 24 } |  |  | 0.377 |
| walker |  | 4903 | 17 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 4, sub: 0, line: 29 } |  |  | 0.377 |
| walker |  | 4920 | 17 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 5, sub: 0, line: 34 } |  |  | 0.377 |
| walker |  | 4925 | 5 | Code::CodeKey { rung: Body, file: experiments/data/utils.py, decl: 2, sub: 0, line: 19 } |  |  | 0.377 |
| walker |  | 5122 | 197 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.381 |
| walker |  | 5164 | 42 | Code::CodeKey { rung: Names, file: xlstm/components/util.py, decl: 0, sub: 0, line: 0 } |  |  | 0.381 |
| ns | 5187 |  | 313 | Roster: every top-level definition in `xlstm/components/` and the sLSTM kernel's Python layer | 3.2 |  | 0.372 |
| walker |  | 5270 | 106 | Code::CodeKey { rung: Decl, file: xlstm/components/util.py, decl: 3, sub: 0, line: 26 } |  |  | 0.372 |
| walker |  | 5278 | 8 | Code::CodeKey { rung: Decl, file: xlstm/components/util.py, decl: 8, sub: 0, line: 61 } |  |  | 0.372 |
| walker |  | 5286 | 8 | Code::CodeKey { rung: Decl, file: xlstm/components/util.py, decl: 9, sub: 0, line: 65 } |  |  | 0.372 |
| walker |  | 5294 | 8 | Code::CodeKey { rung: Decl, file: xlstm/components/util.py, decl: 10, sub: 0, line: 69 } |  |  | 0.372 |
| walker |  | 5356 | 62 | Code::CodeKey { rung: Decl, file: xlstm/components/util.py, decl: 4, sub: 0, line: 34 } |  |  | 0.372 |
| walker |  | 5380 | 24 | Code::CodeKey { rung: Doc, file: xlstm/components/util.py, decl: 2, sub: 0, line: 11 } |  |  | 0.372 |
| walker |  | 5396 | 16 | Code::CodeKey { rung: Body, file: xlstm/components/util.py, decl: 11, sub: 0, line: 73 } |  |  | 0.372 |
| walker |  | 5410 | 14 | Code::CodeKey { rung: Names, file: xlstm/xlstm_large/from_pretrained.py, decl: 0, sub: 0, line: 0 } |  |  | 0.372 |
| ns | 5447 |  | 260 | `xLSTMBlockStackConfig` — the complete field set | 3.3 | 3.1 | 0.392 |
| walker |  | 5525 | 115 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/from_pretrained.py, decl: 1, sub: 0, line: 9 } |  |  | 0.392 |
| walker |  | 5568 | 43 | Code::CodeKey { rung: Names, file: xlstm/components/conv.py, decl: 0, sub: 0, line: 0 } |  |  | 0.395 |
| walker |  | 5631 | 63 | Code::CodeKey { rung: Decl, file: xlstm/components/conv.py, decl: 3, sub: 0, line: 24 } |  |  | 0.395 |
| ns | 5646 |  | 199 | `block_map` property and `_create_block_map` — how `slstm_at` becomes a per-position block type | 3.4 | 3.3 | 0.390 |
| walker |  | 5727 | 96 | Code::CodeKey { rung: Decl, file: xlstm/components/conv.py, decl: 1, sub: 0, line: 12 } |  |  | 0.390 |
| walker |  | 5747 | 20 | Code::CodeKey { rung: Body, file: xlstm/components/conv.py, decl: 2, sub: 0, line: 20 } |  |  | 0.390 |
| ns | 5900 |  | 254 | `xLSTMBlockStackConfig.__post_init__` — the config mutation cascade | 3.5 | 3.3 | 0.381 |
| walker |  | 6038 | 291 | Code::CodeKey { rung: Decl, file: xlstm/components/conv.py, decl: 4, sub: 0, line: 55 } |  |  | 0.381 |
| walker |  | 6064 | 26 | Code::CodeKey { rung: Decl, file: xlstm/components/conv.py, decl: 7, sub: 0, line: 98 } |  |  | 0.381 |
| walker |  | 6114 | 50 | Code::CodeKey { rung: Decl, file: xlstm/components/conv.py, decl: 9, sub: 0, line: 131 } |  |  | 0.381 |
| walker |  | 6170 | 56 | Code::CodeKey { rung: Decl, file: xlstm/components/conv.py, decl: 8, sub: 0, line: 110 } |  |  | 0.381 |
| walker |  | 6178 | 8 | Code::CodeKey { rung: Body, file: xlstm/components/conv.py, decl: 6, sub: 0, line: 95 } |  |  | 0.381 |
| walker |  | 6207 | 29 | Code::CodeKey { rung: Names, file: xlstm/blocks/xlstm_block.py, decl: 0, sub: 0, line: 0 } |  |  | 0.383 |
| ns | 6248 |  | 348 | `xLSTMBlockStack.__init__` and `_create_blocks` — block instantiation and `post_blocks_norm` | 3.6 | 3.1 | 0.371 |
| walker |  | 6324 | 117 | Code::CodeKey { rung: Decl, file: xlstm/blocks/xlstm_block.py, decl: 3, sub: 0, line: 43 } |  |  | 0.371 |
| walker |  | 6461 | 137 | Code::CodeKey { rung: Decl, file: xlstm/blocks/xlstm_block.py, decl: 1, sub: 0, line: 16 } |  |  | 0.372 |
| ns | 6502 |  | 254 | `xLSTMBlockStack.forward` and `step` — the loop and the `block_{i}` state dict | 3.7 | 3.6 | 0.365 |
| walker |  | 6514 | 53 | Code::CodeKey { rung: Doc, file: xlstm/blocks/xlstm_block.py, decl: 3, sub: 0, line: 43 } |  |  | 0.365 |
| walker |  | 6573 | 59 | Code::CodeKey { rung: Body, file: xlstm/blocks/xlstm_block.py, decl: 7, sub: 0, line: 89 } |  |  | 0.365 |
| walker |  | 6602 | 29 | Code::CodeKey { rung: Names, file: xlstm/components/linear_headwise.py, decl: 0, sub: 0, line: 0 } |  |  | 0.368 |
| walker |  | 6689 | 87 | Code::CodeKey { rung: Decl, file: xlstm/components/linear_headwise.py, decl: 3, sub: 0, line: 42 } |  |  | 0.368 |
| ns | 6834 |  | 332 | `xLSTMLMModelConfig` fields, `xLSTMLMModel.__init__`, and the model's complete method roster | 3.8 | 3.1 | 0.364 |
| walker |  | 6946 | 257 | Code::CodeKey { rung: Decl, file: xlstm/components/linear_headwise.py, decl: 1, sub: 0, line: 12 } |  |  | 0.364 |
| walker |  | 6999 | 53 | Code::CodeKey { rung: Doc, file: xlstm/components/linear_headwise.py, decl: 3, sub: 0, line: 42 } |  |  | 0.364 |
| walker |  | 7028 | 29 | Code::CodeKey { rung: Names, file: xlstm/components/ln.py, decl: 0, sub: 0, line: 0 } |  |  | 0.367 |
| walker |  | 7051 | 23 | Code::CodeKey { rung: Decl, file: xlstm/components/ln.py, decl: 6, sub: 0, line: 51 } |  |  | 0.367 |
| ns | 7060 |  | 226 | `xLSTMBlockConfig` — the mLSTM-xor-sLSTM invariant | 3.9 | 3.1 | 0.368 |
| walker |  | 7110 | 59 | Code::CodeKey { rung: Decl, file: xlstm/components/ln.py, decl: 1, sub: 0, line: 8 } |  |  | 0.368 |
| walker |  | 7118 | 8 | Code::CodeKey { rung: Decl, file: xlstm/components/ln.py, decl: 3, sub: 0, line: 27 } |  |  | 0.368 |
| walker |  | 7192 | 74 | Code::CodeKey { rung: Decl, file: xlstm/components/ln.py, decl: 2, sub: 0, line: 11 } |  |  | 0.368 |
| walker |  | 7212 | 20 | Code::CodeKey { rung: Doc, file: xlstm/components/ln.py, decl: 1, sub: 0, line: 8 } |  |  | 0.368 |
| walker |  | 7217 | 5 | Code::CodeKey { rung: Body, file: experiments/data/utils.py, decl: 3, sub: 0, line: 24 } |  |  | 0.368 |
| walker |  | 7235 | 18 | Code::CodeKey { rung: Body, file: xlstm/components/util.py, decl: 1, sub: 0, line: 7 } |  |  | 0.368 |
| walker |  | 7334 | 99 | Code::CodeKey { rung: Names, file: xlstm/xlstm_large/generate.py, decl: 0, sub: 0, line: 0 } |  |  | 0.368 |
| walker |  | 7349 | 15 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/generate.py, decl: 4, sub: 0, line: 18 } |  |  | 0.368 |
| walker |  | 7378 | 29 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/generate.py, decl: 2, sub: 0, line: 8 } |  |  | 0.369 |
| walker |  | 7393 | 15 | Code::CodeKey { rung: Body, file: xlstm/xlstm_large/generate.py, decl: 5, sub: 0, line: 23 } |  |  | 0.369 |
| walker |  | 7410 | 17 | Code::CodeKey { rung: Body, file: xlstm/xlstm_large/generate.py, decl: 3, sub: 0, line: 13 } |  |  | 0.369 |
| ns | 7423 |  | 363 | `xLSTMBlock` — class docstring, `__init__` dispatch, and its complete method roster | 3.10 | 3.1 | 0.360 |
| walker |  | 7430 | 20 | Code::CodeKey { rung: Doc, file: xlstm/xlstm_large/generate.py, decl: 3, sub: 0, line: 13 } |  |  | 0.360 |
| walker |  | 7591 | 161 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/generate.py, decl: 6, sub: 0, line: 27 } |  |  | 0.360 |
| walker |  | 7669 | 78 | Code::CodeKey { rung: Body, file: xlstm/components/linear_headwise.py, decl: 5, sub: 0, line: 67 } |  |  | 0.360 |
| ns | 7674 |  | 251 | Roster: every definition in `xlstm/xlstm_large/`'s support modules | 4.1 |  | 0.361 |
| walker |  | 7770 | 101 | Code::CodeKey { rung: Names, file: xlstm/xlstm_large/components.py, decl: 0, sub: 0, line: 0 } |  |  | 0.371 |
| walker |  | 7791 | 21 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 9, sub: 0, line: 99 } |  |  | 0.371 |
| walker |  | 7812 | 21 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 15, sub: 0, line: 188 } |  |  | 0.371 |
| ns | 7854 |  | 180 | `mLSTMLayerConfig` (NeurIPS) — `conv1d_kernel_size`, `qkv_proj_blocksize`, `num_heads`, `proj_factor` | 4.2 |  | 0.365 |
| walker |  | 7855 | 43 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 12, sub: 0, line: 155 } |  |  | 0.365 |
| walker |  | 7899 | 44 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 6, sub: 0, line: 70 } |  |  | 0.365 |
| walker |  | 7946 | 47 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 11, sub: 0, line: 142 } |  |  | 0.365 |
| walker |  | 7993 | 47 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 17, sub: 0, line: 231 } |  |  | 0.365 |
| walker |  | 8047 | 54 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 2, sub: 0, line: 24 } |  |  | 0.365 |
| walker |  | 8056 | 9 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 5, sub: 0, line: 65 } |  |  | 0.365 |
| walker |  | 8134 | 78 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 3, sub: 0, line: 35 } |  |  | 0.365 |
| ns | 8169 |  | 315 | `sLSTMLayerConfig` fields, and the head of `sLSTMCellConfig` (`backend`, `bias_init`, `num_states`) | 4.3 | 3.1 | 0.358 |
| walker |  | 8222 | 88 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 10, sub: 0, line: 123 } |  |  | 0.358 |
| walker |  | 8310 | 88 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 16, sub: 0, line: 212 } |  |  | 0.358 |
| walker |  | 8318 | 8 | Code::CodeKey { rung: Body, file: xlstm/xlstm_large/components.py, decl: 5, sub: 0, line: 65 } |  |  | 0.358 |
| ns | 8400 |  | 231 | sLSTM backend dispatch: `sLSTMCell.__new__`, and the runtime CUDA build in `sLSTMCellCUDA.instance` | 4.4 | 3.1 | 0.353 |
| walker |  | 8520 | 202 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.353 |
| ns | 8628 |  | 228 | `_act_fn_registry` and `FeedForwardConfig` — the allowed `act_fn` values | 4.5 |  | 0.348 |
| walker |  | 8687 | 167 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.348 |
| ns | 8804 |  | 176 | Complete listings of the sLSTM kernel tree: `src/`, `src/cuda/`, `src/util/`, `src/vanilla/` | 4.6 |  | 0.366 |
| walker |  | 8857 | 170 | Code::CodeKey { rung: Names, file: xlstm/xlstm_large/model.py, decl: 0, sub: 0, line: 0 } |  |  | 0.376 |
| walker |  | 8887 | 30 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 16, sub: 0, line: 310 } |  |  | 0.376 |
| walker |  | 8917 | 30 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 19, sub: 0, line: 455 } |  |  | 0.376 |
| walker |  | 8957 | 40 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 12, sub: 0, line: 232 } |  |  | 0.376 |
| ns | 8968 |  | 164 | README: the three commands that run the parity experiments | 5.1 | 1.6 | 0.374 |
| walker |  | 8999 | 42 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 21, sub: 0, line: 499 } |  |  | 0.375 |
| walker |  | 9047 | 48 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 18, sub: 0, line: 379 } |  |  | 0.375 |
| walker |  | 9097 | 50 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 9, sub: 0, line: 187 } |  |  | 0.375 |
| walker |  | 9139 | 42 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 11, sub: 0, line: 209 } |  |  | 0.372 |
| ns | 9139 |  | 171 | `experiments/main.py` — the dataset registry and the `__main__` config entry point | 5.2 |  | 0.372 |
| walker |  | 9199 | 60 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 5, sub: 0, line: 112 } |  |  | 0.373 |
| walker |  | 9244 | 45 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 7, sub: 0, line: 126 } |  |  | 0.373 |
| walker |  | 9322 | 78 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 8, sub: 0, line: 155 } |  |  | 0.377 |
| ns | 9440 |  | 301 | `experiments/parity_xlstm01.yaml` — the training and model sections of a real config | 5.3 |  | 0.370 |
| ns | 9653 |  | 213 | `tests/conftest.py` in full, plus every test function in the suite | 5.4 |  | 0.365 |
| walker |  | 9702 | 380 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 15, sub: 0, line: 280 } |  |  | 0.367 |
| walker |  | 9707 | 5 | Code::CodeKey { rung: Body, file: experiments/data/utils.py, decl: 4, sub: 0, line: 29 } |  |  | 0.367 |
| walker |  | 9782 | 75 | Code::CodeKey { rung: Body, file: xlstm/xlstm_lm_model.py, decl: 4, sub: 0, line: 41 } |  |  | 0.367 |
| ns | 9837 |  | 184 | `pyproject.toml` — project metadata, the dependency list marker, and package data | 6.1 |  | 0.367 |
| walker |  | 9871 | 89 | Code::CodeKey { rung: Names, file: xlstm/components/feedforward.py, decl: 0, sub: 0, line: 0 } |  |  | 0.372 |
| walker |  | 9940 | 69 | Code::CodeKey { rung: Decl, file: xlstm/components/feedforward.py, decl: 5, sub: 0, line: 49 } |  |  | 0.372 |
| walker |  | 10000 | 60 | Code::CodeKey { rung: Decl, file: xlstm/components/feedforward.py, decl: 1, sub: 0, line: 12 } |  |  | 0.374 |
| ns | 10005 |  | 168 | `pytest.ini` in full, and the README install commands | 6.2 | 1.6 | 0.372 |
