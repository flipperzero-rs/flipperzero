//! Furi HAL Crypto API.

/// Key type.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FuriHalCryptoKeyType {
    /// Master key
    Master,
    /// Simple encrypted key
    Simple,
    /// Encrypted with Master key
    Encrypted,
}

/// Key size bits.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FuriHalCryptoKeySize {
    KeySize128,
    KeySize256,
}

/// Crypto key.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FuriHalCryptoKey {
    key_type: FuriHalCryptoKeyType,
    size: FuriHalCryptoKeySize,
    data: *mut u8,
}

extern "C" {
    /// Decrypt data.
    #[link_name = "furi_hal_crypto_decrypt"]
    pub fn decrypt(input: *const u8, output: *mut u8, size: usize) -> bool;
    /// Encrypt data.
    #[link_name = "furi_hal_crypto_encrypt"]
    pub fn encrypt(input: *const u8, output: *mut u8, size: usize) -> bool;
    /// Store key in crypto storage.
    /// `slot` is a pointer where slot number will be saved.
    #[link_name = "furi_hal_crypto_store_add_key"]
    pub fn store_add_key(key: *mut FuriHalCryptoKey, slot: *mut u8) -> bool;
    /// Init AES engine and load key from crypto store.
    /// `iv` is a pointer to 16-bytes Initialization Vector data.
    #[link_name = "furi_hal_crypto_store_load_key"]
    pub fn store_load_key(slot: u8, iv: *const u8) -> bool;
    /// Unload key engine and deinit AES engine
    #[link_name = "furi_hal_crypto_store_unload_key"]
    pub fn store_unload_key(slot: u8) -> bool;
    #[link_name = "furi_hal_crypto_verify_enclave"]
    pub fn verify_enclave(keys_nb: *mut u8, valid_keys_nb: *mut u8) -> bool;
    #[link_name = "furi_hal_crypto_verify_key"]
    pub fn verify_key(key_slot: u8) -> bool;
}
