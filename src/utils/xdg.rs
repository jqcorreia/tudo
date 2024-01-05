use std::{collections::HashMap, fs};

#[derive(Debug)]
pub struct Section {
    pub header: String,
    pub values: HashMap<String, String>,
}

pub struct IconFinder {
    map: HashMap<String, String>,
}
impl IconFinder {
    pub fn new() -> IconFinder {
        let map = generate_map();
        IconFinder { map: map }
    }
    pub fn get_icon(&self, name: String) -> Option<String> {
        // First check if icon identifier is a path
        if name.starts_with("/") && fs::metadata(name.clone()).is_ok() {
            return Some(name);
        }
        self.map.get(&name).cloned()
    }
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

pub fn generate_map() -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();
    let base_folder = "/usr/share/icons";
    let theme = "hicolor";

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
                    let fpath = file.unwrap().path().into_os_string().into_string().unwrap();
                    let fname = fpath.split("/").last().unwrap().split(".").next().unwrap();

                    map.insert(fname.to_string(), fpath);
                }
            }
            Err(_) => (),
        }
    }
    map
}
pub fn get_icon(name: String) -> Option<String> {
    // First check if icon identifier is a path
    if name.starts_with("/") && fs::metadata(name.clone()).is_ok() {
        return Some(name);
    }
    let base_folder = "/usr/share/icons";
    let theme = "hicolor";

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
                        return Some(
                            file.as_ref()
                                .unwrap()
                                .path()
                                .into_os_string()
                                .into_string()
                                .unwrap(),
                        );
                    }
                }
            }
            Err(_) => (),
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::utils::xdg::get_icon;

    #[test]
    fn test_get_icon() {
        dbg!(get_icon(String::from("computer")));
    }
}
