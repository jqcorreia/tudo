use std::{thread::sleep, time::Duration};

use crate::sources::{Source, SourceItem};

pub struct DummySource {}

impl Default for DummySource {
    fn default() -> Self {
        Self::new()
    }
}

impl DummySource {
    pub fn new() -> DummySource {
        DummySource {}
    }
}

impl Source for DummySource {
    fn is_async(&self) -> bool {
        false
    }
    fn generate_items(&self) -> Vec<SourceItem> {
        sleep(Duration::new(3, 0));
        vec![]
    }
}
