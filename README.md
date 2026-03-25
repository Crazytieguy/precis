# precis

A CLI tool that extracts a token-efficient summary of a path, designed to give AI coding agents fast structural context without reading every file.

## Claude Code plugin (recommended)

The precis plugin automatically injects a structural overview of your project into Claude's context at the start of every session. This eliminates the need for long, manually-maintained `CLAUDE.md` files describing your codebase, and removes the overhead of Explore agents. The plugin also gives Claude the `precis` CLI so it can zoom into specific directories on demand.

```
claude plugin marketplace add Crazytieguy/precis
claude plugin install precis
```

Or add to your `.claude/settings.json` manually:

```json
{
  "enabledPlugins": {
    "precis@precis": true
  },
  "extraKnownMarketplaces": {
    "precis": {
      "source": {
        "source": "github",
        "repo": "Crazytieguy/precis"
      }
    }
  }
}
```

The plugin automatically downloads and updates the binary — no manual install needed.

## Standalone CLI

```
cargo install precis
```

Or with Homebrew:

```
brew install Crazytieguy/tap/precis
```

Or with the install script:

```
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/Crazytieguy/precis/releases/latest/download/precis-installer.sh | sh
```

### Configure your AI agent

Add this to your `CLAUDE.md`, `AGENTS.md`, or equivalent:

```markdown
## Codebase exploration

Always use `precis` for codebase exploration. Run `precis .` for a full overview, or `precis src/some/directory` to zoom into a specific area.
```

## Usage

```
precis .                    # summarize the current directory
precis ./src                # zoom into a subdirectory
precis . --budget 8000      # with a larger token budget
```

The default budget is 4000 BPE tokens (o200k_base tokenizer). Output is plain text with line numbers preserving source indentation.

## Design principles

**Goal.** Maximize a reader's understanding of a codebase per token spent. The reader starts knowing nothing; the output should build the most accurate mental model possible within the budget.

**Don't confuse the reader.** The output inevitably makes implicit claims. Showing 3 of 10 files in a directory implies the other 7 don't matter. Showing one symbol before another implies a ranking. Showing a subset of a group implies they were chosen for a reason. If any of these implications would lead the reader to an incorrect conclusion, the output has a bug. This is testable: show an agent the output, check what it infers, verify whether those inferences are correct. Mechanisms that serve this principle include the source-line constraint (output is always a prefix of actual source lines, never synthesized), making omissions visible, and grouping symbols that can't be meaningfully distinguished.

**Grounded prioritization.** Every value judgment must correspond to a real, articulable difference. If two things would get identical scores, treat them identically — show both or neither. Filling budget with content the tool can't genuinely rank is worse than leaving budget unused, because ungrounded rankings confuse the reader. Proxy metrics like budget utilization and symbol count are particularly dangerous — they reward showing *more* without regard for whether the reader is better served.

**Improvement process.** Look at real output for real projects. Compare to what a knowledgeable human would choose to show. The gap between those is the work. When output changes, read the diffs as a user would — check for regressions in understanding, not just changes in content. The only test that matters is: does this output build a better mental model than the alternative?

## Supported languages

- **Rust** — functions, structs, enums, traits, impls, type aliases, consts, statics, macros, modules
- **TypeScript / JavaScript / TSX** — functions, classes, interfaces, enums, type aliases, consts, namespaces
- **Go** — functions, methods, structs, interfaces, type aliases, consts, vars
- **C** — functions, structs, unions, enums, typedefs, macros, includes
- **Python** — functions, classes, module-level constants
- **Markdown** — heading structure with body content
- **JSON / TOML / YAML** — top-level keys and sections
- **Lua** — functions, module-level variables
- **Plain text fallback** — files in any other language are included as plain text (binary files excluded automatically)
