//! Serial client tool for Flipper Zero.

use std::io::{self, Write};
use std::time::Duration;

use crossterm::event::{Event, KeyEvent, KeyModifiers, KeyCode, KeyEventKind};
use flipperzero_tools::serial;

const DEL: char = '\x7f';

fn main() -> io::Result<()> {
    // Enable ANSI support on Windows
    #[cfg(windows)]
    let _ = crossterm::ansi_support::supports_ansi();

    let port_info = serial::find_flipperzero().expect("unable to find Flipper Zero");
    let mut port = serialport::new(&port_info.port_name, serial::BAUD_115200)
        .timeout(Duration::from_millis(10))
        .open()
        .expect("unable to open serial port");
    
    port.clear(serialport::ClearBuffer::All).unwrap();

    eprintln!("⭐ Press `Ctrl+]` to exit");

    crossterm::terminal::enable_raw_mode()?;
    port.write_data_terminal_ready(true)?;

    if let Err(err) = run(port.as_mut()) {
        eprintln!("ERROR: {}", err);
    }

    crossterm::terminal::disable_raw_mode()?;
    port.write_data_terminal_ready(false)?;

    Ok(())
}

fn run(port: &mut dyn serialport::SerialPort) -> io::Result<()> {
    let mut stdout = io::stdout();
    let mut buf = [0u8; 1024];

    loop {
        match port.read(&mut buf) {
            Err(err) => {
                if err.kind() != io::ErrorKind::TimedOut {
                    return Err(err.into());
                }
            },
            Ok(count) => {
                print!("{}", String::from_utf8_lossy(&buf[..count]));
                stdout.flush()?;
            }
        }

        while crossterm::event::poll(Duration::ZERO)? {
            let event = crossterm::event::read()?;
            match event {
                Event::Key(KeyEvent { code, modifiers, kind, .. }) => {
                    if kind == KeyEventKind::Release {
                        continue;
                    }

                    match (modifiers, code) {
                        (KeyModifiers::CONTROL, KeyCode::Char(']')) => {
                            eprintln!("Exiting...");
                            return Ok(());
                        },
                        (KeyModifiers::NONE, KeyCode::Char(c)) => {
                            write!(port, "{c}")?;
                        }
                        (KeyModifiers::NONE, KeyCode::Backspace) => {
                            write!(port, "{DEL}")?;
                        },
                        (KeyModifiers::NONE, KeyCode::Enter) => {
                            write!(port, "\r\n")?;
                        },
                        _ => (),
                    }
                },
                _ => (),
            };

            port.flush()?;
        }
    }
}
