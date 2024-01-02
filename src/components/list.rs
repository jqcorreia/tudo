use sdl2::keyboard::Keycode;
use sdl2::{event::Event, pixels::Color, rect::Rect, render::Canvas, ttf::Font, video::Window};

use crate::components::traits::{EventConsumer, Render};
use crate::utils::atlas::FontAtlas;

pub struct SelectList {
    pub items: Option<Vec<String>>,
    pub foreground_color: Color,
    pub selected_index: usize,
}

impl Render for SelectList {
    fn id(&self) -> String {
        String::from("select")
    }

    fn render(
        &mut self,
        atlas: &mut FontAtlas,
        font: &Font,
        canvas: &mut Canvas<Window>,
        rect: Rect,
    ) {
        let mut y: u32 = 0;

        canvas.set_draw_color(Color::RGBA(50, 48, 47, 255));
        canvas.clear();

        //FIXME(quadrado): drawing routines should be abstracted
        match &self.items {
            Some(items) => {
                for (idx, item) in items.as_slice().iter().enumerate() {
                    let texture =
                        atlas.draw_string(item.clone(), canvas, font, self.foreground_color);

                    let query = texture.query();
                    let (w, h) = (query.width, query.height);
                    if idx == self.selected_index {
                        canvas.set_draw_color(Color::RGBA(0, 0, 255, 0));
                        canvas
                            .draw_rect(Rect::new(0, y as i32, rect.width(), h))
                            .unwrap();
                    }

                    canvas
                        .copy(&texture, None, Some(Rect::new(10, y as i32, w, h)))
                        .unwrap();
                    y += h + 1;
                }
            }
            None => {
                let texture = atlas.draw_string(
                    "No items found".to_string(),
                    canvas,
                    font,
                    self.foreground_color,
                );

                let query = texture.query();
                let (w, h) = (query.width, query.height);
                canvas
                    .copy(&texture, None, Some(Rect::new(20, y as i32, w, h)))
                    .unwrap();
            }
        }
    }
}

impl EventConsumer for SelectList {
    fn consume_event(&mut self, event: &Event) {
        match event {
            sdl2::event::Event::KeyDown {
                keycode: Some(Keycode::Return),
                ..
            } => (),
            sdl2::event::Event::KeyDown {
                keycode: Some(Keycode::P),
                keymod: sdl2::keyboard::Mod::LCTRLMOD,
                ..
            } => self.select_up(),
            sdl2::event::Event::KeyDown {
                keycode: Some(Keycode::N),
                keymod: sdl2::keyboard::Mod::LCTRLMOD,
                ..
            } => self.select_down(),
            _ => (),
        }
    }
}

impl SelectList {
    pub fn new() -> SelectList {
        SelectList {
            items: None,
            selected_index: 0,
            foreground_color: Color::RGBA(255, 255, 255, 255),
        }
    }
    pub fn select_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }
    pub fn select_down(&mut self) {
        if self.items.is_some() && self.selected_index < self.items.as_ref().unwrap().len() - 1 {
            self.selected_index += 1;
        }
    }

    pub fn set_selected_index(&mut self, idx: usize) {
        self.selected_index = idx;
    }

    pub fn set_list(&mut self, new_list: Option<Vec<String>>) {
        if new_list == self.items {
            return;
        }
        self.items = new_list;
        self.set_selected_index(0);
    }

    pub fn get_selected_item(&mut self) -> Option<String> {
        match &self.items {
            None => None,
            Some(items) => Some(items.get(self.selected_index).unwrap().to_string()),
        }
    }
}
