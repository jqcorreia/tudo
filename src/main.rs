extern crate sdl2;

pub mod animation;
pub mod app;
pub mod components;
pub mod context;
pub mod execute;
pub mod layout;
pub mod sources;
pub mod utils;

use std::process::Command;
use std::time::Duration;
use std::time::Instant;

use animation::Animation;
use app::init;
use app::App;
use components::enums::Component;
use components::list::SelectList;
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
use sources::Source;

use sdl2::{keyboard::Keycode, pixels::Color};
use sources::apps::DesktopApplications;
use sources::secrets::Secrets;
use sources::windows::WindowSource;
use sources::SourceItem;
use utils::cache::TextureCache;
use utils::misc;

use crate::layout::Container;

const FONT_PATH: &str = "/usr/share/fonts/noto/NotoSans-Regular.ttf";

fn main() {
    // First measurement and initial state
    let initial_instant = Instant::now();
    let mut first_render = true;

    // Instantiate ttf since this needs to be passed around as ref
    let ttf = sdl2::ttf::init().unwrap();

    //NOTE(quadrado) The image context just needs to exist. Weird. Use other lib?
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG);

    // Create app context and main window canvas
    let (mut app, mut main_canvas) = init(&ttf);

    app.load_font("normal-20".to_string(), FONT_PATH.to_string(), 20);
    app.load_font("normal-16".to_string(), FONT_PATH.to_string(), 16);

    // Event pump, for now just sits in the main loop
    let mut event_pump = app.sdl.event_pump().unwrap();
    let tc = main_canvas.texture_creator();

    let mut cache = TextureCache::new(&tc);

    // Process sources and pre-calculate global items list
    let mut sources: Vec<Box<dyn Source>> = vec![
        Box::new(DesktopApplications::new()),
        Box::new(WindowSource::new()),
        Box::new(Secrets::new()),
    ];

    for source in sources.iter_mut() {
        source.calculate_items();
    }

    // Generate items list from all sources
    let mut items: Vec<SourceItem> = Vec::new();
    for source in sources {
        for item in source.items().iter() {
            items.push(item.clone());
        }
    }

    // Create main UI components
    let prompt = text::Prompt::new();
    let mut select_list = SelectList::<SourceItem>::new();
    select_list.on_select = execute;

    // Define layout
    let mut layout2 = Layout {
        gap: 2,
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

    // Generate layout rects
    let mut lay = layout2.generate2(
        main_canvas.window().size().0 as usize,
        main_canvas.window().size().1 as usize,
    );

    // misc main loop setup
    let mut tick_time = Instant::now();
    let mut draw_fps = false;
    let mut fps = 0;
    let mut frame_lock = true;
    let frame_lock_value = 60;

    let (ww, mut wh) = main_canvas.window().size();

    let mut anim = Animation::new(&mut wh, 0);

    while app.running {
        // Sometime elapsed time is 0 and we need to account for that
        if tick_time.elapsed().as_millis() > 0 {
            fps = 1000 / tick_time.elapsed().as_millis();
            tick_time = Instant::now();
        }

        let elapsed = initial_instant.elapsed().as_millis();
        let ps: String;
        // We need to do this since we cannot have multiple mutable borrows of lay
        // NOTE(quadrado): must revisit this
        {
            let p: &Prompt = &mut lay.get(0).unwrap().2.as_variant().unwrap();
            ps = p.text.clone().into();
            if ps.len() == 0 {
                anim.set_target(lay.get(0).unwrap().0.height() + 3, Some(elapsed));
            } else {
                anim.set_target(768, Some(elapsed));
            }
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
                } => app.running = false,
                sdl2::event::Event::Quit { .. } => app.running = false,
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

                comp.consume_event(&_event, &mut app);
            }
        }
        anim.tick(elapsed);

        main_canvas.window_mut().set_size(ww, *anim.value).unwrap(); // set_size accepts 0 as "do not change"

        // Set draw color and clear
        main_canvas.set_draw_color(Color::RGBA(50, 50, 50, 255));
        main_canvas.clear();

        // Render all components
        for LayoutItem(rect, _k, p) in lay.iter_mut() {
            let comp: &mut dyn components::traits::Render = match p {
                Component::Prompt(prompt) => prompt,
                Component::SelectList(list) => list,
            };

            let mut tex = tc
                .create_texture_target(PixelFormatEnum::RGBA8888, rect.width(), rect.height())
                .unwrap();
            main_canvas
                .with_texture_canvas(&mut tex, |c| {
                    comp.render(&tc, &mut cache, &app, c, *rect, elapsed);
                })
                .unwrap();

            main_canvas.copy(&tex, None, *rect).unwrap();
        }

        // Draw info
        if draw_fps {
            let info_tex = tc
                .create_texture_from_surface(
                    &app.get_font("normal-20")
                        .render(&format!("{}", fps).to_string())
                        .blended(Color::RGBA(0, 120, 0, 128))
                        .unwrap(),
                )
                .unwrap();
            let info_tex_query = info_tex.query();
            main_canvas
                .copy(
                    &info_tex,
                    None,
                    Rect::new(
                        (main_canvas.window().size().0 - 200) as i32,
                        (main_canvas.window().size().1 - 100) as i32,
                        info_tex_query.width,
                        info_tex_query.height,
                    ),
                )
                .unwrap();
        }

        main_canvas.present();

        if first_render {
            first_render = false;
            println!(
                "Time to first render: {}ms",
                initial_instant.elapsed().as_millis()
            )
        } else {
            let frame_lock_duration = Duration::new(0, (1000 / frame_lock_value) * 1_000_000);

            // Make sure we are not overflowing the substraction
            if frame_lock && frame_lock_duration.as_millis() > tick_time.elapsed().as_millis() {
                spin_sleep::sleep(
                    Duration::new(0, (1000 / frame_lock_value) * 1_000_000) - tick_time.elapsed(),
                );
            }
        }
    }
    if app.clipboard.is_some() {
        let _out = Command::new("sh")
            .arg("-c")
            .arg(format!(
                r"echo -n {} | xsel --clipboard --input",
                app.clipboard.clone().unwrap().replace("\n", "")
            ))
            .output()
            .unwrap()
            .stdout;
    }
}
