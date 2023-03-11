# Examples

This directory contains different examples of using `flipperzero` crate.

## Running

Each project can be built by either using

```shell
$ cargo build --release --project ${project}
```

or

```shell
$ cd ${project}
$ $ cargo build --release
```

Where `${project}` is the name of the [project](#Projects) being buit.

Note that building all projects at a time is currently not possible due to a bug/limitation of Cargo
which causes features unification to happen between crates
thus making all examples require dependency on `alloc` feature of `flipperzero`
leading to them failing due to no allocator specified.
See rust-lang/cargo#4463 for more details.

## Requirements

Example crates require nightly Rust with target set to `thumbv7em-none-eabihf` to be built.
This is set via [`rust-toolchain.toml`](./rust-toolchain.toml) configuration file.

Specific compiler flags are also recommended and set via [`.cargo/config.toml`](./.cargo/config.toml) configuration file.

## Projects

The following example projects are available:

### [`hello-rust`](./hello-rust)

Basic example demonstrating the use of `flipperzero`.

### [`dialog`](./dialog)

Example of using dialog API.

This also shows how to enable and use optional `alloc` feature required for it.

### [`gpio`](./gpio)

This example demonstrates basic GPIO manipulation.

### [`gui`](./gui)

This example shows the basics of creating a user interface with  GUI API.

### [`notification`](./notification)

An example demonstrating the use of the API to display notifications to the user.
