use std::env;

use super::{actions::PassSecretAction, Source, SourceItem};

pub struct Secrets {}

impl Secrets {
    pub fn new() -> Secrets {
        Secrets {}
    }
}

impl Source for Secrets {
    fn is_async(&self) -> bool {
        false
    }
    fn generate_items(&self) -> Vec<SourceItem> {
        let mut res: Vec<SourceItem> = Vec::new();

        let home_path = format!("{}/.password-store", env::var("HOME").unwrap());

        match std::fs::read_dir(home_path) {
            Ok(dir) => {
                for file in dir {
                    // Secret name will be the file name minus the extension
                    // Use it as the item title
                    let filename = file.unwrap().file_name().into_string().unwrap();

                    // Ignore hidden files and .gpg-id
                    if filename.starts_with(".") {
                        continue;
                    }

                    let secret_name = filename.split(".gpg").next().unwrap();

                    res.push(SourceItem {
                        title: secret_name.to_string(),
                        action: Box::new(PassSecretAction {
                            secret_name: secret_name.to_string(),
                        }),
                        icon: None,
                    });
                }
            }
            Err(_) => println!("No password store folder was found."),
        };
        res
    }
}
