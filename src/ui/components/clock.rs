use std::any::Any;

use chrono::prelude::*;
use sdl2::{event::Event, rect::Rect};

use crate::app::App;

use super::{label::Label, traits::UIComponent};

pub struct Clock {
    pub id: String,
    pub label: Label,
}

impl Clock {
    pub fn new(id: String) -> Clock {
        let mut label = Label::new(String::from("clock-label"), String::from(""));
        label.font_name = Some(String::from("normal-20"));

        Clock { id, label }
    }
}

impl UIComponent for Clock {
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
        rect: Rect,
        elapsed: u128,
    ) {
        let date_as_string = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        self.label.text = date_as_string;
        self.label
            .render(texture_creator, cache, _app, canvas, rect, elapsed);
    }
    fn get_state(&self) -> &dyn std::any::Any {
        return self.label.get_state();
    }

    fn set_state(&mut self, state: Box<dyn std::any::Any>) {
        self.label.set_state(state);
    }
    fn handle_event(&mut self, _event: &Event, _app: &mut App, _elapsed: u128) {}
    fn update(&mut self, _: &mut App, _: u128) {}
}
