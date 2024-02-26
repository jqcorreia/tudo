use std::sync::{Arc, Mutex};

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
    execute,
    sources::SourceItem,
    ui::components::{
        list::{SelectList, SelectListState},
        spinner::Spinner,
        text::Prompt,
    },
    ui::layout::{ContainerSize, LayoutBuilder, SplitType},
    utils::{
        cache::TextureCache,
        draw::{
            draw_circle, draw_circle_quadrants, draw_filled_circle_quadrants, draw_rounded_rect,
            DrawExtensions,
        },
    },
};

pub trait Screen {
    fn init(&mut self) {}

    fn update(&mut self, app: &mut App, events: &Vec<Event>, _elapsed: u128);
    fn render(
        &mut self,
        texture_creator: &TextureCreator<WindowContext>,
        cache: &mut TextureCache,
        app: &App,
        main_canvas: &mut Canvas<Window>,
        elapsed: u128,
    );
}

pub struct MainScreen {
    layout: LayoutBuilder,
    source_items: Arc<Mutex<Vec<SourceItem>>>,
}

impl MainScreen {
    pub fn new(
        config: &Config,
        width: usize,
        height: usize,
        items: Arc<Mutex<Vec<SourceItem>>>,
    ) -> MainScreen {
        let prompt = Prompt::new("prompt", config);
        let mut select_list = SelectList::<SourceItem>::new("list");
        select_list.on_select = execute;
        let spinner = Spinner::new("spinner".to_string());

        let mut builder = LayoutBuilder::new().with_gap(2);
        let main_split = builder.add_split(SplitType::Vertical, ContainerSize::Percent(100));
        let _top_split = builder.add_split(SplitType::Horizontal, ContainerSize::Fixed(64));
        builder.add(Box::new(prompt), ContainerSize::Percent(100));
        builder.add(Box::new(spinner), ContainerSize::Fixed(64));
        builder.set_cur_split(main_split);
        builder.add(Box::new(select_list), ContainerSize::Percent(100));
        builder.generate(width, height);

        MainScreen {
            layout: builder,
            source_items: items,
        }
    }
}
impl Screen for MainScreen {
    fn update(&mut self, app: &mut App, events: &Vec<Event>, _elapsed: u128) {
        let ps: String = self
            .layout
            .by_name("prompt".to_string())
            .get_state()
            .downcast_ref::<String>()
            .unwrap()
            .clone();

        self.layout
            .by_name("list".to_string())
            .set_state(Box::new(SelectListState {
                items: self.source_items.lock().unwrap().clone(),
                prompt: ps,
            }));
        for event in events.iter() {
            for component in self.layout.components() {
                component.consume_event(&event, app);
            }
        }

        self.layout
            .by_name("spinner".to_string())
            .set_state(Box::new(app.loading))
    }

    fn render(
        &mut self,
        texture_creator: &TextureCreator<WindowContext>,
        mut cache: &mut TextureCache,
        app: &App,
        main_canvas: &mut Canvas<Window>,
        elapsed: u128,
    ) {
        let clear_color = Color::RGBA(0, 0, 0, 255);
        // Set draw color and clear
        main_canvas.set_draw_color(clear_color);
        main_canvas.clear();

        for (rect, component) in self.layout.components_with_rect() {
            component.draw(
                &texture_creator,
                &mut cache,
                &app,
                main_canvas,
                rect,
                elapsed,
            );
        }
    }
}

pub struct SubMenu {
    layout: LayoutBuilder,
}

impl SubMenu {
    pub fn new(config: &Config) -> SubMenu {
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
        SubMenu { layout: builder }
    }
}

impl Screen for SubMenu {
    fn update(&mut self, app: &mut App, events: &Vec<Event>, _elapsed: u128) {
        for event in events.iter() {
            for component in self.layout.components() {
                component.consume_event(&event, app);
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
        main_canvas.set_draw_color(Color::RGBA(30, 30, 30, 255));
        main_canvas.clear();
        // for (rect, component) in self.layout.components_with_rect() {
        //     component.draw(&texture_creator, cache, &app, main_canvas, rect, elapsed);
        // }

        main_canvas.set_draw_color(Color::RGBA(0, 255, 0, 255));
        main_canvas.draw_rounded_rect(Rect::new(10, 100, 200, 200), 20);

        draw_filled_circle_quadrants(main_canvas, 300, 200, 50, Color::RGBA(255, 0, 0, 255), None);
        // let c = Color::RGBA(255, 255, 255, 255);
        // // draw_circle_quadrants(main_canvas, 200, 20, 20, c, Some(vec![0]));
        // draw_circle_quadrants(main_canvas, 300, 20, 20, c, Some(vec![1]));
        // draw_circle_quadrants(main_canvas, 400, 20, 20, c, Some(vec![2]));
        // draw_circle_quadrants(main_canvas, 500, 30, 20, c, None);
        // draw_circle_quadrants(main_canvas, 40, 70, 20, c, None);
    }
}
