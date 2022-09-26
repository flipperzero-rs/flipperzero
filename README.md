# Rust on the Flipper Zero

This project provides support for running Rust-based applications on the [Flipper Zero](https://flipperzero.one/).

## Initial setup

1. Install [`rustup`](https://rust-lang.github.io/rustup/) by following the instructions on [`rustup.rs`](https://rustup.rs/).
2. Use `rustup` to install the `thumbv7em-none-eabihf` target:
    ```
    rustup target add thumbv7em-none-eabihf
    ```
3. Clone the [`flipperzero-firmware`](https://github.com/flipperdevices/flipperzero-firmware) repository:
    ```
    git clone --recurse-submodules https://github.com/flipperdevices/flipperzero-firmware.git && cd flipperzero-firmware
    ```

## Writing an external app

The Flipper Zero supports installing [externally built applications on the SD card](https://github.com/flipperdevices/flipperzero-firmware/blob/dev/documentation/AppsOnSDCard.md).

For the moment, [flipperzero-hello-rust](https://github.com/dcoles/flipperzero-hello-rust) is the best example on how to do so.

## License

Licensed under the MIT License. See LICENSE for details.
