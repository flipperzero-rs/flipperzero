//! GUI APIs

use crate::canvas::Canvas;
use crate::view_port::{ViewPort, ViewPortCallbacks};
use core::ffi::c_char;
use core::fmt::Debug;
use flipperzero_sys::{self as sys, furi::UnsafeRecord, Gui as SysGui, GuiLayer as SysGuiLayer};

/// System ViewPort.
pub struct Gui {
    gui: UnsafeRecord<SysGui>,
}

impl Gui {
    /// Furi record corresponding to GUI.
    pub const RECORD: *const c_char = sys::c_string!("gui");

    pub fn new() -> Self {
        // SAFETY: `RECORD` is a constant
        let gui = unsafe { UnsafeRecord::open(Self::RECORD) };

        Self { gui }
    }

    pub fn add_view_port<VPC: ViewPortCallbacks>(
        &mut self,
        view_port: ViewPort<VPC>,
        layer: GuiLayer,
    ) -> GuiViewPort<'_, VPC> {
        let gui = self.gui.as_raw();
        let view_port_ptr = view_port.as_raw();
        let layer = layer.into();

        // SAFETY: all pointers are valid and `view_port` outlives this `Gui`
        unsafe { sys::gui_add_view_port(gui, view_port_ptr, layer) };

        GuiViewPort {
            parent: self,
            view_port,
        }
    }

    pub fn get_frame_buffer_size(&self) -> usize {
        let gui = self.gui.as_raw();
        // SAFETY: `gui` is always a valid pointer
        unsafe { sys::gui_get_framebuffer_size(gui) }
    }

    pub fn set_lockdown(&self, lockdown: bool) {
        let gui = self.gui.as_raw();
        // SAFETY: `gui` is always a valid pointer
        unsafe { sys::gui_set_lockdown(gui, lockdown) }
    }

    // TODO: separate `GuiCanvas` (locking the parent)
    //  and `Canvas` (independent of the parent)
    pub fn direct_draw_acquire(&self) -> Canvas<'_> {
        let gui = self.gui.as_raw();

        // SAFETY: `gui` is always a valid pointer
        // let canvas = unsafe { sys::gui_direct_draw_acquire(gui) }
        let canvas = unimplemented!("");

        // SAFETY: `self` os the parent of `canvas`
        // and `canvas` is a freshly created valid pointer
        // unsafe { Canvas::from_raw(self, canvas) }
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
        let gui = self.parent.gui.as_raw();
        let view_port = self.view_port.as_raw();

        // # SAFETY: `self.parent` outlives this `GuiVewPort`
        unsafe { sys::gui_view_port_send_to_front(gui, view_port) };
    }

    // FIXME(Coles): `gui_view_port_send_to_back` is not present in bindings
    // pub fn send_to_back(&mut self) {
    //     let gui = self.gui.as_raw();
    //     let view_port = self.view_port.as_raw();
    //
    //     unsafe { sys::gui_view_port_send_to_back(gui, view_port) };
    // }
}

impl<VPC: ViewPortCallbacks> Drop for GuiViewPort<'_, VPC> {
    fn drop(&mut self) {
        let gui = self.parent.gui.as_raw();
        let view_port = self.view_port().as_raw();

        unsafe { sys::gui_remove_view_port(gui, view_port) }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum GuiLayer {
    Desktop,
    Window,
    StatusBarLeft,
    StatusBarRight,
    Fullscreen,
}

#[derive(Clone, Copy, Debug)]
pub enum FromSysGuiLayerError {
    Max,
    Invalid(SysGuiLayer),
}

impl TryFrom<SysGuiLayer> for GuiLayer {
    type Error = FromSysGuiLayerError;

    fn try_from(value: SysGuiLayer) -> Result<Self, Self::Error> {
        use sys::{
            GuiLayer_GuiLayerDesktop as SYS_GUI_LAYER_DESKTOP,
            GuiLayer_GuiLayerFullscreen as SYS_GUI_LAYER_FULLSCREN,
            GuiLayer_GuiLayerMAX as SYS_GUI_LAYER_MAX,
            GuiLayer_GuiLayerStatusBarLeft as SYS_GUI_LAYER_BAR_LEFT,
            GuiLayer_GuiLayerStatusBarRight as SYS_GUI_LAYER_BAR_RIGHT,
            GuiLayer_GuiLayerWindow as SYS_GUI_LAYER_WINDOW,
        };

        Ok(match value {
            SYS_GUI_LAYER_DESKTOP => Self::Desktop,
            SYS_GUI_LAYER_WINDOW => Self::Window,
            SYS_GUI_LAYER_BAR_LEFT => Self::StatusBarLeft,
            SYS_GUI_LAYER_BAR_RIGHT => Self::StatusBarRight,
            SYS_GUI_LAYER_FULLSCREN => Self::Fullscreen,
            SYS_GUI_LAYER_MAX => Err(Self::Error::Max)?,
            invalid => Err(Self::Error::Invalid(invalid))?,
        })
    }
}

impl From<GuiLayer> for SysGuiLayer {
    fn from(value: GuiLayer) -> Self {
        use sys::{
            GuiLayer_GuiLayerDesktop as SYS_GUI_LAYER_DESKTOP,
            GuiLayer_GuiLayerFullscreen as SYS_GUI_LAYER_FULLSCREN,
            GuiLayer_GuiLayerStatusBarLeft as SYS_GUI_LAYER_BAR_LEFT,
            GuiLayer_GuiLayerStatusBarRight as SYS_GUI_LAYER_BAR_RIGHT,
            GuiLayer_GuiLayerWindow as SYS_GUI_LAYER_WINDOW,
        };

        match value {
            GuiLayer::Desktop => SYS_GUI_LAYER_DESKTOP,
            GuiLayer::Window => SYS_GUI_LAYER_WINDOW,
            GuiLayer::StatusBarLeft => SYS_GUI_LAYER_BAR_LEFT,
            GuiLayer::StatusBarRight => SYS_GUI_LAYER_BAR_RIGHT,
            GuiLayer::Fullscreen => SYS_GUI_LAYER_FULLSCREN,
        }
    }
}

pub trait GuiCallbacks {
    fn on_draw(&mut self, _canvas: *mut sys::Canvas) {}
    fn on_input(&mut self, _event: *mut sys::InputEvent) {}
}
