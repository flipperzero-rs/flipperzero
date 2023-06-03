use core::ffi::CStr;
use core::fmt;

use flipperzero_sys as sys;

/// Stream and file system related error kinds.
///
/// This list may grow over time, and it is not recommended to exhaustively
/// match against it.
///
/// # Handling errors and matching on `Error`
///
/// In application code, use `match` for the `Error` values you are expecting;
/// use `_` to match "all other errors".
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum Error {
    NotReady,
    Exists,
    NotExists,
    InvalidParameter,
    Denied,
    InvalidName,
    Internal,
    NotImplemented,
    AlreadyOpen,

    /// I/O error specific to `flipperzero-rs` to represent the case a call to
    /// `write` returned `Ok(0)`, meaning that the operation could not be
    /// completed.
    WriteZero,

    /// Any I/O error from the Flipper Zero SDK that's not part of this list.
    ///
    /// Errors that are `Uncategorized` now may move to a different or a new [`Error`]
    /// variant in the future.
    #[non_exhaustive]
    #[doc(hidden)]
    Uncategorized(sys::FS_Error),
}

impl Error {
    pub fn to_sys(&self) -> Option<sys::FS_Error> {
        match self {
            Self::NotReady => Some(sys::FS_Error_FSE_NOT_READY),
            Self::Exists => Some(sys::FS_Error_FSE_EXIST),
            Self::NotExists => Some(sys::FS_Error_FSE_NOT_EXIST),
            Self::InvalidParameter => Some(sys::FS_Error_FSE_INVALID_PARAMETER),
            Self::Denied => Some(sys::FS_Error_FSE_DENIED),
            Self::InvalidName => Some(sys::FS_Error_FSE_INVALID_NAME),
            Self::Internal => Some(sys::FS_Error_FSE_INTERNAL),
            Self::NotImplemented => Some(sys::FS_Error_FSE_NOT_IMPLEMENTED),
            Self::AlreadyOpen => Some(sys::FS_Error_FSE_ALREADY_OPEN),
            Self::Uncategorized(error_code) => Some(*error_code),
            _ => None,
        }
    }

    pub fn from_sys(err: sys::FS_Error) -> Option<Self> {
        match err {
            sys::FS_Error_FSE_OK => None,
            sys::FS_Error_FSE_NOT_READY => Some(Self::NotReady),
            sys::FS_Error_FSE_EXIST => Some(Self::Exists),
            sys::FS_Error_FSE_NOT_EXIST => Some(Self::NotExists),
            sys::FS_Error_FSE_INVALID_PARAMETER => Some(Self::InvalidParameter),
            sys::FS_Error_FSE_DENIED => Some(Self::Denied),
            sys::FS_Error_FSE_INVALID_NAME => Some(Self::InvalidName),
            sys::FS_Error_FSE_INTERNAL => Some(Self::Internal),
            sys::FS_Error_FSE_NOT_IMPLEMENTED => Some(Self::NotImplemented),
            sys::FS_Error_FSE_ALREADY_OPEN => Some(Self::AlreadyOpen),
            error_code => Some(Self::Uncategorized(error_code)),
        }
    }

    /// Description associated with [`Error`].
    pub fn description(&self) -> &CStr {
        unsafe { CStr::from_ptr(sys::filesystem_api_error_get_desc(self.to_sys().unwrap())) }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.description().to_bytes().escape_ascii().fmt(f)
    }
}

impl ufmt::uDisplay for Error {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        for c in self.description().to_bytes().escape_ascii() {
            f.write_char(c as char)?;
        }

        Ok(())
    }
}

/// Trait comparable to `std::Read` for the Flipper Zero API
pub trait Read {
    /// Reads some bytes from this source into the given buffer, returning how many bytes
    /// were read.
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error>;
}

/// Trait comparable to `std::Seek` for the Flipper Zero API
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

/// Trait comparable to `std::Write` for the Flipper Zero API
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn flush(&mut self) -> Result<(), Error>;

    fn write_all(&mut self, mut buf: &[u8]) -> Result<(), Error> {
        while !buf.is_empty() {
            match self.write(buf) {
                Ok(0) => return Err(Error::WriteZero),
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
