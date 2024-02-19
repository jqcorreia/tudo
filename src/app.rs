use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;
use sdl2::VideoSubsystem;

pub struct App {
    pub sdl: Sdl,
    pub video: VideoSubsystem,

    pub clipboard: Option<String>,
    pub running: bool,
    pub draw_fps: bool,
    pub frame_lock: bool,
    pub loading: bool,
    pub current_screen_id: String,
}

impl App {
    pub fn init() -> (App, Canvas<Window>) {
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

                running: true,
                frame_lock: true,
                draw_fps: false,
                loading: true,
                current_screen_id: "main".to_string(),
            },
            canvas,
        )
    }

    pub fn handle_global_events(&mut self, events: &Vec<Event>) {
        for event in events.iter() {
            // Deal with main loop events
            // Things like app quit and global window mouse events
            match event {
                sdl2::event::Event::KeyDown {
                    keycode: Some(Keycode::F1),
                    ..
                } => self.draw_fps = !self.draw_fps,
                sdl2::event::Event::KeyDown {
                    keycode: Some(Keycode::F2),
                    ..
                } => self.frame_lock = !self.frame_lock,
                sdl2::event::Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    if &self.current_screen_id == "main" {
                        self.running = false
                    } else {
                        self.current_screen_id = "main".to_string()
                    }
                }
                sdl2::event::Event::KeyDown {
                    keycode: Some(Keycode::F3),
                    ..
                } => self.current_screen_id = "submenu".to_string(),
                sdl2::event::Event::Quit { .. } => self.running = false,
                _ => (),
            }
        }
    }
}
