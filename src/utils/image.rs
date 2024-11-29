use sdl2::{
    image::LoadTexture,
    render::{Texture, TextureCreator},
    video::WindowContext,
};

use std::cell::UnsafeCell;

use std::{collections::HashMap, fs};
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct ImageKey {
    pub path: String,
}

pub struct ImageCache<'fa> {
    pub cache: UnsafeCell<HashMap<ImageKey, Texture<'fa>>>,
    tc: &'fa TextureCreator<WindowContext>,
}

fn gen_tex(path: String, tc: &TextureCreator<WindowContext>) -> Texture<'_> {
    let buf = fs::read(&path).unwrap();

    let tex: Texture = tc.load_texture_bytes(&buf).unwrap();

    tex
}

impl<'fa> ImageCache<'fa> {
    pub fn new(tc: &'fa TextureCreator<WindowContext>) -> Self {
        ImageCache {
            cache: HashMap::new().into(),
            tc,
        }
    }

    // Use interior mutability in order to have a shared reference &self be able to mutate the
    // inner hashmap
    pub fn get_image(&self, path: String) -> &Texture {
        let key = ImageKey { path: path.clone() };

        // SAFETY this is pulled from FrozenMap implementation at https://docs.rs/elsa/latest/src/elsa/map.rs.html#74
        // Still not sure how this works
        let ret = unsafe {
            let map = self.cache.get();
            &*(*map).entry(key).or_insert_with(|| gen_tex(path, self.tc))
        };
        ret
    }
}
