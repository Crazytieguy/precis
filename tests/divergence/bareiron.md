Score(3000)=0.736 I=0.859 C=0.631 ns_rows≤3K=20/55 grid(1000/1442/2080/3000/4327/6240/9000)=0.723/0.628/0.590/0.736/0.818/0.852/0.740

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 29 | 29 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 54 |  | 54 | Project identity, tagline, and target Minecraft/protocol version | 1.1 |  | 0.000 |
| walker |  | 77 | 48 | Fs::DirListing { dir: src } |  |  | 0.000 |
| ns | 83 |  | 29 | Complete repository root listing | 1.2 |  | 0.553 |
| ns | 148 |  | 65 | Stated project goal and priority ordering | 1.3 |  | 0.527 |
| walker |  | 175 | 98 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.762 |
| walker |  | 213 | 38 | Fs::DirListing { dir: include } |  |  | 0.847 |
| walker |  | 220 | 7 | Fs::DirListing { dir: .github } |  |  | 0.847 |
| walker |  | 224 | 4 | Fs::DirListing { dir: .github/workflows } |  |  | 0.847 |
| ns | 234 |  | 86 | Complete src/ and include/ listings | 1.4 |  | 0.807 |
| walker |  | 275 | 51 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.825 |
| ns | 285 |  | 51 | All README H2 section headings | 1.5 |  | 0.817 |
| walker |  | 338 | 63 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 1.000 |
| ns | 343 |  | 58 | .gitignore in full — which sources are generated and absent | 1.6 |  | 0.904 |
| walker |  | 444 | 106 | Plaintext::DeclSurface { file: src/CMakeLists.txt } |  |  | 0.918 |
| walker |  | 460 | 16 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.919 |
| ns | 489 |  | 146 | build.sh: the registry prerequisite check and the actual compile command | 1.7 |  | 0.824 |
| ns | 595 |  | 106 | src/CMakeLists.txt in full — the ESP-IDF/PlatformIO build | 1.8 |  | 0.833 |
| ns | 665 |  | 70 | Connection state constants (STATE_NONE through STATE_PLAY) | 2.1 |  | 0.788 |
| walker |  | 789 | 329 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: true } |  |  | 0.790 |
| ns | 885 |  | 220 | packets.h — serverbound declarations, connection and world interaction | 2.2 |  | 0.723 |
| walker |  | 908 | 119 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.723 |
| ns | 1038 |  | 153 | packets.h — remainder of the serverbound declarations | 2.3 |  | 0.669 |
| walker |  | 1151 | 243 | Code::CodeKey { rung: Names, file: include/globals.h, decl: 0, sub: 0, line: 0 } |  |  | 0.672 |
| walker |  | 1156 | 5 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 4, sub: 0, line: 11 } |  |  | 0.672 |
| walker |  | 1167 | 11 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 9, sub: 0, line: 26 } |  |  | 0.672 |
| walker |  | 1178 | 11 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 10, sub: 0, line: 29 } |  |  | 0.672 |
| walker |  | 1191 | 13 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 14, sub: 0, line: 41 } |  |  | 0.672 |
| walker |  | 1205 | 14 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 7, sub: 0, line: 19 } |  |  | 0.672 |
| walker |  | 1219 | 14 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 12, sub: 0, line: 35 } |  |  | 0.672 |
| ns | 1265 |  | 227 | packets.h — clientbound declarations, login and configuration phase | 2.4 |  | 0.624 |
| walker |  | 1459 | 240 | Code::CodeKey { rung: Names, file: include/globals.h, decl: 0, sub: 1, line: 0 } |  |  | 0.628 |
| walker |  | 1467 | 8 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 19, sub: 0, line: 59 } |  |  | 0.628 |
| walker |  | 1477 | 10 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 21, sub: 0, line: 66 } |  |  | 0.628 |
| walker |  | 1490 | 13 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 26, sub: 0, line: 106 } |  |  | 0.628 |
| ns | 1560 |  | 295 | packets.h — clientbound declarations, world and inventory | 2.5 |  | 0.588 |
| walker |  | 1752 | 262 | Code::CodeKey { rung: Names, file: include/globals.h, decl: 0, sub: 2, line: 0 } |  |  | 0.634 |
| ns | 1849 |  | 289 | packets.h — clientbound declarations, entities, health and registries | 2.6 |  | 0.595 |
| walker |  | 1945 | 193 | Code::CodeKey { rung: Names, file: include/globals.h, decl: 0, sub: 3, line: 0 } |  |  | 0.600 |
| ns | 1949 |  | 100 | varnum.h in full — VarInt encoding primitives and error sentinel | 2.7 |  | 0.582 |
| walker |  | 1950 | 5 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 50, sub: 0, line: 186 } |  |  | 0.582 |
| walker |  | 1971 | 21 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 55, sub: 0, line: 255 } |  |  | 0.582 |
| walker |  | 2005 | 34 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 52, sub: 0, line: 191 } |  |  | 0.584 |
| ns | 2019 |  | 70 | globals.h — BlockChange record and the packed-struct pragma | 3.1 |  | 0.589 |
| walker |  | 2054 | 49 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 56, sub: 0, line: 260 } |  |  | 0.590 |
| walker |  | 2185 | 131 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 54, sub: 0, line: 240 } |  |  | 0.595 |
| ns | 2269 |  | 250 | globals.h — PlayerData fields (identity through inventory) | 3.2 |  | 0.545 |
| walker |  | 2376 | 191 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 53, sub: 0, line: 200 } |  |  | 0.616 |
| ns | 2526 |  | 257 | globals.h — PlayerData flag bits and the overloaded flagval fields | 3.3 | 3.2 | 0.584 |
| walker |  | 2675 | 299 | Code::CodeKey { rung: Decl, file: include/globals.h, decl: 53, sub: 1, line: 200 } |  |  | 0.684 |
| walker |  | 2692 | 17 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 13, sub: 0, line: 38 } |  |  | 0.684 |
| walker |  | 2709 | 17 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 33, sub: 0, line: 149 } |  |  | 0.684 |
| ns | 2756 |  | 230 | globals.h — MobData and EntityData | 3.4 |  | 0.701 |
| walker |  | 2807 | 98 | Plaintext::DeclSurface { file: extract_registries.sh } |  |  | 0.701 |
| walker |  | 2826 | 19 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 18, sub: 0, line: 56 } |  |  | 0.701 |
| walker |  | 2845 | 19 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 34, sub: 0, line: 155 } |  |  | 0.701 |
| ns | 2994 |  | 238 | globals.h — all extern global state declarations | 3.5 |  | 0.718 |
| walker |  | 3027 | 182 | Code::CodeKey { rung: Names, file: include/packets.h, decl: 0, sub: 0, line: 0 } |  |  | 0.743 |
| walker |  | 3036 | 9 | Code::CodeKey { rung: Doc, file: include/packets.h, decl: 1, sub: 0, line: 5 } |  |  | 0.748 |
| walker |  | 3205 | 169 | Code::CodeKey { rung: Names, file: include/packets.h, decl: 0, sub: 1, line: 0 } |  |  | 0.787 |
| ns | 3344 |  | 350 | build_registries.js — the template that generates include/registries.h | 3.6 |  | 0.744 |
| walker |  | 3387 | 182 | Code::CodeKey { rung: Names, file: include/packets.h, decl: 0, sub: 2, line: 0 } |  |  | 0.768 |
| walker |  | 3394 | 7 | Code::CodeKey { rung: Doc, file: include/packets.h, decl: 22, sub: 0, line: 28 } |  |  | 0.771 |
| walker |  | 3559 | 165 | Code::CodeKey { rung: Names, file: include/packets.h, decl: 0, sub: 3, line: 0 } |  |  | 0.791 |
| ns | 3704 |  | 360 | globals.h — configuration macro roster, part 1 (world and player sizing) | 3.7 |  | 0.801 |
| walker |  | 3722 | 163 | Code::CodeKey { rung: Names, file: include/packets.h, decl: 0, sub: 4, line: 0 } |  |  | 0.823 |
| walker |  | 3869 | 147 | Code::CodeKey { rung: Names, file: include/packets.h, decl: 0, sub: 5, line: 0 } |  |  | 0.833 |
| ns | 3978 |  | 274 | globals.h — configuration macro roster, part 2 (feature toggles, including the commented-out ones) | 3.8 |  | 0.814 |
| walker |  | 4029 | 160 | Code::CodeKey { rung: Names, file: include/packets.h, decl: 0, sub: 6, line: 0 } |  |  | 0.840 |
| ns | 4176 |  | 198 | src/globals.c — definitions of the global state, MOTD and brand | 3.9 |  | 0.815 |
| walker |  | 4209 | 180 | Code::CodeKey { rung: Names, file: include/procedures.h, decl: 0, sub: 0, line: 0 } |  |  | 0.817 |
| ns | 4260 |  | 84 | README Configuration section — where the knobs live | 3.10 |  | 0.818 |
| walker |  | 4380 | 171 | Code::CodeKey { rung: Names, file: include/tools.h, decl: 0, sub: 0, line: 0 } |  |  | 0.818 |
| ns | 4402 |  | 142 | README Configuration — the maintainer's tuning guidance | 3.11 |  | 0.819 |
| ns | 4488 |  | 86 | build_registries.js — the complete biome list | 3.12 |  | 0.808 |
| walker |  | 4548 | 168 | Code::CodeKey { rung: Names, file: include/tools.h, decl: 0, sub: 1, line: 0 } |  |  | 0.809 |
| ns | 4721 |  | 233 | procedures.h — client state and player lifecycle API | 4.1 |  | 0.803 |
| walker |  | 4768 | 220 | Code::CodeKey { rung: Names, file: include/worldgen.h, decl: 0, sub: 0, line: 0 } |  |  | 0.805 |
| walker |  | 4802 | 34 | Code::CodeKey { rung: Decl, file: include/worldgen.h, decl: 1, sub: 0, line: 6 } |  |  | 0.806 |
| walker |  | 4836 | 34 | Code::CodeKey { rung: Decl, file: include/worldgen.h, decl: 2, sub: 0, line: 13 } |  |  | 0.807 |
| ns | 4985 |  | 264 | procedures.h — metadata broadcast, slot mapping and block predicates | 4.2 |  | 0.786 |
| walker |  | 5010 | 174 | Code::CodeKey { rung: Names, file: include/serialize.h, decl: 0, sub: 0, line: 0 } |  |  | 0.788 |
| walker |  | 5017 | 7 | Code::CodeKey { rung: Decl, file: include/serialize.h, decl: 10, sub: 0, line: 18 } |  |  | 0.788 |
| walker |  | 5032 | 15 | Code::CodeKey { rung: Doc, file: include/serialize.h, decl: 6, sub: 0, line: 12 } |  |  | 0.788 |
| walker |  | 5132 | 100 | Code::CodeKey { rung: Names, file: include/varnum.h, decl: 0, sub: 0, line: 0 } |  |  | 0.803 |
| walker |  | 5290 | 158 | Code::CodeKey { rung: Names, file: include/tools.h, decl: 0, sub: 2, line: 0 } |  |  | 0.806 |
| walker |  | 5297 | 7 | Code::CodeKey { rung: Decl, file: include/tools.h, decl: 27, sub: 0, line: 46 } |  |  | 0.806 |
| ns | 5304 |  | 319 | procedures.h — mining, actions, fluids, mobs, tick and entity-data API | 4.3 |  | 0.783 |
| ns | 5415 |  | 111 | worldgen.h — ChunkAnchor and ChunkFeature | 4.4 |  | 0.788 |
| walker |  | 5489 | 192 | Code::CodeKey { rung: Names, file: include/procedures.h, decl: 0, sub: 1, line: 0 } |  |  | 0.805 |
| ns | 5587 |  | 172 | worldgen.h — the complete generation API and the shared chunk_section buffer | 4.5 |  | 0.807 |
| walker |  | 5656 | 167 | Code::CodeKey { rung: Names, file: include/procedures.h, decl: 0, sub: 2, line: 0 } |  |  | 0.825 |
| ns | 5794 |  | 207 | tools.h — socket I/O and the byte-order writers | 4.6 |  | 0.827 |
| walker |  | 5819 | 163 | Code::CodeKey { rung: Names, file: include/procedures.h, decl: 0, sub: 3, line: 0 } |  |  | 0.836 |
| walker |  | 5938 | 119 | Code::CodeKey { rung: Names, file: include/procedures.h, decl: 0, sub: 4, line: 0 } |  |  | 0.854 |
| ns | 5991 |  | 197 | tools.h — the readers, string helpers and RNG | 4.7 |  | 0.856 |
| walker |  | 6077 | 139 | Plaintext::DeclSurface { file: build.sh } |  |  | 0.859 |
| walker |  | 6123 | 46 | Code::CodeKey { rung: Names, file: include/crafting.h, decl: 0, sub: 0, line: 0 } |  |  | 0.859 |
| ns | 6129 |  | 138 | tools.h — the inline math helpers and the platform time shim | 4.8 |  | 0.852 |
| walker |  | 6147 | 24 | Code::CodeKey { rung: Names, file: include/structures.h, decl: 0, sub: 0, line: 0 } |  |  | 0.852 |
| walker |  | 6174 | 27 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 11, sub: 0, line: 32 } |  |  | 0.852 |
| walker |  | 6201 | 27 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 15, sub: 0, line: 45 } |  |  | 0.852 |
| walker |  | 6228 | 27 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 23, sub: 0, line: 75 } |  |  | 0.852 |
| walker |  | 6258 | 30 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 30, sub: 0, line: 129 } |  |  | 0.852 |
| walker |  | 6289 | 31 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 16, sub: 0, line: 49 } |  |  | 0.852 |
| walker |  | 6322 | 33 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 20, sub: 0, line: 63 } |  |  | 0.852 |
| ns | 6325 |  | 196 | serialize.h in full — persistence API and its compile-time no-op fallback | 4.9 |  | 0.853 |
| ns | 6410 |  | 85 | crafting.h and structures.h in full — the two smallest module APIs | 4.10 |  | 0.851 |
| ns | 6499 |  | 89 | main.c — the project's own module include list | 5.1 |  | 0.843 |
| walker |  | 6597 | 275 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.843 |
| walker |  | 6634 | 37 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 17, sub: 0, line: 53 } |  |  | 0.843 |
| walker |  | 6672 | 38 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 8, sub: 0, line: 23 } |  |  | 0.843 |
| walker |  | 6724 | 52 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 25, sub: 0, line: 103 } |  |  | 0.843 |
| ns | 6755 |  | 256 | main.c — the maintainer's design note on the packet handlers, and handlePacket's signature | 5.2 |  | 0.826 |
| walker |  | 6778 | 54 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 22, sub: 0, line: 71 } |  |  | 0.826 |
| walker |  | 6795 | 17 | Code::CodeKey { rung: Body, file: include/tools.h, decl: 1, sub: 0, line: 8 } |  |  | 0.828 |
| ns | 7081 |  | 326 | main.c — packet 0x00 dispatch: handshake, status, login, configuration | 5.3 |  | 0.804 |
| walker |  | 7284 | 489 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.804 |
| walker |  | 7350 | 66 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 27, sub: 0, line: 111 } |  |  | 0.788 |
| ns | 7350 |  | 269 | main.c — dispatch table, packet ids 0x07 through 0x19 | 5.4 |  | 0.788 |
| ns | 7692 |  | 342 | main.c — dispatch table, the movement case group and ids 0x28 through the default | 5.5 |  | 0.767 |
| walker |  | 7740 | 390 | Plaintext::Whole { file: build.sh } |  |  | 0.785 |
| walker |  | 7810 | 70 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 31, sub: 0, line: 135 } |  |  | 0.785 |
| walker |  | 7835 | 25 | Code::CodeKey { rung: Body, file: include/tools.h, decl: 2, sub: 0, line: 11 } |  |  | 0.788 |
| walker |  | 7919 | 84 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 32, sub: 0, line: 146 } |  |  | 0.788 |
| walker |  | 8013 | 94 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 29, sub: 0, line: 125 } |  |  | 0.788 |
| ns | 8048 |  | 356 | main.c — the single-threaded accept and tick round-robin loop | 5.6 |  | 0.766 |
| walker |  | 8109 | 96 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 28, sub: 0, line: 118 } |  |  | 0.766 |
| walker |  | 8214 | 105 | Code::CodeKey { rung: Doc, file: include/globals.h, decl: 24, sub: 0, line: 93 } |  |  | 0.766 |
| ns | 8329 |  | 281 | main.c — the ESP32 entry points: FreeRTOS task, WiFi event handler, app_main | 5.7 |  | 0.749 |
| walker |  | 8396 | 182 | Code::CodeKey { rung: Names, file: src/procedures.c, decl: 0, sub: 0, line: 0 } |  |  | 0.749 |
| walker |  | 8412 | 16 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 5, sub: 0, line: 54 } |  |  | 0.749 |
| walker |  | 8428 | 16 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 6, sub: 0, line: 75 } |  |  | 0.749 |
| walker |  | 8444 | 16 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 9, sub: 0, line: 146 } |  |  | 0.749 |
| walker |  | 8463 | 19 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 8, sub: 0, line: 130 } |  |  | 0.749 |
| ns | 8553 |  | 224 | procedures.c — definition locations, part 1 (state, players, slots, block changes) | 6.1 |  | 0.739 |
| walker |  | 8654 | 191 | Code::CodeKey { rung: Names, file: src/procedures.c, decl: 0, sub: 1, line: 0 } |  |  | 0.752 |
| walker |  | 8671 | 17 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 15, sub: 0, line: 321 } |  |  | 0.752 |
| walker |  | 8689 | 18 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 10, sub: 0, line: 177 } |  |  | 0.752 |
| walker |  | 8712 | 23 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 16, sub: 0, line: 396 } |  |  | 0.752 |
| ns | 8770 |  | 217 | procedures.c — definition locations, part 2 (mining, predicates, armour, eating, fluids) | 6.2 |  | 0.741 |
| walker |  | 8878 | 166 | Code::CodeKey { rung: Names, file: src/procedures.c, decl: 0, sub: 2, line: 0 } |  |  | 0.746 |
| walker |  | 8892 | 14 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 25, sub: 0, line: 789 } |  |  | 0.746 |
| ns | 8902 |  | 132 | procedures.c — definition locations, part 3 (actions, mobs, tick, entity data) | 6.3 |  | 0.739 |
| walker |  | 8907 | 15 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 19, sub: 0, line: 495 } |  |  | 0.739 |
| walker |  | 8924 | 17 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 24, sub: 0, line: 775 } |  |  | 0.739 |
| walker |  | 8942 | 18 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 22, sub: 0, line: 721 } |  |  | 0.739 |
| walker |  | 8961 | 19 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 23, sub: 0, line: 746 } |  |  | 0.739 |
| ns | 9117 |  | 215 | worldgen.c — every definition, including the five private generator stages | 6.4 |  | 0.730 |
| walker |  | 9132 | 171 | Code::CodeKey { rung: Names, file: src/procedures.c, decl: 0, sub: 3, line: 0 } |  |  | 0.739 |
| walker |  | 9146 | 14 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 30, sub: 0, line: 858 } |  |  | 0.739 |
| walker |  | 9161 | 15 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 32, sub: 0, line: 939 } |  |  | 0.739 |
| walker |  | 9178 | 17 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 26, sub: 0, line: 802 } |  |  | 0.739 |
| walker |  | 9195 | 17 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 27, sub: 0, line: 812 } |  |  | 0.739 |
| ns | 9304 |  | 187 | serialize.c — the world file path and all five persistence entry points | 6.5 |  | 0.731 |
| walker |  | 9360 | 165 | Code::CodeKey { rung: Names, file: src/procedures.c, decl: 0, sub: 4, line: 0 } |  |  | 0.737 |
| walker |  | 9368 | 8 | Code::CodeKey { rung: Decl, file: src/procedures.c, decl: 37, sub: 0, line: 1121 } |  |  | 0.737 |
| walker |  | 9386 | 18 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 37, sub: 0, line: 1121 } |  |  | 0.737 |
| ns | 9485 |  | 181 | packets.c — the chat command surface (!msg and !help) | 6.6 |  | 0.730 |
| walker |  | 9643 | 257 | Code::CodeKey { rung: Names, file: src/procedures.c, decl: 0, sub: 5, line: 0 } |  |  | 0.740 |
| walker |  | 9651 | 8 | Code::CodeKey { rung: Decl, file: src/procedures.c, decl: 44, sub: 0, line: 1928 } |  |  | 0.740 |
| walker |  | 9668 | 17 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 46, sub: 0, line: 1965 } |  |  | 0.740 |
| walker |  | 9687 | 19 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 47, sub: 0, line: 1983 } |  |  | 0.740 |
| walker |  | 9719 | 32 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 31, sub: 0, line: 909 } |  |  | 0.740 |
| ns | 9744 |  | 259 | crafting.c — the registerSmeltingRecipe macro and the complete recipe table | 6.7 |  | 0.732 |
| walker |  | 9752 | 33 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 17, sub: 0, line: 434 } |  |  | 0.732 |
| ns | 9771 |  | 27 | Complete .github listings (workflow and issue templates) | 7.1 |  | 0.734 |
| walker |  | 9785 | 33 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 33, sub: 0, line: 954 } |  |  | 0.734 |
| walker |  | 9819 | 34 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 44, sub: 0, line: 1928 } |  |  | 0.734 |
| walker |  | 9856 | 37 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 43, sub: 0, line: 1623 } |  |  | 0.734 |
| ns | 9877 |  | 106 | README Contribution — the maintainer's rules for changes | 7.2 |  | 0.731 |
| walker |  | 9895 | 39 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 29, sub: 0, line: 831 } |  |  | 0.731 |
| ns | 9931 |  | 54 | extract_registries.sh — the top-level registry extraction sequence | 7.3 |  | 0.728 |
| walker |  | 9940 | 45 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 21, sub: 0, line: 652 } |  |  | 0.728 |
| ns | 9959 |  | 28 | LICENSE — the license identity line | 7.4 |  | 0.726 |
| walker |  | 9992 | 52 | Code::CodeKey { rung: Doc, file: src/procedures.c, decl: 34, sub: 0, line: 990 } |  |  | 0.726 |
