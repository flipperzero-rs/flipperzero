# Updating the supported SDK

Currently [`flipperzero-sys`] bundles the SDK bindings with each release.

This is convenient for users in that it doesn't require any external dependency on
[`flipperzero-firmware`] or compiler toolchain, however it means that each
[`flipperzero-sys`] release is tightly bound to a specific SDK version and thus can only be used
with a specific range of firmware releases.

A better approach would be to build the bindings using a `build.rs` script.

By default this would fetch the currently "supported" SDK and toolchain and use these to generate
bindings. However it should be possible to either override the SDK and toolchain version downloaded
or even point to a local `flipperzero-firmware` checkout.

## Current process

Prerequisites: [A recent version of libclang installed](https://rust-lang.github.io/rust-bindgen/requirements.html)

To update the SDK you require a checkout of [`flipperzero-firmware`] pointing at the target
commit/tag and need to run `./fbt` to download the toolchain and build a local copy of the SDK.
Alternatively you can download a prebuilt SDK from the [Flipper Zero Update Server](https://update.flipperzero.one/builds/firmware/).

Once the SDK is built, run the [`generate-bindings`] script to build a new [`bindings.rs`]:

```bash
$ cd tools/
$ cargo run --bin generate-bindings ../../flipperzero-firmware/build/f7-firmware-D/sdk_headers
$ cp bindings.rs ../crates/sys/src
```

Make sure to update the SDK details in [`README.md`] before making a new release.

Alternatively, you can generate `binding.rs` in an isolated env using Docker and the following command:

From the root of the repository, to build the binding for the branch/tag `0.101.2` of the official SDK:

```shell
docker run --rm $(docker build --build-arg BRANCH=0.101.2 -q -f tools/Dockerfile .) > crates/sys/src/bindings.rs
```

[`bindings.rs`]: ../crates/sys/src/bindings.rs
[`flipperzero-firmware`]: https://github.com/flipperdevices/flipperzero-firmware
[`flipperzero-sys`]: https://crates.io/crates/flipperzero-sys
[`generate-bindings`]: ../tools/src/bin/generate-bindings.rs
[`README.md`]: ../README.md
