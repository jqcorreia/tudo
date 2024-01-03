use std::fs;

use crate::sources::{Source, SourceItem};

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
                let contents = fs::read_to_string(file).expect("File not found");

                let mut desk_entry = false;
                let mut title: Option<String> = None;
                let mut action: Option<String> = None;
                let mut terminal: Option<String> = None;

                // Idiot parser but it works
                for line in contents.split("\n") {
                    match (line, desk_entry) {
                        ("[Desktop Entry]", false) => {
                            desk_entry = true;
                        }
                        (text, true) => {
                            // This means a section we are not interested in
                            if text.starts_with("[") {
                                desk_entry = false;
                                continue;
                            }

                            let mut split = text.split("=");
                            match (split.next(), split.next()) {
                                (Some("Name"), Some(name)) => title = Some(name.to_string()),
                                (Some("Exec"), Some(exec)) => action = Some(exec.to_string()),
                                (Some("Terminal"), Some(terminal_bool)) => {
                                    terminal = Some(terminal_bool.to_string())
                                }
                                _ => (),
                            };
                        }
                        _ => (),
                    };
                }

                if title.is_some() && action.is_some() {
                    res.push(SourceItem {
                        icon: Some(
                            "/usr/share/icons/Adwaita/16x16/devices/audio-headphones.png"
                                .to_string(),
                        ),
                        title: title.unwrap(),
                        action: action.unwrap(),
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
