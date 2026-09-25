Score(3000)=0.459 I=0.801 C=0.263 ns_rows≤3K=18/55 grid(1000/1442/2080/3000/4327/6240/9000)=0.466/0.745/0.600/0.459/0.380/0.333/0.480

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 91 | 91 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 100 |  | 100 | README title, tagline and feature bullets | 1.1 |  | 0.000 |
| walker |  | 104 | 13 | Fs::DirListing { dir: packages } |  |  | 0.000 |
| ns | 113 |  | 13 | The three workspace packages | 1.2 |  | 0.292 |
| ns | 130 |  | 17 | packages/vite/src top-level split | 1.3 |  | 0.233 |
| walker |  | 146 | 42 | Json::Identity { file: package.json } |  |  | 0.236 |
| walker |  | 179 | 33 | Fs::DirListing { dir: patches } |  |  | 0.236 |
| ns | 205 |  | 75 | README: Vite's one-sentence definition | 1.4 | 1.1 | 0.227 |
| walker |  | 209 | 30 | Fs::DirListing { dir: packages/plugin-legacy } |  |  | 0.228 |
| walker |  | 232 | 23 | Fs::DirListing { dir: packages/plugin-legacy/src } |  |  | 0.228 |
| ns | 296 |  | 91 | Repository root listing (complete) | 1.5 |  | 0.400 |
| ns | 466 |  | 170 | README: dev server vs. build command, and extensibility | 1.6 | 1.4 | 0.372 |
| walker |  | 476 | 244 | Code::CodeKey { rung: Names, file: packages/plugin-legacy/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.372 |
| walker |  | 513 | 37 | Code::CodeKey { rung: Decl, file: packages/plugin-legacy/src/index.ts, decl: 7, sub: 0, line: 103 } |  |  | 0.372 |
| ns | 543 |  | 77 | packages/vite top-level listing | 1.7 |  | 0.309 |
| walker |  | 590 | 77 | Fs::DirListing { dir: packages/vite } |  |  | 0.447 |
| walker |  | 599 | 9 | Fs::DirListing { dir: packages/vite/bin } |  |  | 0.447 |
| ns | 636 |  | 93 | Root package.json identity, engines, package manager | 1.8 |  | 0.426 |
| walker |  | 672 | 73 | Json::Identity { file: packages/vite/package.json } |  |  | 0.432 |
| walker |  | 689 | 17 | Fs::DirListing { dir: packages/vite/src } |  |  | 0.507 |
| walker |  | 760 | 71 | Fs::DirListing { dir: packages/vite/src/module-runner } |  |  | 0.509 |
| walker |  | 778 | 18 | Fs::DirListing { dir: packages/vite/misc } |  |  | 0.509 |
| walker |  | 792 | 14 | Code::CodeKey { rung: Names, file: packages/vite/rollupLicensePlugin.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.509 |
| walker |  | 809 | 17 | Fs::DirListing { dir: packages/vite/src/client } |  |  | 0.510 |
| ns | 859 |  | 223 | Root pnpm scripts: lint, typecheck and the test entry points | 1.9 |  | 0.459 |
| walker |  | 944 | 135 | Fs::DirListing { dir: packages/vite/src/node } |  |  | 0.466 |
| walker |  | 1093 | 149 | Fs::DirListing { dir: packages/vite/src/node/plugins } |  |  | 0.473 |
| ns | 1118 |  | 259 | Root pnpm scripts: debug, docs, build, release | 1.10 | 1.9 | 0.435 |
| walker |  | 1133 | 40 | Json::Runtime { file: packages/vite/package.json } |  |  | 0.435 |
| walker |  | 1145 | 12 | Fs::DirListing { dir: packages/vite/src/module-runner/sourcemap } |  |  | 0.435 |
| ns | 1225 |  | 107 | vite package manifest: name, version, description, bin | 1.11 |  | 0.429 |
| walker |  | 1330 | 185 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.745 |
| walker |  | 1366 | 36 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.745 |
| walker |  | 1378 | 12 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.745 |
| walker |  | 1397 | 19 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.745 |
| walker |  | 1415 | 18 | Code::CodeKey { rung: Names, file: packages/vite/misc/false.d.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.745 |
| walker |  | 1433 | 18 | Code::CodeKey { rung: Names, file: packages/vite/misc/true.d.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.745 |
| walker |  | 1477 | 44 | Fs::DirListing { dir: packages/vite/types } |  |  | 0.746 |
| ns | 1485 |  | 260 | vite package exports and internal import aliases | 1.12 | 1.11 | 0.660 |
| walker |  | 1536 | 59 | Json::Runtime { file: package.json } |  |  | 0.684 |
| walker |  | 1566 | 30 | Fs::DirListing { dir: scripts } |  |  | 0.684 |
| walker |  | 1579 | 13 | Code::CodeKey { rung: Names, file: packages/vite/src/module-runner/runner.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.684 |
| ns | 1640 |  | 155 | src/node/index.ts: primary API factories | 2.1 |  | 0.642 |
| walker |  | 1713 | 134 | Json::Dependencies { file: packages/vite/package.json } |  |  | 0.642 |
| walker |  | 1727 | 14 | Code::CodeKey { rung: Names, file: packages/vite/src/node/cli.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.642 |
| walker |  | 1817 | 90 | Code::CodeKey { rung: Decl, file: packages/plugin-legacy/src/index.ts, decl: 4, sub: 0, line: 43 } |  |  | 0.642 |
| ns | 1853 |  | 213 | vite CLI: the complete command roster | 2.2 |  | 0.600 |
| ns | 2118 |  | 265 | src/node/index.ts: transform helpers and environment factories | 2.3 | 2.1 | 0.555 |
| walker |  | 2160 | 343 | Code::CodeKey { rung: Names, file: packages/plugin-legacy/src/index.ts, decl: 0, sub: 1, line: 0 } |  |  | 0.555 |
| walker |  | 2194 | 34 | Code::CodeKey { rung: Decl, file: packages/plugin-legacy/src/index.ts, decl: 13, sub: 0, line: 128 } |  |  | 0.555 |
| walker |  | 2208 | 14 | Code::CodeKey { rung: Decl, file: packages/plugin-legacy/src/index.ts, decl: 16, sub: 0, line: 146 } |  |  | 0.555 |
| ns | 2242 |  | 124 | src/node/index.ts: SSR/module-runner exports | 2.4 | 2.3 | 0.541 |
| walker |  | 2257 | 49 | Code::CodeKey { rung: Decl, file: packages/plugin-legacy/src/index.ts, decl: 18, sub: 0, line: 794 } |  |  | 0.541 |
| walker |  | 2323 | 66 | Code::CodeKey { rung: Decl, file: packages/plugin-legacy/src/index.ts, decl: 29, sub: 0, line: 1057 } |  |  | 0.541 |
| walker |  | 2344 | 21 | Code::CodeKey { rung: Decl, file: packages/plugin-legacy/src/index.ts, decl: 25, sub: 0, line: 999 } |  |  | 0.541 |
| walker |  | 2372 | 28 | Code::CodeKey { rung: Decl, file: packages/plugin-legacy/src/index.ts, decl: 21, sub: 0, line: 940 } |  |  | 0.541 |
| walker |  | 2406 | 34 | Code::CodeKey { rung: Decl, file: packages/plugin-legacy/src/index.ts, decl: 15, sub: 0, line: 143 } |  |  | 0.541 |
| walker |  | 2472 | 66 | Code::CodeKey { rung: Decl, file: packages/plugin-legacy/src/index.ts, decl: 14, sub: 0, line: 132 } |  |  | 0.541 |
| walker |  | 2487 | 15 | Code::CodeKey { rung: Names, file: packages/vite/src/node/assetSource.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.541 |
| walker |  | 2503 | 16 | Fs::DirListing { dir: packages/plugin-legacy/src/__tests__ } |  |  | 0.541 |
| walker |  | 2540 | 37 | Fs::DirListing { dir: packages/vite/types/internal } |  |  | 0.541 |
| ns | 2571 |  | 329 | src/node/index.ts: constants, utils and remaining value exports | 2.5 | 2.4 | 0.494 |
| walker |  | 2663 | 123 | Fs::DirListing { dir: packages/create-vite } |  |  | 0.495 |
| walker |  | 2667 | 4 | Fs::DirListing { dir: packages/create-vite/src } |  |  | 0.495 |
| ns | 2859 |  | 288 | index.ts type re-exports: config, server, build and plugin-option types | 2.6 | 2.5 | 0.459 |
| walker |  | 2918 | 251 | Code::CodeKey { rung: Names, file: packages/create-vite/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.459 |
| walker |  | 2934 | 16 | Code::CodeKey { rung: Decl, file: packages/create-vite/src/index.ts, decl: 10, sub: 0, line: 392 } |  |  | 0.459 |
| walker |  | 2958 | 24 | Code::CodeKey { rung: Decl, file: packages/create-vite/src/index.ts, decl: 9, sub: 0, line: 387 } |  |  | 0.459 |
| walker |  | 2999 | 41 | Code::CodeKey { rung: Decl, file: packages/create-vite/src/index.ts, decl: 6, sub: 0, line: 64 } |  |  | 0.459 |
| walker |  | 3052 | 53 | Code::CodeKey { rung: Decl, file: packages/create-vite/src/index.ts, decl: 7, sub: 0, line: 70 } |  |  | 0.459 |
| walker |  | 3098 | 46 | Markdown::ReadmeHeadline { file: packages/create-vite/README.md } |  |  | 0.459 |
| ns | 3307 |  | 448 | index.ts type re-exports: server internals, HMR payloads, vendored types | 2.7 | 2.6 | 0.427 |
| walker |  | 3410 | 312 | Code::CodeKey { rung: Names, file: packages/create-vite/src/index.ts, decl: 0, sub: 1, line: 0 } |  |  | 0.427 |
| walker |  | 3431 | 21 | Code::CodeKey { rung: Decl, file: packages/create-vite/src/index.ts, decl: 23, sub: 0, line: 759 } |  |  | 0.427 |
| walker |  | 3519 | 88 | Code::CodeKey { rung: Decl, file: packages/create-vite/src/index.ts, decl: 1, sub: 0, line: 11 } |  |  | 0.427 |
| walker |  | 3543 | 24 | Markdown::HeadingsOutline { file: packages/create-vite/README.md } |  |  | 0.427 |
| walker |  | 3560 | 17 | Code::CodeKey { rung: Names, file: packages/vite/src/module-runner/esmEvaluator.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.427 |
| ns | 3569 |  | 262 | Plugin interface: every Vite-specific hook and flag (names only) | 2.8 |  | 0.410 |
| walker |  | 3577 | 17 | Code::CodeKey { rung: Names, file: packages/vite/src/module-runner/hmrHandler.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.410 |
| walker |  | 3594 | 17 | Code::CodeKey { rung: Names, file: packages/vite/types/internal/lightningcssOptions.d.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.410 |
| walker |  | 3635 | 41 | Fs::DirListing { dir: .github } |  |  | 0.410 |
| walker |  | 3714 | 79 | Fs::DirListing { dir: .github/workflows } |  |  | 0.411 |
| walker |  | 3778 | 64 | Fs::DirListing { dir: docs } |  |  | 0.412 |
| walker |  | 3793 | 15 | Fs::DirListing { dir: docs/_data } |  |  | 0.412 |
| walker |  | 3909 | 116 | Code::CodeKey { rung: Decl, file: packages/create-vite/src/index.ts, decl: 2, sub: 0, line: 25 } |  |  | 0.412 |
| walker |  | 3928 | 19 | Fs::DirListing { dir: docs/.vitepress } |  |  | 0.412 |
| ns | 3931 |  | 362 | ViteDevServer: every member (names only) | 2.9 |  | 0.388 |
| walker |  | 3976 | 48 | Code::CodeKey { rung: Decl, file: packages/vite/rollupLicensePlugin.ts, decl: 1, sub: 0, line: 7 } |  |  | 0.388 |
| walker |  | 4005 | 29 | Fs::DirListing { dir: packages/vite/src/node/optimizer } |  |  | 0.388 |
| ns | 4111 |  | 180 | UserConfig keys: project, sources and transform options | 3.1 |  | 0.379 |
| walker |  | 4137 | 132 | Code::CodeKey { rung: Decl, file: packages/plugin-legacy/src/index.ts, decl: 19, sub: 0, line: 840 } |  |  | 0.379 |
| walker |  | 4158 | 21 | Fs::DirListing { dir: docs/.vitepress/theme } |  |  | 0.379 |
| walker |  | 4191 | 33 | Code::CodeKey { rung: Names, file: packages/vite/types/hot.d.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.379 |
| walker |  | 4208 | 17 | Code::CodeKey { rung: Decl, file: packages/vite/types/hot.d.ts, decl: 1, sub: 0, line: 3 } |  |  | 0.379 |
| walker |  | 4230 | 22 | Fs::DirListing { dir: packages/vite/src/node/__tests_dts__ } |  |  | 0.379 |
| walker |  | 4280 | 50 | Fs::DirListing { dir: packages/vite/src/types } |  |  | 0.380 |
| walker |  | 4299 | 19 | Json::Identity { file: packages/vite/src/types/package.json } |  |  | 0.380 |
| walker |  | 4311 | 12 | Code::CodeKey { rung: Names, file: packages/vite/src/types/connect.d.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.380 |
| walker |  | 4326 | 15 | Code::CodeKey { rung: Names, file: packages/vite/src/types/dynamicImportVars.d.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.380 |
| walker |  | 4342 | 16 | Code::CodeKey { rung: Names, file: packages/vite/src/types/commonjs.d.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.380 |
| walker |  | 4366 | 24 | Fs::DirListing { dir: packages/create-vite/template-preact } |  |  | 0.380 |
| walker |  | 4385 | 19 | Fs::DirListing { dir: packages/create-vite/template-preact/src } |  |  | 0.380 |
| ns | 4401 |  | 290 | UserConfig keys: server, build, env, worker and the rest | 3.2 | 3.1 | 0.366 |
| ns | 4571 |  | 170 | Per-environment options (SharedEnvironmentOptions / EnvironmentOptions) | 3.3 |  | 0.358 |
| walker |  | 4575 | 190 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.376 |
| walker |  | 4630 | 55 | Fs::DirListing { dir: packages/vite/src/shared } |  |  | 0.377 |
| walker |  | 4642 | 12 | Code::CodeKey { rung: Names, file: packages/vite/src/shared/builtin.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.377 |
| walker |  | 4657 | 15 | Code::CodeKey { rung: Names, file: packages/vite/src/shared/hmrHandler.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.377 |
| walker |  | 4682 | 25 | Code::CodeKey { rung: Names, file: packages/vite/src/node/environment.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.377 |
| walker |  | 4707 | 25 | Code::CodeKey { rung: Names, file: packages/vite/src/node/internalIndex.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.372 |
| ns | 4707 |  | 136 | CommonServerOptions: the host/port/https/proxy/cors keys | 3.4 |  | 0.372 |
| walker |  | 4724 | 17 | Code::CodeKey { rung: Names, file: packages/vite/src/node/plugins/esbuildBannerFooterCompatPlugin.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.372 |
| walker |  | 4741 | 17 | Code::CodeKey { rung: Names, file: packages/vite/src/node/plugins/forwardConsole.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.372 |
| walker |  | 4758 | 17 | Code::CodeKey { rung: Names, file: packages/vite/src/node/plugins/prepareOutDir.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.372 |
| walker |  | 4782 | 24 | Code::CodeKey { rung: Decl, file: packages/vite/src/node/assetSource.ts, decl: 1, sub: 0, line: 111 } |  |  | 0.372 |
| walker |  | 4799 | 17 | Code::CodeKey { rung: Decl, file: packages/vite/src/node/plugins/forwardConsole.ts, decl: 1, sub: 0, line: 11 } |  |  | 0.372 |
| walker |  | 4817 | 18 | Code::CodeKey { rung: Names, file: packages/vite/src/node/plugins/wasm.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.372 |
| walker |  | 4844 | 27 | Code::CodeKey { rung: Names, file: packages/vite/src/node/publicDir.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.372 |
| walker |  | 4869 | 25 | Code::CodeKey { rung: Decl, file: packages/vite/src/node/publicDir.ts, decl: 1, sub: 0, line: 13 } |  |  | 0.372 |
| walker |  | 4895 | 26 | Code::CodeKey { rung: Decl, file: packages/vite/src/module-runner/hmrHandler.ts, decl: 1, sub: 0, line: 7 } |  |  | 0.372 |
| walker |  | 4923 | 28 | Fs::DirListing { dir: packages/create-vite/template-qwik } |  |  | 0.372 |
| walker |  | 4942 | 19 | Fs::DirListing { dir: packages/create-vite/template-qwik/src } |  |  | 0.372 |
| ns | 4944 |  | 237 | ServerOptions and FileSystemServeOptions keys | 3.5 |  | 0.362 |
| walker |  | 4970 | 28 | Fs::DirListing { dir: packages/create-vite/template-solid } |  |  | 0.362 |
| walker |  | 4989 | 19 | Fs::DirListing { dir: packages/create-vite/template-solid/src } |  |  | 0.362 |
| walker |  | 5017 | 28 | Code::CodeKey { rung: Names, file: packages/vite/src/module-runner/createImportMeta.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.362 |
| walker |  | 5037 | 20 | Code::CodeKey { rung: Decl, file: packages/vite/src/module-runner/createImportMeta.ts, decl: 1, sub: 0, line: 14 } |  |  | 0.362 |
| walker |  | 5057 | 20 | Code::CodeKey { rung: Decl, file: packages/vite/src/module-runner/createImportMeta.ts, decl: 2, sub: 0, line: 41 } |  |  | 0.362 |
| walker |  | 5084 | 27 | Code::CodeKey { rung: Decl, file: packages/vite/src/node/cli.ts, decl: 1, sub: 0, line: 68 } |  |  | 0.362 |
| walker |  | 5113 | 29 | Fs::DirListing { dir: docs/.vitepress/theme/landing } |  |  | 0.362 |
| walker |  | 5141 | 28 | Code::CodeKey { rung: Decl, file: packages/vite/src/shared/builtin.ts, decl: 1, sub: 0, line: 1 } |  |  | 0.362 |
| walker |  | 5161 | 20 | Code::CodeKey { rung: Names, file: packages/vite/src/node/optimizer/pluginConverter.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.362 |
| ns | 5180 |  | 236 | BuildEnvironmentOptions keys: output, assets, CSS, minification | 3.6 |  | 0.355 |
| walker |  | 5190 | 29 | Code::CodeKey { rung: Decl, file: packages/vite/src/node/environment.ts, decl: 2, sub: 0, line: 20 } |  |  | 0.355 |
| walker |  | 5219 | 29 | Code::CodeKey { rung: Decl, file: packages/vite/src/node/publicDir.ts, decl: 2, sub: 0, line: 36 } |  |  | 0.355 |
| walker |  | 5240 | 21 | Code::CodeKey { rung: Names, file: packages/vite/src/node/plugins/reporter.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.355 |
| walker |  | 5271 | 31 | Code::CodeKey { rung: Names, file: packages/vite/src/node/nodeResolve.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.355 |
| walker |  | 5295 | 24 | Code::CodeKey { rung: Decl, file: packages/vite/src/node/nodeResolve.ts, decl: 1, sub: 0, line: 6 } |  |  | 0.355 |
| walker |  | 5315 | 20 | Code::CodeKey { rung: Decl, file: packages/vite/src/node/plugins/esbuildBannerFooterCompatPlugin.ts, decl: 1, sub: 0, line: 13 } |  |  | 0.355 |
| walker |  | 5347 | 32 | Fs::DirListing { dir: docs/.vitepress/theme/live } |  |  | 0.355 |
| ns | 5437 |  | 257 | BuildEnvironmentOptions keys: bundler passthrough, lib, ssr, reporting | 3.7 | 3.6 | 0.347 |
| walker |  | 5469 | 122 | Markdown::ReadmeHeadline { file: packages/plugin-legacy/README.md } |  |  | 0.347 |
| walker |  | 5519 | 50 | Code::CodeKey { rung: Names, file: packages/vite/types/metadata.d.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.347 |
| ns | 5530 |  | 93 | ExperimentalOptions and FutureOptions | 3.8 |  | 0.344 |
| walker |  | 5544 | 25 | Code::CodeKey { rung: Decl, file: packages/vite/types/metadata.d.ts, decl: 1, sub: 0, line: 1 } |  |  | 0.344 |
| walker |  | 5588 | 44 | Code::CodeKey { rung: Decl, file: packages/vite/types/metadata.d.ts, decl: 2, sub: 0, line: 6 } |  |  | 0.344 |
| walker |  | 5621 | 33 | Fs::DirListing { dir: packages/create-vite/template-react } |  |  | 0.344 |
| walker |  | 5640 | 19 | Fs::DirListing { dir: packages/create-vite/template-react/src } |  |  | 0.344 |
| walker |  | 5673 | 33 | Fs::DirListing { dir: packages/create-vite/template-vue } |  |  | 0.344 |
| walker |  | 5691 | 18 | Fs::DirListing { dir: packages/create-vite/template-vue/src } |  |  | 0.344 |
| ns | 5804 |  | 274 | CLI global options (all five commands) | 3.9 | 2.2 | 0.337 |
| walker |  | 5811 | 120 | Json::IdentityMeta { file: package.json } |  |  | 0.338 |
| walker |  | 5860 | 49 | Fs::DirListing { dir: packages/vite/src/node/ssr } |  |  | 0.338 |
| walker |  | 5871 | 11 | Fs::DirListing { dir: packages/vite/src/node/ssr/runtime } |  |  | 0.338 |
| walker |  | 5886 | 15 | Code::CodeKey { rung: Names, file: packages/vite/src/node/ssr/runnerImport.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.338 |
| walker |  | 5902 | 16 | Code::CodeKey { rung: Names, file: packages/vite/src/node/ssr/ssrModuleLoader.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.338 |
| walker |  | 5919 | 17 | Code::CodeKey { rung: Names, file: packages/vite/src/node/ssr/ssrManifestPlugin.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.338 |
| walker |  | 5951 | 32 | Fs::DirListing { dir: packages/vite/src/node/ssr/__tests__ } |  |  | 0.338 |
| walker |  | 5984 | 33 | Code::CodeKey { rung: Decl, file: packages/vite/src/shared/hmrHandler.ts, decl: 1, sub: 0, line: 4 } |  |  | 0.338 |
| ns | 5995 |  | 191 | CLI dev-server flags | 3.10 | 2.2 | 0.333 |
| walker |  | 6019 | 35 | Code::CodeKey { rung: Names, file: packages/vite/src/module-runner/hmrLogger.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.333 |
| walker |  | 6040 | 21 | Code::CodeKey { rung: Decl, file: packages/vite/src/module-runner/hmrLogger.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.333 |
| walker |  | 6064 | 24 | Code::CodeKey { rung: Names, file: packages/vite/src/module-runner/sourcemap/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.333 |
| walker |  | 6087 | 23 | Code::CodeKey { rung: Decl, file: packages/vite/src/node/optimizer/pluginConverter.ts, decl: 1, sub: 0, line: 31 } |  |  | 0.333 |
| walker |  | 6146 | 59 | Code::CodeKey { rung: Names, file: packages/vite/types/importMeta.d.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.333 |
| walker |  | 6162 | 16 | Code::CodeKey { rung: Decl, file: packages/vite/types/importMeta.d.ts, decl: 1, sub: 0, line: 7 } |  |  | 0.333 |
| walker |  | 6182 | 20 | Code::CodeKey { rung: Decl, file: packages/vite/types/importMeta.d.ts, decl: 2, sub: 0, line: 11 } |  |  | 0.333 |
| walker |  | 6231 | 49 | Code::CodeKey { rung: Decl, file: packages/vite/types/importMeta.d.ts, decl: 3, sub: 0, line: 14 } |  |  | 0.333 |
| walker |  | 6257 | 26 | Code::CodeKey { rung: Names, file: packages/vite/src/module-runner/sourcemap/decoder.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.333 |
| walker |  | 6283 | 26 | Code::CodeKey { rung: Names, file: packages/vite/src/module-runner/sourcemap/interceptor.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.333 |
| walker |  | 6309 | 26 | Code::CodeKey { rung: Names, file: packages/vite/src/node/ssr/fetchModule.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.333 |
| walker |  | 6348 | 39 | Fs::DirListing { dir: packages/vite/src/node/ssr/__tests__/fixtures } |  |  | 0.333 |
| walker |  | 6362 | 14 | Fs::DirListing { dir: packages/vite/src/node/ssr/__tests__/fixtures/named-overwrite-all } |  |  | 0.333 |
| walker |  | 6378 | 16 | Fs::DirListing { dir: packages/vite/src/node/ssr/__tests__/fixtures/modules } |  |  | 0.333 |
| walker |  | 6396 | 18 | Fs::DirListing { dir: packages/vite/src/node/ssr/__tests__/fixtures/multi-source-sourcemaps } |  |  | 0.333 |
| walker |  | 6420 | 24 | Fs::DirListing { dir: packages/vite/src/node/ssr/__tests__/fixtures/errors } |  |  | 0.333 |
| walker |  | 6459 | 39 | Code::CodeKey { rung: Names, file: packages/vite/src/node/env.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.333 |
| ns | 6467 |  | 472 | CLI build flags | 3.11 | 2.2 | 0.320 |
| walker |  | 6483 | 24 | Code::CodeKey { rung: Decl, file: packages/vite/src/node/env.ts, decl: 3, sub: 0, line: 98 } |  |  | 0.320 |
| walker |  | 6512 | 29 | Code::CodeKey { rung: Decl, file: packages/vite/src/node/env.ts, decl: 1, sub: 0, line: 12 } |  |  | 0.320 |
| ns | 6622 |  | 155 | CLI optimize and preview flags | 3.12 | 2.2 | 0.317 |
| ns | 6757 |  | 135 | src/node module roster (complete) | 4.1 |  | 0.368 |
| walker |  | 6773 | 261 | Code::CodeKey { rung: Names, file: packages/vite/client.d.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.368 |
| walker |  | 6795 | 22 | Code::CodeKey { rung: Decl, file: packages/vite/client.d.ts, decl: 19, sub: 0, line: 56 } |  |  | 0.368 |
| walker |  | 6819 | 24 | Code::CodeKey { rung: Decl, file: packages/vite/client.d.ts, decl: 3, sub: 0, line: 9 } |  |  | 0.368 |
| walker |  | 6843 | 24 | Code::CodeKey { rung: Decl, file: packages/vite/client.d.ts, decl: 4, sub: 0, line: 13 } |  |  | 0.368 |
| walker |  | 6867 | 24 | Code::CodeKey { rung: Decl, file: packages/vite/client.d.ts, decl: 5, sub: 0, line: 17 } |  |  | 0.368 |
| walker |  | 6891 | 24 | Code::CodeKey { rung: Decl, file: packages/vite/client.d.ts, decl: 6, sub: 0, line: 21 } |  |  | 0.368 |
| ns | 6906 |  | 149 | src/node/plugins roster (complete) | 4.2 |  | 0.406 |
| walker |  | 6915 | 24 | Code::CodeKey { rung: Decl, file: packages/vite/client.d.ts, decl: 7, sub: 0, line: 25 } |  |  | 0.406 |
| walker |  | 6939 | 24 | Code::CodeKey { rung: Decl, file: packages/vite/client.d.ts, decl: 8, sub: 0, line: 29 } |  |  | 0.406 |
| walker |  | 6963 | 24 | Code::CodeKey { rung: Decl, file: packages/vite/client.d.ts, decl: 9, sub: 0, line: 33 } |  |  | 0.406 |
| ns | 6981 |  | 75 | src/node/server roster (complete) | 4.3 |  | 0.400 |
| walker |  | 6987 | 24 | Code::CodeKey { rung: Decl, file: packages/vite/client.d.ts, decl: 20, sub: 0, line: 60 } |  |  | 0.400 |
| walker |  | 7013 | 26 | Code::CodeKey { rung: Decl, file: packages/vite/client.d.ts, decl: 10, sub: 0, line: 37 } |  |  | 0.400 |
| walker |  | 7040 | 27 | Code::CodeKey { rung: Names, file: packages/vite/src/node/optimizer/optimizer.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.400 |
| walker |  | 7059 | 19 | Code::CodeKey { rung: Decl, file: packages/vite/src/node/optimizer/optimizer.ts, decl: 1, sub: 0, line: 40 } |  |  | 0.400 |
| ns | 7066 |  | 85 | Dev-server middlewares and per-environment implementations | 4.4 |  | 0.393 |
| walker |  | 7078 | 19 | Code::CodeKey { rung: Decl, file: packages/vite/src/node/optimizer/optimizer.ts, decl: 2, sub: 0, line: 756 } |  |  | 0.393 |
| walker |  | 7119 | 41 | Fs::DirListing { dir: packages/create-vite/template-preact-ts } |  |  | 0.393 |
| walker |  | 7140 | 21 | Fs::DirListing { dir: packages/create-vite/template-preact-ts/src } |  |  | 0.393 |
| ns | 7144 |  | 78 | SSR and dependency-optimizer rosters | 4.5 |  | 0.410 |
| walker |  | 7181 | 41 | Code::CodeKey { rung: Names, file: packages/vite/src/node/baseEnvironment.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.410 |
| walker |  | 7197 | 16 | Code::CodeKey { rung: Decl, file: packages/vite/src/node/baseEnvironment.ts, decl: 7, sub: 0, line: 135 } |  |  | 0.410 |
| walker |  | 7238 | 41 | Code::CodeKey { rung: Names, file: packages/vite/src/node/idResolver.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.410 |
| walker |  | 7271 | 33 | Code::CodeKey { rung: Decl, file: packages/vite/src/node/idResolver.ts, decl: 2, sub: 0, line: 23 } |  |  | 0.410 |
| ns | 7287 |  | 143 | Browser client, module-runner and shared rosters | 4.6 |  | 0.437 |
| walker |  | 7304 | 33 | Code::CodeKey { rung: Decl, file: packages/vite/src/node/idResolver.ts, decl: 3, sub: 0, line: 42 } |  |  | 0.437 |
| walker |  | 7345 | 41 | Code::CodeKey { rung: Names, file: packages/vite/src/shared/ssrTransform.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.437 |
| walker |  | 7361 | 16 | Code::CodeKey { rung: Decl, file: packages/vite/src/shared/ssrTransform.ts, decl: 2, sub: 0, line: 14 } |  |  | 0.437 |
| ns | 7400 |  | 113 | resolvePlugins: the built-in plugin pipeline order | 4.7 |  | 0.433 |
| walker |  | 7403 | 42 | Fs::DirListing { dir: docs/config } |  |  | 0.433 |
| walker |  | 7443 | 40 | Code::CodeKey { rung: Decl, file: packages/vite/src/module-runner/hmrLogger.ts, decl: 2, sub: 0, line: 10 } |  |  | 0.433 |
| walker |  | 7483 | 40 | Code::CodeKey { rung: Decl, file: packages/vite/src/node/environment.ts, decl: 1, sub: 0, line: 7 } |  |  | 0.433 |
| ns | 7494 |  | 94 | Published and inlined type declarations | 4.8 |  | 0.447 |
| walker |  | 7549 | 66 | Code::CodeKey { rung: Names, file: packages/vite/types/importGlob.d.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.447 |
| walker |  | 7578 | 29 | Code::CodeKey { rung: Decl, file: packages/vite/types/importGlob.d.ts, decl: 3, sub: 0, line: 44 } |  |  | 0.447 |
| walker |  | 7608 | 30 | Code::CodeKey { rung: Decl, file: packages/vite/types/importGlob.d.ts, decl: 4, sub: 0, line: 49 } |  |  | 0.447 |
| ns | 7609 |  | 115 | Core internal entry-point signatures | 4.9 |  | 0.442 |
| walker |  | 7651 | 43 | Code::CodeKey { rung: Names, file: packages/vite/src/client/overlay.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.442 |
| walker |  | 7694 | 43 | Code::CodeKey { rung: Names, file: packages/vite/src/shared/hmr.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.442 |
| walker |  | 7724 | 30 | Code::CodeKey { rung: Decl, file: packages/vite/src/shared/hmr.ts, decl: 1, sub: 0, line: 19 } |  |  | 0.442 |
| walker |  | 7768 | 44 | Fs::DirListing { dir: docs/.vitepress/theme/components } |  |  | 0.442 |
| walker |  | 7812 | 44 | Fs::DirListing { dir: packages/create-vite/template-svelte } |  |  | 0.442 |
| walker |  | 7831 | 19 | Fs::DirListing { dir: packages/create-vite/template-svelte/src } |  |  | 0.442 |
| walker |  | 7836 | 5 | Fs::DirListing { dir: packages/create-vite/template-svelte/src/lib } |  |  | 0.442 |
| walker |  | 7880 | 44 | Code::CodeKey { rung: Names, file: packages/vite/src/module-runner/evaluatedModules.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.442 |
| walker |  | 7910 | 30 | Code::CodeKey { rung: Names, file: packages/vite/src/node/plugins/json.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.442 |
| walker |  | 7919 | 9 | Code::CodeKey { rung: Decl, file: packages/vite/src/node/plugins/json.ts, decl: 2, sub: 0, line: 18 } |  |  | 0.433 |
| ns | 7919 |  | 310 | CONTRIBUTING.md: every section heading | 5.1 |  | 0.433 |
| ns | 8109 |  | 190 | CONTRIBUTING: local development loop | 5.2 | 5.1 | 0.431 |
| walker |  | 8325 | 406 | Fs::DirListing { dir: playground } |  |  | 0.435 |
| walker |  | 8370 | 45 | Fs::DirListing { dir: docs/changes } |  |  | 0.435 |
| walker |  | 8415 | 45 | Fs::DirListing { dir: packages/create-vite/template-qwik-ts } |  |  | 0.435 |
| walker |  | 8436 | 21 | Fs::DirListing { dir: packages/create-vite/template-qwik-ts/src } |  |  | 0.435 |
| walker |  | 8481 | 45 | Fs::DirListing { dir: packages/create-vite/template-solid-ts } |  |  | 0.435 |
| walker |  | 8502 | 21 | Fs::DirListing { dir: packages/create-vite/template-solid-ts/src } |  |  | 0.435 |
| ns | 8515 |  | 406 | playground/ roster (complete e2e corpus) | 5.3 |  | 0.489 |
| walker |  | 8545 | 43 | Code::CodeKey { rung: Decl, file: packages/vite/src/node/nodeResolve.ts, decl: 2, sub: 0, line: 14 } |  |  | 0.489 |
| walker |  | 8555 | 10 | Code::CodeKey { rung: Names, file: packages/plugin-legacy/src/types.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.489 |
| walker |  | 8599 | 44 | Code::CodeKey { rung: Decl, file: packages/vite/src/client/overlay.ts, decl: 1, sub: 0, line: 10 } |  |  | 0.489 |
| walker |  | 8629 | 30 | Code::CodeKey { rung: Decl, file: packages/vite/src/module-runner/sourcemap/decoder.ts, decl: 3, sub: 0, line: 58 } |  |  | 0.489 |
| walker |  | 8676 | 47 | Code::CodeKey { rung: Names, file: packages/vite/src/node/external.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.489 |
| walker |  | 8703 | 27 | Code::CodeKey { rung: Decl, file: packages/vite/src/node/external.ts, decl: 2, sub: 0, line: 35 } |  |  | 0.489 |
| walker |  | 8739 | 36 | Code::CodeKey { rung: Decl, file: packages/vite/src/node/external.ts, decl: 1, sub: 0, line: 22 } |  |  | 0.489 |
| walker |  | 8771 | 32 | Code::CodeKey { rung: Names, file: packages/vite/src/node/optimizer/rolldownDepPlugin.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.489 |
| ns | 8802 |  | 287 | CONTRIBUTING: how the integration tests work | 5.4 | 5.3 | 0.488 |
| ns | 8919 |  | 117 | Unit test locations under packages/vite/src | 5.5 |  | 0.480 |
| walker |  | 8954 | 183 | Markdown::ReadmeHeadline { file: packages/vite/README.md } |  |  | 0.480 |
| walker |  | 8969 | 15 | Code::CodeKey { rung: Body, file: packages/plugin-legacy/src/index.ts, decl: 30, sub: 0, line: 1069 } |  |  | 0.480 |
| walker |  | 9000 | 31 | Code::CodeKey { rung: Decl, file: packages/vite/src/module-runner/sourcemap/interceptor.ts, decl: 1, sub: 0, line: 16 } |  |  | 0.480 |
| walker |  | 9031 | 31 | Code::CodeKey { rung: Decl, file: packages/vite/src/module-runner/sourcemap/interceptor.ts, decl: 2, sub: 0, line: 59 } |  |  | 0.480 |
| ns | 9050 |  | 131 | How the unit and e2e vitest runs are separated | 5.6 |  | 0.477 |
| walker |  | 9081 | 50 | Fs::DirListing { dir: packages/create-vite/template-react-ts } |  |  | 0.477 |
| walker |  | 9102 | 21 | Fs::DirListing { dir: packages/create-vite/template-react-ts/src } |  |  | 0.477 |
| ns | 9114 |  | 64 | docs/ site roster | 5.7 |  | 0.487 |
| walker |  | 9152 | 50 | Fs::DirListing { dir: packages/create-vite/template-vue-ts } |  |  | 0.487 |
| walker |  | 9170 | 18 | Fs::DirListing { dir: packages/create-vite/template-vue-ts/src } |  |  | 0.487 |
| walker |  | 9244 | 74 | Code::CodeKey { rung: Decl, file: packages/vite/types/importMeta.d.ts, decl: 4, sub: 0, line: 22 } |  |  | 0.487 |
| ns | 9298 |  | 184 | docs/guide and docs/config page rosters | 5.8 |  | 0.478 |
| ns | 9343 |  | 45 | docs/changes: the breaking-change / migration notes | 5.9 |  | 0.481 |
| ns | 9501 |  | 158 | CONTRIBUTING: the dependency policy | 5.10 | 5.1 | 0.479 |
| walker |  | 9563 | 319 | Code::CodeKey { rung: Decl, file: packages/create-vite/src/index.ts, decl: 4, sub: 0, line: 39 } |  |  | 0.479 |
| walker |  | 9597 | 34 | Code::CodeKey { rung: Names, file: packages/vite/src/node/plugins/preAlias.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.479 |
| ns | 9624 |  | 123 | create-vite: package layout and template roster | 5.11 |  | 0.491 |
| walker |  | 9627 | 30 | Code::CodeKey { rung: Decl, file: packages/vite/src/node/plugins/preAlias.ts, decl: 2, sub: 0, line: 129 } |  |  | 0.491 |
| ns | 9677 |  | 53 | plugin-legacy: package layout | 5.12 |  | 0.497 |
| walker |  | 9683 | 56 | Fs::DirListing { dir: packages/create-vite/template-svelte-ts } |  |  | 0.497 |
| walker |  | 9702 | 19 | Fs::DirListing { dir: packages/create-vite/template-svelte-ts/src } |  |  | 0.497 |
| walker |  | 9707 | 5 | Fs::DirListing { dir: packages/create-vite/template-svelte-ts/src/lib } |  |  | 0.497 |
| walker |  | 9756 | 49 | Code::CodeKey { rung: Decl, file: packages/vite/src/node/env.ts, decl: 2, sub: 0, line: 28 } |  |  | 0.497 |
| walker |  | 9805 | 49 | Code::CodeKey { rung: Decl, file: packages/vite/src/node/idResolver.ts, decl: 1, sub: 0, line: 11 } |  |  | 0.497 |
| ns | 9827 |  | 150 | Repository automation: .github and release scripts | 5.13 |  | 0.509 |
| walker |  | 9854 | 49 | Code::CodeKey { rung: Decl, file: packages/vite/src/shared/ssrTransform.ts, decl: 3, sub: 0, line: 23 } |  |  | 0.509 |
| walker |  | 9888 | 34 | Code::CodeKey { rung: Decl, file: packages/vite/src/node/ssr/runnerImport.ts, decl: 1, sub: 0, line: 15 } |  |  | 0.509 |
| walker |  | 9963 | 75 | Fs::DirListing { dir: packages/vite/src/node/server } |  |  | 0.521 |
| walker |  | 9981 | 18 | Fs::DirListing { dir: packages/vite/src/node/server/environments } |  |  | 0.521 |
| walker |  | 9994 | 13 | Code::CodeKey { rung: Names, file: packages/vite/src/node/server/openBrowser.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.521 |
