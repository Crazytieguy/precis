Score(3000)=0.622 I=0.830 C=0.466 ns_rows≤3K=20/55 (reached=10 partial=1 missing=9)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 32 | 32 | listing of '.' |  |  | 0.000 |
| ns | 54 |  | 54 | Project identity, tagline, and target Minecraft/protocol version | 1.1 |  | 0.000 |
| walker |  | 69 | 37 | listing of 'include' |  |  | 0.000 |
| ns | 86 |  | 32 | Complete repository root listing | 1.2 |  | 0.518 |
| walker |  | 116 | 47 | listing of 'src' |  |  | 0.654 |
| walker |  | 124 | 8 | listing of '.github' |  |  | 0.654 |
| walker |  | 127 | 3 | listing of '.github/workflows' |  |  | 0.654 |
| ns | 151 |  | 65 | Stated project goal and priority ordering | 1.3 |  | 0.623 |
| walker |  | 184 | 57 | c whole header in include/structures.h |  |  | 0.623 |
| ns | 235 |  | 84 | Complete src/ and include/ listings | 1.4 |  | 0.638 |
| walker |  | 282 | 98 | README headline in README.md |  |  | 0.807 |
| ns | 286 |  | 51 | All README H2 section headings | 1.5 |  | 0.741 |
| ns | 344 |  | 58 | .gitignore in full — which sources are generated and absent | 1.6 |  | 0.670 |
| walker |  | 378 | 96 | c whole header in include/crafting.h |  |  | 0.671 |
| walker |  | 429 | 51 | headings outline in README.md |  |  | 0.739 |
| ns | 490 |  | 146 | build.sh: the registry prerequisite check and the actual compile command | 1.7 |  | 0.663 |
| walker |  | 492 | 63 | README.md section #0 |  |  | 0.812 |
| ns | 596 |  | 106 | src/CMakeLists.txt in full — the ESP-IDF/PlatformIO build | 1.8 |  | 0.747 |
| walker |  | 642 | 150 | c whole header in include/varnum.h |  |  | 0.752 |
| ns | 666 |  | 70 | Connection state constants (STATE_NONE through STATE_PLAY) | 2.1 |  | 0.712 |
| ns | 886 |  | 220 | packets.h — serverbound declarations, connection and world interaction | 2.2 |  | 0.651 |
| ns | 1039 |  | 153 | packets.h — remainder of the serverbound declarations | 2.3 |  | 0.603 |
| walker |  | 1171 | 529 | plaintext config build.sh |  |  | 0.682 |
| ns | 1266 |  | 227 | packets.h — clientbound declarations, login and configuration phase | 2.4 |  | 0.634 |
| walker |  | 1415 | 244 | c whole header in include/serialize.h |  |  | 0.636 |
| ns | 1561 |  | 295 | packets.h — clientbound declarations, world and inventory | 2.5 |  | 0.596 |
| walker |  | 1751 | 336 | c whole header in include/worldgen.h |  |  | 0.599 |
| walker |  | 1778 | 27 | c decl names surface in src/main.c |  |  | 0.599 |
| walker |  | 1778 | 0 | c decl at src/main.c:68 |  |  | 0.599 |
| walker |  | 1793 | 15 | listing of '.github/ISSUE_TEMPLATE' |  |  | 0.600 |
| ns | 1850 |  | 289 | packets.h — clientbound declarations, entities, health and registries | 2.6 |  | 0.563 |
| ns | 1950 |  | 100 | varnum.h in full — VarInt encoding primitives and error sentinel | 2.7 |  | 0.584 |
| ns | 2020 |  | 70 | globals.h — BlockChange record and the packed-struct pragma | 3.1 |  | 0.564 |
| walker |  | 2164 | 371 | c decl names surface #1 in include/globals.h |  |  | 0.608 |
| walker |  | 2164 | 0 | c decl at include/globals.h:103 |  |  | 0.608 |
| walker |  | 2164 | 0 | c decl at include/globals.h:106 |  |  | 0.608 |
| walker |  | 2172 | 8 | c decl at include/globals.h:200 |  |  | 0.608 |
| walker |  | 2193 | 21 | c decl at include/globals.h:255 |  |  | 0.608 |
| walker |  | 2237 | 44 | c decl at include/globals.h:191 |  |  | 0.632 |
| ns | 2270 |  | 250 | globals.h — PlayerData fields (identity through inventory) | 3.2 |  | 0.579 |
| walker |  | 2294 | 57 | c decl at include/globals.h:260 |  |  | 0.580 |
| walker |  | 2307 | 13 | c decl doc at include/globals.h:106 |  |  | 0.580 |
| ns | 2527 |  | 257 | globals.h — PlayerData flag bits and the overloaded flagval fields | 3.3 | 3.2 | 0.549 |
| walker |  | 2685 | 378 | c decl names surface in include/globals.h |  |  | 0.554 |
| walker |  | 2685 | 0 | c decl at include/globals.h:19 |  |  | 0.554 |
| walker |  | 2685 | 0 | c decl at include/globals.h:23 |  |  | 0.554 |
| walker |  | 2685 | 0 | c decl at include/globals.h:26 |  |  | 0.554 |
| walker |  | 2685 | 0 | c decl at include/globals.h:29 |  |  | 0.554 |
| walker |  | 2685 | 0 | c decl at include/globals.h:32 |  |  | 0.554 |
| walker |  | 2685 | 0 | c decl at include/globals.h:35 |  |  | 0.554 |
| walker |  | 2685 | 0 | c decl at include/globals.h:38 |  |  | 0.554 |
| walker |  | 2685 | 0 | c decl at include/globals.h:41 |  |  | 0.554 |
| walker |  | 2685 | 0 | c decl at include/globals.h:45 |  |  | 0.554 |
| walker |  | 2685 | 0 | c decl at include/globals.h:49 |  |  | 0.554 |
| walker |  | 2685 | 0 | c decl at include/globals.h:53 |  |  | 0.554 |
| walker |  | 2685 | 0 | c decl at include/globals.h:56 |  |  | 0.554 |
| walker |  | 2685 | 0 | c decl at include/globals.h:59 |  |  | 0.554 |
| walker |  | 2685 | 0 | c decl at include/globals.h:63 |  |  | 0.554 |
| walker |  | 2685 | 0 | c decl at include/globals.h:66 |  |  | 0.554 |
| walker |  | 2685 | 0 | c decl at include/globals.h:71 |  |  | 0.554 |
| walker |  | 2685 | 0 | c decl at include/globals.h:75 |  |  | 0.554 |
| walker |  | 2685 | 0 | c decl at include/globals.h:93 |  |  | 0.554 |
| walker |  | 2696 | 11 | c decl doc at include/globals.h:26 |  |  | 0.554 |
| walker |  | 2707 | 11 | c decl doc at include/globals.h:29 |  |  | 0.554 |
| walker |  | 2721 | 14 | c decl doc at include/globals.h:19 |  |  | 0.554 |
| ns | 2757 |  | 230 | globals.h — MobData and EntityData | 3.4 |  | 0.528 |
| walker |  | 2759 | 38 | c decl doc at include/globals.h:23 |  |  | 0.528 |
| walker |  | 2767 | 8 | c decl doc at include/globals.h:59 |  |  | 0.528 |
| walker |  | 2789 | 22 | c includes in include/globals.h |  |  | 0.528 |
| walker |  | 2799 | 10 | c decl doc at include/globals.h:66 |  |  | 0.528 |
| walker |  | 2940 | 141 | c decl at include/globals.h:240 |  |  | 0.594 |
| ns | 2995 |  | 238 | globals.h — all extern global state declarations | 3.5 |  | 0.622 |
| ns | 3345 |  | 350 | build_registries.js — the template that generates include/registries.h | 3.6 |  | 0.588 |
| walker |  | 3418 | 478 | c decl names surface in include/tools.h |  |  | 0.592 |
| walker |  | 3418 | 0 | c decl at include/tools.h:8 |  |  | 0.592 |
| walker |  | 3418 | 0 | c decl at include/tools.h:11 |  |  | 0.592 |
| walker |  | 3435 | 17 | c decl body at include/tools.h:8 |  |  | 0.592 |
| walker |  | 3460 | 25 | c includes in include/tools.h |  |  | 0.592 |
| walker |  | 3485 | 25 | c decl body at include/tools.h:11 |  |  | 0.593 |
| walker |  | 3537 | 52 | c decl names surface in src/tools.c |  |  | 0.593 |
| walker |  | 3537 | 0 | c decl at src/tools.c:42 |  |  | 0.593 |
| walker |  | 3537 | 0 | c decl at src/tools.c:44 |  |  | 0.593 |
| ns | 3705 |  | 360 | globals.h — configuration macro roster, part 1 (world and player sizing) | 3.7 |  | 0.614 |
| walker |  | 3953 | 416 | c decl names surface in include/packets.h |  |  | 0.680 |
| walker |  | 3953 | 0 | c decl at include/packets.h:5 |  |  | 0.680 |
| walker |  | 3953 | 0 | c decl at include/packets.h:28 |  |  | 0.680 |
| walker |  | 3960 | 7 | c decl doc at include/packets.h:28 |  |  | 0.681 |
| walker |  | 3969 | 9 | c decl doc at include/packets.h:5 |  |  | 0.685 |
| ns | 3979 |  | 274 | globals.h — configuration macro roster, part 2 (feature toggles, including the commented-out ones) | 3.8 |  | 0.660 |
| walker |  | 4024 | 55 | c decl names surface in src/structures.c |  |  | 0.660 |
| walker |  | 4024 | 0 | c decl at src/structures.c:9 |  |  | 0.660 |
| walker |  | 4024 | 0 | c decl at src/structures.c:16 |  |  | 0.660 |
| walker |  | 4082 | 58 | c decl names surface in src/varnum.c |  |  | 0.660 |
| walker |  | 4082 | 0 | c decl at src/varnum.c:14 |  |  | 0.660 |
| walker |  | 4082 | 0 | c decl at src/varnum.c:34 |  |  | 0.660 |
| walker |  | 4082 | 0 | c decl at src/varnum.c:43 |  |  | 0.660 |
| ns | 4177 |  | 198 | src/globals.c — definitions of the global state, MOTD and brand | 3.9 |  | 0.641 |
| ns | 4261 |  | 84 | README Configuration section — where the knobs live | 3.10 |  | 0.638 |
| ns | 4403 |  | 142 | README Configuration — the maintainer's tuning guidance | 3.11 |  | 0.635 |
| ns | 4489 |  | 86 | build_registries.js — the complete biome list | 3.12 |  | 0.626 |
| walker |  | 4550 | 468 | c decl names surface in include/procedures.h |  |  | 0.630 |
| walker |  | 4575 | 25 | c includes in include/procedures.h |  |  | 0.630 |
| walker |  | 4642 | 67 | c decl names surface in src/crafting.c |  |  | 0.630 |
| walker |  | 4642 | 0 | c decl at src/crafting.c:9 |  |  | 0.630 |
| walker |  | 4642 | 0 | c decl at src/crafting.c:352 |  |  | 0.630 |
| walker |  | 4655 | 13 | c decl doc at include/globals.h:41 |  |  | 0.630 |
| ns | 4722 |  | 233 | procedures.h — client state and player lifecycle API | 4.1 |  | 0.644 |
| walker |  | 4774 | 119 | README.md section #1 |  |  | 0.644 |
| walker |  | 4802 | 28 | c decl at src/crafting.c:349 |  |  | 0.644 |
| ns | 4986 |  | 264 | procedures.h — metadata broadcast, slot mapping and block predicates | 4.2 |  | 0.650 |
| walker |  | 5160 | 358 | c decl names surface #1 in include/procedures.h |  |  | 0.660 |
| walker |  | 5185 | 25 | README.md section #6 |  |  | 0.660 |
| ns | 5305 |  | 319 | procedures.h — mining, actions, fluids, mobs, tick and entity-data API | 4.3 |  | 0.673 |
| ns | 5416 |  | 111 | worldgen.h — ChunkAnchor and ChunkFeature | 4.4 |  | 0.680 |
| walker |  | 5428 | 243 | c aggregate member group at include/globals.h:200 group 201 |  |  | 0.726 |
| ns | 5588 |  | 172 | worldgen.h — the complete generation API and the shared chunk_section buffer | 4.5 |  | 0.730 |
| walker |  | 5675 | 247 | c aggregate member group at include/globals.h:200 group 223 |  |  | 0.759 |
| walker |  | 5689 | 14 | c decl doc at include/globals.h:35 |  |  | 0.759 |
| walker |  | 5703 | 14 | c decl doc at src/structures.c:16 |  |  | 0.759 |
| ns | 5795 |  | 207 | tools.h — socket I/O and the byte-order writers | 4.6 |  | 0.763 |
| ns | 5992 |  | 197 | tools.h — the readers, string helpers and RNG | 4.7 |  | 0.768 |
| ns | 6130 |  | 138 | tools.h — the inline math helpers and the platform time shim | 4.8 |  | 0.763 |
| ns | 6326 |  | 196 | serialize.h in full — persistence API and its compile-time no-op fallback | 4.9 |  | 0.767 |
| walker |  | 6393 | 690 | plaintext config extract_registries.sh |  |  | 0.767 |
| ns | 6411 |  | 85 | crafting.h and structures.h in full — the two smallest module APIs | 4.10 |  | 0.768 |
| walker |  | 6433 | 40 | README.md section #7 |  |  | 0.768 |
| ns | 6500 |  | 89 | main.c — the project's own module include list | 5.1 |  | 0.760 |
| walker |  | 6602 | 169 | README.md section #2 |  |  | 0.760 |
| ns | 6756 |  | 256 | main.c — the maintainer's design note on the packet handlers, and handlePacket's signature | 5.2 |  | 0.746 |
| walker |  | 6961 | 359 | README.md section #3 |  |  | 0.746 |
| walker |  | 6978 | 17 | c decl doc at include/globals.h:38 |  |  | 0.746 |
| walker |  | 7019 | 41 | README.md section #8 |  |  | 0.746 |
| ns | 7082 |  | 326 | main.c — packet 0x00 dispatch: handshake, status, login, configuration | 5.3 |  | 0.724 |
| ns | 7351 |  | 269 | main.c — dispatch table, packet ids 0x07 through 0x19 | 5.4 |  | 0.710 |
| ns | 7693 |  | 342 | main.c — dispatch table, the movement case group and ids 0x28 through the default | 5.5 |  | 0.691 |
| walker |  | 7771 | 752 | c decl names surface #1 in include/packets.h |  |  | 0.751 |
| walker |  | 7816 | 45 | README.md section #9 |  |  | 0.751 |
| walker |  | 7863 | 47 | README.md section #10 |  |  | 0.751 |
| ns | 8049 |  | 356 | main.c — the single-threaded accept and tick round-robin loop | 5.6 |  | 0.729 |
| walker |  | 8192 | 329 | README.md section #4 |  |  | 0.736 |
| ns | 8330 |  | 281 | main.c — the ESP32 entry points: FreeRTOS task, WiFi event handler, app_main | 5.7 |  | 0.719 |
| walker |  | 8420 | 228 | c decl names surface in src/globals.c |  |  | 0.730 |
| walker |  | 8439 | 19 | c decl doc at include/globals.h:56 |  |  | 0.730 |
| walker |  | 8465 | 26 | imports in build_registries.js |  |  | 0.730 |
| walker |  | 8526 | 61 | c includes in src/structures.c |  |  | 0.730 |
| ns | 8554 |  | 224 | procedures.c — definition locations, part 1 (state, players, slots, block changes) | 6.1 |  | 0.717 |
| walker |  | 8591 | 65 | c includes in src/crafting.c |  |  | 0.717 |
| ns | 8771 |  | 217 | procedures.c — definition locations, part 2 (mining, predicates, armour, eating, fluids) | 6.2 |  | 0.707 |
| ns | 8903 |  | 132 | procedures.c — definition locations, part 3 (actions, mobs, tick, entity data) | 6.3 |  | 0.700 |
| walker |  | 9000 | 409 | c decl names surface in src/worldgen.c |  |  | 0.701 |
| walker |  | 9000 | 0 | c decl at src/worldgen.c:13 |  |  | 0.701 |
| walker |  | 9000 | 0 | c decl at src/worldgen.c:24 |  |  | 0.701 |
| walker |  | 9000 | 0 | c decl at src/worldgen.c:51 |  |  | 0.701 |
| walker |  | 9000 | 0 | c decl at src/worldgen.c:117 |  |  | 0.701 |
| walker |  | 9000 | 0 | c decl at src/worldgen.c:126 |  |  | 0.701 |
| walker |  | 9000 | 0 | c decl at src/worldgen.c:142 |  |  | 0.701 |
| walker |  | 9000 | 0 | c decl at src/worldgen.c:160 |  |  | 0.701 |
| walker |  | 9000 | 0 | c decl at src/worldgen.c:173 |  |  | 0.701 |
| walker |  | 9000 | 0 | c decl at src/worldgen.c:323 |  |  | 0.701 |
| walker |  | 9000 | 0 | c decl at src/worldgen.c:358 |  |  | 0.701 |
| walker |  | 9000 | 0 | c decl at src/worldgen.c:374 |  |  | 0.701 |
| walker |  | 9000 | 0 | c decl at src/worldgen.c:401 |  |  | 0.701 |
| walker |  | 9027 | 27 | c decl doc at src/worldgen.c:160 |  |  | 0.701 |
| ns | 9118 |  | 215 | worldgen.c — every definition, including the five private generator stages | 6.4 |  | 0.706 |
| walker |  | 9302 | 275 | README.md section #5 |  |  | 0.706 |
| ns | 9305 |  | 187 | serialize.c — the world file path and all five persistence entry points | 6.5 |  | 0.698 |
| walker |  | 9340 | 38 | c decl doc at src/worldgen.c:401 |  |  | 0.698 |
| walker |  | 9367 | 27 | c decl doc at include/globals.h:32 |  |  | 0.698 |
| walker |  | 9401 | 34 | c decl doc at src/tools.c:42 |  |  | 0.698 |
| ns | 9486 |  | 181 | packets.c — the chat command surface (!msg and !help) | 6.6 |  | 0.691 |
| walker |  | 9507 | 106 | c includes in src/worldgen.c |  |  | 0.691 |
| walker |  | 9566 | 59 | c decl body at src/structures.c:9 |  |  | 0.691 |
| walker |  | 9675 | 109 | c includes in src/globals.c |  |  | 0.691 |
| walker |  | 9736 | 61 | c decl body at src/varnum.c:34 |  |  | 0.691 |
| ns | 9745 |  | 259 | crafting.c — the registerSmeltingRecipe macro and the complete recipe table | 6.7 |  | 0.684 |
| walker |  | 9763 | 27 | c decl doc at include/globals.h:45 |  |  | 0.684 |
| ns | 9773 |  | 28 | Complete .github listings (workflow and issue templates) | 7.1 |  | 0.686 |
| walker |  | 9815 | 52 | c decl doc at include/globals.h:103 |  |  | 0.686 |
| ns | 9879 |  | 106 | README Contribution — the maintainer's rules for changes | 7.2 |  | 0.687 |
| walker |  | 9933 | 118 | c includes in src/varnum.c |  |  | 0.685 |
| ns | 9933 |  | 54 | extract_registries.sh — the top-level registry extraction sequence | 7.3 |  | 0.685 |
| ns | 9961 |  | 28 | LICENSE — the license identity line | 7.4 |  | 0.683 |
| walker |  | 9985 | 52 | c decl doc at src/worldgen.c:126 |  |  | 0.683 |
