//! Internal implementation details.

use core::{marker::PhantomData, mem};

/// Marker type which is neither [`Send`] nor [`Sync`].
/// This should be used until `negative_trait_bounds` Rust feature is stable.
///
/// # Example
///
/// Make type `Foo` `impl !Sync` and `impl !Send`:
///
/// ```no_run
/// struct Foo {
///     _marker: UnsendUnsync,
/// }
/// ```
pub(crate) struct UnsendUnsync(*const ());

const _: () = {
    assert!(
        mem::size_of::<PhantomData<UnsendUnsync>>() == 0,
        "`PhantomData<UnsendUnsync>` should be a ZST"
    );
};

/// Marker type which is not [`Send`] but is [`Sync`].
/// This should be used until `negative_trait_bounds` Rust feature is stable.
///
/// # Example
///
/// Make type `Foo` `impl !Send`:
///
/// ```no_run
/// struct Foo {
///     _marker: Unsend,
/// }
/// ```
pub(crate) struct Unsend(*const ());

// SAFETY: `Unsend` is just a marker struct
unsafe impl Sync for Unsend {}

const _: () = {
    assert!(
        mem::size_of::<PhantomData<Unsend>>() == 0,
        "`PhantomData<Unsend>` should be a ZST"
    );
};

/// Operations which have unstable implementations
/// but still may be implemented manually on `stable` channel.
///
/// This will use core implementations if `unstable_intrinsics` feature is enabled
/// falling back to ad-hoc implementations otherwise.
pub(crate) mod ops {
    pub const fn div_ceil_u16(divident: u16, divisor: u16) -> u16 {
        #[cfg(feature = "unstable_intrinsics")]
        {
            divident.div_ceil(divisor)
        }
        #[cfg(not(feature = "unstable_intrinsics"))]
        {
            if divident % divisor == 0 {
                divident / divisor
            } else {
                divident / divisor + 1
            }
        }
    }
}
