#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPinMode {
    #[prost(enumeration = "GpioPin", tag = "1")]
    pub pin: i32,
    #[prost(enumeration = "GpioPinMode", tag = "2")]
    pub mode: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetInputPull {
    #[prost(enumeration = "GpioPin", tag = "1")]
    pub pin: i32,
    #[prost(enumeration = "GpioInputPull", tag = "2")]
    pub pull_mode: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPinMode {
    #[prost(enumeration = "GpioPin", tag = "1")]
    pub pin: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPinModeResponse {
    #[prost(enumeration = "GpioPinMode", tag = "1")]
    pub mode: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadPin {
    #[prost(enumeration = "GpioPin", tag = "1")]
    pub pin: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadPinResponse {
    #[prost(uint32, tag = "2")]
    pub value: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WritePin {
    #[prost(enumeration = "GpioPin", tag = "1")]
    pub pin: i32,
    #[prost(uint32, tag = "2")]
    pub value: u32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GpioPin {
    Pc0 = 0,
    Pc1 = 1,
    Pc3 = 2,
    Pb2 = 3,
    Pb3 = 4,
    Pa4 = 5,
    Pa6 = 6,
    Pa7 = 7,
}
impl GpioPin {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            GpioPin::Pc0 => "PC0",
            GpioPin::Pc1 => "PC1",
            GpioPin::Pc3 => "PC3",
            GpioPin::Pb2 => "PB2",
            GpioPin::Pb3 => "PB3",
            GpioPin::Pa4 => "PA4",
            GpioPin::Pa6 => "PA6",
            GpioPin::Pa7 => "PA7",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PC0" => Some(Self::Pc0),
            "PC1" => Some(Self::Pc1),
            "PC3" => Some(Self::Pc3),
            "PB2" => Some(Self::Pb2),
            "PB3" => Some(Self::Pb3),
            "PA4" => Some(Self::Pa4),
            "PA6" => Some(Self::Pa6),
            "PA7" => Some(Self::Pa7),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GpioPinMode {
    Output = 0,
    Input = 1,
}
impl GpioPinMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            GpioPinMode::Output => "OUTPUT",
            GpioPinMode::Input => "INPUT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OUTPUT" => Some(Self::Output),
            "INPUT" => Some(Self::Input),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GpioInputPull {
    No = 0,
    Up = 1,
    Down = 2,
}
impl GpioInputPull {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            GpioInputPull::No => "NO",
            GpioInputPull::Up => "UP",
            GpioInputPull::Down => "DOWN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NO" => Some(Self::No),
            "UP" => Some(Self::Up),
            "DOWN" => Some(Self::Down),
            _ => None,
        }
    }
}
