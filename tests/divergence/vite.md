Score(3000)=0.503 I=0.862 C=0.294 ns_rows≤3K=18/55 grid(1000/1442/2080/3000/4327/6240/9000)=0.802/0.791/0.650/0.503/0.414/0.343/0.482

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 91 | 91 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 100 |  | 100 | README title, tagline and feature bullets | 1.1 |  | 0.000 |
| walker |  | 104 | 13 | Fs::DirListing { dir: packages } |  |  | 0.000 |
| ns | 113 |  | 13 | The three workspace packages | 1.2 |  | 0.292 |
| ns | 130 |  | 17 | packages/vite/src top-level split | 1.3 |  | 0.233 |
| ns | 205 |  | 75 | README: Vite's one-sentence definition | 1.4 | 1.1 | 0.225 |
| walker |  | 289 | 185 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.850 |
| ns | 296 |  | 91 | Repository root listing (complete) | 1.5 |  | 0.898 |
| walker |  | 331 | 42 | Json::Identity { file: package.json } |  |  | 0.901 |
| walker |  | 364 | 33 | Fs::DirListing { dir: patches } |  |  | 0.901 |
| walker |  | 400 | 36 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.901 |
| walker |  | 430 | 30 | Fs::DirListing { dir: packages/plugin-legacy } |  |  | 0.901 |
| walker |  | 453 | 23 | Fs::DirListing { dir: packages/plugin-legacy/src } |  |  | 0.902 |
| ns | 466 |  | 170 | README: dev server vs. build command, and extensibility | 1.6 | 1.4 | 0.839 |
| ns | 543 |  | 77 | packages/vite top-level listing | 1.7 |  | 0.697 |
| walker |  | 595 | 142 | Json::Scripts { file: package.json } |  |  | 0.700 |
| ns | 636 |  | 93 | Root package.json identity, engines, package manager | 1.8 |  | 0.670 |
| walker |  | 718 | 123 | Fs::DirListing { dir: packages/create-vite } |  |  | 0.672 |
| walker |  | 722 | 4 | Fs::DirListing { dir: packages/create-vite/src } |  |  | 0.672 |
| walker |  | 727 | 5 | Fs::DirListing { dir: packages/create-vite/__tests__ } |  |  | 0.672 |
| walker |  | 781 | 54 | Markdown::CommandBlock { file: CONTRIBUTING.md, row: 28 } |  |  | 0.672 |
| walker |  | 840 | 59 | Json::Runtime { file: package.json } |  |  | 0.711 |
| ns | 859 |  | 223 | Root pnpm scripts: lint, typecheck and the test entry points | 1.9 |  | 0.657 |
| walker |  | 917 | 77 | Fs::DirListing { dir: packages/vite } |  |  | 0.799 |
| walker |  | 926 | 9 | Fs::DirListing { dir: packages/vite/bin } |  |  | 0.799 |
| walker |  | 999 | 73 | Json::Identity { file: packages/vite/package.json } |  |  | 0.802 |
| walker |  | 1016 | 17 | Fs::DirListing { dir: packages/vite/src } |  |  | 0.867 |
| walker |  | 1087 | 71 | Fs::DirListing { dir: packages/vite/src/module-runner } |  |  | 0.867 |
| walker |  | 1099 | 12 | Fs::DirListing { dir: packages/vite/src/module-runner/sourcemap } |  |  | 0.867 |
| walker |  | 1117 | 18 | Fs::DirListing { dir: packages/vite/misc } |  |  | 0.867 |
| ns | 1118 |  | 259 | Root pnpm scripts: debug, docs, build, release | 1.10 | 1.9 | 0.801 |
| walker |  | 1157 | 40 | Json::Runtime { file: packages/vite/package.json } |  |  | 0.801 |
| walker |  | 1174 | 17 | Fs::DirListing { dir: packages/vite/src/client } |  |  | 0.802 |
| ns | 1225 |  | 107 | vite package manifest: name, version, description, bin | 1.11 |  | 0.784 |
| walker |  | 1309 | 135 | Fs::DirListing { dir: packages/vite/src/node } |  |  | 0.788 |
| walker |  | 1338 | 29 | Fs::DirListing { dir: packages/vite/src/node/optimizer } |  |  | 0.789 |
| walker |  | 1387 | 49 | Fs::DirListing { dir: packages/vite/src/node/ssr } |  |  | 0.790 |
| walker |  | 1462 | 75 | Fs::DirListing { dir: packages/vite/src/node/server } |  |  | 0.793 |
| ns | 1485 |  | 260 | vite package exports and internal import aliases | 1.12 | 1.11 | 0.702 |
| walker |  | 1611 | 149 | Fs::DirListing { dir: packages/vite/src/node/plugins } |  |  | 0.705 |
| walker |  | 1621 | 10 | Fs::DirListing { dir: packages/vite/src/module-runner/__tests_dts__ } |  |  | 0.705 |
| walker |  | 1632 | 11 | Fs::DirListing { dir: packages/vite/scripts } |  |  | 0.705 |
| ns | 1640 |  | 155 | src/node/index.ts: primary API factories | 2.1 |  | 0.662 |
| walker |  | 1643 | 11 | Fs::DirListing { dir: packages/vite/src/node/ssr/runtime } |  |  | 0.662 |
| walker |  | 1687 | 44 | Fs::DirListing { dir: packages/vite/types } |  |  | 0.662 |
| ns | 1853 |  | 213 | vite CLI: the complete command roster | 2.2 |  | 0.619 |
| walker |  | 1877 | 190 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.649 |
| walker |  | 1907 | 30 | Fs::DirListing { dir: scripts } |  |  | 0.649 |
| walker |  | 1923 | 16 | Fs::DirListing { dir: packages/plugin-legacy/src/__tests__ } |  |  | 0.649 |
| walker |  | 1960 | 37 | Fs::DirListing { dir: packages/vite/types/internal } |  |  | 0.649 |
| walker |  | 2001 | 41 | Fs::DirListing { dir: .github } |  |  | 0.649 |
| walker |  | 2080 | 79 | Fs::DirListing { dir: .github/workflows } |  |  | 0.650 |
| ns | 2118 |  | 265 | src/node/index.ts: transform helpers and environment factories | 2.3 | 2.1 | 0.602 |
| walker |  | 2144 | 64 | Fs::DirListing { dir: docs } |  |  | 0.603 |
| walker |  | 2148 | 4 | Fs::DirListing { dir: docs/plugins } |  |  | 0.603 |
| walker |  | 2163 | 15 | Fs::DirListing { dir: docs/_data } |  |  | 0.603 |
| walker |  | 2181 | 18 | Fs::DirListing { dir: packages/vite/src/node/server/environments } |  |  | 0.603 |
| walker |  | 2200 | 19 | Fs::DirListing { dir: docs/.vitepress } |  |  | 0.603 |
| walker |  | 2209 | 9 | Fs::DirListing { dir: docs/.vitepress/inlined-scripts } |  |  | 0.603 |
| walker |  | 2228 | 19 | Fs::DirListing { dir: packages/create-vite/template-lit } |  |  | 0.603 |
| walker |  | 2240 | 12 | Fs::DirListing { dir: packages/create-vite/template-lit/src } |  |  | 0.603 |
| ns | 2242 |  | 124 | src/node/index.ts: SSR/module-runner exports | 2.4 | 2.3 | 0.587 |
| walker |  | 2261 | 21 | Fs::DirListing { dir: docs/.vitepress/theme } |  |  | 0.587 |
| walker |  | 2265 | 4 | Fs::DirListing { dir: docs/.vitepress/theme/composables } |  |  | 0.587 |
| walker |  | 2287 | 22 | Fs::DirListing { dir: packages/vite/src/node/__tests_dts__ } |  |  | 0.587 |
| walker |  | 2421 | 134 | Json::Dependencies { file: packages/vite/package.json } |  |  | 0.587 |
| walker |  | 2471 | 50 | Fs::DirListing { dir: packages/vite/src/types } |  |  | 0.589 |
| walker |  | 2490 | 19 | Json::Identity { file: packages/vite/src/types/package.json } |  |  | 0.589 |
| ns | 2571 |  | 329 | src/node/index.ts: constants, utils and remaining value exports | 2.5 | 2.4 | 0.538 |
| ns | 2859 |  | 288 | index.ts type re-exports: config, server, build and plugin-option types | 2.6 | 2.5 | 0.498 |
| walker |  | 2896 | 406 | Fs::DirListing { dir: playground } |  |  | 0.503 |
| walker |  | 2918 | 22 | Fs::DirListing { dir: playground/cli } |  |  | 0.503 |
| walker |  | 2940 | 22 | Fs::DirListing { dir: playground/cli-module } |  |  | 0.503 |
| walker |  | 2951 | 11 | Fs::DirListing { dir: playground/resolve-linked } |  |  | 0.503 |
| walker |  | 2955 | 4 | Fs::DirListing { dir: playground/resolve-linked/src } |  |  | 0.503 |
| walker |  | 2988 | 33 | Fs::DirListing { dir: playground/css-dynamic-import } |  |  | 0.503 |
| walker |  | 3024 | 36 | Fs::DirListing { dir: playground/assets-sanitize } |  |  | 0.503 |
| walker |  | 3061 | 37 | Fs::DirListing { dir: playground/env } |  |  | 0.503 |
| walker |  | 3077 | 16 | Fs::DirListing { dir: playground/devtools } |  |  | 0.503 |
| walker |  | 3085 | 8 | Fs::DirListing { dir: playground/devtools/src } |  |  | 0.503 |
| walker |  | 3124 | 39 | Fs::DirListing { dir: playground/csp } |  |  | 0.503 |
| walker |  | 3141 | 17 | Fs::DirListing { dir: playground/css-lightningcss-proxy } |  |  | 0.503 |
| walker |  | 3158 | 17 | Fs::DirListing { dir: playground/css-lightningcss-root } |  |  | 0.503 |
| walker |  | 3175 | 17 | Fs::DirListing { dir: playground/hmr-root } |  |  | 0.503 |
| walker |  | 3179 | 4 | Fs::DirListing { dir: playground/hmr-root/root } |  |  | 0.503 |
| walker |  | 3220 | 41 | Fs::DirListing { dir: playground/css-no-codesplit } |  |  | 0.503 |
| walker |  | 3238 | 18 | Fs::DirListing { dir: playground/client-reload } |  |  | 0.503 |
| walker |  | 3256 | 18 | Fs::DirListing { dir: playground/extensions } |  |  | 0.503 |
| walker |  | 3301 | 45 | Fs::DirListing { dir: playground/transform-plugin } |  |  | 0.503 |
| ns | 3307 |  | 448 | index.ts type re-exports: server internals, HMR payloads, vendored types | 2.7 | 2.6 | 0.468 |
| walker |  | 3321 | 20 | Fs::DirListing { dir: playground/preserve-symlinks } |  |  | 0.468 |
| walker |  | 3325 | 4 | Fs::DirListing { dir: playground/preserve-symlinks/src } |  |  | 0.468 |
| walker |  | 3373 | 48 | Fs::DirListing { dir: playground/multiple-entrypoints } |  |  | 0.468 |
| walker |  | 3394 | 21 | Fs::DirListing { dir: playground/base-conflict } |  |  | 0.468 |
| walker |  | 3415 | 21 | Fs::DirListing { dir: playground/dynamic-import-inline } |  |  | 0.468 |
| walker |  | 3423 | 8 | Fs::DirListing { dir: playground/dynamic-import-inline/src } |  |  | 0.468 |
| walker |  | 3444 | 21 | Fs::DirListing { dir: playground/ssr-alias } |  |  | 0.468 |
| walker |  | 3452 | 8 | Fs::DirListing { dir: playground/ssr-alias/alias-original } |  |  | 0.468 |
| walker |  | 3473 | 21 | Fs::DirListing { dir: playground/ssr-pug } |  |  | 0.468 |
| walker |  | 3477 | 4 | Fs::DirListing { dir: playground/ssr-pug/src } |  |  | 0.468 |
| walker |  | 3498 | 21 | Fs::DirListing { dir: playground/ssr-wasm } |  |  | 0.468 |
| walker |  | 3509 | 11 | Fs::DirListing { dir: playground/preserve-symlinks/module-a } |  |  | 0.468 |
| walker |  | 3517 | 8 | Fs::DirListing { dir: playground/preserve-symlinks/module-a/src } |  |  | 0.468 |
| walker |  | 3539 | 22 | Fs::DirListing { dir: playground/build-old } |  |  | 0.468 |
| walker |  | 3561 | 22 | Fs::DirListing { dir: playground/object-hooks } |  |  | 0.468 |
| ns | 3569 |  | 262 | Plugin interface: every Vite-specific hook and flag (names only) | 2.8 |  | 0.449 |
| walker |  | 3583 | 22 | Fs::DirListing { dir: playground/proxy-hmr } |  |  | 0.449 |
| walker |  | 3607 | 24 | Fs::DirListing { dir: playground/backend-integration } |  |  | 0.449 |
| walker |  | 3617 | 10 | Fs::DirListing { dir: playground/backend-integration/frontend } |  |  | 0.449 |
| walker |  | 3621 | 4 | Fs::DirListing { dir: playground/backend-integration/frontend/images } |  |  | 0.449 |
| walker |  | 3645 | 24 | Fs::DirListing { dir: playground/forward-console } |  |  | 0.449 |
| walker |  | 3649 | 4 | Fs::DirListing { dir: playground/forward-console/src } |  |  | 0.449 |
| walker |  | 3673 | 24 | Fs::DirListing { dir: playground/optimize-deps-no-discovery } |  |  | 0.449 |
| walker |  | 3681 | 8 | Fs::DirListing { dir: playground/optimize-deps-no-discovery/dep-no-discovery } |  |  | 0.449 |
| walker |  | 3706 | 25 | Fs::DirListing { dir: playground/import-assertion } |  |  | 0.449 |
| walker |  | 3718 | 12 | Fs::DirListing { dir: playground/import-assertion/import-assertion-dep } |  |  | 0.449 |
| walker |  | 3743 | 25 | Fs::DirListing { dir: playground/ssr } |  |  | 0.449 |
| walker |  | 3768 | 25 | Fs::DirListing { dir: playground/tsconfig-json-load-error } |  |  | 0.449 |
| walker |  | 3772 | 4 | Fs::DirListing { dir: playground/tsconfig-json-load-error/src } |  |  | 0.449 |
| walker |  | 3781 | 9 | Fs::DirListing { dir: playground/tsconfig-json-load-error/has-error } |  |  | 0.449 |
| walker |  | 3794 | 13 | Fs::DirListing { dir: playground/backend-integration/dir } |  |  | 0.449 |
| walker |  | 3807 | 13 | Fs::DirListing { dir: playground/proxy-hmr/other-app } |  |  | 0.449 |
| walker |  | 3833 | 26 | Fs::DirListing { dir: playground/env-nested } |  |  | 0.449 |
| walker |  | 3844 | 11 | Fs::DirListing { dir: playground/env-nested/envs } |  |  | 0.449 |
| walker |  | 3870 | 26 | Fs::DirListing { dir: playground/environment-react-ssr } |  |  | 0.449 |
| walker |  | 3885 | 15 | Fs::DirListing { dir: playground/base-conflict/src } |  |  | 0.449 |
| walker |  | 3900 | 15 | Fs::DirListing { dir: playground/ssr-alias/src } |  |  | 0.449 |
| walker |  | 3929 | 29 | Fs::DirListing { dir: playground/minify } |  |  | 0.449 |
| ns | 3931 |  | 362 | ViteDevServer: every member (names only) | 2.9 |  | 0.423 |
| walker |  | 3942 | 13 | Fs::DirListing { dir: playground/minify/dir/module } |  |  | 0.423 |
| walker |  | 3971 | 29 | Fs::DirListing { dir: playground/resolve-tsconfig-paths } |  |  | 0.423 |
| walker |  | 3975 | 4 | Fs::DirListing { dir: playground/resolve-tsconfig-paths/fallback } |  |  | 0.423 |
| walker |  | 4004 | 29 | Fs::DirListing { dir: playground/tailwind-sourcemap } |  |  | 0.423 |
| walker |  | 4034 | 30 | Fs::DirListing { dir: playground/alias } |  |  | 0.423 |
| walker |  | 4064 | 30 | Fs::DirListing { dir: playground/tsconfig-json } |  |  | 0.423 |
| walker |  | 4078 | 14 | Fs::DirListing { dir: playground/tsconfig-json/src } |  |  | 0.423 |
| walker |  | 4093 | 15 | Fs::DirListing { dir: playground/tsconfig-json/nested } |  |  | 0.423 |
| walker |  | 4108 | 15 | Fs::DirListing { dir: playground/tsconfig-json/nested-with-extends } |  |  | 0.423 |
| ns | 4111 |  | 180 | UserConfig keys: project, sources and transform options | 3.1 |  | 0.414 |
| walker |  | 4139 | 31 | Fs::DirListing { dir: playground/module-graph } |  |  | 0.414 |
| walker |  | 4170 | 31 | Fs::DirListing { dir: playground/ssr-webworker } |  |  | 0.414 |
| walker |  | 4179 | 9 | Fs::DirListing { dir: playground/ssr-webworker/src } |  |  | 0.414 |
| walker |  | 4191 | 12 | Fs::DirListing { dir: playground/ssr-webworker/browser-exports } |  |  | 0.414 |
| walker |  | 4207 | 16 | Fs::DirListing { dir: playground/ssr-webworker/worker-exports } |  |  | 0.414 |
| walker |  | 4239 | 32 | Fs::DirListing { dir: playground/optimize-missing-deps } |  |  | 0.414 |
| walker |  | 4247 | 8 | Fs::DirListing { dir: playground/optimize-missing-deps/missing-dep } |  |  | 0.414 |
| walker |  | 4260 | 13 | Fs::DirListing { dir: playground/optimize-missing-deps/multi-entry-dep } |  |  | 0.414 |
| walker |  | 4277 | 17 | Fs::DirListing { dir: playground/environment-react-ssr/src } |  |  | 0.414 |
| walker |  | 4310 | 33 | Fs::DirListing { dir: playground/define } |  |  | 0.414 |
| walker |  | 4318 | 8 | Fs::DirListing { dir: playground/define/commonjs-dep } |  |  | 0.414 |
| walker |  | 4351 | 33 | Fs::DirListing { dir: playground/ssr-conditions } |  |  | 0.414 |
| walker |  | 4355 | 4 | Fs::DirListing { dir: playground/ssr-conditions/src } |  |  | 0.414 |
| walker |  | 4389 | 34 | Fs::DirListing { dir: playground/css-codesplit-cjs } |  |  | 0.414 |
| ns | 4401 |  | 290 | UserConfig keys: server, build, env, worker and the rest | 3.2 | 3.1 | 0.398 |
| walker |  | 4423 | 34 | Fs::DirListing { dir: playground/tailwind } |  |  | 0.398 |
| walker |  | 4427 | 4 | Fs::DirListing { dir: playground/tailwind/public } |  |  | 0.398 |
| walker |  | 4437 | 10 | Fs::DirListing { dir: playground/tailwind/src } |  |  | 0.398 |
| walker |  | 4442 | 5 | Fs::DirListing { dir: playground/tailwind/src/components } |  |  | 0.398 |
| walker |  | 4447 | 5 | Fs::DirListing { dir: playground/tailwind/src/views } |  |  | 0.398 |
| walker |  | 4465 | 18 | Fs::DirListing { dir: playground/css-lightningcss-root/root } |  |  | 0.398 |
| walker |  | 4521 | 56 | Json::Identity { file: playground/package.json } |  |  | 0.398 |
| walker |  | 4556 | 35 | Fs::DirListing { dir: playground/fs-serve } |  |  | 0.398 |
| walker |  | 4560 | 4 | Fs::DirListing { dir: playground/fs-serve/nested } |  |  | 0.398 |
| ns | 4571 |  | 170 | Per-environment options (SharedEnvironmentOptions / EnvironmentOptions) | 3.3 |  | 0.390 |
| walker |  | 4596 | 36 | Fs::DirListing { dir: playground/external } |  |  | 0.390 |
| walker |  | 4604 | 8 | Fs::DirListing { dir: playground/external/dep-that-imports } |  |  | 0.390 |
| walker |  | 4612 | 8 | Fs::DirListing { dir: playground/external/dep-that-requires } |  |  | 0.390 |
| walker |  | 4622 | 10 | Fs::DirListing { dir: playground/external/public } |  |  | 0.390 |
| walker |  | 4632 | 10 | Fs::DirListing { dir: playground/external/src } |  |  | 0.390 |
| walker |  | 4637 | 5 | Fs::DirListing { dir: playground/alias/__tests__ } |  |  | 0.390 |
| walker |  | 4642 | 5 | Fs::DirListing { dir: playground/define/__tests__ } |  |  | 0.390 |
| walker |  | 4647 | 5 | Fs::DirListing { dir: playground/env/__tests__ } |  |  | 0.390 |
| walker |  | 4652 | 5 | Fs::DirListing { dir: playground/extensions/__tests__ } |  |  | 0.390 |
| walker |  | 4657 | 5 | Fs::DirListing { dir: playground/external/__tests__ } |  |  | 0.390 |
| walker |  | 4662 | 5 | Fs::DirListing { dir: playground/resolve-tsconfig-paths/__tests__ } |  |  | 0.390 |
| walker |  | 4681 | 19 | Fs::DirListing { dir: playground/resolve-tsconfig-paths/src } |  |  | 0.390 |
| ns | 4707 |  | 136 | CommonServerOptions: the host/port/https/proxy/cors keys | 3.4 |  | 0.384 |
| walker |  | 4722 | 41 | Fs::DirListing { dir: playground/resolve-tsconfig-paths/src/nested } |  |  | 0.384 |
| walker |  | 4759 | 37 | Fs::DirListing { dir: playground/ssr-noexternal } |  |  | 0.384 |
| walker |  | 4764 | 5 | Fs::DirListing { dir: playground/ssr-noexternal/src } |  |  | 0.384 |
| walker |  | 4772 | 8 | Fs::DirListing { dir: playground/ssr-noexternal/require-external-cjs } |  |  | 0.384 |
| walker |  | 4786 | 14 | Fs::DirListing { dir: playground/ssr-noexternal/external-cjs } |  |  | 0.384 |
| walker |  | 4823 | 37 | Fs::DirListing { dir: playground/tailwind-v3 } |  |  | 0.384 |
| walker |  | 4833 | 10 | Fs::DirListing { dir: playground/tailwind-v3/src } |  |  | 0.384 |
| walker |  | 4838 | 5 | Fs::DirListing { dir: playground/tailwind-v3/src/components } |  |  | 0.384 |
| walker |  | 4843 | 5 | Fs::DirListing { dir: playground/tailwind-v3/src/views } |  |  | 0.384 |
| walker |  | 4882 | 39 | Fs::DirListing { dir: playground/ssr-resolve } |  |  | 0.384 |
| walker |  | 4890 | 8 | Fs::DirListing { dir: playground/ssr-resolve/pkg-module-sync } |  |  | 0.384 |
| walker |  | 4902 | 12 | Fs::DirListing { dir: playground/ssr-resolve/pkg-exports } |  |  | 0.384 |
| walker |  | 4919 | 17 | Fs::DirListing { dir: playground/ssr-resolve/deep-import } |  |  | 0.384 |
| walker |  | 4927 | 8 | Fs::DirListing { dir: playground/ssr-resolve/deep-import/foo } |  |  | 0.384 |
| walker |  | 4931 | 4 | Fs::DirListing { dir: playground/ssr-resolve/deep-import/bar } |  |  | 0.384 |
| walker |  | 4935 | 4 | Fs::DirListing { dir: playground/ssr-resolve/deep-import/utils } |  |  | 0.384 |
| ns | 4944 |  | 237 | ServerOptions and FileSystemServeOptions keys | 3.5 |  | 0.375 |
| walker |  | 4946 | 11 | Fs::DirListing { dir: playground/ssr-resolve/entries } |  |  | 0.375 |
| walker |  | 4950 | 4 | Fs::DirListing { dir: playground/ssr-resolve/entries/dir } |  |  | 0.375 |
| walker |  | 4990 | 40 | Fs::DirListing { dir: playground/json } |  |  | 0.375 |
| walker |  | 5002 | 12 | Fs::DirListing { dir: playground/json/dep-json-require } |  |  | 0.375 |
| walker |  | 5006 | 4 | Fs::DirListing { dir: playground/json/public } |  |  | 0.375 |
| walker |  | 5012 | 6 | Fs::DirListing { dir: playground/json/json-bom } |  |  | 0.375 |
| walker |  | 5020 | 8 | Fs::DirListing { dir: playground/json/json-module } |  |  | 0.375 |
| walker |  | 5061 | 41 | Fs::DirListing { dir: playground/dynamic-import } |  |  | 0.375 |
| walker |  | 5073 | 12 | Fs::DirListing { dir: playground/dynamic-import/pkg } |  |  | 0.375 |
| walker |  | 5077 | 4 | Fs::DirListing { dir: playground/dynamic-import/css } |  |  | 0.375 |
| walker |  | 5084 | 7 | Fs::DirListing { dir: playground/dynamic-import/(app) } |  |  | 0.375 |
| walker |  | 5088 | 4 | Fs::DirListing { dir: playground/dynamic-import/(app)/nest } |  |  | 0.375 |
| walker |  | 5120 | 32 | Fs::DirListing { dir: playground/dynamic-import/nested } |  |  | 0.375 |
| walker |  | 5124 | 4 | Fs::DirListing { dir: playground/dynamic-import/nested/nested } |  |  | 0.375 |
| walker |  | 5134 | 10 | Fs::DirListing { dir: playground/dynamic-import/files } |  |  | 0.375 |
| walker |  | 5150 | 16 | Fs::DirListing { dir: playground/dynamic-import/alias } |  |  | 0.375 |
| walker |  | 5167 | 17 | Fs::DirListing { dir: playground/dynamic-import/views } |  |  | 0.375 |
| walker |  | 5177 | 10 | Fs::DirListing { dir: playground/dynamic-import/nested/treeshaken } |  |  | 0.375 |
| ns | 5180 |  | 236 | BuildEnvironmentOptions keys: output, assets, CSS, minification | 3.6 |  | 0.367 |
| walker |  | 5220 | 43 | Fs::DirListing { dir: playground/ssr-html } |  |  | 0.367 |
| walker |  | 5230 | 10 | Fs::DirListing { dir: playground/ssr-html/public } |  |  | 0.367 |
| walker |  | 5273 | 43 | Fs::DirListing { dir: playground/wasm } |  |  | 0.367 |
| walker |  | 5278 | 5 | Fs::DirListing { dir: playground/wasm/__tests__ } |  |  | 0.367 |
| walker |  | 5284 | 6 | Fs::DirListing { dir: playground/build-old/__tests__ } |  |  | 0.367 |
| walker |  | 5290 | 6 | Fs::DirListing { dir: playground/csp/__tests__ } |  |  | 0.367 |
| walker |  | 5296 | 6 | Fs::DirListing { dir: playground/dynamic-import/__tests__ } |  |  | 0.367 |
| walker |  | 5302 | 6 | Fs::DirListing { dir: playground/forward-console/__test__ } |  |  | 0.367 |
| walker |  | 5308 | 6 | Fs::DirListing { dir: playground/minify/__tests__ } |  |  | 0.367 |
| walker |  | 5314 | 6 | Fs::DirListing { dir: playground/object-hooks/__tests__ } |  |  | 0.367 |
| walker |  | 5320 | 6 | Fs::DirListing { dir: playground/tailwind/__test__ } |  |  | 0.367 |
| walker |  | 5343 | 23 | Fs::DirListing { dir: playground/alias/dir } |  |  | 0.367 |
| walker |  | 5351 | 8 | Fs::DirListing { dir: playground/alias/dir/module } |  |  | 0.367 |
| walker |  | 5396 | 45 | Fs::DirListing { dir: playground/hmr-full-bundle-mode } |  |  | 0.367 |
| walker |  | 5420 | 24 | Fs::DirListing { dir: playground/ssr-wasm/src } |  |  | 0.367 |
| ns | 5437 |  | 257 | BuildEnvironmentOptions keys: bundler passthrough, lib, ssr, reporting | 3.7 | 3.6 | 0.359 |
| ns | 5530 |  | 93 | ExperimentalOptions and FutureOptions | 3.8 |  | 0.355 |
| walker |  | 5532 | 112 | Fs::DirListing { dir: playground/css-sourcemap } |  |  | 0.355 |
| walker |  | 5539 | 7 | Fs::DirListing { dir: playground/assets-sanitize/__tests__ } |  |  | 0.355 |
| walker |  | 5546 | 7 | Fs::DirListing { dir: playground/backend-integration/__tests__ } |  |  | 0.355 |
| walker |  | 5553 | 7 | Fs::DirListing { dir: playground/base-conflict/__tests__ } |  |  | 0.355 |
| walker |  | 5560 | 7 | Fs::DirListing { dir: playground/dynamic-import-inline/__tests__ } |  |  | 0.355 |
| walker |  | 5567 | 7 | Fs::DirListing { dir: playground/env-nested/__tests__ } |  |  | 0.355 |
| walker |  | 5574 | 7 | Fs::DirListing { dir: playground/hmr-root/__tests__ } |  |  | 0.355 |
| walker |  | 5581 | 7 | Fs::DirListing { dir: playground/module-graph/__tests__ } |  |  | 0.355 |
| walker |  | 5588 | 7 | Fs::DirListing { dir: playground/multiple-entrypoints/__tests__ } |  |  | 0.355 |
| walker |  | 5595 | 7 | Fs::DirListing { dir: playground/tsconfig-json/__tests__ } |  |  | 0.355 |
| walker |  | 5646 | 51 | Fs::DirListing { dir: playground/preload } |  |  | 0.355 |
| walker |  | 5654 | 8 | Fs::DirListing { dir: playground/preload/dep-a } |  |  | 0.355 |
| walker |  | 5662 | 8 | Fs::DirListing { dir: playground/preload/dep-including-a } |  |  | 0.355 |
| walker |  | 5667 | 5 | Fs::DirListing { dir: playground/preload/public } |  |  | 0.355 |
| walker |  | 5688 | 21 | Fs::DirListing { dir: playground/preload/src } |  |  | 0.355 |
| walker |  | 5731 | 43 | Json::Identity { file: playground/css-lightningcss-root/package.json } |  |  | 0.355 |
| walker |  | 5758 | 27 | Fs::DirListing { dir: playground/ssr-conditions/external } |  |  | 0.355 |
| walker |  | 5785 | 27 | Fs::DirListing { dir: playground/ssr-conditions/no-external } |  |  | 0.355 |
| ns | 5804 |  | 274 | CLI global options (all five commands) | 3.9 | 2.2 | 0.349 |
| walker |  | 5838 | 53 | Fs::DirListing { dir: playground/nested-deps } |  |  | 0.349 |
| walker |  | 5846 | 8 | Fs::DirListing { dir: playground/nested-deps/test-package-a } |  |  | 0.349 |
| walker |  | 5854 | 8 | Fs::DirListing { dir: playground/nested-deps/test-package-f } |  |  | 0.349 |
| walker |  | 5865 | 11 | Fs::DirListing { dir: playground/nested-deps/self-referencing } |  |  | 0.349 |
| walker |  | 5877 | 12 | Fs::DirListing { dir: playground/nested-deps/test-package-b } |  |  | 0.349 |
| walker |  | 5892 | 15 | Fs::DirListing { dir: playground/nested-deps/test-package-d } |  |  | 0.349 |
| walker |  | 5900 | 8 | Fs::DirListing { dir: playground/nested-deps/test-package-d/test-package-d-nested } |  |  | 0.349 |
| walker |  | 5917 | 17 | Fs::DirListing { dir: playground/nested-deps/test-package-c } |  |  | 0.349 |
| walker |  | 5939 | 22 | Fs::DirListing { dir: playground/nested-deps/test-package-e } |  |  | 0.349 |
| walker |  | 5947 | 8 | Fs::DirListing { dir: playground/nested-deps/test-package-e/test-package-e-excluded } |  |  | 0.349 |
| walker |  | 5955 | 8 | Fs::DirListing { dir: playground/nested-deps/test-package-e/test-package-e-included } |  |  | 0.349 |
| walker |  | 5959 | 4 | Fs::DirListing { dir: playground/nested-deps/self-referencing/test } |  |  | 0.349 |
| walker |  | 5966 | 7 | Fs::DirListing { dir: playground/nested-deps/__tests__ } |  |  | 0.349 |
| ns | 5995 |  | 191 | CLI dev-server flags | 3.10 | 2.2 | 0.343 |
| walker |  | 6010 | 44 | Json::Identity { file: playground/css-lightningcss-proxy/package.json } |  |  | 0.343 |
| walker |  | 6027 | 17 | Fs::DirListing { dir: playground/backend-integration/frontend/styles } |  |  | 0.343 |
| walker |  | 6057 | 30 | Fs::DirListing { dir: playground/ssr/src } |  |  | 0.343 |
| walker |  | 6069 | 12 | Fs::DirListing { dir: playground/ssr/src/circular-import } |  |  | 0.343 |
| walker |  | 6081 | 12 | Fs::DirListing { dir: playground/ssr/src/circular-import2 } |  |  | 0.343 |
| walker |  | 6089 | 8 | Fs::DirListing { dir: playground/import-assertion/__tests__ } |  |  | 0.343 |
| walker |  | 6097 | 8 | Fs::DirListing { dir: playground/json/__tests__/csr } |  |  | 0.343 |
| walker |  | 6105 | 8 | Fs::DirListing { dir: playground/ssr-alias/__tests__ } |  |  | 0.343 |
| walker |  | 6113 | 8 | Fs::DirListing { dir: playground/ssr-resolve/__tests__ } |  |  | 0.343 |
| walker |  | 6121 | 8 | Fs::DirListing { dir: playground/tailwind-v3/__test__ } |  |  | 0.343 |
| walker |  | 6151 | 30 | Json::Identity { file: playground/nested-deps/self-referencing/package.json } |  |  | 0.343 |
| walker |  | 6183 | 32 | Fs::DirListing { dir: playground/ssr-html/src } |  |  | 0.343 |
| walker |  | 6245 | 62 | Fs::DirListing { dir: playground/data-uri } |  |  | 0.343 |
| walker |  | 6251 | 6 | Fs::DirListing { dir: playground/data-uri/__tests__ } |  |  | 0.343 |
| walker |  | 6305 | 54 | Json::Identity { file: playground/env/package.json } |  |  | 0.343 |
| walker |  | 6359 | 54 | Json::Identity { file: playground/json/package.json } |  |  | 0.343 |
| walker |  | 6368 | 9 | Fs::DirListing { dir: playground/cli/__tests__ } |  |  | 0.343 |
| walker |  | 6377 | 9 | Fs::DirListing { dir: playground/css-lightningcss-root/__tests__ } |  |  | 0.343 |
| walker |  | 6386 | 9 | Fs::DirListing { dir: playground/css-no-codesplit/__tests__ } |  |  | 0.343 |
| walker |  | 6395 | 9 | Fs::DirListing { dir: playground/environment-react-ssr/__tests__ } |  |  | 0.343 |
| walker |  | 6404 | 9 | Fs::DirListing { dir: playground/preserve-symlinks/__tests__ } |  |  | 0.343 |
| walker |  | 6413 | 9 | Fs::DirListing { dir: playground/tailwind-sourcemap/__tests__ } |  |  | 0.343 |
| ns | 6467 |  | 472 | CLI build flags | 3.11 | 2.2 | 0.330 |
| walker |  | 6468 | 55 | Json::Identity { file: playground/alias/package.json } |  |  | 0.330 |
| walker |  | 6523 | 55 | Json::Identity { file: playground/build-old/package.json } |  |  | 0.330 |
| walker |  | 6578 | 55 | Json::Identity { file: playground/cli/package.json } |  |  | 0.330 |
| ns | 6622 |  | 155 | CLI optimize and preview flags | 3.12 | 2.2 | 0.327 |
| walker |  | 6633 | 55 | Json::Identity { file: playground/cli-module/package.json } |  |  | 0.327 |
| walker |  | 6688 | 55 | Json::Identity { file: playground/csp/package.json } |  |  | 0.327 |
| walker |  | 6743 | 55 | Json::Identity { file: playground/data-uri/package.json } |  |  | 0.327 |
| ns | 6757 |  | 135 | src/node module roster (complete) | 4.1 |  | 0.378 |
| walker |  | 6798 | 55 | Json::Identity { file: playground/define/package.json } |  |  | 0.378 |
| walker |  | 6853 | 55 | Json::Identity { file: playground/devtools/package.json } |  |  | 0.378 |
| ns | 6906 |  | 149 | src/node/plugins roster (complete) | 4.2 |  | 0.415 |
| walker |  | 6908 | 55 | Json::Identity { file: playground/extensions/package.json } |  |  | 0.415 |
| walker |  | 6963 | 55 | Json::Identity { file: playground/external/package.json } |  |  | 0.415 |
| ns | 6981 |  | 75 | src/node/server roster (complete) | 4.3 |  | 0.433 |
| walker |  | 7018 | 55 | Json::Identity { file: playground/forward-console/package.json } |  |  | 0.433 |
| ns | 7066 |  | 85 | Dev-server middlewares and per-environment implementations | 4.4 |  | 0.427 |
| walker |  | 7073 | 55 | Json::Identity { file: playground/minify/package.json } |  |  | 0.427 |
| walker |  | 7128 | 55 | Json::Identity { file: playground/module-graph/package.json } |  |  | 0.427 |
| ns | 7144 |  | 78 | SSR and dependency-optimizer rosters | 4.5 |  | 0.441 |
| walker |  | 7183 | 55 | Json::Identity { file: playground/object-hooks/package.json } |  |  | 0.441 |
| walker |  | 7238 | 55 | Json::Identity { file: playground/preload/package.json } |  |  | 0.441 |
| ns | 7287 |  | 143 | Browser client, module-runner and shared rosters | 4.6 |  | 0.445 |
| walker |  | 7293 | 55 | Json::Identity { file: playground/tailwind/package.json } |  |  | 0.445 |
| walker |  | 7348 | 55 | Json::Identity { file: playground/transform-plugin/package.json } |  |  | 0.445 |
| ns | 7400 |  | 113 | resolvePlugins: the built-in plugin pipeline order | 4.7 |  | 0.440 |
| walker |  | 7403 | 55 | Json::Identity { file: playground/wasm/package.json } |  |  | 0.440 |
| walker |  | 7459 | 56 | Json::Identity { file: playground/assets-sanitize/package.json } |  |  | 0.440 |
| ns | 7494 |  | 94 | Published and inlined type declarations | 4.8 |  | 0.454 |
| walker |  | 7515 | 56 | Json::Identity { file: playground/base-conflict/package.json } |  |  | 0.454 |
| walker |  | 7571 | 56 | Json::Identity { file: playground/client-reload/package.json } |  |  | 0.454 |
| ns | 7609 |  | 115 | Core internal entry-point signatures | 4.9 |  | 0.450 |
| walker |  | 7627 | 56 | Json::Identity { file: playground/dynamic-import/package.json } |  |  | 0.450 |
| walker |  | 7683 | 56 | Json::Identity { file: playground/env-nested/package.json } |  |  | 0.450 |
| walker |  | 7739 | 56 | Json::Identity { file: playground/resolve-linked/package.json } |  |  | 0.450 |
| walker |  | 7795 | 56 | Json::Identity { file: playground/ssr/package.json } |  |  | 0.450 |
| walker |  | 7806 | 11 | Json::Dependencies { file: playground/ssr/package.json } |  |  | 0.450 |
| walker |  | 7862 | 56 | Json::Identity { file: playground/tsconfig-json/package.json } |  |  | 0.450 |
| walker |  | 7895 | 33 | Json::Identity { file: playground/json/dep-json-require/package.json } |  |  | 0.450 |
| ns | 7919 |  | 310 | CONTRIBUTING.md: every section heading | 5.1 |  | 0.440 |
| walker |  | 7952 | 57 | Json::Identity { file: playground/backend-integration/package.json } |  |  | 0.440 |
| walker |  | 8009 | 57 | Json::Identity { file: playground/css-dynamic-import/package.json } |  |  | 0.440 |
| walker |  | 8066 | 57 | Json::Identity { file: playground/css-sourcemap/package.json } |  |  | 0.440 |
| ns | 8109 |  | 190 | CONTRIBUTING: local development loop | 5.2 | 5.1 | 0.438 |
| walker |  | 8123 | 57 | Json::Identity { file: playground/dynamic-import-inline/package.json } |  |  | 0.438 |
| walker |  | 8180 | 57 | Json::Identity { file: playground/fs-serve/package.json } |  |  | 0.438 |
| walker |  | 8237 | 57 | Json::Identity { file: playground/import-assertion/package.json } |  |  | 0.438 |
| walker |  | 8294 | 57 | Json::Identity { file: playground/multiple-entrypoints/package.json } |  |  | 0.438 |
| walker |  | 8351 | 57 | Json::Identity { file: playground/nested-deps/package.json } |  |  | 0.438 |
| walker |  | 8408 | 57 | Json::Identity { file: playground/proxy-hmr/package.json } |  |  | 0.438 |
| walker |  | 8465 | 57 | Json::Identity { file: playground/ssr-alias/package.json } |  |  | 0.438 |
| ns | 8515 |  | 406 | playground/ roster (complete e2e corpus) | 5.3 |  | 0.492 |
| walker |  | 8522 | 57 | Json::Identity { file: playground/ssr-html/package.json } |  |  | 0.492 |
| walker |  | 8533 | 11 | Json::Dependencies { file: playground/ssr-html/package.json } |  |  | 0.492 |
| walker |  | 8590 | 57 | Json::Identity { file: playground/tailwind-v3/package.json } |  |  | 0.492 |
| walker |  | 8611 | 21 | Fs::DirListing { dir: playground/ssr/src/circular-dep-init } |  |  | 0.492 |
| walker |  | 8669 | 58 | Json::Identity { file: playground/css-no-codesplit/package.json } |  |  | 0.492 |
| walker |  | 8727 | 58 | Json::Identity { file: playground/ssr-conditions/package.json } |  |  | 0.492 |
| walker |  | 8785 | 58 | Json::Identity { file: playground/ssr-noexternal/package.json } |  |  | 0.492 |
| ns | 8802 |  | 287 | CONTRIBUTING: how the integration tests work | 5.4 | 5.3 | 0.490 |
| walker |  | 8843 | 58 | Json::Identity { file: playground/ssr-pug/package.json } |  |  | 0.490 |
| walker |  | 8901 | 58 | Json::Identity { file: playground/ssr-wasm/package.json } |  |  | 0.490 |
| walker |  | 8912 | 11 | Json::Dependencies { file: playground/ssr-wasm/package.json } |  |  | 0.490 |
| ns | 8919 |  | 117 | Unit test locations under packages/vite/src | 5.5 |  | 0.482 |
| walker |  | 8970 | 58 | Json::Identity { file: playground/ssr-webworker/package.json } |  |  | 0.482 |
| walker |  | 9028 | 58 | Json::Identity { file: playground/tailwind-sourcemap/package.json } |  |  | 0.482 |
| ns | 9050 |  | 131 | How the unit and e2e vitest runs are separated | 5.6 |  | 0.480 |
| walker |  | 9086 | 58 | Json::Identity { file: playground/tsconfig-json-load-error/package.json } |  |  | 0.480 |
| ns | 9114 |  | 64 | docs/ site roster | 5.7 |  | 0.489 |
| walker |  | 9120 | 34 | Json::Identity { file: playground/json/json-module/package.json } |  |  | 0.489 |
| walker |  | 9179 | 59 | Json::Identity { file: playground/environment-react-ssr/package.json } |  |  | 0.489 |
| walker |  | 9238 | 59 | Json::Identity { file: playground/hmr-full-bundle-mode/package.json } |  |  | 0.489 |
| walker |  | 9297 | 59 | Json::Identity { file: playground/optimize-missing-deps/package.json } |  |  | 0.489 |
| ns | 9298 |  | 184 | docs/guide and docs/config page rosters | 5.8 |  | 0.479 |
| ns | 9343 |  | 45 | docs/changes: the breaking-change / migration notes | 5.9 |  | 0.477 |
| walker |  | 9356 | 59 | Json::Identity { file: playground/preserve-symlinks/package.json } |  |  | 0.477 |
| walker |  | 9415 | 59 | Json::Identity { file: playground/resolve-tsconfig-paths/package.json } |  |  | 0.477 |
| walker |  | 9474 | 59 | Json::Identity { file: playground/ssr-resolve/package.json } |  |  | 0.477 |
| ns | 9501 |  | 158 | CONTRIBUTING: the dependency policy | 5.10 | 5.1 | 0.475 |
| walker |  | 9534 | 60 | Json::Identity { file: playground/css-codesplit-cjs/package.json } |  |  | 0.475 |
| walker |  | 9594 | 60 | Json::Identity { file: playground/optimize-deps-no-discovery/package.json } |  |  | 0.475 |
| walker |  | 9604 | 10 | Fs::DirListing { dir: playground/cli-module/__tests__ } |  |  | 0.475 |
| walker |  | 9614 | 10 | Fs::DirListing { dir: playground/css-codesplit-cjs/__tests__ } |  |  | 0.475 |
| walker |  | 9624 | 10 | Fs::DirListing { dir: playground/hmr-full-bundle-mode/__tests__ } |  |  | 0.487 |
| ns | 9624 |  | 123 | create-vite: package layout and template roster | 5.11 |  | 0.487 |
| walker |  | 9634 | 10 | Fs::DirListing { dir: playground/optimize-deps-no-discovery/__tests__ } |  |  | 0.487 |
| walker |  | 9644 | 10 | Fs::DirListing { dir: playground/ssr/__tests__ } |  |  | 0.487 |
| walker |  | 9657 | 13 | Json::Entry { file: playground/resolve-linked/package.json } |  |  | 0.487 |
| walker |  | 9668 | 11 | Fs::DirListing { dir: playground/client-reload/__tests__ } |  |  | 0.487 |
| ns | 9677 |  | 53 | plugin-legacy: package layout | 5.12 |  | 0.493 |
| walker |  | 9679 | 11 | Fs::DirListing { dir: playground/proxy-hmr/__tests__ } |  |  | 0.493 |
| walker |  | 9690 | 11 | Fs::DirListing { dir: playground/ssr-html/__tests__ } |  |  | 0.493 |
| walker |  | 9718 | 28 | Json::Scripts { file: playground/package.json } |  |  | 0.493 |
| walker |  | 9730 | 12 | Fs::DirListing { dir: playground/css-dynamic-import/__tests__ } |  |  | 0.493 |
| walker |  | 9742 | 12 | Fs::DirListing { dir: playground/forward-console/fixtures/throw-dep } |  |  | 0.493 |
| walker |  | 9754 | 12 | Fs::DirListing { dir: playground/ssr-conditions/__tests__ } |  |  | 0.493 |
| walker |  | 9766 | 12 | Fs::DirListing { dir: playground/ssr-noexternal/__tests__ } |  |  | 0.493 |
| walker |  | 9778 | 12 | Fs::DirListing { dir: playground/ssr-pug/__tests__ } |  |  | 0.493 |
| walker |  | 9790 | 12 | Fs::DirListing { dir: playground/ssr-wasm/__tests__ } |  |  | 0.493 |
| walker |  | 9802 | 12 | Fs::DirListing { dir: playground/ssr-webworker/__tests__ } |  |  | 0.493 |
| ns | 9827 |  | 150 | Repository automation: .github and release scripts | 5.13 |  | 0.505 |
| walker |  | 9889 | 87 | Fs::DirListing { dir: playground/glob-import } |  |  | 0.505 |
| walker |  | 9893 | 4 | Fs::DirListing { dir: playground/glob-import/subpath-imports-sub } |  |  | 0.505 |
| walker |  | 9901 | 8 | Fs::DirListing { dir: playground/glob-import/import-meta-glob-pkg } |  |  | 0.505 |
| walker |  | 9905 | 4 | Fs::DirListing { dir: playground/glob-import/pkg-pages } |  |  | 0.505 |
| walker |  | 9911 | 6 | Fs::DirListing { dir: playground/glob-import/follow-symlinks } |  |  | 0.505 |
| walker |  | 9943 | 32 | Fs::DirListing { dir: playground/glob-import/dir } |  |  | 0.505 |
| walker |  | 9947 | 4 | Fs::DirListing { dir: playground/glob-import/dir/nested } |  |  | 0.505 |
| walker |  | 9955 | 8 | Fs::DirListing { dir: playground/glob-import/array-common-base } |  |  | 0.505 |
| walker |  | 9959 | 4 | Fs::DirListing { dir: playground/glob-import/array-common-base/pattern1 } |  |  | 0.505 |
| walker |  | 9963 | 4 | Fs::DirListing { dir: playground/glob-import/array-common-base/pattern2 } |  |  | 0.505 |
| walker |  | 9971 | 8 | Fs::DirListing { dir: playground/glob-import/array-test-dir } |  |  | 0.505 |
| walker |  | 9979 | 8 | Fs::DirListing { dir: playground/glob-import/imports-path } |  |  | 0.505 |
| walker |  | 9990 | 11 | Fs::DirListing { dir: playground/glob-import/side-effect } |  |  | 0.505 |
| walker |  | 9997 | 7 | Fs::DirListing { dir: playground/glob-import/follow-symlinks/linked } |  |  | 0.505 |
