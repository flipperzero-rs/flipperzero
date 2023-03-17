#!/usr/bin/env python3
# Helper script for running binaries on a connected Flipper Zero.

import argparse
import os
import sys
from pathlib import Path
from subprocess import run

TOOLS_PATH = '../tools'


def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument('binary', type=Path)
    parser.add_argument('arguments', nargs=argparse.REMAINDER)
    return parser.parse_args()


def main():
    args = parse_args()

    # Run the given FAP binary on a connected Flipper Zero.
    result = run(
        [
            'cargo',
            'run',
            '--quiet',
            '--release',
            '--bin',
            'run-fap',
            '--',
            os.fspath(args.binary),
        ] + args.arguments,
        cwd=os.path.join(os.path.dirname(__file__), TOOLS_PATH),
    )
    if result.returncode:
        sys.exit(result.returncode)


if __name__ == '__main__':
    main()
