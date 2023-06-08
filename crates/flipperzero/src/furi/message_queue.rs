use core::ffi::c_void;
use core::ptr::NonNull;
use core::time::Duration;

use flipperzero_sys as sys;
use flipperzero_sys::furi::{duration_to_ticks, Status};

use crate::furi;

/// MessageQueue provides a safe wrapper around the furi message queue primitive.
pub struct MessageQueue<M: Sized> {
    hnd: NonNull<sys::FuriMessageQueue>,
    _marker: core::marker::PhantomData<M>,
}

impl<M: Sized> MessageQueue<M> {
    /// Constructs a message queue with the given capacity.
    pub fn new(capacity: usize) -> Self {
        Self {
            hnd: unsafe {
                NonNull::new_unchecked(sys::furi_message_queue_alloc(
                    capacity as u32,
                    core::mem::size_of::<M>() as u32,
                ))
            },
            _marker: core::marker::PhantomData::<M>,
        }
    }

    // Attempts to add the message to the end of the queue, waiting up to timeout ticks.
    pub fn put(&self, msg: M, timeout: Duration) -> furi::Result<()> {
        let mut msg = core::mem::ManuallyDrop::new(msg);
        let timeout_ticks = duration_to_ticks(timeout);

        let status: Status = unsafe {
            sys::furi_message_queue_put(
                self.hnd.as_ptr(),
                &mut msg as *mut _ as *const c_void,
                timeout_ticks,
            )
            .into()
        };

        status.err_or(())
    }

    // Attempts to read a message from the front of the queue within timeout ticks.
    pub fn get(&self, timeout: Duration) -> furi::Result<M> {
        let timeout_ticks = duration_to_ticks(timeout);
        let mut out = core::mem::MaybeUninit::<M>::uninit();
        let status: Status = unsafe {
            sys::furi_message_queue_get(
                self.hnd.as_ptr(),
                out.as_mut_ptr() as *mut c_void,
                timeout_ticks,
            )
            .into()
        };

        if status.is_ok() {
            Ok(unsafe { out.assume_init() })
        } else {
            Err(status)
        }
    }

    /// Returns the capacity of the queue.
    pub fn capacity(&self) -> usize {
        unsafe { sys::furi_message_queue_get_capacity(self.hnd.as_ptr()) as usize }
    }

    /// Returns the number of elements in the queue.
    pub fn len(&self) -> usize {
        unsafe { sys::furi_message_queue_get_count(self.hnd.as_ptr()) as usize }
    }

    /// Is the message queue empty?
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the number of free slots in the queue.
    pub fn space(&self) -> usize {
        unsafe { sys::furi_message_queue_get_space(self.hnd.as_ptr()) as usize }
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

        unsafe { sys::furi_message_queue_free(self.hnd.as_ptr()) }
    }
}

#[flipperzero_test::tests]
mod tests {
    use core::time::Duration;

    use flipperzero_sys::furi::Status;

    use super::MessageQueue;

    #[test]
    fn capacity() {
        let queue = MessageQueue::new(3);
        assert_eq!(queue.len(), 0);
        assert_eq!(queue.space(), 3);
        assert_eq!(queue.capacity(), 3);

        // Adding a message to the queue should consume capacity.
        queue.put(2, Duration::from_millis(1)).unwrap();
        assert_eq!(queue.len(), 1);
        assert_eq!(queue.space(), 2);
        assert_eq!(queue.capacity(), 3);

        // We should be able to fill the queue to capacity.
        queue.put(4, Duration::from_millis(1)).unwrap();
        queue.put(6, Duration::from_millis(1)).unwrap();
        assert_eq!(queue.len(), 3);
        assert_eq!(queue.space(), 0);
        assert_eq!(queue.capacity(), 3);

        // Attempting to add another message should time out.
        assert_eq!(
            queue.put(7, Duration::from_millis(1)),
            Err(Status::ERR_TIMEOUT),
        );

        // Removing a message from the queue frees up capacity.
        assert_eq!(queue.get(Duration::from_millis(1)), Ok(2));
        assert_eq!(queue.len(), 2);
        assert_eq!(queue.space(), 1);
        assert_eq!(queue.capacity(), 3);
    }
}
