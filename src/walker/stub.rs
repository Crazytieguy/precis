use std::path::PathBuf;

use crate::batch::{Batch, BatchContent, FsEntry, RenderedLine};

use super::{Walker, WalkerCtx};

/// Synthetic walker used by Stage 6 to exercise the scheduler boundary in
/// language-agnostic terms. Stays in the codebase as a permanent contract test:
/// it emits a folder listing, a file content batch with truncated-line
/// renderings, and a follow-on batch that overrides one of those lines with
/// its full form. Together these cover folder listings, file content,
/// parent-child predecessor edges, and line-level override + marginal cost.
pub struct StubWalker;

impl Walker for StubWalker {
    fn seed(&mut self, ctx: &mut WalkerCtx) -> Vec<Batch> {
        vec![Batch {
            id: ctx.alloc_id(),
            descriptor: "stub root listing".to_string(),
            content: BatchContent::FolderListing {
                path: PathBuf::from("/stub"),
                entries: vec![FsEntry {
                    name: "synthetic.rs".to_string(),
                    is_dir: false,
                }],
            },
            parent: None,
            ordering_pred: None,
            raw_value: 100.0,
        }]
    }

    fn successors(&mut self, scheduled: &Batch, ctx: &mut WalkerCtx) -> Vec<Batch> {
        match scheduled.descriptor.as_str() {
            "stub root listing" => vec![Batch {
                id: ctx.alloc_id(),
                descriptor: "synthetic.rs truncated signatures".to_string(),
                content: BatchContent::FileContent {
                    path: PathBuf::from("/stub/synthetic.rs"),
                    lines: vec![
                        RenderedLine {
                            number: 1,
                            text: "fn foo".to_string(),
                            truncated: true,
                        },
                        RenderedLine {
                            number: 2,
                            text: "fn bar".to_string(),
                            truncated: true,
                        },
                    ],
                },
                parent: Some(scheduled.id),
                ordering_pred: None,
                raw_value: 50.0,
            }],
            "synthetic.rs truncated signatures" => vec![Batch {
                id: ctx.alloc_id(),
                descriptor: "synthetic.rs full foo signature".to_string(),
                content: BatchContent::FileContent {
                    path: PathBuf::from("/stub/synthetic.rs"),
                    lines: vec![RenderedLine {
                        number: 1,
                        text: "fn foo(a: i32, b: i32) -> Result<()> {".to_string(),
                        truncated: false,
                    }],
                },
                parent: Some(scheduled.id),
                ordering_pred: None,
                raw_value: 30.0,
            }],
            _ => vec![],
        }
    }
}
