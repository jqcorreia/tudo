use crate::app::App;
use crate::utils::cache::TextureCache;
use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};

pub trait Render {
    fn id(&self) -> String;
    fn render(
        &mut self,
        texture_creator: &TextureCreator<WindowContext>,
        cache: &mut TextureCache,
        app: &App,
        canvas: &mut Canvas<Window>,
        rect: Rect,
        elapsed: u128,
    );
}

pub trait EventConsumer {
    fn consume_event(&mut self, event: &Event, app: &mut App);
}

pub trait UIComponent: Render + EventConsumer {}
