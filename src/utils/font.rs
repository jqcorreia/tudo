use std::collections::HashMap;

use sdl2::ttf::{Font, Sdl2TtfContext};
use std::cell::UnsafeCell;

pub struct FontManager<'a> {
    pub ttf: &'a Sdl2TtfContext,
    cache: UnsafeCell<HashMap<(String, u16), Font<'a, 'a>>>,
}

impl<'a> FontManager<'_> {
    pub fn new(ttf: &'a Sdl2TtfContext) -> FontManager<'a> {
        FontManager {
            ttf,
            cache: UnsafeCell::new(HashMap::new()),
        }
    }
    pub fn load_font(&'a self, path: String, point_size: u16) -> &'a Font {
        let key = (path.clone(), point_size);
        unsafe {
            if self.cache.get().as_ref().unwrap().contains_key(&key) {
                println!("cache");
                return self.cache.get().as_ref().unwrap().get(&key).unwrap();
            }

            self.cache.get().as_mut().unwrap().insert(
                key.clone(),
                self.ttf.load_font(path.clone(), point_size).unwrap(),
            );
            self.cache.get().as_ref().unwrap().get(&key).unwrap()
        }
    }
}
