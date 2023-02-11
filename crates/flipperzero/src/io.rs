use core::ffi::CStr;
use core::fmt;

use flipperzero_sys as sys;

/// Stream and file system related error type
#[derive(Debug, Clone, Copy)]
pub enum Error {
    Ok,
    NotReady,
    Exists,
    NotExists,
    InvalidParameter,
    Denied,
    InvalidName,
    Internal,
    NotImplemented,
    AlreadyOpen,
}

impl Error {
    pub fn to_sys(&self) -> sys::FS_Error {
        match self {
            Self::Ok => sys::FS_Error_FSE_OK,
            Self::NotReady => sys::FS_Error_FSE_NOT_READY,
            Self::Exists => sys::FS_Error_FSE_EXIST,
            Self::NotExists => sys::FS_Error_FSE_NOT_EXIST,
            Self::InvalidParameter => sys::FS_Error_FSE_INVALID_PARAMETER,
            Self::Denied => sys::FS_Error_FSE_DENIED,
            Self::InvalidName => sys::FS_Error_FSE_INVALID_NAME,
            Self::Internal => sys::FS_Error_FSE_INTERNAL,
            Self::NotImplemented => sys::FS_Error_FSE_NOT_IMPLEMENTED,
            Self::AlreadyOpen => sys::FS_Error_FSE_ALREADY_OPEN,
        }
    }

    pub fn from_sys(err: sys::FS_Error) -> Self {
        match err {
            sys::FS_Error_FSE_OK => Self::Ok,
            sys::FS_Error_FSE_NOT_READY => Self::NotReady,
            sys::FS_Error_FSE_EXIST => Self::Exists,
            sys::FS_Error_FSE_NOT_EXIST => Self::NotExists,
            sys::FS_Error_FSE_INVALID_PARAMETER => Self::InvalidParameter,
            sys::FS_Error_FSE_DENIED => Self::Denied,
            sys::FS_Error_FSE_INVALID_NAME => Self::InvalidName,
            sys::FS_Error_FSE_INTERNAL => Self::Internal,
            sys::FS_Error_FSE_NOT_IMPLEMENTED => Self::NotImplemented,
            sys::FS_Error_FSE_ALREADY_OPEN => Self::AlreadyOpen,
            _ => unimplemented!(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = unsafe { CStr::from_ptr(sys::filesystem_api_error_get_desc(self.to_sys())) };
        write!(f, "{}", msg.to_bytes().escape_ascii())
    }
}

/// Trait comparable to `std::Read` for the Flipper stream API
pub trait Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error>;
}

/// Trait comparable to `std::Seek` for the Flipper stream API
pub trait Seek {
    fn seek(&mut self, pos: SeekFrom) -> Result<usize, Error>;

    fn rewind(&mut self) -> Result<(), Error> {
        self.seek(SeekFrom::Start(0))?;
        Ok(())
    }

    fn stream_len(&mut self) -> Result<usize, Error> {
        let old_pos = self.stream_position()?;
        let len = self.seek(SeekFrom::End(0))?;

        // Avoid seeking a third time when we were already at the end of the
        // stream. The branch is usually way cheaper than a seek operation.
        if old_pos != len {
            self.seek(SeekFrom::Start(
                old_pos.try_into().map_err(|_| Error::InvalidParameter)?,
            ))?;
        }

        Ok(len)
    }

    fn stream_position(&mut self) -> Result<usize, Error> {
        self.seek(SeekFrom::Current(0))
    }
}

/// Trait comparable to `std::Write` for the Flipper stream API
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn flush(&mut self) -> Result<(), Error>;

    fn write_all(&mut self, mut buf: &[u8]) -> Result<(), Error> {
        while !buf.is_empty() {
            match self.write(buf) {
                Ok(0) => {
                    // TODO
                }
                Ok(n) => buf = &buf[n..],
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}

/// Enumeration of possible methods to seek within an I/O object.
///
/// It is used by the Seek trait.
pub enum SeekFrom {
    Start(u64),
    End(i64),
    Current(i64),
}
