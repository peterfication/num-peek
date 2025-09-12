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


def create_npy_file(file_path):
    """
    Creates a new .npy file at file_path if it does not exist already.

    Args:
        file_path (str): The path to the .npy file.
    """
    if os.path.exists(file_path):
        print(f"Error: File exists already at '{file_path}'", file=sys.stderr)
        sys.exit(1)

    array = np.array([[1, 4, 3], [8, 22, 12]])

    np.save(file_path, array)


def main():
    """
    Parses command-line arguments and initiates the analysis.
    """
    parser = argparse.ArgumentParser(
        description="Create a demo NumPy (.npy) file.",
        formatter_class=argparse.RawTextHelpFormatter,
    )
    parser.add_argument("file_path", type=str, help="The full path to ouput .npy file")
    args = parser.parse_args()
    create_npy_file(args.file_path)


if __name__ == "__main__":
    main()
