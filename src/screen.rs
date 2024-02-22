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
    components::{
        list::{SelectList, SelectListState},
        spinner::Spinner,
        text::Prompt,
        traits::{EventConsumer, Render},
    },
    config::Config,
    execute,
    layout::{Container, ContainerSize, ContainerType, Layout, Leaf, Split},
    layout2::{LayoutBuilder, SplitType},
    sources::SourceItem,
    utils::cache::TextureCache,
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

        // let mut builder = LayoutBuilder::new().with_gap(2);
        // builder.add_split(SplitType::Vertical, ContainerSize::Percent(100));
        // builder.add(Box::new(prompt), ContainerSize::Fixed(64));
        // builder.add(Box::new(select_list), ContainerSize::Percent(100));
        // let layout = builder.build(width, height);

        let layout = Layout::new(
            2,
            Container {
                container_type: ContainerType::Leaf(Leaf {
                    component: Box::new(Spinner {
                        id: "spin".to_string(),
                        running: true,
                        period_millis: 1000,
                    }),
                }),
                size: ContainerSize::Percent(100),
            },
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

pub struct SubMenu {
    layout: Layout,
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
        // let mut builder = LayoutBuilder::new();

        // // builder.add_split(SplitType::Vertical);
        // builder.add_split(SplitType::Horizontal, ContainerSize::Percent(100));
        // builder.add(Box::new(text1), ContainerSize::Fixed(200));
        // builder.add(Box::new(text2), ContainerSize::Fixed(200));
        // builder.add(Box::new(spinner), ContainerSize::Fixed(200));

        // let layout = builder.build(1000, 500);
        let layout = Layout::new(
            2,
            Container {
                container_type: ContainerType::Leaf(Leaf {
                    component: Box::new(Spinner {
                        id: "spin".to_string(),
                        running: true,
                        period_millis: 1000,
                    }),
                }),
                size: ContainerSize::Percent(100),
            },
            1000,
            500,
        );
        SubMenu { layout }
    }
}

impl Screen for SubMenu {
    fn update(&mut self, app: &mut App, events: &Vec<Event>, _elapsed: u128) {
        for event in events.iter() {
            for event in events.iter() {
                for component in self.layout.components() {
                    component.consume_event(&event, app);
                }
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
        main_canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
        main_canvas.clear();
        for car in self.layout.components_with_rect() {
            car.component.draw(
                &texture_creator,
                cache,
                &app,
                main_canvas,
                car.rect,
                elapsed,
            );
        }
    }
}
