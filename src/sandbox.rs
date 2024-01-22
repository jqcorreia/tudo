#![allow(dead_code, unused)]
extern crate sdl2;

pub mod components;
pub mod context;
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
use context::RenderContext;
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
use utils::font::FontManager;
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
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let ttf = sdl2::ttf::init().unwrap();
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG);

    let fm = FontManager::new(&ttf);
    let font_size = 20;
    let font_path = "/usr/share/fonts/noto/NotoSans-Regular.ttf";

    let font = fm.load_font(font_path.to_string(), font_size);
    let font2 = fm.load_font(font_path.to_string(), font_size);

    let rc = RenderContext {
        // sdl: sdl.clone(),
        // tc: &tc,
        // video: video.clone(),
        fonts: &fm,
    };
}
