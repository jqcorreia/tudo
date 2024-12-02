use std::{
    collections::{HashMap, HashSet},
    fs,
};

use log::debug;

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
    sizes: HashSet<u32>,
}
impl Default for IconFinder {
    fn default() -> Self {
        Self::new()
    }
}

impl IconFinder {
    pub fn new() -> IconFinder {
        let (map, sizes) = generate_map();
        IconFinder { map, sizes }
    }
    pub fn get_icon(&self, name: String) -> Option<String> {
        let candidate: String;

        // First check if icon identifier is a path
        if name.starts_with("/") && fs::metadata(name.clone()).is_ok() {
            candidate = name.clone();
        } else {
            let icon_config = IconConfig { name, size: 32 };
            let opt = self.map.get(&icon_config);
            opt?;
            candidate = self.map.get(&icon_config).unwrap().to_string();
        }

        // Check if candidate is indeed a file
        match std::fs::read(&candidate) {
            Ok(_) => Some(candidate),
            Err(_) => None,
        }
    }

    pub fn get_icon_with_size(&self, name: String, size: u32) -> Option<String> {
        fn check_file(path: String) -> Option<String> {
            match std::fs::read(&path) {
                Ok(_) => Some(path),
                Err(_) => None,
            }
        }

        // First check if icon identifier is a path
        if name.starts_with("/") && fs::metadata(name.clone()).is_ok() {
            return check_file(name);
        }

        // Check for exact match
        if let Some(path) = self.map.get(&IconConfig {
            name: name.clone(),
            size,
        }) {
            return check_file(path.to_string());
        }

        // Scan different sizes until one appears
        // In this case we are going from largest to smallest
        // FIXME(quadrado): We can do better here to try and find the closest match
        let mut _sizes: Vec<u32> = self.sizes.clone().into_iter().collect::<Vec<u32>>();
        _sizes.sort();
        _sizes.reverse();

        for _size in _sizes {
            let icon_config = IconConfig {
                name: name.clone(),
                size: _size,
            };
            if let Some(path) = self.map.get(&icon_config) {
                return check_file(path.to_string());
            }
        }
        None
    }
}

type IniMap = HashMap<String, HashMap<String, String>>;
pub fn parse_ini_file(path: String) -> Result<IniMap, ()> {
    let contents = match std::fs::read(path) {
        Ok(c) => Ok(String::from_utf8(c).unwrap()),
        Err(_) => return Err(()),
    };

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
            if let Some((k, v)) = split {
                res.get_mut(header_title)
                    .unwrap()
                    .insert(k.to_string(), v.to_string());
            };
        }
    }

    Ok(res)
}

fn get_gtk_settings_theme() -> Option<String> {
    let home = std::env::var("HOME").unwrap();
    match parse_ini_file(format!("{}/.config/gtk-3.0/settings.ini", home)) {
        Ok(i) => i
            .get("Settings")
            .unwrap()
            .get("gtk-icon-theme-name")
            .map(|theme| theme.to_string()),
        Err(_) => match parse_ini_file(format!("{}/.config/gtk-4.0/settings.ini", home)) {
            Ok(i) => i
                .get("Settings")
                .unwrap()
                .get("gtk-icon-theme-name")
                .map(|theme| theme.to_string()),
            Err(_) => None,
        },
    }
}
fn generate_map() -> (HashMap<IconConfig, String>, HashSet<u32>) {
    let home = std::env::var("HOME").unwrap();
    let mut map: HashMap<IconConfig, String> = HashMap::new();
    let mut sizes: HashSet<u32> = HashSet::new();

    let env_folders = std::env::var("XDG_DATA_DIRS")
        .unwrap_or(format!("/usr/share:{}/.local/share", home).to_string());

    let base_folders = env_folders.split(":");

    let mut themes = vec!["hicolor".to_string()];

    if let Some(theme_name) = get_gtk_settings_theme() {
        themes.push(theme_name);
    }
    for theme in themes {
        // Try to find and parse the index.theme file for the theme being processed
        for base_folder in base_folders.clone() {
            let path = format!("{}/icons/{}/index.theme", base_folder, theme);

            let ini: IniMap = match parse_ini_file(path.clone()) {
                Ok(i) => i,
                Err(_) => continue,
            };

            let dirs: Vec<String> = ini
                .get("Icon Theme")
                .unwrap()
                .get("Directories")
                .unwrap()
                .split(",")
                .map(|x| x.to_string())
                .filter(|x| !x.is_empty())
                .collect();

            // Traverse the base_folders again to include all the icons that may exist for this theme
            for base_folder in base_folders.clone() {
                for dir in dirs.iter() {
                    if ini.get(dir).is_none() {
                        debug!("Section {} not found", dir);
                        continue;
                    }
                    let section = ini.get(dir).unwrap();
                    let size: u32 = section.get("Size").unwrap().parse().unwrap();
                    let scale = section.get("Scale").map_or("1", |v| v);

                    if scale.parse::<u32>().unwrap() > 1 {
                        //FIXME(quadrado): For now ignore scaled icons
                        continue;
                    }
                    sizes.insert(size);

                    let d = format!("{}/icons/{}/{}", base_folder, theme, dir);
                    if let Ok(files) = fs::read_dir(d) {
                        for file in files {
                            let fpath =
                                file.unwrap().path().into_os_string().into_string().unwrap();

                            let fname_no_ext = std::path::Path::new(&fpath)
                                .file_stem()
                                .unwrap()
                                .to_os_string()
                                .to_str()
                                .unwrap()
                                .to_string();

                            //fpath.split("/").last().unwrap().split(".").next().unwrap();

                            // println!("{} {}", fname_no_ext, fpath);
                            let icon_config = IconConfig {
                                size,
                                name: fname_no_ext,
                            };
                            map.insert(icon_config, fpath);
                        }
                    }
                }
            }
            // index.theme found and processed, can exit now
            break;
        }
    }

    // Process /usr/share/pixmaps
    if let Ok(files) = fs::read_dir("/usr/share/pixmaps/") {
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

    (map, sizes)
}
