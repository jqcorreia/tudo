use std::process::Command;

use dyn_clone::DynClone;
use xcb::x::Window;

use crate::App;

use super::windows::switch_to_window;

pub trait Action: DynClone {
    fn execute(&self, ctx: &mut App);
    fn tags(&self) -> Vec<String>;
}
dyn_clone::clone_trait_object!(Action);

#[derive(Debug, Clone, PartialEq)]
pub struct PassSecretAction {
    pub secret_name: String,
}

impl Action for PassSecretAction {
    fn execute(&self, ctx: &mut App) {
        let pass_args = vec![
            "-c".to_string(),
            format!("pass {}", self.secret_name.to_string()),
        ];
        let pass_otp_args = vec![
            "-c".to_string(),
            format!("pass otp {}", self.secret_name.to_string()),
        ];

        let output = Command::new("sh").args(pass_args.clone()).output();
        let ot = String::from_utf8(output.unwrap().stdout).unwrap();

        if ot.starts_with("otpauth://") {
            let otp_output = Command::new("sh").args(pass_otp_args.clone()).output();
            let oot = String::from_utf8(otp_output.unwrap().stdout).unwrap();
            ctx.clipboard = Some(oot.clone());
        } else {
            ctx.clipboard = Some(ot);
        }
        ctx.should_hide = true;
    }
    fn tags(&self) -> Vec<String> {
        vec!["secret".to_string()]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RunAction {
    pub path: String,
    pub exit_after: bool,
    pub clip_output: bool,
}

impl Action for RunAction {
    fn execute(&self, ctx: &mut App) {
        let args = vec!["-c", &self.path];

        if self.clip_output {
            let output = Command::new("sh").args(args).output();
            let ot = String::from_utf8(output.unwrap().stdout).unwrap();
            ctx.clipboard = Some(ot);
        } else {
            let _cmd = Command::new("sh").args(args).spawn();
        }

        if self.exit_after {
            ctx.should_hide = true;
        }
    }
    fn tags(&self) -> Vec<String> {
        vec!["run".to_string()]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WindowSwitchAction {
    pub window: Window,
    pub exit_after: bool,
}

impl Action for WindowSwitchAction {
    fn execute(&self, ctx: &mut App) {
        let (conn, _) = xcb::Connection::connect(None).unwrap();
        let root = conn.get_setup().roots().nth(0).unwrap().root();
        let _ = switch_to_window(&conn, &self.window, &root);

        if self.exit_after {
            ctx.should_hide = true;
        }
    }
    fn tags(&self) -> Vec<String> {
        vec!["window".to_string()]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TmuxAction {
    pub session: String,
}

impl Action for TmuxAction {
    fn execute(&self, ctx: &mut App) {
        Command::new("sh")
            .args(["-c", &format!("alacritty -e tmux new -As {}", self.session)])
            .spawn()
            .unwrap();
        ctx.should_hide = true;
    }
    fn tags(&self) -> Vec<String> {
        vec!["tmux".to_string()]
    }
}
