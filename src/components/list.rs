use std::rc::Rc;
use std::usize;

use sdl2::keyboard::Keycode;
use sdl2::Sdl;
use sdl2::{event::Event, pixels::Color, rect::Rect, render::Canvas, ttf::Font, video::Window};

use crate::components::traits::{EventConsumer, Render};
use crate::sources::SourceItem;
use crate::utils::cache::TextureCache;
use crate::utils::fuzzy::basic_contains;
use crate::AppContext;

pub struct Viewport(pub usize, pub usize);
impl Viewport {
    pub fn down(&mut self, amount: usize) {
        self.0 += amount;
        self.1 += amount;
    }
    pub fn up(&mut self, amount: usize) {
        self.0 -= amount;
        self.1 -= amount;
    }
}

pub struct SelectList<T> {
    pub items: Vec<T>,
    pub foreground_color: Color,
    pub selected_index: usize,
    pub viewport: Viewport,
    pub on_select: fn(&T, Sdl),
    pub ctx: Rc<AppContext>,
}

impl<T: PartialEq> SelectList<T> {
    pub fn new(ctx: Rc<AppContext>) -> SelectList<T> {
        SelectList {
            items: Vec::<T>::new(),
            selected_index: 0,
            foreground_color: Color::RGBA(255, 255, 255, 255),
            viewport: Viewport(0, 10),
            on_select: |_, _| (),
            ctx,
        }
    }
    pub fn select_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
            if self.selected_index < self.viewport.0 {
                self.viewport.up(1);
            }
        }
    }
    pub fn select_down(&mut self) {
        if self.items.len() > 0 && self.selected_index < self.items.len() - 1 {
            self.selected_index += 1;
            if self.selected_index > self.viewport.1 {
                self.viewport.down(1);
            }
        }
    }

    pub fn set_selected_index(&mut self, idx: usize) {
        self.selected_index = idx;
    }

    pub fn set_list(&mut self, new_list: Vec<T>) {
        if new_list == self.items {
            return;
        }
        self.items = new_list;
        self.set_selected_index(0);
        self.viewport = Viewport(0, 10);
    }

    pub fn get_selected_item(&self) -> Option<&T> {
        match self.items.get(self.selected_index) {
            None => None,
            Some(item) => Some(item),
        }
    }
}

impl SelectList<SourceItem> {
    pub fn set_list_and_prompt(&mut self, new_list: Vec<SourceItem>, prompt: String) {
        if prompt.len() == 0 {
            self.set_list(new_list);
        } else {
            let haystack = new_list
                .iter()
                .map(|i| i.title.clone())
                .collect::<Vec<String>>();
            let matches = basic_contains(prompt.to_string(), &haystack).unwrap_or(Vec::new());
            let mut final_list = Vec::new();

            for m in matches {
                final_list.push(new_list.get(m.original_idx).unwrap().clone());
            }
            self.set_list(final_list);
        }
    }
}

impl Render for SelectList<SourceItem> {
    fn id(&self) -> String {
        String::from("select")
    }

    fn render(
        &mut self,
        cache: &mut TextureCache,
        font: &Font,
        canvas: &mut Canvas<Window>,
        rect: Rect,
    ) {
        let mut y: u32 = 0;

        canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
        canvas.clear();

        //FIXME(quadrado): drawing routines should be abstracted
        if self.items.len() == 0 {
            let texture = cache.font.draw_string(
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
        } else {
            for (idx, item) in self.items.as_slice().iter().enumerate() {
                // Assess if current idx is inside the viewport
                // FIXME(jqcorreia): This could be abstracted
                if idx < self.viewport.0 || idx > self.viewport.1 {
                    continue;
                }
                if y as i32 > rect.h {
                    continue;
                }

                // Draw icon
                let icon_height: u32 = 32;
                if item.icon.is_some() {
                    let icon_texture = cache
                        .images
                        .get_image(item.icon.as_ref().unwrap().to_string());
                    canvas
                        .copy(&icon_texture, None, Rect::new(0, y as i32, 32, 32))
                        .unwrap();
                }

                // Draw text
                let text_texture =
                    cache
                        .font
                        .draw_string(item.title.clone(), canvas, font, self.foreground_color);
                let query = text_texture.query();
                let (w, h) = (query.width, query.height);
                let row_height = std::cmp::max(h, icon_height);

                if idx == self.selected_index {
                    canvas.set_draw_color(Color::RGBA(0, 0, 255, 0));
                    canvas
                        .draw_rect(Rect::new(0, y as i32, rect.width(), row_height))
                        .unwrap();
                }

                canvas
                    .copy(&text_texture, None, Some(Rect::new(34, y as i32, w, h)))
                    .unwrap();
                y += row_height + 1;
            }
        }
    }
}

impl Render for SelectList<String> {
    fn id(&self) -> String {
        String::from("select")
    }

    fn render(
        &mut self,
        cache: &mut TextureCache,
        font: &Font,
        canvas: &mut Canvas<Window>,
        rect: Rect,
    ) {
        let mut y: u32 = 0;

        canvas.set_draw_color(Color::RGBA(50, 48, 47, 255));
        canvas.clear();

        //FIXME(quadrado): drawing routines should be abstracted
        if self.items.len() == 0 {
            let texture = cache.font.draw_string(
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
        } else {
            for (idx, item) in self.items.as_slice().iter().enumerate() {
                let texture =
                    cache
                        .font
                        .draw_string(item.clone(), canvas, font, self.foreground_color);

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
    }
}

impl<T: PartialEq> EventConsumer for SelectList<T> {
    fn consume_event(&mut self, event: &Event) {
        match event {
            sdl2::event::Event::KeyDown {
                keycode: Some(Keycode::Return),
                ..
            } => (self.on_select)(
                self.get_selected_item().as_ref().unwrap(),
                self.ctx.sdl.clone(),
            ),
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
