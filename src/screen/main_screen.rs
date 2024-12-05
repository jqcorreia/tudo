use std::sync::{Arc, Mutex};

use sdl2::{
    event::Event,
    keyboard::Keycode,
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
            label::Label,
            list::{SelectList, SelectListState},
            spinner::Spinner,
            text::TextInput,
            tray::Tray,
            workspaces::Workspaces,
        },
        layout::{ContainerSize, LayoutBuilder, SplitType},
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
        let tray = Tray::new("tray");
        let result = Label::new("result", "ss");

        let mut builder = LayoutBuilder::new().with_gap(2);
        let main_split = builder.add_split(SplitType::Vertical, ContainerSize::Percent(100));
        let _top_split = builder.add_split(SplitType::Horizontal, ContainerSize::Fixed(64));

        builder.add(Box::new(prompt), ContainerSize::Percent(100));
        builder.add(Box::new(spinner), ContainerSize::Fixed(64));

        builder.set_cur_split(main_split);
        builder.add(Box::new(select_list), ContainerSize::Percent(100));
        builder.add(Box::new(result), ContainerSize::Percent(100));
        builder.add(Box::new(clock), ContainerSize::Fixed(32));
        builder.add(Box::new(workspaces), ContainerSize::Fixed(32));
        builder.add(Box::new(tray), ContainerSize::Fixed(32));
        builder.by_name_container("result").hidden = true;

        builder.generate(width, height);

        MainScreen {
            layout: builder,
            source_items: items,
        }
    }
}

impl Screen for MainScreen {
    fn update(&mut self, app: &mut App, events: &Vec<Event>, elapsed: u128) {
        let prompt_text = self
            .layout
            .by_name_typed::<TextInput>("prompt")
            .state
            .text
            .clone();

        self.layout
            .by_name("list".to_string())
            .set_state(Box::new(SelectListState {
                items: self.source_items.lock().unwrap().clone(),
                prompt: prompt_text,
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
                sdl2::event::Event::KeyUp {
                    keycode: Some(Keycode::F5),
                    ..
                } => {
                    self.layout.by_name_container("tray").hidden ^= true; // Toggle
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
            .by_name("spinner".to_string())
            .set_state(Box::new(app.loading));

        // Hide spinner if not loading
        if !app.loading {
            let container = self
                .layout
                .container_by_name("spinner".to_string())
                .unwrap();
            container.hidden = true;
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
        self.layout.by_name_typed::<TextInput>("prompt").clear();
        self.layout
            .by_name("list".to_string())
            .set_state(Box::new(SelectListState {
                items: self.source_items.lock().unwrap().clone(),
                prompt: "".to_string(),
            }));
    }
}
