use crate::app::App;
use crate::utils::cache::TextureCache;
use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::ttf::Font;
use sdl2::video::{Window, WindowContext};

use std::cell::RefCell;
use std::rc::Rc;

pub trait Render {
    fn id(&self) -> String;
    fn render(
        &mut self,
        texture_creator: &TextureCreator<WindowContext>,
        cache: &mut TextureCache,
        font: &Font,
        canvas: &mut Canvas<Window>,
        rect: Rect,
        elapsed: u128,
    );
}

pub trait EventConsumer {
    fn consume_event(&mut self, event: &Event, app: Rc<RefCell<App>>);
}
