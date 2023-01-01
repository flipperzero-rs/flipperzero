//! ViewPort APIs

use crate::gui::Gui;
use core::{ffi::CStr, num::NonZeroU8};
use flipperzero::furi::canvas::Align;
use flipperzero_sys::{
    self as sys, Canvas as SysCanvas, CanvasFontParameters as SysCanvasFontParameters,
    Color as SysColor, Font as SysFont,
};

/// System Canvas.
pub struct Canvas<'a> {
    _parent: &'a Gui,
    canvas: *mut SysCanvas,
}

impl<'a> Canvas<'a> {
    /// Construct a `Canvas` from a raw non-null pointer.
    ///
    /// After calling this function, the raw pointer is owned by the resulting `Canvas`.
    /// Specifically, the `Canvas` destructor will free the allocated memory.
    ///
    /// # Safety
    ///
    /// - `parent` should be the `Gui` which owns this canvas;
    ///
    /// - `raw` should be a valid pointer to [`Canvas`].
    ///
    /// # Examples
    ///
    /// Recreate a `Canvas`
    /// which vas previously converted to a raw pointer using [`Canvas::into_raw`].
    ///
    /// ```
    /// use flipperzero_gui::{canvas::Canvas, gui::Gui};
    ///
    /// let mut gui = Gui::new();
    /// let canvas = gui.direct_draw_acquire();
    /// let ptr = canvas.into_raw();
    /// let canvas = unsafe { Canvas::from_raw(gui, ptr) };
    /// ```
    pub unsafe fn from_raw(parent: &mut Gui, raw: NonNull<SysViewPort>) -> Self {
        Self {
            _parent: parent,
            canvas: raw.as_ptr(),
        }
    }

    /// Consumes this wrapper, returning a non-null raw pointer.
    ///
    /// After calling this function, the caller is responsible
    /// for the memory previously managed by the `Canvas`.
    /// In particular, the caller should properly destroy `SysCanvas` and release the memory.
    /// The easiest way to do this is to convert the raw pointer
    /// back into a `Canvas` with the [ViewPort::from_raw] function,
    /// allowing the `Canvas` destructor to perform the cleanup.
    ///
    /// # Example
    ///
    /// Converting the raw pointer back into a `Canvas`
    /// with `Canvas::from_raw` for automatic cleanup:
    ///
    /// ```
    /// use flipperzero_gui::{canvas::Canvas, gui::Gui};
    ///
    /// let mut gui = Gui::new();
    /// let canvas = gui.direct_draw_acquire();
    /// let ptr = canvas.into_raw();
    /// let canvas = unsafe { Canvas::from_raw(gui, ptr) };
    /// ```
    pub fn into_raw(mut self) -> NonNull<SysCanvas> {
        let raw_pointer = core::mem::replace(&mut self.canvas, null_mut());
        // SAFETY: `self.canvas` is guaranteed to be non-null
        // since it only becomes null after call to this function
        // which consumes the wrapper
        unsafe { NonNull::new_unchecked(raw_pointer) }
    }

    // FIXME:
    // - canvas_reset
    // - canvas_commit

    pub fn width(&self) -> NonZeroU8 {
        // SAFETY: `self.canvas` is always a valid pointer
        unsafe { sys::canvas_width(self.canvas) }
            .try_into()
            .expect("`canvas_width` should produce a positive value")
    }

    pub fn height(&self) -> NonZeroU8 {
        // SAFETY: `self.canvas` is always a valid pointer
        unsafe { sys::canvas_height(self.canvas) }
            .try_into()
            .expect("`canvas_height` should produce a positive value")
    }

    pub fn current_font_height(&self) -> NonZeroU8 {
        // SAFETY: `self.canvas` is always a valid pointer
        unsafe { sys::canvas_current_font_height(self.canvas) }
            .try_into()
            .expect("`canvas_current_font_height` should produce a positive value")
    }

    pub fn get_font_params(&self, font: Font) -> CanvasFontParameters<'_> {
        let font = font.into();
        // SAFETY: `self.canvas` is always a valid pointer
        // and `font` is guaranteed to be a valid value by `From` implementation
        let raw = unsafe { sys::canvas_get_font_params(self.canvas, font) };
        CanvasFontParameters { _parent: self, raw }
    }

    pub fn clear(&mut self) {
        // SAFETY: `self.canvas` is always a valid pointer
        unsafe { sys::canvas_clear(self.canvas) };
    }

    pub fn set_color(&mut self, color: Color) {
        let color = color.into();
        // SAFETY: `self.canvas` is always a valid pointer
        // and `font` is guaranteed to be a valid value by `From` implementation
        unsafe { sys::canvas_set_color(self.canvas, color) };
    }

    pub fn set_font_direction(&mut self, font_direction: CanvasDirection) {
        let font_direction = font_direction.into();
        // SAFETY: `self.canvas` is always a valid pointer
        // and `font_direction` is guaranteed to be a valid value by `From` implementation
        unsafe { sys::canvas_set_font_direction(self.canvas, font_direction) };
    }

    pub fn invert_color(&mut self) {
        // SAFETY: `self.canvas` is always a valid pointer
        unsafe { sys::canvas_invert_color(self.canvas) };
    }

    pub fn set_font(&mut self, font: Font) {
        let font = font.into();
        // SAFETY: `self.canvas` is always a valid pointer
        // and `font` is guaranteed to be a valid value by `From` implementation
        unsafe { sys::canvas_set_font(self.canvas, font) };
    }

    pub fn draw_str(&mut self, x: u8, y: u8, str: impl AsRef<CStr<'_>>) {
        let font = font.into();
        let str = str.as_ref().as_ptr();
        // SAFETY: `self.canvas` is always a valid pointer
        // and `text` is guaranteed to be a valid pointer since it was created from `CStr`
        unsafe { sys::canvas_draw_str(self.canvas, x, y, str) };
    }

    pub fn draw_str_aligned(
        &mut self,
        x: u8,
        y: u8,
        horizontal: Align,
        vertical: Align,
        str: impl AsRef<CStr<'_>>,
    ) {
        let font = font.into();
        let horizontal = horizontal.into();
        let vertical = vertical.into();
        let str = str.as_ref().as_ptr();
        // SAFETY: `self.canvas` is always a valid pointer,
        // `horixontal` and `vertival` are guaranteed to be valid by `From` implementation
        // and `text` is guaranteed to be a valid pointer since it was created from `CStr`
        unsafe { sys::canvas_draw_str_aligned(self.canvas, x, y, horizontal, vertical, str) };
    }

    // TODO:
    // - `canvas_string_width` this API looks quite strange yet
    // - `canvas_flyph_width` this API looks quite strange yet
    // - `canvas_draw_bitmap` bitmap constraints
    // - `canvas_draw_icon_animation` animation lifetimes
    // - `canvas_draw_icon` icon lifetimes
    // - `canvas_draw_xbm` bitmap constraints

    // TODO: decide if we want to pack x-y pairs into tuples

    pub fn draw_dot(&mut self, x: u8, y: u8) {
        // SAFETY: `self.canvas` is always a valid pointer,
        unsafe { sys::canvas_draw_dot(self.canvas, x, y) }
    }

    // TODO: do we need range checks?
    // TODO: do `width` and `height` have to be non-zero
    pub fn draw_box(&mut self, x: u8, y: u8, width: u8, height: u8) {
        // SAFETY: `self.canvas` is always a valid pointer,
        unsafe { sys::canvas_draw_box(self.canvas, x, y, width, height) }
    }

    // TODO: do we need range checks?
    // TODO: do `width` and `height` have to be non-zero
    pub fn draw_frame(&mut self, x: u8, y: u8, width: u8, height: u8) {
        // SAFETY: `self.canvas` is always a valid pointer,
        unsafe { sys::canvas_draw_frame(self.canvas, x, y, width, height) }
    }

    // TODO: do we need range checks?
    // TODO: do `x2` and `y2` have to be non-zero
    pub fn draw_line(&mut self, x1: u8, y1: u8, x2: u8, y2: u8) {
        // SAFETY: `self.canvas` is always a valid pointer,
        unsafe { sys::canvas_draw_line(self.canvas, x1, y1, x2, y2) }
    }

    // TODO: do we need range checks?
    // TODO: does `radius` have to be non-zero
    pub fn draw_circle(&mut self, x: u8, y: u8, radius: u8) {
        // SAFETY: `self.canvas` is always a valid pointer,
        unsafe { sys::canvas_draw_circle(self.canvas, x, y, radius) }
    }

    // TODO: do we need range checks?
    // TODO: does `radius` have to be non-zero
    pub fn draw_disc(&mut self, x: u8, y: u8, radius: u8) {
        // SAFETY: `self.canvas` is always a valid pointer,
        unsafe { sys::canvas_draw_disc(self.canvas, x, y, radius) }
    }

    // TODO: do we need range checks?
    // TODO: do `base` and `height` have to be non-zero
    pub fn draw_triangle(
        &mut self,
        x: u8,
        y: u8,
        base: u8,
        height: u8,
        direction: CanvasDirection,
    ) {
        let direction = direction.into();
        // SAFETY: `self.canvas` is always a valid pointer,
        // and `direction` is guaranteed to be valid by `From` implementation
        unsafe { sys::canvas_draw_triangle(self.canvas, x, y, base, height, direction) }
    }

    // TODO: do we need range checks?
    // TODO: does `character` have to be of a wrapper type
    pub fn draw_glyph(&mut self, x: u8, y: u8, character: u16) {
        // SAFETY: `self.canvas` is always a valid pointer,
        unsafe { sys::canvas_draw_glyph(self.canvas, x, y, character) }
    }

    pub fn set_bitmap_mode(&mut self, alpha: bool) {
        // SAFETY: `self.canvas` is always a valid pointer,
        unsafe { sys::canvas_set_bitmap_mode(self.canvas, alpha) }
    }

    // TODO: do we need range checks?
    // TODO: do `width`, `height` and `radius` have to be non-zero
    pub fn draw_rframe(&mut self, x: u8, y: u8, width: u8, height: u8, radius: u8) {
        // SAFETY: `self.canvas` is always a valid pointer,
        unsafe { sys::canvas_draw_rframe(self.canvas, x, y, width, height, radius) }
    }

    // TODO: do we need range checks?
    // TODO: do `width`, `height` and `radius` have to be non-zero
    pub fn draw_rbox(&mut self, x: u8, y: u8, width: u8, height: u8, radius: u8) {
        // SAFETY: `self.canvas` is always a valid pointer,
        unsafe { sys::canvas_draw_rbox(self.canvas, x, y, width, height, radius) }
    }
}

impl Drop for Canvas<'_> {
    fn drop(&mut self) {
        // unsafe { sys::gui_direct_draw_release(self.parent...) }
    }
}

pub struct CanvasFontParameters<'a> {
    _parent: &'a Canvas,
    raw: *mut SysCanvasFontParameters,
}

impl<'a> CanvasFontParameters<'a> {
    fn leading_default(&self) -> NonZeroU8 {
        // SAFETY: this allways outlives its parent
        unsafe { *self.raw }
            .leading_default
            .try_into()
            .expect("`leading_default` should always be positive")
    }

    fn set_leading_default(&mut self, leading_default: NonZeroU8) {
        // SAFETY: this allways outlives its parent
        unsafe { *self.raw }.leading_default = leading_default.into()
    }

    fn leading_min(&self) -> NonZeroU8 {
        // SAFETY: this allways outlives its parent
        unsafe { *self.raw }
            .leading_min
            .try_into()
            .expect("`leading_min` should always be positive")
    }

    fn set_leading_min(&mut self, leading_min: NonZeroU8) {
        // SAFETY: this allways outlives its parent
        unsafe { *self.raw }.leading_min = leading_min.into()
    }

    fn height(&self) -> NonZeroU8 {
        // SAFETY: this allways outlives its parent
        unsafe { *self.raw }
            .height
            .try_into()
            .expect("`height` should always be positive")
    }

    fn set_height(&mut self, height: NonZeroU8) {
        // SAFETY: this allways outlives its parent
        unsafe { *self.raw }.height = height.into()
    }

    fn descender(&self) -> u8 {
        // SAFETY: this allways outlives its parent
        unsafe { *self.raw }.descender
    }

    fn set_descender(&mut self, descender: u8) {
        // SAFETY: this allways outlives its parent
        unsafe { *self.raw }.descender = descender
    }

    fn snapshot(&self) -> CanvasFontParametersSnapshot {
        unsafe { *self.raw }
            .try_into()
            .expect("raw `CanvasFontParameters` should be valid")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanvasFontParametersSnapshot {
    leading_default: NonZeroU8,
    leading_min: NonZeroU8,
    height: NonZeroU8,
    descender: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FromSysGuiLayerError {
    ZeroLeadingDefault,
    ZeroLeadingMin,
    ZeroHeight,
}

impl TryFrom<SysCanvasFontParameters> for CanvasFontParametersSnapshot {
    type Error = FromSysGuiLayerError;

    fn try_from(value: SysCanvasFontParameters) -> Result<Self, Self::Error> {
        Ok(Self {
            leading_default: value
                .leading_default
                .try_into()
                .or(Err(Self::Error::ZeroLeadingDefault))?,
            leading_min: value
                .leading_min
                .try_into()
                .or(Err(Self::Error::ZeroLeadingMin))?,
            height: value.height.try_into().or(Err(Self::Error::ZeroHeight))?,
            descender: value.descender,
        })
    }
}

impl From<CanvasFontParametersSnapshot> for SysCanvasFontParameters {
    fn from(value: CanvasFontParametersSnapshot) -> Self {
        Self {
            leading_default: value.leading_default.into(),
            leading_min: value.leading_min.into(),
            height: value.height.into(),
            descender: value.descender.into(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
    // TDOO: add this color
    // Xor,
}

#[derive(Clone, Copy, Debug)]
pub enum FromSysColor {
    Invalid(SysColor),
}

impl TryFrom<SysColor> for Color {
    type Error = FromSysColor;

    fn try_from(value: SysColor) -> Result<Self, Self::Error> {
        use sys::{
            Color_ColorBlack as SYS_COLOR_BLACK,
            Color_ColorWhite as SYS_COLOR_WHITE,
            // Color_ColorXOR as SYS_COLOR_XOR,
        };

        Ok(match value {
            SYS_COLOR_WHITE => Self::White,
            SYS_COLOR_BLACK => Self::Black,
            // SYS_COLOR_XOR => Ok(Self::Xor),
            invalid => Err(Self::Error::Invalid(invalid))?,
        })
    }
}

impl From<Color> for SysColor {
    fn from(value: Color) -> Self {
        use sys::{
            Color_ColorBlack as SYS_COLOR_BLACK,
            Color_ColorWhite as SYS_COLOR_WHITE,
            // Color_ColorXOR as SYS_COLOR_XOR,
        };

        match value {
            Color::White => SYS_COLOR_WHITE,
            Color::Black => SYS_COLOR_BLACK,
            // Color::Xor => SYS_COLOR_XOR,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Font {
    Primary,
    Secondary,
    Keyboard,
    BigNumbers,
}

#[derive(Clone, Copy, Debug)]
pub enum FromSysFont {
    TotalNumber,
    Invalid(SysFont),
}

impl TryFrom<SysFont> for Font {
    type Error = FromSysFont;

    fn try_from(value: SysFont) -> Result<Self, Self::Error> {
        use sys::{
            Font_FontBigNumbers as SYS_FONT_BIG_NUMBERS, Font_FontKeyboard as SYS_FONT_KEYBOARD,
            Font_FontPrimary as SYS_FONT_PRIMARY, Font_FontSecondary as SYS_FONT_SECONDARY,
            Font_FontTotalNumber as SYS_FONT_TOTAL_NUMBER,
        };

        Ok(match value {
            SYS_FONT_PRIMARY => Self::Primary,
            SYS_FONT_SECONDARY => Self::Secondary,
            SYS_FONT_KEYBOARD => Self::Keyboard,
            SYS_FONT_BIG_NUMBERS => Self::BigNumbers,
            SYS_FONT_TOTAL_NUMBER => Err(Self::Error::TotalNumber)?,
            invalid => Err(Self::Error::Invalid(invalid))?,
        })
    }
}

impl From<Font> for SysFont {
    fn from(value: Font) -> Self {
        use sys::{
            Font_FontBigNumbers as SYS_FONT_BIG_NUMBERS, Font_FontKeyboard as SYS_FONT_KEYBOARD,
            Font_FontPrimary as SYS_FONT_PRIMARY, Font_FontSecondary as SYS_FONT_SECONDARY,
        };

        match value {
            Font::Primary => SYS_FONT_PRIMARY,
            Font::Secondary => SYS_FONT_SECONDARY,
            Font::Keyboard => SYS_FONT_KEYBOARD,
            Font::BigNumbers => SYS_FONT_BIG_NUMBERS,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum CanvasOrientation {
    Horizontal,
    HorizontalFlip,
    Vertical,
    VerticalFlip,
}

#[derive(Clone, Copy, Debug)]
pub enum FromSysCanvasOrientationError {
    Invalid(SysCanvasOrientation),
}

impl TryFrom<SysCanvasOrientation> for CanvasOrientation {
    type Error = FromSysCanvasOrientationError;

    fn try_from(value: SysCanvasOrientation) -> Result<Self, Self::Error> {
        use sys::{
            CanvasOrientation_CanvasOrientationHorizontal as SYS_CANVAS_ORIENTATION_HORIZONTAL,
            CanvasOrientation_CanvasOrientationHorizontalFlip as SYS_CANVAS_ORIENTATION_HORIZONTAL_FLIP,
            CanvasOrientation_CanvasOrientationVertical as SYS_CANVAS_ORIENTATION_VERTICAL,
            CanvasOrientation_CanvasOrientationVerticalFlip as SYS_CANVAS_ORIENTATION_VERTICAL_FLIP,
        };

        Ok(match value {
            SYS_CANVAS_ORIENTATION_HORIZONTAL => Self::Horizontal,
            SYS_CANVAS_ORIENTATION_HORIZONTAL_FLIP => Self::HorizontalFlip,
            SYS_CANVAS_ORIENTATION_VERTICAL => Self::Vertical,
            SYS_CANVAS_ORIENTATION_VERTICAL_FLIP => Self::VerticalFlip,
            invalid => Err(Self::Error::Invalid(invalid))?,
        })
    }
}

impl From<CanvasOrientation> for SysCanvasOrientation {
    fn from(value: CanvasOrientation) -> Self {
        use sys::{
            CanvasOrientation_CanvasOrientationHorizontal as SYS_CANVAS_ORIENTATION_HORIZONTAL,
            CanvasOrientation_CanvasOrientationHorizontalFlip as SYS_CANVAS_ORIENTATION_HORIZONTAL_FLIP,
            CanvasOrientation_CanvasOrientationVertical as SYS_CANVAS_ORIENTATION_VERTICAL,
            CanvasOrientation_CanvasOrientationVerticalFlip as SYS_CANVAS_ORIENTATION_VERTICAL_FLIP,
        };

        match value {
            CanvasOrientation::Horizontal => SYS_CANVAS_ORIENTATION_HORIZONTAL,
            CanvasOrientation::HorizontalFlip => SYS_CANVAS_ORIENTATION_HORIZONTAL_FLIP,
            CanvasOrientation::Vertical => SYS_CANVAS_ORIENTATION_VERTICAL,
            CanvasOrientation::VerticalFlip => SYS_CANVAS_ORIENTATION_VERTICAL_FLIP,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum CanvasDirection {
    LeftToRight,
    TopToBottom,
    RightToLeft,
    BottomToTop,
}

#[derive(Clone, Copy, Debug)]
pub enum FromSysCanvasDirectionError {
    Invalid(SysCanvasDirection),
}

impl TryFrom<SysCanvasDirection> for CanvasDirection {
    type Error = FromSysCanvasDirectionError;

    fn try_from(value: SysCanvasDirection) -> Result<Self, Self::Error> {
        use sys::{
            CanvasDirection_BottomToTop as SYS_CANVAS_DIRECTION_BOTTOM_TO_TOP,
            CanvasDirection_LeftToRight as SYS_CANVAS_DIRECTION_LEFT_TO_RIGHT,
            CanvasDirection_RightToLeft as SYS_CANVAS_DIRECTION_RIGHT_TO_LEFT,
            CanvasDirection_TopToBottom as SYS_CANVAS_DIRECTION_TOP_TO_BOTTOM,
        };

        Ok(match value {
            SYS_CANVAS_LEFT_TO_RIGHT => Self::LeftToRight,
            SYS_CANVAS_TOP_TO_BOTTOM => Self::TopToBottom,
            SYS_CANVAS_RIGHT_TO_LEFT => Self::RightToLeft,
            SYS_CANVAS_BOTTOM_TO_TOP => Self::BottomToTop,
            invalid => Err(Self::Error::Invalid(invalid))?,
        })
    }
}

impl From<CanvasDirection> for SysCanvasDirection {
    fn from(value: CanvasDirection) -> Self {
        use sys::{
            CanvasDirection_BottomToTop as SYS_CANVAS_DIRECTION_BOTTOM_TO_TOP,
            CanvasDirection_LeftToRight as SYS_CANVAS_DIRECTION_LEFT_TO_RIGHT,
            CanvasDirection_RightToLeft as SYS_CANVAS_DIRECTION_RIGHT_TO_LEFT,
            CanvasDirection_TopToBottom as SYS_CANVAS_DIRECTION_TOP_TO_BOTTOM,
        };

        match value {
            CanvasDirection::BottomToTop => SYS_CANVAS_DIRECTION_BOTTOM_TO_TOP,
            CanvasDirection::LeftToRight => SYS_CANVAS_DIRECTION_LEFT_TO_RIGHT,
            CanvasDirection::RightToLeft => SYS_CANVAS_DIRECTION_RIGHT_TO_LEFT,
            CanvasDirection::TopToBottom => SYS_CANVAS_DIRECTION_TOP_TO_BOTTOM,
        }
    }
}
