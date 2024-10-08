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
        IconFinder { map }
    }
    pub fn get_icon(&self, name: String) -> Option<String> {
        let candidate: String;

        // First check if icon identifier is a path
        if name.starts_with("/") && fs::metadata(name.clone()).is_ok() {
            candidate = name.clone();
        } else {
            let opt = self.map.get(&name);
            if opt.is_none() {
                return None;
            }
            candidate = self.map.get(&name).unwrap().to_string();
        }

        // Check if candidate is indeed a file
        match std::fs::read(&candidate) {
            Ok(_) => Some(candidate),
            Err(_) => None,
        }
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

    res
}

pub fn generate_map() -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();
    let base_folder = "/usr/share/icons";
    let mut theme = "default";
    let mut themes: Vec<String> = Vec::new();

    //FIXME(quadrado): This is buggy and not being used right now. Revisit this
    // Commenting because Inherits field can have a comma separated list of values.
    //let mut ini = parse_ini_file(format!("{}/{}/index.theme", base_folder, theme));
    //loop {
    //    match ini.get("Icon Theme").unwrap().get("Inherits") {
    //        Some(th) => {
    //            themes.push(th.to_string().clone());
    //            theme = th;
    //        }
    //        None => break,
    //    }
    //    ini = parse_ini_file(format!("{}/{}/index.theme", base_folder, theme));
    //}

    themes = vec!["hicolor".to_string()];
    for theme in themes {
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
                        let fname_no_ext =
                            fpath.split("/").last().unwrap().split(".").next().unwrap();

                        map.insert(fname_no_ext.to_string(), fpath);
                    }
                }
                Err(_) => (),
            }
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

                map.insert(fname_no_ext.to_string(), fpath);
            }
        }
        Err(_) => (),
    }

    map
}
pub fn generate_map2() -> HashMap<String, String> {
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
