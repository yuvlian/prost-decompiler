import os
import re
import classes
from typing import Dict, List

inp_file = input("Input file name: ")

if not os.path.exists(inp_file):
    print("File cannot be found")
    exit()

file_stream = open(inp_file, "r")
lines = file_stream.readlines()

def decide(s):
    if s.startswith("#[derive"):
        if s.endswith("::prost::Oneof)]"): return "oneof"
        if s.endswith("::prost::Enumeration)]"): return "enum"
        return "message"

    if s.startswith("#[prost"): return "prop"
    if s.startswith("pub mod"): return "mod"
    return "ignore"

i = 0

# This will be called when the current line we're checking starts with "#[prost"
# (meaning that we're reading property of a message)
def parse_prop() -> classes.Property:
    global i
    args = [x.strip() for x in lines[i].strip()[8:-2].split(",")] # The 8:-2 slicing removes the `#[prost(` and `)]` part of the line string

    # args[0] always contains the type name and optionally the type detail
    parsed_type = [x.strip() for x in args.pop(0).split("=")]
    if len(parsed_type) == 2 and parsed_type[1].startswith('"'):
        while not parsed_type[1].endswith('"'):
            parsed_type[1] += "," + args.pop(0)

    type_name = parsed_type[0]
    type_detail = "" if len(parsed_type) == 1 else parsed_type[1][1:-1] # The 1:-1 slicing removes the quotation from the type detail

    attributes = []
    tag = ""
    while args:
        st = args.pop(0)

        if st.startswith("tags"):
            while not st.endswith('"'):
                st = args.pop(0)
        elif st.startswith("tag"):
            tag = re.search(r"=\s*(.*)$", st).group(1)[1:-1]
        else: attributes.append(st)

    i += 1 # Move to property declaration
    prop_data = re.match(r"^\s*pub (\S+):\s*(.+)", lines[i])
    prop_name = prop_data.group(1)

    prop_detail = prop_data.group(2)
    while decide(lines[i+1].strip()) == "ignore" and lines[i+1].strip() != "}":
        i += 1
        prop_detail += "\n"+lines[i]

    i += 1 # Move to next line
    return classes.Property(prop_name, prop_detail, type_name, type_detail, attributes, tag)

def parse_message() -> classes.Message:
    global i
    i += 1 # Skip the `#derive` line

    msg_header = re.match(r"^(?:pub )?struct (\S+)\s*{(?:\s*})?$", lines[i].strip())
    msg_name = msg_header.group(1)
    # print(msg_name)
    props = []
    if not msg_header.group(0).endswith("}"):
        i += 1

        while not lines[i].strip().endswith("}"):
            if decide(lines[i].strip()) != "prop":
                i += 1
                continue

            props.append(parse_prop())

    i += 1
    return classes.Message(msg_name, props)

def parse_enum():
    global i
    i += 2 # Skip the `#derive` and `#repr` line

    enum_header = re.match(r"^(?:pub\s)?\s*enum (\S+)\s*{(?:\s*})?$", lines[i].strip())
    enum_name = enum_header.group(1)

    key_vals = []
    if not enum_header.group(0).endswith("}"):
        i += 1

        end = False
        while not end:
            st = lines[i].strip()
            if "=" in st:
                spl = [x.strip() for x in st.split("=")]

                if spl[1].endswith("}"): spl[1] = spl[1][:-1]
                if spl[1].endswith(","): spl[1] = spl[1][:-1]

                key_vals.append(spl)

            if st.endswith("}"): end = True
            else: i += 1

    i += 1
    return classes.Enum(enum_name, key_vals)

def parse_oneof_item():
    global i
    args = [x.strip() for x in lines[i].strip()[8:-2].split(",")] # The 8:-2 slicing removes the `#[prost(` and `)]` part of the line string

    # args[0] always contains the type name and optionally the type detail
    parsed_type = [x.strip() for x in args.pop(0).split("=")]
    if len(parsed_type) == 2 and parsed_type[1].startswith('"'):
        while not parsed_type[1].endswith('"'):
            parsed_type[1] += "," + args.pop(0)

    type_name = parsed_type[0]
    type_detail = "" if len(parsed_type) == 1 else parsed_type[1][1:-1] # The 1:-1 slicing removes the quotation from the type detail

    attributes = []
    tag = ""
    for x in range(len(args)):
        st = args[x]

        if st.startswith("tag"):
            tag = re.search(r"=\s*(.*)$", st).group(1)[1:-1]
        else: attributes.append(st)

    i += 1 # Move to property declaration
    item_detail = lines[i]
    while decide(lines[i+1].strip()) == "ignore" and lines[i+1].strip() != "}":
        i += 1
        prop_detail += "\n"+lines[i]

    i += 1
    return classes.OneofItem(type_name, type_detail, tag, attributes, item_detail)

def parse_oneof():
    global i
    i += 1 # Skip the #derive line

    enum_header = re.match(r"^(?:pub\s)?\s*enum (\S+)\s*{(?:\s*})?$", lines[i].strip())
    enum_name = enum_header.group(1)

    items = []
    if not enum_header.group(0).endswith("}"):
        i += 1

        while not lines[i].strip().endswith("}"):
            if decide(lines[i].strip()) != "prop":
                i += 1
                continue

            items.append(parse_oneof_item())

    i += 1
    return classes.Oneof(enum_name, items)

def parse_mod():
    global i

    mod_header = re.match(r"^(?:pub\s)?\s*mod (\S+)\s*{(?:\s*})?$", lines[i].strip())
    mod_name = mod_header.group(1)

    items: List[classes.Oneof] = []
    if not mod_header.group(0).endswith("}"):
        i += 1

        while not lines[i].strip().endswith("}"):
            if decide(lines[i].strip()) != "oneof":
                i += 1
                continue

            items.append(parse_oneof())

    i += 1
    return classes.Mod(mod_name, {x.name: x for x in items})

msgs: Dict[str, classes.Message] = {}
enums: Dict[str, classes.Enum] = {}
mods: Dict[str, classes.Mod] = {}
pa = set()
while i < len(lines):
    if i in pa:
        print("We've encountered a loop at ", i)
        break
    dec = decide(lines[i].strip())

    pa.add(i)

    if dec == "message":
        msg = parse_message()
        msgs[msg.name] = msg
    elif dec == "enum":
        enum = parse_enum()
        enums[enum.name] = enum
    elif dec == "mod":
        mod = parse_mod()
        mods[mod.name] = mod
    else: i += 1

lines = ['syntax="proto3";\n\n']

for msg in msgs.values():
    lines.extend([f"{x}\n" for x in msg.to_lines(msgs, enums, mods)])

for enum in enums.values():
    lines.extend([f"{x}\n" for x in enum.to_lines()])

out_file = input("Output file name: ")


with open(out_file, "w") as file:
    file.writelines(lines)

print("\nDecompiled.")
