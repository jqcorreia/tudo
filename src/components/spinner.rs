use std::usize;

use sdl2::{pixels::PixelFormatEnum, rect::Rect, surface::Surface};

use super::traits::{EventConsumer, Render, UIComponent};

pub struct Spinner {
    pub id: String,
    pub running: bool,
}

impl Render for Spinner {
    fn id(&self) -> String {
        self.id.clone()
    }
    fn render(
        &mut self,
        texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        cache: &mut crate::utils::cache::TextureCache,
        app: &crate::app::App,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        rect: Rect,
        elapsed: u128,
    ) {
        let smallest_dim = std::cmp::min(rect.height(), rect.width()) as usize;
        let mut buf: Vec<u8> = vec![0; smallest_dim * smallest_dim * 4];

        for y in 0..smallest_dim {
            for x in 0..smallest_dim {
                let blue = (x as f32 / smallest_dim as f32) * 255.0;

                buf[y * (smallest_dim * 4) + (x * 4)] = 0;
                buf[y * (smallest_dim * 4) + (x * 4) + 1] = 0;
                buf[y * (smallest_dim * 4) + (x * 4) + 2] = blue as u8;
                buf[y * (smallest_dim * 4) + (x * 4) + 3] = 255;
            }
        }

        let mut tex = texture_creator
            .create_texture_target(
                PixelFormatEnum::RGBA32,
                smallest_dim as u32,
                smallest_dim as u32,
            )
            .unwrap();
        tex.update(None, buf.as_slice(), smallest_dim * 4).unwrap();

        canvas
            .copy(
                &tex,
                None,
                Rect::new(0, 0, smallest_dim as u32, smallest_dim as u32),
            )
            .unwrap();
    }
}
impl EventConsumer for Spinner {
    fn consume_event(&mut self, event: &sdl2::event::Event, app: &mut crate::app::App) {}
}

impl UIComponent for Spinner {
    fn get_state(&self) -> &dyn std::any::Any {
        return &self.running;
    }
    fn set_state(&mut self, state: Box<dyn std::any::Any>) {}
}
