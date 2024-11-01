use std::sync::{Arc, Mutex};

use sdl2::{
    event::Event,
    pixels::Color,
    render::{Canvas, TextureCreator},
    video::{Window, WindowContext},
};

use crate::{
    app::App,
    config::Config,
    execute,
    sources::SourceItem,
    ui::{
        components::{
            clock::Clock,
            list::{SelectList, SelectListState},
            spinner::Spinner,
            text::Prompt,
        },
        layout::{Container, ContainerSize, LayoutBuilder, SplitType},
    },
    utils::cache::TextureCache,
};

use super::Screen;

pub struct MainScreen {
    layout: LayoutBuilder,
    source_items: Arc<Mutex<Vec<SourceItem>>>,
    spinner_idx: usize,
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
        let clock = Clock::new("clock".to_string());

        let mut builder = LayoutBuilder::new().with_gap(2);
        let main_split = builder.add_split(SplitType::Vertical, ContainerSize::Percent(100));
        let _top_split = builder.add_split(SplitType::Horizontal, ContainerSize::Fixed(64));
        builder.add(Box::new(prompt), ContainerSize::Percent(100));
        let spinner_idx = builder.add(Box::new(spinner), ContainerSize::Fixed(64));
        builder.set_cur_split(main_split);
        builder.add(Box::new(select_list), ContainerSize::Percent(100));
        builder.add(Box::new(clock), ContainerSize::Fixed(32));
        builder.generate(width, height);

        MainScreen {
            layout: builder,
            source_items: items,
            spinner_idx,
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
            .set_state(Box::new(app.loading));

        // Hide spinner if not loading
        if !app.loading {
            let container = self.layout.get_container(self.spinner_idx).unwrap();
            match container {
                Container { ref mut size, .. } => *size = ContainerSize::Fixed(0),
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
        let (width, height) = main_canvas.window().size();

        self.layout.generate(width as usize, height as usize);

        // Set draw color and clear
        let clear_color = Color::RGBA(24, 24, 33, 255);
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
