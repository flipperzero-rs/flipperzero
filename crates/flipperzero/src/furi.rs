//! High-level bindings to Furi kernel

use core::ffi::c_void;

use flipperzero_sys as sys;
pub struct Stdout;

impl core::fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe {
            if sys::furi::thread_stdout_write(s.as_ptr(), s.len()) != s.len() {
                return Err(core::fmt::Error);
            }
        }

        Ok(())
    }
}

impl Stdout {
    pub fn flush(&mut self) -> core::fmt::Result {
        unsafe {
            if sys::furi::thread_stdout_flush() != 0 {
                return Err(core::fmt::Error);
            }
        }

        Ok(())
    }
}

/// Puts the current thread to sleep for at least the specified amount of time.
pub fn sleep(duration: core::time::Duration) {
    unsafe {
        // For durations of 1h+, use delay_ms so uint32_t doesn't overflow
        if duration < core::time::Duration::from_secs(3600) {
            sys::furi::delay_us(duration.as_micros() as u32);
        } else {
            sys::furi::delay_ms(duration.as_millis() as u32);
        }
    }
}

/// Error codes returned from Furi function calls
#[derive(Debug)]
pub enum Error {
    /// General runtime error, corresponds to FuriStatusError
    Generic,
    /// Timeout was exceeded, corresponds to FuriStatusErrorTimeout
    Timeout,
    // Resource not available, corresponds to FuriStatusErrorResource
    Resource,
    // Parameter error, corresponds to FuriStatusErrorParameter
    Parameter,
    // System is out of memory, corresponds to FuriStatusErrorNoMemory
    NoMemory,
    // Not allowed in ISR context, corresponds to FuriStatusErrorISR
    ISR,
    // Unrecognized error code.
    Other(u32),
}

impl From<u32> for Error {
    fn from(code: u32) -> Self {
        match i32::from_le_bytes(code.to_le_bytes()) {
            -1 => Error::Generic,
            -2 => Error::Timeout,
            -3 => Error::Resource,
            -4 => Error::Parameter,
            -5 => Error::NoMemory,
            -6 => Error::ISR,

            _ => Error::Other(code)
        }
    }
}

/// Furi result type
pub type Result<T> = core::result::Result<T, Error>;

/// MessageQueue provides a safe wrapper around the furi message queue primitive.
pub struct MessageQueue<M: Sized> {
    hnd: *const sys::message_queue::FuriMessageQueue,
    _marker: core::marker::PhantomData<M>,
}

impl<M: Sized> MessageQueue<M> {
    /// Constructs a message queue with the given capacity.
    pub fn new(capacity: u32) -> Self {
        Self {
            hnd: unsafe { sys::message_queue::alloc(capacity, core::mem::size_of::<M>()) },
            _marker: core::marker::PhantomData::<M>,
        }
    }

    // Attempts to add the message to the end of the queue, waiting up to timeout ticks.
    pub fn put(&self, mut msg: M, timeout_ticks: u32) -> Result<()> {
        let code = unsafe {
            sys::message_queue::put(self.hnd, &mut msg as *mut _ as *const c_void, timeout_ticks)
        };

        match code {
            0 => Ok(()),
            _ => Err(code.into()),
        }
    }

    // Attempts to read a message from the front of the queue within timeout ticks.
    pub fn get(&self, timeout_ticks: u32) -> Result<M> {
        let mut out = core::mem::MaybeUninit::<M>::uninit();
        let code = unsafe {
            sys::message_queue::get(self.hnd, out.as_mut_ptr() as *mut c_void, timeout_ticks)
        };

        match code {
            0 => Ok(unsafe { out.assume_init() }),
            _ => Err(code.into()),
        }
    }

    /// Returns the capacity of the queue.
    pub fn capacity(&self) -> u32 {
        unsafe { sys::message_queue::capacity(self.hnd) }
    }

    /// Returns the number of elements in the queue.
    pub fn len(&self) -> u32 {
        unsafe { sys::message_queue::count(self.hnd) }
    }

    /// Returns the number of free slots in the queue.
    pub fn space(&self) -> u32 {
        unsafe { sys::message_queue::space(self.hnd) }
    }
}

impl<M: Sized> Drop for MessageQueue<M> {
    fn drop(&mut self) {
        unsafe {
            sys::message_queue::free(self.hnd)
        }
    }
}