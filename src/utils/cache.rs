use sdl2::{render::TextureCreator, video::WindowContext};

use super::image::ImageCache;

pub struct TextureCache<'a> {
    pub images: ImageCache<'a>,
}

impl<'a> TextureCache<'a> {
    pub fn new(tc: &'a TextureCreator<WindowContext>) -> Self {
        TextureCache {
            images: ImageCache::new(tc),
        }
    }
}
