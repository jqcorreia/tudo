use std::fs;

use crate::{
    sources::{Source, SourceItem},
    utils::xdg::{parse_ini_file, IconFinder},
};

use super::{Action, RunAction};

pub struct DesktopApplications {
    calculated_items: Vec<SourceItem>,
}

impl DesktopApplications {
    pub fn new() -> DesktopApplications {
        DesktopApplications {
            calculated_items: Vec::new(),
        }
    }
}
impl Source for DesktopApplications {
    fn calculate_items(&mut self) {
        let mut res: Vec<SourceItem> = Vec::new();

        let icon_finder = IconFinder::new();
        for path in [
            "/usr/share/applications",
            "/home/jqcorreia/.local/share/applications",
        ] {
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
                let desk_entry = parse_ini_file(file.to_string());

                let title = desk_entry.get("Desktop Entry").unwrap().get("Name");
                let action = desk_entry.get("Desktop Entry").unwrap().get("Exec");

                let icon = match desk_entry.get("Desktop Entry").unwrap().get("Icon") {
                    Some(_icon) => icon_finder.get_icon(_icon.to_string()),
                    None => None,
                };

                if title.is_some() && action.is_some() {
                    res.push(SourceItem {
                        icon,
                        title: title.unwrap().to_string(),
                        action: Action::Run(RunAction {
                            path: action.unwrap().to_string(),
                            exit_after: true,
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
        self.calculated_items = res;
    }
    fn items(&self) -> &Vec<SourceItem> {
        return &self.calculated_items;
    }
}
