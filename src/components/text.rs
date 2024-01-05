use sdl2::keyboard::Keycode;
use sdl2::{event::Event, pixels::Color, rect::Rect, render::Canvas, ttf::Font, video::Window};

use crate::components::traits::{EventConsumer, Render};
use crate::utils::cache::TextureCache;

use super::traits::Component;

pub struct Prompt {
    pub text: String,
    pub foreground_color: Color,
}
impl Component for Prompt {}

impl Render for Prompt {
    fn id(&self) -> String {
        String::from("prompt")
    }

    fn render(
        &mut self,
        cache: &mut TextureCache,
        font: &Font,
        canvas: &mut Canvas<Window>,
        rect: Rect,
    ) {
        if self.text.len() == 0 {}
        let texture = match self.text.len() {
            0 => cache.font.draw_string(
                "Write something".to_string(),
                canvas,
                font,
                Color::RGBA(100, 100, 100, 255),
            ),
            _ => cache
                .font
                .draw_string(self.text.clone(), canvas, font, self.foreground_color),
        };
        let query = texture.query();
        let (w, h) = (query.width, query.height);
        canvas.set_draw_color(Color::RGBA(0, 0, 255, 0));
        canvas
            .draw_rect(Rect::new(1, 1, rect.width() - 2, rect.height() - 2))
            .unwrap();
        canvas
            .copy(&texture, None, Some(Rect::new(10, 10, w, h)))
            .unwrap();
    }
}

impl EventConsumer for Prompt {
    fn consume_event(&mut self, event: &Event) {
        match event {
            sdl2::event::Event::TextInput { text, .. } => {
                self.text += &text;
            }
            sdl2::event::Event::KeyDown {
                keycode: Some(Keycode::Backspace),
                ..
            } => {
                match self.text.char_indices().nth_back(0) {
                    Some((char_boundary, _)) => {
                        self.text = self.text.get(..char_boundary).unwrap().into()
                    }
                    None => (),
                };
            }
            _ => (),
        };
    }
}
