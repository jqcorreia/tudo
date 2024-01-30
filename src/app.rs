use std::collections::HashMap;

use sdl2::render::Canvas;
use sdl2::ttf::Font;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::Window;
use sdl2::Sdl;
use sdl2::VideoSubsystem;

pub struct App<'a> {
    pub sdl: Sdl,
    pub running: bool,
    pub clipboard: Option<String>,
    pub video: VideoSubsystem,
    fonts: HashMap<String, Font<'a, 'a>>,
    pub ttf: &'a Sdl2TtfContext,
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

    pub fn load_font(&mut self, font_id: String, path: String, point_size: u16) {
        let font = self.ttf.load_font(&path, point_size).unwrap();

        self.fonts.insert(font_id, font);
    }

    pub fn get_font(&'a self, font_id: &'a str) -> &'a Font<'a, 'a> {
        self.fonts.get(font_id).unwrap()
    }
}
pub fn init<'a>(ttf: &'a Sdl2TtfContext) -> (App<'a>, Canvas<Window>) {
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
    // let tc = canvas.texture_creator();
    // let texture_cache = TextureCache::new(&tc);

    (
        App {
            sdl,
            running: true,
            clipboard: None,
            video,
            fonts: HashMap::new(),
            ttf,
        },
        canvas,
    )
}
