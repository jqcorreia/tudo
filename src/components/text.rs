use sdl2::keyboard::Keycode;
use sdl2::{event::Event, pixels::Color, rect::Rect, render::Canvas, ttf::Font, video::Window};

use crate::components::traits::{EventConsumer, Render};
use crate::utils::cache::TextureCache;

pub struct Prompt {
    pub text: String,
    pub foreground_color: Color,
}

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
        elapsed: u128,
    ) {
        let draw_cursor = self.text.len() > 0;

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
        let (w, h) = (query.width as i32, query.height as i32);

        // let alpha = (((elapsed as f32).sin() + 1.0) / 2.0) * 255.0;
        // canvas.set_blend_mode(sdl2::render::BlendMode::Blend);

        // canvas.set_draw_color(Color::RGBA(0, 0, 255, alpha as u8));
        if draw_cursor {
            canvas.fill_rect(Rect::new(w + 10, 0, 5, rect.h as u32));
        }
        canvas.set_draw_color(Color::RGBA(0, 0, 255, 0));
        canvas
            .draw_rect(Rect::new(1, 1, rect.width() - 2, rect.height() - 2))
            .unwrap();
        canvas
            .copy(&texture, None, Some(Rect::new(10, 10, w as u32, h as u32)))
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
