use std::process::Command;

use crate::components::list::SelectList;

use crate::components::traits::{EventConsumer, Render};

use crate::sources::SourceItem;
use crate::utils::atlas::FontAtlas;
use crate::utils::fuzzy::basic;
use sdl2::keyboard::Keycode;
use sdl2::{event::Event, rect::Rect, render::Canvas, ttf::Font, video::Window};

pub struct SourceItemsList {
    pub list: SelectList,
    pub items: Vec<SourceItem>,
}

impl SourceItemsList {
    pub fn new() -> SourceItemsList {
        SourceItemsList {
            list: SelectList::new(),
            items: Vec::new(),
        }
    }

    pub fn set_list_and_prompt(&mut self, new_list: Vec<SourceItem>, prompt: String) {
        if new_list.len() == 0 {
            return;
        }
        let titles_list = new_list
            .iter()
            .map(|i| i.title.clone())
            .collect::<Vec<String>>();

        // If nothing is written just clear the select list items
        if prompt.len() == 0 {
            self.list.set_list(Some(titles_list));
        } else {
            self.list
                .set_list(match basic(prompt.to_string(), &titles_list) {
                    Some(v) => Some(v.iter().map(|x| x.value.clone()).collect()),
                    None => None,
                });
        }
        self.items = new_list;
    }

    fn exec(&mut self) {
        let selected_title = self.list.get_selected_item().unwrap();
        for i in self.items.iter() {
            if i.title == selected_title {
                Command::new(i.action.clone()).spawn();
                dbg!(i);
            }
        }
    }
}

impl Render for SourceItemsList {
    fn id(&self) -> String {
        String::from("items select")
    }

    fn render(
        &mut self,
        atlas: &mut FontAtlas,
        font: &Font,
        canvas: &mut Canvas<Window>,
        rect: Rect,
    ) {
        self.list.render(atlas, font, canvas, rect)
    }
}

impl EventConsumer for SourceItemsList {
    fn consume_event(&mut self, event: &Event) {
        match event {
            sdl2::event::Event::KeyDown {
                keycode: Some(Keycode::Return),
                ..
            } => self.exec(),
            _ => (),
        };
        self.list.consume_event(event)
    }
}
