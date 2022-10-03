//! Low-level bindings to the variable-item list view.

use core::ffi::{c_char, c_void};
use crate::gui::view::View;
use crate::opaque;

opaque!(VariableItem);
opaque!(VariableItemList);

pub type ChangeCallback = extern "C" fn(*mut VariableItem);
pub type EnterCallback = extern "C" fn(*mut c_void, u32);

extern "C" {
    #[link_name = "variable_item_list_alloc"]
    pub fn alloc() -> *mut VariableItemList;
    #[link_name = "variable_item_list_free"]
    pub fn free(vil: *mut VariableItemList);
    #[link_name = "variable_item_list_get_view"]
    pub fn get_view(vil: *mut VariableItemList) -> *mut View;

    #[link_name = "variable_item_list_add"]
    pub fn add_item(vil: *mut VariableItemList, label: *const c_char, count: u8, change_callback: ChangeCallback, context: *mut c_void) -> *mut VariableItem;
    #[link_name = "variable_item_list_get_selected_item_index"]
    pub fn selected_index(vil: *mut VariableItemList) -> u8;

    #[link_name = "variable_item_list_set_enter_callback"]
    pub fn set_enter_callback(vil: *mut VariableItemList, callback: EnterCallback);

    #[link_name = "variable_item_get_current_value_index"]
    pub fn get_current_value_index(vi: *mut VariableItem) -> u8;
    #[link_name = "variable_item_set_current_value_index"]
    pub fn set_current_value_index(vi: *mut VariableItem, idx: u8);
    #[link_name = "variable_item_set_current_value_text"]
    pub fn set_current_value_text(vi: *mut VariableItem, label: *const c_char);
    #[link_name = "variable_item_get_context"]
    pub fn get_context(vi: *mut VariableItem) -> *mut c_void;
}