scores: Sim=0.403 Reached=12/40 Early=4 Late=7 Partial=4 Missing=24 Used=9274/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 9 | 6 | 3 | 0 | 0.91 |
| 2 | 9 | 4 | 1 | 4 | 0.52 |
| 3 | 22 | 2 | 0 | 20 | 0.21 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor |
|----|------:|----------:|--------:|-------:|:-------|:-----------|
| 1.1 | 16 | 224 | +208 | 1.00 | late | Crate one-liner |
| 1.2 | 50 | 34 | -16 | 1.00 | early | Repo root listing |
| 1.3 | 99 | 281 | +182 | 1.00 | late | src/ listing |
| 1.4 | 205 | — | — | 0.78 | partial | Crate-doc lede (what anyhow is) |
| 1.5 | 295 | 3170 | +2875 | 1.00 | late | tests/ listing |
| 1.6 | 446 | 224 | -222 | 1.00 | early | Cargo package identity |
| 1.7 | 584 | 4517 | +3933 | 0.90 | late | Cargo features + runtime deps |
| 1.8 | 815 | — | — | 0.74 | partial | README usage — Result<T>/? return type |
| 1.9 | 1081 | — | — | 0.79 | partial | README usage — .context() and the Caused-by chain |
| 2.1 | 1180 | 4690 | +3510 | 1.00 | late | Public items in src/lib.rs (location batch) |
| 2.2 | 1334 | — | — | 0.00 | missing | Public items in src/error.rs (location batch) |
| 2.3 | 1374 | — | — | 0.50 | partial | Public macros — names (location batch) |
| 2.4 | 1534 | 1074 | -460 | 0.92 | aligned | Context trait — full definition |
| 2.5 | 1721 | 9274 | +7553 | 0.93 | late | Error struct — definition + one-word guarantees |
| 2.6 | 1754 | — | — | 0.44 | missing | Result alias + Ok helper + format_err re-export |
| 2.7 | 1995 | 3076 | +1081 | 0.92 | late | Chain iterator — signature + example |
| 2.8 | 2276 | — | — | 0.00 | missing | Error constructors — signatures + bounds |
| 2.9 | 2499 | — | — | 0.00 | missing | Error inspectors — chain, downcast, is, root_cause signatures |
| 3.1 | 2667 | 715 | -1952 | 0.85 | early | `bail!` macro — all three arms |
| 3.2 | 3187 | — | — | 0.48 | missing | `anyhow!` macro — all three arms + __anyhow helper |
| 3.3 | 3723 | — | — | 0.00 | missing | `ensure!` macro — doc arm and tt-muncher dispatch |
| 3.4 | 4376 | — | — | 0.13 | missing | `ensure!` expansion — BothDebug/NotBothDebug + __fancy/__fallback |
| 3.5 | 4765 | — | — | 0.38 | missing | `anyhow!($expr)` tagged dispatch — AdhocKind + TraitKind |
| 3.6 | 5038 | — | — | 0.35 | missing | `anyhow!($expr)` tagged dispatch — BoxedKind path |
| 3.7 | 5617 | — | — | 0.00 | missing | Context trait impls — Result<T,E> and Option<T> |
| 3.8 | 6113 | — | — | 0.09 | missing | ContextError — struct + Display/StdError impls |
| 3.9 | 6665 | — | — | 0.21 | missing | ErrorImpl + ErrorVTable — the hand-rolled layout |
| 3.10 | 7048 | — | — | 0.00 | missing | Error::construct — how a vtable is attached to an E |
| 3.11 | 7351 | — | — | 0.00 | missing | construct_from_std — vtable instantiation pattern |
| 3.12 | 7584 | — | — | 0.00 | missing | Context chain downcast — walking the nested ContextError |
| 3.13 | 7977 | — | — | 0.37 | missing | Own<T> — the thin-pointer wrapper |
| 3.14 | 8397 | — | — | 0.00 | missing | ErrorImpl::display/debug — rendering a chain |
| 3.15 | 8770 | — | — | 0.26 | missing | Chain<'a> — ChainState + Iterator::next |
| 3.16 | 8863 | — | — | 0.43 | missing | Wrapper types — MessageError / DisplayError / BoxedError |
| 3.17 | 9081 | 2884 | -6197 | 0.80 | early | no_std support — README section |
| 3.18 | 9189 | — | — | 0.12 | missing | Backtrace module — three-way cfg shape |
| 3.19 | 9515 | — | — | 0.00 | missing | build.rs — the cfgs emitted |
| 3.20 | 9746 | — | — | 0.00 | missing | From<E> for Error + Deref — auto-conversion surface |
| 3.21 | 9850 | — | — | 0.00 | missing | Error::into_boxed_dyn_error + reallocate_* signatures |
| 3.22 | 9967 | — | — | 0.08 | missing | Test file grouping (location batch for tests/) |

## Walker waste (cost ≥ 50, no NS intersection)

| first_t | cost | batch |
|--------:|-----:|:------|
| 2585 | 133 | README.md section #3 |
| 3942 | 269 | README.md section #4 |
| 2349 | 291 | README headline in README.md |
| 3369 | 67 | [package] in tests/crate/Cargo.toml |
| 1867 | 274 | crate-doc lede in src/lib.rs |
| 3598 | 135 | impl method sigs in tests/drop/mod.rs |
| 4310 | 82 | listing of 'tests/ui' |
| 3673 | 54 | mod/use plumbing in tests/drop/mod.rs |
| 1541 | 51 | pub item at src/ptr.rs:125 |
| 5255 | 539 | pub-item doc at src/lib.rs:468 |
| 8034 | 1647 | pub-item doc at src/lib.rs:616 |
| 4228 | 286 | pub-item doc at src/lib.rs:650 |
| 2674 | 89 | pub-item names surface in src/nightly.rs |
