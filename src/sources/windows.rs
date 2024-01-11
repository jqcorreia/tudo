use crate::sources::{Action, Source, WindowSwitchAction};
use xcb::x::{self, Atom, ConfigWindow, PropMode, SendEventDest, Window};
use xcb::Connection;

use super::SourceItem;

pub struct WindowSource {
    items: Vec<SourceItem>,
}

fn get_atom(conn: &Connection, name: &str) -> Atom {
    let cookie = conn.send_request(&x::InternAtom {
        only_if_exists: false,
        name: name.as_bytes(),
    });
    let reply = conn.wait_for_reply(cookie).unwrap();
    reply.atom()
}

pub fn get_window_image(conn: &Connection, window: &Window) -> Result<Vec<u8>, xcb::Error> {
    let cookie = conn.send_request(&x::GetGeometry {
        drawable: x::Drawable::Window(*window),
    });
    let geom = conn.wait_for_reply(cookie)?;

    let width = geom.width();
    let height = geom.height();

    let cookie = conn.send_request(&x::GetImage {
        format: x::ImageFormat::ZPixmap,
        drawable: x::Drawable::Window(*window),
        x: 0,
        y: 0,
        width,
        height,
        plane_mask: u32::MAX,
    });

    let reply = conn.wait_for_reply(cookie)?;
    let src = reply.data();
    let mut data = vec![0; width as usize * height as usize * 3];
    for (src, dest) in src.chunks(4).zip(data.chunks_mut(3)) {
        // Captured image stores orders pixels as BGR, so we need to
        // reorder them.
        dest[0] = src[2];
        dest[1] = src[1];
        dest[2] = src[0];
    }
    Ok(data)
}

pub fn switch_to_window(
    conn: &Connection,
    window: &Window,
    root: &Window,
) -> Result<(), xcb::Error> {
    println!("SWITCH");
    // let wm_protocols_atom = get_atom(&conn, "WM_PROTOCOLS");
    let net_active_window_atom = get_atom(&conn, "_NET_ACTIVE_WINDOW");
    let net_wm_desktop_atom = get_atom(&conn, "_NET_WM_DESKTOP");
    let net_current_desktop_atom = get_atom(&conn, "_NET_CURRENT_DESKTOP");

    //  Get window current desktop
    let x = conn.send_request(&x::GetProperty {
        window: *window,
        delete: false,
        long_offset: 0,
        long_length: 100,
        property: net_wm_desktop_atom,
        r#type: x::ATOM_CARDINAL,
    });

    let reply = conn.wait_for_reply(x)?;

    let window_desktop: u32 = reply.value::<u32>()[0];
    println!("Window is in desktop {:?}", window_desktop);
    // let x = conn.send_request(&x::ChangeProperty {
    //     window: *window,
    //     property: net_current_desktop_atom,
    //     r#type: x::ATOM_CARDINAL,
    //     mode: PropMode::Replace,
    //     data: &[window_desktop],
    // });

    let x = conn.send_request_checked(&x::SendEvent {
        destination: SendEventDest::Window(*window),
        event: &x::ClientMessageEvent::new(
            *window,
            net_current_desktop_atom,
            x::ClientMessageData::Data32([window_desktop, 0, 0, 0, 0]),
        ),
        propagate: false,
        event_mask: x::EventMask::STRUCTURE_NOTIFY,
    });
    dbg!(conn.check_request(x)?);

    // Map Window
    let x = conn.send_request_checked(&x::MapWindow { window: *window });
    dbg!(conn.check_request(x)?);

    // Configure Window
    let x = conn.send_request_checked(&x::ConfigureWindow {
        window: *window,
        value_list: &[ConfigWindow::StackMode(x::StackMode::Above)],
    });

    // Send Event _NET_ACTIVE_WINDOW
    dbg!(conn.check_request(x)?);
    let x = conn.send_request_checked(&x::SendEvent {
        destination: SendEventDest::Window(*window),
        event: &x::ClientMessageEvent::new(
            *window,
            // wm_protocols_atom,
            net_active_window_atom,
            x::ClientMessageData::Data32([1, 1, 0, 0, 0]),
        ),
        propagate: false,
        event_mask: x::EventMask::STRUCTURE_NOTIFY,
    });
    dbg!(conn.check_request(x)?);

    // // set input focus
    // let x = conn.send_request_checked(&x::SetInputFocus {
    //     focus: *window,
    //     time: 0,
    //     revert_to: x::InputFocus::None,
    // });
    // dbg!(conn.check_request(x));
    Ok(())
}

impl WindowSource {
    pub fn new() -> Self {
        WindowSource { items: Vec::new() }
    }
}

impl Source for WindowSource {
    fn items(&self) -> &Vec<SourceItem> {
        &self.items
    }
    fn calculate_items(&mut self) {
        let mut res: Vec<SourceItem> = Vec::new();

        // Connect to the X server.
        let (conn, screen_num) = xcb::Connection::connect(None).unwrap();
        let setup = conn.get_setup();
        let screen = setup.roots().nth(screen_num as usize).unwrap();

        let net_client_list_atom = get_atom(&conn, "_NET_CLIENT_LIST");

        let c = conn.send_request(&xcb::x::GetProperty {
            delete: false,
            window: screen.root(),
            long_offset: 0,
            long_length: 99,
            property: net_client_list_atom,
            r#type: x::ATOM_WINDOW,
        });

        let r = conn.wait_for_reply(c).unwrap();
        dbg!(&r);

        for w in r.value() {
            let c = conn.send_request(&xcb::x::GetProperty {
                delete: false,
                window: *w,
                long_offset: 0,
                long_length: 100,
                property: x::ATOM_WM_CLASS,
                r#type: x::ATOM_STRING,
            });

            let r = conn.wait_for_reply(c).unwrap();
            let buf: Vec<u8> = r.value().to_vec();
            let mut split = buf.split(|item| item == &(0 as u8));
            let wname = String::from_utf8(split.nth(1).unwrap().to_vec()).unwrap();
            res.push(SourceItem {
                action: Action::WindowSwitch(WindowSwitchAction {
                    window: *w,
                    exit_after: true,
                }),
                icon: None,
                title: format!("W {}", wname),
            });
        }
        self.items = res;
    }
}
