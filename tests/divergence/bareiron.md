scores: Score(3000)=0.588 ns_rows≤3K=21/42 (reached=10 partial=3 missing=8)

## Per-budget scores

| B | A_B | I(B) | C(B) | Score(B) | walker_used |
|--:|----:|-----:|-----:|---------:|------------:|
| 1000 | 88 | 0.865 | 0.520 | 0.671 | 960 |
| 1442 | 131 | 0.832 | 0.380 | 0.562 | 1431 |
| 2080 | 194 | 0.777 | 0.256 | 0.446 | 1619 |
| 3000 | 278 | 0.819 | 0.422 | 0.588 | 2881 |
| 4327 | 387 | 0.816 | 0.489 | 0.632 | 4078 |
| 6240 | 529 | 0.849 | 0.591 | 0.708 | 5803 |
| 9000 | 694 | 0.843 | 0.529 | 0.668 | 8926 |

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 4 ranking-recoverable (gap@3k=0.00), 16 wrong-slice/granularity (gap@3k=0.60), 0 no-discovered (gap@3k=0.00)
Secondary intervention: free T_max budget for 1 too-expensive candidate
Top rows: 3.7, 3.4, 3.5, 3.2, 3.6, ...

## Top opportunities

_`gap@B` is a non-additive priority score: `Σ over atoms with rank ≤ |A_B|: (1 − damped_credit(a)) / rank(a)`. `gap@3k` is the primary sort key — direct proxy for `Score(3000)` headroom. `gap@1k` and `gap@9k` show how the same intervention scales across the budget vector. Rows can overlap between opportunities; sums are upper bounds on Score(B) impact, not additive estimates._

| intervention | rows | gap@1k | gap@3k | gap@9k | evidence | top row ids |
|:-------------|-----:|-------:|-------:|-------:|:---------|:------------|
| split wrong-slice walker batches | 16 | 0.44 | 0.60 | 0.93 | nearby candidates have low exact atom overlap | 3.7, 3.4, 3.5, 3.2, 3.6, ... |
| finish partially-delivered NS batches | 5 | 0.17 | 0.17 | 0.28 | avg batch completion=0.44 | 3.4, 4.5, 4.9, 4.10, 5.7 |

## Diagnosis rollup

| diagnosis | rows | missing | partial | likely lever |
|:----------|-----:|--------:|--------:|:-------------|
| ranking-recoverable | 4 | 4 | 0 | value/ranking |
| wrong-slice / granularity | 16 | 12 | 4 | walker granularity / wrong slice |
| mixed/unknown | 10 | 10 | 0 | inspect row |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | gap@3k | likely lever |
|:------------|-----:|-------:|:-------------|
| predecessor not scheduled | 2 | 0.00 | promote predecessor |
| too expensive at final margin | 1 | 0.00 | free T_max budget |
| discovered unscheduled | 1 | 0.00 | tune ranking |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=23, unscheduled bbox=5, scheduled same-file=2

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | missing | low | 9 |
| scheduled bbox | missing | high | 6 |
| scheduled bbox | missing | full | 4 |
| scheduled bbox | partial | low | 4 |
| unscheduled bbox | missing | low | 2 |
| unscheduled bbox | missing | high | 2 |
| unscheduled bbox | missing | full | 1 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 4.13 | 7274 | 0.06 | 0.05 | missing | main.c — handlePacket signature + author's design comment | [scheduled bbox exact=2/31] c decl at src/main.c:68 (t=903, 2 atoms); better unscheduled exact=25/31: c decl doc at src/main.c:68 (25 atoms, discovered unscheduled) |
| 4.15 | 8012 | 0.00 | 0.00 | missing | main.c — handlePacket switch case index (truncated) | [unscheduled bbox exact=24/24] c decl body at src/main.c:68 (333 atoms, too expensive at final margin) |
| 5.2 | 8443 | 0.00 | 0.00 | missing | getBlockChange body — linear scan with chest skip | [unscheduled bbox exact=13/15] c decl body at src/procedures.c:478 (13 atoms, predecessor not scheduled: c decl at src/procedures.c:478) |
| 5.5 | 9430 | 0.00 | 0.00 | missing | handleServerTick — world_time + per-player tick header | [unscheduled bbox exact=15/18] c decl body at src/procedures.c:1623 (15 atoms, predecessor not scheduled: c decl at src/procedures.c:1623) |

### wrong-slice / granularity

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 3.2 | 384 | 0.67 | 0.99 | partial | Tools — mod_abs / div_floor inline helpers | [scheduled bbox exact=4/6] c decl names surface in include/tools.h (t=1393, 4 atoms) |
| 3.4 | 653 | 0.40 | 0.68 | missing | structures.h + crafting.h — full (tiny) | [scheduled bbox exact=3/15] c decl names surface in include/crafting.h (t=283, 3 atoms) |
| 3.5 | 803 | 0.64 | 0.84 | partial | varnum.h — full | [scheduled bbox exact=8/14] c decl names surface in include/varnum.h (t=418, 8 atoms) |
| 3.6 | 899 | 0.25 | 0.04 | missing | README — Configuration intro paragraph | [scheduled bbox exact=3/4] README.md section #3 (t=7856, 3 atoms) |
| 3.7 | 1143 | 0.05 | 0.04 | missing | serialize.h — full (disk-sync API + #ifdef shape) | [scheduled bbox exact=1/21] c includes in include/serialize.h (t=303, 1 atoms) |
| 3.13 | 2443 | 0.77 | 0.93 | partial | worldgen.h — full | [scheduled bbox exact=14/30] c decl names surface in include/worldgen.h (t=667, 14 atoms) |
| 4.5 | 3794 | 0.72 | 0.78 | partial | globals.h — runtime extern declarations | [scheduled bbox exact=13/18] c decl names surface in include/globals.h (t=2269, 13 atoms) |
| 4.7 | 4813 | 0.00 | 0.00 | missing | globals.h — feature toggle #defines + their comments | [scheduled same-file] c decl names surface in include/globals.h (t=2269, 82 atoms) |
| 4.9 | 5889 | 0.29 | 0.20 | missing | globals.h — disk-sync + network buffer knobs (with SYNC_WORLD_TO_DISK gate) | [scheduled bbox exact=6/35] c decl names surface in include/globals.h (t=2269, 6 atoms) |
| 4.10 | 6036 | 0.25 | 0.27 | missing | globals.h — preamble + ESP_PLATFORM gating | [scheduled bbox exact=2/16] c decl names surface in include/globals.h (t=2269, 2 atoms) |
| 4.14 | 7762 | 0.00 | 0.00 | missing | main() — startup signature + seed/state initialization | [scheduled same-file] c includes in src/main.c (t=8107, 13 atoms) |
| 5.1 | 8244 | 0.00 | 0.00 | missing | getBlockAt body — block_changes overlay + terrain fallback | [scheduled bbox exact=2/19] c decl at src/worldgen.c:374 (t=6954, 2 atoms); better unscheduled exact=12/19: c decl body at src/worldgen.c:374 (12 atoms, discovered unscheduled) |
| 5.3 | 8853 | 0.00 | 0.00 | missing | getChunkBiome body — biome from world_seed bit pattern | [scheduled bbox exact=2/26] c decl at src/worldgen.c:24 (t=6954, 2 atoms); better unscheduled exact=19/26: c decl body at src/worldgen.c:24 (19 atoms, discovered unscheduled) |
| 5.4 | 9148 | 0.00 | 0.00 | missing | handlePlayerJoin body — chat + spawn entity broadcast | [unscheduled bbox exact=15/21] c decl body at src/procedures.c:177 (15 atoms, predecessor not scheduled: c decl at src/procedures.c:177) |
| 5.6 | 9589 | 0.00 | 0.00 | missing | sc_blockUpdate body — concrete sc_ packet wire format example | [unscheduled bbox exact=5/8] c decl body at src/packets.c:471 (5 atoms, predecessor not scheduled: c decl at src/packets.c:471) |
| 5.7 | 9899 | 0.20 | 0.26 | missing | placeTreeStructure intro — replaceable-helper + trunk | [scheduled bbox exact=4/20] c decl names surface in src/structures.c (t=1484, 4 atoms); better unscheduled exact=10/20: c decl body at src/structures.c:16 (10 atoms, discovered unscheduled) |

### mixed/unknown

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 3.8 | 1350 | 0.00 | 0.00 | missing | packets.h — serverbound (cs_) function names | [scheduled bbox exact=21/22] c decl names surface in include/packets.h (t=5244, 21 atoms) |
| 3.9 | 1545 | 0.00 | 0.00 | missing | packets.h — clientbound (sc_) function names, part 1 | [scheduled bbox exact=19/20] c decl names surface in include/packets.h (t=5244, 19 atoms) |
| 3.10 | 1712 | 0.00 | 0.00 | missing | packets.h — clientbound (sc_) function names, part 2 | [scheduled bbox exact=17/17] c decl names surface in include/packets.h (t=5244, 17 atoms) |
| 3.11 | 1987 | 0.00 | 0.00 | missing | procedures.h — game-logic function names, part 1 | [scheduled bbox exact=26/26] c decl names surface in include/procedures.h (t=4078, 32 atoms) |
| 3.12 | 2107 | 0.00 | 0.00 | missing | procedures.h — game-logic function names, part 2 | [scheduled bbox exact=13/13] c decl names surface in include/procedures.h (t=4078, 18 atoms) |
| 4.2 | 3378 | 0.03 | 0.02 | missing | PlayerData struct — full | [scheduled bbox exact=39/41] c decl at include/globals.h:200 (t=6301, 39 atoms) |
| 4.3 | 3526 | 0.09 | 0.05 | missing | MobData struct — full (data-byte bitfield comment) | [scheduled bbox exact=12/12] c decl at include/globals.h:240 (t=3020, 12 atoms) |
| 4.6 | 4079 | 0.00 | 0.00 | missing | src/globals.c — runtime defaults + literal MOTD/brand | [scheduled bbox exact=21/26] c decl names surface in src/globals.c (t=5707, 21 atoms) |
| 4.11 | 6273 | 0.00 | 0.00 | missing | README — Configuration 'important options' bullets | [scheduled bbox exact=4/5] README.md section #3 (t=7856, 4 atoms) |
| 4.12 | 6815 | 0.10 | 0.01 | missing | README — Compilation section | [scheduled bbox exact=9/10] README.md section #2 (t=8926, 9 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 2 | 374 | README.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 364 | 0.89 | 407 | 6954 | c decl names surface in src/worldgen.c |
| 260 | 1.00 | 260 | 7436 | README.md section #4 |
| 135 | 1.00 | 135 | 8107 | c includes in src/main.c |
| 135 | 1.00 | 135 | 8242 | c includes in src/procedures.c |
| 114 | 1.00 | 114 | 3239 | README.md section #1 |
| 106 | 1.00 | 106 | 7542 | c includes in src/packets.c |
| 103 | 1.00 | 103 | 7176 | c includes in src/worldgen.c |
| 90 | 1.00 | 90 | 9016 | c decl body at src/varnum.c:43 |
| 78 | 1.00 | 78 | 8398 | c decl body at src/worldgen.c:117 |
| 78 | 1.00 | 78 | 8320 | c decl body at src/worldgen.c:13 |
| 591 | — | — | — | +10 more rows |
