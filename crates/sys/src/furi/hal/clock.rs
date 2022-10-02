//! Furi HAL Clock API.

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FuriHalClockMcoSourceId {
    Lse,
    Sysclk,
    Msi100k,
    Msi200k,
    Msi400k,
    Msi800k,
    Msi1m,
    Msi2m,
    Msi4m,
    Msi8m,
    Msi16m,
    Msi24m,
    Msi32m,
    Msi48m,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FuriHalClockMcoDivisorId {
    /// MCO not divided
    Div1 = 0x00000000,
    /// MCO divided by 2
    Div2 = 0x10000000,
    /// MCO divided by 4
    Div4 = 0x20000000,
    /// MCO divided by 8
    Div8 = 0x30000000,
    /// MCO divided by 16
    Div16 = 0x40000000,
}

extern "C" {
    /// Disable clock output on MCO pin.
    #[link_name = "furi_hal_clock_mco_disable"]
    pub fn mco_disable();
    /// Enable clock output on MCO pin.
    #[link_name = "furi_hal_clock_mco_enable"]
    pub fn mco_enable(source: FuriHalClockMcoSourceId, div: FuriHalClockMcoDivisorId);
}
