use sdl2::{pixels::Color, rect::Rect};

use crate::{
    app::App,
    utils::draw::{draw_rounded_rect, draw_string_texture, draw_string_texture_canvas},
};

use super::{text, traits::UIComponent};

pub struct Button {
    id: String,
    pressed: bool,
    text: String,
    pub on_click: fn(&Button, &mut App),
}

impl Button {
    pub fn new(id: String, text: String) -> Button {
        Button {
            id,
            text,
            pressed: false,
            on_click: |_, _| (),
        }
    }
    pub fn with_on_click(mut self, func: fn(&Button, &mut App)) -> Self {
        self.on_click = func;
        self
    }
}

impl UIComponent for Button {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn render(
        &mut self,
        tc: &sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        cache: &mut crate::utils::cache::TextureCache,
        _app: &crate::app::App,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        rect: sdl2::rect::Rect,
        _elapsed: u128,
    ) {
        let r = Rect::new(0, 0, rect.width() - 1, rect.height() - 1);
        let font = cache.fonts.get_font("normal-20".to_string());
        let color = if self.pressed {
            Color::BLUE
        } else {
            Color::GRAY
        };
        let tex = draw_string_texture(self.text.clone(), tc, font, color);
        let (tw, th) = (tex.query().width, tex.query().height);
        let text_x = (rect.w - tw as i32) / 2;
        // let text_y = (rect.h - th as i32) / 2;
        let text_y = -3;

        draw_rounded_rect(canvas, r, 3, Color::RGBA(0x30, 0x30, 0x50, 255));
        canvas
            .copy(&tex, None, Rect::new(text_x, text_y, tw, th))
            .unwrap();
        // draw_string_texture_canvas(canvas, 0, 0, self.text.clone(), font, color);
    }

    fn update(&mut self, event: &sdl2::event::Event, app: &mut App, _elapsed: u128) {
        // dbg!(event);
        match event {
            sdl2::event::Event::MouseButtonDown { .. } => {
                self.pressed = true;
                (self.on_click)(self, app);
            }
            sdl2::event::Event::MouseButtonUp { .. } => self.pressed = false,
            _ => (),
        }
    }

    fn get_state(&self) -> &dyn std::any::Any {
        todo!()
    }

    fn set_state(&mut self, _state: Box<dyn std::any::Any>) {
        todo!()
    }
}
