Score(3000)=0.544 I=0.802 C=0.368 ns_rows≤3K=20/55 grid(1000/1442/2080/3000/4327/6240/9000)=0.732/0.631/0.523/0.544/0.679/0.722/0.719

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 29 | 29 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 54 |  | 54 | Project identity, tagline, and target Minecraft/protocol version | 1.1 |  | 0.000 |
| walker |  | 67 | 38 | Fs::DirListing { dir: include } |  |  | 0.000 |
| ns | 83 |  | 29 | Complete repository root listing | 1.2 |  | 0.518 |
| walker |  | 115 | 48 | Fs::DirListing { dir: src } |  |  | 0.654 |
| walker |  | 122 | 7 | Fs::DirListing { dir: .github } |  |  | 0.654 |
| walker |  | 126 | 4 | Fs::DirListing { dir: .github/workflows } |  |  | 0.654 |
| ns | 148 |  | 65 | Stated project goal and priority ordering | 1.3 |  | 0.623 |
| walker |  | 224 | 98 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.847 |
| ns | 234 |  | 86 | Complete src/ and include/ listings | 1.4 |  | 0.807 |
| walker |  | 275 | 51 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.825 |
| ns | 285 |  | 51 | All README H2 section headings | 1.5 |  | 0.817 |
| walker |  | 338 | 63 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 1.000 |
| ns | 343 |  | 58 | .gitignore in full — which sources are generated and absent | 1.6 |  | 0.904 |
| ns | 489 |  | 146 | build.sh: the registry prerequisite check and the actual compile command | 1.7 |  | 0.811 |
| ns | 595 |  | 106 | src/CMakeLists.txt in full — the ESP-IDF/PlatformIO build | 1.8 |  | 0.746 |
| ns | 665 |  | 70 | Connection state constants (STATE_NONE through STATE_PLAY) | 2.1 |  | 0.707 |
| walker |  | 867 | 529 | Plaintext::Whole { file: build.sh } |  |  | 0.800 |
| walker |  | 883 | 16 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.801 |
| ns | 885 |  | 220 | packets.h — serverbound declarations, connection and world interaction | 2.2 |  | 0.732 |
| ns | 1038 |  | 153 | packets.h — remainder of the serverbound declarations | 2.3 |  | 0.678 |
| walker |  | 1212 | 329 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: true } |  |  | 0.679 |
| ns | 1265 |  | 227 | packets.h — clientbound declarations, login and configuration phase | 2.4 |  | 0.631 |
| walker |  | 1331 | 119 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.631 |
| ns | 1560 |  | 295 | packets.h — clientbound declarations, world and inventory | 2.5 |  | 0.591 |
| walker |  | 1611 | 280 | Code::CodeKey { rung: Names, file: include/globals.h, decl: 0, sub: 0, line: 0 } |  |  | 0.594 |
| walker |  | 1620 | 9 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 1, sub: 0, line: 7 } |  |  | 0.594 |
| walker |  | 1630 | 10 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 4, sub: 0, line: 11 } |  |  | 0.594 |
| walker |  | 1641 | 11 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 9, sub: 0, line: 26 } |  |  | 0.594 |
| walker |  | 1652 | 11 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 10, sub: 0, line: 29 } |  |  | 0.594 |
| walker |  | 1665 | 13 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 14, sub: 0, line: 41 } |  |  | 0.594 |
| walker |  | 1679 | 14 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 7, sub: 0, line: 19 } |  |  | 0.594 |
| walker |  | 1693 | 14 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 12, sub: 0, line: 35 } |  |  | 0.594 |
| walker |  | 1710 | 17 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 13, sub: 0, line: 38 } |  |  | 0.594 |
| walker |  | 1729 | 19 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 18, sub: 0, line: 56 } |  |  | 0.594 |
| walker |  | 1756 | 27 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 11, sub: 0, line: 32 } |  |  | 0.594 |
| walker |  | 1783 | 27 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 15, sub: 0, line: 45 } |  |  | 0.594 |
| walker |  | 1814 | 31 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 16, sub: 0, line: 49 } |  |  | 0.594 |
| ns | 1849 |  | 289 | packets.h — clientbound declarations, entities, health and registries | 2.6 |  | 0.558 |
| walker |  | 1851 | 37 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 17, sub: 0, line: 53 } |  |  | 0.558 |
| walker |  | 1889 | 38 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 8, sub: 0, line: 23 } |  |  | 0.558 |
| ns | 1949 |  | 100 | varnum.h in full — VarInt encoding primitives and error sentinel | 2.7 |  | 0.541 |
| ns | 2019 |  | 70 | globals.h — BlockChange record and the packed-struct pragma | 3.1 |  | 0.523 |
| walker |  | 2188 | 299 | Code::CodeKey { rung: Names, file: include/globals.h, decl: 0, sub: 1, line: 0 } |  |  | 0.563 |
| walker |  | 2196 | 8 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 19, sub: 0, line: 59 } |  |  | 0.563 |
| walker |  | 2206 | 10 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 21, sub: 0, line: 66 } |  |  | 0.563 |
| walker |  | 2219 | 13 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 26, sub: 0, line: 106 } |  |  | 0.563 |
| walker |  | 2246 | 27 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 23, sub: 0, line: 75 } |  |  | 0.563 |
| ns | 2269 |  | 250 | globals.h — PlayerData fields (identity through inventory) | 3.2 |  | 0.516 |
| walker |  | 2279 | 33 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 20, sub: 0, line: 63 } |  |  | 0.516 |
| walker |  | 2331 | 52 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 25, sub: 0, line: 103 } |  |  | 0.516 |
| walker |  | 2385 | 54 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 22, sub: 0, line: 71 } |  |  | 0.516 |
| ns | 2526 |  | 257 | globals.h — PlayerData flag bits and the overloaded flagval fields | 3.3 | 3.2 | 0.489 |
| walker |  | 2539 | 154 | Code::CodeKey { rung: Names, file: include/serialize.h, decl: 0, sub: 0, line: 0 } |  |  | 0.490 |
| walker |  | 2546 | 7 | Code::CodeKey { rung: Decl, file: include/serialize.h, decl: 6, sub: 0, line: 12 } |  |  | 0.490 |
| walker |  | 2553 | 7 | Code::CodeKey { rung: Decl, file: include/serialize.h, decl: 10, sub: 0, line: 18 } |  |  | 0.490 |
| walker |  | 2566 | 13 | Code::CodeKey { rung: Decl, file: include/serialize.h, decl: 1, sub: 0, line: 6 } |  |  | 0.490 |
| walker |  | 2581 | 15 | Code::CodeKey { rung: Doc, file: include/serialize.h, decl: 6, sub: 0, line: 12 } |  |  | 0.490 |
| ns | 2756 |  | 230 | globals.h — MobData and EntityData | 3.4 |  | 0.459 |
| walker |  | 2797 | 216 | Code::CodeKey { rung: Names, file: include/globals.h, decl: 0, sub: 2, line: 0 } |  |  | 0.468 |
| walker |  | 2802 | 5 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 42, sub: 0, line: 186 } |  |  | 0.468 |
| walker |  | 2810 | 8 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 41, sub: 0, line: 184 } |  |  | 0.469 |
| walker |  | 2831 | 21 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 47, sub: 0, line: 255 } |  |  | 0.469 |
| walker |  | 2865 | 34 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 44, sub: 0, line: 191 } |  |  | 0.487 |
| walker |  | 2914 | 49 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 48, sub: 0, line: 260 } |  |  | 0.498 |
| ns | 2994 |  | 238 | globals.h — all extern global state declarations | 3.5 |  | 0.544 |
| walker |  | 3045 | 131 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 46, sub: 0, line: 240 } |  |  | 0.598 |
| walker |  | 3145 | 100 | Code::CodeKey { rung: Names, file: include/varnum.h, decl: 0, sub: 0, line: 0 } |  |  | 0.622 |
| ns | 3344 |  | 350 | build_registries.js — the template that generates include/registries.h | 3.6 |  | 0.588 |
| walker |  | 3356 | 211 | Code::CodeKey { rung: Names, file: include/tools.h, decl: 0, sub: 0, line: 0 } |  |  | 0.589 |
| walker |  | 3373 | 17 | Code::CodeKey { rung: Body, file: include/tools.h, decl: 1, sub: 0, line: 8 } |  |  | 0.589 |
| walker |  | 3398 | 25 | Code::CodeKey { rung: Body, file: include/tools.h, decl: 2, sub: 0, line: 11 } |  |  | 0.589 |
| walker |  | 3628 | 230 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 45, sub: 0, line: 200 } |  |  | 0.653 |
| ns | 3704 |  | 360 | globals.h — configuration macro roster, part 1 (world and player sizing) | 3.7 |  | 0.674 |
| walker |  | 3888 | 260 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 45, sub: 1, line: 200 } |  |  | 0.721 |
| ns | 3978 |  | 274 | globals.h — configuration macro roster, part 2 (feature toggles, including the commented-out ones) | 3.8 |  | 0.694 |
| walker |  | 4155 | 267 | Code::CodeKey { rung: Names, file: include/tools.h, decl: 0, sub: 1, line: 0 } |  |  | 0.698 |
| walker |  | 4164 | 9 | Code::CodeKey { rung: Decl, file: include/tools.h, decl: 26, sub: 0, line: 43 } |  |  | 0.698 |
| walker |  | 4176 | 12 | Code::CodeKey { rung: Decl, file: include/tools.h, decl: 27, sub: 0, line: 46 } |  |  | 0.678 |
| ns | 4176 |  | 198 | src/globals.c — definitions of the global state, MOTD and brand | 3.9 |  | 0.678 |
| ns | 4260 |  | 84 | README Configuration section — where the knobs live | 3.10 |  | 0.679 |
| ns | 4402 |  | 142 | README Configuration — the maintainer's tuning guidance | 3.11 |  | 0.681 |
| ns | 4488 |  | 86 | build_registries.js — the complete biome list | 3.12 |  | 0.671 |
| ns | 4721 |  | 233 | procedures.h — client state and player lifecycle API | 4.1 |  | 0.654 |
| walker |  | 4866 | 690 | Plaintext::Whole { file: extract_registries.sh } |  |  | 0.654 |
| ns | 4985 |  | 264 | procedures.h — metadata broadcast, slot mapping and block predicates | 4.2 |  | 0.637 |
| walker |  | 5077 | 211 | Code::CodeKey { rung: Names, file: include/packets.h, decl: 0, sub: 0, line: 0 } |  |  | 0.662 |
| walker |  | 5086 | 9 | Code::CodeKey { rung: Doc, file: include/packets.h, decl: 1, sub: 0, line: 5 } |  |  | 0.665 |
| ns | 5304 |  | 319 | procedures.h — mining, actions, fluids, mobs, tick and entity-data API | 4.3 |  | 0.647 |
| walker |  | 5306 | 220 | Code::CodeKey { rung: Names, file: include/procedures.h, decl: 0, sub: 0, line: 0 } |  |  | 0.672 |
| ns | 5415 |  | 111 | worldgen.h — ChunkAnchor and ChunkFeature | 4.4 |  | 0.659 |
| walker |  | 5519 | 213 | Code::CodeKey { rung: Names, file: include/procedures.h, decl: 0, sub: 1, line: 0 } |  |  | 0.678 |
| ns | 5587 |  | 172 | worldgen.h — the complete generation API and the shared chunk_section buffer | 4.5 |  | 0.669 |
| walker |  | 5715 | 196 | Code::CodeKey { rung: Names, file: include/procedures.h, decl: 0, sub: 2, line: 0 } |  |  | 0.685 |
| ns | 5794 |  | 207 | tools.h — socket I/O and the byte-order writers | 4.6 |  | 0.691 |
| walker |  | 5907 | 192 | Code::CodeKey { rung: Names, file: include/procedures.h, decl: 0, sub: 3, line: 0 } |  |  | 0.715 |
| ns | 5991 |  | 197 | tools.h — the readers, string helpers and RNG | 4.7 |  | 0.722 |
| walker |  | 6076 | 169 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.722 |
| ns | 6129 |  | 138 | tools.h — the inline math helpers and the platform time shim | 4.8 |  | 0.722 |
| ns | 6325 |  | 196 | serialize.h in full — persistence API and its compile-time no-op fallback | 4.9 |  | 0.727 |
| ns | 6410 |  | 85 | crafting.h and structures.h in full — the two smallest module APIs | 4.10 |  | 0.722 |
| walker |  | 6435 | 359 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: true } |  |  | 0.722 |
| ns | 6499 |  | 89 | main.c — the project's own module include list | 5.1 |  | 0.715 |
| walker |  | 6655 | 220 | Code::CodeKey { rung: Names, file: include/worldgen.h, decl: 0, sub: 0, line: 0 } |  |  | 0.731 |
| walker |  | 6689 | 34 | Code::CodeKey { rung: Decl, file: include/worldgen.h, decl: 1, sub: 0, line: 6 } |  |  | 0.737 |
| walker |  | 6723 | 34 | Code::CodeKey { rung: Decl, file: include/worldgen.h, decl: 2, sub: 0, line: 13 } |  |  | 0.746 |
| ns | 6755 |  | 256 | main.c — the maintainer's design note on the packet handlers, and handlePacket's signature | 5.2 |  | 0.732 |
| walker |  | 6959 | 236 | Code::CodeKey { rung: Names, file: include/packets.h, decl: 0, sub: 1, line: 0 } |  |  | 0.756 |
| walker |  | 6966 | 7 | Code::CodeKey { rung: Doc, file: include/packets.h, decl: 22, sub: 0, line: 28 } |  |  | 0.757 |
| ns | 7081 |  | 326 | main.c — packet 0x00 dispatch: handshake, status, login, configuration | 5.3 |  | 0.735 |
| walker |  | 7160 | 194 | Code::CodeKey { rung: Names, file: include/packets.h, decl: 0, sub: 2, line: 0 } |  |  | 0.754 |
| ns | 7350 |  | 269 | main.c — dispatch table, packet ids 0x07 through 0x19 | 5.4 |  | 0.739 |
| walker |  | 7380 | 220 | Code::CodeKey { rung: Names, file: include/packets.h, decl: 0, sub: 3, line: 0 } |  |  | 0.755 |
| walker |  | 7574 | 194 | Code::CodeKey { rung: Names, file: include/packets.h, decl: 0, sub: 4, line: 0 } |  |  | 0.765 |
| walker |  | 7687 | 113 | Code::CodeKey { rung: Names, file: include/packets.h, decl: 0, sub: 5, line: 0 } |  |  | 0.779 |
| ns | 7692 |  | 342 | main.c — dispatch table, the movement case group and ids 0x28 through the default | 5.5 |  | 0.759 |
| walker |  | 7792 | 105 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 24, sub: 0, line: 93 } |  |  | 0.759 |
| walker |  | 7838 | 46 | Code::CodeKey { rung: Names, file: include/crafting.h, decl: 0, sub: 0, line: 0 } |  |  | 0.760 |
| walker |  | 7862 | 24 | Code::CodeKey { rung: Names, file: include/structures.h, decl: 0, sub: 0, line: 0 } |  |  | 0.762 |
| ns | 8048 |  | 356 | main.c — the single-threaded accept and tick round-robin loop | 5.6 |  | 0.740 |
| walker |  | 8060 | 198 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.741 |
| walker |  | 8166 | 106 | Plaintext::DeclSurface { file: src/CMakeLists.txt } |  |  | 0.757 |
| ns | 8329 |  | 281 | main.c — the ESP32 entry points: FreeRTOS task, WiFi event handler, app_main | 5.7 |  | 0.741 |
| walker |  | 8441 | 275 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.741 |
| ns | 8553 |  | 224 | procedures.c — definition locations, part 1 (state, players, slots, block changes) | 6.1 |  | 0.727 |
| walker |  | 8669 | 228 | Code::CodeKey { rung: Names, file: src/globals.c, decl: 0, sub: 0, line: 0 } |  |  | 0.737 |
| walker |  | 8727 | 58 | Code::CodeKey { rung: Names, file: src/varnum.c, decl: 0, sub: 0, line: 0 } |  |  | 0.737 |
| ns | 8770 |  | 217 | procedures.c — definition locations, part 2 (mining, predicates, armour, eating, fluids) | 6.2 |  | 0.726 |
| walker |  | 8788 | 61 | Code::CodeKey { rung: Body, file: src/varnum.c, decl: 2, sub: 0, line: 34 } |  |  | 0.726 |
| walker |  | 8890 | 102 | Code::CodeKey { rung: Body, file: src/varnum.c, decl: 3, sub: 0, line: 43 } |  |  | 0.726 |
| ns | 8902 |  | 132 | procedures.c — definition locations, part 3 (actions, mobs, tick, entity data) | 6.3 |  | 0.719 |
| walker |  | 8957 | 67 | Code::CodeKey { rung: Names, file: src/crafting.c, decl: 0, sub: 0, line: 0 } |  |  | 0.719 |
| walker |  | 8985 | 28 | Code::CodeKey { rung: Decl, file: src/crafting.c, decl: 2, sub: 0, line: 349 } |  |  | 0.719 |
| ns | 9117 |  | 215 | worldgen.c — every definition, including the five private generator stages | 6.4 |  | 0.710 |
| walker |  | 9219 | 234 | Code::CodeKey { rung: Names, file: src/packets.c, decl: 0, sub: 0, line: 0 } |  |  | 0.710 |
| walker |  | 9230 | 11 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 2, sub: 0, line: 49 } |  |  | 0.710 |
| walker |  | 9241 | 11 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 3, sub: 0, line: 66 } |  |  | 0.710 |
| walker |  | 9252 | 11 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 4, sub: 0, line: 85 } |  |  | 0.710 |
| walker |  | 9263 | 11 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 9, sub: 0, line: 183 } |  |  | 0.710 |
| walker |  | 9275 | 12 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 10, sub: 0, line: 190 } |  |  | 0.710 |
| walker |  | 9288 | 13 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 6, sub: 0, line: 137 } |  |  | 0.710 |
| walker |  | 9301 | 13 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 7, sub: 0, line: 151 } |  |  | 0.710 |
| ns | 9304 |  | 187 | serialize.c — the world file path and all five persistence entry points | 6.5 |  | 0.702 |
| walker |  | 9314 | 13 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 8, sub: 0, line: 166 } |  |  | 0.702 |
| walker |  | 9327 | 13 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 11, sub: 0, line: 245 } |  |  | 0.702 |
| walker |  | 9342 | 15 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 1, sub: 0, line: 28 } |  |  | 0.702 |
| ns | 9485 |  | 181 | packets.c — the chat command surface (!msg and !help) | 6.6 |  | 0.696 |
| walker |  | 9573 | 231 | Code::CodeKey { rung: Names, file: src/packets.c, decl: 0, sub: 1, line: 0 } |  |  | 0.696 |
| walker |  | 9584 | 11 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 14, sub: 0, line: 300 } |  |  | 0.696 |
| walker |  | 9595 | 11 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 20, sub: 0, line: 471 } |  |  | 0.696 |
| walker |  | 9607 | 12 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 16, sub: 0, line: 323 } |  |  | 0.696 |
| walker |  | 9619 | 12 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 19, sub: 0, line: 444 } |  |  | 0.696 |
| walker |  | 9632 | 13 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 12, sub: 0, line: 275 } |  |  | 0.696 |
| walker |  | 9646 | 14 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 17, sub: 0, line: 332 } |  |  | 0.696 |
| walker |  | 9661 | 15 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 13, sub: 0, line: 287 } |  |  | 0.696 |
| walker |  | 9676 | 15 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 18, sub: 0, line: 433 } |  |  | 0.696 |
| walker |  | 9695 | 19 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 15, sub: 0, line: 314 } |  |  | 0.696 |
| ns | 9744 |  | 259 | crafting.c — the registerSmeltingRecipe macro and the complete recipe table | 6.7 |  | 0.688 |
| ns | 9771 |  | 27 | Complete .github listings (workflow and issue templates) | 7.1 |  | 0.690 |
| ns | 9877 |  | 106 | README Contribution — the maintainer's rules for changes | 7.2 |  | 0.692 |
| walker |  | 9922 | 227 | Code::CodeKey { rung: Names, file: src/packets.c, decl: 0, sub: 2, line: 0 } |  |  | 0.692 |
| ns | 9931 |  | 54 | extract_registries.sh — the top-level registry extraction sequence | 7.3 |  | 0.689 |
| walker |  | 9933 | 11 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 22, sub: 0, line: 488 } |  |  | 0.689 |
| walker |  | 9944 | 11 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 23, sub: 0, line: 512 } |  |  | 0.689 |
| walker |  | 9955 | 11 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 24, sub: 0, line: 528 } |  |  | 0.689 |
| ns | 9959 |  | 28 | LICENSE — the license identity line | 7.4 |  | 0.688 |
| walker |  | 9966 | 11 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 26, sub: 0, line: 577 } |  |  | 0.688 |
| walker |  | 9978 | 12 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 25, sub: 0, line: 545 } |  |  | 0.688 |
| walker |  | 9990 | 12 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 27, sub: 0, line: 707 } |  |  | 0.688 |
