use core::ffi::CStr;

use flipperzero_sys::{self as sys, furi_thread_get_id, furi_thread_list_get_at};

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
    let thread_list = unsafe { sys::furi_thread_list_alloc() };

    unsafe { sys::furi_thread_enumerate(thread_list) };
    let n_threads = unsafe { sys::furi_thread_list_size(thread_list) };

    'outer: loop {
        for i in 0..n_threads {
            let item = unsafe { furi_thread_list_get_at(thread_list, i) };
            let thread_id = unsafe { furi_thread_get_id((*item).thread) };

            if thread_id == cur_thread_id {
                // Ignore current thread
                continue;
            }

            let thread_app_id = unsafe { CStr::from_ptr((*item).app_id) };

            if thread_app_id != app_id {
                // Ignore threads of other apps
                continue;
            }

            let thread_name = unsafe { CStr::from_ptr(sys::furi_thread_get_name(thread_id)) };

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

    unsafe { sys::furi_thread_list_free(thread_list) };

    unsafe {
        sys::furi_log_print_format(
            sys::FuriLogLevel_FuriLogLevelDebug,
            c"flipperzero-rt".as_ptr(),
            c"All FAP threads completed".as_ptr(),
        );
    }
}
