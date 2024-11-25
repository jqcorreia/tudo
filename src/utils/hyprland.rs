use std::io::prelude::*;
use std::os::unix::net::UnixStream;

fn open_hyprland_socket_1() -> std::io::Result<UnixStream> {
    let xdg_runtime_dir = std::env::var("XDG_RUNTIME_DIR").unwrap();
    let hyprland_signature = std::env::var("HYPRLAND_INSTANCE_SIGNATURE").unwrap();
    let socket_path = format!(
        "{}/hypr/{}/.socket.sock",
        xdg_runtime_dir, hyprland_signature
    );
    dbg!(&socket_path);
    let stream = UnixStream::connect(socket_path)?;
    return Ok(stream);
}

fn open_hyprland_socket_2() -> std::io::Result<UnixStream> {
    let xdg_runtime_dir = std::env::var("XDG_RUNTIME_DIR").unwrap();
    let hyprland_signature = std::env::var("HYPRLAND_INSTANCE_SIGNATURE").unwrap();
    let socket_path = format!(
        "{}/hypr/{}/.socket2.sock",
        xdg_runtime_dir, hyprland_signature
    );
    dbg!(&socket_path);
    let stream = UnixStream::connect(socket_path)?;
    return Ok(stream);
}

#[derive(Debug)]
pub struct Workspace {
    pub id: u8,
}

pub struct Hyprland {
    cmd_stream: UnixStream,
    listen_stream: UnixStream,
}

impl Hyprland {
    pub fn new() -> std::io::Result<Hyprland> {
        let cmd_stream = open_hyprland_socket_1()?;
        let listen_stream = open_hyprland_socket_2()?;

        Ok(Hyprland {
            cmd_stream,
            listen_stream,
        })
    }

    pub fn send_command(&mut self, command: impl AsRef<str>) -> String {
        self.cmd_stream
            .write_all(command.as_ref().to_string().as_bytes())
            .unwrap();
        let mut response = String::new();
        self.cmd_stream.read_to_string(&mut response).unwrap();

        response
    }

    pub fn get_active_workspace(&mut self) -> Workspace {
        // Same code of get_workspaces() but with early return
        let response = self.send_command("activeworkspace");
        for line in response.lines() {
            if line.starts_with("workspace") {
                let current_workspace_id = line.split(" ").nth(2).unwrap().parse().unwrap();
                return Workspace {
                    id: current_workspace_id,
                };
            }
        }
        Workspace { id: 0 }
    }

    pub fn goto_workspace(&mut self, x: u8) -> bool {
        let response = self.send_command(format!("/dispatch workspace {}", x));
        response == "ok"
    }

    pub fn get_workspaces(&mut self) -> Vec<Workspace> {
        let response = self.send_command("workspaces");
        let mut result: Vec<Workspace> = vec![];

        let mut current_workspace_id;
        for line in response.lines() {
            if line.starts_with("workspace") {
                current_workspace_id = line.split(" ").nth(2).unwrap().parse().unwrap();
                result.push(Workspace {
                    id: current_workspace_id,
                });
            }
        }
        result.sort_by(|a, b| a.id.cmp(&b.id));
        result
    }
}
