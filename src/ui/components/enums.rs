use std::fmt::Debug;

use crate::sources::SourceItem;

use super::{list::SelectList, text::Prompt};
use enum_downcast::EnumDowncast;

#[derive(EnumDowncast)]
pub enum Component {
    Prompt(Prompt),
    SelectList(SelectList<SourceItem>),
}

impl Debug for Component {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("component")
            .field("x", &"as".to_string())
            .finish()
    }
}
