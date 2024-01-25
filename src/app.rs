use std::collections::HashMap;

use sdl2::render::Canvas;
use sdl2::ttf::Font;
use sdl2::video::Window;
use sdl2::video::WindowContext;
use sdl2::Sdl;
use sdl2::VideoSubsystem;

use crate::utils::cache::TextureCache;

pub struct App<'a> {
    pub sdl: Sdl,
    pub running: bool,
    pub clipboard: Option<String>,
    pub video: VideoSubsystem,
    pub fonts: HashMap<String, Font<'a, 'a>>,
    pub canvas: Canvas<Window>,
}

impl<'a> App<'a> {
    pub fn create_window(&self) -> Window {
        let window = self
            .video
            .window("tudo", 1024, 768)
            .opengl()
            .borderless()
            .position_centered()
            .build()
            .unwrap();
        window
    }
}
pub fn init<'a>() -> App<'a> {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();

    let window = video
        .window("tudo", 1024, 768)
        .opengl()
        .borderless()
        .position_centered()
        .build()
        .unwrap();
    let canvas = window.into_canvas().build().unwrap();

    App {
        sdl,
        running: true,
        clipboard: None,
        video,
        fonts: HashMap::new(),
        canvas,
    }
}
