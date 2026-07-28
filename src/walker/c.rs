//! C / C-header walker. Per-decl batches plus file-scope banner /
//! includes / decl-name surface.
//!
//! Per-file keys:
//! - `HeaderBanner { file }`: top-of-file `/* */` block (license / brief)
//! - `Includes { file }`: `#include` directives, plus any top-level
//!   `#if`/`#ifdef` blocks whose body is exclusively directive content
//!   and contains at least one `#include` (the conditional include-map
//!   idiom: `#if CFG_TUH_HID { #include "class/hid/hid_host.h" }`).
//!   Conditional blocks that wrap real code are opaque.
//! - `DeclNames { file, chunk_index }`: surface listing of top-level
//!   declaration first lines — catastrophic-omission hedge. The file's
//!   external- and internal-linkage decls form two independent chunk
//!   series under this one key (see [`DeclLinkage`], [`names_chunks`]).
//!
//! Per-decl keys (keyed by start line so each decl has a distinct batch):
//! - `Decl { file, start_line }`: one top-level public declaration's
//!   signature/header. For function definitions, signature with body
//!   marker. For typedefs, prototypes, structs, enums, `#define`s, the
//!   whole statement.
//! - `DeclDoc { file, start_line }`: doc comment block immediately above
//!   a decl. Predecessor: matching `Decl`.
//! - `DeclBody { file, start_line }`: body interior of a function
//!   definition. Predecessor: matching `Decl`.
//! - `AggregateMemberGroup { file, start_line, group_start_line }`:
//!   per-field-group split for big struct/union bodies (≥3 blank-line
//!   groups) and per-chunk split for big enum bodies
//!   (≥`AGGREGATE_ENUM_CHUNK_MIN` enumerators). Predecessor: matching
//!   `Decl`. When emitted, the parent `Decl` covers only the type
//!   header + closing brace so the two render disjoint body rows.
//!
//! Public-vs-private rule:
//! - `.c` files: top-level `static` items are surfaced when the project
//!   builds a program of its own rather than a library — see
//!   [`admits_internal_decls`] — and excluded otherwise.
//! - `.h` files: `static inline` definitions are included (header-only
//!   inline accessors are part of the public API expansion); other
//!   `static` items are excluded.
//! - The wrapping `#ifndef X` / `#define X` / `#endif` header guard
//!   (recognized structurally — first `#ifndef` whose name is then
//!   `#define`d on the next line, regardless of naming convention) is
//!   descended into transparently, and its `#define X` is envelope
//!   rather than a macro decl — see [`is_header_guard_define`].
//!   `extern "C" { ... }` linkage specs
//!   (including the `#ifdef __cplusplus` wrapper idiom common in C
//!   headers) are likewise descended through so the wrapped decls are
//!   visited as top-level. Other `preproc_if` / `preproc_ifdef` blocks
//!   are opaque — anything they wrap is invisible to the walker.
//!
//! Parse trees are cached in [`WalkCtx`].

use std::cell::{OnceCell, RefCell};
use std::collections::{HashMap, HashSet};
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use tree_sitter::{Node, Tree};

use crate::batch::{Batch, BatchKey, CKey};
use crate::content::BatchContent;
use crate::fs_util::DirFilter;
use crate::value::{mix_signals, names_surface_chunk_factor, roster_mass_factor};

/// Chunk size for C decl-name surfaces with no structural signal.
/// Larger than Python/TS's 12 because C headers regularly expose 50+
/// decls; 24 keeps mid-sized headers coherent while still splitting
/// catalog amalgamation headers.
const C_DECL_NAMES_CHUNK_SIZE: usize = 24;

/// Max non-blank source lines for a header to render whole in one
/// batch instead of decomposing into banner / includes / names-surface
/// / per-decl rows. Tuned to sit above the small single-module API
/// headers whose entire value is "here is the complete public surface
/// plus its constants / `#ifdef` shape" (bareiron's varnum.h 12,
/// serialize.h 17, worldgen.h 24 non-blank) and below mid-sized
/// catalog / multi-section headers the NS deliberately splits into
/// per-region listings (jq.h ~60, bareiron procedures.h / packets.h).
const WHOLE_HEADER_MAX_SRC_LINES: usize = 30;

/// Max number of small (`<= WHOLE_HEADER_MAX_SRC_LINES` non-blank)
/// headers a project may hold, tree-wide, for the whole-file render to
/// apply anywhere in it. Above this the project is a "many small
/// modules" layout (htop's ~88 per-meter / per-platform headers spread
/// across a dozen directories) where the catastrophic-omission hedge is
/// *breadth* — a cheap names-surface per header so the budget reaches
/// dozens of modules — not a full render of any one. A project with
/// only a handful of small headers (bareiron 5, krep 1, jq 9) is one
/// where each is a load-bearing module API that earns its full render.
/// Counted by size alone (no parse) — a coarse project-shape signal.
const WHOLE_HEADER_PROJECT_MAX: usize = 16;

/// Per-run C-walker state. Caches the seed root's autotools
/// `include_HEADERS` set so internal headers can be demoted.
#[derive(Default)]
pub(in crate::walker) struct CState {
    /// `Some(set)` iff `Makefile.am` declares at least one public header.
    /// `None` means no info — every header is treated equally.
    public_headers: OnceCell<Option<HashSet<PathBuf>>>,
    visibility_lookup: RefCell<HashMap<PathBuf, bool>>,
    /// Count of small headers tree-wide (by size), computed once. Gates
    /// the whole-file render off in many-small-modules projects.
    small_header_count: OnceCell<usize>,
    /// Directories that are members of a mirrored-sibling group — a set
    /// of `>= MIN_PORT_SIBLINGS` sibling subdirs sharing `>= MIN_PORT_SHARED_NAMES`
    /// common file names (per-OS / per-vendor platform ports). Their C
    /// content is a parallel reimplementation of one interface; the NS
    /// anchors on the shared root API, not N copies of it.
    port_dirs: OnceCell<HashSet<PathBuf>>,
    /// Per-basename `#include "X"` in-degree across the tree, computed
    /// once. Prices include-graph hubs above leaf headers.
    include_in_degree: OnceCell<IncludeInDegreeIndex>,
    /// One-pass facts about the project's non-test `.c` sources
    /// (computed once per run); `None` when the scan bailed out, which
    /// every derived signal treats as "no information".
    project_scan: OnceCell<Option<CProjectScan>>,
}

impl CState {
    /// Public-API header set for the seed root, computed once per run.
    /// Returns `None` when no `Makefile.am` with `include_HEADERS` is
    /// found — callers must treat that as "no information", not as
    /// "no headers are public".
    fn public_headers(&self, root: &Path) -> Option<&HashSet<PathBuf>> {
        self.public_headers
            .get_or_init(|| parse_include_headers(root))
            .as_ref()
    }

    /// `Some(true)` iff `header` is in the public-API set, `Some(false)`
    /// iff a public set exists but `header` isn't in it, `None` iff no
    /// public-API declaration is available. Canonicalize result is
    /// memoized per file.
    fn is_public_header(&self, header: &Path, root: &Path) -> Option<bool> {
        let public = self.public_headers(root)?;
        if let Some(&hit) = self.visibility_lookup.borrow().get(header) {
            return Some(hit);
        }
        let canonical = header.canonicalize();
        let target = canonical.as_deref().unwrap_or(header);
        let hit = public.contains(target);
        self.visibility_lookup
            .borrow_mut()
            .insert(header.to_path_buf(), hit);
        Some(hit)
    }

    /// True iff the project is small-header-sparse enough for the
    /// whole-file render — count of small headers tree-wide at or below
    /// [`WHOLE_HEADER_PROJECT_MAX`]. Computed once per run.
    fn whole_header_render_allowed(&self, root: &Path, filter: &DirFilter) -> bool {
        *self
            .small_header_count
            .get_or_init(|| count_small_headers(root, filter))
            <= WHOLE_HEADER_PROJECT_MAX
    }

    /// True iff `dir` is a member of a mirrored-sibling platform-port
    /// group (computed once per run).
    fn is_port_dir(&self, dir: &Path, root: &Path, filter: &DirFilter) -> bool {
        self.port_dirs
            .get_or_init(|| collect_port_dirs(root, filter))
            .contains(dir)
    }

    /// Include-graph in-degree index (computed once per run).
    fn include_in_degree(&self, root: &Path, filter: &DirFilter) -> &IncludeInDegreeIndex {
        self.include_in_degree
            .get_or_init(|| collect_include_in_degree(root, filter))
    }

    fn project_scan(&self, root: &Path, filter: &DirFilter) -> Option<&CProjectScan> {
        self.project_scan
            .get_or_init(|| scan_c_project(root, filter))
            .as_ref()
    }

    /// True iff the project builds a program of its own (computed once
    /// per run).
    fn builds_a_program(&self, root: &Path, filter: &DirFilter) -> bool {
        self.project_scan(root, filter)
            .is_some_and(|scan| scan.builds_a_program(root))
    }
}

/// Per-basename include-graph in-degree across the tree. Only targets
/// that exist as exactly one header in the tree are counted — a
/// basename with several copies (vendored per-board trees) or none
/// (uncloned submodule SDK headers) carries no attributable centrality.
#[derive(Default)]
struct IncludeInDegreeIndex {
    /// In-degree counting only includes *from header files* — the
    /// type/interface dependency graph. Cleanly ranks the OOP-backbone
    /// hubs (htop `Object.h`/`Process.h`) while leaving utility headers
    /// (`XUtils.h`, `Macros.h`) that only `.c` files pull in unboosted.
    header_to_header: HashMap<String, usize>,
    /// In-degree counting includes from every `.c`/`.h` file. Used for
    /// the leaf test — a header no other file includes is a leaf even
    /// when the header-to-header graph is silent about it.
    total: HashMap<String, usize>,
    max_header_to_header: usize,
    /// Headers seen tree-wide — the leaf damp only fires in projects
    /// with enough headers that breadth-pricing them all alike floods
    /// the budget.
    header_count: usize,
}

/// Hub boost activates only when the project's include graph has a real
/// spine (htop 29, tinyusb 32; flat single-header projects are ≤ 4).
const INCLUDE_HUB_MIN_MAX_IN_DEGREE: usize = 8;
/// Max boost at the top of the include graph.
const INCLUDE_HUB_BOOST: f64 = 0.6;
/// Damp for headers nothing else includes, in header-rich projects.
const INCLUDE_LEAF_FACTOR: f64 = 0.75;
/// Minimum tree-wide header count for the leaf damp.
const INCLUDE_LEAF_MIN_PROJECT_HEADERS: usize = 20;

/// Walk the tree counting `#include "X"` per basename. Same coarse
/// one-pass pattern (and cap) as [`count_small_headers`].
fn collect_include_in_degree(root: &Path, filter: &DirFilter) -> IncludeInDegreeIndex {
    const SCAN_CAP: usize = 4096;
    let mut c_files: Vec<PathBuf> = Vec::new();
    let mut header_copies: HashMap<String, usize> = HashMap::new();
    let mut stack = vec![root.to_path_buf()];
    let mut scanned = 0usize;
    while let Some(dir) = stack.pop() {
        let Ok(read_dir) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in read_dir.flatten() {
            scanned += 1;
            if scanned > SCAN_CAP {
                stack.clear();
                break;
            }
            let path = entry.path();
            let Ok(file_type) = entry.file_type() else {
                continue;
            };
            if filter.excludes(&path, file_type.is_dir()) {
                continue;
            }
            if file_type.is_dir() {
                if !crate::fs_util::should_skip_dir(&entry.file_name().to_string_lossy()) {
                    stack.push(path);
                }
            } else if file_type.is_file()
                && let Some(name) = path.file_name().and_then(|n| n.to_str())
                && is_c_source_file_name(name)
            {
                if is_header_file(&path) {
                    *header_copies.entry(name.to_string()).or_insert(0) += 1;
                }
                c_files.push(path);
            }
        }
    }
    let mut index = IncludeInDegreeIndex {
        header_count: header_copies.values().sum(),
        ..Default::default()
    };
    for file in &c_files {
        let Ok(text) = std::fs::read_to_string(file) else {
            continue;
        };
        let from_header = is_header_file(file);
        for line in text.lines() {
            let Some(target) = quoted_include_basename(line) else {
                continue;
            };
            if header_copies.get(target) != Some(&1) {
                continue;
            }
            *index.total.entry(target.to_string()).or_insert(0) += 1;
            if from_header {
                *index
                    .header_to_header
                    .entry(target.to_string())
                    .or_insert(0) += 1;
            }
        }
    }
    index.max_header_to_header = index.header_to_header.values().copied().max().unwrap_or(0);
    index
}

/// Basename of a `#include "X"` target, or `None` for non-include lines
/// and angle-bracket (system) includes.
fn quoted_include_basename(line: &str) -> Option<&str> {
    let rest = line.trim_start().strip_prefix('#')?.trim_start();
    let rest = rest.strip_prefix("include")?.trim_start();
    let rest = rest.strip_prefix('"')?;
    let path = &rest[..rest.find('"')?];
    Some(path.rsplit('/').next().unwrap_or(path))
}

/// Include-graph centrality factor for a header's decl-tier batches.
/// Hubs of the header-to-header graph get a bounded log boost (up to
/// `1 + INCLUDE_HUB_BOOST`); headers nothing includes get a mild damp
/// in header-rich projects. `.c` files and projects without an include
/// spine are neutral by construction.
fn include_centrality_factor(file: &Path, ctx: &WalkCtx) -> f64 {
    if !is_header_file(file) {
        return 1.0;
    }
    let Some(name) = file.file_name().and_then(|n| n.to_str()) else {
        return 1.0;
    };
    let index = ctx
        .c_state()
        .include_in_degree(ctx.root(), ctx.dir_filter());
    let mut factor = 1.0;
    if index.max_header_to_header >= INCLUDE_HUB_MIN_MAX_IN_DEGREE
        && let Some(&in_degree) = index.header_to_header.get(name)
    {
        factor *= 1.0
            + INCLUDE_HUB_BOOST * ((1 + in_degree) as f64).ln()
                / ((1 + index.max_header_to_header) as f64).ln();
    }
    if index.header_count >= INCLUDE_LEAF_MIN_PROJECT_HEADERS
        && index.total.get(name).copied().unwrap_or(0) <= 1
    {
        factor *= INCLUDE_LEAF_FACTOR;
    }
    factor
}

/// True iff `file` is a top-tier hub of the header-to-header include
/// graph: the spine is active and the header's in-degree is in the top
/// half of the range. In htop this selects exactly the OOP backbone
/// (Object / Hashtable / Machine / Meter / Process / Panel, h2h 24-29)
/// while the wide per-meter tier (h2h 8-9) stays out.
fn is_top_include_hub(file: &Path, ctx: &WalkCtx) -> bool {
    if !is_header_file(file) {
        return false;
    }
    let Some(name) = file.file_name().and_then(|n| n.to_str()) else {
        return false;
    };
    let index = ctx
        .c_state()
        .include_in_degree(ctx.root(), ctx.dir_filter());
    index.max_header_to_header >= INCLUDE_HUB_MIN_MAX_IN_DEGREE
        && index.header_to_header.get(name).copied().unwrap_or(0) * 2 >= index.max_header_to_header
}

/// Minimum sibling subdirs and shared file names for a directory group to
/// count as mirrored platform ports.
const MIN_PORT_SIBLINGS: usize = 3;
const MIN_PORT_SHARED_NAMES: usize = 2;

/// Walk the tree and collect directories that are members of a
/// mirrored-sibling group: a parent with `>= MIN_PORT_SIBLINGS` subdirs
/// where `>= MIN_PORT_SHARED_NAMES` file names each appear in at least
/// `MIN_PORT_SIBLINGS` of those subdirs (the htop `darwin/ freebsd/ …`
/// per-OS pattern, tinyusb `src/portable/<vendor>/`). Only subdirs that
/// actually carry a shared name are marked, so shared-helper siblings
/// (`generic/`, `zfs/`) that don't mirror the skeleton keep full weight.
/// Size-capped; symlinks not followed.
fn collect_port_dirs(root: &Path, filter: &DirFilter) -> HashSet<PathBuf> {
    const SCAN_CAP: usize = 4096;
    let mut ports = HashSet::new();
    let mut stack = vec![root.to_path_buf()];
    let mut scanned = 0usize;
    while let Some(parent) = stack.pop() {
        let Ok(read_dir) = std::fs::read_dir(&parent) else {
            continue;
        };
        let subdirs: Vec<PathBuf> = read_dir
            .flatten()
            .filter(|e| e.file_type().map(|t| t.is_dir()).unwrap_or(false))
            .filter(|e| !crate::fs_util::should_skip_dir(&e.file_name().to_string_lossy()))
            .map(|e| e.path())
            .filter(|p| !filter.excludes(p, true))
            .collect();
        for sd in &subdirs {
            stack.push(sd.clone());
        }
        scanned += subdirs.len();
        if scanned > SCAN_CAP {
            return ports;
        }
        if subdirs.len() < MIN_PORT_SIBLINGS {
            continue;
        }
        let per_subdir: Vec<(PathBuf, HashSet<String>)> = subdirs
            .iter()
            .map(|sd| {
                let names = std::fs::read_dir(sd)
                    .into_iter()
                    .flatten()
                    .flatten()
                    .filter(|e| e.file_type().map(|t| t.is_file()).unwrap_or(false))
                    .filter_map(|e| e.file_name().to_str().map(String::from))
                    .collect();
                (sd.clone(), names)
            })
            .collect();
        let mut name_counts: HashMap<String, usize> = HashMap::new();
        for (_, names) in &per_subdir {
            for n in names {
                *name_counts.entry(n.clone()).or_insert(0) += 1;
            }
        }
        let shared: HashSet<String> = name_counts
            .into_iter()
            .filter(|(_, c)| *c >= MIN_PORT_SIBLINGS)
            .map(|(n, _)| n)
            .collect();
        if shared.len() < MIN_PORT_SHARED_NAMES {
            continue;
        }
        for (sd, names) in &per_subdir {
            if names.iter().any(|n| shared.contains(n)) {
                ports.insert(sd.clone());
            }
        }
    }
    ports
}

/// Count, tree-wide from `root`, headers with at most
/// [`WHOLE_HEADER_MAX_SRC_LINES`] non-blank lines. Size-only (no parse)
/// — a coarse project-shape signal, capped so a pathological tree can't
/// stall the run. Symlinks are not followed.
fn count_small_headers(root: &Path, filter: &DirFilter) -> usize {
    const SCAN_CAP: usize = 4096;
    let mut count = 0;
    let mut stack = vec![root.to_path_buf()];
    let mut scanned = 0;
    while let Some(dir) = stack.pop() {
        let Ok(read_dir) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in read_dir.flatten() {
            scanned += 1;
            if scanned > SCAN_CAP {
                return count;
            }
            let path = entry.path();
            let Ok(file_type) = entry.file_type() else {
                continue;
            };
            if filter.excludes(&path, file_type.is_dir()) {
                continue;
            }
            if file_type.is_dir() {
                if !crate::fs_util::should_skip_dir(&entry.file_name().to_string_lossy()) {
                    stack.push(path);
                }
            } else if file_type.is_file() && is_header_file(&path) && header_is_small_by_size(&path)
            {
                count += 1;
            }
        }
    }
    count
}

/// Byte budget for the project scan across all counted `.c` files.
/// Covers the corpus' largest C tree (tinyusb, ~4.9 MB of `.c`) with
/// headroom; amalgamation-scale trees (mongoose, ~35 MB) bail out.
/// Fail closed: over budget means no information.
const C_PROJECT_SCAN_MAX_BYTES: u64 = 8 * 1024 * 1024;

/// One-pass facts about the project's non-test `.c` sources.
struct CProjectScan {
    /// Counted `.c` files defining a default-configuration `main`.
    default_mains: Vec<PathBuf>,
}

impl CProjectScan {
    /// True iff the project builds a program of its own — some counted
    /// `.c` file outside an illustrative tree defines a
    /// default-configuration `main`. A library's only `main`s live in
    /// its test harness (already excluded from the scan) or its
    /// `examples/` programs, which demonstrate the library rather than
    /// being it.
    fn builds_a_program(&self, root: &Path) -> bool {
        self.default_mains
            .iter()
            .any(|path| !is_under_illustrative_dir(path, root))
    }
}

/// Directory names whose programs demonstrate a project rather than
/// being it (tinyusb `examples/`, sqlite-vec `examples/simple-c/`) —
/// the same role [`super::fs::is_test_dir_name`] plays for harnesses.
fn is_under_illustrative_dir(file: &Path, root: &Path) -> bool {
    file.strip_prefix(root)
        .unwrap_or(file)
        .parent()
        .into_iter()
        .flat_map(Path::components)
        .filter_map(|c| c.as_os_str().to_str())
        .any(|name| {
            matches!(
                name.to_ascii_lowercase().as_str(),
                "example" | "examples" | "demo" | "demos" | "sample" | "samples"
            )
        })
}

/// Walk the project's non-test `.c` files once. Same coarse one-pass
/// scan (and cap) as [`count_small_headers`], plus a byte budget; test
/// dirs and test-named files are excluded so a large test suite can't
/// mask a single-implementation-file layout. `None` when the scan
/// bailed — callers must treat that as "no information".
fn scan_c_project(root: &Path, filter: &DirFilter) -> Option<CProjectScan> {
    const SCAN_CAP: usize = 4096;
    let mut main_candidates: Vec<PathBuf> = Vec::new();
    let mut remaining_bytes = C_PROJECT_SCAN_MAX_BYTES;
    let mut stack = vec![root.to_path_buf()];
    let mut scanned = 0;
    while let Some(dir) = stack.pop() {
        let Ok(read_dir) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in read_dir.flatten() {
            scanned += 1;
            if scanned > SCAN_CAP {
                return None;
            }
            let path = entry.path();
            let file_name = entry.file_name();
            let name = file_name.to_string_lossy();
            let Ok(file_type) = entry.file_type() else {
                continue;
            };
            if filter.excludes(&path, file_type.is_dir()) {
                continue;
            }
            if file_type.is_dir() {
                if !crate::fs_util::should_skip_dir(&name) && !super::fs::is_test_dir_name(&name) {
                    stack.push(path);
                }
            } else if file_type.is_file()
                && name.to_ascii_lowercase().ends_with(".c")
                && !is_test_c_file_name(&name)
            {
                // Fail closed like the byte-budget gate: an unreadable
                // source file could be the one defining `main`.
                let scan = scan_c_source(&path, remaining_bytes)?;
                if scan.bytes > remaining_bytes {
                    return None;
                }
                remaining_bytes -= scan.bytes;
                if scan.may_define_main {
                    main_candidates.push(path);
                }
            }
        }
    }
    let default_mains = main_candidates
        .into_iter()
        .filter(|candidate| {
            std::fs::read_to_string(candidate).is_ok_and(|text| defines_unconditional_main(&text))
        })
        .collect();
    Some(CProjectScan { default_mains })
}

/// One counted file's scan result: bytes read, and whether any line
/// textually looks like it could define `main` (prefilter for the
/// [`defines_unconditional_main`] parse).
#[derive(Default)]
struct CSourceScan {
    bytes: u64,
    may_define_main: bool,
}

/// Buffered per-line scan of one `.c` file. Reads raw bytes so a
/// non-UTF-8 byte can't silently drop the file from the tally; stops
/// early (with `bytes` past `max_bytes`) once the budget is blown so an
/// oversized file isn't read to the end. `None` on I/O error — the file
/// is skipped, matching the sibling scans.
fn scan_c_source(path: &Path, max_bytes: u64) -> Option<CSourceScan> {
    use std::io::BufRead;
    let file = std::fs::File::open(path).ok()?;
    let mut reader = std::io::BufReader::new(file);
    let mut line = Vec::new();
    let mut scan = CSourceScan::default();
    loop {
        line.clear();
        let n = reader.read_until(b'\n', &mut line).ok()?;
        if n == 0 {
            return Some(scan);
        }
        scan.bytes += n as u64;
        if scan.bytes > max_bytes {
            return Some(scan);
        }
        if !scan.may_define_main && line_may_define_main(&line) {
            scan.may_define_main = true;
        }
    }
}

/// Cheap textual prefilter for a `main` definition: a standalone
/// `main` token followed by `(` (or by nothing — a definition broken
/// across lines). False positives (calls, comments) only cost a
/// parse in [`defines_unconditional_main`].
fn line_may_define_main(line: &[u8]) -> bool {
    let is_ident = |b: u8| b.is_ascii_alphanumeric() || b == b'_';
    line.windows(4).enumerate().any(|(i, window)| {
        window == b"main"
            && (i == 0 || !is_ident(line[i - 1]))
            && match line[i + 4..].iter().find(|b| !b.is_ascii_whitespace()) {
                Some(&next) => next == b'(',
                None => true,
            }
    })
}

/// True iff `source` defines a `main` that is compiled in the default
/// configuration: at the top level, or under a *negative* guard
/// (`#ifndef X` / `#if !defined(X)`, krep's `#if !defined(TESTING)`)
/// that only excludes it from special builds. A main behind a positive
/// feature guard (sds `#ifdef SDS_TEST_MAIN`) is off by default and
/// doesn't count, and neither does one behind a compound condition
/// (`#if !defined(X) && defined(Y)`) — the leading `!` doesn't make
/// the whole guard default-on, so compound guards fail closed.
fn defines_unconditional_main(source: &str) -> bool {
    fn is_negation_only_condition(condition: &str) -> bool {
        let condition = condition.trim();
        condition.starts_with('!') && !condition.contains("&&") && !condition.contains("||")
    }
    fn has_default_on_main(node: Node, source: &str) -> bool {
        let mut cursor = node.walk();
        node.children(&mut cursor).any(|child| match child.kind() {
            "function_definition" => function_definition_name(child, source) == Some("main"),
            "preproc_ifdef" => {
                child.child(0).is_some_and(|d| d.kind() == "#ifndef")
                    && has_default_on_main(child, source)
            }
            "preproc_if" => {
                child
                    .child_by_field_name("condition")
                    .is_some_and(|c| is_negation_only_condition(&source[c.byte_range()]))
                    && has_default_on_main(child, source)
            }
            _ => false,
        })
    }
    let mut parser = tree_sitter::Parser::new();
    if parser
        .set_language(&tree_sitter_c::LANGUAGE.into())
        .is_err()
    {
        return false;
    }
    let Some(tree) = parser.parse(source, None) else {
        return false;
    };
    has_default_on_main(tree.root_node(), source)
}

/// Name of a `function_definition` node: descends declarator wrappers
/// (pointers, parenthesized declarators) to the `function_declarator`
/// and returns its identifier text.
fn function_definition_name<'a>(def: Node, source: &'a str) -> Option<&'a str> {
    let mut node = def.child_by_field_name("declarator")?;
    while node.kind() != "function_declarator" {
        node = node.child_by_field_name("declarator")?;
    }
    let name = node.child_by_field_name("declarator")?;
    (name.kind() == "identifier").then(|| &source[name.byte_range()])
}

fn is_test_c_file_name(name: &str) -> bool {
    let stem = name.to_ascii_lowercase();
    let stem = stem.strip_suffix(".c").unwrap_or(&stem);
    matches!(stem, "test" | "tests")
        || stem.starts_with("test_")
        || stem.ends_with("_test")
        || stem.ends_with("_tests")
}

/// True iff `file`'s non-blank line count is within the whole-render
/// size bound. Reads the file directly; `None`/unreadable → not small.
fn header_is_small_by_size(file: &Path) -> bool {
    let Ok(text) = std::fs::read_to_string(file) else {
        return false;
    };
    let non_blank = text.lines().filter(|l| !l.trim().is_empty()).count();
    non_blank > 0 && non_blank <= WHOLE_HEADER_MAX_SRC_LINES
}

/// Parse `Makefile.am` at `root` for `include_HEADERS` /
/// `pkginclude_HEADERS` declarations, including their `nobase_` /
/// `dist_` / `nodist_` automake-prefix variants. Returns canonical
/// paths of every listed `.h`, or `None` if the file is missing /
/// unreadable / declares no public headers.
/// Fails open on `+=`, conditional assignment, or `$(VAR)`-valued
/// lines — a partial set would silently demote real public headers.
fn parse_include_headers(root: &Path) -> Option<HashSet<PathBuf>> {
    let manifest = root.join("Makefile.am");
    let text = std::fs::read_to_string(&manifest).ok()?;
    let mut out = HashSet::new();
    let mut iter = text.lines();
    while let Some(line) = iter.next() {
        let trimmed = line.trim_start();
        // The installing variables this walker understands, modulo the
        // automake prefixes `nobase_`, `dist_`, and `nodist_` that may
        // decorate them in canonical order. Variables that don't install
        // into the include path (`noinst_HEADERS`, `EXTRA_HEADERS`, …)
        // aren't part of the public-API surface, so they're ignored.
        let stem = trimmed.strip_prefix("nobase_").unwrap_or(trimmed);
        let stem = stem
            .strip_prefix("dist_")
            .or_else(|| stem.strip_prefix("nodist_"))
            .unwrap_or(stem);
        let after_name = ["include_HEADERS", "pkginclude_HEADERS"]
            .iter()
            .find_map(|name| stem.strip_prefix(name));
        let Some(after_name) = after_name else {
            continue;
        };
        let after_name = after_name.trim_start();
        // Fail open on `+=` appends, conditional-assignment operators
        // (`?=`, `:=`), or any operator other than plain `=`. A `+=`
        // adds entries that this parser would silently miss, leaving
        // an incomplete public set; better to drop the signal entirely
        // than to demote a real public header.
        let mut rest = after_name.strip_prefix('=')?;
        // Collect the assignment value, honoring `\` line continuations.
        let mut value = String::new();
        loop {
            let (head, continued) = match rest.strip_suffix('\\') {
                Some(head) => (head, true),
                None => (rest, false),
            };
            value.push_str(head);
            if !continued {
                break;
            }
            value.push(' ');
            let Some(next) = iter.next() else { break };
            rest = next;
        }
        for tok in value.split_whitespace() {
            // Strip automake variable references like `$(srcdir)/foo.h`
            // — we only need the path component for canonicalization.
            let tok = tok.trim_start_matches("$(srcdir)/");
            // A `$(...)` token (variable expansion to an opaque value)
            // means the value list isn't statically known. Fail open.
            if tok.contains("$(") || tok.contains("${") {
                return None;
            }
            if !tok.ends_with(".h") {
                continue;
            }
            let resolved = root.join(tok);
            let canonical = resolved.canonicalize().unwrap_or(resolved);
            out.insert(canonical);
        }
    }
    if out.is_empty() { None } else { Some(out) }
}

use super::{
    FileLines, WalkCtx, budget_chunk_ranges, build_per_file_content, collect_blank_line_groups,
    collect_doc_comments_above_filtered, comment_only_rows, dedup_sorted, extend_nonblank_rows,
    extend_span, file_depth_factor, file_lines_covered_by, node_end_row_trimmed, push_rows,
    signature_end_row, single_file_lines_content, trim_end_before_next_decl,
    whole_file_lines_content,
};

/// Fixed-size source-order chunks of `C_DECL_NAMES_CHUNK_SIZE` decls.
/// A trailing remainder below half a chunk merges into the previous
/// chunk — a tiny tail chunk is so cheap that its ratio jumps the
/// queue, dragging its gated per-decl train with it.
fn count_based_chunk_ranges(decl_count: usize) -> Vec<Range<usize>> {
    budget_chunk_ranges(
        decl_count,
        |range| range.len(),
        C_DECL_NAMES_CHUNK_SIZE,
        C_DECL_NAMES_CHUNK_SIZE / 2,
        |_| true,
        |_| true,
    )
}

/// One names-surface chunk: which decls it lists, and its position in
/// its own linkage group's chunk series. The external and internal
/// groups are chunked and ranked independently — they are two separate
/// rosters that happen to share a key — so adding an internal roster to
/// a file never reprices its public one.
struct NamesChunk {
    range: Range<usize>,
    index_in_group: usize,
    group_chunk_count: usize,
}

/// Chunk `decls` with a forced break between the external-linkage
/// prefix (`external_count` entries) and the internal-linkage tail.
fn names_chunks(external_count: usize, decl_count: usize) -> Vec<NamesChunk> {
    let mut out = Vec::new();
    for (offset, count) in [
        (0, external_count),
        (external_count, decl_count - external_count),
    ] {
        let ranges = count_based_chunk_ranges(count);
        let group_chunk_count = ranges.len();
        out.extend(
            ranges
                .into_iter()
                .enumerate()
                .map(|(index_in_group, range)| NamesChunk {
                    range: range.start + offset..range.end + offset,
                    index_in_group,
                    group_chunk_count,
                }),
        );
    }
    out
}

pub fn expand_in_dir(dir: &Path, ctx: &WalkCtx) -> Vec<Batch<BatchKey>> {
    let c_files = c_source_files(dir, ctx);
    if c_files.is_empty() {
        return Vec::new();
    }

    // Whole-file render applies only when the project (tree-wide) holds
    // a small handful of small headers — each a load-bearing module API.
    // A project flooded with small headers (many-small-modules layout)
    // keeps the decomposed names-surface so budget reaches breadth.
    let allow_whole_headers = ctx
        .c_state()
        .whole_header_render_allowed(ctx.root(), ctx.dir_filter());

    let mut out = Vec::new();
    for file in &c_files {
        let Some((source, tree)) = parse_c(ctx, file) else {
            continue;
        };

        // A small public-API header is delivered whole in one batch
        // rather than decomposed into banner / includes / names-surface /
        // per-decl rows. The decomposition strips macro values, enum
        // bodies, and `#ifdef` shape — exactly the content these tiny
        // headers exist to carry — and fragments a handful of
        // declarations across four batch kinds for no budget benefit.
        if allow_whole_headers && let Some(content) = whole_small_header_content(file, &source, ctx)
        {
            out.push(Batch {
                key: CKey::WholeFile { file: file.clone() }.into(),
                predecessor: None,
                content,
                value: whole_file_value(file, ctx),
            });
            continue;
        }

        if let Some(content) = build_per_file_content(file, ctx, parse_c, collect_header_banner) {
            out.push(Batch {
                key: CKey::HeaderBanner { file: file.clone() }.into(),
                predecessor: None,
                content,
                value: header_banner_value(file, ctx),
            });
        }

        // NS authors rank include blocks tier 4+; the walker's tiny
        // includes batches otherwise flood the early budget on header-
        // rich trees (tinyusb's ~40 class-header includes inside 3K).
        // Gate them behind the file's first names-surface chunk — the
        // include map is reference detail once the file's surface is
        // on the table. Files with no decl surface keep an ungated
        // batch: there the includes ARE the file's content.
        let includes_lines = collect_includes(&tree, &source);
        let includes_content = single_file_lines_content(file, &source, includes_lines);
        let push_includes = |out: &mut Vec<Batch<BatchKey>>, predecessor: Option<BatchKey>| {
            if let Some(content) = includes_content.clone() {
                out.push(Batch {
                    key: CKey::Includes { file: file.clone() }.into(),
                    predecessor,
                    content,
                    value: includes_value(file, ctx),
                });
            }
        };

        let mut decls = find_decls(&tree, &source, file, admits_internal_decls(file, ctx));
        if decls.is_empty() {
            push_includes(&mut out, None);
            continue;
        }
        let configuration_surface = is_configuration_surface_header(file, &source, &tree, &decls);
        if configuration_surface {
            apply_configuration_surface_aggregate_splits(&mut decls, &source);
        }
        // Last 0-based row claimed by the file's HeaderBanner (the
        // leading run of comment children of root). Used as a stop
        // boundary so the first decl's `DeclDoc` doesn't walk into
        // banner territory when there's no blank line between them.
        let banner_end_row = header_banner_end_row(&tree);
        // C catalog files (`sds.h`, headers exposing the whole public
        // surface) often anchor NS rows on the *unified* declaration
        // listing — splitting at the default 12 fragments rows like
        // "Public fn declarations — utility fns" across chunks. Chunk
        // only when the surface is large enough that the unified batch
        // would lose the value/cost race against per-decl batches.
        let external_count = decls
            .iter()
            .filter(|(_, i)| i.linkage == DeclLinkage::External)
            .count();
        let chunks = names_chunks(external_count, decls.len());
        let names_predecessors: Vec<_> = (0..chunks.len())
            .map(|chunk_index| {
                BatchKey::C(CKey::DeclNames {
                    file: file.clone(),
                    chunk_index,
                })
            })
            .collect();
        let all_starts: std::collections::HashSet<usize> =
            decls.iter().map(|(_, i)| i.start_line).collect();
        let src_lines: Vec<&str> = source.lines().collect();
        let names_lines_by_chunk: Vec<FileLines> = chunks
            .iter()
            .map(|chunk| {
                collect_decl_names_from_with_global_starts(
                    &decls[chunk.range.clone()],
                    &all_starts,
                    &src_lines,
                )
            })
            .collect();
        // Per-decl index → chunk index lookup. Built from the chunk
        // ranges so callers don't re-divide.
        let decl_to_chunk: Vec<usize> = {
            let mut v = vec![0_usize; decls.len()];
            for (chunk_index, chunk) in chunks.iter().enumerate() {
                for i in chunk.range.clone() {
                    v[i] = chunk_index;
                }
            }
            v
        };
        let mut names_chunk0_emitted = false;
        for ((chunk_index, chunk), names_lines) in
            chunks.iter().enumerate().zip(&names_lines_by_chunk)
        {
            let Some(content) = single_file_lines_content(file, &source, names_lines.clone())
            else {
                continue;
            };
            names_chunk0_emitted |= chunk_index == 0;
            out.push(Batch {
                key: names_predecessors[chunk_index].clone(),
                predecessor: None,
                content,
                value: decl_names_value(
                    file,
                    ctx,
                    chunk,
                    configuration_surface && configuration_surface_names_chunk_floored(chunk_index),
                ),
            });
        }
        push_includes(
            &mut out,
            names_chunk0_emitted.then(|| names_predecessors[0].clone()),
        );
        let mut configuration_surface_doc_index = 0;
        let mut configuration_surface_aggregate_group_index = 0;
        for (decl_index, (node, info)) in decls.iter().enumerate() {
            let names_chunk_index = decl_to_chunk[decl_index];
            let names_predecessor = names_predecessors[names_chunk_index].clone();
            let chunk_names_lines = &names_lines_by_chunk[names_chunk_index];
            let decl_key = CKey::Decl {
                file: file.clone(),
                start_line: info.start_line,
            };
            let decl_lines = collect_decl(*node, info, &source, &all_starts);
            let doc_lines =
                collect_doc_comments_above_filtered(*node, &source, banner_end_row, |prev, _| {
                    prev.kind() == "comment"
                });
            let body_lines = if info.has_body {
                collect_decl_body(*node, &src_lines)
            } else {
                FileLines::new(Vec::new())
            };
            let decl_has_descendants = !doc_lines.full.is_empty() || !body_lines.full.is_empty();
            if (!file_lines_covered_by(&decl_lines, chunk_names_lines) || decl_has_descendants)
                && let Some(content) = single_file_lines_content(file, &source, decl_lines)
            {
                out.push(Batch {
                    key: decl_key.clone().into(),
                    predecessor: Some(names_predecessor.clone()),
                    content,
                    value: decl_value(file, info.kind, ctx),
                });
            }
            let decl_predecessor = BatchKey::C(decl_key);
            if let Some(content) = single_file_lines_content(file, &source, doc_lines) {
                let eligible_configuration_surface_doc =
                    configuration_surface && is_configuration_surface_decl_kind(info.kind);
                let configuration_surface_doc = eligible_configuration_surface_doc
                    && configuration_surface_doc_floored(configuration_surface_doc_index);
                if eligible_configuration_surface_doc {
                    configuration_surface_doc_index += 1;
                }
                out.push(Batch {
                    key: CKey::DeclDoc {
                        file: file.clone(),
                        start_line: info.start_line,
                    }
                    .into(),
                    predecessor: Some(decl_predecessor.clone()),
                    content,
                    value: decl_doc_value(file, info.kind, ctx, configuration_surface_doc),
                });
            }
            if info.has_body
                && let Some(content) = single_file_lines_content(file, &source, body_lines)
            {
                out.push(Batch {
                    key: CKey::DeclBody {
                        file: file.clone(),
                        start_line: info.start_line,
                    }
                    .into(),
                    predecessor: Some(decl_predecessor.clone()),
                    content,
                    value: decl_body_value(file, info.kind, ctx),
                });
            }
            // Per-group / per-chunk batches for big aggregate bodies.
            // Sibling of `DeclDoc` under the same `Decl` predecessor;
            // the parent `Decl`'s span was trimmed in `collect_decl` to
            // the type header + closer so the group rows don't overlap.
            for group in &info.member_groups {
                // The identity slot rides with the `Decl` header;
                // dropping it here keeps the two batches disjoint.
                let mut rows = group.rows.clone();
                rows.retain(|row| Some(*row) != info.base_member_line);
                let lines = FileLines::new(rows);
                if let Some(content) = single_file_lines_content(file, &source, lines) {
                    let configuration_surface_aggregate_group = configuration_surface
                        && configuration_surface_aggregate_group_floored(
                            configuration_surface_aggregate_group_index,
                        );
                    if configuration_surface {
                        configuration_surface_aggregate_group_index += 1;
                    }
                    out.push(Batch {
                        key: CKey::AggregateMemberGroup {
                            file: file.clone(),
                            start_line: info.start_line,
                            group_start_line: group.group_start_line,
                        }
                        .into(),
                        predecessor: Some(decl_predecessor.clone()),
                        content,
                        value: aggregate_member_group_value(
                            file,
                            info.kind,
                            ctx,
                            configuration_surface_aggregate_group,
                        ),
                    });
                }
            }
        }
    }

    out
}

// --- decl classification ------------------------------------------------

/// Coarse decl kinds for value weighting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DeclKind {
    Typedef,
    Aggregate,
    /// `int foo(...) { ... }` — fn with body.
    FunctionDef,
    /// `int foo(...);` — fn prototype.
    FunctionDecl,
    Variable,
    /// `#define X val`.
    Macro,
    /// `#define X(args) body`.
    MacroFn,
}

impl DeclKind {
    fn kind_weight(self) -> f64 {
        match self {
            DeclKind::Typedef => 1.10,
            DeclKind::Aggregate => 1.10,
            DeclKind::FunctionDef => 1.00,
            DeclKind::FunctionDecl => 0.95,
            DeclKind::MacroFn => 0.95,
            DeclKind::Macro => 0.90,
            DeclKind::Variable => 0.80,
        }
    }
}

/// Whether a top-level decl is visible outside its translation unit.
///
/// In most languages "private" means "lower value", and for a `.h` file
/// that holds: the header *is* the contract, so a `static` item in one
/// is noise. A `.c` file is the opposite — its `static` items are the
/// implementation, and in a program (an interpreter, a CLI tool, a
/// compiler) `main` plus a wall of `static` helpers is essentially all
/// there is. So in a project that [`admits_internal_decls`], internal
/// decls are surfaced as an ordinary decl train, priced by kind exactly
/// like external ones; the linkage split only decides *membership* and
/// which names-surface group a decl chunks into. A restricted value
/// axis was swept (×0.5 / ×0.75 on the whole internal train) and is
/// strictly worse: the damped roster never wins purchase at all, so the
/// recall it unlocks is never delivered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DeclLinkage {
    External,
    Internal,
}

#[derive(Debug, Clone)]
struct DeclInfo {
    start_line: usize,
    kind: DeclKind,
    linkage: DeclLinkage,
    /// Gates `DeclBody` emission — true only for `function_definition`.
    has_body: bool,
    /// Per-group member-batches for big aggregates. When non-empty the
    /// parent `Decl` is trimmed to the type header + closing brace.
    member_groups: Vec<AggregateMemberGroup>,
    /// Line of the aggregate's leading by-value composite member, if it
    /// has one — see [`base_object_member_line`]. Rides with the names
    /// roster and with the trimmed `Decl` header rather than with its
    /// member group.
    base_member_line: Option<usize>,
}

/// Minimum structural evidence for treating a header as an application
/// configuration surface rather than a general declaration catalog.
///
/// The count gate excludes small API headers with a handful of constants;
/// the annotation ratio rejects generated register / enum-like macro catalogs;
/// and the function ceiling rejects ordinary APIs which happen to define many
/// documented constants alongside their callable surface.
const CONFIG_SURFACE_MIN_OBJECT_MACROS: usize = 24;
const CONFIG_SURFACE_MIN_ANNOTATED_MACROS: usize = 12;
const CONFIG_SURFACE_MAX_FUNCTION_DECLS: usize = 4;

#[derive(Debug, Default, PartialEq, Eq)]
struct ConfigurationSurfaceStats {
    object_macros: usize,
    annotated_object_macros: usize,
    function_decls: usize,
    total_decls: usize,
}

fn is_configuration_surface_header(
    file: &Path,
    source: &str,
    tree: &Tree,
    decls: &[(Node, DeclInfo)],
) -> bool {
    if !is_header_file(file) {
        return false;
    }
    let stats = configuration_surface_stats(source, tree, decls);
    stats.object_macros >= CONFIG_SURFACE_MIN_OBJECT_MACROS
        && stats.annotated_object_macros >= CONFIG_SURFACE_MIN_ANNOTATED_MACROS
        && stats.annotated_object_macros * 2 >= stats.object_macros
        && stats.function_decls <= CONFIG_SURFACE_MAX_FUNCTION_DECLS
        && stats.object_macros * 2 >= stats.total_decls
}

fn configuration_surface_stats(
    source: &str,
    tree: &Tree,
    decls: &[(Node, DeclInfo)],
) -> ConfigurationSurfaceStats {
    let lines: Vec<&str> = source.lines().collect();
    let banner_end_row = header_banner_end_row(tree);
    let mut stats = ConfigurationSurfaceStats {
        total_decls: decls.len(),
        ..ConfigurationSurfaceStats::default()
    };
    for (node, info) in decls {
        match info.kind {
            DeclKind::Macro => {
                stats.object_macros += 1;
                if has_adjacent_explanatory_comment(*node, &lines, banner_end_row) {
                    stats.annotated_object_macros += 1;
                }
            }
            DeclKind::FunctionDef | DeclKind::FunctionDecl => stats.function_decls += 1,
            DeclKind::Typedef | DeclKind::Aggregate | DeclKind::Variable | DeclKind::MacroFn => {}
        }
    }
    stats
}

fn has_adjacent_explanatory_comment(
    node: Node,
    lines: &[&str],
    banner_end_row: Option<usize>,
) -> bool {
    let start_row = node.start_position().row;
    let preceding_comment = start_row
        .checked_sub(1)
        .filter(|row| banner_end_row.is_none_or(|banner_end| *row > banner_end))
        .and_then(|row| lines.get(row))
        .is_some_and(|line| {
            let line = line.trim();
            line.starts_with("//")
                || line.starts_with("/*")
                || line.starts_with('*')
                // A `*/` terminator counts only when the line is comment
                // through and through — either a block-comment
                // continuation (no `/*` on the line) or nothing but
                // whitespace before its `/*`. Otherwise a previous
                // macro's own trailing `/* doc */` would be credited to
                // this one.
                || (line.ends_with("*/")
                    && line.find("/*").is_none_or(|open| line[..open].trim().is_empty()))
        });
    if preceding_comment {
        return true;
    }

    // `preproc_def` consumes its terminating newline, so `end_position()`
    // is column 0 of the row after the macro and any same-line trailing
    // comment sits inside the node text. Scan the macro's own last line,
    // minus string-literal contents (`//` inside a URL value is not a
    // comment).
    let end = node.end_position();
    let own_row = if end.column == 0 {
        end.row.saturating_sub(1)
    } else {
        end.row
    };
    lines.get(own_row).is_some_and(|line| {
        let stripped = strip_c_string_literals(line);
        stripped.contains("//") || stripped.contains("/*")
    })
}

/// The line with the contents of its double-quoted string literals
/// removed (escapes respected), so comment markers are only found in
/// actual code/comment text.
fn strip_c_string_literals(line: &str) -> String {
    let mut out = String::with_capacity(line.len());
    let mut in_string = false;
    let mut escaped = false;
    for c in line.chars() {
        if in_string {
            if escaped {
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == '"' {
                in_string = false;
            }
            continue;
        }
        if c == '"' {
            in_string = true;
            continue;
        }
        out.push(c);
    }
    out
}

/// One chunk of a big aggregate body — struct/union field group or
/// sized enum-body chunk.
#[derive(Debug, Clone)]
struct AggregateMemberGroup {
    group_start_line: usize,
    rows: Vec<usize>,
}

/// All public top-level decls in source order. Descends through one
/// wrapping `#ifndef X / #define X / #endif` header guard, through
/// `extern "C" { … }` linkage specs, and — in headers — through
/// declaration-only feature gates (conditionally-compiled API). `.c`
/// gates stay opaque: there, conditional compilation is implementation
/// detail (sqlite-vec's `#ifndef _WIN32` portability-shim typedefs).
/// External-linkage decls come first, then internal-linkage ones — each
/// group in source order, so the two roster groups chunk separately.
fn find_decls<'a>(
    tree: &'a Tree,
    source: &str,
    file: &Path,
    admit_internal: bool,
) -> Vec<(Node<'a>, DeclInfo)> {
    let in_header = is_header_file(file);
    let guard_name = header_guard_name(tree.root_node(), source);
    let mut out = Vec::new();
    walk_top_level(tree.root_node(), source, in_header, &mut |node| {
        if let Some(info) = classify_decl(node, source, in_header, guard_name)
            && (admit_internal || info.linkage == DeclLinkage::External)
        {
            out.push((node, info));
        }
    });
    out.sort_by_key(|(_, d)| d.start_line);
    out.dedup_by_key(|(_, d)| d.start_line);
    out.sort_by_key(|(_, d)| d.linkage == DeclLinkage::Internal);
    out
}

/// Visit each "effective top-level" item — descends through the file's
/// header guard and through `extern "C" { … }` linkage specs.
/// `feature_gates` additionally descends declaration-only `#if` /
/// `#ifdef` blocks.
fn walk_top_level<'a, F: FnMut(Node<'a>)>(
    root: Node<'a>,
    source: &str,
    feature_gates: bool,
    visit: &mut F,
) {
    let header_guard_body = header_guard_body_node(root, source);
    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        if Some(child) == header_guard_body {
            descend_envelopes(child, source, feature_gates, visit);
        } else {
            visit_with_envelope_descent(child, source, feature_gates, visit);
        }
    }
}

/// Visit each child of `node` with envelope-descent — used for header
/// guard / `extern "C"` linkage_specification bodies.
fn descend_envelopes<'a, F: FnMut(Node<'a>)>(
    node: Node<'a>,
    source: &str,
    feature_gates: bool,
    visit: &mut F,
) {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        visit_with_envelope_descent(child, source, feature_gates, visit);
    }
}

/// Visit `node`, descending if it's an `extern "C" { … }` envelope
/// (raw or `#ifdef __cplusplus`-wrapped) or — when `feature_gates` —
/// a declaration-bearing feature gate (`#if`/`#ifdef` whose branches
/// hold only declarations and directives).
fn visit_with_envelope_descent<'a, F: FnMut(Node<'a>)>(
    node: Node<'a>,
    source: &str,
    feature_gates: bool,
    visit: &mut F,
) {
    if let Some(decl_list) = extern_c_declaration_list(node, source) {
        descend_envelopes(decl_list, source, feature_gates, visit);
        return;
    }
    if feature_gates
        && matches!(node.kind(), "preproc_if" | "preproc_ifdef")
        && !is_disabled_preproc_if(node, source)
        && feature_gate_decl_class(node) == GateClass::DeclOnly
    {
        descend_feature_gate_branches(node, source, visit);
        return;
    }
    visit(node);
}

/// Visit every branch of a decl-bearing feature gate: direct children
/// plus the bodies of `#else` / `#elif` alternates. Children go back
/// through [`visit_with_envelope_descent`], so nested gates descend (or
/// stay opaque) on their own merits.
fn descend_feature_gate_branches<'a, F: FnMut(Node<'a>)>(
    node: Node<'a>,
    source: &str,
    visit: &mut F,
) {
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        match child.kind() {
            "preproc_else" | "preproc_elif" | "preproc_elifdef" => {
                descend_feature_gate_branches(child, source, visit);
            }
            _ => visit_with_envelope_descent(child, source, true, visit),
        }
    }
}

/// True for `#if 0`-style disabled blocks — commented-out code, not a
/// feature gate.
fn is_disabled_preproc_if(node: Node, source: &str) -> bool {
    node.child_by_field_name("condition")
        .is_some_and(|cond| source[cond.start_byte()..cond.end_byte()].trim() == "0")
}

/// How a `preproc_if*` subtree relates to the declaration surface.
#[derive(PartialEq, Eq, Clone, Copy)]
enum GateClass {
    /// At least one declaration-shaped child and nothing code-shaped in
    /// any branch — conditionally-compiled top-level API (krep's
    /// per-ISA prototypes, tinyusb's gated application API, bareiron's
    /// `#ifdef SYNC_WORLD_TO_DISK` block).
    DeclOnly,
    /// Only directives / comments — the include-map collector's
    /// territory; nothing for the decl surface.
    DirectiveOnly,
    /// Wraps real code (function bodies, statements) somewhere — the
    /// amalgamation idiom stays opaque.
    Code,
}

/// Classify a `preproc_if*` / `preproc_else*` subtree for feature-gate
/// descent. Branch alternates and nested gates classify recursively; a
/// code-bearing nested gate keeps the whole envelope opaque.
fn feature_gate_decl_class(node: Node) -> GateClass {
    let mut decl_found = false;
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        match child.kind() {
            "declaration" | "type_definition" | "struct_specifier" | "union_specifier"
            | "enum_specifier" => decl_found = true,
            "preproc_include" | "preproc_def" | "preproc_function_def" | "comment" => {}
            "preproc_if" | "preproc_ifdef" | "preproc_else" | "preproc_elif"
            | "preproc_elifdef" => match feature_gate_decl_class(child) {
                GateClass::Code => return GateClass::Code,
                GateClass::DeclOnly => decl_found = true,
                GateClass::DirectiveOnly => {}
            },
            // Condition/name tokens on the `#if` / `#ifdef` itself.
            "identifier"
            | "binary_expression"
            | "parenthesized_expression"
            | "unary_expression"
            | "call_expression"
            | "preproc_defined"
            | "number_literal"
            | "char_literal"
            | "string_literal" => {}
            _ => return GateClass::Code,
        }
    }
    if decl_found {
        GateClass::DeclOnly
    } else {
        GateClass::DirectiveOnly
    }
}

/// If `node` is an `extern "C" { … }` envelope (raw or
/// `#ifdef __cplusplus`-wrapped), return the inner `declaration_list`.
/// The `#ifdef` envelope is accepted only when the guard symbol is
/// `__cplusplus` — feature gates around a linkage spec stay opaque.
fn extern_c_declaration_list<'a>(node: Node<'a>, source: &str) -> Option<Node<'a>> {
    let linkage = match node.kind() {
        "linkage_specification" => node,
        "preproc_ifdef" => cplusplus_wrapped_linkage_specification(node, source)?,
        _ => return None,
    };
    let mut cursor = linkage.walk();
    linkage
        .children(&mut cursor)
        .find(|c| c.kind() == "declaration_list")
}

/// If `ifdef` is `#ifdef __cplusplus / linkage_specification / #endif`,
/// return the `linkage_specification`. Rejects feature gates and
/// `#ifndef`.
fn cplusplus_wrapped_linkage_specification<'a>(ifdef: Node<'a>, source: &str) -> Option<Node<'a>> {
    let mut cursor = ifdef.walk();
    let mut children = ifdef.children(&mut cursor);
    if children.next()?.kind() != "#ifdef" {
        return None;
    }
    let name_node = children.next()?;
    if name_node.kind() != "identifier"
        || &source[name_node.start_byte()..name_node.end_byte()] != "__cplusplus"
    {
        return None;
    }
    let mut found: Option<Node<'a>> = None;
    for child in children {
        match child.kind() {
            "#endif" | "\n" | "comment" => {}
            "linkage_specification" if found.is_none() => found = Some(child),
            _ => return None,
        }
    }
    found
}

/// The `preproc_ifdef` wrapping the file as a header guard — `#ifndef X`
/// whose body's first child is `#define X`. Direct children are the
/// effective top-level lines.
fn header_guard_body_node<'a>(root: Node<'a>, source: &str) -> Option<Node<'a>> {
    let mut cursor = root.walk();
    let mut candidate: Option<Node<'a>> = None;
    let mut other_top_level = 0;
    for child in root.children(&mut cursor) {
        match child.kind() {
            "comment" => continue,
            "preproc_ifdef" if candidate.is_none() && other_top_level == 0 => {
                if is_header_guard(child, source) {
                    candidate = Some(child);
                } else {
                    other_top_level += 1;
                }
            }
            _ => other_top_level += 1,
        }
    }
    if other_top_level == 0 {
        candidate
    } else {
        None
    }
}

/// The symbol of the file's wrapping header guard, when it has one.
fn header_guard_name<'a>(root: Node, source: &'a str) -> Option<&'a str> {
    let guard = header_guard_body_node(root, source)?;
    let name = guard.child_by_field_name("name")?;
    Some(&source[name.start_byte()..name.end_byte()])
}

/// True iff `node` is `#ifndef X / #define X / … / #endif`.
fn is_header_guard(ifdef: Node, source: &str) -> bool {
    let text = &source[ifdef.start_byte()..ifdef.end_byte()];
    let mut lines = text.lines();
    let first = lines.next().unwrap_or("");
    let Some(name) = first
        .trim_start()
        .strip_prefix("#")
        .map(|s| s.trim_start())
        .and_then(|s| s.strip_prefix("ifndef"))
        .map(|s| s.split_whitespace().next().unwrap_or(""))
        .filter(|s| !s.is_empty())
    else {
        return false;
    };
    // Look ahead 8 lines for the `#define X` (blank/comment-tolerant).
    for line in lines.take(8) {
        let t = line.trim_start();
        if t.is_empty() || t.starts_with("//") || t.starts_with("/*") {
            continue;
        }
        let Some(rest) = t.strip_prefix("#").map(str::trim_start) else {
            return false;
        };
        let Some(after) = rest.strip_prefix("define") else {
            return false;
        };
        return after.split_whitespace().next() == Some(name);
    }
    false
}

/// Classify a top-level node as a decl. `None` for unsurfaced kinds
/// (preproc_include, comments, non-inline `static` in `.h` files, …).
/// `guard_name` is the file's header-guard symbol, if it has one — see
/// [`is_header_guard_define`].
fn classify_decl(
    node: Node,
    source: &str,
    in_header: bool,
    guard_name: Option<&str>,
) -> Option<DeclInfo> {
    let start_line = node.start_position().row + 1;
    let (kind, has_body, internal) = match node.kind() {
        "function_definition" => {
            // A header's `static inline` definition is the header-only
            // accessor idiom — part of the API expansion, not internal
            // linkage.
            let internal = has_static_specifier(node, source)
                && !(in_header && has_storage_class(node, source, "inline"));
            (DeclKind::FunctionDef, true, internal)
        }
        "declaration" => {
            let kind = if has_struct_union_or_enum(node) {
                DeclKind::Aggregate
            } else if has_function_declarator(node) {
                DeclKind::FunctionDecl
            } else {
                DeclKind::Variable
            };
            (kind, false, has_static_specifier(node, source))
        }
        "type_definition" => (DeclKind::Typedef, false, false),
        // Tree-sitter-c parses a top-level bare `struct foo { … };`
        // (no declarator) as a `struct_specifier` followed by a `;`
        // token rather than wrapping them in a `declaration`. Surface
        // the specifier itself.
        "struct_specifier" | "union_specifier" | "enum_specifier" => {
            (DeclKind::Aggregate, false, false)
        }
        "preproc_def" => {
            // Skip the header-guard's own `#define X` — it's part of
            // the guard envelope, not a public macro.
            if is_header_guard_define(node, source, guard_name) {
                return None;
            }
            (DeclKind::Macro, false, false)
        }
        "preproc_function_def" => (DeclKind::MacroFn, false, false),
        _ => return None,
    };
    // A `static` item in a header is an implementation leak — the file's
    // job is to declare a contract, so there is nothing to surface. In a
    // `.c` file it is ordinary internal linkage; see [`DeclLinkage`].
    if internal && in_header {
        return None;
    }
    let linkage = if internal {
        DeclLinkage::Internal
    } else {
        DeclLinkage::External
    };
    let aggregate_body = matches!(kind, DeclKind::Aggregate | DeclKind::Typedef)
        .then(|| find_aggregate_body(node))
        .flatten();
    Some(DeclInfo {
        start_line,
        kind,
        linkage,
        has_body,
        member_groups: aggregate_body
            .map(|body| collect_aggregate_member_groups(body, source))
            .unwrap_or_default(),
        base_member_line: aggregate_body.and_then(base_object_member_line),
    })
}

/// True iff `file`'s internal-linkage (`static`) declarations are
/// surfaced at all.
///
/// The distinction is library vs program, and it is project-wide. A
/// library's headers are its contract, so its `static` items really are
/// implementation detail the reader can skip — surfacing them displaces
/// the API surface NS authors anchor on. A program has no third-party
/// contract at all: `main` plus a wall of `static` helpers *is* the
/// program, its headers are C's module plumbing, and hiding internal
/// linkage hides the substance. A project counts as a library when it
/// declares an installed header set or when nothing outside its test
/// and example trees defines `main`.
///
/// Headers never reach here with internal decls — [`classify_decl`]
/// drops those outright, in either kind of project.
fn admits_internal_decls(file: &Path, ctx: &WalkCtx) -> bool {
    !is_header_file(file)
        && ctx.c_state().public_headers(ctx.root()).is_none()
        && ctx.c_state().builds_a_program(ctx.root(), ctx.dir_filter())
}

/// Minimum struct/union body span (lines) for field-group chunking.
const AGGREGATE_STRUCT_MIN_LINES: usize = 30;

/// Minimum field-groups required for struct/union chunking.
const AGGREGATE_STRUCT_MIN_GROUPS: usize = 3;

/// Minimum content rows per struct field group. Blank-line-separated
/// crumbs below this are coalesced with their neighbor: 1-2-line
/// groups are near-free, so their value/cost ratio queue-jumps and
/// early budget drains into struct-field confetti that no NS row
/// wants at that granularity.
const AGGREGATE_STRUCT_GROUP_MIN_ROWS: usize = 5;

/// Comment sections in configuration structs include explanatory rows, so a
/// wider minimum keeps tiny annotated tails from queue-jumping the core fields.
const CONFIGURATION_AGGREGATE_GROUP_MIN_ROWS: usize = 10;

/// Minimum enumerators for enum chunking. Below this, the whole enum
/// fits in budget as one `Decl` and splitting adds churn.
const AGGREGATE_ENUM_CHUNK_MIN: usize = 32;

/// Chunk size for big enum bodies — roughly the inner-row granularity
/// NS authors use ("ND_ADD through ND_SHR" ~10-14 enumerators).
const AGGREGATE_ENUM_CHUNK_SIZE: usize = 12;

/// Line of the aggregate's leading by-value composite member — the slot
/// C uses to say what a type *is* rather than what it holds: the
/// embedded supertype of the vtable idiom (`Row super;` opening
/// `struct Process_`) or the discriminant of a tagged union
/// (`NodeKind kind;` opening `struct Node`).
///
/// A split aggregate renders as its type header plus closing brace,
/// and on the names roster it renders as its opening line alone — in
/// both cases the type's name is all that survives, so a type that
/// *is a* `Row` reads as unrelated to `Row` and a tagged union reads
/// as an untagged bag of fields. The line rides with both tiers,
/// which form a predecessor chain (roster → `Decl` → member groups),
/// so it can never be separated from the type it identifies.
///
/// Scalars are data and stay in their member group; a pointer, array,
/// or bitfield member references something else rather than embedding
/// it, and a multi-line member is not an identity slot.
fn base_object_member_line(body: Node) -> Option<usize> {
    if body.kind() != "field_declaration_list" {
        return None;
    }
    let mut cursor = body.walk();
    let first = body
        .named_children(&mut cursor)
        .find(|child| child.kind() != "comment")?;
    if first.kind() != "field_declaration" || first.start_position().row != first.end_position().row
    {
        return None;
    }
    let member_type = first.child_by_field_name("type")?;
    let composite_type = match member_type.kind() {
        // A named type: `Row super;`. Built-ins (including `size_t` and
        // the `stdint` family) parse as `primitive_type`, so plain
        // scalars never reach here.
        "type_identifier" => true,
        // The untypedef'd form: `struct Row_ super;`. A body here would
        // be an inline anonymous definition, not an embedding.
        "struct_specifier" | "union_specifier" => member_type.child_by_field_name("body").is_none(),
        _ => false,
    };
    // A plain identifier declarator is the by-value form; `*`, `[]`,
    // and `:` shapes wrap it in a pointer/array/bitfield declarator.
    if !composite_type || first.child_by_field_name("declarator")?.kind() != "field_identifier" {
        return None;
    }
    Some(first.start_position().row + 1)
}

fn find_aggregate_body(node: Node) -> Option<Node> {
    fn spec_body(spec: Node) -> Option<Node> {
        match spec.kind() {
            "struct_specifier" | "union_specifier" => {
                let body = spec.child_by_field_name("body")?;
                (body.kind() == "field_declaration_list").then_some(body)
            }
            "enum_specifier" => {
                let body = spec.child_by_field_name("body")?;
                (body.kind() == "enumerator_list").then_some(body)
            }
            _ => None,
        }
    }
    match node.kind() {
        "struct_specifier" | "union_specifier" | "enum_specifier" => spec_body(node),
        "declaration" | "type_definition" => {
            let mut cursor = node.walk();
            node.children(&mut cursor).find_map(spec_body)
        }
        _ => None,
    }
}

/// Aggregate body → member chunks. Struct/union split on blank lines,
/// enum split fixed-size. Empty when too small.
fn collect_aggregate_member_groups(body: Node, source: &str) -> Vec<AggregateMemberGroup> {
    let body_start = body.start_position().row;
    let body_end = body.end_position().row;
    if body_end <= body_start + 1 {
        return Vec::new();
    }
    match body.kind() {
        "field_declaration_list" => {
            if body_end.saturating_sub(body_start) + 1 < AGGREGATE_STRUCT_MIN_LINES {
                return Vec::new();
            }
            // Eligibility is judged on the raw blank-line groups:
            // coalescing may merge below the gate, but a struct whose
            // author laid out 3+ sections stays chunked (as 1-2 wider
            // batches) rather than collapsing to a whole-Decl slab.
            let groups = collect_struct_blank_line_groups(body, source);
            if groups.len() < AGGREGATE_STRUCT_MIN_GROUPS {
                return Vec::new();
            }
            coalesce_crumb_groups(groups)
        }
        "enumerator_list" => collect_enum_chunks(body),
        _ => Vec::new(),
    }
}

/// Configuration structs often use explanatory comment blocks instead of
/// blank lines to divide semantic field groups. For a qualifying header only,
/// let those blocks split an otherwise-atomic large struct. Existing ordinary
/// aggregate splitting wins when it already found useful blank-line groups.
fn apply_configuration_surface_aggregate_splits(decls: &mut [(Node, DeclInfo)], source: &str) {
    for (node, info) in decls {
        if !info.member_groups.is_empty()
            || !matches!(info.kind, DeclKind::Aggregate | DeclKind::Typedef)
        {
            continue;
        }
        let Some(body) = find_aggregate_body(*node) else {
            continue;
        };
        info.member_groups = collect_struct_comment_groups(body, source);
    }
}

fn collect_struct_comment_groups(body: Node, source: &str) -> Vec<AggregateMemberGroup> {
    if body.kind() != "field_declaration_list" {
        return Vec::new();
    }
    let body_start = body.start_position().row;
    let body_end = body.end_position().row;
    if body_end.saturating_sub(body_start) + 1 < AGGREGATE_STRUCT_MIN_LINES {
        return Vec::new();
    }

    let lines: Vec<&str> = source.lines().collect();
    let mut groups = Vec::new();
    let mut rows = Vec::new();
    let mut previous_was_comment = false;
    for row in (body_start + 1)..body_end {
        let Some(line) = lines.get(row) else {
            continue;
        };
        let trimmed = line.trim();
        if trimmed.is_empty() {
            previous_was_comment = false;
            continue;
        }
        let comment_start = trimmed.starts_with("//") || trimmed.starts_with("/*");
        let comment_line = comment_start || trimmed.starts_with('*');
        if comment_start && !previous_was_comment && !rows.is_empty() {
            groups.push(AggregateMemberGroup {
                group_start_line: rows[0],
                rows: std::mem::take(&mut rows),
            });
        }
        rows.push(row + 1);
        previous_was_comment = comment_line;
    }
    if !rows.is_empty() {
        groups.push(AggregateMemberGroup {
            group_start_line: rows[0],
            rows,
        });
    }
    if groups.len() < AGGREGATE_STRUCT_MIN_GROUPS {
        return Vec::new();
    }
    coalesce_crumb_groups_with_min(groups, CONFIGURATION_AGGREGATE_GROUP_MIN_ROWS)
}

/// Blank-line-separated field groups for a struct/union body.
/// Standalone full-line comment rows are elided — NS struct renders
/// skip them, and comment-per-field styles (htop `Process.h`) otherwise
/// cost ~3x the field lines alone. Same-line trailing comments share a
/// row with their field and stay. Groups left empty (pure comment
/// dividers) are dropped.
fn collect_struct_blank_line_groups(body: Node, source: &str) -> Vec<AggregateMemberGroup> {
    let comment_rows = comment_only_rows(body);
    collect_blank_line_groups(body, source)
        .into_iter()
        .filter_map(|(_, rows)| {
            let rows: Vec<usize> = rows
                .into_iter()
                .filter(|row| !comment_rows.contains(&(row - 1)))
                .collect();
            let group_start_line = *rows.first()?;
            Some(AggregateMemberGroup {
                group_start_line,
                rows,
            })
        })
        .collect()
}

/// Merge adjacent groups so none carries fewer than
/// [`AGGREGATE_STRUCT_GROUP_MIN_ROWS`] content rows: a still-small
/// group absorbs the next one until it clears the minimum, and a
/// trailing crumb folds into its predecessor.
fn coalesce_crumb_groups(groups: Vec<AggregateMemberGroup>) -> Vec<AggregateMemberGroup> {
    coalesce_crumb_groups_with_min(groups, AGGREGATE_STRUCT_GROUP_MIN_ROWS)
}

fn coalesce_crumb_groups_with_min(
    groups: Vec<AggregateMemberGroup>,
    min_rows: usize,
) -> Vec<AggregateMemberGroup> {
    budget_chunk_ranges(
        groups.len(),
        |range| groups[range].iter().map(|group| group.rows.len()).sum(),
        min_rows,
        min_rows,
        |_| true,
        |_| true,
    )
    .into_iter()
    .map(|range| {
        let group_start_line = groups[range.start].group_start_line;
        let rows = groups[range]
            .iter()
            .flat_map(|group| group.rows.iter().copied())
            .collect();
        AggregateMemberGroup {
            group_start_line,
            rows,
        }
    })
    .collect()
}

/// Fixed-size enumerator chunks. Continuation rows of multi-line
/// enumerators are included. Empty when below the chunk-min.
fn collect_enum_chunks(body: Node) -> Vec<AggregateMemberGroup> {
    let mut cursor = body.walk();
    let enumerator_spans: Vec<(usize, usize)> = body
        .named_children(&mut cursor)
        .filter(|c| c.kind() == "enumerator")
        .map(|c| (c.start_position().row, c.end_position().row))
        .collect();
    if enumerator_spans.len() < AGGREGATE_ENUM_CHUNK_MIN {
        return Vec::new();
    }
    enumerator_spans
        .chunks(AGGREGATE_ENUM_CHUNK_SIZE)
        .map(|chunk| {
            let mut rows = Vec::new();
            for (start, end) in chunk {
                push_rows(&mut rows, *start, *end);
            }
            AggregateMemberGroup {
                group_start_line: chunk[0].0 + 1,
                rows,
            }
        })
        .collect()
}

/// True iff `#define X` is the back-half of a header guard — the
/// envelope that says "this is a header", not a declaration.
///
/// Matched against the file's own `#ifndef X` when the walker found a
/// wrapping guard, so the recognition is structural rather than a
/// naming convention: a mixed-case guard symbol is as much a guard as
/// an all-caps one, and left in it becomes the file's *first* roster
/// line — the cheapest thing any budget can buy there, so the first
/// token ever spent on the header buys a symbol that carries nothing.
/// The name-shape fallback (all-caps, no value) still covers guards
/// this walker did not recognize as file-wrapping.
fn is_header_guard_define(node: Node, source: &str, guard_name: Option<&str>) -> bool {
    let Some(name_node) = node.child_by_field_name("name") else {
        return false;
    };
    let name = &source[name_node.start_byte()..name_node.end_byte()];
    if name.is_empty() {
        return false;
    }
    if guard_name == Some(name) {
        return node.child_by_field_name("value").is_none();
    }
    if !name
        .chars()
        .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
    {
        return false;
    }
    node.child_by_field_name("value").is_none()
}

fn has_static_specifier(node: Node, source: &str) -> bool {
    has_storage_class(node, source, "static")
}

fn has_storage_class(node: Node, source: &str, keyword: &str) -> bool {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "storage_class_specifier" {
            let text = source[child.start_byte()..child.end_byte()].trim();
            if text == keyword {
                return true;
            }
        }
    }
    false
}

fn has_struct_union_or_enum(node: Node) -> bool {
    let mut cursor = node.walk();
    node.children(&mut cursor).any(|c| {
        matches!(
            c.kind(),
            "struct_specifier" | "union_specifier" | "enum_specifier"
        )
    })
}

/// True if a declarator recursively reaches a `function_declarator`
/// (i.e. the declaration is a function prototype).
fn has_function_declarator(node: Node) -> bool {
    let mut cursor = node.walk();
    node.children(&mut cursor).any(declarator_is_function)
}

fn declarator_is_function(node: Node) -> bool {
    match node.kind() {
        "function_declarator" => true,
        "pointer_declarator" | "parenthesized_declarator" | "init_declarator" => node
            .child_by_field_name("declarator")
            .is_some_and(declarator_is_function),
        _ => false,
    }
}

// --- value functions ----------------------------------------------------

/// C source files this walker owns: `.c` and `.h`.
fn c_source_files(dir: &Path, ctx: &WalkCtx) -> Vec<PathBuf> {
    let Ok(read_dir) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut out: Vec<PathBuf> = read_dir
        .flatten()
        .filter_map(|e| {
            // Non-following, as in `fs::files_with_any_extension`: every
            // path here gets read and parsed, so `Path::is_file`
            // resolving a link would render a file outside the walk root.
            if !e.file_type().ok()?.is_file() {
                return None;
            }
            let path = e.path();
            let name = path.file_name().and_then(|n| n.to_str())?;
            // Same filter the listing uses: a file the listing hides
            // must not come back as a content batch.
            (is_c_source_file_name(name) && !ctx.dir_filter().excludes(&path, false))
                .then_some(path)
        })
        .collect();
    out.sort();
    out
}

/// True for `.c` and `.h` filenames.
fn is_c_source_file_name(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    lower.ends_with(".c") || lower.ends_with(".h")
}

pub(crate) fn is_header_file(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
        return false;
    };
    name.to_ascii_lowercase().ends_with(".h")
}

/// True iff `file` is a header small enough, and with a real public
/// surface, to render whole in one batch (via
/// [`whole_small_header_content`]). The size-only project-wide gate
/// uses the coarser [`header_is_small_by_size`]; this is the precise
/// per-file decision. Parse trees are cached in `ctx`, so the re-parse
/// here is a cache hit.
fn whole_small_header_eligible(file: &Path, ctx: &WalkCtx) -> bool {
    if !is_header_file(file) {
        return false;
    }
    let Some((source, tree)) = parse_c(ctx, file) else {
        return false;
    };
    let non_blank = source.lines().filter(|l| !l.trim().is_empty()).count();
    if non_blank == 0 || non_blank > WHOLE_HEADER_MAX_SRC_LINES {
        return false;
    }
    // Require a real public surface — a header that's only includes /
    // a guard / a forward declaration belongs to the decomposed path
    // (its `Includes` batch), not a whole-file render.
    !find_decls(&tree, &source, file, false).is_empty()
}

/// Whole-file content for a small public-API header, or `None` if it
/// isn't [`whole_small_header_eligible`]. When `Some`, the caller emits
/// a single verbatim batch and skips the per-file decomposition.
fn whole_small_header_content(file: &Path, source: &str, ctx: &WalkCtx) -> Option<BatchContent> {
    if !whole_small_header_eligible(file, ctx) {
        return None;
    }
    whole_file_lines_content(file, source)
}

/// Damp internal headers explicitly marked non-public by `Makefile.am`.
const INTERNAL_HEADER_FACTOR: f64 = 0.4;

fn explicit_visibility_factor(file: &Path, ctx: &WalkCtx) -> f64 {
    if !is_header_file(file) {
        return 1.0;
    }
    match ctx.c_state().is_public_header(file, ctx.root()) {
        Some(false) => INTERNAL_HEADER_FACTOR,
        Some(true) | None => 1.0,
    }
}

/// Catastrophic-axis multiplier: headers carry the public API, `.c`
/// content is implementation detail.
fn header_cat_factor(file: &Path) -> f64 {
    catastrophic_tier(is_header_file(file))
}

/// The two category tiers behind [`header_cat_factor`], defined once
/// so a retune moves every caller.
fn catastrophic_tier(header_tier: bool) -> f64 {
    if header_tier { 1.15 } else { 0.55 }
}

/// Follow-up axis: headers stay neutral, `.c` content stays demoted.
fn body_fu_factor(file: &Path) -> f64 {
    if is_header_file(file) { 1.0 } else { 0.55 }
}

fn c_depth_factor(file: &Path, ctx: &WalkCtx) -> f64 {
    file_depth_factor(file, ctx, is_header_file(file))
        * secondary_root_pair_factor(file, ctx)
        * stdlib_shim_factor(file, ctx)
        * explicit_visibility_factor(file, ctx)
        * platform_port_factor(file, ctx)
}

/// Demote C content under a mirrored-sibling platform-port directory
/// (htop's per-OS `darwin/ freebsd/ …`, tinyusb's `src/portable/<vendor>/`).
/// These are N parallel reimplementations of one interface; the NS anchors
/// on the shared root API and the cheap per-port header flood otherwise
/// crowds the core out of the early budget.
const PLATFORM_PORT_FACTOR: f64 = 0.35;

fn platform_port_factor(file: &Path, ctx: &WalkCtx) -> f64 {
    let Some(dir) = file.parent() else {
        return 1.0;
    };
    if ctx.c_state().is_port_dir(dir, ctx.root(), ctx.dir_filter()) {
        PLATFORM_PORT_FACTOR
    } else {
        1.0
    }
}

/// Damp depth-1 `.c/.h` files whose stem doesn't match the repo's
/// basename when a stem-matching primary pair exists. Mirrors the Rust
/// walker's `workspace_member_value_factor`: in a flat C project
/// with `<repo>.c` + `<repo>.h` plus a sibling vendored algorithm
/// (krep `aho_corasick.*`, single-file libraries pasted next to the
/// project's own header), the primary pair is the orientation
/// surface NS authors anchor on; the secondary stem's deep decl
/// catalog is reference content for the deeper budget.
const SECONDARY_ROOT_PAIR_FACTOR: f64 = 0.5;

fn secondary_root_pair_factor(file: &Path, ctx: &WalkCtx) -> f64 {
    if ctx.depth_from_root(file) != 1 {
        return 1.0;
    }
    let Some(stem) = c_source_stem(file) else {
        return 1.0;
    };
    let Some(repo) = ctx.root().file_name().and_then(|n| n.to_str()) else {
        return 1.0;
    };
    if stem.eq_ignore_ascii_case(repo) {
        return 1.0;
    }
    // A non-eponymous root header in the top tier of the
    // header-to-header include graph is the project's spine (htop
    // `Object.h` / `Process.h` / `Meter.h`), not a vendored sidecar —
    // first-class regardless of the eponymous pair. Inert in projects
    // without an include spine (krep, sds, chibicc), where the damp
    // keeps its vendored-pair reading.
    if is_top_include_hub(file, ctx) {
        return 1.0;
    }
    // Only damp when a stem-matching primary file actually exists at
    // the root — otherwise this is a single-pair flat project (sds
    // structure) where every depth-1 file is part of the project's
    // own surface.
    let primary_present = ["c", "h"]
        .iter()
        .any(|ext| ctx.root().join(format!("{repo}.{ext}")).is_file());
    if primary_present {
        SECONDARY_ROOT_PAIR_FACTOR
    } else {
        1.0
    }
}

/// Project-name stem of a C source file. Thin wrapper over
/// [`Path::file_stem`] kept as a named helper to localize the
/// concept across `c.rs` callers.
fn c_source_stem(file: &Path) -> Option<&str> {
    file.file_stem().and_then(|s| s.to_str())
}

fn header_banner_value(file: &Path, ctx: &WalkCtx) -> f64 {
    // Headers' banner is often the canonical "what is this header"
    // signal; .c file banners are usually license boilerplate.
    let cat = if is_header_file(file) { 0.55 } else { 0.05 };
    mix_signals(cat, 0.4, 0.7, c_depth_factor(file, ctx))
}

fn includes_value(file: &Path, ctx: &WalkCtx) -> f64 {
    let cat = (0.30 * header_cat_factor(file)).min(1.0);
    mix_signals(cat, 0.55, 0.3, c_depth_factor(file, ctx))
}

fn decl_names_value(
    file: &Path,
    ctx: &WalkCtx,
    chunk: &NamesChunk,
    configuration_surface: bool,
) -> f64 {
    let cat = (0.80 * header_cat_factor(file)).min(1.0);
    let base = mix_signals(cat, 0.6, 0.35, c_depth_factor(file, ctx));
    // Roster mass only for top include hubs: their catalog chunks lose
    // the breadth race to tiny-roster siblings (htop's per-meter
    // headers) that the size-invariant value otherwise prefers.
    // Anywhere else the boost just reorders an already-scheduled chunk
    // ahead of NS tier-1 orientation content (krep, chibicc, neco) or
    // promotes big-roster type catalogs the NS ignores (tinyusb
    // pd_types.h), dragging the gated per-decl train along.
    let mass = if is_top_include_hub(file, ctx) {
        roster_mass_factor(chunk.range.len())
    } else {
        1.0
    };
    let value = base
        * names_surface_chunk_factor(chunk.index_in_group, chunk.group_chunk_count)
        * mass
        * include_centrality_factor(file, ctx);
    configuration_surface_value_floor(value, configuration_surface)
}

/// Value for a whole small header delivered in one batch. Catastrophic
/// weight matches the names-surface it replaces (0.80) — same scheduling
/// priority, so consolidating into one verbatim batch doesn't out-bid
/// the decomposition it stands in for; the gain is purely that the
/// constants / `#ifdef` shape the names-surface strips now survive.
/// Follow-up sits below `decl_value` since the whole batch also covers
/// what those per-decl rows would carry.
fn whole_file_value(file: &Path, ctx: &WalkCtx) -> f64 {
    let cat = (0.80 * header_cat_factor(file)).min(1.0);
    let fu = (0.55 * header_cat_factor(file)).min(1.0);
    mix_signals(cat, fu, 0.6, c_depth_factor(file, ctx))
}

/// Damp vendored / shim C-stdlib headers under non-root dirs.
const STDLIB_SHIM_FACTOR: f64 = 0.25;

fn stdlib_shim_factor(file: &Path, ctx: &WalkCtx) -> f64 {
    if !is_header_file(file) || ctx.depth_from_root(file) < 2 {
        return 1.0;
    }
    let Some(stem) = c_source_stem(file) else {
        return 1.0;
    };
    if is_known_c_stdlib_stem(stem) {
        STDLIB_SHIM_FACTOR
    } else {
        1.0
    }
}

fn is_known_c_stdlib_stem(stem: &str) -> bool {
    matches!(
        stem,
        "assert"
            | "complex"
            | "ctype"
            | "errno"
            | "fenv"
            | "float"
            | "inttypes"
            | "iso646"
            | "limits"
            | "locale"
            | "math"
            | "setjmp"
            | "signal"
            | "stdalign"
            | "stdarg"
            | "stdatomic"
            | "stdbool"
            | "stddef"
            | "stdint"
            | "stdio"
            | "stdlib"
            | "stdnoreturn"
            | "string"
            | "tgmath"
            | "threads"
            | "time"
            | "uchar"
            | "wchar"
            | "wctype"
    )
}

/// Configuration headers are the application's control surface. Once the
/// structural role qualifies, keep its existing configuration batches at a
/// floor seeded from the neutral-depth declaration-names tier and re-swept
/// upward on training (1073 -> 1250). This deliberately overrides generic
/// directory-depth damping: `include/` placement does not make an
/// application's user-tuned settings secondary.
const CONFIGURATION_SURFACE_VALUE_FLOOR: f64 = 1250.0;

/// A configuration header may be arbitrarily large, so its role cannot grant
/// the floor independently to every declaration and descendant batch. Reserve
/// it for at most the first two names-surface chunks, four leading declaration
/// docs, and two aggregate member groups: together they expose the header-wide
/// setting/type roster, a small annotation sample, and bounded structural
/// detail while keeping the promoted batch count constant as the header grows.
const CONFIGURATION_SURFACE_FLOORED_NAMES_CHUNKS: usize = 2;
const CONFIGURATION_SURFACE_FLOORED_DOCS: usize = 4;
const CONFIGURATION_SURFACE_FLOORED_AGGREGATE_GROUPS: usize = 2;

fn configuration_surface_names_chunk_floored(chunk_index: usize) -> bool {
    chunk_index < CONFIGURATION_SURFACE_FLOORED_NAMES_CHUNKS
}

fn configuration_surface_doc_floored(doc_index: usize) -> bool {
    doc_index < CONFIGURATION_SURFACE_FLOORED_DOCS
}

fn configuration_surface_aggregate_group_floored(group_index: usize) -> bool {
    group_index < CONFIGURATION_SURFACE_FLOORED_AGGREGATE_GROUPS
}

fn configuration_surface_value_floor(value: f64, configuration_surface: bool) -> f64 {
    if configuration_surface {
        value.max(CONFIGURATION_SURFACE_VALUE_FLOOR)
    } else {
        value
    }
}

fn decl_value(file: &Path, kind: DeclKind, ctx: &WalkCtx) -> f64 {
    let k = kind.kind_weight();
    let cat = (0.70 * k * header_cat_factor(file)).min(1.0);
    let fu = (0.85 * k * body_fu_factor(file)).min(1.0);
    mix_signals(cat, fu, 0.65, c_depth_factor(file, ctx)) * include_centrality_factor(file, ctx)
}

fn is_configuration_surface_decl_kind(kind: DeclKind) -> bool {
    matches!(
        kind,
        DeclKind::Typedef | DeclKind::Aggregate | DeclKind::Variable | DeclKind::Macro
    )
}

fn decl_doc_value(
    file: &Path,
    kind: DeclKind,
    ctx: &WalkCtx,
    configuration_surface_doc: bool,
) -> f64 {
    let k = kind.kind_weight();
    let cat = (0.20 * k * header_cat_factor(file)).min(1.0);
    let fu = (0.6 * k * body_fu_factor(file)).min(1.0);
    configuration_surface_value_floor(
        mix_signals(cat, fu, 0.8, c_depth_factor(file, ctx)),
        configuration_surface_doc,
    )
}

fn decl_body_value(file: &Path, kind: DeclKind, ctx: &WalkCtx) -> f64 {
    let k = kind.kind_weight();
    let cat = (0.30 * k * header_cat_factor(file)).min(1.0);
    let fu = (0.80 * k * body_fu_factor(file)).min(1.0);
    mix_signals(cat, fu, 0.7, c_depth_factor(file, ctx)) * include_centrality_factor(file, ctx)
}

/// Value for one slice of a chunked struct/union/enum body. Calibrated
/// lower than `decl_value` for a whole-Aggregate Decl — each group is
/// one section of the aggregate's identity, not the entire type — so
/// the scheduler still favours unchunked anchors in load-bearing files
/// over a blanket per-group sweep.
fn aggregate_member_group_value(
    file: &Path,
    kind: DeclKind,
    ctx: &WalkCtx,
    configuration_surface_group: bool,
) -> f64 {
    let k = kind.kind_weight();
    let cat = (0.45 * k * header_cat_factor(file)).min(1.0);
    let fu = (0.75 * k * body_fu_factor(file)).min(1.0);
    configuration_surface_value_floor(
        mix_signals(cat, fu, 0.45, c_depth_factor(file, ctx))
            * include_centrality_factor(file, ctx),
        configuration_surface_group,
    )
}

// --- parser -------------------------------------------------------------

fn parse_c(ctx: &WalkCtx, path: &Path) -> Option<(Arc<str>, Arc<Tree>)> {
    ctx.parse_tree(path, &tree_sitter_c::LANGUAGE.into())
}

// --- collectors ---------------------------------------------------------

/// Top-of-file `/* */` block (or run of `//` comments). Stops at the
/// first non-comment, non-blank token.
fn collect_header_banner(tree: &Tree, source: &str) -> FileLines {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut lines = Vec::new();
    for child in root.children(&mut cursor) {
        if child.kind() == "comment" {
            extend_span(&mut lines, child, source);
            continue;
        }
        break;
    }
    FileLines::new(lines)
}

/// Last-comment row of the file's HeaderBanner (`None` if no banner).
fn header_banner_end_row(tree: &Tree) -> Option<usize> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut end = None;
    for child in root.children(&mut cursor) {
        if child.kind() != "comment" {
            break;
        }
        end = Some(child.end_position().row);
    }
    end
}

/// Collect include-map lines. Top-level directive-only conditionals
/// that contain `#include`s are folded into the batch so the gate
/// directives stay legible; code-heavy conditionals (amalgamation
/// idiom) stay opaque and contribute nothing.
fn collect_includes(tree: &Tree, source: &str) -> FileLines {
    let mut lines = Vec::new();
    walk_top_level(
        tree.root_node(),
        source,
        false,
        &mut |node| match node.kind() {
            "preproc_include" => extend_span(&mut lines, node, source),
            "preproc_if" | "preproc_ifdef" => {
                let class = classify_conditional(node);
                if class.directive_only && class.include_count > 0 {
                    extend_span(&mut lines, node, source);
                }
            }
            _ => {}
        },
    );
    FileLines::new(dedup_sorted(lines))
}

/// Classification of a `preproc_if*` / `preproc_else*` block.
#[derive(Default)]
struct ConditionalClass {
    /// True iff every named child is preprocessor content (recursively).
    directive_only: bool,
    /// `#include` count summed across every branch (subtree-wide).
    include_count: usize,
}

/// Classify a `preproc_if*` / `preproc_else*` block.
fn classify_conditional(node: Node) -> ConditionalClass {
    let mut out = ConditionalClass {
        directive_only: true,
        include_count: 0,
    };
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        match child.kind() {
            "preproc_include" => out.include_count += 1,
            "preproc_def" | "preproc_function_def" | "comment" => {}
            "preproc_if" | "preproc_ifdef" | "preproc_else" | "preproc_elif"
            | "preproc_elifdef" => {
                let nested = classify_conditional(child);
                out.include_count += nested.include_count;
                out.directive_only &= nested.directive_only;
            }
            // Condition/name tokens on the `#if` / `#ifdef` itself.
            "identifier"
            | "binary_expression"
            | "parenthesized_expression"
            | "unary_expression"
            | "call_expression"
            | "preproc_defined"
            | "number_literal"
            | "char_literal"
            | "string_literal" => {}
            // Anything else (declaration, function_definition, …) means
            // the block wraps real code.
            _ => out.directive_only = false,
        }
    }
    out
}

fn collect_decl_names_from_with_global_starts(
    decls: &[(Node, DeclInfo)],
    all_starts: &std::collections::HashSet<usize>,
    src_lines: &[&str],
) -> FileLines {
    let mut full = Vec::new();
    let mut ellipses = Vec::new();
    // Skip ellipses on rows owned elsewhere — sibling-decl starts,
    // `#include` lines, comment-only lines all flag via
    // `ellipsis_line_safe` / `all_starts`.
    for (_, info) in decls {
        full.push(info.start_line);
        // The roster is often the only place an aggregate appears, and
        // its opening line alone says nothing about what the type is —
        // see [`base_object_member_line`].
        full.extend(info.base_member_line);
        let ellipsis_line = info.start_line + 1;
        if Some(ellipsis_line) != info.base_member_line
            && !all_starts.contains(&ellipsis_line)
            && ellipsis_line_safe(ellipsis_line, src_lines)
        {
            ellipses.push(ellipsis_line);
        }
    }
    FileLines::new(dedup_sorted(full)).with_ellipses(ellipses)
}

/// True iff the line is safe to claim as an ellipsis marker — not
/// owned by `Includes` or a sibling decl's leading comment.
fn ellipsis_line_safe(line: usize, src_lines: &[&str]) -> bool {
    let Some(text) = src_lines.get(line - 1) else {
        return false;
    };
    let trimmed = text.trim_start();
    // `#include` lines belong to the `Includes` batch; comment-only
    // lines belong to the next decl's `DeclDoc`.
    if trimmed.starts_with("#include")
        || trimmed.starts_with("//")
        || trimmed.starts_with("/*")
        || trimmed.starts_with('*')
    {
        return false;
    }
    true
}

/// Lines for the decl's signature/header. Fn defs get a body-elision
/// marker; other decls render whole. Aggregates with `member_groups`
/// trim to header + closer. Spans are trimmed at sibling decl starts
/// to avoid non-ancestor overlap from tree-sitter node overlap.
fn collect_decl(
    node: Node,
    info: &DeclInfo,
    source: &str,
    all_starts: &std::collections::HashSet<usize>,
) -> FileLines {
    let mut full = Vec::new();
    let mut ellipses = Vec::new();
    let start_row = node.start_position().row;
    match info.kind {
        DeclKind::FunctionDef => {
            let sig_end = trim_end_before_next_decl(signature_end_row(node), start_row, all_starts);
            push_rows(&mut full, start_row, sig_end);
            // A single-line `void foo() { ... }` has body.start_row ==
            // body.end_row; emitting an ellipsis there would land on the
            // next decl's start line and trip non-ancestor overlap.
            if let Some(body) = node.child_by_field_name("body") {
                let bs = body.start_position().row;
                let be = body.end_position().row;
                // 1-based line of the row right after sig_end.
                let ellipsis_line = sig_end + 2;
                if be > bs + 1 && !all_starts.contains(&ellipsis_line) {
                    ellipses.push(ellipsis_line);
                }
            }
        }
        DeclKind::FunctionDecl
        | DeclKind::Variable
        | DeclKind::Typedef
        | DeclKind::Aggregate
        | DeclKind::Macro
        | DeclKind::MacroFn => {
            let end_row = trim_end_before_next_decl(
                node_end_row_trimmed(node, source),
                start_row,
                all_starts,
            );
            if !info.member_groups.is_empty()
                && let Some(body) = find_aggregate_body(node)
            {
                // Chunked aggregate: keep the type header (start_row
                // through the body's opening line), the body's closing
                // brace line, and any trailing rows after the body (the
                // `;` line of a `struct foo { … };` is usually the same
                // as the closing-brace line for a bare specifier; for a
                // `typedef … { … } Name;` shape the typedef name lives
                // there).
                let body_start = body.start_position().row;
                let body_end = body.end_position().row;
                push_rows(&mut full, start_row, body_start);
                // The identity slot is what the type is built on, not
                // one field among many — see
                // [`base_object_member_line`]. `Decl` is the member
                // groups' predecessor, so binding it here makes it
                // impossible to render the type header without it.
                full.extend(info.base_member_line);
                if body_end > body_start && body_end <= end_row {
                    full.push(body_end + 1);
                }
                if end_row > body_end {
                    push_rows(&mut full, body_end + 1, end_row);
                }
            } else {
                push_rows(&mut full, start_row, end_row);
            }
        }
    }
    FileLines::new(dedup_sorted(full)).with_ellipses(dedup_sorted(ellipses))
}

/// Non-blank interior rows of a fn definition's `compound_statement`.
fn collect_decl_body(node: Node, src_lines: &[&str]) -> FileLines {
    let Some(body) = node.child_by_field_name("body") else {
        return FileLines::new(Vec::new());
    };
    let s = body.start_position().row;
    let e = body.end_position().row;
    if e <= s + 1 {
        return FileLines::new(Vec::new());
    }
    let mut out = Vec::new();
    extend_nonblank_rows(&mut out, src_lines, s + 1, e - 1);
    FileLines::new(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scheduler::Scheduler;
    use crate::walker::FsWalker;

    fn parse(source: &str) -> (String, Tree) {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_c::LANGUAGE.into())
            .expect("load c grammar");
        let tree = parser.parse(source, None).expect("parse");
        (source.to_string(), tree)
    }

    #[test]
    fn c_header_guard_recognized_for_various_naming_conventions() {
        let cases = &[
            "#ifndef __SDS_H\n#define __SDS_H\nint foo();\n#endif\n",
            "#ifndef KREP_H\n#define KREP_H\nint foo();\n#endif\n",
            "#ifndef AHO_CORASICK_H\n#define AHO_CORASICK_H\nint foo();\n#endif\n",
            "#ifndef H_PACKETS\n#define H_PACKETS\nint foo();\n#endif\n",
        ];
        for src in cases {
            let (source, tree) = parse(src);
            let body = header_guard_body_node(tree.root_node(), &source);
            assert!(body.is_some(), "header guard not detected in:\n{src}");
            let decls = find_decls(&tree, &source, std::path::Path::new("test.h"), true);
            assert_eq!(
                decls.len(),
                1,
                "expected the one prototype to be visible after descending the guard:\n{src}"
            );
            assert_eq!(decls[0].1.kind, DeclKind::FunctionDecl);
        }
    }

    #[test]
    fn c_mixed_case_header_guard_is_not_a_declaration() {
        // Guard symbols that aren't all-caps (htop's `HEADER_Foo`,
        // soluna's `soluna_foo_h`) are guards all the same. Left on the
        // roster the guard is its first and cheapest line, so the first
        // token spent on the header buys a symbol carrying nothing.
        let cases = &[
            "#ifndef HEADER_ZfsArcStats\n#define HEADER_ZfsArcStats\nint foo();\n#endif\n",
            "#ifndef soluna_version_h\n#define soluna_version_h\nint foo();\n#endif\n",
        ];
        for src in cases {
            let (source, tree) = parse(src);
            let decls = find_decls(&tree, &source, std::path::Path::new("test.h"), true);
            assert_eq!(
                decls.iter().map(|(_, i)| i.kind).collect::<Vec<_>>(),
                vec![DeclKind::FunctionDecl],
                "the guard's `#define` is envelope, not a macro decl:\n{src}"
            );
        }
        // Only the guard's own symbol is envelope: another mixed-case
        // `#define` beside it is a real macro and keeps its place.
        let (source, tree) = parse("#ifndef FOO_H\n#define FOO_H\n#define Foo_Debug\n#endif\n");
        let decls = find_decls(&tree, &source, std::path::Path::new("test.h"), true);
        assert_eq!(decls.len(), 1);
        assert_eq!(decls[0].1.start_line, 3);
    }

    /// `base_member_line` of the sole top-level decl in `src`.
    fn base_member_line_of(src: &str) -> Option<usize> {
        let (source, tree) = parse(src);
        let decls = find_decls(&tree, &source, std::path::Path::new("test.h"), true);
        decls[0].1.base_member_line
    }

    #[test]
    fn c_leading_composite_member_is_the_types_identity_slot() {
        // The vtable idiom's embedded supertype and a tagged union's
        // discriminant both say what the type *is*.
        assert_eq!(
            base_member_line_of("struct P {\n  Row super;\n  int a;\n};\n"),
            Some(2)
        );
        assert_eq!(
            base_member_line_of("struct P {\n  struct Row_ super;\n  int a;\n};\n"),
            Some(2)
        );
        // A scalar first member is data, not identity.
        assert_eq!(
            base_member_line_of("struct P {\n  int a;\n  Row r;\n};\n"),
            None
        );
        assert_eq!(
            base_member_line_of("struct P {\n  size_t n;\n  Row r;\n};\n"),
            None
        );
        // A pointer references another object rather than embedding it.
        assert_eq!(
            base_member_line_of("struct P {\n  Row* parent;\n  int a;\n};\n"),
            None
        );
        assert_eq!(
            base_member_line_of("struct P {\n  Row rows[4];\n  int a;\n};\n"),
            None
        );
        // An inline anonymous aggregate is a definition, not an embed.
        assert_eq!(
            base_member_line_of("struct P {\n  struct { int x; } p;\n  int a;\n};\n"),
            None
        );
    }

    #[test]
    fn c_names_roster_carries_the_identity_slot() {
        // The roster is often the only place a type appears, so its
        // opening line alone would render `Process` as unrelated to the
        // hierarchy it is a leaf of.
        let src = "struct P {\n  Row super;\n  int a;\n};\nint plain(void);\n";
        let (source, tree) = parse(src);
        let decls = find_decls(&tree, &source, std::path::Path::new("test.h"), true);
        let starts: HashSet<usize> = decls.iter().map(|(_, i)| i.start_line).collect();
        let src_lines: Vec<&str> = source.lines().collect();
        let names = collect_decl_names_from_with_global_starts(&decls, &starts, &src_lines);
        assert_eq!(names.full, vec![1, 2, 5]);
        assert!(
            !names.ellipses.contains(&2),
            "the identity slot is real content, not an elision marker"
        );
    }

    fn configuration_surface_role(filename: &str, source: &str) -> bool {
        let (source, tree) = parse(source);
        let file = std::path::Path::new(filename);
        let decls = find_decls(&tree, &source, file, true);
        is_configuration_surface_header(file, &source, &tree, &decls)
    }

    #[test]
    fn c_configuration_surface_role_cases() {
        let mut documented_config = String::new();
        for i in 0..CONFIG_SURFACE_MIN_OBJECT_MACROS {
            if i % 2 == 0 {
                documented_config.push_str(&format!(
                    "// User-facing setting {i}.\n#define CFG_SETTING_{i} {i}\n"
                ));
            } else {
                documented_config.push_str(&format!(
                    "#define CFG_SETTING_{i} {i} // User-facing setting.\n"
                ));
            }
        }
        assert!(configuration_surface_role("config.h", &documented_config));
        assert!(!configuration_surface_role("config.c", &documented_config));

        let undocumented_catalog: String = (0..CONFIG_SURFACE_MIN_OBJECT_MACROS)
            .map(|i| format!("#define ITEM_KIND_{i} {i}\n"))
            .collect();
        assert!(!configuration_surface_role(
            "item_kinds.h",
            &undocumented_catalog
        ));

        let mut callable_api = documented_config.clone();
        for i in 0..=CONFIG_SURFACE_MAX_FUNCTION_DECLS {
            callable_api.push_str(&format!("int api_function_{i}(void);\n"));
        }
        assert!(!configuration_surface_role("api.h", &callable_api));

        let mut macro_minority = documented_config;
        for i in 0..(CONFIG_SURFACE_MIN_OBJECT_MACROS + 1) {
            macro_minority.push_str(&format!("extern int runtime_state_{i};\n"));
        }
        assert!(!configuration_surface_role("state_api.h", &macro_minority));
    }

    #[test]
    fn c_configuration_surface_floor_has_a_fixed_batch_bound() {
        assert!(configuration_surface_names_chunk_floored(0));
        assert!(configuration_surface_names_chunk_floored(1));
        assert!(!configuration_surface_names_chunk_floored(2));
        assert!(!configuration_surface_names_chunk_floored(usize::MAX));
        assert!(configuration_surface_doc_floored(0));
        assert!(configuration_surface_doc_floored(3));
        assert!(!configuration_surface_doc_floored(4));
        assert!(!configuration_surface_doc_floored(usize::MAX));
        assert!(configuration_surface_aggregate_group_floored(0));
        assert!(configuration_surface_aggregate_group_floored(1));
        assert!(!configuration_surface_aggregate_group_floored(2));
        assert!(!configuration_surface_aggregate_group_floored(usize::MAX));
    }

    #[test]
    fn c_includes_cases() {
        let cases: &[(&str, Vec<usize>)] = &[
            (
                // The canonical "feature-gated include map" — `#if CFG_FOO {
                // #include "x.h" }` — should be folded into the Includes batch
                // so a reader sees which optional/platform components are
                // available. The block range (gate + include + #endif) is
                // captured verbatim.
                "\
#include <stdint.h>

#if CFG_FOO_ENABLED
  #include \"foo.h\"

  #if CFG_FOO_HID
    #include \"hid_foo.h\"
  #endif
#endif

int bar(int x);
",
                vec![1, 3, 4, 5, 6, 7, 8, 9],
            ),
            (
                // A conditional that wraps real declarations (mongoose-style
                // amalgamation: `#if MG_ENABLE_HTTP { /* big impl */ }`) must
                // not be captured by Includes — only the unconditional
                // `#include` survives.
                "\
#include <stdint.h>

#if MG_ENABLE_HTTP
#include \"http_priv.h\"
int http_serve(void) { return 0; }
#endif
",
                vec![1],
            ),
            (
                // A directive-only conditional that has no `#include` inside
                // (e.g., `#ifdef __GNUC__ { #define FALLTHROUGH … }`) isn't
                // part of the include map — leave it to the per-decl Macro
                // batches.
                "\
#include <stdint.h>

#ifdef __GNUC__
  #define FALLTHROUGH __attribute__((fallthrough))
#endif
",
                vec![1],
            ),
        ];
        for (src, expected) in cases {
            let (source, tree) = parse(src);
            let fl = collect_includes(&tree, &source);
            let mut full = fl.full.clone();
            full.sort();
            assert_eq!(&full, expected, "source:\n{src}");
        }
    }

    #[test]
    fn c_feature_gated_linkage_block_stays_opaque() {
        // A non-`__cplusplus` `#ifdef`/`#ifndef` around an `extern "C"`
        // block is a feature/platform gate, not the C++-only linkage
        // idiom. The wrapped decls must NOT be surfaced as top-level
        // public API — otherwise the walker would advertise
        // platform-gated entrypoints as unconditional.
        let cases = &[
            "\
#ifdef FEATURE_X
extern \"C\" {
int gated_only(int x);
}
#endif
",
            "\
#ifndef NO_LINKAGE
extern \"C\" {
int gated_only(int x);
}
#endif
",
        ];
        for src in cases {
            let (source, tree) = parse(src);
            let decls = find_decls(&tree, &source, std::path::Path::new("test.h"), true);
            assert!(
                decls.is_empty(),
                "feature-gated linkage spec must stay opaque; got decls:\n{src}\n{decls:?}",
            );
        }
    }

    #[test]
    fn c_extern_c_block_is_descended_transparently() {
        // Bare `extern "C" { ... }` — decls must surface as top-level.
        let bare = "\
extern \"C\" {
int foo(int x);
typedef int bar_t;
}
";
        // `#ifdef __cplusplus / extern \"C\" {` envelope idiom from real
        // headers (jq.h, neco.h, tinyusb's tusb.h). tree-sitter-c parses
        // the matching `{` and `}` into a single `linkage_specification`
        // even though each lives in its own `#ifdef __cplusplus` block.
        let wrapped = "\
#ifdef __cplusplus
extern \"C\" {
#endif

int foo(int x);
typedef int bar_t;

#ifdef __cplusplus
}
#endif
";
        for src in &[bare, wrapped] {
            let (source, tree) = parse(src);
            let decls = find_decls(&tree, &source, std::path::Path::new("test.h"), true);
            let kinds: Vec<DeclKind> = decls.iter().map(|(_, d)| d.kind).collect();
            assert_eq!(
                kinds,
                vec![DeclKind::FunctionDecl, DeclKind::Typedef],
                "expected wrapped decls to be visible at top-level:\n{src}\ngot {decls:?}",
            );
        }
    }

    #[test]
    fn c_static_inline_is_external_in_header_internal_in_c() {
        let header = "static inline int sdslen(const char *s) { return 0; }\n";
        let (source, tree) = parse(header);
        let h_decls = find_decls(&tree, &source, std::path::Path::new("sds.h"), true);
        let c_decls = find_decls(&tree, &source, std::path::Path::new("sds.c"), true);
        assert_eq!(h_decls.len(), 1, "static inline in .h should be public");
        assert_eq!(h_decls[0].1.kind, DeclKind::FunctionDef);
        assert_eq!(h_decls[0].1.linkage, DeclLinkage::External);
        assert_eq!(c_decls.len(), 1, "static in .c is surfaced, not dropped");
        assert_eq!(c_decls[0].1.linkage, DeclLinkage::Internal);
        assert!(
            find_decls(&tree, &source, std::path::Path::new("sds.c"), false).is_empty(),
            "internal-linkage decls stay hidden when the file doesn't admit them"
        );
    }

    #[test]
    fn c_static_non_inline_stays_hidden_in_a_header() {
        let (source, tree) = parse("static int helper(void) { return 0; }\n");
        assert!(
            find_decls(&tree, &source, std::path::Path::new("api.h"), true).is_empty(),
            "a non-inline static definition in a header is an implementation leak"
        );
    }

    #[test]
    fn c_internal_decls_sort_after_external_ones_and_chunk_separately() {
        let src = "\
static int helper_a(void) { return 0; }
int api_one(void);
static int helper_b(void) { return 1; }
int api_two(void);
";
        let (source, tree) = parse(src);
        let decls = find_decls(&tree, &source, std::path::Path::new("prog.c"), true);
        let starts: Vec<usize> = decls.iter().map(|(_, d)| d.start_line).collect();
        assert_eq!(
            starts,
            vec![2, 4, 1, 3],
            "external group first, each group in source order"
        );
        let external_count = decls
            .iter()
            .filter(|(_, d)| d.linkage == DeclLinkage::External)
            .count();
        let chunks = names_chunks(external_count, decls.len());
        assert_eq!(chunks.len(), 2, "one chunk per linkage group");
        assert!(
            chunks
                .iter()
                .all(|c| c.index_in_group == 0 && c.group_chunk_count == 1),
            "each group's chunk series is ranked independently"
        );
    }

    #[test]
    fn c_top_level_typedef_struct_function_decl_classified() {
        let src = "\
typedef char *sds;
struct sdshdr8 { unsigned char flags; };
sds sdsnew(const char *init);
#define SDS_MAX_PREALLOC (1024*1024)
sds sdsnewlen(const void *init, size_t initlen) { return 0; }
";
        let (source, tree) = parse(src);
        let decls = find_decls(&tree, &source, std::path::Path::new("sds.h"), true);
        let kinds: Vec<DeclKind> = decls.iter().map(|(_, d)| d.kind).collect();
        assert_eq!(
            kinds,
            vec![
                DeclKind::Typedef,
                DeclKind::Aggregate,
                DeclKind::FunctionDecl,
                DeclKind::Macro,
                DeclKind::FunctionDef,
            ],
            "decl kinds mismatch; got {:?}\nsource:\n{src}",
            decls.iter().map(|(_, d)| d.kind).collect::<Vec<_>>()
        );
    }

    #[test]
    fn c_macro_modifier_pattern_does_not_overlap_following_decl() {
        // tree-sitter-c folds `LLCO_EXTERN` (an attribute-like macro) into
        // the following function as a type qualifier, so the
        // `function_definition` spans the macro line plus the signature
        // line. If the catch-all decl for the macro and the FunctionDef
        // both claim the signature line, the scheduler panics on
        // non-ancestor overlap. The trim in collect_decl prevents that.
        let src = "\
LLCO_EXTERN
void llco_first(void) { return; }
LLCO_EXTERN
void llco_second(void) { return; }
";
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::write(root.join("neco-mini.c"), src).unwrap();

        let scheduler = Scheduler::new(root.to_path_buf(), FsWalker, 4_000, None);
        // The whole point: this used to panic on non-ancestor overlap.
        let report = scheduler.run_with_report();
        assert!(
            !report.scheduled.is_empty(),
            "expected at least one scheduled batch"
        );

        // Stronger guarantee: spans claimed by C::Decl batches in the
        // same file are pairwise disjoint by row.
        let (source, tree) = parse(src);
        let decls = find_decls(&tree, &source, std::path::Path::new("neco-mini.c"), true);
        let all_starts: std::collections::HashSet<usize> =
            decls.iter().map(|(_, i)| i.start_line).collect();
        let mut claimed: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
        for (node, info) in &decls {
            let lines = collect_decl(*node, info, &source, &all_starts);
            for line in &lines.full {
                let prev = claimed.insert(*line, info.start_line);
                assert!(
                    prev.is_none(),
                    "line {line} claimed by decls starting at {} and {} — overlap",
                    prev.unwrap(),
                    info.start_line,
                );
            }
        }
    }

    #[test]
    fn c_real_dir_renders_seeded_files() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::write(
            root.join("foo.h"),
            "#ifndef FOO_H\n#define FOO_H\nint foo(int x);\n#endif\n",
        )
        .unwrap();
        std::fs::write(
            root.join("foo.c"),
            "#include \"foo.h\"\nint foo(int x) { return x + 1; }\n",
        )
        .unwrap();

        let scheduler = Scheduler::new(root.to_path_buf(), FsWalker, 4_000, None);
        let report = scheduler.run_with_report();
        let rendered = report.tree.render();
        assert!(
            rendered.contains("int foo(int x)"),
            "expected the C prototype/signature in rendered output:\n{rendered}",
        );

        let keys: Vec<_> = report.scheduled.iter().map(|r| r.key.clone()).collect();
        let has_decl_names = keys
            .iter()
            .any(|k| matches!(k, BatchKey::C(CKey::DeclNames { .. })));
        assert!(
            has_decl_names,
            "expected a C::DeclNames batch; keys: {keys:?}"
        );
    }

    /// Run the scheduler on a synthetic single-file C fixture. Returns
    /// only after `run` completes — in debug builds the call panics on
    /// any non-ancestor overlap, so a successful return is the assertion.
    fn assert_c_walker_overlap_free(filename: &str, src: &str) {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::write(root.join(filename), src).unwrap();
        let scheduler = Scheduler::new(root.to_path_buf(), FsWalker, 10_000, None);
        let report = scheduler.run_with_report();
        assert!(
            !report.scheduled.is_empty(),
            "expected at least one scheduled batch for {filename}; src:\n{src}"
        );
    }

    #[test]
    fn c_walker_overlap_audit_cases() {
        let mut multichunk = String::new();
        for i in 0..30 {
            multichunk.push_str(&format!("int fn_{i}(void);\n"));
            if i == 23 {
                multichunk.push_str("// boundary doc\n");
            }
        }
        let mut macros = String::new();
        for i in 0..40 {
            macros.push_str(&format!("#define CONST_{i} {i}\n"));
        }
        let cases: Vec<(&str, String)> = vec![
            (
                "foo.h",
                // chibicc.h shape: a `#define` (a `Macro` decl) on line 1
                // followed immediately by `#include` directives. Without the
                // ellipsis-safety check, `DeclNames` would claim line 2 as an
                // ellipsis while `Includes` claims it as a real line.
                "\
#define _POSIX_C_SOURCE 200809L
#include <assert.h>
#include <stdio.h>

int foo(int x);
"
                .to_string(),
            ),
            (
                "eol.h",
                // tinyusb video.h shape: every decl carries an end-of-line
                // comment. The tree-sitter `comment` for the EOL trailer is a
                // sibling of the *next* decl, and `collect_doc_comments_above`
                // used to grab it as a `DeclDoc` — which then claimed a line
                // already owned by the previous `Decl`.
                "\
typedef int alpha; // trailer
typedef int beta;  // trailer
typedef int gamma;
"
                .to_string(),
            ),
            (
                "eol-run.h",
                // Multiple consecutive EOL-trailered decls — the prev-sibling
                // walk must stop at the first EOL trailer (not fall through to
                // an earlier real doc).
                "\
// real doc for first
typedef int first; // EOL trailer
typedef int second;
"
                .to_string(),
            ),
            (
                "banner-touch.h",
                // File-top comment block followed immediately by the first
                // decl with no blank line. `HeaderBanner` claims the comments
                // and so does `DeclDoc` if it walks back into them — the two
                // are not in an ancestor relationship.
                "\
/* license */
/* brief */
typedef int x;
"
                .to_string(),
            ),
            (
                "multichunk.h",
                // > 24 decls forces `DeclNames` to chunk. A doc comment line
                // sitting between two chunks must not be claimed both by the
                // previous chunk's ellipsis and by the next chunk's first
                // decl's `DeclDoc`.
                multichunk,
            ),
            (
                "macros.h",
                // Long run of single-line `#define`s. `DeclNames` ellipsis
                // would otherwise land on each subsequent decl's start row.
                macros,
            ),
        ];
        for (filename, src) in cases {
            assert_c_walker_overlap_free(filename, &src);
        }
    }

    /// `Makefile.am`'s `include_HEADERS` line, with `$(srcdir)/` refs
    /// and `\` line continuations, parses to the set of declared
    /// public-API header paths. `dist_` / `nodist_` / `nobase_`
    /// automake prefixes on the installing variable are honored too.
    #[test]
    fn c_include_headers_parsed_from_makefile_am() {
        let tmp = tempfile::tempdir().expect("tmpdir");
        let root = tmp.path();
        std::fs::create_dir_all(root.join("src")).expect("mkdir");
        for name in ["jv.h", "jq.h", "dist.h", "pub.h", "jv_private.h"] {
            std::fs::write(root.join("src").join(name), "").expect("write header");
        }
        std::fs::write(
            root.join("Makefile.am"),
            "AM_CFLAGS = -Wall\n\
             include_HEADERS = src/jv.h \\\n\
                               src/jq.h\n\
             dist_include_HEADERS = src/dist.h\n\
             nodist_pkginclude_HEADERS = src/pub.h\n\
             nobase_include_HEADERS = $(srcdir)/src/jv.h\n",
        )
        .expect("write Makefile.am");
        let set = parse_include_headers(root).expect("set");
        assert_eq!(set.len(), 4);
        assert!(set.contains(&root.join("src/jv.h").canonicalize().unwrap()));
        assert!(set.contains(&root.join("src/jq.h").canonicalize().unwrap()));
        assert!(set.contains(&root.join("src/dist.h").canonicalize().unwrap()));
        assert!(set.contains(&root.join("src/pub.h").canonicalize().unwrap()));
        assert!(!set.contains(&root.join("src/jv_private.h").canonicalize().unwrap()));
    }

    /// No `Makefile.am`, or one without an `include_HEADERS` line,
    /// returns `None` — internal-header demotion must not fire on
    /// projects with no public-API declaration.
    #[test]
    fn c_include_headers_absent_returns_none() {
        let tmp = tempfile::tempdir().expect("tmpdir");
        assert!(parse_include_headers(tmp.path()).is_none());
        std::fs::write(tmp.path().join("Makefile.am"), "AM_CFLAGS = -Wall\n").expect("write");
        assert!(parse_include_headers(tmp.path()).is_none());
    }

    #[test]
    fn c_builds_a_program_ignores_test_and_example_mains() {
        let line = "int x;\n";
        let main_def = "int main(int argc, char **argv) { return 0; }\n";
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::write(root.join("lib.c"), line.repeat(10)).unwrap();
        std::fs::create_dir_all(root.join("examples/simple")).unwrap();
        std::fs::write(root.join("examples/simple/demo.c"), main_def).unwrap();
        std::fs::create_dir(root.join("tests")).unwrap();
        std::fs::write(root.join("tests/harness.c"), main_def).unwrap();
        let scan = scan_c_project(root, &DirFilter::unfiltered(root)).unwrap();
        assert!(
            !scan.builds_a_program(root),
            "example and test programs demonstrate a library, they aren't it"
        );

        std::fs::write(root.join("cli.c"), main_def).unwrap();
        let scan = scan_c_project(root, &DirFilter::unfiltered(root)).unwrap();
        assert!(scan.builds_a_program(root));
    }

    fn aggregate_group_count(filename: &str, src: &str) -> usize {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::write(root.join(filename), src).unwrap();
        let scheduler = Scheduler::new(root.to_path_buf(), FsWalker, 10_000, None);
        let report = scheduler.run_with_report();
        report
            .scheduled
            .iter()
            .filter(|r| matches!(&r.key, BatchKey::C(CKey::AggregateMemberGroup { .. })))
            .count()
    }

    #[test]
    fn c_aggregate_chunking_cases() {
        let mut three_groups = String::from("struct Obj {\n");
        for i in 0..10 {
            three_groups.push_str(&format!("  int field_a_{i};\n"));
        }
        three_groups.push('\n');
        for i in 0..10 {
            three_groups.push_str(&format!("  int field_b_{i};\n"));
        }
        three_groups.push('\n');
        for i in 0..10 {
            three_groups.push_str(&format!("  int field_c_{i};\n"));
        }
        three_groups.push_str("};\n");

        let mut crumb_groups = String::from("struct Crumbs {\n");
        for g in 0..15 {
            for i in 0..2 {
                crumb_groups.push_str(&format!("  int crumb_{g}_{i};\n"));
            }
            crumb_groups.push('\n');
        }
        crumb_groups.push_str("};\n");

        let mut wide_plus_crumbs = String::from("struct WidePlusCrumbs {\n");
        for i in 0..28 {
            wide_plus_crumbs.push_str(&format!("  int w_{i};\n"));
        }
        for g in 0..2 {
            wide_plus_crumbs.push('\n');
            for i in 0..2 {
                wide_plus_crumbs.push_str(&format!("  int c_{g}_{i};\n"));
            }
        }
        wide_plus_crumbs.push_str("};\n");

        let mut alternating = String::from("struct Alternating {\n");
        for (g, width) in [20, 2, 20, 2].into_iter().enumerate() {
            if g > 0 {
                alternating.push('\n');
            }
            for i in 0..width {
                alternating.push_str(&format!("  int f_{g}_{i};\n"));
            }
        }
        alternating.push_str("};\n");

        let mut two_groups = String::from("struct Pair {\n");
        for i in 0..20 {
            two_groups.push_str(&format!("  int a_{i};\n"));
        }
        two_groups.push('\n');
        for i in 0..20 {
            two_groups.push_str(&format!("  int b_{i};\n"));
        }
        two_groups.push_str("};\n");

        let mut big_enum = String::from("typedef enum {\n");
        for i in 0..40 {
            big_enum.push_str(&format!("  ND_{i},\n"));
        }
        big_enum.push_str("} NodeKind;\n");

        let mut small_enum = String::from("typedef enum {\n");
        for i in 0..(AGGREGATE_ENUM_CHUNK_MIN - 1) {
            small_enum.push_str(&format!("  K_{i},\n"));
        }
        small_enum.push_str("} SmallKind;\n");

        let cases: Vec<(&str, String, usize)> = vec![
            (
                // chibicc-style: large struct with ≥3 blank-line-separated field
                // groups. Each group becomes an AggregateMemberGroup; the parent
                // Decl is trimmed to the header + closer so the two render
                // disjoint body rows.
                "obj.h",
                three_groups,
                3,
            ),
            (
                // htop-style crumb confetti: 15 two-line groups coalesce
                // (2 -> 4 -> 6 rows, three source groups per batch) into 5
                // groups of AGGREGATE_STRUCT_GROUP_MIN_ROWS+ rows each.
                "crumbs.h",
                crumb_groups,
                5,
            ),
            (
                // Gate-then-coalesce boundary, 3 raw groups -> 1: the
                // [28,2,2] layout passes the 3-group gate on raw
                // groups, then merges (2+2 -> 4, trailing fold) into
                // one 32-row batch instead of losing chunking.
                "wide_plus_crumbs.h",
                wide_plus_crumbs,
                1,
            ),
            (
                // Gate-then-coalesce boundary, 4 raw groups -> 2:
                // [20,2,20,2] -> [20,22,2] -> trailing fold [20,24].
                "alternating.h",
                alternating,
                2,
            ),
            (
                // Two-group struct: below the chunking threshold (3 groups),
                // emitted as a single whole-aggregate Decl.
                "pair.h", two_groups, 0,
            ),
            (
                // chibicc-style NodeKind: 40 enumerators. The enum body splits
                // into AGGREGATE_ENUM_CHUNK_SIZE-sized chunks.
                "nodekind.h",
                big_enum,
                40_usize.div_ceil(AGGREGATE_ENUM_CHUNK_SIZE),
            ),
            (
                // Below AGGREGATE_ENUM_CHUNK_MIN enumerators: a single whole-Decl
                // batch with no chunking.
                "small.h", small_enum, 0,
            ),
        ];
        for (filename, src, expected) in cases {
            let group_count = aggregate_group_count(filename, &src);
            assert_eq!(
                group_count, expected,
                "expected {expected} AggregateMemberGroup batches for {filename}",
            );
        }
    }

    #[test]
    fn c_configuration_struct_uses_comment_boundaries() {
        let mut src = String::from("typedef struct {\n");
        for i in 0..20 {
            src.push_str(&format!("  int core_{i};\n"));
        }
        for (section, comment_rows) in [("cursor", 2), ("flags", 2), ("modes", 6)] {
            for i in 0..comment_rows {
                src.push_str(&format!("  // {section} explanation {i}\n"));
            }
            src.push_str(&format!("  int {section};\n"));
        }
        src.push_str("} ConfigState;\n");

        let (source, tree) = parse(&src);
        let decls = find_decls(&tree, &source, std::path::Path::new("config.h"), true);
        let body = find_aggregate_body(decls[0].0).unwrap();
        assert!(collect_aggregate_member_groups(body, &source).is_empty());
        let groups = collect_struct_comment_groups(body, &source);
        assert_eq!(groups.len(), 2);
        assert!(groups.iter().flat_map(|group| &group.rows).any(|line| {
            source
                .lines()
                .nth(line - 1)
                .unwrap()
                .contains("explanation")
        }));
    }

    #[test]
    fn c_aggregate_big_enum_preserves_multiline_enumerator_rows() {
        // Multi-line enumerator values (`FOO = (1 << 20)\n  | (1 << 21),`)
        // must contribute every row to the chunk that owns them — the
        // trimmed parent Decl no longer covers them, so dropping
        // continuation rows would silently truncate the enum.
        let mut src = String::from("typedef enum {\n");
        for i in 0..(AGGREGATE_ENUM_CHUNK_MIN + 2) {
            // Every other enumerator has a multi-line value.
            if i % 2 == 0 {
                src.push_str(&format!(
                    "  K_{i} = (1 << {i})\n         | (1 << {}),\n",
                    i + 1
                ));
            } else {
                src.push_str(&format!("  K_{i},\n"));
            }
        }
        src.push_str("} Multi;\n");
        let (source, tree) = parse(&src);
        let decls = find_decls(&tree, &source, std::path::Path::new("multi.h"), true);
        let enum_decl = decls
            .iter()
            .find(|(_, d)| d.kind == DeclKind::Typedef && !d.member_groups.is_empty())
            .expect("multi-line enum should be chunked");
        let mut all_rows: Vec<usize> = Vec::new();
        for group in &enum_decl.1.member_groups {
            all_rows.extend(&group.rows);
        }
        all_rows.sort();
        all_rows.dedup();
        // Continuation rows: enumerator i (even) starts at line 2+2i and
        // ends at 2+2i+1. Every row between body open and close should
        // appear in some group.
        let multiline_continuations: Vec<usize> = (0..(AGGREGATE_ENUM_CHUNK_MIN + 2))
            .filter(|i| i % 2 == 0)
            .map(|i| {
                // Lines are 1-based; the typedef header is line 1, first
                // enumerator starts at line 2. Each multi-line enumerator
                // before this one consumed 2 lines, each single-line one
                // consumed 1. K_i starts at: 2 + sum over j<i of (j%2==0 ? 2 : 1).
                let prev_lines: usize = (0..i).map(|j| if j % 2 == 0 { 2 } else { 1 }).sum();
                // continuation row = first row + 1.
                2 + prev_lines + 1
            })
            .collect();
        for cont in &multiline_continuations {
            assert!(
                all_rows.contains(cont),
                "continuation row {cont} missing from chunked rows {all_rows:?}",
            );
        }
    }

    /// `+=` appends and `$(VAR)` expansions would leave an incomplete
    /// public set if parsed naively; both must fail open so a real
    /// public header isn't silently demoted to internal.
    #[test]
    fn c_include_headers_fail_open_on_unsupported_syntax() {
        for body in [
            "include_HEADERS = src/jv.h\ninclude_HEADERS += src/jq.h\n",
            "include_HEADERS = src/jv.h $(EXTRA_API_HEADERS)\n",
            "include_HEADERS = src/jv.h ${EXTRA_API_HEADERS}\n",
            "include_HEADERS += src/jv.h\n",
            "dist_include_HEADERS = src/jv.h $(EXTRA_API_HEADERS)\n",
            "nodist_pkginclude_HEADERS += src/jv.h\n",
        ] {
            let tmp = tempfile::tempdir().expect("tmpdir");
            std::fs::write(tmp.path().join("Makefile.am"), body).expect("write");
            assert!(
                parse_include_headers(tmp.path()).is_none(),
                "expected fail-open for:\n{body}",
            );
        }
    }
}
