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
//   1000 — small focused submodules (a few files)
//   2000 — typical libraries and most entries (the CLI default)
//   4000 — large multi-crate workspaces and monorepos

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
    (anyhow,                  "anyhow",                         2000), // inspected: exceeds explore
    (thiserror,               "thiserror",                      2000), // inspected: exceeds explore
    (thiserror_impl_src,      "thiserror/impl/src",             1000), // inspected: exceeds explore
    (log,                     "log",                            2000), // inspected: logged observation
    (log_src_kv,              "log/src/kv",                     1000), // inspected: logged observation
    (mdbook,                  "mdbook",                         4000), // inspected: logged observation
    (mdbook_guide_src,        "mdbook/guide/src",               2000), // inspected: logged observation
    (toasty,                  "toasty",                         4000), // inspected: logged observation
    (toasty_core,             "toasty/crates/toasty-core",      2000), // inspected: logged observation
    (toasty_codegen,          "toasty/crates/toasty-codegen",   2000), // inspected: logged observation
    (sps,                     "sps",                            4000),
    (sps_core,                "sps/sps-core",                   2000), // inspected: logged observation
    (otree,                   "otree",                          2000),

    // ── Go ──────────────────────────────────────────────────────────────
    (go_multierror,           "go-multierror",                  2000),
    (xxhash,                  "xxhash",                         2000),
    (xxhash_xxhsum,           "xxhash/xxhsum",                  1000),
    (mcphost,                 "mcphost",                        4000), // inspected: logged observation
    (mcphost_sdk,             "mcphost/sdk",                    1000),
    (tock,                    "tock",                            4000), // inspected: deprioritize mocks/
    (tock_internal_core,      "tock/internal/core",             1000), // inspected: deprioritize mocks/

    // ── TypeScript ──────────────────────────────────────────────────────
    (cmdk,                    "cmdk",                           2000), // inspected: logged observation
    (cmdk_cmdk_src,           "cmdk/cmdk/src",                  1000),
    (vaul,                    "vaul",                            2000),
    (ts_pattern,              "ts-pattern",                     2000),
    (ts_pattern_src_types,    "ts-pattern/src/types",           1000),
    (ky,                      "ky",                              2000),
    (ky_source_errors,        "ky/source/errors",               1000), // error hierarchy
    (superstruct,             "superstruct",                    2000),
    (superstruct_src_structs, "superstruct/src/structs",        1000), // validator definitions
    (mitt,                    "mitt",                            1000),
    (enclosed,                "enclosed",                        4000), // inspected: deprioritize locale data files
    (enclosed_crypto,         "enclosed/packages/crypto",       1000), // crypto package
    (enclosed_lib,            "enclosed/packages/lib",          1000), // shared library
    (d2ts,                    "d2ts",                            2000),
    (d2ts_d2ts,               "d2ts/packages/d2ts",             2000), // main library

    // ── JavaScript ──────────────────────────────────────────────────────
    (commander,               "commander",                      2000),
    (semver,                  "semver",                          2000), // inspected: deprioritize .github/
    (semver_classes,          "semver/classes",                  1000), // class definitions
    (semver_internal,         "semver/internal",                 1000), // internal helpers

    // ── Python ──────────────────────────────────────────────────────────
    (pluggy,                  "pluggy",                          2000),
    (typeguard,               "typeguard",                      2000),
    (tomli,                   "tomli",                           2000),
    (peepdb,                  "peepdb",                          2000),
    (peepdb_db,               "peepdb/peepdb/db",               1000), // database adapters
    (swarm,                   "swarm",                           2000),
    (htmy,                    "htmy",                            2000),
    (htmy_renderer,           "htmy/htmy/renderer",             1000), // rendering subsystem
    (microbootstrap,          "microbootstrap",                 2000),
    (microbootstrap_instruments, "microbootstrap/microbootstrap/instruments", 1000), // plugins
    (py3xui,                  "py3xui",                          2000),
    (py3xui_api,              "py3xui/py3xui/api",              1000), // sync API layer

    // ── Python (ML) ─────────────────────────────────────────────────────
    (xlstm,                   "xlstm",                           2000),
    (xlstm_blocks,            "xlstm/xlstm/blocks",             1000), // model blocks
    (nano_vllm,               "nano-vllm",                      2000),
    (nano_vllm_engine,        "nano-vllm/nanovllm/engine",      1000), // inference engine
    (chronos,                 "chronos-forecasting",             2000),

    // ── C ───────────────────────────────────────────────────────────────
    (sds,                     "sds",                             1000),
    (neco,                    "neco",                            1000),
    (bareiron,                "bareiron",                        2000),
    (krep,                    "krep",                            1000),
    (sqlite_vec,              "sqlite-vec",                     2000),
    (soluna,                  "soluna",                          2000),
}
