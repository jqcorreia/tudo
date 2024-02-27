use sdl2::{
    event::Event,
    render::{Canvas, TextureCreator},
    video::{Window, WindowContext},
};

use crate::{app::App, utils::cache::TextureCache};

pub mod debug_screen;
pub mod main_screen;

pub trait Screen {
    fn init(&mut self) {}

    fn update(&mut self, app: &mut App, events: &Vec<Event>, _elapsed: u128);
    fn render(
        &mut self,
        texture_creator: &TextureCreator<WindowContext>,
        cache: &mut TextureCache,
        app: &App,
        main_canvas: &mut Canvas<Window>,
        elapsed: u128,
    );
}
