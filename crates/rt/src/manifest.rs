//! Flipper Zero Manifest.

use core::ffi::c_char;

const MANIFEST_MAGIC: u32 = 0x52474448;
const API_MAJOR: u16 = 2;
const API_MINOR: u16 = 2;
const HARDWARE_TARGET: u16 = 7;
const DEFAULT_STACK_SIZE: u16 = 2048; // 2 KiB

#[macro_export]
macro_rules! manifest {
    ($($field:ident = $value:expr),*) => {
        #[no_mangle]
        #[link_section = ".fapmeta"]
        static FAP_MANIFEST: $crate::manifest::ApplicationManifestV1 = $crate::manifest::ApplicationManifestV1 {
            $( $field: $crate::manifest::set::$field($value), )*
            .. $crate::manifest::ApplicationManifestV1::default()
        };
    };
}

#[macro_export]
macro_rules! _manifest_field {
    (stack_size = $value:expr) => {
        stack_size: $value
    };

    (app_version = $value:expr) => {
        app_version: $value
    };

    (name = $value:expr) => {
        name: $crate::manifest::padded($value.as_bytes())
    }
}

/// Manifest setters
pub mod set {
    use super::padded;

    pub const fn stack_size(n_bytes: u16) -> u16 {
        n_bytes
    }

    pub const fn app_version(version: u32) -> u32 {
        version
    }

    pub const fn name<const SIZE: usize>(name: &str) -> [u8; SIZE] {
        padded(name.as_bytes())
    }
}

#[repr(C, packed)]
pub struct ManifestBase {
    pub manifest_magic: u32,
    pub manifest_version: u32,
    pub api_version_major: u16,
    pub api_version_minor: u16,
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
                api_version_major: API_MAJOR,
                api_version_minor: API_MINOR,
                hardware_target_id: HARDWARE_TARGET,
            },
            stack_size: DEFAULT_STACK_SIZE,
            app_version: 1,
            name: padded(b"Application"),
            has_icon: 0,
            icon: [0; 32],
        }
    }
}

/// Pads an array with NULL bytes.
/// Ensures that there's always a NULL byte at the end.
const fn padded<const SIZE: usize>(value: &[u8]) -> [u8; SIZE] {
    let mut array: [u8; SIZE] = [0; SIZE];

    let mut i = 0;
    while i < array.len() - 1 && i < value.len() {
        array[i] = value[i];
        i += 1;
    }
    array[i] = 0;

    array
}
