//! Per-phase timing + cross-cutting counters. Compiled only with the
//! `timing` feature; without it, this module doesn't exist and the
//! call sites' cfg-gated blocks expand to nothing.

use std::cell::RefCell;
use std::time::{Duration, Instant};

pub struct PhaseTimer {
    name: &'static str,
    start: Instant,
}

impl PhaseTimer {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            start: Instant::now(),
        }
    }
}

impl Drop for PhaseTimer {
    fn drop(&mut self) {
        let ms = self.start.elapsed().as_secs_f64() * 1000.0;
        eprintln!("[timing] {}: {:.3} ms", self.name, ms);
    }
}

pub type CounterField = fn(&mut Counters) -> &mut Counter;

/// RAII guard for a counter slot. Records elapsed-on-Drop into the field
/// returned by `field`. For hit/miss counters, use `record_with_hit` at
/// the cache-branch directly — a guard can't observe which branch ran.
pub struct CounterGuard {
    field: CounterField,
    start: Instant,
}

impl CounterGuard {
    pub fn new(field: CounterField) -> Self {
        Self {
            field,
            start: Instant::now(),
        }
    }
}

impl Drop for CounterGuard {
    fn drop(&mut self) {
        record(self.field, self.start.elapsed(), None);
    }
}

#[derive(Default, Clone, Copy)]
pub struct Counter {
    pub calls: u64,
    pub hits: u64,
    pub misses: u64,
    // u64 ns gives 584 years of headroom — plenty.
    pub ns_total: u64,
}

#[derive(Default)]
pub struct Counters {
    pub tokenizer: Counter,
    pub parse: Counter,
    pub source_read: Counter,
    pub best_exact: Counter,
    pub best_exact_cost_pass: Counter,
    pub best_exact_rank_pass: Counter,
    pub schedule_apply: Counter,
    pub schedule_expand: Counter,
}

thread_local! {
    pub static COUNTERS: RefCell<Counters> = RefCell::new(Counters::default());
}

pub fn record(field: CounterField, elapsed: Duration, hit: Option<bool>) {
    COUNTERS.with(|c| {
        let mut c = c.borrow_mut();
        let counter = field(&mut c);
        counter.calls += 1;
        counter.ns_total = counter.ns_total.saturating_add(elapsed.as_nanos() as u64);
        match hit {
            Some(true) => counter.hits += 1,
            Some(false) => counter.misses += 1,
            None => {}
        }
    });
}

pub fn dump_and_reset() {
    type DumpRow = (&'static str, fn(&Counters) -> &Counter, bool);
    COUNTERS.with(|c| {
        let mut c = c.borrow_mut();
        eprintln!("[timing] counters:");
        let rows: [DumpRow; 8] = [
            ("tokenizer", |c| &c.tokenizer, true),
            ("parse", |c| &c.parse, true),
            ("source_read", |c| &c.source_read, true),
            ("best_exact", |c| &c.best_exact, false),
            (" \u{2517} cost_pass", |c| &c.best_exact_cost_pass, false),
            (" \u{2517} rank_pass", |c| &c.best_exact_rank_pass, false),
            ("sched.apply", |c| &c.schedule_apply, false),
            ("sched.expand", |c| &c.schedule_expand, false),
        ];
        for (label, get, show_cache) in rows {
            let x = get(&c);
            if x.calls == 0 {
                continue;
            }
            let ms = x.ns_total as f64 / 1_000_000.0;
            let per_call_us = (x.ns_total as f64 / 1000.0) / (x.calls as f64);
            let cache = if show_cache {
                format!(" ({} hits / {} misses)", x.hits, x.misses)
            } else {
                String::new()
            };
            eprintln!(
                "[timing]   {label:<14}: {calls} calls{cache}, {ms:.3} ms total, {per:.2} µs/call",
                label = label,
                calls = x.calls,
                cache = cache,
                ms = ms,
                per = per_call_us
            );
        }
        *c = Counters::default();
    });
}
