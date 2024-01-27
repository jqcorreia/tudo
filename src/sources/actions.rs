use std::process::Command;

use xcb::x::Window;

use crate::App;

use super::windows::switch_to_window;

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Run(RunAction),
    WindowSwitch(WindowSwitchAction),
    PassSecret(PassSecretAction),
    Tmux(TmuxAction),
}

impl Action {
    pub fn execute(&self, ctx: &mut App) {
        match self {
            Action::Run(action) => action.execute(ctx),
            Action::PassSecret(action) => action.execute(ctx),
            Action::WindowSwitch(action) => action.execute(ctx),
            Action::Tmux(action) => action.execute(ctx),
        }
    }
    pub fn tags(&self) -> Vec<String> {
        match self {
            Action::Run(_) => vec!["run".to_string()],
            Action::PassSecret(_) => vec!["secret".to_string()],
            Action::WindowSwitch(_) => vec!["window".to_string()],
            Action::Tmux(_) => vec!["tmux".to_string()],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PassSecretAction {
    pub secret_name: String,
}

impl PassSecretAction {
    pub fn execute(&self, ctx: &mut App) {
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
    pub fn execute(&self, ctx: &mut App) {
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
    pub fn execute(&self, ctx: &mut App) {
        let (conn, _) = xcb::Connection::connect(None).unwrap();
        let root = conn.get_setup().roots().nth(0).unwrap().root();
        let _ = switch_to_window(&conn, &self.window, &root);

        if self.exit_after {
            ctx.running = false;
            std::process::exit(0);
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TmuxAction {
    pub session: String,
}

impl TmuxAction {
    pub fn execute(&self, ctx: &mut App) {
        Command::new("sh")
            .args(["-c", &format!("alacritty -e tmux new -As {}", self.session)])
            .spawn()
            .unwrap();
        ctx.running = false;
    }
}
