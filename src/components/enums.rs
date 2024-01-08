use std::fmt::Debug;

use crate::sources::SourceItem;

use super::{list::SelectList, text::Prompt, traits};
use enum_downcast::EnumDowncast;

#[derive(EnumDowncast)]
pub enum Component {
    Prompt(Prompt),
    SelectList(SelectList<SourceItem>),
}

impl Component {
    pub fn component_trait(&self) -> &dyn traits::Component {
        match self {
            Component::Prompt(prompt) => prompt,
            Component::SelectList(list) => list,
        }
    }
}

impl Debug for Component {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("component")
            .field("x", &"as".to_string())
            .finish()
    }
}
