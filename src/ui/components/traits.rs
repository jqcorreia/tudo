use std::any::Any;
use std::fmt::Debug;

use crate::app::App;
use crate::utils::cache::TextureCache;
use sdl2::event::Event;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{BlendMode, Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};

pub trait Render {
    fn id(&self) -> String;
    fn render(
        &mut self,
        texture_creator: &TextureCreator<WindowContext>,
        cache: &mut TextureCache,
        app: &App,
        canvas: &mut Canvas<Window>,
        rect: Rect,
        elapsed: u128,
    );

    fn draw(
        &mut self,
        texture_creator: &TextureCreator<WindowContext>,
        mut cache: &mut TextureCache,
        app: &App,
        main_canvas: &mut Canvas<Window>,
        component_rect: Rect,
        elapsed: u128,
    ) {
        if app.layout_debug {
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
                self.render(
                    &texture_creator,
                    &mut cache,
                    &app,
                    c,
                    component_rect,
                    elapsed,
                );
            })
            .unwrap();

        main_canvas.copy(&tex, None, component_rect).unwrap();
    }
}

pub trait EventConsumer {
    fn consume_event(&mut self, event: &Event, app: &mut App);
}

pub trait UIComponent: Render + EventConsumer {
    fn get_state(&self) -> &dyn Any;
    fn set_state(&mut self, state: Box<dyn Any>);
}

impl Debug for dyn UIComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Id({})", self.id())
    }
}
