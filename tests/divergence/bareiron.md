Score(3000)=0.610 I=0.807 C=0.461 ns_rows≤3K=20/55 grid(1000/1442/2080/3000/4327/6240/9000)=0.649/0.563/0.513/0.610/0.685/0.824/0.727

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
| walker |  | 354 | 16 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.905 |
| ns | 489 |  | 146 | build.sh: the registry prerequisite check and the actual compile command | 1.7 |  | 0.811 |
| ns | 595 |  | 106 | src/CMakeLists.txt in full — the ESP-IDF/PlatformIO build | 1.8 |  | 0.747 |
| ns | 665 |  | 70 | Connection state constants (STATE_NONE through STATE_PLAY) | 2.1 |  | 0.707 |
| walker |  | 683 | 329 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: true } |  |  | 0.708 |
| walker |  | 802 | 119 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.708 |
| ns | 885 |  | 220 | packets.h — serverbound declarations, connection and world interaction | 2.2 |  | 0.648 |
| ns | 1038 |  | 153 | packets.h — remainder of the serverbound declarations | 2.3 |  | 0.600 |
| walker |  | 1082 | 280 | Code::CodeKey { rung: Names, file: include/globals.h, decl: 0, sub: 0, line: 0 } |  |  | 0.603 |
| walker |  | 1091 | 9 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 1, sub: 0, line: 7 } |  |  | 0.603 |
| walker |  | 1101 | 10 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 4, sub: 0, line: 11 } |  |  | 0.604 |
| walker |  | 1112 | 11 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 9, sub: 0, line: 26 } |  |  | 0.604 |
| walker |  | 1123 | 11 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 10, sub: 0, line: 29 } |  |  | 0.604 |
| walker |  | 1136 | 13 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 14, sub: 0, line: 41 } |  |  | 0.604 |
| walker |  | 1150 | 14 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 7, sub: 0, line: 19 } |  |  | 0.604 |
| walker |  | 1164 | 14 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 12, sub: 0, line: 35 } |  |  | 0.604 |
| walker |  | 1181 | 17 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 13, sub: 0, line: 38 } |  |  | 0.604 |
| walker |  | 1200 | 19 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 18, sub: 0, line: 56 } |  |  | 0.604 |
| walker |  | 1227 | 27 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 11, sub: 0, line: 32 } |  |  | 0.604 |
| walker |  | 1254 | 27 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 15, sub: 0, line: 45 } |  |  | 0.604 |
| ns | 1265 |  | 227 | packets.h — clientbound declarations, login and configuration phase | 2.4 |  | 0.561 |
| walker |  | 1285 | 31 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 16, sub: 0, line: 49 } |  |  | 0.561 |
| walker |  | 1322 | 37 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 17, sub: 0, line: 53 } |  |  | 0.561 |
| walker |  | 1360 | 38 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 8, sub: 0, line: 23 } |  |  | 0.561 |
| ns | 1560 |  | 295 | packets.h — clientbound declarations, world and inventory | 2.5 |  | 0.525 |
| walker |  | 1659 | 299 | Code::CodeKey { rung: Names, file: include/globals.h, decl: 0, sub: 1, line: 0 } |  |  | 0.574 |
| walker |  | 1667 | 8 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 19, sub: 0, line: 59 } |  |  | 0.574 |
| walker |  | 1677 | 10 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 21, sub: 0, line: 66 } |  |  | 0.574 |
| walker |  | 1690 | 13 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 26, sub: 0, line: 106 } |  |  | 0.574 |
| walker |  | 1717 | 27 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 23, sub: 0, line: 75 } |  |  | 0.574 |
| walker |  | 1750 | 33 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 20, sub: 0, line: 63 } |  |  | 0.574 |
| walker |  | 1802 | 52 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 25, sub: 0, line: 103 } |  |  | 0.574 |
| ns | 1849 |  | 289 | packets.h — clientbound declarations, entities, health and registries | 2.6 |  | 0.538 |
| walker |  | 1856 | 54 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 22, sub: 0, line: 71 } |  |  | 0.538 |
| ns | 1949 |  | 100 | varnum.h in full — VarInt encoding primitives and error sentinel | 2.7 |  | 0.522 |
| ns | 2019 |  | 70 | globals.h — BlockChange record and the packed-struct pragma | 3.1 |  | 0.504 |
| walker |  | 2072 | 216 | Code::CodeKey { rung: Names, file: include/globals.h, decl: 0, sub: 2, line: 0 } |  |  | 0.512 |
| walker |  | 2077 | 5 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 42, sub: 0, line: 186 } |  |  | 0.513 |
| walker |  | 2085 | 8 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 41, sub: 0, line: 184 } |  |  | 0.513 |
| walker |  | 2106 | 21 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 47, sub: 0, line: 255 } |  |  | 0.513 |
| walker |  | 2140 | 34 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 44, sub: 0, line: 191 } |  |  | 0.537 |
| walker |  | 2189 | 49 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 48, sub: 0, line: 260 } |  |  | 0.538 |
| ns | 2269 |  | 250 | globals.h — PlayerData fields (identity through inventory) | 3.2 |  | 0.493 |
| walker |  | 2320 | 131 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 46, sub: 0, line: 240 } |  |  | 0.497 |
| walker |  | 2474 | 154 | Code::CodeKey { rung: Names, file: include/serialize.h, decl: 0, sub: 0, line: 0 } |  |  | 0.498 |
| walker |  | 2481 | 7 | Code::CodeKey { rung: Decl, file: include/serialize.h, decl: 6, sub: 0, line: 12 } |  |  | 0.498 |
| walker |  | 2488 | 7 | Code::CodeKey { rung: Decl, file: include/serialize.h, decl: 10, sub: 0, line: 18 } |  |  | 0.498 |
| walker |  | 2501 | 13 | Code::CodeKey { rung: Decl, file: include/serialize.h, decl: 1, sub: 0, line: 6 } |  |  | 0.499 |
| walker |  | 2516 | 15 | Code::CodeKey { rung: Doc, file: include/serialize.h, decl: 6, sub: 0, line: 12 } |  |  | 0.499 |
| ns | 2526 |  | 257 | globals.h — PlayerData flag bits and the overloaded flagval fields | 3.3 | 3.2 | 0.473 |
| walker |  | 2616 | 100 | Code::CodeKey { rung: Names, file: include/varnum.h, decl: 0, sub: 0, line: 0 } |  |  | 0.508 |
| ns | 2756 |  | 230 | globals.h — MobData and EntityData | 3.4 |  | 0.549 |
| walker |  | 2827 | 211 | Code::CodeKey { rung: Names, file: include/tools.h, decl: 0, sub: 0, line: 0 } |  |  | 0.551 |
| walker |  | 2844 | 17 | Code::CodeKey { rung: Body, file: include/tools.h, decl: 1, sub: 0, line: 8 } |  |  | 0.551 |
| walker |  | 2869 | 25 | Code::CodeKey { rung: Body, file: include/tools.h, decl: 2, sub: 0, line: 11 } |  |  | 0.551 |
| ns | 2994 |  | 238 | globals.h — all extern global state declarations | 3.5 |  | 0.586 |
| walker |  | 3099 | 230 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 45, sub: 0, line: 200 } |  |  | 0.654 |
| ns | 3344 |  | 350 | build_registries.js — the template that generates include/registries.h | 3.6 |  | 0.618 |
| walker |  | 3359 | 260 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 45, sub: 1, line: 200 } |  |  | 0.671 |
| walker |  | 3579 | 220 | Code::CodeKey { rung: Names, file: include/worldgen.h, decl: 0, sub: 0, line: 0 } |  |  | 0.672 |
| walker |  | 3613 | 34 | Code::CodeKey { rung: Decl, file: include/worldgen.h, decl: 1, sub: 0, line: 6 } |  |  | 0.673 |
| walker |  | 3647 | 34 | Code::CodeKey { rung: Decl, file: include/worldgen.h, decl: 2, sub: 0, line: 13 } |  |  | 0.674 |
| ns | 3704 |  | 360 | globals.h — configuration macro roster, part 1 (world and player sizing) | 3.7 |  | 0.693 |
| walker |  | 3914 | 267 | Code::CodeKey { rung: Names, file: include/tools.h, decl: 0, sub: 1, line: 0 } |  |  | 0.697 |
| walker |  | 3923 | 9 | Code::CodeKey { rung: Decl, file: include/tools.h, decl: 26, sub: 0, line: 43 } |  |  | 0.697 |
| walker |  | 3935 | 12 | Code::CodeKey { rung: Decl, file: include/tools.h, decl: 27, sub: 0, line: 46 } |  |  | 0.697 |
| ns | 3978 |  | 274 | globals.h — configuration macro roster, part 2 (feature toggles, including the commented-out ones) | 3.8 |  | 0.672 |
| walker |  | 4049 | 114 | Plaintext::DeclSurface { file: extract_registries.sh } |  |  | 0.672 |
| ns | 4176 |  | 198 | src/globals.c — definitions of the global state, MOTD and brand | 3.9 |  | 0.652 |
| walker |  | 4260 | 211 | Code::CodeKey { rung: Names, file: include/packets.h, decl: 0, sub: 0, line: 0 } |  |  | 0.681 |
| ns | 4260 |  | 84 | README Configuration section — where the knobs live | 3.10 |  | 0.681 |
| walker |  | 4269 | 9 | Code::CodeKey { rung: Doc, file: include/packets.h, decl: 1, sub: 0, line: 5 } |  |  | 0.685 |
| ns | 4402 |  | 142 | README Configuration — the maintainer's tuning guidance | 3.11 |  | 0.687 |
| ns | 4488 |  | 86 | build_registries.js — the complete biome list | 3.12 |  | 0.677 |
| walker |  | 4489 | 220 | Code::CodeKey { rung: Names, file: include/procedures.h, decl: 0, sub: 0, line: 0 } |  |  | 0.679 |
| walker |  | 4702 | 213 | Code::CodeKey { rung: Names, file: include/procedures.h, decl: 0, sub: 1, line: 0 } |  |  | 0.681 |
| ns | 4721 |  | 233 | procedures.h — client state and player lifecycle API | 4.1 |  | 0.691 |
| walker |  | 4898 | 196 | Code::CodeKey { rung: Names, file: include/procedures.h, decl: 0, sub: 2, line: 0 } |  |  | 0.693 |
| ns | 4985 |  | 264 | procedures.h — metadata broadcast, slot mapping and block predicates | 4.2 |  | 0.702 |
| walker |  | 5090 | 192 | Code::CodeKey { rung: Names, file: include/procedures.h, decl: 0, sub: 3, line: 0 } |  |  | 0.705 |
| walker |  | 5259 | 169 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.705 |
| ns | 5304 |  | 319 | procedures.h — mining, actions, fluids, mobs, tick and entity-data API | 4.3 |  | 0.712 |
| ns | 5415 |  | 111 | worldgen.h — ChunkAnchor and ChunkFeature | 4.4 |  | 0.718 |
| walker |  | 5495 | 236 | Code::CodeKey { rung: Names, file: include/packets.h, decl: 0, sub: 1, line: 0 } |  |  | 0.746 |
| walker |  | 5502 | 7 | Code::CodeKey { rung: Doc, file: include/packets.h, decl: 22, sub: 0, line: 28 } |  |  | 0.747 |
| ns | 5587 |  | 172 | worldgen.h — the complete generation API and the shared chunk_section buffer | 4.5 |  | 0.750 |
| walker |  | 5696 | 194 | Code::CodeKey { rung: Names, file: include/packets.h, decl: 0, sub: 2, line: 0 } |  |  | 0.773 |
| ns | 5794 |  | 207 | tools.h — socket I/O and the byte-order writers | 4.6 |  | 0.776 |
| walker |  | 5916 | 220 | Code::CodeKey { rung: Names, file: include/packets.h, decl: 0, sub: 3, line: 0 } |  |  | 0.795 |
| ns | 5991 |  | 197 | tools.h — the readers, string helpers and RNG | 4.7 |  | 0.798 |
| walker |  | 6110 | 194 | Code::CodeKey { rung: Names, file: include/packets.h, decl: 0, sub: 4, line: 0 } |  |  | 0.810 |
| ns | 6129 |  | 138 | tools.h — the inline math helpers and the platform time shim | 4.8 |  | 0.808 |
| walker |  | 6223 | 113 | Code::CodeKey { rung: Names, file: include/packets.h, decl: 0, sub: 5, line: 0 } |  |  | 0.824 |
| ns | 6325 |  | 196 | serialize.h in full — persistence API and its compile-time no-op fallback | 4.9 |  | 0.826 |
| walker |  | 6328 | 105 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 24, sub: 0, line: 93 } |  |  | 0.826 |
| ns | 6410 |  | 85 | crafting.h and structures.h in full — the two smallest module APIs | 4.10 |  | 0.821 |
| walker |  | 6467 | 139 | Plaintext::DeclSurface { file: build.sh } |  |  | 0.824 |
| ns | 6499 |  | 89 | main.c — the project's own module include list | 5.1 |  | 0.815 |
| walker |  | 6513 | 46 | Code::CodeKey { rung: Names, file: include/crafting.h, decl: 0, sub: 0, line: 0 } |  |  | 0.817 |
| walker |  | 6537 | 24 | Code::CodeKey { rung: Names, file: include/structures.h, decl: 0, sub: 0, line: 0 } |  |  | 0.819 |
| walker |  | 6735 | 198 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.819 |
| ns | 6755 |  | 256 | main.c — the maintainer's design note on the packet handlers, and handlePacket's signature | 5.2 |  | 0.804 |
| walker |  | 6841 | 106 | Plaintext::DeclSurface { file: src/CMakeLists.txt } |  |  | 0.822 |
| ns | 7081 |  | 326 | main.c — packet 0x00 dispatch: handshake, status, login, configuration | 5.3 |  | 0.798 |
| walker |  | 7116 | 275 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.798 |
| ns | 7350 |  | 269 | main.c — dispatch table, packet ids 0x07 through 0x19 | 5.4 |  | 0.782 |
| walker |  | 7475 | 359 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.782 |
| ns | 7692 |  | 342 | main.c — dispatch table, the movement case group and ids 0x28 through the default | 5.5 |  | 0.761 |
| walker |  | 7865 | 390 | Plaintext::Whole { file: build.sh } |  |  | 0.779 |
| ns | 8048 |  | 356 | main.c — the single-threaded accept and tick round-robin loop | 5.6 |  | 0.757 |
| walker |  | 8129 | 264 | Code::CodeKey { rung: Names, file: src/globals.c, decl: 0, sub: 0, line: 0 } |  |  | 0.772 |
| walker |  | 8134 | 5 | Code::CodeKey { rung: Decl, file: src/globals.c, decl: 10, sub: 0, line: 45 } |  |  | 0.774 |
| walker |  | 8142 | 8 | Code::CodeKey { rung: Decl, file: src/globals.c, decl: 9, sub: 0, line: 43 } |  |  | 0.776 |
| walker |  | 8200 | 58 | Code::CodeKey { rung: Names, file: src/varnum.c, decl: 0, sub: 0, line: 0 } |  |  | 0.776 |
| walker |  | 8261 | 61 | Code::CodeKey { rung: Body, file: src/varnum.c, decl: 2, sub: 0, line: 34 } |  |  | 0.776 |
| ns | 8329 |  | 281 | main.c — the ESP32 entry points: FreeRTOS task, WiFi event handler, app_main | 5.7 |  | 0.759 |
| walker |  | 8363 | 102 | Code::CodeKey { rung: Body, file: src/varnum.c, decl: 3, sub: 0, line: 43 } |  |  | 0.759 |
| walker |  | 8430 | 67 | Code::CodeKey { rung: Names, file: src/crafting.c, decl: 0, sub: 0, line: 0 } |  |  | 0.759 |
| walker |  | 8458 | 28 | Code::CodeKey { rung: Decl, file: src/crafting.c, decl: 2, sub: 0, line: 349 } |  |  | 0.759 |
| ns | 8553 |  | 224 | procedures.c — definition locations, part 1 (state, players, slots, block changes) | 6.1 |  | 0.745 |
| walker |  | 8692 | 234 | Code::CodeKey { rung: Names, file: src/packets.c, decl: 0, sub: 0, line: 0 } |  |  | 0.745 |
| walker |  | 8703 | 11 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 2, sub: 0, line: 49 } |  |  | 0.745 |
| walker |  | 8714 | 11 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 3, sub: 0, line: 66 } |  |  | 0.745 |
| walker |  | 8725 | 11 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 4, sub: 0, line: 85 } |  |  | 0.745 |
| walker |  | 8736 | 11 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 9, sub: 0, line: 183 } |  |  | 0.745 |
| walker |  | 8748 | 12 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 10, sub: 0, line: 190 } |  |  | 0.745 |
| walker |  | 8761 | 13 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 6, sub: 0, line: 137 } |  |  | 0.745 |
| ns | 8770 |  | 217 | procedures.c — definition locations, part 2 (mining, predicates, armour, eating, fluids) | 6.2 |  | 0.734 |
| walker |  | 8774 | 13 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 7, sub: 0, line: 151 } |  |  | 0.734 |
| walker |  | 8787 | 13 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 8, sub: 0, line: 166 } |  |  | 0.734 |
| walker |  | 8800 | 13 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 11, sub: 0, line: 245 } |  |  | 0.734 |
| walker |  | 8815 | 15 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 1, sub: 0, line: 28 } |  |  | 0.734 |
| ns | 8902 |  | 132 | procedures.c — definition locations, part 3 (actions, mobs, tick, entity data) | 6.3 |  | 0.727 |
| walker |  | 9046 | 231 | Code::CodeKey { rung: Names, file: src/packets.c, decl: 0, sub: 1, line: 0 } |  |  | 0.727 |
| walker |  | 9057 | 11 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 14, sub: 0, line: 300 } |  |  | 0.727 |
| walker |  | 9068 | 11 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 20, sub: 0, line: 471 } |  |  | 0.727 |
| walker |  | 9080 | 12 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 16, sub: 0, line: 323 } |  |  | 0.727 |
| walker |  | 9092 | 12 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 19, sub: 0, line: 444 } |  |  | 0.727 |
| walker |  | 9105 | 13 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 12, sub: 0, line: 275 } |  |  | 0.727 |
| ns | 9117 |  | 215 | worldgen.c — every definition, including the five private generator stages | 6.4 |  | 0.718 |
| walker |  | 9119 | 14 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 17, sub: 0, line: 332 } |  |  | 0.718 |
| walker |  | 9134 | 15 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 13, sub: 0, line: 287 } |  |  | 0.718 |
| walker |  | 9149 | 15 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 18, sub: 0, line: 433 } |  |  | 0.718 |
| walker |  | 9168 | 19 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 15, sub: 0, line: 314 } |  |  | 0.718 |
| ns | 9304 |  | 187 | serialize.c — the world file path and all five persistence entry points | 6.5 |  | 0.710 |
| walker |  | 9395 | 227 | Code::CodeKey { rung: Names, file: src/packets.c, decl: 0, sub: 2, line: 0 } |  |  | 0.710 |
| walker |  | 9406 | 11 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 22, sub: 0, line: 488 } |  |  | 0.710 |
| walker |  | 9417 | 11 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 23, sub: 0, line: 512 } |  |  | 0.710 |
| walker |  | 9428 | 11 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 24, sub: 0, line: 528 } |  |  | 0.710 |
| walker |  | 9439 | 11 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 26, sub: 0, line: 577 } |  |  | 0.710 |
| walker |  | 9451 | 12 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 25, sub: 0, line: 545 } |  |  | 0.710 |
| walker |  | 9463 | 12 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 27, sub: 0, line: 707 } |  |  | 0.710 |
| walker |  | 9475 | 12 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 29, sub: 0, line: 740 } |  |  | 0.710 |
| ns | 9485 |  | 181 | packets.c — the chat command surface (!msg and !help) | 6.6 |  | 0.704 |
| walker |  | 9489 | 14 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 21, sub: 0, line: 480 } |  |  | 0.704 |
| walker |  | 9503 | 14 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 28, sub: 0, line: 725 } |  |  | 0.704 |
| walker |  | 9716 | 213 | Code::CodeKey { rung: Names, file: src/packets.c, decl: 0, sub: 3, line: 0 } |  |  | 0.704 |
| ns | 9744 |  | 259 | crafting.c — the registerSmeltingRecipe macro and the complete recipe table | 6.7 |  | 0.696 |
| ns | 9771 |  | 27 | Complete .github listings (workflow and issue templates) | 7.1 |  | 0.698 |
| walker |  | 9777 | 61 | Code::CodeKey { rung: Decl, file: src/packets.c, decl: 37, sub: 0, line: 886 } |  |  | 0.698 |
| walker |  | 9788 | 11 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 37, sub: 0, line: 886 } |  |  | 0.698 |
| walker |  | 9800 | 12 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 30, sub: 0, line: 752 } |  |  | 0.698 |
| walker |  | 9812 | 12 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 38, sub: 0, line: 922 } |  |  | 0.698 |
| walker |  | 9826 | 14 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 32, sub: 0, line: 774 } |  |  | 0.698 |
| walker |  | 9840 | 14 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 35, sub: 0, line: 836 } |  |  | 0.698 |
| walker |  | 9855 | 15 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 33, sub: 0, line: 811 } |  |  | 0.698 |
| walker |  | 9870 | 15 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 34, sub: 0, line: 825 } |  |  | 0.698 |
| ns | 9877 |  | 106 | README Contribution — the maintainer's rules for changes | 7.2 |  | 0.699 |
| walker |  | 9885 | 15 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 39, sub: 0, line: 942 } |  |  | 0.699 |
| walker |  | 9903 | 18 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 36, sub: 0, line: 866 } |  |  | 0.699 |
| ns | 9931 |  | 54 | extract_registries.sh — the top-level registry extraction sequence | 7.3 |  | 0.697 |
| ns | 9959 |  | 28 | LICENSE — the license identity line | 7.4 |  | 0.696 |
| walker |  | 9996 | 93 | Code::CodeKey { rung: Names, file: src/packets.c, decl: 0, sub: 4, line: 0 } |  |  | 0.696 |
