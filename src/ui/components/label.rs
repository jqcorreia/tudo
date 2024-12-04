use std::any::Any;

use sdl2::{pixels::Color, rect::Rect};

use crate::{app::App, utils::draw::draw_string_texture};

use super::traits::UIComponent;

pub struct Label {
    pub id: String,
    pub text: String,
    pub font_name: Option<String>,
}

impl Label {
    pub fn new(id: String, text: String) -> Label {
        Label {
            id,
            text,
            font_name: None,
        }
    }
}

impl UIComponent for Label {
    fn id(&self) -> String {
        self.id.clone()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn render(
        &mut self,
        texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        cache: &mut crate::utils::cache::TextureCache,
        _app: &crate::app::App,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        _rect: Rect,
        _elapsed: u128,
    ) {
        let font = cache
            .fonts
            .get_font(self.font_name.clone().unwrap_or("normal-24".to_string()));
        let color = Color::RGBA(100, 100, 100, 255);
        let texture = draw_string_texture(self.text.clone(), texture_creator, font, color);
        let (w, h) = (texture.query().width, texture.query().height);

        canvas.copy(&texture, None, Rect::new(0, 0, w, h)).unwrap();
    }
    fn handle_event(&mut self, _event: &sdl2::event::Event, _app: &mut App, _elapsed: u128) {}
    fn get_state(&self) -> &dyn std::any::Any {
        &self.text
    }

    fn set_state(&mut self, state: Box<dyn std::any::Any>) {
        self.text = state.downcast_ref::<String>().unwrap().to_string();
    }
    fn update(&mut self, _: &mut App, _: u128) {}
}
