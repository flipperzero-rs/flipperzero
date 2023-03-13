let
  here = toString ./.;
  moz_overlay = import (builtins.fetchTarball
    "https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz");
  pkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
  rust = (pkgs.rustChannelOf {
    channel = "nightly";
  }).rust.override {
    extensions = [ "rust-src" "rust-analysis" ];
    targets = [ "thumbv7em-none-eabihf" ];
  };
  rustPlatform = pkgs.makeRustPlatform {
    rustc = rust;
    cargo = rust;
  };



  my-python-packages = p: with p; [
    pyserial
  ];

  systemDeps = with pkgs; [
    udev
  ];
  systemLibStr = pkgs.lib.makeLibraryPath systemDeps;

in pkgs.mkShell {
  packages = [
    (pkgs.python3.withPackages my-python-packages)
  ];

  buildInputs = [
    rust
    pkgs.gcc-arm-embedded
    pkgs.python3
  ] ++ systemDeps;

  nativeBuildInputs = [
    pkgs.pkg-config
  ];

  LIBCLANG_PATH = "${pkgs.llvmPackages.libclang}/lib";
  LD_LIBRARY_PATH = "${pkgs.stdenv.cc.cc.lib}/lib64:$LD_LIBRARY_PATH:${systemLibStr}";
  CARGO_INCREMENTAL = 1;
}
