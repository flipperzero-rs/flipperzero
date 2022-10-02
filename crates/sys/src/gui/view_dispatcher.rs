//! Low-level bindings to the View Dispatcher API.

use core::ffi::c_void;

use crate::opaque;

opaque!(ViewDispatcher);

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Type {
    Desktop,
    Window,
    Fullscreen,
}

pub type NavigationEventCallback = extern "C" fn(*mut c_void);
pub type CustomEventCallback = extern "C" fn(*mut c_void, u32);
pub type TickEventCallback = extern "C" fn(*mut c_void);

extern "C" {
    #[link_name = "view_dispatcher_alloc"]
    pub fn alloc() -> *mut ViewDispatcher;
    #[link_name = "view_dispatcher_free"]
    pub fn free(view_dispatcher: *mut ViewDispatcher);
    #[link_name = "view_dispatcher_enable_queue"]
    pub fn enable_queue(view_dispatcher: *mut ViewDispatcher);
    #[link_name = "view_dispatcher_attach_to_gui"]
    pub fn attach_to_gui(
        view_dispatcher: *mut ViewDispatcher,
        gui: *mut super::Gui,
        attach_type: Type,
    );

    #[link_name = "view_dispatcher_send_custom_event"]
    pub fn send_custom_event(view_dispatcher: *mut ViewDispatcher, event: u32);

    #[link_name = "view_dispatcher_set_custom_event_callback"]
    pub fn set_custom_event_callback(
        view_dispatcher: *mut ViewDispatcher,
        callback: CustomEventCallback,
    );
    #[link_name = "view_dispatcher_set_navigation_event_callback"]
    pub fn set_navigation_event_callback(
        view_dispatcher: *mut ViewDispatcher,
        callback: NavigationEventCallback,
    );
    #[link_name = "view_dispatcher_set_tick_event_callback"]
    pub fn set_tick_event_callback(
        view_dispatcher: *mut ViewDispatcher,
        callback: TickEventCallback,
        tick_period: u32,
    );
    #[link_name = "view_dispatcher_set_event_callback_context"]
    pub fn set_event_callback_context(view_dispatcher: *mut ViewDispatcher, context: *mut c_void);

    #[link_name = "view_dispatcher_run"]
    pub fn run(view_dispatcher: *mut ViewDispatcher);
    #[link_name = "view_dispatcher_stop"]
    pub fn stop(view_dispatcher: *mut ViewDispatcher);

    #[link_name = "view_dispatcher_add_view"]
    pub fn add_view(
        view_dispatcher: *mut ViewDispatcher,
        view_id: u32,
        view: *mut super::view::View,
    );
    #[link_name = "view_dispatcher_remove_view"]
    pub fn remove_view(view_dispatcher: *mut ViewDispatcher, view_id: u32);
    #[link_name = "view_dispatcher_switch_to_view"]
    pub fn switch_to_view(view_dispatcher: *mut ViewDispatcher, view_id: u32);
}
