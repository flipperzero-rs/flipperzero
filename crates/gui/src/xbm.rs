//! User-friendly wrappers of XDM images.

use alloc::vec;
use alloc::vec::Vec;
use core::ops::{Deref, DerefMut};
use core::slice;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct XbmImage<D> {
    data: D,
    width: u8,
    height: u8,
}

impl<D> XbmImage<D> {
    pub const fn width(&self) -> u8 {
        self.width
    }

    pub const fn height(&self) -> u8 {
        self.height
    }

    pub const fn dimensions(&self) -> (u8, u8) {
        (self.width, self.height)
    }

    #[inline]
    const fn dimension_bits(width: u8, height: u8) -> u16 {
        width as u16 * height as u16
    }

    #[inline]
    const fn bits_to_min_required_bytes(bits: u16) -> u16 {
        bits.div_ceil(8)
    }

    #[inline]
    const fn dimension_bytes(width: u8, height: u8) -> u16 {
        Self::bits_to_min_required_bytes(Self::dimension_bits(width, height))
    }

    #[inline]
    const fn offset(&self, x: u8, y: u8) -> Option<u8> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(x * self.width + y)
        }
    }

    #[inline]
    const fn offsets(&self, x: u8, y: u8) -> Option<(u8, u8)> {
        if let Some(offset) = self.offset(x, y) {
            Some((offset / 8, offset % 8))
        } else {
            None
        }
    }
}

impl<T: ?Sized, D: Deref<Target = T>> XbmImage<D> {
    pub fn data(&self) -> &D::Target {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut D::Target
    where
        D: DerefMut,
    {
        &mut self.data
    }
}

impl<D: Deref<Target = [u8]>> XbmImage<D> {
    pub fn new_from(width: u8, height: u8, data: D) -> Self {
        let bits = Self::dimension_bits(width, height);
        let bytes = Self::bits_to_min_required_bytes(bits);

        assert!(
            bytes as usize == data.len(),
            "width={} * height={} = {} should correspond to {} bytes, but data has length {}",
            width,
            height,
            bits,
            bytes,
            data.len()
        );

        Self {
            data,
            width,
            height,
        }
    }

    pub fn get(&self, (x, y): (u8, u8)) -> Option<bool> {
        if let Some((byte, shift)) = self.offsets(x, y) {
            Some((self.data[byte as usize] >> shift) & 0b1 != 0)
        } else {
            None
        }
    }
}

impl<'a> XbmImage<&'a [u8]> {
    pub unsafe fn from_raw(width: u8, height: u8, data: *const u8) -> Self {
        let bytes = Self::dimension_bytes(width, height) as usize;

        // SAFETY: the size is exactly calculated based on width and height
        // and caller upholds the `data` validity invariant
        let data = unsafe { slice::from_raw_parts(data, bytes) };

        Self {
            data,
            width,
            height,
        }
    }
}

impl<'a> XbmImage<&'a mut [u8]> {
    pub unsafe fn from_raw_mut(width: u8, height: u8, data: *mut u8) -> Self {
        let bytes = Self::dimension_bytes(width, height) as usize;

        // SAFETY: the size is exactly calculated based on width and height
        // and caller upholds the `data` validity invariant
        let data = unsafe { slice::from_raw_parts_mut(data, bytes) };

        Self {
            data,
            width,
            height,
        }
    }
}

impl<D: Deref<Target = [u8]> + DerefMut> XbmImage<D> {
    pub fn set(&mut self, coordinates: (u8, u8), value: bool) -> Option<()> {
        if value {
            self.set_1(coordinates)
        } else {
            self.set_0(coordinates)
        }
    }

    pub fn set_1(&mut self, (x, y): (u8, u8)) -> Option<()> {
        let (byte, shift) = self.offsets(x, y)?;
        self.data[byte as usize] |= 1 << shift;
        Some(())
    }

    pub fn set_0(&mut self, (x, y): (u8, u8)) -> Option<()> {
        let (byte, shift) = self.offsets(x, y)?;
        self.data[byte as usize] &= !(1 << shift);
        Some(())
    }

    pub fn xor(&mut self, (x, y): (u8, u8)) -> Option<()> {
        let (byte, shift) = self.offsets(x, y)?;
        self.data[byte as usize] ^= 1 << shift;
        Some(())
    }
}

// Specializations

impl XbmImage<Vec<u8>> {
    pub fn new(width: u8, height: u8) -> Self {
        let bytes = Self::dimension_bytes(width, height) as usize;
        Self {
            data: vec![0; bytes],
            width,
            height,
        }
    }
}

impl XbmImage<&'static [u8]> {
    /// Creates a referencing `XbmImage` from a static `u8` slice.
    ///
    /// This is a constant specialization of [`XbmImage::new_from`]
    /// existing since the latter cannot be generically `const`
    /// until `const_fn_trait_bound` Rust feature becomes stable.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// use flipperzero_gui::xbm::XbmImage;
    ///
    /// const IMAGE: XbmImage<&'static [u8]> = XbmImage::new_from_static(4, 4, &[0xFE, 0x12]);
    /// ```
    pub const fn new_from_static(width: u8, height: u8, data: &'static [u8]) -> Self {
        let bytes = Self::dimension_bytes(width, height);

        assert!(
            bytes as usize == data.len(),
            "dimensions don't match data length",
        );

        Self {
            data,
            width: 0,
            height: 0,
        }
    }
}

impl<const SIZE: usize> XbmImage<ByteArray<SIZE>> {
    /// Creates a referencing `XbmImage` from a `u8` array.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// use flipperzero_gui::xbm::XbmImage;
    ///
    /// const IMAGE: XbmImage<[u8; 2]> = XbmImage::new_from_array::<4, 4>([0xFE, 0x12]);
    /// ```
    pub const fn new_from_array<const WIDTH: u8, const HEIGHT: u8>(data: [u8; SIZE]) -> Self {
        let bytes = Self::dimension_bytes(WIDTH, HEIGHT);

        assert!(bytes as usize == SIZE, "dimensions don't match data length");

        Self {
            data: ByteArray(data),
            width: 0,
            height: 0,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ByteArray<const N: usize>(pub [u8; N]);

impl<const N: usize> Deref for ByteArray<N> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl<const N: usize> DerefMut for ByteArray<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut_slice()
    }
}

#[macro_export]
macro_rules! xbm {
    (
        #define $_width_ident:ident $width:literal
        #define $_height_ident:ident $height:literal
        $(
            #define $_hotspot_x_ident:ident $_hotspot_x:literal
            #define $_hotspot_y_ident:ident $_hotspot_y:literal
        )?
        static char $_bits_ident:ident[] = {
            $($byte:literal),* $(,)?
        };
    ) => {{
        $crate::xbm::XbmImage::new_from_array::<$width, $height>([$($byte,)*])
    }};
}

// TODO: enable test execution
#[cfg(test)]
mod tests {

    #[test]
    fn valid_byte_reading() {
        // 0100110000111100
        // 0000001111111100
        let image = xbm!(
            #define xbm_test_width 16
            #define xbm_test_height 2
            static char xbm_test_bits[] = {
                0x32, // 0b00110010 ~ 0b01001100
                0x3C, // 0b00111100 ~ 0b00111100
                0xC0, // 0b11000000 ~ 0b00000011
                0x3F, // 0b00111111 ~ 0b11111100
            };
        );

        assert!(!image.get((0, 0)));
        assert!(image.get((0, 1)));
        assert!(!image.get((0, 2)));
        assert!(!image.get((0, 3)));
        assert!(image.get((0, 4)));
        assert!(image.get((0, 5)));
        assert!(!image.get((0, 6)));
        assert!(!image.get((0, 7)));
        assert!(!image.get((0, 8)));
        assert!(!image.get((0, 9)));
        assert!(image.get((0, 10)));
        assert!(image.get((0, 11)));
        assert!(image.get((0, 12)));
        assert!(image.get((0, 13)));
        assert!(!image.get((0, 14)));
        assert!(!image.get((0, 15)));
        assert!(!image.get((1, 0)));
        assert!(!image.get((1, 1)));
        assert!(!image.get((1, 2)));
        assert!(!image.get((1, 3)));
        assert!(!image.get((1, 4)));
        assert!(!image.get((1, 5)));
        assert!(image.get((1, 6)));
        assert!(image.get((1, 7)));
        assert!(image.get((1, 8)));
        assert!(image.get((1, 9)));
        assert!(image.get((1, 10)));
        assert!(image.get((1, 11)));
        assert!(image.get((1, 12)));
        assert!(image.get((1, 13)));
        assert!(!image.get((1, 14)));
        assert!(!image.get((1, 15)));
    }
}
