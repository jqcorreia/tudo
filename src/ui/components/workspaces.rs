use sdl2::{pixels::Color, rect::Rect};

use crate::{
    app::App,
    ui::{
        components::button::ButtonState,
        layout::{ContainerSize, LayoutBuilder, SplitType},
    },
    utils::misc::localize_mouse_event,
};

use super::{button::Button, traits::UIComponent};

pub struct Workspaces {
    pub id: String,
    pub selected_workspace: u8,
    pub font_name: Option<String>,
    builder: LayoutBuilder,
    initialized: bool,
}

impl Workspaces {
    pub fn new(id: String) -> Workspaces {
        let builder = LayoutBuilder::new().with_gap(3);

        Workspaces {
            id,
            selected_workspace: 1,
            font_name: None,
            builder,
            initialized: false,
        }
    }
}

impl UIComponent for Workspaces {
    fn id(&self) -> String {
        self.id.clone()
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
            component.draw(texture_creator, cache, app, canvas, rect, elapsed);
        }
    }

    fn handle_event(&mut self, event: &sdl2::event::Event, app: &mut App, elapsed: u128) {
        match event {
            sdl2::event::Event::MouseMotion { .. }
            | sdl2::event::Event::MouseButtonDown { .. }
            | sdl2::event::Event::MouseButtonUp { .. } => {
                for (rect, component) in self.builder.components_with_rect() {
                    let (event, contains) = localize_mouse_event(event, rect);
                    if contains {
                        component.set_focus(true);
                        component.handle_event(&event, app, elapsed);
                    } else {
                        component.set_focus(false);
                    }
                }
            }
            _ => {
                for component in self.builder.components() {
                    component.handle_event(event, app, elapsed);
                }
            }
        }
    }
    fn update(&mut self, app: &mut App, elapsed: u128) {
        if !self.initialized {
            // dbg!(app.hyprland.as_mut().unwrap().get_active_workspace());
            // dbg!(Hyprland::new().unwrap().get_active_workspace());
            let workspaces = app
                .hyprland
                .as_mut()
                .unwrap()
                .get_workspaces()
                .iter()
                .map(|x| x.id)
                .collect::<Vec<u8>>();
            self.builder
                .add_split(SplitType::Horizontal, ContainerSize::Percent(100));

            for x in 1..10 {
                let mut btn =
                    Button::new(x.to_string(), x.to_string()).with_on_click(|btn, app| {
                        app.hyprland
                            .as_mut()
                            .unwrap()
                            .goto_workspace(btn.id().parse::<u8>().unwrap());
                        app.should_hide = true;
                    });
                if !workspaces.contains(&x) {
                    btn.state.active = false
                }
                self.builder.add(Box::new(btn), ContainerSize::Fixed(40));
            }
            self.initialized = true
        }
        // Check for hyprland messages
        if let Ok(msg) = app.hyprland.as_mut().unwrap().rx().try_recv() {
            if msg.starts_with("createworkspace>") {
                let wid = msg.split(">>").nth(1).unwrap();
                let btn = self.builder.by_name(wid.to_string());

                let mut btn_state = btn
                    .get_state()
                    .downcast_ref::<ButtonState>()
                    .unwrap()
                    .clone();
                btn_state.active = true;
                btn.set_state(Box::new(btn_state));
            }
            if msg.starts_with("destroyworkspace>") {
                let wid = msg.split(">>").nth(1).unwrap();
                let btn = self.builder.by_name(wid.to_string());

                let mut btn_state = btn
                    .get_state()
                    .downcast_ref::<ButtonState>()
                    .unwrap()
                    .clone();
                btn_state.active = false;
                btn.set_state(Box::new(btn_state));
            }
            dbg!(msg);
        }
    }

    fn get_state(&self) -> &dyn std::any::Any {
        &self.selected_workspace
    }

    fn set_state(&mut self, state: Box<dyn std::any::Any>) {
        self.selected_workspace = *state.downcast_ref::<u8>().unwrap();
    }
}
