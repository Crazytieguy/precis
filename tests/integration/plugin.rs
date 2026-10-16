//! The per-platform plugin (plugins/precis): its unpacker, its version, and
//! the files it shares with the plain plugin.

use std::path::{Path, PathBuf};
use std::process::Command;

fn plugins() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("plugins")
}

/// A plugin can't import another plugin's files, so each carries a copy.
#[test]
fn plugin_shared_files_match() {
    for file in [
        "hooks/notice-band.tsx",
        "hooks/switch.tsx",
        "types/index.d.ts",
    ] {
        let copies = ["precis", "precis-legacy"]
            .map(|plugin| std::fs::read_to_string(plugins().join(plugin).join(file)).unwrap());
        assert!(
            copies[0] == copies[1],
            "the plugins' copies of {file} differ"
        );
    }
}

/// The plugin archives bundle the binary of the release tagged with the
/// plugin's version.
#[test]
fn plugin_version_is_the_crate_version() {
    let manifest =
        std::fs::read_to_string(plugins().join("precis/.claude-plugin/plugin.json")).unwrap();
    let version = format!("\"version\": \"{}\"", env!("CARGO_PKG_VERSION"));
    assert!(manifest.contains(&version), "{manifest}");
}

/// A copy of the plugin at version `version`, bundling a cargo-dist
/// archive whose binary prints `marker`; returns the plugin and the bundle.
fn bundling_plugin(dir: &Path, version: &str, marker: &str) -> (PathBuf, PathBuf) {
    let root = dir.join("plugin");
    std::fs::create_dir_all(root.join(".claude-plugin")).unwrap();
    std::fs::create_dir_all(root.join("scripts")).unwrap();
    std::fs::create_dir_all(root.join("bin")).unwrap();
    std::fs::copy(
        plugins().join("precis/scripts/unpack.sh"),
        root.join("scripts/unpack.sh"),
    )
    .unwrap();
    std::fs::write(
        root.join(".claude-plugin/plugin.json"),
        format!("{{\n  \"name\": \"precis\",\n  \"version\": \"{version}\"\n}}\n"),
    )
    .unwrap();
    let nested = "precis-aarch64-apple-darwin";
    std::fs::create_dir_all(dir.join(nested)).unwrap();
    std::fs::write(
        dir.join(nested).join("precis"),
        format!("#!/bin/sh\necho {marker}\n"),
    )
    .unwrap();
    let bundle = root.join(format!("bin/{nested}.tar.xz"));
    let packed = Command::new("tar")
        .arg("-cJf")
        .arg(&bundle)
        .arg("-C")
        .arg(dir)
        .arg(nested)
        .status()
        .unwrap();
    assert!(packed.success());
    (root, bundle)
}

fn unpack(root: &Path, bundle: &Path, cache: &Path) -> std::process::Output {
    Command::new("bash")
        .arg(root.join("scripts/unpack.sh"))
        .arg(bundle)
        .env("XDG_CACHE_HOME", cache)
        .output()
        .unwrap()
}

/// The bundled binary lands under the plugin's version, executable, and a
/// later run keeps it.
#[test]
fn plugin_unpack_puts_the_bundled_binary_under_the_version() {
    let temp = tempfile::tempdir().unwrap();
    let (root, bundle) = bundling_plugin(temp.path(), "9.8.7", "bundled");
    let cache = temp.path().join("cache");
    let first = unpack(&root, &bundle, &cache);
    assert!(first.status.success(), "{first:?}");
    let binary = cache.join("precis/v9.8.7/precis");
    assert_eq!(
        String::from_utf8(first.stdout).unwrap().trim(),
        binary.to_str().unwrap()
    );
    let ran = Command::new(&binary).output().unwrap();
    assert_eq!(String::from_utf8(ran.stdout).unwrap(), "bundled\n");

    bundling_plugin(temp.path(), "9.8.7", "rebuilt");
    let again = unpack(&root, &bundle, &cache);
    assert!(again.status.success(), "{again:?}");
    let ran = Command::new(&binary).output().unwrap();
    assert_eq!(String::from_utf8(ran.stdout).unwrap(), "bundled\n");
}
