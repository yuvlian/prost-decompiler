import re


def prost_to_proto(prost_input: str) -> str:
    patterns = {
        "message": r"#\[derive\(.*\)\]\npub struct (\w+) {",
        "field": r"#\[prost\((?P<type>\w+)(?:\s*=\s*\"vec\")?,\s*tag\s*=\s*\"(?P<tag>\d+)\"\)\]\n\s*pub (?P<name>\w+): (?P<field_type>.+?),",
        "enum": r"#\[derive\(.*\)\]\npub enum (\w+) {",
        "enum_value": r"(\w+)\s*=\s*(\d+),",
        "map": r"#\[prost\(map\s*=\s*\"(?P<key_type>\w+),\s*(?P<value_type>\w+)\"\s*,\s*tag\s*=\s*\"(?P<tag>\d+)\"\)\]\n\s*pub (?P<name>\w+): ::std::collections::HashMap<(?P<key>\S+),\s*(?P<value>\S+)>",
        "oneof": r"#\[prost\(oneof\s*=\s*\"(\w+)\"\s*,\s*tags\s*=\s*\"([\d, ]+)\"\)\]\n\s*pub (\w+): ::core::option::Option<two::Choice>",
    }

    output = ['syntax = "proto3";\n']

    def parse_message(message_name, message_body):
        output.append(f"message {message_name} {{")

        for match in re.finditer(
            patterns["field"], message_body
        ):
            field_type = match.group("type")
            field_tag = match.group("tag")
            field_name = match.group("name")
            field_value_type = match.group(
                "field_type"
            ).strip()

            if (
                "::prost::alloc::vec::Vec"
                in field_value_type
            ):
                repeated_type = re.search(
                    r"Vec<(.+)>", field_value_type
                ).group(1)
                output.append(
                    f"    repeated {repeated_type} {field_name} = {field_tag};"
                )
            elif (
                "::prost::alloc::string::String"
                in field_value_type
            ):
                output.append(
                    f"    string {field_name} = {field_tag};"
                )
            elif "Vec<u8>" in field_value_type:
                output.append(
                    f"    bytes {field_name} = {field_tag};"
                )
            else:
                output.append(
                    f"    {field_type} {field_name} = {field_tag};"
                )

        for match in re.finditer(
            patterns["map"], message_body
        ):
            key_type = match.group("key_type")
            value_type = match.group("value_type")
            tag = match.group("tag")
            name = match.group("name")
            output.append(
                f"    map<{key_type}, {value_type}> {name} = {tag};"
            )

        if "oneof" in message_body:
            output.append("    oneof choice {")
            for match in re.finditer(
                patterns["oneof"], message_body
            ):
                oneof_name = match.group(3)
                if "TextOption" in match.group():
                    output.append(
                        f"        string {oneof_name} = 3;"
                    )
                elif "NumberOption" in match.group():
                    output.append(
                        f"        int32 {oneof_name} = 4;"
                    )
                elif "BooleanOption" in match.group():
                    output.append(
                        f"        bool {oneof_name} = 5;"
                    )
            output.append("    }")

        output.append("}")

    def parse_enum(enum_name, enum_body):
        output.append(f"enum {enum_name} {{")
        for match in re.finditer(
            patterns["enum_value"], enum_body
        ):
            enum_value_name = match.group(1)
            enum_value_number = match.group(2)
            output.append(
                f"    {enum_value_name.upper()} = {enum_value_number};"
            )
        output.append("}")

    message_blocks = re.split(
        r"#\[derive\(.*\)\]", prost_input
    )
    for block in message_blocks:
        if "pub struct" in block:
            message_name = re.search(
                r"pub struct (\w+)", block
            ).group(1)
            parse_message(message_name, block)

        elif "pub enum" in block:
            enum_name = re.search(
                r"pub enum (\w+)", block
            ).group(1)
            parse_enum(enum_name, block)

    return "\n".join(output)


def read_file(filepath: str) -> str:
    with open(filepath, "r") as file:
        return file.read()


def write_file(filepath: str, content: str):
    with open(filepath, "w") as file:
        file.write(content)


def main():
    source_file = "test_files/test_prost_input.txt"
    target_file = "test_files/prost_decompiled.proto"
    prost_input = read_file(source_file)
    proto_output = prost_to_proto(prost_input)
    write_file(target_file, proto_output)


try:
    main()
except Exception as e:
    print("Uh, error: {e}")
