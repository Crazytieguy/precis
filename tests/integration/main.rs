//! All integration tests, in one binary so a `src/` change relinks once.

mod cli;
mod fixture_baselines;
mod ns_simulate;
#[cfg(unix)]
mod plugin;
#[cfg(unix)]
mod plugin_legacy;
mod readme_example;
mod robustness;
mod scheduler_invariants;
mod single_file;
