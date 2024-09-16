use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};

mod re;
use re::*;

fn main() -> io::Result<()> {
    let mut input = String::new();

    print!("prost output file name: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;

    let rust_file = input.trim();
    let rust_code = fs::read_to_string(rust_file)?;
    let protobuf_definitions = parse_rust_code(&rust_code);
    
    let mut output = String::new();

    print!("decompiled proto output file name: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut output)?;

    let protobuf_file = output.trim();
    fs::write(protobuf_file, protobuf_definitions)?;
    
    println!("Protobuf definitions written to {}", protobuf_file);
    Ok(())
}

fn parse_rust_code(rust_code: &str) -> String {
    let mut protobuf_definitions = String::new();
    
    protobuf_definitions.push_str("syntax = \"proto3\";\n\n");

    // Extract structs
    for struct_match in STRUCT_REGEX.captures_iter(rust_code) {
        let struct_name = &struct_match[1];
        let fields_block = &struct_match[2];
        
        let mut fields = Vec::new();
        let mut oneofs = HashMap::new();

        // Collect fields and oneofs
        for field_match in FIELD_REGEX.captures_iter(fields_block) {
            let tag = &field_match[1];
            let name = &field_match[2];
            let typ = &field_match[3];
            let mut field_type = match typ {
                "String" => "string".to_string(),
                "i32" => "int32".to_string(),
                "u32" => "uint32".to_string(),
                "i64" => "int64".to_string(),
                "u64" => "uint64".to_string(),
                "f32" => "float".to_string(),
                "f64" => "double".to_string(),
                "bool" => "bool".to_string(),
                "::prost::alloc::string::String" => "string".to_string(),
                "::prost::alloc::vec::Vec<u8>" => "bytes".to_string(),
                "::prost::alloc::vec::Vec<f32>" => "float".to_string(),
                "::prost::alloc::vec::Vec<f64>" => "double".to_string(),
                "::prost::alloc::vec::Vec<i32>" => "int32".to_string(),
                "::prost::alloc::vec::Vec<u32>" => "uint32".to_string(),
                "::prost::alloc::vec::Vec<i64>" => "int64".to_string(),
                "::prost::alloc::vec::Vec<u64>" => "uint64".to_string(),
                "::prost::alloc::vec::Vec<bool>" => "bool".to_string(),
                _ if typ.contains("Option") => {
                    let x = typ.replace("::core::option::Option<", "").replace(">", "");
                    x.to_string()
                },
                _ => typ.to_string()
            };

            if tag.contains("repeated") {
                field_type = format!("repeated {}", field_type);
            } else if tag.contains("map") {
                let map_types: Vec<&str> = tag.split(',').collect();
                let field = format!("map<{}, {}>", map_types[0], map_types[1]);
                field_type = field.replace(r#"""#, "").replace("map = ", "");
            } else if tag.contains("sint32") {
                field_type = String::from("sint32");
            } else if tag.contains("sfixed32") {
                field_type = String::from("sfixed32");
            } else if tag.contains("fixed32") {
                field_type = String::from("fixed32");
            } else if tag.contains("sint64") {
                field_type = String::from("sint64");
            } else if tag.contains("sfixed64") {
                field_type = String::from("sfixed64");
            } else if tag.contains("fixed64") {
                field_type = String::from("fixed64");
            }

            if tag.contains("oneof") {
                let oneof_name = tag.split('(').nth(1).unwrap_or("").trim_end_matches(')');
                oneofs.entry(oneof_name.to_string()).or_insert_with(Vec::new).push(format!("{} = {};", name, fields.len() + 1));
            } else {
                fields.push(format!("    {} {} = {};", field_type, name, fields.len() + 1));
            }
        }

        // Generate the oneof block
        let mut oneof_blocks = Vec::new();
        for (oneof_name, options) in oneofs {
            oneof_blocks.push(format!(
                "    oneof {} {{\n{}\n    }}",
                oneof_name,
                options.join("\n")
            ));
        }

        let protobuf_struct = format!(
            "message {} {{\n{}\n{}\n}}\n",
            struct_name,
            fields.join("\n"),
            oneof_blocks.join("\n")
        );

        protobuf_definitions.push_str(&protobuf_struct);
    }

    // Process enums
    for enum_match in ENUM_REGEX.captures_iter(rust_code) {
        let enum_name = &enum_match[2];
        let enum_variants_block = &enum_match[3];
        
        let mut variants = Vec::new();
        
        for variant_match in ENUM_VARIANT_REGEX.captures_iter(enum_variants_block) {
            let variant_name = &variant_match[1];
            let variant_value = &variant_match[2];
            variants.push(format!("    {} = {};", variant_name, variant_value));
        }
        
        let protobuf_enum = format!(
            "enum {} {{\n{}\n}}\n",
            enum_name,
            variants.join("\n")
        );
        
        protobuf_definitions.push_str(&protobuf_enum);
    }
    
    protobuf_definitions
}
