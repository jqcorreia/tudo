use std::env;

use super::{Action, RunAction, Source, SourceItem};

pub struct Secrets {
    calculated_items: Vec<SourceItem>,
}

impl Secrets {
    pub fn new() -> Secrets {
        Secrets {
            calculated_items: Vec::new(),
        }
    }
}

impl Source for Secrets {
    fn calculate_items(&mut self) {
        let mut res: Vec<SourceItem> = Vec::new();

        let home_path = format!("{}/.password-store", env::var("HOME").unwrap());
        match std::fs::read_dir(home_path) {
            Ok(dir) => {
                for file in dir {
                    let title = file.unwrap().file_name().into_string().unwrap();
                    res.push(SourceItem {
                        title: title.clone(),
                        action: Action::Run(RunAction {
                            path: format!("pass {}", &title).to_string(),
                            clip_output: true,
                            exit_after: true,
                        }),
                        icon: None,
                    });
                }
            }
            Err(_) => println!("No password store folder was found."),
        };
        self.calculated_items = res;
    }
    fn items(&self) -> &Vec<SourceItem> {
        return &self.calculated_items;
    }
}
