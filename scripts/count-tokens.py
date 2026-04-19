#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.11"
# dependencies = ["tiktoken"]
# ///
"""Count tokens in files, line ranges, or arbitrary text using o200k_base.

Usage:
  count-tokens.py <spec> [<spec> ...] [--regex PATTERN]
  count-tokens.py --stdin

Each <spec> is one of:
  <path>                       whole file
  <path>:<start>-<end>         inclusive line range, 1-indexed
  <path>:<line>                a single line

With --regex PATTERN: for each line, anchored-match PATTERN against the line.
If a prefix matches and there's content after it, the line is rendered as that
prefix + "…" (a single ellipsis char) and counted accordingly. If no match,
the line is counted in full. Use this to model honest line-prefix truncation
at syntactic boundaries (e.g., `--regex '^[^{]*'` to truncate at the first `{`,
or `--regex '^[^(]+\\('` to truncate after the first `(`).

With --stdin: count tokens of arbitrary text read from standard input. Use
this for folder listings, file lists, or any rendered text whose token cost
you want to estimate (e.g., `ls src/ | count-tokens.py --stdin`).

Prints per-spec counts and a final total.
"""

import re
import sys
import tiktoken


def parse_spec(spec):
    if ":" in spec:
        path, range_part = spec.rsplit(":", 1)
        if "-" in range_part:
            start_s, end_s = range_part.split("-", 1)
            return path, int(start_s), int(end_s)
        line = int(range_part)
        return path, line, line
    return spec, None, None


def read_lines(path, start, end):
    with open(path, "r", errors="replace") as f:
        lines = f.readlines()
    if start is None:
        return "".join(lines)
    return "".join(lines[start - 1 : end])


def apply_regex(text, pattern):
    pat = re.compile(pattern)
    out = []
    for line in text.splitlines(keepends=True):
        nl = ""
        body = line
        if body.endswith("\n"):
            body, nl = body[:-1], "\n"
        m = pat.match(body)
        if m and m.end() < len(body):
            out.append(m.group(0) + "…" + nl)
        else:
            out.append(line)
    return "".join(out)


def main():
    args = list(sys.argv[1:])
    enc = tiktoken.get_encoding("o200k_base")

    if "--stdin" in args:
        text = sys.stdin.read()
        n = len(enc.encode(text))
        print(f"stdin: {n}")
        return

    regex = None
    if "--regex" in args:
        i = args.index("--regex")
        regex = args[i + 1]
        args = args[:i] + args[i + 2 :]
    if not args:
        sys.stderr.write(__doc__)
        sys.exit(2)

    total = 0
    for spec in args:
        path, start, end = parse_spec(spec)
        text = read_lines(path, start, end)
        if regex is not None:
            text = apply_regex(text, regex)
        n = len(enc.encode(text))
        total += n
        print(f"{spec}: {n}")
    print(f"total: {total}")


if __name__ == "__main__":
    main()
