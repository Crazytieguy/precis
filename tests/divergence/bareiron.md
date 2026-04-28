scores: Sim=0.535 Reached=22/42 Early=3 Late=12 Partial=5 Missing=15 Used=8992/10000

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 4 ranking-recoverable (w×gap=0.07), 16 wrong-slice/granularity (w×gap=1.98), 0 no-discovered (w×gap=0.00)
Secondary intervention: free final budget for 1 too-expensive candidate
Loss reasons: 2 predecessor-gated, 1 too-expensive, 1 discovered-unscheduled
Top rows: 3.7, 3.4, 3.2, 3.5, 3.6, ...
Note: likely lever is heuristic; verify `Sim` moves, not just bucket counts.

## Top opportunities

_`w(t)×gap` is a non-additive priority score: Σ exp(-exp_t/τ) × (1 - credit) per row, τ=2000. Same time weighting and credit gap as Sim, but rows can overlap between opportunities — sums across rows are an upper bound on Sim impact, not an additive estimate._

| intervention | rows | w(t)×gap | bands ≤3k/≤6k/total | evidence | top row ids |
|:-------------|-----:|---------:|:----------------------|:---------|:------------|
| split wrong-slice walker batches | 16 | 1.98 | 6/9/16 | nearby candidates have low exact atom overlap | 3.7, 3.4, 3.2, 3.5, 3.6, ... |
| tune ranking for discovered unscheduled candidates | 1 | 0.02 | 0/0/1 | high-overlap candidates fit but did not win, exact total=25/31 | 4.13 |
| promote c decl signature batches | 2 | 0.02 | 0/0/2 | 1 file, exact total=28/33 | 5.2, 5.5 |
| free final budget / demote late waste | 1 | 0.02 | 0/0/1 | high-overlap candidates exceed final remaining budget, exact total=24/24 | 4.15 |

Tiers: 1=4/4 reached, 0 partial, 0 missing, avg=1.00; 2=2/2 reached, 0 partial, 0 missing, avg=1.00; 3=8/14 reached, 4 partial, 2 missing, avg=0.79; 4=8/15 reached, 1 partial, 6 missing, avg=0.59; 5=0/7 reached, 0 partial, 7 missing, avg=0.08

## Diagnosis rollup

| diagnosis | rows | missing | partial | timing | likely lever |
|:----------|-----:|--------:|--------:|-------:|:-------------|
| ranking-recoverable | 4 | 4 | 0 | 0 | value/ranking |
| wrong-slice / granularity | 16 | 11 | 5 | 0 | walker granularity / wrong slice |
| timing-only | 16 | 0 | 0 | 16 | usually no code change |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | w(t)×gap | likely lever |
|:------------|-----:|---------:|:-------------|
| predecessor not scheduled | 2 | 0.02 | promote predecessor |
| too expensive at final margin | 1 | 0.02 | free final budget |
| discovered unscheduled | 1 | 0.02 | tune ranking |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=29, unscheduled bbox=5, scheduled same-file=2

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | aligned | high | 1 |
| scheduled bbox | early | low | 2 |
| scheduled bbox | early | high | 1 |
| scheduled bbox | late | low | 1 |
| scheduled bbox | late | high | 5 |
| scheduled bbox | late | full | 6 |
| scheduled bbox | missing | low | 8 |
| scheduled bbox | partial | low | 5 |
| unscheduled bbox | missing | low | 2 |
| unscheduled bbox | missing | high | 2 |
| unscheduled bbox | missing | full | 1 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 4.13 | 7274 | — | — | 0.06 | missing | main.c — handlePacket signature + author's design comment | [scheduled bbox exact=2/31] c decl at src/main.c:68 (t=903, 2 atoms); better unscheduled exact=25/31: c decl doc at src/main.c:68 (25 atoms, discovered unscheduled) |
| 4.15 | 8012 | — | — | 0.00 | missing | main.c — handlePacket switch case index (truncated) | [unscheduled bbox exact=24/24] c decl body at src/main.c:68 (333 atoms, too expensive at final margin) |
| 5.2 | 8443 | — | — | 0.00 | missing | getBlockChange body — linear scan with chest skip | [unscheduled bbox exact=13/15] c decl body at src/procedures.c:478 (13 atoms, predecessor not scheduled: c decl at src/procedures.c:478) |
| 5.5 | 9430 | — | — | 0.00 | missing | handleServerTick — world_time + per-player tick header | [unscheduled bbox exact=15/18] c decl body at src/procedures.c:1623 (15 atoms, predecessor not scheduled: c decl at src/procedures.c:1623) |

### wrong-slice / granularity

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 3.2 | 384 | — | — | 0.67 | partial | Tools — mod_abs / div_floor inline helpers | [scheduled bbox exact=4/6] c decl names surface in include/tools.h (t=1393, 4 atoms) |
| 3.4 | 653 | — | — | 0.40 | missing | structures.h + crafting.h — full (tiny) | [scheduled bbox exact=3/15] c decl names surface in include/crafting.h (t=283, 3 atoms) |
| 3.5 | 803 | — | — | 0.64 | partial | varnum.h — full | [scheduled bbox exact=8/14] c decl names surface in include/varnum.h (t=418, 8 atoms) |
| 3.6 | 899 | — | — | 0.75 | partial | README — Configuration intro paragraph | [scheduled bbox exact=3/4] README.md section #3 (t=7832, 3 atoms) |
| 3.7 | 1143 | — | — | 0.05 | missing | serialize.h — full (disk-sync API + #ifdef shape) | [scheduled bbox exact=1/21] c includes in include/serialize.h (t=303, 1 atoms) |
| 3.13 | 2443 | — | — | 0.77 | partial | worldgen.h — full | [scheduled bbox exact=14/30] c decl names surface in include/worldgen.h (t=667, 14 atoms) |
| 4.5 | 3794 | — | — | 0.72 | partial | globals.h — runtime extern declarations | [scheduled bbox exact=13/18] c decl names surface in include/globals.h (t=2269, 13 atoms) |
| 4.7 | 4813 | — | — | 0.00 | missing | globals.h — feature toggle #defines + their comments | [scheduled same-file] c decl names surface in include/globals.h (t=2269, 82 atoms) |
| 4.9 | 5889 | — | — | 0.43 | missing | globals.h — disk-sync + network buffer knobs (with SYNC_WORLD_TO_DISK gate) | [scheduled bbox exact=6/35] c decl names surface in include/globals.h (t=2269, 6 atoms) |
| 4.10 | 6036 | — | — | 0.25 | missing | globals.h — preamble + ESP_PLATFORM gating | [scheduled bbox exact=2/16] c decl names surface in include/globals.h (t=2269, 2 atoms) |
| 4.14 | 7762 | — | — | 0.00 | missing | main() — startup signature + seed/state initialization | [scheduled same-file] c includes in src/main.c (t=8083, 13 atoms) |
| 5.1 | 8244 | — | — | 0.11 | missing | getBlockAt body — block_changes overlay + terrain fallback | [scheduled bbox exact=2/19] c decl at src/worldgen.c:374 (t=6930, 2 atoms); better unscheduled exact=12/19: c decl body at src/worldgen.c:374 (12 atoms, discovered unscheduled) |
| 5.3 | 8853 | — | — | 0.08 | missing | getChunkBiome body — biome from world_seed bit pattern | [scheduled bbox exact=2/26] c decl at src/worldgen.c:24 (t=6930, 2 atoms); better unscheduled exact=19/26: c decl body at src/worldgen.c:24 (19 atoms, discovered unscheduled) |
| 5.4 | 9148 | — | — | 0.00 | missing | handlePlayerJoin body — chat + spawn entity broadcast | [unscheduled bbox exact=15/21] c decl body at src/procedures.c:177 (15 atoms, predecessor not scheduled: c decl at src/procedures.c:177) |
| 5.6 | 9589 | — | — | 0.00 | missing | sc_blockUpdate body — concrete sc_ packet wire format example | [unscheduled bbox exact=5/8] c decl body at src/packets.c:471 (5 atoms, predecessor not scheduled: c decl at src/packets.c:471) |
| 5.7 | 9899 | — | — | 0.35 | missing | placeTreeStructure intro — replaceable-helper + trunk | [scheduled bbox exact=4/20] c decl names surface in src/structures.c (t=1484, 4 atoms); better unscheduled exact=10/20: c decl body at src/structures.c:16 (10 atoms, discovered unscheduled) |

### timing-only

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.3 | 79 | 878 | +799 | 1.00 | late | README — Minecraft + protocol versions | [scheduled bbox exact=2/2] README.md section #0 (t=878, 2 atoms) |
| 1.4 | 146 | 878 | +732 | 1.00 | late | README — project priorities | [scheduled bbox exact=1/1] README.md section #0 (t=878, 1 atoms) |
| 3.1 | 298 | 2269 | +1971 | 1.00 | late | Connection state-machine constants (STATE_*) | [scheduled bbox exact=6/6] c decl names surface in include/globals.h (t=2269, 6 atoms) |
| 3.3 | 500 | 2773 | +2273 | 1.00 | late | Core runtime config: PORT / MAX_PLAYERS / MAX_MOBS | [scheduled bbox exact=5/9] c decl names surface in include/globals.h (t=2269, 5 atoms) |
| 3.8 | 1350 | 5221 | +3871 | 1.00 | late | packets.h — serverbound (cs_) function names | [scheduled bbox exact=21/22] c decl names surface in include/packets.h (t=5221, 21 atoms) |
| 3.9 | 1545 | 5221 | +3676 | 1.00 | late | packets.h — clientbound (sc_) function names, part 1 | [scheduled bbox exact=19/20] c decl names surface in include/packets.h (t=5221, 19 atoms) |
| 3.10 | 1712 | 5221 | +3509 | 1.00 | late | packets.h — clientbound (sc_) function names, part 2 | [scheduled bbox exact=17/17] c decl names surface in include/packets.h (t=5221, 17 atoms) |
| 3.11 | 1987 | 4055 | +2068 | 1.00 | late | procedures.h — game-logic function names, part 1 | [scheduled bbox exact=26/26] c decl names surface in include/procedures.h (t=4055, 32 atoms) |
| 3.12 | 2107 | 4055 | +1948 | 1.00 | late | procedures.h — game-logic function names, part 2 | [scheduled bbox exact=13/13] c decl names surface in include/procedures.h (t=4055, 18 atoms) |
| 3.14 | 2733 | 1393 | -1340 | 0.82 | early | tools.h — I/O + RNG function names (rest) | [scheduled bbox exact=23/28] c decl names surface in include/tools.h (t=1393, 28 atoms) |
| 4.2 | 3378 | 6128 | +2750 | 0.95 | late | PlayerData struct — full | [scheduled bbox exact=39/41] c decl at include/globals.h:200 (t=6128, 39 atoms) |
| 4.4 | 3629 | 2518 | -1111 | 0.92 | early | EntityData / EntityDataValue (entity metadata serialization) | [scheduled bbox exact=7/12] c decl at include/globals.h:260 (t=2518, 7 atoms) |
| 4.6 | 4079 | 5579 | +1500 | 0.81 | late | src/globals.c — runtime defaults + literal MOTD/brand | [scheduled bbox exact=21/26] c decl names surface in src/globals.c (t=5579, 21 atoms) |
| 4.8 | 5344 | 2659 | -2685 | 1.00 | early | globals.h — tickrate / RNG seeds / world-gen knobs | [scheduled bbox exact=22/40] c decl names surface in include/globals.h (t=2269, 22 atoms) |
| 4.11 | 6273 | 7832 | +1559 | 0.80 | aligned | README — Configuration 'important options' bullets | [scheduled bbox exact=4/5] README.md section #3 (t=7832, 4 atoms) |
| 4.12 | 6815 | 8902 | +2087 | 0.90 | late | README — Compilation section | [scheduled bbox exact=9/10] README.md section #2 (t=8902, 9 atoms) |

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
| 669 | — | — | — | +11 more rows |
