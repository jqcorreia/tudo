use std::fs;

use crate::sources::{Source, SourceItem};

pub struct DesktopApplications {}

impl Source for DesktopApplications {
    fn get_items(&self) -> Vec<SourceItem> {
        let mut res: Vec<SourceItem> = Vec::new();

        let desktop_files = fs::read_dir("/usr/share/applications/")
            .unwrap()
            .map(|entry| {
                entry
                    .as_ref()
                    .unwrap()
                    .path()
                    .into_os_string()
                    .into_string()
                    .unwrap()
            })
            .collect::<Vec<String>>();

        for file in desktop_files.iter() {
            let contents = fs::read_to_string(file).expect("File not found");

            let mut desk_entry = false;
            let mut title: Option<String> = None;
            let mut action: Option<String> = None;

            for line in contents.split("\n") {
                match (line, desk_entry) {
                    ("[Desktop Entry]", false) => {
                        desk_entry = true;
                    }
                    (text, true) => {
                        let mut split = text.split("=");
                        match (split.next(), split.next()) {
                            (Some("Name"), Some(name)) => title = Some(name.to_string()),
                            (Some("Exec"), Some(exec)) => action = Some(exec.to_string()),
                            _ => (),
                        };
                    }
                    _ => (),
                };
            }

            if title.is_some() && action.is_some() {
                res.push(SourceItem {
                    title: title.unwrap(),
                    action: action.unwrap(),
                });
            } else {
                println!(
                    "{} file doenst have some info. {} {}",
                    file,
                    title.is_some(),
                    action.is_some()
                );
            }
        }
        return res;
    }
}
