/// There are Server commands (e.g. Storage_write), which have no body message
/// in response. But 'oneof' obligate to have at least 1 encoded message
/// in scope. For this needs Empty message is implemented.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Empty {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopSession {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Main {
    #[prost(uint32, tag = "1")]
    pub command_id: u32,
    #[prost(enumeration = "CommandStatus", tag = "2")]
    pub command_status: i32,
    #[prost(bool, tag = "3")]
    pub has_next: bool,
    #[prost(
        oneof = "main::Content",
        tags = "4, 19, 5, 6, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 46, 44, 45, 28, 29, 59, 60, 24, 25, 7, 8, 9, 10, 11, 12, 13, 14, 15, 30, 42, 43, 16, 17, 18, 47, 48, 49, 50, 63, 64, 65, 20, 21, 22, 23, 26, 27, 51, 52, 53, 54, 55, 56, 57, 58, 61, 62, 66, 67, 68, 69, 70"
    )]
    pub content: ::core::option::Option<main::Content>,
}
/// Nested message and enum types in `Main`.
pub mod main {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Content {
        #[prost(message, tag = "4")]
        Empty(super::Empty),
        #[prost(message, tag = "19")]
        StopSession(super::StopSession),
        #[prost(message, tag = "5")]
        SystemPingRequest(super::super::pb_system::PingRequest),
        #[prost(message, tag = "6")]
        SystemPingResponse(super::super::pb_system::PingResponse),
        #[prost(message, tag = "31")]
        SystemRebootRequest(super::super::pb_system::RebootRequest),
        #[prost(message, tag = "32")]
        SystemDeviceInfoRequest(super::super::pb_system::DeviceInfoRequest),
        #[prost(message, tag = "33")]
        SystemDeviceInfoResponse(super::super::pb_system::DeviceInfoResponse),
        #[prost(message, tag = "34")]
        SystemFactoryResetRequest(super::super::pb_system::FactoryResetRequest),
        #[prost(message, tag = "35")]
        SystemGetDatetimeRequest(super::super::pb_system::GetDateTimeRequest),
        #[prost(message, tag = "36")]
        SystemGetDatetimeResponse(super::super::pb_system::GetDateTimeResponse),
        #[prost(message, tag = "37")]
        SystemSetDatetimeRequest(super::super::pb_system::SetDateTimeRequest),
        #[prost(message, tag = "38")]
        SystemPlayAudiovisualAlertRequest(
            super::super::pb_system::PlayAudiovisualAlertRequest,
        ),
        #[prost(message, tag = "39")]
        SystemProtobufVersionRequest(super::super::pb_system::ProtobufVersionRequest),
        #[prost(message, tag = "40")]
        SystemProtobufVersionResponse(super::super::pb_system::ProtobufVersionResponse),
        #[prost(message, tag = "41")]
        SystemUpdateRequest(super::super::pb_system::UpdateRequest),
        #[prost(message, tag = "46")]
        SystemUpdateResponse(super::super::pb_system::UpdateResponse),
        #[prost(message, tag = "44")]
        SystemPowerInfoRequest(super::super::pb_system::PowerInfoRequest),
        #[prost(message, tag = "45")]
        SystemPowerInfoResponse(super::super::pb_system::PowerInfoResponse),
        #[prost(message, tag = "28")]
        StorageInfoRequest(super::super::pb_storage::InfoRequest),
        #[prost(message, tag = "29")]
        StorageInfoResponse(super::super::pb_storage::InfoResponse),
        #[prost(message, tag = "59")]
        StorageTimestampRequest(super::super::pb_storage::TimestampRequest),
        #[prost(message, tag = "60")]
        StorageTimestampResponse(super::super::pb_storage::TimestampResponse),
        #[prost(message, tag = "24")]
        StorageStatRequest(super::super::pb_storage::StatRequest),
        #[prost(message, tag = "25")]
        StorageStatResponse(super::super::pb_storage::StatResponse),
        #[prost(message, tag = "7")]
        StorageListRequest(super::super::pb_storage::ListRequest),
        #[prost(message, tag = "8")]
        StorageListResponse(super::super::pb_storage::ListResponse),
        #[prost(message, tag = "9")]
        StorageReadRequest(super::super::pb_storage::ReadRequest),
        #[prost(message, tag = "10")]
        StorageReadResponse(super::super::pb_storage::ReadResponse),
        #[prost(message, tag = "11")]
        StorageWriteRequest(super::super::pb_storage::WriteRequest),
        #[prost(message, tag = "12")]
        StorageDeleteRequest(super::super::pb_storage::DeleteRequest),
        #[prost(message, tag = "13")]
        StorageMkdirRequest(super::super::pb_storage::MkdirRequest),
        #[prost(message, tag = "14")]
        StorageMd5sumRequest(super::super::pb_storage::Md5sumRequest),
        #[prost(message, tag = "15")]
        StorageMd5sumResponse(super::super::pb_storage::Md5sumResponse),
        #[prost(message, tag = "30")]
        StorageRenameRequest(super::super::pb_storage::RenameRequest),
        #[prost(message, tag = "42")]
        StorageBackupCreateRequest(super::super::pb_storage::BackupCreateRequest),
        #[prost(message, tag = "43")]
        StorageBackupRestoreRequest(super::super::pb_storage::BackupRestoreRequest),
        #[prost(message, tag = "16")]
        AppStartRequest(super::super::pb_app::StartRequest),
        #[prost(message, tag = "17")]
        AppLockStatusRequest(super::super::pb_app::LockStatusRequest),
        #[prost(message, tag = "18")]
        AppLockStatusResponse(super::super::pb_app::LockStatusResponse),
        #[prost(message, tag = "47")]
        AppExitRequest(super::super::pb_app::AppExitRequest),
        #[prost(message, tag = "48")]
        AppLoadFileRequest(super::super::pb_app::AppLoadFileRequest),
        #[prost(message, tag = "49")]
        AppButtonPressRequest(super::super::pb_app::AppButtonPressRequest),
        #[prost(message, tag = "50")]
        AppButtonReleaseRequest(super::super::pb_app::AppButtonReleaseRequest),
        #[prost(message, tag = "63")]
        AppGetErrorRequest(super::super::pb_app::GetErrorRequest),
        #[prost(message, tag = "64")]
        AppGetErrorResponse(super::super::pb_app::GetErrorResponse),
        #[prost(message, tag = "65")]
        AppDataExchangeRequest(super::super::pb_app::DataExchangeRequest),
        #[prost(message, tag = "20")]
        GuiStartScreenStreamRequest(super::super::pb_gui::StartScreenStreamRequest),
        #[prost(message, tag = "21")]
        GuiStopScreenStreamRequest(super::super::pb_gui::StopScreenStreamRequest),
        #[prost(message, tag = "22")]
        GuiScreenFrame(super::super::pb_gui::ScreenFrame),
        #[prost(message, tag = "23")]
        GuiSendInputEventRequest(super::super::pb_gui::SendInputEventRequest),
        #[prost(message, tag = "26")]
        GuiStartVirtualDisplayRequest(super::super::pb_gui::StartVirtualDisplayRequest),
        #[prost(message, tag = "27")]
        GuiStopVirtualDisplayRequest(super::super::pb_gui::StopVirtualDisplayRequest),
        #[prost(message, tag = "51")]
        GpioSetPinMode(super::super::pb_gpio::SetPinMode),
        #[prost(message, tag = "52")]
        GpioSetInputPull(super::super::pb_gpio::SetInputPull),
        #[prost(message, tag = "53")]
        GpioGetPinMode(super::super::pb_gpio::GetPinMode),
        #[prost(message, tag = "54")]
        GpioGetPinModeResponse(super::super::pb_gpio::GetPinModeResponse),
        #[prost(message, tag = "55")]
        GpioReadPin(super::super::pb_gpio::ReadPin),
        #[prost(message, tag = "56")]
        GpioReadPinResponse(super::super::pb_gpio::ReadPinResponse),
        #[prost(message, tag = "57")]
        GpioWritePin(super::super::pb_gpio::WritePin),
        #[prost(message, tag = "58")]
        AppStateResponse(super::super::pb_app::AppStateResponse),
        #[prost(message, tag = "61")]
        PropertyGetRequest(super::super::pb_property::GetRequest),
        #[prost(message, tag = "62")]
        PropertyGetResponse(super::super::pb_property::GetResponse),
        #[prost(message, tag = "66")]
        DesktopIsLockedRequest(super::super::pb_desktop::IsLockedRequest),
        #[prost(message, tag = "67")]
        DesktopUnlockRequest(super::super::pb_desktop::UnlockRequest),
        #[prost(message, tag = "68")]
        DesktopStatusSubscribeRequest(super::super::pb_desktop::StatusSubscribeRequest),
        #[prost(message, tag = "69")]
        DesktopStatusUnsubscribeRequest(
            super::super::pb_desktop::StatusUnsubscribeRequest,
        ),
        #[prost(message, tag = "70")]
        DesktopStatus(super::super::pb_desktop::Status),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Region {
    #[prost(bytes = "vec", tag = "1")]
    pub country_code: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag = "2")]
    pub bands: ::prost::alloc::vec::Vec<region::Band>,
}
/// Nested message and enum types in `Region`.
pub mod region {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Band {
        #[prost(uint32, tag = "1")]
        pub start: u32,
        #[prost(uint32, tag = "2")]
        pub end: u32,
        #[prost(int32, tag = "3")]
        pub power_limit: i32,
        #[prost(uint32, tag = "4")]
        pub duty_cycle: u32,
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CommandStatus {
    Ok = 0,
    /// *< Common Errors
    ///
    /// *< Unknown error
    Error = 1,
    /// *< Command can't be decoded successfully - command_id in response may be wrong!
    ErrorDecode = 2,
    /// *< Command succesfully decoded, but not implemented (deprecated or not yet implemented)
    ErrorNotImplemented = 3,
    /// *< Somebody took global lock, so not all commands are available
    ErrorBusy = 4,
    /// *< Not received has_next == 0
    ErrorContinuousCommandInterrupted = 14,
    /// *< not provided (or provided invalid) crucial parameters to perform rpc
    ErrorInvalidParameters = 15,
    /// *< Storage Errors
    ///
    /// *< FS not ready
    ErrorStorageNotReady = 5,
    /// *< File/Dir alrady exist
    ErrorStorageExist = 6,
    /// *< File/Dir does not exist
    ErrorStorageNotExist = 7,
    /// *< Invalid API parameter
    ErrorStorageInvalidParameter = 8,
    /// *< Access denied
    ErrorStorageDenied = 9,
    /// *< Invalid name/path
    ErrorStorageInvalidName = 10,
    /// *< Internal error
    ErrorStorageInternal = 11,
    /// *< Functon not implemented
    ErrorStorageNotImplemented = 12,
    /// *< File/Dir already opened
    ErrorStorageAlreadyOpen = 13,
    /// *< Directory, you're going to remove is not empty
    ErrorStorageDirNotEmpty = 18,
    /// *< Application Errors
    ///
    /// *< Can't start app - internal error
    ErrorAppCantStart = 16,
    /// *< Another app is running
    ErrorAppSystemLocked = 17,
    /// *< App is not running or doesn't support RPC commands
    ErrorAppNotRunning = 21,
    /// *< Command execution error
    ErrorAppCmdError = 22,
    /// *< Virtual Display Errors
    ///
    /// *< Virtual Display session can't be started twice
    ErrorVirtualDisplayAlreadyStarted = 19,
    /// *< Virtual Display session can't be stopped when it's not started
    ErrorVirtualDisplayNotStarted = 20,
    /// *< GPIO Errors
    ErrorGpioModeIncorrect = 58,
    ErrorGpioUnknownPinMode = 59,
}
impl CommandStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CommandStatus::Ok => "OK",
            CommandStatus::Error => "ERROR",
            CommandStatus::ErrorDecode => "ERROR_DECODE",
            CommandStatus::ErrorNotImplemented => "ERROR_NOT_IMPLEMENTED",
            CommandStatus::ErrorBusy => "ERROR_BUSY",
            CommandStatus::ErrorContinuousCommandInterrupted => {
                "ERROR_CONTINUOUS_COMMAND_INTERRUPTED"
            }
            CommandStatus::ErrorInvalidParameters => "ERROR_INVALID_PARAMETERS",
            CommandStatus::ErrorStorageNotReady => "ERROR_STORAGE_NOT_READY",
            CommandStatus::ErrorStorageExist => "ERROR_STORAGE_EXIST",
            CommandStatus::ErrorStorageNotExist => "ERROR_STORAGE_NOT_EXIST",
            CommandStatus::ErrorStorageInvalidParameter => {
                "ERROR_STORAGE_INVALID_PARAMETER"
            }
            CommandStatus::ErrorStorageDenied => "ERROR_STORAGE_DENIED",
            CommandStatus::ErrorStorageInvalidName => "ERROR_STORAGE_INVALID_NAME",
            CommandStatus::ErrorStorageInternal => "ERROR_STORAGE_INTERNAL",
            CommandStatus::ErrorStorageNotImplemented => "ERROR_STORAGE_NOT_IMPLEMENTED",
            CommandStatus::ErrorStorageAlreadyOpen => "ERROR_STORAGE_ALREADY_OPEN",
            CommandStatus::ErrorStorageDirNotEmpty => "ERROR_STORAGE_DIR_NOT_EMPTY",
            CommandStatus::ErrorAppCantStart => "ERROR_APP_CANT_START",
            CommandStatus::ErrorAppSystemLocked => "ERROR_APP_SYSTEM_LOCKED",
            CommandStatus::ErrorAppNotRunning => "ERROR_APP_NOT_RUNNING",
            CommandStatus::ErrorAppCmdError => "ERROR_APP_CMD_ERROR",
            CommandStatus::ErrorVirtualDisplayAlreadyStarted => {
                "ERROR_VIRTUAL_DISPLAY_ALREADY_STARTED"
            }
            CommandStatus::ErrorVirtualDisplayNotStarted => {
                "ERROR_VIRTUAL_DISPLAY_NOT_STARTED"
            }
            CommandStatus::ErrorGpioModeIncorrect => "ERROR_GPIO_MODE_INCORRECT",
            CommandStatus::ErrorGpioUnknownPinMode => "ERROR_GPIO_UNKNOWN_PIN_MODE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OK" => Some(Self::Ok),
            "ERROR" => Some(Self::Error),
            "ERROR_DECODE" => Some(Self::ErrorDecode),
            "ERROR_NOT_IMPLEMENTED" => Some(Self::ErrorNotImplemented),
            "ERROR_BUSY" => Some(Self::ErrorBusy),
            "ERROR_CONTINUOUS_COMMAND_INTERRUPTED" => {
                Some(Self::ErrorContinuousCommandInterrupted)
            }
            "ERROR_INVALID_PARAMETERS" => Some(Self::ErrorInvalidParameters),
            "ERROR_STORAGE_NOT_READY" => Some(Self::ErrorStorageNotReady),
            "ERROR_STORAGE_EXIST" => Some(Self::ErrorStorageExist),
            "ERROR_STORAGE_NOT_EXIST" => Some(Self::ErrorStorageNotExist),
            "ERROR_STORAGE_INVALID_PARAMETER" => Some(Self::ErrorStorageInvalidParameter),
            "ERROR_STORAGE_DENIED" => Some(Self::ErrorStorageDenied),
            "ERROR_STORAGE_INVALID_NAME" => Some(Self::ErrorStorageInvalidName),
            "ERROR_STORAGE_INTERNAL" => Some(Self::ErrorStorageInternal),
            "ERROR_STORAGE_NOT_IMPLEMENTED" => Some(Self::ErrorStorageNotImplemented),
            "ERROR_STORAGE_ALREADY_OPEN" => Some(Self::ErrorStorageAlreadyOpen),
            "ERROR_STORAGE_DIR_NOT_EMPTY" => Some(Self::ErrorStorageDirNotEmpty),
            "ERROR_APP_CANT_START" => Some(Self::ErrorAppCantStart),
            "ERROR_APP_SYSTEM_LOCKED" => Some(Self::ErrorAppSystemLocked),
            "ERROR_APP_NOT_RUNNING" => Some(Self::ErrorAppNotRunning),
            "ERROR_APP_CMD_ERROR" => Some(Self::ErrorAppCmdError),
            "ERROR_VIRTUAL_DISPLAY_ALREADY_STARTED" => {
                Some(Self::ErrorVirtualDisplayAlreadyStarted)
            }
            "ERROR_VIRTUAL_DISPLAY_NOT_STARTED" => {
                Some(Self::ErrorVirtualDisplayNotStarted)
            }
            "ERROR_GPIO_MODE_INCORRECT" => Some(Self::ErrorGpioModeIncorrect),
            "ERROR_GPIO_UNKNOWN_PIN_MODE" => Some(Self::ErrorGpioUnknownPinMode),
            _ => None,
        }
    }
}
