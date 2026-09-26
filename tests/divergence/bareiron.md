Score(3000)=0.688 I=0.830 C=0.571 ns_rows≤3K=20/55 grid(1000/1442/2080/3000/4327/6240/9000)=0.648/0.563/0.523/0.688/0.701/0.831/0.734

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
| walker |  | 927 | 125 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.648 |
| ns | 1038 |  | 153 | packets.h — remainder of the serverbound declarations | 2.3 |  | 0.600 |
| walker |  | 1156 | 229 | Code::CodeKey { rung: Names, file: include/globals.h, decl: 0, sub: 0, line: 0 } |  |  | 0.602 |
| walker |  | 1165 | 9 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 1, sub: 0, line: 7 } |  |  | 0.602 |
| walker |  | 1175 | 10 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 4, sub: 0, line: 11 } |  |  | 0.603 |
| walker |  | 1186 | 11 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 9, sub: 0, line: 26 } |  |  | 0.603 |
| walker |  | 1197 | 11 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 10, sub: 0, line: 29 } |  |  | 0.603 |
| walker |  | 1210 | 13 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 14, sub: 0, line: 41 } |  |  | 0.603 |
| walker |  | 1224 | 14 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 7, sub: 0, line: 19 } |  |  | 0.603 |
| walker |  | 1238 | 14 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 12, sub: 0, line: 35 } |  |  | 0.603 |
| walker |  | 1255 | 17 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 13, sub: 0, line: 38 } |  |  | 0.603 |
| ns | 1265 |  | 227 | packets.h — clientbound declarations, login and configuration phase | 2.4 |  | 0.560 |
| walker |  | 1495 | 240 | Code::CodeKey { rung: Names, file: include/globals.h, decl: 0, sub: 1, line: 0 } |  |  | 0.564 |
| walker |  | 1503 | 8 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 19, sub: 0, line: 59 } |  |  | 0.564 |
| walker |  | 1513 | 10 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 21, sub: 0, line: 66 } |  |  | 0.564 |
| walker |  | 1526 | 13 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 26, sub: 0, line: 106 } |  |  | 0.564 |
| ns | 1560 |  | 295 | packets.h — clientbound declarations, world and inventory | 2.5 |  | 0.528 |
| walker |  | 1775 | 249 | Code::CodeKey { rung: Names, file: include/globals.h, decl: 0, sub: 2, line: 0 } |  |  | 0.576 |
| walker |  | 1783 | 8 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 49, sub: 0, line: 184 } |  |  | 0.576 |
| walker |  | 1800 | 17 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 33, sub: 0, line: 149 } |  |  | 0.576 |
| walker |  | 1819 | 19 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 18, sub: 0, line: 56 } |  |  | 0.576 |
| walker |  | 1838 | 19 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 34, sub: 0, line: 155 } |  |  | 0.576 |
| ns | 1849 |  | 289 | packets.h — clientbound declarations, entities, health and registries | 2.6 |  | 0.540 |
| ns | 1949 |  | 100 | varnum.h in full — VarInt encoding primitives and error sentinel | 2.7 |  | 0.524 |
| ns | 2019 |  | 70 | globals.h — BlockChange record and the packed-struct pragma | 3.1 |  | 0.506 |
| walker |  | 2031 | 193 | Code::CodeKey { rung: Names, file: include/globals.h, decl: 0, sub: 3, line: 0 } |  |  | 0.514 |
| walker |  | 2036 | 5 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 50, sub: 0, line: 186 } |  |  | 0.514 |
| walker |  | 2057 | 21 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 55, sub: 0, line: 255 } |  |  | 0.514 |
| walker |  | 2091 | 34 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 52, sub: 0, line: 191 } |  |  | 0.538 |
| walker |  | 2140 | 49 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 56, sub: 0, line: 260 } |  |  | 0.539 |
| ns | 2269 |  | 250 | globals.h — PlayerData fields (identity through inventory) | 3.2 |  | 0.494 |
| walker |  | 2271 | 131 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 54, sub: 0, line: 240 } |  |  | 0.498 |
| walker |  | 2462 | 191 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 53, sub: 0, line: 200 } |  |  | 0.573 |
| ns | 2526 |  | 257 | globals.h — PlayerData flag bits and the overloaded flagval fields | 3.3 | 3.2 | 0.543 |
| ns | 2756 |  | 230 | globals.h — MobData and EntityData | 3.4 |  | 0.578 |
| walker |  | 2761 | 299 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 53, sub: 1, line: 200 } |  |  | 0.667 |
| walker |  | 2915 | 154 | Code::CodeKey { rung: Names, file: include/serialize.h, decl: 0, sub: 0, line: 0 } |  |  | 0.668 |
| walker |  | 2922 | 7 | Code::CodeKey { rung: Decl, file: include/serialize.h, decl: 6, sub: 0, line: 12 } |  |  | 0.668 |
| walker |  | 2929 | 7 | Code::CodeKey { rung: Decl, file: include/serialize.h, decl: 10, sub: 0, line: 18 } |  |  | 0.668 |
| walker |  | 2942 | 13 | Code::CodeKey { rung: Decl, file: include/serialize.h, decl: 1, sub: 0, line: 6 } |  |  | 0.669 |
| walker |  | 2957 | 15 | Code::CodeKey { rung: Doc, file: include/serialize.h, decl: 6, sub: 0, line: 12 } |  |  | 0.669 |
| ns | 2994 |  | 238 | globals.h — all extern global state declarations | 3.5 |  | 0.687 |
| walker |  | 3057 | 100 | Code::CodeKey { rung: Names, file: include/varnum.h, decl: 0, sub: 0, line: 0 } |  |  | 0.709 |
| walker |  | 3228 | 171 | Code::CodeKey { rung: Names, file: include/tools.h, decl: 0, sub: 0, line: 0 } |  |  | 0.710 |
| ns | 3344 |  | 350 | build_registries.js — the template that generates include/registries.h | 3.6 |  | 0.671 |
| walker |  | 3396 | 168 | Code::CodeKey { rung: Names, file: include/tools.h, decl: 0, sub: 1, line: 0 } |  |  | 0.673 |
| walker |  | 3616 | 220 | Code::CodeKey { rung: Names, file: include/worldgen.h, decl: 0, sub: 0, line: 0 } |  |  | 0.675 |
| walker |  | 3650 | 34 | Code::CodeKey { rung: Decl, file: include/worldgen.h, decl: 1, sub: 0, line: 6 } |  |  | 0.675 |
| walker |  | 3684 | 34 | Code::CodeKey { rung: Decl, file: include/worldgen.h, decl: 2, sub: 0, line: 13 } |  |  | 0.676 |
| ns | 3704 |  | 360 | globals.h — configuration macro roster, part 1 (world and player sizing) | 3.7 |  | 0.695 |
| walker |  | 3798 | 114 | Plaintext::DeclSurface { file: extract_registries.sh } |  |  | 0.695 |
| walker |  | 3937 | 139 | Code::CodeKey { rung: Names, file: include/tools.h, decl: 0, sub: 2, line: 0 } |  |  | 0.698 |
| walker |  | 3946 | 9 | Code::CodeKey { rung: Decl, file: include/tools.h, decl: 26, sub: 0, line: 43 } |  |  | 0.698 |
| walker |  | 3958 | 12 | Code::CodeKey { rung: Decl, file: include/tools.h, decl: 27, sub: 0, line: 46 } |  |  | 0.698 |
| ns | 3978 |  | 274 | globals.h — configuration macro roster, part 2 (feature toggles, including the commented-out ones) | 3.8 |  | 0.685 |
| walker |  | 4140 | 182 | Code::CodeKey { rung: Names, file: include/packets.h, decl: 0, sub: 0, line: 0 } |  |  | 0.707 |
| walker |  | 4149 | 9 | Code::CodeKey { rung: Doc, file: include/packets.h, decl: 1, sub: 0, line: 5 } |  |  | 0.710 |
| walker |  | 4176 | 27 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 11, sub: 0, line: 32 } |  |  | 0.689 |
| ns | 4176 |  | 198 | src/globals.c — definitions of the global state, MOTD and brand | 3.9 |  | 0.689 |
| walker |  | 4203 | 27 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 15, sub: 0, line: 45 } |  |  | 0.689 |
| walker |  | 4230 | 27 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 23, sub: 0, line: 75 } |  |  | 0.689 |
| ns | 4260 |  | 84 | README Configuration section — where the knobs live | 3.10 |  | 0.690 |
| walker |  | 4399 | 169 | Code::CodeKey { rung: Names, file: include/packets.h, decl: 0, sub: 1, line: 0 } |  |  | 0.721 |
| ns | 4402 |  | 142 | README Configuration — the maintainer's tuning guidance | 3.11 |  | 0.723 |
| ns | 4488 |  | 86 | build_registries.js — the complete biome list | 3.12 |  | 0.713 |
| walker |  | 4581 | 182 | Code::CodeKey { rung: Names, file: include/packets.h, decl: 0, sub: 2, line: 0 } |  |  | 0.733 |
| walker |  | 4588 | 7 | Code::CodeKey { rung: Doc, file: include/packets.h, decl: 22, sub: 0, line: 28 } |  |  | 0.735 |
| ns | 4721 |  | 233 | procedures.h — client state and player lifecycle API | 4.1 |  | 0.716 |
| walker |  | 4753 | 165 | Code::CodeKey { rung: Names, file: include/packets.h, decl: 0, sub: 3, line: 0 } |  |  | 0.733 |
| walker |  | 4916 | 163 | Code::CodeKey { rung: Names, file: include/packets.h, decl: 0, sub: 4, line: 0 } |  |  | 0.752 |
| ns | 4985 |  | 264 | procedures.h — metadata broadcast, slot mapping and block predicates | 4.2 |  | 0.733 |
| walker |  | 5063 | 147 | Code::CodeKey { rung: Names, file: include/packets.h, decl: 0, sub: 5, line: 0 } |  |  | 0.742 |
| walker |  | 5223 | 160 | Code::CodeKey { rung: Names, file: include/packets.h, decl: 0, sub: 6, line: 0 } |  |  | 0.765 |
| ns | 5304 |  | 319 | procedures.h — mining, actions, fluids, mobs, tick and entity-data API | 4.3 |  | 0.744 |
| walker |  | 5403 | 180 | Code::CodeKey { rung: Names, file: include/procedures.h, decl: 0, sub: 0, line: 0 } |  |  | 0.760 |
| ns | 5415 |  | 111 | worldgen.h — ChunkAnchor and ChunkFeature | 4.4 |  | 0.765 |
| ns | 5587 |  | 172 | worldgen.h — the complete generation API and the shared chunk_section buffer | 4.5 |  | 0.768 |
| walker |  | 5595 | 192 | Code::CodeKey { rung: Names, file: include/procedures.h, decl: 0, sub: 1, line: 0 } |  |  | 0.785 |
| walker |  | 5762 | 167 | Code::CodeKey { rung: Names, file: include/procedures.h, decl: 0, sub: 2, line: 0 } |  |  | 0.803 |
| ns | 5794 |  | 207 | tools.h — socket I/O and the byte-order writers | 4.6 |  | 0.805 |
| walker |  | 5925 | 163 | Code::CodeKey { rung: Names, file: include/procedures.h, decl: 0, sub: 3, line: 0 } |  |  | 0.815 |
| ns | 5991 |  | 197 | tools.h — the readers, string helpers and RNG | 4.7 |  | 0.817 |
| walker |  | 6044 | 119 | Code::CodeKey { rung: Names, file: include/procedures.h, decl: 0, sub: 4, line: 0 } |  |  | 0.834 |
| ns | 6129 |  | 138 | tools.h — the inline math helpers and the platform time shim | 4.8 |  | 0.827 |
| walker |  | 6183 | 139 | Plaintext::DeclSurface { file: build.sh } |  |  | 0.830 |
| walker |  | 6229 | 46 | Code::CodeKey { rung: Names, file: include/crafting.h, decl: 0, sub: 0, line: 0 } |  |  | 0.831 |
| walker |  | 6259 | 30 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 30, sub: 0, line: 129 } |  |  | 0.831 |
| walker |  | 6283 | 24 | Code::CodeKey { rung: Names, file: include/structures.h, decl: 0, sub: 0, line: 0 } |  |  | 0.831 |
| walker |  | 6314 | 31 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 16, sub: 0, line: 49 } |  |  | 0.831 |
| ns | 6325 |  | 196 | serialize.h in full — persistence API and its compile-time no-op fallback | 4.9 |  | 0.833 |
| walker |  | 6347 | 33 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 20, sub: 0, line: 63 } |  |  | 0.833 |
| walker |  | 6384 | 37 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 17, sub: 0, line: 53 } |  |  | 0.833 |
| ns | 6410 |  | 85 | crafting.h and structures.h in full — the two smallest module APIs | 4.10 |  | 0.831 |
| walker |  | 6422 | 38 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 8, sub: 0, line: 23 } |  |  | 0.831 |
| ns | 6499 |  | 89 | main.c — the project's own module include list | 5.1 |  | 0.822 |
| walker |  | 6697 | 275 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.822 |
| ns | 6755 |  | 256 | main.c — the maintainer's design note on the packet handlers, and handlePacket's signature | 5.2 |  | 0.806 |
| walker |  | 7056 | 359 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.806 |
| ns | 7081 |  | 326 | main.c — packet 0x00 dispatch: handshake, status, login, configuration | 5.3 |  | 0.783 |
| walker |  | 7162 | 106 | Plaintext::DeclSurface { file: src/CMakeLists.txt } |  |  | 0.801 |
| walker |  | 7214 | 52 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 25, sub: 0, line: 103 } |  |  | 0.801 |
| walker |  | 7268 | 54 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 22, sub: 0, line: 71 } |  |  | 0.801 |
| walker |  | 7285 | 17 | Code::CodeKey { rung: Body, file: include/tools.h, decl: 1, sub: 0, line: 8 } |  |  | 0.803 |
| ns | 7350 |  | 269 | main.c — dispatch table, packet ids 0x07 through 0x19 | 5.4 |  | 0.787 |
| walker |  | 7351 | 66 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 27, sub: 0, line: 111 } |  |  | 0.787 |
| walker |  | 7421 | 70 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 31, sub: 0, line: 135 } |  |  | 0.787 |
| ns | 7692 |  | 342 | main.c — dispatch table, the movement case group and ids 0x28 through the default | 5.5 |  | 0.766 |
| walker |  | 7811 | 390 | Plaintext::Whole { file: build.sh } |  |  | 0.784 |
| walker |  | 7836 | 25 | Code::CodeKey { rung: Body, file: include/tools.h, decl: 2, sub: 0, line: 11 } |  |  | 0.787 |
| walker |  | 7920 | 84 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 32, sub: 0, line: 146 } |  |  | 0.787 |
| ns | 8048 |  | 356 | main.c — the single-threaded accept and tick round-robin loop | 5.6 |  | 0.764 |
| walker |  | 8184 | 264 | Code::CodeKey { rung: Names, file: src/globals.c, decl: 0, sub: 0, line: 0 } |  |  | 0.779 |
| walker |  | 8189 | 5 | Code::CodeKey { rung: Decl, file: src/globals.c, decl: 10, sub: 0, line: 45 } |  |  | 0.781 |
| walker |  | 8197 | 8 | Code::CodeKey { rung: Decl, file: src/globals.c, decl: 9, sub: 0, line: 43 } |  |  | 0.783 |
| walker |  | 8291 | 94 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 29, sub: 0, line: 125 } |  |  | 0.783 |
| ns | 8329 |  | 281 | main.c — the ESP32 entry points: FreeRTOS task, WiFi event handler, app_main | 5.7 |  | 0.765 |
| walker |  | 8387 | 96 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 28, sub: 0, line: 118 } |  |  | 0.765 |
| walker |  | 8492 | 105 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 24, sub: 0, line: 93 } |  |  | 0.765 |
| walker |  | 8550 | 58 | Code::CodeKey { rung: Names, file: src/varnum.c, decl: 0, sub: 0, line: 0 } |  |  | 0.765 |
| ns | 8553 |  | 224 | procedures.c — definition locations, part 1 (state, players, slots, block changes) | 6.1 |  | 0.752 |
| walker |  | 8617 | 67 | Code::CodeKey { rung: Names, file: src/crafting.c, decl: 0, sub: 0, line: 0 } |  |  | 0.752 |
| walker |  | 8645 | 28 | Code::CodeKey { rung: Decl, file: src/crafting.c, decl: 2, sub: 0, line: 349 } |  |  | 0.752 |
| ns | 8770 |  | 217 | procedures.c — definition locations, part 2 (mining, predicates, armour, eating, fluids) | 6.2 |  | 0.741 |
| walker |  | 8830 | 185 | Code::CodeKey { rung: Names, file: src/packets.c, decl: 0, sub: 0, line: 0 } |  |  | 0.741 |
| walker |  | 8841 | 11 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 2, sub: 0, line: 49 } |  |  | 0.741 |
| walker |  | 8852 | 11 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 3, sub: 0, line: 66 } |  |  | 0.741 |
| walker |  | 8863 | 11 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 4, sub: 0, line: 85 } |  |  | 0.741 |
| walker |  | 8874 | 11 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 9, sub: 0, line: 183 } |  |  | 0.741 |
| walker |  | 8887 | 13 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 6, sub: 0, line: 137 } |  |  | 0.741 |
| walker |  | 8900 | 13 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 7, sub: 0, line: 151 } |  |  | 0.741 |
| ns | 8902 |  | 132 | procedures.c — definition locations, part 3 (actions, mobs, tick, entity data) | 6.3 |  | 0.734 |
| walker |  | 8913 | 13 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 8, sub: 0, line: 166 } |  |  | 0.734 |
| walker |  | 8928 | 15 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 1, sub: 0, line: 28 } |  |  | 0.734 |
| walker |  | 9093 | 165 | Code::CodeKey { rung: Names, file: src/packets.c, decl: 0, sub: 1, line: 0 } |  |  | 0.734 |
| walker |  | 9104 | 11 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 14, sub: 0, line: 300 } |  |  | 0.734 |
| walker |  | 9116 | 12 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 10, sub: 0, line: 190 } |  |  | 0.734 |
| ns | 9117 |  | 215 | worldgen.c — every definition, including the five private generator stages | 6.4 |  | 0.725 |
| walker |  | 9128 | 12 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 16, sub: 0, line: 323 } |  |  | 0.725 |
| walker |  | 9141 | 13 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 11, sub: 0, line: 245 } |  |  | 0.725 |
| walker |  | 9154 | 13 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 12, sub: 0, line: 275 } |  |  | 0.725 |
| walker |  | 9169 | 15 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 13, sub: 0, line: 287 } |  |  | 0.725 |
| walker |  | 9188 | 19 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 15, sub: 0, line: 314 } |  |  | 0.725 |
| ns | 9304 |  | 187 | serialize.c — the world file path and all five persistence entry points | 6.5 |  | 0.716 |
| walker |  | 9372 | 184 | Code::CodeKey { rung: Names, file: src/packets.c, decl: 0, sub: 2, line: 0 } |  |  | 0.716 |
| walker |  | 9383 | 11 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 20, sub: 0, line: 471 } |  |  | 0.716 |
| walker |  | 9394 | 11 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 22, sub: 0, line: 488 } |  |  | 0.716 |
| walker |  | 9405 | 11 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 23, sub: 0, line: 512 } |  |  | 0.716 |
| walker |  | 9417 | 12 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 19, sub: 0, line: 444 } |  |  | 0.716 |
| walker |  | 9431 | 14 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 17, sub: 0, line: 332 } |  |  | 0.716 |
| walker |  | 9445 | 14 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 21, sub: 0, line: 480 } |  |  | 0.716 |
| walker |  | 9460 | 15 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 18, sub: 0, line: 433 } |  |  | 0.716 |
| ns | 9485 |  | 181 | packets.c — the chat command surface (!msg and !help) | 6.6 |  | 0.710 |
| walker |  | 9618 | 158 | Code::CodeKey { rung: Names, file: src/packets.c, decl: 0, sub: 3, line: 0 } |  |  | 0.710 |
| walker |  | 9629 | 11 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 24, sub: 0, line: 528 } |  |  | 0.710 |
| walker |  | 9640 | 11 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 26, sub: 0, line: 577 } |  |  | 0.710 |
| walker |  | 9652 | 12 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 25, sub: 0, line: 545 } |  |  | 0.710 |
| walker |  | 9664 | 12 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 27, sub: 0, line: 707 } |  |  | 0.710 |
| walker |  | 9676 | 12 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 29, sub: 0, line: 740 } |  |  | 0.710 |
| walker |  | 9690 | 14 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 28, sub: 0, line: 725 } |  |  | 0.710 |
| ns | 9744 |  | 259 | crafting.c — the registerSmeltingRecipe macro and the complete recipe table | 6.7 |  | 0.702 |
| ns | 9771 |  | 27 | Complete .github listings (workflow and issue templates) | 7.1 |  | 0.704 |
| ns | 9877 |  | 106 | README Contribution — the maintainer's rules for changes | 7.2 |  | 0.701 |
| walker |  | 9882 | 192 | Code::CodeKey { rung: Names, file: src/packets.c, decl: 0, sub: 4, line: 0 } |  |  | 0.701 |
| ns | 9931 |  | 54 | extract_registries.sh — the top-level registry extraction sequence | 7.3 |  | 0.698 |
| walker |  | 9943 | 61 | Code::CodeKey { rung: Decl, file: src/packets.c, decl: 37, sub: 0, line: 886 } |  |  | 0.698 |
| walker |  | 9954 | 11 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 37, sub: 0, line: 886 } |  |  | 0.698 |
| ns | 9959 |  | 28 | LICENSE — the license identity line | 7.4 |  | 0.697 |
| walker |  | 9966 | 12 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 30, sub: 0, line: 752 } |  |  | 0.697 |
| walker |  | 9978 | 12 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 38, sub: 0, line: 922 } |  |  | 0.697 |
| walker |  | 9992 | 14 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 32, sub: 0, line: 774 } |  |  | 0.697 |
