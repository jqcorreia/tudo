pub mod apps;

#[derive(Debug, Clone)]
pub struct SourceItem {
    pub icon: Option<String>,
    pub title: String,
    pub action: String,
}

pub trait Source {
    fn calculate_items(&mut self);
    fn items(&self) -> &Vec<SourceItem>;
}
