use std::sync::{Arc, Mutex};

use sdl2::{
    event::Event,
    pixels::Color,
    render::{Canvas, TextureCreator},
    video::{Window, WindowContext},
};

use crate::{
    app::App,
    components::{
        list::{SelectList, SelectListState},
        text::Prompt,
    },
    config::Config,
    execute,
    layout::{Container, Layout, Leaf, SizeTypeEnum, Split},
    sources::SourceItem,
    utils::{cache::TextureCache, draw::draw_string},
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
    layout: Layout,
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
        let layout = Layout::new(
            2,
            Container::VSplit(Split {
                children: Vec::from([
                    Container::Leaf(Leaf {
                        size_type: SizeTypeEnum::Fixed,
                        size: 64,
                        component: Box::new(prompt),
                    }),
                    Container::Leaf(Leaf {
                        size_type: SizeTypeEnum::Percent,
                        size: 100,
                        component: Box::new(select_list),
                    }),
                ]),
            }),
            width,
            height,
        );

        MainScreen {
            layout,
            source_items: items,
        }
    }
}
impl Screen for MainScreen {
    fn update(&mut self, app: &mut App, events: &Vec<Event>, _elapsed: u128) {
        let ps: String = self
            .layout
            .by_name("prompt".to_string())
            .component
            .get_state()
            .downcast_ref::<String>()
            .unwrap()
            .clone();

        self.layout
            .by_name("list".to_string())
            .component
            .set_state(Box::new(SelectListState {
                items: self.source_items.lock().unwrap().clone(),
                prompt: ps,
            }));
        for event in events.iter() {
            for component in self.layout.components() {
                component.consume_event(&event, app);
            }
        }
    }

    fn render(
        &mut self,
        texture_creator: &TextureCreator<WindowContext>,
        mut cache: &mut TextureCache,
        app: &App,
        main_canvas: &mut Canvas<Window>,
        elapsed: u128,
    ) {
        let clear_color = if app.loading {
            Color::RGBA(200, 0, 0, 255)
        } else {
            Color::RGBA(50, 50, 50, 255)
        };
        // Set draw color and clear
        main_canvas.set_draw_color(clear_color);
        main_canvas.clear();

        for car in self.layout.components_with_rect() {
            car.component.draw(
                &texture_creator,
                &mut cache,
                &app,
                main_canvas,
                car.rect,
                elapsed,
            );
        }
    }
}

pub struct SubMenu {}

impl SubMenu {
    pub fn new() -> SubMenu {
        SubMenu {}
    }
}

impl Screen for SubMenu {
    fn update(&mut self, app: &mut App, events: &Vec<Event>, _elapsed: u128) {}

    fn render(
        &mut self,
        texture_creator: &TextureCreator<WindowContext>,
        mut cache: &mut TextureCache,
        app: &App,
        main_canvas: &mut Canvas<Window>,
        elapsed: u128,
    ) {
        let clear_color = Color::RGBA(0, 0, 0, 255);
        let font = app.get_font("normal-20");
        // Set draw color and clear
        main_canvas.set_draw_color(clear_color);
        main_canvas.clear();

        draw_string(
            "Sub Menu!!!".to_string(),
            main_canvas,
            font,
            Color::RGBA(128, 128, 128, 255),
            10,
            10,
        )
    }
}
