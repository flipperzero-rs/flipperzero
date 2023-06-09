use core::ffi::c_char;
use core::fmt;
use core::iter::{Copied, FusedIterator};
use core::slice;

use flipperzero_sys as sys;

pub unsafe fn next_code_point<'a, I: Iterator<Item = &'a u8>>(bytes: &mut I) -> Option<u32> {
    // Decode UTF-8
    let mut state = sys::FuriStringUTF8State_FuriStringUTF8StateStarting;
    let mut unicode = 0u32;
    loop {
        sys::furi_string_utf8_decode(*bytes.next()? as c_char, &mut state, &mut unicode);
        match state {
            sys::FuriStringUTF8State_FuriStringUTF8StateStarting => break Some(unicode),
            sys::FuriStringUTF8State_FuriStringUTF8StateError => break Some(0xfffd), // �
            _ => (),
        }
    }
}

/// An iterator over the [`char`]s of a string.
///
/// This struct is created by the [`chars_lossy`] method on [`FuriString`]. See its
/// documentation for more.
///
/// [`char`]: prim@char
/// [`chars_lossy`]: super::FuriString::chars_lossy
/// [`FuriString`]: super::FuriString
#[derive(Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Chars<'a> {
    pub(super) iter: slice::Iter<'a, u8>,
}

impl<'a> Iterator for Chars<'a> {
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<char> {
        // SAFETY: `next_code_point` returns a valid Unicode Scalar Value, replacing
        // invalid data with �.
        unsafe { next_code_point(&mut self.iter).map(|ch| char::from_u32_unchecked(ch)) }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.iter.len();
        // `(len + 3)` can't overflow, because we know that the `slice::Iter` belongs to a
        // slice in memory which has a maximum length of `isize::MAX` (that's well below
        // `usize::MAX`).
        ((len + 3) / 4, Some(len))
    }
}

impl fmt::Debug for Chars<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Chars(")?;
        f.debug_list().entries(self.clone()).finish()?;
        write!(f, ")")?;
        Ok(())
    }
}

impl FusedIterator for Chars<'_> {}

/// An iterator over the [`char`]s of a string, and their positions.
///
/// This struct is created by the [`char_indices_lossy`] method on [`FuriString`]. See its
/// documentation for more.
///
/// [`char`]: prim@char
/// [`char_indices_lossy`]: super::FuriString::char_indices_lossy
/// [`FuriString`]: super::FuriString
#[derive(Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct CharIndices<'a> {
    pub(super) front_offset: usize,
    pub(super) iter: Chars<'a>,
}

impl<'a> Iterator for CharIndices<'a> {
    type Item = (usize, char);

    #[inline]
    fn next(&mut self) -> Option<(usize, char)> {
        let pre_len = self.iter.iter.len();
        match self.iter.next() {
            None => None,
            Some(ch) => {
                let index = self.front_offset;
                let len = self.iter.iter.len();
                self.front_offset += pre_len - len;
                Some((index, ch))
            }
        }
    }

    #[inline]
    fn count(self) -> usize {
        self.iter.count()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl FusedIterator for CharIndices<'_> {}

/// An iterator over the bytes of a string.
///
/// This struct is created by the [`bytes`] method on [`FuriString`]. See its
/// documentation for more.
///
/// [`bytes`]: super::FuriString::bytes
/// [`FuriString`]: super::FuriString
#[must_use = "iterators are lazy and do nothing unless consumed"]
#[derive(Clone, Debug)]
pub struct Bytes<'a>(pub(super) Copied<slice::Iter<'a, u8>>);

impl Iterator for Bytes<'_> {
    type Item = u8;

    #[inline]
    fn next(&mut self) -> Option<u8> {
        self.0.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }

    #[inline]
    fn count(self) -> usize {
        self.0.count()
    }

    #[inline]
    fn last(self) -> Option<Self::Item> {
        self.0.last()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth(n)
    }

    #[inline]
    fn all<F>(&mut self, f: F) -> bool
    where
        F: FnMut(Self::Item) -> bool,
    {
        self.0.all(f)
    }

    #[inline]
    fn any<F>(&mut self, f: F) -> bool
    where
        F: FnMut(Self::Item) -> bool,
    {
        self.0.any(f)
    }

    #[inline]
    fn find<P>(&mut self, predicate: P) -> Option<Self::Item>
    where
        P: FnMut(&Self::Item) -> bool,
    {
        self.0.find(predicate)
    }

    #[inline]
    fn position<P>(&mut self, predicate: P) -> Option<usize>
    where
        P: FnMut(Self::Item) -> bool,
    {
        self.0.position(predicate)
    }

    #[inline]
    fn rposition<P>(&mut self, predicate: P) -> Option<usize>
    where
        P: FnMut(Self::Item) -> bool,
    {
        self.0.rposition(predicate)
    }
}

impl DoubleEndedIterator for Bytes<'_> {
    #[inline]
    fn next_back(&mut self) -> Option<u8> {
        self.0.next_back()
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth_back(n)
    }

    #[inline]
    fn rfind<P>(&mut self, predicate: P) -> Option<Self::Item>
    where
        P: FnMut(&Self::Item) -> bool,
    {
        self.0.rfind(predicate)
    }
}

impl ExactSizeIterator for Bytes<'_> {
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl FusedIterator for Bytes<'_> {}
