use std::process::Command;

use crate::sources::actions::{Action, RunAction};

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
    fn calculate_items(&mut self) {
        let mut res: Vec<SourceItem> = Vec::new();

        let output = Command::new("sh").args(["-c", "tmux ls"]).output();
        let ot = String::from_utf8(output.unwrap().stdout).unwrap();

        for line in ot.lines() {
            let session_name = line.split(":").next().unwrap();
            res.push(SourceItem {
                title: line.to_string(),
                icon: None,
                action: Action::Run(RunAction {
                    clip_output: false,
                    exit_after: true,
                    path: format!("alacritty -e tmux new -As {}", session_name).to_string(),
                }),
            });
            dbg!(line);
        }
        self.calculated_items = res;

        // match std::fs::read_dir(home_path) {
        //     Ok(dir) => {
        //         for file in dir {
        //             // Secret name will be the file name minus the extension
        //             // Use it as the item title
        //             let filename = file.unwrap().file_name().into_string().unwrap();

        //             // Ignore hidden files and .gpg-id
        //             if filename.starts_with(".") {
        //                 continue;
        //             }

        //             let secret_name = filename.split(".gpg").next().unwrap();

        //             res.push(SourceItem {
        //                 title: secret_name.to_string(),
        //                 action: Action::PassSecret(PassSecretAction {
        //                     secret_name: secret_name.to_string(),
        //                 }),
        //                 icon: None,
        //             });
        //         }
        //     }
        //     Err(_) => println!("No password store folder was found."),
        // };
        // self.calculated_items = res;
    }

    fn items(&self) -> &Vec<SourceItem> {
        return &self.calculated_items;
    }
}
