extern crate sdl2;

pub mod components;
pub mod execute;
pub mod layout;
pub mod sources;
pub mod utils;

use std::cell::RefCell;
use std::process::Command;
use std::rc::Rc;
use std::time::Duration;
use std::time::Instant;

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
use sdl2::rect::Rect;
use sdl2::Sdl;
use sources::Source;

use sdl2::{keyboard::Keycode, pixels::Color};
use sources::apps::DesktopApplications;
use sources::secrets::Secrets;
use sources::windows::WindowSource;
use sources::SourceItem;
use utils::cache::TextureCache;
use utils::misc;

use crate::layout::Container;

// Struct that contains "global" pointers such as sdl2
#[derive(Clone)]
pub struct AppContext {
    pub sdl: Sdl,
    pub running: bool,
    pub clipboard: Option<String>,
}

fn main() {
    let initial_instant = Instant::now();
    let mut first_render = true;
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
        Box::new(Secrets::new()),
        // Box::new(LuaSource::new("plugins/pass.lua".to_string())),
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

    let prompt = text::Prompt::new();

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

    let mut tick_time = Instant::now();
    let mut draw_fps = false;
    let mut fps = 0;
    let mut frame_lock = true;
    let frame_lock_value = 60;

    while ctx.borrow().running {
        // Sometime elapsed time is 0 and we need to account for that
        if tick_time.elapsed().as_millis() > 0 {
            fps = 1000 / tick_time.elapsed().as_millis();
            tick_time = Instant::now();
        }

        let elapsed = initial_instant.elapsed().as_millis();
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
            // Ignore NumLock
            let _event = misc::ignore_numlock(&event);

            // Deal with main loop events
            // Things like app quit and global window mouse events
            match _event {
                sdl2::event::Event::KeyDown {
                    keycode: Some(Keycode::F1),
                    ..
                } => draw_fps = !draw_fps,
                sdl2::event::Event::KeyDown {
                    keycode: Some(Keycode::F2),
                    ..
                } => frame_lock = !frame_lock,
                sdl2::event::Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => ctx.borrow_mut().running = false,
                sdl2::event::Event::Quit { .. } => ctx.borrow_mut().running = false,
                sdl2::event::Event::MouseButtonDown { x, y, .. } => {
                    println!("{} {}", x, y)
                }
                _ => (),
            }

            // Pass the event to every component
            for LayoutItem(_, _k, p) in lay.iter_mut() {
                let comp: &mut dyn components::traits::EventConsumer = match p {
                    Component::Prompt(prompt) => prompt,
                    Component::SelectList(list) => list,
                };

                comp.consume_event(&_event);
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
                    comp.render(&tc, &mut cache, &font, c, *rect, elapsed);
                })
                .unwrap();

            canvas.copy(&tex, None, *rect).unwrap();
        }

        // Draw info
        if draw_fps {
            let info_tex = tc
                .create_texture_from_surface(
                    &font
                        .render(&format!("{}", fps).to_string())
                        .blended(Color::RGBA(0, 120, 0, 128))
                        .unwrap(),
                )
                .unwrap();
            let info_tex_query = info_tex.query();
            canvas
                .copy(
                    &info_tex,
                    None,
                    Rect::new(
                        (canvas.window().size().0 - 200) as i32,
                        (canvas.window().size().1 - 100) as i32,
                        info_tex_query.width,
                        info_tex_query.height,
                    ),
                )
                .unwrap();
        }

        canvas.present();

        if first_render {
            first_render = false;
            println!(
                "Time to first render: {}ms",
                initial_instant.elapsed().as_millis()
            )
        } else {
            if frame_lock {
                spin_sleep::sleep(
                    Duration::new(0, (1000 / frame_lock_value) * 1_000_000) - tick_time.elapsed(),
                );
            }
        }
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
