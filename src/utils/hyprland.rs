use std::io::prelude::*;
use std::os::unix::net::UnixStream;
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::time::Duration;

use log::info;
use spin_sleep::sleep;

fn open_hyprland_socket_1() -> std::io::Result<UnixStream> {
    let xdg_runtime_dir = std::env::var("XDG_RUNTIME_DIR").unwrap();
    let hyprland_signature = std::env::var("HYPRLAND_INSTANCE_SIGNATURE").unwrap();
    let socket_path = format!(
        "{}/hypr/{}/.socket.sock",
        xdg_runtime_dir, hyprland_signature
    );
    let stream = UnixStream::connect(socket_path)?;
    Ok(stream)
}

fn open_hyprland_socket_2() -> std::io::Result<UnixStream> {
    let xdg_runtime_dir = std::env::var("XDG_RUNTIME_DIR").unwrap();
    let hyprland_signature = std::env::var("HYPRLAND_INSTANCE_SIGNATURE").unwrap();
    let socket_path = format!(
        "{}/hypr/{}/.socket2.sock",
        xdg_runtime_dir, hyprland_signature
    );
    let stream = UnixStream::connect(socket_path)?;
    Ok(stream)
}

#[derive(Debug)]
pub struct Workspace {
    pub id: u8,
}

pub struct Hyprland {
    rx: Receiver<String>,
}

impl Hyprland {
    pub fn new() -> std::io::Result<Hyprland> {
        info!("Starting hyprland client");
        let mut listen_stream = open_hyprland_socket_2()?;
        info!("Finished Starting hyprland client");

        let (tx, rx) = channel();
        let _tx = tx.clone();
        // info!("Starting hyprland sock2 stream");
        // thread::spawn(move || loop {
        //     let mut buf = String::new();
        //     let mut buf: Vec<u8> = vec![];
        //     listen_stream.read_to_end(&mut buf).unwrap();
        //     dbg!(&buf);
        //     _tx.send(String::from_utf8(buf).unwrap()).unwrap();
        //     sleep(Duration::new(0, 100));
        // });
        Ok(Hyprland { rx })
    }

    pub fn send_command(&mut self, command: impl AsRef<str>) -> String {
        let mut cmd_stream = open_hyprland_socket_1().unwrap();
        cmd_stream
            .write_all(command.as_ref().to_string().as_bytes())
            .unwrap();
        let mut response = String::new();
        cmd_stream.read_to_string(&mut response).unwrap();

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

    pub fn rx(&self) -> &Receiver<String> {
        &self.rx
    }
}

#[cfg(test)]
mod tests {
    use std::os::unix::net::UnixDatagram;

    use super::*;

    #[test]
    fn test_hyprland() {
        let sock = UnixStream::connect("/run/user/1000/hypr/918d8340afd652b011b937d29d5eea0be08467f5_1732916189_1583074004/.socket2.sock");
        dbg!(&sock);
        let mut buf: Vec<u8> = vec![];
        // let i = stream.read_to_end(&mut buf);
        dbg!(sock.unwrap().read_to_end(&mut buf));
        dbg!(buf);
        // let mut stream = UnixStream::connect("/run/user/1000/hypr/918d8340afd652b011b937d29d5eea0be08467f5_1732916189_1583074004/.socket2.sock").unwrap();
        // stream.set_nonblocking(true);

        // let mut buf: Vec<u8> = vec![];
        // let i = stream.read_to_end(&mut buf);
    }
}
