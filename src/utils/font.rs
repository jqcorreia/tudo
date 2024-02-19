use std::{cell::UnsafeCell, collections::HashMap};

use sdl2::ttf::{Font, Sdl2TtfContext};

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct FontConfig {
    pub path: String,
    pub point_size: u16,
}

pub struct FontManager<'a> {
    pub ttf: &'a Sdl2TtfContext,
    fonts: HashMap<FontConfig, Font<'a, 'a>>,
    pub cache: UnsafeCell<HashMap<FontConfig, Font<'a, 'a>>>,
}

fn process_font<'a>(fconfig: FontConfig, ttf: &'a Sdl2TtfContext) -> Font<'a, 'a> {
    ttf.load_font(fconfig.path.clone(), fconfig.point_size)
        .unwrap()
}

impl<'a> FontManager<'_> {
    pub fn new(ttf: &'a Sdl2TtfContext) -> FontManager<'a> {
        FontManager {
            ttf,
            fonts: HashMap::new(),
            cache: HashMap::new().into(),
        }
    }
    // Use interior mutability in order to have a shared reference &self be able to mutate the
    // inner hashmap
    pub fn get_font(&'a self, font_config: FontConfig) -> &'a Font<'a, 'a> {
        let key = font_config.clone();

        // SAFETY this is pulled from FrozenMap implementation at https://docs.rs/elsa/latest/src/elsa/map.rs.html#74
        // Still not sure how this works
        let ret = unsafe {
            let map = self.cache.get();
            &*(*map)
                .entry(key)
                .or_insert_with(|| process_font(font_config, self.ttf))
        };
        ret
    }
}
