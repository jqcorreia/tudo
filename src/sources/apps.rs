use std::fs;

use crate::{
    sources::{Source, SourceItem},
    utils::xdg::{parse_ini_file, IconFinder},
};

use super::actions::RunAction;

pub struct DesktopApplications {}

impl Default for DesktopApplications {
    fn default() -> Self {
        Self::new()
    }
}

impl DesktopApplications {
    pub fn new() -> DesktopApplications {
        DesktopApplications {}
    }
}
impl Source for DesktopApplications {
    fn is_async(&self) -> bool {
        false
    }
    fn generate_items(&self) -> Vec<SourceItem> {
        let mut res: Vec<SourceItem> = Vec::new();

        let icon_finder = IconFinder::new();
        let home = std::env::var("HOME").unwrap();
        let paths = match std::env::var("XDG_DATA_DIRS") {
            Ok(dirs) => dirs
                .split(":")
                .map(|x| format!("{}/applications", x))
                .collect::<Vec<String>>(),
            Err(_) => vec![
                "/usr/share/applications".to_string(),
                format!("/home/{}/.local/share/applications", home).to_string(),
            ],
        };

        for path in paths {
            let desktop_files = match fs::read_dir(path) {
                Ok(entries) => entries
                    .filter(|entry| {
                        entry
                            .as_ref()
                            .unwrap()
                            .file_name()
                            .into_string()
                            .unwrap()
                            .ends_with(".desktop")
                    })
                    .map(|entry| {
                        entry
                            .as_ref()
                            .unwrap()
                            .path()
                            .into_os_string()
                            .into_string()
                            .unwrap()
                    })
                    .collect::<Vec<String>>(),

                Err(_) => Vec::new(),
            };

            for file in desktop_files.iter() {
                let desk_entry = parse_ini_file(file.to_string()).unwrap();

                let title = desk_entry.get("Desktop Entry").unwrap().get("Name");
                let action = desk_entry.get("Desktop Entry").unwrap().get("Exec");

                let icon = match desk_entry.get("Desktop Entry").unwrap().get("Icon") {
                    Some(_icon) => icon_finder.get_icon_with_size(_icon.to_string(), 32),
                    None => None,
                };

                if title.is_some() && action.is_some() {
                    // Discard %U in this context since we don't have anything to interpolate
                    let sanitized_action = action.unwrap().to_string().replace("%U", "");

                    res.push(SourceItem {
                        icon,
                        title: title.unwrap().to_string(),
                        action: Box::new(RunAction {
                            path: sanitized_action,
                            exit_after: true,
                            clip_output: false,
                        }),
                    });
                } else {
                    println!(
                        "{} file doesnt have some info. {} {}",
                        file,
                        title.is_some(),
                        action.is_some()
                    );
                }
            }
        }
        res
    }
}
