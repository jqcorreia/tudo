use std::collections::HashMap;

use sdl2::ttf::{Font, Sdl2TtfContext};

pub struct FontManager<'a> {
    pub ttf: &'a Sdl2TtfContext,
    cache: HashMap<(String, u16), Font<'a, 'a>>,
}

impl<'a> FontManager<'_> {
    pub fn new(ttf: &'a Sdl2TtfContext) -> FontManager<'a> {
        FontManager {
            ttf,
            cache: HashMap::new(),
        }
    }
    pub fn load_font(&'a mut self, path: String, point_size: u16) -> &'a Font {
        if self.cache.get(&(path.clone(), point_size)).is_some() {
            return self.cache.get(&(path, point_size)).unwrap();
        }

        let font = self.ttf.load_font(path.clone(), point_size).unwrap();

        self.cache.insert((path.clone(), point_size), font);
        self.cache.get(&(path, point_size)).unwrap()
    }
}
