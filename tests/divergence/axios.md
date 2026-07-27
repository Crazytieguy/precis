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
| walker |  | 179 | 4 | listing of '.husky' |  |  | 0.536 |
| ns | 182 |  | 65 | Root documentation files | 1.5 |  | 0.542 |
| walker |  | 208 | 29 | listing of 'lib' |  |  | 1.000 |
| walker |  | 216 | 8 | listing of 'lib/defaults' |  |  | 1.000 |
| walker |  | 227 | 11 | listing of 'lib/env' |  |  | 1.000 |
| walker |  | 240 | 13 | listing of 'lib/platform' |  |  | 1.000 |
| walker |  | 247 | 7 | listing of 'lib/platform/browser' |  |  | 1.000 |
| walker |  | 254 | 7 | listing of 'lib/platform/node' |  |  | 1.000 |
| ns | 255 |  | 73 | lib/env, lib/platform listings (incl. node/browser/common subdirs) | 1.6 |  | 0.891 |
| walker |  | 269 | 15 | listing of 'lib/platform/browser/classes' |  |  | 0.932 |
| walker |  | 285 | 16 | listing of 'lib/cancel' |  |  | 0.933 |
| walker |  | 289 | 4 | listing of 'lib/platform/common' |  |  | 0.949 |
| walker |  | 309 | 20 | listing of 'lib/adapters' |  |  | 0.953 |
| walker |  | 314 | 5 | listing of 'lib/env/classes' |  |  | 0.970 |
| ns | 348 |  | 93 | lib/core, lib/adapters, lib/cancel, lib/defaults listings | 1.7 |  | 0.859 |
| walker |  | 390 | 76 | package identity in package.json |  |  | 0.859 |
| walker |  | 396 | 6 | listing of 'scripts' |  |  | 0.859 |
| walker |  | 406 | 10 | export names surface in lib/axios.js |  |  | 0.859 |
| ns | 447 |  | 99 | lib/helpers listing, part 1 (A-N) | 1.8 |  | 0.770 |
| walker |  | 455 | 49 | listing of 'lib/core' |  |  | 0.871 |
| ns | 538 |  | 91 | lib/helpers listing, part 2 (N-Z) | 1.9 |  | 0.795 |
| walker |  | 645 | 190 | listing of 'lib/helpers' |  |  | 0.978 |
| walker |  | 656 | 11 | listing of 'lib/platform/node/classes' |  |  | 1.000 |
| walker |  | 666 | 10 | export names surface in lib/defaults/index.js |  |  | 1.000 |
| walker |  | 677 | 11 | export names surface in lib/platform/index.js |  |  | 1.000 |
| walker |  | 696 | 19 | export at lib/platform/index.js:4 |  |  | 1.000 |
| walker |  | 709 | 13 | imports in index.js |  |  | 1.000 |
| ns | 743 |  | 205 | tests/ top-level + unit top-level + unit/adapters,core,cancel listings | 1.10 |  | 0.840 |
| walker |  | 795 | 86 | package runtime dependencies in package.json |  |  | 0.840 |
| walker |  | 836 | 41 | README headline in lib/env/README.md |  |  | 0.840 |
| walker |  | 855 | 19 | listing of 'tests' |  |  | 0.844 |
| walker |  | 862 | 7 | listing of 'tests/module' |  |  | 0.844 |
| walker |  | 1009 | 147 | export at index.js:26 |  |  | 0.845 |
| ns | 1013 |  | 270 | unit/helpers, unit/utils, browser, setup listings | 1.11 |  | 0.735 |
| walker |  | 1058 | 49 | README headline in lib/adapters/README.md |  |  | 0.735 |
| walker |  | 1069 | 11 | export names surface in lib/platform/browser/index.js |  |  | 0.735 |
| walker |  | 1080 | 11 | export names surface in lib/platform/node/index.js |  |  | 0.735 |
| walker |  | 1122 | 42 | headings outline in CONTRIBUTORS.md |  |  | 0.735 |
| walker |  | 1142 | 20 | CONTRIBUTORS.md section #0 |  |  | 0.735 |
| walker |  | 1154 | 12 | listing of 'tests/module/cjs' |  |  | 0.736 |
| walker |  | 1226 | 72 | README headline in lib/core/README.md |  |  | 0.736 |
| walker |  | 1299 | 73 | README headline in lib/helpers/README.md |  |  | 0.737 |
| ns | 1305 |  | 292 | tests/smoke listings, part 1 (esm, cjs) | 1.12 |  | 0.659 |
| walker |  | 1323 | 24 | lib/helpers/README.md section #0 |  |  | 0.659 |
| walker |  | 1340 | 17 | export names surface in gulpfile.js |  |  | 0.659 |
| walker |  | 1354 | 14 | listing of 'tests/smoke' |  |  | 0.660 |
| walker |  | 1365 | 11 | listing of 'tests/smoke/bun' |  |  | 0.661 |
| walker |  | 1377 | 12 | listing of 'tests/smoke/cjs' |  |  | 0.663 |
| walker |  | 1390 | 13 | listing of 'tests/smoke/deno' |  |  | 0.663 |
| walker |  | 1427 | 37 | listing of '.github' |  |  | 0.664 |
| ns | 1436 |  | 131 | tests/smoke listings, part 2 (bun, deno) | 1.13 |  | 0.638 |
| walker |  | 1467 | 40 | listing of '.github/workflows' |  |  | 0.640 |
| walker |  | 1486 | 19 | CONTRIBUTORS.md section #2 |  |  | 0.640 |
| ns | 1534 |  | 98 | tests/module listings (cjs, esm) | 1.14 |  | 0.619 |
| walker |  | 1537 | 51 | listing of 'docs' |  |  | 0.622 |
| walker |  | 1544 | 7 | listing of 'docs/es' |  |  | 0.622 |
| walker |  | 1551 | 7 | listing of 'docs/fr' |  |  | 0.622 |
| walker |  | 1558 | 7 | listing of 'docs/zh' |  |  | 0.622 |
| walker |  | 1569 | 11 | listing of 'docs/es/pages' |  |  | 0.622 |
| walker |  | 1580 | 11 | listing of 'docs/fr/pages' |  |  | 0.622 |
| walker |  | 1591 | 11 | listing of 'docs/pages' |  |  | 0.623 |
| ns | 1596 |  | 62 | docs/ top-level + docs/pages listing | 1.15 |  | 0.642 |
| walker |  | 1602 | 11 | listing of 'docs/zh/pages' |  |  | 0.642 |
| walker |  | 1615 | 13 | listing of 'docs/es/pages/misc' |  |  | 0.642 |
| walker |  | 1628 | 13 | listing of 'docs/fr/pages/misc' |  |  | 0.642 |
| walker |  | 1641 | 13 | listing of 'docs/pages/misc' |  |  | 0.642 |
| walker |  | 1654 | 13 | listing of 'docs/zh/pages/misc' |  |  | 0.642 |
| walker |  | 1727 | 73 | headings outline in ECOSYSTEM.md |  |  | 0.642 |
| walker |  | 1763 | 36 | ECOSYSTEM.md section #0 |  |  | 0.642 |
| ns | 1774 |  | 178 | docs/pages/advanced, getting-started, misc listings | 1.16 |  | 0.604 |
| walker |  | 1781 | 18 | listing of 'docs/es/pages/getting-started' |  |  | 0.604 |
| walker |  | 1799 | 18 | listing of 'docs/fr/pages/getting-started' |  |  | 0.604 |
| walker |  | 1817 | 18 | listing of 'docs/pages/getting-started' |  |  | 0.607 |
| walker |  | 1835 | 18 | listing of 'docs/zh/pages/getting-started' |  |  | 0.607 |
| walker |  | 1853 | 18 | listing of 'tests/module/esm' |  |  | 0.614 |
| walker |  | 1871 | 18 | listing of 'tests/smoke/esm' |  |  | 0.618 |
| ns | 1876 |  | 102 | examples/ listings (all subdirectories) | 1.17 |  | 0.593 |
| walker |  | 1918 | 47 | lib/core/README.md section #0 |  |  | 0.594 |
| ns | 1975 |  | 99 | sandbox/, scripts/, .github/, .husky/ listings | 1.18 |  | 0.612 |
| walker |  | 1997 | 79 | headings outline in COLLABORATOR_GUIDE.md |  |  | 0.612 |
| walker |  | 2047 | 50 | listing of 'examples' |  |  | 0.624 |
| walker |  | 2058 | 11 | export names surface in lib/utils.js |  |  | 0.624 |
| walker |  | 2081 | 23 | listing of 'tests/module/esm/tests' |  |  | 0.634 |
| walker |  | 2100 | 19 | listing of 'tests/module/esm/tests/helpers' |  |  | 0.634 |
| walker |  | 2169 | 69 | COLLABORATOR_GUIDE.md section #0 |  |  | 0.634 |
| walker |  | 2187 | 18 | module item at lib/defaults/index.js:23 |  |  | 0.634 |
| walker |  | 2271 | 84 | json config tsconfig.json |  |  | 0.634 |
| walker |  | 2306 | 35 | listing of 'tests/smoke/deno/tests' |  |  | 0.642 |
| walker |  | 2316 | 10 | export names surface in lib/core/Axios.js |  |  | 0.642 |
| walker |  | 2326 | 10 | export names surface in lib/helpers/null.js |  |  | 0.642 |
| walker |  | 2336 | 10 | export names surface in lib/helpers/throttle.js |  |  | 0.642 |
| ns | 2351 |  | 376 | index.js — public re-export surface | 2.1 |  | 0.616 |
| walker |  | 2412 | 76 | export at lib/platform/browser/index.js:5 |  |  | 0.616 |
| walker |  | 2450 | 38 | listing of 'tests/module/cjs/tests' |  |  | 0.631 |
| walker |  | 2481 | 31 | listing of 'tests/module/cjs/tests/helpers' |  |  | 0.631 |
| walker |  | 2492 | 11 | export names surface in lib/adapters/adapters.js |  |  | 0.631 |
| walker |  | 2503 | 11 | export names surface in lib/cancel/CancelToken.js |  |  | 0.631 |
| walker |  | 2514 | 11 | export names surface in lib/core/AxiosError.js |  |  | 0.631 |
| walker |  | 2525 | 11 | export names surface in lib/core/AxiosHeaders.js |  |  | 0.631 |
| walker |  | 2536 | 11 | export names surface in lib/helpers/callbackify.js |  |  | 0.631 |
| walker |  | 2547 | 11 | export names surface in lib/helpers/composeSignals.js |  |  | 0.631 |
| walker |  | 2558 | 11 | export names surface in lib/helpers/readBlob.js |  |  | 0.631 |
| walker |  | 2569 | 11 | export names surface in lib/helpers/speedometer.js |  |  | 0.631 |
| walker |  | 2580 | 11 | export names surface in lib/helpers/validator.js |  |  | 0.631 |
| walker |  | 2598 | 18 | export at lib/helpers/validator.js:109 |  |  | 0.631 |
| ns | 2667 |  | 316 | lib/ subdirectory README notes (core, adapters, env, helpers) | 2.2 |  | 0.642 |
| walker |  | 2747 | 149 | module item at index.js:6 |  |  | 0.675 |
| walker |  | 2759 | 12 | export names surface in lib/cancel/CanceledError.js |  |  | 0.675 |
| walker |  | 2771 | 12 | export names surface in lib/core/InterceptorManager.js |  |  | 0.675 |
| walker |  | 2783 | 12 | export names surface in lib/helpers/AxiosTransformStream.js |  |  | 0.675 |
| walker |  | 2795 | 12 | export names surface in lib/helpers/HttpStatusCode.js |  |  | 0.675 |
| walker |  | 2807 | 12 | export names surface in lib/helpers/toFormData.js |  |  | 0.675 |
| walker |  | 2847 | 40 | COLLABORATOR_GUIDE.md section #1 |  |  | 0.675 |
| ns | 2858 |  | 191 | package.json — browser/react-native alias map | 2.3 |  | 0.665 |
| walker |  | 2860 | 13 | module item at lib/platform/node/index.js:5 |  |  | 0.665 |
| walker |  | 2873 | 13 | export names surface in lib/helpers/AxiosURLSearchParams.js |  |  | 0.665 |
| walker |  | 2886 | 13 | export names surface in lib/helpers/formDataToJSON.js |  |  | 0.665 |
| walker |  | 2899 | 13 | export names surface in lib/helpers/formDataToStream.js |  |  | 0.665 |
| walker |  | 2999 | 100 | export at lib/platform/node/index.js:27 |  |  | 0.665 |
| walker |  | 3013 | 14 | module item at lib/platform/node/index.js:7 |  |  | 0.665 |
| walker |  | 3027 | 14 | export names surface in lib/helpers/ZlibHeaderTransformStream.js |  |  | 0.665 |
| walker |  | 3031 | 4 | listing of 'docs/data' |  |  | 0.665 |
| walker |  | 3035 | 4 | listing of 'examples/all' |  |  | 0.667 |
| walker |  | 3039 | 4 | listing of 'examples/amd' |  |  | 0.668 |
| walker |  | 3043 | 4 | listing of 'examples/transform-response' |  |  | 0.670 |
| walker |  | 3058 | 15 | export names surface in lib/adapters/xhr.js |  |  | 0.670 |
| walker |  | 3073 | 15 | export names surface in lib/env/data.js |  |  | 0.670 |
| walker |  | 3088 | 15 | export names surface in lib/helpers/resolveConfig.js |  |  | 0.670 |
| walker |  | 3088 | 0 | export at lib/helpers/resolveConfig.js:38 |  |  | 0.670 |
| walker |  | 3103 | 15 | export names surface in lib/helpers/spread.js |  |  | 0.670 |
| walker |  | 3103 | 0 | export at lib/helpers/spread.js:24 |  |  | 0.670 |
| walker |  | 3119 | 16 | export names surface in lib/cancel/isCancel.js |  |  | 0.670 |
| walker |  | 3119 | 0 | export at lib/cancel/isCancel.js:3 |  |  | 0.670 |
| walker |  | 3134 | 15 | export body at lib/cancel/isCancel.js:3 body 4 |  |  | 0.670 |
| walker |  | 3150 | 16 | export names surface in lib/core/dispatchRequest.js |  |  | 0.670 |
| walker |  | 3150 | 0 | export at lib/core/dispatchRequest.js:34 |  |  | 0.670 |
| walker |  | 3166 | 16 | export names surface in lib/helpers/cookies.js |  |  | 0.670 |
| walker |  | 3182 | 16 | export names surface in lib/helpers/isURLSameOrigin.js |  |  | 0.670 |
| walker |  | 3198 | 16 | export names surface in lib/helpers/parseHeaders.js |  |  | 0.670 |
| walker |  | 3198 | 0 | export at lib/helpers/parseHeaders.js:41 |  |  | 0.670 |
| walker |  | 3214 | 16 | export names surface in lib/helpers/parseProtocol.js |  |  | 0.670 |
| walker |  | 3214 | 0 | export at lib/helpers/parseProtocol.js:3 |  |  | 0.670 |
| ns | 3308 |  | 450 | index.d.ts — AxiosRequestConfig interface, part 1 (url..maxRate) | 2.4 |  | 0.642 |
| walker |  | 3319 | 105 | listing of 'tests/unit' |  |  | 0.669 |
| walker |  | 3352 | 33 | listing of 'tests/unit/adapters' |  |  | 0.686 |
| walker |  | 3388 | 36 | listing of 'tests/unit/core' |  |  | 0.704 |
| walker |  | 3447 | 59 | listing of 'tests/unit/utils' |  |  | 0.708 |
| walker |  | 3584 | 137 | listing of 'docs/es/pages/advanced' |  |  | 0.708 |
| walker |  | 3721 | 137 | listing of 'docs/fr/pages/advanced' |  |  | 0.708 |
| walker |  | 3858 | 137 | listing of 'docs/pages/advanced' |  |  | 0.749 |
| walker |  | 3995 | 137 | listing of 'docs/zh/pages/advanced' |  |  | 0.749 |
| walker |  | 4012 | 17 | export names surface in lib/helpers/isAbsoluteURL.js |  |  | 0.749 |
| walker |  | 4012 | 0 | export at lib/helpers/isAbsoluteURL.js:10 |  |  | 0.749 |
| walker |  | 4029 | 17 | export names surface in lib/helpers/isAxiosError.js |  |  | 0.749 |
| walker |  | 4029 | 0 | export at lib/helpers/isAxiosError.js:12 |  |  | 0.749 |
| ns | 4035 |  | 727 | index.d.ts — AxiosRequestConfig interface, part 2 (beforeRedirect..redact) | 2.5 |  | 0.701 |
| walker |  | 4039 | 10 | export names surface in lib/platform/common/utils.js |  |  | 0.701 |
| ns | 4234 |  | 199 | lib/core/InterceptorManager.js — constructor, use(), forEach() (docblocks elided) | 3.1 |  | 0.686 |
| walker |  | 4292 | 253 | headings outline in THREATMODEL.md |  |  | 0.686 |
| walker |  | 4297 | 5 | THREATMODEL.md section #24 |  |  | 0.686 |
| walker |  | 4302 | 5 | THREATMODEL.md section #33 |  |  | 0.686 |
| walker |  | 4307 | 5 | THREATMODEL.md section #44 |  |  | 0.686 |
| walker |  | 4314 | 7 | THREATMODEL.md section #10 |  |  | 0.686 |
| walker |  | 4321 | 7 | THREATMODEL.md section #53 |  |  | 0.686 |
| walker |  | 4432 | 111 | THREATMODEL.md section #0 |  |  | 0.686 |
| walker |  | 4484 | 52 | THREATMODEL.md section #25 |  |  | 0.686 |
| walker |  | 4538 | 54 | COLLABORATOR_GUIDE.md section #3 |  |  | 0.686 |
| walker |  | 4556 | 18 | export names surface in lib/helpers/bind.js |  |  | 0.686 |
| walker |  | 4556 | 0 | export at lib/helpers/bind.js:10 |  |  | 0.686 |
| walker |  | 4574 | 18 | export names surface in lib/helpers/shouldBypassProxy.js |  |  | 0.686 |
| walker |  | 4574 | 0 | export at lib/helpers/shouldBypassProxy.js:127 |  |  | 0.686 |
| ns | 4729 |  | 495 | lib/core/dispatchRequest.js — part 1 (cancellation check, request transform, adapter call) | 3.2 |  | 0.656 |
| walker |  | 4877 | 303 | lib/adapters/README.md section #1 |  |  | 0.658 |
| walker |  | 4896 | 19 | export names surface in lib/core/settle.js |  |  | 0.658 |
| walker |  | 4896 | 0 | export at lib/core/settle.js:14 |  |  | 0.658 |
| walker |  | 4915 | 19 | export names surface in lib/core/transformData.js |  |  | 0.658 |
| walker |  | 4915 | 0 | export at lib/core/transformData.js:15 |  |  | 0.658 |
| walker |  | 4934 | 19 | export names surface in lib/helpers/estimateDataURLDecodedBytes.js |  |  | 0.658 |
| walker |  | 4934 | 0 | export at lib/helpers/estimateDataURLDecodedBytes.js:10 |  |  | 0.658 |
| ns | 5137 |  | 408 | lib/core/dispatchRequest.js — part 2 (response/rejection transform) | 3.3 |  | 0.633 |
| walker |  | 5962 | 1028 | export names surface in index.d.ts |  |  | 0.634 |
| walker |  | 5962 | 0 | export at index.d.ts:366 |  |  | 0.634 |
| walker |  | 5962 | 0 | export at index.d.ts:461 |  |  | 0.634 |
| walker |  | 5962 | 0 | export at index.d.ts:713 |  |  | 0.634 |
| ns | 6074 |  | 937 | lib/core/mergeConfig.js — part 1 (null-proto result object + per-strategy merge helper fns) | 3.4 |  | 0.595 |
| walker |  | 6222 | 260 | export member names #1 at index.d.ts:366 |  |  | 0.615 |
| walker |  | 6366 | 144 | export member names at index.d.ts:461 |  |  | 0.615 |
| walker |  | 6558 | 192 | export member names at index.d.ts:713 |  |  | 0.615 |
| walker |  | 6571 | 13 | export at index.d.ts:476 |  |  | 0.615 |
| walker |  | 6583 | 12 | export at index.d.ts:535 |  |  | 0.615 |
| walker |  | 6597 | 14 | export at index.d.ts:457 |  |  | 0.615 |
| walker |  | 6611 | 14 | export at index.d.ts:545 |  |  | 0.615 |
| walker |  | 6627 | 16 | export at index.d.ts:541 |  |  | 0.615 |
| walker |  | 6646 | 19 | export at index.d.ts:6 |  |  | 0.615 |
| walker |  | 6665 | 19 | export at index.d.ts:154 |  |  | 0.615 |
| walker |  | 6686 | 21 | export at index.d.ts:158 |  |  | 0.615 |
| walker |  | 6708 | 22 | export at index.d.ts:359 |  |  | 0.618 |
| ns | 6710 |  | 636 | lib/core/mergeConfig.js — part 2 (per-key mergeMap table + apply loop) | 3.5 |  | 0.598 |
| walker |  | 6731 | 23 | export at index.d.ts:480 |  |  | 0.598 |
| walker |  | 6754 | 23 | export at index.d.ts:564 |  |  | 0.598 |
| walker |  | 6777 | 23 | export at index.d.ts:676 |  |  | 0.598 |
| walker |  | 6801 | 24 | export at index.d.ts:323 |  |  | 0.598 |
| walker |  | 6826 | 25 | export at index.d.ts:549 |  |  | 0.598 |
| walker |  | 6852 | 26 | export at index.d.ts:319 |  |  | 0.598 |
| walker |  | 6878 | 26 | export at index.d.ts:327 |  |  | 0.598 |
| walker |  | 6906 | 28 | export at index.d.ts:141 |  |  | 0.598 |
| walker |  | 6936 | 30 | export at index.d.ts:680 |  |  | 0.598 |
| walker |  | 6960 | 24 | export at index.d.ts:686 |  |  | 0.598 |
| walker |  | 6994 | 34 | export at index.d.ts:558 |  |  | 0.598 |
| walker |  | 7029 | 35 | export at index.d.ts:553 |  |  | 0.598 |
| ns | 7046 |  | 336 | lib/core/AxiosError.js — class signature + error code constants | 4.1 |  | 0.589 |
| walker |  | 7065 | 36 | export at index.d.ts:569 |  |  | 0.589 |
| walker |  | 7095 | 30 | export at index.d.ts:274 |  |  | 0.589 |
| walker |  | 7136 | 41 | export at index.d.ts:163 |  |  | 0.589 |
| walker |  | 7181 | 45 | export at index.d.ts:309 |  |  | 0.589 |
| walker |  | 7228 | 47 | export at index.d.ts:293 |  |  | 0.589 |
| walker |  | 7262 | 34 | export at index.d.ts:706 |  |  | 0.589 |
| walker |  | 7311 | 49 | export at index.d.ts:279 |  |  | 0.589 |
| ns | 7338 |  | 292 | lib/cancel/isCancel.js, CanceledError.js — full | 4.2 |  | 0.579 |
| walker |  | 7369 | 58 | export at index.d.ts:145 |  |  | 0.579 |
| walker |  | 7410 | 41 | export at index.d.ts:690 |  |  | 0.576 |
| ns | 7410 |  | 72 | lib/cancel/CancelToken.js — method locations | 4.3 |  | 0.576 |
| walker |  | 7473 | 63 | export at index.d.ts:595 |  |  | 0.576 |
| walker |  | 7540 | 67 | export at index.d.ts:286 |  |  | 0.576 |
| walker |  | 7596 | 56 | export at index.d.ts:112 |  |  | 0.576 |
| ns | 7633 |  | 223 | lib/core/AxiosHeaders.js — method locations | 5.1 |  | 0.566 |
| walker |  | 7669 | 73 | export at index.d.ts:484 |  |  | 0.566 |
| walker |  | 7743 | 74 | export at index.d.ts:299 |  |  | 0.566 |
| walker |  | 7762 | 19 | export body at lib/helpers/isAxiosError.js:12 body 13 |  |  | 0.566 |
| ns | 7780 |  | 147 | lib/adapters/adapters.js, xhr.js, fetch.js, http.js — top-level function/export locations | 5.2 |  | 0.558 |
| walker |  | 7797 | 35 | module item at lib/defaults/index.js:11 |  |  | 0.558 |
| walker |  | 7856 | 59 | COLLABORATOR_GUIDE.md section #5 |  |  | 0.558 |
| walker |  | 7876 | 20 | export names surface in lib/core/mergeConfig.js |  |  | 0.558 |
| walker |  | 7876 | 0 | export at lib/core/mergeConfig.js:17 |  |  | 0.558 |
| walker |  | 7896 | 20 | export names surface in lib/helpers/combineURLs.js |  |  | 0.558 |
| walker |  | 7896 | 0 | export at lib/helpers/combineURLs.js:11 |  |  | 0.558 |
| walker |  | 7916 | 20 | export names surface in lib/helpers/deprecatedMethod.js |  |  | 0.558 |
| walker |  | 7916 | 0 | export at lib/helpers/deprecatedMethod.js:15 |  |  | 0.558 |
| ns | 7983 |  | 203 | lib/helpers/* — one-line-per-file export locations, part 1 (serialization + parsing + config) | 5.3 |  | 0.555 |
| walker |  | 7994 | 78 | listing of 'tests/unit/helpers' |  |  | 0.565 |
| walker |  | 8118 | 124 | listing of 'tests/browser' |  |  | 0.594 |
| ns | 8329 |  | 346 | lib/helpers/* — one-line-per-file export locations, part 2 (streaming + proxy/origin + data URIs + small utilities) | 5.4 |  | 0.587 |
| walker |  | 8580 | 462 | package scripts in package.json |  |  | 0.587 |
| walker |  | 8652 | 72 | listing of 'tests/smoke/bun/tests' |  |  | 0.602 |
| walker |  | 8673 | 21 | export names surface in lib/helpers/toURLEncodedForm.js |  |  | 0.603 |
| walker |  | 8673 | 0 | export at lib/helpers/toURLEncodedForm.js:7 |  |  | 0.603 |
| walker |  | 8738 | 65 | export at index.d.ts:251 |  |  | 0.603 |
| walker |  | 8760 | 22 | export names surface in lib/helpers/fromDataURI.js |  |  | 0.603 |
| walker |  | 8760 | 0 | export at lib/helpers/fromDataURI.js:21 |  |  | 0.603 |
| ns | 8838 |  | 509 | README.md — Error Types table (per-code descriptions) | 6.1 |  | 0.598 |
| walker |  | 8876 | 116 | listing of 'tests/smoke/esm/tests' |  |  | 0.612 |
| walker |  | 8900 | 24 | imports in lib/platform/index.js |  |  | 0.612 |
| walker |  | 8925 | 25 | export names surface in lib/core/buildFullPath.js |  |  | 0.612 |
| walker |  | 8925 | 0 | export at lib/core/buildFullPath.js:16 |  |  | 0.612 |
| walker |  | 8996 | 71 | CONTRIBUTORS.md section #3 |  |  | 0.612 |
| walker |  | 9128 | 132 | listing of 'tests/smoke/cjs/tests' |  |  | 0.637 |
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
