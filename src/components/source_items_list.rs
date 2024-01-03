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

        if prompt.len() == 0 {
            self.list.set_list(titles_list);
        } else {
            let matches = basic(prompt.to_string(), &titles_list).unwrap_or(Vec::new());
            let mut final_list = Vec::new();

            for m in matches {
                final_list.push(new_list.get(m.original_idx).unwrap());
            }
            self.list
                .set_list(final_list.iter().map(|i| i.title.clone()).collect());
        }
        self.items = new_list;
    }

    fn exec(&mut self) {
        let selected_title = self.list.get_selected_item().unwrap();
        for i in self.items.iter() {
            if i.title == selected_title {
                let mut args = vec!["-c"];

                for token in i.action.split(" ") {
                    args.push(token);
                }
                let _cmd = Command::new("sh").args(args).spawn();
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
