Score(3000)=0.715 I=0.847 C=0.603 ns_rows≤3K=20/55 grid(1000/1442/2080/3000/4327/6240/9000)=0.723/0.626/0.566/0.715/0.815/0.850/0.739

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 29 | 29 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 54 |  | 54 | Project identity, tagline, and target Minecraft/protocol version | 1.1 |  | 0.000 |
| ns | 83 |  | 29 | Complete repository root listing | 1.2 |  | 0.489 |
| walker |  | 127 | 98 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.711 |
| ns | 148 |  | 65 | Stated project goal and priority ordering | 1.3 |  | 0.710 |
| walker |  | 165 | 38 | Fs::DirListing { dir: include } |  |  | 0.734 |
| walker |  | 213 | 48 | Fs::DirListing { dir: src } |  |  | 0.847 |
| walker |  | 220 | 7 | Fs::DirListing { dir: .github } |  |  | 0.847 |
| walker |  | 224 | 4 | Fs::DirListing { dir: .github/workflows } |  |  | 0.847 |
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
| walker |  | 908 | 106 | Plaintext::DeclSurface { file: src/CMakeLists.txt } |  |  | 0.723 |
| walker |  | 1033 | 125 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.723 |
| ns | 1038 |  | 153 | packets.h — remainder of the serverbound declarations | 2.3 |  | 0.669 |
| walker |  | 1262 | 229 | Code::CodeKey { rung: Names, file: include/globals.h, decl: 0, sub: 0, line: 0 } |  |  | 0.671 |
| ns | 1265 |  | 227 | packets.h — clientbound declarations, login and configuration phase | 2.4 |  | 0.623 |
| walker |  | 1271 | 9 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 1, sub: 0, line: 7 } |  |  | 0.624 |
| walker |  | 1281 | 10 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 4, sub: 0, line: 11 } |  |  | 0.624 |
| walker |  | 1292 | 11 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 9, sub: 0, line: 26 } |  |  | 0.624 |
| walker |  | 1303 | 11 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 10, sub: 0, line: 29 } |  |  | 0.624 |
| walker |  | 1316 | 13 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 14, sub: 0, line: 41 } |  |  | 0.624 |
| walker |  | 1330 | 14 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 7, sub: 0, line: 19 } |  |  | 0.624 |
| walker |  | 1344 | 14 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 12, sub: 0, line: 35 } |  |  | 0.624 |
| ns | 1560 |  | 295 | packets.h — clientbound declarations, world and inventory | 2.5 |  | 0.584 |
| walker |  | 1584 | 240 | Code::CodeKey { rung: Names, file: include/globals.h, decl: 0, sub: 1, line: 0 } |  |  | 0.588 |
| walker |  | 1592 | 8 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 19, sub: 0, line: 59 } |  |  | 0.588 |
| walker |  | 1602 | 10 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 21, sub: 0, line: 66 } |  |  | 0.588 |
| walker |  | 1615 | 13 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 26, sub: 0, line: 106 } |  |  | 0.588 |
| ns | 1849 |  | 289 | packets.h — clientbound declarations, entities, health and registries | 2.6 |  | 0.552 |
| walker |  | 1864 | 249 | Code::CodeKey { rung: Names, file: include/globals.h, decl: 0, sub: 2, line: 0 } |  |  | 0.595 |
| walker |  | 1872 | 8 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 49, sub: 0, line: 184 } |  |  | 0.595 |
| ns | 1949 |  | 100 | varnum.h in full — VarInt encoding primitives and error sentinel | 2.7 |  | 0.577 |
| ns | 2019 |  | 70 | globals.h — BlockChange record and the packed-struct pragma | 3.1 |  | 0.557 |
| walker |  | 2065 | 193 | Code::CodeKey { rung: Names, file: include/globals.h, decl: 0, sub: 3, line: 0 } |  |  | 0.565 |
| walker |  | 2070 | 5 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 50, sub: 0, line: 186 } |  |  | 0.566 |
| walker |  | 2091 | 21 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 55, sub: 0, line: 255 } |  |  | 0.566 |
| walker |  | 2125 | 34 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 52, sub: 0, line: 191 } |  |  | 0.588 |
| walker |  | 2174 | 49 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 56, sub: 0, line: 260 } |  |  | 0.589 |
| ns | 2269 |  | 250 | globals.h — PlayerData fields (identity through inventory) | 3.2 |  | 0.540 |
| walker |  | 2305 | 131 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 54, sub: 0, line: 240 } |  |  | 0.545 |
| walker |  | 2496 | 191 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 53, sub: 0, line: 200 } |  |  | 0.616 |
| ns | 2526 |  | 257 | globals.h — PlayerData flag bits and the overloaded flagval fields | 3.3 | 3.2 | 0.583 |
| ns | 2756 |  | 230 | globals.h — MobData and EntityData | 3.4 |  | 0.614 |
| walker |  | 2795 | 299 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 53, sub: 1, line: 200 } |  |  | 0.701 |
| walker |  | 2812 | 17 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 13, sub: 0, line: 38 } |  |  | 0.701 |
| walker |  | 2829 | 17 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 33, sub: 0, line: 149 } |  |  | 0.701 |
| walker |  | 2848 | 19 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 18, sub: 0, line: 56 } |  |  | 0.701 |
| walker |  | 2867 | 19 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 34, sub: 0, line: 155 } |  |  | 0.701 |
| walker |  | 2981 | 114 | Plaintext::DeclSurface { file: extract_registries.sh } |  |  | 0.701 |
| ns | 2994 |  | 238 | globals.h — all extern global state declarations | 3.5 |  | 0.715 |
| walker |  | 3163 | 182 | Code::CodeKey { rung: Names, file: include/packets.h, decl: 0, sub: 0, line: 0 } |  |  | 0.741 |
| walker |  | 3172 | 9 | Code::CodeKey { rung: Doc, file: include/packets.h, decl: 1, sub: 0, line: 5 } |  |  | 0.745 |
| walker |  | 3341 | 169 | Code::CodeKey { rung: Names, file: include/packets.h, decl: 0, sub: 1, line: 0 } |  |  | 0.784 |
| ns | 3344 |  | 350 | build_registries.js — the template that generates include/registries.h | 3.6 |  | 0.741 |
| walker |  | 3523 | 182 | Code::CodeKey { rung: Names, file: include/packets.h, decl: 0, sub: 2, line: 0 } |  |  | 0.765 |
| walker |  | 3530 | 7 | Code::CodeKey { rung: Doc, file: include/packets.h, decl: 22, sub: 0, line: 28 } |  |  | 0.768 |
| walker |  | 3695 | 165 | Code::CodeKey { rung: Names, file: include/packets.h, decl: 0, sub: 3, line: 0 } |  |  | 0.789 |
| ns | 3704 |  | 360 | globals.h — configuration macro roster, part 1 (world and player sizing) | 3.7 |  | 0.798 |
| walker |  | 3858 | 163 | Code::CodeKey { rung: Names, file: include/packets.h, decl: 0, sub: 4, line: 0 } |  |  | 0.820 |
| ns | 3978 |  | 274 | globals.h — configuration macro roster, part 2 (feature toggles, including the commented-out ones) | 3.8 |  | 0.802 |
| walker |  | 4005 | 147 | Code::CodeKey { rung: Names, file: include/packets.h, decl: 0, sub: 5, line: 0 } |  |  | 0.812 |
| walker |  | 4165 | 160 | Code::CodeKey { rung: Names, file: include/packets.h, decl: 0, sub: 6, line: 0 } |  |  | 0.838 |
| ns | 4176 |  | 198 | src/globals.c — definitions of the global state, MOTD and brand | 3.9 |  | 0.813 |
| ns | 4260 |  | 84 | README Configuration section — where the knobs live | 3.10 |  | 0.813 |
| walker |  | 4345 | 180 | Code::CodeKey { rung: Names, file: include/procedures.h, decl: 0, sub: 0, line: 0 } |  |  | 0.815 |
| ns | 4402 |  | 142 | README Configuration — the maintainer's tuning guidance | 3.11 |  | 0.816 |
| ns | 4488 |  | 86 | build_registries.js — the complete biome list | 3.12 |  | 0.804 |
| walker |  | 4516 | 171 | Code::CodeKey { rung: Names, file: include/tools.h, decl: 0, sub: 0, line: 0 } |  |  | 0.805 |
| walker |  | 4684 | 168 | Code::CodeKey { rung: Names, file: include/tools.h, decl: 0, sub: 1, line: 0 } |  |  | 0.807 |
| ns | 4721 |  | 233 | procedures.h — client state and player lifecycle API | 4.1 |  | 0.801 |
| walker |  | 4904 | 220 | Code::CodeKey { rung: Names, file: include/worldgen.h, decl: 0, sub: 0, line: 0 } |  |  | 0.803 |
| walker |  | 4938 | 34 | Code::CodeKey { rung: Decl, file: include/worldgen.h, decl: 1, sub: 0, line: 6 } |  |  | 0.804 |
| walker |  | 4972 | 34 | Code::CodeKey { rung: Decl, file: include/worldgen.h, decl: 2, sub: 0, line: 13 } |  |  | 0.805 |
| ns | 4985 |  | 264 | procedures.h — metadata broadcast, slot mapping and block predicates | 4.2 |  | 0.784 |
| walker |  | 5126 | 154 | Code::CodeKey { rung: Names, file: include/serialize.h, decl: 0, sub: 0, line: 0 } |  |  | 0.785 |
| walker |  | 5133 | 7 | Code::CodeKey { rung: Decl, file: include/serialize.h, decl: 6, sub: 0, line: 12 } |  |  | 0.786 |
| walker |  | 5140 | 7 | Code::CodeKey { rung: Decl, file: include/serialize.h, decl: 10, sub: 0, line: 18 } |  |  | 0.786 |
| walker |  | 5153 | 13 | Code::CodeKey { rung: Decl, file: include/serialize.h, decl: 1, sub: 0, line: 6 } |  |  | 0.786 |
| walker |  | 5168 | 15 | Code::CodeKey { rung: Doc, file: include/serialize.h, decl: 6, sub: 0, line: 12 } |  |  | 0.786 |
| walker |  | 5268 | 100 | Code::CodeKey { rung: Names, file: include/varnum.h, decl: 0, sub: 0, line: 0 } |  |  | 0.801 |
| ns | 5304 |  | 319 | procedures.h — mining, actions, fluids, mobs, tick and entity-data API | 4.3 |  | 0.779 |
| walker |  | 5407 | 139 | Code::CodeKey { rung: Names, file: include/tools.h, decl: 0, sub: 2, line: 0 } |  |  | 0.781 |
| ns | 5415 |  | 111 | worldgen.h — ChunkAnchor and ChunkFeature | 4.4 |  | 0.785 |
| walker |  | 5416 | 9 | Code::CodeKey { rung: Decl, file: include/tools.h, decl: 26, sub: 0, line: 43 } |  |  | 0.786 |
| walker |  | 5428 | 12 | Code::CodeKey { rung: Decl, file: include/tools.h, decl: 27, sub: 0, line: 46 } |  |  | 0.786 |
| ns | 5587 |  | 172 | worldgen.h — the complete generation API and the shared chunk_section buffer | 4.5 |  | 0.789 |
| walker |  | 5620 | 192 | Code::CodeKey { rung: Names, file: include/procedures.h, decl: 0, sub: 1, line: 0 } |  |  | 0.805 |
| walker |  | 5787 | 167 | Code::CodeKey { rung: Names, file: include/procedures.h, decl: 0, sub: 2, line: 0 } |  |  | 0.823 |
| ns | 5794 |  | 207 | tools.h — socket I/O and the byte-order writers | 4.6 |  | 0.825 |
| walker |  | 5950 | 163 | Code::CodeKey { rung: Names, file: include/procedures.h, decl: 0, sub: 3, line: 0 } |  |  | 0.835 |
| ns | 5991 |  | 197 | tools.h — the readers, string helpers and RNG | 4.7 |  | 0.837 |
| walker |  | 6069 | 119 | Code::CodeKey { rung: Names, file: include/procedures.h, decl: 0, sub: 4, line: 0 } |  |  | 0.854 |
| ns | 6129 |  | 138 | tools.h — the inline math helpers and the platform time shim | 4.8 |  | 0.847 |
| walker |  | 6208 | 139 | Plaintext::DeclSurface { file: build.sh } |  |  | 0.850 |
| walker |  | 6254 | 46 | Code::CodeKey { rung: Names, file: include/crafting.h, decl: 0, sub: 0, line: 0 } |  |  | 0.850 |
| walker |  | 6278 | 24 | Code::CodeKey { rung: Names, file: include/structures.h, decl: 0, sub: 0, line: 0 } |  |  | 0.850 |
| walker |  | 6305 | 27 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 11, sub: 0, line: 32 } |  |  | 0.850 |
| ns | 6325 |  | 196 | serialize.h in full — persistence API and its compile-time no-op fallback | 4.9 |  | 0.852 |
| walker |  | 6332 | 27 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 15, sub: 0, line: 45 } |  |  | 0.852 |
| walker |  | 6359 | 27 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 23, sub: 0, line: 75 } |  |  | 0.852 |
| walker |  | 6389 | 30 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 30, sub: 0, line: 129 } |  |  | 0.852 |
| ns | 6410 |  | 85 | crafting.h and structures.h in full — the two smallest module APIs | 4.10 |  | 0.849 |
| walker |  | 6420 | 31 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 16, sub: 0, line: 49 } |  |  | 0.849 |
| walker |  | 6453 | 33 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 20, sub: 0, line: 63 } |  |  | 0.849 |
| ns | 6499 |  | 89 | main.c — the project's own module include list | 5.1 |  | 0.841 |
| walker |  | 6728 | 275 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.841 |
| ns | 6755 |  | 256 | main.c — the maintainer's design note on the packet handlers, and handlePacket's signature | 5.2 |  | 0.825 |
| ns | 7081 |  | 326 | main.c — packet 0x00 dispatch: handshake, status, login, configuration | 5.3 |  | 0.801 |
| walker |  | 7087 | 359 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.801 |
| walker |  | 7124 | 37 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 17, sub: 0, line: 53 } |  |  | 0.801 |
| walker |  | 7162 | 38 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 8, sub: 0, line: 23 } |  |  | 0.801 |
| walker |  | 7214 | 52 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 25, sub: 0, line: 103 } |  |  | 0.801 |
| walker |  | 7268 | 54 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 22, sub: 0, line: 71 } |  |  | 0.801 |
| walker |  | 7285 | 17 | Code::CodeKey { rung: Body, file: include/tools.h, decl: 1, sub: 0, line: 8 } |  |  | 0.803 |
| ns | 7350 |  | 269 | main.c — dispatch table, packet ids 0x07 through 0x19 | 5.4 |  | 0.787 |
| walker |  | 7351 | 66 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 27, sub: 0, line: 111 } |  |  | 0.787 |
| ns | 7692 |  | 342 | main.c — dispatch table, the movement case group and ids 0x28 through the default | 5.5 |  | 0.766 |
| walker |  | 7741 | 390 | Plaintext::Whole { file: build.sh } |  |  | 0.784 |
| walker |  | 7811 | 70 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 31, sub: 0, line: 135 } |  |  | 0.784 |
| walker |  | 7836 | 25 | Code::CodeKey { rung: Body, file: include/tools.h, decl: 2, sub: 0, line: 11 } |  |  | 0.787 |
| walker |  | 7920 | 84 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 32, sub: 0, line: 146 } |  |  | 0.787 |
| walker |  | 8014 | 94 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 29, sub: 0, line: 125 } |  |  | 0.787 |
| ns | 8048 |  | 356 | main.c — the single-threaded accept and tick round-robin loop | 5.6 |  | 0.764 |
| walker |  | 8110 | 96 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 28, sub: 0, line: 118 } |  |  | 0.764 |
| walker |  | 8215 | 105 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 24, sub: 0, line: 93 } |  |  | 0.764 |
| ns | 8329 |  | 281 | main.c — the ESP32 entry points: FreeRTOS task, WiFi event handler, app_main | 5.7 |  | 0.747 |
| walker |  | 8397 | 182 | Code::CodeKey { rung: Names, file: src/procedures.c, decl: 0, sub: 0, line: 0 } |  |  | 0.748 |
| walker |  | 8413 | 16 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 5, sub: 0, line: 54 } |  |  | 0.748 |
| walker |  | 8429 | 16 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 6, sub: 0, line: 75 } |  |  | 0.748 |
| walker |  | 8445 | 16 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 9, sub: 0, line: 146 } |  |  | 0.748 |
| walker |  | 8464 | 19 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 8, sub: 0, line: 130 } |  |  | 0.748 |
| ns | 8553 |  | 224 | procedures.c — definition locations, part 1 (state, players, slots, block changes) | 6.1 |  | 0.738 |
| walker |  | 8655 | 191 | Code::CodeKey { rung: Names, file: src/procedures.c, decl: 0, sub: 1, line: 0 } |  |  | 0.751 |
| walker |  | 8672 | 17 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 15, sub: 0, line: 321 } |  |  | 0.751 |
| walker |  | 8690 | 18 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 10, sub: 0, line: 177 } |  |  | 0.751 |
| walker |  | 8713 | 23 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 16, sub: 0, line: 396 } |  |  | 0.751 |
| ns | 8770 |  | 217 | procedures.c — definition locations, part 2 (mining, predicates, armour, eating, fluids) | 6.2 |  | 0.740 |
| walker |  | 8879 | 166 | Code::CodeKey { rung: Names, file: src/procedures.c, decl: 0, sub: 2, line: 0 } |  |  | 0.745 |
| walker |  | 8893 | 14 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 25, sub: 0, line: 789 } |  |  | 0.745 |
| ns | 8902 |  | 132 | procedures.c — definition locations, part 3 (actions, mobs, tick, entity data) | 6.3 |  | 0.738 |
| walker |  | 8908 | 15 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 19, sub: 0, line: 495 } |  |  | 0.738 |
| walker |  | 8925 | 17 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 24, sub: 0, line: 775 } |  |  | 0.738 |
| walker |  | 8943 | 18 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 22, sub: 0, line: 721 } |  |  | 0.738 |
| walker |  | 8962 | 19 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 23, sub: 0, line: 746 } |  |  | 0.738 |
| ns | 9117 |  | 215 | worldgen.c — every definition, including the five private generator stages | 6.4 |  | 0.729 |
| walker |  | 9133 | 171 | Code::CodeKey { rung: Names, file: src/procedures.c, decl: 0, sub: 3, line: 0 } |  |  | 0.738 |
| walker |  | 9147 | 14 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 30, sub: 0, line: 858 } |  |  | 0.738 |
| walker |  | 9162 | 15 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 32, sub: 0, line: 939 } |  |  | 0.738 |
| walker |  | 9179 | 17 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 26, sub: 0, line: 802 } |  |  | 0.738 |
| walker |  | 9196 | 17 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 27, sub: 0, line: 812 } |  |  | 0.738 |
| ns | 9304 |  | 187 | serialize.c — the world file path and all five persistence entry points | 6.5 |  | 0.730 |
| walker |  | 9379 | 183 | Code::CodeKey { rung: Names, file: src/procedures.c, decl: 0, sub: 4, line: 0 } |  |  | 0.736 |
| walker |  | 9402 | 23 | Code::CodeKey { rung: Decl, file: src/procedures.c, decl: 37, sub: 0, line: 1121 } |  |  | 0.736 |
| walker |  | 9420 | 18 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 37, sub: 0, line: 1121 } |  |  | 0.736 |
| ns | 9485 |  | 181 | packets.c — the chat command surface (!msg and !help) | 6.6 |  | 0.729 |
| walker |  | 9629 | 209 | Code::CodeKey { rung: Names, file: src/procedures.c, decl: 0, sub: 5, line: 0 } |  |  | 0.739 |
| walker |  | 9652 | 23 | Code::CodeKey { rung: Decl, file: src/procedures.c, decl: 44, sub: 0, line: 1928 } |  |  | 0.739 |
| walker |  | 9669 | 17 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 46, sub: 0, line: 1965 } |  |  | 0.739 |
| walker |  | 9688 | 19 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 47, sub: 0, line: 1983 } |  |  | 0.739 |
| walker |  | 9720 | 32 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 31, sub: 0, line: 909 } |  |  | 0.739 |
| ns | 9744 |  | 259 | crafting.c — the registerSmeltingRecipe macro and the complete recipe table | 6.7 |  | 0.731 |
| walker |  | 9753 | 33 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 17, sub: 0, line: 434 } |  |  | 0.731 |
| ns | 9771 |  | 27 | Complete .github listings (workflow and issue templates) | 7.1 |  | 0.732 |
| walker |  | 9786 | 33 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 33, sub: 0, line: 954 } |  |  | 0.732 |
| walker |  | 9820 | 34 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 44, sub: 0, line: 1928 } |  |  | 0.732 |
| walker |  | 9857 | 37 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 43, sub: 0, line: 1623 } |  |  | 0.732 |
| ns | 9877 |  | 106 | README Contribution — the maintainer's rules for changes | 7.2 |  | 0.729 |
| walker |  | 9896 | 39 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 29, sub: 0, line: 831 } |  |  | 0.729 |
| ns | 9931 |  | 54 | extract_registries.sh — the top-level registry extraction sequence | 7.3 |  | 0.726 |
| walker |  | 9941 | 45 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 21, sub: 0, line: 652 } |  |  | 0.726 |
| ns | 9959 |  | 28 | LICENSE — the license identity line | 7.4 |  | 0.725 |
| walker |  | 9993 | 52 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 34, sub: 0, line: 990 } |  |  | 0.725 |
