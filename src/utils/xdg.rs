use std::{collections::HashMap, fs};

#[derive(Debug)]
pub struct Section {
    pub header: String,
    pub values: HashMap<String, String>,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct IconConfig {
    pub size: u32,
    pub name: String,
}

impl IconConfig {
    pub fn new(size: u32, name: String) -> Self {
        Self { size, name }
    }
}

pub struct IconFinder {
    map: HashMap<IconConfig, String>,
}
impl IconFinder {
    pub fn new() -> IconFinder {
        let map = generate_map();
        IconFinder { map }
    }
    pub fn get_icon(&self, name: String) -> Option<String> {
        let candidate: String;

        // First check if icon identifier is a path
        if name.starts_with("/") && fs::metadata(name.clone()).is_ok() {
            candidate = name.clone();
        } else {
            let icon_config = IconConfig { name, size: 32 };
            let opt = self.map.get(&icon_config);
            if opt.is_none() {
                return None;
            }
            candidate = self.map.get(&icon_config).unwrap().to_string();
        }

        // Check if candidate is indeed a file
        match std::fs::read(&candidate) {
            Ok(_) => Some(candidate),
            Err(_) => None,
        }
    }
}

type IniMap = HashMap<String, HashMap<String, String>>;
pub fn parse_ini_file(path: String) -> Result<IniMap, ()> {
    let contents = match std::fs::read(path) {
        Ok(c) => Ok(String::from_utf8(c).unwrap()),
        Err(_) => return Err(()),
    };
    // String::from_utf8(std::fs::read(path).unwrap()).unwrap();

    let mut header_title = "";
    let mut res: HashMap<String, HashMap<String, String>> = HashMap::new();

    for line in contents?.lines() {
        if line.starts_with("[") {
            header_title = line.get(1..line.len() - 1).unwrap();
            if res.get(header_title).is_none() {
                res.insert(header_title.to_string(), HashMap::new());
            }
        } else {
            let split = line.split_once("=");
            match split {
                Some((k, v)) => {
                    res.get_mut(header_title)
                        .unwrap()
                        .insert(k.to_string(), v.to_string());
                }
                _ => (),
            };
        }
    }

    Ok(res)
}

pub fn generate_map() -> HashMap<IconConfig, String> {
    let home = std::env::var("HOME").unwrap();
    let mut map: HashMap<IconConfig, String> = HashMap::new();
    let env_folders = std::env::var("XDG_DATA_DIRS")
        .unwrap_or(format!("/usr/share:{}/.local/share", home).to_string());

    let base_folders = env_folders.split(":");

    let themes = vec!["hicolor".to_string()];
    for theme in themes {
        // Try to find and parse the index.theme file for the theme being processed
        for base_folder in base_folders.clone().into_iter() {
            let path = format!("{}/icons/{}/index.theme", base_folder, theme);
            let ini: IniMap;

            match parse_ini_file(path.clone()) {
                Ok(i) => ini = i,
                Err(_) => continue,
            };

            let dirs: Vec<String> = ini
                .get("Icon Theme")
                .unwrap()
                .get("Directories")
                .unwrap()
                .split(",")
                .map(|x| x.to_string())
                .collect();

            // Traverse the base_folders again to include all the icons that may exist for this theme
            for base_folder in base_folders.clone().into_iter() {
                for dir in dirs.iter() {
                    let section = ini.get(dir).unwrap();
                    let size = section.get("Size").unwrap();

                    let d = format!("{}/icons/{}/{}", base_folder, theme, dir);
                    match fs::read_dir(d) {
                        Ok(files) => {
                            for file in files {
                                let fpath =
                                    file.unwrap().path().into_os_string().into_string().unwrap();
                                let fname_no_ext =
                                    fpath.split("/").last().unwrap().split(".").next().unwrap();

                                let icon_config = IconConfig {
                                    size: size.parse().unwrap(),
                                    name: fname_no_ext.to_string(),
                                };
                                map.insert(icon_config, fpath);
                            }
                        }
                        Err(_) => (),
                    }
                }
            }
            // index.theme found and processed, can exit now
            break;
        }
    }

    // Process /usr/share/pixmaps
    match fs::read_dir("/usr/share/pixmaps/") {
        Ok(files) => {
            for file in files {
                if file.as_ref().unwrap().file_type().unwrap().is_dir() {
                    continue;
                }
                let fpath = file.unwrap().path().into_os_string().into_string().unwrap();
                let fname_no_ext = fpath.split("/").last().unwrap().split(".").next().unwrap();

                let icon_config = IconConfig {
                    size: 32, //FIXME(quadrado): Don't use this fixed size that means nothing. Need
                    //to open the image and calculate the proper size.
                    name: fname_no_ext.to_string(),
                };
                map.insert(icon_config, fpath);
            }
        }
        Err(_) => (),
    }

    map
}
