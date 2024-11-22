use sdl2::{pixels::Color, rect::Rect};

use crate::{
    app,
    ui::layout::{ContainerSize, LayoutBuilder, SplitType},
    utils::{hyprland::open_hyprland_socket_1, misc::localize_mouse_event},
};
use std::io::prelude::*;

use super::{button::Button, traits::UIComponent};

pub struct Workspaces {
    pub id: String,
    pub selected_workspace: u8,
    pub font_name: Option<String>,
    builder: LayoutBuilder,
}

fn goto_workspace(x: u8) {
    let mut stream = open_hyprland_socket_1();

    stream.write_all(format!("/dispatch workspace {}", x).as_bytes());
    let mut response = String::new();
    stream.read_to_string(&mut response);
    dbg!(response);
}
impl Workspaces {
    pub fn new(id: String) -> Workspaces {
        let mut builder = LayoutBuilder::new().with_gap(1);
        builder.add_split(SplitType::Horizontal, ContainerSize::Percent(100));

        for x in 1..9 {
            builder.add(
                Box::new(
                    Button::new(x.to_string(), x.to_string()).with_on_click(|btn, app| {
                        goto_workspace(btn.id().parse::<u8>().unwrap());
                        app.should_hide = true;
                    }),
                ),
                ContainerSize::Fixed(64),
            );
        }
        // builder.add(
        //     Box::new(Button::new(String::from("BAR"))),
        //     ContainerSize::Fixed(64),
        // );

        Workspaces {
            id,
            selected_workspace: 1,
            font_name: None,
            builder,
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
        app: &crate::app::App,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        rect: Rect,
        elapsed: u128,
    ) {
        self.builder.generate(rect.w as usize, rect.h as usize);
        // Set draw color and clear
        let clear_color = Color::RGBA(24, 24, 33, 255);
        canvas.set_draw_color(clear_color);
        canvas.clear();

        for (rect, component) in self.builder.components_with_rect() {
            component.draw(&texture_creator, cache, &app, canvas, rect, elapsed);
        }

        // let font = cache
        //     .fonts
        //     .get_font(self.font_name.clone().unwrap_or("normal-20".to_string()));

        // canvas.set_draw_color(Color::RGBA(0x00, 0x00, 0x00, 0xFF));
        // canvas.clear();
        // let rect_height = rect.h - 3;
        // let rect_y = (rect.h - rect_height) / 2;
        // let rect_width = rect_height; // Make width == height to produce squares
        // let text_x_padding = 6; // Leave this as constant for now
        // let text_y_padding = -1; // Leave this as constant for now
        // for workspace_id in 1..10 {
        //     let x = workspace_id - 1; // Workspaces are 1-based but screen positions are '0-based'
        //     let rect_x = rect_width * x + 10 * x;
        //     let rect = Rect::new(rect_x, rect_y, rect_width as u32, rect_height as u32);
        //     draw_rounded_rect(canvas, rect, 3, Color::RGBA(0x30, 0x30, 0x50, 255));
        //     draw_string_texture_canvas(
        //         canvas,
        //         rect_x + text_x_padding,
        //         rect_y + text_y_padding,
        //         workspace_id.to_string(),
        //         font,
        //         Color::GRAY,
        //     );
        // }
    }
    fn update(&mut self, event: &sdl2::event::Event, app: &mut crate::app::App, elapsed: u128) {
        match event {
            sdl2::event::Event::MouseMotion { .. }
            | sdl2::event::Event::MouseButtonDown { .. }
            | sdl2::event::Event::MouseButtonUp { .. } => {
                for (rect, component) in self.builder.components_with_rect() {
                    let (_event, contains) = localize_mouse_event(event, rect);
                    if contains {
                        component.update(&_event, app, elapsed);
                    }
                }
            }
            _ => {
                for component in self.builder.components() {
                    component.update(&event, app, elapsed);
                }
            }
        }
    }

    fn get_state(&self) -> &dyn std::any::Any {
        return &self.selected_workspace;
    }

    fn set_state(&mut self, state: Box<dyn std::any::Any>) {
        self.selected_workspace = *state.downcast_ref::<u8>().unwrap();
    }
}
