// This file is @generated by prost-build.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct One {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(sint32, tag = "2")]
    pub signed_id: i32,
    #[prost(uint32, tag = "3")]
    pub unsigned_id: u32,
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
    #[prost(bool, tag = "5")]
    pub is_active: bool,
    #[prost(double, tag = "6")]
    pub balance: f64,
    #[prost(float, tag = "7")]
    pub rating: f32,
    #[prost(bytes = "vec", tag = "8")]
    pub data_blob: ::prost::alloc::vec::Vec<u8>,
    #[prost(fixed32, tag = "9")]
    pub fixed_value: u32,
    #[prost(sfixed32, tag = "10")]
    pub signed_fixed_value: i32,
    #[prost(float, repeated, tag = "11")]
    pub values: ::prost::alloc::vec::Vec<f32>,
    #[prost(map = "string, int32", tag = "12")]
    pub key_value_pairs: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        i32,
    >,
    #[prost(map = "sint64, enumeration(Status)", tag = "3883")]
    pub status_map: ::std::collections::HashMap<i64, i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Two {
    #[prost(message, optional, tag = "1")]
    pub one_field: ::core::option::Option<One>,
    #[prost(message, repeated, tag = "999")]
    pub multiple_one_field: ::prost::alloc::vec::Vec<One>,
    #[prost(enumeration = "Status", tag = "2")]
    pub status: i32,
    #[prost(enumeration = "Status", repeated, tag = "363")]
    pub multiple_status_field: ::prost::alloc::vec::Vec<i32>,
    #[prost(sint64, tag = "6")]
    pub long_signed_id: i64,
    #[prost(uint64, tag = "7")]
    pub long_unsigned_id: u64,
    #[prost(fixed64, tag = "8")]
    pub long_fixed_value: u64,
    #[prost(sfixed64, tag = "9")]
    pub long_signed_fixed_value: i64,
    #[prost(oneof = "two::Choice", tags = "3, 4, 5")]
    pub choice: ::core::option::Option<two::Choice>,
}
/// Nested message and enum types in `Two`.
pub mod two {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Choice {
        #[prost(string, tag = "3")]
        TextOption(::prost::alloc::string::String),
        #[prost(int32, tag = "4")]
        NumberOption(i32),
        #[prost(bool, tag = "5")]
        BooleanOption(bool),
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Status {
    Unknown = 0,
    Active = 1,
    Inactive = 2,
}
impl Status {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Status::Unknown => "UNKNOWN",
            Status::Active => "ACTIVE",
            Status::Inactive => "INACTIVE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN" => Some(Self::Unknown),
            "ACTIVE" => Some(Self::Active),
            "INACTIVE" => Some(Self::Inactive),
            _ => None,
        }
    }
}
