# Hello, Rust!

Prints "Hello, Rust!" to the console and then exits.

## Usage

```
cargo build --release

```
# "Hello, Rust!" for the Flipper Zero

This is an example of how to build a Rust-based Flipper application that runs
from the SD-card.

**Note:** This depends upon the Flipper Application SDK which is included in
the `0.67` release and some Rust-specific fixes which are included in `0.68.1` release.

## Building

1. Switch to nightly version of Rust:
    ```
    rustup default nightly
    ```
2. Install the `thumbv7em-none-eabihf` Rust target:
    ```
    rustup target add thumbv7em-none-eabihf
    ```
3. Build the application:
    ```
    cargo build --release
    ```
4. Copy to the Flipper Zero as a `.fap`:
    This can either be done using [qFlipper](https://flipperzero.one/update) or the [`flipperzero-firmware`](https://github.com/flipperdevices/flipperzero-firmware) scripts:
    ```
    ${FLIPPERZERO_FIRMWARE}/scripts/storage.py mkdir /ext/apps/Misc
    ${FLIPPERZERO_FIRMWARE}/scripts/storage.py send target/thumbv7em-none-eabihf/release/hello-rust /ext/apps/Misc/hello-rust.fap
    ```

After that you can launch the app on Flipper via `Menu → Applications → Misc → Hello, Rust!`.

## License

Licensed under the MIT License. See [LICENSE](LICENSE) for details.
