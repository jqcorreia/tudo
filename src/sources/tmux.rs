use std::process::Command;

use crate::sources::actions::{Action, RunAction, TmuxAction};

use super::{Source, SourceItem};

pub struct Tmux {
    calculated_items: Vec<SourceItem>,
}

impl Tmux {
    pub fn new() -> Tmux {
        Tmux {
            calculated_items: Vec::new(),
        }
    }
}
impl Source for Tmux {
    fn is_async(&self) -> bool {
        false
    }
    fn calculate_items(&mut self) {
        let mut res: Vec<SourceItem> = Vec::new();

        let output = Command::new("sh").args(["-c", "tmux ls"]).output();
        let ot = String::from_utf8(output.unwrap().stdout).unwrap();

        for line in ot.lines() {
            let session_name = line.split(":").next().unwrap();
            res.push(SourceItem {
                title: line.to_string(),
                icon: None,
                action: Action::Tmux(TmuxAction {
                    session: session_name.to_string(),
                }),
            });
            dbg!(line);
        }
        self.calculated_items = res;
    }

    fn items(&self) -> &Vec<SourceItem> {
        return &self.calculated_items;
    }
}
