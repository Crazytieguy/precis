Score(3000)=0.465 I=0.821 C=0.263 ns_rows≤3K=18/55 grid(1000/1442/2080/3000/4327/6240/9000)=0.822/0.774/0.609/0.465/0.382/0.317/0.410

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
| walker |  | 309 | 77 | Fs::DirListing { dir: packages/vite } |  |  | 0.452 |
| walker |  | 318 | 9 | Fs::DirListing { dir: packages/vite/bin } |  |  | 0.452 |
| walker |  | 391 | 73 | Json::Identity { file: packages/vite/package.json } |  |  | 0.457 |
| walker |  | 408 | 17 | Fs::DirListing { dir: packages/vite/src } |  |  | 0.557 |
| ns | 466 |  | 170 | README: dev server vs. build command, and extensibility | 1.6 | 1.4 | 0.518 |
| walker |  | 479 | 71 | Fs::DirListing { dir: packages/vite/src/module-runner } |  |  | 0.519 |
| walker |  | 497 | 18 | Fs::DirListing { dir: packages/vite/misc } |  |  | 0.519 |
| walker |  | 514 | 17 | Fs::DirListing { dir: packages/vite/src/client } |  |  | 0.520 |
| ns | 543 |  | 77 | packages/vite top-level listing | 1.7 |  | 0.536 |
| ns | 636 |  | 93 | Root package.json identity, engines, package manager | 1.8 |  | 0.510 |
| walker |  | 649 | 135 | Fs::DirListing { dir: packages/vite/src/node } |  |  | 0.518 |
| walker |  | 689 | 40 | Json::Runtime { file: packages/vite/package.json } |  |  | 0.518 |
| walker |  | 701 | 12 | Fs::DirListing { dir: packages/vite/src/module-runner/sourcemap } |  |  | 0.518 |
| ns | 859 |  | 223 | Root pnpm scripts: lint, typecheck and the test entry points | 1.9 |  | 0.466 |
| walker |  | 886 | 185 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.821 |
| walker |  | 922 | 36 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.821 |
| walker |  | 934 | 12 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.821 |
| walker |  | 953 | 19 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.821 |
| walker |  | 997 | 44 | Fs::DirListing { dir: packages/vite/types } |  |  | 0.822 |
| walker |  | 1056 | 59 | Json::Runtime { file: package.json } |  |  | 0.853 |
| walker |  | 1086 | 30 | Fs::DirListing { dir: scripts } |  |  | 0.853 |
| walker |  | 1102 | 16 | Fs::DirListing { dir: packages/plugin-legacy/src/__tests__ } |  |  | 0.853 |
| ns | 1118 |  | 259 | Root pnpm scripts: debug, docs, build, release | 1.10 | 1.9 | 0.786 |
| walker |  | 1139 | 37 | Fs::DirListing { dir: packages/vite/types/internal } |  |  | 0.786 |
| ns | 1225 |  | 107 | vite package manifest: name, version, description, bin | 1.11 |  | 0.769 |
| walker |  | 1262 | 123 | Fs::DirListing { dir: packages/create-vite } |  |  | 0.771 |
| walker |  | 1266 | 4 | Fs::DirListing { dir: packages/create-vite/src } |  |  | 0.771 |
| walker |  | 1307 | 41 | Fs::DirListing { dir: .github } |  |  | 0.772 |
| walker |  | 1386 | 79 | Fs::DirListing { dir: .github/workflows } |  |  | 0.773 |
| walker |  | 1450 | 64 | Fs::DirListing { dir: docs } |  |  | 0.775 |
| walker |  | 1465 | 15 | Fs::DirListing { dir: docs/_data } |  |  | 0.775 |
| walker |  | 1484 | 19 | Fs::DirListing { dir: docs/.vitepress } |  |  | 0.775 |
| ns | 1485 |  | 260 | vite package exports and internal import aliases | 1.12 | 1.11 | 0.686 |
| walker |  | 1513 | 29 | Fs::DirListing { dir: packages/vite/src/node/optimizer } |  |  | 0.686 |
| walker |  | 1534 | 21 | Fs::DirListing { dir: docs/.vitepress/theme } |  |  | 0.686 |
| walker |  | 1556 | 22 | Fs::DirListing { dir: packages/vite/src/node/__tests_dts__ } |  |  | 0.686 |
| walker |  | 1606 | 50 | Fs::DirListing { dir: packages/vite/src/types } |  |  | 0.688 |
| walker |  | 1625 | 19 | Json::Identity { file: packages/vite/src/types/package.json } |  |  | 0.688 |
| ns | 1640 |  | 155 | src/node/index.ts: primary API factories | 2.1 |  | 0.645 |
| ns | 1853 |  | 213 | vite CLI: the complete command roster | 2.2 |  | 0.603 |
| walker |  | 2031 | 406 | Fs::DirListing { dir: playground } |  |  | 0.609 |
| walker |  | 2042 | 11 | Fs::DirListing { dir: playground/resolve-linked } |  |  | 0.609 |
| walker |  | 2046 | 4 | Fs::DirListing { dir: playground/resolve-linked/src } |  |  | 0.609 |
| walker |  | 2062 | 16 | Fs::DirListing { dir: playground/devtools } |  |  | 0.609 |
| walker |  | 2070 | 8 | Fs::DirListing { dir: playground/devtools/src } |  |  | 0.609 |
| walker |  | 2087 | 17 | Fs::DirListing { dir: playground/css-lightningcss-proxy } |  |  | 0.609 |
| walker |  | 2104 | 17 | Fs::DirListing { dir: playground/css-lightningcss-root } |  |  | 0.609 |
| ns | 2118 |  | 265 | src/node/index.ts: transform helpers and environment factories | 2.3 | 2.1 | 0.563 |
| walker |  | 2121 | 17 | Fs::DirListing { dir: playground/hmr-root } |  |  | 0.563 |
| walker |  | 2125 | 4 | Fs::DirListing { dir: playground/hmr-root/root } |  |  | 0.563 |
| walker |  | 2143 | 18 | Fs::DirListing { dir: playground/client-reload } |  |  | 0.563 |
| walker |  | 2161 | 18 | Fs::DirListing { dir: playground/extensions } |  |  | 0.563 |
| walker |  | 2179 | 18 | Fs::DirListing { dir: playground/proxy-bypass } |  |  | 0.563 |
| walker |  | 2199 | 20 | Fs::DirListing { dir: playground/preserve-symlinks } |  |  | 0.563 |
| walker |  | 2203 | 4 | Fs::DirListing { dir: playground/preserve-symlinks/src } |  |  | 0.563 |
| walker |  | 2224 | 21 | Fs::DirListing { dir: playground/base-conflict } |  |  | 0.563 |
| ns | 2242 |  | 124 | src/node/index.ts: SSR/module-runner exports | 2.4 | 2.3 | 0.549 |
| walker |  | 2245 | 21 | Fs::DirListing { dir: playground/dynamic-import-inline } |  |  | 0.549 |
| walker |  | 2253 | 8 | Fs::DirListing { dir: playground/dynamic-import-inline/src } |  |  | 0.549 |
| walker |  | 2274 | 21 | Fs::DirListing { dir: playground/ssr-alias } |  |  | 0.549 |
| walker |  | 2282 | 8 | Fs::DirListing { dir: playground/ssr-alias/alias-original } |  |  | 0.549 |
| walker |  | 2303 | 21 | Fs::DirListing { dir: playground/ssr-pug } |  |  | 0.549 |
| walker |  | 2307 | 4 | Fs::DirListing { dir: playground/ssr-pug/src } |  |  | 0.549 |
| walker |  | 2328 | 21 | Fs::DirListing { dir: playground/ssr-wasm } |  |  | 0.549 |
| walker |  | 2339 | 11 | Fs::DirListing { dir: playground/preserve-symlinks/module-a } |  |  | 0.549 |
| walker |  | 2361 | 22 | Fs::DirListing { dir: playground/build-old } |  |  | 0.549 |
| walker |  | 2383 | 22 | Fs::DirListing { dir: playground/cli } |  |  | 0.549 |
| walker |  | 2405 | 22 | Fs::DirListing { dir: playground/cli-module } |  |  | 0.549 |
| walker |  | 2427 | 22 | Fs::DirListing { dir: playground/object-hooks } |  |  | 0.549 |
| walker |  | 2449 | 22 | Fs::DirListing { dir: playground/proxy-hmr } |  |  | 0.549 |
| walker |  | 2473 | 24 | Fs::DirListing { dir: playground/backend-integration } |  |  | 0.549 |
| walker |  | 2483 | 10 | Fs::DirListing { dir: playground/backend-integration/frontend } |  |  | 0.549 |
| walker |  | 2487 | 4 | Fs::DirListing { dir: playground/backend-integration/frontend/images } |  |  | 0.549 |
| walker |  | 2511 | 24 | Fs::DirListing { dir: playground/forward-console } |  |  | 0.549 |
| walker |  | 2515 | 4 | Fs::DirListing { dir: playground/forward-console/src } |  |  | 0.549 |
| walker |  | 2539 | 24 | Fs::DirListing { dir: playground/optimize-deps-no-discovery } |  |  | 0.549 |
| walker |  | 2547 | 8 | Fs::DirListing { dir: playground/optimize-deps-no-discovery/dep-no-discovery } |  |  | 0.549 |
| ns | 2571 |  | 329 | src/node/index.ts: constants, utils and remaining value exports | 2.5 | 2.4 | 0.501 |
| walker |  | 2572 | 25 | Fs::DirListing { dir: playground/import-assertion } |  |  | 0.501 |
| walker |  | 2584 | 12 | Fs::DirListing { dir: playground/import-assertion/import-assertion-dep } |  |  | 0.501 |
| walker |  | 2609 | 25 | Fs::DirListing { dir: playground/ssr } |  |  | 0.501 |
| walker |  | 2634 | 25 | Fs::DirListing { dir: playground/tsconfig-json-load-error } |  |  | 0.501 |
| walker |  | 2638 | 4 | Fs::DirListing { dir: playground/tsconfig-json-load-error/src } |  |  | 0.501 |
| walker |  | 2647 | 9 | Fs::DirListing { dir: playground/tsconfig-json-load-error/has-error } |  |  | 0.501 |
| walker |  | 2660 | 13 | Fs::DirListing { dir: playground/backend-integration/dir } |  |  | 0.501 |
| walker |  | 2673 | 13 | Fs::DirListing { dir: playground/proxy-hmr/other-app } |  |  | 0.501 |
| walker |  | 2699 | 26 | Fs::DirListing { dir: playground/env-nested } |  |  | 0.501 |
| walker |  | 2710 | 11 | Fs::DirListing { dir: playground/env-nested/envs } |  |  | 0.501 |
| walker |  | 2736 | 26 | Fs::DirListing { dir: playground/environment-react-ssr } |  |  | 0.501 |
| walker |  | 2744 | 8 | Fs::DirListing { dir: playground/preserve-symlinks/module-a/src } |  |  | 0.501 |
| walker |  | 2759 | 15 | Fs::DirListing { dir: playground/base-conflict/src } |  |  | 0.501 |
| walker |  | 2774 | 15 | Fs::DirListing { dir: playground/ssr-alias/src } |  |  | 0.501 |
| walker |  | 2803 | 29 | Fs::DirListing { dir: playground/minify } |  |  | 0.501 |
| walker |  | 2806 | 3 | Fs::DirListing { dir: playground/minify/dir } |  |  | 0.501 |
| walker |  | 2835 | 29 | Fs::DirListing { dir: playground/resolve-tsconfig-paths } |  |  | 0.501 |
| walker |  | 2839 | 4 | Fs::DirListing { dir: playground/resolve-tsconfig-paths/fallback } |  |  | 0.501 |
| ns | 2859 |  | 288 | index.ts type re-exports: config, server, build and plugin-option types | 2.6 | 2.5 | 0.465 |
| walker |  | 2868 | 29 | Fs::DirListing { dir: playground/tailwind-sourcemap } |  |  | 0.465 |
| walker |  | 2898 | 30 | Fs::DirListing { dir: playground/alias } |  |  | 0.465 |
| walker |  | 2928 | 30 | Fs::DirListing { dir: playground/tsconfig-json } |  |  | 0.465 |
| walker |  | 2942 | 14 | Fs::DirListing { dir: playground/tsconfig-json/src } |  |  | 0.465 |
| walker |  | 2957 | 15 | Fs::DirListing { dir: playground/tsconfig-json/nested } |  |  | 0.465 |
| walker |  | 2972 | 15 | Fs::DirListing { dir: playground/tsconfig-json/nested-with-extends } |  |  | 0.465 |
| walker |  | 3003 | 31 | Fs::DirListing { dir: playground/module-graph } |  |  | 0.465 |
| walker |  | 3034 | 31 | Fs::DirListing { dir: playground/ssr-webworker } |  |  | 0.465 |
| walker |  | 3043 | 9 | Fs::DirListing { dir: playground/ssr-webworker/src } |  |  | 0.465 |
| walker |  | 3055 | 12 | Fs::DirListing { dir: playground/ssr-webworker/browser-exports } |  |  | 0.465 |
| walker |  | 3071 | 16 | Fs::DirListing { dir: playground/ssr-webworker/worker-exports } |  |  | 0.465 |
| walker |  | 3103 | 32 | Fs::DirListing { dir: playground/optimize-missing-deps } |  |  | 0.465 |
| walker |  | 3111 | 8 | Fs::DirListing { dir: playground/optimize-missing-deps/missing-dep } |  |  | 0.465 |
| walker |  | 3124 | 13 | Fs::DirListing { dir: playground/optimize-missing-deps/multi-entry-dep } |  |  | 0.465 |
| walker |  | 3141 | 17 | Fs::DirListing { dir: playground/environment-react-ssr/src } |  |  | 0.465 |
| walker |  | 3174 | 33 | Fs::DirListing { dir: playground/css-dynamic-import } |  |  | 0.465 |
| walker |  | 3207 | 33 | Fs::DirListing { dir: playground/define } |  |  | 0.465 |
| walker |  | 3215 | 8 | Fs::DirListing { dir: playground/define/commonjs-dep } |  |  | 0.465 |
| walker |  | 3248 | 33 | Fs::DirListing { dir: playground/ssr-conditions } |  |  | 0.465 |
| walker |  | 3252 | 4 | Fs::DirListing { dir: playground/ssr-conditions/src } |  |  | 0.465 |
| walker |  | 3286 | 34 | Fs::DirListing { dir: playground/css-codesplit-cjs } |  |  | 0.465 |
| ns | 3307 |  | 448 | index.ts type re-exports: server internals, HMR payloads, vendored types | 2.7 | 2.6 | 0.432 |
| walker |  | 3320 | 34 | Fs::DirListing { dir: playground/tailwind } |  |  | 0.432 |
| walker |  | 3324 | 4 | Fs::DirListing { dir: playground/tailwind/public } |  |  | 0.432 |
| walker |  | 3334 | 10 | Fs::DirListing { dir: playground/tailwind/src } |  |  | 0.432 |
| walker |  | 3339 | 5 | Fs::DirListing { dir: playground/tailwind/src/components } |  |  | 0.432 |
| walker |  | 3344 | 5 | Fs::DirListing { dir: playground/tailwind/src/views } |  |  | 0.432 |
| walker |  | 3362 | 18 | Fs::DirListing { dir: playground/css-lightningcss-root/root } |  |  | 0.432 |
| walker |  | 3397 | 35 | Fs::DirListing { dir: playground/fs-serve } |  |  | 0.432 |
| walker |  | 3401 | 4 | Fs::DirListing { dir: playground/fs-serve/nested } |  |  | 0.432 |
| walker |  | 3437 | 36 | Fs::DirListing { dir: playground/assets-sanitize } |  |  | 0.432 |
| walker |  | 3473 | 36 | Fs::DirListing { dir: playground/external } |  |  | 0.432 |
| walker |  | 3481 | 8 | Fs::DirListing { dir: playground/external/dep-that-imports } |  |  | 0.432 |
| walker |  | 3489 | 8 | Fs::DirListing { dir: playground/external/dep-that-requires } |  |  | 0.432 |
| walker |  | 3499 | 10 | Fs::DirListing { dir: playground/external/public } |  |  | 0.432 |
| walker |  | 3509 | 10 | Fs::DirListing { dir: playground/external/src } |  |  | 0.432 |
| walker |  | 3528 | 19 | Fs::DirListing { dir: playground/resolve-tsconfig-paths/src } |  |  | 0.432 |
| walker |  | 3569 | 41 | Fs::DirListing { dir: playground/resolve-tsconfig-paths/src/nested } |  |  | 0.415 |
| ns | 3569 |  | 262 | Plugin interface: every Vite-specific hook and flag (names only) | 2.8 |  | 0.415 |
| walker |  | 3606 | 37 | Fs::DirListing { dir: playground/env } |  |  | 0.415 |
| walker |  | 3643 | 37 | Fs::DirListing { dir: playground/ssr-noexternal } |  |  | 0.415 |
| walker |  | 3648 | 5 | Fs::DirListing { dir: playground/ssr-noexternal/src } |  |  | 0.415 |
| walker |  | 3656 | 8 | Fs::DirListing { dir: playground/ssr-noexternal/require-external-cjs } |  |  | 0.415 |
| walker |  | 3670 | 14 | Fs::DirListing { dir: playground/ssr-noexternal/external-cjs } |  |  | 0.415 |
| walker |  | 3707 | 37 | Fs::DirListing { dir: playground/tailwind-v3 } |  |  | 0.415 |
| walker |  | 3717 | 10 | Fs::DirListing { dir: playground/tailwind-v3/src } |  |  | 0.415 |
| walker |  | 3722 | 5 | Fs::DirListing { dir: playground/tailwind-v3/src/components } |  |  | 0.415 |
| walker |  | 3727 | 5 | Fs::DirListing { dir: playground/tailwind-v3/src/views } |  |  | 0.415 |
| walker |  | 3766 | 39 | Fs::DirListing { dir: playground/csp } |  |  | 0.415 |
| walker |  | 3805 | 39 | Fs::DirListing { dir: playground/ssr-resolve } |  |  | 0.415 |
| walker |  | 3813 | 8 | Fs::DirListing { dir: playground/ssr-resolve/pkg-module-sync } |  |  | 0.415 |
| walker |  | 3824 | 11 | Fs::DirListing { dir: playground/ssr-resolve/entries } |  |  | 0.415 |
| walker |  | 3828 | 4 | Fs::DirListing { dir: playground/ssr-resolve/entries/dir } |  |  | 0.415 |
| walker |  | 3840 | 12 | Fs::DirListing { dir: playground/ssr-resolve/pkg-exports } |  |  | 0.415 |
| walker |  | 3857 | 17 | Fs::DirListing { dir: playground/ssr-resolve/deep-import } |  |  | 0.415 |
| walker |  | 3861 | 4 | Fs::DirListing { dir: playground/ssr-resolve/deep-import/bar } |  |  | 0.415 |
| walker |  | 3865 | 4 | Fs::DirListing { dir: playground/ssr-resolve/deep-import/utils } |  |  | 0.415 |
| walker |  | 3873 | 8 | Fs::DirListing { dir: playground/ssr-resolve/deep-import/foo } |  |  | 0.415 |
| walker |  | 3885 | 12 | Fs::DirListing { dir: playground/minify/dir/module } |  |  | 0.415 |
| walker |  | 3925 | 40 | Fs::DirListing { dir: playground/json } |  |  | 0.415 |
| walker |  | 3929 | 4 | Fs::DirListing { dir: playground/json/public } |  |  | 0.415 |
| ns | 3931 |  | 362 | ViteDevServer: every member (names only) | 2.9 |  | 0.391 |
| walker |  | 3935 | 6 | Fs::DirListing { dir: playground/json/json-bom } |  |  | 0.391 |
| walker |  | 3943 | 8 | Fs::DirListing { dir: playground/json/json-module } |  |  | 0.391 |
| walker |  | 3955 | 12 | Fs::DirListing { dir: playground/json/dep-json-require } |  |  | 0.391 |
| walker |  | 4011 | 56 | Json::Identity { file: playground/package.json } |  |  | 0.391 |
| walker |  | 4052 | 41 | Fs::DirListing { dir: playground/css-no-codesplit } |  |  | 0.391 |
| walker |  | 4093 | 41 | Fs::DirListing { dir: playground/dynamic-import } |  |  | 0.391 |
| walker |  | 4097 | 4 | Fs::DirListing { dir: playground/dynamic-import/css } |  |  | 0.391 |
| walker |  | 4104 | 7 | Fs::DirListing { dir: playground/dynamic-import/(app) } |  |  | 0.391 |
| walker |  | 4108 | 4 | Fs::DirListing { dir: playground/dynamic-import/(app)/nest } |  |  | 0.391 |
| ns | 4111 |  | 180 | UserConfig keys: project, sources and transform options | 3.1 |  | 0.382 |
| walker |  | 4118 | 10 | Fs::DirListing { dir: playground/dynamic-import/files } |  |  | 0.382 |
| walker |  | 4130 | 12 | Fs::DirListing { dir: playground/dynamic-import/pkg } |  |  | 0.382 |
| walker |  | 4146 | 16 | Fs::DirListing { dir: playground/dynamic-import/alias } |  |  | 0.382 |
| walker |  | 4163 | 17 | Fs::DirListing { dir: playground/dynamic-import/views } |  |  | 0.382 |
| walker |  | 4206 | 43 | Fs::DirListing { dir: playground/ssr-html } |  |  | 0.382 |
| walker |  | 4216 | 10 | Fs::DirListing { dir: playground/ssr-html/public } |  |  | 0.382 |
| walker |  | 4259 | 43 | Fs::DirListing { dir: playground/wasm } |  |  | 0.382 |
| walker |  | 4282 | 23 | Fs::DirListing { dir: playground/alias/dir } |  |  | 0.382 |
| walker |  | 4290 | 8 | Fs::DirListing { dir: playground/alias/dir/module } |  |  | 0.382 |
| walker |  | 4335 | 45 | Fs::DirListing { dir: playground/hmr-full-bundle-mode } |  |  | 0.382 |
| walker |  | 4380 | 45 | Fs::DirListing { dir: playground/transform-plugin } |  |  | 0.382 |
| ns | 4401 |  | 290 | UserConfig keys: server, build, env, worker and the rest | 3.2 | 3.1 | 0.368 |
| walker |  | 4404 | 24 | Fs::DirListing { dir: playground/ssr-wasm/src } |  |  | 0.368 |
| walker |  | 4452 | 48 | Fs::DirListing { dir: playground/multiple-entrypoints } |  |  | 0.368 |
| walker |  | 4503 | 51 | Fs::DirListing { dir: playground/preload } |  |  | 0.368 |
| walker |  | 4508 | 5 | Fs::DirListing { dir: playground/preload/public } |  |  | 0.368 |
| walker |  | 4516 | 8 | Fs::DirListing { dir: playground/preload/dep-a } |  |  | 0.368 |
| walker |  | 4524 | 8 | Fs::DirListing { dir: playground/preload/dep-including-a } |  |  | 0.368 |
| walker |  | 4545 | 21 | Fs::DirListing { dir: playground/preload/src } |  |  | 0.368 |
| ns | 4571 |  | 170 | Per-environment options (SharedEnvironmentOptions / EnvironmentOptions) | 3.3 |  | 0.360 |
| walker |  | 4572 | 27 | Fs::DirListing { dir: playground/ssr-conditions/external } |  |  | 0.360 |
| walker |  | 4599 | 27 | Fs::DirListing { dir: playground/ssr-conditions/no-external } |  |  | 0.360 |
| walker |  | 4652 | 53 | Fs::DirListing { dir: playground/nested-deps } |  |  | 0.360 |
| walker |  | 4660 | 8 | Fs::DirListing { dir: playground/nested-deps/test-package-a } |  |  | 0.360 |
| walker |  | 4668 | 8 | Fs::DirListing { dir: playground/nested-deps/test-package-f } |  |  | 0.360 |
| walker |  | 4679 | 11 | Fs::DirListing { dir: playground/nested-deps/self-referencing } |  |  | 0.360 |
| walker |  | 4691 | 12 | Fs::DirListing { dir: playground/nested-deps/test-package-b } |  |  | 0.360 |
| walker |  | 4706 | 15 | Fs::DirListing { dir: playground/nested-deps/test-package-d } |  |  | 0.360 |
| ns | 4707 |  | 136 | CommonServerOptions: the host/port/https/proxy/cors keys | 3.4 |  | 0.355 |
| walker |  | 4714 | 8 | Fs::DirListing { dir: playground/nested-deps/test-package-d/test-package-d-nested } |  |  | 0.355 |
| walker |  | 4731 | 17 | Fs::DirListing { dir: playground/nested-deps/test-package-c } |  |  | 0.355 |
| walker |  | 4753 | 22 | Fs::DirListing { dir: playground/nested-deps/test-package-e } |  |  | 0.355 |
| walker |  | 4761 | 8 | Fs::DirListing { dir: playground/nested-deps/test-package-e/test-package-e-excluded } |  |  | 0.355 |
| walker |  | 4769 | 8 | Fs::DirListing { dir: playground/nested-deps/test-package-e/test-package-e-included } |  |  | 0.355 |
| walker |  | 4786 | 17 | Fs::DirListing { dir: playground/backend-integration/frontend/styles } |  |  | 0.355 |
| walker |  | 4816 | 30 | Fs::DirListing { dir: playground/ssr/src } |  |  | 0.355 |
| walker |  | 4828 | 12 | Fs::DirListing { dir: playground/ssr/src/circular-import } |  |  | 0.355 |
| walker |  | 4840 | 12 | Fs::DirListing { dir: playground/ssr/src/circular-import2 } |  |  | 0.355 |
| walker |  | 4883 | 43 | Json::Identity { file: playground/css-lightningcss-root/package.json } |  |  | 0.355 |
| walker |  | 4915 | 32 | Fs::DirListing { dir: playground/dynamic-import/nested } |  |  | 0.355 |
| walker |  | 4919 | 4 | Fs::DirListing { dir: playground/dynamic-import/nested/nested } |  |  | 0.355 |
| walker |  | 4929 | 10 | Fs::DirListing { dir: playground/dynamic-import/nested/treeshaken } |  |  | 0.355 |
| ns | 4944 |  | 237 | ServerOptions and FileSystemServeOptions keys | 3.5 |  | 0.346 |
| walker |  | 4961 | 32 | Fs::DirListing { dir: playground/ssr-html/src } |  |  | 0.346 |
| walker |  | 5023 | 62 | Fs::DirListing { dir: playground/data-uri } |  |  | 0.346 |
| walker |  | 5067 | 44 | Json::Identity { file: playground/css-lightningcss-proxy/package.json } |  |  | 0.346 |
| walker |  | 5088 | 21 | Fs::DirListing { dir: playground/ssr/src/circular-dep-init } |  |  | 0.346 |
| walker |  | 5118 | 30 | Json::Identity { file: playground/nested-deps/self-referencing/package.json } |  |  | 0.346 |
| walker |  | 5172 | 54 | Json::Identity { file: playground/env/package.json } |  |  | 0.346 |
| ns | 5180 |  | 236 | BuildEnvironmentOptions keys: output, assets, CSS, minification | 3.6 |  | 0.339 |
| walker |  | 5226 | 54 | Json::Identity { file: playground/json/package.json } |  |  | 0.339 |
| walker |  | 5281 | 55 | Json::Identity { file: playground/alias/package.json } |  |  | 0.339 |
| walker |  | 5336 | 55 | Json::Identity { file: playground/build-old/package.json } |  |  | 0.339 |
| walker |  | 5391 | 55 | Json::Identity { file: playground/cli/package.json } |  |  | 0.339 |
| ns | 5437 |  | 257 | BuildEnvironmentOptions keys: bundler passthrough, lib, ssr, reporting | 3.7 | 3.6 | 0.331 |
| walker |  | 5446 | 55 | Json::Identity { file: playground/cli-module/package.json } |  |  | 0.331 |
| walker |  | 5501 | 55 | Json::Identity { file: playground/csp/package.json } |  |  | 0.331 |
| ns | 5530 |  | 93 | ExperimentalOptions and FutureOptions | 3.8 |  | 0.328 |
| walker |  | 5556 | 55 | Json::Identity { file: playground/data-uri/package.json } |  |  | 0.328 |
| walker |  | 5611 | 55 | Json::Identity { file: playground/define/package.json } |  |  | 0.328 |
| walker |  | 5666 | 55 | Json::Identity { file: playground/devtools/package.json } |  |  | 0.328 |
| walker |  | 5721 | 55 | Json::Identity { file: playground/extensions/package.json } |  |  | 0.328 |
| walker |  | 5776 | 55 | Json::Identity { file: playground/external/package.json } |  |  | 0.328 |
| ns | 5804 |  | 274 | CLI global options (all five commands) | 3.9 | 2.2 | 0.322 |
| walker |  | 5831 | 55 | Json::Identity { file: playground/forward-console/package.json } |  |  | 0.322 |
| walker |  | 5886 | 55 | Json::Identity { file: playground/minify/package.json } |  |  | 0.322 |
| walker |  | 5941 | 55 | Json::Identity { file: playground/module-graph/package.json } |  |  | 0.322 |
| ns | 5995 |  | 191 | CLI dev-server flags | 3.10 | 2.2 | 0.317 |
| walker |  | 5996 | 55 | Json::Identity { file: playground/object-hooks/package.json } |  |  | 0.317 |
| walker |  | 6051 | 55 | Json::Identity { file: playground/preload/package.json } |  |  | 0.317 |
| walker |  | 6106 | 55 | Json::Identity { file: playground/tailwind/package.json } |  |  | 0.317 |
| walker |  | 6161 | 55 | Json::Identity { file: playground/transform-plugin/package.json } |  |  | 0.317 |
| walker |  | 6216 | 55 | Json::Identity { file: playground/wasm/package.json } |  |  | 0.317 |
| walker |  | 6272 | 56 | Json::Identity { file: playground/assets-sanitize/package.json } |  |  | 0.317 |
| walker |  | 6328 | 56 | Json::Identity { file: playground/base-conflict/package.json } |  |  | 0.317 |
| walker |  | 6384 | 56 | Json::Identity { file: playground/client-reload/package.json } |  |  | 0.317 |
| walker |  | 6440 | 56 | Json::Identity { file: playground/dynamic-import/package.json } |  |  | 0.317 |
| ns | 6467 |  | 472 | CLI build flags | 3.11 | 2.2 | 0.305 |
| walker |  | 6496 | 56 | Json::Identity { file: playground/env-nested/package.json } |  |  | 0.305 |
| walker |  | 6552 | 56 | Json::Identity { file: playground/resolve-linked/package.json } |  |  | 0.305 |
| walker |  | 6608 | 56 | Json::Identity { file: playground/ssr/package.json } |  |  | 0.305 |
| ns | 6622 |  | 155 | CLI optimize and preview flags | 3.12 | 2.2 | 0.302 |
| walker |  | 6664 | 56 | Json::Identity { file: playground/tsconfig-json/package.json } |  |  | 0.302 |
| walker |  | 6697 | 33 | Json::Identity { file: playground/json/dep-json-require/package.json } |  |  | 0.302 |
| walker |  | 6754 | 57 | Json::Identity { file: playground/backend-integration/package.json } |  |  | 0.302 |
| ns | 6757 |  | 135 | src/node module roster (complete) | 4.1 |  | 0.355 |
| walker |  | 6811 | 57 | Json::Identity { file: playground/css-dynamic-import/package.json } |  |  | 0.355 |
| walker |  | 6868 | 57 | Json::Identity { file: playground/dynamic-import-inline/package.json } |  |  | 0.355 |
| ns | 6906 |  | 149 | src/node/plugins roster (complete) | 4.2 |  | 0.345 |
| walker |  | 6925 | 57 | Json::Identity { file: playground/fs-serve/package.json } |  |  | 0.345 |
| ns | 6981 |  | 75 | src/node/server roster (complete) | 4.3 |  | 0.339 |
| walker |  | 6982 | 57 | Json::Identity { file: playground/import-assertion/package.json } |  |  | 0.339 |
| walker |  | 7039 | 57 | Json::Identity { file: playground/multiple-entrypoints/package.json } |  |  | 0.339 |
| ns | 7066 |  | 85 | Dev-server middlewares and per-environment implementations | 4.4 |  | 0.334 |
| walker |  | 7096 | 57 | Json::Identity { file: playground/nested-deps/package.json } |  |  | 0.334 |
| ns | 7144 |  | 78 | SSR and dependency-optimizer rosters | 4.5 |  | 0.333 |
| walker |  | 7153 | 57 | Json::Identity { file: playground/proxy-bypass/package.json } |  |  | 0.333 |
| walker |  | 7210 | 57 | Json::Identity { file: playground/proxy-hmr/package.json } |  |  | 0.333 |
| walker |  | 7267 | 57 | Json::Identity { file: playground/ssr-alias/package.json } |  |  | 0.333 |
| ns | 7287 |  | 143 | Browser client, module-runner and shared rosters | 4.6 |  | 0.343 |
| walker |  | 7324 | 57 | Json::Identity { file: playground/ssr-html/package.json } |  |  | 0.343 |
| walker |  | 7381 | 57 | Json::Identity { file: playground/tailwind-v3/package.json } |  |  | 0.343 |
| ns | 7400 |  | 113 | resolvePlugins: the built-in plugin pipeline order | 4.7 |  | 0.339 |
| walker |  | 7439 | 58 | Json::Identity { file: playground/css-no-codesplit/package.json } |  |  | 0.339 |
| ns | 7494 |  | 94 | Published and inlined type declarations | 4.8 |  | 0.360 |
| walker |  | 7497 | 58 | Json::Identity { file: playground/ssr-conditions/package.json } |  |  | 0.360 |
| walker |  | 7555 | 58 | Json::Identity { file: playground/ssr-noexternal/package.json } |  |  | 0.360 |
| ns | 7609 |  | 115 | Core internal entry-point signatures | 4.9 |  | 0.356 |
| walker |  | 7613 | 58 | Json::Identity { file: playground/ssr-pug/package.json } |  |  | 0.356 |
| walker |  | 7671 | 58 | Json::Identity { file: playground/ssr-wasm/package.json } |  |  | 0.356 |
| walker |  | 7729 | 58 | Json::Identity { file: playground/ssr-webworker/package.json } |  |  | 0.356 |
| walker |  | 7787 | 58 | Json::Identity { file: playground/tailwind-sourcemap/package.json } |  |  | 0.356 |
| walker |  | 7845 | 58 | Json::Identity { file: playground/tsconfig-json-load-error/package.json } |  |  | 0.356 |
| walker |  | 7879 | 34 | Json::Identity { file: playground/json/json-module/package.json } |  |  | 0.356 |
| ns | 7919 |  | 310 | CONTRIBUTING.md: every section heading | 5.1 |  | 0.349 |
| walker |  | 7938 | 59 | Json::Identity { file: playground/environment-react-ssr/package.json } |  |  | 0.349 |
| walker |  | 7997 | 59 | Json::Identity { file: playground/hmr-full-bundle-mode/package.json } |  |  | 0.349 |
| walker |  | 8056 | 59 | Json::Identity { file: playground/optimize-missing-deps/package.json } |  |  | 0.349 |
| ns | 8109 |  | 190 | CONTRIBUTING: local development loop | 5.2 | 5.1 | 0.346 |
| walker |  | 8115 | 59 | Json::Identity { file: playground/preserve-symlinks/package.json } |  |  | 0.346 |
| walker |  | 8174 | 59 | Json::Identity { file: playground/resolve-tsconfig-paths/package.json } |  |  | 0.346 |
| walker |  | 8233 | 59 | Json::Identity { file: playground/ssr-resolve/package.json } |  |  | 0.346 |
| walker |  | 8293 | 60 | Json::Identity { file: playground/css-codesplit-cjs/package.json } |  |  | 0.346 |
| walker |  | 8353 | 60 | Json::Identity { file: playground/optimize-deps-no-discovery/package.json } |  |  | 0.346 |
| walker |  | 8440 | 87 | Fs::DirListing { dir: playground/glob-import } |  |  | 0.346 |
| walker |  | 8444 | 4 | Fs::DirListing { dir: playground/glob-import/pkg-pages } |  |  | 0.346 |
| walker |  | 8448 | 4 | Fs::DirListing { dir: playground/glob-import/subpath-imports-sub } |  |  | 0.346 |
| walker |  | 8454 | 6 | Fs::DirListing { dir: playground/glob-import/follow-symlinks } |  |  | 0.346 |
| walker |  | 8458 | 4 | Fs::DirListing { dir: playground/glob-import/follow-symlinks/packages } |  |  | 0.346 |
| walker |  | 8466 | 8 | Fs::DirListing { dir: playground/glob-import/array-common-base } |  |  | 0.346 |
| walker |  | 8470 | 4 | Fs::DirListing { dir: playground/glob-import/array-common-base/pattern1 } |  |  | 0.346 |
| walker |  | 8474 | 4 | Fs::DirListing { dir: playground/glob-import/array-common-base/pattern2 } |  |  | 0.346 |
| walker |  | 8482 | 8 | Fs::DirListing { dir: playground/glob-import/array-test-dir } |  |  | 0.346 |
| walker |  | 8490 | 8 | Fs::DirListing { dir: playground/glob-import/import-meta-glob-pkg } |  |  | 0.346 |
| walker |  | 8498 | 8 | Fs::DirListing { dir: playground/glob-import/imports-path } |  |  | 0.346 |
| walker |  | 8501 | 3 | Fs::DirListing { dir: playground/glob-import/follow-symlinks/packages/my-lib } |  |  | 0.346 |
| walker |  | 8512 | 11 | Fs::DirListing { dir: playground/glob-import/side-effect } |  |  | 0.346 |
| ns | 8515 |  | 406 | playground/ roster (complete e2e corpus) | 5.3 |  | 0.418 |
| walker |  | 8519 | 7 | Fs::DirListing { dir: playground/glob-import/follow-symlinks/linked } |  |  | 0.418 |
| walker |  | 8537 | 18 | Fs::DirListing { dir: playground/glob-import/escape } |  |  | 0.418 |
| walker |  | 8544 | 7 | Fs::DirListing { dir: playground/glob-import/escape/(parenthesis) } |  |  | 0.418 |
| walker |  | 8548 | 4 | Fs::DirListing { dir: playground/glob-import/escape/(parenthesis)/mod } |  |  | 0.418 |
| walker |  | 8555 | 7 | Fs::DirListing { dir: playground/glob-import/escape/[brackets] } |  |  | 0.418 |
| walker |  | 8559 | 4 | Fs::DirListing { dir: playground/glob-import/escape/[brackets]/mod } |  |  | 0.418 |
| walker |  | 8566 | 7 | Fs::DirListing { dir: playground/glob-import/escape/{curlies} } |  |  | 0.418 |
| walker |  | 8570 | 4 | Fs::DirListing { dir: playground/glob-import/escape/{curlies}/mod } |  |  | 0.418 |
| walker |  | 8578 | 8 | Fs::DirListing { dir: playground/glob-import/follow-symlinks/packages/my-lib/components } |  |  | 0.418 |
| walker |  | 8610 | 32 | Fs::DirListing { dir: playground/glob-import/dir } |  |  | 0.418 |
| walker |  | 8614 | 4 | Fs::DirListing { dir: playground/glob-import/dir/nested } |  |  | 0.418 |
| walker |  | 8669 | 55 | Json::Identity { file: playground/glob-import/package.json } |  |  | 0.418 |
| walker |  | 8703 | 34 | Json::Identity { file: playground/glob-import/import-meta-glob-pkg/package.json } |  |  | 0.418 |
| walker |  | 8716 | 13 | Json::Entry { file: playground/resolve-linked/package.json } |  |  | 0.418 |
| walker |  | 8763 | 47 | Fs::DirListing { dir: playground/fs-serve/root } |  |  | 0.418 |
| ns | 8802 |  | 287 | CONTRIBUTING: how the integration tests work | 5.4 | 5.3 | 0.417 |
| walker |  | 8857 | 94 | Fs::DirListing { dir: playground/css-lightningcss } |  |  | 0.417 |
| walker |  | 8865 | 8 | Fs::DirListing { dir: playground/css-lightningcss/nested } |  |  | 0.417 |
| walker |  | 8907 | 42 | Json::Identity { file: playground/css-lightningcss/package.json } |  |  | 0.417 |
| ns | 8919 |  | 117 | Unit test locations under packages/vite/src | 5.5 |  | 0.410 |
| walker |  | 8920 | 13 | Fs::DirListing { dir: playground/transform-plugin/__tests__ } |  |  | 0.410 |
| walker |  | 9016 | 96 | Fs::DirListing { dir: playground/assets } |  |  | 0.410 |
| walker |  | 9025 | 9 | Fs::DirListing { dir: playground/assets/import-meta-url } |  |  | 0.410 |
| walker |  | 9040 | 15 | Fs::DirListing { dir: playground/assets/fonts } |  |  | 0.410 |
| ns | 9050 |  | 131 | How the unit and e2e vitest runs are separated | 5.6 |  | 0.408 |
| walker |  | 9057 | 17 | Fs::DirListing { dir: playground/assets/asset } |  |  | 0.408 |
| walker |  | 9094 | 37 | Fs::DirListing { dir: playground/assets/css } |  |  | 0.408 |
| walker |  | 9102 | 8 | Fs::DirListing { dir: playground/assets/css/nested } |  |  | 0.408 |
| ns | 9114 |  | 64 | docs/ site roster | 5.7 |  | 0.420 |
| walker |  | 9156 | 54 | Json::Identity { file: playground/assets/package.json } |  |  | 0.420 |
| walker |  | 9197 | 41 | Fs::DirListing { dir: playground/assets/static } |  |  | 0.420 |
| walker |  | 9225 | 28 | Json::Scripts { file: playground/package.json } |  |  | 0.420 |
| ns | 9298 |  | 184 | docs/guide and docs/config page rosters | 5.8 |  | 0.411 |
| walker |  | 9324 | 99 | Fs::DirListing { dir: playground/lib } |  |  | 0.411 |
| ns | 9343 |  | 45 | docs/changes: the breaking-change / migration notes | 5.9 |  | 0.409 |
| walker |  | 9379 | 55 | Json::Identity { file: playground/lib/package.json } |  |  | 0.409 |
| walker |  | 9393 | 14 | Fs::DirListing { dir: playground/preload/__tests__ } |  |  | 0.409 |
| walker |  | 9425 | 32 | Fs::DirListing { dir: playground/backend-integration/frontend/entrypoints } |  |  | 0.409 |
| walker |  | 9429 | 4 | Fs::DirListing { dir: playground/backend-integration/frontend/entrypoints/public } |  |  | 0.409 |
| walker |  | 9437 | 8 | Fs::DirListing { dir: playground/backend-integration/frontend/entrypoints/nested } |  |  | 0.409 |
| walker |  | 9481 | 44 | Json::Identity { file: playground/nested-deps/test-package-e/package.json } |  |  | 0.409 |
| ns | 9501 |  | 158 | CONTRIBUTING: the dependency policy | 5.10 | 5.1 | 0.408 |
| walker |  | 9525 | 44 | Json::Identity { file: playground/preserve-symlinks/module-a/package.json } |  |  | 0.408 |
| walker |  | 9569 | 44 | Json::Identity { file: playground/ssr-resolve/entries/package.json } |  |  | 0.408 |
| walker |  | 9614 | 45 | Json::Identity { file: playground/ssr-webworker/browser-exports/package.json } |  |  | 0.408 |
| ns | 9624 |  | 123 | create-vite: package layout and template roster | 5.11 |  | 0.423 |
| walker |  | 9659 | 45 | Json::Identity { file: playground/ssr-webworker/worker-exports/package.json } |  |  | 0.423 |
| ns | 9677 |  | 53 | plugin-legacy: package layout | 5.12 |  | 0.430 |
| walker |  | 9705 | 46 | Json::Identity { file: playground/ssr-noexternal/external-cjs/package.json } |  |  | 0.430 |
| walker |  | 9817 | 112 | Fs::DirListing { dir: playground/css-sourcemap } |  |  | 0.430 |
| ns | 9827 |  | 150 | Repository automation: .github and release scripts | 5.13 |  | 0.445 |
| walker |  | 9874 | 57 | Json::Identity { file: playground/css-sourcemap/package.json } |  |  | 0.445 |
| walker |  | 9921 | 47 | Json::Identity { file: playground/external/dep-that-imports/package.json } |  |  | 0.445 |
| walker |  | 9968 | 47 | Json::Identity { file: playground/external/dep-that-requires/package.json } |  |  | 0.445 |
| walker |  | 10000 | 32 | Json::Identity { file: playground/optimize-deps-no-discovery/dep-no-discovery/package.json } |  |  | 0.445 |
