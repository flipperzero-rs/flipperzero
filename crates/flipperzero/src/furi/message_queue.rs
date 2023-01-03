use core::ffi::c_void;
use core::mem::size_of;
use core::ptr::NonNull;
use core::time::Duration;

use flipperzero_sys as sys;
use flipperzero_sys::furi::{duration_to_ticks, Status};

use crate::furi;

/// MessageQueue provides a safe wrapper around the furi message queue primitive.
pub struct MessageQueue<M: Sized> {
    raw: NonNull<sys::FuriMessageQueue>,
    _marker: core::marker::PhantomData<M>,
}

impl<M: Sized> MessageQueue<M> {
    /// Constructs a message queue with the given capacity.
    pub fn new(capacity: u32) -> Self {
        let message_size = size_of::<M>() as u32;
        // SAFETY: there are no expplicit size restrictions
        // and allocation will either succed or crash the application
        let raw = unsafe {
            NonNull::new_unchecked(sys::furi_message_queue_alloc(capacity, message_size))
        };
        Self {
            raw,
            _marker: core::marker::PhantomData::<M>,
        }
    }

    // Attempts to add the message to the end of the queue, waiting up to timeout ticks.
    pub fn put(&self, message: M, timeout: Duration) -> furi::Result<()> {
        // the value will be retrieved from the queue either explicitly or on queue drop
        // after which it will be dropped
        let mut message = core::mem::ManuallyDrop::new(message);
        let message = &mut message as *mut _ as *mut c_void;
        let timeout_ticks = sys::furi::duration_to_ticks(timeout);

        let raw = self.raw.as_ptr().cast();
        let status: Status =
            unsafe { sys::furi_message_queue_put(raw, message, timeout_ticks) }.into();

        status.err_or(())
    }

    // Attempts to read a message from the front of the queue within timeout ticks.
    pub fn get(&self, timeout: Duration) -> furi::Result<M> {
        let raw = self.raw.as_ptr();
        let timeout_ticks = duration_to_ticks(timeout);
        let mut out = core::mem::MaybeUninit::<M>::uninit();
        let out_ptr = out.as_mut_ptr().cast();
        // SAFETY: `raw` is always valid,
        // `out_ptr` is only used to write into is never read from (TODO: check correctness)
        let status: Status =
            unsafe { sys::furi_message_queue_get(raw, out_ptr, timeout_ticks) }.into();

        if status.is_ok() {
            Ok(unsafe { out.assume_init() })
        } else {
            Err(status)
        }
    }

    /// Returns the capacity of the queue.
    pub fn capacity(&self) -> u32 {
        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is always valid
        unsafe { sys::furi_message_queue_get_capacity(raw) }
    }

    /// Returns the number of elements in the queue.
    pub fn len(&self) -> u32 {
        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is always valid
        unsafe { sys::furi_message_queue_get_count(raw) }
    }

    /// Is the message queue empty?
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the number of free slots in the queue.
    pub fn space(&self) -> u32 {
        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is always valid
        unsafe { sys::furi_message_queue_get_space(raw) }
    }
}

impl<M: Sized> Drop for MessageQueue<M> {
    fn drop(&mut self) {
        // Drain any elements from the message queue, so any
        // drop handlers on the message element get called.
        while !self.is_empty() {
            match self.get(Duration::MAX) {
                Ok(msg) => drop(msg),
                Err(_) => break, // we tried
            }
        }

        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is always valid
        unsafe { sys::furi_message_queue_free(raw) }
    }
}
