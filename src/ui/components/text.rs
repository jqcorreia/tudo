use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{BlendMode, TextureCreator};
use sdl2::video::WindowContext;
use sdl2::{event::Event, pixels::Color, rect::Rect, render::Canvas, video::Window};

use crate::app::App;
use crate::config::Config;
use crate::ui::components::traits::{EventConsumer, Render};
use crate::utils::cache::TextureCache;
use crate::utils::draw::{draw_string_texture, DrawExtensions};

use super::traits::UIComponent;

#[derive(Debug)]
pub struct Prompt {
    pub id: String,
    pub text: String,
    pub foreground_color: Color,
    pub cursor_x: i32,
    pub last_cursor_move: u128,
    pub blink: bool,
    pub last_blink: Option<u128>,
    pub input_hint: Option<String>,
}

impl Prompt {
    pub fn new(id: impl AsRef<str>, config: &Config) -> Self {
        Prompt {
            id: id.as_ref().to_string(),
            text: String::from(""),
            foreground_color: config.prompt_color,
            cursor_x: 0,
            last_cursor_move: 0,
            blink: config.cursor_blink,
            last_blink: None,
            input_hint: None,
        }
    }
    pub fn with_input_hint(mut self, input_hint: String) -> Self {
        self.input_hint = Some(input_hint);
        self
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

impl UIComponent for Prompt {
    fn get_state(&self) -> &dyn std::any::Any {
        &self.text
    }

    fn set_state(&mut self, state: Box<dyn std::any::Any>) {
        self.text = state.downcast_ref::<String>().unwrap().to_string();
    }
}

impl Render for Prompt {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn render(
        &mut self,
        texture_creator: &TextureCreator<WindowContext>,
        cache: &mut TextureCache,
        _app: &App,
        canvas: &mut Canvas<Window>,
        rect: Rect,
        elapsed: u128,
    ) {
        let font = cache.fonts.get_font("normal-20");

        let draw_cursor = self.text.len() > 0 || self.input_hint.is_none();

        let texture = match (self.text.len(), &self.input_hint) {
            (0, Some(hint)) => draw_string_texture(
                hint.to_string(),
                &texture_creator,
                font,
                Color::RGBA(100, 100, 100, 255),
            ),
            (0, None) => texture_creator
                .create_texture_target(PixelFormatEnum::RGBA8888, 1, 20) //FIXME(quadrado) just add a minimal size texture
                .unwrap(),
            _ => draw_string_texture(
                self.text.clone(),
                &texture_creator,
                font,
                self.foreground_color,
            ),
        };

        if self.last_blink.is_none() {
            self.last_blink = Some(elapsed);
        }

        let query = texture.query();
        let (w, h) = (query.width as i32, query.height as i32);

        self.last_cursor_move = elapsed;
        self.cursor_x = w;

        let text_rect = Rect::new(10, (rect.h - h) / 2, w as u32, h as u32);

        canvas.set_blend_mode(BlendMode::Blend);
        canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
        canvas.draw_filled_rounded_rect(Rect::new(1, 1, rect.width() - 2, rect.height() - 2), 7);
        canvas.copy(&texture, None, Some(text_rect)).unwrap();

        if draw_cursor {
            let cursor_rect = Rect::new(self.cursor_x + 10, (rect.h - h) / 2, 5, h as u32);
            let alpha = match self.blink {
                true => {
                    (((((elapsed - self.last_blink.unwrap()) as f32 / 100.0) as f32).sin() + 1.0)
                        / 2.0)
                        * 255.0
                }
                false => 255.0,
            };

            canvas.set_draw_color(Color::RGBA(0, 0, 255, alpha as u8));
            canvas.fill_rect(cursor_rect).unwrap();
        }
    }
}

impl EventConsumer for Prompt {
    fn consume_event(&mut self, event: &Event, _: &mut App) {
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
