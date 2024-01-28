use self::actions::Action;

pub mod actions;
pub mod apps;
pub mod lua;
pub mod secrets;
pub mod tmux;
pub mod windows;

#[derive(Debug, Clone, PartialEq)]
pub struct SourceItem {
    pub icon: Option<String>,
    pub title: String,
    pub action: Action,
}

pub trait Source {
    fn calculate_items(&mut self);
    fn items(&self) -> &Vec<SourceItem>;
    fn is_async(&self) -> bool;
}
