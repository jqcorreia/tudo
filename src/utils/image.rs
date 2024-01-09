use sdl2::{
    image::LoadTexture,
    render::{Texture, TextureCreator},
    video::WindowContext,
};

use std::{collections::HashMap, fs};
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct ImageKey {
    pub path: String,
}

pub struct ImageCache<'fa> {
    pub cache: HashMap<ImageKey, Texture<'fa>>,
    tc: &'fa TextureCreator<WindowContext>,
}

impl<'fa> ImageCache<'fa> {
    pub fn new(tc: &'fa TextureCreator<WindowContext>) -> Self {
        ImageCache {
            cache: HashMap::new(),
            tc,
        }
    }
    pub fn generate_new_texture(&mut self, path: String) -> &Texture {
        dbg!(&path);
        let buf = fs::read(&path).unwrap();

        let tex: Texture<'fa> = self.tc.load_texture_bytes(&buf).unwrap();

        let key = ImageKey { path };
        self.cache.insert(key.clone(), tex);
        self.cache.get(&key).unwrap()
    }

    pub fn get_image(&mut self, path: String) -> &Texture {
        let key = ImageKey { path: path.clone() };

        let mut new = false;
        if let None = self.cache.get(&key) {
            new = true
        }
        if new {
            self.generate_new_texture(path)
        } else {
            self.cache.get(&key).unwrap()
        }
    }
}
