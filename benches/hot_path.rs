use criterion::{criterion_group, criterion_main, Criterion};
use precis::{format, layout, schedule, walk};
use std::path::{Path, PathBuf};

/// Pre-loaded fixture data to avoid I/O in benchmark loops.
struct Fixture {
    root: PathBuf,
    files: Vec<PathBuf>,
    sources: Vec<Option<String>>,
    all_symbols: Vec<Vec<precis::parse::Symbol>>,
}

impl Fixture {
    fn load(subpath: &str) -> Option<Self> {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("test/fixtures")
            .join(subpath);
        if !root.exists() {
            return None;
        }
        let files = walk::discover_source_files(&root);
        let sources = format::read_sources(&files);
        let all_symbols = format::extract_all_symbols(&files, &sources);
        Some(Fixture {
            root,
            files,
            sources,
            all_symbols,
        })
    }
}

fn bench_extract_symbols(c: &mut Criterion) {
    let fixtures: &[(&str, &str)] = &[
        ("pluggy/src/pluggy", "extract_symbols/pluggy_src"),
        ("commander/lib", "extract_symbols/commander_lib"),
    ];

    for &(subpath, bench_name) in fixtures {
        let Some(f) = Fixture::load(subpath) else {
            continue;
        };
        c.bench_function(bench_name, |b| {
            b.iter(|| {
                format::extract_all_symbols(&f.files, &f.sources);
            });
        });
    }
}

fn bench_build_groups(c: &mut Criterion) {
    let fixtures: &[(&str, &str)] = &[
        ("pluggy/src/pluggy", "build_groups/pluggy_src"),
        ("commander/lib", "build_groups/commander_lib"),
    ];

    for &(subpath, bench_name) in fixtures {
        let Some(f) = Fixture::load(subpath) else {
            continue;
        };
        let layouts = layout::compute_all_layouts(&f.files, &f.sources, &f.all_symbols);
        c.bench_function(bench_name, |b| {
            b.iter(|| {
                schedule::build_groups(&f.root, &f.files, &f.sources, &f.all_symbols, &layouts, 4000);
            });
        });
    }
}

fn bench_schedule(c: &mut Criterion) {
    let fixtures: &[(&str, &str)] = &[
        ("pluggy/src/pluggy", "schedule/pluggy_src"),
        ("commander/lib", "schedule/commander_lib"),
    ];

    for &(subpath, bench_name) in fixtures {
        let Some(f) = Fixture::load(subpath) else {
            continue;
        };
        let layouts = layout::compute_all_layouts(&f.files, &f.sources, &f.all_symbols);
        let built = schedule::build_groups(&f.root, &f.files, &f.sources, &f.all_symbols, &layouts, 4000);
        c.bench_function(bench_name, |b| {
            b.iter(|| {
                schedule::schedule(&built, &f.root, &f.files);
            });
        });
    }
}

fn bench_render_with_budget(c: &mut Criterion) {
    let configs: &[(&str, usize, &str)] = &[
        ("pluggy/src/pluggy", 4000, "render/pluggy_src_4000"),
        ("commander/lib", 4000, "render/commander_lib_4000"),
    ];

    for &(subpath, budget, bench_name) in configs {
        let Some(f) = Fixture::load(subpath) else {
            continue;
        };
        c.bench_function(bench_name, |b| {
            b.iter(|| {
                format::render_with_budget(budget, &f.root, &f.files, &f.sources);
            });
        });
    }
}

criterion_group!(
    benches,
    bench_extract_symbols,
    bench_build_groups,
    bench_schedule,
    bench_render_with_budget,
);
criterion_main!(benches);
