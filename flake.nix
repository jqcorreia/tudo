{
  description = "TUDO Dev Environment";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs =
    { self, nixpkgs }:
    let
      pkgs = nixpkgs.legacyPackages.x86_64-linux;
    in
    {
      devShells.x86_64-linux.default = pkgs.mkShell {
        packages = with pkgs; [
          cargo
          rust-analyzer
          SDL2
          SDL2_ttf
          SDL2_image
          pkg-config
          dbus
        ];
        shellHook = "zsh";
        name = "Backstage dev shell";
      };
    };
}
