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

import argparse
import os
import sys

import numpy as np


def create_npy_file(file_path, use_float=False):
    """
    Creates a new .npy file at file_path if it does not exist already.

    Args:
        file_path (str): The path to the .npy file.
        use_float (bool): Whether to use float values in the array.
    """
    if os.path.exists(file_path):
        print(f"Error: File exists already at '{file_path}'", file=sys.stderr)
        sys.exit(1)

    if use_float:
        array = np.array([[1.1, 4.2, 3.3], [8.4, 22.5, 12.6]], dtype=float)
    else:
        array = np.array([[1, 4, 3], [8, 22, 12]])

    np.save(file_path, array)

    print(f"Created .npy file at '{file_path}' with contents:\n{array}")


def main():
    """
    Parses command-line arguments and initiates the analysis.
    """
    parser = argparse.ArgumentParser(
        description="Create a demo NumPy (.npy) file.",
        formatter_class=argparse.RawTextHelpFormatter,
    )
    parser.add_argument("file_path", type=str, help="The full path to ouput .npy file")
    parser.add_argument("--float", action="store_true", help="Create array with float values")
    args = parser.parse_args()
    create_npy_file(args.file_path, use_float=args.float)


if __name__ == "__main__":
    main()
