use sdl2::{pixels::Color, rect::Rect};

use crate::utils::draw::{draw_filled_rounded_rect, draw_rounded_rect, draw_string_texture_canvas};

use super::traits::UIComponent;
use crate::utils::draw::DrawExtensions;

pub struct Workspaces {
    pub id: String,
    pub selected_workspace: u8,
    pub font_name: Option<String>,
}

impl Workspaces {
    pub fn new(id: String) -> Workspaces {
        Workspaces {
            id,
            selected_workspace: 1,
            font_name: None,
        }
    }
}

impl UIComponent for Workspaces {
    fn id(&self) -> String {
        return self.id.clone();
    }
    fn render(
        &mut self,
        texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        cache: &mut crate::utils::cache::TextureCache,
        _app: &crate::app::App,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        rect: Rect,
        _elapsed: u128,
    ) {
        let font = cache
            .fonts
            .get_font(self.font_name.clone().unwrap_or("normal-20".to_string()));

        canvas.set_draw_color(Color::RGBA(0x00, 0x00, 0x00, 0xFF));
        canvas.clear();
        let rect_height = rect.h - 3;
        let rect_y = (rect.h - rect_height) / 2;
        let rect_width = rect_height; // Make width == height to produce squares
        let text_x_padding = 6; // Leave this as constant for now
        let text_y_padding = -1; // Leave this as constant for now
        for x in 1..10 {
            let _x = x - 1;
            let rect_x = rect_width * _x + 10 * _x;
            let rect = Rect::new(rect_x, rect_y, rect_width as u32, rect_height as u32);
            draw_rounded_rect(canvas, rect, 3, Color::RGBA(0x30, 0x30, 0x50, 255));
            draw_string_texture_canvas(
                canvas,
                rect_x + text_x_padding,
                rect_y + text_y_padding,
                x.to_string(),
                font,
                Color::GRAY,
            );
        }
        // let color = Color::RGBA(100, 100, 100, 255);
        // let texture = draw_string_texture(self.text.clone(), texture_creator, font, color);
        // let (w, h) = (texture.query().width, texture.query().height);

        // canvas.copy(&texture, None, Rect::new(0, 0, w, h)).unwrap();
    }
    fn update(&mut self, _event: &sdl2::event::Event, _app: &mut crate::app::App, elapsed: u128) {}
    fn get_state(&self) -> &dyn std::any::Any {
        return &self.selected_workspace;
    }

    fn set_state(&mut self, state: Box<dyn std::any::Any>) {
        self.selected_workspace = *state.downcast_ref::<u8>().unwrap();
    }
}
