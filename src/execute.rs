use std::cell::RefCell;
use std::rc::Rc;

use crate::{sources::SourceItem, AppContext};

pub fn execute(item: &SourceItem, ctx: Rc<RefCell<AppContext>>) {
    let mut _ctx = ctx.borrow_mut();
    item.action.execute(&mut _ctx);
}
