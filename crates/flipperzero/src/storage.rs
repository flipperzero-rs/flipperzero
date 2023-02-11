use core::ffi::{c_char, c_void, CStr};

use flipperzero_sys as sys;
use flipperzero_sys::furi::UnsafeRecord;

use crate::io::*;

const RECORD_STORAGE: *const c_char = sys::c_string!("storage");

#[derive(Debug, Default, Clone, Copy)]
pub struct OpenOptions {
    access_mode: u8,
    open_mode: u8,
}

impl OpenOptions {
    pub fn new() -> Self {
        Self::default()
    }

    fn from_parts(access_mode: u8, open_mode: u8) -> Self {
        OpenOptions {
            access_mode,
            open_mode,
        }
    }

    /// Read access
    pub fn read(self, set: bool) -> Self {
        OpenOptions::from_parts(
            if set {
                self.access_mode | sys::FS_AccessMode_FSAM_READ
            } else {
                self.access_mode & !sys::FS_AccessMode_FSAM_READ
            },
            self.open_mode,
        )
    }

    /// Write access
    pub fn write(self, set: bool) -> Self {
        OpenOptions::from_parts(
            if set {
                self.access_mode | sys::FS_AccessMode_FSAM_WRITE
            } else {
                self.access_mode & !sys::FS_AccessMode_FSAM_WRITE
            },
            self.open_mode,
        )
    }

    /// Open file, fail if file doesn't exist
    pub fn open_existing(self, set: bool) -> Self {
        OpenOptions::from_parts(
            self.access_mode,
            if set {
                self.open_mode | sys::FS_OpenMode_FSOM_OPEN_EXISTING
            } else {
                self.open_mode & !sys::FS_OpenMode_FSOM_OPEN_EXISTING
            },
        )
    }

    /// Open file. Create new file if not exist
    pub fn open_always(self, set: bool) -> Self {
        OpenOptions::from_parts(
            self.access_mode,
            if set {
                self.open_mode | sys::FS_OpenMode_FSOM_OPEN_ALWAYS
            } else {
                self.open_mode & !sys::FS_OpenMode_FSOM_OPEN_ALWAYS
            },
        )
    }

    /// Open file. Create new file if not exist. Set R/W pointer to EOF
    pub fn open_append(self, set: bool) -> Self {
        OpenOptions::from_parts(
            self.access_mode,
            if set {
                self.open_mode | sys::FS_OpenMode_FSOM_OPEN_APPEND
            } else {
                self.open_mode & !sys::FS_OpenMode_FSOM_OPEN_APPEND
            },
        )
    }

    /// Creates a new file. Fails if the file is exist
    pub fn create_new(self, set: bool) -> Self {
        OpenOptions::from_parts(
            self.access_mode,
            if set {
                self.open_mode | sys::FS_OpenMode_FSOM_CREATE_NEW
            } else {
                self.open_mode & !sys::FS_OpenMode_FSOM_CREATE_NEW
            },
        )
    }

    /// Creates a new file. If file exist, truncate to zero size
    pub fn create_always(self, set: bool) -> Self {
        OpenOptions::from_parts(
            self.access_mode,
            if set {
                self.open_mode | sys::FS_OpenMode_FSOM_CREATE_ALWAYS
            } else {
                self.open_mode & !sys::FS_OpenMode_FSOM_CREATE_ALWAYS
            },
        )
    }

    pub fn open(self, path: &CStr) -> Result<File, Error> {
        let f = File::new();
        if unsafe {
            sys::storage_file_open(
                f.0,
                path.as_ptr() as *const i8,
                self.access_mode,
                self.open_mode,
            )
        } {
            Ok(f)
        } else {
            // Per docs, "you need to close the file even if the open operation
            // failed," but this is handled by `Drop`.
            Err(Error::from_sys(unsafe { sys::storage_file_get_error(f.0) }))
        }
    }
}

/// File stream with buffered read operations.
pub struct File(*mut sys::File);

impl File {
    pub fn new() -> Self {
        unsafe {
            File(sys::storage_file_alloc(
                UnsafeRecord::open(RECORD_STORAGE).as_ptr(),
            ))
        }
    }
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe {
            // `storage_file_close` calls `storage_file_sync`
            // internally, so it's not necesssary to call it here.
            sys::storage_file_close(self.0);
        }
    }
}

impl Read for File {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        Ok(unsafe {
            sys::storage_file_read(
                self.0,
                buf.as_mut_ptr() as *mut c_void,
                buf.len().try_into().map_err(|_| Error::InvalidParameter)?,
            )
        } as usize)
    }
}

impl Seek for File {
    fn seek(&mut self, pos: SeekFrom) -> Result<usize, Error> {
        let (offset_type, offset) = match pos {
            SeekFrom::Start(n) => (true, n.try_into().map_err(|_| Error::InvalidParameter)?),
            SeekFrom::End(n) => (false, n.try_into().map_err(|_| Error::InvalidParameter)?),
            SeekFrom::Current(n) => (false, n.try_into().map_err(|_| Error::InvalidParameter)?),
        };
        unsafe {
            if sys::storage_file_seek(self.0, offset, offset_type) {
                Ok(sys::storage_file_tell(self.0)
                    .try_into()
                    .map_err(|_| Error::InvalidParameter)?)
            } else {
                Err(Error::from_sys(sys::storage_file_get_error(self.0)))
            }
        }
    }

    fn rewind(&mut self) -> Result<(), Error> {
        self.seek(SeekFrom::Start(0)).map(|_| {})
    }

    fn stream_len(&mut self) -> Result<usize, Error> {
        Ok(unsafe {
            sys::storage_file_size(self.0)
                .try_into()
                .map_err(|_| Error::InvalidParameter)?
        })
    }

    fn stream_position(&mut self) -> Result<usize, Error> {
        Ok(unsafe {
            sys::storage_file_tell(self.0)
                .try_into()
                .map_err(|_| Error::InvalidParameter)?
        })
    }
}

impl Write for File {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        let to_write = buf.len().try_into().map_err(|_| Error::InvalidParameter)?;
        let bytes_written =
            unsafe { sys::storage_file_write(self.0, buf.as_ptr() as *mut c_void, to_write) };
        if bytes_written == to_write
            || bytes_written < to_write && unsafe { sys::storage_file_eof(self.0) }
        {
            Ok(bytes_written as usize)
        } else {
            Err(Error::from_sys(unsafe {
                sys::storage_file_get_error(self.0)
            }))
        }
    }

    fn flush(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

impl Default for File {
    fn default() -> Self {
        Self::new()
    }
}
