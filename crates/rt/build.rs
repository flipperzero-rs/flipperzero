//! Build script for Flipper Zero Rust runtime.

use std::{env, error::Error, fs::File, io::Write, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    // build directory for this crate
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // extend the library search path
    println!("cargo:rustc-link-search={}", out_dir.display());

    // put linker script in build directory
    File::create(out_dir.join("flipperzero-rt.ld"))?
        .write_all(include_bytes!("flipperzero-rt.ld"))?;

    Ok(())
}
