use flipperzero_sys as sys;

/// FlipperZero apps that can generate [`Deed`]s.
///
/// This list may grow over time, and it is not recommended to exhaustively match against
/// it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum App {
    SubGhz,
    Rfid,
    Nfc,
    Ir,
    Ibutton,
    Badusb,
    Plugin,

    /// Any `DolphinApp` enum value from the Flipper Zero SDK that's not part of this
    /// list.
    ///
    /// Apps that are `Uncategorized` now may move to a different or a new [`App`] variant
    /// in the future.
    #[non_exhaustive]
    #[doc(hidden)]
    Uncategorized(sys::DolphinApp),
}

impl App {
    fn from_raw(raw: sys::DolphinApp) -> Self {
        match raw {
            sys::DolphinApp_DolphinAppSubGhz => App::SubGhz,
            sys::DolphinApp_DolphinAppRfid => App::Rfid,
            sys::DolphinApp_DolphinAppNfc => App::Nfc,
            sys::DolphinApp_DolphinAppIr => App::Ir,
            sys::DolphinApp_DolphinAppIbutton => App::Ibutton,
            sys::DolphinApp_DolphinAppBadusb => App::Badusb,
            sys::DolphinApp_DolphinAppPlugin => App::Plugin,
            raw => App::Uncategorized(raw),
        }
    }

    fn to_raw(self) -> sys::DolphinApp {
        match self {
            App::SubGhz => sys::DolphinApp_DolphinAppSubGhz,
            App::Rfid => sys::DolphinApp_DolphinAppRfid,
            App::Nfc => sys::DolphinApp_DolphinAppNfc,
            App::Ir => sys::DolphinApp_DolphinAppIr,
            App::Ibutton => sys::DolphinApp_DolphinAppIbutton,
            App::Badusb => sys::DolphinApp_DolphinAppBadusb,
            App::Plugin => sys::DolphinApp_DolphinAppPlugin,
            App::Uncategorized(raw) => raw,
        }
    }

    /// Returns the limit for this app.
    ///
    /// The FlipperZero SDK refers to returned value as `icounter_limit`.
    pub fn limit(self) -> u8 {
        unsafe { sys::dolphin_deed_get_app_limit(self.to_raw()) }
    }
}

/// Deeds that can contribute to the level of your [`Dolphin`].
///
/// This list may grow over time, and it is not recommended to exhaustively match against
/// it.
///
/// [`Dolphin`]: super::Dolphin
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Deed {
    SubGhzReceiverInfo,
    SubGhzSave,
    SubGhzRawRec,
    SubGhzAddManually,
    SubGhzSend,
    SubGhzFrequencyAnalyzer,

    RfidRead,
    RfidReadSuccess,
    RfidSave,
    RfidEmulate,
    RfidAdd,

    NfcRead,
    NfcReadSuccess,
    NfcSave,
    NfcDetectReader,
    NfcEmulate,
    NfcMfcAdd,
    NfcAddSave,
    NfcAddEmulate,

    IrSend,
    IrLearnSuccess,
    IrSave,

    IbuttonRead,
    IbuttonReadSuccess,
    IbuttonSave,
    IbuttonEmulate,
    IbuttonAdd,

    BadUsbPlayScript,

    U2fAuthorized,
    GpioUartBridge,

    PluginStart,
    PluginGameStart,
    PluginGameWin,

    /// Any `DolphinDeed` enum value from the Flipper Zero SDK that's not part of this
    /// list.
    ///
    /// Deeds that are `Uncategorized` now may move to a different or a new [`Deed`]
    /// variant in the future.
    #[non_exhaustive]
    #[doc(hidden)]
    Uncategorized(sys::DolphinDeed),
}

impl Deed {
    pub(super) fn to_raw(self) -> sys::DolphinDeed {
        match self {
            Deed::SubGhzReceiverInfo => sys::DolphinDeed_DolphinDeedSubGhzReceiverInfo,
            Deed::SubGhzSave => sys::DolphinDeed_DolphinDeedSubGhzSave,
            Deed::SubGhzRawRec => sys::DolphinDeed_DolphinDeedSubGhzRawRec,
            Deed::SubGhzAddManually => sys::DolphinDeed_DolphinDeedSubGhzAddManually,
            Deed::SubGhzSend => sys::DolphinDeed_DolphinDeedSubGhzSend,
            Deed::SubGhzFrequencyAnalyzer => sys::DolphinDeed_DolphinDeedSubGhzFrequencyAnalyzer,
            Deed::RfidRead => sys::DolphinDeed_DolphinDeedRfidRead,
            Deed::RfidReadSuccess => sys::DolphinDeed_DolphinDeedRfidReadSuccess,
            Deed::RfidSave => sys::DolphinDeed_DolphinDeedRfidSave,
            Deed::RfidEmulate => sys::DolphinDeed_DolphinDeedRfidEmulate,
            Deed::RfidAdd => sys::DolphinDeed_DolphinDeedRfidAdd,
            Deed::NfcRead => sys::DolphinDeed_DolphinDeedNfcRead,
            Deed::NfcReadSuccess => sys::DolphinDeed_DolphinDeedNfcReadSuccess,
            Deed::NfcSave => sys::DolphinDeed_DolphinDeedNfcSave,
            Deed::NfcDetectReader => sys::DolphinDeed_DolphinDeedNfcDetectReader,
            Deed::NfcEmulate => sys::DolphinDeed_DolphinDeedNfcEmulate,
            Deed::NfcMfcAdd => sys::DolphinDeed_DolphinDeedNfcMfcAdd,
            Deed::NfcAddSave => sys::DolphinDeed_DolphinDeedNfcAddSave,
            Deed::NfcAddEmulate => sys::DolphinDeed_DolphinDeedNfcAddEmulate,
            Deed::IrSend => sys::DolphinDeed_DolphinDeedIrSend,
            Deed::IrLearnSuccess => sys::DolphinDeed_DolphinDeedIrLearnSuccess,
            Deed::IrSave => sys::DolphinDeed_DolphinDeedIrSave,
            Deed::IbuttonRead => sys::DolphinDeed_DolphinDeedIbuttonRead,
            Deed::IbuttonReadSuccess => sys::DolphinDeed_DolphinDeedIbuttonReadSuccess,
            Deed::IbuttonSave => sys::DolphinDeed_DolphinDeedIbuttonSave,
            Deed::IbuttonEmulate => sys::DolphinDeed_DolphinDeedIbuttonEmulate,
            Deed::IbuttonAdd => sys::DolphinDeed_DolphinDeedIbuttonAdd,
            Deed::BadUsbPlayScript => sys::DolphinDeed_DolphinDeedBadUsbPlayScript,
            Deed::U2fAuthorized => sys::DolphinDeed_DolphinDeedU2fAuthorized,
            Deed::GpioUartBridge => sys::DolphinDeed_DolphinDeedGpioUartBridge,
            Deed::PluginStart => sys::DolphinDeed_DolphinDeedPluginStart,
            Deed::PluginGameStart => sys::DolphinDeed_DolphinDeedPluginGameStart,
            Deed::PluginGameWin => sys::DolphinDeed_DolphinDeedPluginGameWin,
            Deed::Uncategorized(raw) => raw,
        }
    }

    /// Returns the FlipperZero app that this deed canonically corresponds to.
    pub fn app(self) -> App {
        App::from_raw(unsafe { sys::dolphin_deed_get_app(self.to_raw()) })
    }

    /// Returns the weight of this deed.
    ///
    /// The FlipperZero SDK refers to returned value as `icounter`.
    pub fn weight(self) -> u8 {
        unsafe { sys::dolphin_deed_get_weight(self.to_raw()) }
    }
}
