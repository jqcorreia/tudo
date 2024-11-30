use std::time::Duration;

use dbus::blocking::{stdintf::org_freedesktop_dbus::Properties, Connection};
use sdl2::rect::Rect;

use crate::utils::xdg::IconFinder;

use super::traits::UIComponent;

pub struct Tray {
    id: String,
    conn: Connection,
    icon_paths: Vec<String>,
}

impl Tray {
    pub fn new(id: impl AsRef<str>) -> Tray {
        let conn = Connection::new_session().unwrap();
        let proxy = conn.with_proxy(
            "org.kde.StatusNotifierWatcher",
            "/StatusNotifierWatcher",
            Duration::from_millis(2000),
        );

        let sni: Vec<String> = proxy
            .get(
                "org.kde.StatusNotifierWatcher",
                "RegisteredStatusNotifierItems",
            )
            .unwrap();

        let icon_finder = IconFinder::new();

        let mut icon_paths = vec![];
        for item in sni {
            let mut split = item.splitn(2, "/");
            let svc = split.next().unwrap();
            let object = format!("/{}", split.next().unwrap());
            println!("svc : {}, object: {}", svc, object);
            let proxy = conn.with_proxy(svc, object, Duration::from_millis(2000));
            let title: String = proxy.get("org.kde.StatusNotifierItem", "Title").unwrap();
            if title.is_empty() {
                continue;
            }
            let icon: String = proxy.get("org.kde.StatusNotifierItem", "IconName").unwrap();

            // dbg!(&title);
            // dbg!(&icon);
            // dbg!(icon_finder.get_icon_with_size(icon.clone(), 24));

            if let Some(path) = icon_finder.get_icon_with_size(icon, 24) {
                icon_paths.push(path)
            }
            //if title == "blueman" {
            //    proxy
            //        .method_call("org.kde.StatusNotifierItem", "Activate", (1000, 1000))
            //        .unwrap()
            //}
        }

        Tray {
            id: id.as_ref().to_string(),
            conn,
            icon_paths,
        }
    }
}

impl UIComponent for Tray {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn render(
        &mut self,
        _texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        cache: &mut crate::utils::cache::TextureCache,
        _app: &crate::app::App,

        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        _rect: sdl2::rect::Rect,
        _elapsed: u128,
    ) {
        let mut x: i32 = 0;
        for p in self.icon_paths.clone() {
            let tex = cache.images.get_image(p);
            let _w = tex.query().width;
            let _h = tex.query().width;

            canvas
                .copy(tex, None, Some(Rect::new(x, 0, 24, 24)))
                .unwrap();
            x += 24_i32 + 5;
        }
    }
    fn update(&mut self, event: &sdl2::event::Event, app: &mut crate::app::App, elapsed: u128) {
        if let sdl2::event::Event::MouseButtonUp { .. } = event {}
    }
    fn get_state(&self) -> &dyn std::any::Any {
        &false
    }

    fn set_state(&mut self, state: Box<dyn std::any::Any>) {}
}
