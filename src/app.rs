use sdl2::video::Window;
use sdl2::Sdl;
use sdl2::VideoSubsystem;

pub struct App {
    pub sdl: Sdl,
    pub running: bool,
    pub clipboard: Option<String>,
    pub video: VideoSubsystem,
}

impl App {
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
