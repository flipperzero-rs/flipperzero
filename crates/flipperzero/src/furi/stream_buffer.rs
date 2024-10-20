use core::{
    cell::Cell, ffi::c_void, fmt::Debug, marker::PhantomData, num::NonZeroUsize, ptr::NonNull,
};

use crate::furi;

use flipperzero_sys::{self as sys, furi::Status};
use ufmt::uDebug;

/// Zero size type to mark types as not Sync.
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
/// For easy safe abstractions use the
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
// The furi api doesn't impose any restrictions to having a stream buffer moved between threads.
unsafe impl Send for StreamBuffer {}

// SAFETY:
// The furi api only requires users to ensure that only one writer and one reader exists at the same
// time, they may be moved between threads.
// Using this data structure between threads remotely is therefore safe.
// The safety guarantee for sending and receiving data is therefore shifted to the send and receive
// methods.
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
        // The furi api guarantees a valid non-null pointer.
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

    /// Send bytes.
    ///
    /// The bytes are copied into the stream buffer, the amount of sent bytes is returned.
    /// The function blocks if not enough space is available in the stream buffer until either the
    /// data is successfully sent or the timeout runs out.
    /// Passing [`Duration::ZERO`](furi::time::Duration::ZERO) returns immediately after sending as
    /// many bytes as the stream buffer could fit while
    /// [`Duration::WAIT_FOREVER`](furi::time::Duration::WAIT_FOREVER) waits indefinitely.
    ///
    /// # Safety
    ///
    /// The stream buffer requires that only one writer and reader may at exist at any given time.
    /// Since [`StreamBuffer`] is [`Send`] and [`Sync`], you have to ensure you only ever have one
    /// writer at the same time calling `send`.
    ///
    /// A safe alternative is using the
    #[cfg_attr(not(feature = "alloc"), doc = "`Sender`,")]
    #[cfg_attr(feature = "alloc", doc = "[`Sender`],")]
    /// available using the `alloc` feature.
    ///
    /// # Interrupt Routines
    ///
    /// Inside of an interrupt routine the `timeout` is ignored.
    pub unsafe fn send(&self, data: &[u8], timeout: furi::time::Duration) -> usize {
        let self_ptr = self.0.as_ptr();
        let data_ptr = data.as_ptr().cast();
        let data_len = data.len();
        let timeout = timeout.0;
        unsafe { sys::furi_stream_buffer_send(self_ptr, data_ptr, data_len, timeout) }
    }

    /// Receive bytes.
    ///
    /// The received bytes will be copied into the provided buffer, returning how many bytes were
    /// successfully received.
    /// The function blocks until either the [trigger level](Self::set_trigger_level) is reached,
    /// the passed buffer is filled or the timeout ends.
    /// Passing [`Duration::ZERO`](furi::time::Duration::ZERO) returns immediately after receiving as
    /// many bytes as possible and available in the stream buffer while
    /// [`Duration::WAIT_FOREVER`](furi::time::Duration::WAIT_FOREVER) waits indefinitely if the
    /// buffer not fills or the trigger level not reaches.
    ///
    /// # Safety
    ///
    /// The stream buffer requires that only one writer and reader may at exist at any given time.
    /// Since [`StreamBuffer`] is [`Send`] and [`Sync`], you have to ensure you only ever have one
    /// reader at the same time calling `receive`.
    ///
    /// A safe alternative is using the
    #[cfg_attr(not(feature = "alloc"), doc = "`Receiver`,")]
    #[cfg_attr(feature = "alloc", doc = "[`Receiver`],")]
    /// available using the `alloc` feature.
    ///
    /// # Interrupt Routines
    ///
    /// Inside of an interrupt routine the `timeout` is ignored.
    pub unsafe fn receive(&self, data: &mut [u8], timeout: furi::time::Duration) -> usize {
        let self_ptr = self.0.as_ptr();
        let data_ptr: *mut c_void = data.as_mut_ptr().cast();
        let data_len = data.len();
        let timeout = timeout.0;
        unsafe { sys::furi_stream_buffer_receive(self_ptr, data_ptr, data_len, timeout) }
    }

    /// Get the number of bytes currently available.
    pub fn bytes_available(&self) -> usize {
        let self_ptr = self.0.as_ptr();
        unsafe { sys::furi_stream_buffer_bytes_available(self_ptr) }
    }

    /// Get the number of bytes that can still fit.
    pub fn spaces_available(&self) -> usize {
        let self_ptr = self.0.as_ptr();
        unsafe { sys::furi_stream_buffer_spaces_available(self_ptr) }
    }

    /// Check if the buffer is full.
    pub fn is_full(&self) -> bool {
        let self_ptr = self.0.as_ptr();
        unsafe { sys::furi_stream_buffer_is_full(self_ptr) }
    }

    /// Check if the buffer is empty.
    pub fn is_empty(&self) -> bool {
        let self_ptr = self.0.as_ptr();
        unsafe { sys::furi_stream_buffer_is_empty(self_ptr) }
    }

    /// Attempt to reset the stream buffer.
    ///
    /// This will clear the buffer, discarding any data it contains and returning it to its
    /// initial empty state.
    /// The reset can only occur if there are no tasks blocked waiting to send to or receive from
    /// the stream buffer; attempting to reset during this time will result in an [`Err`].
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
        /// Convert a stream buffer into a pair of [`Sender`] and [`Receiver`].
        ///
        /// As a safe abstraction this splitting the stream buffer into a pair of sender (writer)
        /// and receiver (reader) makes sending and receiving bytes safe as both types purposefully
        /// do not implement [`Clone`] nor [`Sync`] and can therefore only be used from one thread
        /// at a time, fulfilling the safety constraints of the stream buffer.
        ///
        /// Both types implement a `as_stream_buffer` method to still get access to all the methods
        /// exposed by the `StreamBuffer`.
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

    /// Sender side of a furi stream buffer.
    ///
    /// An instance can be obtained using the [`StreamBuffer::into_stream`] method.
    ///
    /// This struct allows sending data through the stream buffer in a safe manner.
    /// Use the [`is_receiver_alive`](Self::is_receiver_alive) method to verify if the associated
    /// [`Receiver`] is still alive, helping to avoid sending data that will not be read.
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

    /// Sends data using the `Sender`.
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

    /// Pure additional methods to work with the `Sender` and [`Receiver`].
    impl Sender {
        /// Check if the associated receiver is still alive.
        ///
        /// This method helps prevent unnecessary data transmission when the [`Receiver`] is no
        /// longer available.
        pub fn is_receiver_alive(&self) -> bool {
            // SAFETY:
            // Since both Receiver and Sender are not Clone, the only Arcs are those created by the
            // into_stream method.
            // If the strong count of the Arc referencing the buffer is 2, it indicates that
            // the Receiver and the related Sender are still alive.
            Arc::strong_count(&self.buffer_ref) == 2
        }

        /// Get a reference to the underlying [`StreamBuffer`].
        pub fn as_stream_buffer(&self) -> &StreamBuffer {
            &self.buffer_ref
        }

        /// Try to get the underlying stream buffer.
        /// 
        /// This method tries to get underlying stream buffer, which is only possible if the 
        /// [`Receiver`] is already dropped.
        /// If the `Receiver` is still alive, this will return [`None`].
        pub fn into_stream_buffer(self) -> Option<StreamBuffer> {
            Arc::into_inner(self.buffer_ref)
        } 
    }

    /// Receiver side of a furi stream buffer.
    ///
    /// An instance can be obtained using the [`StreamBuffer::into_stream`] method.
    ///
    /// This struct allows receiving data through the stream buffer in a safe manner.
    /// Use the [`is_sender_alive`](Self::is_sender_alive) method to verify if the associated
    /// [`Sender`] is still alive, helping to avoid trying to receive data that will never be sent.
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

    /// Receive data using the `Receiver`.
    ///
    /// Returns the number of bytes that were successfully received.
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

    /// Pure additional methods to work with the [`Sender`] and `Receiver`.
    impl Receiver {
        /// Check if the associated sender is still alive.
        ///
        /// This method helps prevent unnecessary data reception when the sender is no longer
        /// available.
        pub fn is_sender_alive(&self) -> bool {
            // SAFETY:
            // Since both Receiver and Sender are not Clone, the only Arcs are those created by the
            // into_stream method.
            // If the strong count of the Arc referencing the buffer is 2, it indicates that
            // the Receiver and the related Sender are still alive.
            Arc::strong_count(&self.buffer_ref) == 2
        }

        /// Get a reference to the underlying [`StreamBuffer`].
        pub fn as_stream_buffer(&self) -> &StreamBuffer {
            &self.buffer_ref
        }

        /// Try to get the underlying stream buffer.
        /// 
        /// This method tries to get underlying stream buffer, which is only possible if the 
        /// [`Sender`] is already dropped.
        /// If the `Sender` is still alive, this will return [`None`].
        pub fn into_stream_buffer(self) -> Option<StreamBuffer> {
            Arc::into_inner(self.buffer_ref)
        } 
    }
}
