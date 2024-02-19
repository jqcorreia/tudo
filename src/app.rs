use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;
use sdl2::VideoSubsystem;

use crate::config::load_config;
use crate::config::Config;

pub struct App {
    pub sdl: Sdl,
    pub video: VideoSubsystem,

    pub clipboard: Option<String>,
    pub running: bool,
    pub draw_fps: bool,
    pub frame_lock: bool,
    pub loading: bool,
    pub current_screen_id: String,
    pub config: Config,
    pub lock_path: String,
}

fn already_running(lock_path: &String) -> bool {
    match std::fs::read(lock_path.clone()) {
        Ok(_) => true,
        Err(_) => {
            std::fs::write(lock_path, Vec::new()).unwrap();
            false
        }
    }
}

fn check_config_folder() -> String {
    let home = std::env::var("HOME").expect("$HOME not set, can't create config folder");
    let base_path = format!("{}/.config/tudo", home);

    std::fs::create_dir_all(base_path.clone()).unwrap();
    base_path.to_string()
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

        let base_folder = check_config_folder();
        let config = load_config(format!("{}/config.lua", base_folder));
        let canvas = window.into_canvas().build().unwrap();
        let lock_path = format!("{}/run-lock", base_folder);

        if already_running(&lock_path) {
            panic!("Tudo is already running!");
        }

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
                config,
                lock_path,
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
