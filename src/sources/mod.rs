use xcb::x::Window;

pub mod apps;
pub mod windows;

#[derive(Debug, Clone, PartialEq)]
pub struct RunAction {
    pub path: String,
    pub exit_after: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WindowSwitchAction {
    pub window: Window,
    pub exit_after: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Run(RunAction),
    WindowSwitch(WindowSwitchAction),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SourceItem {
    pub icon: Option<String>,
    pub title: String,
    pub action: Action,
}

pub trait Source {
    fn calculate_items(&mut self);
    fn items(&self) -> &Vec<SourceItem>;
}
