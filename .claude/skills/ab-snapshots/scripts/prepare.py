#!/usr/bin/env python3
"""prepare.py — regenerate snapshots, detect changes, write A/B pair files + manifest.

Workflow:
  1. Run `cargo nextest run --release` so any changed snapshot produces a
     `.snap.new` alongside the committed `.snap`.
  2. For each `.snap.new`, write a pair file under the output dir containing
     the committed and working-tree outputs, with randomized A/B labels.
  3. Emit a manifest (JSON) listing each pair with the A/B→committed/new
     mapping so the parent can decode verdicts.

Output dir defaults to /tmp/precis-ab; override with env AB_OUT_DIR.

The manifest path is printed to stdout. All progress goes to stderr.
"""
import json
import os
import random
import re
import shutil
import subprocess
import sys
from pathlib import Path


def main():
    project_dir = Path(sys.argv[1] if len(sys.argv) > 1 else ".").resolve()
    os.chdir(project_dir)

    out_dir = Path(os.environ.get("AB_OUT_DIR", "/tmp/precis-ab"))
    if out_dir.exists():
        shutil.rmtree(out_dir)
    (out_dir / "pairs").mkdir(parents=True)

    print("[ab] running cargo nextest to regenerate snapshots...", file=sys.stderr)
    subprocess.run(
        ["cargo", "nextest", "run", "--release", "--no-fail-fast"],
        check=False,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )

    # Snapshots live in both tests/snapshots/ (sample tests) and
    # test/snapshots/ (fixture tests) — insta picks the directory from the
    # source-file location where the test macro is expanded.
    snap_dirs = [project_dir / "tests" / "snapshots", project_dir / "test" / "snapshots"]
    new_files: list[Path] = []
    for d in snap_dirs:
        if d.exists():
            new_files.extend(d.glob("*.snap.new"))
    new_files.sort()

    fixtures_path = parse_fixtures_rs(project_dir / "test" / "fixtures.rs")

    manifest = []
    skipped_no_baseline = []
    for new_file in new_files:
        base = new_file.name
        if base.endswith(".snap.new"):
            base = base[: -len(".snap.new")]
        fixture_key = base.removeprefix("snapshots__")
        committed = new_file.with_suffix("")  # drops .new, leaving .snap

        if not committed.exists():
            skipped_no_baseline.append(fixture_key)
            continue

        pair_file = out_dir / "pairs" / f"{fixture_key}.pair.md"

        if random.random() < 0.5:
            a_src, a_path = "committed", committed
            b_src, b_path = "new", new_file
        else:
            a_src, a_path = "new", new_file
            b_src, b_path = "committed", committed

        with open(pair_file, "w") as f:
            f.write(f"# A/B pair for fixture: {fixture_key}\n\n")
            f.write("## Output A\n\n```\n")
            f.write(a_path.read_text())
            f.write("\n```\n\n## Output B\n\n```\n")
            f.write(b_path.read_text())
            f.write("\n```\n")

        fixture_path = fixtures_path.get(fixture_key)
        entry = {
            "fixture": fixture_key,
            "pair_file": str(pair_file),
            "fixture_path": fixture_path,
            "a_is": a_src,
            "b_is": b_src,
        }
        manifest.append(entry)

    manifest_path = out_dir / "manifest.json"
    with open(manifest_path, "w") as f:
        json.dump(manifest, f, indent=2)

    print(f"[ab] {len(manifest)} pair(s) written to {out_dir / 'pairs'}", file=sys.stderr)
    if skipped_no_baseline:
        print(
            f"[ab] {len(skipped_no_baseline)} new snapshot(s) skipped (no committed baseline): "
            + ", ".join(skipped_no_baseline),
            file=sys.stderr,
        )
    print(manifest_path)


def parse_fixtures_rs(path: Path) -> dict[str, str]:
    try:
        content = path.read_text()
    except FileNotFoundError:
        return {}

    in_entries = False
    out: dict[str, str] = {}
    for line in content.splitlines():
        stripped = line.strip()
        if stripped.startswith("with_entries!"):
            in_entries = True
            continue
        if not in_entries:
            continue
        if stripped.startswith("}"):
            break
        m = re.match(r"\(\s*(\w+)\s*,\s*\"([^\"]+)\"\s*,\s*(\d+)", stripped)
        if m:
            out[m.group(1)] = f"test/fixtures/{m.group(2)}"
    return out


if __name__ == "__main__":
    main()
