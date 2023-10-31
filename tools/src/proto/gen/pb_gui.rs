#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScreenFrame {
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration = "ScreenOrientation", tag = "2")]
    pub orientation: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartScreenStreamRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopScreenStreamRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendInputEventRequest {
    #[prost(enumeration = "InputKey", tag = "1")]
    pub key: i32,
    #[prost(enumeration = "InputType", tag = "2")]
    pub r#type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartVirtualDisplayRequest {
    /// optional
    #[prost(message, optional, tag = "1")]
    pub first_frame: ::core::option::Option<ScreenFrame>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopVirtualDisplayRequest {}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum InputKey {
    Up = 0,
    Down = 1,
    Right = 2,
    Left = 3,
    Ok = 4,
    Back = 5,
}
impl InputKey {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            InputKey::Up => "UP",
            InputKey::Down => "DOWN",
            InputKey::Right => "RIGHT",
            InputKey::Left => "LEFT",
            InputKey::Ok => "OK",
            InputKey::Back => "BACK",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UP" => Some(Self::Up),
            "DOWN" => Some(Self::Down),
            "RIGHT" => Some(Self::Right),
            "LEFT" => Some(Self::Left),
            "OK" => Some(Self::Ok),
            "BACK" => Some(Self::Back),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum InputType {
    /// *< Press event, emitted after debounce
    Press = 0,
    /// *< Release event, emitted after debounce
    Release = 1,
    /// *< Short event, emitted after InputTypeRelease done withing INPUT_LONG_PRESS interval
    Short = 2,
    /// *< Long event, emmited after INPUT_LONG_PRESS interval, asynchronouse to InputTypeRelease
    Long = 3,
    /// *< Repeat event, emmited with INPUT_REPEATE_PRESS period after InputTypeLong event
    Repeat = 4,
}
impl InputType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            InputType::Press => "PRESS",
            InputType::Release => "RELEASE",
            InputType::Short => "SHORT",
            InputType::Long => "LONG",
            InputType::Repeat => "REPEAT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PRESS" => Some(Self::Press),
            "RELEASE" => Some(Self::Release),
            "SHORT" => Some(Self::Short),
            "LONG" => Some(Self::Long),
            "REPEAT" => Some(Self::Repeat),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ScreenOrientation {
    /// *< Horizontal
    Horizontal = 0,
    /// *< Horizontal flipped (180)
    HorizontalFlip = 1,
    /// *< Vertical (90)
    Vertical = 2,
    /// *< Vertical flipped
    VerticalFlip = 3,
}
impl ScreenOrientation {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ScreenOrientation::Horizontal => "HORIZONTAL",
            ScreenOrientation::HorizontalFlip => "HORIZONTAL_FLIP",
            ScreenOrientation::Vertical => "VERTICAL",
            ScreenOrientation::VerticalFlip => "VERTICAL_FLIP",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "HORIZONTAL" => Some(Self::Horizontal),
            "HORIZONTAL_FLIP" => Some(Self::HorizontalFlip),
            "VERTICAL" => Some(Self::Vertical),
            "VERTICAL_FLIP" => Some(Self::VerticalFlip),
            _ => None,
        }
    }
}
