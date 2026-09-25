Score(3000)=0.459 I=0.801 C=0.263 ns_rows≤3K=18/55 grid(1000/1442/2080/3000/4327/6240/9000)=0.466/0.745/0.600/0.459/0.380/0.319/0.440

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
| ns | 4401 |  | 290 | UserConfig keys: server, build, env, worker and the rest | 3.2 | 3.1 | 0.366 |
| ns | 4571 |  | 170 | Per-environment options (SharedEnvironmentOptions / EnvironmentOptions) | 3.3 |  | 0.358 |
| ns | 4707 |  | 136 | CommonServerOptions: the host/port/https/proxy/cors keys | 3.4 |  | 0.353 |
| walker |  | 4748 | 406 | Fs::DirListing { dir: playground } |  |  | 0.357 |
| walker |  | 4759 | 11 | Fs::DirListing { dir: playground/resolve-linked } |  |  | 0.357 |
| walker |  | 4763 | 4 | Fs::DirListing { dir: playground/resolve-linked/src } |  |  | 0.357 |
| walker |  | 4788 | 25 | Code::CodeKey { rung: Names, file: playground/resolve-linked/src/index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.357 |
| walker |  | 4804 | 16 | Fs::DirListing { dir: playground/devtools } |  |  | 0.357 |
| walker |  | 4812 | 8 | Fs::DirListing { dir: playground/devtools/src } |  |  | 0.357 |
| walker |  | 4846 | 34 | Code::CodeKey { rung: Names, file: playground/devtools/src/main.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.357 |
| walker |  | 4863 | 17 | Fs::DirListing { dir: playground/css-lightningcss-proxy } |  |  | 0.357 |
| walker |  | 4880 | 17 | Fs::DirListing { dir: playground/css-lightningcss-root } |  |  | 0.357 |
| walker |  | 4897 | 17 | Fs::DirListing { dir: playground/hmr-root } |  |  | 0.357 |
| walker |  | 4901 | 4 | Fs::DirListing { dir: playground/hmr-root/root } |  |  | 0.357 |
| walker |  | 4919 | 18 | Fs::DirListing { dir: playground/client-reload } |  |  | 0.357 |
| walker |  | 4937 | 18 | Fs::DirListing { dir: playground/extensions } |  |  | 0.357 |
| ns | 4944 |  | 237 | ServerOptions and FileSystemServeOptions keys | 3.5 |  | 0.348 |
| walker |  | 4955 | 18 | Fs::DirListing { dir: playground/proxy-bypass } |  |  | 0.348 |
| walker |  | 4975 | 20 | Fs::DirListing { dir: playground/preserve-symlinks } |  |  | 0.348 |
| walker |  | 4979 | 4 | Fs::DirListing { dir: playground/preserve-symlinks/src } |  |  | 0.348 |
| walker |  | 5000 | 21 | Fs::DirListing { dir: playground/base-conflict } |  |  | 0.348 |
| walker |  | 5021 | 21 | Fs::DirListing { dir: playground/dynamic-import-inline } |  |  | 0.348 |
| walker |  | 5029 | 8 | Fs::DirListing { dir: playground/dynamic-import-inline/src } |  |  | 0.348 |
| walker |  | 5045 | 16 | Code::CodeKey { rung: Names, file: playground/dynamic-import-inline/src/index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.348 |
| walker |  | 5066 | 21 | Fs::DirListing { dir: playground/ssr-alias } |  |  | 0.348 |
| walker |  | 5074 | 8 | Fs::DirListing { dir: playground/ssr-alias/alias-original } |  |  | 0.348 |
| walker |  | 5095 | 21 | Fs::DirListing { dir: playground/ssr-pug } |  |  | 0.348 |
| walker |  | 5099 | 4 | Fs::DirListing { dir: playground/ssr-pug/src } |  |  | 0.348 |
| walker |  | 5120 | 21 | Fs::DirListing { dir: playground/ssr-wasm } |  |  | 0.348 |
| walker |  | 5131 | 11 | Fs::DirListing { dir: playground/preserve-symlinks/module-a } |  |  | 0.348 |
| walker |  | 5153 | 22 | Fs::DirListing { dir: playground/build-old } |  |  | 0.348 |
| walker |  | 5175 | 22 | Fs::DirListing { dir: playground/cli } |  |  | 0.348 |
| ns | 5180 |  | 236 | BuildEnvironmentOptions keys: output, assets, CSS, minification | 3.6 |  | 0.341 |
| walker |  | 5197 | 22 | Fs::DirListing { dir: playground/cli-module } |  |  | 0.341 |
| walker |  | 5219 | 22 | Fs::DirListing { dir: playground/object-hooks } |  |  | 0.341 |
| walker |  | 5236 | 17 | Code::CodeKey { rung: Names, file: playground/object-hooks/main.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.341 |
| walker |  | 5258 | 22 | Fs::DirListing { dir: playground/proxy-hmr } |  |  | 0.341 |
| walker |  | 5282 | 24 | Fs::DirListing { dir: playground/backend-integration } |  |  | 0.341 |
| walker |  | 5292 | 10 | Fs::DirListing { dir: playground/backend-integration/frontend } |  |  | 0.341 |
| walker |  | 5296 | 4 | Fs::DirListing { dir: playground/backend-integration/frontend/images } |  |  | 0.341 |
| walker |  | 5320 | 24 | Fs::DirListing { dir: playground/forward-console } |  |  | 0.341 |
| walker |  | 5324 | 4 | Fs::DirListing { dir: playground/forward-console/src } |  |  | 0.341 |
| walker |  | 5398 | 74 | Code::CodeKey { rung: Names, file: playground/forward-console/src/main.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.341 |
| walker |  | 5410 | 12 | Code::CodeKey { rung: Decl, file: playground/forward-console/src/main.ts, decl: 2, sub: 0, line: 25 } |  |  | 0.341 |
| walker |  | 5424 | 14 | Code::CodeKey { rung: Decl, file: playground/forward-console/src/main.ts, decl: 1, sub: 0, line: 3 } |  |  | 0.341 |
| ns | 5437 |  | 257 | BuildEnvironmentOptions keys: bundler passthrough, lib, ssr, reporting | 3.7 | 3.6 | 0.333 |
| walker |  | 5448 | 24 | Fs::DirListing { dir: playground/optimize-deps-no-discovery } |  |  | 0.333 |
| walker |  | 5456 | 8 | Fs::DirListing { dir: playground/optimize-deps-no-discovery/dep-no-discovery } |  |  | 0.333 |
| walker |  | 5481 | 25 | Fs::DirListing { dir: playground/import-assertion } |  |  | 0.333 |
| walker |  | 5493 | 12 | Fs::DirListing { dir: playground/import-assertion/import-assertion-dep } |  |  | 0.333 |
| walker |  | 5518 | 25 | Fs::DirListing { dir: playground/ssr } |  |  | 0.333 |
| ns | 5530 |  | 93 | ExperimentalOptions and FutureOptions | 3.8 |  | 0.330 |
| walker |  | 5543 | 25 | Fs::DirListing { dir: playground/tsconfig-json-load-error } |  |  | 0.330 |
| walker |  | 5547 | 4 | Fs::DirListing { dir: playground/tsconfig-json-load-error/src } |  |  | 0.330 |
| walker |  | 5556 | 9 | Fs::DirListing { dir: playground/tsconfig-json-load-error/has-error } |  |  | 0.330 |
| walker |  | 5569 | 13 | Fs::DirListing { dir: playground/backend-integration/dir } |  |  | 0.330 |
| walker |  | 5582 | 13 | Fs::DirListing { dir: playground/proxy-hmr/other-app } |  |  | 0.330 |
| walker |  | 5608 | 26 | Fs::DirListing { dir: playground/env-nested } |  |  | 0.330 |
| walker |  | 5619 | 11 | Fs::DirListing { dir: playground/env-nested/envs } |  |  | 0.330 |
| walker |  | 5645 | 26 | Fs::DirListing { dir: playground/environment-react-ssr } |  |  | 0.330 |
| walker |  | 5653 | 8 | Fs::DirListing { dir: playground/preserve-symlinks/module-a/src } |  |  | 0.330 |
| walker |  | 5667 | 14 | Code::CodeKey { rung: Names, file: playground/preserve-symlinks/module-a/src/index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.330 |
| walker |  | 5675 | 8 | Code::CodeKey { rung: Body, file: playground/preserve-symlinks/module-a/src/index.js, decl: 1, sub: 0, line: 3 } |  |  | 0.330 |
| walker |  | 5690 | 15 | Fs::DirListing { dir: playground/base-conflict/src } |  |  | 0.330 |
| walker |  | 5705 | 15 | Fs::DirListing { dir: playground/ssr-alias/src } |  |  | 0.330 |
| walker |  | 5716 | 11 | Code::CodeKey { rung: Names, file: playground/ssr-alias/src/main.js, decl: 0, sub: 0, line: 0 } |  |  | 0.330 |
| walker |  | 5749 | 33 | Code::CodeKey { rung: Decl, file: playground/ssr-alias/src/main.js, decl: 1, sub: 0, line: 5 } |  |  | 0.330 |
| walker |  | 5778 | 29 | Fs::DirListing { dir: playground/minify } |  |  | 0.330 |
| walker |  | 5781 | 3 | Fs::DirListing { dir: playground/minify/dir } |  |  | 0.330 |
| ns | 5804 |  | 274 | CLI global options (all five commands) | 3.9 | 2.2 | 0.324 |
| walker |  | 5810 | 29 | Fs::DirListing { dir: playground/resolve-tsconfig-paths } |  |  | 0.324 |
| walker |  | 5814 | 4 | Fs::DirListing { dir: playground/resolve-tsconfig-paths/fallback } |  |  | 0.324 |
| walker |  | 5843 | 29 | Fs::DirListing { dir: playground/tailwind-sourcemap } |  |  | 0.324 |
| walker |  | 5873 | 30 | Fs::DirListing { dir: playground/alias } |  |  | 0.324 |
| walker |  | 5903 | 30 | Fs::DirListing { dir: playground/tsconfig-json } |  |  | 0.324 |
| walker |  | 5917 | 14 | Fs::DirListing { dir: playground/tsconfig-json/src } |  |  | 0.324 |
| walker |  | 5953 | 36 | Code::CodeKey { rung: Names, file: playground/tsconfig-json/src/main.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.324 |
| walker |  | 5970 | 17 | Code::CodeKey { rung: Decl, file: playground/tsconfig-json/src/main.ts, decl: 1, sub: 0, line: 9 } |  |  | 0.324 |
| walker |  | 5985 | 15 | Fs::DirListing { dir: playground/tsconfig-json/nested } |  |  | 0.324 |
| ns | 5995 |  | 191 | CLI dev-server flags | 3.10 | 2.2 | 0.319 |
| walker |  | 6021 | 36 | Code::CodeKey { rung: Names, file: playground/tsconfig-json/nested/main.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.319 |
| walker |  | 6038 | 17 | Code::CodeKey { rung: Decl, file: playground/tsconfig-json/nested/main.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.319 |
| walker |  | 6053 | 15 | Fs::DirListing { dir: playground/tsconfig-json/nested-with-extends } |  |  | 0.319 |
| walker |  | 6101 | 48 | Code::CodeKey { rung: Names, file: playground/tsconfig-json/nested-with-extends/main.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.319 |
| walker |  | 6118 | 17 | Code::CodeKey { rung: Decl, file: playground/tsconfig-json/nested-with-extends/main.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.319 |
| walker |  | 6149 | 31 | Fs::DirListing { dir: playground/module-graph } |  |  | 0.319 |
| walker |  | 6180 | 31 | Fs::DirListing { dir: playground/ssr-webworker } |  |  | 0.319 |
| walker |  | 6189 | 9 | Fs::DirListing { dir: playground/ssr-webworker/src } |  |  | 0.319 |
| walker |  | 6201 | 12 | Fs::DirListing { dir: playground/ssr-webworker/browser-exports } |  |  | 0.319 |
| walker |  | 6217 | 16 | Fs::DirListing { dir: playground/ssr-webworker/worker-exports } |  |  | 0.319 |
| walker |  | 6249 | 32 | Fs::DirListing { dir: playground/optimize-missing-deps } |  |  | 0.319 |
| walker |  | 6257 | 8 | Fs::DirListing { dir: playground/optimize-missing-deps/missing-dep } |  |  | 0.319 |
| walker |  | 6270 | 13 | Fs::DirListing { dir: playground/optimize-missing-deps/multi-entry-dep } |  |  | 0.319 |
| walker |  | 6287 | 17 | Fs::DirListing { dir: playground/environment-react-ssr/src } |  |  | 0.319 |
| walker |  | 6320 | 33 | Fs::DirListing { dir: playground/css-dynamic-import } |  |  | 0.319 |
| walker |  | 6353 | 33 | Fs::DirListing { dir: playground/define } |  |  | 0.319 |
| walker |  | 6361 | 8 | Fs::DirListing { dir: playground/define/commonjs-dep } |  |  | 0.319 |
| walker |  | 6394 | 33 | Fs::DirListing { dir: playground/ssr-conditions } |  |  | 0.319 |
| walker |  | 6398 | 4 | Fs::DirListing { dir: playground/ssr-conditions/src } |  |  | 0.319 |
| walker |  | 6432 | 34 | Fs::DirListing { dir: playground/css-codesplit-cjs } |  |  | 0.319 |
| walker |  | 6466 | 34 | Fs::DirListing { dir: playground/tailwind } |  |  | 0.319 |
| ns | 6467 |  | 472 | CLI build flags | 3.11 | 2.2 | 0.306 |
| walker |  | 6470 | 4 | Fs::DirListing { dir: playground/tailwind/public } |  |  | 0.306 |
| walker |  | 6480 | 10 | Fs::DirListing { dir: playground/tailwind/src } |  |  | 0.306 |
| walker |  | 6502 | 22 | Code::CodeKey { rung: Names, file: playground/tailwind/src/main.js, decl: 0, sub: 0, line: 0 } |  |  | 0.306 |
| walker |  | 6507 | 5 | Fs::DirListing { dir: playground/tailwind/src/components } |  |  | 0.306 |
| walker |  | 6512 | 5 | Fs::DirListing { dir: playground/tailwind/src/views } |  |  | 0.306 |
| walker |  | 6604 | 92 | Code::CodeKey { rung: Decl, file: playground/tailwind/src/main.js, decl: 1, sub: 0, line: 3 } |  |  | 0.306 |
| walker |  | 6622 | 18 | Fs::DirListing { dir: playground/css-lightningcss-root/root } |  |  | 0.303 |
| ns | 6622 |  | 155 | CLI optimize and preview flags | 3.12 | 2.2 | 0.303 |
| walker |  | 6657 | 35 | Fs::DirListing { dir: playground/fs-serve } |  |  | 0.303 |
| walker |  | 6661 | 4 | Fs::DirListing { dir: playground/fs-serve/nested } |  |  | 0.303 |
| walker |  | 6697 | 36 | Fs::DirListing { dir: playground/assets-sanitize } |  |  | 0.303 |
| walker |  | 6733 | 36 | Fs::DirListing { dir: playground/external } |  |  | 0.303 |
| walker |  | 6741 | 8 | Fs::DirListing { dir: playground/external/dep-that-imports } |  |  | 0.303 |
| walker |  | 6749 | 8 | Fs::DirListing { dir: playground/external/dep-that-requires } |  |  | 0.303 |
| ns | 6757 |  | 135 | src/node module roster (complete) | 4.1 |  | 0.357 |
| walker |  | 6759 | 10 | Fs::DirListing { dir: playground/external/public } |  |  | 0.357 |
| walker |  | 6769 | 10 | Fs::DirListing { dir: playground/external/src } |  |  | 0.357 |
| walker |  | 6788 | 19 | Fs::DirListing { dir: playground/resolve-tsconfig-paths/src } |  |  | 0.357 |
| walker |  | 6829 | 41 | Fs::DirListing { dir: playground/resolve-tsconfig-paths/src/nested } |  |  | 0.357 |
| walker |  | 6866 | 37 | Fs::DirListing { dir: playground/env } |  |  | 0.357 |
| walker |  | 6903 | 37 | Fs::DirListing { dir: playground/ssr-noexternal } |  |  | 0.357 |
| ns | 6906 |  | 149 | src/node/plugins roster (complete) | 4.2 |  | 0.395 |
| walker |  | 6908 | 5 | Fs::DirListing { dir: playground/ssr-noexternal/src } |  |  | 0.395 |
| walker |  | 6916 | 8 | Fs::DirListing { dir: playground/ssr-noexternal/require-external-cjs } |  |  | 0.395 |
| walker |  | 6930 | 14 | Fs::DirListing { dir: playground/ssr-noexternal/external-cjs } |  |  | 0.395 |
| walker |  | 6967 | 37 | Fs::DirListing { dir: playground/tailwind-v3 } |  |  | 0.395 |
| walker |  | 6977 | 10 | Fs::DirListing { dir: playground/tailwind-v3/src } |  |  | 0.395 |
| ns | 6981 |  | 75 | src/node/server roster (complete) | 4.3 |  | 0.389 |
| walker |  | 6999 | 22 | Code::CodeKey { rung: Names, file: playground/tailwind-v3/src/main.js, decl: 0, sub: 0, line: 0 } |  |  | 0.389 |
| walker |  | 7004 | 5 | Fs::DirListing { dir: playground/tailwind-v3/src/components } |  |  | 0.389 |
| walker |  | 7009 | 5 | Fs::DirListing { dir: playground/tailwind-v3/src/views } |  |  | 0.389 |
| ns | 7066 |  | 85 | Dev-server middlewares and per-environment implementations | 4.4 |  | 0.383 |
| walker |  | 7101 | 92 | Code::CodeKey { rung: Decl, file: playground/tailwind-v3/src/main.js, decl: 1, sub: 0, line: 3 } |  |  | 0.383 |
| walker |  | 7140 | 39 | Fs::DirListing { dir: playground/csp } |  |  | 0.383 |
| ns | 7144 |  | 78 | SSR and dependency-optimizer rosters | 4.5 |  | 0.381 |
| walker |  | 7179 | 39 | Fs::DirListing { dir: playground/ssr-resolve } |  |  | 0.381 |
| walker |  | 7187 | 8 | Fs::DirListing { dir: playground/ssr-resolve/pkg-module-sync } |  |  | 0.381 |
| walker |  | 7198 | 11 | Fs::DirListing { dir: playground/ssr-resolve/entries } |  |  | 0.381 |
| walker |  | 7202 | 4 | Fs::DirListing { dir: playground/ssr-resolve/entries/dir } |  |  | 0.381 |
| walker |  | 7214 | 12 | Fs::DirListing { dir: playground/ssr-resolve/pkg-exports } |  |  | 0.381 |
| walker |  | 7231 | 17 | Fs::DirListing { dir: playground/ssr-resolve/deep-import } |  |  | 0.381 |
| walker |  | 7235 | 4 | Fs::DirListing { dir: playground/ssr-resolve/deep-import/bar } |  |  | 0.381 |
| walker |  | 7239 | 4 | Fs::DirListing { dir: playground/ssr-resolve/deep-import/utils } |  |  | 0.381 |
| walker |  | 7247 | 8 | Fs::DirListing { dir: playground/ssr-resolve/deep-import/foo } |  |  | 0.381 |
| walker |  | 7259 | 12 | Fs::DirListing { dir: playground/minify/dir/module } |  |  | 0.381 |
| ns | 7287 |  | 143 | Browser client, module-runner and shared rosters | 4.6 |  | 0.388 |
| walker |  | 7299 | 40 | Fs::DirListing { dir: playground/json } |  |  | 0.388 |
| walker |  | 7303 | 4 | Fs::DirListing { dir: playground/json/public } |  |  | 0.388 |
| walker |  | 7309 | 6 | Fs::DirListing { dir: playground/json/json-bom } |  |  | 0.388 |
| walker |  | 7317 | 8 | Fs::DirListing { dir: playground/json/json-module } |  |  | 0.388 |
| walker |  | 7329 | 12 | Fs::DirListing { dir: playground/json/dep-json-require } |  |  | 0.388 |
| walker |  | 7385 | 56 | Json::Identity { file: playground/package.json } |  |  | 0.388 |
| ns | 7400 |  | 113 | resolvePlugins: the built-in plugin pipeline order | 4.7 |  | 0.384 |
| walker |  | 7426 | 41 | Fs::DirListing { dir: playground/css-no-codesplit } |  |  | 0.384 |
| walker |  | 7467 | 41 | Fs::DirListing { dir: playground/dynamic-import } |  |  | 0.384 |
| walker |  | 7471 | 4 | Fs::DirListing { dir: playground/dynamic-import/css } |  |  | 0.384 |
| walker |  | 7478 | 7 | Fs::DirListing { dir: playground/dynamic-import/(app) } |  |  | 0.384 |
| walker |  | 7482 | 4 | Fs::DirListing { dir: playground/dynamic-import/(app)/nest } |  |  | 0.384 |
| walker |  | 7492 | 10 | Fs::DirListing { dir: playground/dynamic-import/files } |  |  | 0.384 |
| ns | 7494 |  | 94 | Published and inlined type declarations | 4.8 |  | 0.401 |
| walker |  | 7504 | 12 | Fs::DirListing { dir: playground/dynamic-import/pkg } |  |  | 0.401 |
| walker |  | 7520 | 16 | Fs::DirListing { dir: playground/dynamic-import/alias } |  |  | 0.401 |
| walker |  | 7537 | 17 | Fs::DirListing { dir: playground/dynamic-import/views } |  |  | 0.401 |
| walker |  | 7580 | 43 | Fs::DirListing { dir: playground/ssr-html } |  |  | 0.401 |
| walker |  | 7590 | 10 | Fs::DirListing { dir: playground/ssr-html/public } |  |  | 0.401 |
| ns | 7609 |  | 115 | Core internal entry-point signatures | 4.9 |  | 0.397 |
| walker |  | 7633 | 43 | Fs::DirListing { dir: playground/wasm } |  |  | 0.397 |
| walker |  | 7656 | 23 | Fs::DirListing { dir: playground/alias/dir } |  |  | 0.397 |
| walker |  | 7664 | 8 | Fs::DirListing { dir: playground/alias/dir/module } |  |  | 0.397 |
| walker |  | 7709 | 45 | Fs::DirListing { dir: playground/hmr-full-bundle-mode } |  |  | 0.397 |
| walker |  | 7754 | 45 | Fs::DirListing { dir: playground/transform-plugin } |  |  | 0.397 |
| walker |  | 7778 | 24 | Fs::DirListing { dir: playground/ssr-wasm/src } |  |  | 0.397 |
| walker |  | 7826 | 48 | Fs::DirListing { dir: playground/multiple-entrypoints } |  |  | 0.397 |
| walker |  | 7877 | 51 | Fs::DirListing { dir: playground/preload } |  |  | 0.397 |
| walker |  | 7882 | 5 | Fs::DirListing { dir: playground/preload/public } |  |  | 0.397 |
| walker |  | 7890 | 8 | Fs::DirListing { dir: playground/preload/dep-a } |  |  | 0.397 |
| walker |  | 7898 | 8 | Fs::DirListing { dir: playground/preload/dep-including-a } |  |  | 0.397 |
| walker |  | 7919 | 21 | Fs::DirListing { dir: playground/preload/src } |  |  | 0.388 |
| ns | 7919 |  | 310 | CONTRIBUTING.md: every section heading | 5.1 |  | 0.388 |
| walker |  | 7931 | 12 | Code::CodeKey { rung: Names, file: playground/preload/src/main.js, decl: 0, sub: 0, line: 0 } |  |  | 0.388 |
| walker |  | 7942 | 11 | Code::CodeKey { rung: Names, file: playground/resolve-tsconfig-paths/src/imported.js, decl: 0, sub: 0, line: 0 } |  |  | 0.388 |
| walker |  | 7953 | 11 | Code::CodeKey { rung: Names, file: playground/ssr-webworker/src/dynamic.js, decl: 0, sub: 0, line: 0 } |  |  | 0.388 |
| walker |  | 7980 | 27 | Fs::DirListing { dir: playground/ssr-conditions/external } |  |  | 0.388 |
| walker |  | 8007 | 27 | Fs::DirListing { dir: playground/ssr-conditions/no-external } |  |  | 0.388 |
| walker |  | 8060 | 53 | Fs::DirListing { dir: playground/nested-deps } |  |  | 0.388 |
| walker |  | 8068 | 8 | Fs::DirListing { dir: playground/nested-deps/test-package-a } |  |  | 0.388 |
| walker |  | 8076 | 8 | Fs::DirListing { dir: playground/nested-deps/test-package-f } |  |  | 0.388 |
| walker |  | 8087 | 11 | Fs::DirListing { dir: playground/nested-deps/self-referencing } |  |  | 0.388 |
| walker |  | 8099 | 12 | Fs::DirListing { dir: playground/nested-deps/test-package-b } |  |  | 0.388 |
| ns | 8109 |  | 190 | CONTRIBUTING: local development loop | 5.2 | 5.1 | 0.386 |
| walker |  | 8114 | 15 | Fs::DirListing { dir: playground/nested-deps/test-package-d } |  |  | 0.386 |
| walker |  | 8122 | 8 | Fs::DirListing { dir: playground/nested-deps/test-package-d/test-package-d-nested } |  |  | 0.386 |
| walker |  | 8139 | 17 | Fs::DirListing { dir: playground/nested-deps/test-package-c } |  |  | 0.386 |
| walker |  | 8161 | 22 | Fs::DirListing { dir: playground/nested-deps/test-package-e } |  |  | 0.386 |
| walker |  | 8169 | 8 | Fs::DirListing { dir: playground/nested-deps/test-package-e/test-package-e-excluded } |  |  | 0.386 |
| walker |  | 8177 | 8 | Fs::DirListing { dir: playground/nested-deps/test-package-e/test-package-e-included } |  |  | 0.386 |
| walker |  | 8194 | 17 | Fs::DirListing { dir: playground/backend-integration/frontend/styles } |  |  | 0.386 |
| walker |  | 8224 | 30 | Fs::DirListing { dir: playground/ssr/src } |  |  | 0.386 |
| walker |  | 8236 | 12 | Fs::DirListing { dir: playground/ssr/src/circular-import } |  |  | 0.386 |
| walker |  | 8248 | 12 | Fs::DirListing { dir: playground/ssr/src/circular-import2 } |  |  | 0.386 |
| walker |  | 8291 | 43 | Json::Identity { file: playground/css-lightningcss-root/package.json } |  |  | 0.386 |
| walker |  | 8304 | 13 | Code::CodeKey { rung: Names, file: playground/resolve-tsconfig-paths/src/js.js, decl: 0, sub: 0, line: 0 } |  |  | 0.386 |
| walker |  | 8317 | 13 | Code::CodeKey { rung: Names, file: playground/resolve-tsconfig-paths/src/ts.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.386 |
| walker |  | 8330 | 13 | Code::CodeKey { rung: Names, file: playground/ssr-wasm/src/app.js, decl: 0, sub: 0, line: 0 } |  |  | 0.386 |
| walker |  | 8362 | 32 | Fs::DirListing { dir: playground/dynamic-import/nested } |  |  | 0.386 |
| walker |  | 8366 | 4 | Fs::DirListing { dir: playground/dynamic-import/nested/nested } |  |  | 0.386 |
| walker |  | 8376 | 10 | Fs::DirListing { dir: playground/dynamic-import/nested/treeshaken } |  |  | 0.386 |
| walker |  | 8408 | 32 | Fs::DirListing { dir: playground/ssr-html/src } |  |  | 0.386 |
| walker |  | 8418 | 10 | Code::CodeKey { rung: Names, file: playground/ssr-html/src/network-imports.js, decl: 0, sub: 0, line: 0 } |  |  | 0.386 |
| walker |  | 8429 | 11 | Code::CodeKey { rung: Names, file: playground/ssr-html/src/error-js.js, decl: 0, sub: 0, line: 0 } |  |  | 0.386 |
| walker |  | 8440 | 11 | Code::CodeKey { rung: Names, file: playground/ssr-html/src/error-ts.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.386 |
| walker |  | 8451 | 11 | Code::CodeKey { rung: Names, file: playground/ssr-html/src/importedVirtual.js, decl: 0, sub: 0, line: 0 } |  |  | 0.386 |
| walker |  | 8513 | 62 | Fs::DirListing { dir: playground/data-uri } |  |  | 0.386 |
| ns | 8515 |  | 406 | playground/ roster (complete e2e corpus) | 5.3 |  | 0.449 |
| walker |  | 8557 | 44 | Json::Identity { file: playground/css-lightningcss-proxy/package.json } |  |  | 0.449 |
| walker |  | 8566 | 9 | Code::CodeKey { rung: Body, file: playground/forward-console/src/main.ts, decl: 5, sub: 0, line: 37 } |  |  | 0.449 |
| walker |  | 8580 | 14 | Code::CodeKey { rung: Names, file: playground/environment-react-ssr/src/root.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.449 |
| walker |  | 8594 | 14 | Code::CodeKey { rung: Names, file: playground/resolve-tsconfig-paths/src/hash.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.449 |
| walker |  | 8608 | 14 | Code::CodeKey { rung: Names, file: playground/ssr-html/src/has-error-deep.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.449 |
| walker |  | 8622 | 14 | Code::CodeKey { rung: Names, file: playground/ssr-wasm/src/static-heavy.js, decl: 0, sub: 0, line: 0 } |  |  | 0.449 |
| walker |  | 8636 | 14 | Code::CodeKey { rung: Names, file: playground/ssr-wasm/src/static-light.js, decl: 0, sub: 0, line: 0 } |  |  | 0.449 |
| walker |  | 8695 | 59 | Code::CodeKey { rung: Decl, file: playground/tsconfig-json/nested/main.ts, decl: 3, sub: 0, line: 10 } |  |  | 0.449 |
| walker |  | 8754 | 59 | Code::CodeKey { rung: Decl, file: playground/tsconfig-json/src/main.ts, decl: 3, sub: 0, line: 14 } |  |  | 0.449 |
| walker |  | 8775 | 21 | Fs::DirListing { dir: playground/ssr/src/circular-dep-init } |  |  | 0.449 |
| walker |  | 8790 | 15 | Code::CodeKey { rung: Names, file: playground/ssr/src/utils.js, decl: 0, sub: 0, line: 0 } |  |  | 0.449 |
| ns | 8802 |  | 287 | CONTRIBUTING: how the integration tests work | 5.4 | 5.3 | 0.447 |
| walker |  | 8805 | 15 | Code::CodeKey { rung: Names, file: playground/ssr-conditions/src/app.js, decl: 0, sub: 0, line: 0 } |  |  | 0.447 |
| walker |  | 8867 | 62 | Code::CodeKey { rung: Decl, file: playground/tsconfig-json/nested-with-extends/main.ts, decl: 3, sub: 0, line: 10 } |  |  | 0.447 |
| walker |  | 8897 | 30 | Json::Identity { file: playground/nested-deps/self-referencing/package.json } |  |  | 0.447 |
| ns | 8919 |  | 117 | Unit test locations under packages/vite/src | 5.5 |  | 0.440 |
| walker |  | 8951 | 54 | Json::Identity { file: playground/env/package.json } |  |  | 0.440 |
| walker |  | 9005 | 54 | Json::Identity { file: playground/json/package.json } |  |  | 0.440 |
| ns | 9050 |  | 131 | How the unit and e2e vitest runs are separated | 5.6 |  | 0.438 |
| walker |  | 9060 | 55 | Json::Identity { file: playground/alias/package.json } |  |  | 0.438 |
| ns | 9114 |  | 64 | docs/ site roster | 5.7 |  | 0.448 |
| walker |  | 9115 | 55 | Json::Identity { file: playground/build-old/package.json } |  |  | 0.448 |
| walker |  | 9170 | 55 | Json::Identity { file: playground/cli/package.json } |  |  | 0.448 |
| walker |  | 9225 | 55 | Json::Identity { file: playground/cli-module/package.json } |  |  | 0.448 |
| walker |  | 9280 | 55 | Json::Identity { file: playground/csp/package.json } |  |  | 0.448 |
| ns | 9298 |  | 184 | docs/guide and docs/config page rosters | 5.8 |  | 0.439 |
| walker |  | 9335 | 55 | Json::Identity { file: playground/data-uri/package.json } |  |  | 0.439 |
| ns | 9343 |  | 45 | docs/changes: the breaking-change / migration notes | 5.9 |  | 0.437 |
| walker |  | 9390 | 55 | Json::Identity { file: playground/define/package.json } |  |  | 0.437 |
| walker |  | 9445 | 55 | Json::Identity { file: playground/devtools/package.json } |  |  | 0.437 |
| walker |  | 9500 | 55 | Json::Identity { file: playground/extensions/package.json } |  |  | 0.437 |
| ns | 9501 |  | 158 | CONTRIBUTING: the dependency policy | 5.10 | 5.1 | 0.435 |
| walker |  | 9555 | 55 | Json::Identity { file: playground/external/package.json } |  |  | 0.435 |
| walker |  | 9610 | 55 | Json::Identity { file: playground/forward-console/package.json } |  |  | 0.435 |
| ns | 9624 |  | 123 | create-vite: package layout and template roster | 5.11 |  | 0.449 |
| walker |  | 9665 | 55 | Json::Identity { file: playground/minify/package.json } |  |  | 0.449 |
| ns | 9677 |  | 53 | plugin-legacy: package layout | 5.12 |  | 0.456 |
| walker |  | 9720 | 55 | Json::Identity { file: playground/module-graph/package.json } |  |  | 0.456 |
| walker |  | 9775 | 55 | Json::Identity { file: playground/object-hooks/package.json } |  |  | 0.456 |
| ns | 9827 |  | 150 | Repository automation: .github and release scripts | 5.13 |  | 0.469 |
| walker |  | 9830 | 55 | Json::Identity { file: playground/preload/package.json } |  |  | 0.469 |
| walker |  | 9885 | 55 | Json::Identity { file: playground/tailwind/package.json } |  |  | 0.469 |
| walker |  | 9940 | 55 | Json::Identity { file: playground/transform-plugin/package.json } |  |  | 0.469 |
| walker |  | 9995 | 55 | Json::Identity { file: playground/wasm/package.json } |  |  | 0.469 |
