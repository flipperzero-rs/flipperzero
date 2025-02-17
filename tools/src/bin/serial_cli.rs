//! Serial client tool for Flipper Zero.

use std::io::{self, Write};
use std::process::ExitCode;
use std::time::Duration;

use clap::Parser;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::style::Stylize;
use flipperzero_tools::serial;

const ETXT: char = '\x03'; // ^C
const DEL: char = '\x7f';

/// Flipper Zero serial client
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Serial port (e.g. `COM3` on Windows or `/dev/ttyUSB0` on Linux)
    #[arg(short, long)]
    port: Option<String>,
}

fn main() -> ExitCode {
    // Enable ANSI support on Windows
    #[cfg(windows)]
    let _ = crossterm::ansi_support::supports_ansi();

    let cli = Cli::parse();

    let port_info = match serial::find_flipperzero(cli.port.as_deref()) {
        Some(p) => p,
        None => {
            eprintln!("{}: unable to find Flipper Zero", "error".red());
            return ExitCode::FAILURE;
        }
    };

    let mut port = serialport::new(port_info.port_name, serial::BAUD_115200)
        .timeout(Duration::from_millis(10))
        .open()
        .expect("unable to open serial port");

    port.clear(serialport::ClearBuffer::All).unwrap();

    eprintln!("🐬 {}", "Press `Ctrl+]` to exit".cyan());

    crossterm::terminal::enable_raw_mode().unwrap();
    port.write_data_terminal_ready(true).unwrap();

    let mut exit_code = ExitCode::SUCCESS;
    if let Err(err) = run(port.as_mut()) {
        eprintln!();
        eprintln!("{}: {err}", "error".red());
        exit_code = ExitCode::FAILURE;
    }

    crossterm::terminal::disable_raw_mode().unwrap();
    port.write_data_terminal_ready(false).ok();

    exit_code
}

fn run(port: &mut dyn serialport::SerialPort) -> io::Result<()> {
    let mut stdout = io::stdout();
    let mut buf = [0u8; 1024];

    loop {
        match port.read(&mut buf) {
            Err(err) => {
                if err.kind() != io::ErrorKind::TimedOut {
                    return Err(err);
                }
            }
            Ok(count) => {
                print!("{}", String::from_utf8_lossy(&buf[..count]));
                stdout.flush()?;
            }
        }

        while crossterm::event::poll(Duration::ZERO)? {
            let event = crossterm::event::read()?;
            if let Event::Key(KeyEvent {
                code,
                modifiers,
                kind,
                ..
            }) = event
            {
                if kind == KeyEventKind::Release {
                    continue;
                }

                match (modifiers, code) {
                    // MacOS converts Ctrl+] to Ctrl+5
                    (KeyModifiers::CONTROL, KeyCode::Char(']') | KeyCode::Char('5')) => {
                        eprintln!();
                        eprintln!("--- exit ---");
                        return Ok(());
                    }
                    (KeyModifiers::CONTROL, KeyCode::Char('c')) => write!(port, "{ETXT}")?,
                    (KeyModifiers::SHIFT, KeyCode::Char(c)) => {
                        write!(port, "{c}")?;
                    }
                    (KeyModifiers::NONE, KeyCode::Char(c)) => {
                        write!(port, "{c}")?;
                    }
                    (KeyModifiers::NONE, KeyCode::Backspace) => {
                        write!(port, "{DEL}")?;
                    }
                    (KeyModifiers::NONE, KeyCode::Enter) => {
                        write!(port, "\r\n")?;
                    }
                    _ => (),
                }
            }

            port.flush()?;
        }
    }
}
