//! Low-level bindings to the Protocols API.

use core::ffi::c_char;

use crate::opaque;

opaque!(ProtocolsArray);

/// The number of entries in LFRFID_PROTOCOLS.
pub const PROTOCOLS_MAX: usize = 16; // SDK version 1.13

opaque!(Dict);

#[repr(transparent)]
pub struct ProtocolId(i32);

extern "C" {
    #[link_name = "lfrfid_protocols"]
    pub static LFRFID_PROTOCOLS: ProtocolsArray;

    #[link_name = "protocol_dict_alloc"]
    pub fn protocol_dict_alloc(
        protocols: &'static ProtocolsArray,
        num_protocols: usize,
    ) -> *mut Dict;
    #[link_name = "protocol_dict_free"]
    pub fn protocol_dict_free(dict: *mut Dict);
    #[link_name = "protocol_dict_get_protocol_by_name"]
    pub fn protocol_dict_get_protocol_by_name(dict: *mut Dict, name: *const c_char) -> ProtocolId;
    #[link_name = "protocol_dict_set_data"]
    pub fn protocol_dict_set_data(
        dict: *mut Dict,
        protocol_id: ProtocolId,
        data: *const u8,
        data_len: usize,
    );
}
