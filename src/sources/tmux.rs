use std::process::Command;

use crate::sources::actions::TmuxAction;

use super::{Source, SourceItem};

pub struct Tmux {}

impl Default for Tmux {
    fn default() -> Self {
        Self::new()
    }
}

impl Tmux {
    pub fn new() -> Tmux {
        Tmux {}
    }
}

impl Source for Tmux {
    fn is_async(&self) -> bool {
        false
    }
    fn generate_items(&self) -> Vec<SourceItem> {
        let mut res: Vec<SourceItem> = Vec::new();

        let output = Command::new("sh").args(["-c", "tmux ls"]).output();
        let ot = String::from_utf8(output.unwrap().stdout).unwrap();

        for line in ot.lines() {
            let session_name = line.split(":").next().unwrap();
            res.push(SourceItem {
                title: line.to_string(),
                icon: None,
                action: Box::new(TmuxAction {
                    session: session_name.to_string(),
                }),
            });
        }
        res
    }
}
