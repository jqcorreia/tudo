use sdl2::{
    event::Event,
    keyboard::Mod,
    rect::{Point, Rect},
};

pub fn ignore_numlock(event: &Event) -> Event {
    let _event = match event {
        sdl2::event::Event::KeyDown {
            timestamp,
            window_id,
            keycode,
            scancode,
            keymod,
            repeat,
            ..
        } => {
            let km = *keymod - Mod::NUMMOD;
            sdl2::event::Event::KeyDown {
                timestamp: *timestamp,
                window_id: *window_id,
                keycode: *keycode,
                scancode: *scancode,
                keymod: km,
                repeat: *repeat,
            }
        }

        _ => event.clone(),
    };
    _event
}

pub fn localize_mouse_event(event: &Event, rect: Rect) -> (Event, bool) {
    let (_event, contains) = match event {
        sdl2::event::Event::MouseMotion {
            timestamp,
            window_id,
            which,
            mousestate,
            x,
            y,
            xrel,
            yrel,
        } => (
            sdl2::event::Event::MouseMotion {
                timestamp: *timestamp,
                window_id: *window_id,
                which: *which,
                mousestate: *mousestate,
                x: *x - rect.x,
                y: *y - rect.y,
                xrel: *xrel,
                yrel: *yrel,
            },
            rect.contains_point(Point::new(*x, *y)),
        ),
        sdl2::event::Event::MouseButtonDown {
            timestamp,
            window_id,
            which,
            x,
            y,
            mouse_btn,
            clicks,
        } => (
            sdl2::event::Event::MouseButtonDown {
                timestamp: *timestamp,
                window_id: *window_id,
                which: *which,
                x: *x - rect.x,
                y: *y - rect.y,
                mouse_btn: *mouse_btn,
                clicks: *clicks,
            },
            rect.contains_point(Point::new(*x, *y)),
        ),
        sdl2::event::Event::MouseButtonUp {
            timestamp,
            window_id,
            which,
            x,
            y,
            mouse_btn,
            clicks,
        } => (
            sdl2::event::Event::MouseButtonUp {
                timestamp: *timestamp,
                window_id: *window_id,
                which: *which,
                x: *x - rect.x,
                y: *y - rect.y,
                mouse_btn: *mouse_btn,
                clicks: *clicks,
            },
            rect.contains_point(Point::new(*x, *y)),
        ),
        _ => (event.clone(), false),
    };

    (_event, contains)
}
