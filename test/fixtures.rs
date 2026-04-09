// Shared fixture data, included by both tests/snapshots.rs and src/bin/clone_fixtures.rs.
//
// Includers must define two macros before including this file:
//   with_fixtures!(($dir, $url, $rev), ...)     — called with all fixture repos
//   with_entries!(($name, $path, $budget), ...) — called with all snapshot entries
//
// Entries should be meaningfully diverse — don't add both root and root/src
// when src is the only interesting content. Subfolder entries are for cases
// like separate crates in a workspace or packages in a monorepo.
//
// Budget guidelines:
//   2000 — small focused submodules (a few files)
//   4000 — typical libraries and most entries (the CLI default)
//   8000 — large multi-crate workspaces and monorepos

with_fixtures! {
    // Rust
    ("anyhow",              "https://github.com/dtolnay/anyhow.git",              "769cba0b"),
    ("thiserror",           "https://github.com/dtolnay/thiserror.git",           "9ac165c4"),
    ("log",                 "https://github.com/rust-lang/log.git",               "43f2c283"),
    ("mdbook",              "https://github.com/rust-lang/mdBook.git",            "b8c90970"),
    ("toasty",              "https://github.com/tokio-rs/toasty.git",             "0fb6be95"),
    ("sps",                 "https://github.com/alexykn/sps.git",                 "5a10e7f4"),
    ("otree",               "https://github.com/fioncat/otree.git",               "a02bdf44"),
    // Go
    ("go-multierror",       "https://github.com/hashicorp/go-multierror.git",     "edef97ed"),
    ("xxhash",              "https://github.com/cespare/xxhash.git",              "ab37246c"),
    ("mcphost",             "https://github.com/mark3labs/mcphost.git",           "191dcea1"),
    ("tock",                "https://github.com/kriuchkov/tock.git",              "b29815f2"),
    // TypeScript
    ("cmdk",                "https://github.com/pacocoursey/cmdk.git",            "dd2250ed"),
    ("vaul",                "https://github.com/emilkowalski/vaul.git",           "3e97aac6"),
    ("ts-pattern",          "https://github.com/gvergnaud/ts-pattern.git",        "2ece6ba5"),
    ("ky",                  "https://github.com/sindresorhus/ky.git",             "eb5c3eba"),
    ("superstruct",         "https://github.com/ianstormtaylor/superstruct.git",  "e414c8af"),
    ("mitt",                "https://github.com/developit/mitt.git",              "6b416705"),
    ("enclosed",            "https://github.com/CorentinTh/enclosed.git",         "461c3d41"),
    ("d2ts",                "https://github.com/electric-sql/d2ts.git",           "418591d5"),
    // JavaScript
    ("commander",           "https://github.com/tj/commander.js.git",             "82473649"),
    ("semver",              "https://github.com/npm/node-semver.git",             "5993c2e4"),
    // Python
    ("pluggy",              "https://github.com/pytest-dev/pluggy.git",           "4cc08c15"),
    ("typeguard",           "https://github.com/agronholm/typeguard.git",         "b05b7dab"),
    ("tomli",               "https://github.com/hukkin/tomli.git",                "920e20b1"),
    ("peepdb",              "https://github.com/evangelosmeklis/peepdb.git",       "929064dd"),
    ("swarm",               "https://github.com/openai/swarm.git",               "0c82d7d8"),
    ("htmy",                "https://github.com/volfpeter/htmy.git",              "4694fb86"),
    ("microbootstrap",      "https://github.com/community-of-python/microbootstrap.git", "609c420b"),
    ("py3xui",              "https://github.com/iwatkot/py3xui.git",              "6004c163"),
    // Python (ML)
    ("xlstm",               "https://github.com/NX-AI/xlstm.git",                "032a6fb8"),
    ("nano-vllm",           "https://github.com/GeeeekExplorer/nano-vllm.git",   "2f214426"),
    ("chronos-forecasting", "https://github.com/amazon-science/chronos-forecasting.git", "f951d9ae"),
    // C
    ("sds",                 "https://github.com/antirez/sds.git",                 "5347739b"),
    ("neco",                "https://github.com/tidwall/neco.git",                "9e8e19e4"),
    ("bareiron",            "https://github.com/p2r3/bareiron.git",               "ddb071c3"),
    ("krep",                "https://github.com/davidesantangelo/krep.git",       "ae96fbd2"),
    ("sqlite-vec",          "https://github.com/asg017/sqlite-vec.git",           "563a3e60"),
    ("soluna",              "https://github.com/cloudwu/soluna.git",              "be822052"),
}

with_entries! {
    // ── Rust ────────────────────────────────────────────────────────────
    (anyhow,                  "anyhow",                         4000),
    (thiserror,               "thiserror",                      4000),
    (thiserror_impl_src,      "thiserror/impl/src",             2000),
    (log,                     "log",                            4000),
    (log_src_kv,              "log/src/kv",                     2000),
    (mdbook,                  "mdbook",                         8000),
    (mdbook_guide_src,        "mdbook/guide/src",               4000),
    (toasty,                  "toasty",                         8000),
    (toasty_core,             "toasty/crates/toasty-core",      4000),
    (toasty_codegen,          "toasty/crates/toasty-codegen",   4000),
    (sps,                     "sps",                            8000),
    (sps_core,                "sps/sps-core",                   4000),
    (otree,                   "otree",                          4000),

    // ── Go ──────────────────────────────────────────────────────────────
    (go_multierror,           "go-multierror",                  4000),
    (xxhash,                  "xxhash",                         4000),
    (xxhash_xxhsum,           "xxhash/xxhsum",                  2000),
    (mcphost,                 "mcphost",                        8000),
    (mcphost_sdk,             "mcphost/sdk",                    2000),
    (tock,                    "tock",                            8000),
    (tock_internal_core,      "tock/internal/core",             2000),

    // ── TypeScript ──────────────────────────────────────────────────────
    (cmdk,                    "cmdk",                           4000),
    (cmdk_cmdk_src,           "cmdk/cmdk/src",                  2000),
    (vaul,                    "vaul",                            4000),
    (ts_pattern,              "ts-pattern",                     4000),
    (ts_pattern_src_types,    "ts-pattern/src/types",           2000),
    (ky,                      "ky",                              4000),
    (ky_source_errors,        "ky/source/errors",               2000),
    (superstruct,             "superstruct",                    4000),
    (superstruct_src_structs, "superstruct/src/structs",        2000),
    (mitt,                    "mitt",                            2000),
    (enclosed,                "enclosed",                        8000),
    (enclosed_crypto,         "enclosed/packages/crypto",       2000),
    (enclosed_lib,            "enclosed/packages/lib",          2000),
    (d2ts,                    "d2ts",                            4000),
    (d2ts_d2ts,               "d2ts/packages/d2ts",             4000),

    // ── JavaScript ──────────────────────────────────────────────────────
    (commander,               "commander",                      4000),
    (semver,                  "semver",                          4000),
    (semver_classes,          "semver/classes",                  2000),
    (semver_internal,         "semver/internal",                 2000),

    // ── Python ──────────────────────────────────────────────────────────
    (pluggy,                  "pluggy",                          4000),
    (typeguard,               "typeguard",                      4000),
    (tomli,                   "tomli",                           4000),
    (peepdb,                  "peepdb",                          4000),
    (peepdb_db,               "peepdb/peepdb/db",               2000),
    (swarm,                   "swarm",                           4000),

    // ── Composite symbols (single-line JSON) ──────────────────────────────
    // vscode's emojis.json: 1,837 key-value pairs on a single 40KB line.
    // Progressive disclosure at two budgets shows the composite rendering.
    // Requires perf fixtures: `cargo run --bin clone_fixtures -- --perf`
    (vscode_emojis_small,     "../perf-fixtures/vscode/extensions/git/resources", 100),
    (vscode_emojis_medium,    "../perf-fixtures/vscode/extensions/git/resources", 200),
    (htmy,                    "htmy",                            4000),
    (htmy_renderer,           "htmy/htmy/renderer",             2000),
    (microbootstrap,          "microbootstrap",                 4000),
    (microbootstrap_instruments, "microbootstrap/microbootstrap/instruments", 2000),
    (py3xui,                  "py3xui",                          4000),
    (py3xui_api,              "py3xui/py3xui/api",              2000),

    // ── Python (ML) ─────────────────────────────────────────────────────
    (xlstm,                   "xlstm",                           4000),
    (xlstm_blocks,            "xlstm/xlstm/blocks",             2000),
    (nano_vllm,               "nano-vllm",                      4000),
    (nano_vllm_engine,        "nano-vllm/nanovllm/engine",      2000),
    (chronos,                 "chronos-forecasting",             4000),

    // ── C ───────────────────────────────────────────────────────────────
    (sds,                     "sds",                             2000),
    (neco,                    "neco",                            2000),
    (bareiron,                "bareiron",                        4000),
    (krep,                    "krep",                            2000),
    (sqlite_vec,              "sqlite-vec",                     4000),
    (soluna,                  "soluna",                          4000),
}
