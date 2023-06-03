//! Canvases.

use crate::gui::{
    icon::Icon,
    icon_animation::{IconAnimation, IconAnimationCallbacks},
    xbm::XbmImage,
};
use crate::{debug, warn};
use core::fmt::Display;
use core::{
    ffi::{c_char, CStr},
    marker::PhantomData,
    num::NonZeroU8,
    ops::Deref,
    ptr::NonNull,
};
use flipperzero_sys::Align as SysAlign;
use flipperzero_sys::{
    self as sys, Canvas as SysCanvas, CanvasDirection as SysCanvasDirection,
    CanvasFontParameters as SysCanvasFontParameters, Color as SysColor, Font as SysFont,
};

/// System Canvas view.
pub struct CanvasView<'a> {
    raw: NonNull<SysCanvas>,
    _lifetime: PhantomData<&'a ()>,
}

impl CanvasView<'_> {
    /// Construct a `CanvasView` from a raw pointer.
    ///
    /// # Safety
    ///
    /// `raw` should be a valid non-null pointer to [`SysCanvas`]
    /// and the lifetime should be outlived by `raw` validity scope.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use flipperzero::gui::canvas::CanvasView;
    ///
    /// let ptr = todo!();
    /// let canvas = unsafe { CanvasView::from_raw(ptr) };
    /// ```
    pub unsafe fn from_raw(raw: *mut SysCanvas) -> Self {
        Self {
            // SAFETY: caller should provide a valid pointer
            raw: unsafe { NonNull::new_unchecked(raw) },
            _lifetime: PhantomData,
        }
    }

    // FIXME:
    //  - canvas_reset
    //  - canvas_commit
    //  This are currently not available in bindings

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

    pub fn get_font_params(&self, font: Font) -> CanvasFontParameters<'_> {
        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is always valid
        let font = font.into();
        // SAFETY: `raw` is always a valid pointer
        // and `font` is guaranteed to be a valid value by `From` implementation
        // `cast_mut` is required since `NonNull` can only be created froma mut-pointer
        let raw = unsafe { sys::canvas_get_font_params(raw, font) }.cast_mut();
        // SAFETY: `raw` is always a valid pointer
        let raw = unsafe { NonNull::new_unchecked(raw) };
        CanvasFontParameters {
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

    // TODO `canvas_draw_bitmap` compressed bitmap support

    // TODO: do we need range checks?
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

    // TODO: do we need range checks?
    pub fn draw_icon<'a, 'b: 'a>(&'a mut self, x: u8, y: u8, animation: &'b Icon) {
        let raw = self.raw.as_ptr();
        let icon = animation.as_raw();
        // SAFETY: `raw` is always valid
        // and `icon` is always valid and outlives this canvas view
        unsafe { sys::canvas_draw_icon(raw, x, y, icon) }
    }

    // TODO: do we need other range checks?
    //  what is the best return type?
    pub fn draw_xbm(
        &mut self,
        x: u8,
        y: u8,
        xbm: &XbmImage<impl Deref<Target = [u8]>>,
    ) -> Option<()> {
        let raw = self.raw.as_ptr();
        let width = xbm.width();
        let height = xbm.height();

        // ensure that the image is not too big
        let _ = x.checked_add(width)?;
        let _ = y.checked_add(height)?;

        let data = xbm.data().as_ptr();

        // SAFETY: `raw` is always valid
        // and `data` is always valid and does not have to outlive the view
        // as it is copied
        if x == 2 && y == 2 {
            warn!(
                "Printing image at {}:{} of dims {}:{}: {:?}",
                x,
                y,
                width,
                height,
                xbm.data()
            );
        }
        unsafe { sys::canvas_draw_xbm(raw, x, y, width, height, data) };
        Some(())
    }

    // TODO:
    // - `canvas_draw_icon` icon lifetimes

    // TODO: decide if we want to pack x-y pairs into tuples

    pub fn draw_dot(&mut self, x: u8, y: u8) {
        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is always valid
        unsafe { sys::canvas_draw_dot(raw, x, y) }
    }

    // TODO: do we need range checks?
    // TODO: do `width` and `height` have to be non-zero
    pub fn draw_box(&mut self, x: u8, y: u8, width: u8, height: u8) {
        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is always valid
        unsafe { sys::canvas_draw_box(raw, x, y, width, height) }
    }

    // TODO: do we need range checks?
    // TODO: do `width` and `height` have to be non-zero
    pub fn draw_frame(&mut self, x: u8, y: u8, width: u8, height: u8) {
        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is always valid
        unsafe { sys::canvas_draw_frame(raw, x, y, width, height) }
    }

    // TODO: do we need range checks?
    // TODO: do `x2` and `y2` have to be non-zero
    pub fn draw_line(&mut self, x1: u8, y1: u8, x2: u8, y2: u8) {
        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is always valid
        unsafe { sys::canvas_draw_line(raw, x1, y1, x2, y2) }
    }

    // TODO: do we need range checks?
    // TODO: does `radius` have to be non-zero
    pub fn draw_circle(&mut self, x: u8, y: u8, radius: u8) {
        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is always valid
        unsafe { sys::canvas_draw_circle(raw, x, y, radius) }
    }

    // TODO: do we need range checks?
    // TODO: does `radius` have to be non-zero
    pub fn draw_disc(&mut self, x: u8, y: u8, radius: u8) {
        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is always valid
        unsafe { sys::canvas_draw_disc(raw, x, y, radius) }
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
        let raw = self.raw.as_ptr();
        let direction = direction.into();
        // SAFETY: `raw` is always valid
        // and `direction` is guaranteed to be valid by `From` implementation
        unsafe { sys::canvas_draw_triangle(raw, x, y, base, height, direction) }
    }

    // TODO: do we need range checks?
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

    // TODO: do we need range checks?
    // TODO: do `width`, `height` and `radius` have to be non-zero
    pub fn draw_rframe(&mut self, x: u8, y: u8, width: u8, height: u8, radius: u8) {
        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is always valid,
        unsafe { sys::canvas_draw_rframe(raw, x, y, width, height, radius) }
    }

    // TODO: do we need range checks?
    // TODO: do `width`, `height` and `radius` have to be non-zero
    pub fn draw_rbox(&mut self, x: u8, y: u8, width: u8, height: u8, radius: u8) {
        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is always valid,
        unsafe { sys::canvas_draw_rbox(raw, x, y, width, height, radius) }
    }
}

pub struct CanvasFontParameters<'a> {
    // this wraps an effectively const pointer thus it should never be used for weiting
    raw: NonNull<SysCanvasFontParameters>,
    _parent: PhantomData<&'a CanvasView<'a>>,
}

impl<'a> CanvasFontParameters<'a> {
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

    pub fn snapshot(&self) -> CanvasFontParametersSnapshot {
        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is always valid and this allways outlives its parent
        unsafe { *raw }
            .try_into()
            .expect("raw `CanvasFontParameters` should be valid")
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct CanvasFontParametersSnapshot {
    pub leading_default: NonZeroU8,
    pub leading_min: NonZeroU8,
    pub height: NonZeroU8,
    pub descender: u8,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
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
            descender: value.descender,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Color {
    White,
    Black,
    Xor,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum FromSysColorError {
    Invalid(SysColor),
}

impl TryFrom<SysColor> for Color {
    type Error = FromSysColorError;

    fn try_from(value: SysColor) -> Result<Self, Self::Error> {
        Ok(match value {
            sys::Color_ColorWhite => Self::White,
            sys::Color_ColorBlack => Self::Black,
            sys::Color_ColorXOR => Self::Xor,
            invalid => Err(Self::Error::Invalid(invalid))?,
        })
    }
}

impl From<Color> for SysColor {
    fn from(value: Color) -> Self {
        match value {
            Color::White => sys::Color_ColorWhite,
            Color::Black => sys::Color_ColorBlack,
            Color::Xor => sys::Color_ColorXOR,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Font {
    Primary,
    Secondary,
    Keyboard,
    BigNumbers,
}

impl Font {
    /// Gets the total number of available fonts.
    ///
    /// # Example
    ///
    /// ```
    /// # use flipperzero::gui::canvas::Font;
    /// assert_eq!(Font::total_number(), 4);
    /// ```
    pub const fn total_number() -> usize {
        sys::Font_FontTotalNumber as usize
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum FromSysFontError {
    TotalNumber,
    Invalid(SysFont),
}

impl TryFrom<SysFont> for Font {
    type Error = FromSysFontError;

    fn try_from(value: SysFont) -> Result<Self, Self::Error> {
        Ok(match value {
            sys::Font_FontPrimary => Self::Primary,
            sys::Font_FontSecondary => Self::Secondary,
            sys::Font_FontKeyboard => Self::Keyboard,
            sys::Font_FontBigNumbers => Self::BigNumbers,
            sys::Font_FontTotalNumber => Err(Self::Error::TotalNumber)?,
            invalid => Err(Self::Error::Invalid(invalid))?,
        })
    }
}

impl From<Font> for SysFont {
    fn from(value: Font) -> Self {
        match value {
            Font::Primary => sys::Font_FontPrimary,
            Font::Secondary => sys::Font_FontSecondary,
            Font::Keyboard => sys::Font_FontKeyboard,
            Font::BigNumbers => sys::Font_FontBigNumbers,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum CanvasDirection {
    LeftToRight,
    TopToBottom,
    RightToLeft,
    BottomToTop,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum FromSysCanvasDirectionError {
    Invalid(SysCanvasDirection),
}

impl TryFrom<SysCanvasDirection> for CanvasDirection {
    type Error = FromSysCanvasDirectionError;

    fn try_from(value: SysCanvasDirection) -> Result<Self, Self::Error> {
        Ok(match value {
            sys::CanvasDirection_CanvasDirectionLeftToRight => Self::LeftToRight,
            sys::CanvasDirection_CanvasDirectionTopToBottom => Self::TopToBottom,
            sys::CanvasDirection_CanvasDirectionRightToLeft => Self::RightToLeft,
            sys::CanvasDirection_CanvasDirectionBottomToTop => Self::BottomToTop,
            invalid => Err(Self::Error::Invalid(invalid))?,
        })
    }
}

impl From<CanvasDirection> for SysCanvasDirection {
    fn from(value: CanvasDirection) -> Self {
        match value {
            CanvasDirection::BottomToTop => sys::CanvasDirection_CanvasDirectionBottomToTop,
            CanvasDirection::LeftToRight => sys::CanvasDirection_CanvasDirectionLeftToRight,
            CanvasDirection::RightToLeft => sys::CanvasDirection_CanvasDirectionRightToLeft,
            CanvasDirection::TopToBottom => sys::CanvasDirection_CanvasDirectionTopToBottom,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Align {
    Left,
    Right,
    Top,
    Bottom,
    Center,
}

#[derive(Clone, Copy, Debug)]
pub enum FromSysAlignError {
    Invalid(SysAlign),
}

impl TryFrom<SysAlign> for Align {
    type Error = FromSysAlignError;

    fn try_from(value: SysAlign) -> Result<Self, Self::Error> {
        Ok(match value {
            sys::Align_AlignLeft => Self::Left,
            sys::Align_AlignRight => Self::Right,
            sys::Align_AlignTop => Self::Top,
            sys::Align_AlignBottom => Self::Bottom,
            sys::Align_AlignCenter => Self::Center,
            invalid => Err(Self::Error::Invalid(invalid))?,
        })
    }
}

impl From<Align> for SysAlign {
    fn from(value: Align) -> Self {
        match value {
            Align::Left => sys::Align_AlignLeft,
            Align::Right => sys::Align_AlignRight,
            Align::Top => sys::Align_AlignTop,
            Align::Bottom => sys::Align_AlignBottom,
            Align::Center => sys::Align_AlignCenter,
        }
    }
}
