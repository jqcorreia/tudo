use std::any::Any;
use std::usize;

use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{BlendMode, Texture, TextureCreator};
use sdl2::video::WindowContext;
use sdl2::{event::Event, pixels::Color, rect::Rect, render::Canvas, ttf::Font, video::Window};

use crate::animation::{Animation, AnimationType};
use crate::sources::SourceItem;
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
        is_selected: bool,
        index: usize,
    ) -> Texture;
}

pub struct Viewport {
    top: i32,
    bottom: i32,
    anim: Animation,
}

impl Viewport {
    pub fn down(&mut self, amount: i32) {
        self.top += amount;
        self.bottom += amount;
        self.anim.set_target(self.top as u32, None);
    }
    pub fn up(&mut self, amount: i32) {
        self.top = std::cmp::max(0, self.top - amount);
        self.bottom -= amount;
        self.anim.set_target(self.top as u32, None);
    }
}

pub struct RenderViewport(pub i32, pub i32, pub i32);

impl RenderViewport {
    pub fn down(&mut self, amount: i32) {
        self.0 += amount;
        self.1 += amount;
    }
    pub fn up(&mut self, amount: i32) {
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
    pub render_viewport: RenderViewport,
    pub on_select: fn(&T, &mut App),
    pub vertical_bar_width: u32,
    pub row_height: u32,
    pub last_mouse_y: i32,
}

impl UIComponent for SelectList<SourceItem> {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

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

        if prompt.is_empty() {
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
                    .map(|i| i.action.tags().first().unwrap().clone())
                    .collect::<Vec<String>>();
                matches = basic_contains(tag.to_string(), &haystack).unwrap_or_default();

                // if filter present, further filter the list with another haystack
                if !filter.is_empty() {
                    let mut list2 = Vec::new();
                    for m in matches.iter() {
                        list2.push(new_list.get(m.original_idx).unwrap().clone());
                    }

                    let haystack2 = list2
                        .iter()
                        .map(|i| i.title.clone())
                        .collect::<Vec<String>>();
                    matches = basic_contains(filter.to_string(), &haystack2).unwrap_or_default();

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
                matches = basic_contains(prompt.to_string(), &haystack).unwrap_or_default();

                for m in matches.iter() {
                    final_list.push(new_list.get(m.original_idx).unwrap().clone());
                }
            }
        }

        // Sort by title
        final_list.sort_by(|this, other| this.title.cmp(&other.title));
        self.set_list(final_list);
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
        let font = cache.fonts.get_font("normal-20");
        let font2 = cache.fonts.get_font("normal-16");

        canvas.set_draw_color(Color::BLACK);

        canvas.draw_filled_rounded_rect(Rect::new(0, 0, rect.w as u32, rect.h as u32), 7);
        if self.items.is_empty() {
            draw_string(
                "No items found".to_string(),
                canvas,
                font,
                self.foreground_color,
                10,
                10,
            );
        } else {
            let mut y: u32 = 0;

            let mut all_rows = texture_creator
                .create_texture_target(
                    PixelFormatEnum::RGBA8888,
                    rect.w as u32,
                    self.items.len() as u32 * self.row_height,
                )
                .unwrap();

            self.render_viewport.1 = self.render_viewport.0 + rect.h;
            self.viewport.bottom = self.viewport.top + rect.h;
            let rv0 = self.viewport.anim.tick(elapsed);

            for (idx, item) in self.items.as_slice().iter().enumerate() {
                // if idx < self.viewport.0 || idx > self.viewport.1 {
                //     continue;
                // }

                let _y = y as i32;
                if _y < rv0 as i32 - self.row_height as i32 || y >= rv0 + rect.h as u32 {
                    y += self.row_height;
                    continue;
                }

                let row_texture = self.render_row(
                    item,
                    texture_creator,
                    cache,
                    font2,
                    canvas,
                    Rect::new(0, 0, rect.w as u32, self.row_height),
                    elapsed,
                    idx == self.selected_index,
                    idx,
                );
                canvas
                    .with_texture_canvas(&mut all_rows, |c| {
                        c.copy(
                            &row_texture,
                            None,
                            Some(Rect::new(0, y as i32, rect.w as u32, self.row_height)),
                        )
                        .unwrap();
                    })
                    .unwrap();

                y += self.row_height;
            }
            let source_rect = Rect::new(
                0,
                rv0 as i32,
                rect.w as u32,
                std::cmp::min(rect.h as u32, y),
            );
            let destination_rect = Rect::new(0, 0, rect.w as u32, std::cmp::min(y, rect.h as u32));
            canvas
                .copy(&all_rows, Some(source_rect), Some(destination_rect))
                .unwrap();
        }
    }
    fn handle_event(&mut self, event: &Event, app: &mut App, _: u128) {
        match event {
            sdl2::event::Event::KeyDown {
                keycode: Some(Keycode::Return),
                ..
            }
            | sdl2::event::Event::MouseButtonDown { .. } => {
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
                    self.viewport.up(self.row_height as i32);
                } else {
                    let tex_h = self.items.len() as i32 * self.row_height as i32;
                    if self.viewport.bottom < tex_h {
                        self.viewport.down(self.row_height as i32);
                    }
                }
            }
            sdl2::event::Event::MouseMotion { y, .. } => {
                self.last_mouse_y = *y;
                self.set_selected_index(
                    ((self.viewport.anim.value as i32 + self.last_mouse_y) / self.row_height as i32)
                        as usize,
                );
            }
            _ => (),
        }
    }

    fn update(&mut self, _: &mut App, _: u128) {}
}

impl<T: PartialEq> SelectList<T> {
    pub fn new(id: impl AsRef<str>) -> SelectList<T> {
        SelectList {
            id: id.as_ref().to_string(),
            items: Vec::<T>::new(),
            selected_index: 0,
            foreground_color: Color::RGBA(255, 255, 255, 255),
            viewport: Viewport {
                top: 0,
                bottom: 10,
                anim: Animation::new(0, 0, AnimationType::EaseOut, 100.0),
            },
            render_viewport: RenderViewport(0, 100, 0),
            vertical_bar_width: 5,
            on_select: |_, _| (),
            row_height: 34, // Make this the same height as the font
            last_mouse_y: 0,
        }
    }
    pub fn with_on_select(mut self, func: fn(&T, &mut App)) -> Self {
        self.on_select = func;
        self
    }
    pub fn select_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
            if (self.selected_index as i32) < self.viewport.top / self.row_height as i32 {
                self.viewport.up(self.row_height as i32);
            }
        }
    }
    pub fn select_down(&mut self) {
        if !self.items.is_empty() && self.selected_index < self.items.len() - 1 {
            self.selected_index += 1;
            if (self.selected_index as i32) > self.viewport.bottom / self.row_height as i32 {
                self.viewport.down(self.row_height as i32);
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
        self.viewport = Viewport {
            top: 0,
            bottom: 10,
            anim: Animation::new(0, 0, AnimationType::EaseOut, 100.0),
        };
        self.render_viewport = RenderViewport(0, 10, 0); // The bottom setting dynamic, this 10 is irrelevant
    }

    pub fn get_selected_item(&self) -> Option<&T> {
        match self.items.get(self.selected_index) {
            None => None,
            Some(item) => Some(item),
        }
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
        is_selected: bool,
        index: usize,
    ) -> Texture {
        let mut tex = texture_creator
            .create_texture_target(PixelFormatEnum::RGBA8888, rect.w as u32, rect.h as u32)
            .unwrap();

        let padding = 3;
        let vertical_bar_spacing = self.vertical_bar_width as i32 + padding;

        tex.set_blend_mode(BlendMode::Blend);
        canvas
            .with_texture_canvas(&mut tex, |canvas| {
                //canvas.set_draw_color(Color::BLUE);
                if is_selected {
                    canvas.set_draw_color(Color::RGBA(20, 20, 50, 255));
                    canvas.clear();

                    canvas.set_draw_color(Color::RGBA(0, 0, 255, 255));
                    canvas
                        .fill_rect(Rect::new(0, 0, self.vertical_bar_width, rect.h as u32))
                        .unwrap();
                } else if index % 2 == 0 {
                    canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
                    canvas.clear();
                } else {
                    canvas.set_draw_color(Color::RGBA(10, 10, 10, 255));
                    canvas.clear();
                }

                // Draw icon
                let icon_height: u32 = 32;
                if item.icon.is_some() {
                    let icon_texture = cache.images.get_image(item.icon.as_ref().unwrap());
                    canvas
                        .copy(
                            icon_texture,
                            None,
                            Rect::new(vertical_bar_spacing, 0, icon_height, icon_height),
                        )
                        .unwrap();
                }

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
                    .copy(
                        &text_texture,
                        None,
                        Some(Rect::new(vertical_bar_spacing + 34, hpad as i32, w, h)),
                    )
                    .unwrap();

                // Draw tag
                let tag_texture = draw_string_texture(
                    format!(":{}", item.action.tags().first().unwrap().clone()),
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
