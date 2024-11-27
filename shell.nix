{
  pkgs ? import <nixpkgs> { },
}:
pkgs.mkShell {
  packages = with pkgs; [
    cargo
    rust-analyzer
    SDL2
    SDL2_ttf
    SDL2_image
    pkg-config
    dbus
  ];
}
