use sdl2::{render::TextureCreator, video::WindowContext, Sdl, VideoSubsystem};

pub struct RenderContext {
    sdl: Sdl,
    tc: TextureCreator<WindowContext>,
    video: VideoSubsystem,
}
