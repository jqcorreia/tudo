pub mod animation;
pub mod app;
pub mod config;
pub mod execute;
pub mod screen;
pub mod sources;
pub mod utils;

pub mod ui;

use std::borrow::Borrow;
use std::collections::HashMap;
use std::process::Command;
use std::sync::mpsc::channel;
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;

use app::App;

use execute::execute;
use log::info;
use mlua::Lua;
use screen::debug_screen::DebugScreen;
use screen::info_screen::InfoScreen;
use screen::main_screen::MainScreen;
use sdl2::event::Event;
use sdl2::image::InitFlag;
use sdl2::libc::SIGINT;
use sdl2::libc::SIGUSR2;
use signal_hook::iterator::Signals;
use simple_logger::SimpleLogger;
use sources::Source;

use screen::Screen;
use sdl2::pixels::Color;
use sources::apps::DesktopApplications;
use sources::secrets::Secrets;
use sources::tmux::Tmux;
use sources::windows::WindowSource;
use sources::SourceItem;
use std::sync::{Arc, Mutex};
use utils::cache::TextureCache;
use utils::draw::draw_string;
use utils::font::FontConfig;
use utils::misc;

const PID_FILE: &str = "/run/user/1000/tudo.pid"; //TODO(quadrado): Use configuration value instead
                                                  //of this one.

fn check_running_state() -> bool {
    match std::fs::exists(PID_FILE) {
        Ok(true) => {
            let pid = String::from_utf8(std::fs::read(PID_FILE).unwrap()).unwrap();
            println!("Lock file PID: {}", pid);
            if !std::fs::exists(format!("/proc/{}", pid)).unwrap() {
                println!("PID in lock file not running. Starting new instance");
                return false;
            }

            println!("Opening existing tudo session");
            let _ = Command::new("sh")
                .args(["-c", &format!("kill -s USR2 {}", &pid)])
                .spawn();
            true
        }
        _ => false,
    }
}

fn calc() {
    let lua = Lua::new();
    let script = "
    local sin = math.sin
    local cos = math.cos
    local sqrt = math.sqrt
    ";

    dbg!(lua
        .load(script.to_string() + "return sqrt(25) + 30")
        .eval::<f32>()
        .unwrap());
}

fn main() {
    calc();
    // Initialize logging
    SimpleLogger::new().init().unwrap();
    if let Ok(value) = std::env::var("XDG_SESSION_TYPE") {
        if value == "wayland" {
            unsafe { std::env::set_var("SDL_VIDEODRIVER", "wayland") };
        }
    }

    info!("Starting TUDO");

    if check_running_state() {
        return;
    };

    std::fs::write(PID_FILE, std::process::id().to_string()).unwrap();

    // Control channel for signalling handling
    let (tx, rx) = channel::<i32>();

    let mut signals = Signals::new([SIGUSR2, SIGINT]).unwrap();

    thread::spawn(move || {
        for sig in signals.forever() {
            tx.send(sig).unwrap();
        }
    });

    // First measurement and initial state
    let initial_instant = Instant::now();
    let mut first_render = true;

    // Instantiate ttf since this needs to be passed around as ref
    let ttf = sdl2::ttf::init().unwrap();

    //NOTE(quadrado) The image context just needs to exist. Weird. Use other lib?
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG);

    info!("Initializing App State");
    // Create app context and main window canvas
    let (mut app, mut main_canvas) = App::init();
    info!("Finished initializing App State");

    // Create texture creator for the main window canvas
    let tc = main_canvas.texture_creator();

    // Create texture caches
    let mut cache = TextureCache::new(&tc, &ttf);

    // Load initial fonts
    cache.fonts.load_font(FontConfig {
        alias: "normal-30".to_string(),
        family: app.config.font_family.clone(),
        point_size: 30,
    });
    cache.fonts.load_font(FontConfig {
        alias: "normal-28".to_string(),
        family: app.config.font_family.clone(),
        point_size: 28,
    });
    cache.fonts.load_font(FontConfig {
        alias: "normal-20".to_string(),
        family: app.config.font_family.clone(),
        point_size: 20,
    });
    cache.fonts.load_font(FontConfig {
        alias: "normal-16".to_string(),
        family: app.config.font_family.clone(),
        point_size: 14,
    });

    // Generate items list from all sources
    let items: Arc<Mutex<Vec<SourceItem>>> = Arc::new(Mutex::new(Vec::new()));
    let completed_threads: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));

    let sources: Vec<Box<dyn Source + Send>> = vec![
        Box::new(DesktopApplications::new()),
        Box::new(WindowSource::new()),
        Box::new(Secrets::new()),
        Box::new(Tmux::new()),
        // Box::new(LuaSource::new("plugins/vlad.lua".to_string())),
        // Box::new(DummySource::new()),
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
    let window_width = main_canvas.window().size().0 as i32;
    let window_height = main_canvas.window().size().1 as i32;

    let main_screen = MainScreen::new(
        &app.config,
        main_canvas.window().size().0 as usize,
        main_canvas.window().size().1 as usize,
        items.clone(),
    );

    let debug = DebugScreen::new(&app.config);
    let info_screen = InfoScreen::new(&app.config);

    let mut screen_map: HashMap<String, Box<dyn Screen>> = HashMap::new();
    screen_map.insert("main".to_string(), Box::new(main_screen));
    screen_map.insert("debug".to_string(), Box::new(debug));
    screen_map.insert("info".to_string(), Box::new(info_screen));

    while app.running {
        let current_screen = screen_map.get_mut(&app.current_screen_id).unwrap();
        let ct = completed_threads.lock().unwrap();
        app.loading = *ct != total_threads as u32;
        // We need to drop here in order to yield the lock
        drop(ct);

        if app.hidden {
            if let Ok(sig) = rx.try_recv() {
                match sig {
                    SIGINT => app.running = false, // This will stop the main thread when hidden
                    // and receiving a Ctrl-C
                    SIGUSR2 => {
                        main_canvas.window_mut().show();
                        app.hidden = false
                    }
                    _ => (),
                }
            }
            sleep(Duration::from_millis(10));
        }

        // Sometime elapsed time is 0 and we need to account for that
        if tick_time.elapsed().as_millis() > 0 {
            fps = 1000 / tick_time.elapsed().as_millis();
            tick_time = Instant::now();
        }

        // Calculate elapsed time
        let elapsed = initial_instant.elapsed().as_millis();

        // Consume events and process them
        let cur_events = app
            .event_pump
            .poll_iter()
            .map(|event| misc::ignore_numlock(&event))
            .collect::<Vec<Event>>();

        // Handle application global events
        app.handle_global_events(&cur_events);
        if app.should_hide {
            main_canvas.window_mut().hide();
            current_screen.reset();
            app.should_hide = false;
            app.hidden = true
        }

        // Screen update
        current_screen.update(&mut app, &cur_events, elapsed);

        // Screen render
        current_screen.render(&tc, &mut cache, &app, &mut main_canvas, elapsed);

        // Draw info directly into the canvas
        if app.draw_fps {
            let font = &cache.fonts.get_font("normal-20");

            draw_string(
                format!("{}", fps).to_string(),
                &mut main_canvas,
                font,
                Color::RGBA(0, 120, 0, 128),
                window_width - 200,
                window_height - 200,
            );
        }

        main_canvas.present();

        if first_render {
            first_render = false;
            println!(
                "Time to first render: {}ms",
                initial_instant.elapsed().as_millis()
            )
        }
        let frame_lock_duration = Duration::new(0, (1000 / frame_lock_value) * 1_000_000);

        // Make sure we are not overflowing the substraction
        if app.frame_lock && frame_lock_duration.as_millis() > tick_time.elapsed().as_millis() {
            spin_sleep::sleep(
                Duration::new(0, (1000 / frame_lock_value) * 1_000_000) - tick_time.elapsed(),
            );
        }
        if app.clipboard.is_some() {
            let cmd = match std::env::var("XDG_SESSION_TYPE") {
                Ok(session) => match session.borrow() {
                    "wayland" => "wl-copy",
                    "x11" => "xsel --clipboard --input",
                    _ => "",
                },
                _ => "",
            };
            let full_cmd = format!(
                r"echo -n {} | {}",
                app.clipboard.clone().unwrap().replace("\n", ""),
                cmd
            );

            let _ = Command::new("sh").arg("-c").arg(full_cmd).spawn();
            app.clipboard = None;
        }
    }
    std::fs::remove_file(PID_FILE).unwrap();
}
