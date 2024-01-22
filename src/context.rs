use sdl2::{render::TextureCreator, video::WindowContext, Sdl, VideoSubsystem};

use crate::utils::font::FontManager;

pub struct RenderContext<'a> {
    // pub sdl: Sdl,
    // pub tc: &'a TextureCreator<WindowContext>,
    // pub video: VideoSubsystem,
    pub fonts: &'a FontManager<'a>,
}
