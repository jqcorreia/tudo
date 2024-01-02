pub mod apps;

pub struct SourceItem {
    pub title: String,
    pub action: String,
}

pub trait Source {
    fn get_items(&self) -> Vec<SourceItem>;
}
