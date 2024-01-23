use std::cell::RefCell;
use std::rc::Rc;

use crate::{sources::SourceItem, App};

pub fn execute(item: &SourceItem, ctx: &mut App) {
    item.action.execute(ctx);
}
