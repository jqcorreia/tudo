use std::fmt::Debug;

use crate::utils::cache::TextureCache;
use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::ttf::Font;
use sdl2::video::Window;

pub trait Render {
    fn id(&self) -> String;
    fn render(
        &mut self,
        tex_cache: &mut TextureCache,
        font: &Font,
        canvas: &mut Canvas<Window>,
        rect: Rect,
    );
}

pub trait EventConsumer {
    fn consume_event(&mut self, event: &Event);
}

pub trait Component: Render + EventConsumer {}

impl Debug for dyn Component {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Component").field("id", &self.id()).finish()
    }
}
