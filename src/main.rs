extern crate sdl2;

pub mod components;
pub mod layout;
pub mod layout2;
pub mod sources;
pub mod utils;

use components::enums::Component;
use components::list::SelectList;
use components::text;

use components::text::Prompt;
use components::traits::EventConsumer;
use layout2::LayoutItem;
use layout2::Leaf;
use layout2::SizeTypeEnum;
use layout2::Split;
use sdl2::image::InitFlag;
use sdl2::pixels::PixelFormatEnum;
use sources::Source;

use sdl2::{keyboard::Keycode, pixels::Color};
use sources::apps::DesktopApplications;
use sources::SourceItem;
use utils::cache::TextureCache;

use crate::layout2::Container;
use crate::layout2::Layout2;

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

    let prompt = text::Prompt {
        text: String::from(""),
        foreground_color: Color::RGBA(255, 255, 255, 255),
    };

    let select_list = SelectList::<SourceItem>::new();

    let mut layout2 = Layout2 {
        gap: 10,
        root: Container::VSplit(Split {
            children: Vec::from([
                Container::Leaf(Leaf {
                    key: "prompt".to_string(),
                    size_type: SizeTypeEnum::Fixed,
                    size: 64,
                    component: Component::Prompt(prompt),
                }),
                Container::Leaf(Leaf {
                    key: "list".to_string(),
                    size_type: SizeTypeEnum::Percent,
                    size: 100,
                    component: Component::SelectList(select_list),
                }),
            ]),
        }),
    };

    let mut lay = layout2.generate2(
        canvas.window().size().0 as usize,
        canvas.window().size().1 as usize,
    );

    // let mut cur_prompt = "a".to_string(); //FIXME this is wack, just a value to not be equal to
    //initial prompt
    while running {
        // p.type_id();
        // dbg!(&p.1)c;
        // let pr = p.downcast_ref::<Box<Prompt>>();
        // let prompt_text = pr.unwrap().text.clone();

        // let l: &mut dyn Any = comps.get_mut("list").unwrap();
        // let li: &mut SelectList<SourceItem> = l.downcast_mut::<SelectList<SourceItem>>().unwrap();

        // // let prompt_text = &comps.get("prompt").unwrap().text;
        // if prompt_text != cur_prompt {
        //     li.set_list_and_prompt(items.clone(), prompt_text.to_string());
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
            for LayoutItem(_, _k, p) in lay.iter_mut() {
                let comp: &mut dyn components::traits::Component = match p {
                    Component::Prompt(prompt) => prompt,
                    Component::SelectList(list) => list,
                };
                comp.consume_event(event);
            }
        }

        // Set draw color and clear
        canvas.set_draw_color(Color::RGBA(50, 50, 50, 0));
        canvas.clear();

        // Render all components
        for LayoutItem(rect, _k, p) in lay.iter_mut() {
            let comp: &mut dyn components::traits::Component = match p {
                Component::Prompt(prompt) => prompt,
                Component::SelectList(list) => list,
            };

            let mut tex = tc
                .create_texture_target(PixelFormatEnum::RGBA8888, rect.width(), rect.height())
                .unwrap();
            // let comp = comps.get_mut(key).unwrap();
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
