//! Low-level bindings to the LF-RFID API.

pub mod worker {
    use crate::opaque;
    use crate::toolbox::protocols;

    opaque!(Worker);

    extern "C" {
        #[link_name = "lfrfid_worker_alloc"]
        pub fn lfrfid_worker_alloc(dict: *mut protocols::Dict) -> *mut Worker;
        #[link_name = "lfrfid_worker_free"]
        pub fn lfrfid_worker_free(worker: *mut Worker);
        #[link_name = "lfrfid_worker_start_thread"]
        pub fn lfrfid_worker_start_thread(worker: *mut Worker);
        #[link_name = "lfrfid_worker_stop_thread"]
        pub fn lfrfid_worker_stop_thread(worker: *mut Worker);

        #[link_name = "lfrfid_worker_emulate_start"]
        pub fn lfrfid_worker_emulate_start(worker: *mut Worker, protocol_id: protocols::ProtocolId);
        #[link_name = "lfrfid_worker_stop"]
        pub fn lfrfid_worker_stop(worker: *mut Worker);
    }
}
