//! Storage interface.

use std::fmt::Display;
use std::io;
use std::ops::Add;

use bytes::BytesMut;
use once_cell::sync::Lazy;
use regex::bytes::Regex;
use serialport::SerialPort;

const BUF_SIZE: usize = 1024;

static CLI_PROMPT: Lazy<Regex> = Lazy::new(|| Regex::new(r">: ").unwrap());
static CLI_EOL: Lazy<Regex> = Lazy::new(|| Regex::new(r"\r\n").unwrap());

/// Buffered reader for [`SerialPort`].
pub struct SerialReader {
    port: Box<dyn SerialPort>,
    buffer: BytesMut,
}

impl SerialReader {
    /// Create new [`SerialReader`] connected to a [`SerialPort`].
    pub fn new(port: Box<dyn SerialPort>) -> Self {
        Self { 
            port,
            buffer: BytesMut::with_capacity(BUF_SIZE),
        }
    }

    /// Get reference to underlying [`SerialPort`].
    pub fn get_ref(&self) -> &dyn SerialPort {
        self.port.as_ref()
    }

    /// Get mutable reference to underlying [`SerialPort`].
    pub fn get_mut(&mut self) -> &mut dyn SerialPort {
        self.port.as_mut()
    }

    /// Read from [`SerialPort`] until [`Regex`] is matched.
    pub fn read_until(&mut self, regex: &Regex, trim: bool) -> io::Result<BytesMut> {
        let mut buf = [0u8; BUF_SIZE];
        loop {
            if let Some(m) = regex.find(&self.buffer) {
                let start = m.start();
                let end = m.end();

                let mut data = self.buffer.split_to(end);
                if trim {
                    data.truncate(data.len() - (end - start));
                }

                return Ok(data);
            }

            // We always read at least 1 byte.
            let n = (self.port.bytes_to_read()? as usize).clamp(1, buf.len());

            self.port.read_exact(&mut buf[..n])?;
            self.buffer.extend_from_slice(&buf[..n]);
        }
    }
}

pub struct FlipperStorage {
    reader: SerialReader,
}

impl FlipperStorage {
    /// Create new [`FlipperStorage`] connected to a [`SerialPort`].
    pub fn new(port: Box<dyn SerialPort>) -> Self {
        Self {
            reader: SerialReader::new(port),
        }
    }

    /// Associated stream.
    pub fn port(&self) -> &dyn SerialPort {
        self.reader.get_ref()
    }
    
    pub fn port_mut(&mut self) -> &mut dyn SerialPort {
        self.reader.get_mut()
    }


    pub fn start(&mut self) -> serialport::Result<()> {
        self.port().clear(serialport::ClearBuffer::Input)?;

        // Send command with known syntax to make sure buffer is flushed
        self.send("device_info\r")?;
        self.reader.read_until(&Regex::new(r"hardware_model").unwrap(), true)?;

        // Read buffer until we get prompt
        self.reader.read_until(&*CLI_PROMPT, true)?;

        Ok(())
    }

    /// Send line to device.
    pub fn send(&mut self, line: &str) -> io::Result<()> {
        write!(self.port_mut(), "{line}")
    }

    /// Send line to device and wait for next end-of-line.
    pub fn send_and_wait_eol(&mut self, line: &str) -> io::Result<BytesMut> {
        self.send(line)?;

        self.reader.read_until(&*CLI_EOL, true)
    }

    /// Send line to device and wait for next CLI prompt.
    pub fn send_and_wait_prompt(&mut self, line: &str) -> io::Result<BytesMut> {
        self.send(line)?;

        self.reader.read_until(&*CLI_PROMPT, true)
    }

    /// Extract error text.
    fn get_error(data: &str) -> Option<&str> {
        let (_, text) = data.split_once("Storage error: ")?;

        Some(text.trim())
    }

    /// List files and directories on the device.
    pub fn list_tree(&mut self, path: FlipperPath) -> io::Result<()> {
        // Note: The `storage list` command expects that paths do not end with a slash.
        self.send_and_wait_eol(&format!("storage list {}\r", path))?;

        let data = self.reader.read_until(&*CLI_PROMPT, true)?;
        for line in CLI_EOL.split(&data).map(|line| String::from_utf8_lossy(line)) {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            if let Some(error) = Self::get_error(line) {
                eprintln!("ERROR: {error}");
                continue;
            }

            if line == "Empty" {
                continue;
            }

            if let Some((typ, info)) = line.split_once(" ") {
                match typ {
                    // Directory
                    "[D]" => {
                        let path = path.clone() + info;

                        eprintln!("{path}");
                        self.list_tree(path)?;
                    },
                    // File
                    "[F]" => {
                        if let Some((name, size)) = info.rsplit_once(" ") {
                            let path = path.clone() + name;

                            eprintln!("{path}, size {size}");
                        }
                    },
                    // We got something unexpected, ignore it
                    _ => (),
                }
            }
        }

        Ok(())
    }
}

/// A path on the Flipper device.
/// 
/// [`FlipperPath`] maintains certain invariants:
/// - Paths are valid UTF-8
/// - Paths are always absolute (start with `/`)
/// - Paths do not end in a `/`
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FlipperPath(String);

impl FlipperPath {
    /// Create a new [`FlipperPath`].
    pub fn new() -> Self {
        Self(String::from("/"))
    }

    /// Push a path fragment to this path
    pub fn push(&mut self, path: &str) {
        let path = path.trim_end_matches('/');
        if path.starts_with('/') {
            // Absolute path
            self.0 = String::from(path);
        } else {
            // Relative path
            self.0 += "/";
            self.0 += path;
        }
    }
}

impl Default for FlipperPath {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for FlipperPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl AsRef<str> for FlipperPath {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl From<String> for FlipperPath {
    fn from(mut value: String) -> Self {
        if let Some(p) = value.rfind(|c| c != '/') {
            // Drop any trailing `/`
            value.truncate(p + 1);
        }

        if !value.starts_with('/') {
            // Make path absolute
            let mut path = Self::new();
            path.0.extend([value]);

            path
        } else {
            Self(value)
        }
    }
}

impl From<&str> for FlipperPath {
    fn from(value: &str) -> Self {
        FlipperPath::from(value.to_string())
    }
}

impl Add<&str> for FlipperPath {
    type Output = Self;

    fn add(mut self, rhs: &str) -> Self::Output {
        self.push(rhs);

        self
    }
}
