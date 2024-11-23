use sdl2::{pixels::Color, rect::Rect};

use crate::{
    app::App,
    utils::draw::{draw_rounded_rect, draw_string_texture},
};

use super::traits::UIComponent;

pub struct Button {
    id: String,
    pressed: bool,
    text: String,
    pub on_click: fn(&Button, &mut App),
    focus: bool,
}

impl Button {
    pub fn new(id: String, text: String) -> Button {
        Button {
            id,
            text,
            pressed: false,
            on_click: |_, _| (),
            focus: false,
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
        let color = match (self.pressed, self.get_focus()) {
            (true, _) => Color::RED,
            (false, true) => Color::BLUE,
            _ => Color::GRAY,
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
        match event {
            sdl2::event::Event::MouseButtonDown { .. } => {
                self.pressed = true;
            }
            sdl2::event::Event::MouseButtonUp { .. } => {
                self.pressed = false;
                (self.on_click)(self, app);
            }
            _ => (),
        }
    }

    fn get_state(&self) -> &dyn std::any::Any {
        todo!()
    }

    fn set_state(&mut self, _state: Box<dyn std::any::Any>) {
        todo!()
    }
    fn set_focus(&mut self, focus: bool) {
        self.focus = focus;
    }
    fn get_focus(&self) -> bool {
        self.focus
    }
}
