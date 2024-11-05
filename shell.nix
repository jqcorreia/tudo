{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell {
  packages = with pkgs; [
    SDL2
    SDL2_ttf
    SDL2_image
  ];
}
