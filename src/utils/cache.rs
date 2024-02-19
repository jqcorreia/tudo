use sdl2::{render::TextureCreator, ttf::Sdl2TtfContext, video::WindowContext};

use super::{font::FontManager, image::ImageCache};

pub struct TextureCache<'a> {
    pub images: ImageCache<'a>,
    pub fonts: FontManager<'a>,
}

impl<'a> TextureCache<'a> {
    pub fn new(tc: &'a TextureCreator<WindowContext>, ttf: &'a Sdl2TtfContext) -> Self {
        TextureCache {
            images: ImageCache::new(tc),
            fonts: FontManager::new(ttf),
        }
    }
}
