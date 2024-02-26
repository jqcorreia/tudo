use std::usize;

use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use sdl2::{event::Event, pixels::Color, rect::Rect, render::Canvas, ttf::Font, video::Window};

use crate::sources::SourceItem;
use crate::ui::components::traits::{EventConsumer, Render};
use crate::utils::cache::TextureCache;
use crate::utils::draw::{draw_string, draw_string_texture, DrawExtensions};
use crate::utils::fuzzy::basic_contains;
use crate::App;

use super::traits::UIComponent;

trait RenderItem<T> {
    fn render_row<'a>(
        &'a self,
        item: &'a T,
        texture_creator: &'a TextureCreator<WindowContext>,
        tex_cache: &TextureCache,
        font: &Font,
        canvas: &mut Canvas<Window>,
        rect: Rect,
        elapsed: u128,
    ) -> Texture;
}

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
#[derive(Clone)]
pub struct SelectListState<T> {
    pub items: Vec<T>,
    pub prompt: String,
}

pub struct SelectList<T> {
    pub id: String,
    pub items: Vec<T>,
    pub foreground_color: Color,
    pub selected_index: usize,
    pub viewport: Viewport,
    pub on_select: fn(&T, &mut App),
}

impl UIComponent for SelectList<SourceItem> {
    fn get_state(&self) -> &dyn std::any::Any {
        &self.items
    }

    fn set_state(&mut self, state: Box<dyn std::any::Any>) {
        let new_state = state.downcast_ref::<SelectListState<SourceItem>>().unwrap();
        let prompt = &new_state.prompt;
        let new_list = &new_state.items;

        //FIXME(quadrado) this whole function is whack
        // Refactor this ASAP
        // Rewriting the fuzzy search in a generic way that returns clones is possibility
        // in order to not have to manage original list indices
        let mut final_list = Vec::new();

        if prompt.len() == 0 {
            final_list = new_list.clone();
        } else {
            let haystack: Vec<String>;
            let mut matches;

            // Tag and value search
            if prompt.starts_with(":") {
                let clean_prompt = prompt.replace(":", "");
                let (tag, filter) = clean_prompt.split_once(" ").unwrap_or((&clean_prompt, ""));

                // action type searching
                haystack = new_list
                    .iter()
                    .map(|i| i.action.tags().get(0).unwrap().clone())
                    .collect::<Vec<String>>();
                matches = basic_contains(tag.to_string(), &haystack).unwrap_or(Vec::new());

                // if filter present, further filter the list with another haystack
                if filter != "".to_string() {
                    let mut list2 = Vec::new();
                    for m in matches.iter() {
                        list2.push(new_list.get(m.original_idx).unwrap().clone());
                    }

                    let haystack2 = list2
                        .iter()
                        .map(|i| i.title.clone())
                        .collect::<Vec<String>>();
                    matches = basic_contains(filter.to_string(), &haystack2).unwrap_or(Vec::new());

                    // We cant to the final list computation on the outside since we are getting values
                    // from different lists based on
                    for m in matches.iter() {
                        final_list.push(list2.get(m.original_idx).unwrap().clone());
                    }
                } else {
                    for m in matches.iter() {
                        final_list.push(new_list.get(m.original_idx).unwrap().clone());
                    }
                }
            } else {
                // Simple title search
                haystack = new_list
                    .iter()
                    .map(|i| i.title.clone())
                    .collect::<Vec<String>>();
                matches = basic_contains(prompt.to_string(), &haystack).unwrap_or(Vec::new());

                for m in matches.iter() {
                    final_list.push(new_list.get(m.original_idx).unwrap().clone());
                }
            }
        }

        // Sort by title
        final_list.sort_by(|this, other| this.title.cmp(&other.title));
        self.set_list(final_list);
    }
}

impl UIComponent for SelectList<String> {
    fn get_state(&self) -> &dyn std::any::Any {
        &self.items
    }

    fn set_state(&mut self, state: Box<dyn std::any::Any>) {
        self.items = state.downcast_ref::<Vec<String>>().unwrap().to_vec();
    }
}

impl<T: PartialEq> SelectList<T> {
    pub fn new(id: impl AsRef<str>) -> SelectList<T> {
        SelectList {
            id: id.as_ref().to_string(),
            items: Vec::<T>::new(),
            selected_index: 0,
            foreground_color: Color::RGBA(255, 255, 255, 255),
            viewport: Viewport(0, 10),
            on_select: |_, _| (),
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
        self.viewport = Viewport(0, 10); // The bottom setting dynamic, this 10 is irrelevant
    }

    pub fn get_selected_item(&self) -> Option<&T> {
        match self.items.get(self.selected_index) {
            None => None,
            Some(item) => Some(item),
        }
    }

    pub fn move_viewport_up(&mut self) {
        if self.viewport.0 > 0 {
            self.viewport.0 -= 1;
            // Do this to simulate an adjustment to viewport bottom without
            // the need for row height
            self.viewport.1 -= 1;
            if self.viewport.1 < self.selected_index {
                self.selected_index = self.viewport.1
            }
        }
    }

    pub fn move_viewport_down(&mut self) {
        if self.viewport.1 < self.items.len() {
            self.viewport.0 += 1;
            if self.viewport.0 > self.selected_index {
                self.selected_index = self.viewport.0
            }
        }
    }
}

impl Render for SelectList<SourceItem> {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn render(
        &mut self,
        texture_creator: &TextureCreator<WindowContext>,
        cache: &mut TextureCache,
        _app: &App,
        canvas: &mut Canvas<Window>,
        rect: Rect,
        elapsed: u128,
    ) {
        let mut y: u32 = 0;
        let font = cache.fonts.get_font("normal-20");
        let font2 = cache.fonts.get_font("normal-16");

        if self.items.len() == 0 {
            draw_string(
                "No items found".to_string(),
                canvas,
                font,
                self.foreground_color,
                20,
                y as i32,
            );
        } else {
            let row_height: u32 = 34;

            // Setting the bottom of viewport is either janky or we don't need viewport bottom at
            // all. Veredict is still ongoing
            self.viewport.1 = self.viewport.0 + (rect.h / row_height as i32 - 2) as usize;
            for (idx, item) in self.items.as_slice().iter().enumerate() {
                if idx < self.viewport.0 || idx > self.viewport.1 {
                    continue;
                }
                if y as i32 > rect.h {
                    continue;
                }

                let row_texture = self.render_row(
                    item,
                    texture_creator,
                    cache,
                    font2,
                    canvas,
                    Rect::new(0, 0, rect.w as u32, row_height as u32),
                    elapsed,
                );
                canvas
                    .copy(
                        &row_texture,
                        None,
                        Some(Rect::new(0, y as i32, rect.w as u32, row_height)),
                    )
                    .unwrap();
                if idx == self.selected_index {
                    canvas.set_draw_color(Color::RGBA(0, 0, 255, 255));
                    canvas.draw_rounded_rect(Rect::new(0, y as i32, rect.width(), row_height), 5)
                }

                y += row_height;
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
        texture_creator: &TextureCreator<WindowContext>,
        cache: &mut TextureCache,
        _app: &App,
        canvas: &mut Canvas<Window>,
        rect: Rect,
        elapsed: u128,
    ) {
        let mut y: u32 = 0;
        let font = &cache.fonts.get_font("normal-20");

        //FIXME(quadrado): drawing routines should be abstracted
        if self.items.len() == 0 {
            draw_string(
                "No items found".to_string(),
                canvas,
                font,
                self.foreground_color,
                20,
                y as i32,
            );
        } else {
            for (idx, item) in self.items.as_slice().iter().enumerate() {
                // FIXME(jqcorreia): This could be abstracted
                if idx < self.viewport.0 || idx > self.viewport.1 {
                    continue;
                }
                if y as i32 > rect.h {
                    continue;
                }

                let row_height: u32 = 34;
                let row_texture = self.render_row(
                    item,
                    texture_creator,
                    cache,
                    font,
                    canvas,
                    Rect::new(0, 0, rect.w as u32, row_height as u32),
                    elapsed,
                );
                canvas
                    .copy(
                        &row_texture,
                        None,
                        Some(Rect::new(0, y as i32, rect.w as u32, row_height)),
                    )
                    .unwrap();
                if idx == self.selected_index {
                    canvas.set_draw_color(Color::RGBA(0, 0, 255, 0));
                    canvas
                        .draw_rect(Rect::new(0, y as i32, rect.width(), row_height))
                        .unwrap();
                }

                y += row_height + 1;
            }
        }
    }
}

impl RenderItem<String> for SelectList<String> {
    fn render_row<'a>(
        &'a self,
        item: &String,
        texture_creator: &'a TextureCreator<WindowContext>,
        _cache: &TextureCache,
        font: &Font,
        canvas: &mut Canvas<Window>,
        rect: Rect,
        _elapsed: u128,
    ) -> Texture {
        let mut tex = texture_creator
            .create_texture_target(PixelFormatEnum::RGBA8888, rect.w as u32, rect.h as u32)
            .unwrap();

        canvas
            .with_texture_canvas(&mut tex, |canvas| {
                draw_string(item.clone(), canvas, font, self.foreground_color, 0, 0);
            })
            .unwrap();
        tex
    }
}

impl RenderItem<SourceItem> for SelectList<SourceItem> {
    fn render_row<'a>(
        &'a self,
        item: &SourceItem,
        texture_creator: &'a TextureCreator<WindowContext>,
        cache: &TextureCache,
        font: &Font,
        canvas: &mut Canvas<Window>,
        rect: Rect,
        _elapsed: u128,
    ) -> Texture {
        let mut tex = texture_creator
            .create_texture_target(PixelFormatEnum::RGBA8888, rect.w as u32, rect.h as u32)
            .unwrap();

        canvas
            .with_texture_canvas(&mut tex, |canvas| {
                // Assess if current idx is inside the viewport
                // Draw icon
                // canvas.set_draw_color(Color::RGBA(30, 44, 66, 255));
                canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
                canvas.clear();
                let icon_height: u32 = 32;
                if item.icon.is_some() {
                    let icon_texture = cache
                        .images
                        .get_image(item.icon.as_ref().unwrap().to_string());
                    canvas
                        .copy(
                            &icon_texture,
                            None,
                            Rect::new(0, 0, icon_height, icon_height),
                        )
                        .unwrap();
                }

                {
                    // Draw text
                    let text_texture = draw_string_texture(
                        item.title.clone(),
                        texture_creator,
                        font,
                        self.foreground_color,
                    );
                    let query = text_texture.query();
                    let (w, h) = (query.width, query.height);
                    let hpad = (rect.height() - h) / 2;
                    canvas
                        .copy(&text_texture, None, Some(Rect::new(34, hpad as i32, w, h)))
                        .unwrap();
                }

                // Draw tag
                let tag_texture = draw_string_texture(
                    format!(":{}", item.action.tags().get(0).unwrap().clone()),
                    texture_creator,
                    font,
                    Color::RGBA(128, 128, 128, 128),
                );
                let query = tag_texture.query();
                let (w, h) = (query.width, query.height);
                let hpad = (rect.height() - h) / 2;
                canvas
                    .copy(
                        &tag_texture,
                        None,
                        Some(Rect::new((rect.width() - w - 5) as i32, hpad as i32, w, h)),
                    )
                    .unwrap();
            })
            .unwrap();
        tex
    }
}
impl<T: PartialEq> EventConsumer for SelectList<T> {
    fn consume_event(&mut self, event: &Event, app: &mut App) {
        match event {
            sdl2::event::Event::KeyDown {
                keycode: Some(Keycode::Return),
                ..
            } => {
                if self.get_selected_item().as_ref().is_some() {
                    (self.on_select)(self.get_selected_item().as_ref().unwrap(), app)
                }
            }
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
            sdl2::event::Event::KeyDown {
                keycode: Some(Keycode::Up),
                ..
            } => self.select_up(),
            sdl2::event::Event::KeyDown {
                keycode: Some(Keycode::Down),
                ..
            } => self.select_down(),
            sdl2::event::Event::MouseWheel { y, .. } => {
                if *y == 1 {
                    self.move_viewport_up();
                } else {
                    self.move_viewport_down();
                }
            }
            sdl2::event::Event::MouseButtonDown { x, y, .. } => {
                println!("{} {}", x, y)
            }
            _ => (),
        }
    }
}
