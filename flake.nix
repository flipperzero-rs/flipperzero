{
  description = "Rust on the Flipper Zero";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs =
    {
      nixpkgs,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };
        rust = pkgs.rust-bin.fromRustupToolchainFile ./crates/rust-toolchain.toml;
      in
      {
        formatter = pkgs.nixfmt-rfc-style;
        devShells = {
          default = pkgs.mkShell {
            nativeBuildInputs = [
              rust
              pkgs.python3
              pkgs.pkg-config
              pkgs.systemd
            ];
          };
          github-actions = pkgs.mkShell {
            nativeBuildInputs = [
              pkgs.act
              pkgs.actionlint
              pkgs.pinact
            ];
          };
        };
      }
    );
}
