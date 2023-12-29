extern crate sdl2;

use sdl2::pixels::PixelFormatEnum;

use sdl2::rect::Point;
use sdl2::{
    keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, ttf::Font, video::Window,
};
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let ttf = sdl2::ttf::init().unwrap();
    let mut window = video
        .window("boom", 1024, 768)
        .position_centered()
        .build()
        .unwrap();
    window.show();

    let mut font_size = 14;
    let mut font = ttf
        .load_font("/usr/share/fonts/droid/DroidSansMono.ttf", font_size)
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl.event_pump().unwrap();
    let mut running = true;
    let mut draw_debug_info = true;

    let tc = canvas.texture_creator();
    let mut cur_time = Instant::now();
    let mut prompt = String::from(">");

    while running {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::TextInput { text, .. } => {
                    prompt += &text;
                    // self.insert_char(dbg!(text.to_owned()))
                }
                sdl2::event::Event::KeyDown {
                    keycode: Some(Keycode::Backspace),
                    ..
                } => prompt = prompt.get(..prompt.len() - 1).unwrap().into(),
                sdl2::event::Event::KeyDown {
                    keycode: Some(Keycode::F1),
                    ..
                } => draw_debug_info = !draw_debug_info,
                sdl2::event::Event::KeyDown {
                    keycode: Some(Keycode::F2),
                    ..
                } => {
                    font_size += 1;
                    font = ttf
                        .load_font("/usr/share/fonts/droid/DroidSansMono.ttf", font_size)
                        .unwrap()
                }
                sdl2::event::Event::KeyDown {
                    keycode: Some(Keycode::F3),
                    ..
                } => {
                    font_size -= 1;
                    font = ttf
                        .load_font("/usr/share/fonts/droid/DroidSansMono.ttf", font_size)
                        .unwrap()
                }
                sdl2::event::Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    keymod: sdl2::keyboard::Mod::LCTRLMOD,
                    ..
                } => running = false,
                sdl2::event::Event::Quit { .. } => running = false,
                sdl2::event::Event::MouseButtonDown { x, y, .. } => {
                    println!("{} {}", x, y)
                }
                _ => (),
            }
        }
        // let fps = (1_000_000_000 / (&cur_time.elapsed().as_nanos())) as u32;
        cur_time = Instant::now();

        canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
        canvas.clear();
        canvas.set_draw_color(Color::RGBA(255, 0, 0, 0));
        let texture = tc
            .create_texture_from_surface(
                font.render(&prompt)
                    .blended(Color::RGBA(255, 255, 255, 255))
                    .unwrap(),
            )
            .unwrap();

        let query = texture.query();
        let (w, h) = (query.width, query.height);
        canvas.copy(&texture, None, Some(Rect::new(10, 10, w, h)));

        // for (rect, key) in lay.iter_mut() {
        //     let comp = components.get_mut(key).unwrap();
        //     let mut tex = tc2
        //         .create_texture_target(PixelFormatEnum::RGBA8888, rect.width(), rect.height())
        //         .unwrap();

        //     canvas
        //         .with_texture_canvas(&mut tex, |c| {
        //             comp.render(&mut atlas2, &font, c, *rect);
        //             let border_color = if comp.is_focused() {
        //                 Color::RGBA(0, 255, 0, 255)
        //             } else {
        //                 Color::RGBA(100, 100, 100, 255)
        //             };
        //             c.set_draw_color(border_color);
        //             c.draw_rect(Rect::new(0, 0, rect.width(), rect.height()))
        //                 .unwrap();
        //         })
        //         .unwrap();

        //     canvas.copy(&tex, None, *rect).unwrap();
        // }

        // // Draw the FPS counter directly into the window canvas
        // if draw_debug_info {
        //     draw_fps(&mut canvas, &font, fps);
        // }
        canvas.present();
    }
}
