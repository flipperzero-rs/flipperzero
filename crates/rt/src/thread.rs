use core::ffi::CStr;

use flipperzero_sys as sys;

#[derive(Debug)]
struct ThreadItem {
    inner: sys::FuriThreadListItem,
}

impl ThreadItem {
    fn new(inner: sys::FuriThreadListItem) -> Self {
        Self { inner }
    }

    /// Get the thread ID
    fn id(&self) -> sys::FuriThreadId {
        unsafe { sys::furi_thread_get_id(self.inner.thread) }
    }

    /// Get the thread name
    fn name(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.inner.name) }
    }

    /// Get the app ID
    fn app_id(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.inner.app_id) }
    }
}

#[derive(Debug)]
struct ThreadList {
    inner: *mut sys::FuriThreadList,
}

impl ThreadList {
    /// Create a new thread list
    fn new() -> Self {
        let inner = unsafe { sys::furi_thread_list_alloc() };
        Self { inner }
    }

    /// Get the number of threads in the list
    fn size(&self) -> usize {
        unsafe { sys::furi_thread_list_size(self.inner) }
    }

    /// Get the thread at the given index
    fn get_at(&self, index: usize) -> Option<sys::FuriThreadListItem> {
        let thread = unsafe { sys::furi_thread_list_get_at(self.inner, index) };
        if thread.is_null() {
            None
        } else {
            Some(unsafe { *thread })
        }
    }

    /// Load the list of threads
    fn load(&self) {
        unsafe {
            sys::furi_thread_enumerate(self.inner);
        }
    }

    /// Get an iterator over the threads in the list
    fn iter(&self) -> ThreadListIterator {
        ThreadListIterator {
            list: self,
            size: self.size(),
            index: 0,
        }
    }
}

/// Iterator over the threads in a thread list
#[derive(Debug)]
struct ThreadListIterator<'a> {
    list: &'a ThreadList,
    index: usize,
    size: usize,
}

impl<'a> Iterator for ThreadListIterator<'a> {
    type Item = ThreadItem;

    /// Get the next thread in the list
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.size {
            let item = self.list.get_at(self.index)?;
            self.index += 1;
            return Some(ThreadItem::new(item));
        }

        None
    }
}

impl Drop for ThreadList {
    /// Free the thread list
    fn drop(&mut self) {
        unsafe {
            sys::furi_thread_list_free(self.inner);
        }
    }
}

/// Wait for threads with the same app ID as the current thread to finish.
///
/// We assume there are not more than 30 threads running in the background outside this
/// app, so we will observe at least one of any thread the app might have left running.
pub fn wait_for_completion() {
    unsafe {
        sys::furi_log_print_format(
            sys::FuriLogLevel_FuriLogLevelDebug,
            c"flipperzero-rt".as_ptr(),
            c"Waiting for FAP threads to complete...".as_ptr(),
        );
    }

    let cur_thread_id = unsafe { sys::furi_thread_get_current_id() };
    let app_id = unsafe { CStr::from_ptr(sys::furi_thread_get_appid(cur_thread_id)) };

    let thread_list = ThreadList::new();

    thread_list.load();

    'outer: loop {
        for thread_item in thread_list.iter() {
            if thread_item.id() == cur_thread_id || thread_item.app_id() != app_id {
                // Ignore this thread or the threads of other apps
                continue;
            }

            if thread_item.name().to_bytes().ends_with(b"Srv") {
                // This is a workaround for an issue where the current appid matches one
                // of the built-in service names (e.g. "gui"). Otherwise we will see the
                // service thread (e.g. "GuiSrv") and assume that it's one of our threads
                // that still needs to exit, thus causing the app to hang at exit.
                continue;
            }

            // There is a thread that is still running, so wait for it to exit...
            unsafe {
                sys::furi_delay_ms(10);
            }

            continue 'outer;
        }

        break;
    }

    unsafe {
        sys::furi_log_print_format(
            sys::FuriLogLevel_FuriLogLevelDebug,
            c"flipperzero-rt".as_ptr(),
            c"All FAP threads completed".as_ptr(),
        );
    }
}
