use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

include!("re.rs");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input_file = String::new();
    let mut output_file = String::new();

    println!("Enter the name of the file to be decompiled:");
    io::stdin().read_line(&mut input_file)?;
    let input_file = input_file.trim();
    let input_path = Path::new(input_file);
    let file = File::open(&input_path)?;
    let reader = io::BufReader::new(file);

    println!("Enter the name for the output file:");
    io::stdin().read_line(&mut output_file)?;
    let output_file = output_file.trim();
    let output_path = Path::new(output_file);

    let mut output = File::create(&output_path)?;
    writeln!(output, r#"syntax = "proto3";"#)?;

    let mut is_in_message = false;

    let mut lines = reader.lines().peekable();
    
    while let Some(line) = lines.next() {
        let line = line?;
        let trimmed_line = line.trim();
        
        if trimmed_line.starts_with("//") || trimmed_line.starts_with("#[derive") {
            continue;
        }

        if let Some(caps) = STRUCT_RE.captures(trimmed_line) {
            if is_in_message {
                writeln!(output, "}}")?;
            }
            writeln!(output, "")?;
            let message_name = caps.get(1).unwrap().as_str().replace(":", "");
            writeln!(output, "message {} {{", message_name)?;
            is_in_message = true;
            continue;
        }

        if let Some(caps) = MAP_RE.captures(trimmed_line) {
            let key_type = caps.get(1).unwrap().as_str();
            let value_type = caps.get(2).unwrap().as_str();
            let tag = caps.get(3).unwrap().as_str();
            if let Some(next_line) = lines.peek() {
                if let Ok(next_line) = next_line {
                    let next_line = next_line.trim();
                    let field_name = next_line.split_whitespace().nth(1).unwrap().replace(":", "");
                    writeln!(output, "  map<{}, {}> {} = {};",
                             key_type, value_type, field_name, tag)?;
                    lines.next();
                }
            }
            continue;
        }

        if let Some(caps) = REPEATED_RE.captures(trimmed_line) {
            let field_type = caps.get(1).unwrap().as_str();
            let tag = caps.get(2).unwrap().as_str();
            if let Some(next_line) = lines.peek() {
                if let Ok(next_line) = next_line {
                    let next_line = next_line.trim();
                    let field_name = next_line.split_whitespace().nth(1).unwrap().replace(":", "");
                    writeln!(output, "  repeated {} {} = {};",
                             field_type, field_name, tag)?;
                    lines.next();
                }
            }
            continue;
        }

        if let Some(caps) = BASIC_RE.captures(trimmed_line) {
            let field_type = caps.get(1).unwrap().as_str();
            let tag = caps.get(2).unwrap().as_str();
            if let Some(next_line) = lines.peek() {
                if let Ok(next_line) = next_line {
                    let next_line = next_line.trim();
                    let field_name = next_line.split_whitespace().nth(1).unwrap().replace(":", "");
                    writeln!(output, "  {} {} = {};",
                             field_type, field_name, tag)?;
                    lines.next();
                }
            }
            continue;
        }

        if let Some(caps) = OPTIONAL_RE.captures(trimmed_line) {
            let tag = caps.get(1).unwrap().as_str();
            if let Some(next_line) = lines.peek() {
                if let Ok(next_line) = next_line {
                    let next_line = next_line.trim();
                    let parts: Vec<&str> = next_line.split_whitespace().collect();
                    let field_name = parts[1].split(':').next().unwrap().replace(":", "");
                    let field_type = parts[1].split('<').last().unwrap().split('>').next().unwrap();
                    writeln!(output, "  {} {} = {};",
                             field_type, field_name, tag)?;
                    lines.next();
                }
            }
            continue;
        }

        if let Some(caps) = REPEATED_MESSAGE_RE.captures(trimmed_line) {
            let tag = caps.get(1).unwrap().as_str();
            if let Some(next_line) = lines.peek() {
                if let Ok(next_line) = next_line {
                    let next_line = next_line.trim();
                    let parts: Vec<&str> = next_line.split_whitespace().collect();
                    let field_name = parts[1].split(':').next().unwrap().replace(":", "");
                    let field_type = parts[1].split('<').last().unwrap().split('>').next().unwrap();
                    writeln!(output, "  repeated {} {} = {};",
                             field_type, field_name, tag)?;
                    lines.next();
                }
            }
            continue;
        }
    }

    if is_in_message {
        writeln!(output, "}}")?;
    }

    Ok(())
}