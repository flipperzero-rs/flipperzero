use core::fmt;

use digest::{
    block_buffer::Eager,
    core_api::{Buffer, BufferKindUser, CoreWrapper, FixedOutputCore, UpdateCore},
    crypto_common::{AlgorithmName, Block, BlockSizeUser},
    typenum::{Unsigned, U32, U64},
    HashMarker, Output, OutputSizeUser, Reset,
};
use flipperzero_sys as sys;

/// The [SHA-256 hash function][1].
///
/// Equivalent to [`sha2::Sha256`].
///
/// [1]: https://en.wikipedia.org/wiki/SHA-2
///
/// [`sha2::Sha256`]: https://docs.rs/sha2/latest/sha2/type.Sha256.html
pub type Sha256 = CoreWrapper<Sha256Core>;

/// Core block-level SHA-256 hasher.
pub struct Sha256Core {
    state: sys::sha256_context,
}

impl HashMarker for Sha256Core {}

impl BlockSizeUser for Sha256Core {
    type BlockSize = U64;
}

impl BufferKindUser for Sha256Core {
    type BufferKind = Eager;
}

impl OutputSizeUser for Sha256Core {
    type OutputSize = U32;
}

impl Default for Sha256Core {
    #[inline]
    fn default() -> Self {
        let mut state = sys::sha256_context {
            total: [0; 2],
            state: [0; 8],
            wbuf: [0; 16],
        };
        unsafe { sys::sha256_start(&mut state) };
        Self { state }
    }
}

impl UpdateCore for Sha256Core {
    #[inline]
    fn update_blocks(&mut self, blocks: &[Block<Self>]) {
        for block in blocks {
            self.state.total[0] += Self::BlockSize::U32; // i.e. 64u32
            if self.state.total[0] < Self::BlockSize::U32 {
                self.state.total[1] += 1;
            }

            unsafe {
                core::ptr::copy_nonoverlapping(
                    block.as_ptr(),
                    self.state.wbuf.as_mut_ptr().cast(),
                    Self::BlockSize::USIZE,
                );
                sys::sha256_process(&mut self.state);
            }
        }
    }
}

impl FixedOutputCore for Sha256Core {
    #[inline]
    fn finalize_fixed_core(&mut self, buffer: &mut Buffer<Self>, out: &mut Output<Self>) {
        unsafe {
            sys::sha256_update(
                &mut self.state,
                buffer.get_data().as_ptr(),
                buffer.get_data().len() as u32,
            );
            sys::sha256_finish(&mut self.state, out.as_mut_ptr());
        }
    }
}

impl Reset for Sha256Core {
    #[inline]
    fn reset(&mut self) {
        *self = Default::default();
    }
}

impl AlgorithmName for Sha256Core {
    fn write_alg_name(f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Sha256")
    }
}

#[flipperzero_test::tests]
mod tests {
    use digest::Digest;

    use super::Sha256;

    #[test]
    fn rustcrypto() {
        let mut fz = Sha256::new();
        let mut rc = sha2::Sha256::new();

        for i in 0..100 {
            let buf = [i; 50];
            fz.update(buf);
            rc.update(buf);
        }

        assert_eq!(fz.finalize(), rc.finalize());
    }
}
