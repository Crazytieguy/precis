//! Walker trait + shared context.
//!
//! A walker emits [`Batch`] units containing the key, optional
//! predecessor edge, fully-built `BatchContent`, and a scalar `value`. The
//! scheduler ranks emitted batches by `value / cost^k`, gates by
//! predecessor scheduling, and applies content to the rendered tree.
//! Source text is cached on [`WalkCtx`]; parse trees are not.

use std::cell::{Cell, OnceCell};
use std::collections::{BTreeSet, HashMap};
use std::ops::Range;
use std::panic::AssertUnwindSafe;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::{Arc, Mutex, mpsc};

use tree_sitter::{Language, Node, Tree};

use crate::batch::{Batch, BatchKey};
use crate::content::{BatchContent, Render, Span};
use crate::fs_util::{DirFilter, lists_file};
use crate::render::{MAX_SOURCE_BYTES, Source, SourceCache};

pub(crate) mod code;
mod fs;
mod go_mod;
mod json;
mod markdown;
mod plaintext;
mod prisma;
mod survey;
mod toml;
mod workspace;

use survey::EssentialSource;
pub(in crate::walker) use survey::language_group;

/// Greedily partition source-ordered items once a range reaches `target`.
/// A final range cheaper than `min_tail` folds into its predecessor.
/// `split_after` uses exclusive item indices, allowing callers to
/// preserve structural cut boundaries.
pub(super) fn budget_chunk_ranges(
    item_count: usize,
    cost: impl Fn(Range<usize>) -> usize,
    target: usize,
    min_tail: usize,
    split_after: impl Fn(usize) -> bool,
) -> Vec<Range<usize>> {
    let mut ranges = Vec::new();
    let mut start = 0;
    for end in 1..item_count {
        if split_after(end) && cost(start..end) >= target {
            ranges.push(start..end);
            start = end;
        }
    }
    if start < item_count {
        let tail = start..item_count;
        if cost(tail.clone()) < min_tail
            && let Some(previous) = ranges.last_mut()
        {
            previous.end = tail.end;
        } else {
            ranges.push(tail);
        }
    }
    ranges
}

/// Walker contract. [`FsWalker`] is the production walker; tests drive
/// the scheduler with stub walkers.
pub trait Walker {
    /// Initial batches before any scheduling decision (typically the
    /// root FS listing).
    fn seed(&mut self, ctx: &WalkCtx) -> Vec<Batch>;

    /// Called when a batch is scheduled — returns newly-discovered
    /// batches.
    fn expand(&mut self, scheduled: &BatchKey, ctx: &WalkCtx) -> Vec<Batch>;

    /// Called once, when every batch in `emitted` that can be scheduled
    /// is: batches that spend only the budget left over.
    fn floor(&mut self, _emitted: &[Batch], _ctx: &WalkCtx) -> Vec<Batch> {
        Vec::new()
    }
}

/// Top-level walker: FS listings drive discovery; per-language modules
/// own per-dir candidate emission and are dispatched here directly.
pub struct FsWalker;

impl Walker for FsWalker {
    fn seed(&mut self, ctx: &WalkCtx) -> Vec<Batch> {
        fs::seed(ctx)
    }

    fn expand(&mut self, scheduled: &BatchKey, ctx: &WalkCtx) -> Vec<Batch> {
        let BatchKey::Fs(key) = scheduled else {
            return Vec::new();
        };
        let (mut out, fully_listed) = fs::expand_listed(key, ctx);
        let Some(dir) = fully_listed else {
            return out;
        };
        out.extend(markdown::expand_in_dir(dir, ctx));
        out.extend(toml::expand_in_dir(dir, ctx));
        out.extend(json::expand_in_dir(dir, ctx));
        out.extend(plaintext::expand_in_dir(dir, ctx));
        out.extend(prisma::expand_in_dir(dir, ctx));
        out.extend(go_mod::expand_in_dir(dir, ctx));
        out.extend(code::expand_in_dir(dir, ctx));
        out
    }

    fn floor(&mut self, emitted: &[Batch], ctx: &WalkCtx) -> Vec<Batch> {
        plaintext::floor_batches(emitted, ctx)
    }
}

/// Per-run context: seed root, the listing filter and source cache shared
/// with the renderer, the run's parse budget, and each walker's own run
/// state.
pub struct WalkCtx {
    root: PathBuf,
    /// Built once per run — every listing and file enumeration in the
    /// walk goes through it, so an ignored subtree is invisible to
    /// discovery rather than filtered out downstream.
    dir_filter: Rc<DirFilter>,
    source_cache: SourceCache,
    /// What is left of [`RUN_PARSE_BYTE_CAP`].
    parse_bytes_left: Cell<usize>,
    cargo_workspace: workspace::WorkspaceMembership,
    fs_state: fs::FsState,
    json_state: json::JsonState,
    /// The tree's essential source, walked once; `None` past
    /// [`crate::fs_util::PROBE_ENTRY_CAP`].
    essential_source: OnceCell<Option<EssentialSource>>,
    /// The language family carrying the most essential source bytes.
    primary_language: OnceCell<Option<&'static str>>,
    /// The one source file that carries a dominant share of the tree's
    /// essential source bytes, if any.
    dominant_source_file: OnceCell<Option<PathBuf>>,
}

impl WalkCtx {
    pub fn new(root: PathBuf) -> Self {
        Self::with_cache(root, SourceCache::new())
    }

    pub fn with_cache(root: PathBuf, source_cache: SourceCache) -> Self {
        Self::with_filter(DirFilter::new(&root), source_cache)
    }

    /// Context for a walk scoped by `dir_filter`, rooted at its root.
    pub fn with_filter(dir_filter: DirFilter, source_cache: SourceCache) -> Self {
        Self {
            root: dir_filter.root().to_path_buf(),
            dir_filter: Rc::new(dir_filter),
            source_cache,
            parse_bytes_left: Cell::new(RUN_PARSE_BYTE_CAP),
            cargo_workspace: workspace::WorkspaceMembership::default(),
            fs_state: fs::FsState::default(),
            json_state: json::JsonState::default(),
            essential_source: OnceCell::new(),
            primary_language: OnceCell::new(),
            dominant_source_file: OnceCell::new(),
        }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Shared with the renderer, so cost probes reuse this run's ignore
    /// caches instead of rebuilding them per probe.
    pub fn dir_filter(&self) -> &Rc<DirFilter> {
        &self.dir_filter
    }

    pub fn source_cache(&self) -> &SourceCache {
        &self.source_cache
    }

    /// Depth of `path` relative to the seed root (root itself = 0).
    /// **Fails open**: a path outside the root also reads as depth 0 —
    /// the same as the root itself — which un-damps every depth-priced
    /// value. Callers handling possibly-out-of-root paths
    /// (canonicalized paths, workspace members) must check containment
    /// first.
    pub fn depth_from_root(&self, path: &Path) -> usize {
        path.strip_prefix(&self.root)
            .map(|p| p.components().count())
            .unwrap_or(0)
    }

    /// Non-essential discount, scoped to this run's root.
    pub fn non_essential_factor(&self, path: &Path) -> f64 {
        crate::value::non_essential_factor(path, &self.root)
    }

    /// The single source file the repository is *about*, when one
    /// exists: a mass-dominant essential source file that independently
    /// carries at least [`DOMINANT_SOURCE_MASS_SHARE`] of the tree's source
    /// bytes, the largest if several do. Single-implementation-file libraries (a
    /// one-file parser, a single amalgamated `.c`, a `core.py`) put the content
    /// an NS author wants in one place, and the scheduler's
    /// breadth-first instincts spend the marginal token elsewhere.
    /// Returns `None` when source mass is spread across peers, which is
    /// the common case.
    pub fn dominant_source_file(&self) -> Option<&Path> {
        self.dominant_source_file
            .get_or_init(|| {
                survey::find_dominant_source_file(
                    self.essential_source()?,
                    self.primary_language()?,
                    |path| self.read_source(path),
                )
            })
            .as_deref()
    }

    /// The language family ([`language_group`]) the repository is written
    /// in: the one carrying the most essential source bytes. `None` for a
    /// tie for the lead — "the language this repo is written in" has no
    /// answer there, and answering it by hasher seeding would make the
    /// output differ between processes on identical input.
    pub(crate) fn primary_language(&self) -> Option<&'static str> {
        *self
            .primary_language
            .get_or_init(|| survey::find_primary_language(self.essential_source()?))
    }

    fn essential_source(&self) -> Option<&EssentialSource> {
        self.essential_source
            .get_or_init(|| survey::enumerate_essential_source(&self.root, &self.dir_filter))
            .as_ref()
    }

    /// Read `path` into memory, caching the result. `None` unless its
    /// directory's listing admits it as a file, so a path a walker names
    /// without listing it (a workspace manifest, an `__init__.py`) is held
    /// to the listing's rules — and see [`SourceCache::get`].
    pub fn read_source(&self, path: &Path) -> Option<Arc<Source>> {
        lists_file(path, &self.dir_filter)
            .then(|| self.source_cache.get(path))
            .flatten()
    }

    /// Parse `path` with `language`. The tree is not cached: a file is
    /// parsed while its directory expands, and its batches carry
    /// everything they need from it.
    pub fn parse_tree(&self, path: &Path, language: &Language) -> Option<(Arc<Source>, Tree)> {
        self.parse_tree_prefix(path, language, str::len)
    }

    /// [`Self::parse_tree`] over only the first `prefix_len(source)` bytes
    /// of `path`, for a grammar that cannot survive some later input.
    pub fn parse_tree_prefix(
        &self,
        path: &Path,
        language: &Language,
        prefix_len: impl FnOnce(&str) -> usize,
    ) -> Option<(Arc<Source>, Tree)> {
        let source = self.read_for_parse(path)?;
        let prefix = &source.as_bytes()[..prefix_len(&source)];
        let tree = parser_for(language).parse(prefix, None)?;
        Some((source, tree))
    }

    /// `path`'s source for a parse, charged to the run's
    /// [`RUN_PARSE_BYTE_CAP`]. A file that doesn't fit what is left is not
    /// read, so directories expanded later in the run parse less.
    pub(in crate::walker) fn read_for_parse(&self, path: &Path) -> Option<Arc<Source>> {
        let left = self.parse_bytes_left.get();
        let source = gated_read_source(path, self, left)?;
        self.parse_bytes_left.set(left.saturating_sub(source.len()));
        Some(source)
    }

    /// `true` iff `file` is a Cargo workspace-member `Cargo.toml`.
    pub fn is_cargo_workspace_member(&self, file: &Path) -> bool {
        self.cargo_workspace
            .is_member(file, || toml::collect_workspace_members(self))
    }

    /// `true` iff `file` is a JS/TS workspace-member `package.json`.
    pub fn is_js_workspace_member(&self, file: &Path) -> bool {
        self.json_state.is_workspace_member(file, self)
    }

    /// `true` iff `file` is the unique primary JS/TS workspace member.
    pub fn is_primary_js_workspace_member(&self, file: &Path) -> bool {
        self.json_state.is_primary_workspace_member(file, self)
    }
}

fn parser_for(language: &Language) -> tree_sitter::Parser {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(language)
        .expect("tree-sitter language load");
    parser
}

/// Parse each of `files`, whose sources were read with
/// [`WalkCtx::read_for_parse`], handing each parse to `visit` in index
/// order. Files parse on one worker per core, and a file starts parsing
/// only while the sources of the parses not yet visited total at most
/// [`MAX_SOURCE_BYTES`] (or none are pending), so the trees alive at once
/// stay within what one capped file costs.
pub(in crate::walker) fn parse_each(
    files: &[(Arc<Source>, Language)],
    mut visit: impl FnMut(usize, Arc<Source>, Tree),
) {
    let source_len = |index: usize| files[index].0.len();
    let workers = std::thread::available_parallelism()
        .map_or(1, usize::from)
        .min(files.len());
    let (job_sender, job_receiver) = mpsc::channel::<usize>();
    let job_receiver = Mutex::new(job_receiver);
    let (parsed_sender, parsed_receiver) = mpsc::channel();
    std::thread::scope(|scope| {
        for _ in 0..workers {
            let parsed_sender = parsed_sender.clone();
            let job_receiver = &job_receiver;
            scope.spawn(move || {
                while let Ok(index) = job_receiver.lock().expect("parse job queue").recv() {
                    let parsed = std::panic::catch_unwind(AssertUnwindSafe(|| {
                        let (source, language) = &files[index];
                        parser_for(language).parse(source.as_bytes(), None)
                    }));
                    if parsed_sender.send((index, parsed)).is_err() {
                        return;
                    }
                }
            });
        }
        drop(parsed_sender);
        let mut next_job = 0;
        let mut pending_bytes = 0;
        let mut parsed_out_of_order = HashMap::new();
        for (index, (source, _)) in files.iter().enumerate() {
            while next_job < files.len()
                && (pending_bytes == 0 || pending_bytes + source_len(next_job) <= MAX_SOURCE_BYTES)
            {
                pending_bytes += source_len(next_job);
                job_sender
                    .send(next_job)
                    .expect("parse workers outlive the jobs");
                next_job += 1;
            }
            let parsed = loop {
                if let Some(parsed) = parsed_out_of_order.remove(&index) {
                    break parsed;
                }
                let (parsed_index, parsed) = parsed_receiver
                    .recv()
                    .expect("parse workers outlive the jobs");
                parsed_out_of_order.insert(parsed_index, parsed);
            };
            let tree = parsed.unwrap_or_else(|panic| std::panic::resume_unwind(panic));
            if let Some(tree) = tree {
                visit(index, source.clone(), tree);
            }
            pending_bytes -= source_len(index);
        }
        drop(job_sender);
    });
}

/// Most source bytes one run parses, over every file. Sources stay cached
/// for the whole run and a parse costs several times its source in memory
/// and time, so without it a directory of generated sources each under
/// [`MAX_SOURCE_BYTES`] runs to gigabytes. The 186-repo robustness sweep's
/// largest run parses 19.8 MB at an 8000-token budget; the training
/// corpus's at most 4.5 MB at 9000.
const RUN_PARSE_BYTE_CAP: usize = 32 * 1024 * 1024;

pub(in crate::walker) fn first_child_of_kind<'a>(node: Node<'a>, kind: &str) -> Option<Node<'a>> {
    let mut cursor = node.walk();
    node.children(&mut cursor)
        .find(|child| child.kind() == kind)
}

/// Path-relative location prior: depth penalty × non-essential-dir
/// discount. Use [`file_depth_factor`] to add entrypoint pinning.
pub(crate) fn path_depth_factor(file: &Path, ctx: &WalkCtx) -> f64 {
    file_depth_factor(file, ctx, false)
}

/// [`path_depth_factor`] with optional entrypoint pinning — when
/// `is_entrypoint`, depth clamps to 1 so an `index.ts` at any depth
/// ranks like depth 1. Non-essential discount still applies.
pub(crate) fn file_depth_factor(file: &Path, ctx: &WalkCtx, is_entrypoint: bool) -> f64 {
    let depth = ctx.depth_from_root(file);
    let pinned_depth = if is_entrypoint { depth.min(1) } else { depth };
    crate::value::depth_factor(pinned_depth) * ctx.non_essential_factor(file)
}

/// `BatchContent::Lines` rendering `rows` of one file in full. `None`
/// if the resulting span set is empty.
pub(crate) fn single_file_lines_content(
    path: &Path,
    source: &Source,
    rows: Vec<usize>,
) -> Option<BatchContent> {
    let spans = build_file_spans(path, source, rows);
    if spans.is_empty() {
        return None;
    }
    Some(BatchContent::Lines {
        spans,
        units: Vec::new(),
    })
}

/// Whether precis refuses to show any of `text`, read from `path`: a
/// credential file, by its own name or its link target's
/// ([`plaintext::is_credential_name`]), text holding a private key, or a
/// link to a file in its own directory (`CLAUDE.md -> AGENTS.md`), whose
/// text shows under the target's row. [`SourceCache`] applies it to
/// everything it holds, so no walker can show such a file.
pub(crate) fn is_refused(path: &Path, text: &str) -> bool {
    is_refused_by_name(path) || holds_private_key(text)
}

/// The half of [`is_refused`] that needs no read.
pub(crate) fn is_refused_by_name(path: &Path) -> bool {
    plaintext::is_credential_name(path)
        || path.canonicalize().is_ok_and(|target| {
            plaintext::is_credential_name(&target)
                || (target != path && target.parent() == path.parent())
        })
}

/// A PEM or PGP private-key block: an armor header naming a private key,
/// then key material — a base64 run at least one armor line (64
/// characters, RFC 7468 and RFC 4880) long — then the closing armor. A
/// header alone (a parser's constant, a documented placeholder) holds
/// no key, whatever long token (a SHA-256 hex digest) follows it. A block
/// whose closing armor is missing (a head read cut short of it) counts
/// when its first line past the armor headers is a whole base64 armor
/// line.
pub(crate) fn holds_private_key(text: &str) -> bool {
    let is_base64 = |ch: char| ch.is_ascii_alphanumeric() || ch == '+' || ch == '/';
    text.split("-----BEGIN ").skip(1).any(|block| {
        let Some((label, body)) = block.split_once("-----") else {
            return false;
        };
        if !label.contains("PRIVATE KEY") {
            return false;
        }
        match body.split_once("-----END ") {
            Some((body, _)) => body
                .split(|ch: char| !is_base64(ch))
                .any(|run| run.len() >= 64),
            None => body
                .lines()
                .skip(1)
                .map(str::trim)
                .find(|line| !line.is_empty() && !line.contains(':'))
                .is_some_and(|line| {
                    line.len() >= 64 && line.chars().all(|ch| is_base64(ch) || ch == '=')
                }),
        }
    })
}

/// Cached read behind an FS-metadata byte pre-flight — skips the read
/// (and returns `None`) when the size hint alone disqualifies the
/// file. Bytes-per-line multipliers are per-format — callers keep
/// their own gate constants.
pub(crate) fn gated_read_source(
    file: &Path,
    ctx: &WalkCtx,
    byte_gate: usize,
) -> Option<Arc<Source>> {
    if std::fs::metadata(file).is_ok_and(|m| m.len() as usize > byte_gate) {
        return None;
    }
    ctx.read_source(file)
}

/// Whole-file content behind a size gate: [`gated_read_source`] byte
/// pre-flight, then a line cap on the read source.
pub(crate) fn gated_whole_file_content(
    file: &Path,
    ctx: &WalkCtx,
    byte_gate: usize,
    line_cap: usize,
) -> Option<BatchContent> {
    let source = gated_read_source(file, ctx, byte_gate)?;
    if source.line_count() > line_cap {
        return None;
    }
    single_file_lines_content(file, &source, (1..=source.line_count()).collect())
}

/// Rows (any order, duplicates allowed) → contiguous full-line [`Span`]
/// ranges. Blank source lines are dropped at span edges but **preserved
/// when interior**: a gap between two kept rows that consists solely of
/// blank source rows is bridged into one span, so the emitted region
/// mirrors the source's shape (NS spans are contiguous ranges that
/// include interior blanks). Walkers emit no `…` rows of their own; the
/// renderer marks every elided non-blank gap.
pub(crate) fn build_file_spans(path: &Path, source: &Source, rows: Vec<usize>) -> Vec<Span> {
    let blank = |n: usize| source.line(n).is_some_and(|t| t.trim().is_empty());
    let full: BTreeSet<usize> = rows
        .into_iter()
        .filter(|&n| source.line(n).is_some_and(|t| !t.trim().is_empty()))
        .collect();

    let mut spans = Vec::new();
    // Merge runs of Full line numbers into single-range spans. An
    // all-blank gap is an empty or bridgeable range, so one condition
    // covers both adjacency and interior-blank bridging.
    let full_vec: Vec<usize> = full.iter().copied().collect();
    let mut i = 0;
    while i < full_vec.len() {
        let start = full_vec[i];
        let mut end = start;
        while i + 1 < full_vec.len() && (end + 1..full_vec[i + 1]).all(blank) {
            end = full_vec[i + 1];
            i += 1;
        }
        spans.push(Span {
            path: path.to_path_buf(),
            start,
            end,
            render: Render::Full,
        });
        i += 1;
    }
    spans
}

// --- shared tree-sitter span helpers ---

/// 0-based final row covered by `node`, ignoring trailing whitespace.
/// Trimming all whitespace (not just newlines) matters for grammars
/// whose block nodes swallow the next sibling's leading indentation
/// (tree-sitter-markdown list items): a final row the node covers only
/// with whitespace renders nothing and must not be claimed — sibling
/// batches each claiming it is a walker-contract overlap.
pub(crate) fn node_end_row_trimmed(node: Node, source: &str) -> usize {
    let text = &source[node.start_byte()..node.end_byte()];
    let newlines = text
        .trim_end()
        .bytes()
        .filter(|&byte| byte == b'\n')
        .count();
    node.start_position().row + newlines
}

#[cfg(test)]
mod tests {
    use super::*;

    fn span_shapes(source: &str, rows: Vec<usize>) -> Vec<(usize, usize, Render)> {
        build_file_spans(Path::new("f.txt"), &Source::new(source.into()), rows)
            .into_iter()
            .map(|s| (s.start, s.end, s.render))
            .collect()
    }

    #[test]
    fn walker_mod_parses_stop_at_the_run_byte_cap() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path();
        let paths: Vec<PathBuf> = ["a.rs", "b.rs", "c.rs", "d.rs"]
            .iter()
            .map(|name| root.join(name))
            .collect();
        let bytes = [
            "fn a() {}\n",
            "fn big() { let x = 1; }\n",
            "fn c() {}\n",
            "fn d() {}\n",
        ];
        for (path, text) in paths.iter().zip(bytes) {
            std::fs::write(path, text).unwrap();
        }
        let ctx = WalkCtx::new(root.to_path_buf());
        ctx.parse_bytes_left
            .set(bytes[0].len() + bytes[2].len() + 1);
        let admitted: Vec<bool> = paths[..3]
            .iter()
            .map(|path| ctx.read_for_parse(path).is_some())
            .collect();
        assert_eq!(admitted, [true, false, true]);
        let language: Language = tree_sitter_rust::LANGUAGE.into();
        assert!(ctx.parse_tree(&paths[3], &language).is_none());
    }

    #[test]
    fn walker_mod_private_keys_need_key_material_after_their_armor() {
        let body = "MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQC7VJTUt9Us8cKj";
        for text in [
            format!("-----BEGIN RSA PRIVATE KEY-----\n{body}\n-----END RSA PRIVATE KEY-----\n"),
            format!(
                "const KEY = \"-----BEGIN EC PRIVATE KEY-----\\n{body}\\n-----END EC PRIVATE KEY-----\";\n"
            ),
            format!(
                "-----BEGIN PGP PRIVATE KEY BLOCK-----\n\n{body}\n=ab12\n-----END PGP PRIVATE KEY BLOCK-----\n"
            ),
            format!(
                "-----BEGIN RSA PRIVATE KEY-----\n{body}\n{body}\n{}",
                &body[..20]
            ),
            format!(
                "-----BEGIN PGP PRIVATE KEY BLOCK-----\nVersion: GnuPG v2\n\n{body}\n{}",
                &body[..20]
            ),
        ] {
            assert!(holds_private_key(&text), "{text}");
        }
        for text in [
            "Paste your PRIVATE KEY into the settings page.\n".to_string(),
            "-----BEGIN PRIVATE KEY-----\n<your key here>\n-----END PRIVATE KEY-----\n".to_string(),
            "if line == \"-----BEGIN RSA PRIVATE KEY-----\" {\n    parse(line)\n}\n".to_string(),
            format!("-----BEGIN CERTIFICATE-----\n{body}\n-----END CERTIFICATE-----\n"),
            format!("-----BEGIN CERTIFICATE-----\n{body}\n{}", &body[..20]),
            "-----BEGIN PRIVATE KEY-----\n<your key here>\n".to_string(),
            format!(
                "const HEADER = \"-----BEGIN RSA PRIVATE KEY-----\";\nconst EMPTY_SHA256 = \"{}\";\n",
                "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
            ),
        ] {
            assert!(!holds_private_key(&text), "{text}");
        }
    }

    #[test]
    fn walker_mod_build_file_spans_bridges_interior_blank_gaps() {
        // Rows 2 and 4 are blank; collecting 1/3/5 must yield one span
        // covering the whole region, blanks included.
        let source = "a\n\nb\n\nc\n";
        let shapes = span_shapes(source, vec![1, 3, 5]);
        assert_eq!(shapes, vec![(1, 5, Render::Full)]);
    }

    #[test]
    fn walker_mod_build_file_spans_drops_edge_blanks_and_content_gaps() {
        // Blank rows at the edges of the collected set never render, and
        // a gap containing an uncollected *content* row is not bridged.
        let source = "\na\nskipped\nb\n\n";
        let shapes = span_shapes(source, vec![1, 2, 4, 5]);
        assert_eq!(shapes, vec![(2, 2, Render::Full), (4, 4, Render::Full)]);
    }
}
