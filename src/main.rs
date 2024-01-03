extern crate sdl2;

pub mod components;
pub mod sources;
pub mod utils;

use components::list::SelectList;
use components::text;
use components::traits::EventConsumer;
use components::traits::Render;

use sdl2::image::InitFlag;
use sources::Source;

use sdl2::pixels::PixelFormatEnum;
use sdl2::{keyboard::Keycode, pixels::Color, rect::Rect};
use sources::apps::DesktopApplications;
use sources::SourceItem;
use utils::atlas::FontAtlas;
use utils::icons::IconCache;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let ttf = sdl2::ttf::init().unwrap();
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG);
    let window = video
        .window("tudo", 1024, 768)
        .opengl()
        .position_centered()
        .build()
        .unwrap();

    let font_size = 20;
    let font_path = "/usr/share/fonts/noto/NotoSans-Regular.ttf";
    // let font_path = "/usr/share/fonts/windows/Inkfree.ttf";
    let font = ttf.load_font(font_path, font_size).unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl.event_pump().unwrap();
    let mut running = true;

    let tc = canvas.texture_creator();
    let mut prompt = text::Prompt {
        text: String::from(""),
        foreground_color: Color::RGBA(255, 255, 255, 255),
    };
    let mut select_list = SelectList::<SourceItem>::new();
    let mut atlas = FontAtlas::new(&tc);
    let mut icons = IconCache::new(&tc);

    // Process sources and generate global items list
    let mut sources: Vec<Box<dyn Source>> = vec![Box::new(DesktopApplications::new())];

    for source in sources.iter_mut() {
        source.calculate_items();
    }

    // Check how you can do this
    // for source in sources {
    //     for item in source.items().iter() {
    //         items.push(item);
    //     }
    // }

    while running {
        let mut items: Vec<SourceItem> = Vec::new();
        for item in sources[0].items().iter() {
            items.push(item.clone());
        }
        let prompt_text = &prompt.text;
        select_list.set_list_and_prompt(items.clone(), prompt_text.to_string());

        // Consume events and pass them to the components
        let cur_events: Vec<_> = event_pump.poll_iter().collect();
        for event in cur_events.iter() {
            match event {
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
            prompt.consume_event(&event);
            select_list.consume_event(&event);
        }

        // Set draw color and clear
        canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
        canvas.clear();

        // Draw components
        let rect = Rect::new(10, 10, 400, 40);
        let mut tex = tc
            .create_texture_target(PixelFormatEnum::RGBA8888, rect.width(), rect.height())
            .unwrap();

        canvas
            .with_texture_canvas(&mut tex, |c| {
                prompt.render(&mut atlas, &font, c, rect);
            })
            .unwrap();

        canvas.copy(&tex, None, rect).unwrap();

        let rect = Rect::new(10, 300, 400, 300);
        let mut tex = tc
            .create_texture_target(PixelFormatEnum::RGBA8888, rect.width(), rect.height())
            .unwrap();

        canvas
            .with_texture_canvas(&mut tex, |c| {
                select_list.render(&mut atlas, &font, c, rect);
            })
            .unwrap();
        canvas.copy(&tex, None, rect).unwrap();

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

        canvas.present();
    }
}
