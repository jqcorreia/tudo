use std::cell::RefCell;
use std::process::Command;
use std::rc::Rc;

use crate::{
    sources::{
        windows::switch_to_window, Action, PassSecretAction, RunAction, SourceItem,
        WindowSwitchAction,
    },
    AppContext,
};

pub fn execute(item: &SourceItem, ctx: Rc<RefCell<AppContext>>) {
    let mut _ctx = ctx.borrow_mut();

    match &item.action {
        Action::Run(RunAction {
            path,
            exit_after,
            clip_output,
        }) => {
            let args = vec!["-c", path];

            if *clip_output {
                // let video = sdl.video().unwrap();
                dbg!(&args);
                let output = Command::new("sh").args(args).output();
                let ot = String::from_utf8(output.unwrap().stdout).unwrap();
                dbg!(&ot);
                // let r = video.clipboard().set_clipboard_text(&ot);
            } else {
                let _cmd = Command::new("sh").args(args).spawn();
            }

            if *exit_after {
                _ctx.running = false;
            }
        }
        Action::WindowSwitch(WindowSwitchAction { window, exit_after }) => {
            let (conn, _) = xcb::Connection::connect(None).unwrap();
            let root = conn.get_setup().roots().nth(0).unwrap().root();
            let _ = switch_to_window(&conn, window, &root);

            if *exit_after {
                _ctx.running = false;
                std::process::exit(0);
            }
        }
        Action::PassSecret(PassSecretAction { secret_name }) => {
            let pass_args = vec![
                "-c".to_string(),
                format!("pass {}", secret_name.to_string()),
            ];
            let pass_otp_args = vec![
                "-c".to_string(),
                format!("pass otp {}", secret_name.to_string()),
            ];

            let output = Command::new("sh").args(pass_args.clone()).output();
            let ot = String::from_utf8(output.unwrap().stdout).unwrap();

            if ot.starts_with("otpauth://") {
                dbg!("otp");
                let otp_output = Command::new("sh").args(pass_otp_args.clone()).output();
                let oot = String::from_utf8(otp_output.unwrap().stdout).unwrap();
                _ctx.clipboard = Some(oot.clone());
                println!("{}", oot);
            } else {
                dbg!("normal");
                _ctx.clipboard = Some(ot);
            }
            _ctx.running = false;
        }
    }
}
