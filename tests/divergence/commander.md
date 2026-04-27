scores: Sim=0.376 Reached=6/41 Early=3 Late=1 Partial=4 Missing=31 Used=9969/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 5 | 3 | 1 | 1 | 0.84 |
| 2 | 8 | 1 | 2 | 5 | 0.31 |
| 3 | 8 | 0 | 0 | 8 | 0.00 |
| 4 | 4 | 0 | 0 | 4 | 0.00 |
| 5 | 7 | 0 | 1 | 6 | 0.11 |
| 6 | 2 | 0 | 0 | 2 | 0.00 |
| 7 | 7 | 2 | 0 | 5 | 0.30 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 51 | — | — | 0.75 | partial | Package identity (name, version, description) | package identity in package.json (t=509, 3 atoms) |
| 1.2 | 153 | 102 | -51 | 1.00 | early | Top-level fs |  |
| 1.3 | 195 | 261 | +66 | 1.00 | late | lib/ + typings/ listings |  |
| 1.4 | 225 | 132 | -93 | 1.00 | early | Readme title + tagline | README headline in Readme.md (t=132, 2 atoms) |
| 1.5 | 300 | — | — | 0.44 | missing | Readme major H2 sections | Readme.md section #3 (t=6318, 23 atoms) |
| 2.1 | 383 | — | — | 0.00 | missing | index.js — class exports |  |
| 2.2 | 542 | — | — | 0.00 | missing | index.js — imports, program, factories |  |
| 2.3 | 710 | — | — | 0.73 | partial | Terminology doc — definitions | docs/terminology.md section #0 (t=6542, 8 atoms) |
| 2.4 | 791 | — | — | 0.71 | partial | Terminology doc — example | docs/terminology.md section #0 (t=6542, 5 atoms) |
| 2.5 | 1040 | — | — | 0.00 | missing | Quick Start — split subcommand |  |
| 2.6 | 1240 | — | — | 0.00 | missing | Quick Start — join subcommand and parse |  |
| 2.7 | 1376 | — | — | 0.00 | missing | esm.mjs wrapper |  |
| 3.1 | 2067 | — | — | 0.00 | missing | CommanderError + InvalidArgumentError class shapes |  |
| 3.2 | 2170 | — | — | 0.00 | missing | Argument class — method names |  |
| 3.3 | 2360 | — | — | 0.00 | missing | Option class — method names |  |
| 3.4 | 2477 | — | — | 0.00 | missing | Command class — registration & options method names |  |
| 3.5 | 2641 | — | — | 0.00 | missing | Command class — config & option-value method names |  |
| 3.6 | 2751 | — | — | 0.00 | missing | Command class — parsing & action method names |  |
| 3.7 | 2925 | — | — | 0.00 | missing | Command class — help/usage/info method names |  |
| 3.8 | 3205 | — | — | 0.00 | missing | Help class — public method names |  |
| 4.1 | 3497 | — | — | 0.00 | missing | Argument constructor body |  |
| 4.2 | 3559 | — | — | 0.00 | missing | humanReadableArgName |  |
| 4.3 | 3964 | — | — | 0.00 | missing | Option constructor body |  |
| 4.4 | 4284 | — | — | 0.00 | missing | splitOptionFlags — flag-parser core |  |
| 5.2b | 5411 | — | — | 0.00 | missing | _parseCommand body — action handler + hooks |  |
| 5.3b | 6799 | — | — | 0.00 | missing | parseOptions body — flag dispatch branches |  |
| 5.4b | 7597 | — | — | 0.00 | missing | addOption body — handleOptionValue closure |  |
| 5.1 | 4569 | — | — | 0.78 | partial | parsing-and-hooks doc | docs/parsing-and-hooks.md section #0 (t=6802, 18 atoms) |
| 5.2 | 5020 | — | — | 0.00 | missing | _parseCommand body — option pass + subcommand routing |  |
| 5.3 | 6016 | — | — | 0.00 | missing | parseOptions body — entry + main loop frame |  |
| 5.4 | 7221 | — | — | 0.00 | missing | addOption body — defaults + listener registration |  |
| 6.1b | 8479 | — | — | 0.00 | missing | Help formatHelp — options, global options, commands |  |
| 6.1 | 7964 | — | — | 0.00 | missing | Help formatHelp — usage + description + arguments |  |
| 7.1 | 8752 | — | — | 0.13 | missing | examples/ + docs/ listings |  |
| 7.2 | 9565 | — | — | 0.00 | missing | tests/ listing |  |
| 7.4 | 9679 | — | — | 0.00 | missing | Readme Options sub-headings |  |
| 7.5 | 9802 | — | — | 0.00 | missing | Readme Commands + Automated help sub-headings |  |
| 7.6 | 9921 | — | — | 0.00 | missing | Readme Bits-and-pieces sub-headings |  |
| 7.7 | 9995 | 6060 | -3935 | 1.00 | early | options-in-depth — table of contents | docs/options-in-depth.md section #0 (t=6060, 3 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 7 | 1637 | export at typings/index.d.ts:<n> |
| 4 | 1363 | Readme.md section #<n> |
| 5 | 728 | docs/deprecated.md section #<n> |
| 4 | 338 | CHANGELOG.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 825 | 1.00 | 825 | 4376 | Readme.md section #0 |
| 759 | 1.00 | 759 | 7805 | export at typings/index.d.ts:210 |
| 527 | 1.00 | 527 | 9651 | json config tsconfig.json |
| 410 | 1.00 | 410 | 5294 | export at typings/index.d.ts:95 |
| 352 | 1.00 | 352 | 2507 | export names surface in typings/index.d.ts |
| 340 | 1.00 | 340 | 8723 | docs/deprecated.md section #0 |
| 303 | 0.97 | 311 | 8347 | docs/release-policy.md section #0 |
| 259 | 1.00 | 259 | 1694 | package scripts in package.json |
| 256 | 1.00 | 256 | 2077 | package dependencies in package.json |
| 246 | 0.95 | 258 | 6318 | Readme.md section #3 |
| 239 | 1.00 | 239 | 4884 | json config tsconfig.ts.json |
| 223 | 0.97 | 231 | 8036 | Readme.md section #9 |
| 192 | 1.00 | 192 | 5784 | docs/help-in-depth.md section #0 |
| 190 | 0.96 | 197 | 1153 | headings outline in docs/deprecated.md |
| 176 | 1.00 | 176 | 3148 | export at typings/index.d.ts:48 |
| 165 | 1.00 | 165 | 6967 | json config package-support.json |
| 164 | 1.00 | 164 | 3551 | json config tsconfig.js.json |
| 163 | 1.00 | 163 | 3311 | headings outline in docs/zh-CN/不再推荐使用的功能.md |
| 162 | 0.74 | 220 | 509 | package identity in package.json |
| 141 | 0.66 | 215 | 6060 | docs/options-in-depth.md section #0 |
| 127 | 1.00 | 127 | 1821 | headings outline in docs/zh-CN/可变参数的选项.md |
| 121 | 1.00 | 121 | 2972 | export at typings/index.d.ts:354 |
| 116 | 1.00 | 116 | 5556 | CHANGELOG.md section #1 |
| 110 | 1.00 | 110 | 5404 | CONTRIBUTING.md section #1 |
| 106 | 1.00 | 106 | 785 | CHANGELOG.md section #0 |
| 106 | 1.00 | 106 | 9757 | docs/deprecated.md section #9 |
| 104 | 1.00 | 104 | 4535 | plaintext config .editorconfig |
| 102 | 1.00 | 102 | 9025 | docs/deprecated.md section #1 |
| 101 | 1.00 | 101 | 8923 | docs/deprecated.md section #10 |
| 79 | 1.00 | 79 | 7046 | docs/deprecated.md section #2 |
| 74 | 1.00 | 74 | 4609 | Readme_zh-CN.md section #7 |
| 73 | 1.00 | 73 | 884 | SECURITY.md section #0 |
| 69 | 0.88 | 78 | 2155 | Readme.md section #7 |
| 63 | 1.00 | 63 | 8822 | docs/zh-CN/不再推荐使用的功能.md section #10 |
| 63 | 1.00 | 63 | 2851 | export at typings/index.d.ts:40 |
| 61 | 1.00 | 61 | 5845 | CHANGELOG.md section #5 |
| 58 | 0.87 | 67 | 679 | headings outline in docs/options-in-depth.md |
| 55 | 1.00 | 55 | 4431 | CHANGELOG.md section #2 |
| 55 | 1.00 | 55 | 2788 | export at typings/index.d.ts:16 |
| 53 | 1.00 | 53 | 2733 | export at typings/index.d.ts:1094 |
