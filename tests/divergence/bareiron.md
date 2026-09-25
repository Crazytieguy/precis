Score(3000)=0.665 I=0.825 C=0.536 ns_rows≤3K=20/55 grid(1000/1442/2080/3000/4327/6240/9000)=0.648/0.563/0.540/0.665/0.685/0.824/0.727

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
| walker |  | 971 | 169 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.648 |
| ns | 1038 |  | 153 | packets.h — remainder of the serverbound declarations | 2.3 |  | 0.600 |
| walker |  | 1251 | 280 | Code::CodeKey { rung: Names, file: include/globals.h, decl: 0, sub: 0, line: 0 } |  |  | 0.603 |
| walker |  | 1260 | 9 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 1, sub: 0, line: 7 } |  |  | 0.603 |
| ns | 1265 |  | 227 | packets.h — clientbound declarations, login and configuration phase | 2.4 |  | 0.560 |
| walker |  | 1270 | 10 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 4, sub: 0, line: 11 } |  |  | 0.561 |
| walker |  | 1281 | 11 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 9, sub: 0, line: 26 } |  |  | 0.561 |
| walker |  | 1292 | 11 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 10, sub: 0, line: 29 } |  |  | 0.561 |
| walker |  | 1305 | 13 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 14, sub: 0, line: 41 } |  |  | 0.561 |
| walker |  | 1319 | 14 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 7, sub: 0, line: 19 } |  |  | 0.561 |
| walker |  | 1333 | 14 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 12, sub: 0, line: 35 } |  |  | 0.561 |
| walker |  | 1350 | 17 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 13, sub: 0, line: 38 } |  |  | 0.561 |
| ns | 1560 |  | 295 | packets.h — clientbound declarations, world and inventory | 2.5 |  | 0.525 |
| walker |  | 1649 | 299 | Code::CodeKey { rung: Names, file: include/globals.h, decl: 0, sub: 1, line: 0 } |  |  | 0.574 |
| walker |  | 1657 | 8 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 19, sub: 0, line: 59 } |  |  | 0.574 |
| walker |  | 1667 | 10 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 21, sub: 0, line: 66 } |  |  | 0.574 |
| walker |  | 1680 | 13 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 26, sub: 0, line: 106 } |  |  | 0.574 |
| ns | 1849 |  | 289 | packets.h — clientbound declarations, entities, health and registries | 2.6 |  | 0.538 |
| walker |  | 1896 | 216 | Code::CodeKey { rung: Names, file: include/globals.h, decl: 0, sub: 2, line: 0 } |  |  | 0.543 |
| walker |  | 1901 | 5 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 42, sub: 0, line: 186 } |  |  | 0.544 |
| walker |  | 1909 | 8 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 41, sub: 0, line: 184 } |  |  | 0.544 |
| walker |  | 1930 | 21 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 47, sub: 0, line: 255 } |  |  | 0.544 |
| ns | 1949 |  | 100 | varnum.h in full — VarInt encoding primitives and error sentinel | 2.7 |  | 0.527 |
| walker |  | 1964 | 34 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 44, sub: 0, line: 191 } |  |  | 0.529 |
| walker |  | 2013 | 49 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 48, sub: 0, line: 260 } |  |  | 0.530 |
| ns | 2019 |  | 70 | globals.h — BlockChange record and the packed-struct pragma | 3.1 |  | 0.538 |
| walker |  | 2144 | 131 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 46, sub: 0, line: 240 } |  |  | 0.543 |
| walker |  | 2163 | 19 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 18, sub: 0, line: 56 } |  |  | 0.543 |
| ns | 2269 |  | 250 | globals.h — PlayerData fields (identity through inventory) | 3.2 |  | 0.497 |
| walker |  | 2317 | 154 | Code::CodeKey { rung: Names, file: include/serialize.h, decl: 0, sub: 0, line: 0 } |  |  | 0.498 |
| walker |  | 2324 | 7 | Code::CodeKey { rung: Decl, file: include/serialize.h, decl: 6, sub: 0, line: 12 } |  |  | 0.498 |
| walker |  | 2331 | 7 | Code::CodeKey { rung: Decl, file: include/serialize.h, decl: 10, sub: 0, line: 18 } |  |  | 0.498 |
| walker |  | 2344 | 13 | Code::CodeKey { rung: Decl, file: include/serialize.h, decl: 1, sub: 0, line: 6 } |  |  | 0.499 |
| walker |  | 2359 | 15 | Code::CodeKey { rung: Doc, file: include/serialize.h, decl: 6, sub: 0, line: 12 } |  |  | 0.499 |
| walker |  | 2459 | 100 | Code::CodeKey { rung: Names, file: include/varnum.h, decl: 0, sub: 0, line: 0 } |  |  | 0.536 |
| ns | 2526 |  | 257 | globals.h — PlayerData flag bits and the overloaded flagval fields | 3.3 | 3.2 | 0.508 |
| walker |  | 2670 | 211 | Code::CodeKey { rung: Names, file: include/tools.h, decl: 0, sub: 0, line: 0 } |  |  | 0.509 |
| walker |  | 2687 | 17 | Code::CodeKey { rung: Body, file: include/tools.h, decl: 1, sub: 0, line: 8 } |  |  | 0.509 |
| ns | 2756 |  | 230 | globals.h — MobData and EntityData | 3.4 |  | 0.551 |
| walker |  | 2917 | 230 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 45, sub: 0, line: 200 } |  |  | 0.630 |
| ns | 2994 |  | 238 | globals.h — all extern global state declarations | 3.5 |  | 0.654 |
| walker |  | 3177 | 260 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 45, sub: 1, line: 200 } |  |  | 0.709 |
| ns | 3344 |  | 350 | build_registries.js — the template that generates include/registries.h | 3.6 |  | 0.670 |
| walker |  | 3397 | 220 | Code::CodeKey { rung: Names, file: include/worldgen.h, decl: 0, sub: 0, line: 0 } |  |  | 0.672 |
| walker |  | 3431 | 34 | Code::CodeKey { rung: Decl, file: include/worldgen.h, decl: 1, sub: 0, line: 6 } |  |  | 0.673 |
| walker |  | 3465 | 34 | Code::CodeKey { rung: Decl, file: include/worldgen.h, decl: 2, sub: 0, line: 13 } |  |  | 0.674 |
| ns | 3704 |  | 360 | globals.h — configuration macro roster, part 1 (world and player sizing) | 3.7 |  | 0.693 |
| walker |  | 3732 | 267 | Code::CodeKey { rung: Names, file: include/tools.h, decl: 0, sub: 1, line: 0 } |  |  | 0.696 |
| walker |  | 3741 | 9 | Code::CodeKey { rung: Decl, file: include/tools.h, decl: 26, sub: 0, line: 43 } |  |  | 0.697 |
| walker |  | 3753 | 12 | Code::CodeKey { rung: Decl, file: include/tools.h, decl: 27, sub: 0, line: 46 } |  |  | 0.697 |
| walker |  | 3867 | 114 | Plaintext::DeclSurface { file: extract_registries.sh } |  |  | 0.697 |
| ns | 3978 |  | 274 | globals.h — configuration macro roster, part 2 (feature toggles, including the commented-out ones) | 3.8 |  | 0.672 |
| walker |  | 4078 | 211 | Code::CodeKey { rung: Names, file: include/packets.h, decl: 0, sub: 0, line: 0 } |  |  | 0.700 |
| walker |  | 4087 | 9 | Code::CodeKey { rung: Doc, file: include/packets.h, decl: 1, sub: 0, line: 5 } |  |  | 0.704 |
| walker |  | 4114 | 27 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 11, sub: 0, line: 32 } |  |  | 0.704 |
| walker |  | 4141 | 27 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 15, sub: 0, line: 45 } |  |  | 0.704 |
| walker |  | 4168 | 27 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 23, sub: 0, line: 75 } |  |  | 0.704 |
| ns | 4176 |  | 198 | src/globals.c — definitions of the global state, MOTD and brand | 3.9 |  | 0.683 |
| ns | 4260 |  | 84 | README Configuration section — where the knobs live | 3.10 |  | 0.684 |
| walker |  | 4388 | 220 | Code::CodeKey { rung: Names, file: include/procedures.h, decl: 0, sub: 0, line: 0 } |  |  | 0.687 |
| ns | 4402 |  | 142 | README Configuration — the maintainer's tuning guidance | 3.11 |  | 0.689 |
| ns | 4488 |  | 86 | build_registries.js — the complete biome list | 3.12 |  | 0.679 |
| walker |  | 4601 | 213 | Code::CodeKey { rung: Names, file: include/procedures.h, decl: 0, sub: 1, line: 0 } |  |  | 0.681 |
| ns | 4721 |  | 233 | procedures.h — client state and player lifecycle API | 4.1 |  | 0.691 |
| walker |  | 4797 | 196 | Code::CodeKey { rung: Names, file: include/procedures.h, decl: 0, sub: 2, line: 0 } |  |  | 0.692 |
| ns | 4985 |  | 264 | procedures.h — metadata broadcast, slot mapping and block predicates | 4.2 |  | 0.702 |
| walker |  | 4989 | 192 | Code::CodeKey { rung: Names, file: include/procedures.h, decl: 0, sub: 3, line: 0 } |  |  | 0.704 |
| walker |  | 5225 | 236 | Code::CodeKey { rung: Names, file: include/packets.h, decl: 0, sub: 1, line: 0 } |  |  | 0.734 |
| walker |  | 5232 | 7 | Code::CodeKey { rung: Doc, file: include/packets.h, decl: 22, sub: 0, line: 28 } |  |  | 0.736 |
| ns | 5304 |  | 319 | procedures.h — mining, actions, fluids, mobs, tick and entity-data API | 4.3 |  | 0.742 |
| ns | 5415 |  | 111 | worldgen.h — ChunkAnchor and ChunkFeature | 4.4 |  | 0.747 |
| walker |  | 5426 | 194 | Code::CodeKey { rung: Names, file: include/packets.h, decl: 0, sub: 2, line: 0 } |  |  | 0.770 |
| ns | 5587 |  | 172 | worldgen.h — the complete generation API and the shared chunk_section buffer | 4.5 |  | 0.772 |
| walker |  | 5646 | 220 | Code::CodeKey { rung: Names, file: include/packets.h, decl: 0, sub: 3, line: 0 } |  |  | 0.792 |
| ns | 5794 |  | 207 | tools.h — socket I/O and the byte-order writers | 4.6 |  | 0.795 |
| walker |  | 5840 | 194 | Code::CodeKey { rung: Names, file: include/packets.h, decl: 0, sub: 4, line: 0 } |  |  | 0.807 |
| walker |  | 5953 | 113 | Code::CodeKey { rung: Names, file: include/packets.h, decl: 0, sub: 5, line: 0 } |  |  | 0.823 |
| ns | 5991 |  | 197 | tools.h — the readers, string helpers and RNG | 4.7 |  | 0.826 |
| walker |  | 6092 | 139 | Plaintext::DeclSurface { file: build.sh } |  |  | 0.829 |
| ns | 6129 |  | 138 | tools.h — the inline math helpers and the platform time shim | 4.8 |  | 0.824 |
| walker |  | 6138 | 46 | Code::CodeKey { rung: Names, file: include/crafting.h, decl: 0, sub: 0, line: 0 } |  |  | 0.824 |
| walker |  | 6162 | 24 | Code::CodeKey { rung: Names, file: include/structures.h, decl: 0, sub: 0, line: 0 } |  |  | 0.824 |
| walker |  | 6193 | 31 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 16, sub: 0, line: 49 } |  |  | 0.824 |
| walker |  | 6226 | 33 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 20, sub: 0, line: 63 } |  |  | 0.824 |
| walker |  | 6263 | 37 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 17, sub: 0, line: 53 } |  |  | 0.824 |
| walker |  | 6288 | 25 | Code::CodeKey { rung: Body, file: include/tools.h, decl: 2, sub: 0, line: 11 } |  |  | 0.827 |
| ns | 6325 |  | 196 | serialize.h in full — persistence API and its compile-time no-op fallback | 4.9 |  | 0.829 |
| walker |  | 6326 | 38 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 8, sub: 0, line: 23 } |  |  | 0.829 |
| ns | 6410 |  | 85 | crafting.h and structures.h in full — the two smallest module APIs | 4.10 |  | 0.827 |
| ns | 6499 |  | 89 | main.c — the project's own module include list | 5.1 |  | 0.819 |
| walker |  | 6601 | 275 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.819 |
| ns | 6755 |  | 256 | main.c — the maintainer's design note on the packet handlers, and handlePacket's signature | 5.2 |  | 0.803 |
| walker |  | 6960 | 359 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.803 |
| walker |  | 7066 | 106 | Plaintext::DeclSurface { file: src/CMakeLists.txt } |  |  | 0.821 |
| ns | 7081 |  | 326 | main.c — packet 0x00 dispatch: handshake, status, login, configuration | 5.3 |  | 0.798 |
| walker |  | 7118 | 52 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 25, sub: 0, line: 103 } |  |  | 0.798 |
| walker |  | 7172 | 54 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 22, sub: 0, line: 71 } |  |  | 0.798 |
| ns | 7350 |  | 269 | main.c — dispatch table, packet ids 0x07 through 0x19 | 5.4 |  | 0.782 |
| walker |  | 7562 | 390 | Plaintext::Whole { file: build.sh } |  |  | 0.800 |
| ns | 7692 |  | 342 | main.c — dispatch table, the movement case group and ids 0x28 through the default | 5.5 |  | 0.779 |
| walker |  | 7826 | 264 | Code::CodeKey { rung: Names, file: src/globals.c, decl: 0, sub: 0, line: 0 } |  |  | 0.794 |
| walker |  | 7831 | 5 | Code::CodeKey { rung: Decl, file: src/globals.c, decl: 10, sub: 0, line: 45 } |  |  | 0.796 |
| walker |  | 7839 | 8 | Code::CodeKey { rung: Decl, file: src/globals.c, decl: 9, sub: 0, line: 43 } |  |  | 0.798 |
| walker |  | 7944 | 105 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 24, sub: 0, line: 93 } |  |  | 0.798 |
| walker |  | 8002 | 58 | Code::CodeKey { rung: Names, file: src/varnum.c, decl: 0, sub: 0, line: 0 } |  |  | 0.798 |
| ns | 8048 |  | 356 | main.c — the single-threaded accept and tick round-robin loop | 5.6 |  | 0.775 |
| walker |  | 8069 | 67 | Code::CodeKey { rung: Names, file: src/crafting.c, decl: 0, sub: 0, line: 0 } |  |  | 0.775 |
| walker |  | 8097 | 28 | Code::CodeKey { rung: Decl, file: src/crafting.c, decl: 2, sub: 0, line: 349 } |  |  | 0.775 |
| ns | 8329 |  | 281 | main.c — the ESP32 entry points: FreeRTOS task, WiFi event handler, app_main | 5.7 |  | 0.758 |
| walker |  | 8331 | 234 | Code::CodeKey { rung: Names, file: src/packets.c, decl: 0, sub: 0, line: 0 } |  |  | 0.758 |
| walker |  | 8342 | 11 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 2, sub: 0, line: 49 } |  |  | 0.758 |
| walker |  | 8353 | 11 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 3, sub: 0, line: 66 } |  |  | 0.758 |
| walker |  | 8364 | 11 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 4, sub: 0, line: 85 } |  |  | 0.758 |
| walker |  | 8375 | 11 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 9, sub: 0, line: 183 } |  |  | 0.758 |
| walker |  | 8387 | 12 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 10, sub: 0, line: 190 } |  |  | 0.758 |
| walker |  | 8400 | 13 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 6, sub: 0, line: 137 } |  |  | 0.758 |
| walker |  | 8413 | 13 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 7, sub: 0, line: 151 } |  |  | 0.758 |
| walker |  | 8426 | 13 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 8, sub: 0, line: 166 } |  |  | 0.758 |
| walker |  | 8439 | 13 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 11, sub: 0, line: 245 } |  |  | 0.758 |
| walker |  | 8454 | 15 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 1, sub: 0, line: 28 } |  |  | 0.758 |
| ns | 8553 |  | 224 | procedures.c — definition locations, part 1 (state, players, slots, block changes) | 6.1 |  | 0.745 |
| walker |  | 8685 | 231 | Code::CodeKey { rung: Names, file: src/packets.c, decl: 0, sub: 1, line: 0 } |  |  | 0.745 |
| walker |  | 8696 | 11 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 14, sub: 0, line: 300 } |  |  | 0.745 |
| walker |  | 8707 | 11 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 20, sub: 0, line: 471 } |  |  | 0.745 |
| walker |  | 8719 | 12 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 16, sub: 0, line: 323 } |  |  | 0.745 |
| walker |  | 8731 | 12 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 19, sub: 0, line: 444 } |  |  | 0.745 |
| walker |  | 8744 | 13 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 12, sub: 0, line: 275 } |  |  | 0.745 |
| walker |  | 8758 | 14 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 17, sub: 0, line: 332 } |  |  | 0.745 |
| ns | 8770 |  | 217 | procedures.c — definition locations, part 2 (mining, predicates, armour, eating, fluids) | 6.2 |  | 0.734 |
| walker |  | 8773 | 15 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 13, sub: 0, line: 287 } |  |  | 0.734 |
| walker |  | 8788 | 15 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 18, sub: 0, line: 433 } |  |  | 0.734 |
| walker |  | 8807 | 19 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 15, sub: 0, line: 314 } |  |  | 0.734 |
| ns | 8902 |  | 132 | procedures.c — definition locations, part 3 (actions, mobs, tick, entity data) | 6.3 |  | 0.727 |
| walker |  | 9034 | 227 | Code::CodeKey { rung: Names, file: src/packets.c, decl: 0, sub: 2, line: 0 } |  |  | 0.727 |
| walker |  | 9045 | 11 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 22, sub: 0, line: 488 } |  |  | 0.727 |
| walker |  | 9056 | 11 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 23, sub: 0, line: 512 } |  |  | 0.727 |
| walker |  | 9067 | 11 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 24, sub: 0, line: 528 } |  |  | 0.727 |
| walker |  | 9078 | 11 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 26, sub: 0, line: 577 } |  |  | 0.727 |
| walker |  | 9090 | 12 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 25, sub: 0, line: 545 } |  |  | 0.727 |
| walker |  | 9102 | 12 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 27, sub: 0, line: 707 } |  |  | 0.727 |
| walker |  | 9114 | 12 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 29, sub: 0, line: 740 } |  |  | 0.727 |
| ns | 9117 |  | 215 | worldgen.c — every definition, including the five private generator stages | 6.4 |  | 0.718 |
| walker |  | 9128 | 14 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 21, sub: 0, line: 480 } |  |  | 0.718 |
| walker |  | 9142 | 14 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 28, sub: 0, line: 725 } |  |  | 0.718 |
| ns | 9304 |  | 187 | serialize.c — the world file path and all five persistence entry points | 6.5 |  | 0.710 |
| walker |  | 9355 | 213 | Code::CodeKey { rung: Names, file: src/packets.c, decl: 0, sub: 3, line: 0 } |  |  | 0.710 |
| walker |  | 9416 | 61 | Code::CodeKey { rung: Decl, file: src/packets.c, decl: 37, sub: 0, line: 886 } |  |  | 0.710 |
| walker |  | 9427 | 11 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 37, sub: 0, line: 886 } |  |  | 0.710 |
| walker |  | 9439 | 12 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 30, sub: 0, line: 752 } |  |  | 0.710 |
| walker |  | 9451 | 12 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 38, sub: 0, line: 922 } |  |  | 0.710 |
| walker |  | 9465 | 14 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 32, sub: 0, line: 774 } |  |  | 0.710 |
| walker |  | 9479 | 14 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 35, sub: 0, line: 836 } |  |  | 0.710 |
| ns | 9485 |  | 181 | packets.c — the chat command surface (!msg and !help) | 6.6 |  | 0.703 |
| walker |  | 9494 | 15 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 33, sub: 0, line: 811 } |  |  | 0.703 |
| walker |  | 9509 | 15 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 34, sub: 0, line: 825 } |  |  | 0.703 |
| walker |  | 9524 | 15 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 39, sub: 0, line: 942 } |  |  | 0.703 |
| walker |  | 9542 | 18 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 36, sub: 0, line: 866 } |  |  | 0.703 |
| ns | 9744 |  | 259 | crafting.c — the registerSmeltingRecipe macro and the complete recipe table | 6.7 |  | 0.695 |
| walker |  | 9767 | 225 | Code::CodeKey { rung: Names, file: src/packets.c, decl: 0, sub: 4, line: 0 } |  |  | 0.695 |
| ns | 9771 |  | 27 | Complete .github listings (workflow and issue templates) | 7.1 |  | 0.697 |
| walker |  | 9810 | 43 | Code::CodeKey { rung: Decl, file: src/packets.c, decl: 41, sub: 0, line: 965 } |  |  | 0.697 |
| walker |  | 9821 | 11 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 40, sub: 0, line: 954 } |  |  | 0.697 |
| walker |  | 9833 | 12 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 41, sub: 0, line: 965 } |  |  | 0.697 |
| walker |  | 9845 | 12 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 42, sub: 0, line: 995 } |  |  | 0.697 |
| walker |  | 9857 | 12 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 44, sub: 0, line: 1026 } |  |  | 0.697 |
| walker |  | 9869 | 12 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 45, sub: 0, line: 1041 } |  |  | 0.697 |
| ns | 9877 |  | 106 | README Contribution — the maintainer's rules for changes | 7.2 |  | 0.695 |
| walker |  | 9881 | 12 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 46, sub: 0, line: 1054 } |  |  | 0.695 |
| walker |  | 9893 | 12 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 47, sub: 0, line: 1088 } |  |  | 0.695 |
| walker |  | 9905 | 12 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 48, sub: 0, line: 1105 } |  |  | 0.695 |
| walker |  | 9917 | 12 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 49, sub: 0, line: 1122 } |  |  | 0.695 |
| walker |  | 9930 | 13 | Code::CodeKey { rung: Doc, file: src/packets.c, decl: 43, sub: 0, line: 1009 } |  |  | 0.695 |
| ns | 9931 |  | 54 | extract_registries.sh — the top-level registry extraction sequence | 7.3 |  | 0.692 |
| ns | 9959 |  | 28 | LICENSE — the license identity line | 7.4 |  | 0.691 |
| walker |  | 9987 | 57 | Code::CodeKey { rung: Names, file: src/procedures.c, decl: 0, sub: 0, line: 0 } |  |  | 0.691 |
