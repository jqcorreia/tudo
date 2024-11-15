use std::{cell::UnsafeCell, collections::HashMap, process::Command};

use sdl2::ttf::{Font, Sdl2TtfContext};

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct FontConfig {
    pub alias: String,
    pub family: String,
    pub point_size: u16,
}

pub struct FontManager<'a> {
    pub ttf: &'a Sdl2TtfContext,
    file_map: HashMap<String, String>,
    pub cache: UnsafeCell<HashMap<String, Font<'a, 'a>>>,
}

fn process_font<'a>(
    file_map: HashMap<String, String>,
    fconfig: FontConfig,
    ttf: &'a Sdl2TtfContext,
) -> Font<'a, 'a> {
    dbg!(&fconfig.family);
    let path = file_map.get(&fconfig.family).unwrap();
    ttf.load_font(path, fconfig.point_size).unwrap()
}

impl<'a> FontManager<'_> {
    pub fn new(ttf: &'a Sdl2TtfContext) -> FontManager<'a> {
        let mut file_map: HashMap<String, String> = HashMap::new();
        let fc_list = Command::new("fc-list").output();

        for line in String::from_utf8(fc_list.unwrap().stdout).unwrap().lines() {
            if !line.contains("style=Regular") {
                continue;
            }
            let split = line.split(":").collect::<Vec<&str>>();
            let path = split.get(0).unwrap();
            let family_names = split.get(1).unwrap();
            for family in family_names.split(",") {
                file_map.insert(family.trim().to_string(), path.to_string());
            }
        }
        FontManager {
            file_map,
            ttf,
            cache: HashMap::new().into(),
        }
    }

    pub fn get_font(&'a self, alias: impl AsRef<str>) -> &'a Font<'a, 'a> {
        let map = self.cache.get();

        unsafe { (*map).get(alias.as_ref()).unwrap() }
    }

    // Use interior mutability in order to have a shared reference &self be able to mutate the
    // inner hashmap
    pub fn load_font(&'a self, font_config: FontConfig) -> &'a Font<'a, 'a> {
        let key = font_config.clone();

        // SAFETY this is pulled from FrozenMap implementation at https://docs.rs/elsa/latest/src/elsa/map.rs.html#74
        // Still not sure how this works
        let ret = unsafe {
            let map = self.cache.get();
            &*(*map)
                .entry(key.alias)
                .or_insert_with(|| process_font(self.file_map.clone(), font_config, self.ttf))
        };
        ret
    }
}
