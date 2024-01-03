use sdl2::{
    image::LoadTexture,
    render::{Texture, TextureCreator},
    video::WindowContext,
};

use std::{collections::HashMap, fs};
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct IconKey {
    pub path: String,
}

pub struct IconCache<'fa> {
    pub cache: HashMap<IconKey, Texture<'fa>>,
    tc: &'fa TextureCreator<WindowContext>,
}

impl<'fa> IconCache<'fa> {
    pub fn new(tc: &'fa TextureCreator<WindowContext>) -> Self {
        IconCache {
            cache: HashMap::new(),
            tc,
        }
    }
    pub fn generate_new_texture(&mut self, path: String) -> &Texture {
        let buf = fs::read(&path).unwrap();

        let tex: Texture<'fa> = self.tc.load_texture_bytes(&buf).unwrap();

        let key = IconKey { path };
        self.cache.insert(key.clone(), tex);
        self.cache.get(&key).unwrap()
    }

    pub fn get_icon(&mut self, path: String) -> &Texture {
        let key = IconKey { path: path.clone() };

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
