#!/usr/bin/env python3
"""Build and publish the platform-specific plugin archives.

The `precis-<target>` marketplace entries install from these zips: the
tracked files of plugins/precis plus that target's release binary at
bin/precis-<target>.tar.xz, so plugin and binary install as one artifact. The
binary is the release whose tag matches the plugin's version, which the
release skill keeps equal to the crate's.

The zips live at fixed urls, on one rolling release. Claude Code downloads an
archive-sourced plugin on every update pass and reads the version out of the
downloaded zip, so the url carries no version and `marketplace.json` never
changes: publishing replaces the assets in place. Between a release's push and
this upload, a url serves the previous release's plugin and binary together.

The build is byte-deterministic (contents and modes from `git ls-files -s`,
sorted entries, fixed timestamps, stored uncompressed), so publish skips
assets that are already identical.

Commands:
  build     build the zips into dist/plugin-archives/ (downloads the binaries)
  publish   build, then upload, replacing what is there (the release workflow).

Ported from alignment-hive's scripts/plugin-archives.py.
"""

from __future__ import annotations

import argparse
import hashlib
import io
import json
import subprocess
import sys
import urllib.error
import urllib.request
import zipfile
from pathlib import Path

REPO = "Crazytieguy/precis"
PLUGIN = "precis"

# One rolling release holds every archive. Not a `v*` tag, so the cargo-dist
# release workflow never runs for it.
ARCHIVE_TAG = "plugin-archives"

RELEASE_NOTES = (
    "Platform-specific plugin archives, referenced by the `precis-<target>` "
    "marketplace entries. The urls are fixed: each release replaces these "
    "assets in place, and the version lives in the `plugin.json` inside each zip."
)

# 1980-01-01, the earliest timestamp the zip format can represent.
ZIP_EPOCH = (1980, 1, 1, 0, 0, 0)


def run(*args: str, cwd: Path | None = None) -> str:
    result = subprocess.run(args, cwd=cwd, capture_output=True, text=True)
    if result.returncode != 0:
        sys.exit(
            f"error: `{' '.join(args)}` exited {result.returncode}\n"
            f"{result.stderr.strip() or result.stdout.strip()}"
        )
    return result.stdout


def repo_root() -> Path:
    return Path(run("git", "rev-parse", "--show-toplevel").strip())


def targets(root: Path) -> list[str]:
    """The targets marketplace.json has archive entries for."""
    marketplace = json.loads((root / ".claude-plugin" / "marketplace.json").read_text())
    prefix = f"{PLUGIN}-"
    return [
        entry["name"].removeprefix(prefix)
        for entry in marketplace["plugins"]
        if isinstance(entry["source"], dict) and entry["source"].get("source") == "archive"
    ]


def plugin_version(root: Path) -> str:
    manifest = root / "plugins" / PLUGIN / ".claude-plugin" / "plugin.json"
    return json.loads(manifest.read_text())["version"]


def download_url(tag: str, name: str) -> str:
    return f"https://github.com/{REPO}/releases/download/{tag}/{name}"


# --- building ----------------------------------------------------------------


def fetch(url: str) -> bytes:
    with urllib.request.urlopen(url, timeout=120) as response:
        return response.read()


def binary_asset(root: Path, version: str, target: str) -> bytes:
    """The cargo-dist .tar.xz for one target, checked against its sidecar.

    The sidecar is fetched even on a cache hit: re-running a release for an
    existing tag replaces the bytes under the same version.
    """
    name = f"{PLUGIN}-{target}.tar.xz"
    tag = f"v{version}"
    cache = root / "dist" / "plugin-archives" / ".cache" / tag / name

    try:
        expected = fetch(download_url(tag, f"{name}.sha256")).decode().split()[0]
    except urllib.error.HTTPError as error:
        if error.code == 404:
            sys.exit(f"error: release {tag} has no asset {name}; publish the release first")
        raise

    if cache.exists():
        data = cache.read_bytes()
        if hashlib.sha256(data).hexdigest() == expected:
            return data
        print(f"warning: cached {name} is stale, re-downloading", file=sys.stderr)

    data = fetch(download_url(tag, name))
    actual = hashlib.sha256(data).hexdigest()
    if actual != expected:
        sys.exit(f"error: {name} digest {actual} != published {expected}")

    cache.parent.mkdir(parents=True, exist_ok=True)
    cache.write_bytes(data)
    return data


def plugin_files(root: Path) -> list[tuple[str, int, bytes]]:
    """(path relative to the plugin dir, unix mode, contents), from git.

    git, not a filesystem walk, is the authority: it carries the executable
    bit and leaves out untracked files that would vary between machines.
    """
    prefix = f"plugins/{PLUGIN}/"
    files = []
    for line in run("git", "ls-files", "-s", "--", prefix, cwd=root).splitlines():
        meta, path = line.split("\t", 1)
        git_mode = meta.split()[0]
        if git_mode not in ("100644", "100755"):
            sys.exit(f"error: {path} has unsupported git mode {git_mode}")
        mode = 0o755 if git_mode == "100755" else 0o644
        files.append((path[len(prefix):], mode, (root / path).read_bytes()))
    if not files:
        sys.exit(f"error: no tracked files under {prefix}")
    return sorted(files)


def write_entry(archive: zipfile.ZipFile, name: str, mode: int, data: bytes) -> None:
    info = zipfile.ZipInfo(name, date_time=ZIP_EPOCH)
    info.create_system = 3  # unix, so the mode bits below are honoured
    info.external_attr = mode << 16
    archive.writestr(info, data)


def build_zip(files: list[tuple[str, int, bytes]], bundle: str, binary: bytes) -> bytes:
    buffer = io.BytesIO()
    with zipfile.ZipFile(buffer, "w", zipfile.ZIP_STORED) as archive:
        for relative, mode, data in files:
            write_entry(archive, relative, mode, data)
        write_entry(archive, bundle, 0o644, binary)
    return buffer.getvalue()


def build(root: Path) -> list[dict]:
    out_dir = root / "dist" / "plugin-archives"
    version = plugin_version(root)
    files = plugin_files(root)
    archives = []

    for target in targets(root):
        # The name scripts/unpack.sh looks up for its own target: a bin/ that
        # holds only another target's bundle is the wrong variant.
        bundle = f"bin/{PLUGIN}-{target}.tar.xz"
        data = build_zip(files, bundle, binary_asset(root, version, target))
        path = out_dir / f"{PLUGIN}-{target}.zip"
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(data)
        digest = hashlib.sha256(data).hexdigest()
        archives.append({"name": path.name, "path": path, "sha256": digest})
        print(f"{digest}  {len(data) / 1e6:5.2f} MB  {path.name}")

    return archives


# --- publishing --------------------------------------------------------------


def published_digests() -> dict[str, str] | None:
    """{asset name: sha256} for the rolling release, or None if it is absent."""
    result = subprocess.run(
        ["gh", "api", f"repos/{REPO}/releases/tags/{ARCHIVE_TAG}"],
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        # Only a 404 means "not created yet": a network failure reported as
        # absent would send us into `gh release create`, which then fails.
        if "HTTP 404" not in result.stderr:
            sys.exit(f"error: could not read release {ARCHIVE_TAG}\n{result.stderr.strip()}")
        return None
    return {
        asset["name"]: (asset.get("digest") or "").removeprefix("sha256:")
        for asset in json.loads(result.stdout)["assets"]
    }


def publish(archives: list[dict]) -> int:
    digests = published_digests()

    if digests is None:
        print(f"Creating release {ARCHIVE_TAG}")
        run(
            "gh", "release", "create", ARCHIVE_TAG,
            "--repo", REPO,
            "--title", "Plugin archives (rolling)",
            "--notes", RELEASE_NOTES,
            "--latest=false",
        )
        digests = {}

    changed = [archive for archive in archives if digests.get(archive["name"]) != archive["sha256"]]
    for archive in archives:
        if archive not in changed:
            print(f"{archive['name']} already published and identical")
    if changed:
        print(f"Uploading {', '.join(archive['name'] for archive in changed)}")
        paths = [str(archive["path"]) for archive in changed]
        run("gh", "release", "upload", ARCHIVE_TAG, *paths, "--repo", REPO, "--clobber")

    return 0


def main() -> int:
    parser = argparse.ArgumentParser(description="Build and publish the platform-specific plugin archives.")
    parser.add_argument("command", choices=("build", "publish"))
    args = parser.parse_args()

    archives = build(repo_root())
    if args.command == "publish":
        return publish(archives)
    return 0


if __name__ == "__main__":
    sys.exit(main())
