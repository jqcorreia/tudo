use std::cell::RefCell;
use std::rc::Rc;

use sdl2::keyboard::Keycode;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use sdl2::{event::Event, pixels::Color, rect::Rect, render::Canvas, ttf::Font, video::Window};

use crate::app::App;
use crate::components::traits::{EventConsumer, Render};
use crate::utils::cache::TextureCache;

pub struct Prompt {
    pub text: String,
    pub foreground_color: Color,
    pub cursor_x: i32,
    pub last_cursor_move: u128,
}

impl Prompt {
    pub fn new() -> Self {
        Prompt {
            text: String::from(""),
            foreground_color: Color::RGBA(255, 255, 255, 255),
            cursor_x: 0,
            last_cursor_move: 0,
        }
    }
    pub fn with_text(mut self, text: String) -> Self {
        self.text = text;
        self
    }
    pub fn with_foreground_color(mut self, color: Color) -> Self {
        self.foreground_color = color;
        self
    }
    pub fn set_text(&mut self, text: String) {
        self.text = text
    }
}

impl Render for Prompt {
    fn id(&self) -> String {
        String::from("prompt")
    }

    fn render(
        &mut self,
        _texture_creator: &TextureCreator<WindowContext>,
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

        self.last_cursor_move = elapsed;
        self.cursor_x = w;

        let text_rect = Rect::new(10, (rect.h - h) / 2, w as u32, h as u32);

        if draw_cursor {
            let cursor_rect = Rect::new(self.cursor_x + 10, (rect.h - h) / 2, 5, h as u32);
            let alpha = ((((elapsed as f32 / 100.0) as f32).sin() + 1.0) / 2.0) * 255.0;

            canvas.set_blend_mode(sdl2::render::BlendMode::Blend);

            canvas.set_draw_color(Color::RGBA(0, 0, 255, alpha as u8));
            if draw_cursor {
                canvas.fill_rect(cursor_rect).unwrap();
            }
            canvas.set_blend_mode(sdl2::render::BlendMode::None);
        }

        canvas.set_draw_color(Color::RGBA(0, 0, 255, 0));
        canvas
            .draw_rect(Rect::new(1, 1, rect.width() - 2, rect.height() - 2))
            .unwrap();
        canvas.copy(&texture, None, Some(text_rect)).unwrap();
    }
}

impl EventConsumer for Prompt {
    fn consume_event(&mut self, event: &Event, app: &mut App) {
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
