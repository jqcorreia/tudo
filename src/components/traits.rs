use crate::utils::atlas::FontAtlas;
use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::ttf::Font;
use sdl2::video::Window;

pub trait Render {
    fn id(&self) -> String;
    fn render(
        &mut self,
        atlas: &mut FontAtlas,
        font: &Font,
        canvas: &mut Canvas<Window>,
        rect: Rect,
    );
}

pub trait EventConsumer {
    fn consume_event(&mut self, event: &Event);
}
