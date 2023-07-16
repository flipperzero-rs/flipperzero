//! Canvas-related APIs allowing to draw on it.

mod align;
mod canvas_direction;
mod color;
mod font;
mod font_parameters;

use core::{
    ffi::{c_char, CStr},
    marker::PhantomData,
    num::NonZeroU8,
    ops::Deref,
    ptr::NonNull,
};

pub use align::*;
pub use canvas_direction::*;
pub use color::*;
use flipperzero_sys::{
    self as sys, Canvas as SysCanvas, CanvasFontParameters as SysCanvasFontParameters,
};
pub use font::*;
pub use font_parameters::*;

use crate::gui::{
    icon::Icon,
    icon_animation::{IconAnimation, IconAnimationCallbacks},
};
#[cfg(feature = "xbm")]
use crate::xbm::XbmImage;

/// System Canvas view.
pub struct CanvasView<'a> {
    raw: NonNull<SysCanvas>,
    _lifetime: PhantomData<&'a SysCanvas>,
}

impl CanvasView<'_> {
    /// Construct a `CanvasView` from a raw pointer.
    ///
    /// # Safety
    ///
    /// `raw` should be a valid non-null pointer to [`sys::Canvas`]
    /// and the lifetime should be outlived by `raw` validity scope.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use flipperzero::gui::canvas::CanvasView;
    /// # let canvas_ptr: *mut flipperzero_sys::Canvas = todo!();
    /// // wrap a raw pointer to a canvas
    /// let canvas = unsafe { CanvasView::from_raw(canvas_ptr) };
    /// ```
    pub unsafe fn from_raw(raw: *mut SysCanvas) -> Self {
        Self {
            // SAFETY: caller should provide a valid pointer
            raw: unsafe { NonNull::new_unchecked(raw) },
            _lifetime: PhantomData,
        }
    }

    /// Resets canvas drawing tools configuration.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use flipperzero::gui::canvas::{CanvasView, Color};
    /// # let mut canvas: CanvasView<'static> = todo!();
    /// // change canvas color and use it for drawing
    /// canvas.set_color(Color::Xor);
    /// canvas.draw_circle(10, 10, 5);
    /// // reset canvas options and use defaults for drawing
    /// canvas.reset();
    /// canvas.draw_circle(20, 20, 5);
    /// ```
    pub fn reset(&mut self) {
        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is always valid
        unsafe { sys::canvas_reset(raw) };
    }

    /// Commits canvas sending its buffer to display.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use flipperzero::gui::canvas::{CanvasView, Color};
    /// # let mut canvas: CanvasView<'static> = todo!();
    /// // perform some draw operations on the canvas
    /// canvas.draw_frame(0, 0, 51, 51);
    /// canvas.draw_circle(25, 25, 10);
    /// // commit changes
    /// canvas.commit();
    /// ```
    pub fn commit(&mut self) {
        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is always valid
        unsafe { sys::canvas_commit(raw) };
    }

    pub fn width(&self) -> NonZeroU8 {
        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is always valid
        unsafe { sys::canvas_width(raw) }
            .try_into()
            .expect("`canvas_width` should produce a positive value")
    }

    pub fn height(&self) -> NonZeroU8 {
        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is always valid
        unsafe { sys::canvas_height(raw) }
            .try_into()
            .expect("`canvas_height` should produce a positive value")
    }

    pub fn current_font_height(&self) -> NonZeroU8 {
        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is always valid
        unsafe { sys::canvas_current_font_height(raw) }
            .try_into()
            .expect("`canvas_current_font_height` should produce a positive value")
    }

    pub fn get_font_params(&self, font: Font) -> OwnedCanvasFontParameters<'_> {
        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is always valid
        let font = font.into();
        // SAFETY: `raw` is a valid pointer
        // and `font` is guaranteed to be a valid value by `From` implementation
        // `cast_mut` is required since `NonNull` can only be created froma mut-pointer
        let raw = unsafe { sys::canvas_get_font_params(raw, font) }.cast_mut();
        // SAFETY: `raw` is a valid pointer
        let raw = unsafe { NonNull::new_unchecked(raw) };
        OwnedCanvasFontParameters {
            raw,
            _parent: PhantomData,
        }
    }

    pub fn clear(&mut self) {
        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is always valid
        unsafe { sys::canvas_clear(raw) };
    }

    pub fn set_color(&mut self, color: Color) {
        let raw = self.raw.as_ptr();
        let color = color.into();
        // SAFETY: `raw` is always valid
        // and `font` is guaranteed to be a valid value by `From` implementation
        unsafe { sys::canvas_set_color(raw, color) };
    }

    pub fn set_font_direction(&mut self, font_direction: CanvasDirection) {
        let raw = self.raw.as_ptr();
        let font_direction = font_direction.into();
        // SAFETY: `raw` is always valid
        // and `font_direction` is guaranteed to be a valid value by `From` implementation
        unsafe { sys::canvas_set_font_direction(raw, font_direction) };
    }

    pub fn invert_color(&mut self) {
        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is always valid
        unsafe { sys::canvas_invert_color(raw) };
    }

    pub fn set_font(&mut self, font: Font) {
        let raw = self.raw.as_ptr();
        let font = font.into();
        // SAFETY: `raw` is always valid
        // and `font` is guaranteed to be a valid value by `From` implementation
        unsafe { sys::canvas_set_font(raw, font) };
    }

    pub fn draw_str(&mut self, x: u8, y: u8, string: impl AsRef<CStr>) {
        let raw = self.raw.as_ptr();
        let string = string.as_ref().as_ptr();
        // SAFETY: `raw` is always valid
        // and `string` is guaranteed to be a valid pointer since it was created from `CStr`
        unsafe { sys::canvas_draw_str(raw, x, y, string) };
    }

    pub fn draw_str_aligned(
        &mut self,
        x: u8,
        y: u8,
        horizontal: Align,
        vertical: Align,
        str: impl AsRef<CStr>,
    ) {
        let raw = self.raw.as_ptr();
        let horizontal = horizontal.into();
        let vertical = vertical.into();
        let str = str.as_ref().as_ptr();
        // SAFETY: `raw` is always valid,
        // `horixontal` and `vertival` are guaranteed to be valid by `From` implementation
        // and `text` is guaranteed to be a valid pointer since it was created from `CStr`
        unsafe { sys::canvas_draw_str_aligned(raw, x, y, horizontal, vertical, str) };
    }

    // note: for some reason, this mutates internal state
    pub fn string_width(&mut self, string: impl AsRef<CStr>) -> u16 {
        let raw = self.raw.as_ptr();
        let string = string.as_ref().as_ptr();
        // SAFETY: `raw` is always valid
        // and `string` is guaranteed to be a valid pointer since it was created from `CStr`
        unsafe { sys::canvas_string_width(raw, string) }
    }

    // note: for some reason, this mutates internal state
    pub fn glyph_width(&mut self, glyph: c_char) -> u8 {
        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is always valid
        unsafe { sys::canvas_glyph_width(raw, glyph) }
    }

    // Note: FURI is guaranteed to correctly handle out-of-bounds draws
    // so we don't need to check the bounds

    // TODO `canvas_draw_bitmap` compressed bitmap support

    pub fn draw_icon_animation<'a, 'b: 'a>(
        &'a mut self,
        x: u8,
        y: u8,
        icon_animation: &'b IconAnimation<'_, impl IconAnimationCallbacks>,
    ) {
        let raw = self.raw.as_ptr();
        let icon_animation = icon_animation.as_raw();
        // SAFETY: `raw` is always valid
        // and `icon_animation` is always valid and outlives this canvas view
        unsafe { sys::canvas_draw_icon_animation(raw, x, y, icon_animation) }
    }

    pub fn draw_icon<'a, 'b: 'a>(&'a mut self, x: u8, y: u8, animation: &'b Icon) {
        let raw = self.raw.as_ptr();
        let icon = animation.as_raw();
        // SAFETY: `raw` is always valid
        // and `icon` is always valid and outlives this canvas view
        unsafe { sys::canvas_draw_icon(raw, x, y, icon) }
    }

    #[cfg(feature = "xbm")]
    pub fn draw_xbm(&mut self, x: u8, y: u8, xbm: &XbmImage<impl Deref<Target = [u8]>>) {
        let raw = self.raw.as_ptr();
        let width = xbm.width();
        let height = xbm.height();

        let data = xbm.data().as_ptr();

        // SAFETY: `raw` is always valid
        // and `data` is always valid and does not have to outlive the view
        // as it is copied
        unsafe { sys::canvas_draw_xbm(raw, x, y, width, height, data) };
    }

    // TODO:
    // - `canvas_draw_icon` icon lifetimes

    // TODO: decide if we want to pack x-y pairs into tuples

    pub fn draw_dot(&mut self, x: u8, y: u8) {
        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is always valid
        unsafe { sys::canvas_draw_dot(raw, x, y) }
    }

    // TODO: do `width` and `height` have to be non-zero
    pub fn draw_box(&mut self, x: u8, y: u8, width: u8, height: u8) {
        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is always valid
        unsafe { sys::canvas_draw_box(raw, x, y, width, height) }
    }

    // TODO: do `width` and `height` have to be non-zero
    pub fn draw_frame(&mut self, x: u8, y: u8, width: u8, height: u8) {
        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is always valid
        unsafe { sys::canvas_draw_frame(raw, x, y, width, height) }
    }

    // TODO: do `x2` and `y2` have to be non-zero
    pub fn draw_line(&mut self, x1: u8, y1: u8, x2: u8, y2: u8) {
        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is always valid
        unsafe { sys::canvas_draw_line(raw, x1, y1, x2, y2) }
    }

    // TODO: does `radius` have to be non-zero
    pub fn draw_circle(&mut self, x: u8, y: u8, radius: u8) {
        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is always valid
        unsafe { sys::canvas_draw_circle(raw, x, y, radius) }
    }

    // TODO: does `radius` have to be non-zero
    pub fn draw_disc(&mut self, x: u8, y: u8, radius: u8) {
        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is always valid
        unsafe { sys::canvas_draw_disc(raw, x, y, radius) }
    }

    // TODO: do `base` and `height` have to be non-zero
    pub fn draw_triangle(
        &mut self,
        x: u8,
        y: u8,
        base: u8,
        height: u8,
        direction: CanvasDirection,
    ) {
        let raw = self.raw.as_ptr();
        let direction = direction.into();
        // SAFETY: `raw` is always valid
        // and `direction` is guaranteed to be valid by `From` implementation
        unsafe { sys::canvas_draw_triangle(raw, x, y, base, height, direction) }
    }

    // TODO: does `character` have to be of a wrapper type
    pub fn draw_glyph(&mut self, x: u8, y: u8, character: u16) {
        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is always valid,
        unsafe { sys::canvas_draw_glyph(raw, x, y, character) }
    }

    pub fn set_bitmap_mode(&mut self, alpha: bool) {
        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is always valid,
        unsafe { sys::canvas_set_bitmap_mode(raw, alpha) }
    }

    // TODO: do `width`, `height` and `radius` have to be non-zero
    pub fn draw_rframe(&mut self, x: u8, y: u8, width: u8, height: u8, radius: u8) {
        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is always valid,
        unsafe { sys::canvas_draw_rframe(raw, x, y, width, height, radius) }
    }

    // TODO: do `width`, `height` and `radius` have to be non-zero
    pub fn draw_rbox(&mut self, x: u8, y: u8, width: u8, height: u8, radius: u8) {
        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is always valid,
        unsafe { sys::canvas_draw_rbox(raw, x, y, width, height, radius) }
    }
}

pub struct OwnedCanvasFontParameters<'a> {
    // this wraps an effectively const pointer thus it should never be used for weiting
    raw: NonNull<SysCanvasFontParameters>,
    _parent: PhantomData<&'a CanvasView<'a>>,
}

impl<'a> OwnedCanvasFontParameters<'a> {
    pub fn leading_default(&self) -> NonZeroU8 {
        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is always valid and this allways outlives its parent
        unsafe { *raw }
            .leading_default
            .try_into()
            .expect("`leading_default` should always be positive")
    }

    pub fn leading_min(&self) -> NonZeroU8 {
        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is always valid and this allways outlives its parent
        unsafe { *raw }
            .leading_min
            .try_into()
            .expect("`leading_min` should always be positive")
    }

    pub fn height(&self) -> NonZeroU8 {
        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is always valid and this allways outlives its parent
        unsafe { *raw }
            .height
            .try_into()
            .expect("`height` should always be positive")
    }

    pub fn descender(&self) -> u8 {
        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is always valid and this allways outlives its parent
        unsafe { *raw }.descender
    }

    pub fn snapshot(&self) -> CanvasFontParameters {
        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is always valid and this allways outlives its parent
        unsafe { *raw }
            .try_into()
            .expect("raw `CanvasFontParameters` should be valid")
    }
}
