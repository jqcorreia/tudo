use chrono::prelude::*;
use sdl2::rect::Rect;

use super::{
    label::Label,
    traits::{EventConsumer, Render, UIComponent},
};

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

impl Render for Clock {
    fn id(&self) -> String {
        self.id.clone()
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
}

impl EventConsumer for Clock {
    fn consume_event(&mut self, _event: &sdl2::event::Event, _app: &mut crate::app::App) {}
}

impl UIComponent for Clock {
    fn get_state(&self) -> &dyn std::any::Any {
        return self.label.get_state();
    }

    fn set_state(&mut self, state: Box<dyn std::any::Any>) {
        self.label.set_state(state);
    }
}
