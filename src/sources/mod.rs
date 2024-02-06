use self::actions::Action;

pub mod actions;
pub mod apps;
pub mod lua;
pub mod secrets;
pub mod tmux;
pub mod windows;

#[derive(Clone)]
pub struct SourceItem {
    pub icon: Option<String>,
    pub title: String,
    pub action: Box<dyn Action + Send>,
}

impl PartialEq for SourceItem {
    fn eq(&self, other: &Self) -> bool {
        self.icon == other.icon
            && self.title == other.title
            && self.action.tags() == other.action.tags()
    }
}

pub trait Source {
    fn generate_items(&self) -> Vec<SourceItem>;

    fn is_async(&self) -> bool; //TODO(quadrado): Use this in order async load or not, unused for
                                //now
}
