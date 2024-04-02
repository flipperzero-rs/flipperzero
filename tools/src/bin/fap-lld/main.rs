use std::{
    env, fmt, io,
    path::PathBuf,
    process::{self, Command},
};

use elf::ParseError;
use which::which;

mod fastrel;

#[derive(Debug)]
enum Error {
    Io(io::Error),
    Parse(ParseError),
    NoSymbolTable,
    NoSectionHeaders,
    ObjcopyFailed,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "IO error: {}", e),
            Error::Parse(e) => write!(f, "Error while parsing ELF: {}", e),
            Error::NoSymbolTable => write!(f, "No symbol table in ELF"),
            Error::NoSectionHeaders => write!(f, "No section headers in ELF"),
            Error::ObjcopyFailed => write!(f, "Error while running objcopy"),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<ParseError> for Error {
    fn from(e: ParseError) -> Self {
        Error::Parse(e)
    }
}

fn main() -> Result<(), Error> {
    // Run the real linker with the given arguments.
    let res = Command::new("rust-lld")
        .args(env::args_os().skip(1))
        .status()?;
    if !res.success() {
        process::exit(res.code().unwrap_or(-1));
    }

    // If we don't have objcopy available, skip post-linking optimizations.
    if let Ok(objcopy) = which("llvm-objcopy") {
        // Parse the arguments to find the path to the linked binary.
        let output_fap = PathBuf::from(env::args_os().skip_while(|a| a != "-o").nth(1).unwrap());

        // Add `.fast.rel` sections.
        fastrel::postprocess_fap(&output_fap, &objcopy)?;
    } else {
        println!("Cannot find llvm-objcopy, skipping post-linker optimizations.");
        println!("Please install the llvm-tools for your Rust compiler. For example:");
        println!("    rustup component add llvm-tools");
    }

    Ok(())
}
