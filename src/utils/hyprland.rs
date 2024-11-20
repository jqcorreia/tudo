use std::io::prelude::*;
use std::os::unix::net::UnixStream;

pub fn open_hyprland_socket_1() {
    let xdg_runtime_dir = std::env::var("XDG_RUNTIME_DIR").unwrap();
    let hyprland_signature = std::env::var("HYPRLAND_INSTANCE_SIGNATURE").unwrap();
    let socket_path = format!(
        "{}/hypr/{}/.socket.sock",
        xdg_runtime_dir, hyprland_signature
    );
    dbg!(&socket_path);
    let mut stream = UnixStream::connect(socket_path).unwrap();
    stream.write_all(b"/workspaces");
    let mut response = String::new();
    stream.read_to_string(&mut response);
    dbg!(response);
}
