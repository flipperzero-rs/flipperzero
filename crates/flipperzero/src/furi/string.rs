//! String primitives built around `FuriString`.

use core::{
    cmp::Ordering,
    convert::Infallible,
    ffi::{c_char, CStr},
    fmt::{self, Write},
    hash,
    ops::{Add, AddAssign},
    ptr,
};

#[cfg(feature = "alloc")]
use alloc::{borrow::Cow, boxed::Box, ffi::CString};

use flipperzero_sys as sys;

/// A Furi string.
///
/// This is similar to Rust's [`CString`] in that it represents an owned, C-compatible,
/// nul-terminated string with no nul bytes in the middle. It also has additional methods
/// to provide the flexibility of Rust's [`String`]. It is used in various APIs of the
/// Flipper Zero SDK.
///
/// This type does not requre the `alloc` feature flag, because it does not use the Rust
/// allocator. Very short strings (7 bytes or fewer) are stored directly inside the
/// `FuriString` struct (which is stored on the heap), while longer strings are allocated
/// on the heap by the Flipper Zero firmware.
#[derive(Eq)]
pub struct String(*mut sys::FuriString);

impl Drop for String {
    fn drop(&mut self) {
        unsafe { sys::furi_string_free(self.0) };
    }
}

// Implementations matching `std::string::String`.
impl String {
    /// Creates a new empty `String`.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        String(unsafe { sys::furi_string_alloc() })
    }

    /// Creates a new empty `String` with at least the specified capacity.
    #[inline]
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        let s = Self::new();
        // The size passed to `sys::furi_string_reserve` needs to include the nul
        // terminator.
        unsafe { sys::furi_string_reserve(s.0, capacity + 1) };
        s
    }

    #[inline]
    #[must_use]
    fn as_c_ptr(&self) -> *const c_char {
        unsafe { sys::furi_string_get_cstr(self.0) }
    }

    /// Extracts a `CStr` containing the entire string slice, with nul termination.
    #[inline]
    #[must_use]
    pub fn as_c_str(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_c_ptr()) }
    }

    /// Appends a given `String` onto the end of this `String`.
    #[inline]
    pub fn push_string(&mut self, string: &String) {
        unsafe { sys::furi_string_cat(self.0, string.0) }
    }

    /// Appends a given `str` onto the end of this `String`.
    #[inline]
    pub fn push_str(&mut self, string: &str) {
        self.extend(string.chars());
    }

    /// Appends a given `CStr` onto the end of this `String`.
    #[inline]
    pub fn push_c_str(&mut self, string: &CStr) {
        unsafe { sys::furi_string_cat_str(self.0, string.as_ptr()) }
    }

    /// Reserves capacity for at least `additional` bytes more than the current length.
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        // `self.len()` counts the number of bytes excluding the terminating nul, but the
        // size passed to `sys::furi_string_reserve` needs to include the nul terminator.
        unsafe { sys::furi_string_reserve(self.0, self.len() + additional + 1) };
    }

    /// Appends the given [`char`] to the end of this `String`.
    #[inline]
    pub fn push(&mut self, ch: char) {
        match ch.len_utf8() {
            1 => unsafe { sys::furi_string_push_back(self.0, ch as c_char) },
            _ => unsafe { sys::furi_string_utf8_push(self.0, ch as u32) },
        }
    }

    /// Returns a byte slice of this `String`'s contents.
    ///
    /// The returned slice will **not** contain the trailing nul terminator that the
    /// underlying C string has.
    #[inline]
    #[must_use]
    pub fn to_bytes(&self) -> &[u8] {
        self.as_c_str().to_bytes()
    }

    /// Returns a byte slice of this `String`'s contents with the trailing nul byte.
    ///
    /// This function is the equivalent of [`String::to_bytes`] except that it will retain
    /// the trailing nul terminator instead of chopping it off.
    #[inline]
    #[must_use]
    pub fn to_bytes_with_nul(&self) -> &[u8] {
        self.as_c_str().to_bytes_with_nul()
    }

    /// Shortens this `String` to the specified length.
    ///
    /// If `new_len` is greater than the string's current length, this has no effect.
    #[inline]
    pub fn truncate(&mut self, new_len: usize) {
        unsafe { sys::furi_string_left(self.0, new_len) };
    }

    /// Inserts a character into this `String` at a byte position.
    ///
    /// This is an *O*(*n*) operation as it requires copying every element in the buffer.
    ///
    /// # Panics
    ///
    /// Panics if `idx` is larger than the `String`'s length.
    #[inline]
    pub fn insert(&mut self, idx: usize, ch: char) {
        let mut buf = [0; 4];
        let encoded = ch.encode_utf8(&mut buf).as_bytes();
        self.insert_bytes(idx, encoded);
    }

    /// Inserts a string slice into this `String` at a byte position.
    ///
    /// This is an *O*(*n*) operation as it requires copying every element in the buffer.
    ///
    /// # Panics
    ///
    /// Panics if `idx` is larger than the `String`'s length.
    #[inline]
    pub fn insert_str(&mut self, idx: usize, string: &str) {
        self.insert_bytes(idx, string.as_bytes());
    }

    fn insert_bytes(&mut self, idx: usize, bytes: &[u8]) {
        let len = self.len();
        assert!(idx <= len);

        // Reserve sufficient space to insert `bytes` without repeat re-allocations.
        let amt = bytes.len();
        self.reserve(amt);

        // Append `bytes` to force the underlying `FuriString` to be the correct length.
        for byte in bytes.iter() {
            unsafe { sys::furi_string_push_back(self.0, *byte as c_char) };
        }

        // Now use pointer access to construct the expected `String` contents.
        let c_ptr = self.as_c_ptr().cast::<u8>();
        unsafe {
            ptr::copy(c_ptr.add(idx), c_ptr.cast_mut().add(idx + amt), len - idx);
            ptr::copy_nonoverlapping(bytes.as_ptr(), c_ptr.cast_mut().add(idx), amt);
        }
    }

    /// Returns the length of this `String`.
    ///
    /// This length is in bytes, not [`char`]s or graphemes. In other words, it might not
    /// be what a human considers the length of the string.
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        unsafe { sys::furi_string_size(self.0) }
    }

    /// Returns `true` if this `String` has a length of zero, and `false` otherwise.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        unsafe { sys::furi_string_empty(self.0) }
    }

    /// Splits the string into two at the given byte index.
    ///
    /// Returns a newly allocated `String`. `self` contains bytes `[0, at)`, and the
    /// returned `String` contains bytes `[at, len)`.
    ///
    /// Note that the capacity of `self` does not change.
    ///
    /// # Panics
    ///
    /// Panics if `at` is beyond the last byte of the string.
    #[inline]
    #[must_use = "use `.truncate()` if you don't need the other half"]
    pub fn split_off(&mut self, at: usize) -> String {
        // SAFETY: Trimming the beginning of a C string results in a valid C string, as
        // long as the nul byte is not trimmed.
        assert!(at <= self.len());
        let ret = String(unsafe { sys::furi_string_alloc_set_str(self.as_c_ptr().add(at)) });
        self.truncate(at);
        ret
    }

    /// Truncates this `String`, removing all contents.
    ///
    /// While this means the `String` will have a length of zero, it does not touch its
    /// capacity.
    #[inline]
    pub fn clear(&mut self) {
        unsafe { sys::furi_string_reset(self.0) };
    }
}

impl Clone for String {
    fn clone(&self) -> Self {
        Self(unsafe { sys::furi_string_alloc_set(self.0) })
    }
}

impl Default for String {
    fn default() -> Self {
        Self::new()
    }
}

impl AsRef<CStr> for String {
    #[inline]
    fn as_ref(&self) -> &CStr {
        self.as_c_str()
    }
}

impl From<char> for String {
    fn from(value: char) -> Self {
        let mut buf = String::with_capacity(value.len_utf8());
        buf.push(value);
        buf
    }
}

impl From<&str> for String {
    fn from(value: &str) -> Self {
        let mut buf = String::with_capacity(value.len());
        buf.push_str(value);
        buf
    }
}

impl From<&mut str> for String {
    fn from(value: &mut str) -> Self {
        From::<&str>::from(value)
    }
}

impl From<&CStr> for String {
    fn from(value: &CStr) -> Self {
        Self(unsafe { sys::furi_string_alloc_set_str(value.as_ptr()) })
    }
}

#[cfg(feature = "alloc")]
impl From<Box<str>> for String {
    fn from(value: Box<str>) -> Self {
        String::from(value.as_ref())
    }
}

#[cfg(feature = "alloc")]
impl<'a> From<Cow<'a, str>> for String {
    fn from(value: Cow<'a, str>) -> Self {
        String::from(value.as_ref())
    }
}

impl Extend<String> for String {
    fn extend<T: IntoIterator<Item = String>>(&mut self, iter: T) {
        iter.into_iter().for_each(move |s| self.push_string(&s));
    }
}

impl Extend<char> for String {
    fn extend<T: IntoIterator<Item = char>>(&mut self, iter: T) {
        let iterator = iter.into_iter();
        let (lower_bound, _) = iterator.size_hint();
        self.reserve(lower_bound);
        iterator.for_each(move |c| self.push(c));
    }
}

impl<'a> Extend<&'a char> for String {
    fn extend<T: IntoIterator<Item = &'a char>>(&mut self, iter: T) {
        self.extend(iter.into_iter().cloned());
    }
}

impl<'a> Extend<&'a str> for String {
    fn extend<T: IntoIterator<Item = &'a str>>(&mut self, iter: T) {
        iter.into_iter().for_each(move |s| self.push_str(s));
    }
}

impl<'a> Extend<&'a CStr> for String {
    fn extend<T: IntoIterator<Item = &'a CStr>>(&mut self, iter: T) {
        iter.into_iter().for_each(move |s| self.push_c_str(s));
    }
}

#[cfg(feature = "alloc")]
impl Extend<Box<str>> for String {
    fn extend<T: IntoIterator<Item = Box<str>>>(&mut self, iter: T) {
        iter.into_iter().for_each(move |s| self.push_str(&s));
    }
}

#[cfg(feature = "alloc")]
impl<'a> Extend<Cow<'a, str>> for String {
    fn extend<T: IntoIterator<Item = Cow<'a, str>>>(&mut self, iter: T) {
        iter.into_iter().for_each(move |s| self.push_str(&s));
    }
}

impl FromIterator<String> for String {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let mut iterator = iter.into_iter();

        // Because we're iterating over `String`s, we can avoid at least one allocation by
        // getting the first string from the iterator and appending to it all the
        // subsequent strings.
        match iterator.next() {
            None => String::new(),
            Some(mut buf) => {
                buf.extend(iterator);
                buf
            }
        }
    }
}

impl FromIterator<char> for String {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        let mut buf = String::new();
        buf.extend(iter);
        buf
    }
}

impl<'a> FromIterator<&'a char> for String {
    fn from_iter<T: IntoIterator<Item = &'a char>>(iter: T) -> Self {
        let mut buf = String::new();
        buf.extend(iter);
        buf
    }
}

impl<'a> FromIterator<&'a str> for String {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let mut buf = String::new();
        buf.extend(iter);
        buf
    }
}

impl<'a> FromIterator<&'a CStr> for String {
    fn from_iter<T: IntoIterator<Item = &'a CStr>>(iter: T) -> Self {
        let mut buf = String::new();
        buf.extend(iter);
        buf
    }
}

#[cfg(feature = "alloc")]
impl FromIterator<Box<str>> for String {
    fn from_iter<T: IntoIterator<Item = Box<str>>>(iter: T) -> Self {
        let mut buf = String::new();
        buf.extend(iter);
        buf
    }
}

#[cfg(feature = "alloc")]
impl<'a> FromIterator<Cow<'a, str>> for String {
    fn from_iter<T: IntoIterator<Item = Cow<'a, str>>>(iter: T) -> Self {
        let mut buf = String::new();
        buf.extend(iter);
        buf
    }
}

impl Add<&str> for String {
    type Output = String;

    #[inline]
    fn add(mut self, rhs: &str) -> Self::Output {
        self.push_str(rhs);
        self
    }
}

impl AddAssign<&str> for String {
    #[inline]
    fn add_assign(&mut self, rhs: &str) {
        self.push_str(rhs);
    }
}

impl PartialEq for String {
    fn eq(&self, other: &Self) -> bool {
        unsafe { sys::furi_string_equal(self.0, other.0) }
    }
}

impl PartialEq<CStr> for String {
    fn eq(&self, other: &CStr) -> bool {
        unsafe { sys::furi_string_equal_str(self.0, other.as_ptr()) }
    }
}

impl PartialEq<String> for CStr {
    fn eq(&self, other: &String) -> bool {
        other.eq(self)
    }
}

impl PartialEq<str> for String {
    fn eq(&self, other: &str) -> bool {
        self.to_bytes().eq(other.as_bytes())
    }
}

impl PartialEq<String> for str {
    fn eq(&self, other: &String) -> bool {
        other.eq(self)
    }
}

impl PartialEq<&str> for String {
    fn eq(&self, other: &&str) -> bool {
        self.eq(*other)
    }
}

impl PartialEq<String> for &str {
    fn eq(&self, other: &String) -> bool {
        other.eq(*self)
    }
}

#[cfg(feature = "alloc")]
impl PartialEq<CString> for String {
    fn eq(&self, other: &CString) -> bool {
        self.eq(other.as_c_str())
    }
}

#[cfg(feature = "alloc")]
impl PartialEq<String> for CString {
    fn eq(&self, other: &String) -> bool {
        other.eq(self.as_c_str())
    }
}

impl Ord for String {
    fn cmp(&self, other: &Self) -> Ordering {
        match unsafe { sys::furi_string_cmp(self.0, other.0) } {
            ..=-1 => Ordering::Less,
            0 => Ordering::Equal,
            1.. => Ordering::Greater,
        }
    }
}

impl PartialOrd for String {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl hash::Hash for String {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.as_c_str().hash(state);
    }
}

impl fmt::Debug for String {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char('"')?;
        for byte in self.to_bytes().escape_ascii() {
            f.write_char(byte as char)?;
        }
        f.write_char('"')
    }
}

impl ufmt::uDebug for String {
    #[inline]
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        f.write_char('"')?;
        for byte in self.to_bytes().escape_ascii() {
            f.write_char(byte as char)?;
        }
        f.write_char('"')
    }
}

impl fmt::Display for String {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.to_bytes().escape_ascii() {
            f.write_char(byte as char)?;
        }
        Ok(())
    }
}

impl ufmt::uDisplay for String {
    #[inline]
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        for byte in self.to_bytes().escape_ascii() {
            f.write_char(byte as char)?;
        }
        Ok(())
    }
}

impl fmt::Write for String {
    #[inline]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.push_str(s);
        Ok(())
    }

    #[inline]
    fn write_char(&mut self, c: char) -> fmt::Result {
        self.push(c);
        Ok(())
    }
}

impl ufmt::uWrite for String {
    type Error = Infallible;

    #[inline]
    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        self.push_str(s);
        Ok(())
    }

    #[inline]
    fn write_char(&mut self, c: char) -> Result<(), Self::Error> {
        self.push(c);
        Ok(())
    }
}
