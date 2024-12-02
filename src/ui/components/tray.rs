use std::{
    sync::mpsc::{channel, Receiver, Sender},
    time::Duration,
};

use dbus::{
    arg::ReadAll,
    blocking::{stdintf::org_freedesktop_dbus::Properties, Connection},
    message::SignalArgs,
    Message,
};
use sdl2::rect::Rect;

use crate::{app::App, utils::xdg::IconFinder};

use super::traits::UIComponent;

pub struct Tray {
    id: String,
    conn: Connection,
    icon_paths: Option<Vec<String>>,
    signals_tx: Sender<String>,
    signals_rx: Receiver<String>,
}

struct Signal {}
impl SignalArgs for Signal {
    const NAME: &'static str = "NewToolTip";

    const INTERFACE: &'static str = "org.kde.StatusNotifierItem";
}

impl ReadAll for Signal {
    fn read(_: &mut dbus::arg::Iter) -> Result<Self, dbus::arg::TypeMismatchError> {
        Ok(Self {})
    }
}

impl Tray {
    pub fn new(id: impl AsRef<str>) -> Tray {
        let conn = Connection::new_session().unwrap();

        let (tx, rx) = channel();
        Tray {
            id: id.as_ref().to_string(),
            conn,
            icon_paths: None,
            signals_tx: tx,
            signals_rx: rx,
        }
    }

    pub fn refresh_icons(&mut self) {
        let proxy = self.conn.with_proxy(
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
            let proxy = self
                .conn
                .with_proxy(svc, object, Duration::from_millis(2000));
            let title: String = proxy.get("org.kde.StatusNotifierItem", "Title").unwrap();
            if title.is_empty() {
                continue;
            }
            let icon: String = proxy.get("org.kde.StatusNotifierItem", "IconName").unwrap();
            let tx = self.signals_tx.clone();
            proxy
                .match_signal(move |_: Signal, _: &Connection, m: &Message| {
                    tx.send(String::from("new icon")).unwrap();
                    dbg!(m);
                    true
                })
                .unwrap();

            if let Some(path) = icon_finder.get_icon_with_size(icon, 24) {
                icon_paths.push(path)
            }
        }
        self.icon_paths = if icon_paths.is_empty() {
            None
        } else {
            Some(icon_paths)
        };
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
        if self.icon_paths.is_some() {
            for p in self.icon_paths.as_ref().unwrap().clone() {
                let tex = cache.images.get_image(p);
                let _w = tex.query().width;
                let _h = tex.query().width;

                canvas
                    .copy(tex, None, Some(Rect::new(x, 0, 24, 24)))
                    .unwrap();
                x += 24_i32 + 5;
            }
        }
    }

    fn handle_event(&mut self, event: &sdl2::event::Event, _app: &mut App, _elapsed: u128) {
        if let sdl2::event::Event::MouseButtonUp { .. } = event {}
    }
    fn update(&mut self, _app: &mut App, _elapsed: u128) {
        if self.icon_paths.is_none() {
            self.refresh_icons();
        }
        if self.signals_rx.try_recv().is_ok() {
            self.refresh_icons()
        }
        // This should
        self.conn.process(Duration::new(0, 500_000)).unwrap();
    }
    fn get_state(&self) -> &dyn std::any::Any {
        todo!();
    }

    fn set_state(&mut self, _state: Box<dyn std::any::Any>) {}
}

//#[allow(unused)]
//#[cfg(test)]
//mod tests {
//    use dbus::{
//        arg::ReadAll,
//        message::{MatchRule, SignalArgs},
//    };
//
//    use super::*;
//
//    struct Signal {}
//    impl SignalArgs for Signal {
//        const NAME: &'static str = "NewToolTip";
//
//        const INTERFACE: &'static str = "org.kde.StatusNotifierItem";
//    }
//
//    impl ReadAll for Signal {
//        fn read(i: &mut dbus::arg::Iter) -> Result<Self, dbus::arg::TypeMismatchError> {
//            Ok(Self {})
//        }
//    }
//
//    #[test]
//    fn test_tray() {
//        let conn = Connection::new_session().unwrap();
//        let proxy = conn.with_proxy(":1.804", "/org/blueman/sni", Duration::from_millis(2000));
//        let mr = MatchRule::default()
//            .with_interface("org.kde.StatusNotifierItem")
//            .with_member("NewToolTip");
//        proxy.match_signal(|s: Signal, _: &Connection, m: &dbus::Message| {
//            dbg!(m);
//            true
//        });
//        loop {
//            conn.process(Duration::from_millis(1000));
//        }
//    }
//}
