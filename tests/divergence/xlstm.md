Score(3000)=0.502 I=0.547 C=0.461 ns_rows≤3K=16/45 grid(1000/1442/2080/3000/4327/6240/9000)=0.641/0.710/0.605/0.502/0.432/0.515/0.602

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 65 | 65 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 96 |  | 96 | What xLSTM is, from the README lede | 1.1 |  | 0.000 |
| walker |  | 101 | 36 | Fs::DirListing { dir: xlstm } |  |  | 0.000 |
| walker |  | 123 | 22 | Fs::DirListing { dir: xlstm/blocks } |  |  | 0.000 |
| walker |  | 146 | 23 | Fs::DirListing { dir: xlstm/blocks/slstm } |  |  | 0.000 |
| ns | 161 |  | 65 | Complete repository-root listing | 1.2 |  | 0.622 |
| walker |  | 168 | 22 | Fs::DirListing { dir: xlstm/blocks/slstm/src } |  |  | 0.622 |
| walker |  | 177 | 9 | Fs::DirListing { dir: notebooks } |  |  | 0.624 |
| ns | 225 |  | 64 | README: the 7B model and the name "xLSTM Large" | 1.3 |  | 0.573 |
| walker |  | 280 | 103 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.592 |
| ns | 346 |  | 121 | Complete listings of `xlstm/`, `xlstm/blocks/`, `xlstm/components/`, `xlstm/xlstm_large/` | 1.4 |  | 0.438 |
| walker |  | 365 | 85 | Toml::Identity { file: pyproject.toml } |  |  | 0.438 |
| walker |  | 381 | 16 | Fs::DirListing { dir: xlstm/blocks/slstm/src/vanilla } |  |  | 0.439 |
| walker |  | 398 | 17 | Code::CodeKey { rung: ModuleDoc, file: xlstm/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.439 |
| walker |  | 421 | 23 | Fs::DirListing { dir: xlstm/blocks/mlstm } |  |  | 0.443 |
| walker |  | 449 | 28 | Fs::DirListing { dir: res } |  |  | 0.447 |
| walker |  | 477 | 28 | Fs::DirListing { dir: xlstm/xlstm_large } |  |  | 0.537 |
| walker |  | 512 | 35 | Fs::DirListing { dir: xlstm/components } |  |  | 0.685 |
| walker |  | 527 | 15 | Fs::DirListing { dir: notebooks/xlstm_large } |  |  | 0.688 |
| ns | 528 |  | 182 | `xlstm/__init__.py` in full — version plus the entire public export block | 1.5 |  | 0.620 |
| walker |  | 573 | 46 | Fs::DirListing { dir: experiments } |  |  | 0.623 |
| walker |  | 589 | 16 | Fs::DirListing { dir: experiments/data } |  |  | 0.626 |
| walker |  | 615 | 26 | Fs::DirListing { dir: experiments/data/formal_language } |  |  | 0.632 |
| walker |  | 643 | 28 | Fs::DirListing { dir: experiments/data/formal_language/tasks } |  |  | 0.640 |
| walker |  | 653 | 10 | Fs::DirListing { dir: .github/workflows } |  |  | 0.643 |
| walker |  | 686 | 33 | Fs::DirListing { dir: notebooks/xlstm } |  |  | 0.648 |
| ns | 712 |  | 184 | Every README heading (H1/H2/H3), line-located | 1.6 | 1.1 | 0.568 |
| walker |  | 872 | 186 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.683 |
| walker |  | 872 | 0 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.683 |
| walker |  | 921 | 49 | Markdown::CommandBlock { file: README.md, row: 43 } |  |  | 0.683 |
| ns | 968 |  | 256 | README 75-85: the 7B code is `xlstm/xlstm_large`, standalone on `mlstm_kernels` | 1.7 | 1.6 | 0.627 |
| walker |  | 976 | 55 | Fs::DirListing { dir: tests } |  |  | 0.641 |
| ns | 1139 |  | 171 | Complete listings of `tests/`, `experiments/` and the `experiments/data/` tree | 1.8 |  | 0.661 |
| walker |  | 1163 | 187 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.661 |
| walker |  | 1219 | 56 | Fs::DirListing { dir: xlstm/blocks/slstm/src/cuda } |  |  | 0.662 |
| ns | 1278 |  | 139 | Complete listings of `xlstm/blocks/mlstm/`, `xlstm/blocks/slstm/`, `notebooks/`, `res/`, `.github/workflows/` | 1.9 |  | 0.668 |
| walker |  | 1379 | 160 | Code::CodeKey { rung: Names, file: xlstm/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.709 |
| walker |  | 1425 | 46 | Code::CodeKey { rung: Names, file: xlstm/xlstm_lm_model.py, decl: 0, sub: 0, line: 0 } |  |  | 0.709 |
| walker |  | 1483 | 58 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_lm_model.py, decl: 1, sub: 0, line: 14 } |  |  | 0.710 |
| ns | 1548 |  | 270 | README quickstart: instantiate `xLSTMLargeConfig` + `xLSTMLarge` and run a forward pass | 2.1 |  | 0.653 |
| walker |  | 1631 | 148 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_lm_model.py, decl: 2, sub: 0, line: 22 } |  |  | 0.654 |
| walker |  | 1662 | 31 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_lm_model.py, decl: 6, sub: 0, line: 56 } |  |  | 0.654 |
| ns | 1667 |  | 119 | Every top-level class in `xlstm/xlstm_large/model.py`, name + line | 2.2 |  | 0.632 |
| walker |  | 1693 | 31 | Code::CodeKey { rung: Names, file: xlstm/xlstm_block_stack.py, decl: 0, sub: 0, line: 0 } |  |  | 0.632 |
| walker |  | 1831 | 138 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_block_stack.py, decl: 5, sub: 0, line: 77 } |  |  | 0.633 |
| ns | 1842 |  | 175 | `xLSTMLargeConfig` part 1/4 — the four required fields and the norm/bias toggles | 2.3 | 2.2 | 0.602 |
| walker |  | 1859 | 28 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_block_stack.py, decl: 10, sub: 0, line: 126 } |  |  | 0.603 |
| walker |  | 2177 | 318 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_block_stack.py, decl: 1, sub: 0, line: 15 } |  |  | 0.607 |
| walker |  | 2198 | 21 | Code::CodeKey { rung: Doc, file: xlstm/xlstm_block_stack.py, decl: 3, sub: 0, line: 40 } |  |  | 0.607 |
| ns | 2215 |  | 373 | `xLSTMLargeConfig` part 2/4 — qk/v dim factors and kernel selection (`chunkwise_kernel`, `sequence_kernel`, `step_kernel`, `mode`) | 2.4 | 2.3 | 0.560 |
| walker |  | 2280 | 82 | Fs::DirListing { dir: xlstm/blocks/slstm/src/util } |  |  | 0.562 |
| walker |  | 2437 | 157 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.568 |
| ns | 2458 |  | 243 | `xLSTMLargeConfig` part 3/4 — chunking, state return, and kernel dtypes | 2.5 | 2.4 | 0.542 |
| walker |  | 2511 | 74 | Plaintext::DeclSurface { file: pytest.ini } |  |  | 0.542 |
| walker |  | 2601 | 90 | Plaintext::DeclSurface { file: setup.cfg } |  |  | 0.542 |
| walker |  | 2632 | 31 | Code::CodeKey { rung: Names, file: xlstm/utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.543 |
| ns | 2698 |  | 240 | `xLSTMLargeConfig` part 4/4 — feedforward sizing, soft caps, `weight_mode` | 2.6 | 2.5 | 0.522 |
| walker |  | 2755 | 123 | Code::CodeKey { rung: Decl, file: xlstm/utils.py, decl: 1, sub: 0, line: 11 } |  |  | 0.522 |
| ns | 2891 |  | 193 | `xlstm/xlstm_large/model.py` header: the hard `mlstm_kernels` dependency and the state type aliases | 2.7 |  | 0.502 |
| walker |  | 2903 | 148 | Code::CodeKey { rung: Decl, file: xlstm/utils.py, decl: 3, sub: 0, line: 32 } |  |  | 0.502 |
| walker |  | 2921 | 18 | Code::CodeKey { rung: Decl, file: xlstm/utils.py, decl: 8, sub: 0, line: 97 } |  |  | 0.502 |
| walker |  | 2986 | 65 | Code::CodeKey { rung: Names, file: experiments/main.py, decl: 0, sub: 0, line: 0 } |  |  | 0.502 |
| walker |  | 3003 | 17 | Code::CodeKey { rung: Decl, file: experiments/main.py, decl: 1, sub: 0, line: 21 } |  |  | 0.502 |
| ns | 3038 |  | 147 | `xLSTMLarge.__init__` — embedding, backbone, lm_head | 2.8 | 2.2 | 0.489 |
| walker |  | 3047 | 44 | Code::CodeKey { rung: Decl, file: experiments/main.py, decl: 2, sub: 0, line: 25 } |  |  | 0.489 |
| walker |  | 3249 | 202 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.489 |
| ns | 3269 |  | 231 | `xLSTMLarge.forward` — signature, shape assert, `soft_cap`, and the conditional state return | 2.9 | 2.8 | 0.469 |
| walker |  | 3416 | 167 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.469 |
| ns | 3417 |  | 148 | `xLSTMLarge.generate` — signature and delegation to `generate_tokens` | 2.10 | 2.8 | 0.456 |
| walker |  | 3586 | 170 | Code::CodeKey { rung: Names, file: xlstm/xlstm_large/model.py, decl: 0, sub: 0, line: 0 } |  |  | 0.474 |
| walker |  | 3626 | 40 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 12, sub: 0, line: 232 } |  |  | 0.474 |
| ns | 3633 |  | 216 | `xLSTMLargeBlockStack.__init__` — the `mLSTMBlock` list and the `add_out_norm` switch | 2.11 | 2.2 | 0.456 |
| walker |  | 3676 | 50 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 19, sub: 0, line: 455 } |  |  | 0.457 |
| walker |  | 3698 | 22 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 21, sub: 0, line: 499 } |  |  | 0.457 |
| walker |  | 3752 | 54 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 16, sub: 0, line: 310 } |  |  | 0.457 |
| walker |  | 3776 | 24 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 18, sub: 0, line: 379 } |  |  | 0.457 |
| walker |  | 3846 | 70 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 9, sub: 0, line: 187 } |  |  | 0.458 |
| walker |  | 3868 | 22 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 11, sub: 0, line: 209 } |  |  | 0.458 |
| ns | 3894 |  | 261 | `xLSTMLargeBlockStack.forward` — the in-place per-layer state update | 2.12 | 2.11 | 0.442 |
| walker |  | 3971 | 103 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 5, sub: 0, line: 112 } |  |  | 0.445 |
| walker |  | 3993 | 22 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 7, sub: 0, line: 126 } |  |  | 0.446 |
| walker |  | 4051 | 58 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 8, sub: 0, line: 155 } |  |  | 0.453 |
| ns | 4260 |  | 366 | `mLSTMBlock.__init__` (xlstm_large) — the flat-config to nested-config mapping | 2.13 | 2.2 | 0.429 |
| ns | 4421 |  | 161 | `mLSTMBlock.forward` (xlstm_large) — pre-norm + residual around layer and FFN | 2.14 | 2.13 | 0.423 |
| walker |  | 4431 | 380 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 15, sub: 0, line: 280 } |  |  | 0.426 |
| walker |  | 4610 | 179 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 4, sub: 0, line: 30 } |  |  | 0.444 |
| walker |  | 4821 | 211 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 4, sub: 1, line: 30 } |  |  | 0.462 |
| ns | 4874 |  | 453 | Roster: every top-level class/function of the NeurIPS models and blocks, name + line | 3.1 |  | 0.446 |
| walker |  | 5057 | 236 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 4, sub: 2, line: 30 } |  |  | 0.485 |
| ns | 5187 |  | 313 | Roster: every top-level definition in `xlstm/components/` and the sLSTM kernel's Python layer | 3.2 |  | 0.470 |
| walker |  | 5234 | 177 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 4, sub: 3, line: 30 } |  |  | 0.486 |
| walker |  | 5416 | 182 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 4, sub: 4, line: 30 } |  |  | 0.506 |
| ns | 5447 |  | 260 | `xLSTMBlockStackConfig` — the complete field set | 3.3 | 3.1 | 0.517 |
| walker |  | 5568 | 152 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/model.py, decl: 4, sub: 5, line: 30 } |  |  | 0.534 |
| walker |  | 5608 | 40 | Code::CodeKey { rung: Names, file: experiments/lr_scheduler.py, decl: 0, sub: 0, line: 0 } |  |  | 0.534 |
| ns | 5646 |  | 199 | `block_map` property and `_create_block_map` — how `slstm_at` becomes a per-position block type | 3.4 | 3.3 | 0.525 |
| walker |  | 5679 | 71 | Code::CodeKey { rung: Decl, file: experiments/lr_scheduler.py, decl: 1, sub: 0, line: 9 } |  |  | 0.525 |
| walker |  | 5762 | 83 | Code::CodeKey { rung: Decl, file: experiments/lr_scheduler.py, decl: 5, sub: 0, line: 24 } |  |  | 0.525 |
| walker |  | 5776 | 14 | Code::CodeKey { rung: Doc, file: experiments/lr_scheduler.py, decl: 4, sub: 0, line: 18 } |  |  | 0.525 |
| walker |  | 5784 | 8 | Code::CodeKey { rung: Body, file: experiments/lr_scheduler.py, decl: 4, sub: 0, line: 18 } |  |  | 0.525 |
| walker |  | 5798 | 14 | Code::CodeKey { rung: Names, file: experiments/metrics.py, decl: 0, sub: 0, line: 0 } |  |  | 0.525 |
| ns | 5900 |  | 254 | `xLSTMBlockStackConfig.__post_init__` — the config mutation cascade | 3.5 | 3.3 | 0.514 |
| walker |  | 5901 | 103 | Code::CodeKey { rung: Decl, file: experiments/metrics.py, decl: 1, sub: 0, line: 9 } |  |  | 0.514 |
| walker |  | 5910 | 9 | Code::CodeKey { rung: Body, file: experiments/metrics.py, decl: 4, sub: 0, line: 22 } |  |  | 0.514 |
| walker |  | 5926 | 16 | Code::CodeKey { rung: Doc, file: experiments/lr_scheduler.py, decl: 3, sub: 0, line: 13 } |  |  | 0.514 |
| walker |  | 5934 | 8 | Code::CodeKey { rung: Body, file: experiments/lr_scheduler.py, decl: 3, sub: 0, line: 13 } |  |  | 0.514 |
| walker |  | 5950 | 16 | Code::CodeKey { rung: Doc, file: experiments/lr_scheduler.py, decl: 8, sub: 0, line: 47 } |  |  | 0.514 |
| walker |  | 5960 | 10 | Code::CodeKey { rung: Body, file: xlstm/utils.py, decl: 4, sub: 0, line: 33 } |  |  | 0.514 |
| walker |  | 6049 | 89 | Code::CodeKey { rung: Names, file: xlstm/components/feedforward.py, decl: 0, sub: 0, line: 0 } |  |  | 0.514 |
| walker |  | 6118 | 69 | Code::CodeKey { rung: Decl, file: xlstm/components/feedforward.py, decl: 5, sub: 0, line: 49 } |  |  | 0.514 |
| walker |  | 6209 | 91 | Code::CodeKey { rung: Decl, file: xlstm/components/feedforward.py, decl: 1, sub: 0, line: 12 } |  |  | 0.515 |
| ns | 6248 |  | 348 | `xLSTMBlockStack.__init__` and `_create_blocks` — block instantiation and `post_blocks_norm` | 3.6 | 3.1 | 0.498 |
| walker |  | 6342 | 133 | Code::CodeKey { rung: Decl, file: xlstm/components/feedforward.py, decl: 3, sub: 0, line: 31 } |  |  | 0.500 |
| walker |  | 6354 | 12 | Code::CodeKey { rung: Body, file: experiments/lr_scheduler.py, decl: 2, sub: 0, line: 10 } |  |  | 0.500 |
| walker |  | 6408 | 54 | Code::CodeKey { rung: Names, file: experiments/data/utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.500 |
| walker |  | 6483 | 75 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 10, sub: 0, line: 55 } |  |  | 0.500 |
| ns | 6502 |  | 254 | `xLSTMBlockStack.forward` and `step` — the loop and the `block_{i}` state dict | 3.7 | 3.6 | 0.490 |
| walker |  | 6566 | 83 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 6, sub: 0, line: 39 } |  |  | 0.490 |
| walker |  | 6686 | 120 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 1, sub: 0, line: 17 } |  |  | 0.490 |
| walker |  | 6693 | 7 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 2, sub: 0, line: 19 } |  |  | 0.490 |
| walker |  | 6700 | 7 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 3, sub: 0, line: 24 } |  |  | 0.490 |
| walker |  | 6707 | 7 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 4, sub: 0, line: 29 } |  |  | 0.490 |
| walker |  | 6714 | 7 | Code::CodeKey { rung: Decl, file: experiments/data/utils.py, decl: 5, sub: 0, line: 34 } |  |  | 0.490 |
| walker |  | 6719 | 5 | Code::CodeKey { rung: Body, file: experiments/data/utils.py, decl: 2, sub: 0, line: 19 } |  |  | 0.490 |
| walker |  | 6724 | 5 | Code::CodeKey { rung: Body, file: experiments/data/utils.py, decl: 3, sub: 0, line: 24 } |  |  | 0.490 |
| walker |  | 6729 | 5 | Code::CodeKey { rung: Body, file: experiments/data/utils.py, decl: 4, sub: 0, line: 29 } |  |  | 0.490 |
| walker |  | 6734 | 5 | Code::CodeKey { rung: Body, file: experiments/data/utils.py, decl: 5, sub: 0, line: 34 } |  |  | 0.490 |
| walker |  | 6763 | 29 | Code::CodeKey { rung: Names, file: xlstm/blocks/xlstm_block.py, decl: 0, sub: 0, line: 0 } |  |  | 0.491 |
| ns | 6834 |  | 332 | `xLSTMLMModelConfig` fields, `xLSTMLMModel.__init__`, and the model's complete method roster | 3.8 | 3.1 | 0.481 |
| walker |  | 6880 | 117 | Code::CodeKey { rung: Decl, file: xlstm/blocks/xlstm_block.py, decl: 3, sub: 0, line: 43 } |  |  | 0.481 |
| walker |  | 7017 | 137 | Code::CodeKey { rung: Decl, file: xlstm/blocks/xlstm_block.py, decl: 1, sub: 0, line: 16 } |  |  | 0.481 |
| walker |  | 7032 | 15 | Code::CodeKey { rung: Body, file: xlstm/xlstm_block_stack.py, decl: 2, sub: 0, line: 36 } |  |  | 0.482 |
| walker |  | 7054 | 22 | Code::CodeKey { rung: Body, file: experiments/metrics.py, decl: 5, sub: 0, line: 25 } |  |  | 0.482 |
| ns | 7060 |  | 226 | `xLSTMBlockConfig` — the mLSTM-xor-sLSTM invariant | 3.9 | 3.1 | 0.481 |
| walker |  | 7077 | 23 | Code::CodeKey { rung: Body, file: experiments/metrics.py, decl: 2, sub: 0, line: 13 } |  |  | 0.481 |
| walker |  | 7178 | 101 | Code::CodeKey { rung: Names, file: xlstm/xlstm_large/components.py, decl: 0, sub: 0, line: 0 } |  |  | 0.481 |
| walker |  | 7221 | 43 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 12, sub: 0, line: 155 } |  |  | 0.481 |
| walker |  | 7265 | 44 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 6, sub: 0, line: 70 } |  |  | 0.481 |
| walker |  | 7316 | 51 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 9, sub: 0, line: 99 } |  |  | 0.481 |
| walker |  | 7341 | 25 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 11, sub: 0, line: 142 } |  |  | 0.481 |
| walker |  | 7392 | 51 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 15, sub: 0, line: 188 } |  |  | 0.481 |
| walker |  | 7417 | 25 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 17, sub: 0, line: 231 } |  |  | 0.481 |
| ns | 7423 |  | 363 | `xLSTMBlock` — class docstring, `__init__` dispatch, and its complete method roster | 3.10 | 3.1 | 0.467 |
| walker |  | 7488 | 71 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 2, sub: 0, line: 24 } |  |  | 0.467 |
| walker |  | 7558 | 70 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 3, sub: 0, line: 35 } |  |  | 0.467 |
| walker |  | 7638 | 80 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 10, sub: 0, line: 123 } |  |  | 0.467 |
| ns | 7674 |  | 251 | Roster: every definition in `xlstm/xlstm_large/`'s support modules | 4.1 |  | 0.461 |
| walker |  | 7718 | 80 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/components.py, decl: 16, sub: 0, line: 212 } |  |  | 0.461 |
| ns | 7854 |  | 180 | `mLSTMLayerConfig` (NeurIPS) — `conv1d_kernel_size`, `qkv_proj_blocksize`, `num_heads`, `proj_factor` | 4.2 |  | 0.454 |
| ns | 8169 |  | 315 | `sLSTMLayerConfig` fields, and the head of `sLSTMCellConfig` (`backend`, `bias_init`, `num_states`) | 4.3 | 3.1 | 0.446 |
| walker |  | 8172 | 454 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.563 |
| walker |  | 8208 | 36 | Code::CodeKey { rung: Names, file: xlstm/blocks/mlstm/layer.py, decl: 0, sub: 0, line: 0 } |  |  | 0.565 |
| walker |  | 8318 | 110 | Code::CodeKey { rung: Decl, file: xlstm/blocks/mlstm/layer.py, decl: 3, sub: 0, line: 39 } |  |  | 0.565 |
| walker |  | 8372 | 54 | Code::CodeKey { rung: Decl, file: xlstm/blocks/mlstm/layer.py, decl: 6, sub: 0, line: 127 } |  |  | 0.565 |
| ns | 8400 |  | 231 | sLSTM backend dispatch: `sLSTMCell.__new__`, and the runtime CUDA build in `sLSTMCellCUDA.instance` | 4.4 | 3.1 | 0.557 |
| walker |  | 8560 | 188 | Code::CodeKey { rung: Decl, file: xlstm/blocks/mlstm/layer.py, decl: 1, sub: 0, line: 18 } |  |  | 0.574 |
| walker |  | 8596 | 36 | Code::CodeKey { rung: Names, file: xlstm/blocks/slstm/layer.py, decl: 0, sub: 0, line: 0 } |  |  | 0.576 |
| ns | 8628 |  | 228 | `_act_fn_registry` and `FeedForwardConfig` — the allowed `act_fn` values | 4.5 |  | 0.586 |
| walker |  | 8688 | 92 | Code::CodeKey { rung: Decl, file: xlstm/blocks/slstm/layer.py, decl: 3, sub: 0, line: 33 } |  |  | 0.586 |
| walker |  | 8736 | 48 | Code::CodeKey { rung: Decl, file: xlstm/blocks/slstm/layer.py, decl: 6, sub: 0, line: 92 } |  |  | 0.586 |
| walker |  | 8802 | 66 | Code::CodeKey { rung: Decl, file: xlstm/blocks/slstm/layer.py, decl: 7, sub: 0, line: 123 } |  |  | 0.586 |
| ns | 8804 |  | 176 | Complete listings of the sLSTM kernel tree: `src/`, `src/cuda/`, `src/util/`, `src/vanilla/` | 4.6 |  | 0.601 |
| walker |  | 8935 | 133 | Code::CodeKey { rung: Decl, file: xlstm/blocks/slstm/layer.py, decl: 1, sub: 0, line: 18 } |  |  | 0.604 |
| ns | 8968 |  | 164 | README: the three commands that run the parity experiments | 5.1 | 1.6 | 0.600 |
| walker |  | 8991 | 56 | Code::CodeKey { rung: Names, file: xlstm/xlstm_large/utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.602 |
| walker |  | 9005 | 14 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/utils.py, decl: 2, sub: 0, line: 10 } |  |  | 0.602 |
| walker |  | 9022 | 17 | Code::CodeKey { rung: Doc, file: xlstm/xlstm_large/utils.py, decl: 1, sub: 0, line: 5 } |  |  | 0.602 |
| ns | 9139 |  | 171 | `experiments/main.py` — the dataset registry and the `__main__` config entry point | 5.2 |  | 0.596 |
| walker |  | 9148 | 126 | Code::CodeKey { rung: Names, file: xlstm/xlstm_large/generate.py, decl: 0, sub: 0, line: 0 } |  |  | 0.603 |
| walker |  | 9163 | 15 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/generate.py, decl: 4, sub: 0, line: 18 } |  |  | 0.606 |
| walker |  | 9192 | 29 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/generate.py, decl: 2, sub: 0, line: 8 } |  |  | 0.610 |
| walker |  | 9328 | 136 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/generate.py, decl: 6, sub: 0, line: 27 } |  |  | 0.610 |
| walker |  | 9356 | 28 | Code::CodeKey { rung: Names, file: xlstm/xlstm_large/from_pretrained.py, decl: 0, sub: 0, line: 0 } |  |  | 0.612 |
| ns | 9440 |  | 301 | `experiments/parity_xlstm01.yaml` — the training and model sections of a real config | 5.3 |  | 0.600 |
| walker |  | 9457 | 101 | Code::CodeKey { rung: Decl, file: xlstm/xlstm_large/from_pretrained.py, decl: 1, sub: 0, line: 9 } |  |  | 0.600 |
| walker |  | 9485 | 28 | Code::CodeKey { rung: Body, file: experiments/main.py, decl: 3, sub: 0, line: 32 } |  |  | 0.600 |
| walker |  | 9507 | 22 | Code::CodeKey { rung: Doc, file: xlstm/xlstm_large/generate.py, decl: 3, sub: 0, line: 13 } |  |  | 0.600 |
| walker |  | 9516 | 9 | Code::CodeKey { rung: Body, file: experiments/data/utils.py, decl: 9, sub: 0, line: 50 } |  |  | 0.600 |
| walker |  | 9558 | 42 | Code::CodeKey { rung: Names, file: xlstm/xlstm_large/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.600 |
| walker |  | 9568 | 10 | Code::CodeKey { rung: Body, file: experiments/data/utils.py, decl: 8, sub: 0, line: 46 } |  |  | 0.600 |
| walker |  | 9597 | 29 | Code::CodeKey { rung: Names, file: xlstm/blocks/mlstm/cell.py, decl: 0, sub: 0, line: 0 } |  |  | 0.602 |
| walker |  | 9642 | 45 | Code::CodeKey { rung: Decl, file: xlstm/blocks/mlstm/cell.py, decl: 1, sub: 0, line: 13 } |  |  | 0.602 |
| ns | 9653 |  | 213 | `tests/conftest.py` in full, plus every test function in the suite | 5.4 |  | 0.595 |
| walker |  | 9760 | 118 | Code::CodeKey { rung: Decl, file: xlstm/blocks/mlstm/cell.py, decl: 2, sub: 0, line: 20 } |  |  | 0.595 |
| walker |  | 9826 | 66 | Code::CodeKey { rung: Decl, file: xlstm/blocks/mlstm/cell.py, decl: 5, sub: 0, line: 75 } |  |  | 0.595 |
| ns | 9837 |  | 184 | `pyproject.toml` — project metadata, the dependency list marker, and package data | 6.1 |  | 0.590 |
| walker |  | 9848 | 22 | Code::CodeKey { rung: Body, file: xlstm/xlstm_large/utils.py, decl: 1, sub: 0, line: 5 } |  |  | 0.590 |
| walker |  | 9902 | 54 | Code::CodeKey { rung: Doc, file: xlstm/utils.py, decl: 5, sub: 0, line: 36 } |  |  | 0.590 |
| walker |  | 9958 | 56 | Code::CodeKey { rung: Doc, file: xlstm/utils.py, decl: 6, sub: 0, line: 61 } |  |  | 0.590 |
| walker |  | 9971 | 13 | Code::CodeKey { rung: Body, file: xlstm/xlstm_large/generate.py, decl: 5, sub: 0, line: 23 } |  |  | 0.590 |
| walker |  | 9997 | 26 | Code::CodeKey { rung: Names, file: xlstm/blocks/mlstm/backends.py, decl: 0, sub: 0, line: 0 } |  |  | 0.591 |
| ns | 10005 |  | 168 | `pytest.ini` in full, and the README install commands | 6.2 | 1.6 | 0.598 |
