extern crate sdl2;

pub mod components;
pub mod execute;
pub mod layout;
pub mod sources;
pub mod utils;

use std::cell::RefCell;
use std::process::Command;
use std::rc::Rc;
use std::sync::atomic::AtomicBool;
use std::thread::sleep;
use std::time::Duration;

use components::enums::Component;
use components::list::SelectList;
use components::list::Viewport;
use components::text;

use components::text::Prompt;
use enum_downcast::AsVariant;
use enum_downcast::AsVariantMut;
use execute::execute;
use layout::Layout;
use layout::LayoutItem;
use layout::Leaf;
use layout::SizeTypeEnum;
use layout::Split;
use sdl2::image::InitFlag;
use sdl2::pixels::PixelFormatEnum;
use sdl2::Sdl;
use sources::Source;

use sdl2::{keyboard::Keycode, pixels::Color};
use sources::apps::DesktopApplications;
use sources::lua::LuaSource;
use sources::secrets::Secrets;
use sources::windows::WindowSource;
use sources::SourceItem;
use utils::cache::TextureCache;

use crate::layout::Container;

static mut BLEND: bool = true;

// Struct that contains "global" pointers such as sdl2
#[derive(Clone)]
pub struct AppContext {
    pub sdl: Sdl,
    pub running: bool,
    pub clipboard: Option<String>,
}

fn main() {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let ttf = sdl2::ttf::init().unwrap();
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG);
    let window = video
        .window("tudo", 1024, 768)
        .opengl()
        .borderless()
        .position_centered()
        .build()
        .unwrap();

    let running = true;

    let ctx = Rc::new(RefCell::new(AppContext {
        sdl: sdl.clone(),
        running,
        clipboard: None,
    }));

    let font_size = 20;
    let font_path = "/usr/share/fonts/noto/NotoSans-Regular.ttf";

    let font = ttf.load_font(font_path, font_size).unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl.event_pump().unwrap();

    let tc = canvas.texture_creator();

    let mut cache = TextureCache::new(&tc);

    // Process sources and generate global items list
    let mut sources: Vec<Box<dyn Source>> = vec![
        Box::new(DesktopApplications::new()),
        Box::new(WindowSource::new()),
        // Box::new(Secrets::new()),
        Box::new(LuaSource::new("plugins/pass.lua".to_string())),
    ];

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

    let mut select_list = SelectList::<SourceItem>::new(Rc::clone(&ctx));
    select_list.viewport = Viewport(0, 20);

    select_list.on_select = execute;

    let mut layout2 = Layout {
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

    while ctx.borrow().running {
        let ps: String;
        // We need to do this since we cannot have multiple mutable borrows of lay
        {
            let p: &Prompt = &mut lay.get(0).unwrap().2.as_variant().unwrap();
            ps = p.text.clone().into();
        }
        {
            let l: &mut SelectList<SourceItem> =
                &mut lay.get_mut(1).unwrap().2.as_variant_mut().unwrap();
            l.set_list_and_prompt(items.clone(), ps)
        }

        // Consume events and pass them to the components
        let cur_events: Vec<_> = event_pump.poll_iter().collect();
        for event in cur_events.iter() {
            match event {
                sdl2::event::Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => ctx.borrow_mut().running = false,
                sdl2::event::Event::KeyDown {
                    keycode: Some(Keycode::F1),
                    ..
                } => unsafe { BLEND = !BLEND },
                sdl2::event::Event::Quit { .. } => ctx.borrow_mut().running = false,
                sdl2::event::Event::MouseButtonDown { x, y, .. } => {
                    println!("{} {}", x, y)
                }
                _ => (),
            }
            for LayoutItem(_, _k, p) in lay.iter_mut() {
                let comp: &mut dyn components::traits::EventConsumer = match p {
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
            let comp: &mut dyn components::traits::Render = match p {
                Component::Prompt(prompt) => prompt,
                Component::SelectList(list) => list,
            };

            let mut tex = tc
                .create_texture_target(PixelFormatEnum::RGBA8888, rect.width(), rect.height())
                .unwrap();
            canvas
                .with_texture_canvas(&mut tex, |c| {
                    comp.render(&mut cache, &font, c, *rect);
                })
                .unwrap();

            canvas.copy(&tex, None, *rect).unwrap();
        }
        canvas.present();

        // lock it as 60 frames per second
        sleep(Duration::new(0, (1000 / 60) * 1_000_000));
    }
    if ctx.borrow().clipboard.is_some() {
        let _out = Command::new("sh")
            .arg("-c")
            .arg(format!(
                r"echo -n {} | xsel --clipboard --input",
                ctx.borrow().clipboard.clone().unwrap().replace("\n", "")
            ))
            .output()
            .unwrap()
            .stdout;
    }
}
