// #[cfg(feature = "alloc")]
// use alloc::vec::Vec;
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

// #[cfg(feature = "alloc")]
#[derive(Clone, Debug)]
pub enum SubGhzPreset<'a> {
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
    Custom(CustomSubGhzPreset<'a>),
}

impl<'a> SubGhzPreset<'a> {
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

#[derive(Clone, Debug)]
/// This struct is defined for creating custom configurations. It enforces the null padding needed at the end. along with the power amplifier config?
pub struct CustomSubGhzPreset<'a>(&'a [u8]);

impl<'a> CustomSubGhzPreset<'a> {
    // #[cfg(feature = "alloc")]
    // pub fn builder() -> CustomSubGhzPresetBuilder {
    //     CustomSubGhzPresetBuilder {
    //         data: Vec::new(),
    //         pa_table: [0; 8],
    //     }
    // }

    // #[cfg(feature = "alloc")]
    // /// TODO: best way to describe the capacity variable?
    // pub fn builder_with_capacity(capacity: usize) -> CustomSubGhzPresetBuilder {
    //     CustomSubGhzPresetBuilder {
    //         data: Vec::with_capacity(capacity + 8),
    //         pa_table: [0; 8],
    //     }
    // }

    /// # Safety
    /// Read the manual for the CC1101 chip and review the [`furi_hal_subghz_load_custom_preset()`](https://github.com/flipperdevices/flipperzero-firmware/blob/0eaad8bf64f01a6f932647a9cda5475dd9ea1524/targets/f7/furi_hal/furi_hal_subghz.c#L162) function provided by the flipper API
    pub unsafe fn from_raw(raw_config: &[u8]) -> CustomSubGhzPreset {
        CustomSubGhzPreset(raw_config)
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr()
    }
}

// #[cfg(feature = "alloc")]
// pub struct CustomSubGhzPresetBuilder {
//     data: Vec<u8>,
//     pa_table: [u8; 8],
// }

// impl CustomSubGhzPresetBuilder {
//     pub fn build(mut self) -> CustomSubGhzPreset {
//         // Null Terminate the register/data pairs.
//         self.data.extend_from_slice(&[0, 0]);

//         self.data.extend_from_slice(&self.pa_table);

//         CustomSubGhzPreset(self.data)
//     }
// }
