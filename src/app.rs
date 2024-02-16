use std::collections::HashMap;

use sdl2::render::Canvas;
use sdl2::ttf::Font;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::Window;
use sdl2::Sdl;
use sdl2::VideoSubsystem;

pub struct App<'a> {
    pub sdl: Sdl,
    pub clipboard: Option<String>,
    pub video: VideoSubsystem,
    fonts: HashMap<String, Font<'a, 'a>>,
    pub ttf: &'a Sdl2TtfContext,

    pub running: bool,
    pub draw_fps: bool,
    pub frame_lock: bool,
    pub loading: bool,
}

impl<'a> App<'a> {
    // pub fn create_window(&self, title: String, width: u32, height: u32) -> Window {
    //     let window = self
    //         .video
    //         .window(&title, width, height)
    //         .opengl()
    //         .borderless()
    //         .position_centered()
    //         .build()
    //         .unwrap();
    //     window
    // }

    pub fn load_font(&mut self, font_id: String, path: impl AsRef<str>, point_size: u16) {
        let font = self.ttf.load_font(&path.as_ref(), point_size).unwrap();

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

    (
        App {
            sdl,
            clipboard: None,
            video,
            fonts: HashMap::new(),
            ttf,

            running: true,
            frame_lock: true,
            draw_fps: false,
            loading: true,
        },
        canvas,
    )
}
