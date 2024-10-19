use core::{fmt::Debug, num::NonZeroUsize, ptr::NonNull};

use alloc::sync::Arc;

use flipperzero_sys::{self as sys, furi::Status};

/// Basic Result which only indicates whether something succeeded or not.
type EmptyResult = Result<(), ()>;

// https://github.com/flipperdevices/flipperzero-firmware/blob/b723d463afccf628712475e11a3d4579f0331f5c/furi/core/base.h#L13
const FURI_WAIT_FOREVER: u32 = 0xFFFFFFFF;

/// Furi stream buffer primitive.
///
/// Stream buffers are used to send a continous stream of data from one task or interrupt to another.
/// Their implementation is light weight, making them particularly suited for interrupt to task and
/// core to core communication scenarios.
///
/// # Note
///
/// Stream buffer implementation assumes there is only one task or interrupt that will write to the
/// buffer (the writer), and only one task or interrupt that will read from the buffer (the reader).
///
/// # Visibility and Safety
///
/// Since the stream buffer's implementation assumes that only one task or interrupt will write to
/// the buffer and only one task or interrupt will read from the buffer, we have to ensure this
/// behavior in the type system to make sound safe abstractions.
///
/// To achieve this, the `StreamBuffer` itself is not public.
/// To send and receive data, we use the [`Sender`] and [`Receiver`], both of them own a reference
/// to the `StreamBuffer` to perform any actions on it.
/// Both are not [`Clone`], making sure we don't have any more senders and receivers on the same
/// stream buffer.
/// Also both are [`Send`] to send them to other threads but not [`Sync`] to ensure that only one
/// thread can use them at any time respectively.
#[derive(Debug)]
struct StreamBuffer(NonNull<sys::FuriStreamBuffer>);

/// Direct implementation of the `furi_stream_buffer` api.
///
/// The [`Sender`] and [`Receiver`] have some more sugar to use that api.
impl StreamBuffer {
    /// Create a new instance of a `StreamBuffer`.
    ///
    /// The `furi_stream_buffer_alloc` function checks that the size is not 0, to always fulfill
    /// this requirement the `size` is a `NonZeroUsize`.
    ///
    /// Further user reference is explained at the [`stream_buffer`] function.
    fn new(size: NonZeroUsize, trigger_level: usize) -> Self {
        unsafe {
            Self(NonNull::new_unchecked(sys::furi_stream_buffer_alloc(
                size.into(),
                trigger_level,
            )))
        }
    }

    fn set_trigger_level(&self, trigger_level: usize) -> EmptyResult {
        let updated = unsafe { sys::furi_stream_set_trigger_level(self.0.as_ptr(), trigger_level) };
        match updated {
            true => Ok(()),
            false => Err(()),
        }
    }

    fn send(&self, data: &[u8], timeout: u32) -> usize {
        unsafe {
            sys::furi_stream_buffer_send(self.0.as_ptr(), data.as_ptr().cast(), data.len(), timeout)
        }
    }

    fn receive(&self, data: &mut [u8], timeout: u32) -> usize {
        unsafe {
            sys::furi_stream_buffer_receive(
                self.0.as_ptr(),
                data.as_mut_ptr().cast(),
                data.len(),
                timeout,
            )
        }
    }

    fn bytes_available(&self) -> usize {
        unsafe { sys::furi_stream_buffer_bytes_available(self.0.as_ptr()) }
    }

    fn spaces_available(&self) -> usize {
        unsafe { sys::furi_stream_buffer_spaces_available(self.0.as_ptr()) }
    }

    fn is_full(&self) -> bool {
        unsafe { sys::furi_stream_buffer_is_full(self.0.as_ptr()) }
    }

    fn is_empty(&self) -> bool {
        unsafe { sys::furi_stream_buffer_is_empty(self.0.as_ptr()) }
    }

    fn reset(&self) -> EmptyResult {
        let status = unsafe { sys::furi_stream_buffer_reset(self.0.as_ptr()) };
        let status = Status(status);
        match status {
            Status::OK => Ok(()),
            Status::ERR => Err(()),
            _ => unreachable!("furi_stream_buffer_reset only returns Ok or Error"),
        }
    }
}

impl Drop for StreamBuffer {
    fn drop(&mut self) {
        // SAFETY:
        // Since we keep an Arc in both, the sender and receiver, we know we only drop when both of
        // them are dropped too.
        unsafe {
            sys::furi_stream_buffer_free(self.0.as_ptr());
        }
    }
}

/// Sender side of a furi stream buffer.
///
/// An instance can be obtained using the [`stream_buffer`] function.
///
/// This struct allows sending data through the stream buffer, checking its state, or resetting it.
/// Use the [`is_receiver_alive`](Self::is_receiver_alive) method to verify if the associated
/// [`Receiver`] is still alive, helping to avoid sending data that will not be read.
pub struct Sender {
    buffer_ref: Arc<StreamBuffer>,
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

unsafe impl Send for Sender {}

/// Sends data using the `Sender`.
///
/// Data is sent only when the amount of bytes reaches the trigger level in the underlying stream
/// buffer, which wakes up the listening [`Receiver`] if applicable.
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
        self.buffer_ref.send(data, 0)
    }

    /// Sends bytes in a blocking manner.
    ///
    /// Blocks until all bytes are sent.
    ///
    /// # Interrupt Routines
    ///
    /// In an interrupt routine, this method behaves like [`send`](Self::send).
    pub fn send_blocking(&self, data: &[u8]) -> usize {
        self.buffer_ref.send(data, FURI_WAIT_FOREVER)
    }

    /// Sends bytes with a timeout.
    ///
    /// Attempts to send as many bytes as possible within the specified timeout duration.
    /// It may wait until the timeout is reached if necessary, but it returns immediately once all
    /// bytes are sent or the timeout expires.
    ///
    /// # Interrupt Routines
    ///
    /// In an interrupt routine, this method behaves like [`send`](Self::send).
    pub fn send_with_timeout(&self, data: &[u8], timeout: core::time::Duration) -> usize {
        let timeout = sys::furi::duration_to_ticks(timeout);
        self.buffer_ref.send(data, timeout)
    }
}

/// Check or modify the underlying furi stream buffer.
impl Sender {
    /// Set the trigger level for the underlying stream buffer.
    ///
    /// The trigger level is the number of bytes that must be present in the stream buffer before
    /// any blocked tasks waiting for data can proceed.
    ///
    /// If the specified trigger level exceeds the buffer's length, an [`Err`] is returned.
    pub fn set_trigger_level(&mut self, trigger_level: usize) -> EmptyResult {
        self.buffer_ref.set_trigger_level(trigger_level)
    }

    /// Get the number of bytes currently available in the underlying stream buffer.
    pub fn bytes_available(&self) -> usize {
        self.buffer_ref.bytes_available()
    }

    /// Get the number of bytes that can still fit in the underlying stream buffer.
    pub fn spaces_available(&self) -> usize {
        self.buffer_ref.spaces_available()
    }

    /// Check if the underlying stream buffer is full.
    pub fn is_full(&self) -> bool {
        self.buffer_ref.is_full()
    }

    /// Check if the underlying stream buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.buffer_ref.is_empty()
    }

    /// Attempt to reset the underlying stream buffer.
    ///
    /// This will clear the buffer, discarding any data it contains and returning it to its initial
    /// empty state.
    /// The reset can only occur if there are no tasks blocked waiting to send to or receive from
    /// the stream buffer; attempting to reset during this time will result in an [`Err`].
    pub fn reset(&mut self) -> EmptyResult {
        self.buffer_ref.reset()
    }
}

/// Pure additional methods to work with the `Sender` and [`Receiver`].
impl Sender {
    /// Check if the associated receiver is still alive.
    ///
    /// This method helps prevent unnecessary data transmission when the [`Receiver`] is no longer
    /// available.
    pub fn is_receiver_alive(&self) -> bool {
        // SAFETY:
        // Since both Receiver and Sender are not Clone, the only Arcs are those created by the
        // stream_buffer function.
        // If the strong count of the Arc referencing the buffer is 2, it indicates that
        // the Receiver and the related Sender are still alive.
        Arc::strong_count(&self.buffer_ref) == 2
    }
}

/// Receiver side of a furi stream buffer.
///
/// An instance can be obtained using the [`stream_buffer`] function.
///
/// This struct allows receiving data through the stream buffer, checking its state, or resetting it.
/// Use the [`is_sender_alive`](Self::is_sender_alive) method to verify if the associated
/// [`Sender`] is still alive, helping to avoid trying to receive data that will never be sent.
pub struct Receiver {
    buffer_ref: Arc<StreamBuffer>,
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

unsafe impl Send for Receiver {}

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
    /// Tries to receive bytes immediately. It will either receive all available bytes or fill the
    /// buffer, whichever happens first.
    /// Returns the number of bytes successfully received.
    pub fn recv(&self, data: &mut [u8]) -> usize {
        self.buffer_ref.receive(data, 0)
    }

    /// Receive bytes, blocking if necessary.
    ///
    /// Waits until the buffer is filled or the [trigger level](Self::set_trigger_level) is reached.
    /// More bytes than the trigger level may be received if a large enough chunk arrives at once,
    /// though it may still be less than the full buffer.
    /// Returns the number of bytes successfully received.
    ///
    /// # Interrupt Routines
    ///
    /// If called in an interrupt routine, this behaves like [`recv`](Self::recv).
    pub fn recv_blocking(&self, data: &mut [u8]) -> usize {
        self.buffer_ref.receive(data, FURI_WAIT_FOREVER)
    }

    /// Receive bytes with a timeout.
    ///
    /// Waits until the buffer is filled, the [trigger level](Self::set_trigger_level) is reached,
    /// or the timeout expires, whichever happens first.
    /// Returns the number of bytes successfully received.
    ///
    /// # Interrupt Routines
    ///
    /// In an interrupt routine, this method behaves like [`recv`](Self::recv).
    pub fn recv_with_timeout(&self, data: &mut [u8], timeout: core::time::Duration) -> usize {
        let timeout = sys::furi::duration_to_ticks(timeout);
        self.buffer_ref.receive(data, timeout)
    }
}

/// Check or modify the underlying furi stream buffer.
impl Receiver {
    /// Set the trigger level for the underlying stream buffer.
    ///
    /// The trigger level is the number of bytes that must be present in the stream buffer
    /// before blocked tasks waiting to receive data can proceed.
    /// If the specified trigger level exceeds the buffer's length, an [`Err`] is returned.
    pub fn set_trigger_level(&mut self, trigger_level: usize) -> EmptyResult {
        self.buffer_ref.set_trigger_level(trigger_level)
    }

    /// Get the number of bytes currently available in the underlying stream buffer.
    pub fn bytes_available(&self) -> usize {
        self.buffer_ref.bytes_available()
    }

    /// Get the number of bytes that can still fit in the underlying stream buffer.
    pub fn spaces_available(&self) -> usize {
        self.buffer_ref.spaces_available()
    }

    /// Check if the underlying stream buffer is full.
    pub fn is_full(&self) -> bool {
        self.buffer_ref.is_full()
    }

    /// Check if the underlying stream buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.buffer_ref.is_empty()
    }

    /// Attempt to reset the underlying stream buffer.
    ///
    /// This will clear the buffer, discarding any data it contains and returning it to its
    /// initial empty state.
    /// The reset can only occur if there are no tasks blocked waiting to send to or receive from
    /// the stream buffer; attempting to reset during this time will result in an [`Err`].
    pub fn reset(&mut self) -> EmptyResult {
        self.buffer_ref.reset()
    }
}

/// Pure additional methods to work with the [`Sender`] and `Receiver`.
impl Receiver {
    /// Check if the associated sender is still alive.
    ///
    /// This method helps prevent unnecessary data reception when the sender is no longer available.
    pub fn is_sender_alive(&self) -> bool {
        // SAFETY:
        // Since both Receiver and Sender are not Clone, the only Arcs are those created by the
        // stream_buffer function.
        // If the strong count of the Arc referencing the buffer is 2, it indicates that
        // the Receiver and the related Sender are still alive.
        Arc::strong_count(&self.buffer_ref) == 2
    }
}

/// Create a [`Sender`]/[`Receiver`] pair for a furi stream buffer.
///
/// This function initializes a furi stream buffer and provides access through the returned
/// [`Sender`] and [`Receiver`] instances.
///
/// The `size` parameter must be a non-zero value, as the stream buffer requires a valid size
/// to allocate memory.
/// The `furi_stream_buffer_alloc` function enforces this constraint by rejecting zero sizes.
///
/// The `trigger_level` defines the number of bytes that must be present in the stream buffer
/// before any blocked tasks waiting for data can proceed.
// TODO: investigate what happens when we set the trigger level to high here
///
/// The Furi stream buffer is designed for single-task or single-interrupt access for both
/// writing and reading operations.
/// To enforce this, it splits the buffer into two parts, similar to the `mpsc::Sender` and
/// `mpsc::Receiver` from the standard library.
/// Both the sender and receiver maintain an owned reference to the stream buffer, ensuring it
/// remains alive as long as either one exists.
/// They can verify if their corresponding pair is still alive using [`Sender::is_receiver_alive`]
/// and [`Receiver::is_sender_alive`] respectively.
///
/// Additionally, both the sender and receiver implement [`Send`] to allow transfer between
/// threads.
/// However, they explicitly do not implement [`Sync`] or [`Clone`] to guarantee that only one
/// writer and one reader can exist at any given time per task.
pub fn stream_buffer(size: NonZeroUsize, trigger_level: usize) -> (Sender, Receiver) {
    let stream_buffer = StreamBuffer::new(size, trigger_level);
    let stream_buffer = Arc::new(stream_buffer);
    let sender = Sender {
        buffer_ref: stream_buffer.clone(),
    };
    let receiver = Receiver {
        buffer_ref: stream_buffer.clone(),
    };
    (sender, receiver)
}
