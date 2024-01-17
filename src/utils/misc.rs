use sdl2::{event::Event, keyboard::Mod};

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

