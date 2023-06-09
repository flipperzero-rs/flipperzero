use core::fmt;

use digest::{
    block_buffer::Eager,
    core_api::{Buffer, BufferKindUser, CoreWrapper, FixedOutputCore, UpdateCore},
    crypto_common::{AlgorithmName, Block, BlockSizeUser},
    typenum::{Unsigned, U16, U64},
    HashMarker, Output, OutputSizeUser, Reset,
};
use flipperzero_sys as sys;

/// The [MD5 hash function][1].
///
/// Equivalent to [`md5::Md5`].
///
/// ## ⚠️ Security Warning
///
/// This type is provided for the purposes of legacy interoperability with protocols and
/// systems which mandate the use of MD5.
///
/// However, MD5 is [cryptographically broken and unsuitable for further use][2].
///
/// Collision attacks against MD5 are both practical and trivial, and
/// [theoretical attacks against MD5's preimage resistance have been found][3].
///
/// [RFC6151][4] advises no new IETF protocols can be designed using MD5-based
/// constructions, including HMAC-MD5.
///
/// [1]: https://en.wikipedia.org/wiki/MD5
/// [2]: https://www.kb.cert.org/vuls/id/836068
/// [3]: https://dl.acm.org/citation.cfm?id=1724151
/// [4]: https://tools.ietf.org/html/rfc6151
///
/// [`md5::Md5`]: https://docs.rs/md-5/latest/md5/type.Md5.html
pub type Md5 = CoreWrapper<Md5Core>;

/// Core MD5 hasher.
pub struct Md5Core {
    state: sys::md5_context,
}

impl HashMarker for Md5Core {}

impl BlockSizeUser for Md5Core {
    type BlockSize = U64;
}

impl BufferKindUser for Md5Core {
    type BufferKind = Eager;
}

impl OutputSizeUser for Md5Core {
    type OutputSize = U16;
}

impl Default for Md5Core {
    #[inline]
    fn default() -> Self {
        let mut state = sys::md5_context {
            total: [0; 2],
            state: [0; 4],
            buffer: [0; 64],
        };
        unsafe { sys::md5_starts(&mut state) };
        Self { state }
    }
}

impl UpdateCore for Md5Core {
    #[inline]
    fn update_blocks(&mut self, blocks: &[Block<Self>]) {
        for block in blocks {
            self.state.total[0] += Self::BlockSize::U32; // i.e. 64u32
            if self.state.total[0] < Self::BlockSize::U32 {
                self.state.total[1] += 1;
            }
            unsafe { sys::md5_process(&mut self.state, block.as_ptr()) };
        }
    }
}

impl FixedOutputCore for Md5Core {
    #[inline]
    fn finalize_fixed_core(&mut self, buffer: &mut Buffer<Self>, out: &mut Output<Self>) {
        unsafe {
            sys::md5_update(
                &mut self.state,
                buffer.get_data().as_ptr(),
                buffer.get_data().len(),
            );
            sys::md5_finish(&mut self.state, out.as_mut_ptr());
        }
    }
}

impl Reset for Md5Core {
    #[inline]
    fn reset(&mut self) {
        *self = Default::default();
    }
}

impl AlgorithmName for Md5Core {
    fn write_alg_name(f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Md5")
    }
}

#[flipperzero_test::tests]
mod tests {
    use digest::Digest;

    use super::Md5;

    #[test]
    fn rustcrypto() {
        let mut fz = Md5::new();
        let mut rc = md5::Md5::new();

        for i in 0..100 {
            let buf = [i; 50];
            fz.update(buf);
            rc.update(buf);
        }

        assert_eq!(fz.finalize(), rc.finalize());
    }
}
