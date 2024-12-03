use std::process::Command;

use sdl2::keyboard::Keycode;
use sdl2::render::{BlendMode, TextureCreator};
use sdl2::video::WindowContext;
use sdl2::{event::Event, pixels::Color, rect::Rect, render::Canvas, video::Window};

use crate::animation::{Animation, AnimationType};
use crate::app::App;
use crate::config::Config;
use crate::utils::cache::TextureCache;
use crate::utils::draw::{draw_string_texture, DrawExtensions};
use crate::utils::font::FontConfig;

use super::traits::UIComponent;

pub struct TextInput {
    pub id: String,
    pub text: String,
    pub text_changed: bool,
    pub foreground_color: Color,
    pub cursor_x: i32,
    pub blink: bool,
    pub last_blink: Option<u128>,
    pub input_hint: Option<String>,
    pub cursor_anim: Animation,
    pub cursor_position: usize,
}

impl TextInput {
    pub fn new(id: impl AsRef<str>, config: &Config) -> Self {
        TextInput {
            id: id.as_ref().to_string(),
            text: String::from(""),
            text_changed: false,
            foreground_color: config.prompt_color,
            cursor_x: 0,
            blink: config.cursor_blink,
            last_blink: None,
            input_hint: None,
            cursor_anim: Animation::new(0, 0, AnimationType::EaseOut, 40.0),
            cursor_position: 0,
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
        self.text = text;
        self.text_changed = true;
    }

    pub fn insert_at_cursor(&mut self, text: String) {
        let char_boundary = match self.text.char_indices().nth(self.cursor_position) {
            Some((cb, _)) => cb,
            None => self.text.char_indices().count(),
        };
        let lhs: String = self.text.get(..char_boundary).unwrap().into();
        let rhs: String = self.text.get(char_boundary..).unwrap().into();
        dbg!(char_boundary, &lhs, &rhs);
        self.set_text(lhs + &text + &rhs);
        self.cursor_position += 1;
    }
    pub fn delete_at_cursor(&mut self) {
        if !self.text.is_empty() && self.cursor_position > 0 {
            if let Some((char_boundary, _)) = self.text.char_indices().nth(self.cursor_position - 1)
            {
                let lhs: String = self.text.get(..char_boundary).unwrap().into();
                let rhs: String = self.text.get(char_boundary + 1..).unwrap().into();
                self.set_text(lhs + &rhs);
                self.cursor_left();
            };
        }
    }

    pub fn cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
            self.text_changed = true;
        }
    }
    pub fn cursor_right(&mut self) {
        if self.cursor_position < self.text.char_indices().count() {
            self.cursor_position += 1;
            self.text_changed = true;
        }
    }
}

impl UIComponent for TextInput {
    fn get_state(&self) -> &dyn std::any::Any {
        &self.text
    }

    fn set_state(&mut self, state: Box<dyn std::any::Any>) {
        self.text = state.downcast_ref::<String>().unwrap().to_string();
    }
    fn id(&self) -> String {
        self.id.clone()
    }

    fn render(
        &mut self,
        texture_creator: &TextureCreator<WindowContext>,
        cache: &mut TextureCache,
        app: &App,
        canvas: &mut Canvas<Window>,
        rect: Rect,
        elapsed: u128,
    ) {
        // Draw outline and set transparency
        canvas.set_blend_mode(BlendMode::Blend);
        canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
        canvas.draw_filled_rounded_rect(Rect::new(1, 1, rect.width() - 2, rect.height() - 2), 7);

        cache.fonts.load_font(FontConfig {
            alias: "normal-24".to_string(),
            family: app.config.font_family.clone(),
            point_size: 24,
        });
        let font = cache.fonts.get_font("normal-24");
        let (_fw, fh) = font.size_of(" ").unwrap();

        let draw_cursor = !self.text.is_empty() || self.input_hint.is_none();
        let texture = match (self.text.len(), &self.input_hint) {
            (0, Some(hint)) => Some(draw_string_texture(
                hint.to_string(),
                texture_creator,
                font,
                Color::RGBA(100, 100, 100, 255),
            )),
            (0, None) => None,
            _ => Some(draw_string_texture(
                self.text.clone(),
                texture_creator,
                font,
                self.foreground_color,
            )),
        };

        if self.last_blink.is_none() {
            self.last_blink = Some(elapsed);
        }

        self.cursor_anim.tick(elapsed);
        match texture {
            Some(tex) => {
                let query = tex.query();
                let (w, h) = (query.width as i32, query.height as i32);
                if self.text_changed {
                    //self.cursor_anim.set_target(w as u32, None);
                    self.cursor_anim.set_target(
                        (w as u32 / self.text.len() as u32) * self.cursor_position as u32,
                        None,
                    );
                    self.text_changed = false;
                }

                self.cursor_x = self.cursor_anim.value as i32;

                let text_rect = Rect::new(10, (rect.h - h) / 2, w as u32, h as u32);

                canvas.copy(&tex, None, Some(text_rect)).unwrap();
            }
            None => self.cursor_x = 0,
        };

        if draw_cursor {
            let cursor_rect = Rect::new(self.cursor_x + 10, (rect.h - fh as i32) / 2, 5, fh);
            let alpha = match self.blink {
                true => {
                    ((((elapsed - self.last_blink.unwrap()) as f32 / 100.0).sin() + 1.0) / 2.0)
                        * 255.0
                }
                false => 255.0,
            };

            canvas.set_draw_color(Color::RGBA(0, 0, 255, alpha as u8));
            canvas.fill_rect(cursor_rect).unwrap();
        }
    }
    fn handle_event(&mut self, event: &Event, ctx: &mut App, _: u128) {
        match event {
            sdl2::event::Event::KeyDown {
                keycode: Some(Keycode::Return),
                ..
            } => {
                if self.text.starts_with("!") {
                    let t = self.text.replace("!", "");
                    let args = vec!["-c", &t];
                    let _cmd = Command::new("sh").args(args).spawn();
                    ctx.should_hide = true;
                }
            }
            sdl2::event::Event::TextInput { text, .. } => {
                // Ignore text input with ctrl pressed because of wayland reasons
                // On key press repeat a text input event is emitted
                if ctx.ctrl_pressed {
                    return;
                }
                self.insert_at_cursor(text.to_string());
            }
            sdl2::event::Event::KeyDown {
                keycode: Some(Keycode::Backspace),
                ..
            } => {
                self.delete_at_cursor();
            }
            sdl2::event::Event::KeyDown {
                keycode: Some(Keycode::Left),
                ..
            } => {
                self.cursor_left();
            }
            sdl2::event::Event::KeyDown {
                keycode: Some(Keycode::Right),
                ..
            } => {
                self.cursor_right();
            }
            _ => (),
        };
    }
    fn update(&mut self, app: &mut App, elapsed: u128) {}
}
