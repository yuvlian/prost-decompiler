use std::sync::LazyLock;
use regex::Regex;

pub static STRUCT_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"#\[derive\(.*?Message\)\]\s*pub struct (\w+) \{([^}]*)\}").unwrap()
});

pub static FIELD_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"#\[prost\((.*?)\)\]\s*pub (\w+): (\w+)").unwrap()
});

pub static ENUM_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"#\[derive\(.*?Enumeration\)\]\s*#[repr\((.*?)\)]\s*pub enum (\w+) \{([^}]*)\}").unwrap()
});

pub static ENUM_VARIANT_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(\w+) = (\d+),").unwrap()
});

pub static ONEOF_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"#\[prost\(oneof\((.*?)\)\]\s*pub (\w+): (\w+)").unwrap()
});