extern crate sdl2;

pub mod components;
pub mod layout;
pub mod sources;
pub mod utils;

use std::collections::HashMap;

use components::list::SelectList;
use components::text;
use components::traits::Component;
use components::traits::EventConsumer;
use components::traits::Render;

use layout::Container;
use layout::ContainerType;
use layout::Layout;
use layout::SizeTypeEnum;
use sdl2::image::InitFlag;
use sources::Source;

use sdl2::pixels::PixelFormatEnum;
use sdl2::{keyboard::Keycode, pixels::Color, rect::Rect};
use sources::apps::DesktopApplications;
use sources::SourceItem;
use utils::cache::TextureCache;

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

    let mut cache = TextureCache::new(&tc);

    // Process sources and generate global items list
    let mut sources: Vec<Box<dyn Source>> = vec![Box::new(DesktopApplications::new())];

    for source in sources.iter_mut() {
        source.calculate_items();
    }

    let mut items: Vec<SourceItem> = Vec::new();
    for source in sources {
        for item in source.items().iter() {
            items.push(item.clone());
        }
    }

    let mut prompt = text::Prompt {
        text: String::from(""),
        foreground_color: Color::RGBA(255, 255, 255, 255),
    };
    let mut select_list = SelectList::<SourceItem>::new();

    let mut comps: HashMap<String, Box<dyn Component>> = HashMap::new();
    comps.insert("prompt".to_string(), Box::new(prompt));
    comps.insert("list".to_string(), Box::new(select_list));

    let layout = Layout {
        gap: 60,
        root: Container {
            key: Some("root".to_string()),
            container_type: ContainerType::VSplit,
            size: 64,
            size_type: SizeTypeEnum::Fixed,
            nodes: Some(Vec::from([
                Container {
                    key: Some("prompt".to_string()),
                    container_type: ContainerType::Leaf,
                    size: 64,
                    size_type: SizeTypeEnum::Fixed,
                    nodes: None,
                },
                Container {
                    key: Some("list".to_string()),
                    container_type: ContainerType::Leaf,
                    size: 64,
                    size_type: SizeTypeEnum::Fixed,
                    nodes: None,
                },
            ])),
        },
    };
    let mut lay = layout.generate(
        canvas.window().size().0 as usize,
        canvas.window().size().1 as usize,
    );

    // let mut cur_prompt = "a".to_string(); //FIXME this is wack, just a value to not be equal to
    //initial prompt
    while running {
        // let prompt_text = &comps.get("prompt").unwrap().text;
        // if prompt_text != &cur_prompt {
        //     select_list.set_list_and_prompt(items.clone(), prompt_text.to_string());
        //     cur_prompt = prompt_text.to_string();
        // }

        // Consume events and pass them to the components
        let cur_events: Vec<_> = event_pump.poll_iter().collect();
        for event in cur_events.iter() {
            match event {
                sdl2::event::Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => running = false,
                sdl2::event::Event::Quit { .. } => running = false,
                sdl2::event::Event::MouseButtonDown { x, y, .. } => {
                    println!("{} {}", x, y)
                }
                _ => (),
            }
            for comp in comps.values_mut() {
                comp.consume_event(&event);
            }
        }

        // Set draw color and clear
        canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
        canvas.clear();

        for (rect, key) in lay.iter_mut() {
            let mut tex = tc
                .create_texture_target(PixelFormatEnum::RGBA8888, rect.width(), rect.height())
                .unwrap();
            let comp = comps.get_mut(key).unwrap();
            canvas
                .with_texture_canvas(&mut tex, |c| {
                    comp.render(&mut cache, &font, c, *rect);
                })
                .unwrap();

            canvas.copy(&tex, None, *rect).unwrap();
        }

        canvas.present();
    }
}
