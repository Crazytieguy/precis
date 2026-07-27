Score(3000)=0.638 I=0.897 C=0.453 ns_rows≤3K=20/40 (reached=9 partial=0 missing=11)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 59 | 59 | listing of '.' |  |  | 1.000 |
| ns | 59 |  | 59 | Root directory listing | 1.1 |  | 1.000 |
| walker |  | 90 | 31 | listing of 'packages' |  |  | 1.000 |
| ns | 90 |  | 31 | packages/ directory listing | 1.2 |  | 1.000 |
| walker |  | 154 | 64 | package identity in package.json |  |  | 1.000 |
| ns | 164 |  | 74 | README title + tagline | 1.3 |  | 0.860 |
| walker |  | 172 | 18 | listing of 'packages/docs' |  |  | 0.860 |
| walker |  | 195 | 23 | listing of 'packages/app-server' |  |  | 0.860 |
| walker |  | 219 | 24 | listing of 'packages/deploy-cloudflare' |  |  | 0.861 |
| walker |  | 245 | 26 | listing of 'packages/lib' |  |  | 0.861 |
| walker |  | 264 | 19 | listing of 'packages/lib/src' |  |  | 0.862 |
| ns | 282 |  | 118 | README intro paragraph | 1.4 |  | 0.750 |
| walker |  | 284 | 20 | listing of 'packages/lib/src/api' |  |  | 0.751 |
| walker |  | 310 | 26 | listing of 'packages/lib/src/crypto' |  |  | 0.752 |
| ns | 312 |  | 30 | .github/ directory listing | 1.5 |  | 0.682 |
| walker |  | 333 | 23 | listing of 'packages/lib/src/crypto/encryption-algorithms' |  |  | 0.683 |
| walker |  | 359 | 26 | listing of 'packages/lib/src/crypto/serialization' |  |  | 0.685 |
| walker |  | 385 | 26 | listing of 'packages/lib/src/notes' |  |  | 0.687 |
| walker |  | 389 | 4 | listing of 'packages/lib/src/files' |  |  | 0.688 |
| ns | 411 |  | 99 | docker-compose.yml | 1.6 |  | 0.595 |
| walker |  | 415 | 26 | listing of 'packages/crypto' |  |  | 0.595 |
| walker |  | 425 | 10 | export names surface in packages/lib/src/index.ts |  |  | 0.595 |
| walker |  | 455 | 30 | listing of 'packages/cli' |  |  | 0.595 |
| walker |  | 459 | 4 | listing of 'packages/cli/bin' |  |  | 0.595 |
| ns | 491 |  | 80 | .github/workflows listing | 1.7 |  | 0.520 |
| ns | 504 |  | 13 | .github/ISSUE_TEMPLATE listing | 1.8 |  | 0.506 |
| walker |  | 569 | 110 | README headline in README.md |  |  | 0.599 |
| walker |  | 591 | 22 | listing of 'packages/app-server/src' |  |  | 0.599 |
| walker |  | 610 | 19 | listing of 'packages/app-server/src/modules' |  |  | 0.599 |
| walker |  | 623 | 13 | listing of 'packages/app-server/src/modules/storage' |  |  | 0.599 |
| walker |  | 638 | 15 | listing of 'packages/app-server/src/modules/shared' |  |  | 0.599 |
| walker |  | 652 | 14 | listing of 'packages/app-server/src/modules/shared/logger' |  |  | 0.600 |
| ns | 653 |  | 149 | pnpm workspace + node version | 1.9 |  | 0.544 |
| walker |  | 673 | 21 | listing of 'packages/app-server/src/modules/tasks' |  |  | 0.544 |
| walker |  | 698 | 25 | listing of 'packages/app-server/src/modules/app' |  |  | 0.545 |
| ns | 713 |  | 60 | Repo hygiene: renovate config | 1.10 |  | 0.522 |
| walker |  | 722 | 24 | listing of 'packages/app-server/src/modules/app/config' |  |  | 0.522 |
| walker |  | 757 | 35 | listing of 'packages/app-server/src/modules/app/middlewares' |  |  | 0.523 |
| walker |  | 794 | 37 | listing of 'packages/app-server/src/modules/storage/factories' |  |  | 0.524 |
| walker |  | 836 | 42 | listing of 'packages/app-server/src/modules/app/auth' |  |  | 0.525 |
| walker |  | 839 | 3 | listing of 'packages/app-server/src/modules/shared/utils' |  |  | 0.525 |
| ns | 853 |  | 140 | README Project Structure prose | 1.11 |  | 0.504 |
| walker |  | 903 | 64 | listing of 'packages/app-server/src/modules/notes' |  |  | 0.507 |
| walker |  | 907 | 4 | listing of 'packages/app-server/src/modules/app/users' |  |  | 0.507 |
| walker |  | 932 | 25 | listing of 'packages/cli/src' |  |  | 0.507 |
| walker |  | 952 | 20 | listing of 'packages/cli/src/config' |  |  | 0.507 |
| walker |  | 972 | 20 | listing of 'packages/cli/src/create-note' |  |  | 0.508 |
| walker |  | 981 | 9 | listing of 'packages/cli/src/shared' |  |  | 0.508 |
| walker |  | 991 | 10 | listing of 'packages/cli/src/files' |  |  | 0.508 |
| walker |  | 1002 | 11 | listing of 'packages/cli/src/view-note' |  |  | 0.508 |
| walker |  | 1053 | 51 | package runtime metadata in package.json |  |  | 0.510 |
| walker |  | 1061 | 8 | listing of 'packages/app-server/src/modules/shared/errors' |  |  | 0.510 |
| walker |  | 1069 | 8 | listing of 'packages/app-server/src/modules/shared/validation' |  |  | 0.510 |
| walker |  | 1103 | 34 | listing of 'packages/crypto/src' |  |  | 0.512 |
| walker |  | 1124 | 21 | listing of 'packages/crypto/src/node' |  |  | 0.512 |
| walker |  | 1145 | 21 | listing of 'packages/crypto/src/web' |  |  | 0.513 |
| walker |  | 1194 | 49 | listing of 'packages/crypto/src/encryption-algorithms' |  |  | 0.516 |
| ns | 1203 |  | 350 | README Features bullet list | 1.12 |  | 0.472 |
| walker |  | 1230 | 36 | listing of 'packages/docs/src' |  |  | 0.472 |
| walker |  | 1243 | 13 | listing of 'packages/docs/src/integrations' |  |  | 0.472 |
| walker |  | 1273 | 30 | listing of 'packages/docs/src/self-hosting' |  |  | 0.472 |
| walker |  | 1282 | 9 | listing of 'packages/docs/src/components' |  |  | 0.473 |
| walker |  | 1292 | 10 | listing of 'packages/docs/src/resources' |  |  | 0.473 |
| walker |  | 1303 | 11 | listing of 'packages/docs/src/data' |  |  | 0.473 |
| walker |  | 1368 | 65 | listing of 'packages/app-client' |  |  | 0.473 |
| walker |  | 1378 | 10 | listing of 'packages/app-client/e2e-tests' |  |  | 0.473 |
| walker |  | 1407 | 29 | listing of 'packages/app-client/src' |  |  | 0.473 |
| walker |  | 1410 | 3 | listing of 'packages/app-client/src/assets' |  |  | 0.473 |
| walker |  | 1447 | 37 | listing of 'packages/app-client/src/modules' |  |  | 0.474 |
| walker |  | 1454 | 7 | listing of 'packages/app-client/src/modules/ui' |  |  | 0.474 |
| walker |  | 1469 | 15 | listing of 'packages/app-client/src/modules/config' |  |  | 0.474 |
| ns | 1472 |  | 269 | Root package.json | 1.13 |  | 0.449 |
| walker |  | 1487 | 18 | listing of 'packages/app-client/src/modules/auth' |  |  | 0.449 |
| walker |  | 1506 | 19 | listing of 'packages/app-client/src/modules/shared' |  |  | 0.449 |
| walker |  | 1528 | 22 | listing of 'packages/app-client/src/modules/i18n' |  |  | 0.450 |
| walker |  | 1550 | 22 | listing of 'packages/app-client/src/modules/shared/http' |  |  | 0.450 |
| ns | 1584 |  | 112 | CONTRIBUTING: i18n pointer | 1.14 |  | 0.436 |
| walker |  | 1590 | 40 | listing of 'packages/app-client/src/modules/notes' |  |  | 0.436 |
| walker |  | 1637 | 47 | listing of 'packages/app-client/src/modules/ui/components' |  |  | 0.437 |
| ns | 1708 |  | 124 | CI: representative per-package workflow | 1.15 |  | 0.411 |
| walker |  | 1723 | 86 | listing of 'packages/app-client/src/locales' |  |  | 0.412 |
| walker |  | 1727 | 4 | listing of 'packages/app-client/src/modules/theme' |  |  | 0.413 |
| walker |  | 1730 | 3 | listing of 'packages/app-client/src/modules/shared/hooks' |  |  | 0.413 |
| walker |  | 1733 | 3 | listing of 'packages/app-client/src/modules/shared/style' |  |  | 0.413 |
| walker |  | 1737 | 4 | listing of 'packages/app-client/src/modules/shared/utils' |  |  | 0.413 |
| walker |  | 1742 | 5 | listing of 'packages/app-client/src/modules/auth/pages' |  |  | 0.413 |
| walker |  | 1747 | 5 | listing of 'packages/app-client/src/modules/ui/layouts' |  |  | 0.413 |
| walker |  | 1757 | 10 | listing of 'packages/app-client/src/modules/docs' |  |  | 0.413 |
| walker |  | 1767 | 10 | listing of 'packages/app-client/src/modules/files' |  |  | 0.414 |
| walker |  | 1774 | 7 | listing of 'packages/app-client/src/modules/shared/files' |  |  | 0.414 |
| walker |  | 1817 | 43 | README headline in packages/app-client/README.md |  |  | 0.414 |
| walker |  | 1828 | 11 | listing of 'packages/docs/.vitepress' |  |  | 0.414 |
| walker |  | 1885 | 57 | README headline in packages/cli/README.md |  |  | 0.414 |
| walker |  | 1944 | 59 | README headline in packages/lib/README.md |  |  | 0.414 |
| walker |  | 1992 | 48 | YAML config at docker-compose.yml |  |  | 0.431 |
| walker |  | 2050 | 58 | YAML config tail at docker-compose.yml |  |  | 0.493 |
| walker |  | 2080 | 30 | listing of '.github' |  |  | 0.535 |
| walker |  | 2160 | 80 | listing of '.github/workflows' |  |  | 0.616 |
| ns | 2185 |  | 477 | README 'How It Works' 13-step flow | 1.16 |  | 0.578 |
| walker |  | 2238 | 78 | package scripts in package.json |  |  | 0.609 |
| walker |  | 2309 | 71 | README headline in packages/crypto/README.md |  |  | 0.609 |
| walker |  | 2338 | 29 | headings outline in packages/crypto/README.md |  |  | 0.609 |
| walker |  | 2351 | 13 | listing of 'packages/app-client/src/modules/notes/components' |  |  | 0.610 |
| walker |  | 2364 | 13 | listing of 'packages/app-client/src/modules/notes/pages' |  |  | 0.610 |
| walker |  | 2378 | 14 | listing of 'packages/lib/src/crypto/serialization/cbor-array' |  |  | 0.611 |
| walker |  | 2400 | 22 | listing of 'packages/crypto/src/node/encryption-algorithms' |  |  | 0.612 |
| walker |  | 2422 | 22 | listing of 'packages/crypto/src/web/encryption-algorithms' |  |  | 0.613 |
| ns | 2453 |  | 268 | CONTRIBUTING: local dev setup + testing | 1.17 |  | 0.559 |
| walker |  | 2460 | 38 | headings outline in packages/lib/README.md |  |  | 0.559 |
| walker |  | 2523 | 63 | listing of 'packages/app-client/public' |  |  | 0.559 |
| ns | 2553 |  | 100 | Root Dockerfile | 1.18 |  | 0.549 |
| walker |  | 2558 | 35 | plaintext config packages/app-client/.env.example |  |  | 0.549 |
| ns | 2754 |  | 201 | crypto/ directory structure | 2.1 |  | 0.606 |
| walker |  | 2756 | 198 | headings outline in README.md |  |  | 0.607 |
| ns | 2825 |  | 71 | crypto package README | 2.2 |  | 0.611 |
| walker |  | 2862 | 106 | README.md section #0 |  |  | 0.637 |
| walker |  | 2883 | 21 | README.md section #32 |  |  | 0.637 |
| walker |  | 2917 | 34 | README.md section #18 |  |  | 0.637 |
| walker |  | 2935 | 18 | listing of 'packages/app-server/src/modules/notes/tasks' |  |  | 0.637 |
| walker |  | 2968 | 33 | README.md section #25 |  |  | 0.637 |
| walker |  | 2987 | 19 | README.md section #6 |  |  | 0.638 |
| walker |  | 3004 | 17 | README.md section #7 |  |  | 0.639 |
| walker |  | 3023 | 19 | README.md section #8 |  |  | 0.640 |
| walker |  | 3075 | 52 | listing of 'packages/docs/src/public' |  |  | 0.640 |
| walker |  | 3095 | 20 | README.md section #2 |  |  | 0.642 |
| walker |  | 3116 | 21 | README.md section #5 |  |  | 0.644 |
| walker |  | 3137 | 21 | README.md section #9 |  |  | 0.647 |
| ns | 3155 |  | 330 | createEnclosedCryptoApi factory | 2.3 |  | 0.610 |
| walker |  | 3193 | 56 | README headline in packages/app-client/src/locales/README.md |  |  | 0.610 |
| walker |  | 3215 | 22 | headings outline in packages/app-client/src/locales/README.md |  |  | 0.610 |
| walker |  | 3237 | 22 | README.md section #12 |  |  | 0.613 |
| walker |  | 3271 | 34 | package dev/peer dependencies in package.json |  |  | 0.613 |
| walker |  | 3294 | 23 | README.md section #11 |  |  | 0.616 |
| walker |  | 3316 | 22 | README.md section #10 |  |  | 0.620 |
| walker |  | 3359 | 43 | listing of 'packages/docs/src/public/logos' |  |  | 0.620 |
| walker |  | 3474 | 115 | packages/lib/README.md section #2 |  |  | 0.620 |
| walker |  | 3517 | 43 | headings outline in packages/docs/src/index.md |  |  | 0.620 |
| walker |  | 3542 | 25 | README.md section #3 |  |  | 0.625 |
| walker |  | 3572 | 30 | headings outline in packages/docs/src/self-hosting/troubleshooting.md |  |  | 0.625 |
| walker |  | 3572 | 0 | packages/docs/src/self-hosting/troubleshooting.md section #0 |  |  | 0.625 |
| ns | 3669 |  | 514 | Node key-derivation + buffer usecases | 2.4 |  | 0.575 |
| walker |  | 3725 | 153 | package identity metadata in package.json |  |  | 0.604 |
| walker |  | 3754 | 29 | README.md section #4 |  |  | 0.610 |
| ns | 4220 |  | 551 | Node AES-256-GCM implementation | 2.5 |  | 0.567 |
| walker |  | 4273 | 519 | plaintext config Dockerfile |  |  | 0.582 |
| walker |  | 4310 | 37 | headings outline in packages/docs/src/resources/i18n.md |  |  | 0.582 |
| walker |  | 4410 | 100 | headings outline in packages/cli/README.md |  |  | 0.582 |
| ns | 4412 |  | 192 | lib/ directory structure | 3.1 |  | 0.620 |
| walker |  | 4438 | 28 | packages/cli/README.md section #2 |  |  | 0.620 |
| walker |  | 4601 | 163 | README.md section #23 |  |  | 0.635 |
| walker |  | 4765 | 164 | export at packages/lib/src/index.ts:10 |  |  | 0.636 |
| ns | 4771 |  | 359 | lib public API surface (index.ts) | 3.2 |  | 0.622 |
| walker |  | 4791 | 26 | packages/crypto/README.md section #3 |  |  | 0.622 |
| walker |  | 4906 | 115 | headings outline in packages/app-client/README.md |  |  | 0.622 |
| walker |  | 4906 | 0 | packages/app-client/README.md section #3 |  |  | 0.622 |
| walker |  | 4983 | 77 | packages/app-client/README.md section #0 |  |  | 0.622 |
| walker |  | 5009 | 26 | packages/cli/README.md section #7 |  |  | 0.622 |
| walker |  | 5042 | 33 | packages/crypto/README.md section #2 |  |  | 0.622 |
| walker |  | 5072 | 30 | packages/app-client/README.md section #2 |  |  | 0.622 |
| walker |  | 5108 | 36 | README.md section #1 |  |  | 0.627 |
| walker |  | 5134 | 26 | packages/lib/README.md section #4 |  |  | 0.627 |
| walker |  | 5185 | 51 | headings outline in packages/docs/src/integrations/npm-package.md |  |  | 0.627 |
| walker |  | 5218 | 33 | packages/cli/README.md section #6 |  |  | 0.627 |
| walker |  | 5276 | 58 | headings outline in packages/docs/src/self-hosting/configuration.md |  |  | 0.627 |
| walker |  | 5309 | 33 | packages/lib/README.md section #3 |  |  | 0.627 |
| walker |  | 5369 | 60 | json config renovate.json |  |  | 0.641 |
| walker |  | 5398 | 29 | README.md section #31 |  |  | 0.641 |
| walker |  | 5429 | 31 | README.md section #30 |  |  | 0.641 |
| walker |  | 5467 | 38 | README.md section #16 |  |  | 0.641 |
| ns | 5512 |  | 741 | crypto.usecases.ts: encryptNote/decryptNote | 3.3 |  | 0.589 |
| walker |  | 5582 | 115 | README.md section #24 |  |  | 0.589 |
| walker |  | 5667 | 85 | headings outline in packages/docs/src/self-hosting/other-platforms.md |  |  | 0.589 |
| walker |  | 5667 | 0 | packages/docs/src/self-hosting/other-platforms.md section #0 |  |  | 0.589 |
| walker |  | 5680 | 13 | export names surface in packages/lib/src/notes/notes.usecases.ts |  |  | 0.589 |
| walker |  | 5740 | 60 | README.md section #13 |  |  | 0.597 |
| walker |  | 5778 | 38 | README.md section #29 |  |  | 0.597 |
| walker |  | 5939 | 161 | plaintext config pnpm-workspace.yaml |  |  | 0.621 |
| ns | 6004 |  | 492 | app-server/ directory structure | 4.1 |  | 0.658 |
| walker |  | 6066 | 127 | README.md section #26 |  |  | 0.658 |
| walker |  | 6091 | 25 | packages/docs/src/resources/i18n.md section #0 |  |  | 0.658 |
| ns | 6118 |  | 114 | REST API surface (method + path per route) | 4.2 |  | 0.652 |
| walker |  | 6198 | 107 | headings outline in packages/docs/src/resources/brand-kit.md |  |  | 0.652 |
| walker |  | 6287 | 89 | declaration surface of packages/docs/src/components/credential-inputs.vue |  |  | 0.652 |
| walker |  | 6355 | 68 | package identity in packages/deploy-cloudflare/package.json |  |  | 0.652 |
| walker |  | 6372 | 17 | package runtime metadata in packages/deploy-cloudflare/package.json |  |  | 0.652 |
| walker |  | 6401 | 29 | package scripts in packages/deploy-cloudflare/package.json |  |  | 0.652 |
| walker |  | 6417 | 16 | export names surface in packages/lib/src/crypto/crypto.usecases.ts |  |  | 0.652 |
| walker |  | 6433 | 16 | export names surface in packages/lib/src/notes/notes.services.ts |  |  | 0.652 |
| walker |  | 6527 | 94 | declaration surface of packages/docs/src/components/toggle.vue |  |  | 0.652 |
| walker |  | 6616 | 89 | packages/crypto/README.md section #1 |  |  | 0.652 |
| ns | 6622 |  | 504 | server.ts: Hono app assembly | 4.3 |  | 0.626 |
| ns | 6704 |  | 82 | config.ts top-level section keys | 4.4 |  | 0.621 |
| walker |  | 6809 | 193 | headings outline in packages/docs/src/how-it-works.md |  |  | 0.621 |
| walker |  | 6886 | 77 | package identity in packages/app-client/package.json |  |  | 0.621 |
| walker |  | 6937 | 51 | package runtime metadata in packages/app-client/package.json |  |  | 0.621 |
| walker |  | 7014 | 77 | package identity in packages/app-server/package.json |  |  | 0.621 |
| walker |  | 7065 | 51 | package runtime metadata in packages/app-server/package.json |  |  | 0.621 |
| walker |  | 7142 | 77 | package identity in packages/docs/package.json |  |  | 0.621 |
| ns | 7149 |  | 445 | auth.middleware.ts: JWT gate | 4.5 |  | 0.598 |
| walker |  | 7156 | 14 | package entrypoints in packages/docs/package.json |  |  | 0.598 |
| walker |  | 7173 | 17 | package runtime metadata in packages/docs/package.json |  |  | 0.598 |
| walker |  | 7260 | 87 | package scripts in packages/docs/package.json |  |  | 0.598 |
| walker |  | 7291 | 31 | package runtime dependencies in packages/docs/package.json |  |  | 0.598 |
| walker |  | 7370 | 79 | package identity in packages/crypto/package.json |  |  | 0.598 |
| walker |  | 7421 | 51 | package runtime metadata in packages/crypto/package.json |  |  | 0.598 |
| ns | 7518 |  | 369 | notes.models.ts + notes.constants.ts | 4.6 |  | 0.584 |
| walker |  | 7572 | 151 | package scripts in packages/crypto/package.json |  |  | 0.584 |
| walker |  | 7603 | 31 | package runtime dependencies in packages/crypto/package.json |  |  | 0.584 |
| walker |  | 7682 | 79 | package identity in packages/lib/package.json |  |  | 0.584 |
| walker |  | 7733 | 51 | package runtime metadata in packages/lib/package.json |  |  | 0.584 |
| walker |  | 7857 | 124 | package entrypoints in packages/lib/package.json |  |  | 0.584 |
| walker |  | 8006 | 149 | package scripts in packages/lib/package.json |  |  | 0.584 |
| ns | 8047 |  | 529 | app-client/ directory structure | 5.1 |  | 0.631 |
| walker |  | 8206 | 200 | package scripts in packages/app-client/package.json |  |  | 0.631 |
| walker |  | 8286 | 80 | package identity in packages/cli/package.json |  |  | 0.631 |
| walker |  | 8337 | 51 | package runtime metadata in packages/cli/package.json |  |  | 0.631 |
| walker |  | 8421 | 84 | package entrypoints in packages/cli/package.json |  |  | 0.631 |
| ns | 8490 |  | 443 | routes.tsx: page routing table | 5.2 |  | 0.612 |
| walker |  | 8572 | 151 | package scripts in packages/cli/package.json |  |  | 0.612 |
| walker |  | 8622 | 50 | README.md section #22 |  |  | 0.612 |
| walker |  | 8769 | 147 | headings outline in packages/docs/src/self-hosting/docker-compose.md |  |  | 0.612 |
| walker |  | 8833 | 64 | README.md section #14 |  |  | 0.612 |
| walker |  | 8919 | 86 | packages/lib/README.md section #1 |  |  | 0.612 |
| walker |  | 8932 | 13 | listing of '.github/ISSUE_TEMPLATE' |  |  | 0.616 |
| walker |  | 8992 | 60 | packages/docs/src/how-it-works.md section #0 |  |  | 0.616 |
| ns | 9067 |  | 577 | notes.usecases.ts + notes.services.ts | 5.3 |  | 0.592 |
| walker |  | 9156 | 164 | headings outline in packages/docs/src/self-hosting/docker.md |  |  | 0.592 |
| walker |  | 9199 | 43 | packages/docs/src/self-hosting/docker.md section #0 |  |  | 0.592 |
| walker |  | 9224 | 25 | export names surface in packages/lib/src/api/api.models.ts |  |  | 0.592 |
| walker |  | 9276 | 52 | README.md section #28 |  |  | 0.592 |
| ns | 9422 |  | 355 | Locales roster | 5.4 |  | 0.585 |
| walker |  | 9557 | 281 | package scripts in packages/app-server/package.json |  |  | 0.585 |
| ns | 9558 |  | 136 | cli/ directory structure | 6.1 |  | 0.595 |
| walker |  | 9666 | 109 | json config packages/lib/tsconfig.json |  |  | 0.595 |
| ns | 9747 |  | 189 | cli.ts: subcommand registration | 6.2 |  | 0.588 |
| walker |  | 9787 | 121 | json config packages/crypto/tsconfig.json |  |  | 0.588 |
| walker |  | 9838 | 51 | packages/docs/src/self-hosting/docker-compose.md section #0 |  |  | 0.588 |
| walker |  | 9868 | 30 | export names surface in packages/lib/src/files/files.models.ts |  |  | 0.588 |
| walker |  | 9898 | 30 | export names surface in packages/lib/src/notes/notes.models.ts |  |  | 0.588 |
| ns | 9928 |  | 181 | docs/ + deploy-cloudflare/ directory structure | 7.1 |  | 0.598 |
| walker |  | 9929 | 31 | packages/docs/src/how-it-works.md section #13 |  |  | 0.598 |
| walker |  | 9944 | 15 | declaration surface of packages/docs/src/public/robots.txt |  |  | 0.598 |
| walker |  | 9998 | 54 | packages/docs/src/resources/brand-kit.md section #0 |  |  | 0.598 |
| ns | 10049 |  | 121 | docs: versioning policy | 7.2 |  | 0.596 |
