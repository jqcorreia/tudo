use std::collections::HashMap;

use sdl2::ttf::{Font, Sdl2TtfContext};

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct FontConfig {
    pub path: String,
    pub point_size: u16,
}

pub struct FontManager<'a> {
    pub ttf: &'a Sdl2TtfContext,
    fonts: HashMap<FontConfig, Font<'a, 'a>>,
}

impl<'a> FontManager<'_> {
    pub fn new(ttf: &'a Sdl2TtfContext) -> FontManager<'a> {
        FontManager {
            ttf,
            fonts: HashMap::new(),
        }
    }
    pub fn check_font_config(&self, fconfig: FontConfig) -> bool {
        self.fonts.contains_key(&fconfig)
    }

    pub fn add_font(&mut self, fconfig: FontConfig) -> &'a Font {
        self.fonts.insert(
            fconfig.clone(),
            self.ttf
                .load_font(fconfig.path.clone(), fconfig.point_size)
                .unwrap(),
        );
        self.fonts.get(&fconfig).unwrap()
    }
}
