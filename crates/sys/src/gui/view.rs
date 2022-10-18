//! Low-level bindings to the View API.

use crate::opaque;
use core::ffi::c_void;

opaque!(View);
opaque!(ViewModel);

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ModelType {
    None,
    LockFree,
    Locking,
}


pub type DrawCallback = extern "C" fn(*mut super::canvas::Canvas, *mut c_void);
pub type InputCallback = extern "C" fn(*mut super::InputEvent, *mut c_void) -> bool;
pub type CustomCallback = extern "C" fn(u32, *mut c_void) -> bool;

extern "C" {
    #[link_name = "view_alloc"]
    pub fn alloc() -> *mut View;
    #[link_name = "view_free"]
    pub fn free(view: *mut View);
    #[link_name = "view_set_context"]
    pub fn set_context(view: *mut View, context: *mut c_void);

    #[link_name = "view_set_draw_callback"]
    pub fn set_draw_callback(view: *mut View, callback: DrawCallback);
    #[link_name = "view_set_input_callback"]
    pub fn set_input_callback(view: *mut View, callback: InputCallback);
    #[link_name = "view_set_custom_callback"]
    pub fn set_custom_callback(view: *mut View, callback: CustomCallback);

    #[link_name = "view_allocate_model"]
    pub fn allocate_model(view: *mut View, model_type: ModelType, size: usize) -> *mut ViewModel;
    #[link_name = "view_free_model"]
    pub fn free_model(view: *mut View);
    #[link_name = "view_get_model"]
    pub fn get_model(view: *mut View) -> *mut c_void;
}