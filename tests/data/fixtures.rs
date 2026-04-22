// Shared fixture repo list, included by src/bin/clone_fixtures.rs and future
// test consumers. Includers must define a `with_fixtures!` macro that accepts
// `($dir, $url, $rev)` tuples.
//
// Listing here only declares a fixture as cloneable. Fixtures become *active*
// snapshot targets when added to `tests/snapshots.rs::FIXTURES` — typically
// after a North Star at `tests/north-stars/<name>.md` is in place. See
// `Skill(add-fixture)` for the full integration flow.

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
