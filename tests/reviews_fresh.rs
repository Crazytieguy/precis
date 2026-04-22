//! Every committed snapshot must have a matching review in `tests/reviews/`
//! whose `snapshot_hash` frontmatter equals the snapshot's current SHA-256.
//!
//! If this test fails, the snapshots and reviews have drifted — re-run the
//! alignment reviewer for each listed `<fixture>__<budget>`, commit the
//! refreshed reports, and the test flips back green.
//!
//! The hash rule matches exactly what the `alignment-reviewer` agent
//! computes (`shasum -a 256 <snap>`); both read the full file bytes
//! (including insta's YAML header), so nothing extra to strip.

use std::path::PathBuf;

use sha2::{Digest, Sha256};

#[test]
fn every_snapshot_has_a_fresh_review() {
    let snapshots_dir =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/snapshots/fixtures");
    let reviews_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/reviews");

    let mut entries: Vec<PathBuf> = std::fs::read_dir(&snapshots_dir)
        .unwrap_or_else(|e| panic!("read_dir {}: {e}", snapshots_dir.display()))
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("snap"))
        .collect();
    entries.sort();

    let mut problems = Vec::new();
    for snap_path in entries {
        let stem = snap_path
            .file_stem()
            .and_then(|s| s.to_str())
            .expect("snapshot filename must be utf-8")
            .to_string();

        let snap_bytes = std::fs::read(&snap_path).unwrap_or_else(|e| panic!("read snap: {e}"));
        let actual = format!("{:x}", Sha256::digest(&snap_bytes));

        let review_path = reviews_dir.join(format!("{stem}.md"));
        if !review_path.exists() {
            problems.push(format!(
                "{stem}: no review at {}",
                review_path.display()
            ));
            continue;
        }
        let review = std::fs::read_to_string(&review_path)
            .unwrap_or_else(|e| panic!("read review {}: {e}", review_path.display()));
        let expected = parse_snapshot_hash(&review).unwrap_or_else(|| {
            panic!(
                "{stem}: review {} has no `snapshot_hash:` frontmatter",
                review_path.display()
            )
        });
        if expected != actual {
            problems.push(format!(
                "{stem}: review hash {expected} != snapshot hash {actual}"
            ));
        }
    }

    if !problems.is_empty() {
        panic!(
            "Stale reviews — re-run alignment-reviewer for each listed snapshot:\n  {}",
            problems.join("\n  ")
        );
    }
}

/// Extract `snapshot_hash: <hex>` from a review's leading YAML frontmatter.
/// Matches the format written by the `alignment-reviewer` agent.
fn parse_snapshot_hash(review: &str) -> Option<String> {
    let mut lines = review.lines();
    if lines.next()?.trim() != "---" {
        return None;
    }
    for line in lines {
        let line = line.trim();
        if line == "---" {
            return None;
        }
        if let Some(rest) = line.strip_prefix("snapshot_hash:") {
            return Some(rest.trim().to_string());
        }
    }
    None
}
