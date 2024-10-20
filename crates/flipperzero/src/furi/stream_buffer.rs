//! Furi stream buffer primitive.

use core::{
    cell::Cell, ffi::c_void, fmt::Debug, marker::PhantomData, num::NonZeroUsize, ptr::NonNull,
};

use crate::furi;

use flipperzero_sys::{self as sys, furi::Status};
use ufmt::uDebug;

/// A zero-sized type used to mark types as `!Sync`.
type PhantomUnsync = PhantomData<Cell<()>>;

/// Furi stream buffer primitive.
///
/// Stream buffers are used to send a continous stream of data from one task or interrupt to another.
/// Their implementation is light weight, making them particularly suited for interrupt to task and
/// core to core communication scenarios.
///
/// # Safety
///
/// Stream buffer implementation assumes there is only one task or interrupt that will write to the
/// buffer (the writer), and only one task or interrupt that will read from the buffer (the reader).
/// This behavior has to be carefully ensured when using [`send`](Self::send) and
/// [`receive`](Self::receive) directly.
///
/// For safer usage, consider the
#[cfg_attr(not(feature = "alloc"), doc = "`Sender`")]
#[cfg_attr(feature = "alloc", doc = "[`Sender`]")]
/// and
#[cfg_attr(not(feature = "alloc"), doc = "`Receiver`,")]
#[cfg_attr(feature = "alloc", doc = "[`Receiver`],")]
/// available via the `alloc` feature, to send and receive data.
#[derive(Debug)]
pub struct StreamBuffer(NonNull<sys::FuriStreamBuffer>);

impl uDebug for StreamBuffer {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        f.debug_tuple("StreamBuffer")?
            .field(&self.0.as_ptr())?
            .finish()
    }
}

// SAFETY:
// The Furi API does not impose any restrictions on moving a stream buffer between threads.
// Since the API permits this, `StreamBuffer` can safely implement `Send`.
unsafe impl Send for StreamBuffer {}

// SAFETY:
// The Furi API requires that there be only one writer and one reader at any given time.
// However, both the writer and reader may be moved between threads.
// This ensures that using the stream buffer across threads is safe, provided that the
// one-writer-one-reader rule is upheld.
// The responsibility for maintaining safety while sending and receiving data lies within the
// `send` and `receive` methods.
unsafe impl Sync for StreamBuffer {}

impl StreamBuffer {
    /// Create a new instance of a `StreamBuffer`.
    ///
    /// The `trigger_level` defines the number of bytes that must be present in the stream buffer
    /// before any blocked tasks waiting for data can proceed.
    ///
    /// For sending and receiving data safely use
    #[cfg_attr(not(feature = "alloc"), doc = "`into_stream`")]
    #[cfg_attr(feature = "alloc", doc = "[`into_stream`](Self::into_stream)")]
    /// which is available using the `alloc` feature.
    pub fn new(size: NonZeroUsize, trigger_level: usize) -> Self {
        let size: usize = size.into();

        // SAFETY:
        // The Furi api guarantees a valid non-null pointer.
        // The `furi_stream_buffer_alloc` function checks that the size is not 0, we always
        // fulfill that using the NonZeroUsize type.
        let ptr =
            unsafe { NonNull::new_unchecked(sys::furi_stream_buffer_alloc(size, trigger_level)) };

        Self(ptr)
    }

    /// Set the trigger level.
    ///
    /// The trigger level is the number of bytes that must be present in the stream buffer before
    /// any blocked tasks waiting for data can proceed.
    ///
    /// If the specified trigger level exceeds the buffer's length, an [`Err`] is returned.
    pub fn set_trigger_level(&self, trigger_level: usize) -> Result<(), ()> {
        let self_ptr = self.0.as_ptr();
        let updated = unsafe { sys::furi_stream_set_trigger_level(self_ptr, trigger_level) };
        if updated {
            Ok(())
        } else {
            Err(())
        }
    }

    /// Sends data to the buffer.
    ///
    /// The function copies the bytes into the buffer, returning the number of bytes successfully
    /// sent.
    /// It blocks if not enough space is available until the data is sent or the timeout expires.
    /// Passing [`Duration::ZERO`](furi::time::Duration::ZERO) immediately returns with as many
    /// bytes as can fit, while [`Duration::WAIT_FOREVER`](furi::time::Duration::WAIT_FOREVER) waits
    /// indefinitely.
    ///
    /// # Safety
    ///
    /// Only one writer and one reader may exist at a time. Since [`StreamBuffer`] is both [`Send`]
    /// and [`Sync`], it is your responsibility to ensure that only one writer calls `send` at any
    /// given time.
    ///
    /// For safer alternatives, consider using the
    #[cfg_attr(not(feature = "alloc"), doc = "`Sender`")]
    #[cfg_attr(feature = "alloc", doc = "[`Sender`]")]
    /// abstraction available with the `alloc` feature.
    ///
    /// # Interrupt Routines
    ///
    /// The `timeout` is ignored when called from an interrupt routine.
    pub unsafe fn send(&self, data: &[u8], timeout: furi::time::Duration) -> usize {
        let self_ptr = self.0.as_ptr();
        let data_ptr = data.as_ptr().cast();
        let data_len = data.len();
        let timeout = timeout.0;
        unsafe { sys::furi_stream_buffer_send(self_ptr, data_ptr, data_len, timeout) }
    }

    /// Receives data from the buffer.
    ///
    /// Copies received bytes into the provided buffer, returning the number of bytes successfully
    /// received.
    /// The function blocks until the [trigger level](Self::set_trigger_level) is reached, the
    /// buffer is filled, or the timeout expires.
    /// Passing [`Duration::ZERO`](furi::time::Duration::ZERO) returns immediately with as many
    /// bytes as available, while [`Duration::WAIT_FOREVER`](furi::time::Duration::WAIT_FOREVER)
    /// waits indefinitely.
    ///
    /// # Safety
    ///
    /// Only one writer and one reader may exist at a time. Since [`StreamBuffer`] is both [`Send`]
    /// and [`Sync`], it is your responsibility to ensure that only one reader calls `receive` at
    /// any given time.
    ///
    /// For safer alternatives, consider using the
    #[cfg_attr(not(feature = "alloc"), doc = "`Receiver`")]
    #[cfg_attr(feature = "alloc", doc = "[`Receiver`]")]
    /// abstraction available with the `alloc` feature.
    ///
    /// # Interrupt Routines
    ///
    /// The `timeout` is ignored when called from an interrupt routine.
    pub unsafe fn receive(&self, data: &mut [u8], timeout: furi::time::Duration) -> usize {
        let self_ptr = self.0.as_ptr();
        let data_ptr: *mut c_void = data.as_mut_ptr().cast();
        let data_len = data.len();
        let timeout = timeout.0;
        unsafe { sys::furi_stream_buffer_receive(self_ptr, data_ptr, data_len, timeout) }
    }

    /// Returns the number of bytes currently available in the buffer.
    pub fn bytes_available(&self) -> usize {
        let self_ptr = self.0.as_ptr();
        unsafe { sys::furi_stream_buffer_bytes_available(self_ptr) }
    }

    /// Returns the number of bytes that can still fit in the buffer.
    pub fn spaces_available(&self) -> usize {
        let self_ptr = self.0.as_ptr();
        unsafe { sys::furi_stream_buffer_spaces_available(self_ptr) }
    }

    /// Checks if the buffer is full.
    pub fn is_full(&self) -> bool {
        let self_ptr = self.0.as_ptr();
        unsafe { sys::furi_stream_buffer_is_full(self_ptr) }
    }

    /// Checks if the buffer is empty.
    pub fn is_empty(&self) -> bool {
        let self_ptr = self.0.as_ptr();
        unsafe { sys::furi_stream_buffer_is_empty(self_ptr) }
    }

    /// Attempts to reset the stream buffer.
    ///
    /// Clears the buffer, discarding any data it contains and returning it to its initial empty
    /// state.
    /// The reset can only succeed if no tasks are blocked waiting to send or receive data;
    /// otherwise, an [`Err`] is returned.
    pub fn reset(&self) -> furi::Result<()> {
        let status = unsafe { sys::furi_stream_buffer_reset(self.0.as_ptr()) };
        let status = Status(status);
        status.err_or(())
    }
}

impl Drop for StreamBuffer {
    fn drop(&mut self) {
        unsafe {
            sys::furi_stream_buffer_free(self.0.as_ptr());
        }
    }
}

#[cfg(feature = "alloc")]
pub use stream::*;

#[cfg(feature = "alloc")]
mod stream {
    use crate::furi;

    use super::*;

    use alloc::sync::Arc;

    impl StreamBuffer {
        /// Converts the stream buffer into a pair of [`Sender`] and [`Receiver`].
        ///
        /// This provides a safe abstraction by splitting the stream buffer into a sender (writer)
        /// and receiver (reader), ensuring that sending and receiving bytes is safe.
        /// Neither [`Sender`] nor [`Receiver`] implement [`Clone`] or [`Sync`], meaning they can
        /// only be used from a single thread at a time, thus adhering to the stream buffer's safety
        /// constraints.
        ///
        /// Both types provide an `as_stream_buffer` method, allowing access to all the methods
        /// exposed by the underlying `StreamBuffer`.
        pub fn into_stream(self) -> (Sender, Receiver) {
            let stream_buffer = Arc::new(self);

            let sender = Sender {
                buffer_ref: stream_buffer.clone(),
                _unsync: PhantomUnsync::default(),
            };

            let receiver = Receiver {
                buffer_ref: stream_buffer,
                _unsync: PhantomUnsync::default(),
            };

            (sender, receiver)
        }
    }

    /// The sending side of a Furi stream buffer.
    ///
    /// This struct allows data to be sent through the stream buffer in a safe manner.
    /// An instance can be obtained via [`StreamBuffer::into_stream`].
    ///
    /// Use the [`is_receiver_alive`](Self::is_receiver_alive) method to verify if the corresponding
    /// [`Receiver`] is still alive, ensuring that data isn't sent to a dropped receiver.
    pub struct Sender {
        buffer_ref: Arc<StreamBuffer>,
        _unsync: PhantomUnsync,
    }

    impl Debug for Sender {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            // the receiver_alive field is not real but a nice debug information
            f.debug_struct("Sender")
                .field("buffer_ref", &self.buffer_ref)
                .field("receiver_alive", &self.is_receiver_alive())
                .finish()
        }
    }

    impl uDebug for Sender {
        fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
        where
            W: ufmt::uWrite + ?Sized,
        {
            // the receiver_alive field is not real but a nice debug information
            f.debug_struct("Sender")?
                .field("buffer_ref", &self.buffer_ref.0.as_ptr())?
                .field("receiver_alive", &self.is_receiver_alive())?
                .finish()
        }
    }

    /// Implements sending data through the `Sender`.
    ///
    /// Data is sent only when the amount of bytes reaches the trigger level in the underlying
    /// stream buffer, which wakes up the listening [`Receiver`] if applicable.
    ///
    /// Returns the number of bytes that were successfully sent.
    ///
    /// # Interrupt Routines
    ///
    /// When used in an interrupt routine, the timeout will be ignored.
    impl Sender {
        /// Sends bytes without blocking.
        ///
        /// Attempts to send the specified bytes immediately.
        /// If the underlying stream buffer does not have enough free space, it sends only the bytes
        /// that fit and returns immediately.
        pub fn send(&self, data: &[u8]) -> usize {
            unsafe { self.buffer_ref.send(data, furi::time::Duration::ZERO) }
        }

        /// Sends bytes in a blocking manner.
        ///
        /// Blocks until all bytes are sent.
        ///
        /// # Interrupt Routines
        ///
        /// In an interrupt routine, this method behaves like [`send`](Self::send).
        pub fn send_blocking(&self, data: &[u8]) -> usize {
            unsafe {
                self.buffer_ref
                    .send(data, furi::time::Duration::WAIT_FOREVER)
            }
        }

        /// Sends bytes with a timeout.
        ///
        /// Attempts to send as many bytes as possible within the specified timeout duration.
        /// It may wait until the timeout is reached if necessary, but it returns immediately once
        /// all bytes are sent or the timeout expires.
        ///
        /// # Interrupt Routines
        ///
        /// In an interrupt routine, this method behaves like [`send`](Self::send).
        pub fn send_with_timeout(&self, data: &[u8], timeout: furi::time::Duration) -> usize {
            unsafe { self.buffer_ref.send(data, timeout) }
        }
    }

    impl Sender {
        /// Checks if the associated [`Receiver`] is still alive.
        ///
        /// This method helps avoid sending data when the [`Receiver`] has already been dropped.
        /// If the receiver is still active, the method returns `true`.
        pub fn is_receiver_alive(&self) -> bool {
            // SAFETY:
            // Since both Receiver and Sender are not Clone, the only Arcs are those created by the
            // `into_stream` method.
            // If the strong count of the Arc referencing the buffer is 2, it indicates that
            // the Receiver and the related Sender are still alive.
            Arc::strong_count(&self.buffer_ref) == 2
        }

        /// Returns a reference to the underlying [`StreamBuffer`].
        pub fn as_stream_buffer(&self) -> &StreamBuffer {
            &self.buffer_ref
        }

        /// Attempts to take ownership of the underlying [`StreamBuffer`].
        ///
        /// This method consumes the `Sender` and attempts to return the underlying stream buffer.
        /// It can only succeed if the corresponding [`Receiver`] has already been dropped.
        /// If the receiver is still alive, it returns [`None`].
        pub fn into_stream_buffer(self) -> Option<StreamBuffer> {
            Arc::into_inner(self.buffer_ref)
        }
    }

    /// The receiving side of a Furi stream buffer.
    ///
    /// This struct allows data to be received through the stream buffer in a safe manner.
    /// An instance can be obtained via [`StreamBuffer::into_stream`].
    ///
    /// Use the [`is_sender_alive`](Self::is_sender_alive) method to check if the associated
    /// [`Sender`] is still active, helping to avoid trying to receive data when no more will be
    /// sent.
    pub struct Receiver {
        buffer_ref: Arc<StreamBuffer>,
        _unsync: PhantomUnsync,
    }

    impl Debug for Receiver {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            // the sender_alive field is not real but a nice debug information
            f.debug_struct("Receiver")
                .field("buffer_ref", &self.buffer_ref)
                .field("sender_alive", &self.is_sender_alive())
                .finish()
        }
    }

    impl uDebug for Receiver {
        fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
        where
            W: ufmt::uWrite + ?Sized,
        {
            // the sender_alive field is not real but a nice debug information
            f.debug_struct("Receiver")?
                .field("buffer_ref", &self.buffer_ref.0.as_ptr())?
                .field("sender_alive", &self.is_sender_alive())?
                .finish()
        }
    }

    /// Implements receiving data through the `Receiver`.
    ///
    /// Returns the number of bytes successfully received.
    ///
    /// # Interrupt Routines
    ///
    /// When used in an interrupt routine, the timeout will be ignored.
    impl Receiver {
        /// Receive bytes without blocking.
        ///
        /// Tries to receive bytes immediately.
        /// It will either receive all available bytes or fill the buffer, whichever happens first.
        /// Returns the number of bytes successfully received.
        pub fn recv(&self, data: &mut [u8]) -> usize {
            unsafe { self.buffer_ref.receive(data, furi::time::Duration::ZERO) }
        }

        /// Receive bytes, blocking if necessary.
        ///
        /// Waits until the buffer is filled or the [trigger level](StreamBuffer::set_trigger_level)
        /// is reached.
        /// More bytes than the trigger level may be received if a large enough chunk arrives at
        /// once, though it may still be less than the full buffer.
        /// Returns the number of bytes successfully received.
        ///
        /// # Interrupt Routines
        ///
        /// If called in an interrupt routine, this behaves like [`recv`](Self::recv).
        pub fn recv_blocking(&self, data: &mut [u8]) -> usize {
            unsafe {
                self.buffer_ref
                    .receive(data, furi::time::Duration::WAIT_FOREVER)
            }
        }

        /// Receive bytes with a timeout.
        ///
        /// Waits until the buffer is filled, the [trigger level](StreamBuffer::set_trigger_level)
        /// is reached, or the timeout expires, whichever happens first.
        /// Returns the number of bytes successfully received.
        ///
        /// # Interrupt Routines
        ///
        /// In an interrupt routine, this method behaves like [`recv`](Self::recv).
        pub fn recv_with_timeout(&self, data: &mut [u8], timeout: furi::time::Duration) -> usize {
            unsafe { self.buffer_ref.receive(data, timeout) }
        }
    }

    impl Receiver {
        /// Checks if the associated [`Sender`] is still alive.
        ///
        /// This method helps avoid sending data when the [`Sender`] has already been dropped.
        /// If the sendr is still active, the method returns `true`.
        pub fn is_sender_alive(&self) -> bool {
            // SAFETY:
            // Since both Receiver and Sender are not Clone, the only Arcs are those created by the
            // `into_stream` method.
            // If the strong count of the Arc referencing the buffer is 2, it indicates that
            // the Receiver and the related Sender are still alive.
            Arc::strong_count(&self.buffer_ref) == 2
        }

        /// Returns a reference to the underlying [`StreamBuffer`].
        pub fn as_stream_buffer(&self) -> &StreamBuffer {
            &self.buffer_ref
        }

        /// Attempts to take ownership of the underlying [`StreamBuffer`].
        ///
        /// This method consumes the `Receiver` and attempts to return the underlying stream buffer.
        /// It can only succeed if the corresponding [`Sender`] has already been dropped.
        /// If the sender is still alive, it returns [`None`].
        pub fn into_stream_buffer(self) -> Option<StreamBuffer> {
            Arc::into_inner(self.buffer_ref)
        }
    }
}
