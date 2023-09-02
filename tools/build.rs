use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

macro_rules! flipper_proto {
    ($name:literal) => {
        concat!("flipperzero-protobuf/", $name, ".proto")
    };
}

macro_rules! stage_generated_proto {
    ($from:ident, $name:literal) => {
        fs::copy($from.join($name), concat!("src/proto/gen/", $name))
    };
}

const FLIPPER_PROTO_DIR: &str = "flipperzero-protobuf/";
const FLIPPER_PROTO: &str = flipper_proto!("flipper");

fn main() -> io::Result<()> {
    // - We don't include the proto files in the repository so that downstreams do not
    //   need to regenerate the bindings even if protoc is present.
    // - We check for the existence of protoc in the same way as prost-build, so that
    //   people building from source do not need to have protoc installed.
    if Path::new(FLIPPER_PROTO).exists()
        && env::var_os("PROTOC")
            .map(PathBuf::from)
            .or_else(|| which::which("protoc").ok())
            .is_some()
    {
        gen_protobufs()?;
    }

    Ok(())
}

fn gen_protobufs() -> io::Result<()> {
    let out: PathBuf = env::var_os("OUT_DIR")
        .expect("Cannot find OUT_DIR environment variable")
        .into();

    // Build the compact format types.
    prost_build::compile_protos(
        &[
            FLIPPER_PROTO,
            flipper_proto!("application"),
            flipper_proto!("desktop"),
            flipper_proto!("gpio"),
            flipper_proto!("gui"),
            flipper_proto!("property"),
            flipper_proto!("storage"),
            flipper_proto!("system"),
        ],
        &[FLIPPER_PROTO_DIR],
    )?;

    // Copy the generated types into the source tree so changes can be committed.
    stage_generated_proto!(out, "pb.rs")?;
    stage_generated_proto!(out, "pb_app.rs")?;
    stage_generated_proto!(out, "pb_desktop.rs")?;
    stage_generated_proto!(out, "pb_gpio.rs")?;
    stage_generated_proto!(out, "pb_gui.rs")?;
    stage_generated_proto!(out, "pb_property.rs")?;
    stage_generated_proto!(out, "pb_storage.rs")?;
    stage_generated_proto!(out, "pb_system.rs")?;

    Ok(())
}
