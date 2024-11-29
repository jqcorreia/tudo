use std::{ops::Rem, usize};

use sdl2::{pixels::PixelFormatEnum, rect::Rect};

use super::traits::UIComponent;

pub struct Spinner {
    pub id: String,
    pub running: bool,
    pub period_millis: u128,
}

impl Spinner {
    pub fn new(id: String) -> Spinner {
        Spinner {
            id,
            running: true,
            period_millis: 1000,
        }
    }
}
impl UIComponent for Spinner {
    fn id(&self) -> String {
        self.id.clone()
    }
    fn render(
        &mut self,
        texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        _cache: &mut crate::utils::cache::TextureCache,
        _app: &crate::app::App,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        rect: Rect,
        elapsed: u128,
    ) {
        let smallest_dim = std::cmp::min(rect.height(), rect.width()) as usize;
        let mut buf: Vec<u8> = vec![0; smallest_dim * smallest_dim * 4];

        if self.running {
            let c = (((elapsed.rem(self.period_millis) as f32 / self.period_millis as f32)
                * 360.0)
                .to_radians()
                .sin()
                + 1.0)
                / 2.0
                * smallest_dim as f32;

            let _b = (elapsed.rem(self.period_millis)) as f32
                * (smallest_dim as f32 / self.period_millis as f32);
            let cx = smallest_dim / 2;
            let cy = smallest_dim / 2;
            let radius: usize = smallest_dim / 2 - 10;

            for y in 0..smallest_dim {
                for x in 0..smallest_dim {
                    let _x = x as i32;
                    let _y = y as i32;
                    if (_x - cx as i32).pow(2) + (_y - cy as i32).pow(2) > radius.pow(2) as i32 {
                        continue;
                    }
                    let blue = (x.abs_diff(c as usize) as f32 / smallest_dim as f32) * 255.0;

                    buf[y * (smallest_dim * 4) + (x * 4)] = 0;
                    buf[y * (smallest_dim * 4) + (x * 4) + 1] = 0;
                    buf[y * (smallest_dim * 4) + (x * 4) + 2] = blue as u8;
                    buf[y * (smallest_dim * 4) + (x * 4) + 3] = 255;
                }
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

    fn update(&mut self, _event: &sdl2::event::Event, _app: &mut crate::app::App, _elapsed: u128) {}

    fn get_state(&self) -> &dyn std::any::Any {
        &self.running
    }

    fn set_state(&mut self, state: Box<dyn std::any::Any>) {
        self.running = *state.downcast_ref::<bool>().unwrap();
    }
}
