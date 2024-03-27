use sdl2::{
    event::Event,
    pixels::Color,
    rect::Rect,
    render::{Canvas, TextureCreator},
    video::{Window, WindowContext},
};

use crate::{
    app::App,
    config::Config,
    ui::components::{label::Label, spinner::Spinner, text::Prompt},
    ui::layout::{ContainerSize, LayoutBuilder, SplitType},
    utils::{
        cache::TextureCache,
        draw::{draw_filled_circle_quadrants, draw_filled_rounded_rect, DrawExtensions},
    },
};

use super::Screen;

pub struct InfoScreen {
    layout: LayoutBuilder,
}

impl InfoScreen {
    pub fn new(config: &Config) -> InfoScreen {
        let mut builder = LayoutBuilder::new();

        let main_split = builder.add_split(SplitType::Vertical, ContainerSize::Percent(100));
        for s in [String::from("foo"), String::from("bar")] {
            builder.add(
                Box::new(Label::new(s.clone(), s.clone())),
                ContainerSize::Percent(20),
            );
        }

        InfoScreen { layout: builder }
    }
}

impl Screen for InfoScreen {
    fn update(&mut self, app: &mut App, events: &Vec<Event>, elapsed: u128) {
        for event in events.iter() {
            for component in self.layout.components() {
                component.update(&event, app, elapsed);
            }
        }
    }

    fn render(
        &mut self,
        texture_creator: &TextureCreator<WindowContext>,
        cache: &mut TextureCache,
        app: &App,
        main_canvas: &mut Canvas<Window>,
        elapsed: u128,
    ) {
        let (width, height) = main_canvas.window().size();
        self.layout.generate(width as usize, height as usize);

        // Set draw color and clear
        let clear_color = Color::RGBA(24, 24, 33, 255);
        main_canvas.set_draw_color(clear_color);
        main_canvas.clear();

        for (rect, component) in self.layout.components_with_rect() {
            component.draw(&texture_creator, cache, &app, main_canvas, rect, elapsed);
        }
    }
}
