#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PingRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PingResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RebootRequest {
    #[prost(enumeration = "reboot_request::RebootMode", tag = "1")]
    pub mode: i32,
}
/// Nested message and enum types in `RebootRequest`.
pub mod reboot_request {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum RebootMode {
        /// default value
        Os = 0,
        Dfu = 1,
        Update = 2,
    }
    impl RebootMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RebootMode::Os => "OS",
                RebootMode::Dfu => "DFU",
                RebootMode::Update => "UPDATE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "OS" => Some(Self::Os),
                "DFU" => Some(Self::Dfu),
                "UPDATE" => Some(Self::Update),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceInfoRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceInfoResponse {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FactoryResetRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDateTimeRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDateTimeResponse {
    #[prost(message, optional, tag = "1")]
    pub datetime: ::core::option::Option<DateTime>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetDateTimeRequest {
    #[prost(message, optional, tag = "1")]
    pub datetime: ::core::option::Option<DateTime>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DateTime {
    /// Time
    ///
    /// *< Hour in 24H format: 0-23
    #[prost(uint32, tag = "1")]
    pub hour: u32,
    /// *< Minute: 0-59
    #[prost(uint32, tag = "2")]
    pub minute: u32,
    /// *< Second: 0-59
    #[prost(uint32, tag = "3")]
    pub second: u32,
    /// Date
    ///
    /// *< Current day: 1-31
    #[prost(uint32, tag = "4")]
    pub day: u32,
    /// *< Current month: 1-12
    #[prost(uint32, tag = "5")]
    pub month: u32,
    /// *< Current year: 2000-2099
    #[prost(uint32, tag = "6")]
    pub year: u32,
    /// *< Current weekday: 1-7
    #[prost(uint32, tag = "7")]
    pub weekday: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayAudiovisualAlertRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtobufVersionRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtobufVersionResponse {
    #[prost(uint32, tag = "1")]
    pub major: u32,
    #[prost(uint32, tag = "2")]
    pub minor: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRequest {
    #[prost(string, tag = "1")]
    pub update_manifest: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateResponse {
    #[prost(enumeration = "update_response::UpdateResultCode", tag = "1")]
    pub code: i32,
}
/// Nested message and enum types in `UpdateResponse`.
pub mod update_response {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum UpdateResultCode {
        Ok = 0,
        ManifestPathInvalid = 1,
        ManifestFolderNotFound = 2,
        ManifestInvalid = 3,
        StageMissing = 4,
        StageIntegrityError = 5,
        ManifestPointerError = 6,
        TargetMismatch = 7,
        OutdatedManifestVersion = 8,
        IntFull = 9,
        UnspecifiedError = 10,
    }
    impl UpdateResultCode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                UpdateResultCode::Ok => "OK",
                UpdateResultCode::ManifestPathInvalid => "ManifestPathInvalid",
                UpdateResultCode::ManifestFolderNotFound => "ManifestFolderNotFound",
                UpdateResultCode::ManifestInvalid => "ManifestInvalid",
                UpdateResultCode::StageMissing => "StageMissing",
                UpdateResultCode::StageIntegrityError => "StageIntegrityError",
                UpdateResultCode::ManifestPointerError => "ManifestPointerError",
                UpdateResultCode::TargetMismatch => "TargetMismatch",
                UpdateResultCode::OutdatedManifestVersion => "OutdatedManifestVersion",
                UpdateResultCode::IntFull => "IntFull",
                UpdateResultCode::UnspecifiedError => "UnspecifiedError",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "OK" => Some(Self::Ok),
                "ManifestPathInvalid" => Some(Self::ManifestPathInvalid),
                "ManifestFolderNotFound" => Some(Self::ManifestFolderNotFound),
                "ManifestInvalid" => Some(Self::ManifestInvalid),
                "StageMissing" => Some(Self::StageMissing),
                "StageIntegrityError" => Some(Self::StageIntegrityError),
                "ManifestPointerError" => Some(Self::ManifestPointerError),
                "TargetMismatch" => Some(Self::TargetMismatch),
                "OutdatedManifestVersion" => Some(Self::OutdatedManifestVersion),
                "IntFull" => Some(Self::IntFull),
                "UnspecifiedError" => Some(Self::UnspecifiedError),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PowerInfoRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PowerInfoResponse {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
