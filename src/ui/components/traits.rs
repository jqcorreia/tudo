use std::any::Any;
use std::fmt::Debug;

use crate::app::App;
use crate::utils::cache::TextureCache;
use sdl2::event::Event;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::{BlendMode, Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};

pub trait UIComponent {
    fn id(&self) -> String;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn render(
        &mut self,
        texture_creator: &TextureCreator<WindowContext>,
        cache: &mut TextureCache,
        app: &App,
        canvas: &mut Canvas<Window>,
        rect: Rect,
        elapsed: u128,
    );

    // Standard function that sets the texture based on the rect and copies it to the main canvas
    // in the correct position and with the correct size
    fn draw(
        &mut self,
        texture_creator: &TextureCreator<WindowContext>,
        cache: &mut TextureCache,
        app: &App,
        main_canvas: &mut Canvas<Window>,
        component_rect: Rect,
        elapsed: u128,
    ) {
        if app.layout_debug {
            main_canvas.set_draw_color(Color::GREEN);
            main_canvas.draw_rect(component_rect).unwrap();
        }

        let mut tex = texture_creator
            .create_texture_target(
                PixelFormatEnum::RGBA8888,
                component_rect.width(),
                component_rect.height(),
            )
            .unwrap();
        tex.set_blend_mode(BlendMode::Blend);
        main_canvas
            .with_texture_canvas(&mut tex, |c| {
                self.render(texture_creator, cache, app, c, component_rect, elapsed);
            })
            .unwrap();

        main_canvas.copy(&tex, None, component_rect).unwrap();
    }
    fn handle_event(&mut self, event: &Event, app: &mut App, elapsed: u128);
    fn update(&mut self, app: &mut App, elapsed: u128);

    fn get_state(&self) -> &dyn Any;
    fn set_state(&mut self, state: Box<dyn Any>);

    fn set_focus(&mut self, _: bool) {}

    fn get_focus(&self) -> bool {
        false
    }
}

impl Debug for dyn UIComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Id({})", self.id())
    }
}
