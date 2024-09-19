use std::sync::LazyLock;
use regex::Regex;

macro_rules! rgx {
    ($name:ident, $pattern:expr) => {
        pub static $name: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new($pattern).unwrap()
        });
    };
}

rgx!(STRUCT_RE, 
    r###"^pub struct (\w+)"###
);

rgx!(MAP_RE, 
    r###"#\[prost\(map\s*=\s*"(\w+),\s*(\w+)",\s*tag\s*=\s*"(\d+)"\)\]"###
);

rgx!(REPEATED_RE, 
    r###"#\[prost\((\w+),\s*repeated,\s*tag\s*=\s*"(\d+)"\)\]"###
);

rgx!(BASIC_RE, 
    r###"#\[prost\((\w+),\s*tag\s*=\s*"(\d+)"\)\]"###
);

rgx!(OPTIONAL_RE, 
    r###"#\[prost\(message,\s*optional,\s*tag\s*=\s*"(\d+)"\)\]"###
);

rgx!(REPEATED_MESSAGE_RE, 
    r###"#\[prost\(message,\s*repeated,\s*tag\s*=\s*"(\d+)"\)\]"###
);