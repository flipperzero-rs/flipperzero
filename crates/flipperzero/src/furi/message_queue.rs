use core::ffi::c_void;
use flipperzero_sys::furi::message_queue;
use crate::furi::Result;

/// MessageQueue provides a safe wrapper around the furi message queue primitive.
pub struct MessageQueue<M: Sized> {
    hnd: *const message_queue::FuriMessageQueue,
    _marker: core::marker::PhantomData<M>,
}

impl<M: Sized> MessageQueue<M> {
    /// Constructs a message queue with the given capacity.
    pub fn new(capacity: u32) -> Self {
        Self {
            hnd: unsafe { message_queue::alloc(capacity, core::mem::size_of::<M>()) },
            _marker: core::marker::PhantomData::<M>,
        }
    }

    // Attempts to add the message to the end of the queue, waiting up to timeout ticks.
    pub fn put(&self, mut msg: M, timeout_ticks: u32) -> Result<()> {
        let status = unsafe {
            message_queue::put(self.hnd, &mut msg as *mut _ as *const c_void, timeout_ticks)
        };

        match status.is_ok() {
            true => Ok(()),
            _ => Err(status),
        }
    }

    // Attempts to read a message from the front of the queue within timeout ticks.
    pub fn get(&self, timeout_ticks: u32) -> Result<M> {
        let mut out = core::mem::MaybeUninit::<M>::uninit();
        let status = unsafe {
            message_queue::get(self.hnd, out.as_mut_ptr() as *mut c_void, timeout_ticks)
        };

        match status.is_ok() {
            true => Ok(unsafe { out.assume_init() }),
            _ => Err(status),
        }
    }

    /// Returns the capacity of the queue.
    pub fn capacity(&self) -> u32 {
        unsafe { message_queue::capacity(self.hnd) }
    }

    /// Returns the number of elements in the queue.
    pub fn len(&self) -> u32 {
        unsafe { message_queue::count(self.hnd) }
    }

    /// Returns the number of free slots in the queue.
    pub fn space(&self) -> u32 {
        unsafe { message_queue::space(self.hnd) }
    }
}

impl<M: Sized> Drop for MessageQueue<M> {
    fn drop(&mut self) {
        unsafe {
            message_queue::free(self.hnd)
        }
    }
}