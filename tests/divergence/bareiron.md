scores: Sim=0.356 Reached=8/42 Early=2 Late=3 Partial=1 Missing=33 Used=1816/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 4 | 4 | 0 | 0 | 1.00 |
| 2 | 2 | 2 | 0 | 0 | 1.00 |
| 3 | 14 | 0 | 1 | 13 | 0.05 |
| 4 | 15 | 2 | 0 | 13 | 0.11 |
| 5 | 7 | 0 | 0 | 7 | 0.00 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.3 | 79 | 296 | +217 | 1.00 | late | README — Minecraft + protocol versions | README.md section #0 (t=296, 2 atoms) |
| 1.4 | 146 | 296 | +150 | 1.00 | late | README — project priorities | README.md section #0 (t=296, 1 atoms) |
| 2.1 | 194 | 344 | +150 | 1.00 | late | src/ listing |  |
| 3.1 | 298 | — | — | 0.00 | missing | Connection state-machine constants (STATE_*) |  |
| 3.2 | 384 | — | — | 0.00 | missing | Tools — mod_abs / div_floor inline helpers |  |
| 3.3 | 500 | — | — | 0.00 | missing | Core runtime config: PORT / MAX_PLAYERS / MAX_MOBS |  |
| 3.4 | 653 | — | — | 0.00 | missing | structures.h + crafting.h — full (tiny) |  |
| 3.5 | 803 | — | — | 0.00 | missing | varnum.h — full |  |
| 3.6 | 899 | — | — | 0.75 | partial | README — Configuration intro paragraph | README.md section #3 (t=1288, 3 atoms) |
| 3.7 | 1143 | — | — | 0.00 | missing | serialize.h — full (disk-sync API + #ifdef shape) |  |
| 3.8 | 1350 | — | — | 0.00 | missing | packets.h — serverbound (cs_) function names |  |
| 3.9 | 1545 | — | — | 0.00 | missing | packets.h — clientbound (sc_) function names, part 1 |  |
| 3.10 | 1712 | — | — | 0.00 | missing | packets.h — clientbound (sc_) function names, part 2 |  |
| 3.11 | 1987 | — | — | 0.00 | missing | procedures.h — game-logic function names, part 1 |  |
| 3.12 | 2107 | — | — | 0.00 | missing | procedures.h — game-logic function names, part 2 |  |
| 3.13 | 2443 | — | — | 0.00 | missing | worldgen.h — full |  |
| 3.14 | 2733 | — | — | 0.00 | missing | tools.h — I/O + RNG function names (rest) |  |
| 4.1 | 2854 | — | — | 0.00 | missing | BlockChange struct + extern arrays (block_changes/player_data/mob_data) |  |
| 4.2 | 3378 | — | — | 0.00 | missing | PlayerData struct — full |  |
| 4.3 | 3526 | — | — | 0.00 | missing | MobData struct — full (data-byte bitfield comment) |  |
| 4.4 | 3629 | — | — | 0.00 | missing | EntityData / EntityDataValue (entity metadata serialization) |  |
| 4.5 | 3794 | — | — | 0.00 | missing | globals.h — runtime extern declarations |  |
| 4.6 | 4079 | — | — | 0.00 | missing | src/globals.c — runtime defaults + literal MOTD/brand |  |
| 4.7 | 4813 | — | — | 0.00 | missing | globals.h — feature toggle #defines + their comments |  |
| 4.8 | 5344 | — | — | 0.00 | missing | globals.h — tickrate / RNG seeds / world-gen knobs |  |
| 4.9 | 5889 | — | — | 0.00 | missing | globals.h — disk-sync + network buffer knobs (with SYNC_WORLD_TO_DISK gate) |  |
| 4.10 | 6036 | — | — | 0.00 | missing | globals.h — preamble + ESP_PLATFORM gating |  |
| 4.11 | 6273 | 1288 | -4985 | 0.80 | early | README — Configuration 'important options' bullets | README.md section #3 (t=1288, 4 atoms) |
| 4.12 | 6815 | 1816 | -4999 | 0.90 | early | README — Compilation section | README.md section #2 (t=1816, 9 atoms) |
| 4.13 | 7274 | — | — | 0.00 | missing | main.c — handlePacket signature + author's design comment |  |
| 4.14 | 7762 | — | — | 0.00 | missing | main() — startup signature + seed/state initialization |  |
| 4.15 | 8012 | — | — | 0.00 | missing | main.c — handlePacket switch case index (truncated) |  |
| 5.1 | 8244 | — | — | 0.00 | missing | getBlockAt body — block_changes overlay + terrain fallback |  |
| 5.2 | 8443 | — | — | 0.00 | missing | getBlockChange body — linear scan with chest skip |  |
| 5.3 | 8853 | — | — | 0.00 | missing | getChunkBiome body — biome from world_seed bit pattern |  |
| 5.4 | 9148 | — | — | 0.00 | missing | handlePlayerJoin body — chat + spawn entity broadcast |  |
| 5.5 | 9430 | — | — | 0.00 | missing | handleServerTick — world_time + per-player tick header |  |
| 5.6 | 9589 | — | — | 0.00 | missing | sc_blockUpdate body — concrete sc_ packet wire format example |  |
| 5.7 | 9899 | — | — | 0.00 | missing | placeTreeStructure intro — replaceable-helper + trunk |  |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 3 | 572 | README.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 260 | 1.00 | 260 | 974 | README.md section #4 |
| 198 | 1.00 | 198 | 714 | README.md section #5 |
| 114 | 1.00 | 114 | 458 | README.md section #1 |
| 58 | 1.00 | 58 | 516 | plaintext config .gitignore |
