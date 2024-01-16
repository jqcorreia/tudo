use std::process::Command;

use xcb::x::Window;

use crate::AppContext;

use super::windows::switch_to_window;

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Run(RunAction),
    WindowSwitch(WindowSwitchAction),
    PassSecret(PassSecretAction),
}

pub trait ActionTrait {
    fn execute2(&self, ctx: &mut AppContext);
}

impl Action {
    pub fn execute(&self, ctx: &mut AppContext) {
        match self {
            Action::Run(action) => action.execute(ctx),
            Action::PassSecret(action) => action.execute(ctx),
            Action::WindowSwitch(action) => action.execute(ctx),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PassSecretAction {
    pub secret_name: String,
}

impl PassSecretAction {
    pub fn execute(&self, ctx: &mut AppContext) {
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
        ctx.running = false;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RunAction {
    pub path: String,
    pub exit_after: bool,
    pub clip_output: bool,
}

impl RunAction {
    pub fn execute(&self, ctx: &mut AppContext) {
        let args = vec!["-c", &self.path];

        if self.clip_output {
            let output = Command::new("sh").args(args).output();
            let ot = String::from_utf8(output.unwrap().stdout).unwrap();
            ctx.clipboard = Some(ot);
        } else {
            let _cmd = Command::new("sh").args(args).spawn();
        }

        if self.exit_after {
            ctx.running = false;
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WindowSwitchAction {
    pub window: Window,
    pub exit_after: bool,
}

impl WindowSwitchAction {
    pub fn execute(&self, ctx: &mut AppContext) {
        let (conn, _) = xcb::Connection::connect(None).unwrap();
        let root = conn.get_setup().roots().nth(0).unwrap().root();
        let _ = switch_to_window(&conn, &self.window, &root);

        if self.exit_after {
            ctx.running = false;
            std::process::exit(0);
        }
    }
}
