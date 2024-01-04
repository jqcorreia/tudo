use std::{collections::HashMap, fs};

#[derive(Debug)]
pub struct Section {
    pub header: String,
    pub values: HashMap<String, String>,
}

pub fn parse_ini_file(path: String) -> HashMap<String, HashMap<String, String>> {
    let contents = String::from_utf8(std::fs::read(path).unwrap()).unwrap();

    let mut header_title = "";
    let mut res: HashMap<String, HashMap<String, String>> = HashMap::new();

    for line in contents.lines() {
        if line.starts_with("[") {
            header_title = line.get(1..line.len() - 1).unwrap();
            if res.get(header_title).is_none() {
                res.insert(header_title.to_string(), HashMap::new());
            }
        } else {
            let mut split = line.split("=");
            match (split.next(), split.next()) {
                (Some(k), Some(v)) => {
                    res.get_mut(header_title)
                        .unwrap()
                        .insert(k.to_string(), v.to_string());
                }
                _ => (),
            };
        }
    }

    res
}

pub fn get_icon(name: String) -> String {
    let base_folder = "/usr/share/icons";
    let theme = "Adwaita";

    let ini = parse_ini_file(format!("{}/{}/index.theme", base_folder, theme));

    let dirs: Vec<String> = ini
        .get("Icon Theme")
        .unwrap()
        .get("Directories")
        .unwrap()
        .split(",")
        .map(|x| x.to_string())
        .collect();

    for dir in dirs.iter() {
        let d = format!("{}/{}/{}", base_folder, theme, dir);
        match fs::read_dir(d) {
            Ok(files) => {
                for file in files {
                    let file_name = file.as_ref().unwrap().file_name().into_string().unwrap();
                    println!(
                        "{} {} {}",
                        file_name,
                        file_name.split(".").next().unwrap(),
                        name
                    );
                    if file
                        .as_ref()
                        .unwrap()
                        .file_name()
                        .into_string()
                        .unwrap()
                        .split(".")
                        .next()
                        .unwrap()
                        == name
                    {
                        return file
                            .as_ref()
                            .unwrap()
                            .path()
                            .into_os_string()
                            .into_string()
                            .unwrap();
                    }
                }
            }
            Err(_) => (),
        }
    }

    "/usr/share/icons/Adwaita/16x16/devices/audio-headphones.png".to_string()
}

#[cfg(test)]
mod tests {
    use crate::utils::xdg::get_icon;

    #[test]
    fn test_get_icon() {
        dbg!(get_icon(String::from("computer")));
    }
}
