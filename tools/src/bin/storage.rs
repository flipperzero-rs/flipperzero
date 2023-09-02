//! Storage CLI.
//!
//! See: [`storage.py`][1] script.
//!
//! [1]: https://github.com/flipperdevices/flipperzero-firmware/blob/dev/scripts/storage.py

use std::path::PathBuf;
use std::process;
use std::time::Duration;

use clap::{Parser, Subcommand};
use flipperzero_tools::storage::FlipperPath;
use flipperzero_tools::{serial, storage};

/// Flipper Zero storage tool
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Serial port (e.g. `COM3` on Windows or `/dev/ttyUSB0` on Linux)
    #[arg(short, long)]
    port: Option<String>,
    /// Commands
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Create directory
    Mkdir {
        /// Flipper path
        flipper_path: FlipperPath,
    },
    // /// Format flash card
    // Format,
    // /// Remove file/directory
    Remove {
        /// Flipper path
        flipper_path: FlipperPath,
    },
    /// Read file
    Read,
    /// Print size of file (in bytes)
    Size {
        /// Flipper path
        flipper_path: FlipperPath,
    },
    /// Receive file
    Receive {
        /// Flipper path
        flipper_path: FlipperPath,
        /// Local path
        local_path: PathBuf,
    },
    /// Send file or directory
    Send {
        /// Local path
        local_path: PathBuf,
        /// Flipper path
        flipper_path: FlipperPath,
    },
    /// Recursively list files and dirs
    List {
        /// Flipper path
        #[arg(default_value = "/")]
        flipper_path: FlipperPath,
    },
    /// Calculate MD5 hash of remote file
    Md5sum {
        /// Flipper path
        flipper_path: FlipperPath,
    },
}

fn main() {
    let cli = Cli::parse();

    let command = match &cli.command {
        None => {
            eprintln!("No subcommand specified");
            process::exit(2);
        }
        Some(c) => c,
    };

    let port_info =
        serial::find_flipperzero(cli.port.as_deref()).expect("unable to find Flipper Zero");
    let port = serialport::new(port_info.port_name, serial::BAUD_115200)
        .timeout(Duration::from_secs(30))
        .open()
        .expect("unable to open serial port");

    let mut store = storage::FlipperStorage::new(port).unwrap();

    let result = match command {
        Commands::Mkdir { flipper_path } => store.mkdir(flipper_path),
        // Commands::Format => store.format_ext(),
        Commands::Remove { flipper_path } => store.remove(flipper_path, true),
        Commands::Read => todo!(),
        Commands::Size { flipper_path } => match store.size(flipper_path) {
            Err(err) => Err(err),
            Ok(size) => {
                println!("{size}");

                Ok(())
            }
        },
        Commands::Receive {
            flipper_path,
            local_path,
        } => store.receive_file(flipper_path, local_path),
        Commands::Send {
            local_path,
            flipper_path,
        } => store.send_file(local_path.as_path(), flipper_path),
        Commands::List { flipper_path } => store.list_tree(flipper_path),
        Commands::Md5sum { flipper_path } => match store.md5sum(flipper_path) {
            Err(err) => Err(err),
            Ok(md5sum) => {
                println!("{md5sum}");

                Ok(())
            }
        },
    };

    if let Err(err) = result {
        eprintln!("ERROR: {err}");
        process::exit(1);
    }
}
