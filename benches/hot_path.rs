use criterion::{Criterion, black_box, criterion_group, criterion_main};
use std::path::Path;

struct RenderCase {
    name: &'static str,
    root: &'static str,
    budget: usize,
}

const FIXTURE_CASES: &[RenderCase] = &[
    RenderCase {
        name: "fixture/pluggy_src_4k",
        root: "test/fixtures/pluggy/src/pluggy",
        budget: 4000,
    },
    RenderCase {
        name: "fixture/commander_lib_4k",
        root: "test/fixtures/commander/lib",
        budget: 4000,
    },
    RenderCase {
        name: "fixture/mdbook_8k",
        root: "test/fixtures/mdbook",
        budget: 8000,
    },
    RenderCase {
        name: "fixture/toasty_8k",
        root: "test/fixtures/toasty",
        budget: 8000,
    },
    RenderCase {
        name: "fixture/sps_8k",
        root: "test/fixtures/sps",
        budget: 8000,
    },
];

const PERF_CASES: &[RenderCase] = &[
    RenderCase {
        name: "perf/django_4k",
        root: "test/perf-fixtures/django",
        budget: 4000,
    },
    RenderCase {
        name: "perf/deno_4k",
        root: "test/perf-fixtures/deno",
        budget: 4000,
    },
    RenderCase {
        name: "perf/vscode_4k",
        root: "test/perf-fixtures/vscode",
        budget: 4000,
    },
    RenderCase {
        name: "perf/cpython_4k",
        root: "test/perf-fixtures/cpython",
        budget: 4000,
    },
    RenderCase {
        name: "perf/typescript_4k",
        root: "test/perf-fixtures/typescript",
        budget: 4000,
    },
    RenderCase {
        name: "perf/typescript_10k",
        root: "test/perf-fixtures/typescript",
        budget: 10_000,
    },
];

fn bench_cases(c: &mut Criterion, cases: &[RenderCase]) {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    for case in cases {
        let root = manifest_dir.join(case.root);
        if root.exists() {
            c.bench_function(case.name, |b| {
                b.iter(|| black_box(precis::render(&root, case.budget, None)));
            });
        } else {
            eprintln!("skipping missing benchmark fixture: {}", root.display());
        }
    }
}

fn bench_render_fixtures(c: &mut Criterion) {
    bench_cases(c, FIXTURE_CASES);
}

fn bench_render_perf_fixtures(c: &mut Criterion) {
    bench_cases(c, PERF_CASES);
}

criterion_group!(benches, bench_render_fixtures, bench_render_perf_fixtures,);
criterion_main!(benches);
