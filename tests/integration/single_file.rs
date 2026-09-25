//! A file path summarizes that one file: the same walkers that render it
//! inside a directory, with the file's own row as the header and none of
//! its siblings, then as much of the rest of the file as the budget
//! leaves room for.

use std::path::Path;

fn write(dir: &Path, name: &str, contents: &str) -> std::path::PathBuf {
    let path = dir.join(name);
    std::fs::write(&path, contents).unwrap();
    path
}

fn render(path: &Path, token_budget: usize, char_budget: Option<usize>) -> String {
    precis::render(path, token_budget, char_budget).expect("render failed")
}

#[test]
fn single_file_python_renders_only_that_file() {
    let temp = tempfile::tempdir().unwrap();
    let file = write(
        temp.path(),
        "app.py",
        "def main():\n    return 1\n\n\nclass Runner:\n    pass\n",
    );
    write(temp.path(), "sibling.py", "def sibling():\n    pass\n");
    std::fs::create_dir(temp.path().join("pkg")).unwrap();

    let out = render(&file, 3000, None);

    assert!(out.starts_with("app.py\n"), "{out}");
    assert!(out.contains("1→def main():"), "{out}");
    assert!(out.contains("class Runner:"), "{out}");
    for sibling in ["sibling", "pkg", "…"] {
        assert!(!out.contains(sibling), "`{sibling}` in:\n{out}");
    }
}

#[test]
fn single_file_markdown_renders_its_outline() {
    let temp = tempfile::tempdir().unwrap();
    let file = write(
        temp.path(),
        "guide.md",
        "# Guide\n\nHow to use it.\n\n## Install\n\nRun the installer.\n",
    );
    write(temp.path(), "other.md", "# Other\n");

    let out = render(&file, 3000, None);

    assert!(out.starts_with("guide.md\n"), "{out}");
    assert!(out.contains("1→# Guide"), "{out}");
    assert!(out.contains("## Install"), "{out}");
    assert!(!out.contains("other.md"), "{out}");
}

#[test]
fn single_file_rst_renders_its_outline() {
    let temp = tempfile::tempdir().unwrap();
    let file = write(
        temp.path(),
        "index.rst",
        "Guide\n=====\n\nHow to use it.\n\nInstall\n-------\n\nRun the installer.\n",
    );
    write(temp.path(), "other.rst", "Other\n=====\n");

    let out = render(&file, 3000, None);

    assert!(out.starts_with("index.rst\n"), "{out}");
    assert!(out.contains("1→Guide"), "{out}");
    assert!(out.contains("Install"), "{out}");
    assert!(!out.contains("other.rst"), "{out}");
}

#[test]
fn single_file_unsupported_extension_renders_its_content() {
    let temp = tempfile::tempdir().unwrap();
    let file = write(temp.path(), "notes.xyz", "alpha\nbeta\n");

    assert_eq!(
        render(&file, 3000, None),
        "notes.xyz\n  1→alpha\n  2→beta\n"
    );
}

#[test]
fn single_file_binary_renders_only_its_row() {
    let temp = tempfile::tempdir().unwrap();
    let file = write(temp.path(), "blob.xyz", "alpha\0beta\n");

    assert_eq!(render(&file, 3000, None), "blob.xyz\n");
}

/// A test file declares nothing, so no walker has a surface for it; the
/// budget goes to its head.
#[test]
fn single_file_without_declarations_renders_its_head() {
    let temp = tempfile::tempdir().unwrap();
    let source: String = (0..400)
        .map(|i| format!("assert(isOdd({}));\n", 2 * i + 1))
        .collect();
    let file = write(temp.path(), "test.js", &source);

    let token_budget = 500;
    let out = render(&file, token_budget, None);
    assert!(out.starts_with("test.js\n  1→assert(isOdd(1));\n"), "{out}");
    assert!(!out.contains("isOdd(799)"), "{out}");
    assert!(precis::tokenizer::count(&out) <= token_budget, "{out}");
}

/// Past what the walkers choose to show, spare budget renders the rest of
/// the file, so a file that fits renders whole.
#[test]
fn single_file_that_fits_renders_whole() {
    let temp = tempfile::tempdir().unwrap();
    let source = "import os\n\nLIMIT = 3\n\n\ndef main():\n    return os.getcwd()\n\n\nmain()\n";
    let file = write(temp.path(), "app.py", source);

    let out = render(&file, 3000, None);
    for (index, line) in source.lines().enumerate() {
        if !line.is_empty() {
            assert!(out.contains(&format!("{}→{line}\n", index + 1)), "{out}");
        }
    }
    assert!(!out.contains('…'), "{out}");
}

#[test]
fn single_file_larger_than_the_budget_stays_within_it() {
    let temp = tempfile::tempdir().unwrap();
    let source: String = (0..400)
        .map(|i| format!("def function_{i}(argument):\n    return argument + {i}\n\n\n"))
        .collect();
    let file = write(temp.path(), "big.py", &source);

    let token_budget = 1000;
    let out = render(&file, token_budget, None);
    assert!(out.starts_with("big.py\n"), "{out}");
    assert!(out.contains('…'), "{out}");
    assert!(precis::tokenizer::count(&out) <= token_budget, "{out}");

    let char_budget = 2000;
    let capped = render(&file, token_budget, Some(char_budget));
    assert!(capped.contains("1→def function_0"), "{capped}");
    assert!(precis::char_units(&capped) <= char_budget, "{capped}");
}

#[test]
fn single_file_path_errors_when_missing() {
    let temp = tempfile::tempdir().unwrap();
    assert!(precis::render(&temp.path().join("absent.rs"), 3000, None).is_err());
}

#[cfg(unix)]
#[test]
fn single_file_path_errors_on_a_fifo_instead_of_reading_it() {
    let temp = tempfile::tempdir().unwrap();
    let fifo = temp.path().join("events.txt");
    let status = std::process::Command::new("mkfifo")
        .arg(&fifo)
        .status()
        .unwrap();
    assert!(status.success());

    let error = precis::render(&fifo, 3000, None).unwrap_err();
    assert!(error.to_string().contains("regular file"), "{error}");
}
