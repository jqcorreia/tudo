use crate::sources::SourceItem;

use super::{list::SelectList, text::Prompt};

pub enum Component {
    Prompt(Prompt),
    SelectList(SelectList<SourceItem>),
}
