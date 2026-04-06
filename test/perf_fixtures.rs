// Large repos for performance benchmarking.
//
// Includers must define:
//   with_perf_fixtures!(($dir, $url, $tag), ...) — called with all perf fixture repos
//
// These are pinned to tags (not commit hashes) so shallow clone works:
//   git clone --depth 1 --branch <tag> <url> <target>
//
// Stored in test/perf-fixtures/ (separate from test/fixtures/).

with_perf_fixtures! {
    ("typescript",  "https://github.com/microsoft/TypeScript.git",  "v5.6.2"),
    ("deno",        "https://github.com/denoland/deno.git",         "v2.0.0"),
    ("cpython",     "https://github.com/python/cpython.git",        "v3.13.0"),
    ("vscode",      "https://github.com/microsoft/vscode.git",      "1.95.0"),
    ("django",      "https://github.com/django/django.git",         "5.1"),
}
