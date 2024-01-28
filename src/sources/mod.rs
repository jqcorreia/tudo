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
    fn generate_items(&self) -> Vec<SourceItem>;

    fn is_async(&self) -> bool; //TODO(quadrado): Use this in order async load or not, unused for
                                //now
}
