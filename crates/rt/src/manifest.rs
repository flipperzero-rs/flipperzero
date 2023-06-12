//! Flipper Zero Manifest.

use core::ffi::c_char;

use flipperzero_sys as sys;

const MANIFEST_MAGIC: u32 = 0x52474448;
const HARDWARE_TARGET: u16 = 7;
const DEFAULT_STACK_SIZE: u16 = 2048; // 2 KiB

/// Define application manifest.
///
/// # Examples
///
/// ```
/// # use flipperzero_rt::manifest;
/// manifest!(
///     name = "MyApp",
///     stack_size = 1024,
///     app_version = 1,
///     has_icon = true,
///     icon: "app.icon",
/// );
/// ```
#[macro_export]
macro_rules! manifest {
    ($($field:ident = $value:expr),* $(,)?) => {
        #[no_mangle]
        #[link_section = ".fapmeta"]
        static FAP_MANIFEST: $crate::manifest::ApplicationManifestV1 = $crate::manifest::ApplicationManifestV1 {
            $( $field: $crate::_manifest_field!($field = $value), )*
            .. $crate::manifest::ApplicationManifestV1::default()
        };
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! _manifest_field {
    (stack_size = $value:expr) => {
        $value
    };
    (app_version = $value:expr) => {
        $value
    };
    (name = $value:expr) => {
        $crate::manifest::_padded($value.as_bytes())
    };
    (has_icon = $value:expr) => {
        $value as core::ffi::c_char
    };
    (icon = $value:expr) => {
        $crate::manifest::_padded(include_bytes!($value))
    };
}

#[repr(C, packed)]
pub struct ManifestBase {
    pub manifest_magic: u32,
    pub manifest_version: u32,
    pub api_version: u32,
    pub hardware_target_id: u16,
}

/// Application Manifest (version 1).
#[repr(C, packed)]
pub struct ApplicationManifestV1 {
    pub base: ManifestBase,
    pub stack_size: u16,
    pub app_version: u32,
    pub name: [u8; 32],
    pub has_icon: c_char,
    pub icon: [u8; 32],
}

impl ApplicationManifestV1 {
    /// Default manifest.
    pub const fn default() -> Self {
        Self {
            base: ManifestBase {
                manifest_magic: MANIFEST_MAGIC,
                manifest_version: 1,
                api_version: sys::API_VERSION,
                hardware_target_id: HARDWARE_TARGET,
            },
            stack_size: DEFAULT_STACK_SIZE,
            app_version: 1,
            name: _padded(b"Application"),
            has_icon: 0,
            icon: [0; 32],
        }
    }
}

/// Pads an array with NULL bytes.
/// Ensures that there's always a NULL byte at the end.
#[doc(hidden)]
pub const fn _padded<const SIZE: usize>(value: &[u8]) -> [u8; SIZE] {
    let mut array: [u8; SIZE] = [0; SIZE];

    let mut i = 0;
    while i < array.len() - 1 && i < value.len() {
        array[i] = value[i];
        i += 1;
    }
    array[i] = 0;

    array
}
