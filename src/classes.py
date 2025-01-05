from typing import Any, Dict, List
import re

class Property:
    def __init__(
            self,

            # Let's use this as example:
            # [prost(bytes = "vec", optional, tag = "8")]
            # pub data: ::prost::alloc::vec::Vec<u8>,
            prop_name: str,         # This will be "data"
            prop_detail: str,       # This will be "::prost::alloc::vec::Vec<u8>,"
            type_name: str,         # This will be "bytes"
            type_detail: str,       # This will be "vec"
            attributes: List[str],  # This will be ["optional"]
            tag: str                # This will be "8"
        ):
        self.prop_name = prop_name
        self.prop_detail = prop_detail
        self.type_name = type_name
        self.type_detail = type_detail
        self.attributes = attributes
        self.tag = tag

    def to_lines(self, messages, enums, mods):
        lines = []

        pref_attrs = " ".join([x for x in self.attributes if x != "optional"])

        if self.type_name == "enumeration":
            lines.append(f"{pref_attrs} {self.type_detail} {self.prop_name} = {self.tag};".strip())
        elif self.type_name == "map":
            types = self.type_detail.split(",")

            types[0] = re.match(r"^(?:.*?\()?([^\)]*)", types[0].strip()).group(1)
            types[1] = re.match(r"^(?:.*?\()?([^\)]*)", types[1].strip()).group(1)

            lines.append(f"{pref_attrs} map<{', '.join(types)}> {self.prop_name} = {self.tag};".strip())
        elif self.type_name == "message":
            dat = re.search(r"<\s*(\w+)", self.prop_detail)
            msg = dat.group(1)

            lines.append(f"{pref_attrs} {msg} {self.prop_name} = {self.tag};".strip())
        elif self.type_name == "oneof":
            spl = self.type_detail.split("::")
            oneof = mods[spl[0]].items[spl[1]]

            lines.append(f"{pref_attrs} oneof {self.prop_name} {{".strip())

            for i, item in enumerate(oneof.items):
                lines.append("    " + item.to_str(f"prop{i}", messages, enums, mods))

            lines.append("}")
        else:
            lines.append(f"{pref_attrs} {self.type_name} {self.prop_name} = {self.tag};".strip())

        return lines

class Message:
    def __init__(self, name: str, properties: List[Property]):
        self.name = name
        self.properties = {prop.prop_name: prop for prop in properties}

    def to_lines(self, messages, enums, mods):
        lines = [f"message {self.name} {{"]

        for props in self.properties.values():
            lines.extend(["    " + x for x in props.to_lines(messages, enums, mods)])

        lines.append("}")
        return lines

class Enum:
    def __init__(self, name: str, key_vals: List[List[str]]):
        self.name = name
        self.key_vals = key_vals

    def to_lines(self):
        lines = [f"enum {self.name} {{"]

        for key, val in self.key_vals:
            lines.append(f"    {key} = {val};")

        lines.append("}")

        return lines

class OneofItem:
    def __init__(self, type_name: str, type_detail: str, tag: str, attributes: List[str], item_detail: str):
        self.type_name = type_name
        self.type_detail = type_detail
        self.tag = tag
        self.attributes = attributes
        self.item_detail = item_detail

    def to_str(self, prop_name: str, messages, enums, mods):
        pref_attrs = " ".join([x for x in self.attributes if x != "optional"])

        if self.type_name == "message":
            dat = re.search(r"(\S+)\(super::(\S+)\s*\)", self.item_detail)

            prop_name = dat.group(1)
            msg_name = dat.group(2)

            return f"{pref_attrs} {msg_name} {prop_name} = {self.tag};".strip()

        if self.type_name == "enumeration":
            return f"{pref_attrs} {self.type_detail} {prop_name} = {self.tag};".strip()

        if self.type_name == "map":
            types = self.type_detail.split(",")

            types[0] = re.match(r"^(?:.*?\()?(.*)(?:\))?$", types[0].strip()).group(1)
            types[1] = re.match(r"^(?:.*?\()?(.*)(?:\))?$", types[1].strip()).group(1)

            return f"{pref_attrs} map<{', '.join(types)}> {prop_name} = {self.tag};".strip()

        return f"{pref_attrs} {self.type_name} {prop_name} = {self.tag};".strip()

class Oneof:
    def __init__(self, name: str, items: List[OneofItem]):
        self.name = name
        self.items = items

class Mod:
    def __init__(self, name: str, items: Dict[str, Any]):
        self.name = name
        self.items = items
