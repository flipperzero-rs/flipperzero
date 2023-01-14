# Notification example

Turns the notification LED red, green, blue and then play the "success" notification.

**Note:** this currently uses the low-level [`flipperzero-sys`](https://crates.io/crates/flipperzero-sys) crate as
notifications aren't currently exposed in the [`flipperzero`](https://crates.io/crates/flipperzero) crate.

## Usage

```
cargo build --release
```

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
    ${FLIPPERZERO_FIRMWARE}/scripts/storage.py mkdir /ext/apps/Examples
    ${FLIPPERZERO_FIRMWARE}/scripts/storage.py send target/thumbv7em-none-eabihf/release/notification.fap /ext/apps/Examples/notification.fap
    ```

After that you can launch the app on Flipper via `Menu → Applications → Examples → Rust notification example`.

## License

Licensed under the MIT License. See [LICENSE](LICENSE) for details.
