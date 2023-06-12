use flipperzero_sys as sys;

/// The [CRC32 error-detecting code][1].
///
/// Equivalent to [`crc32fast::Hasher`].
///
/// [1]: https://en.wikipedia.org/wiki/Cyclic_redundancy_check
///
/// [`crc32fast::Hasher`]: https://docs.rs/crc32fast/latest/crc32fast/struct.Hasher.html
#[derive(Clone)]
pub struct Crc32 {
    state: u32,
}

impl Crc32 {
    /// Creates a new CRC32 calculator.
    pub fn new() -> Self {
        Self { state: 0 }
    }

    /// Initializes a new CRC32 calculator with a specific initial state.
    pub fn new_with_initial(init: u32) -> Self {
        Self { state: init }
    }

    /// Processes the given buffer, updating the internal state.
    pub fn update(&mut self, buf: &[u8]) {
        self.state = unsafe { sys::crc32_calc_buffer(self.state, buf.as_ptr().cast(), buf.len()) };
    }

    /// Retrieves the computed CRC32 value and consumes the calculator instance.
    pub fn finalize(self) -> u32 {
        self.state
    }

    /// Resets the internal state.
    ///
    /// This is equivalent to `*self = Crc32::new()`, and does not recover any initial
    /// state that might have been passed to a previous [`Crc32::new_with_initial`] call.
    pub fn reset(&mut self) {
        self.state = 0;
    }
}

impl Default for Crc32 {
    fn default() -> Self {
        Self::new()
    }
}

#[flipperzero_test::tests]
mod tests {
    use super::Crc32;

    #[test]
    fn crc32fast() {
        let mut fz = Crc32::new();
        let mut rs = crc32fast::Hasher::new();

        for i in 0..100 {
            let buf = [i; 50];
            fz.update(&buf);
            rs.update(&buf);
        }

        assert_eq!(fz.finalize(), rs.finalize());
    }

    #[test]
    fn crc32fast_with_init() {
        for init in 0..5 {
            let mut fz = Crc32::new_with_initial(init);
            let mut rs = crc32fast::Hasher::new_with_initial(init);

            for i in 0..100 {
                let buf = [i; 50];
                fz.update(&buf);
                rs.update(&buf);
            }

            assert_eq!(fz.finalize(), rs.finalize());
        }
    }
}
