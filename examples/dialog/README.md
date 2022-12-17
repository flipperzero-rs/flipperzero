# Rust dialog example.

Demonstrates the high-level bindings to the Dialog API.

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
    ${FLIPPERZERO_FIRMWARE}/scripts/storage.py send target/thumbv7em-none-eabihf/release/dialog /ext/apps/Misc/dialog.fap
    ```

After that you can launch the app on Flipper via `Menu → Applications → Misc → Rust dialog example`.

## License

Licensed under the MIT License. See [LICENSE](LICENSE) for details.
