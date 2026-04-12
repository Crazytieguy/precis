use criterion::{criterion_group, criterion_main, Criterion};
use std::path::Path;

fn bench_render_small(c: &mut Criterion) {
    let fixtures: &[(&str, usize, &str)] = &[
        ("pluggy/src/pluggy", 4000, "render/pluggy_src_4000"),
        ("commander/lib", 4000, "render/commander_lib_4000"),
    ];

    for &(subpath, budget, bench_name) in fixtures {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("test/fixtures")
            .join(subpath);
        if !root.exists() {
            continue;
        }
        c.bench_function(bench_name, |b| {
            b.iter(|| {
                precis::render(&root, budget, None);
            });
        });
    }
}

fn bench_render_large(c: &mut Criterion) {
    let fixtures: &[(&str, usize, &str)] = &[
        ("sps", 8000, "render/sps_8000"),
    ];

    for &(subpath, budget, bench_name) in fixtures {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("test/fixtures")
            .join(subpath);
        if !root.exists() {
            continue;
        }
        c.bench_function(bench_name, |b| {
            b.iter(|| {
                precis::render(&root, budget, None);
            });
        });
    }
}

criterion_group!(
    benches,
    bench_render_small,
    bench_render_large,
);
criterion_main!(benches);
