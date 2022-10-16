# Rust for Flipper Zero 🐬❤️🦀

[![crates.io](https://img.shields.io/crates/v/flipperzero)](https://crates.io/crates/flipperzero)
[![docs.rs](https://img.shields.io/docsrs/flipperzero)](https://docs.rs/flipperzero)
[![MIT license](https://img.shields.io/crates/l/flipperzero)](LICENSE)

Rust application support for the [Flipper Zero](https://flipperzero.one/).

## Initial setup

1. Install [`rustup`](https://rust-lang.github.io/rustup/) by following the instructions on [`rustup.rs`](https://rustup.rs/).
2. Use `rustup` to install the `thumbv7em-none-eabihf` target:
    ```
    rustup target add thumbv7em-none-eabihf
    ```

## Writing an external app

The Flipper Zero supports installing [externally built applications on the SD card](https://github.com/flipperdevices/flipperzero-firmware/blob/dev/documentation/AppsOnSDCard.md).

See [`examples/hello-rust`](examples/hello-rust) for an example application.

## License

Licensed under the MIT License. See LICENSE for details.
