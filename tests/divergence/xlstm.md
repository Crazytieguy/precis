Score(3000)=0.492 I=0.534 C=0.454 ns_rows≤3K=16/45 grid(1000/1442/2080/3000/4327/6240/9000)=0.547/0.668/0.569/0.492/0.398/0.382/0.374

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
| walker |  | 874 | 187 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.568 |
| walker |  | 929 | 55 | Fs::DirListing { dir: tests } |  |  | 0.582 |
| ns | 968 |  | 256 | README 75-85: the 7B code is `xlstm/xlstm_large`, standalone on `mlstm_kernels` | 1.7 | 1.6 | 0.534 |
| walker |  | 1115 | 186 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.641 |
| ns | 1139 |  | 171 | Complete listings of `tests/`, `experiments/` and the `experiments/data/` tree | 1.8 |  | 0.661 |
| walker |  | 1253 | 138 | Toml::PackageMetadata { file: pyproject.toml } |  |  | 0.661 |
| ns | 1278 |  | 139 | Complete listings of `xlstm/blocks/mlstm/`, `xlstm/blocks/slstm/`, `notebooks/`, `res/`, `.github/workflows/` | 1.9 |  | 0.667 |
| walker |  | 1309 | 56 | Fs::DirListing { dir: xlstm/blocks/slstm/src/cuda } |  |  | 0.668 |
| walker |  | 1421 | 112 | Code::CodeKey { rung: Names, file: xlstm/blocks/slstm/src/vanilla/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.668 |
| walker |  | 1458 | 37 | Code::CodeKey { rung: Decl, file: xlstm/blocks/slstm/src/vanilla/__init__.py, decl: 1, sub: 0, line: 11 } |  |  | 0.668 |
| ns | 1548 |  | 270 | README quickstart: instantiate `xLSTMLargeConfig` + `xLSTMLarge` and run a forward pass | 2.1 |  | 0.615 |
| walker |  | 1632 | 174 | Code::CodeKey { rung: Decl, file: xlstm/blocks/slstm/src/vanilla/__init__.py, decl: 3, sub: 0, line: 77 } |  |  | 0.615 |
| ns | 1667 |  | 119 | Every top-level class in `xlstm/xlstm_large/model.py`, name + line | 2.2 |  | 0.594 |
| walker |  | 1808 | 176 | Code::CodeKey { rung: Decl, file: xlstm/blocks/slstm/src/vanilla/__init__.py, decl: 2, sub: 0, line: 17 } |  |  | 0.594 |
| ns | 1842 |  | 175 | `xLSTMLargeConfig` part 1/4 — the four required fields and the norm/bias toggles | 2.3 | 2.2 | 0.566 |
| walker |  | 1850 | 42 | Code::CodeKey { rung: Names, file: xlstm/xlstm_large/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.566 |
| walker |  | 2052 | 202 | Toml::Config { file: pyproject.toml } |  |  | 0.567 |
| walker |  | 2212 | 160 | Code::CodeKey { rung: Names, file: xlstm/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.602 |
| ns | 2215 |  | 373 | `xLSTMLargeConfig` part 2/4 — qk/v dim factors and kernel selection (`chunkwise_kernel`, `sequence_kernel`, `step_kernel`, `mode`) | 2.4 | 2.3 | 0.555 |
| walker |  | 2294 | 82 | Fs::DirListing { dir: xlstm/blocks/slstm/src/util } |  |  | 0.558 |
| walker |  | 2368 | 74 | Plaintext::DeclSurface { file: pytest.ini } |  |  | 0.558 |
| walker |  | 2382 | 14 | Code::CodeKey { rung: Names, file: experiments/metrics.py, decl: 0, sub: 0, line: 0 } |  |  | 0.558 |
| ns | 2458 |  | 243 | `xLSTMLargeConfig` part 3/4 — chunking, state return, and kernel dtypes | 2.5 | 2.4 | 0.532 |
| walker |  | 2485 | 103 | Code::CodeKey { rung: Decl, file: experiments/metrics.py, decl: 1, sub: 0, line: 9 } |  |  | 0.532 |
| walker |  | 2494 | 9 | Code::CodeKey { rung: Body, file: experiments/metrics.py, decl: 4, sub: 0, line: 22 } |  |  | 0.532 |
| walker |  | 2516 | 22 | Code::CodeKey { rung: Body, file: experiments/metrics.py, decl: 5, sub: 0, line: 25 } |  |  | 0.532 |
| walker |  | 2539 | 23 | Code::CodeKey { rung: Body, file: experiments/metrics.py, decl: 2, sub: 0, line: 13 } |  |  | 0.532 |
| walker |  | 2579 | 40 | Code::CodeKey { rung: Body, file: experiments/metrics.py, decl: 3, sub: 0, line: 17 } |  |  | 0.532 |
| walker |  | 2610 | 31 | Code::CodeKey { rung: Names, file: xlstm/utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.532 |
| ns | 2698 |  | 240 | `xLSTMLargeConfig` part 4/4 — feedforward sizing, soft caps, `weight_mode` | 2.6 | 2.5 | 0.512 |
| walker |  | 2733 | 123 | Code::CodeKey { rung: Decl, file: xlstm/utils.py, decl: 1, sub: 0, line: 11 } |  |  | 0.512 |
| walker |  | 2860 | 127 | Code::CodeKey { rung: Decl, file: xlstm/utils.py, decl: 3, sub: 0, line: 32 } |  |  | 0.512 |
| ns | 2891 |  | 193 | `xlstm/xlstm_large/model.py` header: the hard `mlstm_kernels` dependency and the state type aliases | 2.7 |  | 0.492 |
| walker |  | 2899 | 39 | Code::CodeKey { rung: Decl, file: xlstm/utils.py, decl: 8, sub: 0, line: 97 } |  |  | 0.492 |
| walker |  | 2909 | 10 | Code::CodeKey { rung: Body, file: xlstm/utils.py, decl: 4, sub: 0, line: 33 } |  |  | 0.492 |
| walker |  | 2963 | 54 | Code::CodeKey { rung: Doc, file: xlstm/utils.py, decl: 5, sub: 0, line: 36 } |  |  | 0.492 |
| walker |  | 3019 | 56 | Code::CodeKey { rung: Doc, file: xlstm/utils.py, decl: 6, sub: 0, line: 61 } |  |  | 0.492 |
| ns | 3038 |  | 147 | `xLSTMLarge.__init__` — embedding, backbone, lm_head | 2.8 | 2.2 | 0.479 |
| walker |  | 3050 | 31 | Code::CodeKey { rung: Names, file: xlstm/xlstm_block_stack.py, decl: 0, sub: 0, line: 0 } |  |  | 0.479 |
| walker |  | 3160 | 110 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_block_stack.py, decl: 5, sub: 0, line: 77 } |  |  | 0.479 |
| walker |  | 3216 | 56 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_block_stack.py, decl: 10, sub: 0, line: 126 } |  |  | 0.480 |
| walker |  | 3262 | 46 | Code::CodeKey { rung: Body, file: xlstm/xlstm_block_stack.py, decl: 8, sub: 0, line: 111 } |  |  | 0.480 |
| ns | 3269 |  | 231 | `xLSTMLarge.forward` — signature, shape assert, `soft_cap`, and the conditional state return | 2.9 | 2.8 | 0.460 |
| ns | 3417 |  | 148 | `xLSTMLarge.generate` — signature and delegation to `generate_tokens` | 2.10 | 2.8 | 0.448 |
| walker |  | 3569 | 307 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_block_stack.py, decl: 1, sub: 0, line: 15 } |  |  | 0.451 |
| walker |  | 3575 | 6 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_block_stack.py, decl: 2, sub: 0, line: 36 } |  |  | 0.451 |
| walker |  | 3596 | 21 | Code::CodeKey { rung: Doc, file: xlstm/xlstm_block_stack.py, decl: 3, sub: 0, line: 40 } |  |  | 0.451 |
| walker |  | 3611 | 15 | Code::CodeKey { rung: Body, file: xlstm/xlstm_block_stack.py, decl: 2, sub: 0, line: 36 } |  |  | 0.451 |
| ns | 3633 |  | 216 | `xLSTMLargeBlockStack.__init__` — the `mLSTMBlock` list and the `add_out_norm` switch | 2.11 | 2.2 | 0.434 |
| walker |  | 3676 | 65 | Code::CodeKey { rung: Names, file: experiments/main.py, decl: 0, sub: 0, line: 0 } |  |  | 0.434 |
| walker |  | 3693 | 17 | Code::CodeKey { rung: Decl, file: experiments/main.py, decl: 1, sub: 0, line: 21 } |  |  | 0.434 |
| walker |  | 3737 | 44 | Code::CodeKey { rung: Decl, file: experiments/main.py, decl: 2, sub: 0, line: 25 } |  |  | 0.434 |
| walker |  | 3765 | 28 | Code::CodeKey { rung: Body, file: experiments/main.py, decl: 3, sub: 0, line: 32 } |  |  | 0.434 |
| walker |  | 3848 | 83 | Code::CodeKey { rung: Doc, file: xlstm/utils.py, decl: 7, sub: 0, line: 77 } |  |  | 0.434 |
| ns | 3894 |  | 261 | `xLSTMLargeBlockStack.forward` — the in-place per-layer state update | 2.12 | 2.11 | 0.419 |
| walker |  | 4003 | 155 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.419 |
| walker |  | 4043 | 40 | Code::CodeKey { rung: Names, file: experiments/lr_scheduler.py, decl: 0, sub: 0, line: 0 } |  |  | 0.419 |
| walker |  | 4096 | 53 | Code::CodeKey { rung: Decl, file: experiments/lr_scheduler.py, decl: 1, sub: 0, line: 9 } |  |  | 0.419 |
| walker |  | 4105 | 9 | Code::CodeKey { rung: Decl, file: experiments/lr_scheduler.py, decl: 3, sub: 0, line: 13 } |  |  | 0.419 |
| walker |  | 4114 | 9 | Code::CodeKey { rung: Decl, file: experiments/lr_scheduler.py, decl: 4, sub: 0, line: 18 } |  |  | 0.419 |
| walker |  | 4124 | 10 | Code::CodeKey { rung: Body, file: experiments/lr_scheduler.py, decl: 3, sub: 0, line: 13 } |  |  | 0.419 |
| walker |  | 4199 | 75 | Code::CodeKey { rung: Decl, file: experiments/lr_scheduler.py, decl: 5, sub: 0, line: 24 } |  |  | 0.419 |
| walker |  | 4207 | 8 | Code::CodeKey { rung: Decl, file: experiments/lr_scheduler.py, decl: 7, sub: 0, line: 33 } |  |  | 0.419 |
| walker |  | 4221 | 14 | Code::CodeKey { rung: Doc, file: experiments/lr_scheduler.py, decl: 3, sub: 0, line: 13 } |  |  | 0.419 |
| walker |  | 4235 | 14 | Code::CodeKey { rung: Doc, file: experiments/lr_scheduler.py, decl: 4, sub: 0, line: 18 } |  |  | 0.419 |
| walker |  | 4251 | 16 | Code::CodeKey { rung: Doc, file: experiments/lr_scheduler.py, decl: 8, sub: 0, line: 47 } |  |  | 0.419 |
| walker |  | 4259 | 8 | Code::CodeKey { rung: Body, file: experiments/lr_scheduler.py, decl: 4, sub: 0, line: 18 } |  |  | 0.419 |
| ns | 4260 |  | 366 | `mLSTMBlock.__init__` (xlstm_large) — the flat-config to nested-config mapping | 2.13 | 2.2 | 0.397 |
| walker |  | 4271 | 12 | Code::CodeKey { rung: Body, file: experiments/lr_scheduler.py, decl: 2, sub: 0, line: 10 } |  |  | 0.397 |
| walker |  | 4317 | 46 | Code::CodeKey { rung: Names, file: xlstm/xlstm_lm_model.py, decl: 0, sub: 0, line: 0 } |  |  | 0.398 |
| walker |  | 4375 | 58 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_lm_model.py, decl: 1, sub: 0, line: 14 } |  |  | 0.398 |
| ns | 4421 |  | 161 | `mLSTMBlock.forward` (xlstm_large) — pre-norm + residual around layer and FFN | 2.14 | 2.13 | 0.390 |
| walker |  | 4495 | 120 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_lm_model.py, decl: 2, sub: 0, line: 22 } |  |  | 0.391 |
| walker |  | 4554 | 59 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_lm_model.py, decl: 6, sub: 0, line: 56 } |  |  | 0.391 |
| walker |  | 4612 | 58 | Code::CodeKey { rung: Body, file: xlstm/xlstm_lm_model.py, decl: 5, sub: 0, line: 49 } |  |  | 0.391 |
| walker |  | 4682 | 70 | Code::CodeKey { rung: Body, file: xlstm/xlstm_lm_model.py, decl: 6, sub: 0, line: 56 } |  |  | 0.391 |
| walker |  | 4736 | 54 | Code::CodeKey { rung: Names, file: experiments/data/utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.391 |
| walker |  | 4784 | 48 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 10, sub: 0, line: 55 } |  |  | 0.391 |
| walker |  | 4790 | 6 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 11, sub: 0, line: 57 } |  |  | 0.391 |
| walker |  | 4798 | 8 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 12, sub: 0, line: 77 } |  |  | 0.391 |
| walker |  | 4806 | 8 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 13, sub: 0, line: 85 } |  |  | 0.391 |
| walker |  | 4873 | 67 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 6, sub: 0, line: 39 } |  |  | 0.391 |
| ns | 4874 |  | 453 | Roster: every top-level class/function of the NeurIPS models and blocks, name + line | 3.1 |  | 0.378 |
| walker |  | 4881 | 8 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 8, sub: 0, line: 46 } |  |  | 0.378 |
| walker |  | 4889 | 8 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 9, sub: 0, line: 50 } |  |  | 0.378 |
| walker |  | 4966 | 77 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 1, sub: 0, line: 17 } |  |  | 0.378 |
| walker |  | 4981 | 15 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 2, sub: 0, line: 19 } |  |  | 0.378 |
| walker |  | 4998 | 17 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 3, sub: 0, line: 24 } |  |  | 0.378 |
| walker |  | 5015 | 17 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 4, sub: 0, line: 29 } |  |  | 0.378 |
| walker |  | 5032 | 17 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 5, sub: 0, line: 34 } |  |  | 0.378 |
| walker |  | 5037 | 5 | Code::CodeKey { rung: Body, file: experiments/data/utils.py, decl: 2, sub: 0, line: 19 } |  |  | 0.378 |
| ns | 5187 |  | 313 | Roster: every top-level definition in `xlstm/components/` and the sLSTM kernel's Python layer | 3.2 |  | 0.367 |
| walker |  | 5234 | 197 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.371 |
| walker |  | 5276 | 42 | Code::CodeKey { rung: Names, file: xlstm/components/util.py, decl: 0, sub: 0, line: 0 } |  |  | 0.373 |
| walker |  | 5382 | 106 | Code::CodeKey { rung: Decl, file: xlstm/components/util.py, decl: 3, sub: 0, line: 26 } |  |  | 0.373 |
| walker |  | 5390 | 8 | Code::CodeKey { rung: Decl, file: xlstm/components/util.py, decl: 8, sub: 0, line: 61 } |  |  | 0.373 |
| walker |  | 5398 | 8 | Code::CodeKey { rung: Decl, file: xlstm/components/util.py, decl: 9, sub: 0, line: 65 } |  |  | 0.373 |
| walker |  | 5406 | 8 | Code::CodeKey { rung: Decl, file: xlstm/components/util.py, decl: 10, sub: 0, line: 69 } |  |  | 0.373 |
| ns | 5447 |  | 260 | `xLSTMBlockStackConfig` — the complete field set | 3.3 | 3.1 | 0.393 |
| walker |  | 5468 | 62 | Code::CodeKey { rung: Decl, file: xlstm/components/util.py, decl: 4, sub: 0, line: 34 } |  |  | 0.393 |
| walker |  | 5492 | 24 | Code::CodeKey { rung: Doc, file: xlstm/components/util.py, decl: 2, sub: 0, line: 11 } |  |  | 0.393 |
| walker |  | 5508 | 16 | Code::CodeKey { rung: Body, file: xlstm/components/util.py, decl: 11, sub: 0, line: 73 } |  |  | 0.393 |
| walker |  | 5522 | 14 | Code::CodeKey { rung: Names, file: xlstm/xlstm_large/from_pretrained.py, decl: 0, sub: 0, line: 0 } |  |  | 0.393 |
| walker |  | 5637 | 115 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/from_pretrained.py, decl: 1, sub: 0, line: 9 } |  |  | 0.393 |
| ns | 5646 |  | 199 | `block_map` property and `_create_block_map` — how `slstm_at` becomes a per-position block type | 3.4 | 3.3 | 0.388 |
| walker |  | 5680 | 43 | Code::CodeKey { rung: Names, file: xlstm/components/conv.py, decl: 0, sub: 0, line: 0 } |  |  | 0.391 |
| walker |  | 5743 | 63 | Code::CodeKey { rung: Decl, file: xlstm/components/conv.py, decl: 3, sub: 0, line: 24 } |  |  | 0.391 |
| walker |  | 5839 | 96 | Code::CodeKey { rung: Decl, file: xlstm/components/conv.py, decl: 1, sub: 0, line: 12 } |  |  | 0.391 |
| walker |  | 5859 | 20 | Code::CodeKey { rung: Body, file: xlstm/components/conv.py, decl: 2, sub: 0, line: 20 } |  |  | 0.391 |
| ns | 5900 |  | 254 | `xLSTMBlockStackConfig.__post_init__` — the config mutation cascade | 3.5 | 3.3 | 0.382 |
| walker |  | 6150 | 291 | Code::CodeKey { rung: Decl, file: xlstm/components/conv.py, decl: 4, sub: 0, line: 55 } |  |  | 0.382 |
| walker |  | 6176 | 26 | Code::CodeKey { rung: Decl, file: xlstm/components/conv.py, decl: 7, sub: 0, line: 98 } |  |  | 0.382 |
| walker |  | 6226 | 50 | Code::CodeKey { rung: Decl, file: xlstm/components/conv.py, decl: 9, sub: 0, line: 131 } |  |  | 0.382 |
| ns | 6248 |  | 348 | `xLSTMBlockStack.__init__` and `_create_blocks` — block instantiation and `post_blocks_norm` | 3.6 | 3.1 | 0.370 |
| walker |  | 6282 | 56 | Code::CodeKey { rung: Decl, file: xlstm/components/conv.py, decl: 8, sub: 0, line: 110 } |  |  | 0.370 |
| walker |  | 6290 | 8 | Code::CodeKey { rung: Body, file: xlstm/components/conv.py, decl: 6, sub: 0, line: 95 } |  |  | 0.370 |
| walker |  | 6319 | 29 | Code::CodeKey { rung: Names, file: xlstm/blocks/xlstm_block.py, decl: 0, sub: 0, line: 0 } |  |  | 0.372 |
| walker |  | 6436 | 117 | Code::CodeKey { rung: Decl, file: xlstm/blocks/xlstm_block.py, decl: 3, sub: 0, line: 43 } |  |  | 0.372 |
| ns | 6502 |  | 254 | `xLSTMBlockStack.forward` and `step` — the loop and the `block_{i}` state dict | 3.7 | 3.6 | 0.365 |
| walker |  | 6573 | 137 | Code::CodeKey { rung: Decl, file: xlstm/blocks/xlstm_block.py, decl: 1, sub: 0, line: 16 } |  |  | 0.366 |
| walker |  | 6626 | 53 | Code::CodeKey { rung: Doc, file: xlstm/blocks/xlstm_block.py, decl: 3, sub: 0, line: 43 } |  |  | 0.366 |
| walker |  | 6685 | 59 | Code::CodeKey { rung: Body, file: xlstm/blocks/xlstm_block.py, decl: 7, sub: 0, line: 89 } |  |  | 0.366 |
| walker |  | 6714 | 29 | Code::CodeKey { rung: Names, file: xlstm/components/linear_headwise.py, decl: 0, sub: 0, line: 0 } |  |  | 0.369 |
| walker |  | 6801 | 87 | Code::CodeKey { rung: Decl, file: xlstm/components/linear_headwise.py, decl: 3, sub: 0, line: 42 } |  |  | 0.369 |
| ns | 6834 |  | 332 | `xLSTMLMModelConfig` fields, `xLSTMLMModel.__init__`, and the model's complete method roster | 3.8 | 3.1 | 0.364 |
| walker |  | 7058 | 257 | Code::CodeKey { rung: Decl, file: xlstm/components/linear_headwise.py, decl: 1, sub: 0, line: 12 } |  |  | 0.364 |
| ns | 7060 |  | 226 | `xLSTMBlockConfig` — the mLSTM-xor-sLSTM invariant | 3.9 | 3.1 | 0.366 |
| walker |  | 7111 | 53 | Code::CodeKey { rung: Doc, file: xlstm/components/linear_headwise.py, decl: 3, sub: 0, line: 42 } |  |  | 0.366 |
| walker |  | 7140 | 29 | Code::CodeKey { rung: Names, file: xlstm/components/ln.py, decl: 0, sub: 0, line: 0 } |  |  | 0.369 |
| walker |  | 7163 | 23 | Code::CodeKey { rung: Decl, file: xlstm/components/ln.py, decl: 6, sub: 0, line: 51 } |  |  | 0.369 |
| walker |  | 7222 | 59 | Code::CodeKey { rung: Decl, file: xlstm/components/ln.py, decl: 1, sub: 0, line: 8 } |  |  | 0.369 |
| walker |  | 7230 | 8 | Code::CodeKey { rung: Decl, file: xlstm/components/ln.py, decl: 3, sub: 0, line: 27 } |  |  | 0.369 |
| walker |  | 7304 | 74 | Code::CodeKey { rung: Decl, file: xlstm/components/ln.py, decl: 2, sub: 0, line: 11 } |  |  | 0.369 |
| walker |  | 7324 | 20 | Code::CodeKey { rung: Doc, file: xlstm/components/ln.py, decl: 1, sub: 0, line: 8 } |  |  | 0.369 |
| walker |  | 7329 | 5 | Code::CodeKey { rung: Body, file: experiments/data/utils.py, decl: 3, sub: 0, line: 24 } |  |  | 0.369 |
| walker |  | 7347 | 18 | Code::CodeKey { rung: Body, file: xlstm/components/util.py, decl: 1, sub: 0, line: 7 } |  |  | 0.369 |
| ns | 7423 |  | 363 | `xLSTMBlock` — class docstring, `__init__` dispatch, and its complete method roster | 3.10 | 3.1 | 0.360 |
| walker |  | 7446 | 99 | Code::CodeKey { rung: Names, file: xlstm/xlstm_large/generate.py, decl: 0, sub: 0, line: 0 } |  |  | 0.361 |
| walker |  | 7461 | 15 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/generate.py, decl: 4, sub: 0, line: 18 } |  |  | 0.361 |
| walker |  | 7490 | 29 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/generate.py, decl: 2, sub: 0, line: 8 } |  |  | 0.361 |
| walker |  | 7505 | 15 | Code::CodeKey { rung: Body, file: xlstm/xlstm_large/generate.py, decl: 5, sub: 0, line: 23 } |  |  | 0.361 |
| walker |  | 7522 | 17 | Code::CodeKey { rung: Body, file: xlstm/xlstm_large/generate.py, decl: 3, sub: 0, line: 13 } |  |  | 0.361 |
| walker |  | 7542 | 20 | Code::CodeKey { rung: Doc, file: xlstm/xlstm_large/generate.py, decl: 3, sub: 0, line: 13 } |  |  | 0.361 |
| ns | 7674 |  | 251 | Roster: every definition in `xlstm/xlstm_large/`'s support modules | 4.1 |  | 0.360 |
| walker |  | 7703 | 161 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/generate.py, decl: 6, sub: 0, line: 27 } |  |  | 0.361 |
| walker |  | 7781 | 78 | Code::CodeKey { rung: Body, file: xlstm/components/linear_headwise.py, decl: 5, sub: 0, line: 67 } |  |  | 0.361 |
| ns | 7854 |  | 180 | `mLSTMLayerConfig` (NeurIPS) — `conv1d_kernel_size`, `qkv_proj_blocksize`, `num_heads`, `proj_factor` | 4.2 |  | 0.356 |
| walker |  | 7882 | 101 | Code::CodeKey { rung: Names, file: xlstm/xlstm_large/components.py, decl: 0, sub: 0, line: 0 } |  |  | 0.366 |
| walker |  | 7903 | 21 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 9, sub: 0, line: 99 } |  |  | 0.366 |
| walker |  | 7924 | 21 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 15, sub: 0, line: 188 } |  |  | 0.366 |
| walker |  | 7967 | 43 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 12, sub: 0, line: 155 } |  |  | 0.366 |
| walker |  | 8011 | 44 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 6, sub: 0, line: 70 } |  |  | 0.366 |
| walker |  | 8058 | 47 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 11, sub: 0, line: 142 } |  |  | 0.366 |
| walker |  | 8105 | 47 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 17, sub: 0, line: 231 } |  |  | 0.366 |
| walker |  | 8159 | 54 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 2, sub: 0, line: 24 } |  |  | 0.366 |
| walker |  | 8168 | 9 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 5, sub: 0, line: 65 } |  |  | 0.366 |
| ns | 8169 |  | 315 | `sLSTMLayerConfig` fields, and the head of `sLSTMCellConfig` (`backend`, `bias_init`, `num_states`) | 4.3 | 3.1 | 0.359 |
| walker |  | 8246 | 78 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 3, sub: 0, line: 35 } |  |  | 0.359 |
| walker |  | 8334 | 88 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 10, sub: 0, line: 123 } |  |  | 0.359 |
| ns | 8400 |  | 231 | sLSTM backend dispatch: `sLSTMCell.__new__`, and the runtime CUDA build in `sLSTMCellCUDA.instance` | 4.4 | 3.1 | 0.354 |
| walker |  | 8422 | 88 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 16, sub: 0, line: 212 } |  |  | 0.354 |
| walker |  | 8430 | 8 | Code::CodeKey { rung: Body, file: xlstm/xlstm_large/components.py, decl: 5, sub: 0, line: 65 } |  |  | 0.354 |
| ns | 8628 |  | 228 | `_act_fn_registry` and `FeedForwardConfig` — the allowed `act_fn` values | 4.5 |  | 0.349 |
| walker |  | 8632 | 202 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.349 |
| walker |  | 8799 | 167 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.349 |
| ns | 8804 |  | 176 | Complete listings of the sLSTM kernel tree: `src/`, `src/cuda/`, `src/util/`, `src/vanilla/` | 4.6 |  | 0.367 |
| ns | 8968 |  | 164 | README: the three commands that run the parity experiments | 5.1 | 1.6 | 0.365 |
| walker |  | 8969 | 170 | Code::CodeKey { rung: Names, file: xlstm/xlstm_large/model.py, decl: 0, sub: 0, line: 0 } |  |  | 0.374 |
| walker |  | 8999 | 30 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 16, sub: 0, line: 310 } |  |  | 0.374 |
| walker |  | 9029 | 30 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 19, sub: 0, line: 455 } |  |  | 0.374 |
| walker |  | 9069 | 40 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 12, sub: 0, line: 232 } |  |  | 0.374 |
| walker |  | 9111 | 42 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 21, sub: 0, line: 499 } |  |  | 0.375 |
| ns | 9139 |  | 171 | `experiments/main.py` — the dataset registry and the `__main__` config entry point | 5.2 |  | 0.372 |
| walker |  | 9159 | 48 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 18, sub: 0, line: 379 } |  |  | 0.372 |
| walker |  | 9209 | 50 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 9, sub: 0, line: 187 } |  |  | 0.372 |
| walker |  | 9251 | 42 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 11, sub: 0, line: 209 } |  |  | 0.373 |
| walker |  | 9311 | 60 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 5, sub: 0, line: 112 } |  |  | 0.373 |
| walker |  | 9356 | 45 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 7, sub: 0, line: 126 } |  |  | 0.374 |
| walker |  | 9434 | 78 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 8, sub: 0, line: 155 } |  |  | 0.378 |
| ns | 9440 |  | 301 | `experiments/parity_xlstm01.yaml` — the training and model sections of a real config | 5.3 |  | 0.370 |
| ns | 9653 |  | 213 | `tests/conftest.py` in full, plus every test function in the suite | 5.4 |  | 0.366 |
| walker |  | 9814 | 380 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 15, sub: 0, line: 280 } |  |  | 0.368 |
| walker |  | 9819 | 5 | Code::CodeKey { rung: Body, file: experiments/data/utils.py, decl: 4, sub: 0, line: 29 } |  |  | 0.368 |
| ns | 9837 |  | 184 | `pyproject.toml` — project metadata, the dependency list marker, and package data | 6.1 |  | 0.376 |
| walker |  | 9894 | 75 | Code::CodeKey { rung: Body, file: xlstm/xlstm_lm_model.py, decl: 4, sub: 0, line: 41 } |  |  | 0.376 |
| walker |  | 9983 | 89 | Code::CodeKey { rung: Names, file: xlstm/components/feedforward.py, decl: 0, sub: 0, line: 0 } |  |  | 0.381 |
| walker |  | 10000 | 17 | Code::CodeKey { rung: Decl, file: xlstm/components/feedforward.py, decl: 5, sub: 0, line: 49 } |  |  | 0.381 |
| ns | 10005 |  | 168 | `pytest.ini` in full, and the README install commands | 6.2 | 1.6 | 0.379 |
