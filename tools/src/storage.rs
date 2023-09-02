//! Storage interface.

use std::fmt::Display;
use std::io::{Read, Write};
use std::ops::Add;
use std::path::Path;
use std::{fs, io};

use bytes::BytesMut;
use serialport::SerialPort;

use crate::{
    proto::{
        gen::{pb, pb_storage},
        RpcSession,
    },
    serial::SerialCli,
};

const BUF_SIZE: usize = 1024;

/// Interface to Flipper device storage.
pub struct FlipperStorage {
    session: RpcSession,
}

impl FlipperStorage {
    /// Create new [`FlipperStorage`] connected to a [`SerialPort`].
    pub fn new(port: Box<dyn SerialPort>) -> io::Result<Self> {
        let mut cli = SerialCli::new(port);
        cli.start()?;

        Ok(Self {
            session: cli.start_rpc_session()?,
        })
    }

    /// Returns a mutable reference to the underlying [`RpcSession`].
    pub fn session_mut(&mut self) -> &mut RpcSession {
        &mut self.session
    }

    /// List files and directories on the device.
    pub fn list_tree(&mut self, path: &FlipperPath) -> io::Result<()> {
        // Note: The `storage list` command expects that paths do not end with a slash.
        let files = {
            let mut files = vec![];
            self.session.request_many(
                0,
                pb::main::Content::StorageListRequest(pb_storage::ListRequest {
                    path: path.to_string(),
                    include_md5: false,
                    filter_max_size: u32::MAX,
                }),
                |resp| {
                    Ok(match resp {
                        pb::main::Content::StorageListResponse(resp) => {
                            files.extend(resp.file);
                            Ok(())
                        }
                        r => Err(r),
                    })
                },
            )?;
            files
        };

        for f in files {
            match f.r#type() {
                pb_storage::file::FileType::Dir => {
                    let path = path.clone() + f.name.as_str();

                    eprintln!("{path}");
                    self.list_tree(&path)?;
                }
                pb_storage::file::FileType::File => {
                    let path = path.clone() + f.name.as_str();

                    eprintln!("{path}, size {}", f.size);
                }
            }
        }

        Ok(())
    }

    /// Send local file to the device.
    pub fn send_file(&mut self, from: impl AsRef<Path>, to: &FlipperPath) -> io::Result<()> {
        // Try to create directory on Flipper
        if let Some(dir) = to.0.rsplit_once('/') {
            self.mkdir(&FlipperPath::from(dir.0)).ok();
        }
        self.remove(to, false).ok();

        let mut file = pb_storage::File::default();
        file.set_type(pb_storage::file::FileType::File);

        fs::File::open(from.as_ref())?.read_to_end(&mut file.data)?;

        self.session.request(
            0,
            pb::main::Content::StorageWriteRequest(pb_storage::WriteRequest {
                path: to.to_string(),
                file: Some(file),
            }),
            |resp| match resp {
                pb::main::Content::Empty(_) => Ok(()),
                r => Err(r),
            },
        )
    }

    /// Receive remote file from the device.
    pub fn receive_file(&mut self, from: &FlipperPath, to: impl AsRef<Path>) -> io::Result<()> {
        let mut file = fs::File::options()
            .create(true)
            .write(true)
            .open(to.as_ref())?;

        let data = self.read_file(from)?;
        file.write_all(&data)?;

        Ok(())
    }

    /// Read file data from the device.
    pub fn read_file(&mut self, path: &FlipperPath) -> io::Result<BytesMut> {
        let mut data = BytesMut::with_capacity(BUF_SIZE);

        self.session.request_many(
            0,
            pb::main::Content::StorageReadRequest(pb_storage::ReadRequest {
                path: path.to_string(),
            }),
            |resp| {
                Ok(match resp {
                    pb::main::Content::StorageReadResponse(resp) => {
                        let file = resp.file.ok_or_else(|| {
                            io::Error::new(io::ErrorKind::Other, "file does not exist")
                        })?;
                        data.extend(file.data);
                        Ok(())
                    }
                    r => Err(r),
                })
            },
        )?;

        Ok(data)
    }

    /// Does the file or directory exist on the device?
    pub fn exist(&mut self, path: &FlipperPath) -> io::Result<bool> {
        self.stat(path).map(|f| f.is_some())
    }

    /// Does the directory exist on the device?
    pub fn exist_dir(&mut self, path: &FlipperPath) -> io::Result<bool> {
        self.stat(path).map(|stat| match stat {
            Some(f) => matches!(f.r#type(), pb_storage::file::FileType::Dir),
            None => false,
        })
    }

    /// Does the file exist on the device?
    pub fn exist_file(&mut self, path: &FlipperPath) -> io::Result<bool> {
        self.stat(path).map(|stat| match stat {
            Some(f) => matches!(f.r#type(), pb_storage::file::FileType::File),
            None => false,
        })
    }

    /// File size in bytes
    pub fn size(&mut self, path: &FlipperPath) -> io::Result<usize> {
        self.stat(path)?
            .map(|f| f.size as usize)
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "file does not exist"))
    }

    /// Stat a file or directory.
    fn stat(&mut self, path: &FlipperPath) -> io::Result<Option<pb_storage::File>> {
        self.session.request(
            0,
            pb::main::Content::StorageStatRequest(pb_storage::StatRequest {
                path: path.to_string(),
            }),
            |resp| match resp {
                pb::main::Content::StorageStatResponse(resp) => Ok(resp.file),
                r => Err(r),
            },
        )
    }

    /// Make directory on the device.
    pub fn mkdir(&mut self, path: &FlipperPath) -> io::Result<()> {
        self.session.request(
            0,
            pb::main::Content::StorageMkdirRequest(pb_storage::MkdirRequest {
                path: path.to_string(),
            }),
            |resp| match resp {
                pb::main::Content::Empty(_) => Ok(()),
                r => Err(r),
            },
        )
    }

    // /// Format external storage.
    // pub fn format_ext(&mut self) -> io::Result<()> {
    //     self.cli.send_and_wait_eol("storage format /ext")?;
    //     self.cli.send_and_wait_eol("y")?;
    //     self.cli.consume_response()?;

    //     Ok(())
    // }

    /// Remove file or directory.
    pub fn remove(&mut self, path: &FlipperPath, recursive: bool) -> io::Result<()> {
        self.session.request(
            0,
            pb::main::Content::StorageDeleteRequest(pb_storage::DeleteRequest {
                path: path.to_string(),
                recursive,
            }),
            |resp| match resp {
                pb::main::Content::Empty(_) => Ok(()),
                r => Err(r),
            },
        )
    }

    /// Calculate MD5 hash of file.
    pub fn md5sum(&mut self, path: &FlipperPath) -> io::Result<String> {
        self.session.request(
            0,
            pb::main::Content::StorageMd5sumRequest(pb_storage::Md5sumRequest {
                path: path.to_string(),
            }),
            |resp| match resp {
                pb::main::Content::StorageMd5sumResponse(resp) => Ok(resp.md5sum),
                r => Err(r),
            },
        )
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
            if !self.0.ends_with('/') {
                self.0 += "/";
            }
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
