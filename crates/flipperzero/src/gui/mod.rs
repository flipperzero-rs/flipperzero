//! GUI service.

pub mod canvas;
pub mod icon;
pub mod icon_animation;
pub mod view;
pub mod view_port;
pub mod xbm;

use crate::{
    gui::{
        canvas::CanvasView,
        view_port::{ViewPort, ViewPortCallbacks},
    },
    input::InputEvent,
};
use core::{
    ffi::c_char,
    fmt::Debug,
    ops::{Deref, DerefMut},
};
use flipperzero_sys::{
    self as sys, furi::UnsafeRecord, Canvas as SysCanvas, Gui as SysGui, GuiLayer as SysGuiLayer,
};

/// System Gui wrapper.
pub struct Gui {
    raw: UnsafeRecord<SysGui>,
}

impl Gui {
    /// Furi record corresponding to GUI.
    pub const RECORD: *const c_char = sys::c_string!("gui");

    pub fn new() -> Self {
        // SAFETY: `RECORD` is a constant
        let gui = unsafe { UnsafeRecord::open(Self::RECORD) };

        Self { raw: gui }
    }

    pub fn as_raw(&self) -> *mut SysGui {
        self.raw.as_raw()
    }

    pub fn add_view_port<VPC: ViewPortCallbacks>(
        &mut self,
        view_port: ViewPort<VPC>,
        layer: GuiLayer,
    ) -> GuiViewPort<'_, VPC> {
        let raw = self.as_raw();
        let view_port_ptr = view_port.as_raw();
        let layer = layer.into();

        // SAFETY: all pointers are valid and `view_port` outlives this `Gui`
        unsafe { sys::gui_add_view_port(raw, view_port_ptr, layer) };

        GuiViewPort {
            parent: self,
            view_port,
        }
    }

    pub fn get_frame_buffer_size(&self) -> usize {
        let raw = self.as_raw();
        // SAFETY: `raw` is always a valid pointer
        unsafe { sys::gui_get_framebuffer_size(raw) }
    }

    pub fn set_lockdown(&self, lockdown: bool) {
        let raw = self.raw.as_raw();
        // SAFETY: `raw` is always a valid pointer
        unsafe { sys::gui_set_lockdown(raw, lockdown) }
    }

    // TODO: separate `GuiCanvas` (locking the parent)
    //  and `Canvas` (independent of the parent)
    pub fn direct_draw_acquire(&mut self) -> ExclusiveCanvas<'_> {
        let raw = self.as_raw();

        // SAFETY: `raw` is always a valid pointer
        let canvas = unsafe { CanvasView::from_raw(sys::gui_direct_draw_acquire(raw)) };

        ExclusiveCanvas { gui: self, canvas }
    }

    // TODO: canvas method
    // TODO: callback methods
}

impl Default for Gui {
    fn default() -> Self {
        Self::new()
    }
}

/// `ViewPort` bound to a `Gui`.
pub struct GuiViewPort<'a, VPC: ViewPortCallbacks> {
    parent: &'a Gui,
    view_port: ViewPort<VPC>,
}

impl<'a, VPC: ViewPortCallbacks> GuiViewPort<'a, VPC> {
    pub fn view_port(&self) -> &ViewPort<VPC> {
        &self.view_port
    }

    pub fn view_port_mut(&mut self) -> &mut ViewPort<VPC> {
        &mut self.view_port
    }

    pub fn send_to_front(&mut self) {
        let gui = self.parent.raw.as_raw();
        let view_port = self.view_port.as_raw();

        // SAFETY: `self.parent` outlives this `GuiVewPort`
        unsafe { sys::gui_view_port_send_to_front(gui, view_port) };
    }

    // FIXME: `gui_view_port_send_to_back` is not present in bindings
    // pub fn send_to_back(&mut self) {
    //     let gui = self.gui.as_raw();
    //     let view_port = self.view_port.as_raw();
    //
    //     unsafe { sys::gui_view_port_send_to_back(gui, view_port) };
    // }
}

impl<VPC: ViewPortCallbacks> Drop for GuiViewPort<'_, VPC> {
    fn drop(&mut self) {
        let gui = self.parent.raw.as_raw();
        let view_port = self.view_port().as_raw();

        // SAFETY: `gui` and `view_port` are valid pointers
        // and this view port should have been added to the gui on creation
        unsafe { sys::gui_remove_view_port(gui, view_port) }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum GuiLayer {
    Desktop,
    Window,
    StatusBarLeft,
    StatusBarRight,
    Fullscreen,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum FromSysGuiLayerError {
    Max,
    Invalid(SysGuiLayer),
}

impl TryFrom<SysGuiLayer> for GuiLayer {
    type Error = FromSysGuiLayerError;

    fn try_from(value: SysGuiLayer) -> Result<Self, Self::Error> {
        Ok(match value {
            sys::GuiLayer_GuiLayerDesktop => Self::Desktop,
            sys::GuiLayer_GuiLayerWindow => Self::Window,
            sys::GuiLayer_GuiLayerStatusBarLeft => Self::StatusBarLeft,
            sys::GuiLayer_GuiLayerStatusBarRight => Self::StatusBarRight,
            sys::GuiLayer_GuiLayerFullscreen => Self::Fullscreen,
            sys::GuiLayer_GuiLayerMAX => Err(Self::Error::Max)?,
            invalid => Err(Self::Error::Invalid(invalid))?,
        })
    }
}

impl From<GuiLayer> for SysGuiLayer {
    fn from(value: GuiLayer) -> Self {
        match value {
            GuiLayer::Desktop => sys::GuiLayer_GuiLayerDesktop,
            GuiLayer::Window => sys::GuiLayer_GuiLayerWindow,
            GuiLayer::StatusBarLeft => sys::GuiLayer_GuiLayerStatusBarLeft,
            GuiLayer::StatusBarRight => sys::GuiLayer_GuiLayerStatusBarRight,
            GuiLayer::Fullscreen => sys::GuiLayer_GuiLayerFullscreen,
        }
    }
}

pub trait GuiCallbacks {
    fn on_draw(&mut self, _canvas: *mut SysCanvas) {}
    fn on_input(&mut self, _event: InputEvent) {}
}

impl GuiCallbacks for () {}

/// Exclusively accessible canvas.
pub struct ExclusiveCanvas<'a> {
    gui: &'a mut Gui,
    canvas: CanvasView<'a>,
}

impl Drop for ExclusiveCanvas<'_> {
    fn drop(&mut self) {
        let gui = self.gui.as_raw();
        // SAFETY: this instance should have been created from `gui`
        // using `gui_direct_draw_acquire`
        // and will no longer be available since it is dropped
        unsafe { sys::gui_direct_draw_release(gui) };
    }
}

impl<'a> Deref for ExclusiveCanvas<'a> {
    type Target = CanvasView<'a>;

    fn deref(&self) -> &Self::Target {
        &self.canvas
    }
}

impl<'a> DerefMut for ExclusiveCanvas<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.canvas
    }
}
