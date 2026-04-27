scores: Sim=0.474 Reached=22/42 Early=3 Late=13 Partial=5 Missing=15 Used=8992/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 4 | 4 | 0 | 0 | 1.00 |
| 2 | 2 | 2 | 0 | 0 | 1.00 |
| 3 | 14 | 8 | 4 | 2 | 0.79 |
| 4 | 15 | 8 | 1 | 6 | 0.59 |
| 5 | 7 | 0 | 0 | 7 | 0.08 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.3 | 79 | 830 | +751 | 1.00 | late | README — Minecraft + protocol versions | README.md section #0 (t=830, 2 atoms) |
| 1.4 | 146 | 830 | +684 | 1.00 | late | README — project priorities | README.md section #0 (t=830, 1 atoms) |
| 2.1 | 194 | 878 | +684 | 1.00 | late | src/ listing |  |
| 3.1 | 298 | 2269 | +1971 | 1.00 | late | Connection state-machine constants (STATE_*) | c decl names surface in include/globals.h (t=2269, 6 atoms) |
| 3.2 | 384 | — | — | 0.67 | partial | Tools — mod_abs / div_floor inline helpers | c decl names surface in include/tools.h (t=1393, 4 atoms) |
| 3.3 | 500 | 2773 | +2273 | 1.00 | late | Core runtime config: PORT / MAX_PLAYERS / MAX_MOBS | c decl names surface in include/globals.h (t=2269, 5 atoms) |
| 3.4 | 653 | — | — | 0.40 | missing | structures.h + crafting.h — full (tiny) | c decl names surface in include/crafting.h (t=235, 3 atoms) |
| 3.5 | 803 | — | — | 0.64 | partial | varnum.h — full | c decl names surface in include/varnum.h (t=370, 8 atoms) |
| 3.6 | 899 | — | — | 0.75 | partial | README — Configuration intro paragraph | README.md section #3 (t=7832, 3 atoms) |
| 3.7 | 1143 | — | — | 0.05 | missing | serialize.h — full (disk-sync API + #ifdef shape) | c includes in include/serialize.h (t=255, 1 atoms) |
| 3.8 | 1350 | 5221 | +3871 | 1.00 | late | packets.h — serverbound (cs_) function names | c decl names surface in include/packets.h (t=5221, 21 atoms) |
| 3.9 | 1545 | 5221 | +3676 | 1.00 | late | packets.h — clientbound (sc_) function names, part 1 | c decl names surface in include/packets.h (t=5221, 19 atoms) |
| 3.10 | 1712 | 5221 | +3509 | 1.00 | late | packets.h — clientbound (sc_) function names, part 2 | c decl names surface in include/packets.h (t=5221, 17 atoms) |
| 3.11 | 1987 | 4055 | +2068 | 1.00 | late | procedures.h — game-logic function names, part 1 | c decl names surface in include/procedures.h (t=4055, 32 atoms) |
| 3.12 | 2107 | 4055 | +1948 | 1.00 | late | procedures.h — game-logic function names, part 2 | c decl names surface in include/procedures.h (t=4055, 18 atoms) |
| 3.13 | 2443 | — | — | 0.77 | partial | worldgen.h — full | c decl names surface in include/worldgen.h (t=619, 14 atoms) |
| 3.14 | 2733 | 1393 | -1340 | 0.82 | early | tools.h — I/O + RNG function names (rest) | c decl names surface in include/tools.h (t=1393, 28 atoms) |
| 4.2 | 3378 | 6128 | +2750 | 0.95 | late | PlayerData struct — full | c decl at include/globals.h:200 (t=6128, 39 atoms) |
| 4.4 | 3629 | 2518 | -1111 | 0.92 | early | EntityData / EntityDataValue (entity metadata serialization) | c decl at include/globals.h:260 (t=2518, 7 atoms) |
| 4.5 | 3794 | — | — | 0.72 | partial | globals.h — runtime extern declarations | c decl names surface in include/globals.h (t=2269, 13 atoms) |
| 4.6 | 4079 | 5579 | +1500 | 0.81 | late | src/globals.c — runtime defaults + literal MOTD/brand | c decl names surface in src/globals.c (t=5579, 21 atoms) |
| 4.7 | 4813 | — | — | 0.00 | missing | globals.h — feature toggle #defines + their comments |  |
| 4.8 | 5344 | 2659 | -2685 | 1.00 | early | globals.h — tickrate / RNG seeds / world-gen knobs | c decl names surface in include/globals.h (t=2269, 22 atoms) |
| 4.9 | 5889 | — | — | 0.43 | missing | globals.h — disk-sync + network buffer knobs (with SYNC_WORLD_TO_DISK gate) | c decl names surface in include/globals.h (t=2269, 6 atoms) |
| 4.10 | 6036 | — | — | 0.25 | missing | globals.h — preamble + ESP_PLATFORM gating | c includes in include/globals.h (t=390, 2 atoms) |
| 4.11 | 6273 | 7832 | +1559 | 0.80 | aligned | README — Configuration 'important options' bullets | README.md section #3 (t=7832, 4 atoms) |
| 4.12 | 6815 | 8902 | +2087 | 0.90 | late | README — Compilation section | README.md section #2 (t=8902, 9 atoms) |
| 4.13 | 7274 | — | — | 0.06 | missing | main.c — handlePacket signature + author's design comment | c decl names surface in src/main.c (t=903, 2 atoms) |
| 4.14 | 7762 | — | — | 0.00 | missing | main() — startup signature + seed/state initialization |  |
| 4.15 | 8012 | — | — | 0.00 | missing | main.c — handlePacket switch case index (truncated) |  |
| 5.1 | 8244 | — | — | 0.11 | missing | getBlockAt body — block_changes overlay + terrain fallback | c decl names surface in src/worldgen.c (t=6930, 2 atoms) |
| 5.2 | 8443 | — | — | 0.00 | missing | getBlockChange body — linear scan with chest skip |  |
| 5.3 | 8853 | — | — | 0.08 | missing | getChunkBiome body — biome from world_seed bit pattern | c decl names surface in src/worldgen.c (t=6930, 2 atoms) |
| 5.4 | 9148 | — | — | 0.00 | missing | handlePlayerJoin body — chat + spawn entity broadcast |  |
| 5.5 | 9430 | — | — | 0.00 | missing | handleServerTick — world_time + per-player tick header |  |
| 5.6 | 9589 | — | — | 0.00 | missing | sc_blockUpdate body — concrete sc_ packet wire format example |  |
| 5.7 | 9899 | — | — | 0.35 | missing | placeTreeStructure intro — replaceable-helper + trunk | c decl names surface in src/structures.c (t=1484, 4 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 3 | 572 | README.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 364 | 0.89 | 407 | 6930 | c decl names surface in src/worldgen.c |
| 260 | 1.00 | 260 | 7412 | README.md section #4 |
| 198 | 1.00 | 198 | 6523 | README.md section #5 |
| 135 | 1.00 | 135 | 8083 | c includes in src/main.c |
| 135 | 1.00 | 135 | 8218 | c includes in src/procedures.c |
| 114 | 1.00 | 114 | 3239 | README.md section #1 |
| 106 | 1.00 | 106 | 7518 | c includes in src/packets.c |
| 103 | 1.00 | 103 | 7152 | c includes in src/worldgen.c |
| 90 | 1.00 | 90 | 8992 | c decl body at src/varnum.c:43 |
| 78 | 1.00 | 78 | 8374 | c decl body at src/worldgen.c:117 |
| 78 | 1.00 | 78 | 8296 | c decl body at src/worldgen.c:13 |
| 72 | 1.00 | 72 | 6325 | c includes in src/tools.c |
| 65 | 1.00 | 65 | 1619 | c decl names surface in src/crafting.c |
| 63 | 1.00 | 63 | 6253 | c includes in src/structures.c |
| 62 | 1.00 | 62 | 6190 | c includes in src/crafting.c |
| 59 | 1.00 | 59 | 7948 | c decl body at src/varnum.c:34 |
| 58 | 1.00 | 58 | 5371 | plaintext config .gitignore |
| 56 | 1.00 | 56 | 1540 | c decl names surface in src/varnum.c |
| 53 | 0.08 | 650 | 2269 | c decl names surface in include/globals.h |
| 52 | 1.00 | 52 | 7049 | c decl doc at src/worldgen.c:126 |
| 51 | 1.00 | 51 | 5630 | c includes in src/varnum.c |
