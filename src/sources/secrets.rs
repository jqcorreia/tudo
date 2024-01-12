use std::env;

use super::{Action, PassSecretAction, Source, SourceItem};

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
                    // Secret name will be the file name minus the extension
                    // Use it as the item title
                    let filename = file.unwrap().file_name().into_string().unwrap();

                    let secret_name = filename.split(".gpg").next().unwrap();

                    res.push(SourceItem {
                        title: secret_name.to_string(),
                        action: Action::PassSecret(PassSecretAction {
                            secret_name: secret_name.to_string(),
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
