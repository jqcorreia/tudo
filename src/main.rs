extern crate sdl2;

pub mod animation;
pub mod app;
pub mod components;
pub mod config;
pub mod execute;
pub mod layout;
pub mod screen;
pub mod sources;
pub mod utils;

use std::env;
use std::fs::create_dir_all;
use std::process::Command;
use std::thread;
use std::time::Duration;
use std::time::Instant;

use app::init;
use app::App;

use config::load_config;
use execute::execute;
use screen::MainScreen;
use sdl2::event::Event;
use sdl2::image::InitFlag;
use sdl2::rect::Rect;
use sources::Source;

use sdl2::pixels::Color;
use sources::apps::DesktopApplications;
use sources::lua::LuaSource;
use sources::secrets::Secrets;
use sources::tmux::Tmux;
use sources::windows::WindowSource;
use sources::SourceItem;
use std::sync::{Arc, Mutex};
use utils::cache::TextureCache;
use utils::misc;

fn already_running(lock_path: &String) -> bool {
    match std::fs::read(lock_path.clone()) {
        Ok(_) => true,
        Err(_) => {
            std::fs::write(lock_path, Vec::new()).unwrap();
            false
        }
    }
}

fn check_config_folder() -> String {
    let home = env::var("HOME").expect("$HOME not set, can't create config folder");
    let base_path = format!("{}/.config/tudo", home);

    create_dir_all(base_path.clone()).unwrap();
    base_path.to_string()
}

fn main() {
    let base_folder = check_config_folder();
    let lock_path = format!("{}/run-lock", base_folder);

    if already_running(&lock_path) {
        println!("Tudo is already running!");
        return;
    }

    let config = load_config(format!("{}/config.lua", base_folder));

    // First measurement and initial state
    let initial_instant = Instant::now();
    let mut first_render = true;

    // Instantiate ttf since this needs to be passed around as ref
    let ttf = sdl2::ttf::init().unwrap();

    //NOTE(quadrado) The image context just needs to exist. Weird. Use other lib?
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG);

    // Create app context and main window canvas
    let (mut app, mut main_canvas) = init(&ttf);

    // Load initial fonts
    app.load_font("normal-20".to_string(), &config.font_file, 20);
    app.load_font("normal-16".to_string(), &config.font_file, 16);

    // Event pump, for now just sits in the main loop
    let mut event_pump = app.sdl.event_pump().unwrap();

    // Create texture creator for the main window canvas
    let tc = main_canvas.texture_creator();

    let mut cache = TextureCache::new(&tc);

    // Generate items list from all sources
    let items: Arc<Mutex<Vec<SourceItem>>> = Arc::new(Mutex::new(Vec::new()));
    let completed_threads: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));

    let sources: Vec<Box<dyn Source + Send>> = vec![
        Box::new(DesktopApplications::new()),
        Box::new(WindowSource::new()),
        Box::new(Secrets::new()),
        Box::new(Tmux::new()),
        Box::new(LuaSource::new("plugins/vlad.lua".to_string())),
    ];

    // Get number of sources before consuming them
    let total_threads = sources.len();

    // 'async' it
    for source in sources {
        let i = items.clone();
        let ct = completed_threads.clone();
        thread::spawn(move || {
            // Calc, ...
            let is = source.generate_items();

            // ... and then lock
            let mut items = i.lock().unwrap();
            items.extend(is);

            // Increment completed_threads
            let mut ct = ct.lock().unwrap();
            *ct += 1;
        });
    }

    // misc main loop setup
    let mut tick_time = Instant::now();
    let mut fps = 0;
    let frame_lock_value = 60;

    let mut main_screen = MainScreen::new(
        &config,
        main_canvas.window().size().0 as usize,
        main_canvas.window().size().1 as usize,
        items.clone(),
    );

    while app.running {
        let ct = completed_threads.lock().unwrap();
        app.loading = *ct != total_threads as u32;

        let clear_color = if app.loading {
            Color::RGBA(200, 0, 0, 255)
        } else {
            Color::RGBA(50, 50, 50, 255)
        };
        // We need to drop here in order to yield the lock
        drop(ct);

        // Sometime elapsed time is 0 and we need to account for that
        if tick_time.elapsed().as_millis() > 0 {
            fps = 1000 / tick_time.elapsed().as_millis();
            tick_time = Instant::now();
        }

        // Calculate elapsed time
        let elapsed = initial_instant.elapsed().as_millis();

        // Consume events and process them
        let cur_events = event_pump
            .poll_iter()
            .map(|event| misc::ignore_numlock(&event))
            .collect::<Vec<Event>>();

        // Handle application global events
        app.handle_global_events(&cur_events);

        // Screen update
        main_screen.update(&mut app, &cur_events, elapsed);

        // Set draw color and clear
        main_canvas.set_draw_color(clear_color);
        main_canvas.clear();

        main_screen.render(&tc, &mut cache, &app, &mut main_canvas, elapsed);

        // Draw info
        if app.draw_fps {
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
            if app.frame_lock && frame_lock_duration.as_millis() > tick_time.elapsed().as_millis() {
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

    // Remove run lock
    let _ = std::fs::remove_file(lock_path);
}
