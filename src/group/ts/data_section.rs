//! JSON / TOML / YAML data section kinds.

use super::kind::TsGroupKindMethods;
use crate::group::{Group, TsGroup, TsGroupKey, TsItem};
use crate::render::LineEntry;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct DataSection;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct DataSectionBody;

impl TsGroupKindMethods for DataSection {
    fn render_item<'s>(&self, item: &TsItem<'s>) -> Vec<LineEntry<'s>> {
        super::render_data_section_line(item)
    }

    fn children<'s>(&self, parent: &TsGroup<'s>) -> Vec<Group<'s>> {
        let mut out = Vec::new();
        super::spawn_simple_child(
            &mut out,
            parent,
            TsGroupKey::DataSectionBody(DataSectionBody),
        );
        out
    }
}

impl TsGroupKindMethods for DataSectionBody {
    fn render_item<'s>(&self, item: &TsItem<'s>) -> Vec<LineEntry<'s>> {
        super::render_data_section_body_lines(item)
    }
}
