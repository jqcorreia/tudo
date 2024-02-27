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
    ui::components::{spinner::Spinner, text::Prompt},
    ui::layout::{ContainerSize, LayoutBuilder, SplitType},
    utils::{
        cache::TextureCache,
        draw::{draw_filled_circle_quadrants, draw_filled_rounded_rect, DrawExtensions},
    },
};

use super::Screen;

pub struct DebugScreen {
    layout: LayoutBuilder,
}

impl DebugScreen {
    pub fn new(config: &Config) -> DebugScreen {
        let text1 = Prompt::new("t1", config);
        let text2 = Prompt::new("t2", config);
        let spinner = Spinner {
            id: "spinner".to_string(),
            running: true,
            period_millis: 1000,
        };
        let mut builder = LayoutBuilder::new();

        // builder.add_split(SplitType::Vertical);
        builder.add_split(SplitType::Horizontal, ContainerSize::Percent(100));
        builder.add(Box::new(text1), ContainerSize::Fixed(200));
        builder.add(Box::new(text2), ContainerSize::Fixed(200));
        builder.add(Box::new(spinner), ContainerSize::Fixed(200));

        builder.generate(1000, 500);
        DebugScreen { layout: builder }
    }
}

impl Screen for DebugScreen {
    fn update(&mut self, app: &mut App, events: &Vec<Event>, _elapsed: u128) {
        for event in events.iter() {
            for component in self.layout.components() {
                component.consume_event(&event, app);
            }
        }
    }

    fn render(
        &mut self,
        _texture_creator: &TextureCreator<WindowContext>,
        _cache: &mut TextureCache,
        _app: &App,
        main_canvas: &mut Canvas<Window>,
        _elapsed: u128,
    ) {
        main_canvas.set_draw_color(Color::RGBA(30, 30, 30, 255));
        main_canvas.clear();
        // for (rect, component) in self.layout.components_with_rect() {
        //     component.draw(&texture_creator, cache, &app, main_canvas, rect, elapsed);
        // }

        main_canvas.set_draw_color(Color::RGBA(0, 255, 0, 255));

        draw_filled_circle_quadrants(
            main_canvas,
            300,
            200,
            50,
            Color::RGBA(255, 0, 0, 255),
            Some(vec![0]),
        );

        draw_filled_rounded_rect(
            main_canvas,
            Rect::new(10, 50, 200, 100),
            10,
            Color::RGBA(0, 255, 255, 255),
        );
    }
}
