use sdl2::{render::TextureCreator, video::WindowContext};

use super::{atlas::FontAtlas, image::ImageCache};

pub struct TextureCache<'a> {
    pub font: FontAtlas<'a>,
    pub images: ImageCache<'a>,
}

impl<'a> TextureCache<'a> {
    pub fn new(tc: &'a TextureCreator<WindowContext>) -> Self {
        TextureCache {
            font: FontAtlas::new(tc),
            images: ImageCache::new(tc),
        }
    }
}
