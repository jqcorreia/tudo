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
            text::{TextInput, TextInputState},
            tray::Tray,
            workspaces::Workspaces,
        },
        layout::{Container, ContainerSize, LayoutBuilder, SplitType},
    },
    utils::{cache::TextureCache, misc::localize_mouse_event},
};

use super::Screen;

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
        let prompt = TextInput::new("prompt", config);
        let select_list = SelectList::<SourceItem>::new("list").with_on_select(execute);
        let spinner = Spinner::new("spinner".to_string());
        let clock = Clock::new("clock".to_string());
        let workspaces = Workspaces::new("workspaces".to_string());
        let tray = Tray::new("workspaces");

        let mut builder = LayoutBuilder::new().with_gap(2);
        let main_split = builder.add_split(SplitType::Vertical, ContainerSize::Percent(100));
        let _top_split = builder.add_split(SplitType::Horizontal, ContainerSize::Fixed(64));

        builder.add(Box::new(prompt), ContainerSize::Percent(100));
        builder.add(Box::new(spinner), ContainerSize::Fixed(64));
        builder.set_cur_split(main_split);
        builder.add(Box::new(select_list), ContainerSize::Percent(100));
        builder.add(Box::new(clock), ContainerSize::Fixed(32));
        builder.add(Box::new(workspaces), ContainerSize::Fixed(32));
        builder.add(Box::new(tray), ContainerSize::Fixed(32));
        builder.generate(width, height);

        MainScreen {
            layout: builder,
            source_items: items,
        }
    }
}

impl Screen for MainScreen {
    fn update(&mut self, app: &mut App, events: &Vec<Event>, elapsed: u128) {
        let ps = self
            .layout
            .by_name("prompt")
            .get_state()
            .downcast_ref::<TextInputState>()
            .unwrap()
            .clone();

        self.layout
            .by_name("list")
            .set_state(Box::new(SelectListState {
                items: self.source_items.lock().unwrap().clone(),
                prompt: ps.text.clone(),
            }));

        for event in events.iter() {
            // If it's a mouse event then we need to localize it and send it to the apropriate
            // component
            // FIXME(quadrado): Do this per component?
            match event {
                sdl2::event::Event::MouseMotion { .. }
                | sdl2::event::Event::MouseButtonDown { .. }
                | sdl2::event::Event::MouseButtonUp { .. } => {
                    for (rect, component) in self.layout.components_with_rect() {
                        let (_event, contains) = localize_mouse_event(event, rect);
                        if contains {
                            component.handle_event(&_event, app, elapsed);
                        }
                    }
                }
                _ => {
                    for component in self.layout.components() {
                        component.handle_event(event, app, elapsed);
                    }
                }
            }
        }
        for component in self.layout.components() {
            component.update(app, elapsed);
        }

        self.layout
            .by_name("spinner")
            .set_state(Box::new(app.loading));

        // Hide spinner if not loading
        if !app.loading {
            let container = self
                .layout
                .container_by_name("spinner".to_string())
                .unwrap();
            match container {
                Container { ref mut size, .. } => *size = ContainerSize::Fixed(0),
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
            component.draw(texture_creator, cache, app, main_canvas, rect, elapsed);
        }
    }
    fn reset(&mut self) {
        self.layout.by_name_2::<TextInput>("prompt").clear();

        // Clear both prompt and select list search
        // self.layout
        //     .by_name("prompt".to_string())
        //     .set_state(Box::new(TextInputState {
        //         text: "".to_string(),
        //         cursor_position: 0,
        //     }));
        self.layout
            .by_name("list")
            .set_state(Box::new(SelectListState {
                items: self.source_items.lock().unwrap().clone(),
                prompt: "".to_string(),
            }));
    }
}
