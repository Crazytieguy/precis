Score(3000)=0.592 I=0.826 C=0.424 ns_rows≤3K=16/47 grid(1000/1442/2080/3000/4327/6240/9000)=0.565/0.531/0.595/0.592/0.553/0.625/0.614

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 57 | 57 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 64 |  | 64 | What Enclosed is: README title, tagline, opening sentence | 1.1 |  | 0.000 |
| walker |  | 133 | 76 | Markdown::ReadmeHeadline { file: README.md } |  |  | 1.000 |
| ns | 141 |  | 77 | README introduction: the zero-knowledge guarantee and note options | 1.2 |  | 0.707 |
| walker |  | 158 | 25 | Fs::DirListing { dir: packages } |  |  | 0.707 |
| walker |  | 222 | 64 | Json::Identity { file: package.json } |  |  | 0.707 |
| ns | 223 |  | 82 | Repository shape: complete root listing and complete packages/ listing | 1.3 |  | 0.869 |
| walker |  | 239 | 17 | Fs::DirListing { dir: packages/docs } |  |  | 0.869 |
| walker |  | 258 | 19 | Fs::DirListing { dir: packages/deploy-cloudflare } |  |  | 0.870 |
| walker |  | 281 | 23 | Fs::DirListing { dir: packages/app-server } |  |  | 0.870 |
| walker |  | 307 | 26 | Fs::DirListing { dir: packages/crypto } |  |  | 0.870 |
| walker |  | 333 | 26 | Fs::DirListing { dir: packages/lib } |  |  | 0.871 |
| walker |  | 349 | 16 | Fs::DirListing { dir: packages/lib/src } |  |  | 0.873 |
| walker |  | 354 | 5 | Fs::DirListing { dir: packages/lib/src/files } |  |  | 0.874 |
| walker |  | 383 | 29 | Fs::DirListing { dir: packages/cli } |  |  | 0.874 |
| ns | 385 |  | 162 | README 'Project Structure': role of each workspace package | 1.4 |  | 0.761 |
| walker |  | 388 | 5 | Fs::DirListing { dir: packages/cli/bin } |  |  | 0.761 |
| walker |  | 409 | 21 | Fs::DirListing { dir: packages/app-server/src } |  |  | 0.762 |
| walker |  | 430 | 21 | Fs::DirListing { dir: packages/cli/src } |  |  | 0.762 |
| walker |  | 440 | 10 | Fs::DirListing { dir: packages/cli/src/shared } |  |  | 0.763 |
| walker |  | 451 | 11 | Fs::DirListing { dir: packages/cli/src/files } |  |  | 0.763 |
| walker |  | 463 | 12 | Fs::DirListing { dir: packages/cli/src/view-note } |  |  | 0.764 |
| walker |  | 478 | 15 | Fs::DirListing { dir: packages/app-server/src/modules } |  |  | 0.764 |
| walker |  | 509 | 31 | Fs::DirListing { dir: packages/docs/src } |  |  | 0.765 |
| walker |  | 519 | 10 | Fs::DirListing { dir: packages/docs/src/components } |  |  | 0.765 |
| walker |  | 530 | 11 | Fs::DirListing { dir: packages/docs/src/resources } |  |  | 0.766 |
| walker |  | 542 | 12 | Fs::DirListing { dir: packages/docs/src/data } |  |  | 0.766 |
| ns | 546 |  | 161 | pnpm-workspace.yaml in full: workspace glob and dependency catalog | 1.5 |  | 0.639 |
| walker |  | 556 | 14 | Fs::DirListing { dir: packages/docs/src/integrations } |  |  | 0.639 |
| walker |  | 588 | 32 | Fs::DirListing { dir: packages/crypto/src } |  |  | 0.641 |
| walker |  | 600 | 12 | Fs::DirListing { dir: packages/app-server/src/modules/shared } |  |  | 0.641 |
| walker |  | 604 | 4 | Fs::DirListing { dir: packages/app-server/src/modules/shared/utils } |  |  | 0.641 |
| walker |  | 667 | 63 | Fs::DirListing { dir: packages/app-client } |  |  | 0.641 |
| walker |  | 693 | 26 | Fs::DirListing { dir: packages/app-client/src } |  |  | 0.642 |
| walker |  | 697 | 4 | Fs::DirListing { dir: packages/app-client/src/assets } |  |  | 0.642 |
| walker |  | 708 | 11 | Fs::DirListing { dir: packages/app-client/e2e-tests } |  |  | 0.642 |
| ns | 726 |  | 180 | Root package.json: version, package manager, engines, release and docker scripts | 1.6 |  | 0.567 |
| walker |  | 759 | 51 | Json::Runtime { file: package.json } |  |  | 0.598 |
| walker |  | 772 | 13 | Fs::DirListing { dir: packages/app-server/src/modules/storage } |  |  | 0.598 |
| walker |  | 781 | 9 | Fs::DirListing { dir: packages/app-server/src/modules/shared/errors } |  |  | 0.598 |
| walker |  | 790 | 9 | Fs::DirListing { dir: packages/app-server/src/modules/shared/validation } |  |  | 0.598 |
| walker |  | 811 | 21 | Fs::DirListing { dir: packages/cli/src/config } |  |  | 0.599 |
| walker |  | 832 | 21 | Fs::DirListing { dir: packages/cli/src/create-note } |  |  | 0.600 |
| walker |  | 853 | 21 | Fs::DirListing { dir: packages/crypto/src/node } |  |  | 0.600 |
| walker |  | 874 | 21 | Fs::DirListing { dir: packages/crypto/src/web } |  |  | 0.601 |
| ns | 890 |  | 164 | README feature list, first half | 1.7 |  | 0.557 |
| walker |  | 895 | 21 | Fs::DirListing { dir: packages/lib/src/api } |  |  | 0.559 |
| walker |  | 906 | 11 | Fs::DirListing { dir: packages/docs/.vitepress } |  |  | 0.559 |
| walker |  | 931 | 25 | Fs::DirListing { dir: packages/lib/src/crypto } |  |  | 0.562 |
| walker |  | 943 | 12 | Fs::DirListing { dir: packages/app-client/src/scripts } |  |  | 0.562 |
| walker |  | 955 | 12 | Fs::DirListing { dir: packages/docs/.vitepress/theme } |  |  | 0.562 |
| walker |  | 982 | 27 | Fs::DirListing { dir: packages/lib/src/notes } |  |  | 0.565 |
| walker |  | 1011 | 29 | Fs::DirListing { dir: .github } |  |  | 0.565 |
| ns | 1076 |  | 186 | README feature list, second half | 1.8 |  | 0.532 |
| walker |  | 1092 | 81 | Fs::DirListing { dir: .github/workflows } |  |  | 0.533 |
| walker |  | 1121 | 29 | Fs::DirListing { dir: packages/app-client/src/modules } |  |  | 0.534 |
| walker |  | 1126 | 5 | Fs::DirListing { dir: packages/app-client/src/modules/theme } |  |  | 0.534 |
| walker |  | 1132 | 6 | Fs::DirListing { dir: packages/app-client/src/modules/ui } |  |  | 0.534 |
| walker |  | 1138 | 6 | Fs::DirListing { dir: packages/app-client/src/modules/ui/layouts } |  |  | 0.534 |
| walker |  | 1149 | 11 | Fs::DirListing { dir: packages/app-client/src/modules/docs } |  |  | 0.534 |
| walker |  | 1160 | 11 | Fs::DirListing { dir: packages/app-client/src/modules/files } |  |  | 0.535 |
| walker |  | 1175 | 15 | Fs::DirListing { dir: packages/app-client/src/modules/shared } |  |  | 0.535 |
| walker |  | 1179 | 4 | Fs::DirListing { dir: packages/app-client/src/modules/shared/hooks } |  |  | 0.535 |
| walker |  | 1183 | 4 | Fs::DirListing { dir: packages/app-client/src/modules/shared/style } |  |  | 0.535 |
| walker |  | 1188 | 5 | Fs::DirListing { dir: packages/app-client/src/modules/shared/utils } |  |  | 0.535 |
| walker |  | 1196 | 8 | Fs::DirListing { dir: packages/app-client/src/modules/shared/files } |  |  | 0.535 |
| walker |  | 1212 | 16 | Fs::DirListing { dir: packages/app-client/src/modules/config } |  |  | 0.535 |
| walker |  | 1230 | 18 | Fs::DirListing { dir: packages/app-client/src/modules/auth } |  |  | 0.536 |
| walker |  | 1236 | 6 | Fs::DirListing { dir: packages/app-client/src/modules/auth/pages } |  |  | 0.536 |
| ns | 1237 |  | 161 | CONTRIBUTING: local development setup commands | 1.9 |  | 0.464 |
| walker |  | 1314 | 78 | Json::Scripts { file: package.json } |  |  | 0.544 |
| walker |  | 1345 | 31 | Fs::DirListing { dir: packages/docs/src/self-hosting } |  |  | 0.545 |
| ns | 1399 |  | 162 | Self-host quickstart: docker run invocation plus the whole docker-compose.yml | 1.10 |  | 0.496 |
| walker |  | 1451 | 106 | Plaintext::Whole { file: docker-compose.yml } |  |  | 0.538 |
| walker |  | 1473 | 22 | Fs::DirListing { dir: packages/app-server/src/modules/app } |  |  | 0.539 |
| walker |  | 1478 | 5 | Fs::DirListing { dir: packages/app-server/src/modules/app/users } |  |  | 0.539 |
| walker |  | 1500 | 22 | Fs::DirListing { dir: packages/app-server/src/modules/tasks } |  |  | 0.540 |
| walker |  | 1515 | 15 | Fs::DirListing { dir: packages/app-server/src/modules/shared/logger } |  |  | 0.540 |
| walker |  | 1538 | 23 | Fs::DirListing { dir: packages/app-client/src/modules/i18n } |  |  | 0.540 |
| walker |  | 1561 | 23 | Fs::DirListing { dir: packages/crypto/src/node/encryption-algorithms } |  |  | 0.541 |
| walker |  | 1584 | 23 | Fs::DirListing { dir: packages/crypto/src/web/encryption-algorithms } |  |  | 0.542 |
| walker |  | 1648 | 64 | Fs::DirListing { dir: packages/app-client/public } |  |  | 0.542 |
| walker |  | 1672 | 24 | Fs::DirListing { dir: packages/lib/src/crypto/encryption-algorithms } |  |  | 0.544 |
| walker |  | 1698 | 26 | Fs::DirListing { dir: packages/lib/src/crypto/serialization } |  |  | 0.547 |
| walker |  | 1713 | 15 | Fs::DirListing { dir: packages/lib/src/crypto/serialization/cbor-array } |  |  | 0.549 |
| ns | 1758 |  | 359 | packages/lib/src/index.ts in full — the definitive @enclosed/lib export surface | 2.1 |  | 0.480 |
| walker |  | 1887 | 174 | Code::CodeKey { rung: Names, file: packages/lib/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.519 |
| walker |  | 1907 | 20 | Fs::DirListing { dir: packages/app-server/src/scripts } |  |  | 0.519 |
| walker |  | 1917 | 10 | Code::CodeKey { rung: Names, file: packages/app-client/src/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.519 |
| ns | 1943 |  | 185 | Complete file roster of packages/lib (package root and every src directory) | 2.2 |  | 0.595 |
| walker |  | 2115 | 198 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.595 |
| walker |  | 2192 | 77 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.649 |
| walker |  | 2213 | 21 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.649 |
| ns | 2219 |  | 276 | README 'How It Works': note creation, steps 1-7 | 2.3 |  | 0.628 |
| ns | 2400 |  | 181 | createNote: option names and defaults (notes.usecases.ts) | 2.4 |  | 0.595 |
| walker |  | 2552 | 339 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.658 |
| walker |  | 2602 | 50 | Fs::DirListing { dir: packages/crypto/src/encryption-algorithms } |  |  | 0.660 |
| walker |  | 2624 | 22 | Code::CodeKey { rung: Names, file: packages/cli/src/cli.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.661 |
| ns | 2658 |  | 258 | createNote body: encrypt, store, build URL | 2.5 | 2.4 | 0.612 |
| walker |  | 2715 | 91 | Code::CodeKey { rung: Decl, file: packages/cli/src/cli.ts, decl: 1, sub: 0, line: 6 } |  |  | 0.613 |
| walker |  | 2767 | 52 | Fs::DirListing { dir: packages/docs/src/public } |  |  | 0.613 |
| ns | 2853 |  | 195 | Note URL hash-fragment scheme: the 'pw' / 'dar' markers and the fragment builder | 2.6 |  | 0.592 |
| ns | 3077 |  | 224 | createNoteUrl and parseNoteUrl: link assembly and the reverse parse | 2.7 | 2.6 | 0.567 |
| walker |  | 3090 | 323 | Code::CodeKey { rung: Decl, file: packages/app-client/src/index.tsx, decl: 1, sub: 0, line: 14 } |  |  | 0.567 |
| walker |  | 3113 | 23 | Fs::DirListing { dir: packages/app-client/src/modules/shared/http } |  |  | 0.567 |
| walker |  | 3138 | 25 | Fs::DirListing { dir: packages/app-server/src/modules/app/config } |  |  | 0.568 |
| walker |  | 3177 | 39 | Fs::DirListing { dir: packages/app-client/src/modules/notes } |  |  | 0.569 |
| walker |  | 3191 | 14 | Fs::DirListing { dir: packages/app-client/src/modules/notes/components } |  |  | 0.569 |
| walker |  | 3205 | 14 | Fs::DirListing { dir: packages/app-client/src/modules/notes/pages } |  |  | 0.569 |
| walker |  | 3249 | 44 | Fs::DirListing { dir: packages/docs/src/public/logos } |  |  | 0.569 |
| walker |  | 3285 | 36 | Fs::DirListing { dir: packages/app-server/src/modules/app/middlewares } |  |  | 0.571 |
| walker |  | 3323 | 38 | Fs::DirListing { dir: packages/app-server/src/modules/storage/factories } |  |  | 0.571 |
| walker |  | 3365 | 42 | Fs::DirListing { dir: packages/app-server/src/modules/app/auth } |  |  | 0.573 |
| walker |  | 3373 | 8 | Fs::DirListing { dir: packages/app-server/src/modules/app/auth/e2e } |  |  | 0.574 |
| ns | 3390 |  | 313 | encryptNote: crypto primitives imported, options, and the encryption sequence | 2.8 |  | 0.545 |
| walker |  | 3436 | 63 | Fs::DirListing { dir: packages/app-server/src/modules/notes } |  |  | 0.547 |
| walker |  | 3455 | 19 | Fs::DirListing { dir: packages/app-server/src/modules/notes/tasks } |  |  | 0.548 |
| walker |  | 3542 | 87 | Fs::DirListing { dir: packages/app-client/src/locales } |  |  | 0.549 |
| ns | 3584 |  | 194 | Payload vocabulary: the algorithm and compression constants | 2.9 |  | 0.540 |
| walker |  | 3590 | 48 | Fs::DirListing { dir: packages/app-client/src/modules/ui/components } |  |  | 0.541 |
| walker |  | 3642 | 52 | Fs::DirListing { dir: packages/app-server/src/modules/notes/e2e } |  |  | 0.542 |
| ns | 3780 |  | 196 | Complete file roster of packages/crypto | 2.10 |  | 0.580 |
| ns | 4036 |  | 256 | @enclosed/crypto entry points: the eight exported names and the web/node swap | 2.11 |  | 0.562 |
| walker |  | 4161 | 519 | Plaintext::Whole { file: Dockerfile } |  |  | 0.562 |
| walker |  | 4221 | 60 | Json::Whole { file: renovate.json } |  |  | 0.562 |
| ns | 4311 |  | 275 | Key derivation parameters: generateBaseKey and deriveMasterKey (web implementation) | 2.12 | 2.11 | 0.545 |
| walker |  | 4384 | 163 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.571 |
| ns | 4520 |  | 209 | AES-256-GCM: the `iv:payload` ciphertext string format | 2.13 | 2.11 | 0.561 |
| walker |  | 4630 | 246 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.586 |
| ns | 4660 |  | 140 | The complete HTTP endpoint set of the server, one span per route registration | 3.1 |  | 0.577 |
| walker |  | 4791 | 161 | Plaintext::Whole { file: pnpm-workspace.yaml } |  |  | 0.616 |
| walker |  | 4874 | 83 | Plaintext::DeclSurface { file: packages/docs/src/components/credential-inputs.vue } |  |  | 0.616 |
| ns | 4877 |  | 217 | app-server file roster, part 1: package root, entry points, and the modules/app subtree | 3.2 |  | 0.653 |
| walker |  | 4889 | 15 | Plaintext::DeclSurface { file: packages/docs/src/public/robots.txt } |  |  | 0.653 |
| walker |  | 4957 | 68 | Json::Identity { file: packages/deploy-cloudflare/package.json } |  |  | 0.653 |
| walker |  | 4974 | 17 | Json::Runtime { file: packages/deploy-cloudflare/package.json } |  |  | 0.653 |
| walker |  | 5003 | 29 | Json::Scripts { file: packages/deploy-cloudflare/package.json } |  |  | 0.653 |
| walker |  | 5096 | 93 | Plaintext::DeclSurface { file: packages/docs/src/components/toggle.vue } |  |  | 0.653 |
| walker |  | 5106 | 10 | Code::CodeKey { rung: Names, file: packages/app-server/src/index.cloudflare.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.653 |
| ns | 5133 |  | 256 | app-server file roster, part 2: notes, storage, tasks and shared modules | 3.3 |  | 0.679 |
| walker |  | 5183 | 77 | Json::Identity { file: packages/app-client/package.json } |  |  | 0.679 |
| walker |  | 5234 | 51 | Json::Runtime { file: packages/app-client/package.json } |  |  | 0.679 |
| ns | 5361 |  | 228 | createServer: the ordered middleware stack and route registration | 3.4 | 3.1 | 0.661 |
| walker |  | 5434 | 200 | Json::Scripts { file: packages/app-client/package.json } |  |  | 0.661 |
| walker |  | 5511 | 77 | Json::Identity { file: packages/app-server/package.json } |  |  | 0.661 |
| walker |  | 5562 | 51 | Json::Runtime { file: packages/app-server/package.json } |  |  | 0.661 |
| walker |  | 5790 | 228 | Json::Dependencies { file: packages/app-server/package.json } |  |  | 0.661 |
| ns | 5858 |  | 497 | Every environment variable the server reads, with its config section | 3.5 |  | 0.638 |
| walker |  | 5867 | 77 | Json::Identity { file: packages/docs/package.json } |  |  | 0.638 |
| walker |  | 5881 | 14 | Json::Entry { file: packages/docs/package.json } |  |  | 0.638 |
| walker |  | 5898 | 17 | Json::Runtime { file: packages/docs/package.json } |  |  | 0.638 |
| walker |  | 5931 | 33 | Json::Dependencies { file: packages/docs/package.json } |  |  | 0.638 |
| walker |  | 6016 | 85 | Json::Scripts { file: packages/docs/package.json } |  |  | 0.638 |
| ns | 6084 |  | 226 | POST /api/notes: the zod request schema and the payload-size limit | 3.6 | 3.1 | 0.625 |
| walker |  | 6095 | 79 | Json::Identity { file: packages/crypto/package.json } |  |  | 0.625 |
| walker |  | 6128 | 33 | Json::Dependencies { file: packages/crypto/package.json } |  |  | 0.625 |
| walker |  | 6179 | 51 | Json::Runtime { file: packages/crypto/package.json } |  |  | 0.625 |
| ns | 6281 |  | 197 | The stored note record and the note repository's method set | 3.7 |  | 0.611 |
| walker |  | 6328 | 149 | Json::Scripts { file: packages/crypto/package.json } |  |  | 0.611 |
| walker |  | 6407 | 79 | Json::Identity { file: packages/lib/package.json } |  |  | 0.611 |
| walker |  | 6458 | 51 | Json::Runtime { file: packages/lib/package.json } |  |  | 0.611 |
| ns | 6484 |  | 203 | The three storage drivers behind the unstorage abstraction | 3.8 |  | 0.604 |
| walker |  | 6572 | 114 | Json::Dependencies { file: packages/lib/package.json } |  |  | 0.604 |
| walker |  | 6696 | 124 | Json::Entry { file: packages/lib/package.json } |  |  | 0.604 |
| ns | 6772 |  | 288 | Error catalogue: every note and auth error code with its status | 3.9 |  | 0.592 |
| walker |  | 6843 | 147 | Json::Scripts { file: packages/lib/package.json } |  |  | 0.592 |
| walker |  | 6923 | 80 | Json::Identity { file: packages/cli/package.json } |  |  | 0.592 |
| walker |  | 6974 | 51 | Json::Runtime { file: packages/cli/package.json } |  |  | 0.592 |
| ns | 6987 |  | 215 | Optional authentication: the two middlewares and the users source | 3.10 |  | 0.586 |
| walker |  | 7058 | 84 | Json::Entry { file: packages/cli/package.json } |  |  | 0.586 |
| ns | 7186 |  | 199 | Expired-note deletion task and its config wiring | 3.11 | 3.5 | 0.579 |
| walker |  | 7202 | 144 | Json::Dependencies { file: packages/cli/package.json } |  |  | 0.579 |
| walker |  | 7351 | 149 | Json::Scripts { file: packages/cli/package.json } |  |  | 0.579 |
| walker |  | 7365 | 14 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.579 |
| ns | 7406 |  | 220 | The two server entry points: node bootstrap and Cloudflare worker | 3.12 | 3.4 | 0.570 |
| ns | 7575 |  | 169 | cli.ts: the three subcommands and the citty entry point | 4.1 |  | 0.569 |
| walker |  | 7656 | 291 | Json::Dependencies { file: packages/app-client/package.json } |  |  | 0.569 |
| ns | 7705 |  | 130 | Complete file roster of packages/cli | 4.2 |  | 0.585 |
| walker |  | 7935 | 279 | Json::Scripts { file: packages/app-server/package.json } |  |  | 0.586 |
| ns | 7969 |  | 264 | `enclosed create`: every flag, its description and its short alias | 4.3 | 4.1 | 0.576 |
| walker |  | 7970 | 35 | Plaintext::DeclSurface { file: packages/app-client/public/robots.txt } |  |  | 0.576 |
| walker |  | 8002 | 32 | Code::CodeKey { rung: Names, file: packages/crypto/src/api-definition.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.576 |
| walker |  | 8153 | 151 | Code::CodeKey { rung: Decl, file: packages/crypto/src/api-definition.ts, decl: 1, sub: 0, line: 6 } |  |  | 0.576 |
| walker |  | 8195 | 42 | Plaintext::DeclSurface { file: packages/app-client/public/humans.txt } |  |  | 0.576 |
| ns | 8227 |  | 258 | `enclosed view`: arguments, and `enclosed config` set/get/delete/reset | 4.4 | 4.1 | 0.566 |
| ns | 8502 |  | 275 | app-client file roster, part 1: package root, entry files and the feature modules | 5.1 |  | 0.595 |
| walker |  | 8572 | 377 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.595 |
| walker |  | 8589 | 17 | Code::CodeKey { rung: Names, file: packages/app-client/src/routes.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.595 |
| walker |  | 8600 | 11 | Code::CodeKey { rung: Names, file: packages/docs/src/data/i18n.data.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.595 |
| walker |  | 8630 | 30 | Code::CodeKey { rung: Decl, file: packages/docs/src/data/i18n.data.ts, decl: 1, sub: 0, line: 59 } |  |  | 0.595 |
| walker |  | 8641 | 11 | Code::CodeKey { rung: Names, file: packages/docs/src/data/configuration.data.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.595 |
| walker |  | 8673 | 32 | Code::CodeKey { rung: Decl, file: packages/docs/src/data/configuration.data.ts, decl: 1, sub: 0, line: 55 } |  |  | 0.595 |
| walker |  | 8684 | 11 | Code::CodeKey { rung: Body, file: packages/docs/src/data/configuration.data.ts, decl: 2, sub: 0, line: 57 } |  |  | 0.595 |
| ns | 8731 |  | 229 | app-client file roster, part 2: shared layer, UI component library, locales and e2e tests | 5.2 |  | 0.614 |
| ns | 8974 |  | 243 | Client route table: the four routes and the components behind them | 5.3 |  | 0.603 |
| walker |  | 9148 | 464 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.614 |
| walker |  | 9173 | 25 | Code::CodeKey { rung: Names, file: packages/lib/src/api/api.client.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.614 |
| ns | 9210 |  | 236 | Client runtime configuration: the nine build-time config fields and their VITE_ variables | 5.4 |  | 0.609 |
| walker |  | 9263 | 90 | Code::CodeKey { rung: Decl, file: packages/lib/src/api/api.client.ts, decl: 1, sub: 0, line: 20 } |  |  | 0.609 |
| walker |  | 9304 | 41 | Code::CodeKey { rung: Names, file: packages/lib/src/crypto/crypto.usecases.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.610 |
| ns | 9334 |  | 124 | Complete .github listing: every CI/CD workflow and issue template | 6.1 |  | 0.617 |
| walker |  | 9416 | 112 | Code::CodeKey { rung: Decl, file: packages/lib/src/crypto/crypto.usecases.ts, decl: 1, sub: 0, line: 9 } |  |  | 0.619 |
| walker |  | 9530 | 114 | Code::CodeKey { rung: Decl, file: packages/lib/src/crypto/crypto.usecases.ts, decl: 2, sub: 0, line: 38 } |  |  | 0.619 |
| ns | 9578 |  | 244 | app-server package scripts: both runtime targets, tests, typecheck | 6.2 |  | 0.622 |
| ns | 9754 |  | 176 | Dockerfile: the two-stage image build and its runtime contract | 6.3 |  | 0.626 |
| ns | 9910 |  | 156 | Documentation site and Cloudflare deploy package: complete file rosters | 6.4 |  | 0.636 |
| walker |  | 9924 | 394 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.636 |
| walker |  | 9986 | 62 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.636 |
