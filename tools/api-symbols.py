#!/usr/bin/env python3
# Convert `api_symbols.csv` to Rust definitions.
# Example:
# > api-symbols.py --name 'furi_*' firmware/targets/f7/api_symbols.csv

import argparse
import csv
from typing import *

ENTRY_VARIABLE = "Variable"
ENTRY_FUNCTION = "Function"

PUBLIC = "+"
PRIVATE = "-"

RUST_TYPES = {
    "int": "i32",
    "int16_t": "i16",
    "uint16_t": "u16",
    "int32_t": "i32",
    "uint32_t": "u32",
    "int8_t": "i8",
    "uint8_t": "u8",
    "size_t": "usize",
    "const char*": "*const c_char",
    "const uint8_t*": "*const u8",
    "_Bool": "bool",
}


def load(name: str) -> Generator[Dict, None, None]:
    """
    Load symbols from `api_symbols.csv`.
    """
    with open(name, newline='') as f:
        reader = csv.DictReader(f)
        for row in reader:
            yield {k: v.strip() for k, v in row.items()}

def rust_variable_def(symbol: dict, rust_name: Optional[str] = None) -> str:
    """
    Generate Rust variable definition for symbol.
    """

    if symbol["entry"] != ENTRY_VARIABLE:
        raise TypeError(f"Expected Variable, got {symbol['Entry']}")

    name = symbol["name"]
    type_ = symbol["type"]

    lines = [
        f"#[link_name = \"{name}\"]",
        f"pub static {name.upper() or rust_name}: {rust_type(type_)};",
    ]
    
    return "\n".join(lines)

def rust_function_def(symbol: dict, rust_name: Optional[str] = None) -> str:
    """
    Generate Rust function definition for symbol.
    """
    if symbol["entry"] != ENTRY_FUNCTION:
        raise TypeError(f"Expected Function, got {symbol['Entry']}")

    name = symbol["name"]
    rtype = rust_type(symbol["type"])
    params = [rust_type(p) for p in map(lambda s: s.strip(), symbol["params"].split(",")) if p]

    return_ = f" -> {rtype}" if rtype and rtype != "void" else ""
    params_ = ", ".join((f"_arg{n}: {t}" for n, t in enumerate(params)))

    lines = []
    if rust_name:
        lines.append(f"#[link_name = \"{name}\"]")
    lines.append(f"pub fn {rust_name or name}({params_}){return_};")
    
    return "\n".join(lines)


def rust_type(t: str) -> str:
    """
    Try to determine the matching Rust type.
    """
    t = t.strip()
    rust_type = RUST_TYPES.get(t)
    if rust_type is not None:
        return rust_type

    p = 0
    while t.endswith("*"):
        t = t[:-1].rstrip()
        p += 1

    ptr = "".join(["*mut "] * p)
    return f"{ptr}{RUST_TYPES.get(t, t)}"


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("filename")
    parser.add_argument("-m", "--match-prefix", help="match symbol name prefix")
    parser.add_argument("-S", "--strip-prefix", action="store_true", help="strip match prefix from definitions")
    args = parser.parse_args()
    
    api = load(args.filename)
    for symbol in api:
        if symbol["status"] != PUBLIC:
            continue

        if args.match_prefix and not symbol["name"].startswith(args.match_prefix):
            continue

        # Optionally strip symbol prefix
        rust_name = symbol["name"][len(args.match_prefix):] if args.match_prefix and args.strip_prefix else None

        if symbol["entry"] == ENTRY_VARIABLE:
            print(rust_variable_def(symbol, rust_name=rust_name))
        elif symbol["entry"] == ENTRY_FUNCTION:
            print(rust_function_def(symbol, rust_name=rust_name))


if __name__ == "__main__":
    main()
