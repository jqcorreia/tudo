use std::process::Command;

use sdl2::Sdl;

use crate::sources::{
    windows::switch_to_window, Action, RunAction, SourceItem, WindowSwitchAction,
};

pub fn execute(item: &SourceItem, sdl: Sdl) {
    match &item.action {
        Action::Run(RunAction {
            path,
            exit_after,
            clip_output,
        }) => {
            let mut args = vec!["-c"];

            for token in path.split(" ") {
                args.push(token);
            }
            if *clip_output {
                let video = sdl.video().unwrap();
                dbg!(&args);
                let output = Command::new("sh").args(args).output();
                let ot = String::from_utf8(output.unwrap().stdout).unwrap();
                dbg!(&ot);
                // let r = video.clipboard().set_clipboard_text(&ot);
            } else {
                let _cmd = Command::new("sh").args(&args).spawn();
            }

            if *exit_after {
                std::process::exit(0);
            }
        }
        Action::WindowSwitch(WindowSwitchAction { window, exit_after }) => {
            let (conn, _) = xcb::Connection::connect(None).unwrap();
            let root = conn.get_setup().roots().nth(0).unwrap().root();
            let _ = switch_to_window(&conn, window, &root);

            if *exit_after {
                std::process::exit(0);
            }
        }
    }
}
