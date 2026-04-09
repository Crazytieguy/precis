use criterion::{criterion_group, criterion_main, Criterion};
use precis::{parse, schedule, walk, Corpus, FileData};
use std::path::{Path, PathBuf};

/// Pre-loaded fixture data to avoid I/O in benchmark loops.
struct Fixture {
    root: PathBuf,
    files: Vec<PathBuf>,
    file_data: Vec<FileData>,
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
        let sources = precis::read_sources(&files);
        let file_data = precis::build_file_data(&root, &files, sources);
        Some(Fixture { root, files, file_data })
    }

    fn corpus(&self) -> Corpus<'_> {
        Corpus { files: &self.file_data }
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
        let configs = parse::build_language_configs(&f.files);
        c.bench_function(bench_name, |b| {
            b.iter(|| {
                for fd in &f.file_data {
                    if let Some(ref s) = fd.source {
                        parse::extract_file_symbols(&fd.info.relative_path, s, &configs);
                    }
                }
            });
        });
    }
}

fn bench_build_file_data(c: &mut Criterion) {
    let fixtures: &[(&str, &str)] = &[
        ("pluggy/src/pluggy", "build_file_data/pluggy_src"),
        ("commander/lib", "build_file_data/commander_lib"),
    ];

    for &(subpath, bench_name) in fixtures {
        let Some(f) = Fixture::load(subpath) else {
            continue;
        };
        c.bench_function(bench_name, |b| {
            b.iter(|| {
                // Clone sources to simulate fresh owned data each iteration
                let sources: Vec<Option<String>> = f.file_data.iter()
                    .map(|fd| fd.source.clone())
                    .collect();
                precis::build_file_data(&f.root, &f.files, sources);
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
        let corpus = f.corpus();
        c.bench_function(bench_name, |b| {
            b.iter(|| {
                schedule::build_groups(&corpus, 4000);
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
        let corpus = f.corpus();
        let built = schedule::build_groups(&corpus, 4000);
        c.bench_function(bench_name, |b| {
            b.iter(|| {
                schedule::schedule(&built, &corpus, None);
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
                // Clone sources to exercise the full pipeline each iteration
                let sources: Vec<Option<String>> = f.file_data.iter()
                    .map(|fd| fd.source.clone())
                    .collect();
                precis::render_with_budget(budget, None, &f.root, &f.files, sources);
            });
        });
    }
}

criterion_group!(
    benches,
    bench_extract_symbols,
    bench_build_file_data,
    bench_build_groups,
    bench_schedule,
    bench_render_with_budget,
);
criterion_main!(benches);
