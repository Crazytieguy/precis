Score(3000)=0.479 I=0.800 C=0.287 ns_rows≤3K=20/55 grid(1000/1442/2080/3000/4327/6240/9000)=0.733/0.631/0.524/0.479/0.447/0.671/0.710

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
| ns | 885 |  | 220 | packets.h — serverbound declarations, connection and world interaction | 2.2 |  | 0.732 |
| walker |  | 891 | 24 | Code::CodeKey { rung: Names, file: include/structures.h, decl: 0, sub: 0, line: 0 } |  |  | 0.732 |
| walker |  | 907 | 16 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.732 |
| walker |  | 953 | 46 | Code::CodeKey { rung: Names, file: include/crafting.h, decl: 0, sub: 0, line: 0 } |  |  | 0.733 |
| ns | 1038 |  | 153 | packets.h — remainder of the serverbound declarations | 2.3 |  | 0.678 |
| ns | 1265 |  | 227 | packets.h — clientbound declarations, login and configuration phase | 2.4 |  | 0.630 |
| walker |  | 1282 | 329 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: true } |  |  | 0.631 |
| walker |  | 1493 | 211 | Code::CodeKey { rung: Names, file: include/tools.h, decl: 0, sub: 0, line: 0 } |  |  | 0.633 |
| walker |  | 1510 | 17 | Code::CodeKey { rung: Body, file: include/tools.h, decl: 1, sub: 0, line: 8 } |  |  | 0.633 |
| walker |  | 1535 | 25 | Code::CodeKey { rung: Body, file: include/tools.h, decl: 2, sub: 0, line: 11 } |  |  | 0.633 |
| ns | 1560 |  | 295 | packets.h — clientbound declarations, world and inventory | 2.5 |  | 0.593 |
| walker |  | 1654 | 119 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.593 |
| ns | 1849 |  | 289 | packets.h — clientbound declarations, entities, health and registries | 2.6 |  | 0.556 |
| walker |  | 1934 | 280 | Code::CodeKey { rung: Names, file: include/globals.h, decl: 0, sub: 0, line: 0 } |  |  | 0.559 |
| walker |  | 1943 | 9 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 1, sub: 0, line: 7 } |  |  | 0.559 |
| ns | 1949 |  | 100 | varnum.h in full — VarInt encoding primitives and error sentinel | 2.7 |  | 0.542 |
| walker |  | 1953 | 10 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 4, sub: 0, line: 11 } |  |  | 0.542 |
| walker |  | 1964 | 11 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 9, sub: 0, line: 26 } |  |  | 0.542 |
| walker |  | 1975 | 11 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 10, sub: 0, line: 29 } |  |  | 0.542 |
| walker |  | 1988 | 13 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 14, sub: 0, line: 41 } |  |  | 0.542 |
| walker |  | 2002 | 14 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 7, sub: 0, line: 19 } |  |  | 0.542 |
| walker |  | 2016 | 14 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 12, sub: 0, line: 35 } |  |  | 0.542 |
| ns | 2019 |  | 70 | globals.h — BlockChange record and the packed-struct pragma | 3.1 |  | 0.524 |
| walker |  | 2033 | 17 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 13, sub: 0, line: 38 } |  |  | 0.524 |
| walker |  | 2052 | 19 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 18, sub: 0, line: 56 } |  |  | 0.524 |
| walker |  | 2079 | 27 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 11, sub: 0, line: 32 } |  |  | 0.524 |
| walker |  | 2106 | 27 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 15, sub: 0, line: 45 } |  |  | 0.524 |
| walker |  | 2137 | 31 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 16, sub: 0, line: 49 } |  |  | 0.524 |
| walker |  | 2174 | 37 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 17, sub: 0, line: 53 } |  |  | 0.524 |
| walker |  | 2212 | 38 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 8, sub: 0, line: 23 } |  |  | 0.524 |
| ns | 2269 |  | 250 | globals.h — PlayerData fields (identity through inventory) | 3.2 |  | 0.480 |
| walker |  | 2312 | 100 | Code::CodeKey { rung: Names, file: include/varnum.h, decl: 0, sub: 0, line: 0 } |  |  | 0.517 |
| ns | 2526 |  | 257 | globals.h — PlayerData flag bits and the overloaded flagval fields | 3.3 | 3.2 | 0.490 |
| walker |  | 2532 | 220 | Code::CodeKey { rung: Names, file: include/procedures.h, decl: 0, sub: 0, line: 0 } |  |  | 0.492 |
| walker |  | 2559 | 27 | Code::CodeKey { rung: Names, file: src/main.c, decl: 0, sub: 0, line: 0 } |  |  | 0.492 |
| ns | 2756 |  | 230 | globals.h — MobData and EntityData | 3.4 |  | 0.460 |
| walker |  | 2770 | 211 | Code::CodeKey { rung: Names, file: include/packets.h, decl: 0, sub: 0, line: 0 } |  |  | 0.506 |
| walker |  | 2779 | 9 | Code::CodeKey { rung: Doc, file: include/packets.h, decl: 1, sub: 0, line: 5 } |  |  | 0.512 |
| walker |  | 2933 | 154 | Code::CodeKey { rung: Names, file: include/serialize.h, decl: 0, sub: 0, line: 0 } |  |  | 0.513 |
| walker |  | 2940 | 7 | Code::CodeKey { rung: Decl, file: include/serialize.h, decl: 6, sub: 0, line: 12 } |  |  | 0.513 |
| walker |  | 2947 | 7 | Code::CodeKey { rung: Decl, file: include/serialize.h, decl: 10, sub: 0, line: 18 } |  |  | 0.513 |
| walker |  | 2960 | 13 | Code::CodeKey { rung: Decl, file: include/serialize.h, decl: 1, sub: 0, line: 6 } |  |  | 0.513 |
| walker |  | 2975 | 15 | Code::CodeKey { rung: Doc, file: include/serialize.h, decl: 6, sub: 0, line: 12 } |  |  | 0.514 |
| ns | 2994 |  | 238 | globals.h — all extern global state declarations | 3.5 |  | 0.479 |
| ns | 3344 |  | 350 | build_registries.js — the template that generates include/registries.h | 3.6 |  | 0.453 |
| walker |  | 3665 | 690 | Plaintext::Whole { file: extract_registries.sh } |  |  | 0.453 |
| ns | 3704 |  | 360 | globals.h — configuration macro roster, part 1 (world and player sizing) | 3.7 |  | 0.475 |
| walker |  | 3834 | 169 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.475 |
| ns | 3978 |  | 274 | globals.h — configuration macro roster, part 2 (feature toggles, including the commented-out ones) | 3.8 |  | 0.457 |
| ns | 4176 |  | 198 | src/globals.c — definitions of the global state, MOTD and brand | 3.9 |  | 0.443 |
| walker |  | 4193 | 359 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: true } |  |  | 0.443 |
| ns | 4260 |  | 84 | README Configuration section — where the knobs live | 3.10 |  | 0.447 |
| ns | 4402 |  | 142 | README Configuration — the maintainer's tuning guidance | 3.11 |  | 0.453 |
| walker |  | 4460 | 267 | Code::CodeKey { rung: Names, file: include/tools.h, decl: 0, sub: 1, line: 0 } |  |  | 0.455 |
| walker |  | 4469 | 9 | Code::CodeKey { rung: Decl, file: include/tools.h, decl: 26, sub: 0, line: 43 } |  |  | 0.455 |
| walker |  | 4481 | 12 | Code::CodeKey { rung: Decl, file: include/tools.h, decl: 27, sub: 0, line: 46 } |  |  | 0.455 |
| ns | 4488 |  | 86 | build_registries.js — the complete biome list | 3.12 |  | 0.449 |
| ns | 4721 |  | 233 | procedures.h — client state and player lifecycle API | 4.1 |  | 0.470 |
| walker |  | 4780 | 299 | Code::CodeKey { rung: Names, file: include/globals.h, decl: 0, sub: 1, line: 0 } |  |  | 0.523 |
| walker |  | 4788 | 8 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 19, sub: 0, line: 59 } |  |  | 0.523 |
| walker |  | 4798 | 10 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 21, sub: 0, line: 66 } |  |  | 0.523 |
| walker |  | 4811 | 13 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 26, sub: 0, line: 106 } |  |  | 0.523 |
| walker |  | 4838 | 27 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 23, sub: 0, line: 75 } |  |  | 0.523 |
| walker |  | 4871 | 33 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 20, sub: 0, line: 63 } |  |  | 0.523 |
| walker |  | 4923 | 52 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 25, sub: 0, line: 103 } |  |  | 0.523 |
| walker |  | 4977 | 54 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 22, sub: 0, line: 71 } |  |  | 0.523 |
| ns | 4985 |  | 264 | procedures.h — metadata broadcast, slot mapping and block predicates | 4.2 |  | 0.509 |
| walker |  | 5082 | 105 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 24, sub: 0, line: 93 } |  |  | 0.509 |
| walker |  | 5302 | 220 | Code::CodeKey { rung: Names, file: include/worldgen.h, decl: 0, sub: 0, line: 0 } |  |  | 0.510 |
| ns | 5304 |  | 319 | procedures.h — mining, actions, fluids, mobs, tick and entity-data API | 4.3 |  | 0.496 |
| walker |  | 5336 | 34 | Code::CodeKey { rung: Decl, file: include/worldgen.h, decl: 1, sub: 0, line: 6 } |  |  | 0.497 |
| walker |  | 5370 | 34 | Code::CodeKey { rung: Decl, file: include/worldgen.h, decl: 2, sub: 0, line: 13 } |  |  | 0.497 |
| ns | 5415 |  | 111 | worldgen.h — ChunkAnchor and ChunkFeature | 4.4 |  | 0.514 |
| walker |  | 5586 | 216 | Code::CodeKey { rung: Names, file: include/globals.h, decl: 0, sub: 2, line: 0 } |  |  | 0.551 |
| ns | 5587 |  | 172 | worldgen.h — the complete generation API and the shared chunk_section buffer | 4.5 |  | 0.560 |
| walker |  | 5591 | 5 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 42, sub: 0, line: 186 } |  |  | 0.562 |
| walker |  | 5599 | 8 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 41, sub: 0, line: 184 } |  |  | 0.566 |
| walker |  | 5620 | 21 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 47, sub: 0, line: 255 } |  |  | 0.566 |
| walker |  | 5654 | 34 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 44, sub: 0, line: 191 } |  |  | 0.575 |
| walker |  | 5703 | 49 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 48, sub: 0, line: 260 } |  |  | 0.580 |
| ns | 5794 |  | 207 | tools.h — socket I/O and the byte-order writers | 4.6 |  | 0.589 |
| walker |  | 5834 | 131 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 46, sub: 0, line: 240 } |  |  | 0.620 |
| ns | 5991 |  | 197 | tools.h — the readers, string helpers and RNG | 4.7 |  | 0.630 |
| walker |  | 6064 | 230 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 45, sub: 0, line: 200 } |  |  | 0.669 |
| ns | 6129 |  | 138 | tools.h — the inline math helpers and the platform time shim | 4.8 |  | 0.671 |
| walker |  | 6324 | 260 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 45, sub: 1, line: 200 } |  |  | 0.703 |
| ns | 6325 |  | 196 | serialize.h in full — persistence API and its compile-time no-op fallback | 4.9 |  | 0.708 |
| walker |  | 6376 | 52 | Code::CodeKey { rung: Names, file: src/tools.c, decl: 0, sub: 0, line: 0 } |  |  | 0.708 |
| ns | 6410 |  | 85 | crafting.h and structures.h in full — the two smallest module APIs | 4.10 |  | 0.707 |
| walker |  | 6431 | 55 | Code::CodeKey { rung: Names, file: src/structures.c, decl: 0, sub: 0, line: 0 } |  |  | 0.707 |
| walker |  | 6445 | 14 | Code::CodeKey { rung: Doc, file: src/structures.c, decl: 2, sub: 0, line: 16 } |  |  | 0.707 |
| ns | 6499 |  | 89 | main.c — the project's own module include list | 5.1 |  | 0.700 |
| walker |  | 6658 | 213 | Code::CodeKey { rung: Names, file: include/procedures.h, decl: 0, sub: 1, line: 0 } |  |  | 0.716 |
| ns | 6755 |  | 256 | main.c — the maintainer's design note on the packet handlers, and handlePacket's signature | 5.2 |  | 0.702 |
| walker |  | 6854 | 196 | Code::CodeKey { rung: Names, file: include/procedures.h, decl: 0, sub: 2, line: 0 } |  |  | 0.715 |
| walker |  | 7046 | 192 | Code::CodeKey { rung: Names, file: include/procedures.h, decl: 0, sub: 3, line: 0 } |  |  | 0.735 |
| ns | 7081 |  | 326 | main.c — packet 0x00 dispatch: handshake, status, login, configuration | 5.3 |  | 0.714 |
| walker |  | 7104 | 58 | Code::CodeKey { rung: Names, file: src/varnum.c, decl: 0, sub: 0, line: 0 } |  |  | 0.714 |
| walker |  | 7138 | 34 | Code::CodeKey { rung: Doc, file: src/tools.c, decl: 1, sub: 0, line: 42 } |  |  | 0.714 |
| walker |  | 7205 | 67 | Code::CodeKey { rung: Names, file: src/crafting.c, decl: 0, sub: 0, line: 0 } |  |  | 0.714 |
| walker |  | 7233 | 28 | Code::CodeKey { rung: Decl, file: src/crafting.c, decl: 2, sub: 0, line: 349 } |  |  | 0.714 |
| ns | 7350 |  | 269 | main.c — dispatch table, packet ids 0x07 through 0x19 | 5.4 |  | 0.700 |
| walker |  | 7469 | 236 | Code::CodeKey { rung: Names, file: include/packets.h, decl: 0, sub: 1, line: 0 } |  |  | 0.723 |
| walker |  | 7476 | 7 | Code::CodeKey { rung: Doc, file: include/packets.h, decl: 22, sub: 0, line: 28 } |  |  | 0.724 |
| walker |  | 7670 | 194 | Code::CodeKey { rung: Names, file: include/packets.h, decl: 0, sub: 2, line: 0 } |  |  | 0.742 |
| ns | 7692 |  | 342 | main.c — dispatch table, the movement case group and ids 0x28 through the default | 5.5 |  | 0.723 |
| walker |  | 7890 | 220 | Code::CodeKey { rung: Names, file: include/packets.h, decl: 0, sub: 3, line: 0 } |  |  | 0.739 |
| ns | 8048 |  | 356 | main.c — the single-threaded accept and tick round-robin loop | 5.6 |  | 0.718 |
| walker |  | 8084 | 194 | Code::CodeKey { rung: Names, file: include/packets.h, decl: 0, sub: 4, line: 0 } |  |  | 0.727 |
| walker |  | 8197 | 113 | Code::CodeKey { rung: Names, file: include/packets.h, decl: 0, sub: 5, line: 0 } |  |  | 0.741 |
| ns | 8329 |  | 281 | main.c — the ESP32 entry points: FreeRTOS task, WiFi event handler, app_main | 5.7 |  | 0.724 |
| walker |  | 8395 | 198 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.725 |
| walker |  | 8501 | 106 | Plaintext::DeclSurface { file: src/CMakeLists.txt } |  |  | 0.741 |
| ns | 8553 |  | 224 | procedures.c — definition locations, part 1 (state, players, slots, block changes) | 6.1 |  | 0.727 |
| walker |  | 8560 | 59 | Code::CodeKey { rung: Body, file: src/structures.c, decl: 1, sub: 0, line: 9 } |  |  | 0.727 |
| ns | 8770 |  | 217 | procedures.c — definition locations, part 2 (mining, predicates, armour, eating, fluids) | 6.2 |  | 0.717 |
| walker |  | 8835 | 275 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.717 |
| walker |  | 8896 | 61 | Code::CodeKey { rung: Body, file: src/varnum.c, decl: 2, sub: 0, line: 34 } |  |  | 0.717 |
| ns | 8902 |  | 132 | procedures.c — definition locations, part 3 (actions, mobs, tick, entity data) | 6.3 |  | 0.710 |
| walker |  | 9090 | 194 | Code::CodeKey { rung: Names, file: src/worldgen.c, decl: 0, sub: 0, line: 0 } |  |  | 0.710 |
| walker |  | 9117 | 27 | Code::CodeKey { rung: Doc, file: src/worldgen.c, decl: 7, sub: 0, line: 160 } |  |  | 0.704 |
| ns | 9117 |  | 215 | worldgen.c — every definition, including the five private generator stages | 6.4 |  | 0.704 |
| walker |  | 9169 | 52 | Code::CodeKey { rung: Doc, file: src/worldgen.c, decl: 5, sub: 0, line: 126 } |  |  | 0.704 |
| ns | 9304 |  | 187 | serialize.c — the world file path and all five persistence entry points | 6.5 |  | 0.696 |
| walker |  | 9412 | 243 | Code::CodeKey { rung: Names, file: src/procedures.c, decl: 0, sub: 0, line: 0 } |  |  | 0.703 |
| walker |  | 9428 | 16 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 5, sub: 0, line: 54 } |  |  | 0.703 |
| walker |  | 9444 | 16 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 6, sub: 0, line: 75 } |  |  | 0.703 |
| walker |  | 9460 | 16 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 9, sub: 0, line: 146 } |  |  | 0.703 |
| walker |  | 9478 | 18 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 10, sub: 0, line: 177 } |  |  | 0.703 |
| ns | 9485 |  | 181 | packets.c — the chat command surface (!msg and !help) | 6.6 |  | 0.696 |
| walker |  | 9497 | 19 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 8, sub: 0, line: 130 } |  |  | 0.696 |
| walker |  | 9710 | 213 | Code::CodeKey { rung: Names, file: src/procedures.c, decl: 0, sub: 1, line: 0 } |  |  | 0.709 |
| walker |  | 9725 | 15 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 19, sub: 0, line: 495 } |  |  | 0.709 |
| walker |  | 9742 | 17 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 15, sub: 0, line: 321 } |  |  | 0.709 |
| ns | 9744 |  | 259 | crafting.c — the registerSmeltingRecipe macro and the complete recipe table | 6.7 |  | 0.701 |
| walker |  | 9765 | 23 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 16, sub: 0, line: 396 } |  |  | 0.701 |
| ns | 9771 |  | 27 | Complete .github listings (workflow and issue templates) | 7.1 |  | 0.703 |
| walker |  | 9798 | 33 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 17, sub: 0, line: 434 } |  |  | 0.703 |
| walker |  | 9843 | 45 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 21, sub: 0, line: 652 } |  |  | 0.703 |
| ns | 9877 |  | 106 | README Contribution — the maintainer's rules for changes | 7.2 |  | 0.705 |
| ns | 9931 |  | 54 | extract_registries.sh — the top-level registry extraction sequence | 7.3 |  | 0.702 |
| ns | 9959 |  | 28 | LICENSE — the license identity line | 7.4 |  | 0.701 |
