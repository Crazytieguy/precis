Score(3000)=0.482 I=0.809 C=0.287 ns_rows≤3K=20/55 grid(1000/1442/2080/3000/4327/6240/9000)=0.732/0.635/0.622/0.482/0.513/0.730/0.703

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
| walker |  | 150 | 24 | Code::CodeKey { rung: Names, file: include/structures.h, decl: 0, sub: 0, line: 0 } |  |  | 0.623 |
| ns | 234 |  | 86 | Complete src/ and include/ listings | 1.4 |  | 0.638 |
| walker |  | 248 | 98 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.807 |
| ns | 285 |  | 51 | All README H2 section headings | 1.5 |  | 0.741 |
| walker |  | 299 | 51 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.817 |
| ns | 343 |  | 58 | .gitignore in full — which sources are generated and absent | 1.6 |  | 0.738 |
| walker |  | 362 | 63 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.904 |
| walker |  | 408 | 46 | Code::CodeKey { rung: Names, file: include/crafting.h, decl: 0, sub: 0, line: 0 } |  |  | 0.904 |
| ns | 489 |  | 146 | build.sh: the registry prerequisite check and the actual compile command | 1.7 |  | 0.811 |
| ns | 595 |  | 106 | src/CMakeLists.txt in full — the ESP-IDF/PlatformIO build | 1.8 |  | 0.747 |
| ns | 665 |  | 70 | Connection state constants (STATE_NONE through STATE_PLAY) | 2.1 |  | 0.707 |
| ns | 885 |  | 220 | packets.h — serverbound declarations, connection and world interaction | 2.2 |  | 0.647 |
| walker |  | 937 | 529 | Plaintext::Whole { file: build.sh } |  |  | 0.732 |
| ns | 1038 |  | 153 | packets.h — remainder of the serverbound declarations | 2.3 |  | 0.678 |
| walker |  | 1148 | 211 | Code::CodeKey { rung: Names, file: include/tools.h, decl: 0, sub: 0, line: 0 } |  |  | 0.680 |
| ns | 1265 |  | 227 | packets.h — clientbound declarations, login and configuration phase | 2.4 |  | 0.631 |
| walker |  | 1428 | 280 | Code::CodeKey { rung: Names, file: include/globals.h, decl: 0, sub: 0, line: 0 } |  |  | 0.634 |
| walker |  | 1437 | 9 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 1, sub: 0, line: 7 } |  |  | 0.635 |
| walker |  | 1447 | 10 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 4, sub: 0, line: 11 } |  |  | 0.635 |
| walker |  | 1458 | 11 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 9, sub: 0, line: 26 } |  |  | 0.635 |
| walker |  | 1469 | 11 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 10, sub: 0, line: 29 } |  |  | 0.635 |
| ns | 1560 |  | 295 | packets.h — clientbound declarations, world and inventory | 2.5 |  | 0.595 |
| walker |  | 1569 | 100 | Code::CodeKey { rung: Names, file: include/varnum.h, decl: 0, sub: 0, line: 0 } |  |  | 0.598 |
| walker |  | 1586 | 17 | Code::CodeKey { rung: Body, file: include/tools.h, decl: 1, sub: 0, line: 8 } |  |  | 0.598 |
| walker |  | 1806 | 220 | Code::CodeKey { rung: Names, file: include/procedures.h, decl: 0, sub: 0, line: 0 } |  |  | 0.601 |
| walker |  | 1833 | 27 | Code::CodeKey { rung: Names, file: src/main.c, decl: 0, sub: 0, line: 0 } |  |  | 0.601 |
| walker |  | 1846 | 13 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 14, sub: 0, line: 41 } |  |  | 0.601 |
| ns | 1849 |  | 289 | packets.h — clientbound declarations, entities, health and registries | 2.6 |  | 0.564 |
| walker |  | 1862 | 16 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.564 |
| ns | 1949 |  | 100 | varnum.h in full — VarInt encoding primitives and error sentinel | 2.7 |  | 0.585 |
| ns | 2019 |  | 70 | globals.h — BlockChange record and the packed-struct pragma | 3.1 |  | 0.566 |
| walker |  | 2073 | 211 | Code::CodeKey { rung: Names, file: include/packets.h, decl: 0, sub: 0, line: 0 } |  |  | 0.622 |
| walker |  | 2082 | 9 | Code::CodeKey { rung: Doc, file: include/packets.h, decl: 1, sub: 0, line: 5 } |  |  | 0.630 |
| walker |  | 2236 | 154 | Code::CodeKey { rung: Names, file: include/serialize.h, decl: 0, sub: 0, line: 0 } |  |  | 0.631 |
| walker |  | 2243 | 7 | Code::CodeKey { rung: Decl, file: include/serialize.h, decl: 6, sub: 0, line: 12 } |  |  | 0.631 |
| walker |  | 2250 | 7 | Code::CodeKey { rung: Decl, file: include/serialize.h, decl: 10, sub: 0, line: 18 } |  |  | 0.631 |
| walker |  | 2263 | 13 | Code::CodeKey { rung: Decl, file: include/serialize.h, decl: 1, sub: 0, line: 6 } |  |  | 0.631 |
| ns | 2269 |  | 250 | globals.h — PlayerData fields (identity through inventory) | 3.2 |  | 0.578 |
| walker |  | 2278 | 15 | Code::CodeKey { rung: Doc, file: include/serialize.h, decl: 6, sub: 0, line: 12 } |  |  | 0.579 |
| walker |  | 2303 | 25 | Code::CodeKey { rung: Body, file: include/tools.h, decl: 2, sub: 0, line: 11 } |  |  | 0.579 |
| ns | 2526 |  | 257 | globals.h — PlayerData flag bits and the overloaded flagval fields | 3.3 | 3.2 | 0.548 |
| walker |  | 2632 | 329 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: true } |  |  | 0.549 |
| walker |  | 2646 | 14 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 7, sub: 0, line: 19 } |  |  | 0.549 |
| ns | 2756 |  | 230 | globals.h — MobData and EntityData | 3.4 |  | 0.514 |
| walker |  | 2866 | 220 | Code::CodeKey { rung: Names, file: include/worldgen.h, decl: 0, sub: 0, line: 0 } |  |  | 0.515 |
| walker |  | 2900 | 34 | Code::CodeKey { rung: Decl, file: include/worldgen.h, decl: 1, sub: 0, line: 6 } |  |  | 0.516 |
| walker |  | 2934 | 34 | Code::CodeKey { rung: Decl, file: include/worldgen.h, decl: 2, sub: 0, line: 13 } |  |  | 0.517 |
| walker |  | 2986 | 52 | Code::CodeKey { rung: Names, file: src/tools.c, decl: 0, sub: 0, line: 0 } |  |  | 0.517 |
| ns | 2994 |  | 238 | globals.h — all extern global state declarations | 3.5 |  | 0.482 |
| walker |  | 3041 | 55 | Code::CodeKey { rung: Names, file: src/structures.c, decl: 0, sub: 0, line: 0 } |  |  | 0.482 |
| walker |  | 3055 | 14 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 12, sub: 0, line: 35 } |  |  | 0.482 |
| walker |  | 3113 | 58 | Code::CodeKey { rung: Names, file: src/varnum.c, decl: 0, sub: 0, line: 0 } |  |  | 0.482 |
| walker |  | 3180 | 67 | Code::CodeKey { rung: Names, file: src/crafting.c, decl: 0, sub: 0, line: 0 } |  |  | 0.482 |
| walker |  | 3208 | 28 | Code::CodeKey { rung: Decl, file: src/crafting.c, decl: 2, sub: 0, line: 349 } |  |  | 0.482 |
| walker |  | 3327 | 119 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.482 |
| ns | 3344 |  | 350 | build_registries.js — the template that generates include/registries.h | 3.6 |  | 0.456 |
| walker |  | 3594 | 267 | Code::CodeKey { rung: Names, file: include/tools.h, decl: 0, sub: 1, line: 0 } |  |  | 0.458 |
| walker |  | 3603 | 9 | Code::CodeKey { rung: Decl, file: include/tools.h, decl: 26, sub: 0, line: 43 } |  |  | 0.458 |
| walker |  | 3615 | 12 | Code::CodeKey { rung: Decl, file: include/tools.h, decl: 27, sub: 0, line: 46 } |  |  | 0.459 |
| ns | 3704 |  | 360 | globals.h — configuration macro roster, part 1 (world and player sizing) | 3.7 |  | 0.481 |
| walker |  | 3914 | 299 | Code::CodeKey { rung: Names, file: include/globals.h, decl: 0, sub: 1, line: 0 } |  |  | 0.543 |
| walker |  | 3922 | 8 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 19, sub: 0, line: 59 } |  |  | 0.543 |
| walker |  | 3936 | 14 | Code::CodeKey { rung: Doc, file: src/structures.c, decl: 2, sub: 0, line: 16 } |  |  | 0.543 |
| ns | 3978 |  | 274 | globals.h — configuration macro roster, part 2 (feature toggles, including the commented-out ones) | 3.8 |  | 0.524 |
| walker |  | 4149 | 213 | Code::CodeKey { rung: Names, file: include/procedures.h, decl: 0, sub: 1, line: 0 } |  |  | 0.526 |
| walker |  | 4159 | 10 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 21, sub: 0, line: 66 } |  |  | 0.526 |
| ns | 4176 |  | 198 | src/globals.c — definitions of the global state, MOTD and brand | 3.9 |  | 0.510 |
| ns | 4260 |  | 84 | README Configuration section — where the knobs live | 3.10 |  | 0.513 |
| walker |  | 4395 | 236 | Code::CodeKey { rung: Names, file: include/packets.h, decl: 0, sub: 1, line: 0 } |  |  | 0.551 |
| walker |  | 4402 | 7 | Code::CodeKey { rung: Doc, file: include/packets.h, decl: 22, sub: 0, line: 28 } |  |  | 0.556 |
| ns | 4402 |  | 142 | README Configuration — the maintainer's tuning guidance | 3.11 |  | 0.556 |
| ns | 4488 |  | 86 | build_registries.js — the complete biome list | 3.12 |  | 0.548 |
| walker |  | 4598 | 196 | Code::CodeKey { rung: Names, file: include/procedures.h, decl: 0, sub: 2, line: 0 } |  |  | 0.550 |
| ns | 4721 |  | 233 | procedures.h — client state and player lifecycle API | 4.1 |  | 0.569 |
| walker |  | 4814 | 216 | Code::CodeKey { rung: Names, file: include/globals.h, decl: 0, sub: 2, line: 0 } |  |  | 0.609 |
| walker |  | 4819 | 5 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 42, sub: 0, line: 186 } |  |  | 0.612 |
| walker |  | 4827 | 8 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 41, sub: 0, line: 184 } |  |  | 0.616 |
| walker |  | 4848 | 21 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 47, sub: 0, line: 255 } |  |  | 0.616 |
| walker |  | 4882 | 34 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 44, sub: 0, line: 191 } |  |  | 0.626 |
| walker |  | 4931 | 49 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 48, sub: 0, line: 260 } |  |  | 0.631 |
| ns | 4985 |  | 264 | procedures.h — metadata broadcast, slot mapping and block predicates | 4.2 |  | 0.645 |
| walker |  | 5062 | 131 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 46, sub: 0, line: 240 } |  |  | 0.679 |
| walker |  | 5292 | 230 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 45, sub: 0, line: 200 } |  |  | 0.723 |
| ns | 5304 |  | 319 | procedures.h — mining, actions, fluids, mobs, tick and entity-data API | 4.3 |  | 0.707 |
| ns | 5415 |  | 111 | worldgen.h — ChunkAnchor and ChunkFeature | 4.4 |  | 0.714 |
| walker |  | 5486 | 194 | Code::CodeKey { rung: Names, file: src/worldgen.c, decl: 0, sub: 0, line: 0 } |  |  | 0.714 |
| ns | 5587 |  | 172 | worldgen.h — the complete generation API and the shared chunk_section buffer | 4.5 |  | 0.718 |
| walker |  | 5729 | 243 | Code::CodeKey { rung: Names, file: src/procedures.c, decl: 0, sub: 0, line: 0 } |  |  | 0.719 |
| walker |  | 5745 | 16 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 5, sub: 0, line: 54 } |  |  | 0.719 |
| walker |  | 5761 | 16 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 6, sub: 0, line: 75 } |  |  | 0.719 |
| walker |  | 5777 | 16 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 9, sub: 0, line: 146 } |  |  | 0.719 |
| ns | 5794 |  | 207 | tools.h — socket I/O and the byte-order writers | 4.6 |  | 0.724 |
| walker |  | 5795 | 18 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 10, sub: 0, line: 177 } |  |  | 0.724 |
| walker |  | 5814 | 19 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 8, sub: 0, line: 130 } |  |  | 0.724 |
| ns | 5991 |  | 197 | tools.h — the readers, string helpers and RNG | 4.7 |  | 0.730 |
| ns | 6129 |  | 138 | tools.h — the inline math helpers and the platform time shim | 4.8 |  | 0.730 |
| ns | 6325 |  | 196 | serialize.h in full — persistence API and its compile-time no-op fallback | 4.9 |  | 0.735 |
| ns | 6410 |  | 85 | crafting.h and structures.h in full — the two smallest module APIs | 4.10 |  | 0.734 |
| ns | 6499 |  | 89 | main.c — the project's own module include list | 5.1 |  | 0.727 |
| walker |  | 6504 | 690 | Plaintext::Whole { file: extract_registries.sh } |  |  | 0.727 |
| walker |  | 6673 | 169 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.727 |
| ns | 6755 |  | 256 | main.c — the maintainer's design note on the packet handlers, and handlePacket's signature | 5.2 |  | 0.713 |
| walker |  | 7032 | 359 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: true } |  |  | 0.713 |
| ns | 7081 |  | 326 | main.c — packet 0x00 dispatch: handshake, status, login, configuration | 5.3 |  | 0.692 |
| walker |  | 7266 | 234 | Code::CodeKey { rung: Names, file: src/packets.c, decl: 0, sub: 0, line: 0 } |  |  | 0.692 |
| walker |  | 7277 | 11 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 2, sub: 0, line: 49 } |  |  | 0.692 |
| walker |  | 7288 | 11 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 3, sub: 0, line: 66 } |  |  | 0.692 |
| walker |  | 7299 | 11 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 4, sub: 0, line: 85 } |  |  | 0.692 |
| walker |  | 7310 | 11 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 9, sub: 0, line: 183 } |  |  | 0.692 |
| walker |  | 7322 | 12 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 10, sub: 0, line: 190 } |  |  | 0.692 |
| walker |  | 7349 | 27 | Code::CodeKey { rung: Doc, file: src/worldgen.c, decl: 7, sub: 0, line: 160 } |  |  | 0.692 |
| ns | 7350 |  | 269 | main.c — dispatch table, packet ids 0x07 through 0x19 | 5.4 |  | 0.678 |
| walker |  | 7362 | 13 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 6, sub: 0, line: 137 } |  |  | 0.678 |
| walker |  | 7575 | 213 | Code::CodeKey { rung: Names, file: src/procedures.c, decl: 0, sub: 1, line: 0 } |  |  | 0.679 |
| walker |  | 7590 | 15 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 19, sub: 0, line: 495 } |  |  | 0.679 |
| walker |  | 7607 | 17 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 15, sub: 0, line: 321 } |  |  | 0.679 |
| ns | 7692 |  | 342 | main.c — dispatch table, the movement case group and ids 0x28 through the default | 5.5 |  | 0.661 |
| walker |  | 7835 | 228 | Code::CodeKey { rung: Names, file: src/globals.c, decl: 0, sub: 0, line: 0 } |  |  | 0.672 |
| walker |  | 8029 | 194 | Code::CodeKey { rung: Names, file: include/packets.h, decl: 0, sub: 2, line: 0 } |  |  | 0.691 |
| ns | 8048 |  | 356 | main.c — the single-threaded accept and tick round-robin loop | 5.6 |  | 0.671 |
| walker |  | 8289 | 260 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 45, sub: 1, line: 200 } |  |  | 0.697 |
| walker |  | 8323 | 34 | Code::CodeKey { rung: Doc, file: src/tools.c, decl: 1, sub: 0, line: 42 } |  |  | 0.697 |
| ns | 8329 |  | 281 | main.c — the ESP32 entry points: FreeRTOS task, WiFi event handler, app_main | 5.7 |  | 0.681 |
| walker |  | 8515 | 192 | Code::CodeKey { rung: Names, file: include/procedures.h, decl: 0, sub: 3, line: 0 } |  |  | 0.699 |
| walker |  | 8528 | 13 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 7, sub: 0, line: 151 } |  |  | 0.699 |
| ns | 8553 |  | 224 | procedures.c — definition locations, part 1 (state, players, slots, block changes) | 6.1 |  | 0.705 |
| walker |  | 8748 | 220 | Code::CodeKey { rung: Names, file: include/packets.h, decl: 0, sub: 3, line: 0 } |  |  | 0.720 |
| ns | 8770 |  | 217 | procedures.c — definition locations, part 2 (mining, predicates, armour, eating, fluids) | 6.2 |  | 0.709 |
| ns | 8902 |  | 132 | procedures.c — definition locations, part 3 (actions, mobs, tick, entity data) | 6.3 |  | 0.702 |
| walker |  | 8946 | 198 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.703 |
| walker |  | 9052 | 106 | Plaintext::DeclSurface { file: src/CMakeLists.txt } |  |  | 0.718 |
| ns | 9117 |  | 215 | worldgen.c — every definition, including the five private generator stages | 6.4 |  | 0.712 |
| walker |  | 9266 | 214 | Code::CodeKey { rung: Names, file: src/procedures.c, decl: 0, sub: 2, line: 0 } |  |  | 0.720 |
| walker |  | 9280 | 14 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 25, sub: 0, line: 789 } |  |  | 0.720 |
| walker |  | 9294 | 14 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 30, sub: 0, line: 858 } |  |  | 0.720 |
| ns | 9304 |  | 187 | serialize.c — the world file path and all five persistence entry points | 6.5 |  | 0.712 |
| walker |  | 9307 | 13 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 8, sub: 0, line: 166 } |  |  | 0.712 |
| walker |  | 9359 | 52 | Code::CodeKey { rung: Doc, file: src/worldgen.c, decl: 5, sub: 0, line: 126 } |  |  | 0.712 |
| walker |  | 9418 | 59 | Code::CodeKey { rung: Body, file: src/structures.c, decl: 1, sub: 0, line: 9 } |  |  | 0.712 |
| ns | 9485 |  | 181 | packets.c — the chat command surface (!msg and !help) | 6.6 |  | 0.705 |
| walker |  | 9693 | 275 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.705 |
| ns | 9744 |  | 259 | crafting.c — the registerSmeltingRecipe macro and the complete recipe table | 6.7 |  | 0.697 |
| walker |  | 9754 | 61 | Code::CodeKey { rung: Body, file: src/varnum.c, decl: 2, sub: 0, line: 34 } |  |  | 0.697 |
| walker |  | 9767 | 13 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 11, sub: 0, line: 245 } |  |  | 0.697 |
| ns | 9771 |  | 27 | Complete .github listings (workflow and issue templates) | 7.1 |  | 0.699 |
| walker |  | 9784 | 17 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 24, sub: 0, line: 775 } |  |  | 0.699 |
| ns | 9877 |  | 106 | README Contribution — the maintainer's rules for changes | 7.2 |  | 0.701 |
| ns | 9931 |  | 54 | extract_registries.sh — the top-level registry extraction sequence | 7.3 |  | 0.698 |
| ns | 9959 |  | 28 | LICENSE — the license identity line | 7.4 |  | 0.697 |
| walker |  | 9978 | 194 | Code::CodeKey { rung: Names, file: include/packets.h, decl: 0, sub: 4, line: 0 } |  |  | 0.705 |
