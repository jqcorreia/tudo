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
    info!("Hyprland socket2 @ {}", socket_path);
    UnixStream::connect(socket_path)
    // let stream = UnixStream::connect(socket_path)?;
    // Ok(stream)
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
        info!("Starting hyprland sock2 stream");
        let mut sock = open_hyprland_socket_2().unwrap();
        let (tx, rx) = channel::<String>();
        sock.set_nonblocking(true).unwrap();
        thread::spawn(move || loop {
            let mut buf: Vec<u8> = vec![];
            match sock.read_to_end(&mut buf) {
                Ok(foo) => {
                    // This never happens...!
                    dbg!(foo);
                }
                Err(_) => {
                    // dbg!(e);
                }
            };
            // sock.read_to_end(&mut buf).unwrap();

            if !buf.is_empty() {
                let buf_string = String::from_utf8(buf.clone()).unwrap();

                // Split by '\n' and send them down the pipe
                buf_string.split('\n').for_each(|e| {
                    tx.send(e.to_string()).unwrap();
                });
            }
            // dbg!(String::from_utf8(buf.clone()));
            sleep(Duration::new(0, 100_000_000));
        });
        info!("Finished Starting hyprland client");
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

#[allow(unused)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hyprland() {
        // Hyprland::new();
        // loop {}
        // let mut sock = UnixStream::connect(
        //     "/run/user/1000/hypr/918d8340afd652b011b937d29d5eea0be08467f5_1733075473_1355729644/.socket2.sock",
        // ).unwrap();
        // dbg!(&sock);
        // sock.set_nonblocking(true);
        // loop {
        //     let mut buf: Vec<u8> = vec![];
        //     match sock.read_to_end(&mut buf) {
        //         Ok(foo) => {
        //             dbg!(foo);
        //         }
        //         Err(e) => {
        //             dbg!(e);
        //         }
        //     };
        //     dbg!(String::from_utf8(buf.clone()));
        //     sleep(Duration::new(0, 100_000_000));
        // }
    }
}
