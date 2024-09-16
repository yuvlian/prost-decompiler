use regex::Regex;
use std::fs;
use std::io::{self, Write};
use std::collections::HashMap;

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

    // Regex for extracting structs and fields
    let struct_regex = Regex::new(r"#\[derive\(.*?Message\)\]\s*pub struct (\w+) \{([^}]*)\}").unwrap();
    let field_regex = Regex::new(r"#\[prost\((.*?)\)\]\s*pub (\w+): (\w+)").unwrap();
    
    // Regex for extracting enums
    let enum_regex = Regex::new(r"#\[derive\(.*?Enumeration\)\]\s*#[repr\((.*?)\)]\s*pub enum (\w+) \{([^}]*)\}").unwrap();
    let enum_variant_regex = Regex::new(r"(\w+) = (\d+),").unwrap();
    
    // Regex for extracting oneof fields
    let oneof_regex = Regex::new(r"#\[prost\(oneof\((.*?)\)\]\s*pub (\w+): (\w+)").unwrap();

    // Extract structs
    for struct_match in struct_regex.captures_iter(rust_code) {
        let struct_name = &struct_match[1];
        let fields_block = &struct_match[2];
        
        let mut fields = Vec::new();
        let mut oneofs = HashMap::new();

        // Collect fields and oneofs
        for field_match in field_regex.captures_iter(fields_block) {
            let tag = &field_match[1];
            let name = &field_match[2];
            let typ = &field_match[3];
            let mut field_type = match typ {
                "String" => "string".to_string(),
                "Vec<u8>" => "bytes".to_string(),
                "i32" => "int32".to_string(),
                "u32" => "uint32".to_string(),
                "i64" => "int64".to_string(),
                "u64" => "uint64".to_string(),
                "f32" => "float".to_string(),
                "f64" => "double".to_string(),
                "bool" => "bool".to_string(),
                _ => typ.to_string(),
            };

            if tag.contains("repeated") {
                field_type = format!("repeated {}", field_type);
            } else if tag.contains("map") {
                let map_types: Vec<&str> = tag.split(',').collect();
                field_type = format!("map<{}, {}>", map_types[0], map_types[1]);
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
    for enum_match in enum_regex.captures_iter(rust_code) {
        let enum_name = &enum_match[2];
        let enum_variants_block = &enum_match[3];
        
        let mut variants = Vec::new();
        
        for variant_match in enum_variant_regex.captures_iter(enum_variants_block) {
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
