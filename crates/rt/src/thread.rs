use core::ffi::CStr;
use core::ptr;

use flipperzero_sys as sys;

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

    const MAX_THREADS: usize = 32;
    let cur_thread_id = unsafe { sys::furi_thread_get_current_id() };
    let app_id = unsafe { CStr::from_ptr(sys::furi_thread_get_appid(cur_thread_id)) };
    let mut thread_ids: [sys::FuriThreadId; MAX_THREADS] = [ptr::null_mut(); MAX_THREADS];

    'outer: loop {
        let thread_count =
            unsafe { sys::furi_thread_enumerate(thread_ids.as_mut_ptr(), MAX_THREADS as u32) }
                as usize;

        for &thread_id in thread_ids[..thread_count].iter() {
            let thread_app_id = unsafe { CStr::from_ptr(sys::furi_thread_get_appid(thread_id)) };

            if thread_id == cur_thread_id || thread_app_id != app_id {
                // Ignore this thread or the threads of other apps
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

    unsafe {
        sys::furi_log_print_format(
            sys::FuriLogLevel_FuriLogLevelDebug,
            c"flipperzero-rt".as_ptr(),
            c"All FAP threads completed".as_ptr(),
        );
    }
}
