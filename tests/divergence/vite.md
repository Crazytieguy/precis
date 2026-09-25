Score(3000)=0.493 I=0.856 C=0.284 ns_rows≤3K=18/55 grid(1000/1442/2080/3000/4327/6240/9000)=0.476/0.772/0.639/0.493/0.405/0.336/0.478

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
| walker |  | 355 | 123 | Fs::DirListing { dir: packages/create-vite } |  |  | 0.405 |
| walker |  | 359 | 4 | Fs::DirListing { dir: packages/create-vite/src } |  |  | 0.405 |
| walker |  | 436 | 77 | Fs::DirListing { dir: packages/vite } |  |  | 0.456 |
| walker |  | 445 | 9 | Fs::DirListing { dir: packages/vite/bin } |  |  | 0.456 |
| ns | 466 |  | 170 | README: dev server vs. build command, and extensibility | 1.6 | 1.4 | 0.424 |
| walker |  | 518 | 73 | Json::Identity { file: packages/vite/package.json } |  |  | 0.429 |
| walker |  | 535 | 17 | Fs::DirListing { dir: packages/vite/src } |  |  | 0.522 |
| ns | 543 |  | 77 | packages/vite top-level listing | 1.7 |  | 0.537 |
| walker |  | 606 | 71 | Fs::DirListing { dir: packages/vite/src/module-runner } |  |  | 0.539 |
| walker |  | 618 | 12 | Fs::DirListing { dir: packages/vite/src/module-runner/sourcemap } |  |  | 0.539 |
| walker |  | 636 | 18 | Fs::DirListing { dir: packages/vite/misc } |  |  | 0.512 |
| ns | 636 |  | 93 | Root package.json identity, engines, package manager | 1.8 |  | 0.512 |
| walker |  | 653 | 17 | Fs::DirListing { dir: packages/vite/src/client } |  |  | 0.513 |
| walker |  | 788 | 135 | Fs::DirListing { dir: packages/vite/src/node } |  |  | 0.522 |
| walker |  | 817 | 29 | Fs::DirListing { dir: packages/vite/src/node/optimizer } |  |  | 0.522 |
| ns | 859 |  | 223 | Root pnpm scripts: lint, typecheck and the test entry points | 1.9 |  | 0.470 |
| walker |  | 866 | 49 | Fs::DirListing { dir: packages/vite/src/node/ssr } |  |  | 0.473 |
| walker |  | 941 | 75 | Fs::DirListing { dir: packages/vite/src/node/server } |  |  | 0.476 |
| walker |  | 981 | 40 | Json::Runtime { file: packages/vite/package.json } |  |  | 0.476 |
| ns | 1118 |  | 259 | Root pnpm scripts: debug, docs, build, release | 1.10 | 1.9 | 0.438 |
| walker |  | 1130 | 149 | Fs::DirListing { dir: packages/vite/src/node/plugins } |  |  | 0.444 |
| ns | 1225 |  | 107 | vite package manifest: name, version, description, bin | 1.11 |  | 0.438 |
| walker |  | 1315 | 185 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.751 |
| walker |  | 1351 | 36 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.751 |
| walker |  | 1362 | 11 | Fs::DirListing { dir: packages/vite/src/node/ssr/runtime } |  |  | 0.751 |
| walker |  | 1406 | 44 | Fs::DirListing { dir: packages/vite/types } |  |  | 0.752 |
| walker |  | 1465 | 59 | Json::Runtime { file: package.json } |  |  | 0.779 |
| ns | 1485 |  | 260 | vite package exports and internal import aliases | 1.12 | 1.11 | 0.690 |
| walker |  | 1495 | 30 | Fs::DirListing { dir: scripts } |  |  | 0.690 |
| ns | 1640 |  | 155 | src/node/index.ts: primary API factories | 2.1 |  | 0.648 |
| walker |  | 1685 | 190 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.680 |
| walker |  | 1701 | 16 | Fs::DirListing { dir: packages/plugin-legacy/src/__tests__ } |  |  | 0.680 |
| walker |  | 1738 | 37 | Fs::DirListing { dir: packages/vite/types/internal } |  |  | 0.680 |
| walker |  | 1779 | 41 | Fs::DirListing { dir: .github } |  |  | 0.680 |
| ns | 1853 |  | 213 | vite CLI: the complete command roster | 2.2 |  | 0.635 |
| walker |  | 1858 | 79 | Fs::DirListing { dir: .github/workflows } |  |  | 0.637 |
| walker |  | 1922 | 64 | Fs::DirListing { dir: docs } |  |  | 0.638 |
| walker |  | 1937 | 15 | Fs::DirListing { dir: docs/_data } |  |  | 0.638 |
| walker |  | 1955 | 18 | Fs::DirListing { dir: packages/vite/src/node/server/environments } |  |  | 0.638 |
| walker |  | 1974 | 19 | Fs::DirListing { dir: docs/.vitepress } |  |  | 0.638 |
| walker |  | 1995 | 21 | Fs::DirListing { dir: docs/.vitepress/theme } |  |  | 0.638 |
| walker |  | 2017 | 22 | Fs::DirListing { dir: packages/vite/src/node/__tests_dts__ } |  |  | 0.638 |
| walker |  | 2067 | 50 | Fs::DirListing { dir: packages/vite/src/types } |  |  | 0.639 |
| walker |  | 2086 | 19 | Json::Identity { file: packages/vite/src/types/package.json } |  |  | 0.639 |
| ns | 2118 |  | 265 | src/node/index.ts: transform helpers and environment factories | 2.3 | 2.1 | 0.591 |
| ns | 2242 |  | 124 | src/node/index.ts: SSR/module-runner exports | 2.4 | 2.3 | 0.576 |
| walker |  | 2492 | 406 | Fs::DirListing { dir: playground } |  |  | 0.582 |
| walker |  | 2514 | 22 | Fs::DirListing { dir: playground/cli } |  |  | 0.582 |
| walker |  | 2536 | 22 | Fs::DirListing { dir: playground/cli-module } |  |  | 0.582 |
| walker |  | 2547 | 11 | Fs::DirListing { dir: playground/resolve-linked } |  |  | 0.582 |
| walker |  | 2551 | 4 | Fs::DirListing { dir: playground/resolve-linked/src } |  |  | 0.582 |
| ns | 2571 |  | 329 | src/node/index.ts: constants, utils and remaining value exports | 2.5 | 2.4 | 0.532 |
| walker |  | 2584 | 33 | Fs::DirListing { dir: playground/css-dynamic-import } |  |  | 0.532 |
| walker |  | 2620 | 36 | Fs::DirListing { dir: playground/assets-sanitize } |  |  | 0.532 |
| walker |  | 2657 | 37 | Fs::DirListing { dir: playground/env } |  |  | 0.532 |
| walker |  | 2673 | 16 | Fs::DirListing { dir: playground/devtools } |  |  | 0.532 |
| walker |  | 2681 | 8 | Fs::DirListing { dir: playground/devtools/src } |  |  | 0.532 |
| walker |  | 2720 | 39 | Fs::DirListing { dir: playground/csp } |  |  | 0.532 |
| walker |  | 2737 | 17 | Fs::DirListing { dir: playground/css-lightningcss-proxy } |  |  | 0.532 |
| walker |  | 2754 | 17 | Fs::DirListing { dir: playground/css-lightningcss-root } |  |  | 0.532 |
| walker |  | 2771 | 17 | Fs::DirListing { dir: playground/hmr-root } |  |  | 0.532 |
| walker |  | 2775 | 4 | Fs::DirListing { dir: playground/hmr-root/root } |  |  | 0.532 |
| walker |  | 2816 | 41 | Fs::DirListing { dir: playground/css-no-codesplit } |  |  | 0.532 |
| walker |  | 2834 | 18 | Fs::DirListing { dir: playground/client-reload } |  |  | 0.532 |
| walker |  | 2852 | 18 | Fs::DirListing { dir: playground/extensions } |  |  | 0.532 |
| ns | 2859 |  | 288 | index.ts type re-exports: config, server, build and plugin-option types | 2.6 | 2.5 | 0.493 |
| walker |  | 2870 | 18 | Fs::DirListing { dir: playground/proxy-bypass } |  |  | 0.493 |
| walker |  | 2915 | 45 | Fs::DirListing { dir: playground/transform-plugin } |  |  | 0.493 |
| walker |  | 2935 | 20 | Fs::DirListing { dir: playground/preserve-symlinks } |  |  | 0.493 |
| walker |  | 2939 | 4 | Fs::DirListing { dir: playground/preserve-symlinks/src } |  |  | 0.493 |
| walker |  | 2987 | 48 | Fs::DirListing { dir: playground/multiple-entrypoints } |  |  | 0.493 |
| walker |  | 3008 | 21 | Fs::DirListing { dir: playground/base-conflict } |  |  | 0.493 |
| walker |  | 3029 | 21 | Fs::DirListing { dir: playground/dynamic-import-inline } |  |  | 0.493 |
| walker |  | 3037 | 8 | Fs::DirListing { dir: playground/dynamic-import-inline/src } |  |  | 0.493 |
| walker |  | 3058 | 21 | Fs::DirListing { dir: playground/ssr-alias } |  |  | 0.493 |
| walker |  | 3066 | 8 | Fs::DirListing { dir: playground/ssr-alias/alias-original } |  |  | 0.493 |
| walker |  | 3087 | 21 | Fs::DirListing { dir: playground/ssr-pug } |  |  | 0.493 |
| walker |  | 3091 | 4 | Fs::DirListing { dir: playground/ssr-pug/src } |  |  | 0.493 |
| walker |  | 3112 | 21 | Fs::DirListing { dir: playground/ssr-wasm } |  |  | 0.493 |
| walker |  | 3123 | 11 | Fs::DirListing { dir: playground/preserve-symlinks/module-a } |  |  | 0.493 |
| walker |  | 3131 | 8 | Fs::DirListing { dir: playground/preserve-symlinks/module-a/src } |  |  | 0.493 |
| walker |  | 3153 | 22 | Fs::DirListing { dir: playground/build-old } |  |  | 0.493 |
| walker |  | 3175 | 22 | Fs::DirListing { dir: playground/object-hooks } |  |  | 0.493 |
| walker |  | 3197 | 22 | Fs::DirListing { dir: playground/proxy-hmr } |  |  | 0.493 |
| walker |  | 3221 | 24 | Fs::DirListing { dir: playground/backend-integration } |  |  | 0.493 |
| walker |  | 3231 | 10 | Fs::DirListing { dir: playground/backend-integration/frontend } |  |  | 0.493 |
| walker |  | 3235 | 4 | Fs::DirListing { dir: playground/backend-integration/frontend/images } |  |  | 0.493 |
| walker |  | 3259 | 24 | Fs::DirListing { dir: playground/forward-console } |  |  | 0.493 |
| walker |  | 3263 | 4 | Fs::DirListing { dir: playground/forward-console/src } |  |  | 0.493 |
| walker |  | 3287 | 24 | Fs::DirListing { dir: playground/optimize-deps-no-discovery } |  |  | 0.493 |
| walker |  | 3295 | 8 | Fs::DirListing { dir: playground/optimize-deps-no-discovery/dep-no-discovery } |  |  | 0.493 |
| ns | 3307 |  | 448 | index.ts type re-exports: server internals, HMR payloads, vendored types | 2.7 | 2.6 | 0.458 |
| walker |  | 3320 | 25 | Fs::DirListing { dir: playground/import-assertion } |  |  | 0.458 |
| walker |  | 3332 | 12 | Fs::DirListing { dir: playground/import-assertion/import-assertion-dep } |  |  | 0.458 |
| walker |  | 3357 | 25 | Fs::DirListing { dir: playground/ssr } |  |  | 0.458 |
| walker |  | 3382 | 25 | Fs::DirListing { dir: playground/tsconfig-json-load-error } |  |  | 0.458 |
| walker |  | 3386 | 4 | Fs::DirListing { dir: playground/tsconfig-json-load-error/src } |  |  | 0.458 |
| walker |  | 3395 | 9 | Fs::DirListing { dir: playground/tsconfig-json-load-error/has-error } |  |  | 0.458 |
| walker |  | 3408 | 13 | Fs::DirListing { dir: playground/backend-integration/dir } |  |  | 0.458 |
| walker |  | 3421 | 13 | Fs::DirListing { dir: playground/proxy-hmr/other-app } |  |  | 0.458 |
| walker |  | 3447 | 26 | Fs::DirListing { dir: playground/env-nested } |  |  | 0.458 |
| walker |  | 3458 | 11 | Fs::DirListing { dir: playground/env-nested/envs } |  |  | 0.458 |
| walker |  | 3484 | 26 | Fs::DirListing { dir: playground/environment-react-ssr } |  |  | 0.458 |
| walker |  | 3499 | 15 | Fs::DirListing { dir: playground/base-conflict/src } |  |  | 0.458 |
| walker |  | 3514 | 15 | Fs::DirListing { dir: playground/ssr-alias/src } |  |  | 0.458 |
| walker |  | 3543 | 29 | Fs::DirListing { dir: playground/minify } |  |  | 0.458 |
| walker |  | 3546 | 3 | Fs::DirListing { dir: playground/minify/dir } |  |  | 0.458 |
| walker |  | 3558 | 12 | Fs::DirListing { dir: playground/minify/dir/module } |  |  | 0.458 |
| ns | 3569 |  | 262 | Plugin interface: every Vite-specific hook and flag (names only) | 2.8 |  | 0.440 |
| walker |  | 3587 | 29 | Fs::DirListing { dir: playground/resolve-tsconfig-paths } |  |  | 0.440 |
| walker |  | 3591 | 4 | Fs::DirListing { dir: playground/resolve-tsconfig-paths/fallback } |  |  | 0.440 |
| walker |  | 3620 | 29 | Fs::DirListing { dir: playground/tailwind-sourcemap } |  |  | 0.440 |
| walker |  | 3650 | 30 | Fs::DirListing { dir: playground/alias } |  |  | 0.440 |
| walker |  | 3680 | 30 | Fs::DirListing { dir: playground/tsconfig-json } |  |  | 0.440 |
| walker |  | 3694 | 14 | Fs::DirListing { dir: playground/tsconfig-json/src } |  |  | 0.440 |
| walker |  | 3709 | 15 | Fs::DirListing { dir: playground/tsconfig-json/nested } |  |  | 0.440 |
| walker |  | 3724 | 15 | Fs::DirListing { dir: playground/tsconfig-json/nested-with-extends } |  |  | 0.440 |
| walker |  | 3755 | 31 | Fs::DirListing { dir: playground/module-graph } |  |  | 0.440 |
| walker |  | 3786 | 31 | Fs::DirListing { dir: playground/ssr-webworker } |  |  | 0.440 |
| walker |  | 3795 | 9 | Fs::DirListing { dir: playground/ssr-webworker/src } |  |  | 0.440 |
| walker |  | 3807 | 12 | Fs::DirListing { dir: playground/ssr-webworker/browser-exports } |  |  | 0.440 |
| walker |  | 3823 | 16 | Fs::DirListing { dir: playground/ssr-webworker/worker-exports } |  |  | 0.440 |
| walker |  | 3855 | 32 | Fs::DirListing { dir: playground/optimize-missing-deps } |  |  | 0.440 |
| walker |  | 3863 | 8 | Fs::DirListing { dir: playground/optimize-missing-deps/missing-dep } |  |  | 0.440 |
| walker |  | 3876 | 13 | Fs::DirListing { dir: playground/optimize-missing-deps/multi-entry-dep } |  |  | 0.440 |
| walker |  | 3893 | 17 | Fs::DirListing { dir: playground/environment-react-ssr/src } |  |  | 0.440 |
| walker |  | 3926 | 33 | Fs::DirListing { dir: playground/define } |  |  | 0.440 |
| ns | 3931 |  | 362 | ViteDevServer: every member (names only) | 2.9 |  | 0.414 |
| walker |  | 3934 | 8 | Fs::DirListing { dir: playground/define/commonjs-dep } |  |  | 0.414 |
| walker |  | 3967 | 33 | Fs::DirListing { dir: playground/ssr-conditions } |  |  | 0.414 |
| walker |  | 3971 | 4 | Fs::DirListing { dir: playground/ssr-conditions/src } |  |  | 0.414 |
| walker |  | 4005 | 34 | Fs::DirListing { dir: playground/css-codesplit-cjs } |  |  | 0.414 |
| walker |  | 4039 | 34 | Fs::DirListing { dir: playground/tailwind } |  |  | 0.414 |
| walker |  | 4043 | 4 | Fs::DirListing { dir: playground/tailwind/public } |  |  | 0.414 |
| walker |  | 4053 | 10 | Fs::DirListing { dir: playground/tailwind/src } |  |  | 0.414 |
| walker |  | 4058 | 5 | Fs::DirListing { dir: playground/tailwind/src/components } |  |  | 0.414 |
| walker |  | 4063 | 5 | Fs::DirListing { dir: playground/tailwind/src/views } |  |  | 0.414 |
| walker |  | 4081 | 18 | Fs::DirListing { dir: playground/css-lightningcss-root/root } |  |  | 0.414 |
| ns | 4111 |  | 180 | UserConfig keys: project, sources and transform options | 3.1 |  | 0.405 |
| walker |  | 4116 | 35 | Fs::DirListing { dir: playground/fs-serve } |  |  | 0.405 |
| walker |  | 4120 | 4 | Fs::DirListing { dir: playground/fs-serve/nested } |  |  | 0.405 |
| walker |  | 4156 | 36 | Fs::DirListing { dir: playground/external } |  |  | 0.405 |
| walker |  | 4164 | 8 | Fs::DirListing { dir: playground/external/dep-that-imports } |  |  | 0.405 |
| walker |  | 4172 | 8 | Fs::DirListing { dir: playground/external/dep-that-requires } |  |  | 0.405 |
| walker |  | 4182 | 10 | Fs::DirListing { dir: playground/external/public } |  |  | 0.405 |
| walker |  | 4192 | 10 | Fs::DirListing { dir: playground/external/src } |  |  | 0.405 |
| walker |  | 4211 | 19 | Fs::DirListing { dir: playground/resolve-tsconfig-paths/src } |  |  | 0.405 |
| walker |  | 4252 | 41 | Fs::DirListing { dir: playground/resolve-tsconfig-paths/src/nested } |  |  | 0.405 |
| walker |  | 4289 | 37 | Fs::DirListing { dir: playground/ssr-noexternal } |  |  | 0.405 |
| walker |  | 4294 | 5 | Fs::DirListing { dir: playground/ssr-noexternal/src } |  |  | 0.405 |
| walker |  | 4302 | 8 | Fs::DirListing { dir: playground/ssr-noexternal/require-external-cjs } |  |  | 0.405 |
| walker |  | 4316 | 14 | Fs::DirListing { dir: playground/ssr-noexternal/external-cjs } |  |  | 0.405 |
| walker |  | 4353 | 37 | Fs::DirListing { dir: playground/tailwind-v3 } |  |  | 0.405 |
| walker |  | 4363 | 10 | Fs::DirListing { dir: playground/tailwind-v3/src } |  |  | 0.405 |
| walker |  | 4368 | 5 | Fs::DirListing { dir: playground/tailwind-v3/src/components } |  |  | 0.405 |
| walker |  | 4373 | 5 | Fs::DirListing { dir: playground/tailwind-v3/src/views } |  |  | 0.405 |
| ns | 4401 |  | 290 | UserConfig keys: server, build, env, worker and the rest | 3.2 | 3.1 | 0.390 |
| walker |  | 4412 | 39 | Fs::DirListing { dir: playground/ssr-resolve } |  |  | 0.390 |
| walker |  | 4420 | 8 | Fs::DirListing { dir: playground/ssr-resolve/pkg-module-sync } |  |  | 0.390 |
| walker |  | 4432 | 12 | Fs::DirListing { dir: playground/ssr-resolve/pkg-exports } |  |  | 0.390 |
| walker |  | 4449 | 17 | Fs::DirListing { dir: playground/ssr-resolve/deep-import } |  |  | 0.390 |
| walker |  | 4457 | 8 | Fs::DirListing { dir: playground/ssr-resolve/deep-import/foo } |  |  | 0.390 |
| walker |  | 4461 | 4 | Fs::DirListing { dir: playground/ssr-resolve/deep-import/bar } |  |  | 0.390 |
| walker |  | 4465 | 4 | Fs::DirListing { dir: playground/ssr-resolve/deep-import/utils } |  |  | 0.390 |
| walker |  | 4476 | 11 | Fs::DirListing { dir: playground/ssr-resolve/entries } |  |  | 0.390 |
| walker |  | 4480 | 4 | Fs::DirListing { dir: playground/ssr-resolve/entries/dir } |  |  | 0.390 |
| walker |  | 4520 | 40 | Fs::DirListing { dir: playground/json } |  |  | 0.390 |
| walker |  | 4532 | 12 | Fs::DirListing { dir: playground/json/dep-json-require } |  |  | 0.390 |
| walker |  | 4536 | 4 | Fs::DirListing { dir: playground/json/public } |  |  | 0.390 |
| walker |  | 4542 | 6 | Fs::DirListing { dir: playground/json/json-bom } |  |  | 0.390 |
| walker |  | 4550 | 8 | Fs::DirListing { dir: playground/json/json-module } |  |  | 0.390 |
| ns | 4571 |  | 170 | Per-environment options (SharedEnvironmentOptions / EnvironmentOptions) | 3.3 |  | 0.382 |
| walker |  | 4606 | 56 | Json::Identity { file: playground/package.json } |  |  | 0.382 |
| walker |  | 4647 | 41 | Fs::DirListing { dir: playground/dynamic-import } |  |  | 0.382 |
| walker |  | 4659 | 12 | Fs::DirListing { dir: playground/dynamic-import/pkg } |  |  | 0.382 |
| walker |  | 4663 | 4 | Fs::DirListing { dir: playground/dynamic-import/css } |  |  | 0.382 |
| walker |  | 4670 | 7 | Fs::DirListing { dir: playground/dynamic-import/(app) } |  |  | 0.382 |
| walker |  | 4674 | 4 | Fs::DirListing { dir: playground/dynamic-import/(app)/nest } |  |  | 0.382 |
| walker |  | 4706 | 32 | Fs::DirListing { dir: playground/dynamic-import/nested } |  |  | 0.382 |
| ns | 4707 |  | 136 | CommonServerOptions: the host/port/https/proxy/cors keys | 3.4 |  | 0.376 |
| walker |  | 4710 | 4 | Fs::DirListing { dir: playground/dynamic-import/nested/nested } |  |  | 0.376 |
| walker |  | 4720 | 10 | Fs::DirListing { dir: playground/dynamic-import/files } |  |  | 0.376 |
| walker |  | 4736 | 16 | Fs::DirListing { dir: playground/dynamic-import/alias } |  |  | 0.376 |
| walker |  | 4753 | 17 | Fs::DirListing { dir: playground/dynamic-import/views } |  |  | 0.376 |
| walker |  | 4763 | 10 | Fs::DirListing { dir: playground/dynamic-import/nested/treeshaken } |  |  | 0.376 |
| walker |  | 4806 | 43 | Fs::DirListing { dir: playground/ssr-html } |  |  | 0.376 |
| walker |  | 4816 | 10 | Fs::DirListing { dir: playground/ssr-html/public } |  |  | 0.376 |
| walker |  | 4859 | 43 | Fs::DirListing { dir: playground/wasm } |  |  | 0.376 |
| walker |  | 4882 | 23 | Fs::DirListing { dir: playground/alias/dir } |  |  | 0.376 |
| walker |  | 4890 | 8 | Fs::DirListing { dir: playground/alias/dir/module } |  |  | 0.376 |
| walker |  | 4935 | 45 | Fs::DirListing { dir: playground/hmr-full-bundle-mode } |  |  | 0.376 |
| ns | 4944 |  | 237 | ServerOptions and FileSystemServeOptions keys | 3.5 |  | 0.367 |
| walker |  | 4959 | 24 | Fs::DirListing { dir: playground/ssr-wasm/src } |  |  | 0.367 |
| walker |  | 5071 | 112 | Fs::DirListing { dir: playground/css-sourcemap } |  |  | 0.367 |
| walker |  | 5122 | 51 | Fs::DirListing { dir: playground/preload } |  |  | 0.367 |
| walker |  | 5130 | 8 | Fs::DirListing { dir: playground/preload/dep-a } |  |  | 0.367 |
| walker |  | 5138 | 8 | Fs::DirListing { dir: playground/preload/dep-including-a } |  |  | 0.367 |
| walker |  | 5143 | 5 | Fs::DirListing { dir: playground/preload/public } |  |  | 0.367 |
| walker |  | 5164 | 21 | Fs::DirListing { dir: playground/preload/src } |  |  | 0.367 |
| ns | 5180 |  | 236 | BuildEnvironmentOptions keys: output, assets, CSS, minification | 3.6 |  | 0.360 |
| walker |  | 5191 | 27 | Fs::DirListing { dir: playground/ssr-conditions/external } |  |  | 0.360 |
| walker |  | 5218 | 27 | Fs::DirListing { dir: playground/ssr-conditions/no-external } |  |  | 0.360 |
| walker |  | 5271 | 53 | Fs::DirListing { dir: playground/nested-deps } |  |  | 0.360 |
| walker |  | 5279 | 8 | Fs::DirListing { dir: playground/nested-deps/test-package-a } |  |  | 0.360 |
| walker |  | 5287 | 8 | Fs::DirListing { dir: playground/nested-deps/test-package-f } |  |  | 0.360 |
| walker |  | 5298 | 11 | Fs::DirListing { dir: playground/nested-deps/self-referencing } |  |  | 0.360 |
| walker |  | 5310 | 12 | Fs::DirListing { dir: playground/nested-deps/test-package-b } |  |  | 0.360 |
| walker |  | 5325 | 15 | Fs::DirListing { dir: playground/nested-deps/test-package-d } |  |  | 0.360 |
| walker |  | 5333 | 8 | Fs::DirListing { dir: playground/nested-deps/test-package-d/test-package-d-nested } |  |  | 0.360 |
| walker |  | 5350 | 17 | Fs::DirListing { dir: playground/nested-deps/test-package-c } |  |  | 0.360 |
| walker |  | 5372 | 22 | Fs::DirListing { dir: playground/nested-deps/test-package-e } |  |  | 0.360 |
| walker |  | 5380 | 8 | Fs::DirListing { dir: playground/nested-deps/test-package-e/test-package-e-excluded } |  |  | 0.360 |
| walker |  | 5388 | 8 | Fs::DirListing { dir: playground/nested-deps/test-package-e/test-package-e-included } |  |  | 0.360 |
| walker |  | 5392 | 4 | Fs::DirListing { dir: playground/nested-deps/self-referencing/test } |  |  | 0.360 |
| walker |  | 5409 | 17 | Fs::DirListing { dir: playground/backend-integration/frontend/styles } |  |  | 0.360 |
| ns | 5437 |  | 257 | BuildEnvironmentOptions keys: bundler passthrough, lib, ssr, reporting | 3.7 | 3.6 | 0.351 |
| walker |  | 5439 | 30 | Fs::DirListing { dir: playground/ssr/src } |  |  | 0.351 |
| walker |  | 5451 | 12 | Fs::DirListing { dir: playground/ssr/src/circular-import } |  |  | 0.351 |
| walker |  | 5463 | 12 | Fs::DirListing { dir: playground/ssr/src/circular-import2 } |  |  | 0.351 |
| walker |  | 5506 | 43 | Json::Identity { file: playground/css-lightningcss-root/package.json } |  |  | 0.351 |
| ns | 5530 |  | 93 | ExperimentalOptions and FutureOptions | 3.8 |  | 0.348 |
| walker |  | 5538 | 32 | Fs::DirListing { dir: playground/ssr-html/src } |  |  | 0.348 |
| walker |  | 5600 | 62 | Fs::DirListing { dir: playground/data-uri } |  |  | 0.348 |
| walker |  | 5644 | 44 | Json::Identity { file: playground/css-lightningcss-proxy/package.json } |  |  | 0.348 |
| walker |  | 5665 | 21 | Fs::DirListing { dir: playground/ssr/src/circular-dep-init } |  |  | 0.348 |
| walker |  | 5695 | 30 | Json::Identity { file: playground/nested-deps/self-referencing/package.json } |  |  | 0.348 |
| walker |  | 5749 | 54 | Json::Identity { file: playground/env/package.json } |  |  | 0.348 |
| walker |  | 5803 | 54 | Json::Identity { file: playground/json/package.json } |  |  | 0.348 |
| ns | 5804 |  | 274 | CLI global options (all five commands) | 3.9 | 2.2 | 0.341 |
| walker |  | 5858 | 55 | Json::Identity { file: playground/alias/package.json } |  |  | 0.341 |
| walker |  | 5913 | 55 | Json::Identity { file: playground/build-old/package.json } |  |  | 0.341 |
| walker |  | 5968 | 55 | Json::Identity { file: playground/cli/package.json } |  |  | 0.341 |
| ns | 5995 |  | 191 | CLI dev-server flags | 3.10 | 2.2 | 0.336 |
| walker |  | 6023 | 55 | Json::Identity { file: playground/cli-module/package.json } |  |  | 0.336 |
| walker |  | 6078 | 55 | Json::Identity { file: playground/csp/package.json } |  |  | 0.336 |
| walker |  | 6133 | 55 | Json::Identity { file: playground/data-uri/package.json } |  |  | 0.336 |
| walker |  | 6188 | 55 | Json::Identity { file: playground/define/package.json } |  |  | 0.336 |
| walker |  | 6243 | 55 | Json::Identity { file: playground/devtools/package.json } |  |  | 0.336 |
| walker |  | 6298 | 55 | Json::Identity { file: playground/extensions/package.json } |  |  | 0.336 |
| walker |  | 6353 | 55 | Json::Identity { file: playground/external/package.json } |  |  | 0.336 |
| walker |  | 6408 | 55 | Json::Identity { file: playground/forward-console/package.json } |  |  | 0.336 |
| walker |  | 6463 | 55 | Json::Identity { file: playground/minify/package.json } |  |  | 0.336 |
| ns | 6467 |  | 472 | CLI build flags | 3.11 | 2.2 | 0.323 |
| walker |  | 6518 | 55 | Json::Identity { file: playground/module-graph/package.json } |  |  | 0.323 |
| walker |  | 6573 | 55 | Json::Identity { file: playground/object-hooks/package.json } |  |  | 0.323 |
| ns | 6622 |  | 155 | CLI optimize and preview flags | 3.12 | 2.2 | 0.320 |
| walker |  | 6628 | 55 | Json::Identity { file: playground/preload/package.json } |  |  | 0.320 |
| walker |  | 6683 | 55 | Json::Identity { file: playground/tailwind/package.json } |  |  | 0.320 |
| walker |  | 6738 | 55 | Json::Identity { file: playground/transform-plugin/package.json } |  |  | 0.320 |
| ns | 6757 |  | 135 | src/node module roster (complete) | 4.1 |  | 0.372 |
| walker |  | 6793 | 55 | Json::Identity { file: playground/wasm/package.json } |  |  | 0.372 |
| walker |  | 6849 | 56 | Json::Identity { file: playground/assets-sanitize/package.json } |  |  | 0.372 |
| walker |  | 6905 | 56 | Json::Identity { file: playground/base-conflict/package.json } |  |  | 0.372 |
| ns | 6906 |  | 149 | src/node/plugins roster (complete) | 4.2 |  | 0.410 |
| walker |  | 6961 | 56 | Json::Identity { file: playground/client-reload/package.json } |  |  | 0.410 |
| ns | 6981 |  | 75 | src/node/server roster (complete) | 4.3 |  | 0.428 |
| walker |  | 7017 | 56 | Json::Identity { file: playground/dynamic-import/package.json } |  |  | 0.428 |
| ns | 7066 |  | 85 | Dev-server middlewares and per-environment implementations | 4.4 |  | 0.422 |
| walker |  | 7073 | 56 | Json::Identity { file: playground/env-nested/package.json } |  |  | 0.422 |
| walker |  | 7129 | 56 | Json::Identity { file: playground/resolve-linked/package.json } |  |  | 0.422 |
| ns | 7144 |  | 78 | SSR and dependency-optimizer rosters | 4.5 |  | 0.437 |
| walker |  | 7185 | 56 | Json::Identity { file: playground/ssr/package.json } |  |  | 0.437 |
| walker |  | 7241 | 56 | Json::Identity { file: playground/tsconfig-json/package.json } |  |  | 0.437 |
| walker |  | 7274 | 33 | Json::Identity { file: playground/json/dep-json-require/package.json } |  |  | 0.437 |
| ns | 7287 |  | 143 | Browser client, module-runner and shared rosters | 4.6 |  | 0.440 |
| walker |  | 7331 | 57 | Json::Identity { file: playground/backend-integration/package.json } |  |  | 0.440 |
| walker |  | 7388 | 57 | Json::Identity { file: playground/css-dynamic-import/package.json } |  |  | 0.440 |
| ns | 7400 |  | 113 | resolvePlugins: the built-in plugin pipeline order | 4.7 |  | 0.435 |
| walker |  | 7445 | 57 | Json::Identity { file: playground/css-sourcemap/package.json } |  |  | 0.435 |
| ns | 7494 |  | 94 | Published and inlined type declarations | 4.8 |  | 0.450 |
| walker |  | 7502 | 57 | Json::Identity { file: playground/dynamic-import-inline/package.json } |  |  | 0.450 |
| walker |  | 7559 | 57 | Json::Identity { file: playground/fs-serve/package.json } |  |  | 0.450 |
| ns | 7609 |  | 115 | Core internal entry-point signatures | 4.9 |  | 0.445 |
| walker |  | 7616 | 57 | Json::Identity { file: playground/import-assertion/package.json } |  |  | 0.445 |
| walker |  | 7673 | 57 | Json::Identity { file: playground/multiple-entrypoints/package.json } |  |  | 0.445 |
| walker |  | 7730 | 57 | Json::Identity { file: playground/nested-deps/package.json } |  |  | 0.445 |
| walker |  | 7787 | 57 | Json::Identity { file: playground/proxy-bypass/package.json } |  |  | 0.445 |
| walker |  | 7844 | 57 | Json::Identity { file: playground/proxy-hmr/package.json } |  |  | 0.445 |
| walker |  | 7901 | 57 | Json::Identity { file: playground/ssr-alias/package.json } |  |  | 0.445 |
| ns | 7919 |  | 310 | CONTRIBUTING.md: every section heading | 5.1 |  | 0.436 |
| walker |  | 7958 | 57 | Json::Identity { file: playground/ssr-html/package.json } |  |  | 0.436 |
| walker |  | 8015 | 57 | Json::Identity { file: playground/tailwind-v3/package.json } |  |  | 0.436 |
| walker |  | 8073 | 58 | Json::Identity { file: playground/css-no-codesplit/package.json } |  |  | 0.436 |
| ns | 8109 |  | 190 | CONTRIBUTING: local development loop | 5.2 | 5.1 | 0.433 |
| walker |  | 8131 | 58 | Json::Identity { file: playground/ssr-conditions/package.json } |  |  | 0.433 |
| walker |  | 8189 | 58 | Json::Identity { file: playground/ssr-noexternal/package.json } |  |  | 0.433 |
| walker |  | 8247 | 58 | Json::Identity { file: playground/ssr-pug/package.json } |  |  | 0.433 |
| walker |  | 8305 | 58 | Json::Identity { file: playground/ssr-wasm/package.json } |  |  | 0.433 |
| walker |  | 8363 | 58 | Json::Identity { file: playground/ssr-webworker/package.json } |  |  | 0.433 |
| walker |  | 8421 | 58 | Json::Identity { file: playground/tailwind-sourcemap/package.json } |  |  | 0.433 |
| walker |  | 8479 | 58 | Json::Identity { file: playground/tsconfig-json-load-error/package.json } |  |  | 0.433 |
| walker |  | 8513 | 34 | Json::Identity { file: playground/json/json-module/package.json } |  |  | 0.433 |
| ns | 8515 |  | 406 | playground/ roster (complete e2e corpus) | 5.3 |  | 0.488 |
| walker |  | 8572 | 59 | Json::Identity { file: playground/environment-react-ssr/package.json } |  |  | 0.488 |
| walker |  | 8631 | 59 | Json::Identity { file: playground/hmr-full-bundle-mode/package.json } |  |  | 0.488 |
| walker |  | 8690 | 59 | Json::Identity { file: playground/optimize-missing-deps/package.json } |  |  | 0.488 |
| walker |  | 8749 | 59 | Json::Identity { file: playground/preserve-symlinks/package.json } |  |  | 0.488 |
| ns | 8802 |  | 287 | CONTRIBUTING: how the integration tests work | 5.4 | 5.3 | 0.486 |
| walker |  | 8808 | 59 | Json::Identity { file: playground/resolve-tsconfig-paths/package.json } |  |  | 0.486 |
| walker |  | 8867 | 59 | Json::Identity { file: playground/ssr-resolve/package.json } |  |  | 0.486 |
| ns | 8919 |  | 117 | Unit test locations under packages/vite/src | 5.5 |  | 0.478 |
| walker |  | 8927 | 60 | Json::Identity { file: playground/css-codesplit-cjs/package.json } |  |  | 0.478 |
| walker |  | 8987 | 60 | Json::Identity { file: playground/optimize-deps-no-discovery/package.json } |  |  | 0.478 |
| ns | 9050 |  | 131 | How the unit and e2e vitest runs are separated | 5.6 |  | 0.476 |
| walker |  | 9074 | 87 | Fs::DirListing { dir: playground/glob-import } |  |  | 0.476 |
| walker |  | 9078 | 4 | Fs::DirListing { dir: playground/glob-import/subpath-imports-sub } |  |  | 0.476 |
| walker |  | 9086 | 8 | Fs::DirListing { dir: playground/glob-import/import-meta-glob-pkg } |  |  | 0.476 |
| walker |  | 9090 | 4 | Fs::DirListing { dir: playground/glob-import/pkg-pages } |  |  | 0.476 |
| walker |  | 9096 | 6 | Fs::DirListing { dir: playground/glob-import/follow-symlinks } |  |  | 0.476 |
| walker |  | 9100 | 4 | Fs::DirListing { dir: playground/glob-import/follow-symlinks/packages } |  |  | 0.476 |
| ns | 9114 |  | 64 | docs/ site roster | 5.7 |  | 0.485 |
| walker |  | 9132 | 32 | Fs::DirListing { dir: playground/glob-import/dir } |  |  | 0.485 |
| walker |  | 9136 | 4 | Fs::DirListing { dir: playground/glob-import/dir/nested } |  |  | 0.485 |
| walker |  | 9144 | 8 | Fs::DirListing { dir: playground/glob-import/array-common-base } |  |  | 0.485 |
| walker |  | 9148 | 4 | Fs::DirListing { dir: playground/glob-import/array-common-base/pattern1 } |  |  | 0.485 |
| walker |  | 9152 | 4 | Fs::DirListing { dir: playground/glob-import/array-common-base/pattern2 } |  |  | 0.485 |
| walker |  | 9160 | 8 | Fs::DirListing { dir: playground/glob-import/array-test-dir } |  |  | 0.485 |
| walker |  | 9168 | 8 | Fs::DirListing { dir: playground/glob-import/imports-path } |  |  | 0.485 |
| walker |  | 9171 | 3 | Fs::DirListing { dir: playground/glob-import/follow-symlinks/packages/my-lib } |  |  | 0.485 |
| walker |  | 9182 | 11 | Fs::DirListing { dir: playground/glob-import/side-effect } |  |  | 0.485 |
| walker |  | 9189 | 7 | Fs::DirListing { dir: playground/glob-import/follow-symlinks/linked } |  |  | 0.485 |
| walker |  | 9207 | 18 | Fs::DirListing { dir: playground/glob-import/escape } |  |  | 0.485 |
| walker |  | 9214 | 7 | Fs::DirListing { dir: playground/glob-import/escape/(parenthesis) } |  |  | 0.485 |
| walker |  | 9218 | 4 | Fs::DirListing { dir: playground/glob-import/escape/(parenthesis)/mod } |  |  | 0.485 |
| walker |  | 9225 | 7 | Fs::DirListing { dir: playground/glob-import/escape/[brackets] } |  |  | 0.485 |
| walker |  | 9229 | 4 | Fs::DirListing { dir: playground/glob-import/escape/[brackets]/mod } |  |  | 0.485 |
| walker |  | 9236 | 7 | Fs::DirListing { dir: playground/glob-import/escape/{curlies} } |  |  | 0.485 |
| walker |  | 9240 | 4 | Fs::DirListing { dir: playground/glob-import/escape/{curlies}/mod } |  |  | 0.485 |
| walker |  | 9248 | 8 | Fs::DirListing { dir: playground/glob-import/follow-symlinks/packages/my-lib/components } |  |  | 0.485 |
| ns | 9298 |  | 184 | docs/guide and docs/config page rosters | 5.8 |  | 0.475 |
| walker |  | 9303 | 55 | Json::Identity { file: playground/glob-import/package.json } |  |  | 0.475 |
| walker |  | 9337 | 34 | Json::Identity { file: playground/glob-import/import-meta-glob-pkg/package.json } |  |  | 0.475 |
| ns | 9343 |  | 45 | docs/changes: the breaking-change / migration notes | 5.9 |  | 0.473 |
| walker |  | 9350 | 13 | Json::Entry { file: playground/resolve-linked/package.json } |  |  | 0.473 |
| walker |  | 9397 | 47 | Fs::DirListing { dir: playground/fs-serve/root } |  |  | 0.473 |
| walker |  | 9491 | 94 | Fs::DirListing { dir: playground/css-lightningcss } |  |  | 0.473 |
| walker |  | 9499 | 8 | Fs::DirListing { dir: playground/css-lightningcss/nested } |  |  | 0.473 |
| ns | 9501 |  | 158 | CONTRIBUTING: the dependency policy | 5.10 | 5.1 | 0.471 |
| walker |  | 9541 | 42 | Json::Identity { file: playground/css-lightningcss/package.json } |  |  | 0.471 |
| walker |  | 9554 | 13 | Fs::DirListing { dir: playground/transform-plugin/__tests__ } |  |  | 0.471 |
| ns | 9624 |  | 123 | create-vite: package layout and template roster | 5.11 |  | 0.484 |
| walker |  | 9650 | 96 | Fs::DirListing { dir: playground/assets } |  |  | 0.484 |
| walker |  | 9659 | 9 | Fs::DirListing { dir: playground/assets/import-meta-url } |  |  | 0.484 |
| walker |  | 9674 | 15 | Fs::DirListing { dir: playground/assets/fonts } |  |  | 0.484 |
| ns | 9677 |  | 53 | plugin-legacy: package layout | 5.12 |  | 0.490 |
| walker |  | 9691 | 17 | Fs::DirListing { dir: playground/assets/asset } |  |  | 0.490 |
| walker |  | 9728 | 37 | Fs::DirListing { dir: playground/assets/css } |  |  | 0.490 |
| walker |  | 9736 | 8 | Fs::DirListing { dir: playground/assets/css/nested } |  |  | 0.490 |
| walker |  | 9790 | 54 | Json::Identity { file: playground/assets/package.json } |  |  | 0.490 |
| ns | 9827 |  | 150 | Repository automation: .github and release scripts | 5.13 |  | 0.501 |
| walker |  | 9831 | 41 | Fs::DirListing { dir: playground/assets/static } |  |  | 0.501 |
| walker |  | 9859 | 28 | Json::Scripts { file: playground/package.json } |  |  | 0.501 |
| walker |  | 9958 | 99 | Fs::DirListing { dir: playground/lib } |  |  | 0.501 |
| walker |  | 9987 | 29 | Json::Identity { file: playground/lib/package.json } |  |  | 0.501 |
