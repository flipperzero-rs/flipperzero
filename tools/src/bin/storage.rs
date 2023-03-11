//! Storage CLI.

use std::io;
use std::time::Duration;

use clap::{Parser, Subcommand};
use flipperzero_tools::storage::FlipperPath;
use flipperzero_tools::{serial, storage};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    /// Create directory
    Mkdir,
    /// Format flash card
    Format,
    /// Remove file/directory
    Remove,
    /// Read file
    Read,
    /// Print size of file
    Size,
    /// Receive file
    Receive,
    /// Send file or directory
    Send,
    /// Recursively list files and dirs
    List {
        /// Flipper path
        #[arg(default_value = "/")]
        flipper_path: String,
    },
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let command = match &cli.command {
        None => {
            eprintln!("No subcommand specified");
            return Ok(());
        },
        Some(c) => c,
    };

    let port_info = serial::find_flipperzero().expect("unable to find Flipper Zero");
    let mut port = serialport::new(&port_info.port_name, serial::BAUD_115200)
        .timeout(Duration::from_secs(30))
        .open()
        .expect("unable to open serial port");

    port.clear(serialport::ClearBuffer::All).unwrap();
    port.write_data_terminal_ready(true)?;

    let mut store = storage::FlipperStorage::new(port);
    store.start()?;

    match command {
        Commands::Mkdir => todo!(),
        Commands::Format => todo!(),
        Commands::Remove => todo!(),
        Commands::Read => todo!(),
        Commands::Size => todo!(),
        Commands::Receive => todo!(),
        Commands::Send => todo!(),
        Commands::List { flipper_path } => {
            let flipper_path = FlipperPath::from(flipper_path.as_str());
            store.list_tree(flipper_path)?;
        },
    }

    Ok(())
}
