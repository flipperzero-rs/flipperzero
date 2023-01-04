//! User-friendly wrappers of XDM images.

use core::slice;

pub struct XbmImage<'a> {
    data: &'a [u8],
    width: u8,
    height: u8,
}

impl<'a> XbmImage<'a> {
    pub fn new(data: &'a [u8], width: u8, height: u8) -> Self {
        assert!(
            (width * height).div_ceil(8) as usize == data.len(),
            "dimensions should correspond to data size"
        );

        Self {
            data,
            width,
            height,
        }
    }

    pub unsafe fn from_raw(data: *const u8, width: u8, height: u8) -> Self {
        // each byte stores 8 dot-bits,
        // if the value is not divisible by 8 then the last byte is used partially
        let size = (width * height).div_ceil(8) as usize;

        // SAFETY: the size is exactly calculated based on width and height
        // and caller upholds the `data` validity invariant
        let data = unsafe { slice::from_raw_parts(data, size) };

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

impl<'a> XbmImageMut<'a> {
    pub fn new(data: &'a mut [u8], width: u8, height: u8) -> Self {
        assert!(
            (width * height).div_ceil(8) as usize == data.len(),
            "dimensions should correspond to data size"
        );

        Self {
            data,
            width,
            height,
        }
    }

    pub unsafe fn from_raw(data: *mut u8, width: u8, height: u8) -> Self {
        // each byte stores 8 dot-bits,
        // if the value is not divisible by 8 then the last byte is used partially
        let size = (width * height).div_ceil(8) as usize;

        // SAFETY: the size is exactly calculated based on width and height
        // and caller upholds the `data` validity invariant
        let data = unsafe { slice::from_raw_parts_mut(data, size) };

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
