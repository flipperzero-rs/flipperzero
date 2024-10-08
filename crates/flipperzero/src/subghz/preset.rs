#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use flipperzero_sys::{
    FuriHalSubGhzPreset, FuriHalSubGhzPreset_FuriHalSubGhzPreset2FSKDev238Async,
    FuriHalSubGhzPreset_FuriHalSubGhzPreset2FSKDev476Async,
    FuriHalSubGhzPreset_FuriHalSubGhzPresetCustom,
    FuriHalSubGhzPreset_FuriHalSubGhzPresetGFSK9_99KbAsync,
    FuriHalSubGhzPreset_FuriHalSubGhzPresetIDLE,
    FuriHalSubGhzPreset_FuriHalSubGhzPresetMSK99_97KbAsync,
    FuriHalSubGhzPreset_FuriHalSubGhzPresetOok270Async,
    FuriHalSubGhzPreset_FuriHalSubGhzPresetOok650Async,
};

#[cfg(feature = "alloc")]
#[derive(Clone, Debug)]
pub enum SubGhzPreset {
    /// default configuration
    IDLE,
    /// OOK, bandwidth 270kHz, asynchronous
    Ook270Async,
    /// OOK, bandwidth 650kHz, asynchronous
    Ook650Async,
    /// FM, deviation 2.380371 kHz, asynchronous
    _2FSKDev238Async,
    /// FM, deviation 47.60742 kHz, asynchronous
    _2FSKDev476Async,
    /// MSK, deviation 47.60742 kHz, 99.97Kb/s, asynchronous
    MSK99_97KbAsync,
    /// GFSK, deviation 19.042969 kHz, 9.996Kb/s, asynchronous
    GFSK9_99KbAsync,
    Custom(CustomSubGhzPreset),
}

impl SubGhzPreset {
    pub fn into_furi_preset(&self) -> FuriHalSubGhzPreset {
        match self {
            SubGhzPreset::IDLE => FuriHalSubGhzPreset_FuriHalSubGhzPresetIDLE,
            SubGhzPreset::Ook270Async => FuriHalSubGhzPreset_FuriHalSubGhzPresetOok270Async,
            SubGhzPreset::Ook650Async => FuriHalSubGhzPreset_FuriHalSubGhzPresetOok650Async,
            SubGhzPreset::_2FSKDev238Async => {
                FuriHalSubGhzPreset_FuriHalSubGhzPreset2FSKDev238Async
            }
            SubGhzPreset::_2FSKDev476Async => {
                FuriHalSubGhzPreset_FuriHalSubGhzPreset2FSKDev476Async
            }
            SubGhzPreset::MSK99_97KbAsync => FuriHalSubGhzPreset_FuriHalSubGhzPresetMSK99_97KbAsync,
            SubGhzPreset::GFSK9_99KbAsync => FuriHalSubGhzPreset_FuriHalSubGhzPresetGFSK9_99KbAsync,
            SubGhzPreset::Custom(_) => FuriHalSubGhzPreset_FuriHalSubGhzPresetCustom,
        }
    }
}

#[cfg(feature = "alloc")]
impl From<SubGhzBuiltinPreset> for SubGhzPreset {
    fn from(value: SubGhzBuiltinPreset) -> Self {
        match value {
            SubGhzBuiltinPreset::IDLE => Self::IDLE,
            SubGhzBuiltinPreset::Ook270Async => Self::Ook270Async,
            SubGhzBuiltinPreset::Ook650Async => Self::Ook650Async,
            SubGhzBuiltinPreset::_2FSKDev238Async => Self::_2FSKDev238Async,
            SubGhzBuiltinPreset::_2FSKDev476Async => Self::_2FSKDev476Async,
            SubGhzBuiltinPreset::MSK99_97KbAsync => Self::MSK99_97KbAsync,
            SubGhzBuiltinPreset::GFSK9_99KbAsync => Self::GFSK9_99KbAsync,
        }
    }
}

#[cfg(feature = "alloc")]
#[derive(Clone, Debug)]
/// This struct is defined for creating custom configurations. It enforces the null padding needed at the end. along with the power amplifier config?
pub struct CustomSubGhzPreset(Vec<u8>);

impl CustomSubGhzPreset {
    pub fn builder() -> CustomSubGhzPresetBuilder {
        CustomSubGhzPresetBuilder(Vec::new())
    }

    pub fn as_mut_ptr(mut self) -> *mut u8 {
        self.0.as_mut_ptr()
    }
}

#[cfg(feature = "alloc")]
pub struct CustomSubGhzPresetBuilder(Vec<u8>);

impl CustomSubGhzPresetBuilder {
    pub fn build(mut self) -> CustomSubGhzPreset {
        // Pad with 7 zeros, in theory only need 2, but the built in ones seem to have about 7
        // Realize I have slightly misinterpreted it, I will think more later.
        self.0.extend_from_slice(&[0; 7]);

        CustomSubGhzPreset(self.0)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SubGhzBuiltinPreset {
    /// default configuration
    IDLE,
    /// OOK, bandwidth 270kHz, asynchronous
    Ook270Async,
    /// OOK, bandwidth 650kHz, asynchronous
    Ook650Async,
    /// FM, deviation 2.380371 kHz, asynchronous
    _2FSKDev238Async,
    /// FM, deviation 47.60742 kHz, asynchronous
    _2FSKDev476Async,
    /// MSK, deviation 47.60742 kHz, 99.97Kb/s, asynchronous
    MSK99_97KbAsync,
    /// GFSK, deviation 19.042969 kHz, 9.996Kb/s, asynchronous
    GFSK9_99KbAsync,
}

impl SubGhzBuiltinPreset {
    pub fn into_furi_preset(self) -> FuriHalSubGhzPreset {
        match self {
            SubGhzBuiltinPreset::IDLE => FuriHalSubGhzPreset_FuriHalSubGhzPresetIDLE,
            SubGhzBuiltinPreset::Ook270Async => FuriHalSubGhzPreset_FuriHalSubGhzPresetOok270Async,
            SubGhzBuiltinPreset::Ook650Async => FuriHalSubGhzPreset_FuriHalSubGhzPresetOok650Async,
            SubGhzBuiltinPreset::_2FSKDev238Async => {
                FuriHalSubGhzPreset_FuriHalSubGhzPreset2FSKDev238Async
            }
            SubGhzBuiltinPreset::_2FSKDev476Async => {
                FuriHalSubGhzPreset_FuriHalSubGhzPreset2FSKDev476Async
            }
            SubGhzBuiltinPreset::MSK99_97KbAsync => {
                FuriHalSubGhzPreset_FuriHalSubGhzPresetMSK99_97KbAsync
            }
            SubGhzBuiltinPreset::GFSK9_99KbAsync => {
                FuriHalSubGhzPreset_FuriHalSubGhzPresetGFSK9_99KbAsync
            }
        }
    }
}
