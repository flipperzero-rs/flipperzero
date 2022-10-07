# `flipperzero-sys` bindings

This is a set of general guidelines when adding new bindings to the `flipperzero-sys` crate. It's highly recommended to read through the [FFI section of the Rustonomicon](https://doc.rust-lang.org/nomicon/ffi.html) as there's a lot of gotchas when writing bindings.

## Module structure should mirror header include path

For example:
- `#include <furi/core/thread.h>` → `furi::core::thread`
- `#include <toolbox/protocols/protocol_dict.h>` → `toolbox::protocols::dict`
- `#include <gui/canvas.h>` → `gui::canvas`

When in doubt, try to match the structure of other modules.

## Strip symbol prefix from Rust identifier

For example, in the `furi::thread` module:
```rust
#[link_name = "furi_thread_get_current_id"]
pub fn get_current_id() -> *const FuriThreadId;
```

When using [`api-symbols.py`](../tools/api-symbols.py) you can use a combination of the `--match-prefix <prefix>` and `--strip-prefix` flags to generate a Rust definition with the prefix automatically removed.

## Use `opaque!` macro for opaque types

It's common for APIs to use a `OpaqueContext *` to hide the internal implementation of a struct.

The `flippperzero_sys::opaque!` macro can be used to define an opaque `struct` that ensures the compiler does not mark the struct as `Send`, `Sync` or `Unpin`.

## Enumerations

If there's any chance that an API may return new enumeration values then Rust's enum type can't be used. Unlike enums in C, it's not possible to cast an unknown integer to an enum type. Doing so is undefined behaviour.

Instead use a [new-type](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) struct of the matching type of the enum (typically `i32`) and create constants for the known values:

```rust
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FuriHalRtcBootMode(u32);

impl FuriHalRtcBootMode {
    /// Normal boot mode, default value.
    pub const NORMAL: FuriHalRtcBootMode = Self(0);
    /// Boot to DFU (MCU bootloader by ST).
    pub const DFU: FuriHalRtcBootMode = Self(1);
    // ...
}
```

If an enum is used solely as an input parameter, then Rust enums are fine to use:

```rust
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FuriHalCryptoKeySize {
    KeySize128,
    KeySize256,
}
```
