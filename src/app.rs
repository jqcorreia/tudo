use log::info;
use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::Sdl;
use sdl2::VideoSubsystem;

use crate::config::load_config;
use crate::config::Config;
use crate::utils::hyprland::Hyprland;

pub struct App {
    pub sdl: Sdl,
    pub video: VideoSubsystem,
    pub event_pump: EventPump,

    pub clipboard: Option<String>,
    pub running: bool,
    pub draw_fps: bool,
    pub frame_lock: bool,
    pub loading: bool,
    pub current_screen_id: String,
    pub config: Config,
    pub layout_debug: bool,
    pub ctrl_pressed: bool,
    pub hyprland: std::io::Result<Hyprland>,

    pub should_hide: bool,
    pub hidden: bool,
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
        let event_pump = sdl.event_pump().unwrap();

        let window = video
            .window("tudo", 1024, 768)
            .opengl()
            .borderless()
            .position_centered()
            .build()
            .unwrap();

        info!("Computing base folder");
        let base_folder = check_config_folder();
        info!("Base folder: {}", &base_folder);

        info!("Loading config: {}", format!("{}/config.lua", base_folder));
        let config = load_config(format!("{}/config.lua", base_folder));
        info!("Initializing window canvas");
        let canvas = window.into_canvas().build().unwrap();
        info!("Finished initializing canvas");

        let hyprland = Hyprland::new();

        (
            App {
                sdl,
                clipboard: None,
                video,
                event_pump,

                running: true,
                frame_lock: config.frame_lock,
                draw_fps: false,
                loading: true,
                current_screen_id: "main".to_string(),
                config,
                layout_debug: false,
                ctrl_pressed: false,
                hyprland,

                should_hide: false,
                hidden: false,
            },
            canvas,
        )
    }

    pub fn handle_global_events(&mut self, events: &Vec<Event>) {
        for event in events.iter() {
            // Deal with main loop events
            // Things like app quit and global window mouse events
            match event {
                // Trap ctrl in order to bypass a bug in SDL2 and wayland
                // Set it on the application state so other components can react to it
                // Text components need this to ignore TextInput events when Ctrl is pressed
                sdl2::event::Event::KeyDown {
                    keycode: Some(Keycode::LCtrl),
                    ..
                } => self.ctrl_pressed = true,
                sdl2::event::Event::KeyUp {
                    keycode: Some(Keycode::LCtrl),
                    ..
                } => self.ctrl_pressed = false,
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
                        self.should_hide = true;
                    } else {
                        self.current_screen_id = "main".to_string()
                    }
                }
                sdl2::event::Event::KeyDown {
                    keycode: Some(Keycode::F3),
                    ..
                } => self.current_screen_id = "debug".to_string(),
                sdl2::event::Event::KeyDown {
                    keycode: Some(Keycode::F4),
                    ..
                } => self.current_screen_id = "info".to_string(),
                sdl2::event::Event::Window {
                    win_event: WindowEvent::FocusLost,
                    ..
                } => self.should_hide = true,
                sdl2::event::Event::Quit { .. } => self.running = false,
                _ => (),
            }
        }
    }
}

impl Drop for App {
    fn drop(&mut self) {
        // App destructor
    }
}
