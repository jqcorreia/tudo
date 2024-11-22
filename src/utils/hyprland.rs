use std::io::prelude::*;
use std::os::unix::net::UnixStream;

pub fn open_hyprland_socket_1() -> UnixStream {
    let xdg_runtime_dir = std::env::var("XDG_RUNTIME_DIR").unwrap();
    let hyprland_signature = std::env::var("HYPRLAND_INSTANCE_SIGNATURE").unwrap();
    let socket_path = format!(
        "{}/hypr/{}/.socket.sock",
        xdg_runtime_dir, hyprland_signature
    );
    dbg!(&socket_path);
    let stream = UnixStream::connect(socket_path).unwrap();
    return stream;
}
