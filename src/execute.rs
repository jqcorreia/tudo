use std::cell::RefCell;
use std::rc::Rc;

use crate::{sources::SourceItem, App};

pub fn execute(item: &SourceItem, ctx: Rc<RefCell<App>>) {
    let mut _ctx = ctx.borrow_mut();
    item.action.execute(&mut _ctx);
}
