use core::ffi::CStr;
use flipperzero_sys as sys;

pub use self::list::ThreadList;

mod list;

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
    let mut thread_list = ThreadList::new();

    'outer: loop {
        // FIXME: handle error
        let _ = thread_list.try_enumerate_into();

        for thread in &thread_list {
            // SAFETY: the thread should be alive at this point
            let thread_id = unsafe { thread.id() };
            if thread_id == cur_thread_id {
                continue;
            }

            // SAFETY: the thread should be alive at this point
            let thread_app_id = unsafe { thread.app_id() };
            if thread_app_id != app_id {
                continue;
            }

            // SAFETY: the thread should be alive at this point
            let thread_name = unsafe { thread.name() };

            if thread_name.to_bytes().ends_with(b"Srv") {
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

pub fn enumerate() -> Option<ThreadList> {
    let mut thread_list = ThreadList::new();
    match thread_list.try_enumerate_into() {
        Ok(()) => Some(thread_list),
        Err(()) => None,
    }
}
