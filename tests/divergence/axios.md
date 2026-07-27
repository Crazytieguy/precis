Score(3000)=0.665 I=0.852 C=0.519 ns_rows≤3K=21/40 (reached=13 partial=1 missing=7)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 29 |  | 29 | lib/ top-level listing | 1.1 |  | 0.000 |
| ns | 47 |  | 18 | Root directories | 1.2 |  | 0.000 |
| ns | 59 |  | 12 | Root entry points and type declarations | 1.3 |  | 0.000 |
| ns | 117 |  | 58 | Root config/build/tooling files | 1.4 |  | 0.000 |
| walker |  | 153 | 153 | listing of '.' |  |  | 0.536 |
| walker |  | 163 | 10 | export names surface in index.js |  |  | 0.536 |
| walker |  | 175 | 12 | listing of 'sandbox' |  |  | 0.536 |
| ns | 182 |  | 65 | Root documentation files | 1.5 |  | 0.542 |
| walker |  | 204 | 29 | listing of 'lib' |  |  | 1.000 |
| walker |  | 212 | 8 | listing of 'lib/defaults' |  |  | 1.000 |
| walker |  | 223 | 11 | listing of 'lib/env' |  |  | 1.000 |
| walker |  | 236 | 13 | listing of 'lib/platform' |  |  | 1.000 |
| walker |  | 243 | 7 | listing of 'lib/platform/browser' |  |  | 1.000 |
| walker |  | 250 | 7 | listing of 'lib/platform/node' |  |  | 1.000 |
| ns | 255 |  | 73 | lib/env, lib/platform listings (incl. node/browser/common subdirs) | 1.6 |  | 0.891 |
| walker |  | 265 | 15 | listing of 'lib/platform/browser/classes' |  |  | 0.932 |
| walker |  | 281 | 16 | listing of 'lib/cancel' |  |  | 0.933 |
| walker |  | 285 | 4 | listing of 'lib/platform/common' |  |  | 0.948 |
| walker |  | 305 | 20 | listing of 'lib/adapters' |  |  | 0.953 |
| walker |  | 310 | 5 | listing of 'lib/env/classes' |  |  | 0.970 |
| walker |  | 314 | 4 | listing of '.husky' |  |  | 0.970 |
| ns | 348 |  | 93 | lib/core, lib/adapters, lib/cancel, lib/defaults listings | 1.7 |  | 0.859 |
| walker |  | 390 | 76 | package identity in package.json |  |  | 0.859 |
| walker |  | 396 | 6 | listing of 'scripts' |  |  | 0.859 |
| walker |  | 445 | 49 | listing of 'lib/core' |  |  | 0.971 |
| ns | 447 |  | 99 | lib/helpers listing, part 1 (A-N) | 1.8 |  | 0.871 |
| walker |  | 455 | 10 | export names surface in lib/axios.js |  |  | 0.871 |
| ns | 538 |  | 91 | lib/helpers listing, part 2 (N-Z) | 1.9 |  | 0.795 |
| walker |  | 645 | 190 | listing of 'lib/helpers' |  |  | 0.978 |
| walker |  | 656 | 11 | listing of 'lib/platform/node/classes' |  |  | 1.000 |
| walker |  | 666 | 10 | export names surface in lib/defaults/index.js |  |  | 1.000 |
| walker |  | 677 | 11 | export names surface in lib/platform/index.js |  |  | 1.000 |
| walker |  | 696 | 19 | export at lib/platform/index.js:4 |  |  | 1.000 |
| walker |  | 709 | 13 | imports in index.js |  |  | 1.000 |
| walker |  | 728 | 19 | listing of 'tests' |  |  | 1.000 |
| walker |  | 735 | 7 | listing of 'tests/module' |  |  | 1.000 |
| ns | 743 |  | 205 | tests/ top-level + unit top-level + unit/adapters,core,cancel listings | 1.10 |  | 0.844 |
| walker |  | 776 | 41 | README headline in lib/env/README.md |  |  | 0.844 |
| walker |  | 923 | 147 | export at index.js:26 |  |  | 0.845 |
| walker |  | 972 | 49 | README headline in lib/adapters/README.md |  |  | 0.846 |
| walker |  | 983 | 11 | export names surface in lib/platform/browser/index.js |  |  | 0.846 |
| walker |  | 994 | 11 | export names surface in lib/platform/node/index.js |  |  | 0.846 |
| ns | 1013 |  | 270 | unit/helpers, unit/utils, browser, setup listings | 1.11 |  | 0.735 |
| walker |  | 1036 | 42 | headings outline in CONTRIBUTORS.md |  |  | 0.735 |
| walker |  | 1056 | 20 | CONTRIBUTORS.md section #0 |  |  | 0.735 |
| walker |  | 1068 | 12 | listing of 'tests/module/cjs' |  |  | 0.736 |
| walker |  | 1082 | 14 | listing of 'tests/smoke' |  |  | 0.736 |
| walker |  | 1093 | 11 | listing of 'tests/smoke/bun' |  |  | 0.736 |
| walker |  | 1105 | 12 | listing of 'tests/smoke/cjs' |  |  | 0.736 |
| walker |  | 1118 | 13 | listing of 'tests/smoke/deno' |  |  | 0.737 |
| walker |  | 1190 | 72 | README headline in lib/core/README.md |  |  | 0.737 |
| walker |  | 1263 | 73 | README headline in lib/helpers/README.md |  |  | 0.738 |
| walker |  | 1287 | 24 | lib/helpers/README.md section #0 |  |  | 0.738 |
| walker |  | 1304 | 17 | export names surface in gulpfile.js |  |  | 0.738 |
| ns | 1305 |  | 292 | tests/smoke listings, part 1 (esm, cjs) | 1.12 |  | 0.663 |
| walker |  | 1341 | 37 | listing of '.github' |  |  | 0.664 |
| walker |  | 1381 | 40 | listing of '.github/workflows' |  |  | 0.666 |
| walker |  | 1432 | 51 | listing of 'docs' |  |  | 0.669 |
| ns | 1436 |  | 131 | tests/smoke listings, part 2 (bun, deno) | 1.13 |  | 0.642 |
| walker |  | 1439 | 7 | listing of 'docs/es' |  |  | 0.642 |
| walker |  | 1446 | 7 | listing of 'docs/fr' |  |  | 0.642 |
| walker |  | 1453 | 7 | listing of 'docs/zh' |  |  | 0.642 |
| walker |  | 1464 | 11 | listing of 'docs/es/pages' |  |  | 0.642 |
| walker |  | 1475 | 11 | listing of 'docs/fr/pages' |  |  | 0.642 |
| walker |  | 1486 | 11 | listing of 'docs/pages' |  |  | 0.643 |
| walker |  | 1497 | 11 | listing of 'docs/zh/pages' |  |  | 0.643 |
| walker |  | 1510 | 13 | listing of 'docs/es/pages/misc' |  |  | 0.643 |
| walker |  | 1523 | 13 | listing of 'docs/fr/pages/misc' |  |  | 0.643 |
| ns | 1534 |  | 98 | tests/module listings (cjs, esm) | 1.14 |  | 0.623 |
| walker |  | 1536 | 13 | listing of 'docs/pages/misc' |  |  | 0.623 |
| walker |  | 1549 | 13 | listing of 'docs/zh/pages/misc' |  |  | 0.623 |
| walker |  | 1567 | 18 | listing of 'docs/es/pages/getting-started' |  |  | 0.623 |
| walker |  | 1585 | 18 | listing of 'docs/fr/pages/getting-started' |  |  | 0.623 |
| ns | 1596 |  | 62 | docs/ top-level + docs/pages listing | 1.15 |  | 0.642 |
| walker |  | 1603 | 18 | listing of 'docs/pages/getting-started' |  |  | 0.642 |
| walker |  | 1621 | 18 | listing of 'docs/zh/pages/getting-started' |  |  | 0.642 |
| walker |  | 1639 | 18 | listing of 'tests/module/esm' |  |  | 0.650 |
| walker |  | 1657 | 18 | listing of 'tests/smoke/esm' |  |  | 0.654 |
| walker |  | 1676 | 19 | CONTRIBUTORS.md section #2 |  |  | 0.654 |
| walker |  | 1726 | 50 | listing of 'examples' |  |  | 0.655 |
| ns | 1774 |  | 178 | docs/pages/advanced, getting-started, misc listings | 1.16 |  | 0.619 |
| walker |  | 1799 | 73 | headings outline in ECOSYSTEM.md |  |  | 0.619 |
| walker |  | 1835 | 36 | ECOSYSTEM.md section #0 |  |  | 0.619 |
| ns | 1876 |  | 102 | examples/ listings (all subdirectories) | 1.17 |  | 0.605 |
| walker |  | 1882 | 47 | lib/core/README.md section #0 |  |  | 0.606 |
| walker |  | 1961 | 79 | headings outline in COLLABORATOR_GUIDE.md |  |  | 0.606 |
| walker |  | 1972 | 11 | export names surface in lib/utils.js |  |  | 0.606 |
| ns | 1975 |  | 99 | sandbox/, scripts/, .github/, .husky/ listings | 1.18 |  | 0.624 |
| walker |  | 1995 | 23 | listing of 'tests/module/esm/tests' |  |  | 0.634 |
| walker |  | 2014 | 19 | listing of 'tests/module/esm/tests/helpers' |  |  | 0.634 |
| walker |  | 2083 | 69 | COLLABORATOR_GUIDE.md section #0 |  |  | 0.634 |
| walker |  | 2101 | 18 | module item at lib/defaults/index.js:23 |  |  | 0.634 |
| walker |  | 2136 | 35 | listing of 'tests/smoke/deno/tests' |  |  | 0.642 |
| walker |  | 2220 | 84 | json config tsconfig.json |  |  | 0.642 |
| walker |  | 2258 | 38 | listing of 'tests/module/cjs/tests' |  |  | 0.659 |
| walker |  | 2289 | 31 | listing of 'tests/module/cjs/tests/helpers' |  |  | 0.659 |
| walker |  | 2299 | 10 | export names surface in lib/core/Axios.js |  |  | 0.659 |
| walker |  | 2309 | 10 | export names surface in lib/helpers/null.js |  |  | 0.659 |
| walker |  | 2319 | 10 | export names surface in lib/helpers/throttle.js |  |  | 0.659 |
| ns | 2351 |  | 376 | index.js — public re-export surface | 2.1 |  | 0.631 |
| walker |  | 2395 | 76 | export at lib/platform/browser/index.js:5 |  |  | 0.631 |
| walker |  | 2406 | 11 | export names surface in lib/adapters/adapters.js |  |  | 0.631 |
| walker |  | 2417 | 11 | export names surface in lib/cancel/CancelToken.js |  |  | 0.631 |
| walker |  | 2428 | 11 | export names surface in lib/core/AxiosError.js |  |  | 0.631 |
| walker |  | 2439 | 11 | export names surface in lib/core/AxiosHeaders.js |  |  | 0.631 |
| walker |  | 2450 | 11 | export names surface in lib/helpers/callbackify.js |  |  | 0.631 |
| walker |  | 2461 | 11 | export names surface in lib/helpers/composeSignals.js |  |  | 0.631 |
| walker |  | 2472 | 11 | export names surface in lib/helpers/readBlob.js |  |  | 0.631 |
| walker |  | 2483 | 11 | export names surface in lib/helpers/speedometer.js |  |  | 0.631 |
| walker |  | 2494 | 11 | export names surface in lib/helpers/validator.js |  |  | 0.631 |
| walker |  | 2512 | 18 | export at lib/helpers/validator.js:109 |  |  | 0.631 |
| walker |  | 2661 | 149 | module item at index.js:6 |  |  | 0.667 |
| ns | 2667 |  | 316 | lib/ subdirectory README notes (core, adapters, env, helpers) | 2.2 |  | 0.675 |
| walker |  | 2673 | 12 | export names surface in lib/cancel/CanceledError.js |  |  | 0.675 |
| walker |  | 2685 | 12 | export names surface in lib/core/InterceptorManager.js |  |  | 0.675 |
| walker |  | 2697 | 12 | export names surface in lib/helpers/AxiosTransformStream.js |  |  | 0.675 |
| walker |  | 2709 | 12 | export names surface in lib/helpers/HttpStatusCode.js |  |  | 0.675 |
| walker |  | 2721 | 12 | export names surface in lib/helpers/toFormData.js |  |  | 0.675 |
| walker |  | 2761 | 40 | COLLABORATOR_GUIDE.md section #1 |  |  | 0.675 |
| walker |  | 2774 | 13 | module item at lib/platform/node/index.js:5 |  |  | 0.675 |
| walker |  | 2787 | 13 | export names surface in lib/helpers/AxiosURLSearchParams.js |  |  | 0.675 |
| walker |  | 2800 | 13 | export names surface in lib/helpers/formDataToJSON.js |  |  | 0.675 |
| walker |  | 2813 | 13 | export names surface in lib/helpers/formDataToStream.js |  |  | 0.675 |
| ns | 2858 |  | 191 | package.json — browser/react-native alias map | 2.3 |  | 0.665 |
| walker |  | 2913 | 100 | export at lib/platform/node/index.js:27 |  |  | 0.665 |
| walker |  | 2927 | 14 | module item at lib/platform/node/index.js:7 |  |  | 0.665 |
| walker |  | 2941 | 14 | export names surface in lib/helpers/ZlibHeaderTransformStream.js |  |  | 0.665 |
| walker |  | 3046 | 105 | listing of 'tests/unit' |  |  | 0.694 |
| walker |  | 3079 | 33 | listing of 'tests/unit/adapters' |  |  | 0.712 |
| walker |  | 3115 | 36 | listing of 'tests/unit/core' |  |  | 0.730 |
| walker |  | 3119 | 4 | listing of 'docs/data' |  |  | 0.730 |
| walker |  | 3123 | 4 | listing of 'examples/all' |  |  | 0.732 |
| walker |  | 3127 | 4 | listing of 'examples/amd' |  |  | 0.733 |
| walker |  | 3131 | 4 | listing of 'examples/transform-response' |  |  | 0.735 |
| walker |  | 3190 | 59 | listing of 'tests/unit/utils' |  |  | 0.739 |
| ns | 3308 |  | 450 | index.d.ts — AxiosRequestConfig interface, part 1 (url..maxRate) | 2.4 |  | 0.708 |
| walker |  | 3327 | 137 | listing of 'docs/es/pages/advanced' |  |  | 0.708 |
| walker |  | 3464 | 137 | listing of 'docs/fr/pages/advanced' |  |  | 0.708 |
| walker |  | 3601 | 137 | listing of 'docs/pages/advanced' |  |  | 0.748 |
| walker |  | 3738 | 137 | listing of 'docs/zh/pages/advanced' |  |  | 0.748 |
| walker |  | 3753 | 15 | export names surface in lib/adapters/xhr.js |  |  | 0.748 |
| walker |  | 3768 | 15 | export names surface in lib/env/data.js |  |  | 0.748 |
| walker |  | 3783 | 15 | export names surface in lib/helpers/resolveConfig.js |  |  | 0.748 |
| walker |  | 3783 | 0 | export at lib/helpers/resolveConfig.js:38 |  |  | 0.748 |
| walker |  | 3798 | 15 | export names surface in lib/helpers/spread.js |  |  | 0.748 |
| walker |  | 3798 | 0 | export at lib/helpers/spread.js:24 |  |  | 0.748 |
| walker |  | 3814 | 16 | export names surface in lib/cancel/isCancel.js |  |  | 0.748 |
| walker |  | 3814 | 0 | export at lib/cancel/isCancel.js:3 |  |  | 0.748 |
| walker |  | 3829 | 15 | export body at lib/cancel/isCancel.js:3 body 4 |  |  | 0.749 |
| walker |  | 3845 | 16 | export names surface in lib/core/dispatchRequest.js |  |  | 0.749 |
| walker |  | 3845 | 0 | export at lib/core/dispatchRequest.js:34 |  |  | 0.749 |
| walker |  | 3861 | 16 | export names surface in lib/helpers/cookies.js |  |  | 0.749 |
| walker |  | 3877 | 16 | export names surface in lib/helpers/isURLSameOrigin.js |  |  | 0.749 |
| walker |  | 3893 | 16 | export names surface in lib/helpers/parseHeaders.js |  |  | 0.749 |
| walker |  | 3893 | 0 | export at lib/helpers/parseHeaders.js:41 |  |  | 0.749 |
| walker |  | 3909 | 16 | export names surface in lib/helpers/parseProtocol.js |  |  | 0.749 |
| walker |  | 3909 | 0 | export at lib/helpers/parseProtocol.js:3 |  |  | 0.749 |
| walker |  | 3926 | 17 | export names surface in lib/helpers/isAbsoluteURL.js |  |  | 0.749 |
| walker |  | 3926 | 0 | export at lib/helpers/isAbsoluteURL.js:10 |  |  | 0.749 |
| walker |  | 3943 | 17 | export names surface in lib/helpers/isAxiosError.js |  |  | 0.749 |
| walker |  | 3943 | 0 | export at lib/helpers/isAxiosError.js:12 |  |  | 0.749 |
| walker |  | 3953 | 10 | export names surface in lib/platform/common/utils.js |  |  | 0.749 |
| ns | 4035 |  | 727 | index.d.ts — AxiosRequestConfig interface, part 2 (beforeRedirect..redact) | 2.5 |  | 0.701 |
| walker |  | 4206 | 253 | headings outline in THREATMODEL.md |  |  | 0.701 |
| walker |  | 4211 | 5 | THREATMODEL.md section #24 |  |  | 0.701 |
| walker |  | 4216 | 5 | THREATMODEL.md section #33 |  |  | 0.701 |
| walker |  | 4221 | 5 | THREATMODEL.md section #44 |  |  | 0.701 |
| walker |  | 4228 | 7 | THREATMODEL.md section #10 |  |  | 0.701 |
| ns | 4234 |  | 199 | lib/core/InterceptorManager.js — constructor, use(), forEach() (docblocks elided) | 3.1 |  | 0.686 |
| walker |  | 4235 | 7 | THREATMODEL.md section #53 |  |  | 0.686 |
| walker |  | 4346 | 111 | THREATMODEL.md section #0 |  |  | 0.686 |
| walker |  | 4398 | 52 | THREATMODEL.md section #25 |  |  | 0.686 |
| walker |  | 4476 | 78 | listing of 'tests/unit/helpers' |  |  | 0.699 |
| walker |  | 4530 | 54 | COLLABORATOR_GUIDE.md section #3 |  |  | 0.699 |
| walker |  | 4654 | 124 | listing of 'tests/browser' |  |  | 0.737 |
| walker |  | 4726 | 72 | listing of 'tests/smoke/bun/tests' |  |  | 0.757 |
| ns | 4729 |  | 495 | lib/core/dispatchRequest.js — part 1 (cancellation check, request transform, adapter call) | 3.2 |  | 0.725 |
| walker |  | 4812 | 86 | package runtime dependencies in package.json |  |  | 0.725 |
| walker |  | 4830 | 18 | export names surface in lib/helpers/bind.js |  |  | 0.725 |
| walker |  | 4830 | 0 | export at lib/helpers/bind.js:10 |  |  | 0.725 |
| walker |  | 4848 | 18 | export names surface in lib/helpers/shouldBypassProxy.js |  |  | 0.725 |
| walker |  | 4848 | 0 | export at lib/helpers/shouldBypassProxy.js:127 |  |  | 0.725 |
| ns | 5137 |  | 408 | lib/core/dispatchRequest.js — part 2 (response/rejection transform) | 3.3 |  | 0.698 |
| walker |  | 5151 | 303 | lib/adapters/README.md section #1 |  |  | 0.699 |
| walker |  | 5170 | 19 | export names surface in lib/core/settle.js |  |  | 0.699 |
| walker |  | 5170 | 0 | export at lib/core/settle.js:14 |  |  | 0.699 |
| walker |  | 5189 | 19 | export names surface in lib/core/transformData.js |  |  | 0.699 |
| walker |  | 5189 | 0 | export at lib/core/transformData.js:15 |  |  | 0.699 |
| walker |  | 5208 | 19 | export names surface in lib/helpers/estimateDataURLDecodedBytes.js |  |  | 0.699 |
| walker |  | 5208 | 0 | export at lib/helpers/estimateDataURLDecodedBytes.js:10 |  |  | 0.699 |
| ns | 6074 |  | 937 | lib/core/mergeConfig.js — part 1 (null-proto result object + per-strategy merge helper fns) | 3.4 |  | 0.655 |
| walker |  | 6236 | 1028 | export names surface in index.d.ts |  |  | 0.656 |
| walker |  | 6236 | 0 | export at index.d.ts:366 |  |  | 0.656 |
| walker |  | 6236 | 0 | export at index.d.ts:461 |  |  | 0.656 |
| walker |  | 6236 | 0 | export at index.d.ts:713 |  |  | 0.656 |
| walker |  | 6496 | 260 | export member names #1 at index.d.ts:366 |  |  | 0.676 |
| walker |  | 6640 | 144 | export member names at index.d.ts:461 |  |  | 0.676 |
| ns | 6710 |  | 636 | lib/core/mergeConfig.js — part 2 (per-key mergeMap table + apply loop) | 3.5 |  | 0.653 |
| walker |  | 6832 | 192 | export member names at index.d.ts:713 |  |  | 0.653 |
| walker |  | 6845 | 13 | export at index.d.ts:476 |  |  | 0.653 |
| walker |  | 6857 | 12 | export at index.d.ts:535 |  |  | 0.653 |
| walker |  | 6871 | 14 | export at index.d.ts:457 |  |  | 0.653 |
| walker |  | 6885 | 14 | export at index.d.ts:545 |  |  | 0.653 |
| walker |  | 6901 | 16 | export at index.d.ts:541 |  |  | 0.653 |
| walker |  | 6920 | 19 | export at index.d.ts:6 |  |  | 0.653 |
| walker |  | 6939 | 19 | export at index.d.ts:154 |  |  | 0.653 |
| walker |  | 6960 | 21 | export at index.d.ts:158 |  |  | 0.653 |
| walker |  | 6982 | 22 | export at index.d.ts:359 |  |  | 0.657 |
| walker |  | 7005 | 23 | export at index.d.ts:480 |  |  | 0.657 |
| walker |  | 7028 | 23 | export at index.d.ts:564 |  |  | 0.657 |
| ns | 7046 |  | 336 | lib/core/AxiosError.js — class signature + error code constants | 4.1 |  | 0.647 |
| walker |  | 7051 | 23 | export at index.d.ts:676 |  |  | 0.647 |
| walker |  | 7075 | 24 | export at index.d.ts:323 |  |  | 0.647 |
| walker |  | 7100 | 25 | export at index.d.ts:549 |  |  | 0.647 |
| walker |  | 7126 | 26 | export at index.d.ts:319 |  |  | 0.647 |
| walker |  | 7152 | 26 | export at index.d.ts:327 |  |  | 0.647 |
| walker |  | 7180 | 28 | export at index.d.ts:141 |  |  | 0.647 |
| walker |  | 7210 | 30 | export at index.d.ts:680 |  |  | 0.647 |
| walker |  | 7234 | 24 | export at index.d.ts:686 |  |  | 0.647 |
| walker |  | 7268 | 34 | export at index.d.ts:558 |  |  | 0.647 |
| walker |  | 7303 | 35 | export at index.d.ts:553 |  |  | 0.647 |
| ns | 7338 |  | 292 | lib/cancel/isCancel.js, CanceledError.js — full | 4.2 |  | 0.635 |
| walker |  | 7339 | 36 | export at index.d.ts:569 |  |  | 0.635 |
| walker |  | 7369 | 30 | export at index.d.ts:274 |  |  | 0.635 |
| walker |  | 7410 | 41 | export at index.d.ts:163 |  |  | 0.632 |
| ns | 7410 |  | 72 | lib/cancel/CancelToken.js — method locations | 4.3 |  | 0.632 |
| walker |  | 7455 | 45 | export at index.d.ts:309 |  |  | 0.632 |
| walker |  | 7502 | 47 | export at index.d.ts:293 |  |  | 0.632 |
| walker |  | 7536 | 34 | export at index.d.ts:706 |  |  | 0.632 |
| walker |  | 7585 | 49 | export at index.d.ts:279 |  |  | 0.632 |
| ns | 7633 |  | 223 | lib/core/AxiosHeaders.js — method locations | 5.1 |  | 0.622 |
| walker |  | 7643 | 58 | export at index.d.ts:145 |  |  | 0.622 |
| walker |  | 7684 | 41 | export at index.d.ts:690 |  |  | 0.622 |
| walker |  | 7747 | 63 | export at index.d.ts:595 |  |  | 0.622 |
| ns | 7780 |  | 147 | lib/adapters/adapters.js, xhr.js, fetch.js, http.js — top-level function/export locations | 5.2 |  | 0.612 |
| walker |  | 7814 | 67 | export at index.d.ts:286 |  |  | 0.612 |
| walker |  | 7870 | 56 | export at index.d.ts:112 |  |  | 0.612 |
| walker |  | 7943 | 73 | export at index.d.ts:484 |  |  | 0.612 |
| ns | 7983 |  | 203 | lib/helpers/* — one-line-per-file export locations, part 1 (serialization + parsing + config) | 5.3 |  | 0.608 |
| walker |  | 8017 | 74 | export at index.d.ts:299 |  |  | 0.608 |
| walker |  | 8036 | 19 | export body at lib/helpers/isAxiosError.js:12 body 13 |  |  | 0.608 |
| walker |  | 8152 | 116 | listing of 'tests/smoke/esm/tests' |  |  | 0.622 |
| walker |  | 8187 | 35 | module item at lib/defaults/index.js:11 |  |  | 0.622 |
| walker |  | 8246 | 59 | COLLABORATOR_GUIDE.md section #5 |  |  | 0.622 |
| walker |  | 8266 | 20 | export names surface in lib/core/mergeConfig.js |  |  | 0.622 |
| walker |  | 8266 | 0 | export at lib/core/mergeConfig.js:17 |  |  | 0.622 |
| walker |  | 8286 | 20 | export names surface in lib/helpers/combineURLs.js |  |  | 0.623 |
| walker |  | 8286 | 0 | export at lib/helpers/combineURLs.js:11 |  |  | 0.623 |
| walker |  | 8306 | 20 | export names surface in lib/helpers/deprecatedMethod.js |  |  | 0.623 |
| walker |  | 8306 | 0 | export at lib/helpers/deprecatedMethod.js:15 |  |  | 0.623 |
| ns | 8329 |  | 346 | lib/helpers/* — one-line-per-file export locations, part 2 (streaming + proxy/origin + data URIs + small utilities) | 5.4 |  | 0.616 |
| walker |  | 8768 | 462 | package scripts in package.json |  |  | 0.616 |
| walker |  | 8789 | 21 | export names surface in lib/helpers/toURLEncodedForm.js |  |  | 0.617 |
| walker |  | 8789 | 0 | export at lib/helpers/toURLEncodedForm.js:7 |  |  | 0.617 |
| ns | 8838 |  | 509 | README.md — Error Types table (per-code descriptions) | 6.1 |  | 0.611 |
| walker |  | 8854 | 65 | export at index.d.ts:251 |  |  | 0.611 |
| walker |  | 8876 | 22 | export names surface in lib/helpers/fromDataURI.js |  |  | 0.612 |
| walker |  | 8876 | 0 | export at lib/helpers/fromDataURI.js:21 |  |  | 0.612 |
| walker |  | 9008 | 132 | listing of 'tests/smoke/cjs/tests' |  |  | 0.637 |
| walker |  | 9032 | 24 | imports in lib/platform/index.js |  |  | 0.637 |
| walker |  | 9057 | 25 | export names surface in lib/core/buildFullPath.js |  |  | 0.637 |
| walker |  | 9057 | 0 | export at lib/core/buildFullPath.js:16 |  |  | 0.637 |
| walker |  | 9128 | 71 | CONTRIBUTORS.md section #3 |  |  | 0.637 |
| walker |  | 9227 | 99 | export at index.d.ts:338 |  |  | 0.637 |
| walker |  | 9253 | 26 | export names surface in lib/adapters/fetch.js |  |  | 0.637 |
| walker |  | 9253 | 0 | export at lib/adapters/fetch.js:448 |  |  | 0.637 |
| ns | 9323 |  | 485 | CHANGELOG.md — v1.16.0 Notable Changes | 6.2 |  | 0.631 |
| walker |  | 9330 | 77 | CONTRIBUTORS.md section #1 |  |  | 0.631 |
| walker |  | 9359 | 29 | module item at lib/platform/node/index.js:15 |  |  | 0.631 |
| walker |  | 9388 | 29 | export names surface in lib/adapters/http.js |  |  | 0.632 |
| walker |  | 9396 | 8 | listing of 'docs/.vitepress' |  |  | 0.632 |
| walker |  | 9404 | 8 | listing of 'examples/abort-controller' |  |  | 0.634 |
| walker |  | 9412 | 8 | listing of 'examples/get' |  |  | 0.637 |
| walker |  | 9420 | 8 | listing of 'examples/post' |  |  | 0.640 |
| walker |  | 9428 | 8 | listing of 'examples/postMultipartFormData' |  |  | 0.643 |
| walker |  | 9436 | 8 | listing of 'examples/upload' |  |  | 0.646 |
| walker |  | 9447 | 11 | export names surface in lib/platform/node/classes/FormData.js |  |  | 0.646 |
| walker |  | 9473 | 26 | THREATMODEL.md section #52 |  |  | 0.646 |
| walker |  | 9502 | 29 | export body at lib/helpers/bind.js:10 body 11 |  |  | 0.646 |
| walker |  | 9531 | 29 | export body at lib/helpers/spread.js:24 body 25 |  |  | 0.646 |
| ns | 9858 |  | 535 | lib/utils.js — full export roster | 6.3 |  | 0.624 |
| ns | 9929 |  | 71 | lib/defaults/transitional.js — full | 6.4 |  | 0.621 |
| ns | 9988 |  | 59 | lib/platform/index.js — full | 6.5 |  | 0.623 |
