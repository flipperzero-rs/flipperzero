//! User-friendly wrappers of XDM images.

use core::slice;

pub struct XbmImage<'a> {
    data: &'a [u8],
    width: u8,
    height: u8,
}

impl<'a> XbmImage<'a> {
    pub const fn new(width: u8, height: u8, data: &'a [u8]) -> Self {
        let bytes = xds_bytes(width, height);
        assert!(
            bytes == data.len(),
            "bit-dimensions don't match bit-size of data"
        );

        Self {
            data,
            width,
            height,
        }
    }

    pub const fn width(&self) -> u8 {
        self.width
    }

    pub const fn height(&self) -> u8 {
        self.height
    }

    pub const fn data(&self) -> &[u8] {
        self.data
    }

    pub unsafe fn from_raw(height: u8, width: u8, data: *const u8) -> Self {
        let bytes = xds_bytes(width, height);

        // SAFETY: the size is exactly calculated based on width and height
        // and caller upholds the `data` validity invariant
        let data = unsafe { slice::from_raw_parts(data, bytes) };

        Self {
            data,
            width,
            height,
        }
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

    pub const fn get(&self, (x, y): (u8, u8)) -> Option<bool> {
        if let Some((byte, shift)) = self.offsets(x, y) {
            Some((self.data[byte as usize] >> (7 - shift)) & 0b1 != 0)
        } else {
            None
        }
    }
}

pub struct XbmImageMut<'a> {
    data: &'a mut [u8],
    width: u8,
    height: u8,
}

const fn xds_bytes(width: u8, height: u8) -> usize {
    (width as usize * height as usize).div_ceil(8)
}

impl<'a> XbmImageMut<'a> {
    pub fn new(data: &'a mut [u8], width: u8, height: u8) -> Self {
        let bytes = xds_bytes(width, height);
        assert!(
            bytes == data.len(),
            "bit-dimensions don't match bit-size of data"
        );

        Self {
            data,
            width,
            height,
        }
    }

    pub unsafe fn from_raw(data: *mut u8, width: u8, height: u8) -> Self {
        let bytes = xds_bytes(width, height);

        // SAFETY: the size is exactly calculated based on width and height
        // and caller upholds the `data` validity invariant
        let data = unsafe { slice::from_raw_parts_mut(data, bytes) };

        Self {
            data,
            width,
            height,
        }
    }

    pub const fn width(&self) -> u8 {
        self.width
    }

    pub const fn height(&self) -> u8 {
        self.height
    }

    pub const fn data(&self) -> &[u8] {
        self.data
    }

    pub const fn data_mut(&self) -> &[u8] {
        self.data
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

    pub const fn get(&self, (x, y): (u8, u8)) -> Option<bool> {
        if let Some((byte, shift)) = self.offsets(x, y) {
            Some((self.data[byte as usize] >> (7 - shift)) & 0b1 != 0)
        } else {
            None
        }
    }

    pub fn set(&mut self, coordinates: (u8, u8), value: bool) -> Option<()> {
        if value {
            self.set_1(coordinates)
        } else {
            self.set_0(coordinates)
        }
    }

    pub fn set_1(&mut self, (x, y): (u8, u8)) -> Option<()> {
        let (byte, shift) = self.offsets(x, y)?;
        self.data[byte as usize] |= 1 << (7 - shift);
        Some(())
    }

    pub fn set_0(&mut self, (x, y): (u8, u8)) -> Option<()> {
        let (byte, shift) = self.offsets(x, y)?;
        self.data[byte as usize] &= !(1 << (7 - shift));
        Some(())
    }

    pub fn xor(&mut self, (x, y): (u8, u8)) -> Option<()> {
        let (byte, shift) = self.offsets(x, y)?;
        self.data[byte as usize] ^= 1 << (7 - shift);
        Some(())
    }
}

impl<'a> From<XbmImageMut<'a>> for XbmImage<'a> {
    fn from(value: XbmImageMut<'a>) -> Self {
        Self {
            data: value.data,
            width: value.width,
            height: value.height,
        }
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
        $crate::xbm::XbmImage::new($width, $height, &[$($byte,)*])
    }};
}
