# /// script
# dependencies = [
#   "numpy",
# ]
# ///
# -*- coding: utf-8 -*-
"""
A utility script to create a .npy file.

Usage:
    uv run npy_create.py /path/to/output.npy
"""

import json

import numpy as np


NPY_DEFINITIONS_FILE_PATH = "scripts/npy_definitions.json"


def create_npy_files():
    """
    Creates .npy files according to npy_definitions.json.
    """
    data = []
    with open(NPY_DEFINITIONS_FILE_PATH, "r") as f:
        data = json.load(f)

    print(f"Creating .npy files as per '{NPY_DEFINITIONS_FILE_PATH}'...")
    print("")

    for item in data:
        array = np.array(item["values"], dtype=item["dtype"])
        np.save(item["file_path"], array)
        print(
            f"Created .npy file at '{item['file_path']}' "
            f"with dtype {item['dtype']} contents:\n{array}"
        )
        print("")

    print("All .npy files created successfully.")


def main():
    create_npy_files()


if __name__ == "__main__":
    main()
